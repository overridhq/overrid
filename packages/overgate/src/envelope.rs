use axum::http::header::CONTENT_TYPE;
use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::canonical::{body_hash_ref, CANONICALIZATION_VERSION};
use crate::errors::OvergateError;

pub const MAX_COMMAND_ENVELOPE_BYTES: usize = 16 * 1024;
pub const SUPPORTED_COMMAND_SCHEMA_VERSION: &str = "shared-schema-package.v0.1";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommandEnvelope {
    pub command_id: String,
    pub command_type: String,
    #[serde(alias = "tenant_ref")]
    pub tenant_id: String,
    #[serde(alias = "actor_ref")]
    pub actor_id: String,
    pub trace_id: String,
    pub idempotency_key: String,
    #[serde(alias = "credential_ref")]
    pub credential_id: String,
    pub schema_version: String,
    pub payload_type: String,
    #[serde(alias = "request_hash_ref")]
    pub request_hash: String,
    #[serde(alias = "payload_hash_ref")]
    pub payload_hash: String,
    pub timestamp: String,
    pub signature_metadata: SignatureMetadata,
    pub privacy_class: String,
    pub payload_ref: Option<String>,
}

impl CommandEnvelope {
    pub fn parse_http(
        headers: &HeaderMap,
        body: &[u8],
    ) -> Result<ParsedCommandEnvelope, OvergateError> {
        if !is_json_content_type(headers) {
            return Err(OvergateError::wrong_content_type());
        }
        if body.len() > MAX_COMMAND_ENVELOPE_BYTES {
            return Err(OvergateError::oversized());
        }
        if contains_raw_secret_marker(body) {
            return Err(OvergateError::raw_secret_rejected());
        }

        let envelope: Self = serde_json::from_slice(body).map_err(map_serde_error)?;
        envelope.validate()?;

        Ok(ParsedCommandEnvelope {
            envelope,
            body_hash: body_hash_ref(body),
        })
    }

    pub fn validate(&self) -> Result<(), OvergateError> {
        for (field, value) in [
            ("command_id", self.command_id.as_str()),
            ("command_type", self.command_type.as_str()),
            ("tenant_id", self.tenant_id.as_str()),
            ("actor_id", self.actor_id.as_str()),
            ("trace_id", self.trace_id.as_str()),
            ("idempotency_key", self.idempotency_key.as_str()),
            ("credential_id", self.credential_id.as_str()),
            ("schema_version", self.schema_version.as_str()),
            ("payload_type", self.payload_type.as_str()),
            ("request_hash", self.request_hash.as_str()),
            ("payload_hash", self.payload_hash.as_str()),
            ("timestamp", self.timestamp.as_str()),
            ("privacy_class", self.privacy_class.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(missing_field_error(field));
            }
        }

        if self.schema_version != SUPPORTED_COMMAND_SCHEMA_VERSION {
            return Err(OvergateError::unsupported_schema_version());
        }
        if !self.command_type.starts_with("overgate.") {
            return Err(OvergateError::unsupported_command_type());
        }
        if !looks_like_hash_ref(&self.request_hash) {
            return Err(OvergateError::malformed_ref("request_hash"));
        }
        if !looks_like_hash_ref(&self.payload_hash) {
            return Err(OvergateError::malformed_ref("payload_hash"));
        }
        if !is_supported_payload_type(&self.payload_type) {
            return Err(OvergateError::malformed_ref("payload_type"));
        }
        if !is_private_command_privacy_class(&self.privacy_class) {
            return Err(OvergateError::wrong_privacy_class());
        }
        if let Some(payload_ref) = &self.payload_ref {
            if !looks_like_payload_ref(payload_ref) {
                return Err(OvergateError::malformed_ref("payload_ref"));
            }
        }
        self.signature_metadata.validate()?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureMetadata {
    pub signature_ref: String,
    pub algorithm: String,
    pub key_version: String,
    pub canonicalization_version: String,
}

impl SignatureMetadata {
    fn validate(&self) -> Result<(), OvergateError> {
        for (field, value) in [
            (
                "signature_metadata.signature_ref",
                self.signature_ref.as_str(),
            ),
            ("signature_metadata.algorithm", self.algorithm.as_str()),
            ("signature_metadata.key_version", self.key_version.as_str()),
            (
                "signature_metadata.canonicalization_version",
                self.canonicalization_version.as_str(),
            ),
        ] {
            if value.trim().is_empty() {
                return Err(missing_field_error(field));
            }
        }
        if !self.signature_ref.starts_with("signature:") {
            return Err(OvergateError::malformed_ref(
                "signature_metadata.signature_ref",
            ));
        }
        if self.algorithm != "ed25519" {
            return Err(OvergateError::malformed_ref("signature_metadata.algorithm"));
        }
        if self.canonicalization_version != CANONICALIZATION_VERSION {
            return Err(OvergateError::unsupported_canonicalization_version());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCommandEnvelope {
    pub envelope: CommandEnvelope,
    pub body_hash: String,
}

pub fn trace_id_hint(headers: &HeaderMap, body: &[u8], fallback: &str) -> String {
    if let Some(trace) = crate::routes::header_value(headers, crate::routes::TRACE_HEADER) {
        return trace;
    }
    serde_json::from_slice::<serde_json::Value>(body)
        .ok()
        .and_then(|value| {
            value
                .get("trace_id")
                .and_then(|trace| trace.as_str())
                .filter(|trace| !trace.trim().is_empty())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| fallback.to_owned())
}

fn is_json_content_type(headers: &HeaderMap) -> bool {
    headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(|value| {
            let media_type = value
                .split(';')
                .next()
                .unwrap_or_default()
                .trim()
                .to_ascii_lowercase();
            media_type == "application/json" || media_type.ends_with("+json")
        })
        .unwrap_or(false)
}

fn map_serde_error(error: serde_json::Error) -> OvergateError {
    let message = error.to_string();
    if message.contains("unknown field") {
        return OvergateError::unknown_sensitive_field();
    }
    if message.contains("missing field") {
        return OvergateError::missing_required_field(parse_missing_field(&message));
    }
    OvergateError::malformed_json()
}

fn parse_missing_field(message: &str) -> &'static str {
    for field in [
        "command_id",
        "command_type",
        "tenant_id",
        "actor_id",
        "trace_id",
        "idempotency_key",
        "credential_id",
        "schema_version",
        "payload_type",
        "request_hash",
        "payload_hash",
        "timestamp",
        "signature_metadata",
        "privacy_class",
    ] {
        if message.contains(field) {
            return field;
        }
    }
    "command_envelope"
}

fn missing_field_error(field: &str) -> OvergateError {
    match field {
        "command_id" => OvergateError::missing_required_field("command_id"),
        "command_type" => OvergateError::missing_required_field("command_type"),
        "tenant_id" => OvergateError::missing_required_field("tenant_id"),
        "actor_id" => OvergateError::missing_required_field("actor_id"),
        "trace_id" => OvergateError::missing_required_field("trace_id"),
        "idempotency_key" => OvergateError::missing_required_field("idempotency_key"),
        "credential_id" => OvergateError::missing_required_field("credential_id"),
        "schema_version" => OvergateError::missing_required_field("schema_version"),
        "payload_type" => OvergateError::missing_required_field("payload_type"),
        "request_hash" => OvergateError::missing_required_field("request_hash"),
        "payload_hash" => OvergateError::missing_required_field("payload_hash"),
        "timestamp" => OvergateError::missing_required_field("timestamp"),
        "signature_metadata.signature_ref" => {
            OvergateError::missing_required_field("signature_metadata.signature_ref")
        }
        "signature_metadata.algorithm" => {
            OvergateError::missing_required_field("signature_metadata.algorithm")
        }
        "signature_metadata.key_version" => {
            OvergateError::missing_required_field("signature_metadata.key_version")
        }
        "signature_metadata.canonicalization_version" => {
            OvergateError::missing_required_field("signature_metadata.canonicalization_version")
        }
        "privacy_class" => OvergateError::missing_required_field("privacy_class"),
        _ => OvergateError::missing_required_field("command_envelope"),
    }
}

fn looks_like_hash_ref(value: &str) -> bool {
    value.starts_with("hash:") || value.starts_with("hash_")
}

fn looks_like_payload_ref(value: &str) -> bool {
    value.starts_with("fixture://")
        || value.starts_with("payload:")
        || value.starts_with("overstore:")
        || value.starts_with("ref:")
}

fn is_supported_payload_type(value: &str) -> bool {
    value == "application/json" || value.ends_with("+json")
}

fn is_private_command_privacy_class(value: &str) -> bool {
    matches!(
        value,
        "tenant_private"
            | "regulated"
            | "encrypted_private"
            | "user_content"
            | "system_service_only"
    )
}

fn contains_raw_secret_marker(body: &[u8]) -> bool {
    let lower = String::from_utf8_lossy(body).to_ascii_lowercase();
    [
        "raw_secret_value",
        "\"secret\"",
        "private_key",
        "-----begin",
        "api_key",
        "password=",
        "token=",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}
