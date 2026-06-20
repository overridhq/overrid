import type {
  AdminCapabilitiesResponse,
  AdminDependencyStatus,
  AdminRoutePath,
  AdminTimelineResponse,
  OverridRef,
  ReasonCode,
  TraceId,
} from "./contracts";

export type WorkloadTimelineOutcome =
  | "successful"
  | "failed"
  | "retryable"
  | "cancelled"
  | "timed_out"
  | "denied"
  | "disputed"
  | "partial"
  | "unavailable";

export type TimelineStage =
  | "request"
  | "command_acceptance"
  | "queue_item"
  | "scheduler_decision"
  | "lease"
  | "node_assignment"
  | "runner_lifecycle"
  | "policy_decision"
  | "usage_rollup"
  | "receipt"
  | "dispute"
  | "correction_ref"
  | "gap";

export type TimelineLayer = "core" | "policy" | "usage" | "accounting" | "dispute" | "diagnostic";
export type TimelineRetryClass =
  | "none"
  | "retryable"
  | "terminal"
  | "policy_denied"
  | "cancelled"
  | "timed_out"
  | "disputed"
  | "waiting";
export type TimelineFollowMode = "overgate_event_stream" | "bounded_polling" | "disabled";

type TimelineNode = AdminTimelineResponse["nodes"][number];
type TimelineAuditRef = AdminTimelineResponse["audit_refs"][number];

export interface WorkloadTimelineOptions {
  readonly loadedAtEpochMs?: number;
  readonly evaluatedAtEpochMs?: number;
  readonly staleAfterMs?: number;
  readonly backendEventStreamAvailable?: boolean;
  readonly refreshWindowMs?: number;
  readonly manualRefresh?: boolean;
}

export interface RenderedTimelineNode {
  readonly nodeId: OverridRef;
  readonly kind: TimelineNode["kind"];
  readonly stage: TimelineStage;
  readonly layer: TimelineLayer;
  readonly status: TimelineNode["status"];
  readonly traceId: TraceId;
  readonly refs: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
  readonly auditRefs: readonly TimelineAuditRef[];
  readonly retryClass: TimelineRetryClass;
  readonly staleAgeMs: number;
  readonly diagnosticReason?: ReasonCode;
  readonly order: number;
}

export interface TimelineGap {
  readonly nodeId: OverridRef;
  readonly owningService: string;
  readonly retryClass: TimelineRetryClass;
  readonly staleAgeMs: number;
  readonly diagnosticReason: ReasonCode;
  readonly refs: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
  readonly auditRefs: readonly TimelineAuditRef[];
}

export interface TimelineOverlay {
  readonly layer: Exclude<TimelineLayer, "core" | "diagnostic">;
  readonly visible: boolean;
  readonly redacted: boolean;
  readonly nodeIds: readonly OverridRef[];
  readonly refs: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
  readonly auditRefs: readonly TimelineAuditRef[];
}

export interface TimelineDiagnosticBundle {
  readonly schema_version: "operator-workload-timeline-diagnostic.v0.1";
  readonly workload_id: OverridRef;
  readonly trace_id: TraceId;
  readonly command_refs: readonly OverridRef[];
  readonly reason_codes: readonly ReasonCode[];
  readonly schema_versions: Readonly<Record<string, string>>;
  readonly dependency_statuses: readonly AdminDependencyStatus[];
  readonly audit_refs: readonly string[];
  readonly safe_refs: readonly OverridRef[];
  readonly redacted_fields: readonly string[];
}

export interface TimelineFollowPlan {
  readonly mode: TimelineFollowMode;
  readonly route: AdminRoutePath;
  readonly refreshWindowMs: number;
  readonly staleAfterMs: number;
  readonly manualRefresh: boolean;
  readonly usesOvergateOnly: boolean;
  readonly directOverwatchConnection: false;
  readonly reasonCodes: readonly ReasonCode[];
}

export interface WorkloadTimelineState {
  readonly workloadId: OverridRef;
  readonly traceId: TraceId;
  readonly status: AdminTimelineResponse["status"];
  readonly outcome: WorkloadTimelineOutcome;
  readonly route: AdminRoutePath;
  readonly nodes: readonly RenderedTimelineNode[];
  readonly gaps: readonly TimelineGap[];
  readonly overlays: readonly TimelineOverlay[];
  readonly diagnosticBundle: TimelineDiagnosticBundle;
  readonly followPlan: TimelineFollowPlan;
  readonly partialDependencies: readonly AdminDependencyStatus[];
  readonly loadedAtEpochMs: number;
  readonly staleAgeMs: number;
  readonly refreshDue: boolean;
}

export const TIMELINE_ROUTE = "/admin/workloads/{id}/timeline";

export const REQUIRED_TIMELINE_STAGES: readonly TimelineStage[] = [
  "request",
  "command_acceptance",
  "queue_item",
  "scheduler_decision",
  "lease",
  "node_assignment",
  "runner_lifecycle",
  "policy_decision",
  "usage_rollup",
  "receipt",
  "dispute",
  "correction_ref",
];

export const REQUIRED_TIMELINE_OUTCOMES: readonly WorkloadTimelineOutcome[] = [
  "successful",
  "failed",
  "retryable",
  "cancelled",
  "timed_out",
  "denied",
  "disputed",
];

export const REQUIRED_GAP_DEPENDENCY_CASES = [
  "missing_overmeter",
  "missing_receipt",
  "delayed_overwatch_event",
  "unavailable_dispute_service",
] as const;

export const TIMELINE_OVERLAY_LAYERS: readonly TimelineOverlay["layer"][] = [
  "policy",
  "usage",
  "accounting",
  "dispute",
];

const DEFAULT_REFRESH_WINDOW_MS = 30000;
const MIN_REFRESH_WINDOW_MS = 5000;
const DEFAULT_STALE_AFTER_MS = 120000;
const STAGE_ORDER = new Map<TimelineStage, number>(REQUIRED_TIMELINE_STAGES.map((stage, index) => [stage, index]));

const UNSAFE_DIAGNOSTIC_PATTERNS = [
  /\bpassword\s*=/i,
  /\bsecret\b/i,
  /\bcredential\b/i,
  /\bprivate[_ -]?payload\b/i,
  /\bdecrypted\b/i,
  /\bprompt\b/i,
  /\bkey[_ -]?material\b/i,
  /\/Users\/|\/home\/|[A-Za-z]:\\/,
];

const ACTIVE_DISPUTE_SIGNALS = [
  "disputed",
  "dispute_hold",
  "dispute_open",
  "dispute_active",
  "dispute_pending",
  "settlement_hold",
  "payout_hold",
] as const;

export function buildWorkloadTimelineState(
  response: AdminTimelineResponse,
  capabilities: AdminCapabilitiesResponse,
  options: WorkloadTimelineOptions = {},
): WorkloadTimelineState {
  const route = assertTimelineRoute(response.path);
  const loadedAtEpochMs = options.loadedAtEpochMs ?? Date.now();
  const evaluatedAtEpochMs = options.evaluatedAtEpochMs ?? Date.now();
  const staleAfterMs = options.staleAfterMs ?? DEFAULT_STALE_AFTER_MS;
  const staleAgeMs = Math.max(0, evaluatedAtEpochMs - loadedAtEpochMs);
  const refreshDue = staleAgeMs >= staleAfterMs;
  const partialDependencies = response.partial_dependencies.map(normalizeDependencyStatus);

  const nodes = response.nodes
    .map((node, index) => renderTimelineNode(node, index, staleAgeMs))
    .sort((left, right) => left.order - right.order);
  const gaps = buildTimelineGaps(nodes, partialDependencies, staleAgeMs);
  const overlays = buildTimelineOverlays(nodes);
  const followPlan = planTimelineFollowMode(capabilities, options);
  const outcome = classifyTimelineOutcome(response, nodes, gaps);
  const diagnosticBundle = createTimelineDiagnosticBundle(response, nodes, partialDependencies, gaps);

  return {
    workloadId: response.workload_id,
    traceId: response.trace_id,
    status: response.status,
    outcome,
    route,
    nodes,
    gaps,
    overlays,
    diagnosticBundle,
    followPlan,
    partialDependencies,
    loadedAtEpochMs,
    staleAgeMs,
    refreshDue,
  };
}

export function renderTimelineNode(node: TimelineNode, index: number, staleAgeMs = 0): RenderedTimelineNode {
  const stage = resolveTimelineStage(node);
  const retryClass = classifyTimelineRetry(node.reason_codes, node.status);
  return {
    nodeId: node.node_id,
    kind: node.kind,
    stage,
    layer: resolveTimelineLayer(stage),
    status: node.status,
    traceId: node.trace_id,
    refs: node.refs,
    reasonCodes: node.reason_codes,
    auditRefs: node.audit_refs,
    retryClass,
    staleAgeMs,
    diagnosticReason: node.status === "available" ? undefined : node.reason_codes[0],
    order: (STAGE_ORDER.get(stage) ?? REQUIRED_TIMELINE_STAGES.length) * 100 + index,
  };
}

export function buildTimelineGaps(
  nodes: readonly RenderedTimelineNode[],
  partialDependencies: readonly AdminDependencyStatus[],
  staleAgeMs: number,
): readonly TimelineGap[] {
  const nodeGaps = nodes
    .filter((node) => node.stage === "gap" || node.status !== "available")
    .map((node) => ({
      nodeId: node.nodeId,
      owningService: resolveOwningService(node),
      retryClass: node.retryClass,
      staleAgeMs,
      diagnosticReason: node.diagnosticReason ?? ("admin.timeline.partial_dependency" as ReasonCode),
      refs: node.refs,
      reasonCodes: node.reasonCodes,
      auditRefs: node.auditRefs,
    }));
  const existingServices = new Set(nodeGaps.map((gap) => gap.owningService));
  const dependencyGaps = partialDependencies
    .filter((dependency) => !existingServices.has(dependency.service.toLowerCase()))
    .map((dependency) => createDependencyGap(dependency, staleAgeMs));
  return [...nodeGaps, ...dependencyGaps];
}

export function buildTimelineOverlays(nodes: readonly RenderedTimelineNode[]): readonly TimelineOverlay[] {
  return TIMELINE_OVERLAY_LAYERS.map((layer) => {
    const layerNodes = nodes.filter((node) => node.layer === layer);
    return {
      layer,
      visible: layerNodes.length > 0,
      redacted: layerNodes.some((node) => node.status === "redacted"),
      nodeIds: layerNodes.map((node) => node.nodeId),
      refs: uniqueRefs(layerNodes.flatMap((node) => node.refs)).filter(isCopySafeRef),
      reasonCodes: uniqueReasonCodes(layerNodes.flatMap((node) => node.reasonCodes)),
      auditRefs: layerNodes.flatMap((node) => node.auditRefs),
    };
  });
}

export function createTimelineDiagnosticBundle(
  response: AdminTimelineResponse,
  nodes: readonly RenderedTimelineNode[],
  partialDependencies: readonly AdminDependencyStatus[],
  gaps: readonly TimelineGap[] = [],
): TimelineDiagnosticBundle {
  const refs = uniqueRefs(nodes.flatMap((node) => node.refs).filter(isCopySafeRef));
  const auditRefs = uniqueAuditRefs([
    ...response.audit_refs,
    ...nodes.flatMap((node) => node.auditRefs),
    ...gaps.flatMap((gap) => gap.auditRefs),
  ]);
  const reasonCodes = uniqueReasonCodes([
    ...nodes.flatMap((node) => node.reasonCodes),
    ...partialDependencies.map((dependency) => dependency.reason_code),
    ...gaps.flatMap((gap) => gap.reasonCodes),
  ]);
  return {
    schema_version: "operator-workload-timeline-diagnostic.v0.1",
    workload_id: response.workload_id,
    trace_id: response.trace_id,
    command_refs: refs.filter((ref) => ref.startsWith("command:")),
    reason_codes: reasonCodes,
    schema_versions: { admin_read_api_contracts: response.schema_version },
    dependency_statuses: partialDependencies,
    audit_refs: auditRefs.map(formatTimelineAuditRef),
    safe_refs: refs,
    redacted_fields: ["payload_body", "auth_values", "local_paths", "raw_result_body"],
  };
}

export function planTimelineFollowMode(
  capabilities: AdminCapabilitiesResponse,
  options: WorkloadTimelineOptions = {},
): TimelineFollowPlan {
  const routeAvailable = capabilities.routes.some((route) => route.path === TIMELINE_ROUTE && route.available);
  const hasTimelineCapability = capabilities.feature_flags.includes("admin.timeline.read");
  const refreshWindowMs = clampRefreshWindow(options.refreshWindowMs);
  const staleAfterMs = options.staleAfterMs ?? DEFAULT_STALE_AFTER_MS;

  if (!routeAvailable || !hasTimelineCapability) {
    return {
      mode: "disabled",
      route: TIMELINE_ROUTE,
      refreshWindowMs,
      staleAfterMs,
      manualRefresh: true,
      usesOvergateOnly: true,
      directOverwatchConnection: false,
      reasonCodes: ["admin.timeline.capability_unavailable" as ReasonCode],
    };
  }

  if (options.backendEventStreamAvailable) {
    return {
      mode: "overgate_event_stream",
      route: TIMELINE_ROUTE,
      refreshWindowMs,
      staleAfterMs,
      manualRefresh: false,
      usesOvergateOnly: true,
      directOverwatchConnection: false,
      reasonCodes: ["admin.timeline.follow_overgate_stream" as ReasonCode],
    };
  }

  return {
    mode: "bounded_polling",
    route: TIMELINE_ROUTE,
    refreshWindowMs,
    staleAfterMs,
    manualRefresh: options.manualRefresh ?? true,
    usesOvergateOnly: true,
    directOverwatchConnection: false,
    reasonCodes: ["admin.timeline.follow_bounded_polling" as ReasonCode],
  };
}

export function classifyTimelineOutcome(
  response: AdminTimelineResponse,
  nodes: readonly RenderedTimelineNode[],
  gaps: readonly TimelineGap[],
): WorkloadTimelineOutcome {
  if (response.status === "unavailable") {
    return "unavailable";
  }
  const outcomeSignals = [
    response.status,
    ...nodes.flatMap((node) => [node.status, ...node.reasonCodes]),
    ...gaps.flatMap((gap) => [gap.retryClass, gap.diagnosticReason, ...gap.reasonCodes]),
  ]
    .join(" ")
    .toLowerCase();

  if (hasActiveDisputeSignal(outcomeSignals)) {
    return "disputed";
  }
  if (outcomeSignals.includes("cancel")) {
    return "cancelled";
  }
  if (outcomeSignals.includes("timeout") || outcomeSignals.includes("timed_out")) {
    return "timed_out";
  }
  if (outcomeSignals.includes("denied") || outcomeSignals.includes("policy_denied")) {
    return "denied";
  }
  if (outcomeSignals.includes("retryable")) {
    return "retryable";
  }
  if (outcomeSignals.includes("failed") || outcomeSignals.includes("terminal")) {
    return "failed";
  }
  if (response.status === "partial" || gaps.length > 0) {
    return "partial";
  }
  return "successful";
}

export function hasRequiredTimelineStageCoverage(nodes: readonly RenderedTimelineNode[]): boolean {
  const stages = new Set(nodes.map((node) => node.stage));
  return REQUIRED_TIMELINE_STAGES.every((stage) => stages.has(stage));
}

export function hasImmutableOverlayRefs(overlays: readonly TimelineOverlay[]): boolean {
  return overlays.every(
    (overlay) =>
      !overlay.visible ||
      (overlay.refs.length > 0 &&
        overlay.auditRefs.length > 0 &&
        overlay.refs.every(isCopySafeRef) &&
        !containsUnsafeDiagnosticText(overlay)),
  );
}

export function containsUnsafeDiagnosticText(value: unknown): boolean {
  if (typeof value === "string") {
    return UNSAFE_DIAGNOSTIC_PATTERNS.some((pattern) => pattern.test(value));
  }
  if (Array.isArray(value)) {
    return value.some(containsUnsafeDiagnosticText);
  }
  if (value && typeof value === "object") {
    return Object.entries(value).some(
      ([key, nestedValue]) => containsUnsafeDiagnosticText(key) || containsUnsafeDiagnosticText(nestedValue),
    );
  }
  return false;
}

export function assertTimelineRoute(path: string): AdminRoutePath {
  if (path !== TIMELINE_ROUTE) {
    throw new Error(`Workload timeline must use Overgate admin timeline route: ${path}`);
  }
  return TIMELINE_ROUTE;
}

function resolveTimelineStage(node: TimelineNode): TimelineStage {
  const haystack = [node.kind, ...node.refs, ...node.reason_codes].join(" ").toLowerCase();
  if (node.kind === "gap") {
    return "gap";
  }
  if (haystack.includes("correction")) {
    return "correction_ref";
  }
  if (node.kind === "overgate_request" && (haystack.includes("command:") || haystack.includes("command."))) {
    return "command_acceptance";
  }
  if (node.kind === "overgate_request") {
    return "request";
  }
  if (node.kind === "overqueue_item") {
    return "queue_item";
  }
  if (node.kind === "oversched_placement") {
    return "scheduler_decision";
  }
  if (node.kind === "overlease_reservation") {
    return "lease";
  }
  if (node.kind === "overcell_execution") {
    return "node_assignment";
  }
  if (node.kind === "overrun_result") {
    return "runner_lifecycle";
  }
  if (node.kind === "overguard_decision") {
    return "policy_decision";
  }
  if (node.kind === "overmeter_rollup") {
    return "usage_rollup";
  }
  if (node.kind === "seal_ledger_receipt") {
    return "receipt";
  }
  if (node.kind === "overclaim_dispute") {
    return "dispute";
  }
  return "gap";
}

function resolveTimelineLayer(stage: TimelineStage): TimelineLayer {
  if (stage === "policy_decision") {
    return "policy";
  }
  if (stage === "usage_rollup") {
    return "usage";
  }
  if (stage === "receipt" || stage === "correction_ref") {
    return "accounting";
  }
  if (stage === "dispute") {
    return "dispute";
  }
  if (stage === "gap") {
    return "diagnostic";
  }
  return "core";
}

function classifyTimelineRetry(
  reasonCodes: readonly ReasonCode[],
  status: TimelineNode["status"],
): TimelineRetryClass {
  const haystack = [status, ...reasonCodes].join(" ").toLowerCase();
  if (haystack.includes("retryable")) {
    return "retryable";
  }
  if (haystack.includes("denied")) {
    return "policy_denied";
  }
  if (haystack.includes("cancel")) {
    return "cancelled";
  }
  if (haystack.includes("timeout") || haystack.includes("timed_out")) {
    return "timed_out";
  }
  if (hasActiveDisputeSignal(haystack)) {
    return "disputed";
  }
  if (status === "unavailable" || status === "degraded") {
    return "waiting";
  }
  if (haystack.includes("failed") || haystack.includes("terminal")) {
    return "terminal";
  }
  return "none";
}

function resolveOwningService(node: RenderedTimelineNode): string {
  const dependencyRef = node.refs.find((ref) => ref.startsWith("dependency:"));
  if (dependencyRef) {
    return dependencyRef.replace("dependency:", "").toLowerCase();
  }
  if (node.kind === "seal_ledger_receipt") {
    return "seal_ledger";
  }
  if (node.kind === "gap") {
    return "overgate";
  }
  return node.kind.split("_")[0] || "overgate";
}

function createDependencyGap(dependency: AdminDependencyStatus, staleAgeMs: number): TimelineGap {
  const normalized = normalizeDependencyStatus(dependency);
  return {
    nodeId: `timeline_gap:${normalized.service}` as OverridRef,
    owningService: normalized.service,
    retryClass: normalized.status === "unavailable" ? "waiting" : "retryable",
    staleAgeMs,
    diagnosticReason: normalized.reason_code,
    refs: [`dependency:${normalized.service}` as OverridRef],
    reasonCodes: [normalized.reason_code],
    auditRefs: [],
  };
}

function normalizeDependencyStatus(status: AdminDependencyStatus): AdminDependencyStatus {
  return {
    ...status,
    service: status.service.toLowerCase(),
  };
}

function clampRefreshWindow(value: number | undefined): number {
  if (value === undefined) {
    return DEFAULT_REFRESH_WINDOW_MS;
  }
  if (!Number.isInteger(value) || value < MIN_REFRESH_WINDOW_MS) {
    return MIN_REFRESH_WINDOW_MS;
  }
  return value;
}

function isCopySafeRef(ref: OverridRef): boolean {
  return !UNSAFE_DIAGNOSTIC_PATTERNS.some((pattern) => pattern.test(ref));
}

function formatTimelineAuditRef(ref: TimelineAuditRef): string {
  return `${ref.source_service}:${ref.audit_id}:${ref.trace_id}`;
}

function hasActiveDisputeSignal(haystack: string): boolean {
  return ACTIVE_DISPUTE_SIGNALS.some((signal) => haystack.includes(signal));
}

function uniqueRefs(refs: readonly OverridRef[]): readonly OverridRef[] {
  return [...new Set(refs)];
}

function uniqueReasonCodes(reasonCodes: readonly ReasonCode[]): readonly ReasonCode[] {
  return [...new Set(reasonCodes)];
}

function uniqueAuditRefs(refs: readonly TimelineAuditRef[]): readonly TimelineAuditRef[] {
  const seen = new Set<string>();
  return refs.filter((ref) => {
    const key = formatTimelineAuditRef(ref);
    if (seen.has(key)) {
      return false;
    }
    seen.add(key);
    return true;
  });
}
