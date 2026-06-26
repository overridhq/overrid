use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::errors::OverkeyError;
use crate::records::{
    AffectedInventory, CacheInvalidation, CredentialRecord, CredentialStatus, PropagationStatus,
    DelegationRecord, PolicyHandoff, RevocationRecord, RotationRecord, SecretRef,
    VerificationResult,
};
use crate::repository::CredentialMetadataRepository;
use crate::schema::{
    API_KEY_RECORD_SCHEMA_REF, CREDENTIAL_RECORD_SCHEMA_REF, DELEGATION_RECORD_SCHEMA_REF,
    OVERKEY_PHASE3_RESPONSE_SCHEMA_VERSION, OVERKEY_PHASE4_RESPONSE_SCHEMA_VERSION,
    OVERKEY_PHASE5_RESPONSE_SCHEMA_VERSION, OVERKEY_PHASE6_RESPONSE_SCHEMA_VERSION,
    PUBLIC_KEY_RECORD_SCHEMA_REF,
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
pub const ROUTE_CREATE_DELEGATION: &str = "POST /v1/delegations";
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
const ORDINARY_POSITIVE_CACHE_TTL_SECONDS: u64 = 30;
const HIGH_RISK_POSITIVE_CACHE_TTL_SECONDS: u64 = 5;
const PHASE5_PROPAGATION_SERVICES: [&str; 7] = [
    "service:overgate",
    "service:overvault",
    "service:overqueue",
    "service:oversched",
    "service:overcell",
    "service:system-services",
    "service:product-clients",
];
const APPROVED_VERIFICATION_SERVICE_ACCOUNTS: [&str; 4] = [
    "service-account:overgate",
    "service-account:overkey-internal",
    "service-account:overvault",
    "service-account:system",
];
const SERVICE_ACCOUNT_ALLOWED_SERVICES: [&str; 8] = [
    "service:overgate",
    "service:node-agent",
    "service:system-service",
    "service:worker",
    "service:overvault",
    "service:grid-resident",
    "service:overqueue",
    "service:overrun",
];
const SERVICE_ACCOUNT_ALLOWED_COMMAND_CLASSES: [&str; 10] = [
    "command.verify",
    "command.credential.read",
    "command.credential.rotate",
    "command.credential.revoke",
    "command.node.enroll",
    "command.secret.resolve",
    "command.workload.execute",
    "command.system.operate",
    "command.queue.execution_callback",
    "command.worker.runtime_callback",
];
const PHASE6_DELEGATION_ALLOWED_SCOPES: [&str; 6] = [
    "scope:credential.read",
    "scope:credential.verify",
    "scope:credential.rotate",
    "scope:secret.resolve",
    "scope:queue.callback",
    "scope:runtime.callback",
];
const PHASE2_RESPONSE_SCHEMA_COMPATIBILITY: &str = "overkey.phase2.response.v0";
const SUPPORTED_RESPONSE_SCHEMA_VERSIONS: [&str; 5] = [
    PHASE2_RESPONSE_SCHEMA_COMPATIBILITY,
    OVERKEY_PHASE3_RESPONSE_SCHEMA_VERSION,
    OVERKEY_PHASE4_RESPONSE_SCHEMA_VERSION,
    OVERKEY_PHASE5_RESPONSE_SCHEMA_VERSION,
    OVERKEY_PHASE6_RESPONSE_SCHEMA_VERSION,
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
        Self::new_with_schema(
            OVERKEY_PHASE3_RESPONSE_SCHEMA_VERSION,
            trace_id,
            status,
            reason_code,
            data,
        )
    }

    pub fn new_with_schema(
        schema_version: &'static str,
        trace_id: impl Into<String>,
        status: &'static str,
        reason_code: &'static str,
        data: T,
    ) -> Self {
        Self {
            schema_version,
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
struct Phase5LifecycleData {
    route: &'static str,
    tenant_id: String,
    credential_id: String,
    key_id: String,
    key_version: u32,
    record_kind: &'static str,
    schema_ref: &'static str,
    repository_action: &'static str,
    lifecycle_status: CredentialStatus,
    rotation_record: Option<RotationRecord>,
    revocation_record: Option<RevocationRecord>,
    cache_invalidation: CacheInvalidation,
    propagation_status: Vec<PropagationStatus>,
    affected_inventory: AffectedInventory,
    break_glass_accepted: bool,
    idempotency_key: Option<String>,
    audit_refs: Vec<String>,
    raw_secret_persisted: bool,
    redacted_fields: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct Phase6DelegationData {
    route: &'static str,
    tenant_id: String,
    delegation_id: String,
    record_kind: &'static str,
    schema_ref: &'static str,
    repository_action: &'static str,
    delegator_ref: String,
    delegate_ref: String,
    allowed_scopes: Vec<String>,
    allowed_command_classes: Vec<String>,
    not_after: String,
    revocation_state: String,
    evidence_refs: Vec<String>,
    policy_handoff: PolicyHandoff,
    overgate_admission_required: bool,
    overwatch_event_ref: String,
    audit_refs: Vec<String>,
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
    key_id: String,
    key_version: u32,
    subject_ref: String,
    allowed_use: String,
    command_class: String,
    verification_class: &'static str,
    verified: bool,
    verification_state: String,
    reason_code: String,
    schema_ref: &'static str,
    internal_only: bool,
    service_account_ref: String,
    algorithm: String,
    canonicalization: String,
    body_hash_ref: String,
    request_hash_ref: String,
    verification_evidence_ref: String,
    revocation_epoch: u64,
    cache_guidance: CacheGuidance,
    policy_handoff: PolicyHandoff,
    audit_refs: Vec<String>,
    retryability: String,
    raw_secret_persisted: bool,
    redacted_fields: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
struct CacheGuidance {
    cacheable: bool,
    max_positive_ttl_seconds: u64,
    revocation_epoch: u64,
    cache_key_ref: String,
}

#[derive(Debug, Serialize)]
struct UsageData {
    route: &'static str,
    tenant_id: String,
    credential_id: String,
    last_used_at: String,
    usage_recorded: bool,
    retry_safe_update_queued: bool,
    overwatch_event_ref: String,
    overmeter_event_refs: Vec<String>,
    usage_event_classes: Vec<String>,
    oru_balance_mutated: bool,
    seal_ledger_mutated: bool,
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
struct DelegationRequest {
    delegation_id: Option<String>,
    delegator_ref: String,
    delegate_ref: String,
    allowed_scopes: Vec<String>,
    allowed_command_classes: Vec<String>,
    not_after: String,
    revocation_state: Option<String>,
    evidence_refs: Option<Vec<String>>,
    audit_refs: Option<Vec<String>>,
    overguard_policy_decision_ref: Option<String>,
    overguard_decision: Option<String>,
    overpass_delegate_state: Option<String>,
    overtenant_tenant_state: Option<String>,
    overtenant_membership_state: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LifecycleRequest {
    reason_code: Option<String>,
    audit_ref: Option<String>,
    successor_credential_id: Option<String>,
    successor_key_id: Option<String>,
    successor_key_version: Option<u32>,
    grace_window_seconds: Option<u64>,
    initiated_by: Option<String>,
    activation_at: Option<String>,
    revoked_by: Option<String>,
    effective_at: Option<String>,
    affected_command_classes: Option<Vec<String>>,
    incident_refs: Option<Vec<String>>,
    evidence_refs: Option<Vec<String>>,
    expected_current_status: Option<CredentialStatus>,
    break_glass: Option<bool>,
    operator_role: Option<String>,
    protection_class: Option<String>,
    overgate_command_signature: Option<String>,
    idempotency_key: Option<String>,
    operator_lifecycle: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct LastUsedRequest {
    credential_id: String,
    used_at: Option<String>,
    audit_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SignatureVerificationRequest {
    credential_id: String,
    key_id: String,
    key_version: u32,
    algorithm: Option<String>,
    canonicalization: Option<String>,
    timestamp: String,
    replay_window_id: Option<String>,
    body_hash_ref: String,
    body_hash_payload: Option<String>,
    allowed_use: String,
    command_class: String,
    tenant_id: Option<String>,
    subject_ref: Option<String>,
    signature_ref: Option<String>,
    revocation_epoch: Option<u64>,
    rotation_window_state: Option<String>,
    overpass_subject_state: Option<String>,
    overtenant_tenant_state: Option<String>,
    overtenant_membership_state: Option<String>,
    overguard_policy_decision_ref: Option<String>,
    overguard_decision: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiKeyVerificationRequest {
    credential_id: String,
    api_key_prefix: String,
    api_key_hash_ref: Option<String>,
    raw_api_key: Option<String>,
    timestamp: String,
    replay_window_id: Option<String>,
    allowed_use: String,
    command_class: String,
    tenant_id: Option<String>,
    subject_ref: Option<String>,
    revocation_epoch: Option<u64>,
    overpass_subject_state: Option<String>,
    overtenant_tenant_state: Option<String>,
    overtenant_membership_state: Option<String>,
    overguard_policy_decision_ref: Option<String>,
    overguard_decision: Option<String>,
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
        .route("/v1/delegations", post(create_delegation))
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
        api_key_prefix: Some(prefix.clone()),
        api_key_hash_ref: Some(api_key_hash_ref.clone()),
        public_key_ref: None,
        key_fingerprint_ref: None,
        allowed_services: vec!["service:overgate".to_owned()],
        allowed_command_classes: vec!["command.verify".to_owned()],
        revocation_epoch: 0,
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
            allowed_services: vec!["service:overgate".to_owned()],
            allowed_command_classes: vec!["command.verify".to_owned()],
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
        api_key_prefix: None,
        api_key_hash_ref: None,
        public_key_ref: Some(request.public_key_ref.clone()),
        key_fingerprint_ref: Some(key_fingerprint_ref.clone()),
        allowed_services: vec!["service:overgate".to_owned()],
        allowed_command_classes: vec!["command.verify".to_owned()],
        revocation_epoch: 0,
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
        api_key_prefix: None,
        api_key_hash_ref: None,
        public_key_ref: Some(public_key_ref.clone()),
        key_fingerprint_ref: Some(blake3_ref("key-fingerprint", &public_key_ref)),
        allowed_services: request.allowed_services.clone(),
        allowed_command_classes: request.allowed_command_classes.clone(),
        revocation_epoch: 0,
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
            key_fingerprint_ref: Some(blake3_ref("key-fingerprint", &request.service_account_id)),
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

async fn create_delegation(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<ApiResponse<Phase6DelegationData>>, OverkeyError> {
    let trace_id = trace_id(&headers);
    require_internal_service_account(&headers).map_err(|_| {
        OverkeyError::repository_rejected(
            trace_id.clone(),
            crate::repository::RepositoryError::UnsignedServiceAccountCall,
        )
    })?;
    let tenant_id = tenant_from_headers(&headers)?;
    let request: DelegationRequest = parse_json_body(&headers, body)?;
    let policy_handoff = validate_delegation_request(&headers, &request, &trace_id)?;
    let delegation_id = request
        .delegation_id
        .clone()
        .unwrap_or_else(|| format!("delegation:overkey:{}", stable_trace_token(&trace_id)));
    let evidence_refs = non_empty_refs(
        request.evidence_refs.clone(),
        format!(
            "evidence:overkey:delegation:{}",
            stable_trace_token(&trace_id)
        ),
    );
    let audit_refs = non_empty_refs(
        request.audit_refs.clone(),
        format!(
            "audit:overkey:delegation:{}",
            stable_trace_token(&trace_id)
        ),
    );
    let revocation_state = request
        .revocation_state
        .clone()
        .unwrap_or_else(|| "revocable".to_owned());
    let record = DelegationRecord {
        delegation_id: delegation_id.clone(),
        tenant_id: tenant_id.clone(),
        delegator_ref: request.delegator_ref.clone(),
        delegate_ref: request.delegate_ref.clone(),
        subject_ref: request.delegator_ref.clone(),
        delegated_to: request.delegate_ref.clone(),
        allowed_scopes: request.allowed_scopes.clone(),
        allowed_uses: request.allowed_scopes.clone(),
        allowed_command_classes: request.allowed_command_classes.clone(),
        not_after: request.not_after.clone(),
        revocation_state: revocation_state.clone(),
        evidence_refs: evidence_refs.clone(),
        policy_decision_ref: policy_handoff.decision_ref.clone(),
        status: CredentialStatus::Active,
        audit_refs: audit_refs.clone(),
    };
    state
        .repository
        .append_delegation_record(record)
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response_with_schema(
        OVERKEY_PHASE6_RESPONSE_SCHEMA_VERSION,
        trace_id,
        "overkey.delegation_recorded_phase6",
        Phase6DelegationData {
            route: ROUTE_CREATE_DELEGATION,
            tenant_id,
            delegation_id,
            record_kind: "delegation_record",
            schema_ref: DELEGATION_RECORD_SCHEMA_REF,
            repository_action: "append_delegation_record",
            delegator_ref: request.delegator_ref,
            delegate_ref: request.delegate_ref,
            allowed_scopes: request.allowed_scopes,
            allowed_command_classes: request.allowed_command_classes,
            not_after: request.not_after,
            revocation_state,
            evidence_refs,
            policy_handoff,
            overgate_admission_required: true,
            overwatch_event_ref: format!("event:overwatch:overkey:delegation:{delegation_id}"),
            audit_refs,
            raw_secret_persisted: false,
            redacted_fields: safe_metadata_redactions(),
        },
    ))
}

async fn rotate_credential(
    State(state): State<OverkeyState>,
    Path(credential_id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<ApiResponse<Phase5LifecycleData>>, OverkeyError> {
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
    validate_operator_lifecycle_request(&headers, &request, &current, &trace_id)?;
    let audit_ref = request
        .audit_ref
        .clone()
        .unwrap_or_else(|| format!("audit:overkey:rotation:{}", stable_trace_token(&trace_id)));
    let reason_code = request
        .reason_code
        .clone()
        .unwrap_or_else(|| "overkey.rotation_requested".to_owned());
    let successor_credential_id = request.successor_credential_id.clone().unwrap_or_else(|| {
        format!(
            "{}:successor:{}",
            credential_id,
            stable_trace_token(&trace_id)
        )
    });
    let successor_key_version = request
        .successor_key_version
        .unwrap_or(current.key_version + 1);
    let successor_key_id = request
        .successor_key_id
        .clone()
        .unwrap_or_else(|| format!("{}:v{}", current.key_id, successor_key_version));
    validate_rotation_successor(
        &state,
        &tenant_id,
        &credential_id,
        &successor_credential_id,
        &successor_key_id,
        successor_key_version,
        &current,
        &trace_id,
    )?;
    let revocation_epoch = current.revocation_epoch + 1;
    let propagation_status = default_propagation_status("rotation", &audit_ref);
    let affected_command_classes = affected_command_classes(&request, &current);
    let rotation_record = RotationRecord {
        rotation_id: format!(
            "rotation:{}:{}",
            credential_id,
            stable_trace_token(&trace_id)
        ),
        tenant_id: tenant_id.clone(),
        credential_id: credential_id.clone(),
        predecessor_credential_id: credential_id.clone(),
        predecessor_key_id: current.key_id.clone(),
        predecessor_key_version: current.key_version,
        successor_credential_id: successor_credential_id.clone(),
        successor_key_id: successor_key_id.clone(),
        successor_key_version,
        grace_window_seconds: request.grace_window_seconds.unwrap_or(300),
        rotation_state: "rotation_started".to_owned(),
        initiated_by: request
            .initiated_by
            .clone()
            .unwrap_or_else(|| "actor:overgate:lifecycle".to_owned()),
        reason_code: reason_code.clone(),
        activation_at: request
            .activation_at
            .clone()
            .unwrap_or_else(|| "2026-06-26T00:05:00Z".to_owned()),
        evidence_refs: non_empty_refs(
            request.evidence_refs.clone(),
            format!(
                "evidence:overkey:rotation:{}",
                stable_trace_token(&trace_id)
            ),
        ),
        revocation_epoch,
        propagation_status: propagation_status.clone(),
        audit_refs: vec![audit_ref.clone()],
    };
    state
        .repository
        .append_rotation_record(rotation_record.clone())
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response_with_schema(
        OVERKEY_PHASE5_RESPONSE_SCHEMA_VERSION,
        trace_id,
        "overkey.rotation_started_phase5",
        phase5_lifecycle_data(Phase5LifecycleInput {
            route: ROUTE_ROTATE_CREDENTIAL,
            tenant_id,
            credential_id,
            key_id: successor_key_id,
            key_version: successor_key_version,
            record_kind: "rotation_record",
            schema_ref: ROTATION_RECORD_SCHEMA_REF,
            repository_action: "append_rotation_record",
            lifecycle_status: CredentialStatus::Rotating,
            rotation_record: Some(rotation_record),
            revocation_record: None,
            revocation_epoch,
            invalidation_reason: reason_code,
            affected_command_classes,
            subject_ref: current.subject_ref,
            propagation_status,
            break_glass_accepted: false,
            idempotency_key: None,
            audit_refs: vec![audit_ref],
        }),
    ))
}

async fn revoke_credential(
    State(state): State<OverkeyState>,
    Path(credential_id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<ApiResponse<Phase5LifecycleData>>, OverkeyError> {
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
    validate_operator_lifecycle_request(&headers, &request, &current, &trace_id)?;
    let audit_ref = request
        .audit_ref
        .clone()
        .unwrap_or_else(|| format!("audit:overkey:revocation:{}", stable_trace_token(&trace_id)));
    validate_break_glass_request(&headers, &request, &current, &trace_id)?;
    if let Some(idempotency_key) = request.idempotency_key.as_deref() {
        if let Some(existing) = state
            .repository
            .revocation_records(&credential_id)
            .into_iter()
            .find(|record| {
                record.tenant_id == tenant_id
                    && record.idempotency_key.as_deref() == Some(idempotency_key)
            })
        {
            let propagation_status = existing.propagation_status.clone();
            return Ok(json_response_with_schema(
                OVERKEY_PHASE5_RESPONSE_SCHEMA_VERSION,
                trace_id,
                "overkey.break_glass_revocation_idempotent_replay",
                phase5_lifecycle_data(Phase5LifecycleInput {
                    route: ROUTE_REVOKE_CREDENTIAL,
                    tenant_id,
                    credential_id,
                    key_id: current.key_id,
                    key_version: current.key_version,
                    record_kind: "revocation_record",
                    schema_ref: REVOCATION_RECORD_SCHEMA_REF,
                    repository_action: "idempotent_replay",
                    lifecycle_status: CredentialStatus::Revoked,
                    rotation_record: None,
                    revocation_record: Some(existing.clone()),
                    revocation_epoch: existing.revocation_epoch,
                    invalidation_reason: existing.reason_code,
                    affected_command_classes: existing.affected_command_classes,
                    subject_ref: current.subject_ref,
                    propagation_status,
                    break_glass_accepted: existing.break_glass,
                    idempotency_key: existing.idempotency_key,
                    audit_refs: existing.audit_refs,
                }),
            ));
        }
    }
    let reason_code = request
        .reason_code
        .clone()
        .unwrap_or_else(|| "overkey.revocation_requested".to_owned());
    let affected_command_classes = affected_command_classes(&request, &current);
    let revocation_epoch = current.revocation_epoch + 1;
    let break_glass = request.break_glass.unwrap_or(false);
    let propagation_status = default_propagation_status("revocation", &audit_ref);
    let idempotency_key = request.idempotency_key.clone();
    let revocation_record = RevocationRecord {
        revocation_id: format!(
            "revocation:{}:{}",
            credential_id,
            stable_trace_token(&trace_id)
        ),
        tenant_id: tenant_id.clone(),
        credential_id: credential_id.clone(),
        revoked_by: request
            .revoked_by
            .clone()
            .or_else(|| request.initiated_by.clone())
            .unwrap_or_else(|| "actor:overgate:lifecycle".to_owned()),
        revoked_at: LOCAL_PHASE3_TIMESTAMP.to_owned(),
        effective_at: request
            .effective_at
            .clone()
            .unwrap_or_else(|| LOCAL_PHASE3_TIMESTAMP.to_owned()),
        reason_code: reason_code.clone(),
        affected_command_classes: affected_command_classes.clone(),
        incident_refs: request
            .incident_refs
            .clone()
            .unwrap_or_else(|| vec!["incident:overkey:phase5:local-fixture".to_owned()]),
        evidence_refs: non_empty_refs(
            request.evidence_refs.clone(),
            format!(
                "evidence:overkey:revocation:{}",
                stable_trace_token(&trace_id)
            ),
        ),
        expected_current_status: request
            .expected_current_status
            .clone()
            .unwrap_or_else(|| current.status.clone()),
        revocation_epoch,
        break_glass,
        idempotency_key,
        propagation_status: propagation_status.clone(),
        audit_refs: vec![audit_ref.clone()],
    };
    state
        .repository
        .append_revocation_record(revocation_record.clone())
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;
    let response_idempotency_key = revocation_record.idempotency_key.clone();

    Ok(json_response_with_schema(
        OVERKEY_PHASE5_RESPONSE_SCHEMA_VERSION,
        trace_id,
        if break_glass {
            "overkey.break_glass_revocation_accepted_phase5"
        } else {
            "overkey.revocation_recorded_phase5"
        },
        phase5_lifecycle_data(Phase5LifecycleInput {
            route: ROUTE_REVOKE_CREDENTIAL,
            tenant_id,
            credential_id,
            key_id: current.key_id,
            key_version: current.key_version,
            record_kind: "revocation_record",
            schema_ref: REVOCATION_RECORD_SCHEMA_REF,
            repository_action: "append_revocation_record",
            lifecycle_status: CredentialStatus::Revoked,
            rotation_record: None,
            revocation_record: Some(revocation_record),
            revocation_epoch,
            invalidation_reason: reason_code,
            affected_command_classes,
            subject_ref: current.subject_ref,
            propagation_status,
            break_glass_accepted: break_glass,
            idempotency_key: response_idempotency_key,
            audit_refs: vec![audit_ref],
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
    body: Bytes,
) -> Result<Json<ApiResponse<VerificationData>>, OverkeyError> {
    require_json(&headers)?;
    require_internal_service_account(&headers)?;
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let request: SignatureVerificationRequest = parse_json_body(&headers, body)?;
    let service_account_ref = header_value(&headers, SERVICE_ACCOUNT_HEADER)
        .unwrap_or_else(|| "service-account:unknown".to_owned());
    let data = verify_signature_request(&state, tenant_id, service_account_ref, request);
    state
        .repository
        .record_verification(verification_result_from_data(&data))
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response_with_schema(
        OVERKEY_PHASE4_RESPONSE_SCHEMA_VERSION,
        trace_id,
        verification_response_reason(&data.verification_state),
        data,
    ))
}

async fn verify_api_key(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<ApiResponse<VerificationData>>, OverkeyError> {
    require_json(&headers)?;
    require_internal_service_account(&headers)?;
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let request: ApiKeyVerificationRequest = parse_json_body(&headers, body)?;
    let service_account_ref = header_value(&headers, SERVICE_ACCOUNT_HEADER)
        .unwrap_or_else(|| "service-account:unknown".to_owned());
    let data = verify_api_key_request(&state, tenant_id, service_account_ref, request);
    state
        .repository
        .record_verification(verification_result_from_data(&data))
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response_with_schema(
        OVERKEY_PHASE4_RESPONSE_SCHEMA_VERSION,
        trace_id,
        verification_response_reason(&data.verification_state),
        data,
    ))
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
    let audit_ref = request.audit_ref.unwrap_or_else(|| {
        format!("audit:overkey:last-used:{}", stable_trace_token(&trace_id))
    });
    let update_result = state
        .repository
        .update_last_used(
            &tenant_id,
            &request.credential_id,
            used_at.clone(),
            audit_ref.clone(),
        );
    let (usage_recorded, retry_safe_update_queued, response_reason) = match update_result {
        Ok(()) => (true, false, "overkey.usage_recorded_phase6"),
        Err(crate::repository::RepositoryError::CredentialNotFound) => {
            (false, true, "overkey.usage_update_queued_after_repository_miss")
        }
        Err(error) => return Err(OverkeyError::repository_rejected(trace_id.clone(), error)),
    };

    Ok(json_response(
        trace_id.clone(),
        response_reason,
        UsageData {
            route: ROUTE_USAGE_LAST_USED,
            tenant_id,
            credential_id: request.credential_id.clone(),
            last_used_at: used_at,
            usage_recorded,
            retry_safe_update_queued,
            overwatch_event_ref: format!("event:overwatch:overkey:last-used:{trace_id}"),
            overmeter_event_refs: vec![
                format!("event:overmeter:overkey:verification-volume:{trace_id}"),
                format!("event:overmeter:overkey:credential-class:{trace_id}"),
            ],
            usage_event_classes: vec![
                "overkey.verification_volume".to_owned(),
                "overkey.lifecycle_operation".to_owned(),
                "overkey.service_account_use".to_owned(),
                "overkey.credential_class_usage".to_owned(),
            ],
            oru_balance_mutated: false,
            seal_ledger_mutated: false,
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

fn verify_signature_request(
    state: &OverkeyState,
    header_tenant_id: String,
    service_account_ref: String,
    request: SignatureVerificationRequest,
) -> VerificationData {
    let tenant_id = request
        .tenant_id
        .clone()
        .unwrap_or(header_tenant_id.clone());
    let subject_ref = request
        .subject_ref
        .clone()
        .unwrap_or_else(|| "actor:overpass:unknown".to_owned());
    let mut data = verification_data_base(
        ROUTE_VERIFY_SIGNATURE,
        tenant_id.clone(),
        request.credential_id.clone(),
        request.key_id.clone(),
        request.key_version,
        subject_ref,
        request.allowed_use.clone(),
        request.command_class.clone(),
        "signature",
        service_account_ref.clone(),
        request
            .algorithm
            .clone()
            .unwrap_or_else(|| SIGNING_ALGORITHM.to_owned()),
        request
            .canonicalization
            .clone()
            .unwrap_or_else(|| CANONICALIZATION_VERSION.to_owned()),
        request.body_hash_ref.clone(),
        request.timestamp.as_str(),
        request.replay_window_id.as_deref(),
        request.revocation_epoch.unwrap_or(0),
    );
    data.policy_handoff = policy_handoff_from_decision(
        request.overguard_policy_decision_ref.as_deref(),
        request.overguard_decision.as_deref(),
        &request.command_class,
    );

    let record = state
        .repository
        .credential(&tenant_id, &request.credential_id);
    let denial = verification_denial_for_common_checks(
        &data,
        &header_tenant_id,
        &service_account_ref,
        record.as_ref(),
        request.timestamp.as_str(),
        request.replay_window_id.as_deref(),
        request.revocation_epoch,
        request.overpass_subject_state.as_deref(),
        request.overtenant_tenant_state.as_deref(),
        request.overtenant_membership_state.as_deref(),
    )
    .or_else(|| {
        let record = record.as_ref()?;
        if request.algorithm.as_deref().unwrap_or(SIGNING_ALGORITHM) != SIGNING_ALGORITHM {
            return Some("auth.signature_algorithm_denied");
        }
        if request
            .canonicalization
            .as_deref()
            .unwrap_or(CANONICALIZATION_VERSION)
            != CANONICALIZATION_VERSION
        {
            return Some("auth.signature_canonicalization_denied");
        }
        if record.key_id != request.key_id {
            return Some("auth.key_id_mismatch");
        }
        if record.key_version != request.key_version {
            return Some("auth.key_version_mismatch");
        }
        if record.public_key_ref.is_none() {
            return Some("auth.public_key_ref_missing");
        }
        if let Some(subject_ref) = &request.subject_ref {
            if record.subject_ref != *subject_ref {
                return Some("auth.subject_mismatch");
            }
        }
        if !record
            .allowed_uses
            .iter()
            .any(|value| value == &request.allowed_use)
        {
            return Some("auth.allowed_use_denied");
        }
        if !command_class_allowed(record, &request.command_class) {
            return Some("auth.command_class_denied");
        }
        if let Some(reason_code) = policy_denial_for_high_risk(
            &request.command_class,
            request.overguard_policy_decision_ref.as_deref(),
            request.overguard_decision.as_deref(),
        ) {
            return Some(reason_code);
        }
        if !body_hash_ref_valid(&request.body_hash_ref, request.body_hash_payload.as_deref()) {
            return Some("auth.body_hash_mismatch");
        }
        if matches!(
            request.rotation_window_state.as_deref(),
            Some("rotation:denied" | "rotation:expired" | "rotated_out")
        ) {
            return Some("auth.credential_rotation_denied");
        }
        if request
            .signature_ref
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
        {
            return Some("auth.signature_ref_required");
        }
        None
    });

    apply_verification_decision(&mut data, denial);
    data
}

fn verify_api_key_request(
    state: &OverkeyState,
    header_tenant_id: String,
    service_account_ref: String,
    request: ApiKeyVerificationRequest,
) -> VerificationData {
    let tenant_id = request
        .tenant_id
        .clone()
        .unwrap_or(header_tenant_id.clone());
    let record = state
        .repository
        .credential(&tenant_id, &request.credential_id);
    let key_id = record
        .as_ref()
        .map(|record| record.key_id.clone())
        .unwrap_or_else(|| format!("key:api:{}", request.api_key_prefix));
    let key_version = record
        .as_ref()
        .map(|record| record.key_version)
        .unwrap_or(0);
    let subject_ref = request
        .subject_ref
        .clone()
        .or_else(|| record.as_ref().map(|record| record.subject_ref.clone()))
        .unwrap_or_else(|| "actor:overpass:unknown".to_owned());
    let body_hash_ref = blake3_ref(
        "overkey-api-key-verification-body",
        &format!(
            "{}:{}:{}:{}",
            tenant_id, request.credential_id, request.api_key_prefix, request.timestamp
        ),
    );
    let mut data = verification_data_base(
        ROUTE_VERIFY_API_KEY,
        tenant_id.clone(),
        request.credential_id.clone(),
        key_id,
        key_version,
        subject_ref,
        request.allowed_use.clone(),
        request.command_class.clone(),
        "api_key",
        service_account_ref.clone(),
        API_KEY_LOOKUP_HASH_ALGORITHM.to_owned(),
        CANONICALIZATION_VERSION.to_owned(),
        body_hash_ref,
        request.timestamp.as_str(),
        request.replay_window_id.as_deref(),
        request.revocation_epoch.unwrap_or(0),
    );
    data.policy_handoff = policy_handoff_from_decision(
        request.overguard_policy_decision_ref.as_deref(),
        request.overguard_decision.as_deref(),
        &request.command_class,
    );

    let denial = verification_denial_for_common_checks(
        &data,
        &header_tenant_id,
        &service_account_ref,
        record.as_ref(),
        request.timestamp.as_str(),
        request.replay_window_id.as_deref(),
        request.revocation_epoch,
        request.overpass_subject_state.as_deref(),
        request.overtenant_tenant_state.as_deref(),
        request.overtenant_membership_state.as_deref(),
    )
    .or_else(|| {
        let record = record.as_ref()?;
        if record.api_key_prefix.as_deref() != Some(request.api_key_prefix.as_str()) {
            return Some("auth.api_key_prefix_unknown");
        }
        let supplied_hash_ref = request
            .api_key_hash_ref
            .clone()
            .or_else(|| request.raw_api_key.as_deref().map(api_key_lookup_hash_ref));
        if supplied_hash_ref.as_deref().unwrap_or("")
            != record.api_key_hash_ref.as_deref().unwrap_or("")
        {
            return Some("auth.api_key_hash_mismatch");
        }
        if let Some(subject_ref) = &request.subject_ref {
            if record.subject_ref != *subject_ref {
                return Some("auth.subject_mismatch");
            }
        }
        if !record
            .allowed_uses
            .iter()
            .any(|value| value == &request.allowed_use)
        {
            return Some("auth.allowed_use_denied");
        }
        if !command_class_allowed(record, &request.command_class) {
            return Some("auth.command_class_denied");
        }
        if let Some(reason_code) = policy_denial_for_high_risk(
            &request.command_class,
            request.overguard_policy_decision_ref.as_deref(),
            request.overguard_decision.as_deref(),
        ) {
            return Some(reason_code);
        }
        None
    });

    apply_verification_decision(&mut data, denial);
    data
}

fn verification_data_base(
    route: &'static str,
    tenant_id: String,
    credential_id: String,
    key_id: String,
    key_version: u32,
    subject_ref: String,
    allowed_use: String,
    command_class: String,
    verification_class: &'static str,
    service_account_ref: String,
    algorithm: String,
    canonicalization: String,
    body_hash_ref: String,
    timestamp: &str,
    replay_window_id: Option<&str>,
    revocation_epoch: u64,
) -> VerificationData {
    let replay_window_id = replay_window_id.unwrap_or("replay:missing");
    let canonical = format!(
        "{route}|{tenant_id}|{credential_id}|{key_id}|{key_version}|{subject_ref}|{allowed_use}|{command_class}|{algorithm}|{canonicalization}|{body_hash_ref}|{timestamp}|{replay_window_id}|{revocation_epoch}"
    );
    let request_hash_ref = blake3_ref("overkey-verification-request", &canonical);
    let verification_evidence_ref = blake3_ref("overkey-verification-evidence", &request_hash_ref);
    let cache_key_ref = blake3_ref(
        "overkey-verification-cache-key",
        &format!(
            "{tenant_id}|{credential_id}|{key_version}|{allowed_use}|{command_class}|{canonicalization}|{revocation_epoch}"
        ),
    );
    let audit_ref = format!(
        "audit:overkey:verify:{verification_class}:{}",
        stable_trace_token(&credential_id)
    );

    VerificationData {
        route,
        tenant_id,
        credential_id,
        key_id,
        key_version,
        subject_ref,
        allowed_use,
        command_class,
        verification_class,
        verified: false,
        verification_state: "denied".to_owned(),
        reason_code: "auth.verification_denied".to_owned(),
        schema_ref: VERIFICATION_RESULT_SCHEMA_REF,
        internal_only: true,
        service_account_ref,
        algorithm,
        canonicalization,
        body_hash_ref,
        request_hash_ref,
        verification_evidence_ref,
        revocation_epoch,
        cache_guidance: CacheGuidance {
            cacheable: false,
            max_positive_ttl_seconds: 0,
            revocation_epoch,
            cache_key_ref,
        },
        policy_handoff: local_policy_handoff("policy:overguard:phase6:local-low-risk"),
        audit_refs: vec![audit_ref],
        retryability: "terminal".to_owned(),
        raw_secret_persisted: false,
        redacted_fields: safe_metadata_redactions(),
    }
}

fn verification_denial_for_common_checks(
    data: &VerificationData,
    header_tenant_id: &str,
    service_account_ref: &str,
    record: Option<&CredentialRecord>,
    timestamp: &str,
    replay_window_id: Option<&str>,
    requested_revocation_epoch: Option<u64>,
    overpass_subject_state: Option<&str>,
    overtenant_tenant_state: Option<&str>,
    overtenant_membership_state: Option<&str>,
) -> Option<&'static str> {
    if !APPROVED_VERIFICATION_SERVICE_ACCOUNTS
        .iter()
        .any(|approved| service_account_ref == *approved)
    {
        return Some("auth.service_account_not_approved");
    }
    if data.tenant_id != header_tenant_id {
        return Some("auth.tenant_header_mismatch");
    }
    if timestamp.trim().is_empty() {
        return Some("auth.timestamp_required");
    }
    if replay_window_id.unwrap_or("").trim().is_empty() {
        return Some("auth.replay_window_required");
    }
    if dependency_state_denied(overpass_subject_state) {
        return Some("auth.subject_dependency_denied");
    }
    if dependency_state_denied(overtenant_tenant_state) {
        return Some("auth.tenant_dependency_denied");
    }
    if dependency_state_denied(overtenant_membership_state) {
        return Some("auth.membership_dependency_denied");
    }
    let record = match record {
        Some(record) => record,
        None => return Some("auth.credential_unknown"),
    };
    if record.tenant_id != data.tenant_id {
        return Some("auth.tenant_scope_denied");
    }
    if !credential_status_allows_verification(record) {
        return Some("auth.credential_not_active");
    }
    if timestamp < record.not_before.as_str() || timestamp > record.not_after.as_str() {
        return Some("auth.signature_expired");
    }
    if requested_revocation_epoch
        .map(|epoch| epoch != record.revocation_epoch)
        .unwrap_or(false)
    {
        return Some("auth.revocation_epoch_mismatch");
    }
    None
}

fn dependency_state_denied(state: Option<&str>) -> bool {
    matches!(
        state,
        Some(
            "disabled"
                | "deleted"
                | "missing"
                | "unknown"
                | "suspended"
                | "inactive"
                | "not_member"
                | "role_denied"
        )
    )
}

fn credential_status_allows_verification(record: &CredentialRecord) -> bool {
    matches!(record.status, CredentialStatus::Active)
}

fn command_class_allowed(record: &CredentialRecord, command_class: &str) -> bool {
    record
        .allowed_command_classes
        .iter()
        .any(|value| value == command_class)
}

fn policy_denial_for_high_risk(
    command_class: &str,
    overguard_policy_decision_ref: Option<&str>,
    overguard_decision: Option<&str>,
) -> Option<&'static str> {
    if !is_high_risk_command_class(command_class) {
        return None;
    }
    if overguard_policy_decision_ref.unwrap_or("").trim().is_empty() {
        return Some("policy.overguard_handoff_required");
    }
    if overguard_decision
        .map(|decision| decision != "allow")
        .unwrap_or(true)
    {
        return Some("policy.overguard_denied");
    }
    None
}

fn body_hash_ref_valid(body_hash_ref: &str, body_hash_payload: Option<&str>) -> bool {
    if !body_hash_ref.starts_with("hash:body:blake3:") {
        return false;
    }
    match body_hash_payload {
        Some(payload) => blake3_ref("body", payload) == body_hash_ref,
        None => true,
    }
}

fn apply_verification_decision(data: &mut VerificationData, denial: Option<&'static str>) {
    if let Some(reason_code) = denial {
        data.verified = false;
        data.verification_state = if reason_code.contains("dependency") {
            "blocked".to_owned()
        } else {
            "denied".to_owned()
        };
        data.reason_code = reason_code.to_owned();
        data.retryability = if data.verification_state == "blocked" {
            "retryable_after_dependency_recovery".to_owned()
        } else {
            "terminal".to_owned()
        };
        data.cache_guidance.cacheable = false;
        data.cache_guidance.max_positive_ttl_seconds = 0;
        data.audit_refs
            .push(format!("audit:overkey:verification-denial:{reason_code}"));
        return;
    }

    data.verified = true;
    data.verification_state = "verified".to_owned();
    data.reason_code = verification_positive_reason(data.verification_class).to_owned();
    data.retryability = "not_retryable_success".to_owned();
    data.cache_guidance.cacheable = true;
    data.cache_guidance.max_positive_ttl_seconds = if is_high_risk_command_class(&data.command_class)
    {
        HIGH_RISK_POSITIVE_CACHE_TTL_SECONDS
    } else {
        ORDINARY_POSITIVE_CACHE_TTL_SECONDS
    };
}

fn verification_positive_reason(verification_class: &str) -> &'static str {
    match verification_class {
        "signature" => "auth.signature_verified_phase4",
        "api_key" => "auth.api_key_verified_phase4",
        _ => "auth.verification_completed_phase4",
    }
}

fn verification_result_from_data(data: &VerificationData) -> VerificationResult {
    VerificationResult {
        tenant_id: data.tenant_id.clone(),
        credential_id: data.credential_id.clone(),
        key_id: data.key_id.clone(),
        key_version: data.key_version,
        subject_ref: data.subject_ref.clone(),
        allowed_use: data.allowed_use.clone(),
        command_class: data.command_class.clone(),
        verified: data.verified,
        verification_state: data.verification_state.clone(),
        verification_class: data.verification_class.to_owned(),
        reason_code: data.reason_code.clone(),
        algorithm: data.algorithm.clone(),
        canonicalization: data.canonicalization.clone(),
        body_hash_ref: data.body_hash_ref.clone(),
        request_hash_ref: data.request_hash_ref.clone(),
        evidence_ref: data.verification_evidence_ref.clone(),
        revocation_epoch: data.revocation_epoch,
        cache_key_ref: data.cache_guidance.cache_key_ref.clone(),
        retryability: data.retryability.clone(),
        audit_refs: data.audit_refs.clone(),
    }
}

fn verification_response_reason(verification_state: &str) -> &'static str {
    match verification_state {
        "verified" => "overkey.verification_completed",
        "blocked" => "overkey.verification_blocked",
        _ => "overkey.verification_denied",
    }
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
    api_key_prefix: Option<String>,
    api_key_hash_ref: Option<String>,
    public_key_ref: Option<String>,
    key_fingerprint_ref: Option<String>,
    allowed_services: Vec<String>,
    allowed_command_classes: Vec<String>,
    revocation_epoch: u64,
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

struct Phase5LifecycleInput {
    route: &'static str,
    tenant_id: String,
    credential_id: String,
    key_id: String,
    key_version: u32,
    record_kind: &'static str,
    schema_ref: &'static str,
    repository_action: &'static str,
    lifecycle_status: CredentialStatus,
    rotation_record: Option<RotationRecord>,
    revocation_record: Option<RevocationRecord>,
    revocation_epoch: u64,
    invalidation_reason: String,
    affected_command_classes: Vec<String>,
    subject_ref: String,
    propagation_status: Vec<PropagationStatus>,
    break_glass_accepted: bool,
    idempotency_key: Option<String>,
    audit_refs: Vec<String>,
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
        api_key_prefix: input.api_key_prefix,
        api_key_hash_ref: input.api_key_hash_ref,
        public_key_ref: input.public_key_ref,
        key_fingerprint_ref: input.key_fingerprint_ref,
        allowed_services: input.allowed_services,
        allowed_command_classes: input.allowed_command_classes,
        revocation_epoch: input.revocation_epoch,
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

fn phase5_lifecycle_data(input: Phase5LifecycleInput) -> Phase5LifecycleData {
    let cache_invalidation = cache_invalidation_for(
        &input.tenant_id,
        &input.credential_id,
        input.key_version,
        &input.affected_command_classes,
        input.revocation_epoch,
        &input.invalidation_reason,
    );
    let affected_inventory = affected_inventory_for(
        &input.tenant_id,
        &input.subject_ref,
        &input.credential_id,
        input.affected_command_classes.clone(),
    );
    Phase5LifecycleData {
        route: input.route,
        tenant_id: input.tenant_id,
        credential_id: input.credential_id,
        key_id: input.key_id,
        key_version: input.key_version,
        record_kind: input.record_kind,
        schema_ref: input.schema_ref,
        repository_action: input.repository_action,
        lifecycle_status: input.lifecycle_status,
        rotation_record: input.rotation_record,
        revocation_record: input.revocation_record,
        cache_invalidation,
        propagation_status: input.propagation_status,
        affected_inventory,
        break_glass_accepted: input.break_glass_accepted,
        idempotency_key: input.idempotency_key,
        audit_refs: input.audit_refs,
        raw_secret_persisted: false,
        redacted_fields: safe_metadata_redactions(),
    }
}

fn cache_invalidation_for(
    tenant_id: &str,
    credential_id: &str,
    key_version: u32,
    command_classes: &[String],
    revocation_epoch: u64,
    reason_code: &str,
) -> CacheInvalidation {
    let command_scope = if command_classes.is_empty() {
        "command.any".to_owned()
    } else {
        command_classes.join(",")
    };
    let cache_key_ref = blake3_ref(
        "overkey-phase5-cache-key",
        &format!(
            "{tenant_id}|{credential_id}|{key_version}|{command_scope}|{CANONICALIZATION_VERSION}|{revocation_epoch}"
        ),
    );
    CacheInvalidation {
        invalidation_event_ref: format!(
            "event:overkey:cache_invalidation:{}",
            stable_trace_token(&cache_key_ref)
        ),
        cache_key_ref,
        revocation_epoch,
        max_positive_ttl_seconds: ORDINARY_POSITIVE_CACHE_TTL_SECONDS,
        high_risk_max_positive_ttl_seconds: HIGH_RISK_POSITIVE_CACHE_TTL_SECONDS,
        invalidation_reason: reason_code.to_owned(),
        invalidated_at: LOCAL_PHASE3_TIMESTAMP.to_owned(),
    }
}

fn default_propagation_status(action: &str, audit_ref: &str) -> Vec<PropagationStatus> {
    PHASE5_PROPAGATION_SERVICES
        .iter()
        .map(|service_id| PropagationStatus {
            service_id: (*service_id).to_owned(),
            propagation_state: if *service_id == "service:overgate" {
                "confirmed".to_owned()
            } else {
                "pending_confirmation".to_owned()
            },
            required_before_unblock: *service_id != "service:product-clients",
            last_checked_at: LOCAL_PHASE3_TIMESTAMP.to_owned(),
            audit_ref: format!("{audit_ref}:{action}:{}", service_id.replace(':', "-")),
        })
        .collect()
}

fn affected_inventory_for(
    tenant_id: &str,
    subject_ref: &str,
    credential_id: &str,
    command_classes: Vec<String>,
) -> AffectedInventory {
    AffectedInventory {
        tenant_id: tenant_id.to_owned(),
        subject_ref: subject_ref.to_owned(),
        credential_id: credential_id.to_owned(),
        command_classes,
        services: PHASE5_PROPAGATION_SERVICES
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        product_clients: vec![
            "client:sdk".to_owned(),
            "client:cli".to_owned(),
            "client:admin-ui".to_owned(),
        ],
        follow_up_tasks: vec![
            "operator:confirm-propagation".to_owned(),
            "operator:rotate-successor-or-reenroll".to_owned(),
        ],
    }
}

fn non_empty_refs(input: Option<Vec<String>>, fallback: String) -> Vec<String> {
    let refs = input.unwrap_or_default();
    if refs.iter().any(|value| !value.trim().is_empty()) {
        refs.into_iter()
            .filter(|value| !value.trim().is_empty())
            .collect()
    } else {
        vec![fallback]
    }
}

fn affected_command_classes(request: &LifecycleRequest, current: &CredentialRecord) -> Vec<String> {
    request
        .affected_command_classes
        .clone()
        .filter(|values| values.iter().any(|value| !value.trim().is_empty()))
        .unwrap_or_else(|| {
            if current.allowed_command_classes.is_empty() {
                vec!["command.verify".to_owned()]
            } else {
                current.allowed_command_classes.clone()
            }
        })
}

fn validate_delegation_request(
    headers: &HeaderMap,
    request: &DelegationRequest,
    trace_id: &str,
) -> Result<PolicyHandoff, OverkeyError> {
    let service_account = header_value(headers, SERVICE_ACCOUNT_HEADER).unwrap_or_default();
    if service_account != "service-account:overgate" {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.delegation_overgate_required",
            "delegation lifecycle commands must enter through Overgate",
            vec!["header:x-overrid-service-account"],
        ));
    }
    if request.delegator_ref.trim().is_empty()
        || request.delegate_ref.trim().is_empty()
        || request.delegator_ref == request.delegate_ref
    {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.delegation_subject_invalid",
            "delegation requires distinct delegator and delegate refs",
            vec!["delegation_record.delegator_ref", "delegation_record.delegate_ref"],
        ));
    }
    if !narrow_delegation_scope(&request.allowed_scopes)
        || !narrow_service_account_scope(&request.allowed_command_classes)
        || !request.allowed_scopes.iter().all(|value| {
            PHASE6_DELEGATION_ALLOWED_SCOPES
                .iter()
                .any(|allowed| value == allowed)
        })
        || !request.allowed_command_classes.iter().all(|value| {
            SERVICE_ACCOUNT_ALLOWED_COMMAND_CLASSES
                .iter()
                .any(|allowed| value == allowed)
        })
    {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.delegation_scope_too_broad",
            "delegated access must use narrow scopes and command classes",
            vec![
                "delegation_record.allowed_scopes",
                "delegation_record.allowed_command_classes",
            ],
        ));
    }
    if request.not_after.trim().is_empty() {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.delegation_expiry_required",
            "delegated access requires expiry metadata",
            vec!["delegation_record.not_after"],
        ));
    }
    if dependency_state_denied(request.overpass_delegate_state.as_deref())
        || dependency_state_denied(request.overtenant_tenant_state.as_deref())
        || dependency_state_denied(request.overtenant_membership_state.as_deref())
    {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.delegation_delegate_stale",
            "delegated access is denied when delegate, tenant, or membership state is stale",
            vec![
                "delegation_record.delegate_ref",
                "overpass_delegate_state",
                "overtenant_membership_state",
            ],
        ));
    }
    let has_evidence = request
        .evidence_refs
        .as_ref()
        .map(|refs| refs.iter().any(|value| !value.trim().is_empty()))
        .unwrap_or(false);
    if !has_evidence {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.delegation_evidence_required",
            "delegated access requires evidence refs",
            vec!["delegation_record.evidence_refs"],
        ));
    }
    validate_overguard_policy_handoff(
        request.overguard_policy_decision_ref.as_deref(),
        request.overguard_decision.as_deref(),
        trace_id,
    )
}

fn validate_overguard_policy_handoff(
    decision_ref: Option<&str>,
    decision: Option<&str>,
    trace_id: &str,
) -> Result<PolicyHandoff, OverkeyError> {
    if decision_ref.unwrap_or("").trim().is_empty() {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "policy.overguard_handoff_required",
            "credential-class and delegation policy require an Overguard decision ref",
            vec!["policy_handoff.overguard_policy_decision_ref"],
        ));
    }
    if decision.unwrap_or("") != "allow" {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "policy.overguard_denied",
            "Overkey honors Overguard deny decisions without storing policy truth",
            vec!["policy_handoff.overguard_decision"],
        ));
    }
    Ok(policy_handoff_from_decision(
        decision_ref,
        decision,
        "command.delegation.create",
    ))
}

fn policy_handoff_from_decision(
    decision_ref: Option<&str>,
    decision: Option<&str>,
    command_class: &str,
) -> PolicyHandoff {
    let decision_ref = decision_ref
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("policy:overguard:phase6:local-low-risk");
    PolicyHandoff {
        policy_engine_ref: "service:overguard".to_owned(),
        decision_ref: decision_ref.to_owned(),
        decision: decision.unwrap_or("local_phase1_allow").to_owned(),
        local_fallback: if is_high_risk_command_class(command_class) {
            "deny_by_default_until_overguard_allow".to_owned()
        } else {
            "phase1_local_low_risk_checks".to_owned()
        },
        checked_dimensions: vec![
            "credential_class".to_owned(),
            "delegation".to_owned(),
            "protection_class".to_owned(),
            "command_class".to_owned(),
            "high_risk_operation".to_owned(),
        ],
        overkey_policy_truth_stored: false,
    }
}

fn local_policy_handoff(decision_ref: &str) -> PolicyHandoff {
    policy_handoff_from_decision(Some(decision_ref), Some("local_phase1_allow"), "command.verify")
}

fn narrow_delegation_scope(values: &[String]) -> bool {
    narrow_service_account_scope(values)
}

fn is_high_risk_command_class(command_class: &str) -> bool {
    command_class.contains("operator")
        || command_class.contains("admin")
        || command_class == "command.credential.rotate"
        || command_class == "command.credential.revoke"
        || command_class == "command.secret.resolve"
        || command_class == "command.system.operate"
        || command_class == "command.queue.execution_callback"
        || command_class == "command.worker.runtime_callback"
}

fn validate_rotation_successor(
    state: &OverkeyState,
    tenant_id: &str,
    credential_id: &str,
    successor_credential_id: &str,
    successor_key_id: &str,
    successor_key_version: u32,
    current: &CredentialRecord,
    trace_id: &str,
) -> Result<(), OverkeyError> {
    let invalid_successor = |field_refs| {
        OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "overkey.rotation_successor_invalid",
            "rotation requires a distinct active successor credential in the same tenant scope",
            field_refs,
        )
    };

    if successor_credential_id.trim().is_empty()
        || successor_key_id.trim().is_empty()
        || successor_credential_id == credential_id
        || successor_key_version <= current.key_version
    {
        return Err(invalid_successor(vec![
            "rotation_record.successor_credential_id",
            "rotation_record.successor_key_id",
            "rotation_record.successor_key_version",
        ]));
    }

    let Some(successor) = state
        .repository
        .credential(tenant_id, successor_credential_id)
    else {
        return Err(invalid_successor(vec![
            "rotation_record.successor_credential_id",
        ]));
    };

    if successor.status != CredentialStatus::Active
        || successor.key_id != successor_key_id
        || successor.key_version != successor_key_version
    {
        return Err(invalid_successor(vec![
            "rotation_record.successor_credential_id",
            "rotation_record.successor_key_id",
            "rotation_record.successor_key_version",
        ]));
    }

    Ok(())
}

fn validate_break_glass_request(
    headers: &HeaderMap,
    request: &LifecycleRequest,
    current: &CredentialRecord,
    trace_id: &str,
) -> Result<(), OverkeyError> {
    if !request.break_glass.unwrap_or(false) {
        return Ok(());
    }
    let service_account = header_value(headers, SERVICE_ACCOUNT_HEADER).unwrap_or_default();
    let service_signature = header_value(headers, SERVICE_SIGNATURE_HEADER).unwrap_or_default();
    if service_account != "service-account:overgate" || service_signature.trim().is_empty() {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.break_glass_unsigned",
            "break-glass revocation must enter through signed Overgate service account command",
            vec![
                "header:x-overrid-service-account",
                "header:x-overrid-service-signature",
            ],
        ));
    }
    if request
        .overgate_command_signature
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
    {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.break_glass_unsigned",
            "break-glass revocation requires a signed Overgate command envelope",
            vec!["revocation_record.overgate_command_signature"],
        ));
    }
    let role = request.operator_role.as_deref().unwrap_or("");
    if !matches!(
        role,
        "role:operator" | "role:admin" | "role:break_glass_admin"
    ) {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.break_glass_wrong_role",
            "break-glass revocation requires an operator or admin role",
            vec!["revocation_record.operator_role"],
        ));
    }
    let protection_class = request
        .protection_class
        .as_deref()
        .unwrap_or(current.protection_class.as_str());
    if !(protection_class.contains("break_glass")
        || protection_class.contains("hardware")
        || protection_class.contains("secure_enclave"))
    {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.break_glass_protection_class_required",
            "break-glass revocation requires a high-risk protection class",
            vec!["revocation_record.protection_class"],
        ));
    }
    let has_evidence = request
        .evidence_refs
        .as_ref()
        .map(|refs| refs.iter().any(|value| !value.trim().is_empty()))
        .unwrap_or(false);
    if !has_evidence {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.break_glass_missing_evidence",
            "break-glass revocation requires evidence refs",
            vec!["revocation_record.evidence_refs"],
        ));
    }
    if request
        .idempotency_key
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
    {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.break_glass_idempotency_required",
            "break-glass revocation requires an idempotency key",
            vec!["revocation_record.idempotency_key"],
        ));
    }
    Ok(())
}

fn validate_operator_lifecycle_request(
    headers: &HeaderMap,
    request: &LifecycleRequest,
    current: &CredentialRecord,
    trace_id: &str,
) -> Result<(), OverkeyError> {
    if !request.operator_lifecycle.unwrap_or(false) {
        return Ok(());
    }
    let service_account = header_value(headers, SERVICE_ACCOUNT_HEADER).unwrap_or_default();
    let service_signature = header_value(headers, SERVICE_SIGNATURE_HEADER).unwrap_or_default();
    if service_account != "service-account:overgate" || service_signature.trim().is_empty() {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.operator_lifecycle_unsigned",
            "high-risk operator lifecycle commands must enter through signed Overgate service account commands",
            vec![
                "header:x-overrid-service-account",
                "header:x-overrid-service-signature",
            ],
        ));
    }
    if request
        .overgate_command_signature
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
    {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.operator_lifecycle_unsigned",
            "high-risk operator lifecycle commands require a signed Overgate command envelope",
            vec!["lifecycle_request.overgate_command_signature"],
        ));
    }
    let role = request.operator_role.as_deref().unwrap_or("");
    if !matches!(
        role,
        "role:operator" | "role:admin" | "role:break_glass_admin"
    ) {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.operator_lifecycle_role_required",
            "high-risk lifecycle commands require operator or admin role evidence",
            vec!["lifecycle_request.operator_role"],
        ));
    }
    let protection_class = request
        .protection_class
        .as_deref()
        .unwrap_or(current.protection_class.as_str());
    if !(protection_class.contains("operator")
        || protection_class.contains("hardware")
        || protection_class.contains("secure_enclave")
        || protection_class.contains("break_glass"))
    {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.operator_lifecycle_protection_class_required",
            "high-risk lifecycle commands require a stronger operator/admin protection class",
            vec!["lifecycle_request.protection_class"],
        ));
    }
    let has_evidence = request
        .evidence_refs
        .as_ref()
        .map(|refs| refs.iter().any(|value| !value.trim().is_empty()))
        .unwrap_or(false);
    if !has_evidence || request.audit_ref.as_deref().unwrap_or("").trim().is_empty() {
        return Err(OverkeyError::invalid_enrollment(
            trace_id.to_owned(),
            "auth.operator_lifecycle_evidence_required",
            "high-risk lifecycle commands require Overwatch audit and evidence refs",
            vec!["lifecycle_request.audit_ref", "lifecycle_request.evidence_refs"],
        ));
    }
    Ok(())
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
        && services.iter().all(|service| {
            command_classes
                .iter()
                .any(|command| service_supports_command(service, command))
        })
        && command_classes.iter().all(|command| {
            services
                .iter()
                .any(|service| service_supports_command(service, command))
        })
        && command_classes.iter().all(|command| {
            !matches!(
                command.as_str(),
                "command.accounting.mutate"
                    | "command.rights.mutate"
                    | "command.payout.mutate"
                    | "command.namespace.mutate"
                    | "command.policy.mutate"
            )
        })
}

fn service_supports_command(service: &str, command: &str) -> bool {
    match service {
        "service:overgate" => matches!(
            command,
            "command.verify"
                | "command.credential.read"
                | "command.credential.rotate"
                | "command.credential.revoke"
        ),
        "service:overvault" => command == "command.secret.resolve",
        "service:node-agent" => command == "command.node.enroll",
        "service:system-service" | "service:grid-resident" => command == "command.system.operate",
        "service:overqueue" => command == "command.queue.execution_callback",
        "service:worker" | "service:overrun" => {
            matches!(
                command,
                "command.worker.runtime_callback" | "command.workload.execute"
            )
        }
        _ => false,
    }
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

fn json_response_with_schema<T: Serialize>(
    schema_version: &'static str,
    trace_id: String,
    reason_code: &'static str,
    data: T,
) -> Json<ApiResponse<T>> {
    Json(ApiResponse::new_with_schema(
        schema_version,
        trace_id,
        "accepted",
        reason_code,
        data,
    ))
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
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use crate::repository::StatusTransitionRecord;
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
                    .body(Body::from(
                        json!({
                            "credential_id": "credential:api-key:missing",
                            "api_key_prefix": "ovk_missing",
                            "api_key_hash_ref": "hash:api_key:blake3:missing",
                            "timestamp": "2026-06-26T12:00:00Z",
                            "replay_window_id": "replay:phase4:required",
                            "allowed_use": "request.verify",
                            "command_class": "command.verify"
                        })
                        .to_string(),
                    ))
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
    async fn phase4_signature_verification_checks_metadata_and_dependencies() {
        let router = OverkeyService::default().router();
        let create = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/signing-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(TRACE_HEADER, "trace:overkey:phase4:signing-create")
                    .body(Body::from(
                        json!({
                            "credential_id": "credential:signing:phase4",
                            "subject_ref": "actor:overpass:phase4",
                            "key_id": "key:tenant:phase4-signer",
                            "public_key_ref": "public-key-ref:ed25519:phase4",
                            "allowed_signature_uses": ["signature.verify"],
                            "not_after": "2026-12-31T23:59:59Z",
                            "protection_class": "protection:tenant_bound_public_key"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create.status(), StatusCode::OK);

        let signature_body_payload = "phase4-signature-body";
        let signature_body_hash_ref = blake3_ref("body", signature_body_payload);
        let valid_request = json!({
            "credential_id": "credential:signing:phase4",
            "key_id": "key:tenant:phase4-signer",
            "key_version": 1,
            "algorithm": "Ed25519",
            "canonicalization": "overrid.canonical_json.v0",
            "timestamp": "2026-06-26T12:00:00Z",
            "replay_window_id": "replay:phase4:signature",
            "body_hash_ref": signature_body_hash_ref,
            "body_hash_payload": signature_body_payload,
            "allowed_use": "signature.verify",
            "command_class": "command.verify",
            "tenant_id": "tenant:phase4",
            "subject_ref": "actor:overpass:phase4",
            "signature_ref": "signature:fixture:phase4",
            "revocation_epoch": 0,
            "overpass_subject_state": "active",
            "overtenant_tenant_state": "active",
            "overtenant_membership_state": "active"
        });
        let verified = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/signature")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase4")
                    .header(TRACE_HEADER, "trace:overkey:phase4:signature")
                    .body(Body::from(valid_request.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(verified.status(), StatusCode::OK);
        let verified_body = response_json(verified).await;
        assert_eq!(
            verified_body["schema_version"],
            "overkey.phase4.response.v0"
        );
        assert_eq!(verified_body["data"]["verified"], true);
        assert_eq!(
            verified_body["data"]["reason_code"],
            "auth.signature_verified_phase4"
        );
        assert_eq!(verified_body["data"]["cache_guidance"]["cacheable"], true);

        let mut timestamp_changed_request = valid_request.clone();
        timestamp_changed_request["timestamp"] = json!("2026-06-26T12:00:01Z");
        let timestamp_changed = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/signature")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase4")
                    .body(Body::from(timestamp_changed_request.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let timestamp_changed_body = response_json(timestamp_changed).await;
        assert_eq!(timestamp_changed_body["data"]["verified"], true);
        assert_ne!(
            verified_body["data"]["request_hash_ref"],
            timestamp_changed_body["data"]["request_hash_ref"]
        );

        let mut replay_changed_request = valid_request.clone();
        replay_changed_request["replay_window_id"] = json!("replay:phase4:signature:changed");
        let replay_changed = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/signature")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase4")
                    .body(Body::from(replay_changed_request.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let replay_changed_body = response_json(replay_changed).await;
        assert_eq!(replay_changed_body["data"]["verified"], true);
        assert_ne!(
            verified_body["data"]["request_hash_ref"],
            replay_changed_body["data"]["request_hash_ref"]
        );

        let mut body_mismatch_request = valid_request.clone();
        body_mismatch_request["body_hash_payload"] = json!("phase4-tampered-body");
        let body_mismatch = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/signature")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase4")
                    .body(Body::from(body_mismatch_request.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let body_mismatch_body = response_json(body_mismatch).await;
        assert_eq!(body_mismatch_body["data"]["verified"], false);
        assert_eq!(
            body_mismatch_body["data"]["reason_code"],
            "auth.body_hash_mismatch"
        );

        let mut denied_request = valid_request.clone();
        denied_request["command_class"] = json!("command.secret.resolve");
        let denied = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/signature")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase4")
                    .body(Body::from(denied_request.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(denied.status(), StatusCode::OK);
        let denied_body = response_json(denied).await;
        assert_eq!(denied_body["data"]["verified"], false);
        assert_eq!(
            denied_body["data"]["reason_code"],
            "auth.command_class_denied"
        );
    }

    #[tokio::test]
    async fn phase4_api_key_verification_never_returns_raw_key_material() {
        let router = OverkeyService::default().router();
        let create = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/api-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(TRACE_HEADER, "trace:overkey:phase4:api-key-create")
                    .body(Body::from(
                        json!({
                            "credential_id": "credential:api-key:phase4",
                            "api_key_prefix": "ovk_phase4",
                            "raw_api_key": "phase4-local-api-key",
                            "allowed_uses": ["request.verify"]
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create.status(), StatusCode::OK);
        let created = response_json(create).await;
        let hash_ref = created["data"]["api_key_hash_ref"].as_str().unwrap();

        let verified = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/api-key")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase4")
                    .body(Body::from(
                        json!({
                            "credential_id": "credential:api-key:phase4",
                            "api_key_prefix": "ovk_phase4",
                            "api_key_hash_ref": hash_ref,
                            "timestamp": "2026-06-26T12:00:00Z",
                            "replay_window_id": "replay:phase4:api-key",
                            "allowed_use": "request.verify",
                            "command_class": "command.verify",
                            "tenant_id": "tenant:phase4"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(verified.status(), StatusCode::OK);
        let verified_body = response_json(verified).await;
        assert_eq!(
            verified_body["schema_version"],
            "overkey.phase4.response.v0"
        );
        assert_eq!(verified_body["data"]["verified"], true);
        assert_eq!(verified_body["data"]["raw_secret_persisted"], false);
        assert!(verified_body["data"]["redacted_fields"]
            .as_array()
            .unwrap()
            .iter()
            .any(|field| field == "raw_api_key"));
        let serialized = serde_json::to_string(&verified_body).unwrap();
        assert!(!serialized.contains("phase4-local-api-key"));

        let denied = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/api-key")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase4")
                    .body(Body::from(
                        json!({
                            "credential_id": "credential:api-key:phase4",
                            "api_key_prefix": "ovk_phase4",
                            "raw_api_key": "wrong-local-api-key",
                            "timestamp": "2026-06-26T12:00:00Z",
                            "replay_window_id": "replay:phase4:api-key-denied",
                            "allowed_use": "request.verify",
                            "command_class": "command.verify"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let denied_body = response_json(denied).await;
        assert_eq!(denied_body["data"]["verified"], false);
        assert_eq!(
            denied_body["data"]["reason_code"],
            "auth.api_key_hash_mismatch"
        );
    }

    #[tokio::test]
    async fn phase4_signature_verification_covers_plan_denials() {
        let router = OverkeyService::default().router();
        create_phase4_signing_credential(router.clone(), "credential:signing:phase4-denials").await;

        let body_hash_payload = "phase4-denial-body";
        let body_hash_ref = blake3_ref("body", body_hash_payload);
        let valid_request = phase4_signature_request(
            "credential:signing:phase4-denials",
            "key:tenant:phase4-denials-signer",
            body_hash_ref.as_str(),
            body_hash_payload,
        );

        let mut wrong_tenant = valid_request.clone();
        wrong_tenant["tenant_id"] = json!("tenant:other");
        assert_phase4_denial(
            router.clone(),
            wrong_tenant,
            "service-account:overgate",
            "auth.tenant_header_mismatch",
            "denied",
        )
        .await;

        let mut wrong_key_version = valid_request.clone();
        wrong_key_version["key_version"] = json!(2);
        assert_phase4_denial(
            router.clone(),
            wrong_key_version,
            "service-account:overgate",
            "auth.key_version_mismatch",
            "denied",
        )
        .await;

        let mut expired = valid_request.clone();
        expired["timestamp"] = json!("2027-01-01T00:00:00Z");
        assert_phase4_denial(
            router.clone(),
            expired,
            "service-account:overgate",
            "auth.signature_expired",
            "denied",
        )
        .await;

        let mut replayed = valid_request.clone();
        replayed["replay_window_id"] = json!("");
        assert_phase4_denial(
            router.clone(),
            replayed,
            "service-account:overgate",
            "auth.replay_window_required",
            "denied",
        )
        .await;

        let mut unknown_credential = valid_request.clone();
        unknown_credential["credential_id"] = json!("credential:signing:missing");
        assert_phase4_denial(
            router.clone(),
            unknown_credential,
            "service-account:overgate",
            "auth.credential_unknown",
            "denied",
        )
        .await;

        let mut disabled_subject = valid_request.clone();
        disabled_subject["overpass_subject_state"] = json!("disabled");
        assert_phase4_denial(
            router.clone(),
            disabled_subject,
            "service-account:overgate",
            "auth.subject_dependency_denied",
            "blocked",
        )
        .await;

        assert_phase4_denial(
            router.clone(),
            valid_request.clone(),
            "service-account:unapproved",
            "auth.service_account_not_approved",
            "denied",
        )
        .await;

        let malformed = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/signature")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase4")
                    .body(Body::from("{not-json"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(malformed.status(), StatusCode::BAD_REQUEST);

        let revoked_router = OverkeyService::default().router();
        create_phase4_signing_credential(
            revoked_router.clone(),
            "credential:signing:phase4-revoked",
        )
        .await;
        lifecycle_transition(
            revoked_router.clone(),
            "/v1/credentials/credential:signing:phase4-revoked/revoke",
        )
        .await;
        assert_phase4_denial(
            revoked_router,
            phase4_signature_request(
                "credential:signing:phase4-revoked",
                "key:tenant:phase4-denials-signer",
                body_hash_ref.as_str(),
                body_hash_payload,
            ),
            "service-account:overgate",
            "auth.credential_not_active",
            "denied",
        )
        .await;

        let rotating_router = OverkeyService::default().router();
        create_phase4_signing_credential(
            rotating_router.clone(),
            "credential:signing:phase4-rotating",
        )
        .await;
        create_phase4_signing_credential_with_key(
            rotating_router.clone(),
            "credential:signing:phase4-rotating-successor",
            "key:tenant:phase4-denials-signer-v2",
            2,
        )
        .await;
        lifecycle_transition(
            rotating_router.clone(),
            "/v1/credentials/credential:signing:phase4-rotating/rotate",
        )
        .await;
        assert_phase4_denial(
            rotating_router,
            phase4_signature_request(
                "credential:signing:phase4-rotating",
                "key:tenant:phase4-denials-signer",
                body_hash_ref.as_str(),
                body_hash_payload,
            ),
            "service-account:overgate",
            "auth.credential_not_active",
            "denied",
        )
        .await;

        let suspended_service = OverkeyService::default();
        let suspended_router = suspended_service.router();
        create_phase4_signing_credential(
            suspended_router.clone(),
            "credential:signing:phase4-suspended",
        )
        .await;
        suspended_service
            .state()
            .repository
            .append_status_transition(StatusTransitionRecord {
                tenant_id: "tenant:phase4".to_owned(),
                credential_id: "credential:signing:phase4-suspended".to_owned(),
                from_status: CredentialStatus::Active,
                to_status: CredentialStatus::Suspended,
                reason_code: "overkey.phase4.test_suspension".to_owned(),
                audit_ref: "audit:overkey:phase4:test-suspension".to_owned(),
            })
            .unwrap();
        assert_phase4_denial(
            suspended_router,
            phase4_signature_request(
                "credential:signing:phase4-suspended",
                "key:tenant:phase4-denials-signer",
                body_hash_ref.as_str(),
                body_hash_payload,
            ),
            "service-account:overgate",
            "auth.credential_not_active",
            "denied",
        )
        .await;
    }

    #[tokio::test]
    async fn phase5_rotation_records_epoch_cache_and_propagation() {
        let router = OverkeyService::default().router();
        create_phase5_signing_credential(
            router.clone(),
            "credential:signing:phase5-rotation",
            "key:tenant:phase5-rotation",
        )
        .await;

        let invalid_successor = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/credential:signing:phase5-rotation/rotate")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase5")
                    .header(TRACE_HEADER, "trace:overkey:phase5:rotation-invalid")
                    .body(Body::from(
                        json!({
                            "successor_credential_id": "credential:signing:phase5-missing-successor",
                            "successor_key_id": "key:tenant:phase5-missing-successor",
                            "successor_key_version": 2,
                            "reason_code": "overkey.rotation_started_phase5",
                            "affected_command_classes": ["command.verify"],
                            "evidence_refs": ["evidence:overkey:phase5:rotation-invalid"],
                            "audit_ref": "audit:overkey:phase5:rotation-invalid"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(invalid_successor.status(), StatusCode::BAD_REQUEST);
        let invalid_successor_body = response_json(invalid_successor).await;
        assert_eq!(
            invalid_successor_body["reason_code"],
            "overkey.rotation_successor_invalid"
        );

        let still_active = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/v1/credentials/credential:signing:phase5-rotation")
                    .header(TENANT_HEADER, "tenant:phase5")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let still_active_body = response_json(still_active).await;
        assert_eq!(still_active_body["data"]["lifecycle_status"], "active");

        create_phase5_signing_credential_with_version(
            router.clone(),
            "credential:signing:phase5-rotation-successor",
            "key:tenant:phase5-rotation-v2",
            2,
        )
        .await;

        let rotated = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/credential:signing:phase5-rotation/rotate")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase5")
                    .header(TRACE_HEADER, "trace:overkey:phase5:rotation")
                    .body(Body::from(
                        json!({
                            "successor_credential_id": "credential:signing:phase5-rotation-successor",
                            "successor_key_id": "key:tenant:phase5-rotation-v2",
                            "successor_key_version": 2,
                            "grace_window_seconds": 120,
                            "initiated_by": "actor:operator:phase5",
                            "activation_at": "2026-06-26T00:02:00Z",
                            "reason_code": "overkey.rotation_started_phase5",
                            "affected_command_classes": ["command.verify", "command.operator.rotate"],
                            "evidence_refs": ["evidence:overkey:phase5:rotation"],
                            "audit_ref": "audit:overkey:phase5:rotation"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(rotated.status(), StatusCode::OK);
        let body = response_json(rotated).await;
        assert_eq!(body["schema_version"], "overkey.phase5.response.v0");
        assert_eq!(body["reason_code"], "overkey.rotation_started_phase5");
        assert_eq!(body["data"]["record_kind"], "rotation_record");
        assert_eq!(
            body["data"]["rotation_record"]["successor_key_id"],
            "key:tenant:phase5-rotation-v2"
        );
        assert_eq!(body["data"]["rotation_record"]["grace_window_seconds"], 120);
        assert_eq!(body["data"]["cache_invalidation"]["revocation_epoch"], 1);
        assert_eq!(
            body["data"]["cache_invalidation"]["max_positive_ttl_seconds"],
            30
        );
        assert_eq!(
            body["data"]["cache_invalidation"]["high_risk_max_positive_ttl_seconds"],
            5
        );
        assert!(body["data"]["propagation_status"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry["service_id"] == "service:overgate"
                && entry["propagation_state"] == "confirmed"));

        let lookup = router
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/v1/credentials/credential:signing:phase5-rotation")
                    .header(TENANT_HEADER, "tenant:phase5")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let lookup_body = response_json(lookup).await;
        assert_eq!(lookup_body["data"]["lifecycle_status"], "rotating");
        assert!(lookup_body["data"]["rotation_refs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|entry| entry.as_str().unwrap().contains("rotation:")));
    }

    #[tokio::test]
    async fn phase5_break_glass_revocation_requires_signed_idempotent_command() {
        let router = OverkeyService::default().router();
        create_phase5_signing_credential(
            router.clone(),
            "credential:signing:phase5-break-glass",
            "key:tenant:phase5-break-glass",
        )
        .await;

        let cross_tenant = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/credential:signing:phase5-break-glass/revoke")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase5-other")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase5")
                    .body(Body::from(
                        json!({
                            "break_glass": true,
                            "operator_role": "role:admin",
                            "protection_class": "protection:break_glass_hardware_key",
                            "overgate_command_signature": "signature:overgate:phase5",
                            "idempotency_key": "idem:phase5:bg-cross-tenant",
                            "evidence_refs": ["evidence:phase5:bg"]
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(cross_tenant.status(), StatusCode::NOT_FOUND);
        let cross_tenant_body = response_json(cross_tenant).await;
        assert_eq!(
            cross_tenant_body["reason_code"],
            "overkey.credential_not_found"
        );

        let unsigned = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/credential:signing:phase5-break-glass/revoke")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase5")
                    .body(Body::from(
                        json!({
                            "break_glass": true,
                            "operator_role": "role:admin",
                            "protection_class": "protection:break_glass_hardware_key",
                            "overgate_command_signature": "signature:overgate:phase5",
                            "idempotency_key": "idem:phase5:bg",
                            "evidence_refs": ["evidence:phase5:bg"]
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(unsigned.status(), StatusCode::BAD_REQUEST);
        let unsigned_body = response_json(unsigned).await;
        assert_eq!(unsigned_body["reason_code"], "auth.break_glass_unsigned");

        let wrong_role = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/credential:signing:phase5-break-glass/revoke")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase5")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase5")
                    .body(Body::from(
                        json!({
                            "break_glass": true,
                            "operator_role": "role:viewer",
                            "protection_class": "protection:break_glass_hardware_key",
                            "overgate_command_signature": "signature:overgate:phase5",
                            "idempotency_key": "idem:phase5:bg",
                            "evidence_refs": ["evidence:phase5:bg"]
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let wrong_role_body = response_json(wrong_role).await;
        assert_eq!(
            wrong_role_body["reason_code"],
            "auth.break_glass_wrong_role"
        );

        let weak_protection = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/credential:signing:phase5-break-glass/revoke")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase5")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase5")
                    .body(Body::from(
                        json!({
                            "break_glass": true,
                            "operator_role": "role:admin",
                            "protection_class": "protection:tenant_bound_public_key",
                            "overgate_command_signature": "signature:overgate:phase5",
                            "idempotency_key": "idem:phase5:bg",
                            "evidence_refs": ["evidence:phase5:bg"]
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let weak_protection_body = response_json(weak_protection).await;
        assert_eq!(
            weak_protection_body["reason_code"],
            "auth.break_glass_protection_class_required"
        );

        let valid_body = json!({
            "break_glass": true,
            "operator_role": "role:admin",
            "protection_class": "protection:break_glass_hardware_key",
            "overgate_command_signature": "signature:overgate:phase5",
            "idempotency_key": "idem:phase5:bg",
            "revoked_by": "actor:operator:phase5",
            "effective_at": "2026-06-26T00:00:00Z",
            "reason_code": "overkey.break_glass_revocation",
            "affected_command_classes": ["command.verify", "command.operator.revoke"],
            "incident_refs": ["incident:phase5:bg"],
            "evidence_refs": ["evidence:phase5:bg"],
            "expected_current_status": "active",
            "audit_ref": "audit:overkey:phase5:break-glass"
        });
        let accepted = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/credential:signing:phase5-break-glass/revoke")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase5")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase5")
                    .header(TRACE_HEADER, "trace:overkey:phase5:break-glass")
                    .body(Body::from(valid_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(accepted.status(), StatusCode::OK);
        let accepted_body = response_json(accepted).await;
        assert_eq!(
            accepted_body["reason_code"],
            "overkey.break_glass_revocation_accepted_phase5"
        );
        assert_eq!(accepted_body["data"]["break_glass_accepted"], true);
        assert_eq!(
            accepted_body["data"]["revocation_record"]["break_glass"],
            true
        );
        assert_eq!(
            accepted_body["data"]["revocation_record"]["idempotency_key"],
            "idem:phase5:bg"
        );
        assert_eq!(
            accepted_body["data"]["cache_invalidation"]["revocation_epoch"],
            1
        );

        let replay = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/credential:signing:phase5-break-glass/revoke")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase5")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase5")
                    .body(Body::from(valid_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(replay.status(), StatusCode::OK);
        let replay_body = response_json(replay).await;
        assert_eq!(
            replay_body["reason_code"],
            "overkey.break_glass_revocation_idempotent_replay"
        );
        assert_eq!(
            replay_body["data"]["repository_action"],
            "idempotent_replay"
        );

        let denied_after_revoke = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/signature")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase5")
                    .header(SERVICE_ACCOUNT_HEADER, "service-account:overgate")
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase5")
                    .body(Body::from(
                        json!({
                            "credential_id": "credential:signing:phase5-break-glass",
                            "key_id": "key:tenant:phase5-break-glass",
                            "key_version": 1,
                            "algorithm": "Ed25519",
                            "canonicalization": "overrid.canonical_json.v0",
                            "timestamp": "2026-06-26T12:00:00Z",
                            "replay_window_id": "replay:phase5:bg",
                            "body_hash_ref": blake3_ref("body", "phase5-bg-body"),
                            "body_hash_payload": "phase5-bg-body",
                            "allowed_use": "signature.verify",
                            "command_class": "command.verify",
                            "tenant_id": "tenant:phase5",
                            "subject_ref": "actor:overpass:phase5",
                            "signature_ref": "signature:fixture:phase5",
                            "revocation_epoch": 1,
                            "overpass_subject_state": "active",
                            "overtenant_tenant_state": "active",
                            "overtenant_membership_state": "active"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let denied_after_revoke_body = response_json(denied_after_revoke).await;
        assert_eq!(
            denied_after_revoke_body["data"]["reason_code"],
            "auth.credential_not_active"
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

    async fn create_phase4_signing_credential(router: Router, credential_id: &str) {
        create_phase4_signing_credential_with_key(
            router,
            credential_id,
            "key:tenant:phase4-denials-signer",
            1,
        )
        .await;
    }

    async fn create_phase4_signing_credential_with_key(
        router: Router,
        credential_id: &str,
        key_id: &str,
        key_version: u32,
    ) {
        let create = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/signing-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(TRACE_HEADER, "trace:overkey:phase4:signing-denial-create")
                    .body(Body::from(
                        json!({
                            "credential_id": credential_id,
                            "subject_ref": "actor:overpass:phase4",
                            "key_id": key_id,
                            "key_version": key_version,
                            "public_key_ref": "public-key-ref:ed25519:phase4-denials",
                            "allowed_signature_uses": ["signature.verify"],
                            "not_after": "2026-12-31T23:59:59Z",
                            "protection_class": "protection:tenant_bound_public_key"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create.status(), StatusCode::OK);
    }

    async fn create_phase5_signing_credential(router: Router, credential_id: &str, key_id: &str) {
        create_phase5_signing_credential_with_version(router, credential_id, key_id, 1).await;
    }

    async fn create_phase5_signing_credential_with_version(
        router: Router,
        credential_id: &str,
        key_id: &str,
        key_version: u32,
    ) {
        let create = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/credentials/signing-keys")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase5")
                    .header(TRACE_HEADER, "trace:overkey:phase5:signing-create")
                    .body(Body::from(
                        json!({
                            "credential_id": credential_id,
                            "subject_ref": "actor:overpass:phase5",
                            "key_id": key_id,
                            "key_version": key_version,
                            "public_key_ref": "public-key-ref:ed25519:phase5",
                            "allowed_signature_uses": ["signature.verify"],
                            "not_after": "2026-12-31T23:59:59Z",
                            "protection_class": "protection:tenant_bound_public_key"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create.status(), StatusCode::OK);
    }

    fn phase4_signature_request(
        credential_id: &str,
        key_id: &str,
        body_hash_ref: &str,
        body_hash_payload: &str,
    ) -> Value {
        json!({
            "credential_id": credential_id,
            "key_id": key_id,
            "key_version": 1,
            "algorithm": "Ed25519",
            "canonicalization": "overrid.canonical_json.v0",
            "timestamp": "2026-06-26T12:00:00Z",
            "replay_window_id": "replay:phase4:denial",
            "body_hash_ref": body_hash_ref,
            "body_hash_payload": body_hash_payload,
            "allowed_use": "signature.verify",
            "command_class": "command.verify",
            "tenant_id": "tenant:phase4",
            "subject_ref": "actor:overpass:phase4",
            "signature_ref": "signature:fixture:phase4-denial",
            "revocation_epoch": 0,
            "overpass_subject_state": "active",
            "overtenant_tenant_state": "active",
            "overtenant_membership_state": "active"
        })
    }

    async fn assert_phase4_denial(
        router: Router,
        request: Value,
        service_account_ref: &str,
        expected_reason_code: &str,
        expected_state: &str,
    ) {
        let response = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/verify/signature")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .header(SERVICE_ACCOUNT_HEADER, service_account_ref)
                    .header(SERVICE_SIGNATURE_HEADER, "signature:service-account:phase4")
                    .body(Body::from(request.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response_json(response).await;
        assert_eq!(body["data"]["verified"], false);
        assert_eq!(body["data"]["reason_code"], expected_reason_code);
        assert_eq!(body["data"]["verification_state"], expected_state);
    }

    async fn lifecycle_transition(router: Router, uri: &str) {
        let body = if uri.ends_with("/rotate") {
            json!({
                "successor_credential_id": "credential:signing:phase4-rotating-successor",
                "successor_key_id": "key:tenant:phase4-denials-signer-v2",
                "successor_key_version": 2,
                "reason_code": "overkey.phase4.test_transition",
                "audit_ref": "audit:overkey:phase4:test-transition"
            })
        } else {
            json!({
                "reason_code": "overkey.phase4.test_transition",
                "audit_ref": "audit:overkey:phase4:test-transition"
            })
        };
        let response = router
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(uri)
                    .header(CONTENT_TYPE, "application/json")
                    .header(TENANT_HEADER, "tenant:phase4")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    async fn response_json(response: axum::response::Response) -> Value {
        let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }
}
