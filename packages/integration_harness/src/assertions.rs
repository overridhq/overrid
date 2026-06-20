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
}
