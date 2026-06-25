#![forbid(unsafe_code)]

pub mod admin;
pub mod admission;
pub mod canonical;
pub mod dependencies;
pub mod envelope;
pub mod errors;
pub mod retention;
pub mod routes;
pub mod schema;
pub mod service;

pub use admission::{
    ActorResolutionRecord, AdmissionContext, OperatorAdmissionRecord,
    ServiceAccountAdmissionRecord, SignatureCheckRecord, TenantAuthorizationRecord,
};
pub use canonical::{CanonicalRequestInput, CANONICALIZATION_VERSION};
pub use dependencies::{DependencyCheck, DependencyMatrix, DependencyRequirement, DependencyState};
pub use envelope::{CommandEnvelope, SignatureMetadata};
pub use errors::{ApiErrorData, OvergateError, Retryability};
pub use retention::{RedactionDecision, RetentionDecision};
pub use routes::{ApiResponse, TRACE_HEADER};
pub use schema::SchemaValidationReport;
pub use service::{OvergateConfig, OvergateService, OvergateState};
