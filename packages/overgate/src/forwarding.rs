use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use serde::Serialize;

use crate::canonical::{stable_hash_ref, stable_short_token};
use crate::envelope::CommandEnvelope;
use crate::errors::OvergateError;
use crate::idempotency::IdempotencyOutcome;
use crate::prechecks::{CommandClassAdmission, PrecheckOutcome};

pub const PHASE8_FORWARDING_ADAPTER_ID: &str = "overgate.phase8.local_forwarding_adapter";
pub const PHASE8_TARGET_REGISTRY_REF: &str = "forwarding_target_registry:overgate:phase8";
pub const PHASE8_OVERQUEUE_CONTRACT_REF: &str = "overqueue.dispatch.v0";
pub const PHASE8_PRODUCT_CLIENT_CHECKLIST_REF: &str = "product_client_flows:overgate:phase8";

#[derive(Debug, Clone, Default)]
pub struct ForwardingStore {
    inner: Arc<Mutex<ForwardingState>>,
}

#[derive(Debug, Default)]
struct ForwardingState {
    records: HashMap<String, ForwardingRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardingInput {
    pub command_id: String,
    pub command_type: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub trace_id: String,
    pub request_id: String,
    pub idempotency_record_ref: String,
    pub idempotency_replayed: bool,
    pub request_hash_ref: String,
    pub payload_hash_ref: String,
    pub schema_version: String,
    pub audit_refs: Vec<String>,
    pub precheck: PrecheckOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ForwardingOutcome {
    pub adapter_id: &'static str,
    pub outcome_state: &'static str,
    pub reason_code: &'static str,
    pub target_registry_ref: &'static str,
    pub target: DownstreamTarget,
    pub record: ForwardingRecord,
    pub overqueue_item: Option<OverqueueWorkItem>,
    pub synchronous_completion: Option<SynchronousCompletion>,
    pub retry: RetryMetadata,
    pub status_projection: ForwardingStatusProjection,
    pub product_client_flows: ProductClientFlowChecklist,
    pub raw_private_payload_written: bool,
    pub direct_downstream_state_write: bool,
    pub external_queue_dependency: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ForwardingRecord {
    pub record_id: String,
    pub idempotency_record_ref: String,
    pub command_ref: String,
    pub trace_id: String,
    pub tenant_ref: String,
    pub actor_ref: String,
    pub target_ref: String,
    pub owner_service: &'static str,
    pub dispatch_mode: &'static str,
    pub forwarding_state: &'static str,
    pub status_projection: &'static str,
    pub request_hash_ref: String,
    pub payload_hash_ref: String,
    pub audit_refs: Vec<String>,
    pub queue_item_ref: Option<String>,
    pub retry_ref: String,
    pub terminal_reason: Option<&'static str>,
    pub no_direct_downstream_state_write: bool,
    pub downstream_state_owner: &'static str,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DownstreamTarget {
    pub target_id: String,
    pub owner_service: &'static str,
    pub command_class: &'static str,
    pub dispatch_mode: &'static str,
    pub queue_route: Option<&'static str>,
    pub api_route: Option<&'static str>,
    pub required_schema_version: &'static str,
    pub permission_requirement: &'static str,
    pub policy_requirement: &'static str,
    pub retry_behavior: &'static str,
    pub failover_behavior: &'static str,
    pub audit_mapping_ref: String,
    pub tenant_isolation_rule: &'static str,
    pub downstream_state_owner: &'static str,
    pub direct_downstream_state_write: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OverqueueWorkItem {
    pub queue_item_ref: String,
    pub overqueue_contract_ref: &'static str,
    pub durable_state: &'static str,
    pub queue_route: &'static str,
    pub target_service: &'static str,
    pub command_ref: String,
    pub trace_id: String,
    pub idempotency_record_ref: String,
    pub request_hash_ref: String,
    pub payload_hash_ref: String,
    pub audit_refs: Vec<String>,
    pub retry_ref: String,
    pub native_overqueue_boundary: bool,
    pub external_queue_dependency: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SynchronousCompletion {
    pub completion_ref: String,
    pub target_service: &'static str,
    pub api_route: &'static str,
    pub completed_before_response: bool,
    pub waited_on_runtime_service: bool,
    pub execution_side_effect: bool,
    pub storage_side_effect: bool,
    pub accounting_side_effect: bool,
    pub native_app_side_effect: bool,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetryMetadata {
    pub retry_ref: String,
    pub retryable: bool,
    pub retry_state: &'static str,
    pub retry_count: u32,
    pub max_attempts: u32,
    pub next_attempt_ref: Option<String>,
    pub dead_letter_ref: Option<String>,
    pub terminal_reason: Option<&'static str>,
    pub safe_overqueue_retry: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ForwardingStatusProjection {
    pub status_ref: String,
    pub current_state: &'static str,
    pub forwarding_state: &'static str,
    pub status_reason_code: &'static str,
    pub caller_visible: bool,
    pub retry_state: &'static str,
    pub completed_before_response: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProductClientFlowChecklist {
    pub checklist_ref: &'static str,
    pub flows: Vec<ProductClientFlow>,
    pub bypass_internal_apis: bool,
    pub signing_required: bool,
    pub idempotency_required: bool,
    pub trace_id_required: bool,
    pub stable_errors_required: bool,
    pub audit_refs_required: bool,
    pub forwarding_required: bool,
    pub direct_downstream_state_writes_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProductClientFlow {
    pub client: &'static str,
    pub entry_route: &'static str,
    pub required_contract: &'static str,
    pub internal_api_bypass_allowed: bool,
}

impl ForwardingInput {
    pub fn from_parts(
        envelope: &CommandEnvelope,
        request_id: String,
        idempotency: &IdempotencyOutcome,
        precheck: &PrecheckOutcome,
        audit_refs: Vec<String>,
    ) -> Self {
        Self {
            command_id: envelope.command_id.clone(),
            command_type: envelope.command_type.clone(),
            tenant_id: envelope.tenant_id.clone(),
            actor_id: envelope.actor_id.clone(),
            trace_id: envelope.trace_id.clone(),
            request_id,
            idempotency_record_ref: idempotency.record.record_id.clone(),
            idempotency_replayed: idempotency.replayed,
            request_hash_ref: envelope.request_hash.clone(),
            payload_hash_ref: envelope.payload_hash.clone(),
            schema_version: envelope.schema_version.clone(),
            audit_refs,
            precheck: precheck.clone(),
        }
    }
}

impl ForwardingStore {
    pub fn forward_after_acceptance(
        &self,
        input: ForwardingInput,
    ) -> Result<ForwardingOutcome, OvergateError> {
        if product_client_bypass_requested(&input.command_type) {
            return Err(OvergateError::product_client_bypass_denied(vec![
                "route:POST /v1/commands".to_owned(),
                PHASE8_PRODUCT_CLIENT_CHECKLIST_REF.to_owned(),
                "internal_api_bypass:false".to_owned(),
            ]));
        }

        if input.idempotency_replayed {
            let state = self.lock_state();
            if let Some(record) = state.records.get(&input.idempotency_record_ref) {
                return Ok(ForwardingOutcome::from_replayed_record(
                    &input,
                    target_for(&input).ok_or_else(|| target_error(&input))?,
                    record.clone(),
                ));
            }
        }

        let target = target_for(&input).ok_or_else(|| target_error(&input))?;
        if !target.registry_entry_valid() {
            return Err(target_error(&input));
        }

        let outcome = ForwardingOutcome::from_input(input.clone(), target);
        let mut state = self.lock_state();
        state
            .records
            .insert(input.idempotency_record_ref, outcome.record.clone());
        Ok(outcome)
    }

    fn lock_state(&self) -> MutexGuard<'_, ForwardingState> {
        self.inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl ForwardingOutcome {
    fn from_input(input: ForwardingInput, target: DownstreamTarget) -> Self {
        let failure = DispatchFailure::from_command_type(&input.command_type);
        let queue_item = if target.uses_overqueue() || failure.retry_through_overqueue() {
            Some(OverqueueWorkItem::from_input(&input, &target))
        } else {
            None
        };
        let synchronous_completion = target.is_synchronous().then(|| {
            SynchronousCompletion::from_input(
                &input,
                &target,
                "overgate.synchronous_forwarding_completed_phase8",
            )
        });
        let retry = RetryMetadata::from_failure(&input, &target, failure);
        let status_projection = ForwardingStatusProjection::from_target(&input, &target, failure);
        let forwarding_state = status_projection.forwarding_state;
        let reason_code = status_projection.status_reason_code;
        let outcome_state = if failure.is_failure() {
            "failed_after_acceptance_phase8"
        } else if target.uses_overqueue() {
            "overqueue_dispatch_recorded_phase8"
        } else if target.is_synchronous() {
            "synchronous_forwarding_completed_phase8"
        } else {
            "forwarding_projection_recorded_phase8"
        };
        let record = ForwardingRecord::from_parts(
            &input,
            &target,
            forwarding_state,
            status_projection.current_state,
            queue_item.as_ref().map(|item| item.queue_item_ref.clone()),
            retry.retry_ref.clone(),
            retry.terminal_reason,
            reason_code,
            failure,
        );

        Self {
            adapter_id: PHASE8_FORWARDING_ADAPTER_ID,
            outcome_state,
            reason_code,
            target_registry_ref: PHASE8_TARGET_REGISTRY_REF,
            target,
            record,
            overqueue_item: queue_item,
            synchronous_completion,
            retry,
            status_projection,
            product_client_flows: ProductClientFlowChecklist::default(),
            raw_private_payload_written: false,
            direct_downstream_state_write: false,
            external_queue_dependency: "none_native_overqueue_contract_only",
        }
    }

    fn from_replayed_record(
        input: &ForwardingInput,
        target: DownstreamTarget,
        record: ForwardingRecord,
    ) -> Self {
        let status_projection = ForwardingStatusProjection {
            status_ref: format!(
                "status:overgate:phase8:replay:{}",
                stable_short_token(&[
                    input.command_id.as_str(),
                    input.trace_id.as_str(),
                    input.idempotency_record_ref.as_str(),
                ])
            ),
            current_state: record.status_projection,
            forwarding_state: record.forwarding_state,
            status_reason_code: "overgate.forwarding_replayed_phase8",
            caller_visible: true,
            retry_state: "replay_uses_existing_forwarding_record",
            completed_before_response: record.forwarding_state == "synchronous_completed_phase8",
        };
        Self {
            adapter_id: PHASE8_FORWARDING_ADAPTER_ID,
            outcome_state: "idempotency_replay_no_redelivery_phase8",
            reason_code: "overgate.forwarding_replayed_phase8",
            target_registry_ref: PHASE8_TARGET_REGISTRY_REF,
            target,
            record,
            overqueue_item: None,
            synchronous_completion: None,
            retry: RetryMetadata {
                retry_ref: format!(
                    "retry:overgate:phase8:replay:{}",
                    stable_short_token(&[input.idempotency_record_ref.as_str()])
                ),
                retryable: false,
                retry_state: "not_redelivered_on_idempotency_replay",
                retry_count: 0,
                max_attempts: 0,
                next_attempt_ref: None,
                dead_letter_ref: None,
                terminal_reason: None,
                safe_overqueue_retry: false,
            },
            status_projection,
            product_client_flows: ProductClientFlowChecklist::default(),
            raw_private_payload_written: false,
            direct_downstream_state_write: false,
            external_queue_dependency: "none_native_overqueue_contract_only",
        }
    }
}

impl ForwardingRecord {
    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        input: &ForwardingInput,
        target: &DownstreamTarget,
        forwarding_state: &'static str,
        status_projection: &'static str,
        queue_item_ref: Option<String>,
        retry_ref: String,
        terminal_reason: Option<&'static str>,
        reason_code: &'static str,
        failure: DispatchFailure,
    ) -> Self {
        let event_type = if failure.is_failure() {
            "overgate.forwarding_failed"
        } else {
            "overgate.command_forwarded"
        };
        let forwarding_audit_ref = format!(
            "audit:overwatch:overgate:phase8:{}:{}",
            event_type.replace('.', "_"),
            stable_short_token(&[
                input.command_id.as_str(),
                input.trace_id.as_str(),
                input.request_id.as_str(),
            ])
        );
        let mut audit_refs = input.audit_refs.clone();
        if !audit_refs.contains(&forwarding_audit_ref) {
            audit_refs.push(forwarding_audit_ref);
        }
        Self {
            record_id: format!(
                "forwarding:overgate:phase8:{}",
                stable_short_token(&[
                    input.idempotency_record_ref.as_str(),
                    target.target_id.as_str(),
                    input.trace_id.as_str(),
                ])
            ),
            idempotency_record_ref: input.idempotency_record_ref.clone(),
            command_ref: input.command_id.clone(),
            trace_id: input.trace_id.clone(),
            tenant_ref: hashed_ref("tenant_ref:overgate:phase8", &input.tenant_id),
            actor_ref: hashed_ref("actor_ref:overgate:phase8", &input.actor_id),
            target_ref: target.target_id.clone(),
            owner_service: target.owner_service,
            dispatch_mode: target.dispatch_mode,
            forwarding_state,
            status_projection,
            request_hash_ref: input.request_hash_ref.clone(),
            payload_hash_ref: input.payload_hash_ref.clone(),
            audit_refs,
            queue_item_ref,
            retry_ref,
            terminal_reason,
            no_direct_downstream_state_write: true,
            downstream_state_owner: target.downstream_state_owner,
            reason_code,
        }
    }
}

impl DownstreamTarget {
    fn registry_entry_valid(&self) -> bool {
        !self.target_id.is_empty()
            && !self.owner_service.is_empty()
            && !self.command_class.is_empty()
            && !self.dispatch_mode.is_empty()
            && !self.required_schema_version.is_empty()
            && !self.permission_requirement.is_empty()
            && !self.retry_behavior.is_empty()
            && !self.audit_mapping_ref.is_empty()
            && !self.tenant_isolation_rule.is_empty()
            && !self.direct_downstream_state_write
    }

    fn uses_overqueue(&self) -> bool {
        self.dispatch_mode == "overqueue_durable_dispatch_phase8"
    }

    fn is_synchronous(&self) -> bool {
        self.dispatch_mode == "synchronous_phase1_forwarding_phase8"
            || self.dispatch_mode == "synchronous_operator_forwarding_phase8"
            || self.dispatch_mode == "bodyless_read_no_forwarding_phase8"
    }
}

impl OverqueueWorkItem {
    fn from_input(input: &ForwardingInput, target: &DownstreamTarget) -> Self {
        let queue_route = target
            .queue_route
            .unwrap_or("overqueue://retry-safe-handoff");
        let queue_item_ref = format!(
            "overqueue:item:overgate:phase8:{}",
            stable_short_token(&[
                input.command_id.as_str(),
                input.trace_id.as_str(),
                target.target_id.as_str(),
            ])
        );
        Self {
            queue_item_ref,
            overqueue_contract_ref: PHASE8_OVERQUEUE_CONTRACT_REF,
            durable_state: "durable_pending_work_phase8",
            queue_route,
            target_service: target.owner_service,
            command_ref: input.command_id.clone(),
            trace_id: input.trace_id.clone(),
            idempotency_record_ref: input.idempotency_record_ref.clone(),
            request_hash_ref: input.request_hash_ref.clone(),
            payload_hash_ref: input.payload_hash_ref.clone(),
            audit_refs: input.audit_refs.clone(),
            retry_ref: retry_ref(input, target),
            native_overqueue_boundary: true,
            external_queue_dependency: "none_native_overqueue_contract_only",
        }
    }
}

impl SynchronousCompletion {
    fn from_input(
        input: &ForwardingInput,
        target: &DownstreamTarget,
        reason_code: &'static str,
    ) -> Self {
        Self {
            completion_ref: format!(
                "sync_completion:overgate:phase8:{}",
                stable_short_token(&[
                    input.command_id.as_str(),
                    target.target_id.as_str(),
                    input.trace_id.as_str(),
                ])
            ),
            target_service: target.owner_service,
            api_route: target.api_route.unwrap_or("/v1/commands/synthetic"),
            completed_before_response: true,
            waited_on_runtime_service: false,
            execution_side_effect: false,
            storage_side_effect: false,
            accounting_side_effect: false,
            native_app_side_effect: false,
            reason_code,
        }
    }
}

impl RetryMetadata {
    fn from_failure(
        input: &ForwardingInput,
        target: &DownstreamTarget,
        failure: DispatchFailure,
    ) -> Self {
        let retry_ref = retry_ref(input, target);
        let dead_letter_ref = failure.dead_letter_state().map(|state| {
            format!(
                "dead_letter:overgate:phase8:{}:{}",
                state,
                stable_short_token(&[input.command_id.as_str(), input.trace_id.as_str()])
            )
        });
        Self {
            retry_ref: retry_ref.clone(),
            retryable: failure.retryable(),
            retry_state: failure.retry_state(),
            retry_count: u32::from(failure.is_failure()),
            max_attempts: if failure.retryable() { 3 } else { 0 },
            next_attempt_ref: failure.retryable().then(|| {
                format!(
                    "retry_after:overgate:phase8:{}",
                    stable_short_token(&[retry_ref.as_str(), input.trace_id.as_str()])
                )
            }),
            dead_letter_ref,
            terminal_reason: failure.terminal_reason(),
            safe_overqueue_retry: failure.retry_through_overqueue(),
        }
    }
}

impl ForwardingStatusProjection {
    fn from_target(
        input: &ForwardingInput,
        target: &DownstreamTarget,
        failure: DispatchFailure,
    ) -> Self {
        let (current_state, forwarding_state, reason_code, completed_before_response) =
            if failure.is_failure() {
                (
                    "failed_after_acceptance",
                    failure.forwarding_state(),
                    "overgate.forwarding_failed_after_acceptance",
                    false,
                )
            } else if target.uses_overqueue() {
                (
                    "pending_overqueue_dispatch",
                    "overqueue_pending_phase8",
                    "overgate.overqueue_dispatch_enqueued_phase8",
                    false,
                )
            } else if target.dispatch_mode == "bodyless_read_no_forwarding_phase8" {
                (
                    "completed_without_forwarding",
                    "bodyless_read_no_forwarding_phase8",
                    "overgate.no_forwarding_required_phase8",
                    true,
                )
            } else {
                (
                    "completed_synchronously",
                    "synchronous_completed_phase8",
                    "overgate.synchronous_forwarding_completed_phase8",
                    true,
                )
            };
        Self {
            status_ref: format!(
                "status:overgate:phase8:{}",
                stable_short_token(&[
                    input.command_id.as_str(),
                    input.trace_id.as_str(),
                    forwarding_state,
                ])
            ),
            current_state,
            forwarding_state,
            status_reason_code: reason_code,
            caller_visible: true,
            retry_state: failure.retry_state(),
            completed_before_response,
        }
    }
}

impl Default for ProductClientFlowChecklist {
    fn default() -> Self {
        let flows = [
            "sdk",
            "cli",
            "admin_ui",
            "docdex_adapter",
            "mcoda_adapter",
            "codali_adapter",
            "native_app",
            "mobile_client",
            "node_agent",
            "service_account",
        ]
        .iter()
        .map(|client| ProductClientFlow {
            client: *client,
            entry_route: "POST /v1/commands",
            required_contract: "signed_command_envelope_with_idempotency_trace_audit_forwarding",
            internal_api_bypass_allowed: false,
        })
        .collect();
        Self {
            checklist_ref: PHASE8_PRODUCT_CLIENT_CHECKLIST_REF,
            flows,
            bypass_internal_apis: false,
            signing_required: true,
            idempotency_required: true,
            trace_id_required: true,
            stable_errors_required: true,
            audit_refs_required: true,
            forwarding_required: true,
            direct_downstream_state_writes_allowed: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DispatchFailure {
    None,
    DownstreamUnavailable,
    Timeout,
    FinalFailure,
    Cancelled,
    DeadLetter,
}

impl DispatchFailure {
    fn from_command_type(command_type: &str) -> Self {
        if command_type.contains("forwarding.dead_letter") || command_type.contains("dead_letter") {
            Self::DeadLetter
        } else if command_type.contains("forwarding.cancelled")
            || command_type.contains("cancelled")
        {
            Self::Cancelled
        } else if command_type.contains("forwarding.final_failure")
            || command_type.contains("final_failure")
        {
            Self::FinalFailure
        } else if command_type.contains("forwarding.timeout") || command_type.contains("timeout") {
            Self::Timeout
        } else if command_type.contains("downstream.unavailable")
            || command_type.contains("downstream_unavailable")
        {
            Self::DownstreamUnavailable
        } else {
            Self::None
        }
    }

    fn is_failure(self) -> bool {
        self != Self::None
    }

    fn retryable(self) -> bool {
        matches!(self, Self::DownstreamUnavailable | Self::Timeout)
    }

    fn retry_through_overqueue(self) -> bool {
        self.retryable()
    }

    fn retry_state(self) -> &'static str {
        match self {
            Self::None => "retry_not_required",
            Self::DownstreamUnavailable => "retry_scheduled_through_overqueue",
            Self::Timeout => "retry_scheduled_after_timeout",
            Self::FinalFailure => "terminal_failure_no_retry",
            Self::Cancelled => "cancelled_no_retry",
            Self::DeadLetter => "dead_lettered_no_retry",
        }
    }

    fn forwarding_state(self) -> &'static str {
        match self {
            Self::None => "forwarded_phase8",
            Self::DownstreamUnavailable | Self::Timeout => "retry_scheduled_phase8",
            Self::FinalFailure => "terminal_failure_phase8",
            Self::Cancelled => "cancelled_after_acceptance_phase8",
            Self::DeadLetter => "dead_lettered_phase8",
        }
    }

    fn terminal_reason(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::DownstreamUnavailable => Some("downstream_unavailable_after_acceptance"),
            Self::Timeout => Some("downstream_timeout_after_acceptance"),
            Self::FinalFailure => Some("downstream_final_failure_after_acceptance"),
            Self::Cancelled => Some("forwarding_cancelled_after_acceptance"),
            Self::DeadLetter => Some("overqueue_dead_letter_after_acceptance"),
        }
    }

    fn dead_letter_state(self) -> Option<&'static str> {
        match self {
            Self::FinalFailure => Some("terminal_failure"),
            Self::Cancelled => Some("cancelled"),
            Self::DeadLetter => Some("dead_lettered"),
            _ => None,
        }
    }
}

pub fn target_registry() -> Vec<DownstreamTarget> {
    [
        "overgate.phase8.read.status",
        "overgate.phase8.tenant.profile_update",
        "overgate.phase8.queue.workload.submit",
        "overgate.phase8.policy.evaluate",
        "overgate.phase8.accounting.quota.reserve",
        "overgate.phase8.storage.namespace.bind",
        "overgate.phase8.native_app.side_effect",
        "overgate.phase8.admin.expire",
        "overgate.phase8.break_glass.audit_override",
    ]
    .iter()
    .filter_map(|command_type| {
        let precheck = CommandClassAdmission::from_command_type(command_type);
        target_from_parts(command_type, &precheck)
    })
    .collect()
}

pub fn validate_target_registry() -> bool {
    let targets = target_registry();
    let classes = targets
        .iter()
        .map(|target| target.command_class)
        .collect::<std::collections::HashSet<_>>();
    let required = [
        "low_risk_read",
        "phase1_control_plane_mutation",
        "queue_producing_workload",
        "policy_heavy",
        "accounting_affecting",
        "storage_namespace",
        "native_app_side_effect",
        "admin",
        "break_glass",
    ];
    required.iter().all(|class| classes.contains(class))
        && targets.iter().all(DownstreamTarget::registry_entry_valid)
}

fn target_for(input: &ForwardingInput) -> Option<DownstreamTarget> {
    if input.command_type.contains("unregistered") || input.command_type.contains("missing_target")
    {
        return None;
    }
    target_from_parts(&input.command_type, &input.precheck.command_class)
}

fn target_from_parts(
    command_type: &str,
    class: &CommandClassAdmission,
) -> Option<DownstreamTarget> {
    let command_class = class.command_class;
    let (owner_service, dispatch_mode, queue_route, api_route, downstream_state_owner) =
        match command_class {
            "low_risk_read" => (
                "service:overgate",
                "bodyless_read_no_forwarding_phase8",
                None,
                Some("/v1/commands/status-projection"),
                "overgate_status_projection_only",
            ),
            "phase1_control_plane_mutation" => (
                phase1_owner_service(command_type),
                "synchronous_phase1_forwarding_phase8",
                None,
                Some(phase1_api_route(command_type)),
                phase1_downstream_state_owner(command_type),
            ),
            "queue_producing_workload" => (
                "service:overqueue",
                "overqueue_durable_dispatch_phase8",
                Some("overqueue://workload-submission"),
                None,
                "downstream_workload_services",
            ),
            "policy_heavy" => (
                "service:overguard",
                "overqueue_durable_dispatch_phase8",
                Some("overqueue://policy-heavy-command"),
                None,
                "overguard_policy_truth",
            ),
            "accounting_affecting" => (
                "service:overqueue",
                "overqueue_durable_dispatch_phase8",
                Some("overqueue://accounting-handoff"),
                None,
                "accounting_services_own_settlement",
            ),
            "storage_namespace" => (
                "service:overqueue",
                "overqueue_durable_dispatch_phase8",
                Some("overqueue://storage-namespace-handoff"),
                None,
                "storage_namespace_services",
            ),
            "native_app_side_effect" => (
                "service:overqueue",
                "overqueue_durable_dispatch_phase8",
                Some("overqueue://native-app-side-effect"),
                None,
                "native_app_service",
            ),
            "admin" => (
                "service:overgate",
                "synchronous_operator_forwarding_phase8",
                None,
                Some("/v1/admin/commands"),
                "overgate_operator_records",
            ),
            "break_glass" => (
                "service:overwatch",
                "synchronous_operator_forwarding_phase8",
                None,
                Some("/v1/audit/break-glass"),
                "overwatch_break_glass_evidence",
            ),
            _ => return None,
        };
    let target_id = format!(
        "target:overgate:phase8:{}:{}",
        command_class,
        stable_short_token(&[command_type, owner_service])
    );
    Some(DownstreamTarget {
        audit_mapping_ref: format!(
            "audit_mapping:overgate:phase8:{}",
            stable_short_token(&[target_id.as_str(), command_type])
        ),
        target_id,
        owner_service,
        command_class,
        dispatch_mode,
        queue_route,
        api_route,
        required_schema_version: "shared-schema-package.v0.1",
        permission_requirement: permission_requirement(command_class),
        policy_requirement: policy_requirement(command_class),
        retry_behavior: retry_behavior(command_class),
        failover_behavior: failover_behavior(command_class),
        tenant_isolation_rule: "tenant_actor_service_account_scoped",
        downstream_state_owner,
        direct_downstream_state_write: false,
    })
}

fn target_error(input: &ForwardingInput) -> OvergateError {
    OvergateError::forwarding_target_unregistered(vec![
        PHASE8_TARGET_REGISTRY_REF.to_owned(),
        format!(
            "command_class:{}",
            input.precheck.command_class.command_class
        ),
        format!("command_type:{}", input.command_type),
    ])
}

fn phase1_owner_service(command_type: &str) -> &'static str {
    if command_type.contains("credential") || command_type.contains("key") {
        "service:overkey_lite"
    } else if command_type.contains("identity")
        || command_type.contains("actor")
        || command_type.contains("pass")
    {
        "service:overpass"
    } else if command_type.contains("manifest") || command_type.contains("registry") {
        "service:overregistry"
    } else if command_type.contains("trace")
        || command_type.contains("status")
        || command_type.contains("limit")
        || command_type.contains("synthetic")
        || command_type.contains("noop")
    {
        "service:overgate"
    } else {
        "service:overtenant"
    }
}

fn phase1_api_route(command_type: &str) -> &'static str {
    match phase1_owner_service(command_type) {
        "service:overkey_lite" => "/v1/credentials/admit",
        "service:overpass" => "/v1/actors/admit",
        "service:overregistry" => "/v1/manifests/admit",
        "service:overgate" => "/v1/commands/synthetic",
        _ => "/v1/tenants/admit",
    }
}

fn phase1_downstream_state_owner(command_type: &str) -> &'static str {
    match phase1_owner_service(command_type) {
        "service:overkey_lite" => "overkey_lite_credential_state",
        "service:overpass" => "overpass_actor_state",
        "service:overregistry" => "overregistry_manifest_state",
        "service:overgate" => "overgate_status_projection_only",
        _ => "overtenant_tenant_state",
    }
}

fn permission_requirement(command_class: &str) -> &'static str {
    match command_class {
        "low_risk_read" => "tenant_read_or_public_bodyless_read",
        "phase1_control_plane_mutation" => "signed_tenant_actor_or_service_account",
        "admin" | "break_glass" => "signed_operator_or_system_service",
        _ => "signed_tenant_actor_service_account_with_policy_and_quota",
    }
}

fn policy_requirement(command_class: &str) -> &'static str {
    match command_class {
        "low_risk_read" | "phase1_control_plane_mutation" => {
            "policy_not_required_for_phase8_target"
        }
        _ => "overguard_policy_handoff_required",
    }
}

fn retry_behavior(command_class: &str) -> &'static str {
    match command_class {
        "low_risk_read" | "phase1_control_plane_mutation" | "admin" | "break_glass" => {
            "no_overqueue_retry_for_synchronous_phase8"
        }
        _ => "retry_safe_native_overqueue_metadata",
    }
}

fn failover_behavior(command_class: &str) -> &'static str {
    match command_class {
        "phase1_control_plane_mutation" => "synchronous_fail_closed_before_downstream_state_write",
        "admin" | "break_glass" => "operator_fail_closed_with_overwatch_audit",
        "low_risk_read" => "no_forwarding_failover_required",
        _ => "durable_overqueue_retry_or_dead_letter",
    }
}

fn retry_ref(input: &ForwardingInput, target: &DownstreamTarget) -> String {
    format!(
        "retry:overgate:phase8:{}",
        stable_short_token(&[
            input.command_id.as_str(),
            input.trace_id.as_str(),
            target.target_id.as_str(),
        ])
    )
}

fn product_client_bypass_requested(command_type: &str) -> bool {
    command_type.contains("bypass_internal_api")
        || command_type.contains("internal_api_bypass")
        || command_type.contains("direct_downstream_write")
}

fn hashed_ref(prefix: &str, value: &str) -> String {
    stable_hash_ref(&[prefix, value])
}
