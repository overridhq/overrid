use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use serde_json::{json, Value};

use crate::admission::{admit_command, AdmissionContext};
use crate::canonical::CanonicalRequestInput;
use crate::envelope::{trace_id_hint, CommandEnvelope};
use crate::errors::{ApiErrorData, OvergateError};
use crate::idempotency::{
    CommandLookup, IdempotencyLimitSummary, IdempotencyOutcome, IdempotencyRecord,
    IdempotencyReservationInput, TraceSummary,
};
use crate::retention::RetentionDecision;
use crate::schema::{validate_command_envelope, SchemaValidationReport};
use crate::service::OvergateState;

pub const TRACE_HEADER: &str = "x-overrid-trace-id";
pub const TENANT_HEADER: &str = "x-overrid-tenant-id";
pub const PHASE3_RESPONSE_SCHEMA_VERSION: &str = "overgate.phase3.response.v0";
pub const PHASE4_RESPONSE_SCHEMA_VERSION: &str = "overgate.phase4.response.v0";
pub const PHASE4_COMMAND_ADMITTED_REASON: &str = "overgate.command_admitted_phase4";
pub const PHASE4_ADMISSION_FORWARDING_STATE: &str = "not_forwarded_phase4_admission_only";
pub const PHASE5_RESPONSE_SCHEMA_VERSION: &str = "overgate.phase5.response.v0";
pub const PHASE3_VALIDATED_REASON: &str = "overgate.command_validated_phase3";
pub const PHASE3_FORWARDING_STATE: &str = "not_forwarded_phase3_validation_only";
// Phase 2 validator compatibility: schema_version: "overgate.phase2.response.v0"

pub const ROUTE_SUBMIT_COMMAND: &str = "POST /v1/commands";
pub const ROUTE_COMMAND_STATUS: &str = "GET /v1/commands/{command_id}";
pub const ROUTE_TRACE_STATUS: &str = "GET /v1/traces/{trace_id}";
pub const ROUTE_LIMITS: &str = "GET /v1/limits";
pub const ROUTE_POLICY_DRY_RUN: &str = "POST /v1/policy/dry-run";
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
            schema_version: PHASE5_RESPONSE_SCHEMA_VERSION,
            service: "service:overgate",
            trace_id: trace_id.into(),
            status,
            reason_code,
            data,
        }
    }
}

#[derive(Debug, Serialize)]
struct CommandAcceptedData {
    route: &'static str,
    request_id: String,
    command_id: String,
    audit_ref: String,
    forwarding_state: &'static str,
    payload_hash_ref: String,
    request_hash_ref: String,
    body_hash_ref: String,
    schema_validation: SchemaValidationReport,
    canonical_request: CanonicalRequestInput,
    admission: AdmissionContext,
    retention: RetentionDecision,
    idempotency: IdempotencyOutcome,
}

#[derive(Debug, Serialize)]
struct CommandStatusData {
    route: &'static str,
    command_id: String,
    admission_state: &'static str,
    forwarding_state: &'static str,
    owner: &'static str,
    status_visibility: &'static str,
    idempotency_record: Option<IdempotencyRecord>,
    audit_refs: Vec<String>,
    response_digest_ref: Option<String>,
    retention_class: Option<&'static str>,
}

#[derive(Debug, Serialize)]
struct TraceStatusData {
    route: &'static str,
    trace_id: String,
    audit_refs: Vec<String>,
    caller_visible: bool,
    trace_summary: TraceSummary,
}

#[derive(Debug, Serialize)]
struct LimitsData {
    route: &'static str,
    tenant_scope: String,
    rate_limit_refs: Vec<String>,
    quota_precheck_refs: Vec<String>,
    idempotency_summary: IdempotencyLimitSummary,
}

#[derive(Debug, Serialize)]
struct PolicyDryRunData {
    route: &'static str,
    policy_state: &'static str,
    mutation_allowed: bool,
    policy_ref: &'static str,
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

pub fn public_routes() -> Router<OvergateState> {
    Router::new()
        .route("/v1/commands", post(submit_command))
        .route("/v1/commands/:command_id", get(command_status))
        .route("/v1/traces/:trace_id", get(trace_status))
        .route("/v1/limits", get(limits))
        .route("/v1/policy/dry-run", post(policy_dry_run))
        .route("/v1/healthz", get(healthz))
        .route("/v1/readyz", get(readyz))
}

async fn submit_command(
    State(state): State<OvergateState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<
    (StatusCode, Json<ApiResponse<CommandAcceptedData>>),
    (StatusCode, Json<ApiResponse<ApiErrorData>>),
> {
    let fallback_trace = trace_id_hint(&headers, &body, "trace_overgate_phase3_parse_denied");
    let parsed = CommandEnvelope::parse_http(&headers, &body)
        .map_err(|error| api_error_response(fallback_trace.clone(), None, error))?;
    let canonical_request = CanonicalRequestInput::from_envelope(
        "POST",
        "/v1/commands",
        &parsed.envelope,
        &parsed.body_hash,
    );
    let request_id = command_request_id(&canonical_request, &parsed.envelope.trace_id);
    let admission = admit_command(&parsed.envelope, &canonical_request).map_err(|error| {
        api_error_response(
            parsed.envelope.trace_id.clone(),
            Some(request_id.clone()),
            error,
        )
    })?;
    let schema_validation = validate_command_envelope(&parsed.envelope).map_err(|error| {
        api_error_response(
            parsed.envelope.trace_id.clone(),
            Some(request_id.clone()),
            error,
        )
    })?;
    let retention = RetentionDecision::from_envelope(&parsed.envelope, &parsed.body_hash);
    let audit_ref = format!("audit:overgate:request_received:{request_id}");
    let idempotency = state
        .idempotency
        .reserve_or_replay(IdempotencyReservationInput::from_envelope(
            &parsed.envelope,
            &canonical_request,
            request_id.clone(),
            retention.clone(),
            audit_ref.clone(),
        ))
        .map_err(|error| {
            api_error_response(
                parsed.envelope.trace_id.clone(),
                Some(request_id.clone()),
                error,
            )
        })?;
    let (http_status, response_status, reason_code) = if idempotency.replayed {
        (StatusCode::OK, "ok", "overgate.idempotency_replayed")
    } else {
        (
            StatusCode::ACCEPTED,
            "accepted",
            "overgate.command_accepted_phase5",
        )
    };

    Ok((
        http_status,
        Json(ApiResponse::new(
            parsed.envelope.trace_id.clone(),
            response_status,
            reason_code,
            CommandAcceptedData {
                route: ROUTE_SUBMIT_COMMAND,
                request_id: request_id.clone(),
                command_id: parsed.envelope.command_id.clone(),
                audit_ref,
                forwarding_state: idempotency.record.forwarding_state,
                payload_hash_ref: parsed.envelope.payload_hash.clone(),
                request_hash_ref: parsed.envelope.request_hash.clone(),
                body_hash_ref: parsed.body_hash.clone(),
                schema_validation,
                canonical_request,
                admission,
                retention,
                idempotency,
            },
        )),
    ))
}

async fn command_status(
    State(state): State<OvergateState>,
    headers: HeaderMap,
    Path(command_id): Path<String>,
) -> Result<Json<ApiResponse<CommandStatusData>>, (StatusCode, Json<ApiResponse<ApiErrorData>>)> {
    let trace_id = trace_id(&headers, "trace_overgate_phase5_status");
    let tenant_scope =
        header_value(&headers, TENANT_HEADER).unwrap_or_else(|| "tenant:local:test".to_owned());
    match state.idempotency.lookup_command(&tenant_scope, &command_id) {
        CommandLookup::Found(record) => Ok(Json(ApiResponse::new(
            trace_id,
            "ok",
            "overgate.command_status_phase5",
            CommandStatusData {
                route: ROUTE_COMMAND_STATUS,
                command_id,
                admission_state: record.current_state,
                forwarding_state: record.forwarding_state,
                owner: record.owner,
                status_visibility: record.status_visibility,
                audit_refs: record.audit_refs.clone(),
                response_digest_ref: Some(record.response_digest_ref.clone()),
                retention_class: Some(record.retention.idempotency_retention_class),
                idempotency_record: Some(record),
            },
        ))),
        CommandLookup::Forbidden => Err(api_error_response(
            trace_id,
            None,
            OvergateError::status_visibility_denied(),
        )),
        CommandLookup::Missing => Err(api_error_response(
            trace_id,
            None,
            OvergateError::status_not_found(),
        )),
    }
}

async fn trace_status(
    State(state): State<OvergateState>,
    headers: HeaderMap,
    Path(trace_id_from_path): Path<String>,
) -> Json<ApiResponse<TraceStatusData>> {
    let trace_id = trace_id(&headers, &trace_id_from_path);
    let tenant_scope =
        header_value(&headers, TENANT_HEADER).unwrap_or_else(|| "tenant:local:test".to_owned());
    let trace_summary = state.idempotency.trace_summary(&tenant_scope, &trace_id);
    Json(ApiResponse::new(
        trace_id.clone(),
        "ok",
        "overgate.trace_summary_phase5",
        TraceStatusData {
            route: ROUTE_TRACE_STATUS,
            trace_id,
            audit_refs: trace_summary.audit_refs.clone(),
            caller_visible: trace_summary.caller_visible,
            trace_summary,
        },
    ))
}

async fn limits(
    headers: HeaderMap,
    State(_state): State<OvergateState>,
) -> Json<ApiResponse<LimitsData>> {
    let tenant_scope =
        header_value(&headers, TENANT_HEADER).unwrap_or_else(|| "tenant:local:test".to_owned());
    let idempotency_summary = _state.idempotency.limit_summary(&tenant_scope);
    Json(ApiResponse::new(
        trace_id(&headers, "trace_overgate_phase5_limits"),
        "ok",
        "overgate.limits_phase5",
        LimitsData {
            route: ROUTE_LIMITS,
            tenant_scope: tenant_scope.clone(),
            rate_limit_refs: vec![format!(
                "rate_limit:overgate:phase5:{}",
                stable_short_token(&[tenant_scope.as_str()])
            )],
            quota_precheck_refs: idempotency_summary.quota_precheck_refs.clone(),
            idempotency_summary,
        },
    ))
}

async fn policy_dry_run(
    headers: HeaderMap,
    Json(_payload): Json<Value>,
) -> (StatusCode, Json<ApiResponse<PolicyDryRunData>>) {
    (
        StatusCode::ACCEPTED,
        Json(ApiResponse::new(
            trace_id(&headers, "trace_overgate_phase2_policy_dry_run"),
            "accepted",
            "overgate.policy_dry_run_route_skeleton",
            PolicyDryRunData {
                route: ROUTE_POLICY_DRY_RUN,
                policy_state: "overguard_not_configured_phase2",
                mutation_allowed: false,
                policy_ref: "policy:dry_run:phase2_placeholder",
            },
        )),
    )
}

async fn healthz(
    State(state): State<OvergateState>,
    headers: HeaderMap,
) -> Json<ApiResponse<HealthData>> {
    Json(ApiResponse::new(
        trace_id(&headers, "trace_overgate_phase2_healthz"),
        "ok",
        "overgate.live",
        HealthData {
            route: ROUTE_HEALTHZ,
            service_id: state.config.service_id,
            liveness: "process_live",
            readiness_claimed: false,
        },
    ))
}

async fn readyz(
    State(state): State<OvergateState>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<ReadinessData>>) {
    let ready = state.dependencies.required_ready();
    let status = if ready {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (
        status,
        Json(ApiResponse::new(
            trace_id(&headers, "trace_overgate_phase2_readyz"),
            if ready { "ready" } else { "not_ready" },
            state.dependencies.readiness_reason_code(),
            ReadinessData {
                route: ROUTE_READYZ,
                service_id: state.config.service_id,
                ready,
                required_failures: state.dependencies.required_failures(),
                dependency_matrix: state.dependencies,
            },
        )),
    )
}

pub(crate) fn trace_id(headers: &HeaderMap, fallback: &str) -> String {
    header_value(headers, TRACE_HEADER).unwrap_or_else(|| fallback.to_owned())
}

pub(crate) fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.trim().to_owned())
}

pub(crate) fn stable_short_token(parts: &[&str]) -> String {
    crate::canonical::stable_short_token(parts)
}

fn command_request_id(canonical_request: &CanonicalRequestInput, trace_id: &str) -> String {
    format!(
        "request_{}",
        stable_short_token(&[canonical_request.canonical_hash.as_str(), trace_id])
    )
}

fn api_error_response(
    trace_id: String,
    request_id: Option<String>,
    error: OvergateError,
) -> (StatusCode, Json<ApiResponse<ApiErrorData>>) {
    let status = error.status;
    (
        status,
        Json(ApiResponse::new(
            trace_id,
            "denied",
            error.reason_code,
            error.to_data(request_id),
        )),
    )
}

pub fn documented_public_routes() -> Value {
    json!([
        ROUTE_SUBMIT_COMMAND,
        ROUTE_COMMAND_STATUS,
        ROUTE_TRACE_STATUS,
        ROUTE_LIMITS,
        ROUTE_POLICY_DRY_RUN,
        ROUTE_HEALTHZ,
        ROUTE_READYZ
    ])
}

#[cfg(test)]
mod tests {
    use axum::body::{to_bytes, Body};
    use axum::http::header::CONTENT_TYPE;
    use axum::http::{Request, StatusCode};
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use super::*;
    use crate::admin::{OPERATOR_ROLE_HEADER, OPERATOR_SIGNATURE_HEADER};
    use crate::dependencies::{DependencyMatrix, DependencyState};
    use crate::envelope::{MAX_COMMAND_ENVELOPE_BYTES, SUPPORTED_COMMAND_SCHEMA_VERSION};
    use crate::service::{OvergateConfig, OvergateService};

    async fn body_json(response: axum::response::Response) -> Value {
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");
        serde_json::from_slice(&bytes).expect("response body should be JSON")
    }

    fn empty_get(uri: &str) -> Request<Body> {
        Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())
            .expect("request should build")
    }

    fn valid_command_envelope(trace_id: &str) -> Value {
        json!({
            "command_id": "command:overgate:phase3:0001",
            "command_type": "overgate.phase3.noop",
            "tenant_id": "tenant:local:test",
            "actor_id": "actor:local:test",
            "trace_id": trace_id,
            "idempotency_key": "idem:overgate:phase3:0001",
            "credential_id": "credential:local:test",
            "schema_version": SUPPORTED_COMMAND_SCHEMA_VERSION,
            "payload_type": "application/vnd.overrid.command.noop+json",
            "request_hash": "hash:fixture:phase3_request",
            "payload_hash": "hash:fixture:phase3_payload",
            "timestamp": "2026-06-25T00:00:00Z",
            "signature_metadata": {
                "signature_ref": "signature:fixture:phase3",
                "algorithm": "ed25519",
                "key_version": "key_version:local:test",
                "canonicalization_version": "overgate.canonical.v0.1"
            },
            "privacy_class": "tenant_private",
            "payload_ref": "fixture://overgate/phase3/noop_payload"
        })
    }

    fn command_post(uri: &str, envelope: Value) -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri(uri)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(envelope.to_string()))
            .expect("request should build")
    }

    #[tokio::test]
    async fn public_routes_register_and_preserve_trace_json() {
        let app = OvergateService::default().router();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/commands")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TRACE_HEADER, "trace_test_command")
                    .body(Body::from(
                        valid_command_envelope("trace_test_command").to_string(),
                    ))
                    .expect("request should build"),
            )
            .await
            .expect("route should respond");

        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .expect("JSON response should set content type");
        assert!(content_type.starts_with("application/json"));

        let body = body_json(response).await;
        assert_eq!(body["service"], "service:overgate");
        assert_eq!(body["trace_id"], "trace_test_command");
        assert_eq!(body["schema_version"], PHASE5_RESPONSE_SCHEMA_VERSION);
        assert_eq!(body["reason_code"], "overgate.command_accepted_phase5");
        assert_eq!(body["data"]["route"], ROUTE_SUBMIT_COMMAND);
        assert_eq!(body["data"]["command_id"], "command:overgate:phase3:0001");
        assert_eq!(
            body["data"]["idempotency"]["reason_code"],
            "overgate.idempotency_reserved"
        );
        assert_eq!(
            body["data"]["admission"]["adapter_id"],
            "overgate.phase4.local_admission_adapter"
        );
        assert_eq!(
            body["data"]["admission"]["signature_check"]["reason_code"],
            "auth.signature_verified_phase4"
        );
        assert_eq!(
            body["data"]["admission"]["actor_resolution"]["reason_code"],
            "auth.actor_resolved_phase4"
        );
        assert_eq!(
            body["data"]["admission"]["tenant_authorization"]["reason_code"],
            "auth.tenant_authorized_phase4"
        );
        assert_eq!(
            body["data"]["schema_validation"]["adapter_id"],
            "overgate.phase3.shared_schema_adapter"
        );
        assert_eq!(
            body["data"]["canonical_request"]["canonicalization_version"],
            "overgate.canonical.v0.1"
        );
        assert_eq!(
            body["data"]["canonical_request"]["request_hash"],
            "hash:fixture:phase3_request"
        );
        assert_eq!(
            body["data"]["retention"]["body_retention"],
            "raw_body_not_retained"
        );
        assert_eq!(
            body["data"]["retention"]["idempotency_retention_class"],
            "low_risk_metadata_write"
        );

        let response = app
            .oneshot(empty_get("/v1/commands/command:overgate:phase3:0001"))
            .await
            .expect("status route should respond");
        assert_eq!(response.status(), StatusCode::OK);
        let body = body_json(response).await;
        assert_eq!(body["data"]["route"], ROUTE_COMMAND_STATUS);
        assert_eq!(body["reason_code"], "overgate.command_status_phase5");
        assert_eq!(body["data"]["owner"], "overgate_until_downstream_handoff");
        assert_eq!(
            body["data"]["response_digest_ref"]
                .as_str()
                .expect("response digest should be present")
                .starts_with("hash:overgate:"),
            true
        );
    }

    #[tokio::test]
    async fn local_base_path_routes_to_same_public_surface() {
        let app = OvergateService::default().router();
        let response = app
            .oneshot(empty_get("/overgate/v1/healthz"))
            .await
            .expect("nested local route should respond");
        assert_eq!(response.status(), StatusCode::OK);
        let body = body_json(response).await;
        assert_eq!(body["reason_code"], "overgate.live");
        assert_eq!(body["data"]["route"], ROUTE_HEALTHZ);
        assert_eq!(body["data"]["readiness_claimed"], false);
    }

    #[tokio::test]
    async fn local_fixture_command_smoke_submits_through_overgate_base_path() {
        let fixture: Value = serde_json::from_str(include_str!(
            "../fixtures/valid/phase2_local_command.valid.json"
        ))
        .expect("local command fixture should parse");
        let local_stack_service = &fixture["local_stack_service"];
        assert_eq!(local_stack_service["service_id"], "service:overgate");
        assert_eq!(local_stack_service["port_owner_service_id"], "service:api");
        assert_eq!(local_stack_service["base_path"], "/overgate");
        assert_eq!(local_stack_service["local_only"], true);
        assert_eq!(local_stack_service["test_only"], true);

        let envelope = fixture["command_envelope"].clone();
        let trace_id = envelope["trace_id"]
            .as_str()
            .expect("fixture command envelope should carry a trace id")
            .to_owned();

        let app = OvergateService::default().router();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/overgate/v1/commands")
                    .header(CONTENT_TYPE, "application/json")
                    .header(TRACE_HEADER, trace_id.as_str())
                    .body(Body::from(envelope.to_string()))
                    .expect("fixture command request should build"),
            )
            .await
            .expect("local fixture command smoke should respond");

        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let body = body_json(response).await;
        assert_eq!(body["service"], "service:overgate");
        assert_eq!(body["trace_id"].as_str(), Some(trace_id.as_str()));
        assert_eq!(body["reason_code"], "overgate.command_accepted_phase5");
        assert_eq!(body["data"]["route"], ROUTE_SUBMIT_COMMAND);
        assert_eq!(
            body["data"]["forwarding_state"],
            "pending_forwarding_phase5"
        );
        assert!(body["data"]["request_id"]
            .as_str()
            .expect("request id should be present")
            .starts_with("request_"));
        assert!(body["data"]["request_hash_ref"]
            .as_str()
            .expect("request hash ref should be present")
            .starts_with("hash:"));
        assert!(body["data"]["payload_hash_ref"]
            .as_str()
            .expect("payload hash ref should be present")
            .starts_with("hash:"));
        assert_eq!(
            body["data"]["retention"]["redaction"]["raw_secrets_redacted"],
            true
        );
    }

    #[tokio::test]
    async fn command_envelope_errors_are_stable_and_redacted() {
        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_missing_tenant");
        envelope
            .as_object_mut()
            .expect("fixture should be an object")
            .remove("tenant_id");
        let missing = app
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("missing-field request should respond");
        assert_eq!(missing.status(), StatusCode::BAD_REQUEST);
        let body = body_json(missing).await;
        assert_eq!(body["status"], "denied");
        assert_eq!(body["reason_code"], "schema.missing_required_field");
        assert_eq!(body["data"]["retryability"], "not_retryable");
        assert_eq!(body["data"]["diagnostics"]["redacted"], true);
        assert_eq!(
            body["data"]["diagnostics"]["privacy_class"],
            "redacted_diagnostic"
        );

        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_unknown_field");
        envelope
            .as_object_mut()
            .expect("fixture should be an object")
            .insert("private_payload".to_owned(), json!("do-not-store"));
        let unknown = app
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("unknown-field request should respond");
        assert_eq!(unknown.status(), StatusCode::BAD_REQUEST);
        let body = body_json(unknown).await;
        assert_eq!(body["reason_code"], "schema.unknown_sensitive_field");

        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_raw_secret");
        envelope
            .as_object_mut()
            .expect("fixture should be an object")
            .insert("payload_ref".to_owned(), json!("raw_secret_value"));
        let secret = app
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("secret request should respond");
        assert_eq!(secret.status(), StatusCode::BAD_REQUEST);
        let body = body_json(secret).await;
        assert_eq!(body["reason_code"], "schema.raw_secret_rejected");
        let body_text = body.to_string();
        assert!(!body_text.contains("raw_secret_value"));
    }

    #[tokio::test]
    async fn command_envelope_rejects_wrong_content_type_oversized_and_unsupported() {
        let app = OvergateService::default().router();
        let wrong_content_type = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/commands")
                    .body(Body::from(
                        valid_command_envelope("trace_wrong_type").to_string(),
                    ))
                    .expect("request should build"),
            )
            .await
            .expect("wrong-content-type request should respond");
        assert_eq!(
            wrong_content_type.status(),
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        );
        let body = body_json(wrong_content_type).await;
        assert_eq!(body["reason_code"], "schema.wrong_content_type");

        let app = OvergateService::default().router();
        let oversized = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/commands")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(" ".repeat(MAX_COMMAND_ENVELOPE_BYTES + 1)))
                    .expect("request should build"),
            )
            .await
            .expect("oversized request should respond");
        assert_eq!(oversized.status(), StatusCode::PAYLOAD_TOO_LARGE);
        let body = body_json(oversized).await;
        assert_eq!(body["reason_code"], "schema.payload_too_large");

        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_unsupported");
        envelope["schema_version"] = json!("shared-schema-package.v99.0");
        let unsupported = app
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("unsupported-version request should respond");
        assert_eq!(unsupported.status(), StatusCode::BAD_REQUEST);
        let body = body_json(unsupported).await;
        assert_eq!(body["reason_code"], "schema.unsupported_version");

        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_bad_canonicalization_version");
        envelope["signature_metadata"]["canonicalization_version"] =
            json!("overgate.canonical.v99.0");
        let unsupported_canonicalization = app
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("unsupported-canonicalization request should respond");
        assert_eq!(
            unsupported_canonicalization.status(),
            StatusCode::BAD_REQUEST
        );
        let body = body_json(unsupported_canonicalization).await;
        assert_eq!(body["reason_code"], "schema.unsupported_version");
        assert!(body["data"]["correction_fields"]
            .as_array()
            .expect("correction fields should be an array")
            .iter()
            .any(|value| value == "signature_metadata.canonicalization_version"));
    }

    #[tokio::test]
    async fn canonicalization_is_deterministic_and_input_sensitive() {
        let app = OvergateService::default().router();
        let first = app
            .oneshot(command_post(
                "/v1/commands",
                valid_command_envelope("trace_canonical"),
            ))
            .await
            .expect("first canonical request should respond");
        assert_eq!(first.status(), StatusCode::ACCEPTED);
        let first_body = body_json(first).await;

        let app = OvergateService::default().router();
        let second = app
            .oneshot(command_post(
                "/v1/commands",
                valid_command_envelope("trace_canonical"),
            ))
            .await
            .expect("second canonical request should respond");
        let second_body = body_json(second).await;
        assert_eq!(
            first_body["data"]["canonical_request"]["canonical_hash"],
            second_body["data"]["canonical_request"]["canonical_hash"]
        );
        assert_eq!(
            first_body["data"]["canonical_request"]["request_hash"],
            "hash:fixture:phase3_request"
        );

        let app = OvergateService::default().router();
        let mut changed = valid_command_envelope("trace_canonical");
        changed["idempotency_key"] = json!("idem:overgate:phase3:changed");
        let changed_response = app
            .oneshot(command_post("/v1/commands", changed))
            .await
            .expect("changed canonical request should respond");
        let changed_body = body_json(changed_response).await;
        assert_ne!(
            first_body["data"]["canonical_request"]["canonical_hash"],
            changed_body["data"]["canonical_request"]["canonical_hash"]
        );

        let app = OvergateService::default().router();
        let mut changed_request_hash = valid_command_envelope("trace_canonical");
        changed_request_hash["request_hash"] = json!("hash:fixture:phase3_request_changed");
        let changed_request_hash_response = app
            .oneshot(command_post("/v1/commands", changed_request_hash))
            .await
            .expect("request-hash changed canonical request should respond");
        let changed_request_hash_body = body_json(changed_request_hash_response).await;
        assert_ne!(
            first_body["data"]["canonical_request"]["canonical_hash"],
            changed_request_hash_body["data"]["canonical_request"]["canonical_hash"]
        );
        assert_eq!(
            changed_request_hash_body["data"]["canonical_request"]["request_hash"],
            "hash:fixture:phase3_request_changed"
        );
    }

    #[tokio::test]
    async fn phase5_idempotency_reserves_replays_and_conflicts_by_request_hash() {
        let app = OvergateService::default().router();
        let first = app
            .clone()
            .oneshot(command_post(
                "/v1/commands",
                valid_command_envelope("trace_phase5_replay_first"),
            ))
            .await
            .expect("first reservation should respond");
        assert_eq!(first.status(), StatusCode::ACCEPTED);
        let first_body = body_json(first).await;
        assert_eq!(
            first_body["reason_code"],
            "overgate.command_accepted_phase5"
        );
        assert_eq!(
            first_body["data"]["idempotency"]["outcome_state"],
            "reserved"
        );
        let first_digest = first_body["data"]["idempotency"]["record"]["response_digest_ref"]
            .as_str()
            .expect("first digest should be present")
            .to_owned();

        let replay = app
            .clone()
            .oneshot(command_post(
                "/v1/commands",
                valid_command_envelope("trace_phase5_replay_second"),
            ))
            .await
            .expect("replay request should respond");
        assert_eq!(replay.status(), StatusCode::OK);
        let replay_body = body_json(replay).await;
        assert_eq!(replay_body["reason_code"], "overgate.idempotency_replayed");
        assert_eq!(replay_body["data"]["idempotency"]["replayed"], true);
        assert_eq!(
            replay_body["data"]["idempotency"]["replay_metadata"]["first_trace_id"],
            "trace_phase5_replay_first"
        );
        assert_eq!(
            replay_body["data"]["idempotency"]["record"]["response_digest_ref"],
            first_digest
        );
        assert_eq!(
            replay_body["data"]["idempotency"]["replay_metadata"]["private_payload_disclosed"],
            false
        );

        let mut conflict = valid_command_envelope("trace_phase5_conflict");
        conflict["request_hash"] = json!("hash:fixture:phase5_conflict_request");
        let conflict_response = app
            .oneshot(command_post("/v1/commands", conflict))
            .await
            .expect("conflict request should respond");
        assert_eq!(conflict_response.status(), StatusCode::CONFLICT);
        let conflict_body = body_json(conflict_response).await;
        assert_eq!(
            conflict_body["reason_code"],
            "overgate.idempotency_conflict"
        );
    }

    #[tokio::test]
    async fn phase5_status_trace_and_limits_are_tenant_filtered() {
        let app = OvergateService::default().router();
        let response = app
            .clone()
            .oneshot(command_post(
                "/v1/commands",
                valid_command_envelope("trace_phase5_status"),
            ))
            .await
            .expect("status seed command should respond");
        assert_eq!(response.status(), StatusCode::ACCEPTED);

        let status = app
            .clone()
            .oneshot(empty_get("/v1/commands/command:overgate:phase3:0001"))
            .await
            .expect("status route should respond");
        assert_eq!(status.status(), StatusCode::OK);
        let body = body_json(status).await;
        assert_eq!(body["reason_code"], "overgate.command_status_phase5");
        assert_eq!(body["data"]["admission_state"], "pending_forwarding");
        assert_eq!(body["data"]["retention_class"], "low_risk_metadata_write");

        let trace = app
            .clone()
            .oneshot(empty_get("/v1/traces/trace_phase5_status"))
            .await
            .expect("trace route should respond");
        assert_eq!(trace.status(), StatusCode::OK);
        let body = body_json(trace).await;
        assert_eq!(body["reason_code"], "overgate.trace_summary_phase5");
        assert_eq!(body["data"]["trace_summary"]["command_count"], 1);
        assert_eq!(body["data"]["caller_visible"], true);

        let limits = app
            .clone()
            .oneshot(empty_get("/v1/limits"))
            .await
            .expect("limits route should respond");
        assert_eq!(limits.status(), StatusCode::OK);
        let body = body_json(limits).await;
        assert_eq!(body["reason_code"], "overgate.limits_phase5");
        assert_eq!(
            body["data"]["idempotency_summary"]["visible_record_count"],
            1
        );

        let cross_tenant = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/v1/commands/command:overgate:phase3:0001")
                    .header(TENANT_HEADER, "tenant:other")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("cross-tenant status route should respond");
        assert_eq!(cross_tenant.status(), StatusCode::FORBIDDEN);
        let body = body_json(cross_tenant).await;
        assert_eq!(body["reason_code"], "overgate.status_visibility_denied");

        let missing = app
            .oneshot(empty_get("/v1/commands/command:overgate:missing"))
            .await
            .expect("missing status route should respond");
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);
        let body = body_json(missing).await;
        assert_eq!(body["reason_code"], "overgate.status_not_found");
    }

    #[tokio::test]
    async fn phase5_retention_classes_cover_control_queue_finality_and_extension_refs() {
        let cases = [
            (
                "overgate.phase5.tenant.update",
                "control_plane_mutation",
                7 * 24 * 60 * 60,
            ),
            (
                "overgate.phase5.queue.workload.submit",
                "queue_producing_workload_command",
                7 * 24 * 60 * 60,
            ),
            (
                "overgate.phase5.ledger.finality.dispute",
                "finality_or_rights_command",
                90 * 24 * 60 * 60,
            ),
        ];

        for (command_type, retention_class, minimum_seconds) in cases {
            let app = OvergateService::default().router();
            let mut envelope = valid_command_envelope(&format!("trace_{retention_class}"));
            envelope["command_type"] = json!(command_type);
            envelope["idempotency_key"] = json!(format!("idem:{retention_class}"));
            envelope["request_hash"] = json!(format!("hash:fixture:{retention_class}:request"));
            envelope["payload_hash"] = json!(format!("hash:fixture:{retention_class}:payload"));
            let response = app
                .oneshot(command_post("/v1/commands", envelope))
                .await
                .expect("retention-class request should respond");
            assert_eq!(response.status(), StatusCode::ACCEPTED);
            let body = body_json(response).await;
            assert_eq!(
                body["data"]["retention"]["idempotency_retention_class"],
                retention_class
            );
            assert_eq!(
                body["data"]["retention"]["minimum_retention_seconds"],
                minimum_seconds
            );
            if command_type.contains("dispute") {
                assert!(body["data"]["retention"]["retention_extension_refs"]
                    .as_array()
                    .expect("extension refs should be an array")
                    .iter()
                    .any(|value| value == "retention_extension:dispute_ref"));
            }
        }
    }

    #[tokio::test]
    async fn phase4_command_admission_records_signature_actor_tenant_and_service_accounts() {
        let app = OvergateService::default().router();
        let mut service_account = valid_command_envelope("trace_phase4_service_account");
        service_account["command_id"] = json!("command:overgate:phase4:service_account");
        service_account["command_type"] = json!("overgate.phase4.service_account.noop");
        service_account["actor_id"] = json!("service_account:local:ingress");
        service_account["credential_id"] = json!("credential:service_account:local:ingress");
        service_account["signature_metadata"]["signature_ref"] =
            json!("signature:fixture:phase4_service_account");
        service_account["request_hash"] = json!("hash:fixture:phase4_service_account_request");
        service_account["payload_hash"] = json!("hash:fixture:phase4_service_account_payload");

        let response = app
            .oneshot(command_post("/v1/commands", service_account))
            .await
            .expect("service account admission should respond");
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let body = body_json(response).await;
        assert_eq!(body["reason_code"], "overgate.command_accepted_phase5");
        assert_eq!(
            body["data"]["admission"]["service_account_admission"]["reason_code"],
            "auth.service_account_admitted_phase4"
        );
        assert_eq!(
            body["data"]["admission"]["service_account_admission"]["narrow_permission_state"],
            "narrow_command_class_allowed"
        );
        assert_eq!(
            body["data"]["admission"]["signature_check"]["public_key_ref"]
                .as_str()
                .expect("public key ref should be present")
                .starts_with("public_key:overkey:"),
            true
        );

        let app = OvergateService::default().router();
        let mut node_agent = valid_command_envelope("trace_phase4_node_agent");
        node_agent["command_id"] = json!("command:overgate:phase4:node_agent");
        node_agent["command_type"] = json!("overgate.phase4.node_agent.callback");
        node_agent["actor_id"] = json!("node_agent:local:worker-1");
        node_agent["credential_id"] = json!("credential:node_agent:local:worker-1");
        node_agent["signature_metadata"]["signature_ref"] =
            json!("signature:fixture:phase4_node_agent");
        node_agent["request_hash"] = json!("hash:fixture:phase4_node_agent_request");
        node_agent["payload_hash"] = json!("hash:fixture:phase4_node_agent_payload");
        let response = app
            .oneshot(command_post("/v1/commands", node_agent))
            .await
            .expect("node-agent admission should respond");
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let body = body_json(response).await;
        assert_eq!(
            body["data"]["admission"]["service_account_admission"]["reason_code"],
            "auth.node_agent_admitted_phase4"
        );
    }

    #[tokio::test]
    async fn phase4_command_admission_denies_credential_actor_tenant_and_service_failures() {
        let cases = [
            (
                "credential_id",
                json!("credential:unknown:local"),
                StatusCode::UNAUTHORIZED,
                "auth.credential_unknown",
            ),
            (
                "credential_id",
                json!("credential:wrong_tenant:local"),
                StatusCode::UNAUTHORIZED,
                "auth.credential_wrong_tenant",
            ),
            (
                "timestamp",
                json!("2020-01-01T00:00:00Z"),
                StatusCode::UNAUTHORIZED,
                "auth.signature_expired",
            ),
            (
                "signature_ref",
                json!("signature:fixture:replayed"),
                StatusCode::UNAUTHORIZED,
                "auth.signature_replay_window_failed",
            ),
            (
                "signature_ref",
                json!("signature:fixture:malformed"),
                StatusCode::UNAUTHORIZED,
                "auth.signature_invalid",
            ),
            (
                "credential_id",
                json!("credential:revoked:local"),
                StatusCode::UNAUTHORIZED,
                "auth.credential_revoked",
            ),
            (
                "key_version",
                json!("key_version:rotated:local"),
                StatusCode::UNAUTHORIZED,
                "auth.credential_rotation_denied",
            ),
            (
                "key_version",
                json!("key_version:wrong:local"),
                StatusCode::UNAUTHORIZED,
                "auth.key_version_denied",
            ),
            (
                "algorithm",
                json!("rsa-pkcs1v15"),
                StatusCode::UNAUTHORIZED,
                "auth.signature_algorithm_denied",
            ),
            (
                "actor_id",
                json!("actor:unknown:local"),
                StatusCode::FORBIDDEN,
                "auth.actor_unknown",
            ),
            (
                "actor_id",
                json!("actor:disabled:local"),
                StatusCode::FORBIDDEN,
                "auth.actor_disabled",
            ),
            (
                "actor_id",
                json!("actor:suspended:local"),
                StatusCode::FORBIDDEN,
                "auth.actor_suspended",
            ),
            (
                "actor_id",
                json!("actor:deleted:local"),
                StatusCode::FORBIDDEN,
                "auth.actor_deleted",
            ),
            (
                "actor_id",
                json!("actor:wrong_type:local"),
                StatusCode::FORBIDDEN,
                "auth.actor_wrong_type",
            ),
            (
                "actor_id",
                json!("actor:env_mismatch:local"),
                StatusCode::FORBIDDEN,
                "auth.actor_environment_mismatch",
            ),
            (
                "tenant_id",
                json!("tenant:suspended:local"),
                StatusCode::FORBIDDEN,
                "auth.tenant_suspended",
            ),
            (
                "actor_id",
                json!("actor:no_membership:local"),
                StatusCode::FORBIDDEN,
                "auth.tenant_membership_denied",
            ),
            (
                "actor_id",
                json!("actor:role_denied:local"),
                StatusCode::FORBIDDEN,
                "auth.tenant_role_denied",
            ),
            (
                "service_account_broad",
                json!("overgate.phase4.service_account.broad"),
                StatusCode::FORBIDDEN,
                "auth.service_account_scope_denied",
            ),
            (
                "service_account_dev_secret",
                json!("credential:service_account:dev_secret"),
                StatusCode::FORBIDDEN,
                "auth.hardcoded_development_secret_denied",
            ),
            (
                "service_account_command_class",
                json!("overgate.phase4.tenant.write"),
                StatusCode::FORBIDDEN,
                "auth.service_account_command_class_denied",
            ),
            (
                "node_agent_command_class",
                json!("overgate.phase4.service_account.noop"),
                StatusCode::FORBIDDEN,
                "auth.node_agent_command_class_denied",
            ),
            (
                "service_account_missing_audit",
                json!("trace_phase4_missing_audit"),
                StatusCode::FORBIDDEN,
                "auth.service_account_audit_context_required",
            ),
            (
                "node_agent_missing_audit",
                json!("trace_phase4_missing_audit"),
                StatusCode::FORBIDDEN,
                "auth.node_agent_audit_context_required",
            ),
        ];

        for (field, value, status, reason_code) in cases {
            let app = OvergateService::default().router();
            let mut envelope = valid_command_envelope(&format!("trace_phase4_{reason_code}"));
            match field {
                "credential_id" => envelope["credential_id"] = value,
                "timestamp" => envelope["timestamp"] = value,
                "signature_ref" => envelope["signature_metadata"]["signature_ref"] = value,
                "key_version" => envelope["signature_metadata"]["key_version"] = value,
                "algorithm" => envelope["signature_metadata"]["algorithm"] = value,
                "actor_id" => envelope["actor_id"] = value,
                "tenant_id" => envelope["tenant_id"] = value,
                "service_account_broad" => {
                    envelope["actor_id"] = json!("service_account:local:ingress");
                    envelope["credential_id"] = json!("credential:service_account:broad");
                    envelope["command_type"] = value;
                }
                "service_account_dev_secret" => {
                    envelope["actor_id"] = json!("service_account:local:ingress");
                    envelope["credential_id"] = value;
                    envelope["command_type"] = json!("overgate.phase4.service_account.noop");
                }
                "service_account_command_class" => {
                    envelope["actor_id"] = json!("service_account:local:ingress");
                    envelope["credential_id"] = json!("credential:service_account:local:ingress");
                    envelope["command_type"] = value;
                }
                "node_agent_command_class" => {
                    envelope["actor_id"] = json!("node_agent:local:worker-1");
                    envelope["credential_id"] = json!("credential:node_agent:local:worker-1");
                    envelope["command_type"] = value;
                }
                "service_account_missing_audit" => {
                    envelope["actor_id"] = json!("service_account:local:ingress");
                    envelope["credential_id"] = json!("credential:service_account:local:ingress");
                    envelope["command_type"] = json!("overgate.phase4.service_account.noop");
                    envelope["trace_id"] = value;
                }
                "node_agent_missing_audit" => {
                    envelope["actor_id"] = json!("node_agent:local:worker-1");
                    envelope["credential_id"] = json!("credential:node_agent:local:worker-1");
                    envelope["command_type"] = json!("overgate.phase4.node_agent.callback");
                    envelope["trace_id"] = value;
                }
                _ => unreachable!("unknown denial case"),
            }
            let response = app
                .oneshot(command_post("/v1/commands", envelope))
                .await
                .expect("phase4 denial request should respond");
            assert_eq!(response.status(), status, "case {reason_code}");
            let body = body_json(response).await;
            assert_eq!(body["reason_code"], reason_code, "case {reason_code}");
            assert_eq!(body["data"]["diagnostics"]["redacted"], true);
        }
    }

    #[tokio::test]
    async fn readyz_separates_liveness_from_dependency_authority() {
        let dependencies = DependencyMatrix::default()
            .with_dependency_state("overkey_lite", DependencyState::Unavailable);
        let service = OvergateService::with_dependencies(OvergateConfig::default(), dependencies);

        let health_response = service
            .router()
            .oneshot(empty_get("/v1/healthz"))
            .await
            .expect("health route should respond");
        assert_eq!(health_response.status(), StatusCode::OK);

        let ready_response = service
            .router()
            .oneshot(empty_get("/v1/readyz"))
            .await
            .expect("ready route should respond");
        assert_eq!(ready_response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = body_json(ready_response).await;
        assert_eq!(body["reason_code"], "overgate.dependency_unavailable");
        assert_eq!(body["data"]["ready"], false);
        assert!(body["data"]["required_failures"]
            .as_array()
            .expect("failures should be an array")
            .iter()
            .any(|value| value == "overkey_lite"));
    }

    #[tokio::test]
    async fn admin_routes_deny_unsigned_non_operator_and_cross_tenant_requests() {
        let app = OvergateService::default().router();
        let unsigned = app
            .oneshot(empty_get("/v1/admin/ingress/request_local_1"))
            .await
            .expect("admin route should deny unsigned request");
        assert_eq!(unsigned.status(), StatusCode::UNAUTHORIZED);
        let body = body_json(unsigned).await;
        assert_eq!(body["reason_code"], "auth.operator_signature_required");

        let app = OvergateService::default().router();
        let malformed_signature = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/v1/admin/rate-limits")
                    .header(OPERATOR_SIGNATURE_HEADER, "sig:test:operator")
                    .header(OPERATOR_ROLE_HEADER, "operator")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("admin route should deny malformed operator signature");
        assert_eq!(malformed_signature.status(), StatusCode::UNAUTHORIZED);
        let body = body_json(malformed_signature).await;
        assert_eq!(body["reason_code"], "auth.operator_signature_malformed");

        let app = OvergateService::default().router();
        let non_operator = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/v1/admin/rate-limits")
                    .header(OPERATOR_SIGNATURE_HEADER, "signature:fixture:operator")
                    .header(OPERATOR_ROLE_HEADER, "viewer")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("admin route should deny non-operator request");
        assert_eq!(non_operator.status(), StatusCode::FORBIDDEN);
        let body = body_json(non_operator).await;
        assert_eq!(body["reason_code"], "auth.operator_role_required");

        let app = OvergateService::default().router();
        let cross_tenant = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/v1/admin/idempotency/tenant:alpha/idem_1")
                    .header(OPERATOR_SIGNATURE_HEADER, "signature:fixture:operator")
                    .header(OPERATOR_ROLE_HEADER, "operator")
                    .header(TENANT_HEADER, "tenant:beta")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("admin route should deny cross-tenant request");
        assert_eq!(cross_tenant.status(), StatusCode::FORBIDDEN);
        let body = body_json(cross_tenant).await;
        assert_eq!(body["reason_code"], "auth.cross_tenant_denied");

        let app = OvergateService::default().router();
        let allowed = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/v1/admin/idempotency/tenant:alpha/idem_1")
                    .header(OPERATOR_SIGNATURE_HEADER, "signature:fixture:operator")
                    .header(OPERATOR_ROLE_HEADER, "operator")
                    .header(TENANT_HEADER, "tenant:alpha")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("admin route should allow signed operator request");
        assert_eq!(allowed.status(), StatusCode::OK);
        let body = body_json(allowed).await;
        assert_eq!(
            body["reason_code"],
            "overgate.admin_idempotency_lookup_phase5"
        );
        assert_eq!(
            body["data"]["operator_admission"]["reason_code"],
            "auth.operator_admitted_phase4"
        );

        let app = OvergateService::default().router();
        let mut tenant_alpha_command = valid_command_envelope("trace_phase5_admin_expire_alpha");
        tenant_alpha_command["tenant_id"] = json!("tenant:alpha");
        tenant_alpha_command["idempotency_key"] = json!("idem:overgate:phase5:admin_expire");
        tenant_alpha_command["request_hash"] = json!("hash:fixture:phase5_admin_expire_request");
        tenant_alpha_command["payload_hash"] = json!("hash:fixture:phase5_admin_expire_payload");
        let accepted = app
            .clone()
            .oneshot(command_post("/v1/commands", tenant_alpha_command))
            .await
            .expect("tenant alpha command should be accepted");
        assert_eq!(accepted.status(), StatusCode::ACCEPTED);
        let accepted_body = body_json(accepted).await;
        let record_id = accepted_body["data"]["idempotency"]["record"]["record_id"]
            .as_str()
            .expect("record id should be present")
            .to_owned();

        let cross_tenant_expire = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/admin/idempotency/{record_id}/expire"))
                    .header(OPERATOR_SIGNATURE_HEADER, "signature:fixture:operator")
                    .header(OPERATOR_ROLE_HEADER, "operator")
                    .header(TENANT_HEADER, "tenant:beta")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("cross-tenant expire route should respond");
        assert_eq!(cross_tenant_expire.status(), StatusCode::FORBIDDEN);
        let body = body_json(cross_tenant_expire).await;
        assert_eq!(body["reason_code"], "auth.cross_tenant_denied");
        assert_eq!(
            body["data"]["denial"],
            "cross_tenant_idempotency_expire_denied"
        );

        let allowed_expire = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/admin/idempotency/{record_id}/expire"))
                    .header(OPERATOR_SIGNATURE_HEADER, "signature:fixture:operator")
                    .header(OPERATOR_ROLE_HEADER, "operator")
                    .header(TENANT_HEADER, "tenant:alpha")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("tenant-scoped expire route should respond");
        assert_eq!(allowed_expire.status(), StatusCode::OK);
        let body = body_json(allowed_expire).await;
        assert_eq!(
            body["reason_code"],
            "overgate.admin_idempotency_expire_phase5"
        );
        assert_eq!(
            body["data"]["idempotency_records"][0]["current_state"],
            "retention_expired"
        );

        let dependencies = DependencyMatrix::default()
            .with_dependency_state("overwatch", DependencyState::Unavailable);
        let service = OvergateService::with_dependencies(OvergateConfig::default(), dependencies);
        let audit_blocked = service
            .router()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/v1/admin/rate-limits")
                    .header(OPERATOR_SIGNATURE_HEADER, "signature:fixture:operator")
                    .header(OPERATOR_ROLE_HEADER, "operator")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("admin route should fail closed without Overwatch");
        assert_eq!(audit_blocked.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = body_json(audit_blocked).await;
        assert_eq!(body["reason_code"], "auth.operator_audit_unavailable");
    }
}
