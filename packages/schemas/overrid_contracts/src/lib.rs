#![forbid(unsafe_code)]

use std::fmt;

pub const CONTRACT_SOURCE_ROOT: &str = "packages/schemas";
pub const CLI_SCHEMA_FAMILY: &str = "cli-command";
pub const SUPPORTED_SCHEMA_VERSION: &str = "cli-command.v0.1";
pub const GENERATED_CONTRACT_STATUS: &str = "rust_projection_from_json_schema_source";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedContractSet {
    pub schema_family: &'static str,
    pub schema_version: &'static str,
    pub source_root: &'static str,
    pub projection_status: &'static str,
}

pub fn cli_contract_set() -> GeneratedContractSet {
    GeneratedContractSet {
        schema_family: CLI_SCHEMA_FAMILY,
        schema_version: SUPPORTED_SCHEMA_VERSION,
        source_root: CONTRACT_SOURCE_ROOT,
        projection_status: GENERATED_CONTRACT_STATUS,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaVersion {
    raw: String,
    family: String,
    major: u16,
    minor: u16,
}

impl SchemaVersion {
    pub fn parse(raw: &str) -> Result<Self, ContractError> {
        if raw.trim().is_empty() {
            return Err(ContractError::MissingSchemaVersion);
        }

        let (family, version) = raw
            .split_once(".v")
            .ok_or_else(|| ContractError::InvalidSchemaVersion(raw.to_owned()))?;
        let (major, minor) = version
            .split_once('.')
            .ok_or_else(|| ContractError::InvalidSchemaVersion(raw.to_owned()))?;

        let major = major
            .parse::<u16>()
            .map_err(|_| ContractError::InvalidSchemaVersion(raw.to_owned()))?;
        let minor = minor
            .parse::<u16>()
            .map_err(|_| ContractError::InvalidSchemaVersion(raw.to_owned()))?;

        Ok(Self {
            raw: raw.to_owned(),
            family: family.to_owned(),
            major,
            minor,
        })
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn minor(&self) -> u16 {
        self.minor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    MissingSchemaVersion,
    InvalidSchemaVersion(String),
    UnsupportedSchemaVersion {
        provided: String,
        supported: &'static str,
    },
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSchemaVersion => formatter.write_str("schema version is required"),
            Self::InvalidSchemaVersion(raw) => write!(formatter, "invalid schema version: {raw}"),
            Self::UnsupportedSchemaVersion {
                provided,
                supported,
            } => write!(
                formatter,
                "unsupported schema version {provided}; supported version is {supported}"
            ),
        }
    }
}

impl std::error::Error for ContractError {}

pub fn ensure_supported_schema_version(raw: &str) -> Result<SchemaVersion, ContractError> {
    let parsed = SchemaVersion::parse(raw)?;
    let supported = SchemaVersion::parse(SUPPORTED_SCHEMA_VERSION)?;
    if parsed.family() != CLI_SCHEMA_FAMILY
        || parsed.major() != supported.major()
        || parsed.minor() > supported.minor()
    {
        return Err(ContractError::UnsupportedSchemaVersion {
            provided: raw.to_owned(),
            supported: SUPPORTED_SCHEMA_VERSION,
        });
    }
    Ok(parsed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentClass {
    Local,
    Seed,
    Staging,
    ProductionLike,
    Ci,
}

impl EnvironmentClass {
    pub fn parse(raw: &str) -> Result<Self, ProfileValidationError> {
        match raw {
            "local" => Ok(Self::Local),
            "seed" => Ok(Self::Seed),
            "staging" => Ok(Self::Staging),
            "production_like" => Ok(Self::ProductionLike),
            "ci" => Ok(Self::Ci),
            other => Err(ProfileValidationError::UnknownEnvironment(other.to_owned())),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Seed => "seed",
            Self::Staging => "staging",
            Self::ProductionLike => "production_like",
            Self::Ci => "ci",
        }
    }

    pub fn locks_endpoint_override(self) -> bool {
        matches!(self, Self::Seed | Self::ProductionLike)
    }

    pub fn requires_profile_confirmation(self) -> bool {
        matches!(self, Self::Seed | Self::ProductionLike)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialReferenceClass {
    Keychain,
    SecretService,
    EncryptedStore,
    SigningAgent,
    HardwareToken,
    Fixture,
    CiReference,
}

impl CredentialReferenceClass {
    pub fn parse(raw: &str) -> Result<Self, ProfileValidationError> {
        match raw {
            "keychain" => Ok(Self::Keychain),
            "secret_service" | "secret-service" => Ok(Self::SecretService),
            "encrypted_store" | "encrypted-store" => Ok(Self::EncryptedStore),
            "signing_agent" | "signing-agent" => Ok(Self::SigningAgent),
            "hardware_token" | "hardware-token" => Ok(Self::HardwareToken),
            "fixture" => Ok(Self::Fixture),
            "ci_reference" | "ci-reference" => Ok(Self::CiReference),
            other => Err(ProfileValidationError::UnknownCredentialClass(
                other.to_owned(),
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Keychain => "keychain",
            Self::SecretService => "secret_service",
            Self::EncryptedStore => "encrypted_store",
            Self::SigningAgent => "signing_agent",
            Self::HardwareToken => "hardware_token",
            Self::Fixture => "fixture",
            Self::CiReference => "ci_reference",
        }
    }

    pub fn is_fixture(self) -> bool {
        matches!(self, Self::Fixture)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureAllowance {
    Denied,
    LocalOnly,
    TestHarnessOnly,
}

impl FixtureAllowance {
    pub fn parse(raw: &str) -> Result<Self, ProfileValidationError> {
        match raw {
            "denied" => Ok(Self::Denied),
            "local_only" => Ok(Self::LocalOnly),
            "test_harness_only" => Ok(Self::TestHarnessOnly),
            other => Err(ProfileValidationError::UnknownFixtureAllowance(
                other.to_owned(),
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Denied => "denied",
            Self::LocalOnly => "local_only",
            Self::TestHarnessOnly => "test_harness_only",
        }
    }

    pub fn allows_fixture(self, environment: EnvironmentClass, test_harness_profile: bool) -> bool {
        match self {
            Self::Denied => false,
            Self::LocalOnly => environment == EnvironmentClass::Local,
            Self::TestHarnessOnly => {
                test_harness_profile && environment != EnvironmentClass::ProductionLike
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationPolicy {
    Standard,
    ConfirmSensitive,
    AlwaysConfirm,
}

impl ConfirmationPolicy {
    pub fn parse(raw: &str) -> Result<Self, ProfileValidationError> {
        match raw {
            "standard" => Ok(Self::Standard),
            "confirm_sensitive" => Ok(Self::ConfirmSensitive),
            "always_confirm" => Ok(Self::AlwaysConfirm),
            other => Err(ProfileValidationError::UnknownConfirmationPolicy(
                other.to_owned(),
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::ConfirmSensitive => "confirm_sensitive",
            Self::AlwaysConfirm => "always_confirm",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandLifecycleState {
    Parsed,
    ProfileLoaded,
    CredentialReady,
    PayloadValidated,
    Signed,
    Submitted,
    Accepted,
    Waiting,
    Completed,
    Denied,
    Failed,
}

impl CommandLifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Parsed => "parsed",
            Self::ProfileLoaded => "profile_loaded",
            Self::CredentialReady => "credential_ready",
            Self::PayloadValidated => "payload_validated",
            Self::Signed => "signed",
            Self::Submitted => "submitted",
            Self::Accepted => "accepted",
            Self::Waiting => "waiting",
            Self::Completed => "completed",
            Self::Denied => "denied",
            Self::Failed => "failed",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Denied | Self::Failed)
    }
}

pub const ALL_COMMAND_LIFECYCLE_STATES: [CommandLifecycleState; 11] = [
    CommandLifecycleState::Parsed,
    CommandLifecycleState::ProfileLoaded,
    CommandLifecycleState::CredentialReady,
    CommandLifecycleState::PayloadValidated,
    CommandLifecycleState::Signed,
    CommandLifecycleState::Submitted,
    CommandLifecycleState::Accepted,
    CommandLifecycleState::Waiting,
    CommandLifecycleState::Completed,
    CommandLifecycleState::Denied,
    CommandLifecycleState::Failed,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandLifecycle {
    pub states: Vec<CommandLifecycleState>,
}

impl CommandLifecycle {
    pub fn new(states: Vec<CommandLifecycleState>) -> Self {
        Self { states }
    }

    pub fn terminal_state(&self) -> Option<CommandLifecycleState> {
        self.states
            .iter()
            .rev()
            .copied()
            .find(|state| state.is_terminal())
    }

    pub fn has_terminal_state(&self) -> bool {
        self.terminal_state().is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryClass {
    NotRetryable,
    SafeRetry,
    RetryAfter,
    OperatorReview,
}

impl RetryClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotRetryable => "not_retryable",
            Self::SafeRetry => "safe_retry",
            Self::RetryAfter => "retry_after",
            Self::OperatorReview => "operator_review",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCodeClass {
    Success,
    Usage,
    Config,
    Credential,
    Schema,
    Policy,
    Phase,
    Idempotency,
    Transport,
    Timeout,
    Platform,
    LocalIo,
}

impl ExitCodeClass {
    pub const fn code(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::Usage => 2,
            Self::Config => 3,
            Self::Credential => 4,
            Self::Schema => 5,
            Self::Policy => 6,
            Self::Phase => 7,
            Self::Idempotency => 8,
            Self::Transport => 9,
            Self::Timeout => 10,
            Self::Platform => 11,
            Self::LocalIo => 12,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Usage => "usage",
            Self::Config => "config",
            Self::Credential => "credential",
            Self::Schema => "schema",
            Self::Policy => "policy",
            Self::Phase => "phase",
            Self::Idempotency => "idempotency",
            Self::Transport => "transport",
            Self::Timeout => "timeout",
            Self::Platform => "platform",
            Self::LocalIo => "local_io",
        }
    }
}

pub const EXIT_CODE_REGISTRY: [ExitCodeClass; 12] = [
    ExitCodeClass::Success,
    ExitCodeClass::Usage,
    ExitCodeClass::Config,
    ExitCodeClass::Credential,
    ExitCodeClass::Schema,
    ExitCodeClass::Policy,
    ExitCodeClass::Phase,
    ExitCodeClass::Idempotency,
    ExitCodeClass::Transport,
    ExitCodeClass::Timeout,
    ExitCodeClass::Platform,
    ExitCodeClass::LocalIo,
];

pub fn exit_code_class_for_code(code: i32) -> Option<ExitCodeClass> {
    EXIT_CODE_REGISTRY
        .iter()
        .copied()
        .find(|exit_class| exit_class.code() == code)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilitySnapshot {
    pub route: String,
    pub available: bool,
    pub phase_gate: String,
    pub schema_versions: Vec<String>,
    pub stale_age_ms: u64,
    pub fail_closed: bool,
}

impl CapabilitySnapshot {
    pub fn local_phase_gate(route: impl Into<String>, phase_gate: impl Into<String>) -> Self {
        Self {
            route: route.into(),
            available: false,
            phase_gate: phase_gate.into(),
            schema_versions: vec![SUPPORTED_SCHEMA_VERSION.to_owned()],
            stale_age_ms: 0,
            fail_closed: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandContext {
    pub command_name: String,
    pub trace_context: TraceContext,
    pub lifecycle: CommandLifecycle,
    pub capability_snapshot: Option<CapabilitySnapshot>,
}

impl CommandContext {
    pub fn new(
        command_name: impl Into<String>,
        trace_id: impl Into<String>,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            command_name: command_name.into(),
            trace_context: TraceContext::new(trace_id, SUPPORTED_SCHEMA_VERSION)?,
            lifecycle: CommandLifecycle::new(vec![CommandLifecycleState::Parsed]),
            capability_snapshot: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliProfile {
    pub name: String,
    pub endpoint: String,
    pub endpoint_fingerprint: String,
    pub environment: EnvironmentClass,
    pub tenant_id: String,
    pub actor_id: String,
    pub credential_namespace: String,
    pub allowed_credential_classes: Vec<CredentialReferenceClass>,
    pub fixture_allowance: FixtureAllowance,
    pub default_output_mode: String,
    pub confirmation_policy: ConfirmationPolicy,
    pub schema_pins: Vec<String>,
    pub test_harness_profile: bool,
}

impl CliProfile {
    pub fn validate(&self) -> Result<(), ProfileValidationError> {
        require_non_empty(&self.name, "profile name")?;
        require_non_empty(&self.endpoint, "endpoint")?;
        require_non_empty(&self.endpoint_fingerprint, "endpoint fingerprint")?;
        require_non_empty(&self.tenant_id, "tenant id")?;
        require_non_empty(&self.actor_id, "actor id")?;
        require_non_empty(&self.credential_namespace, "credential namespace")?;

        if self.allowed_credential_classes.is_empty() {
            return Err(ProfileValidationError::MissingCredentialClass);
        }

        match self.default_output_mode.as_str() {
            "human" | "json" => {}
            other => return Err(ProfileValidationError::InvalidOutputMode(other.to_owned())),
        }

        if self.schema_pins.is_empty() {
            return Err(ProfileValidationError::MissingSchemaPin);
        }
        for schema_pin in &self.schema_pins {
            ensure_supported_schema_version(schema_pin)
                .map_err(ProfileValidationError::UnsupportedSchemaPin)?;
        }

        let fixture_allowed = self
            .fixture_allowance
            .allows_fixture(self.environment, self.test_harness_profile);
        if self
            .allowed_credential_classes
            .iter()
            .any(|class| class.is_fixture())
            && !fixture_allowed
        {
            return Err(ProfileValidationError::FixtureCredentialNotAllowed {
                environment: self.environment.as_str(),
            });
        }

        if self.environment.locks_endpoint_override()
            && self.endpoint_fingerprint.eq_ignore_ascii_case("unknown")
        {
            return Err(ProfileValidationError::MissingEndpointFingerprint);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialReference {
    pub reference_id: String,
    pub class: CredentialReferenceClass,
    pub namespace: String,
    pub key_id: String,
    pub revoked: bool,
    pub expired: bool,
}

impl CredentialReference {
    pub fn validate_for_profile(&self, profile: &CliProfile) -> Result<(), ProfileValidationError> {
        profile.validate()?;
        require_non_empty(&self.reference_id, "credential reference")?;
        require_non_empty(&self.namespace, "credential namespace")?;
        require_non_empty(&self.key_id, "key id")?;

        for value in [&self.reference_id, &self.namespace, &self.key_id] {
            if contains_raw_secret_marker(value) {
                return Err(ProfileValidationError::RawKeyMaterialInReference);
            }
        }

        if self.namespace != profile.credential_namespace {
            return Err(ProfileValidationError::CredentialNamespaceMismatch {
                expected: profile.credential_namespace.clone(),
                actual: self.namespace.clone(),
            });
        }

        if !profile.allowed_credential_classes.contains(&self.class) {
            return Err(ProfileValidationError::CredentialClassNotAllowed {
                class: self.class.as_str(),
            });
        }

        if self.class.is_fixture()
            && !profile
                .fixture_allowance
                .allows_fixture(profile.environment, profile.test_harness_profile)
        {
            return Err(ProfileValidationError::FixtureCredentialNotAllowed {
                environment: profile.environment.as_str(),
            });
        }

        if self.revoked {
            return Err(ProfileValidationError::CredentialRevoked);
        }
        if self.expired {
            return Err(ProfileValidationError::CredentialExpired);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignerHandoff {
    pub credential_reference_id: String,
    pub key_id: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub signature_ref: String,
    pub exposes_key_material: bool,
}

impl SignerHandoff {
    pub fn new(
        profile: &CliProfile,
        credential: &CredentialReference,
        command_type: &str,
    ) -> Result<Self, ProfileValidationError> {
        require_non_empty(command_type, "command type")?;
        credential.validate_for_profile(profile)?;
        let normalized_command = command_type.replace([' ', '/'], "_");
        Ok(Self {
            credential_reference_id: credential.reference_id.clone(),
            key_id: credential.key_id.clone(),
            tenant_id: profile.tenant_id.clone(),
            actor_id: profile.actor_id.clone(),
            signature_ref: format!(
                "sigref:{}:{}:{}",
                credential.namespace, credential.key_id, normalized_command
            ),
            exposes_key_material: false,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootstrapCommandFamily {
    Auth,
    Tenant,
    Identity,
    Key,
    Manifest,
    Workload,
    Node,
    Policy,
    Package,
    Usage,
    Receipt,
    Ledger,
    Dispute,
}

impl BootstrapCommandFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auth => "auth",
            Self::Tenant => "tenant",
            Self::Identity => "identity",
            Self::Key => "key",
            Self::Manifest => "manifest",
            Self::Workload => "workload",
            Self::Node => "node",
            Self::Policy => "policy",
            Self::Package => "package",
            Self::Usage => "usage",
            Self::Receipt => "receipt",
            Self::Ledger => "ledger",
            Self::Dispute => "dispute",
        }
    }

    pub fn phase_gate(self) -> &'static str {
        match self {
            Self::Node => "phase_2_seed_private_swarm",
            Self::Policy => "phase_4_trust_policy_verification",
            Self::Package => "phase_9_overpack_deployment_platform",
            Self::Usage | Self::Receipt | Self::Ledger | Self::Dispute => {
                "phase_5_metering_accounting"
            }
            _ => "phase_1_control_plane_bootstrap",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedCommandEnvelope {
    pub family: BootstrapCommandFamily,
    pub command_type: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub target_ref: String,
    pub payload_type: String,
    pub expected_state: Option<String>,
    pub reason: Option<String>,
    pub idempotency: IdempotencyRecord,
    pub trace_context: TraceContext,
    pub signature_ref: String,
    pub exposes_key_material: bool,
}

impl SignedCommandEnvelope {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        family: BootstrapCommandFamily,
        command_type: impl Into<String>,
        tenant_id: impl Into<String>,
        actor_id: impl Into<String>,
        target_ref: impl Into<String>,
        payload_type: impl Into<String>,
        expected_state: Option<String>,
        reason: Option<String>,
        idempotency_key: impl Into<String>,
        trace_id: impl Into<String>,
        signature_ref: impl Into<String>,
    ) -> Result<Self, ContractError> {
        let command_type = command_type.into();
        Ok(Self {
            family,
            tenant_id: tenant_id.into(),
            actor_id: actor_id.into(),
            target_ref: target_ref.into(),
            payload_type: payload_type.into(),
            expected_state,
            reason,
            idempotency: IdempotencyRecord::new(
                idempotency_key,
                command_type.clone(),
                SUPPORTED_SCHEMA_VERSION,
            )?,
            trace_context: TraceContext::new(trace_id, SUPPORTED_SCHEMA_VERSION)?,
            signature_ref: signature_ref.into(),
            command_type,
            exposes_key_material: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapAcceptanceRecord {
    pub command_type: String,
    pub accepted_ref: String,
    pub phase_gate: String,
    pub pending_state: String,
    pub audit_refs: Vec<String>,
}

impl BootstrapAcceptanceRecord {
    pub fn new(
        command_type: impl Into<String>,
        accepted_ref: impl Into<String>,
        pending_state: impl Into<String>,
        audit_refs: Vec<String>,
    ) -> Self {
        Self {
            command_type: command_type.into(),
            accepted_ref: accepted_ref.into(),
            phase_gate: "phase_1_control_plane_bootstrap".to_owned(),
            pending_state: pending_state.into(),
            audit_refs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestBootstrapRef {
    pub manifest_ref: String,
    pub manifest_kind: String,
    pub immutable_ref: String,
    pub submitted_via: String,
}

impl ManifestBootstrapRef {
    pub fn new(manifest_ref: impl Into<String>, manifest_kind: impl Into<String>) -> Self {
        let manifest_ref = manifest_ref.into();
        let manifest_kind = manifest_kind.into();
        Self {
            immutable_ref: format!("manifest:{}:immutable", manifest_ref),
            manifest_ref,
            manifest_kind,
            submitted_via: "sdk_overgate_contract".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntheticWorkloadPendingState {
    pub workload_ref: String,
    pub workload_kind: String,
    pub queue_state: String,
    pub execution_implied: bool,
    pub timeline_refs: Vec<String>,
}

impl SyntheticWorkloadPendingState {
    pub fn pending(workload_ref: impl Into<String>, workload_kind: impl Into<String>) -> Self {
        let workload_ref = workload_ref.into();
        Self {
            timeline_refs: vec![format!("timeline:{}:pending", workload_ref)],
            workload_ref,
            workload_kind: workload_kind.into(),
            queue_state: "pending".to_owned(),
            execution_implied: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Live,
    Stale,
    Expired,
    Draining,
    Disabled,
}

impl NodeState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Draining => "draining",
            Self::Disabled => "disabled",
        }
    }

    pub fn accepts_work(self) -> bool {
        matches!(self, Self::Live)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeStatusRecord {
    pub node_ref: String,
    pub state: NodeState,
    pub profile_name: String,
    pub credential_ref: String,
    pub credential_checked: bool,
    pub capability_refs: Vec<String>,
    pub heartbeat_ref: String,
    pub registered_via: String,
    pub direct_node_access: bool,
}

impl NodeStatusRecord {
    pub fn new(
        node_ref: impl Into<String>,
        state: NodeState,
        profile_name: impl Into<String>,
        credential_ref: impl Into<String>,
    ) -> Self {
        let node_ref = node_ref.into();
        Self {
            heartbeat_ref: format!("overwatch:heartbeat:{node_ref}"),
            capability_refs: vec![
                format!("overcell:node:{node_ref}:capabilities"),
                format!("overregistry:node:{node_ref}:profile"),
            ],
            node_ref,
            state,
            profile_name: profile_name.into(),
            credential_ref: credential_ref.into(),
            credential_checked: true,
            registered_via: "sdk_overgate_contract".to_owned(),
            direct_node_access: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadExecutionState {
    Scheduled,
    Leased,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
    DeadLettered,
}

impl WorkloadExecutionState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Scheduled => "scheduled",
            Self::Leased => "leased",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::DeadLettered => "dead_lettered",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded | Self::Failed | Self::Cancelled | Self::TimedOut | Self::DeadLettered
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionDiagnosticEvent {
    pub state: WorkloadExecutionState,
    pub service_ref: String,
    pub reason_code: String,
    pub evidence_ref: String,
}

impl ExecutionDiagnosticEvent {
    pub fn new(
        state: WorkloadExecutionState,
        service_ref: impl Into<String>,
        reason_code: impl Into<String>,
        evidence_ref: impl Into<String>,
    ) -> Self {
        Self {
            state,
            service_ref: service_ref.into(),
            reason_code: reason_code.into(),
            evidence_ref: evidence_ref.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionTimeline {
    pub workload_ref: String,
    pub states: Vec<WorkloadExecutionState>,
    pub diagnostic_events: Vec<ExecutionDiagnosticEvent>,
    pub owning_service_refs: Vec<String>,
    pub trace_id: String,
    pub direct_node_access: bool,
}

impl ExecutionTimeline {
    pub fn new(
        workload_ref: impl Into<String>,
        states: Vec<WorkloadExecutionState>,
        trace_id: impl Into<String>,
    ) -> Self {
        let workload_ref = workload_ref.into();
        Self {
            diagnostic_events: vec![
                ExecutionDiagnosticEvent::new(
                    WorkloadExecutionState::Scheduled,
                    "oversched:scheduler",
                    "scheduler.accepted",
                    format!("trace:{workload_ref}:scheduler"),
                ),
                ExecutionDiagnosticEvent::new(
                    WorkloadExecutionState::Leased,
                    "overlease:lease",
                    "lease.active",
                    format!("trace:{workload_ref}:lease"),
                ),
                ExecutionDiagnosticEvent::new(
                    WorkloadExecutionState::Running,
                    "overrun:runner",
                    "runner.started",
                    format!("trace:{workload_ref}:runner"),
                ),
                ExecutionDiagnosticEvent::new(
                    WorkloadExecutionState::Running,
                    "overcell:node-heartbeat",
                    "node.heartbeat.live",
                    format!("trace:{workload_ref}:node_heartbeat"),
                ),
                ExecutionDiagnosticEvent::new(
                    WorkloadExecutionState::Running,
                    "overpack:package",
                    "package.resolved",
                    format!("trace:{workload_ref}:package"),
                ),
                ExecutionDiagnosticEvent::new(
                    states
                        .last()
                        .copied()
                        .unwrap_or(WorkloadExecutionState::Succeeded),
                    "overstore:result-state-ref",
                    result_state_reason_code(
                        states
                            .last()
                            .copied()
                            .unwrap_or(WorkloadExecutionState::Succeeded),
                    ),
                    format!("trace:{workload_ref}:result_state"),
                ),
            ],
            owning_service_refs: vec![
                "overgate:execution-command".to_owned(),
                "overqueue:workload-state".to_owned(),
                "oversched:scheduler".to_owned(),
                "overlease:lease".to_owned(),
                "overrun:runner".to_owned(),
                "overcell:node-heartbeat".to_owned(),
                "overpack:package".to_owned(),
                "overstore:result-state-ref".to_owned(),
                "overwatch:trace".to_owned(),
            ],
            workload_ref,
            states,
            trace_id: trace_id.into(),
            direct_node_access: false,
        }
    }
}

fn result_state_reason_code(state: WorkloadExecutionState) -> &'static str {
    match state {
        WorkloadExecutionState::Scheduled => "result.scheduled",
        WorkloadExecutionState::Leased => "result.leased",
        WorkloadExecutionState::Running => "result.pending",
        WorkloadExecutionState::Succeeded => "result.ref.available",
        WorkloadExecutionState::Failed => "result.failed",
        WorkloadExecutionState::Cancelled => "result.cancelled",
        WorkloadExecutionState::TimedOut => "result.timed_out",
        WorkloadExecutionState::DeadLettered => "result.dead_lettered",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionLogBundle {
    pub workload_ref: String,
    pub log_ref: String,
    pub redaction_policy: String,
    pub bounded_streaming: bool,
    pub trace_linked_ref: String,
    pub contains_private_payload: bool,
    pub direct_node_path_exposed: bool,
}

impl ExecutionLogBundle {
    pub fn new(workload_ref: impl Into<String>, trace_id: impl Into<String>) -> Self {
        let workload_ref = workload_ref.into();
        Self {
            log_ref: format!("overwatch:logs:{workload_ref}:redacted"),
            trace_linked_ref: format!("trace:{}:logs", trace_id.into()),
            workload_ref,
            redaction_policy: "secret_free_refs_only".to_owned(),
            bounded_streaming: true,
            contains_private_payload: false,
            direct_node_path_exposed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResultRef {
    pub workload_ref: String,
    pub result_ref: String,
    pub authorized_control_plane_ref: String,
    pub trace_linked_ref: String,
    pub contains_private_payload: bool,
    pub direct_object_store_path_exposed: bool,
}

impl ExecutionResultRef {
    pub fn new(workload_ref: impl Into<String>, trace_id: impl Into<String>) -> Self {
        let workload_ref = workload_ref.into();
        Self {
            result_ref: format!("overstore:result:{workload_ref}:authorized_ref"),
            authorized_control_plane_ref: format!("overgate:result:{workload_ref}"),
            trace_linked_ref: format!("trace:{}:result", trace_id.into()),
            workload_ref,
            contains_private_payload: false,
            direct_object_store_path_exposed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PollingPlan {
    pub wait: bool,
    pub follow: bool,
    pub timeout_ms: u64,
    pub poll_interval_ms: u64,
    pub event_stream_preferred: bool,
    pub fallback_polling: bool,
    pub interruptible: bool,
}

impl PollingPlan {
    pub fn bounded(wait: bool, follow: bool, timeout_ms: u64, poll_interval_ms: u64) -> Self {
        Self {
            wait,
            follow,
            timeout_ms: timeout_ms.clamp(1, 600_000),
            poll_interval_ms: poll_interval_ms.clamp(100, 60_000),
            event_stream_preferred: true,
            fallback_polling: true,
            interruptible: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDryRunDecisionState {
    Accepted,
    Denied,
}

impl PolicyDryRunDecisionState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Denied => "denied",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyDryRunDecision {
    pub target_ref: String,
    pub decision: PolicyDryRunDecisionState,
    pub reason_codes: Vec<String>,
    pub workload_class: String,
    pub data_sensitivity: String,
    pub quota_ref: String,
    pub package_trust_ref: String,
    pub egress_policy_ref: String,
    pub provider_eligibility_ref: String,
    pub budget_ref: String,
    pub evaluated_via: String,
    pub mutates_platform_state: bool,
    pub direct_policy_service_access: bool,
}

impl PolicyDryRunDecision {
    pub fn new(target_ref: impl Into<String>, reason_codes: Vec<String>) -> Self {
        let target_ref = target_ref.into();
        let decision = if reason_codes.is_empty() {
            PolicyDryRunDecisionState::Accepted
        } else {
            PolicyDryRunDecisionState::Denied
        };
        Self {
            quota_ref: format!("overmeter:quota:{target_ref}"),
            package_trust_ref: format!("overguard:package_trust:{target_ref}"),
            egress_policy_ref: format!("overguard:egress:{target_ref}"),
            provider_eligibility_ref: format!("overguard:provider:{target_ref}"),
            budget_ref: format!("oru:budget:{target_ref}"),
            target_ref,
            decision,
            reason_codes,
            workload_class: "standard_compute".to_owned(),
            data_sensitivity: "low_sensitivity".to_owned(),
            evaluated_via: "sdk_overgate_contract".to_owned(),
            mutates_platform_state: false,
            direct_policy_service_access: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageValidationState {
    Accepted,
    InvalidPackage,
    UnsupportedVersion,
    MissingProvenance,
    PolicyIncompatible,
}

impl PackageValidationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::InvalidPackage => "invalid_package",
            Self::UnsupportedVersion => "unsupported_version",
            Self::MissingProvenance => "missing_provenance",
            Self::PolicyIncompatible => "policy_incompatible",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageValidationSummary {
    pub package_ref: String,
    pub validation_state: PackageValidationState,
    pub reason_codes: Vec<String>,
    pub schema_checked: bool,
    pub signature_checked: bool,
    pub hash_checked: bool,
    pub dependency_checked: bool,
    pub permission_checked: bool,
    pub provenance_available: bool,
    pub sbom_ref: String,
    pub policy_compatibility_checked: bool,
    pub submitted_via: String,
    pub direct_package_store_access: bool,
}

impl PackageValidationSummary {
    pub fn new(
        package_ref: impl Into<String>,
        validation_state: PackageValidationState,
        reason_codes: Vec<String>,
    ) -> Self {
        let package_ref = package_ref.into();
        Self {
            sbom_ref: format!("overpack:sbom:{package_ref}"),
            package_ref,
            validation_state,
            reason_codes,
            schema_checked: true,
            signature_checked: true,
            hash_checked: true,
            dependency_checked: true,
            permission_checked: true,
            provenance_available: !matches!(
                validation_state,
                PackageValidationState::MissingProvenance
            ),
            policy_compatibility_checked: true,
            submitted_via: "sdk_overgate_contract".to_owned(),
            direct_package_store_access: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageOruRollup {
    pub tenant_ref: String,
    pub usage_ref: String,
    pub units: Vec<String>,
    pub budget_state: String,
    pub disputed_usage: bool,
    pub read_via: String,
    pub payment_behavior_created: bool,
    pub direct_meter_access: bool,
}

impl UsageOruRollup {
    pub fn new(tenant_ref: impl Into<String>, usage_ref: impl Into<String>) -> Self {
        let usage_ref = usage_ref.into();
        let normalized = usage_ref.to_ascii_lowercase();
        Self {
            tenant_ref: tenant_ref.into(),
            usage_ref,
            units: [
                "CPU-ORU", "GPU-ORU", "STOR-ORU", "NET-ORU", "MEM-ORU", "DATA-ORU",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            budget_state: if normalized.contains("budget_exhausted") {
                "budget_exhausted"
            } else {
                "within_budget"
            }
            .to_owned(),
            disputed_usage: normalized.contains("disputed"),
            read_via: "sdk_overgate_contract".to_owned(),
            payment_behavior_created: false,
            direct_meter_access: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptLedgerRead {
    pub receipt_ref: String,
    pub ledger_refs: Vec<String>,
    pub invoice_status: String,
    pub refund_ref: String,
    pub correction_ref: String,
    pub payout_hold_ref: String,
    pub audit_refs: Vec<String>,
    pub read_via: String,
    pub pricing_assumptions_present: bool,
    pub revenue_assumptions_present: bool,
    pub customer_count_assumptions_present: bool,
    pub market_volume_assumptions_present: bool,
    pub direct_ledger_access: bool,
}

impl ReceiptLedgerRead {
    pub fn new(receipt_ref: impl Into<String>) -> Self {
        let receipt_ref = receipt_ref.into();
        Self {
            ledger_refs: vec![
                format!("seal-ledger:entry:{receipt_ref}"),
                format!("overbill:receipt:{receipt_ref}"),
            ],
            refund_ref: format!("overbill:refund:{receipt_ref}"),
            correction_ref: format!("seal-ledger:correction:{receipt_ref}"),
            payout_hold_ref: format!("seal-ledger:payout-hold:{receipt_ref}"),
            audit_refs: vec![format!("overwatch:audit:{receipt_ref}")],
            receipt_ref,
            invoice_status: "issued".to_owned(),
            read_via: "sdk_overgate_contract".to_owned(),
            pricing_assumptions_present: false,
            revenue_assumptions_present: false,
            customer_count_assumptions_present: false,
            market_volume_assumptions_present: false,
            direct_ledger_access: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisputeReadModel {
    pub dispute_ref: String,
    pub case_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub hold_status: String,
    pub correction_refs: Vec<String>,
    pub resolution_state: String,
    pub tenant_role_filtered: bool,
    pub read_via: String,
    pub direct_dispute_mutation: bool,
    pub direct_ledger_mutation: bool,
}

impl DisputeReadModel {
    pub fn new(dispute_ref: impl Into<String>) -> Self {
        let dispute_ref = dispute_ref.into();
        let normalized = dispute_ref.to_ascii_lowercase();
        Self {
            case_refs: vec![
                format!("overclaim:case:{dispute_ref}:primary"),
                format!("overclaim:case:{dispute_ref}:appeal"),
            ],
            evidence_refs: vec![format!("overclaim:evidence:{dispute_ref}:bundle")],
            hold_status: if normalized.contains("released") {
                "released"
            } else {
                "held"
            }
            .to_owned(),
            correction_refs: vec![format!("seal-ledger:correction:{dispute_ref}")],
            resolution_state: if normalized.contains("resolved") {
                "resolved"
            } else {
                "open"
            }
            .to_owned(),
            dispute_ref,
            tenant_role_filtered: true,
            read_via: "sdk_overgate_contract".to_owned(),
            direct_dispute_mutation: false,
            direct_ledger_mutation: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductKind {
    Docdex,
    Mcoda,
    Codali,
}

impl ProductKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Docdex => "docdex",
            Self::Mcoda => "mcoda",
            Self::Codali => "codali",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductWorkflowRecipe {
    pub product: ProductKind,
    pub workflow_ref: String,
    pub workload_kind: String,
    pub command_recipes: Vec<String>,
    pub required_refs: Vec<String>,
    pub expected_failure_modes: Vec<String>,
    pub safe_retry_patterns: Vec<String>,
    pub submitted_via: String,
    pub sdk_overgate_only: bool,
    pub authorized_refs_only: bool,
    pub secret_free_json_output: bool,
    pub dynamic_model_resource_selection: bool,
    pub direct_internal_api_access: bool,
    pub direct_storage_access: bool,
    pub raw_http_required: bool,
    pub hardcoded_model_or_provider: bool,
    pub hardcoded_node_assumption: bool,
    pub paid_service_assumption: bool,
}

impl ProductWorkflowRecipe {
    pub fn new(
        product: ProductKind,
        workflow_ref: impl Into<String>,
        workload_kind: impl Into<String>,
    ) -> Self {
        let workflow_ref = workflow_ref.into();
        let workload_kind = workload_kind.into();
        let (command_recipes, required_refs, expected_failure_modes, safe_retry_patterns) =
            match product {
                ProductKind::Docdex => (
                    vec![
                        format!(
                            "overrid workload submit --workload-kind {workload_kind} --workload-ref {workflow_ref} --json"
                        ),
                        format!("overrid workload status --workload-ref {workflow_ref} --json"),
                        format!("overrid workload result --workload-ref {workflow_ref} --json"),
                        format!(
                            "overrid workload cancel --workload-ref {workflow_ref} --reason operator_requested --json"
                        ),
                        format!("overrid usage show --target-ref usage:{workflow_ref} --json"),
                        format!("overrid receipt show --target-ref receipt:{workflow_ref} --json"),
                    ],
                    vec![
                        "encrypted_index_ref",
                        "retrieval_job_ref",
                        "search_result_ref",
                        "usage_rollup_ref",
                        "receipt_ref",
                    ],
                    vec![
                        "policy.egress_denied",
                        "budget.exhausted",
                        "credential.scope_denied",
                        "result.cancelled",
                    ],
                    vec![
                        "retry_same_idempotency_key_for_transport",
                        "resume_with_workload_ref",
                        "cancel_then_resubmit_after_policy_fix",
                    ],
                ),
                ProductKind::Mcoda => (
                    vec![
                        format!(
                            "overrid workload submit --workload-kind {workload_kind} --workload-ref {workflow_ref} --json"
                        ),
                        format!("overrid workload status --workload-ref {workflow_ref} --json"),
                        format!("overrid workload logs --workload-ref {workflow_ref} --json"),
                        format!("overrid workload result --workload-ref {workflow_ref} --json"),
                        format!(
                            "overrid workload cancel --workload-ref {workflow_ref} --reason operator_requested --json"
                        ),
                        format!("overrid usage show --target-ref usage:{workflow_ref} --json"),
                    ],
                    vec![
                        "agent_workload_ref",
                        "dynamic_model_metadata_ref",
                        "resource_metadata_ref",
                        "tool_boundary_ref",
                        "budget_ref",
                        "usage_rollup_ref",
                    ],
                    vec![
                        "policy.unsupported_workload_class",
                        "budget.exhausted",
                        "tool.boundary_denied",
                        "provider.unavailable",
                    ],
                    vec![
                        "retry_same_idempotency_key_for_safe_transport",
                        "switch_model_via_metadata_ref",
                        "operator_review_for_policy_denial",
                    ],
                ),
                ProductKind::Codali => (
                    vec![
                        format!("overrid package validate --target-ref package:{workflow_ref} --json"),
                        format!(
                            "overrid workload submit --workload-kind {workload_kind} --workload-ref {workflow_ref} --json"
                        ),
                        format!("overrid workload logs --workload-ref {workflow_ref} --json"),
                        format!("overrid workload result --workload-ref {workflow_ref} --json"),
                        format!(
                            "overrid workload cancel --workload-ref {workflow_ref} --reason operator_requested --json"
                        ),
                        format!("overrid usage show --target-ref usage:{workflow_ref} --json"),
                    ],
                    vec![
                        "code_agent_package_ref",
                        "repository_context_ref",
                        "execution_log_ref",
                        "artifact_refs",
                        "repair_boundary_ref",
                        "phase_usage_ref",
                    ],
                    vec![
                        "policy.resource_denied",
                        "package.invalid",
                        "repo_context.ref_denied",
                        "repair.boundary_exceeded",
                    ],
                    vec![
                        "retry_failed_phase_after_repair_ref",
                        "preserve_repo_context_ref",
                        "operator_review_for_policy_denial",
                    ],
                ),
            };

        Self {
            product,
            workflow_ref,
            workload_kind,
            command_recipes,
            required_refs: required_refs.into_iter().map(str::to_owned).collect(),
            expected_failure_modes: expected_failure_modes
                .into_iter()
                .map(str::to_owned)
                .collect(),
            safe_retry_patterns: safe_retry_patterns
                .into_iter()
                .map(str::to_owned)
                .collect(),
            submitted_via: "sdk_overgate_contract".to_owned(),
            sdk_overgate_only: true,
            authorized_refs_only: true,
            secret_free_json_output: true,
            dynamic_model_resource_selection: true,
            direct_internal_api_access: false,
            direct_storage_access: false,
            raw_http_required: false,
            hardcoded_model_or_provider: false,
            hardcoded_node_assumption: false,
            paid_service_assumption: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CiAutomationProfile {
    pub profile_kind: String,
    pub environment_class: EnvironmentClass,
    pub credential_reference_class: String,
    pub credential_reference_id: String,
    pub allowed_credential_ref_kinds: Vec<String>,
    pub submitted_via: String,
    pub short_lived_service_account_required: bool,
    pub ambient_persistent_keychain_allowed: bool,
    pub requires_non_interactive_confirmation: bool,
    pub json_output_stable: bool,
    pub secret_free_output: bool,
    pub branch_on_exit_class: bool,
}

impl CiAutomationProfile {
    pub fn new(
        environment_class: EnvironmentClass,
        credential_reference_class: impl Into<String>,
        credential_reference_id: impl Into<String>,
    ) -> Self {
        Self {
            profile_kind: "ci".to_owned(),
            environment_class,
            credential_reference_class: credential_reference_class.into(),
            credential_reference_id: credential_reference_id.into(),
            allowed_credential_ref_kinds: [
                "ci_reference",
                "signing_agent",
                "fixture",
                "hardware_token",
                "mounted_secret_ref",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            submitted_via: "sdk_overgate_contract".to_owned(),
            short_lived_service_account_required: true,
            ambient_persistent_keychain_allowed: false,
            requires_non_interactive_confirmation: true,
            json_output_stable: true,
            secret_free_output: true,
            branch_on_exit_class: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileValidationError {
    MissingRequiredField(&'static str),
    UnknownEnvironment(String),
    UnknownCredentialClass(String),
    UnknownFixtureAllowance(String),
    UnknownConfirmationPolicy(String),
    InvalidOutputMode(String),
    MissingCredentialClass,
    MissingSchemaPin,
    UnsupportedSchemaPin(ContractError),
    MissingEndpointFingerprint,
    FixtureCredentialNotAllowed { environment: &'static str },
    CredentialNamespaceMismatch { expected: String, actual: String },
    CredentialClassNotAllowed { class: &'static str },
    CredentialRevoked,
    CredentialExpired,
    RawKeyMaterialInReference,
    UnsafeFilePermissions(u32),
}

impl fmt::Display for ProfileValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredField(field) => write!(formatter, "{field} is required"),
            Self::UnknownEnvironment(value) => write!(formatter, "unknown environment: {value}"),
            Self::UnknownCredentialClass(value) => {
                write!(formatter, "unknown credential class: {value}")
            }
            Self::UnknownFixtureAllowance(value) => {
                write!(formatter, "unknown fixture allowance: {value}")
            }
            Self::UnknownConfirmationPolicy(value) => {
                write!(formatter, "unknown confirmation policy: {value}")
            }
            Self::InvalidOutputMode(value) => write!(formatter, "invalid output mode: {value}"),
            Self::MissingCredentialClass => formatter.write_str("credential class is required"),
            Self::MissingSchemaPin => formatter.write_str("at least one schema pin is required"),
            Self::UnsupportedSchemaPin(error) => error.fmt(formatter),
            Self::MissingEndpointFingerprint => {
                formatter.write_str("endpoint fingerprint is required for locked environments")
            }
            Self::FixtureCredentialNotAllowed { environment } => write!(
                formatter,
                "fixture credential is not allowed for {environment} profile"
            ),
            Self::CredentialNamespaceMismatch { expected, actual } => write!(
                formatter,
                "credential namespace mismatch: expected {expected}, got {actual}"
            ),
            Self::CredentialClassNotAllowed { class } => {
                write!(formatter, "credential class is not allowed: {class}")
            }
            Self::CredentialRevoked => formatter.write_str("credential is revoked"),
            Self::CredentialExpired => formatter.write_str("credential is expired"),
            Self::RawKeyMaterialInReference => {
                formatter.write_str("credential reference contains raw key material")
            }
            Self::UnsafeFilePermissions(mode) => {
                write!(formatter, "file mode {mode:o} is not owner-only")
            }
        }
    }
}

impl std::error::Error for ProfileValidationError {}

pub fn validate_owner_only_file_mode(mode: u32) -> Result<(), ProfileValidationError> {
    if mode & 0o077 == 0 {
        Ok(())
    } else {
        Err(ProfileValidationError::UnsafeFilePermissions(mode))
    }
}

fn require_non_empty(value: &str, field: &'static str) -> Result<(), ProfileValidationError> {
    if value.trim().is_empty() {
        Err(ProfileValidationError::MissingRequiredField(field))
    } else {
        Ok(())
    }
}

fn contains_raw_secret_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    value.contains("-----BEGIN")
        || lower.contains("private key")
        || lower.contains("secret=")
        || lower.contains("token=")
        || lower.contains("raw_key")
}

fn idempotency_component(value: &str) -> String {
    let mut rendered = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | ':' | '-') {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    while rendered.contains("__") {
        rendered = rendered.replace("__", "_");
    }
    rendered.trim_matches('_').to_owned()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceContext {
    pub trace_id: String,
    pub schema_version: SchemaVersion,
}

impl TraceContext {
    pub fn new(trace_id: impl Into<String>, schema_version: &str) -> Result<Self, ContractError> {
        Ok(Self {
            trace_id: trace_id.into(),
            schema_version: ensure_supported_schema_version(schema_version)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdempotencyRecord {
    pub key: String,
    pub command_type: String,
    pub schema_version: SchemaVersion,
}

impl IdempotencyRecord {
    pub fn new(
        key: impl Into<String>,
        command_type: impl Into<String>,
        schema_version: &str,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            key: key.into(),
            command_type: command_type.into(),
            schema_version: ensure_supported_schema_version(schema_version)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalIdempotencyFingerprint {
    pub environment_class: EnvironmentClass,
    pub endpoint_identity: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub command_type: String,
    pub target_ref: String,
    pub canonical_payload_hash: String,
    pub expected_current_state: Option<String>,
    pub reason: Option<String>,
    pub schema_version: SchemaVersion,
    pub fingerprint: String,
}

impl CanonicalIdempotencyFingerprint {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        environment_class: EnvironmentClass,
        endpoint_identity: impl Into<String>,
        tenant_id: impl Into<String>,
        actor_id: impl Into<String>,
        command_type: impl Into<String>,
        target_ref: impl Into<String>,
        canonical_payload_hash: impl Into<String>,
        expected_current_state: Option<String>,
        reason: Option<String>,
        schema_version: &str,
        fingerprint: impl Into<String>,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            environment_class,
            endpoint_identity: endpoint_identity.into(),
            tenant_id: tenant_id.into(),
            actor_id: actor_id.into(),
            command_type: command_type.into(),
            target_ref: target_ref.into(),
            canonical_payload_hash: canonical_payload_hash.into(),
            expected_current_state,
            reason,
            schema_version: ensure_supported_schema_version(schema_version)?,
            fingerprint: fingerprint.into(),
        })
    }

    pub fn idempotency_key(&self) -> String {
        format!("idem_{}", idempotency_component(&self.fingerprint))
    }

    pub fn new_operation_idempotency_key(&self, trace_id: &str) -> String {
        format!(
            "idem_new_{}_{}",
            idempotency_component(&self.fingerprint),
            idempotency_component(trace_id)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryTimeoutPolicy {
    pub max_retries: u8,
    pub timeout_ms: u64,
    pub bounded: bool,
    pub retryable_classes: Vec<RetryClass>,
    pub non_retryable_reason_families: Vec<String>,
}

impl RetryTimeoutPolicy {
    pub fn bounded(max_retries: u8, timeout_ms: u64) -> Self {
        Self {
            max_retries: max_retries.min(5),
            timeout_ms: timeout_ms.clamp(1, 600_000),
            bounded: true,
            retryable_classes: vec![RetryClass::SafeRetry, RetryClass::RetryAfter],
            non_retryable_reason_families: [
                "schema",
                "auth",
                "policy",
                "phase",
                "credential",
                "idempotency_duplicate",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorDecodeRecord {
    pub source_family: String,
    pub reason_code: String,
    pub retry_class: RetryClass,
    pub exit_class: ExitCodeClass,
    pub remediation_hint: String,
    pub raw_internal_error_exposed: bool,
}

impl ErrorDecodeRecord {
    pub fn new(
        source_family: impl Into<String>,
        reason_code: impl Into<String>,
        retry_class: RetryClass,
        exit_class: ExitCodeClass,
        remediation_hint: impl Into<String>,
    ) -> Self {
        Self {
            source_family: source_family.into(),
            reason_code: reason_code.into(),
            retry_class,
            exit_class,
            remediation_hint: remediation_hint.into(),
            raw_internal_error_exposed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalIdempotencyCacheRecord {
    pub cache_scope: String,
    pub profile_name: String,
    pub environment_class: EnvironmentClass,
    pub command_fingerprint: String,
    pub idempotency_key: String,
    pub owner_only: bool,
    pub contains_private_payload: bool,
    pub resettable: bool,
    pub inspectable: bool,
}

impl LocalIdempotencyCacheRecord {
    pub fn new(
        profile_name: impl Into<String>,
        environment_class: EnvironmentClass,
        command_fingerprint: impl Into<String>,
        idempotency_key: impl Into<String>,
    ) -> Self {
        Self {
            cache_scope: "profile_environment".to_owned(),
            profile_name: profile_name.into(),
            environment_class,
            command_fingerprint: command_fingerprint.into(),
            idempotency_key: idempotency_key.into(),
            owner_only: true,
            contains_private_payload: false,
            resettable: true,
            inspectable: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    pub reason_code: String,
    pub message: String,
    pub phase_gate: String,
    pub retry_class: RetryClass,
    pub trace_id: Option<String>,
    pub schema_version: SchemaVersion,
}

impl ApiError {
    pub fn new(
        reason_code: impl Into<String>,
        message: impl Into<String>,
        phase_gate: impl Into<String>,
        retry_class: RetryClass,
        trace_id: Option<String>,
        schema_version: &str,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            reason_code: reason_code.into(),
            message: message.into(),
            phase_gate: phase_gate.into(),
            retry_class,
            trace_id,
            schema_version: ensure_supported_schema_version(schema_version)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputEnvelope {
    pub ok: bool,
    pub schema_version: SchemaVersion,
    pub reason_code: Option<String>,
    pub retry_class: RetryClass,
    pub trace_id: Option<String>,
    pub audit_refs: Vec<String>,
    pub warnings: Vec<String>,
    pub exit_code: i32,
    pub exit_class: ExitCodeClass,
    pub lifecycle: CommandLifecycle,
    pub timing_ms: u64,
    pub diagnostic_bundle: Option<DiagnosticBundle>,
    pub capability_snapshot: Option<CapabilitySnapshot>,
}

impl OutputEnvelope {
    pub fn success(schema_version: &str) -> Result<Self, ContractError> {
        Ok(Self {
            ok: true,
            schema_version: ensure_supported_schema_version(schema_version)?,
            reason_code: None,
            retry_class: RetryClass::NotRetryable,
            trace_id: None,
            audit_refs: Vec::new(),
            warnings: Vec::new(),
            exit_code: ExitCodeClass::Success.code(),
            exit_class: ExitCodeClass::Success,
            lifecycle: CommandLifecycle::new(vec![
                CommandLifecycleState::Parsed,
                CommandLifecycleState::Completed,
            ]),
            timing_ms: 0,
            diagnostic_bundle: None,
            capability_snapshot: None,
        })
    }

    pub fn failure(
        schema_version: &str,
        reason_code: impl Into<String>,
    ) -> Result<Self, ContractError> {
        Self::failure_with_exit(
            schema_version,
            reason_code,
            ExitCodeClass::Config,
            RetryClass::NotRetryable,
        )
    }

    pub fn failure_with_exit(
        schema_version: &str,
        reason_code: impl Into<String>,
        exit_class: ExitCodeClass,
        retry_class: RetryClass,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            ok: false,
            schema_version: ensure_supported_schema_version(schema_version)?,
            reason_code: Some(reason_code.into()),
            retry_class,
            trace_id: None,
            audit_refs: Vec::new(),
            warnings: Vec::new(),
            exit_code: exit_class.code(),
            exit_class,
            lifecycle: CommandLifecycle::new(vec![
                CommandLifecycleState::Parsed,
                CommandLifecycleState::Failed,
            ]),
            timing_ms: 0,
            diagnostic_bundle: None,
            capability_snapshot: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticBundle {
    pub command_name: String,
    pub schema_version: SchemaVersion,
    pub profile_name: Option<String>,
    pub endpoint_fingerprint: Option<String>,
    pub schema_versions: Vec<String>,
    pub trace_ids: Vec<String>,
    pub reason_codes: Vec<String>,
    pub retry_count: u8,
    pub dependency_status: Vec<String>,
    pub redaction_policy: String,
}

impl DiagnosticBundle {
    pub fn new(
        command_name: impl Into<String>,
        schema_version: &str,
    ) -> Result<Self, ContractError> {
        let schema_version = ensure_supported_schema_version(schema_version)?;
        Ok(Self {
            command_name: command_name.into(),
            schema_versions: vec![schema_version.raw().to_owned()],
            schema_version,
            profile_name: None,
            endpoint_fingerprint: None,
            trace_ids: Vec::new(),
            reason_codes: Vec::new(),
            retry_count: 0,
            dependency_status: vec!["local_contracts_available".to_owned()],
            redaction_policy: "secret_free_refs_only".to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn accepts_supported_cli_schema_version() {
        let parsed = ensure_supported_schema_version(SUPPORTED_SCHEMA_VERSION).unwrap();
        assert_eq!(parsed.family(), CLI_SCHEMA_FAMILY);
        assert_eq!(parsed.major(), 0);
        assert_eq!(parsed.minor(), 1);
    }

    #[test]
    fn rejects_missing_schema_version() {
        assert_eq!(
            ensure_supported_schema_version("").unwrap_err(),
            ContractError::MissingSchemaVersion
        );
    }

    #[test]
    fn rejects_unknown_schema_family() {
        assert!(matches!(
            ensure_supported_schema_version("admin-ui.v0.1"),
            Err(ContractError::UnsupportedSchemaVersion { .. })
        ));
    }

    #[test]
    fn rejects_future_minor_schema_version() {
        assert!(matches!(
            ensure_supported_schema_version("cli-command.v0.2"),
            Err(ContractError::UnsupportedSchemaVersion { .. })
        ));
    }

    #[test]
    fn rejects_malformed_schema_version() {
        assert!(matches!(
            ensure_supported_schema_version("cli-command-0-1"),
            Err(ContractError::InvalidSchemaVersion(_))
        ));
    }

    #[test]
    fn exposes_contract_projection_metadata() {
        let set = cli_contract_set();
        assert_eq!(set.source_root, CONTRACT_SOURCE_ROOT);
        assert_eq!(set.projection_status, GENERATED_CONTRACT_STATUS);
    }

    #[test]
    fn validates_local_fixture_profile_and_credential() {
        let profile = local_profile();
        let credential = fixture_credential();
        profile.validate().unwrap();
        credential.validate_for_profile(&profile).unwrap();
    }

    #[test]
    fn rejects_missing_tenant_before_profile_use() {
        let mut profile = local_profile();
        profile.tenant_id.clear();
        assert!(matches!(
            profile.validate(),
            Err(ProfileValidationError::MissingRequiredField("tenant id"))
        ));
    }

    #[test]
    fn rejects_fixture_credential_for_seed_profile_without_test_harness() {
        let mut profile = local_profile();
        profile.environment = EnvironmentClass::Seed;
        profile.fixture_allowance = FixtureAllowance::Denied;
        assert!(matches!(
            profile.validate(),
            Err(ProfileValidationError::FixtureCredentialNotAllowed { .. })
        ));
    }

    #[test]
    fn rejects_raw_key_material_in_credential_reference() {
        let profile = local_profile();
        let mut credential = fixture_credential();
        credential.reference_id = "-----BEGIN PRIVATE KEY-----".to_owned();
        assert!(matches!(
            credential.validate_for_profile(&profile),
            Err(ProfileValidationError::RawKeyMaterialInReference)
        ));
    }

    #[test]
    fn rejects_revoked_and_expired_credential_references() {
        let profile = local_profile();
        let mut revoked = fixture_credential();
        revoked.revoked = true;
        assert!(matches!(
            revoked.validate_for_profile(&profile),
            Err(ProfileValidationError::CredentialRevoked)
        ));

        let mut expired = fixture_credential();
        expired.expired = true;
        assert!(matches!(
            expired.validate_for_profile(&profile),
            Err(ProfileValidationError::CredentialExpired)
        ));
    }

    #[test]
    fn signer_handoff_returns_refs_without_key_material() {
        let profile = local_profile();
        let credential = fixture_credential();
        let handoff = SignerHandoff::new(&profile, &credential, "tenant create").unwrap();
        assert_eq!(handoff.credential_reference_id, credential.reference_id);
        assert!(!handoff.exposes_key_material);
        assert!(handoff.signature_ref.starts_with("sigref:local-dev:key-1:"));
        assert!(!handoff.signature_ref.contains("PRIVATE KEY"));
    }

    #[test]
    fn signed_bootstrap_envelope_carries_phase5_refs() {
        let envelope = SignedCommandEnvelope::new(
            BootstrapCommandFamily::Tenant,
            "tenant create",
            "tenant_local",
            "actor_local",
            "tenant_local",
            "tenant_bootstrap_command",
            Some("absent".to_owned()),
            Some("bootstrap tenant".to_owned()),
            "idem_local_tenant_create_tenant_local",
            "trace_cli_local",
            "sigref:local-dev:key-1:tenant_create",
        )
        .unwrap();

        assert_eq!(envelope.family.as_str(), "tenant");
        assert_eq!(
            envelope.family.phase_gate(),
            "phase_1_control_plane_bootstrap"
        );
        assert_eq!(
            envelope.idempotency.key,
            "idem_local_tenant_create_tenant_local"
        );
        assert_eq!(envelope.trace_context.trace_id, "trace_cli_local");
        assert!(!envelope.exposes_key_material);
    }

    #[test]
    fn canonical_idempotency_fingerprint_carries_phase6_inputs() {
        let fingerprint = CanonicalIdempotencyFingerprint::new(
            EnvironmentClass::Local,
            "fp_local_dev",
            "tenant_local",
            "actor_local",
            "tenant create",
            "tenant_local",
            "hash_1234",
            Some("absent".to_owned()),
            Some("bootstrap tenant".to_owned()),
            SUPPORTED_SCHEMA_VERSION,
            "local:fp_local_dev:tenant:create",
        )
        .unwrap();

        assert_eq!(fingerprint.endpoint_identity, "fp_local_dev");
        assert_eq!(
            fingerprint.expected_current_state.as_deref(),
            Some("absent")
        );
        assert_eq!(fingerprint.reason.as_deref(), Some("bootstrap tenant"));
        assert_eq!(fingerprint.schema_version.raw(), SUPPORTED_SCHEMA_VERSION);
        assert!(fingerprint.idempotency_key().starts_with("idem_"));
        assert!(fingerprint
            .new_operation_idempotency_key("trace_cli_new")
            .starts_with("idem_new_"));
    }

    #[test]
    fn retry_timeout_policy_is_bounded_and_classified() {
        let policy = RetryTimeoutPolicy::bounded(9, 900_000);

        assert_eq!(policy.max_retries, 5);
        assert_eq!(policy.timeout_ms, 600_000);
        assert!(policy.bounded);
        assert!(policy.retryable_classes.contains(&RetryClass::SafeRetry));
        assert!(policy
            .non_retryable_reason_families
            .contains(&"phase".to_owned()));
    }

    #[test]
    fn error_decode_record_hides_internal_errors() {
        let record = ErrorDecodeRecord::new(
            "platform",
            "transport_unavailable",
            RetryClass::SafeRetry,
            ExitCodeClass::Transport,
            "retry with the same idempotency key",
        );

        assert_eq!(record.retry_class, RetryClass::SafeRetry);
        assert_eq!(record.exit_class, ExitCodeClass::Transport);
        assert!(!record.raw_internal_error_exposed);
    }

    #[test]
    fn local_idempotency_cache_record_is_refs_only() {
        let record = LocalIdempotencyCacheRecord::new(
            "local-dev",
            EnvironmentClass::Local,
            "fingerprint_local",
            "idem_fingerprint_local",
        );

        assert_eq!(record.cache_scope, "profile_environment");
        assert!(record.owner_only);
        assert!(!record.contains_private_payload);
        assert!(record.inspectable);
        assert!(record.resettable);
    }

    #[test]
    fn node_status_record_is_profile_scoped_and_refs_only() {
        let record = NodeStatusRecord::new(
            "node_live",
            NodeState::Live,
            "local-dev",
            "fixture://local-dev/key-1",
        );

        assert_eq!(record.state.as_str(), "live");
        assert!(record.state.accepts_work());
        assert!(record.credential_checked);
        assert_eq!(record.registered_via, "sdk_overgate_contract");
        assert!(!record.direct_node_access);
        assert!(record
            .capability_refs
            .contains(&"overcell:node:node_live:capabilities".to_owned()));
        assert_eq!(NodeState::Draining.as_str(), "draining");
        assert!(!NodeState::Disabled.accepts_work());
    }

    #[test]
    fn workload_execution_contracts_are_refs_only_and_terminal_aware() {
        let timeline = ExecutionTimeline::new(
            "workload_local",
            vec![
                WorkloadExecutionState::Scheduled,
                WorkloadExecutionState::Leased,
                WorkloadExecutionState::Running,
                WorkloadExecutionState::Succeeded,
            ],
            "trace_cli_local",
        );
        let logs = ExecutionLogBundle::new("workload_local", "trace_cli_local");
        let result = ExecutionResultRef::new("workload_local", "trace_cli_local");
        let polling = PollingPlan::bounded(true, true, 900_000, 10);

        assert!(WorkloadExecutionState::Succeeded.is_terminal());
        assert!(WorkloadExecutionState::TimedOut.is_terminal());
        assert!(!WorkloadExecutionState::Running.is_terminal());
        assert_eq!(timeline.states[0].as_str(), "scheduled");
        assert_eq!(timeline.states[3].as_str(), "succeeded");
        assert_eq!(timeline.diagnostic_events.len(), 6);
        assert!(timeline
            .owning_service_refs
            .contains(&"overwatch:trace".to_owned()));
        assert!(timeline
            .owning_service_refs
            .contains(&"overcell:node-heartbeat".to_owned()));
        assert!(timeline
            .owning_service_refs
            .contains(&"overpack:package".to_owned()));
        assert!(timeline
            .owning_service_refs
            .contains(&"overstore:result-state-ref".to_owned()));
        assert!(timeline
            .diagnostic_events
            .iter()
            .any(|event| event.reason_code == "result.ref.available"));
        let failed_timeline = ExecutionTimeline::new(
            "workload_failed",
            vec![
                WorkloadExecutionState::Scheduled,
                WorkloadExecutionState::Leased,
                WorkloadExecutionState::Running,
                WorkloadExecutionState::Failed,
            ],
            "trace_cli_local",
        );
        assert!(failed_timeline
            .diagnostic_events
            .iter()
            .any(|event| event.reason_code == "result.failed"));
        assert!(!timeline.direct_node_access);
        assert_eq!(logs.redaction_policy, "secret_free_refs_only");
        assert!(logs.bounded_streaming);
        assert!(!logs.contains_private_payload);
        assert!(!logs.direct_node_path_exposed);
        assert!(result.result_ref.starts_with("overstore:result:"));
        assert!(!result.contains_private_payload);
        assert!(!result.direct_object_store_path_exposed);
        assert_eq!(polling.timeout_ms, 600_000);
        assert_eq!(polling.poll_interval_ms, 100);
        assert!(polling.event_stream_preferred);
        assert!(polling.fallback_polling);
    }

    #[test]
    fn manifest_and_workload_phase5_refs_are_pending_only() {
        let manifest = ManifestBootstrapRef::new("manifest_local", "workload");
        assert_eq!(manifest.submitted_via, "sdk_overgate_contract");
        assert_eq!(manifest.immutable_ref, "manifest:manifest_local:immutable");

        let workload = SyntheticWorkloadPendingState::pending("workload_local", "synthetic");
        assert_eq!(workload.queue_state, "pending");
        assert!(!workload.execution_implied);
        assert_eq!(
            workload.timeline_refs,
            vec!["timeline:workload_local:pending".to_owned()]
        );
    }

    #[test]
    fn phase9_product_workflow_recipe_keeps_products_on_sdk_overgate_refs() {
        let docdex = ProductWorkflowRecipe::new(
            ProductKind::Docdex,
            "workload_docdex_index",
            "docdex_encrypted_index",
        );
        assert_eq!(docdex.product.as_str(), "docdex");
        assert_eq!(docdex.submitted_via, "sdk_overgate_contract");
        assert!(docdex.sdk_overgate_only);
        assert!(docdex.authorized_refs_only);
        assert!(docdex.secret_free_json_output);
        assert!(docdex.required_refs.contains(&"encrypted_index_ref".to_owned()));
        assert!(!docdex.direct_internal_api_access);
        assert!(!docdex.direct_storage_access);
        assert!(!docdex.raw_http_required);

        let mcoda = ProductWorkflowRecipe::new(
            ProductKind::Mcoda,
            "workload_mcoda_agent",
            "mcoda_agent_workload",
        );
        assert!(mcoda.dynamic_model_resource_selection);
        assert!(mcoda
            .required_refs
            .contains(&"dynamic_model_metadata_ref".to_owned()));
        assert!(!mcoda.hardcoded_model_or_provider);
        assert!(!mcoda.hardcoded_node_assumption);
        assert!(!mcoda.paid_service_assumption);

        let codali = ProductWorkflowRecipe::new(
            ProductKind::Codali,
            "workload_codali_package",
            "codali_code_agent_package",
        );
        assert!(codali.required_refs.contains(&"artifact_refs".to_owned()));
        assert!(codali
            .expected_failure_modes
            .contains(&"policy.resource_denied".to_owned()));
        assert!(codali
            .safe_retry_patterns
            .contains(&"retry_failed_phase_after_repair_ref".to_owned()));
    }

    #[test]
    fn phase9_ci_automation_profile_requires_explicit_short_lived_refs() {
        let profile = CiAutomationProfile::new(
            EnvironmentClass::Ci,
            "ci_reference",
            "ci://overrid/service-account/short-lived",
        );

        assert_eq!(profile.profile_kind, "ci");
        assert_eq!(profile.environment_class.as_str(), "ci");
        assert_eq!(profile.submitted_via, "sdk_overgate_contract");
        assert!(profile.short_lived_service_account_required);
        assert!(profile.requires_non_interactive_confirmation);
        assert!(profile.json_output_stable);
        assert!(profile.secret_free_output);
        assert!(profile.branch_on_exit_class);
        assert!(!profile.ambient_persistent_keychain_allowed);
        assert!(profile
            .allowed_credential_ref_kinds
            .contains(&"mounted_secret_ref".to_owned()));
    }

    #[test]
    fn bootstrap_acceptance_record_tracks_audit_refs() {
        let record = BootstrapAcceptanceRecord::new(
            "workload submit",
            "accepted:workload_local",
            "pending",
            vec!["audit_cli_bootstrap_workload_submit".to_owned()],
        );

        assert_eq!(record.phase_gate, "phase_1_control_plane_bootstrap");
        assert_eq!(record.pending_state, "pending");
        assert_eq!(record.audit_refs.len(), 1);
    }

    #[test]
    fn command_lifecycle_tracks_terminal_states() {
        let lifecycle = CommandLifecycle::new(vec![
            CommandLifecycleState::Parsed,
            CommandLifecycleState::ProfileLoaded,
            CommandLifecycleState::CredentialReady,
            CommandLifecycleState::PayloadValidated,
            CommandLifecycleState::Completed,
        ]);

        assert!(lifecycle.has_terminal_state());
        assert_eq!(
            lifecycle.terminal_state(),
            Some(CommandLifecycleState::Completed)
        );
        assert_eq!(CommandLifecycleState::Denied.as_str(), "denied");
        assert_eq!(ALL_COMMAND_LIFECYCLE_STATES.len(), 11);
    }

    #[test]
    fn exit_code_registry_locks_phase4_numeric_classes() {
        let pairs = EXIT_CODE_REGISTRY
            .iter()
            .map(|class| (class.as_str(), class.code()))
            .collect::<Vec<_>>();

        assert_eq!(
            pairs,
            vec![
                ("success", 0),
                ("usage", 2),
                ("config", 3),
                ("credential", 4),
                ("schema", 5),
                ("policy", 6),
                ("phase", 7),
                ("idempotency", 8),
                ("transport", 9),
                ("timeout", 10),
                ("platform", 11),
                ("local_io", 12),
            ]
        );
        assert_eq!(exit_code_class_for_code(7), Some(ExitCodeClass::Phase));
        assert_eq!(exit_code_class_for_code(1), None);
    }

    #[test]
    fn output_envelope_carries_phase4_fields() {
        let success = OutputEnvelope::success(SUPPORTED_SCHEMA_VERSION).unwrap();
        assert_eq!(success.exit_class, ExitCodeClass::Success);
        assert_eq!(success.retry_class, RetryClass::NotRetryable);
        assert_eq!(
            success.lifecycle.terminal_state(),
            Some(CommandLifecycleState::Completed)
        );

        let failure = OutputEnvelope::failure_with_exit(
            SUPPORTED_SCHEMA_VERSION,
            "not_available_in_phase",
            ExitCodeClass::Phase,
            RetryClass::NotRetryable,
        )
        .unwrap();
        assert_eq!(failure.exit_code, 7);
        assert_eq!(failure.exit_class.as_str(), "phase");
        assert_eq!(
            failure.reason_code.as_deref(),
            Some("not_available_in_phase")
        );
    }

    #[test]
    fn diagnostic_bundle_is_secret_free_and_refs_only() {
        let bundle = DiagnosticBundle::new("doctor", SUPPORTED_SCHEMA_VERSION).unwrap();
        assert_eq!(bundle.redaction_policy, "secret_free_refs_only");
        assert_eq!(
            bundle.schema_versions,
            vec![SUPPORTED_SCHEMA_VERSION.to_owned()]
        );
        assert!(bundle.trace_ids.is_empty());
        assert!(bundle.reason_codes.is_empty());
        assert!(bundle
            .dependency_status
            .contains(&"local_contracts_available".to_owned()));
    }

    #[test]
    fn capability_snapshot_fails_closed_when_route_unavailable() {
        let capability = CapabilitySnapshot::local_phase_gate("policy dry-run", "phase_4");
        assert!(!capability.available);
        assert!(capability.fail_closed);
        assert_eq!(capability.phase_gate, "phase_4");
        assert_eq!(
            capability.schema_versions,
            vec![SUPPORTED_SCHEMA_VERSION.to_owned()]
        );
    }

    #[test]
    fn owner_only_file_mode_rejects_group_or_world_bits() {
        validate_owner_only_file_mode(0o600).unwrap();
        validate_owner_only_file_mode(0o700).unwrap();
        assert!(matches!(
            validate_owner_only_file_mode(0o644),
            Err(ProfileValidationError::UnsafeFilePermissions(0o644))
        ));
    }
}
