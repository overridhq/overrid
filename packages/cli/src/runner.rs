use overrid_contracts::{
    BootstrapAcceptanceRecord, BootstrapCommandFamily, CanonicalIdempotencyFingerprint, CliProfile,
    ConfirmationPolicy, CredentialReference, CredentialReferenceClass, EnvironmentClass,
    ErrorDecodeRecord, ExecutionDiagnosticEvent, ExecutionLogBundle, ExecutionResultRef,
    ExecutionTimeline, ExitCodeClass, FixtureAllowance, LocalIdempotencyCacheRecord,
    ManifestBootstrapRef, NodeState, NodeStatusRecord, PollingPlan, ProfileValidationError,
    RetryClass, RetryTimeoutPolicy, SignedCommandEnvelope, SyntheticWorkloadPendingState,
    WorkloadExecutionState, SUPPORTED_SCHEMA_VERSION,
};
use overrid_sdk::{
    decode_phase6_error, enforce_profile_environment, retry_timeout_policy, CommandSafetyInput,
    SdkError,
};

use crate::build_metadata::{human_version_lines, version_info};
use crate::parser::{
    parse_cli, AuthCommand, Command, CredentialCommand, GlobalOptions, IdempotencyCacheCommand,
    IdentityCommand, KeyCommand, ManifestCommand, NodeCommand, OutputMode, PlannedCommand,
    ProfileCommand, TenantCommand, WorkloadCommand,
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
        "  idempotency-cache inspect|reset".to_owned(),
        "  auth login|whoami".to_owned(),
        "  tenant create|list|inspect|suspend".to_owned(),
        "  identity create|list|inspect|disable".to_owned(),
        "  key enroll|list|rotate|revoke".to_owned(),
        "  manifest validate|submit|inspect".to_owned(),
        "  node register|inspect|health".to_owned(),
        "  workload submit|status|timeline|logs|cancel|result|follow".to_owned(),
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
                    "\"acceptance\":{}",
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
                ",\"synthetic_workload_pending_state\":{}{}{}",
                render_workload_pending_json(&workload),
                event_suffix,
                execution_suffix
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
    fn normal_help_shows_phase7_commands() {
        let result = run_args(["overrid", "help"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("node register|inspect|health"));
        assert!(result
            .stdout
            .contains("workload submit|status|timeline|logs|cancel|result|follow"));
    }

    #[test]
    fn all_phases_help_documents_phase_gated_commands() {
        let result = run_args(["overrid", "help", "--all-phases"]);
        assert_eq!(result.exit_code, EXIT_SUCCESS);
        assert!(result.stdout.contains("node register|inspect|health"));
        assert!(result.stdout.contains("policy dry-run"));
        assert!(result
            .stdout
            .contains("governance|incident|compliance|migration"));
    }

    #[test]
    fn planned_command_fails_with_stable_phase_reason() {
        let result = run_args(["overrid", "policy", "dry-run", "--json"]);
        assert_eq!(result.exit_code, EXIT_NOT_AVAILABLE_IN_PHASE);
        assert!(result
            .stdout
            .contains("\"reason_code\":\"not_available_in_phase\""));
        assert!(result.stdout.contains("\"phase_gate\":\"phase_4\""));
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
