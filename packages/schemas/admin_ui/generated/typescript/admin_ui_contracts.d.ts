// Generated from packages/schemas/admin_ui/v0/admin_ui_contracts.schema.json.
// Source of truth: canonical JSON Schema. Do not treat this projection as authoritative.

export type AdminUiSchemaVersion = `admin-ui.v0.${number}`;
export type OverridRef = `${string}:${string}`;
export type TraceId = `trace_${string}`;
export type IdempotencyKey = `idem_${string}`;
export type ReasonCode = `${string}.${string}`;
export type NonEmptyArray<T> = [T, ...T[]];
export type CapabilityFlag =
  | "admin.session.read"
  | "admin.summary.read"
  | "admin.timeline.read"
  | "admin.diagnostics.copy"
  | "admin.actions.submit";

export interface AuditRef {
  audit_id: OverridRef;
  source_service: string;
  trace_id: TraceId;
}

export interface PolicyRef {
  policy_id: OverridRef;
  policy_version: string;
  decision_ref: OverridRef;
}

export interface RedactionProfile {
  profile_id: OverridRef;
  classification:
    | "public_ref"
    | "tenant_private_ref"
    | "regulated_ref"
    | "encrypted_private_ref"
    | "system_service_only_ref"
    | "redacted_diagnostic_ref";
  ruleset_version: string;
  hidden_fields: string[];
}

export interface RoleBinding {
  tenant_id: OverridRef;
  actor_id: OverridRef;
  role:
    | "platform_owner"
    | "tenant_owner"
    | "tenant_admin"
    | "product_integrator"
    | "support_viewer"
    | "incident_responder"
    | "accounting_viewer"
    | "service_account"
    | "system_service";
}

export interface AdminSessionContext {
  schema_version: AdminUiSchemaVersion;
  active_tenant_id: OverridRef;
  actor_id: OverridRef;
  selected_environment: {
    environment_id: OverridRef;
    display_name: string;
    overgate_route_scope: string;
  };
  role_bindings: NonEmptyArray<RoleBinding>;
  redaction_profile: RedactionProfile;
  visible_capabilities: CapabilityFlag[];
  audit_refs: NonEmptyArray<AuditRef>;
}

export interface ResourceSummaryView {
  schema_version: AdminUiSchemaVersion;
  kind:
    | "tenant"
    | "identity"
    | "key"
    | "node"
    | "package"
    | "workload"
    | "queue_item"
    | "lease"
    | "usage"
    | "receipt"
    | "dispute"
    | "policy_decision";
  resource_id: OverridRef;
  tenant_id: OverridRef;
  trace_id: TraceId;
  display_name: string;
  state: string;
  pagination: { limit: number; cursor_ref: string };
  stale_data: { as_of: string; stale_after_seconds: number };
  redaction_markers: string[];
  reason_codes: ReasonCode[];
  audit_refs: NonEmptyArray<AuditRef>;
}

export interface TimelineNode {
  node_id: OverridRef;
  kind:
    | "overgate_request"
    | "overqueue_item"
    | "oversched_placement"
    | "overlease_reservation"
    | "overcell_assignment"
    | "overrun_execution"
    | "overguard_decision"
    | "overmeter_rollup"
    | "seal_ledger_receipt"
    | "overclaim_dispute"
    | "gap";
  state: string;
  occurred_at: string;
  owning_service: string;
  refs: OverridRef[];
  reason_codes: ReasonCode[];
}

export interface TimelineEdge {
  from: OverridRef;
  to: OverridRef;
  edge_kind: "trace_next" | "caused_by" | "blocked_by" | "resolved_by";
}

export interface JobTimelineView {
  schema_version: AdminUiSchemaVersion;
  workload_id: OverridRef;
  tenant_id: OverridRef;
  trace_id: TraceId;
  outcome: "successful" | "denied" | "cancelled" | "timed_out" | "disputed";
  nodes: NonEmptyArray<TimelineNode>;
  edges: TimelineEdge[];
  audit_refs: NonEmptyArray<AuditRef>;
}

export interface AdminActionTarget {
  target_id: OverridRef;
  target_kind: "workload" | "node" | "dispute" | "credential" | "receipt" | "incident";
  owning_service: string;
}

export interface AdminActionRequest {
  schema_version: AdminUiSchemaVersion;
  command_id: OverridRef;
  tenant_id: OverridRef;
  actor_id: OverridRef;
  target: AdminActionTarget;
  action_type:
    | "cancel_workload"
    | "retry_workload"
    | "pause_node"
    | "drain_node"
    | "annotate_dispute"
    | "request_credential_rotation"
    | "acknowledge_receipt";
  reason: string;
  expected_current_state: string;
  idempotency_key: IdempotencyKey;
  trace_id: TraceId;
  signature_refs: NonEmptyArray<OverridRef>;
  policy_refs: NonEmptyArray<PolicyRef>;
  audit_refs: NonEmptyArray<AuditRef>;
}

export interface AdminActionReceipt {
  schema_version: AdminUiSchemaVersion;
  receipt_id: OverridRef;
  command_id: OverridRef;
  tenant_id: OverridRef;
  actor_id: OverridRef;
  trace_id: TraceId;
  outcome:
    | "accepted"
    | "denied"
    | "duplicate"
    | "stale_expected_state"
    | "downstream_failed"
    | "applied"
    | "completed"
    | "failed";
  affected_refs: OverridRef[];
  reason_codes: ReasonCode[];
  audit_refs: NonEmptyArray<AuditRef>;
}

export interface UiDiagnosticEvent {
  schema_version: AdminUiSchemaVersion;
  event_id: OverridRef;
  occurred_at: string;
  trace_id: TraceId;
  severity: "info" | "warning" | "error";
  reason_code: ReasonCode;
  message: string;
  redaction_profile: RedactionProfile;
  safe_refs: OverridRef[];
}
