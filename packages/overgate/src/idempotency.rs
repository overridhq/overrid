use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use serde::Serialize;

use crate::canonical::{stable_hash_ref, stable_short_token, CanonicalRequestInput};
use crate::envelope::CommandEnvelope;
use crate::errors::OvergateError;
use crate::retention::RetentionDecision;

#[derive(Debug, Clone, Default)]
pub struct IdempotencyStore {
    inner: Arc<Mutex<IdempotencyState>>,
}

#[derive(Debug, Default)]
struct IdempotencyState {
    records: HashMap<IdempotencyScope, IdempotencyRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct IdempotencyScope {
    tenant_id: String,
    actor_id: String,
    command_type: String,
    idempotency_key: String,
    credential_context_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdempotencyReservationInput {
    pub request_id: String,
    pub command_id: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub command_type: String,
    pub idempotency_key: String,
    pub credential_id: String,
    pub key_version: String,
    pub request_hash: String,
    pub first_trace_id: String,
    pub canonical_hash: String,
    pub retention: RetentionDecision,
    pub audit_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct IdempotencyOutcome {
    pub outcome_state: &'static str,
    pub reason_code: &'static str,
    pub replayed: bool,
    pub record: IdempotencyRecord,
    pub replay_metadata: SafeReplayMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SafeReplayMetadata {
    pub first_trace_id: String,
    pub replay_count: u32,
    pub response_digest_ref: String,
    pub private_payload_disclosed: bool,
    pub credential_material_disclosed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct IdempotencyRecord {
    pub record_id: String,
    pub request_id: String,
    pub command_ref: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub command_type: String,
    pub idempotency_key: String,
    pub credential_context_ref: String,
    pub request_hash: String,
    pub canonical_hash: String,
    pub first_trace_id: String,
    pub trace_ids: Vec<String>,
    pub current_state: &'static str,
    pub forwarding_state: &'static str,
    pub response_digest_ref: String,
    pub replay_count: u32,
    pub retention: RetentionDecision,
    pub audit_refs: Vec<String>,
    pub status_visibility: &'static str,
    pub owner: &'static str,
    pub conflict_reason: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandLookup {
    Found(IdempotencyRecord),
    Forbidden,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdempotencyMutation {
    Applied(IdempotencyRecord),
    Forbidden,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraceSummary {
    pub trace_id: String,
    pub command_count: usize,
    pub records: Vec<TraceCommandSummary>,
    pub audit_refs: Vec<String>,
    pub caller_visible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraceCommandSummary {
    pub command_id: String,
    pub current_state: &'static str,
    pub forwarding_state: &'static str,
    pub response_digest_ref: String,
    pub retention_class: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct IdempotencyLimitSummary {
    pub tenant_id: String,
    pub visible_record_count: usize,
    pub retention_classes: Vec<&'static str>,
    pub quota_precheck_refs: Vec<String>,
}

impl IdempotencyReservationInput {
    pub fn from_envelope(
        envelope: &CommandEnvelope,
        canonical_request: &CanonicalRequestInput,
        request_id: String,
        retention: RetentionDecision,
        audit_ref: String,
    ) -> Self {
        Self {
            request_id,
            command_id: envelope.command_id.clone(),
            tenant_id: envelope.tenant_id.clone(),
            actor_id: envelope.actor_id.clone(),
            command_type: envelope.command_type.clone(),
            idempotency_key: envelope.idempotency_key.clone(),
            credential_id: envelope.credential_id.clone(),
            key_version: envelope.signature_metadata.key_version.clone(),
            request_hash: envelope.request_hash.clone(),
            first_trace_id: envelope.trace_id.clone(),
            canonical_hash: canonical_request.canonical_hash.clone(),
            retention,
            audit_ref,
        }
    }

    fn scope(&self) -> IdempotencyScope {
        IdempotencyScope {
            tenant_id: self.tenant_id.clone(),
            actor_id: self.actor_id.clone(),
            command_type: self.command_type.clone(),
            idempotency_key: self.idempotency_key.clone(),
            credential_context_ref: credential_context_ref(&self.credential_id, &self.key_version),
        }
    }
}

impl IdempotencyStore {
    pub fn reserve_or_replay(
        &self,
        input: IdempotencyReservationInput,
    ) -> Result<IdempotencyOutcome, OvergateError> {
        let scope = input.scope();
        let mut state = self.lock_state();
        if let Some(existing) = state.records.get_mut(&scope) {
            if existing.request_hash != input.request_hash {
                existing.conflict_reason = Some("idempotency.request_hash_conflict");
                return Err(OvergateError::idempotency_conflict());
            }
            if !existing
                .trace_ids
                .iter()
                .any(|trace| trace == &input.first_trace_id)
            {
                existing.trace_ids.push(input.first_trace_id);
            }
            existing.replay_count = existing.replay_count.saturating_add(1);
            return Ok(IdempotencyOutcome::replayed(existing.clone()));
        }

        let record = IdempotencyRecord::from_input(input);
        state.records.insert(scope, record.clone());
        Ok(IdempotencyOutcome::reserved(record))
    }

    pub fn lookup_command(&self, tenant_id: &str, command_id: &str) -> CommandLookup {
        let state = self.lock_state();
        let mut saw_command = false;
        for record in state.records.values() {
            if record.command_ref == command_id {
                saw_command = true;
                if record.tenant_id == tenant_id {
                    return CommandLookup::Found(record.clone());
                }
            }
        }
        if saw_command {
            CommandLookup::Forbidden
        } else {
            CommandLookup::Missing
        }
    }

    pub fn trace_summary(&self, tenant_id: &str, trace_id: &str) -> TraceSummary {
        let state = self.lock_state();
        let records = state
            .records
            .values()
            .filter(|record| {
                record.tenant_id == tenant_id
                    && record.trace_ids.iter().any(|trace| trace == trace_id)
            })
            .cloned()
            .collect::<Vec<_>>();
        trace_summary_from_records(trace_id, records)
    }

    pub fn limit_summary(&self, tenant_id: &str) -> IdempotencyLimitSummary {
        let state = self.lock_state();
        let mut retention_classes = Vec::new();
        let mut visible_record_count = 0usize;
        for record in state
            .records
            .values()
            .filter(|record| record.tenant_id == tenant_id)
        {
            visible_record_count += 1;
            if !retention_classes.contains(&record.retention.idempotency_retention_class) {
                retention_classes.push(record.retention.idempotency_retention_class);
            }
        }
        IdempotencyLimitSummary {
            tenant_id: tenant_id.to_owned(),
            visible_record_count,
            retention_classes,
            quota_precheck_refs: vec![format!(
                "quota_precheck:overgate:phase5:{}",
                stable_short_token(&[tenant_id])
            )],
        }
    }

    pub fn admin_records_for_key(
        &self,
        tenant_id: &str,
        idempotency_key: &str,
    ) -> Vec<IdempotencyRecord> {
        let state = self.lock_state();
        state
            .records
            .values()
            .filter(|record| {
                record.tenant_id == tenant_id && record.idempotency_key == idempotency_key
            })
            .cloned()
            .collect()
    }

    pub fn expire_record(&self, tenant_id: &str, record_id: &str) -> IdempotencyMutation {
        let mut state = self.lock_state();
        for record in state.records.values_mut() {
            if record.record_id == record_id {
                if record.tenant_id != tenant_id {
                    return IdempotencyMutation::Forbidden;
                }
                record.current_state = "retention_expired";
                record.forwarding_state = "expired_after_retention_window";
                record.audit_refs.push(format!(
                    "audit:overgate:idempotency_expired:{}",
                    stable_short_token(&[record_id])
                ));
                return IdempotencyMutation::Applied(record.clone());
            }
        }
        IdempotencyMutation::Missing
    }

    pub fn seed_record(&self, record: IdempotencyRecord) {
        let mut state = self.lock_state();
        state.records.insert(record.scope(), record);
    }

    fn lock_state(&self) -> MutexGuard<'_, IdempotencyState> {
        self.inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl IdempotencyOutcome {
    fn reserved(record: IdempotencyRecord) -> Self {
        Self {
            outcome_state: "reserved",
            reason_code: "overgate.idempotency_reserved",
            replayed: false,
            replay_metadata: SafeReplayMetadata::from_record(&record),
            record,
        }
    }

    fn replayed(record: IdempotencyRecord) -> Self {
        Self {
            outcome_state: "replayed",
            reason_code: "overgate.idempotency_replayed",
            replayed: true,
            replay_metadata: SafeReplayMetadata::from_record(&record),
            record,
        }
    }
}

impl SafeReplayMetadata {
    fn from_record(record: &IdempotencyRecord) -> Self {
        Self {
            first_trace_id: record.first_trace_id.clone(),
            replay_count: record.replay_count,
            response_digest_ref: record.response_digest_ref.clone(),
            private_payload_disclosed: false,
            credential_material_disclosed: false,
        }
    }
}

impl IdempotencyRecord {
    fn from_input(input: IdempotencyReservationInput) -> Self {
        let credential_context_ref =
            credential_context_ref(&input.credential_id, &input.key_version);
        let record_id = format!(
            "idempotency:overgate:{}",
            stable_short_token(&[
                input.tenant_id.as_str(),
                input.actor_id.as_str(),
                input.command_type.as_str(),
                input.idempotency_key.as_str(),
                credential_context_ref.as_str(),
            ])
        );
        let response_digest_ref = stable_hash_ref(&[
            "overgate.phase5.response_digest",
            input.request_id.as_str(),
            input.command_id.as_str(),
            input.request_hash.as_str(),
            input.first_trace_id.as_str(),
        ]);
        Self {
            record_id: record_id.clone(),
            request_id: input.request_id,
            command_ref: input.command_id,
            tenant_id: input.tenant_id,
            actor_id: input.actor_id,
            command_type: input.command_type,
            idempotency_key: input.idempotency_key,
            credential_context_ref,
            request_hash: input.request_hash,
            canonical_hash: input.canonical_hash,
            first_trace_id: input.first_trace_id.clone(),
            trace_ids: vec![input.first_trace_id],
            current_state: "pending_forwarding",
            forwarding_state: "pending_forwarding_phase5",
            response_digest_ref,
            replay_count: 0,
            retention: input.retention,
            audit_refs: vec![
                input.audit_ref,
                format!("audit:overgate:idempotency_reserved:{record_id}"),
            ],
            status_visibility: "tenant_actor_command_scoped",
            owner: "overgate_until_downstream_handoff",
            conflict_reason: None,
        }
    }

    pub fn with_state(
        mut self,
        current_state: &'static str,
        forwarding_state: &'static str,
    ) -> Self {
        self.current_state = current_state;
        self.forwarding_state = forwarding_state;
        self
    }

    fn scope(&self) -> IdempotencyScope {
        IdempotencyScope {
            tenant_id: self.tenant_id.clone(),
            actor_id: self.actor_id.clone(),
            command_type: self.command_type.clone(),
            idempotency_key: self.idempotency_key.clone(),
            credential_context_ref: self.credential_context_ref.clone(),
        }
    }
}

fn trace_summary_from_records(trace_id: &str, records: Vec<IdempotencyRecord>) -> TraceSummary {
    let mut audit_refs = Vec::new();
    let mut summaries = Vec::new();
    for record in records {
        for audit_ref in &record.audit_refs {
            if !audit_refs.contains(audit_ref) {
                audit_refs.push(audit_ref.clone());
            }
        }
        summaries.push(TraceCommandSummary {
            command_id: record.command_ref,
            current_state: record.current_state,
            forwarding_state: record.forwarding_state,
            response_digest_ref: record.response_digest_ref,
            retention_class: record.retention.idempotency_retention_class,
        });
    }
    TraceSummary {
        trace_id: trace_id.to_owned(),
        command_count: summaries.len(),
        records: summaries,
        audit_refs,
        caller_visible: true,
    }
}

fn credential_context_ref(credential_id: &str, key_version: &str) -> String {
    format!(
        "credential_context:overgate:{}",
        stable_short_token(&[credential_id, key_version])
    )
}
