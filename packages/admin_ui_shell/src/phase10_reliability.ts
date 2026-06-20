import type {
  AdminCapabilitiesResponse,
  AdminDependencyStatus,
  AdminRoutePath,
  AuditRef,
  OverridRef,
  ReasonCode,
  TraceId,
  UiDiagnosticEvent,
} from "./contracts";

export const PHASE10_SCHEMA_VERSION = "operator-phase10-validation.v0.1" as const;

export const REQUIRED_PHASE10_PRODUCTS = ["docdex", "mcoda", "codali", "sdk", "cli"] as const;
export type Phase10Product = (typeof REQUIRED_PHASE10_PRODUCTS)[number];

export const REQUIRED_PHASE10_RELIABILITY_OUTCOMES = [
  "successful",
  "retryable_failure",
  "final_failure",
  "cancelled",
  "timed_out",
  "policy_denied",
  "budget_exhausted",
  "node_disconnected",
  "disputed_usage",
] as const;
export type Phase10ReliabilityOutcome = (typeof REQUIRED_PHASE10_RELIABILITY_OUTCOMES)[number];

export const REQUIRED_PHASE10_SECURITY_PROBES = [
  "cross_tenant_access",
  "role_limits",
  "redaction_profiles",
  "encrypted_docdex_rag_metadata",
  "secret_bearing_fields",
  "key_metadata",
  "diagnostic_bundles",
  "raw_prompts",
  "decrypted_snippets",
  "file_paths",
  "query_text",
  "credentials",
  "private_payloads",
  "unredacted_result_contents",
] as const;
export type Phase10SecurityProbeKind = (typeof REQUIRED_PHASE10_SECURITY_PROBES)[number];

export const REQUIRED_PHASE10_ACCESSIBILITY_CHECKS = [
  "keyboard_navigation",
  "focus_order",
  "screen_reader_labels",
  "long_reason_code_wrapping",
  "stable_table_dimensions",
  "loading_state",
  "empty_state",
  "responsive_behavior",
] as const;
export type Phase10AccessibilityCheckKind = (typeof REQUIRED_PHASE10_ACCESSIBILITY_CHECKS)[number];

export const REQUIRED_PHASE10_CONTRACT_CHECKS = [
  "generated_bindings",
  "schema_compatibility",
  "overgate_admin_routes",
  "read_only_mode",
  "action_submission",
  "idempotency",
  "stale_state_blocking",
  "timeline_assembly",
] as const;
export type Phase10ContractCheckKind = (typeof REQUIRED_PHASE10_CONTRACT_CHECKS)[number];

export const REQUIRED_PHASE10_HANDOFF_SURFACES = [
  "system_service_operations",
  "incident_readiness",
  "break_glass_execution",
  "governance_reporting",
  "compliance_views",
] as const;
export type Phase10HandoffSurfaceKind = (typeof REQUIRED_PHASE10_HANDOFF_SURFACES)[number];

export const PHASE10_FORBIDDEN_RENDER_FIELDS = [
  "rawPromptsRendered",
  "decryptedSnippetsRendered",
  "privateFilePathsRendered",
  "queryTextRendered",
  "keyMaterialRendered",
  "credentialsRendered",
  "secretsRendered",
  "privatePayloadsRendered",
  "unredactedResultContentsRendered",
] as const;

export type Phase10ForbiddenRenderField = (typeof PHASE10_FORBIDDEN_RENDER_FIELDS)[number];
export type Phase10ContractRoute = AdminRoutePath | "/admin/actions";
export type Phase10ContractMethod = "GET" | "POST";
export type Phase10HandoffGate = "phase7" | "phase13";

export interface Phase10ProductReliabilityCase {
  readonly product: Phase10Product;
  readonly outcome: Phase10ReliabilityOutcome;
  readonly workloadId: OverridRef;
  readonly tenantId: OverridRef;
  readonly traceId: TraceId;
  readonly auditRefs: readonly AuditRef[];
  readonly usageRefs: readonly OverridRef[];
  readonly receiptRefs: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
  readonly evidenceRefs: readonly OverridRef[];
  readonly diagnosticBundleRef: OverridRef;
  readonly readableAuditTrail: true;
  readonly readableUsageTrail: true;
  readonly readableReceiptTrail: true;
  readonly readableReasonTrail: true;
}

export interface Phase10SecurityProbe {
  readonly probe: Phase10SecurityProbeKind;
  readonly owningServiceAuthorized: boolean;
  readonly redactionProfileRef: OverridRef;
  readonly safeRefs: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
  readonly auditRefs: readonly AuditRef[];
  readonly rawPromptsRendered: false;
  readonly decryptedSnippetsRendered: false;
  readonly privateFilePathsRendered: false;
  readonly queryTextRendered: false;
  readonly keyMaterialRendered: false;
  readonly credentialsRendered: false;
  readonly secretsRendered: false;
  readonly privatePayloadsRendered: false;
  readonly unredactedResultContentsRendered: false;
}

export interface Phase10AccessibilityCheck {
  readonly check: Phase10AccessibilityCheckKind;
  readonly automated: true;
  readonly manualReview: true;
  readonly stableDimensions: true;
  readonly evidenceRefs: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
}

export interface Phase10ContractCheck {
  readonly check: Phase10ContractCheckKind;
  readonly method: Phase10ContractMethod;
  readonly route: Phase10ContractRoute;
  readonly pass: true;
  readonly usesGeneratedBindings: true;
  readonly usesOvergateRoute: true;
  readonly noPrivilegedBackdoor: true;
  readonly idempotencyProtected?: true;
  readonly staleStateProtected?: true;
  readonly readOnlyMode?: true;
  readonly evidenceRefs: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
}

export interface Phase10HandoffSurface {
  readonly surface: Phase10HandoffSurfaceKind;
  readonly phaseGate: Phase10HandoffGate;
  readonly disabled: true;
  readonly readinessOnly: true;
  readonly highRiskOperationEnabled: false;
  readonly evidenceRefs: readonly OverridRef[];
  readonly reasonCodes: readonly ReasonCode[];
}

export interface Phase10ValidationInput {
  readonly productCases: readonly Phase10ProductReliabilityCase[];
  readonly securityProbes: readonly Phase10SecurityProbe[];
  readonly accessibilityChecks: readonly Phase10AccessibilityCheck[];
  readonly contractChecks: readonly Phase10ContractCheck[];
  readonly handoffSurfaces: readonly Phase10HandoffSurface[];
  readonly diagnosticEvents?: readonly UiDiagnosticEvent[];
  readonly dependencyStatuses?: readonly AdminDependencyStatus[];
  readonly capabilities?: AdminCapabilitiesResponse;
  readonly usesOvergateOnly?: true;
  readonly directStorageAccess?: false;
  readonly directServiceAccess?: false;
  readonly directOverwatchConnection?: false;
  readonly rustServicesAuthoritative?: true;
  readonly typeScriptClientSurfaceOnly?: true;
}

export interface Phase10CoverageReport {
  readonly products: Readonly<Record<Phase10Product, boolean>>;
  readonly outcomes: Readonly<Record<Phase10ReliabilityOutcome, boolean>>;
  readonly securityProbes: Readonly<Record<Phase10SecurityProbeKind, boolean>>;
  readonly accessibilityChecks: Readonly<Record<Phase10AccessibilityCheckKind, boolean>>;
  readonly contractChecks: Readonly<Record<Phase10ContractCheckKind, boolean>>;
  readonly handoffSurfaces: Readonly<Record<Phase10HandoffSurfaceKind, boolean>>;
}

export interface Phase10DiagnosticBundle {
  readonly schemaVersion: typeof PHASE10_SCHEMA_VERSION;
  readonly traceIds: readonly TraceId[];
  readonly reasonCodes: readonly ReasonCode[];
  readonly safeRefs: readonly OverridRef[];
  readonly auditRefs: readonly AuditRef[];
  readonly redactedFields: readonly string[];
  readonly rawDiagnosticPayloadIncluded: false;
  readonly containsUnsafeContent: boolean;
}

export interface Phase10ValidationReport {
  readonly schemaVersion: typeof PHASE10_SCHEMA_VERSION;
  readonly ok: boolean;
  readonly coverage: Phase10CoverageReport;
  readonly missingProducts: readonly Phase10Product[];
  readonly missingOutcomes: readonly Phase10ReliabilityOutcome[];
  readonly missingSecurityProbes: readonly Phase10SecurityProbeKind[];
  readonly missingAccessibilityChecks: readonly Phase10AccessibilityCheckKind[];
  readonly missingContractChecks: readonly Phase10ContractCheckKind[];
  readonly missingHandoffSurfaces: readonly Phase10HandoffSurfaceKind[];
  readonly unsafeContentPaths: readonly string[];
  readonly unsafeSecurityRenderFlags: readonly Phase10ForbiddenRenderField[];
  readonly unsafeHandoffSurfaces: readonly Phase10HandoffSurfaceKind[];
  readonly diagnosticBundle: Phase10DiagnosticBundle;
  readonly dependencyStatuses: readonly AdminDependencyStatus[];
  readonly capabilities?: AdminCapabilitiesResponse;
  readonly usesOvergateOnly: true;
  readonly directStorageAccess: false;
  readonly directServiceAccess: false;
  readonly directOverwatchConnection: false;
  readonly rustServicesAuthoritative: true;
  readonly typeScriptClientSurfaceOnly: true;
}

const UNSAFE_PHASE10_PATTERNS: readonly RegExp[] = [
  /\bpassword\b/i,
  /\bcredential\b/i,
  /\bsecret\b/i,
  /\btoken\b/i,
  /\bprivate payload\b/i,
  /\bdecrypted\b/i,
  /\braw prompt\b/i,
  /\bprompt text\b/i,
  /\bquery text\b/i,
  /\bkey material\b/i,
  /-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----/i,
  /(?:^|\s)\/Users\/[^\s]+/i,
  /(?:^|\s)\/home\/[^\s]+/i,
  /[A-Za-z]:\\[^\s]+/,
];

function unique<T extends string>(values: readonly T[]): T[] {
  return Array.from(new Set(values));
}

function coverageMap<T extends string>(
  required: readonly T[],
  values: readonly T[],
): Readonly<Record<T, boolean>> {
  const covered = new Set<T>(values);
  return required.reduce((result, item) => {
    result[item] = covered.has(item);
    return result;
  }, {} as Record<T, boolean>);
}

function missingRequired<T extends string>(required: readonly T[], values: readonly T[]): T[] {
  const covered = new Set<T>(values);
  return required.filter((item) => !covered.has(item));
}

function nonEmpty<T>(values: readonly T[]): boolean {
  return values.length > 0;
}

export function isPhase10OvergateRoute(route: Phase10ContractRoute): boolean {
  return route === "/admin/actions" || route.startsWith("/admin/");
}

export function listUnsafePhase10Content(value: unknown, path = "$"): string[] {
  const unsafePaths: string[] = [];

  if (typeof value === "string") {
    if (UNSAFE_PHASE10_PATTERNS.some((pattern) => pattern.test(value))) {
      unsafePaths.push(path);
    }
    return unsafePaths;
  }

  if (Array.isArray(value)) {
    value.forEach((item, index) => {
      unsafePaths.push(...listUnsafePhase10Content(item, `${path}[${index}]`));
    });
    return unsafePaths;
  }

  if (value && typeof value === "object") {
    Object.entries(value as Readonly<Record<string, unknown>>).forEach(([key, item]) => {
      unsafePaths.push(...listUnsafePhase10Content(item, `${path}.${key}`));
    });
  }

  return unsafePaths;
}

export function containsUnsafePhase10Content(value: unknown): boolean {
  return listUnsafePhase10Content(value).length > 0;
}

export function hasRequiredProductCoverage(cases: readonly Phase10ProductReliabilityCase[]): boolean {
  return missingRequired(
    REQUIRED_PHASE10_PRODUCTS,
    cases.map((item) => item.product),
  ).length === 0;
}

export function hasRequiredOutcomeCoverage(cases: readonly Phase10ProductReliabilityCase[]): boolean {
  return missingRequired(
    REQUIRED_PHASE10_RELIABILITY_OUTCOMES,
    cases.map((item) => item.outcome),
  ).length === 0;
}

export function securityRenderFlags(probe: Phase10SecurityProbe): readonly Phase10ForbiddenRenderField[] {
  const flags: Phase10ForbiddenRenderField[] = [];

  PHASE10_FORBIDDEN_RENDER_FIELDS.forEach((field) => {
    if (probe[field] !== false) {
      flags.push(field);
    }
  });

  return flags;
}

export function isPhase10ProductCaseReadable(item: Phase10ProductReliabilityCase): boolean {
  return (
    item.readableAuditTrail === true &&
    item.readableUsageTrail === true &&
    item.readableReceiptTrail === true &&
    item.readableReasonTrail === true &&
    nonEmpty(item.auditRefs) &&
    nonEmpty(item.usageRefs) &&
    nonEmpty(item.receiptRefs) &&
    nonEmpty(item.reasonCodes) &&
    nonEmpty(item.evidenceRefs) &&
    !containsUnsafePhase10Content(item)
  );
}

export function isPhase10SecurityProbeSafe(probe: Phase10SecurityProbe): boolean {
  return (
    securityRenderFlags(probe).length === 0 &&
    nonEmpty(probe.auditRefs) &&
    nonEmpty(probe.reasonCodes) &&
    nonEmpty(probe.safeRefs) &&
    !containsUnsafePhase10Content(probe)
  );
}

export function isPhase10AccessibilityCheckComplete(check: Phase10AccessibilityCheck): boolean {
  return (
    check.automated === true &&
    check.manualReview === true &&
    check.stableDimensions === true &&
    nonEmpty(check.evidenceRefs) &&
    nonEmpty(check.reasonCodes)
  );
}

export function isPhase10ContractCheckComplete(check: Phase10ContractCheck): boolean {
  return (
    check.pass === true &&
    check.usesGeneratedBindings === true &&
    check.usesOvergateRoute === true &&
    check.noPrivilegedBackdoor === true &&
    isPhase10OvergateRoute(check.route) &&
    nonEmpty(check.evidenceRefs) &&
    nonEmpty(check.reasonCodes)
  );
}

export function isPhase10HandoffSurfaceSafe(surface: Phase10HandoffSurface): boolean {
  return (
    surface.disabled === true &&
    surface.readinessOnly === true &&
    surface.highRiskOperationEnabled === false &&
    nonEmpty(surface.evidenceRefs) &&
    nonEmpty(surface.reasonCodes)
  );
}

export function hasRequiredSecurityCoverage(probes: readonly Phase10SecurityProbe[]): boolean {
  return (
    missingRequired(
      REQUIRED_PHASE10_SECURITY_PROBES,
      probes.filter(isPhase10SecurityProbeSafe).map((probe) => probe.probe),
    ).length === 0
  );
}

export function hasRequiredAccessibilityCoverage(checks: readonly Phase10AccessibilityCheck[]): boolean {
  return (
    missingRequired(
      REQUIRED_PHASE10_ACCESSIBILITY_CHECKS,
      checks.filter(isPhase10AccessibilityCheckComplete).map((check) => check.check),
    ).length === 0
  );
}

export function hasRequiredContractCoverage(checks: readonly Phase10ContractCheck[]): boolean {
  return (
    missingRequired(
      REQUIRED_PHASE10_CONTRACT_CHECKS,
      checks.filter(isPhase10ContractCheckComplete).map((check) => check.check),
    ).length === 0
  );
}

export function hasRequiredHandoffCoverage(surfaces: readonly Phase10HandoffSurface[]): boolean {
  return (
    missingRequired(
      REQUIRED_PHASE10_HANDOFF_SURFACES,
      surfaces.filter(isPhase10HandoffSurfaceSafe).map((surface) => surface.surface),
    ).length === 0
  );
}

export function buildPhase10DiagnosticBundle(
  events: readonly UiDiagnosticEvent[],
  productCases: readonly Phase10ProductReliabilityCase[],
  redactedFields: readonly string[] = [],
): Phase10DiagnosticBundle {
  const traceIds = unique([
    ...events.map((event) => event.trace_id),
    ...productCases.map((item) => item.traceId),
  ]);
  const reasonCodes = unique([
    ...events.map((event) => event.reason_code),
    ...productCases.flatMap((item) => item.reasonCodes),
  ]);
  const safeRefs = unique([
    ...events.flatMap((event) => event.safe_refs),
    ...productCases.flatMap((item) => [
      item.workloadId,
      item.tenantId,
      item.diagnosticBundleRef,
      ...item.usageRefs,
      ...item.receiptRefs,
      ...item.evidenceRefs,
    ]),
  ]);
  const auditRefs = productCases.flatMap((item) => item.auditRefs);

  return {
    schemaVersion: PHASE10_SCHEMA_VERSION,
    traceIds,
    reasonCodes,
    safeRefs,
    auditRefs,
    redactedFields,
    rawDiagnosticPayloadIncluded: false,
    containsUnsafeContent: containsUnsafePhase10Content({ events, productCases, redactedFields }),
  };
}

export function buildPhase10ValidationReport(input: Phase10ValidationInput): Phase10ValidationReport {
  const readableProductCases = input.productCases.filter(isPhase10ProductCaseReadable);
  const safeSecurityProbes = input.securityProbes.filter(isPhase10SecurityProbeSafe);
  const completeAccessibilityChecks = input.accessibilityChecks.filter(isPhase10AccessibilityCheckComplete);
  const completeContractChecks = input.contractChecks.filter(isPhase10ContractCheckComplete);
  const safeHandoffSurfaces = input.handoffSurfaces.filter(isPhase10HandoffSurfaceSafe);
  const diagnosticBundle = buildPhase10DiagnosticBundle(
    input.diagnosticEvents ?? [],
    input.productCases,
    PHASE10_FORBIDDEN_RENDER_FIELDS.map((field) => String(field)),
  );
  const unsafeSecurityRenderFlags = unique(
    input.securityProbes.flatMap((probe) => securityRenderFlags(probe)),
  );
  const unsafeHandoffSurfaces = input.handoffSurfaces
    .filter((surface) => !isPhase10HandoffSurfaceSafe(surface))
    .map((surface) => surface.surface);
  const unsafeContentPaths = listUnsafePhase10Content(input);

  const missingProducts = missingRequired(
    REQUIRED_PHASE10_PRODUCTS,
    readableProductCases.map((item) => item.product),
  );
  const missingOutcomes = missingRequired(
    REQUIRED_PHASE10_RELIABILITY_OUTCOMES,
    readableProductCases.map((item) => item.outcome),
  );
  const missingSecurityProbes = missingRequired(
    REQUIRED_PHASE10_SECURITY_PROBES,
    safeSecurityProbes.map((probe) => probe.probe),
  );
  const missingAccessibilityChecks = missingRequired(
    REQUIRED_PHASE10_ACCESSIBILITY_CHECKS,
    completeAccessibilityChecks.map((check) => check.check),
  );
  const missingContractChecks = missingRequired(
    REQUIRED_PHASE10_CONTRACT_CHECKS,
    completeContractChecks.map((check) => check.check),
  );
  const missingHandoffSurfaces = missingRequired(
    REQUIRED_PHASE10_HANDOFF_SURFACES,
    safeHandoffSurfaces.map((surface) => surface.surface),
  );

  const ok =
    missingProducts.length === 0 &&
    missingOutcomes.length === 0 &&
    missingSecurityProbes.length === 0 &&
    missingAccessibilityChecks.length === 0 &&
    missingContractChecks.length === 0 &&
    missingHandoffSurfaces.length === 0 &&
    unsafeContentPaths.length === 0 &&
    unsafeSecurityRenderFlags.length === 0 &&
    unsafeHandoffSurfaces.length === 0;

  return {
    schemaVersion: PHASE10_SCHEMA_VERSION,
    ok,
    coverage: {
      products: coverageMap(
        REQUIRED_PHASE10_PRODUCTS,
        readableProductCases.map((item) => item.product),
      ),
      outcomes: coverageMap(
        REQUIRED_PHASE10_RELIABILITY_OUTCOMES,
        readableProductCases.map((item) => item.outcome),
      ),
      securityProbes: coverageMap(
        REQUIRED_PHASE10_SECURITY_PROBES,
        safeSecurityProbes.map((probe) => probe.probe),
      ),
      accessibilityChecks: coverageMap(
        REQUIRED_PHASE10_ACCESSIBILITY_CHECKS,
        completeAccessibilityChecks.map((check) => check.check),
      ),
      contractChecks: coverageMap(
        REQUIRED_PHASE10_CONTRACT_CHECKS,
        completeContractChecks.map((check) => check.check),
      ),
      handoffSurfaces: coverageMap(
        REQUIRED_PHASE10_HANDOFF_SURFACES,
        safeHandoffSurfaces.map((surface) => surface.surface),
      ),
    },
    missingProducts,
    missingOutcomes,
    missingSecurityProbes,
    missingAccessibilityChecks,
    missingContractChecks,
    missingHandoffSurfaces,
    unsafeContentPaths,
    unsafeSecurityRenderFlags,
    unsafeHandoffSurfaces,
    diagnosticBundle,
    dependencyStatuses: input.dependencyStatuses ?? [],
    capabilities: input.capabilities,
    usesOvergateOnly: true,
    directStorageAccess: false,
    directServiceAccess: false,
    directOverwatchConnection: false,
    rustServicesAuthoritative: true,
    typeScriptClientSurfaceOnly: true,
  };
}
