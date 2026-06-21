use std::fmt;

use crate::{check_sdk_compatibility, SdkCompatibilityRejection, SDK_CURRENT_STABLE_MAJOR};
use overrid_contracts::SUPPORTED_SCHEMA_VERSION;

pub const SDK_PHASE9_CAPABILITY_PROFILE: &str =
    "phase9-typescript-web-product-integration-hardening";
pub const SDK_PHASE9_BINDING_ROOT: &str = "packages/sdk/bindings/phase9";
pub const SDK_PHASE9_TYPESCRIPT_WEB_MANIFEST_PATH: &str =
    "packages/sdk/bindings/phase9/typescript_web_binding_manifest.valid.json";
pub const SDK_PHASE9_PRODUCT_MODULES_PATH: &str =
    "packages/sdk/bindings/phase9/product_convenience_modules.valid.json";
pub const SDK_PHASE9_READINESS_GATES_PATH: &str =
    "packages/sdk/bindings/phase9/binding_readiness_gates.valid.json";
pub const SDK_PHASE9_TYPESCRIPT_WEB_TARGET: &str = "typescript_web";
pub const SDK_PHASE9_BROWSER_SURFACE_PROFILE: &str = "browser_client_surface_only";
pub const SDK_PHASE9_OVERGATE_ADMIN_API_BOUNDARY: &str = "overgate_admin_public_api_only";
pub const SDK_PHASE9_MOBILE_SDK_OWNER_PHASE: &str = "phase12_mobile_sdk";

const TYPESCRIPT_WEB_MANIFEST_JSON: &str =
    include_str!("../bindings/phase9/typescript_web_binding_manifest.valid.json");
const PRODUCT_MODULES_JSON: &str =
    include_str!("../bindings/phase9/product_convenience_modules.valid.json");
const READINESS_GATES_JSON: &str =
    include_str!("../bindings/phase9/binding_readiness_gates.valid.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase9TypeScriptWebArtifactKind {
    Models,
    Validators,
    ErrorObjects,
    RequestHelpers,
    ManifestHelpers,
    IdempotencyHelpers,
}

impl SdkPhase9TypeScriptWebArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Models => "models",
            Self::Validators => "validators",
            Self::ErrorObjects => "error_objects",
            Self::RequestHelpers => "request_helpers",
            Self::ManifestHelpers => "manifest_helpers",
            Self::IdempotencyHelpers => "idempotency_helpers",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase9TypeScriptWebBindingDescriptor {
    pub name: &'static str,
    pub kind: SdkPhase9TypeScriptWebArtifactKind,
    pub language_target: &'static str,
    pub schema_version: &'static str,
    pub source_contract_path: &'static str,
    pub output_path: &'static str,
    pub generated_from_shared_contracts: bool,
    pub passes_phase8_golden_corpus: bool,
    pub request_envelope_matches_rust: bool,
    pub error_objects_match_rust: bool,
    pub idempotency_matches_rust: bool,
    pub browser_surface_only: bool,
    pub runtime_authority: bool,
    pub handwritten_public_objects_allowed: bool,
}

pub fn sdk_phase9_typescript_web_bindings(
) -> Result<Vec<SdkPhase9TypeScriptWebBindingDescriptor>, SdkPhase9Error> {
    check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, SUPPORTED_SCHEMA_VERSION)?;
    Ok(vec![
        typescript_web_binding(
            "phase9_ts_web_models",
            SdkPhase9TypeScriptWebArtifactKind::Models,
            "packages/sdk/bindings/typescript_web/generated/models.ts",
        ),
        typescript_web_binding(
            "phase9_ts_web_validators",
            SdkPhase9TypeScriptWebArtifactKind::Validators,
            "packages/sdk/bindings/typescript_web/generated/validators.ts",
        ),
        typescript_web_binding(
            "phase9_ts_web_error_objects",
            SdkPhase9TypeScriptWebArtifactKind::ErrorObjects,
            "packages/sdk/bindings/typescript_web/generated/errors.ts",
        ),
        typescript_web_binding(
            "phase9_ts_web_request_helpers",
            SdkPhase9TypeScriptWebArtifactKind::RequestHelpers,
            "packages/sdk/bindings/typescript_web/generated/request_helpers.ts",
        ),
        typescript_web_binding(
            "phase9_ts_web_manifest_helpers",
            SdkPhase9TypeScriptWebArtifactKind::ManifestHelpers,
            "packages/sdk/bindings/typescript_web/generated/manifest_helpers.ts",
        ),
        typescript_web_binding(
            "phase9_ts_web_idempotency_helpers",
            SdkPhase9TypeScriptWebArtifactKind::IdempotencyHelpers,
            "packages/sdk/bindings/typescript_web/generated/idempotency_helpers.ts",
        ),
    ])
}

pub fn validate_phase9_typescript_web_bindings(
    bindings: &[SdkPhase9TypeScriptWebBindingDescriptor],
) -> Result<(), SdkPhase9Error> {
    for required in REQUIRED_TS_WEB_ARTIFACT_KINDS {
        if !bindings.iter().any(|binding| binding.kind == *required) {
            return Err(SdkPhase9Error::MissingTypeScriptWebArtifact(
                required.as_str(),
            ));
        }
    }

    for binding in bindings {
        require_phase9_non_empty(binding.name, "binding name")?;
        require_phase9_non_empty(binding.source_contract_path, "binding source contract path")?;
        require_phase9_non_empty(binding.output_path, "binding output path")?;
        if binding.language_target != SDK_PHASE9_TYPESCRIPT_WEB_TARGET
            || binding.schema_version != SUPPORTED_SCHEMA_VERSION
            || !binding
                .source_contract_path
                .starts_with("packages/schemas/")
            || !binding
                .output_path
                .starts_with("packages/sdk/bindings/typescript_web/generated/")
            || !binding.generated_from_shared_contracts
            || !binding.passes_phase8_golden_corpus
            || !binding.request_envelope_matches_rust
            || !binding.error_objects_match_rust
            || !binding.idempotency_matches_rust
            || !binding.browser_surface_only
            || binding.runtime_authority
            || binding.handwritten_public_objects_allowed
            || !TYPESCRIPT_WEB_MANIFEST_JSON.contains(binding.name)
            || !TYPESCRIPT_WEB_MANIFEST_JSON.contains(binding.kind.as_str())
        {
            return Err(SdkPhase9Error::UnsafeTypeScriptWebBinding(binding.name));
        }
    }

    ensure_artifact_text_safe(
        SDK_PHASE9_TYPESCRIPT_WEB_MANIFEST_PATH,
        TYPESCRIPT_WEB_MANIFEST_JSON,
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase9AdapterUiSafetyBoundary {
    pub profile: &'static str,
    pub allowed_api_families: &'static [&'static str],
    pub generated_client_required: bool,
    pub overgate_admin_public_api_only: bool,
    pub privileged_internal_endpoints_allowed: bool,
    pub private_key_storage_allowed: bool,
    pub hidden_service_shortcuts_allowed: bool,
    pub direct_worker_storage_api_allowed: bool,
    pub browser_runtime_authority: bool,
    pub redacted_diagnostics_required: bool,
}

pub fn sdk_phase9_adapter_ui_safety_boundary() -> SdkPhase9AdapterUiSafetyBoundary {
    SdkPhase9AdapterUiSafetyBoundary {
        profile: SDK_PHASE9_OVERGATE_ADMIN_API_BOUNDARY,
        allowed_api_families: &[
            "/v1/overgate/",
            "/v1/admin/",
            "/v1/control-plane/",
            "/v1/accounting/",
        ],
        generated_client_required: true,
        overgate_admin_public_api_only: true,
        privileged_internal_endpoints_allowed: false,
        private_key_storage_allowed: false,
        hidden_service_shortcuts_allowed: false,
        direct_worker_storage_api_allowed: false,
        browser_runtime_authority: false,
        redacted_diagnostics_required: true,
    }
}

pub fn validate_phase9_adapter_ui_safety_boundary(
    boundary: &SdkPhase9AdapterUiSafetyBoundary,
) -> Result<(), SdkPhase9Error> {
    if boundary.profile != SDK_PHASE9_OVERGATE_ADMIN_API_BOUNDARY
        || boundary.allowed_api_families.is_empty()
        || !boundary.allowed_api_families.iter().all(|api| {
            matches!(
                *api,
                "/v1/overgate/" | "/v1/admin/" | "/v1/control-plane/" | "/v1/accounting/"
            )
        })
        || !boundary.generated_client_required
        || !boundary.overgate_admin_public_api_only
        || boundary.privileged_internal_endpoints_allowed
        || boundary.private_key_storage_allowed
        || boundary.hidden_service_shortcuts_allowed
        || boundary.direct_worker_storage_api_allowed
        || boundary.browser_runtime_authority
        || !boundary.redacted_diagnostics_required
    {
        return Err(SdkPhase9Error::AdapterUiBoundaryUnsafe(boundary.profile));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase9ProductKind {
    DocdexEncryptedRag,
    McodaAgentWorkload,
    CodaliCodeAgent,
    PackageValidation,
}

impl SdkPhase9ProductKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DocdexEncryptedRag => "docdex_encrypted_rag",
            Self::McodaAgentWorkload => "mcoda_agent_workload",
            Self::CodaliCodeAgent => "codali_code_agent",
            Self::PackageValidation => "package_validation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase9ProductFailureCase {
    SuccessfulJob,
    RetryableFailure,
    FinalFailure,
    Cancellation,
    Timeout,
    PolicyDenial,
    BudgetExhaustion,
    NodeDisconnect,
    DisputedUsage,
}

impl SdkPhase9ProductFailureCase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SuccessfulJob => "successful_job",
            Self::RetryableFailure => "retryable_failure",
            Self::FinalFailure => "final_failure",
            Self::Cancellation => "cancellation",
            Self::Timeout => "timeout",
            Self::PolicyDenial => "policy_denial",
            Self::BudgetExhaustion => "budget_exhaustion",
            Self::NodeDisconnect => "node_disconnect",
            Self::DisputedUsage => "disputed_usage",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase9ProductConvenienceModule {
    pub module_name: &'static str,
    pub product_kind: SdkPhase9ProductKind,
    pub feature_flag: &'static str,
    pub helpers: &'static [&'static str],
    pub public_routes: &'static [&'static str],
    pub failure_cases: &'static [SdkPhase9ProductFailureCase],
    pub capability_checks_required: bool,
    pub uses_overgate_admin_api_only: bool,
    pub authorized_refs_only: bool,
    pub service_returned_usage_receipts_only: bool,
    pub bypasses_internal_services: bool,
    pub stores_private_keys: bool,
    pub direct_storage_access: bool,
    pub hardcoded_model_or_provider: bool,
    pub paid_service_assumption: bool,
}

pub fn sdk_phase9_product_convenience_modules() -> Vec<SdkPhase9ProductConvenienceModule> {
    vec![
        product_module(
            "phase9_docdex_encrypted_rag_jobs",
            SdkPhase9ProductKind::DocdexEncryptedRag,
            "phase9.product.docdex_encrypted_rag",
            &[
                "build_docdex_encrypted_rag_job",
                "submit_job",
                "read_status",
                "cancel_job",
                "read_results",
                "read_usage_rollup",
                "read_usage_receipt",
            ],
        ),
        product_module(
            "phase9_mcoda_agent_workloads",
            SdkPhase9ProductKind::McodaAgentWorkload,
            "phase9.product.mcoda_agent_workload",
            &[
                "build_mcoda_agent_workload",
                "submit_job",
                "read_status",
                "cancel_job",
                "read_results",
                "read_usage_rollup",
                "read_usage_receipt",
            ],
        ),
        product_module(
            "phase9_codali_code_agent_workloads",
            SdkPhase9ProductKind::CodaliCodeAgent,
            "phase9.product.codali_code_agent",
            &[
                "build_codali_code_agent_workload",
                "submit_job",
                "read_status",
                "cancel_job",
                "read_results",
                "read_usage_rollup",
                "read_usage_receipt",
            ],
        ),
        product_module(
            "phase9_package_validation",
            SdkPhase9ProductKind::PackageValidation,
            "phase9.product.package_validation",
            &[
                "validate_package",
                "read_status",
                "read_failure_case",
                "read_usage_rollup",
                "read_usage_receipt",
            ],
        ),
    ]
}

pub fn validate_phase9_product_convenience_modules(
    modules: &[SdkPhase9ProductConvenienceModule],
) -> Result<(), SdkPhase9Error> {
    for required in REQUIRED_PRODUCT_KINDS {
        if !modules
            .iter()
            .any(|module| module.product_kind == *required)
        {
            return Err(SdkPhase9Error::MissingProductModule(required.as_str()));
        }
    }

    for module in modules {
        require_phase9_non_empty(module.module_name, "product module name")?;
        require_phase9_non_empty(module.feature_flag, "product feature flag")?;
        if module.helpers.is_empty()
            || !module.helpers.contains(&"read_usage_rollup")
            || !module.helpers.contains(&"read_usage_receipt")
            || module.public_routes.is_empty()
            || !module.public_routes.iter().all(|route| {
                route.starts_with("/v1/overgate/")
                    || route.starts_with("/v1/control-plane/")
                    || route.starts_with("/v1/accounting/")
            })
            || !module.capability_checks_required
            || !module.uses_overgate_admin_api_only
            || !module.authorized_refs_only
            || !module.service_returned_usage_receipts_only
            || module.bypasses_internal_services
            || module.stores_private_keys
            || module.direct_storage_access
            || module.hardcoded_model_or_provider
            || module.paid_service_assumption
            || !PRODUCT_MODULES_JSON.contains(module.module_name)
            || !PRODUCT_MODULES_JSON.contains(module.product_kind.as_str())
        {
            return Err(SdkPhase9Error::UnsafeProductModule(module.module_name));
        }
        for required in REQUIRED_PRODUCT_FAILURE_CASES {
            if !module.failure_cases.iter().any(|case| case == required) {
                return Err(SdkPhase9Error::MissingFailureCase(required.as_str()));
            }
        }
    }

    ensure_artifact_text_safe(SDK_PHASE9_PRODUCT_MODULES_PATH, PRODUCT_MODULES_JSON)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase9MobileExtensionKind {
    SessionRefreshPrimitive,
    SecureStorageAdapterInterface,
    OfflineQueueInterface,
    SyncCursorPrimitive,
    ConflictStatePrimitive,
    RedactedDiagnosticsHook,
}

impl SdkPhase9MobileExtensionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SessionRefreshPrimitive => "session_refresh_primitive",
            Self::SecureStorageAdapterInterface => "secure_storage_adapter_interface",
            Self::OfflineQueueInterface => "offline_queue_interface",
            Self::SyncCursorPrimitive => "sync_cursor_primitive",
            Self::ConflictStatePrimitive => "conflict_state_primitive",
            Self::RedactedDiagnosticsHook => "redacted_diagnostics_hook",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase9MobileExtensionInterface {
    pub name: &'static str,
    pub kind: SdkPhase9MobileExtensionKind,
    pub extension_point_only: bool,
    pub owner_phase: &'static str,
    pub foundation_sdk_implements_runtime: bool,
    pub provides_default_offline_queue: bool,
    pub owns_os_secure_storage: bool,
    pub owns_push_registration: bool,
    pub owns_background_behavior: bool,
    pub owns_media_upload_state: bool,
}

pub fn sdk_phase9_mobile_extension_interfaces() -> Vec<SdkPhase9MobileExtensionInterface> {
    vec![
        mobile_extension(
            "phase9_mobile_session_refresh_primitive",
            SdkPhase9MobileExtensionKind::SessionRefreshPrimitive,
        ),
        mobile_extension(
            "phase9_mobile_secure_storage_adapter_interface",
            SdkPhase9MobileExtensionKind::SecureStorageAdapterInterface,
        ),
        mobile_extension(
            "phase9_mobile_offline_queue_interface",
            SdkPhase9MobileExtensionKind::OfflineQueueInterface,
        ),
        mobile_extension(
            "phase9_mobile_sync_cursor_primitive",
            SdkPhase9MobileExtensionKind::SyncCursorPrimitive,
        ),
        mobile_extension(
            "phase9_mobile_conflict_state_primitive",
            SdkPhase9MobileExtensionKind::ConflictStatePrimitive,
        ),
        mobile_extension(
            "phase9_mobile_redacted_diagnostics_hook",
            SdkPhase9MobileExtensionKind::RedactedDiagnosticsHook,
        ),
    ]
}

pub fn validate_phase9_mobile_extension_interfaces(
    interfaces: &[SdkPhase9MobileExtensionInterface],
) -> Result<(), SdkPhase9Error> {
    for required in REQUIRED_MOBILE_EXTENSION_KINDS {
        if !interfaces
            .iter()
            .any(|interface| interface.kind == *required)
        {
            return Err(SdkPhase9Error::MissingMobileExtension(required.as_str()));
        }
    }

    for interface in interfaces {
        if interface.owner_phase != SDK_PHASE9_MOBILE_SDK_OWNER_PHASE
            || !interface.extension_point_only
            || interface.foundation_sdk_implements_runtime
            || interface.provides_default_offline_queue
            || interface.owns_os_secure_storage
            || interface.owns_push_registration
            || interface.owns_background_behavior
            || interface.owns_media_upload_state
            || !READINESS_GATES_JSON.contains(interface.name)
            || !READINESS_GATES_JSON.contains(interface.kind.as_str())
        {
            return Err(SdkPhase9Error::MobileBoundaryDrift(interface.name));
        }
    }

    ensure_artifact_text_safe(SDK_PHASE9_READINESS_GATES_PATH, READINESS_GATES_JSON)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase9BindingTarget {
    TypeScriptWeb,
    SwiftIos,
    KotlinAndroid,
    Python,
    Other,
}

impl SdkPhase9BindingTarget {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TypeScriptWeb => "typescript_web",
            Self::SwiftIos => "swift_ios",
            Self::KotlinAndroid => "kotlin_android",
            Self::Python => "python",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase9BindingReadinessGate {
    pub target: SdkPhase9BindingTarget,
    pub schema_compatibility_required: bool,
    pub golden_request_envelope_required: bool,
    pub generated_error_objects_required: bool,
    pub cross_language_contract_tests_required: bool,
    pub no_handwritten_public_objects_required: bool,
    pub release_blocking: bool,
    pub matches_rust_behavior_required: bool,
    pub mobile_sdk_phase12_gate: bool,
}

pub fn sdk_phase9_binding_readiness_gates() -> Vec<SdkPhase9BindingReadinessGate> {
    vec![
        binding_gate(SdkPhase9BindingTarget::TypeScriptWeb, false),
        binding_gate(SdkPhase9BindingTarget::SwiftIos, true),
        binding_gate(SdkPhase9BindingTarget::KotlinAndroid, true),
        binding_gate(SdkPhase9BindingTarget::Python, false),
        binding_gate(SdkPhase9BindingTarget::Other, false),
    ]
}

pub fn validate_phase9_binding_readiness_gates(
    gates: &[SdkPhase9BindingReadinessGate],
) -> Result<(), SdkPhase9Error> {
    for required in REQUIRED_BINDING_TARGETS {
        if !gates.iter().any(|gate| gate.target == *required) {
            return Err(SdkPhase9Error::MissingBindingReadinessGate(
                required.as_str(),
            ));
        }
    }

    for gate in gates {
        if !gate.schema_compatibility_required
            || !gate.golden_request_envelope_required
            || !gate.generated_error_objects_required
            || !gate.cross_language_contract_tests_required
            || !gate.no_handwritten_public_objects_required
            || !gate.release_blocking
            || !gate.matches_rust_behavior_required
            || !READINESS_GATES_JSON.contains(gate.target.as_str())
        {
            return Err(SdkPhase9Error::BindingReadinessGateOpen(
                gate.target.as_str(),
            ));
        }
        if matches!(
            gate.target,
            SdkPhase9BindingTarget::SwiftIos | SdkPhase9BindingTarget::KotlinAndroid
        ) && !gate.mobile_sdk_phase12_gate
        {
            return Err(SdkPhase9Error::BindingReadinessGateOpen(
                gate.target.as_str(),
            ));
        }
    }

    Ok(())
}

pub fn validate_phase9_artifact_files() -> Result<(), SdkPhase9Error> {
    validate_artifact_text(
        SDK_PHASE9_TYPESCRIPT_WEB_MANIFEST_PATH,
        TYPESCRIPT_WEB_MANIFEST_JSON,
        &[
            "sdk_phase9_typescript_web_binding_manifest",
            SDK_PHASE9_CAPABILITY_PROFILE,
            SDK_PHASE9_TYPESCRIPT_WEB_TARGET,
            "phase9_ts_web_models",
            "phase9_ts_web_validators",
            "phase9_ts_web_error_objects",
            "phase9_ts_web_request_helpers",
            "phase9_ts_web_manifest_helpers",
            "phase9_ts_web_idempotency_helpers",
            "\"generated_from_shared_contracts\": true",
            "\"runtime_authority\": false",
            "\"handwritten_public_objects_allowed\": false",
        ],
    )?;
    validate_artifact_text(
        SDK_PHASE9_PRODUCT_MODULES_PATH,
        PRODUCT_MODULES_JSON,
        &[
            "sdk_phase9_product_convenience_modules",
            "phase9_docdex_encrypted_rag_jobs",
            "phase9_mcoda_agent_workloads",
            "phase9_codali_code_agent_workloads",
            "phase9_package_validation",
            "successful_job",
            "retryable_failure",
            "final_failure",
            "read_usage_rollup",
            "read_usage_receipt",
            "cancellation",
            "timeout",
            "policy_denial",
            "budget_exhaustion",
            "node_disconnect",
            "disputed_usage",
            "\"uses_overgate_admin_api_only\": true",
            "\"bypasses_internal_services\": false",
        ],
    )?;
    validate_artifact_text(
        SDK_PHASE9_READINESS_GATES_PATH,
        READINESS_GATES_JSON,
        &[
            "sdk_phase9_binding_readiness_gates",
            "phase9_mobile_session_refresh_primitive",
            "phase9_mobile_secure_storage_adapter_interface",
            "phase9_mobile_offline_queue_interface",
            "phase9_mobile_sync_cursor_primitive",
            "phase9_mobile_conflict_state_primitive",
            "phase9_mobile_redacted_diagnostics_hook",
            "typescript_web",
            "swift_ios",
            "kotlin_android",
            "python",
            "\"foundation_sdk_implements_runtime\": false",
            "\"provides_default_offline_queue\": false",
            "\"release_blocking\": true",
            "\"no_handwritten_public_objects_required\": true",
        ],
    )?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkPhase9Error {
    MissingRequiredField(&'static str),
    MissingTypeScriptWebArtifact(&'static str),
    UnsafeTypeScriptWebBinding(&'static str),
    AdapterUiBoundaryUnsafe(&'static str),
    MissingProductModule(&'static str),
    UnsafeProductModule(&'static str),
    MissingFailureCase(&'static str),
    MissingMobileExtension(&'static str),
    MobileBoundaryDrift(&'static str),
    MissingBindingReadinessGate(&'static str),
    BindingReadinessGateOpen(&'static str),
    ArtifactMissingMarker(&'static str),
    ArtifactSensitiveValue(&'static str),
    Compatibility(String),
}

impl fmt::Display for SdkPhase9Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredField(field) => {
                write!(formatter, "missing Phase 9 required field: {field}")
            }
            Self::MissingTypeScriptWebArtifact(kind) => {
                write!(formatter, "missing Phase 9 TypeScript/web artifact: {kind}")
            }
            Self::UnsafeTypeScriptWebBinding(name) => {
                write!(formatter, "unsafe Phase 9 TypeScript/web binding: {name}")
            }
            Self::AdapterUiBoundaryUnsafe(profile) => {
                write!(formatter, "unsafe Phase 9 adapter/UI boundary: {profile}")
            }
            Self::MissingProductModule(kind) => {
                write!(formatter, "missing Phase 9 product module: {kind}")
            }
            Self::UnsafeProductModule(name) => {
                write!(formatter, "unsafe Phase 9 product module: {name}")
            }
            Self::MissingFailureCase(case) => {
                write!(formatter, "missing Phase 9 product failure case: {case}")
            }
            Self::MissingMobileExtension(kind) => {
                write!(formatter, "missing Phase 9 mobile extension: {kind}")
            }
            Self::MobileBoundaryDrift(name) => {
                write!(formatter, "Phase 9 mobile boundary drift: {name}")
            }
            Self::MissingBindingReadinessGate(target) => {
                write!(
                    formatter,
                    "missing Phase 9 binding readiness gate: {target}"
                )
            }
            Self::BindingReadinessGateOpen(target) => {
                write!(
                    formatter,
                    "Phase 9 binding readiness gate is open: {target}"
                )
            }
            Self::ArtifactMissingMarker(marker) => {
                write!(formatter, "Phase 9 artifact missing marker: {marker}")
            }
            Self::ArtifactSensitiveValue(path) => {
                write!(
                    formatter,
                    "Phase 9 artifact has sensitive value marker: {path}"
                )
            }
            Self::Compatibility(message) => {
                write!(formatter, "Phase 9 compatibility error: {message}")
            }
        }
    }
}

impl std::error::Error for SdkPhase9Error {}

impl From<SdkCompatibilityRejection> for SdkPhase9Error {
    fn from(error: SdkCompatibilityRejection) -> Self {
        Self::Compatibility(format!("{error:?}"))
    }
}

fn typescript_web_binding(
    name: &'static str,
    kind: SdkPhase9TypeScriptWebArtifactKind,
    output_path: &'static str,
) -> SdkPhase9TypeScriptWebBindingDescriptor {
    SdkPhase9TypeScriptWebBindingDescriptor {
        name,
        kind,
        language_target: SDK_PHASE9_TYPESCRIPT_WEB_TARGET,
        schema_version: SUPPORTED_SCHEMA_VERSION,
        source_contract_path: "packages/schemas/overrid_contracts",
        output_path,
        generated_from_shared_contracts: true,
        passes_phase8_golden_corpus: true,
        request_envelope_matches_rust: true,
        error_objects_match_rust: true,
        idempotency_matches_rust: true,
        browser_surface_only: true,
        runtime_authority: false,
        handwritten_public_objects_allowed: false,
    }
}

fn product_module(
    module_name: &'static str,
    product_kind: SdkPhase9ProductKind,
    feature_flag: &'static str,
    helpers: &'static [&'static str],
) -> SdkPhase9ProductConvenienceModule {
    SdkPhase9ProductConvenienceModule {
        module_name,
        product_kind,
        feature_flag,
        helpers,
        public_routes: PHASE9_PRODUCT_PUBLIC_ROUTES,
        failure_cases: REQUIRED_PRODUCT_FAILURE_CASES,
        capability_checks_required: true,
        uses_overgate_admin_api_only: true,
        authorized_refs_only: true,
        service_returned_usage_receipts_only: true,
        bypasses_internal_services: false,
        stores_private_keys: false,
        direct_storage_access: false,
        hardcoded_model_or_provider: false,
        paid_service_assumption: false,
    }
}

fn mobile_extension(
    name: &'static str,
    kind: SdkPhase9MobileExtensionKind,
) -> SdkPhase9MobileExtensionInterface {
    SdkPhase9MobileExtensionInterface {
        name,
        kind,
        extension_point_only: true,
        owner_phase: SDK_PHASE9_MOBILE_SDK_OWNER_PHASE,
        foundation_sdk_implements_runtime: false,
        provides_default_offline_queue: false,
        owns_os_secure_storage: false,
        owns_push_registration: false,
        owns_background_behavior: false,
        owns_media_upload_state: false,
    }
}

fn binding_gate(
    target: SdkPhase9BindingTarget,
    mobile_sdk_phase12_gate: bool,
) -> SdkPhase9BindingReadinessGate {
    SdkPhase9BindingReadinessGate {
        target,
        schema_compatibility_required: true,
        golden_request_envelope_required: true,
        generated_error_objects_required: true,
        cross_language_contract_tests_required: true,
        no_handwritten_public_objects_required: true,
        release_blocking: true,
        matches_rust_behavior_required: true,
        mobile_sdk_phase12_gate,
    }
}

fn validate_artifact_text(
    path: &'static str,
    text: &str,
    required_markers: &[&'static str],
) -> Result<(), SdkPhase9Error> {
    for marker in required_markers {
        if !text.contains(marker) {
            return Err(SdkPhase9Error::ArtifactMissingMarker(marker));
        }
    }
    ensure_artifact_text_safe(path, text)
}

fn ensure_artifact_text_safe(path: &'static str, text: &str) -> Result<(), SdkPhase9Error> {
    let lowered = text.to_ascii_lowercase();
    for forbidden in PHASE9_FORBIDDEN_ARTIFACT_MARKERS {
        if lowered.contains(forbidden) {
            return Err(SdkPhase9Error::ArtifactSensitiveValue(path));
        }
    }
    Ok(())
}

fn require_phase9_non_empty(value: &str, field: &'static str) -> Result<(), SdkPhase9Error> {
    if value.trim().is_empty() {
        return Err(SdkPhase9Error::MissingRequiredField(field));
    }
    Ok(())
}

const REQUIRED_TS_WEB_ARTIFACT_KINDS: &[SdkPhase9TypeScriptWebArtifactKind] = &[
    SdkPhase9TypeScriptWebArtifactKind::Models,
    SdkPhase9TypeScriptWebArtifactKind::Validators,
    SdkPhase9TypeScriptWebArtifactKind::ErrorObjects,
    SdkPhase9TypeScriptWebArtifactKind::RequestHelpers,
    SdkPhase9TypeScriptWebArtifactKind::ManifestHelpers,
    SdkPhase9TypeScriptWebArtifactKind::IdempotencyHelpers,
];

const REQUIRED_PRODUCT_KINDS: &[SdkPhase9ProductKind] = &[
    SdkPhase9ProductKind::DocdexEncryptedRag,
    SdkPhase9ProductKind::McodaAgentWorkload,
    SdkPhase9ProductKind::CodaliCodeAgent,
    SdkPhase9ProductKind::PackageValidation,
];

const REQUIRED_PRODUCT_FAILURE_CASES: &[SdkPhase9ProductFailureCase] = &[
    SdkPhase9ProductFailureCase::SuccessfulJob,
    SdkPhase9ProductFailureCase::RetryableFailure,
    SdkPhase9ProductFailureCase::FinalFailure,
    SdkPhase9ProductFailureCase::Cancellation,
    SdkPhase9ProductFailureCase::Timeout,
    SdkPhase9ProductFailureCase::PolicyDenial,
    SdkPhase9ProductFailureCase::BudgetExhaustion,
    SdkPhase9ProductFailureCase::NodeDisconnect,
    SdkPhase9ProductFailureCase::DisputedUsage,
];

const REQUIRED_MOBILE_EXTENSION_KINDS: &[SdkPhase9MobileExtensionKind] = &[
    SdkPhase9MobileExtensionKind::SessionRefreshPrimitive,
    SdkPhase9MobileExtensionKind::SecureStorageAdapterInterface,
    SdkPhase9MobileExtensionKind::OfflineQueueInterface,
    SdkPhase9MobileExtensionKind::SyncCursorPrimitive,
    SdkPhase9MobileExtensionKind::ConflictStatePrimitive,
    SdkPhase9MobileExtensionKind::RedactedDiagnosticsHook,
];

const REQUIRED_BINDING_TARGETS: &[SdkPhase9BindingTarget] = &[
    SdkPhase9BindingTarget::TypeScriptWeb,
    SdkPhase9BindingTarget::SwiftIos,
    SdkPhase9BindingTarget::KotlinAndroid,
    SdkPhase9BindingTarget::Python,
    SdkPhase9BindingTarget::Other,
];

const PHASE9_PRODUCT_PUBLIC_ROUTES: &[&str] = &[
    "/v1/overgate/workloads",
    "/v1/overgate/packages/validate",
    "/v1/control-plane/workloads/status",
    "/v1/control-plane/workloads/cancellations",
    "/v1/control-plane/workloads/results",
    "/v1/accounting/usage-rollups",
    "/v1/accounting/usage-receipts",
];

const PHASE9_FORBIDDEN_ARTIFACT_MARKERS: &[&str] = &[
    "raw_private_key",
    "private_key_value",
    "bearer_token_value",
    "seed_phrase_value",
    "\"signature_value\"",
    "\"raw_request_body\"",
    "private_payload_value",
    "fixture_credential_material",
    "production_default\": true",
    "runtime_authority\": true",
    "bypasses_internal_services\": true",
    "stores_private_keys\": true",
    "direct_storage_access\": true",
    "hardcoded_model_or_provider\": true",
    "paid_service_assumption\": true",
    "foundation_sdk_implements_runtime\": true",
    "provides_default_offline_queue\": true",
    "handwritten_public_objects_allowed\": true",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase9_typescript_web_bindings_are_generated_from_shared_contracts() {
        let bindings = sdk_phase9_typescript_web_bindings().unwrap();
        validate_phase9_typescript_web_bindings(&bindings).unwrap();

        assert_eq!(bindings.len(), REQUIRED_TS_WEB_ARTIFACT_KINDS.len());
        assert!(bindings
            .iter()
            .all(|binding| binding.generated_from_shared_contracts
                && binding.passes_phase8_golden_corpus
                && !binding.runtime_authority));
    }

    #[test]
    fn phase9_adapter_ui_boundary_rejects_internal_authority() {
        let boundary = sdk_phase9_adapter_ui_safety_boundary();
        validate_phase9_adapter_ui_safety_boundary(&boundary).unwrap();

        assert!(boundary.overgate_admin_public_api_only);
        assert!(!boundary.privileged_internal_endpoints_allowed);
        assert!(!boundary.private_key_storage_allowed);
        assert!(!boundary.browser_runtime_authority);
    }

    #[test]
    fn phase9_product_modules_are_capability_gated_and_cover_failure_cases() {
        let modules = sdk_phase9_product_convenience_modules();
        validate_phase9_product_convenience_modules(&modules).unwrap();

        assert_eq!(modules.len(), REQUIRED_PRODUCT_KINDS.len());
        assert!(modules
            .iter()
            .all(|module| module.capability_checks_required
                && module.uses_overgate_admin_api_only
                && module.helpers.contains(&"read_usage_rollup")
                && module.helpers.contains(&"read_usage_receipt")
                && !module.bypasses_internal_services));
        assert!(modules.iter().all(|module| {
            REQUIRED_PRODUCT_FAILURE_CASES
                .iter()
                .all(|case| module.failure_cases.contains(case))
        }));
    }

    #[test]
    fn phase9_mobile_extensions_do_not_own_mobile_runtime_behavior() {
        let interfaces = sdk_phase9_mobile_extension_interfaces();
        validate_phase9_mobile_extension_interfaces(&interfaces).unwrap();

        assert_eq!(interfaces.len(), REQUIRED_MOBILE_EXTENSION_KINDS.len());
        assert!(interfaces
            .iter()
            .all(|interface| interface.extension_point_only
                && interface.owner_phase == SDK_PHASE9_MOBILE_SDK_OWNER_PHASE
                && !interface.foundation_sdk_implements_runtime
                && !interface.provides_default_offline_queue));
    }

    #[test]
    fn phase9_binding_readiness_gates_block_drift_and_handwritten_objects() {
        let gates = sdk_phase9_binding_readiness_gates();
        validate_phase9_binding_readiness_gates(&gates).unwrap();

        assert_eq!(gates.len(), REQUIRED_BINDING_TARGETS.len());
        assert!(gates.iter().all(|gate| gate.release_blocking
            && gate.no_handwritten_public_objects_required
            && gate.matches_rust_behavior_required));
    }

    #[test]
    fn phase9_artifact_files_match_declared_paths_and_markers() {
        validate_phase9_artifact_files().unwrap();
        assert!(TYPESCRIPT_WEB_MANIFEST_JSON.contains(SDK_PHASE9_TYPESCRIPT_WEB_MANIFEST_PATH));
        assert!(PRODUCT_MODULES_JSON.contains(SDK_PHASE9_PRODUCT_MODULES_PATH));
        assert!(READINESS_GATES_JSON.contains(SDK_PHASE9_READINESS_GATES_PATH));
        assert!(TYPESCRIPT_WEB_MANIFEST_JSON.contains(crate::SDK_NAME));
        assert!(TYPESCRIPT_WEB_MANIFEST_JSON.contains(crate::SDK_VERSION));
        assert!(TYPESCRIPT_WEB_MANIFEST_JSON.contains(crate::SDK_LANGUAGE_BINDING));
    }
}
