use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use serde::Serialize;

use crate::records::{CredentialRecord, CredentialStatus, VerificationResult};
use crate::schema::contains_raw_secret_marker;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
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
        self.state
            .lock()
            .expect("credential repository lock poisoned")
            .credentials
            .insert(key, record);
        Ok(())
    }

    fn append_status_transition(
        &self,
        record: StatusTransitionRecord,
    ) -> Result<(), RepositoryError> {
        reject_raw_secret_material(&record)?;
        self.state
            .lock()
            .expect("credential repository lock poisoned")
            .status_history
            .push(record);
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
