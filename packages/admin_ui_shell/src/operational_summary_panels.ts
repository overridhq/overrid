import type {
  AdminCapabilitiesResponse,
  AdminDependencyStatus,
  AdminListResponse,
  AdminRoutePath,
  AdminSessionContext,
  OverridRef,
  ReasonCode,
  TraceId,
} from "./contracts";
import type { AdminRequestOptions } from "./overgate_client";
import { findOperatorPanel, type PanelId } from "./operator_shell";

export type OperationalSummaryPanelId = Extract<
  PanelId,
  "tenants" | "identities" | "keys" | "nodes" | "packages" | "workloads" | "queue"
>;

export type DependencyService =
  | "overgate"
  | "overpass"
  | "overtenant"
  | "overkey"
  | "overregistry"
  | "overwatch"
  | "overqueue"
  | "oversched"
  | "overlease"
  | "overcell"
  | "overrun";

export type OperationalPanelStatus = "ready" | "degraded" | "disabled" | "empty" | "stale";
export type SummaryRefreshMode = "initial_load" | "manual_refresh" | "scheduled_refresh";
export type NodeOperationalState = "live" | "stale" | "expired" | "draining" | "denied" | "unverified";
export type ProductWorkloadFamily = "docdex" | "mcoda" | "codali" | "cli" | "sdk";

export interface OperationalSummaryQueryInput {
  readonly limit?: number;
  readonly cursor?: string;
  readonly filters?: Readonly<Record<string, string | number | boolean | readonly string[]>>;
  readonly refreshMode?: SummaryRefreshMode;
  readonly refreshWindowMs?: number;
  readonly staleAfterMs?: number;
  readonly manualRefresh?: boolean;
}

export interface OperationalSummaryQuery {
  readonly limit: number;
  readonly cursor?: string;
  readonly filters: Readonly<Record<string, string>>;
  readonly refreshMode: SummaryRefreshMode;
  readonly refreshWindowMs: number;
  readonly staleAfterMs: number;
  readonly manualRefresh: boolean;
  readonly reasonCodes: readonly ReasonCode[];
}

export interface OperationalSummaryRow {
  readonly id: OverridRef;
  readonly tenantId: OverridRef;
  readonly kind: AdminListResponse["items"][number]["kind"];
  readonly state: string;
  readonly traceId: TraceId;
  readonly reasonCodes: AdminListResponse["items"][number]["reason_codes"];
  readonly auditRefs: AdminListResponse["items"][number]["audit_refs"];
  readonly tenantVisible: boolean;
  readonly nodeState?: NodeOperationalState;
  readonly productFamily?: ProductWorkloadFamily;
  readonly disabledReasonCodes: readonly ReasonCode[];
}

export interface OperationalSummaryPanelState {
  readonly panelId: OperationalSummaryPanelId;
  readonly route: AdminRoutePath;
  readonly status: OperationalPanelStatus;
  readonly rows: readonly OperationalSummaryRow[];
  readonly rejectedRowIds: readonly OverridRef[];
  readonly dependencyStatuses: readonly AdminDependencyStatus[];
  readonly disabledReasonCodes: readonly ReasonCode[];
  readonly query: OperationalSummaryQuery;
  readonly loadedAtEpochMs: number;
  readonly staleAgeMs: number;
  readonly refreshDue: boolean;
}

export const PHASE5_OPERATIONAL_PANEL_IDS: readonly OperationalSummaryPanelId[] = [
  "tenants",
  "identities",
  "keys",
  "nodes",
  "packages",
  "workloads",
  "queue",
];

export const REQUIRED_DEPENDENCY_SERVICES: readonly DependencyService[] = [
  "overgate",
  "overpass",
  "overtenant",
  "overkey",
  "overregistry",
  "overwatch",
  "overqueue",
  "oversched",
  "overlease",
  "overcell",
  "overrun",
];

export const REQUIRED_NODE_FIXTURE_STATES: readonly NodeOperationalState[] = [
  "live",
  "stale",
  "expired",
  "draining",
  "denied",
  "unverified",
];

export const REQUIRED_WORKLOAD_PRODUCT_FAMILIES: readonly ProductWorkloadFamily[] = [
  "docdex",
  "mcoda",
  "codali",
  "cli",
  "sdk",
];

const PANEL_ROUTES: Readonly<Record<OperationalSummaryPanelId, AdminRoutePath>> = {
  tenants: "/admin/tenants",
  identities: "/admin/identities",
  keys: "/admin/keys",
  nodes: "/admin/nodes",
  packages: "/admin/packages",
  workloads: "/admin/workloads",
  queue: "/admin/queue-items",
};

const PANEL_DEPENDENCIES: Readonly<Record<OperationalSummaryPanelId, readonly DependencyService[]>> = {
  tenants: ["overgate", "overpass", "overtenant"],
  identities: ["overgate", "overpass", "overtenant"],
  keys: ["overgate", "overkey"],
  nodes: ["overgate", "overregistry", "overwatch"],
  packages: ["overgate", "overregistry", "overwatch"],
  workloads: ["overgate", "overwatch", "overqueue", "oversched", "overlease", "overcell", "overrun"],
  queue: ["overgate", "overqueue", "oversched", "overlease"],
};

const PANEL_ALLOWED_FILTERS: Readonly<Record<OperationalSummaryPanelId, readonly string[]>> = {
  tenants: ["state", "role"],
  identities: ["role", "state"],
  keys: ["rotation_state", "revocation_state", "state"],
  nodes: ["health", "trust_class", "region", "state", "verification_state"],
  packages: ["state", "package_ref", "product_family"],
  workloads: ["state", "trace_id", "stale", "product_family"],
  queue: ["state", "trace_id", "priority", "stale"],
};

const PANEL_DEFAULT_LIMITS: Readonly<Record<OperationalSummaryPanelId, number>> = {
  tenants: 50,
  identities: 50,
  keys: 50,
  nodes: 75,
  packages: 75,
  workloads: 75,
  queue: 75,
};

const DEFAULT_REFRESH_WINDOW_MS = 30000;
const MIN_REFRESH_WINDOW_MS = 5000;
const DEFAULT_STALE_AFTER_MS = 120000;
const FALLBACK_MAX_FILTER_COUNT = 6;

export function getOperationalSummaryPanels(): readonly OperationalSummaryPanelId[] {
  return PHASE5_OPERATIONAL_PANEL_IDS;
}

export function getPanelRoute(panelId: OperationalSummaryPanelId): AdminRoutePath {
  return PANEL_ROUTES[panelId];
}

export function getPanelDependencies(panelId: OperationalSummaryPanelId): readonly DependencyService[] {
  return PANEL_DEPENDENCIES[panelId];
}

export function createBoundedSummaryQuery(
  panelId: OperationalSummaryPanelId,
  capabilities: AdminCapabilitiesResponse,
  input: OperationalSummaryQueryInput = {},
): OperationalSummaryQuery {
  const reasonCodes: ReasonCode[] = [];
  const routeLimit = capabilities.limits.route_max_limit;
  const requestedLimit = input.limit ?? PANEL_DEFAULT_LIMITS[panelId];
  const limit = clampLimit(requestedLimit, routeLimit, reasonCodes);
  const filters = buildBoundedFilters(panelId, capabilities, input.filters ?? {}, reasonCodes);
  const refreshWindowMs = clampRefreshWindow(input.refreshWindowMs, reasonCodes);

  if (capabilities.limits.cursor_required && !input.cursor) {
    reasonCodes.push("admin.query.cursor_required" as ReasonCode);
  }

  return {
    limit,
    cursor: input.cursor,
    filters,
    refreshMode: input.refreshMode ?? (input.manualRefresh ? "manual_refresh" : "initial_load"),
    refreshWindowMs,
    staleAfterMs: input.staleAfterMs ?? DEFAULT_STALE_AFTER_MS,
    manualRefresh: input.manualRefresh ?? true,
    reasonCodes,
  };
}

export function toAdminRequestOptions(query: OperationalSummaryQuery): AdminRequestOptions {
  return {
    cursor: query.cursor,
    limit: query.limit,
    filters: query.filters,
  };
}

export function buildDependencyHealthStrip(capabilities: AdminCapabilitiesResponse): readonly AdminDependencyStatus[] {
  const statuses = new Map<string, AdminDependencyStatus>();
  for (const disabledPanel of capabilities.disabled_panels) {
    const status = normalizeDependencyStatus(disabledPanel.dependency_status);
    statuses.set(status.service, status);
  }
  return REQUIRED_DEPENDENCY_SERVICES.map((service) => {
    const status = statuses.get(service);
    return (
      status ?? {
        service,
        status: "available",
        reason_code: "admin.dependency.available" as ReasonCode,
      }
    );
  });
}

export function getPanelDependencyStatuses(
  panelId: OperationalSummaryPanelId,
  capabilities: AdminCapabilitiesResponse,
): readonly AdminDependencyStatus[] {
  const dependencies = new Set(getPanelDependencies(panelId));
  return buildDependencyHealthStrip(capabilities).filter((status) =>
    dependencies.has(status.service.toLowerCase() as DependencyService),
  );
}

export function getAffectedPanelsForDependency(service: DependencyService): readonly OperationalSummaryPanelId[] {
  return PHASE5_OPERATIONAL_PANEL_IDS.filter((panelId) => PANEL_DEPENDENCIES[panelId].includes(service));
}

export function createOperationalSummaryRows(
  response: AdminListResponse,
  context: AdminSessionContext,
): readonly OperationalSummaryRow[] {
  const visibleTenantIds = new Set<OverridRef>([
    context.active_tenant_id,
    ...context.role_bindings.map((binding) => binding.tenant_id),
  ]);

  return response.items.map((item) => {
    const tenantVisible = visibleTenantIds.has(item.tenant_id);
    const disabledReasonCodes = tenantVisible ? [] : (["admin.summary.cross_tenant_denied" as ReasonCode] as const);
    return {
      id: item.id,
      tenantId: item.tenant_id,
      kind: item.kind,
      state: item.state,
      traceId: item.trace_id,
      reasonCodes: item.reason_codes,
      auditRefs: item.audit_refs,
      tenantVisible,
      nodeState: item.kind === "node" ? classifyNodeOperationalState(item.state, item.reason_codes) : undefined,
      productFamily: detectProductWorkloadFamily(item.id, item.reason_codes),
      disabledReasonCodes,
    };
  });
}

export function filterTenantVisibleRows(rows: readonly OperationalSummaryRow[]): readonly OperationalSummaryRow[] {
  return rows.filter((row) => row.tenantVisible);
}

export function assertTenantScopedSummaries(rows: readonly OperationalSummaryRow[]): {
  readonly ok: boolean;
  readonly rejectedRowIds: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
} {
  const rejectedRowIds = rows.filter((row) => !row.tenantVisible).map((row) => row.id);
  return {
    ok: rejectedRowIds.length === 0,
    rejectedRowIds,
    reasonCodes: rejectedRowIds.length ? (["admin.summary.cross_tenant_denied" as ReasonCode] as const) : [],
  };
}

export function buildOperationalSummaryPanelState(
  panelId: OperationalSummaryPanelId,
  response: AdminListResponse,
  context: AdminSessionContext,
  capabilities: AdminCapabilitiesResponse,
  query: OperationalSummaryQuery,
  loadedAtEpochMs = Date.now(),
  evaluatedAtEpochMs = Date.now(),
): OperationalSummaryPanelState {
  const route = getPanelRoute(panelId);
  if (response.path !== route) {
    throw new Error(`Admin summary response path ${response.path} does not match panel route ${route}`);
  }
  const operatorPanel = findOperatorPanel(panelId);
  if (operatorPanel.route !== route) {
    throw new Error(`Operator panel ${panelId} must route through ${route}`);
  }

  const dependencyStatuses = getPanelDependencyStatuses(panelId, capabilities);
  const unavailableDependencies = dependencyStatuses.filter((dependency) => dependency.status === "unavailable");
  const degradedDependencies = dependencyStatuses.filter((dependency) => dependency.status === "degraded");
  const rows = createOperationalSummaryRows(response, context);
  const tenantScope = assertTenantScopedSummaries(rows);
  const visibleRows = filterTenantVisibleRows(rows);
  const staleAgeMs = Math.max(0, evaluatedAtEpochMs - loadedAtEpochMs);
  const refreshDue = staleAgeMs >= query.staleAfterMs;
  const disabledReasonCodes = [
    ...query.reasonCodes,
    ...tenantScope.reasonCodes,
    ...unavailableDependencies.map((dependency) => dependency.reason_code),
  ];

  return {
    panelId,
    route,
    status: resolvePanelStatus(response, visibleRows, unavailableDependencies, degradedDependencies, query, refreshDue),
    rows: visibleRows,
    rejectedRowIds: tenantScope.rejectedRowIds,
    dependencyStatuses,
    disabledReasonCodes,
    query,
    loadedAtEpochMs,
    staleAgeMs,
    refreshDue,
  };
}

export function classifyNodeOperationalState(
  state: string,
  reasonCodes: readonly ReasonCode[],
): NodeOperationalState {
  const normalizedState = state.toLowerCase();
  const normalizedReasons = reasonCodes.map((reason) => reason.toLowerCase());
  if (normalizedState.includes("draining") || normalizedReasons.some((reason) => reason.includes("draining"))) {
    return "draining";
  }
  if (normalizedState.includes("denied") || normalizedReasons.some((reason) => reason.includes("denied"))) {
    return "denied";
  }
  if (normalizedState.includes("expired") || normalizedReasons.some((reason) => reason.includes("expired"))) {
    return "expired";
  }
  if (normalizedState.includes("unverified") || normalizedReasons.some((reason) => reason.includes("unverified"))) {
    return "unverified";
  }
  if (normalizedState.includes("stale") || normalizedReasons.some((reason) => reason.includes("stale"))) {
    return "stale";
  }
  return "live";
}

export function detectProductWorkloadFamily(
  id: OverridRef,
  reasonCodes: readonly ReasonCode[],
): ProductWorkloadFamily | undefined {
  const haystack = [id, ...reasonCodes].join(" ").toLowerCase();
  return REQUIRED_WORKLOAD_PRODUCT_FAMILIES.find((family) => haystack.includes(family));
}

export function hasRequiredProductFixtureCoverage(rows: readonly OperationalSummaryRow[]): boolean {
  const covered = new Set(rows.map((row) => row.productFamily).filter(Boolean));
  return REQUIRED_WORKLOAD_PRODUCT_FAMILIES.every((family) => covered.has(family));
}

function clampLimit(requestedLimit: number, routeMaxLimit: number, reasonCodes: ReasonCode[]): number {
  if (!Number.isInteger(requestedLimit) || requestedLimit < 1) {
    reasonCodes.push("admin.query.limit_invalid" as ReasonCode);
    return 1;
  }
  if (requestedLimit > routeMaxLimit) {
    reasonCodes.push("admin.query.limit_clamped" as ReasonCode);
    return routeMaxLimit;
  }
  return requestedLimit;
}

function buildBoundedFilters(
  panelId: OperationalSummaryPanelId,
  capabilities: AdminCapabilitiesResponse,
  filters: Readonly<Record<string, string | number | boolean | readonly string[]>>,
  reasonCodes: ReasonCode[],
): Readonly<Record<string, string>> {
  const allowedFilters = new Set(PANEL_ALLOWED_FILTERS[panelId]);
  const maxFilterCount = capabilities.limits.filter_max_count || FALLBACK_MAX_FILTER_COUNT;
  const boundedEntries: Array<[string, string]> = [];

  for (const [key, value] of Object.entries(filters)) {
    if (!allowedFilters.has(key)) {
      reasonCodes.push("admin.query.filter_invalid" as ReasonCode);
      continue;
    }
    if (boundedEntries.length >= maxFilterCount) {
      reasonCodes.push("admin.query.filter_limit_exceeded" as ReasonCode);
      break;
    }
    boundedEntries.push([key, serializeFilterValue(value)]);
  }

  return Object.fromEntries(boundedEntries);
}

function clampRefreshWindow(requestedRefreshWindowMs: number | undefined, reasonCodes: ReasonCode[]): number {
  if (requestedRefreshWindowMs === undefined) {
    return DEFAULT_REFRESH_WINDOW_MS;
  }
  if (!Number.isInteger(requestedRefreshWindowMs) || requestedRefreshWindowMs < MIN_REFRESH_WINDOW_MS) {
    reasonCodes.push("admin.query.refresh_window_clamped" as ReasonCode);
    return MIN_REFRESH_WINDOW_MS;
  }
  return requestedRefreshWindowMs;
}

function serializeFilterValue(value: string | number | boolean | readonly string[]): string {
  return Array.isArray(value) ? value.join(",") : String(value);
}

function normalizeDependencyStatus(status: AdminDependencyStatus): AdminDependencyStatus {
  return {
    ...status,
    service: status.service.toLowerCase(),
  };
}

function resolvePanelStatus(
  response: AdminListResponse,
  rows: readonly OperationalSummaryRow[],
  unavailableDependencies: readonly AdminDependencyStatus[],
  degradedDependencies: readonly AdminDependencyStatus[],
  query: OperationalSummaryQuery,
  refreshDue: boolean,
): OperationalPanelStatus {
  if (unavailableDependencies.length > 0) {
    return "disabled";
  }
  if (query.reasonCodes.includes("admin.query.cursor_required" as ReasonCode)) {
    return "disabled";
  }
  if (response.status === "empty" || rows.length === 0) {
    return "empty";
  }
  if (response.status === "degraded" || degradedDependencies.length > 0 || response.degraded_dependencies.length > 0) {
    return "degraded";
  }
  if (refreshDue) {
    return "stale";
  }
  return "ready";
}
