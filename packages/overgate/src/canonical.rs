use serde::Serialize;

use crate::envelope::CommandEnvelope;

pub const CANONICALIZATION_VERSION: &str = "overgate.canonical.v0.1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CanonicalRequestInput {
    pub canonicalization_version: &'static str,
    pub method: String,
    pub path: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub command_type: String,
    pub idempotency_key: String,
    pub payload_hash: String,
    pub timestamp: String,
    pub credential_id: String,
    pub signature_algorithm: String,
    pub key_version: String,
    pub body_hash: String,
    pub canonical_hash: String,
}

impl CanonicalRequestInput {
    pub fn from_envelope(
        method: &str,
        path: &str,
        envelope: &CommandEnvelope,
        body_hash: &str,
    ) -> Self {
        let canonical_parts = [
            CANONICALIZATION_VERSION,
            method,
            path,
            &envelope.tenant_id,
            &envelope.actor_id,
            &envelope.command_type,
            &envelope.idempotency_key,
            &envelope.payload_hash,
            &envelope.timestamp,
            &envelope.credential_id,
            &envelope.signature_metadata.algorithm,
            &envelope.signature_metadata.key_version,
            body_hash,
        ];
        Self {
            canonicalization_version: CANONICALIZATION_VERSION,
            method: method.to_owned(),
            path: path.to_owned(),
            tenant_id: envelope.tenant_id.clone(),
            actor_id: envelope.actor_id.clone(),
            command_type: envelope.command_type.clone(),
            idempotency_key: envelope.idempotency_key.clone(),
            payload_hash: envelope.payload_hash.clone(),
            timestamp: envelope.timestamp.clone(),
            credential_id: envelope.credential_id.clone(),
            signature_algorithm: envelope.signature_metadata.algorithm.clone(),
            key_version: envelope.signature_metadata.key_version.clone(),
            body_hash: body_hash.to_owned(),
            canonical_hash: stable_hash_ref(&canonical_parts),
        }
    }

    pub fn canonical_string(&self) -> String {
        [
            ("canonicalization_version", self.canonicalization_version),
            ("method", self.method.as_str()),
            ("path", self.path.as_str()),
            ("tenant_id", self.tenant_id.as_str()),
            ("actor_id", self.actor_id.as_str()),
            ("command_type", self.command_type.as_str()),
            ("idempotency_key", self.idempotency_key.as_str()),
            ("payload_hash", self.payload_hash.as_str()),
            ("timestamp", self.timestamp.as_str()),
            ("credential_id", self.credential_id.as_str()),
            ("signature_algorithm", self.signature_algorithm.as_str()),
            ("key_version", self.key_version.as_str()),
            ("body_hash", self.body_hash.as_str()),
        ]
        .into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("\n")
    }
}

pub(crate) fn body_hash_ref(bytes: &[u8]) -> String {
    let mut hash = fnv_offset();
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(fnv_prime());
    }
    format!("hash:overgate:{hash:016x}")
}

pub(crate) fn stable_hash_ref(parts: &[&str]) -> String {
    format!("hash:overgate:{}", stable_short_token(parts))
}

pub(crate) fn stable_short_token(parts: &[&str]) -> String {
    let mut hash = fnv_offset();
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(fnv_prime());
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(fnv_prime());
    }
    format!("{hash:016x}").chars().take(12).collect()
}

const fn fnv_offset() -> u64 {
    0xcbf2_9ce4_8422_2325_u64
}

const fn fnv_prime() -> u64 {
    0x0000_0100_0000_01b3
}
