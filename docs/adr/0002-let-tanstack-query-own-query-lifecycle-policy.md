# Let TanStack Query own query lifecycle policy

Generated query keys include a required atomic cache scope, resource, operation, and complete declared request data, but exclude clients, credentials, headers, callbacks, and abort signals. The cache scope represents every response-affecting client context that is absent from request data, while add-on transport overrides are limited to timeout and retry count so unkeyed headers or query parameters cannot create collisions.

Generated query and mutation functions disable SDK retries by default so TanStack Query owns retry policy, with an explicit numeric SDK retry escape hatch; query functions also forward TanStack Query's `AbortSignal`. Because TanStack Query rejects `undefined` query results, operations that model a successful empty response return `null` through the add-on while preserving the base SDK's `undefined` result.
