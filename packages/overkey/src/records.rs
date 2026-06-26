use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialStatus {
    Pending,
    Active,
    Rotating,
    Suspended,
    Revoked,
    Expired,
    Tombstoned,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretRef {
    pub provider: String,
    pub reference: String,
    pub protection_class: String,
    pub resolvable_by: Vec<String>,
}

impl SecretRef {
    pub fn local_fixture(reference: &str) -> Self {
        Self {
            provider: "overvault.local_stub".to_owned(),
            reference: reference.to_owned(),
            protection_class: "protection:tenant_bound_secret_ref".to_owned(),
            resolvable_by: vec!["service:overkey".to_owned()],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialRecord {
    pub schema_version: String,
    pub tenant_id: String,
    pub subject_ref: String,
    pub credential_id: String,
    pub key_id: String,
    pub key_version: u32,
    pub credential_class: String,
    pub allowed_uses: Vec<String>,
    pub status: CredentialStatus,
    pub created_at: String,
    pub not_before: String,
    pub not_after: String,
    pub algorithm: String,
    pub canonicalization: String,
    pub audit_refs: Vec<String>,
    pub protection_class: String,
    pub secret_ref: SecretRef,
    pub api_key_prefix: Option<String>,
    pub api_key_hash_ref: Option<String>,
    pub public_key_ref: Option<String>,
    pub key_fingerprint_ref: Option<String>,
    pub allowed_services: Vec<String>,
    pub allowed_command_classes: Vec<String>,
    pub revocation_epoch: u64,
    pub last_used_at: Option<String>,
    pub rotation_refs: Vec<String>,
    pub revocation_refs: Vec<String>,
}

impl CredentialRecord {
    pub fn local_fixture(
        tenant_id: &str,
        subject_ref: &str,
        credential_id: &str,
        credential_class: &str,
        allowed_uses: Vec<String>,
        secret_ref: SecretRef,
    ) -> Self {
        Self {
            schema_version: "schema:overkey:credential_record:v0".to_owned(),
            tenant_id: tenant_id.to_owned(),
            subject_ref: subject_ref.to_owned(),
            credential_id: credential_id.to_owned(),
            key_id: format!("key:{}", credential_id.trim_start_matches("credential:")),
            key_version: 1,
            credential_class: credential_class.to_owned(),
            allowed_uses,
            status: CredentialStatus::Active,
            created_at: "2026-01-01T00:00:00Z".to_owned(),
            not_before: "2026-01-01T00:00:00Z".to_owned(),
            not_after: "2026-12-31T23:59:59Z".to_owned(),
            algorithm: "Ed25519".to_owned(),
            canonicalization: "overrid.canonical_json.v0".to_owned(),
            audit_refs: vec!["audit:overkey:phase2:fixture".to_owned()],
            protection_class: "protection:tenant_bound_secret_ref".to_owned(),
            secret_ref,
            api_key_prefix: None,
            api_key_hash_ref: None,
            public_key_ref: None,
            key_fingerprint_ref: None,
            allowed_services: Vec::new(),
            allowed_command_classes: Vec::new(),
            revocation_epoch: 0,
            last_used_at: None,
            rotation_refs: Vec::new(),
            revocation_refs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiKeyRecord {
    pub credential_id: String,
    pub tenant_id: String,
    pub key_version: u32,
    pub api_key_prefix: String,
    pub api_key_hash_ref: String,
    pub hash_algorithm: String,
    pub allowed_uses: Vec<String>,
    pub status: CredentialStatus,
    pub created_at: String,
    pub last_used_at: Option<String>,
    pub lookup_hint_rules: Vec<String>,
    pub raw_key_discarded: bool,
    pub audit_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKeyRecord {
    pub credential_id: String,
    pub tenant_id: String,
    pub key_id: String,
    pub key_version: u32,
    pub algorithm: String,
    pub public_key_ref: String,
    pub key_fingerprint_ref: String,
    pub canonicalization: String,
    pub allowed_signature_uses: Vec<String>,
    pub not_before: String,
    pub not_after: String,
    pub protection_class: String,
    pub status: CredentialStatus,
    pub audit_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceAccountKey {
    pub credential_id: String,
    pub tenant_id: String,
    pub service_account_id: String,
    pub key_version: u32,
    pub public_key_ref: String,
    pub algorithm: String,
    pub allowed_services: Vec<String>,
    pub allowed_command_classes: Vec<String>,
    pub allowed_uses: Vec<String>,
    pub protection_class: String,
    pub signed_call_required: bool,
    pub status: CredentialStatus,
    pub audit_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DelegationRecord {
    pub delegation_id: String,
    pub tenant_id: String,
    pub subject_ref: String,
    pub delegated_to: String,
    pub allowed_uses: Vec<String>,
    pub not_after: String,
    pub status: CredentialStatus,
    pub audit_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RotationRecord {
    pub rotation_id: String,
    pub tenant_id: String,
    pub credential_id: String,
    pub predecessor_credential_id: String,
    pub predecessor_key_id: String,
    pub predecessor_key_version: u32,
    pub successor_credential_id: String,
    pub successor_key_id: String,
    pub successor_key_version: u32,
    pub grace_window_seconds: u64,
    pub rotation_state: String,
    pub initiated_by: String,
    pub reason_code: String,
    pub activation_at: String,
    pub evidence_refs: Vec<String>,
    pub revocation_epoch: u64,
    pub propagation_status: Vec<PropagationStatus>,
    pub audit_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevocationRecord {
    pub revocation_id: String,
    pub tenant_id: String,
    pub credential_id: String,
    pub revoked_by: String,
    pub revoked_at: String,
    pub effective_at: String,
    pub reason_code: String,
    pub affected_command_classes: Vec<String>,
    pub incident_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub expected_current_status: CredentialStatus,
    pub revocation_epoch: u64,
    pub break_glass: bool,
    pub idempotency_key: Option<String>,
    pub propagation_status: Vec<PropagationStatus>,
    pub audit_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheInvalidation {
    pub cache_key_ref: String,
    pub revocation_epoch: u64,
    pub max_positive_ttl_seconds: u64,
    pub high_risk_max_positive_ttl_seconds: u64,
    pub invalidation_event_ref: String,
    pub invalidation_reason: String,
    pub invalidated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropagationStatus {
    pub service_id: String,
    pub propagation_state: String,
    pub required_before_unblock: bool,
    pub last_checked_at: String,
    pub audit_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedInventory {
    pub tenant_id: String,
    pub subject_ref: String,
    pub credential_id: String,
    pub command_classes: Vec<String>,
    pub services: Vec<String>,
    pub product_clients: Vec<String>,
    pub follow_up_tasks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationResult {
    pub tenant_id: String,
    pub credential_id: String,
    pub key_id: String,
    pub key_version: u32,
    pub subject_ref: String,
    pub allowed_use: String,
    pub command_class: String,
    pub verified: bool,
    pub verification_state: String,
    pub verification_class: String,
    pub reason_code: String,
    pub algorithm: String,
    pub canonicalization: String,
    pub body_hash_ref: String,
    pub request_hash_ref: String,
    pub evidence_ref: String,
    pub revocation_epoch: u64,
    pub cache_key_ref: String,
    pub retryability: String,
    pub audit_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverwatchEvent {
    pub event_id: String,
    pub tenant_id: String,
    pub event_class: String,
    pub credential_id: String,
    pub reason_code: String,
    pub audit_ref: String,
}
