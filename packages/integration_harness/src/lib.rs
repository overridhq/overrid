#![forbid(unsafe_code)]

pub mod artifacts;
pub mod assertions;
pub mod fixtures;
pub mod handoff;
pub mod local_stack;
pub mod manifests;
pub mod phase_gate;
pub mod runner;
pub mod step_runners;

pub use handoff::{
    phase10_harness_validation_report, DownstreamHandoffRule, HarnessPhase10Report,
    HarnessValidationItem, SecurityRedactionCheck, TechStackAlignmentCheck, PHASE10_SCHEMA_VERSION,
};
pub use local_stack::{
    LocalStackHarness, LocalStackReport, LocalStackSnapshot, ServiceHealthSummary,
    LOCAL_TEST_STATE_MARKER,
};
pub use runner::{
    HarnessCliCommand, HarnessCliOutput, HarnessLifecycleRecorder, HarnessLifecycleState,
    HarnessRunContext, HarnessRunner, RunnerOptions,
};
pub use step_runners::{
    ScenarioStepExecutionContext, ScenarioStepExecutionReport, ScenarioStepResult,
    ScenarioStepRunner,
};
