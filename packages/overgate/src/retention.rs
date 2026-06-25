use serde::Serialize;

use crate::envelope::CommandEnvelope;

pub const PHASE3_HASHES_AND_REFS_POLICY_ID: &str = "overgate.phase3.hashes_and_refs_only";
pub const PHASE5_CLASSED_HASHES_AND_REFS_POLICY_ID: &str =
    "overgate.phase5.classed_hashes_and_refs_only";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetentionDecision {
    pub policy_id: &'static str,
    pub command_type: String,
    pub privacy_class: String,
    pub idempotency_retention_class: &'static str,
    pub retention_window: &'static str,
    pub minimum_retention_seconds: u64,
    pub maximum_retention_seconds: Option<u64>,
    pub retention_extension_refs: Vec<&'static str>,
    pub expiration_job_contract: &'static str,
    pub body_retention: &'static str,
    pub retained_body_ref: Option<String>,
    pub retained_hash_refs: Vec<String>,
    pub redaction: RedactionDecision,
}

impl RetentionDecision {
    pub fn from_envelope(envelope: &CommandEnvelope, body_hash: &str) -> Self {
        let retention_class = retention_class(&envelope.command_type);
        let retention_extension_refs = retention_extension_refs(&envelope.command_type);
        Self {
            policy_id: PHASE5_CLASSED_HASHES_AND_REFS_POLICY_ID,
            command_type: envelope.command_type.clone(),
            privacy_class: envelope.privacy_class.clone(),
            idempotency_retention_class: retention_class.id,
            retention_window: retention_class.window,
            minimum_retention_seconds: retention_class.minimum_seconds,
            maximum_retention_seconds: retention_class.maximum_seconds,
            retention_extension_refs,
            expiration_job_contract:
                "overgate.phase5.expire_after_window_unless_extension_refs_hold",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RetentionClass {
    id: &'static str,
    window: &'static str,
    minimum_seconds: u64,
    maximum_seconds: Option<u64>,
}

fn retention_class(command_type: &str) -> RetentionClass {
    if command_type.contains("bodyless_read") || command_type.contains("read") {
        return RetentionClass {
            id: "bodyless_read_trace_cache",
            window: "up_to_24_hours_optional_trace_status_cache",
            minimum_seconds: 0,
            maximum_seconds: Some(24 * 60 * 60),
        };
    }
    if command_type.contains("accounting")
        || command_type.contains("ledger")
        || command_type.contains("oru")
        || command_type.contains("rights")
        || command_type.contains("payout")
        || command_type.contains("namespace")
        || command_type.contains("credential_recovery")
        || command_type.contains("policy_enforcement")
    {
        return RetentionClass {
            id: "finality_or_rights_command",
            window: "owning_service_audit_finality_window_at_least_90_days",
            minimum_seconds: 90 * 24 * 60 * 60,
            maximum_seconds: None,
        };
    }
    if command_type.contains("workload") || command_type.contains("queue") {
        return RetentionClass {
            id: "queue_producing_workload_command",
            window: "terminal_queue_state_plus_7_days_with_30_day_cap",
            minimum_seconds: 7 * 24 * 60 * 60,
            maximum_seconds: Some(30 * 24 * 60 * 60),
        };
    }
    if command_type.contains("tenant")
        || command_type.contains("identity")
        || command_type.contains("credential")
        || command_type.contains("manifest")
        || command_type.contains("admin")
        || command_type.contains("control_plane")
    {
        return RetentionClass {
            id: "control_plane_mutation",
            window: "at_least_7_days",
            minimum_seconds: 7 * 24 * 60 * 60,
            maximum_seconds: None,
        };
    }
    RetentionClass {
        id: "low_risk_metadata_write",
        window: "at_least_24_hours",
        minimum_seconds: 24 * 60 * 60,
        maximum_seconds: None,
    }
}

fn retention_extension_refs(command_type: &str) -> Vec<&'static str> {
    let mut refs = Vec::new();
    if command_type.contains("dispute") {
        refs.push("retention_extension:dispute_ref");
    }
    if command_type.contains("retry") {
        refs.push("retention_extension:retry_ref");
    }
    if command_type.contains("incident") {
        refs.push("retention_extension:incident_ref");
    }
    if command_type.contains("finality") {
        refs.push("retention_extension:finality_ref");
    }
    refs
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
