use crate::{
    check_sdk_compatibility, OvergateEndpoint, OverridErrorRecord, SdkCommandClass, SdkError,
    SdkIdempotencyEntry, SDK_CURRENT_STABLE_MAJOR, SDK_LANGUAGE_BINDING, SDK_NAME,
    SDK_PHASE3_CAPABILITY_PROFILE, SDK_PHASE3_GENERATED_CONTRACT_REVISION, SDK_VERSION,
};
use overrid_contracts::{
    BootstrapAcceptanceRecord, BootstrapCommandFamily, RetryClass, SignedCommandEnvelope,
};

pub const SDK_PHASE4_CAPABILITY_PROFILE: &str = "phase4-command-pipeline-idempotency-retry-errors";
pub const SDK_PHASE4_COMMAND_ROUTE: &str = "/v1/overgate/commands";
pub const SDK_PHASE4_IN_FLIGHT_RETRY_RETENTION_MS: u64 = 15 * 60 * 1_000;
pub const SDK_PHASE4_PHASE1_TERMINAL_RETENTION_MS: u64 = 24 * 60 * 60 * 1_000;
pub const SDK_PHASE4_WORKLOAD_REF_RETENTION_MS: u64 = 7 * 24 * 60 * 60 * 1_000;
pub const SDK_PHASE4_SECURITY_SENSITIVE_RETENTION_MS: u64 = 4 * 60 * 60 * 1_000;
pub const SDK_PHASE4_CLEANUP_CONTROL: &str = "expiry_sweep_and_dev_reset";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkCommandPayload {
    pub payload_type: String,
    pub fields: Vec<(String, String)>,
}

impl SdkCommandPayload {
    pub fn new(
        payload_type: impl Into<String>,
        fields: Vec<(String, String)>,
    ) -> Result<Self, SdkError> {
        let payload_type = payload_type.into();
        require_phase4_non_empty(&payload_type, "payload type")?;
        if fields.is_empty() {
            return Err(SdkError::MissingRequiredField("payload fields"));
        }
        for (key, value) in &fields {
            require_phase4_non_empty(key, "payload field key")?;
            require_phase4_non_empty(value, "payload field value")?;
        }
        Ok(Self {
            payload_type,
            fields,
        })
    }

    pub fn canonical_payload(&self) -> String {
        let mut fields = self.fields.clone();
        fields.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
        let mut canonical = format!("payload_type={}", canonical_escape(&self.payload_type));
        for (key, value) in fields {
            canonical.push('\n');
            canonical.push_str("field:");
            canonical.push_str(&canonical_escape(&key));
            canonical.push('=');
            canonical.push_str(&canonical_escape(&value));
        }
        canonical
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkCommandBuildInput {
    pub family: BootstrapCommandFamily,
    pub command_type: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub target_ref: String,
    pub payload: SdkCommandPayload,
    pub expected_state: Option<String>,
    pub reason: Option<String>,
    pub idempotency_key: String,
    pub trace_id: String,
    pub timestamp_ms: u64,
    pub signature_ref: String,
    pub schema_version: String,
    pub command_class: SdkCommandClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkCommandEnvelope {
    pub envelope: SignedCommandEnvelope,
    pub timestamp_ms: u64,
    pub schema_version: String,
    pub canonical_payload: String,
    pub payload_hash: String,
    pub command_class: SdkCommandClass,
    pub headers: Vec<(String, String)>,
}

pub fn build_command(input: SdkCommandBuildInput) -> Result<SdkCommandEnvelope, SdkError> {
    if matches!(input.command_class, SdkCommandClass::ReadOnly) {
        return Err(SdkError::MissingRequiredField("mutating command class"));
    }
    require_phase4_non_empty(&input.command_type, "command type")?;
    require_phase4_non_empty(&input.tenant_id, "tenant id")?;
    require_phase4_non_empty(&input.actor_id, "actor id")?;
    require_phase4_non_empty(&input.target_ref, "target ref")?;
    require_phase4_non_empty(&input.idempotency_key, "idempotency key")?;
    require_phase4_non_empty(&input.trace_id, "trace id")?;
    require_phase4_non_empty(&input.signature_ref, "signature ref")?;
    if input.timestamp_ms == 0 {
        return Err(SdkError::MissingRequiredField("timestamp"));
    }
    let schema_version = check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, &input.schema_version)?;
    let canonical_payload = input.payload.canonical_payload();
    let payload_hash = stable_phase4_hash("hash", &canonical_payload);

    let envelope = SignedCommandEnvelope::new(
        input.family,
        input.command_type,
        input.tenant_id,
        input.actor_id,
        input.target_ref,
        input.payload.payload_type,
        input.expected_state,
        input.reason,
        input.idempotency_key,
        input.trace_id,
        input.signature_ref,
    )?;

    Ok(SdkCommandEnvelope {
        headers: phase4_headers(
            schema_version.raw(),
            &envelope.trace_context.trace_id,
            &envelope.idempotency.key,
            &envelope.command_type,
            &envelope.tenant_id,
            &envelope.actor_id,
            input.timestamp_ms,
            &payload_hash,
        ),
        envelope,
        timestamp_ms: input.timestamp_ms,
        schema_version: schema_version.raw().to_owned(),
        canonical_payload,
        payload_hash,
        command_class: input.command_class,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkOvergateSubmission {
    pub endpoint: String,
    pub route: &'static str,
    pub method: &'static str,
    pub command: SdkCommandEnvelope,
    pub body_hash: String,
    pub headers: Vec<(String, String)>,
    pub overgate_only: bool,
}

pub fn prepare_overgate_submission(
    endpoint: &OvergateEndpoint,
    command: SdkCommandEnvelope,
) -> Result<SdkOvergateSubmission, SdkError> {
    Ok(SdkOvergateSubmission {
        endpoint: endpoint.raw().to_owned(),
        route: SDK_PHASE4_COMMAND_ROUTE,
        method: "POST",
        body_hash: command.payload_hash.clone(),
        headers: command.headers.clone(),
        command,
        overgate_only: true,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkOvergateResponseStatus {
    Accepted,
    Completed,
    Denied,
    Duplicate,
    Retryable,
    TerminalFailure,
}

impl SdkOvergateResponseStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Completed => "completed",
            Self::Denied => "denied",
            Self::Duplicate => "duplicate",
            Self::Retryable => "retry_wait",
            Self::TerminalFailure => "failed",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Denied | Self::Duplicate | Self::TerminalFailure
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkOvergateResponse {
    pub status: SdkOvergateResponseStatus,
    pub accepted_ref: Option<String>,
    pub queue_ref: Option<String>,
    pub trace_id: String,
    pub audit_refs: Vec<String>,
    pub retry_class: RetryClass,
    pub error: Option<OverridErrorRecord>,
    pub duplicate_of: Option<String>,
    pub retry_after_ms: Option<u64>,
    pub terminal_digest: Option<String>,
    pub explicit_service_response: bool,
}

impl SdkOvergateResponse {
    pub fn accepted(
        trace_id: impl Into<String>,
        accepted_ref: impl Into<String>,
        queue_ref: impl Into<String>,
        audit_refs: Vec<String>,
    ) -> Self {
        Self {
            status: SdkOvergateResponseStatus::Accepted,
            accepted_ref: Some(accepted_ref.into()),
            queue_ref: Some(queue_ref.into()),
            trace_id: trace_id.into(),
            audit_refs,
            retry_class: RetryClass::NotRetryable,
            error: None,
            duplicate_of: None,
            retry_after_ms: None,
            terminal_digest: None,
            explicit_service_response: true,
        }
    }

    pub fn completed(
        trace_id: impl Into<String>,
        accepted_ref: impl Into<String>,
        audit_refs: Vec<String>,
    ) -> Self {
        let trace_id = trace_id.into();
        let accepted_ref = accepted_ref.into();
        Self {
            status: SdkOvergateResponseStatus::Completed,
            accepted_ref: Some(accepted_ref.clone()),
            queue_ref: None,
            terminal_digest: Some(stable_phase4_hash(
                "digest",
                &format!("{trace_id}:{accepted_ref}:completed"),
            )),
            trace_id,
            audit_refs,
            retry_class: RetryClass::NotRetryable,
            error: None,
            duplicate_of: None,
            retry_after_ms: None,
            explicit_service_response: true,
        }
    }

    pub fn denied(trace_id: impl Into<String>, error: OverridErrorRecord) -> Self {
        let trace_id = trace_id.into();
        Self {
            status: SdkOvergateResponseStatus::Denied,
            terminal_digest: Some(stable_phase4_hash(
                "digest",
                &format!("{}:{}:denied", trace_id, error.reason_code),
            )),
            trace_id,
            accepted_ref: None,
            queue_ref: None,
            audit_refs: error.audit_refs.clone(),
            retry_class: error.retry_class,
            error: Some(error),
            duplicate_of: None,
            retry_after_ms: None,
            explicit_service_response: true,
        }
    }

    pub fn duplicate(
        trace_id: impl Into<String>,
        duplicate_of: impl Into<String>,
        audit_refs: Vec<String>,
    ) -> Self {
        let trace_id = trace_id.into();
        let duplicate_of = duplicate_of.into();
        Self {
            status: SdkOvergateResponseStatus::Duplicate,
            terminal_digest: Some(stable_phase4_hash(
                "digest",
                &format!("{trace_id}:{duplicate_of}:duplicate"),
            )),
            trace_id,
            accepted_ref: None,
            queue_ref: None,
            audit_refs,
            retry_class: RetryClass::NotRetryable,
            error: None,
            duplicate_of: Some(duplicate_of),
            retry_after_ms: None,
            explicit_service_response: true,
        }
    }

    pub fn retryable(
        trace_id: impl Into<String>,
        retry_after_ms: u64,
        audit_refs: Vec<String>,
    ) -> Self {
        Self {
            status: SdkOvergateResponseStatus::Retryable,
            accepted_ref: None,
            queue_ref: None,
            trace_id: trace_id.into(),
            audit_refs,
            retry_class: RetryClass::RetryAfter,
            error: None,
            duplicate_of: None,
            retry_after_ms: Some(retry_after_ms),
            terminal_digest: None,
            explicit_service_response: true,
        }
    }

    pub fn terminal_failure(trace_id: impl Into<String>, error: OverridErrorRecord) -> Self {
        let trace_id = trace_id.into();
        Self {
            status: SdkOvergateResponseStatus::TerminalFailure,
            terminal_digest: Some(stable_phase4_hash(
                "digest",
                &format!("{}:{}:failed", trace_id, error.reason_code),
            )),
            trace_id,
            accepted_ref: None,
            queue_ref: None,
            audit_refs: error.audit_refs.clone(),
            retry_class: error.retry_class,
            error: Some(error),
            duplicate_of: None,
            retry_after_ms: None,
            explicit_service_response: true,
        }
    }

    pub fn without_service_confirmation(mut self) -> Self {
        self.explicit_service_response = false;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkCommandLifecycleState {
    Prepared,
    PayloadValidated,
    Signed,
    Submitted,
    Accepted,
    RetryWait,
    Completed,
    Denied,
    Duplicate,
    Failed,
}

impl SdkCommandLifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Prepared => "prepared",
            Self::PayloadValidated => "payload_validated",
            Self::Signed => "signed",
            Self::Submitted => "submitted",
            Self::Accepted => "accepted",
            Self::RetryWait => "retry_wait",
            Self::Completed => "completed",
            Self::Denied => "denied",
            Self::Duplicate => "duplicate",
            Self::Failed => "failed",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Denied | Self::Duplicate | Self::Failed
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkCommandOutcome {
    pub status: SdkOvergateResponseStatus,
    pub lifecycle: Vec<SdkCommandLifecycleState>,
    pub acceptance: Option<BootstrapAcceptanceRecord>,
    pub trace_id: String,
    pub audit_refs: Vec<String>,
    pub retry_class: RetryClass,
    pub retry_after_ms: Option<u64>,
    pub error: Option<OverridErrorRecord>,
    pub duplicate_of: Option<String>,
}

pub fn decode_overgate_submission(
    command: &SdkCommandEnvelope,
    response: SdkOvergateResponse,
) -> Result<SdkCommandOutcome, SdkError> {
    if command.envelope.trace_context.trace_id != response.trace_id {
        return Err(SdkError::TraceMismatch {
            expected: command.envelope.trace_context.trace_id.clone(),
            actual: response.trace_id,
        });
    }
    if !response.explicit_service_response {
        return Err(SdkError::InvalidLifecycleTransition {
            from: "submitted",
            to: response.status.as_str(),
        });
    }

    let mut lifecycle = vec![
        SdkCommandLifecycleState::Prepared,
        SdkCommandLifecycleState::PayloadValidated,
        SdkCommandLifecycleState::Signed,
        SdkCommandLifecycleState::Submitted,
    ];
    lifecycle.push(match response.status {
        SdkOvergateResponseStatus::Accepted => SdkCommandLifecycleState::Accepted,
        SdkOvergateResponseStatus::Completed => SdkCommandLifecycleState::Completed,
        SdkOvergateResponseStatus::Denied => SdkCommandLifecycleState::Denied,
        SdkOvergateResponseStatus::Duplicate => SdkCommandLifecycleState::Duplicate,
        SdkOvergateResponseStatus::Retryable => SdkCommandLifecycleState::RetryWait,
        SdkOvergateResponseStatus::TerminalFailure => SdkCommandLifecycleState::Failed,
    });

    let acceptance = match response.status {
        SdkOvergateResponseStatus::Accepted => Some(BootstrapAcceptanceRecord::new(
            command.envelope.command_type.clone(),
            response
                .accepted_ref
                .clone()
                .ok_or(SdkError::MissingRequiredField("accepted ref"))?,
            response
                .queue_ref
                .clone()
                .ok_or(SdkError::MissingRequiredField("queue ref"))?,
            response.audit_refs.clone(),
        )),
        SdkOvergateResponseStatus::Completed => Some(BootstrapAcceptanceRecord::new(
            command.envelope.command_type.clone(),
            response
                .accepted_ref
                .clone()
                .ok_or(SdkError::MissingRequiredField("accepted ref"))?,
            "completed",
            response.audit_refs.clone(),
        )),
        _ => None,
    };

    Ok(SdkCommandOutcome {
        status: response.status,
        lifecycle,
        acceptance,
        trace_id: response.trace_id,
        audit_refs: response.audit_refs,
        retry_class: response.retry_class,
        retry_after_ms: response.retry_after_ms,
        error: response.error,
        duplicate_of: response.duplicate_of,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkIdempotencyCacheStatus {
    SkipReadOnly,
    StoreInFlightRetry,
    StoreTerminalDigest,
    ClearedByDevReset,
}

impl SdkIdempotencyCacheStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SkipReadOnly => "skip_read_only",
            Self::StoreInFlightRetry => "store_in_flight_retry",
            Self::StoreTerminalDigest => "store_terminal_digest",
            Self::ClearedByDevReset => "cleared_by_dev_reset",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkIdempotencyCachePolicy {
    pub command_class: SdkCommandClass,
    pub read_only_cached: bool,
    pub terminal_retention_ms: Option<u64>,
    pub in_flight_retry_retention_ms: u64,
    pub user_clearable: bool,
    pub cleanup_control: &'static str,
}

pub fn phase4_idempotency_policy(command_class: SdkCommandClass) -> SdkIdempotencyCachePolicy {
    let terminal_retention_ms = match command_class {
        SdkCommandClass::ReadOnly => None,
        SdkCommandClass::Phase1Mutating => Some(SDK_PHASE4_PHASE1_TERMINAL_RETENTION_MS),
        SdkCommandClass::LongRunningWorkload => Some(SDK_PHASE4_WORKLOAD_REF_RETENTION_MS),
        SdkCommandClass::SecuritySensitive => Some(SDK_PHASE4_SECURITY_SENSITIVE_RETENTION_MS),
        SdkCommandClass::AccountingReceiptOrDispute => {
            SdkCommandClass::AccountingReceiptOrDispute.retention_ms()
        }
    };

    SdkIdempotencyCachePolicy {
        command_class,
        read_only_cached: false,
        terminal_retention_ms,
        in_flight_retry_retention_ms: SDK_PHASE4_IN_FLIGHT_RETRY_RETENTION_MS,
        user_clearable: true,
        cleanup_control: SDK_PHASE4_CLEANUP_CONTROL,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkIdempotencyCacheDecision {
    pub status: SdkIdempotencyCacheStatus,
    pub idempotency_key: String,
    pub request_hash: String,
    pub terminal_response_digest: Option<String>,
    pub trace_id: String,
    pub audit_refs: Vec<String>,
    pub retention_ms: Option<u64>,
    pub user_clearable: bool,
    pub cleanup_control: &'static str,
}

pub fn evaluate_idempotency_cache(
    command: &SdkCommandEnvelope,
    response: &SdkOvergateResponse,
    existing: Option<&SdkIdempotencyCacheDecision>,
) -> Result<SdkIdempotencyCacheDecision, SdkError> {
    let policy = phase4_idempotency_policy(command.command_class);
    if matches!(command.command_class, SdkCommandClass::ReadOnly) {
        return Ok(SdkIdempotencyCacheDecision {
            status: SdkIdempotencyCacheStatus::SkipReadOnly,
            idempotency_key: command.envelope.idempotency.key.clone(),
            request_hash: command.payload_hash.clone(),
            terminal_response_digest: None,
            trace_id: command.envelope.trace_context.trace_id.clone(),
            audit_refs: vec![],
            retention_ms: None,
            user_clearable: true,
            cleanup_control: SDK_PHASE4_CLEANUP_CONTROL,
        });
    }
    if let Some(existing) = existing {
        if existing.request_hash != command.payload_hash {
            return Err(SdkError::IdempotencyConflict {
                key: command.envelope.idempotency.key.clone(),
            });
        }
    }

    let terminal = response.status.is_terminal();
    let terminal_response_digest = if terminal {
        response.terminal_digest.clone().or_else(|| {
            Some(stable_phase4_hash(
                "digest",
                &format!(
                    "{}:{}:{}",
                    response.trace_id,
                    command.envelope.idempotency.key,
                    response.status.as_str()
                ),
            ))
        })
    } else {
        None
    };
    let retention_ms = if terminal {
        policy.terminal_retention_ms
    } else {
        Some(policy.in_flight_retry_retention_ms)
    };

    let _contract_entry = SdkIdempotencyEntry::for_command_class(
        command.command_class,
        &crate::SdkRequestContextRecord::new(
            command.envelope.actor_id.clone(),
            command.envelope.tenant_id.clone(),
            command.envelope.trace_context.trace_id.clone(),
            command.envelope.idempotency.key.clone(),
            command.envelope.command_type.clone(),
            None,
            command.timestamp_ms,
            &command.schema_version,
        )?,
        command.payload_hash.clone(),
        terminal_response_digest.clone(),
        response.audit_refs.clone(),
        response.retry_class,
        vec![],
    )?;

    Ok(SdkIdempotencyCacheDecision {
        status: if terminal {
            SdkIdempotencyCacheStatus::StoreTerminalDigest
        } else {
            SdkIdempotencyCacheStatus::StoreInFlightRetry
        },
        idempotency_key: command.envelope.idempotency.key.clone(),
        request_hash: command.payload_hash.clone(),
        terminal_response_digest,
        trace_id: response.trace_id.clone(),
        audit_refs: response.audit_refs.clone(),
        retention_ms,
        user_clearable: policy.user_clearable,
        cleanup_control: policy.cleanup_control,
    })
}

pub fn clear_phase4_idempotency_cache(
    idempotency_key: impl Into<String>,
) -> Result<SdkIdempotencyCacheDecision, SdkError> {
    let idempotency_key = idempotency_key.into();
    require_phase4_non_empty(&idempotency_key, "idempotency key")?;
    Ok(SdkIdempotencyCacheDecision {
        status: SdkIdempotencyCacheStatus::ClearedByDevReset,
        idempotency_key,
        request_hash: "hash_cleared_by_dev_reset".to_owned(),
        terminal_response_digest: None,
        trace_id: "trace_local_dev_reset".to_owned(),
        audit_refs: vec![],
        retention_ms: Some(0),
        user_clearable: true,
        cleanup_control: SDK_PHASE4_CLEANUP_CONTROL,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkRetrySignal {
    TransportFailure,
    Timeout,
    ServiceRetryAfter(u64),
    ServiceRetryable,
    ServiceAccepted,
    ServiceDenied,
    ServiceDuplicate,
    ServiceTerminalFailure,
    LocalValidationFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkRetryClassification {
    pub retry_class: RetryClass,
    pub safe_to_retry: bool,
    pub retry_after_ms: Option<u64>,
    pub requires_same_idempotency_key: bool,
    pub lifecycle_state: SdkCommandLifecycleState,
    pub reason: &'static str,
}

pub fn classify_phase4_retry(signal: SdkRetrySignal) -> SdkRetryClassification {
    match signal {
        SdkRetrySignal::TransportFailure => SdkRetryClassification {
            retry_class: RetryClass::SafeRetry,
            safe_to_retry: true,
            retry_after_ms: None,
            requires_same_idempotency_key: true,
            lifecycle_state: SdkCommandLifecycleState::Submitted,
            reason: "transport failure before terminal service response",
        },
        SdkRetrySignal::Timeout => SdkRetryClassification {
            retry_class: RetryClass::SafeRetry,
            safe_to_retry: true,
            retry_after_ms: None,
            requires_same_idempotency_key: true,
            lifecycle_state: SdkCommandLifecycleState::Submitted,
            reason: "timeout requires status check and same idempotency key",
        },
        SdkRetrySignal::ServiceRetryAfter(retry_after_ms) => SdkRetryClassification {
            retry_class: RetryClass::RetryAfter,
            safe_to_retry: true,
            retry_after_ms: Some(retry_after_ms),
            requires_same_idempotency_key: true,
            lifecycle_state: SdkCommandLifecycleState::RetryWait,
            reason: "service explicitly returned retry_after",
        },
        SdkRetrySignal::ServiceRetryable => SdkRetryClassification {
            retry_class: RetryClass::SafeRetry,
            safe_to_retry: true,
            retry_after_ms: None,
            requires_same_idempotency_key: true,
            lifecycle_state: SdkCommandLifecycleState::RetryWait,
            reason: "service explicitly marked retryable",
        },
        SdkRetrySignal::ServiceAccepted => SdkRetryClassification {
            retry_class: RetryClass::NotRetryable,
            safe_to_retry: false,
            retry_after_ms: None,
            requires_same_idempotency_key: false,
            lifecycle_state: SdkCommandLifecycleState::Accepted,
            reason: "accepted response is not a retry signal",
        },
        SdkRetrySignal::ServiceDenied => SdkRetryClassification {
            retry_class: RetryClass::NotRetryable,
            safe_to_retry: false,
            retry_after_ms: None,
            requires_same_idempotency_key: false,
            lifecycle_state: SdkCommandLifecycleState::Denied,
            reason: "denied response is terminal until caller correction",
        },
        SdkRetrySignal::ServiceDuplicate => SdkRetryClassification {
            retry_class: RetryClass::NotRetryable,
            safe_to_retry: false,
            retry_after_ms: None,
            requires_same_idempotency_key: false,
            lifecycle_state: SdkCommandLifecycleState::Duplicate,
            reason: "duplicate response resolves from service state",
        },
        SdkRetrySignal::ServiceTerminalFailure | SdkRetrySignal::LocalValidationFailure => {
            SdkRetryClassification {
                retry_class: RetryClass::NotRetryable,
                safe_to_retry: false,
                retry_after_ms: None,
                requires_same_idempotency_key: false,
                lifecycle_state: SdkCommandLifecycleState::Failed,
                reason: "terminal failure is not retryable without correction",
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkServiceErrorInput {
    pub reason_code: String,
    pub message: String,
    pub trace_id: Option<String>,
    pub audit_refs: Vec<String>,
    pub retry_class: RetryClass,
    pub correction_fields: Vec<String>,
    pub dependency_name: Option<String>,
    pub policy_refs: Vec<String>,
    pub schema_version: String,
}

pub fn decode_stable_overrid_error(
    input: SdkServiceErrorInput,
) -> Result<OverridErrorRecord, SdkError> {
    OverridErrorRecord::new(
        input.reason_code,
        input.message,
        input.trace_id,
        input.audit_refs,
        input.retry_class,
        input.correction_fields,
        input.dependency_name,
        input.policy_refs,
        &input.schema_version,
    )
}

#[allow(clippy::too_many_arguments)]
fn phase4_headers(
    schema_version: &str,
    trace_id: &str,
    idempotency_key: &str,
    command_type: &str,
    tenant_id: &str,
    actor_id: &str,
    timestamp_ms: u64,
    payload_hash: &str,
) -> Vec<(String, String)> {
    vec![
        (
            "x-overrid-schema-version".to_owned(),
            schema_version.to_owned(),
        ),
        ("x-overrid-trace-id".to_owned(), trace_id.to_owned()),
        (
            "x-overrid-idempotency-key".to_owned(),
            idempotency_key.to_owned(),
        ),
        ("x-overrid-command-type".to_owned(), command_type.to_owned()),
        ("x-overrid-tenant-id".to_owned(), tenant_id.to_owned()),
        ("x-overrid-actor-id".to_owned(), actor_id.to_owned()),
        (
            "x-overrid-command-timestamp-ms".to_owned(),
            timestamp_ms.to_string(),
        ),
        ("x-overrid-payload-hash".to_owned(), payload_hash.to_owned()),
        ("x-overrid-target".to_owned(), "overgate".to_owned()),
        ("x-overrid-sdk-name".to_owned(), SDK_NAME.to_owned()),
        ("x-overrid-sdk-version".to_owned(), SDK_VERSION.to_owned()),
        (
            "x-overrid-sdk-language-binding".to_owned(),
            SDK_LANGUAGE_BINDING.to_owned(),
        ),
        (
            "x-overrid-sdk-capability-profile".to_owned(),
            SDK_PHASE4_CAPABILITY_PROFILE.to_owned(),
        ),
        (
            "x-overrid-sdk-previous-capability-profile".to_owned(),
            SDK_PHASE3_CAPABILITY_PROFILE.to_owned(),
        ),
        (
            "x-overrid-generated-contract-revision".to_owned(),
            SDK_PHASE3_GENERATED_CONTRACT_REVISION.to_owned(),
        ),
    ]
}

fn require_phase4_non_empty(value: &str, field: &'static str) -> Result<(), SdkError> {
    if value.trim().is_empty() {
        return Err(SdkError::MissingRequiredField(field));
    }
    Ok(())
}

fn canonical_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('=', "\\=")
}

fn stable_phase4_hash(prefix: &str, value: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{prefix}_{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use overrid_contracts::SUPPORTED_SCHEMA_VERSION;

    fn phase4_payload() -> SdkCommandPayload {
        SdkCommandPayload::new(
            "tenant.create",
            vec![
                ("tenant_name".to_owned(), "local tenant".to_owned()),
                ("region".to_owned(), "local".to_owned()),
            ],
        )
        .unwrap()
    }

    fn phase4_command() -> SdkCommandEnvelope {
        build_command(SdkCommandBuildInput {
            family: BootstrapCommandFamily::Tenant,
            command_type: "tenant create".to_owned(),
            tenant_id: "tenant_local".to_owned(),
            actor_id: "actor_local".to_owned(),
            target_ref: "tenant:local".to_owned(),
            payload: phase4_payload(),
            expected_state: Some("prepared".to_owned()),
            reason: Some("phase4 test".to_owned()),
            idempotency_key: "idem_phase4".to_owned(),
            trace_id: "trace_phase4".to_owned(),
            timestamp_ms: 1_782_021_000_000,
            signature_ref: "sigref:local-dev:key-1:tenant_create".to_owned(),
            schema_version: SUPPORTED_SCHEMA_VERSION.to_owned(),
            command_class: SdkCommandClass::Phase1Mutating,
        })
        .unwrap()
    }

    fn phase4_error(reason_code: &str, retry_class: RetryClass) -> OverridErrorRecord {
        decode_stable_overrid_error(SdkServiceErrorInput {
            reason_code: reason_code.to_owned(),
            message: format!("{reason_code} preserved"),
            trace_id: Some("trace_phase4".to_owned()),
            audit_refs: vec![format!("audit:{reason_code}")],
            retry_class,
            correction_fields: vec!["fix_input".to_owned()],
            dependency_name: Some("overgate".to_owned()),
            policy_refs: vec!["policy:phase4".to_owned()],
            schema_version: SUPPORTED_SCHEMA_VERSION.to_owned(),
        })
        .unwrap()
    }

    #[test]
    fn phase4_command_builder_requires_fields_and_canonicalizes_payload() {
        let command = phase4_command();

        assert_eq!(command.envelope.tenant_id, "tenant_local");
        assert_eq!(command.envelope.actor_id, "actor_local");
        assert_eq!(command.envelope.trace_context.trace_id, "trace_phase4");
        assert_eq!(command.envelope.idempotency.key, "idem_phase4");
        assert_eq!(command.schema_version, SUPPORTED_SCHEMA_VERSION);
        assert!(command
            .canonical_payload
            .starts_with("payload_type=tenant.create\nfield:region=local"));
        assert!(command.payload_hash.starts_with("hash_"));
        assert!(command.headers.contains(&(
            "x-overrid-sdk-capability-profile".to_owned(),
            SDK_PHASE4_CAPABILITY_PROFILE.to_owned()
        )));

        let mut missing_timestamp = SdkCommandBuildInput {
            family: BootstrapCommandFamily::Tenant,
            command_type: "tenant create".to_owned(),
            tenant_id: "tenant_local".to_owned(),
            actor_id: "actor_local".to_owned(),
            target_ref: "tenant:local".to_owned(),
            payload: phase4_payload(),
            expected_state: None,
            reason: None,
            idempotency_key: "idem_phase4".to_owned(),
            trace_id: "trace_phase4".to_owned(),
            timestamp_ms: 0,
            signature_ref: "sigref:local-dev:key-1:tenant_create".to_owned(),
            schema_version: SUPPORTED_SCHEMA_VERSION.to_owned(),
            command_class: SdkCommandClass::Phase1Mutating,
        };
        assert!(matches!(
            build_command(missing_timestamp.clone()),
            Err(SdkError::MissingRequiredField("timestamp"))
        ));
        missing_timestamp.command_class = SdkCommandClass::ReadOnly;
        missing_timestamp.timestamp_ms = 1;
        assert!(matches!(
            build_command(missing_timestamp),
            Err(SdkError::MissingRequiredField("mutating command class"))
        ));
    }

    #[test]
    fn phase4_overgate_submission_preserves_refs_and_route() {
        let command = phase4_command();
        let config = ClientConfig::local_overgate("http://127.0.0.1:18080/overgate").unwrap();
        let submission = prepare_overgate_submission(&config.endpoint, command.clone()).unwrap();

        assert_eq!(submission.method, "POST");
        assert_eq!(submission.route, SDK_PHASE4_COMMAND_ROUTE);
        assert!(submission.overgate_only);
        assert_eq!(submission.body_hash, command.payload_hash);
        assert!(submission
            .headers
            .contains(&("x-overrid-target".to_owned(), "overgate".to_owned())));

        let response = SdkOvergateResponse::accepted(
            "trace_phase4",
            "accepted:tenant:create:1",
            "queue:pending:1",
            vec!["audit:overwatch:phase4".to_owned()],
        );
        let outcome = decode_overgate_submission(&command, response).unwrap();
        let acceptance = outcome.acceptance.unwrap();

        assert_eq!(outcome.status, SdkOvergateResponseStatus::Accepted);
        assert_eq!(outcome.trace_id, "trace_phase4");
        assert_eq!(acceptance.accepted_ref, "accepted:tenant:create:1");
        assert_eq!(acceptance.pending_state, "queue:pending:1");
        assert_eq!(acceptance.audit_refs, vec!["audit:overwatch:phase4"]);
        assert!(outcome
            .lifecycle
            .contains(&SdkCommandLifecycleState::Accepted));
    }

    #[test]
    fn phase4_idempotency_cache_bounds_replay_conflict_and_cleanup() {
        let command = phase4_command();
        let retry_response =
            SdkOvergateResponse::retryable("trace_phase4", 1_000, vec!["audit:retry".to_owned()]);
        let retry_decision = evaluate_idempotency_cache(&command, &retry_response, None).unwrap();

        assert_eq!(
            retry_decision.status,
            SdkIdempotencyCacheStatus::StoreInFlightRetry
        );
        assert_eq!(
            retry_decision.retention_ms,
            Some(SDK_PHASE4_IN_FLIGHT_RETRY_RETENTION_MS)
        );
        assert!(retry_decision.terminal_response_digest.is_none());
        assert!(retry_decision.user_clearable);

        let terminal_response = SdkOvergateResponse::completed(
            "trace_phase4",
            "accepted:tenant:create:1",
            vec!["audit:completed".to_owned()],
        );
        let terminal_decision =
            evaluate_idempotency_cache(&command, &terminal_response, Some(&retry_decision))
                .unwrap();
        assert_eq!(
            terminal_decision.status,
            SdkIdempotencyCacheStatus::StoreTerminalDigest
        );
        assert_eq!(
            terminal_decision.retention_ms,
            Some(SDK_PHASE4_PHASE1_TERMINAL_RETENTION_MS)
        );
        assert!(terminal_decision.terminal_response_digest.is_some());

        let mut conflicting = terminal_decision.clone();
        conflicting.request_hash = "hash_conflicting".to_owned();
        assert!(matches!(
            evaluate_idempotency_cache(&command, &terminal_response, Some(&conflicting)),
            Err(SdkError::IdempotencyConflict { key }) if key == "idem_phase4"
        ));

        let read_policy = phase4_idempotency_policy(SdkCommandClass::ReadOnly);
        assert!(!read_policy.read_only_cached);
        assert_eq!(read_policy.terminal_retention_ms, None);
        assert_eq!(
            phase4_idempotency_policy(SdkCommandClass::LongRunningWorkload).terminal_retention_ms,
            Some(SDK_PHASE4_WORKLOAD_REF_RETENTION_MS)
        );
        assert_eq!(
            phase4_idempotency_policy(SdkCommandClass::SecuritySensitive).terminal_retention_ms,
            Some(SDK_PHASE4_SECURITY_SENSITIVE_RETENTION_MS)
        );

        let cleared = clear_phase4_idempotency_cache("idem_phase4").unwrap();
        assert_eq!(cleared.status, SdkIdempotencyCacheStatus::ClearedByDevReset);
        assert_eq!(cleared.cleanup_control, SDK_PHASE4_CLEANUP_CONTROL);
    }

    #[test]
    fn phase4_retry_classifier_only_retries_safe_or_explicit_signals() {
        let transport = classify_phase4_retry(SdkRetrySignal::TransportFailure);
        assert_eq!(transport.retry_class, RetryClass::SafeRetry);
        assert!(transport.safe_to_retry);
        assert!(transport.requires_same_idempotency_key);

        let retry_after = classify_phase4_retry(SdkRetrySignal::ServiceRetryAfter(2_500));
        assert_eq!(retry_after.retry_class, RetryClass::RetryAfter);
        assert_eq!(retry_after.retry_after_ms, Some(2_500));
        assert_eq!(
            retry_after.lifecycle_state,
            SdkCommandLifecycleState::RetryWait
        );

        for signal in [
            SdkRetrySignal::ServiceAccepted,
            SdkRetrySignal::ServiceDenied,
            SdkRetrySignal::ServiceDuplicate,
            SdkRetrySignal::ServiceTerminalFailure,
            SdkRetrySignal::LocalValidationFailure,
        ] {
            let classification = classify_phase4_retry(signal);
            assert!(!classification.safe_to_retry);
            assert!(!classification.requires_same_idempotency_key);
        }
    }

    #[test]
    fn phase4_error_decoder_preserves_stable_service_fields() {
        for (reason_code, retry_class) in [
            ("invalid_signature", RetryClass::NotRetryable),
            ("duplicate_idempotency_conflict", RetryClass::NotRetryable),
            ("schema_version_unsupported", RetryClass::NotRetryable),
            ("revoked_credential", RetryClass::NotRetryable),
            ("policy_denial", RetryClass::OperatorReview),
        ] {
            let error = phase4_error(reason_code, retry_class);
            assert_eq!(error.reason_code, reason_code);
            assert_eq!(error.trace_id.as_deref(), Some("trace_phase4"));
            assert_eq!(error.audit_refs, vec![format!("audit:{reason_code}")]);
            assert_eq!(error.retry_class, retry_class);
            assert_eq!(error.dependency_name.as_deref(), Some("overgate"));
            assert_eq!(error.policy_refs, vec!["policy:phase4"]);
            assert_eq!(error.schema_version.raw(), SUPPORTED_SCHEMA_VERSION);
        }
    }

    #[test]
    fn phase4_lifecycle_requires_explicit_service_response_for_terminal_states() {
        let command = phase4_command();
        let denied = SdkOvergateResponse::denied(
            "trace_phase4",
            phase4_error("policy_denial", RetryClass::NotRetryable),
        );
        let outcome = decode_overgate_submission(&command, denied.clone()).unwrap();
        assert_eq!(outcome.status, SdkOvergateResponseStatus::Denied);
        assert!(outcome
            .lifecycle
            .iter()
            .any(|state| state.is_terminal() && *state == SdkCommandLifecycleState::Denied));

        assert!(matches!(
            decode_overgate_submission(&command, denied.without_service_confirmation()),
            Err(SdkError::InvalidLifecycleTransition {
                from: "submitted",
                to: "denied"
            })
        ));

        let trace_mismatch = SdkOvergateResponse::duplicate(
            "trace_other",
            "accepted:tenant:create:1",
            vec!["audit:duplicate".to_owned()],
        );
        assert!(matches!(
            decode_overgate_submission(&command, trace_mismatch),
            Err(SdkError::TraceMismatch { expected, actual })
                if expected == "trace_phase4" && actual == "trace_other"
        ));
    }
}
