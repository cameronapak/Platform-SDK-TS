import { readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { replaceExactly } from '../../scripts/generation.mjs';

export function adaptPhp(directory) {
  function edit(file, transform) {
    const path = join(directory, file);
    const replace = (source, search, replacement, count = 1) => replaceExactly(source, search, replacement, count, path);
    writeFileSync(path, transform(readFileSync(path, 'utf8'), replace));
  }

  edit('composer.json', (source) => {
    const manifest = JSON.parse(source);
    if (!manifest['require-dev']?.['phpunit/phpunit'] || !manifest['autoload-dev'] || !manifest.scripts) throw new Error('PHP maintainer manifest changed');
    // The generated test suite is not shipped. The separate consumer owns tools.
    delete manifest['require-dev'];
    delete manifest['autoload-dev'];
    delete manifest.scripts;
    manifest.archive = { exclude: ['/test', '/adapt.mjs', '/operation-map.mjs', '/check.mjs'] };
    return JSON.stringify(manifest, null, 2) + '\n';
  });
  edit('src/Core/Client/RetryDecoratingClient.php', (source, replace) => replace(source,
    "return $this->client->send($request, ['timeout' => $timeout]);",
    "return $this->client->send($request, ['timeout' => $timeout, 'allow_redirects' => false, 'http_errors' => false]);"));

  edit('src/Core/Client/RawClient.php', (source, replace) => {
    source = replace(source, `        $url = $this->buildUrl($request, $options);
        $headers = $this->encodeHeaders($request, $options);`, `        $headers = $this->encodeHeaders($request, $options);
        $url = $this->buildUrl($request, $options, $headers);`);
    source = replace(source, `    private function buildUrl(
        BaseApiRequest $request,
        array          $options,
    ): string {`, `    private function buildUrl(
        BaseApiRequest $request,
        array          $options,
        array          $headers,
    ): string {`);
    source = replace(source,
      '$authHeaders = $this->getAuthHeaders !== null ? ($this->getAuthHeaders)() : [];',
      `$query = array_merge($request->query, $options['queryParameters'] ?? []);
        $approval = trim($request->path, '/') === 'data-exchange';
        $suppressBearer = $approval && ($request->method === HttpMethod::GET || array_key_exists('token', $query));
        $authHeaders = !$suppressBearer && $this->getAuthHeaders !== null ? ($this->getAuthHeaders)() : [];`);
    source = replace(source, '        return match (get_class($request)) {', '        $headers = match (get_class($request)) {');
    source = replace(source, `                $this->headers,
                $authHeaders,
                $request->headers,`, `                $request->headers,
                $this->headers,
                $authHeaders,`, 2);
    source = replace(source,
      "            default => throw new InvalidArgumentException('Unsupported request type: ' . get_class($request)),\n        };",
      `            default => throw new InvalidArgumentException('Unsupported request type: ' . get_class($request)),
        };
        $headers = array_change_key_case($headers, CASE_LOWER);
        if ($suppressBearer) {
            unset($headers['authorization']);
        }
        return $headers;`);
    // Normalize each header layer before merging, not after case-sensitive array_merge.
    for (const header of ['$request->headers', '$this->headers', '$authHeaders', "$options['headers'] ?? []"]) {
      source = replace(source, `                ${header},`, `                array_change_key_case(${header}, CASE_LOWER),`, 2);
    }
    source = replace(source, `        if (!empty($query)) {
            $url .= '?' . $this->encodeQuery($query);`, `        if (preg_match('#^v1/fonts/[^/]+/stylesheet$#', trim($request->path, '/'))) {
            if (isset($headers['x-yvp-app-key'])) {
                $query['app_key'] = $headers['x-yvp-app-key'];
            }
        }
        if (!empty($query)) {
            $url .= '?' . $this->encodeQuery($query);`);
    return replace(source, `    /**
     * Check if an array is sequential, not associative.
     * @param mixed[] $arr
     * @return bool
     */
    private static function isSequential(array $arr): bool`, `    /** Merge caller options without discarding unrelated client headers.
     * @param array<string, mixed> $defaults
     * @param ?array<string, mixed> $overrides
     * @return array<string, mixed>
     */
    public static function mergeOptions(array $defaults, ?array $overrides): array
    {
        $result = array_merge($defaults, $overrides ?? []);
        $result['headers'] = array_merge(
            array_change_key_case($defaults['headers'] ?? [], CASE_LOWER),
            array_change_key_case($overrides['headers'] ?? [], CASE_LOWER),
        );
        return $result;
    }

    /** Encode one path segment before a transport can normalize it. */
    public static function encodePathSegment(string $value): string
    {
        if ($value === '.' || $value === '..') {
            throw new InvalidArgumentException('Path identifiers cannot be dot segments');
        }
        return rawurlencode($value);
    }

    /**
     * Check if an array is sequential, not associative.
     * @param mixed[] $arr
     * @return bool
     */
    private static function isSequential(array $arr): bool`);
  });

  edit('src/Exceptions/CameronapakApiException.php', (source, replace) => {
    source = replace(source, '    private mixed $body;', '    private mixed $body;\n\n    /** @var array<string, array<string>> */\n    private array $headers;');
    source = replace(source, '        ?Throwable $previous = null,', '        ?Throwable $previous = null,\n        array $headers = [],');
    source = replace(source, '        $this->body = $body;', '        $this->body = $body;\n        $this->headers = array_change_key_case($headers, CASE_LOWER);');
    source = replace(source, '    public function __toString(): string', `    /** @return array<string, array<string>> */
    public function getHeaders(): array
    {
        return $this->headers;
    }

    public function __toString(): string`);
    return replace(source, `        if (empty($this->body)) {
            return $this->message . '; Status Code: ' . $this->getCode() . "\\n";
        }
        return $this->message . '; Status Code: ' . $this->getCode() . '; Body: ' . print_r($this->body, true) . "\\n";`,
      `        // Explicit getBody()/getHeaders() retain data; automatic logging does not.
        return $this->message . '; Status Code: ' . $this->getCode() . "\\n";`);
  });
  edit('src/Exceptions/CameronapakException.php', (source, replace) => replace(source,
    'class CameronapakException extends Exception\n{\n}',
    `class CameronapakException extends Exception
{
    public function __toString(): string
    {
        // Do not print request arguments or a credential-bearing transport cause.
        return static::class . ': ' . $this->message . "\\n";
    }
}`));
  edit('src/Core/Json/JsonSerializableType.php', (source, replace) => replace(source,
    '            if ($dateTypeAttr) {',
    '            if ($dateTypeAttr && !($value === null && $property->getType()?->allowsNull())) {'));

  let operations = 0;
  let pathSegments = 0;
  let nullableQueries = 0;
  let deserializationErrors = 0;
  for (const resource of readdirSync(join(directory, 'src'), { withFileTypes: true }).filter((entry) => entry.isDirectory())) {
    const file = `src/${resource.name}/${resource.name}Client.php`;
    if (!readdirSync(join(directory, 'src', resource.name)).includes(`${resource.name}Client.php`)) continue;
    edit(file, (source, replace) => {
      const count = source.split('$options = array_merge($this->options, $options ?? []);').length - 1;
      operations += count;
      nullableQueries += source.split(' != null').length - 1;
      source = source.replaceAll(' != null', ' !== null');
      source = replace(source, '$options = array_merge($this->options, $options ?? []);', '$options = RawClient::mergeOptions($this->options, $options);', count);
      source = replace(source, 'throw new CameronapakException(message: $e->getMessage(), previous: $e);', "throw new CameronapakException(message: 'HTTP transport failed', previous: $e);", count);
      const unsafeMessage = 'throw new CameronapakException(message: "Failed to deserialize response: {$e->getMessage()}", previous: $e);';
      const errors = source.split(unsafeMessage).length - 1;
      deserializationErrors += errors;
      source = replace(source, unsafeMessage, "throw new CameronapakException(message: 'Failed to deserialize response', previous: $e);", errors);
      source = replace(source, '            body: $response->getBody()->getContents(),', '            body: $response->getBody()->getContents(),\n            headers: $response->getHeaders(),', count);
      source = source.replace(/(    public function \w+\(([^\n]+)\): [^\n]+\n    \{\n)/g, (match, start, signature) => {
        const params = [...signature.matchAll(/string \$(\w+)/g)].map(([, name]) => name);
        pathSegments += params.length;
        return start + params.map((name) => `        $${name} = RawClient::encodePathSegment($${name});\n`).join('');
      });
      return source;
    });
  }
  if (operations !== 33 || pathSegments !== 16 || nullableQueries !== 40) throw new Error(`PHP endpoint inventory changed: ${operations} operations, ${pathSegments} string segments, ${nullableQueries} nullable queries`);
  if (deserializationErrors !== 29) throw new Error(`PHP deserialization error inventory changed: ${deserializationErrors} wrappers`);

  for (const [resource, type] of [
    ['Bibles', 'BiblesCollectionGetRequest'],
    ['Languages', 'V1LanguagesCollectionGetRequest'],
    ['Organizations', 'V1OrganizationsCollectionGetRequest'],
    ['Organizations', 'V1OrganizationsBiblesCollectionGetRequest'],
  ]) {
    edit(`src/${resource}/Requests/${type}.php`, (source, replace) => {
      source = replace(source, `?value-of<${type}PageSize>`, 'int|string|null', 2);
      return replace(source, 'public ?string $pageSize;', 'public int|string|null $pageSize;');
    });
  }
  for (const [resource, type] of [['Languages', 'V1LanguagesCollectionGetRequest'], ['Organizations', 'V1OrganizationsCollectionGetRequest']]) {
    edit(`src/${resource}/Requests/${type}.php`, (source, replace) => {
      source = replace(source, "   acceptLanguage: 'en',", "   acceptLanguage?: 'en',");
      source = replace(source, '        array $values,', '        array $values = [],');
      return replace(source, "$this->acceptLanguage = $values['acceptLanguage'];", "$this->acceptLanguage = $values['acceptLanguage'] ?? 'en';");
    });
  }

  edit('src/DataExchange/DataExchangeClient.php', (source, replace) => {
    source = replace(source, 'use JsonException;', 'use JsonException;\nuse Psr\\Http\\Message\\ResponseInterface;');
    // Keep generated request construction and errors in one raw method per approval.
    for (const [method, type, result] of [
      ['approvalGet', 'DataExchangeApprovalGetRequest', 'string'],
      ['approvalPost', 'DataExchangeApprovalPostRequest', 'void'],
    ]) {
      const signature = `public function ${method}(${type} $request${method === 'approvalPost' ? ` = new ${type}()` : ''}, ?array $options = null): ${result}`;
      const wrapper = `public function ${method}(${type} $request${method === 'approvalPost' ? ` = new ${type}()` : ''}, ?array $options = null): ${result}
    {
        ${method === 'approvalGet' ? 'return ' : ''}$this->${method}WithResponse($request, $options)${method === 'approvalGet' ? '->getBody()->getContents()' : ''};
    }

    /**
     * Returns status, headers, and the original body without following redirects.
     * @param ${type} $request
     * @param ?array{baseUrl?: string, maxRetries?: int, timeout?: float, headers?: array<string, string>, queryParameters?: array<string, mixed>, bodyProperties?: array<string, mixed>} $options
     * @return ResponseInterface
     */
    public function ${method}WithResponse(${type} $request${method === 'approvalPost' ? ` = new ${type}()` : ''}, ?array $options = null): ResponseInterface`;
      source = replace(source, signature, wrapper);
    }
    source = replace(source, '                return $response->getBody()->getContents();', '                return $response;');
    return replace(source, '                return;', '                return $response;');
  });
}
