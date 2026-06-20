import type {
  AdminCapabilitiesResponse,
  AdminDependencyStatus,
  AdminListResponse,
  AdminRoutePath,
  AdminTimelineResponse,
  OverridRef,
  ReasonCode,
  TraceId,
} from "./contracts";

export type EvidenceViewKind = "policy" | "verification" | "dispute" | "incident";
export type EvidenceViewStatus = "ready" | "degraded" | "disabled" | "empty" | "redacted";
export type EvidenceRefKind =
  | "trace"
  | "audit"
  | "policy"
  | "matched_rule"
  | "input_fact"
  | "placement"
  | "provider_verification"
  | "challenge"
  | "benchmark"
  | "trust_class"
  | "dispute"
  | "correction"
  | "refund"
  | "settlement"
  | "incident"
  | "owning_service";

export type PolicyDecisionCase =
  | "allowed"
  | "denied_egress"
  | "insufficient_trust"
  | "quota_exhaustion"
  | "package_trust_failure"
  | "wrong_tenant"
  | "budget_precheck_failure";

export type VerificationEvidenceState =
  | "verified"
  | "degraded"
  | "challenged"
  | "expired"
  | "disputed"
  | "untrusted";

export type DisputeCorrectionState =
  | "open"
  | "hold"
  | "challenge_window"
  | "corrected"
  | "refunded"
  | "settled"
  | "closed";

export type IncidentReadinessState = "ready" | "disabled" | "blocked" | "degraded" | "unavailable";

type EvidenceSummary = AdminListResponse["items"][number];
type TimelineNode = AdminTimelineResponse["nodes"][number];
type TimelineAuditRef = AdminTimelineResponse["audit_refs"][number];

export interface EvidenceAttachment {
  readonly kind: EvidenceRefKind;
  readonly ref: OverridRef;
  readonly sourceService?: string;
  readonly traceId?: TraceId;
  readonly reasonCodes?: readonly ReasonCode[];
}

export interface EvidenceLink {
  readonly kind: EvidenceRefKind;
  readonly ref: OverridRef;
  readonly sourceService: string;
  readonly traceId?: TraceId;
  readonly reasonCodes: readonly ReasonCode[];
  readonly auditRefs: readonly string[];
  readonly stable: boolean;
}

export interface PolicyDecisionEvidenceRecord {
  readonly decisionId: OverridRef;
  readonly tenantId: OverridRef;
  readonly traceId: TraceId;
  readonly state: string;
  readonly caseType: PolicyDecisionCase;
  readonly policyVersionRefs: readonly OverridRef[];
  readonly matchedRuleRefs: readonly OverridRef[];
  readonly inputFactRefs: readonly OverridRef[];
  readonly expectedPlacementClass?: string;
  readonly deniedClass?: string;
  readonly allowedClass?: string;
  readonly correctionOptions: readonly ReasonCode[];
  readonly reasonCodes: readonly ReasonCode[];
  readonly evidenceLinks: readonly EvidenceLink[];
}

export interface VerificationEvidenceRecord {
  readonly subjectRef: OverridRef;
  readonly subjectKind: "node" | "provider" | "package" | "workload";
  readonly tenantId: OverridRef;
  readonly traceId: TraceId;
  readonly state: VerificationEvidenceState;
  readonly providerVerificationRefs: readonly OverridRef[];
  readonly challengeRefs: readonly OverridRef[];
  readonly benchmarkRefs: readonly OverridRef[];
  readonly trustClassRefs: readonly OverridRef[];
  readonly stale: boolean;
  readonly redacted: boolean;
  readonly reasonCodes: readonly ReasonCode[];
  readonly evidenceLinks: readonly EvidenceLink[];
}

export interface DisputeCorrectionRecord {
  readonly caseRef: OverridRef;
  readonly tenantId: OverridRef;
  readonly traceId: TraceId;
  readonly state: DisputeCorrectionState;
  readonly evidenceRefs: readonly OverridRef[];
  readonly holdSettlementVisibility: boolean;
  readonly challengeWindowOpen: boolean;
  readonly correctionOutcomeRefs: readonly OverridRef[];
  readonly refundRefs: readonly OverridRef[];
  readonly settlementImpactRefs: readonly OverridRef[];
  readonly directLedgerMutation: false;
  readonly reasonCodes: readonly ReasonCode[];
  readonly evidenceLinks: readonly EvidenceLink[];
}

export interface IncidentReadinessRecord {
  readonly incidentRef: OverridRef;
  readonly state: IncidentReadinessState;
  readonly policyState: "available" | "missing" | "degraded";
  readonly dependencyStatuses: readonly AdminDependencyStatus[];
  readonly missingContracts: readonly string[];
  readonly disabledActionReasonCodes: readonly ReasonCode[];
  readonly breakGlassExecutionDisabled: true;
  readonly evidenceLinks: readonly EvidenceLink[];
}

export interface EvidenceLinkConsistencyResult {
  readonly ok: boolean;
  readonly missingStableRefIds: readonly OverridRef[];
  readonly privateRefIds: readonly OverridRef[];
  readonly mutableRefIds: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
}

export interface PolicyDecisionExplorerState {
  readonly kind: "policy";
  readonly route: AdminRoutePath;
  readonly status: EvidenceViewStatus;
  readonly records: readonly PolicyDecisionEvidenceRecord[];
  readonly requiredCaseCoverage: readonly PolicyDecisionCase[];
  readonly missingCaseCoverage: readonly PolicyDecisionCase[];
  readonly linkCheck: EvidenceLinkConsistencyResult;
}

export interface VerificationEvidencePanelState {
  readonly kind: "verification";
  readonly status: EvidenceViewStatus;
  readonly records: readonly VerificationEvidenceRecord[];
  readonly requiredStateCoverage: readonly VerificationEvidenceState[];
  readonly missingStateCoverage: readonly VerificationEvidenceState[];
  readonly dependencyStatuses: readonly AdminDependencyStatus[];
  readonly linkCheck: EvidenceLinkConsistencyResult;
}

export interface DisputeCorrectionViewState {
  readonly kind: "dispute";
  readonly route: AdminRoutePath;
  readonly status: EvidenceViewStatus;
  readonly records: readonly DisputeCorrectionRecord[];
  readonly linkCheck: EvidenceLinkConsistencyResult;
}

export interface IncidentReadinessViewState {
  readonly kind: "incident";
  readonly status: EvidenceViewStatus;
  readonly records: readonly IncidentReadinessRecord[];
  readonly breakGlassExecutionDisabled: true;
  readonly dependencyStatuses: readonly AdminDependencyStatus[];
  readonly linkCheck: EvidenceLinkConsistencyResult;
}

export interface Phase7EvidenceWorkspaceState {
  readonly schemaVersion: "operator-policy-evidence.v0.1";
  readonly policy: PolicyDecisionExplorerState;
  readonly verification: VerificationEvidencePanelState;
  readonly disputes: DisputeCorrectionViewState;
  readonly incidents: IncidentReadinessViewState;
  readonly readOnly: true;
  readonly usesOvergateOnly: true;
  readonly directLedgerMutation: false;
  readonly linkCheck: EvidenceLinkConsistencyResult;
}

export interface Phase7EvidenceWorkspaceInput {
  readonly policyResponse: AdminListResponse;
  readonly verificationResponses: readonly AdminListResponse[];
  readonly disputeResponse: AdminListResponse;
  readonly capabilities: AdminCapabilitiesResponse;
  readonly timeline?: AdminTimelineResponse;
  readonly attachments?: Readonly<Record<string, readonly EvidenceAttachment[]>>;
  readonly incidentRefs?: readonly OverridRef[];
}

export const PHASE7_EVIDENCE_ROUTES: readonly AdminRoutePath[] = [
  "/admin/policy-decisions",
  "/admin/nodes",
  "/admin/workloads",
  "/admin/disputes",
  "/admin/capabilities",
];

export const REQUIRED_POLICY_DENIAL_CASES: readonly Exclude<PolicyDecisionCase, "allowed">[] = [
  "denied_egress",
  "insufficient_trust",
  "quota_exhaustion",
  "package_trust_failure",
  "wrong_tenant",
  "budget_precheck_failure",
];

export const REQUIRED_VERIFICATION_STATES: readonly VerificationEvidenceState[] = [
  "verified",
  "degraded",
  "challenged",
  "expired",
  "disputed",
  "untrusted",
];

export const REQUIRED_INCIDENT_BREAK_GLASS_CONTRACTS: readonly string[] = [
  "overgate.signed_break_glass_command",
  "overkey.break_glass_expiry",
  "overguard.break_glass_policy",
  "overwatch.break_glass_receipt",
];

const STABLE_EVIDENCE_PREFIXES = [
  "audit",
  "benchmark",
  "challenge",
  "command",
  "correction",
  "decision",
  "dependency",
  "dispute",
  "evidence",
  "fact",
  "incident",
  "node",
  "overclaim",
  "overguard",
  "oververify",
  "overwatch",
  "package",
  "placement",
  "policy",
  "provider",
  "receipt",
  "refund",
  "rule",
  "settlement",
  "trace",
  "trust",
  "workload",
] as const;

const FORBIDDEN_EVIDENCE_TEXT = [
  "password",
  "secret",
  "credential",
  "private_payload",
  "private-payload",
  "decrypted",
  "prompt",
  "key_material",
  "key-material",
] as const;

export function buildPhase7EvidenceWorkspace(input: Phase7EvidenceWorkspaceInput): Phase7EvidenceWorkspaceState {
  const policy = buildPolicyDecisionExplorer(input.policyResponse, input.timeline, input.attachments);
  const verification = buildVerificationEvidencePanel(
    input.verificationResponses,
    input.capabilities,
    input.timeline,
    input.attachments,
  );
  const disputes = buildDisputeCorrectionViews(input.disputeResponse, input.timeline, input.attachments);
  const incidents = buildIncidentReadinessViews(input.capabilities, input.incidentRefs, input.attachments);
  const combinedCheck = checkEvidenceLinkConsistency([
    ...collectPolicyLinks(policy.records),
    ...collectVerificationLinks(verification.records),
    ...collectDisputeLinks(disputes.records),
    ...collectIncidentLinks(incidents.records),
  ]);

  return {
    schemaVersion: "operator-policy-evidence.v0.1",
    policy,
    verification,
    disputes,
    incidents,
    readOnly: true,
    usesOvergateOnly: true,
    directLedgerMutation: false,
    linkCheck: combinedCheck,
  };
}

export function buildPolicyDecisionExplorer(
  response: AdminListResponse,
  timeline?: AdminTimelineResponse,
  attachments: Readonly<Record<string, readonly EvidenceAttachment[]>> = {},
): PolicyDecisionExplorerState {
  assertRoute(response.path, "/admin/policy-decisions");
  const records = response.items.map((item) => buildPolicyRecord(item, timeline, attachments));
  const missingCaseCoverage = REQUIRED_POLICY_DENIAL_CASES.filter(
    (requiredCase) => !records.some((record) => record.caseType === requiredCase),
  );
  const linkCheck = checkEvidenceLinkConsistency(collectPolicyLinks(records));
  return {
    kind: "policy",
    route: response.path,
    status: resolveEvidenceStatus(response.status, records.length, response.degraded_dependencies, linkCheck),
    records,
    requiredCaseCoverage: REQUIRED_POLICY_DENIAL_CASES,
    missingCaseCoverage,
    linkCheck,
  };
}

export function buildVerificationEvidencePanel(
  responses: readonly AdminListResponse[],
  capabilities: AdminCapabilitiesResponse,
  timeline?: AdminTimelineResponse,
  attachments: Readonly<Record<string, readonly EvidenceAttachment[]>> = {},
): VerificationEvidencePanelState {
  const records = responses.flatMap((response) =>
    response.items
      .filter((item) => isVerificationEvidenceSummary(item, attachments))
      .map((item) => buildVerificationRecord(item, timeline, attachments)),
  );
  const dependencyStatuses = collectDependencyStatuses(capabilities, ["overgate", "oververify", "overwatch"]);
  const missingStateCoverage = REQUIRED_VERIFICATION_STATES.filter(
    (requiredState) => !records.some((record) => record.state === requiredState),
  );
  const linkCheck = checkEvidenceLinkConsistency(collectVerificationLinks(records));
  return {
    kind: "verification",
    status: resolveEvidenceStatus("ok", records.length, dependencyStatuses, linkCheck),
    records,
    requiredStateCoverage: REQUIRED_VERIFICATION_STATES,
    missingStateCoverage,
    dependencyStatuses,
    linkCheck,
  };
}

export function buildDisputeCorrectionViews(
  response: AdminListResponse,
  timeline?: AdminTimelineResponse,
  attachments: Readonly<Record<string, readonly EvidenceAttachment[]>> = {},
): DisputeCorrectionViewState {
  assertRoute(response.path, "/admin/disputes");
  const records = response.items.map((item) => buildDisputeRecord(item, timeline, attachments));
  const linkCheck = checkEvidenceLinkConsistency(collectDisputeLinks(records));
  return {
    kind: "dispute",
    route: response.path,
    status: resolveEvidenceStatus(response.status, records.length, response.degraded_dependencies, linkCheck),
    records,
    linkCheck,
  };
}

export function buildIncidentReadinessViews(
  capabilities: AdminCapabilitiesResponse,
  incidentRefs: readonly OverridRef[] = ["incident:phase7_readiness" as OverridRef],
  attachments: Readonly<Record<string, readonly EvidenceAttachment[]>> = {},
): IncidentReadinessViewState {
  const dependencyStatuses = collectDependencyStatuses(capabilities, ["overgate", "overkey", "overguard", "overwatch"]);
  const records = incidentRefs.map((incidentRef) => {
    const links = buildEvidenceLinks(
      [
        { kind: "incident", ref: incidentRef, sourceService: "overwatch" },
        ...getAttachments(attachments, incidentRef),
      ],
      capabilities.audit_refs,
      "overwatch",
      capabilities.audit_refs[0]?.trace_id,
      ["admin.incident.break_glass_contract_missing" as ReasonCode],
    );
    return {
      incidentRef,
      state: resolveIncidentState(dependencyStatuses),
      policyState: resolvePolicyState(dependencyStatuses),
      dependencyStatuses,
      missingContracts: REQUIRED_INCIDENT_BREAK_GLASS_CONTRACTS,
      disabledActionReasonCodes: [
        "admin.incident.break_glass_contract_missing" as ReasonCode,
        ...dependencyStatuses.map((dependency) => dependency.reason_code),
      ],
      breakGlassExecutionDisabled: true,
      evidenceLinks: links,
    } satisfies IncidentReadinessRecord;
  });
  const linkCheck = checkEvidenceLinkConsistency(collectIncidentLinks(records));
  return {
    kind: "incident",
    status: dependencyStatuses.some((dependency) => dependency.status === "unavailable") ? "disabled" : "ready",
    records,
    breakGlassExecutionDisabled: true,
    dependencyStatuses,
    linkCheck,
  };
}

export function checkEvidenceLinkConsistency(links: readonly EvidenceLink[]): EvidenceLinkConsistencyResult {
  const missingStableRefIds = uniqueRefs(links.filter((link) => !link.stable).map((link) => link.ref));
  const privateRefIds = uniqueRefs(links.filter((link) => containsUnsafeEvidenceText(link.ref)).map((link) => link.ref));
  const mutableRefIds = uniqueRefs(links.filter((link) => isMutableEvidenceRef(link.ref)).map((link) => link.ref));
  const reasonCodes: ReasonCode[] = [];
  if (missingStableRefIds.length > 0) {
    reasonCodes.push("admin.evidence.stable_ref_missing" as ReasonCode);
  }
  if (privateRefIds.length > 0) {
    reasonCodes.push("admin.evidence.private_ref_rejected" as ReasonCode);
  }
  if (mutableRefIds.length > 0) {
    reasonCodes.push("admin.evidence.mutable_ref_rejected" as ReasonCode);
  }
  return {
    ok: reasonCodes.length === 0,
    missingStableRefIds,
    privateRefIds,
    mutableRefIds,
    reasonCodes,
  };
}

export function hasRequiredPolicyDenialCoverage(records: readonly PolicyDecisionEvidenceRecord[]): boolean {
  return REQUIRED_POLICY_DENIAL_CASES.every((requiredCase) =>
    records.some((record) => record.caseType === requiredCase),
  );
}

export function hasRequiredVerificationCoverage(records: readonly VerificationEvidenceRecord[]): boolean {
  return REQUIRED_VERIFICATION_STATES.every((requiredState) =>
    records.some((record) => record.state === requiredState),
  );
}

export function containsUnsafeEvidenceText(value: string): boolean {
  const lower = value.toLowerCase();
  return FORBIDDEN_EVIDENCE_TEXT.some((pattern) => lower.includes(pattern));
}

function buildPolicyRecord(
  item: EvidenceSummary,
  timeline: AdminTimelineResponse | undefined,
  attachments: Readonly<Record<string, readonly EvidenceAttachment[]>>,
): PolicyDecisionEvidenceRecord {
  const timelineRefs = collectTimelineRefs(timeline, item.trace_id, "overguard_decision");
  const localAttachments = getAttachments(attachments, item.id);
  const links = buildEvidenceLinks(
    [
      { kind: "policy", ref: item.id, sourceService: "overguard" },
      ...localAttachments,
      ...timelineRefs.map((ref) => ({ kind: inferEvidenceRefKind(ref), ref, sourceService: "overguard" })),
    ],
    item.audit_refs,
    "overguard",
    item.trace_id,
    item.reason_codes,
  );

  return {
    decisionId: item.id,
    tenantId: item.tenant_id,
    traceId: item.trace_id,
    state: item.state,
    caseType: classifyPolicyDecisionCase(item, localAttachments, timelineRefs),
    policyVersionRefs: refsByKind(links, "policy"),
    matchedRuleRefs: refsByKind(links, "matched_rule"),
    inputFactRefs: refsByKind(links, "input_fact"),
    expectedPlacementClass: findSignal(item.reason_codes, "placement"),
    deniedClass: item.state.toLowerCase().includes("denied") ? findSignal(item.reason_codes, "denied") : undefined,
    allowedClass: item.state.toLowerCase().includes("allowed") ? findSignal(item.reason_codes, "allowed") : undefined,
    correctionOptions: item.reason_codes.filter((reason) => reason.toLowerCase().includes("correction")),
    reasonCodes: item.reason_codes,
    evidenceLinks: links,
  };
}

function buildVerificationRecord(
  item: EvidenceSummary,
  timeline: AdminTimelineResponse | undefined,
  attachments: Readonly<Record<string, readonly EvidenceAttachment[]>>,
): VerificationEvidenceRecord {
  const localAttachments = getAttachments(attachments, item.id);
  const timelineRefs = collectTimelineRefs(timeline, item.trace_id, "overguard_decision");
  const links = buildEvidenceLinks(
    [
      { kind: "provider_verification", ref: item.id, sourceService: "oververify" },
      ...localAttachments,
      ...timelineRefs.map((ref) => ({ kind: inferEvidenceRefKind(ref), ref, sourceService: "oververify" })),
    ],
    item.audit_refs,
    "oververify",
    item.trace_id,
    item.reason_codes,
  );

  return {
    subjectRef: item.id,
    subjectKind: resolveVerificationSubjectKind(item, localAttachments),
    tenantId: item.tenant_id,
    traceId: item.trace_id,
    state: classifyVerificationState(item, localAttachments),
    providerVerificationRefs: refsByKind(links, "provider_verification"),
    challengeRefs: refsByKind(links, "challenge"),
    benchmarkRefs: refsByKind(links, "benchmark"),
    trustClassRefs: refsByKind(links, "trust_class"),
    stale: item.state.toLowerCase().includes("stale") || item.reason_codes.some((reason) => reason.includes("stale")),
    redacted: item.redaction.redacted_fields.length > 0,
    reasonCodes: item.reason_codes,
    evidenceLinks: links,
  };
}

function buildDisputeRecord(
  item: EvidenceSummary,
  timeline: AdminTimelineResponse | undefined,
  attachments: Readonly<Record<string, readonly EvidenceAttachment[]>>,
): DisputeCorrectionRecord {
  const localAttachments = getAttachments(attachments, item.id);
  const timelineRefs = collectTimelineRefs(timeline, item.trace_id, "overclaim_dispute");
  const links = buildEvidenceLinks(
    [
      { kind: "dispute", ref: item.id, sourceService: "overclaim" },
      ...localAttachments,
      ...timelineRefs.map((ref) => ({ kind: inferEvidenceRefKind(ref), ref, sourceService: "overclaim" })),
    ],
    item.audit_refs,
    "overclaim",
    item.trace_id,
    item.reason_codes,
  );
  const haystack = createSignalText(item, localAttachments, timelineRefs);

  return {
    caseRef: item.id,
    tenantId: item.tenant_id,
    traceId: item.trace_id,
    state: classifyDisputeState(item, localAttachments),
    evidenceRefs: refsByKind(links, "owning_service"),
    holdSettlementVisibility: haystack.includes("hold") || haystack.includes("settlement_visibility"),
    challengeWindowOpen: haystack.includes("challenge_window"),
    correctionOutcomeRefs: refsByKind(links, "correction"),
    refundRefs: refsByKind(links, "refund"),
    settlementImpactRefs: refsByKind(links, "settlement"),
    directLedgerMutation: false,
    reasonCodes: item.reason_codes,
    evidenceLinks: links,
  };
}

function buildEvidenceLinks(
  attachments: readonly EvidenceAttachment[],
  auditRefs: readonly TimelineAuditRef[],
  fallbackService: string,
  traceId: TraceId | undefined,
  reasonCodes: readonly ReasonCode[],
): readonly EvidenceLink[] {
  const links = attachments.map((attachment) => ({
    kind: attachment.kind,
    ref: attachment.ref,
    sourceService: attachment.sourceService ?? fallbackService,
    traceId: attachment.traceId ?? traceId,
    reasonCodes: attachment.reasonCodes ?? reasonCodes,
    auditRefs: auditRefs.map(formatAuditRef),
    stable: isStableEvidenceRef(attachment.ref),
  }));
  const auditLinks = auditRefs.map((auditRef) => ({
    kind: "audit" as const,
    ref: auditRef.audit_id,
    sourceService: auditRef.source_service,
    traceId: auditRef.trace_id,
    reasonCodes,
    auditRefs: [formatAuditRef(auditRef)],
    stable: isStableEvidenceRef(auditRef.audit_id),
  }));
  return dedupeLinks([...links, ...auditLinks]);
}

function classifyPolicyDecisionCase(
  item: EvidenceSummary,
  attachments: readonly EvidenceAttachment[],
  timelineRefs: readonly OverridRef[],
): PolicyDecisionCase {
  const haystack = createSignalText(item, attachments, timelineRefs);
  if (haystack.includes("egress")) return "denied_egress";
  if (haystack.includes("insufficient_trust") || haystack.includes("trust")) return "insufficient_trust";
  if (haystack.includes("quota")) return "quota_exhaustion";
  if (haystack.includes("package_trust") || haystack.includes("package")) return "package_trust_failure";
  if (haystack.includes("wrong_tenant") || haystack.includes("cross_tenant")) return "wrong_tenant";
  if (haystack.includes("budget") || haystack.includes("precheck")) return "budget_precheck_failure";
  return "allowed";
}

function classifyVerificationState(
  item: EvidenceSummary,
  attachments: readonly EvidenceAttachment[],
): VerificationEvidenceState {
  const haystack = createSignalText(item, attachments, []);
  if (haystack.includes("challenged") || haystack.includes("challenge")) return "challenged";
  if (haystack.includes("expired")) return "expired";
  if (haystack.includes("disputed")) return "disputed";
  if (haystack.includes("untrusted") || haystack.includes("denied")) return "untrusted";
  if (haystack.includes("degraded") || haystack.includes("stale")) return "degraded";
  return "verified";
}

function classifyDisputeState(
  item: EvidenceSummary,
  attachments: readonly EvidenceAttachment[],
): DisputeCorrectionState {
  const haystack = createSignalText(item, attachments, []);
  if (haystack.includes("challenge_window")) return "challenge_window";
  if (haystack.includes("refund")) return "refunded";
  if (haystack.includes("correction") || haystack.includes("corrected")) return "corrected";
  if (haystack.includes("settled")) return "settled";
  if (haystack.includes("closed")) return "closed";
  if (haystack.includes("hold")) return "hold";
  return "open";
}

function isVerificationEvidenceSummary(
  item: EvidenceSummary,
  attachments: Readonly<Record<string, readonly EvidenceAttachment[]>>,
): boolean {
  const localAttachments = getAttachments(attachments, item.id);
  return (
    isVerificationResourceKind(item.kind) ||
    isProviderEvidenceRef(item.id) ||
    localAttachments.some(
      (attachment) => attachment.kind === "provider_verification" || isProviderEvidenceRef(attachment.ref),
    )
  );
}

function isVerificationResourceKind(
  kind: EvidenceSummary["kind"],
): kind is Extract<EvidenceSummary["kind"], "node" | "package" | "workload"> {
  if (kind === "node" || kind === "package" || kind === "workload") {
    return true;
  }
  return false;
}

function resolveVerificationSubjectKind(
  item: EvidenceSummary,
  attachments: readonly EvidenceAttachment[],
): VerificationEvidenceRecord["subjectKind"] {
  if (
    isProviderEvidenceRef(item.id) ||
    attachments.some(
      (attachment) => attachment.kind === "provider_verification" || isProviderEvidenceRef(attachment.ref),
    )
  ) {
    return "provider";
  }
  if (isVerificationResourceKind(item.kind)) {
    return item.kind;
  }
  return "provider";
}

function resolveEvidenceStatus(
  status: AdminListResponse["status"],
  recordCount: number,
  dependencyStatuses: readonly AdminDependencyStatus[],
  linkCheck: EvidenceLinkConsistencyResult,
): EvidenceViewStatus {
  if (!linkCheck.ok) return "disabled";
  if (dependencyStatuses.some((dependency) => dependency.status === "unavailable")) return "disabled";
  if (dependencyStatuses.some((dependency) => dependency.status === "redacted")) return "redacted";
  if (status === "empty" || recordCount === 0) return "empty";
  if (status === "degraded" || dependencyStatuses.some((dependency) => dependency.status === "degraded")) {
    return "degraded";
  }
  return "ready";
}

function resolveIncidentState(dependencies: readonly AdminDependencyStatus[]): IncidentReadinessState {
  if (dependencies.some((dependency) => dependency.status === "unavailable")) return "unavailable";
  if (dependencies.some((dependency) => dependency.status === "degraded")) return "degraded";
  return "disabled";
}

function resolvePolicyState(dependencies: readonly AdminDependencyStatus[]): IncidentReadinessRecord["policyState"] {
  const overguard = dependencies.find((dependency) => dependency.service.toLowerCase() === "overguard");
  if (!overguard || overguard.status === "unavailable") return "missing";
  if (overguard.status === "degraded" || overguard.status === "redacted") return "degraded";
  return "available";
}

function collectDependencyStatuses(
  capabilities: AdminCapabilitiesResponse,
  services: readonly string[],
): readonly AdminDependencyStatus[] {
  const disabledStatuses = capabilities.disabled_panels.map((panel) => ({
    ...panel.dependency_status,
    service: panel.dependency_status.service.toLowerCase(),
  }));
  return services.map((service) => {
    const status = disabledStatuses.find((candidate) => candidate.service === service);
    return (
      status ?? {
        service,
        status: "available",
        reason_code: "admin.dependency.available" as ReasonCode,
      }
    );
  });
}

function collectTimelineRefs(
  timeline: AdminTimelineResponse | undefined,
  traceId: TraceId,
  kind: TimelineNode["kind"],
): readonly OverridRef[] {
  if (!timeline) return [];
  return timeline.nodes
    .filter((node) => node.trace_id === traceId && node.kind === kind)
    .flatMap((node) => node.refs);
}

function getAttachments(
  attachments: Readonly<Record<string, readonly EvidenceAttachment[]>>,
  ref: OverridRef,
): readonly EvidenceAttachment[] {
  return attachments[ref] ?? [];
}

function createSignalText(
  item: EvidenceSummary,
  attachments: readonly EvidenceAttachment[],
  timelineRefs: readonly OverridRef[],
): string {
  return [
    item.id,
    item.state,
    item.kind,
    ...item.reason_codes,
    ...attachments.map((attachment) => attachment.ref),
    ...attachments.flatMap((attachment) => attachment.reasonCodes ?? []),
    ...timelineRefs,
  ]
    .join(" ")
    .toLowerCase();
}

function refsByKind(links: readonly EvidenceLink[], kind: EvidenceRefKind): readonly OverridRef[] {
  return uniqueRefs(links.filter((link) => link.kind === kind).map((link) => link.ref));
}

function findSignal(reasonCodes: readonly ReasonCode[], token: string): string | undefined {
  return reasonCodes.find((reasonCode) => reasonCode.toLowerCase().includes(token));
}

function inferEvidenceRefKind(ref: OverridRef): EvidenceRefKind {
  const prefix = ref.split(":", 1)[0];
  switch (prefix) {
    case "audit":
      return "audit";
    case "policy":
      return "policy";
    case "rule":
      return "matched_rule";
    case "fact":
      return "input_fact";
    case "placement":
      return "placement";
    case "provider":
      return "provider_verification";
    case "challenge":
      return "challenge";
    case "benchmark":
      return "benchmark";
    case "trust":
      return "trust_class";
    case "dispute":
    case "overclaim":
      return "dispute";
    case "correction":
      return "correction";
    case "refund":
      return "refund";
    case "settlement":
      return "settlement";
    case "incident":
      return "incident";
    case "trace":
      return "trace";
    default:
      return "owning_service";
  }
}

function isProviderEvidenceRef(ref: OverridRef): boolean {
  return ref.split(":", 1)[0] === "provider";
}

function isStableEvidenceRef(ref: OverridRef): boolean {
  const [prefix, value] = ref.split(":", 2);
  return Boolean(
    prefix &&
      value &&
      STABLE_EVIDENCE_PREFIXES.includes(prefix as (typeof STABLE_EVIDENCE_PREFIXES)[number]) &&
      !containsUnsafeEvidenceText(ref) &&
      !isMutableEvidenceRef(ref),
  );
}

function isMutableEvidenceRef(ref: OverridRef): boolean {
  const lower = ref.toLowerCase();
  return lower.includes("draft") || lower.includes("mutable") || lower.includes("temporary") || lower.includes("tmp");
}

function collectPolicyLinks(records: readonly PolicyDecisionEvidenceRecord[]): readonly EvidenceLink[] {
  return records.flatMap((record) => record.evidenceLinks);
}

function collectVerificationLinks(records: readonly VerificationEvidenceRecord[]): readonly EvidenceLink[] {
  return records.flatMap((record) => record.evidenceLinks);
}

function collectDisputeLinks(records: readonly DisputeCorrectionRecord[]): readonly EvidenceLink[] {
  return records.flatMap((record) => record.evidenceLinks);
}

function collectIncidentLinks(records: readonly IncidentReadinessRecord[]): readonly EvidenceLink[] {
  return records.flatMap((record) => record.evidenceLinks);
}

function dedupeLinks(links: readonly EvidenceLink[]): readonly EvidenceLink[] {
  const seen = new Set<string>();
  return links.filter((link) => {
    const key = `${link.kind}:${link.ref}:${link.sourceService}`;
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

function uniqueRefs(refs: readonly OverridRef[]): readonly OverridRef[] {
  return Array.from(new Set(refs));
}

function formatAuditRef(ref: TimelineAuditRef): string {
  return `${ref.source_service}:${ref.audit_id}:${ref.trace_id}`;
}

function assertRoute(actual: AdminRoutePath, expected: AdminRoutePath): void {
  if (actual !== expected) {
    throw new Error(`Phase 7 evidence view expected ${expected}, received ${actual}`);
  }
}
