use std::path::{Path, PathBuf};

use overrid_contracts::{
    ArtifactRetentionClass, ArtifactRetentionPolicy, FlakeMetadata, HarnessRunStatus,
    RedactionScanReport,
};

use crate::fixtures::sanitize_identifier;

pub const DEFAULT_ARTIFACT_ROOT: &str = "target/overrid/integration_harness/artifacts";
pub const REDACTION_POLICY: &str = "secret_free_refs_only";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactCollectionRefs {
    pub redacted_log_refs: Vec<String>,
    pub overwatch_export_refs: Vec<String>,
    pub cli_output_refs: Vec<String>,
    pub api_payload_envelope_refs: Vec<String>,
    pub stack_health_refs: Vec<String>,
    pub fixture_version_refs: Vec<String>,
    pub schema_version_refs: Vec<String>,
    pub assertion_diff_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactSummary {
    pub run_id: String,
    pub bundle_ref: String,
    pub path: String,
    pub manifest_ref: String,
    pub redaction_report_ref: String,
    pub retention_class: ArtifactRetentionClass,
    pub redaction_policy: String,
    pub collection_refs: ArtifactCollectionRefs,
    pub redaction_report: RedactionScanReport,
    pub reproduction_command: String,
    pub flake_metadata: FlakeMetadata,
    pub retention_policy: ArtifactRetentionPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactLocator {
    root: PathBuf,
}

impl ArtifactLocator {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn default_for_repo(repo_root: impl AsRef<Path>) -> Self {
        Self::new(repo_root.as_ref().join(DEFAULT_ARTIFACT_ROOT))
    }

    pub fn lookup(&self, run_id: &str, retention_class: ArtifactRetentionClass) -> ArtifactSummary {
        let sanitized = sanitize_identifier(run_id);
        let collection_refs = ArtifactCollectionRefs::for_run(&sanitized);
        ArtifactSummary {
            run_id: sanitized.clone(),
            bundle_ref: format!("artifact:bundle:{sanitized}"),
            path: self
                .root
                .join(&sanitized)
                .join("bundle.json")
                .to_string_lossy()
                .into_owned(),
            manifest_ref: format!("artifact:manifest:{sanitized}"),
            redaction_report_ref: format!("artifact:redaction_report:{sanitized}"),
            retention_class,
            redaction_policy: REDACTION_POLICY.to_owned(),
            collection_refs,
            redaction_report: RedactionScanReport::passed(default_redacted_fields()),
            reproduction_command: format!("overrid test artifacts {sanitized} --json"),
            flake_metadata: FlakeMetadata::stable(),
            retention_policy: ArtifactRetentionPolicy::for_class(retention_class),
        }
    }
}

impl ArtifactCollectionRefs {
    pub fn for_run(run_id: &str) -> Self {
        let sanitized = sanitize_identifier(run_id);
        Self {
            redacted_log_refs: vec![format!("artifact:logs:{sanitized}:redacted")],
            overwatch_export_refs: vec![format!("artifact:overwatch:{sanitized}")],
            cli_output_refs: vec![format!("artifact:cli_output:{sanitized}")],
            api_payload_envelope_refs: vec![format!("artifact:api_envelope:{sanitized}:redacted")],
            stack_health_refs: vec![format!("artifact:health:{sanitized}:local_stack")],
            fixture_version_refs: vec![format!("artifact:fixture_version:{sanitized}")],
            schema_version_refs: vec!["artifact:schema_version:integration_harness_v0_1".to_owned()],
            assertion_diff_refs: vec![format!("artifact:assertion_diff:{sanitized}")],
        }
    }
}

impl ArtifactSummary {
    pub fn with_retention_class(mut self, retention_class: ArtifactRetentionClass) -> Self {
        self.retention_class = retention_class;
        self.retention_policy = ArtifactRetentionPolicy::for_class(retention_class);
        self
    }

    pub fn with_run_context(
        mut self,
        profile: &str,
        scenario_id: &str,
        fixture_manifest_refs: &[String],
        trace_root: &str,
        assertion_refs: &[String],
        flake_detected: bool,
    ) -> Self {
        let scenario = sanitize_identifier(scenario_id);
        let trace = sanitize_identifier(trace_root);
        let fixture_manifest = fixture_manifest_refs
            .first()
            .cloned()
            .unwrap_or_else(|| "fixture:phase0_smoke".to_owned());
        let artifact_output = repo_relative_artifact_output_path(&self.path, &self.run_id);
        self.manifest_ref = format!("artifact:manifest:{scenario}");
        self.redaction_report_ref = format!("artifact:redaction_report:{}", self.run_id);
        self.collection_refs = ArtifactCollectionRefs::for_run(&self.run_id);
        self.collection_refs.assertion_diff_refs = if assertion_refs.is_empty() {
            vec![format!("artifact:assertion_diff:{scenario}")]
        } else {
            assertion_refs
                .iter()
                .map(|assertion_ref| {
                    format!(
                        "artifact:assertion_diff:{}",
                        sanitize_identifier(assertion_ref)
                    )
                })
                .collect()
        };
        self.reproduction_command = format!(
            "overrid test scenario {} --profile {} --fixture {} --trace-root {} --artifact-output {} --json",
            scenario_id,
            sanitize_identifier(profile),
            sanitize_identifier(&fixture_manifest),
            trace,
            artifact_output,
        );
        self.flake_metadata = if flake_detected {
            FlakeMetadata::unstable_event_ordering(vec![
                format!("assertion:flake:{}", sanitize_identifier(scenario_id)),
                "unstable_event_ordering".to_owned(),
            ])
        } else {
            FlakeMetadata::stable()
        };
        self.redaction_report = scan_artifact_values(&[
            &self.reproduction_command,
            &self.bundle_ref,
            &self.manifest_ref,
            &self.redaction_report_ref,
        ]);
        self
    }
}

pub fn retention_class_for_outcome(
    status: HarnessRunStatus,
    gate_class: &str,
) -> ArtifactRetentionClass {
    match status {
        HarnessRunStatus::Failed | HarnessRunStatus::Blocked => {
            ArtifactRetentionClass::FailureEvidence
        }
        HarnessRunStatus::Passed if gate_class == "smoke" => ArtifactRetentionClass::SmokeCompact,
        HarnessRunStatus::Passed if gate_class == "release_candidate" => {
            ArtifactRetentionClass::ReleaseCandidate
        }
        HarnessRunStatus::Passed => ArtifactRetentionClass::PhaseGateEvidence,
        HarnessRunStatus::Planned | HarnessRunStatus::Running => {
            ArtifactRetentionClass::FailureEvidence
        }
    }
}

pub fn scan_artifact_values(values: &[&str]) -> RedactionScanReport {
    let mut rejected_markers = Vec::new();
    for value in values {
        let lower = value.to_ascii_lowercase();
        for marker in [
            "-----begin",
            "private key",
            "secret=",
            "token=",
            "signature=",
            "raw_key",
            "private_payload",
            "decrypted_rag",
            "fixture_key_material",
            "/users/",
            "/home/",
            "\\users\\",
        ] {
            if lower.contains(marker) {
                rejected_markers.push(marker.replace('\\', "_"));
            }
        }
    }
    rejected_markers.sort();
    rejected_markers.dedup();
    if rejected_markers.is_empty() {
        RedactionScanReport::passed(default_redacted_fields())
    } else {
        RedactionScanReport::failed(rejected_markers)
    }
}

fn default_redacted_fields() -> Vec<String> {
    vec![
        "headers.authorization".to_owned(),
        "payload.private".to_owned(),
        "fixture.key_material".to_owned(),
    ]
}

fn repo_relative_artifact_output_path(path: &str, run_id: &str) -> String {
    if let Some(index) = path.find(DEFAULT_ARTIFACT_ROOT) {
        return path[index..].to_owned();
    }
    format!(
        "{DEFAULT_ARTIFACT_ROOT}/{}/bundle.json",
        sanitize_identifier(run_id)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_lookup_is_stable_and_redacted() {
        let locator = ArtifactLocator::new("target/test-artifacts");
        let first = locator.lookup("run:Phase0 Smoke", ArtifactRetentionClass::FailureEvidence);
        let second = locator.lookup("run:Phase0 Smoke", ArtifactRetentionClass::FailureEvidence);
        assert_eq!(first, second);
        assert_eq!(first.run_id, "run_phase0_smoke");
        assert!(first.bundle_ref.starts_with("artifact:bundle:"));
        assert_eq!(first.redaction_policy, REDACTION_POLICY);
        assert!(first.redaction_report.scanner_passed);
        assert_eq!(
            first.retention_policy.retention_class,
            ArtifactRetentionClass::FailureEvidence
        );
        assert!(first
            .collection_refs
            .schema_version_refs
            .contains(&"artifact:schema_version:integration_harness_v0_1".to_owned()));
    }

    #[test]
    fn artifact_summary_adds_reproduction_flake_and_retention_context() {
        let locator = ArtifactLocator::new("/Users/example/repo/target/test-artifacts");
        let summary = locator
            .lookup("run:Phase8 Flake", ArtifactRetentionClass::FailureEvidence)
            .with_run_context(
                "local",
                "scenario_phase8_flake_detection",
                &["fixture:phase8_flake_detection".to_owned()],
                "trace:phase8:flake",
                &["assertion:phase8:unstable_event_ordering".to_owned()],
                true,
            );

        assert_eq!(
            summary.retention_policy.retention_class,
            ArtifactRetentionClass::FailureEvidence
        );
        assert!(summary
            .reproduction_command
            .contains("overrid test scenario scenario_phase8_flake_detection"));
        assert!(summary.reproduction_command.contains("--trace-root"));
        assert!(!summary.reproduction_command.contains("/Users/"));
        assert!(summary.flake_metadata.is_nondeterministic());
        assert!(summary.redaction_report.scanner_passed);
        assert!(summary
            .collection_refs
            .assertion_diff_refs
            .iter()
            .any(|value| value.contains("unstable_event_ordering")));
    }

    #[test]
    fn redaction_scanner_rejects_forbidden_artifact_material() {
        let report = scan_artifact_values(&[
            "token=abc",
            "-----BEGIN PRIVATE KEY-----",
            "/Users/example/private/path",
            "fixture_key_material",
        ]);

        assert!(!report.scanner_passed);
        assert!(report.rejected_markers.contains(&"token=".to_owned()));
        assert!(report.rejected_markers.contains(&"/users/".to_owned()));
    }

    #[test]
    fn retention_class_follows_run_outcome_and_gate_class() {
        assert_eq!(
            retention_class_for_outcome(HarnessRunStatus::Passed, "smoke"),
            ArtifactRetentionClass::SmokeCompact
        );
        assert_eq!(
            retention_class_for_outcome(HarnessRunStatus::Passed, "release_candidate"),
            ArtifactRetentionClass::ReleaseCandidate
        );
        assert_eq!(
            retention_class_for_outcome(HarnessRunStatus::Failed, "regression"),
            ArtifactRetentionClass::FailureEvidence
        );
    }
}
