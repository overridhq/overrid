use overrid_contracts::{
    AssertionResult, GoldenTrace, GoldenTraceMode, HarnessContractError,
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
}
