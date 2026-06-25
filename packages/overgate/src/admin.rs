use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;

use crate::admission::{operator_admission_record, overwatch_ready, OperatorAdmissionRecord};
use crate::dependencies::DependencyMatrix;
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
    admission: OperatorAdmissionRecord,
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
    operator_role: String,
    operator_admission: OperatorAdmissionRecord,
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
    State(state): State<OvergateState>,
    headers: HeaderMap,
    Path(request_id): Path<String>,
) -> Result<Json<ApiResponse<AdminLookupData>>, (StatusCode, Json<ApiResponse<AdminDeniedData>>)> {
    let trace_id = trace_id(&headers, "trace_overgate_admin_ingress");
    let operator = authorize_operator(&headers, None, &trace_id, &state.dependencies)?;
    let audit_hook_ref = admin_audit_ref(ADMIN_ROUTE_INGRESS_LOOKUP, &trace_id);
    Ok(Json(ApiResponse::new(
        trace_id.clone(),
        "ok",
        "overgate.admin_ingress_admitted_phase4",
        AdminLookupData {
            route: ADMIN_ROUTE_INGRESS_LOOKUP,
            requested_ref: request_id,
            operator_tenant: operator.tenant_id,
            operator_role: operator.role,
            operator_admission: operator.admission,
            visibility: "signed_operator_tenant_scoped_phase4",
            audit_hook_ref,
        },
    )))
}

async fn admin_idempotency_lookup(
    State(state): State<OvergateState>,
    headers: HeaderMap,
    Path((tenant_id, idempotency_key)): Path<(String, String)>,
) -> Result<Json<ApiResponse<AdminLookupData>>, (StatusCode, Json<ApiResponse<AdminDeniedData>>)> {
    let trace_id = trace_id(&headers, "trace_overgate_admin_idempotency");
    let operator = authorize_operator(&headers, Some(&tenant_id), &trace_id, &state.dependencies)?;
    let audit_hook_ref = admin_audit_ref(ADMIN_ROUTE_IDEMPOTENCY_LOOKUP, &trace_id);
    Ok(Json(ApiResponse::new(
        trace_id.clone(),
        "ok",
        "overgate.admin_idempotency_admitted_phase4",
        AdminLookupData {
            route: ADMIN_ROUTE_IDEMPOTENCY_LOOKUP,
            requested_ref: format!("{tenant_id}/{idempotency_key}"),
            operator_tenant: operator.tenant_id,
            operator_role: operator.role,
            operator_admission: operator.admission,
            visibility: "tenant_scoped_operator_phase4",
            audit_hook_ref,
        },
    )))
}

async fn admin_idempotency_expire(
    State(state): State<OvergateState>,
    headers: HeaderMap,
    Path(record_id): Path<String>,
) -> Result<Json<ApiResponse<AdminLookupData>>, (StatusCode, Json<ApiResponse<AdminDeniedData>>)> {
    let trace_id = trace_id(&headers, "trace_overgate_admin_idempotency_expire");
    let operator = authorize_operator(&headers, None, &trace_id, &state.dependencies)?;
    let audit_hook_ref = admin_audit_ref(ADMIN_ROUTE_IDEMPOTENCY_EXPIRE, &trace_id);
    Ok(Json(ApiResponse::new(
        trace_id.clone(),
        "accepted",
        "overgate.admin_idempotency_expire_admitted_phase4",
        AdminLookupData {
            route: ADMIN_ROUTE_IDEMPOTENCY_EXPIRE,
            requested_ref: record_id,
            operator_tenant: operator.tenant_id,
            operator_role: operator.role,
            operator_admission: operator.admission,
            visibility: "signed_operator_expire_phase4",
            audit_hook_ref,
        },
    )))
}

async fn admin_rate_limits(
    State(state): State<OvergateState>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<AdminLookupData>>, (StatusCode, Json<ApiResponse<AdminDeniedData>>)> {
    let trace_id = trace_id(&headers, "trace_overgate_admin_rate_limits");
    let operator = authorize_operator(&headers, None, &trace_id, &state.dependencies)?;
    let audit_hook_ref = admin_audit_ref(ADMIN_ROUTE_RATE_LIMITS, &trace_id);
    Ok(Json(ApiResponse::new(
        trace_id.clone(),
        "ok",
        "overgate.admin_rate_limits_admitted_phase4",
        AdminLookupData {
            route: ADMIN_ROUTE_RATE_LIMITS,
            requested_ref: "rate_limits:phase4_operator_view".to_owned(),
            operator_tenant: operator.tenant_id,
            operator_role: operator.role,
            operator_admission: operator.admission,
            visibility: "signed_operator_tenant_scoped_phase4",
            audit_hook_ref,
        },
    )))
}

fn authorize_operator(
    headers: &HeaderMap,
    tenant_scope: Option<&str>,
    trace_id: &str,
    dependencies: &DependencyMatrix,
) -> Result<OperatorContext, (StatusCode, Json<ApiResponse<AdminDeniedData>>)> {
    let signature = header_value(headers, OPERATOR_SIGNATURE_HEADER);
    let Some(signature) = signature else {
        return Err(admin_denial(
            StatusCode::UNAUTHORIZED,
            trace_id,
            "auth.operator_signature_required",
            "missing_operator_signature",
        ));
    };
    if !signature.starts_with("signature:") {
        return Err(admin_denial(
            StatusCode::UNAUTHORIZED,
            trace_id,
            "auth.operator_signature_malformed",
            "malformed_operator_signature",
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

    if !overwatch_ready(dependencies) {
        return Err(admin_denial(
            StatusCode::SERVICE_UNAVAILABLE,
            trace_id,
            "auth.operator_audit_unavailable",
            "overwatch_unavailable_fail_closed",
        ));
    }

    let admission = operator_admission_record(&role, &tenant_id, trace_id);
    Ok(OperatorContext {
        tenant_id,
        role,
        admission,
    })
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
            "signature:fixture:operator".to_owned(),
        ),
        (OPERATOR_ROLE_HEADER.to_owned(), "operator".to_owned()),
        (TENANT_HEADER.to_owned(), tenant_id.to_owned()),
    ]
}
