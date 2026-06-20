use std::collections::BTreeSet;

use overrid_contracts::{
    AssertionResult, GoldenTrace, GoldenTraceMode, HarnessContractError, HarnessRunStatus,
    SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
};

pub fn phase0_trace_contract(
    scenario_id: &str,
) -> Result<(GoldenTrace, AssertionResult), HarnessContractError> {
    let trace = GoldenTrace::new(
        "golden_trace_phase0_noop",
        GoldenTraceMode::Exact,
        vec![
            "event_command_accepted".to_owned(),
            "event_audit_written".to_owned(),
        ],
        vec![(
            "event_command_accepted".to_owned(),
            "event_audit_written".to_owned(),
        )],
        SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
    )?;
    let assertion = AssertionResult::passed("assertion_phase0_trace_order", scenario_id);
    assertion.validate()?;
    Ok((trace, assertion))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedTraceEvent {
    pub node_id: String,
    pub event_kind: String,
    pub schema_version: String,
    pub reason_code: String,
    pub diagnostic_extra: bool,
}

impl ObservedTraceEvent {
    pub fn new(
        node_id: impl Into<String>,
        event_kind: impl Into<String>,
        reason_code: impl Into<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            event_kind: event_kind.into(),
            schema_version: SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION.to_owned(),
            reason_code: reason_code.into(),
            diagnostic_extra: false,
        }
    }

    pub fn diagnostic(
        node_id: impl Into<String>,
        event_kind: impl Into<String>,
        reason_code: impl Into<String>,
    ) -> Self {
        Self {
            diagnostic_extra: true,
            ..Self::new(node_id, event_kind, reason_code)
        }
    }

    pub fn with_schema_version(mut self, schema_version: impl Into<String>) -> Self {
        self.schema_version = schema_version.into();
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedTrace {
    pub events: Vec<ObservedTraceEvent>,
    pub causal_edges: Vec<(String, String)>,
}

impl ObservedTrace {
    pub fn new(events: Vec<ObservedTraceEvent>, causal_edges: Vec<(String, String)>) -> Self {
        Self {
            events,
            causal_edges,
        }
    }

    pub fn event_ids(&self) -> Vec<String> {
        self.events
            .iter()
            .map(|event| event.node_id.clone())
            .collect()
    }

    pub fn state_event_ids(&self) -> Vec<String> {
        self.events
            .iter()
            .filter(|event| !event.diagnostic_extra)
            .map(|event| event.node_id.clone())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoldenTraceAssertion {
    pub assertion: AssertionResult,
    pub missing_nodes: Vec<String>,
    pub missing_edges: Vec<(String, String)>,
    pub forbidden_transitions: Vec<(String, String)>,
    pub extra_state_events: Vec<String>,
    pub order_mismatches: Vec<(String, String)>,
    pub unstable_reason_codes: Vec<String>,
    pub schema_version_mismatches: Vec<String>,
}

pub fn phase01_protocol_trace_contract(
    scenario_id: &str,
) -> Result<(GoldenTrace, AssertionResult), HarnessContractError> {
    let mut trace = GoldenTrace::new(
        "golden_trace_phase7_phase01_protocol",
        GoldenTraceMode::Exact,
        vec![
            "event_signed_command_admitted".to_owned(),
            "event_schema_denial_observed".to_owned(),
            "event_tenant_created".to_owned(),
            "event_identity_created".to_owned(),
            "event_key_created".to_owned(),
            "event_audit_written".to_owned(),
            "event_audit_read".to_owned(),
            "event_idempotency_observed".to_owned(),
            "event_pending_queue_transition".to_owned(),
        ],
        vec![
            (
                "event_signed_command_admitted".to_owned(),
                "event_schema_denial_observed".to_owned(),
            ),
            (
                "event_schema_denial_observed".to_owned(),
                "event_tenant_created".to_owned(),
            ),
            (
                "event_tenant_created".to_owned(),
                "event_identity_created".to_owned(),
            ),
            (
                "event_identity_created".to_owned(),
                "event_key_created".to_owned(),
            ),
            (
                "event_key_created".to_owned(),
                "event_audit_written".to_owned(),
            ),
            (
                "event_audit_written".to_owned(),
                "event_audit_read".to_owned(),
            ),
            (
                "event_audit_read".to_owned(),
                "event_idempotency_observed".to_owned(),
            ),
            (
                "event_idempotency_observed".to_owned(),
                "event_pending_queue_transition".to_owned(),
            ),
        ],
        SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
    )?;
    trace.stable_reason_codes = vec![
        "command.accepted".to_owned(),
        "schema.denial_observed".to_owned(),
        "tenant.created".to_owned(),
        "identity.created".to_owned(),
        "key.created".to_owned(),
        "audit.written".to_owned(),
        "audit.read".to_owned(),
        "idempotency.observed".to_owned(),
        "queue.pending".to_owned(),
    ];
    trace.forbidden_transitions = vec![(
        "event_pending_queue_transition".to_owned(),
        "event_signed_command_admitted".to_owned(),
    )];
    trace.validate()?;

    let assertion = AssertionResult::passed("assertion_phase7_phase01_protocol", scenario_id);
    assertion.validate()?;
    Ok((trace, assertion))
}

pub fn execution_loop_dag_trace_contract() -> Result<GoldenTrace, HarnessContractError> {
    phase7_dag_trace(
        "golden_trace_phase7_execution_loop",
        &[
            ("event_queue_admitted", "execution.queue", "queue.admitted"),
            (
                "event_scheduler_dispatched",
                "execution.scheduler",
                "scheduler.dispatched",
            ),
            ("event_lease_acquired", "execution.lease", "lease.acquired"),
            ("event_runner_started", "execution.runner", "runner.started"),
            (
                "event_result_recorded",
                "execution.result",
                "result.recorded",
            ),
            (
                "event_retry_evaluated",
                "execution.retry",
                "retry.evaluated",
            ),
            (
                "event_cancellation_checked",
                "execution.cancellation",
                "cancellation.checked",
            ),
            (
                "event_timeout_evaluated",
                "execution.timeout",
                "timeout.evaluated",
            ),
            (
                "event_dead_letter_checked",
                "execution.dead_letter",
                "dead_letter.checked",
            ),
            ("event_usage_recorded", "execution.usage", "usage.recorded"),
            ("event_audit_written", "execution.audit", "audit.written"),
        ],
        &[
            ("event_queue_admitted", "event_scheduler_dispatched"),
            ("event_scheduler_dispatched", "event_lease_acquired"),
            ("event_lease_acquired", "event_runner_started"),
            ("event_runner_started", "event_result_recorded"),
            ("event_result_recorded", "event_retry_evaluated"),
            ("event_result_recorded", "event_cancellation_checked"),
            ("event_result_recorded", "event_timeout_evaluated"),
            ("event_result_recorded", "event_dead_letter_checked"),
            ("event_result_recorded", "event_usage_recorded"),
            ("event_usage_recorded", "event_audit_written"),
        ],
        &[("event_audit_written", "event_runner_started")],
    )
}

pub fn policy_dispute_dag_trace_contract() -> Result<GoldenTrace, HarnessContractError> {
    phase7_dag_trace(
        "golden_trace_phase7_policy_dispute",
        &[
            (
                "event_policy_package_loaded",
                "policy.package",
                "policy.loaded",
            ),
            (
                "event_verification_requested",
                "verification.request",
                "verification.requested",
            ),
            (
                "event_verification_decided",
                "verification.decision",
                "verification.decided",
            ),
            (
                "event_dispute_window_opened",
                "dispute.window",
                "dispute.window_opened",
            ),
            (
                "event_dispute_evidence_recorded",
                "dispute.evidence",
                "dispute.evidence_recorded",
            ),
            (
                "event_policy_audit_written",
                "policy.audit",
                "audit.written",
            ),
        ],
        &[
            (
                "event_policy_package_loaded",
                "event_verification_requested",
            ),
            ("event_verification_requested", "event_verification_decided"),
            ("event_verification_decided", "event_dispute_window_opened"),
            (
                "event_dispute_window_opened",
                "event_dispute_evidence_recorded",
            ),
            (
                "event_dispute_evidence_recorded",
                "event_policy_audit_written",
            ),
        ],
        &[("event_policy_audit_written", "event_verification_requested")],
    )
}

pub fn accounting_ledger_dag_trace_contract() -> Result<GoldenTrace, HarnessContractError> {
    phase7_dag_trace(
        "golden_trace_phase7_accounting_ledger",
        &[
            ("event_usage_metered", "usage.meter", "usage.metered"),
            ("event_ledger_debited", "ledger.debit", "ledger.debited"),
            ("event_receipt_written", "receipt.write", "receipt.written"),
            ("event_receipt_read", "receipt.read", "receipt.read"),
            (
                "event_ledger_audit_written",
                "ledger.audit",
                "audit.written",
            ),
        ],
        &[
            ("event_usage_metered", "event_ledger_debited"),
            ("event_ledger_debited", "event_receipt_written"),
            ("event_receipt_written", "event_receipt_read"),
            ("event_receipt_read", "event_ledger_audit_written"),
        ],
        &[("event_receipt_written", "event_usage_metered")],
    )
}

pub fn assert_golden_trace(
    assertion_id: &str,
    scenario_id: &str,
    trace: &GoldenTrace,
    observed: &ObservedTrace,
) -> Result<GoldenTraceAssertion, HarnessContractError> {
    let observed_nodes = observed.event_ids();
    let observed_state_nodes = observed.state_event_ids();
    let observed_node_set = observed_nodes.iter().cloned().collect::<BTreeSet<_>>();
    let observed_edges = observed
        .causal_edges
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();

    let missing_nodes = trace
        .required_nodes
        .iter()
        .filter(|node| !observed_node_set.contains(*node))
        .cloned()
        .collect::<Vec<_>>();
    let missing_edges = trace
        .required_causal_edges
        .iter()
        .filter(|edge| !observed_edges.contains(*edge))
        .cloned()
        .collect::<Vec<_>>();
    let forbidden_transitions = trace
        .forbidden_transitions
        .iter()
        .filter(|(from, to)| {
            observed
                .causal_edges
                .iter()
                .any(|edge| edge.0 == *from && edge.1 == *to)
                || observed_state_nodes
                    .windows(2)
                    .any(|window| window[0] == *from && window[1] == *to)
        })
        .cloned()
        .collect::<Vec<_>>();
    let extra_state_events = observed
        .events
        .iter()
        .filter(|event| {
            !event.diagnostic_extra
                && event.node_id.starts_with("event_")
                && !trace.required_nodes.contains(&event.node_id)
        })
        .map(|event| event.node_id.clone())
        .collect::<Vec<_>>();
    let order_mismatches = if trace.mode == GoldenTraceMode::Exact
        && observed_state_nodes.len() >= trace.required_nodes.len()
        && observed_state_nodes[..trace.required_nodes.len()] != trace.required_nodes[..]
    {
        trace
            .required_nodes
            .iter()
            .zip(observed_state_nodes.iter())
            .filter(|(expected, actual)| expected.as_str() != actual.as_str())
            .map(|(expected, actual)| (expected.clone(), actual.clone()))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let unstable_reason_codes = observed
        .events
        .iter()
        .filter(|event| {
            !event.diagnostic_extra && !trace.stable_reason_codes.contains(&event.reason_code)
        })
        .map(|event| event.reason_code.clone())
        .collect::<Vec<_>>();
    let schema_version_mismatches = observed
        .events
        .iter()
        .filter(|event| event.schema_version != trace.schema_version.raw())
        .map(|event| event.node_id.clone())
        .collect::<Vec<_>>();

    let reason_code = first_failure_reason(
        &missing_nodes,
        &missing_edges,
        &forbidden_transitions,
        &extra_state_events,
        &order_mismatches,
        &unstable_reason_codes,
        &schema_version_mismatches,
    );
    let status = if reason_code == "assertion.passed" {
        HarnessRunStatus::Passed
    } else {
        HarnessRunStatus::Failed
    };
    let assertion = AssertionResult {
        assertion_id: assertion_id.to_owned(),
        scenario_id: scenario_id.to_owned(),
        status,
        reason_code,
        field_path: "$.golden_trace".to_owned(),
        expected_value: format!("{}:{}", trace.trace_id, trace.mode.as_str()),
        actual_value: observed_state_nodes.join(">"),
        artifact_refs: vec![format!("artifact:golden_trace:{}:redacted", trace.trace_id)],
    };
    assertion.validate()?;

    Ok(GoldenTraceAssertion {
        assertion,
        missing_nodes,
        missing_edges,
        forbidden_transitions,
        extra_state_events,
        order_mismatches,
        unstable_reason_codes,
        schema_version_mismatches,
    })
}

pub fn assert_reason_code(
    assertion_id: &str,
    scenario_id: &str,
    expected_reason_code: &str,
    actual_reason_code: &str,
    artifact_refs: &[String],
) -> Result<AssertionResult, HarnessContractError> {
    let status = if expected_reason_code == actual_reason_code {
        HarnessRunStatus::Passed
    } else {
        HarnessRunStatus::Failed
    };
    let assertion = AssertionResult {
        assertion_id: assertion_id.to_owned(),
        scenario_id: scenario_id.to_owned(),
        status,
        reason_code: if status == HarnessRunStatus::Passed {
            "assertion.passed".to_owned()
        } else {
            "assertion.reason_code_mismatch".to_owned()
        },
        field_path: "$.reason_code".to_owned(),
        expected_value: expected_reason_code.to_owned(),
        actual_value: actual_reason_code.to_owned(),
        artifact_refs: artifact_refs.to_vec(),
    };
    assertion.validate()?;
    Ok(assertion)
}

fn phase7_dag_trace(
    trace_id: &str,
    nodes: &[(&str, &str, &str)],
    edges: &[(&str, &str)],
    forbidden_edges: &[(&str, &str)],
) -> Result<GoldenTrace, HarnessContractError> {
    let mut trace = GoldenTrace::new(
        trace_id,
        GoldenTraceMode::Dag,
        nodes
            .iter()
            .map(|(node_id, _, _)| (*node_id).to_owned())
            .collect(),
        edges
            .iter()
            .map(|(from, to)| ((*from).to_owned(), (*to).to_owned()))
            .collect(),
        SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
    )?;
    trace.forbidden_transitions = forbidden_edges
        .iter()
        .map(|(from, to)| ((*from).to_owned(), (*to).to_owned()))
        .collect();
    trace.stable_reason_codes = nodes
        .iter()
        .map(|(_, _, reason_code)| (*reason_code).to_owned())
        .collect();
    if !trace
        .stable_reason_codes
        .iter()
        .any(|reason| reason == "assertion.passed")
    {
        trace
            .stable_reason_codes
            .push("assertion.passed".to_owned());
    }
    trace.validate()?;
    Ok(trace)
}

fn first_failure_reason(
    missing_nodes: &[String],
    missing_edges: &[(String, String)],
    forbidden_transitions: &[(String, String)],
    extra_state_events: &[String],
    order_mismatches: &[(String, String)],
    unstable_reason_codes: &[String],
    schema_version_mismatches: &[String],
) -> String {
    if !missing_nodes.is_empty() {
        "golden_trace.event_missing"
    } else if !missing_edges.is_empty() {
        "golden_trace.edge_missing"
    } else if !forbidden_transitions.is_empty() {
        "golden_trace.forbidden_transition"
    } else if !extra_state_events.is_empty() {
        "golden_trace.extra_state_event"
    } else if !order_mismatches.is_empty() {
        "golden_trace.exact_order_mismatch"
    } else if !unstable_reason_codes.is_empty() {
        "golden_trace.reason_code_unstable"
    } else if !schema_version_mismatches.is_empty() {
        "golden_trace.schema_version_mismatch"
    } else {
        "assertion.passed"
    }
    .to_owned()
}

pub fn assert_terminal_status(
    assertion_id: &str,
    scenario_id: &str,
    expected_status: HarnessRunStatus,
    actual_status: HarnessRunStatus,
) -> Result<AssertionResult, HarnessContractError> {
    let status = if expected_status == actual_status {
        HarnessRunStatus::Passed
    } else {
        HarnessRunStatus::Failed
    };
    let assertion = AssertionResult {
        assertion_id: assertion_id.to_owned(),
        scenario_id: scenario_id.to_owned(),
        status,
        reason_code: if status == HarnessRunStatus::Passed {
            "assertion.passed".to_owned()
        } else {
            "assertion.terminal_state_mismatch".to_owned()
        },
        field_path: "$.status".to_owned(),
        expected_value: expected_status.as_str().to_owned(),
        actual_value: actual_status.as_str().to_owned(),
        artifact_refs: vec![format!("artifact:status:{}", actual_status.as_str())],
    };
    assertion.validate()?;
    Ok(assertion)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase0_trace_contract_validates_existing_projection_types() {
        let (trace, assertion) = phase0_trace_contract("scenario_phase0_smoke").unwrap();
        trace
            .assert_observed_nodes(&[
                "event_command_accepted".to_owned(),
                "event_audit_written".to_owned(),
            ])
            .unwrap();
        assert_eq!(assertion.reason_code, "assertion.passed");
    }

    #[test]
    fn reason_code_assertion_reports_mismatch_without_secret_payloads() {
        let assertion = assert_reason_code(
            "assertion_reason_code",
            "scenario_phase6",
            "run.passed",
            "signature.invalid",
            &["artifact:step:signature:redacted".to_owned()],
        )
        .unwrap();
        assert_eq!(assertion.status, HarnessRunStatus::Failed);
        assert_eq!(assertion.reason_code, "assertion.reason_code_mismatch");
        assert_eq!(assertion.field_path, "$.reason_code");
        assert!(assertion.artifact_refs[0].ends_with(":redacted"));
    }

    #[test]
    fn terminal_status_assertion_uses_harness_status_vocabulary() {
        let assertion = assert_terminal_status(
            "assertion_terminal_status",
            "scenario_phase6",
            HarnessRunStatus::Blocked,
            HarnessRunStatus::Failed,
        )
        .unwrap();
        assert_eq!(assertion.status, HarnessRunStatus::Failed);
        assert_eq!(assertion.reason_code, "assertion.terminal_state_mismatch");
        assert_eq!(assertion.expected_value, "blocked");
        assert_eq!(assertion.actual_value, "failed");
    }

    #[test]
    fn phase7_exact_protocol_trace_passes_for_required_phase01_events() {
        let (trace, _) =
            phase01_protocol_trace_contract("scenario_phase7_exact_protocol_trace").unwrap();
        let observed = ObservedTrace::new(
            trace
                .required_nodes
                .iter()
                .zip(trace.stable_reason_codes.iter())
                .map(|(node, reason)| ObservedTraceEvent::new(node, "phase01.protocol", reason))
                .collect(),
            trace.required_causal_edges.clone(),
        );

        let assertion = assert_golden_trace(
            "assertion_phase7_exact_protocol_trace",
            "scenario_phase7_exact_protocol_trace",
            &trace,
            &observed,
        )
        .unwrap();

        assert_eq!(assertion.assertion.status, HarnessRunStatus::Passed);
        assert_eq!(assertion.assertion.reason_code, "assertion.passed");
        assert!(assertion.missing_nodes.is_empty());
        assert!(assertion.missing_edges.is_empty());
    }

    #[test]
    fn phase7_exact_protocol_trace_reports_reordered_events() {
        let (trace, _) =
            phase01_protocol_trace_contract("scenario_phase7_exact_protocol_trace").unwrap();
        let mut observed_events = trace
            .required_nodes
            .iter()
            .zip(trace.stable_reason_codes.iter())
            .map(|(node, reason)| ObservedTraceEvent::new(node, "phase01.protocol", reason))
            .collect::<Vec<_>>();
        observed_events.swap(1, 2);
        let observed = ObservedTrace::new(observed_events, trace.required_causal_edges.clone());

        let assertion = assert_golden_trace(
            "assertion_phase7_exact_protocol_trace",
            "scenario_phase7_exact_protocol_trace",
            &trace,
            &observed,
        )
        .unwrap();

        assert_eq!(assertion.assertion.status, HarnessRunStatus::Failed);
        assert_eq!(
            assertion.assertion.reason_code,
            "golden_trace.exact_order_mismatch"
        );
        assert!(!assertion.order_mismatches.is_empty());
    }

    #[test]
    fn phase7_dag_trace_allows_shuffled_events_and_diagnostic_extras() {
        let trace = execution_loop_dag_trace_contract().unwrap();
        let observed = ObservedTrace::new(
            vec![
                ObservedTraceEvent::new(
                    "event_usage_recorded",
                    "execution.usage",
                    "usage.recorded",
                ),
                ObservedTraceEvent::new(
                    "event_queue_admitted",
                    "execution.queue",
                    "queue.admitted",
                ),
                ObservedTraceEvent::diagnostic(
                    "event_execution_diagnostic_extra",
                    "execution.diagnostic",
                    "diagnostic.extra",
                ),
                ObservedTraceEvent::new(
                    "event_scheduler_dispatched",
                    "execution.scheduler",
                    "scheduler.dispatched",
                ),
                ObservedTraceEvent::new(
                    "event_lease_acquired",
                    "execution.lease",
                    "lease.acquired",
                ),
                ObservedTraceEvent::new(
                    "event_runner_started",
                    "execution.runner",
                    "runner.started",
                ),
                ObservedTraceEvent::new(
                    "event_result_recorded",
                    "execution.result",
                    "result.recorded",
                ),
                ObservedTraceEvent::new(
                    "event_retry_evaluated",
                    "execution.retry",
                    "retry.evaluated",
                ),
                ObservedTraceEvent::new(
                    "event_cancellation_checked",
                    "execution.cancellation",
                    "cancellation.checked",
                ),
                ObservedTraceEvent::new(
                    "event_timeout_evaluated",
                    "execution.timeout",
                    "timeout.evaluated",
                ),
                ObservedTraceEvent::new(
                    "event_dead_letter_checked",
                    "execution.dead_letter",
                    "dead_letter.checked",
                ),
                ObservedTraceEvent::new("event_audit_written", "execution.audit", "audit.written"),
            ],
            trace.required_causal_edges.clone(),
        );

        let assertion = assert_golden_trace(
            "assertion_phase7_execution_dag_trace",
            "scenario_phase7_execution_dag_trace",
            &trace,
            &observed,
        )
        .unwrap();

        assert_eq!(assertion.assertion.status, HarnessRunStatus::Passed);
        assert!(assertion.extra_state_events.is_empty());
        assert!(assertion.order_mismatches.is_empty());
    }

    #[test]
    fn phase7_dag_trace_reports_missing_edges_and_forbidden_transitions() {
        let trace = execution_loop_dag_trace_contract().unwrap();
        let observed = ObservedTrace::new(
            trace
                .required_nodes
                .iter()
                .zip(trace.stable_reason_codes.iter())
                .map(|(node, reason)| ObservedTraceEvent::new(node, "execution.loop", reason))
                .collect(),
            vec![(
                "event_audit_written".to_owned(),
                "event_runner_started".to_owned(),
            )],
        );

        let assertion = assert_golden_trace(
            "assertion_phase7_execution_dag_trace",
            "scenario_phase7_execution_dag_trace",
            &trace,
            &observed,
        )
        .unwrap();

        assert_eq!(assertion.assertion.status, HarnessRunStatus::Failed);
        assert_eq!(assertion.assertion.reason_code, "golden_trace.edge_missing");
        assert!(!assertion.missing_edges.is_empty());
        assert!(!assertion.forbidden_transitions.is_empty());
    }

    #[test]
    fn phase7_trace_reports_unstable_reason_and_schema_version() {
        let trace = accounting_ledger_dag_trace_contract().unwrap();
        let mut observed = ObservedTrace::new(
            trace
                .required_nodes
                .iter()
                .zip(trace.stable_reason_codes.iter())
                .map(|(node, reason)| ObservedTraceEvent::new(node, "accounting.ledger", reason))
                .collect(),
            trace.required_causal_edges.clone(),
        );
        observed.events[0].reason_code = "usage.private_payload_seen".to_owned();
        observed.events[1] = observed.events[1]
            .clone()
            .with_schema_version("integration-harness.v0.99");

        let assertion = assert_golden_trace(
            "assertion_phase7_accounting_ledger_trace",
            "scenario_phase7_accounting_ledger_trace",
            &trace,
            &observed,
        )
        .unwrap();

        assert_eq!(assertion.assertion.status, HarnessRunStatus::Failed);
        assert_eq!(
            assertion.assertion.reason_code,
            "golden_trace.reason_code_unstable"
        );
        assert_eq!(
            assertion.unstable_reason_codes,
            vec!["usage.private_payload_seen".to_owned()]
        );
        assert_eq!(
            assertion.schema_version_mismatches,
            vec!["event_ledger_debited".to_owned()]
        );
    }
}
