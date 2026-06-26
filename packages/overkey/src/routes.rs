use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;

use crate::errors::OverkeyError;
use crate::records::{CredentialRecord, CredentialStatus, SecretRef, VerificationResult};
use crate::repository::{CredentialMetadataRepository, StatusTransitionRecord};
use crate::schema::{
    API_KEY_RECORD_SCHEMA_REF, CREDENTIAL_RECORD_SCHEMA_REF,
    OVERKEY_PHASE2_RESPONSE_SCHEMA_VERSION, PUBLIC_KEY_RECORD_SCHEMA_REF,
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
            schema_version: OVERKEY_PHASE2_RESPONSE_SCHEMA_VERSION,
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
    record_kind: &'static str,
    schema_ref: &'static str,
    repository_action: &'static str,
    storage_boundary: &'static str,
    protection_class: &'static str,
    allowed_uses: Vec<&'static str>,
    audit_refs: Vec<String>,
    overgate_admission_required: bool,
    overwatch_event_ref: String,
    overvault_secret_ref: String,
    raw_secret_persisted: bool,
}

#[derive(Debug, Serialize)]
struct CredentialLookupData {
    route: &'static str,
    tenant_id: String,
    credential_id: String,
    credential_known: bool,
    schema_ref: &'static str,
    status: &'static str,
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
    usage_recorded: bool,
    overwatch_event_ref: String,
}

#[derive(Debug, Serialize)]
struct HealthData {
    route: &'static str,
    service_id: String,
    liveness: &'static str,
    readiness_claimed: bool,
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
    _body: Bytes,
) -> Result<Json<ApiResponse<CredentialRouteData>>, OverkeyError> {
    require_json(&headers)?;
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let credential_id = format!("credential:api-key:{}", stable_trace_token(&trace_id));
    append_placeholder_credential(
        &state,
        &tenant_id,
        &credential_id,
        "api_key",
        vec![
            "request.authenticate".to_owned(),
            "request.verify".to_owned(),
        ],
        "secret://overvault/local/overkey/api-key-ref",
    )
    .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id,
        "overkey.credential_registered",
        credential_data(
            ROUTE_CREATE_API_KEY,
            tenant_id,
            credential_id,
            "api_key_record",
            API_KEY_RECORD_SCHEMA_REF,
            vec!["request.authenticate", "request.verify"],
            "secret://overvault/local/overkey/api-key-ref",
        ),
    ))
}

async fn create_signing_key(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
    _body: Bytes,
) -> Result<Json<ApiResponse<CredentialRouteData>>, OverkeyError> {
    require_json(&headers)?;
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let credential_id = format!("credential:signing-key:{}", stable_trace_token(&trace_id));
    append_placeholder_credential(
        &state,
        &tenant_id,
        &credential_id,
        "public_signing_key",
        vec!["signature.verify".to_owned()],
        "secret://overvault/local/overkey/signing-key-ref",
    )
    .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id,
        "overkey.credential_registered",
        credential_data(
            ROUTE_CREATE_SIGNING_KEY,
            tenant_id,
            credential_id,
            "public_key_record",
            PUBLIC_KEY_RECORD_SCHEMA_REF,
            vec!["signature.verify"],
            "secret://overvault/local/overkey/signing-key-ref",
        ),
    ))
}

async fn create_service_account(
    State(state): State<OverkeyState>,
    headers: HeaderMap,
    _body: Bytes,
) -> Result<Json<ApiResponse<CredentialRouteData>>, OverkeyError> {
    require_json(&headers)?;
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let credential_id = format!(
        "credential:service-account:{}",
        stable_trace_token(&trace_id)
    );
    append_placeholder_credential(
        &state,
        &tenant_id,
        &credential_id,
        "service_account_key",
        vec![
            "service.authenticate".to_owned(),
            "signature.verify".to_owned(),
        ],
        "secret://overvault/local/overkey/service-account-key-ref",
    )
    .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id,
        "overkey.credential_registered",
        credential_data(
            ROUTE_CREATE_SERVICE_ACCOUNT,
            tenant_id,
            credential_id,
            "service_account_key",
            SERVICE_ACCOUNT_KEY_SCHEMA_REF,
            vec!["service.authenticate", "signature.verify"],
            "secret://overvault/local/overkey/service-account-key-ref",
        ),
    ))
}

async fn rotate_credential(
    State(state): State<OverkeyState>,
    Path(credential_id): Path<String>,
    headers: HeaderMap,
    _body: Bytes,
) -> Result<Json<ApiResponse<CredentialRouteData>>, OverkeyError> {
    require_json(&headers)?;
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    state
        .repository
        .append_status_transition(StatusTransitionRecord {
            tenant_id: tenant_id.clone(),
            credential_id: credential_id.clone(),
            from_status: CredentialStatus::Active,
            to_status: CredentialStatus::Rotating,
            reason_code: "overkey.rotation_requested".to_owned(),
            audit_ref: format!("audit:overkey:rotation:{}", stable_trace_token(&trace_id)),
        })
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id,
        "overkey.rotation_requested",
        credential_data(
            ROUTE_ROTATE_CREDENTIAL,
            tenant_id,
            credential_id,
            "rotation_record",
            ROTATION_RECORD_SCHEMA_REF,
            vec!["credential.rotate"],
            "secret://overvault/local/overkey/rotation-target-ref",
        ),
    ))
}

async fn revoke_credential(
    State(state): State<OverkeyState>,
    Path(credential_id): Path<String>,
    headers: HeaderMap,
    _body: Bytes,
) -> Result<Json<ApiResponse<CredentialRouteData>>, OverkeyError> {
    require_json(&headers)?;
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    state
        .repository
        .append_status_transition(StatusTransitionRecord {
            tenant_id: tenant_id.clone(),
            credential_id: credential_id.clone(),
            from_status: CredentialStatus::Active,
            to_status: CredentialStatus::Revoked,
            reason_code: "overkey.revocation_requested".to_owned(),
            audit_ref: format!("audit:overkey:revocation:{}", stable_trace_token(&trace_id)),
        })
        .map_err(|error| OverkeyError::repository_rejected(trace_id.clone(), error))?;

    Ok(json_response(
        trace_id,
        "overkey.revocation_requested",
        credential_data(
            ROUTE_REVOKE_CREDENTIAL,
            tenant_id,
            credential_id,
            "revocation_record",
            REVOCATION_RECORD_SCHEMA_REF,
            vec!["credential.revoke"],
            "secret://overvault/local/overkey/revocation-ref",
        ),
    ))
}

async fn get_credential(
    State(state): State<OverkeyState>,
    Path(credential_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<CredentialLookupData>>, OverkeyError> {
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let credential_known = state
        .repository
        .credential(&tenant_id, &credential_id)
        .is_some();

    Ok(json_response(
        trace_id,
        "overkey.credential_lookup",
        CredentialLookupData {
            route: ROUTE_GET_CREDENTIAL,
            tenant_id,
            credential_id,
            credential_known,
            schema_ref: CREDENTIAL_RECORD_SCHEMA_REF,
            status: if credential_known { "known" } else { "unknown" },
        },
    ))
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
    headers: HeaderMap,
    _body: Bytes,
) -> Result<Json<ApiResponse<UsageData>>, OverkeyError> {
    require_json(&headers)?;
    let trace_id = trace_id(&headers);
    let tenant_id = tenant_from_headers(&headers)?;
    let credential_id = format!("credential:last-used:{}", stable_trace_token(&trace_id));

    Ok(json_response(
        trace_id.clone(),
        "overkey.usage_recorded",
        UsageData {
            route: ROUTE_USAGE_LAST_USED,
            tenant_id,
            credential_id,
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

fn append_placeholder_credential(
    state: &OverkeyState,
    tenant_id: &str,
    credential_id: &str,
    credential_class: &str,
    allowed_uses: Vec<String>,
    secret_ref: &str,
) -> Result<(), crate::repository::RepositoryError> {
    state
        .repository
        .append_credential(CredentialRecord::local_fixture(
            tenant_id,
            "actor:overpass:phase2-fixture",
            credential_id,
            credential_class,
            allowed_uses,
            SecretRef::local_fixture(secret_ref),
        ))
}

fn credential_data(
    route: &'static str,
    tenant_id: String,
    credential_id: String,
    record_kind: &'static str,
    schema_ref: &'static str,
    allowed_uses: Vec<&'static str>,
    overvault_secret_ref: &'static str,
) -> CredentialRouteData {
    CredentialRouteData {
        route,
        tenant_id,
        credential_id,
        record_kind,
        schema_ref,
        repository_action: "append_metadata_record",
        storage_boundary: "append_only_local_overrid_stub",
        protection_class: "protection:tenant_bound_secret_ref",
        allowed_uses,
        audit_refs: vec!["audit:overkey:phase2:route".to_owned()],
        overgate_admission_required: true,
        overwatch_event_ref: "event:overwatch:credential_lifecycle:phase2".to_owned(),
        overvault_secret_ref: overvault_secret_ref.to_owned(),
        raw_secret_persisted: false,
    }
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
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response_json(response).await;
        assert_eq!(body["schema_version"], "overkey.phase2.response.v0");
        assert_eq!(body["service"], "service:overkey");
        assert_eq!(body["trace_id"], "trace:overkey:test");
        assert_eq!(body["data"]["route"], ROUTE_CREATE_API_KEY);
        assert_eq!(body["data"]["raw_secret_persisted"], false);
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
