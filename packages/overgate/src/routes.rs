use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use serde_json::{json, Value};

use crate::service::OvergateState;

pub const TRACE_HEADER: &str = "x-overrid-trace-id";
pub const TENANT_HEADER: &str = "x-overrid-tenant-id";

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
            schema_version: "overgate.phase2.response.v0",
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
    Json(payload): Json<Value>,
) -> (StatusCode, Json<ApiResponse<CommandAcceptedData>>) {
    let trace_id = trace_id(&headers, "trace_overgate_phase2_command");
    let request_id = format!("request_{}", stable_short_token(&[&trace_id, "command"]));
    let command_id = format!("command_{}", stable_short_token(&[&trace_id, "accepted"]));
    let payload_hash_ref = format!(
        "hash:placeholder:{}",
        stable_short_token(&[&payload.to_string()])
    );
    (
        StatusCode::ACCEPTED,
        Json(ApiResponse::new(
            trace_id,
            "accepted",
            "overgate.command_route_skeleton",
            CommandAcceptedData {
                route: ROUTE_SUBMIT_COMMAND,
                request_id: request_id.clone(),
                command_id,
                audit_ref: format!("audit:overgate:request_received:{request_id}"),
                forwarding_state: "not_forwarded_phase2_placeholder",
                payload_hash_ref,
            },
        )),
    )
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
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}").chars().take(12).collect()
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
    use serde_json::Value;
    use tower::ServiceExt;

    use super::*;
    use crate::admin::{OPERATOR_ROLE_HEADER, OPERATOR_SIGNATURE_HEADER};
    use crate::dependencies::{DependencyMatrix, DependencyState};
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
                    .body(Body::from(r#"{"command_type":"overgate.phase2.noop"}"#))
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
        assert_eq!(body["reason_code"], "overgate.command_route_skeleton");
        assert_eq!(body["data"]["route"], ROUTE_SUBMIT_COMMAND);

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
