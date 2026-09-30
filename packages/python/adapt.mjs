import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { replaceExactly } from '../../scripts/generation.mjs';

export function adaptPython(directory) {
  function edit(file, transform) {
    const path = join(directory, file);
    const source = readFileSync(path, 'utf8');
    const replace = (text, search, replacement, count = 1) =>
      replaceExactly(text, search, replacement, count, path);
    writeFileSync(path, transform(source, replace));
  }

  edit('core/jsonable_encoder.py', (source, replace) => {
    source = replace(source, 'from types import GeneratorType', 'from types import GeneratorType\nfrom urllib.parse import quote');
    return replace(source, 'return str(jsonable_encoder(obj))', 'return quote(str(jsonable_encoder(obj)), safe="-_.!~*\'()")');
  });

  edit('core/client_wrapper.py', (source, replace) => {
    source = replace(source,
      '    def get_headers(self) -> typing.Dict[str, str]:',
      '    def get_headers(self, *, include_auth: bool = True) -> typing.Dict[str, str]:');
    source = replace(source, '        token = self._get_token()', '        token = self._get_token() if include_auth else None');
    source = replace(source,
      '    def get_custom_headers(self) -> typing.Optional[typing.Dict[str, str]]:',
      '    def get_app_key(self) -> str:\n        return self._yvp_app_key\n\n    def get_custom_headers(self) -> typing.Optional[typing.Dict[str, str]]:');
    return replace(source,
      '    async def async_get_headers(self) -> typing.Dict[str, str]:\n        headers = self.get_headers()\n        if self._async_token is not None:',
      '    async def async_get_headers(self, *, include_auth: bool = True) -> typing.Dict[str, str]:\n        headers = self.get_headers(include_auth=include_auth and self._async_token is None)\n        if include_auth and self._async_token is not None:');
  });

  edit('core/http_client.py', (source, replace) => {
    source = replace(source, '        "authorization",', '        "authorization",\n        "x-yvp-app-key",');
    source = replace(source, 'base_headers: typing.Callable[[], typing.Dict[str, str]],', 'base_headers: typing.Callable[..., typing.Dict[str, str]],', 2);
    source = replace(source, 'async_base_headers: typing.Optional[typing.Callable[[], typing.Awaitable[typing.Dict[str, str]]]]', 'async_base_headers: typing.Optional[typing.Callable[..., typing.Awaitable[typing.Dict[str, str]]]]');
    source = replace(source,
      '    async def _get_headers(self) -> typing.Dict[str, str]:\n        if self.async_base_headers is not None:\n            return await self.async_base_headers()\n        return self.base_headers()',
      '    async def _get_headers(self, *, include_auth: bool = True) -> typing.Dict[str, str]:\n        if self.async_base_headers is not None:\n            return await self.async_base_headers(include_auth=include_auth)\n        return self.base_headers(include_auth=include_auth)');
    // Only request(), not streaming, needs endpoint-owned auth and redirect overrides.
    source = replace(source,
      '        force_multipart: typing.Optional[bool] = None,\n    ) -> httpx.Response:',
      '        force_multipart: typing.Optional[bool] = None,\n        follow_redirects: typing.Optional[bool] = None,\n        include_auth: bool = True,\n    ) -> httpx.Response:', 2);
    source = replace(source,
      '                files=request_files,\n                timeout=timeout,\n            )',
      '                files=request_files,\n                timeout=timeout,\n                **({"follow_redirects": follow_redirects} if follow_redirects is not None else {}),\n            )', 2);
    source = replace(source,
      '                    force_multipart=force_multipart,\n                )',
      '                    force_multipart=force_multipart,\n                    follow_redirects=follow_redirects,\n                    include_auth=include_auth,\n                )', 4);
    // Header names are case-insensitive. Preserve the HTTPX client's own headers
    // while making per-request overrides win over generated and client defaults.
    for (const base of ['self.base_headers()', '_headers']) {
      source = replace(source,
        `                    **${base},\n                    **(headers if headers is not None else {}),`,
        `                    **{k.lower(): v for k, v in ${base}.items()},\n                    **{k.lower(): v for k, v in (headers or {}).items()},`, 2);
    }
    source = replace(source,
      '**(request_options.get("additional_headers", {}) or {} if request_options is not None else {}),',
      '**{k.lower(): v for k, v in ((request_options.get("additional_headers") or {}) if request_options is not None else {}).items()},', 2);
    source = replace(source,
      '**(request_options.get("additional_headers", {}) if request_options is not None else {}),',
      '**{k.lower(): v for k, v in ((request_options.get("additional_headers") or {}) if request_options is not None else {}).items()},', 2);
    let requests = 0;
    source = source.replace(/^    (?:async )?def request\([\s\S]*?(?=^    @)/gm, (block) => {
      requests++;
      const getter = block.startsWith('    async ') ? 'self._get_headers' : 'self.base_headers';
      return replace(block, `${getter}()`, `${getter}(include_auth=include_auth)`);
    });
    if (requests !== 2) throw new Error(`Expected sync and async HTTP request methods, found ${requests}`);
    return source;
  });

  // Fix only these endpoint blocks, leaving unrelated redirects and status codes alone.
  edit('data_exchange/raw_client.py', (source, replace) => {
    for (const method of ['approval_get', 'approval_post']) {
      const pattern = new RegExp(`(^    (?:async )?def ${method}\\([\\s\\S]*?)(?=^    (?:async )?def |^class |$(?![\\s\\S]))`, 'gm');
      let count = 0;
      source = source.replace(pattern, (block) => {
        count++;
        block = replace(block,
          '            request_options=request_options,\n        )',
          `            headers={"Authorization": None}${method === 'approval_post' ? ' if token is not None else None' : ''},\n            include_auth=${method === 'approval_post' ? 'token is None' : 'False'},\n            follow_redirects=False,\n            request_options=request_options,\n        )`);
        block = replace(block, 'if 200 <= _response.status_code < 300:',
          'if 200 <= _response.status_code < 300 or _response.status_code == 303:');
        return block;
      });
      if (count !== 2) throw new Error(`Expected sync and async ${method}, found ${count}`);
    }
    return source;
  });

  edit('fonts/raw_client.py', (source, replace) => replace(source,
    '            method="GET",\n            request_options=request_options,\n        )\n        try:\n            if 200 <= _response.status_code < 300:\n                return',
    '            method="GET",\n            params={"app_key": self._client_wrapper.get_app_key()},\n            request_options=request_options,\n        )\n        try:\n            if 200 <= _response.status_code < 300:\n                return', 2));

  for (const resource of ['languages', 'organizations']) {
    edit(`${resource}/raw_client.py`, (source, replace) => replace(source,
      '                "Accept-Language": "en",',
      '                "Accept-Language": next((value for key, value in (self._client_wrapper.get_custom_headers() or {}).items() if key.lower() == "accept-language"), "en"),', 4));
  }

  for (const resource of ['bibles', 'search_queries', 'search_topics', 'search_unified']) {
    for (const file of ['client.py', 'raw_client.py']) {
      edit(`${resource}/${file}`, (source, replace) => replace(source,
        '        language_ranges: typing.Optional[typing.Union[str, typing.Sequence[str]]] = None,',
        '        language_ranges: typing.Union[str, typing.Sequence[str]],', 2));
    }
  }

  for (const [resource, count] of Object.entries({ bibles: 8, highlights: 2, search_queries: 2, verse_of_the_days: 2, languages: 2, organizations: 2 })) {
    edit(`${resource}/raw_client.py`, (source, replace) => replace(source,
      'if _response is None or not _response.text.strip():',
      'if 200 <= _response.status_code < 300 and not _response.text.strip():', count));
  }

  // Lazy resource properties otherwise erase the method signatures for consumers.
  edit('client.py', (source, replace) => {
    source = replace(source, '    ):\n        _defaulted_timeout', '    ) -> None:\n        _defaulted_timeout', 2);
    const sections = source.split('class AsyncPlatformClient:');
    if (sections.length !== 2) throw new Error('Missing async client boundary');
    for (let index = 0; index < sections.length; index++) {
      const classes = [...sections[index].matchAll(/self\._(\w+): typing.Optional\[(\w+)\] = None/g)];
      if (classes.length !== 14) throw new Error(`Expected 14 resource properties, found ${classes.length}`);
      for (const [, resource, cls] of classes) {
        sections[index] = replace(sections[index], `    def ${resource}(self):`, `    def ${resource}(self) -> ${cls}:`);
      }
    }
    return sections.join('class AsyncPlatformClient:');
  });
}
