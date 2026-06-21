use std::fmt;

use overrid_contracts::{
    BootstrapCommandFamily, GENERATED_CONTRACT_STATUS, SUPPORTED_SCHEMA_VERSION,
};

pub const SDK_PHASE3_CAPABILITY_PROFILE: &str = "phase3-generated-rust-sdk-skeleton";
pub const SDK_PHASE3_GENERATED_MODELS_PATH: &str = "packages/sdk/src/generated.rs";
pub const SDK_PHASE3_HANDWRITTEN_CLIENT_PATH: &str = "packages/sdk/src/lib.rs";
pub const SDK_PHASE3_READ_HELPERS_PATH: &str = "packages/sdk/src/read.rs";
pub const SDK_PHASE3_SCHEMA_SOURCE_PATH: &str =
    "packages/schemas/overrid_contracts/v0/cli_command.schema.json";
pub const SDK_PHASE3_RUST_PROJECTION_SOURCE_PATH: &str =
    "packages/schemas/overrid_contracts/src/lib.rs";
pub const SDK_PHASE3_REASON_CODE_SOURCE_PATH: &str = "docs/specs/reason_codes_and_events.md";
pub const SDK_PHASE3_PHASE_GATE: &str = "phase_1_control_plane_bootstrap";
pub const SDK_PHASE3_GENERATED_CONTRACT_REVISION: &str = GENERATED_CONTRACT_STATUS;
pub const SDK_PHASE3_SCHEMA_SET: &[&str] = &[SUPPORTED_SCHEMA_VERSION];
pub const SDK_PHASE3_PUBLIC_ENTRYPOINTS: &[&str] = &[
    "OverridSdkClient",
    "configure_client",
    "sdk_version_report",
    "sdk_package_boundary",
    "sdk_generated_model_descriptors",
    "build_control_plane_read_request",
];

const PHASE3_MODEL_KINDS: &[SdkGeneratedModelKind] = &[
    SdkGeneratedModelKind::Phase1Command,
    SdkGeneratedModelKind::Tenant,
    SdkGeneratedModelKind::Identity,
    SdkGeneratedModelKind::KeyMetadata,
    SdkGeneratedModelKind::Manifest,
    SdkGeneratedModelKind::QueueStatus,
    SdkGeneratedModelKind::AuditRef,
    SdkGeneratedModelKind::Error,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkModuleOwnership {
    GeneratedContractProjection,
    HandwrittenClientOrchestration,
    HandwrittenReadHelper,
}

impl SdkModuleOwnership {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GeneratedContractProjection => "generated_contract_projection",
            Self::HandwrittenClientOrchestration => "handwritten_client_orchestration",
            Self::HandwrittenReadHelper => "handwritten_read_helper",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkModuleBoundary {
    pub path: &'static str,
    pub ownership: SdkModuleOwnership,
    pub source_authority: &'static str,
    pub public_entrypoints: &'static [&'static str],
    pub contract_authority: bool,
}

pub fn sdk_package_boundary() -> Vec<SdkModuleBoundary> {
    vec![
        SdkModuleBoundary {
            path: SDK_PHASE3_GENERATED_MODELS_PATH,
            ownership: SdkModuleOwnership::GeneratedContractProjection,
            source_authority: SDK_PHASE3_RUST_PROJECTION_SOURCE_PATH,
            public_entrypoints: &[
                "sdk_generated_model_descriptors",
                "validate_generated_model_descriptors",
            ],
            contract_authority: false,
        },
        SdkModuleBoundary {
            path: SDK_PHASE3_HANDWRITTEN_CLIENT_PATH,
            ownership: SdkModuleOwnership::HandwrittenClientOrchestration,
            source_authority: SDK_PHASE3_SCHEMA_SOURCE_PATH,
            public_entrypoints: &["OverridSdkClient", "configure_client", "sdk_version_report"],
            contract_authority: false,
        },
        SdkModuleBoundary {
            path: SDK_PHASE3_READ_HELPERS_PATH,
            ownership: SdkModuleOwnership::HandwrittenReadHelper,
            source_authority: SDK_PHASE3_SCHEMA_SOURCE_PATH,
            public_entrypoints: &[
                "build_control_plane_read_request",
                "control_plane_reader_capability",
            ],
            contract_authority: false,
        },
    ]
}

pub fn validate_sdk_package_boundary(
    boundary: &[SdkModuleBoundary],
) -> Result<(), SdkPhase3ValidationError> {
    for expected_path in [
        SDK_PHASE3_GENERATED_MODELS_PATH,
        SDK_PHASE3_HANDWRITTEN_CLIENT_PATH,
        SDK_PHASE3_READ_HELPERS_PATH,
    ] {
        if !boundary.iter().any(|module| module.path == expected_path) {
            return Err(SdkPhase3ValidationError::MissingBoundaryPath(expected_path));
        }
    }

    for module in boundary {
        if module.contract_authority {
            return Err(SdkPhase3ValidationError::GeneratedOutputIsAuthority(
                module.path,
            ));
        }
        if module.source_authority.trim().is_empty() {
            return Err(SdkPhase3ValidationError::MissingField(
                "module source authority",
            ));
        }
        if module.public_entrypoints.is_empty() {
            return Err(SdkPhase3ValidationError::MissingField(
                "module public entrypoints",
            ));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkGeneratedModelKind {
    Phase1Command,
    Tenant,
    Identity,
    KeyMetadata,
    Manifest,
    QueueStatus,
    AuditRef,
    Error,
}

impl SdkGeneratedModelKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Phase1Command => "phase1_command",
            Self::Tenant => "tenant",
            Self::Identity => "identity",
            Self::KeyMetadata => "key_metadata",
            Self::Manifest => "manifest",
            Self::QueueStatus => "queue_status",
            Self::AuditRef => "audit_ref",
            Self::Error => "error",
        }
    }

    pub fn command_family(self) -> Option<BootstrapCommandFamily> {
        match self {
            Self::Phase1Command => Some(BootstrapCommandFamily::Auth),
            Self::Tenant => Some(BootstrapCommandFamily::Tenant),
            Self::Identity => Some(BootstrapCommandFamily::Identity),
            Self::KeyMetadata => Some(BootstrapCommandFamily::Key),
            Self::Manifest => Some(BootstrapCommandFamily::Manifest),
            Self::QueueStatus => Some(BootstrapCommandFamily::Workload),
            Self::AuditRef => None,
            Self::Error => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkGeneratedModelDescriptor {
    pub kind: SdkGeneratedModelKind,
    pub public_type: &'static str,
    pub source_contract_name: &'static str,
    pub source_path: &'static str,
    pub schema_version: &'static str,
    pub generated_output_path: &'static str,
    pub validator_symbol: &'static str,
    pub stable_enum_mapping: bool,
    pub reason_code_object: bool,
}

pub fn sdk_generated_model_descriptors() -> Vec<SdkGeneratedModelDescriptor> {
    vec![
        SdkGeneratedModelDescriptor {
            kind: SdkGeneratedModelKind::Phase1Command,
            public_type: "SignedCommandEnvelope",
            source_contract_name: "signed_command_envelope",
            source_path: SDK_PHASE3_RUST_PROJECTION_SOURCE_PATH,
            schema_version: SUPPORTED_SCHEMA_VERSION,
            generated_output_path: SDK_PHASE3_GENERATED_MODELS_PATH,
            validator_symbol: "SignedCommandEnvelope::new",
            stable_enum_mapping: true,
            reason_code_object: false,
        },
        SdkGeneratedModelDescriptor {
            kind: SdkGeneratedModelKind::Tenant,
            public_type: "BootstrapAcceptanceRecord",
            source_contract_name: "tenant_control_plane_object",
            source_path: SDK_PHASE3_RUST_PROJECTION_SOURCE_PATH,
            schema_version: SUPPORTED_SCHEMA_VERSION,
            generated_output_path: SDK_PHASE3_GENERATED_MODELS_PATH,
            validator_symbol: "BootstrapAcceptanceRecord::new",
            stable_enum_mapping: true,
            reason_code_object: false,
        },
        SdkGeneratedModelDescriptor {
            kind: SdkGeneratedModelKind::Identity,
            public_type: "CommandContext",
            source_contract_name: "identity_context",
            source_path: SDK_PHASE3_RUST_PROJECTION_SOURCE_PATH,
            schema_version: SUPPORTED_SCHEMA_VERSION,
            generated_output_path: SDK_PHASE3_GENERATED_MODELS_PATH,
            validator_symbol: "CommandContext::new",
            stable_enum_mapping: true,
            reason_code_object: false,
        },
        SdkGeneratedModelDescriptor {
            kind: SdkGeneratedModelKind::KeyMetadata,
            public_type: "CredentialReference",
            source_contract_name: "credential_reference",
            source_path: SDK_PHASE3_SCHEMA_SOURCE_PATH,
            schema_version: SUPPORTED_SCHEMA_VERSION,
            generated_output_path: SDK_PHASE3_GENERATED_MODELS_PATH,
            validator_symbol: "CredentialReference::validate_for_profile",
            stable_enum_mapping: true,
            reason_code_object: false,
        },
        SdkGeneratedModelDescriptor {
            kind: SdkGeneratedModelKind::Manifest,
            public_type: "ManifestBootstrapRef",
            source_contract_name: "manifest_bootstrap_ref",
            source_path: SDK_PHASE3_RUST_PROJECTION_SOURCE_PATH,
            schema_version: SUPPORTED_SCHEMA_VERSION,
            generated_output_path: SDK_PHASE3_GENERATED_MODELS_PATH,
            validator_symbol: "ManifestBootstrapRef::new",
            stable_enum_mapping: true,
            reason_code_object: false,
        },
        SdkGeneratedModelDescriptor {
            kind: SdkGeneratedModelKind::QueueStatus,
            public_type: "SyntheticWorkloadPendingState",
            source_contract_name: "queue_pending_state",
            source_path: SDK_PHASE3_RUST_PROJECTION_SOURCE_PATH,
            schema_version: SUPPORTED_SCHEMA_VERSION,
            generated_output_path: SDK_PHASE3_GENERATED_MODELS_PATH,
            validator_symbol: "SyntheticWorkloadPendingState::pending",
            stable_enum_mapping: true,
            reason_code_object: false,
        },
        SdkGeneratedModelDescriptor {
            kind: SdkGeneratedModelKind::AuditRef,
            public_type: "CapabilitySnapshot",
            source_contract_name: "capability_snapshot",
            source_path: SDK_PHASE3_SCHEMA_SOURCE_PATH,
            schema_version: SUPPORTED_SCHEMA_VERSION,
            generated_output_path: SDK_PHASE3_GENERATED_MODELS_PATH,
            validator_symbol: "CapabilitySnapshot::local_phase_gate",
            stable_enum_mapping: true,
            reason_code_object: false,
        },
        SdkGeneratedModelDescriptor {
            kind: SdkGeneratedModelKind::Error,
            public_type: "OverridErrorRecord",
            source_contract_name: "api_error",
            source_path: SDK_PHASE3_REASON_CODE_SOURCE_PATH,
            schema_version: SUPPORTED_SCHEMA_VERSION,
            generated_output_path: SDK_PHASE3_GENERATED_MODELS_PATH,
            validator_symbol: "OverridErrorRecord::new",
            stable_enum_mapping: true,
            reason_code_object: true,
        },
    ]
}

pub fn validate_generated_model_descriptors(
    descriptors: &[SdkGeneratedModelDescriptor],
) -> Result<(), SdkPhase3ValidationError> {
    for kind in PHASE3_MODEL_KINDS {
        if !descriptors
            .iter()
            .any(|descriptor| descriptor.kind == *kind)
        {
            return Err(SdkPhase3ValidationError::MissingGeneratedModel(
                kind.as_str(),
            ));
        }
    }

    for descriptor in descriptors {
        for (field, value) in [
            ("public type", descriptor.public_type),
            ("source contract name", descriptor.source_contract_name),
            ("source path", descriptor.source_path),
            ("schema version", descriptor.schema_version),
            ("generated output path", descriptor.generated_output_path),
            ("validator symbol", descriptor.validator_symbol),
        ] {
            if value.trim().is_empty() {
                return Err(SdkPhase3ValidationError::MissingField(field));
            }
        }
        if descriptor.schema_version != SUPPORTED_SCHEMA_VERSION {
            return Err(SdkPhase3ValidationError::UnsupportedSchemaVersion(
                descriptor.schema_version,
            ));
        }
        if descriptor.generated_output_path != SDK_PHASE3_GENERATED_MODELS_PATH {
            return Err(SdkPhase3ValidationError::UnexpectedGeneratedOutput(
                descriptor.generated_output_path,
            ));
        }
        if !descriptor.stable_enum_mapping {
            return Err(SdkPhase3ValidationError::MissingStableEnumMapping(
                descriptor.kind.as_str(),
            ));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkPhase3ValidationError {
    MissingBoundaryPath(&'static str),
    GeneratedOutputIsAuthority(&'static str),
    MissingGeneratedModel(&'static str),
    MissingField(&'static str),
    UnsupportedSchemaVersion(&'static str),
    UnexpectedGeneratedOutput(&'static str),
    MissingStableEnumMapping(&'static str),
}

impl fmt::Display for SdkPhase3ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingBoundaryPath(path) => {
                write!(formatter, "missing SDK boundary path: {path}")
            }
            Self::GeneratedOutputIsAuthority(path) => {
                write!(
                    formatter,
                    "generated SDK output must not be contract authority: {path}"
                )
            }
            Self::MissingGeneratedModel(kind) => {
                write!(formatter, "missing generated model: {kind}")
            }
            Self::MissingField(field) => write!(formatter, "{field} is required"),
            Self::UnsupportedSchemaVersion(version) => {
                write!(
                    formatter,
                    "unsupported generated model schema version: {version}"
                )
            }
            Self::UnexpectedGeneratedOutput(path) => {
                write!(formatter, "unexpected generated model output path: {path}")
            }
            Self::MissingStableEnumMapping(kind) => {
                write!(
                    formatter,
                    "missing stable enum mapping for generated model: {kind}"
                )
            }
        }
    }
}

impl std::error::Error for SdkPhase3ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase3_boundary_keeps_generated_outputs_non_authoritative() {
        let boundary = sdk_package_boundary();

        validate_sdk_package_boundary(&boundary).unwrap();
        assert!(boundary
            .iter()
            .any(|module| module.path == SDK_PHASE3_GENERATED_MODELS_PATH));
        assert!(boundary.iter().all(|module| !module.contract_authority));
    }

    #[test]
    fn phase3_generated_model_descriptors_cover_initial_control_plane_objects() {
        let descriptors = sdk_generated_model_descriptors();

        validate_generated_model_descriptors(&descriptors).unwrap();
        for expected in PHASE3_MODEL_KINDS {
            assert!(descriptors
                .iter()
                .any(|descriptor| descriptor.kind == *expected));
        }
        assert!(descriptors
            .iter()
            .any(|descriptor| descriptor.reason_code_object));
    }
}
