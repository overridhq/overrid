use std::collections::BTreeSet;
use std::path::PathBuf;

use overrid_contracts::{ArtifactRetentionClass, HarnessRunStatus};

use crate::artifacts::{ArtifactLocator, ArtifactSummary};
use crate::fixtures::{sanitize_identifier, stable_short_token, DEFAULT_FIXTURE_SEED};
use crate::manifests::{FixtureManifestRef, ScenarioManifestRef};

pub const LOCAL_TEST_STATE_MARKER: &str = "overrid.local_test_state.v0";

const READY_STACK_COMPONENTS: &[(&str, bool)] = &[
    ("service:local_stack", true),
    ("service:overgate", true),
    ("service:overwatch", true),
    ("component:api", true),
    ("component:worker", true),
    ("component:overqueue_durable_state", true),
    ("component:event_log", true),
    ("component:object_artifact_stub", true),
    ("component:node_agent_simulator", true),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceHealthSummary {
    pub service_id: String,
    pub state: String,
    pub required: bool,
    pub reason_code: String,
    pub evidence_ref: String,
}

impl ServiceHealthSummary {
    fn ready(service_id: &str, required: bool, trace_root: &str) -> Self {
        Self {
            service_id: service_id.to_string(),
            state: "ready".to_string(),
            required,
            reason_code: "health.ready".to_string(),
            evidence_ref: format!(
                "health:{}:{}",
                sanitize_identifier(service_id),
                stable_short_token(&[trace_root])
            ),
        }
    }

    fn unavailable(service_id: &str, trace_root: &str) -> Self {
        Self {
            service_id: service_id.to_string(),
            state: "unavailable".to_string(),
            required: true,
            reason_code: "dependency.service_unavailable".to_string(),
            evidence_ref: format!(
                "health:{}:{}",
                sanitize_identifier(service_id),
                stable_short_token(&[trace_root])
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackSnapshot {
    pub profile: String,
    pub service_health: Vec<ServiceHealthSummary>,
    pub dependency_status: Vec<String>,
    pub event_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackReport {
    pub status: HarnessRunStatus,
    pub reason_code: String,
    pub reason_class: String,
    pub message: String,
    pub service_health: Vec<ServiceHealthSummary>,
    pub dependency_status: Vec<String>,
    pub reset_refs: Vec<String>,
    pub seed_refs: Vec<String>,
    pub diagnostic_refs: Vec<String>,
    pub smoke_refs: Vec<String>,
    pub artifacts: Vec<ArtifactSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackHarness {
    profile: String,
    artifact_root: PathBuf,
}

impl LocalStackHarness {
    pub fn new(profile: impl Into<String>, artifact_root: impl Into<PathBuf>) -> Self {
        Self {
            profile: profile.into(),
            artifact_root: artifact_root.into(),
        }
    }

    pub fn start_stack(&self, trace_root: &str) -> LocalStackSnapshot {
        let service_health = READY_STACK_COMPONENTS
            .iter()
            .map(|(service_id, required)| {
                ServiceHealthSummary::ready(service_id, *required, trace_root)
            })
            .collect::<Vec<_>>();
        let dependency_status = service_health
            .iter()
            .map(|health| format!("{}:{}", health.service_id, health.state))
            .collect::<Vec<_>>();
        let event_token = stable_short_token(&[trace_root]);

        LocalStackSnapshot {
            profile: self.profile.clone(),
            service_health,
            dependency_status,
            event_refs: vec![
                format!("event:local_stack:start_requested:{event_token}"),
                format!("event:local_stack:services_ready:{event_token}"),
                format!("event:local_stack:health_verified:{event_token}"),
            ],
        }
    }

    pub fn reset_stack(
        &self,
        fixtures: &[FixtureManifestRef],
        run_id: &str,
        trace_root: &str,
    ) -> LocalStackReport {
        let snapshot = self.start_stack(trace_root);
        let reset_refs = self.reset_refs(run_id);
        let seed_refs = self.seed(fixtures);
        let (diagnostic_refs, artifacts) = self.diagnostics(
            run_id,
            trace_root,
            ArtifactRetentionClass::PhaseGateEvidence,
        );

        LocalStackReport {
            status: HarnessRunStatus::Passed,
            reason_code: "run.passed".to_string(),
            reason_class: "success".to_string(),
            message: "Local/test stack reset and deterministic seed completed".to_string(),
            service_health: snapshot.service_health,
            dependency_status: snapshot.dependency_status,
            reset_refs,
            seed_refs,
            diagnostic_refs,
            smoke_refs: Vec::new(),
            artifacts,
        }
    }

    pub fn run_phase0_smoke(
        &self,
        scenario: &ScenarioManifestRef,
        fixtures: &[FixtureManifestRef],
        run_id: &str,
        trace_root: &str,
    ) -> LocalStackReport {
        let snapshot = self.start_stack(trace_root);
        let missing_services = self.missing_required_services(scenario, &snapshot);
        let reset_refs = self.reset_refs(run_id);
        let seed_refs = self.seed(fixtures);

        if missing_services.is_empty() {
            let (diagnostic_refs, artifacts) = self.diagnostics(
                run_id,
                trace_root,
                ArtifactRetentionClass::PhaseGateEvidence,
            );

            return LocalStackReport {
                status: HarnessRunStatus::Passed,
                reason_code: "run.passed".to_string(),
                reason_class: "success".to_string(),
                message:
                    "Phase 0 smoke orchestration completed through deterministic local stack hooks"
                        .to_string(),
                service_health: snapshot.service_health,
                dependency_status: snapshot.dependency_status,
                reset_refs,
                seed_refs,
                diagnostic_refs,
                smoke_refs: self.smoke_refs(run_id, trace_root),
                artifacts,
            };
        }

        self.blocked_dependency_report(
            snapshot,
            missing_services,
            reset_refs,
            seed_refs,
            run_id,
            trace_root,
        )
    }

    pub fn diagnostics(
        &self,
        run_id: &str,
        trace_root: &str,
        retention_class: ArtifactRetentionClass,
    ) -> (Vec<String>, Vec<ArtifactSummary>) {
        let trace_token = stable_short_token(&[trace_root]);
        let run_token = stable_short_token(&[run_id]);
        let diagnostic_refs = vec![
            format!("artifact:logs:{run_token}:redacted"),
            format!("artifact:health:local_stack:{trace_token}"),
            format!("artifact:overwatch:{trace_token}"),
            format!("artifact:cli_output:{run_token}"),
            format!("artifact:api_envelope:{trace_token}"),
            format!("artifact:fixture_version:{run_token}"),
        ];
        let artifacts =
            vec![ArtifactLocator::new(self.artifact_root.clone()).lookup(run_id, retention_class)];

        (diagnostic_refs, artifacts)
    }

    fn blocked_dependency_report(
        &self,
        snapshot: LocalStackSnapshot,
        missing_services: Vec<String>,
        reset_refs: Vec<String>,
        seed_refs: Vec<String>,
        run_id: &str,
        trace_root: &str,
    ) -> LocalStackReport {
        let mut service_health = snapshot.service_health;
        service_health.extend(
            missing_services
                .iter()
                .map(|service_id| ServiceHealthSummary::unavailable(service_id, trace_root)),
        );
        let mut dependency_status = snapshot.dependency_status;
        dependency_status.extend(
            missing_services
                .iter()
                .map(|service_id| format!("{service_id}:unavailable")),
        );
        let (diagnostic_refs, artifacts) =
            self.diagnostics(run_id, trace_root, ArtifactRetentionClass::FailureEvidence);

        LocalStackReport {
            status: HarnessRunStatus::Blocked,
            reason_code: "dependency.service_unavailable".to_string(),
            reason_class: "dependency".to_string(),
            message: format!(
                "Required local/test service unavailable: {}",
                missing_services.join(", ")
            ),
            service_health,
            dependency_status,
            reset_refs,
            seed_refs,
            diagnostic_refs,
            smoke_refs: Vec::new(),
            artifacts,
        }
    }

    fn missing_required_services(
        &self,
        scenario: &ScenarioManifestRef,
        snapshot: &LocalStackSnapshot,
    ) -> Vec<String> {
        let ready_services = snapshot
            .service_health
            .iter()
            .filter(|health| health.state == "ready")
            .map(|health| health.service_id.as_str())
            .collect::<BTreeSet<_>>();

        scenario
            .required_services
            .iter()
            .filter(|service_id| !ready_services.contains(service_id.as_str()))
            .cloned()
            .collect()
    }

    fn reset_refs(&self, run_id: &str) -> Vec<String> {
        let profile = sanitize_identifier(&self.profile);
        let run_token = stable_short_token(&[run_id]);
        vec![
            format!("reset:{profile}:marker:{LOCAL_TEST_STATE_MARKER}"),
            format!("cleanup:{profile}:queue_state:{run_token}"),
            format!("cleanup:{profile}:event_log:{run_token}"),
            format!("cleanup:{profile}:object_artifact_stub:{run_token}"),
            format!("cleanup:{profile}:node_agent_simulator:{run_token}"),
        ]
    }

    fn seed(&self, fixtures: &[FixtureManifestRef]) -> Vec<String> {
        if fixtures.is_empty() {
            return vec![
                format!("seed:default:fixture:{DEFAULT_FIXTURE_SEED}"),
                format!("seed:default:marker:{LOCAL_TEST_STATE_MARKER}"),
            ];
        }

        let mut refs = Vec::new();
        for fixture in fixtures {
            let fixture_token = stable_short_token(&[fixture.fixture_id.as_str()]);
            refs.push(format!(
                "seed:fixture:{}",
                sanitize_identifier(&fixture.fixture_id)
            ));
            refs.push(format!(
                "seed:tenant:{}",
                sanitize_identifier(&fixture.tenant_ref)
            ));
            refs.push(format!(
                "seed:actor:{}",
                sanitize_identifier(&fixture.actor_ref)
            ));
            refs.push(format!("seed:test_key:{fixture_token}"));
            refs.push(format!("seed:node_agent_simulator:{fixture_token}"));
            refs.push(format!("seed:manifest:{fixture_token}"));
            refs.push(format!("seed:workload:local_noop:{fixture_token}"));
            refs.push(format!("seed:package:local_noop:{fixture_token}"));
            refs.push(format!("seed:local_oru_account:{fixture_token}"));
            refs.push(format!("seed:policy_context:{fixture_token}"));
            refs.push(format!("seed:marker:{LOCAL_TEST_STATE_MARKER}"));
        }
        refs
    }

    fn smoke_refs(&self, run_id: &str, trace_root: &str) -> Vec<String> {
        let trace_token = stable_short_token(&[trace_root]);
        let run_token = stable_short_token(&[run_id]);
        vec![
            format!("smoke:signed_noop_command:{trace_token}"),
            format!("smoke:audit_log_write_read:{trace_token}"),
            format!("smoke:invalid_schema_denial:{trace_token}"),
            format!("smoke:redacted_artifact_export:{run_token}"),
        ]
    }
}
