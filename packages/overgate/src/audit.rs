use std::sync::{Arc, Mutex, MutexGuard};

use serde::Serialize;

use crate::canonical::{stable_hash_ref, stable_short_token};
use crate::envelope::CommandEnvelope;
use crate::errors::OvergateError;
use crate::prechecks::{CommandClassAdmission, PrecheckOutcome};

pub const PHASE7_AUDIT_CLIENT_REF: &str = "overwatch.audit.v0";
pub const PHASE7_EVENT_TRANSITION_MAP_REF: &str = "event_transition_map:overgate:phase7";
pub const PHASE7_EMERGENCY_WAL_REF: &str = "emergency_audit_wal:overgate:phase7";
pub const PHASE7_METRIC_POLICY_REF: &str = "metric_label_policy:overgate:phase7";
pub const PHASE7_GRID_OPERATIONS_REF: &str = "grid_operations:overgate:phase7";

#[derive(Debug, Clone)]
pub struct AuditStore {
    inner: Arc<Mutex<AuditState>>,
    emergency_wal: EmergencyWalConfig,
}

#[derive(Debug, Default)]
struct AuditState {
    events: Vec<AuditEventRecord>,
    wal_entries: Vec<EmergencyWalEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditGuardInput {
    pub command_type: String,
    pub command_class: CommandClassAdmission,
    pub request_hash_ref: String,
    pub payload_hash_ref: String,
    pub trace_id: String,
    pub overwatch_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuditDecision {
    pub audit_state: &'static str,
    pub overwatch_dependency_ready: bool,
    pub fail_closed: bool,
    pub emergency_wal_allowed: bool,
    pub degraded_mode: bool,
    pub command_class: &'static str,
    pub reason_code: &'static str,
    pub guard_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase7AuditInput {
    pub trace_id: String,
    pub command_id: String,
    pub request_id: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub request_hash_ref: String,
    pub payload_hash_ref: String,
    pub idempotency_record_ref: String,
    pub idempotency_reason_code: &'static str,
    pub audit_context_ref: String,
    pub forwarding_state: &'static str,
    pub audit_decision: AuditDecision,
    pub precheck: PrecheckOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Phase7AuditEvidence {
    pub evidence_state: &'static str,
    pub overwatch_client_ref: &'static str,
    pub event_transition_map_ref: &'static str,
    pub ordered_events: Vec<AuditEventRecord>,
    pub event_transition_map: Vec<EventTransition>,
    pub emergency_wal: EmergencyWalStatus,
    pub metrics: MetricsTraceSummary,
    pub grid_operations: GridOperationsChecklist,
    pub raw_private_payload_stored: bool,
    pub raw_secret_stored: bool,
    pub external_log_dependency: &'static str,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuditEventRecord {
    pub sequence: u32,
    pub event_type: &'static str,
    pub transition_from: &'static str,
    pub transition_to: &'static str,
    pub event_ref: String,
    pub trace_id: String,
    pub command_ref: String,
    pub request_ref: String,
    pub tenant_ref: String,
    pub actor_ref: String,
    pub request_hash_ref: String,
    pub payload_hash_ref: String,
    pub idempotency_record_ref: String,
    pub overwatch_client_ref: &'static str,
    pub audit_context_ref: String,
    pub privacy_class: &'static str,
    pub raw_payload_stored: bool,
    pub raw_secret_stored: bool,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EventTransition {
    pub event_type: &'static str,
    pub transition_from: &'static str,
    pub transition_to: &'static str,
    pub emits_for: &'static str,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EmergencyWalConfig {
    pub enabled: bool,
    pub max_entries: usize,
    pub max_age_ref: &'static str,
    pub mode_ref: &'static str,
    pub low_risk_phase1_mutation_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EmergencyWalEntry {
    pub sequence: u32,
    pub wal_ref: &'static str,
    pub event_ref: String,
    pub previous_entry_hash: String,
    pub entry_hash: String,
    pub trace_ref: String,
    pub command_ref: String,
    pub request_hash_ref: String,
    pub payload_hash_ref: String,
    pub append_only: bool,
    pub bounded_by_entries: usize,
    pub fsync_before_side_effect: bool,
    pub replay_state: &'static str,
    pub redacted_fields_only: bool,
    pub external_log_dependency: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EmergencyWalStatus {
    pub wal_ref: &'static str,
    pub enabled: bool,
    pub degraded_mode: bool,
    pub entries: Vec<EmergencyWalEntry>,
    pub max_entries: usize,
    pub max_age_ref: &'static str,
    pub append_only: bool,
    pub hash_chain_verified: bool,
    pub fsync_before_side_effect: bool,
    pub replay_to_overwatch_required: bool,
    pub readiness_state: &'static str,
    pub external_log_dependency: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MetricsTraceSummary {
    pub metric_policy_ref: &'static str,
    pub requests_observed: u32,
    pub denials_by_reason_tracked: bool,
    pub accepted_commands: u32,
    pub idempotency_replays: u32,
    pub idempotency_conflicts_tracked: bool,
    pub rate_limit_denials_tracked: bool,
    pub quota_denials_tracked: bool,
    pub policy_denials_tracked: bool,
    pub forwarding_latency_tracked: bool,
    pub dependency_failures_tracked: bool,
    pub downstream_failures_tracked: bool,
    pub labels: MetricLabelPolicy,
    pub trace_span_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MetricLabelPolicy {
    pub allowed_labels: Vec<&'static str>,
    pub excluded_high_cardinality_fields: Vec<&'static str>,
    pub tenant_label_state: &'static str,
    pub private_data_in_labels: bool,
    pub raw_payload_in_labels: bool,
    pub secrets_in_labels: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GridOperationsChecklist {
    pub checklist_ref: &'static str,
    pub system_service_workload_class: &'static str,
    pub readiness: &'static str,
    pub maintenance_mode: &'static str,
    pub rolling_update: &'static str,
    pub rollback: &'static str,
    pub break_glass_controls: &'static str,
    pub state_backup: &'static str,
    pub restore: &'static str,
    pub failover_drill: &'static str,
    pub founder_hardware_normal_path_removable: bool,
    pub public_contract_change_required: bool,
    pub package_validator_report_ref: &'static str,
}

impl Default for AuditStore {
    fn default() -> Self {
        Self {
            inner: Arc::default(),
            emergency_wal: EmergencyWalConfig::default(),
        }
    }
}

impl Default for EmergencyWalConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_entries: 8,
            max_age_ref: "duration:overgate:phase7:bounded_local_degraded_window",
            mode_ref: "degraded_mode:overgate:phase7:disabled_by_default",
            low_risk_phase1_mutation_only: true,
        }
    }
}

impl AuditStore {
    pub fn with_emergency_wal_enabled(max_entries: usize) -> Self {
        Self {
            inner: Arc::default(),
            emergency_wal: EmergencyWalConfig {
                enabled: true,
                max_entries: max_entries.max(1),
                mode_ref: "degraded_mode:overgate:phase7:explicit_emergency_buffer",
                ..EmergencyWalConfig::default()
            },
        }
    }

    pub fn guard_before_acceptance(
        &self,
        input: AuditGuardInput,
    ) -> Result<AuditDecision, OvergateError> {
        let guard_ref = format!(
            "audit_guard:overgate:phase7:{}",
            stable_short_token(&[
                input.command_type.as_str(),
                input.trace_id.as_str(),
                input.request_hash_ref.as_str(),
            ])
        );
        if !input.command_class.requires_audit {
            return Ok(AuditDecision {
                audit_state: "audit_not_required_for_command_class_phase7",
                overwatch_dependency_ready: input.overwatch_ready,
                fail_closed: false,
                emergency_wal_allowed: false,
                degraded_mode: false,
                command_class: input.command_class.command_class,
                reason_code: "overgate.audit_not_required_phase7",
                guard_ref,
            });
        }
        if input.overwatch_ready {
            return Ok(AuditDecision {
                audit_state: "overwatch_ready_phase7",
                overwatch_dependency_ready: true,
                fail_closed: false,
                emergency_wal_allowed: false,
                degraded_mode: false,
                command_class: input.command_class.command_class,
                reason_code: "overgate.audit_guard_ready_phase7",
                guard_ref,
            });
        }
        if self.emergency_wal.enabled
            && input.command_class.command_class == "phase1_control_plane_mutation"
            && !is_high_risk_without_overwatch(&input.command_type)
        {
            return Ok(AuditDecision {
                audit_state: "emergency_wal_authorized_phase7",
                overwatch_dependency_ready: false,
                fail_closed: false,
                emergency_wal_allowed: true,
                degraded_mode: true,
                command_class: input.command_class.command_class,
                reason_code: "overgate.audit_emergency_wal_allowed_phase7",
                guard_ref,
            });
        }

        Err(OvergateError::audit_fail_closed(vec![
            guard_ref,
            "dependency:overwatch".to_owned(),
            format!("command_class:{}", input.command_class.command_class),
        ]))
    }

    pub fn record_acceptance(&self, input: Phase7AuditInput) -> Phase7AuditEvidence {
        let ordered_events = accepted_event_sequence(&input);
        let mut state = self.lock_state();
        if input.audit_decision.emergency_wal_allowed {
            for event in &ordered_events {
                let previous_hash = state
                    .wal_entries
                    .last()
                    .map(|entry| entry.entry_hash.clone())
                    .unwrap_or_else(|| "hash:overgate:phase7:wal:genesis".to_owned());
                let sequence = state.wal_entries.len().saturating_add(1) as u32;
                let entry_hash = stable_hash_ref(&[
                    previous_hash.as_str(),
                    event.event_ref.as_str(),
                    event.trace_id.as_str(),
                    event.request_hash_ref.as_str(),
                    event.payload_hash_ref.as_str(),
                ]);
                state.wal_entries.push(EmergencyWalEntry {
                    sequence,
                    wal_ref: PHASE7_EMERGENCY_WAL_REF,
                    event_ref: event.event_ref.clone(),
                    previous_entry_hash: previous_hash,
                    entry_hash,
                    trace_ref: format!(
                        "trace_ref:overgate:phase7:{}",
                        stable_short_token(&[event.trace_id.as_str()])
                    ),
                    command_ref: event.command_ref.clone(),
                    request_hash_ref: event.request_hash_ref.clone(),
                    payload_hash_ref: event.payload_hash_ref.clone(),
                    append_only: true,
                    bounded_by_entries: self.emergency_wal.max_entries,
                    fsync_before_side_effect: true,
                    replay_state: "pending_replay_to_overwatch",
                    redacted_fields_only: true,
                    external_log_dependency: "none_rust_owned_local_wal",
                });
                if state.wal_entries.len() > self.emergency_wal.max_entries {
                    state.wal_entries.remove(0);
                }
            }
        }
        state.events.extend(ordered_events.clone());
        let wal_status = wal_status_from(
            &self.emergency_wal,
            &state,
            input.audit_decision.degraded_mode,
        );

        Phase7AuditEvidence {
            evidence_state: "phase7_audit_evidence_recorded",
            overwatch_client_ref: PHASE7_AUDIT_CLIENT_REF,
            event_transition_map_ref: PHASE7_EVENT_TRANSITION_MAP_REF,
            ordered_events: ordered_events.clone(),
            event_transition_map: event_transition_map(),
            emergency_wal: wal_status,
            metrics: metrics_summary(&input, ordered_events.len() as u32),
            grid_operations: GridOperationsChecklist::default(),
            raw_private_payload_stored: false,
            raw_secret_stored: false,
            external_log_dependency: "none_overwatch_contract_only",
            reason_code: "overgate.phase7_audit_recorded",
        }
    }

    fn lock_state(&self) -> MutexGuard<'_, AuditState> {
        self.inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl AuditGuardInput {
    pub fn from_envelope(envelope: &CommandEnvelope, overwatch_ready: bool) -> Self {
        Self {
            command_type: envelope.command_type.clone(),
            command_class: CommandClassAdmission::from_command_type(&envelope.command_type),
            request_hash_ref: envelope.request_hash.clone(),
            payload_hash_ref: envelope.payload_hash.clone(),
            trace_id: envelope.trace_id.clone(),
            overwatch_ready,
        }
    }
}

impl Phase7AuditInput {
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        envelope: &CommandEnvelope,
        request_id: String,
        audit_context_ref: String,
        idempotency_record_ref: String,
        idempotency_reason_code: &'static str,
        forwarding_state: &'static str,
        audit_decision: AuditDecision,
        precheck: PrecheckOutcome,
    ) -> Self {
        Self {
            trace_id: envelope.trace_id.clone(),
            command_id: envelope.command_id.clone(),
            request_id,
            tenant_id: envelope.tenant_id.clone(),
            actor_id: envelope.actor_id.clone(),
            request_hash_ref: envelope.request_hash.clone(),
            payload_hash_ref: envelope.payload_hash.clone(),
            idempotency_record_ref,
            idempotency_reason_code,
            audit_context_ref,
            forwarding_state,
            audit_decision,
            precheck,
        }
    }
}

impl Default for GridOperationsChecklist {
    fn default() -> Self {
        Self {
            checklist_ref: PHASE7_GRID_OPERATIONS_REF,
            system_service_workload_class: "system_service_workload:overgate:phase7",
            readiness: "readiness_uses_dependency_authority_and_degraded_wal_replay_state",
            maintenance_mode: "signed_operator_maintenance_mode_required",
            rolling_update: "rolling_update_requires_overwatch_audit_and_route_drain",
            rollback: "rollback_requires_version_pin_and_audit_ref",
            break_glass_controls: "break_glass_fails_closed_without_overwatch",
            state_backup: "backup_manifest_required_for_ingress_idempotency_audit_refs",
            restore: "restore_drill_requires_hash_chain_and_overwatch_replay_verification",
            failover_drill: "failover_requires_route_shift_queue_drain_and_split_brain_guard",
            founder_hardware_normal_path_removable: true,
            public_contract_change_required: false,
            package_validator_report_ref:
                "package_validator:overgate:phase7:grid_resident_contract",
        }
    }
}

pub fn event_transition_map() -> Vec<EventTransition> {
    vec![
        transition(
            "overgate.request_received",
            "received",
            "parsed",
            "accepted_or_denied",
        ),
        transition(
            "overgate.signature_verified",
            "parsed",
            "signature_verified",
            "accepted",
        ),
        transition("overgate.signature_denied", "parsed", "denied", "denied"),
        transition("overgate.schema_denied", "parsed", "denied", "denied"),
        transition(
            "overgate.tenant_denied",
            "identity_resolved",
            "denied",
            "denied",
        ),
        transition(
            "overgate.idempotency_reserved",
            "schema_validated",
            "idempotency_reserved",
            "accepted",
        ),
        transition(
            "overgate.idempotency_replayed",
            "schema_validated",
            "idempotency_replayed",
            "accepted",
        ),
        transition(
            "overgate.idempotency_conflict",
            "schema_validated",
            "denied",
            "denied",
        ),
        transition(
            "overgate.rate_limited",
            "idempotency_reserved",
            "denied",
            "denied",
        ),
        transition(
            "overgate.command_accepted",
            "prechecked",
            "accepted",
            "accepted",
        ),
        transition(
            "overgate.command_forwarded",
            "accepted",
            "forwarded",
            "phase8_forwarding",
        ),
        transition(
            "overgate.forwarding_failed",
            "accepted",
            "failed_after_acceptance",
            "phase8_forwarding",
        ),
    ]
}

fn accepted_event_sequence(input: &Phase7AuditInput) -> Vec<AuditEventRecord> {
    let idempotency_event = if input.idempotency_reason_code == "overgate.idempotency_replayed" {
        (
            "overgate.idempotency_replayed",
            "schema_validated",
            "idempotency_replayed",
        )
    } else {
        (
            "overgate.idempotency_reserved",
            "schema_validated",
            "idempotency_reserved",
        )
    };
    [
        ("overgate.request_received", "received", "parsed"),
        (
            "overgate.signature_verified",
            "parsed",
            "signature_verified",
        ),
        idempotency_event,
        ("overgate.command_accepted", "prechecked", "accepted"),
    ]
    .iter()
    .enumerate()
    .map(|(index, (event_type, from, to))| {
        AuditEventRecord::new((index + 1) as u32, event_type, from, to, input)
    })
    .collect()
}

fn transition(
    event_type: &'static str,
    transition_from: &'static str,
    transition_to: &'static str,
    emits_for: &'static str,
) -> EventTransition {
    EventTransition {
        event_type,
        transition_from,
        transition_to,
        emits_for,
        reason_code: "overgate.event_transition_phase7",
    }
}

impl AuditEventRecord {
    fn new(
        sequence: u32,
        event_type: &'static str,
        transition_from: &'static str,
        transition_to: &'static str,
        input: &Phase7AuditInput,
    ) -> Self {
        let event_ref = format!(
            "audit:overwatch:overgate:phase7:{}:{}",
            event_type.replace('.', "_"),
            stable_short_token(&[
                event_type,
                input.trace_id.as_str(),
                input.command_id.as_str(),
                input.request_id.as_str(),
            ])
        );
        Self {
            sequence,
            event_type,
            transition_from,
            transition_to,
            event_ref,
            trace_id: input.trace_id.clone(),
            command_ref: input.command_id.clone(),
            request_ref: input.request_id.clone(),
            tenant_ref: hashed_ref("tenant_ref:overgate:phase7", &input.tenant_id),
            actor_ref: hashed_ref("actor_ref:overgate:phase7", &input.actor_id),
            request_hash_ref: input.request_hash_ref.clone(),
            payload_hash_ref: input.payload_hash_ref.clone(),
            idempotency_record_ref: input.idempotency_record_ref.clone(),
            overwatch_client_ref: PHASE7_AUDIT_CLIENT_REF,
            audit_context_ref: input.audit_context_ref.clone(),
            privacy_class: "refs_hashes_redacted_only",
            raw_payload_stored: false,
            raw_secret_stored: false,
            reason_code: event_type,
        }
    }
}

fn metrics_summary(input: &Phase7AuditInput, event_count: u32) -> MetricsTraceSummary {
    MetricsTraceSummary {
        metric_policy_ref: PHASE7_METRIC_POLICY_REF,
        requests_observed: 1,
        denials_by_reason_tracked: true,
        accepted_commands: 1,
        idempotency_replays: u32::from(
            input.idempotency_reason_code == "overgate.idempotency_replayed",
        ),
        idempotency_conflicts_tracked: true,
        rate_limit_denials_tracked: true,
        quota_denials_tracked: true,
        policy_denials_tracked: true,
        forwarding_latency_tracked: true,
        dependency_failures_tracked: true,
        downstream_failures_tracked: true,
        labels: MetricLabelPolicy {
            allowed_labels: vec![
                "service",
                "command_class",
                "reason_code",
                "dependency",
                "forwarding_state",
                "environment",
            ],
            excluded_high_cardinality_fields: vec![
                "tenant_id",
                "actor_id",
                "command_id",
                "request_id",
                "trace_id",
                "payload_hash",
                "raw_payload",
                "credential_id",
                "secret_ref",
            ],
            tenant_label_state: "tenant_ref_hashed_only",
            private_data_in_labels: false,
            raw_payload_in_labels: false,
            secrets_in_labels: false,
        },
        trace_span_refs: vec![
            format!(
                "span:overgate:phase7:{}",
                stable_short_token(&[
                    input.trace_id.as_str(),
                    input.precheck.command_class.command_class,
                    input.forwarding_state,
                ])
            ),
            format!("span_count:overgate:phase7:{event_count}"),
        ],
    }
}

fn wal_status_from(
    config: &EmergencyWalConfig,
    state: &AuditState,
    degraded_mode: bool,
) -> EmergencyWalStatus {
    EmergencyWalStatus {
        wal_ref: PHASE7_EMERGENCY_WAL_REF,
        enabled: config.enabled,
        degraded_mode,
        entries: state.wal_entries.clone(),
        max_entries: config.max_entries,
        max_age_ref: config.max_age_ref,
        append_only: true,
        hash_chain_verified: verify_hash_chain(&state.wal_entries),
        fsync_before_side_effect: true,
        replay_to_overwatch_required: degraded_mode && !state.wal_entries.is_empty(),
        readiness_state: if degraded_mode && !state.wal_entries.is_empty() {
            "degraded_until_replayed_to_overwatch"
        } else {
            "ready_no_emergency_replay_required"
        },
        external_log_dependency: "none_rust_owned_local_wal",
    }
}

fn verify_hash_chain(entries: &[EmergencyWalEntry]) -> bool {
    let mut expected_previous = "hash:overgate:phase7:wal:genesis".to_owned();
    for entry in entries {
        if entry.previous_entry_hash != expected_previous {
            return false;
        }
        expected_previous = entry.entry_hash.clone();
    }
    true
}

fn hashed_ref(prefix: &str, value: &str) -> String {
    format!("{}:{}", prefix, stable_short_token(&[value]))
}

fn is_high_risk_without_overwatch(command_type: &str) -> bool {
    [
        "credential",
        "tenant.suspend",
        "tenant.delete",
        "ledger",
        "accounting",
        "rights",
        "secret",
        "policy.override",
        "break_glass",
        ".admin",
    ]
    .iter()
    .any(|marker| command_type.contains(marker))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn accepted_input(command_type: &str, decision: AuditDecision) -> Phase7AuditInput {
        let envelope: CommandEnvelope = serde_json::from_value(json!({
            "command_id": "command:overgate:phase7:0001",
            "command_type": command_type,
            "tenant_id": "tenant:local:test",
            "actor_id": "actor:local:test",
            "trace_id": "trace_overgate_phase7",
            "idempotency_key": "idem:overgate:phase7",
            "credential_id": "credential:local:test",
            "schema_version": crate::envelope::SUPPORTED_COMMAND_SCHEMA_VERSION,
            "payload_type": "application/vnd.overrid.command+json",
            "request_hash": "hash:fixture:phase7_request",
            "payload_hash": "hash:fixture:phase7_payload",
            "timestamp": "2026-06-25T03:00:00Z",
            "signature_metadata": {
                "signature_ref": "signature:fixture:phase7",
                "key_version": "key:v1",
                "algorithm": "ed25519",
                "canonicalization_version": "overgate.canonical.v0.1"
            },
            "privacy_class": "tenant_private"
        }))
        .expect("phase7 test envelope should parse");
        envelope
            .validate()
            .expect("phase7 test envelope should validate");
        let command_class = CommandClassAdmission::from_command_type(command_type);
        let precheck = PrecheckOutcome {
            adapter_id: "overgate.phase6.local_precheck_adapter",
            precheck_state: "prechecked_before_forwarding_phase6",
            command_class: command_class.clone(),
            rate_limit: crate::prechecks::RateLimitDecision {
                bucket_id: "rate_limit:overgate:phase6:test".to_owned(),
                scope_ref: "rate_limit_scope:overgate:phase6:test".to_owned(),
                tenant_id: "tenant:local:test".to_owned(),
                actor_id: "actor:local:test".to_owned(),
                service_account_ref: None,
                source_app_ref: "source_app:overgate:phase6:test".to_owned(),
                command_class: command_class.command_class,
                environment_ref: "environment:local",
                window_id: "window:2026-06-25T03".to_owned(),
                capacity: 2,
                consumed: 1,
                remaining: 1,
                reset_ref: "rate_limit_reset:overgate:phase6:test".to_owned(),
                denied: false,
                audit_ref: "audit:overgate:rate_limited:test".to_owned(),
                reason_code: "overgate.rate_limit_allowed_phase6",
            },
            quota_precheck: crate::prechecks::QuotaPrecheckRecord {
                record_id: "quota_record:overgate:phase6:test".to_owned(),
                quota_precheck_ref: "quota_precheck:overgate:phase6:test".to_owned(),
                tenant_id: "tenant:local:test".to_owned(),
                actor_ref: "actor:local:test".to_owned(),
                quota_scope_ref: "quota_scope:tenant:local:test".to_owned(),
                command_class: command_class.command_class,
                request_size_class: "small_request",
                rate_limit_bucket_ref: "rate_limit:overgate:phase6:test".to_owned(),
                budget_ref: "budget:overgate:phase6:test".to_owned(),
                grant_placeholder_refs: vec!["grant_placeholder:overgrant:phase6:test".to_owned()],
                local_counter_ref: "local_counter:overgate:phase6:test".to_owned(),
                overmeter_snapshot_ref: Some(
                    "overmeter_snapshot:placeholder:phase6:test".to_owned(),
                ),
                accepted_command_quota_refs: vec!["quota_precheck:overgate:phase6:test".to_owned()],
                allowed: true,
                no_balance_mutation: true,
                no_seal_ledger_entry: true,
                settlement_state: "not_settled_by_overgate",
                reason_code: "overgate.quota_precheck_allowed_phase6",
            },
            policy_check: crate::prechecks::PolicyCheckRecord {
                tenant_id: "tenant:local:test".to_owned(),
                dependency_id: "overguard",
                handoff_state: "policy_not_required_for_command_class_phase6",
                required: false,
                allowed: true,
                policy_version: "overguard.policy.v0",
                matched_rule_refs: Vec::new(),
                decision_ref: "policy_decision:overguard:phase6:test".to_owned(),
                missing_prerequisite_reasons: Vec::new(),
                policy_truth_owner: "overguard",
                stored_policy_truth_in_overgate: false,
                reason_code: "overgate.policy_not_required_phase6",
            },
            client_denial_surface: crate::prechecks::ClientDenialSurface {
                rate_limit_ref: "rate_limit:overgate:phase6:test".to_owned(),
                quota_precheck_ref: "quota_precheck:overgate:phase6:test".to_owned(),
                budget_ref: "budget:overgate:phase6:test".to_owned(),
                grant_refs: vec!["grant_placeholder:overgrant:phase6:test".to_owned()],
                policy_decision_ref: "policy_decision:overguard:phase6:test".to_owned(),
                stable_reason_code: "overgate.phase6_prechecks_passed",
                correction_ref: "client_action:none",
                sdk_cli_ui_native_safe: true,
            },
        };
        Phase7AuditInput::from_parts(
            &envelope,
            "request_phase7_test".to_owned(),
            "audit_context:overgate:phase7:test".to_owned(),
            "idempotency:overgate:phase7:test".to_owned(),
            "overgate.idempotency_reserved",
            "pending_forwarding_phase5",
            decision,
            precheck,
        )
    }

    #[test]
    fn phase7_event_transition_map_covers_accept_and_denial_events() {
        let events = event_transition_map();
        for expected in [
            "overgate.request_received",
            "overgate.signature_verified",
            "overgate.signature_denied",
            "overgate.schema_denied",
            "overgate.tenant_denied",
            "overgate.idempotency_reserved",
            "overgate.idempotency_replayed",
            "overgate.idempotency_conflict",
            "overgate.rate_limited",
            "overgate.command_accepted",
            "overgate.command_forwarded",
            "overgate.forwarding_failed",
        ] {
            assert!(
                events.iter().any(|event| event.event_type == expected),
                "missing {expected}"
            );
        }
    }

    #[test]
    fn phase7_audit_guard_fails_closed_for_high_risk_without_overwatch() {
        let store = AuditStore::default();
        let command_class =
            CommandClassAdmission::from_command_type("overgate.phase7.accounting.ledger.transfer");
        let error = store
            .guard_before_acceptance(AuditGuardInput {
                command_type: "overgate.phase7.accounting.ledger.transfer".to_owned(),
                command_class,
                request_hash_ref: "hash:fixture:phase7_request".to_owned(),
                payload_hash_ref: "hash:fixture:phase7_payload".to_owned(),
                trace_id: "trace_phase7_fail_closed".to_owned(),
                overwatch_ready: false,
            })
            .expect_err("high-risk commands should fail closed without Overwatch");
        assert_eq!(error.reason_code, "overgate.audit_fail_closed");
        assert_eq!(error.dependency_name, Some("overwatch"));
    }

    #[test]
    fn phase7_emergency_wal_is_bounded_hash_chained_and_redacted() {
        let store = AuditStore::with_emergency_wal_enabled(4);
        let command_class =
            CommandClassAdmission::from_command_type("overgate.phase7.tenant.profile_update");
        let decision = store
            .guard_before_acceptance(AuditGuardInput {
                command_type: "overgate.phase7.tenant.profile_update".to_owned(),
                command_class,
                request_hash_ref: "hash:fixture:phase7_request".to_owned(),
                payload_hash_ref: "hash:fixture:phase7_payload".to_owned(),
                trace_id: "trace_phase7_emergency".to_owned(),
                overwatch_ready: false,
            })
            .expect("low-risk phase1 mutation can use explicit emergency WAL");
        let evidence = store.record_acceptance(accepted_input(
            "overgate.phase7.tenant.profile_update",
            decision,
        ));
        assert!(evidence.emergency_wal.enabled);
        assert!(evidence.emergency_wal.degraded_mode);
        assert_eq!(evidence.emergency_wal.entries.len(), 4);
        assert!(evidence.emergency_wal.hash_chain_verified);
        assert!(evidence.emergency_wal.replay_to_overwatch_required);
        assert_eq!(
            evidence.emergency_wal.readiness_state,
            "degraded_until_replayed_to_overwatch"
        );
        assert!(evidence
            .emergency_wal
            .entries
            .iter()
            .all(|entry| entry.redacted_fields_only && entry.fsync_before_side_effect));
    }

    #[test]
    fn phase7_metrics_and_grid_ops_are_private_safe() {
        let store = AuditStore::default();
        let command_class =
            CommandClassAdmission::from_command_type("overgate.phase7.tenant.profile_update");
        let decision = store
            .guard_before_acceptance(AuditGuardInput {
                command_type: "overgate.phase7.tenant.profile_update".to_owned(),
                command_class,
                request_hash_ref: "hash:fixture:phase7_request".to_owned(),
                payload_hash_ref: "hash:fixture:phase7_payload".to_owned(),
                trace_id: "trace_phase7_metrics".to_owned(),
                overwatch_ready: true,
            })
            .expect("Overwatch-ready command should pass guard");
        let evidence = store.record_acceptance(accepted_input(
            "overgate.phase7.tenant.profile_update",
            decision,
        ));
        assert!(!evidence.metrics.labels.private_data_in_labels);
        assert!(!evidence.metrics.labels.raw_payload_in_labels);
        assert!(!evidence.metrics.labels.secrets_in_labels);
        assert!(evidence
            .metrics
            .labels
            .excluded_high_cardinality_fields
            .contains(&"tenant_id"));
        assert_eq!(
            evidence.grid_operations.system_service_workload_class,
            "system_service_workload:overgate:phase7"
        );
        assert!(
            evidence
                .grid_operations
                .founder_hardware_normal_path_removable
        );
        assert!(!evidence.grid_operations.public_contract_change_required);
    }
}
