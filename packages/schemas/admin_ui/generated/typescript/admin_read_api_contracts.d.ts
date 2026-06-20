// Generated from packages/schemas/admin_ui/v0/admin_read_api_contracts.schema.json.
// Source of truth: canonical JSON Schema. Do not treat this projection as authoritative.

export type AdminReadApiSchemaVersion = `admin-ui-admin-api.v0.${number}`;
export type AdminUiSchemaVersion = `admin-ui.v0.${number}`;
export type OverridRef = `${string}:${string}`;
export type TraceId = `trace_${string}`;
export type ReasonCode = `${string}.${string}`;
export type NonEmptyArray<T> = [T, ...T[]];

export type AdminRole =
  | "platform_owner"
  | "tenant_owner"
  | "tenant_admin"
  | "product_integrator"
  | "support_viewer"
  | "incident_responder"
  | "accounting_viewer"
  | "service_account"
  | "system_service";

export type AdminRoutePath =
  | "/admin/tenants"
  | "/admin/identities"
  | "/admin/keys"
  | "/admin/nodes"
  | "/admin/packages"
  | "/admin/workloads"
  | "/admin/queue-items"
  | "/admin/policy-decisions"
  | "/admin/usage"
  | "/admin/ledger"
  | "/admin/receipts"
  | "/admin/disputes"
  | "/admin/workloads/{id}/timeline"
  | "/admin/capabilities";

export type AdminResourceKind =
  | "tenant"
  | "identity"
  | "key"
  | "node"
  | "package"
  | "workload"
  | "queue_item"
  | "policy_decision"
  | "usage"
  | "ledger"
  | "receipt"
  | "dispute"
  | "workload_timeline"
  | "capability";

export type AdminCapabilityFlag =
  | "admin.session.read"
  | "admin.summary.read"
  | "admin.timeline.read"
  | "admin.diagnostics.copy"
  | "admin.actions.submit";

export type AdminDataClass =
  | "public_ref"
  | "tenant_private_ref"
  | "regulated_ref"
  | "encrypted_private_ref"
  | "system_service_only_ref"
  | "redacted_diagnostic_ref";

export type AdminPolicyScope =
  | "admin.read.session"
  | "admin.read.summary"
  | "admin.read.timeline"
  | "admin.read.accounting"
  | "admin.read.policy"
  | "admin.read.incident"
  | "admin.read.capability";

export type AdminTimelineNodeKind =
  | "overgate_request"
  | "overqueue_item"
  | "oversched_placement"
  | "overlease_reservation"
  | "overcell_execution"
  | "overrun_result"
  | "overguard_decision"
  | "overmeter_rollup"
  | "seal_ledger_receipt"
  | "overclaim_dispute"
  | "gap";

export interface AdminAuditRef {
  audit_id: OverridRef;
  source_service: string;
  trace_id: TraceId;
}

export interface AdminCursorPagination {
  supported: boolean;
  default_limit: number;
  max_limit: number;
  cursor_field: string;
}

export interface AdminAuthorizationFilter {
  tenant_scoped: boolean;
  actor_scoped: boolean;
  roles_allowed: NonEmptyArray<AdminRole>;
  data_classes: NonEmptyArray<AdminDataClass>;
  policy_scopes: NonEmptyArray<AdminPolicyScope>;
  denies_cross_tenant: boolean;
  server_side_only: boolean;
}

export interface AdminRedactionRule {
  profile_ref: OverridRef;
  redacted_fields: string[];
  high_risk_fields_fail_closed: boolean;
}

export interface AdminReadRoute {
  schema_version: AdminReadApiSchemaVersion;
  route_id: OverridRef;
  method: "GET";
  path: AdminRoutePath;
  resource_kind: AdminResourceKind;
  owning_service: "overgate";
  capability: AdminCapabilityFlag;
  response_contract: "admin_list_response" | "admin_timeline_response" | "admin_capabilities_response";
  error_contract: "admin_error_response";
  bounded_filters: NonEmptyArray<string>;
  cursor_pagination: AdminCursorPagination;
  server_authorization: AdminAuthorizationFilter;
  redaction: AdminRedactionRule;
  audit_refs: NonEmptyArray<AdminAuditRef>;
}

export interface AdminListResponse {
  schema_version: AdminReadApiSchemaVersion;
  path: AdminRoutePath;
  resource_kind: string;
  schema_versions: {
    admin_ui_contracts: AdminUiSchemaVersion;
    admin_read_api_contracts: AdminReadApiSchemaVersion;
  };
  status: "ok" | "degraded" | "empty";
  items: AdminReadModelSummary[];
  page: {
    limit: number;
    cursor_ref: string;
    next_cursor_ref?: string;
  };
  degraded_dependencies: AdminDependencyStatus[];
  audit_refs: NonEmptyArray<AdminAuditRef>;
}

export interface AdminReadModelSummary {
  id: OverridRef;
  tenant_id: OverridRef;
  kind: AdminResourceKind;
  state: string;
  trace_id: TraceId;
  schema_version: AdminUiSchemaVersion;
  reason_codes: NonEmptyArray<ReasonCode>;
  redaction: AdminRedactionRule;
  audit_refs: NonEmptyArray<AdminAuditRef>;
}

export interface AdminTimelineResponse {
  schema_version: AdminReadApiSchemaVersion;
  path: "/admin/workloads/{id}/timeline";
  workload_id: OverridRef;
  trace_id: TraceId;
  status: "complete" | "partial" | "unavailable";
  nodes: NonEmptyArray<AdminTimelineNode>;
  partial_dependencies: AdminDependencyStatus[];
  audit_refs: NonEmptyArray<AdminAuditRef>;
}

export interface AdminTimelineNode {
  node_id: OverridRef;
  kind: AdminTimelineNodeKind;
  status: "available" | "unavailable" | "degraded" | "redacted";
  trace_id: TraceId;
  refs: NonEmptyArray<OverridRef>;
  reason_codes: NonEmptyArray<ReasonCode>;
  audit_refs: NonEmptyArray<AdminAuditRef>;
}

export interface AdminAuthorizationMatrix {
  schema_version: AdminReadApiSchemaVersion;
  roles: NonEmptyArray<{
    role: AdminRole;
    allowed_routes: NonEmptyArray<AdminRoutePath>;
    denied_fields: string[];
    data_classes: NonEmptyArray<AdminDataClass>;
    policy_scopes: NonEmptyArray<AdminPolicyScope>;
    cross_tenant_access: boolean;
  }>;
  fail_closed_defaults: {
    missing_role_denied: boolean;
    missing_tenant_denied: boolean;
    high_risk_field_redacted: boolean;
  };
  audit_refs: NonEmptyArray<AdminAuditRef>;
}

export interface AdminCapabilitiesResponse {
  schema_version: AdminReadApiSchemaVersion;
  backend_schema_version: AdminUiSchemaVersion;
  generated_at: string;
  routes: NonEmptyArray<{
    path: AdminRoutePath;
    available: boolean;
    required_capability: AdminCapabilityFlag;
    schema_version: AdminReadApiSchemaVersion;
    reason_codes: ReasonCode[];
  }>;
  feature_flags: NonEmptyArray<AdminCapabilityFlag>;
  limits: {
    route_max_limit: number;
    filter_max_count: number;
    cursor_required: boolean;
  };
  disabled_panels: Array<{
    panel: string;
    reason_code: ReasonCode;
    dependency_status: AdminDependencyStatus;
  }>;
  audit_refs: NonEmptyArray<AdminAuditRef>;
}

export interface AdminDependencyStatus {
  service: string;
  status: "available" | "unavailable" | "degraded" | "redacted";
  reason_code: ReasonCode;
}

export interface AdminErrorResponse {
  schema_version: AdminReadApiSchemaVersion;
  status: number;
  reason_code: ReasonCode;
  trace_id: TraceId;
  retryable: boolean;
  user_visible_message: string;
  details_ref: OverridRef;
  audit_refs: NonEmptyArray<AdminAuditRef>;
}
