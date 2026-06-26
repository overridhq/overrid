use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::errors::OverkeyError;
use crate::records::{CredentialRecord, CredentialStatus, SecretRef, VerificationResult};
use crate::repository::{CredentialMetadataRepository, StatusTransitionRecord};
use crate::schema::{
    API_KEY_RECORD_SCHEMA_REF, CREDENTIAL_RECORD_SCHEMA_REF,
    OVERKEY_PHASE3_RESPONSE_SCHEMA_VERSION, PUBLIC_KEY_RECORD_SCHEMA_REF,
    REVOCATION_RECORD_SCHEMA_REF, ROTATION_RECORD_SCHEMA_REF, SERVICE_ACCOUNT_KEY_SCHEMA_REF,
    VERIFICATION_RESULT_SCHEMA_REF,
};
use crate::service::OverkeyState;

pub const TRACE_HEADER: &str = "x-overrid-trace-id";
pub const TENANT_HEADER: &str = "x-overrid-tenant-id";
pub const SERVICE_ACCOUNT_HEADER: &str = "x-overrid-service-account";
pub const SERVICE_SIGNATURE_HEADER: &str = "x-overrid-service-signature";

pub const ROUTE_CREATE_API_KEY: &str = "POST /v1/credentials/api-keys";
pub const ROUTE_CREATE_SIGNING_KEY: &str = "POST /v1/credentials/signing-keys";
pub const ROUTE_CREATE_SERVICE_ACCOUNT: &str = "POST /v1/credentials/service-accounts";
pub const ROUTE_ROTATE_CREDENTIAL: &str = "POST /v1/credentials/{credential_id}/rotate";
pub const ROUTE_REVOKE_CREDENTIAL: &str = "POST /v1/credentials/{credential_id}/revoke";
pub const ROUTE_GET_CREDENTIAL: &str = "GET /v1/credentials/{credential_id}";
pub const ROUTE_VERIFY_SIGNATURE: &str = "POST /v1/verify/signature";
pub const ROUTE_VERIFY_API_KEY: &str = "POST /v1/verify/api-key";
pub const ROUTE_USAGE_LAST_USED: &str = "POST /v1/usage/last-used";
pub const ROUTE_HEALTHZ: &str = "GET /v1/healthz";
pub const ROUTE_READYZ: &str = "GET /v1/readyz";

const CANONICALIZATION_VERSION: &str = "overrid.canonical_json.v0";
const LOCAL_PHASE3_TIMESTAMP: &str = "2026-06-26T00:00:00Z";
const API_KEY_LOOKUP_HASH_ALGORITHM: &str = "BLAKE3-keyed-lookup";
const API_KEY_HASH_REF_PREFIX: &str = "hash:api_key:blake3:";
const API_KEY_LOOKUP_HASH_CONTEXT: &str = "overrid.overkey.api_key_lookup.v0";
const API_KEY_LOOKUP_KEY_REF: &str = "secret://overvault/local/overkey/api-key-lookup-key";
const SIGNING_ALGORITHM: &str = "Ed25519";
const SERVICE_ACCOUNT_ALLOWED_SERVICES: [&str; 6] = [
    "service:overgate",
    "service:node-agent",
    "service:system-service",
    "service:worker",
    "service:overvault",
    "service:grid-resident",
];
const SERVICE_ACCOUNT_ALLOWED_COMMAND_CLASSES: [&str; 8] = [
    "command.verify",
    "command.credential.read",
    "command.credential.rotate",
    "command.credential.revoke",
    "command.node.enroll",
    "command.secret.resolve",
    "command.workload.execute",
    "command.system.operate",
];
const PHASE2_RESPONSE_SCHEMA_COMPATIBILITY: &str = "overkey.phase2.response.v0";
const SUPPORTED_RESPONSE_SCHEMA_VERSIONS: [&str; 2] = [
    PHASE2_RESPONSE_SCHEMA_COMPATIBILITY,
    OVERKEY_PHASE3_RESPONSE_SCHEMA_VERSION,
];

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub schema_version: &'static str,
    pub service: &'static str,
    pub trace_id: String,
    pub status: &'static str,
    pub reason_code: &'static str,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(
        trace_id: impl Into<String>,
        status: &'static str,
        reason_code: &'static str,
        data: T,
    ) -> Self {
        Self {
            schema_version: OVERKEY_PHASE3_RESPONSE_SCHEMA_VERSION,
            service: "service:overkey",
            trace_id: trace_id.into(),
            status,
            reason_code,
            data,
        }
    }
}

impl IntoResponse for OverkeyError {
    fn into_response(self) -> Response {
        let payload = ApiResponse::new(self.trace_id, "rejected", self.reason_code, self.data);
        (self.http_status, Json(payload)).into_response()
    }
}

#[derive(Debug, Serialize)]
struct CredentialRouteData {
    route: &'static str,
    tenant_id: String,
    credential_id: String,
    key_id: String,
    key_version: u32,
    record_kind: &'static str,
    schema_ref: &'static str,
    repository_action: &'static str,
    storage_boundary: &'static str,
    protection_class: String,
    allowed_uses: Vec<String>,
    allowed_services: Vec<String>,
    allowed_command_classes: Vec<String>,
    api_key_prefix: Option<String>,
    api_key_hash_ref: Option<String>,
    public_key_ref: Option<String>,
    key_fingerprint_ref: Option<String>,
    service_account_id: Option<String>,
    audit_refs: Vec<String>,
    overgate_admission_required: bool,
    overwatch_event_ref: String,
    overvault_secret_ref: String,
    lifecycle_status: CredentialStatus,
    raw_key_discarded: bool,
    raw_secret_persisted: bool,
    redacted_fields: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct CredentialLookupData {
    route: &'static str,
    tenant_id: String,
    credential_id: String,
    credential_known: bool,
    schema_ref: &'static str,
    lifecycle_status: String,
    credential_class: Option<String>,
    subject_ref: Option<String>,
    key_id: Option<String>,
    key_version: Option<u32>,
    allowed_uses: Vec<String>,
    not_after: Option<String>,
    last_used_at: Option<String>,
    rotation_refs: Vec<String>,
    revocation_refs: Vec<String>,
    protection_class_label: Option<String>,
    tenant_isolated: bool,
    raw_secret_persisted: bool,
    redacted_fields: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct VerificationData {
    route: &'static str,
    tenant_id: String,
    credential_id: String,
    verification_class: &'static str,
    verified: bool,
    schema_ref: &'static str,
    internal_only: bool,
    service_account_ref: String,
    body_hash_ref: &'static str,
    audit_refs: Vec<String>,
}

#[derive(Debug, Serialize)]
struct UsageData {
    route: &'static str,
    tenant_id: String,
    credential_id: String,
    last_used_at: String,
    usage_recorded: bool,
    overwatch_event_ref: String,
}

#[derive(Debug, Deserialize)]
struct ApiKeyEnrollmentRequest {
    credential_id: Option<String>,
    subject_ref: Option<String>,
    api_key_prefix: Option<String>,
    api_key_hash_ref: Option<String>,
    raw_api_key: Option<String>,
    allowed_uses: Option<Vec<String>>,
    not_after: Option<String>,
    audit_refs: Option<Vec<String>>,
    protection_class: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SigningKeyEnrollmentRequest {
    credential_id: Option<String>,
    subject_ref: Option<String>,
    key_id: String,
    key_version: Option<u32>,
    algorithm: Option<String>,
    public_key_ref: String,
    canonicalization: Option<String>,
    allowed_signature_uses: Option<Vec<String>>,
    not_before: Option<String>,
    not_after: Option<String>,
    protection_class: Option<String>,
    audit_refs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct ServiceAccountCredentialRequest {
    credential_id: Option<String>,
    service_account_id: String,
    key_version: Option<u32>,
    public_key_ref: Option<String>,
    allowed_services: Vec<String>,
    allowed_command_classes: Vec<String>,
    not_after: Option<String>,
    protection_class: Option<String>,
    audit_refs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct LifecycleRequest {
    reason_code: Option<String>,
    audit_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LastUsedRequest {
    credential_id: String,
    used_at: Option<String>,
    audit_ref: Option<String>,
}

#[derive(Debug, Serialize)]
struct HealthData {
    route: &'static str,
    service_id: String,
    liveness: &'static str,
    readiness_claimed: bool,
    supported_response_schema_versions: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct ReadinessData {
    route: &'static str,
    service_id: String,
    ready: bool,
    required_failures: Vec<String>,
    dependency_matrix: crate::dependencies::DependencyMatrix,
}

pub fn public_routes() -> Router<OverkeyState> {
    Router::new()
        .route("/v1/credentials/api-keys", post(create_api_key))
        .route("/v1/credentials/signing-keys", post(create_signing_key))
        .route(
            "/v1/credentials/service-accounts",
            post(create_service_account),
        )
        .route(
            "/v1/credentials/:credential_id/rotate",
            post(rotate_credential),
        )
        .route(
            "/v1/credentials/:credential_id/revoke",
            post(revoke_credential),
        )
        .route("/v1/credentials/:credential_id", get(get_credential))
        .route("/v1/verify/signature", post(verify_signature))
        .route("/v1/verify/api-key", post(verify_api_key))
        .route("/v1/usage/last-used", post(record_last_used))
        .route("/v1/healthz", get(healthz))
        .route("/v1/readyz", get(readyz))
}

async fn create_api_key(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<ApiResponse<CredentialRouteData>>, OverkeyError> {
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let request: ApiKeyEnrollmentRequest = parse_json_body(&headers, body)?;
    let credential_id = request
        .credential_id
        .clone()
        .unwrap_or_else(|| format!("credential:api-key:{}", stable_trace_token(&trace_id)));
    let prefix = request
        .api_key_prefix
        .clone()
        .unwrap_or_else(|| format!("ovk_{}", stable_trace_token(&trace_id)));
    let api_key_hash_ref = api_key_hash_ref(&request, &trace_id)?;
    let key_id = format!("key:api:{}", prefix);
    let allowed_uses = request.allowed_uses.clone().unwrap_or_else(|| {
        vec![
            "request.authenticate".to_owned(),
            "request.verify".to_owned(),
        ]
    });
    let audit_refs = request.audit_refs.clone().unwrap_or_else(|| {
        vec![format!(
            "audit:overkey:api-key:{}",
            stable_trace_token(&trace_id)
        )]
    });
    let protection_class = request
        .protection_class
        .clone()
        .unwrap_or_else(|| "protection:tenant_bound_secret_ref".to_owned());
    let record = credential_record_for_phase3(CredentialRecordInput {
        tenant_id: tenant_id.clone(),
        subject_ref: request
            .subject_ref
            .clone()
            .unwrap_or_else(|| "actor:overpass:phase3-api-key".to_owned()),
        credential_id: credential_id.clone(),
        key_id: key_id.clone(),
        key_version: 1,
        credential_class: "api_key".to_owned(),
        allowed_uses: allowed_uses.clone(),
        not_before: LOCAL_PHASE3_TIMESTAMP.to_owned(),
        not_after: request
            .not_after
            .clone()
            .unwrap_or_else(|| "2026-12-31T23:59:59Z".to_owned()),
        algorithm: API_KEY_LOOKUP_HASH_ALGORITHM.to_owned(),
        canonicalization: CANONICALIZATION_VERSION.to_owned(),
        audit_refs: audit_refs.clone(),
        protection_class: protection_class.clone(),
        secret_ref: SecretRef::local_fixture("secret://overvault/local/overkey/api-key-ref"),
    });
    state
        .repository
        .append_credential(record)
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id,
        "overkey.api_key_hash_enrolled",
        credential_data(CredentialRouteInput {
            route: ROUTE_CREATE_API_KEY,
            tenant_id,
            credential_id,
            key_id,
            key_version: 1,
            record_kind: "api_key_record",
            schema_ref: API_KEY_RECORD_SCHEMA_REF,
            allowed_uses,
            allowed_services: Vec::new(),
            allowed_command_classes: Vec::new(),
            api_key_prefix: Some(prefix),
            api_key_hash_ref: Some(api_key_hash_ref),
            public_key_ref: None,
            key_fingerprint_ref: None,
            service_account_id: None,
            audit_refs,
            overvault_secret_ref: "secret://overvault/local/overkey/api-key-ref".to_owned(),
            protection_class,
            lifecycle_status: CredentialStatus::Active,
            raw_key_discarded: request.raw_api_key.is_some(),
        }),
    ))
}

async fn create_signing_key(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<ApiResponse<CredentialRouteData>>, OverkeyError> {
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let request: SigningKeyEnrollmentRequest = parse_json_body(&headers, body)?;
    validate_signing_key_request(&trace_id, &request)?;
    let credential_id = request
        .credential_id
        .clone()
        .unwrap_or_else(|| format!("credential:signing-key:{}", stable_trace_token(&trace_id)));
    let key_version = request.key_version.unwrap_or(1);
    let allowed_uses = request
        .allowed_signature_uses
        .clone()
        .unwrap_or_else(|| vec!["signature.verify".to_owned()]);
    let audit_refs = request.audit_refs.clone().unwrap_or_else(|| {
        vec![format!(
            "audit:overkey:signing-key:{}",
            stable_trace_token(&trace_id)
        )]
    });
    let protection_class = request
        .protection_class
        .clone()
        .unwrap_or_else(|| "protection:tenant_bound_public_key".to_owned());
    let key_fingerprint_ref = blake3_ref("key-fingerprint", &request.public_key_ref);
    let record = credential_record_for_phase3(CredentialRecordInput {
        tenant_id: tenant_id.clone(),
        subject_ref: request
            .subject_ref
            .clone()
            .unwrap_or_else(|| "actor:overpass:phase3-signer".to_owned()),
        credential_id: credential_id.clone(),
        key_id: request.key_id.clone(),
        key_version,
        credential_class: "public_signing_key".to_owned(),
        allowed_uses: allowed_uses.clone(),
        not_before: request
            .not_before
            .clone()
            .unwrap_or_else(|| LOCAL_PHASE3_TIMESTAMP.to_owned()),
        not_after: request.not_after.clone().unwrap_or_default(),
        algorithm: SIGNING_ALGORITHM.to_owned(),
        canonicalization: CANONICALIZATION_VERSION.to_owned(),
        audit_refs: audit_refs.clone(),
        protection_class: protection_class.clone(),
        secret_ref: SecretRef::local_fixture("secret://overvault/local/overkey/signing-key-ref"),
    });
    state
        .repository
        .append_credential(record)
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id,
        "overkey.public_key_enrolled",
        credential_data(CredentialRouteInput {
            route: ROUTE_CREATE_SIGNING_KEY,
            tenant_id,
            credential_id,
            key_id: request.key_id,
            key_version,
            record_kind: "public_key_record",
            schema_ref: PUBLIC_KEY_RECORD_SCHEMA_REF,
            allowed_uses,
            allowed_services: Vec::new(),
            allowed_command_classes: Vec::new(),
            api_key_prefix: None,
            api_key_hash_ref: None,
            public_key_ref: Some(request.public_key_ref),
            key_fingerprint_ref: Some(key_fingerprint_ref),
            service_account_id: None,
            audit_refs,
            overvault_secret_ref: "secret://overvault/local/overkey/signing-key-ref".to_owned(),
            protection_class,
            lifecycle_status: CredentialStatus::Active,
            raw_key_discarded: false,
        }),
    ))
}

async fn create_service_account(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<ApiResponse<CredentialRouteData>>, OverkeyError> {
    let trace_id = trace_id(&headers);
    require_internal_service_account(&headers).map_err(|_| {
        OverkeyError::repository_rejected(
            trace_id.clone(),
            crate::repository::RepositoryError::UnsignedServiceAccountCall,
        )
    })?;
    let tenant_id = tenant_from_headers(&headers)?;
    let request: ServiceAccountCredentialRequest = parse_json_body(&headers, body)?;
    if !allowed_service_account_scope(&request.allowed_services, &request.allowed_command_classes) {
        return Err(OverkeyError::repository_rejected(
            trace_id,
            crate::repository::RepositoryError::BroadServiceAccountScope,
        ));
    }
    let credential_id = request.credential_id.clone().unwrap_or_else(|| {
        format!(
            "credential:service-account:{}",
            stable_trace_token(&trace_id)
        )
    });
    let key_version = request.key_version.unwrap_or(1);
    let public_key_ref = request.public_key_ref.clone().unwrap_or_else(|| {
        format!(
            "public-key-ref:overkey:service-account:{}",
            stable_trace_token(&trace_id)
        )
    });
    let audit_refs = request.audit_refs.clone().unwrap_or_else(|| {
        vec![format!(
            "audit:overkey:service-account:{}",
            stable_trace_token(&trace_id)
        )]
    });
    let protection_class = request
        .protection_class
        .clone()
        .unwrap_or_else(|| "protection:service_account_internal".to_owned());
    let record = credential_record_for_phase3(CredentialRecordInput {
        tenant_id: tenant_id.clone(),
        subject_ref: request.service_account_id.clone(),
        credential_id: credential_id.clone(),
        key_id: format!("key:service-account:{}", request.service_account_id),
        key_version,
        credential_class: "service_account_key".to_owned(),
        allowed_uses: request.allowed_command_classes.clone(),
        not_before: LOCAL_PHASE3_TIMESTAMP.to_owned(),
        not_after: request
            .not_after
            .clone()
            .unwrap_or_else(|| "2026-12-31T23:59:59Z".to_owned()),
        algorithm: SIGNING_ALGORITHM.to_owned(),
        canonicalization: CANONICALIZATION_VERSION.to_owned(),
        audit_refs: audit_refs.clone(),
        protection_class: protection_class.clone(),
        secret_ref: SecretRef::local_fixture(
            "secret://overvault/local/overkey/service-account-key-ref",
        ),
    });
    state
        .repository
        .append_credential(record)
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id,
        "overkey.service_account_credential_enrolled",
        credential_data(CredentialRouteInput {
            route: ROUTE_CREATE_SERVICE_ACCOUNT,
            tenant_id,
            credential_id,
            key_id: format!("key:service-account:{}", request.service_account_id),
            key_version,
            record_kind: "service_account_key",
            schema_ref: SERVICE_ACCOUNT_KEY_SCHEMA_REF,
            allowed_uses: request.allowed_command_classes.clone(),
            allowed_services: request.allowed_services,
            allowed_command_classes: request.allowed_command_classes,
            api_key_prefix: None,
            api_key_hash_ref: None,
            public_key_ref: Some(public_key_ref),
            key_fingerprint_ref: None,
            service_account_id: Some(request.service_account_id),
            audit_refs,
            overvault_secret_ref: "secret://overvault/local/overkey/service-account-key-ref"
                .to_owned(),
            protection_class,
            lifecycle_status: CredentialStatus::Active,
            raw_key_discarded: false,
        }),
    ))
}

async fn rotate_credential(
    State(state): State<OverkeyState>,
    Path(credential_id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<ApiResponse<CredentialRouteData>>, OverkeyError> {
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let request: LifecycleRequest = parse_json_body(&headers, body)?;
    let current = state
        .repository
        .credential(&tenant_id, &credential_id)
        .ok_or_else(|| {
            OverkeyError::repository_rejected(
                trace_id.clone(),
                crate::repository::RepositoryError::CredentialNotFound,
            )
        })?;
    let audit_ref = request
        .audit_ref
        .unwrap_or_else(|| format!("audit:overkey:rotation:{}", stable_trace_token(&trace_id)));
    state
        .repository
        .append_status_transition(StatusTransitionRecord {
            tenant_id: tenant_id.clone(),
            credential_id: credential_id.clone(),
            from_status: current.status.clone(),
            to_status: CredentialStatus::Rotating,
            reason_code: request
                .reason_code
                .unwrap_or_else(|| "overkey.rotation_requested".to_owned()),
            audit_ref: audit_ref.clone(),
        })
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id,
        "overkey.rotation_requested",
        credential_data(CredentialRouteInput {
            route: ROUTE_ROTATE_CREDENTIAL,
            tenant_id,
            credential_id,
            key_id: current.key_id,
            key_version: current.key_version + 1,
            record_kind: "rotation_record",
            schema_ref: ROTATION_RECORD_SCHEMA_REF,
            allowed_uses: vec!["credential.rotate".to_owned()],
            allowed_services: Vec::new(),
            allowed_command_classes: Vec::new(),
            api_key_prefix: None,
            api_key_hash_ref: None,
            public_key_ref: None,
            key_fingerprint_ref: None,
            service_account_id: None,
            audit_refs: vec![audit_ref],
            overvault_secret_ref: "secret://overvault/local/overkey/rotation-target-ref".to_owned(),
            protection_class: current.protection_class,
            lifecycle_status: CredentialStatus::Rotating,
            raw_key_discarded: false,
        }),
    ))
}

async fn revoke_credential(
    State(state): State<OverkeyState>,
    Path(credential_id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<ApiResponse<CredentialRouteData>>, OverkeyError> {
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let request: LifecycleRequest = parse_json_body(&headers, body)?;
    let current = state
        .repository
        .credential(&tenant_id, &credential_id)
        .ok_or_else(|| {
            OverkeyError::repository_rejected(
                trace_id.clone(),
                crate::repository::RepositoryError::CredentialNotFound,
            )
        })?;
    let audit_ref = request
        .audit_ref
        .unwrap_or_else(|| format!("audit:overkey:revocation:{}", stable_trace_token(&trace_id)));
    state
        .repository
        .append_status_transition(StatusTransitionRecord {
            tenant_id: tenant_id.clone(),
            credential_id: credential_id.clone(),
            from_status: current.status.clone(),
            to_status: CredentialStatus::Revoked,
            reason_code: request
                .reason_code
                .unwrap_or_else(|| "overkey.revocation_requested".to_owned()),
            audit_ref: audit_ref.clone(),
        })
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id,
        "overkey.revocation_requested",
        credential_data(CredentialRouteInput {
            route: ROUTE_REVOKE_CREDENTIAL,
            tenant_id,
            credential_id,
            key_id: current.key_id,
            key_version: current.key_version,
            record_kind: "revocation_record",
            schema_ref: REVOCATION_RECORD_SCHEMA_REF,
            allowed_uses: vec!["credential.revoke".to_owned()],
            allowed_services: Vec::new(),
            allowed_command_classes: Vec::new(),
            api_key_prefix: None,
            api_key_hash_ref: None,
            public_key_ref: None,
            key_fingerprint_ref: None,
            service_account_id: None,
            audit_refs: vec![audit_ref],
            overvault_secret_ref: "secret://overvault/local/overkey/revocation-ref".to_owned(),
            protection_class: current.protection_class,
            lifecycle_status: CredentialStatus::Revoked,
            raw_key_discarded: false,
        }),
    ))
}

async fn get_credential(
    State(state): State<OverkeyState>,
    Path(credential_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<CredentialLookupData>>, OverkeyError> {
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let credential = state.repository.credential(&tenant_id, &credential_id);
    let data = match credential {
        Some(record) => CredentialLookupData {
            route: ROUTE_GET_CREDENTIAL,
            tenant_id,
            credential_id,
            credential_known: true,
            schema_ref: CREDENTIAL_RECORD_SCHEMA_REF,
            lifecycle_status: format!("{:?}", record.status).to_ascii_lowercase(),
            credential_class: Some(record.credential_class),
            subject_ref: Some(record.subject_ref),
            key_id: Some(record.key_id),
            key_version: Some(record.key_version),
            allowed_uses: record.allowed_uses,
            not_after: Some(record.not_after),
            last_used_at: record.last_used_at,
            rotation_refs: record.rotation_refs,
            revocation_refs: record.revocation_refs,
            protection_class_label: Some(record.protection_class),
            tenant_isolated: true,
            raw_secret_persisted: false,
            redacted_fields: safe_metadata_redactions(),
        },
        None => CredentialLookupData {
            route: ROUTE_GET_CREDENTIAL,
            tenant_id,
            credential_id,
            credential_known: false,
            schema_ref: CREDENTIAL_RECORD_SCHEMA_REF,
            lifecycle_status: "unknown".to_owned(),
            credential_class: None,
            subject_ref: None,
            key_id: None,
            key_version: None,
            allowed_uses: Vec::new(),
            not_after: None,
            last_used_at: None,
            rotation_refs: Vec::new(),
            revocation_refs: Vec::new(),
            protection_class_label: None,
            tenant_isolated: true,
            raw_secret_persisted: false,
            redacted_fields: safe_metadata_redactions(),
        },
    };

    Ok(json_response(trace_id, "overkey.credential_lookup", data))
}

async fn verify_signature(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
    _body: Bytes,
) -> Result<Json<ApiResponse<VerificationData>>, OverkeyError> {
    require_json(&headers)?;
    require_internal_service_account(&headers)?;
    verify_with_class(state, headers, ROUTE_VERIFY_SIGNATURE, "signature")
}

async fn verify_api_key(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
    _body: Bytes,
) -> Result<Json<ApiResponse<VerificationData>>, OverkeyError> {
    require_json(&headers)?;
    require_internal_service_account(&headers)?;
    verify_with_class(state, headers, ROUTE_VERIFY_API_KEY, "api_key")
}

async fn record_last_used(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<ApiResponse<UsageData>>, OverkeyError> {
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let request: LastUsedRequest = parse_json_body(&headers, body)?;
    let used_at = request
        .used_at
        .unwrap_or_else(|| LOCAL_PHASE3_TIMESTAMP.to_owned());
    state
        .repository
        .update_last_used(
            &tenant_id,
            &request.credential_id,
            used_at.clone(),
            request.audit_ref.unwrap_or_else(|| {
                format!("audit:overkey:last-used:{}", stable_trace_token(&trace_id))
            }),
        )
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id.clone(),
        "overkey.usage_recorded",
        UsageData {
            route: ROUTE_USAGE_LAST_USED,
            tenant_id,
            credential_id: request.credential_id,
            last_used_at: used_at,
            usage_recorded: true,
            overwatch_event_ref: format!("event:overwatch:overkey:last-used:{trace_id}"),
        },
    ))
}

async fn healthz(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
) -> Json<ApiResponse<HealthData>> {
    json_response(
        trace_id(&headers),
        "overkey.live",
        HealthData {
            route: ROUTE_HEALTHZ,
            service_id: state.config.service_id,
            liveness: "alive",
            readiness_claimed: false,
            supported_response_schema_versions: SUPPORTED_RESPONSE_SCHEMA_VERSIONS.to_vec(),
        },
    )
}

async fn readyz(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
) -> Json<ApiResponse<ReadinessData>> {
    let ready = state.dependencies.ready();
    let required_failures = state
        .dependencies
        .required_failures()
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    json_response(
        trace_id(&headers),
        if ready {
            "overkey.ready"
        } else {
            "overkey.dependency_unavailable"
        },
        ReadinessData {
            route: ROUTE_READYZ,
            service_id: state.config.service_id,
            ready,
            required_failures,
            dependency_matrix: state.dependencies,
        },
    )
}

fn verify_with_class(
    state: OverkeyState,
    headers: HeaderMap,
    route: &'static str,
    verification_class: &'static str,
) -> Result<Json<ApiResponse<VerificationData>>, OverkeyError> {
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let service_account_ref = header_value(&headers, SERVICE_ACCOUNT_HEADER)
        .unwrap_or_else(|| "service-account:unknown".to_owned());
    let credential_id = format!(
        "credential:{verification_class}:{}",
        stable_trace_token(&trace_id)
    );
    let result = VerificationResult {
        tenant_id: tenant_id.clone(),
        credential_id: credential_id.clone(),
        verified: true,
        verification_class: verification_class.to_owned(),
        algorithm: if verification_class == "signature" {
            "Ed25519".to_owned()
        } else {
            "BLAKE3-keyed-lookup".to_owned()
        },
        canonicalization: "overrid.canonical_json.v0".to_owned(),
        body_hash_ref: "hash:blake3:overkey:phase2:body".to_owned(),
        audit_refs: vec![format!(
            "audit:overkey:verify:{verification_class}:{}",
            stable_trace_token(&trace_id)
        )],
    };
    state
        .repository
        .record_verification(result)
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id,
        "overkey.verification_completed",
        VerificationData {
            route,
            tenant_id,
            credential_id,
            verification_class,
            verified: true,
            schema_ref: VERIFICATION_RESULT_SCHEMA_REF,
            internal_only: true,
            service_account_ref,
            body_hash_ref: "hash:blake3:overkey:phase2:body",
            audit_refs: vec!["audit:overkey:verification:phase2".to_owned()],
        },
    ))
}

struct CredentialRecordInput {
    tenant_id: String,
    subject_ref: String,
    credential_id: String,
    key_id: String,
    key_version: u32,
    credential_class: String,
    allowed_uses: Vec<String>,
    not_before: String,
    not_after: String,
    algorithm: String,
    canonicalization: String,
    audit_refs: Vec<String>,
    protection_class: String,
    secret_ref: SecretRef,
}

struct CredentialRouteInput {
    route: &'static str,
    tenant_id: String,
    credential_id: String,
    key_id: String,
    key_version: u32,
    record_kind: &'static str,
    schema_ref: &'static str,
    allowed_uses: Vec<String>,
    allowed_services: Vec<String>,
    allowed_command_classes: Vec<String>,
    api_key_prefix: Option<String>,
    api_key_hash_ref: Option<String>,
    public_key_ref: Option<String>,
    key_fingerprint_ref: Option<String>,
    service_account_id: Option<String>,
    audit_refs: Vec<String>,
    overvault_secret_ref: String,
    protection_class: String,
    lifecycle_status: CredentialStatus,
    raw_key_discarded: bool,
}

fn credential_record_for_phase3(input: CredentialRecordInput) -> CredentialRecord {
    CredentialRecord {
        schema_version: CREDENTIAL_RECORD_SCHEMA_REF.to_owned(),
        tenant_id: input.tenant_id,
        subject_ref: input.subject_ref,
        credential_id: input.credential_id,
        key_id: input.key_id,
        key_version: input.key_version,
        credential_class: input.credential_class,
        allowed_uses: input.allowed_uses,
        status: CredentialStatus::Active,
        created_at: LOCAL_PHASE3_TIMESTAMP.to_owned(),
        not_before: input.not_before,
        not_after: input.not_after,
        algorithm: input.algorithm,
        canonicalization: input.canonicalization,
        audit_refs: input.audit_refs,
        protection_class: input.protection_class,
        secret_ref: input.secret_ref,
        last_used_at: None,
        rotation_refs: Vec::new(),
        revocation_refs: Vec::new(),
    }
}

fn credential_data(input: CredentialRouteInput) -> CredentialRouteData {
    CredentialRouteData {
        route: input.route,
        tenant_id: input.tenant_id,
        credential_id: input.credential_id,
        key_id: input.key_id,
        key_version: input.key_version,
        record_kind: input.record_kind,
        schema_ref: input.schema_ref,
        repository_action: "append_metadata_record",
        storage_boundary: "append_only_local_overrid_stub",
        protection_class: input.protection_class,
        allowed_uses: input.allowed_uses,
        allowed_services: input.allowed_services,
        allowed_command_classes: input.allowed_command_classes,
        api_key_prefix: input.api_key_prefix,
        api_key_hash_ref: input.api_key_hash_ref,
        public_key_ref: input.public_key_ref,
        key_fingerprint_ref: input.key_fingerprint_ref,
        service_account_id: input.service_account_id,
        audit_refs: input.audit_refs,
        overgate_admission_required: true,
        overwatch_event_ref: "event:overwatch:credential_lifecycle:phase3".to_owned(),
        overvault_secret_ref: input.overvault_secret_ref,
        lifecycle_status: input.lifecycle_status,
        raw_key_discarded: input.raw_key_discarded,
        raw_secret_persisted: false,
        redacted_fields: safe_metadata_redactions(),
    }
}

fn parse_json_body<T: for<'de> Deserialize<'de>>(
    headers: &HeaderMap,
    body: Bytes,
) -> Result<T, OverkeyError> {
    require_json(headers)?;
    serde_json::from_slice(&body).map_err(|_| {
        OverkeyError::invalid_enrollment(
            trace_id(headers),
            "overkey.invalid_json_payload",
            "request body must be valid JSON for the Overkey route",
            vec!["body"],
        )
    })
}

fn api_key_hash_ref(
    request: &ApiKeyEnrollmentRequest,
    trace_id: &str,
) -> Result<String, OverkeyError> {
    if let Some(hash_ref) = &request.api_key_hash_ref {
        if hash_ref.starts_with(API_KEY_HASH_REF_PREFIX) {
            return Ok(hash_ref.clone());
        }
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.api_key_hash_ref_invalid",
            "api key enrollment requires a typed BLAKE3 API key hash ref",
            vec!["api_key_hash_ref"],
        ));
    }
    let raw_api_key = request.raw_api_key.as_deref().ok_or_else(|| {
        OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.api_key_hash_required",
            "api key enrollment requires either raw_api_key for immediate hashing or api_key_hash_ref",
            vec!["raw_api_key", "api_key_hash_ref"],
        )
    })?;
    Ok(api_key_lookup_hash_ref(raw_api_key))
}

fn validate_signing_key_request(
    trace_id: &str,
    request: &SigningKeyEnrollmentRequest,
) -> Result<(), OverkeyError> {
    if request.algorithm.as_deref().unwrap_or(SIGNING_ALGORITHM) != SIGNING_ALGORITHM {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.weak_algorithm_rejected",
            "Overkey Phase 3 signing keys require Ed25519",
            vec!["algorithm"],
        ));
    }
    if request
        .canonicalization
        .as_deref()
        .unwrap_or(CANONICALIZATION_VERSION)
        != CANONICALIZATION_VERSION
    {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.unknown_canonicalization_rejected",
            "unknown canonicalization versions are denied by default",
            vec!["canonicalization"],
        ));
    }
    if request.not_after.as_deref().unwrap_or("").trim().is_empty() {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.expiry_required",
            "signing public key enrollment requires not_after metadata",
            vec!["not_after"],
        ));
    }
    if request
        .protection_class
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
    {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.protection_class_required",
            "signing public key enrollment requires protection-class metadata",
            vec!["protection_class"],
        ));
    }
    Ok(())
}

fn narrow_service_account_scope(values: &[String]) -> bool {
    !values.is_empty()
        && values.len() <= 4
        && values.iter().all(|value| {
            let lowered = value.to_ascii_lowercase();
            lowered != "*"
                && lowered != "all"
                && !lowered.contains("admin:*")
                && !lowered.contains("root")
                && !lowered.contains("wildcard")
        })
}

fn allowed_service_account_scope(services: &[String], command_classes: &[String]) -> bool {
    narrow_service_account_scope(services)
        && narrow_service_account_scope(command_classes)
        && services.iter().all(|value| {
            SERVICE_ACCOUNT_ALLOWED_SERVICES
                .iter()
                .any(|allowed| value == allowed)
        })
        && command_classes.iter().all(|value| {
            SERVICE_ACCOUNT_ALLOWED_COMMAND_CLASSES
                .iter()
                .any(|allowed| value == allowed)
        })
}

fn api_key_lookup_hash_ref(raw_api_key: &str) -> String {
    let lookup_key = blake3::derive_key(
        API_KEY_LOOKUP_HASH_CONTEXT,
        API_KEY_LOOKUP_KEY_REF.as_bytes(),
    );
    let hash = blake3::keyed_hash(&lookup_key, raw_api_key.as_bytes());
    format!("{API_KEY_HASH_REF_PREFIX}{hash}")
}

fn blake3_ref(kind: &str, value: &str) -> String {
    let hash = blake3::hash(value.as_bytes());
    format!("hash:{kind}:blake3:{hash}")
}

fn safe_metadata_redactions() -> Vec<&'static str> {
    vec![
        "raw_api_key",
        "api_key_lookup_hash_internal",
        "private_key_material",
        "secret_ref.resolved_value",
        "service_signature",
    ]
}

fn require_json(headers: &HeaderMap) -> Result<(), OverkeyError> {
    let content_type = header_value(headers, CONTENT_TYPE.as_str());
    match content_type
        .as_deref()
        .and_then(|value| value.split(';').next())
        .map(str::trim)
    {
        Some(value) if value.eq_ignore_ascii_case("application/json") => Ok(()),
        _ => Err(OverkeyError::unsupported_media_type(trace_id(headers))),
    }
}

fn tenant_from_headers(headers: &HeaderMap) -> Result<String, OverkeyError> {
    header_value(headers, TENANT_HEADER)
        .ok_or_else(|| OverkeyError::missing_tenant(trace_id(headers)))
}

fn require_internal_service_account(headers: &HeaderMap) -> Result<(), OverkeyError> {
    let trace_id = trace_id(headers);
    if header_value(headers, SERVICE_ACCOUNT_HEADER).is_some()
        && header_value(headers, SERVICE_SIGNATURE_HEADER).is_some()
    {
        Ok(())
    } else {
        Err(OverkeyError::missing_service_account(trace_id))
    }
}

fn trace_id(headers: &HeaderMap) -> String {
    header_value(headers, TRACE_HEADER).unwrap_or_else(|| "trace:overkey:phase2".to_owned())
}

fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

fn json_response<T: Serialize>(
    trace_id: String,
    reason_code: &'static str,
    data: T,
) -> Json<ApiResponse<T>> {
    Json(ApiResponse::new(trace_id, "accepted", reason_code, data))
}

fn stable_trace_token(trace_id: &str) -> String {
    let token = trace_id
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_owned();

    if token.is_empty() {
        "phase2".to_owned()
    } else {
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{to_bytes, Body};
    use axum::http::{Method, Request, StatusCode};
    use serde_json::Value;
    use tower::ServiceExt;

    use crate::service::OverkeyService;

    #[tokio::test]
    async fn public_routes_register_and_preserve_trace_json() {
        let router = OverkeyService::default().router();
        let response = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/api-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase2")
                    .header(TRACE_HEADER, "trace:overkey:test")
                    .body(Body::from(
                        r#"{"api_key_prefix":"ovk_test","raw_api_key":"local-test-api-key"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response_json(response).await;
        assert_eq!(body["schema_version"], "overkey.phase3.response.v0");
        assert_eq!(body["service"], "service:overkey");
        assert_eq!(body["trace_id"], "trace:overkey:test");
        assert_eq!(body["data"]["route"], ROUTE_CREATE_API_KEY);
        assert_eq!(body["data"]["raw_secret_persisted"], false);
        assert_eq!(body["data"]["raw_key_discarded"], true);
        assert!(body["data"]["api_key_hash_ref"]
            .as_str()
            .unwrap()
            .starts_with("hash:api_key:blake3:"));
    }

    #[tokio::test]
    async fn api_key_enrollment_requires_blake3_lookup_hash_ref() {
        let router = OverkeyService::default().router();
        let rejected = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/api-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase3")
                    .header(TRACE_HEADER, "trace:overkey:api-key:bad-hash")
                    .body(Body::from(
                        r#"{"api_key_prefix":"ovk_bad","api_key_hash_ref":"hash:api_key:sha256:not-accepted"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(rejected.status(), StatusCode::BAD_REQUEST);
        let rejected_body = response_json(rejected).await;
        assert_eq!(
            rejected_body["reason_code"],
            "overkey.api_key_hash_ref_invalid"
        );

        let raw_key = "phase3-keyed-lookup-test-key";
        let accepted = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/api-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase3")
                    .header(TRACE_HEADER, "trace:overkey:api-key:keyed-hash")
                    .body(Body::from(format!(
                        r#"{{"api_key_prefix":"ovk_keyed","raw_api_key":"{raw_key}"}}"#
                    )))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(accepted.status(), StatusCode::OK);
        let accepted_body = response_json(accepted).await;
        let hash_ref = accepted_body["data"]["api_key_hash_ref"].as_str().unwrap();
        assert!(hash_ref.starts_with(API_KEY_HASH_REF_PREFIX));
        let plain_hash = blake3::hash(raw_key.as_bytes());
        assert_ne!(hash_ref, format!("hash:api_key:blake3:{plain_hash}"));
    }

    #[tokio::test]
    async fn base_path_routes_to_same_surface() {
        let response = OverkeyService::default()
            .router()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/overkey/v1/healthz")
                    .header(TRACE_HEADER, "trace:overkey:base-path")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response_json(response).await;
        assert_eq!(body["trace_id"], "trace:overkey:base-path");
        assert_eq!(body["data"]["service_id"], "service:overkey");
    }

    #[tokio::test]
    async fn credential_routes_require_json_and_tenant_context() {
        let router = OverkeyService::default().router();
        let missing_json = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/signing-keys")
                    .header(TENANT_HEADER, "tenant:phase2")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(missing_json.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        let missing_tenant = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/signing-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(missing_tenant.status(), StatusCode::BAD_REQUEST);
        let body = response_json(missing_tenant).await;
        assert_eq!(body["reason_code"], "overkey.tenant_context_required");
    }

    #[tokio::test]
    async fn verification_routes_require_internal_service_account_headers() {
        let router = OverkeyService::default().router();
        let denied = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/signature")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase2")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(denied.status(), StatusCode::FORBIDDEN);
        let denied_body = response_json(denied).await;
        assert_eq!(denied_body["reason_code"], "auth.service_account_required");

        let allowed = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/api-key")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase2")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:local-fixture")
                    .header(TRACE_HEADER, "trace:overkey:verify")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(allowed.status(), StatusCode::OK);
        let allowed_body = response_json(allowed).await;
        assert_eq!(allowed_body["trace_id"], "trace:overkey:verify");
        assert_eq!(allowed_body["data"]["internal_only"], true);
        assert_eq!(
            allowed_body["data"]["schema_ref"],
            "schema:overkey:verification_result:v0"
        );
    }

    #[tokio::test]
    async fn signing_key_enrollment_rejects_duplicate_active_key_ids() {
        let router = OverkeyService::default().router();
        let request = r#"{
            "credential_id":"credential:signing:phase3-a",
            "subject_ref":"actor:overpass:phase3",
            "key_id":"key:tenant:phase3-shared",
            "public_key_ref":"public-key-ref:ed25519:phase3",
            "not_after":"2026-12-31T23:59:59Z",
            "protection_class":"protection:tenant_bound_public_key"
        }"#;
        let first = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/signing-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase3")
                    .header(TRACE_HEADER, "trace:overkey:signing:first")
                    .body(Body::from(request))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(first.status(), StatusCode::OK);

        let duplicate = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/signing-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase3")
                    .header(TRACE_HEADER, "trace:overkey:signing:duplicate")
                    .body(Body::from(
                        r#"{
                            "credential_id":"credential:signing:phase3-b",
                            "subject_ref":"actor:overpass:phase3",
                            "key_id":"key:tenant:phase3-shared",
                            "public_key_ref":"public-key-ref:ed25519:phase3-b",
                            "not_after":"2026-12-31T23:59:59Z",
                            "protection_class":"protection:tenant_bound_public_key"
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(duplicate.status(), StatusCode::CONFLICT);
        let body = response_json(duplicate).await;
        assert_eq!(body["reason_code"], "overkey.duplicate_active_key_rejected");
    }

    #[tokio::test]
    async fn service_account_enrollment_requires_signed_narrow_scope() {
        let router = OverkeyService::default().router();
        let unsigned = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/service-accounts")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase3")
                    .body(Body::from(
                        r#"{"service_account_id":"service-account:overgate","allowed_services":["service:overgate"],"allowed_command_classes":["command.verify"]}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(unsigned.status(), StatusCode::FORBIDDEN);

        let broad = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/service-accounts")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase3")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:phase3")
                    .body(Body::from(
                        r#"{"service_account_id":"service-account:overgate","allowed_services":["*"],"allowed_command_classes":["command.verify"]}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(broad.status(), StatusCode::BAD_REQUEST);
        let body = response_json(broad).await;
        assert_eq!(
            body["reason_code"],
            "overkey.broad_service_account_scope_rejected"
        );

        let unsupported = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/service-accounts")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase3")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:phase3")
                    .body(Body::from(
                        r#"{"service_account_id":"service-account:overgate","allowed_services":["service:unknown"],"allowed_command_classes":["command.verify"]}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(unsupported.status(), StatusCode::BAD_REQUEST);
        let unsupported_body = response_json(unsupported).await;
        assert_eq!(
            unsupported_body["reason_code"],
            "overkey.broad_service_account_scope_rejected"
        );
    }

    #[tokio::test]
    async fn metadata_reads_are_tenant_scoped_and_redacted() {
        let router = OverkeyService::default().router();
        let create = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/api-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase3-a")
                    .header(TRACE_HEADER, "trace:overkey:metadata")
                    .body(Body::from(
                        r#"{"credential_id":"credential:api-key:metadata","api_key_prefix":"ovk_meta","raw_api_key":"metadata-local-key"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create.status(), StatusCode::OK);

        let found = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/v1/credentials/credential:api-key:metadata")
                    .header(TENANT_HEADER, "tenant:phase3-a")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let found_body = response_json(found).await;
        assert_eq!(found_body["data"]["credential_known"], true);
        assert_eq!(found_body["data"]["raw_secret_persisted"], false);
        assert!(found_body["data"]["redacted_fields"]
            .as_array()
            .unwrap()
            .iter()
            .any(|field| field == "raw_api_key"));

        let isolated = router
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/v1/credentials/credential:api-key:metadata")
                    .header(TENANT_HEADER, "tenant:phase3-b")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let isolated_body = response_json(isolated).await;
        assert_eq!(isolated_body["data"]["credential_known"], false);
    }

    #[tokio::test]
    async fn last_used_updates_safe_metadata_summary() {
        let router = OverkeyService::default().router();
        let create = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/api-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase3")
                    .header(TRACE_HEADER, "trace:overkey:last-used-create")
                    .body(Body::from(
                        r#"{"credential_id":"credential:api-key:last-used","api_key_prefix":"ovk_used","raw_api_key":"last-used-local-key"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create.status(), StatusCode::OK);

        let update = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/usage/last-used")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase3")
                    .body(Body::from(
                        r#"{"credential_id":"credential:api-key:last-used","used_at":"2026-06-26T12:00:00Z"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(update.status(), StatusCode::OK);

        let lookup = router
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/v1/credentials/credential:api-key:last-used")
                    .header(TENANT_HEADER, "tenant:phase3")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = response_json(lookup).await;
        assert_eq!(body["data"]["last_used_at"], "2026-06-26T12:00:00Z");
    }

    #[tokio::test]
    async fn readyz_reports_dependency_matrix() {
        let response = OverkeyService::default()
            .router()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/v1/readyz")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response_json(response).await;
        assert_eq!(body["reason_code"], "overkey.ready");
        assert_eq!(body["data"]["ready"], true);
        assert!(body["data"]["dependency_matrix"]["checks"]
            .as_array()
            .unwrap()
            .iter()
            .any(|check| check["dependency_id"] == "overvault_secret_ref_resolver"));
    }

    #[tokio::test]
    async fn local_fixture_credential_smoke_uses_overkey_base_path() {
        let fixture: Value = serde_json::from_str(include_str!(
            "../fixtures/valid/phase2_local_credential.valid.json"
        ))
        .unwrap();

        assert_eq!(
            fixture["local_stack_service"]["service_id"],
            "service:overkey"
        );
        assert_eq!(
            fixture["local_stack_service"]["port_owner_service_id"],
            "service:api"
        );
        assert_eq!(fixture["local_stack_service"]["port"], 18080);
        assert_eq!(fixture["local_stack_service"]["base_path"], "/overkey");
        assert_eq!(fixture["credential_record"]["algorithm"], "Ed25519");
        assert_eq!(
            fixture["credential_record"]["canonicalization"],
            "overrid.canonical_json.v0"
        );
    }

    async fn response_json(response: axum::response::Response) -> Value {
        let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }
}
