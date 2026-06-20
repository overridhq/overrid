import type { AdminCapabilityFlag, AdminRoutePath } from "./contracts";

export type PanelId =
  | "overview"
  | "tenants"
  | "identities"
  | "keys"
  | "nodes"
  | "packages"
  | "workloads"
  | "queue"
  | "policy"
  | "usage"
  | "ledger"
  | "disputes"
  | "receipts"
  | "diagnostics";

export interface OperatorColumnDefinition {
  readonly key: string;
  readonly label: string;
  readonly minWidth: number;
  readonly priority: "primary" | "secondary" | "detail";
}

export interface OperatorPanelDefinition {
  readonly id: PanelId;
  readonly label: string;
  readonly route: AdminRoutePath | "local:diagnostics";
  readonly requiredCapability: AdminCapabilityFlag;
  readonly ariaLabel: string;
  readonly minTableWidth: number;
  readonly columns: readonly OperatorColumnDefinition[];
  readonly defaultFilters: readonly string[];
  readonly emptyState: string;
  readonly loadingState: string;
}

const SUMMARY_COLUMNS: readonly OperatorColumnDefinition[] = [
  { key: "state", label: "State", minWidth: 112, priority: "primary" },
  { key: "reason_codes", label: "Reason codes", minWidth: 220, priority: "secondary" },
  { key: "trace_id", label: "Trace id", minWidth: 180, priority: "secondary" },
  { key: "audit_refs", label: "Audit refs", minWidth: 180, priority: "detail" },
];

const TENANT_COLUMNS: readonly OperatorColumnDefinition[] = [
  ...SUMMARY_COLUMNS,
  { key: "tenant_id", label: "Tenant id", minWidth: 160, priority: "secondary" },
  { key: "role_bindings", label: "Role bindings", minWidth: 180, priority: "detail" },
  { key: "quota_state", label: "Quota state", minWidth: 140, priority: "detail" },
];

const IDENTITY_COLUMNS: readonly OperatorColumnDefinition[] = [
  ...SUMMARY_COLUMNS,
  { key: "identity_type", label: "Identity type", minWidth: 140, priority: "secondary" },
  { key: "role_bindings", label: "Role bindings", minWidth: 180, priority: "detail" },
  { key: "service_account", label: "Service account", minWidth: 150, priority: "detail" },
  { key: "system_service", label: "System service", minWidth: 150, priority: "detail" },
  { key: "last_seen_at", label: "Last seen", minWidth: 160, priority: "detail" },
];

const KEY_COLUMNS: readonly OperatorColumnDefinition[] = [
  ...SUMMARY_COLUMNS,
  { key: "key_metadata_ref", label: "Key metadata", minWidth: 180, priority: "secondary" },
  { key: "rotation_state", label: "Rotation", minWidth: 140, priority: "secondary" },
  { key: "revocation_state", label: "Revocation", minWidth: 140, priority: "secondary" },
  { key: "last_used_at", label: "Last used", minWidth: 160, priority: "detail" },
];

const NODE_COLUMNS: readonly OperatorColumnDefinition[] = [
  ...SUMMARY_COLUMNS,
  { key: "health", label: "Health", minWidth: 120, priority: "primary" },
  { key: "heartbeat_age", label: "Heartbeat age", minWidth: 150, priority: "secondary" },
  { key: "capability_records", label: "Capabilities", minWidth: 180, priority: "secondary" },
  { key: "trust_class", label: "Trust class", minWidth: 140, priority: "secondary" },
  { key: "region", label: "Region", minWidth: 120, priority: "secondary" },
  { key: "current_leases", label: "Leases", minWidth: 120, priority: "detail" },
  { key: "verification_state", label: "Verification", minWidth: 150, priority: "detail" },
  { key: "benchmark_refs", label: "Benchmarks", minWidth: 170, priority: "detail" },
  { key: "drain_readiness", label: "Drain", minWidth: 120, priority: "detail" },
  { key: "maintenance_readiness", label: "Maintenance", minWidth: 150, priority: "detail" },
];

const PACKAGE_COLUMNS: readonly OperatorColumnDefinition[] = [
  ...SUMMARY_COLUMNS,
  { key: "manifest_ref", label: "Manifest", minWidth: 180, priority: "secondary" },
  { key: "package_ref", label: "Package ref", minWidth: 180, priority: "secondary" },
  { key: "verification_state", label: "Verification", minWidth: 150, priority: "secondary" },
  { key: "product_family", label: "Product", minWidth: 120, priority: "detail" },
];

const WORKLOAD_COLUMNS: readonly OperatorColumnDefinition[] = [
  ...SUMMARY_COLUMNS,
  { key: "workload_request_ref", label: "Request", minWidth: 180, priority: "secondary" },
  { key: "manifest_ref", label: "Manifest", minWidth: 180, priority: "secondary" },
  { key: "package_ref", label: "Package ref", minWidth: 180, priority: "secondary" },
  { key: "queue_state", label: "Queue state", minWidth: 140, priority: "secondary" },
  { key: "priority", label: "Priority", minWidth: 120, priority: "secondary" },
  { key: "retry_count", label: "Retries", minWidth: 110, priority: "detail" },
  { key: "cancellation_eligible", label: "Cancellable", minWidth: 130, priority: "detail" },
  { key: "timeout_state", label: "Timeout", minWidth: 130, priority: "detail" },
  { key: "terminal_outcome", label: "Outcome", minWidth: 140, priority: "detail" },
  { key: "stale_age", label: "Stale age", minWidth: 120, priority: "secondary" },
];

const QUEUE_COLUMNS: readonly OperatorColumnDefinition[] = [
  ...SUMMARY_COLUMNS,
  { key: "queue_state", label: "Queue state", minWidth: 140, priority: "primary" },
  { key: "priority", label: "Priority", minWidth: 120, priority: "secondary" },
  { key: "retry_count", label: "Retries", minWidth: 110, priority: "detail" },
  { key: "cancellation_eligible", label: "Cancellable", minWidth: 130, priority: "detail" },
  { key: "timeout_state", label: "Timeout", minWidth: 130, priority: "detail" },
  { key: "terminal_outcome", label: "Outcome", minWidth: 140, priority: "detail" },
  { key: "stale_age", label: "Stale age", minWidth: 120, priority: "secondary" },
];

export const OPERATOR_PANELS: readonly OperatorPanelDefinition[] = [
  {
    id: "overview",
    label: "Overview",
    route: "/admin/capabilities",
    requiredCapability: "admin.session.read",
    ariaLabel: "Overview panel with session and dependency status",
    minTableWidth: 720,
    columns: SUMMARY_COLUMNS,
    defaultFilters: ["dependency_status", "capability"],
    emptyState: "No session context is available.",
    loadingState: "Loading session context.",
  },
  {
    id: "tenants",
    label: "Tenants",
    route: "/admin/tenants",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Tenant summary table",
    minTableWidth: 840,
    columns: TENANT_COLUMNS,
    defaultFilters: ["state", "role"],
    emptyState: "No visible tenants match the current filters.",
    loadingState: "Loading tenants.",
  },
  {
    id: "identities",
    label: "Identities",
    route: "/admin/identities",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Identity summary table",
    minTableWidth: 880,
    columns: IDENTITY_COLUMNS,
    defaultFilters: ["role", "state"],
    emptyState: "No visible identities match the current filters.",
    loadingState: "Loading identities.",
  },
  {
    id: "keys",
    label: "Keys",
    route: "/admin/keys",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Key metadata table without key material",
    minTableWidth: 880,
    columns: KEY_COLUMNS,
    defaultFilters: ["rotation_state", "revocation_state"],
    emptyState: "No visible key metadata matches the current filters.",
    loadingState: "Loading key metadata.",
  },
  {
    id: "nodes",
    label: "Nodes",
    route: "/admin/nodes",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Node capability and health table",
    minTableWidth: 960,
    columns: NODE_COLUMNS,
    defaultFilters: ["health", "trust_class", "region"],
    emptyState: "No visible nodes match the current filters.",
    loadingState: "Loading nodes.",
  },
  {
    id: "packages",
    label: "Packages",
    route: "/admin/packages",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Package manifest and verification summary table",
    minTableWidth: 1000,
    columns: PACKAGE_COLUMNS,
    defaultFilters: ["state", "package_ref", "product_family"],
    emptyState: "No package summaries match the current filters.",
    loadingState: "Loading package summaries.",
  },
  {
    id: "workloads",
    label: "Workloads",
    route: "/admin/workloads",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Workload and queue summary table",
    minTableWidth: 1040,
    columns: WORKLOAD_COLUMNS,
    defaultFilters: ["state", "trace_id", "stale"],
    emptyState: "No workloads match the current filters.",
    loadingState: "Loading workloads.",
  },
  {
    id: "queue",
    label: "Queue",
    route: "/admin/queue-items",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Queue item summary table",
    minTableWidth: 1000,
    columns: QUEUE_COLUMNS,
    defaultFilters: ["state", "trace_id", "priority", "stale"],
    emptyState: "No queue items match the current filters.",
    loadingState: "Loading queue items.",
  },
  {
    id: "policy",
    label: "Policy",
    route: "/admin/policy-decisions",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Policy decision summary table",
    minTableWidth: 920,
    columns: SUMMARY_COLUMNS,
    defaultFilters: ["decision", "reason_code"],
    emptyState: "No policy decisions match the current filters.",
    loadingState: "Loading policy decisions.",
  },
  {
    id: "usage",
    label: "Usage",
    route: "/admin/usage",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Usage summary table",
    minTableWidth: 920,
    columns: SUMMARY_COLUMNS,
    defaultFilters: ["state", "tenant"],
    emptyState: "No usage summaries match the current filters.",
    loadingState: "Loading usage.",
  },
  {
    id: "ledger",
    label: "Ledger",
    route: "/admin/ledger",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Ledger summary table",
    minTableWidth: 920,
    columns: SUMMARY_COLUMNS,
    defaultFilters: ["state", "receipt_ref"],
    emptyState: "No ledger rows match the current filters.",
    loadingState: "Loading ledger.",
  },
  {
    id: "disputes",
    label: "Disputes",
    route: "/admin/disputes",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Dispute summary table",
    minTableWidth: 920,
    columns: SUMMARY_COLUMNS,
    defaultFilters: ["state", "reason_code"],
    emptyState: "No disputes match the current filters.",
    loadingState: "Loading disputes.",
  },
  {
    id: "receipts",
    label: "Receipts",
    route: "/admin/receipts",
    requiredCapability: "admin.summary.read",
    ariaLabel: "Receipt summary table",
    minTableWidth: 920,
    columns: SUMMARY_COLUMNS,
    defaultFilters: ["state", "trace_id"],
    emptyState: "No receipts match the current filters.",
    loadingState: "Loading receipts.",
  },
  {
    id: "diagnostics",
    label: "Diagnostics",
    route: "local:diagnostics",
    requiredCapability: "admin.diagnostics.copy",
    ariaLabel: "Copy-safe local diagnostics panel",
    minTableWidth: 760,
    columns: SUMMARY_COLUMNS,
    defaultFilters: ["reason_code", "severity"],
    emptyState: "No copy-safe diagnostic references are available.",
    loadingState: "Preparing diagnostic references.",
  },
];

export const OPERATOR_PANEL_IDS: readonly PanelId[] = OPERATOR_PANELS.map((panel) => panel.id);

export function findOperatorPanel(panelId: PanelId): OperatorPanelDefinition {
  const panel = OPERATOR_PANELS.find((candidate) => candidate.id === panelId);
  if (!panel) {
    throw new Error(`Unknown operator panel: ${panelId}`);
  }
  return panel;
}

export function getPanelsForCapabilities(capabilities: readonly AdminCapabilityFlag[]): readonly OperatorPanelDefinition[] {
  const capabilitySet = new Set(capabilities);
  return OPERATOR_PANELS.filter((panel) => capabilitySet.has(panel.requiredCapability));
}
