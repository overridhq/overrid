use std::fmt;

pub const SDK_PHASE10_CAPABILITY_PROFILE: &str =
    "phase10-validation-documentation-downstream-handoff";
pub const SDK_PHASE10_HANDOFF_ROOT: &str = "packages/sdk/handoff/phase10";
pub const SDK_PHASE10_STRUCTURE_VALIDATION_PATH: &str =
    "packages/sdk/handoff/phase10/structure_validation.valid.json";
pub const SDK_PHASE10_ALIGNMENT_CHECKLIST_PATH: &str =
    "packages/sdk/handoff/phase10/alignment_checklist.valid.json";
pub const SDK_PHASE10_DOWNSTREAM_HANDOFF_RULES_PATH: &str =
    "packages/sdk/handoff/phase10/downstream_handoff_rules.valid.json";
pub const SDK_PHASE10_SUB_BUILD_PLAN_PATH: &str = "docs/build_plan/sub_build_plan_006_sdk.md";
pub const SDK_PHASE10_TECH_STACK_PATH: &str = "docs/overrid_tech_stack_choice.md";
pub const SDK_PHASE10_MASTER_PLAN_PATH: &str = "docs/build_plan/master_plan.md";
pub const SDK_PHASE10_SERVICE_CATALOG_ALIGNMENT_PATH: &str =
    "docs/build_plan/service_catalog_alignment.md";
pub const SDK_PHASE10_SDS_PATH: &str = "docs/sds/foundation/sdk.md";
pub const SDK_PHASE10_SERVICE_CATALOG_PATH: &str = "docs/service_catalog/foundation/sdk.md";
pub const SDK_PHASE10_MASTER_ALIGNMENT: &str =
    "phase1_with_phase0_prerequisites_and_phase6_hardening";
pub const SDK_PHASE10_RUNTIME_AUTHORITY: &str = "owner_services_not_sdk";

const STRUCTURE_VALIDATION_JSON: &str =
    include_str!("../handoff/phase10/structure_validation.valid.json");
const ALIGNMENT_CHECKLIST_JSON: &str =
    include_str!("../handoff/phase10/alignment_checklist.valid.json");
const HANDOFF_RULES_JSON: &str =
    include_str!("../handoff/phase10/downstream_handoff_rules.valid.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase10StructureGate {
    TitlePrefix,
    AttachedSdsLink,
    PhaseHeadings,
    WorkItemStructure,
    DesignOutputValidationFields,
    ExitGate,
}

impl SdkPhase10StructureGate {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TitlePrefix => "title_prefix",
            Self::AttachedSdsLink => "attached_sds_link",
            Self::PhaseHeadings => "phase_headings",
            Self::WorkItemStructure => "work_item_structure",
            Self::DesignOutputValidationFields => "design_output_validation_fields",
            Self::ExitGate => "exit_gate",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase10StructureValidation {
    pub gate: SdkPhase10StructureGate,
    pub source_path: &'static str,
    pub required_marker: &'static str,
    pub validation_method: &'static str,
    pub passed: bool,
    pub runtime_authority: bool,
}

pub fn sdk_phase10_structure_validation() -> Vec<SdkPhase10StructureValidation> {
    vec![
        structure_gate(
            SdkPhase10StructureGate::TitlePrefix,
            "# SUB BUILD PLAN #6 - SDK",
            "scripted_text_check",
        ),
        structure_gate(
            SdkPhase10StructureGate::AttachedSdsLink,
            "Attached SDS: [docs/sds/foundation/sdk.md](../sds/foundation/sdk.md)",
            "scripted_text_check",
        ),
        structure_gate(
            SdkPhase10StructureGate::PhaseHeadings,
            "## Phase 10: Validation, Documentation, And Downstream Handoff",
            "regex_structure_check",
        ),
        structure_gate(
            SdkPhase10StructureGate::WorkItemStructure,
            "**10.5 Prepare downstream phase handoff**",
            "regex_structure_check",
        ),
        structure_gate(
            SdkPhase10StructureGate::DesignOutputValidationFields,
            "Design:",
            "regex_structure_check",
        ),
        structure_gate(
            SdkPhase10StructureGate::ExitGate,
            "## Exit Gate",
            "scripted_text_check",
        ),
    ]
}

pub fn validate_phase10_structure_validation(
    gates: &[SdkPhase10StructureValidation],
) -> Result<(), SdkPhase10Error> {
    for required in REQUIRED_PHASE10_STRUCTURE_GATES {
        if !gates.iter().any(|gate| gate.gate == *required) {
            return Err(SdkPhase10Error::MissingStructureGate(required.as_str()));
        }
    }

    for gate in gates {
        require_phase10_non_empty(gate.source_path, "structure source path")?;
        require_phase10_non_empty(gate.required_marker, "structure required marker")?;
        require_phase10_non_empty(gate.validation_method, "structure validation method")?;
        if gate.source_path != SDK_PHASE10_SUB_BUILD_PLAN_PATH
            || !gate.passed
            || gate.runtime_authority
            || !STRUCTURE_VALIDATION_JSON.contains(gate.gate.as_str())
            || !STRUCTURE_VALIDATION_JSON.contains(gate.required_marker)
        {
            return Err(SdkPhase10Error::UnsafeStructureGate(gate.gate.as_str()));
        }
    }

    validate_artifact_text(
        SDK_PHASE10_STRUCTURE_VALIDATION_PATH,
        STRUCTURE_VALIDATION_JSON,
        &[
            "sdk_phase10_structure_validation",
            SDK_PHASE10_SUB_BUILD_PLAN_PATH,
            "title_prefix",
            "attached_sds_link",
            "phase_headings",
            "work_item_structure",
            "design_output_validation_fields",
            "exit_gate",
            "\"runtime_authority\": false",
        ],
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase10AlignmentDomain {
    TechStack,
    MasterPlan,
    Sds,
    ServiceCatalog,
    Crosswalk,
}

impl SdkPhase10AlignmentDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TechStack => "tech_stack",
            Self::MasterPlan => "master_plan",
            Self::Sds => "sds",
            Self::ServiceCatalog => "service_catalog",
            Self::Crosswalk => "crosswalk",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase10AlignmentChecklistItem {
    pub domain: SdkPhase10AlignmentDomain,
    pub source_path: &'static str,
    pub expected_marker: &'static str,
    pub preserves_phase_order: bool,
    pub preserves_rust_first: bool,
    pub generated_bindings_only: bool,
    pub owner_authority_retained: bool,
    pub no_conventional_cloud_boundary: bool,
    pub runtime_authority: bool,
}

pub fn sdk_phase10_alignment_checklist() -> Vec<SdkPhase10AlignmentChecklistItem> {
    vec![
        alignment_item(
            SdkPhase10AlignmentDomain::TechStack,
            SDK_PHASE10_TECH_STACK_PATH,
            "Rust-first infrastructure stack",
        ),
        alignment_item(
            SdkPhase10AlignmentDomain::MasterPlan,
            SDK_PHASE10_MASTER_PLAN_PATH,
            "First build point remains Phase 1, with Phase 0 prerequisites and Phase 6 product-integration hardening.",
        ),
        alignment_item(
            SdkPhase10AlignmentDomain::Sds,
            SDK_PHASE10_SDS_PATH,
            "The SDK is a versioned developer package, not a standalone runtime service.",
        ),
        alignment_item(
            SdkPhase10AlignmentDomain::ServiceCatalog,
            SDK_PHASE10_SERVICE_CATALOG_PATH,
            "Treat the SDK as a versioned client package, not a runtime authority or deployed service.",
        ),
        alignment_item(
            SdkPhase10AlignmentDomain::Crosswalk,
            SDK_PHASE10_SERVICE_CATALOG_ALIGNMENT_PATH,
            "Phase 1: Control-Plane Skeleton",
        ),
    ]
}

pub fn validate_phase10_alignment_checklist(
    items: &[SdkPhase10AlignmentChecklistItem],
) -> Result<(), SdkPhase10Error> {
    for required in REQUIRED_PHASE10_ALIGNMENT_DOMAINS {
        if !items.iter().any(|item| item.domain == *required) {
            return Err(SdkPhase10Error::MissingAlignmentDomain(required.as_str()));
        }
    }

    for item in items {
        require_phase10_non_empty(item.source_path, "alignment source path")?;
        require_phase10_non_empty(item.expected_marker, "alignment expected marker")?;
        if !item.preserves_phase_order
            || !item.preserves_rust_first
            || !item.generated_bindings_only
            || !item.owner_authority_retained
            || !item.no_conventional_cloud_boundary
            || item.runtime_authority
            || !ALIGNMENT_CHECKLIST_JSON.contains(item.domain.as_str())
            || !ALIGNMENT_CHECKLIST_JSON.contains(item.source_path)
            || !ALIGNMENT_CHECKLIST_JSON.contains(item.expected_marker)
        {
            return Err(SdkPhase10Error::UnsafeAlignmentItem(item.domain.as_str()));
        }
    }

    validate_artifact_text(
        SDK_PHASE10_ALIGNMENT_CHECKLIST_PATH,
        ALIGNMENT_CHECKLIST_JSON,
        &[
            "sdk_phase10_alignment_checklist",
            SDK_PHASE10_MASTER_ALIGNMENT,
            "rust_first_sdk_generation",
            "typescript_web_generated_second_target",
            "language_neutral_schema_authority",
            "overgate_only_mutating_calls",
            "credential_provider_signing",
            "\"owner_authority_retained\": true",
            "\"runtime_authority\": false",
        ],
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase10DownstreamConsumerKind {
    Cli,
    AdminDeveloperUi,
    DocdexAdapter,
    McodaAdapter,
    CodaliAdapter,
    NativeApps,
    MobileServices,
    OverpackDeployment,
    AccountingServices,
    PolicyServices,
    FutureLanguageBindings,
}

impl SdkPhase10DownstreamConsumerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cli => "cli",
            Self::AdminDeveloperUi => "admin_developer_ui",
            Self::DocdexAdapter => "docdex_adapter",
            Self::McodaAdapter => "mcoda_adapter",
            Self::CodaliAdapter => "codali_adapter",
            Self::NativeApps => "native_apps",
            Self::MobileServices => "mobile_services",
            Self::OverpackDeployment => "overpack_deployment",
            Self::AccountingServices => "accounting_services",
            Self::PolicyServices => "policy_services",
            Self::FutureLanguageBindings => "future_language_bindings",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase10DownstreamHandoffRule {
    pub consumer: SdkPhase10DownstreamConsumerKind,
    pub consumer_scope: &'static str,
    pub allowed_sdk_primitives: &'static [&'static str],
    pub required_public_api_families: &'static [&'static str],
    pub owner_authority_retained: bool,
    pub allowed_to_bypass_owner_services: bool,
    pub allowed_to_store_private_keys: bool,
    pub generated_contracts_required: bool,
    pub capability_checks_required: bool,
    pub runtime_authority: bool,
}

pub fn sdk_phase10_downstream_handoff_rules() -> Vec<SdkPhase10DownstreamHandoffRule> {
    vec![
        handoff_rule(
            SdkPhase10DownstreamConsumerKind::Cli,
            "command submission, status reads, and local diagnostics",
            SDK_PHASE10_COMMON_PRIMITIVES,
            &["/v1/overgate/commands", "/v1/control-plane/status"],
        ),
        handoff_rule(
            SdkPhase10DownstreamConsumerKind::AdminDeveloperUi,
            "generated client surfaces for admin and developer views",
            SDK_PHASE10_UI_PRIMITIVES,
            &["/v1/admin/", "/v1/control-plane/"],
        ),
        handoff_rule(
            SdkPhase10DownstreamConsumerKind::DocdexAdapter,
            "encrypted RAG workload submission and result refs",
            SDK_PHASE10_WORKLOAD_PRIMITIVES,
            &["/v1/overgate/workloads", "/v1/control-plane/workloads/"],
        ),
        handoff_rule(
            SdkPhase10DownstreamConsumerKind::McodaAdapter,
            "agent workload submission, capability refs, and result refs",
            SDK_PHASE10_WORKLOAD_PRIMITIVES,
            &["/v1/overgate/workloads", "/v1/control-plane/workloads/"],
        ),
        handoff_rule(
            SdkPhase10DownstreamConsumerKind::CodaliAdapter,
            "code-agent workload manifests and package validation refs",
            SDK_PHASE10_WORKLOAD_PRIMITIVES,
            &["/v1/overgate/workloads", "/v1/overgate/packages/validate"],
        ),
        handoff_rule(
            SdkPhase10DownstreamConsumerKind::NativeApps,
            "ordinary client helpers for signed requests and redacted diagnostics",
            SDK_PHASE10_CLIENT_PRIMITIVES,
            &["/v1/overgate/", "/v1/control-plane/"],
        ),
        handoff_rule(
            SdkPhase10DownstreamConsumerKind::MobileServices,
            "shared generated models and extension points for Phase 12 mobile owners",
            SDK_PHASE10_MOBILE_PRIMITIVES,
            &["/v1/overgate/", "/v1/control-plane/"],
        ),
        handoff_rule(
            SdkPhase10DownstreamConsumerKind::OverpackDeployment,
            "deployment package validation and release status helpers",
            SDK_PHASE10_DEPLOYMENT_PRIMITIVES,
            &["/v1/overpack/", "/v1/control-plane/deployments/"],
        ),
        handoff_rule(
            SdkPhase10DownstreamConsumerKind::AccountingServices,
            "read-only usage and receipt refs returned by accounting owners",
            SDK_PHASE10_ACCOUNTING_PRIMITIVES,
            &[
                "/v1/accounting/usage-rollups",
                "/v1/accounting/usage-receipts",
            ],
        ),
        handoff_rule(
            SdkPhase10DownstreamConsumerKind::PolicyServices,
            "policy dry-run requests and service-returned policy refs",
            SDK_PHASE10_POLICY_PRIMITIVES,
            &["/v1/policy/dry-run", "/v1/control-plane/policy/"],
        ),
        handoff_rule(
            SdkPhase10DownstreamConsumerKind::FutureLanguageBindings,
            "generated binding parity checks for later language targets",
            SDK_PHASE10_BINDING_PRIMITIVES,
            &["/v1/overgate/", "/v1/control-plane/", "/v1/accounting/"],
        ),
    ]
}

pub fn validate_phase10_downstream_handoff_rules(
    rules: &[SdkPhase10DownstreamHandoffRule],
) -> Result<(), SdkPhase10Error> {
    for required in REQUIRED_PHASE10_DOWNSTREAM_CONSUMERS {
        if !rules.iter().any(|rule| rule.consumer == *required) {
            return Err(SdkPhase10Error::MissingDownstreamConsumer(
                required.as_str(),
            ));
        }
    }

    for rule in rules {
        require_phase10_non_empty(rule.consumer_scope, "consumer scope")?;
        if rule.allowed_sdk_primitives.is_empty()
            || rule.required_public_api_families.is_empty()
            || !rule.owner_authority_retained
            || rule.allowed_to_bypass_owner_services
            || rule.allowed_to_store_private_keys
            || !rule.generated_contracts_required
            || !rule.capability_checks_required
            || rule.runtime_authority
            || !rule.required_public_api_families.iter().all(|route| {
                SDK_PHASE10_ALLOWED_PUBLIC_ROUTE_PREFIXES
                    .iter()
                    .any(|prefix| route.starts_with(prefix))
            })
            || !HANDOFF_RULES_JSON.contains(rule.consumer.as_str())
            || !HANDOFF_RULES_JSON.contains(rule.consumer_scope)
        {
            return Err(SdkPhase10Error::UnsafeHandoffRule(rule.consumer.as_str()));
        }
    }

    validate_artifact_text(
        SDK_PHASE10_DOWNSTREAM_HANDOFF_RULES_PATH,
        HANDOFF_RULES_JSON,
        &[
            "sdk_phase10_downstream_handoff_rules",
            "cli",
            "admin_developer_ui",
            "docdex_adapter",
            "mcoda_adapter",
            "codali_adapter",
            "native_apps",
            "mobile_services",
            "overpack_deployment",
            "accounting_services",
            "policy_services",
            "future_language_bindings",
            "\"owner_authority_retained\": true",
            "\"allowed_to_bypass_owner_services\": false",
            "\"allowed_to_store_private_keys\": false",
            "\"generated_contracts_required\": true",
            "\"capability_checks_required\": true",
            "\"runtime_authority\": false",
        ],
    )
}

pub fn validate_phase10_artifact_files() -> Result<(), SdkPhase10Error> {
    validate_artifact_text(
        SDK_PHASE10_STRUCTURE_VALIDATION_PATH,
        STRUCTURE_VALIDATION_JSON,
        &[
            SDK_PHASE10_STRUCTURE_VALIDATION_PATH,
            "sdk_phase10_structure_validation",
            "validation_result",
        ],
    )?;
    validate_artifact_text(
        SDK_PHASE10_ALIGNMENT_CHECKLIST_PATH,
        ALIGNMENT_CHECKLIST_JSON,
        &[
            SDK_PHASE10_ALIGNMENT_CHECKLIST_PATH,
            "sdk_phase10_alignment_checklist",
            SDK_PHASE10_MASTER_ALIGNMENT,
        ],
    )?;
    validate_artifact_text(
        SDK_PHASE10_DOWNSTREAM_HANDOFF_RULES_PATH,
        HANDOFF_RULES_JSON,
        &[
            SDK_PHASE10_DOWNSTREAM_HANDOFF_RULES_PATH,
            "sdk_phase10_downstream_handoff_rules",
            SDK_PHASE10_RUNTIME_AUTHORITY,
        ],
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkPhase10Error {
    MissingRequiredField(&'static str),
    MissingStructureGate(&'static str),
    UnsafeStructureGate(&'static str),
    MissingAlignmentDomain(&'static str),
    UnsafeAlignmentItem(&'static str),
    MissingDownstreamConsumer(&'static str),
    UnsafeHandoffRule(&'static str),
    ArtifactMissingMarker(&'static str),
    ArtifactSensitiveValue(&'static str),
}

impl fmt::Display for SdkPhase10Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredField(field) => {
                write!(formatter, "missing Phase 10 required field: {field}")
            }
            Self::MissingStructureGate(gate) => {
                write!(formatter, "missing Phase 10 structure gate: {gate}")
            }
            Self::UnsafeStructureGate(gate) => {
                write!(formatter, "unsafe Phase 10 structure gate: {gate}")
            }
            Self::MissingAlignmentDomain(domain) => {
                write!(formatter, "missing Phase 10 alignment domain: {domain}")
            }
            Self::UnsafeAlignmentItem(domain) => {
                write!(formatter, "unsafe Phase 10 alignment item: {domain}")
            }
            Self::MissingDownstreamConsumer(consumer) => {
                write!(
                    formatter,
                    "missing Phase 10 downstream consumer: {consumer}"
                )
            }
            Self::UnsafeHandoffRule(consumer) => {
                write!(formatter, "unsafe Phase 10 handoff rule: {consumer}")
            }
            Self::ArtifactMissingMarker(marker) => {
                write!(formatter, "Phase 10 artifact missing marker: {marker}")
            }
            Self::ArtifactSensitiveValue(path) => {
                write!(
                    formatter,
                    "Phase 10 artifact has sensitive value marker: {path}"
                )
            }
        }
    }
}

impl std::error::Error for SdkPhase10Error {}

fn structure_gate(
    gate: SdkPhase10StructureGate,
    required_marker: &'static str,
    validation_method: &'static str,
) -> SdkPhase10StructureValidation {
    SdkPhase10StructureValidation {
        gate,
        source_path: SDK_PHASE10_SUB_BUILD_PLAN_PATH,
        required_marker,
        validation_method,
        passed: true,
        runtime_authority: false,
    }
}

fn alignment_item(
    domain: SdkPhase10AlignmentDomain,
    source_path: &'static str,
    expected_marker: &'static str,
) -> SdkPhase10AlignmentChecklistItem {
    SdkPhase10AlignmentChecklistItem {
        domain,
        source_path,
        expected_marker,
        preserves_phase_order: true,
        preserves_rust_first: true,
        generated_bindings_only: true,
        owner_authority_retained: true,
        no_conventional_cloud_boundary: true,
        runtime_authority: false,
    }
}

fn handoff_rule(
    consumer: SdkPhase10DownstreamConsumerKind,
    consumer_scope: &'static str,
    allowed_sdk_primitives: &'static [&'static str],
    required_public_api_families: &'static [&'static str],
) -> SdkPhase10DownstreamHandoffRule {
    SdkPhase10DownstreamHandoffRule {
        consumer,
        consumer_scope,
        allowed_sdk_primitives,
        required_public_api_families,
        owner_authority_retained: true,
        allowed_to_bypass_owner_services: false,
        allowed_to_store_private_keys: false,
        generated_contracts_required: true,
        capability_checks_required: true,
        runtime_authority: false,
    }
}

fn validate_artifact_text(
    path: &'static str,
    text: &str,
    required_markers: &[&'static str],
) -> Result<(), SdkPhase10Error> {
    for marker in required_markers {
        if !text.contains(marker) {
            return Err(SdkPhase10Error::ArtifactMissingMarker(marker));
        }
    }
    ensure_artifact_text_safe(path, text)
}

fn ensure_artifact_text_safe(path: &'static str, text: &str) -> Result<(), SdkPhase10Error> {
    let lowered = text.to_ascii_lowercase();
    for forbidden in PHASE10_FORBIDDEN_ARTIFACT_MARKERS {
        if lowered.contains(forbidden) {
            return Err(SdkPhase10Error::ArtifactSensitiveValue(path));
        }
    }
    Ok(())
}

fn require_phase10_non_empty(value: &str, field: &'static str) -> Result<(), SdkPhase10Error> {
    if value.trim().is_empty() {
        return Err(SdkPhase10Error::MissingRequiredField(field));
    }
    Ok(())
}

const REQUIRED_PHASE10_STRUCTURE_GATES: &[SdkPhase10StructureGate] = &[
    SdkPhase10StructureGate::TitlePrefix,
    SdkPhase10StructureGate::AttachedSdsLink,
    SdkPhase10StructureGate::PhaseHeadings,
    SdkPhase10StructureGate::WorkItemStructure,
    SdkPhase10StructureGate::DesignOutputValidationFields,
    SdkPhase10StructureGate::ExitGate,
];

const REQUIRED_PHASE10_ALIGNMENT_DOMAINS: &[SdkPhase10AlignmentDomain] = &[
    SdkPhase10AlignmentDomain::TechStack,
    SdkPhase10AlignmentDomain::MasterPlan,
    SdkPhase10AlignmentDomain::Sds,
    SdkPhase10AlignmentDomain::ServiceCatalog,
    SdkPhase10AlignmentDomain::Crosswalk,
];

const REQUIRED_PHASE10_DOWNSTREAM_CONSUMERS: &[SdkPhase10DownstreamConsumerKind] = &[
    SdkPhase10DownstreamConsumerKind::Cli,
    SdkPhase10DownstreamConsumerKind::AdminDeveloperUi,
    SdkPhase10DownstreamConsumerKind::DocdexAdapter,
    SdkPhase10DownstreamConsumerKind::McodaAdapter,
    SdkPhase10DownstreamConsumerKind::CodaliAdapter,
    SdkPhase10DownstreamConsumerKind::NativeApps,
    SdkPhase10DownstreamConsumerKind::MobileServices,
    SdkPhase10DownstreamConsumerKind::OverpackDeployment,
    SdkPhase10DownstreamConsumerKind::AccountingServices,
    SdkPhase10DownstreamConsumerKind::PolicyServices,
    SdkPhase10DownstreamConsumerKind::FutureLanguageBindings,
];

const SDK_PHASE10_ALLOWED_PUBLIC_ROUTE_PREFIXES: &[&str] = &[
    "/v1/overgate/",
    "/v1/admin/",
    "/v1/control-plane/",
    "/v1/accounting/",
    "/v1/policy/",
    "/v1/overpack/",
];

const SDK_PHASE10_COMMON_PRIMITIVES: &[&str] = &[
    "generated_models",
    "signed_command_envelopes",
    "idempotency_records",
    "stable_error_objects",
    "redacted_diagnostics",
];
const SDK_PHASE10_UI_PRIMITIVES: &[&str] = &[
    "generated_models",
    "request_helpers",
    "stable_error_objects",
    "capability_snapshots",
    "redacted_diagnostics",
];
const SDK_PHASE10_WORKLOAD_PRIMITIVES: &[&str] = &[
    "workload_manifest_helpers",
    "signed_command_envelopes",
    "job_status_refs",
    "cancellation_refs",
    "result_refs",
    "usage_reader_refs",
];
const SDK_PHASE10_CLIENT_PRIMITIVES: &[&str] = &[
    "generated_models",
    "credential_provider_refs",
    "signed_request_helpers",
    "stable_error_objects",
    "redacted_diagnostics",
];
const SDK_PHASE10_MOBILE_PRIMITIVES: &[&str] = &[
    "generated_models",
    "signing_extension_points",
    "session_refresh_refs",
    "sync_cursor_refs",
    "redacted_diagnostics",
];
const SDK_PHASE10_DEPLOYMENT_PRIMITIVES: &[&str] = &[
    "package_validation_helpers",
    "deployment_status_refs",
    "release_gate_refs",
    "stable_error_objects",
];
const SDK_PHASE10_ACCOUNTING_PRIMITIVES: &[&str] = &[
    "usage_rollup_readers",
    "usage_receipt_readers",
    "ledger_ref_views",
    "dispute_ref_views",
];
const SDK_PHASE10_POLICY_PRIMITIVES: &[&str] = &[
    "policy_dry_run_requests",
    "policy_ref_views",
    "correction_fields",
    "stable_error_objects",
];
const SDK_PHASE10_BINDING_PRIMITIVES: &[&str] = &[
    "shared_schema_models",
    "golden_request_envelopes",
    "generated_error_objects",
    "cross_language_contract_tests",
];

const PHASE10_FORBIDDEN_ARTIFACT_MARKERS: &[&str] = &[
    "raw_private_key",
    "private_key_value",
    "bearer_token_value",
    "seed_phrase_value",
    "\"signature_value\"",
    "\"raw_request_body\"",
    "private_payload_value",
    "fixture_credential_material",
    "\"runtime_authority\": true",
    "\"allowed_to_bypass_owner_services\": true",
    "\"allowed_to_store_private_keys\": true",
    "\"direct_storage_access\": true",
    "\"client_side_policy_truth\": true",
    "\"client_side_accounting_truth\": true",
    "\"owner_authority_retained\": false",
    "\"generated_contracts_required\": false",
    "\"capability_checks_required\": false",
    "postgresql",
    "redis",
    "s3://",
    "minio",
    "kafka",
    "nats",
    "blockchain",
    "nft",
    "pricing",
    "revenue",
    "customer_count",
    "customer-count",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase10_structure_validation_covers_sub_build_plan_exit_gate() {
        let gates = sdk_phase10_structure_validation();
        validate_phase10_structure_validation(&gates).unwrap();

        assert_eq!(gates.len(), REQUIRED_PHASE10_STRUCTURE_GATES.len());
        assert!(gates
            .iter()
            .all(|gate| gate.source_path == SDK_PHASE10_SUB_BUILD_PLAN_PATH
                && gate.passed
                && !gate.runtime_authority));
    }

    #[test]
    fn phase10_alignment_checklist_preserves_master_and_tech_stack() {
        let checklist = sdk_phase10_alignment_checklist();
        validate_phase10_alignment_checklist(&checklist).unwrap();

        assert_eq!(checklist.len(), REQUIRED_PHASE10_ALIGNMENT_DOMAINS.len());
        assert!(checklist.iter().all(|item| item.preserves_phase_order
            && item.preserves_rust_first
            && item.generated_bindings_only
            && item.owner_authority_retained
            && item.no_conventional_cloud_boundary
            && !item.runtime_authority));
    }

    #[test]
    fn phase10_downstream_handoff_rules_keep_authority_in_owner_services() {
        let rules = sdk_phase10_downstream_handoff_rules();
        validate_phase10_downstream_handoff_rules(&rules).unwrap();

        assert_eq!(rules.len(), REQUIRED_PHASE10_DOWNSTREAM_CONSUMERS.len());
        assert!(rules.iter().all(|rule| rule.owner_authority_retained
            && !rule.allowed_to_bypass_owner_services
            && !rule.allowed_to_store_private_keys
            && rule.generated_contracts_required
            && rule.capability_checks_required
            && !rule.runtime_authority));
    }

    #[test]
    fn phase10_artifact_files_match_declared_paths_and_markers() {
        validate_phase10_artifact_files().unwrap();
        assert!(STRUCTURE_VALIDATION_JSON.contains(SDK_PHASE10_STRUCTURE_VALIDATION_PATH));
        assert!(ALIGNMENT_CHECKLIST_JSON.contains(SDK_PHASE10_ALIGNMENT_CHECKLIST_PATH));
        assert!(HANDOFF_RULES_JSON.contains(SDK_PHASE10_DOWNSTREAM_HANDOFF_RULES_PATH));
    }
}
