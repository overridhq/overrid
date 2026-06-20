import type {
  AdminActionReceipt,
  AdminActionRequest,
  AdminCapabilitiesResponse,
  AdminSessionContext,
  AuditRef,
  IdempotencyKey,
  OverridRef,
  ReasonCode,
  TraceId,
} from "./contracts";

export type AdminActionType = AdminActionRequest["action_type"];
export type AdminActionTarget = AdminActionRequest["target"];
export type AdminActionTargetKind = AdminActionTarget["target_kind"];
export type AdminActionReceiptOutcome = AdminActionReceipt["outcome"];
export type AdminActionRoutePath = "/admin/actions";
export type AdminActionMethod = "POST";
export type AdminActionDraftStatus = "blocked" | "ready_for_signing";
export type AdminActionRiskLevel = "bounded" | "operator_review" | "disabled_high_risk";
export type SigningHandoffStatus = "blocked" | "unsigned" | "malformed_signature" | "ready";
export type AdminActionRetryClass =
  | "no_retry"
  | "safe_retry_after_refresh"
  | "operator_review_required"
  | "retry_idempotent_duplicate"
  | "downstream_retry_after_receipt";

type PolicyRef = AdminActionRequest["policy_refs"][number];
type RequestAuditRef = AdminActionRequest["audit_refs"][number];

export const PHASE9_ACTION_ROUTE: AdminActionRoutePath = "/admin/actions";
export const PHASE9_ACTION_METHOD: AdminActionMethod = "POST";

export const SUPPORTED_ADMIN_ACTIONS: readonly AdminActionType[] = [
  "cancel_workload",
  "retry_workload",
  "pause_node",
  "drain_node",
  "annotate_dispute",
  "request_credential_rotation",
  "acknowledge_receipt",
];

export const PHASE9_HIGH_RISK_DENYLIST = [
  "backbone_maintenance",
  "forced_rollback",
  "break_glass_activation",
  "ledger_correction_execution",
  "provider_payout_override",
  "direct_data_repair",
  "raw_key_recovery",
  "annotate_incident",
] as const;

export type Phase9HighRiskAction = (typeof PHASE9_HIGH_RISK_DENYLIST)[number];

export const ACTION_TARGET_KIND: Readonly<Record<AdminActionType, AdminActionTargetKind>> = {
  cancel_workload: "workload",
  retry_workload: "workload",
  pause_node: "node",
  drain_node: "node",
  annotate_dispute: "dispute",
  request_credential_rotation: "credential",
  acknowledge_receipt: "receipt",
};

export const ACTION_RISK_LEVEL: Readonly<Record<AdminActionType, AdminActionRiskLevel>> = {
  cancel_workload: "bounded",
  retry_workload: "bounded",
  pause_node: "operator_review",
  drain_node: "operator_review",
  annotate_dispute: "bounded",
  request_credential_rotation: "operator_review",
  acknowledge_receipt: "bounded",
};

const RECEIPT_RETRY_CLASS: Readonly<Record<AdminActionReceiptOutcome, AdminActionRetryClass>> = {
  accepted: "no_retry",
  denied: "no_retry",
  duplicate: "retry_idempotent_duplicate",
  stale_expected_state: "safe_retry_after_refresh",
  downstream_failed: "downstream_retry_after_receipt",
  applied: "no_retry",
  completed: "no_retry",
  failed: "operator_review_required",
};

const TERMINAL_RECEIPT_OUTCOMES = new Set<AdminActionReceiptOutcome>([
  "denied",
  "duplicate",
  "stale_expected_state",
  "downstream_failed",
  "completed",
  "failed",
]);

const STATE_CHANGE_RECEIPT_OUTCOMES = new Set<AdminActionReceiptOutcome>(["applied", "completed"]);

export interface AdminActionDraftBlocker {
  readonly code:
    | "unsupported_action"
    | "disabled_high_risk_action"
    | "missing_reason"
    | "missing_expected_state"
    | "missing_trace"
    | "missing_idempotency_key"
    | "missing_policy_ref"
    | "missing_audit_ref"
    | "missing_signature"
    | "malformed_signature"
    | "capability_missing"
    | "tenant_mismatch"
    | "target_kind_mismatch"
    | "refresh_required"
    | "stale_expected_state";
  readonly reasonCode: ReasonCode;
  readonly detail: string;
}

export interface AdminActionDraftInput {
  readonly schemaVersion: AdminActionRequest["schema_version"];
  readonly actionType: string;
  readonly commandId: OverridRef;
  readonly tenantId: OverridRef;
  readonly actorId: OverridRef;
  readonly target: AdminActionTarget;
  readonly visibleActiveTenantId: OverridRef;
  readonly targetTenantId?: OverridRef;
  readonly reason: string;
  readonly expectedCurrentState: string;
  readonly observedCurrentState?: string;
  readonly idempotencyKey: IdempotencyKey;
  readonly traceId: TraceId;
  readonly policyRefs: readonly PolicyRef[];
  readonly auditRefs: readonly RequestAuditRef[];
  readonly capabilities: AdminCapabilitiesResponse;
  readonly refreshedAtEpochMs?: number;
  readonly nowEpochMs?: number;
  readonly staleAfterSeconds?: number;
}

export interface AdminActionDraft {
  readonly schemaVersion: AdminActionRequest["schema_version"];
  readonly actionType: string;
  readonly riskLevel: AdminActionRiskLevel;
  readonly commandId: OverridRef;
  readonly tenantId: OverridRef;
  readonly actorId: OverridRef;
  readonly target: AdminActionTarget;
  readonly visibleActiveTenantId: OverridRef;
  readonly targetTenantId?: OverridRef;
  readonly reason: string;
  readonly expectedCurrentState: string;
  readonly observedCurrentState?: string;
  readonly idempotencyKey: IdempotencyKey;
  readonly traceId: TraceId;
  readonly policyRefs: readonly PolicyRef[];
  readonly auditRefs: readonly RequestAuditRef[];
  readonly status: AdminActionDraftStatus;
  readonly blockers: readonly AdminActionDraftBlocker[];
  readonly refreshRequired: boolean;
  readonly staleStateProtected: boolean;
  readonly usesOvergateOnly: true;
  readonly directStorageAccess: false;
  readonly directServiceAccess: false;
  readonly generatedContractsProjectionOnly: true;
}

export interface AdminActionSigningHandoffInput {
  readonly draft: AdminActionDraft;
  readonly providerRef: OverridRef;
  readonly signingFlowRef: OverridRef;
  readonly signatureRefs: readonly OverridRef[];
  readonly malformedSignature?: boolean;
}

export interface AdminActionSigningHandoff {
  readonly status: SigningHandoffStatus;
  readonly providerRef: OverridRef;
  readonly signingFlowRef: OverridRef;
  readonly signatureRefs: readonly OverridRef[];
  readonly idempotencyKey: IdempotencyKey;
  readonly traceId: TraceId;
  readonly request?: AdminActionRequest;
  readonly blockers: readonly AdminActionDraftBlocker[];
  readonly reasonCodes: readonly ReasonCode[];
}

export interface AdminActionSubmissionRequest {
  readonly method: AdminActionMethod;
  readonly path: AdminActionRoutePath;
  readonly traceId: TraceId;
  readonly idempotencyKey: IdempotencyKey;
  readonly headers: Readonly<Record<string, string>>;
  readonly body: AdminActionRequest;
  readonly usesOvergateOnly: true;
  readonly directStorageAccess: false;
  readonly directServiceAccess: false;
}

export interface AdminActionReceiptPanel {
  readonly receiptId: OverridRef;
  readonly commandId: OverridRef;
  readonly tenantId: OverridRef;
  readonly actorId: OverridRef;
  readonly traceId: TraceId;
  readonly outcome: AdminActionReceiptOutcome;
  readonly overgateStatus:
    | "accepted"
    | "denied"
    | "duplicate"
    | "stale_expected_state"
    | "downstream_failed"
    | "applied"
    | "completed"
    | "failed";
  readonly owningServiceStateChange: boolean;
  readonly terminalOutcome: boolean;
  readonly retryClass: AdminActionRetryClass;
  readonly affectedRefs: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
  readonly auditRefs: readonly string[];
  readonly overwatchRefs: readonly string[];
  readonly timelineInsertion: {
    readonly traceId: TraceId;
    readonly commandId: OverridRef;
    readonly receiptId: OverridRef;
    readonly affectedRefs: readonly OverridRef[];
  };
}

export interface StaleStateProtectionResult {
  readonly ok: boolean;
  readonly requiresRefresh: boolean;
  readonly changedTargetState: boolean;
  readonly silentRetryAllowed: boolean;
  readonly operatorReviewRequired: boolean;
  readonly reasonCodes: readonly ReasonCode[];
}

export interface DisabledActionGate {
  readonly actionName: string;
  readonly disabled: true;
  readonly routeFeatureFlagBlocked: true;
  readonly directComponentInvocationBlocked: true;
  readonly reasonCode: ReasonCode;
}

export function createAdminActionDraft(input: AdminActionDraftInput): AdminActionDraft {
  const blockers = buildDraftBlockers(input);
  const supported = isSupportedAdminAction(input.actionType);
  const riskLevel = supported ? ACTION_RISK_LEVEL[input.actionType] : resolveUnknownActionRisk(input.actionType);
  const refreshRequired = blockers.some(
    (blocker) => blocker.code === "refresh_required" || blocker.code === "stale_expected_state",
  );

  return {
    schemaVersion: input.schemaVersion,
    actionType: input.actionType,
    riskLevel,
    commandId: input.commandId,
    tenantId: input.tenantId,
    actorId: input.actorId,
    target: input.target,
    visibleActiveTenantId: input.visibleActiveTenantId,
    targetTenantId: input.targetTenantId,
    reason: input.reason.trim(),
    expectedCurrentState: input.expectedCurrentState.trim(),
    observedCurrentState: input.observedCurrentState,
    idempotencyKey: input.idempotencyKey,
    traceId: input.traceId,
    policyRefs: [...input.policyRefs],
    auditRefs: [...input.auditRefs],
    status: blockers.length ? "blocked" : "ready_for_signing",
    blockers,
    refreshRequired,
    staleStateProtected: true,
    usesOvergateOnly: true,
    directStorageAccess: false,
    directServiceAccess: false,
    generatedContractsProjectionOnly: true,
  };
}

export function buildLocalSigningHandoff(input: AdminActionSigningHandoffInput): AdminActionSigningHandoff {
  if (input.draft.blockers.length) {
    return {
      status: "blocked",
      providerRef: input.providerRef,
      signingFlowRef: input.signingFlowRef,
      signatureRefs: [],
      idempotencyKey: input.draft.idempotencyKey,
      traceId: input.draft.traceId,
      blockers: input.draft.blockers,
      reasonCodes: input.draft.blockers.map((blocker) => blocker.reasonCode),
    };
  }

  if (!input.signatureRefs.length) {
    return {
      status: "unsigned",
      providerRef: input.providerRef,
      signingFlowRef: input.signingFlowRef,
      signatureRefs: [],
      idempotencyKey: input.draft.idempotencyKey,
      traceId: input.draft.traceId,
      blockers: [blocker("missing_signature", "admin_action.signature.missing", "Signed admin action handoff requires at least one signature ref.")],
      reasonCodes: ["admin_action.signature.missing" as ReasonCode],
    };
  }

  if (input.malformedSignature === true || input.signatureRefs.some((ref) => !isSignatureRef(ref))) {
    return {
      status: "malformed_signature",
      providerRef: input.providerRef,
      signingFlowRef: input.signingFlowRef,
      signatureRefs: [...input.signatureRefs],
      idempotencyKey: input.draft.idempotencyKey,
      traceId: input.draft.traceId,
      blockers: [
        blocker(
          "malformed_signature",
          "admin_action.signature.malformed",
          "Signed admin action handoff rejected malformed signature refs before Overgate submission.",
        ),
      ],
      reasonCodes: ["admin_action.signature.malformed" as ReasonCode],
    };
  }

  return {
    status: "ready",
    providerRef: input.providerRef,
    signingFlowRef: input.signingFlowRef,
    signatureRefs: [...input.signatureRefs],
    idempotencyKey: input.draft.idempotencyKey,
    traceId: input.draft.traceId,
    request: buildSignedAdminActionRequest(input.draft, input.signatureRefs),
    blockers: [],
    reasonCodes: ["admin_action.signature.ready" as ReasonCode],
  };
}

export function buildSignedAdminActionRequest(
  draft: AdminActionDraft,
  signatureRefs: readonly OverridRef[],
): AdminActionRequest {
  if (draft.blockers.length) {
    throw new Error(`Cannot build signed admin action request from blocked draft: ${draft.blockers[0].code}`);
  }
  if (!isSupportedAdminAction(draft.actionType)) {
    throw new Error(`Unsupported admin action: ${draft.actionType}`);
  }
  if (!signatureRefs.length || signatureRefs.some((ref) => !isSignatureRef(ref))) {
    throw new Error("Signed admin action request requires well-formed signature refs.");
  }

  return {
    schema_version: draft.schemaVersion,
    command_id: draft.commandId,
    tenant_id: draft.tenantId,
    actor_id: draft.actorId,
    target: draft.target,
    action_type: draft.actionType,
    reason: draft.reason,
    expected_current_state: draft.expectedCurrentState,
    idempotency_key: draft.idempotencyKey,
    trace_id: draft.traceId,
    signature_refs: [...signatureRefs] as AdminActionRequest["signature_refs"],
    policy_refs: [...draft.policyRefs] as AdminActionRequest["policy_refs"],
    audit_refs: [...draft.auditRefs] as AdminActionRequest["audit_refs"],
  };
}

export function buildOvergateActionSubmission(request: AdminActionRequest): AdminActionSubmissionRequest {
  assertSubmittableSignedAdminActionRequest(request);

  return {
    method: PHASE9_ACTION_METHOD,
    path: PHASE9_ACTION_ROUTE,
    traceId: request.trace_id,
    idempotencyKey: request.idempotency_key,
    headers: {
      accept: "application/json",
      "content-type": "application/json",
      "x-overrid-trace-id": request.trace_id,
      "x-overrid-schema-version": request.schema_version,
      "x-overrid-tenant-id": request.tenant_id,
      "x-overrid-actor-id": request.actor_id,
      "x-overrid-idempotency-key": request.idempotency_key,
      "x-overrid-signature-refs": request.signature_refs.join(","),
    },
    body: request,
    usesOvergateOnly: true,
    directStorageAccess: false,
    directServiceAccess: false,
  };
}

export function buildAdminActionReceiptPanel(receipt: AdminActionReceipt): AdminActionReceiptPanel {
  const auditRefs = receipt.audit_refs.map(formatAuditRef);
  const overwatchRefs = receipt.audit_refs
    .filter((auditRef) => auditRef.source_service === "overwatch")
    .map(formatAuditRef);

  return {
    receiptId: receipt.receipt_id,
    commandId: receipt.command_id,
    tenantId: receipt.tenant_id,
    actorId: receipt.actor_id,
    traceId: receipt.trace_id,
    outcome: receipt.outcome,
    overgateStatus: receipt.outcome,
    owningServiceStateChange: STATE_CHANGE_RECEIPT_OUTCOMES.has(receipt.outcome),
    terminalOutcome: TERMINAL_RECEIPT_OUTCOMES.has(receipt.outcome),
    retryClass: RECEIPT_RETRY_CLASS[receipt.outcome],
    affectedRefs: [...receipt.affected_refs],
    reasonCodes: [...receipt.reason_codes],
    auditRefs,
    overwatchRefs,
    timelineInsertion: {
      traceId: receipt.trace_id,
      commandId: receipt.command_id,
      receiptId: receipt.receipt_id,
      affectedRefs: [...receipt.affected_refs],
    },
  };
}

export function evaluateStaleStateProtection(
  draft: AdminActionDraft,
  observedCurrentState: string,
): StaleStateProtectionResult {
  const changedTargetState = Boolean(draft.expectedCurrentState && observedCurrentState !== draft.expectedCurrentState);
  const operatorReviewRequired = draft.riskLevel === "operator_review" || draft.riskLevel === "disabled_high_risk";
  const requiresRefresh = draft.refreshRequired || changedTargetState;
  const reasonCodes: ReasonCode[] = [];
  if (requiresRefresh) {
    reasonCodes.push("admin_action.stale_state.refresh_required" as ReasonCode);
  }
  if (operatorReviewRequired) {
    reasonCodes.push("admin_action.retry.operator_review_required" as ReasonCode);
  }

  return {
    ok: !requiresRefresh && draft.status === "ready_for_signing",
    requiresRefresh,
    changedTargetState,
    silentRetryAllowed: !requiresRefresh && !operatorReviewRequired,
    operatorReviewRequired,
    reasonCodes,
  };
}

export function buildDisabledActionGate(actionName: string): DisabledActionGate {
  return {
    actionName,
    disabled: true,
    routeFeatureFlagBlocked: true,
    directComponentInvocationBlocked: true,
    reasonCode: "admin_action.disabled_until_phase7_or_phase13_contract" as ReasonCode,
  };
}

export function isSupportedAdminAction(actionType: string): actionType is AdminActionType {
  return (SUPPORTED_ADMIN_ACTIONS as readonly string[]).includes(actionType);
}

export function isPhase9HighRiskDeniedAction(actionName: string): actionName is Phase9HighRiskAction {
  return (PHASE9_HIGH_RISK_DENYLIST as readonly string[]).includes(actionName);
}

function buildDraftBlockers(input: AdminActionDraftInput): readonly AdminActionDraftBlocker[] {
  const blockers: AdminActionDraftBlocker[] = [];
  const reason = input.reason.trim();
  const expectedCurrentState = input.expectedCurrentState.trim();

  if (!isSupportedAdminAction(input.actionType)) {
    blockers.push(
      blocker("unsupported_action", "admin_action.unsupported", `Unsupported action type: ${input.actionType}`),
    );
  }
  if (isPhase9HighRiskDeniedAction(input.actionType)) {
    blockers.push(
      blocker(
        "disabled_high_risk_action",
        "admin_action.disabled_until_phase7_or_phase13_contract",
        `High-risk action remains disabled in Phase 6 UI scope: ${input.actionType}`,
      ),
    );
  }
  if (!reason) {
    blockers.push(blocker("missing_reason", "admin_action.reason.required", "Action draft requires an operator reason."));
  }
  if (!expectedCurrentState) {
    blockers.push(
      blocker(
        "missing_expected_state",
        "admin_action.expected_state.required",
        "Action draft requires expected current state.",
      ),
    );
  }
  if (!input.traceId) {
    blockers.push(blocker("missing_trace", "admin_action.trace.required", "Action draft requires a trace id."));
  }
  if (!input.idempotencyKey || !String(input.idempotencyKey).startsWith("idem_")) {
    blockers.push(
      blocker(
        "missing_idempotency_key",
        "admin_action.idempotency.required",
        "Action draft requires a stable idempotency key.",
      ),
    );
  }
  if (!input.policyRefs.length) {
    blockers.push(blocker("missing_policy_ref", "admin_action.policy_ref.required", "Action draft requires policy refs."));
  }
  if (!input.auditRefs.length) {
    blockers.push(blocker("missing_audit_ref", "admin_action.audit_ref.required", "Action draft requires audit refs."));
  }
  if (!hasAdminActionCapability(input.capabilities)) {
    blockers.push(
      blocker(
        "capability_missing",
        "admin_action.capability.missing",
        "Admin capabilities must expose admin.actions.submit before mutation controls can be enabled.",
      ),
    );
  }
  if (input.visibleActiveTenantId !== input.tenantId || (input.targetTenantId && input.targetTenantId !== input.tenantId)) {
    blockers.push(
      blocker(
        "tenant_mismatch",
        "admin_action.tenant_mismatch",
        "Action tenant, visible active tenant, and target tenant must match.",
      ),
    );
  }
  if (isSupportedAdminAction(input.actionType) && input.target.target_kind !== ACTION_TARGET_KIND[input.actionType]) {
    blockers.push(
      blocker(
        "target_kind_mismatch",
        "admin_action.target_kind_mismatch",
        `Action ${input.actionType} requires target kind ${ACTION_TARGET_KIND[input.actionType]}.`,
      ),
    );
  }
  if (input.refreshedAtEpochMs === undefined) {
    blockers.push(
      blocker(
        "refresh_required",
        "admin_action.refresh.required",
        "Mutation drafts require a fresh target state before signing.",
      ),
    );
  } else if (isRefreshStale(input)) {
    blockers.push(
      blocker(
        "refresh_required",
        "admin_action.refresh.stale",
        "Mutation drafts require refresh when target state age exceeds the stale threshold.",
      ),
    );
  }
  if (input.observedCurrentState !== undefined && expectedCurrentState && input.observedCurrentState !== expectedCurrentState) {
    blockers.push(
      blocker(
        "stale_expected_state",
        "admin_action.stale_expected_state",
        "Observed target state no longer matches the draft expected current state.",
      ),
    );
  }

  return blockers;
}

function hasAdminActionCapability(capabilities: AdminCapabilitiesResponse): boolean {
  return capabilities.feature_flags.includes("admin.actions.submit");
}

function resolveUnknownActionRisk(actionType: string): AdminActionRiskLevel {
  return isPhase9HighRiskDeniedAction(actionType) ? "disabled_high_risk" : "bounded";
}

function isRefreshStale(input: AdminActionDraftInput): boolean {
  const staleAfterSeconds = input.staleAfterSeconds ?? 60;
  const nowEpochMs = input.nowEpochMs ?? input.refreshedAtEpochMs ?? 0;
  const refreshedAtEpochMs = input.refreshedAtEpochMs ?? 0;
  return nowEpochMs - refreshedAtEpochMs > staleAfterSeconds * 1000;
}

function assertSubmittableSignedAdminActionRequest(request: AdminActionRequest): void {
  if (!isSupportedAdminAction(request.action_type)) {
    throw new Error(`Unsupported admin action: ${request.action_type}`);
  }
  if (isPhase9HighRiskDeniedAction(request.action_type)) {
    throw new Error(`High-risk admin action remains disabled in Phase 9: ${request.action_type}`);
  }
  if (request.target.target_kind !== ACTION_TARGET_KIND[request.action_type]) {
    throw new Error(`Action ${request.action_type} requires target kind ${ACTION_TARGET_KIND[request.action_type]}.`);
  }
  if (!request.reason.trim()) {
    throw new Error("Signed admin action request requires an operator reason.");
  }
  if (!request.expected_current_state.trim()) {
    throw new Error("Signed admin action request requires expected current state.");
  }
  if (!request.trace_id) {
    throw new Error("Signed admin action request requires a trace id.");
  }
  if (!request.idempotency_key || !String(request.idempotency_key).startsWith("idem_")) {
    throw new Error("Signed admin action request requires a stable idempotency key.");
  }
  if (!request.policy_refs.length) {
    throw new Error("Signed admin action request requires policy refs.");
  }
  if (!request.audit_refs.length) {
    throw new Error("Signed admin action request requires audit refs.");
  }
  if (!request.signature_refs.length) {
    throw new Error("Signed admin action request requires at least one signature ref.");
  }
  if (request.signature_refs.some((ref) => !isSignatureRef(ref))) {
    throw new Error("Signed admin action request requires well-formed signature refs.");
  }
}

function isSignatureRef(ref: OverridRef): boolean {
  return String(ref).startsWith("signature:") && String(ref).length > "signature:".length;
}

function formatAuditRef(ref: AuditRef): string {
  return `${ref.source_service}:${ref.audit_id}:${ref.trace_id}`;
}

function blocker(
  code: AdminActionDraftBlocker["code"],
  reasonCode: string,
  detail: string,
): AdminActionDraftBlocker {
  return {
    code,
    reasonCode: reasonCode as ReasonCode,
    detail,
  };
}
