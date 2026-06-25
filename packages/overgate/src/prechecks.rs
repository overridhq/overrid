use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, MutexGuard};

use serde::Serialize;
use serde_json::Value;

use crate::admission::AdmissionContext;
use crate::canonical::{stable_hash_ref, stable_short_token};
use crate::envelope::CommandEnvelope;
use crate::errors::OvergateError;
use crate::idempotency::IdempotencyOutcome;

pub const PHASE6_PRECHECK_ADAPTER_ID: &str = "overgate.phase6.local_precheck_adapter";
pub const PHASE6_COMMAND_CLASS_MATRIX_REF: &str = "command_class_matrix:overgate:phase6";
const DEFAULT_BUCKET_CAPACITY: u32 = 2;

#[derive(Debug, Clone, Default)]
pub struct PrecheckStore {
    inner: Arc<Mutex<PrecheckState>>,
}

#[derive(Debug, Default)]
struct PrecheckState {
    rate_buckets: HashMap<RateLimitScope, RateLimitBucket>,
    quota_records: Vec<QuotaPrecheckRecord>,
    policy_records: Vec<PolicyCheckRecord>,
    precheck_results: HashMap<String, Result<PrecheckOutcome, OvergateError>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RateLimitScope {
    tenant_id: String,
    actor_id: String,
    service_account_ref: String,
    source_app_ref: String,
    command_class: &'static str,
    environment_ref: &'static str,
    window_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RateLimitBucket {
    consumed: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrecheckInput {
    pub tenant_id: String,
    pub actor_id: String,
    pub command_type: String,
    pub credential_id: String,
    pub trace_id: String,
    pub request_hash: String,
    pub payload_hash: String,
    pub timestamp: String,
    pub request_body_len: usize,
    pub quota_scope_ref: String,
    pub idempotency_record_ref: String,
    pub idempotency_reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrecheckOutcome {
    pub adapter_id: &'static str,
    pub precheck_state: &'static str,
    pub command_class: CommandClassAdmission,
    pub rate_limit: RateLimitDecision,
    pub quota_precheck: QuotaPrecheckRecord,
    pub policy_check: PolicyCheckRecord,
    pub client_denial_surface: ClientDenialSurface,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommandClassAdmission {
    pub matrix_ref: &'static str,
    pub command_class: &'static str,
    pub requires_signature: bool,
    pub requires_tenant: bool,
    pub requires_idempotency: bool,
    pub requires_policy: bool,
    pub requires_quota: bool,
    pub requires_audit: bool,
    pub forwarding_mode: &'static str,
    pub fail_closed_behavior: &'static str,
    pub matrix_state: &'static str,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RateLimitDecision {
    pub bucket_id: String,
    pub scope_ref: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub service_account_ref: Option<String>,
    pub source_app_ref: String,
    pub command_class: &'static str,
    pub environment_ref: &'static str,
    pub window_id: String,
    pub capacity: u32,
    pub consumed: u32,
    pub remaining: u32,
    pub reset_ref: String,
    pub denied: bool,
    pub audit_ref: String,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuotaPrecheckRecord {
    pub record_id: String,
    pub quota_precheck_ref: String,
    pub tenant_id: String,
    pub actor_ref: String,
    pub quota_scope_ref: String,
    pub command_class: &'static str,
    pub request_size_class: &'static str,
    pub rate_limit_bucket_ref: String,
    pub budget_ref: String,
    pub grant_placeholder_refs: Vec<String>,
    pub local_counter_ref: String,
    pub overmeter_snapshot_ref: Option<String>,
    pub accepted_command_quota_refs: Vec<String>,
    pub allowed: bool,
    pub no_balance_mutation: bool,
    pub no_seal_ledger_entry: bool,
    pub settlement_state: &'static str,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PolicyCheckRecord {
    pub tenant_id: String,
    pub dependency_id: &'static str,
    pub handoff_state: &'static str,
    pub required: bool,
    pub allowed: bool,
    pub policy_version: &'static str,
    pub matched_rule_refs: Vec<String>,
    pub decision_ref: String,
    pub missing_prerequisite_reasons: Vec<String>,
    pub policy_truth_owner: &'static str,
    pub stored_policy_truth_in_overgate: bool,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClientDenialSurface {
    pub rate_limit_ref: String,
    pub quota_precheck_ref: String,
    pub budget_ref: String,
    pub grant_refs: Vec<String>,
    pub policy_decision_ref: String,
    pub stable_reason_code: &'static str,
    pub correction_ref: &'static str,
    pub sdk_cli_ui_native_safe: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrecheckLimitSummary {
    pub tenant_id: String,
    pub bucket_count: usize,
    pub buckets: Vec<RateLimitBucketView>,
    pub quota_precheck_refs: Vec<String>,
    pub budget_refs: Vec<String>,
    pub policy_decision_refs: Vec<String>,
    pub command_class_matrix_ref: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RateLimitBucketView {
    pub bucket_id: String,
    pub command_class: &'static str,
    pub window_id: String,
    pub capacity: u32,
    pub consumed: u32,
    pub remaining: u32,
    pub reset_ref: String,
}

impl PrecheckInput {
    pub fn from_parts(
        envelope: &CommandEnvelope,
        admission: &AdmissionContext,
        idempotency: &IdempotencyOutcome,
        request_body_len: usize,
    ) -> Self {
        Self {
            tenant_id: envelope.tenant_id.clone(),
            actor_id: envelope.actor_id.clone(),
            command_type: envelope.command_type.clone(),
            credential_id: envelope.credential_id.clone(),
            trace_id: envelope.trace_id.clone(),
            request_hash: envelope.request_hash.clone(),
            payload_hash: envelope.payload_hash.clone(),
            timestamp: envelope.timestamp.clone(),
            request_body_len,
            quota_scope_ref: admission.tenant_authorization.quota_scope_ref.clone(),
            idempotency_record_ref: idempotency.record.record_id.clone(),
            idempotency_reason_code: idempotency.reason_code,
        }
    }
}

impl PrecheckStore {
    pub fn precheck_command(&self, input: PrecheckInput) -> Result<PrecheckOutcome, OvergateError> {
        if input.idempotency_reason_code == "overgate.idempotency_replayed" {
            let state = self.lock_state();
            if let Some(result) = state.precheck_results.get(&input.idempotency_record_ref) {
                return result.clone();
            }
        }

        let command_class = CommandClassAdmission::from_command_type(&input.command_type);
        if !command_class.matrix_complete() {
            let error = OvergateError::command_class_matrix_denied();
            let mut state = self.lock_state();
            state
                .precheck_results
                .insert(input.idempotency_record_ref.clone(), Err(error.clone()));
            return Err(error);
        }

        let mut state = self.lock_state();
        let rate_limit = state.consume_rate_limit(&input, command_class.command_class);
        if rate_limit.denied {
            let error = OvergateError::rate_limited(vec![
                rate_limit.bucket_id.clone(),
                rate_limit.audit_ref.clone(),
                rate_limit.reset_ref.clone(),
            ]);
            state
                .precheck_results
                .insert(input.idempotency_record_ref.clone(), Err(error.clone()));
            return Err(error);
        }

        let quota_precheck = QuotaPrecheckRecord::from_input(&input, &command_class, &rate_limit);
        if !quota_precheck.allowed {
            let refs = quota_precheck.client_denial_refs();
            state.quota_records.push(quota_precheck);
            let error = OvergateError::quota_precheck_denied(refs);
            state
                .precheck_results
                .insert(input.idempotency_record_ref.clone(), Err(error.clone()));
            return Err(error);
        }

        let policy_check = PolicyCheckRecord::from_input(&input, &command_class);
        if !policy_check.allowed {
            let refs = vec![
                policy_check.decision_ref.clone(),
                policy_check
                    .matched_rule_refs
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "policy_rule:overguard:phase6:missing".to_owned()),
            ];
            state.quota_records.push(quota_precheck);
            state.policy_records.push(policy_check);
            let error = OvergateError::policy_denied(refs);
            state
                .precheck_results
                .insert(input.idempotency_record_ref.clone(), Err(error.clone()));
            return Err(error);
        }

        let client_denial_surface =
            ClientDenialSurface::accepted(&rate_limit, &quota_precheck, &policy_check);
        state.quota_records.push(quota_precheck.clone());
        state.policy_records.push(policy_check.clone());

        let outcome = PrecheckOutcome {
            adapter_id: PHASE6_PRECHECK_ADAPTER_ID,
            precheck_state: "prechecked_before_forwarding_phase6",
            command_class,
            rate_limit,
            quota_precheck,
            policy_check,
            client_denial_surface,
        };
        state
            .precheck_results
            .insert(input.idempotency_record_ref.clone(), Ok(outcome.clone()));
        Ok(outcome)
    }

    pub fn policy_dry_run(
        &self,
        tenant_id: &str,
        payload: &Value,
        trace_id: &str,
    ) -> PolicyCheckRecord {
        let command_type = payload
            .get("command_type")
            .and_then(Value::as_str)
            .unwrap_or("overgate.phase6.policy.dry_run");
        let decision = payload
            .get("simulate_decision")
            .and_then(Value::as_str)
            .unwrap_or("allow");
        let class = CommandClassAdmission::from_command_type(command_type);
        let mut record =
            PolicyCheckRecord::from_command_type(command_type, tenant_id, &class, trace_id);
        if decision == "deny" {
            record.allowed = false;
            record.reason_code = "overgate.policy_dry_run_denied_phase6";
            record.matched_rule_refs = vec![format!(
                "policy_rule:overguard:phase6:{}",
                stable_short_token(&[command_type, trace_id, "deny"])
            )];
        }
        let mut state = self.lock_state();
        state.policy_records.push(record.clone());
        record
    }

    pub fn limit_summary(&self, tenant_id: &str) -> PrecheckLimitSummary {
        let state = self.lock_state();
        let mut buckets = Vec::new();
        for (scope, bucket) in &state.rate_buckets {
            if scope.tenant_id == tenant_id {
                let bucket_id = rate_limit_bucket_id(scope);
                buckets.push(RateLimitBucketView {
                    bucket_id,
                    command_class: scope.command_class,
                    window_id: scope.window_id.clone(),
                    capacity: DEFAULT_BUCKET_CAPACITY,
                    consumed: bucket.consumed,
                    remaining: DEFAULT_BUCKET_CAPACITY.saturating_sub(bucket.consumed),
                    reset_ref: rate_limit_reset_ref(&scope.window_id),
                });
            }
        }
        let quota_precheck_refs = state
            .quota_records
            .iter()
            .filter(|record| record.tenant_id == tenant_id)
            .map(|record| record.quota_precheck_ref.clone())
            .collect::<Vec<_>>();
        let mut budget_refs = state
            .quota_records
            .iter()
            .filter(|record| record.tenant_id == tenant_id)
            .map(|record| record.budget_ref.clone())
            .collect::<Vec<_>>();
        budget_refs.sort();
        budget_refs.dedup();
        let policy_decision_refs = state
            .policy_records
            .iter()
            .filter(|record| record.tenant_id == tenant_id)
            .map(|record| record.decision_ref.clone())
            .collect::<Vec<_>>();

        PrecheckLimitSummary {
            tenant_id: tenant_id.to_owned(),
            bucket_count: buckets.len(),
            buckets,
            quota_precheck_refs,
            budget_refs,
            policy_decision_refs,
            command_class_matrix_ref: PHASE6_COMMAND_CLASS_MATRIX_REF,
        }
    }

    fn lock_state(&self) -> MutexGuard<'_, PrecheckState> {
        self.inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl PrecheckState {
    fn consume_rate_limit(
        &mut self,
        input: &PrecheckInput,
        command_class: &'static str,
    ) -> RateLimitDecision {
        let scope = RateLimitScope::from_input(input, command_class);
        let bucket_id = rate_limit_bucket_id(&scope);
        let scope_ref = format!(
            "rate_limit_scope:overgate:phase6:{}",
            stable_short_token(&[
                scope.tenant_id.as_str(),
                scope.actor_id.as_str(),
                scope.command_class,
                scope.window_id.as_str(),
            ])
        );
        let bucket = self
            .rate_buckets
            .entry(scope.clone())
            .or_insert(RateLimitBucket { consumed: 0 });
        let attempted = bucket.consumed.saturating_add(1);
        let denied = attempted > DEFAULT_BUCKET_CAPACITY
            || input.command_type.contains("rate_limit.exhausted");
        if !denied {
            bucket.consumed = attempted;
        }
        let consumed = if denied { bucket.consumed } else { attempted };
        let reset_ref = rate_limit_reset_ref(&scope.window_id);
        RateLimitDecision {
            bucket_id: bucket_id.clone(),
            scope_ref,
            tenant_id: scope.tenant_id,
            actor_id: scope.actor_id,
            service_account_ref: optional_service_account_ref(&input.actor_id),
            source_app_ref: source_app_ref(input),
            command_class,
            environment_ref: scope.environment_ref,
            window_id: scope.window_id.clone(),
            capacity: DEFAULT_BUCKET_CAPACITY,
            consumed,
            remaining: DEFAULT_BUCKET_CAPACITY.saturating_sub(consumed),
            reset_ref,
            denied,
            audit_ref: format!(
                "audit:overgate:rate_limited:{}",
                stable_short_token(&[bucket_id.as_str(), input.trace_id.as_str()])
            ),
            reason_code: if denied {
                "overgate.rate_limited"
            } else {
                "overgate.rate_limit_allowed_phase6"
            },
        }
    }
}

impl RateLimitScope {
    fn from_input(input: &PrecheckInput, command_class: &'static str) -> Self {
        Self {
            tenant_id: input.tenant_id.clone(),
            actor_id: input.actor_id.clone(),
            service_account_ref: optional_service_account_ref(&input.actor_id)
                .unwrap_or_else(|| "service_account:none".to_owned()),
            source_app_ref: source_app_ref(input),
            command_class,
            environment_ref: "environment:local",
            window_id: rate_window_id(&input.timestamp),
        }
    }
}

impl CommandClassAdmission {
    pub fn from_command_type(command_type: &str) -> Self {
        let command_class = command_class_name(command_type);
        let (
            requires_signature,
            requires_idempotency,
            requires_policy,
            requires_quota,
            requires_audit,
            forwarding_mode,
            fail_closed_behavior,
        ) = match command_class {
            "low_risk_read" => (
                false,
                false,
                false,
                false,
                false,
                "bodyless_read_no_forwarding",
                "fail_closed_if_private_data_requested",
            ),
            "phase1_control_plane_mutation" => (
                true,
                true,
                false,
                true,
                true,
                "synchronous_phase1_or_overqueue_later",
                "fail_closed_on_missing_admission_precheck",
            ),
            "queue_producing_workload" => (
                true,
                true,
                true,
                true,
                true,
                "overqueue_required_later_phase8",
                "fail_closed_before_queue_handoff",
            ),
            "policy_heavy" => (
                true,
                true,
                true,
                true,
                true,
                "overguard_handoff_required",
                "fail_closed_on_policy_missing_or_deny",
            ),
            "accounting_affecting" => (
                true,
                true,
                true,
                true,
                true,
                "accounting_refs_only_no_settlement",
                "fail_closed_on_quota_or_policy_gap",
            ),
            "storage_namespace" => (
                true,
                true,
                true,
                true,
                true,
                "storage_namespace_target_later_phase8",
                "fail_closed_before_storage_namespace_handoff",
            ),
            "native_app_side_effect" => (
                true,
                true,
                true,
                true,
                true,
                "native_app_target_later_phase12",
                "fail_closed_before_native_side_effect",
            ),
            "admin" => (
                true,
                true,
                true,
                true,
                true,
                "signed_admin_route_or_operator_command",
                "fail_closed_on_overwatch_or_policy_gap",
            ),
            "break_glass" => (
                true,
                true,
                true,
                true,
                true,
                "signed_break_glass_operator_command",
                "strict_fail_closed_with_overwatch_required",
            ),
            _ => (
                true,
                true,
                true,
                true,
                true,
                "unknown_command_class_rejected",
                "fail_closed_unknown_class",
            ),
        };
        Self {
            matrix_ref: PHASE6_COMMAND_CLASS_MATRIX_REF,
            command_class,
            requires_signature,
            requires_tenant: command_class != "low_risk_read",
            requires_idempotency,
            requires_policy,
            requires_quota,
            requires_audit,
            forwarding_mode,
            fail_closed_behavior,
            matrix_state: "matrix_entry_complete_phase6",
            reason_code: "overgate.command_class_matrix_phase6",
        }
    }

    pub fn matrix_complete(&self) -> bool {
        !self.command_class.is_empty()
            && !self.forwarding_mode.is_empty()
            && !self.fail_closed_behavior.is_empty()
            && self.matrix_state == "matrix_entry_complete_phase6"
            && self.reason_code == "overgate.command_class_matrix_phase6"
    }
}

impl QuotaPrecheckRecord {
    fn from_input(
        input: &PrecheckInput,
        command_class: &CommandClassAdmission,
        rate_limit: &RateLimitDecision,
    ) -> Self {
        let quota_precheck_ref = format!(
            "quota_precheck:overgate:phase6:{}",
            stable_short_token(&[
                input.tenant_id.as_str(),
                input.actor_id.as_str(),
                input.command_type.as_str(),
                input.request_hash.as_str(),
            ])
        );
        let budget_ref = format!(
            "budget:overgate:phase6:{}",
            stable_short_token(&[input.tenant_id.as_str(), command_class.command_class])
        );
        let grant_placeholder_refs = vec![format!(
            "grant_placeholder:overgrant:phase6:{}",
            stable_short_token(&[input.tenant_id.as_str(), input.credential_id.as_str()])
        )];
        let local_counter_ref = format!(
            "local_counter:overgate:phase6:{}",
            stable_short_token(&[
                input.tenant_id.as_str(),
                input.actor_id.as_str(),
                rate_limit.bucket_id.as_str(),
            ])
        );
        let allowed = !(input.command_type.contains("quota.denied")
            || input.command_type.contains("quota_exhausted")
            || input.command_type.contains("budget.denied")
            || input.command_type.contains("budget_exhausted"));
        let accepted_command_quota_refs = if allowed {
            vec![
                quota_precheck_ref.clone(),
                budget_ref.clone(),
                input.quota_scope_ref.clone(),
            ]
        } else {
            Vec::new()
        };
        Self {
            record_id: format!(
                "quota_record:overgate:phase6:{}",
                stable_short_token(&[quota_precheck_ref.as_str(), input.trace_id.as_str()])
            ),
            quota_precheck_ref,
            tenant_id: input.tenant_id.clone(),
            actor_ref: input.actor_id.clone(),
            quota_scope_ref: input.quota_scope_ref.clone(),
            command_class: command_class.command_class,
            request_size_class: request_size_class(input.request_body_len),
            rate_limit_bucket_ref: rate_limit.bucket_id.clone(),
            budget_ref,
            grant_placeholder_refs,
            local_counter_ref,
            overmeter_snapshot_ref: Some(format!(
                "overmeter_snapshot:placeholder:phase6:{}",
                stable_short_token(&[input.tenant_id.as_str(), input.command_type.as_str()])
            )),
            accepted_command_quota_refs,
            allowed,
            no_balance_mutation: true,
            no_seal_ledger_entry: true,
            settlement_state: "not_settled_by_overgate",
            reason_code: if allowed {
                "overgate.quota_precheck_allowed_phase6"
            } else {
                "overgate.quota_precheck_denied"
            },
        }
    }

    fn client_denial_refs(&self) -> Vec<String> {
        let mut refs = vec![
            self.quota_precheck_ref.clone(),
            self.budget_ref.clone(),
            self.quota_scope_ref.clone(),
        ];
        refs.extend(self.grant_placeholder_refs.clone());
        refs
    }
}

impl PolicyCheckRecord {
    fn from_input(input: &PrecheckInput, command_class: &CommandClassAdmission) -> Self {
        Self::from_command_type(
            &input.command_type,
            &input.tenant_id,
            command_class,
            &input.trace_id,
        )
    }

    fn from_command_type(
        command_type: &str,
        tenant_id: &str,
        command_class: &CommandClassAdmission,
        trace_id: &str,
    ) -> Self {
        let required = command_class.requires_policy || command_type.contains("policy");
        let denied = command_type.contains("policy.deny") || command_type.contains("policy_denied");
        let missing = command_type.contains("policy.missing")
            || command_type.contains("policy_prerequisite_missing");
        let matched_rule_refs = if required {
            vec![format!(
                "policy_rule:overguard:phase6:{}",
                stable_short_token(&[command_type, trace_id])
            )]
        } else {
            Vec::new()
        };
        Self {
            tenant_id: tenant_id.to_owned(),
            dependency_id: "overguard",
            handoff_state: if required {
                "overguard_dry_run_handoff_phase6"
            } else {
                "policy_not_required_for_command_class_phase6"
            },
            required,
            allowed: !(denied || missing),
            policy_version: "overguard.policy.v0",
            matched_rule_refs,
            decision_ref: format!(
                "policy_decision:overguard:phase6:{}",
                stable_short_token(&[command_type, trace_id])
            ),
            missing_prerequisite_reasons: if missing {
                vec!["policy_prerequisite_missing".to_owned()]
            } else {
                Vec::new()
            },
            policy_truth_owner: "overguard",
            stored_policy_truth_in_overgate: false,
            reason_code: if denied {
                "overgate.policy_denied"
            } else if missing {
                "overgate.policy_prerequisite_missing"
            } else if required {
                "overgate.policy_handoff_allowed_phase6"
            } else {
                "overgate.policy_not_required_phase6"
            },
        }
    }
}

impl ClientDenialSurface {
    fn accepted(
        rate_limit: &RateLimitDecision,
        quota_precheck: &QuotaPrecheckRecord,
        policy_check: &PolicyCheckRecord,
    ) -> Self {
        Self {
            rate_limit_ref: rate_limit.bucket_id.clone(),
            quota_precheck_ref: quota_precheck.quota_precheck_ref.clone(),
            budget_ref: quota_precheck.budget_ref.clone(),
            grant_refs: quota_precheck.grant_placeholder_refs.clone(),
            policy_decision_ref: policy_check.decision_ref.clone(),
            stable_reason_code: "overgate.phase6_prechecks_passed",
            correction_ref: "client_action:none",
            sdk_cli_ui_native_safe: true,
        }
    }
}

pub fn command_class_matrix() -> Vec<CommandClassAdmission> {
    [
        "overgate.phase6.read.status",
        "overgate.phase6.tenant.update",
        "overgate.phase6.queue.workload.submit",
        "overgate.phase6.policy.evaluate",
        "overgate.phase6.accounting.quota.reserve",
        "overgate.phase6.storage.namespace.bind",
        "overgate.phase6.native_app.side_effect",
        "overgate.phase6.admin.expire",
        "overgate.phase6.break_glass.audit_override",
    ]
    .iter()
    .map(|command_type| CommandClassAdmission::from_command_type(command_type))
    .collect()
}

pub fn validate_command_class_matrix() -> bool {
    let classes = command_class_matrix();
    let names = classes
        .iter()
        .map(|entry| entry.command_class)
        .collect::<HashSet<_>>();
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
    required.iter().all(|name| names.contains(name))
        && classes.iter().all(CommandClassAdmission::matrix_complete)
}

fn command_class_name(command_type: &str) -> &'static str {
    if command_type.contains("break_glass") {
        "break_glass"
    } else if command_type.contains(".admin") {
        "admin"
    } else if command_type.contains("native_app") || command_type.contains(".native.") {
        "native_app_side_effect"
    } else if command_type.contains("storage") || command_type.contains("namespace") {
        "storage_namespace"
    } else if command_type.contains("accounting")
        || command_type.contains("ledger")
        || command_type.contains("quota")
        || command_type.contains("budget")
        || command_type.contains("grant")
        || command_type.contains("oru")
        || command_type.contains("seal")
    {
        "accounting_affecting"
    } else if command_type.contains("policy") {
        "policy_heavy"
    } else if command_type.contains("queue") || command_type.contains("workload") {
        "queue_producing_workload"
    } else if command_type.contains(".read") || command_type.contains(".get") {
        "low_risk_read"
    } else {
        "phase1_control_plane_mutation"
    }
}

fn request_size_class(len: usize) -> &'static str {
    if len <= 1024 {
        "small_request"
    } else if len <= 8192 {
        "medium_request"
    } else {
        "large_request"
    }
}

fn rate_window_id(timestamp: &str) -> String {
    let trimmed = timestamp.trim();
    if trimmed.len() >= 13 {
        format!("window:{}", &trimmed[..13])
    } else {
        "window:unknown".to_owned()
    }
}

fn rate_limit_bucket_id(scope: &RateLimitScope) -> String {
    format!(
        "rate_limit:overgate:phase6:{}",
        stable_short_token(&[
            scope.tenant_id.as_str(),
            scope.actor_id.as_str(),
            scope.command_class,
            scope.window_id.as_str(),
            scope.source_app_ref.as_str(),
        ])
    )
}

fn rate_limit_reset_ref(window_id: &str) -> String {
    format!(
        "rate_limit_reset:overgate:phase6:{}:next_window",
        stable_short_token(&[window_id])
    )
}

fn optional_service_account_ref(actor_id: &str) -> Option<String> {
    actor_id
        .starts_with("service_account:")
        .then(|| actor_id.to_owned())
}

fn source_app_ref(input: &PrecheckInput) -> String {
    format!(
        "source_app:overgate:phase6:{}",
        stable_short_token(&[
            input.tenant_id.as_str(),
            input.actor_id.as_str(),
            input.credential_id.as_str(),
        ])
    )
}

pub fn accepted_quota_refs(precheck: &PrecheckOutcome) -> Vec<String> {
    precheck.quota_precheck.accepted_command_quota_refs.clone()
}

pub fn precheck_digest_ref(precheck: &PrecheckOutcome) -> String {
    stable_hash_ref(&[
        precheck.rate_limit.bucket_id.as_str(),
        precheck.quota_precheck.quota_precheck_ref.as_str(),
        precheck.policy_check.decision_ref.as_str(),
    ])
}
