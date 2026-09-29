# Keep React Query in a separate SDK add-on

Ship the TanStack React Query integration as `@cameronapak/platform-sdk-react-query`, a sibling package generated from the Platform SDK operation inventory. It binds a caller-owned Platform SDK client and cache scope into reusable keys, options, and hooks while leaving client construction, React context, cache policy, hydration, optimistic updates, and cross-resource invalidation to applications; this keeps `@cameronapak/platform-sdk` framework-neutral without creating a second endpoint inventory.
