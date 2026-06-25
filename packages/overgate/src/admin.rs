use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;

use crate::routes::{
    header_value, stable_short_token, trace_id, ApiResponse, TENANT_HEADER, TRACE_HEADER,
};
use crate::service::OvergateState;

pub const OPERATOR_SIGNATURE_HEADER: &str = "x-overrid-operator-signature";
pub const OPERATOR_ROLE_HEADER: &str = "x-overrid-operator-role";

pub const ADMIN_ROUTE_INGRESS_LOOKUP: &str = "GET /v1/admin/ingress/{request_id}";
pub const ADMIN_ROUTE_IDEMPOTENCY_LOOKUP: &str =
    "GET /v1/admin/idempotency/{tenant_id}/{idempotency_key}";
pub const ADMIN_ROUTE_IDEMPOTENCY_EXPIRE: &str = "POST /v1/admin/idempotency/{record_id}/expire";
pub const ADMIN_ROUTE_RATE_LIMITS: &str = "GET /v1/admin/rate-limits";

#[derive(Debug, Clone, PartialEq, Eq)]
struct OperatorContext {
    tenant_id: String,
    role: String,
}

#[derive(Debug, Serialize)]
struct AdminDeniedData {
    route_scope: &'static str,
    denial: &'static str,
}

#[derive(Debug, Serialize)]
struct AdminLookupData {
    route: &'static str,
    requested_ref: String,
    operator_tenant: String,
    visibility: &'static str,
    audit_hook_ref: String,
}

pub fn admin_routes() -> Router<OvergateState> {
    Router::new()
        .route("/v1/admin/ingress/:request_id", get(admin_ingress_lookup))
        .route(
            "/v1/admin/idempotency/:tenant_id/:idempotency_key",
            get(admin_idempotency_lookup),
        )
        .route(
            "/v1/admin/idempotency/:record_id/expire",
            post(admin_idempotency_expire),
        )
        .route("/v1/admin/rate-limits", get(admin_rate_limits))
}

async fn admin_ingress_lookup(
    State(_state): State<OvergateState>,
    headers: HeaderMap,
    Path(request_id): Path<String>,
) -> Result<Json<ApiResponse<AdminLookupData>>, (StatusCode, Json<ApiResponse<AdminDeniedData>>)> {
    let trace_id = trace_id(&headers, "trace_overgate_admin_ingress");
    let operator = authorize_operator(&headers, None, &trace_id)?;
    Ok(Json(ApiResponse::new(
        trace_id,
        "ok",
        "overgate.admin_ingress_route_skeleton",
        AdminLookupData {
            route: ADMIN_ROUTE_INGRESS_LOOKUP,
            requested_ref: request_id,
            operator_tenant: operator.tenant_id,
            visibility: "signed_operator_placeholder",
            audit_hook_ref: "audit_hook:overgate:admin_ingress_lookup".to_owned(),
        },
    )))
}

async fn admin_idempotency_lookup(
    headers: HeaderMap,
    Path((tenant_id, idempotency_key)): Path<(String, String)>,
) -> Result<Json<ApiResponse<AdminLookupData>>, (StatusCode, Json<ApiResponse<AdminDeniedData>>)> {
    let trace_id = trace_id(&headers, "trace_overgate_admin_idempotency");
    let operator = authorize_operator(&headers, Some(&tenant_id), &trace_id)?;
    Ok(Json(ApiResponse::new(
        trace_id,
        "ok",
        "overgate.admin_idempotency_route_skeleton",
        AdminLookupData {
            route: ADMIN_ROUTE_IDEMPOTENCY_LOOKUP,
            requested_ref: format!("{tenant_id}/{idempotency_key}"),
            operator_tenant: operator.tenant_id,
            visibility: "tenant_scoped_operator_placeholder",
            audit_hook_ref: "audit_hook:overgate:admin_idempotency_lookup".to_owned(),
        },
    )))
}

async fn admin_idempotency_expire(
    headers: HeaderMap,
    Path(record_id): Path<String>,
) -> Result<Json<ApiResponse<AdminLookupData>>, (StatusCode, Json<ApiResponse<AdminDeniedData>>)> {
    let trace_id = trace_id(&headers, "trace_overgate_admin_idempotency_expire");
    let operator = authorize_operator(&headers, None, &trace_id)?;
    Ok(Json(ApiResponse::new(
        trace_id,
        "accepted",
        "overgate.admin_idempotency_expire_route_skeleton",
        AdminLookupData {
            route: ADMIN_ROUTE_IDEMPOTENCY_EXPIRE,
            requested_ref: record_id,
            operator_tenant: operator.tenant_id,
            visibility: "signed_operator_expire_placeholder",
            audit_hook_ref: "audit_hook:overgate:admin_idempotency_expire".to_owned(),
        },
    )))
}

async fn admin_rate_limits(
    headers: HeaderMap,
) -> Result<Json<ApiResponse<AdminLookupData>>, (StatusCode, Json<ApiResponse<AdminDeniedData>>)> {
    let trace_id = trace_id(&headers, "trace_overgate_admin_rate_limits");
    let operator = authorize_operator(&headers, None, &trace_id)?;
    Ok(Json(ApiResponse::new(
        trace_id,
        "ok",
        "overgate.admin_rate_limits_route_skeleton",
        AdminLookupData {
            route: ADMIN_ROUTE_RATE_LIMITS,
            requested_ref: "rate_limits:phase2_placeholder".to_owned(),
            operator_tenant: operator.tenant_id,
            visibility: "signed_operator_placeholder",
            audit_hook_ref: "audit_hook:overgate:admin_rate_limits".to_owned(),
        },
    )))
}

fn authorize_operator(
    headers: &HeaderMap,
    tenant_scope: Option<&str>,
    trace_id: &str,
) -> Result<OperatorContext, (StatusCode, Json<ApiResponse<AdminDeniedData>>)> {
    let signature = header_value(headers, OPERATOR_SIGNATURE_HEADER);
    if signature.is_none() {
        return Err(admin_denial(
            StatusCode::UNAUTHORIZED,
            trace_id,
            "auth.operator_signature_required",
            "missing_operator_signature",
        ));
    }

    let role = header_value(headers, OPERATOR_ROLE_HEADER).unwrap_or_default();
    if role != "operator" && role != "system_service" {
        return Err(admin_denial(
            StatusCode::FORBIDDEN,
            trace_id,
            "auth.operator_role_required",
            "operator_role_required",
        ));
    }

    let tenant_id =
        header_value(headers, TENANT_HEADER).unwrap_or_else(|| "tenant:local:test".to_owned());
    if let Some(scope) = tenant_scope {
        if tenant_id != scope {
            return Err(admin_denial(
                StatusCode::FORBIDDEN,
                trace_id,
                "auth.cross_tenant_denied",
                "cross_tenant_denied",
            ));
        }
    }

    Ok(OperatorContext { tenant_id, role })
}

fn admin_denial(
    status: StatusCode,
    trace_id: &str,
    reason_code: &'static str,
    denial: &'static str,
) -> (StatusCode, Json<ApiResponse<AdminDeniedData>>) {
    (
        status,
        Json(ApiResponse::new(
            trace_id.to_owned(),
            "denied",
            reason_code,
            AdminDeniedData {
                route_scope: "overgate_admin",
                denial,
            },
        )),
    )
}

pub fn admin_audit_ref(route: &str, trace: &str) -> String {
    format!(
        "audit_hook:overgate:{}:{}",
        stable_short_token(&[route]),
        stable_short_token(&[trace])
    )
}

pub fn placeholder_admin_headers(trace: &str, tenant_id: &str) -> [(String, String); 4] {
    [
        (TRACE_HEADER.to_owned(), trace.to_owned()),
        (
            OPERATOR_SIGNATURE_HEADER.to_owned(),
            "sig:test:operator".to_owned(),
        ),
        (OPERATOR_ROLE_HEADER.to_owned(), "operator".to_owned()),
        (TENANT_HEADER.to_owned(), tenant_id.to_owned()),
    ]
}
