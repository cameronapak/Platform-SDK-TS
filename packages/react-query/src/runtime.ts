import type { BaseRequestOptions } from '@cameronapak/platform-sdk';
import {
  mutationOptions,
  queryOptions,
  useMutation,
  useQuery,
  type DefaultError,
  type DefinedInitialDataOptions,
  type DefinedUseQueryResult,
  type MutationFilters,
  type QueryClient,
  type QueryFilters,
  type QueryKeyWithDataTag,
  type UndefinedInitialDataOptions,
  type UseMutationOptions,
  type UseMutationResult,
  type UseQueryResult,
  type WithRequired,
} from '@tanstack/react-query';

export const PLATFORM_QUERY_NAMESPACE = '@cameronapak/platform-sdk-react-query' as const;

export interface SdkExecutionOptions {
  /** Override the SDK request timeout without changing cache identity. */
  timeoutInSeconds?: number;
  /** Override SDK retries. Defaults to zero so TanStack Query owns retry policy. */
  maxRetries?: number;
}

type LockedQueryOption = 'queryKey' | 'queryFn' | 'queryHash' | 'queryKeyHashFn';
type LockedMutationOption = 'mutationKey' | 'mutationFn';

type WithSdkOptions<T> = T & { sdk?: SdkExecutionOptions };

export type PlatformQueryKey<TRequest> = readonly [
  typeof PLATFORM_QUERY_NAMESPACE,
  string,
  string,
  string,
  Readonly<TRequest>,
];

export type PlatformMutationKey = readonly [
  typeof PLATFORM_QUERY_NAMESPACE,
  string,
  string,
  string,
];

type TaggedQueryKey<TRequest, TQueryFnData> = QueryKeyWithDataTag<
  PlatformQueryKey<TRequest>,
  TQueryFnData,
  DefaultError
>['queryKey'];

type UndefinedBindingOptions<TQueryFnData, TData, TRequest> = WithSdkOptions<
  Omit<
    UndefinedInitialDataOptions<TQueryFnData, DefaultError, TData, PlatformQueryKey<TRequest>>,
    LockedQueryOption
  >
>;

type DefinedBindingOptions<TQueryFnData, TData, TRequest> = WithSdkOptions<
  Omit<
    DefinedInitialDataOptions<TQueryFnData, DefaultError, TData, PlatformQueryKey<TRequest>>,
    LockedQueryOption
  >
>;

type UndefinedBindingResult<TQueryFnData, TData, TRequest> = UndefinedInitialDataOptions<
  TQueryFnData,
  DefaultError,
  TData,
  PlatformQueryKey<TRequest>
> &
  QueryKeyWithDataTag<PlatformQueryKey<TRequest>, TQueryFnData, DefaultError>;

type DefinedBindingResult<TQueryFnData, TData, TRequest> = DefinedInitialDataOptions<
  TQueryFnData,
  DefaultError,
  TData,
  PlatformQueryKey<TRequest>
> &
  QueryKeyWithDataTag<PlatformQueryKey<TRequest>, TQueryFnData, DefaultError>;

export type BindingMutationOptions<TData, TVariables, TOnMutateResult> = WithSdkOptions<
  Omit<UseMutationOptions<TData, DefaultError, TVariables, TOnMutateResult>, LockedMutationOption>
>;

type BindingMutationResult<TData, TVariables, TOnMutateResult> = WithRequired<
  UseMutationOptions<TData, DefaultError, TVariables, TOnMutateResult>,
  'mutationKey'
>;

interface QueryBindingCommon<TRequest, TQueryFnData> {
  queryKey(request: TRequest): TaggedQueryKey<TRequest, TQueryFnData>;
  queryFilters(): QueryFilters;
}

export interface RequiredQueryBinding<TRequest, TQueryFnData>
  extends QueryBindingCommon<TRequest, TQueryFnData> {
  queryOptions<TData = TQueryFnData>(
    request: TRequest,
    options: DefinedBindingOptions<TQueryFnData, TData, TRequest>,
  ): DefinedBindingResult<TQueryFnData, TData, TRequest>;
  queryOptions<TData = TQueryFnData>(
    request: TRequest,
    options?: UndefinedBindingOptions<TQueryFnData, TData, TRequest>,
  ): UndefinedBindingResult<TQueryFnData, TData, TRequest>;
  useQuery<TData = TQueryFnData>(
    request: TRequest,
    options: DefinedBindingOptions<TQueryFnData, TData, TRequest>,
    queryClient?: QueryClient,
  ): DefinedUseQueryResult<TData, DefaultError>;
  useQuery<TData = TQueryFnData>(
    request: TRequest,
    options?: UndefinedBindingOptions<TQueryFnData, TData, TRequest>,
    queryClient?: QueryClient,
  ): UseQueryResult<TData, DefaultError>;
}

export interface OptionalQueryBinding<TRequest, TQueryFnData>
  extends QueryBindingCommon<TRequest, TQueryFnData> {
  queryOptions<TData = TQueryFnData>(
    request: TRequest | undefined,
    options: DefinedBindingOptions<TQueryFnData, TData, TRequest>,
  ): DefinedBindingResult<TQueryFnData, TData, TRequest>;
  queryOptions<TData = TQueryFnData>(
    request?: TRequest,
    options?: UndefinedBindingOptions<TQueryFnData, TData, TRequest>,
  ): UndefinedBindingResult<TQueryFnData, TData, TRequest>;
  useQuery<TData = TQueryFnData>(
    request: TRequest | undefined,
    options: DefinedBindingOptions<TQueryFnData, TData, TRequest>,
    queryClient?: QueryClient,
  ): DefinedUseQueryResult<TData, DefaultError>;
  useQuery<TData = TQueryFnData>(
    request?: TRequest,
    options?: UndefinedBindingOptions<TQueryFnData, TData, TRequest>,
    queryClient?: QueryClient,
  ): UseQueryResult<TData, DefaultError>;
}

export interface RequestlessQueryBinding<TQueryFnData> {
  queryKey(): TaggedQueryKey<Record<string, never>, TQueryFnData>;
  queryFilters(): QueryFilters;
  queryOptions<TData = TQueryFnData>(
    options: DefinedBindingOptions<TQueryFnData, TData, Record<string, never>>,
  ): DefinedBindingResult<TQueryFnData, TData, Record<string, never>>;
  queryOptions<TData = TQueryFnData>(
    options?: UndefinedBindingOptions<TQueryFnData, TData, Record<string, never>>,
  ): UndefinedBindingResult<TQueryFnData, TData, Record<string, never>>;
  useQuery<TData = TQueryFnData>(
    options: DefinedBindingOptions<TQueryFnData, TData, Record<string, never>>,
    queryClient?: QueryClient,
  ): DefinedUseQueryResult<TData, DefaultError>;
  useQuery<TData = TQueryFnData>(
    options?: UndefinedBindingOptions<TQueryFnData, TData, Record<string, never>>,
    queryClient?: QueryClient,
  ): UseQueryResult<TData, DefaultError>;
}

export interface MutationBinding<TVariables, TData> {
  mutationKey(): PlatformMutationKey;
  mutationFilters(): MutationFilters;
  mutationOptions<TOnMutateResult = unknown>(
    options?: BindingMutationOptions<TData, TVariables, TOnMutateResult>,
  ): BindingMutationResult<TData, TVariables, TOnMutateResult>;
  useMutation<TOnMutateResult = unknown>(
    options?: BindingMutationOptions<TData, TVariables, TOnMutateResult>,
    queryClient?: QueryClient,
  ): UseMutationResult<TData, DefaultError, TVariables, TOnMutateResult>;
}

interface QueryBindingConfig<TRequest extends object, TResult, TEmptyAsNull extends boolean> {
  cacheScope: string;
  resource: string;
  operationId: string;
  emptyAsNull: TEmptyAsNull;
  requestProperties: readonly (keyof TRequest)[];
  execute(request: TRequest, requestOptions: BaseRequestOptions): PromiseLike<TResult>;
}

interface RequestlessQueryBindingConfig<TResult, TEmptyAsNull extends boolean> {
  cacheScope: string;
  resource: string;
  operationId: string;
  emptyAsNull: TEmptyAsNull;
  execute(requestOptions: BaseRequestOptions): PromiseLike<TResult>;
}

interface MutationBindingConfig<TVariables, TResult> {
  cacheScope: string;
  resource: string;
  operationId: string;
  execute(variables: TVariables, requestOptions: BaseRequestOptions): PromiseLike<TResult>;
}

type NormalizedQueryResult<TResult, TEmptyAsNull extends boolean> = TEmptyAsNull extends true
  ? Exclude<Awaited<TResult>, undefined> | null
  : Awaited<TResult>;

function cloneRequestValue<T>(value: T): T {
  if (Array.isArray(value)) return value.map(cloneRequestValue) as T;
  if (value != null && typeof value === 'object') {
    return Object.fromEntries(
      Object.entries(value).map(([key, child]) => [key, cloneRequestValue(child)]),
    ) as T;
  }
  return value;
}

function projectRequest<TRequest extends object>(
  request: TRequest,
  properties: readonly (keyof TRequest)[],
): TRequest {
  const projected: Partial<TRequest> = {};
  for (const key of properties) {
    if (key in request) projected[key] = cloneRequestValue(request[key]);
  }
  return projected as TRequest;
}

function requestOptions(sdk: SdkExecutionOptions | undefined, abortSignal?: AbortSignal): BaseRequestOptions {
  return {
    abortSignal,
    maxRetries: sdk?.maxRetries ?? 0,
    timeoutInSeconds: sdk?.timeoutInSeconds,
  };
}

function normalizeQueryResult<TResult, TEmptyAsNull extends boolean>(
  result: Awaited<TResult>,
  emptyAsNull: TEmptyAsNull,
  operationId: string,
): NormalizedQueryResult<TResult, TEmptyAsNull> {
  if (result !== undefined) return result as NormalizedQueryResult<TResult, TEmptyAsNull>;
  if (emptyAsNull) return null as NormalizedQueryResult<TResult, TEmptyAsNull>;
  throw new TypeError(`${operationId} returned an unexpected empty response`);
}

function splitQueryOptions<T extends { sdk?: SdkExecutionOptions }>(
  options: T | undefined,
): [Omit<T, 'sdk' | LockedQueryOption>, SdkExecutionOptions | undefined] {
  const {
    sdk,
    queryKey: _queryKey,
    queryFn: _queryFn,
    queryHash: _queryHash,
    queryKeyHashFn: _queryKeyHashFn,
    ...tanstackOptions
  } = (options ?? {}) as T & Record<LockedQueryOption, unknown>;
  return [tanstackOptions, sdk];
}

function splitMutationOptions<T extends { sdk?: SdkExecutionOptions }>(
  options: T | undefined,
): [Omit<T, 'sdk' | LockedMutationOption>, SdkExecutionOptions | undefined] {
  const { sdk, mutationKey: _mutationKey, mutationFn: _mutationFn, ...tanstackOptions } = (options ??
    {}) as T & Record<LockedMutationOption, unknown>;
  return [tanstackOptions, sdk];
}

function operationQueryKey<TRequest>(
  cacheScope: string,
  resource: string,
  operationId: string,
  request: TRequest,
): PlatformQueryKey<TRequest> {
  return [PLATFORM_QUERY_NAMESPACE, cacheScope, resource, operationId, request] as const;
}

function operationMutationKey(
  cacheScope: string,
  resource: string,
  operationId: string,
): PlatformMutationKey {
  return [PLATFORM_QUERY_NAMESPACE, cacheScope, resource, operationId] as const;
}

function createQueryBinding<TRequest extends object, TResult, TEmptyAsNull extends boolean>(
  config: QueryBindingConfig<TRequest, TResult, TEmptyAsNull>,
) {
  type QueryResult = NormalizedQueryResult<TResult, TEmptyAsNull>;

  const queryFilters = (): QueryFilters => ({
    queryKey: [PLATFORM_QUERY_NAMESPACE, config.cacheScope, config.resource, config.operationId],
  });

  const makeQueryOptions = <TData = QueryResult>(
    request: TRequest,
    options?: UndefinedBindingOptions<QueryResult, TData, TRequest>,
  ): UndefinedBindingResult<QueryResult, TData, TRequest> => {
    const snapshot = projectRequest(request, config.requestProperties);
    const [tanstackOptions, sdk] = splitQueryOptions(options);
    return queryOptions({
      ...tanstackOptions,
      queryKey: operationQueryKey(config.cacheScope, config.resource, config.operationId, snapshot),
      queryFn: async ({ signal }) =>
        normalizeQueryResult(
          await config.execute(snapshot, requestOptions(sdk, signal)),
          config.emptyAsNull,
          config.operationId,
        ),
    }) as unknown as UndefinedBindingResult<QueryResult, TData, TRequest>;
  };

  return {
    queryKey(request: TRequest) {
      return operationQueryKey(
        config.cacheScope,
        config.resource,
        config.operationId,
        projectRequest(request, config.requestProperties),
      ) as unknown as TaggedQueryKey<TRequest, QueryResult>;
    },
    queryFilters,
    queryOptions: makeQueryOptions,
    useQuery<TData = QueryResult>(
      request: TRequest,
      options?: UndefinedBindingOptions<QueryResult, TData, TRequest>,
      queryClient?: QueryClient,
    ) {
      return useQuery(makeQueryOptions(request, options), queryClient);
    },
  } as RequiredQueryBinding<TRequest, QueryResult>;
}

export function createRequiredQueryBinding<
  TRequest extends object,
  TResult,
  TEmptyAsNull extends boolean,
>(
  config: QueryBindingConfig<TRequest, TResult, TEmptyAsNull>,
): RequiredQueryBinding<TRequest, NormalizedQueryResult<TResult, TEmptyAsNull>> {
  return createQueryBinding(config);
}

export function createOptionalQueryBinding<TRequest extends object, TResult, TEmptyAsNull extends boolean>(
  config: QueryBindingConfig<TRequest, TResult, TEmptyAsNull>,
): OptionalQueryBinding<TRequest, NormalizedQueryResult<TResult, TEmptyAsNull>> {
  const binding = createQueryBinding(config);
  return {
    ...binding,
    queryKey(request?: TRequest) {
      return binding.queryKey(request ?? ({} as TRequest));
    },
    queryOptions<TData = NormalizedQueryResult<TResult, TEmptyAsNull>>(
      request?: TRequest,
      options?: UndefinedBindingOptions<
        NormalizedQueryResult<TResult, TEmptyAsNull>,
        TData,
        TRequest
      >,
    ) {
      return binding.queryOptions(request ?? ({} as TRequest), options);
    },
    useQuery<TData = NormalizedQueryResult<TResult, TEmptyAsNull>>(
      request?: TRequest,
      options?: UndefinedBindingOptions<
        NormalizedQueryResult<TResult, TEmptyAsNull>,
        TData,
        TRequest
      >,
      queryClient?: QueryClient,
    ) {
      return binding.useQuery(request ?? ({} as TRequest), options, queryClient);
    },
  } as OptionalQueryBinding<TRequest, NormalizedQueryResult<TResult, TEmptyAsNull>>;
}

export function createRequestlessQueryBinding<TResult, TEmptyAsNull extends boolean>(
  config: RequestlessQueryBindingConfig<TResult, TEmptyAsNull>,
): RequestlessQueryBinding<NormalizedQueryResult<TResult, TEmptyAsNull>> {
  type EmptyRequest = Record<string, never>;
  const binding = createQueryBinding<EmptyRequest, TResult, TEmptyAsNull>({
    ...config,
    requestProperties: [],
    execute: (_request, options) => config.execute(options),
  });
  const emptyRequest: EmptyRequest = {};
  return {
    queryKey: () => binding.queryKey(emptyRequest),
    queryFilters: binding.queryFilters,
    queryOptions: (options) => binding.queryOptions(emptyRequest, options),
    useQuery: (options, queryClient) => binding.useQuery(emptyRequest, options, queryClient),
  } as RequestlessQueryBinding<NormalizedQueryResult<TResult, TEmptyAsNull>>;
}

export function createMutationBinding<TVariables, TResult>(
  config: MutationBindingConfig<TVariables, TResult>,
): MutationBinding<TVariables, Awaited<TResult>> {
  const key = operationMutationKey(config.cacheScope, config.resource, config.operationId);
  const makeMutationOptions = <TOnMutateResult = unknown>(
    options?: BindingMutationOptions<Awaited<TResult>, TVariables, TOnMutateResult>,
  ): BindingMutationResult<Awaited<TResult>, TVariables, TOnMutateResult> => {
    const [tanstackOptions, sdk] = splitMutationOptions(options);
    return mutationOptions({
      ...tanstackOptions,
      mutationKey: key,
      mutationFn: async (variables): Promise<Awaited<TResult>> =>
        (await config.execute(variables, requestOptions(sdk))) as Awaited<TResult>,
    });
  };

  return {
    mutationKey: () => key,
    mutationFilters: () => ({ mutationKey: key }),
    mutationOptions: makeMutationOptions,
    useMutation: (options, queryClient) => useMutation(makeMutationOptions(options), queryClient),
  };
}

export function validateCacheScope(cacheScope: string): string {
  if (typeof cacheScope !== 'string' || cacheScope.trim().length === 0) {
    throw new TypeError('cacheScope must be a non-empty, non-secret string');
  }
  return cacheScope;
}

export function rootQueryFilters(cacheScope: string): QueryFilters {
  return { queryKey: [PLATFORM_QUERY_NAMESPACE, cacheScope] };
}

export function rootMutationFilters(cacheScope: string): MutationFilters {
  return { mutationKey: [PLATFORM_QUERY_NAMESPACE, cacheScope] };
}

export function resourceQueryFilters(cacheScope: string, resource: string): QueryFilters {
  return { queryKey: [PLATFORM_QUERY_NAMESPACE, cacheScope, resource] };
}

export function resourceMutationFilters(cacheScope: string, resource: string): MutationFilters {
  return { mutationKey: [PLATFORM_QUERY_NAMESPACE, cacheScope, resource] };
}
