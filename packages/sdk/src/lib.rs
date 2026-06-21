#![forbid(unsafe_code)]

use std::fmt;

pub mod command;
pub mod generated;
pub mod read;

pub use command::*;
pub use generated::*;
use overrid_contracts::{
    cli_contract_set, ensure_supported_schema_version, ContractError, GeneratedContractSet,
    ProfileValidationError, SchemaVersion, SUPPORTED_SCHEMA_VERSION,
};
pub use overrid_contracts::{
    BootstrapAcceptanceRecord, BootstrapCommandFamily, CapabilitySnapshot, CliProfile,
    CommandContext, ConfirmationPolicy, CredentialReference, CredentialReferenceClass,
    EnvironmentClass, ErrorDecodeRecord, ExitCodeClass, FixtureAllowance, IdempotencyRecord,
    ManifestBootstrapRef, RetryClass, RetryTimeoutPolicy, SignedCommandEnvelope, SignerHandoff,
    SyntheticWorkloadPendingState, TraceContext,
};
pub use read::*;

pub const DEFAULT_TIMEOUT_MS: u64 = 10_000;
pub const DEFAULT_MAX_RETRIES: u8 = 2;
pub const SDK_NAME: &str = "overrid-rust-sdk";
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SDK_LANGUAGE_BINDING: &str = "rust";
pub const SDK_CAPABILITY_PROFILE: &str = "phase1-control-plane-thin-client";
pub const SDK_CURRENT_STABLE_MAJOR: u16 = 0;
pub const SDK_PREVIOUS_STABLE_MAJOR: Option<u16> = None;
pub const SDK_UNSUPPORTED_VERSION_REASON_CODE: &str = "unsupported_sdk_version";
pub const SDK_UNSUPPORTED_SCHEMA_REASON_CODE: &str = "schema_version_unsupported";
pub const SDK_CAPABILITY_UNAVAILABLE_REASON_CODE: &str = "sdk_capability_unavailable";
pub const SDK_PHASE2_CAPABILITY_PROFILE: &str = "phase2-contract-intake-local-data-model";
pub const SDK_PHASE2_GENERATED_OUTPUT_PATH: &str = "packages/sdk/src/lib.rs";
pub const SDK_CONTRACT_SCHEMA_PATH: &str =
    "packages/schemas/overrid_contracts/v0/cli_command.schema.json";
pub const SDK_CONTRACT_CODEGEN_MANIFEST_PATH: &str =
    "packages/schemas/overrid_contracts/codegen_manifest.json";
pub const SDK_CONTRACT_RUST_PROJECTION_PATH: &str = "packages/schemas/overrid_contracts/src/lib.rs";
pub const SDK_DEPRECATION_BEHAVIOR: &str =
    "support current stable major and previous stable major when present";
pub const SDK_UPGRADE_GUIDANCE: &str =
    "read Overgate capability profiles before using optional SDK helpers";
pub const SDK_EMERGENCY_BREAK_POLICY: &str =
    "security-critical breaks return stable unsupported reason codes";
pub const SDK_SUPPORTED_SCHEMA_VERSIONS: &[&str] = &[SUPPORTED_SCHEMA_VERSION];
pub const SDK_DEFAULT_TRACE_POLICY: &str = "generate_if_missing";
pub const SDK_REDACTION_DEFAULTS_PROFILE: &str = "redact_payloads_signatures_and_secret_refs";
pub const SDK_RELEASE_CHECKLIST_ITEMS: &[&str] = &[
    "schema versions named",
    "service capability profile named",
    "deprecation behavior documented",
    "upgrade guidance documented",
    "security-critical emergency break handling documented",
    "Rust-first binding is the first release target",
    "TypeScript/web bindings remain generated second target after schema stability",
    "credential-provider-only signing preserved",
    "bounded idempotency retention documented",
    "Mobile SDK boundary remains separate",
    "current-plus-previous stable major compatibility checked",
    "unsupported_sdk_version returned for unsafe SDK majors",
    "schema_version_unsupported returned for unsafe schema versions",
];

const PRIVATE_SERVICE_TARGETS: &[&str] = &[
    "overbase",
    "overstore",
    "overvault",
    "overqueue",
    "overwatch",
    "seal-ledger",
    "seal_ledger",
    "node-agent",
    "node_agent",
    "overtenant",
    "overpass",
    "overkey",
    "overregistry",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OvergateEndpoint {
    raw: String,
    environment: EnvironmentClass,
}

impl OvergateEndpoint {
    pub fn parse(raw: impl Into<String>, environment: EnvironmentClass) -> Result<Self, SdkError> {
        let raw = raw.into();
        validate_overgate_target(&raw)?;
        Ok(Self { raw, environment })
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn environment(&self) -> EnvironmentClass {
        self.environment
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientConfig {
    pub endpoint: OvergateEndpoint,
    pub timeout_ms: u64,
    pub max_retries: u8,
    pub schema_version: String,
}

impl ClientConfig {
    pub fn local_overgate(raw_endpoint: impl Into<String>) -> Result<Self, SdkError> {
        Ok(Self {
            endpoint: OvergateEndpoint::parse(raw_endpoint, EnvironmentClass::Local)?,
            timeout_ms: DEFAULT_TIMEOUT_MS,
            max_retries: DEFAULT_MAX_RETRIES,
            schema_version: SUPPORTED_SCHEMA_VERSION.to_owned(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkTracePolicy {
    GenerateIfMissing,
    RequireCallerTrace,
    Inherit,
}

impl SdkTracePolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GenerateIfMissing => "generate_if_missing",
            Self::RequireCallerTrace => "require_caller_trace",
            Self::Inherit => "inherit",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkRedactionDefaults {
    pub redact_payloads: bool,
    pub redact_signatures: bool,
    pub redact_secret_refs: bool,
    pub profile: &'static str,
}

impl SdkRedactionDefaults {
    pub fn phase3_default() -> Self {
        Self {
            redact_payloads: true,
            redact_signatures: true,
            redact_secret_refs: true,
            profile: SDK_REDACTION_DEFAULTS_PROFILE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkCredentialProviderRef {
    pub provider_id: String,
    pub credential_id: String,
    pub redaction_class: &'static str,
    pub production_capable: bool,
    pub stores_private_material: bool,
}

impl SdkCredentialProviderRef {
    pub fn from_config(config: &SdkConfigRecord) -> Result<Self, SdkError> {
        let credential = &config.credential_ref;
        if credential.revoked {
            return Err(SdkError::Credential(
                ProfileValidationError::CredentialRevoked,
            ));
        }
        if credential.expired {
            return Err(SdkError::Credential(
                ProfileValidationError::CredentialExpired,
            ));
        }
        Ok(Self {
            provider_id: format!("provider:{}", credential.credential_class.as_str()),
            credential_id: credential.credential_id.clone(),
            redaction_class: credential.redaction_class,
            production_capable: !matches!(
                credential.credential_class,
                CredentialReferenceClass::Fixture
            ),
            stores_private_material: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfiguredSdkClient {
    pub client: OverridSdkClient,
    pub config: SdkConfigRecord,
    pub trace_policy: SdkTracePolicy,
    pub redaction_defaults: SdkRedactionDefaults,
    pub credential_provider: SdkCredentialProviderRef,
}

pub fn configure_client(config: SdkConfigRecord) -> Result<ConfiguredSdkClient, SdkError> {
    if matches!(
        config.environment,
        EnvironmentClass::Local | EnvironmentClass::Ci
    ) && !config.live_endpoint_confirmed
        && !is_loopback_overgate_endpoint(config.endpoint.raw())
    {
        return Err(SdkError::LiveEndpointConfirmationRequired {
            environment: config.environment.as_str(),
        });
    }

    let client_config = ClientConfig {
        endpoint: config.endpoint.clone(),
        timeout_ms: config.timeout_policy.timeout_ms,
        max_retries: config.timeout_policy.max_retries,
        schema_version: config.schema_version.raw().to_owned(),
    };
    let client = OverridSdkClient::new(client_config)?;
    let credential_provider = SdkCredentialProviderRef::from_config(&config)?;

    Ok(ConfiguredSdkClient {
        client,
        config,
        trace_policy: SdkTracePolicy::GenerateIfMissing,
        redaction_defaults: SdkRedactionDefaults::phase3_default(),
        credential_provider,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkCompatibilityMetadata {
    pub sdk_name: &'static str,
    pub sdk_version: &'static str,
    pub current_stable_major: u16,
    pub previous_stable_major: Option<u16>,
    pub language_binding: &'static str,
    pub supported_schema_versions: &'static [&'static str],
    pub service_capability_profile: &'static str,
    pub deprecation_behavior: &'static str,
    pub upgrade_guidance: &'static str,
    pub emergency_break_policy: &'static str,
    pub unsupported_sdk_version_reason: &'static str,
    pub unsupported_schema_version_reason: &'static str,
}

pub fn sdk_compatibility_metadata() -> SdkCompatibilityMetadata {
    SdkCompatibilityMetadata {
        sdk_name: SDK_NAME,
        sdk_version: SDK_VERSION,
        current_stable_major: SDK_CURRENT_STABLE_MAJOR,
        previous_stable_major: SDK_PREVIOUS_STABLE_MAJOR,
        language_binding: SDK_LANGUAGE_BINDING,
        supported_schema_versions: SDK_SUPPORTED_SCHEMA_VERSIONS,
        service_capability_profile: SDK_CAPABILITY_PROFILE,
        deprecation_behavior: SDK_DEPRECATION_BEHAVIOR,
        upgrade_guidance: SDK_UPGRADE_GUIDANCE,
        emergency_break_policy: SDK_EMERGENCY_BREAK_POLICY,
        unsupported_sdk_version_reason: SDK_UNSUPPORTED_VERSION_REASON_CODE,
        unsupported_schema_version_reason: SDK_UNSUPPORTED_SCHEMA_REASON_CODE,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkReleaseChecklist {
    pub metadata: SdkCompatibilityMetadata,
    pub required_items: &'static [&'static str],
}

pub fn sdk_release_checklist() -> SdkReleaseChecklist {
    SdkReleaseChecklist {
        metadata: sdk_compatibility_metadata(),
        required_items: SDK_RELEASE_CHECKLIST_ITEMS,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkVersionReport {
    pub sdk_name: &'static str,
    pub sdk_version: &'static str,
    pub schema_set: &'static [&'static str],
    pub generated_contract_revision: &'static str,
    pub supported_feature_flags: Vec<&'static str>,
    pub language_binding: &'static str,
    pub service_capability_profile: &'static str,
}

pub fn sdk_version_report() -> SdkVersionReport {
    SdkVersionReport {
        sdk_name: SDK_NAME,
        sdk_version: SDK_VERSION,
        schema_set: SDK_PHASE3_SCHEMA_SET,
        generated_contract_revision: SDK_PHASE3_GENERATED_CONTRACT_REVISION,
        supported_feature_flags: vec![
            SdkFeatureFlag::CommandEnvelopes.as_str(),
            SdkFeatureFlag::SigningMetadata.as_str(),
            SdkFeatureFlag::IdempotencyCache.as_str(),
            SdkFeatureFlag::ErrorDecoding.as_str(),
            SdkFeatureFlag::CapabilityNegotiation.as_str(),
        ],
        language_binding: SDK_LANGUAGE_BINDING,
        service_capability_profile: SDK_PHASE3_CAPABILITY_PROFILE,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkContractSourceKind {
    JsonSchema,
    CodegenManifest,
    RustProjection,
    DocsSpec,
    Sds,
    BuildPlan,
}

impl SdkContractSourceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::JsonSchema => "json_schema",
            Self::CodegenManifest => "codegen_manifest",
            Self::RustProjection => "rust_projection",
            Self::DocsSpec => "docs_spec",
            Self::Sds => "sds",
            Self::BuildPlan => "build_plan",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkContractIntakeEntry {
    pub object_name: &'static str,
    pub source_contract_name: &'static str,
    pub source_path: &'static str,
    pub source_kind: SdkContractSourceKind,
    pub owning_phase: &'static str,
    pub generated_output_path: &'static str,
    pub schema_version: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkContractIntakeManifest {
    pub contract_set: GeneratedContractSet,
    pub entries: Vec<SdkContractIntakeEntry>,
    pub generated_output_path: &'static str,
    pub codegen_manifest_path: &'static str,
    pub rust_projection_path: &'static str,
    pub freshness_policy: &'static str,
    pub version_metadata: SdkCompatibilityMetadata,
}

pub fn sdk_contract_intake_manifest() -> SdkContractIntakeManifest {
    SdkContractIntakeManifest {
        contract_set: cli_contract_set(),
        generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
        codegen_manifest_path: SDK_CONTRACT_CODEGEN_MANIFEST_PATH,
        rust_projection_path: SDK_CONTRACT_RUST_PROJECTION_PATH,
        freshness_policy: "schema:check and SDK generation fail when generated output is newer than source contracts",
        version_metadata: sdk_compatibility_metadata(),
        entries: vec![
            SdkContractIntakeEntry {
                object_name: "sdk_config",
                source_contract_name: "cli_profile",
                source_path: SDK_CONTRACT_SCHEMA_PATH,
                source_kind: SdkContractSourceKind::JsonSchema,
                owning_phase: "phase_2_contract_intake_and_local_client_data_model",
                generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
                schema_version: SUPPORTED_SCHEMA_VERSION,
            },
            SdkContractIntakeEntry {
                object_name: "credential_ref",
                source_contract_name: "credential_reference",
                source_path: SDK_CONTRACT_SCHEMA_PATH,
                source_kind: SdkContractSourceKind::JsonSchema,
                owning_phase: "phase_2_contract_intake_and_local_client_data_model",
                generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
                schema_version: SUPPORTED_SCHEMA_VERSION,
            },
            SdkContractIntakeEntry {
                object_name: "request_context",
                source_contract_name: "trace_context",
                source_path: SDK_CONTRACT_SCHEMA_PATH,
                source_kind: SdkContractSourceKind::JsonSchema,
                owning_phase: "phase_2_contract_intake_and_local_client_data_model",
                generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
                schema_version: SUPPORTED_SCHEMA_VERSION,
            },
            SdkContractIntakeEntry {
                object_name: "signed_request",
                source_contract_name: "signed_command_envelope",
                source_path: SDK_CONTRACT_SCHEMA_PATH,
                source_kind: SdkContractSourceKind::JsonSchema,
                owning_phase: "phase_2_contract_intake_and_local_client_data_model",
                generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
                schema_version: SUPPORTED_SCHEMA_VERSION,
            },
            SdkContractIntakeEntry {
                object_name: "idempotency_entry",
                source_contract_name: "local_idempotency_cache_record",
                source_path: SDK_CONTRACT_SCHEMA_PATH,
                source_kind: SdkContractSourceKind::JsonSchema,
                owning_phase: "phase_2_contract_intake_and_local_client_data_model",
                generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
                schema_version: SUPPORTED_SCHEMA_VERSION,
            },
            SdkContractIntakeEntry {
                object_name: "overrid_error",
                source_contract_name: "api_error",
                source_path: SDK_CONTRACT_SCHEMA_PATH,
                source_kind: SdkContractSourceKind::JsonSchema,
                owning_phase: "phase_2_contract_intake_and_local_client_data_model",
                generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
                schema_version: SUPPORTED_SCHEMA_VERSION,
            },
            SdkContractIntakeEntry {
                object_name: "service_capability_profile",
                source_contract_name: "capability_snapshot",
                source_path: SDK_CONTRACT_SCHEMA_PATH,
                source_kind: SdkContractSourceKind::JsonSchema,
                owning_phase: "phase_2_contract_intake_and_local_client_data_model",
                generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
                schema_version: SUPPORTED_SCHEMA_VERSION,
            },
            SdkContractIntakeEntry {
                object_name: "reason_codes_and_events",
                source_contract_name: "api_error",
                source_path: "docs/specs/reason_codes_and_events.md",
                source_kind: SdkContractSourceKind::DocsSpec,
                owning_phase: "phase_2_contract_intake_and_local_client_data_model",
                generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
                schema_version: SUPPORTED_SCHEMA_VERSION,
            },
            SdkContractIntakeEntry {
                object_name: "contract_authority",
                source_contract_name: "codegen_manifest",
                source_path: "docs/specs/contract_authority.md",
                source_kind: SdkContractSourceKind::DocsSpec,
                owning_phase: "phase_2_contract_intake_and_local_client_data_model",
                generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
                schema_version: SUPPORTED_SCHEMA_VERSION,
            },
            SdkContractIntakeEntry {
                object_name: "sdk_data_model",
                source_contract_name: "SDK SDS Data Model",
                source_path: "docs/sds/foundation/sdk.md",
                source_kind: SdkContractSourceKind::Sds,
                owning_phase: "phase_2_contract_intake_and_local_client_data_model",
                generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
                schema_version: SUPPORTED_SCHEMA_VERSION,
            },
            SdkContractIntakeEntry {
                object_name: "phase_2_build_plan",
                source_contract_name: "Phase 2: Contract Intake And Local Client Data Model",
                source_path: "docs/build_plan/sub_build_plan_006_sdk.md",
                source_kind: SdkContractSourceKind::BuildPlan,
                owning_phase: "phase_2_contract_intake_and_local_client_data_model",
                generated_output_path: SDK_PHASE2_GENERATED_OUTPUT_PATH,
                schema_version: SUPPORTED_SCHEMA_VERSION,
            },
        ],
    }
}

pub fn validate_contract_intake_manifest(
    manifest: &SdkContractIntakeManifest,
) -> Result<(), SdkError> {
    if manifest.generated_output_path != SDK_PHASE2_GENERATED_OUTPUT_PATH {
        return Err(SdkError::MissingRequiredField("generated output path"));
    }
    if manifest.codegen_manifest_path != SDK_CONTRACT_CODEGEN_MANIFEST_PATH {
        return Err(SdkError::MissingRequiredField("codegen manifest path"));
    }
    if manifest.rust_projection_path != SDK_CONTRACT_RUST_PROJECTION_PATH {
        return Err(SdkError::MissingRequiredField("rust projection path"));
    }

    for entry in &manifest.entries {
        require_sdk_non_empty(entry.object_name, "contract object name")?;
        require_sdk_non_empty(entry.source_contract_name, "source contract name")?;
        require_sdk_non_empty(entry.source_path, "contract source path")?;
        require_sdk_non_empty(entry.owning_phase, "owning phase")?;
        require_sdk_non_empty(entry.generated_output_path, "generated output path")?;
        if entry.generated_output_path != SDK_PHASE2_GENERATED_OUTPUT_PATH {
            return Err(SdkError::MissingRequiredField(
                "entry generated output path",
            ));
        }
        check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, entry.schema_version)?;
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkCompatibilityRejection {
    UnsupportedSdkVersion {
        provided_major: u16,
        current_stable_major: u16,
        previous_stable_major: Option<u16>,
        reason_code: &'static str,
    },
    UnsupportedSchemaVersion {
        provided: String,
        supported: &'static str,
        reason_code: &'static str,
    },
}

impl SdkCompatibilityRejection {
    pub fn reason_code(&self) -> &'static str {
        match self {
            Self::UnsupportedSdkVersion { reason_code, .. } => reason_code,
            Self::UnsupportedSchemaVersion { reason_code, .. } => reason_code,
        }
    }
}

pub fn check_sdk_compatibility(
    sdk_major: u16,
    schema_version: &str,
) -> Result<SchemaVersion, SdkCompatibilityRejection> {
    let metadata = sdk_compatibility_metadata();
    let sdk_major_supported = sdk_major == metadata.current_stable_major
        || metadata.previous_stable_major == Some(sdk_major);

    if !sdk_major_supported {
        return Err(SdkCompatibilityRejection::UnsupportedSdkVersion {
            provided_major: sdk_major,
            current_stable_major: metadata.current_stable_major,
            previous_stable_major: metadata.previous_stable_major,
            reason_code: SDK_UNSUPPORTED_VERSION_REASON_CODE,
        });
    }

    let parsed = ensure_supported_schema_version(schema_version).map_err(|_| {
        SdkCompatibilityRejection::UnsupportedSchemaVersion {
            provided: schema_version.to_owned(),
            supported: SUPPORTED_SCHEMA_VERSION,
            reason_code: SDK_UNSUPPORTED_SCHEMA_REASON_CODE,
        }
    })?;

    if !metadata
        .supported_schema_versions
        .iter()
        .any(|supported| *supported == parsed.raw())
    {
        return Err(SdkCompatibilityRejection::UnsupportedSchemaVersion {
            provided: schema_version.to_owned(),
            supported: SUPPORTED_SCHEMA_VERSION,
            reason_code: SDK_UNSUPPORTED_SCHEMA_REASON_CODE,
        });
    }

    Ok(parsed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkFeatureFlag {
    CommandEnvelopes,
    SigningMetadata,
    IdempotencyCache,
    ErrorDecoding,
    CapabilityNegotiation,
    TestFixtures,
}

impl SdkFeatureFlag {
    pub fn parse(raw: &str) -> Result<Self, SdkError> {
        match raw {
            "command_envelopes" => Ok(Self::CommandEnvelopes),
            "signing_metadata" => Ok(Self::SigningMetadata),
            "idempotency_cache" => Ok(Self::IdempotencyCache),
            "error_decoding" => Ok(Self::ErrorDecoding),
            "capability_negotiation" => Ok(Self::CapabilityNegotiation),
            "test_fixtures" => Ok(Self::TestFixtures),
            other => Err(SdkError::UnknownFeatureFlag(other.to_owned())),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CommandEnvelopes => "command_envelopes",
            Self::SigningMetadata => "signing_metadata",
            Self::IdempotencyCache => "idempotency_cache",
            Self::ErrorDecoding => "error_decoding",
            Self::CapabilityNegotiation => "capability_negotiation",
            Self::TestFixtures => "test_fixtures",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkServiceCapabilityProfile {
    pub profile_name: String,
    pub supported_schema_versions: Vec<String>,
    pub supported_sdk_majors: Vec<u16>,
    pub signing: bool,
    pub idempotency: bool,
    pub dry_run: bool,
    pub accounting: bool,
}

impl SdkServiceCapabilityProfile {
    pub fn phase2_local() -> Self {
        Self {
            profile_name: SDK_PHASE2_CAPABILITY_PROFILE.to_owned(),
            supported_schema_versions: SDK_SUPPORTED_SCHEMA_VERSIONS
                .iter()
                .map(|version| (*version).to_owned())
                .collect(),
            supported_sdk_majors: vec![SDK_CURRENT_STABLE_MAJOR],
            signing: true,
            idempotency: true,
            dry_run: false,
            accounting: false,
        }
    }

    fn supports_schema(&self, schema_version: &str) -> bool {
        self.supported_schema_versions
            .iter()
            .any(|version| version == schema_version)
    }

    fn supports_sdk_major(&self, sdk_major: u16) -> bool {
        self.supported_sdk_majors.contains(&sdk_major)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkConfigInput {
    pub environment: Option<EnvironmentClass>,
    pub base_url: String,
    pub timeout_ms: Option<u64>,
    pub max_retries: Option<u8>,
    pub feature_flags: Vec<String>,
    pub default_tenant_id: Option<String>,
    pub client_identity_ref: String,
    pub credential_ref: CredentialReference,
    pub service_capability_profile: SdkServiceCapabilityProfile,
    pub live_endpoint_confirmed: bool,
    pub test_fixtures_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkConfigRecord {
    pub endpoint: OvergateEndpoint,
    pub environment: EnvironmentClass,
    pub timeout_policy: RetryTimeoutPolicy,
    pub feature_flags: Vec<SdkFeatureFlag>,
    pub default_tenant_id: Option<String>,
    pub client_identity_ref: String,
    pub credential_ref: SdkCredentialReferenceRecord,
    pub service_capability_profile: SdkServiceCapabilityProfile,
    pub live_endpoint_confirmed: bool,
    pub test_fixtures_enabled: bool,
    pub schema_version: SchemaVersion,
}

impl SdkConfigRecord {
    pub fn from_input(input: SdkConfigInput) -> Result<Self, SdkError> {
        let environment = input.environment.ok_or(SdkError::MissingEnvironment)?;
        if input.client_identity_ref.trim().is_empty() {
            return Err(SdkError::MissingRequiredField("client identity reference"));
        }
        if input
            .default_tenant_id
            .as_ref()
            .is_some_and(|tenant| tenant.trim().is_empty())
        {
            return Err(SdkError::MissingRequiredField("default tenant"));
        }

        let endpoint = OvergateEndpoint::parse(input.base_url, environment)?;
        let feature_flags = input
            .feature_flags
            .iter()
            .map(|flag| SdkFeatureFlag::parse(flag))
            .collect::<Result<Vec<_>, _>>()?;

        if matches!(
            environment,
            EnvironmentClass::Seed | EnvironmentClass::ProductionLike
        ) && !input.live_endpoint_confirmed
        {
            return Err(SdkError::LiveEndpointConfirmationRequired {
                environment: environment.as_str(),
            });
        }

        let test_fixtures_requested = input.test_fixtures_enabled
            || feature_flags
                .iter()
                .any(|flag| *flag == SdkFeatureFlag::TestFixtures);
        if matches!(environment, EnvironmentClass::ProductionLike) && test_fixtures_requested {
            return Err(SdkError::FixtureInProduction);
        }

        let schema_version =
            check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, SUPPORTED_SCHEMA_VERSION)?;
        if !input
            .service_capability_profile
            .supports_schema(schema_version.raw())
        {
            return Err(SdkError::CapabilityUnavailable {
                helper: "sdk_config",
                reason_code: SDK_CAPABILITY_UNAVAILABLE_REASON_CODE,
            });
        }

        Ok(Self {
            endpoint,
            environment,
            timeout_policy: retry_timeout_policy(input.max_retries, input.timeout_ms),
            feature_flags,
            default_tenant_id: input.default_tenant_id,
            client_identity_ref: input.client_identity_ref,
            credential_ref: SdkCredentialReferenceRecord::from_credential(&input.credential_ref)?,
            service_capability_profile: input.service_capability_profile,
            live_endpoint_confirmed: input.live_endpoint_confirmed,
            test_fixtures_enabled: input.test_fixtures_enabled,
            schema_version,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkCredentialReferenceRecord {
    pub credential_id: String,
    pub credential_class: CredentialReferenceClass,
    pub namespace: String,
    pub key_id: String,
    pub redaction_class: &'static str,
    pub stores_private_material: bool,
    pub stores_bearer_token: bool,
    pub revoked: bool,
    pub expired: bool,
}

impl SdkCredentialReferenceRecord {
    pub fn from_credential(credential: &CredentialReference) -> Result<Self, SdkError> {
        require_sdk_non_empty(&credential.reference_id, "credential reference")?;
        require_sdk_non_empty(&credential.namespace, "credential namespace")?;
        require_sdk_non_empty(&credential.key_id, "credential key id")?;

        for (field, value) in [
            ("credential reference", credential.reference_id.as_str()),
            ("credential namespace", credential.namespace.as_str()),
            ("credential key id", credential.key_id.as_str()),
        ] {
            reject_secret_like_value(field, value)?;
        }

        Ok(Self {
            credential_id: credential.reference_id.clone(),
            credential_class: credential.class,
            namespace: credential.namespace.clone(),
            key_id: credential.key_id.clone(),
            redaction_class: match credential.class {
                CredentialReferenceClass::Fixture => "test_fixture_ref",
                CredentialReferenceClass::CiReference => "ci_secret_ref",
                _ => "credential_ref",
            },
            stores_private_material: false,
            stores_bearer_token: false,
            revoked: credential.revoked,
            expired: credential.expired,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkRequestContextRecord {
    pub actor_id: String,
    pub tenant_id: String,
    pub trace_context: TraceContext,
    pub idempotency: IdempotencyRecord,
    pub command_type: String,
    pub caller_app_id: Option<String>,
    pub timestamp_ms: u64,
    pub schema_version: SchemaVersion,
}

impl SdkRequestContextRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        actor_id: impl Into<String>,
        tenant_id: impl Into<String>,
        trace_id: impl Into<String>,
        idempotency_key: impl Into<String>,
        command_type: impl Into<String>,
        caller_app_id: Option<String>,
        timestamp_ms: u64,
        schema_version: &str,
    ) -> Result<Self, SdkError> {
        let actor_id = actor_id.into();
        let tenant_id = tenant_id.into();
        let command_type = command_type.into();
        require_sdk_non_empty(&actor_id, "actor id")?;
        require_sdk_non_empty(&tenant_id, "tenant id")?;
        require_sdk_non_empty(&command_type, "command type")?;
        if timestamp_ms == 0 {
            return Err(SdkError::MissingRequiredField("timestamp"));
        }

        let schema_version = check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, schema_version)?;
        let trace_context = TraceContext::new(trace_id, schema_version.raw())?;
        let idempotency =
            IdempotencyRecord::new(idempotency_key, command_type.clone(), schema_version.raw())?;

        Ok(Self {
            actor_id,
            tenant_id,
            trace_context,
            idempotency,
            command_type,
            caller_app_id,
            timestamp_ms,
            schema_version,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkSignedRequestRecord {
    pub method: String,
    pub path: String,
    pub body_hash: String,
    pub signature_metadata: String,
    pub credential_id: String,
    pub replay_window_ms: u64,
    pub context: SdkRequestContextRecord,
    pub stores_private_material: bool,
    pub stores_bearer_token: bool,
}

impl SdkSignedRequestRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        context: SdkRequestContextRecord,
        credential_ref: &SdkCredentialReferenceRecord,
        method: impl Into<String>,
        path: impl Into<String>,
        body_hash: impl Into<String>,
        signature_metadata: impl Into<String>,
        replay_window_ms: u64,
    ) -> Result<Self, SdkError> {
        let method = method.into();
        let path = path.into();
        let body_hash = body_hash.into();
        let signature_metadata = signature_metadata.into();
        require_sdk_non_empty(&method, "request method")?;
        require_sdk_non_empty(&path, "request path")?;
        require_sdk_non_empty(&body_hash, "body hash")?;
        require_sdk_non_empty(&signature_metadata, "signature metadata")?;
        if !body_hash.starts_with("hash_") {
            return Err(SdkError::InvalidBodyHash(body_hash));
        }
        if replay_window_ms == 0 {
            return Err(SdkError::MissingRequiredField("replay window"));
        }
        reject_secret_like_value("signature metadata", &signature_metadata)?;

        Ok(Self {
            method,
            path,
            body_hash,
            signature_metadata,
            credential_id: credential_ref.credential_id.clone(),
            replay_window_ms,
            context,
            stores_private_material: false,
            stores_bearer_token: false,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkCommandClass {
    ReadOnly,
    Phase1Mutating,
    LongRunningWorkload,
    SecuritySensitive,
    AccountingReceiptOrDispute,
}

impl SdkCommandClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::Phase1Mutating => "phase1_mutating",
            Self::LongRunningWorkload => "long_running_workload",
            Self::SecuritySensitive => "security_sensitive",
            Self::AccountingReceiptOrDispute => "accounting_receipt_or_dispute",
        }
    }

    pub fn retention_ms(self) -> Option<u64> {
        match self {
            Self::ReadOnly => None,
            Self::Phase1Mutating | Self::SecuritySensitive => Some(24 * 60 * 60 * 1_000),
            Self::LongRunningWorkload => Some(7 * 24 * 60 * 60 * 1_000),
            Self::AccountingReceiptOrDispute => Some(30 * 24 * 60 * 60 * 1_000),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkIdempotencyEntry {
    pub command_class: SdkCommandClass,
    pub idempotency_key: String,
    pub command_type: String,
    pub request_hash: String,
    pub terminal_response_digest: Option<String>,
    pub trace_id: String,
    pub audit_refs: Vec<String>,
    pub retry_class: RetryClass,
    pub correction_fields: Vec<String>,
    pub retention_ms: u64,
    pub contains_raw_payload: bool,
}

impl SdkIdempotencyEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn for_command_class(
        command_class: SdkCommandClass,
        context: &SdkRequestContextRecord,
        request_hash: impl Into<String>,
        terminal_response_digest: Option<String>,
        audit_refs: Vec<String>,
        retry_class: RetryClass,
        correction_fields: Vec<String>,
    ) -> Result<Option<Self>, SdkError> {
        let Some(retention_ms) = command_class.retention_ms() else {
            return Ok(None);
        };
        let request_hash = request_hash.into();
        require_sdk_non_empty(&request_hash, "request hash")?;
        if !request_hash.starts_with("hash_") {
            return Err(SdkError::InvalidBodyHash(request_hash));
        }

        Ok(Some(Self {
            command_class,
            idempotency_key: context.idempotency.key.clone(),
            command_type: context.command_type.clone(),
            request_hash,
            terminal_response_digest,
            trace_id: context.trace_context.trace_id.clone(),
            audit_refs,
            retry_class,
            correction_fields,
            retention_ms,
            contains_raw_payload: false,
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverridErrorRecord {
    pub reason_code: String,
    pub message: String,
    pub trace_id: Option<String>,
    pub audit_refs: Vec<String>,
    pub retryable: bool,
    pub retry_class: RetryClass,
    pub correction_fields: Vec<String>,
    pub dependency_name: Option<String>,
    pub policy_refs: Vec<String>,
    pub schema_version: SchemaVersion,
}

impl OverridErrorRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        reason_code: impl Into<String>,
        message: impl Into<String>,
        trace_id: Option<String>,
        audit_refs: Vec<String>,
        retry_class: RetryClass,
        correction_fields: Vec<String>,
        dependency_name: Option<String>,
        policy_refs: Vec<String>,
        schema_version: &str,
    ) -> Result<Self, SdkError> {
        let reason_code = reason_code.into();
        let message = message.into();
        require_sdk_non_empty(&reason_code, "reason code")?;
        require_sdk_non_empty(&message, "error message")?;
        reject_secret_like_value("error message", &message)?;
        if let Some(dependency_name) = dependency_name.as_deref() {
            reject_secret_like_value("dependency name", dependency_name)?;
        }

        Ok(Self {
            reason_code,
            message,
            trace_id,
            audit_refs,
            retryable: matches!(retry_class, RetryClass::SafeRetry | RetryClass::RetryAfter),
            retry_class,
            correction_fields,
            dependency_name,
            policy_refs,
            schema_version: check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, schema_version)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkOptionalHelper {
    CommandSubmission,
    WorkloadSubmission,
    PolicyDryRun,
    AccountingReaders,
}

impl SdkOptionalHelper {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CommandSubmission => "command_submission",
            Self::WorkloadSubmission => "workload_submission",
            Self::PolicyDryRun => "policy_dry_run",
            Self::AccountingReaders => "accounting_readers",
        }
    }

    fn phase_gate(self) -> &'static str {
        match self {
            Self::PolicyDryRun => "phase_4_trust_policy_verification",
            Self::AccountingReaders => "phase_5_metering_accounting",
            Self::WorkloadSubmission => "phase_3_private_execution_loop",
            Self::CommandSubmission => "phase_1_control_plane_bootstrap",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkCapabilityDecision {
    pub helper: SdkOptionalHelper,
    pub schema_version: SchemaVersion,
    pub sdk_major: u16,
    pub snapshot: CapabilitySnapshot,
}

pub fn negotiate_sdk_capability(
    profile: &SdkServiceCapabilityProfile,
    helper: SdkOptionalHelper,
    schema_version: &str,
    sdk_major: u16,
) -> Result<SdkCapabilityDecision, SdkError> {
    let schema_version = check_sdk_compatibility(sdk_major, schema_version)?;
    let helper_supported = match helper {
        SdkOptionalHelper::CommandSubmission => profile.signing && profile.idempotency,
        SdkOptionalHelper::WorkloadSubmission => profile.signing && profile.idempotency,
        SdkOptionalHelper::PolicyDryRun => profile.dry_run,
        SdkOptionalHelper::AccountingReaders => profile.accounting,
    };

    if !profile.supports_sdk_major(sdk_major)
        || !profile.supports_schema(schema_version.raw())
        || !helper_supported
    {
        return Err(SdkError::CapabilityUnavailable {
            helper: helper.as_str(),
            reason_code: SDK_CAPABILITY_UNAVAILABLE_REASON_CODE,
        });
    }

    Ok(SdkCapabilityDecision {
        helper,
        schema_version,
        sdk_major,
        snapshot: CapabilitySnapshot {
            route: format!("sdk:{}", helper.as_str()),
            available: true,
            phase_gate: helper.phase_gate().to_owned(),
            schema_versions: profile.supported_schema_versions.clone(),
            stale_age_ms: 0,
            fail_closed: true,
        },
    })
}

pub fn retry_timeout_policy(
    max_retries: Option<u8>,
    timeout_ms: Option<u64>,
) -> RetryTimeoutPolicy {
    RetryTimeoutPolicy::bounded(
        max_retries.unwrap_or(DEFAULT_MAX_RETRIES),
        timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS),
    )
}

pub fn decode_phase6_error(
    reason_code: &str,
    _message: &str,
    exit_class: ExitCodeClass,
    retry_class: RetryClass,
) -> ErrorDecodeRecord {
    ErrorDecodeRecord::new(
        error_source_family(exit_class),
        reason_code,
        retry_class,
        exit_class,
        remediation_hint(reason_code, exit_class, retry_class),
    )
}

fn error_source_family(exit_class: ExitCodeClass) -> &'static str {
    match exit_class {
        ExitCodeClass::Success => "success",
        ExitCodeClass::Usage | ExitCodeClass::Config | ExitCodeClass::LocalIo => "cli",
        ExitCodeClass::Credential => "credential",
        ExitCodeClass::Schema => "schema",
        ExitCodeClass::Policy => "policy",
        ExitCodeClass::Phase => "phase",
        ExitCodeClass::Idempotency => "idempotency",
        ExitCodeClass::Transport | ExitCodeClass::Timeout | ExitCodeClass::Platform => "platform",
    }
}

fn remediation_hint(
    reason_code: &str,
    exit_class: ExitCodeClass,
    retry_class: RetryClass,
) -> &'static str {
    match retry_class {
        RetryClass::SafeRetry => "retry with the same idempotency key",
        RetryClass::RetryAfter => {
            "retry after the service-provided wait window with the same idempotency key"
        }
        RetryClass::OperatorReview => "capture the trace id and request operator review",
        RetryClass::NotRetryable => match exit_class {
            ExitCodeClass::Phase => "wait for the owning phase before retrying this route",
            ExitCodeClass::Credential => "fix the credential reference before retrying",
            ExitCodeClass::Schema => "update the CLI or schema pin before retrying",
            ExitCodeClass::Policy => "change the request to satisfy policy before retrying",
            ExitCodeClass::Idempotency => {
                "inspect or reset the local idempotency cache before retrying"
            }
            ExitCodeClass::Timeout => "check status by trace id before deciding whether to retry",
            ExitCodeClass::Usage if reason_code == "missing_reason" => {
                "provide a non-empty --reason for admin-impacting operations"
            }
            ExitCodeClass::Usage => "correct the command arguments before retrying",
            ExitCodeClass::Config => {
                "correct the profile or endpoint configuration before retrying"
            }
            ExitCodeClass::Transport | ExitCodeClass::Platform | ExitCodeClass::LocalIo => {
                "inspect diagnostics and retry only after the local condition is fixed"
            }
            ExitCodeClass::Success => "no remediation required",
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverridSdkClient {
    config: ClientConfig,
    schema_version: SchemaVersion,
}

impl OverridSdkClient {
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        let schema_version =
            check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, &config.schema_version)?;
        Ok(Self {
            config,
            schema_version,
        })
    }

    pub fn endpoint(&self) -> &OvergateEndpoint {
        &self.config.endpoint
    }

    pub fn schema_version(&self) -> &SchemaVersion {
        &self.schema_version
    }

    pub fn compatibility_metadata(&self) -> SdkCompatibilityMetadata {
        sdk_compatibility_metadata()
    }

    pub fn build_request(
        &self,
        operation: impl Into<String>,
        trace_id: impl Into<String>,
    ) -> OvergateRequest {
        OvergateRequest {
            endpoint: self.config.endpoint.raw().to_owned(),
            operation: operation.into(),
            trace_id: trace_id.into(),
            schema_version: self.schema_version.raw().to_owned(),
            timeout_ms: self.config.timeout_ms,
            max_retries: self.config.max_retries,
            headers: vec![
                (
                    "x-overrid-schema-version".to_owned(),
                    self.schema_version.raw().to_owned(),
                ),
                ("x-overrid-target".to_owned(), "overgate".to_owned()),
                ("x-overrid-sdk-name".to_owned(), SDK_NAME.to_owned()),
                ("x-overrid-sdk-version".to_owned(), SDK_VERSION.to_owned()),
                (
                    "x-overrid-sdk-language-binding".to_owned(),
                    SDK_LANGUAGE_BINDING.to_owned(),
                ),
                (
                    "x-overrid-sdk-capability-profile".to_owned(),
                    SDK_PHASE3_CAPABILITY_PROFILE.to_owned(),
                ),
                (
                    "x-overrid-generated-contract-revision".to_owned(),
                    SDK_PHASE3_GENERATED_CONTRACT_REVISION.to_owned(),
                ),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OvergateRequest {
    pub endpoint: String,
    pub operation: String,
    pub trace_id: String,
    pub schema_version: String,
    pub timeout_ms: u64,
    pub max_retries: u8,
    pub headers: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSafetyInput<'a> {
    pub profile: &'a CliProfile,
    pub credential: &'a CredentialReference,
    pub endpoint_override: Option<&'a str>,
    pub explicit_profile: bool,
    pub confirm_profile: bool,
    pub mutating: bool,
    pub admin_impacting: bool,
    pub reason: Option<&'a str>,
    pub command_type: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSafetyDecision {
    pub endpoint: String,
    pub environment: EnvironmentClass,
    pub requires_confirmation: bool,
    pub signer_handoff: Option<SignerHandoff>,
}

pub fn enforce_profile_environment(
    input: CommandSafetyInput<'_>,
) -> Result<CommandSafetyDecision, SdkError> {
    input.profile.validate()?;
    input
        .credential
        .validate_for_profile(input.profile)
        .map_err(SdkError::Credential)?;

    if input.endpoint_override.is_some() && input.profile.environment.locks_endpoint_override() {
        return Err(SdkError::UnsafeEndpointOverride {
            environment: input.profile.environment.as_str(),
        });
    }

    if input.mutating && input.profile.environment.requires_profile_confirmation() {
        if !input.explicit_profile {
            return Err(SdkError::MissingExplicitProfile {
                environment: input.profile.environment.as_str(),
            });
        }
        if !input.confirm_profile {
            return Err(SdkError::MissingProfileConfirmation {
                environment: input.profile.environment.as_str(),
            });
        }
    }

    if input.admin_impacting && input.reason.is_none_or(|reason| reason.trim().is_empty()) {
        return Err(SdkError::MissingReason);
    }

    let signer_handoff = if input.mutating {
        Some(prepare_signer_handoff(
            input.profile,
            input.credential,
            input.command_type,
        )?)
    } else {
        None
    };

    Ok(CommandSafetyDecision {
        endpoint: input
            .endpoint_override
            .unwrap_or(input.profile.endpoint.as_str())
            .to_owned(),
        environment: input.profile.environment,
        requires_confirmation: input.mutating
            && input.profile.environment.requires_profile_confirmation(),
        signer_handoff,
    })
}

pub fn prepare_signer_handoff(
    profile: &CliProfile,
    credential: &CredentialReference,
    command_type: &str,
) -> Result<SignerHandoff, SdkError> {
    SignerHandoff::new(profile, credential, command_type).map_err(SdkError::Credential)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkError {
    UnsupportedScheme(String),
    MissingOvergateTarget(String),
    PrivateServiceTarget(String),
    Contract(ContractError),
    Compatibility(SdkCompatibilityRejection),
    MissingEnvironment,
    MissingRequiredField(&'static str),
    UnknownFeatureFlag(String),
    LiveEndpointConfirmationRequired {
        environment: &'static str,
    },
    FixtureInProduction,
    SecretMaterialRejected {
        field: &'static str,
    },
    InvalidBodyHash(String),
    CapabilityUnavailable {
        helper: &'static str,
        reason_code: &'static str,
    },
    Profile(ProfileValidationError),
    Credential(ProfileValidationError),
    UnsafeEndpointOverride {
        environment: &'static str,
    },
    MissingExplicitProfile {
        environment: &'static str,
    },
    MissingProfileConfirmation {
        environment: &'static str,
    },
    MissingReason,
    IdempotencyConflict {
        key: String,
    },
    TraceMismatch {
        expected: String,
        actual: String,
    },
    InvalidLifecycleTransition {
        from: &'static str,
        to: &'static str,
    },
}

impl fmt::Display for SdkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedScheme(raw) => {
                write!(formatter, "unsupported endpoint scheme for {raw}")
            }
            Self::MissingOvergateTarget(raw) => {
                write!(formatter, "endpoint does not target Overgate: {raw}")
            }
            Self::PrivateServiceTarget(raw) => {
                write!(
                    formatter,
                    "endpoint bypasses Overgate or targets a private service: {raw}"
                )
            }
            Self::Contract(error) => error.fmt(formatter),
            Self::Compatibility(rejection) => match rejection {
                SdkCompatibilityRejection::UnsupportedSdkVersion {
                    provided_major,
                    current_stable_major,
                    previous_stable_major,
                    reason_code,
                } => write!(
                    formatter,
                    "{reason_code}: unsupported SDK major {provided_major}; supported current major is {current_stable_major} and previous supported major is {previous_stable_major:?}"
                ),
                SdkCompatibilityRejection::UnsupportedSchemaVersion {
                    provided,
                    supported,
                    reason_code,
                } => write!(
                    formatter,
                    "{reason_code}: unsupported schema version {provided}; supported version is {supported}"
                ),
            },
            Self::MissingEnvironment => formatter.write_str("sdk_config environment is required"),
            Self::MissingRequiredField(field) => write!(formatter, "{field} is required"),
            Self::UnknownFeatureFlag(flag) => write!(formatter, "unknown SDK feature flag: {flag}"),
            Self::LiveEndpointConfirmationRequired { environment } => write!(
                formatter,
                "live endpoint confirmation is required for {environment} SDK config"
            ),
            Self::FixtureInProduction => {
                formatter.write_str("test fixtures cannot be enabled in production SDK config")
            }
            Self::SecretMaterialRejected { field } => {
                write!(formatter, "{field} contains secret-like material")
            }
            Self::InvalidBodyHash(hash) => write!(formatter, "invalid body/request hash: {hash}"),
            Self::CapabilityUnavailable {
                helper,
                reason_code,
            } => write!(
                formatter,
                "{reason_code}: SDK helper is unavailable before safe network use: {helper}"
            ),
            Self::Profile(error) => error.fmt(formatter),
            Self::Credential(error) => error.fmt(formatter),
            Self::UnsafeEndpointOverride { environment } => write!(
                formatter,
                "endpoint override is not allowed for {environment} profiles"
            ),
            Self::MissingExplicitProfile { environment } => write!(
                formatter,
                "mutating {environment} commands require explicit profile selection"
            ),
            Self::MissingProfileConfirmation { environment } => write!(
                formatter,
                "mutating {environment} commands require --confirm-profile"
            ),
            Self::MissingReason => formatter.write_str("admin-impacting commands require --reason"),
            Self::IdempotencyConflict { key } => write!(
                formatter,
                "duplicate idempotency key has a conflicting request hash: {key}"
            ),
            Self::TraceMismatch { expected, actual } => {
                write!(
                    formatter,
                    "service trace id mismatch: expected {expected}, got {actual}"
                )
            }
            Self::InvalidLifecycleTransition { from, to } => write!(
                formatter,
                "invalid command lifecycle transition without explicit service evidence: {from} -> {to}"
            ),
        }
    }
}

impl std::error::Error for SdkError {}

impl From<ContractError> for SdkError {
    fn from(error: ContractError) -> Self {
        Self::Contract(error)
    }
}

impl From<SdkCompatibilityRejection> for SdkError {
    fn from(error: SdkCompatibilityRejection) -> Self {
        Self::Compatibility(error)
    }
}

impl From<ProfileValidationError> for SdkError {
    fn from(error: ProfileValidationError) -> Self {
        Self::Profile(error)
    }
}

fn require_sdk_non_empty(value: &str, field: &'static str) -> Result<(), SdkError> {
    if value.trim().is_empty() {
        return Err(SdkError::MissingRequiredField(field));
    }
    Ok(())
}

fn reject_secret_like_value(field: &'static str, value: &str) -> Result<(), SdkError> {
    let lower = value.to_ascii_lowercase();
    let contains_secret_like_marker = [
        "-----begin",
        "private_key",
        "raw_secret",
        "seed_phrase",
        "bearer ",
        "bearer_",
        "token=",
        "api_key=",
    ]
    .iter()
    .any(|marker| lower.contains(marker));

    if contains_secret_like_marker {
        return Err(SdkError::SecretMaterialRejected { field });
    }

    Ok(())
}

pub fn validate_overgate_target(raw: &str) -> Result<(), SdkError> {
    let lower = raw.to_ascii_lowercase();
    let Some((scheme, rest)) = lower.split_once("://") else {
        return Err(SdkError::UnsupportedScheme(raw.to_owned()));
    };
    if scheme != "http" && scheme != "https" {
        return Err(SdkError::UnsupportedScheme(raw.to_owned()));
    }

    for target in PRIVATE_SERVICE_TARGETS {
        let dotted = format!("{target}.");
        let path = format!("/{target}");
        if lower.contains(&dotted) || lower.contains(&path) {
            return Err(SdkError::PrivateServiceTarget(raw.to_owned()));
        }
    }

    if !rest.contains("overgate") {
        return Err(SdkError::MissingOvergateTarget(raw.to_owned()));
    }

    Ok(())
}

fn is_loopback_overgate_endpoint(raw: &str) -> bool {
    let lower = raw.to_ascii_lowercase();
    lower.contains("://localhost") || lower.contains("://127.0.0.1") || lower.contains("://[::1]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use overrid_contracts::{
        ConfirmationPolicy, CredentialReferenceClass, FixtureAllowance, SUPPORTED_SCHEMA_VERSION,
    };

    fn local_profile() -> CliProfile {
        CliProfile {
            name: "local-dev".to_owned(),
            endpoint: "http://127.0.0.1:18080/overgate".to_owned(),
            endpoint_fingerprint: "fp_local_dev".to_owned(),
            environment: EnvironmentClass::Local,
            tenant_id: "tenant_local".to_owned(),
            actor_id: "actor_local".to_owned(),
            credential_namespace: "local-dev".to_owned(),
            allowed_credential_classes: vec![CredentialReferenceClass::Fixture],
            fixture_allowance: FixtureAllowance::LocalOnly,
            default_output_mode: "json".to_owned(),
            confirmation_policy: ConfirmationPolicy::ConfirmSensitive,
            schema_pins: vec![SUPPORTED_SCHEMA_VERSION.to_owned()],
            test_harness_profile: false,
        }
    }

    fn seed_profile() -> CliProfile {
        CliProfile {
            name: "seed".to_owned(),
            endpoint: "https://overgate.seed.overrid.local".to_owned(),
            endpoint_fingerprint: "fp_seed".to_owned(),
            environment: EnvironmentClass::Seed,
            tenant_id: "tenant_seed".to_owned(),
            actor_id: "actor_seed".to_owned(),
            credential_namespace: "seed".to_owned(),
            allowed_credential_classes: vec![CredentialReferenceClass::Keychain],
            fixture_allowance: FixtureAllowance::Denied,
            default_output_mode: "json".to_owned(),
            confirmation_policy: ConfirmationPolicy::ConfirmSensitive,
            schema_pins: vec![SUPPORTED_SCHEMA_VERSION.to_owned()],
            test_harness_profile: false,
        }
    }

    fn fixture_credential() -> CredentialReference {
        CredentialReference {
            reference_id: "fixture://local-dev/key-1".to_owned(),
            class: CredentialReferenceClass::Fixture,
            namespace: "local-dev".to_owned(),
            key_id: "key-1".to_owned(),
            revoked: false,
            expired: false,
        }
    }

    fn keychain_credential() -> CredentialReference {
        CredentialReference {
            reference_id: "keychain://overrid/seed/key-1".to_owned(),
            class: CredentialReferenceClass::Keychain,
            namespace: "seed".to_owned(),
            key_id: "seed-key-1".to_owned(),
            revoked: false,
            expired: false,
        }
    }

    fn phase2_config_input() -> SdkConfigInput {
        SdkConfigInput {
            environment: Some(EnvironmentClass::Local),
            base_url: "http://127.0.0.1:18080/overgate".to_owned(),
            timeout_ms: Some(DEFAULT_TIMEOUT_MS),
            max_retries: Some(DEFAULT_MAX_RETRIES),
            feature_flags: vec![
                "command_envelopes".to_owned(),
                "signing_metadata".to_owned(),
                "idempotency_cache".to_owned(),
                "error_decoding".to_owned(),
                "capability_negotiation".to_owned(),
            ],
            default_tenant_id: Some("tenant_local".to_owned()),
            client_identity_ref: "client:overrid-sdk:local-dev".to_owned(),
            credential_ref: fixture_credential(),
            service_capability_profile: SdkServiceCapabilityProfile::phase2_local(),
            live_endpoint_confirmed: false,
            test_fixtures_enabled: false,
        }
    }

    fn phase2_request_context() -> SdkRequestContextRecord {
        SdkRequestContextRecord::new(
            "actor_local",
            "tenant_local",
            "trace_sdk_phase2",
            "idem_sdk_phase2",
            "tenant create",
            Some("app:phase2-test".to_owned()),
            1_782_018_000_000,
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap()
    }

    #[test]
    fn phase2_contract_intake_manifest_names_authoritative_sources() {
        let manifest = sdk_contract_intake_manifest();
        validate_contract_intake_manifest(&manifest).unwrap();
        assert_eq!(
            manifest.contract_set.schema_version,
            SUPPORTED_SCHEMA_VERSION
        );
        assert_eq!(
            manifest.generated_output_path,
            SDK_PHASE2_GENERATED_OUTPUT_PATH
        );
        assert_eq!(
            manifest.codegen_manifest_path,
            SDK_CONTRACT_CODEGEN_MANIFEST_PATH
        );
        assert!(manifest
            .freshness_policy
            .contains("generated output is newer than source contracts"));

        for expected in [
            ("sdk_config", "cli_profile"),
            ("credential_ref", "credential_reference"),
            ("request_context", "trace_context"),
            ("signed_request", "signed_command_envelope"),
            ("idempotency_entry", "local_idempotency_cache_record"),
            ("overrid_error", "api_error"),
            ("service_capability_profile", "capability_snapshot"),
        ] {
            assert!(manifest.entries.iter().any(|entry| {
                entry.object_name == expected.0
                    && entry.source_contract_name == expected.1
                    && entry.source_path == SDK_CONTRACT_SCHEMA_PATH
            }));
        }
    }

    #[test]
    fn phase2_config_accepts_explicit_local_overgate_contract() {
        let config = SdkConfigRecord::from_input(phase2_config_input()).unwrap();
        assert_eq!(config.environment, EnvironmentClass::Local);
        assert_eq!(config.endpoint.raw(), "http://127.0.0.1:18080/overgate");
        assert_eq!(config.schema_version.raw(), SUPPORTED_SCHEMA_VERSION);
        assert!(config
            .feature_flags
            .contains(&SdkFeatureFlag::CapabilityNegotiation));
        assert_eq!(config.credential_ref.redaction_class, "test_fixture_ref");
        assert!(!config.credential_ref.stores_private_material);
        assert!(!config.credential_ref.stores_bearer_token);
    }

    #[test]
    fn phase2_config_rejects_missing_environment_and_unknown_flags() {
        let mut missing_environment = phase2_config_input();
        missing_environment.environment = None;
        assert!(matches!(
            SdkConfigRecord::from_input(missing_environment),
            Err(SdkError::MissingEnvironment)
        ));

        let mut unknown_feature = phase2_config_input();
        unknown_feature
            .feature_flags
            .push("surprise_flag".to_owned());
        assert!(matches!(
            SdkConfigRecord::from_input(unknown_feature),
            Err(SdkError::UnknownFeatureFlag(flag)) if flag == "surprise_flag"
        ));
    }

    #[test]
    fn phase2_config_rejects_implicit_live_endpoint_and_production_fixtures() {
        let mut live_without_confirmation = phase2_config_input();
        live_without_confirmation.environment = Some(EnvironmentClass::ProductionLike);
        live_without_confirmation.base_url = "https://overgate.production.overrid.local".to_owned();
        assert!(matches!(
            SdkConfigRecord::from_input(live_without_confirmation),
            Err(SdkError::LiveEndpointConfirmationRequired {
                environment: "production_like"
            })
        ));

        let mut production_fixture = phase2_config_input();
        production_fixture.environment = Some(EnvironmentClass::ProductionLike);
        production_fixture.base_url = "https://overgate.production.overrid.local".to_owned();
        production_fixture.live_endpoint_confirmed = true;
        production_fixture.test_fixtures_enabled = true;
        assert!(matches!(
            SdkConfigRecord::from_input(production_fixture),
            Err(SdkError::FixtureInProduction)
        ));
    }

    #[test]
    fn phase2_request_and_signed_records_preserve_refs_without_secret_material() {
        let context = phase2_request_context();
        assert_eq!(context.actor_id, "actor_local");
        assert_eq!(context.tenant_id, "tenant_local");
        assert_eq!(context.trace_context.trace_id, "trace_sdk_phase2");
        assert_eq!(context.idempotency.key, "idem_sdk_phase2");

        let credential_ref =
            SdkCredentialReferenceRecord::from_credential(&fixture_credential()).unwrap();
        let signed = SdkSignedRequestRecord::new(
            context,
            &credential_ref,
            "POST",
            "/commands",
            "hash_0123456789abcdef",
            "sigref:local-dev:key-1:tenant_create",
            120_000,
        )
        .unwrap();
        assert_eq!(signed.credential_id, "fixture://local-dev/key-1");
        assert!(!signed.stores_private_material);
        assert!(!signed.stores_bearer_token);
    }

    #[test]
    fn phase2_credential_records_reject_secret_like_values() {
        let mut credential = fixture_credential();
        credential.key_id = "private_key_value".to_owned();
        assert!(matches!(
            SdkCredentialReferenceRecord::from_credential(&credential),
            Err(SdkError::SecretMaterialRejected {
                field: "credential key id"
            })
        ));
    }

    #[test]
    fn phase2_idempotency_records_are_bounded_and_skip_read_only_cache() {
        let context = phase2_request_context();
        assert!(SdkIdempotencyEntry::for_command_class(
            SdkCommandClass::ReadOnly,
            &context,
            "hash_0123456789abcdef",
            None,
            vec![],
            RetryClass::NotRetryable,
            vec![],
        )
        .unwrap()
        .is_none());

        let entry = SdkIdempotencyEntry::for_command_class(
            SdkCommandClass::Phase1Mutating,
            &context,
            "hash_0123456789abcdef",
            Some("digest_terminal_success".to_owned()),
            vec!["audit:overwatch:phase2".to_owned()],
            RetryClass::SafeRetry,
            vec!["retry_after_ms".to_owned()],
        )
        .unwrap()
        .unwrap();
        assert_eq!(entry.command_class.as_str(), "phase1_mutating");
        assert_eq!(entry.retention_ms, 24 * 60 * 60 * 1_000);
        assert!(!entry.contains_raw_payload);
        assert_eq!(entry.trace_id, "trace_sdk_phase2");
    }

    #[test]
    fn phase2_error_records_preserve_reason_trace_audit_and_retryability() {
        let error = OverridErrorRecord::new(
            "policy_denied",
            "policy denied this command",
            Some("trace_sdk_phase2".to_owned()),
            vec!["audit:overwatch:policy_denied".to_owned()],
            RetryClass::NotRetryable,
            vec!["change_policy_ref".to_owned()],
            Some("overguard".to_owned()),
            vec!["policy:seed:deny-egress".to_owned()],
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap();
        assert_eq!(error.reason_code, "policy_denied");
        assert_eq!(error.trace_id.as_deref(), Some("trace_sdk_phase2"));
        assert_eq!(error.audit_refs.len(), 1);
        assert!(!error.retryable);
        assert_eq!(error.schema_version.raw(), SUPPORTED_SCHEMA_VERSION);
    }

    #[test]
    fn phase2_capability_negotiation_fails_closed_before_unsafe_helpers() {
        let profile = SdkServiceCapabilityProfile::phase2_local();
        let accepted = negotiate_sdk_capability(
            &profile,
            SdkOptionalHelper::CommandSubmission,
            SUPPORTED_SCHEMA_VERSION,
            SDK_CURRENT_STABLE_MAJOR,
        )
        .unwrap();
        assert!(accepted.snapshot.available);
        assert!(accepted.snapshot.fail_closed);
        assert_eq!(
            accepted.snapshot.phase_gate,
            "phase_1_control_plane_bootstrap"
        );

        assert!(matches!(
            negotiate_sdk_capability(
                &profile,
                SdkOptionalHelper::PolicyDryRun,
                SUPPORTED_SCHEMA_VERSION,
                SDK_CURRENT_STABLE_MAJOR,
            ),
            Err(SdkError::CapabilityUnavailable {
                helper: "policy_dry_run",
                reason_code: SDK_CAPABILITY_UNAVAILABLE_REASON_CODE
            })
        ));
    }

    #[test]
    fn phase3_package_boundary_and_generated_models_are_validated() {
        let boundary = sdk_package_boundary();
        let descriptors = sdk_generated_model_descriptors();

        validate_sdk_package_boundary(&boundary).unwrap();
        validate_generated_model_descriptors(&descriptors).unwrap();
        assert!(boundary.iter().all(|module| !module.contract_authority));
        assert!(descriptors
            .iter()
            .any(|descriptor| descriptor.kind == SdkGeneratedModelKind::Error
                && descriptor.reason_code_object));
    }

    #[test]
    fn phase3_configure_client_applies_trace_redaction_and_provider_refs() {
        let config = SdkConfigRecord::from_input(phase2_config_input()).unwrap();

        let configured = configure_client(config).unwrap();

        assert_eq!(configured.trace_policy.as_str(), SDK_DEFAULT_TRACE_POLICY);
        assert_eq!(
            configured.redaction_defaults.profile,
            SDK_REDACTION_DEFAULTS_PROFILE
        );
        assert!(configured.redaction_defaults.redact_payloads);
        assert!(configured.redaction_defaults.redact_signatures);
        assert!(configured.redaction_defaults.redact_secret_refs);
        assert_eq!(
            configured.credential_provider.credential_id,
            "fixture://local-dev/key-1"
        );
        assert!(!configured.credential_provider.stores_private_material);
        assert_eq!(
            configured.client.endpoint().raw(),
            "http://127.0.0.1:18080/overgate"
        );
    }

    #[test]
    fn phase3_configure_client_blocks_non_loopback_local_without_explicit_confirmation() {
        let mut input = phase2_config_input();
        input.base_url = "https://dev-overgate.example.test".to_owned();
        input.live_endpoint_confirmed = false;
        let config = SdkConfigRecord::from_input(input).unwrap();

        assert!(matches!(
            configure_client(config),
            Err(SdkError::LiveEndpointConfirmationRequired { environment })
                if environment == "local"
        ));
    }

    #[test]
    fn phase3_read_helpers_preserve_control_plane_refs() {
        let config = SdkConfigRecord::from_input(phase2_config_input()).unwrap();
        let pagination = SdkPagination::new(Some(50), Some("cursor_next".to_owned())).unwrap();

        let request = build_control_plane_read_request(
            &config.endpoint,
            SdkControlPlaneReadKind::Identity,
            "identity:alice",
            "request_phase3",
            "trace_phase3",
            SUPPORTED_SCHEMA_VERSION,
            pagination,
            vec!["audit:identity:alice".to_owned()],
        )
        .unwrap();

        assert!(request.read_only);
        assert_eq!(request.route, "/v1/control-plane/identities");
        assert_eq!(request.object_ref, "identity:alice");
        assert_eq!(request.request_id, "request_phase3");
        assert_eq!(request.trace_id, "trace_phase3");
        assert_eq!(request.pagination.cursor.as_deref(), Some("cursor_next"));
        assert_eq!(request.audit_refs, vec!["audit:identity:alice"]);
    }

    #[test]
    fn phase3_version_report_names_schema_revision_and_features() {
        let report = sdk_version_report();

        assert_eq!(report.sdk_name, SDK_NAME);
        assert_eq!(report.schema_set, SDK_PHASE3_SCHEMA_SET);
        assert_eq!(
            report.generated_contract_revision,
            SDK_PHASE3_GENERATED_CONTRACT_REVISION
        );
        assert_eq!(report.language_binding, SDK_LANGUAGE_BINDING);
        assert_eq!(
            report.service_capability_profile,
            SDK_PHASE3_CAPABILITY_PROFILE
        );
        assert!(report
            .supported_feature_flags
            .contains(&"command_envelopes"));
        assert!(report
            .supported_feature_flags
            .contains(&"capability_negotiation"));
    }

    #[test]
    fn accepts_loopback_overgate_path() {
        let endpoint =
            OvergateEndpoint::parse("http://127.0.0.1:18080/overgate", EnvironmentClass::Local)
                .unwrap();
        assert_eq!(endpoint.environment().as_str(), "local");
    }

    #[test]
    fn accepts_overgate_host() {
        let endpoint = OvergateEndpoint::parse(
            "https://overgate.seed.overrid.local",
            EnvironmentClass::Seed,
        )
        .unwrap();
        assert_eq!(endpoint.raw(), "https://overgate.seed.overrid.local");
    }

    #[test]
    fn rejects_private_service_target() {
        assert!(matches!(
            OvergateEndpoint::parse("http://127.0.0.1:18080/overqueue", EnvironmentClass::Local),
            Err(SdkError::PrivateServiceTarget(_))
        ));
    }

    #[test]
    fn rejects_endpoint_without_overgate_target() {
        assert!(matches!(
            OvergateEndpoint::parse("http://127.0.0.1:18080", EnvironmentClass::Local),
            Err(SdkError::MissingOvergateTarget(_))
        ));
    }

    #[test]
    fn rejects_unsupported_schema_version_before_request_build() {
        let mut config = ClientConfig::local_overgate("http://127.0.0.1:18080/overgate").unwrap();
        config.schema_version = "cli-command.v9.0".to_owned();
        assert!(matches!(
            OverridSdkClient::new(config),
            Err(SdkError::Compatibility(
                SdkCompatibilityRejection::UnsupportedSchemaVersion {
                    reason_code: SDK_UNSUPPORTED_SCHEMA_REASON_CODE,
                    ..
                }
            ))
        ));
    }

    #[test]
    fn sdk_compatibility_metadata_names_release_gate_values() {
        let metadata = sdk_compatibility_metadata();
        assert_eq!(metadata.sdk_name, SDK_NAME);
        assert_eq!(metadata.language_binding, "rust");
        assert_eq!(metadata.current_stable_major, SDK_CURRENT_STABLE_MAJOR);
        assert_eq!(metadata.previous_stable_major, SDK_PREVIOUS_STABLE_MAJOR);
        assert_eq!(
            metadata.supported_schema_versions,
            SDK_SUPPORTED_SCHEMA_VERSIONS
        );
        assert!(metadata
            .supported_schema_versions
            .contains(&SUPPORTED_SCHEMA_VERSION));
        assert_eq!(
            metadata.unsupported_sdk_version_reason,
            "unsupported_sdk_version"
        );
        assert_eq!(
            metadata.unsupported_schema_version_reason,
            "schema_version_unsupported"
        );
        assert!(metadata
            .service_capability_profile
            .contains("phase1-control-plane"));
        assert!(metadata.upgrade_guidance.contains("Overgate"));
    }

    #[test]
    fn sdk_release_checklist_carries_required_phase1_gate_items() {
        let checklist = sdk_release_checklist();
        assert_eq!(checklist.metadata, sdk_compatibility_metadata());

        for required in [
            "schema versions named",
            "service capability profile named",
            "deprecation behavior documented",
            "upgrade guidance documented",
            "security-critical emergency break handling documented",
            "Rust-first binding is the first release target",
            "TypeScript/web bindings remain generated second target after schema stability",
            "credential-provider-only signing preserved",
            "bounded idempotency retention documented",
            "Mobile SDK boundary remains separate",
            "current-plus-previous stable major compatibility checked",
            "unsupported_sdk_version returned for unsafe SDK majors",
            "schema_version_unsupported returned for unsafe schema versions",
        ] {
            assert!(checklist.required_items.contains(&required));
        }
    }

    #[test]
    fn compatibility_check_rejects_unsafe_sdk_or_schema_without_downgrade() {
        let accepted =
            check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, SUPPORTED_SCHEMA_VERSION).unwrap();
        assert_eq!(accepted.raw(), SUPPORTED_SCHEMA_VERSION);

        let version_error =
            check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR + 99, SUPPORTED_SCHEMA_VERSION)
                .unwrap_err();
        assert_eq!(
            version_error.reason_code(),
            SDK_UNSUPPORTED_VERSION_REASON_CODE
        );
        assert!(matches!(
            version_error,
            SdkCompatibilityRejection::UnsupportedSdkVersion { .. }
        ));

        let schema_error =
            check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, "cli-command.v99.0").unwrap_err();
        assert_eq!(
            schema_error.reason_code(),
            SDK_UNSUPPORTED_SCHEMA_REASON_CODE
        );
        assert!(matches!(
            schema_error,
            SdkCompatibilityRejection::UnsupportedSchemaVersion { .. }
        ));

        let downgrade_error =
            check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, "cli-command.v0.0").unwrap_err();
        assert_eq!(
            downgrade_error.reason_code(),
            SDK_UNSUPPORTED_SCHEMA_REASON_CODE
        );
        assert!(matches!(
            downgrade_error,
            SdkCompatibilityRejection::UnsupportedSchemaVersion { .. }
        ));
    }

    #[test]
    fn builds_overgate_only_request_metadata() {
        let config = ClientConfig::local_overgate("http://127.0.0.1:18080/overgate").unwrap();
        let client = OverridSdkClient::new(config).unwrap();
        let request = client.build_request("version", "trace_cli_phase2");
        assert_eq!(
            client.compatibility_metadata(),
            sdk_compatibility_metadata()
        );
        assert_eq!(request.operation, "version");
        assert_eq!(request.schema_version, SUPPORTED_SCHEMA_VERSION);
        assert!(request
            .headers
            .contains(&("x-overrid-target".to_owned(), "overgate".to_owned())));
        assert!(request
            .headers
            .contains(&("x-overrid-sdk-name".to_owned(), SDK_NAME.to_owned())));
        assert!(request
            .headers
            .contains(&("x-overrid-sdk-version".to_owned(), SDK_VERSION.to_owned())));
        assert!(request.headers.contains(&(
            "x-overrid-sdk-capability-profile".to_owned(),
            SDK_PHASE3_CAPABILITY_PROFILE.to_owned()
        )));
    }

    #[test]
    fn builds_bounded_retry_timeout_policy() {
        let default_policy = retry_timeout_policy(None, None);
        assert_eq!(default_policy.max_retries, DEFAULT_MAX_RETRIES);
        assert_eq!(default_policy.timeout_ms, DEFAULT_TIMEOUT_MS);
        assert!(default_policy.bounded);

        let clamped = retry_timeout_policy(Some(9), Some(900_000));
        assert_eq!(clamped.max_retries, 5);
        assert_eq!(clamped.timeout_ms, 600_000);
    }

    #[test]
    fn decodes_errors_without_internal_details() {
        let record = decode_phase6_error(
            "transport_unavailable",
            "socket reset",
            ExitCodeClass::Transport,
            RetryClass::SafeRetry,
        );

        assert_eq!(record.source_family, "platform");
        assert_eq!(record.retry_class, RetryClass::SafeRetry);
        assert!(record.remediation_hint.contains("same idempotency key"));
        assert!(!record.raw_internal_error_exposed);
    }

    #[test]
    fn allows_local_fixture_profile_before_request_build() {
        let profile = local_profile();
        let credential = fixture_credential();
        let decision = enforce_profile_environment(CommandSafetyInput {
            profile: &profile,
            credential: &credential,
            endpoint_override: None,
            explicit_profile: true,
            confirm_profile: false,
            mutating: false,
            admin_impacting: false,
            reason: None,
            command_type: "profile inspect",
        })
        .unwrap();
        assert_eq!(decision.environment, EnvironmentClass::Local);
        assert!(decision.signer_handoff.is_none());
    }

    #[test]
    fn rejects_seed_endpoint_override_before_request_build() {
        let profile = seed_profile();
        let credential = keychain_credential();
        assert!(matches!(
            enforce_profile_environment(CommandSafetyInput {
                profile: &profile,
                credential: &credential,
                endpoint_override: Some("https://overgate.other.example"),
                explicit_profile: true,
                confirm_profile: true,
                mutating: true,
                admin_impacting: false,
                reason: None,
                command_type: "tenant create",
            }),
            Err(SdkError::UnsafeEndpointOverride {
                environment: "seed"
            })
        ));
    }

    #[test]
    fn requires_profile_confirmation_for_seed_mutation() {
        let profile = seed_profile();
        let credential = keychain_credential();
        assert!(matches!(
            enforce_profile_environment(CommandSafetyInput {
                profile: &profile,
                credential: &credential,
                endpoint_override: None,
                explicit_profile: true,
                confirm_profile: false,
                mutating: true,
                admin_impacting: false,
                reason: None,
                command_type: "tenant create",
            }),
            Err(SdkError::MissingProfileConfirmation {
                environment: "seed"
            })
        ));
    }

    #[test]
    fn requires_reason_for_admin_impacting_mutation() {
        let profile = seed_profile();
        let credential = keychain_credential();
        assert!(matches!(
            enforce_profile_environment(CommandSafetyInput {
                profile: &profile,
                credential: &credential,
                endpoint_override: None,
                explicit_profile: true,
                confirm_profile: true,
                mutating: true,
                admin_impacting: true,
                reason: None,
                command_type: "key revoke",
            }),
            Err(SdkError::MissingReason)
        ));
    }

    #[test]
    fn signer_handoff_uses_reference_only_for_seed_mutation() {
        let profile = seed_profile();
        let credential = keychain_credential();
        let decision = enforce_profile_environment(CommandSafetyInput {
            profile: &profile,
            credential: &credential,
            endpoint_override: None,
            explicit_profile: true,
            confirm_profile: true,
            mutating: true,
            admin_impacting: true,
            reason: Some("rotation drill"),
            command_type: "key revoke",
        })
        .unwrap();
        let handoff = decision.signer_handoff.unwrap();
        assert_eq!(handoff.credential_reference_id, credential.reference_id);
        assert!(!handoff.exposes_key_material);
        assert!(handoff.signature_ref.starts_with("sigref:seed:seed-key-1:"));
    }

    #[test]
    fn revoked_credential_fails_as_credential_error_before_request_build() {
        let profile = seed_profile();
        let mut credential = keychain_credential();
        credential.revoked = true;

        assert!(matches!(
            enforce_profile_environment(CommandSafetyInput {
                profile: &profile,
                credential: &credential,
                endpoint_override: None,
                explicit_profile: true,
                confirm_profile: true,
                mutating: true,
                admin_impacting: false,
                reason: None,
                command_type: "credential inspect",
            }),
            Err(SdkError::Credential(
                ProfileValidationError::CredentialRevoked
            ))
        ));
    }
}
