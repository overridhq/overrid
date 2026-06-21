use std::fmt;

use crate::{
    check_sdk_compatibility, SdkCompatibilityRejection, SdkError, SDK_CURRENT_STABLE_MAJOR,
};
use overrid_contracts::{EnvironmentClass, SUPPORTED_SCHEMA_VERSION};

pub const SDK_PHASE8_CAPABILITY_PROFILE: &str =
    "phase8-fixtures-contract-tests-validation-artifacts";
pub const SDK_PHASE8_FIXTURE_ROOT: &str = "packages/sdk/fixtures/phase8";
pub const SDK_PHASE8_LOCAL_FIXTURE_SET_PATH: &str =
    "packages/sdk/fixtures/phase8/local_sdk_fixture_set.valid.json";
pub const SDK_PHASE8_GOLDEN_CORPUS_PATH: &str =
    "packages/sdk/fixtures/phase8/golden_cross_language_corpus.valid.json";
pub const SDK_PHASE8_VALIDATION_ARTIFACTS_PATH: &str =
    "packages/sdk/fixtures/phase8/validation_artifacts_manifest.valid.json";
pub const SDK_PHASE8_REDACTION_PROFILE: &str = "phase8_redacted_test_fixture_refs_only";
pub const SDK_PHASE8_DETERMINISTIC_SEED: &str = "sdk-phase8-local-seed-v0";
pub const SDK_PHASE8_RESET_MARKER: &str = "local-dev-resettable-fixture-set";

const LOCAL_FIXTURE_SET_JSON: &str =
    include_str!("../fixtures/phase8/local_sdk_fixture_set.valid.json");
const GOLDEN_CORPUS_JSON: &str =
    include_str!("../fixtures/phase8/golden_cross_language_corpus.valid.json");
const VALIDATION_ARTIFACTS_JSON: &str =
    include_str!("../fixtures/phase8/validation_artifacts_manifest.valid.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase8FixtureKind {
    Tenant,
    Actor,
    Credential,
    CommandEnvelope,
    Manifest,
    Signature,
    IdempotencyEntry,
    Error,
    UsageRef,
    AuditRef,
}

impl SdkPhase8FixtureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Tenant => "tenant",
            Self::Actor => "actor",
            Self::Credential => "credential",
            Self::CommandEnvelope => "command_envelope",
            Self::Manifest => "manifest",
            Self::Signature => "signature",
            Self::IdempotencyEntry => "idempotency_entry",
            Self::Error => "error",
            Self::UsageRef => "usage_ref",
            Self::AuditRef => "audit_ref",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase8LocalFixtureRecord {
    pub fixture_id: String,
    pub kind: SdkPhase8FixtureKind,
    pub value_ref: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub schema_version: String,
    pub environment: EnvironmentClass,
    pub deterministic_seed: &'static str,
    pub reset_marker: &'static str,
    pub redaction_profile: &'static str,
    pub production_default: bool,
    pub contains_private_material: bool,
    pub contains_raw_payload: bool,
}

impl SdkPhase8LocalFixtureRecord {
    pub fn new(
        kind: SdkPhase8FixtureKind,
        fixture_id: impl Into<String>,
        value_ref: impl Into<String>,
    ) -> Result<Self, SdkPhase8Error> {
        let schema_version =
            check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, SUPPORTED_SCHEMA_VERSION)?;
        let fixture_id = fixture_id.into();
        let value_ref = value_ref.into();
        require_phase8_non_empty(&fixture_id, "fixture id")?;
        require_phase8_non_empty(&value_ref, "fixture value ref")?;

        Ok(Self {
            fixture_id,
            kind,
            value_ref,
            tenant_id: "tenant:local-fixture".to_owned(),
            actor_id: "actor:local-developer".to_owned(),
            schema_version: schema_version.raw().to_owned(),
            environment: EnvironmentClass::Local,
            deterministic_seed: SDK_PHASE8_DETERMINISTIC_SEED,
            reset_marker: SDK_PHASE8_RESET_MARKER,
            redaction_profile: SDK_PHASE8_REDACTION_PROFILE,
            production_default: false,
            contains_private_material: false,
            contains_raw_payload: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase8LocalFixtureCorpus {
    pub corpus_id: &'static str,
    pub fixture_root: &'static str,
    pub artifact_path: &'static str,
    pub deterministic_seed: &'static str,
    pub reset_marker: &'static str,
    pub records: Vec<SdkPhase8LocalFixtureRecord>,
    pub redaction_profile: &'static str,
    pub production_defaults_allowed: bool,
}

pub fn sdk_phase8_local_fixture_corpus() -> Result<SdkPhase8LocalFixtureCorpus, SdkPhase8Error> {
    let fixtures = [
        (
            SdkPhase8FixtureKind::Tenant,
            "tenant_local_fixture",
            "tenant:local-fixture",
        ),
        (
            SdkPhase8FixtureKind::Actor,
            "actor_local_developer",
            "actor:local-developer",
        ),
        (
            SdkPhase8FixtureKind::Credential,
            "credential_redacted_fixture_ref",
            "credential_ref:redacted-local-fixture",
        ),
        (
            SdkPhase8FixtureKind::CommandEnvelope,
            "command_envelope_fixture",
            "command:submit-synthetic-workload",
        ),
        (
            SdkPhase8FixtureKind::Manifest,
            "manifest_fixture",
            "manifest:synthetic-cpu-batch",
        ),
        (
            SdkPhase8FixtureKind::Signature,
            "signature_fixture_ref",
            "signature_ref:redacted-ed25519-fixture",
        ),
        (
            SdkPhase8FixtureKind::IdempotencyEntry,
            "idempotency_fixture",
            "idempotency:sdk-phase8-command",
        ),
        (
            SdkPhase8FixtureKind::Error,
            "error_fixture",
            "error:unsupported_schema_version",
        ),
        (
            SdkPhase8FixtureKind::UsageRef,
            "usage_ref_fixture",
            "overmeter:usage-rollup-fixture",
        ),
        (
            SdkPhase8FixtureKind::AuditRef,
            "audit_ref_fixture",
            "overwatch:audit-fixture",
        ),
    ];

    let records = fixtures
        .into_iter()
        .map(|(kind, fixture_id, value_ref)| {
            SdkPhase8LocalFixtureRecord::new(kind, fixture_id, value_ref)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(SdkPhase8LocalFixtureCorpus {
        corpus_id: "sdk_phase8_local_fixture_corpus",
        fixture_root: SDK_PHASE8_FIXTURE_ROOT,
        artifact_path: SDK_PHASE8_LOCAL_FIXTURE_SET_PATH,
        deterministic_seed: SDK_PHASE8_DETERMINISTIC_SEED,
        reset_marker: SDK_PHASE8_RESET_MARKER,
        records,
        redaction_profile: SDK_PHASE8_REDACTION_PROFILE,
        production_defaults_allowed: false,
    })
}

pub fn validate_phase8_local_fixture_corpus(
    corpus: &SdkPhase8LocalFixtureCorpus,
) -> Result<(), SdkPhase8Error> {
    if corpus.production_defaults_allowed {
        return Err(SdkPhase8Error::UnsafeFixture(
            "phase8 fixtures cannot be production defaults",
        ));
    }
    require_phase8_non_empty(corpus.deterministic_seed, "deterministic seed")?;
    require_phase8_non_empty(corpus.reset_marker, "reset marker")?;
    require_phase8_non_empty(corpus.redaction_profile, "redaction profile")?;

    for required in REQUIRED_FIXTURE_KINDS {
        if !corpus.records.iter().any(|record| record.kind == *required) {
            return Err(SdkPhase8Error::MissingFixtureKind(required.as_str()));
        }
    }

    for record in &corpus.records {
        require_phase8_non_empty(&record.fixture_id, "fixture id")?;
        require_phase8_non_empty(&record.value_ref, "fixture value ref")?;
        require_phase8_non_empty(&record.tenant_id, "tenant id")?;
        require_phase8_non_empty(&record.actor_id, "actor id")?;
        reject_phase8_sensitive_value(&record.value_ref)?;
        if record.environment != EnvironmentClass::Local
            || record.production_default
            || record.contains_private_material
            || record.contains_raw_payload
        {
            return Err(SdkPhase8Error::UnsafeFixture(record.kind.as_str()));
        }
        if record.deterministic_seed != corpus.deterministic_seed
            || record.reset_marker != corpus.reset_marker
            || record.redaction_profile != corpus.redaction_profile
        {
            return Err(SdkPhase8Error::UnsafeFixture(
                "fixture corpus metadata drift",
            ));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase8ContractTestKind {
    SignedCommandSubmission,
    DuplicateIdempotency,
    StableErrorPreservation,
    StatusRead,
}

impl SdkPhase8ContractTestKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SignedCommandSubmission => "signed_command_submission",
            Self::DuplicateIdempotency => "duplicate_idempotency",
            Self::StableErrorPreservation => "stable_error_preservation",
            Self::StatusRead => "status_read",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase8ContractTestDescriptor {
    pub name: &'static str,
    pub kind: SdkPhase8ContractTestKind,
    pub route: &'static str,
    pub method: &'static str,
    pub public_api_only: bool,
    pub local_stack_contract: bool,
    pub uses_internal_service_mock: bool,
    pub owning_services_available: bool,
    pub blocker: Option<&'static str>,
    pub assertions: &'static [&'static str],
}

pub fn sdk_phase8_contract_tests(
    owning_services_available: bool,
) -> Vec<SdkPhase8ContractTestDescriptor> {
    let blocker = if owning_services_available {
        None
    } else {
        Some("blocked until local stack exposes the owning public Overgate/control-plane service")
    };

    vec![
        SdkPhase8ContractTestDescriptor {
            name: "phase8_signed_command_submission_public_overgate",
            kind: SdkPhase8ContractTestKind::SignedCommandSubmission,
            route: "/v1/overgate/commands",
            method: "POST",
            public_api_only: true,
            local_stack_contract: true,
            uses_internal_service_mock: false,
            owning_services_available,
            blocker,
            assertions: &[
                "signed command reaches public Overgate route",
                "trace id is preserved",
                "audit ref is returned by service evidence",
            ],
        },
        SdkPhase8ContractTestDescriptor {
            name: "phase8_duplicate_idempotency_public_overgate",
            kind: SdkPhase8ContractTestKind::DuplicateIdempotency,
            route: "/v1/overgate/commands",
            method: "POST",
            public_api_only: true,
            local_stack_contract: true,
            uses_internal_service_mock: false,
            owning_services_available,
            blocker,
            assertions: &[
                "same idempotency key resolves duplicate response",
                "conflicting request hash stays a stable error",
                "terminal digest is preserved",
            ],
        },
        SdkPhase8ContractTestDescriptor {
            name: "phase8_stable_error_public_overgate",
            kind: SdkPhase8ContractTestKind::StableErrorPreservation,
            route: "/v1/overgate/commands",
            method: "POST",
            public_api_only: true,
            local_stack_contract: true,
            uses_internal_service_mock: false,
            owning_services_available,
            blocker,
            assertions: &[
                "reason code is preserved",
                "trace id is preserved",
                "correction fields are not masked",
            ],
        },
        SdkPhase8ContractTestDescriptor {
            name: "phase8_status_read_public_control_plane",
            kind: SdkPhase8ContractTestKind::StatusRead,
            route: "/v1/control-plane/commands/status",
            method: "GET",
            public_api_only: true,
            local_stack_contract: true,
            uses_internal_service_mock: false,
            owning_services_available,
            blocker,
            assertions: &[
                "status reads use public control-plane route",
                "pending and terminal states are service-returned",
                "pagination and audit refs are preserved",
            ],
        },
    ]
}

pub fn validate_phase8_contract_tests(
    tests: &[SdkPhase8ContractTestDescriptor],
) -> Result<(), SdkPhase8Error> {
    for required in REQUIRED_CONTRACT_TEST_KINDS {
        if !tests.iter().any(|test| test.kind == *required) {
            return Err(SdkPhase8Error::MissingContractTest(required.as_str()));
        }
    }

    for test in tests {
        if !test.public_api_only
            || !test.local_stack_contract
            || test.uses_internal_service_mock
            || !(test.route.starts_with("/v1/overgate/")
                || test.route.starts_with("/v1/control-plane/"))
        {
            return Err(SdkPhase8Error::ContractTestBypassesPublicApi(test.name));
        }
        if !test.owning_services_available && test.blocker.is_none() {
            return Err(SdkPhase8Error::ContractTestMissingBlocker(test.name));
        }
        if test.assertions.is_empty() {
            return Err(SdkPhase8Error::ContractTestMissingAssertion(test.name));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase8GoldenFixtureKind {
    RequestEnvelope,
    CanonicalSigningInput,
    ResponseError,
    ManifestValidation,
    IdempotencyCase,
    RedactionCase,
}

impl SdkPhase8GoldenFixtureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequestEnvelope => "request_envelope",
            Self::CanonicalSigningInput => "canonical_signing_input",
            Self::ResponseError => "response_error",
            Self::ManifestValidation => "manifest_validation",
            Self::IdempotencyCase => "idempotency_case",
            Self::RedactionCase => "redaction_case",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase8GoldenFixtureDescriptor {
    pub case_name: &'static str,
    pub kind: SdkPhase8GoldenFixtureKind,
    pub path: &'static str,
    pub schema_version: &'static str,
    pub rust_required: bool,
    pub typescript_web_release_blocked_until_pass: bool,
    pub later_bindings_blocked_until_pass: bool,
    pub content_marker: &'static str,
}

pub fn sdk_phase8_golden_fixtures() -> Vec<SdkPhase8GoldenFixtureDescriptor> {
    vec![
        golden_fixture(
            "golden_request_envelope",
            SdkPhase8GoldenFixtureKind::RequestEnvelope,
        ),
        golden_fixture(
            "golden_canonical_signing_input",
            SdkPhase8GoldenFixtureKind::CanonicalSigningInput,
        ),
        golden_fixture(
            "golden_response_error",
            SdkPhase8GoldenFixtureKind::ResponseError,
        ),
        golden_fixture(
            "golden_manifest_validation",
            SdkPhase8GoldenFixtureKind::ManifestValidation,
        ),
        golden_fixture(
            "golden_idempotency_case",
            SdkPhase8GoldenFixtureKind::IdempotencyCase,
        ),
        golden_fixture(
            "golden_redaction_case",
            SdkPhase8GoldenFixtureKind::RedactionCase,
        ),
    ]
}

pub fn validate_phase8_golden_fixtures(
    fixtures: &[SdkPhase8GoldenFixtureDescriptor],
) -> Result<(), SdkPhase8Error> {
    for required in REQUIRED_GOLDEN_KINDS {
        if !fixtures.iter().any(|fixture| fixture.kind == *required) {
            return Err(SdkPhase8Error::GoldenCorpusIncomplete(required.as_str()));
        }
    }

    for fixture in fixtures {
        if fixture.path != SDK_PHASE8_GOLDEN_CORPUS_PATH
            || fixture.schema_version != SUPPORTED_SCHEMA_VERSION
            || !fixture.rust_required
            || !fixture.typescript_web_release_blocked_until_pass
            || !fixture.later_bindings_blocked_until_pass
            || !GOLDEN_CORPUS_JSON.contains(fixture.content_marker)
        {
            return Err(SdkPhase8Error::GoldenCorpusIncomplete(fixture.case_name));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase8SecurityRedactionCheck {
    pub name: &'static str,
    pub production_test_separation: bool,
    pub fake_signer_isolated: bool,
    pub secret_refs_redacted: bool,
    pub rejects_raw_request_bodies: bool,
    pub rejects_signature_values: bool,
    pub rejects_private_payload_logs: bool,
    pub fixture_credentials_reference_only: bool,
    pub credential_lifecycle_failure_checked: bool,
    pub unsupported_schema_checked: bool,
    pub unsafe_downgrade_checked: bool,
}

pub fn sdk_phase8_security_redaction_checks() -> Vec<SdkPhase8SecurityRedactionCheck> {
    vec![
        SdkPhase8SecurityRedactionCheck {
            name: "phase8_production_test_separation",
            production_test_separation: true,
            fake_signer_isolated: true,
            secret_refs_redacted: true,
            rejects_raw_request_bodies: true,
            rejects_signature_values: true,
            rejects_private_payload_logs: true,
            fixture_credentials_reference_only: true,
            credential_lifecycle_failure_checked: true,
            unsupported_schema_checked: true,
            unsafe_downgrade_checked: true,
        },
        SdkPhase8SecurityRedactionCheck {
            name: "phase8_redacted_validation_artifacts",
            production_test_separation: true,
            fake_signer_isolated: true,
            secret_refs_redacted: true,
            rejects_raw_request_bodies: true,
            rejects_signature_values: true,
            rejects_private_payload_logs: true,
            fixture_credentials_reference_only: true,
            credential_lifecycle_failure_checked: true,
            unsupported_schema_checked: true,
            unsafe_downgrade_checked: true,
        },
    ]
}

pub fn validate_phase8_security_redaction_checks(
    checks: &[SdkPhase8SecurityRedactionCheck],
) -> Result<(), SdkPhase8Error> {
    if checks.is_empty() {
        return Err(SdkPhase8Error::SecurityCheckIncomplete(
            "missing phase8 security checks",
        ));
    }

    for check in checks {
        if !check.production_test_separation
            || !check.fake_signer_isolated
            || !check.secret_refs_redacted
            || !check.rejects_raw_request_bodies
            || !check.rejects_signature_values
            || !check.rejects_private_payload_logs
            || !check.fixture_credentials_reference_only
            || !check.credential_lifecycle_failure_checked
            || !check.unsupported_schema_checked
            || !check.unsafe_downgrade_checked
        {
            return Err(SdkPhase8Error::SecurityCheckIncomplete(check.name));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkPhase8ValidationArtifactKind {
    SchemaGeneration,
    ContractTests,
    SigningGoldenChecks,
    IdempotencyBehavior,
    RedactionChecks,
    CompatibilityChecks,
    DocsAlignment,
}

impl SdkPhase8ValidationArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SchemaGeneration => "schema_generation",
            Self::ContractTests => "contract_tests",
            Self::SigningGoldenChecks => "signing_golden_checks",
            Self::IdempotencyBehavior => "idempotency_behavior",
            Self::RedactionChecks => "redaction_checks",
            Self::CompatibilityChecks => "compatibility_checks",
            Self::DocsAlignment => "docs_alignment",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase8ValidationArtifact {
    pub name: &'static str,
    pub kind: SdkPhase8ValidationArtifactKind,
    pub path: &'static str,
    pub retention_rule: &'static str,
    pub docdex_index_expected: bool,
    pub overwatch_runtime_event: bool,
    pub progress_evidence_required: bool,
}

pub fn sdk_phase8_validation_artifacts() -> Vec<SdkPhase8ValidationArtifact> {
    vec![
        phase8_artifact(
            "phase8_schema_generation_artifact",
            SdkPhase8ValidationArtifactKind::SchemaGeneration,
        ),
        phase8_artifact(
            "phase8_contract_test_artifact",
            SdkPhase8ValidationArtifactKind::ContractTests,
        ),
        phase8_artifact(
            "phase8_signing_golden_artifact",
            SdkPhase8ValidationArtifactKind::SigningGoldenChecks,
        ),
        phase8_artifact(
            "phase8_idempotency_artifact",
            SdkPhase8ValidationArtifactKind::IdempotencyBehavior,
        ),
        phase8_artifact(
            "phase8_redaction_artifact",
            SdkPhase8ValidationArtifactKind::RedactionChecks,
        ),
        phase8_artifact(
            "phase8_compatibility_artifact",
            SdkPhase8ValidationArtifactKind::CompatibilityChecks,
        ),
        phase8_artifact(
            "phase8_docs_alignment_artifact",
            SdkPhase8ValidationArtifactKind::DocsAlignment,
        ),
    ]
}

pub fn validate_phase8_validation_artifacts(
    artifacts: &[SdkPhase8ValidationArtifact],
) -> Result<(), SdkPhase8Error> {
    for required in REQUIRED_ARTIFACT_KINDS {
        if !artifacts.iter().any(|artifact| artifact.kind == *required) {
            return Err(SdkPhase8Error::ValidationArtifactInvalid(required.as_str()));
        }
    }

    for artifact in artifacts {
        if artifact.path != SDK_PHASE8_VALIDATION_ARTIFACTS_PATH
            || artifact.retention_rule.trim().is_empty()
            || !artifact.docdex_index_expected
            || artifact.overwatch_runtime_event
            || !artifact.progress_evidence_required
            || !VALIDATION_ARTIFACTS_JSON.contains(artifact.name)
        {
            return Err(SdkPhase8Error::ValidationArtifactInvalid(artifact.name));
        }
    }

    Ok(())
}

pub fn validate_phase8_fixture_artifact_files() -> Result<(), SdkPhase8Error> {
    validate_fixture_artifact_text(
        SDK_PHASE8_LOCAL_FIXTURE_SET_PATH,
        LOCAL_FIXTURE_SET_JSON,
        &[
            "sdk_phase8_local_fixture_corpus",
            "tenant_local_fixture",
            "credential_redacted_fixture_ref",
            "signature_ref:redacted-ed25519-fixture",
            "test_only",
            "deterministic_seed",
            "local-dev-resettable-fixture-set",
        ],
    )?;
    validate_fixture_artifact_text(
        SDK_PHASE8_GOLDEN_CORPUS_PATH,
        GOLDEN_CORPUS_JSON,
        &[
            "golden_request_envelope",
            "golden_canonical_signing_input",
            "golden_response_error",
            "golden_manifest_validation",
            "golden_idempotency_case",
            "golden_redaction_case",
            "typescript_web_release_blocked_until_pass",
        ],
    )?;
    validate_fixture_artifact_text(
        SDK_PHASE8_VALIDATION_ARTIFACTS_PATH,
        VALIDATION_ARTIFACTS_JSON,
        &[
            "phase8_schema_generation_artifact",
            "phase8_contract_test_artifact",
            "phase8_signing_golden_artifact",
            "phase8_idempotency_artifact",
            "phase8_redaction_artifact",
            "phase8_compatibility_artifact",
            "phase8_docs_alignment_artifact",
            "docdex_index_expected",
            "overwatch_runtime_event",
        ],
    )?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkPhase8Error {
    MissingFixtureKind(&'static str),
    MissingContractTest(&'static str),
    MissingRequiredField(&'static str),
    UnsafeFixture(&'static str),
    ArtifactMissingMarker(&'static str),
    ArtifactSensitiveValue(&'static str),
    ContractTestBypassesPublicApi(&'static str),
    ContractTestMissingBlocker(&'static str),
    ContractTestMissingAssertion(&'static str),
    GoldenCorpusIncomplete(&'static str),
    SecurityCheckIncomplete(&'static str),
    ValidationArtifactInvalid(&'static str),
    Compatibility(String),
}

impl fmt::Display for SdkPhase8Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFixtureKind(kind) => {
                write!(formatter, "missing Phase 8 fixture kind: {kind}")
            }
            Self::MissingContractTest(name) => {
                write!(formatter, "missing Phase 8 contract test: {name}")
            }
            Self::MissingRequiredField(field) => {
                write!(formatter, "missing Phase 8 required field: {field}")
            }
            Self::UnsafeFixture(reason) => write!(formatter, "unsafe Phase 8 fixture: {reason}"),
            Self::ArtifactMissingMarker(marker) => {
                write!(formatter, "Phase 8 artifact missing marker: {marker}")
            }
            Self::ArtifactSensitiveValue(value) => {
                write!(
                    formatter,
                    "Phase 8 artifact has sensitive value marker: {value}"
                )
            }
            Self::ContractTestBypassesPublicApi(name) => {
                write!(
                    formatter,
                    "Phase 8 contract test bypasses public API: {name}"
                )
            }
            Self::ContractTestMissingBlocker(name) => {
                write!(formatter, "Phase 8 contract test missing blocker: {name}")
            }
            Self::ContractTestMissingAssertion(name) => {
                write!(formatter, "Phase 8 contract test missing assertion: {name}")
            }
            Self::GoldenCorpusIncomplete(name) => {
                write!(formatter, "Phase 8 golden corpus incomplete: {name}")
            }
            Self::SecurityCheckIncomplete(name) => {
                write!(formatter, "Phase 8 security check incomplete: {name}")
            }
            Self::ValidationArtifactInvalid(name) => {
                write!(formatter, "Phase 8 validation artifact invalid: {name}")
            }
            Self::Compatibility(message) => {
                write!(formatter, "Phase 8 compatibility error: {message}")
            }
        }
    }
}

impl std::error::Error for SdkPhase8Error {}

impl From<SdkError> for SdkPhase8Error {
    fn from(error: SdkError) -> Self {
        Self::Compatibility(error.to_string())
    }
}

impl From<SdkCompatibilityRejection> for SdkPhase8Error {
    fn from(error: SdkCompatibilityRejection) -> Self {
        Self::Compatibility(format!("{error:?}"))
    }
}

fn golden_fixture(
    case_name: &'static str,
    kind: SdkPhase8GoldenFixtureKind,
) -> SdkPhase8GoldenFixtureDescriptor {
    SdkPhase8GoldenFixtureDescriptor {
        case_name,
        kind,
        path: SDK_PHASE8_GOLDEN_CORPUS_PATH,
        schema_version: SUPPORTED_SCHEMA_VERSION,
        rust_required: true,
        typescript_web_release_blocked_until_pass: true,
        later_bindings_blocked_until_pass: true,
        content_marker: case_name,
    }
}

fn phase8_artifact(
    name: &'static str,
    kind: SdkPhase8ValidationArtifactKind,
) -> SdkPhase8ValidationArtifact {
    SdkPhase8ValidationArtifact {
        name,
        kind,
        path: SDK_PHASE8_VALIDATION_ARTIFACTS_PATH,
        retention_rule: "keep latest CI artifact plus progress evidence",
        docdex_index_expected: true,
        overwatch_runtime_event: false,
        progress_evidence_required: true,
    }
}

fn validate_fixture_artifact_text(
    path: &'static str,
    text: &str,
    required_markers: &[&'static str],
) -> Result<(), SdkPhase8Error> {
    for marker in required_markers {
        if !text.contains(marker) {
            return Err(SdkPhase8Error::ArtifactMissingMarker(marker));
        }
    }
    for forbidden in PHASE8_FORBIDDEN_ARTIFACT_MARKERS {
        if text.to_ascii_lowercase().contains(forbidden) {
            return Err(SdkPhase8Error::ArtifactSensitiveValue(path));
        }
    }
    Ok(())
}

fn require_phase8_non_empty(value: &str, field: &'static str) -> Result<(), SdkPhase8Error> {
    if value.trim().is_empty() {
        return Err(SdkPhase8Error::MissingRequiredField(field));
    }
    Ok(())
}

fn reject_phase8_sensitive_value(value: &str) -> Result<(), SdkPhase8Error> {
    let value = value.to_ascii_lowercase();
    for forbidden in PHASE8_FORBIDDEN_ARTIFACT_MARKERS {
        if value.contains(forbidden) {
            return Err(SdkPhase8Error::ArtifactSensitiveValue(forbidden));
        }
    }
    Ok(())
}

const REQUIRED_FIXTURE_KINDS: &[SdkPhase8FixtureKind] = &[
    SdkPhase8FixtureKind::Tenant,
    SdkPhase8FixtureKind::Actor,
    SdkPhase8FixtureKind::Credential,
    SdkPhase8FixtureKind::CommandEnvelope,
    SdkPhase8FixtureKind::Manifest,
    SdkPhase8FixtureKind::Signature,
    SdkPhase8FixtureKind::IdempotencyEntry,
    SdkPhase8FixtureKind::Error,
    SdkPhase8FixtureKind::UsageRef,
    SdkPhase8FixtureKind::AuditRef,
];

const REQUIRED_CONTRACT_TEST_KINDS: &[SdkPhase8ContractTestKind] = &[
    SdkPhase8ContractTestKind::SignedCommandSubmission,
    SdkPhase8ContractTestKind::DuplicateIdempotency,
    SdkPhase8ContractTestKind::StableErrorPreservation,
    SdkPhase8ContractTestKind::StatusRead,
];

const REQUIRED_GOLDEN_KINDS: &[SdkPhase8GoldenFixtureKind] = &[
    SdkPhase8GoldenFixtureKind::RequestEnvelope,
    SdkPhase8GoldenFixtureKind::CanonicalSigningInput,
    SdkPhase8GoldenFixtureKind::ResponseError,
    SdkPhase8GoldenFixtureKind::ManifestValidation,
    SdkPhase8GoldenFixtureKind::IdempotencyCase,
    SdkPhase8GoldenFixtureKind::RedactionCase,
];

const REQUIRED_ARTIFACT_KINDS: &[SdkPhase8ValidationArtifactKind] = &[
    SdkPhase8ValidationArtifactKind::SchemaGeneration,
    SdkPhase8ValidationArtifactKind::ContractTests,
    SdkPhase8ValidationArtifactKind::SigningGoldenChecks,
    SdkPhase8ValidationArtifactKind::IdempotencyBehavior,
    SdkPhase8ValidationArtifactKind::RedactionChecks,
    SdkPhase8ValidationArtifactKind::CompatibilityChecks,
    SdkPhase8ValidationArtifactKind::DocsAlignment,
];

const PHASE8_FORBIDDEN_ARTIFACT_MARKERS: &[&str] = &[
    "raw_private_key",
    "private_key_value",
    "bearer_token_value",
    "seed_phrase_value",
    "signature_value",
    "raw_request_body",
    "private_payload_value",
    "fixture_credential_material",
    "production_default\": true",
    "overwatch_runtime_event\": true",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase8_local_fixtures_are_deterministic_redacted_resettable_and_non_production() {
        let corpus = sdk_phase8_local_fixture_corpus().unwrap();
        validate_phase8_local_fixture_corpus(&corpus).unwrap();

        assert_eq!(corpus.fixture_root, SDK_PHASE8_FIXTURE_ROOT);
        assert_eq!(corpus.records.len(), REQUIRED_FIXTURE_KINDS.len());
        assert!(corpus
            .records
            .iter()
            .all(|record| record.schema_version == SUPPORTED_SCHEMA_VERSION));
        assert!(corpus
            .records
            .iter()
            .all(|record| !record.production_default && !record.contains_private_material));
        assert!(crate::SDK_SUPPORTED_SCHEMA_VERSIONS.contains(&SUPPORTED_SCHEMA_VERSION));
    }

    #[test]
    fn phase8_contract_tests_target_public_local_stack_routes_and_record_owner_blockers() {
        let tests = sdk_phase8_contract_tests(false);
        validate_phase8_contract_tests(&tests).unwrap();

        assert!(tests.iter().all(|test| test.public_api_only));
        assert!(tests.iter().all(|test| !test.uses_internal_service_mock));
        assert!(tests.iter().all(|test| test.blocker.is_some()));
    }

    #[test]
    fn phase8_golden_fixtures_gate_cross_language_bindings_on_rust_corpus() {
        let fixtures = sdk_phase8_golden_fixtures();
        validate_phase8_golden_fixtures(&fixtures).unwrap();

        assert_eq!(fixtures.len(), REQUIRED_GOLDEN_KINDS.len());
        assert!(fixtures.iter().all(|fixture| fixture.rust_required
            && fixture.typescript_web_release_blocked_until_pass
            && fixture.later_bindings_blocked_until_pass));
    }

    #[test]
    fn phase8_security_redaction_checks_block_secret_bearing_artifacts() {
        let checks = sdk_phase8_security_redaction_checks();
        validate_phase8_security_redaction_checks(&checks).unwrap();

        assert!(checks.iter().all(|check| check.fake_signer_isolated));
        assert!(checks
            .iter()
            .all(|check| check.rejects_signature_values && check.rejects_raw_request_bodies));
    }

    #[test]
    fn phase8_validation_artifacts_are_docdex_indexed_not_runtime_events() {
        let artifacts = sdk_phase8_validation_artifacts();
        validate_phase8_validation_artifacts(&artifacts).unwrap();

        assert_eq!(artifacts.len(), REQUIRED_ARTIFACT_KINDS.len());
        assert!(artifacts
            .iter()
            .all(|artifact| artifact.docdex_index_expected && !artifact.overwatch_runtime_event));
    }

    #[test]
    fn phase8_fixture_artifact_files_match_declared_paths_and_markers() {
        validate_phase8_fixture_artifact_files().unwrap();
        assert!(LOCAL_FIXTURE_SET_JSON.contains(SDK_PHASE8_DETERMINISTIC_SEED));
        assert!(GOLDEN_CORPUS_JSON.contains(crate::SDK_NAME));
        assert!(GOLDEN_CORPUS_JSON.contains(crate::SDK_VERSION));
        assert!(GOLDEN_CORPUS_JSON.contains(crate::SDK_LANGUAGE_BINDING));
        assert!(VALIDATION_ARTIFACTS_JSON.contains(SDK_PHASE8_CAPABILITY_PROFILE));
    }
}
