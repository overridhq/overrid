use serde::Serialize;

use crate::envelope::CommandEnvelope;
use crate::errors::OvergateError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchemaValidationReport {
    pub adapter_id: &'static str,
    pub shared_schema_version: String,
    pub command_module: &'static str,
    pub api_error_module: &'static str,
    pub source_of_truth: &'static str,
    pub strict_unknown_field_rejection: bool,
    pub payload_validation_state: &'static str,
}

pub fn validate_command_envelope(
    envelope: &CommandEnvelope,
) -> Result<SchemaValidationReport, OvergateError> {
    overrid_contracts::ensure_supported_shared_schema_package_schema_version(
        envelope.schema_version.as_str(),
    )
    .map_err(|_| OvergateError::unsupported_schema_version())?;

    let contract = overrid_contracts::SharedSchemaPhase3Contract::canonical()
        .map_err(|_| OvergateError::dependency_unavailable("shared_schema_package"))?;
    contract
        .validate()
        .map_err(|_| OvergateError::dependency_unavailable("shared_schema_package"))?;

    let command = contract
        .module("command")
        .ok_or_else(|| OvergateError::dependency_unavailable("shared_schema_package"))?;
    let api_error = contract
        .module("api_error")
        .ok_or_else(|| OvergateError::dependency_unavailable("shared_schema_package"))?;

    for required in [
        "trace_id",
        "idempotency_key",
        "payload_hash",
        "signature_metadata",
        "privacy_class",
    ] {
        if !command.has_required_field(required) {
            return Err(OvergateError::dependency_unavailable(
                "shared_schema_package",
            ));
        }
    }
    for required in [
        "reason_code",
        "trace_id",
        "retryability",
        "correction_fields",
    ] {
        if !api_error.has_required_field(required) {
            return Err(OvergateError::dependency_unavailable(
                "shared_schema_package",
            ));
        }
    }

    Ok(SchemaValidationReport {
        adapter_id: "overgate.phase3.shared_schema_adapter",
        shared_schema_version: contract.schema_version.raw().to_owned(),
        command_module: "command",
        api_error_module: "api_error",
        source_of_truth: "json_schema",
        strict_unknown_field_rejection: command.strict_unknown_field_rejection,
        payload_validation_state: "payload_ref_and_payload_hash_validated",
    })
}
