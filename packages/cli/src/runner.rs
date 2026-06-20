use overrid_contracts::{
    BootstrapAcceptanceRecord, BootstrapCommandFamily, CliProfile, ConfirmationPolicy,
    CredentialReference, CredentialReferenceClass, EnvironmentClass, ExitCodeClass,
    FixtureAllowance, ManifestBootstrapRef, ProfileValidationError, RetryClass,
    SignedCommandEnvelope, SyntheticWorkloadPendingState, SUPPORTED_SCHEMA_VERSION,
};
use overrid_sdk::{enforce_profile_environment, CommandSafetyInput, SdkError};

use crate::build_metadata::{human_version_lines, version_info};
use crate::parser::{
    parse_cli, AuthCommand, Command, CredentialCommand, GlobalOptions, IdentityCommand, KeyCommand,
    ManifestCommand, OutputMode, PlannedCommand, ProfileCommand, TenantCommand, WorkloadCommand,
};

const LOCAL_TRACE_ID: &str = "trace_cli_local";
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
            Command::Profile(command) => profile_command_result(command, &parsed.globals),
            Command::Credential(command) => credential_command_result(command, &parsed.globals),
            Command::Auth(command) => auth_command_result(command, &parsed.globals),
            Command::Tenant(command) => tenant_command_result(command, &parsed.globals),
            Command::Identity(command) => identity_command_result(command, &parsed.globals),
            Command::Key(command) => key_command_result(command, &parsed.globals),
            Command::Manifest(command) => manifest_command_result(command, &parsed.globals),
            Command::Workload(command) => workload_command_result(command, &parsed.globals),
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

fn workload_command_result(command: WorkloadCommand, globals: &GlobalOptions) -> CliRunResult {
    let target_ref = default_target_ref(
        globals,
        globals.workload_ref.as_deref().unwrap_or("workload_local"),
    );
    let result_kind = match command {
        WorkloadCommand::Submit => BootstrapResultKind::WorkloadSubmit,
        WorkloadCommand::Status => BootstrapResultKind::WorkloadStatus,
        WorkloadCommand::Timeline => BootstrapResultKind::WorkloadTimeline,
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
    target_ref: String,
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
    let idempotency_key = globals.idempotency_key.clone().unwrap_or_else(|| {
        deterministic_idempotency_key(&profile, command_name, target_ref.as_str())
    });
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
        target_ref,
        signed_envelope,
        audit_refs,
    })
}

fn planned_command_result(command: PlannedCommand, globals: &GlobalOptions) -> CliRunResult {
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
        "  version                         print CLI, SDK, and schema compatibility metadata".to_owned(),
        "  doctor                          print redacted local diagnostics and capability status".to_owned(),
        "  profile create|list|select|inspect|reset".to_owned(),
        "  credential enroll|inspect".to_owned(),
        "  auth login|whoami".to_owned(),
        "  tenant create|list|inspect|suspend".to_owned(),
        "  identity create|list|inspect|disable".to_owned(),
        "  key enroll|list|rotate|revoke".to_owned(),
        "  manifest validate|submit|inspect".to_owned(),
        "  workload submit|status|timeline".to_owned(),
        "  help                            print command help".to_owned(),
        "".to_owned(),
        "global flags:".to_owned(),
        "  --json                          render stable JSON output".to_owned(),
        "  --output MODE                   human or json".to_owned(),
        "  --no-color                      disable color".to_owned(),
        "  --verbose                       include local diagnostic detail".to_owned(),
        "  --profile NAME                  select a local profile".to_owned(),
        "  --environment CLASS             local, seed, staging, production_like, or ci".to_owned(),
        "  --endpoint URL                  Overgate endpoint".to_owned(),
        "  --endpoint-fingerprint VALUE    pinned endpoint identity".to_owned(),
        "  --credential-class CLASS        keychain, secret_service, encrypted_store, signing_agent, hardware_token, fixture, or ci_reference".to_owned(),
        "  --credential-ref REF            secret-free credential reference".to_owned(),
        "  --confirm-profile               confirm sensitive profile use".to_owned(),
        "  --reason TEXT                   reason for admin-impacting operations".to_owned(),
        "  --trace-id ID                   override local trace id".to_owned(),
        "  --idempotency-key KEY           override deterministic idempotency key".to_owned(),
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
            "  node register|inspect|health             phase_2".to_owned(),
            "  workload logs|cancel|result|follow       phase_3".to_owned(),
            "  policy dry-run                           phase_4".to_owned(),
            "  usage|receipt|dispute                    phase_5_or_phase_6".to_owned(),
            "  package validate and deployment helpers  phase_9".to_owned(),
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
    let error_json = format!(
        concat!(
            "{{",
            "\"reason_code\":\"{}\",",
            "\"message\":\"{}\",",
            "\"phase_gate\":\"{}\",",
            "\"retry_class\":\"{}\"",
            "}}"
        ),
        json_escape(reason_code),
        json_escape(message),
        json_escape(phase_gate),
        json_escape(retry_class.as_str()),
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

fn render_lifecycle_json(states: &[&str]) -> String {
    let terminal_state = states
        .iter()
        .rev()
        .copied()
        .find(|state| matches!(*state, "completed" | "denied" | "failed"));
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
                    "\"trace_id\":\"{}\",",
                    "\"signed\":{},",
                    "\"signature_ref\":{},",
                    "\"expected_state\":{},",
                    "\"reason\":{},",
                    "\"dry_run\":{},",
                    "\"submitted_via\":\"sdk_overgate_contract\",",
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
            format!(
                ",\"synthetic_workload_pending_state\":{}{}",
                render_workload_pending_json(&workload),
                event_suffix
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

    fn args_with<'a>(prefix: &[&'a str], suffix: &[&'a str]) -> Vec<&'a str> {
        let mut values = vec!["overrid"];
        values.extend_from_slice(prefix);
        values.extend_from_slice(suffix);
        values
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
    fn normal_help_hides_phase_gated_commands() {
        let result = run_args(["overrid", "help"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(!result.stdout.contains("node register"));
    }

    #[test]
    fn all_phases_help_documents_phase_gated_commands() {
        let result = run_args(["overrid", "help", "--all-phases"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("node register|inspect|health"));
        assert!(result
            .stdout
            .contains("governance|incident|compliance|migration"));
    }

    #[test]
    fn planned_command_fails_with_stable_phase_reason() {
        let result = run_args(["overrid", "node", "register", "--json"]);
        assert_eq!(result.exit_code, EXIT_NOT_AVAILABLE_IN_PHASE);
        assert!(result
            .stdout
            .contains("\"reason_code\":\"not_available_in_phase\""));
        assert!(result.stdout.contains("\"phase_gate\":\"phase_2\""));
        assert!(result.stdout.contains("\"exit_class\":\"phase\""));
        assert!(result.stdout.contains("\"fail_closed\":true"));
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
        assert!(first.stdout.contains("\"idempotency_key\":\"idem_local_tenant_local_actor_local_tenant_create_tenant_local\""));
        assert!(first
            .stdout
            .contains("\"audit_refs\":[\"audit_cli_bootstrap_tenant_create\"]"));
        assert!(first.stdout.contains("\"expected_state\":\"absent\""));
    }

    #[test]
    fn key_revoke_requires_reason_before_mutation() {
        let args = args_with(&["key", "revoke", "--json"], LOCAL_PROFILE_ARGS);
        let result = run_args(args);

        assert_eq!(result.exit_code, EXIT_USAGE);
        assert!(result.stdout.contains("\"reason_code\":\"missing_reason\""));
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
    }

    #[test]
    fn real_workload_logs_remain_phase_gated() {
        let result = run_args(["overrid", "workload", "logs", "--json"]);
        assert_eq!(result.exit_code, EXIT_NOT_AVAILABLE_IN_PHASE);
        assert!(result
            .stdout
            .contains("\"reason_code\":\"not_available_in_phase\""));
        assert!(result.stdout.contains("\"phase_gate\":\"phase_3\""));
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
