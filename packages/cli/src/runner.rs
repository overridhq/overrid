use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use overrid_contracts::{
    BootstrapAcceptanceRecord, BootstrapCommandFamily, CanonicalIdempotencyFingerprint,
    CiAutomationProfile, CliPhaseAvailabilityRecord, CliProfile, CliReleaseReadinessReport,
    CliSecurityReviewReport, ConfirmationPolicy, CredentialReference, CredentialReferenceClass,
    DisputeReadModel, EnvironmentClass, ErrorDecodeRecord, ExecutionDiagnosticEvent,
    ExecutionLogBundle, ExecutionResultRef, ExecutionTimeline, ExitCodeClass, FixtureAllowance,
    LocalIdempotencyCacheRecord, ManifestBootstrapRef, NodeState, NodeStatusRecord,
    PackageValidationState, PackageValidationSummary, PolicyDryRunDecision, PollingPlan,
    ProductKind, ProductWorkflowRecipe, ProfileValidationError, ReceiptLedgerRead, RetryClass,
    RetryTimeoutPolicy, SignedCommandEnvelope, SyntheticWorkloadPendingState, UsageOruRollup,
    WorkloadExecutionState, SUPPORTED_SCHEMA_VERSION,
};
use overrid_integration_harness::{
    HarnessCliCommand, HarnessCliOutput, HarnessRunner, RunnerOptions,
};
use overrid_local_stack::{
    DevCommand as LocalStackDevCommand, LocalStackCommandOutput, LocalStackOptions,
    LocalStackRunner,
};
use overrid_sdk::{
    decode_phase6_error, enforce_profile_environment, retry_timeout_policy, CommandSafetyInput,
    SdkError,
};

use crate::build_metadata::{human_version_lines, version_info};
use crate::parser::{
    parse_cli, AuthCommand, Command, CredentialCommand, DevCommand, DisputeCommand, GlobalOptions,
    IdempotencyCacheCommand, IdentityCommand, KeyCommand, LedgerCommand, ManifestCommand,
    NodeCommand, OutputMode, PackageCommand, PlannedCommand, PolicyCommand, ProfileCommand,
    ReceiptCommand, RootCommand, TenantCommand, TestCommand, UsageCommand, WorkloadCommand,
};

const LOCAL_TRACE_ID: &str = "trace_cli_local";
const LOCAL_STACK_TRACE_ID: &str = "trace_cli_local_stack";
const TIMING_MS: u64 = 0;

pub const EXIT_SUCCESS: i32 = ExitCodeClass::Success.code();
pub const EXIT_USAGE: i32 = ExitCodeClass::Usage.code();
pub const EXIT_CONFIG: i32 = ExitCodeClass::Config.code();
pub const EXIT_CREDENTIAL: i32 = ExitCodeClass::Credential.code();
pub const EXIT_NOT_AVAILABLE_IN_PHASE: i32 = ExitCodeClass::Phase.code();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliRunResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Copy)]
struct RootCommandRecord {
    name: &'static str,
    purpose: &'static str,
    inputs: &'static [&'static str],
    outputs: &'static [&'static str],
    owning_tool: &'static str,
    phase_gate: &'static str,
    canonical_invocation: &'static str,
    result_envelope: bool,
    failure_classes: &'static [&'static str],
    aliases: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LayoutCheckRecord {
    check_name: &'static str,
    status: &'static str,
    reason_code: &'static str,
    path: String,
    owning_phase: &'static str,
    module_id: Option<&'static str>,
}

const ROOT_COMMAND_REGISTRY: &[RootCommandRecord] = &[
    RootCommandRecord {
        name: "build",
        purpose: "Compile and check implemented Rust workspace packages.",
        inputs: &["Cargo.toml", "overrid.workspace.toml"],
        outputs: &["build_check.passed", "build_check.failed"],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid build",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["cargo check --workspace"],
    },
    RootCommandRecord {
        name: "test",
        purpose: "Run unit and fast package validation through the Rust CLI command surface.",
        inputs: &["Cargo.toml", "overrid.workspace.toml"],
        outputs: &["test_check.passed", "test_check.failed"],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid test",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["cargo test --workspace"],
    },
    RootCommandRecord {
        name: "test:integration",
        purpose: "Run bounded local integration scenarios through the integration harness.",
        inputs: &["tests/integration", "infra/local", "overrid.workspace.toml"],
        outputs: &["integration_test.passed", "integration_test.failed"],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid test integration",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["overrid test:integration"],
    },
    RootCommandRecord {
        name: "dev:start",
        purpose: "Start the loopback-only Overrid local development stack.",
        inputs: &["infra/local/profiles", "infra/local/service-definitions"],
        outputs: &["local_stack.started", "local_stack.failed"],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid dev start",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["overrid dev:start"],
    },
    RootCommandRecord {
        name: "dev:stop",
        purpose: "Stop the loopback-only Overrid local development stack.",
        inputs: &["infra/local/profiles", "infra/local/service-definitions"],
        outputs: &["local_stack.stopped", "local_stack.failed"],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid dev stop",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["overrid dev:stop"],
    },
    RootCommandRecord {
        name: "dev:reset",
        purpose: "Reset ignored local state under marker-gated local development paths.",
        inputs: &["infra/local/state", "infra/local/job-tables", "infra/local/artifacts"],
        outputs: &["local_stack.reset", "local_stack.failed"],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid dev reset",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["overrid dev:reset"],
    },
    RootCommandRecord {
        name: "dev:seed",
        purpose: "Seed deterministic local development fixtures through Rust local-stack tooling.",
        inputs: &["infra/local/profiles", "tests/integration/scenarios"],
        outputs: &["local_stack.seeded", "local_stack.failed"],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid dev seed",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["overrid dev:seed"],
    },
    RootCommandRecord {
        name: "dev:status",
        purpose: "Report local development stack health and capability status.",
        inputs: &["infra/local/profiles", "infra/local/service-definitions"],
        outputs: &["local_stack.status", "local_stack.failed"],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid dev status",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["overrid dev:status"],
    },
    RootCommandRecord {
        name: "dev:smoke",
        purpose: "Run the loopback-only local stack smoke path used by clean-checkout CI.",
        inputs: &[
            "infra/local/profiles",
            "infra/local/service-definitions",
            "tests/integration/scenarios",
        ],
        outputs: &["local_stack.smoke.passed", "local_stack.smoke.failed"],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid dev smoke",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["overrid dev:smoke"],
    },
    RootCommandRecord {
        name: "schema:check",
        purpose: "Validate shared JSON Schema sources, fixtures, and generated projection metadata.",
        inputs: &["packages/schemas", "docs/specs/contract_authority.md"],
        outputs: &["schema_check.passed", "schema_check.failed"],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid schema:check",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["python3 scripts/validate_repository_layout_phase4.py"],
    },
    RootCommandRecord {
        name: "docs:check",
        purpose: "Validate docs links, headings, stale markers, restricted economics language, and SDS/build-plan alignment.",
        inputs: &["docs", "overrid.workspace.toml"],
        outputs: &["docs_check.passed", "docs_check.failed"],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid docs:check",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["python3 scripts/validate_overrid.py"],
    },
    RootCommandRecord {
        name: "layout:check",
        purpose: "Validate repository layout directories, manifest records, contracts, ignore markers, package boundaries, local state, and docs links.",
        inputs: &["overrid.workspace.toml", "docs/specs", "packages", "infra/local", "tests/integration"],
        outputs: &[
            "layout_check.passed",
            "layout_check.failed",
            "package_boundary_violation",
            "missing_service_contract",
            "missing_test_target",
            "generated_file_committed",
            "secret_file_committed",
            "schema_ref_missing",
            "premature_service_split",
            "split_review_missing",
            "local_test_boundary_violation",
            "local_state_committed",
            "docdex_index_hygiene_violation",
            "artifact_redaction_violation",
            "module_lifecycle_violation",
            "stale_layout_reference",
            "local_stack_discovery_violation",
            "harness_discovery_violation",
            "ci_command_sequence_violation",
            "validation_evidence_missing",
            "artifact_consumer_violation",
            "sub_build_plan_structure_violation",
            "tech_stack_alignment_violation",
            "master_plan_alignment_violation",
            "source_document_alignment_violation",
            "downstream_handoff_violation",
        ],
        owning_tool: "overrid-cli",
        phase_gate: "phase_0",
        canonical_invocation: "overrid layout:check",
        result_envelope: true,
        failure_classes: &["usage", "config", "platform"],
        aliases: &["overrid layout check"],
    },
];

const LAYOUT_VALIDATION_ARTIFACTS: &[&str] = &[
    "layout_check.passed",
    "layout_check.failed",
    "package_boundary_violation",
    "missing_service_contract",
    "missing_test_target",
    "generated_file_committed",
    "secret_file_committed",
    "schema_ref_missing",
    "premature_service_split",
    "split_review_missing",
    "local_test_boundary_violation",
    "local_state_committed",
    "docdex_index_hygiene_violation",
    "artifact_redaction_violation",
    "module_lifecycle_violation",
    "stale_layout_reference",
    "local_stack_discovery_violation",
    "harness_discovery_violation",
    "ci_command_sequence_violation",
    "validation_evidence_missing",
    "artifact_consumer_violation",
    "sub_build_plan_structure_violation",
    "tech_stack_alignment_violation",
    "master_plan_alignment_violation",
    "source_document_alignment_violation",
    "downstream_handoff_violation",
];

pub fn main_entry<I, S>(args: I) -> i32
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let result = run_args(args);
    if !result.stdout.is_empty() {
        println!("{}", result.stdout);
    }
    if !result.stderr.is_empty() {
        eprintln!("{}", result.stderr);
    }
    result.exit_code
}

pub fn run_args<I, S>(args: I) -> CliRunResult
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args = args.into_iter().map(Into::into).collect::<Vec<_>>();
    match parse_cli(args.clone()) {
        Ok(parsed) => match parsed.command {
            Command::Help => success(render_help(parsed.globals.all_phases)),
            Command::Version => success(render_version(&parsed.globals)),
            Command::Doctor => success(render_doctor(&parsed.globals)),
            Command::CommandRegistry => command_registry_result(&parsed.globals),
            Command::Root(command) => root_command_result(command, &parsed.globals),
            Command::Profile(command) => profile_command_result(command, &parsed.globals),
            Command::Credential(command) => credential_command_result(command, &parsed.globals),
            Command::IdempotencyCache(command) => {
                idempotency_cache_command_result(command, &parsed.globals)
            }
            Command::Auth(command) => auth_command_result(command, &parsed.globals),
            Command::Tenant(command) => tenant_command_result(command, &parsed.globals),
            Command::Identity(command) => identity_command_result(command, &parsed.globals),
            Command::Key(command) => key_command_result(command, &parsed.globals),
            Command::Manifest(command) => manifest_command_result(command, &parsed.globals),
            Command::Node(command) => node_command_result(command, &parsed.globals),
            Command::Workload(command) => workload_command_result(command, &parsed.globals),
            Command::Policy(command) => policy_command_result(command, &parsed.globals),
            Command::Package(command) => package_command_result(command, &parsed.globals),
            Command::Usage(command) => usage_command_result(command, &parsed.globals),
            Command::Receipt(command) => receipt_command_result(command, &parsed.globals),
            Command::Ledger(command) => ledger_command_result(command, &parsed.globals),
            Command::Dispute(command) => dispute_command_result(command, &parsed.globals),
            Command::Dev(command) => dev_command_result(command, &parsed.globals),
            Command::Test(command) => test_command_result(command, &parsed.globals),
            Command::Planned(command) => planned_command_result(command, &parsed.globals),
        },
        Err(error) => parse_error_result(&args, &error.to_string()),
    }
}

fn success(stdout: String) -> CliRunResult {
    CliRunResult {
        exit_code: EXIT_SUCCESS,
        stdout,
        stderr: String::new(),
    }
}

fn command_registry_result(globals: &GlobalOptions) -> CliRunResult {
    match globals.output {
        OutputMode::Human => success(render_command_registry_human()),
        OutputMode::Json => success(render_success_json(
            "command-registry",
            &render_command_registry_json(),
            &["parsed", "registry_loaded", "completed"],
            globals.profile.as_deref(),
            None,
            &[
                "root_command_registry_defined",
                "rust_owned_command_execution_defined",
            ],
            &[],
        )),
    }
}

fn root_command_result(command: RootCommand, globals: &GlobalOptions) -> CliRunResult {
    if command == RootCommand::LayoutCheck {
        return layout_check_result(globals);
    }

    let record = root_command_record(command.as_str()).expect("root command must be registered");
    match globals.output {
        OutputMode::Human => success(render_root_command_human(record)),
        OutputMode::Json => success(render_success_json(
            record.name,
            &render_root_command_execution_json(record),
            &["parsed", "orchestration_record_loaded", "completed"],
            globals.profile.as_deref(),
            None,
            &[
                "root_command_registry_defined",
                "rust_owned_command_execution_defined",
            ],
            &[],
        )),
    }
}

fn layout_check_result(globals: &GlobalOptions) -> CliRunResult {
    let repo_root =
        resolve_repo_root(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let records = collect_layout_check_records(&repo_root);
    let ok = records.iter().all(|record| record.status == "passed");
    let artifact_refs = layout_artifact_refs(ok, &records);
    let stdout = match globals.output {
        OutputMode::Human => render_layout_check_human(ok, &repo_root, &records, &artifact_refs),
        OutputMode::Json if ok => render_success_json_with_trace(
            "layout:check",
            &render_layout_check_result_json(ok, &repo_root, &records, &artifact_refs),
            &["parsed", "layout_scanned", "completed"],
            globals.profile.as_deref(),
            None,
            &["layout_check_defined", "validation_artifacts_defined"],
            &[],
            globals.trace_id.as_deref(),
            &artifact_refs,
        ),
        OutputMode::Json => {
            let error_json = render_layout_check_error_json(&records);
            render_envelope_json(
                "layout:check",
                false,
                &render_layout_check_result_json(ok, &repo_root, &records, &artifact_refs),
                &error_json,
                globals.trace_id.as_deref(),
                Some("layout_check.failed"),
                ExitCodeClass::Config,
                RetryClass::NotRetryable,
                &["parsed", "layout_scanned", "failed"],
                globals.profile.as_deref(),
                None,
                &["layout_check_defined", "validation_artifacts_defined"],
                &[],
                &artifact_refs,
            )
        }
    };

    CliRunResult {
        exit_code: if ok { EXIT_SUCCESS } else { EXIT_CONFIG },
        stdout,
        stderr: String::new(),
    }
}

fn profile_command_result(command: ProfileCommand, globals: &GlobalOptions) -> CliRunResult {
    match command {
        ProfileCommand::List => success(render_profile_list(globals)),
        ProfileCommand::Reset if !globals.confirm_profile => phase3_error_result(
            EXIT_CONFIG,
            globals.output,
            "missing_profile_confirmation",
            "profile reset requires --confirm-profile",
        ),
        ProfileCommand::Reset => success(render_profile_reset(globals)),
        ProfileCommand::Create | ProfileCommand::Select | ProfileCommand::Inspect => {
            let profile = match build_profile(globals) {
                Ok(profile) => profile,
                Err(error) => {
                    return phase3_error_result(
                        EXIT_CONFIG,
                        globals.output,
                        "profile_validation_failed",
                        &error.to_string(),
                    )
                }
            };
            let credential = match build_credential(globals) {
                Ok(credential) => credential,
                Err(error) => {
                    return phase3_error_result(
                        EXIT_CREDENTIAL,
                        globals.output,
                        "credential_validation_failed",
                        &error.to_string(),
                    )
                }
            };
            if let Err(error) = enforce_profile_environment(CommandSafetyInput {
                profile: &profile,
                credential: &credential,
                endpoint_override: globals.endpoint_override.as_deref(),
                explicit_profile: globals.profile.is_some(),
                confirm_profile: globals.confirm_profile,
                mutating: false,
                admin_impacting: false,
                reason: globals.reason.as_deref(),
                command_type: command.as_str(),
            }) {
                return sdk_error_result(globals.output, error);
            }

            success(render_profile_result(command, &profile, globals.output))
        }
    }
}

fn credential_command_result(command: CredentialCommand, globals: &GlobalOptions) -> CliRunResult {
    let profile = match build_profile(globals) {
        Ok(profile) => profile,
        Err(error) => {
            return phase3_error_result(
                EXIT_CONFIG,
                globals.output,
                "profile_validation_failed",
                &error.to_string(),
            )
        }
    };
    let credential = match build_credential(globals) {
        Ok(credential) => credential,
        Err(error) => {
            return phase3_error_result(
                EXIT_CREDENTIAL,
                globals.output,
                "credential_validation_failed",
                &error.to_string(),
            )
        }
    };

    let mutating = matches!(command, CredentialCommand::Enroll);
    let safety = match enforce_profile_environment(CommandSafetyInput {
        profile: &profile,
        credential: &credential,
        endpoint_override: globals.endpoint_override.as_deref(),
        explicit_profile: globals.profile.is_some(),
        confirm_profile: globals.confirm_profile,
        mutating,
        admin_impacting: false,
        reason: globals.reason.as_deref(),
        command_type: command.as_str(),
    }) {
        Ok(safety) => safety,
        Err(error) => return sdk_error_result(globals.output, error),
    };

    success(render_credential_result(
        command,
        &profile,
        &credential,
        safety
            .signer_handoff
            .as_ref()
            .map(|handoff| handoff.signature_ref.as_str()),
        globals.output,
    ))
}

fn idempotency_cache_command_result(
    command: IdempotencyCacheCommand,
    globals: &GlobalOptions,
) -> CliRunResult {
    let profile = match build_profile(globals) {
        Ok(profile) => profile,
        Err(error) => {
            return phase3_error_result(
                EXIT_CONFIG,
                globals.output,
                "profile_validation_failed",
                &error.to_string(),
            )
        }
    };
    let command_fingerprint = cache_fingerprint_for_profile(&profile);
    let idempotency_key = format!("idem_{command_fingerprint}");
    let cache_record = LocalIdempotencyCacheRecord::new(
        profile.name.clone(),
        profile.environment,
        command_fingerprint,
        idempotency_key,
    );

    success(render_idempotency_cache_result(
        command,
        &profile,
        &cache_record,
        globals.output,
    ))
}

fn test_command_result(command: TestCommand, globals: &GlobalOptions) -> CliRunResult {
    let repo_root =
        resolve_repo_root(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let mut options = RunnerOptions::new(repo_root);
    options.profile = globals
        .profile
        .clone()
        .unwrap_or_else(|| "local".to_owned());
    options.requested_phase = globals.phase;
    options.selection_service = globals.selection_service.clone();
    options.selection_tag = globals.selection_tag.clone();
    options.selection_changed_path = globals.selection_changed_path.clone();
    options.selection_required_dependency = globals.selection_required_dependency.clone();
    options.selection_gate_class = globals.selection_gate_class.clone();
    options.selection_scenario_name = globals.selection_scenario_name.clone();
    options.trace_id = globals.trace_id.clone();
    options.test_harness_profile = globals.test_harness_profile;

    let harness_command = match command {
        TestCommand::Integration => HarnessCliCommand::Integration,
        TestCommand::Scenario { name } => HarnessCliCommand::Scenario { name },
        TestCommand::List => HarnessCliCommand::List,
        TestCommand::Reset => HarnessCliCommand::Reset,
        TestCommand::Artifacts { run_id } => HarnessCliCommand::Artifacts { run_id },
    };
    let output = HarnessRunner::new(options).run(harness_command);
    let ok = output.is_ok();
    let exit_class = if ok {
        ExitCodeClass::Success
    } else {
        match output.status {
            overrid_contracts::HarnessRunStatus::Failed => ExitCodeClass::Platform,
            _ => ExitCodeClass::Config,
        }
    };
    let stdout = match globals.output {
        OutputMode::Human => output.human_summary(),
        OutputMode::Json => {
            let lifecycle = output.lifecycle_strs();
            let dependency_status = output.dependency_status_strs();
            let result_json = output.result_json();
            let error_json = if ok {
                "null".to_owned()
            } else {
                render_harness_error_json(&output, exit_class)
            };
            render_envelope_json(
                &output.command_name,
                ok,
                &result_json,
                &error_json,
                output.trace_root.as_deref().or(globals.trace_id.as_deref()),
                (!ok).then_some(output.reason_code.as_str()),
                exit_class,
                RetryClass::NotRetryable,
                &lifecycle,
                Some(output.profile.as_str()),
                None,
                &dependency_status,
                &[],
                &[],
            )
        }
    };

    CliRunResult {
        exit_code: exit_class.code(),
        stdout,
        stderr: String::new(),
    }
}

fn dev_command_result(command: DevCommand, globals: &GlobalOptions) -> CliRunResult {
    let repo_root =
        resolve_repo_root(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let mut options = LocalStackOptions::new(repo_root);
    options.profile = globals
        .profile
        .clone()
        .unwrap_or_else(|| "local".to_owned());
    options.master_phase = globals.phase.unwrap_or(0);
    options.trace_id = globals
        .trace_id
        .clone()
        .unwrap_or_else(|| LOCAL_STACK_TRACE_ID.to_owned());
    options.timeout_ms = globals.timeout_ms;
    options.poll_interval_ms = globals.poll_interval_ms;
    options.wait = globals.wait;
    options.follow = globals.follow;
    options.dry_run = globals.dry_run;

    let output = LocalStackRunner::new(options).run(to_local_stack_dev_command(command));
    let exit_class = output.exit_class;
    let stdout = match globals.output {
        OutputMode::Human => output.human_summary(),
        OutputMode::Json => render_local_stack_output_json(&output),
    };

    CliRunResult {
        exit_code: exit_class.code(),
        stdout,
        stderr: String::new(),
    }
}

fn to_local_stack_dev_command(command: DevCommand) -> LocalStackDevCommand {
    match command {
        DevCommand::Start => LocalStackDevCommand::Start,
        DevCommand::Stop => LocalStackDevCommand::Stop,
        DevCommand::Restart => LocalStackDevCommand::Restart,
        DevCommand::Status => LocalStackDevCommand::Status,
        DevCommand::Reset => LocalStackDevCommand::Reset,
        DevCommand::Seed => LocalStackDevCommand::Seed,
        DevCommand::Smoke => LocalStackDevCommand::Smoke,
        DevCommand::Logs => LocalStackDevCommand::Logs,
        DevCommand::Doctor => LocalStackDevCommand::Doctor,
        DevCommand::Prune => LocalStackDevCommand::Prune,
    }
}

fn resolve_repo_root(start: impl AsRef<Path>) -> PathBuf {
    let start = start.as_ref();
    let mut cursor = start;
    loop {
        if cursor.join("Cargo.toml").is_file()
            && cursor
                .join("packages/schemas/overrid_contracts/fixtures/valid")
                .is_dir()
        {
            return cursor.to_path_buf();
        }
        match cursor.parent() {
            Some(parent) => cursor = parent,
            None => return start.to_path_buf(),
        }
    }
}

fn root_command_record(name: &str) -> Option<&'static RootCommandRecord> {
    ROOT_COMMAND_REGISTRY
        .iter()
        .find(|record| record.name == name)
}

fn render_command_registry_human() -> String {
    let mut lines = vec![
        "root command registry".to_owned(),
        "owner: overrid-cli".to_owned(),
        "machine_readable_result_envelope: true".to_owned(),
        "alias_policy: thin_alias_only".to_owned(),
    ];
    for record in ROOT_COMMAND_REGISTRY {
        lines.push(format!(
            "- {} -> {} ({})",
            record.name, record.canonical_invocation, record.phase_gate
        ));
        lines.push(format!("  purpose: {}", record.purpose));
        lines.push(format!("  outputs: {}", record.outputs.join(", ")));
    }
    lines.join("\n")
}

fn render_root_command_human(record: &RootCommandRecord) -> String {
    [
        format!("root command: {}", record.name),
        format!("purpose: {}", record.purpose),
        format!("owner: {}", record.owning_tool),
        format!("canonical_invocation: {}", record.canonical_invocation),
        format!(
            "machine_readable_result_envelope: {}",
            record.result_envelope
        ),
        format!("failure_classes: {}", record.failure_classes.join(", ")),
        format!("outputs: {}", record.outputs.join(", ")),
    ]
    .join("\n")
}

fn render_command_registry_json() -> String {
    let commands = ROOT_COMMAND_REGISTRY
        .iter()
        .map(render_root_command_record_json)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "{{",
            "\"registry_version\":\"repository-layout-phase-5\",",
            "\"owner\":\"overrid-cli\",",
            "\"alias_policy\":\"thin_alias_only\",",
            "\"commands\":[{}]",
            "}}"
        ),
        commands
    )
}

fn render_root_command_execution_json(record: &RootCommandRecord) -> String {
    format!(
        concat!(
            "{{",
            "\"status\":\"registered\",",
            "\"phase_gate\":\"{}\",",
            "\"canonical_invocation\":\"{}\",",
            "\"owner\":\"{}\",",
            "\"machine_readable_result_envelope\":{},",
            "\"record\":{}",
            "}}"
        ),
        json_escape(record.phase_gate),
        json_escape(record.canonical_invocation),
        json_escape(record.owning_tool),
        record.result_envelope,
        render_root_command_record_json(record),
    )
}

fn render_root_command_record_json(record: &RootCommandRecord) -> String {
    format!(
        concat!(
            "{{",
            "\"name\":\"{}\",",
            "\"purpose\":\"{}\",",
            "\"inputs\":{},",
            "\"outputs\":{},",
            "\"owning_tool\":\"{}\",",
            "\"phase_gate\":\"{}\",",
            "\"canonical_invocation\":\"{}\",",
            "\"machine_readable_result_envelope\":{},",
            "\"failure_classes\":{},",
            "\"aliases\":{}",
            "}}"
        ),
        json_escape(record.name),
        json_escape(record.purpose),
        json_string_array(record.inputs),
        json_string_array(record.outputs),
        json_escape(record.owning_tool),
        json_escape(record.phase_gate),
        json_escape(record.canonical_invocation),
        record.result_envelope,
        json_string_array(record.failure_classes),
        json_string_array(record.aliases),
    )
}

fn collect_layout_check_records(repo_root: &Path) -> Vec<LayoutCheckRecord> {
    let mut records = Vec::new();
    for (path, module_id) in [
        ("services/control-plane", Some("control-plane-root")),
        ("services/node-agent", Some("node-agent-root")),
        ("packages/schemas", Some("shared-schemas")),
        ("packages/sdk", Some("sdk")),
        ("packages/cli", Some("cli")),
        ("infra/local", Some("local-infra")),
        ("tests/integration", Some("integration-tests")),
        ("docs/specs", Some("docs-specs")),
        ("infra/local/service-definitions", Some("local-infra")),
        ("infra/local/profiles", Some("local-infra")),
        ("tests/integration/scenarios", Some("integration-tests")),
    ] {
        push_path_presence(
            &mut records,
            repo_root,
            "required_directory",
            path,
            module_id,
            "required_directory_present",
            "missing_directory",
        );
    }

    for (path, module_id) in [
        ("overrid.workspace.toml", None),
        (
            "docs/specs/service_contract_template.md",
            Some("docs-specs"),
        ),
        ("docs/specs/new_module_checklist.md", Some("docs-specs")),
        ("docs/specs/reason_codes_and_events.md", Some("docs-specs")),
        ("docs/specs/contract_authority.md", Some("docs-specs")),
        ("packages/cli/README.md", Some("cli")),
    ] {
        push_path_presence(
            &mut records,
            repo_root,
            "required_contract_file",
            path,
            module_id,
            "required_contract_present",
            "missing_service_contract",
        );
    }
    for (path, fail_reason) in [
        (".gitignore", "generated_file_committed"),
        (".docdexignore", "docdex_index_hygiene_violation"),
    ] {
        push_path_presence(
            &mut records,
            repo_root,
            "required_hygiene_file",
            path,
            None,
            "required_hygiene_file_present",
            fail_reason,
        );
    }

    for path in [
        "infra/local/state/.gitignore",
        "infra/local/job-tables/.gitignore",
        "infra/local/artifacts/.gitignore",
        "tests/integration/artifacts/.gitignore",
        "docs/specs/generated/.gitignore",
    ] {
        push_path_presence(
            &mut records,
            repo_root,
            "generated_or_local_ignore_marker",
            path,
            None,
            "ignore_marker_present",
            "generated_file_committed",
        );
    }

    push_manifest_contains(
        &mut records,
        repo_root,
        "module_records_have_test_targets",
        "test_targets",
        "missing_test_target",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "package_boundaries_defined",
        "allowed_dependency_groups",
        "package_boundary_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "phase6_boundary_enforcement",
        "[package_boundary_enforcement]",
        "package_boundary_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "shared_schema_dependency_path",
        "shared_schema_dependency_source = \"packages/schemas\"",
        "schema_ref_missing",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "modular_control_plane_shape",
        "single_modular_rust_process_through_phase_3",
        "premature_service_split",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "split_review_criteria",
        "split_review_first_allowed_phase = 4",
        "split_review_missing",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "local_test_only_separation",
        "runtime_forbidden_dependency_groups",
        "local_test_boundary_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "phase7_artifact_hygiene",
        "[artifact_hygiene]",
        "docdex_index_hygiene_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "generated_output_ignore_rules",
        "generated_output_roots",
        "generated_file_committed",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "local_state_ignore_rules",
        "local_state_roots",
        "local_state_committed",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "secret_file_rules",
        "secret_file_deny_patterns",
        "secret_file_committed",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "docdex_indexing_hygiene",
        "docdex_index_include_roots",
        "docdex_index_hygiene_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "artifact_redaction_expectations",
        "redaction_classes",
        "artifact_redaction_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "phase8_module_lifecycle",
        "[module_lifecycle]",
        "module_lifecycle_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "service_contract_template_usage",
        "service_contract_template = \"docs/specs/service_contract_template.md\"",
        "missing_service_contract",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "new_module_checklist",
        "new_module_checklist = \"docs/specs/new_module_checklist.md\"",
        "missing_service_contract",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "module_addition_workflow",
        "module_addition_workflow_defined",
        "module_lifecycle_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "deprecation_removal_workflow",
        "deprecation_removal_workflow_defined",
        "module_lifecycle_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "cross_document_maintenance_rules",
        "cross_document_maintenance_rules_defined",
        "stale_layout_reference",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "module_lifecycle_acceptance_evidence",
        "required_acceptance_evidence",
        "missing_test_target",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "phase9_foundation_integration",
        "[foundation_integration]",
        "artifact_consumer_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "local_stack_discovery_metadata",
        "local_stack_discovery_fields",
        "local_stack_discovery_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "local_stack_service_definition_roots",
        "local_stack_service_definition_roots",
        "local_stack_discovery_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "local_stack_safe_reset_markers",
        "local_stack_safe_reset_markers",
        "local_stack_discovery_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "harness_discovery_metadata",
        "harness_discovery_fields",
        "harness_discovery_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "harness_scenario_roots",
        "harness_scenario_roots",
        "harness_discovery_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "harness_schema_refs",
        "harness_schema_refs",
        "harness_discovery_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "clean_checkout_ci_behavior",
        "clean_checkout_ci_commands",
        "ci_command_sequence_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "clean_checkout_ci_statuses",
        "clean_checkout_ci_statuses",
        "ci_command_sequence_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "validation_evidence_model",
        "validation_evidence_entries",
        "validation_evidence_missing",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "validation_artifact_consumers",
        "artifact_consumers",
        "artifact_consumer_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "artifact_consumer_boundary",
        "build_ci_evidence_not_overwatch_runtime_events",
        "artifact_consumer_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "phase10_alignment_handoff",
        "[alignment_handoff]",
        "downstream_handoff_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "sub_build_plan_structure_checks",
        "structure_checks",
        "sub_build_plan_structure_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "tech_stack_alignment_checks",
        "tech_stack_alignment_checks",
        "tech_stack_alignment_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "master_plan_alignment_checks",
        "master_plan_alignment_checks",
        "master_plan_alignment_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "source_alignment_documents",
        "source_alignment_documents",
        "source_document_alignment_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "phase10_planning_documents",
        "phase_planning_documents",
        "source_document_alignment_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "downstream_phase_handoff_rules",
        "downstream_phase_handoff_rules",
        "downstream_handoff_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "downstream_handoff_boundary",
        "existing_layout_with_sds_backed_expansion_no_top_level_sprawl",
        "downstream_handoff_violation",
    );
    push_manifest_contains(
        &mut records,
        repo_root,
        "phase10_validation_script",
        "scripts/validate_repository_layout_phase10.py",
        "source_document_alignment_violation",
    );
    push_lifecycle_state_validity(&mut records, repo_root);
    push_accepted_module_validation_evidence(&mut records, repo_root);
    for (group, path, module_id, expected) in [
        (
            "contracts",
            "packages/schemas/overrid_contracts/Cargo.toml",
            Some("shared-schemas"),
            &[][..],
        ),
        (
            "sdk",
            "packages/sdk/Cargo.toml",
            Some("sdk"),
            &["overrid-contracts"][..],
        ),
        (
            "local_stack",
            "packages/local_stack/Cargo.toml",
            Some("local-stack"),
            &["overrid-contracts"][..],
        ),
        (
            "integration_harness",
            "packages/integration_harness/Cargo.toml",
            Some("integration-harness"),
            &["overrid-contracts"][..],
        ),
        (
            "cli",
            "packages/cli/Cargo.toml",
            Some("cli"),
            &[
                "overrid-contracts",
                "overrid-integration-harness",
                "overrid-local-stack",
                "overrid-sdk",
            ][..],
        ),
    ] {
        push_internal_dependency_direction(
            &mut records,
            repo_root,
            group,
            path,
            module_id,
            expected,
        );
    }
    for (path, module_id) in [
        ("services/control-plane", Some("control-plane-root")),
        ("services/node-agent", Some("node-agent-root")),
    ] {
        push_service_manifest_absence(&mut records, repo_root, path, module_id);
    }
    push_manifest_contains(
        &mut records,
        repo_root,
        "root_commands_registered",
        "[[root_commands]]",
        "layout_check.failed",
    );
    push_generated_specs_clean(&mut records, repo_root);
    for (check_name, path, fail_reason, module_id) in [
        (
            "local_state_marker_clean",
            "infra/local/state",
            "local_state_committed",
            Some("local-infra"),
        ),
        (
            "local_job_table_marker_clean",
            "infra/local/job-tables",
            "local_state_committed",
            Some("local-infra"),
        ),
        (
            "local_artifact_marker_clean",
            "infra/local/artifacts",
            "generated_file_committed",
            Some("local-infra"),
        ),
        (
            "integration_artifact_marker_clean",
            "tests/integration/artifacts",
            "generated_file_committed",
            Some("integration-tests"),
        ),
    ] {
        push_marker_only_directory(
            &mut records,
            repo_root,
            check_name,
            path,
            fail_reason,
            module_id,
        );
    }
    for (ignore_file, marker, check_name, fail_reason) in [
        (
            ".gitignore",
            "target/",
            "gitignore_generated_output_rule",
            "generated_file_committed",
        ),
        (
            ".gitignore",
            "node_modules/",
            "gitignore_dependency_cache_rule",
            "generated_file_committed",
        ),
        (
            ".gitignore",
            ".overrid/",
            "gitignore_local_state_rule",
            "local_state_committed",
        ),
        (
            ".gitignore",
            "packages/**/generated/**",
            "gitignore_generated_projection_rule",
            "generated_file_committed",
        ),
        (
            ".gitignore",
            "*.secret.*",
            "gitignore_secret_file_rule",
            "secret_file_committed",
        ),
        (
            ".docdexignore",
            "docs/specs/generated/",
            "docdexignore_generated_specs_rule",
            "docdex_index_hygiene_violation",
        ),
        (
            ".docdexignore",
            "infra/local/state/",
            "docdexignore_local_state_rule",
            "docdex_index_hygiene_violation",
        ),
        (
            ".docdexignore",
            "packages/**/generated/",
            "docdexignore_generated_projection_rule",
            "docdex_index_hygiene_violation",
        ),
    ] {
        push_ignore_file_contains(
            &mut records,
            repo_root,
            ignore_file,
            marker,
            check_name,
            fail_reason,
        );
    }
    for path in [
        ".env",
        ".env.local",
        "secrets.toml",
        "private.key",
        "id_ed25519",
        "infra/local/secrets.toml",
    ] {
        push_forbidden_path_absent(&mut records, repo_root, path);
    }
    push_secret_like_paths_absent(&mut records, repo_root);

    records
}

fn push_path_presence(
    records: &mut Vec<LayoutCheckRecord>,
    repo_root: &Path,
    check_name: &'static str,
    path: &str,
    module_id: Option<&'static str>,
    pass_reason: &'static str,
    fail_reason: &'static str,
) {
    let exists = repo_root.join(path).exists();
    records.push(LayoutCheckRecord {
        check_name,
        status: if exists { "passed" } else { "failed" },
        reason_code: if exists { pass_reason } else { fail_reason },
        path: path.to_owned(),
        owning_phase: "phase_0",
        module_id,
    });
}

fn push_manifest_contains(
    records: &mut Vec<LayoutCheckRecord>,
    repo_root: &Path,
    check_name: &'static str,
    marker: &str,
    fail_reason: &'static str,
) {
    let manifest_path = repo_root.join("overrid.workspace.toml");
    let present = std::fs::read_to_string(&manifest_path)
        .map(|manifest| manifest.contains(marker))
        .unwrap_or(false);
    records.push(LayoutCheckRecord {
        check_name,
        status: if present { "passed" } else { "failed" },
        reason_code: if present {
            "manifest_marker_present"
        } else {
            fail_reason
        },
        path: "overrid.workspace.toml".to_owned(),
        owning_phase: "phase_0",
        module_id: None,
    });
}

fn push_lifecycle_state_validity(records: &mut Vec<LayoutCheckRecord>, repo_root: &Path) {
    let manifest_path = repo_root.join("overrid.workspace.toml");
    let manifest = std::fs::read_to_string(&manifest_path).unwrap_or_default();
    let allowed = [
        "proposed",
        "scaffolded",
        "contracted",
        "wired",
        "validated",
        "accepted",
        "deprecated",
        "removed",
    ];
    let state_catalog_ok = allowed
        .iter()
        .all(|state| manifest.contains(&format!("\"{state}\"")));
    records.push(LayoutCheckRecord {
        check_name: "module_lifecycle_state_catalog",
        status: if state_catalog_ok { "passed" } else { "failed" },
        reason_code: if state_catalog_ok {
            "lifecycle_state_catalog_present"
        } else {
            "module_lifecycle_violation"
        },
        path: "overrid.workspace.toml".to_owned(),
        owning_phase: "phase_0",
        module_id: None,
    });

    for block in manifest.split("[[modules]]").skip(1) {
        if let Some(state) = quoted_manifest_field(block, "lifecycle_state") {
            if !allowed.contains(&state.as_str()) {
                records.push(LayoutCheckRecord {
                    check_name: "module_lifecycle_state_valid",
                    status: "failed",
                    reason_code: "module_lifecycle_violation",
                    path: format!(
                        "overrid.workspace.toml#{}",
                        quoted_manifest_field(block, "name")
                            .unwrap_or_else(|| "unknown".to_owned())
                    ),
                    owning_phase: "phase_0",
                    module_id: None,
                });
            }
        }
    }
}

fn push_accepted_module_validation_evidence(
    records: &mut Vec<LayoutCheckRecord>,
    repo_root: &Path,
) {
    let manifest_path = repo_root.join("overrid.workspace.toml");
    let manifest = std::fs::read_to_string(&manifest_path).unwrap_or_default();
    for block in manifest.split("[[modules]]").skip(1) {
        if quoted_manifest_field(block, "lifecycle_state").as_deref() != Some("accepted") {
            continue;
        }
        let name = quoted_manifest_field(block, "name").unwrap_or_else(|| "unknown".to_owned());
        let has_test_targets =
            block.contains("test_targets = [") && !block.contains("test_targets = []");
        let has_docs = block.contains("documentation_links = [");
        let has_local_stack = block.contains("local_stack_participation = ");
        let ok = has_test_targets && has_docs && has_local_stack;
        records.push(LayoutCheckRecord {
            check_name: "accepted_module_validation_evidence",
            status: if ok { "passed" } else { "failed" },
            reason_code: if ok {
                "accepted_module_validation_evidence_present"
            } else {
                "missing_test_target"
            },
            path: format!("overrid.workspace.toml#{name}"),
            owning_phase: "phase_0",
            module_id: None,
        });
    }
}

fn quoted_manifest_field(block: &str, key: &str) -> Option<String> {
    let needle = format!("{key} = \"");
    let start = block.find(&needle)? + needle.len();
    let rest = &block[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}

fn push_internal_dependency_direction(
    records: &mut Vec<LayoutCheckRecord>,
    repo_root: &Path,
    group: &'static str,
    manifest_path: &str,
    module_id: Option<&'static str>,
    expected_dependencies: &[&str],
) {
    let expected = expected_dependencies
        .iter()
        .map(|dependency| (*dependency).to_owned())
        .collect::<BTreeSet<_>>();
    let actual = internal_dependency_names(&repo_root.join(manifest_path));
    let passed = actual
        .as_ref()
        .map(|dependencies| dependencies == &expected)
        .unwrap_or(false);
    records.push(LayoutCheckRecord {
        check_name: "dependency_direction_group",
        status: if passed { "passed" } else { "failed" },
        reason_code: if passed {
            "dependency_direction_allowed"
        } else {
            "package_boundary_violation"
        },
        path: manifest_path.to_owned(),
        owning_phase: "phase_0",
        module_id,
    });
    if !passed && actual.is_ok() {
        records.push(LayoutCheckRecord {
            check_name: "dependency_direction_expected_group",
            status: "failed",
            reason_code: "package_boundary_violation",
            path: group.to_owned(),
            owning_phase: "phase_0",
            module_id,
        });
    }
}

fn internal_dependency_names(manifest_path: &Path) -> std::io::Result<BTreeSet<String>> {
    let manifest = std::fs::read_to_string(manifest_path)?;
    let mut names = BTreeSet::new();
    let mut in_dependencies = false;
    for raw_line in manifest.lines() {
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.starts_with('[') && line.ends_with(']') {
            in_dependencies = line == "[dependencies]";
            continue;
        }
        if !in_dependencies || line.is_empty() {
            continue;
        }
        if let Some((name, _)) = line.split_once('=') {
            let name = name.trim().trim_matches('"');
            if name.starts_with("overrid-") {
                names.insert(name.to_owned());
            }
        }
    }
    Ok(names)
}

fn push_service_manifest_absence(
    records: &mut Vec<LayoutCheckRecord>,
    repo_root: &Path,
    service_root: &str,
    module_id: Option<&'static str>,
) {
    let first_manifest = first_cargo_manifest_under(repo_root, &repo_root.join(service_root));
    records.push(LayoutCheckRecord {
        check_name: "service_contract_root_not_deployable",
        status: if first_manifest.is_none() {
            "passed"
        } else {
            "failed"
        },
        reason_code: if first_manifest.is_none() {
            "service_split_absent"
        } else {
            "premature_service_split"
        },
        path: first_manifest.unwrap_or_else(|| service_root.to_owned()),
        owning_phase: "phase_0",
        module_id,
    });
}

fn first_cargo_manifest_under(repo_root: &Path, root: &Path) -> Option<String> {
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let mut entries = std::fs::read_dir(&dir)
            .ok()?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            if path.is_file()
                && path
                    .file_name()
                    .map(|name| name.to_string_lossy() == "Cargo.toml")
                    .unwrap_or(false)
            {
                return path
                    .strip_prefix(repo_root)
                    .ok()
                    .map(|relative| relative.to_string_lossy().replace('\\', "/"));
            }
            if path.is_dir() {
                stack.push(path);
            }
        }
    }
    None
}

fn push_generated_specs_clean(records: &mut Vec<LayoutCheckRecord>, repo_root: &Path) {
    let path = "docs/specs/generated";
    let clean = std::fs::read_dir(repo_root.join(path))
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .all(|entry| entry.file_name().to_string_lossy() == ".gitignore")
        })
        .unwrap_or(false);
    records.push(LayoutCheckRecord {
        check_name: "generated_specs_not_committed",
        status: if clean { "passed" } else { "failed" },
        reason_code: if clean {
            "generated_output_clean"
        } else {
            "generated_file_committed"
        },
        path: path.to_owned(),
        owning_phase: "phase_0",
        module_id: Some("docs-specs"),
    });
}

fn push_marker_only_directory(
    records: &mut Vec<LayoutCheckRecord>,
    repo_root: &Path,
    check_name: &'static str,
    path: &str,
    fail_reason: &'static str,
    module_id: Option<&'static str>,
) {
    let clean = std::fs::read_dir(repo_root.join(path))
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .all(|entry| entry.file_name().to_string_lossy() == ".gitignore")
        })
        .unwrap_or(false);
    records.push(LayoutCheckRecord {
        check_name,
        status: if clean { "passed" } else { "failed" },
        reason_code: if clean {
            "local_or_generated_marker_clean"
        } else {
            fail_reason
        },
        path: path.to_owned(),
        owning_phase: "phase_0",
        module_id,
    });
}

fn push_ignore_file_contains(
    records: &mut Vec<LayoutCheckRecord>,
    repo_root: &Path,
    ignore_file: &str,
    marker: &str,
    check_name: &'static str,
    fail_reason: &'static str,
) {
    let present = std::fs::read_to_string(repo_root.join(ignore_file))
        .map(|contents| contents.contains(marker))
        .unwrap_or(false);
    records.push(LayoutCheckRecord {
        check_name,
        status: if present { "passed" } else { "failed" },
        reason_code: if present {
            "ignore_rule_present"
        } else {
            fail_reason
        },
        path: ignore_file.to_owned(),
        owning_phase: "phase_0",
        module_id: None,
    });
}

fn push_forbidden_path_absent(records: &mut Vec<LayoutCheckRecord>, repo_root: &Path, path: &str) {
    let absent = !repo_root.join(path).exists();
    records.push(LayoutCheckRecord {
        check_name: "secret_file_absence",
        status: if absent { "passed" } else { "failed" },
        reason_code: if absent {
            "secret_file_absent"
        } else {
            "secret_file_committed"
        },
        path: path.to_owned(),
        owning_phase: "phase_0",
        module_id: None,
    });
}

fn push_secret_like_paths_absent(records: &mut Vec<LayoutCheckRecord>, repo_root: &Path) {
    let first_secret = first_forbidden_secret_like_path(repo_root);
    records.push(LayoutCheckRecord {
        check_name: "secret_like_path_absence",
        status: if first_secret.is_none() {
            "passed"
        } else {
            "failed"
        },
        reason_code: if first_secret.is_none() {
            "secret_file_absent"
        } else {
            "secret_file_committed"
        },
        path: first_secret.unwrap_or_else(|| "secret-like paths".to_owned()),
        owning_phase: "phase_0",
        module_id: None,
    });
}

fn first_forbidden_secret_like_path(repo_root: &Path) -> Option<String> {
    let mut stack = vec![repo_root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let mut entries = std::fs::read_dir(&dir)
            .ok()?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                if !should_skip_secret_scan_dir(&path) {
                    stack.push(path);
                }
                continue;
            }
            let relative = path
                .strip_prefix(repo_root)
                .ok()
                .map(|relative| relative.to_string_lossy().replace('\\', "/"))?;
            if is_forbidden_secret_like_path(&relative) {
                return Some(relative);
            }
        }
    }
    None
}

fn should_skip_secret_scan_dir(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(
            ".git"
                | ".docdex"
                | ".docdex-state"
                | ".mcoda"
                | ".cache"
                | ".overrid"
                | "target"
                | "node_modules"
                | "coverage"
                | "logs"
                | "tmp"
                | "temp"
        )
    )
}

fn is_forbidden_secret_like_path(relative_path: &str) -> bool {
    let file_name = relative_path
        .rsplit('/')
        .next()
        .unwrap_or(relative_path)
        .to_ascii_lowercase();
    if file_name == ".env.example" || file_name.ends_with(".example") {
        return false;
    }
    file_name == ".env"
        || file_name.starts_with(".env.")
        || file_name.contains(".local.")
        || file_name.contains(".secret.")
        || file_name.ends_with(".key")
        || file_name.ends_with(".pem")
        || file_name.ends_with(".p12")
        || file_name.ends_with(".pfx")
        || file_name.ends_with(".token")
        || file_name.starts_with("secrets.")
        || file_name.starts_with("id_ed25519")
}

fn layout_artifact_refs(ok: bool, records: &[LayoutCheckRecord]) -> Vec<String> {
    let mut artifacts = vec![if ok {
        "layout_check.passed:phase_0:repository_layout".to_owned()
    } else {
        "layout_check.failed:phase_0:repository_layout".to_owned()
    }];
    for record in records.iter().filter(|record| record.status == "failed") {
        if LAYOUT_VALIDATION_ARTIFACTS.contains(&record.reason_code) {
            artifacts.push(format!(
                "{}:{}:{}",
                record.reason_code, record.owning_phase, record.path
            ));
        }
    }
    artifacts
}

fn render_layout_check_human(
    ok: bool,
    repo_root: &Path,
    records: &[LayoutCheckRecord],
    artifact_refs: &[String],
) -> String {
    let mut lines = vec![
        format!("layout:check {}", if ok { "passed" } else { "failed" }),
        format!("repo_root: {}", repo_root.display()),
        format!("artifact_refs: {}", artifact_refs.join(", ")),
    ];
    for record in records {
        lines.push(format!(
            "- {} {} {}",
            record.status, record.reason_code, record.path
        ));
    }
    lines.join("\n")
}

fn render_layout_check_result_json(
    ok: bool,
    repo_root: &Path,
    records: &[LayoutCheckRecord],
    artifact_refs: &[String],
) -> String {
    let checks = records
        .iter()
        .map(render_layout_check_record_json)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "{{",
            "\"command_name\":\"layout:check\",",
            "\"status\":\"{}\",",
            "\"artifact\":\"{}\",",
            "\"repo_root\":\"{}\",",
            "\"artifact_refs\":{},",
            "\"validation_artifact_schema\":{},",
            "\"checks\":[{}]",
            "}}"
        ),
        if ok { "passed" } else { "failed" },
        if ok {
            "layout_check.passed"
        } else {
            "layout_check.failed"
        },
        json_escape(&repo_root.display().to_string()),
        json_owned_string_array(artifact_refs),
        json_string_array(LAYOUT_VALIDATION_ARTIFACTS),
        checks,
    )
}

fn render_layout_check_record_json(record: &LayoutCheckRecord) -> String {
    format!(
        concat!(
            "{{",
            "\"check\":\"{}\",",
            "\"status\":\"{}\",",
            "\"reason_code\":\"{}\",",
            "\"path\":\"{}\",",
            "\"owning_phase\":\"{}\",",
            "\"module_id\":{}",
            "}}"
        ),
        json_escape(record.check_name),
        json_escape(record.status),
        json_escape(record.reason_code),
        json_escape(&record.path),
        json_escape(record.owning_phase),
        json_optional_string(record.module_id),
    )
}

fn render_layout_check_error_json(records: &[LayoutCheckRecord]) -> String {
    let failed = records
        .iter()
        .find(|record| record.status == "failed")
        .expect("failed layout check must have at least one failing record");
    let message = format!("layout check failed for {}", failed.path);
    let error_decode_record = decode_phase6_error(
        failed.reason_code,
        &message,
        ExitCodeClass::Config,
        RetryClass::NotRetryable,
    );
    format!(
        concat!(
            "{{",
            "\"reason_code\":\"{}\",",
            "\"message\":\"{}\",",
            "\"phase_gate\":\"phase_0\",",
            "\"retry_class\":\"{}\",",
            "\"remediation_hint\":\"{}\",",
            "\"error_decode_record\":{}",
            "}}"
        ),
        json_escape(failed.reason_code),
        json_escape(&message),
        json_escape(RetryClass::NotRetryable.as_str()),
        json_escape(&error_decode_record.remediation_hint),
        render_error_decode_record_json(&error_decode_record),
    )
}

fn auth_command_result(command: AuthCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(globals, globals.actor.as_deref().unwrap_or("actor_local"));
    match command {
        AuthCommand::Login => bootstrap_command_result(
            BootstrapCommandFamily::Auth,
            command.as_str(),
            "auth_login_command",
            target_ref,
            true,
            false,
            BootstrapResultKind::AuthLogin,
            globals,
        ),
        AuthCommand::Whoami => bootstrap_command_result(
            BootstrapCommandFamily::Auth,
            command.as_str(),
            "auth_whoami_query",
            target_ref,
            false,
            false,
            BootstrapResultKind::AuthWhoami,
            globals,
        ),
    }
}

fn tenant_command_result(command: TenantCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref =
        default_target_ref(globals, globals.tenant.as_deref().unwrap_or("tenant_local"));
    bootstrap_command_result(
        BootstrapCommandFamily::Tenant,
        command.as_str(),
        "tenant_bootstrap_command",
        target_ref,
        matches!(command, TenantCommand::Create | TenantCommand::Suspend),
        matches!(command, TenantCommand::Suspend),
        BootstrapResultKind::Tenant,
        globals,
    )
}

fn identity_command_result(command: IdentityCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(globals, globals.actor.as_deref().unwrap_or("actor_local"));
    bootstrap_command_result(
        BootstrapCommandFamily::Identity,
        command.as_str(),
        "identity_bootstrap_command",
        target_ref,
        matches!(command, IdentityCommand::Create | IdentityCommand::Disable),
        matches!(command, IdentityCommand::Disable),
        BootstrapResultKind::Identity,
        globals,
    )
}

fn key_command_result(command: KeyCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(globals, globals.key_id.as_deref().unwrap_or("key-1"));
    bootstrap_command_result(
        BootstrapCommandFamily::Key,
        command.as_str(),
        "key_bootstrap_command",
        target_ref,
        matches!(
            command,
            KeyCommand::Enroll | KeyCommand::Rotate | KeyCommand::Revoke
        ),
        matches!(command, KeyCommand::Revoke),
        BootstrapResultKind::Key,
        globals,
    )
}

fn manifest_command_result(command: ManifestCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(
        globals,
        globals.manifest_ref.as_deref().unwrap_or("manifest_local"),
    );
    let result_kind = match command {
        ManifestCommand::Validate => BootstrapResultKind::ManifestValidate,
        ManifestCommand::Submit => BootstrapResultKind::ManifestSubmit,
        ManifestCommand::Inspect => BootstrapResultKind::ManifestInspect,
    };
    bootstrap_command_result(
        BootstrapCommandFamily::Manifest,
        command.as_str(),
        "manifest_bootstrap_command",
        target_ref,
        matches!(command, ManifestCommand::Submit),
        false,
        result_kind,
        globals,
    )
}

fn node_command_result(command: NodeCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(globals, "node_local");
    let payload_type = match command {
        NodeCommand::Register => "node_registration_command",
        NodeCommand::Inspect => "node_inspection_query",
        NodeCommand::Health => "node_health_query",
    };
    let context = match prepare_bootstrap_context(
        BootstrapCommandFamily::Node,
        command.as_str(),
        payload_type,
        target_ref,
        matches!(command, NodeCommand::Register),
        false,
        globals,
    ) {
        Ok(context) => context,
        Err(result) => return result,
    };

    success(render_node_result(command, globals, &context))
}

fn workload_command_result(command: WorkloadCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(
        globals,
        globals.workload_ref.as_deref().unwrap_or("workload_local"),
    );
    match command {
        WorkloadCommand::Submit | WorkloadCommand::Status | WorkloadCommand::Timeline => {
            let result_kind = match command {
                WorkloadCommand::Submit => BootstrapResultKind::WorkloadSubmit,
                WorkloadCommand::Status => BootstrapResultKind::WorkloadStatus,
                WorkloadCommand::Timeline => BootstrapResultKind::WorkloadTimeline,
                WorkloadCommand::Logs
                | WorkloadCommand::Cancel
                | WorkloadCommand::Result
                | WorkloadCommand::Follow => unreachable!("execution commands are handled below"),
            };
            bootstrap_command_result(
                BootstrapCommandFamily::Workload,
                command.as_str(),
                "synthetic_workload_command",
                target_ref,
                matches!(command, WorkloadCommand::Submit),
                false,
                result_kind,
                globals,
            )
        }
        WorkloadCommand::Logs
        | WorkloadCommand::Cancel
        | WorkloadCommand::Result
        | WorkloadCommand::Follow => {
            let context = match prepare_bootstrap_context(
                BootstrapCommandFamily::Workload,
                command.as_str(),
                "workload_execution_command",
                target_ref,
                matches!(command, WorkloadCommand::Cancel),
                false,
                globals,
            ) {
                Ok(context) => context,
                Err(result) => return result,
            };
            success(render_workload_execution_result(command, globals, &context))
        }
    }
}

fn policy_command_result(command: PolicyCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(globals, "policy_accept");
    let context = match prepare_bootstrap_context(
        BootstrapCommandFamily::Policy,
        command.as_str(),
        "policy_dry_run_request",
        target_ref,
        false,
        false,
        globals,
    ) {
        Ok(context) => context,
        Err(result) => return result,
    };

    success(render_policy_dry_run_result(command, &context, globals))
}

fn package_command_result(command: PackageCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(globals, "package_accept");
    let context = match prepare_bootstrap_context(
        BootstrapCommandFamily::Package,
        command.as_str(),
        "package_validation_request",
        target_ref,
        false,
        false,
        globals,
    ) {
        Ok(context) => context,
        Err(result) => return result,
    };

    success(render_package_validation_result(command, &context, globals))
}

fn usage_command_result(command: UsageCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(globals, "usage_within_budget");
    let context = match prepare_bootstrap_context(
        BootstrapCommandFamily::Usage,
        command.as_str(),
        "usage_oru_read_request",
        target_ref,
        false,
        false,
        globals,
    ) {
        Ok(context) => context,
        Err(result) => return result,
    };

    success(render_usage_result(command, &context, globals))
}

fn receipt_command_result(command: ReceiptCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(globals, "receipt_local");
    let context = match prepare_bootstrap_context(
        BootstrapCommandFamily::Receipt,
        command.as_str(),
        "receipt_read_request",
        target_ref,
        false,
        false,
        globals,
    ) {
        Ok(context) => context,
        Err(result) => return result,
    };

    success(render_receipt_ledger_result(
        command.as_str(),
        &context,
        globals,
    ))
}

fn ledger_command_result(command: LedgerCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(globals, "ledger_local");
    let context = match prepare_bootstrap_context(
        BootstrapCommandFamily::Ledger,
        command.as_str(),
        "ledger_ref_read_request",
        target_ref,
        false,
        false,
        globals,
    ) {
        Ok(context) => context,
        Err(result) => return result,
    };

    success(render_receipt_ledger_result(
        command.as_str(),
        &context,
        globals,
    ))
}

fn dispute_command_result(command: DisputeCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(globals, "dispute_open");
    let context = match prepare_bootstrap_context(
        BootstrapCommandFamily::Dispute,
        command.as_str(),
        "dispute_read_request",
        target_ref,
        false,
        false,
        globals,
    ) {
        Ok(context) => context,
        Err(result) => return result,
    };

    success(render_dispute_result(command, &context, globals))
}

#[derive(Debug, Clone, Copy)]
enum BootstrapResultKind {
    AuthLogin,
    AuthWhoami,
    Tenant,
    Identity,
    Key,
    ManifestValidate,
    ManifestSubmit,
    ManifestInspect,
    WorkloadSubmit,
    WorkloadStatus,
    WorkloadTimeline,
}

#[derive(Debug, Clone)]
struct BootstrapContext {
    profile: CliProfile,
    credential: CredentialReference,
    trace_id: String,
    idempotency_key: String,
    idempotency_key_source: String,
    target_ref: String,
    fingerprint: CanonicalIdempotencyFingerprint,
    retry_policy: RetryTimeoutPolicy,
    cache_record: LocalIdempotencyCacheRecord,
    signed_envelope: Option<SignedCommandEnvelope>,
    audit_refs: Vec<String>,
}

#[allow(clippy::too_many_arguments)]
fn bootstrap_command_result(
    family: BootstrapCommandFamily,
    command_name: &'static str,
    payload_type: &'static str,
    target_ref: String,
    mutating: bool,
    admin_impacting: bool,
    result_kind: BootstrapResultKind,
    globals: &GlobalOptions,
) -> CliRunResult {
    let context = match prepare_bootstrap_context(
        family,
        command_name,
        payload_type,
        target_ref,
        mutating,
        admin_impacting,
        globals,
    ) {
        Ok(context) => context,
        Err(result) => return result,
    };

    success(render_bootstrap_result(
        family,
        command_name,
        payload_type,
        result_kind,
        globals,
        &context,
    ))
}

#[allow(clippy::too_many_arguments)]
fn prepare_bootstrap_context(
    family: BootstrapCommandFamily,
    command_name: &'static str,
    payload_type: &'static str,
    target_ref: String,
    mutating: bool,
    admin_impacting: bool,
    globals: &GlobalOptions,
) -> Result<BootstrapContext, CliRunResult> {
    let profile = match build_profile(globals) {
        Ok(profile) => profile,
        Err(error) => {
            return Err(phase3_error_result(
                EXIT_CONFIG,
                globals.output,
                "profile_validation_failed",
                &error.to_string(),
            ))
        }
    };
    let credential = match build_credential(globals) {
        Ok(credential) => credential,
        Err(error) => {
            return Err(phase3_error_result(
                EXIT_CREDENTIAL,
                globals.output,
                "credential_validation_failed",
                &error.to_string(),
            ))
        }
    };

    let safety = match enforce_profile_environment(CommandSafetyInput {
        profile: &profile,
        credential: &credential,
        endpoint_override: globals.endpoint_override.as_deref(),
        explicit_profile: globals.profile.is_some(),
        confirm_profile: globals.confirm_profile,
        mutating,
        admin_impacting,
        reason: globals.reason.as_deref(),
        command_type: command_name,
    }) {
        Ok(safety) => safety,
        Err(error) => return Err(sdk_error_result(globals.output, error)),
    };

    let trace_id = globals
        .trace_id
        .clone()
        .unwrap_or_else(|| LOCAL_TRACE_ID.to_owned());
    let retry_policy = retry_timeout_policy(globals.max_retries, globals.timeout_ms);
    let payload_hash = canonical_payload_hash(payload_type, target_ref.as_str(), globals);
    let fingerprint = match canonical_fingerprint(
        &profile,
        command_name,
        target_ref.as_str(),
        payload_hash.as_str(),
        globals,
    ) {
        Ok(fingerprint) => fingerprint,
        Err(error) => {
            return Err(contract_error_result(
                globals.output,
                command_name,
                &error.to_string(),
            ))
        }
    };
    let (idempotency_key, idempotency_key_source) =
        if let Some(idempotency_key) = globals.idempotency_key.clone() {
            (idempotency_key, "explicit".to_owned())
        } else if globals.new_idempotency_key {
            (
                fingerprint.new_operation_idempotency_key(&trace_id),
                "new_operation".to_owned(),
            )
        } else {
            (
                fingerprint.idempotency_key(),
                "canonical_fingerprint".to_owned(),
            )
        };
    let cache_record = LocalIdempotencyCacheRecord::new(
        profile.name.clone(),
        profile.environment,
        fingerprint.fingerprint.clone(),
        idempotency_key.clone(),
    );
    let audit_refs = if mutating {
        vec![format!(
            "audit_cli_bootstrap_{}",
            id_component(command_name)
        )]
    } else {
        Vec::new()
    };
    let signed_envelope = if mutating {
        let signature_ref = safety
            .signer_handoff
            .as_ref()
            .map(|handoff| handoff.signature_ref.clone())
            .unwrap_or_else(|| {
                format!(
                    "sigref:{}:{}:{}",
                    credential.namespace,
                    credential.key_id,
                    id_component(command_name)
                )
            });
        match SignedCommandEnvelope::new(
            family,
            command_name,
            profile.tenant_id.clone(),
            profile.actor_id.clone(),
            target_ref.clone(),
            payload_type,
            globals.expected_state.clone(),
            globals.reason.clone(),
            idempotency_key.clone(),
            trace_id.clone(),
            signature_ref,
        ) {
            Ok(envelope) => Some(envelope),
            Err(error) => {
                return Err(contract_error_result(
                    globals.output,
                    command_name,
                    &error.to_string(),
                ))
            }
        }
    } else {
        None
    };

    Ok(BootstrapContext {
        profile,
        credential,
        trace_id,
        idempotency_key,
        idempotency_key_source,
        target_ref,
        fingerprint,
        retry_policy,
        cache_record,
        signed_envelope,
        audit_refs,
    })
}

fn planned_command_result(command: PlannedCommand, globals: &GlobalOptions) -> CliRunResult {
    if command == PlannedCommand::ReleaseReadiness {
        return release_readiness_result(globals);
    }

    let reason_code = "not_available_in_phase";
    let message = format!(
        "{} is gated until {}",
        command.command_name(),
        command.phase_gate()
    );

    let stdout = match globals.output {
        OutputMode::Human => format!("{reason_code}: {message} trace_id={LOCAL_TRACE_ID}"),
        OutputMode::Json => render_error_json(
            command.command_name(),
            reason_code,
            &message,
            command.phase_gate(),
            ExitCodeClass::Phase,
            RetryClass::NotRetryable,
            &["parsed", "failed"],
            &["capability_route_unavailable", "fail_closed"],
            &[CapabilityRoute {
                route: command.command_name(),
                phase_gate: command.phase_gate(),
                available: false,
            }],
            Some(LOCAL_TRACE_ID),
        ),
    };

    CliRunResult {
        exit_code: EXIT_NOT_AVAILABLE_IN_PHASE,
        stdout,
        stderr: String::new(),
    }
}

fn release_readiness_result(globals: &GlobalOptions) -> CliRunResult {
    let report = CliReleaseReadinessReport::new();
    let stdout = match globals.output {
        OutputMode::Human => [
            "release_readiness: ready",
            "contract_snapshots: schema_contracts output_envelope exit_code_registry help_text json_output human_output error_decode_records",
            "security_review: secret_free no_raw_keys no_tokens no_signatures no_private_payloads",
            "phase_availability: available_through_phase_10 phase_10_federation_handoff_denied phase_7_or_phase_13_handoff_denied",
            "integration_matrix: tenant identity key manifest workload policy package usage receipt cancellation timeout retry product_workflows",
            "handoff: high_risk_phase7_phase13_operations_disabled",
        ]
        .join("\n"),
        OutputMode::Json => {
            let result_json = render_release_readiness_report_json(&report);
            render_success_json(
                "release-readiness",
                &result_json,
                &["parsed", "completed"],
                globals.profile.as_deref(),
                globals.endpoint_fingerprint.as_deref(),
                &[
                    "local_contracts_available",
                    "snapshot_validation_available",
                    "security_redaction_validated",
                    "handoff_gates_fail_closed",
                ],
                &[CapabilityRoute {
                    route: "release-readiness",
                    phase_gate: "phase_10",
                    available: true,
                }],
            )
        }
    };

    CliRunResult {
        exit_code: EXIT_SUCCESS,
        stdout,
        stderr: String::new(),
    }
}

fn parse_error_result(args: &[String], message: &str) -> CliRunResult {
    let output = requested_output_mode(args);
    let stdout = match output {
        OutputMode::Human => String::new(),
        OutputMode::Json => render_error_json(
            "parser",
            "usage_error",
            message,
            "phase_4",
            ExitCodeClass::Usage,
            RetryClass::NotRetryable,
            &["parsed", "failed"],
            &["parser_validation_failed", "fail_closed"],
            &[CapabilityRoute {
                route: "parser",
                phase_gate: "phase_4",
                available: false,
            }],
            Some(LOCAL_TRACE_ID),
        ),
    };
    let stderr = match output {
        OutputMode::Human => format!("usage_error: {message}"),
        OutputMode::Json => String::new(),
    };
    CliRunResult {
        exit_code: EXIT_USAGE,
        stdout,
        stderr,
    }
}

fn requested_output_mode(args: &[String]) -> OutputMode {
    let mut output = OutputMode::Human;
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--json" => output = OutputMode::Json,
            "--output" => {
                if matches!(iter.next().map(String::as_str), Some("json")) {
                    output = OutputMode::Json;
                }
            }
            _ => {}
        }
    }
    output
}

fn build_profile(globals: &GlobalOptions) -> Result<CliProfile, ProfileValidationError> {
    let environment =
        EnvironmentClass::parse(required(globals.environment.as_deref(), "environment")?)?;
    let credential_class = CredentialReferenceClass::parse(required(
        globals.credential_class.as_deref(),
        "credential class",
    )?)?;
    let fixture_allowance =
        FixtureAllowance::parse(globals.fixture_allowance.as_deref().unwrap_or("denied"))?;
    let confirmation_policy = ConfirmationPolicy::parse(
        globals
            .confirmation_policy
            .as_deref()
            .unwrap_or("confirm_sensitive"),
    )?;

    let profile = CliProfile {
        name: required(globals.profile.as_deref(), "profile name")?.to_owned(),
        endpoint: required(globals.endpoint.as_deref(), "endpoint")?.to_owned(),
        endpoint_fingerprint: required(
            globals.endpoint_fingerprint.as_deref(),
            "endpoint fingerprint",
        )?
        .to_owned(),
        environment,
        tenant_id: required(globals.tenant.as_deref(), "tenant id")?.to_owned(),
        actor_id: required(globals.actor.as_deref(), "actor id")?.to_owned(),
        credential_namespace: required(
            globals.credential_namespace.as_deref(),
            "credential namespace",
        )?
        .to_owned(),
        allowed_credential_classes: vec![credential_class],
        fixture_allowance,
        default_output_mode: globals.output.as_str().to_owned(),
        confirmation_policy,
        schema_pins: vec![globals
            .schema_pin
            .clone()
            .unwrap_or_else(|| SUPPORTED_SCHEMA_VERSION.to_owned())],
        test_harness_profile: globals.test_harness_profile,
    };
    profile.validate()?;
    Ok(profile)
}

fn build_credential(
    globals: &GlobalOptions,
) -> Result<CredentialReference, ProfileValidationError> {
    let class = CredentialReferenceClass::parse(required(
        globals.credential_class.as_deref(),
        "credential class",
    )?)?;
    let reference_id = required(globals.credential_ref.as_deref(), "credential reference")?;
    let namespace = required(
        globals.credential_namespace.as_deref(),
        "credential namespace",
    )?;
    let key_id = globals.key_id.as_deref().unwrap_or(reference_id);

    Ok(CredentialReference {
        reference_id: reference_id.to_owned(),
        class,
        namespace: namespace.to_owned(),
        key_id: key_id.to_owned(),
        revoked: globals.revoked,
        expired: globals.expired,
    })
}

fn required<'a>(
    value: Option<&'a str>,
    field: &'static str,
) -> Result<&'a str, ProfileValidationError> {
    let Some(value) = value else {
        return Err(ProfileValidationError::MissingRequiredField(field));
    };
    if value.trim().is_empty() {
        Err(ProfileValidationError::MissingRequiredField(field))
    } else {
        Ok(value)
    }
}

fn sdk_error_result(output: OutputMode, error: SdkError) -> CliRunResult {
    let (exit_class, reason_code) = match error {
        SdkError::Profile(_) => (ExitCodeClass::Config, "profile_validation_failed"),
        SdkError::Credential(_) => (ExitCodeClass::Credential, "credential_validation_failed"),
        SdkError::MissingReason => (ExitCodeClass::Usage, "missing_reason"),
        SdkError::MissingProfileConfirmation { .. } => {
            (ExitCodeClass::Usage, "missing_profile_confirmation")
        }
        SdkError::MissingExplicitProfile { .. } => {
            (ExitCodeClass::Usage, "missing_explicit_profile")
        }
        SdkError::UnsafeEndpointOverride { .. } => {
            (ExitCodeClass::Config, "unsafe_endpoint_override")
        }
        SdkError::PrivateServiceTarget(_) | SdkError::MissingOvergateTarget(_) => {
            (ExitCodeClass::Config, "invalid_overgate_endpoint")
        }
        SdkError::UnsupportedScheme(_) => (ExitCodeClass::Config, "unsupported_endpoint_scheme"),
        SdkError::Compatibility(ref rejection) => (ExitCodeClass::Schema, rejection.reason_code()),
        SdkError::Contract(_) => (ExitCodeClass::Schema, "contract_validation_failed"),
    };
    phase_error_result(
        exit_class,
        output,
        reason_code,
        &error.to_string(),
        "phase_3",
        "sdk_guard",
        &["parsed", "profile_loaded", "failed"],
    )
}

fn phase3_error_result(
    exit_code: i32,
    output: OutputMode,
    reason_code: &str,
    message: &str,
) -> CliRunResult {
    phase_error_result(
        exit_class_for_code(exit_code),
        output,
        reason_code,
        message,
        "phase_3",
        "profile_or_credential",
        &["parsed", "failed"],
    )
}

fn phase_error_result(
    exit_class: ExitCodeClass,
    output: OutputMode,
    reason_code: &str,
    message: &str,
    phase_gate: &str,
    command_name: &str,
    lifecycle: &[&str],
) -> CliRunResult {
    let stdout = match output {
        OutputMode::Human => format!("{reason_code}: {message} trace_id={LOCAL_TRACE_ID}"),
        OutputMode::Json => render_error_json(
            command_name,
            reason_code,
            message,
            phase_gate,
            exit_class,
            RetryClass::NotRetryable,
            lifecycle,
            &["local_validation_failed", "fail_closed"],
            &[],
            Some(LOCAL_TRACE_ID),
        ),
    };
    CliRunResult {
        exit_code: exit_class.code(),
        stdout,
        stderr: String::new(),
    }
}

fn contract_error_result(output: OutputMode, command_name: &str, message: &str) -> CliRunResult {
    phase_error_result(
        ExitCodeClass::Schema,
        output,
        "contract_validation_failed",
        message,
        "phase_5",
        command_name,
        &["parsed", "payload_validated", "failed"],
    )
}

fn exit_class_for_code(exit_code: i32) -> ExitCodeClass {
    match exit_code {
        EXIT_SUCCESS => ExitCodeClass::Success,
        EXIT_USAGE => ExitCodeClass::Usage,
        EXIT_CONFIG => ExitCodeClass::Config,
        EXIT_CREDENTIAL => ExitCodeClass::Credential,
        EXIT_NOT_AVAILABLE_IN_PHASE => ExitCodeClass::Phase,
        _ => ExitCodeClass::Platform,
    }
}

fn render_version(globals: &GlobalOptions) -> String {
    match globals.output {
        OutputMode::Human => human_version_lines().join("\n"),
        OutputMode::Json => render_version_json(),
    }
}

fn render_doctor(globals: &GlobalOptions) -> String {
    match globals.output {
        OutputMode::Human => [
            "doctor_status: ok",
            "schema: cli-command.v0.1",
            "sdk_target: overgate_only",
            "diagnostic_bundle: secret_free_refs_only",
            "capability_cache: local_static stale_age_ms=0",
        ]
        .join("\n"),
        OutputMode::Json => render_success_json(
            "doctor",
            concat!(
                "{",
                "\"doctor_status\":\"ok\",",
                "\"schema_version\":\"cli-command.v0.1\",",
                "\"sdk_target\":\"overgate_only\",",
                "\"profile_storage_policy\":\"owner_only_file_backed_config\",",
                "\"diagnostic_bundle_ref\":\"local_diagnostic_bundle\",",
                "\"dependency_status\":[\"local_contracts_available\",\"sdk_metadata_available\",\"capability_cache_static\"],",
                "\"redaction_policy\":\"secret_free_refs_only\"",
                "}"
            ),
            &["parsed", "completed"],
            globals.profile.as_deref(),
            globals.endpoint_fingerprint.as_deref(),
            &[
                "local_contracts_available",
                "sdk_metadata_available",
                "capability_cache_static",
            ],
            &[CapabilityRoute {
                route: "doctor",
                phase_gate: "phase_4",
                available: true,
            }],
        ),
    }
}

fn render_help(all_phases: bool) -> String {
    let mut lines = vec![
        "overrid".to_owned(),
        "".to_owned(),
        "available commands:".to_owned(),
        "  command-registry                print semantic root command registry".to_owned(),
        "  version                         print CLI, SDK, and schema compatibility metadata".to_owned(),
        "  doctor                          print redacted local diagnostics and capability status".to_owned(),
        "  build                           print Rust-owned build orchestration record".to_owned(),
        "  schema:check                    print schema check orchestration record".to_owned(),
        "  docs:check                      print docs check orchestration record".to_owned(),
        "  layout:check                    run Repository Layout checks with stable artifacts".to_owned(),
        "  profile create|list|select|inspect|reset".to_owned(),
        "  credential enroll|inspect".to_owned(),
        "  idempotency-cache inspect|reset".to_owned(),
        "  auth login|whoami".to_owned(),
        "  tenant create|list|inspect|suspend".to_owned(),
        "  identity create|list|inspect|disable".to_owned(),
        "  key enroll|list|rotate|revoke".to_owned(),
        "  manifest validate|submit|inspect".to_owned(),
        "  node register|inspect|health".to_owned(),
        "  workload submit|status|timeline|logs|cancel|result|follow".to_owned(),
        "  policy dry-run".to_owned(),
        "  package validate".to_owned(),
        "  usage show".to_owned(),
        "  receipt show".to_owned(),
        "  ledger inspect".to_owned(),
        "  dispute list|inspect".to_owned(),
        "  dev start|stop|restart|status|reset|seed|smoke|logs|doctor|prune".to_owned(),
        "  dev:start|dev:stop|dev:reset|dev:seed|dev:status".to_owned(),
        "  test integration|scenario|list|reset|artifacts".to_owned(),
        "  test:integration".to_owned(),
        "  release-readiness               run Phase 10 release, security, and handoff validation evidence".to_owned(),
        "  help                            print command help".to_owned(),
        "".to_owned(),
        "global flags:".to_owned(),
        "  --json                          render stable JSON output".to_owned(),
        "  --output MODE                   human or json".to_owned(),
        "  --no-color                      disable color".to_owned(),
        "  --verbose                       include local diagnostic detail".to_owned(),
        "  --profile NAME                  select a local profile".to_owned(),
        "  --phase N                       filter integration harness scenarios or local-stack capabilities by master phase".to_owned(),
        "  --service REF                   filter integration harness scenarios by required service".to_owned(),
        "  --tag TAG                       filter integration harness scenarios by tag".to_owned(),
        "  --changed-path PATH             filter integration harness scenarios by changed path".to_owned(),
        "  --required-dependency REF       filter integration harness scenarios by service or fixture dependency".to_owned(),
        "  --gate-class CLASS              filter integration harness scenarios by gate class".to_owned(),
        "  --scenario-name ID              filter integration harness scenarios by exact scenario id".to_owned(),
        "  --environment CLASS             local, seed, staging, production_like, or ci".to_owned(),
        "  --endpoint URL                  Overgate endpoint".to_owned(),
        "  --endpoint-fingerprint VALUE    pinned endpoint identity".to_owned(),
        "  --credential-class CLASS        keychain, secret_service, encrypted_store, signing_agent, hardware_token, fixture, or ci_reference".to_owned(),
        "  --credential-ref REF            secret-free credential reference".to_owned(),
        "  --confirm-profile               confirm sensitive profile use".to_owned(),
        "  --reason TEXT                   reason for admin-impacting operations".to_owned(),
        "  --trace-id ID                   override local trace id".to_owned(),
        "  --idempotency-key KEY           override deterministic idempotency key".to_owned(),
        "  --new-idempotency-key           create a new key for the same canonical fingerprint".to_owned(),
        "  --timeout-ms MS                 bounded SDK wait timeout".to_owned(),
        "  --timeout MS                    bounded wait timeout alias".to_owned(),
        "  --poll-interval MS              bounded execution poll interval".to_owned(),
        "  --wait                          wait for execution state refs".to_owned(),
        "  --follow                        follow execution state/log refs".to_owned(),
        "  --max-retries COUNT             bounded SDK retry count".to_owned(),
        "  --expected-state STATE          expected current state for mutation".to_owned(),
        "  --target-ref REF                command target reference".to_owned(),
        "  --manifest-kind KIND            resource, workload, package, provider, or native_app".to_owned(),
        "  --manifest-ref REF              manifest reference".to_owned(),
        "  --workload-kind KIND            synthetic".to_owned(),
        "  --workload-ref REF              workload reference".to_owned(),
        "  --dry-run                       render validated command without implying execution".to_owned(),
        "  --all-phases                    show phase-gated commands".to_owned(),
    ];

    if all_phases {
        lines.extend([
            "".to_owned(),
            "phase-gated commands:".to_owned(),
            "  deployment helpers                       phase_9".to_owned(),
            "  federation|public-interest|purpose-tag   phase_10".to_owned(),
            "  governance|incident|compliance|migration phase_7_or_phase_13".to_owned(),
        ]);
    }

    lines.join("\n")
}

#[derive(Debug, Clone, Copy)]
struct CapabilityRoute<'a> {
    route: &'a str,
    phase_gate: &'a str,
    available: bool,
}

fn render_success_json(
    command_name: &str,
    result_json: &str,
    lifecycle: &[&str],
    profile_name: Option<&str>,
    endpoint_fingerprint: Option<&str>,
    dependency_status: &[&str],
    capabilities: &[CapabilityRoute<'_>],
) -> String {
    render_success_json_with_trace(
        command_name,
        result_json,
        lifecycle,
        profile_name,
        endpoint_fingerprint,
        dependency_status,
        capabilities,
        None,
        &[],
    )
}

#[allow(clippy::too_many_arguments)]
fn render_success_json_with_trace(
    command_name: &str,
    result_json: &str,
    lifecycle: &[&str],
    profile_name: Option<&str>,
    endpoint_fingerprint: Option<&str>,
    dependency_status: &[&str],
    capabilities: &[CapabilityRoute<'_>],
    trace_id: Option<&str>,
    audit_refs: &[String],
) -> String {
    render_envelope_json(
        command_name,
        true,
        result_json,
        "null",
        trace_id,
        None,
        ExitCodeClass::Success,
        RetryClass::NotRetryable,
        lifecycle,
        profile_name,
        endpoint_fingerprint,
        dependency_status,
        capabilities,
        audit_refs,
    )
}

fn render_harness_error_json(output: &HarnessCliOutput, exit_class: ExitCodeClass) -> String {
    let phase_gate = output
        .phase_filter
        .map(|phase| format!("phase_{phase}"))
        .unwrap_or_else(|| "phase_0".to_owned());
    let error_decode_record = decode_phase6_error(
        &output.reason_code,
        &output.message,
        exit_class,
        RetryClass::NotRetryable,
    );
    format!(
        concat!(
            "{{",
            "\"reason_code\":\"{}\",",
            "\"message\":\"{}\",",
            "\"phase_gate\":\"{}\",",
            "\"retry_class\":\"{}\",",
            "\"remediation_hint\":\"{}\",",
            "\"error_decode_record\":{}",
            "}}"
        ),
        json_escape(&output.reason_code),
        json_escape(&output.message),
        json_escape(&phase_gate),
        json_escape(RetryClass::NotRetryable.as_str()),
        json_escape(&error_decode_record.remediation_hint),
        render_error_decode_record_json(&error_decode_record),
    )
}

fn render_error_json(
    command_name: &str,
    reason_code: &str,
    message: &str,
    phase_gate: &str,
    exit_class: ExitCodeClass,
    retry_class: RetryClass,
    lifecycle: &[&str],
    dependency_status: &[&str],
    capabilities: &[CapabilityRoute<'_>],
    trace_id: Option<&str>,
) -> String {
    let error_decode_record = decode_phase6_error(reason_code, message, exit_class, retry_class);
    let error_json = format!(
        concat!(
            "{{",
            "\"reason_code\":\"{}\",",
            "\"message\":\"{}\",",
            "\"phase_gate\":\"{}\",",
            "\"retry_class\":\"{}\",",
            "\"remediation_hint\":\"{}\",",
            "\"error_decode_record\":{}",
            "}}"
        ),
        json_escape(reason_code),
        json_escape(message),
        json_escape(phase_gate),
        json_escape(retry_class.as_str()),
        json_escape(&error_decode_record.remediation_hint),
        render_error_decode_record_json(&error_decode_record),
    );
    render_envelope_json(
        command_name,
        false,
        "null",
        &error_json,
        trace_id,
        Some(reason_code),
        exit_class,
        retry_class,
        lifecycle,
        None,
        None,
        dependency_status,
        capabilities,
        &[],
    )
}

fn render_local_stack_output_json(output: &LocalStackCommandOutput) -> String {
    let lifecycle = output.lifecycle_strs();
    let dependency_status = output.dependency_status_strs();
    let capabilities = local_stack_capability_routes(output);
    let result_json = if output.is_ok() {
        output.result_json()
    } else {
        "null".to_owned()
    };
    let error_json = if output.is_ok() {
        "null".to_owned()
    } else {
        output.error_json()
    };
    render_envelope_json(
        &output.command_name,
        output.is_ok(),
        &result_json,
        &error_json,
        Some(&output.trace_id),
        (!output.is_ok()).then_some(output.reason_code.as_str()),
        output.exit_class,
        output.retry_class,
        &lifecycle,
        Some(output.profile.as_str()),
        None,
        &dependency_status,
        &capabilities,
        &output.artifact_refs,
    )
}

fn local_stack_capability_routes(output: &LocalStackCommandOutput) -> Vec<CapabilityRoute<'_>> {
    output
        .capabilities
        .iter()
        .map(|capability| CapabilityRoute {
            route: capability.service_id.as_str(),
            phase_gate: capability.phase_gate.as_str(),
            available: capability.available,
        })
        .collect()
}

fn render_envelope_json(
    command_name: &str,
    ok: bool,
    result_json: &str,
    error_json: &str,
    trace_id: Option<&str>,
    reason_code: Option<&str>,
    exit_class: ExitCodeClass,
    retry_class: RetryClass,
    lifecycle: &[&str],
    profile_name: Option<&str>,
    endpoint_fingerprint: Option<&str>,
    dependency_status: &[&str],
    capabilities: &[CapabilityRoute<'_>],
    audit_refs: &[String],
) -> String {
    format!(
        concat!(
            "{{",
            "\"schema_version\":\"{}\",",
            "\"ok\":{},",
            "\"result\":{},",
            "\"error\":{},",
            "\"trace_id\":{},",
            "\"reason_code\":{},",
            "\"retry_class\":\"{}\",",
            "\"exit_code\":{},",
            "\"exit_class\":\"{}\",",
            "\"timing_ms\":{},",
            "\"lifecycle\":{},",
            "\"diagnostic_bundle\":{},",
            "\"capabilities\":{},",
            "\"audit_refs\":{},",
            "\"warnings\":[]",
            "}}"
        ),
        json_escape(SUPPORTED_SCHEMA_VERSION),
        ok,
        result_json,
        error_json,
        json_optional_string(trace_id),
        json_optional_string(reason_code),
        json_escape(retry_class.as_str()),
        exit_class.code(),
        json_escape(exit_class.as_str()),
        TIMING_MS,
        render_lifecycle_json(lifecycle),
        render_diagnostic_bundle_json(
            command_name,
            profile_name,
            endpoint_fingerprint,
            trace_id,
            reason_code,
            dependency_status,
        ),
        render_capabilities_json(capabilities),
        json_owned_string_array(audit_refs),
    )
}

fn render_error_decode_record_json(record: &ErrorDecodeRecord) -> String {
    format!(
        concat!(
            "{{",
            "\"source_family\":\"{}\",",
            "\"reason_code\":\"{}\",",
            "\"retry_class\":\"{}\",",
            "\"exit_class\":\"{}\",",
            "\"remediation_hint\":\"{}\",",
            "\"raw_internal_error_exposed\":{}",
            "}}"
        ),
        json_escape(&record.source_family),
        json_escape(&record.reason_code),
        json_escape(record.retry_class.as_str()),
        json_escape(record.exit_class.as_str()),
        json_escape(&record.remediation_hint),
        record.raw_internal_error_exposed,
    )
}

fn render_lifecycle_json(states: &[&str]) -> String {
    let terminal_state = states.iter().rev().copied().find(|state| {
        matches!(
            *state,
            "completed" | "denied" | "failed" | "passed" | "blocked"
        )
    });
    format!(
        concat!("{{", "\"states\":{},", "\"terminal_state\":{}", "}}"),
        json_string_array(states),
        json_optional_string(terminal_state),
    )
}

fn render_diagnostic_bundle_json(
    command_name: &str,
    profile_name: Option<&str>,
    endpoint_fingerprint: Option<&str>,
    trace_id: Option<&str>,
    reason_code: Option<&str>,
    dependency_status: &[&str],
) -> String {
    let trace_ids = trace_id.map(|value| vec![value]).unwrap_or_default();
    let reason_codes = reason_code.map(|value| vec![value]).unwrap_or_default();
    format!(
        concat!(
            "{{",
            "\"command_name\":\"{}\",",
            "\"profile_name\":{},",
            "\"endpoint_fingerprint\":{},",
            "\"schema_versions\":[\"{}\"],",
            "\"trace_ids\":{},",
            "\"reason_codes\":{},",
            "\"retry_count\":0,",
            "\"dependency_status\":{},",
            "\"redaction_policy\":\"secret_free_refs_only\"",
            "}}"
        ),
        json_escape(command_name),
        json_optional_string(profile_name),
        json_optional_string(endpoint_fingerprint),
        json_escape(SUPPORTED_SCHEMA_VERSION),
        json_string_array(&trace_ids),
        json_string_array(&reason_codes),
        json_string_array(dependency_status),
    )
}

fn render_capabilities_json(capabilities: &[CapabilityRoute<'_>]) -> String {
    let fail_closed = capabilities.iter().any(|capability| !capability.available);
    let routes = capabilities
        .iter()
        .map(|capability| {
            format!(
                concat!(
                    "{{",
                    "\"route\":\"{}\",",
                    "\"available\":{},",
                    "\"phase_gate\":\"{}\",",
                    "\"schema_versions\":[\"{}\"]",
                    "}}"
                ),
                json_escape(capability.route),
                capability.available,
                json_escape(capability.phase_gate),
                json_escape(SUPPORTED_SCHEMA_VERSION),
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "{{",
            "\"source\":\"local_capability_cache\",",
            "\"stale_age_ms\":0,",
            "\"fail_closed\":{},",
            "\"routes\":[{}]",
            "}}"
        ),
        fail_closed, routes
    )
}

fn render_profile_list(globals: &GlobalOptions) -> String {
    match globals.output {
        OutputMode::Human => "profile_storage_policy: owner_only_file_backed_config".to_owned(),
        OutputMode::Json => render_success_json(
            "profile list",
            concat!(
                "{",
                "\"profile_count\":0,",
                "\"storage_policy\":\"owner_only_file_backed_config\",",
                "\"commands\":[\"profile create\",\"profile list\",\"profile select\",\"profile inspect\",\"profile reset\"]",
                "}"
            ),
            &["parsed", "completed"],
            globals.profile.as_deref(),
            globals.endpoint_fingerprint.as_deref(),
            &["local_profile_store_available", "owner_only_storage_policy"],
            &[CapabilityRoute {
                route: "profile",
                phase_gate: "phase_4",
                available: true,
            }],
        ),
    }
}

fn render_profile_reset(globals: &GlobalOptions) -> String {
    match globals.output {
        OutputMode::Human => "profile_reset_ready: owner_only_file_backed_config".to_owned(),
        OutputMode::Json => render_success_json(
            "profile reset",
            concat!(
                "{",
                "\"profile_reset_ready\":true,",
                "\"storage_policy\":\"owner_only_file_backed_config\"",
                "}"
            ),
            &["parsed", "profile_loaded", "completed"],
            globals.profile.as_deref(),
            globals.endpoint_fingerprint.as_deref(),
            &["local_profile_store_available", "owner_only_storage_policy"],
            &[CapabilityRoute {
                route: "profile",
                phase_gate: "phase_4",
                available: true,
            }],
        ),
    }
}

fn render_profile_result(
    command: ProfileCommand,
    profile: &CliProfile,
    output: OutputMode,
) -> String {
    match output {
        OutputMode::Human => format!(
            "profile_ready: {} {} {}",
            profile.name,
            profile.environment.as_str(),
            profile.endpoint_fingerprint
        ),
        OutputMode::Json => {
            let classes = profile
                .allowed_credential_classes
                .iter()
                .map(|class| class.as_str())
                .collect::<Vec<_>>();
            let result_json = format!(
                concat!(
                    "{{",
                    "\"command\":\"{}\",",
                    "\"profile_name\":\"{}\",",
                    "\"environment_class\":\"{}\",",
                    "\"endpoint\":\"{}\",",
                    "\"endpoint_fingerprint\":\"{}\",",
                    "\"tenant_id\":\"{}\",",
                    "\"actor_id\":\"{}\",",
                    "\"credential_namespace\":\"{}\",",
                    "\"allowed_credential_classes\":{},",
                    "\"fixture_allowance\":\"{}\",",
                    "\"confirmation_policy\":\"{}\",",
                    "\"schema_pins\":{},",
                    "\"test_harness_profile\":{},",
                    "\"storage_policy\":\"owner_only_file_backed_config\"",
                    "}}"
                ),
                json_escape(command.as_str()),
                json_escape(&profile.name),
                json_escape(profile.environment.as_str()),
                json_escape(&profile.endpoint),
                json_escape(&profile.endpoint_fingerprint),
                json_escape(&profile.tenant_id),
                json_escape(&profile.actor_id),
                json_escape(&profile.credential_namespace),
                json_string_array(&classes),
                json_escape(profile.fixture_allowance.as_str()),
                json_escape(profile.confirmation_policy.as_str()),
                json_owned_string_array(&profile.schema_pins),
                profile.test_harness_profile,
            );
            render_success_json(
                command.as_str(),
                &result_json,
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "completed",
                ],
                Some(&profile.name),
                Some(&profile.endpoint_fingerprint),
                &[
                    "local_profile_store_available",
                    "credential_reference_checked",
                ],
                &[CapabilityRoute {
                    route: "profile",
                    phase_gate: "phase_4",
                    available: true,
                }],
            )
        }
    }
}

fn render_credential_result(
    command: CredentialCommand,
    profile: &CliProfile,
    credential: &CredentialReference,
    signature_ref: Option<&str>,
    output: OutputMode,
) -> String {
    match output {
        OutputMode::Human => format!(
            "credential_ready: {} {} {}",
            credential.reference_id,
            credential.class.as_str(),
            profile.environment.as_str()
        ),
        OutputMode::Json => {
            let result_json = format!(
                concat!(
                    "{{",
                    "\"command\":\"{}\",",
                    "\"profile_name\":\"{}\",",
                    "\"environment_class\":\"{}\",",
                    "\"credential_reference_id\":\"{}\",",
                    "\"credential_class\":\"{}\",",
                    "\"credential_namespace\":\"{}\",",
                    "\"key_id\":\"{}\",",
                    "\"revoked\":{},",
                    "\"expired\":{},",
                    "\"signature_ref\":{},",
                    "\"exposes_key_material\":false",
                    "}}"
                ),
                json_escape(command.as_str()),
                json_escape(&profile.name),
                json_escape(profile.environment.as_str()),
                json_escape(&credential.reference_id),
                json_escape(credential.class.as_str()),
                json_escape(&credential.namespace),
                json_escape(&credential.key_id),
                credential.revoked,
                credential.expired,
                json_optional_string(signature_ref),
            );
            let lifecycle = if matches!(command, CredentialCommand::Enroll) {
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "signed",
                    "completed",
                ][..]
            } else {
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "completed",
                ][..]
            };
            render_success_json(
                command.as_str(),
                &result_json,
                lifecycle,
                Some(&profile.name),
                Some(&profile.endpoint_fingerprint),
                &["credential_reference_checked", "signer_handoff_refs_only"],
                &[CapabilityRoute {
                    route: "credential",
                    phase_gate: "phase_4",
                    available: true,
                }],
            )
        }
    }
}

fn render_idempotency_cache_result(
    command: IdempotencyCacheCommand,
    profile: &CliProfile,
    cache_record: &LocalIdempotencyCacheRecord,
    output: OutputMode,
) -> String {
    match output {
        OutputMode::Human => format!(
            "idempotency_cache_{}: {} {} private_payload=false",
            match command {
                IdempotencyCacheCommand::Inspect => "inspect",
                IdempotencyCacheCommand::Reset => "reset",
            },
            profile.name,
            profile.environment.as_str()
        ),
        OutputMode::Json => {
            let result_json = format!(
                concat!(
                    "{{",
                    "\"command\":\"{}\",",
                    "\"cache_action\":\"{}\",",
                    "\"cache_status\":\"{}\",",
                    "\"local_idempotency_cache\":{}",
                    "}}"
                ),
                json_escape(command.as_str()),
                match command {
                    IdempotencyCacheCommand::Inspect => "inspect",
                    IdempotencyCacheCommand::Reset => "reset",
                },
                match command {
                    IdempotencyCacheCommand::Inspect => "empty",
                    IdempotencyCacheCommand::Reset => "reset",
                },
                render_local_idempotency_cache_json(cache_record),
            );
            render_success_json(
                command.as_str(),
                &result_json,
                &["parsed", "profile_loaded", "payload_validated", "completed"],
                Some(&profile.name),
                Some(&profile.endpoint_fingerprint),
                &[
                    "local_idempotency_cache_available",
                    "owner_only_storage_policy",
                    "secret_free_refs_only",
                ],
                &[CapabilityRoute {
                    route: "idempotency-cache",
                    phase_gate: "phase_6",
                    available: true,
                }],
            )
        }
    }
}

fn render_bootstrap_result(
    family: BootstrapCommandFamily,
    command_name: &str,
    payload_type: &str,
    result_kind: BootstrapResultKind,
    globals: &GlobalOptions,
    context: &BootstrapContext,
) -> String {
    match globals.output {
        OutputMode::Human => format!(
            "bootstrap_ready: {} {} trace_id={} target_ref={}",
            command_name,
            family.as_str(),
            context.trace_id,
            context.target_ref
        ),
        OutputMode::Json => {
            let acceptance = context.signed_envelope.as_ref().map(|_| {
                BootstrapAcceptanceRecord::new(
                    command_name,
                    format!("accepted:{}:{}", family.as_str(), context.target_ref),
                    if matches!(result_kind, BootstrapResultKind::WorkloadSubmit) {
                        "pending"
                    } else {
                        "accepted"
                    },
                    context.audit_refs.clone(),
                )
            });
            let lifecycle = if context.signed_envelope.is_some() {
                if matches!(result_kind, BootstrapResultKind::WorkloadSubmit) {
                    &[
                        "parsed",
                        "profile_loaded",
                        "credential_ready",
                        "payload_validated",
                        "signed",
                        "submitted",
                        "accepted",
                        "waiting",
                        "completed",
                    ][..]
                } else {
                    &[
                        "parsed",
                        "profile_loaded",
                        "credential_ready",
                        "payload_validated",
                        "signed",
                        "submitted",
                        "accepted",
                        "completed",
                    ][..]
                }
            } else {
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "completed",
                ][..]
            };
            let signed = context.signed_envelope.is_some();
            let result_json = format!(
                concat!(
                    "{{",
                    "\"command\":\"{}\",",
                    "\"family\":\"{}\",",
                    "\"phase_gate\":\"{}\",",
                    "\"sdk_target\":\"overgate_only\",",
                    "\"profile_name\":\"{}\",",
                    "\"environment_class\":\"{}\",",
                    "\"tenant_id\":\"{}\",",
                    "\"actor_id\":\"{}\",",
                    "\"target_ref\":\"{}\",",
                    "\"payload_type\":\"{}\",",
                    "\"idempotency_key\":\"{}\",",
                    "\"idempotency_key_source\":\"{}\",",
                    "\"trace_id\":\"{}\",",
                    "\"signed\":{},",
                    "\"signature_ref\":{},",
                    "\"expected_state\":{},",
                    "\"reason\":{},",
                    "\"dry_run\":{},",
                    "\"submitted_via\":\"sdk_overgate_contract\",",
                    "\"canonical_idempotency_fingerprint\":{},",
                    "\"retry_timeout_policy\":{},",
                    "\"local_idempotency_cache\":{},",
                    "\"signed_command_envelope\":{},",
                    "\"acceptance\":{}{}",
                    "}}"
                ),
                json_escape(command_name),
                json_escape(family.as_str()),
                json_escape(family.phase_gate()),
                json_escape(&context.profile.name),
                json_escape(context.profile.environment.as_str()),
                json_escape(&context.profile.tenant_id),
                json_escape(&context.profile.actor_id),
                json_escape(&context.target_ref),
                json_escape(payload_type),
                json_escape(&context.idempotency_key),
                json_escape(&context.idempotency_key_source),
                json_escape(&context.trace_id),
                signed,
                json_optional_string(
                    context
                        .signed_envelope
                        .as_ref()
                        .map(|envelope| envelope.signature_ref.as_str()),
                ),
                json_optional_string(globals.expected_state.as_deref()),
                json_optional_string(globals.reason.as_deref()),
                globals.dry_run,
                render_canonical_fingerprint_json(&context.fingerprint),
                render_retry_timeout_policy_json(&context.retry_policy),
                render_local_idempotency_cache_json(&context.cache_record),
                render_signed_command_envelope_json(context.signed_envelope.as_ref()),
                render_acceptance_json(acceptance.as_ref()),
                render_bootstrap_extra_json(result_kind, globals, context),
            );
            render_success_json_with_trace(
                command_name,
                &result_json,
                lifecycle,
                Some(&context.profile.name),
                Some(&context.profile.endpoint_fingerprint),
                &[
                    "phase1_bootstrap_contracts_available",
                    "sdk_overgate_contract_available",
                    "credential_reference_checked",
                ],
                &[CapabilityRoute {
                    route: family.as_str(),
                    phase_gate: family.phase_gate(),
                    available: true,
                }],
                Some(&context.trace_id),
                &context.audit_refs,
            )
        }
    }
}

fn render_node_result(
    command: NodeCommand,
    globals: &GlobalOptions,
    context: &BootstrapContext,
) -> String {
    let state = node_state_for_ref(&context.target_ref, command);
    let status = NodeStatusRecord::new(
        context.target_ref.clone(),
        state,
        context.profile.name.clone(),
        context.credential.reference_id.clone(),
    );

    match globals.output {
        OutputMode::Human => format!(
            "node_status: {} {} trace_id={}",
            status.node_ref,
            status.state.as_str(),
            context.trace_id
        ),
        OutputMode::Json => {
            let signed = context.signed_envelope.is_some();
            let result_json = format!(
                concat!(
                    "{{",
                    "\"command\":\"{}\",",
                    "\"family\":\"node\",",
                    "\"phase_gate\":\"phase_2_seed_private_swarm\",",
                    "\"sdk_target\":\"overgate_only\",",
                    "\"profile_name\":\"{}\",",
                    "\"target_ref\":\"{}\",",
                    "\"credential_ref\":\"{}\",",
                    "\"profile_scoped_credential_check\":true,",
                    "\"signed\":{},",
                    "\"signature_ref\":{},",
                    "\"submitted_via\":\"sdk_overgate_contract\",",
                    "\"node_status\":{},",
                    "\"capability_refs\":{},",
                    "\"signed_command_envelope\":{},",
                    "\"acceptance\":{}",
                    "}}"
                ),
                json_escape(command.as_str()),
                json_escape(&context.profile.name),
                json_escape(&context.target_ref),
                json_escape(&context.credential.reference_id),
                signed,
                json_optional_string(
                    context
                        .signed_envelope
                        .as_ref()
                        .map(|envelope| envelope.signature_ref.as_str()),
                ),
                render_node_status_json(&status),
                json_owned_string_array(&status.capability_refs),
                render_signed_command_envelope_json(context.signed_envelope.as_ref()),
                render_phase_acceptance_json(
                    command.as_str(),
                    &format!("accepted:node:{}", context.target_ref),
                    "phase_2_seed_private_swarm",
                    status.state.as_str(),
                    &context.audit_refs,
                    signed,
                ),
            );
            let lifecycle = if signed {
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "signed",
                    "submitted",
                    "accepted",
                    "completed",
                ][..]
            } else {
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "completed",
                ][..]
            };
            render_success_json_with_trace(
                command.as_str(),
                &result_json,
                lifecycle,
                Some(&context.profile.name),
                Some(&context.profile.endpoint_fingerprint),
                &[
                    "phase2_node_contracts_available",
                    "sdk_overgate_contract_available",
                    "profile_scoped_credential_check",
                    "node_capability_refs_only",
                ],
                &[CapabilityRoute {
                    route: "node",
                    phase_gate: "phase_2_seed_private_swarm",
                    available: true,
                }],
                Some(&context.trace_id),
                &context.audit_refs,
            )
        }
    }
}

fn render_workload_execution_result(
    command: WorkloadCommand,
    globals: &GlobalOptions,
    context: &BootstrapContext,
) -> String {
    let execution_state = workload_execution_state_for_command(command, &context.target_ref);
    let states = workload_execution_states(execution_state);
    let timeline =
        ExecutionTimeline::new(context.target_ref.clone(), states, context.trace_id.clone());
    let log_bundle = matches!(command, WorkloadCommand::Logs | WorkloadCommand::Follow)
        .then(|| ExecutionLogBundle::new(context.target_ref.clone(), context.trace_id.clone()));
    let result_ref = matches!(command, WorkloadCommand::Result | WorkloadCommand::Follow)
        .then(|| ExecutionResultRef::new(context.target_ref.clone(), context.trace_id.clone()));
    let polling = polling_plan_for(command, globals);

    match globals.output {
        OutputMode::Human => format!(
            "workload_execution: {} {} trace_id={}",
            command.as_str(),
            execution_state.as_str(),
            context.trace_id
        ),
        OutputMode::Json => {
            let signed = context.signed_envelope.is_some();
            let result_json = format!(
                concat!(
                    "{{",
                    "\"command\":\"{}\",",
                    "\"family\":\"workload_execution\",",
                    "\"phase_gate\":\"phase_3_private_execution_loop\",",
                    "\"sdk_target\":\"overgate_only\",",
                    "\"profile_name\":\"{}\",",
                    "\"workload_ref\":\"{}\",",
                    "\"execution_state\":\"{}\",",
                    "\"signed\":{},",
                    "\"signature_ref\":{},",
                    "\"submitted_via\":\"sdk_overgate_contract\",",
                    "\"polling_plan\":{},",
                    "\"execution_timeline\":{},",
                    "\"execution_logs\":{},",
                    "\"execution_result\":{},",
                    "\"signed_command_envelope\":{},",
                    "\"acceptance\":{}{}{}",
                    "}}"
                ),
                json_escape(command.as_str()),
                json_escape(&context.profile.name),
                json_escape(&context.target_ref),
                json_escape(execution_state.as_str()),
                signed,
                json_optional_string(
                    context
                        .signed_envelope
                        .as_ref()
                        .map(|envelope| envelope.signature_ref.as_str()),
                ),
                render_polling_plan_json(&polling),
                render_execution_timeline_json(&timeline),
                log_bundle
                    .as_ref()
                    .map(render_execution_log_bundle_json)
                    .unwrap_or_else(|| "null".to_owned()),
                result_ref
                    .as_ref()
                    .map(render_execution_result_ref_json)
                    .unwrap_or_else(|| "null".to_owned()),
                render_signed_command_envelope_json(context.signed_envelope.as_ref()),
                render_phase_acceptance_json(
                    command.as_str(),
                    &format!("accepted:workload_execution:{}", context.target_ref),
                    "phase_3_private_execution_loop",
                    execution_state.as_str(),
                    &context.audit_refs,
                    signed,
                ),
                render_product_workflow_recipe_optional_json(globals, context),
                render_ci_automation_profile_optional_json(context),
            );
            let lifecycle = if signed {
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "signed",
                    "submitted",
                    "accepted",
                    "completed",
                ][..]
            } else if polling.wait || polling.follow {
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "submitted",
                    "waiting",
                    "completed",
                ][..]
            } else {
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "completed",
                ][..]
            };
            render_success_json_with_trace(
                command.as_str(),
                &result_json,
                lifecycle,
                Some(&context.profile.name),
                Some(&context.profile.endpoint_fingerprint),
                &[
                    "overgate_execution_contract_available",
                    "overqueue_state_ref",
                    "oversched_scheduler_ref",
                    "overlease_lease_ref",
                    "overrun_runner_ref",
                    "overcell_node_heartbeat_ref",
                    "overpack_package_ref",
                    "overstore_result_state_ref",
                    "overwatch_trace_ref",
                ],
                &[CapabilityRoute {
                    route: "workload_execution",
                    phase_gate: "phase_3_private_execution_loop",
                    available: true,
                }],
                Some(&context.trace_id),
                &context.audit_refs,
            )
        }
    }
}

fn render_policy_dry_run_result(
    command: PolicyCommand,
    context: &BootstrapContext,
    globals: &GlobalOptions,
) -> String {
    let decision = PolicyDryRunDecision::new(
        context.target_ref.clone(),
        policy_reason_codes(&context.target_ref),
    );
    match globals.output {
        OutputMode::Human => format!(
            "policy_dry_run: {} {} trace_id={}",
            context.target_ref,
            decision.decision.as_str(),
            context.trace_id
        ),
        OutputMode::Json => {
            let result_json = format!(
                concat!(
                    "{{",
                    "\"command\":\"{}\",",
                    "\"family\":\"policy\",",
                    "\"phase_gate\":\"phase_4_trust_policy_verification\",",
                    "\"sdk_target\":\"overgate_only\",",
                    "\"profile_name\":\"{}\",",
                    "\"policy_dry_run_decision\":{}",
                    "}}"
                ),
                json_escape(command.as_str()),
                json_escape(&context.profile.name),
                render_policy_dry_run_decision_json(&decision),
            );
            render_success_json_with_trace(
                command.as_str(),
                &result_json,
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "completed",
                ],
                Some(&context.profile.name),
                Some(&context.profile.endpoint_fingerprint),
                &[
                    "overguard_policy_dry_run_ref",
                    "quota_ref",
                    "package_trust_ref",
                    "egress_policy_ref",
                    "provider_eligibility_ref",
                    "budget_placeholder_ref",
                ],
                &[CapabilityRoute {
                    route: "policy dry-run",
                    phase_gate: "phase_4_trust_policy_verification",
                    available: true,
                }],
                Some(&context.trace_id),
                &[],
            )
        }
    }
}

fn render_package_validation_result(
    command: PackageCommand,
    context: &BootstrapContext,
    globals: &GlobalOptions,
) -> String {
    let validation_state = package_validation_state_for_ref(&context.target_ref);
    let summary = PackageValidationSummary::new(
        context.target_ref.clone(),
        validation_state,
        package_reason_codes(validation_state),
    );
    match globals.output {
        OutputMode::Human => format!(
            "package_validation: {} {} trace_id={}",
            context.target_ref,
            summary.validation_state.as_str(),
            context.trace_id
        ),
        OutputMode::Json => {
            let result_json = format!(
                concat!(
                    "{{",
                    "\"command\":\"{}\",",
                    "\"family\":\"package\",",
                    "\"phase_gate\":\"phase_9_overpack_deployment_platform\",",
                    "\"sdk_target\":\"overgate_only\",",
                    "\"profile_name\":\"{}\",",
                    "\"package_validation_summary\":{}",
                    "}}"
                ),
                json_escape(command.as_str()),
                json_escape(&context.profile.name),
                render_package_validation_summary_json(&summary),
            );
            render_success_json_with_trace(
                command.as_str(),
                &result_json,
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "completed",
                ],
                Some(&context.profile.name),
                Some(&context.profile.endpoint_fingerprint),
                &[
                    "package_validator_contract_ref",
                    "schema_signature_hash_checks",
                    "sbom_provenance_ref",
                    "policy_compatibility_ref",
                ],
                &[CapabilityRoute {
                    route: "package validate",
                    phase_gate: "phase_9_overpack_deployment_platform",
                    available: true,
                }],
                Some(&context.trace_id),
                &[],
            )
        }
    }
}

fn render_usage_result(
    command: UsageCommand,
    context: &BootstrapContext,
    globals: &GlobalOptions,
) -> String {
    let rollup = UsageOruRollup::new(
        context.profile.tenant_id.clone(),
        context.target_ref.clone(),
    );
    match globals.output {
        OutputMode::Human => format!(
            "usage_oru_rollup: {} {} trace_id={}",
            rollup.usage_ref, rollup.budget_state, context.trace_id
        ),
        OutputMode::Json => {
            let result_json = format!(
                concat!(
                    "{{",
                    "\"command\":\"{}\",",
                    "\"family\":\"usage\",",
                    "\"phase_gate\":\"phase_5_metering_accounting\",",
                    "\"sdk_target\":\"overgate_only\",",
                    "\"profile_name\":\"{}\",",
                    "\"usage_oru_rollup\":{}",
                    "}}"
                ),
                json_escape(command.as_str()),
                json_escape(&context.profile.name),
                render_usage_oru_rollup_json(&rollup),
            );
            render_success_json_with_trace(
                command.as_str(),
                &result_json,
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "completed",
                ],
                Some(&context.profile.name),
                Some(&context.profile.endpoint_fingerprint),
                &[
                    "overmeter_rollup_ref",
                    "oru_balance_ref",
                    "reservation_hold_ref",
                    "budget_status_ref",
                ],
                &[CapabilityRoute {
                    route: "usage show",
                    phase_gate: "phase_5_metering_accounting",
                    available: true,
                }],
                Some(&context.trace_id),
                &[],
            )
        }
    }
}

fn render_receipt_ledger_result(
    command_name: &str,
    context: &BootstrapContext,
    globals: &GlobalOptions,
) -> String {
    let receipt = ReceiptLedgerRead::new(context.target_ref.clone());
    match globals.output {
        OutputMode::Human => format!(
            "receipt_ledger_read: {} {} trace_id={}",
            command_name, receipt.receipt_ref, context.trace_id
        ),
        OutputMode::Json => {
            let result_json = format!(
                concat!(
                    "{{",
                    "\"command\":\"{}\",",
                    "\"family\":\"receipt_ledger\",",
                    "\"phase_gate\":\"phase_5_metering_accounting\",",
                    "\"sdk_target\":\"overgate_only\",",
                    "\"profile_name\":\"{}\",",
                    "\"receipt_ledger_read\":{}",
                    "}}"
                ),
                json_escape(command_name),
                json_escape(&context.profile.name),
                render_receipt_ledger_read_json(&receipt),
            );
            render_success_json_with_trace(
                command_name,
                &result_json,
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "completed",
                ],
                Some(&context.profile.name),
                Some(&context.profile.endpoint_fingerprint),
                &[
                    "seal_ledger_ref",
                    "overbill_receipt_ref",
                    "refund_correction_ref",
                    "payout_hold_ref",
                    "audit_ref",
                ],
                &[CapabilityRoute {
                    route: command_name,
                    phase_gate: "phase_5_metering_accounting",
                    available: true,
                }],
                Some(&context.trace_id),
                &receipt.audit_refs,
            )
        }
    }
}

fn render_dispute_result(
    command: DisputeCommand,
    context: &BootstrapContext,
    globals: &GlobalOptions,
) -> String {
    let dispute = DisputeReadModel::new(context.target_ref.clone());
    match globals.output {
        OutputMode::Human => format!(
            "dispute_read: {} {} trace_id={}",
            command.as_str(),
            dispute.resolution_state,
            context.trace_id
        ),
        OutputMode::Json => {
            let result_json = format!(
                concat!(
                    "{{",
                    "\"command\":\"{}\",",
                    "\"family\":\"dispute\",",
                    "\"phase_gate\":\"phase_5_metering_accounting\",",
                    "\"sdk_target\":\"overgate_only\",",
                    "\"profile_name\":\"{}\",",
                    "\"dispute_read_model\":{}",
                    "}}"
                ),
                json_escape(command.as_str()),
                json_escape(&context.profile.name),
                render_dispute_read_model_json(&dispute),
            );
            render_success_json_with_trace(
                command.as_str(),
                &result_json,
                &[
                    "parsed",
                    "profile_loaded",
                    "credential_ready",
                    "payload_validated",
                    "completed",
                ],
                Some(&context.profile.name),
                Some(&context.profile.endpoint_fingerprint),
                &[
                    "overclaim_case_ref",
                    "evidence_ref",
                    "hold_status_ref",
                    "correction_ref",
                    "tenant_role_filter",
                ],
                &[CapabilityRoute {
                    route: command.as_str(),
                    phase_gate: "phase_5_metering_accounting",
                    available: true,
                }],
                Some(&context.trace_id),
                &[],
            )
        }
    }
}

fn render_phase_acceptance_json(
    command_type: &str,
    accepted_ref: &str,
    phase_gate: &str,
    pending_state: &str,
    audit_refs: &[String],
    present: bool,
) -> String {
    if !present {
        return "null".to_owned();
    }
    format!(
        concat!(
            "{{",
            "\"command_type\":\"{}\",",
            "\"accepted_ref\":\"{}\",",
            "\"phase_gate\":\"{}\",",
            "\"pending_state\":\"{}\",",
            "\"audit_refs\":{}",
            "}}"
        ),
        json_escape(command_type),
        json_escape(accepted_ref),
        json_escape(phase_gate),
        json_escape(pending_state),
        json_owned_string_array(audit_refs),
    )
}

fn render_node_status_json(record: &NodeStatusRecord) -> String {
    format!(
        concat!(
            "{{",
            "\"node_ref\":\"{}\",",
            "\"state\":\"{}\",",
            "\"profile_name\":\"{}\",",
            "\"credential_ref\":\"{}\",",
            "\"credential_checked\":{},",
            "\"capability_refs\":{},",
            "\"heartbeat_ref\":\"{}\",",
            "\"registered_via\":\"{}\",",
            "\"direct_node_access\":{}",
            "}}"
        ),
        json_escape(&record.node_ref),
        json_escape(record.state.as_str()),
        json_escape(&record.profile_name),
        json_escape(&record.credential_ref),
        record.credential_checked,
        json_owned_string_array(&record.capability_refs),
        json_escape(&record.heartbeat_ref),
        json_escape(&record.registered_via),
        record.direct_node_access,
    )
}

fn render_execution_timeline_json(timeline: &ExecutionTimeline) -> String {
    let states = timeline
        .states
        .iter()
        .map(|state| state.as_str().to_owned())
        .collect::<Vec<_>>();
    let diagnostics = timeline
        .diagnostic_events
        .iter()
        .map(render_execution_diagnostic_event_json)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "{{",
            "\"workload_ref\":\"{}\",",
            "\"states\":{},",
            "\"diagnostic_events\":[{}],",
            "\"owning_service_refs\":{},",
            "\"trace_id\":\"{}\",",
            "\"direct_node_access\":{}",
            "}}"
        ),
        json_escape(&timeline.workload_ref),
        json_owned_string_array(&states),
        diagnostics,
        json_owned_string_array(&timeline.owning_service_refs),
        json_escape(&timeline.trace_id),
        timeline.direct_node_access,
    )
}

fn render_execution_diagnostic_event_json(event: &ExecutionDiagnosticEvent) -> String {
    format!(
        concat!(
            "{{",
            "\"state\":\"{}\",",
            "\"service_ref\":\"{}\",",
            "\"reason_code\":\"{}\",",
            "\"evidence_ref\":\"{}\"",
            "}}"
        ),
        json_escape(event.state.as_str()),
        json_escape(&event.service_ref),
        json_escape(&event.reason_code),
        json_escape(&event.evidence_ref),
    )
}

fn render_execution_log_bundle_json(bundle: &ExecutionLogBundle) -> String {
    format!(
        concat!(
            "{{",
            "\"workload_ref\":\"{}\",",
            "\"log_ref\":\"{}\",",
            "\"redaction_policy\":\"{}\",",
            "\"bounded_streaming\":{},",
            "\"trace_linked_ref\":\"{}\",",
            "\"contains_private_payload\":{},",
            "\"direct_node_path_exposed\":{}",
            "}}"
        ),
        json_escape(&bundle.workload_ref),
        json_escape(&bundle.log_ref),
        json_escape(&bundle.redaction_policy),
        bundle.bounded_streaming,
        json_escape(&bundle.trace_linked_ref),
        bundle.contains_private_payload,
        bundle.direct_node_path_exposed,
    )
}

fn render_execution_result_ref_json(result: &ExecutionResultRef) -> String {
    format!(
        concat!(
            "{{",
            "\"workload_ref\":\"{}\",",
            "\"result_ref\":\"{}\",",
            "\"authorized_control_plane_ref\":\"{}\",",
            "\"trace_linked_ref\":\"{}\",",
            "\"contains_private_payload\":{},",
            "\"direct_object_store_path_exposed\":{}",
            "}}"
        ),
        json_escape(&result.workload_ref),
        json_escape(&result.result_ref),
        json_escape(&result.authorized_control_plane_ref),
        json_escape(&result.trace_linked_ref),
        result.contains_private_payload,
        result.direct_object_store_path_exposed,
    )
}

fn render_polling_plan_json(plan: &PollingPlan) -> String {
    format!(
        concat!(
            "{{",
            "\"wait\":{},",
            "\"follow\":{},",
            "\"timeout_ms\":{},",
            "\"poll_interval_ms\":{},",
            "\"event_stream_preferred\":{},",
            "\"fallback_polling\":{},",
            "\"interruptible\":{}",
            "}}"
        ),
        plan.wait,
        plan.follow,
        plan.timeout_ms,
        plan.poll_interval_ms,
        plan.event_stream_preferred,
        plan.fallback_polling,
        plan.interruptible,
    )
}

fn render_policy_dry_run_decision_json(decision: &PolicyDryRunDecision) -> String {
    format!(
        concat!(
            "{{",
            "\"target_ref\":\"{}\",",
            "\"decision\":\"{}\",",
            "\"reason_codes\":{},",
            "\"workload_class\":\"{}\",",
            "\"data_sensitivity\":\"{}\",",
            "\"quota_ref\":\"{}\",",
            "\"package_trust_ref\":\"{}\",",
            "\"egress_policy_ref\":\"{}\",",
            "\"provider_eligibility_ref\":\"{}\",",
            "\"budget_ref\":\"{}\",",
            "\"evaluated_via\":\"{}\",",
            "\"mutates_platform_state\":{},",
            "\"direct_policy_service_access\":{}",
            "}}"
        ),
        json_escape(&decision.target_ref),
        json_escape(decision.decision.as_str()),
        json_owned_string_array(&decision.reason_codes),
        json_escape(&decision.workload_class),
        json_escape(&decision.data_sensitivity),
        json_escape(&decision.quota_ref),
        json_escape(&decision.package_trust_ref),
        json_escape(&decision.egress_policy_ref),
        json_escape(&decision.provider_eligibility_ref),
        json_escape(&decision.budget_ref),
        json_escape(&decision.evaluated_via),
        decision.mutates_platform_state,
        decision.direct_policy_service_access,
    )
}

fn render_package_validation_summary_json(summary: &PackageValidationSummary) -> String {
    format!(
        concat!(
            "{{",
            "\"package_ref\":\"{}\",",
            "\"validation_state\":\"{}\",",
            "\"reason_codes\":{},",
            "\"schema_checked\":{},",
            "\"signature_checked\":{},",
            "\"hash_checked\":{},",
            "\"dependency_checked\":{},",
            "\"permission_checked\":{},",
            "\"provenance_available\":{},",
            "\"sbom_ref\":\"{}\",",
            "\"policy_compatibility_checked\":{},",
            "\"submitted_via\":\"{}\",",
            "\"direct_package_store_access\":{}",
            "}}"
        ),
        json_escape(&summary.package_ref),
        json_escape(summary.validation_state.as_str()),
        json_owned_string_array(&summary.reason_codes),
        summary.schema_checked,
        summary.signature_checked,
        summary.hash_checked,
        summary.dependency_checked,
        summary.permission_checked,
        summary.provenance_available,
        json_escape(&summary.sbom_ref),
        summary.policy_compatibility_checked,
        json_escape(&summary.submitted_via),
        summary.direct_package_store_access,
    )
}

fn render_usage_oru_rollup_json(rollup: &UsageOruRollup) -> String {
    format!(
        concat!(
            "{{",
            "\"tenant_ref\":\"{}\",",
            "\"usage_ref\":\"{}\",",
            "\"units\":{},",
            "\"budget_state\":\"{}\",",
            "\"disputed_usage\":{},",
            "\"read_via\":\"{}\",",
            "\"payment_behavior_created\":{},",
            "\"direct_meter_access\":{}",
            "}}"
        ),
        json_escape(&rollup.tenant_ref),
        json_escape(&rollup.usage_ref),
        json_owned_string_array(&rollup.units),
        json_escape(&rollup.budget_state),
        rollup.disputed_usage,
        json_escape(&rollup.read_via),
        rollup.payment_behavior_created,
        rollup.direct_meter_access,
    )
}

fn render_receipt_ledger_read_json(read: &ReceiptLedgerRead) -> String {
    format!(
        concat!(
            "{{",
            "\"receipt_ref\":\"{}\",",
            "\"ledger_refs\":{},",
            "\"invoice_status\":\"{}\",",
            "\"refund_ref\":\"{}\",",
            "\"correction_ref\":\"{}\",",
            "\"payout_hold_ref\":\"{}\",",
            "\"audit_refs\":{},",
            "\"read_via\":\"{}\",",
            "\"pricing_assumptions_present\":{},",
            "\"revenue_assumptions_present\":{},",
            "\"customer_count_assumptions_present\":{},",
            "\"market_volume_assumptions_present\":{},",
            "\"direct_ledger_access\":{}",
            "}}"
        ),
        json_escape(&read.receipt_ref),
        json_owned_string_array(&read.ledger_refs),
        json_escape(&read.invoice_status),
        json_escape(&read.refund_ref),
        json_escape(&read.correction_ref),
        json_escape(&read.payout_hold_ref),
        json_owned_string_array(&read.audit_refs),
        json_escape(&read.read_via),
        read.pricing_assumptions_present,
        read.revenue_assumptions_present,
        read.customer_count_assumptions_present,
        read.market_volume_assumptions_present,
        read.direct_ledger_access,
    )
}

fn render_dispute_read_model_json(dispute: &DisputeReadModel) -> String {
    format!(
        concat!(
            "{{",
            "\"dispute_ref\":\"{}\",",
            "\"case_refs\":{},",
            "\"evidence_refs\":{},",
            "\"hold_status\":\"{}\",",
            "\"correction_refs\":{},",
            "\"resolution_state\":\"{}\",",
            "\"tenant_role_filtered\":{},",
            "\"read_via\":\"{}\",",
            "\"direct_dispute_mutation\":{},",
            "\"direct_ledger_mutation\":{}",
            "}}"
        ),
        json_escape(&dispute.dispute_ref),
        json_owned_string_array(&dispute.case_refs),
        json_owned_string_array(&dispute.evidence_refs),
        json_escape(&dispute.hold_status),
        json_owned_string_array(&dispute.correction_refs),
        json_escape(&dispute.resolution_state),
        dispute.tenant_role_filtered,
        json_escape(&dispute.read_via),
        dispute.direct_dispute_mutation,
        dispute.direct_ledger_mutation,
    )
}

fn render_signed_command_envelope_json(envelope: Option<&SignedCommandEnvelope>) -> String {
    let Some(envelope) = envelope else {
        return "null".to_owned();
    };
    format!(
        concat!(
            "{{",
            "\"family\":\"{}\",",
            "\"command_type\":\"{}\",",
            "\"tenant_id\":\"{}\",",
            "\"actor_id\":\"{}\",",
            "\"target_ref\":\"{}\",",
            "\"payload_type\":\"{}\",",
            "\"expected_state\":{},",
            "\"reason\":{},",
            "\"idempotency_key\":\"{}\",",
            "\"trace_id\":\"{}\",",
            "\"signature_ref\":\"{}\",",
            "\"exposes_key_material\":false",
            "}}"
        ),
        json_escape(envelope.family.as_str()),
        json_escape(&envelope.command_type),
        json_escape(&envelope.tenant_id),
        json_escape(&envelope.actor_id),
        json_escape(&envelope.target_ref),
        json_escape(&envelope.payload_type),
        json_optional_string(envelope.expected_state.as_deref()),
        json_optional_string(envelope.reason.as_deref()),
        json_escape(&envelope.idempotency.key),
        json_escape(&envelope.trace_context.trace_id),
        json_escape(&envelope.signature_ref),
    )
}

fn render_acceptance_json(record: Option<&BootstrapAcceptanceRecord>) -> String {
    let Some(record) = record else {
        return "null".to_owned();
    };
    format!(
        concat!(
            "{{",
            "\"command_type\":\"{}\",",
            "\"accepted_ref\":\"{}\",",
            "\"phase_gate\":\"{}\",",
            "\"pending_state\":\"{}\",",
            "\"audit_refs\":{}",
            "}}"
        ),
        json_escape(&record.command_type),
        json_escape(&record.accepted_ref),
        json_escape(&record.phase_gate),
        json_escape(&record.pending_state),
        json_owned_string_array(&record.audit_refs),
    )
}

fn render_canonical_fingerprint_json(fingerprint: &CanonicalIdempotencyFingerprint) -> String {
    format!(
        concat!(
            "{{",
            "\"environment_class\":\"{}\",",
            "\"endpoint_identity\":\"{}\",",
            "\"tenant_id\":\"{}\",",
            "\"actor_id\":\"{}\",",
            "\"command_type\":\"{}\",",
            "\"target_ref\":\"{}\",",
            "\"canonical_payload_hash\":\"{}\",",
            "\"expected_current_state\":{},",
            "\"reason\":{},",
            "\"schema_version\":\"{}\",",
            "\"fingerprint\":\"{}\"",
            "}}"
        ),
        json_escape(fingerprint.environment_class.as_str()),
        json_escape(&fingerprint.endpoint_identity),
        json_escape(&fingerprint.tenant_id),
        json_escape(&fingerprint.actor_id),
        json_escape(&fingerprint.command_type),
        json_escape(&fingerprint.target_ref),
        json_escape(&fingerprint.canonical_payload_hash),
        json_optional_string(fingerprint.expected_current_state.as_deref()),
        json_optional_string(fingerprint.reason.as_deref()),
        json_escape(fingerprint.schema_version.raw()),
        json_escape(&fingerprint.fingerprint),
    )
}

fn render_retry_timeout_policy_json(policy: &RetryTimeoutPolicy) -> String {
    let retryable_classes = policy
        .retryable_classes
        .iter()
        .map(|retry_class| retry_class.as_str().to_owned())
        .collect::<Vec<_>>();
    format!(
        concat!(
            "{{",
            "\"max_retries\":{},",
            "\"timeout_ms\":{},",
            "\"bounded\":{},",
            "\"retryable_classes\":{},",
            "\"non_retryable_reason_families\":{}",
            "}}"
        ),
        policy.max_retries,
        policy.timeout_ms,
        policy.bounded,
        json_owned_string_array(&retryable_classes),
        json_owned_string_array(&policy.non_retryable_reason_families),
    )
}

fn render_local_idempotency_cache_json(record: &LocalIdempotencyCacheRecord) -> String {
    format!(
        concat!(
            "{{",
            "\"cache_scope\":\"{}\",",
            "\"profile_name\":\"{}\",",
            "\"environment_class\":\"{}\",",
            "\"command_fingerprint\":\"{}\",",
            "\"idempotency_key\":\"{}\",",
            "\"owner_only\":{},",
            "\"contains_private_payload\":{},",
            "\"resettable\":{},",
            "\"inspectable\":{}",
            "}}"
        ),
        json_escape(&record.cache_scope),
        json_escape(&record.profile_name),
        json_escape(record.environment_class.as_str()),
        json_escape(&record.command_fingerprint),
        json_escape(&record.idempotency_key),
        record.owner_only,
        record.contains_private_payload,
        record.resettable,
        record.inspectable,
    )
}

fn render_bootstrap_extra_json(
    result_kind: BootstrapResultKind,
    globals: &GlobalOptions,
    context: &BootstrapContext,
) -> String {
    match result_kind {
        BootstrapResultKind::AuthLogin => format!(
            concat!(
                ",\"auth_status\":\"session_ready\",",
                "\"session_ref\":\"session:{}\",",
                "\"actor_roles\":[\"operator\"],",
                "\"credential_key_id\":\"{}\",",
                "\"endpoint_fingerprint\":\"{}\""
            ),
            json_escape(&context.profile.actor_id),
            json_escape(&context.credential.key_id),
            json_escape(&context.profile.endpoint_fingerprint),
        ),
        BootstrapResultKind::AuthWhoami => format!(
            concat!(
                ",\"auth_status\":\"authenticated\",",
                "\"actor_roles\":[\"operator\"],",
                "\"credential_key_id\":\"{}\",",
                "\"endpoint_fingerprint\":\"{}\""
            ),
            json_escape(&context.credential.key_id),
            json_escape(&context.profile.endpoint_fingerprint),
        ),
        BootstrapResultKind::Tenant => {
            let tenant_state = if context
                .signed_envelope
                .as_ref()
                .is_some_and(|envelope| envelope.command_type == "tenant suspend")
            {
                "suspended"
            } else {
                "active"
            };
            format!(
                ",\"tenant_id\":\"{}\",\"tenant_state\":\"{}\",\"tenant_refs\":[\"{}\"]",
                json_escape(&context.profile.tenant_id),
                tenant_state,
                json_escape(&context.profile.tenant_id),
            )
        }
        BootstrapResultKind::Identity => {
            let identity_state = if context
                .signed_envelope
                .as_ref()
                .is_some_and(|envelope| envelope.command_type == "identity disable")
            {
                "disabled"
            } else {
                "active"
            };
            format!(
                ",\"identity_id\":\"{}\",\"identity_state\":\"{}\",\"identity_refs\":[\"{}\"]",
                json_escape(&context.profile.actor_id),
                identity_state,
                json_escape(&context.profile.actor_id),
            )
        }
        BootstrapResultKind::Key => {
            let key_state = context
                .signed_envelope
                .as_ref()
                .map(|envelope| match envelope.command_type.as_str() {
                    "key revoke" => "revoked",
                    "key rotate" => "rotated",
                    "key enroll" => "enrolled",
                    _ => "active",
                })
                .unwrap_or("active");
            format!(
                concat!(
                    ",\"key_id\":\"{}\",",
                    "\"key_state\":\"{}\",",
                    "\"credential_reference_id\":\"{}\",",
                    "\"exposes_key_material\":false"
                ),
                json_escape(&context.credential.key_id),
                key_state,
                json_escape(&context.credential.reference_id),
            )
        }
        BootstrapResultKind::ManifestValidate => {
            let manifest = manifest_ref(globals);
            format!(
                concat!(
                    ",\"manifest_ref\":\"{}\",",
                    "\"manifest_kind\":\"{}\",",
                    "\"local_validation\":\"accepted\""
                ),
                json_escape(&manifest.manifest_ref),
                json_escape(&manifest.manifest_kind),
            )
        }
        BootstrapResultKind::ManifestSubmit | BootstrapResultKind::ManifestInspect => {
            let manifest = manifest_ref(globals);
            format!(
                ",\"manifest_bootstrap_ref\":{}",
                render_manifest_ref_json(&manifest)
            )
        }
        BootstrapResultKind::WorkloadSubmit
        | BootstrapResultKind::WorkloadStatus
        | BootstrapResultKind::WorkloadTimeline => {
            let workload = workload_pending_state(globals);
            let event_suffix = if matches!(result_kind, BootstrapResultKind::WorkloadTimeline) {
                ",\"timeline_events\":[{\"state\":\"pending\",\"source\":\"overqueue_pending_ref\"}]"
            } else {
                ""
            };
            let execution_suffix = if matches!(
                result_kind,
                BootstrapResultKind::WorkloadStatus | BootstrapResultKind::WorkloadTimeline
            ) {
                let command = if matches!(result_kind, BootstrapResultKind::WorkloadStatus) {
                    WorkloadCommand::Status
                } else {
                    WorkloadCommand::Timeline
                };
                let execution_state =
                    workload_execution_state_for_command(command, &context.target_ref);
                let timeline = ExecutionTimeline::new(
                    context.target_ref.clone(),
                    workload_execution_states(execution_state),
                    context.trace_id.clone(),
                );
                let polling = polling_plan_for(command, globals);
                format!(
                    ",\"execution_state\":\"{}\",\"polling_plan\":{},\"execution_timeline\":{}",
                    json_escape(execution_state.as_str()),
                    render_polling_plan_json(&polling),
                    render_execution_timeline_json(&timeline),
                )
            } else {
                String::new()
            };
            format!(
                ",\"synthetic_workload_pending_state\":{}{}{}{}{}",
                render_workload_pending_json(&workload),
                event_suffix,
                execution_suffix,
                render_product_workflow_recipe_optional_json(globals, context),
                render_ci_automation_profile_optional_json(context),
            )
        }
    }
}

fn render_manifest_ref_json(manifest: &ManifestBootstrapRef) -> String {
    format!(
        concat!(
            "{{",
            "\"manifest_ref\":\"{}\",",
            "\"manifest_kind\":\"{}\",",
            "\"immutable_ref\":\"{}\",",
            "\"submitted_via\":\"{}\"",
            "}}"
        ),
        json_escape(&manifest.manifest_ref),
        json_escape(&manifest.manifest_kind),
        json_escape(&manifest.immutable_ref),
        json_escape(&manifest.submitted_via),
    )
}

fn render_workload_pending_json(workload: &SyntheticWorkloadPendingState) -> String {
    format!(
        concat!(
            "{{",
            "\"workload_ref\":\"{}\",",
            "\"workload_kind\":\"{}\",",
            "\"queue_state\":\"{}\",",
            "\"execution_implied\":{},",
            "\"timeline_refs\":{}",
            "}}"
        ),
        json_escape(&workload.workload_ref),
        json_escape(&workload.workload_kind),
        json_escape(&workload.queue_state),
        workload.execution_implied,
        json_owned_string_array(&workload.timeline_refs),
    )
}

fn render_product_workflow_recipe_optional_json(
    globals: &GlobalOptions,
    context: &BootstrapContext,
) -> String {
    product_workflow_recipe(globals, context)
        .map(|recipe| {
            format!(
                ",\"product_workflow_recipe\":{}",
                render_product_workflow_recipe_json(&recipe)
            )
        })
        .unwrap_or_default()
}

fn product_workflow_recipe(
    globals: &GlobalOptions,
    context: &BootstrapContext,
) -> Option<ProductWorkflowRecipe> {
    let product = product_kind_for_workload(globals, context)?;
    let workflow_ref = globals
        .workload_ref
        .as_deref()
        .unwrap_or(&context.target_ref);
    let workload_kind = globals
        .workload_kind
        .as_deref()
        .unwrap_or_else(|| default_product_workload_kind(product));
    Some(ProductWorkflowRecipe::new(
        product,
        workflow_ref,
        workload_kind,
    ))
}

fn product_kind_for_workload(
    globals: &GlobalOptions,
    context: &BootstrapContext,
) -> Option<ProductKind> {
    let normalized = format!(
        "{} {} {}",
        globals.workload_kind.as_deref().unwrap_or_default(),
        globals.workload_ref.as_deref().unwrap_or_default(),
        context.target_ref,
    )
    .to_ascii_lowercase()
    .replace('-', "_");

    if normalized.contains("docdex")
        || normalized.contains("encrypted_index")
        || normalized.contains("retrieval")
        || normalized.contains("search_job")
    {
        Some(ProductKind::Docdex)
    } else if normalized.contains("codali")
        || normalized.contains("code_agent")
        || normalized.contains("repo_context")
    {
        Some(ProductKind::Codali)
    } else if normalized.contains("mcoda")
        || normalized.contains("model_metadata")
        || normalized.contains("tool_boundary")
    {
        Some(ProductKind::Mcoda)
    } else {
        None
    }
}

fn default_product_workload_kind(product: ProductKind) -> &'static str {
    match product {
        ProductKind::Docdex => "docdex_encrypted_index",
        ProductKind::Mcoda => "mcoda_agent_workload",
        ProductKind::Codali => "codali_code_agent_package",
    }
}

fn render_product_workflow_recipe_json(recipe: &ProductWorkflowRecipe) -> String {
    format!(
        concat!(
            "{{",
            "\"product\":\"{}\",",
            "\"workflow_ref\":\"{}\",",
            "\"workload_kind\":\"{}\",",
            "\"command_recipes\":{},",
            "\"required_refs\":{},",
            "\"expected_failure_modes\":{},",
            "\"safe_retry_patterns\":{},",
            "\"submitted_via\":\"{}\",",
            "\"sdk_overgate_only\":{},",
            "\"authorized_refs_only\":{},",
            "\"secret_free_json_output\":{},",
            "\"dynamic_model_resource_selection\":{},",
            "\"direct_internal_api_access\":{},",
            "\"direct_storage_access\":{},",
            "\"raw_http_required\":{},",
            "\"hardcoded_model_or_provider\":{},",
            "\"hardcoded_node_assumption\":{},",
            "\"paid_service_assumption\":{}",
            "}}"
        ),
        json_escape(recipe.product.as_str()),
        json_escape(&recipe.workflow_ref),
        json_escape(&recipe.workload_kind),
        json_owned_string_array(&recipe.command_recipes),
        json_owned_string_array(&recipe.required_refs),
        json_owned_string_array(&recipe.expected_failure_modes),
        json_owned_string_array(&recipe.safe_retry_patterns),
        json_escape(&recipe.submitted_via),
        recipe.sdk_overgate_only,
        recipe.authorized_refs_only,
        recipe.secret_free_json_output,
        recipe.dynamic_model_resource_selection,
        recipe.direct_internal_api_access,
        recipe.direct_storage_access,
        recipe.raw_http_required,
        recipe.hardcoded_model_or_provider,
        recipe.hardcoded_node_assumption,
        recipe.paid_service_assumption,
    )
}

fn render_ci_automation_profile_optional_json(context: &BootstrapContext) -> String {
    if context.profile.environment != EnvironmentClass::Ci {
        return String::new();
    }
    let profile = CiAutomationProfile::new(
        context.profile.environment,
        context.credential.class.as_str(),
        context.credential.reference_id.clone(),
    );
    format!(
        ",\"ci_automation_profile\":{}",
        render_ci_automation_profile_json(&profile)
    )
}

fn render_ci_automation_profile_json(profile: &CiAutomationProfile) -> String {
    format!(
        concat!(
            "{{",
            "\"profile_kind\":\"{}\",",
            "\"environment_class\":\"{}\",",
            "\"credential_reference_class\":\"{}\",",
            "\"credential_reference_id\":\"{}\",",
            "\"allowed_credential_ref_kinds\":{},",
            "\"submitted_via\":\"{}\",",
            "\"short_lived_service_account_required\":{},",
            "\"ambient_persistent_keychain_allowed\":{},",
            "\"requires_non_interactive_confirmation\":{},",
            "\"json_output_stable\":{},",
            "\"secret_free_output\":{},",
            "\"branch_on_exit_class\":{}",
            "}}"
        ),
        json_escape(&profile.profile_kind),
        json_escape(profile.environment_class.as_str()),
        json_escape(&profile.credential_reference_class),
        json_escape(&profile.credential_reference_id),
        json_owned_string_array(&profile.allowed_credential_ref_kinds),
        json_escape(&profile.submitted_via),
        profile.short_lived_service_account_required,
        profile.ambient_persistent_keychain_allowed,
        profile.requires_non_interactive_confirmation,
        profile.json_output_stable,
        profile.secret_free_output,
        profile.branch_on_exit_class,
    )
}

fn render_release_readiness_report_json(report: &CliReleaseReadinessReport) -> String {
    let phase_records = report
        .phase_availability_matrix
        .iter()
        .map(render_phase_availability_record_json)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "{{",
            "\"contract_snapshot_suite\":{},",
            "\"help_snapshot_commands\":{},",
            "\"exit_code_classes\":{},",
            "\"reason_code_families\":{},",
            "\"security_review_report\":{},",
            "\"phase_availability_matrix\":[{}],",
            "\"integration_validation_matrix\":{},",
            "\"automation_compatibility_matrix\":{},",
            "\"handoff_notes\":{},",
            "\"release_ready\":{},",
            "\"sdk_overgate_only\":{},",
            "\"direct_private_shortcut\":{},",
            "\"high_risk_phase7_phase13_enabled\":{}",
            "}}"
        ),
        json_owned_string_array(&report.contract_snapshot_suite),
        json_owned_string_array(&report.help_snapshot_commands),
        json_owned_string_array(&report.exit_code_classes),
        json_owned_string_array(&report.reason_code_families),
        render_security_review_report_json(&report.security_review_report),
        phase_records,
        json_owned_string_array(&report.integration_validation_matrix),
        json_owned_string_array(&report.automation_compatibility_matrix),
        json_owned_string_array(&report.handoff_notes),
        report.release_ready,
        report.sdk_overgate_only,
        report.direct_private_shortcut,
        report.high_risk_phase7_phase13_enabled,
    )
}

fn render_security_review_report_json(report: &CliSecurityReviewReport) -> String {
    format!(
        concat!(
            "{{",
            "\"reviewed_surfaces\":{},",
            "\"redaction_probes\":{},",
            "\"forbidden_output_markers\":{},",
            "\"raw_keys_exposed\":{},",
            "\"tokens_exposed\":{},",
            "\"signatures_exposed\":{},",
            "\"secrets_exposed\":{},",
            "\"private_payloads_exposed\":{},",
            "\"decrypted_content_exposed\":{},",
            "\"unsafe_endpoints_allowed\":{},",
            "\"cross_tenant_access_allowed\":{}",
            "}}"
        ),
        json_owned_string_array(&report.reviewed_surfaces),
        json_owned_string_array(&report.redaction_probes),
        json_owned_string_array(&report.forbidden_output_markers),
        report.raw_keys_exposed,
        report.tokens_exposed,
        report.signatures_exposed,
        report.secrets_exposed,
        report.private_payloads_exposed,
        report.decrypted_content_exposed,
        report.unsafe_endpoints_allowed,
        report.cross_tenant_access_allowed,
    )
}

fn render_phase_availability_record_json(record: &CliPhaseAvailabilityRecord) -> String {
    format!(
        concat!(
            "{{",
            "\"command\":\"{}\",",
            "\"phase_gate\":\"{}\",",
            "\"availability\":\"{}\",",
            "\"stable_reason_code\":\"{}\",",
            "\"hidden_in_normal_help\":{},",
            "\"direct_private_shortcut\":{}",
            "}}"
        ),
        json_escape(&record.command),
        json_escape(&record.phase_gate),
        json_escape(&record.availability),
        json_escape(&record.stable_reason_code),
        record.hidden_in_normal_help,
        record.direct_private_shortcut,
    )
}

fn render_version_json() -> String {
    let info = version_info();
    let result_json = format!(
        concat!(
            "{{",
            "\"cli_name\":\"{}\",",
            "\"cli_package\":\"{}\",",
            "\"cli_version\":\"{}\",",
            "\"contract_source_root\":\"{}\",",
            "\"contract_status\":\"{}\",",
            "\"sdk_target\":\"{}\"",
            "}}"
        ),
        json_escape(info.cli_name),
        json_escape(info.cli_package),
        json_escape(info.cli_version),
        json_escape(info.contract_source_root),
        json_escape(info.contract_status),
        json_escape(info.sdk_target),
    );
    render_success_json(
        "version",
        &result_json,
        &["parsed", "completed"],
        None,
        None,
        &["local_contracts_available", "sdk_metadata_available"],
        &[CapabilityRoute {
            route: "version",
            phase_gate: "phase_4",
            available: true,
        }],
    )
}

fn default_target_ref(globals: &GlobalOptions, fallback: &str) -> String {
    globals
        .target_ref
        .clone()
        .unwrap_or_else(|| fallback.to_owned())
}

#[allow(dead_code)]
fn deterministic_idempotency_key(
    profile: &CliProfile,
    command_name: &str,
    target_ref: &str,
) -> String {
    format!(
        "idem_{}_{}_{}_{}_{}",
        id_component(profile.environment.as_str()),
        id_component(&profile.tenant_id),
        id_component(&profile.actor_id),
        id_component(command_name),
        id_component(target_ref),
    )
}

fn canonical_payload_hash(payload_type: &str, target_ref: &str, globals: &GlobalOptions) -> String {
    stable_hash(&format!(
        "payload_type={}|target_ref={}|expected_state={}|reason={}|manifest_kind={}|manifest_ref={}|workload_kind={}|workload_ref={}|schema={}",
        payload_type,
        target_ref,
        globals.expected_state.as_deref().unwrap_or("none"),
        globals.reason.as_deref().unwrap_or("none"),
        globals.manifest_kind.as_deref().unwrap_or("none"),
        globals.manifest_ref.as_deref().unwrap_or("none"),
        globals.workload_kind.as_deref().unwrap_or("none"),
        globals.workload_ref.as_deref().unwrap_or("none"),
        SUPPORTED_SCHEMA_VERSION,
    ))
}

fn canonical_fingerprint(
    profile: &CliProfile,
    command_name: &str,
    target_ref: &str,
    canonical_payload_hash: &str,
    globals: &GlobalOptions,
) -> Result<CanonicalIdempotencyFingerprint, overrid_contracts::ContractError> {
    let source = format!(
        "environment={}|endpoint={}|tenant={}|actor={}|command={}|target={}|payload_hash={}|expected_state={}|reason={}|schema={}",
        profile.environment.as_str(),
        profile.endpoint_fingerprint,
        profile.tenant_id,
        profile.actor_id,
        command_name,
        target_ref,
        canonical_payload_hash,
        globals.expected_state.as_deref().unwrap_or("none"),
        globals.reason.as_deref().unwrap_or("none"),
        SUPPORTED_SCHEMA_VERSION,
    );
    let fingerprint = id_component(&format!("fp_{}", stable_hash(&source)));
    CanonicalIdempotencyFingerprint::new(
        profile.environment,
        profile.endpoint_fingerprint.clone(),
        profile.tenant_id.clone(),
        profile.actor_id.clone(),
        command_name,
        target_ref,
        canonical_payload_hash,
        globals.expected_state.clone(),
        globals.reason.clone(),
        SUPPORTED_SCHEMA_VERSION,
        fingerprint,
    )
}

fn stable_hash(value: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("hash_{hash:016x}")
}

fn cache_fingerprint_for_profile(profile: &CliProfile) -> String {
    id_component(&format!(
        "cache_{}_{}_{}",
        profile.name,
        profile.environment.as_str(),
        profile.endpoint_fingerprint
    ))
}

fn id_component(value: &str) -> String {
    let mut rendered = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | ':' | '-') {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    while rendered.contains("__") {
        rendered = rendered.replace("__", "_");
    }
    rendered.trim_matches('_').to_owned()
}

fn manifest_ref(globals: &GlobalOptions) -> ManifestBootstrapRef {
    ManifestBootstrapRef::new(
        globals.manifest_ref.as_deref().unwrap_or("manifest_local"),
        globals.manifest_kind.as_deref().unwrap_or("workload"),
    )
}

fn workload_pending_state(globals: &GlobalOptions) -> SyntheticWorkloadPendingState {
    SyntheticWorkloadPendingState::pending(
        globals.workload_ref.as_deref().unwrap_or("workload_local"),
        globals.workload_kind.as_deref().unwrap_or("synthetic"),
    )
}

fn node_state_for_ref(target_ref: &str, command: NodeCommand) -> NodeState {
    if matches!(command, NodeCommand::Register) {
        return NodeState::Live;
    }
    let normalized = target_ref.to_ascii_lowercase();
    if normalized.contains("disabled") {
        NodeState::Disabled
    } else if normalized.contains("draining") {
        NodeState::Draining
    } else if normalized.contains("expired") {
        NodeState::Expired
    } else if normalized.contains("stale") {
        NodeState::Stale
    } else {
        NodeState::Live
    }
}

fn workload_execution_state_for_command(
    command: WorkloadCommand,
    target_ref: &str,
) -> WorkloadExecutionState {
    if matches!(command, WorkloadCommand::Cancel) {
        return WorkloadExecutionState::Cancelled;
    }

    let normalized = target_ref.to_ascii_lowercase().replace('-', "_");
    if normalized.contains("cancelled") || normalized.contains("canceled") {
        return WorkloadExecutionState::Cancelled;
    }
    if normalized.contains("dead_letter") || normalized.contains("deadletter") {
        return WorkloadExecutionState::DeadLettered;
    }
    if normalized.contains("timed_out") || normalized.contains("timeout") {
        return WorkloadExecutionState::TimedOut;
    }
    if normalized.contains("failed") || normalized.contains("failure") {
        return WorkloadExecutionState::Failed;
    }

    match command {
        WorkloadCommand::Cancel => unreachable!("cancel is handled before target-ref mapping"),
        WorkloadCommand::Status => WorkloadExecutionState::Running,
        WorkloadCommand::Logs
        | WorkloadCommand::Timeline
        | WorkloadCommand::Result
        | WorkloadCommand::Follow => WorkloadExecutionState::Succeeded,
        WorkloadCommand::Submit => WorkloadExecutionState::Scheduled,
    }
}

fn workload_execution_states(
    terminal_state: WorkloadExecutionState,
) -> Vec<WorkloadExecutionState> {
    let mut states = vec![
        WorkloadExecutionState::Scheduled,
        WorkloadExecutionState::Leased,
        WorkloadExecutionState::Running,
    ];
    if !matches!(terminal_state, WorkloadExecutionState::Running) {
        states.push(terminal_state);
    }
    states
}

fn polling_plan_for(command: WorkloadCommand, globals: &GlobalOptions) -> PollingPlan {
    let follow = globals.follow || matches!(command, WorkloadCommand::Follow);
    let wait = globals.wait || follow;
    PollingPlan::bounded(
        wait,
        follow,
        globals.timeout_ms.unwrap_or(10_000),
        globals.poll_interval_ms.unwrap_or(1_000),
    )
}

fn policy_reason_codes(target_ref: &str) -> Vec<String> {
    let normalized = target_ref.to_ascii_lowercase().replace('-', "_");
    let reason = if normalized.contains("denied_egress") || normalized.contains("egress") {
        Some("policy.egress_denied")
    } else if normalized.contains("wrong_tenant") || normalized.contains("tenant") {
        Some("policy.wrong_tenant")
    } else if normalized.contains("insufficient_trust") || normalized.contains("trust") {
        Some("policy.insufficient_trust")
    } else if normalized.contains("quota_exhausted") || normalized.contains("quota") {
        Some("policy.quota_exhausted")
    } else if normalized.contains("unsupported_workload") || normalized.contains("unsupported") {
        Some("policy.unsupported_workload_class")
    } else {
        None
    };
    reason.into_iter().map(str::to_owned).collect()
}

fn package_validation_state_for_ref(target_ref: &str) -> PackageValidationState {
    let normalized = target_ref.to_ascii_lowercase().replace('-', "_");
    if normalized.contains("unsupported_version") || normalized.contains("unsupported") {
        PackageValidationState::UnsupportedVersion
    } else if normalized.contains("missing_provenance") || normalized.contains("provenance") {
        PackageValidationState::MissingProvenance
    } else if normalized.contains("policy_incompatible") || normalized.contains("incompatible") {
        PackageValidationState::PolicyIncompatible
    } else if normalized.contains("invalid") {
        PackageValidationState::InvalidPackage
    } else {
        PackageValidationState::Accepted
    }
}

fn package_reason_codes(state: PackageValidationState) -> Vec<String> {
    let reason = match state {
        PackageValidationState::Accepted => None,
        PackageValidationState::InvalidPackage => Some("package.invalid"),
        PackageValidationState::UnsupportedVersion => Some("package.unsupported_version"),
        PackageValidationState::MissingProvenance => Some("package.missing_provenance"),
        PackageValidationState::PolicyIncompatible => Some("package.policy_incompatible"),
    };
    reason.into_iter().map(str::to_owned).collect()
}

fn json_optional_string(value: Option<&str>) -> String {
    value
        .map(|value| format!("\"{}\"", json_escape(value)))
        .unwrap_or_else(|| "null".to_owned())
}

fn json_string_array(values: &[&str]) -> String {
    let rendered = values
        .iter()
        .map(|value| format!("\"{}\"", json_escape(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{rendered}]")
}

fn json_owned_string_array(values: &[String]) -> String {
    let rendered = values
        .iter()
        .map(|value| format!("\"{}\"", json_escape(value)))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{rendered}]")
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOCAL_PROFILE_ARGS: &[&str] = &[
        "--profile",
        "local-dev",
        "--environment",
        "local",
        "--endpoint",
        "http://127.0.0.1:18080/overgate",
        "--endpoint-fingerprint",
        "fp_local",
        "--tenant",
        "tenant_local",
        "--actor",
        "actor_local",
        "--credential-namespace",
        "local-dev",
        "--credential-class",
        "fixture",
        "--credential-ref",
        "fixture://local-dev/key-1",
        "--key-id",
        "key-1",
        "--fixture-allowance",
        "local_only",
    ];

    const CI_PROFILE_ARGS: &[&str] = &[
        "--profile",
        "ci-automation",
        "--environment",
        "ci",
        "--endpoint",
        "http://127.0.0.1:18080/overgate",
        "--endpoint-fingerprint",
        "fp_ci",
        "--tenant",
        "tenant_ci",
        "--actor",
        "actor_ci",
        "--credential-namespace",
        "ci",
        "--credential-class",
        "ci_reference",
        "--credential-ref",
        "ci://overrid/service-account/short-lived",
        "--key-id",
        "ci-key-1",
        "--fixture-allowance",
        "test_harness_only",
    ];

    fn args_with<'a>(prefix: &[&'a str], suffix: &[&'a str]) -> Vec<&'a str> {
        let mut values = vec!["overrid"];
        values.extend_from_slice(prefix);
        values.extend_from_slice(suffix);
        values
    }

    fn write_test_file(root: &Path, path: &str, content: &str) {
        let full_path = root.join(path);
        std::fs::create_dir_all(full_path.parent().expect("test file has parent"))
            .expect("test parent directory should be created");
        std::fs::write(full_path, content).expect("test file should be written");
    }

    #[test]
    fn renders_human_version_metadata() {
        let result = run_args(["overrid", "version"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("overrid 0.1.0"));
        assert!(result.stdout.contains("schema: cli-command.v0.1"));
        assert!(result.stdout.contains("sdk_target: overgate_only"));
    }

    #[test]
    fn renders_json_version_envelope() {
        let result = run_args(["overrid", "version", "--json", "--no-color"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result
            .stdout
            .contains("\"schema_version\":\"cli-command.v0.1\""));
        assert!(result.stdout.contains("\"sdk_target\":\"overgate_only\""));
        assert!(result.stdout.contains("\"exit_class\":\"success\""));
        assert!(result
            .stdout
            .contains("\"lifecycle\":{\"states\":[\"parsed\",\"completed\"]"));
        assert!(result.stdout.contains("\"diagnostic_bundle\""));
    }

    #[test]
    fn doctor_renders_redacted_diagnostic_bundle() {
        let result = run_args(["overrid", "doctor", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"command_name\":\"doctor\""));
        assert!(result
            .stdout
            .contains("\"redaction_policy\":\"secret_free_refs_only\""));
        assert!(result.stdout.contains("\"capability_cache_static\""));
        assert!(!result.stdout.to_ascii_lowercase().contains("private key"));
    }

    #[test]
    fn normal_help_shows_phase7_commands() {
        let result = run_args(["overrid", "help"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("command-registry"));
        assert!(result.stdout.contains("layout:check"));
        assert!(result.stdout.contains("schema:check"));
        assert!(result.stdout.contains("docs:check"));
        assert!(result.stdout.contains("node register|inspect|health"));
        assert!(result
            .stdout
            .contains("workload submit|status|timeline|logs|cancel|result|follow"));
        assert!(result.stdout.contains("policy dry-run"));
        assert!(result.stdout.contains("package validate"));
        assert!(result.stdout.contains("usage show"));
        assert!(result.stdout.contains("receipt show"));
        assert!(result.stdout.contains("ledger inspect"));
        assert!(result.stdout.contains("dispute list|inspect"));
        assert!(result
            .stdout
            .contains("test integration|scenario|list|reset|artifacts"));
        assert!(result.stdout.contains("--service REF"));
        assert!(result.stdout.contains("--changed-path PATH"));
        assert!(result.stdout.contains("--gate-class CLASS"));
        assert!(result.stdout.contains("release-readiness"));
    }

    #[test]
    fn command_registry_lists_semantic_root_commands_and_layout_artifacts() {
        let result = run_args(["overrid", "command-registry", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result
            .stdout
            .contains("\"command_name\":\"command-registry\""));
        for command in [
            "build",
            "test",
            "test:integration",
            "dev:start",
            "dev:stop",
            "dev:reset",
            "dev:seed",
            "dev:status",
            "schema:check",
            "docs:check",
            "layout:check",
        ] {
            assert!(result.stdout.contains(&format!("\"name\":\"{command}\"")));
        }
        assert!(result
            .stdout
            .contains("\"machine_readable_result_envelope\":true"));
        assert!(result.stdout.contains("\"owning_tool\":\"overrid-cli\""));
        for artifact in LAYOUT_VALIDATION_ARTIFACTS {
            assert!(result.stdout.contains(artifact));
        }
    }

    #[test]
    fn root_command_orchestration_records_are_json_enveloped() {
        let result = run_args(["overrid", "build", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"command_name\":\"build\""));
        assert!(result.stdout.contains("\"status\":\"registered\""));
        assert!(result
            .stdout
            .contains("\"canonical_invocation\":\"overrid build\""));
        assert!(result
            .stdout
            .contains("rust_owned_command_execution_defined"));

        let test_result = run_args(["overrid", "test", "--json"]);
        assert_eq!(test_result.exit_code, EXIT_SUCCESS);
        assert!(test_result.stdout.contains("\"command_name\":\"test\""));
        assert!(test_result.stdout.contains("\"status\":\"registered\""));
        assert!(test_result
            .stdout
            .contains("\"canonical_invocation\":\"overrid test\""));
    }

    #[test]
    fn layout_check_emits_stable_validation_artifacts() {
        let result = run_args(["overrid", "layout:check", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"command_name\":\"layout:check\""));
        assert!(result.stdout.contains("\"status\":\"passed\""));
        assert!(result.stdout.contains("layout_check.passed"));
        assert!(result.stdout.contains("secret_file_committed"));
        assert!(result.stdout.contains("\"check\":\"secret_file_absence\""));
        assert!(!result.stdout.to_ascii_lowercase().contains("private key"));
    }

    #[test]
    fn layout_check_emits_phase6_boundary_records() {
        let result = run_args(["overrid", "layout:check", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        for check in [
            "phase6_boundary_enforcement",
            "shared_schema_dependency_path",
            "modular_control_plane_shape",
            "split_review_criteria",
            "local_test_only_separation",
            "dependency_direction_group",
            "service_contract_root_not_deployable",
        ] {
            assert!(result.stdout.contains(&format!("\"check\":\"{check}\"")));
        }
        for artifact in [
            "schema_ref_missing",
            "premature_service_split",
            "split_review_missing",
            "local_test_boundary_violation",
        ] {
            assert!(result.stdout.contains(artifact));
        }
        assert!(result.stdout.contains("\"validation_artifact_schema\""));
    }

    #[test]
    fn layout_check_emits_phase7_hygiene_records() {
        let result = run_args(["overrid", "layout:check", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        for check in [
            "phase7_artifact_hygiene",
            "generated_output_ignore_rules",
            "local_state_ignore_rules",
            "secret_file_rules",
            "docdex_indexing_hygiene",
            "artifact_redaction_expectations",
            "required_hygiene_file",
            "local_state_marker_clean",
            "docdexignore_generated_specs_rule",
            "secret_like_path_absence",
        ] {
            assert!(result.stdout.contains(&format!("\"check\":\"{check}\"")));
        }
        for artifact in [
            "local_state_committed",
            "docdex_index_hygiene_violation",
            "artifact_redaction_violation",
        ] {
            assert!(result.stdout.contains(artifact));
        }
        assert!(!result.stdout.contains("OVERRID_PHASE7_SENTINEL_SECRET"));
    }

    #[test]
    fn layout_check_emits_phase8_lifecycle_records() {
        let result = run_args(["overrid", "layout:check", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        for check in [
            "phase8_module_lifecycle",
            "service_contract_template_usage",
            "new_module_checklist",
            "module_addition_workflow",
            "deprecation_removal_workflow",
            "cross_document_maintenance_rules",
            "module_lifecycle_state_catalog",
            "accepted_module_validation_evidence",
        ] {
            assert!(result.stdout.contains(&format!("\"check\":\"{check}\"")));
        }
        for artifact in ["module_lifecycle_violation", "stale_layout_reference"] {
            assert!(result.stdout.contains(artifact));
        }
    }

    #[test]
    fn layout_check_emits_phase9_foundation_integration_records() {
        let result = run_args(["overrid", "layout:check", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        for check in [
            "phase9_foundation_integration",
            "local_stack_discovery_metadata",
            "local_stack_service_definition_roots",
            "local_stack_safe_reset_markers",
            "harness_discovery_metadata",
            "harness_scenario_roots",
            "harness_schema_refs",
            "clean_checkout_ci_behavior",
            "clean_checkout_ci_statuses",
            "validation_evidence_model",
            "validation_artifact_consumers",
            "artifact_consumer_boundary",
        ] {
            assert!(result.stdout.contains(&format!("\"check\":\"{check}\"")));
        }
        for artifact in [
            "local_stack_discovery_violation",
            "harness_discovery_violation",
            "ci_command_sequence_violation",
            "validation_evidence_missing",
            "artifact_consumer_violation",
        ] {
            assert!(result.stdout.contains(artifact));
        }
    }

    #[test]
    fn layout_check_emits_phase10_alignment_handoff_records() {
        let result = run_args(["overrid", "layout:check", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        for check in [
            "phase10_alignment_handoff",
            "sub_build_plan_structure_checks",
            "tech_stack_alignment_checks",
            "master_plan_alignment_checks",
            "source_alignment_documents",
            "phase10_planning_documents",
            "downstream_phase_handoff_rules",
            "downstream_handoff_boundary",
            "phase10_validation_script",
        ] {
            assert!(result.stdout.contains(&format!("\"check\":\"{check}\"")));
        }
        for artifact in [
            "sub_build_plan_structure_violation",
            "tech_stack_alignment_violation",
            "master_plan_alignment_violation",
            "source_document_alignment_violation",
            "downstream_handoff_violation",
        ] {
            assert!(result.stdout.contains(artifact));
        }
    }

    #[test]
    fn layout_check_rejects_real_phase6_boundary_violations() {
        let temp_root = std::env::temp_dir().join(format!(
            "overrid-phase6-layout-check-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&temp_root);
        write_test_file(
            &temp_root,
            "packages/sdk/Cargo.toml",
            "[package]\nname = \"overrid-sdk\"\n\n[dependencies]\n",
        );
        write_test_file(
            &temp_root,
            "services/control-plane/overgate/Cargo.toml",
            "[package]\nname = \"overrid-overgate\"\n",
        );

        let records = collect_layout_check_records(&temp_root);
        assert!(records.iter().any(|record| {
            record.check_name == "dependency_direction_group"
                && record.status == "failed"
                && record.reason_code == "package_boundary_violation"
                && record.path == "packages/sdk/Cargo.toml"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "service_contract_root_not_deployable"
                && record.status == "failed"
                && record.reason_code == "premature_service_split"
                && record.path == "services/control-plane/overgate/Cargo.toml"
        }));

        std::fs::remove_dir_all(&temp_root).expect("temporary repo should be removable");
    }

    #[test]
    fn layout_check_rejects_phase7_hygiene_violations() {
        let temp_root = std::env::temp_dir().join(format!(
            "overrid-phase7-layout-check-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&temp_root);
        write_test_file(
            &temp_root,
            "docs/specs/generated/.gitignore",
            "*\n!.gitignore\n",
        );
        write_test_file(
            &temp_root,
            "docs/specs/generated/generated.json",
            "{\"generated\":true}\n",
        );
        write_test_file(
            &temp_root,
            "infra/local/state/.gitignore",
            "*\n!.gitignore\n",
        );
        write_test_file(&temp_root, "infra/local/state/local.db", "state\n");
        write_test_file(
            &temp_root,
            ".env.local",
            "OVERRID_PHASE7_SENTINEL_SECRET=raw\n",
        );

        let records = collect_layout_check_records(&temp_root);
        assert!(records.iter().any(|record| {
            record.check_name == "generated_specs_not_committed"
                && record.status == "failed"
                && record.reason_code == "generated_file_committed"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "local_state_marker_clean"
                && record.status == "failed"
                && record.reason_code == "local_state_committed"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "secret_like_path_absence"
                && record.status == "failed"
                && record.reason_code == "secret_file_committed"
                && record.path == ".env.local"
        }));

        let refs = layout_artifact_refs(false, &records);
        assert!(refs
            .iter()
            .any(|artifact| artifact.contains("generated_file_committed")));
        assert!(refs
            .iter()
            .any(|artifact| artifact.contains("local_state_committed")));
        assert!(refs
            .iter()
            .any(|artifact| artifact.contains("secret_file_committed")));
        assert!(!refs.join("\n").contains("OVERRID_PHASE7_SENTINEL_SECRET"));

        std::fs::remove_dir_all(&temp_root).expect("temporary repo should be removable");
    }

    #[test]
    fn layout_check_rejects_phase8_lifecycle_violations() {
        let temp_root = std::env::temp_dir().join(format!(
            "overrid-phase8-layout-check-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&temp_root);
        write_test_file(
            &temp_root,
            "overrid.workspace.toml",
            concat!(
                "accepted_lifecycle_states = [\"scaffolded\", \"accepted\"]\n",
                "\n[[modules]]\n",
                "name = \"bad-module\"\n",
                "lifecycle_state = \"accepted\"\n",
                "documentation_links = []\n",
                "local_stack_participation = \"none\"\n",
                "\n[[modules]]\n",
                "name = \"invalid-module\"\n",
                "lifecycle_state = \"invented\"\n",
                "test_targets = [\"python3 scripts/validate_fake.py\"]\n",
                "documentation_links = []\n",
                "local_stack_participation = \"none\"\n",
            ),
        );

        let records = collect_layout_check_records(&temp_root);
        assert!(records.iter().any(|record| {
            record.check_name == "module_lifecycle_state_catalog"
                && record.status == "failed"
                && record.reason_code == "module_lifecycle_violation"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "module_lifecycle_state_valid"
                && record.status == "failed"
                && record.reason_code == "module_lifecycle_violation"
                && record.path == "overrid.workspace.toml#invalid-module"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "accepted_module_validation_evidence"
                && record.status == "failed"
                && record.reason_code == "missing_test_target"
                && record.path == "overrid.workspace.toml#bad-module"
        }));

        std::fs::remove_dir_all(&temp_root).expect("temporary repo should be removable");
    }

    #[test]
    fn layout_check_rejects_phase9_foundation_integration_violations() {
        let temp_root = std::env::temp_dir().join(format!(
            "overrid-phase9-layout-check-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&temp_root);
        write_test_file(
            &temp_root,
            "overrid.workspace.toml",
            concat!(
                "[foundation_integration]\n",
                "states = [\"local_stack_discovery_metadata_defined\"]\n",
                "local_stack_discovery_fields = [\"profile_roots\"]\n",
                "clean_checkout_ci_commands = [\"overrid layout:check\"]\n",
            ),
        );

        let records = collect_layout_check_records(&temp_root);
        assert!(records.iter().any(|record| {
            record.check_name == "local_stack_service_definition_roots"
                && record.status == "failed"
                && record.reason_code == "local_stack_discovery_violation"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "harness_discovery_metadata"
                && record.status == "failed"
                && record.reason_code == "harness_discovery_violation"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "clean_checkout_ci_statuses"
                && record.status == "failed"
                && record.reason_code == "ci_command_sequence_violation"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "validation_evidence_model"
                && record.status == "failed"
                && record.reason_code == "validation_evidence_missing"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "artifact_consumer_boundary"
                && record.status == "failed"
                && record.reason_code == "artifact_consumer_violation"
        }));

        std::fs::remove_dir_all(&temp_root).expect("temporary repo should be removable");
    }

    #[test]
    fn layout_check_rejects_phase10_alignment_handoff_violations() {
        let temp_root = std::env::temp_dir().join(format!(
            "overrid-phase10-layout-check-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&temp_root);
        write_test_file(
            &temp_root,
            "overrid.workspace.toml",
            concat!(
                "[alignment_handoff]\n",
                "states = [\"sub_build_plan_structure_validated\"]\n",
                "structure_checks = [\"title_prefix\"]\n",
                "tech_stack_alignment_checks = [\"rust_first_workspace\"]\n",
            ),
        );

        let records = collect_layout_check_records(&temp_root);
        assert!(records.iter().any(|record| {
            record.check_name == "master_plan_alignment_checks"
                && record.status == "failed"
                && record.reason_code == "master_plan_alignment_violation"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "source_alignment_documents"
                && record.status == "failed"
                && record.reason_code == "source_document_alignment_violation"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "downstream_phase_handoff_rules"
                && record.status == "failed"
                && record.reason_code == "downstream_handoff_violation"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "downstream_handoff_boundary"
                && record.status == "failed"
                && record.reason_code == "downstream_handoff_violation"
        }));
        assert!(records.iter().any(|record| {
            record.check_name == "phase10_validation_script"
                && record.status == "failed"
                && record.reason_code == "source_document_alignment_violation"
        }));

        std::fs::remove_dir_all(&temp_root).expect("temporary repo should be removable");
    }

    #[test]
    fn test_list_renders_stable_json_with_phase_filter() {
        let result = run_args(["overrid", "test", "list", "--phase", "0", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"command_name\":\"test list\""));
        assert!(result.stdout.contains("\"phase_filter\":0"));
        assert!(result
            .stdout
            .contains("\"scenario_id\":\"scenario_phase0_smoke\""));
        assert!(result
            .stdout
            .contains("\"scenario_id\":\"scenario_blocked_dependency\""));
        assert!(result
            .stdout
            .contains("\"schema_version\":\"integration-harness.v0.1\""));
    }

    #[test]
    fn test_list_renders_phase9_selector_filters() {
        let result = run_args([
            "overrid",
            "test",
            "list",
            "--phase",
            "9",
            "--service",
            "service:overgate",
            "--tag",
            "control_plane_spine",
            "--changed-path",
            "services/overgate/routes.rs",
            "--required-dependency",
            "fixture:phase9_control_plane_spine",
            "--gate-class",
            "contract_spine",
            "--scenario-name",
            "scenario_phase1_control_plane_spine",
            "--json",
        ]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result
            .stdout
            .contains("\"scenario_id\":\"scenario_phase1_control_plane_spine\""));
        assert!(result.stdout.contains("\"filters\""));
        assert!(result.stdout.contains("service:service:overgate"));
        assert!(result.stdout.contains("tag:control_plane_spine"));
        assert!(result
            .stdout
            .contains("changed_path:services/overgate/routes.rs"));
        assert!(result
            .stdout
            .contains("required_dependency:fixture:phase9_control_plane_spine"));
        assert!(result.stdout.contains("gate_class:contract_spine"));
        assert!(result
            .stdout
            .contains("scenario_name:scenario_phase1_control_plane_spine"));
    }

    #[test]
    fn test_scenario_reports_blocked_dependency_with_artifacts() {
        let result = run_args([
            "overrid",
            "test",
            "scenario",
            "scenario_blocked_dependency",
            "--json",
        ]);
        assert_eq!(result.exit_code, EXIT_CONFIG);
        assert!(result.stdout.contains("\"ok\":false"));
        assert!(result
            .stdout
            .contains("\"reason_code\":\"dependency.service_unavailable\""));
        assert!(result.stdout.contains("\"status\":\"blocked\""));
        assert!(result.stdout.contains("\"collecting_artifacts\""));
        assert!(result.stdout.contains("service:overqueue:unavailable"));
        assert!(result.stdout.contains("artifact:bundle:"));
    }

    #[test]
    fn test_integration_runs_phase0_smoke_with_local_stack_health() {
        let result = run_args(["overrid", "test", "integration", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"reason_code\":\"run.passed\""));
        assert!(result.stdout.contains("\"command\":\"test integration\""));
        assert!(result.stdout.contains("\"status\":\"passed\""));
        assert!(result.stdout.contains("\"stack_ready\""));
        assert!(result.stdout.contains("\"reset_refs\""));
        assert!(result.stdout.contains("\"seed_refs\""));
        assert!(result.stdout.contains("\"diagnostic_refs\""));
        assert!(result.stdout.contains("\"smoke_refs\""));
        assert!(result
            .stdout
            .contains("\"service_id\":\"service:local_stack\""));
    }

    #[test]
    fn test_integration_phase9_uses_ci_smoke_entrypoint() {
        let result = run_args(["overrid", "test", "integration", "--phase", "9", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result
            .stdout
            .contains("\"ci_entrypoint\":\"scenario_phase1_control_plane_spine\""));
        assert!(result
            .stdout
            .contains("\"scenario_id\":\"scenario_phase1_control_plane_spine\""));
        assert!(result
            .stdout
            .contains("\"service_id\":\"service:deployment_planner\""));
        assert!(result
            .stdout
            .contains("\"status\":\"missing_required_contract\""));
        assert!(!result.stdout.contains("flake.unstable_event_ordering"));
    }

    #[test]
    fn test_reset_runs_local_stack_reset_and_seed() {
        let result = run_args(["overrid", "test", "reset", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"command\":\"test reset\""));
        assert!(result.stdout.contains("\"status\":\"passed\""));
        assert!(result.stdout.contains("\"resetting\""));
        assert!(result.stdout.contains("\"seeding\""));
        assert!(result.stdout.contains("\"reset_refs\""));
        assert!(result.stdout.contains("\"seed_refs\""));
    }

    #[test]
    fn test_reset_rejects_non_local_profile() {
        let result = run_args([
            "overrid",
            "test",
            "reset",
            "--profile",
            "production_like",
            "--json",
        ]);
        assert_eq!(result.exit_code, EXIT_CONFIG);
        assert!(result
            .stdout
            .contains("\"reason_code\":\"safety.non_local_profile\""));
        assert!(result.stdout.contains("\"status\":\"blocked\""));
    }

    #[test]
    fn dev_prune_reports_marker_gated_retention_policy() {
        let result = run_args([
            "overrid",
            "dev",
            "prune",
            "--json",
            "--trace-id",
            "trace_cli_prune",
        ]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"command\":\"dev prune\""));
        assert!(result
            .stdout
            .contains("\"reason_code\":\"local_stack.artifact_retention_prune_verified\""));
        assert!(result.stdout.contains("\"artifact_retention_policies\""));
        assert!(result
            .stdout
            .contains("\"requires_test_state_marker\":true"));
        assert!(result
            .stdout
            .contains("\"deletes_unmarked_user_dirs\":false"));
        assert!(result
            .stdout
            .contains("\"deletes_production_like_state\":false"));
        assert!(result
            .stdout
            .contains("\"deletes_non_local_artifacts\":false"));
        assert!(result.stdout.contains("cleanup_prune_command_integrated"));
    }

    #[test]
    fn test_artifacts_lookup_succeeds_with_stable_ref() {
        let result = run_args(["overrid", "test", "artifacts", "run_phase0_smoke", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result
            .stdout
            .contains("\"reason_code\":\"artifact.lookup_ready\""));
        assert!(result.stdout.contains("artifact:bundle:run_phase0_smoke"));
        assert!(result.stdout.contains("\"terminal_state\":\"passed\""));
    }

    #[test]
    fn all_phases_help_documents_phase_gated_commands() {
        let result = run_args(["overrid", "help", "--all-phases"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("node register|inspect|health"));
        assert!(result.stdout.contains("deployment helpers"));
        assert!(result
            .stdout
            .contains("federation|public-interest|purpose-tag"));
        assert!(result
            .stdout
            .contains("governance|incident|compliance|migration"));
    }

    #[test]
    fn planned_command_fails_with_stable_phase_reason() {
        let result = run_args(["overrid", "deployment", "--json"]);
        assert_eq!(result.exit_code, EXIT_NOT_AVAILABLE_IN_PHASE);
        assert!(result
            .stdout
            .contains("\"reason_code\":\"not_available_in_phase\""));
        assert!(result.stdout.contains("\"phase_gate\":\"phase_9\""));
        assert!(result.stdout.contains("\"exit_class\":\"phase\""));
        assert!(result.stdout.contains("\"fail_closed\":true"));
    }

    #[test]
    fn phase10_release_readiness_emits_safe_validation_evidence() {
        let result = run_args(["overrid", "release-readiness", "--json"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result
            .stdout
            .contains("\"command_name\":\"release-readiness\""));
        assert!(result.stdout.contains("\"phase_gate\":\"phase_10\""));
        assert!(result.stdout.contains("\"release_ready\":true"));
        assert!(result.stdout.contains("\"sdk_overgate_only\":true"));
        assert!(result.stdout.contains("\"direct_private_shortcut\":false"));
        assert!(result
            .stdout
            .contains("\"high_risk_phase7_phase13_enabled\":false"));
        assert!(result
            .stdout
            .contains("\"stable_reason_code\":\"not_available_in_phase\""));
        assert!(result
            .stdout
            .contains("federation/public-interest/purpose-tag"));
        assert!(result
            .stdout
            .contains("governance/incident/compliance/migration"));
        assert!(result.stdout.contains("\"raw_keys_exposed\":false"));
        assert!(result.stdout.contains("\"tokens_exposed\":false"));
        assert!(result.stdout.contains("\"signatures_exposed\":false"));
        assert!(result.stdout.contains("\"private_payloads_exposed\":false"));
        assert!(result.stdout.contains("tenant_setup"));
        assert!(result.stdout.contains("real_private_job"));
        assert!(result.stdout.contains("ci_non_interactive_credentials"));
    }

    #[test]
    fn json_parse_errors_use_phase4_envelope() {
        let result = run_args(["overrid", "--json", "unknown"]);
        assert_eq!(result.exit_code, EXIT_USAGE);
        assert!(result.stderr.is_empty());
        assert!(result.stdout.contains("\"ok\":false"));
        assert!(result.stdout.contains("\"command_name\":\"parser\""));
        assert!(result.stdout.contains("\"reason_code\":\"usage_error\""));
        assert!(result.stdout.contains("\"exit_class\":\"usage\""));
        assert!(result.stdout.contains("\"terminal_state\":\"failed\""));
        assert!(result.stdout.contains("\"fail_closed\":true"));
    }

    #[test]
    fn profile_inspect_renders_sanitized_phase3_json() {
        let args = args_with(&["profile", "inspect", "--json"], LOCAL_PROFILE_ARGS);
        let result = run_args(args);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"profile_name\":\"local-dev\""));
        assert!(result.stdout.contains("\"environment_class\":\"local\""));
        assert!(result
            .stdout
            .contains("\"storage_policy\":\"owner_only_file_backed_config\""));
        assert!(!result.stdout.contains("PRIVATE KEY"));
    }

    #[test]
    fn credential_enroll_hands_off_signature_ref_without_key_material() {
        let args = args_with(&["credential", "enroll", "--json"], LOCAL_PROFILE_ARGS);
        let result = run_args(args);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"signature_ref\":\"sigref:"));
        assert!(result.stdout.contains("\"exposes_key_material\":false"));
        assert!(!result.stdout.contains("PRIVATE KEY"));
    }

    #[test]
    fn auth_whoami_reports_actor_without_key_material() {
        let args = args_with(&["auth", "whoami", "--json"], LOCAL_PROFILE_ARGS);
        let result = run_args(args);

        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"command\":\"auth whoami\""));
        assert!(result.stdout.contains("\"auth_status\":\"authenticated\""));
        assert!(result.stdout.contains("\"sdk_target\":\"overgate_only\""));
        assert!(result.stdout.contains("\"credential_key_id\":\"key-1\""));
        assert!(result.stdout.contains("\"trace_id\":\"trace_cli_local\""));
        assert!(!result.stdout.contains("PRIVATE KEY"));
    }

    #[test]
    fn tenant_create_uses_signed_deterministic_idempotency() {
        let args = args_with(
            &["tenant", "create", "--json", "--expected-state", "absent"],
            LOCAL_PROFILE_ARGS,
        );
        let first = run_args(args.clone());
        let second = run_args(args);

        assert_eq!(first.exit_code, EXIT_SUCCESS);
        assert_eq!(second.exit_code, EXIT_SUCCESS);
        assert_eq!(first.stdout, second.stdout);
        assert!(first.stdout.contains("\"signed\":true"));
        assert!(first.stdout.contains("\"signed_command_envelope\""));
        assert!(first.stdout.contains("\"idempotency_key\":\"idem_fp_hash_"));
        assert!(first
            .stdout
            .contains("\"idempotency_key_source\":\"canonical_fingerprint\""));
        assert!(first
            .stdout
            .contains("\"canonical_idempotency_fingerprint\""));
        assert!(first.stdout.contains("\"endpoint_identity\":\"fp_local\""));
        assert!(first.stdout.contains("\"canonical_payload_hash\":\"hash_"));
        assert!(first
            .stdout
            .contains("\"expected_current_state\":\"absent\""));
        assert!(first.stdout.contains("\"retry_timeout_policy\""));
        assert!(first.stdout.contains("\"max_retries\":2"));
        assert!(first.stdout.contains("\"timeout_ms\":10000"));
        assert!(first.stdout.contains("\"local_idempotency_cache\""));
        assert!(first.stdout.contains("\"contains_private_payload\":false"));
        assert!(first
            .stdout
            .contains("\"audit_refs\":[\"audit_cli_bootstrap_tenant_create\"]"));
        assert!(first.stdout.contains("\"expected_state\":\"absent\""));
    }

    #[test]
    fn new_idempotency_key_changes_operation_without_changing_fingerprint_shape() {
        let default_args = args_with(
            &["tenant", "create", "--json", "--expected-state", "absent"],
            LOCAL_PROFILE_ARGS,
        );
        let new_args = args_with(
            &[
                "tenant",
                "create",
                "--json",
                "--expected-state",
                "absent",
                "--new-idempotency-key",
                "--trace-id",
                "trace_cli_new",
            ],
            LOCAL_PROFILE_ARGS,
        );
        let default_result = run_args(default_args);
        let new_result = run_args(new_args);

        assert_eq!(default_result.exit_code, EXIT_SUCCESS);
        assert_eq!(new_result.exit_code, EXIT_SUCCESS);
        assert_ne!(default_result.stdout, new_result.stdout);
        assert!(new_result
            .stdout
            .contains("\"idempotency_key\":\"idem_new_fp_hash_"));
        assert!(new_result
            .stdout
            .contains("\"idempotency_key_source\":\"new_operation\""));
        assert!(new_result.stdout.contains("\"trace_id\":\"trace_cli_new\""));
    }

    #[test]
    fn timeout_and_retry_flags_flow_into_policy() {
        let args = args_with(
            &[
                "tenant",
                "create",
                "--json",
                "--timeout-ms",
                "4500",
                "--max-retries",
                "3",
            ],
            LOCAL_PROFILE_ARGS,
        );
        let result = run_args(args);

        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"retry_timeout_policy\""));
        assert!(result.stdout.contains("\"timeout_ms\":4500"));
        assert!(result.stdout.contains("\"max_retries\":3"));
    }

    #[test]
    fn idempotency_cache_commands_are_local_and_secret_free() {
        let inspect_args = args_with(
            &["idempotency-cache", "inspect", "--json"],
            LOCAL_PROFILE_ARGS,
        );
        let reset_args = args_with(
            &["idempotency-cache", "reset", "--json"],
            LOCAL_PROFILE_ARGS,
        );

        let inspect = run_args(inspect_args);
        let reset = run_args(reset_args);

        assert_eq!(inspect.exit_code, EXIT_SUCCESS);
        assert_eq!(reset.exit_code, EXIT_SUCCESS);
        assert!(inspect.stdout.contains("\"cache_action\":\"inspect\""));
        assert!(inspect.stdout.contains("\"owner_only\":true"));
        assert!(inspect
            .stdout
            .contains("\"contains_private_payload\":false"));
        assert!(reset.stdout.contains("\"cache_action\":\"reset\""));
        assert!(reset.stdout.contains("\"resettable\":true"));
    }

    #[test]
    fn key_revoke_requires_reason_before_mutation() {
        let args = args_with(&["key", "revoke", "--json"], LOCAL_PROFILE_ARGS);
        let result = run_args(args);

        assert_eq!(result.exit_code, EXIT_USAGE);
        assert!(result.stdout.contains("\"reason_code\":\"missing_reason\""));
        assert!(result.stdout.contains("\"error_decode_record\""));
        assert!(result
            .stdout
            .contains("\"raw_internal_error_exposed\":false"));
    }

    #[test]
    fn manifest_validate_and_submit_return_refs() {
        let validate_args = args_with(
            &[
                "manifest",
                "validate",
                "--json",
                "--manifest-kind",
                "workload",
                "--manifest-ref",
                "manifest_local",
            ],
            LOCAL_PROFILE_ARGS,
        );
        let validate = run_args(validate_args);
        assert_eq!(validate.exit_code, EXIT_SUCCESS);
        assert!(validate
            .stdout
            .contains("\"local_validation\":\"accepted\""));
        assert!(validate.stdout.contains("\"manifest_kind\":\"workload\""));

        let submit_args = args_with(
            &[
                "manifest",
                "submit",
                "--json",
                "--manifest-kind",
                "workload",
                "--manifest-ref",
                "manifest_local",
            ],
            LOCAL_PROFILE_ARGS,
        );
        let submit = run_args(submit_args);
        assert_eq!(submit.exit_code, EXIT_SUCCESS);
        assert!(submit.stdout.contains("\"signed\":true"));
        assert!(submit
            .stdout
            .contains("\"immutable_ref\":\"manifest:manifest_local:immutable\""));
    }

    #[test]
    fn synthetic_workload_submit_stops_at_pending_state() {
        let submit_args = args_with(
            &[
                "workload",
                "submit",
                "--json",
                "--workload-kind",
                "synthetic",
                "--workload-ref",
                "workload_local",
            ],
            LOCAL_PROFILE_ARGS,
        );
        let submit = run_args(submit_args);

        assert_eq!(submit.exit_code, EXIT_SUCCESS);
        assert!(submit.stdout.contains("\"queue_state\":\"pending\""));
        assert!(submit.stdout.contains("\"execution_implied\":false"));
        assert!(submit.stdout.contains("\"signed\":true"));

        let timeline_args = args_with(
            &[
                "workload",
                "timeline",
                "--json",
                "--workload-kind",
                "synthetic",
                "--workload-ref",
                "workload_local",
            ],
            LOCAL_PROFILE_ARGS,
        );
        let timeline = run_args(timeline_args);
        assert_eq!(timeline.exit_code, EXIT_SUCCESS);
        assert!(timeline.stdout.contains("\"timeline_events\""));
        assert!(timeline.stdout.contains("\"queue_state\":\"pending\""));
        assert!(timeline.stdout.contains("\"execution_timeline\""));
    }

    #[test]
    fn node_commands_emit_profile_scoped_status_refs() {
        let register_args = args_with(&["node", "register", "--json"], LOCAL_PROFILE_ARGS);
        let register = run_args(register_args);
        assert_eq!(register.exit_code, EXIT_SUCCESS);
        assert!(register.stdout.contains("\"command\":\"node register\""));
        assert!(register
            .stdout
            .contains("\"phase_gate\":\"phase_2_seed_private_swarm\""));
        assert!(register.stdout.contains("\"state\":\"live\""));
        assert!(register.stdout.contains("\"credential_checked\":true"));
        assert!(register.stdout.contains("\"direct_node_access\":false"));
        assert!(register.stdout.contains("\"signed\":true"));
        assert!(register
            .stdout
            .contains("\"submitted_via\":\"sdk_overgate_contract\""));

        let inspect_args = args_with(
            &["node", "inspect", "--json", "--target-ref", "node_draining"],
            LOCAL_PROFILE_ARGS,
        );
        let inspect = run_args(inspect_args);
        assert_eq!(inspect.exit_code, EXIT_SUCCESS);
        assert!(inspect.stdout.contains("\"state\":\"draining\""));
        assert!(inspect.stdout.contains("\"signed\":false"));
    }

    #[test]
    fn real_workload_execution_commands_emit_refs_only() {
        let logs_args = args_with(
            &[
                "workload",
                "logs",
                "--json",
                "--workload-ref",
                "workload_local",
                "--wait",
                "--timeout",
                "12000",
                "--poll-interval",
                "500",
            ],
            LOCAL_PROFILE_ARGS,
        );
        let logs = run_args(logs_args);
        assert_eq!(logs.exit_code, EXIT_SUCCESS);
        assert!(logs.stdout.contains("\"command\":\"workload logs\""));
        assert!(logs
            .stdout
            .contains("\"phase_gate\":\"phase_3_private_execution_loop\""));
        assert!(logs.stdout.contains("\"execution_logs\""));
        assert!(logs
            .stdout
            .contains("\"redaction_policy\":\"secret_free_refs_only\""));
        assert!(logs.stdout.contains("\"bounded_streaming\":true"));
        assert!(logs.stdout.contains("\"direct_node_path_exposed\":false"));
        assert!(logs.stdout.contains("\"wait\":true"));
        assert!(logs.stdout.contains("\"timeout_ms\":12000"));
        assert!(logs.stdout.contains("\"poll_interval_ms\":500"));

        let follow_args = args_with(
            &[
                "workload",
                "follow",
                "--json",
                "--workload-ref",
                "workload_local",
            ],
            LOCAL_PROFILE_ARGS,
        );
        let follow = run_args(follow_args);
        assert_eq!(follow.exit_code, EXIT_SUCCESS);
        assert!(follow.stdout.contains("\"follow\":true"));
        assert!(follow.stdout.contains("\"scheduled\""));
        assert!(follow.stdout.contains("\"leased\""));
        assert!(follow.stdout.contains("\"running\""));
        assert!(follow.stdout.contains("\"succeeded\""));
        assert!(follow.stdout.contains("\"oversched:scheduler\""));
        assert!(follow.stdout.contains("\"overlease:lease\""));
        assert!(follow.stdout.contains("\"overrun:runner\""));
        assert!(follow.stdout.contains("\"overwatch:trace\""));

        let result_args = args_with(
            &[
                "workload",
                "result",
                "--json",
                "--workload-ref",
                "workload_local",
            ],
            LOCAL_PROFILE_ARGS,
        );
        let result = run_args(result_args);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("\"execution_result\""));
        assert!(result
            .stdout
            .contains("\"direct_object_store_path_exposed\":false"));
        assert!(result
            .stdout
            .contains("\"authorized_control_plane_ref\":\"overgate:result:workload_local\""));

        let cancel_args = args_with(
            &[
                "workload",
                "cancel",
                "--json",
                "--workload-ref",
                "workload_local",
                "--reason",
                "operator requested",
            ],
            LOCAL_PROFILE_ARGS,
        );
        let cancel = run_args(cancel_args);
        assert_eq!(cancel.exit_code, EXIT_SUCCESS);
        assert!(cancel.stdout.contains("\"execution_state\":\"cancelled\""));
        assert!(cancel.stdout.contains("\"signed\":true"));
        assert!(cancel.stdout.contains("\"signature_ref\":\"sigref:"));
        assert!(cancel
            .stdout
            .contains("\"audit_cli_bootstrap_workload_cancel\""));

        for (fixture_ref, expected_state, expected_reason) in [
            ("workload_cancelled", "cancelled", "result.cancelled"),
            ("workload_retryable_failed", "failed", "result.failed"),
            ("workload_final_failed", "failed", "result.failed"),
            ("workload_timed_out", "timed_out", "result.timed_out"),
            (
                "workload_dead_lettered",
                "dead_lettered",
                "result.dead_lettered",
            ),
        ] {
            let state_args = args_with(
                &[
                    "workload",
                    "status",
                    "--json",
                    "--workload-ref",
                    fixture_ref,
                ],
                LOCAL_PROFILE_ARGS,
            );
            let state = run_args(state_args);
            assert_eq!(state.exit_code, EXIT_SUCCESS);
            assert!(state
                .stdout
                .contains(&format!("\"execution_state\":\"{expected_state}\"")));
            assert!(state.stdout.contains(expected_reason));
            assert!(state
                .stdout
                .contains("\"synthetic_workload_pending_state\""));
            assert!(state.stdout.contains("\"execution_timeline\""));
        }
    }

    #[test]
    fn phase8_policy_package_accounting_commands_emit_refs_only() {
        for (target_ref, expected_decision, expected_reason) in [
            ("policy_accept", "accepted", ""),
            ("policy_denied_egress", "denied", "policy.egress_denied"),
            ("policy_wrong_tenant", "denied", "policy.wrong_tenant"),
            (
                "policy_insufficient_trust",
                "denied",
                "policy.insufficient_trust",
            ),
            ("policy_quota_exhausted", "denied", "policy.quota_exhausted"),
            (
                "policy_unsupported_workload",
                "denied",
                "policy.unsupported_workload_class",
            ),
        ] {
            let args = args_with(
                &["policy", "dry-run", "--json", "--target-ref", target_ref],
                LOCAL_PROFILE_ARGS,
            );
            let result = run_args(args);
            assert_eq!(result.exit_code, EXIT_SUCCESS);
            assert!(result.stdout.contains("\"policy_dry_run_decision\""));
            assert!(result
                .stdout
                .contains(&format!("\"decision\":\"{expected_decision}\"")));
            assert!(result.stdout.contains("\"mutates_platform_state\":false"));
            assert!(result
                .stdout
                .contains("\"direct_policy_service_access\":false"));
            assert!(result
                .stdout
                .contains("\"evaluated_via\":\"sdk_overgate_contract\""));
            if !expected_reason.is_empty() {
                assert!(result.stdout.contains(expected_reason));
            }
        }

        for (target_ref, expected_state, expected_reason) in [
            ("package_accept", "accepted", ""),
            ("package_invalid", "invalid_package", "package.invalid"),
            (
                "package_unsupported_version",
                "unsupported_version",
                "package.unsupported_version",
            ),
            (
                "package_missing_provenance",
                "missing_provenance",
                "package.missing_provenance",
            ),
            (
                "package_policy_incompatible",
                "policy_incompatible",
                "package.policy_incompatible",
            ),
        ] {
            let args = args_with(
                &["package", "validate", "--json", "--target-ref", target_ref],
                LOCAL_PROFILE_ARGS,
            );
            let result = run_args(args);
            assert_eq!(result.exit_code, EXIT_SUCCESS);
            assert!(result.stdout.contains("\"package_validation_summary\""));
            assert!(result
                .stdout
                .contains(&format!("\"validation_state\":\"{expected_state}\"")));
            assert!(result.stdout.contains("\"schema_checked\":true"));
            assert!(result.stdout.contains("\"signature_checked\":true"));
            assert!(result.stdout.contains("\"hash_checked\":true"));
            assert!(result
                .stdout
                .contains("\"direct_package_store_access\":false"));
            if !expected_reason.is_empty() {
                assert!(result.stdout.contains(expected_reason));
            }
        }

        let usage = run_args(args_with(
            &["usage", "show", "--json", "--target-ref", "usage_disputed"],
            LOCAL_PROFILE_ARGS,
        ));
        assert_eq!(usage.exit_code, EXIT_SUCCESS);
        for unit in [
            "CPU-ORU", "GPU-ORU", "STOR-ORU", "NET-ORU", "MEM-ORU", "DATA-ORU",
        ] {
            assert!(usage.stdout.contains(unit));
        }
        assert!(usage.stdout.contains("\"disputed_usage\":true"));
        assert!(usage.stdout.contains("\"payment_behavior_created\":false"));
        assert!(usage.stdout.contains("\"direct_meter_access\":false"));

        let receipt = run_args(args_with(
            &["receipt", "show", "--json", "--target-ref", "receipt_local"],
            LOCAL_PROFILE_ARGS,
        ));
        assert_eq!(receipt.exit_code, EXIT_SUCCESS);
        assert!(receipt.stdout.contains("\"receipt_ledger_read\""));
        assert!(receipt.stdout.contains("seal-ledger:entry:receipt_local"));
        assert!(receipt.stdout.contains("overbill:receipt:receipt_local"));
        assert!(receipt
            .stdout
            .contains("\"pricing_assumptions_present\":false"));
        assert!(receipt
            .stdout
            .contains("\"revenue_assumptions_present\":false"));
        assert!(receipt
            .stdout
            .contains("\"customer_count_assumptions_present\":false"));
        assert!(receipt
            .stdout
            .contains("\"market_volume_assumptions_present\":false"));
        assert!(receipt.stdout.contains("\"direct_ledger_access\":false"));

        let ledger = run_args(args_with(
            &[
                "ledger",
                "inspect",
                "--json",
                "--target-ref",
                "ledger_local",
            ],
            LOCAL_PROFILE_ARGS,
        ));
        assert_eq!(ledger.exit_code, EXIT_SUCCESS);
        assert!(ledger.stdout.contains("\"command\":\"ledger inspect\""));
        assert!(ledger.stdout.contains("\"receipt_ledger_read\""));
        assert!(ledger.stdout.contains("\"direct_ledger_access\":false"));

        let dispute = run_args(args_with(
            &[
                "dispute",
                "inspect",
                "--json",
                "--target-ref",
                "dispute_resolved_released",
            ],
            LOCAL_PROFILE_ARGS,
        ));
        assert_eq!(dispute.exit_code, EXIT_SUCCESS);
        assert!(dispute.stdout.contains("\"dispute_read_model\""));
        assert!(dispute
            .stdout
            .contains("overclaim:case:dispute_resolved_released"));
        assert!(dispute
            .stdout
            .contains("overclaim:evidence:dispute_resolved_released"));
        assert!(dispute.stdout.contains("\"hold_status\":\"released\""));
        assert!(dispute.stdout.contains("\"resolution_state\":\"resolved\""));
        assert!(dispute.stdout.contains("\"tenant_role_filtered\":true"));
        assert!(dispute.stdout.contains("\"direct_dispute_mutation\":false"));
        assert!(dispute.stdout.contains("\"direct_ledger_mutation\":false"));
    }

    #[test]
    fn phase9_product_and_ci_workflows_emit_authorized_recipes() {
        let docdex = run_args(args_with(
            &[
                "workload",
                "submit",
                "--json",
                "--workload-kind",
                "docdex_encrypted_index",
                "--workload-ref",
                "workload_docdex_index",
                "--target-ref",
                "docdex_index",
            ],
            LOCAL_PROFILE_ARGS,
        ));
        assert_eq!(docdex.exit_code, EXIT_SUCCESS);
        assert!(docdex.stdout.contains("\"product_workflow_recipe\""));
        assert!(docdex.stdout.contains("\"product\":\"docdex\""));
        assert!(docdex.stdout.contains("encrypted_index_ref"));
        assert!(docdex.stdout.contains("overrid workload cancel"));
        assert!(docdex.stdout.contains("overrid usage show"));
        assert!(docdex.stdout.contains("overrid receipt show"));
        assert!(docdex.stdout.contains("\"sdk_overgate_only\":true"));
        assert!(docdex
            .stdout
            .contains("\"direct_internal_api_access\":false"));
        assert!(docdex.stdout.contains("\"raw_http_required\":false"));
        assert!(docdex.stdout.contains("\"authorized_refs_only\":true"));

        let mcoda = run_args(args_with(
            &[
                "workload",
                "status",
                "--json",
                "--workload-kind",
                "mcoda_agent_workload",
                "--workload-ref",
                "workload_mcoda_agent",
                "--target-ref",
                "mcoda_model_metadata",
            ],
            LOCAL_PROFILE_ARGS,
        ));
        assert_eq!(mcoda.exit_code, EXIT_SUCCESS);
        assert!(mcoda.stdout.contains("\"product\":\"mcoda\""));
        assert!(mcoda.stdout.contains("dynamic_model_metadata_ref"));
        assert!(mcoda.stdout.contains("resource_metadata_ref"));
        assert!(mcoda.stdout.contains("tool_boundary_ref"));
        assert!(mcoda
            .stdout
            .contains("\"dynamic_model_resource_selection\":true"));
        assert!(mcoda
            .stdout
            .contains("\"hardcoded_model_or_provider\":false"));
        assert!(mcoda.stdout.contains("\"hardcoded_node_assumption\":false"));
        assert!(mcoda.stdout.contains("\"paid_service_assumption\":false"));

        let codali = run_args(args_with(
            &[
                "workload",
                "result",
                "--json",
                "--workload-kind",
                "codali_code_agent_package",
                "--workload-ref",
                "workload_codali_package",
                "--target-ref",
                "codali_repo_context",
            ],
            LOCAL_PROFILE_ARGS,
        ));
        assert_eq!(codali.exit_code, EXIT_SUCCESS);
        assert!(codali.stdout.contains("\"product\":\"codali\""));
        assert!(codali.stdout.contains("repository_context_ref"));
        assert!(codali.stdout.contains("artifact_refs"));
        assert!(codali.stdout.contains("repair_boundary_ref"));
        assert!(codali.stdout.contains("phase_usage_ref"));
        assert!(codali.stdout.contains("policy.resource_denied"));
        assert!(codali.stdout.contains("\"direct_storage_access\":false"));
        assert!(codali.stdout.contains("\"execution_result\""));

        let ci = run_args(args_with(
            &[
                "workload",
                "submit",
                "--json",
                "--workload-kind",
                "docdex_encrypted_index",
                "--workload-ref",
                "workload_docdex_ci",
                "--target-ref",
                "docdex_ci_index",
            ],
            CI_PROFILE_ARGS,
        ));
        assert_eq!(ci.exit_code, EXIT_SUCCESS);
        assert!(ci.stdout.contains("\"ci_automation_profile\""));
        assert!(ci.stdout.contains("\"profile_kind\":\"ci\""));
        assert!(ci.stdout.contains("\"environment_class\":\"ci\""));
        assert!(ci
            .stdout
            .contains("\"credential_reference_class\":\"ci_reference\""));
        assert!(ci.stdout.contains("mounted_secret_ref"));
        assert!(ci
            .stdout
            .contains("\"ambient_persistent_keychain_allowed\":false"));
        assert!(ci
            .stdout
            .contains("\"requires_non_interactive_confirmation\":true"));
        assert!(ci.stdout.contains("\"json_output_stable\":true"));
        assert!(ci.stdout.contains("\"branch_on_exit_class\":true"));
    }

    #[test]
    fn seed_mutation_requires_confirm_profile() {
        let result = run_args([
            "overrid",
            "credential",
            "enroll",
            "--json",
            "--profile",
            "seed",
            "--environment",
            "seed",
            "--endpoint",
            "https://overgate.seed.overrid.local",
            "--endpoint-fingerprint",
            "fp_seed",
            "--tenant",
            "tenant_seed",
            "--actor",
            "actor_seed",
            "--credential-namespace",
            "seed",
            "--credential-class",
            "keychain",
            "--credential-ref",
            "keychain://overrid/seed/key-1",
            "--key-id",
            "seed-key-1",
        ]);
        assert_eq!(result.exit_code, EXIT_USAGE);
        assert!(result
            .stdout
            .contains("\"reason_code\":\"missing_profile_confirmation\""));
    }

    #[test]
    fn revoked_credential_returns_credential_error_json() {
        let mut args = args_with(&["credential", "inspect", "--json"], LOCAL_PROFILE_ARGS);
        args.push("--revoked");

        let result = run_args(args);

        assert_eq!(result.exit_code, EXIT_CREDENTIAL);
        assert!(result
            .stdout
            .contains("\"reason_code\":\"credential_validation_failed\""));
        assert!(result.stdout.contains("credential is revoked"));
    }
}
