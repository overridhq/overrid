#![forbid(unsafe_code)]

use std::fmt;

use overrid_contracts::{
    ensure_supported_schema_version, ContractError, ProfileValidationError, SchemaVersion,
    SUPPORTED_SCHEMA_VERSION,
};
pub use overrid_contracts::{
    CliProfile, ConfirmationPolicy, CredentialReference, CredentialReferenceClass,
    EnvironmentClass, ErrorDecodeRecord, ExitCodeClass, FixtureAllowance, RetryClass,
    RetryTimeoutPolicy, SignerHandoff,
};

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
pub const SDK_DEPRECATION_BEHAVIOR: &str =
    "support current stable major and previous stable major when present";
pub const SDK_UPGRADE_GUIDANCE: &str =
    "read Overgate capability profiles before using optional SDK helpers";
pub const SDK_EMERGENCY_BREAK_POLICY: &str =
    "security-critical breaks return stable unsupported reason codes";
pub const SDK_SUPPORTED_SCHEMA_VERSIONS: &[&str] = &[SUPPORTED_SCHEMA_VERSION];
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
    Profile(ProfileValidationError),
    Credential(ProfileValidationError),
    UnsafeEndpointOverride { environment: &'static str },
    MissingExplicitProfile { environment: &'static str },
    MissingProfileConfirmation { environment: &'static str },
    MissingReason,
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
