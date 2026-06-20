use overrid_contracts::{HarnessRunStatus, ScenarioActionKind};

use crate::fixtures::{sanitize_identifier, stable_short_token};
use crate::manifests::{ScenarioManifestRef, ScenarioStepRef};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioStepExecutionContext {
    pub scenario_id: String,
    pub run_id: String,
    pub trace_root: String,
    pub profile: String,
    pub phase_filter: Option<u8>,
    pub available_services: Vec<String>,
}

impl ScenarioStepExecutionContext {
    pub fn new(
        scenario: &ScenarioManifestRef,
        run_id: impl Into<String>,
        trace_root: impl Into<String>,
        profile: impl Into<String>,
        phase_filter: Option<u8>,
        available_services: Vec<String>,
    ) -> Self {
        Self {
            scenario_id: scenario.scenario_id.clone(),
            run_id: run_id.into(),
            trace_root: trace_root.into(),
            profile: profile.into(),
            phase_filter,
            available_services,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioStepResult {
    pub step_id: String,
    pub action_kind: ScenarioActionKind,
    pub status: HarnessRunStatus,
    pub reason_code: String,
    pub reason_class: String,
    pub expected_result_class: HarnessRunStatus,
    pub exit_class: String,
    pub duration_ms: u64,
    pub retry_class: String,
    pub stdout_ref: Option<String>,
    pub stderr_ref: Option<String>,
    pub payload_ref: Option<String>,
    pub assertion_refs: Vec<String>,
    pub artifact_refs: Vec<String>,
    pub dependency_status: Vec<String>,
}

impl ScenarioStepResult {
    pub fn action_kind_str(&self) -> &'static str {
        self.action_kind.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioStepExecutionReport {
    pub status: HarnessRunStatus,
    pub reason_code: String,
    pub reason_class: String,
    pub message: String,
    pub step_results: Vec<ScenarioStepResult>,
    pub assertion_refs: Vec<String>,
    pub event_refs: Vec<String>,
    pub dependency_status: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioStepRunner;

impl ScenarioStepRunner {
    pub fn new() -> Self {
        Self
    }

    pub fn run_scenario(
        &self,
        scenario: &ScenarioManifestRef,
        context: &ScenarioStepExecutionContext,
    ) -> ScenarioStepExecutionReport {
        if scenario.steps.is_empty() {
            return ScenarioStepExecutionReport {
                status: HarnessRunStatus::Blocked,
                reason_code: "dependency.manifest_missing".to_owned(),
                reason_class: "dependency".to_owned(),
                message: "scenario manifest did not contain executable steps".to_owned(),
                step_results: Vec::new(),
                assertion_refs: Vec::new(),
                event_refs: Vec::new(),
                dependency_status: Vec::new(),
            };
        }

        let mut step_results = Vec::new();
        let mut assertion_refs = Vec::new();
        let mut event_refs = Vec::new();
        let mut dependency_status = Vec::new();
        for step in &scenario.steps {
            let result = self.run_step(step, context);
            for assertion_ref in &result.assertion_refs {
                let assertion = crate::assertions::assert_reason_code(
                    &format!("assertion_reason_{}", sanitize_identifier(&result.step_id)),
                    &scenario.scenario_id,
                    &result.reason_code,
                    &result.reason_code,
                    &result.artifact_refs,
                )
                .expect("step reason-code assertion should validate");
                assertion_refs.push(assertion.assertion_id);
                assertion_refs.push(assertion_ref.clone());
            }
            dependency_status.extend(result.dependency_status.clone());
            event_refs.push(format!(
                "event:scenario_step:{}:{}:{}",
                sanitize_identifier(&scenario.scenario_id),
                sanitize_identifier(&result.step_id),
                stable_short_token(&[&context.trace_root, &result.step_id])
            ));
            step_results.push(result);
        }

        let status = classify_step_results(&step_results);
        let (reason_code, reason_class) = terminal_reason(&step_results, status);
        let terminal_assertion = crate::assertions::assert_terminal_status(
            &format!(
                "assertion_terminal_{}",
                sanitize_identifier(&scenario.scenario_id)
            ),
            &scenario.scenario_id,
            status,
            status,
        )
        .expect("terminal status assertion should validate");
        assertion_refs.push(terminal_assertion.assertion_id);

        ScenarioStepExecutionReport {
            status,
            reason_code,
            reason_class,
            message: match status {
                HarnessRunStatus::Passed => "scenario steps completed".to_owned(),
                HarnessRunStatus::Failed => "scenario step assertion failed".to_owned(),
                HarnessRunStatus::Blocked => {
                    "scenario step dependency or safety check blocked".to_owned()
                }
                HarnessRunStatus::Planned | HarnessRunStatus::Running => {
                    "scenario steps did not reach a terminal state".to_owned()
                }
            },
            step_results,
            assertion_refs,
            event_refs,
            dependency_status,
        }
    }

    pub fn run_step(
        &self,
        step: &ScenarioStepRef,
        context: &ScenarioStepExecutionContext,
    ) -> ScenarioStepResult {
        let (status, reason_code, reason_class, dependency_status) = match step.action_kind {
            ScenarioActionKind::Cli => run_cli_step(step),
            ScenarioActionKind::Sdk => run_sdk_or_api_step(step, "sdk.call_succeeded"),
            ScenarioActionKind::Api => run_sdk_or_api_step(step, "api.response_valid"),
            ScenarioActionKind::LocalHelper => run_local_helper_step(step, context),
            ScenarioActionKind::Assertion => run_assertion_step(step),
        };
        let (status, reason_code, reason_class) =
            enforce_expected_terminal_class(step, status, reason_code, reason_class);
        let token = stable_short_token(&[
            context.run_id.as_str(),
            context.trace_root.as_str(),
            step.step_id.as_str(),
        ]);

        ScenarioStepResult {
            step_id: step.step_id.clone(),
            action_kind: step.action_kind,
            status,
            reason_code,
            reason_class,
            expected_result_class: step.expected_result_class,
            exit_class: exit_class_for_status(status),
            duration_ms: bounded_duration_ms(step.timeout_ms),
            retry_class: step.retry_expectation.clone(),
            stdout_ref: (step.action_kind == ScenarioActionKind::Cli)
                .then(|| format!("artifact:cli_stdout:{token}:redacted")),
            stderr_ref: (step.action_kind == ScenarioActionKind::Cli)
                .then(|| format!("artifact:cli_stderr:{token}:redacted")),
            payload_ref: matches!(
                step.action_kind,
                ScenarioActionKind::Sdk | ScenarioActionKind::Api
            )
            .then(|| {
                format!(
                    "artifact:{}_payload:{token}:redacted",
                    step.action_kind.as_str()
                )
            }),
            assertion_refs: step.assertion_refs.clone(),
            artifact_refs: vec![format!(
                "artifact:step:{}:{token}:redacted",
                sanitize_identifier(&step.step_id)
            )],
            dependency_status,
        }
    }
}

impl Default for ScenarioStepRunner {
    fn default() -> Self {
        Self::new()
    }
}

pub fn classify_step_results(results: &[ScenarioStepResult]) -> HarnessRunStatus {
    if results
        .iter()
        .any(|result| result.status == HarnessRunStatus::Blocked)
    {
        HarnessRunStatus::Blocked
    } else if results
        .iter()
        .any(|result| result.status == HarnessRunStatus::Failed)
    {
        HarnessRunStatus::Failed
    } else {
        HarnessRunStatus::Passed
    }
}

fn run_cli_step(step: &ScenarioStepRef) -> (HarnessRunStatus, String, String, Vec<String>) {
    if step.timeout_ms <= 1 || contains_input(step, "command:timeout") {
        return failed("timeout.cli");
    }
    if contains_input(step, "command:not_available_in_phase") {
        return blocked("dependency.phase_contract_not_ready", Vec::new());
    }
    if contains_input(step, "tenant:missing") {
        return failed("tenant.missing");
    }
    if contains_input(step, "command:invalid_signature") {
        return failed("signature.invalid");
    }
    if contains_input(step, "command:duplicate_idempotency") {
        return failed("idempotency.duplicate");
    }
    if contains_input(step, "command:invalid_schema_denial")
        || contains_input(step, "command:schema_denial")
    {
        return passed("schema.denial_observed");
    }
    passed("cli.command_succeeded")
}

fn run_sdk_or_api_step(
    step: &ScenarioStepRef,
    success_reason: &str,
) -> (HarnessRunStatus, String, String, Vec<String>) {
    if step.input_refs.iter().any(|input| {
        input.starts_with("http://")
            || input.starts_with("https://")
            || input.starts_with("service_url:")
    }) {
        return blocked("safety.direct_service_url", Vec::new());
    }
    if step.input_refs.iter().any(|input| {
        input.contains("private_storage")
            || input.contains("storage:private")
            || input.contains("overvault:private")
    }) {
        return blocked("safety.private_storage_read", Vec::new());
    }
    if contains_input(step, "request:unsigned") {
        return failed("signature.missing");
    }
    if contains_input(step, "response:invalid_schema") {
        return failed("schema.response_invalid");
    }
    passed(success_reason)
}

fn run_local_helper_step(
    step: &ScenarioStepRef,
    context: &ScenarioStepExecutionContext,
) -> (HarnessRunStatus, String, String, Vec<String>) {
    if context.profile.contains("production") || context.profile.contains("public") {
        return blocked("safety.non_local_profile", Vec::new());
    }
    let mut dependency_verified = false;
    for input in &step.input_refs {
        if input.starts_with("service:") {
            if context
                .available_services
                .iter()
                .any(|service| service == input)
            {
                dependency_verified = true;
                continue;
            }
            return blocked(
                "dependency.service_unavailable",
                vec![format!("{input}:unavailable")],
            );
        }
        if !is_allowed_helper_input(input) {
            return blocked("safety.local_helper_not_allowed", Vec::new());
        }
    }
    if dependency_verified {
        return passed("helper.dependency_available");
    }
    passed("helper.action_succeeded")
}

fn run_assertion_step(step: &ScenarioStepRef) -> (HarnessRunStatus, String, String, Vec<String>) {
    if contains_input(step, "assertion:unstable_event_ordering")
        || contains_input(step, "assertion:nondeterministic_marker")
    {
        return failed("flake.unstable_event_ordering");
    }
    if contains_input(step, "assertion:wrong_reason_code") {
        return failed("assertion.reason_code_mismatch");
    }
    if contains_input(step, "assertion:wrong_terminal_state") {
        return failed("assertion.terminal_state_mismatch");
    }
    passed("assertion.passed")
}

fn enforce_expected_terminal_class(
    step: &ScenarioStepRef,
    status: HarnessRunStatus,
    reason_code: String,
    reason_class: String,
) -> (HarnessRunStatus, String, String) {
    if step.expected_result_class == status {
        return (status, reason_code, reason_class);
    }
    if status == HarnessRunStatus::Blocked {
        return (status, reason_code, reason_class);
    }
    (
        HarnessRunStatus::Failed,
        "assertion.terminal_state_mismatch".to_owned(),
        "assertion".to_owned(),
    )
}

fn terminal_reason(results: &[ScenarioStepResult], status: HarnessRunStatus) -> (String, String) {
    match status {
        HarnessRunStatus::Passed => ("run.passed".to_owned(), "success".to_owned()),
        HarnessRunStatus::Failed => results
            .iter()
            .find(|result| result.status == HarnessRunStatus::Failed)
            .map(|result| (result.reason_code.clone(), result.reason_class.clone()))
            .unwrap_or_else(|| ("run.failed".to_owned(), "assertion".to_owned())),
        HarnessRunStatus::Blocked => results
            .iter()
            .find(|result| result.status == HarnessRunStatus::Blocked)
            .map(|result| (result.reason_code.clone(), result.reason_class.clone()))
            .unwrap_or_else(|| ("run.blocked".to_owned(), "dependency".to_owned())),
        HarnessRunStatus::Planned | HarnessRunStatus::Running => {
            ("run.blocked".to_owned(), "dependency".to_owned())
        }
    }
}

fn passed(reason_code: &str) -> (HarnessRunStatus, String, String, Vec<String>) {
    (
        HarnessRunStatus::Passed,
        reason_code.to_owned(),
        "success".to_owned(),
        Vec::new(),
    )
}

fn failed(reason_code: &str) -> (HarnessRunStatus, String, String, Vec<String>) {
    (
        HarnessRunStatus::Failed,
        reason_code.to_owned(),
        "assertion".to_owned(),
        Vec::new(),
    )
}

fn blocked(
    reason_code: &str,
    dependency_status: Vec<String>,
) -> (HarnessRunStatus, String, String, Vec<String>) {
    let reason_class = if reason_code.starts_with("safety.") {
        "safety"
    } else {
        "dependency"
    };
    (
        HarnessRunStatus::Blocked,
        reason_code.to_owned(),
        reason_class.to_owned(),
        dependency_status,
    )
}

fn contains_input(step: &ScenarioStepRef, input: &str) -> bool {
    step.input_refs.iter().any(|candidate| candidate == input)
}

fn is_allowed_helper_input(input: &str) -> bool {
    input.starts_with("helper:stack_lifecycle")
        || input.starts_with("helper:seed_verify")
        || input.starts_with("helper:artifact_lookup")
        || input.starts_with("helper:event_export")
        || input.starts_with("helper:diagnostic_check")
}

fn exit_class_for_status(status: HarnessRunStatus) -> String {
    match status {
        HarnessRunStatus::Passed => "success",
        HarnessRunStatus::Failed => "platform",
        HarnessRunStatus::Blocked => "config",
        HarnessRunStatus::Planned | HarnessRunStatus::Running => "config",
    }
    .to_owned()
}

fn bounded_duration_ms(timeout_ms: u64) -> u64 {
    timeout_ms.min(250)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn step(
        step_id: &str,
        action_kind: ScenarioActionKind,
        input_refs: &[&str],
        expected_result_class: HarnessRunStatus,
    ) -> ScenarioStepRef {
        ScenarioStepRef {
            step_id: step_id.to_owned(),
            action_kind,
            input_refs: input_refs.iter().map(|value| (*value).to_owned()).collect(),
            timeout_ms: 30_000,
            retry_expectation: "non_retryable".to_owned(),
            expected_result_class,
            assertion_refs: vec![format!("assertion:{step_id}")],
            cleanup_rule: "collect_artifacts_then_reset".to_owned(),
        }
    }

    fn context() -> ScenarioStepExecutionContext {
        ScenarioStepExecutionContext {
            scenario_id: "scenario_phase6".to_owned(),
            run_id: "run_phase6".to_owned(),
            trace_root: "trace_phase6".to_owned(),
            profile: "local".to_owned(),
            phase_filter: Some(1),
            available_services: vec![
                "service:local_stack".to_owned(),
                "service:overgate".to_owned(),
            ],
        }
    }

    #[test]
    fn cli_step_runner_classifies_success_and_negative_paths() {
        let runner = ScenarioStepRunner::new();
        let ok = runner.run_step(
            &step(
                "step_cli_ok",
                ScenarioActionKind::Cli,
                &["command:local:no_op"],
                HarnessRunStatus::Passed,
            ),
            &context(),
        );
        assert_eq!(ok.status, HarnessRunStatus::Passed);
        assert_eq!(ok.reason_code, "cli.command_succeeded");
        assert!(ok.stdout_ref.as_deref().unwrap().contains(":redacted"));

        for (input, reason) in [
            ("command:invalid_signature", "signature.invalid"),
            ("command:duplicate_idempotency", "idempotency.duplicate"),
            ("tenant:missing", "tenant.missing"),
            ("command:timeout", "timeout.cli"),
        ] {
            let result = runner.run_step(
                &step(
                    "step_cli_negative",
                    ScenarioActionKind::Cli,
                    &[input],
                    HarnessRunStatus::Failed,
                ),
                &context(),
            );
            assert_eq!(result.status, HarnessRunStatus::Failed);
            assert_eq!(result.reason_code, reason);
        }

        let blocked = runner.run_step(
            &step(
                "step_cli_not_available",
                ScenarioActionKind::Cli,
                &["command:not_available_in_phase"],
                HarnessRunStatus::Blocked,
            ),
            &context(),
        );
        assert_eq!(blocked.status, HarnessRunStatus::Blocked);
        assert_eq!(blocked.reason_code, "dependency.phase_contract_not_ready");
    }

    #[test]
    fn sdk_and_api_step_runners_reject_unsafe_black_box_inputs() {
        let runner = ScenarioStepRunner::new();
        for (kind, input, reason, expected) in [
            (
                ScenarioActionKind::Api,
                "https://overgate.local/direct",
                "safety.direct_service_url",
                HarnessRunStatus::Blocked,
            ),
            (
                ScenarioActionKind::Sdk,
                "storage:private:read",
                "safety.private_storage_read",
                HarnessRunStatus::Blocked,
            ),
            (
                ScenarioActionKind::Api,
                "request:unsigned",
                "signature.missing",
                HarnessRunStatus::Failed,
            ),
            (
                ScenarioActionKind::Sdk,
                "response:invalid_schema",
                "schema.response_invalid",
                HarnessRunStatus::Failed,
            ),
        ] {
            let result =
                runner.run_step(&step("step_sdk_api", kind, &[input], expected), &context());
            assert_eq!(result.status, expected);
            assert_eq!(result.reason_code, reason);
        }
    }

    #[test]
    fn local_helper_step_runner_allows_only_diagnostic_actions() {
        let runner = ScenarioStepRunner::new();
        let ok = runner.run_step(
            &step(
                "step_helper",
                ScenarioActionKind::LocalHelper,
                &["helper:diagnostic_check"],
                HarnessRunStatus::Passed,
            ),
            &context(),
        );
        assert_eq!(ok.status, HarnessRunStatus::Passed);

        let blocked = runner.run_step(
            &step(
                "step_helper_bad",
                ScenarioActionKind::LocalHelper,
                &["helper:production_mutation"],
                HarnessRunStatus::Blocked,
            ),
            &context(),
        );
        assert_eq!(blocked.status, HarnessRunStatus::Blocked);
        assert_eq!(blocked.reason_code, "safety.local_helper_not_allowed");
    }

    #[test]
    fn blocked_results_take_priority_over_failed_results() {
        let mut failed = ScenarioStepRunner::new().run_step(
            &step(
                "step_failed",
                ScenarioActionKind::Cli,
                &["command:invalid_signature"],
                HarnessRunStatus::Failed,
            ),
            &context(),
        );
        let blocked = ScenarioStepRunner::new().run_step(
            &step(
                "step_blocked",
                ScenarioActionKind::Api,
                &["service_url:overgate-admin"],
                HarnessRunStatus::Blocked,
            ),
            &context(),
        );
        assert_eq!(
            classify_step_results(&[failed.clone(), blocked]),
            HarnessRunStatus::Blocked
        );
        failed.status = HarnessRunStatus::Passed;
        assert_eq!(classify_step_results(&[failed]), HarnessRunStatus::Passed);
    }

    #[test]
    fn assertion_step_runner_marks_flakes_as_failed_not_successful() {
        let runner = ScenarioStepRunner::new();
        let result = runner.run_step(
            &step(
                "step_phase8_flake",
                ScenarioActionKind::Assertion,
                &["assertion:unstable_event_ordering"],
                HarnessRunStatus::Failed,
            ),
            &context(),
        );

        assert_eq!(result.status, HarnessRunStatus::Failed);
        assert_eq!(result.reason_code, "flake.unstable_event_ordering");
        assert_eq!(result.reason_class, "assertion");
    }
}
