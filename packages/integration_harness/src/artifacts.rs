use std::path::{Path, PathBuf};

use overrid_contracts::ArtifactRetentionClass;

use crate::fixtures::sanitize_identifier;

pub const DEFAULT_ARTIFACT_ROOT: &str = "target/overrid/integration_harness/artifacts";
pub const REDACTION_POLICY: &str = "secret_free_refs_only";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactSummary {
    pub run_id: String,
    pub bundle_ref: String,
    pub path: String,
    pub retention_class: ArtifactRetentionClass,
    pub redaction_policy: String,
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
        ArtifactSummary {
            run_id: sanitized.clone(),
            bundle_ref: format!("artifact:bundle:{sanitized}"),
            path: self
                .root
                .join(&sanitized)
                .join("bundle.json")
                .to_string_lossy()
                .into_owned(),
            retention_class,
            redaction_policy: REDACTION_POLICY.to_owned(),
        }
    }
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
    }
}
