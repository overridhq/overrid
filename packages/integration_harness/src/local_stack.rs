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
    ("service:overkey", true),
    ("service:overwatch", true),
    ("component:api", true),
    ("component:worker", true),
    ("component:overqueue_durable_state", true),
    ("component:event_log", true),
    ("component:object_artifact_stub", true),
    ("component:node_agent_simulator", true),
    ("component:diagnostic_log_stream", false),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocalStackProfileMode {
    CleanStart,
    AlreadyRunning,
    HealthTimeout,
    PortConflict,
    DegradedOptional,
    ResetIncomplete,
    UnmarkedState,
}

impl LocalStackProfileMode {
    fn from_profile(profile: &str) -> Self {
        match profile {
            "local-already-running" => Self::AlreadyRunning,
            "local-health-timeout" => Self::HealthTimeout,
            "local-port-conflict" => Self::PortConflict,
            "local-degraded-optional" => Self::DegradedOptional,
            "local-reset-incomplete" => Self::ResetIncomplete,
            "local-unmarked-state" => Self::UnmarkedState,
            _ => Self::CleanStart,
        }
    }
}

pub fn is_local_test_profile(profile: &str) -> bool {
    matches!(
        profile,
        "local"
            | "local-dev"
            | "test"
            | "ci"
            | "local-already-running"
            | "local-health-timeout"
            | "local-port-conflict"
            | "local-degraded-optional"
            | "local-reset-incomplete"
            | "local-unmarked-state"
    )
}

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
        Self::with_state(service_id, "ready", required, "health.ready", trace_root)
    }

    fn with_state(
        service_id: &str,
        state: &str,
        required: bool,
        reason_code: &str,
        trace_root: &str,
    ) -> Self {
        Self {
            service_id: service_id.to_string(),
            state: state.to_string(),
            required,
            reason_code: reason_code.to_string(),
            evidence_ref: format!(
                "health:{}:{}",
                sanitize_identifier(service_id),
                stable_short_token(&[trace_root])
            ),
        }
    }

    fn unavailable(service_id: &str, trace_root: &str) -> Self {
        Self::with_state(
            service_id,
            "unavailable",
            true,
            "dependency.service_unavailable",
            trace_root,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackSnapshot {
    pub profile: String,
    pub ready: bool,
    pub reason_code: String,
    pub reason_class: String,
    pub message: String,
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
    pub event_refs: Vec<String>,
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
        let mode = self.profile_mode();
        let service_health = READY_STACK_COMPONENTS
            .iter()
            .map(|(service_id, required)| match (mode, *service_id) {
                (LocalStackProfileMode::HealthTimeout, "component:api") => {
                    ServiceHealthSummary::with_state(
                        service_id,
                        "timeout",
                        *required,
                        "health.timeout",
                        trace_root,
                    )
                }
                (LocalStackProfileMode::PortConflict, "component:api") => {
                    ServiceHealthSummary::with_state(
                        service_id,
                        "port_conflict",
                        *required,
                        "health.port_conflict",
                        trace_root,
                    )
                }
                (LocalStackProfileMode::DegradedOptional, "component:diagnostic_log_stream") => {
                    ServiceHealthSummary::with_state(
                        service_id,
                        "degraded",
                        *required,
                        "health.degraded_optional",
                        trace_root,
                    )
                }
                _ => ServiceHealthSummary::ready(service_id, *required, trace_root),
            })
            .collect::<Vec<_>>();
        let dependency_status = service_health
            .iter()
            .map(|health| format!("{}:{}", health.service_id, health.state))
            .collect::<Vec<_>>();
        let event_token = stable_short_token(&[trace_root]);
        let ready = service_health
            .iter()
            .all(|health| !health.required || health.state == "ready");
        let (reason_code, reason_class, message) = if ready {
            (
                "health.ready",
                "success",
                "Local/test stack readiness verified",
            )
        } else {
            match mode {
                LocalStackProfileMode::HealthTimeout => (
                    "dependency.local_stack_unavailable",
                    "dependency",
                    "Local stack health checks timed out before readiness",
                ),
                LocalStackProfileMode::PortConflict => (
                    "dependency.local_stack_unavailable",
                    "dependency",
                    "Local stack startup blocked by a loopback port conflict",
                ),
                _ => (
                    "dependency.local_stack_unavailable",
                    "dependency",
                    "Local stack startup failed before readiness",
                ),
            }
        };
        let mut event_refs = vec![format!("event:local_stack:start_requested:{event_token}")];
        if mode == LocalStackProfileMode::AlreadyRunning {
            event_refs.push(format!(
                "event:local_stack:already_running_verified:{event_token}"
            ));
        }
        for health in &service_health {
            event_refs.push(format!(
                "event:local_stack:service_starting:{}:{event_token}",
                sanitize_identifier(&health.service_id)
            ));
            match health.state.as_str() {
                "ready" => event_refs.push(format!(
                    "event:local_stack:service_ready:{}:{event_token}",
                    sanitize_identifier(&health.service_id)
                )),
                "degraded" => event_refs.push(format!(
                    "event:local_stack:service_degraded:{}:{event_token}",
                    sanitize_identifier(&health.service_id)
                )),
                _ => event_refs.push(format!(
                    "event:local_stack:failed:{}:{event_token}",
                    sanitize_identifier(&health.service_id)
                )),
            }
        }
        if ready {
            event_refs.push(format!("event:local_stack:health_verified:{event_token}"));
        } else {
            event_refs.push(format!("event:local_stack:failed:{event_token}"));
        }

        LocalStackSnapshot {
            profile: self.profile.clone(),
            ready,
            reason_code: reason_code.to_string(),
            reason_class: reason_class.to_string(),
            message: message.to_string(),
            service_health,
            dependency_status,
            event_refs,
        }
    }

    pub fn status_stack(&self, trace_root: &str) -> LocalStackSnapshot {
        self.start_stack(trace_root)
    }

    pub fn reset_stack(
        &self,
        fixtures: &[FixtureManifestRef],
        run_id: &str,
        trace_root: &str,
    ) -> LocalStackReport {
        let snapshot = self.start_stack(trace_root);
        if !snapshot.ready {
            return self.start_blocked_report(snapshot, run_id, trace_root);
        }

        let reset_refs = self.reset_refs(run_id);
        if let Some(report) = self.reset_safety_blocked_report(
            snapshot.clone(),
            reset_refs.clone(),
            run_id,
            trace_root,
        ) {
            return report;
        }

        let seed_refs = self.seed(fixtures);
        let event_refs = self.lifecycle_event_refs(
            snapshot.event_refs,
            &[
                "reset_started",
                "reset_completed",
                "seed_started",
                "seed_completed",
            ],
            trace_root,
        );
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
            event_refs,
            artifacts,
        }
    }

    pub fn seed_stack(
        &self,
        fixtures: &[FixtureManifestRef],
        run_id: &str,
        trace_root: &str,
    ) -> LocalStackReport {
        let snapshot = self.start_stack(trace_root);
        if !snapshot.ready {
            return self.start_blocked_report(snapshot, run_id, trace_root);
        }

        if let Some(report) =
            self.reset_safety_blocked_report(snapshot.clone(), Vec::new(), run_id, trace_root)
        {
            return report;
        }

        let seed_refs = self.seed(fixtures);
        let event_refs = self.lifecycle_event_refs(
            snapshot.event_refs,
            &["seed_started", "seed_completed"],
            trace_root,
        );
        let (diagnostic_refs, artifacts) = self.diagnostics(
            run_id,
            trace_root,
            ArtifactRetentionClass::PhaseGateEvidence,
        );

        LocalStackReport {
            status: HarnessRunStatus::Passed,
            reason_code: "run.passed".to_string(),
            reason_class: "success".to_string(),
            message: "Local/test stack deterministic seed completed".to_string(),
            service_health: snapshot.service_health,
            dependency_status: snapshot.dependency_status,
            reset_refs: Vec::new(),
            seed_refs,
            diagnostic_refs,
            smoke_refs: Vec::new(),
            event_refs,
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
        if !snapshot.ready {
            return self.start_blocked_report(snapshot, run_id, trace_root);
        }

        let missing_services = self.missing_required_services(scenario, &snapshot);
        if !missing_services.is_empty() {
            return self.blocked_dependency_report(
                snapshot,
                missing_services,
                Vec::new(),
                Vec::new(),
                run_id,
                trace_root,
            );
        }

        let reset_refs = self.reset_refs(run_id);
        if let Some(report) = self.reset_safety_blocked_report(
            snapshot.clone(),
            reset_refs.clone(),
            run_id,
            trace_root,
        ) {
            return report;
        }

        let seed_refs = self.seed(fixtures);
        let event_refs = self.lifecycle_event_refs(
            snapshot.event_refs,
            &[
                "reset_started",
                "reset_completed",
                "seed_started",
                "seed_completed",
                "smoke_started",
                "smoke_completed",
            ],
            trace_root,
        );
        let (diagnostic_refs, artifacts) = self.diagnostics(
            run_id,
            trace_root,
            ArtifactRetentionClass::PhaseGateEvidence,
        );

        LocalStackReport {
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
            event_refs,
            artifacts,
        }
    }

    pub fn logs(&self, run_id: &str, trace_root: &str) -> Vec<String> {
        let (diagnostic_refs, _) = self.diagnostics(
            run_id,
            trace_root,
            ArtifactRetentionClass::PhaseGateEvidence,
        );
        diagnostic_refs
            .into_iter()
            .filter(|reference| reference.starts_with("artifact:logs:"))
            .collect()
    }

    pub fn health_snapshots(&self, trace_root: &str) -> Vec<ServiceHealthSummary> {
        self.start_stack(trace_root).service_health
    }

    pub fn event_export(&self, trace_root: &str) -> Vec<String> {
        self.start_stack(trace_root).event_refs
    }

    pub fn artifact_collection(&self, run_id: &str, trace_root: &str) -> Vec<ArtifactSummary> {
        let (_, artifacts) = self.diagnostics(
            run_id,
            trace_root,
            ArtifactRetentionClass::PhaseGateEvidence,
        );
        artifacts
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
            format!("artifact:reason_codes:{trace_token}"),
            format!("artifact:time_window:{trace_token}"),
            format!("artifact:reproduction_command:{run_token}:redacted"),
        ];
        let artifacts =
            vec![ArtifactLocator::new(self.artifact_root.clone()).lookup(run_id, retention_class)];

        (diagnostic_refs, artifacts)
    }

    fn start_blocked_report(
        &self,
        snapshot: LocalStackSnapshot,
        run_id: &str,
        trace_root: &str,
    ) -> LocalStackReport {
        let (diagnostic_refs, artifacts) =
            self.diagnostics(run_id, trace_root, ArtifactRetentionClass::FailureEvidence);

        LocalStackReport {
            status: HarnessRunStatus::Blocked,
            reason_code: snapshot.reason_code,
            reason_class: snapshot.reason_class,
            message: snapshot.message,
            service_health: snapshot.service_health,
            dependency_status: snapshot.dependency_status,
            reset_refs: Vec::new(),
            seed_refs: Vec::new(),
            diagnostic_refs,
            smoke_refs: Vec::new(),
            event_refs: snapshot.event_refs,
            artifacts,
        }
    }

    fn reset_safety_blocked_report(
        &self,
        snapshot: LocalStackSnapshot,
        reset_refs: Vec<String>,
        run_id: &str,
        trace_root: &str,
    ) -> Option<LocalStackReport> {
        let mode = self.profile_mode();
        let profile = sanitize_identifier(&self.profile);
        let (reason_code, reason_class, message, reset_refs) = match mode {
            LocalStackProfileMode::ResetIncomplete => {
                let mut refs = reset_refs;
                refs.push(format!("cleanup:{profile}:incomplete"));
                (
                    "dependency.reset_incomplete",
                    "dependency",
                    "Local/test cleanup incomplete; seed aborted",
                    refs,
                )
            }
            LocalStackProfileMode::UnmarkedState => (
                "safety.unmarked_test_state",
                "safety",
                "Reset target is missing the required local test-state marker",
                vec![format!("reset:{profile}:marker_missing")],
            ),
            _ => return None,
        };
        let (diagnostic_refs, artifacts) =
            self.diagnostics(run_id, trace_root, ArtifactRetentionClass::FailureEvidence);
        let event_refs = self.lifecycle_event_refs(
            snapshot.event_refs,
            &["reset_started", "failed"],
            trace_root,
        );

        Some(LocalStackReport {
            status: HarnessRunStatus::Blocked,
            reason_code: reason_code.to_string(),
            reason_class: reason_class.to_string(),
            message: message.to_string(),
            service_health: snapshot.service_health,
            dependency_status: snapshot.dependency_status,
            reset_refs,
            seed_refs: Vec::new(),
            diagnostic_refs,
            smoke_refs: Vec::new(),
            event_refs,
            artifacts,
        })
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
        let event_refs = self.lifecycle_event_refs(snapshot.event_refs, &["failed"], trace_root);

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
            event_refs,
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
            format!("smoke:fixture_tenant_created:{trace_token}"),
            format!("smoke:fixture_actor_created:{trace_token}"),
            format!("smoke:test_key_used:{trace_token}"),
            format!("smoke:signed_noop_command:{trace_token}"),
            format!("smoke:audit_log_write_read:{trace_token}"),
            format!("smoke:invalid_schema_denial:{trace_token}"),
            format!("smoke:redacted_artifact_export:{run_token}"),
        ]
    }

    fn profile_mode(&self) -> LocalStackProfileMode {
        LocalStackProfileMode::from_profile(&self.profile)
    }

    fn lifecycle_event_refs(
        &self,
        mut event_refs: Vec<String>,
        event_names: &[&str],
        trace_root: &str,
    ) -> Vec<String> {
        let event_token = stable_short_token(&[trace_root]);
        event_refs.extend(
            event_names
                .iter()
                .map(|event_name| format!("event:local_stack:{event_name}:{event_token}")),
        );
        event_refs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase8_named_hooks_expose_status_seed_logs_health_events_and_artifacts() {
        let harness = LocalStackHarness::new("local", "target/test-artifacts");

        let status = harness.status_stack("trace_phase8_status_hook");
        assert!(status.ready);
        assert!(status.service_health.iter().any(|health| {
            health.service_id == "component:node_agent_simulator" && health.state == "ready"
        }));

        let seed_report = harness.seed_stack(&[], "run_phase8_seed_hook", "trace_phase8_seed_hook");
        assert_eq!(seed_report.status, HarnessRunStatus::Passed);
        assert!(seed_report.reset_refs.is_empty());
        assert!(seed_report
            .seed_refs
            .iter()
            .any(|reference| reference.contains(LOCAL_TEST_STATE_MARKER)));
        assert!(seed_report
            .event_refs
            .iter()
            .any(|reference| reference.contains("seed_completed")));

        let logs = harness.logs("run_phase8_logs_hook", "trace_phase8_logs_hook");
        assert_eq!(logs.len(), 1);
        assert!(logs[0].starts_with("artifact:logs:"));
        assert!(logs[0].ends_with(":redacted"));

        let health = harness.health_snapshots("trace_phase8_health_hook");
        assert!(health
            .iter()
            .any(|summary| summary.service_id == "component:node_agent_simulator"));

        let events = harness.event_export("trace_phase8_event_hook");
        assert!(events
            .iter()
            .any(|reference| reference.starts_with("event:local_stack:")));

        let artifacts =
            harness.artifact_collection("run_phase8_artifact_hook", "trace_phase8_artifact_hook");
        assert!(!artifacts.is_empty());
    }

    #[test]
    fn phase8_seed_hook_blocks_unmarked_state() {
        let harness = LocalStackHarness::new("local-unmarked-state", "target/test-artifacts");
        let report =
            harness.seed_stack(&[], "run_phase8_seed_blocked", "trace_phase8_seed_blocked");

        assert_eq!(report.status, HarnessRunStatus::Blocked);
        assert_eq!(report.reason_code, "safety.unmarked_test_state");
        assert!(report.seed_refs.is_empty());
    }
}
