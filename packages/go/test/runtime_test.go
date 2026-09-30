package consumer_test

import (
	"context"
	"errors"
	"io"
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	platform "github.com/cameronapak/Platform-SDK-TS/packages/go"
	"github.com/cameronapak/Platform-SDK-TS/packages/go/client"
	"github.com/cameronapak/Platform-SDK-TS/packages/go/core"
	"github.com/cameronapak/Platform-SDK-TS/packages/go/option"
)

func TestTokenSupplierSuppressionFailureAndOverrides(t *testing.T) {
	var requests, tokens atomic.Int32
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		requests.Add(1)
		if r.URL.Path == "/data-exchange" {
			if r.Header.Get("Authorization") != "" {
				t.Error("approval leaked bearer auth")
			}
			w.Header().Set("Location", "/callback")
			w.WriteHeader(303)
			return
		}
		if r.Header.Get("Authorization") != "Bearer override" {
			t.Error("incorrect request auth")
		}
		w.WriteHeader(204)
	}))
	defer server.Close()
	unavailable := errors.New("token unavailable")
	c := client.NewPlatformClient(option.WithBaseURL(server.URL), option.WithYvpAppKey("synthetic-app-key"),
		option.WithTokenFunc(func() (string, error) { tokens.Add(1); return "", unavailable }), option.WithoutRetries())
	_, err := c.DataExchange.WithRawResponse.ApprovalGet(context.Background(), &platform.DataExchangeApprovalGetRequest{Token: "get-token"})
	if err != nil {
		t.Fatal(err)
	}
	err = c.DataExchange.ApprovalPost(context.Background(), &platform.DataExchangeApprovalPostRequest{Token: platform.String("post-token")},
		option.WithToken("must-not-leak"), option.WithHTTPHeader(http.Header{"authorization": {"Bearer must-not-leak"}}))
	if err != nil {
		t.Fatal(err)
	}
	if tokens.Load() != 0 || requests.Load() != 2 {
		t.Fatal("exchange approval acquired OAuth")
	}
	_, err = c.Highlights.V1HighlightsCollectionGet(context.Background(), &platform.V1HighlightsCollectionGetRequest{})
	if !errors.Is(err, unavailable) || requests.Load() != 2 || tokens.Load() != 1 {
		t.Fatalf("swallowed token failure: %v", err)
	}
	err = c.DataExchange.ApprovalPost(context.Background(), &platform.DataExchangeApprovalPostRequest{})
	if !errors.Is(err, unavailable) || requests.Load() != 2 {
		t.Fatal("tokenless approval bypassed token supplier")
	}
	err = c.Highlights.V1HighlightsResourceDelete(context.Background(), &platform.V1HighlightsResourceDeleteRequest{PassageIDPath: "JHN.3.16", BibleID: 111}, option.WithToken("override"))
	if err != nil || requests.Load() != 3 || tokens.Load() != 2 {
		t.Fatalf("request token override did not bypass supplier: %v", err)
	}
}

func TestApprovalPostQueryTokenSuppressesBearer(t *testing.T) {
	for _, tc := range []struct {
		name     string
		values   []string
		supplier bool
		suppress bool
	}{
		{"static-token", []string{"exchange+/="}, false, true},
		{"failing-supplier", []string{"exchange+/="}, true, true},
		{"empty-token-value", []string{""}, true, true},
		{"nil-values", nil, true, false},
		{"empty-values", []string{}, true, false},
	} {
		t.Run(tc.name, func(t *testing.T) {
			var requests, tokens atomic.Int32
			query := url.Values{"token": tc.values}
			server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				requests.Add(1)
				if r.URL.RawQuery != query.Encode() {
					t.Errorf("query: %s, want %s", r.URL.RawQuery, query.Encode())
				}
				if auth := r.Header.Get("Authorization"); auth != "" {
					t.Errorf("approval emitted bearer auth: %q", auth)
				}
				w.Header().Set("Location", "/callback")
				w.WriteHeader(http.StatusSeeOther)
			}))
			defer server.Close()
			unavailable := errors.New("token unavailable")
			var auth option.RequestOption = option.WithToken("must-not-leak")
			if tc.supplier {
				auth = option.WithTokenFunc(func() (string, error) {
					tokens.Add(1)
					return "", unavailable
				})
			}
			c := client.NewPlatformClient(option.WithBaseURL(server.URL), option.WithYvpAppKey("app-key"), auth,
				option.WithHTTPHeader(http.Header{"authorization": {"Bearer must-not-leak"}}), option.WithoutRetries())
			err := c.DataExchange.ApprovalPost(context.Background(), &platform.DataExchangeApprovalPostRequest{},
				option.WithQueryParameters(query))
			if tc.suppress {
				if err != nil || requests.Load() != 1 || tokens.Load() != 0 {
					t.Fatalf("approval: %v, requests=%d, supplier calls=%d", err, requests.Load(), tokens.Load())
				}
			} else if !errors.Is(err, unavailable) || requests.Load() != 0 || tokens.Load() != 1 {
				t.Fatalf("tokenless approval: %v, requests=%d, supplier calls=%d", err, requests.Load(), tokens.Load())
			}
		})
	}
}

func TestPageSizeConstructors(t *testing.T) {
	constructors := map[string]func(string) (string, error){
		"bibles": func(s string) (string, error) {
			value, err := platform.NewBiblesCollectionGetRequestPageSizeFromString(s)
			return string(value), err
		},
		"languages": func(s string) (string, error) {
			value, err := platform.NewV1LanguagesCollectionGetRequestPageSizeFromString(s)
			return string(value), err
		},
		"organization-bibles": func(s string) (string, error) {
			value, err := platform.NewV1OrganizationsBiblesCollectionGetRequestPageSizeFromString(s)
			return string(value), err
		},
		"organizations": func(s string) (string, error) {
			value, err := platform.NewV1OrganizationsCollectionGetRequestPageSizeFromString(s)
			return string(value), err
		},
	}
	for name, construct := range constructors {
		t.Run(name, func(t *testing.T) {
			for _, size := range []string{"*", "1", "25", "99", "+25", "025"} {
				value, err := construct(size)
				if err != nil || value != size {
					t.Errorf("valid page size %q: value=%q, err=%v", size, value, err)
				}
			}
			for _, size := range []string{"", "0", "100", "-1", "1.5", "all", " 25", "99999999999999999999"} {
				if value, err := construct(size); err == nil {
					t.Errorf("invalid page size %q accepted as %q", size, value)
				}
			}
		})
	}
}

func TestRetryLimitsBodyReplayAndClientDefaults(t *testing.T) {
	var requests atomic.Int32
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		body, _ := io.ReadAll(r.Body)
		if r.Method != "POST" || string(body) != `{"requested_permissions":["highlights"]}` {
			t.Errorf("retry changed body: %s %s", r.Method, body)
		}
		w.Header().Set("Retry-After", "1")
		if requests.Add(1) == 2 {
			_, _ = io.WriteString(w, `{"token":"created","token_type":"data_exchange","expires_in":300}`)
			return
		}
		w.WriteHeader(503)
		_, _ = io.WriteString(w, `{"message":"unavailable"}`)
	}))
	defer server.Close()
	c := client.NewPlatformClient(option.WithBaseURL(server.URL), option.WithYvpAppKey("app-key"), option.WithoutRetries())
	body := &platform.DataExchangeTokenPostRequest{RequestedPermissions: []string{"highlights"}}
	_, err := c.DataExchange.TokenPost(context.Background(), body)
	var apiError *core.APIError
	if !errors.As(err, &apiError) || apiError.StatusCode != 503 || requests.Load() != 1 {
		t.Fatalf("client WithoutRetries ignored: %v", err)
	}
	requests.Store(0)
	result, err := c.DataExchange.TokenPost(context.Background(), body, option.WithMaxAttempts(2))
	if err != nil || result.Token != "created" || requests.Load() != 2 {
		t.Fatalf("request retry override failed: %v", err)
	}
	requests.Store(3)
	_, err = c.DataExchange.TokenPost(context.Background(), body, option.WithMaxAttempts(2))
	if !errors.As(err, &apiError) || requests.Load() != 5 {
		t.Fatalf("wrong total attempts: %d, %v", requests.Load(), err)
	}
}

func TestContextCancellationDuringRequestAndBackoff(t *testing.T) {
	for _, backoff := range []bool{false, true} {
		t.Run(map[bool]string{false: "in-flight", true: "retry-backoff"}[backoff], func(t *testing.T) {
			started := make(chan struct{}, 1)
			var requests atomic.Int32
			server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				requests.Add(1)
				if backoff {
					w.Header().Set("Retry-After", "30")
					w.WriteHeader(503)
					_, _ = io.WriteString(w, "busy")
					started <- struct{}{}
					return
				}
				started <- struct{}{}
				<-r.Context().Done()
			}))
			defer server.Close()
			c := client.NewPlatformClient(option.WithBaseURL(server.URL), option.WithYvpAppKey("app-key"), option.WithMaxAttempts(3))
			ctx, cancel := context.WithCancel(context.Background())
			defer cancel()
			done := make(chan error, 1)
			go func() { _, err := c.Fonts.V1FontsCollectionGet(ctx); done <- err }()
			select {
			case <-started:
			case <-time.After(time.Second):
				t.Fatal("request did not start")
			}
			// Let the response reach the retry loop, then cancel its long backoff.
			if backoff {
				time.Sleep(30 * time.Millisecond)
			}
			cancel()
			select {
			case err := <-done:
				if !errors.Is(err, context.Canceled) || requests.Load() != 1 {
					t.Fatalf("cancellation: %v, requests=%d", err, requests.Load())
				}
			case <-time.After(time.Second):
				t.Fatal("cancellation did not interrupt request/backoff")
			}
			_, err := c.Fonts.V1FontsCollectionGet(ctx)
			if !errors.Is(err, context.Canceled) || requests.Load() != 1 {
				t.Fatal("already-canceled context sent another request")
			}
		})
	}
}

func TestDeadlineAndInjectedTransportTimeout(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) { <-r.Context().Done() }))
	defer server.Close()
	for _, transportTimeout := range []bool{false, true} {
		transport := server.Client()
		ctx := context.Background()
		if transportTimeout {
			transport.Timeout = 30 * time.Millisecond
		} else {
			var cancel context.CancelFunc
			ctx, cancel = context.WithTimeout(ctx, 30*time.Millisecond)
			defer cancel()
		}
		c := client.NewPlatformClient(option.WithBaseURL(server.URL), option.WithYvpAppKey("app-key"), option.WithHTTPClient(transport), option.WithoutRetries())
		_, err := c.Fonts.V1FontsCollectionGet(ctx)
		if !errors.Is(err, context.DeadlineExceeded) {
			t.Fatalf("lost timeout/deadline: %v", err)
		}
	}
}

type failingTransport struct{}

func (failingTransport) RoundTrip(*http.Request) (*http.Response, error) {
	return nil, io.ErrUnexpectedEOF
}

func TestTransportErrorsRedactQueryCredentials(t *testing.T) {
	c := client.NewPlatformClient(option.WithBaseURL("http://unused.invalid"), option.WithYvpAppKey("private-font-key"),
		option.WithHTTPClient(&http.Client{Transport: failingTransport{}}), option.WithoutRetries())
	_, err := c.Fonts.V1FontsStylesheetGet(context.Background(), &platform.V1FontsStylesheetGetRequest{FontID: 42})
	if !errors.Is(err, io.ErrUnexpectedEOF) || strings.Contains(err.Error(), "private-font-key") {
		t.Fatalf("credential leak or lost cause: %v", err)
	}
	_, err = c.DataExchange.ApprovalGet(context.Background(), &platform.DataExchangeApprovalGetRequest{Token: "private-exchange-token", XYvpAppKey: platform.String("private-browser-key")})
	if !errors.Is(err, io.ErrUnexpectedEOF) || strings.Contains(err.Error(), "private-exchange-token") || strings.Contains(err.Error(), "private-browser-key") {
		t.Fatalf("exchange credential leak: %v", err)
	}
	var transportError *url.Error
	if !errors.As(err, &transportError) {
		t.Fatal("lost transport error type")
	}
}

func TestConcurrentApprovalAndNormalRedirectPolicy(t *testing.T) {
	var callbackRequests atomic.Int32
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/callback" {
			callbackRequests.Add(1)
			_, _ = io.WriteString(w, `{"data":[]}`)
			return
		}
		if r.URL.Path == "/data-exchange" {
			w.Header().Set("Location", "/callback")
			w.WriteHeader(303)
			return
		}
		w.Header().Set("Location", "/callback")
		w.WriteHeader(302)
	}))
	defer server.Close()
	transport := server.Client()
	clientHeaders := http.Header{"accept-language": {"es"}}
	c := client.NewPlatformClient(option.WithBaseURL(server.URL), option.WithYvpAppKey("app-key"), option.WithHTTPClient(transport),
		option.WithHTTPHeader(clientHeaders), option.WithoutRetries())
	var group sync.WaitGroup
	for range 8 {
		group.Add(1)
		go func() {
			defer group.Done()
			result, err := c.DataExchange.WithRawResponse.ApprovalPost(context.Background(), &platform.DataExchangeApprovalPostRequest{Token: platform.String("exchange")})
			if err != nil || result.StatusCode != 303 {
				t.Errorf("approval response: %v", err)
			}
			_, err = c.Fonts.V1FontsCollectionGet(context.Background())
			if err != nil {
				t.Error(err)
			}
		}()
	}
	group.Wait()
	if callbackRequests.Load() != 8 || transport.CheckRedirect != nil || clientHeaders.Get("accept-language") != "" || clientHeaders["accept-language"][0] != "es" {
		t.Fatal("approval changed caller-owned transport/headers or followed callback")
	}
}

// These calls compile against the consumer module's public API, with no
// reflection or generated inventory providing their request/response types.
func TestTypedRequestsAndManualPagination(t *testing.T) {
	var requests atomic.Int32
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		query := r.URL.Query()
		if query.Get("page_size") != "*" || strings.Join(query["language_ranges[]"], ",") != "es-419,sr-Latn" {
			t.Error("lost wildcard or repeated filters")
		}
		if requests.Add(1) == 1 {
			if query.Get("page_token") != "" {
				t.Error("unexpected first page token")
			}
			_, _ = io.WriteString(w, `{"data":[{"id":111}],"next_page_token":"next+2/="}`)
		} else {
			if query.Get("page_token") != "next+2/=" {
				t.Error("lost returned page token")
			}
			_, _ = io.WriteString(w, `{"data":[{"id":206}]}`)
		}
	}))
	defer server.Close()
	c := client.NewPlatformClient(option.WithBaseURL(server.URL), option.WithYvpAppKey("app-key"), option.WithoutRetries())
	request := &platform.BiblesCollectionGetRequest{LanguageRangesArray: []*string{platform.String("es-419"), platform.String("sr-Latn")},
		FieldsArray: []*string{platform.String("id")}, PageSize: platform.BiblesCollectionGetRequestPageSizeAll.Ptr()}
	var first *platform.BiblesCollectionGetResponse
	first, err := c.Bibles.CollectionGet(context.Background(), request)
	if err != nil || *first.Data[0].ID != 111 {
		t.Fatalf("first page: %v", err)
	}
	request.PageToken = first.NextPageToken
	second, err := c.Bibles.CollectionGet(context.Background(), request)
	if err != nil || *second.Data[0].ID != 206 || second.NextPageToken != nil {
		t.Fatalf("second page: %v", err)
	}
	_ = &platform.V1HighlightsCollectionPostRequest{RequestID: "20000000-0000-4000-8000-000000000001",
		Highlight: &platform.V1HighlightsCollectionPostRequestHighlight{BibleID: 111, PassageID: "JHN.3.16", Color: "12abef"}}
}
