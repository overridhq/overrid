#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};

use overrid_contracts::{
    ensure_supported_local_development_stack_schema_version, ExitCodeClass, RetryClass,
    SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
};

pub const DEFAULT_LOCAL_STACK_MANIFEST_PATH: &str = "packages/schemas/overrid_contracts/fixtures/valid/local_development_stack_phase2_default_local.valid.json";
pub const LOCAL_STACK_PHASE_GATE: &str = "phase_3_local_development_stack";
pub const LOCAL_STACK_PHASE4_TOPOLOGY_GATE: &str = "phase_4_loopback_topology";
pub const LOCAL_STACK_PHASE5_BACKING_GATE: &str = "phase_5_embedded_state_queue_store";
pub const LOCAL_STACK_ENV_EXAMPLE_PATH: &str = ".env.example";

const RESERVED_PORT_BINDINGS: [ReservedPortBinding; 6] = [
    ReservedPortBinding {
        service_id: "service:api",
        port: 18080,
        bind_host: "127.0.0.1",
        purpose: "api_health_ready",
        endpoint_ref: "http://127.0.0.1:18080/healthz",
    },
    ReservedPortBinding {
        service_id: "service:worker",
        port: 18081,
        bind_host: "127.0.0.1",
        purpose: "worker_health_metrics",
        endpoint_ref: "http://127.0.0.1:18081/healthz",
    },
    ReservedPortBinding {
        service_id: "service:node_agent_simulator",
        port: 18082,
        bind_host: "127.0.0.1",
        purpose: "node_agent_simulator_health",
        endpoint_ref: "http://127.0.0.1:18082/healthz",
    },
    ReservedPortBinding {
        service_id: "service:overstore_stub",
        port: 18083,
        bind_host: "127.0.0.1",
        purpose: "object_artifact_stub",
        endpoint_ref: "http://127.0.0.1:18083/healthz",
    },
    ReservedPortBinding {
        service_id: "service:event_audit",
        port: 18084,
        bind_host: "127.0.0.1",
        purpose: "local_event_audit_query",
        endpoint_ref: "http://127.0.0.1:18084/healthz",
    },
    ReservedPortBinding {
        service_id: "service:developer_ui",
        port: 18085,
        bind_host: "127.0.0.1",
        purpose: "optional_developer_ui",
        endpoint_ref: "http://127.0.0.1:18085/healthz",
    },
];

const LOCAL_ENV_VARIABLES: [LocalEnvVariable; 2] = [
    LocalEnvVariable {
        name: "OVERRID_LOCAL_PROFILE",
        value_ref: "env://OVERRID_LOCAL_PROFILE",
        redacted: false,
        example_only: false,
    },
    LocalEnvVariable {
        name: "OVERRID_LOCAL_TEST_SECRET_REF",
        value_ref: "secret://local_stack/fixture_key_ref",
        redacted: true,
        example_only: false,
    },
];

const LOCAL_SECRET_RECORDS: [LocalSecretRecord; 1] = [LocalSecretRecord {
    secret_ref: "secret://local_stack/fixture_key_ref",
    secret_kind: "fixture_key",
    redaction_ref: "redaction:fixture_key_ref",
    local_only: true,
    test_only: true,
    raw_secret_present: false,
}];

const READY_DOCTOR_CHECKS: [LocalDoctorCheck; 7] = [
    LocalDoctorCheck {
        check_id: "doctor:rust_toolchain",
        state: "ready",
        reason_code: "doctor.rust_toolchain_ready",
        remediation_hint: "install the repository-pinned Rust toolchain",
    },
    LocalDoctorCheck {
        check_id: "doctor:repo_layout",
        state: "ready",
        reason_code: "doctor.repo_layout_ready",
        remediation_hint: "run from a clean Overrid workspace checkout",
    },
    LocalDoctorCheck {
        check_id: "doctor:schema_outputs",
        state: "ready",
        reason_code: "doctor.schemas_ready",
        remediation_hint: "regenerate shared schema outputs",
    },
    LocalDoctorCheck {
        check_id: "doctor:reserved_ports",
        state: "ready",
        reason_code: "doctor.ports_available",
        remediation_hint: "free the reserved loopback port range 18080-18085",
    },
    LocalDoctorCheck {
        check_id: "doctor:file_permissions",
        state: "ready",
        reason_code: "doctor.file_permissions_ready",
        remediation_hint: "fix local stack volume and env file permissions",
    },
    LocalDoctorCheck {
        check_id: "doctor:local_volume_markers",
        state: "ready",
        reason_code: "doctor.volume_markers_ready",
        remediation_hint: "restore .overrid-local-test-state markers before reset",
    },
    LocalDoctorCheck {
        check_id: "doctor:ci_runner",
        state: "ready",
        reason_code: "doctor.ci_runner_compatible",
        remediation_hint: "use the reproducible Linux x86_64 local/test runner",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevCommand {
    Start,
    Stop,
    Restart,
    Status,
    Reset,
    Seed,
    Smoke,
    Logs,
    Doctor,
}

impl DevCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "dev start",
            Self::Stop => "dev stop",
            Self::Restart => "dev restart",
            Self::Status => "dev status",
            Self::Reset => "dev reset",
            Self::Seed => "dev seed",
            Self::Smoke => "dev smoke",
            Self::Logs => "dev logs",
            Self::Doctor => "dev doctor",
        }
    }

    fn action(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Stop => "stop",
            Self::Restart => "restart",
            Self::Status => "status",
            Self::Reset => "reset",
            Self::Seed => "seed",
            Self::Smoke => "smoke",
            Self::Logs => "logs",
            Self::Doctor => "doctor",
        }
    }

    fn gates_unavailable_future_services(self) -> bool {
        matches!(self, Self::Start | Self::Restart | Self::Smoke)
    }

    fn checks_reserved_ports(self) -> bool {
        matches!(
            self,
            Self::Start | Self::Restart | Self::Smoke | Self::Doctor
        )
    }

    fn checks_schema_compatibility(self) -> bool {
        matches!(self, Self::Start | Self::Seed | Self::Smoke)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackOptions {
    pub repo_root: PathBuf,
    pub profile: String,
    pub master_phase: u8,
    pub trace_id: String,
    pub timeout_ms: Option<u64>,
    pub poll_interval_ms: Option<u64>,
    pub wait: bool,
    pub follow: bool,
    pub dry_run: bool,
    pub port_preflight: bool,
}

impl LocalStackOptions {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
            profile: "local".to_owned(),
            master_phase: 0,
            trace_id: "trace_local_stack".to_owned(),
            timeout_ms: None,
            poll_interval_ms: None,
            wait: false,
            follow: false,
            dry_run: false,
            port_preflight: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackRunner {
    options: LocalStackOptions,
}

impl LocalStackRunner {
    pub fn new(options: LocalStackOptions) -> Self {
        Self { options }
    }

    pub fn run(&self, command: DevCommand) -> LocalStackCommandOutput {
        let mut output = LocalStackCommandOutput::new(command, &self.options);
        let manifest = match LocalStackManifest::load_default(&self.options.repo_root) {
            Ok(manifest) => manifest,
            Err(error) => {
                output.fail_from_manifest(error);
                return output;
            }
        };
        output.manifest_path = manifest.path_ref();

        if let Some(profile_error) = profile_blocker(&self.options.profile) {
            output.block(profile_error);
            return output;
        }

        output
            .push_state(LocalCommandState::PrerequisitesChecked)
            .expect("local-stack prerequisite transition is valid");

        if let Some(profile_failure) = profile_backing_failure(&self.options.profile) {
            output.block(profile_failure);
            return output;
        }

        if command.checks_schema_compatibility() {
            let gates = schema_compatibility_gates_for_profile(&self.options.profile);
            if gates.iter().any(|gate| !gate.compatible) {
                output.apply_schema_compatibility_gates(gates);
                output.block(LocalStackFailure {
                    reason_code: "local_stack.schema_version_incompatible",
                    message: "local stack backing records use an incompatible schema version",
                    exit_class: ExitCodeClass::Schema,
                    retry_class: RetryClass::OperatorReview,
                    status: LocalStackStatus::Blocked,
                });
                return output;
            }
        }

        if self.options.port_preflight && command.checks_reserved_ports() {
            let conflicts = detect_reserved_port_conflicts();
            if !conflicts.is_empty() {
                output.apply_port_conflicts(conflicts);
                output.block(LocalStackFailure {
                    reason_code: "local_stack.port_conflict",
                    message: "one or more reserved local development ports are unavailable",
                    exit_class: ExitCodeClass::Config,
                    retry_class: RetryClass::OperatorReview,
                    status: LocalStackStatus::Blocked,
                });
                return output;
            }
        }

        if self.options.master_phase > 0 && command.gates_unavailable_future_services() {
            output.block(LocalStackFailure {
                reason_code: "phase.local_service_unavailable",
                message: "selected master phase requires local services that are not implemented in the phase 3 stack",
                exit_class: ExitCodeClass::Phase,
                retry_class: RetryClass::OperatorReview,
                status: LocalStackStatus::Blocked,
            });
            return output;
        }

        match command {
            DevCommand::Start => self.start(output),
            DevCommand::Stop => self.stop(output),
            DevCommand::Restart => self.restart(output),
            DevCommand::Status => self.status(output),
            DevCommand::Reset => self.reset(output),
            DevCommand::Seed => self.seed(output),
            DevCommand::Smoke => self.smoke(output),
            DevCommand::Logs => self.logs(output),
            DevCommand::Doctor => self.doctor(output, &manifest),
        }
    }

    fn start(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        output
            .push_state(LocalCommandState::Starting)
            .expect("local-stack start transition is valid");
        output
            .push_state(LocalCommandState::Ready)
            .expect("local-stack ready transition is valid");
        output.complete(
            LocalStackStatus::Ready,
            "local_stack.ready",
            "local stack services are ready",
        )
    }

    fn stop(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        output
            .push_state(LocalCommandState::Stopped)
            .expect("local-stack stop transition is valid");
        output.complete(
            LocalStackStatus::Stopped,
            "local_stack.stopped",
            "local stack services are stopped",
        )
    }

    fn restart(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        output
            .push_state(LocalCommandState::Stopped)
            .expect("local-stack restart stop transition is valid");
        output
            .push_state(LocalCommandState::Starting)
            .expect("local-stack restart start transition is valid");
        output
            .push_state(LocalCommandState::Ready)
            .expect("local-stack restart ready transition is valid");
        output.complete(
            LocalStackStatus::Ready,
            "local_stack.ready",
            "local stack services restarted and are ready",
        )
    }

    fn status(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        output
            .push_state(LocalCommandState::Ready)
            .expect("local-stack status transition is valid");
        output.complete(
            LocalStackStatus::Ready,
            "local_stack.ready",
            "local stack status is ready",
        )
    }

    fn reset(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        output
            .push_state(LocalCommandState::Resetting)
            .expect("local-stack reset transition is valid");
        output
            .push_state(LocalCommandState::CollectingArtifacts)
            .expect("local-stack reset artifact transition is valid");
        output.artifact_refs.push(format!(
            "artifact://local_stack/reset/{}",
            id_component(&self.options.trace_id)
        ));
        output.complete(
            LocalStackStatus::Completed,
            "local_stack.reset_completed",
            "local stack reset completed",
        )
    }

    fn seed(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        output
            .push_state(LocalCommandState::Seeding)
            .expect("local-stack seed transition is valid");
        output.complete(
            LocalStackStatus::Completed,
            "local_stack.seed_completed",
            "local stack seed completed",
        )
    }

    fn smoke(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        for state in [
            LocalCommandState::Starting,
            LocalCommandState::Ready,
            LocalCommandState::Resetting,
            LocalCommandState::Seeding,
            LocalCommandState::Smoking,
            LocalCommandState::CollectingArtifacts,
        ] {
            output
                .push_state(state)
                .expect("local-stack smoke transition is valid");
        }
        output.artifact_refs.push(format!(
            "artifact://local_stack/smoke/{}",
            id_component(&self.options.trace_id)
        ));
        output.complete(
            LocalStackStatus::Completed,
            "local_stack.smoke_passed",
            "local stack smoke checks passed",
        )
    }

    fn logs(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        output
            .push_state(LocalCommandState::CollectingArtifacts)
            .expect("local-stack logs transition is valid");
        output.artifact_refs.push(format!(
            "log://local_stack/{}",
            id_component(&self.options.trace_id)
        ));
        output.complete(
            LocalStackStatus::Completed,
            "local_stack.logs_collected",
            "local stack log references collected",
        )
    }

    fn doctor(
        &self,
        mut output: LocalStackCommandOutput,
        manifest: &LocalStackManifest,
    ) -> LocalStackCommandOutput {
        output
            .push_state(LocalCommandState::CollectingArtifacts)
            .expect("local-stack doctor artifact transition is valid");
        output.service_health = service_health_for_manifest(manifest);
        output.complete(
            LocalStackStatus::Completed,
            "local_stack.doctor_ok",
            "local stack doctor checks passed",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackManifest {
    pub path: PathBuf,
    pub schema_version: String,
    pub service_ids: Vec<String>,
    pub dependency_ids: Vec<String>,
}

impl LocalStackManifest {
    pub fn load_default(repo_root: impl AsRef<Path>) -> Result<Self, ManifestValidationError> {
        Self::load_from_path(repo_root.as_ref().join(DEFAULT_LOCAL_STACK_MANIFEST_PATH))
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ManifestValidationError> {
        let path = path.as_ref();
        let text =
            fs::read_to_string(path).map_err(|error| ManifestValidationError::ReadFailed {
                path: path.display().to_string(),
                message: error.to_string(),
            })?;
        let content = Self::validate_text(&text)?;
        Ok(Self {
            path: path.to_path_buf(),
            schema_version: content.schema_version,
            service_ids: content.service_ids,
            dependency_ids: content.dependency_ids,
        })
    }

    pub fn validate_text(text: &str) -> Result<LocalStackManifestContent, ManifestValidationError> {
        if !looks_like_json_object(text) {
            return Err(ManifestValidationError::InvalidJson);
        }

        let schema_version = extract_values_for_key(text, "schema_version")
            .into_iter()
            .next()
            .ok_or(ManifestValidationError::MissingSchemaVersion)?;
        ensure_supported_local_development_stack_schema_version(&schema_version).map_err(|_| {
            ManifestValidationError::IncompatibleSchemaVersion {
                provided: schema_version.clone(),
                supported: SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
            }
        })?;

        if !text.contains("\"local_only\": true") || !text.contains("\"test_only\": true") {
            return Err(ManifestValidationError::MissingLocalMarkers);
        }

        for key in ["endpoint", "default_bind_host", "bind_host"] {
            for endpoint in extract_values_for_key(text, key) {
                if !is_safe_endpoint(&endpoint) {
                    return Err(ManifestValidationError::UnsafeEndpoint(endpoint));
                }
            }
        }

        let enabled_service_ids = extract_string_arrays_for_key(text, "enabled_services");
        let service_definitions = extract_array_section(text, "service_definitions")
            .ok_or(ManifestValidationError::MissingServiceDefinitions)?;
        let service_ids = extract_values_for_key(service_definitions, "service_id");
        if service_ids.is_empty() {
            return Err(ManifestValidationError::MissingServiceDefinitions);
        }

        let mut defined_service_ids = BTreeSet::new();
        for service_id in &service_ids {
            if !defined_service_ids.insert(service_id.clone()) {
                return Err(ManifestValidationError::DuplicateServiceId(
                    service_id.clone(),
                ));
            }
        }

        let mut declared_dependency_ids = defined_service_ids.clone();
        declared_dependency_ids.extend(enabled_service_ids);
        let dependency_ids = extract_string_arrays_for_key(service_definitions, "depends_on");
        for dependency_id in &dependency_ids {
            if !declared_dependency_ids.contains(dependency_id) {
                return Err(ManifestValidationError::MissingDependency(
                    dependency_id.clone(),
                ));
            }
        }

        Ok(LocalStackManifestContent {
            schema_version,
            service_ids,
            dependency_ids,
        })
    }

    pub fn path_ref(&self) -> String {
        self.path.display().to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackManifestContent {
    pub schema_version: String,
    pub service_ids: Vec<String>,
    pub dependency_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestValidationError {
    ReadFailed {
        path: String,
        message: String,
    },
    InvalidJson,
    MissingSchemaVersion,
    IncompatibleSchemaVersion {
        provided: String,
        supported: &'static str,
    },
    MissingLocalMarkers,
    MissingServiceDefinitions,
    DuplicateServiceId(String),
    MissingDependency(String),
    UnsafeEndpoint(String),
}

impl ManifestValidationError {
    pub fn reason_code(&self) -> &'static str {
        match self {
            Self::ReadFailed { .. } => "manifest.read_failed",
            Self::InvalidJson => "manifest.invalid_json",
            Self::MissingSchemaVersion => "manifest.schema_version_missing",
            Self::IncompatibleSchemaVersion { .. } => "manifest.schema_version_incompatible",
            Self::MissingLocalMarkers => "manifest.local_test_markers_missing",
            Self::MissingServiceDefinitions => "manifest.service_definitions_missing",
            Self::DuplicateServiceId(_) => "manifest.duplicate_service_id",
            Self::MissingDependency(_) => "manifest.missing_dependency",
            Self::UnsafeEndpoint(_) => "manifest.unsafe_endpoint",
        }
    }

    pub fn exit_class(&self) -> ExitCodeClass {
        match self {
            Self::ReadFailed { .. } => ExitCodeClass::LocalIo,
            Self::InvalidJson
            | Self::MissingSchemaVersion
            | Self::IncompatibleSchemaVersion { .. }
            | Self::DuplicateServiceId(_)
            | Self::MissingDependency(_) => ExitCodeClass::Schema,
            Self::MissingLocalMarkers
            | Self::MissingServiceDefinitions
            | Self::UnsafeEndpoint(_) => ExitCodeClass::Config,
        }
    }
}

impl fmt::Display for ManifestValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadFailed { path, message } => {
                write!(
                    formatter,
                    "failed to read local-stack manifest {path}: {message}"
                )
            }
            Self::InvalidJson => formatter.write_str("local-stack manifest is not a JSON object"),
            Self::MissingSchemaVersion => {
                formatter.write_str("local-stack manifest schema_version is required")
            }
            Self::IncompatibleSchemaVersion {
                provided,
                supported,
            } => write!(
                formatter,
                "local-stack manifest schema version {provided} is incompatible with {supported}"
            ),
            Self::MissingLocalMarkers => {
                formatter.write_str("local-stack manifest must be local_only and test_only")
            }
            Self::MissingServiceDefinitions => {
                formatter.write_str("local-stack manifest must define service_definitions")
            }
            Self::DuplicateServiceId(service_id) => {
                write!(formatter, "duplicate local-stack service id {service_id}")
            }
            Self::MissingDependency(service_id) => write!(
                formatter,
                "local-stack dependency {service_id} is not defined"
            ),
            Self::UnsafeEndpoint(endpoint) => {
                write!(formatter, "unsafe local-stack endpoint {endpoint}")
            }
        }
    }
}

impl std::error::Error for ManifestValidationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalCommandState {
    Planned,
    PrerequisitesChecked,
    Starting,
    Ready,
    Resetting,
    Seeding,
    Smoking,
    CollectingArtifacts,
    Stopped,
    Blocked,
    Failed,
    Completed,
}

impl LocalCommandState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::PrerequisitesChecked => "prerequisites_checked",
            Self::Starting => "starting",
            Self::Ready => "ready",
            Self::Resetting => "resetting",
            Self::Seeding => "seeding",
            Self::Smoking => "smoking",
            Self::CollectingArtifacts => "collecting_artifacts",
            Self::Stopped => "stopped",
            Self::Blocked => "blocked",
            Self::Failed => "failed",
            Self::Completed => "completed",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Blocked | Self::Failed | Self::Completed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCommandRecord {
    states: Vec<LocalCommandState>,
}

impl LocalCommandRecord {
    pub fn new() -> Self {
        Self {
            states: vec![LocalCommandState::Planned],
        }
    }

    pub fn states(&self) -> &[LocalCommandState] {
        &self.states
    }

    pub fn push(&mut self, next: LocalCommandState) -> Result<(), StateTransitionError> {
        let previous = self
            .states
            .last()
            .copied()
            .unwrap_or(LocalCommandState::Planned);
        if !transition_allowed(previous, next) {
            return Err(StateTransitionError { previous, next });
        }
        self.states.push(next);
        Ok(())
    }

    pub fn lifecycle_strs(&self) -> Vec<&'static str> {
        self.states.iter().map(|state| state.as_str()).collect()
    }
}

impl Default for LocalCommandRecord {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateTransitionError {
    pub previous: LocalCommandState,
    pub next: LocalCommandState,
}

impl fmt::Display for StateTransitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid local-stack state transition from {} to {}",
            self.previous.as_str(),
            self.next.as_str()
        )
    }
}

impl std::error::Error for StateTransitionError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalStackStatus {
    Ready,
    Stopped,
    Blocked,
    Failed,
    Completed,
}

impl LocalStackStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Stopped => "stopped",
            Self::Blocked => "blocked",
            Self::Failed => "failed",
            Self::Completed => "completed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalServiceCapability {
    pub service_id: String,
    pub phase_gate: String,
    pub available: bool,
    pub reason_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalServiceHealth {
    pub service_id: String,
    pub state: String,
    pub endpoint_ref: String,
    pub bind_host: String,
    pub port: Option<u16>,
    pub loopback_only: bool,
    pub reason_code: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReservedPortBinding {
    pub service_id: &'static str,
    pub port: u16,
    pub bind_host: &'static str,
    pub purpose: &'static str,
    pub endpoint_ref: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReservedPortConflict {
    pub service_id: &'static str,
    pub port: u16,
    pub bind_host: &'static str,
    pub purpose: &'static str,
    pub endpoint_ref: &'static str,
    pub reason_code: &'static str,
    pub error_kind: &'static str,
    pub os_error_code: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalEnvVariable {
    pub name: &'static str,
    pub value_ref: &'static str,
    pub redacted: bool,
    pub example_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalSecretRecord {
    pub secret_ref: &'static str,
    pub secret_kind: &'static str,
    pub redaction_ref: &'static str,
    pub local_only: bool,
    pub test_only: bool,
    pub raw_secret_present: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalDoctorCheck {
    pub check_id: &'static str,
    pub state: &'static str,
    pub reason_code: &'static str,
    pub remediation_hint: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalStateRecord {
    pub record_id: &'static str,
    pub record_kind: &'static str,
    pub contract_ref: &'static str,
    pub schema_version: &'static str,
    pub storage_boundary: &'static str,
    pub external_database_semantics: bool,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalQueueJobRecord {
    pub job_id: &'static str,
    pub state: &'static str,
    pub idempotency_key: &'static str,
    pub idempotency_status: &'static str,
    pub trace_id: &'static str,
    pub priority: u8,
    pub retry_count: u8,
    pub max_retries: u8,
    pub timeout_ms: u64,
    pub dead_letter_reason_code: Option<&'static str>,
    pub terminal: bool,
    pub schema_version: &'static str,
    pub reason_code: &'static str,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalArtifactManifest {
    pub artifact_ref: &'static str,
    pub object_manifest_ref: &'static str,
    pub hash_algorithm: &'static str,
    pub content_hash: String,
    pub content_address: String,
    pub byte_length: usize,
    pub upload_grant_ref: &'static str,
    pub download_grant_ref: &'static str,
    pub reset_marker_ref: &'static str,
    pub reset_safe: bool,
    pub filesystem_backed: bool,
    pub external_object_store_boundary: bool,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalAuditQueryRecord {
    pub query_id: &'static str,
    pub event_ref: &'static str,
    pub event_type: &'static str,
    pub trace_id: &'static str,
    pub service_id: &'static str,
    pub time_window_ref: &'static str,
    pub local_diagnostic_event: bool,
    pub production_overwatch_authority: bool,
    pub redaction_summary: &'static str,
    pub contains_raw_secret: bool,
    pub reason_code: &'static str,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaCompatibilityGate {
    pub gate_id: String,
    pub surface: &'static str,
    pub schema_version: String,
    pub compatible: bool,
    pub reason_code: String,
    pub blocks_start: bool,
    pub blocks_seed: bool,
    pub blocks_smoke: bool,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackCommandOutput {
    pub command_name: String,
    pub profile: String,
    pub master_phase: u8,
    pub trace_id: String,
    pub ok: bool,
    pub status: LocalStackStatus,
    pub reason_code: String,
    pub message: String,
    pub exit_class: ExitCodeClass,
    pub retry_class: RetryClass,
    pub lifecycle: LocalCommandRecord,
    pub capabilities: Vec<LocalServiceCapability>,
    pub service_health: Vec<LocalServiceHealth>,
    pub port_bindings: Vec<ReservedPortBinding>,
    pub port_conflicts: Vec<ReservedPortConflict>,
    pub env_variables: Vec<LocalEnvVariable>,
    pub secret_records: Vec<LocalSecretRecord>,
    pub doctor_checks: Vec<LocalDoctorCheck>,
    pub local_state_records: Vec<LocalStateRecord>,
    pub queue_job_records: Vec<LocalQueueJobRecord>,
    pub artifact_manifests: Vec<LocalArtifactManifest>,
    pub audit_query_records: Vec<LocalAuditQueryRecord>,
    pub schema_compatibility_gates: Vec<SchemaCompatibilityGate>,
    pub diagnostic_refs: Vec<String>,
    pub artifact_refs: Vec<String>,
    pub manifest_path: String,
    pub timeout_ms: Option<u64>,
    pub poll_interval_ms: Option<u64>,
    pub wait: bool,
    pub follow: bool,
    pub dry_run: bool,
}

impl LocalStackCommandOutput {
    fn new(command: DevCommand, options: &LocalStackOptions) -> Self {
        let service_health = foundation_service_health();
        Self {
            command_name: command.as_str().to_owned(),
            profile: options.profile.clone(),
            master_phase: options.master_phase,
            trace_id: options.trace_id.clone(),
            ok: false,
            status: LocalStackStatus::Failed,
            reason_code: "local_stack.not_started".to_owned(),
            message: "local stack command has not completed".to_owned(),
            exit_class: ExitCodeClass::Platform,
            retry_class: RetryClass::NotRetryable,
            lifecycle: LocalCommandRecord::new(),
            capabilities: capabilities_for_phase(options.master_phase),
            service_health,
            port_bindings: reserved_port_bindings(),
            port_conflicts: Vec::new(),
            env_variables: local_env_variables(),
            secret_records: local_secret_records(),
            doctor_checks: doctor_checks_for_profile(&options.profile),
            local_state_records: local_state_records(),
            queue_job_records: local_queue_job_records(),
            artifact_manifests: local_artifact_manifests(),
            audit_query_records: local_audit_query_records(),
            schema_compatibility_gates: schema_compatibility_gates_for_profile(&options.profile),
            diagnostic_refs: vec![format!(
                "diagnostic://local_stack/{}/{}",
                command.action(),
                id_component(&options.trace_id)
            )],
            artifact_refs: Vec::new(),
            manifest_path: DEFAULT_LOCAL_STACK_MANIFEST_PATH.to_owned(),
            timeout_ms: options.timeout_ms,
            poll_interval_ms: options.poll_interval_ms,
            wait: options.wait,
            follow: options.follow,
            dry_run: options.dry_run,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.ok
    }

    pub fn lifecycle_strs(&self) -> Vec<&'static str> {
        self.lifecycle.lifecycle_strs()
    }

    pub fn dependency_status_strs(&self) -> Vec<&'static str> {
        if self.ok {
            vec![
                "local_stack_manifest_valid",
                "local_stack_profile_local_test",
                "local_stack_loopback_only",
                "local_stack_reserved_ports_checked",
                "local_stack_env_manifest_redacted",
                "local_stack_test_secrets_ref_only",
                "local_stack_doctor_checks_ready",
                "overbase_local_state_contract_ready",
                "overqueue_local_jobs_contract_ready",
                "overstore_artifact_stub_ready",
                "local_diagnostic_events_not_authoritative_overwatch",
                "schema_compatibility_gates_passed",
                "redacted_diagnostics_only",
            ]
        } else {
            vec![
                "local_stack_manifest_checked",
                "schema_compatibility_checked",
                "local_stack_fail_closed",
                "local_stack_preflight_failed",
                "redacted_diagnostics_only",
            ]
        }
    }

    pub fn human_summary(&self) -> String {
        let mut lines = vec![
            format!("command: {}", self.command_name),
            format!("status: {}", self.status.as_str()),
            format!("reason_code: {}", self.reason_code),
            format!("profile: {}", self.profile),
            format!("master_phase: {}", self.master_phase),
            format!("trace_id: {}", self.trace_id),
        ];
        if !self.capabilities.is_empty() {
            lines.push("capabilities:".to_owned());
            for capability in &self.capabilities {
                lines.push(format!(
                    "  {} available={} reason_code={}",
                    capability.service_id, capability.available, capability.reason_code
                ));
            }
        }
        lines.join("\n")
    }

    pub fn result_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"command\":\"{}\",",
                "\"profile\":\"{}\",",
                "\"master_phase\":{},",
                "\"status\":\"{}\",",
                "\"reason_code\":\"{}\",",
                "\"schema_version\":\"{}\",",
                "\"manifest_path\":\"{}\",",
                "\"topology_phase_gate\":\"{}\",",
                "\"backing_phase_gate\":\"{}\",",
                "\"local_only\":true,",
                "\"test_only\":true,",
                "\"node_or_ts_runtime_authority\":false,",
                "\"port_registry\":{},",
                "\"port_conflicts\":{},",
                "\"env_manifest\":{},",
                "\"secret_records\":{},",
                "\"doctor_checks\":{},",
                "\"local_state_records\":{},",
                "\"queue_job_records\":{},",
                "\"artifact_manifests\":{},",
                "\"audit_query_records\":{},",
                "\"schema_compatibility_gates\":{},",
                "\"capabilities\":{},",
                "\"service_health\":{},",
                "\"diagnostic_refs\":{},",
                "\"artifact_refs\":{},",
                "\"timeout_ms\":{},",
                "\"poll_interval_ms\":{},",
                "\"wait\":{},",
                "\"follow\":{},",
                "\"dry_run\":{}",
                "}}"
            ),
            json_escape(&self.command_name),
            json_escape(&self.profile),
            self.master_phase,
            json_escape(self.status.as_str()),
            json_escape(&self.reason_code),
            json_escape(SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION),
            json_escape(&self.manifest_path),
            json_escape(LOCAL_STACK_PHASE4_TOPOLOGY_GATE),
            json_escape(LOCAL_STACK_PHASE5_BACKING_GATE),
            render_port_registry_json(&self.port_bindings),
            render_port_conflicts_json(&self.port_conflicts),
            render_env_manifest_json(&self.env_variables),
            render_secret_records_json(&self.secret_records),
            render_doctor_checks_json(&self.doctor_checks),
            render_local_state_records_json(&self.local_state_records),
            render_queue_job_records_json(&self.queue_job_records),
            render_artifact_manifests_json(&self.artifact_manifests),
            render_audit_query_records_json(&self.audit_query_records),
            render_schema_compatibility_gates_json(&self.schema_compatibility_gates),
            render_capabilities_json(&self.capabilities),
            render_service_health_json(&self.service_health),
            json_owned_string_array(&self.diagnostic_refs),
            json_owned_string_array(&self.artifact_refs),
            json_optional_u64(self.timeout_ms),
            json_optional_u64(self.poll_interval_ms),
            self.wait,
            self.follow,
            self.dry_run,
        )
    }

    pub fn error_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"reason_code\":\"{}\",",
                "\"message\":\"{}\",",
                "\"phase_gate\":\"{}\",",
                "\"retry_class\":\"{}\",",
                "\"remediation_hint\":\"{}\",",
                "\"topology_phase_gate\":\"{}\",",
                "\"backing_phase_gate\":\"{}\",",
                "\"port_registry\":{},",
                "\"port_conflicts\":{},",
                "\"schema_compatibility_gates\":{},",
                "\"doctor_checks\":{},",
                "\"diagnostic_refs\":{}",
                "}}"
            ),
            json_escape(&self.reason_code),
            json_escape(&self.message),
            json_escape(LOCAL_STACK_PHASE_GATE),
            json_escape(self.retry_class.as_str()),
            json_escape(remediation_hint(&self.reason_code)),
            json_escape(LOCAL_STACK_PHASE4_TOPOLOGY_GATE),
            json_escape(LOCAL_STACK_PHASE5_BACKING_GATE),
            render_port_registry_json(&self.port_bindings),
            render_port_conflicts_json(&self.port_conflicts),
            render_schema_compatibility_gates_json(&self.schema_compatibility_gates),
            render_doctor_checks_json(&self.doctor_checks),
            json_owned_string_array(&self.diagnostic_refs),
        )
    }

    fn complete(
        mut self,
        status: LocalStackStatus,
        reason_code: &'static str,
        message: &'static str,
    ) -> Self {
        self.push_state(LocalCommandState::Completed)
            .expect("local-stack completed transition is valid");
        self.ok = true;
        self.status = status;
        self.reason_code = reason_code.to_owned();
        self.message = message.to_owned();
        self.exit_class = ExitCodeClass::Success;
        self.retry_class = RetryClass::NotRetryable;
        self
    }

    fn apply_port_conflicts(&mut self, conflicts: Vec<ReservedPortConflict>) {
        for conflict in &conflicts {
            if let Some(health) = self
                .service_health
                .iter_mut()
                .find(|health| health.service_id == conflict.service_id)
            {
                health.state = "failed".to_owned();
                health.reason_code = conflict.reason_code.to_owned();
                health.endpoint_ref = conflict.endpoint_ref.to_owned();
                health.bind_host = conflict.bind_host.to_owned();
                health.port = Some(conflict.port);
                health.loopback_only = is_loopback_host(conflict.bind_host);
            } else {
                self.service_health.push(LocalServiceHealth {
                    service_id: conflict.service_id.to_owned(),
                    state: "failed".to_owned(),
                    endpoint_ref: conflict.endpoint_ref.to_owned(),
                    bind_host: conflict.bind_host.to_owned(),
                    port: Some(conflict.port),
                    loopback_only: is_loopback_host(conflict.bind_host),
                    reason_code: conflict.reason_code.to_owned(),
                });
            }
        }
        self.port_conflicts = conflicts;
    }

    fn apply_schema_compatibility_gates(&mut self, gates: Vec<SchemaCompatibilityGate>) {
        self.schema_compatibility_gates = gates;
        self.diagnostic_refs.push(format!(
            "diagnostic://local_stack/schema_compatibility/{}",
            id_component(&self.trace_id)
        ));
    }

    fn block(&mut self, failure: LocalStackFailure) {
        self.push_state(LocalCommandState::Blocked)
            .expect("local-stack blocked transition is valid");
        self.doctor_checks = doctor_checks_for_reason(failure.reason_code);
        self.ok = false;
        self.status = failure.status;
        self.reason_code = failure.reason_code.to_owned();
        self.message = failure.message.to_owned();
        self.exit_class = failure.exit_class;
        self.retry_class = failure.retry_class;
    }

    fn fail_from_manifest(&mut self, error: ManifestValidationError) {
        self.push_state(LocalCommandState::Failed)
            .expect("local-stack failed transition is valid");
        self.doctor_checks = doctor_checks_for_reason(error.reason_code());
        self.ok = false;
        self.status = LocalStackStatus::Failed;
        self.reason_code = error.reason_code().to_owned();
        self.message = error.to_string();
        self.exit_class = error.exit_class();
        self.retry_class = RetryClass::OperatorReview;
    }

    fn push_state(&mut self, state: LocalCommandState) -> Result<(), StateTransitionError> {
        self.lifecycle.push(state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LocalStackFailure {
    reason_code: &'static str,
    message: &'static str,
    exit_class: ExitCodeClass,
    retry_class: RetryClass,
    status: LocalStackStatus,
}

pub fn capabilities_for_phase(master_phase: u8) -> Vec<LocalServiceCapability> {
    let mut capabilities = foundation_service_ids()
        .into_iter()
        .map(|service_id| LocalServiceCapability {
            service_id: service_id.to_owned(),
            phase_gate: "phase_0_foundation".to_owned(),
            available: true,
            reason_code: "local_stack.phase0_available".to_owned(),
        })
        .collect::<Vec<_>>();

    if master_phase > 0 {
        capabilities.extend(
            ["service:node_agent_simulator", "service:execution_loop"]
                .into_iter()
                .map(|service_id| LocalServiceCapability {
                    service_id: service_id.to_owned(),
                    phase_gate: format!("phase_{master_phase}_blocked"),
                    available: false,
                    reason_code: "phase.local_service_unavailable".to_owned(),
                }),
        );
    }

    capabilities
}

fn foundation_service_ids() -> [&'static str; 6] {
    [
        "service:embedded_state",
        "service:api",
        "service:worker",
        "service:overqueue_jobs",
        "service:overstore_stub",
        "service:event_audit",
    ]
}

fn foundation_service_health() -> Vec<LocalServiceHealth> {
    foundation_service_ids()
        .into_iter()
        .map(service_health_record)
        .collect()
}

fn service_health_for_manifest(manifest: &LocalStackManifest) -> Vec<LocalServiceHealth> {
    let mut service_ids = manifest.service_ids.clone();
    for service_id in foundation_service_ids() {
        if !service_ids.iter().any(|existing| existing == service_id) {
            service_ids.push(service_id.to_owned());
        }
    }
    service_ids
        .iter()
        .map(|service_id| service_health_record(service_id))
        .collect()
}

fn service_health_record(service_id: &str) -> LocalServiceHealth {
    let binding = binding_for_service(service_id);
    LocalServiceHealth {
        service_id: service_id.to_owned(),
        state: "ready".to_owned(),
        endpoint_ref: binding
            .map(|binding| binding.endpoint_ref.to_owned())
            .unwrap_or_else(|| local_service_endpoint_ref(service_id).to_owned()),
        bind_host: binding
            .map(|binding| binding.bind_host.to_owned())
            .unwrap_or_else(|| "local".to_owned()),
        port: binding.map(|binding| binding.port),
        loopback_only: binding
            .map(|binding| is_loopback_host(binding.bind_host))
            .unwrap_or(true),
        reason_code: "local_stack.service_ready".to_owned(),
    }
}

fn local_service_endpoint_ref(service_id: &str) -> &'static str {
    match service_id {
        "service:embedded_state" => "local-state://embedded_state/ready.marker",
        "service:overqueue_jobs" => "overqueue://local/jobs/ready.marker",
        "service:overstore_stub" => "artifact://local_stack/ready.marker",
        "service:event_audit" => "event-audit://local_stack/events/ready.marker",
        _ => "local-state://embedded_state/ready.marker",
    }
}

fn binding_for_service(service_id: &str) -> Option<ReservedPortBinding> {
    RESERVED_PORT_BINDINGS
        .into_iter()
        .find(|binding| binding.service_id == service_id)
}

fn reserved_port_bindings() -> Vec<ReservedPortBinding> {
    RESERVED_PORT_BINDINGS.to_vec()
}

fn detect_reserved_port_conflicts() -> Vec<ReservedPortConflict> {
    RESERVED_PORT_BINDINGS
        .into_iter()
        .filter_map(|binding| {
            if !is_loopback_host(binding.bind_host) {
                return Some(ReservedPortConflict {
                    service_id: binding.service_id,
                    port: binding.port,
                    bind_host: binding.bind_host,
                    purpose: binding.purpose,
                    endpoint_ref: binding.endpoint_ref,
                    reason_code: "local_stack.non_loopback_binding",
                    error_kind: "non_loopback_binding",
                    os_error_code: None,
                });
            }

            match TcpListener::bind((binding.bind_host, binding.port)) {
                Ok(listener) => {
                    drop(listener);
                    None
                }
                Err(error) => Some(ReservedPortConflict {
                    service_id: binding.service_id,
                    port: binding.port,
                    bind_host: binding.bind_host,
                    purpose: binding.purpose,
                    endpoint_ref: binding.endpoint_ref,
                    reason_code: "local_stack.port_conflict",
                    error_kind: bind_error_kind(&error),
                    os_error_code: error.raw_os_error(),
                }),
            }
        })
        .collect()
}

fn bind_error_kind(error: &std::io::Error) -> &'static str {
    match error.kind() {
        std::io::ErrorKind::AddrInUse => "address_in_use",
        std::io::ErrorKind::AddrNotAvailable => "address_not_available",
        std::io::ErrorKind::PermissionDenied => "permission_denied",
        _ => "bind_failed",
    }
}

fn local_env_variables() -> Vec<LocalEnvVariable> {
    LOCAL_ENV_VARIABLES.to_vec()
}

fn local_secret_records() -> Vec<LocalSecretRecord> {
    LOCAL_SECRET_RECORDS.to_vec()
}

fn local_state_records() -> Vec<LocalStateRecord> {
    [
        (
            "state:tenant:local_alpha",
            "tenant",
            "overbase://local_state/tenants/tenant:local:alpha",
        ),
        (
            "state:identity:local_builder",
            "identity",
            "overbase://local_state/identities/actor:local:builder",
        ),
        (
            "state:key:local_test_signing",
            "key",
            "overbase://local_state/keys/key:local:test_signing",
        ),
        (
            "state:manifest:local_noop",
            "manifest",
            "overbase://local_state/manifests/manifest:local:noop",
        ),
        (
            "state:event:service_ready_api",
            "event",
            "overbase://local_state/events/event:local_stack:service_ready:api",
        ),
        (
            "state:fixture_metadata:phase5",
            "fixture_metadata",
            "overbase://local_state/fixtures/fixture:local_stack:phase5",
        ),
        (
            "state:reset_marker:embedded_state",
            "reset_marker",
            "local-state://embedded_state/.overrid-local-test-state",
        ),
        (
            "state:schema_version:local_development_stack",
            "schema_version",
            "overbase://local_state/schema_versions/local-development-stack.v0.1",
        ),
    ]
    .into_iter()
    .map(|(record_id, record_kind, contract_ref)| LocalStateRecord {
        record_id,
        record_kind,
        contract_ref,
        schema_version: SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
        storage_boundary: "overbase_shaped_embedded_state",
        external_database_semantics: false,
        local_only: true,
        test_only: true,
    })
    .collect()
}

fn local_queue_job_records() -> Vec<LocalQueueJobRecord> {
    vec![
        LocalQueueJobRecord {
            job_id: "job:local:phase5_smoke_pending",
            state: "pending",
            idempotency_key: "idem:local_stack:phase5:smoke",
            idempotency_status: "accepted",
            trace_id: "trace_local_stack_phase5_queue",
            priority: 10,
            retry_count: 0,
            max_retries: 3,
            timeout_ms: 60000,
            dead_letter_reason_code: None,
            terminal: false,
            schema_version: SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
            reason_code: "local_stack.job_pending",
            local_only: true,
            test_only: true,
        },
        LocalQueueJobRecord {
            job_id: "job:local:phase5_smoke_duplicate",
            state: "duplicate_suppressed",
            idempotency_key: "idem:local_stack:phase5:smoke",
            idempotency_status: "duplicate_suppressed",
            trace_id: "trace_local_stack_phase5_queue",
            priority: 10,
            retry_count: 0,
            max_retries: 3,
            timeout_ms: 60000,
            dead_letter_reason_code: None,
            terminal: true,
            schema_version: SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
            reason_code: "local_stack.job_duplicate_idempotency_key",
            local_only: true,
            test_only: true,
        },
        LocalQueueJobRecord {
            job_id: "job:local:phase5_retry",
            state: "retry_scheduled",
            idempotency_key: "idem:local_stack:phase5:retry",
            idempotency_status: "accepted",
            trace_id: "trace_local_stack_phase5_retry",
            priority: 20,
            retry_count: 1,
            max_retries: 3,
            timeout_ms: 60000,
            dead_letter_reason_code: None,
            terminal: false,
            schema_version: SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
            reason_code: "local_stack.job_retry_scheduled",
            local_only: true,
            test_only: true,
        },
        LocalQueueJobRecord {
            job_id: "job:local:phase5_dead_letter",
            state: "dead_letter",
            idempotency_key: "idem:local_stack:phase5:dead_letter",
            idempotency_status: "accepted",
            trace_id: "trace_local_stack_phase5_dead_letter",
            priority: 90,
            retry_count: 3,
            max_retries: 3,
            timeout_ms: 60000,
            dead_letter_reason_code: Some("local_stack.job_timeout"),
            terminal: true,
            schema_version: SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
            reason_code: "local_stack.job_dead_lettered",
            local_only: true,
            test_only: true,
        },
    ]
}

fn local_artifact_manifests() -> Vec<LocalArtifactManifest> {
    let payload = local_artifact_payload();
    let content_hash = blake3_hex(payload.as_bytes());
    vec![LocalArtifactManifest {
        artifact_ref: "artifact://local_stack/phase5/noop_payload",
        object_manifest_ref: "overstore://local_stub/manifests/phase5_noop_payload",
        hash_algorithm: "BLAKE3",
        content_address: format!("overstore://local_stub/blake3/{content_hash}"),
        content_hash,
        byte_length: payload.len(),
        upload_grant_ref: "grant://local_stack/upload/test_only",
        download_grant_ref: "grant://local_stack/download/test_only",
        reset_marker_ref: "artifact://local_stack/.overrid-local-test-state",
        reset_safe: true,
        filesystem_backed: true,
        external_object_store_boundary: false,
        local_only: true,
        test_only: true,
    }]
}

fn local_audit_query_records() -> Vec<LocalAuditQueryRecord> {
    vec![
        LocalAuditQueryRecord {
            query_id: "audit_query:local_stack:service_ready",
            event_ref: "event:local_stack:service_ready:api",
            event_type: "local_stack.service_ready",
            trace_id: "trace_local_stack_phase5_audit",
            service_id: "service:api",
            time_window_ref: "time_window:local_stack_phase5_ready",
            local_diagnostic_event: true,
            production_overwatch_authority: false,
            redaction_summary: "secret_free",
            contains_raw_secret: false,
            reason_code: "local_stack.local_diagnostic_event",
            local_only: true,
            test_only: true,
        },
        LocalAuditQueryRecord {
            query_id: "audit_query:local_stack:dead_letter",
            event_ref: "event:local_stack:job_dead_lettered:phase5",
            event_type: "local_stack.job_dead_lettered",
            trace_id: "trace_local_stack_phase5_dead_letter",
            service_id: "service:overqueue_jobs",
            time_window_ref: "time_window:local_stack_phase5_queue",
            local_diagnostic_event: true,
            production_overwatch_authority: false,
            redaction_summary: "secret_free",
            contains_raw_secret: false,
            reason_code: "local_stack.local_diagnostic_event",
            local_only: true,
            test_only: true,
        },
    ]
}

fn schema_compatibility_gates_for_profile(profile: &str) -> Vec<SchemaCompatibilityGate> {
    let normalized = profile.to_ascii_lowercase();
    [
        ("local_state", "local_state"),
        ("overqueue_job", "queue"),
        ("overstore_artifact_manifest", "artifact"),
        ("fixture_manifest", "fixture"),
        ("service_endpoint", "endpoint"),
    ]
    .into_iter()
    .map(|(surface, profile_key)| {
        let hyphenated_key = profile_key.replace('_', "-");
        let incompatible = normalized.contains(&format!("stale-{profile_key}-schema"))
            || normalized.contains(&format!("incompatible-{profile_key}-schema"))
            || normalized.contains(&format!("stale-{hyphenated_key}-schema"))
            || normalized.contains(&format!("incompatible-{hyphenated_key}-schema"));
        let schema_version = if incompatible {
            "local-development-stack.v99.0".to_owned()
        } else {
            SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION.to_owned()
        };
        let reason_code = if incompatible {
            format!("local_stack.{surface}_schema_incompatible")
        } else {
            "local_stack.schema_compatible".to_owned()
        };
        SchemaCompatibilityGate {
            gate_id: format!("schema_gate:{surface}"),
            surface,
            schema_version,
            compatible: !incompatible,
            reason_code,
            blocks_start: true,
            blocks_seed: true,
            blocks_smoke: true,
            evidence_ref: format!("compatibility://local_stack/{surface}"),
        }
    })
    .collect()
}

fn ready_doctor_checks() -> Vec<LocalDoctorCheck> {
    READY_DOCTOR_CHECKS.to_vec()
}

fn doctor_checks_for_profile(profile: &str) -> Vec<LocalDoctorCheck> {
    let normalized = profile.to_ascii_lowercase();
    if normalized.contains("missing-runtime") {
        return doctor_checks_for_reason("doctor.rust_toolchain_missing");
    }
    if normalized.contains("wrong-permissions") {
        return doctor_checks_for_reason("doctor.file_permissions_invalid");
    }
    if normalized.contains("stale-schemas") {
        return doctor_checks_for_reason("doctor.schemas_stale");
    }
    if normalized.contains("missing-profile") {
        return doctor_checks_for_reason("doctor.profile_missing");
    }
    if normalized.contains("unsafe-env") {
        return doctor_checks_for_reason("doctor.unsafe_env_value");
    }
    if normalized.contains("port-conflict") {
        return doctor_checks_for_reason("local_stack.port_conflict");
    }
    ready_doctor_checks()
}

fn doctor_checks_for_reason(reason_code: &str) -> Vec<LocalDoctorCheck> {
    let mut checks = ready_doctor_checks();
    let failure = match reason_code {
        "doctor.rust_toolchain_missing" => Some((
            "doctor:rust_toolchain",
            "doctor.rust_toolchain_missing",
            "install the repository-pinned Rust toolchain",
        )),
        "doctor.file_permissions_invalid" => Some((
            "doctor:file_permissions",
            "doctor.file_permissions_invalid",
            "fix local stack volume and env file permissions",
        )),
        "doctor.schemas_stale" | "manifest.schema_version_incompatible" => Some((
            "doctor:schema_outputs",
            "doctor.schemas_stale",
            "regenerate shared schema outputs",
        )),
        "local_stack.schema_version_incompatible" => Some((
            "doctor:schema_outputs",
            "local_stack.schema_version_incompatible",
            "reset local backing records and regenerate local-stack schema fixtures",
        )),
        "doctor.profile_missing" => Some((
            "doctor:repo_layout",
            "doctor.profile_missing",
            "select an existing local or CI test profile",
        )),
        "doctor.unsafe_env_value" => Some((
            "doctor:local_env_manifest",
            "doctor.unsafe_env_value",
            "remove unsafe or raw secret values from local env inputs",
        )),
        "local_stack.port_conflict" => Some((
            "doctor:reserved_ports",
            "local_stack.port_conflict",
            "free the reserved loopback port range 18080-18085",
        )),
        "profile.not_local_test" => Some((
            "doctor:profile_scope",
            "profile.not_local_test",
            "select a local or CI test profile before running local-stack commands",
        )),
        _ if reason_code.starts_with("manifest.") => Some((
            "doctor:schema_outputs",
            "manifest.validation_failed",
            "fix the canonical local development stack manifest",
        )),
        _ => None,
    };

    if let Some((check_id, failed_reason_code, remediation_hint)) = failure {
        if let Some(existing) = checks.iter_mut().find(|check| check.check_id == check_id) {
            *existing = LocalDoctorCheck {
                check_id,
                state: "failed",
                reason_code: failed_reason_code,
                remediation_hint,
            };
        } else {
            checks.push(LocalDoctorCheck {
                check_id,
                state: "failed",
                reason_code: failed_reason_code,
                remediation_hint,
            });
        }
    }
    checks
}

fn profile_blocker(profile: &str) -> Option<LocalStackFailure> {
    let normalized = profile.to_ascii_lowercase();
    if normalized.contains("seed")
        || normalized.contains("staging")
        || normalized.contains("production")
    {
        return Some(LocalStackFailure {
            reason_code: "profile.not_local_test",
            message: "local stack commands require a local or CI test profile",
            exit_class: ExitCodeClass::Config,
            retry_class: RetryClass::OperatorReview,
            status: LocalStackStatus::Blocked,
        });
    }
    None
}

fn profile_backing_failure(profile: &str) -> Option<LocalStackFailure> {
    let normalized = profile.to_ascii_lowercase();
    if normalized.contains("missing-runtime") {
        return Some(LocalStackFailure {
            reason_code: "doctor.rust_toolchain_missing",
            message: "Rust toolchain prerequisite is missing",
            exit_class: ExitCodeClass::Config,
            retry_class: RetryClass::OperatorReview,
            status: LocalStackStatus::Blocked,
        });
    }
    if normalized.contains("wrong-permissions") {
        return Some(LocalStackFailure {
            reason_code: "doctor.file_permissions_invalid",
            message: "local stack file permissions are not safe for reset or env generation",
            exit_class: ExitCodeClass::LocalIo,
            retry_class: RetryClass::OperatorReview,
            status: LocalStackStatus::Blocked,
        });
    }
    if normalized.contains("stale-schemas") {
        return Some(LocalStackFailure {
            reason_code: "doctor.schemas_stale",
            message: "local stack schema generation outputs are stale",
            exit_class: ExitCodeClass::Schema,
            retry_class: RetryClass::OperatorReview,
            status: LocalStackStatus::Blocked,
        });
    }
    if normalized.contains("missing-profile") {
        return Some(LocalStackFailure {
            reason_code: "doctor.profile_missing",
            message: "selected local stack profile is missing",
            exit_class: ExitCodeClass::Config,
            retry_class: RetryClass::OperatorReview,
            status: LocalStackStatus::Blocked,
        });
    }
    if normalized.contains("unsafe-env") {
        return Some(LocalStackFailure {
            reason_code: "doctor.unsafe_env_value",
            message: "local env inputs contain unsafe or raw secret values",
            exit_class: ExitCodeClass::Config,
            retry_class: RetryClass::OperatorReview,
            status: LocalStackStatus::Blocked,
        });
    }
    if normalized.contains("health-timeout")
        || normalized.contains("backing-services-unavailable")
        || normalized.contains("unavailable")
    {
        return Some(LocalStackFailure {
            reason_code: "local_stack.backing_services_unavailable",
            message: "local backing service prerequisites are unavailable",
            exit_class: ExitCodeClass::Platform,
            retry_class: RetryClass::SafeRetry,
            status: LocalStackStatus::Blocked,
        });
    }
    if normalized.contains("port-conflict") {
        return Some(LocalStackFailure {
            reason_code: "local_stack.port_conflict",
            message: "reserved local development port is already in use",
            exit_class: ExitCodeClass::Config,
            retry_class: RetryClass::OperatorReview,
            status: LocalStackStatus::Blocked,
        });
    }
    None
}

fn transition_allowed(previous: LocalCommandState, next: LocalCommandState) -> bool {
    use LocalCommandState::{
        Blocked, CollectingArtifacts, Completed, Failed, Planned, PrerequisitesChecked, Ready,
        Resetting, Seeding, Smoking, Starting, Stopped,
    };
    match previous {
        Planned => matches!(next, PrerequisitesChecked | Blocked | Failed),
        PrerequisitesChecked => matches!(
            next,
            Starting
                | Ready
                | Resetting
                | Seeding
                | Smoking
                | CollectingArtifacts
                | Stopped
                | Blocked
                | Failed
        ),
        Starting => matches!(next, Ready | Blocked | Failed),
        Ready => matches!(
            next,
            Resetting
                | Seeding
                | Smoking
                | CollectingArtifacts
                | Stopped
                | Completed
                | Blocked
                | Failed
        ),
        Resetting => matches!(
            next,
            Seeding | CollectingArtifacts | Completed | Blocked | Failed
        ),
        Seeding => matches!(
            next,
            Smoking | CollectingArtifacts | Completed | Blocked | Failed
        ),
        Smoking => matches!(next, CollectingArtifacts | Completed | Blocked | Failed),
        CollectingArtifacts => matches!(next, Completed | Blocked | Failed),
        Stopped => matches!(next, Starting | Completed | Blocked | Failed),
        Blocked | Failed | Completed => false,
    }
}

fn looks_like_json_object(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed.starts_with('{') && trimmed.ends_with('}') && balanced_delimiters(trimmed)
}

fn balanced_delimiters(text: &str) -> bool {
    let mut braces = 0_i32;
    let mut brackets = 0_i32;
    let mut in_string = false;
    let mut escaped = false;
    for character in text.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        if in_string {
            if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }
        match character {
            '"' => in_string = true,
            '{' => braces += 1,
            '}' => braces -= 1,
            '[' => brackets += 1,
            ']' => brackets -= 1,
            _ => {}
        }
        if braces < 0 || brackets < 0 {
            return false;
        }
    }
    braces == 0 && brackets == 0 && !in_string
}

fn extract_values_for_key(text: &str, key: &str) -> Vec<String> {
    let needle = format!("\"{key}\"");
    let mut offset = 0;
    let mut values = Vec::new();
    while let Some(relative) = text[offset..].find(&needle) {
        let key_start = offset + relative;
        let after_key = key_start + needle.len();
        let Some(colon_relative) = text[after_key..].find(':') else {
            break;
        };
        let value_start = after_key + colon_relative + 1;
        let trimmed_start = value_start + leading_whitespace_len(&text[value_start..]);
        if let Some((value, consumed)) = parse_json_string(&text[trimmed_start..]) {
            values.push(value);
            offset = trimmed_start + consumed;
        } else {
            offset = after_key;
        }
    }
    values
}

fn extract_string_arrays_for_key(text: &str, key: &str) -> Vec<String> {
    let needle = format!("\"{key}\"");
    let mut offset = 0;
    let mut values = Vec::new();
    while let Some(relative) = text[offset..].find(&needle) {
        let key_start = offset + relative;
        let after_key = key_start + needle.len();
        let Some(colon_relative) = text[after_key..].find(':') else {
            break;
        };
        let value_start = after_key + colon_relative + 1;
        let Some(array_relative) = text[value_start..].find('[') else {
            offset = after_key;
            continue;
        };
        let array_start = value_start + array_relative;
        let Some(array_end) = find_matching_delimiter(text, array_start, '[', ']') else {
            offset = after_key;
            continue;
        };
        values.extend(extract_all_strings(&text[array_start..=array_end]));
        offset = array_end + 1;
    }
    values
}

fn extract_array_section<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("\"{key}\"");
    let key_start = text.find(&needle)?;
    let after_key = key_start + needle.len();
    let colon_relative = text[after_key..].find(':')?;
    let value_start = after_key + colon_relative + 1;
    let array_relative = text[value_start..].find('[')?;
    let array_start = value_start + array_relative;
    let array_end = find_matching_delimiter(text, array_start, '[', ']')?;
    Some(&text[array_start..=array_end])
}

fn find_matching_delimiter(text: &str, start: usize, open: char, close: char) -> Option<usize> {
    let mut depth = 0_i32;
    let mut in_string = false;
    let mut escaped = false;
    for (relative, character) in text[start..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if in_string {
            if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }
        if character == '"' {
            in_string = true;
        } else if character == open {
            depth += 1;
        } else if character == close {
            depth -= 1;
            if depth == 0 {
                return Some(start + relative);
            }
        }
    }
    None
}

fn extract_all_strings(text: &str) -> Vec<String> {
    let mut offset = 0;
    let mut values = Vec::new();
    while let Some(relative) = text[offset..].find('"') {
        let start = offset + relative;
        if let Some((value, consumed)) = parse_json_string(&text[start..]) {
            values.push(value);
            offset = start + consumed;
        } else {
            break;
        }
    }
    values
}

fn parse_json_string(text: &str) -> Option<(String, usize)> {
    if !text.starts_with('"') {
        return None;
    }
    let mut value = String::new();
    let mut escaped = false;
    for (index, character) in text.char_indices().skip(1) {
        if escaped {
            value.push(character);
            escaped = false;
            continue;
        }
        match character {
            '\\' => escaped = true,
            '"' => return Some((value, index + 1)),
            other => value.push(other),
        }
    }
    None
}

fn leading_whitespace_len(text: &str) -> usize {
    text.len() - text.trim_start().len()
}

fn is_safe_endpoint(value: &str) -> bool {
    if matches!(value, "127.0.0.1" | "localhost" | "::1") {
        return true;
    }
    if value.starts_with("http://127.0.0.1:")
        || value.starts_with("http://localhost:")
        || value.starts_with("http://[::1]:")
        || value.starts_with("local-state://")
        || value.starts_with("log://")
        || value.starts_with("artifact://")
        || value.starts_with("fixture://")
        || value.starts_with("env://")
        || value.starts_with("secret://")
    {
        return true;
    }
    let lowered = value.to_ascii_lowercase();
    !(lowered.starts_with("http://")
        || lowered.starts_with("https://")
        || lowered.contains("0.0.0.0")
        || lowered.contains("production")
        || lowered.contains("staging")
        || lowered.contains("seed.overrid"))
}

fn is_loopback_host(value: &str) -> bool {
    matches!(value, "127.0.0.1" | "localhost" | "::1")
}

fn render_capabilities_json(capabilities: &[LocalServiceCapability]) -> String {
    let rendered = capabilities
        .iter()
        .map(|capability| {
            format!(
                concat!(
                    "{{",
                    "\"service_id\":\"{}\",",
                    "\"phase_gate\":\"{}\",",
                    "\"available\":{},",
                    "\"reason_code\":\"{}\"",
                    "}}"
                ),
                json_escape(&capability.service_id),
                json_escape(&capability.phase_gate),
                capability.available,
                json_escape(&capability.reason_code),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_service_health_json(service_health: &[LocalServiceHealth]) -> String {
    let rendered = service_health
        .iter()
        .map(|health| {
            format!(
                concat!(
                    "{{",
                    "\"service_id\":\"{}\",",
                    "\"state\":\"{}\",",
                    "\"endpoint_ref\":\"{}\",",
                    "\"bind_host\":\"{}\",",
                    "\"port\":{},",
                    "\"loopback_only\":{},",
                    "\"reason_code\":\"{}\"",
                    "}}"
                ),
                json_escape(&health.service_id),
                json_escape(&health.state),
                json_escape(&health.endpoint_ref),
                json_escape(&health.bind_host),
                json_optional_u16(health.port),
                health.loopback_only,
                json_escape(&health.reason_code),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_port_registry_json(bindings: &[ReservedPortBinding]) -> String {
    let rendered = bindings
        .iter()
        .map(|binding| {
            format!(
                concat!(
                    "{{",
                    "\"service_id\":\"{}\",",
                    "\"port\":{},",
                    "\"bind_host\":\"{}\",",
                    "\"purpose\":\"{}\",",
                    "\"endpoint_ref\":\"{}\",",
                    "\"loopback_only\":{}",
                    "}}"
                ),
                json_escape(binding.service_id),
                binding.port,
                json_escape(binding.bind_host),
                json_escape(binding.purpose),
                json_escape(binding.endpoint_ref),
                is_loopback_host(binding.bind_host),
            )
        })
        .collect::<Vec<_>>();
    format!(
        concat!(
            "{{",
            "\"registry_id\":\"port_registry:reserved_18080_18085\",",
            "\"collision_policy\":\"fail_before_startup\",",
            "\"local_only\":true,",
            "\"test_only\":true,",
            "\"bindings\":[{}]",
            "}}"
        ),
        rendered.join(","),
    )
}

fn render_port_conflicts_json(conflicts: &[ReservedPortConflict]) -> String {
    let rendered = conflicts
        .iter()
        .map(|conflict| {
            format!(
                concat!(
                    "{{",
                    "\"service_id\":\"{}\",",
                    "\"port\":{},",
                    "\"bind_host\":\"{}\",",
                    "\"purpose\":\"{}\",",
                    "\"endpoint_ref\":\"{}\",",
                    "\"reason_code\":\"{}\",",
                    "\"error_kind\":\"{}\",",
                    "\"os_error_code\":{}",
                    "}}"
                ),
                json_escape(conflict.service_id),
                conflict.port,
                json_escape(conflict.bind_host),
                json_escape(conflict.purpose),
                json_escape(conflict.endpoint_ref),
                json_escape(conflict.reason_code),
                json_escape(conflict.error_kind),
                json_optional_i32(conflict.os_error_code),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_env_manifest_json(variables: &[LocalEnvVariable]) -> String {
    let rendered = variables
        .iter()
        .map(|variable| {
            format!(
                concat!(
                    "{{",
                    "\"name\":\"{}\",",
                    "\"value_ref\":\"{}\",",
                    "\"redacted\":{},",
                    "\"example_only\":{}",
                    "}}"
                ),
                json_escape(variable.name),
                json_escape(variable.value_ref),
                variable.redacted,
                variable.example_only,
            )
        })
        .collect::<Vec<_>>();
    format!(
        concat!(
            "{{",
            "\"manifest_id\":\"env_manifest:local_default\",",
            "\"generated_env_target\":\"repo://.env.local\",",
            "\"example_env_target\":\"repo://{}\",",
            "\"redaction_policy\":\"secret_free_refs_only\",",
            "\"contains_raw_secret\":false,",
            "\"example_values_include_raw_secrets\":false,",
            "\"drift_check\":\"schema_checked_names_only\",",
            "\"local_only\":true,",
            "\"test_only\":true,",
            "\"variables\":[{}]",
            "}}"
        ),
        json_escape(LOCAL_STACK_ENV_EXAMPLE_PATH),
        rendered.join(","),
    )
}

fn render_secret_records_json(records: &[LocalSecretRecord]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"secret_ref\":\"{}\",",
                    "\"secret_kind\":\"{}\",",
                    "\"redaction_ref\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{},",
                    "\"raw_secret_present\":{}",
                    "}}"
                ),
                json_escape(record.secret_ref),
                json_escape(record.secret_kind),
                json_escape(record.redaction_ref),
                record.local_only,
                record.test_only,
                record.raw_secret_present,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_doctor_checks_json(checks: &[LocalDoctorCheck]) -> String {
    let rendered = checks
        .iter()
        .map(|check| {
            format!(
                concat!(
                    "{{",
                    "\"check_id\":\"{}\",",
                    "\"state\":\"{}\",",
                    "\"reason_code\":\"{}\",",
                    "\"remediation_hint\":\"{}\"",
                    "}}"
                ),
                json_escape(check.check_id),
                json_escape(check.state),
                json_escape(check.reason_code),
                json_escape(check.remediation_hint),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_local_state_records_json(records: &[LocalStateRecord]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"record_id\":\"{}\",",
                    "\"record_kind\":\"{}\",",
                    "\"contract_ref\":\"{}\",",
                    "\"schema_version\":\"{}\",",
                    "\"storage_boundary\":\"{}\",",
                    "\"external_database_semantics\":{},",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(record.record_id),
                json_escape(record.record_kind),
                json_escape(record.contract_ref),
                json_escape(record.schema_version),
                json_escape(record.storage_boundary),
                record.external_database_semantics,
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_queue_job_records_json(records: &[LocalQueueJobRecord]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"job_id\":\"{}\",",
                    "\"state\":\"{}\",",
                    "\"idempotency_key\":\"{}\",",
                    "\"idempotency_status\":\"{}\",",
                    "\"trace_id\":\"{}\",",
                    "\"priority\":{},",
                    "\"retry_count\":{},",
                    "\"max_retries\":{},",
                    "\"timeout_ms\":{},",
                    "\"dead_letter_reason_code\":{},",
                    "\"terminal\":{},",
                    "\"schema_version\":\"{}\",",
                    "\"reason_code\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(record.job_id),
                json_escape(record.state),
                json_escape(record.idempotency_key),
                json_escape(record.idempotency_status),
                json_escape(record.trace_id),
                record.priority,
                record.retry_count,
                record.max_retries,
                record.timeout_ms,
                json_optional_static_string(record.dead_letter_reason_code),
                record.terminal,
                json_escape(record.schema_version),
                json_escape(record.reason_code),
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_artifact_manifests_json(manifests: &[LocalArtifactManifest]) -> String {
    let rendered = manifests
        .iter()
        .map(|manifest| {
            format!(
                concat!(
                    "{{",
                    "\"artifact_ref\":\"{}\",",
                    "\"object_manifest_ref\":\"{}\",",
                    "\"hash_algorithm\":\"{}\",",
                    "\"content_hash\":\"{}\",",
                    "\"content_address\":\"{}\",",
                    "\"byte_length\":{},",
                    "\"upload_grant_ref\":\"{}\",",
                    "\"download_grant_ref\":\"{}\",",
                    "\"reset_marker_ref\":\"{}\",",
                    "\"reset_safe\":{},",
                    "\"filesystem_backed\":{},",
                    "\"external_object_store_boundary\":{},",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(manifest.artifact_ref),
                json_escape(manifest.object_manifest_ref),
                json_escape(manifest.hash_algorithm),
                json_escape(&manifest.content_hash),
                json_escape(&manifest.content_address),
                manifest.byte_length,
                json_escape(manifest.upload_grant_ref),
                json_escape(manifest.download_grant_ref),
                json_escape(manifest.reset_marker_ref),
                manifest.reset_safe,
                manifest.filesystem_backed,
                manifest.external_object_store_boundary,
                manifest.local_only,
                manifest.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_audit_query_records_json(records: &[LocalAuditQueryRecord]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"query_id\":\"{}\",",
                    "\"event_ref\":\"{}\",",
                    "\"event_type\":\"{}\",",
                    "\"trace_id\":\"{}\",",
                    "\"service_id\":\"{}\",",
                    "\"time_window_ref\":\"{}\",",
                    "\"local_diagnostic_event\":{},",
                    "\"production_overwatch_authority\":{},",
                    "\"redaction_summary\":\"{}\",",
                    "\"contains_raw_secret\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(record.query_id),
                json_escape(record.event_ref),
                json_escape(record.event_type),
                json_escape(record.trace_id),
                json_escape(record.service_id),
                json_escape(record.time_window_ref),
                record.local_diagnostic_event,
                record.production_overwatch_authority,
                json_escape(record.redaction_summary),
                record.contains_raw_secret,
                json_escape(record.reason_code),
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_schema_compatibility_gates_json(gates: &[SchemaCompatibilityGate]) -> String {
    let rendered = gates
        .iter()
        .map(|gate| {
            format!(
                concat!(
                    "{{",
                    "\"gate_id\":\"{}\",",
                    "\"surface\":\"{}\",",
                    "\"schema_version\":\"{}\",",
                    "\"compatible\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"blocks_start\":{},",
                    "\"blocks_seed\":{},",
                    "\"blocks_smoke\":{},",
                    "\"evidence_ref\":\"{}\"",
                    "}}"
                ),
                json_escape(&gate.gate_id),
                json_escape(gate.surface),
                json_escape(&gate.schema_version),
                gate.compatible,
                json_escape(&gate.reason_code),
                gate.blocks_start,
                gate.blocks_seed,
                gate.blocks_smoke,
                json_escape(&gate.evidence_ref),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn json_owned_string_array(values: &[String]) -> String {
    let rendered = values
        .iter()
        .map(|value| format!("\"{}\"", json_escape(value)))
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn json_optional_u64(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_owned())
}

fn json_optional_u16(value: Option<u16>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_owned())
}

fn json_optional_i32(value: Option<i32>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_owned())
}

fn json_optional_static_string(value: Option<&'static str>) -> String {
    value
        .map(|value| format!("\"{}\"", json_escape(value)))
        .unwrap_or_else(|| "null".to_owned())
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::new();
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            other => escaped.push(other),
        }
    }
    escaped
}

fn local_artifact_payload() -> &'static str {
    "overrid-local-stack-phase5-noop-payload:local_only:test_only"
}

fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn remediation_hint(reason_code: &str) -> &'static str {
    match reason_code {
        "profile.not_local_test" => {
            "select a local or CI test profile before running local-stack commands"
        }
        "phase.local_service_unavailable" => {
            "select phase 0 or wait until the requested phase service simulator is implemented"
        }
        "local_stack.backing_services_unavailable" => {
            "rerun doctor and inspect redacted diagnostic refs for unavailable local prerequisites"
        }
        "local_stack.port_conflict" => "free the reserved loopback port range 18080-18085",
        "local_stack.schema_version_incompatible" => {
            "reset local backing records and regenerate local-stack schema fixtures"
        }
        "manifest.read_failed"
        | "manifest.invalid_json"
        | "manifest.schema_version_missing"
        | "manifest.schema_version_incompatible"
        | "manifest.local_test_markers_missing"
        | "manifest.service_definitions_missing"
        | "manifest.duplicate_service_id"
        | "manifest.missing_dependency"
        | "manifest.unsafe_endpoint" => "fix the canonical local development stack manifest",
        _ => "inspect redacted local-stack diagnostics",
    }
}

fn id_component(value: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("local-stack crate lives under packages/")
            .to_path_buf()
    }

    fn test_options() -> LocalStackOptions {
        let mut options = LocalStackOptions::new(repo_root());
        options.port_preflight = false;
        options
    }

    fn minimal_manifest() -> String {
        r#"{
  "schema_version": "local-development-stack.v0.1",
  "local_only": true,
  "test_only": true,
  "service_definitions": [
    {
      "service_id": "service:embedded_state",
      "depends_on": [],
      "health_check": { "endpoint": "local-state://embedded_state/ready.marker" }
    },
    {
      "service_id": "service:api",
      "depends_on": ["service:embedded_state"],
      "health_check": { "endpoint": "http://127.0.0.1:18080/healthz" }
    }
  ]
}"#
        .to_owned()
    }

    #[test]
    fn loads_default_manifest() {
        let manifest = LocalStackManifest::load_default(repo_root()).unwrap();
        assert_eq!(
            manifest.schema_version,
            SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION
        );
        assert!(manifest
            .service_ids
            .contains(&"service:embedded_state".to_owned()));
        assert!(manifest.service_ids.contains(&"service:api".to_owned()));
        assert!(manifest.service_ids.contains(&"service:worker".to_owned()));
    }

    #[test]
    fn rejects_invalid_json() {
        assert_eq!(
            LocalStackManifest::validate_text("not json").unwrap_err(),
            ManifestValidationError::InvalidJson
        );
    }

    #[test]
    fn rejects_incompatible_schema_version() {
        let text = minimal_manifest().replace(
            "local-development-stack.v0.1",
            "local-development-stack.v99.0",
        );
        assert!(matches!(
            LocalStackManifest::validate_text(&text).unwrap_err(),
            ManifestValidationError::IncompatibleSchemaVersion { .. }
        ));
    }

    #[test]
    fn rejects_duplicate_service_id() {
        let text = minimal_manifest().replace(
            "\"service_id\": \"service:api\"",
            "\"service_id\": \"service:embedded_state\"",
        );
        assert_eq!(
            LocalStackManifest::validate_text(&text).unwrap_err(),
            ManifestValidationError::DuplicateServiceId("service:embedded_state".to_owned())
        );
    }

    #[test]
    fn rejects_missing_dependency() {
        let text = minimal_manifest().replace(
            "\"depends_on\": [\"service:embedded_state\"]",
            "\"depends_on\": [\"service:missing\"]",
        );
        assert_eq!(
            LocalStackManifest::validate_text(&text).unwrap_err(),
            ManifestValidationError::MissingDependency("service:missing".to_owned())
        );
    }

    #[test]
    fn rejects_unsafe_endpoint() {
        let text =
            minimal_manifest().replace("http://127.0.0.1:18080/healthz", "http://0.0.0.0:18080");
        assert_eq!(
            LocalStackManifest::validate_text(&text).unwrap_err(),
            ManifestValidationError::UnsafeEndpoint("http://0.0.0.0:18080".to_owned())
        );
    }

    #[test]
    fn phase0_capabilities_expose_foundation_only() {
        let capabilities = capabilities_for_phase(0);
        assert_eq!(capabilities.len(), 6);
        assert!(capabilities.iter().all(|capability| capability.available));
        assert!(capabilities
            .iter()
            .any(|capability| capability.service_id == "service:api"));
        assert!(capabilities
            .iter()
            .any(|capability| capability.service_id == "service:overqueue_jobs"));
        assert!(capabilities
            .iter()
            .any(|capability| capability.service_id == "service:overstore_stub"));
        assert!(capabilities
            .iter()
            .any(|capability| capability.service_id == "service:event_audit"));
    }

    #[test]
    fn later_phase_capabilities_include_blocked_reasons() {
        let capabilities = capabilities_for_phase(2);
        assert!(capabilities
            .iter()
            .any(|capability| capability.reason_code == "phase.local_service_unavailable"));
        assert!(capabilities.iter().any(|capability| !capability.available));
    }

    #[test]
    fn runs_smoke_lifecycle() {
        let mut options = test_options();
        options.trace_id = "trace_smoke_test".to_owned();
        let output = LocalStackRunner::new(options).run(DevCommand::Smoke);
        assert!(output.is_ok());
        assert_eq!(output.reason_code, "local_stack.smoke_passed");
        let states = output.lifecycle_strs();
        for expected in [
            "planned",
            "prerequisites_checked",
            "starting",
            "ready",
            "resetting",
            "seeding",
            "smoking",
            "collecting_artifacts",
            "completed",
        ] {
            assert!(states.contains(&expected));
        }
    }

    #[test]
    fn selected_future_phase_blocks_smoke() {
        let mut options = test_options();
        options.master_phase = 2;
        let output = LocalStackRunner::new(options).run(DevCommand::Smoke);
        assert!(!output.is_ok());
        assert_eq!(output.exit_class, ExitCodeClass::Phase);
        assert_eq!(output.reason_code, "phase.local_service_unavailable");
        assert!(output.lifecycle_strs().contains(&"blocked"));
    }

    #[test]
    fn reserved_port_registry_matches_phase4_defaults() {
        let bindings = reserved_port_bindings();
        assert_eq!(bindings.len(), 6);
        assert_eq!(
            bindings
                .iter()
                .map(|binding| binding.port)
                .collect::<Vec<_>>(),
            vec![18080, 18081, 18082, 18083, 18084, 18085]
        );
        assert!(
            bindings
                .iter()
                .all(|binding| binding.bind_host == "127.0.0.1"
                    && is_loopback_host(binding.bind_host))
        );
        assert!(bindings
            .iter()
            .any(|binding| binding.service_id == "service:worker"
                && binding.purpose == "worker_health_metrics"));
    }

    #[test]
    fn loopback_binding_metadata_is_captured_in_health() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Doctor);
        assert!(output.is_ok());
        let api = output
            .service_health
            .iter()
            .find(|health| health.service_id == "service:api")
            .expect("api health is present");
        assert_eq!(api.bind_host, "127.0.0.1");
        assert_eq!(api.port, Some(18080));
        assert!(api.loopback_only);
        assert!(output
            .result_json()
            .contains("\"topology_phase_gate\":\"phase_4_loopback_topology\""));
    }

    #[test]
    fn env_manifest_and_secret_records_are_redacted() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        assert!(output
            .env_variables
            .iter()
            .any(|variable| variable.name == "OVERRID_LOCAL_TEST_SECRET_REF"
                && variable.redacted
                && variable.value_ref.starts_with("secret://")));
        assert!(output
            .secret_records
            .iter()
            .all(|record| record.local_only && record.test_only && !record.raw_secret_present));
        let rendered = output.result_json().to_ascii_lowercase();
        assert!(rendered.contains("\"example_env_target\":\"repo://.env.example\""));
        for forbidden in [
            "password=",
            "token=",
            "api_key=",
            "private key",
            "-----begin",
        ] {
            assert!(!rendered.contains(forbidden));
        }
    }

    #[test]
    fn phase5_local_state_records_are_overbase_shaped() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        let kinds = output
            .local_state_records
            .iter()
            .map(|record| record.record_kind)
            .collect::<BTreeSet<_>>();
        for expected in [
            "tenant",
            "identity",
            "key",
            "manifest",
            "event",
            "fixture_metadata",
            "reset_marker",
            "schema_version",
        ] {
            assert!(kinds.contains(expected));
        }
        assert!(output.local_state_records.iter().all(|record| {
            record.local_only
                && record.test_only
                && !record.external_database_semantics
                && record.storage_boundary == "overbase_shaped_embedded_state"
                && record.schema_version == SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION
                && (record.contract_ref.starts_with("overbase://")
                    || record.contract_ref.starts_with("local-state://"))
        }));
        assert!(output
            .result_json()
            .contains("\"backing_phase_gate\":\"phase_5_embedded_state_queue_store\""));
    }

    #[test]
    fn phase5_queue_records_model_idempotency_retries_and_dead_letters() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        let duplicate_count = output
            .queue_job_records
            .iter()
            .filter(|job| job.idempotency_key == "idem:local_stack:phase5:smoke")
            .count();
        assert_eq!(duplicate_count, 2);
        assert!(output.queue_job_records.iter().any(|job| {
            job.state == "duplicate_suppressed"
                && job.idempotency_status == "duplicate_suppressed"
                && job.terminal
        }));
        assert!(output
            .queue_job_records
            .iter()
            .any(|job| job.state == "retry_scheduled" && job.retry_count == 1));
        assert!(output.queue_job_records.iter().any(|job| {
            job.state == "dead_letter"
                && job.dead_letter_reason_code == Some("local_stack.job_timeout")
                && job.terminal
        }));
    }

    #[test]
    fn phase5_artifact_stub_uses_blake3_content_addressing() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        let artifact = output
            .artifact_manifests
            .first()
            .expect("phase 5 artifact manifest exists");
        assert_eq!(artifact.hash_algorithm, "BLAKE3");
        assert_eq!(
            artifact.content_hash,
            blake3_hex(local_artifact_payload().as_bytes())
        );
        assert!(artifact.content_address.ends_with(&artifact.content_hash));
        assert!(artifact.filesystem_backed);
        assert!(artifact.reset_safe);
        assert!(!artifact.external_object_store_boundary);
        assert!(artifact.local_only && artifact.test_only);
    }

    #[test]
    fn phase5_audit_query_is_not_authoritative_overwatch() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        assert!(output.audit_query_records.iter().all(|record| {
            record.local_diagnostic_event
                && !record.production_overwatch_authority
                && !record.contains_raw_secret
                && record.redaction_summary == "secret_free"
                && record.local_only
                && record.test_only
        }));
        assert!(output
            .audit_query_records
            .iter()
            .any(|record| record.service_id == "service:overqueue_jobs"));
    }

    #[test]
    fn phase5_schema_incompatibility_blocks_seed_and_smoke() {
        for command in [DevCommand::Seed, DevCommand::Smoke] {
            let mut options = test_options();
            options.profile = "local-stale-local-state-schema".to_owned();
            let output = LocalStackRunner::new(options).run(command);
            assert!(!output.is_ok());
            assert_eq!(
                output.reason_code,
                "local_stack.schema_version_incompatible"
            );
            assert_eq!(output.exit_class, ExitCodeClass::Schema);
            assert!(output.lifecycle_strs().contains(&"blocked"));
            assert!(output
                .schema_compatibility_gates
                .iter()
                .any(|gate| gate.surface == "local_state" && !gate.compatible));
            assert!(output
                .error_json()
                .contains("\"schema_compatibility_gates\""));
        }
    }

    #[test]
    fn occupied_reserved_port_blocks_before_starting() {
        let _listener = match TcpListener::bind(("127.0.0.1", 18080)) {
            Ok(listener) => Some(listener),
            Err(_) => None,
        };
        let options = LocalStackOptions::new(repo_root());
        let output = LocalStackRunner::new(options).run(DevCommand::Start);
        assert!(!output.is_ok());
        assert_eq!(output.reason_code, "local_stack.port_conflict");
        assert_eq!(output.exit_class, ExitCodeClass::Config);
        assert!(output.lifecycle_strs().contains(&"prerequisites_checked"));
        assert!(!output.lifecycle_strs().contains(&"starting"));
        assert!(output
            .port_conflicts
            .iter()
            .any(|conflict| conflict.port == 18080
                && conflict.bind_host == "127.0.0.1"
                && conflict.reason_code == "local_stack.port_conflict"));
        assert!(output
            .error_json()
            .contains("\"port_conflicts\":[{\"service_id\":\"service:api\""));
        assert!(output
            .doctor_checks
            .iter()
            .any(|check| check.check_id == "doctor:reserved_ports" && check.state == "failed"));
    }

    #[test]
    fn doctor_failure_reason_codes_are_stable() {
        let mut options = test_options();
        options.profile = "local-unsafe-env".to_owned();
        let output = LocalStackRunner::new(options).run(DevCommand::Doctor);
        assert!(!output.is_ok());
        assert_eq!(output.reason_code, "doctor.unsafe_env_value");
        assert!(output
            .error_json()
            .contains("\"topology_phase_gate\":\"phase_4_loopback_topology\""));
        assert!(
            output
                .doctor_checks
                .iter()
                .any(|check| check.reason_code == "doctor.unsafe_env_value"
                    && check.state == "failed")
        );
    }

    #[test]
    fn rejects_non_local_profile() {
        let mut options = test_options();
        options.profile = "seed".to_owned();
        let output = LocalStackRunner::new(options).run(DevCommand::Start);
        assert!(!output.is_ok());
        assert_eq!(output.exit_class, ExitCodeClass::Config);
        assert_eq!(output.reason_code, "profile.not_local_test");
    }

    #[test]
    fn backing_service_failure_is_redacted() {
        let mut options = test_options();
        options.profile = "local-health-timeout".to_owned();
        let output = LocalStackRunner::new(options).run(DevCommand::Start);
        assert!(!output.is_ok());
        assert_eq!(
            output.reason_code,
            "local_stack.backing_services_unavailable"
        );
        assert!(output
            .diagnostic_refs
            .iter()
            .all(|reference| !reference.contains("secret") && !reference.contains("token")));
    }

    #[test]
    fn impossible_transition_is_rejected() {
        let mut record = LocalCommandRecord::new();
        record
            .push(LocalCommandState::PrerequisitesChecked)
            .unwrap();
        record.push(LocalCommandState::Ready).unwrap();
        record.push(LocalCommandState::Completed).unwrap();
        let error = record.push(LocalCommandState::Starting).unwrap_err();
        assert_eq!(error.previous, LocalCommandState::Completed);
        assert_eq!(error.next, LocalCommandState::Starting);
    }
}
