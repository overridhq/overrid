use std::fmt;
use std::path::{Path, PathBuf};

use overrid_contracts::{
    ArtifactRetentionClass, HarnessRunStatus, TestRunRecord,
    SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
};

use crate::artifacts::{ArtifactLocator, ArtifactSummary};
use crate::fixtures::{sanitize_identifier, stable_short_token, DEFAULT_FIXTURE_SEED};
use crate::manifests::{
    HarnessManifestCatalog, HarnessManifestLoader, ManifestLoadError, ScenarioManifestRef,
};

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
    pub source_path: String,
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
    pub lifecycle: Vec<HarnessLifecycleState>,
    pub artifacts: Vec<ArtifactSummary>,
    pub dependency_status: Vec<String>,
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
                "\"lifecycle\":{},",
                "\"artifacts\":{},",
                "\"dependency_status\":{},",
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
            lifecycle_json(&self.lifecycle),
            artifacts_json(&self.artifacts),
            json_string_array(&self.dependency_status),
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
        if !self.artifacts.is_empty() {
            lines.push("artifacts:".to_owned());
            for artifact in &self.artifacts {
                lines.push(format!("  {} {}", artifact.bundle_ref, artifact.path));
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
                let scenarios =
                    scenario_items(catalog.scenarios_for_phase(self.options.requested_phase));
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
                    lifecycle: vec![HarnessLifecycleState::Planned],
                    artifacts: Vec::new(),
                    dependency_status: Vec::new(),
                    message: "scenario manifest catalog loaded".to_owned(),
                }
            }
            Err(error) => self.manifest_blocked_output("test list", error),
        }
    }

    fn run_integration(&self) -> HarnessCliOutput {
        match self.load_catalog() {
            Ok(catalog) => {
                let scenarios = catalog.scenarios_for_phase(self.options.requested_phase);
                if scenarios.is_empty() {
                    return self.blocked_without_scenario(
                        "test integration",
                        "dependency.phase_tag_unsupported",
                        "dependency",
                        "no scenarios match the requested phase filter",
                    );
                }
                let context = self.context_for_scenario(&catalog, &scenarios[0]);
                self.blocked_run_output(
                    "test integration",
                    scenarios,
                    context,
                    "dependency.local_stack_unavailable",
                    "Phase 4 local stack hooks are not available in the Phase 3 runner shell",
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
                let reason_code = if scenario.scenario_id == "scenario_blocked_dependency" {
                    "dependency.service_unavailable"
                } else {
                    "dependency.local_stack_unavailable"
                };
                let message = if reason_code == "dependency.service_unavailable" {
                    "required service is unavailable; blocked result keeps artifact refs"
                } else {
                    "Phase 4 local stack hooks are not available in the Phase 3 runner shell"
                };
                let context = self.context_for_scenario(&catalog, scenario);
                self.blocked_run_output(
                    "test scenario",
                    vec![scenario.clone()],
                    context,
                    reason_code,
                    message,
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

        let context = HarnessRunContext::deterministic(
            "scenario_reset_shell",
            DEFAULT_FIXTURE_SEED,
            &self.options.artifact_root,
        );
        let mut lifecycle = HarnessLifecycleRecorder::new();
        lifecycle
            .transition(HarnessLifecycleState::Resetting)
            .expect("planned to resetting is valid");
        lifecycle
            .transition(HarnessLifecycleState::CollectingArtifacts)
            .expect("resetting to collecting_artifacts is valid");
        lifecycle
            .transition(HarnessLifecycleState::Blocked)
            .expect("collecting_artifacts to blocked is valid");
        let artifact = ArtifactLocator::new(&self.options.artifact_root)
            .lookup(&context.run_id, ArtifactRetentionClass::FailureEvidence);
        HarnessCliOutput {
            command_name: "test reset".to_owned(),
            status: HarnessRunStatus::Blocked,
            reason_code: "dependency.local_stack_unavailable".to_owned(),
            reason_class: "dependency".to_owned(),
            profile: self.options.profile.clone(),
            phase_filter: self.options.requested_phase,
            run_id: Some(context.run_id),
            trace_root: Some(self.options.trace_id.clone().unwrap_or(context.trace_root)),
            scenarios: Vec::new(),
            lifecycle: lifecycle.into_states(),
            artifacts: vec![artifact],
            dependency_status: vec!["local_stack:unavailable".to_owned()],
            message: "Phase 4 reset hook is not available in the Phase 3 shell".to_owned(),
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
            lifecycle: lifecycle.into_states(),
            artifacts: vec![artifact],
            dependency_status: Vec::new(),
            message: "artifact lookup returned deterministic bundle path".to_owned(),
        }
    }

    fn blocked_run_output(
        &self,
        command_name: &str,
        scenarios: Vec<ScenarioManifestRef>,
        context: HarnessRunContext,
        reason_code: &str,
        message: &str,
    ) -> HarnessCliOutput {
        if !self.local_profile_allowed() {
            return self.blocked_without_scenario(
                command_name,
                "safety.non_local_profile",
                "safety",
                "test runs require a local or explicit test harness profile",
            );
        }

        let mut lifecycle = HarnessLifecycleRecorder::new();
        lifecycle
            .transition(HarnessLifecycleState::StackStarting)
            .expect("planned to stack_starting is valid");
        lifecycle
            .transition(HarnessLifecycleState::CollectingArtifacts)
            .expect("stack_starting to collecting_artifacts is valid");
        lifecycle
            .transition(HarnessLifecycleState::Blocked)
            .expect("collecting_artifacts to blocked is valid");
        let artifact = ArtifactLocator::new(&self.options.artifact_root)
            .lookup(&context.run_id, ArtifactRetentionClass::FailureEvidence);
        let _ = context.test_run_record(HarnessRunStatus::Blocked);

        HarnessCliOutput {
            command_name: command_name.to_owned(),
            status: HarnessRunStatus::Blocked,
            reason_code: reason_code.to_owned(),
            reason_class: reason_code
                .split_once('.')
                .map(|(class, _)| class)
                .unwrap_or("dependency")
                .to_owned(),
            profile: self.options.profile.clone(),
            phase_filter: self.options.requested_phase,
            run_id: Some(context.run_id),
            trace_root: Some(self.options.trace_id.clone().unwrap_or(context.trace_root)),
            scenarios: scenario_items(scenarios),
            lifecycle: lifecycle.into_states(),
            artifacts: vec![artifact],
            dependency_status: vec!["local_stack:unavailable".to_owned()],
            message: message.to_owned(),
        }
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
            lifecycle: lifecycle.into_states(),
            artifacts: Vec::new(),
            dependency_status: Vec::new(),
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

    fn local_profile_allowed(&self) -> bool {
        self.options.test_harness_profile
            || matches!(
                self.options.profile.as_str(),
                "local" | "local-dev" | "test" | "ci"
            )
    }
}

fn scenario_items(scenarios: Vec<ScenarioManifestRef>) -> Vec<ScenarioListItem> {
    scenarios
        .into_iter()
        .map(|scenario| ScenarioListItem {
            scenario_id: scenario.scenario_id,
            master_phase: scenario.master_phase,
            gate_class: scenario.gate_class,
            tags: scenario.tags,
            required_services: scenario.required_services,
            setup_fixture_refs: scenario.setup_fixture_refs,
            source_path: scenario.source_path,
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
                    "\"source_path\":\"{}\"",
                    "}}"
                ),
                json_escape(&scenario.scenario_id),
                scenario.master_phase,
                json_escape(&scenario.gate_class),
                json_string_array(&scenario.tags),
                json_string_array(&scenario.required_services),
                json_string_array(&scenario.setup_fixture_refs),
                json_escape(&scenario.source_path),
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
                    "\"retention_class\":\"{}\",",
                    "\"redaction_policy\":\"{}\"",
                    "}}"
                ),
                json_escape(&artifact.run_id),
                json_escape(&artifact.bundle_ref),
                json_escape(&artifact.path),
                json_escape(artifact.retention_class.as_str()),
                json_escape(&artifact.redaction_policy),
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
    fn blocked_lifecycle_collects_artifacts_before_terminal_state() {
        let output = runner().run(HarnessCliCommand::Scenario {
            name: "scenario_phase0_smoke".to_owned(),
        });
        assert_eq!(output.status, HarnessRunStatus::Blocked);
        assert!(output
            .lifecycle
            .contains(&HarnessLifecycleState::CollectingArtifacts));
        assert!(!output.artifacts.is_empty());
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
    fn blocked_dependency_scenario_reports_stable_reason() {
        let output = runner().run(HarnessCliCommand::Scenario {
            name: "scenario_blocked_dependency".to_owned(),
        });
        assert_eq!(output.status, HarnessRunStatus::Blocked);
        assert_eq!(output.reason_code, "dependency.service_unavailable");
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
