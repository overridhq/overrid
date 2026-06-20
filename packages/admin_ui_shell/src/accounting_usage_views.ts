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

export type UsageResourceDimension = "CPU-ORU" | "GPU-ORU" | "STOR-ORU" | "NET-ORU" | "MEM-ORU" | "DATA-ORU";
export type UsageRollupGrouping =
  | "tenant"
  | "actor"
  | "workload"
  | "app"
  | "provider"
  | "resource_class"
  | "time_window"
  | "trace_id";
export type AccountingViewStatus = "ready" | "degraded" | "disabled" | "empty" | "redacted";
export type AccountingRefKind =
  | "trace"
  | "audit"
  | "usage_rollup"
  | "observed_usage"
  | "settled_accounting"
  | "ledger"
  | "receipt"
  | "invoice"
  | "payment_provider"
  | "refund"
  | "correction"
  | "payout_hold"
  | "grant"
  | "purpose_scope"
  | "resource_right"
  | "storage_binding"
  | "namespace_binding"
  | "route_binding"
  | "entitlement"
  | "owning_service";
export type LedgerBalanceState =
  | "available"
  | "reserved"
  | "held"
  | "spent"
  | "earned"
  | "sponsored"
  | "refunded_corrected"
  | "expired_revoked"
  | "disputed";
export type BillingDocumentState =
  | "receipt_recorded"
  | "invoice_open"
  | "invoice_settled"
  | "payment_provider_ref_visible"
  | "refund_visible"
  | "correction_visible"
  | "payout_hold_visible";
export type GrantRightsState = "active" | "sponsored" | "purpose_scoped" | "expired" | "corrected" | "revoked";
export type AccountingRole = AdminSessionContext["role_bindings"][number]["role"];
export type AccountingPanelKind = "usage" | "ledger" | "billing" | "grants" | "rights" | "access";

type AccountingSummary = AdminListResponse["items"][number];
type AccountingAuditRef = AdminListResponse["audit_refs"][number];

export interface AccountingAttachment {
  readonly kind: AccountingRefKind | UsageRollupGrouping;
  readonly ref: OverridRef;
  readonly sourceService?: string;
  readonly traceId?: TraceId;
  readonly reasonCodes?: readonly ReasonCode[];
  readonly dimension?: UsageResourceDimension;
  readonly ledgerState?: LedgerBalanceState;
  readonly billingState?: BillingDocumentState;
  readonly grantState?: GrantRightsState;
}

export interface AccountingEvidenceLink {
  readonly kind: AccountingRefKind;
  readonly ref: OverridRef;
  readonly sourceService: string;
  readonly traceId?: TraceId;
  readonly reasonCodes: readonly ReasonCode[];
  readonly auditRefs: readonly string[];
  readonly stable: boolean;
}

export interface AccountingRefConsistencyResult {
  readonly ok: boolean;
  readonly missingStableRefIds: readonly OverridRef[];
  readonly privateRefIds: readonly OverridRef[];
  readonly mutableRefIds: readonly OverridRef[];
  readonly assumptionRefIds: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
}

export interface UsageRollupRecord {
  readonly rollupRef: OverridRef;
  readonly tenantId: OverridRef;
  readonly traceId: TraceId;
  readonly dimension: UsageResourceDimension;
  readonly groupings: readonly UsageRollupGrouping[];
  readonly observedUsageRefs: readonly OverridRef[];
  readonly settledAccountingRefs: readonly OverridRef[];
  readonly settlementState: "observed_only" | "settled" | "held" | "disputed";
  readonly timelineRefs: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
  readonly evidenceLinks: readonly AccountingEvidenceLink[];
}

export interface LedgerReadRecord {
  readonly accountRef: OverridRef;
  readonly tenantId: OverridRef;
  readonly traceId: TraceId;
  readonly state: LedgerBalanceState;
  readonly ledgerRefs: readonly OverridRef[];
  readonly immutableLedgerRefs: readonly OverridRef[];
  readonly editableByUi: false;
  readonly reasonCodes: readonly ReasonCode[];
  readonly evidenceLinks: readonly AccountingEvidenceLink[];
}

export interface BillingDocumentRecord {
  readonly documentRef: OverridRef;
  readonly tenantId: OverridRef;
  readonly traceId: TraceId;
  readonly state: BillingDocumentState;
  readonly receiptRefs: readonly OverridRef[];
  readonly invoiceRefs: readonly OverridRef[];
  readonly paymentProviderRefs: readonly OverridRef[];
  readonly refundRefs: readonly OverridRef[];
  readonly correctionRefs: readonly OverridRef[];
  readonly payoutHoldRefs: readonly OverridRef[];
  readonly encodesPricingAssumption: false;
  readonly reasonCodes: readonly ReasonCode[];
  readonly evidenceLinks: readonly AccountingEvidenceLink[];
}

export interface GrantVisibilityRecord {
  readonly grantRef: OverridRef;
  readonly tenantId: OverridRef;
  readonly traceId: TraceId;
  readonly state: GrantRightsState;
  readonly grantScopeRefs: readonly OverridRef[];
  readonly sponsoredAllocationRefs: readonly OverridRef[];
  readonly purposeScopeRefs: readonly OverridRef[];
  readonly expirationRefs: readonly OverridRef[];
  readonly correctionRefs: readonly OverridRef[];
  readonly readOnly: true;
  readonly reasonCodes: readonly ReasonCode[];
  readonly evidenceLinks: readonly AccountingEvidenceLink[];
}

export interface RightsVisibilityRecord {
  readonly assetRef: OverridRef;
  readonly tenantId: OverridRef;
  readonly traceId: TraceId;
  readonly state: GrantRightsState;
  readonly resourceRightsRefs: readonly OverridRef[];
  readonly storageBindingRefs: readonly OverridRef[];
  readonly namespaceBindingRefs: readonly OverridRef[];
  readonly routeBindingRefs: readonly OverridRef[];
  readonly entitlementRefs: readonly OverridRef[];
  readonly expirationRefs: readonly OverridRef[];
  readonly correctionRefs: readonly OverridRef[];
  readonly blockchainOwnershipModel: false;
  readonly nftOwnershipModel: false;
  readonly reasonCodes: readonly ReasonCode[];
  readonly evidenceLinks: readonly AccountingEvidenceLink[];
}

export interface AccountingAccessRule {
  readonly role: AccountingRole;
  readonly allowedPanels: Readonly<Record<AccountingPanelKind, boolean>>;
  readonly crossTenantAccess: boolean;
  readonly redactedFields: readonly string[];
  readonly reasonCodes: readonly ReasonCode[];
}

export interface UsageRollupPanelState {
  readonly kind: "usage";
  readonly route: AdminRoutePath;
  readonly status: AccountingViewStatus;
  readonly records: readonly UsageRollupRecord[];
  readonly requiredDimensions: readonly UsageResourceDimension[];
  readonly missingDimensions: readonly UsageResourceDimension[];
  readonly requiredGroupings: readonly UsageRollupGrouping[];
  readonly missingGroupings: readonly UsageRollupGrouping[];
  readonly linkCheck: AccountingRefConsistencyResult;
}

export interface LedgerReadPanelState {
  readonly kind: "ledger";
  readonly route: AdminRoutePath;
  readonly status: AccountingViewStatus;
  readonly records: readonly LedgerReadRecord[];
  readonly requiredStates: readonly LedgerBalanceState[];
  readonly missingStates: readonly LedgerBalanceState[];
  readonly directLedgerMutation: false;
  readonly linkCheck: AccountingRefConsistencyResult;
}

export interface BillingPanelState {
  readonly kind: "billing";
  readonly route: AdminRoutePath;
  readonly status: AccountingViewStatus;
  readonly records: readonly BillingDocumentRecord[];
  readonly noPricingAssumptions: true;
  readonly linkCheck: AccountingRefConsistencyResult;
}

export interface GrantRightsPanelState {
  readonly kind: "grants" | "rights";
  readonly route: AdminRoutePath;
  readonly status: AccountingViewStatus;
  readonly grantRecords: readonly GrantVisibilityRecord[];
  readonly rightsRecords: readonly RightsVisibilityRecord[];
  readonly blockchainOwnershipModel: false;
  readonly nftOwnershipModel: false;
  readonly linkCheck: AccountingRefConsistencyResult;
}

export interface AccountingAccessMatrixState {
  readonly kind: "access";
  readonly rules: readonly AccountingAccessRule[];
  readonly requiredRoles: readonly AccountingRole[];
  readonly missingRoles: readonly AccountingRole[];
  readonly dependencyStatuses: readonly AdminDependencyStatus[];
}

export interface Phase8AccountingWorkspaceState {
  readonly schemaVersion: "operator-accounting-usage.v0.1";
  readonly usage: UsageRollupPanelState;
  readonly ledger: LedgerReadPanelState;
  readonly billing: BillingPanelState;
  readonly grantsAndRights: GrantRightsPanelState;
  readonly access: AccountingAccessMatrixState;
  readonly readOnly: true;
  readonly usesOvergateOnly: true;
  readonly directStorageAccess: false;
  readonly directLedgerMutation: false;
  readonly noPricingAssumptions: true;
  readonly blockchainOwnershipModel: false;
  readonly nftOwnershipModel: false;
  readonly linkCheck: AccountingRefConsistencyResult;
}

export interface Phase8AccountingWorkspaceInput {
  readonly usageResponse: AdminListResponse;
  readonly ledgerResponse: AdminListResponse;
  readonly receiptResponse: AdminListResponse;
  readonly grantResponse: AdminListResponse;
  readonly rightsResponse: AdminListResponse;
  readonly capabilities: AdminCapabilitiesResponse;
  readonly context?: AdminSessionContext;
  readonly attachments?: Readonly<Record<string, readonly AccountingAttachment[]>>;
}

export const PHASE8_ACCOUNTING_ROUTES: readonly AdminRoutePath[] = [
  "/admin/usage",
  "/admin/ledger",
  "/admin/receipts",
  "/admin/capabilities",
];

export const REQUIRED_USAGE_DIMENSIONS: readonly UsageResourceDimension[] = [
  "CPU-ORU",
  "GPU-ORU",
  "STOR-ORU",
  "NET-ORU",
  "MEM-ORU",
  "DATA-ORU",
];

export const REQUIRED_USAGE_GROUPINGS: readonly UsageRollupGrouping[] = [
  "tenant",
  "actor",
  "workload",
  "app",
  "provider",
  "resource_class",
  "time_window",
  "trace_id",
];

export const REQUIRED_LEDGER_STATES: readonly LedgerBalanceState[] = [
  "available",
  "reserved",
  "held",
  "spent",
  "earned",
  "sponsored",
  "refunded_corrected",
  "expired_revoked",
  "disputed",
];

export const REQUIRED_ACCOUNTING_ROLES: readonly AccountingRole[] = [
  "platform_owner",
  "tenant_owner",
  "tenant_admin",
  "support_viewer",
  "product_integrator",
  "incident_responder",
  "accounting_viewer",
];

export const REQUIRED_ACCOUNTING_DEPENDENCIES = [
  "overgate",
  "overmeter",
  "oru_account_service",
  "seal_ledger",
  "overbill",
  "overgrant",
  "overasset",
  "overwatch",
] as const;

const STABLE_ACCOUNTING_PREFIXES = [
  "actor",
  "app",
  "asset",
  "audit",
  "correction",
  "entitlement",
  "grant",
  "invoice",
  "ledger",
  "namespace",
  "oru",
  "overasset",
  "overbill",
  "overgrant",
  "overmeter",
  "payment_provider",
  "payout_hold",
  "provider",
  "purpose",
  "receipt",
  "refund",
  "resource",
  "rights",
  "route",
  "seal_ledger",
  "settlement",
  "storage",
  "tenant",
  "time_window",
  "trace",
  "usage",
  "workload",
] as const;

const FORBIDDEN_ACCOUNTING_TEXT = [
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

const FORBIDDEN_ACCOUNTING_ASSUMPTIONS = [
  "price_per",
  "price:",
  "pricing_model",
  "customer_count",
  "market_volume",
  "revenue_projection",
  "blockchain",
  "nft",
  "tokenized_ownership",
] as const;

export function buildPhase8AccountingWorkspace(
  input: Phase8AccountingWorkspaceInput,
): Phase8AccountingWorkspaceState {
  const usage = buildUsageRollupPanel(input.usageResponse, input.attachments);
  const ledger = buildLedgerReadViews(input.ledgerResponse, input.attachments);
  const billing = buildReceiptInvoiceViews(input.receiptResponse, input.attachments);
  const grants = buildGrantVisibility(input.grantResponse, input.attachments);
  const rights = buildRightsVisibility(input.rightsResponse, input.attachments);
  const grantsAndRights = buildGrantRightsPanel(grants, rights, input.rightsResponse);
  const access = buildAccountingAccessMatrix(input.capabilities, input.context);
  const linkCheck = checkAccountingRefConsistency([
    ...collectUsageLinks(usage.records),
    ...collectLedgerLinks(ledger.records),
    ...collectBillingLinks(billing.records),
    ...collectGrantLinks(grantsAndRights.grantRecords),
    ...collectRightsLinks(grantsAndRights.rightsRecords),
  ]);

  return {
    schemaVersion: "operator-accounting-usage.v0.1",
    usage,
    ledger,
    billing,
    grantsAndRights,
    access,
    readOnly: true,
    usesOvergateOnly: true,
    directStorageAccess: false,
    directLedgerMutation: false,
    noPricingAssumptions: true,
    blockchainOwnershipModel: false,
    nftOwnershipModel: false,
    linkCheck,
  };
}

export function buildUsageRollupPanel(
  response: AdminListResponse,
  attachments: Readonly<Record<string, readonly AccountingAttachment[]>> = {},
): UsageRollupPanelState {
  assertRoute(response.path, "/admin/usage");
  const records = response.items.map((item) => buildUsageRollupRecord(item, attachments));
  const missingDimensions = REQUIRED_USAGE_DIMENSIONS.filter(
    (dimension) => !records.some((record) => record.dimension === dimension),
  );
  const coveredGroupings = new Set(records.flatMap((record) => record.groupings));
  const missingGroupings = REQUIRED_USAGE_GROUPINGS.filter((grouping) => !coveredGroupings.has(grouping));
  const linkCheck = checkAccountingRefConsistency(collectUsageLinks(records));
  return {
    kind: "usage",
    route: response.path,
    status: resolveAccountingStatus(response, records.length, response.degraded_dependencies, linkCheck),
    records,
    requiredDimensions: REQUIRED_USAGE_DIMENSIONS,
    missingDimensions,
    requiredGroupings: REQUIRED_USAGE_GROUPINGS,
    missingGroupings,
    linkCheck,
  };
}

export function buildLedgerReadViews(
  response: AdminListResponse,
  attachments: Readonly<Record<string, readonly AccountingAttachment[]>> = {},
): LedgerReadPanelState {
  assertRoute(response.path, "/admin/ledger");
  const records = response.items.map((item) => buildLedgerRecord(item, attachments));
  const missingStates = REQUIRED_LEDGER_STATES.filter(
    (state) => !records.some((record) => record.state === state),
  );
  const linkCheck = checkAccountingRefConsistency(collectLedgerLinks(records));
  return {
    kind: "ledger",
    route: response.path,
    status: resolveAccountingStatus(response, records.length, response.degraded_dependencies, linkCheck),
    records,
    requiredStates: REQUIRED_LEDGER_STATES,
    missingStates,
    directLedgerMutation: false,
    linkCheck,
  };
}

export function buildReceiptInvoiceViews(
  response: AdminListResponse,
  attachments: Readonly<Record<string, readonly AccountingAttachment[]>> = {},
): BillingPanelState {
  assertRoute(response.path, "/admin/receipts");
  const records = response.items.map((item) => buildBillingRecord(item, attachments));
  const linkCheck = checkAccountingRefConsistency(collectBillingLinks(records));
  return {
    kind: "billing",
    route: response.path,
    status: resolveAccountingStatus(response, records.length, response.degraded_dependencies, linkCheck),
    records,
    noPricingAssumptions: true,
    linkCheck,
  };
}

export function buildGrantVisibility(
  response: AdminListResponse,
  attachments: Readonly<Record<string, readonly AccountingAttachment[]>> = {},
): readonly GrantVisibilityRecord[] {
  assertRoute(response.path, "/admin/capabilities");
  return response.items.map((item) => buildGrantRecord(item, attachments));
}

export function buildRightsVisibility(
  response: AdminListResponse,
  attachments: Readonly<Record<string, readonly AccountingAttachment[]>> = {},
): readonly RightsVisibilityRecord[] {
  assertRoute(response.path, "/admin/capabilities");
  return response.items.map((item) => buildRightsRecord(item, attachments));
}

export function buildGrantRightsPanel(
  grantRecords: readonly GrantVisibilityRecord[],
  rightsRecords: readonly RightsVisibilityRecord[],
  response: AdminListResponse,
): GrantRightsPanelState {
  const links = [...collectGrantLinks(grantRecords), ...collectRightsLinks(rightsRecords)];
  const linkCheck = checkAccountingRefConsistency(links);
  return {
    kind: "rights",
    route: response.path,
    status: resolveAccountingStatus(response, grantRecords.length + rightsRecords.length, response.degraded_dependencies, linkCheck),
    grantRecords,
    rightsRecords,
    blockchainOwnershipModel: false,
    nftOwnershipModel: false,
    linkCheck,
  };
}

export function buildAccountingAccessMatrix(
  capabilities: AdminCapabilitiesResponse,
  context?: AdminSessionContext,
): AccountingAccessMatrixState {
  const dependencyStatuses = collectAccountingDependencyStatuses(capabilities);
  const activeRoles = new Set<AccountingRole>(context?.role_bindings.map((binding) => binding.role) ?? []);
  const roles = new Set<AccountingRole>([...REQUIRED_ACCOUNTING_ROLES, ...activeRoles]);
  const rules = [...roles].map((role) => buildAccountingAccessRule(role));
  const missingRoles = REQUIRED_ACCOUNTING_ROLES.filter((role) => !rules.some((rule) => rule.role === role));
  return {
    kind: "access",
    rules,
    requiredRoles: REQUIRED_ACCOUNTING_ROLES,
    missingRoles,
    dependencyStatuses,
  };
}

export function checkAccountingRefConsistency(
  links: readonly AccountingEvidenceLink[],
): AccountingRefConsistencyResult {
  const missingStableRefIds = uniqueRefs(links.filter((link) => !link.stable).map((link) => link.ref));
  const privateRefIds = uniqueRefs(links.filter((link) => containsUnsafeAccountingText(link.ref)).map((link) => link.ref));
  const mutableRefIds = uniqueRefs(links.filter((link) => isMutableAccountingRef(link.ref)).map((link) => link.ref));
  const assumptionRefIds = uniqueRefs(
    links.filter((link) => containsForbiddenAccountingAssumption(link.ref)).map((link) => link.ref),
  );
  const reasonCodes: ReasonCode[] = [];
  if (missingStableRefIds.length > 0) {
    reasonCodes.push("admin.accounting.stable_ref_missing" as ReasonCode);
  }
  if (privateRefIds.length > 0) {
    reasonCodes.push("admin.accounting.private_ref_rejected" as ReasonCode);
  }
  if (mutableRefIds.length > 0) {
    reasonCodes.push("admin.accounting.mutable_ref_rejected" as ReasonCode);
  }
  if (assumptionRefIds.length > 0) {
    reasonCodes.push("admin.accounting.assumption_ref_rejected" as ReasonCode);
  }
  return {
    ok: reasonCodes.length === 0,
    missingStableRefIds,
    privateRefIds,
    mutableRefIds,
    assumptionRefIds,
    reasonCodes,
  };
}

export function hasRequiredUsageDimensionCoverage(records: readonly UsageRollupRecord[]): boolean {
  return REQUIRED_USAGE_DIMENSIONS.every((dimension) => records.some((record) => record.dimension === dimension));
}

export function hasRequiredLedgerStateCoverage(records: readonly LedgerReadRecord[]): boolean {
  return REQUIRED_LEDGER_STATES.every((state) => records.some((record) => record.state === state));
}

export function containsUnsafeAccountingText(value: string): boolean {
  const lower = value.toLowerCase();
  return FORBIDDEN_ACCOUNTING_TEXT.some((pattern) => lower.includes(pattern));
}

export function containsForbiddenAccountingAssumption(value: string): boolean {
  const lower = value.toLowerCase();
  return FORBIDDEN_ACCOUNTING_ASSUMPTIONS.some((pattern) => lower.includes(pattern));
}

function buildUsageRollupRecord(
  item: AccountingSummary,
  attachments: Readonly<Record<string, readonly AccountingAttachment[]>>,
): UsageRollupRecord {
  const localAttachments = getAttachments(attachments, item.id);
  const links = buildAccountingLinks(
    [
      { kind: "usage_rollup", ref: item.id, sourceService: "overmeter" },
      ...localAttachments,
    ],
    item.audit_refs,
    "overmeter",
    item.trace_id,
    item.reason_codes,
  );
  return {
    rollupRef: item.id,
    tenantId: item.tenant_id,
    traceId: item.trace_id,
    dimension: resolveUsageDimension(item, localAttachments),
    groupings: resolveUsageGroupings(item, localAttachments),
    observedUsageRefs: refsByKind(links, "observed_usage"),
    settledAccountingRefs: refsByKind(links, "settled_accounting"),
    settlementState: resolveUsageSettlementState(item, links),
    timelineRefs: refsByKind(links, "trace"),
    reasonCodes: item.reason_codes,
    evidenceLinks: links,
  };
}

function buildLedgerRecord(
  item: AccountingSummary,
  attachments: Readonly<Record<string, readonly AccountingAttachment[]>>,
): LedgerReadRecord {
  const localAttachments = getAttachments(attachments, item.id);
  const links = buildAccountingLinks(
    [{ kind: "ledger", ref: item.id, sourceService: "seal_ledger" }, ...localAttachments],
    item.audit_refs,
    "seal_ledger",
    item.trace_id,
    item.reason_codes,
  );
  const ledgerRefs = refsByKind(links, "ledger");
  return {
    accountRef: item.id,
    tenantId: item.tenant_id,
    traceId: item.trace_id,
    state: resolveLedgerState(item, localAttachments),
    ledgerRefs,
    immutableLedgerRefs: ledgerRefs.filter((ref) => isStableAccountingRef(ref) && !isMutableAccountingRef(ref)),
    editableByUi: false,
    reasonCodes: item.reason_codes,
    evidenceLinks: links,
  };
}

function buildBillingRecord(
  item: AccountingSummary,
  attachments: Readonly<Record<string, readonly AccountingAttachment[]>>,
): BillingDocumentRecord {
  const localAttachments = getAttachments(attachments, item.id);
  const links = buildAccountingLinks(
    [{ kind: "receipt", ref: item.id, sourceService: "overbill" }, ...localAttachments],
    item.audit_refs,
    "overbill",
    item.trace_id,
    item.reason_codes,
  );
  return {
    documentRef: item.id,
    tenantId: item.tenant_id,
    traceId: item.trace_id,
    state: resolveBillingState(item, localAttachments),
    receiptRefs: refsByKind(links, "receipt"),
    invoiceRefs: refsByKind(links, "invoice"),
    paymentProviderRefs: refsByKind(links, "payment_provider"),
    refundRefs: refsByKind(links, "refund"),
    correctionRefs: refsByKind(links, "correction"),
    payoutHoldRefs: refsByKind(links, "payout_hold"),
    encodesPricingAssumption: false,
    reasonCodes: item.reason_codes,
    evidenceLinks: links,
  };
}

function buildGrantRecord(
  item: AccountingSummary,
  attachments: Readonly<Record<string, readonly AccountingAttachment[]>>,
): GrantVisibilityRecord {
  const localAttachments = getAttachments(attachments, item.id);
  const links = buildAccountingLinks(
    [{ kind: "grant", ref: item.id, sourceService: "overgrant" }, ...localAttachments],
    item.audit_refs,
    "overgrant",
    item.trace_id,
    item.reason_codes,
  );
  return {
    grantRef: item.id,
    tenantId: item.tenant_id,
    traceId: item.trace_id,
    state: resolveGrantRightsState(item, localAttachments),
    grantScopeRefs: refsByKind(links, "grant"),
    sponsoredAllocationRefs: refsByKind(links, "settled_accounting"),
    purposeScopeRefs: refsByKind(links, "purpose_scope"),
    expirationRefs: links.filter((link) => link.reasonCodes.some((reason) => reason.includes("expired"))).map((link) => link.ref),
    correctionRefs: refsByKind(links, "correction"),
    readOnly: true,
    reasonCodes: item.reason_codes,
    evidenceLinks: links,
  };
}

function buildRightsRecord(
  item: AccountingSummary,
  attachments: Readonly<Record<string, readonly AccountingAttachment[]>>,
): RightsVisibilityRecord {
  const localAttachments = getAttachments(attachments, item.id);
  const links = buildAccountingLinks(
    [{ kind: "resource_right", ref: item.id, sourceService: "overasset" }, ...localAttachments],
    item.audit_refs,
    "overasset",
    item.trace_id,
    item.reason_codes,
  );
  return {
    assetRef: item.id,
    tenantId: item.tenant_id,
    traceId: item.trace_id,
    state: resolveGrantRightsState(item, localAttachments),
    resourceRightsRefs: refsByKind(links, "resource_right"),
    storageBindingRefs: refsByKind(links, "storage_binding"),
    namespaceBindingRefs: refsByKind(links, "namespace_binding"),
    routeBindingRefs: refsByKind(links, "route_binding"),
    entitlementRefs: refsByKind(links, "entitlement"),
    expirationRefs: links.filter((link) => link.reasonCodes.some((reason) => reason.includes("expired"))).map((link) => link.ref),
    correctionRefs: refsByKind(links, "correction"),
    blockchainOwnershipModel: false,
    nftOwnershipModel: false,
    reasonCodes: item.reason_codes,
    evidenceLinks: links,
  };
}

function buildAccountingLinks(
  attachments: readonly AccountingAttachment[],
  auditRefs: readonly AccountingAuditRef[],
  defaultSourceService: string,
  traceId: TraceId,
  reasonCodes: readonly ReasonCode[],
): readonly AccountingEvidenceLink[] {
  const attachmentLinks = attachments.map((attachment) => {
    const kind = normalizeAttachmentKind(attachment.kind, attachment.ref);
    return {
      kind,
      ref: attachment.ref,
      sourceService: attachment.sourceService ?? defaultSourceService,
      traceId: attachment.traceId ?? traceId,
      reasonCodes: uniqueReasonCodes([...(attachment.reasonCodes ?? []), ...reasonCodes]),
      auditRefs: auditRefs.map(formatAuditRef),
      stable: isStableAccountingRef(attachment.ref),
    } satisfies AccountingEvidenceLink;
  });
  const auditLinks = auditRefs.map((audit) => ({
    kind: "audit" as const,
    ref: audit.audit_id,
    sourceService: audit.source_service,
    traceId: audit.trace_id,
    reasonCodes: uniqueReasonCodes(reasonCodes),
    auditRefs: [formatAuditRef(audit)],
    stable: isStableAccountingRef(audit.audit_id),
  }));
  return dedupeLinks([...attachmentLinks, ...auditLinks]);
}

function buildAccountingAccessRule(role: AccountingRole): AccountingAccessRule {
  const allowed = {
    usage: true,
    ledger: role !== "support_viewer" && role !== "product_integrator" && role !== "incident_responder",
    billing: role === "platform_owner" || role === "tenant_owner" || role === "accounting_viewer",
    grants: role === "platform_owner" || role === "tenant_owner" || role === "accounting_viewer",
    rights: role !== "support_viewer",
    access: role === "platform_owner" || role === "tenant_owner",
  } satisfies Readonly<Record<AccountingPanelKind, boolean>>;
  return {
    role,
    allowedPanels: allowed,
    crossTenantAccess: role === "platform_owner",
    redactedFields:
      role === "support_viewer" || role === "product_integrator" || role === "incident_responder"
        ? ["payment_provider_refs", "payout_hold_refs", "tenant_private_refs"]
        : [],
    reasonCodes: allowed.ledger
      ? (["admin.accounting.role_authorized"] as ReasonCode[])
      : (["admin.accounting.role_limited"] as ReasonCode[]),
  };
}

function collectAccountingDependencyStatuses(
  capabilities: AdminCapabilitiesResponse,
): readonly AdminDependencyStatus[] {
  const statuses = new Map<string, AdminDependencyStatus>();
  for (const disabledPanel of capabilities.disabled_panels) {
    const status = normalizeDependencyStatus(disabledPanel.dependency_status);
    statuses.set(status.service, status);
  }
  return REQUIRED_ACCOUNTING_DEPENDENCIES.map((service) => {
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

function resolveUsageDimension(
  item: AccountingSummary,
  attachments: readonly AccountingAttachment[],
): UsageResourceDimension {
  const explicit = attachments.find((attachment) => attachment.dimension)?.dimension;
  if (explicit) {
    return explicit;
  }
  const haystack = createAccountingSignalText(item, attachments).toUpperCase();
  return REQUIRED_USAGE_DIMENSIONS.find((dimension) => haystack.includes(dimension)) ?? "CPU-ORU";
}

function resolveUsageGroupings(
  item: AccountingSummary,
  attachments: readonly AccountingAttachment[],
): readonly UsageRollupGrouping[] {
  const haystack = createAccountingSignalText(item, attachments).toLowerCase();
  const groupings = REQUIRED_USAGE_GROUPINGS.filter(
    (grouping) =>
      attachments.some((attachment) => attachment.kind === grouping) ||
      haystack.includes(grouping) ||
      (grouping === "trace_id" && Boolean(item.trace_id)),
  );
  return groupings.length ? groupings : (["tenant", "trace_id"] as const);
}

function resolveUsageSettlementState(
  item: AccountingSummary,
  links: readonly AccountingEvidenceLink[],
): UsageRollupRecord["settlementState"] {
  const haystack = [item.state, ...item.reason_codes, ...links.map((link) => link.ref)].join(" ").toLowerCase();
  if (haystack.includes("dispute")) {
    return "disputed";
  }
  if (haystack.includes("hold") || haystack.includes("held")) {
    return "held";
  }
  if (links.some((link) => link.kind === "settled_accounting" || link.kind === "ledger" || link.kind === "receipt")) {
    return "settled";
  }
  return "observed_only";
}

function resolveLedgerState(
  item: AccountingSummary,
  attachments: readonly AccountingAttachment[],
): LedgerBalanceState {
  const explicit = attachments.find((attachment) => attachment.ledgerState)?.ledgerState;
  if (explicit) {
    return explicit;
  }
  const haystack = createAccountingSignalText(item, attachments).toLowerCase();
  if (haystack.includes("reserved")) return "reserved";
  if (haystack.includes("held") || haystack.includes("hold")) return "held";
  if (haystack.includes("spent")) return "spent";
  if (haystack.includes("earned")) return "earned";
  if (haystack.includes("sponsored")) return "sponsored";
  if (haystack.includes("refund") || haystack.includes("corrected")) return "refunded_corrected";
  if (haystack.includes("expired") || haystack.includes("revoked")) return "expired_revoked";
  if (haystack.includes("disputed")) return "disputed";
  return "available";
}

function resolveBillingState(
  item: AccountingSummary,
  attachments: readonly AccountingAttachment[],
): BillingDocumentState {
  const explicit = attachments.find((attachment) => attachment.billingState)?.billingState;
  if (explicit) {
    return explicit;
  }
  const haystack = createAccountingSignalText(item, attachments).toLowerCase();
  if (haystack.includes("invoice") && haystack.includes("settled")) return "invoice_settled";
  if (haystack.includes("invoice")) return "invoice_open";
  if (haystack.includes("payment_provider")) return "payment_provider_ref_visible";
  if (haystack.includes("refund")) return "refund_visible";
  if (haystack.includes("correction")) return "correction_visible";
  if (haystack.includes("payout_hold")) return "payout_hold_visible";
  return "receipt_recorded";
}

function resolveGrantRightsState(
  item: AccountingSummary,
  attachments: readonly AccountingAttachment[],
): GrantRightsState {
  const explicit = attachments.find((attachment) => attachment.grantState)?.grantState;
  if (explicit) {
    return explicit;
  }
  const haystack = createAccountingSignalText(item, attachments).toLowerCase();
  if (haystack.includes("sponsored")) return "sponsored";
  if (haystack.includes("purpose")) return "purpose_scoped";
  if (haystack.includes("expired")) return "expired";
  if (haystack.includes("corrected")) return "corrected";
  if (haystack.includes("revoked")) return "revoked";
  return "active";
}

function normalizeAttachmentKind(kind: AccountingAttachment["kind"], ref: OverridRef): AccountingRefKind {
  if (isAccountingRefKind(kind)) {
    return kind;
  }
  const prefix = ref.split(":", 1)[0];
  if (prefix === "trace") return "trace";
  if (prefix === "ledger" || prefix === "seal_ledger" || prefix === "oru") return "ledger";
  if (prefix === "receipt") return "receipt";
  if (prefix === "invoice") return "invoice";
  if (prefix === "grant") return "grant";
  if (prefix === "rights" || prefix === "asset" || prefix === "overasset") return "resource_right";
  return "owning_service";
}

function isAccountingRefKind(kind: string): kind is AccountingRefKind {
  return [
    "trace",
    "audit",
    "usage_rollup",
    "observed_usage",
    "settled_accounting",
    "ledger",
    "receipt",
    "invoice",
    "payment_provider",
    "refund",
    "correction",
    "payout_hold",
    "grant",
    "purpose_scope",
    "resource_right",
    "storage_binding",
    "namespace_binding",
    "route_binding",
    "entitlement",
    "owning_service",
  ].includes(kind);
}

function resolveAccountingStatus(
  response: AdminListResponse,
  recordCount: number,
  dependencyStatuses: readonly AdminDependencyStatus[],
  linkCheck: AccountingRefConsistencyResult,
): AccountingViewStatus {
  if (!linkCheck.ok) {
    return "disabled";
  }
  if (dependencyStatuses.some((dependency) => dependency.status === "unavailable")) {
    return "disabled";
  }
  if (response.status === "empty" || recordCount === 0) {
    return "empty";
  }
  if (
    response.status === "degraded" ||
    dependencyStatuses.some((dependency) => dependency.status === "degraded")
  ) {
    return "degraded";
  }
  if (dependencyStatuses.some((dependency) => dependency.status === "redacted")) {
    return "redacted";
  }
  return "ready";
}

function getAttachments(
  attachments: Readonly<Record<string, readonly AccountingAttachment[]>>,
  ref: OverridRef,
): readonly AccountingAttachment[] {
  return attachments[ref] ?? [];
}

function createAccountingSignalText(
  item: AccountingSummary,
  attachments: readonly AccountingAttachment[],
): string {
  return [
    item.id,
    item.kind,
    item.state,
    item.trace_id,
    ...item.reason_codes,
    ...attachments.flatMap((attachment) => [
      attachment.kind,
      attachment.ref,
      attachment.dimension ?? "",
      attachment.ledgerState ?? "",
      attachment.billingState ?? "",
      attachment.grantState ?? "",
      ...(attachment.reasonCodes ?? []),
    ]),
  ].join(" ");
}

function refsByKind(
  links: readonly AccountingEvidenceLink[],
  kind: AccountingRefKind,
): readonly OverridRef[] {
  return uniqueRefs(links.filter((link) => link.kind === kind).map((link) => link.ref));
}

function collectUsageLinks(records: readonly UsageRollupRecord[]): readonly AccountingEvidenceLink[] {
  return records.flatMap((record) => record.evidenceLinks);
}

function collectLedgerLinks(records: readonly LedgerReadRecord[]): readonly AccountingEvidenceLink[] {
  return records.flatMap((record) => record.evidenceLinks);
}

function collectBillingLinks(records: readonly BillingDocumentRecord[]): readonly AccountingEvidenceLink[] {
  return records.flatMap((record) => record.evidenceLinks);
}

function collectGrantLinks(records: readonly GrantVisibilityRecord[]): readonly AccountingEvidenceLink[] {
  return records.flatMap((record) => record.evidenceLinks);
}

function collectRightsLinks(records: readonly RightsVisibilityRecord[]): readonly AccountingEvidenceLink[] {
  return records.flatMap((record) => record.evidenceLinks);
}

function isStableAccountingRef(ref: OverridRef): boolean {
  const [prefix, value] = ref.split(":", 2);
  return Boolean(prefix && value && STABLE_ACCOUNTING_PREFIXES.includes(prefix as (typeof STABLE_ACCOUNTING_PREFIXES)[number]));
}

function isMutableAccountingRef(ref: OverridRef): boolean {
  const lower = ref.toLowerCase();
  return lower.includes(":draft") || lower.includes(":editable") || lower.includes(":mutable");
}

function uniqueRefs(refs: readonly OverridRef[]): readonly OverridRef[] {
  return [...new Set(refs)];
}

function uniqueReasonCodes(reasonCodes: readonly ReasonCode[]): readonly ReasonCode[] {
  return [...new Set(reasonCodes)];
}

function dedupeLinks(links: readonly AccountingEvidenceLink[]): readonly AccountingEvidenceLink[] {
  const seen = new Set<string>();
  return links.filter((link) => {
    const key = `${link.kind}:${link.ref}:${link.sourceService}`;
    if (seen.has(key)) {
      return false;
    }
    seen.add(key);
    return true;
  });
}

function formatAuditRef(ref: AccountingAuditRef): string {
  return `${ref.source_service}:${ref.audit_id}:${ref.trace_id}`;
}

function normalizeDependencyStatus(status: AdminDependencyStatus): AdminDependencyStatus {
  return {
    ...status,
    service: status.service.toLowerCase(),
  };
}

function assertRoute(actual: AdminRoutePath, expected: AdminRoutePath): void {
  if (actual !== expected) {
    throw new Error(`Admin accounting response path ${actual} does not match expected route ${expected}`);
  }
}
