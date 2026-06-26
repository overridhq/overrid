use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use serde::Serialize;

use crate::records::{CredentialRecord, CredentialStatus, VerificationResult};
use crate::schema::contains_raw_secret_marker;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    DuplicateCredential,
    DuplicateActiveKey,
    CredentialNotFound,
    InvalidStatusTransition,
    BroadServiceAccountScope,
    UnsignedServiceAccountCall,
    RawSecretMaterial,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusTransitionRecord {
    pub tenant_id: String,
    pub credential_id: String,
    pub from_status: CredentialStatus,
    pub to_status: CredentialStatus,
    pub reason_code: String,
    pub audit_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VerificationLogRecord {
    pub tenant_id: String,
    pub credential_id: String,
    pub verification_class: String,
    pub verified: bool,
    pub audit_ref: String,
}

pub trait CredentialMetadataRepository: Clone + Send + Sync + 'static {
    fn append_credential(&self, record: CredentialRecord) -> Result<(), RepositoryError>;
    fn append_status_transition(
        &self,
        record: StatusTransitionRecord,
    ) -> Result<(), RepositoryError>;
    fn record_verification(&self, record: VerificationResult) -> Result<(), RepositoryError>;
    fn update_last_used(
        &self,
        tenant_id: &str,
        credential_id: &str,
        used_at: String,
        audit_ref: String,
    ) -> Result<(), RepositoryError>;
    fn credential(&self, tenant_id: &str, credential_id: &str) -> Option<CredentialRecord>;
    fn status_history(&self, credential_id: &str) -> Vec<StatusTransitionRecord>;
    fn verification_log(&self, credential_id: &str) -> Vec<VerificationLogRecord>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryCredentialRepository {
    state: Arc<Mutex<RepositoryState>>,
}

#[derive(Debug, Default)]
struct RepositoryState {
    credentials: BTreeMap<String, CredentialRecord>,
    status_history: Vec<StatusTransitionRecord>,
    verification_log: Vec<VerificationLogRecord>,
}

impl CredentialMetadataRepository for InMemoryCredentialRepository {
    fn append_credential(&self, record: CredentialRecord) -> Result<(), RepositoryError> {
        reject_raw_secret_material(&record)?;
        let key = repository_key(&record.tenant_id, &record.credential_id);
        let mut state = self
            .state
            .lock()
            .expect("credential repository lock poisoned");
        if state.credentials.contains_key(&key) {
            return Err(RepositoryError::DuplicateCredential);
        }
        if !record.key_id.is_empty()
            && record.status == CredentialStatus::Active
            && state.credentials.values().any(|existing| {
                existing.tenant_id == record.tenant_id
                    && existing.key_id == record.key_id
                    && existing.status == CredentialStatus::Active
            })
        {
            return Err(RepositoryError::DuplicateActiveKey);
        }
        state.credentials.insert(key, record);
        Ok(())
    }

    fn append_status_transition(
        &self,
        record: StatusTransitionRecord,
    ) -> Result<(), RepositoryError> {
        reject_raw_secret_material(&record)?;
        let mut state = self
            .state
            .lock()
            .expect("credential repository lock poisoned");
        let key = repository_key(&record.tenant_id, &record.credential_id);
        let credential = state
            .credentials
            .get_mut(&key)
            .ok_or(RepositoryError::CredentialNotFound)?;
        if credential.status != record.from_status
            || !valid_status_transition(&record.from_status, &record.to_status)
        {
            return Err(RepositoryError::InvalidStatusTransition);
        }
        credential.status = record.to_status.clone();
        match credential.status {
            CredentialStatus::Rotating => credential.rotation_refs.push(record.audit_ref.clone()),
            CredentialStatus::Revoked => credential.revocation_refs.push(record.audit_ref.clone()),
            _ => {}
        }
        state.status_history.push(record);
        Ok(())
    }

    fn record_verification(&self, record: VerificationResult) -> Result<(), RepositoryError> {
        reject_raw_secret_material(&record)?;
        let log_record = VerificationLogRecord {
            tenant_id: record.tenant_id,
            credential_id: record.credential_id,
            verification_class: record.verification_class,
            verified: record.verified,
            audit_ref: record
                .audit_refs
                .first()
                .cloned()
                .unwrap_or_else(|| "audit:overkey:verification:missing".to_owned()),
        };
        self.state
            .lock()
            .expect("credential repository lock poisoned")
            .verification_log
            .push(log_record);
        Ok(())
    }

    fn update_last_used(
        &self,
        tenant_id: &str,
        credential_id: &str,
        used_at: String,
        audit_ref: String,
    ) -> Result<(), RepositoryError> {
        reject_raw_secret_material(&used_at)?;
        reject_raw_secret_material(&audit_ref)?;
        let mut state = self
            .state
            .lock()
            .expect("credential repository lock poisoned");
        let credential = state
            .credentials
            .get_mut(&repository_key(tenant_id, credential_id))
            .ok_or(RepositoryError::CredentialNotFound)?;
        credential.last_used_at = Some(used_at);
        state.verification_log.push(VerificationLogRecord {
            tenant_id: tenant_id.to_owned(),
            credential_id: credential_id.to_owned(),
            verification_class: "last_used".to_owned(),
            verified: true,
            audit_ref,
        });
        Ok(())
    }

    fn credential(&self, tenant_id: &str, credential_id: &str) -> Option<CredentialRecord> {
        self.state
            .lock()
            .expect("credential repository lock poisoned")
            .credentials
            .get(&repository_key(tenant_id, credential_id))
            .cloned()
    }

    fn status_history(&self, credential_id: &str) -> Vec<StatusTransitionRecord> {
        self.state
            .lock()
            .expect("credential repository lock poisoned")
            .status_history
            .iter()
            .filter(|record| record.credential_id == credential_id)
            .cloned()
            .collect()
    }

    fn verification_log(&self, credential_id: &str) -> Vec<VerificationLogRecord> {
        self.state
            .lock()
            .expect("credential repository lock poisoned")
            .verification_log
            .iter()
            .filter(|record| record.credential_id == credential_id)
            .cloned()
            .collect()
    }
}

fn reject_raw_secret_material(record: &impl Serialize) -> Result<(), RepositoryError> {
    let serialized =
        serde_json::to_string(record).map_err(|_| RepositoryError::RawSecretMaterial)?;
    if contains_raw_secret_marker(&serialized) {
        return Err(RepositoryError::RawSecretMaterial);
    }
    Ok(())
}

fn repository_key(tenant_id: &str, credential_id: &str) -> String {
    format!("{tenant_id}:{credential_id}")
}

fn valid_status_transition(from: &CredentialStatus, to: &CredentialStatus) -> bool {
    match (from, to) {
        (CredentialStatus::Pending, CredentialStatus::Active)
        | (CredentialStatus::Pending, CredentialStatus::Suspended)
        | (CredentialStatus::Pending, CredentialStatus::Revoked)
        | (CredentialStatus::Pending, CredentialStatus::Tombstoned)
        | (CredentialStatus::Active, CredentialStatus::Rotating)
        | (CredentialStatus::Active, CredentialStatus::Suspended)
        | (CredentialStatus::Active, CredentialStatus::Revoked)
        | (CredentialStatus::Active, CredentialStatus::Expired)
        | (CredentialStatus::Active, CredentialStatus::Tombstoned)
        | (CredentialStatus::Rotating, CredentialStatus::Active)
        | (CredentialStatus::Rotating, CredentialStatus::Suspended)
        | (CredentialStatus::Rotating, CredentialStatus::Revoked)
        | (CredentialStatus::Rotating, CredentialStatus::Expired)
        | (CredentialStatus::Rotating, CredentialStatus::Tombstoned)
        | (CredentialStatus::Suspended, CredentialStatus::Active)
        | (CredentialStatus::Suspended, CredentialStatus::Revoked)
        | (CredentialStatus::Suspended, CredentialStatus::Expired)
        | (CredentialStatus::Suspended, CredentialStatus::Tombstoned)
        | (CredentialStatus::Expired, CredentialStatus::Tombstoned)
        | (CredentialStatus::Revoked, CredentialStatus::Tombstoned) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::SecretRef;

    #[test]
    fn appends_lifecycle_history_without_overwriting_record_identity() {
        let repository = InMemoryCredentialRepository::default();
        let record = CredentialRecord::local_fixture(
            "tenant:fixture",
            "actor:fixture",
            "credential:api-key:fixture",
            "api_key",
            vec!["request.verify".to_owned()],
            SecretRef::local_fixture("secret://overvault/local/overkey/api-key"),
        );

        repository.append_credential(record.clone()).unwrap();
        repository
            .append_status_transition(StatusTransitionRecord {
                tenant_id: record.tenant_id.clone(),
                credential_id: record.credential_id.clone(),
                from_status: CredentialStatus::Active,
                to_status: CredentialStatus::Rotating,
                reason_code: "overkey.rotation_requested".to_owned(),
                audit_ref: "audit:overkey:rotation:fixture".to_owned(),
            })
            .unwrap();

        assert_eq!(
            repository
                .credential("tenant:fixture", "credential:api-key:fixture")
                .unwrap()
                .credential_id,
            "credential:api-key:fixture"
        );
        assert_eq!(
            repository
                .status_history("credential:api-key:fixture")
                .len(),
            1
        );
    }

    #[test]
    fn rejects_duplicate_credential_without_overwriting_record_identity() {
        let repository = InMemoryCredentialRepository::default();
        let record = CredentialRecord::local_fixture(
            "tenant:fixture",
            "actor:original",
            "credential:api-key:fixture",
            "api_key",
            vec!["request.verify".to_owned()],
            SecretRef::local_fixture("secret://overvault/local/overkey/original-api-key"),
        );
        let duplicate = CredentialRecord::local_fixture(
            "tenant:fixture",
            "actor:replacement",
            "credential:api-key:fixture",
            "api_key",
            vec!["request.verify".to_owned()],
            SecretRef::local_fixture("secret://overvault/local/overkey/replacement-api-key"),
        );

        repository.append_credential(record).unwrap();
        assert_eq!(
            repository.append_credential(duplicate),
            Err(RepositoryError::DuplicateCredential)
        );
        assert_eq!(
            repository
                .credential("tenant:fixture", "credential:api-key:fixture")
                .unwrap()
                .subject_ref,
            "actor:original"
        );
    }

    #[test]
    fn rejects_duplicate_active_key_ids_inside_tenant_scope() {
        let repository = InMemoryCredentialRepository::default();
        let mut first = CredentialRecord::local_fixture(
            "tenant:fixture",
            "actor:first",
            "credential:signing:first",
            "public_signing_key",
            vec!["signature.verify".to_owned()],
            SecretRef::local_fixture("secret://overvault/local/overkey/first"),
        );
        first.key_id = "key:tenant:shared".to_owned();
        let mut duplicate = CredentialRecord::local_fixture(
            "tenant:fixture",
            "actor:second",
            "credential:signing:second",
            "public_signing_key",
            vec!["signature.verify".to_owned()],
            SecretRef::local_fixture("secret://overvault/local/overkey/second"),
        );
        duplicate.key_id = "key:tenant:shared".to_owned();

        repository.append_credential(first).unwrap();
        assert_eq!(
            repository.append_credential(duplicate),
            Err(RepositoryError::DuplicateActiveKey)
        );
    }

    #[test]
    fn rejects_invalid_lifecycle_resurrection() {
        let repository = InMemoryCredentialRepository::default();
        let record = CredentialRecord::local_fixture(
            "tenant:fixture",
            "actor:fixture",
            "credential:api-key:fixture",
            "api_key",
            vec!["request.verify".to_owned()],
            SecretRef::local_fixture("secret://overvault/local/overkey/api-key"),
        );

        repository.append_credential(record.clone()).unwrap();
        repository
            .append_status_transition(StatusTransitionRecord {
                tenant_id: record.tenant_id.clone(),
                credential_id: record.credential_id.clone(),
                from_status: CredentialStatus::Active,
                to_status: CredentialStatus::Revoked,
                reason_code: "overkey.revocation_requested".to_owned(),
                audit_ref: "audit:overkey:revocation:fixture".to_owned(),
            })
            .unwrap();

        assert_eq!(
            repository.append_status_transition(StatusTransitionRecord {
                tenant_id: record.tenant_id,
                credential_id: record.credential_id,
                from_status: CredentialStatus::Revoked,
                to_status: CredentialStatus::Active,
                reason_code: "overkey.invalid_resurrection".to_owned(),
                audit_ref: "audit:overkey:invalid:fixture".to_owned(),
            }),
            Err(RepositoryError::InvalidStatusTransition)
        );
    }

    #[test]
    fn rejects_raw_private_or_api_key_material() {
        let repository = InMemoryCredentialRepository::default();
        let record = CredentialRecord::local_fixture(
            "tenant:fixture",
            "actor:fixture",
            "credential:bad",
            "api_key",
            vec!["request.verify".to_owned()],
            SecretRef::local_fixture("private_key=inline-material"),
        );

        assert_eq!(
            repository.append_credential(record),
            Err(RepositoryError::RawSecretMaterial)
        );
    }
}
