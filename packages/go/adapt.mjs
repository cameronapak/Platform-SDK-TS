import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { replaceExactly } from '../../scripts/generation.mjs';

// Adapt only contracts the pinned generator cannot express. Counted replacements
// deliberately fail on output drift instead of silently changing HTTP behavior.
export function adaptGo(directory) {
  function edit(file, transform) {
    const path = join(directory, file);
    const source = readFileSync(path, 'utf8');
    const replace = (text, search, replacement, count = 1) =>
      replaceExactly(text, search, replacement, count, path);
    writeFileSync(path, transform(source, replace));
  }

  edit('core/request_option.go', (source, replace) => {
    source = replace(source, '\tfmt "fmt"\n', '');
    const start = source.indexOf('func (r *RequestOptions) ToHeader() http.Header {');
    const end = source.indexOf('// BaseURLOption implements', start);
    if (start < 0 || end < 0) throw new Error('Missing Go header conversion boundary');
    return source.slice(0, start) + `func (r *RequestOptions) ToHeader(overrides *RequestOptions, includeAuth bool) (http.Header, error) {
	header := make(http.Header)
	for _, values := range []http.Header{r.HTTPHeader, overrides.HTTPHeader} {
		for name, value := range values {
			header[http.CanonicalHeaderKey(name)] = append([]string(nil), value...)
		}
	}
	appKey := r.YvpAppKey
	if overrides.YvpAppKey != "" {
		appKey = overrides.YvpAppKey
	}
	header.Set("X-YVP-App-Key", appKey)
	if !includeAuth {
		header.Del("Authorization")
		return header, nil
	}
	token, tokenFunc := r.Token, r.TokenFunc
	if overrides.Token != "" || overrides.TokenFunc != nil {
		token, tokenFunc = overrides.Token, overrides.TokenFunc
	}
	if token == "" && tokenFunc != nil {
		var err error
		token, err = tokenFunc()
		if err != nil {
			return nil, err
		}
	}
	if token != "" {
		header.Set("Authorization", "Bearer "+token)
	}
	return header, nil
}

` + source.slice(end);
  });

  const resources = {
    apps: 1, bibles: 10, dataexchange: 3, fonts: 3, highlights: 3,
    languages: 2, licenses: 1, organizations: 3, permissions: 1,
    searchqueries: 1, searchtopics: 1, searchunified: 1, searchverses: 1, verseofthedays: 2,
  };
  for (const [resource, count] of Object.entries(resources)) {
    edit(`${resource}/raw_client.go`, (source, replace) => replace(source,
      '\theaders := internal.MergeHeaders(\n\t\tr.options.ToHeader(),\n\t\toptions.ToHeader(),\n\t)',
      '\theaders, err := r.options.ToHeader(options, true)\n\tif err != nil {\n\t\treturn nil, err\n\t}', count));
  }

  edit('dataexchange/raw_client.go', (source, replace) => {
    let count = 0;
    source = source.replace(/func \(r \*RawClient\) Approval(Get|Post)\([\s\S]*?(?=\nfunc |$)/g, (block, verb) => {
      count++;
      block = replace(block, 'r.options.ToHeader(options, true)',
        `r.options.ToHeader(options, ${verb === 'Get' ? 'false' : 'request == nil || request.Token == nil'})`);
      return replace(block, '\t\t\tURL:             endpointURL,',
        '\t\t\tURL:             endpointURL,\n\t\t\tManualRedirect:  true,');
    });
    if (count !== 2) throw new Error('Expected two Go approval operations');
    return source;
  });

  edit('fonts/raw_client.go', (source, replace) => {
    source = replace(source, '\thttp "net/http"', '\thttp "net/http"\n\turl "net/url"');
    return replace(source, '\tresponse := bytes.NewBuffer(nil)',
      '\tquery := make(url.Values)\n\tfor name, values := range options.QueryParameters {\n\t\tquery[name] = append([]string(nil), values...)\n\t}\n\tappKey := r.options.YvpAppKey\n\tif options.YvpAppKey != "" {\n\t\tappKey = options.YvpAppKey\n\t}\n\tquery.Set("app_key", appKey)\n\toptions.QueryParameters = query\n\tresponse := bytes.NewBuffer(nil)');
  });

  for (const resource of ['languages', 'organizations']) {
    edit(`${resource}/raw_client.go`, (source, replace) => replace(source,
      '\theaders.Add("Accept-Language", "en")',
      '\tif headers.Get("Accept-Language") == "" {\n\t\theaders.Set("Accept-Language", "en")\n\t}', 2));
  }

  edit('internal/http.go', (source, replace) => {
    source = replace(source, '\t"reflect"', '\t"reflect"\n\t"strings"');
    return replace(source, 'url.PathEscape(fmt.Sprintf("%v", value))',
      'strings.ReplaceAll(url.QueryEscape(fmt.Sprintf("%v", value)), "+", "%20")');
  });

  edit('internal/caller.go', (source, replace) => {
    source = replace(source, '\turl := buildURL(params.URL, params.QueryParameters)', '\trequestURL := buildURL(params.URL, params.QueryParameters)');
    source = replace(source, '\t\turl,\n\t\tparams.Method,', '\t\trequestURL,\n\t\tparams.Method,');
    source = replace(source, '\tURL                string', '\tManualRedirect     bool\n\tURL                string');
    source = replace(source, '\tresp, err := c.retrier.Run(', `	if params.ManualRedirect {
		if transport, ok := client.(*http.Client); ok {
			// Preserve caller-owned transport, timeout, and cookie jar without mutation.
			copy := *transport
			copy.CheckRedirect = func(*http.Request, []*http.Request) error { return http.ErrUseLastResponse }
			client = &copy
		}
	}

	resp, err := c.retrier.Run(`);
    source = replace(source,
      '\t\tbuildRetryOptions(params.MaxAttempts, params.DisableRetries)...,\n\t)\n\tif err != nil {\n\t\treturn nil, err\n\t}',
      '\t\tbuildRetryOptions(params.MaxAttempts, params.DisableRetries)...,\n\t)\n\tif err != nil {\n\t\tvar transportError *url.Error\n\t\tif errors.As(err, &transportError) {\n\t\t\tcopy := *transportError\n\t\t\tif parsed, parseError := url.Parse(copy.URL); parseError == nil {\n\t\t\t\tquery := parsed.Query()\n\t\t\t\tfor _, name := range []string{"app_key", "token", "x-yvp-app-key"} {\n\t\t\t\t\tif query.Has(name) { query.Set(name, "[REDACTED]") }\n\t\t\t\t}\n\t\t\t\tparsed.RawQuery = query.Encode()\n\t\t\t\tcopy.URL = parsed.Redacted()\n\t\t\t}\n\t\t\terr = &copy\n\t\t}\n\t\treturn nil, err\n\t}');
    return replace(source, 'if resp.StatusCode < 200 || resp.StatusCode >= 300 {',
      'if (resp.StatusCode < 200 || resp.StatusCode >= 300) && !(params.ManualRedirect && resp.StatusCode == http.StatusSeeOther) {');
  });

  edit('internal/retrier.go', (source, replace) => {
    source = replace(source, 'type Retrier struct {\n\tattempts uint\n}',
      'type Retrier struct {\n\tattempts uint\n\tdisabled bool\n}');
    source = replace(source, '\t\tattempts: attempts,', '\t\tattempts: attempts,\n\t\tdisabled: options.disabled,');
    source = replace(source, 'if options.disabled {\n\t\tmaxRetryAttempts = 1',
      'if options.disabled || (r.disabled && options.attempts == 0) {\n\t\tmaxRetryAttempts = 1');
    // The final attempt returns immediately through Caller, without sleeping again.
    source = replace(source, 'if r.shouldRetry(response) {',
      'if r.shouldRetry(response) && retryAttempt+1 < maxRetryAttempts {');
    return replace(source, '\t\ttime.Sleep(delay)', `		timer := time.NewTimer(delay)
		defer timer.Stop()
		select {
		case <-request.Context().Done():
			return nil, request.Context().Err()
		case <-timer.C:
		}`);
  });

  edit('core/api_error.go', (source, replace) => {
    source = replace(source, '\t"fmt"', '\t"fmt"\n\t"encoding/json"');
    source = replace(source, '\tStatusCode int', '\t// Body preserves the JSON value or text returned by the API.\n\tBody any `json:"-"`\n\tStatusCode int');
    return replace(source, '\treturn &APIError{\n\t\terr:        err,',
      '\tvar body any\n\tif err != nil {\n\t\tif json.Unmarshal([]byte(err.Error()), &body) != nil {\n\t\t\tbody = err.Error()\n\t\t}\n\t}\n\treturn &APIError{\n\t\tBody:       body,\n\t\terr:        err,');
  });
}
