#![forbid(unsafe_code)]

use std::fmt;

use overrid_contracts::{
    ensure_supported_schema_version, ContractError, ProfileValidationError, SchemaVersion,
    SUPPORTED_SCHEMA_VERSION,
};
pub use overrid_contracts::{
    CliProfile, ConfirmationPolicy, CredentialReference, CredentialReferenceClass,
    EnvironmentClass, FixtureAllowance, SignerHandoff,
};

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
            timeout_ms: 10_000,
            max_retries: 2,
            schema_version: SUPPORTED_SCHEMA_VERSION.to_owned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverridSdkClient {
    config: ClientConfig,
    schema_version: SchemaVersion,
}

impl OverridSdkClient {
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        let schema_version = ensure_supported_schema_version(&config.schema_version)?;
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
        ConfirmationPolicy, ContractError, CredentialReferenceClass, FixtureAllowance,
        SUPPORTED_SCHEMA_VERSION,
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
            Err(SdkError::Contract(
                ContractError::UnsupportedSchemaVersion { .. }
            ))
        ));
    }

    #[test]
    fn builds_overgate_only_request_metadata() {
        let config = ClientConfig::local_overgate("http://127.0.0.1:18080/overgate").unwrap();
        let client = OverridSdkClient::new(config).unwrap();
        let request = client.build_request("version", "trace_cli_phase2");
        assert_eq!(request.operation, "version");
        assert_eq!(request.schema_version, SUPPORTED_SCHEMA_VERSION);
        assert!(request
            .headers
            .contains(&("x-overrid-target".to_owned(), "overgate".to_owned())));
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
