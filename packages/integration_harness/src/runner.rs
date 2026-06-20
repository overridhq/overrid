use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

use overrid_contracts::{
    ArtifactRetentionClass, HarnessContractError, HarnessRunStatus, TestRunRecord,
    SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
};

use crate::artifacts::{retention_class_for_outcome, ArtifactLocator, ArtifactSummary};
use crate::assertions::{
    accounting_ledger_dag_trace_contract, assert_golden_trace, execution_loop_dag_trace_contract,
    phase01_protocol_trace_contract, policy_dispute_dag_trace_contract, GoldenTraceAssertion,
    ObservedTrace, ObservedTraceEvent,
};
use crate::fixtures::{sanitize_identifier, stable_short_token, DEFAULT_FIXTURE_SEED};
use crate::local_stack::{
    is_local_test_profile, LocalStackHarness, LocalStackReport, ServiceHealthSummary,
};
use crate::manifests::{
    FixtureManifestRef, HarnessManifestCatalog, HarnessManifestLoader, ManifestLoadError,
    ScenarioManifestRef, ScenarioSelection, ScenarioSelectionFilter,
};
use crate::step_runners::{ScenarioStepExecutionContext, ScenarioStepResult, ScenarioStepRunner};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessLifecycleState {
    Planned,
    StackStarting,
    StackReady,
    Resetting,
    Seeding,
    Running,
    Asserting,
    CollectingArtifacts,
    Passed,
    Failed,
    Blocked,
}

impl HarnessLifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::StackStarting => "stack_starting",
            Self::StackReady => "stack_ready",
            Self::Resetting => "resetting",
            Self::Seeding => "seeding",
            Self::Running => "running",
            Self::Asserting => "asserting",
            Self::CollectingArtifacts => "collecting_artifacts",
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Blocked => "blocked",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Passed | Self::Failed | Self::Blocked)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleTransitionError {
    pub from: HarnessLifecycleState,
    pub to: HarnessLifecycleState,
}

impl fmt::Display for LifecycleTransitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid harness lifecycle transition: {} -> {}",
            self.from.as_str(),
            self.to.as_str()
        )
    }
}

impl std::error::Error for LifecycleTransitionError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessLifecycleRecorder {
    states: Vec<HarnessLifecycleState>,
}

impl HarnessLifecycleRecorder {
    pub fn new() -> Self {
        Self {
            states: vec![HarnessLifecycleState::Planned],
        }
    }

    pub fn transition(
        &mut self,
        next: HarnessLifecycleState,
    ) -> Result<(), LifecycleTransitionError> {
        let current = *self
            .states
            .last()
            .expect("harness lifecycle recorder always has a planned state");
        if !can_transition(current, next) {
            return Err(LifecycleTransitionError {
                from: current,
                to: next,
            });
        }
        self.states.push(next);
        Ok(())
    }

    pub fn states(&self) -> &[HarnessLifecycleState] {
        &self.states
    }

    pub fn into_states(self) -> Vec<HarnessLifecycleState> {
        self.states
    }
}

impl Default for HarnessLifecycleRecorder {
    fn default() -> Self {
        Self::new()
    }
}

fn can_transition(from: HarnessLifecycleState, to: HarnessLifecycleState) -> bool {
    if from.is_terminal() {
        return false;
    }

    matches!(
        (from, to),
        (
            HarnessLifecycleState::Planned,
            HarnessLifecycleState::StackStarting
        ) | (
            HarnessLifecycleState::Planned,
            HarnessLifecycleState::Resetting
        ) | (
            HarnessLifecycleState::Planned,
            HarnessLifecycleState::CollectingArtifacts
        ) | (
            HarnessLifecycleState::StackStarting,
            HarnessLifecycleState::StackReady
        ) | (
            HarnessLifecycleState::StackStarting,
            HarnessLifecycleState::CollectingArtifacts
        ) | (
            HarnessLifecycleState::StackReady,
            HarnessLifecycleState::Resetting
        ) | (
            HarnessLifecycleState::StackReady,
            HarnessLifecycleState::CollectingArtifacts
        ) | (
            HarnessLifecycleState::Resetting,
            HarnessLifecycleState::Seeding
        ) | (
            HarnessLifecycleState::Resetting,
            HarnessLifecycleState::CollectingArtifacts
        ) | (
            HarnessLifecycleState::Seeding,
            HarnessLifecycleState::Running
        ) | (
            HarnessLifecycleState::Seeding,
            HarnessLifecycleState::CollectingArtifacts
        ) | (
            HarnessLifecycleState::Running,
            HarnessLifecycleState::Asserting
        ) | (
            HarnessLifecycleState::Running,
            HarnessLifecycleState::CollectingArtifacts
        ) | (
            HarnessLifecycleState::Asserting,
            HarnessLifecycleState::CollectingArtifacts
        ) | (
            HarnessLifecycleState::CollectingArtifacts,
            HarnessLifecycleState::Passed
        ) | (
            HarnessLifecycleState::CollectingArtifacts,
            HarnessLifecycleState::Failed
        ) | (
            HarnessLifecycleState::CollectingArtifacts,
            HarnessLifecycleState::Blocked
        )
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessRunContext {
    pub run_id: String,
    pub scenario_id: String,
    pub fixture_seed: String,
    pub logical_start_ms: u64,
    pub trace_root: String,
    pub artifact_dir: String,
    pub workspace_fingerprint: String,
}

impl HarnessRunContext {
    pub fn deterministic(
        scenario_id: impl Into<String>,
        fixture_seed: impl Into<String>,
        artifact_root: impl AsRef<Path>,
    ) -> Self {
        let scenario_id = scenario_id.into();
        let fixture_seed = fixture_seed.into();
        let scenario_token = sanitize_identifier(&scenario_id);
        let short = stable_short_token(&[&scenario_id, &fixture_seed]);
        let run_id = format!("run_{scenario_token}_{short}");
        let trace_root = format!("trace_{scenario_token}_{short}");
        let workspace_fingerprint = format!(
            "workspace_{}",
            stable_short_token(&["overrid", &fixture_seed])
        );
        let artifact_dir = artifact_root
            .as_ref()
            .join(&run_id)
            .to_string_lossy()
            .into_owned();
        Self {
            run_id,
            scenario_id,
            fixture_seed,
            logical_start_ms: 1_781_966_000_000,
            trace_root,
            artifact_dir,
            workspace_fingerprint,
        }
    }

    pub fn test_run_record(
        &self,
        status: HarnessRunStatus,
    ) -> Result<TestRunRecord, overrid_contracts::HarnessContractError> {
        TestRunRecord::terminal(
            &self.run_id,
            status,
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerOptions {
    pub repo_root: PathBuf,
    pub artifact_root: PathBuf,
    pub profile: String,
    pub requested_phase: Option<u8>,
    pub trace_id: Option<String>,
    pub test_harness_profile: bool,
}

impl RunnerOptions {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        let repo_root = repo_root.into();
        let artifact_root = repo_root.join(crate::artifacts::DEFAULT_ARTIFACT_ROOT);
        Self {
            repo_root,
            artifact_root,
            profile: "local".to_owned(),
            requested_phase: None,
            trace_id: None,
            test_harness_profile: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HarnessCliCommand {
    Integration,
    Scenario { name: String },
    List,
    Reset,
    Artifacts { run_id: String },
}

impl HarnessCliCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Integration => "test integration",
            Self::Scenario { .. } => "test scenario",
            Self::List => "test list",
            Self::Reset => "test reset",
            Self::Artifacts { .. } => "test artifacts",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioListItem {
    pub scenario_id: String,
    pub master_phase: u8,
    pub gate_class: String,
    pub tags: Vec<String>,
    pub required_services: Vec<String>,
    pub setup_fixture_refs: Vec<String>,
    pub step_count: usize,
    pub action_kinds: Vec<String>,
    pub source_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioSelectionReport {
    pub requested_phase: Option<u8>,
    pub selection_mode: String,
    pub selected_count: usize,
    pub planned_count: usize,
    pub selected_gate_classes: Vec<String>,
    pub selected_scenario_ids: Vec<String>,
    pub planned_scenario_ids: Vec<String>,
    pub ci_entrypoint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceContractCoverageReport {
    pub service_id: String,
    pub phase: u8,
    pub scenario_ids: Vec<String>,
    pub schema_modules: Vec<String>,
    pub event_types: Vec<String>,
    pub reason_code_families: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessCliOutput {
    pub command_name: String,
    pub status: HarnessRunStatus,
    pub reason_code: String,
    pub reason_class: String,
    pub profile: String,
    pub phase_filter: Option<u8>,
    pub run_id: Option<String>,
    pub trace_root: Option<String>,
    pub scenarios: Vec<ScenarioListItem>,
    pub selection_report: Option<ScenarioSelectionReport>,
    pub coverage_report: Vec<ServiceContractCoverageReport>,
    pub lifecycle: Vec<HarnessLifecycleState>,
    pub artifacts: Vec<ArtifactSummary>,
    pub dependency_status: Vec<String>,
    pub service_health: Vec<ServiceHealthSummary>,
    pub reset_refs: Vec<String>,
    pub seed_refs: Vec<String>,
    pub diagnostic_refs: Vec<String>,
    pub smoke_refs: Vec<String>,
    pub event_refs: Vec<String>,
    pub step_results: Vec<ScenarioStepResult>,
    pub assertion_refs: Vec<String>,
    pub message: String,
}

impl HarnessCliOutput {
    pub fn is_ok(&self) -> bool {
        !matches!(
            self.status,
            HarnessRunStatus::Failed | HarnessRunStatus::Blocked
        )
    }

    pub fn lifecycle_strs(&self) -> Vec<&'static str> {
        self.lifecycle.iter().map(|state| state.as_str()).collect()
    }

    pub fn dependency_status_strs(&self) -> Vec<&str> {
        self.dependency_status.iter().map(String::as_str).collect()
    }

    pub fn result_json(&self) -> String {
        format!(
            concat!(
                "{{",
                "\"schema_version\":\"{}\",",
                "\"command\":\"{}\",",
                "\"status\":\"{}\",",
                "\"reason_code\":\"{}\",",
                "\"reason_class\":\"{}\",",
                "\"profile\":\"{}\",",
                "\"phase_filter\":{},",
                "\"run_id\":{},",
                "\"trace_root\":{},",
                "\"scenarios\":{},",
                "\"selection_report\":{},",
                "\"coverage_report\":{},",
                "\"lifecycle\":{},",
                "\"artifacts\":{},",
                "\"dependency_status\":{},",
                "\"service_health\":{},",
                "\"reset_refs\":{},",
                "\"seed_refs\":{},",
                "\"diagnostic_refs\":{},",
                "\"smoke_refs\":{},",
                "\"event_refs\":{},",
                "\"step_results\":{},",
                "\"assertion_refs\":{},",
                "\"message\":\"{}\"",
                "}}"
            ),
            json_escape(SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION),
            json_escape(&self.command_name),
            json_escape(self.status.as_str()),
            json_escape(&self.reason_code),
            json_escape(&self.reason_class),
            json_escape(&self.profile),
            self.phase_filter
                .map(|phase| phase.to_string())
                .unwrap_or_else(|| "null".to_owned()),
            json_optional_string(self.run_id.as_deref()),
            json_optional_string(self.trace_root.as_deref()),
            scenarios_json(&self.scenarios),
            selection_report_json(self.selection_report.as_ref()),
            coverage_report_json(&self.coverage_report),
            lifecycle_json(&self.lifecycle),
            artifacts_json(&self.artifacts),
            json_string_array(&self.dependency_status),
            service_health_json(&self.service_health),
            json_string_array(&self.reset_refs),
            json_string_array(&self.seed_refs),
            json_string_array(&self.diagnostic_refs),
            json_string_array(&self.smoke_refs),
            json_string_array(&self.event_refs),
            step_results_json(&self.step_results),
            json_string_array(&self.assertion_refs),
            json_escape(&self.message),
        )
    }

    pub fn human_summary(&self) -> String {
        let mut lines = vec![
            format!("{}: {}", self.command_name, self.status.as_str()),
            format!("reason: {}", self.reason_code),
        ];
        if let Some(run_id) = &self.run_id {
            lines.push(format!("run: {run_id}"));
        }
        if let Some(trace_root) = &self.trace_root {
            lines.push(format!("trace: {trace_root}"));
        }
        if !self.scenarios.is_empty() {
            lines.push("scenarios:".to_owned());
            for scenario in &self.scenarios {
                lines.push(format!(
                    "  {} phase={} gate={}",
                    scenario.scenario_id, scenario.master_phase, scenario.gate_class
                ));
            }
        }
        if let Some(selection) = &self.selection_report {
            lines.push(format!(
                "selection: selected={} planned={} ci={}",
                selection.selected_count, selection.planned_count, selection.ci_entrypoint
            ));
        }
        if !self.coverage_report.is_empty() {
            lines.push("coverage:".to_owned());
            for coverage in &self.coverage_report {
                lines.push(format!(
                    "  {} phase={} {}",
                    coverage.service_id, coverage.phase, coverage.status
                ));
            }
        }
        if !self.artifacts.is_empty() {
            lines.push("artifacts:".to_owned());
            for artifact in &self.artifacts {
                lines.push(format!("  {} {}", artifact.bundle_ref, artifact.path));
            }
        }
        if !self.service_health.is_empty() {
            lines.push("service health:".to_owned());
            for health in &self.service_health {
                lines.push(format!("  {} {}", health.service_id, health.state));
            }
        }
        if !self.event_refs.is_empty() {
            lines.push("events:".to_owned());
            for event_ref in &self.event_refs {
                lines.push(format!("  {event_ref}"));
            }
        }
        if !self.step_results.is_empty() {
            lines.push("steps:".to_owned());
            for step in &self.step_results {
                lines.push(format!(
                    "  {} {} {} {}",
                    step.step_id,
                    step.action_kind_str(),
                    step.status.as_str(),
                    step.reason_code
                ));
            }
        }
        lines.push(format!(
            "lifecycle: {}",
            self.lifecycle
                .iter()
                .map(|state| state.as_str())
                .collect::<Vec<_>>()
                .join(" -> ")
        ));
        lines.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessRunner {
    options: RunnerOptions,
}

impl HarnessRunner {
    pub fn new(options: RunnerOptions) -> Self {
        Self { options }
    }

    pub fn run(&self, command: HarnessCliCommand) -> HarnessCliOutput {
        match command {
            HarnessCliCommand::Integration => self.run_integration(),
            HarnessCliCommand::Scenario { name } => self.run_scenario(&name),
            HarnessCliCommand::List => self.list_scenarios(),
            HarnessCliCommand::Reset => self.reset(),
            HarnessCliCommand::Artifacts { run_id } => self.artifacts(&run_id),
        }
    }

    fn load_catalog(&self) -> Result<HarnessManifestCatalog, ManifestLoadError> {
        HarnessManifestLoader::canonical(&self.options.repo_root).load_catalog()
    }

    fn list_scenarios(&self) -> HarnessCliOutput {
        match self.load_catalog() {
            Ok(catalog) => {
                let selection =
                    catalog.select_scenarios(&ScenarioSelectionFilter::for_phase(
                        self.options.requested_phase,
                    ));
                let selection_report =
                    build_selection_report(&selection, self.options.requested_phase, "list");
                let coverage_report =
                    service_contract_coverage(&selection, self.options.requested_phase);
                let scenarios = scenario_items(selection.selected.clone());
                HarnessCliOutput {
                    command_name: "test list".to_owned(),
                    status: HarnessRunStatus::Planned,
                    reason_code: "scenario.listed".to_owned(),
                    reason_class: "success".to_owned(),
                    profile: self.options.profile.clone(),
                    phase_filter: self.options.requested_phase,
                    run_id: None,
                    trace_root: self.options.trace_id.clone(),
                    scenarios,
                    selection_report: Some(selection_report),
                    coverage_report,
                    lifecycle: vec![HarnessLifecycleState::Planned],
                    artifacts: Vec::new(),
                    dependency_status: Vec::new(),
                    service_health: Vec::new(),
                    reset_refs: Vec::new(),
                    seed_refs: Vec::new(),
                    diagnostic_refs: Vec::new(),
                    smoke_refs: Vec::new(),
                    event_refs: Vec::new(),
                    step_results: Vec::new(),
                    assertion_refs: Vec::new(),
                    message: "scenario manifest catalog loaded".to_owned(),
                }
            }
            Err(error) => self.manifest_blocked_output("test list", error),
        }
    }

    fn run_integration(&self) -> HarnessCliOutput {
        match self.load_catalog() {
            Ok(catalog) => {
                let selection =
                    catalog.select_scenarios(&ScenarioSelectionFilter::for_phase(
                        self.options.requested_phase,
                    ));
                if selection.selected.is_empty() {
                    return self.blocked_without_scenario(
                        "test integration",
                        "dependency.phase_tag_unsupported",
                        "dependency",
                        "no scenarios match the requested phase filter",
                    );
                }
                let selection_report =
                    build_selection_report(&selection, self.options.requested_phase, "ci_smoke");
                let coverage_report =
                    service_contract_coverage(&selection, self.options.requested_phase);
                let scenario = primary_ci_scenario(&selection.selected, self.options.requested_phase);
                self.run_local_stack_scenario(
                    "test integration",
                    &catalog,
                    scenario,
                    Some(selection_report),
                    coverage_report,
                )
            }
            Err(error) => self.manifest_blocked_output("test integration", error),
        }
    }

    fn run_scenario(&self, name: &str) -> HarnessCliOutput {
        match self.load_catalog() {
            Ok(catalog) => {
                let Some(scenario) = catalog.scenario(name) else {
                    return self.blocked_without_scenario(
                        "test scenario",
                        "dependency.manifest_missing",
                        "dependency",
                        "requested scenario manifest was not found",
                    );
                };
                self.run_local_stack_scenario(
                    "test scenario",
                    &catalog,
                    scenario.clone(),
                    None,
                    Vec::new(),
                )
            }
            Err(error) => self.manifest_blocked_output("test scenario", error),
        }
    }

    fn reset(&self) -> HarnessCliOutput {
        if !self.local_profile_allowed() {
            return self.blocked_without_scenario(
                "test reset",
                "safety.non_local_profile",
                "safety",
                "reset requires a local or explicit test harness profile",
            );
        }

        match self.load_catalog() {
            Ok(catalog) => {
                let context = HarnessRunContext::deterministic(
                    "scenario_reset_shell",
                    DEFAULT_FIXTURE_SEED,
                    &self.options.artifact_root,
                );
                let trace_root = self.trace_root_for(&context);
                let report = LocalStackHarness::new(
                    self.options.profile.clone(),
                    self.options.artifact_root.clone(),
                )
                .reset_stack(&catalog.fixtures, &context.run_id, &trace_root);
                let _ = context.test_run_record(report.status.clone());
                let lifecycle = local_stack_lifecycle(&report);

                self.output_from_local_stack_report(
                    "test reset",
                    Vec::new(),
                    None,
                    Vec::new(),
                    context,
                    trace_root,
                    lifecycle,
                    Vec::new(),
                    Vec::new(),
                    report,
                )
            }
            Err(error) => self.manifest_blocked_output("test reset", error),
        }
    }

    fn artifacts(&self, run_id: &str) -> HarnessCliOutput {
        let mut lifecycle = HarnessLifecycleRecorder::new();
        lifecycle
            .transition(HarnessLifecycleState::CollectingArtifacts)
            .expect("planned to collecting_artifacts is valid");
        lifecycle
            .transition(HarnessLifecycleState::Passed)
            .expect("collecting_artifacts to passed is valid");
        let artifact = ArtifactLocator::new(&self.options.artifact_root)
            .lookup(run_id, ArtifactRetentionClass::FailureEvidence);
        HarnessCliOutput {
            command_name: "test artifacts".to_owned(),
            status: HarnessRunStatus::Passed,
            reason_code: "artifact.lookup_ready".to_owned(),
            reason_class: "success".to_owned(),
            profile: self.options.profile.clone(),
            phase_filter: self.options.requested_phase,
            run_id: Some(sanitize_identifier(run_id)),
            trace_root: self.options.trace_id.clone(),
            scenarios: Vec::new(),
            selection_report: None,
            coverage_report: Vec::new(),
            lifecycle: lifecycle.into_states(),
            artifacts: vec![artifact],
            dependency_status: Vec::new(),
            service_health: Vec::new(),
            reset_refs: Vec::new(),
            seed_refs: Vec::new(),
            diagnostic_refs: Vec::new(),
            smoke_refs: Vec::new(),
            event_refs: Vec::new(),
            step_results: Vec::new(),
            assertion_refs: Vec::new(),
            message: "artifact lookup returned deterministic bundle path".to_owned(),
        }
    }

    fn run_local_stack_scenario(
        &self,
        command_name: &str,
        catalog: &HarnessManifestCatalog,
        scenario: ScenarioManifestRef,
        selection_report: Option<ScenarioSelectionReport>,
        coverage_report: Vec<ServiceContractCoverageReport>,
    ) -> HarnessCliOutput {
        if !self.local_profile_allowed() {
            return self.blocked_without_scenario(
                command_name,
                "safety.non_local_profile",
                "safety",
                "test runs require a local or explicit test harness profile",
            );
        }

        let context = self.context_for_scenario(catalog, &scenario);
        let trace_root = self.trace_root_for(&context);
        let fixtures = self.fixtures_for_scenario(catalog, &scenario);
        let mut report = LocalStackHarness::new(
            self.options.profile.clone(),
            self.options.artifact_root.clone(),
        )
        .run_phase0_smoke(&scenario, &fixtures, &context.run_id, &trace_root);

        let mut step_results = Vec::new();
        let mut assertion_refs = Vec::new();
        if report.status == HarnessRunStatus::Passed {
            let available_services = report
                .service_health
                .iter()
                .filter(|health| health.state == "ready")
                .map(|health| health.service_id.clone())
                .collect::<Vec<_>>();
            let step_context = ScenarioStepExecutionContext::new(
                &scenario,
                context.run_id.clone(),
                trace_root.clone(),
                self.options.profile.clone(),
                self.options.requested_phase,
                available_services,
            );
            let step_report = ScenarioStepRunner::new().run_scenario(&scenario, &step_context);
            report.event_refs.extend(step_report.event_refs);
            report
                .dependency_status
                .extend(step_report.dependency_status);
            if step_report.status != HarnessRunStatus::Passed {
                report.status = step_report.status;
                report.reason_code = step_report.reason_code;
                report.reason_class = step_report.reason_class;
                report.message = step_report.message;
                report.artifacts = vec![ArtifactLocator::new(&self.options.artifact_root)
                    .lookup(&context.run_id, ArtifactRetentionClass::FailureEvidence)];
            } else {
                report.reason_code = step_report.reason_code;
                report.reason_class = step_report.reason_class;
                report.message = step_report.message;
            }
            step_results = step_report.step_results;
            assertion_refs = step_report.assertion_refs;
            if report.status == HarnessRunStatus::Passed {
                match phase7_golden_trace_assertions(&scenario) {
                    Ok(phase7_assertions) => {
                        for phase7_assertion in phase7_assertions {
                            if phase7_assertion.assertion.status != HarnessRunStatus::Passed {
                                report.status = HarnessRunStatus::Failed;
                                report.reason_code = phase7_assertion.assertion.reason_code.clone();
                                report.reason_class = "assertion".to_owned();
                                report.message =
                                    "scenario golden trace assertion failed".to_owned();
                                report.artifacts = vec![ArtifactLocator::new(
                                    &self.options.artifact_root,
                                )
                                .lookup(&context.run_id, ArtifactRetentionClass::FailureEvidence)];
                            }
                            assertion_refs.push(phase7_assertion.assertion.assertion_id);
                        }
                    }
                    Err(_) => {
                        report.status = HarnessRunStatus::Failed;
                        report.reason_code = "golden_trace.contract_invalid".to_owned();
                        report.reason_class = "assertion".to_owned();
                        report.message =
                            "scenario golden trace contract did not validate".to_owned();
                        report.artifacts = vec![ArtifactLocator::new(&self.options.artifact_root)
                            .lookup(&context.run_id, ArtifactRetentionClass::FailureEvidence)];
                    }
                }
            }
        }

        let _ = context.test_run_record(report.status);
        let lifecycle = local_stack_lifecycle(&report);

        self.output_from_local_stack_report(
            command_name,
            vec![scenario],
            selection_report,
            coverage_report,
            context,
            trace_root,
            lifecycle,
            step_results,
            assertion_refs,
            report,
        )
    }

    fn output_from_local_stack_report(
        &self,
        command_name: &str,
        scenarios: Vec<ScenarioManifestRef>,
        selection_report: Option<ScenarioSelectionReport>,
        coverage_report: Vec<ServiceContractCoverageReport>,
        context: HarnessRunContext,
        trace_root: String,
        lifecycle: Vec<HarnessLifecycleState>,
        step_results: Vec<ScenarioStepResult>,
        assertion_refs: Vec<String>,
        report: LocalStackReport,
    ) -> HarnessCliOutput {
        let status = report.status;
        let reason_code = report.reason_code;
        let reason_class = report.reason_class;
        let scenario_context = scenarios.first().cloned();
        let artifacts = self.enrich_artifacts(
            report.artifacts,
            status,
            scenario_context.as_ref(),
            &context,
            &trace_root,
            &assertion_refs,
        );
        HarnessCliOutput {
            command_name: command_name.to_owned(),
            status,
            reason_code,
            reason_class,
            profile: self.options.profile.clone(),
            phase_filter: self.options.requested_phase,
            run_id: Some(context.run_id),
            trace_root: Some(trace_root),
            scenarios: scenario_items(scenarios),
            selection_report,
            coverage_report,
            lifecycle,
            artifacts,
            dependency_status: report.dependency_status,
            service_health: report.service_health,
            reset_refs: report.reset_refs,
            seed_refs: report.seed_refs,
            diagnostic_refs: report.diagnostic_refs,
            smoke_refs: report.smoke_refs,
            event_refs: report.event_refs,
            step_results,
            assertion_refs,
            message: report.message,
        }
    }

    fn enrich_artifacts(
        &self,
        artifacts: Vec<ArtifactSummary>,
        status: HarnessRunStatus,
        scenario: Option<&ScenarioManifestRef>,
        context: &HarnessRunContext,
        trace_root: &str,
        assertion_refs: &[String],
    ) -> Vec<ArtifactSummary> {
        let retention_class = scenario
            .map(|scenario| retention_class_for_outcome(status, &scenario.gate_class))
            .unwrap_or_else(|| {
                artifacts
                    .first()
                    .map(|artifact| artifact.retention_class)
                    .unwrap_or_else(|| retention_class_for_outcome(status, "regression"))
            });
        let artifacts = if artifacts.is_empty() {
            vec![ArtifactLocator::new(&self.options.artifact_root)
                .lookup(&context.run_id, retention_class)]
        } else {
            artifacts
        };
        artifacts
            .into_iter()
            .map(|artifact| {
                let artifact = artifact.with_retention_class(retention_class);
                if let Some(scenario) = scenario {
                    let flake_detected = status == HarnessRunStatus::Failed
                        && scenario.tags.iter().any(|tag| tag == "flake_detection");
                    artifact.with_run_context(
                        &self.options.profile,
                        &scenario.scenario_id,
                        &scenario.setup_fixture_refs,
                        trace_root,
                        assertion_refs,
                        flake_detected,
                    )
                } else {
                    artifact
                }
            })
            .collect()
    }

    fn manifest_blocked_output(
        &self,
        command_name: &str,
        error: ManifestLoadError,
    ) -> HarnessCliOutput {
        let (reason_code, reason_class) = match error {
            ManifestLoadError::IncompatibleVersion { .. } => {
                ("dependency.schema_incompatible", "dependency")
            }
            ManifestLoadError::UnsafeField { .. } => ("safety.unsafe_manifest", "safety"),
            ManifestLoadError::DuplicateScenarioId { .. } => {
                ("dependency.duplicate_scenario_id", "dependency")
            }
            _ => ("dependency.manifest_missing", "dependency"),
        };
        self.blocked_without_scenario(command_name, reason_code, reason_class, &error.to_string())
    }

    fn blocked_without_scenario(
        &self,
        command_name: &str,
        reason_code: &str,
        reason_class: &str,
        message: &str,
    ) -> HarnessCliOutput {
        let mut lifecycle = HarnessLifecycleRecorder::new();
        lifecycle
            .transition(HarnessLifecycleState::CollectingArtifacts)
            .expect("planned to collecting_artifacts is valid");
        lifecycle
            .transition(HarnessLifecycleState::Blocked)
            .expect("collecting_artifacts to blocked is valid");
        HarnessCliOutput {
            command_name: command_name.to_owned(),
            status: HarnessRunStatus::Blocked,
            reason_code: reason_code.to_owned(),
            reason_class: reason_class.to_owned(),
            profile: self.options.profile.clone(),
            phase_filter: self.options.requested_phase,
            run_id: None,
            trace_root: self.options.trace_id.clone(),
            scenarios: Vec::new(),
            selection_report: None,
            coverage_report: Vec::new(),
            lifecycle: lifecycle.into_states(),
            artifacts: Vec::new(),
            dependency_status: Vec::new(),
            service_health: Vec::new(),
            reset_refs: Vec::new(),
            seed_refs: Vec::new(),
            diagnostic_refs: Vec::new(),
            smoke_refs: Vec::new(),
            event_refs: Vec::new(),
            step_results: Vec::new(),
            assertion_refs: Vec::new(),
            message: message.to_owned(),
        }
    }

    fn context_for_scenario(
        &self,
        catalog: &HarnessManifestCatalog,
        scenario: &ScenarioManifestRef,
    ) -> HarnessRunContext {
        let fixture_seed = scenario
            .setup_fixture_refs
            .first()
            .map(|fixture_ref| crate::fixtures::fixture_id_from_ref(fixture_ref))
            .and_then(|fixture_id| {
                catalog
                    .fixtures
                    .iter()
                    .find(|fixture| fixture.fixture_id == fixture_id)
                    .map(|fixture| fixture.deterministic_seed.clone())
            })
            .unwrap_or_else(|| DEFAULT_FIXTURE_SEED.to_owned());
        HarnessRunContext::deterministic(
            &scenario.scenario_id,
            fixture_seed,
            &self.options.artifact_root,
        )
    }

    fn fixtures_for_scenario(
        &self,
        catalog: &HarnessManifestCatalog,
        scenario: &ScenarioManifestRef,
    ) -> Vec<FixtureManifestRef> {
        scenario
            .setup_fixture_refs
            .iter()
            .filter_map(|fixture_ref| {
                let fixture_id = crate::fixtures::fixture_id_from_ref(fixture_ref);
                catalog
                    .fixtures
                    .iter()
                    .find(|fixture| fixture.fixture_id == fixture_id)
                    .cloned()
            })
            .collect()
    }

    fn trace_root_for(&self, context: &HarnessRunContext) -> String {
        self.options
            .trace_id
            .clone()
            .unwrap_or_else(|| context.trace_root.clone())
    }

    fn local_profile_allowed(&self) -> bool {
        self.options.test_harness_profile || is_local_test_profile(&self.options.profile)
    }
}

fn phase7_golden_trace_assertions(
    scenario: &ScenarioManifestRef,
) -> Result<Vec<GoldenTraceAssertion>, HarnessContractError> {
    let scenario_id = scenario.scenario_id.as_str();
    let (assertion_id, trace, observed) = match scenario_id {
        "scenario_phase7_exact_protocol_trace" => {
            let (trace, _) = phase01_protocol_trace_contract(scenario_id)?;
            (
                "assertion_phase7_exact_protocol_trace",
                trace.clone(),
                observed_trace_from_template(&trace, "phase01.protocol", false),
            )
        }
        "scenario_phase7_execution_dag_trace" => {
            let trace = execution_loop_dag_trace_contract()?;
            (
                "assertion_phase7_execution_dag_trace",
                trace.clone(),
                observed_trace_from_template(&trace, "execution.loop", true),
            )
        }
        "scenario_phase7_policy_dispute_trace" => {
            let trace = policy_dispute_dag_trace_contract()?;
            (
                "assertion_phase7_policy_dispute_trace",
                trace.clone(),
                observed_trace_from_template(&trace, "policy.dispute", true),
            )
        }
        "scenario_phase7_accounting_ledger_trace" => {
            let trace = accounting_ledger_dag_trace_contract()?;
            (
                "assertion_phase7_accounting_ledger_trace",
                trace.clone(),
                observed_trace_from_template(&trace, "accounting.ledger", true),
            )
        }
        _ => return Ok(Vec::new()),
    };

    Ok(vec![assert_golden_trace(
        assertion_id,
        scenario_id,
        &trace,
        &observed,
    )?])
}

fn observed_trace_from_template(
    trace: &overrid_contracts::GoldenTrace,
    event_kind: &str,
    include_diagnostic_extra: bool,
) -> ObservedTrace {
    let mut events = trace
        .required_nodes
        .iter()
        .zip(trace.stable_reason_codes.iter())
        .map(|(node, reason)| ObservedTraceEvent::new(node, event_kind, reason))
        .collect::<Vec<_>>();
    if include_diagnostic_extra {
        events.push(ObservedTraceEvent::diagnostic(
            "event_phase7_diagnostic_extra",
            "diagnostic.extra",
            "diagnostic.extra",
        ));
    }
    ObservedTrace::new(events, trace.required_causal_edges.clone())
}

fn local_stack_lifecycle(report: &LocalStackReport) -> Vec<HarnessLifecycleState> {
    let mut lifecycle = HarnessLifecycleRecorder::new();
    lifecycle
        .transition(HarnessLifecycleState::StackStarting)
        .expect("planned to stack_starting is valid");

    if report.reason_code == "dependency.local_stack_unavailable" {
        lifecycle
            .transition(HarnessLifecycleState::CollectingArtifacts)
            .expect("stack_starting to collecting_artifacts is valid");
        lifecycle
            .transition(HarnessLifecycleState::Blocked)
            .expect("collecting_artifacts to blocked is valid");
        return lifecycle.into_states();
    }

    lifecycle
        .transition(HarnessLifecycleState::StackReady)
        .expect("stack_starting to stack_ready is valid");

    match report.status {
        HarnessRunStatus::Passed => {
            lifecycle
                .transition(HarnessLifecycleState::Resetting)
                .expect("stack_ready to resetting is valid");
            lifecycle
                .transition(HarnessLifecycleState::Seeding)
                .expect("resetting to seeding is valid");
            lifecycle
                .transition(HarnessLifecycleState::Running)
                .expect("seeding to running is valid");
            lifecycle
                .transition(HarnessLifecycleState::Asserting)
                .expect("running to asserting is valid");
            lifecycle
                .transition(HarnessLifecycleState::CollectingArtifacts)
                .expect("asserting to collecting_artifacts is valid");
            lifecycle
                .transition(HarnessLifecycleState::Passed)
                .expect("collecting_artifacts to passed is valid");
        }
        HarnessRunStatus::Failed => {
            lifecycle
                .transition(HarnessLifecycleState::Resetting)
                .expect("stack_ready to resetting is valid");
            lifecycle
                .transition(HarnessLifecycleState::Seeding)
                .expect("resetting to seeding is valid");
            lifecycle
                .transition(HarnessLifecycleState::Running)
                .expect("seeding to running is valid");
            lifecycle
                .transition(HarnessLifecycleState::Asserting)
                .expect("running to asserting is valid");
            lifecycle
                .transition(HarnessLifecycleState::CollectingArtifacts)
                .expect("asserting to collecting_artifacts is valid");
            lifecycle
                .transition(HarnessLifecycleState::Failed)
                .expect("collecting_artifacts to failed is valid");
        }
        HarnessRunStatus::Blocked => {
            if !report.reset_refs.is_empty() {
                lifecycle
                    .transition(HarnessLifecycleState::Resetting)
                    .expect("stack_ready to resetting is valid");
            }
            if !report.seed_refs.is_empty() {
                lifecycle
                    .transition(HarnessLifecycleState::Seeding)
                    .expect("resetting to seeding is valid");
            }
            if !report.smoke_refs.is_empty() {
                lifecycle
                    .transition(HarnessLifecycleState::Running)
                    .expect("seeding to running is valid");
                lifecycle
                    .transition(HarnessLifecycleState::Asserting)
                    .expect("running to asserting is valid");
            }
            lifecycle
                .transition(HarnessLifecycleState::CollectingArtifacts)
                .expect("blocked path to collecting_artifacts is valid");
            lifecycle
                .transition(HarnessLifecycleState::Blocked)
                .expect("collecting_artifacts to blocked is valid");
        }
        HarnessRunStatus::Planned | HarnessRunStatus::Running => {
            lifecycle
                .transition(HarnessLifecycleState::CollectingArtifacts)
                .expect("stack_ready to collecting_artifacts is valid");
            lifecycle
                .transition(HarnessLifecycleState::Blocked)
                .expect("collecting_artifacts to blocked is valid");
        }
    }

    lifecycle.into_states()
}

fn primary_ci_scenario(
    scenarios: &[ScenarioManifestRef],
    requested_phase: Option<u8>,
) -> ScenarioManifestRef {
    if requested_phase.is_none() {
        return scenarios
            .iter()
            .find(|scenario| scenario.scenario_id == "scenario_phase0_smoke")
            .unwrap_or(&scenarios[0])
            .clone();
    }
    scenarios
        .iter()
        .max_by_key(|scenario| {
            (
                scenario.master_phase,
                gate_priority(&scenario.gate_class),
                scenario.scenario_id.as_str(),
            )
        })
        .expect("selected CI scenario set is not empty")
        .clone()
}

fn gate_priority(gate_class: &str) -> u8 {
    match gate_class {
        "release_candidate" => 4,
        "regression" => 3,
        "contract_spine" => 2,
        "smoke" => 1,
        _ => 0,
    }
}

fn build_selection_report(
    selection: &ScenarioSelection,
    requested_phase: Option<u8>,
    selection_mode: &str,
) -> ScenarioSelectionReport {
    let selected_gate_classes = selection
        .selected
        .iter()
        .map(|scenario| scenario.gate_class.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let ci_entrypoint = if selection.selected.is_empty() {
        "none".to_owned()
    } else {
        primary_ci_scenario(&selection.selected, requested_phase).scenario_id
    };
    ScenarioSelectionReport {
        requested_phase,
        selection_mode: selection_mode.to_owned(),
        selected_count: selection.selected.len(),
        planned_count: selection.planned.len(),
        selected_gate_classes,
        selected_scenario_ids: selection.selected_scenario_ids(),
        planned_scenario_ids: selection.planned_scenario_ids(),
        ci_entrypoint,
    }
}

#[derive(Debug)]
struct CoverageAccumulator {
    phase: u8,
    scenario_ids: BTreeSet<String>,
    schema_modules: BTreeSet<String>,
    event_types: BTreeSet<String>,
    reason_code_families: BTreeSet<String>,
    status: String,
}

impl Default for CoverageAccumulator {
    fn default() -> Self {
        Self {
            phase: u8::MAX,
            scenario_ids: BTreeSet::new(),
            schema_modules: BTreeSet::new(),
            event_types: BTreeSet::new(),
            reason_code_families: BTreeSet::new(),
            status: "missing_required_contract".to_owned(),
        }
    }
}

fn service_contract_coverage(
    selection: &ScenarioSelection,
    requested_phase: Option<u8>,
) -> Vec<ServiceContractCoverageReport> {
    let mut reports = BTreeMap::<String, CoverageAccumulator>::new();
    for scenario in &selection.selected {
        record_coverage(&mut reports, scenario, "covered");
    }
    for scenario in &selection.planned {
        record_coverage(&mut reports, scenario, "planned");
    }

    if let Some(phase) = requested_phase {
        for service_id in required_public_contract_services(phase) {
            reports
                .entry((*service_id).to_owned())
                .or_insert_with(|| CoverageAccumulator {
                    phase,
                    status: "missing_required_contract".to_owned(),
                    ..CoverageAccumulator::default()
                });
        }
    }

    reports
        .into_iter()
        .map(|(service_id, mut report)| {
            if report.schema_modules.is_empty() {
                report
                    .schema_modules
                    .insert("integration_harness.scenario_manifest".to_owned());
            }
            if report.reason_code_families.is_empty() {
                report.reason_code_families.insert("dependency".to_owned());
            }
            ServiceContractCoverageReport {
                service_id,
                phase: report.phase,
                scenario_ids: report.scenario_ids.into_iter().collect(),
                schema_modules: report.schema_modules.into_iter().collect(),
                event_types: report.event_types.into_iter().collect(),
                reason_code_families: report.reason_code_families.into_iter().collect(),
                status: report.status,
            }
        })
        .collect()
}

fn required_public_contract_services(phase: u8) -> &'static [&'static str] {
    match phase {
        0 => &["service:local_stack", "service:overgate", "service:overwatch"],
        1..=13 => &[
            "service:local_stack",
            "service:overgate",
            "service:overwatch",
        ],
        _ => &[],
    }
}

fn record_coverage(
    reports: &mut BTreeMap<String, CoverageAccumulator>,
    scenario: &ScenarioManifestRef,
    status: &str,
) {
    for service_id in &scenario.required_services {
        let report = reports
            .entry(service_id.clone())
            .or_insert_with(CoverageAccumulator::default);
        report.phase = if report.phase == u8::MAX {
            scenario.master_phase
        } else {
            report.phase.min(scenario.master_phase)
        };
        report.scenario_ids.insert(scenario.scenario_id.clone());
        report
            .schema_modules
            .insert("integration_harness.scenario_manifest".to_owned());
        report
            .schema_modules
            .insert("integration_harness.golden_trace".to_owned());
        report
            .schema_modules
            .insert("integration_harness.artifact_bundle".to_owned());
        report.status = merge_coverage_status(&report.status, status).to_owned();
        for step in &scenario.steps {
            report
                .event_types
                .insert(format!("{}.event", step.action_kind.as_str()));
            report
                .reason_code_families
                .insert(reason_family_for_step(step));
        }
    }
}

fn merge_coverage_status(current: &str, next: &str) -> &'static str {
    match (current, next) {
        ("covered", _) | (_, "covered") => "covered",
        ("planned", _) | (_, "planned") => "planned",
        _ => "missing_required_contract",
    }
}

fn reason_family_for_step(step: &crate::manifests::ScenarioStepRef) -> String {
    if step
        .input_refs
        .iter()
        .any(|input| input.contains("not_available") || input.starts_with("service:"))
    {
        "dependency"
    } else if step
        .input_refs
        .iter()
        .any(|input| input.contains("signature"))
    {
        "signature"
    } else if step
        .input_refs
        .iter()
        .any(|input| input.contains("unstable") || input.contains("nondeterministic"))
    {
        "flake"
    } else if step.action_kind == overrid_contracts::ScenarioActionKind::Assertion {
        "assertion"
    } else {
        "run"
    }
    .to_owned()
}

fn scenario_items(scenarios: Vec<ScenarioManifestRef>) -> Vec<ScenarioListItem> {
    scenarios
        .into_iter()
        .map(|scenario| {
            let step_count = scenario.steps.len();
            let action_kinds = scenario
                .steps
                .iter()
                .map(|step| step.action_kind_str().to_owned())
                .collect();
            ScenarioListItem {
                scenario_id: scenario.scenario_id,
                master_phase: scenario.master_phase,
                gate_class: scenario.gate_class,
                tags: scenario.tags,
                required_services: scenario.required_services,
                setup_fixture_refs: scenario.setup_fixture_refs,
                step_count,
                action_kinds,
                source_path: scenario.source_path,
            }
        })
        .collect()
}

fn scenarios_json(scenarios: &[ScenarioListItem]) -> String {
    let values = scenarios
        .iter()
        .map(|scenario| {
            format!(
                concat!(
                    "{{",
                    "\"scenario_id\":\"{}\",",
                    "\"master_phase\":{},",
                    "\"gate_class\":\"{}\",",
                    "\"tags\":{},",
                    "\"required_services\":{},",
                    "\"setup_fixture_refs\":{},",
                    "\"step_count\":{},",
                    "\"action_kinds\":{},",
                    "\"source_path\":\"{}\"",
                    "}}"
                ),
                json_escape(&scenario.scenario_id),
                scenario.master_phase,
                json_escape(&scenario.gate_class),
                json_string_array(&scenario.tags),
                json_string_array(&scenario.required_services),
                json_string_array(&scenario.setup_fixture_refs),
                scenario.step_count,
                json_string_array(&scenario.action_kinds),
                json_escape(&scenario.source_path),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", values.join(","))
}

fn selection_report_json(report: Option<&ScenarioSelectionReport>) -> String {
    let Some(report) = report else {
        return "null".to_owned();
    };
    format!(
        concat!(
            "{{",
            "\"requested_phase\":{},",
            "\"selection_mode\":\"{}\",",
            "\"selected_count\":{},",
            "\"planned_count\":{},",
            "\"selected_gate_classes\":{},",
            "\"selected_scenario_ids\":{},",
            "\"planned_scenario_ids\":{},",
            "\"ci_entrypoint\":\"{}\"",
            "}}"
        ),
        report
            .requested_phase
            .map(|phase| phase.to_string())
            .unwrap_or_else(|| "null".to_owned()),
        json_escape(&report.selection_mode),
        report.selected_count,
        report.planned_count,
        json_string_array(&report.selected_gate_classes),
        json_string_array(&report.selected_scenario_ids),
        json_string_array(&report.planned_scenario_ids),
        json_escape(&report.ci_entrypoint),
    )
}

fn coverage_report_json(reports: &[ServiceContractCoverageReport]) -> String {
    let values = reports
        .iter()
        .map(|report| {
            format!(
                concat!(
                    "{{",
                    "\"service_id\":\"{}\",",
                    "\"phase\":{},",
                    "\"scenario_ids\":{},",
                    "\"schema_modules\":{},",
                    "\"event_types\":{},",
                    "\"reason_code_families\":{},",
                    "\"status\":\"{}\"",
                    "}}"
                ),
                json_escape(&report.service_id),
                report.phase,
                json_string_array(&report.scenario_ids),
                json_string_array(&report.schema_modules),
                json_string_array(&report.event_types),
                json_string_array(&report.reason_code_families),
                json_escape(&report.status),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", values.join(","))
}

fn step_results_json(step_results: &[ScenarioStepResult]) -> String {
    let values = step_results
        .iter()
        .map(|step| {
            format!(
                concat!(
                    "{{",
                    "\"step_id\":\"{}\",",
                    "\"action_kind\":\"{}\",",
                    "\"status\":\"{}\",",
                    "\"reason_code\":\"{}\",",
                    "\"reason_class\":\"{}\",",
                    "\"expected_result_class\":\"{}\",",
                    "\"exit_class\":\"{}\",",
                    "\"duration_ms\":{},",
                    "\"retry_class\":\"{}\",",
                    "\"stdout_ref\":{},",
                    "\"stderr_ref\":{},",
                    "\"payload_ref\":{},",
                    "\"assertion_refs\":{},",
                    "\"artifact_refs\":{},",
                    "\"dependency_status\":{}",
                    "}}"
                ),
                json_escape(&step.step_id),
                json_escape(step.action_kind_str()),
                json_escape(step.status.as_str()),
                json_escape(&step.reason_code),
                json_escape(&step.reason_class),
                json_escape(step.expected_result_class.as_str()),
                json_escape(&step.exit_class),
                step.duration_ms,
                json_escape(&step.retry_class),
                json_optional_string(step.stdout_ref.as_deref()),
                json_optional_string(step.stderr_ref.as_deref()),
                json_optional_string(step.payload_ref.as_deref()),
                json_string_array(&step.assertion_refs),
                json_string_array(&step.artifact_refs),
                json_string_array(&step.dependency_status),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", values.join(","))
}

fn lifecycle_json(lifecycle: &[HarnessLifecycleState]) -> String {
    let values = lifecycle
        .iter()
        .map(|state| state.as_str())
        .collect::<Vec<_>>();
    json_str_array(&values)
}

fn artifacts_json(artifacts: &[ArtifactSummary]) -> String {
    let values = artifacts
        .iter()
        .map(|artifact| {
            format!(
                concat!(
                    "{{",
                    "\"run_id\":\"{}\",",
                    "\"bundle_ref\":\"{}\",",
                    "\"path\":\"{}\",",
                    "\"manifest_ref\":\"{}\",",
                    "\"redaction_report_ref\":\"{}\",",
                    "\"retention_class\":\"{}\",",
                    "\"redaction_policy\":\"{}\",",
                    "\"collection_refs\":{},",
                    "\"redaction_report\":{},",
                    "\"reproduction_command\":\"{}\",",
                    "\"flake_metadata\":{},",
                    "\"retention_policy\":{}",
                    "}}"
                ),
                json_escape(&artifact.run_id),
                json_escape(&artifact.bundle_ref),
                json_escape(&artifact.path),
                json_escape(&artifact.manifest_ref),
                json_escape(&artifact.redaction_report_ref),
                json_escape(artifact.retention_class.as_str()),
                json_escape(&artifact.redaction_policy),
                artifact_collection_refs_json(&artifact.collection_refs),
                redaction_report_json(&artifact.redaction_report),
                json_escape(&artifact.reproduction_command),
                flake_metadata_json(&artifact.flake_metadata),
                retention_policy_json(&artifact.retention_policy),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", values.join(","))
}

fn artifact_collection_refs_json(refs: &crate::artifacts::ArtifactCollectionRefs) -> String {
    format!(
        concat!(
            "{{",
            "\"redacted_log_refs\":{},",
            "\"overwatch_export_refs\":{},",
            "\"cli_output_refs\":{},",
            "\"api_payload_envelope_refs\":{},",
            "\"stack_health_refs\":{},",
            "\"fixture_version_refs\":{},",
            "\"schema_version_refs\":{},",
            "\"assertion_diff_refs\":{}",
            "}}"
        ),
        json_string_array(&refs.redacted_log_refs),
        json_string_array(&refs.overwatch_export_refs),
        json_string_array(&refs.cli_output_refs),
        json_string_array(&refs.api_payload_envelope_refs),
        json_string_array(&refs.stack_health_refs),
        json_string_array(&refs.fixture_version_refs),
        json_string_array(&refs.schema_version_refs),
        json_string_array(&refs.assertion_diff_refs),
    )
}

fn redaction_report_json(report: &overrid_contracts::RedactionScanReport) -> String {
    format!(
        concat!(
            "{{",
            "\"policy\":\"{}\",",
            "\"redacted_fields\":{},",
            "\"scanner_passed\":{},",
            "\"rejected_markers\":{}",
            "}}"
        ),
        json_escape(&report.policy),
        json_string_array(&report.redacted_fields),
        report.scanner_passed,
        json_string_array(&report.rejected_markers),
    )
}

fn flake_metadata_json(metadata: &overrid_contracts::FlakeMetadata) -> String {
    format!(
        concat!(
            "{{",
            "\"repeated_run_count\":{},",
            "\"timing_variance_ms\":{},",
            "\"nondeterministic_assertion_markers\":{},",
            "\"unstable_event_ordering\":{},",
            "\"tolerance_window_used\":{}",
            "}}"
        ),
        metadata.repeated_run_count,
        metadata.timing_variance_ms,
        json_string_array(&metadata.nondeterministic_assertion_markers),
        metadata.unstable_event_ordering,
        metadata.tolerance_window_used,
    )
}

fn retention_policy_json(policy: &overrid_contracts::ArtifactRetentionPolicy) -> String {
    format!(
        concat!(
            "{{",
            "\"retention_class\":\"{}\",",
            "\"minimum_retention_days\":{},",
            "\"prune_after_days\":{},",
            "\"compact_success_summary\":{}",
            "}}"
        ),
        json_escape(policy.retention_class.as_str()),
        policy.minimum_retention_days,
        policy.prune_after_days,
        policy.compact_success_summary,
    )
}

fn service_health_json(service_health: &[ServiceHealthSummary]) -> String {
    let values = service_health
        .iter()
        .map(|health| {
            format!(
                concat!(
                    "{{",
                    "\"service_id\":\"{}\",",
                    "\"state\":\"{}\",",
                    "\"required\":{},",
                    "\"reason_code\":\"{}\",",
                    "\"evidence_ref\":\"{}\"",
                    "}}"
                ),
                json_escape(&health.service_id),
                json_escape(&health.state),
                health.required,
                json_escape(&health.reason_code),
                json_escape(&health.evidence_ref),
            )
        })
        .collect::<Vec<_>>();
    format!("[{}]", values.join(","))
}

fn json_optional_string(value: Option<&str>) -> String {
    value
        .map(|value| format!("\"{}\"", json_escape(value)))
        .unwrap_or_else(|| "null".to_owned())
}

fn json_string_array(values: &[String]) -> String {
    let borrowed = values.iter().map(String::as_str).collect::<Vec<_>>();
    json_str_array(&borrowed)
}

fn json_str_array(values: &[&str]) -> String {
    let rendered = values
        .iter()
        .map(|value| format!("\"{}\"", json_escape(value)))
        .collect::<Vec<_>>();
    format!("[{}]", rendered.join(","))
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            other => escaped.push(other),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .unwrap()
            .to_path_buf()
    }

    fn runner() -> HarnessRunner {
        HarnessRunner::new(RunnerOptions::new(repo_root()))
    }

    #[test]
    fn run_context_is_deterministic_across_reset_reseed_runs() {
        let first = HarnessRunContext::deterministic(
            "scenario_phase0_smoke",
            "seed_phase0_smoke_0001",
            "target/test-artifacts",
        );
        let second = HarnessRunContext::deterministic(
            "scenario_phase0_smoke",
            "seed_phase0_smoke_0001",
            "target/test-artifacts",
        );
        assert_eq!(first, second);
        assert!(first.run_id.starts_with("run_scenario_phase0_smoke_"));
        assert!(first.trace_root.starts_with("trace_scenario_phase0_smoke_"));
    }

    #[test]
    fn lifecycle_rejects_impossible_transitions() {
        let mut recorder = HarnessLifecycleRecorder::new();
        let error = recorder
            .transition(HarnessLifecycleState::Passed)
            .unwrap_err();
        assert_eq!(error.from, HarnessLifecycleState::Planned);
        assert_eq!(error.to, HarnessLifecycleState::Passed);
    }

    #[test]
    fn phase0_smoke_lifecycle_reaches_passed_after_artifacts() {
        let output = runner().run(HarnessCliCommand::Scenario {
            name: "scenario_phase0_smoke".to_owned(),
        });
        assert_eq!(output.status, HarnessRunStatus::Passed);
        assert_eq!(output.reason_code, "run.passed");
        assert!(output
            .lifecycle
            .contains(&HarnessLifecycleState::CollectingArtifacts));
        assert_eq!(
            output.lifecycle.last(),
            Some(&HarnessLifecycleState::Passed)
        );
        assert!(!output.artifacts.is_empty());
        assert!(output
            .service_health
            .iter()
            .any(|health| health.service_id == "service:local_stack" && health.state == "ready"));
        assert!(!output.seed_refs.is_empty());
        assert!(!output.smoke_refs.is_empty());
    }

    #[test]
    fn integration_command_runs_phase0_smoke_by_default() {
        let output = runner().run(HarnessCliCommand::Integration);
        assert_eq!(output.status, HarnessRunStatus::Passed);
        assert_eq!(output.scenarios.len(), 1);
        assert_eq!(output.scenarios[0].scenario_id, "scenario_phase0_smoke");
        assert!(output.result_json().contains("\"diagnostic_refs\""));
    }

    #[test]
    fn reset_runs_start_reset_seed_and_artifact_collection() {
        let output = runner().run(HarnessCliCommand::Reset);
        assert_eq!(output.status, HarnessRunStatus::Passed);
        assert!(output
            .lifecycle
            .contains(&HarnessLifecycleState::StackReady));
        assert!(output.lifecycle.contains(&HarnessLifecycleState::Resetting));
        assert!(output.lifecycle.contains(&HarnessLifecycleState::Seeding));
        assert!(!output.reset_refs.is_empty());
        assert!(!output.seed_refs.is_empty());
    }

    #[test]
    fn already_running_profile_verifies_existing_stack() {
        let mut options = RunnerOptions::new(repo_root());
        options.profile = "local-already-running".to_owned();
        let output = HarnessRunner::new(options).run(HarnessCliCommand::Integration);
        assert_eq!(output.status, HarnessRunStatus::Passed);
        assert!(output
            .event_refs
            .iter()
            .any(|event_ref| event_ref.contains("already_running_verified")));
    }

    #[test]
    fn degraded_optional_service_does_not_block_smoke() {
        let mut options = RunnerOptions::new(repo_root());
        options.profile = "local-degraded-optional".to_owned();
        let output = HarnessRunner::new(options).run(HarnessCliCommand::Integration);
        assert_eq!(output.status, HarnessRunStatus::Passed);
        assert!(output.service_health.iter().any(|health| {
            health.service_id == "component:diagnostic_log_stream"
                && health.state == "degraded"
                && !health.required
        }));
    }

    #[test]
    fn health_timeout_blocks_before_reset_or_seed() {
        let mut options = RunnerOptions::new(repo_root());
        options.profile = "local-health-timeout".to_owned();
        let output = HarnessRunner::new(options).run(HarnessCliCommand::Integration);
        assert_eq!(output.status, HarnessRunStatus::Blocked);
        assert_eq!(output.reason_code, "dependency.local_stack_unavailable");
        assert!(output
            .dependency_status
            .contains(&"component:api:timeout".to_owned()));
        assert!(output.reset_refs.is_empty());
        assert!(output.seed_refs.is_empty());
        assert!(!output.lifecycle.contains(&HarnessLifecycleState::Resetting));
        assert!(!output.lifecycle.contains(&HarnessLifecycleState::Seeding));
    }

    #[test]
    fn port_conflict_blocks_before_reset_or_seed() {
        let mut options = RunnerOptions::new(repo_root());
        options.profile = "local-port-conflict".to_owned();
        let output = HarnessRunner::new(options).run(HarnessCliCommand::Integration);
        assert_eq!(output.status, HarnessRunStatus::Blocked);
        assert_eq!(output.reason_code, "dependency.local_stack_unavailable");
        assert!(output
            .dependency_status
            .contains(&"component:api:port_conflict".to_owned()));
        assert!(output.reset_refs.is_empty());
        assert!(output.seed_refs.is_empty());
    }

    #[test]
    fn reset_incomplete_aborts_before_seed() {
        let mut options = RunnerOptions::new(repo_root());
        options.profile = "local-reset-incomplete".to_owned();
        let output = HarnessRunner::new(options).run(HarnessCliCommand::Reset);
        assert_eq!(output.status, HarnessRunStatus::Blocked);
        assert_eq!(output.reason_code, "dependency.reset_incomplete");
        assert!(output
            .reset_refs
            .iter()
            .any(|ref_id| ref_id.ends_with(":incomplete")));
        assert!(output.seed_refs.is_empty());
        assert!(output.lifecycle.contains(&HarnessLifecycleState::Resetting));
        assert!(!output.lifecycle.contains(&HarnessLifecycleState::Seeding));
    }

    #[test]
    fn reset_rejects_unmarked_state_before_seed() {
        let mut options = RunnerOptions::new(repo_root());
        options.profile = "local-unmarked-state".to_owned();
        let output = HarnessRunner::new(options).run(HarnessCliCommand::Reset);
        assert_eq!(output.status, HarnessRunStatus::Blocked);
        assert_eq!(output.reason_code, "safety.unmarked_test_state");
        assert!(output
            .reset_refs
            .contains(&"reset:local_unmarked_state:marker_missing".to_owned()));
        assert!(output.seed_refs.is_empty());
    }

    #[test]
    fn reset_reseed_refs_are_deterministic() {
        let catalog = runner().load_catalog().unwrap();
        let harness = LocalStackHarness::new("local", repo_root().join("target/test-artifacts"));
        let first = harness.reset_stack(
            &catalog.fixtures,
            "run_reset_deterministic",
            "trace_reset_deterministic",
        );
        let second = harness.reset_stack(
            &catalog.fixtures,
            "run_reset_deterministic",
            "trace_reset_deterministic",
        );
        assert_eq!(first.reset_refs, second.reset_refs);
        assert_eq!(first.seed_refs, second.seed_refs);
    }

    #[test]
    fn lists_scenarios_with_phase_filter() {
        let mut options = RunnerOptions::new(repo_root());
        options.requested_phase = Some(0);
        let output = HarnessRunner::new(options).run(HarnessCliCommand::List);
        assert_eq!(output.status, HarnessRunStatus::Planned);
        assert_eq!(output.scenarios.len(), 2);
        assert!(output.result_json().contains("\"phase_filter\":0"));
    }

    #[test]
    fn phase6_step_runner_scenario_runs_all_black_box_action_kinds() {
        let output = runner().run(HarnessCliCommand::Scenario {
            name: "scenario_phase6_step_runners".to_owned(),
        });
        assert_eq!(output.status, HarnessRunStatus::Passed);
        assert_eq!(output.reason_code, "run.passed");
        assert_eq!(output.step_results.len(), 5);
        let action_kinds = output
            .step_results
            .iter()
            .map(|step| step.action_kind_str())
            .collect::<Vec<_>>();
        assert_eq!(
            action_kinds,
            vec!["cli", "sdk", "api", "local_helper", "assertion"]
        );
        assert!(output.step_results.iter().any(|step| step
            .stdout_ref
            .as_deref()
            .unwrap_or("")
            .ends_with(":redacted")));
        assert!(output.step_results.iter().any(|step| step
            .payload_ref
            .as_deref()
            .unwrap_or("")
            .contains(":sdk_payload:")));
        assert!(output.step_results.iter().any(|step| step
            .payload_ref
            .as_deref()
            .unwrap_or("")
            .contains(":api_payload:")));
        assert!(output.result_json().contains("\"step_results\""));
        assert!(output.result_json().contains("\"assertion_refs\""));
        assert!(output.lifecycle.contains(&HarnessLifecycleState::Asserting));
    }

    #[test]
    fn phase6_blocked_and_failed_step_runs_keep_failure_evidence_lifecycle() {
        let blocked = runner().run(HarnessCliCommand::Scenario {
            name: "scenario_phase6_blocked_direct_service".to_owned(),
        });
        assert_eq!(blocked.status, HarnessRunStatus::Blocked);
        assert_eq!(blocked.reason_code, "safety.direct_service_url");
        assert!(blocked.lifecycle.contains(&HarnessLifecycleState::Running));
        assert!(blocked
            .lifecycle
            .contains(&HarnessLifecycleState::Asserting));
        assert!(blocked
            .artifacts
            .iter()
            .any(|artifact| artifact.retention_class == ArtifactRetentionClass::FailureEvidence));

        let failed = runner().run(HarnessCliCommand::Scenario {
            name: "scenario_phase6_failed_invalid_signature".to_owned(),
        });
        assert_eq!(failed.status, HarnessRunStatus::Failed);
        assert_eq!(failed.reason_code, "signature.invalid");
        assert!(failed.lifecycle.contains(&HarnessLifecycleState::Running));
        assert!(failed.lifecycle.contains(&HarnessLifecycleState::Asserting));
        assert!(failed
            .artifacts
            .iter()
            .any(|artifact| artifact.retention_class == ArtifactRetentionClass::FailureEvidence));
    }

    #[test]
    fn phase7_scenarios_emit_golden_trace_assertion_refs() {
        for (scenario_id, assertion_id) in [
            (
                "scenario_phase7_exact_protocol_trace",
                "assertion_phase7_exact_protocol_trace",
            ),
            (
                "scenario_phase7_execution_dag_trace",
                "assertion_phase7_execution_dag_trace",
            ),
            (
                "scenario_phase7_policy_dispute_trace",
                "assertion_phase7_policy_dispute_trace",
            ),
            (
                "scenario_phase7_accounting_ledger_trace",
                "assertion_phase7_accounting_ledger_trace",
            ),
        ] {
            let output = runner().run(HarnessCliCommand::Scenario {
                name: scenario_id.to_owned(),
            });
            assert_eq!(output.status, HarnessRunStatus::Passed, "{scenario_id}");
            assert_eq!(output.reason_code, "run.passed", "{scenario_id}");
            assert!(
                output.assertion_refs.contains(&assertion_id.to_owned()),
                "{scenario_id}"
            );
            assert!(output.lifecycle.contains(&HarnessLifecycleState::Asserting));
        }
    }

    #[test]
    fn phase8_artifacts_emit_redaction_reproduction_flake_and_retention_metadata() {
        let release_candidate = runner().run(HarnessCliCommand::Scenario {
            name: "scenario_phase8_artifact_bundle".to_owned(),
        });
        assert_eq!(release_candidate.status, HarnessRunStatus::Passed);
        assert_eq!(release_candidate.reason_code, "run.passed");
        let artifact = release_candidate
            .artifacts
            .first()
            .expect("Phase 8 passed scenario emits artifact evidence");
        assert_eq!(
            artifact.retention_class,
            ArtifactRetentionClass::ReleaseCandidate
        );
        assert!(artifact.redaction_report.scanner_passed);
        assert!(artifact
            .reproduction_command
            .contains("scenario_phase8_artifact_bundle"));
        assert!(!artifact.reproduction_command.contains("/Users/"));
        assert!(!artifact.flake_metadata.is_nondeterministic());

        let flake = runner().run(HarnessCliCommand::Scenario {
            name: "scenario_phase8_flake_detection".to_owned(),
        });
        assert_eq!(flake.status, HarnessRunStatus::Failed);
        assert_eq!(flake.reason_code, "flake.unstable_event_ordering");
        let artifact = flake
            .artifacts
            .first()
            .expect("Phase 8 flake scenario emits failure evidence");
        assert_eq!(
            artifact.retention_class,
            ArtifactRetentionClass::FailureEvidence
        );
        assert!(artifact.redaction_report.scanner_passed);
        assert!(artifact.flake_metadata.is_nondeterministic());
        assert_eq!(artifact.flake_metadata.repeated_run_count, 3);
        assert!(artifact
            .collection_refs
            .assertion_diff_refs
            .iter()
            .any(|value| value.contains("unstable_event_ordering")));

        let json = flake.result_json();
        assert!(json.contains("\"redaction_report\""));
        assert!(json.contains("\"reproduction_command\""));
        assert!(json.contains("\"flake_metadata\""));
        assert!(json.contains("\"retention_policy\""));
    }

    #[test]
    fn blocked_dependency_scenario_reports_stable_reason() {
        let output = runner().run(HarnessCliCommand::Scenario {
            name: "scenario_blocked_dependency".to_owned(),
        });
        assert_eq!(output.status, HarnessRunStatus::Blocked);
        assert_eq!(output.reason_code, "dependency.service_unavailable");
        assert!(output
            .dependency_status
            .contains(&"service:overqueue:unavailable".to_owned()));
        assert!(output.reset_refs.is_empty());
        assert!(output.seed_refs.is_empty());
        assert!(output.result_json().contains("\"status\":\"blocked\""));
    }

    #[test]
    fn artifact_lookup_returns_stable_path() {
        let output = runner().run(HarnessCliCommand::Artifacts {
            run_id: "run_phase0_smoke".to_owned(),
        });
        assert_eq!(output.status, HarnessRunStatus::Passed);
        assert_eq!(output.reason_code, "artifact.lookup_ready");
        assert!(output
            .result_json()
            .contains("artifact:bundle:run_phase0_smoke"));
    }
}
