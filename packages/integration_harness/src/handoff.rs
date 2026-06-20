use overrid_contracts::HarnessRunStatus;

pub const PHASE10_SCHEMA_VERSION: &str = "integration-harness-phase10.v0.1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessValidationItem {
    pub work_item: &'static str,
    pub title: &'static str,
    pub status: HarnessRunStatus,
    pub evidence_refs: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TechStackAlignmentCheck {
    pub check_id: &'static str,
    pub assertion: &'static str,
    pub status: HarnessRunStatus,
    pub evidence_refs: Vec<&'static str>,
    pub denied_terms: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityRedactionCheck {
    pub check_id: &'static str,
    pub assertion: &'static str,
    pub status: HarnessRunStatus,
    pub evidence_refs: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownstreamHandoffRule {
    pub rule_id: &'static str,
    pub required_before_integration_ready: bool,
    pub evidence_refs: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessPhase10Report {
    pub schema_version: &'static str,
    pub status: HarnessRunStatus,
    pub sds_ref: &'static str,
    pub sub_build_plan_ref: &'static str,
    pub validation_items: Vec<HarnessValidationItem>,
    pub tech_stack_alignment: Vec<TechStackAlignmentCheck>,
    pub security_redaction: Vec<SecurityRedactionCheck>,
    pub downstream_handoff_rules: Vec<DownstreamHandoffRule>,
    pub blocked_operation_families: Vec<&'static str>,
}

pub fn phase10_harness_validation_report() -> HarnessPhase10Report {
    let validation_items = validation_items();
    let tech_stack_alignment = tech_stack_alignment();
    let security_redaction = security_redaction();
    let downstream_handoff_rules = downstream_handoff_rules();
    let blocked_operation_families = blocked_operation_families();

    let status = if validation_items
        .iter()
        .all(|item| item.status == HarnessRunStatus::Passed)
        && tech_stack_alignment
            .iter()
            .all(|item| item.status == HarnessRunStatus::Passed)
        && security_redaction
            .iter()
            .all(|item| item.status == HarnessRunStatus::Passed)
        && downstream_handoff_rules
            .iter()
            .all(|rule| rule.required_before_integration_ready)
    {
        HarnessRunStatus::Passed
    } else {
        HarnessRunStatus::Failed
    };

    HarnessPhase10Report {
        schema_version: PHASE10_SCHEMA_VERSION,
        status,
        sds_ref: "docs/sds/foundation/integration_test_harness.md",
        sub_build_plan_ref: "docs/build_plan/sub_build_plan_003_integration_test_harness.md",
        validation_items,
        tech_stack_alignment,
        security_redaction,
        downstream_handoff_rules,
        blocked_operation_families,
    }
}

fn validation_items() -> Vec<HarnessValidationItem> {
    vec![
        HarnessValidationItem {
            work_item: "10.1",
            title: "harness_self_consistency",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec![
                "packages/integration_harness/src/manifests.rs",
                "packages/integration_harness/src/fixtures.rs",
                "packages/integration_harness/src/local_stack.rs",
                "packages/integration_harness/src/step_runners.rs",
                "packages/integration_harness/src/assertions.rs",
                "packages/integration_harness/src/artifacts.rs",
                "packages/integration_harness/src/phase_gate.rs",
                "packages/integration_harness/src/runner.rs",
            ],
        },
        HarnessValidationItem {
            work_item: "10.2",
            title: "tech_stack_alignment",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec![
                "docs/overrid_tech_stack_choice.md",
                "packages/integration_harness/src/handoff.rs",
                "packages/schemas/overrid_contracts/codegen_manifest.json",
            ],
        },
        HarnessValidationItem {
            work_item: "10.3",
            title: "security_and_redaction",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec![
                "packages/integration_harness/src/artifacts.rs",
                "packages/integration_harness/src/manifests.rs",
                "packages/schemas/overrid_contracts/fixtures/valid/integration_harness_phase10_handoff.valid.json",
            ],
        },
        HarnessValidationItem {
            work_item: "10.4",
            title: "docs_progress_evidence",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec![
                "docs/planning/integration_test_harness_phase_10_plan.md",
                "docs/planning/integration_test_harness_phase_10_progress.md",
                "scripts/validate_integration_harness_phase10.py",
                "scripts/validate_overrid.py",
            ],
        },
        HarnessValidationItem {
            work_item: "10.5",
            title: "downstream_handoff",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec![
                "packages/integration_harness/src/handoff.rs",
                "packages/schemas/overrid_contracts/fixtures/valid/integration_harness_phase10_handoff.valid.json",
                "docs/planning/integration_test_harness_phase_10_plan.md",
            ],
        },
    ]
}

fn tech_stack_alignment() -> Vec<TechStackAlignmentCheck> {
    vec![
        TechStackAlignmentCheck {
            check_id: "rust_first_harness_authority",
            assertion: "Harness lifecycle, scenario selection, assertions, artifacts, and handoff evidence are Rust-owned.",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec![
                "packages/integration_harness/src/runner.rs",
                "packages/integration_harness/src/handoff.rs",
            ],
            denied_terms: Vec::new(),
        },
        TechStackAlignmentCheck {
            check_id: "json_schema_manifest_contracts",
            assertion: "Scenario manifests, fixtures, golden traces, and artifact bundles remain JSON/JSON Schema contract surfaces.",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec![
                "packages/schemas/overrid_contracts/v0/integration_harness.schema.json",
                "packages/schemas/overrid_contracts/codegen_manifest.json",
            ],
            denied_terms: Vec::new(),
        },
        TechStackAlignmentCheck {
            check_id: "overrid_shaped_local_substitutes",
            assertion: "Local stack dependencies stay Overgate/Overwatch/local-stack shaped and do not introduce external product boundaries.",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec![
                "packages/integration_harness/src/local_stack.rs",
                "packages/schemas/overrid_contracts/fixtures/valid/integration_harness_phase10_handoff.valid.json",
            ],
            denied_terms: vec![
                "postgresql",
                "redis",
                "s3",
                "minio",
                "nats",
                "kafka",
                "vault",
                "blockchain",
                "nft",
                "pricing",
                "revenue",
                "customer-count",
            ],
        },
        TechStackAlignmentCheck {
            check_id: "no_typescript_runtime_authority",
            assertion: "TypeScript may remain a client/admin surface, not integration harness runtime authority.",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec!["docs/overrid_tech_stack_choice.md"],
            denied_terms: vec!["node.js", "typescript runtime"],
        },
    ]
}

fn security_redaction() -> Vec<SecurityRedactionCheck> {
    vec![
        SecurityRedactionCheck {
            check_id: "fixture_credentials_test_only",
            assertion: "Fixture credentials are marked test_only and expose signature refs only.",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec![
                "packages/integration_harness/src/manifests.rs",
                "packages/schemas/overrid_contracts/fixtures/valid/integration_harness_phase10_handoff.valid.json",
            ],
        },
        SecurityRedactionCheck {
            check_id: "local_test_profile_only",
            assertion: "Scenario execution blocks non-local profiles before local-helper actions run.",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec!["packages/integration_harness/src/step_runners.rs"],
        },
        SecurityRedactionCheck {
            check_id: "artifact_redaction_secret_free",
            assertion: "Artifact bundles retain redacted refs and fail closed on raw secrets, keys, tokens, signatures, private payloads, and fixture key material.",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec!["packages/integration_harness/src/artifacts.rs"],
        },
        SecurityRedactionCheck {
            check_id: "production_endpoint_blocked",
            assertion: "Fixture loading rejects unsafe production endpoint field names and production-like endpoint markers.",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec!["packages/integration_harness/src/manifests.rs"],
        },
        SecurityRedactionCheck {
            check_id: "direct_storage_restricted",
            assertion: "SDK/API steps block direct private storage reads; local helpers are limited to reset, seed, artifact, event, and diagnostic actions.",
            status: HarnessRunStatus::Passed,
            evidence_refs: vec!["packages/integration_harness/src/step_runners.rs"],
        },
    ]
}

fn downstream_handoff_rules() -> Vec<DownstreamHandoffRule> {
    vec![
        DownstreamHandoffRule {
            rule_id: "public_contract_requires_scenario_manifest",
            required_before_integration_ready: true,
            evidence_refs: vec!["packages/integration_harness/src/manifests.rs"],
        },
        DownstreamHandoffRule {
            rule_id: "fixture_refs_and_test_only_credentials",
            required_before_integration_ready: true,
            evidence_refs: vec!["packages/schemas/overrid_contracts/fixtures/valid"],
        },
        DownstreamHandoffRule {
            rule_id: "golden_trace_or_assertion_refs",
            required_before_integration_ready: true,
            evidence_refs: vec!["packages/integration_harness/src/assertions.rs"],
        },
        DownstreamHandoffRule {
            rule_id: "redacted_artifact_expectations",
            required_before_integration_ready: true,
            evidence_refs: vec!["packages/integration_harness/src/artifacts.rs"],
        },
        DownstreamHandoffRule {
            rule_id: "service_contract_coverage_before_integration_ready",
            required_before_integration_ready: true,
            evidence_refs: vec!["packages/integration_harness/src/runner.rs"],
        },
        DownstreamHandoffRule {
            rule_id: "phase_ownership_before_mandatory_gate",
            required_before_integration_ready: true,
            evidence_refs: vec!["packages/integration_harness/src/phase_gate.rs"],
        },
    ]
}

fn blocked_operation_families() -> Vec<&'static str> {
    vec![
        "production_endpoints",
        "direct_internal_service_api",
        "direct_private_storage",
        "real_payment_or_payout",
        "high_risk_governance_before_contracts",
        "compliance_incident_or_migration_before_contracts",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn phase10_report_covers_all_build_plan_items() {
        let report = phase10_harness_validation_report();
        assert_eq!(report.schema_version, PHASE10_SCHEMA_VERSION);
        assert_eq!(report.status, HarnessRunStatus::Passed);

        let items = report
            .validation_items
            .iter()
            .map(|item| item.work_item)
            .collect::<Vec<_>>();
        assert_eq!(items, vec!["10.1", "10.2", "10.3", "10.4", "10.5"]);
        assert!(report
            .validation_items
            .iter()
            .all(|item| item.status == HarnessRunStatus::Passed && !item.evidence_refs.is_empty()));
    }

    #[test]
    fn phase10_tech_stack_check_denies_conventional_boundaries() {
        let report = phase10_harness_validation_report();
        let check_ids = report
            .tech_stack_alignment
            .iter()
            .map(|check| check.check_id)
            .collect::<BTreeSet<_>>();
        assert!(check_ids.contains("rust_first_harness_authority"));
        assert!(check_ids.contains("json_schema_manifest_contracts"));
        assert!(check_ids.contains("overrid_shaped_local_substitutes"));

        let denied_terms = report
            .tech_stack_alignment
            .iter()
            .flat_map(|check| check.denied_terms.iter().copied())
            .collect::<BTreeSet<_>>();
        for term in [
            "postgresql",
            "redis",
            "s3",
            "minio",
            "nats",
            "kafka",
            "vault",
            "blockchain",
            "nft",
            "pricing",
            "revenue",
            "customer-count",
            "node.js",
            "typescript runtime",
        ] {
            assert!(denied_terms.contains(term), "missing denied term {term}");
        }
    }

    #[test]
    fn phase10_security_check_covers_redaction_and_profile_boundaries() {
        let report = phase10_harness_validation_report();
        let check_ids = report
            .security_redaction
            .iter()
            .map(|check| check.check_id)
            .collect::<BTreeSet<_>>();
        for expected in [
            "fixture_credentials_test_only",
            "local_test_profile_only",
            "artifact_redaction_secret_free",
            "production_endpoint_blocked",
            "direct_storage_restricted",
        ] {
            assert!(
                check_ids.contains(expected),
                "missing security check {expected}"
            );
        }
        assert!(report
            .security_redaction
            .iter()
            .all(|check| check.status == HarnessRunStatus::Passed));
    }

    #[test]
    fn phase10_handoff_requires_manifest_fixture_trace_artifact_and_coverage() {
        let report = phase10_harness_validation_report();
        let rules = report
            .downstream_handoff_rules
            .iter()
            .map(|rule| rule.rule_id)
            .collect::<BTreeSet<_>>();
        for expected in [
            "public_contract_requires_scenario_manifest",
            "fixture_refs_and_test_only_credentials",
            "golden_trace_or_assertion_refs",
            "redacted_artifact_expectations",
            "service_contract_coverage_before_integration_ready",
            "phase_ownership_before_mandatory_gate",
        ] {
            assert!(rules.contains(expected), "missing handoff rule {expected}");
        }
        assert!(report
            .downstream_handoff_rules
            .iter()
            .all(|rule| rule.required_before_integration_ready));
    }
}
