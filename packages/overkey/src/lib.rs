#![forbid(unsafe_code)]

pub mod dependencies;
pub mod errors;
pub mod records;
pub mod repository;
pub mod routes;
pub mod schema;
pub mod service;

pub use dependencies::{DependencyCheck, DependencyMatrix, DependencyRequirement, DependencyState};
pub use errors::{ApiErrorData, OverkeyError, Retryability};
pub use records::{
    ApiKeyRecord, CredentialRecord, CredentialStatus, DelegationRecord, OverwatchEvent,
    PublicKeyRecord, RevocationRecord, RotationRecord, SecretRef, ServiceAccountKey,
    VerificationResult,
};
pub use repository::{
    CredentialMetadataRepository, InMemoryCredentialRepository, RepositoryError,
    StatusTransitionRecord, VerificationLogRecord,
};
pub use routes::{ApiResponse, TENANT_HEADER, TRACE_HEADER};
pub use service::{OverkeyConfig, OverkeyService, OverkeyState};
