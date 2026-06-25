use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use serde_json::{json, Value};

use crate::canonical::CanonicalRequestInput;
use crate::envelope::{trace_id_hint, CommandEnvelope};
use crate::errors::{ApiErrorData, OvergateError};
use crate::retention::RetentionDecision;
use crate::schema::{validate_command_envelope, SchemaValidationReport};
use crate::service::OvergateState;

pub const TRACE_HEADER: &str = "x-overrid-trace-id";
pub const TENANT_HEADER: &str = "x-overrid-tenant-id";
pub const PHASE3_RESPONSE_SCHEMA_VERSION: &str = "overgate.phase3.response.v0";
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
            schema_version: PHASE3_RESPONSE_SCHEMA_VERSION,
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
    retention: RetentionDecision,
}

#[derive(Debug, Serialize)]
struct CommandStatusData {
    route: &'static str,
    command_id: String,
    admission_state: &'static str,
    forwarding_state: &'static str,
    owner: &'static str,
}

#[derive(Debug, Serialize)]
struct TraceStatusData {
    route: &'static str,
    trace_id: String,
    audit_refs: Vec<String>,
    caller_visible: bool,
}

#[derive(Debug, Serialize)]
struct LimitsData {
    route: &'static str,
    tenant_scope: String,
    rate_limit_refs: Vec<&'static str>,
    quota_precheck_refs: Vec<&'static str>,
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
    State(_state): State<OvergateState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<
    (StatusCode, Json<ApiResponse<CommandAcceptedData>>),
    (StatusCode, Json<ApiResponse<ApiErrorData>>),
> {
    let fallback_trace = trace_id_hint(&headers, &body, "trace_overgate_phase3_parse_denied");
    let parsed = CommandEnvelope::parse_http(&headers, &body)
        .map_err(|error| api_error_response(fallback_trace.clone(), None, error))?;
    let schema_validation = validate_command_envelope(&parsed.envelope)
        .map_err(|error| api_error_response(parsed.envelope.trace_id.clone(), None, error))?;
    let canonical_request = CanonicalRequestInput::from_envelope(
        "POST",
        "/v1/commands",
        &parsed.envelope,
        &parsed.body_hash,
    );
    let retention = RetentionDecision::from_envelope(&parsed.envelope, &parsed.body_hash);
    let request_id = format!(
        "request_{}",
        stable_short_token(&[
            canonical_request.canonical_hash.as_str(),
            parsed.envelope.trace_id.as_str()
        ])
    );

    Ok((
        StatusCode::ACCEPTED,
        Json(ApiResponse::new(
            parsed.envelope.trace_id.clone(),
            "accepted",
            "overgate.command_validated_phase3",
            CommandAcceptedData {
                route: ROUTE_SUBMIT_COMMAND,
                request_id: request_id.clone(),
                command_id: parsed.envelope.command_id.clone(),
                audit_ref: format!("audit:overgate:request_received:{request_id}"),
                forwarding_state: "not_forwarded_phase3_validation_only",
                payload_hash_ref: parsed.envelope.payload_hash.clone(),
                request_hash_ref: parsed.envelope.request_hash.clone(),
                body_hash_ref: parsed.body_hash,
                schema_validation,
                canonical_request,
                retention,
            },
        )),
    ))
}

async fn command_status(
    headers: HeaderMap,
    Path(command_id): Path<String>,
) -> Json<ApiResponse<CommandStatusData>> {
    Json(ApiResponse::new(
        trace_id(&headers, "trace_overgate_phase2_status"),
        "ok",
        "overgate.command_status_route_skeleton",
        CommandStatusData {
            route: ROUTE_COMMAND_STATUS,
            command_id,
            admission_state: "route_skeleton_only",
            forwarding_state: "not_forwarded_phase2_placeholder",
            owner: "overgate_until_downstream_handoff",
        },
    ))
}

async fn trace_status(
    headers: HeaderMap,
    Path(trace_id_from_path): Path<String>,
) -> Json<ApiResponse<TraceStatusData>> {
    let trace_id = trace_id(&headers, &trace_id_from_path);
    Json(ApiResponse::new(
        trace_id.clone(),
        "ok",
        "overgate.trace_route_skeleton",
        TraceStatusData {
            route: ROUTE_TRACE_STATUS,
            trace_id,
            audit_refs: vec!["audit:overgate:placeholder".to_owned()],
            caller_visible: true,
        },
    ))
}

async fn limits(
    headers: HeaderMap,
    State(_state): State<OvergateState>,
) -> Json<ApiResponse<LimitsData>> {
    let tenant_scope =
        header_value(&headers, TENANT_HEADER).unwrap_or_else(|| "tenant:local:test".to_owned());
    Json(ApiResponse::new(
        trace_id(&headers, "trace_overgate_phase2_limits"),
        "ok",
        "overgate.limits_route_skeleton",
        LimitsData {
            route: ROUTE_LIMITS,
            tenant_scope,
            rate_limit_refs: vec!["rate_limit:phase2:placeholder"],
            quota_precheck_refs: vec!["quota_precheck:phase2:placeholder"],
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
        assert_eq!(body["schema_version"], PHASE3_RESPONSE_SCHEMA_VERSION);
        assert_eq!(body["reason_code"], "overgate.command_validated_phase3");
        assert_eq!(body["data"]["route"], ROUTE_SUBMIT_COMMAND);
        assert_eq!(body["data"]["command_id"], "command:overgate:phase3:0001");
        assert_eq!(
            body["data"]["schema_validation"]["adapter_id"],
            "overgate.phase3.shared_schema_adapter"
        );
        assert_eq!(
            body["data"]["canonical_request"]["canonicalization_version"],
            "overgate.canonical.v0.1"
        );
        assert_eq!(
            body["data"]["retention"]["body_retention"],
            "raw_body_not_retained"
        );

        let app = OvergateService::default().router();
        let response = app
            .oneshot(empty_get("/v1/commands/command_local_1"))
            .await
            .expect("status route should respond");
        assert_eq!(response.status(), StatusCode::OK);
        let body = body_json(response).await;
        assert_eq!(body["data"]["route"], ROUTE_COMMAND_STATUS);
        assert_eq!(body["data"]["owner"], "overgate_until_downstream_handoff");
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
        assert_eq!(body["reason_code"], "overgate.command_validated_phase3");
        assert_eq!(body["data"]["route"], ROUTE_SUBMIT_COMMAND);
        assert_eq!(
            body["data"]["forwarding_state"],
            "not_forwarded_phase3_validation_only"
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
        let non_operator = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/v1/admin/rate-limits")
                    .header(OPERATOR_SIGNATURE_HEADER, "sig:test:operator")
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
                    .header(OPERATOR_SIGNATURE_HEADER, "sig:test:operator")
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
                    .header(OPERATOR_SIGNATURE_HEADER, "sig:test:operator")
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
            "overgate.admin_idempotency_route_skeleton"
        );
    }
}
