import type {
  AdminCapabilitiesResponse,
  AdminErrorResponse,
  AdminListResponse,
  AdminRoutePath,
  AdminTimelineResponse,
  IdempotencyKey,
  OverridRef,
  ReasonCode,
  TraceId,
} from "./contracts";

export type OvergateBaseUrl = string & { readonly __overgateBaseUrl: unique symbol };

export interface AdminReadSignature {
  readonly signatureRefs: readonly OverridRef[];
  readonly idempotencyKey?: IdempotencyKey;
}

export interface AdminRequestOptions {
  readonly cursor?: string;
  readonly limit?: number;
  readonly filters?: Readonly<Record<string, string | number | boolean | readonly string[]>>;
  readonly pathParams?: Readonly<Record<string, string>>;
  readonly traceId?: TraceId;
  readonly signatureRefs?: readonly OverridRef[];
  readonly idempotencyKey?: IdempotencyKey;
}

export interface AdminRequest {
  readonly method: "GET";
  readonly path: AdminRoutePath;
  readonly url: string;
  readonly traceId: TraceId;
  readonly headers: Readonly<Record<string, string>>;
  readonly query: Readonly<Record<string, string>>;
  readonly signatureRefs: readonly OverridRef[];
  readonly idempotencyKey?: IdempotencyKey;
}

export interface AdminTransportResponse {
  readonly status: number;
  json(): Promise<unknown>;
}

export type AdminTransport = (request: AdminRequest) => Promise<AdminTransportResponse>;

export interface OvergateClientConfig {
  readonly baseUrl: OvergateBaseUrl;
  readonly schemaVersion: string;
  readonly activeTenantId?: OverridRef;
  readonly actorId?: OverridRef;
  readonly defaultLimit?: number;
  readonly transport?: AdminTransport;
}

export interface PaginatedCollection<T> {
  readonly pages: number;
  readonly items: readonly T[];
  readonly lastCursor?: string;
}

const DISALLOWED_BASE_URL_PROTOCOLS = new Set([
  "postgres:",
  "mysql:",
  "redis:",
  "s3:",
  "vault:",
  "file:",
]);

const DISALLOWED_SERVICE_HOST_HINTS = [
  "overqueue",
  "overcell",
  "overrun",
  "overmeter",
  "seal-ledger",
  "seal_ledger",
  "overvault",
  "overbase",
  "overstore",
];

const MAX_ADMIN_QUERY_LIMIT = 250;
const MAX_ADMIN_FILTER_COUNT = 8;

export function assertOvergateBaseUrl(rawUrl: string, overgateRouteScope?: string): OvergateBaseUrl {
  const parsed = new URL(rawUrl);
  const hostname = parsed.hostname.toLowerCase();
  const pathname = parsed.pathname.toLowerCase();
  const routeScope = (overgateRouteScope ?? "").toLowerCase();

  if (DISALLOWED_BASE_URL_PROTOCOLS.has(parsed.protocol)) {
    throw new Error("Admin UI base URL must use an Overgate HTTP route.");
  }
  if (parsed.protocol !== "https:" && parsed.protocol !== "http:") {
    throw new Error("Admin UI base URL must use http or https.");
  }
  if (parsed.username || parsed.password || parsed.search || parsed.hash) {
    throw new Error("Admin UI base URL must not carry credentials, query strings, or fragments.");
  }
  if (DISALLOWED_SERVICE_HOST_HINTS.some((hint) => hostname.includes(hint))) {
    throw new Error("Admin UI base URL must not target a core service directly.");
  }

  const loopbackHost = hostname === "localhost" || hostname === "127.0.0.1" || hostname === "::1";
  const overgateNamed = hostname.includes("overgate") || pathname.includes("overgate") || routeScope.includes("overgate");
  if (!loopbackHost && !overgateNamed) {
    throw new Error("Admin UI base URL must be an Overgate route.");
  }

  return trimTrailingSlash(parsed.toString()) as OvergateBaseUrl;
}

export function createTraceId(prefix = "admin_ui"): TraceId {
  const safePrefix = prefix.replace(/[^a-z0-9_]/gi, "_").toLowerCase();
  return `trace_${safePrefix}_${Date.now().toString(36)}` as TraceId;
}

export function buildSignedAdminReadOptions(signature: AdminReadSignature, options: AdminRequestOptions = {}): AdminRequestOptions {
  return {
    ...options,
    signatureRefs: [...signature.signatureRefs],
    idempotencyKey: signature.idempotencyKey,
  };
}

export function buildAdminRequest(config: OvergateClientConfig, path: AdminRoutePath, options: AdminRequestOptions = {}): AdminRequest {
  if (!path.startsWith("/admin/") && path !== "/admin/capabilities") {
    throw new Error(`Admin UI route must stay under Overgate admin APIs: ${path}`);
  }

  const traceId = options.traceId ?? createTraceId("admin_ui_request");
  const signatureRefs = options.signatureRefs ?? [];
  const pathWithParams = applyPathParams(path, options.pathParams);
  const base = new URL(config.baseUrl);
  const url = new URL(`${trimTrailingSlash(base.pathname)}/${trimLeadingSlash(pathWithParams)}`, base);
  const query = buildQuery(config, options);

  for (const [key, value] of Object.entries(query)) {
    url.searchParams.set(key, value);
  }

  return {
    method: "GET",
    path,
    url: url.toString(),
    traceId,
    query,
    headers: {
      accept: "application/json",
      "x-overrid-trace-id": traceId,
      "x-overrid-schema-version": config.schemaVersion,
      ...(signatureRefs.length ? { "x-overrid-signature-refs": signatureRefs.join(",") } : {}),
      ...(options.idempotencyKey ? { "x-overrid-idempotency-key": options.idempotencyKey } : {}),
      ...(config.activeTenantId ? { "x-overrid-tenant-id": config.activeTenantId } : {}),
      ...(config.actorId ? { "x-overrid-actor-id": config.actorId } : {}),
    },
    signatureRefs,
    idempotencyKey: options.idempotencyKey,
  };
}

export function decodeAdminError(payload: unknown, fallbackTraceId: TraceId): AdminErrorResponse {
  if (isAdminErrorResponse(payload)) {
    return payload;
  }

  return {
    schema_version: "admin-ui-admin-api.v0.1",
    status: 500,
    reason_code: "admin.error.unparseable_response" as ReasonCode,
    trace_id: fallbackTraceId,
    retryable: false,
    user_visible_message: "The admin API returned an unreadable error shape.",
    details_ref: "error:unparseable" as OverridRef,
    audit_refs: [
      {
        audit_id: "audit:unparseable_admin_error" as OverridRef,
        source_service: "overgate",
        trace_id: fallbackTraceId,
      },
    ],
  };
}

export async function collectPaginatedSummaries(
  fetchPage: (cursor?: string) => Promise<AdminListResponse>,
  maxPages = 8,
): Promise<PaginatedCollection<AdminListResponse["items"][number]>> {
  const items: AdminListResponse["items"] = [];
  const seenCursors = new Set<string>();
  let cursor: string | undefined;
  let pages = 0;

  while (pages < maxPages) {
    const page = await fetchPage(cursor);
    items.push(...page.items);
    pages += 1;

    const nextCursor = page.page.next_cursor_ref;
    if (!nextCursor || seenCursors.has(nextCursor)) {
      return { pages, items, lastCursor: nextCursor };
    }
    seenCursors.add(nextCursor);
    cursor = nextCursor;
  }

  return { pages, items, lastCursor: cursor };
}

export function createOvergateClient(config: OvergateClientConfig) {
  const transport = config.transport;
  if (!transport) {
    throw new Error("Admin UI Overgate client requires an explicit transport adapter.");
  }
  const transportAdapter: AdminTransport = transport;

  async function requestJson<T>(path: AdminRoutePath, options?: AdminRequestOptions): Promise<T> {
    const request = buildAdminRequest(config, path, options);
    const response = await transportAdapter(request);
    const payload = await response.json();
    if (response.status >= 400) {
      throw decodeAdminError(payload, request.traceId);
    }
    return payload as T;
  }

  return {
    getCapabilities: (options?: AdminRequestOptions) => requestJson<AdminCapabilitiesResponse>("/admin/capabilities", options),
    list: (path: AdminRoutePath, options?: AdminRequestOptions) => requestJson<AdminListResponse>(path, options),
    timeline: (workloadId: OverridRef, options?: AdminRequestOptions) =>
      requestJson<AdminTimelineResponse>("/admin/workloads/{id}/timeline", {
        ...options,
        pathParams: { ...(options?.pathParams ?? {}), id: workloadId },
      }),
  };
}

function buildQuery(config: OvergateClientConfig, options: AdminRequestOptions): Record<string, string> {
  const query: Record<string, string> = {};
  const limit = options.limit ?? config.defaultLimit;
  if (limit !== undefined) {
    if (!Number.isInteger(limit) || limit < 1 || limit > MAX_ADMIN_QUERY_LIMIT) {
      throw new Error("Admin UI summary queries must use a bounded server-side limit.");
    }
    query.limit = String(limit);
  }
  if (options.cursor) {
    query.cursor = options.cursor;
  }
  const filterEntries = Object.entries(options.filters ?? {});
  if (filterEntries.length > MAX_ADMIN_FILTER_COUNT) {
    throw new Error("Admin UI summary queries must use bounded filter sets.");
  }
  for (const [key, value] of filterEntries) {
    query[`filter.${key}`] = Array.isArray(value) ? value.join(",") : String(value);
  }
  return query;
}

function applyPathParams(path: AdminRoutePath, params?: Readonly<Record<string, string>>): string {
  if (!path.includes("{")) {
    return path;
  }
  const id = params?.id;
  if (!id) {
    throw new Error(`Missing admin route path parameter for ${path}`);
  }
  return path.replace("{id}", encodeURIComponent(id));
}

function isAdminErrorResponse(payload: unknown): payload is AdminErrorResponse {
  if (!payload || typeof payload !== "object") {
    return false;
  }
  const candidate = payload as Partial<AdminErrorResponse>;
  return (
    typeof candidate.schema_version === "string" &&
    typeof candidate.status === "number" &&
    typeof candidate.reason_code === "string" &&
    typeof candidate.trace_id === "string" &&
    typeof candidate.retryable === "boolean" &&
    typeof candidate.user_visible_message === "string" &&
    typeof candidate.details_ref === "string" &&
    Array.isArray(candidate.audit_refs)
  );
}

function trimTrailingSlash(value: string): string {
  return value.replace(/\/+$/, "");
}

function trimLeadingSlash(value: string): string {
  return value.replace(/^\/+/, "");
}
