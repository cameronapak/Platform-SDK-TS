use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("BadRequestError: Bad request - {message}")]
    BadRequestError {
        body: serde_json::Value,
        message: String,
        field: Option<String>,
        details: Option<String>,
    },
    #[error("UnauthorizedError: Authentication failed - {message}")]
    UnauthorizedError {
        body: serde_json::Value,
        message: String,
        auth_type: Option<String>,
    },
    #[error("NotFoundError: Resource not found - {message}")]
    NotFoundError {
        body: serde_json::Value,
        message: String,
        resource_id: Option<String>,
        resource_type: Option<String>,
    },
    #[error("UnprocessableEntityError: Unprocessable entity - {message}")]
    UnprocessableEntityError {
        body: serde_json::Value,
        message: String,
        field: Option<String>,
        validation_error: Option<String>,
    },
    #[error("HTTP error {status}: {message}")]
    Http {
        status: u16,
        message: String,
        body: serde_json::Value,
    },
    #[error("Network error: {0}")]
    Network(reqwest::Error),
    #[error("Request executor error: {0}")]
    Executor(Box<dyn std::error::Error + Send + Sync>),
    #[error("Serialization error: {0}")]
    Serialization(serde_json::Error),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Invalid header value")]
    InvalidHeader,
    #[error("Could not clone request for retry")]
    RequestClone,
    #[error("SSE stream terminated")]
    StreamTerminated,
    #[error("SSE stream timed out waiting for next event")]
    StreamTimeout,
    #[error("SSE parse error: {0}")]
    SseParseError(String),
}

impl ApiError {
    pub fn network(error: reqwest::Error) -> Self {
        Self::Network(error.without_url())
    }

    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::BadRequestError { .. } => Some(400),
            Self::UnauthorizedError { .. } => Some(401),
            Self::NotFoundError { .. } => Some(404),
            Self::UnprocessableEntityError { .. } => Some(422),
            Self::Http { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub fn body(&self) -> Option<&serde_json::Value> {
        match self {
            Self::BadRequestError { body, .. }
            | Self::UnauthorizedError { body, .. }
            | Self::NotFoundError { body, .. }
            | Self::UnprocessableEntityError { body, .. }
            | Self::Http { body, .. } => Some(body),
            _ => None,
        }
    }

    pub fn from_response(status_code: u16, body: Option<&str>) -> Self {
        let response_body = body
            .map(|text| {
                serde_json::from_str(text)
                    .unwrap_or_else(|_| serde_json::Value::String(text.into()))
            })
            .unwrap_or(serde_json::Value::Null);
        match status_code {
            400 => {
                // Parse error body for BadRequestError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::BadRequestError {
                            body: response_body.clone(),
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            field: parsed
                                .get("field")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            details: parsed
                                .get("details")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::BadRequestError {
                    body: response_body.clone(),
                    message: body.unwrap_or("Unknown error").to_string(),
                    field: None,
                    details: None,
                };
            }
            401 => {
                // Parse error body for UnauthorizedError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::UnauthorizedError {
                            body: response_body.clone(),
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            auth_type: parsed
                                .get("authType")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::UnauthorizedError {
                    body: response_body.clone(),
                    message: body.unwrap_or("Unknown error").to_string(),
                    auth_type: None,
                };
            }
            404 => {
                // Parse error body for NotFoundError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::NotFoundError {
                            body: response_body.clone(),
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            resource_id: parsed
                                .get("resourceId")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            resource_type: parsed
                                .get("resourceType")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::NotFoundError {
                    body: response_body.clone(),
                    message: body.unwrap_or("Unknown error").to_string(),
                    resource_id: None,
                    resource_type: None,
                };
            }
            422 => {
                // Parse error body for UnprocessableEntityError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::UnprocessableEntityError {
                            body: response_body.clone(),
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            field: parsed
                                .get("field")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            validation_error: parsed
                                .get("validationError")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::UnprocessableEntityError {
                    body: response_body.clone(),
                    message: body.unwrap_or("Unknown error").to_string(),
                    field: None,
                    validation_error: None,
                };
            }
            _ => Self::Http {
                body: response_body,
                status: status_code,
                message: body.unwrap_or("Unknown error").to_string(),
            },
        }
    }
}

/// Error returned when a required field was not set on a builder.
#[derive(Debug)]
pub struct BuildError {
    field: &'static str,
}

impl BuildError {
    pub fn missing_field(field: &'static str) -> Self {
        Self { field }
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` was not set but is required", self.field)
    }
}

impl std::error::Error for BuildError {}
