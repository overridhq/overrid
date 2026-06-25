use serde::Serialize;

use crate::envelope::CommandEnvelope;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetentionDecision {
    pub policy_id: &'static str,
    pub command_type: String,
    pub privacy_class: String,
    pub body_retention: &'static str,
    pub retained_body_ref: Option<String>,
    pub retained_hash_refs: Vec<String>,
    pub redaction: RedactionDecision,
}

impl RetentionDecision {
    pub fn from_envelope(envelope: &CommandEnvelope, body_hash: &str) -> Self {
        Self {
            policy_id: "overgate.phase3.hashes_and_refs_only",
            command_type: envelope.command_type.clone(),
            privacy_class: envelope.privacy_class.clone(),
            body_retention: "raw_body_not_retained",
            retained_body_ref: envelope.payload_ref.clone(),
            retained_hash_refs: vec![
                body_hash.to_owned(),
                envelope.request_hash.clone(),
                envelope.payload_hash.clone(),
            ],
            redaction: RedactionDecision::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RedactionDecision {
    pub private_payloads_redacted: bool,
    pub raw_secrets_redacted: bool,
    pub credential_material_redacted: bool,
    pub diagnostics_privacy_class: &'static str,
}

impl Default for RedactionDecision {
    fn default() -> Self {
        Self {
            private_payloads_redacted: true,
            raw_secrets_redacted: true,
            credential_material_redacted: true,
            diagnostics_privacy_class: "redacted_diagnostic",
        }
    }
}
