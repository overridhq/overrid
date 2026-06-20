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
pub const LOCAL_STACK_PHASE6_LIFECYCLE_GATE: &str = "phase_6_lifecycle_orchestration";
pub const LOCAL_STACK_PHASE7_FIXTURE_GATE: &str = "phase_7_reset_seed_fixtures";
pub const LOCAL_STACK_PHASE8_SMOKE_GATE: &str = "phase_8_node_simulator_smoke_harness";
pub const LOCAL_STACK_PHASE9_DIAGNOSTICS_GATE: &str = "phase_9_diagnostics_artifacts_ci_flake";
pub const LOCAL_STACK_ENV_EXAMPLE_PATH: &str = ".env.example";
pub const DEFAULT_LIFECYCLE_TIMEOUT_MS: u64 = 60_000;
pub const DEFAULT_LIFECYCLE_POLL_INTERVAL_MS: u64 = 250;
pub const LOCAL_STACK_PHASE7_FIXTURE_VERSION: &str = "fixture:phase7_control_plane_seed.v1";
pub const LOCAL_STACK_PHASE7_DETERMINISTIC_SEED: &str = "seed:local_stack:phase7:0001";
pub const LOCAL_STACK_PHASE8_SMOKE_FIXTURE_VERSION: &str = "fixture:phase8_node_smoke.v1";
pub const LOCAL_STACK_PHASE9_CI_RUNNER_REF: &str =
    "ci://local_stack/linux_x86_64/ubuntu_24_04_equivalent";

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
            let schema_report = SchemaCompatibilityReport::for_profile(&self.options.profile);
            if !schema_report.is_compatible() {
                output.apply_schema_compatibility_gates(schema_report.into_gates());
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
        let mode = lifecycle_mode(&self.options.profile);
        output.apply_lifecycle_mode(mode, DevCommand::Start);
        output
            .push_state(LocalCommandState::Starting)
            .expect("local-stack start transition is valid");
        if matches!(
            mode,
            LifecycleMode::HealthTimeout | LifecycleMode::RequiredFailure
        ) {
            return output.fail_lifecycle(
                "local_stack.backing_services_unavailable",
                "local stack required service readiness failed before startup completed",
                ExitCodeClass::Platform,
                RetryClass::SafeRetry,
            );
        }
        output
            .push_state(LocalCommandState::Ready)
            .expect("local-stack ready transition is valid");
        if mode == LifecycleMode::OptionalDegraded {
            return output.complete(
                LocalStackStatus::Degraded,
                "local_stack.degraded",
                "local stack required services are ready with an optional service degraded",
            );
        }
        if mode == LifecycleMode::AlreadyRunning {
            return output.complete(
                LocalStackStatus::Ready,
                "local_stack.already_running",
                "local stack services were already running and readiness was verified",
            );
        }
        output.complete(
            LocalStackStatus::Ready,
            "local_stack.ready",
            "local stack services are ready",
        )
    }

    fn stop(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        output.apply_shutdown_reports(DevCommand::Stop);
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
        let mode = lifecycle_mode(&self.options.profile);
        output.apply_lifecycle_mode(mode, DevCommand::Restart);
        output.apply_shutdown_reports(DevCommand::Restart);
        output
            .push_state(LocalCommandState::Stopped)
            .expect("local-stack restart stop transition is valid");
        output
            .push_state(LocalCommandState::Starting)
            .expect("local-stack restart start transition is valid");
        if matches!(
            mode,
            LifecycleMode::HealthTimeout | LifecycleMode::RequiredFailure
        ) {
            return output.fail_lifecycle(
                "local_stack.backing_services_unavailable",
                "local stack restart failed while waiting for required service readiness",
                ExitCodeClass::Platform,
                RetryClass::SafeRetry,
            );
        }
        output
            .push_state(LocalCommandState::Ready)
            .expect("local-stack restart ready transition is valid");
        if mode == LifecycleMode::OptionalDegraded {
            return output.complete(
                LocalStackStatus::Degraded,
                "local_stack.degraded",
                "local stack restarted with required services ready and an optional service degraded",
            );
        }
        output.complete(
            LocalStackStatus::Ready,
            "local_stack.ready",
            "local stack services restarted and are ready",
        )
    }

    fn status(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        output.apply_lifecycle_mode(lifecycle_mode(&self.options.profile), DevCommand::Status);
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
        output.lifecycle_events.push(lifecycle_event(
            &self.options.profile,
            &self.options.trace_id,
            "local_stack.reset_started",
            "stack",
            "reset",
        ));
        output
            .push_state(LocalCommandState::Resetting)
            .expect("local-stack reset transition is valid");
        if output
            .reset_safety_checks
            .iter()
            .any(|check| !check.safe_to_delete)
        {
            output.diagnostic_refs.push(format!(
                "diagnostic://local_stack/reset_safety/{}",
                id_component(&self.options.trace_id)
            ));
            output.block(LocalStackFailure {
                reason_code: "local_stack.reset_unsafe_state",
                message: "local stack reset aborted because test-state markers are missing, stale, or inconsistent",
                exit_class: ExitCodeClass::Config,
                retry_class: RetryClass::OperatorReview,
                status: LocalStackStatus::Blocked,
            });
            return output;
        }
        output
            .push_state(LocalCommandState::CollectingArtifacts)
            .expect("local-stack reset artifact transition is valid");
        output.artifact_refs.push(format!(
            "artifact://local_stack/reset/{}",
            id_component(&self.options.trace_id)
        ));
        output.lifecycle_events.push(lifecycle_event(
            &self.options.profile,
            &self.options.trace_id,
            "local_stack.reset_completed",
            "stack",
            "reset",
        ));
        output.complete(
            LocalStackStatus::Completed,
            "local_stack.reset_completed",
            "local stack reset completed",
        )
    }

    fn seed(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        output.lifecycle_events.push(lifecycle_event(
            &self.options.profile,
            &self.options.trace_id,
            "local_stack.seed_started",
            "stack",
            "seed",
        ));
        output
            .push_state(LocalCommandState::Seeding)
            .expect("local-stack seed transition is valid");
        if output
            .fixture_drift_reports
            .iter()
            .any(|report| report.drift_detected && report.blocks_seed)
        {
            output.diagnostic_refs.push(format!(
                "diagnostic://local_stack/fixture_drift/{}",
                id_component(&self.options.trace_id)
            ));
            output.block(LocalStackFailure {
                reason_code: "local_stack.fixture_drift_detected",
                message:
                    "local stack seed aborted because deterministic fixture drift was detected",
                exit_class: ExitCodeClass::Schema,
                retry_class: RetryClass::OperatorReview,
                status: LocalStackStatus::Blocked,
            });
            return output;
        }
        output.artifact_refs.push(format!(
            "artifact://local_stack/seed/{}",
            id_component(&self.options.trace_id)
        ));
        output.lifecycle_events.push(lifecycle_event(
            &self.options.profile,
            &self.options.trace_id,
            "local_stack.seed_completed",
            "stack",
            "seed",
        ));
        output.complete(
            LocalStackStatus::Completed,
            "local_stack.seed_completed",
            "local stack seed completed",
        )
    }

    fn smoke(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        let mode = lifecycle_mode(&self.options.profile);
        output.apply_lifecycle_mode(mode, DevCommand::Smoke);
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
            if state == LocalCommandState::Starting
                && matches!(
                    mode,
                    LifecycleMode::HealthTimeout | LifecycleMode::RequiredFailure
                )
            {
                return output.fail_lifecycle(
                    "local_stack.backing_services_unavailable",
                    "local stack smoke failed while waiting for required service readiness",
                    ExitCodeClass::Platform,
                    RetryClass::SafeRetry,
                );
            }
        }
        if output
            .fixture_drift_reports
            .iter()
            .any(|report| report.drift_detected && report.blocks_seed)
        {
            output.diagnostic_refs.push(format!(
                "diagnostic://local_stack/fixture_drift/{}",
                id_component(&self.options.trace_id)
            ));
            output.block(LocalStackFailure {
                reason_code: "local_stack.fixture_drift_detected",
                message:
                    "local stack smoke aborted because deterministic fixture drift was detected",
                exit_class: ExitCodeClass::Schema,
                retry_class: RetryClass::OperatorReview,
                status: LocalStackStatus::Blocked,
            });
            return output;
        }
        for (event_type, discriminator) in [
            ("local_stack.reset_started", "smoke_reset"),
            ("local_stack.seed_started", "smoke_seed"),
            ("local_stack.seed_completed", "smoke_seed"),
            ("local_stack.smoke_started", "smoke"),
            ("local_stack.smoke_completed", "smoke"),
        ] {
            output.lifecycle_events.push(lifecycle_event(
                &self.options.profile,
                &self.options.trace_id,
                event_type,
                "stack",
                discriminator,
            ));
        }
        output.artifact_refs.push(format!(
            "artifact://local_stack/smoke/{}",
            id_component(&self.options.trace_id)
        ));
        output.artifact_refs.push(format!(
            "artifact://local_stack/smoke/{}/redacted_bundle",
            id_component(&self.options.trace_id)
        ));
        if let Some(bundle) = output.diagnostic_artifact_bundles.first() {
            output.artifact_refs.push(bundle.bundle_ref.clone());
        }
        output.complete(
            LocalStackStatus::Completed,
            "local_stack.smoke_passed",
            "local stack smoke checks passed",
        )
    }

    fn logs(&self, mut output: LocalStackCommandOutput) -> LocalStackCommandOutput {
        output.lifecycle_events.push(lifecycle_event(
            &self.options.profile,
            &self.options.trace_id,
            "local_stack.logs_requested",
            "stack",
            "logs",
        ));
        output
            .push_state(LocalCommandState::CollectingArtifacts)
            .expect("local-stack logs transition is valid");
        output.artifact_refs.push(format!(
            "log://local_stack/{}",
            id_component(&self.options.trace_id)
        ));
        output.artifact_refs.extend(
            output
                .redacted_log_exports
                .iter()
                .map(|record| record.log_ref.clone()),
        );
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
        output.lifecycle_events.push(lifecycle_event(
            &self.options.profile,
            &self.options.trace_id,
            "local_stack.doctor_requested",
            "stack",
            "doctor",
        ));
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
    Degraded,
    Stopped,
    Blocked,
    Failed,
    Completed,
}

impl LocalStackStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Degraded => "degraded",
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
pub struct LocalServiceLifecycleStep {
    pub service_id: String,
    pub dependency_order: u8,
    pub required: bool,
    pub dependencies: Vec<String>,
    pub startup_state: String,
    pub health_state: String,
    pub readiness_state: String,
    pub liveness_state: String,
    pub start_after_ms: u64,
    pub ready_after_ms: u64,
    pub timeout_ms: u64,
    pub reason_code: String,
    pub rollback_on_failure: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalServiceShutdownReport {
    pub service_id: String,
    pub shutdown_order: u8,
    pub graceful: bool,
    pub state_preserved: bool,
    pub reason_code: String,
    pub diagnostic_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalReadinessCheck {
    pub service_id: String,
    pub endpoint_ref: String,
    pub health_state: String,
    pub readiness_state: String,
    pub liveness_state: String,
    pub dependency_state: String,
    pub elapsed_ms: u64,
    pub timeout_ms: u64,
    pub reason_code: String,
    pub stale_schema: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalWaitPolicy {
    pub timeout_ms: u64,
    pub poll_interval_ms: u64,
    pub bounded: bool,
    pub no_unbounded_sleep: bool,
    pub timeout_class: String,
    pub reason_code: String,
    pub dependency_wait_diagnostics_ref: String,
    pub logs_exported_on_timeout: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalRollbackReport {
    pub service_id: String,
    pub rollback_order: u8,
    pub action: String,
    pub state_preserved: bool,
    pub reason_code: String,
    pub artifact_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalLifecycleEventRecord {
    pub event_id: String,
    pub event_type: String,
    pub service_id: String,
    pub trace_id: String,
    pub stack_profile: String,
    pub fixture_version: String,
    pub schema_version: String,
    pub artifact_ref: String,
    pub redaction_summary: String,
    pub contains_raw_secret: bool,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalResetSafetyCheck {
    pub check_id: String,
    pub target_ref: String,
    pub marker_ref: String,
    pub marker_state: String,
    pub planned_action: String,
    pub deletion_plan_ref: String,
    pub safe_to_delete: bool,
    pub reason_code: String,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalSeedFixtureRecord {
    pub fixture_id: &'static str,
    pub fixture_kind: &'static str,
    pub fixture_ref: &'static str,
    pub stable_id: &'static str,
    pub apply_order: u8,
    pub fixture_version: &'static str,
    pub deterministic_seed: &'static str,
    pub profile_binding: &'static str,
    pub schema_version: &'static str,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalFixtureIsolationCheck {
    pub check_id: String,
    pub fixture_ref: String,
    pub credential_ref: String,
    pub profile_class: String,
    pub bound_profile: String,
    pub accepted: bool,
    pub reason_code: String,
    pub contains_raw_secret: bool,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalControlPlaneSeedPrerequisite {
    pub prerequisite_id: &'static str,
    pub prerequisite_kind: &'static str,
    pub fixture_ref: &'static str,
    pub phase_gate: &'static str,
    pub stable_ref: &'static str,
    pub signing_required: bool,
    pub id_preserved_for_phase1: bool,
    pub bypasses_signing: bool,
    pub reason_code: &'static str,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalFixtureDriftReport {
    pub report_id: String,
    pub fixture_ref: String,
    pub diff_field: String,
    pub expected_ref: String,
    pub actual_ref: String,
    pub drift_detected: bool,
    pub reason_code: String,
    pub blocks_seed: bool,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalNodeSimulatorRecord {
    pub simulator_id: String,
    pub service_id: String,
    pub deterministic_identity_ref: String,
    pub heartbeat_ref: String,
    pub capability_report_ref: String,
    pub health_endpoint_ref: String,
    pub accepted_fixture_workload_ref: String,
    pub noop_handoff_ref: String,
    pub real_hardware_discovery: bool,
    pub gpu_runtime_integration: bool,
    pub benchmark_publication: bool,
    pub installer_update_flow: bool,
    pub remote_shell_behavior: bool,
    pub provider_eligibility_decision: bool,
    pub reason_code: String,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalPhase0SmokeRecord {
    pub smoke_id: String,
    pub signed_noop_command_ref: String,
    pub audit_event_ref: String,
    pub invalid_schema_ref: String,
    pub trace_id: String,
    pub fixture_state_ref: String,
    pub redacted_artifact_ref: String,
    pub signed_noop_admitted: bool,
    pub audit_event_write_read: bool,
    pub invalid_schema_denied: bool,
    pub trace_id_propagated: bool,
    pub fixture_state_inspected: bool,
    pub public_local_api_only: bool,
    pub generated_contracts_only: bool,
    pub contains_raw_secret: bool,
    pub reason_code: String,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalHarnessHookRecord {
    pub hook_id: String,
    pub command_surface: String,
    pub harness_method: String,
    pub evidence_ref: String,
    pub generated_contract_ref: String,
    pub required_for_phase0_smoke: bool,
    pub public_local_api_only: bool,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSdkSmokeSupportRecord {
    pub support_id: String,
    pub client_surface: String,
    pub request_example_ref: String,
    pub generated_contract_ref: String,
    pub public_local_api_only: bool,
    pub avoids_private_storage: bool,
    pub avoids_simulator_internals: bool,
    pub reason_code: String,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSimulatorExpansionRule {
    pub rule_id: String,
    pub target_phase: String,
    pub owning_contract_ref: String,
    pub local_test_marker_required: bool,
    pub production_contract_shape_required: bool,
    pub phase0_responsibility_drift_allowed: bool,
    pub blocks_without_owner: bool,
    pub reason_code: String,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalRedactedLogExportRecord {
    pub stream_id: String,
    pub service_id: String,
    pub log_ref: String,
    pub bundle_section_ref: String,
    pub redacts_secrets: bool,
    pub redacts_tokens: bool,
    pub redacts_signatures: bool,
    pub redacts_private_payloads: bool,
    pub redacts_encrypted_content: bool,
    pub export_blocked_until_secret_free: bool,
    pub scanner_passed: bool,
    pub reason_code: String,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDiagnosticArtifactBundleRecord {
    pub bundle_id: String,
    pub bundle_ref: String,
    pub manifest_ref: String,
    pub health_snapshot_refs: Vec<String>,
    pub stack_profile: String,
    pub schema_version: String,
    pub fixture_version: String,
    pub command_lifecycle_ref: String,
    pub trace_id: String,
    pub reason_code_ref: String,
    pub local_event_refs: Vec<String>,
    pub queue_state_refs: Vec<String>,
    pub object_refs: Vec<String>,
    pub reproduction_command: String,
    pub retention_class: String,
    pub contains_raw_secret: bool,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCleanCheckoutCiEntrypointRecord {
    pub entrypoint_id: String,
    pub runner_ref: String,
    pub os_family: String,
    pub arch: String,
    pub ubuntu_24_04_equivalent: bool,
    pub repo_pinned_rust_toolchain: bool,
    pub loopback_networking: bool,
    pub cloud_credentials_allowed: bool,
    pub external_database_allowed: bool,
    pub external_queue_allowed: bool,
    pub external_object_store_allowed: bool,
    pub commands: Vec<String>,
    pub allowed_outcomes: Vec<String>,
    pub machine_readable_output: bool,
    pub reason_code: String,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalFlakeEvidenceRecord {
    pub evidence_id: String,
    pub trace_id: String,
    pub repeated_run_count: u16,
    pub startup_timing_variance_ms: u64,
    pub nondeterministic_fixture_ids: bool,
    pub unstable_event_ordering: bool,
    pub retry_count: u16,
    pub health_timeout_class: String,
    pub tolerance_window_used: bool,
    pub flake_detected: bool,
    pub reason_code: String,
    pub artifact_ref: String,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalArtifactRetentionPolicyRecord {
    pub policy_id: String,
    pub retention_class: String,
    pub compact_success_summary: bool,
    pub retain_failure_bundle: bool,
    pub failure_retention_days: u16,
    pub prune_command_ref: String,
    pub requires_test_state_marker: bool,
    pub deletes_unmarked_user_dirs: bool,
    pub deletes_production_like_state: bool,
    pub deletes_non_local_artifacts: bool,
    pub reason_code: String,
    pub local_only: bool,
    pub test_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LifecycleMode {
    CleanStart,
    AlreadyRunning,
    HealthTimeout,
    RequiredFailure,
    OptionalDegraded,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStateStore {
    records: Vec<LocalStateRecord>,
}

impl LocalStateStore {
    pub fn new(records: Vec<LocalStateRecord>) -> Self {
        Self { records }
    }

    pub fn default_local() -> Self {
        Self::new(local_state_records())
    }

    pub fn records(&self) -> &[LocalStateRecord] {
        &self.records
    }

    pub fn get(&self, record_id: &str) -> Option<&LocalStateRecord> {
        self.records
            .iter()
            .find(|record| record.record_id == record_id)
    }

    pub fn by_kind(&self, record_kind: &str) -> Vec<&LocalStateRecord> {
        self.records
            .iter()
            .filter(|record| record.record_kind == record_kind)
            .collect()
    }

    pub fn contract_ref(&self, record_id: &str) -> Option<&'static str> {
        self.get(record_id).map(|record| record.contract_ref)
    }

    pub fn schema_versions_compatible(&self) -> bool {
        self.records
            .iter()
            .all(|record| record.schema_version == SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION)
    }

    pub fn all_overbase_shaped(&self) -> bool {
        self.records.iter().all(|record| {
            record.local_only
                && record.test_only
                && !record.external_database_semantics
                && record.storage_boundary == "overbase_shaped_embedded_state"
                && (record.contract_ref.starts_with("overbase://")
                    || record.contract_ref.starts_with("local-state://"))
        })
    }
}

impl Default for LocalStateStore {
    fn default() -> Self {
        Self::default_local()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalQueueIdempotencyOutcome {
    pub idempotency_key: String,
    pub accepted: bool,
    pub existing_job_id: Option<&'static str>,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalQueueTable {
    jobs: Vec<LocalQueueJobRecord>,
}

impl LocalQueueTable {
    pub fn new(jobs: Vec<LocalQueueJobRecord>) -> Self {
        Self { jobs }
    }

    pub fn default_local() -> Self {
        Self::new(local_queue_job_records())
    }

    pub fn jobs(&self) -> &[LocalQueueJobRecord] {
        &self.jobs
    }

    pub fn submit_preview(&self, idempotency_key: &str) -> LocalQueueIdempotencyOutcome {
        if let Some(existing) = self
            .jobs
            .iter()
            .find(|job| job.idempotency_key == idempotency_key)
        {
            return LocalQueueIdempotencyOutcome {
                idempotency_key: idempotency_key.to_owned(),
                accepted: false,
                existing_job_id: Some(existing.job_id),
                reason_code: "local_stack.job_duplicate_idempotency_key",
            };
        }

        LocalQueueIdempotencyOutcome {
            idempotency_key: idempotency_key.to_owned(),
            accepted: true,
            existing_job_id: None,
            reason_code: "local_stack.job_pending",
        }
    }

    pub fn jobs_by_state(&self, state: &str) -> Vec<&LocalQueueJobRecord> {
        self.jobs.iter().filter(|job| job.state == state).collect()
    }

    pub fn pending_jobs(&self) -> Vec<&LocalQueueJobRecord> {
        self.jobs_by_state("pending")
    }

    pub fn retry_jobs(&self) -> Vec<&LocalQueueJobRecord> {
        self.jobs_by_state("retry_scheduled")
    }

    pub fn dead_letter_jobs(&self) -> Vec<&LocalQueueJobRecord> {
        self.jobs_by_state("dead_letter")
    }

    pub fn terminal_jobs(&self) -> Vec<&LocalQueueJobRecord> {
        self.jobs.iter().filter(|job| job.terminal).collect()
    }
}

impl Default for LocalQueueTable {
    fn default() -> Self {
        Self::default_local()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalArtifactStore {
    manifests: Vec<LocalArtifactManifest>,
}

impl LocalArtifactStore {
    pub fn new(manifests: Vec<LocalArtifactManifest>) -> Self {
        Self { manifests }
    }

    pub fn default_local() -> Self {
        Self::new(local_artifact_manifests())
    }

    pub fn manifests(&self) -> &[LocalArtifactManifest] {
        &self.manifests
    }

    pub fn manifest_for_ref(&self, artifact_ref: &str) -> Option<&LocalArtifactManifest> {
        self.manifests
            .iter()
            .find(|manifest| manifest.artifact_ref == artifact_ref)
    }

    pub fn verify_payload(&self, artifact_ref: &str, payload: &str) -> bool {
        let Some(manifest) = self.manifest_for_ref(artifact_ref) else {
            return false;
        };
        let content_hash = blake3_hex(payload.as_bytes());
        manifest.hash_algorithm == "BLAKE3"
            && manifest.content_hash == content_hash
            && manifest.content_address == Self::content_address_for_payload(payload)
            && manifest.byte_length == payload.len()
            && manifest.filesystem_backed
            && !manifest.external_object_store_boundary
            && manifest.local_only
            && manifest.test_only
    }

    pub fn resettable_manifests(&self) -> Vec<&LocalArtifactManifest> {
        self.manifests
            .iter()
            .filter(|manifest| manifest.reset_safe)
            .collect()
    }

    pub fn content_address_for_payload(payload: &str) -> String {
        let content_hash = blake3_hex(payload.as_bytes());
        format!("overstore://local_stub/blake3/{content_hash}")
    }
}

impl Default for LocalArtifactStore {
    fn default() -> Self {
        Self::default_local()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LocalAuditQueryFilter<'a> {
    pub trace_id: Option<&'a str>,
    pub service_id: Option<&'a str>,
    pub time_window_ref: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalAuditEventStore {
    records: Vec<LocalAuditQueryRecord>,
}

impl LocalAuditEventStore {
    pub fn new(records: Vec<LocalAuditQueryRecord>) -> Self {
        Self { records }
    }

    pub fn default_local() -> Self {
        Self::new(local_audit_query_records())
    }

    pub fn records(&self) -> &[LocalAuditQueryRecord] {
        &self.records
    }

    pub fn query(&self, filter: LocalAuditQueryFilter<'_>) -> Vec<&LocalAuditQueryRecord> {
        self.records
            .iter()
            .filter(|record| {
                filter
                    .trace_id
                    .map(|trace_id| record.trace_id == trace_id)
                    .unwrap_or(true)
                    && filter
                        .service_id
                        .map(|service_id| record.service_id == service_id)
                        .unwrap_or(true)
                    && filter
                        .time_window_ref
                        .map(|time_window_ref| record.time_window_ref == time_window_ref)
                        .unwrap_or(true)
            })
            .collect()
    }

    pub fn all_local_diagnostics_only(&self) -> bool {
        self.records.iter().all(|record| {
            record.local_diagnostic_event
                && !record.production_overwatch_authority
                && !record.contains_raw_secret
                && record.redaction_summary == "secret_free"
                && record.local_only
                && record.test_only
        })
    }
}

impl Default for LocalAuditEventStore {
    fn default() -> Self {
        Self::default_local()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaCompatibilityReport {
    gates: Vec<SchemaCompatibilityGate>,
}

impl SchemaCompatibilityReport {
    pub fn for_profile(profile: &str) -> Self {
        Self {
            gates: schema_compatibility_gates_for_profile(profile),
        }
    }

    pub fn gates(&self) -> &[SchemaCompatibilityGate] {
        &self.gates
    }

    pub fn into_gates(self) -> Vec<SchemaCompatibilityGate> {
        self.gates
    }

    pub fn is_compatible(&self) -> bool {
        self.gates.iter().all(|gate| gate.compatible)
    }

    pub fn blocking_surfaces_for_command(&self, command: DevCommand) -> Vec<&'static str> {
        self.gates
            .iter()
            .filter(|gate| {
                !gate.compatible
                    && match command {
                        DevCommand::Start => gate.blocks_start,
                        DevCommand::Seed => gate.blocks_seed,
                        DevCommand::Smoke => gate.blocks_smoke,
                        _ => false,
                    }
            })
            .map(|gate| gate.surface)
            .collect()
    }
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
    pub startup_graph: Vec<LocalServiceLifecycleStep>,
    pub shutdown_reports: Vec<LocalServiceShutdownReport>,
    pub readiness_checks: Vec<LocalReadinessCheck>,
    pub wait_policy: LocalWaitPolicy,
    pub rollback_reports: Vec<LocalRollbackReport>,
    pub lifecycle_events: Vec<LocalLifecycleEventRecord>,
    pub reset_safety_checks: Vec<LocalResetSafetyCheck>,
    pub seed_fixture_records: Vec<LocalSeedFixtureRecord>,
    pub fixture_isolation_checks: Vec<LocalFixtureIsolationCheck>,
    pub phase1_seed_prerequisites: Vec<LocalControlPlaneSeedPrerequisite>,
    pub fixture_drift_reports: Vec<LocalFixtureDriftReport>,
    pub node_simulator_records: Vec<LocalNodeSimulatorRecord>,
    pub phase0_smoke_records: Vec<LocalPhase0SmokeRecord>,
    pub harness_hook_records: Vec<LocalHarnessHookRecord>,
    pub cli_sdk_smoke_support_records: Vec<LocalSdkSmokeSupportRecord>,
    pub simulator_expansion_rules: Vec<LocalSimulatorExpansionRule>,
    pub redacted_log_exports: Vec<LocalRedactedLogExportRecord>,
    pub diagnostic_artifact_bundles: Vec<LocalDiagnosticArtifactBundleRecord>,
    pub clean_checkout_ci_entries: Vec<LocalCleanCheckoutCiEntrypointRecord>,
    pub flake_evidence_records: Vec<LocalFlakeEvidenceRecord>,
    pub artifact_retention_policies: Vec<LocalArtifactRetentionPolicyRecord>,
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
            local_state_records: LocalStateStore::default_local().records().to_vec(),
            queue_job_records: LocalQueueTable::default_local().jobs().to_vec(),
            artifact_manifests: LocalArtifactStore::default_local().manifests().to_vec(),
            audit_query_records: LocalAuditEventStore::default_local().records().to_vec(),
            schema_compatibility_gates: SchemaCompatibilityReport::for_profile(&options.profile)
                .gates()
                .to_vec(),
            startup_graph: lifecycle_startup_steps(
                &options.profile,
                &options.trace_id,
                options.timeout_ms,
            ),
            shutdown_reports: Vec::new(),
            readiness_checks: readiness_checks_for_profile(
                &options.profile,
                &options.trace_id,
                options.timeout_ms,
            ),
            wait_policy: wait_policy_for_options(
                &options.profile,
                options.timeout_ms,
                options.poll_interval_ms,
            ),
            rollback_reports: Vec::new(),
            lifecycle_events: vec![lifecycle_event(
                &options.profile,
                &options.trace_id,
                "local_stack.command_requested",
                "stack",
                command.action(),
            )],
            reset_safety_checks: reset_safety_checks_for_profile(&options.profile, options.dry_run),
            seed_fixture_records: seed_fixture_records(),
            fixture_isolation_checks: fixture_isolation_checks(&options.profile),
            phase1_seed_prerequisites: phase1_seed_prerequisites(),
            fixture_drift_reports: fixture_drift_reports_for_profile(&options.profile),
            node_simulator_records: node_simulator_records(),
            phase0_smoke_records: phase0_smoke_records(&options.trace_id),
            harness_hook_records: harness_hook_records(),
            cli_sdk_smoke_support_records: cli_sdk_smoke_support_records(),
            simulator_expansion_rules: simulator_expansion_rules(),
            redacted_log_exports: redacted_log_export_records(&options.trace_id),
            diagnostic_artifact_bundles: diagnostic_artifact_bundle_records(
                &options.profile,
                &options.trace_id,
            ),
            clean_checkout_ci_entries: clean_checkout_ci_entries(),
            flake_evidence_records: flake_evidence_records_for_profile(
                &options.profile,
                &options.trace_id,
            ),
            artifact_retention_policies: artifact_retention_policies(),
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
                "dependency_ordered_startup_verified",
                "reverse_shutdown_order_verified",
                "readiness_liveness_checks_ready",
                "bounded_wait_policy_ready",
                "lifecycle_events_secret_free",
                "reset_safety_checked",
                "deterministic_seed_verified",
                "fixture_isolation_verified",
                "phase1_seed_prerequisites_ready",
                "fixture_drift_report_clean",
                "node_simulator_local_only_ready",
                "phase0_smoke_path_verified",
                "integration_harness_hooks_ready",
                "cli_sdk_smoke_support_public_api_only",
                "simulator_expansion_rules_enforced",
                "redacted_log_exports_ready",
                "diagnostic_artifact_bundle_ready",
                "clean_checkout_ci_entrypoint_ready",
                "flake_metadata_recorded",
                "artifact_retention_policy_enforced",
                "redacted_diagnostics_only",
            ]
        } else {
            vec![
                "local_stack_manifest_checked",
                "schema_compatibility_checked",
                "lifecycle_orchestration_attempted",
                "bounded_wait_policy_applied",
                "redacted_lifecycle_events_collected",
                "reset_safety_checked",
                "fixture_drift_report_collected",
                "phase0_smoke_path_collected",
                "simulator_expansion_rules_enforced",
                "redacted_log_exports_collected",
                "diagnostic_artifact_bundle_collected",
                "flake_metadata_recorded",
                "artifact_retention_policy_enforced",
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
                "\"lifecycle_phase_gate\":\"{}\",",
                "\"fixture_phase_gate\":\"{}\",",
                "\"smoke_phase_gate\":\"{}\",",
                "\"diagnostics_phase_gate\":\"{}\",",
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
                "\"startup_graph\":{},",
                "\"shutdown_reports\":{},",
                "\"readiness_checks\":{},",
                "\"wait_policy\":{},",
                "\"rollback_reports\":{},",
                "\"lifecycle_events\":{},",
                "\"reset_safety_checks\":{},",
                "\"seed_fixture_records\":{},",
                "\"fixture_isolation_checks\":{},",
                "\"phase1_seed_prerequisites\":{},",
                "\"fixture_drift_reports\":{},",
                "\"node_simulator_records\":{},",
                "\"phase0_smoke_records\":{},",
                "\"harness_hook_records\":{},",
                "\"cli_sdk_smoke_support_records\":{},",
                "\"simulator_expansion_rules\":{},",
                "\"redacted_log_exports\":{},",
                "\"diagnostic_artifact_bundles\":{},",
                "\"clean_checkout_ci_entries\":{},",
                "\"flake_evidence_records\":{},",
                "\"artifact_retention_policies\":{},",
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
            json_escape(LOCAL_STACK_PHASE6_LIFECYCLE_GATE),
            json_escape(LOCAL_STACK_PHASE7_FIXTURE_GATE),
            json_escape(LOCAL_STACK_PHASE8_SMOKE_GATE),
            json_escape(LOCAL_STACK_PHASE9_DIAGNOSTICS_GATE),
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
            render_startup_graph_json(&self.startup_graph),
            render_shutdown_reports_json(&self.shutdown_reports),
            render_readiness_checks_json(&self.readiness_checks),
            render_wait_policy_json(&self.wait_policy),
            render_rollback_reports_json(&self.rollback_reports),
            render_lifecycle_events_json(&self.lifecycle_events),
            render_reset_safety_checks_json(&self.reset_safety_checks),
            render_seed_fixture_records_json(&self.seed_fixture_records),
            render_fixture_isolation_checks_json(&self.fixture_isolation_checks),
            render_phase1_seed_prerequisites_json(&self.phase1_seed_prerequisites),
            render_fixture_drift_reports_json(&self.fixture_drift_reports),
            render_node_simulator_records_json(&self.node_simulator_records),
            render_phase0_smoke_records_json(&self.phase0_smoke_records),
            render_harness_hook_records_json(&self.harness_hook_records),
            render_cli_sdk_smoke_support_records_json(&self.cli_sdk_smoke_support_records),
            render_simulator_expansion_rules_json(&self.simulator_expansion_rules),
            render_redacted_log_exports_json(&self.redacted_log_exports),
            render_diagnostic_artifact_bundles_json(&self.diagnostic_artifact_bundles),
            render_clean_checkout_ci_entries_json(&self.clean_checkout_ci_entries),
            render_flake_evidence_records_json(&self.flake_evidence_records),
            render_artifact_retention_policies_json(&self.artifact_retention_policies),
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
                "\"lifecycle_phase_gate\":\"{}\",",
                "\"fixture_phase_gate\":\"{}\",",
                "\"smoke_phase_gate\":\"{}\",",
                "\"diagnostics_phase_gate\":\"{}\",",
                "\"port_registry\":{},",
                "\"port_conflicts\":{},",
                "\"schema_compatibility_gates\":{},",
                "\"startup_graph\":{},",
                "\"readiness_checks\":{},",
                "\"wait_policy\":{},",
                "\"rollback_reports\":{},",
                "\"lifecycle_events\":{},",
                "\"reset_safety_checks\":{},",
                "\"fixture_isolation_checks\":{},",
                "\"fixture_drift_reports\":{},",
                "\"node_simulator_records\":{},",
                "\"phase0_smoke_records\":{},",
                "\"simulator_expansion_rules\":{},",
                "\"redacted_log_exports\":{},",
                "\"diagnostic_artifact_bundles\":{},",
                "\"clean_checkout_ci_entries\":{},",
                "\"flake_evidence_records\":{},",
                "\"artifact_retention_policies\":{},",
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
            json_escape(LOCAL_STACK_PHASE6_LIFECYCLE_GATE),
            json_escape(LOCAL_STACK_PHASE7_FIXTURE_GATE),
            json_escape(LOCAL_STACK_PHASE8_SMOKE_GATE),
            json_escape(LOCAL_STACK_PHASE9_DIAGNOSTICS_GATE),
            render_port_registry_json(&self.port_bindings),
            render_port_conflicts_json(&self.port_conflicts),
            render_schema_compatibility_gates_json(&self.schema_compatibility_gates),
            render_startup_graph_json(&self.startup_graph),
            render_readiness_checks_json(&self.readiness_checks),
            render_wait_policy_json(&self.wait_policy),
            render_rollback_reports_json(&self.rollback_reports),
            render_lifecycle_events_json(&self.lifecycle_events),
            render_reset_safety_checks_json(&self.reset_safety_checks),
            render_fixture_isolation_checks_json(&self.fixture_isolation_checks),
            render_fixture_drift_reports_json(&self.fixture_drift_reports),
            render_node_simulator_records_json(&self.node_simulator_records),
            render_phase0_smoke_records_json(&self.phase0_smoke_records),
            render_simulator_expansion_rules_json(&self.simulator_expansion_rules),
            render_redacted_log_exports_json(&self.redacted_log_exports),
            render_diagnostic_artifact_bundles_json(&self.diagnostic_artifact_bundles),
            render_clean_checkout_ci_entries_json(&self.clean_checkout_ci_entries),
            render_flake_evidence_records_json(&self.flake_evidence_records),
            render_artifact_retention_policies_json(&self.artifact_retention_policies),
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
        self.readiness_checks = stale_schema_readiness_checks(&self.profile, &self.trace_id);
        self.lifecycle_events.push(lifecycle_event(
            &self.profile,
            &self.trace_id,
            "local_stack.failed",
            "schema_compatibility",
            "schema_incompatible",
        ));
        self.diagnostic_refs.push(format!(
            "diagnostic://local_stack/schema_compatibility/{}",
            id_component(&self.trace_id)
        ));
    }

    fn apply_lifecycle_mode(&mut self, mode: LifecycleMode, command: DevCommand) {
        self.startup_graph =
            lifecycle_startup_steps_for_mode(mode, &self.profile, &self.trace_id, self.timeout_ms);
        self.readiness_checks =
            readiness_checks_for_mode(mode, &self.profile, &self.trace_id, self.timeout_ms);
        self.wait_policy =
            wait_policy_for_mode(mode, &self.profile, self.timeout_ms, self.poll_interval_ms);
        self.service_health = service_health_from_readiness(&self.readiness_checks);
        self.lifecycle_events = lifecycle_events_for_mode(
            mode,
            command,
            &self.profile,
            &self.trace_id,
            &self.startup_graph,
        );
        self.rollback_reports =
            rollback_reports_for_mode(mode, &self.trace_id, &self.startup_graph);
        if matches!(
            mode,
            LifecycleMode::HealthTimeout | LifecycleMode::RequiredFailure
        ) {
            self.artifact_refs.push(format!(
                "log://local_stack/startup_failure/{}",
                id_component(&self.trace_id)
            ));
        }
    }

    fn apply_shutdown_reports(&mut self, command: DevCommand) {
        self.shutdown_reports = shutdown_reports(&self.trace_id);
        self.lifecycle_events.extend(shutdown_lifecycle_events(
            command,
            &self.profile,
            &self.trace_id,
            &self.shutdown_reports,
        ));
    }

    fn fail_lifecycle(
        mut self,
        reason_code: &'static str,
        message: &'static str,
        exit_class: ExitCodeClass,
        retry_class: RetryClass,
    ) -> Self {
        self.push_state(LocalCommandState::CollectingArtifacts)
            .expect("local-stack failure artifact transition is valid");
        self.push_state(LocalCommandState::Failed)
            .expect("local-stack failed transition is valid");
        self.doctor_checks = doctor_checks_for_reason(reason_code);
        self.ok = false;
        self.status = LocalStackStatus::Failed;
        self.reason_code = reason_code.to_owned();
        self.message = message.to_owned();
        self.exit_class = exit_class;
        self.retry_class = retry_class;
        self
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
        capabilities.extend(["service:execution_loop"].into_iter().map(|service_id| {
            LocalServiceCapability {
                service_id: service_id.to_owned(),
                phase_gate: format!("phase_{master_phase}_blocked"),
                available: false,
                reason_code: "phase.local_service_unavailable".to_owned(),
            }
        }));
    }

    capabilities
}

fn foundation_service_ids() -> [&'static str; 7] {
    [
        "service:embedded_state",
        "service:api",
        "service:worker",
        "service:node_agent_simulator",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LifecycleServiceTemplate {
    service_id: &'static str,
    dependency_order: u8,
    required: bool,
    dependencies: &'static [&'static str],
    timeout_ms: u64,
}

const NO_SERVICE_DEPS: &[&str] = &[];
const EMBEDDED_STATE_DEPS: &[&str] = &["service:embedded_state"];
const API_DEPS: &[&str] = &[
    "service:embedded_state",
    "service:overqueue_jobs",
    "service:overstore_stub",
];
const WORKER_DEPS: &[&str] = &["service:overqueue_jobs", "service:event_audit"];
const EVENT_AUDIT_DEPS: &[&str] = &["service:embedded_state", "service:overqueue_jobs"];
const NODE_AGENT_DEPS: &[&str] = &[
    "service:api",
    "service:worker",
    "service:overqueue_jobs",
    "service:event_audit",
];

fn lifecycle_mode(profile: &str) -> LifecycleMode {
    let normalized = profile.to_ascii_lowercase();
    if normalized.contains("already-running") {
        LifecycleMode::AlreadyRunning
    } else if normalized.contains("health-timeout") {
        LifecycleMode::HealthTimeout
    } else if normalized.contains("required-failure") {
        LifecycleMode::RequiredFailure
    } else if normalized.contains("degraded-optional") || normalized.contains("optional-degraded") {
        LifecycleMode::OptionalDegraded
    } else {
        LifecycleMode::CleanStart
    }
}

fn lifecycle_templates(include_optional_developer_ui: bool) -> Vec<LifecycleServiceTemplate> {
    let mut templates = vec![
        LifecycleServiceTemplate {
            service_id: "service:embedded_state",
            dependency_order: 1,
            required: true,
            dependencies: NO_SERVICE_DEPS,
            timeout_ms: 10_000,
        },
        LifecycleServiceTemplate {
            service_id: "service:overqueue_jobs",
            dependency_order: 2,
            required: true,
            dependencies: EMBEDDED_STATE_DEPS,
            timeout_ms: 10_000,
        },
        LifecycleServiceTemplate {
            service_id: "service:overstore_stub",
            dependency_order: 3,
            required: true,
            dependencies: EMBEDDED_STATE_DEPS,
            timeout_ms: 10_000,
        },
        LifecycleServiceTemplate {
            service_id: "service:event_audit",
            dependency_order: 4,
            required: true,
            dependencies: EVENT_AUDIT_DEPS,
            timeout_ms: 10_000,
        },
        LifecycleServiceTemplate {
            service_id: "service:api",
            dependency_order: 5,
            required: true,
            dependencies: API_DEPS,
            timeout_ms: DEFAULT_LIFECYCLE_TIMEOUT_MS,
        },
        LifecycleServiceTemplate {
            service_id: "service:worker",
            dependency_order: 6,
            required: true,
            dependencies: WORKER_DEPS,
            timeout_ms: DEFAULT_LIFECYCLE_TIMEOUT_MS,
        },
        LifecycleServiceTemplate {
            service_id: "service:node_agent_simulator",
            dependency_order: 7,
            required: true,
            dependencies: NODE_AGENT_DEPS,
            timeout_ms: DEFAULT_LIFECYCLE_TIMEOUT_MS,
        },
    ];
    if include_optional_developer_ui {
        templates.push(LifecycleServiceTemplate {
            service_id: "service:developer_ui",
            dependency_order: 8,
            required: false,
            dependencies: &["service:api"],
            timeout_ms: 15_000,
        });
    }
    templates
}

fn lifecycle_startup_steps(
    profile: &str,
    trace_id: &str,
    timeout_ms: Option<u64>,
) -> Vec<LocalServiceLifecycleStep> {
    lifecycle_startup_steps_for_mode(lifecycle_mode(profile), profile, trace_id, timeout_ms)
}

fn lifecycle_startup_steps_for_mode(
    mode: LifecycleMode,
    _profile: &str,
    _trace_id: &str,
    timeout_ms: Option<u64>,
) -> Vec<LocalServiceLifecycleStep> {
    let failure_service = lifecycle_failure_service(mode);
    let failure_order = failure_service.and_then(|service_id| {
        lifecycle_templates(mode == LifecycleMode::OptionalDegraded)
            .into_iter()
            .find(|template| template.service_id == service_id)
            .map(|template| template.dependency_order)
    });
    lifecycle_templates(mode == LifecycleMode::OptionalDegraded)
        .into_iter()
        .map(|template| {
            let service_timeout = timeout_ms.unwrap_or(template.timeout_ms);
            let dependency_blocked = failure_order
                .map(|order| template.dependency_order > order)
                .unwrap_or(false);
            let (startup_state, health_state, readiness_state, liveness_state, reason_code) =
                if Some(template.service_id) == failure_service {
                    match mode {
                        LifecycleMode::HealthTimeout => (
                            "timeout",
                            "timeout",
                            "timeout",
                            "failed",
                            "local_stack.health_timeout",
                        ),
                        LifecycleMode::RequiredFailure => (
                            "failed",
                            "failed",
                            "failed",
                            "failed",
                            "local_stack.required_service_failed",
                        ),
                        _ => (
                            "ready",
                            "ready",
                            "ready",
                            "ready",
                            "local_stack.service_ready",
                        ),
                    }
                } else if dependency_blocked {
                    (
                        "not_started",
                        "blocked",
                        "blocked",
                        "not_started",
                        "local_stack.dependency_wait_blocked",
                    )
                } else if mode == LifecycleMode::AlreadyRunning {
                    (
                        "already_running",
                        "ready",
                        "ready",
                        "ready",
                        "local_stack.service_already_running",
                    )
                } else if mode == LifecycleMode::OptionalDegraded
                    && template.service_id == "service:developer_ui"
                {
                    (
                        "degraded",
                        "degraded",
                        "degraded",
                        "degraded",
                        "local_stack.optional_service_degraded",
                    )
                } else {
                    (
                        "ready",
                        "ready",
                        "ready",
                        "ready",
                        "local_stack.service_ready",
                    )
                };

            let start_after_ms = u64::from(template.dependency_order.saturating_sub(1)) * 25;
            let ready_after_ms = if startup_state == "timeout" {
                service_timeout
            } else if startup_state == "not_started" {
                start_after_ms
            } else {
                start_after_ms + 25
            };

            LocalServiceLifecycleStep {
                service_id: template.service_id.to_owned(),
                dependency_order: template.dependency_order,
                required: template.required,
                dependencies: template
                    .dependencies
                    .iter()
                    .map(|dependency| (*dependency).to_owned())
                    .collect(),
                startup_state: startup_state.to_owned(),
                health_state: health_state.to_owned(),
                readiness_state: readiness_state.to_owned(),
                liveness_state: liveness_state.to_owned(),
                start_after_ms,
                ready_after_ms,
                timeout_ms: service_timeout,
                reason_code: reason_code.to_owned(),
                rollback_on_failure: failure_order
                    .map(|order| template.required && template.dependency_order < order)
                    .unwrap_or(false),
            }
        })
        .collect()
}

fn lifecycle_failure_service(mode: LifecycleMode) -> Option<&'static str> {
    match mode {
        LifecycleMode::HealthTimeout => Some("service:api"),
        LifecycleMode::RequiredFailure => Some("service:worker"),
        _ => None,
    }
}

fn readiness_checks_for_profile(
    profile: &str,
    trace_id: &str,
    timeout_ms: Option<u64>,
) -> Vec<LocalReadinessCheck> {
    readiness_checks_for_mode(lifecycle_mode(profile), profile, trace_id, timeout_ms)
}

fn readiness_checks_for_mode(
    mode: LifecycleMode,
    profile: &str,
    trace_id: &str,
    timeout_ms: Option<u64>,
) -> Vec<LocalReadinessCheck> {
    lifecycle_startup_steps_for_mode(mode, profile, trace_id, timeout_ms)
        .into_iter()
        .map(|step| readiness_check_from_step(&step, false))
        .collect()
}

fn stale_schema_readiness_checks(profile: &str, trace_id: &str) -> Vec<LocalReadinessCheck> {
    lifecycle_startup_steps_for_mode(LifecycleMode::CleanStart, profile, trace_id, None)
        .into_iter()
        .map(|mut step| {
            step.startup_state = "failed".to_owned();
            step.health_state = "failed".to_owned();
            step.readiness_state = "stale_schema".to_owned();
            step.liveness_state = "not_started".to_owned();
            step.reason_code = "local_stack.schema_version_incompatible".to_owned();
            readiness_check_from_step(&step, true)
        })
        .collect()
}

fn readiness_check_from_step(
    step: &LocalServiceLifecycleStep,
    stale_schema: bool,
) -> LocalReadinessCheck {
    LocalReadinessCheck {
        service_id: step.service_id.clone(),
        endpoint_ref: binding_for_service(&step.service_id)
            .map(|binding| binding.endpoint_ref.to_owned())
            .unwrap_or_else(|| local_service_endpoint_ref(&step.service_id).to_owned()),
        health_state: step.health_state.clone(),
        readiness_state: step.readiness_state.clone(),
        liveness_state: step.liveness_state.clone(),
        dependency_state: if step.startup_state == "not_started" {
            "blocked".to_owned()
        } else if step
            .dependencies
            .iter()
            .all(|dependency| !dependency.is_empty())
        {
            "satisfied".to_owned()
        } else {
            "unknown".to_owned()
        },
        elapsed_ms: step.ready_after_ms,
        timeout_ms: step.timeout_ms,
        reason_code: step.reason_code.clone(),
        stale_schema,
    }
}

fn service_health_from_readiness(
    readiness_checks: &[LocalReadinessCheck],
) -> Vec<LocalServiceHealth> {
    readiness_checks
        .iter()
        .map(|check| {
            let binding = binding_for_service(&check.service_id);
            LocalServiceHealth {
                service_id: check.service_id.clone(),
                state: check.health_state.clone(),
                endpoint_ref: check.endpoint_ref.clone(),
                bind_host: binding
                    .map(|binding| binding.bind_host.to_owned())
                    .unwrap_or_else(|| "local".to_owned()),
                port: binding.map(|binding| binding.port),
                loopback_only: binding
                    .map(|binding| is_loopback_host(binding.bind_host))
                    .unwrap_or(true),
                reason_code: check.reason_code.clone(),
            }
        })
        .collect()
}

fn wait_policy_for_options(
    profile: &str,
    timeout_ms: Option<u64>,
    poll_interval_ms: Option<u64>,
) -> LocalWaitPolicy {
    wait_policy_for_mode(
        lifecycle_mode(profile),
        profile,
        timeout_ms,
        poll_interval_ms,
    )
}

fn wait_policy_for_mode(
    mode: LifecycleMode,
    profile: &str,
    timeout_ms: Option<u64>,
    poll_interval_ms: Option<u64>,
) -> LocalWaitPolicy {
    let timeout = timeout_ms.unwrap_or(DEFAULT_LIFECYCLE_TIMEOUT_MS);
    let poll_interval = poll_interval_ms.unwrap_or(DEFAULT_LIFECYCLE_POLL_INTERVAL_MS);
    let (timeout_class, reason_code, logs_exported_on_timeout) = match mode {
        LifecycleMode::HealthTimeout => (
            "required_service_readiness_timeout",
            "local_stack.health_timeout",
            true,
        ),
        LifecycleMode::RequiredFailure => (
            "required_service_failed",
            "local_stack.required_service_failed",
            true,
        ),
        _ => (
            "bounded_startup_window",
            "local_stack.bounded_wait_configured",
            false,
        ),
    };
    LocalWaitPolicy {
        timeout_ms: timeout,
        poll_interval_ms: poll_interval,
        bounded: true,
        no_unbounded_sleep: true,
        timeout_class: timeout_class.to_owned(),
        reason_code: reason_code.to_owned(),
        dependency_wait_diagnostics_ref: format!(
            "diagnostic://local_stack/wait/{}/{}",
            id_component(profile),
            timeout
        ),
        logs_exported_on_timeout,
    }
}

fn shutdown_reports(trace_id: &str) -> Vec<LocalServiceShutdownReport> {
    let mut templates = lifecycle_templates(false);
    templates.sort_by_key(|template| std::cmp::Reverse(template.dependency_order));
    templates
        .into_iter()
        .enumerate()
        .map(|(index, template)| LocalServiceShutdownReport {
            service_id: template.service_id.to_owned(),
            shutdown_order: (index + 1) as u8,
            graceful: true,
            state_preserved: true,
            reason_code: "local_stack.service_stopped".to_owned(),
            diagnostic_ref: format!(
                "diagnostic://local_stack/shutdown/{}/{}",
                id_component(template.service_id),
                id_component(trace_id)
            ),
        })
        .collect()
}

fn rollback_reports_for_mode(
    mode: LifecycleMode,
    trace_id: &str,
    startup_graph: &[LocalServiceLifecycleStep],
) -> Vec<LocalRollbackReport> {
    if !matches!(
        mode,
        LifecycleMode::HealthTimeout | LifecycleMode::RequiredFailure
    ) {
        return Vec::new();
    }
    startup_graph
        .iter()
        .filter(|step| step.rollback_on_failure)
        .rev()
        .enumerate()
        .map(|(index, step)| LocalRollbackReport {
            service_id: step.service_id.clone(),
            rollback_order: (index + 1) as u8,
            action: "stop_started_service".to_owned(),
            state_preserved: true,
            reason_code: "local_stack.partial_start_rollback".to_owned(),
            artifact_ref: format!(
                "artifact://local_stack/rollback/{}/{}",
                id_component(&step.service_id),
                id_component(trace_id)
            ),
        })
        .collect()
}

fn lifecycle_event(
    profile: &str,
    trace_id: &str,
    event_type: &str,
    service_id: &str,
    discriminator: &str,
) -> LocalLifecycleEventRecord {
    LocalLifecycleEventRecord {
        event_id: format!(
            "event://local_stack/{}/{}/{}",
            id_component(event_type),
            id_component(service_id),
            id_component(discriminator)
        ),
        event_type: event_type.to_owned(),
        service_id: service_id.to_owned(),
        trace_id: trace_id.to_owned(),
        stack_profile: profile.to_owned(),
        fixture_version: "fixture:phase2_default_local".to_owned(),
        schema_version: SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION.to_owned(),
        artifact_ref: format!(
            "diagnostic://local_stack/lifecycle/{}/{}",
            id_component(event_type),
            id_component(trace_id)
        ),
        redaction_summary: "secret_free".to_owned(),
        contains_raw_secret: false,
        local_only: true,
        test_only: true,
    }
}

fn lifecycle_events_for_mode(
    mode: LifecycleMode,
    command: DevCommand,
    profile: &str,
    trace_id: &str,
    startup_graph: &[LocalServiceLifecycleStep],
) -> Vec<LocalLifecycleEventRecord> {
    let mut events = vec![lifecycle_event(
        profile,
        trace_id,
        "local_stack.start_requested",
        "stack",
        command.action(),
    )];
    for step in startup_graph {
        events.push(lifecycle_event(
            profile,
            trace_id,
            "local_stack.service_starting",
            &step.service_id,
            &step.dependency_order.to_string(),
        ));
        let event_type = match step.startup_state.as_str() {
            "ready" => "local_stack.service_ready",
            "already_running" => "local_stack.service_ready",
            "degraded" => "local_stack.service_degraded",
            "timeout" => "local_stack.failed",
            "failed" => "local_stack.failed",
            "not_started" => "local_stack.service_blocked",
            _ => "local_stack.service_starting",
        };
        events.push(lifecycle_event(
            profile,
            trace_id,
            event_type,
            &step.service_id,
            &step.startup_state,
        ));
    }
    match mode {
        LifecycleMode::HealthTimeout | LifecycleMode::RequiredFailure => {
            events.push(lifecycle_event(
                profile,
                trace_id,
                "local_stack.failed",
                "stack",
                command.action(),
            ))
        }
        LifecycleMode::OptionalDegraded => events.push(lifecycle_event(
            profile,
            trace_id,
            "local_stack.service_degraded",
            "service:developer_ui",
            command.action(),
        )),
        _ => {}
    }
    events
}

fn shutdown_lifecycle_events(
    command: DevCommand,
    profile: &str,
    trace_id: &str,
    reports: &[LocalServiceShutdownReport],
) -> Vec<LocalLifecycleEventRecord> {
    let mut events = vec![lifecycle_event(
        profile,
        trace_id,
        match command {
            DevCommand::Restart => "local_stack.restart_requested",
            _ => "local_stack.stop_requested",
        },
        "stack",
        command.action(),
    )];
    for report in reports {
        events.push(lifecycle_event(
            profile,
            trace_id,
            "local_stack.service_stopped",
            &report.service_id,
            &report.shutdown_order.to_string(),
        ));
    }
    events
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

fn reset_safety_checks_for_profile(profile: &str, dry_run: bool) -> Vec<LocalResetSafetyCheck> {
    let normalized = profile.to_ascii_lowercase();
    let unsafe_marker = if normalized.contains("missing-reset-marker") {
        Some(("missing", "local_stack.reset_marker_missing"))
    } else if normalized.contains("stale-reset-marker") {
        Some(("stale", "local_stack.reset_marker_stale"))
    } else if normalized.contains("inconsistent-reset-marker") {
        Some(("inconsistent", "local_stack.reset_marker_inconsistent"))
    } else {
        None
    };
    let planned_action = if dry_run {
        "dry_run_delete"
    } else {
        "delete_marked_test_state"
    };

    [
        (
            "reset_check:embedded_state",
            "local-state://embedded_state",
            "local-state://embedded_state/.overrid-local-test-state",
        ),
        (
            "reset_check:overqueue_jobs",
            "overqueue://local/jobs",
            "local-state://overqueue_jobs/.overrid-local-test-state",
        ),
        (
            "reset_check:overstore_stub",
            "artifact://local_stack",
            "artifact://local_stack/.overrid-local-test-state",
        ),
        (
            "reset_check:event_audit",
            "event-audit://local_stack/events",
            "local-state://event_audit/.overrid-local-test-state",
        ),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (check_id, target_ref, marker_ref))| {
        let (marker_state, reason_code) = if index == 0 {
            unsafe_marker.unwrap_or(("verified", "local_stack.reset_marker_verified"))
        } else {
            ("verified", "local_stack.reset_marker_verified")
        };
        let safe_to_delete = marker_state == "verified";
        LocalResetSafetyCheck {
            check_id: check_id.to_owned(),
            target_ref: target_ref.to_owned(),
            marker_ref: marker_ref.to_owned(),
            marker_state: marker_state.to_owned(),
            planned_action: planned_action.to_owned(),
            deletion_plan_ref: format!(
                "reset-plan://local_stack/phase7/{}",
                id_component(target_ref)
            ),
            safe_to_delete,
            reason_code: reason_code.to_owned(),
            local_only: true,
            test_only: true,
        }
    })
    .collect()
}

fn seed_fixture_records() -> Vec<LocalSeedFixtureRecord> {
    [
        (
            "fixture:phase7:tenant",
            "tenant",
            "fixture://local_stack/tenant/local_alpha",
            "tenant:local:alpha",
        ),
        (
            "fixture:phase7:actor",
            "actor",
            "fixture://local_stack/actor/local_builder",
            "actor:local:builder",
        ),
        (
            "fixture:phase7:key",
            "key",
            "fixture://local_stack/key/local_test_signing",
            "key:local:test_signing",
        ),
        (
            "fixture:phase7:node",
            "node",
            "fixture://local_stack/node/local_simulator_one",
            "node:local:simulator:one",
        ),
        (
            "fixture:phase7:manifest",
            "manifest",
            "fixture://local_stack/manifest/local_noop",
            "manifest:local:noop",
        ),
        (
            "fixture:phase7:package",
            "package",
            "fixture://local_stack/package/local_noop_package",
            "package:local:noop_package",
        ),
        (
            "fixture:phase7:workload",
            "workload",
            "fixture://local_stack/workload/phase1_pending_work",
            "workload:local:phase1_pending_work",
        ),
        (
            "fixture:phase7:oru_account",
            "oru_account",
            "fixture://local_stack/oru_account/local_alpha",
            "oru:local:account:alpha",
        ),
        (
            "fixture:phase7:seal_ledger_ref",
            "seal_ledger_ref",
            "fixture://local_stack/seal_ledger/phase1",
            "ledger:local:seal:phase1",
        ),
        (
            "fixture:phase7:policy_context",
            "policy_context",
            "fixture://local_stack/policy_context/phase1_smoke",
            "policy:local:allow_phase1_smoke",
        ),
    ]
    .into_iter()
    .enumerate()
    .map(
        |(index, (fixture_id, fixture_kind, fixture_ref, stable_id))| LocalSeedFixtureRecord {
            fixture_id,
            fixture_kind,
            fixture_ref,
            stable_id,
            apply_order: (index + 1) as u8,
            fixture_version: LOCAL_STACK_PHASE7_FIXTURE_VERSION,
            deterministic_seed: LOCAL_STACK_PHASE7_DETERMINISTIC_SEED,
            profile_binding: "profile:local_default",
            schema_version: SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
            local_only: true,
            test_only: true,
        },
    )
    .collect()
}

fn fixture_isolation_checks(profile: &str) -> Vec<LocalFixtureIsolationCheck> {
    [
        ("local", true, "fixture.isolation.profile_allowed"),
        ("ci", true, "fixture.isolation.profile_allowed"),
        ("seed", false, "profile.not_local_test"),
        ("staging", false, "profile.not_local_test"),
        ("production_like", false, "profile.not_local_test"),
        ("federation", false, "profile.not_local_test"),
        ("public_provider", false, "profile.not_local_test"),
        ("non_local", false, "profile.not_local_test"),
    ]
    .into_iter()
    .map(
        |(profile_class, accepted, reason_code)| LocalFixtureIsolationCheck {
            check_id: format!("fixture_isolation:{profile_class}"),
            fixture_ref: "fixture://local_stack/key/local_test_signing".to_owned(),
            credential_ref: "secret://local_stack/fixture_key_ref".to_owned(),
            profile_class: profile_class.to_owned(),
            bound_profile: profile.to_owned(),
            accepted,
            reason_code: reason_code.to_owned(),
            contains_raw_secret: false,
            local_only: true,
            test_only: true,
        },
    )
    .collect()
}

fn phase1_seed_prerequisites() -> Vec<LocalControlPlaneSeedPrerequisite> {
    [
        (
            "phase1_seed:tenant",
            "tenant",
            "fixture://local_stack/tenant/local_alpha",
            "tenant:local:alpha",
        ),
        (
            "phase1_seed:actor",
            "actor",
            "fixture://local_stack/actor/local_builder",
            "actor:local:builder",
        ),
        (
            "phase1_seed:key",
            "key",
            "fixture://local_stack/key/local_test_signing",
            "key:local:test_signing",
        ),
        (
            "phase1_seed:manifest",
            "manifest",
            "fixture://local_stack/manifest/local_noop",
            "manifest:local:noop",
        ),
        (
            "phase1_seed:idempotency_key",
            "idempotency_key",
            "fixture://local_stack/idempotency/phase1_control_plane",
            "idem:local_stack:phase1:control_plane",
        ),
        (
            "phase1_seed:trace_root",
            "trace_root",
            "fixture://local_stack/trace/phase1_control_plane",
            "trace_local_stack_phase1_control_plane",
        ),
        (
            "phase1_seed:pending_work_target",
            "pending_work_target",
            "fixture://local_stack/workload/phase1_pending_work",
            "overqueue://local/jobs/workload:local:phase1_pending_work",
        ),
    ]
    .into_iter()
    .map(
        |(prerequisite_id, prerequisite_kind, fixture_ref, stable_ref)| {
            LocalControlPlaneSeedPrerequisite {
                prerequisite_id,
                prerequisite_kind,
                fixture_ref,
                phase_gate: "local_smoke_prerequisite",
                stable_ref,
                signing_required: true,
                id_preserved_for_phase1: true,
                bypasses_signing: false,
                reason_code: "local_stack.phase1_seed_prerequisite_ready",
                local_only: true,
                test_only: true,
            }
        },
    )
    .collect()
}

fn fixture_drift_reports_for_profile(profile: &str) -> Vec<LocalFixtureDriftReport> {
    let normalized = profile.to_ascii_lowercase();
    let mut reports = vec![
        fixture_drift_report(
            "generated_ids",
            "fixture://local_stack/seed_manifest/phase7",
            "seed_ids:stable",
            "seed_ids:stable",
            false,
            "local_stack.fixture_ids_stable",
        ),
        fixture_drift_report(
            "schema_versions",
            "fixture://local_stack/schema/local_development_stack",
            SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
            SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
            false,
            "local_stack.fixture_schema_current",
        ),
        fixture_drift_report(
            "event_refs",
            "fixture://local_stack/events/phase7",
            "event:local_stack:seed_completed:phase7",
            "event:local_stack:seed_completed:phase7",
            false,
            "local_stack.fixture_event_refs_stable",
        ),
        fixture_drift_report(
            "local_accounts",
            "fixture://local_stack/oru_account/local_alpha",
            "oru:local:account:alpha",
            "oru:local:account:alpha",
            false,
            "local_stack.fixture_account_refs_stable",
        ),
        fixture_drift_report(
            "artifact_hashes",
            "artifact://local_stack/phase5/noop_payload",
            &blake3_hex(local_artifact_payload().as_bytes()),
            &blake3_hex(local_artifact_payload().as_bytes()),
            false,
            "local_stack.fixture_artifact_hashes_stable",
        ),
    ];

    if normalized.contains("nondeterministic-fixture-ids") {
        reports.push(fixture_drift_report(
            "generated_ids",
            "fixture://local_stack/seed_manifest/phase7",
            "seed_ids:stable",
            "seed_ids:randomized",
            true,
            "local_stack.fixture_id_drift",
        ));
    }
    if normalized.contains("fixture-schema-drift") {
        reports.push(fixture_drift_report(
            "schema_versions",
            "fixture://local_stack/schema/local_development_stack",
            SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
            "local-development-stack.v99.0",
            true,
            "local_stack.fixture_schema_drift",
        ));
    }
    if normalized.contains("missing-fixture-ref") {
        reports.push(fixture_drift_report(
            "missing_refs",
            "fixture://local_stack/workload/phase1_pending_work",
            "workload:local:phase1_pending_work",
            "missing",
            true,
            "local_stack.fixture_ref_missing",
        ));
    }
    if normalized.contains("extra-fixture-record") {
        reports.push(fixture_drift_report(
            "unexpected_extra_records",
            "fixture://local_stack/seed_manifest/phase7",
            "extra_records:none",
            "extra_records:1",
            true,
            "local_stack.fixture_extra_record",
        ));
    }

    reports
}

fn fixture_drift_report(
    diff_field: &str,
    fixture_ref: &str,
    expected_ref: &str,
    actual_ref: &str,
    drift_detected: bool,
    reason_code: &str,
) -> LocalFixtureDriftReport {
    LocalFixtureDriftReport {
        report_id: format!("fixture_drift:{diff_field}:{}", id_component(fixture_ref)),
        fixture_ref: fixture_ref.to_owned(),
        diff_field: diff_field.to_owned(),
        expected_ref: expected_ref.to_owned(),
        actual_ref: actual_ref.to_owned(),
        drift_detected,
        reason_code: reason_code.to_owned(),
        blocks_seed: drift_detected,
        local_only: true,
        test_only: true,
    }
}

fn node_simulator_records() -> Vec<LocalNodeSimulatorRecord> {
    vec![LocalNodeSimulatorRecord {
        simulator_id: "node_simulator:local:overcell_like:one".to_owned(),
        service_id: "service:node_agent_simulator".to_owned(),
        deterministic_identity_ref: "node://local_stack/simulator/node-agent-0001".to_owned(),
        heartbeat_ref: "heartbeat://local_stack/node-agent-0001/periodic".to_owned(),
        capability_report_ref: "capability://local_stack/node-agent-0001/phase8_fixture".to_owned(),
        health_endpoint_ref: "http://127.0.0.1:18082/healthz".to_owned(),
        accepted_fixture_workload_ref: "workload://local_stack/phase8/noop_fixture".to_owned(),
        noop_handoff_ref: "handoff://local_stack/node-agent-0001/noop_execution".to_owned(),
        real_hardware_discovery: false,
        gpu_runtime_integration: false,
        benchmark_publication: false,
        installer_update_flow: false,
        remote_shell_behavior: false,
        provider_eligibility_decision: false,
        reason_code: "local_stack.node_simulator_fixture_ready".to_owned(),
        local_only: true,
        test_only: true,
    }]
}

fn phase0_smoke_records(trace_id: &str) -> Vec<LocalPhase0SmokeRecord> {
    let trace_component = id_component(trace_id);
    vec![LocalPhase0SmokeRecord {
        smoke_id: format!("phase0_smoke:{}", trace_component),
        signed_noop_command_ref: "command://local_stack/phase0/noop_signed".to_owned(),
        audit_event_ref: format!(
            "event://local_stack/phase0_smoke/{trace_component}/audit_roundtrip"
        ),
        invalid_schema_ref: "schema://local_stack/phase0/noop_command/invalid_denied".to_owned(),
        trace_id: trace_id.to_owned(),
        fixture_state_ref: "state://local_stack/fixture/phase8/noop_workload".to_owned(),
        redacted_artifact_ref: format!(
            "artifact://local_stack/smoke/{trace_component}/redacted_bundle"
        ),
        signed_noop_admitted: true,
        audit_event_write_read: true,
        invalid_schema_denied: true,
        trace_id_propagated: true,
        fixture_state_inspected: true,
        public_local_api_only: true,
        generated_contracts_only: true,
        contains_raw_secret: false,
        reason_code: "local_stack.phase0_smoke_path_verified".to_owned(),
        local_only: true,
        test_only: true,
    }]
}

fn harness_hook_records() -> Vec<LocalHarnessHookRecord> {
    [
        (
            "start",
            "overrid dev start",
            "LocalStackHarness::start_stack",
            "health_snapshot",
        ),
        (
            "status",
            "overrid dev status",
            "LocalStackHarness::status_stack",
            "status_snapshot",
        ),
        (
            "reset",
            "overrid dev reset",
            "LocalStackHarness::reset_stack",
            "reset_report",
        ),
        (
            "seed",
            "overrid dev seed",
            "LocalStackHarness::seed_stack",
            "seed_report",
        ),
        (
            "smoke",
            "overrid dev smoke",
            "LocalStackHarness::run_phase0_smoke",
            "smoke_report",
        ),
        (
            "logs",
            "overrid dev logs",
            "LocalStackHarness::logs",
            "redacted_logs",
        ),
        (
            "health_snapshots",
            "overrid dev doctor",
            "LocalStackHarness::health_snapshots",
            "health_snapshot",
        ),
        (
            "event_export",
            "overrid dev smoke --json",
            "LocalStackHarness::event_export",
            "event_export",
        ),
        (
            "artifact_collection",
            "overrid dev smoke --json",
            "LocalStackHarness::artifact_collection",
            "artifact_collection",
        ),
    ]
    .into_iter()
    .map(
        |(hook_id, command_surface, harness_method, evidence_ref)| LocalHarnessHookRecord {
            hook_id: format!("harness_hook:{hook_id}"),
            command_surface: command_surface.to_owned(),
            harness_method: harness_method.to_owned(),
            evidence_ref: format!("evidence://local_stack/phase8/{evidence_ref}"),
            generated_contract_ref: format!("contract://overrid/local_stack/harness/{hook_id}"),
            required_for_phase0_smoke: true,
            public_local_api_only: true,
            local_only: true,
            test_only: true,
        },
    )
    .collect()
}

fn cli_sdk_smoke_support_records() -> Vec<LocalSdkSmokeSupportRecord> {
    [
        (
            "cli",
            "overrid-cli",
            "cli://overrid/dev/smoke?profile=local&json=true",
            "contract://overrid/local_stack/phase0_smoke/cli_request",
        ),
        (
            "rust_sdk",
            "overrid-rust-sdk",
            "sdk://overrid/local_stack/phase0_smoke/request",
            "contract://overrid/local_stack/phase0_smoke/rust_sdk_request",
        ),
    ]
    .into_iter()
    .map(
        |(support_id, client_surface, request_example_ref, generated_contract_ref)| {
            LocalSdkSmokeSupportRecord {
                support_id: format!("smoke_support:{support_id}"),
                client_surface: client_surface.to_owned(),
                request_example_ref: request_example_ref.to_owned(),
                generated_contract_ref: generated_contract_ref.to_owned(),
                public_local_api_only: true,
                avoids_private_storage: true,
                avoids_simulator_internals: true,
                reason_code: "local_stack.phase0_smoke_support_public_api_only".to_owned(),
                local_only: true,
                test_only: true,
            }
        },
    )
    .collect()
}

fn simulator_expansion_rules() -> Vec<LocalSimulatorExpansionRule> {
    [
        (
            "node_registration",
            "phase_2_node_registration",
            "contract://overrid/node_registration/local_simulator_owner",
            "local_stack.simulator_owner_contract_required",
        ),
        (
            "execution_loop",
            "phase_3_execution_loop",
            "contract://overrid/execution_loop/local_simulator_owner",
            "local_stack.execution_owner_contract_required",
        ),
        (
            "policy",
            "phase_4_policy",
            "contract://overrid/policy/local_simulator_owner",
            "local_stack.policy_owner_contract_required",
        ),
        (
            "accounting",
            "phase_5_accounting",
            "contract://overrid/accounting/local_simulator_owner",
            "local_stack.accounting_owner_contract_required",
        ),
        (
            "storage_namespace",
            "phase_8_storage_namespace",
            "contract://overrid/storage_namespace/local_simulator_owner",
            "local_stack.storage_namespace_owner_contract_required",
        ),
    ]
    .into_iter()
    .map(
        |(rule_id, target_phase, owning_contract_ref, reason_code)| LocalSimulatorExpansionRule {
            rule_id: format!("simulator_expansion_rule:{rule_id}"),
            target_phase: target_phase.to_owned(),
            owning_contract_ref: owning_contract_ref.to_owned(),
            local_test_marker_required: true,
            production_contract_shape_required: true,
            phase0_responsibility_drift_allowed: false,
            blocks_without_owner: true,
            reason_code: reason_code.to_owned(),
            local_only: true,
            test_only: true,
        },
    )
    .collect()
}

fn redacted_log_export_records(trace_id: &str) -> Vec<LocalRedactedLogExportRecord> {
    let trace_component = id_component(trace_id);
    [
        ("api", "service:api"),
        ("worker", "service:worker"),
        ("embedded_state", "service:embedded_state"),
        ("overqueue_jobs", "service:overqueue_jobs"),
        ("overstore_stub", "service:overstore_stub"),
        ("event_audit", "service:event_audit"),
        ("node_agent_simulator", "service:node_agent_simulator"),
    ]
    .into_iter()
    .map(|(surface, service_id)| LocalRedactedLogExportRecord {
        stream_id: format!("redacted_log_stream:{surface}:{trace_component}"),
        service_id: service_id.to_owned(),
        log_ref: format!("log://local_stack/{surface}/{trace_component}/redacted"),
        bundle_section_ref: format!(
            "artifact://local_stack/diagnostics/{trace_component}/logs/{surface}"
        ),
        redacts_secrets: true,
        redacts_tokens: true,
        redacts_signatures: true,
        redacts_private_payloads: true,
        redacts_encrypted_content: true,
        export_blocked_until_secret_free: true,
        scanner_passed: true,
        reason_code: "local_stack.redacted_log_export_ready".to_owned(),
        local_only: true,
        test_only: true,
    })
    .collect()
}

fn diagnostic_artifact_bundle_records(
    profile: &str,
    trace_id: &str,
) -> Vec<LocalDiagnosticArtifactBundleRecord> {
    let trace_component = id_component(trace_id);
    vec![LocalDiagnosticArtifactBundleRecord {
        bundle_id: format!("diagnostic_bundle:local_stack:{trace_component}"),
        bundle_ref: format!("artifact://local_stack/diagnostics/{trace_component}/bundle.json"),
        manifest_ref: format!("manifest://local_stack/diagnostics/{trace_component}"),
        health_snapshot_refs: vec![
            "health://local_stack/service/api".to_owned(),
            "health://local_stack/service/worker".to_owned(),
            "health://local_stack/service/node_agent_simulator".to_owned(),
            "health://local_stack/service/overstore_stub".to_owned(),
            "health://local_stack/service/event_audit".to_owned(),
        ],
        stack_profile: profile.to_owned(),
        schema_version: SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION.to_owned(),
        fixture_version: LOCAL_STACK_PHASE8_SMOKE_FIXTURE_VERSION.to_owned(),
        command_lifecycle_ref: format!("lifecycle://local_stack/{trace_component}"),
        trace_id: trace_id.to_owned(),
        reason_code_ref: format!("reason://local_stack/{trace_component}"),
        local_event_refs: vec![
            format!("event://local_stack/{trace_component}/command_requested"),
            format!("event://local_stack/{trace_component}/collecting_artifacts"),
        ],
        queue_state_refs: vec![
            "queue://local_stack/overqueue/job/job-local-0001".to_owned(),
            "queue://local_stack/overqueue/job/job-local-0002".to_owned(),
        ],
        object_refs: vec![
            "overstore://local_stub/blake3/phase5_noop_payload".to_owned(),
            format!("artifact://local_stack/smoke/{trace_component}/redacted_bundle"),
        ],
        reproduction_command: format!("overrid dev smoke --json --trace-id {trace_id}"),
        retention_class: "failure_evidence".to_owned(),
        contains_raw_secret: false,
        local_only: true,
        test_only: true,
    }]
}

fn clean_checkout_ci_entries() -> Vec<LocalCleanCheckoutCiEntrypointRecord> {
    vec![LocalCleanCheckoutCiEntrypointRecord {
        entrypoint_id: "ci_entrypoint:local_stack:clean_checkout".to_owned(),
        runner_ref: LOCAL_STACK_PHASE9_CI_RUNNER_REF.to_owned(),
        os_family: "linux".to_owned(),
        arch: "x86_64".to_owned(),
        ubuntu_24_04_equivalent: true,
        repo_pinned_rust_toolchain: true,
        loopback_networking: true,
        cloud_credentials_allowed: false,
        external_database_allowed: false,
        external_queue_allowed: false,
        external_object_store_allowed: false,
        commands: [
            "dev:start",
            "dev:reset",
            "dev:seed",
            "dev:smoke",
            "schema:check",
            "layout:check",
            "docs:check",
            "harness:smoke",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
        allowed_outcomes: ["success", "blocked", "failed"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        machine_readable_output: true,
        reason_code: "local_stack.clean_checkout_ci_entrypoint_ready".to_owned(),
        local_only: true,
        test_only: true,
    }]
}

fn flake_evidence_records_for_profile(
    profile: &str,
    trace_id: &str,
) -> Vec<LocalFlakeEvidenceRecord> {
    let normalized = profile.to_ascii_lowercase();
    let trace_component = id_component(trace_id);
    let nondeterministic_fixture_ids = normalized.contains("nondeterministic-fixture-ids");
    let unstable_event_ordering = normalized.contains("unstable-event-ordering");
    let health_timeout = normalized.contains("health-timeout");
    let tolerance_window_used = normalized.contains("tolerance-window");
    let flake_detected = nondeterministic_fixture_ids
        || unstable_event_ordering
        || health_timeout
        || tolerance_window_used;

    vec![LocalFlakeEvidenceRecord {
        evidence_id: format!("flake_evidence:local_stack:{trace_component}"),
        trace_id: trace_id.to_owned(),
        repeated_run_count: 5,
        startup_timing_variance_ms: if health_timeout { 1_250 } else { 12 },
        nondeterministic_fixture_ids,
        unstable_event_ordering,
        retry_count: if flake_detected { 2 } else { 0 },
        health_timeout_class: if health_timeout {
            "required_service_timeout".to_owned()
        } else {
            "none".to_owned()
        },
        tolerance_window_used,
        flake_detected,
        reason_code: if flake_detected {
            "local_stack.flake_signal_recorded".to_owned()
        } else {
            "local_stack.flake_metadata_stable".to_owned()
        },
        artifact_ref: format!("artifact://local_stack/flake/{trace_component}/evidence.json"),
        local_only: true,
        test_only: true,
    }]
}

fn artifact_retention_policies() -> Vec<LocalArtifactRetentionPolicyRecord> {
    [
        (
            "success_summary",
            true,
            false,
            7,
            "local_stack.success_summary_compacted",
        ),
        (
            "failure_evidence",
            false,
            true,
            30,
            "local_stack.failure_bundle_retained",
        ),
    ]
    .into_iter()
    .map(
        |(
            retention_class,
            compact_success_summary,
            retain_failure_bundle,
            failure_retention_days,
            reason_code,
        )| LocalArtifactRetentionPolicyRecord {
            policy_id: format!("retention_policy:local_stack:{retention_class}"),
            retention_class: retention_class.to_owned(),
            compact_success_summary,
            retain_failure_bundle,
            failure_retention_days,
            prune_command_ref: "command://local_stack/prune --test-state-marker-required"
                .to_owned(),
            requires_test_state_marker: true,
            deletes_unmarked_user_dirs: false,
            deletes_production_like_state: false,
            deletes_non_local_artifacts: false,
            reason_code: reason_code.to_owned(),
            local_only: true,
            test_only: true,
        },
    )
    .collect()
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
        "local_stack.reset_unsafe_state" => Some((
            "doctor:local_volume_markers",
            "local_stack.reset_unsafe_state",
            "restore .overrid-local-test-state markers before reset",
        )),
        "local_stack.fixture_drift_detected" => Some((
            "doctor:schema_outputs",
            "local_stack.fixture_drift_detected",
            "regenerate deterministic fixture records and inspect drift reports",
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
        || normalized.contains("federation")
        || normalized.contains("public-provider")
        || normalized.contains("public_provider")
        || normalized.contains("non-local")
        || normalized.contains("non_local")
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
        Starting => matches!(next, Ready | CollectingArtifacts | Blocked | Failed),
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

fn render_startup_graph_json(steps: &[LocalServiceLifecycleStep]) -> String {
    let rendered = steps
        .iter()
        .map(|step| {
            format!(
                concat!(
                    "{{",
                    "\"service_id\":\"{}\",",
                    "\"dependency_order\":{},",
                    "\"required\":{},",
                    "\"dependencies\":{},",
                    "\"startup_state\":\"{}\",",
                    "\"health_state\":\"{}\",",
                    "\"readiness_state\":\"{}\",",
                    "\"liveness_state\":\"{}\",",
                    "\"start_after_ms\":{},",
                    "\"ready_after_ms\":{},",
                    "\"timeout_ms\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"rollback_on_failure\":{}",
                    "}}"
                ),
                json_escape(&step.service_id),
                step.dependency_order,
                step.required,
                json_owned_string_array(&step.dependencies),
                json_escape(&step.startup_state),
                json_escape(&step.health_state),
                json_escape(&step.readiness_state),
                json_escape(&step.liveness_state),
                step.start_after_ms,
                step.ready_after_ms,
                step.timeout_ms,
                json_escape(&step.reason_code),
                step.rollback_on_failure,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_shutdown_reports_json(reports: &[LocalServiceShutdownReport]) -> String {
    let rendered = reports
        .iter()
        .map(|report| {
            format!(
                concat!(
                    "{{",
                    "\"service_id\":\"{}\",",
                    "\"shutdown_order\":{},",
                    "\"graceful\":{},",
                    "\"state_preserved\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"diagnostic_ref\":\"{}\"",
                    "}}"
                ),
                json_escape(&report.service_id),
                report.shutdown_order,
                report.graceful,
                report.state_preserved,
                json_escape(&report.reason_code),
                json_escape(&report.diagnostic_ref),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_readiness_checks_json(checks: &[LocalReadinessCheck]) -> String {
    let rendered = checks
        .iter()
        .map(|check| {
            format!(
                concat!(
                    "{{",
                    "\"service_id\":\"{}\",",
                    "\"endpoint_ref\":\"{}\",",
                    "\"health_state\":\"{}\",",
                    "\"readiness_state\":\"{}\",",
                    "\"liveness_state\":\"{}\",",
                    "\"dependency_state\":\"{}\",",
                    "\"elapsed_ms\":{},",
                    "\"timeout_ms\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"stale_schema\":{}",
                    "}}"
                ),
                json_escape(&check.service_id),
                json_escape(&check.endpoint_ref),
                json_escape(&check.health_state),
                json_escape(&check.readiness_state),
                json_escape(&check.liveness_state),
                json_escape(&check.dependency_state),
                check.elapsed_ms,
                check.timeout_ms,
                json_escape(&check.reason_code),
                check.stale_schema,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_wait_policy_json(policy: &LocalWaitPolicy) -> String {
    format!(
        concat!(
            "{{",
            "\"timeout_ms\":{},",
            "\"poll_interval_ms\":{},",
            "\"bounded\":{},",
            "\"no_unbounded_sleep\":{},",
            "\"timeout_class\":\"{}\",",
            "\"reason_code\":\"{}\",",
            "\"dependency_wait_diagnostics_ref\":\"{}\",",
            "\"logs_exported_on_timeout\":{}",
            "}}"
        ),
        policy.timeout_ms,
        policy.poll_interval_ms,
        policy.bounded,
        policy.no_unbounded_sleep,
        json_escape(&policy.timeout_class),
        json_escape(&policy.reason_code),
        json_escape(&policy.dependency_wait_diagnostics_ref),
        policy.logs_exported_on_timeout,
    )
}

fn render_rollback_reports_json(reports: &[LocalRollbackReport]) -> String {
    let rendered = reports
        .iter()
        .map(|report| {
            format!(
                concat!(
                    "{{",
                    "\"service_id\":\"{}\",",
                    "\"rollback_order\":{},",
                    "\"action\":\"{}\",",
                    "\"state_preserved\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"artifact_ref\":\"{}\"",
                    "}}"
                ),
                json_escape(&report.service_id),
                report.rollback_order,
                json_escape(&report.action),
                report.state_preserved,
                json_escape(&report.reason_code),
                json_escape(&report.artifact_ref),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_lifecycle_events_json(events: &[LocalLifecycleEventRecord]) -> String {
    let rendered = events
        .iter()
        .map(|event| {
            format!(
                concat!(
                    "{{",
                    "\"event_id\":\"{}\",",
                    "\"event_type\":\"{}\",",
                    "\"service_id\":\"{}\",",
                    "\"trace_id\":\"{}\",",
                    "\"stack_profile\":\"{}\",",
                    "\"fixture_version\":\"{}\",",
                    "\"schema_version\":\"{}\",",
                    "\"artifact_ref\":\"{}\",",
                    "\"redaction_summary\":\"{}\",",
                    "\"contains_raw_secret\":{},",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&event.event_id),
                json_escape(&event.event_type),
                json_escape(&event.service_id),
                json_escape(&event.trace_id),
                json_escape(&event.stack_profile),
                json_escape(&event.fixture_version),
                json_escape(&event.schema_version),
                json_escape(&event.artifact_ref),
                json_escape(&event.redaction_summary),
                event.contains_raw_secret,
                event.local_only,
                event.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_reset_safety_checks_json(checks: &[LocalResetSafetyCheck]) -> String {
    let rendered = checks
        .iter()
        .map(|check| {
            format!(
                concat!(
                    "{{",
                    "\"check_id\":\"{}\",",
                    "\"target_ref\":\"{}\",",
                    "\"marker_ref\":\"{}\",",
                    "\"marker_state\":\"{}\",",
                    "\"planned_action\":\"{}\",",
                    "\"deletion_plan_ref\":\"{}\",",
                    "\"safe_to_delete\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&check.check_id),
                json_escape(&check.target_ref),
                json_escape(&check.marker_ref),
                json_escape(&check.marker_state),
                json_escape(&check.planned_action),
                json_escape(&check.deletion_plan_ref),
                check.safe_to_delete,
                json_escape(&check.reason_code),
                check.local_only,
                check.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_seed_fixture_records_json(records: &[LocalSeedFixtureRecord]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"fixture_id\":\"{}\",",
                    "\"fixture_kind\":\"{}\",",
                    "\"fixture_ref\":\"{}\",",
                    "\"stable_id\":\"{}\",",
                    "\"apply_order\":{},",
                    "\"fixture_version\":\"{}\",",
                    "\"deterministic_seed\":\"{}\",",
                    "\"profile_binding\":\"{}\",",
                    "\"schema_version\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(record.fixture_id),
                json_escape(record.fixture_kind),
                json_escape(record.fixture_ref),
                json_escape(record.stable_id),
                record.apply_order,
                json_escape(record.fixture_version),
                json_escape(record.deterministic_seed),
                json_escape(record.profile_binding),
                json_escape(record.schema_version),
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_fixture_isolation_checks_json(checks: &[LocalFixtureIsolationCheck]) -> String {
    let rendered = checks
        .iter()
        .map(|check| {
            format!(
                concat!(
                    "{{",
                    "\"check_id\":\"{}\",",
                    "\"fixture_ref\":\"{}\",",
                    "\"credential_ref\":\"{}\",",
                    "\"profile_class\":\"{}\",",
                    "\"bound_profile\":\"{}\",",
                    "\"accepted\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"contains_raw_secret\":{},",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&check.check_id),
                json_escape(&check.fixture_ref),
                json_escape(&check.credential_ref),
                json_escape(&check.profile_class),
                json_escape(&check.bound_profile),
                check.accepted,
                json_escape(&check.reason_code),
                check.contains_raw_secret,
                check.local_only,
                check.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_phase1_seed_prerequisites_json(records: &[LocalControlPlaneSeedPrerequisite]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"prerequisite_id\":\"{}\",",
                    "\"prerequisite_kind\":\"{}\",",
                    "\"fixture_ref\":\"{}\",",
                    "\"phase_gate\":\"{}\",",
                    "\"stable_ref\":\"{}\",",
                    "\"signing_required\":{},",
                    "\"id_preserved_for_phase1\":{},",
                    "\"bypasses_signing\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(record.prerequisite_id),
                json_escape(record.prerequisite_kind),
                json_escape(record.fixture_ref),
                json_escape(record.phase_gate),
                json_escape(record.stable_ref),
                record.signing_required,
                record.id_preserved_for_phase1,
                record.bypasses_signing,
                json_escape(record.reason_code),
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_fixture_drift_reports_json(reports: &[LocalFixtureDriftReport]) -> String {
    let rendered = reports
        .iter()
        .map(|report| {
            format!(
                concat!(
                    "{{",
                    "\"report_id\":\"{}\",",
                    "\"fixture_ref\":\"{}\",",
                    "\"diff_field\":\"{}\",",
                    "\"expected_ref\":\"{}\",",
                    "\"actual_ref\":\"{}\",",
                    "\"drift_detected\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"blocks_seed\":{},",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&report.report_id),
                json_escape(&report.fixture_ref),
                json_escape(&report.diff_field),
                json_escape(&report.expected_ref),
                json_escape(&report.actual_ref),
                report.drift_detected,
                json_escape(&report.reason_code),
                report.blocks_seed,
                report.local_only,
                report.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_node_simulator_records_json(records: &[LocalNodeSimulatorRecord]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"simulator_id\":\"{}\",",
                    "\"service_id\":\"{}\",",
                    "\"deterministic_identity_ref\":\"{}\",",
                    "\"heartbeat_ref\":\"{}\",",
                    "\"capability_report_ref\":\"{}\",",
                    "\"health_endpoint_ref\":\"{}\",",
                    "\"accepted_fixture_workload_ref\":\"{}\",",
                    "\"noop_handoff_ref\":\"{}\",",
                    "\"real_hardware_discovery\":{},",
                    "\"gpu_runtime_integration\":{},",
                    "\"benchmark_publication\":{},",
                    "\"installer_update_flow\":{},",
                    "\"remote_shell_behavior\":{},",
                    "\"provider_eligibility_decision\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&record.simulator_id),
                json_escape(&record.service_id),
                json_escape(&record.deterministic_identity_ref),
                json_escape(&record.heartbeat_ref),
                json_escape(&record.capability_report_ref),
                json_escape(&record.health_endpoint_ref),
                json_escape(&record.accepted_fixture_workload_ref),
                json_escape(&record.noop_handoff_ref),
                record.real_hardware_discovery,
                record.gpu_runtime_integration,
                record.benchmark_publication,
                record.installer_update_flow,
                record.remote_shell_behavior,
                record.provider_eligibility_decision,
                json_escape(&record.reason_code),
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_phase0_smoke_records_json(records: &[LocalPhase0SmokeRecord]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"smoke_id\":\"{}\",",
                    "\"signed_noop_command_ref\":\"{}\",",
                    "\"audit_event_ref\":\"{}\",",
                    "\"invalid_schema_ref\":\"{}\",",
                    "\"trace_id\":\"{}\",",
                    "\"fixture_state_ref\":\"{}\",",
                    "\"redacted_artifact_ref\":\"{}\",",
                    "\"signed_noop_admitted\":{},",
                    "\"audit_event_write_read\":{},",
                    "\"invalid_schema_denied\":{},",
                    "\"trace_id_propagated\":{},",
                    "\"fixture_state_inspected\":{},",
                    "\"public_local_api_only\":{},",
                    "\"generated_contracts_only\":{},",
                    "\"contains_raw_secret\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&record.smoke_id),
                json_escape(&record.signed_noop_command_ref),
                json_escape(&record.audit_event_ref),
                json_escape(&record.invalid_schema_ref),
                json_escape(&record.trace_id),
                json_escape(&record.fixture_state_ref),
                json_escape(&record.redacted_artifact_ref),
                record.signed_noop_admitted,
                record.audit_event_write_read,
                record.invalid_schema_denied,
                record.trace_id_propagated,
                record.fixture_state_inspected,
                record.public_local_api_only,
                record.generated_contracts_only,
                record.contains_raw_secret,
                json_escape(&record.reason_code),
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_harness_hook_records_json(records: &[LocalHarnessHookRecord]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"hook_id\":\"{}\",",
                    "\"command_surface\":\"{}\",",
                    "\"harness_method\":\"{}\",",
                    "\"evidence_ref\":\"{}\",",
                    "\"generated_contract_ref\":\"{}\",",
                    "\"required_for_phase0_smoke\":{},",
                    "\"public_local_api_only\":{},",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&record.hook_id),
                json_escape(&record.command_surface),
                json_escape(&record.harness_method),
                json_escape(&record.evidence_ref),
                json_escape(&record.generated_contract_ref),
                record.required_for_phase0_smoke,
                record.public_local_api_only,
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_cli_sdk_smoke_support_records_json(records: &[LocalSdkSmokeSupportRecord]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"support_id\":\"{}\",",
                    "\"client_surface\":\"{}\",",
                    "\"request_example_ref\":\"{}\",",
                    "\"generated_contract_ref\":\"{}\",",
                    "\"public_local_api_only\":{},",
                    "\"avoids_private_storage\":{},",
                    "\"avoids_simulator_internals\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&record.support_id),
                json_escape(&record.client_surface),
                json_escape(&record.request_example_ref),
                json_escape(&record.generated_contract_ref),
                record.public_local_api_only,
                record.avoids_private_storage,
                record.avoids_simulator_internals,
                json_escape(&record.reason_code),
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_simulator_expansion_rules_json(records: &[LocalSimulatorExpansionRule]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"rule_id\":\"{}\",",
                    "\"target_phase\":\"{}\",",
                    "\"owning_contract_ref\":\"{}\",",
                    "\"local_test_marker_required\":{},",
                    "\"production_contract_shape_required\":{},",
                    "\"phase0_responsibility_drift_allowed\":{},",
                    "\"blocks_without_owner\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&record.rule_id),
                json_escape(&record.target_phase),
                json_escape(&record.owning_contract_ref),
                record.local_test_marker_required,
                record.production_contract_shape_required,
                record.phase0_responsibility_drift_allowed,
                record.blocks_without_owner,
                json_escape(&record.reason_code),
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_redacted_log_exports_json(records: &[LocalRedactedLogExportRecord]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"stream_id\":\"{}\",",
                    "\"service_id\":\"{}\",",
                    "\"log_ref\":\"{}\",",
                    "\"bundle_section_ref\":\"{}\",",
                    "\"redacts_secrets\":{},",
                    "\"redacts_tokens\":{},",
                    "\"redacts_signatures\":{},",
                    "\"redacts_private_payloads\":{},",
                    "\"redacts_encrypted_content\":{},",
                    "\"export_blocked_until_secret_free\":{},",
                    "\"scanner_passed\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&record.stream_id),
                json_escape(&record.service_id),
                json_escape(&record.log_ref),
                json_escape(&record.bundle_section_ref),
                record.redacts_secrets,
                record.redacts_tokens,
                record.redacts_signatures,
                record.redacts_private_payloads,
                record.redacts_encrypted_content,
                record.export_blocked_until_secret_free,
                record.scanner_passed,
                json_escape(&record.reason_code),
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_diagnostic_artifact_bundles_json(
    records: &[LocalDiagnosticArtifactBundleRecord],
) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"bundle_id\":\"{}\",",
                    "\"bundle_ref\":\"{}\",",
                    "\"manifest_ref\":\"{}\",",
                    "\"health_snapshot_refs\":{},",
                    "\"stack_profile\":\"{}\",",
                    "\"schema_version\":\"{}\",",
                    "\"fixture_version\":\"{}\",",
                    "\"command_lifecycle_ref\":\"{}\",",
                    "\"trace_id\":\"{}\",",
                    "\"reason_code_ref\":\"{}\",",
                    "\"local_event_refs\":{},",
                    "\"queue_state_refs\":{},",
                    "\"object_refs\":{},",
                    "\"reproduction_command\":\"{}\",",
                    "\"retention_class\":\"{}\",",
                    "\"contains_raw_secret\":{},",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&record.bundle_id),
                json_escape(&record.bundle_ref),
                json_escape(&record.manifest_ref),
                json_owned_string_array(&record.health_snapshot_refs),
                json_escape(&record.stack_profile),
                json_escape(&record.schema_version),
                json_escape(&record.fixture_version),
                json_escape(&record.command_lifecycle_ref),
                json_escape(&record.trace_id),
                json_escape(&record.reason_code_ref),
                json_owned_string_array(&record.local_event_refs),
                json_owned_string_array(&record.queue_state_refs),
                json_owned_string_array(&record.object_refs),
                json_escape(&record.reproduction_command),
                json_escape(&record.retention_class),
                record.contains_raw_secret,
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_clean_checkout_ci_entries_json(
    records: &[LocalCleanCheckoutCiEntrypointRecord],
) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"entrypoint_id\":\"{}\",",
                    "\"runner_ref\":\"{}\",",
                    "\"os_family\":\"{}\",",
                    "\"arch\":\"{}\",",
                    "\"ubuntu_24_04_equivalent\":{},",
                    "\"repo_pinned_rust_toolchain\":{},",
                    "\"loopback_networking\":{},",
                    "\"cloud_credentials_allowed\":{},",
                    "\"external_database_allowed\":{},",
                    "\"external_queue_allowed\":{},",
                    "\"external_object_store_allowed\":{},",
                    "\"commands\":{},",
                    "\"allowed_outcomes\":{},",
                    "\"machine_readable_output\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&record.entrypoint_id),
                json_escape(&record.runner_ref),
                json_escape(&record.os_family),
                json_escape(&record.arch),
                record.ubuntu_24_04_equivalent,
                record.repo_pinned_rust_toolchain,
                record.loopback_networking,
                record.cloud_credentials_allowed,
                record.external_database_allowed,
                record.external_queue_allowed,
                record.external_object_store_allowed,
                json_owned_string_array(&record.commands),
                json_owned_string_array(&record.allowed_outcomes),
                record.machine_readable_output,
                json_escape(&record.reason_code),
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_flake_evidence_records_json(records: &[LocalFlakeEvidenceRecord]) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"evidence_id\":\"{}\",",
                    "\"trace_id\":\"{}\",",
                    "\"repeated_run_count\":{},",
                    "\"startup_timing_variance_ms\":{},",
                    "\"nondeterministic_fixture_ids\":{},",
                    "\"unstable_event_ordering\":{},",
                    "\"retry_count\":{},",
                    "\"health_timeout_class\":\"{}\",",
                    "\"tolerance_window_used\":{},",
                    "\"flake_detected\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"artifact_ref\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&record.evidence_id),
                json_escape(&record.trace_id),
                record.repeated_run_count,
                record.startup_timing_variance_ms,
                record.nondeterministic_fixture_ids,
                record.unstable_event_ordering,
                record.retry_count,
                json_escape(&record.health_timeout_class),
                record.tolerance_window_used,
                record.flake_detected,
                json_escape(&record.reason_code),
                json_escape(&record.artifact_ref),
                record.local_only,
                record.test_only,
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn render_artifact_retention_policies_json(
    records: &[LocalArtifactRetentionPolicyRecord],
) -> String {
    let rendered = records
        .iter()
        .map(|record| {
            format!(
                concat!(
                    "{{",
                    "\"policy_id\":\"{}\",",
                    "\"retention_class\":\"{}\",",
                    "\"compact_success_summary\":{},",
                    "\"retain_failure_bundle\":{},",
                    "\"failure_retention_days\":{},",
                    "\"prune_command_ref\":\"{}\",",
                    "\"requires_test_state_marker\":{},",
                    "\"deletes_unmarked_user_dirs\":{},",
                    "\"deletes_production_like_state\":{},",
                    "\"deletes_non_local_artifacts\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"local_only\":{},",
                    "\"test_only\":{}",
                    "}}"
                ),
                json_escape(&record.policy_id),
                json_escape(&record.retention_class),
                record.compact_success_summary,
                record.retain_failure_bundle,
                record.failure_retention_days,
                json_escape(&record.prune_command_ref),
                record.requires_test_state_marker,
                record.deletes_unmarked_user_dirs,
                record.deletes_production_like_state,
                record.deletes_non_local_artifacts,
                json_escape(&record.reason_code),
                record.local_only,
                record.test_only,
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
        "local_stack.health_timeout" => {
            "inspect bounded wait diagnostics and exported redacted startup logs"
        }
        "local_stack.required_service_failed" => {
            "inspect lifecycle startup graph and rollback artifact refs for the failed service"
        }
        "local_stack.degraded" => {
            "inspect optional service readiness before relying on non-required local tooling"
        }
        "local_stack.already_running" => {
            "inspect readiness checks if the existing local stack should be restarted"
        }
        "local_stack.port_conflict" => "free the reserved loopback port range 18080-18085",
        "local_stack.schema_version_incompatible" => {
            "reset local backing records and regenerate local-stack schema fixtures"
        }
        "local_stack.reset_unsafe_state" => {
            "restore local/test reset markers before deleting local stack state"
        }
        "local_stack.fixture_drift_detected" => {
            "regenerate deterministic local fixtures and compare drift report fields"
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
        assert_eq!(capabilities.len(), 7);
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
        assert!(capabilities.iter().any(|capability| {
            capability.service_id == "service:node_agent_simulator"
                && capability.phase_gate == "phase_0_foundation"
        }));
    }

    #[test]
    fn later_phase_capabilities_include_blocked_reasons() {
        let capabilities = capabilities_for_phase(2);
        assert!(capabilities
            .iter()
            .any(|capability| capability.reason_code == "phase.local_service_unavailable"));
        assert!(capabilities.iter().any(|capability| !capability.available));
        assert!(!capabilities.iter().any(|capability| {
            capability.service_id == "service:node_agent_simulator" && !capability.available
        }));
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
    fn phase5_state_store_supports_contract_lookup() {
        let store = LocalStateStore::default_local();
        assert!(store.all_overbase_shaped());
        assert!(store.schema_versions_compatible());
        assert_eq!(
            store.contract_ref("state:tenant:local_alpha"),
            Some("overbase://local_state/tenants/tenant:local:alpha")
        );
        assert_eq!(store.by_kind("tenant").len(), 1);
        assert_eq!(store.by_kind("schema_version").len(), 1);
        assert!(store.get("state:missing").is_none());
    }

    #[test]
    fn phase5_queue_table_reports_idempotency_and_terminal_sets() {
        let table = LocalQueueTable::default_local();
        let duplicate = table.submit_preview("idem:local_stack:phase5:smoke");
        assert!(!duplicate.accepted);
        assert_eq!(
            duplicate.existing_job_id,
            Some("job:local:phase5_smoke_pending")
        );
        assert_eq!(
            duplicate.reason_code,
            "local_stack.job_duplicate_idempotency_key"
        );

        let accepted = table.submit_preview("idem:local_stack:phase5:new");
        assert!(accepted.accepted);
        assert_eq!(accepted.existing_job_id, None);
        assert_eq!(accepted.reason_code, "local_stack.job_pending");

        assert_eq!(table.pending_jobs().len(), 1);
        assert_eq!(table.retry_jobs().len(), 1);
        assert_eq!(table.dead_letter_jobs().len(), 1);
        assert_eq!(table.terminal_jobs().len(), 2);
    }

    #[test]
    fn phase5_artifact_store_verifies_payload_hash() {
        let store = LocalArtifactStore::default_local();
        let artifact_ref = "artifact://local_stack/phase5/noop_payload";
        let manifest = store
            .manifest_for_ref(artifact_ref)
            .expect("phase 5 artifact manifest exists");
        assert_eq!(
            manifest.content_address,
            LocalArtifactStore::content_address_for_payload(local_artifact_payload())
        );
        assert!(store.verify_payload(artifact_ref, local_artifact_payload()));
        assert!(!store.verify_payload(artifact_ref, "tampered"));
        assert_eq!(store.resettable_manifests().len(), 1);
    }

    #[test]
    fn phase5_audit_store_filters_by_trace_service_and_window() {
        let store = LocalAuditEventStore::default_local();
        assert!(store.all_local_diagnostics_only());

        let queue_events = store.query(LocalAuditQueryFilter {
            trace_id: Some("trace_local_stack_phase5_dead_letter"),
            service_id: Some("service:overqueue_jobs"),
            time_window_ref: Some("time_window:local_stack_phase5_queue"),
        });
        assert_eq!(queue_events.len(), 1);
        assert_eq!(queue_events[0].event_type, "local_stack.job_dead_lettered");

        assert!(store
            .query(LocalAuditQueryFilter {
                trace_id: Some("trace_missing"),
                service_id: Some("service:overqueue_jobs"),
                time_window_ref: None,
            })
            .is_empty());
    }

    #[test]
    fn phase5_schema_report_blocks_start_seed_and_smoke() {
        let report = SchemaCompatibilityReport::for_profile("local-stale-local-state-schema");
        assert!(!report.is_compatible());
        assert_eq!(
            report.blocking_surfaces_for_command(DevCommand::Start),
            vec!["local_state"]
        );
        assert_eq!(
            report.blocking_surfaces_for_command(DevCommand::Seed),
            vec!["local_state"]
        );
        assert_eq!(
            report.blocking_surfaces_for_command(DevCommand::Smoke),
            vec!["local_state"]
        );
        assert!(report
            .blocking_surfaces_for_command(DevCommand::Status)
            .is_empty());
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
    fn phase5_schema_incompatibility_blocks_start_seed_and_smoke() {
        for command in [DevCommand::Start, DevCommand::Seed, DevCommand::Smoke] {
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
    fn phase6_startup_graph_orders_dependencies_and_events() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Start);
        assert!(output.is_ok());
        assert_eq!(output.reason_code, "local_stack.ready");
        assert_eq!(
            output
                .startup_graph
                .iter()
                .map(|step| step.service_id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "service:embedded_state",
                "service:overqueue_jobs",
                "service:overstore_stub",
                "service:event_audit",
                "service:api",
                "service:worker",
                "service:node_agent_simulator",
            ]
        );
        assert!(output
            .startup_graph
            .windows(2)
            .all(|window| window[0].dependency_order <= window[1].dependency_order));
        assert!(output
            .readiness_checks
            .iter()
            .all(|check| check.health_state == "ready"
                && check.readiness_state == "ready"
                && check.liveness_state == "ready"
                && !check.stale_schema));
        assert!(output.wait_policy.bounded);
        assert!(output.wait_policy.no_unbounded_sleep);
        assert_eq!(
            output.wait_policy.poll_interval_ms,
            DEFAULT_LIFECYCLE_POLL_INTERVAL_MS
        );
        assert!(output.lifecycle_events.iter().any(|event| {
            event.event_type == "local_stack.start_requested"
                && event.local_only
                && event.test_only
                && !event.contains_raw_secret
        }));
        assert!(output
            .result_json()
            .contains("\"lifecycle_phase_gate\":\"phase_6_lifecycle_orchestration\""));
    }

    #[test]
    fn phase6_already_running_start_is_verified_without_reset() {
        let mut options = test_options();
        options.profile = "local-already-running".to_owned();
        let output = LocalStackRunner::new(options).run(DevCommand::Start);
        assert!(output.is_ok());
        assert_eq!(output.reason_code, "local_stack.already_running");
        assert!(output
            .startup_graph
            .iter()
            .all(|step| step.startup_state == "already_running"));
        assert!(output.rollback_reports.is_empty());
    }

    #[test]
    fn phase6_optional_degraded_service_does_not_fail_required_stack() {
        let mut options = test_options();
        options.profile = "local-degraded-optional".to_owned();
        let output = LocalStackRunner::new(options).run(DevCommand::Start);
        assert!(output.is_ok());
        assert_eq!(output.status, LocalStackStatus::Degraded);
        assert_eq!(output.reason_code, "local_stack.degraded");
        assert!(output.startup_graph.iter().any(|step| {
            step.service_id == "service:developer_ui"
                && !step.required
                && step.startup_state == "degraded"
        }));
        assert!(output
            .readiness_checks
            .iter()
            .any(|check| check.service_id == "service:developer_ui"
                && check.readiness_state == "degraded"));
    }

    #[test]
    fn phase6_health_timeout_exports_logs_and_rolls_back_started_services() {
        let mut options = test_options();
        options.profile = "local-health-timeout".to_owned();
        options.timeout_ms = Some(4_500);
        options.poll_interval_ms = Some(250);
        let output = LocalStackRunner::new(options).run(DevCommand::Start);
        assert!(!output.is_ok());
        assert_eq!(
            output.reason_code,
            "local_stack.backing_services_unavailable"
        );
        assert!(output.lifecycle_strs().contains(&"collecting_artifacts"));
        assert!(output.lifecycle_strs().contains(&"failed"));
        assert_eq!(output.wait_policy.reason_code, "local_stack.health_timeout");
        assert!(output.wait_policy.logs_exported_on_timeout);
        assert_eq!(output.wait_policy.timeout_ms, 4_500);
        assert!(output
            .startup_graph
            .iter()
            .any(|step| step.service_id == "service:api"
                && step.startup_state == "timeout"
                && step.reason_code == "local_stack.health_timeout"));
        assert!(output
            .startup_graph
            .iter()
            .any(|step| step.service_id == "service:worker"
                && step.startup_state == "not_started"
                && step.reason_code == "local_stack.dependency_wait_blocked"));
        assert!(!output.rollback_reports.is_empty());
        assert!(output
            .artifact_refs
            .iter()
            .any(|reference| reference.starts_with("log://local_stack/startup_failure/")));
        assert!(output
            .error_json()
            .contains("\"logs_exported_on_timeout\":true"));
    }

    #[test]
    fn phase6_restart_reports_reverse_shutdown_order() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Restart);
        assert!(output.is_ok());
        assert!(!output.shutdown_reports.is_empty());
        assert_eq!(
            output
                .shutdown_reports
                .iter()
                .map(|report| report.service_id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "service:node_agent_simulator",
                "service:worker",
                "service:api",
                "service:event_audit",
                "service:overstore_stub",
                "service:overqueue_jobs",
                "service:embedded_state",
            ]
        );
        assert!(output
            .shutdown_reports
            .iter()
            .all(|report| report.graceful && report.state_preserved));
        assert!(output
            .lifecycle_events
            .iter()
            .any(|event| event.event_type == "local_stack.restart_requested"));
    }

    #[test]
    fn phase6_schema_block_marks_readiness_as_stale_schema() {
        let mut options = test_options();
        options.profile = "local-stale-local-state-schema".to_owned();
        let output = LocalStackRunner::new(options).run(DevCommand::Start);
        assert!(!output.is_ok());
        assert_eq!(
            output.reason_code,
            "local_stack.schema_version_incompatible"
        );
        assert!(output
            .readiness_checks
            .iter()
            .all(|check| check.stale_schema
                && check.readiness_state == "stale_schema"
                && check.reason_code == "local_stack.schema_version_incompatible"));
        assert!(output
            .error_json()
            .contains("\"lifecycle_phase_gate\":\"phase_6_lifecycle_orchestration\""));
    }

    #[test]
    fn phase7_reset_safety_reports_marker_backed_deletion_plan() {
        let mut options = test_options();
        options.dry_run = true;
        let output = LocalStackRunner::new(options).run(DevCommand::Reset);
        assert!(output.is_ok());
        assert_eq!(output.reason_code, "local_stack.reset_completed");
        assert!(!output.reset_safety_checks.is_empty());
        assert!(output.reset_safety_checks.iter().all(|check| {
            check.safe_to_delete
                && check.marker_state == "verified"
                && check.planned_action == "dry_run_delete"
                && check.local_only
                && check.test_only
                && check.marker_ref.contains(".overrid-local-test-state")
        }));
        assert!(output
            .artifact_refs
            .iter()
            .any(|reference| reference.starts_with("artifact://local_stack/reset/")));
        assert!(output
            .result_json()
            .contains("\"fixture_phase_gate\":\"phase_7_reset_seed_fixtures\""));
    }

    #[test]
    fn phase7_reset_aborts_when_markers_are_missing_stale_or_inconsistent() {
        for (profile, marker_state) in [
            ("local-missing-reset-marker", "missing"),
            ("local-stale-reset-marker", "stale"),
            ("local-inconsistent-reset-marker", "inconsistent"),
        ] {
            let mut options = test_options();
            options.profile = profile.to_owned();
            let output = LocalStackRunner::new(options).run(DevCommand::Reset);
            assert!(!output.is_ok(), "{profile} should fail closed");
            assert_eq!(output.reason_code, "local_stack.reset_unsafe_state");
            assert_eq!(output.exit_class, ExitCodeClass::Config);
            assert!(output.lifecycle_strs().contains(&"blocked"));
            assert!(output
                .reset_safety_checks
                .iter()
                .any(|check| { !check.safe_to_delete && check.marker_state == marker_state }));
            assert!(output.error_json().contains("\"reset_safety_checks\""));
        }
    }

    #[test]
    fn phase7_seed_fixtures_are_deterministic_and_ordered() {
        let first = LocalStackRunner::new(test_options()).run(DevCommand::Seed);
        let second = LocalStackRunner::new(test_options()).run(DevCommand::Seed);
        assert!(first.is_ok());
        assert!(second.is_ok());
        assert_eq!(first.seed_fixture_records, second.seed_fixture_records);
        assert_eq!(
            first
                .seed_fixture_records
                .iter()
                .map(|fixture| fixture.apply_order)
                .collect::<Vec<_>>(),
            (1_u8..=10).collect::<Vec<_>>()
        );
        let kinds = first
            .seed_fixture_records
            .iter()
            .map(|fixture| fixture.fixture_kind)
            .collect::<BTreeSet<_>>();
        for expected in [
            "tenant",
            "actor",
            "key",
            "node",
            "manifest",
            "package",
            "workload",
            "oru_account",
            "seal_ledger_ref",
            "policy_context",
        ] {
            assert!(kinds.contains(expected));
        }
        assert!(first.seed_fixture_records.iter().all(|fixture| {
            fixture.fixture_version == LOCAL_STACK_PHASE7_FIXTURE_VERSION
                && fixture.deterministic_seed == LOCAL_STACK_PHASE7_DETERMINISTIC_SEED
                && fixture.schema_version == SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION
                && fixture.local_only
                && fixture.test_only
        }));
    }

    #[test]
    fn phase7_fixture_isolation_rejects_non_local_profile_classes() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        for rejected in [
            "seed",
            "staging",
            "production_like",
            "federation",
            "public_provider",
            "non_local",
        ] {
            assert!(output.fixture_isolation_checks.iter().any(|check| {
                check.profile_class == rejected
                    && !check.accepted
                    && check.reason_code == "profile.not_local_test"
                    && !check.contains_raw_secret
            }));
        }
        assert!(output
            .fixture_isolation_checks
            .iter()
            .any(|check| { check.profile_class == "local" && check.accepted }));
        assert!(output
            .fixture_isolation_checks
            .iter()
            .any(|check| { check.profile_class == "ci" && check.accepted }));

        for profile in [
            "seed",
            "staging",
            "production_like",
            "federation",
            "public-provider",
            "non-local",
        ] {
            let mut options = test_options();
            options.profile = profile.to_owned();
            let blocked = LocalStackRunner::new(options).run(DevCommand::Seed);
            assert!(!blocked.is_ok(), "{profile} should be rejected");
            assert_eq!(blocked.reason_code, "profile.not_local_test");
        }
    }

    #[test]
    fn phase7_phase1_seed_prerequisites_require_signing_and_preserve_ids() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Seed);
        assert!(output.is_ok());
        let kinds = output
            .phase1_seed_prerequisites
            .iter()
            .map(|record| record.prerequisite_kind)
            .collect::<BTreeSet<_>>();
        for expected in [
            "tenant",
            "actor",
            "key",
            "manifest",
            "idempotency_key",
            "trace_root",
            "pending_work_target",
        ] {
            assert!(kinds.contains(expected));
        }
        assert!(output.phase1_seed_prerequisites.iter().all(|record| {
            record.phase_gate == "local_smoke_prerequisite"
                && record.signing_required
                && record.id_preserved_for_phase1
                && !record.bypasses_signing
                && record.local_only
                && record.test_only
        }));
    }

    #[test]
    fn phase7_fixture_drift_blocks_seed_and_reports_stable_fields() {
        let clean = LocalStackRunner::new(test_options()).run(DevCommand::Seed);
        assert!(clean.is_ok());
        assert!(clean
            .fixture_drift_reports
            .iter()
            .all(|report| !report.drift_detected && !report.blocks_seed));

        for (profile, reason_code) in [
            (
                "local-nondeterministic-fixture-ids",
                "local_stack.fixture_id_drift",
            ),
            (
                "local-fixture-schema-drift",
                "local_stack.fixture_schema_drift",
            ),
            (
                "local-missing-fixture-ref",
                "local_stack.fixture_ref_missing",
            ),
            (
                "local-extra-fixture-record",
                "local_stack.fixture_extra_record",
            ),
        ] {
            let mut options = test_options();
            options.profile = profile.to_owned();
            let output = LocalStackRunner::new(options).run(DevCommand::Seed);
            assert!(!output.is_ok(), "{profile} should block seed");
            assert_eq!(output.reason_code, "local_stack.fixture_drift_detected");
            assert_eq!(output.exit_class, ExitCodeClass::Schema);
            assert!(output.fixture_drift_reports.iter().any(|report| {
                report.drift_detected
                    && report.blocks_seed
                    && report.reason_code == reason_code
                    && report.expected_ref != report.actual_ref
            }));
            assert!(output.error_json().contains("\"fixture_drift_reports\""));
        }
    }

    #[test]
    fn phase8_node_simulator_is_local_overcell_like_and_denies_provider_behaviors() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        let record = output
            .node_simulator_records
            .first()
            .expect("phase 8 node simulator record exists");
        assert_eq!(record.service_id, "service:node_agent_simulator");
        assert_eq!(
            record.reason_code,
            "local_stack.node_simulator_fixture_ready"
        );
        assert!(record
            .health_endpoint_ref
            .starts_with("http://127.0.0.1:18082/"));
        assert!(record.local_only && record.test_only);
        assert!(!record.real_hardware_discovery);
        assert!(!record.gpu_runtime_integration);
        assert!(!record.benchmark_publication);
        assert!(!record.installer_update_flow);
        assert!(!record.remote_shell_behavior);
        assert!(!record.provider_eligibility_decision);
        assert!(output.service_health.iter().any(|health| {
            health.service_id == "service:node_agent_simulator"
                && health.loopback_only
                && health.port == Some(18082)
        }));
        assert!(output
            .result_json()
            .contains("\"smoke_phase_gate\":\"phase_8_node_simulator_smoke_harness\""));
    }

    #[test]
    fn phase8_smoke_path_proves_signed_command_audit_schema_trace_and_redaction() {
        let mut options = test_options();
        options.trace_id = "trace_phase8_smoke".to_owned();
        let output = LocalStackRunner::new(options).run(DevCommand::Smoke);
        assert!(output.is_ok());
        assert_eq!(output.reason_code, "local_stack.smoke_passed");
        let smoke = output
            .phase0_smoke_records
            .first()
            .expect("phase 8 smoke record exists");
        assert_eq!(smoke.trace_id, "trace_phase8_smoke");
        assert!(smoke.signed_noop_admitted);
        assert!(smoke.audit_event_write_read);
        assert!(smoke.invalid_schema_denied);
        assert!(smoke.trace_id_propagated);
        assert!(smoke.fixture_state_inspected);
        assert!(smoke.public_local_api_only);
        assert!(smoke.generated_contracts_only);
        assert!(!smoke.contains_raw_secret);
        assert!(smoke.redacted_artifact_ref.ends_with("/redacted_bundle"));
        assert!(output
            .artifact_refs
            .iter()
            .any(|reference| reference.ends_with("/redacted_bundle")));
        assert!(output
            .dependency_status_strs()
            .contains(&"phase0_smoke_path_verified"));
    }

    #[test]
    fn phase8_harness_hooks_cover_lifecycle_snapshots_events_and_artifacts() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        let hook_ids = output
            .harness_hook_records
            .iter()
            .map(|record| record.hook_id.as_str())
            .collect::<BTreeSet<_>>();
        for expected in [
            "harness_hook:start",
            "harness_hook:status",
            "harness_hook:reset",
            "harness_hook:seed",
            "harness_hook:smoke",
            "harness_hook:logs",
            "harness_hook:health_snapshots",
            "harness_hook:event_export",
            "harness_hook:artifact_collection",
        ] {
            assert!(hook_ids.contains(expected), "{expected} hook missing");
        }
        for (hook_id, harness_method) in [
            ("harness_hook:start", "LocalStackHarness::start_stack"),
            ("harness_hook:status", "LocalStackHarness::status_stack"),
            ("harness_hook:reset", "LocalStackHarness::reset_stack"),
            ("harness_hook:seed", "LocalStackHarness::seed_stack"),
            ("harness_hook:smoke", "LocalStackHarness::run_phase0_smoke"),
            ("harness_hook:logs", "LocalStackHarness::logs"),
            (
                "harness_hook:health_snapshots",
                "LocalStackHarness::health_snapshots",
            ),
            (
                "harness_hook:event_export",
                "LocalStackHarness::event_export",
            ),
            (
                "harness_hook:artifact_collection",
                "LocalStackHarness::artifact_collection",
            ),
        ] {
            let record = output
                .harness_hook_records
                .iter()
                .find(|record| record.hook_id == hook_id)
                .expect("phase 8 harness hook record exists");
            assert_eq!(record.harness_method, harness_method);
        }
        assert!(output.harness_hook_records.iter().all(|record| {
            record.required_for_phase0_smoke
                && record.public_local_api_only
                && record.local_only
                && record.test_only
                && record.generated_contract_ref.starts_with("contract://")
        }));
    }

    #[test]
    fn phase8_cli_and_sdk_smoke_support_use_public_contract_surfaces() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        let surfaces = output
            .cli_sdk_smoke_support_records
            .iter()
            .map(|record| record.client_surface.as_str())
            .collect::<BTreeSet<_>>();
        assert!(surfaces.contains("overrid-cli"));
        assert!(surfaces.contains("overrid-rust-sdk"));
        assert!(output.cli_sdk_smoke_support_records.iter().all(|record| {
            record.public_local_api_only
                && record.avoids_private_storage
                && record.avoids_simulator_internals
                && record.local_only
                && record.test_only
                && record.generated_contract_ref.starts_with("contract://")
        }));
    }

    #[test]
    fn phase8_simulator_expansion_rules_prevent_phase0_responsibility_drift() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        assert!(output.simulator_expansion_rules.len() >= 5);
        let target_phases = output
            .simulator_expansion_rules
            .iter()
            .map(|rule| rule.target_phase.as_str())
            .collect::<BTreeSet<_>>();
        for expected in [
            "phase_2_node_registration",
            "phase_3_execution_loop",
            "phase_4_policy",
            "phase_5_accounting",
            "phase_8_storage_namespace",
        ] {
            assert!(
                target_phases.contains(expected),
                "{expected} expansion rule missing"
            );
        }
        assert!(output.simulator_expansion_rules.iter().all(|rule| {
            rule.local_test_marker_required
                && rule.production_contract_shape_required
                && !rule.phase0_responsibility_drift_allowed
                && rule.blocks_without_owner
                && rule.local_only
                && rule.test_only
                && rule.owning_contract_ref.starts_with("contract://")
        }));
        assert!(output
            .dependency_status_strs()
            .contains(&"simulator_expansion_rules_enforced"));
    }

    #[test]
    fn phase9_redacted_log_exports_cover_local_surfaces_and_sensitive_material() {
        let mut options = test_options();
        options.trace_id = "trace_phase9_logs".to_owned();
        let output = LocalStackRunner::new(options).run(DevCommand::Logs);
        assert!(output.is_ok());
        let service_ids = output
            .redacted_log_exports
            .iter()
            .map(|record| record.service_id.as_str())
            .collect::<BTreeSet<_>>();
        for expected in [
            "service:api",
            "service:worker",
            "service:embedded_state",
            "service:overqueue_jobs",
            "service:overstore_stub",
            "service:event_audit",
            "service:node_agent_simulator",
        ] {
            assert!(
                service_ids.contains(expected),
                "{expected} log export missing"
            );
        }
        assert!(output.redacted_log_exports.iter().all(|record| {
            record.redacts_secrets
                && record.redacts_tokens
                && record.redacts_signatures
                && record.redacts_private_payloads
                && record.redacts_encrypted_content
                && record.export_blocked_until_secret_free
                && record.scanner_passed
                && record.local_only
                && record.test_only
        }));
        assert!(output.artifact_refs.iter().any(|reference| {
            reference == "log://local_stack/node_agent_simulator/trace_phase9_logs/redacted"
        }));
        assert!(output
            .result_json()
            .contains("\"diagnostics_phase_gate\":\"phase_9_diagnostics_artifacts_ci_flake\""));
    }

    #[test]
    fn phase9_diagnostic_bundle_captures_reproduction_context_without_secrets() {
        let mut options = test_options();
        options.trace_id = "trace_phase9_bundle".to_owned();
        let output = LocalStackRunner::new(options).run(DevCommand::Smoke);
        assert!(output.is_ok());
        let bundle = output
            .diagnostic_artifact_bundles
            .first()
            .expect("phase 9 diagnostic bundle record exists");
        assert_eq!(bundle.trace_id, "trace_phase9_bundle");
        assert_eq!(
            bundle.schema_version,
            SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION
        );
        assert_eq!(
            bundle.fixture_version,
            LOCAL_STACK_PHASE8_SMOKE_FIXTURE_VERSION
        );
        assert_eq!(bundle.retention_class, "failure_evidence");
        assert!(!bundle.contains_raw_secret);
        assert!(bundle.reproduction_command.contains("overrid dev smoke"));
        assert!(!bundle.health_snapshot_refs.is_empty());
        assert!(!bundle.local_event_refs.is_empty());
        assert!(!bundle.queue_state_refs.is_empty());
        assert!(!bundle.object_refs.is_empty());
        assert!(output
            .artifact_refs
            .iter()
            .any(|reference| reference == &bundle.bundle_ref));
    }

    #[test]
    fn phase9_clean_checkout_ci_entrypoint_is_loopback_only_and_reproducible() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        let entry = output
            .clean_checkout_ci_entries
            .first()
            .expect("phase 9 CI entrypoint record exists");
        assert_eq!(entry.runner_ref, LOCAL_STACK_PHASE9_CI_RUNNER_REF);
        assert_eq!(entry.os_family, "linux");
        assert_eq!(entry.arch, "x86_64");
        assert!(entry.ubuntu_24_04_equivalent);
        assert!(entry.repo_pinned_rust_toolchain);
        assert!(entry.loopback_networking);
        assert!(!entry.cloud_credentials_allowed);
        assert!(!entry.external_database_allowed);
        assert!(!entry.external_queue_allowed);
        assert!(!entry.external_object_store_allowed);
        for expected in [
            "dev:start",
            "dev:reset",
            "dev:seed",
            "dev:smoke",
            "schema:check",
            "layout:check",
            "docs:check",
            "harness:smoke",
        ] {
            assert!(entry.commands.iter().any(|command| command == expected));
        }
        for expected in ["success", "blocked", "failed"] {
            assert!(entry
                .allowed_outcomes
                .iter()
                .any(|outcome| outcome == expected));
        }
    }

    #[test]
    fn phase9_flake_evidence_records_repeat_variance_retries_and_timeout_classes() {
        let mut options = test_options();
        options.profile =
            "local-nondeterministic-fixture-ids-unstable-event-ordering-health-timeout".to_owned();
        options.trace_id = "trace_phase9_flake".to_owned();
        let output = LocalStackRunner::new(options).run(DevCommand::Status);
        let evidence = output
            .flake_evidence_records
            .first()
            .expect("phase 9 flake evidence record exists");
        assert!(evidence.flake_detected);
        assert!(evidence.nondeterministic_fixture_ids);
        assert!(evidence.unstable_event_ordering);
        assert_eq!(evidence.health_timeout_class, "required_service_timeout");
        assert!(evidence.repeated_run_count >= 3);
        assert!(evidence.startup_timing_variance_ms >= 1_000);
        assert!(evidence.retry_count > 0);
        assert_eq!(evidence.trace_id, "trace_phase9_flake");
        assert!(output.error_json().contains("\"flake_evidence_records\""));
    }

    #[test]
    fn phase9_retention_policy_prune_keeps_user_production_and_non_local_state_safe() {
        let output = LocalStackRunner::new(test_options()).run(DevCommand::Status);
        assert!(output.is_ok());
        assert!(output.artifact_retention_policies.len() >= 2);
        assert!(output.artifact_retention_policies.iter().any(|policy| {
            policy.retention_class == "success_summary" && policy.compact_success_summary
        }));
        assert!(output.artifact_retention_policies.iter().any(|policy| {
            policy.retention_class == "failure_evidence"
                && policy.retain_failure_bundle
                && policy.failure_retention_days >= 30
        }));
        assert!(output.artifact_retention_policies.iter().all(|policy| {
            policy.requires_test_state_marker
                && !policy.deletes_unmarked_user_dirs
                && !policy.deletes_production_like_state
                && !policy.deletes_non_local_artifacts
                && policy.local_only
                && policy.test_only
        }));
        assert!(output
            .dependency_status_strs()
            .contains(&"artifact_retention_policy_enforced"));
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
