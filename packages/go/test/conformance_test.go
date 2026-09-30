package consumer_test

import (
	"context"
	"encoding/json"
	"errors"
	"io"
	"net/http"
	"net/http/httptest"
	"net/url"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"sync"
	"testing"

	"github.com/cameronapak/Platform-SDK-TS/packages/go/client"
	"github.com/cameronapak/Platform-SDK-TS/packages/go/core"
	"github.com/cameronapak/Platform-SDK-TS/packages/go/option"
)

type mapping struct {
	Accessor, Method, HTTPMethod, Path string
	RequestType                        *string
	Parameters                         map[string]string
}

type wireCase struct {
	Name, OperationID string
	Client            struct {
		AppKey, Token string
		Headers       map[string]string
	}
	Parameters     map[string]json.RawMessage
	RequestHeaders map[string]string
	Response       struct {
		Status  int
		Headers map[string]string
		Body    json.RawMessage
		Text    *string
	}
	Expect struct {
		Method, Path string
		Query        url.Values
		Headers      map[string]*string
		Body         json.RawMessage
	}
}

func readJSON(t *testing.T, path string, value any) {
	t.Helper()
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(data, value); err != nil {
		t.Fatal(err)
	}
}

func sameJSON(t *testing.T, actual, expected any) {
	t.Helper()
	encode := func(value any) any {
		data, err := json.Marshal(value)
		if err != nil {
			t.Fatal(err)
		}
		var normalized any
		if err := json.Unmarshal(data, &normalized); err != nil {
			t.Fatal(err)
		}
		return normalized
	}
	if a, e := encode(actual), encode(expected); !reflect.DeepEqual(a, e) {
		t.Fatalf("JSON mismatch\nactual: %#v\nexpected: %#v", a, e)
	}
}

func headers(values map[string]string) http.Header {
	result := make(http.Header)
	for name, value := range values {
		result[name] = []string{value}
	}
	return result
}

// Check public model fields rather than requiring JSON re-serialization to
// preserve optional nulls. Fern retains unknown response values separately.
func checkModel(t *testing.T, actual reflect.Value, expected any) {
	t.Helper()
	if actual.IsValid() && actual.Kind() == reflect.Pointer && !actual.IsNil() && actual.Elem().Kind() == reflect.Struct {
		object, ok := expected.(map[string]any)
		if !ok {
			t.Fatalf("expected object, got %#v", expected)
		}
		fields := actual.Elem()
		extras := actual.Interface().(interface{ GetExtraProperties() map[string]any }).GetExtraProperties()
		for name, value := range object {
			found := false
			for index := 0; index < fields.NumField(); index++ {
				if strings.Split(fields.Type().Field(index).Tag.Get("json"), ",")[0] == name {
					checkModel(t, fields.Field(index), value)
					found = true
					break
				}
			}
			if !found {
				extra, exists := extras[name]
				if !exists {
					// Literal fields use generated accessors and custom marshaling.
					data, err := json.Marshal(actual.Interface())
					if err != nil {
						t.Fatal(err)
					}
					var serialized map[string]any
					if err := json.Unmarshal(data, &serialized); err != nil {
						t.Fatal(err)
					}
					extra, exists = serialized[name]
				}
				if !exists {
					t.Fatalf("response lost property %s", name)
				}
				sameJSON(t, extra, value)
			}
		}
		return
	}
	if actual.IsValid() && actual.Kind() == reflect.Slice {
		items, ok := expected.([]any)
		if !ok || actual.Len() != len(items) {
			t.Fatalf("array mismatch: %v, %#v", actual, expected)
		}
		for index, value := range items {
			checkModel(t, actual.Index(index), value)
		}
		return
	}
	var value any
	if actual.IsValid() {
		value = actual.Interface()
	}
	sameJSON(t, value, expected)
}

func TestConsumerInventoryAndSharedWireCases(t *testing.T) {
	var inventory map[string]mapping
	readJSON(t, filepath.Join(os.Getenv("SDK_PACKAGE_DIR"), "sdk-map.json"), &inventory)
	var cases []wireCase
	readJSON(t, filepath.Join(os.Getenv("SDK_REPOSITORY"), "test/conformance/cases.json"), &cases)
	var spec struct {
		Paths map[string]map[string]json.RawMessage
	}
	readJSON(t, filepath.Join(os.Getenv("SDK_REPOSITORY"), "openapi/openapi.json"), &spec)
	operations := make(map[string]bool)
	for path, item := range spec.Paths {
		for verb, raw := range item {
			if !strings.Contains(" get post delete put patch head options trace ", " "+verb+" ") {
				continue
			}
			var operation struct{ OperationID string }
			if err := json.Unmarshal(raw, &operation); err != nil {
				t.Fatal(err)
			}
			entry, ok := inventory[operation.OperationID]
			if !ok || entry.Path != path || entry.HTTPMethod != strings.ToUpper(verb) {
				t.Fatalf("inventory mismatch for %s: %+v", operation.OperationID, entry)
			}
			operations[operation.OperationID] = true
		}
	}
	if len(operations) != 33 || len(inventory) != len(operations) {
		t.Fatal("operation inventory is incomplete")
	}
	covered := make(map[string]bool)
	for _, item := range cases {
		covered[item.OperationID] = true
		t.Run(item.Name, func(t *testing.T) {
			var mu sync.Mutex
			var calls []struct {
				Method, Path string
				Query        url.Values
				Header       http.Header
				Body         []byte
			}
			server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				body, err := io.ReadAll(r.Body)
				if err != nil {
					t.Error(err)
				}
				mu.Lock()
				calls = append(calls, struct {
					Method, Path string
					Query        url.Values
					Header       http.Header
					Body         []byte
				}{
					r.Method, r.URL.EscapedPath(), r.URL.Query(), r.Header.Clone(), body,
				})
				mu.Unlock()
				for name, value := range item.Response.Headers {
					w.Header().Set(name, value)
				}
				// Redirect to this server so following it is observable, never external.
				if item.Response.Status == 303 {
					w.Header().Set("Location", "/callback?approved=true")
				}
				w.WriteHeader(item.Response.Status)
				if len(item.Response.Body) != 0 {
					_, _ = w.Write(item.Response.Body)
				}
				if item.Response.Text != nil {
					_, _ = io.WriteString(w, *item.Response.Text)
				}
			}))
			defer server.Close()
			transport := server.Client() // Default redirect policy deliberately follows 303s.
			c := client.NewPlatformClient(option.WithBaseURL(server.URL), option.WithYvpAppKey(item.Client.AppKey),
				option.WithToken(item.Client.Token), option.WithHTTPHeader(headers(item.Client.Headers)),
				option.WithHTTPClient(transport), option.WithoutRetries())
			entry := inventory[item.OperationID]
			owner := reflect.ValueOf(c).Elem().FieldByName(entry.Accessor)
			if item.Response.Status == 303 {
				owner = owner.Elem().FieldByName("WithRawResponse")
			}
			method := owner.MethodByName(entry.Method)
			if !method.IsValid() {
				t.Fatalf("missing public method %s.%s", entry.Accessor, entry.Method)
			}
			arguments := []reflect.Value{reflect.ValueOf(context.Background())}
			if entry.RequestType != nil {
				request := reflect.New(method.Type().In(1).Elem())
				if request.Elem().Type().Name() != *entry.RequestType {
					t.Fatal("request type mismatch")
				}
				for name, raw := range item.Parameters {
					field := request.Elem().FieldByName(entry.Parameters[name])
					if !field.IsValid() {
						t.Fatalf("missing request field %s", name)
					}
					// Query values are textual. The pinned Go generator represents
					// mixed numeric/wildcard page sizes as named strings.
					if name == "page_size" && field.Type().Elem().Kind() == reflect.String && raw[0] != '"' {
						var err error
						raw, err = json.Marshal(string(raw))
						if err != nil {
							t.Fatal(err)
						}
					}
					if err := json.Unmarshal(raw, field.Addr().Interface()); err != nil {
						t.Fatalf("%s: %v", name, err)
					}
				}
				arguments = append(arguments, request)
			}
			arguments = append(arguments, reflect.ValueOf([]option.RequestOption{option.WithHTTPHeader(headers(item.RequestHeaders))}))
			results := method.CallSlice(arguments)
			last := results[len(results)-1]
			var err error
			if !last.IsNil() {
				err = last.Interface().(error)
			}
			if item.Response.Status >= 300 && item.Response.Status != 303 {
				var apiError *core.APIError
				if !errors.As(err, &apiError) {
					t.Fatalf("expected APIError, got %v", err)
				}
				if apiError.StatusCode != item.Response.Status {
					t.Fatal(apiError.StatusCode)
				}
				if len(item.Response.Body) != 0 {
					sameJSON(t, apiError.Body, item.Response.Body)
				}
				if item.Response.Text != nil && apiError.Body != *item.Response.Text {
					t.Fatal(apiError.Body)
				}
				for name, value := range item.Response.Headers {
					if apiError.Header.Get(name) != value {
						t.Fatalf("lost error header %s", name)
					}
				}
			} else {
				if err != nil {
					t.Fatal(err)
				}
				var returned any
				if len(results) == 2 {
					returned = results[0].Interface()
				}
				if item.Response.Status == 303 {
					raw := results[0].Elem()
					if raw.FieldByName("StatusCode").Int() != 303 {
						t.Fatal("lost 303 status")
					}
					if raw.FieldByName("Header").Interface().(http.Header).Get("Location") != "/callback?approved=true" {
						t.Fatal("lost callback location")
					}
					returned = raw.FieldByName("Body").Interface()
				}
				if len(item.Response.Body) != 0 {
					var expected any
					if err := json.Unmarshal(item.Response.Body, &expected); err != nil {
						t.Fatal(err)
					}
					checkModel(t, reflect.ValueOf(returned), expected)
				} else if item.Response.Text != nil {
					if returned != *item.Response.Text {
						t.Fatal(returned)
					}
				} else {
					sameJSON(t, returned, nil)
				}
			}
			mu.Lock()
			defer mu.Unlock()
			if len(calls) != 1 {
				t.Fatalf("%d requests, unexpected callback or retry", len(calls))
			}
			call := calls[0]
			if call.Method != item.Expect.Method || call.Path != item.Expect.Path {
				t.Fatalf("%s %s", call.Method, call.Path)
			}
			if !reflect.DeepEqual(call.Query, item.Expect.Query) {
				t.Fatalf("query: %#v, want %#v", call.Query, item.Expect.Query)
			}
			for name, expected := range item.Expect.Headers {
				if expected == nil {
					if _, ok := call.Header[http.CanonicalHeaderKey(name)]; ok {
						t.Fatalf("unexpected header %s", name)
					}
				} else if call.Header.Get(name) != *expected || len(call.Header.Values(name)) != 1 {
					t.Fatalf("header %s: %v", name, call.Header.Values(name))
				}
			}
			if len(item.Expect.Body) != 0 {
				sameJSON(t, json.RawMessage(call.Body), item.Expect.Body)
			} else if len(call.Body) != 0 {
				t.Fatalf("unexpected body %s", call.Body)
			}
		})
	}
	if !reflect.DeepEqual(covered, operations) {
		t.Fatal("shared cases do not cover every operation")
	}
}
