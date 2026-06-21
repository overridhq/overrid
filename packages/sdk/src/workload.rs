use std::fmt;

use crate::{
    build_command, check_sdk_compatibility, decode_overgate_submission, negotiate_sdk_capability,
    phase5_signature_ref, prepare_overgate_submission, sign_request, OvergateEndpoint,
    SdkCapabilityDecision, SdkCommandBuildInput, SdkCommandClass, SdkCommandEnvelope,
    SdkCommandOutcome, SdkCommandPayload, SdkError, SdkOptionalHelper, SdkOvergateResponse,
    SdkOvergateSubmission, SdkPhase5Error, SdkServiceCapabilityProfile, SdkSignedOvergateRequest,
    SDK_CURRENT_STABLE_MAJOR, SDK_LANGUAGE_BINDING, SDK_NAME, SDK_PHASE4_COMMAND_ROUTE,
    SDK_VERSION,
};
use overrid_contracts::{BootstrapCommandFamily, SchemaVersion};

pub const SDK_PHASE6_CAPABILITY_PROFILE: &str = "phase6-workload-manifest-status-policy-helpers";
pub const SDK_PHASE6_WORKLOAD_COMMAND_TYPE: &str = "workload submit";
pub const SDK_PHASE6_POLICY_DRY_RUN_ROUTE: &str = "/v1/overgate/policy-dry-runs";
pub const SDK_PHASE6_WORKLOAD_STATUS_ROUTE: &str = "/v1/control-plane/workloads/status";
pub const SDK_PHASE6_JOB_STATUS_ROUTE: &str = "/v1/control-plane/jobs/status";
pub const SDK_PHASE6_COMMAND_STATUS_ROUTE: &str = "/v1/control-plane/commands/status";
pub const SDK_PHASE6_RESULT_ROUTE: &str = "/v1/control-plane/workloads/results";
pub const SDK_PHASE6_CANCELLATION_ROUTE: &str = "/v1/control-plane/workloads/cancellations";
pub const SDK_PHASE6_RUNTIME_AUTHORITY_OWNERS: &[&str] = &[
    "Overgate",
    "Overguard",
    "Overqueue",
    "Overrun",
    "Overmeter",
    "Overwatch",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkWorkloadClass {
    CpuBatch,
    GpuBatch,
    WasmComponent,
    ContainerTask,
}

impl SdkWorkloadClass {
    pub fn parse(raw: &str) -> Result<Self, SdkPhase6Error> {
        match raw {
            "cpu_batch" => Ok(Self::CpuBatch),
            "gpu_batch" => Ok(Self::GpuBatch),
            "wasm_component" => Ok(Self::WasmComponent),
            "container_task" => Ok(Self::ContainerTask),
            other => Err(SdkPhase6Error::UnsupportedWorkloadClass(other.to_owned())),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CpuBatch => "cpu_batch",
            Self::GpuBatch => "gpu_batch",
            Self::WasmComponent => "wasm_component",
            Self::ContainerTask => "container_task",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkResourceKind {
    Cpu,
    Memory,
    Gpu,
    Storage,
}

impl SdkResourceKind {
    pub fn parse(raw: &str) -> Result<Self, SdkPhase6Error> {
        match raw {
            "cpu" => Ok(Self::Cpu),
            "memory" => Ok(Self::Memory),
            "gpu" => Ok(Self::Gpu),
            "storage" => Ok(Self::Storage),
            other => Err(SdkPhase6Error::UnsupportedResourceKind(other.to_owned())),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Memory => "memory",
            Self::Gpu => "gpu",
            Self::Storage => "storage",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkResourceDeclaration {
    pub name: String,
    pub kind: SdkResourceKind,
    pub amount: u64,
    pub unit: String,
}

impl SdkResourceDeclaration {
    pub fn new(
        name: impl Into<String>,
        kind: impl AsRef<str>,
        amount: u64,
        unit: impl Into<String>,
    ) -> Result<Self, SdkPhase6Error> {
        let name = name.into();
        let unit = unit.into();
        require_phase6_non_empty(&name, "resource name")?;
        require_phase6_non_empty(&unit, "resource unit")?;
        if amount == 0 {
            return Err(SdkPhase6Error::InvalidResourceAmount(name));
        }
        Ok(Self {
            name,
            kind: SdkResourceKind::parse(kind.as_ref())?,
            amount,
            unit,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkDataDeclaration {
    pub data_ref: String,
    pub access_mode: String,
}

impl SdkDataDeclaration {
    pub fn new(
        data_ref: impl Into<String>,
        access_mode: impl Into<String>,
    ) -> Result<Self, SdkPhase6Error> {
        let data_ref = data_ref.into();
        let access_mode = access_mode.into();
        require_phase6_non_empty(&data_ref, "data ref")?;
        require_phase6_non_empty(&access_mode, "data access mode")?;
        Ok(Self {
            data_ref,
            access_mode,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPolicyDeclaration {
    pub policy_ref: String,
    pub required: bool,
}

impl SdkPolicyDeclaration {
    pub fn new(policy_ref: impl Into<String>, required: bool) -> Result<Self, SdkPhase6Error> {
        let policy_ref = policy_ref.into();
        require_phase6_non_empty(&policy_ref, "policy ref")?;
        Ok(Self {
            policy_ref,
            required,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkEgressMode {
    DenyAll,
    Allowlist,
}

impl SdkEgressMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DenyAll => "deny_all",
            Self::Allowlist => "allowlist",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkEgressDeclaration {
    pub mode: SdkEgressMode,
    pub destinations: Vec<String>,
}

impl SdkEgressDeclaration {
    pub fn deny_all() -> Self {
        Self {
            mode: SdkEgressMode::DenyAll,
            destinations: vec![],
        }
    }

    pub fn allowlist(destinations: Vec<String>) -> Result<Self, SdkPhase6Error> {
        if destinations.is_empty() {
            return Err(SdkPhase6Error::ForbiddenEgress(
                "empty allowlist".to_owned(),
            ));
        }
        for destination in &destinations {
            validate_egress_destination(destination)?;
        }
        Ok(Self {
            mode: SdkEgressMode::Allowlist,
            destinations,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkOutputDeclaration {
    pub output_ref: String,
    pub content_type: String,
    pub retention_class: String,
}

impl SdkOutputDeclaration {
    pub fn new(
        output_ref: impl Into<String>,
        content_type: impl Into<String>,
        retention_class: impl Into<String>,
    ) -> Result<Self, SdkPhase6Error> {
        let output_ref = output_ref.into();
        let content_type = content_type.into();
        let retention_class = retention_class.into();
        require_phase6_non_empty(&output_ref, "output ref")?;
        require_phase6_non_empty(&content_type, "output content type")?;
        require_phase6_non_empty(&retention_class, "output retention class")?;
        Ok(Self {
            output_ref,
            content_type,
            retention_class,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkSecretReferenceDeclaration {
    pub secret_ref: String,
    pub mount_intent: String,
    pub required: bool,
}

impl SdkSecretReferenceDeclaration {
    pub fn new(
        secret_ref: impl Into<String>,
        mount_intent: impl Into<String>,
        required: bool,
    ) -> Result<Self, SdkPhase6Error> {
        let secret_ref = secret_ref.into();
        let mount_intent = mount_intent.into();
        validate_secret_ref(&secret_ref)?;
        require_phase6_non_empty(&mount_intent, "secret mount intent")?;
        Ok(Self {
            secret_ref,
            mount_intent,
            required,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkWorkloadManifestInput {
    pub workload_id: String,
    pub workload_class: String,
    pub schema_version: String,
    pub resources: Vec<SdkResourceDeclaration>,
    pub data_refs: Vec<SdkDataDeclaration>,
    pub policy_refs: Vec<SdkPolicyDeclaration>,
    pub egress: SdkEgressDeclaration,
    pub output: SdkOutputDeclaration,
    pub secret_refs: Vec<SdkSecretReferenceDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkWorkloadManifestValidation {
    pub valid_local_shape: bool,
    pub runtime_acceptance_claimed: bool,
    pub checked_fields: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkWorkloadManifest {
    pub manifest_ref: String,
    pub workload_id: String,
    pub workload_class: SdkWorkloadClass,
    pub schema_version: SchemaVersion,
    pub resources: Vec<SdkResourceDeclaration>,
    pub data_refs: Vec<SdkDataDeclaration>,
    pub policy_refs: Vec<SdkPolicyDeclaration>,
    pub egress: SdkEgressDeclaration,
    pub output: SdkOutputDeclaration,
    pub secret_refs: Vec<SdkSecretReferenceDeclaration>,
    pub validation: SdkWorkloadManifestValidation,
}

impl SdkWorkloadManifest {
    pub fn command_payload(&self) -> Result<SdkCommandPayload, SdkPhase6Error> {
        SdkCommandPayload::new("workload.manifest", self.command_payload_fields())
            .map_err(SdkPhase6Error::Sdk)
    }

    pub fn command_payload_fields(&self) -> Vec<(String, String)> {
        let mut fields = vec![
            ("manifest_ref".to_owned(), self.manifest_ref.clone()),
            ("workload_id".to_owned(), self.workload_id.clone()),
            (
                "workload_class".to_owned(),
                self.workload_class.as_str().to_owned(),
            ),
            (
                "schema_version".to_owned(),
                self.schema_version.raw().to_owned(),
            ),
            ("output_ref".to_owned(), self.output.output_ref.clone()),
            (
                "egress_mode".to_owned(),
                self.egress.mode.as_str().to_owned(),
            ),
            (
                "runtime_acceptance_claimed".to_owned(),
                self.validation.runtime_acceptance_claimed.to_string(),
            ),
        ];
        for resource in &self.resources {
            fields.push((
                format!("resource:{}", resource.name),
                format!(
                    "{}:{}:{}",
                    resource.kind.as_str(),
                    resource.amount,
                    resource.unit
                ),
            ));
        }
        for data_ref in &self.data_refs {
            fields.push(("data_ref".to_owned(), data_ref.data_ref.clone()));
            fields.push((
                format!("data_access:{}", data_ref.data_ref),
                data_ref.access_mode.clone(),
            ));
        }
        for policy_ref in &self.policy_refs {
            fields.push(("policy_ref".to_owned(), policy_ref.policy_ref.clone()));
            fields.push((
                format!("policy_required:{}", policy_ref.policy_ref),
                policy_ref.required.to_string(),
            ));
        }
        for secret_ref in &self.secret_refs {
            fields.push(("secret_ref".to_owned(), secret_ref.secret_ref.clone()));
        }
        fields
    }
}

pub fn build_workload_manifest(
    input: SdkWorkloadManifestInput,
) -> Result<SdkWorkloadManifest, SdkPhase6Error> {
    require_phase6_non_empty(&input.workload_id, "workload id")?;
    require_phase6_non_empty(&input.schema_version, "schema version")?;
    let schema_version = check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, &input.schema_version)?;
    let workload_class = SdkWorkloadClass::parse(&input.workload_class)?;
    if input.resources.is_empty() {
        return Err(SdkPhase6Error::MissingField("resource declarations"));
    }
    validate_data_declarations(&input.data_refs)?;
    validate_policy_declarations(&input.policy_refs)?;
    validate_egress(&input.egress)?;

    Ok(SdkWorkloadManifest {
        manifest_ref: format!(
            "manifest:{}:{}",
            workload_class.as_str(),
            stable_phase6_hash(&input.workload_id)
        ),
        workload_id: input.workload_id,
        workload_class,
        schema_version,
        resources: input.resources,
        data_refs: input.data_refs,
        policy_refs: input.policy_refs,
        egress: input.egress,
        output: input.output,
        secret_refs: input.secret_refs,
        validation: SdkWorkloadManifestValidation {
            valid_local_shape: true,
            runtime_acceptance_claimed: false,
            checked_fields: vec![
                "workload",
                "resources",
                "data",
                "policy",
                "egress",
                "output",
                "secret_refs",
                "schema_version",
            ],
        },
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkWorkloadSubmissionInput {
    pub endpoint: OvergateEndpoint,
    pub service_capability_profile: SdkServiceCapabilityProfile,
    pub manifest: SdkWorkloadManifest,
    pub provider: crate::SdkCredentialProvider,
    pub tenant_id: String,
    pub actor_id: String,
    pub idempotency_key: String,
    pub trace_id: String,
    pub timestamp_ms: u64,
    pub command_deadline_at_ms: Option<u64>,
    pub replay_window_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkWorkloadSubmission {
    pub manifest: SdkWorkloadManifest,
    pub command: SdkCommandEnvelope,
    pub signed_request: SdkSignedOvergateRequest,
    pub overgate_submission: SdkOvergateSubmission,
    pub capability: SdkCapabilityDecision,
    pub runtime_authority_claimed: bool,
}

pub fn submit_workload(
    input: SdkWorkloadSubmissionInput,
) -> Result<SdkWorkloadSubmission, SdkPhase6Error> {
    let capability = negotiate_sdk_capability(
        &input.service_capability_profile,
        SdkOptionalHelper::WorkloadSubmission,
        input.manifest.schema_version.raw(),
        SDK_CURRENT_STABLE_MAJOR,
    )?;
    require_phase6_non_empty(&input.tenant_id, "tenant id")?;
    require_phase6_non_empty(&input.actor_id, "actor id")?;
    require_phase6_non_empty(&input.idempotency_key, "idempotency key")?;
    require_phase6_non_empty(&input.trace_id, "trace id")?;
    if input.timestamp_ms == 0 {
        return Err(SdkPhase6Error::MissingField("timestamp"));
    }

    let signature_ref = phase5_signature_ref(&input.provider, SDK_PHASE6_WORKLOAD_COMMAND_TYPE)
        .map_err(SdkPhase6Error::Signing)?;
    let command = build_command(SdkCommandBuildInput {
        family: BootstrapCommandFamily::Workload,
        command_type: SDK_PHASE6_WORKLOAD_COMMAND_TYPE.to_owned(),
        tenant_id: input.tenant_id,
        actor_id: input.actor_id,
        target_ref: input.manifest.manifest_ref.clone(),
        payload: input.manifest.command_payload()?,
        expected_state: Some("pending_queue".to_owned()),
        reason: Some("phase6 workload submission helper".to_owned()),
        idempotency_key: input.idempotency_key,
        trace_id: input.trace_id,
        timestamp_ms: input.timestamp_ms,
        command_deadline_at_ms: input.command_deadline_at_ms,
        signature_ref,
        schema_version: input.manifest.schema_version.raw().to_owned(),
        command_class: SdkCommandClass::LongRunningWorkload,
    })?;
    let signed_request = sign_request(
        &command,
        &input.provider,
        "POST",
        SDK_PHASE4_COMMAND_ROUTE,
        input.replay_window_ms,
    )
    .map_err(SdkPhase6Error::Signing)?;
    let overgate_submission = prepare_overgate_submission(&input.endpoint, command.clone())?;

    Ok(SdkWorkloadSubmission {
        manifest: input.manifest,
        command,
        signed_request,
        overgate_submission,
        capability,
        runtime_authority_claimed: false,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkWorkloadSubmissionOutcome {
    pub command_outcome: SdkCommandOutcome,
    pub pending_queue_reached: bool,
    pub denied_reason_code: Option<String>,
    pub runtime_completion_invented: bool,
}

pub fn decode_workload_submission_response(
    submission: &SdkWorkloadSubmission,
    response: SdkOvergateResponse,
) -> Result<SdkWorkloadSubmissionOutcome, SdkPhase6Error> {
    let command_outcome = decode_overgate_submission(&submission.command, response)?;
    let pending_queue_reached = command_outcome
        .acceptance
        .as_ref()
        .is_some_and(|acceptance| acceptance.pending_state.contains("queue"));
    let denied_reason_code = command_outcome
        .error
        .as_ref()
        .map(|error| error.reason_code.clone());

    Ok(SdkWorkloadSubmissionOutcome {
        command_outcome,
        pending_queue_reached,
        denied_reason_code,
        runtime_completion_invented: false,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkWorkloadReadKind {
    CommandStatus,
    WorkloadStatus,
    JobStatus,
    Result,
    CancellationStatus,
}

impl SdkWorkloadReadKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CommandStatus => "command_status",
            Self::WorkloadStatus => "workload_status",
            Self::JobStatus => "job_status",
            Self::Result => "result",
            Self::CancellationStatus => "cancellation_status",
        }
    }

    pub fn route(self) -> &'static str {
        match self {
            Self::CommandStatus => SDK_PHASE6_COMMAND_STATUS_ROUTE,
            Self::WorkloadStatus => SDK_PHASE6_WORKLOAD_STATUS_ROUTE,
            Self::JobStatus => SDK_PHASE6_JOB_STATUS_ROUTE,
            Self::Result => SDK_PHASE6_RESULT_ROUTE,
            Self::CancellationStatus => SDK_PHASE6_CANCELLATION_ROUTE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkWorkloadReadRequest {
    pub endpoint: String,
    pub route: &'static str,
    pub read_kind: SdkWorkloadReadKind,
    pub object_ref: String,
    pub request_id: String,
    pub trace_id: String,
    pub schema_version: SchemaVersion,
    pub headers: Vec<(String, String)>,
    pub read_only: bool,
    pub public_control_plane_path: bool,
}

pub fn build_workload_read_request(
    endpoint: &OvergateEndpoint,
    read_kind: SdkWorkloadReadKind,
    object_ref: impl Into<String>,
    request_id: impl Into<String>,
    trace_id: impl Into<String>,
    schema_version: &str,
) -> Result<SdkWorkloadReadRequest, SdkPhase6Error> {
    let object_ref = object_ref.into();
    let request_id = request_id.into();
    let trace_id = trace_id.into();
    require_phase6_non_empty(&object_ref, "read object ref")?;
    require_phase6_non_empty(&request_id, "request id")?;
    require_phase6_non_empty(&trace_id, "trace id")?;
    let schema_version = check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, schema_version)?;

    Ok(SdkWorkloadReadRequest {
        endpoint: endpoint.raw().to_owned(),
        route: read_kind.route(),
        read_kind,
        object_ref,
        request_id: request_id.clone(),
        trace_id: trace_id.clone(),
        schema_version: schema_version.clone(),
        headers: phase6_headers(
            schema_version.raw(),
            &request_id,
            &trace_id,
            read_kind.route(),
            true,
        ),
        read_only: true,
        public_control_plane_path: true,
    })
}

pub fn build_command_status_request(
    endpoint: &OvergateEndpoint,
    command_ref: impl Into<String>,
    request_id: impl Into<String>,
    trace_id: impl Into<String>,
    schema_version: &str,
) -> Result<SdkWorkloadReadRequest, SdkPhase6Error> {
    build_workload_read_request(
        endpoint,
        SdkWorkloadReadKind::CommandStatus,
        command_ref,
        request_id,
        trace_id,
        schema_version,
    )
}

pub fn build_workload_status_request(
    endpoint: &OvergateEndpoint,
    workload_ref: impl Into<String>,
    request_id: impl Into<String>,
    trace_id: impl Into<String>,
    schema_version: &str,
) -> Result<SdkWorkloadReadRequest, SdkPhase6Error> {
    build_workload_read_request(
        endpoint,
        SdkWorkloadReadKind::WorkloadStatus,
        workload_ref,
        request_id,
        trace_id,
        schema_version,
    )
}

pub fn build_job_status_request(
    endpoint: &OvergateEndpoint,
    job_ref: impl Into<String>,
    request_id: impl Into<String>,
    trace_id: impl Into<String>,
    schema_version: &str,
) -> Result<SdkWorkloadReadRequest, SdkPhase6Error> {
    build_workload_read_request(
        endpoint,
        SdkWorkloadReadKind::JobStatus,
        job_ref,
        request_id,
        trace_id,
        schema_version,
    )
}

pub fn build_workload_result_request(
    endpoint: &OvergateEndpoint,
    workload_ref: impl Into<String>,
    request_id: impl Into<String>,
    trace_id: impl Into<String>,
    schema_version: &str,
) -> Result<SdkWorkloadReadRequest, SdkPhase6Error> {
    build_workload_read_request(
        endpoint,
        SdkWorkloadReadKind::Result,
        workload_ref,
        request_id,
        trace_id,
        schema_version,
    )
}

pub fn build_cancellation_status_request(
    endpoint: &OvergateEndpoint,
    cancellation_ref: impl Into<String>,
    request_id: impl Into<String>,
    trace_id: impl Into<String>,
    schema_version: &str,
) -> Result<SdkWorkloadReadRequest, SdkPhase6Error> {
    build_workload_read_request(
        endpoint,
        SdkWorkloadReadKind::CancellationStatus,
        cancellation_ref,
        request_id,
        trace_id,
        schema_version,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkWorkloadServiceState {
    PendingQueue,
    Accepted,
    Running,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
    Duplicate,
    DeadLetter,
}

impl SdkWorkloadServiceState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PendingQueue => "pending_queue",
            Self::Accepted => "accepted",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Duplicate => "duplicate",
            Self::DeadLetter => "dead_letter",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Failed
                | Self::Cancelled
                | Self::TimedOut
                | Self::Duplicate
                | Self::DeadLetter
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkWorkloadStatusInput {
    pub state: SdkWorkloadServiceState,
    pub service_reported: bool,
    pub trace_id: String,
    pub audit_refs: Vec<String>,
    pub queue_ref: Option<String>,
    pub result_ref: Option<String>,
    pub duplicate_of: Option<String>,
    pub dead_letter_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkWorkloadStatusRecord {
    pub state: SdkWorkloadServiceState,
    pub trace_id: String,
    pub audit_refs: Vec<String>,
    pub queue_ref: Option<String>,
    pub result_ref: Option<String>,
    pub duplicate_of: Option<String>,
    pub dead_letter_ref: Option<String>,
    pub service_reported: bool,
    pub sdk_invented_terminal_state: bool,
}

impl SdkWorkloadStatusRecord {
    pub fn from_service(input: SdkWorkloadStatusInput) -> Result<Self, SdkPhase6Error> {
        if !input.service_reported {
            return Err(SdkPhase6Error::ServiceEvidenceRequired(
                "workload status service response",
            ));
        }
        require_phase6_non_empty(&input.trace_id, "status trace id")?;
        if matches!(input.state, SdkWorkloadServiceState::Completed)
            && input
                .result_ref
                .as_ref()
                .is_none_or(|value| value.trim().is_empty())
        {
            return Err(SdkPhase6Error::MissingField("completed result ref"));
        }
        if matches!(input.state, SdkWorkloadServiceState::Duplicate)
            && input
                .duplicate_of
                .as_ref()
                .is_none_or(|value| value.trim().is_empty())
        {
            return Err(SdkPhase6Error::MissingField("duplicate ref"));
        }
        if matches!(input.state, SdkWorkloadServiceState::DeadLetter)
            && input
                .dead_letter_ref
                .as_ref()
                .is_none_or(|value| value.trim().is_empty())
        {
            return Err(SdkPhase6Error::MissingField("dead-letter ref"));
        }

        Ok(Self {
            state: input.state,
            trace_id: input.trace_id,
            audit_refs: input.audit_refs,
            queue_ref: input.queue_ref,
            result_ref: input.result_ref,
            duplicate_of: input.duplicate_of,
            dead_letter_ref: input.dead_letter_ref,
            service_reported: true,
            sdk_invented_terminal_state: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkWorkloadCancellationRequest {
    pub endpoint: String,
    pub route: &'static str,
    pub workload_ref: String,
    pub request_id: String,
    pub trace_id: String,
    pub reason: String,
    pub schema_version: SchemaVersion,
    pub headers: Vec<(String, String)>,
    pub public_control_plane_path: bool,
    pub requires_service_state_for_cancelled: bool,
}

pub fn build_workload_cancellation_request(
    endpoint: &OvergateEndpoint,
    workload_ref: impl Into<String>,
    request_id: impl Into<String>,
    trace_id: impl Into<String>,
    reason: impl Into<String>,
    schema_version: &str,
) -> Result<SdkWorkloadCancellationRequest, SdkPhase6Error> {
    let workload_ref = workload_ref.into();
    let request_id = request_id.into();
    let trace_id = trace_id.into();
    let reason = reason.into();
    require_phase6_non_empty(&workload_ref, "workload ref")?;
    require_phase6_non_empty(&request_id, "request id")?;
    require_phase6_non_empty(&trace_id, "trace id")?;
    require_phase6_non_empty(&reason, "cancellation reason")?;
    let schema_version = check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, schema_version)?;

    Ok(SdkWorkloadCancellationRequest {
        endpoint: endpoint.raw().to_owned(),
        route: SDK_PHASE6_CANCELLATION_ROUTE,
        workload_ref,
        request_id: request_id.clone(),
        trace_id: trace_id.clone(),
        reason,
        schema_version: schema_version.clone(),
        headers: phase6_headers(
            schema_version.raw(),
            &request_id,
            &trace_id,
            SDK_PHASE6_CANCELLATION_ROUTE,
            false,
        ),
        public_control_plane_path: true,
        requires_service_state_for_cancelled: true,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPolicyDryRunInput {
    pub manifest: SdkWorkloadManifest,
    pub request_id: String,
    pub trace_id: String,
    pub schema_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPolicyDryRunRequest {
    pub endpoint: String,
    pub route: &'static str,
    pub manifest_ref: String,
    pub request_id: String,
    pub trace_id: String,
    pub policy_refs: Vec<String>,
    pub schema_version: SchemaVersion,
    pub capability: SdkCapabilityDecision,
    pub headers: Vec<(String, String)>,
    pub mutates_runtime_state: bool,
    pub cacheable_as_policy_truth: bool,
    pub fail_closed: bool,
}

pub fn build_policy_dry_run_request(
    endpoint: &OvergateEndpoint,
    profile: &SdkServiceCapabilityProfile,
    input: SdkPolicyDryRunInput,
) -> Result<SdkPolicyDryRunRequest, SdkPhase6Error> {
    require_phase6_non_empty(&input.request_id, "request id")?;
    require_phase6_non_empty(&input.trace_id, "trace id")?;
    let schema_version = check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, &input.schema_version)?;
    let capability = negotiate_sdk_capability(
        profile,
        SdkOptionalHelper::PolicyDryRun,
        schema_version.raw(),
        SDK_CURRENT_STABLE_MAJOR,
    )?;
    Ok(SdkPolicyDryRunRequest {
        endpoint: endpoint.raw().to_owned(),
        route: SDK_PHASE6_POLICY_DRY_RUN_ROUTE,
        manifest_ref: input.manifest.manifest_ref,
        request_id: input.request_id.clone(),
        trace_id: input.trace_id.clone(),
        policy_refs: input
            .manifest
            .policy_refs
            .into_iter()
            .map(|policy| policy.policy_ref)
            .collect(),
        schema_version: schema_version.clone(),
        capability,
        headers: phase6_headers(
            schema_version.raw(),
            &input.request_id,
            &input.trace_id,
            SDK_PHASE6_POLICY_DRY_RUN_ROUTE,
            true,
        ),
        mutates_runtime_state: false,
        cacheable_as_policy_truth: false,
        fail_closed: true,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPolicyDryRunResultInput {
    pub service_reported: bool,
    pub matched_policy_refs: Vec<String>,
    pub reason_codes: Vec<String>,
    pub estimated_placement_class: String,
    pub correction_fields: Vec<String>,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPolicyDryRunResult {
    pub matched_policy_refs: Vec<String>,
    pub reason_codes: Vec<String>,
    pub estimated_placement_class: String,
    pub correction_fields: Vec<String>,
    pub trace_id: String,
    pub mutates_runtime_state: bool,
    pub cacheable_as_policy_truth: bool,
    pub service_reported: bool,
}

pub fn decode_policy_dry_run_result(
    input: SdkPolicyDryRunResultInput,
) -> Result<SdkPolicyDryRunResult, SdkPhase6Error> {
    if !input.service_reported {
        return Err(SdkPhase6Error::ServiceEvidenceRequired(
            "policy dry-run service response",
        ));
    }
    require_phase6_non_empty(&input.estimated_placement_class, "placement class")?;
    require_phase6_non_empty(&input.trace_id, "dry-run trace id")?;
    validate_ref_list("matched policy ref", &input.matched_policy_refs)?;
    validate_ref_list("reason code", &input.reason_codes)?;

    Ok(SdkPolicyDryRunResult {
        matched_policy_refs: input.matched_policy_refs,
        reason_codes: input.reason_codes,
        estimated_placement_class: input.estimated_placement_class,
        correction_fields: input.correction_fields,
        trace_id: input.trace_id,
        mutates_runtime_state: false,
        cacheable_as_policy_truth: false,
        service_reported: true,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase6AuthorityBoundary {
    pub helper: &'static str,
    pub approved_public_route: &'static str,
    pub overgate_or_public_control_plane_only: bool,
    pub wrapper_only: bool,
    pub owns_scheduler: bool,
    pub owns_policy_truth: bool,
    pub owns_storage_reader: bool,
    pub owns_metering_truth: bool,
    pub owning_services: &'static [&'static str],
}

pub fn sdk_phase6_authority_review() -> Vec<SdkPhase6AuthorityBoundary> {
    vec![
        SdkPhase6AuthorityBoundary {
            helper: "workload_manifest_builder",
            approved_public_route: "local_validation_only",
            overgate_or_public_control_plane_only: true,
            wrapper_only: true,
            owns_scheduler: false,
            owns_policy_truth: false,
            owns_storage_reader: false,
            owns_metering_truth: false,
            owning_services: &["Overgate", "Overwatch"],
        },
        SdkPhase6AuthorityBoundary {
            helper: "workload_submission_helper",
            approved_public_route: SDK_PHASE4_COMMAND_ROUTE,
            overgate_or_public_control_plane_only: true,
            wrapper_only: true,
            owns_scheduler: false,
            owns_policy_truth: false,
            owns_storage_reader: false,
            owns_metering_truth: false,
            owning_services: &["Overgate", "Overqueue", "Overrun", "Overwatch"],
        },
        SdkPhase6AuthorityBoundary {
            helper: "status_result_cancellation_readers",
            approved_public_route: "/v1/control-plane/workloads/*",
            overgate_or_public_control_plane_only: true,
            wrapper_only: true,
            owns_scheduler: false,
            owns_policy_truth: false,
            owns_storage_reader: false,
            owns_metering_truth: false,
            owning_services: &["Overgate", "Overqueue", "Overrun", "Overwatch"],
        },
        SdkPhase6AuthorityBoundary {
            helper: "policy_dry_run_helper",
            approved_public_route: SDK_PHASE6_POLICY_DRY_RUN_ROUTE,
            overgate_or_public_control_plane_only: true,
            wrapper_only: true,
            owns_scheduler: false,
            owns_policy_truth: false,
            owns_storage_reader: false,
            owns_metering_truth: false,
            owning_services: &["Overgate", "Overguard", "Overwatch"],
        },
        SdkPhase6AuthorityBoundary {
            helper: "runtime_authority_boundary",
            approved_public_route: "review_checklist",
            overgate_or_public_control_plane_only: true,
            wrapper_only: true,
            owns_scheduler: false,
            owns_policy_truth: false,
            owns_storage_reader: false,
            owns_metering_truth: false,
            owning_services: SDK_PHASE6_RUNTIME_AUTHORITY_OWNERS,
        },
    ]
}

pub fn validate_phase6_authority_review(
    review: &[SdkPhase6AuthorityBoundary],
) -> Result<(), SdkPhase6Error> {
    for boundary in review {
        if !boundary.overgate_or_public_control_plane_only || !boundary.wrapper_only {
            return Err(SdkPhase6Error::RuntimeAuthorityLeak(boundary.helper));
        }
        if boundary.owns_scheduler
            || boundary.owns_policy_truth
            || boundary.owns_storage_reader
            || boundary.owns_metering_truth
        {
            return Err(SdkPhase6Error::RuntimeAuthorityLeak(boundary.helper));
        }
    }
    for owner in SDK_PHASE6_RUNTIME_AUTHORITY_OWNERS {
        if !review
            .iter()
            .any(|boundary| boundary.owning_services.contains(owner))
        {
            return Err(SdkPhase6Error::RuntimeAuthorityLeak(owner));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkPhase6Error {
    Sdk(SdkError),
    Signing(SdkPhase5Error),
    MissingField(&'static str),
    UnsupportedWorkloadClass(String),
    UnsupportedResourceKind(String),
    InvalidResourceAmount(String),
    MalformedSecretRef(String),
    ForbiddenEgress(String),
    ServiceEvidenceRequired(&'static str),
    RuntimeAuthorityLeak(&'static str),
}

impl fmt::Display for SdkPhase6Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sdk(error) => error.fmt(formatter),
            Self::Signing(error) => error.fmt(formatter),
            Self::MissingField(field) => write!(formatter, "{field} is required"),
            Self::UnsupportedWorkloadClass(class) => {
                write!(formatter, "unsupported workload class: {class}")
            }
            Self::UnsupportedResourceKind(kind) => {
                write!(formatter, "unsupported resource kind: {kind}")
            }
            Self::InvalidResourceAmount(resource) => {
                write!(formatter, "resource amount must be positive: {resource}")
            }
            Self::MalformedSecretRef(secret_ref) => {
                write!(formatter, "malformed secret reference: {secret_ref}")
            }
            Self::ForbiddenEgress(destination) => {
                write!(formatter, "forbidden egress declaration: {destination}")
            }
            Self::ServiceEvidenceRequired(surface) => {
                write!(formatter, "service evidence is required for {surface}")
            }
            Self::RuntimeAuthorityLeak(helper) => {
                write!(formatter, "SDK helper claims runtime authority: {helper}")
            }
        }
    }
}

impl std::error::Error for SdkPhase6Error {}

impl From<SdkError> for SdkPhase6Error {
    fn from(error: SdkError) -> Self {
        Self::Sdk(error)
    }
}

impl From<crate::SdkCompatibilityRejection> for SdkPhase6Error {
    fn from(error: crate::SdkCompatibilityRejection) -> Self {
        Self::Sdk(SdkError::Compatibility(error))
    }
}

fn phase6_headers(
    schema_version: &str,
    request_id: &str,
    trace_id: &str,
    route: &str,
    read_only: bool,
) -> Vec<(String, String)> {
    vec![
        (
            "x-overrid-schema-version".to_owned(),
            schema_version.to_owned(),
        ),
        ("x-overrid-request-id".to_owned(), request_id.to_owned()),
        ("x-overrid-trace-id".to_owned(), trace_id.to_owned()),
        ("x-overrid-route".to_owned(), route.to_owned()),
        ("x-overrid-read-only".to_owned(), read_only.to_string()),
        ("x-overrid-sdk-name".to_owned(), SDK_NAME.to_owned()),
        ("x-overrid-sdk-version".to_owned(), SDK_VERSION.to_owned()),
        (
            "x-overrid-sdk-language-binding".to_owned(),
            SDK_LANGUAGE_BINDING.to_owned(),
        ),
        (
            "x-overrid-sdk-capability-profile".to_owned(),
            SDK_PHASE6_CAPABILITY_PROFILE.to_owned(),
        ),
    ]
}

fn validate_ref_list(field: &'static str, refs: &[String]) -> Result<(), SdkPhase6Error> {
    if refs.is_empty() {
        return Err(SdkPhase6Error::MissingField(field));
    }
    for value in refs {
        require_phase6_non_empty(value, field)?;
    }
    Ok(())
}

fn validate_data_declarations(data_refs: &[SdkDataDeclaration]) -> Result<(), SdkPhase6Error> {
    if data_refs.is_empty() {
        return Err(SdkPhase6Error::MissingField("data ref"));
    }
    for data_ref in data_refs {
        require_phase6_non_empty(&data_ref.data_ref, "data ref")?;
        require_phase6_non_empty(&data_ref.access_mode, "data access mode")?;
    }
    Ok(())
}

fn validate_policy_declarations(
    policy_refs: &[SdkPolicyDeclaration],
) -> Result<(), SdkPhase6Error> {
    if policy_refs.is_empty() {
        return Err(SdkPhase6Error::MissingField("policy ref"));
    }
    for policy_ref in policy_refs {
        require_phase6_non_empty(&policy_ref.policy_ref, "policy ref")?;
    }
    Ok(())
}

fn validate_egress(egress: &SdkEgressDeclaration) -> Result<(), SdkPhase6Error> {
    match egress.mode {
        SdkEgressMode::DenyAll => Ok(()),
        SdkEgressMode::Allowlist => {
            if egress.destinations.is_empty() {
                return Err(SdkPhase6Error::ForbiddenEgress(
                    "empty allowlist".to_owned(),
                ));
            }
            for destination in &egress.destinations {
                validate_egress_destination(destination)?;
            }
            Ok(())
        }
    }
}

fn validate_egress_destination(destination: &str) -> Result<(), SdkPhase6Error> {
    require_phase6_non_empty(destination, "egress destination")?;
    let lower = destination.to_ascii_lowercase();
    if matches!(lower.as_str(), "*" | "0.0.0.0/0" | "::/0")
        || lower.starts_with("http://*")
        || lower.starts_with("https://*")
    {
        return Err(SdkPhase6Error::ForbiddenEgress(destination.to_owned()));
    }
    Ok(())
}

fn validate_secret_ref(secret_ref: &str) -> Result<(), SdkPhase6Error> {
    require_phase6_non_empty(secret_ref, "secret ref")?;
    if secret_ref.starts_with("secret_ref:")
        || secret_ref.starts_with("overvault://")
        || secret_ref.starts_with("vault_ref:")
    {
        return Ok(());
    }
    Err(SdkPhase6Error::MalformedSecretRef(secret_ref.to_owned()))
}

fn require_phase6_non_empty(value: &str, field: &'static str) -> Result<(), SdkPhase6Error> {
    if value.trim().is_empty() {
        return Err(SdkPhase6Error::MissingField(field));
    }
    Ok(())
}

fn stable_phase6_hash(value: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("phase6_{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        SdkConfigInput, SdkConfigRecord, SdkCredentialProvider,
        SDK_CAPABILITY_UNAVAILABLE_REASON_CODE,
    };
    use overrid_contracts::{
        CredentialReference, CredentialReferenceClass, EnvironmentClass, RetryClass,
        SUPPORTED_SCHEMA_VERSION,
    };

    const PHASE6_TIMESTAMP_MS: u64 = 1_782_024_000_000;

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

    fn phase6_profile(dry_run: bool) -> SdkServiceCapabilityProfile {
        SdkServiceCapabilityProfile {
            profile_name: SDK_PHASE6_CAPABILITY_PROFILE.to_owned(),
            supported_schema_versions: vec![SUPPORTED_SCHEMA_VERSION.to_owned()],
            supported_sdk_majors: vec![SDK_CURRENT_STABLE_MAJOR],
            signing: true,
            idempotency: true,
            dry_run,
            accounting: false,
        }
    }

    fn provider() -> SdkCredentialProvider {
        let config = SdkConfigRecord::from_input(SdkConfigInput {
            environment: Some(EnvironmentClass::Local),
            base_url: "http://127.0.0.1:18080/overgate".to_owned(),
            timeout_ms: Some(10_000),
            max_retries: Some(2),
            feature_flags: vec!["test_fixtures".to_owned()],
            default_tenant_id: Some("tenant_local".to_owned()),
            client_identity_ref: "client:overrid-sdk:phase6".to_owned(),
            credential_ref: fixture_credential(),
            service_capability_profile: phase6_profile(true),
            live_endpoint_confirmed: false,
            test_fixtures_enabled: true,
        })
        .unwrap();
        SdkCredentialProvider::from_config(&config).unwrap()
    }

    fn manifest_input() -> SdkWorkloadManifestInput {
        SdkWorkloadManifestInput {
            workload_id: "workload_phase6".to_owned(),
            workload_class: "container_task".to_owned(),
            schema_version: SUPPORTED_SCHEMA_VERSION.to_owned(),
            resources: vec![
                SdkResourceDeclaration::new("cpu", "cpu", 2, "cores").unwrap(),
                SdkResourceDeclaration::new("memory", "memory", 4096, "mb").unwrap(),
            ],
            data_refs: vec![SdkDataDeclaration::new("data:input:phase6", "read").unwrap()],
            policy_refs: vec![SdkPolicyDeclaration::new("policy:deny-public-egress", true).unwrap()],
            egress: SdkEgressDeclaration::deny_all(),
            output: SdkOutputDeclaration::new("output:phase6", "application/json", "ephemeral")
                .unwrap(),
            secret_refs: vec![SdkSecretReferenceDeclaration::new(
                "secret_ref:tenant_local/api-key",
                "/secrets/api-key",
                true,
            )
            .unwrap()],
        }
    }

    fn manifest() -> SdkWorkloadManifest {
        build_workload_manifest(manifest_input()).unwrap()
    }

    #[test]
    fn phase6_manifest_builder_validates_local_shape_without_runtime_acceptance() {
        let manifest = manifest();

        assert!(manifest.validation.valid_local_shape);
        assert!(!manifest.validation.runtime_acceptance_claimed);
        assert_eq!(manifest.workload_class, SdkWorkloadClass::ContainerTask);
        assert_eq!(manifest.schema_version.raw(), SUPPORTED_SCHEMA_VERSION);
        assert!(manifest.validation.checked_fields.contains(&"secret_refs"));
        assert!(manifest
            .command_payload_fields()
            .contains(&("runtime_acceptance_claimed".to_owned(), "false".to_owned())));
        assert!(manifest
            .command_payload_fields()
            .contains(&("data_ref".to_owned(), "data:input:phase6".to_owned())));
        assert!(manifest.command_payload_fields().contains(&(
            "policy_ref".to_owned(),
            "policy:deny-public-egress".to_owned()
        )));

        let mut missing_schema = manifest_input();
        missing_schema.schema_version.clear();
        assert!(matches!(
            build_workload_manifest(missing_schema),
            Err(SdkPhase6Error::MissingField("schema version"))
        ));

        let mut unsupported = manifest_input();
        unsupported.workload_class = "scheduler_override".to_owned();
        assert!(matches!(
            build_workload_manifest(unsupported),
            Err(SdkPhase6Error::UnsupportedWorkloadClass(class)) if class == "scheduler_override"
        ));

        let mut missing_data = manifest_input();
        missing_data.data_refs.clear();
        assert!(matches!(
            build_workload_manifest(missing_data),
            Err(SdkPhase6Error::MissingField("data ref"))
        ));

        assert!(matches!(
            SdkDataDeclaration::new("data:input:phase6", ""),
            Err(SdkPhase6Error::MissingField("data access mode"))
        ));
        assert!(matches!(
            SdkPolicyDeclaration::new("", true),
            Err(SdkPhase6Error::MissingField("policy ref"))
        ));
        assert!(matches!(
            SdkResourceDeclaration::new("cpu", "cpu", 0, "cores"),
            Err(SdkPhase6Error::InvalidResourceAmount(resource)) if resource == "cpu"
        ));
        assert!(matches!(
            SdkSecretReferenceDeclaration::new("raw-secret-value", "/secrets/raw", true),
            Err(SdkPhase6Error::MalformedSecretRef(value)) if value == "raw-secret-value"
        ));
        assert!(matches!(
            SdkEgressDeclaration::allowlist(vec!["0.0.0.0/0".to_owned()]),
            Err(SdkPhase6Error::ForbiddenEgress(value)) if value == "0.0.0.0/0"
        ));
    }

    #[test]
    fn phase6_submit_workload_wraps_manifest_command_signing_and_overgate_submission() {
        let endpoint =
            OvergateEndpoint::parse("http://127.0.0.1:18080/overgate", EnvironmentClass::Local)
                .unwrap();
        let submission = submit_workload(SdkWorkloadSubmissionInput {
            endpoint,
            service_capability_profile: phase6_profile(true),
            manifest: manifest(),
            provider: provider(),
            tenant_id: "tenant_local".to_owned(),
            actor_id: "actor_local".to_owned(),
            idempotency_key: "idem_phase6_workload".to_owned(),
            trace_id: "trace_phase6_workload".to_owned(),
            timestamp_ms: PHASE6_TIMESTAMP_MS,
            command_deadline_at_ms: Some(PHASE6_TIMESTAMP_MS + 60_000),
            replay_window_ms: 120_000,
        })
        .unwrap();

        assert_eq!(
            submission.command.envelope.command_type,
            SDK_PHASE6_WORKLOAD_COMMAND_TYPE
        );
        assert_eq!(
            submission.command.command_class,
            SdkCommandClass::LongRunningWorkload
        );
        assert_eq!(
            submission.overgate_submission.route,
            SDK_PHASE4_COMMAND_ROUTE
        );
        assert!(submission.overgate_submission.overgate_only);
        assert!(!submission.runtime_authority_claimed);
        assert_eq!(
            submission.signed_request.algorithm,
            crate::SDK_PHASE5_SIGNING_ALGORITHM
        );

        let outcome = decode_workload_submission_response(
            &submission,
            SdkOvergateResponse::accepted(
                "trace_phase6_workload",
                "accepted:workload:phase6",
                "queue:pending:phase6",
                vec!["audit:overwatch:phase6".to_owned()],
            ),
        )
        .unwrap();

        assert!(outcome.pending_queue_reached);
        assert!(!outcome.runtime_completion_invented);
        assert!(outcome.denied_reason_code.is_none());

        let pending_without_queue = decode_workload_submission_response(
            &submission,
            SdkOvergateResponse::accepted(
                "trace_phase6_workload",
                "accepted:workload:phase6:review",
                "pending_review",
                vec!["audit:overwatch:phase6:review".to_owned()],
            ),
        )
        .unwrap();
        assert!(!pending_without_queue.pending_queue_reached);
        assert!(!pending_without_queue.runtime_completion_invented);
    }

    #[test]
    fn phase6_status_result_and_cancellation_helpers_preserve_service_states() {
        let endpoint =
            OvergateEndpoint::parse("http://127.0.0.1:18080/overgate", EnvironmentClass::Local)
                .unwrap();
        let command_status_request = build_command_status_request(
            &endpoint,
            "command:phase6",
            "request_phase6_command_status",
            "trace_phase6_command_status",
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap();
        assert!(command_status_request.read_only);
        assert_eq!(
            command_status_request.route,
            SDK_PHASE6_COMMAND_STATUS_ROUTE
        );

        let status_request = build_workload_status_request(
            &endpoint,
            "workload:phase6",
            "request_phase6_status",
            "trace_phase6_status",
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap();
        assert!(status_request.read_only);
        assert!(status_request.public_control_plane_path);
        assert_eq!(status_request.route, SDK_PHASE6_WORKLOAD_STATUS_ROUTE);

        let job_status_request = build_job_status_request(
            &endpoint,
            "job:phase6",
            "request_phase6_job_status",
            "trace_phase6_job_status",
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap();
        assert_eq!(job_status_request.route, SDK_PHASE6_JOB_STATUS_ROUTE);

        let result_request = build_workload_result_request(
            &endpoint,
            "workload:phase6",
            "request_phase6_result",
            "trace_phase6_result",
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap();
        assert_eq!(result_request.route, SDK_PHASE6_RESULT_ROUTE);
        assert!(result_request.read_only);

        let cancellation_status_request = build_cancellation_status_request(
            &endpoint,
            "cancel:phase6",
            "request_phase6_cancel_status",
            "trace_phase6_cancel_status",
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap();
        assert_eq!(
            cancellation_status_request.route,
            SDK_PHASE6_CANCELLATION_ROUTE
        );
        assert!(cancellation_status_request.read_only);

        for state in [
            SdkWorkloadServiceState::Failed,
            SdkWorkloadServiceState::Cancelled,
            SdkWorkloadServiceState::TimedOut,
        ] {
            let record = SdkWorkloadStatusRecord::from_service(SdkWorkloadStatusInput {
                state,
                service_reported: true,
                trace_id: "trace_phase6_status".to_owned(),
                audit_refs: vec![format!("audit:{}", state.as_str())],
                queue_ref: Some("queue:phase6".to_owned()),
                result_ref: None,
                duplicate_of: None,
                dead_letter_ref: None,
            })
            .unwrap();
            assert_eq!(record.state, state);
            assert!(!record.sdk_invented_terminal_state);
        }

        let dead_letter = SdkWorkloadStatusRecord::from_service(SdkWorkloadStatusInput {
            state: SdkWorkloadServiceState::DeadLetter,
            service_reported: true,
            trace_id: "trace_phase6_dead_letter".to_owned(),
            audit_refs: vec!["audit:dead-letter".to_owned()],
            queue_ref: Some("queue:phase6".to_owned()),
            result_ref: None,
            duplicate_of: None,
            dead_letter_ref: Some("dead-letter:phase6".to_owned()),
        })
        .unwrap();
        assert_eq!(
            dead_letter.dead_letter_ref.as_deref(),
            Some("dead-letter:phase6")
        );

        let duplicate = SdkWorkloadStatusRecord::from_service(SdkWorkloadStatusInput {
            state: SdkWorkloadServiceState::Duplicate,
            service_reported: true,
            trace_id: "trace_phase6_duplicate".to_owned(),
            audit_refs: vec!["audit:duplicate".to_owned()],
            queue_ref: Some("queue:phase6".to_owned()),
            result_ref: None,
            duplicate_of: Some("workload:phase6-original".to_owned()),
            dead_letter_ref: None,
        })
        .unwrap();
        assert_eq!(
            duplicate.duplicate_of.as_deref(),
            Some("workload:phase6-original")
        );

        assert!(matches!(
            SdkWorkloadStatusRecord::from_service(SdkWorkloadStatusInput {
                state: SdkWorkloadServiceState::Completed,
                service_reported: false,
                trace_id: "trace_without_service".to_owned(),
                audit_refs: vec![],
                queue_ref: None,
                result_ref: Some("result:phase6".to_owned()),
                duplicate_of: None,
                dead_letter_ref: None,
            }),
            Err(SdkPhase6Error::ServiceEvidenceRequired(
                "workload status service response"
            ))
        ));

        let cancellation = build_workload_cancellation_request(
            &endpoint,
            "workload:phase6",
            "request_phase6_cancel",
            "trace_phase6_cancel",
            "operator requested stop",
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap();
        assert_eq!(cancellation.route, SDK_PHASE6_CANCELLATION_ROUTE);
        assert!(cancellation.requires_service_state_for_cancelled);
        assert!(cancellation.public_control_plane_path);
    }

    #[test]
    fn phase6_policy_dry_run_fails_closed_and_never_caches_policy_truth() {
        let endpoint =
            OvergateEndpoint::parse("http://127.0.0.1:18080/overgate", EnvironmentClass::Local)
                .unwrap();
        let no_dry_run = phase6_profile(false);
        assert!(matches!(
            build_policy_dry_run_request(
                &endpoint,
                &no_dry_run,
                SdkPolicyDryRunInput {
                    manifest: manifest(),
                    request_id: "request_phase6_dry_run".to_owned(),
                    trace_id: "trace_phase6_dry_run".to_owned(),
                    schema_version: SUPPORTED_SCHEMA_VERSION.to_owned(),
                },
            ),
            Err(SdkPhase6Error::Sdk(SdkError::CapabilityUnavailable {
                helper: "policy_dry_run",
                reason_code: SDK_CAPABILITY_UNAVAILABLE_REASON_CODE
            }))
        ));

        let request = build_policy_dry_run_request(
            &endpoint,
            &phase6_profile(true),
            SdkPolicyDryRunInput {
                manifest: manifest(),
                request_id: "request_phase6_dry_run".to_owned(),
                trace_id: "trace_phase6_dry_run".to_owned(),
                schema_version: SUPPORTED_SCHEMA_VERSION.to_owned(),
            },
        )
        .unwrap();
        assert_eq!(request.route, SDK_PHASE6_POLICY_DRY_RUN_ROUTE);
        assert!(request.fail_closed);
        assert!(!request.mutates_runtime_state);
        assert!(!request.cacheable_as_policy_truth);

        let result = decode_policy_dry_run_result(SdkPolicyDryRunResultInput {
            service_reported: true,
            matched_policy_refs: vec!["policy:deny-public-egress".to_owned()],
            reason_codes: vec!["policy_dry_run_allowed".to_owned()],
            estimated_placement_class: "private_seed_pool".to_owned(),
            correction_fields: vec!["resource.cpu".to_owned()],
            trace_id: "trace_phase6_dry_run".to_owned(),
        })
        .unwrap();
        assert_eq!(
            result.matched_policy_refs,
            vec!["policy:deny-public-egress"]
        );
        assert!(!result.mutates_runtime_state);
        assert!(!result.cacheable_as_policy_truth);
    }

    #[test]
    fn phase6_authority_review_blocks_runtime_ownership_inside_sdk() {
        let review = sdk_phase6_authority_review();
        validate_phase6_authority_review(&review).unwrap();
        assert!(review.iter().all(|boundary| boundary.wrapper_only));
        assert!(review.iter().all(|boundary| !boundary.owns_scheduler));
        assert!(review.iter().all(|boundary| !boundary.owns_policy_truth));
        assert!(review.iter().all(|boundary| !boundary.owns_storage_reader));
        assert!(review.iter().all(|boundary| !boundary.owns_metering_truth));
        for owner in SDK_PHASE6_RUNTIME_AUTHORITY_OWNERS {
            assert!(review
                .iter()
                .any(|boundary| boundary.owning_services.contains(owner)));
        }
    }

    #[test]
    fn phase6_policy_and_status_errors_preserve_stable_reason_refs() {
        let error = crate::decode_stable_overrid_error(crate::SdkServiceErrorInput {
            reason_code: "policy_denial".to_owned(),
            message: "policy denied workload".to_owned(),
            trace_id: Some("trace_phase6_policy".to_owned()),
            audit_refs: vec!["audit:phase6:policy".to_owned()],
            retry_class: RetryClass::NotRetryable,
            correction_fields: vec!["policy_ref".to_owned()],
            dependency_name: Some("overguard".to_owned()),
            policy_refs: vec!["policy:deny-public-egress".to_owned()],
            schema_version: SUPPORTED_SCHEMA_VERSION.to_owned(),
        })
        .unwrap();

        assert_eq!(error.reason_code, "policy_denial");
        assert_eq!(error.policy_refs, vec!["policy:deny-public-egress"]);
        assert_eq!(error.dependency_name.as_deref(), Some("overguard"));
    }
}
