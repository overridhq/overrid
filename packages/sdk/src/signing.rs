use std::fmt;

use crate::{
    SdkCommandEnvelope, SdkConfigRecord, SdkCredentialReferenceRecord, SdkError,
    SdkRequestContextRecord, SdkSignedRequestRecord,
};
use overrid_contracts::{CredentialReferenceClass, EnvironmentClass};

pub const SDK_PHASE5_CAPABILITY_PROFILE: &str = "phase5-credential-signing-security-guardrails";
pub const SDK_PHASE5_SIGNING_ALGORITHM: &str = "ed25519-overrid-v0";
pub const SDK_PHASE5_REDACTION_POLICY: &str = "phase5_secret_free_refs_only";
pub const SDK_PHASE5_TEST_SIGNER_MODULE: &str = "packages/sdk/src/signing.rs#test-only-fixtures";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkCredentialProviderKind {
    FileBackedLocalTest,
    HostSigningAgentSocket,
    PlatformKeychain,
    HardwareBackedDevice,
    OverkeyCredentialRef,
}

impl SdkCredentialProviderKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FileBackedLocalTest => "file_backed_local_test",
            Self::HostSigningAgentSocket => "host_signing_agent_socket",
            Self::PlatformKeychain => "platform_keychain",
            Self::HardwareBackedDevice => "hardware_backed_device",
            Self::OverkeyCredentialRef => "overkey_credential_ref",
        }
    }

    pub fn from_credential_class(class: CredentialReferenceClass) -> Self {
        match class {
            CredentialReferenceClass::Fixture => Self::FileBackedLocalTest,
            CredentialReferenceClass::SigningAgent => Self::HostSigningAgentSocket,
            CredentialReferenceClass::Keychain
            | CredentialReferenceClass::SecretService
            | CredentialReferenceClass::EncryptedStore => Self::PlatformKeychain,
            CredentialReferenceClass::HardwareToken => Self::HardwareBackedDevice,
            CredentialReferenceClass::CiReference => Self::OverkeyCredentialRef,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkSigningCapability {
    pub algorithm: &'static str,
    pub can_sign: bool,
    pub requires_online_agent: bool,
    pub production_capable: bool,
    pub local_test_only: bool,
}

impl SdkSigningCapability {
    fn for_provider_kind(kind: SdkCredentialProviderKind) -> Self {
        Self {
            algorithm: SDK_PHASE5_SIGNING_ALGORITHM,
            can_sign: true,
            requires_online_agent: matches!(
                kind,
                SdkCredentialProviderKind::HostSigningAgentSocket
            ),
            production_capable: !matches!(kind, SdkCredentialProviderKind::FileBackedLocalTest),
            local_test_only: matches!(kind, SdkCredentialProviderKind::FileBackedLocalTest),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkCredentialProvider {
    pub provider_id: String,
    pub kind: SdkCredentialProviderKind,
    pub credential_id: String,
    pub credential_namespace: String,
    pub key_id: String,
    pub public_metadata: Vec<(String, String)>,
    pub signing_capability: SdkSigningCapability,
    pub rotation_hint: String,
    pub revocation_hint: String,
    pub redaction_class: &'static str,
    pub production_capable: bool,
    pub test_only: bool,
    pub stores_private_material: bool,
    pub stores_bearer_token: bool,
    pub stores_seed_phrase: bool,
    pub stores_vault_value: bool,
}

impl SdkCredentialProvider {
    pub fn from_config(config: &SdkConfigRecord) -> Result<Self, SdkPhase5Error> {
        Self::from_record(
            &config.credential_ref,
            config.environment,
            config.test_fixtures_enabled,
        )
    }

    pub fn from_record(
        credential: &SdkCredentialReferenceRecord,
        environment: EnvironmentClass,
        test_fixtures_enabled: bool,
    ) -> Result<Self, SdkPhase5Error> {
        require_phase5_non_empty(&credential.credential_id, "credential id")?;
        require_phase5_non_empty(&credential.namespace, "credential namespace")?;
        require_phase5_non_empty(&credential.key_id, "credential key id")?;

        for (field, value) in [
            ("credential id", credential.credential_id.as_str()),
            ("credential namespace", credential.namespace.as_str()),
            ("credential key id", credential.key_id.as_str()),
        ] {
            reject_phase5_secret_like_value(field, value)?;
        }

        if credential.revoked {
            return Err(SdkPhase5Error::CredentialLifecycle(
                credential_lifecycle_failure(
                    SdkCredentialLifecycleStatus::Revoked,
                    Some(credential.credential_id.as_str()),
                    false,
                ),
            ));
        }
        if credential.expired {
            return Err(SdkPhase5Error::CredentialLifecycle(
                credential_lifecycle_failure(
                    SdkCredentialLifecycleStatus::Expired,
                    Some(credential.credential_id.as_str()),
                    false,
                ),
            ));
        }

        let kind = SdkCredentialProviderKind::from_credential_class(credential.credential_class);
        let signing_capability = SdkSigningCapability::for_provider_kind(kind);
        let provider = Self {
            provider_id: format!(
                "{}:{}:{}",
                kind.as_str(),
                credential.namespace,
                credential.key_id
            ),
            kind,
            credential_id: credential.credential_id.clone(),
            credential_namespace: credential.namespace.clone(),
            key_id: credential.key_id.clone(),
            public_metadata: vec![
                (
                    "credential_class".to_owned(),
                    credential.credential_class.as_str().to_owned(),
                ),
                (
                    "credential_namespace".to_owned(),
                    credential.namespace.clone(),
                ),
                ("key_id".to_owned(), credential.key_id.clone()),
                (
                    "redaction_class".to_owned(),
                    credential.redaction_class.to_owned(),
                ),
            ],
            signing_capability: signing_capability.clone(),
            rotation_hint: format!("overkey:rotation-hint:{}", credential.key_id),
            revocation_hint: format!("overkey:revocation-hint:{}", credential.key_id),
            redaction_class: credential.redaction_class,
            production_capable: signing_capability.production_capable,
            test_only: signing_capability.local_test_only,
            stores_private_material: false,
            stores_bearer_token: false,
            stores_seed_phrase: false,
            stores_vault_value: false,
        };

        if provider.kind == SdkCredentialProviderKind::FileBackedLocalTest {
            validate_fixture_signer_installation(environment, test_fixtures_enabled, &provider)?;
        }

        Ok(provider)
    }

    pub fn credential_record(&self) -> SdkCredentialReferenceRecord {
        SdkCredentialReferenceRecord {
            credential_id: self.credential_id.clone(),
            credential_class: match self.kind {
                SdkCredentialProviderKind::FileBackedLocalTest => CredentialReferenceClass::Fixture,
                SdkCredentialProviderKind::HostSigningAgentSocket => {
                    CredentialReferenceClass::SigningAgent
                }
                SdkCredentialProviderKind::PlatformKeychain => CredentialReferenceClass::Keychain,
                SdkCredentialProviderKind::HardwareBackedDevice => {
                    CredentialReferenceClass::HardwareToken
                }
                SdkCredentialProviderKind::OverkeyCredentialRef => {
                    CredentialReferenceClass::CiReference
                }
            },
            namespace: self.credential_namespace.clone(),
            key_id: self.key_id.clone(),
            redaction_class: self.redaction_class,
            stores_private_material: false,
            stores_bearer_token: false,
            revoked: false,
            expired: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkTestSignerGate {
    pub environment: EnvironmentClass,
    pub requires_explicit_installation: bool,
    pub fixture_provider: bool,
    pub production_configurable: bool,
    pub module_path: &'static str,
}

pub fn validate_fixture_signer_installation(
    environment: EnvironmentClass,
    test_fixtures_enabled: bool,
    provider: &SdkCredentialProvider,
) -> Result<SdkTestSignerGate, SdkPhase5Error> {
    if provider.kind != SdkCredentialProviderKind::FileBackedLocalTest {
        return Ok(SdkTestSignerGate {
            environment,
            requires_explicit_installation: false,
            fixture_provider: false,
            production_configurable: provider.production_capable,
            module_path: SDK_PHASE5_TEST_SIGNER_MODULE,
        });
    }

    let explicitly_local_or_ci =
        matches!(environment, EnvironmentClass::Local | EnvironmentClass::Ci);
    if !explicitly_local_or_ci || !test_fixtures_enabled {
        return Err(SdkPhase5Error::FixtureSignerNotAllowed {
            environment: environment.as_str(),
        });
    }

    Ok(SdkTestSignerGate {
        environment,
        requires_explicit_installation: true,
        fixture_provider: true,
        production_configurable: false,
        module_path: SDK_PHASE5_TEST_SIGNER_MODULE,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkCanonicalSigningInput {
    pub method: String,
    pub path: String,
    pub sorted_headers: Vec<(String, String)>,
    pub body_hash: String,
    pub timestamp_ms: u64,
    pub schema_version: String,
    pub credential_id: String,
    pub replay_window_ms: u64,
    pub tenant_id: String,
    pub actor_id: String,
    pub trace_id: String,
    pub idempotency_key: String,
    canonical_string: String,
}

impl SdkCanonicalSigningInput {
    pub fn canonical_string(&self) -> &str {
        &self.canonical_string
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        self.canonical_string.as_bytes().to_vec()
    }
}

pub fn phase5_signature_ref(
    provider: &SdkCredentialProvider,
    command_type: &str,
) -> Result<String, SdkPhase5Error> {
    require_phase5_non_empty(command_type, "command type")?;
    Ok(format!(
        "sigref:{}:{}:{}",
        canonical_token(&provider.credential_namespace),
        canonical_token(&provider.key_id),
        canonical_token(command_type)
    ))
}

pub fn build_canonical_signing_input(
    command: &SdkCommandEnvelope,
    provider: &SdkCredentialProvider,
    method: impl Into<String>,
    path: impl Into<String>,
    replay_window_ms: u64,
) -> Result<SdkCanonicalSigningInput, SdkPhase5Error> {
    let method = method.into().to_ascii_uppercase();
    let path = path.into();
    require_phase5_non_empty(&method, "request method")?;
    require_phase5_non_empty(&path, "request path")?;
    require_phase5_non_empty(&command.payload_hash, "body hash")?;
    if !command.payload_hash.starts_with("hash_") {
        return Err(SdkPhase5Error::InvalidSigningField { field: "body hash" });
    }
    if command.timestamp_ms == 0 {
        return Err(SdkPhase5Error::InvalidSigningField { field: "timestamp" });
    }
    if replay_window_ms == 0 {
        return Err(SdkPhase5Error::InvalidSigningField {
            field: "replay window",
        });
    }
    if !provider.signing_capability.can_sign {
        return Err(SdkPhase5Error::MissingSigningCapability {
            provider_id: provider.provider_id.clone(),
        });
    }
    if provider.signing_capability.algorithm != SDK_PHASE5_SIGNING_ALGORITHM {
        return Err(SdkPhase5Error::UnsupportedSigningAlgorithm {
            algorithm: provider.signing_capability.algorithm.to_owned(),
        });
    }

    let expected_signature_ref = phase5_signature_ref(provider, &command.envelope.command_type)?;
    if command.envelope.signature_ref != expected_signature_ref {
        return Err(SdkPhase5Error::MismatchedCredentialId {
            expected: provider.credential_id.clone(),
            actual: command.envelope.signature_ref.clone(),
        });
    }

    let mut sorted_headers = command.headers.clone();
    sorted_headers.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));

    let mut lines = vec![
        format!("method={}", canonical_escape(&method)),
        format!("path={}", canonical_escape(&path)),
        format!("body_hash={}", canonical_escape(&command.payload_hash)),
        format!("timestamp_ms={}", command.timestamp_ms),
        format!(
            "schema_version={}",
            canonical_escape(&command.schema_version)
        ),
        format!(
            "credential_id={}",
            canonical_escape(&provider.credential_id)
        ),
        format!("replay_window_ms={replay_window_ms}"),
        format!(
            "tenant_id={}",
            canonical_escape(&command.envelope.tenant_id)
        ),
        format!("actor_id={}", canonical_escape(&command.envelope.actor_id)),
        format!(
            "trace_id={}",
            canonical_escape(&command.envelope.trace_context.trace_id)
        ),
        format!(
            "idempotency_key={}",
            canonical_escape(&command.envelope.idempotency.key)
        ),
    ];
    for (key, value) in &sorted_headers {
        lines.push(format!(
            "header:{}={}",
            canonical_escape(key),
            canonical_escape(value)
        ));
    }

    Ok(SdkCanonicalSigningInput {
        method,
        path,
        sorted_headers,
        body_hash: command.payload_hash.clone(),
        timestamp_ms: command.timestamp_ms,
        schema_version: command.schema_version.clone(),
        credential_id: provider.credential_id.clone(),
        replay_window_ms,
        tenant_id: command.envelope.tenant_id.clone(),
        actor_id: command.envelope.actor_id.clone(),
        trace_id: command.envelope.trace_context.trace_id.clone(),
        idempotency_key: command.envelope.idempotency.key.clone(),
        canonical_string: lines.join("\n"),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkSignedOvergateRequest {
    pub canonical_input: SdkCanonicalSigningInput,
    pub signature_ref: String,
    pub signature_metadata: String,
    pub algorithm: String,
    pub signed_record: SdkSignedRequestRecord,
    pub stores_private_material: bool,
    pub stores_bearer_token: bool,
    pub stores_seed_phrase: bool,
    pub stores_vault_value: bool,
}

pub fn sign_request(
    command: &SdkCommandEnvelope,
    provider: &SdkCredentialProvider,
    method: impl Into<String>,
    path: impl Into<String>,
    replay_window_ms: u64,
) -> Result<SdkSignedOvergateRequest, SdkPhase5Error> {
    let method = method.into();
    let path = path.into();
    let canonical_input = build_canonical_signing_input(
        command,
        provider,
        method.clone(),
        path.clone(),
        replay_window_ms,
    )?;
    let signature_ref = phase5_signature_ref(provider, &command.envelope.command_type)?;
    let signature_metadata = format!(
        "algorithm={};credential_id={};signature_ref=redacted",
        SDK_PHASE5_SIGNING_ALGORITHM, provider.credential_id
    );
    let context = SdkRequestContextRecord::new(
        command.envelope.actor_id.clone(),
        command.envelope.tenant_id.clone(),
        command.envelope.trace_context.trace_id.clone(),
        command.envelope.idempotency.key.clone(),
        command.envelope.command_type.clone(),
        None,
        command.timestamp_ms,
        command.schema_version.as_str(),
    )?;
    let credential_record = provider.credential_record();
    let signed_record = SdkSignedRequestRecord::new(
        context,
        &credential_record,
        method,
        path,
        command.payload_hash.clone(),
        signature_metadata.clone(),
        replay_window_ms,
    )?;

    Ok(SdkSignedOvergateRequest {
        canonical_input,
        signature_ref,
        signature_metadata,
        algorithm: SDK_PHASE5_SIGNING_ALGORITHM.to_owned(),
        signed_record,
        stores_private_material: false,
        stores_bearer_token: false,
        stores_seed_phrase: false,
        stores_vault_value: false,
    })
}

pub fn validate_signed_request_invariants(
    signed: &SdkSignedOvergateRequest,
    command: &SdkCommandEnvelope,
    provider: &SdkCredentialProvider,
    expected_payload_hash: &str,
    expected_tenant_id: &str,
    now_ms: u64,
) -> Result<(), SdkPhase5Error> {
    if signed.canonical_input.body_hash != expected_payload_hash {
        return Err(SdkPhase5Error::MutatedPayload {
            expected: expected_payload_hash.to_owned(),
            actual: signed.canonical_input.body_hash.clone(),
        });
    }
    if command.envelope.tenant_id != expected_tenant_id {
        return Err(SdkPhase5Error::WrongTenant {
            expected: expected_tenant_id.to_owned(),
            actual: command.envelope.tenant_id.clone(),
        });
    }
    if signed.canonical_input.credential_id != provider.credential_id {
        return Err(SdkPhase5Error::MismatchedCredentialId {
            expected: provider.credential_id.clone(),
            actual: signed.canonical_input.credential_id.clone(),
        });
    }
    if signed.algorithm != SDK_PHASE5_SIGNING_ALGORITHM {
        return Err(SdkPhase5Error::UnsupportedSigningAlgorithm {
            algorithm: signed.algorithm.clone(),
        });
    }
    if now_ms
        > signed
            .canonical_input
            .timestamp_ms
            .saturating_add(signed.canonical_input.replay_window_ms)
    {
        return Err(SdkPhase5Error::ExpiredTimestamp {
            timestamp_ms: signed.canonical_input.timestamp_ms,
            now_ms,
            replay_window_ms: signed.canonical_input.replay_window_ms,
        });
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkDiagnosticEventKind {
    RequestBuilt,
    RequestSigned,
    RequestSent,
    ResponseReceived,
    RetryScheduled,
    RequestDenied,
    RequestFailed,
    DuplicateResolved,
}

impl SdkDiagnosticEventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequestBuilt => "request_built",
            Self::RequestSigned => "request_signed",
            Self::RequestSent => "request_sent",
            Self::ResponseReceived => "response_received",
            Self::RetryScheduled => "retry_scheduled",
            Self::RequestDenied => "request_denied",
            Self::RequestFailed => "request_failed",
            Self::DuplicateResolved => "duplicate_resolved",
        }
    }
}

pub const SDK_PHASE5_DIAGNOSTIC_EVENTS: [SdkDiagnosticEventKind; 8] = [
    SdkDiagnosticEventKind::RequestBuilt,
    SdkDiagnosticEventKind::RequestSigned,
    SdkDiagnosticEventKind::RequestSent,
    SdkDiagnosticEventKind::ResponseReceived,
    SdkDiagnosticEventKind::RetryScheduled,
    SdkDiagnosticEventKind::RequestDenied,
    SdkDiagnosticEventKind::RequestFailed,
    SdkDiagnosticEventKind::DuplicateResolved,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkRedactedDiagnosticEvent {
    pub kind: SdkDiagnosticEventKind,
    pub trace_id: String,
    pub idempotency_key: String,
    pub credential_id: String,
    pub body_hash: String,
    pub reason_code: Option<String>,
    pub redaction_policy: &'static str,
    pub payload_redacted: bool,
    pub signature_redacted: bool,
    pub secret_refs_redacted: bool,
    pub contains_private_payload: bool,
}

impl SdkRedactedDiagnosticEvent {
    pub fn render(&self) -> String {
        format!(
            "kind={};trace_id={};idempotency_key={};credential_id={};body_hash={};reason_code={};redaction_policy={};payload=redacted;signature=redacted;secret_refs=redacted",
            self.kind.as_str(),
            self.trace_id,
            self.idempotency_key,
            self.credential_id,
            self.body_hash,
            self.reason_code.as_deref().unwrap_or("none"),
            self.redaction_policy
        )
    }
}

pub fn redacted_diagnostic_event(
    kind: SdkDiagnosticEventKind,
    signed: &SdkSignedOvergateRequest,
    reason_code: Option<String>,
    _private_payload_hint: &str,
    _raw_signature_hint: &str,
) -> SdkRedactedDiagnosticEvent {
    SdkRedactedDiagnosticEvent {
        kind,
        trace_id: signed.canonical_input.trace_id.clone(),
        idempotency_key: signed.canonical_input.idempotency_key.clone(),
        credential_id: signed.canonical_input.credential_id.clone(),
        body_hash: signed.canonical_input.body_hash.clone(),
        reason_code,
        redaction_policy: SDK_PHASE5_REDACTION_POLICY,
        payload_redacted: true,
        signature_redacted: true,
        secret_refs_redacted: true,
        contains_private_payload: false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkCredentialLifecycleStatus {
    Usable,
    Expired,
    Revoked,
    Rotated,
    MissingCredentialRef,
    MismatchedCredentialId,
    UnknownCredential,
    InsufficientCredential,
    HostSignerUnavailable,
    RetryProhibitedSigningFailure,
}

impl SdkCredentialLifecycleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Usable => "usable",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::Rotated => "rotated",
            Self::MissingCredentialRef => "missing_credential_ref",
            Self::MismatchedCredentialId => "mismatched_credential_id",
            Self::UnknownCredential => "unknown_credential",
            Self::InsufficientCredential => "insufficient_credential",
            Self::HostSignerUnavailable => "host_signer_unavailable",
            Self::RetryProhibitedSigningFailure => "retry_prohibited_signing_failure",
        }
    }

    pub fn reason_code(self) -> &'static str {
        match self {
            Self::Usable => "credential_usable",
            Self::Expired => "credential_expired",
            Self::Revoked => "credential_revoked",
            Self::Rotated => "credential_rotated",
            Self::MissingCredentialRef => "credential_ref_missing",
            Self::MismatchedCredentialId => "credential_id_mismatch",
            Self::UnknownCredential => "credential_unknown",
            Self::InsufficientCredential => "credential_insufficient",
            Self::HostSignerUnavailable => "host_signer_unavailable",
            Self::RetryProhibitedSigningFailure => "signing_failure_retry_prohibited",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkCredentialLifecycleDecision {
    pub credential_id: Option<String>,
    pub status: SdkCredentialLifecycleStatus,
    pub reason_code: &'static str,
    pub retryable: bool,
    pub terminal: bool,
    pub correction_hint: &'static str,
}

pub fn credential_lifecycle_failure(
    status: SdkCredentialLifecycleStatus,
    credential_id: Option<&str>,
    overgate_retryable_correction: bool,
) -> SdkCredentialLifecycleDecision {
    let usable = status == SdkCredentialLifecycleStatus::Usable;
    let retryable = !usable && overgate_retryable_correction;
    SdkCredentialLifecycleDecision {
        credential_id: credential_id.map(ToOwned::to_owned),
        status,
        reason_code: status.reason_code(),
        retryable,
        terminal: !usable && !retryable,
        correction_hint: if retryable {
            "retry only through explicit overgate correction path with same idempotency key"
        } else if usable {
            "credential usable"
        } else {
            "refresh credential provider state before retry"
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkPhase5Error {
    Sdk(SdkError),
    MissingSigningCapability {
        provider_id: String,
    },
    FixtureSignerNotAllowed {
        environment: &'static str,
    },
    CredentialLifecycle(SdkCredentialLifecycleDecision),
    InvalidSigningField {
        field: &'static str,
    },
    MutatedPayload {
        expected: String,
        actual: String,
    },
    WrongTenant {
        expected: String,
        actual: String,
    },
    ExpiredTimestamp {
        timestamp_ms: u64,
        now_ms: u64,
        replay_window_ms: u64,
    },
    MismatchedCredentialId {
        expected: String,
        actual: String,
    },
    UnsupportedSigningAlgorithm {
        algorithm: String,
    },
    SecretMaterialRejected {
        field: &'static str,
    },
}

impl fmt::Display for SdkPhase5Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sdk(error) => error.fmt(formatter),
            Self::MissingSigningCapability { provider_id } => {
                write!(formatter, "credential provider cannot sign: {provider_id}")
            }
            Self::FixtureSignerNotAllowed { environment } => write!(
                formatter,
                "test fixture signer is not allowed for {environment} SDK config"
            ),
            Self::CredentialLifecycle(decision) => write!(
                formatter,
                "{}: credential lifecycle failure for {:?}",
                decision.reason_code, decision.credential_id
            ),
            Self::InvalidSigningField { field } => {
                write!(formatter, "invalid canonical signing field: {field}")
            }
            Self::MutatedPayload { expected, actual } => write!(
                formatter,
                "payload hash changed after signing input was built: expected {expected}, got {actual}"
            ),
            Self::WrongTenant { expected, actual } => write!(
                formatter,
                "tenant changed after signing input was built: expected {expected}, got {actual}"
            ),
            Self::ExpiredTimestamp {
                timestamp_ms,
                now_ms,
                replay_window_ms,
            } => write!(
                formatter,
                "signing timestamp expired: timestamp_ms={timestamp_ms}, now_ms={now_ms}, replay_window_ms={replay_window_ms}"
            ),
            Self::MismatchedCredentialId { expected, actual } => write!(
                formatter,
                "credential id mismatch before signing: expected {expected}, got {actual}"
            ),
            Self::UnsupportedSigningAlgorithm { algorithm } => {
                write!(formatter, "unsupported signing algorithm: {algorithm}")
            }
            Self::SecretMaterialRejected { field } => {
                write!(formatter, "{field} contains secret-like material")
            }
        }
    }
}

impl std::error::Error for SdkPhase5Error {}

impl From<SdkError> for SdkPhase5Error {
    fn from(error: SdkError) -> Self {
        Self::Sdk(error)
    }
}

fn require_phase5_non_empty(value: &str, field: &'static str) -> Result<(), SdkPhase5Error> {
    if value.trim().is_empty() {
        return Err(SdkPhase5Error::InvalidSigningField { field });
    }
    Ok(())
}

fn reject_phase5_secret_like_value(field: &'static str, value: &str) -> Result<(), SdkPhase5Error> {
    if contains_phase5_secret_like_value(value) {
        return Err(SdkPhase5Error::SecretMaterialRejected { field });
    }
    Ok(())
}

fn contains_phase5_secret_like_value(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "-----begin",
        "private_key",
        "raw_secret",
        "seed_phrase",
        "bearer ",
        "bearer_",
        "token=",
        "api_key=",
        "vault_value",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}

fn canonical_token(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn canonical_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('=', "\\=")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        build_command, SdkCommandBuildInput, SdkCommandClass, SdkCommandPayload, SdkConfigInput,
        SdkServiceCapabilityProfile, DEFAULT_MAX_RETRIES, DEFAULT_TIMEOUT_MS,
    };
    use overrid_contracts::{
        BootstrapCommandFamily, CredentialReference, SUPPORTED_SCHEMA_VERSION,
    };

    const PHASE5_TIMESTAMP_MS: u64 = 1_782_018_000_000;

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

    fn config_input(
        environment: EnvironmentClass,
        credential: CredentialReference,
        test_fixtures_enabled: bool,
    ) -> SdkConfigInput {
        let base_url = match environment {
            EnvironmentClass::Local | EnvironmentClass::Ci => "http://127.0.0.1:18080/overgate",
            EnvironmentClass::Seed => "https://overgate.seed.overrid.local",
            EnvironmentClass::Staging => "https://overgate.staging.overrid.local",
            EnvironmentClass::ProductionLike => "https://overgate.production.overrid.local",
        };
        SdkConfigInput {
            environment: Some(environment),
            base_url: base_url.to_owned(),
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
            client_identity_ref: "client:overrid-sdk:phase5".to_owned(),
            credential_ref: credential,
            service_capability_profile: SdkServiceCapabilityProfile::phase2_local(),
            live_endpoint_confirmed: matches!(
                environment,
                EnvironmentClass::Seed
                    | EnvironmentClass::Staging
                    | EnvironmentClass::ProductionLike
            ),
            test_fixtures_enabled,
        }
    }

    fn local_provider() -> SdkCredentialProvider {
        let config = SdkConfigRecord::from_input(config_input(
            EnvironmentClass::Local,
            fixture_credential(),
            true,
        ))
        .unwrap();
        SdkCredentialProvider::from_config(&config).unwrap()
    }

    fn phase5_command(provider: &SdkCredentialProvider) -> SdkCommandEnvelope {
        let signature_ref = phase5_signature_ref(provider, "tenant create").unwrap();
        build_command(SdkCommandBuildInput {
            family: BootstrapCommandFamily::Tenant,
            command_type: "tenant create".to_owned(),
            tenant_id: "tenant_local".to_owned(),
            actor_id: "actor_local".to_owned(),
            target_ref: "tenant:new".to_owned(),
            payload: SdkCommandPayload::new(
                "tenant",
                vec![
                    ("display_name".to_owned(), "Local Tenant".to_owned()),
                    ("tenant_id".to_owned(), "tenant_local".to_owned()),
                ],
            )
            .unwrap(),
            expected_state: Some("pending".to_owned()),
            reason: Some("phase5 signing test".to_owned()),
            idempotency_key: "idem_phase5".to_owned(),
            trace_id: "trace_phase5".to_owned(),
            timestamp_ms: PHASE5_TIMESTAMP_MS,
            command_deadline_at_ms: Some(PHASE5_TIMESTAMP_MS + 120_000),
            signature_ref,
            schema_version: SUPPORTED_SCHEMA_VERSION.to_owned(),
            command_class: SdkCommandClass::Phase1Mutating,
        })
        .unwrap()
    }

    #[test]
    fn phase5_provider_interfaces_describe_capabilities_without_private_material() {
        let provider = local_provider();
        assert_eq!(
            provider.kind,
            SdkCredentialProviderKind::FileBackedLocalTest
        );
        assert!(provider.test_only);
        assert!(!provider.production_capable);
        assert!(!provider.stores_private_material);
        assert!(!provider.stores_bearer_token);
        assert!(!provider.stores_seed_phrase);
        assert!(!provider.stores_vault_value);
        assert_eq!(
            provider.signing_capability.algorithm,
            SDK_PHASE5_SIGNING_ALGORITHM
        );
        assert!(provider
            .public_metadata
            .contains(&("credential_class".to_owned(), "fixture".to_owned())));

        let config = SdkConfigRecord::from_input(config_input(
            EnvironmentClass::Seed,
            keychain_credential(),
            false,
        ))
        .unwrap();
        let provider = SdkCredentialProvider::from_config(&config).unwrap();
        assert_eq!(provider.kind, SdkCredentialProviderKind::PlatformKeychain);
        assert!(provider.production_capable);
        assert!(!provider.test_only);
    }

    #[test]
    fn phase5_canonical_signing_input_is_stable_and_rejects_mutations() {
        let provider = local_provider();
        let command = phase5_command(&provider);
        let signed = sign_request(
            &command,
            &provider,
            "POST",
            "/v1/overgate/commands",
            120_000,
        )
        .unwrap();

        assert_eq!(signed.signature_ref, command.envelope.signature_ref);
        assert_eq!(signed.algorithm, SDK_PHASE5_SIGNING_ALGORITHM);
        assert!(signed
            .canonical_input
            .canonical_string()
            .starts_with("method=POST\npath=/v1/overgate/commands\nbody_hash=hash_"));
        assert!(signed
            .canonical_input
            .canonical_string()
            .contains("credential_id=fixture://local-dev/key-1"));
        assert!(signed
            .canonical_input
            .canonical_bytes()
            .starts_with(b"method=POST"));

        validate_signed_request_invariants(
            &signed,
            &command,
            &provider,
            &command.payload_hash,
            "tenant_local",
            PHASE5_TIMESTAMP_MS + 1_000,
        )
        .unwrap();

        let mut mutated = signed.clone();
        mutated.canonical_input.body_hash = "hash_mutated".to_owned();
        assert!(matches!(
            validate_signed_request_invariants(
                &mutated,
                &command,
                &provider,
                &command.payload_hash,
                "tenant_local",
                PHASE5_TIMESTAMP_MS + 1_000,
            ),
            Err(SdkPhase5Error::MutatedPayload { .. })
        ));

        assert!(matches!(
            validate_signed_request_invariants(
                &signed,
                &command,
                &provider,
                &command.payload_hash,
                "tenant_other",
                PHASE5_TIMESTAMP_MS + 1_000,
            ),
            Err(SdkPhase5Error::WrongTenant { .. })
        ));

        assert!(matches!(
            validate_signed_request_invariants(
                &signed,
                &command,
                &provider,
                &command.payload_hash,
                "tenant_local",
                PHASE5_TIMESTAMP_MS + 120_001,
            ),
            Err(SdkPhase5Error::ExpiredTimestamp { .. })
        ));
    }

    #[test]
    fn phase5_test_signers_are_structurally_separate_from_production() {
        let provider = local_provider();
        let gate =
            validate_fixture_signer_installation(EnvironmentClass::Local, true, &provider).unwrap();
        assert!(gate.requires_explicit_installation);
        assert!(gate.fixture_provider);
        assert!(!gate.production_configurable);
        assert_eq!(gate.module_path, SDK_PHASE5_TEST_SIGNER_MODULE);

        assert!(matches!(
            validate_fixture_signer_installation(EnvironmentClass::Local, false, &provider),
            Err(SdkPhase5Error::FixtureSignerNotAllowed {
                environment: "local"
            })
        ));
        assert!(matches!(
            validate_fixture_signer_installation(EnvironmentClass::ProductionLike, true, &provider),
            Err(SdkPhase5Error::FixtureSignerNotAllowed {
                environment: "production_like"
            })
        ));
    }

    #[test]
    fn phase5_redacted_diagnostics_never_emit_secrets_or_signatures() {
        let provider = local_provider();
        let command = phase5_command(&provider);
        let signed = sign_request(
            &command,
            &provider,
            "POST",
            "/v1/overgate/commands",
            120_000,
        )
        .unwrap();

        let rendered = SDK_PHASE5_DIAGNOSTIC_EVENTS
            .iter()
            .map(|kind| {
                redacted_diagnostic_event(
                    *kind,
                    &signed,
                    Some("phase5_fixture".to_owned()),
                    "sentinel_private_key=do-not-leak",
                    "sentinel_signature_raw_do_not_leak",
                )
                .render()
            })
            .collect::<Vec<_>>()
            .join("\n");

        for expected in [
            "request_built",
            "request_signed",
            "request_sent",
            "response_received",
            "retry_scheduled",
            "request_denied",
            "request_failed",
            "duplicate_resolved",
            "trace_phase5",
            "idem_phase5",
        ] {
            assert!(rendered.contains(expected));
        }
        for forbidden in [
            "sentinel_private_key",
            "sentinel_signature_raw",
            "do-not-leak",
            "Local Tenant",
        ] {
            assert!(!rendered.contains(forbidden));
        }
    }

    #[test]
    fn phase5_credential_lifecycle_failures_are_terminal_without_explicit_retry() {
        let mut revoked = fixture_credential();
        revoked.revoked = true;
        let config =
            SdkConfigRecord::from_input(config_input(EnvironmentClass::Local, revoked, true))
                .unwrap();
        assert!(matches!(
            SdkCredentialProvider::from_config(&config),
            Err(SdkPhase5Error::CredentialLifecycle(decision))
                if decision.reason_code == "credential_revoked"
                    && decision.terminal
                    && !decision.retryable
        ));

        let host_terminal = credential_lifecycle_failure(
            SdkCredentialLifecycleStatus::HostSignerUnavailable,
            Some("signer://local/socket"),
            false,
        );
        assert!(host_terminal.terminal);
        assert!(!host_terminal.retryable);

        let host_retryable = credential_lifecycle_failure(
            SdkCredentialLifecycleStatus::HostSignerUnavailable,
            Some("signer://local/socket"),
            true,
        );
        assert!(!host_retryable.terminal);
        assert!(host_retryable.retryable);

        for status in [
            SdkCredentialLifecycleStatus::Rotated,
            SdkCredentialLifecycleStatus::MissingCredentialRef,
            SdkCredentialLifecycleStatus::MismatchedCredentialId,
            SdkCredentialLifecycleStatus::UnknownCredential,
            SdkCredentialLifecycleStatus::InsufficientCredential,
            SdkCredentialLifecycleStatus::RetryProhibitedSigningFailure,
        ] {
            let decision = credential_lifecycle_failure(status, Some("credential:test"), false);
            assert!(decision.terminal);
            assert!(!decision.retryable);
            assert_ne!(decision.reason_code, "credential_usable");
        }
    }

    #[test]
    fn phase5_signing_rejects_mismatched_credentials_and_unsupported_algorithms() {
        let provider = local_provider();
        let command = phase5_command(&provider);
        let config = SdkConfigRecord::from_input(config_input(
            EnvironmentClass::Seed,
            keychain_credential(),
            false,
        ))
        .unwrap();
        let keychain_provider = SdkCredentialProvider::from_config(&config).unwrap();

        assert!(matches!(
            sign_request(
                &command,
                &keychain_provider,
                "POST",
                "/v1/overgate/commands",
                120_000,
            ),
            Err(SdkPhase5Error::MismatchedCredentialId { .. })
        ));

        let mut signed = sign_request(
            &command,
            &provider,
            "POST",
            "/v1/overgate/commands",
            120_000,
        )
        .unwrap();
        signed.algorithm = "rsa-unsupported".to_owned();
        assert!(matches!(
            validate_signed_request_invariants(
                &signed,
                &command,
                &provider,
                &command.payload_hash,
                "tenant_local",
                PHASE5_TIMESTAMP_MS + 1_000,
            ),
            Err(SdkPhase5Error::UnsupportedSigningAlgorithm { algorithm })
                if algorithm == "rsa-unsupported"
        ));

        let mut unsafe_credential = fixture_credential();
        unsafe_credential.key_id = "private_key_value".to_owned();
        let config = SdkConfigRecord::from_input(config_input(
            EnvironmentClass::Local,
            unsafe_credential,
            true,
        ));
        assert!(matches!(
            config,
            Err(SdkError::SecretMaterialRejected {
                field: "credential key id"
            })
        ));
    }
}
