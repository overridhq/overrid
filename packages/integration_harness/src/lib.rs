#![forbid(unsafe_code)]

pub mod artifacts;
pub mod assertions;
pub mod fixtures;
pub mod local_stack;
pub mod manifests;
pub mod phase_gate;
pub mod runner;
pub mod step_runners;

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
