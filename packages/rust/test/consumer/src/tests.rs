use super::dispatch;
use cameronapak_platform_sdk::*;
use reqwest::Url;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Notify;
use tokio::task::JoinHandle;

#[derive(Debug)]
struct Recorded {
    method: String,
    target: String,
    headers: BTreeMap<String, String>,
    body: String,
}

struct Server {
    base: String,
    requests: Arc<Mutex<Vec<Recorded>>>,
    received: Arc<Notify>,
    task: JoinHandle<()>,
}
impl Drop for Server {
    fn drop(&mut self) {
        self.task.abort();
    }
}
impl Server {
    async fn start(responses: Vec<Value>, delay: Duration, tls: bool) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!(
            "{}://{}",
            if tls { "https" } else { "http" },
            listener.local_addr().unwrap()
        );
        let requests = Arc::new(Mutex::new(Vec::new()));
        let received = Arc::new(Notify::new());
        let tls = tls.then(|| {
            use tokio_rustls::rustls::{pki_types::PrivateKeyDer, ServerConfig};
            let cert = std::fs::read(std::env::var("SDK_TLS_CERT").unwrap()).unwrap();
            let key = std::fs::read(std::env::var("SDK_TLS_KEY").unwrap()).unwrap();
            let certs = rustls_pemfile::certs(&mut cert.as_slice())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let key: PrivateKeyDer = rustls_pemfile::private_key(&mut key.as_slice())
                .unwrap()
                .unwrap();
            tokio_rustls::TlsAcceptor::from(Arc::new(
                ServerConfig::builder()
                    .with_no_client_auth()
                    .with_single_cert(certs, key)
                    .unwrap(),
            ))
        });
        let captured = requests.clone();
        let notify = received.clone();
        let callback = format!("{base}/callback?data_exchange_status=granted");
        let task = tokio::spawn(async move {
            let mut index = 0;
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                let response = responses
                    .get(index)
                    .unwrap_or_else(|| responses.last().unwrap())
                    .clone();
                index += 1;
                let captured = captured.clone();
                let notify = notify.clone();
                let callback = callback.clone();
                let tls = tls.clone();
                tokio::spawn(async move {
                    if let Some(tls) = tls {
                        // The untrusted-root test deliberately fails the handshake.
                        if let Ok(stream) = tls.accept(stream).await {
                            respond(stream, response, delay, callback, captured, notify).await;
                        }
                    } else {
                        respond(stream, response, delay, callback, captured, notify).await;
                    }
                });
            }
        });
        Self {
            base,
            requests,
            received,
            task,
        }
    }

    async fn wait_for(&self, count: usize) {
        tokio::time::timeout(Duration::from_secs(3), async {
            loop {
                let notification = self.received.notified();
                if self.requests.lock().unwrap().len() >= count {
                    break;
                }
                notification.await;
            }
        })
        .await
        .unwrap();
    }
}

async fn respond<S: AsyncRead + AsyncWrite + Unpin>(
    mut stream: S,
    response: Value,
    delay: Duration,
    callback: String,
    requests: Arc<Mutex<Vec<Recorded>>>,
    received: Arc<Notify>,
) {
    let mut bytes = Vec::new();
    while !bytes.ends_with(b"\r\n\r\n") {
        let mut byte = [0];
        if stream.read(&mut byte).await.unwrap_or(0) == 0 {
            return;
        }
        bytes.push(byte[0]);
    }
    let head = String::from_utf8(bytes).unwrap();
    let mut lines = head.split("\r\n");
    let mut first = lines.next().unwrap().split_whitespace();
    let method = first.next().unwrap().to_string();
    let target = first.next().unwrap().to_string();
    let headers: BTreeMap<_, _> = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(name, value)| (name.to_lowercase(), value.trim().to_string()))
        .collect();
    let length = headers
        .get("content-length")
        .map(|value| value.parse::<usize>().unwrap())
        .unwrap_or(0);
    let mut body = vec![0; length];
    if stream.read_exact(&mut body).await.is_err() {
        return;
    }
    requests.lock().unwrap().push(Recorded {
        method,
        target,
        headers,
        body: String::from_utf8(body).unwrap(),
    });
    received.notify_one();
    tokio::time::sleep(delay).await;
    // A missing status simulates an uncertain transport failure after receiving a request.
    let Some(status) = response.get("status").and_then(Value::as_u64) else {
        return;
    };
    let body = response
        .get("body")
        .map(Value::to_string)
        .unwrap_or_else(|| {
            response
                .get("text")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string()
        });
    let mut headers = format!(
        "HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\n",
        body.len()
    );
    if let Some(values) = response.get("headers").and_then(Value::as_object) {
        for (name, value) in values {
            let value = if name.eq_ignore_ascii_case("location") {
                callback.as_str()
            } else {
                value.as_str().unwrap()
            };
            headers.push_str(&format!("{name}: {value}\r\n"));
        }
    }
    let _ = stream
        .write_all(format!("{headers}\r\n{body}").as_bytes())
        .await;
}

fn client(server: &Server, retries: u32) -> PlatformClient {
    ApiClientBuilder::new(&server.base)
        .api_key("synthetic-app-key")
        .max_retries(retries)
        .build()
        .unwrap()
}

fn subset(expected: &Value, actual: &Value, schema: &Value) {
    match expected {
        Value::Object(properties) => {
            for (name, value) in properties {
                // Determine declared fields from OpenAPI, never from the generated model.
                if schema["properties"].is_object() && schema["properties"].get(name).is_none() {
                    continue;
                }
                // Native Serde may omit optional nulls on reserialization.
                let required = schema["required"]
                    .as_array()
                    .is_some_and(|fields| fields.contains(&json!(name)));
                let nullable = schema["properties"][name]["nullable"] == true;
                if value.is_null() && actual.get(name).is_none() && (!required || nullable) {
                    continue;
                }
                assert!(
                    actual.get(name).is_some(),
                    "missing typed property {name}; actual {actual}"
                );
                subset(value, &actual[name], &schema["properties"][name]);
            }
        }
        Value::Array(items) => {
            let actual = actual.as_array().expect("typed array");
            assert_eq!(items.len(), actual.len());
            for (expected, actual) in items.iter().zip(actual) {
                subset(expected, actual, &schema["items"]);
            }
        }
        _ => assert_eq!(actual, expected),
    }
}

#[tokio::test]
async fn installed_crate_conforms_to_all_shared_wire_cases() {
    let root = std::env::var("SDK_REPOSITORY").unwrap();
    let cases: Vec<Value> = serde_json::from_slice(
        &std::fs::read(format!("{root}/test/conformance/cases.json")).unwrap(),
    )
    .unwrap();
    let spec: Value =
        serde_json::from_slice(&std::fs::read(format!("{root}/openapi/openapi.json")).unwrap())
            .unwrap();
    let map: Value = serde_json::from_slice(
        &std::fs::read(format!(
            "{}/sdk-map.json",
            std::env::var("SDK_PACKAGE_DIR").unwrap()
        ))
        .unwrap(),
    )
    .unwrap();
    let mut operations = BTreeMap::new();
    for (path, item) in spec["paths"].as_object().unwrap() {
        for (method, operation) in item.as_object().unwrap() {
            if let Some(id) = operation["operationId"].as_str() {
                operations.insert(id, (method.to_uppercase(), path.as_str()));
            }
        }
    }
    assert_eq!(operations.len(), 33);
    assert_eq!(
        operations.keys().copied().collect::<BTreeSet<_>>(),
        map.as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect()
    );
    assert_eq!(
        operations.keys().copied().collect::<BTreeSet<_>>(),
        cases
            .iter()
            .map(|case| case["operationId"].as_str().unwrap())
            .collect()
    );
    for case in &cases {
        let id = case["operationId"].as_str().unwrap();
        println!("wire case: {}", case["name"]);
        assert_eq!(map[id]["httpMethod"], operations[id].0);
        assert_eq!(map[id]["path"], operations[id].1);
        let server = Server::start(vec![case["response"].clone()], Duration::ZERO, false).await;
        let mut builder = ApiClientBuilder::new(&server.base)
            .api_key(case["client"]["appKey"].as_str().unwrap())
            .max_retries(0);
        if let Some(token) = case["client"]["token"].as_str() {
            builder = builder.token(token);
        }
        if let Some(headers) = case["client"]["headers"].as_object() {
            for (name, value) in headers {
                builder = builder.custom_header(name, value.as_str().unwrap());
            }
        }
        let sdk = builder.build().unwrap();
        let mut options = RequestOptions::new();
        if let Some(headers) = case["requestHeaders"].as_object() {
            for (name, value) in headers {
                options = options.additional_header(name, value.as_str().unwrap());
            }
        }
        let result = dispatch::invoke(&sdk, id, &case["parameters"], Some(options)).await;
        let status = case["response"]["status"].as_u64().unwrap() as u16;
        if status >= 400 {
            let error = result.err().expect("API error");
            assert_eq!(error.status_code(), Some(status));
            let expected = case["response"]
                .get("body")
                .or_else(|| case["response"].get("text"))
                .unwrap();
            assert_eq!(error.body(), Some(expected));
        } else {
            let result = result.unwrap_or_else(|error| panic!("{id}: {error:?}"));
            if status == 303 {
                assert_eq!(result.status, Some(303));
                assert_eq!(
                    result.location,
                    Some(format!(
                        "{}/callback?data_exchange_status=granted",
                        server.base
                    ))
                );
            }
            let expected = case["response"]
                .get("body")
                .or_else(|| case["response"].get("text"))
                .cloned()
                .unwrap_or(Value::Null);
            let operation = &spec["paths"][operations[id].1][operations[id].0.to_lowercase()];
            let schema = &operation["responses"][status.to_string()]["content"]["application/json"]
                ["schema"];
            subset(&expected, &result.body, schema);
        }
        let requests = server.requests.lock().unwrap();
        assert_eq!(
            requests.len(),
            1,
            "{id}: unexpected retry or callback request"
        );
        let request = &requests[0];
        assert_eq!(request.method, case["expect"]["method"]);
        let url = Url::parse(&format!("{}{}", server.base, request.target)).unwrap();
        assert_eq!(url.path(), case["expect"]["path"], "{id}: path");
        let mut query = BTreeMap::<String, Vec<String>>::new();
        for (name, value) in url.query_pairs() {
            query
                .entry(name.into_owned())
                .or_default()
                .push(value.into_owned());
        }
        assert_eq!(
            serde_json::to_value(query).unwrap(),
            case["expect"]["query"],
            "{id}: query"
        );
        for (name, value) in case["expect"]["headers"].as_object().unwrap() {
            let header = request.headers.get(&name.to_lowercase());
            assert_eq!(
                header.map(String::as_str),
                value.as_str(),
                "{id}: header {name}"
            );
        }
        if let Some(body) = case["expect"].get("body") {
            assert_eq!(serde_json::from_str::<Value>(&request.body).unwrap(), *body);
        } else {
            assert!(request.body.is_empty());
        }
    }
    println!(
        "{} shared wire cases cover all {} operations",
        cases.len(),
        operations.len()
    );
}

#[test]
fn page_size_boundaries_and_typed_builders() {
    macro_rules! check {
        ($type:ty) => {
            for value in [1, 25, 99] {
                let size = serde_json::from_value::<$type>(json!(value)).unwrap();
                assert_eq!(serde_json::to_value(size).unwrap(), json!(value));
                assert_eq!(
                    value.to_string().parse::<$type>().unwrap().to_string(),
                    value.to_string()
                );
            }
            assert_eq!(
                serde_json::to_value("*".parse::<$type>().unwrap()).unwrap(),
                json!("*")
            );
            for value in [json!(0), json!(100), json!(-1), json!(1.5), json!("bad")] {
                assert!(serde_json::from_value::<$type>(value).is_err());
            }
        };
    }
    check!(BiblesCollectionGetRequestPageSize);
    check!(V1LanguagesCollectionGetRequestPageSize);
    check!(V1OrganizationsCollectionGetRequestPageSize);
    check!(V1OrganizationsBiblesCollectionGetRequestPageSize);
    assert!(
        CollectionGetQueryRequest::builder().build().is_err(),
        "required language ranges remain required"
    );
    assert!(CollectionGetQueryRequest::builder()
        .language_ranges_array(vec![Some("en".into())])
        .build()
        .unwrap()
        .fields_array
        .is_empty());
    assert!(V1LanguagesCollectionGetQueryRequest::builder()
        .build()
        .unwrap()
        .fields_array
        .is_empty());
    assert!(V1OrganizationsCollectionGetQueryRequest::builder()
        .build()
        .unwrap()
        .bible_ids_array
        .is_empty());
    assert!(V1OrganizationsBiblesCollectionGetQueryRequest::builder()
        .build()
        .unwrap()
        .fields_array
        .is_empty());
    assert!(V1SearchUnifiedCollectionGetQueryRequest::builder()
        .query("hope")
        .bible_id(206)
        .language_ranges_array(vec![Some("en".into())])
        .build()
        .unwrap()
        .fields_array
        .is_empty());
    assert!(V1HighlightsCollectionPostRequest::builder()
        .build()
        .is_err());
    let request = V1HighlightsCollectionPostRequest::builder()
        .request_id("synthetic-id")
        .highlight(
            V1HighlightsCollectionPostRequestHighlight::builder()
                .bible_id(206)
                .passage_id("PSA.23.4")
                .color("12abef")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    assert_eq!(request.highlight.passage_id, "PSA.23.4");
    let language: V1LanguagesCollectionGetResponseDataItem = serde_json::from_value(
        json!({"id":"sr-Latn","language":"sr","localized_name":"serbe","new_property":7}),
    )
    .unwrap();
    assert_eq!(language.localized_name.as_deref(), Some("serbe"));
    assert!(serde_json::to_value(language)
        .unwrap()
        .get("new_property")
        .is_none());
}

#[tokio::test]
async fn tls_transport_defaults_and_manual_approval_redirects() {
    let server = Server::start(
        vec![
            json!({"status":303,"headers":{"location":"local"}}),
            json!({"status":204}),
        ],
        Duration::ZERO,
        true,
    )
    .await;
    let certificate = reqwest::Certificate::from_pem(
        &std::fs::read(std::env::var("SDK_TLS_CERT").unwrap()).unwrap(),
    )
    .unwrap();
    let sdk = ApiClientBuilder::new(&server.base)
        .api_key("tls-app-key")
        .token("tls-access-token")
        .custom_header("Authorization", "Bearer SDK-header-must-not-leak")
        .custom_header("X-SDK-Custom", "preserved")
        .max_retries(0)
        .transport(move || {
            reqwest::Client::builder()
                .add_root_certificate(certificate.clone())
                .redirect(reqwest::redirect::Policy::limited(5))
        })
        .build()
        .unwrap();
    let response = sdk
        .data_exchange
        .approval_post_with_raw_response(
            &ApprovalPostQueryRequest::builder()
                .token("tls-exchange")
                .build()
                .unwrap(),
            None,
        )
        .await
        .unwrap();
    assert_eq!(response.status_code, 303);
    assert_eq!(server.requests.lock().unwrap().len(), 1);
    assert!(!server.requests.lock().unwrap()[0]
        .headers
        .contains_key("authorization"));
    assert_eq!(
        server.requests.lock().unwrap()[0].headers["x-sdk-custom"],
        "preserved"
    );
    sdk.bibles
        .collection_get(&CollectionGetQueryRequest::default(), None)
        .await
        .unwrap();
    {
        let requests = server.requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(
            requests[1].headers["authorization"],
            "Bearer tls-access-token"
        );
        assert_eq!(requests[1].headers["x-yvp-app-key"], "tls-app-key");
    }
    let error = client(&server, 0)
        .data_exchange
        .approval_post(&ApprovalPostQueryRequest::default(), None)
        .await
        .unwrap_err();
    assert!(
        matches!(error, ApiError::Network(_)),
        "untrusted certificate must be rejected"
    );
}

#[tokio::test]
async fn approval_auth_suppresses_additional_query_tokens_and_headers() {
    let server = Server::start(
        vec![json!({"status":303,"headers":{"location":"local"}})],
        Duration::ZERO,
        false,
    )
    .await;
    let mut config = ClientConfig {
        base_url: server.base.clone(),
        api_key: Some("app-key".into()),
        token: Some("must-not-leak".into()),
        max_retries: 0,
        ..Default::default()
    };
    // If auth suppression fails, the runtime attempts a token exchange instead.
    config.oauth_token_endpoint = Some("must-not-fetch-token".into());
    config.client_id = Some("client".into());
    config.client_secret = Some("synthetic-secret".into());
    config
        .custom_headers
        .insert("authorization".into(), "Bearer also-must-not-leak".into());
    let sdk = PlatformClient::new(config).unwrap();
    let options = RequestOptions::new()
        .additional_query_param("token", "exchange-through-options")
        .additional_header("AUTHORIZATION", "Bearer request-must-not-leak")
        .additional_header("X-SDK-Custom", "preserved");
    let response = sdk
        .data_exchange
        .approval_post_with_raw_response(
            &ApprovalPostQueryRequest::default(),
            Some(options.clone()),
        )
        .await
        .unwrap();
    assert_eq!(response.status_code, 303);
    sdk.data_exchange
        .approval_get_with_raw_response(
            &ApprovalGetQueryRequest::builder()
                .token("get-exchange")
                .build()
                .unwrap(),
            Some(options),
        )
        .await
        .unwrap();
    let requests = server.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    for request in requests.iter() {
        assert!(!request.headers.contains_key("authorization"));
        assert_eq!(request.headers["x-yvp-app-key"], "app-key");
        assert_eq!(request.headers["x-sdk-custom"], "preserved");
        assert!(request.target.contains("token=exchange-through-options"));
    }
}

#[tokio::test]
async fn retries_overrides_cancellation_and_timeouts() {
    for failure in [
        json!({"status":408}),
        json!({"status":429}),
        json!({"status":503}),
        json!({}),
    ] {
        let server =
            Server::start(vec![failure, json!({"status":204})], Duration::ZERO, false).await;
        client(&server, 1)
            .bibles
            .collection_get(&CollectionGetQueryRequest::default(), None)
            .await
            .unwrap();
        assert_eq!(server.requests.lock().unwrap().len(), 2);
    }
    let server = Server::start(vec![json!({"status":409})], Duration::ZERO, false).await;
    let error = client(&server, 3)
        .bibles
        .collection_get(&CollectionGetQueryRequest::default(), None)
        .await
        .unwrap_err();
    assert_eq!(error.status_code(), Some(409));
    assert_eq!(
        server.requests.lock().unwrap().len(),
        1,
        "non-retryable error"
    );
    let server = Server::start(vec![json!({"status":503})], Duration::ZERO, false).await;
    let sdk = client(&server, 3);
    let error = sdk
        .bibles
        .collection_get(
            &CollectionGetQueryRequest::default(),
            Some(RequestOptions::new().max_retries(0)),
        )
        .await
        .unwrap_err();
    assert_eq!(error.status_code(), Some(503));
    assert_eq!(server.requests.lock().unwrap().len(), 1);
    let server = Server::start(vec![json!({"status":503})], Duration::ZERO, false).await;
    let sdk = client(&server, 10);
    let task = tokio::spawn(async move {
        sdk.bibles
            .collection_get(&CollectionGetQueryRequest::default(), None)
            .await
    });
    server.wait_for(1).await;
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert_eq!(
        server.requests.lock().unwrap().len(),
        1,
        "aborted future must not retry"
    );
    let server = Server::start(vec![json!({"status":204})], Duration::from_secs(2), false).await;
    let sdk = ApiClientBuilder::new(&server.base)
        .timeout(Duration::from_millis(30))
        .max_retries(0)
        .build()
        .unwrap();
    let error = sdk
        .bibles
        .collection_get(&CollectionGetQueryRequest::default(), None)
        .await
        .unwrap_err();
    assert!(matches!(error, ApiError::Network(error) if error.is_timeout()));
    let server = Server::start(
        vec![json!({"status":204})],
        Duration::from_millis(100),
        false,
    )
    .await;
    let sdk = ApiClientBuilder::new(&server.base)
        .timeout(Duration::from_millis(30))
        .max_retries(0)
        .build()
        .unwrap();
    sdk.bibles
        .collection_get(
            &CollectionGetQueryRequest::default(),
            Some(RequestOptions::new().timeout_seconds(1)),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn normal_redirects_and_query_credentials_in_network_errors() {
    let server = Server::start(
        vec![
            json!({"status":302,"headers":{"location":"local"}}),
            json!({"status":204}),
        ],
        Duration::ZERO,
        false,
    )
    .await;
    client(&server, 0)
        .bibles
        .collection_get(&CollectionGetQueryRequest::default(), None)
        .await
        .unwrap();
    assert_eq!(
        server.requests.lock().unwrap().len(),
        2,
        "ordinary endpoint retains native redirects"
    );
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    drop(listener);
    let sdk = ApiClientBuilder::new(base)
        .api_key("secret-query-key")
        .max_retries(0)
        .build()
        .unwrap();
    let error = sdk
        .fonts
        .v1fonts_stylesheet_get(42, None)
        .await
        .unwrap_err();
    assert!(!format!("{error} {error:?}").contains("secret-query-key"));
    assert!(matches!(error, ApiError::Network(error) if error.is_connect()));
}

#[tokio::test]
async fn review_search_queries_preserve_literal_text() {
    let root = std::env::var("SDK_REPOSITORY").unwrap();
    let cases: Vec<Value> = serde_json::from_slice(
        &std::fs::read(format!("{root}/test/conformance/cases.json")).unwrap(),
    )
    .unwrap();
    for operation in [
        "v1.search_queries.collection_get",
        "v1.search_unified.collection_get",
        "v1.search_topics.collection_get",
        "v1.search_verses.collection_get",
    ] {
        let case = cases
            .iter()
            .find(|case| case["operationId"] == operation)
            .unwrap();
        for text in ["John3:16", "topic:\"hope,joy\" reference:PSA.23.4", ""] {
            let server = Server::start(vec![case["response"].clone()], Duration::ZERO, false).await;
            let mut parameters = case["parameters"].clone();
            parameters["query"] = json!(text);
            dispatch::invoke(&client(&server, 0), operation, &parameters, None)
                .await
                .unwrap();
            let requests = server.requests.lock().unwrap();
            assert_eq!(requests.len(), 1);
            let url = Url::parse(&format!("{}{}", server.base, requests[0].target)).unwrap();
            let mut query = BTreeMap::<String, Vec<String>>::new();
            for (name, value) in url.query_pairs() {
                query
                    .entry(name.into_owned())
                    .or_default()
                    .push(value.into_owned());
            }
            let mut expected = case["expect"]["query"].clone();
            expected["query"] = json!([text]);
            assert_eq!(
                serde_json::to_value(query).unwrap(),
                expected,
                "{operation}: {text}"
            );
        }
    }
}

#[tokio::test]
async fn review_approval_redirect_preserves_metadata_with_a_body() {
    for body in [
        "<html>Continue to your app</html>",
        "not JSON",
        "{\"redirect\":true}",
    ] {
        let server = Server::start(
            vec![json!({"status":303,"text":body,"headers":{"location":"local"}})],
            Duration::ZERO,
            false,
        )
        .await;
        let sdk = client(&server, 0);
        let request = ApprovalPostQueryRequest::builder()
            .token("synthetic-exchange")
            .build()
            .unwrap();
        let response = sdk
            .data_exchange
            .approval_post_with_raw_response(&request, None)
            .await
            .unwrap();
        assert_eq!(response.status_code, 303);
        assert_eq!(
            response.headers["location"],
            format!("{}/callback?data_exchange_status=granted", server.base)
        );
        sdk.data_exchange
            .approval_post(&request, None)
            .await
            .unwrap();
        assert_eq!(
            server.requests.lock().unwrap().len(),
            2,
            "no callback request or retry"
        );
    }
}

#[tokio::test]
async fn review_empty_required_text_and_optional_responses() {
    let server = Server::start(
        vec![json!({"status":200,"text":"","headers":{"content-type":"text/css"}})],
        Duration::ZERO,
        false,
    )
    .await;
    assert_eq!(
        client(&server, 0)
            .fonts
            .v1fonts_stylesheet_get(42, None)
            .await
            .unwrap(),
        ""
    );
    let server = Server::start(vec![json!({"status":200,"text":""})], Duration::ZERO, false).await;
    let sdk = client(&server, 0);
    assert!(sdk
        .data_exchange
        .approval_get(
            &ApprovalGetQueryRequest::builder()
                .token("synthetic-exchange")
                .build()
                .unwrap(),
            None
        )
        .await
        .unwrap()
        .is_none());
    assert!(sdk
        .bibles
        .collection_get(&CollectionGetQueryRequest::default(), None)
        .await
        .unwrap()
        .is_none());
    let server = Server::start(
        vec![json!({"status":401,"text":"denied"})],
        Duration::ZERO,
        false,
    )
    .await;
    let error = client(&server, 0)
        .data_exchange
        .approval_post(&ApprovalPostQueryRequest::default(), None)
        .await
        .unwrap_err();
    assert_eq!(error.status_code(), Some(401));
    assert_eq!(error.body(), Some(&json!("denied")));
}

#[tokio::test]
async fn review_dot_segments_are_rejected_before_sending() {
    let root = std::env::var("SDK_REPOSITORY").unwrap();
    let spec: Value =
        serde_json::from_slice(&std::fs::read(format!("{root}/openapi/openapi.json")).unwrap())
            .unwrap();
    let cases: Vec<Value> = serde_json::from_slice(
        &std::fs::read(format!("{root}/test/conformance/cases.json")).unwrap(),
    )
    .unwrap();
    let server = Server::start(vec![json!({"status":204})], Duration::ZERO, false).await;
    let sdk = client(&server, 0);
    let mut paths = 0;
    for item in spec["paths"].as_object().unwrap().values() {
        for operation in item.as_object().unwrap().values() {
            let Some(id) = operation["operationId"].as_str() else {
                continue;
            };
            for parameter in operation["parameters"].as_array().into_iter().flatten() {
                if parameter["in"] != "path" || parameter["schema"]["type"] != "string" {
                    continue;
                }
                paths += 1;
                let case = cases.iter().find(|case| case["operationId"] == id).unwrap();
                for segment in [".", ".."] {
                    let mut parameters = case["parameters"].clone();
                    parameters[parameter["name"].as_str().unwrap()] = json!(segment);
                    let error = dispatch::invoke(&sdk, id, &parameters, None)
                        .await
                        .err()
                        .expect("dot-segment error");
                    assert!(
                        matches!(error, ApiError::Configuration(_)),
                        "{id}: {error:?}"
                    );
                    assert!(
                        server.requests.lock().unwrap().is_empty(),
                        "{id}: invalid path must not be sent"
                    );
                }
            }
        }
    }
    assert_eq!(paths, 16, "all string path arguments are checked");
    sdk.bibles
        .books_chapters_collection_get(111, "%2E%2E", None)
        .await
        .unwrap();
    assert_eq!(
        server.requests.lock().unwrap()[0].target,
        "/v1/bibles/111/books/%252E%252E/chapters"
    );
    let case = cases
        .iter()
        .find(|case| case["operationId"] == "bibles_passages.resource_get")
        .unwrap();
    let server = Server::start(vec![case["response"].clone()], Duration::ZERO, false).await;
    let mut parameters = case["parameters"].clone();
    parameters["passage_id_path"] = json!("PSA.23.4");
    dispatch::invoke(
        &client(&server, 0),
        "bibles_passages.resource_get",
        &parameters,
        None,
    )
    .await
    .unwrap();
    let url = Url::parse(&format!(
        "{}{}",
        server.base,
        server.requests.lock().unwrap()[0].target
    ))
    .unwrap();
    assert!(url.path().ends_with("/passages/PSA.23.4"));
}
