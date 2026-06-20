use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    Human,
    Json,
}

impl OutputMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Json => "json",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalOptions {
    pub output: OutputMode,
    pub no_color: bool,
    pub verbose: bool,
    pub all_phases: bool,
    pub phase: Option<u8>,
    pub selection_service: Option<String>,
    pub selection_tag: Option<String>,
    pub selection_changed_path: Option<String>,
    pub selection_required_dependency: Option<String>,
    pub selection_gate_class: Option<String>,
    pub selection_scenario_name: Option<String>,
    pub profile: Option<String>,
    pub environment: Option<String>,
    pub endpoint: Option<String>,
    pub endpoint_fingerprint: Option<String>,
    pub tenant: Option<String>,
    pub actor: Option<String>,
    pub credential_namespace: Option<String>,
    pub credential_class: Option<String>,
    pub credential_ref: Option<String>,
    pub key_id: Option<String>,
    pub fixture_allowance: Option<String>,
    pub confirmation_policy: Option<String>,
    pub schema_pin: Option<String>,
    pub endpoint_override: Option<String>,
    pub confirm_profile: bool,
    pub test_harness_profile: bool,
    pub revoked: bool,
    pub expired: bool,
    pub reason: Option<String>,
    pub trace_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub new_idempotency_key: bool,
    pub expected_state: Option<String>,
    pub target_ref: Option<String>,
    pub manifest_kind: Option<String>,
    pub manifest_ref: Option<String>,
    pub workload_ref: Option<String>,
    pub workload_kind: Option<String>,
    pub timeout_ms: Option<u64>,
    pub poll_interval_ms: Option<u64>,
    pub max_retries: Option<u8>,
    pub wait: bool,
    pub follow: bool,
    pub dry_run: bool,
}

impl Default for GlobalOptions {
    fn default() -> Self {
        Self {
            output: OutputMode::Human,
            no_color: false,
            verbose: false,
            all_phases: false,
            phase: None,
            selection_service: None,
            selection_tag: None,
            selection_changed_path: None,
            selection_required_dependency: None,
            selection_gate_class: None,
            selection_scenario_name: None,
            profile: None,
            environment: None,
            endpoint: None,
            endpoint_fingerprint: None,
            tenant: None,
            actor: None,
            credential_namespace: None,
            credential_class: None,
            credential_ref: None,
            key_id: None,
            fixture_allowance: None,
            confirmation_policy: None,
            schema_pin: None,
            endpoint_override: None,
            confirm_profile: false,
            test_harness_profile: false,
            revoked: false,
            expired: false,
            reason: None,
            trace_id: None,
            idempotency_key: None,
            new_idempotency_key: false,
            expected_state: None,
            target_ref: None,
            manifest_kind: None,
            manifest_ref: None,
            workload_ref: None,
            workload_kind: None,
            timeout_ms: None,
            poll_interval_ms: None,
            max_retries: None,
            wait: false,
            follow: false,
            dry_run: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
    Doctor,
    Profile(ProfileCommand),
    Credential(CredentialCommand),
    Auth(AuthCommand),
    Tenant(TenantCommand),
    Identity(IdentityCommand),
    Key(KeyCommand),
    Manifest(ManifestCommand),
    Node(NodeCommand),
    Workload(WorkloadCommand),
    Policy(PolicyCommand),
    Package(PackageCommand),
    Usage(UsageCommand),
    Receipt(ReceiptCommand),
    Ledger(LedgerCommand),
    Dispute(DisputeCommand),
    IdempotencyCache(IdempotencyCacheCommand),
    Dev(DevCommand),
    Test(TestCommand),
    Planned(PlannedCommand),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevCommand {
    Start,
    Stop,
    Restart,
    Status,
    Reset,
    Seed,
    Smoke,
    Logs,
    Doctor,
}

impl DevCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "dev start",
            Self::Stop => "dev stop",
            Self::Restart => "dev restart",
            Self::Status => "dev status",
            Self::Reset => "dev reset",
            Self::Seed => "dev seed",
            Self::Smoke => "dev smoke",
            Self::Logs => "dev logs",
            Self::Doctor => "dev doctor",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileCommand {
    Create,
    List,
    Select,
    Inspect,
    Reset,
}

impl ProfileCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Create => "profile create",
            Self::List => "profile list",
            Self::Select => "profile select",
            Self::Inspect => "profile inspect",
            Self::Reset => "profile reset",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialCommand {
    Enroll,
    Inspect,
}

impl CredentialCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enroll => "credential enroll",
            Self::Inspect => "credential inspect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthCommand {
    Login,
    Whoami,
}

impl AuthCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Login => "auth login",
            Self::Whoami => "auth whoami",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantCommand {
    Create,
    List,
    Inspect,
    Suspend,
}

impl TenantCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Create => "tenant create",
            Self::List => "tenant list",
            Self::Inspect => "tenant inspect",
            Self::Suspend => "tenant suspend",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityCommand {
    Create,
    List,
    Inspect,
    Disable,
}

impl IdentityCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Create => "identity create",
            Self::List => "identity list",
            Self::Inspect => "identity inspect",
            Self::Disable => "identity disable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCommand {
    Enroll,
    List,
    Rotate,
    Revoke,
}

impl KeyCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enroll => "key enroll",
            Self::List => "key list",
            Self::Rotate => "key rotate",
            Self::Revoke => "key revoke",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestCommand {
    Validate,
    Submit,
    Inspect,
}

impl ManifestCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Validate => "manifest validate",
            Self::Submit => "manifest submit",
            Self::Inspect => "manifest inspect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadCommand {
    Submit,
    Status,
    Timeline,
    Logs,
    Cancel,
    Result,
    Follow,
}

impl WorkloadCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Submit => "workload submit",
            Self::Status => "workload status",
            Self::Timeline => "workload timeline",
            Self::Logs => "workload logs",
            Self::Cancel => "workload cancel",
            Self::Result => "workload result",
            Self::Follow => "workload follow",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeCommand {
    Register,
    Inspect,
    Health,
}

impl NodeCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Register => "node register",
            Self::Inspect => "node inspect",
            Self::Health => "node health",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyCommand {
    DryRun,
}

impl PolicyCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DryRun => "policy dry-run",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageCommand {
    Validate,
}

impl PackageCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Validate => "package validate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsageCommand {
    Show,
}

impl UsageCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Show => "usage show",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptCommand {
    Show,
}

impl ReceiptCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Show => "receipt show",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerCommand {
    Inspect,
}

impl LedgerCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Inspect => "ledger inspect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisputeCommand {
    List,
    Inspect,
}

impl DisputeCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::List => "dispute list",
            Self::Inspect => "dispute inspect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdempotencyCacheCommand {
    Inspect,
    Reset,
}

impl IdempotencyCacheCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Inspect => "idempotency-cache inspect",
            Self::Reset => "idempotency-cache reset",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestCommand {
    Integration,
    Scenario { name: String },
    List,
    Reset,
    Artifacts { run_id: String },
}

impl TestCommand {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlannedCommand {
    Package,
    FederationPublicInterest,
    Governance,
    ReleaseReadiness,
}

impl PlannedCommand {
    pub fn command_name(self) -> &'static str {
        match self {
            PlannedCommand::Package => "deployment",
            PlannedCommand::FederationPublicInterest => "federation/public-interest/purpose-tag",
            PlannedCommand::Governance => "governance/incident/compliance",
            PlannedCommand::ReleaseReadiness => "release-readiness",
        }
    }

    pub fn phase_gate(self) -> &'static str {
        match self {
            PlannedCommand::Package => "phase_9",
            PlannedCommand::FederationPublicInterest => "phase_10",
            PlannedCommand::Governance => "phase_7_or_phase_13",
            PlannedCommand::ReleaseReadiness => "phase_10",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCli {
    pub globals: GlobalOptions,
    pub command: Command,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliParseError {
    MissingFlagValue(&'static str),
    UnknownFlag(String),
    UnknownCommand(String),
    ConflictingOutputMode,
    InvalidOutputMode(String),
    InvalidNumericFlag(&'static str, String),
}

impl fmt::Display for CliParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFlagValue(flag) => write!(formatter, "{flag} requires a value"),
            Self::UnknownFlag(flag) => write!(formatter, "unknown flag: {flag}"),
            Self::UnknownCommand(command) => write!(formatter, "unknown command: {command}"),
            Self::ConflictingOutputMode => {
                formatter.write_str("--json conflicts with --output human")
            }
            Self::InvalidOutputMode(mode) => write!(formatter, "invalid output mode: {mode}"),
            Self::InvalidNumericFlag(flag, value) => {
                write!(formatter, "invalid numeric value for {flag}: {value}")
            }
        }
    }
}

impl std::error::Error for CliParseError {}

pub fn parse_cli<I, S>(args: I) -> Result<ParsedCli, CliParseError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut globals = GlobalOptions::default();
    let mut command_tokens = Vec::new();
    let mut explicit_output_mode = None;
    let mut iter = args.into_iter().map(Into::into).peekable();

    let _program_name = iter.next();

    while let Some(token) = iter.next() {
        match token.as_str() {
            "--help" | "-h" => command_tokens.push("help".to_owned()),
            "--json" => {
                set_output_mode(&mut globals, &mut explicit_output_mode, OutputMode::Json)?;
            }
            "--no-color" => globals.no_color = true,
            "--verbose" | "-v" => globals.verbose = true,
            "--all-phases" => globals.all_phases = true,
            "--confirm-profile" => globals.confirm_profile = true,
            "--test-harness-profile" => globals.test_harness_profile = true,
            "--revoked" => globals.revoked = true,
            "--expired" => globals.expired = true,
            "--dry-run" => globals.dry_run = true,
            "--new-idempotency-key" => globals.new_idempotency_key = true,
            "--wait" => globals.wait = true,
            "--follow" => globals.follow = true,
            "--profile" => globals.profile = Some(next_value(&mut iter, "--profile")?),
            "--phase" => {
                let value = next_value(&mut iter, "--phase")?;
                globals.phase = Some(parse_numeric_flag("--phase", &value)?);
            }
            "--service" => globals.selection_service = Some(next_value(&mut iter, "--service")?),
            "--tag" => globals.selection_tag = Some(next_value(&mut iter, "--tag")?),
            "--changed-path" => {
                globals.selection_changed_path = Some(next_value(&mut iter, "--changed-path")?)
            }
            "--required-dependency" => {
                globals.selection_required_dependency =
                    Some(next_value(&mut iter, "--required-dependency")?)
            }
            "--gate-class" => {
                globals.selection_gate_class = Some(next_value(&mut iter, "--gate-class")?)
            }
            "--scenario-name" => {
                globals.selection_scenario_name = Some(next_value(&mut iter, "--scenario-name")?)
            }
            "--environment" => globals.environment = Some(next_value(&mut iter, "--environment")?),
            "--endpoint" => globals.endpoint = Some(next_value(&mut iter, "--endpoint")?),
            "--endpoint-fingerprint" => {
                globals.endpoint_fingerprint =
                    Some(next_value(&mut iter, "--endpoint-fingerprint")?)
            }
            "--tenant" => globals.tenant = Some(next_value(&mut iter, "--tenant")?),
            "--actor" => globals.actor = Some(next_value(&mut iter, "--actor")?),
            "--credential-namespace" => {
                globals.credential_namespace =
                    Some(next_value(&mut iter, "--credential-namespace")?)
            }
            "--credential-class" => {
                globals.credential_class = Some(next_value(&mut iter, "--credential-class")?)
            }
            "--credential-ref" => {
                globals.credential_ref = Some(next_value(&mut iter, "--credential-ref")?)
            }
            "--key-id" => globals.key_id = Some(next_value(&mut iter, "--key-id")?),
            "--fixture-allowance" => {
                globals.fixture_allowance = Some(next_value(&mut iter, "--fixture-allowance")?)
            }
            "--confirmation-policy" => {
                globals.confirmation_policy = Some(next_value(&mut iter, "--confirmation-policy")?)
            }
            "--schema-pin" => globals.schema_pin = Some(next_value(&mut iter, "--schema-pin")?),
            "--endpoint-override" => {
                globals.endpoint_override = Some(next_value(&mut iter, "--endpoint-override")?)
            }
            "--reason" => globals.reason = Some(next_value(&mut iter, "--reason")?),
            "--trace-id" => globals.trace_id = Some(next_value(&mut iter, "--trace-id")?),
            "--idempotency-key" => {
                globals.idempotency_key = Some(next_value(&mut iter, "--idempotency-key")?)
            }
            "--expected-state" => {
                globals.expected_state = Some(next_value(&mut iter, "--expected-state")?)
            }
            "--target-ref" => globals.target_ref = Some(next_value(&mut iter, "--target-ref")?),
            "--manifest-kind" => {
                globals.manifest_kind = Some(next_value(&mut iter, "--manifest-kind")?)
            }
            "--manifest-ref" => {
                globals.manifest_ref = Some(next_value(&mut iter, "--manifest-ref")?)
            }
            "--workload-ref" => {
                globals.workload_ref = Some(next_value(&mut iter, "--workload-ref")?)
            }
            "--workload-kind" => {
                globals.workload_kind = Some(next_value(&mut iter, "--workload-kind")?)
            }
            "--timeout-ms" => {
                let value = next_value(&mut iter, "--timeout-ms")?;
                globals.timeout_ms = Some(parse_numeric_flag("--timeout-ms", &value)?);
            }
            "--timeout" => {
                let value = next_value(&mut iter, "--timeout")?;
                globals.timeout_ms = Some(parse_numeric_flag("--timeout", &value)?);
            }
            "--poll-interval" => {
                let value = next_value(&mut iter, "--poll-interval")?;
                globals.poll_interval_ms = Some(parse_numeric_flag("--poll-interval", &value)?);
            }
            "--max-retries" => {
                let value = next_value(&mut iter, "--max-retries")?;
                globals.max_retries = Some(parse_numeric_flag("--max-retries", &value)?);
            }
            "--output" => {
                let value = next_value(&mut iter, "--output")?;
                match value.as_str() {
                    "human" => {
                        set_output_mode(&mut globals, &mut explicit_output_mode, OutputMode::Human)?
                    }
                    "json" => {
                        set_output_mode(&mut globals, &mut explicit_output_mode, OutputMode::Json)?
                    }
                    _ => return Err(CliParseError::InvalidOutputMode(value)),
                }
            }
            flag if flag.starts_with('-') => return Err(CliParseError::UnknownFlag(token)),
            _ => command_tokens.push(token),
        }
    }

    let command = command_from_tokens(&command_tokens)?;
    Ok(ParsedCli { globals, command })
}

fn parse_numeric_flag<T>(flag: &'static str, value: &str) -> Result<T, CliParseError>
where
    T: std::str::FromStr,
{
    value
        .parse::<T>()
        .map_err(|_| CliParseError::InvalidNumericFlag(flag, value.to_owned()))
}

fn next_value<I>(iter: &mut I, flag: &'static str) -> Result<String, CliParseError>
where
    I: Iterator<Item = String>,
{
    iter.next().ok_or(CliParseError::MissingFlagValue(flag))
}

fn set_output_mode(
    globals: &mut GlobalOptions,
    explicit_output_mode: &mut Option<OutputMode>,
    next_mode: OutputMode,
) -> Result<(), CliParseError> {
    if explicit_output_mode.is_some_and(|previous| previous != next_mode) {
        return Err(CliParseError::ConflictingOutputMode);
    }
    *explicit_output_mode = Some(next_mode);
    globals.output = next_mode;
    Ok(())
}

fn command_from_tokens(tokens: &[String]) -> Result<Command, CliParseError> {
    if tokens.is_empty() {
        return Ok(Command::Help);
    }

    match tokens[0].as_str() {
        "help" => Ok(Command::Help),
        "version" => Ok(Command::Version),
        "doctor" => Ok(Command::Doctor),
        "profile" => profile_command(tokens),
        "credential" => credential_command(tokens),
        "auth" => auth_command(tokens),
        "tenant" => tenant_command(tokens),
        "identity" => identity_command(tokens),
        "key" => key_command(tokens),
        "manifest" => manifest_command(tokens),
        "node" => node_command(tokens),
        "workload" => workload_command(tokens),
        "idempotency" | "idempotency-cache" => idempotency_cache_command(tokens),
        "policy" => policy_command(tokens),
        "package" => package_command(tokens),
        "usage" => usage_command(tokens),
        "receipt" => receipt_command(tokens),
        "ledger" => ledger_command(tokens),
        "dispute" => dispute_command(tokens),
        "dev" => dev_command(tokens),
        "test" => test_command(tokens),
        "release-readiness" | "readiness" | "phase10" | "phase-10" => {
            Ok(Command::Planned(PlannedCommand::ReleaseReadiness))
        }
        "deploy" | "deployment" => Ok(Command::Planned(PlannedCommand::Package)),
        "federation"
        | "federation-template"
        | "federation-template-service"
        | "public-interest"
        | "public-interest-pool"
        | "purpose-tag"
        | "purpose-tags"
        | "purpose-tag-registry" => Ok(Command::Planned(PlannedCommand::FederationPublicInterest)),
        "governance" | "incident" | "compliance" | "migration" => {
            Ok(Command::Planned(PlannedCommand::Governance))
        }
        other => Err(CliParseError::UnknownCommand(other.to_owned())),
    }
}

fn dev_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("status") {
        "start" => Ok(Command::Dev(DevCommand::Start)),
        "stop" => Ok(Command::Dev(DevCommand::Stop)),
        "restart" => Ok(Command::Dev(DevCommand::Restart)),
        "status" => Ok(Command::Dev(DevCommand::Status)),
        "reset" => Ok(Command::Dev(DevCommand::Reset)),
        "seed" => Ok(Command::Dev(DevCommand::Seed)),
        "smoke" => Ok(Command::Dev(DevCommand::Smoke)),
        "logs" => Ok(Command::Dev(DevCommand::Logs)),
        "doctor" => Ok(Command::Dev(DevCommand::Doctor)),
        other => Err(CliParseError::UnknownCommand(format!("dev {other}"))),
    }
}

fn test_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("list") {
        "integration" => Ok(Command::Test(TestCommand::Integration)),
        "scenario" => match tokens.get(2) {
            Some(name) => Ok(Command::Test(TestCommand::Scenario { name: name.clone() })),
            None => Err(CliParseError::UnknownCommand("test scenario".to_owned())),
        },
        "list" => Ok(Command::Test(TestCommand::List)),
        "reset" => Ok(Command::Test(TestCommand::Reset)),
        "artifacts" => match tokens.get(2) {
            Some(run_id) => Ok(Command::Test(TestCommand::Artifacts {
                run_id: run_id.clone(),
            })),
            None => Err(CliParseError::UnknownCommand("test artifacts".to_owned())),
        },
        other => Err(CliParseError::UnknownCommand(format!("test {other}"))),
    }
}

fn auth_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("whoami") {
        "login" => Ok(Command::Auth(AuthCommand::Login)),
        "whoami" => Ok(Command::Auth(AuthCommand::Whoami)),
        other => Err(CliParseError::UnknownCommand(format!("auth {other}"))),
    }
}

fn tenant_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("inspect") {
        "create" => Ok(Command::Tenant(TenantCommand::Create)),
        "list" => Ok(Command::Tenant(TenantCommand::List)),
        "inspect" => Ok(Command::Tenant(TenantCommand::Inspect)),
        "suspend" => Ok(Command::Tenant(TenantCommand::Suspend)),
        other => Err(CliParseError::UnknownCommand(format!("tenant {other}"))),
    }
}

fn identity_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("inspect") {
        "create" => Ok(Command::Identity(IdentityCommand::Create)),
        "list" => Ok(Command::Identity(IdentityCommand::List)),
        "inspect" => Ok(Command::Identity(IdentityCommand::Inspect)),
        "disable" => Ok(Command::Identity(IdentityCommand::Disable)),
        other => Err(CliParseError::UnknownCommand(format!("identity {other}"))),
    }
}

fn key_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("list") {
        "enroll" => Ok(Command::Key(KeyCommand::Enroll)),
        "list" => Ok(Command::Key(KeyCommand::List)),
        "rotate" => Ok(Command::Key(KeyCommand::Rotate)),
        "revoke" => Ok(Command::Key(KeyCommand::Revoke)),
        other => Err(CliParseError::UnknownCommand(format!("key {other}"))),
    }
}

fn manifest_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("inspect") {
        "validate" => Ok(Command::Manifest(ManifestCommand::Validate)),
        "submit" => Ok(Command::Manifest(ManifestCommand::Submit)),
        "inspect" => Ok(Command::Manifest(ManifestCommand::Inspect)),
        other => Err(CliParseError::UnknownCommand(format!("manifest {other}"))),
    }
}

fn node_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("inspect") {
        "register" => Ok(Command::Node(NodeCommand::Register)),
        "inspect" => Ok(Command::Node(NodeCommand::Inspect)),
        "health" => Ok(Command::Node(NodeCommand::Health)),
        other => Err(CliParseError::UnknownCommand(format!("node {other}"))),
    }
}

fn workload_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("status") {
        "submit" => Ok(Command::Workload(WorkloadCommand::Submit)),
        "status" => Ok(Command::Workload(WorkloadCommand::Status)),
        "timeline" => Ok(Command::Workload(WorkloadCommand::Timeline)),
        "logs" => Ok(Command::Workload(WorkloadCommand::Logs)),
        "cancel" => Ok(Command::Workload(WorkloadCommand::Cancel)),
        "result" => Ok(Command::Workload(WorkloadCommand::Result)),
        "follow" => Ok(Command::Workload(WorkloadCommand::Follow)),
        other => Err(CliParseError::UnknownCommand(format!("workload {other}"))),
    }
}

fn idempotency_cache_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("inspect") {
        "inspect" => Ok(Command::IdempotencyCache(IdempotencyCacheCommand::Inspect)),
        "reset" => Ok(Command::IdempotencyCache(IdempotencyCacheCommand::Reset)),
        other => Err(CliParseError::UnknownCommand(format!(
            "idempotency-cache {other}"
        ))),
    }
}

fn policy_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("dry-run") {
        "dry-run" | "dryrun" => Ok(Command::Policy(PolicyCommand::DryRun)),
        other => Err(CliParseError::UnknownCommand(format!("policy {other}"))),
    }
}

fn package_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("validate") {
        "validate" => Ok(Command::Package(PackageCommand::Validate)),
        "build" | "deploy" | "release" => Ok(Command::Planned(PlannedCommand::Package)),
        other => Err(CliParseError::UnknownCommand(format!("package {other}"))),
    }
}

fn usage_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("show") {
        "show" => Ok(Command::Usage(UsageCommand::Show)),
        other => Err(CliParseError::UnknownCommand(format!("usage {other}"))),
    }
}

fn receipt_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("show") {
        "show" => Ok(Command::Receipt(ReceiptCommand::Show)),
        other => Err(CliParseError::UnknownCommand(format!("receipt {other}"))),
    }
}

fn ledger_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("inspect") {
        "inspect" => Ok(Command::Ledger(LedgerCommand::Inspect)),
        other => Err(CliParseError::UnknownCommand(format!("ledger {other}"))),
    }
}

fn dispute_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("list") {
        "list" => Ok(Command::Dispute(DisputeCommand::List)),
        "inspect" => Ok(Command::Dispute(DisputeCommand::Inspect)),
        other => Err(CliParseError::UnknownCommand(format!("dispute {other}"))),
    }
}

fn profile_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("inspect") {
        "create" => Ok(Command::Profile(ProfileCommand::Create)),
        "list" => Ok(Command::Profile(ProfileCommand::List)),
        "select" => Ok(Command::Profile(ProfileCommand::Select)),
        "inspect" => Ok(Command::Profile(ProfileCommand::Inspect)),
        "reset" => Ok(Command::Profile(ProfileCommand::Reset)),
        other => Err(CliParseError::UnknownCommand(format!("profile {other}"))),
    }
}

fn credential_command(tokens: &[String]) -> Result<Command, CliParseError> {
    match tokens.get(1).map(String::as_str).unwrap_or("inspect") {
        "enroll" => Ok(Command::Credential(CredentialCommand::Enroll)),
        "inspect" => Ok(Command::Credential(CredentialCommand::Inspect)),
        other => Err(CliParseError::UnknownCommand(format!("credential {other}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_version_with_global_flags() {
        let parsed = parse_cli([
            "overrid",
            "version",
            "--json",
            "--no-color",
            "--verbose",
            "--profile",
            "local-dev",
        ])
        .unwrap();
        assert_eq!(parsed.command, Command::Version);
        assert_eq!(parsed.globals.output, OutputMode::Json);
        assert!(parsed.globals.no_color);
        assert!(parsed.globals.verbose);
        assert_eq!(parsed.globals.profile.as_deref(), Some("local-dev"));
    }

    #[test]
    fn rejects_missing_profile_value() {
        assert_eq!(
            parse_cli(["overrid", "version", "--profile"]).unwrap_err(),
            CliParseError::MissingFlagValue("--profile")
        );
    }

    #[test]
    fn rejects_conflicting_output_modes() {
        assert_eq!(
            parse_cli(["overrid", "version", "--json", "--output", "human"]).unwrap_err(),
            CliParseError::ConflictingOutputMode
        );
        assert_eq!(
            parse_cli(["overrid", "version", "--output", "human", "--json"]).unwrap_err(),
            CliParseError::ConflictingOutputMode
        );
    }

    #[test]
    fn maps_phase7_node_command() {
        let parsed = parse_cli(["overrid", "node", "register"]).unwrap();
        assert_eq!(parsed.command, Command::Node(NodeCommand::Register));
    }

    #[test]
    fn maps_phase10_federation_public_interest_commands() {
        for command in [
            "federation",
            "federation-template",
            "public-interest",
            "public-interest-pool",
            "purpose-tag",
            "purpose-tag-registry",
        ] {
            let parsed = parse_cli(["overrid", command]).unwrap();
            assert_eq!(
                parsed.command,
                Command::Planned(PlannedCommand::FederationPublicInterest)
            );
        }
    }

    #[test]
    fn maps_doctor_command() {
        let parsed = parse_cli(["overrid", "doctor", "--json"]).unwrap();
        assert_eq!(parsed.command, Command::Doctor);
        assert_eq!(parsed.globals.output, OutputMode::Json);
    }

    #[test]
    fn maps_local_stack_dev_commands() {
        for (subcommand, expected) in [
            ("start", DevCommand::Start),
            ("stop", DevCommand::Stop),
            ("restart", DevCommand::Restart),
            ("status", DevCommand::Status),
            ("reset", DevCommand::Reset),
            ("seed", DevCommand::Seed),
            ("smoke", DevCommand::Smoke),
            ("logs", DevCommand::Logs),
            ("doctor", DevCommand::Doctor),
        ] {
            let parsed = parse_cli(["overrid", "dev", subcommand, "--json"]).unwrap();
            assert_eq!(parsed.command, Command::Dev(expected));
            assert_eq!(parsed.globals.output, OutputMode::Json);
            assert_eq!(expected.as_str(), format!("dev {subcommand}"));
        }

        let default_status = parse_cli(["overrid", "dev"]).unwrap();
        assert_eq!(default_status.command, Command::Dev(DevCommand::Status));
    }

    #[test]
    fn parses_profile_phase3_flags() {
        let parsed = parse_cli([
            "overrid",
            "profile",
            "inspect",
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
            "--fixture-allowance",
            "local_only",
            "--confirm-profile",
            "--reason",
            "test run",
        ])
        .unwrap();
        assert_eq!(parsed.command, Command::Profile(ProfileCommand::Inspect));
        assert!(parsed.globals.confirm_profile);
        assert_eq!(parsed.globals.reason.as_deref(), Some("test run"));
    }

    #[test]
    fn maps_credential_inspect_command() {
        let parsed = parse_cli(["overrid", "credential", "inspect"]).unwrap();
        assert_eq!(
            parsed.command,
            Command::Credential(CredentialCommand::Inspect)
        );
    }

    #[test]
    fn maps_phase5_bootstrap_commands() {
        assert_eq!(
            parse_cli(["overrid", "auth", "whoami"]).unwrap().command,
            Command::Auth(AuthCommand::Whoami)
        );
        assert_eq!(
            parse_cli(["overrid", "tenant", "create"]).unwrap().command,
            Command::Tenant(TenantCommand::Create)
        );
        assert_eq!(
            parse_cli(["overrid", "identity", "disable"])
                .unwrap()
                .command,
            Command::Identity(IdentityCommand::Disable)
        );
        assert_eq!(
            parse_cli(["overrid", "key", "rotate"]).unwrap().command,
            Command::Key(KeyCommand::Rotate)
        );
        assert_eq!(
            parse_cli(["overrid", "manifest", "submit"])
                .unwrap()
                .command,
            Command::Manifest(ManifestCommand::Submit)
        );
        assert_eq!(
            parse_cli(["overrid", "workload", "submit"])
                .unwrap()
                .command,
            Command::Workload(WorkloadCommand::Submit)
        );
        assert_eq!(
            parse_cli(["overrid", "workload", "logs"]).unwrap().command,
            Command::Workload(WorkloadCommand::Logs)
        );
        assert_eq!(
            parse_cli(["overrid", "idempotency-cache", "inspect"])
                .unwrap()
                .command,
            Command::IdempotencyCache(IdempotencyCacheCommand::Inspect)
        );
    }

    #[test]
    fn parses_phase5_bootstrap_flags() {
        let parsed = parse_cli([
            "overrid",
            "tenant",
            "create",
            "--trace-id",
            "trace_cli_custom",
            "--idempotency-key",
            "idem_custom",
            "--expected-state",
            "absent",
            "--target-ref",
            "tenant_local",
            "--manifest-kind",
            "workload",
            "--manifest-ref",
            "manifest_local",
            "--workload-kind",
            "synthetic",
            "--workload-ref",
            "workload_local",
            "--new-idempotency-key",
            "--timeout-ms",
            "4500",
            "--poll-interval",
            "250",
            "--wait",
            "--follow",
            "--max-retries",
            "3",
            "--dry-run",
        ])
        .unwrap();

        assert_eq!(parsed.command, Command::Tenant(TenantCommand::Create));
        assert_eq!(parsed.globals.trace_id.as_deref(), Some("trace_cli_custom"));
        assert_eq!(
            parsed.globals.idempotency_key.as_deref(),
            Some("idem_custom")
        );
        assert_eq!(parsed.globals.expected_state.as_deref(), Some("absent"));
        assert_eq!(parsed.globals.target_ref.as_deref(), Some("tenant_local"));
        assert_eq!(parsed.globals.manifest_kind.as_deref(), Some("workload"));
        assert_eq!(
            parsed.globals.manifest_ref.as_deref(),
            Some("manifest_local")
        );
        assert_eq!(parsed.globals.workload_kind.as_deref(), Some("synthetic"));
        assert_eq!(
            parsed.globals.workload_ref.as_deref(),
            Some("workload_local")
        );
        assert!(parsed.globals.new_idempotency_key);
        assert_eq!(parsed.globals.timeout_ms, Some(4500));
        assert_eq!(parsed.globals.poll_interval_ms, Some(250));
        assert!(parsed.globals.wait);
        assert!(parsed.globals.follow);
        assert_eq!(parsed.globals.max_retries, Some(3));
        assert!(parsed.globals.dry_run);
    }

    #[test]
    fn rejects_invalid_phase6_numeric_flags() {
        assert!(matches!(
            parse_cli(["overrid", "tenant", "create", "--timeout-ms", "soon"]),
            Err(CliParseError::InvalidNumericFlag("--timeout-ms", value)) if value == "soon"
        ));
        assert!(matches!(
            parse_cli(["overrid", "workload", "logs", "--poll-interval", "fast"]),
            Err(CliParseError::InvalidNumericFlag("--poll-interval", value)) if value == "fast"
        ));
    }

    #[test]
    fn maps_phase7_execution_commands_and_timeout_alias() {
        let parsed = parse_cli([
            "overrid",
            "workload",
            "follow",
            "--timeout",
            "12000",
            "--poll-interval",
            "500",
            "--follow",
        ])
        .unwrap();
        assert_eq!(parsed.command, Command::Workload(WorkloadCommand::Follow));
        assert_eq!(parsed.globals.timeout_ms, Some(12000));
        assert_eq!(parsed.globals.poll_interval_ms, Some(500));
        assert!(parsed.globals.follow);
    }

    #[test]
    fn maps_phase8_policy_package_accounting_commands() {
        assert_eq!(
            parse_cli(["overrid", "policy", "dry-run"]).unwrap().command,
            Command::Policy(PolicyCommand::DryRun)
        );
        assert_eq!(
            parse_cli(["overrid", "package", "validate"])
                .unwrap()
                .command,
            Command::Package(PackageCommand::Validate)
        );
        assert_eq!(
            parse_cli(["overrid", "usage", "show"]).unwrap().command,
            Command::Usage(UsageCommand::Show)
        );
        assert_eq!(
            parse_cli(["overrid", "receipt", "show"]).unwrap().command,
            Command::Receipt(ReceiptCommand::Show)
        );
        assert_eq!(
            parse_cli(["overrid", "ledger", "inspect"]).unwrap().command,
            Command::Ledger(LedgerCommand::Inspect)
        );
        assert_eq!(
            parse_cli(["overrid", "dispute", "inspect"])
                .unwrap()
                .command,
            Command::Dispute(DisputeCommand::Inspect)
        );
        assert_eq!(
            parse_cli(["overrid", "deployment"]).unwrap().command,
            Command::Planned(PlannedCommand::Package)
        );
        assert_eq!(
            parse_cli(["overrid", "release-readiness"]).unwrap().command,
            Command::Planned(PlannedCommand::ReleaseReadiness)
        );
    }

    #[test]
    fn maps_integration_harness_test_commands() {
        assert_eq!(
            parse_cli(["overrid", "test", "integration"])
                .unwrap()
                .command,
            Command::Test(TestCommand::Integration)
        );
        assert_eq!(
            parse_cli(["overrid", "test", "scenario", "scenario_phase0_smoke"])
                .unwrap()
                .command,
            Command::Test(TestCommand::Scenario {
                name: "scenario_phase0_smoke".to_owned()
            })
        );
        assert_eq!(
            parse_cli(["overrid", "test", "list", "--phase", "0"])
                .unwrap()
                .command,
            Command::Test(TestCommand::List)
        );
        assert_eq!(
            parse_cli(["overrid", "test", "reset"]).unwrap().command,
            Command::Test(TestCommand::Reset)
        );
        assert_eq!(
            parse_cli(["overrid", "test", "artifacts", "run_phase0_smoke"])
                .unwrap()
                .command,
            Command::Test(TestCommand::Artifacts {
                run_id: "run_phase0_smoke".to_owned()
            })
        );
        let parsed = parse_cli(["overrid", "test", "list", "--phase", "0"]).unwrap();
        assert_eq!(parsed.globals.phase, Some(0));

        let parsed = parse_cli([
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
        ])
        .unwrap();
        assert_eq!(parsed.globals.phase, Some(9));
        assert_eq!(
            parsed.globals.selection_service.as_deref(),
            Some("service:overgate")
        );
        assert_eq!(
            parsed.globals.selection_tag.as_deref(),
            Some("control_plane_spine")
        );
        assert_eq!(
            parsed.globals.selection_changed_path.as_deref(),
            Some("services/overgate/routes.rs")
        );
        assert_eq!(
            parsed.globals.selection_required_dependency.as_deref(),
            Some("fixture:phase9_control_plane_spine")
        );
        assert_eq!(
            parsed.globals.selection_gate_class.as_deref(),
            Some("contract_spine")
        );
        assert_eq!(
            parsed.globals.selection_scenario_name.as_deref(),
            Some("scenario_phase1_control_plane_spine")
        );
    }
}
