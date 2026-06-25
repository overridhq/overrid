use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;

use crate::admission::{operator_admission_record, overwatch_ready, OperatorAdmissionRecord};
use crate::dependencies::DependencyMatrix;
use crate::idempotency::{AdminRecordLookup, IdempotencyMutation, IdempotencyRecord};
use crate::prechecks::{PrecheckLimitSummary, RateLimitBucketView};
use crate::routes::{
    header_value, stable_short_token, trace_id, ApiResponse, ClientResponseShape,
    PHASE9_RESPONSE_SCHEMA_VERSION, TENANT_HEADER, TRACE_HEADER,
};
use crate::service::OvergateState;

pub const OPERATOR_SIGNATURE_HEADER: &str = "x-overrid-operator-signature";
pub const OPERATOR_ROLE_HEADER: &str = "x-overrid-operator-role";

pub const ADMIN_ROUTE_INGRESS_LOOKUP: &str = "GET /v1/admin/ingress/{request_id}";
pub const ADMIN_ROUTE_IDEMPOTENCY_LOOKUP: &str =
    "GET /v1/admin/idempotency/{tenant_id}/{idempotency_key}";
pub const ADMIN_ROUTE_IDEMPOTENCY_EXPIRE: &str = "POST /v1/admin/idempotency/{record_id}/expire";
pub const ADMIN_ROUTE_RATE_LIMITS: &str = "GET /v1/admin/rate-limits";
pub const ADMIN_INGRESS_PHASE4_COMPAT_VISIBILITY: &str = "signed_operator_tenant_scoped_phase4";
pub const ADMIN_IDEMPOTENCY_PHASE5_COMPAT_LOOKUP_REASON: &str =
    "overgate.admin_idempotency_lookup_phase5";
pub const ADMIN_IDEMPOTENCY_PHASE5_COMPAT_EXPIRE_REASON: &str =
    "overgate.admin_idempotency_expire_phase5";
pub const ADMIN_RATE_LIMITS_PHASE6_COMPAT_REASON: &str = "overgate.admin_rate_limits_phase6";
pub const ADMIN_RATE_LIMITS_PHASE6_COMPAT_REQUEST_REF: &str = "rate_limits:phase6_operator_view";
pub const ADMIN_RATE_LIMITS_PHASE6_COMPAT_VISIBILITY: &str = "signed_operator_tenant_scoped_phase6";

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
    audit_hook_ref: String,
    client_response_shape: ClientResponseShape,
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
    idempotency_records: Vec<IdempotencyRecord>,
    rate_limit_buckets: Vec<RateLimitBucketView>,
    quota_precheck_refs: Vec<String>,
    local_counter_refs: Vec<String>,
    grant_placeholder_refs: Vec<String>,
    policy_decision_refs: Vec<String>,
    phase6_precheck_summary: Option<PrecheckLimitSummary>,
    redaction_state: &'static str,
    dependency_refs: Vec<&'static str>,
    incident_hook_refs: Vec<String>,
    operator_runbook_steps: Vec<&'static str>,
    client_response_shape: ClientResponseShape,
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
    let idempotency_records = match state
        .idempotency
        .admin_records_for_request(&operator.tenant_id, &request_id)
    {
        AdminRecordLookup::Found(records) => records,
        AdminRecordLookup::Forbidden => {
            return Err(admin_denial(
                StatusCode::FORBIDDEN,
                &trace_id,
                ADMIN_ROUTE_INGRESS_LOOKUP,
                "auth.cross_tenant_denied",
                "cross_tenant_ingress_lookup_denied",
            ));
        }
        AdminRecordLookup::Missing => {
            return Err(admin_denial(
                StatusCode::NOT_FOUND,
                &trace_id,
                ADMIN_ROUTE_INGRESS_LOOKUP,
                "overgate.admin_ingress_record_not_found",
                "ingress_request_not_found",
            ));
        }
    };
    Ok(Json(ApiResponse::new_with_schema_version(
        PHASE9_RESPONSE_SCHEMA_VERSION,
        trace_id.clone(),
        "ok",
        "overgate.admin_ingress_lookup_phase9",
        AdminLookupData {
            route: ADMIN_ROUTE_INGRESS_LOOKUP,
            requested_ref: request_id,
            operator_tenant: operator.tenant_id,
            operator_role: operator.role,
            operator_admission: operator.admission,
            visibility: "signed_operator_tenant_scoped_redacted_phase9",
            audit_hook_ref,
            idempotency_records,
            rate_limit_buckets: Vec::new(),
            quota_precheck_refs: Vec::new(),
            local_counter_refs: Vec::new(),
            grant_placeholder_refs: Vec::new(),
            policy_decision_refs: Vec::new(),
            phase6_precheck_summary: None,
            redaction_state: "private_metadata_redacted_phase9",
            dependency_refs: vec!["service:overwatch", "service:overgate"],
            incident_hook_refs: incident_hook_refs(ADMIN_ROUTE_INGRESS_LOOKUP, &trace_id),
            operator_runbook_steps: ingress_runbook_steps(),
            client_response_shape: ClientResponseShape::new(
                ADMIN_ROUTE_INGRESS_LOOKUP,
                "overgate.admin_ingress_lookup_phase9",
                vec![
                    "visibility",
                    "audit_hook_ref",
                    "idempotency_records",
                    "redaction_state",
                    "incident_hook_refs",
                    "operator_runbook_steps",
                ],
            ),
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
    let idempotency_records = state
        .idempotency
        .admin_records_for_key(&tenant_id, &idempotency_key);
    Ok(Json(ApiResponse::new_with_schema_version(
        PHASE9_RESPONSE_SCHEMA_VERSION,
        trace_id.clone(),
        "ok",
        "overgate.admin_idempotency_lookup_phase9",
        AdminLookupData {
            route: ADMIN_ROUTE_IDEMPOTENCY_LOOKUP,
            requested_ref: format!("{tenant_id}/{idempotency_key}"),
            operator_tenant: operator.tenant_id,
            operator_role: operator.role,
            operator_admission: operator.admission,
            visibility: "tenant_scoped_operator_redacted_phase9",
            audit_hook_ref,
            idempotency_records,
            rate_limit_buckets: Vec::new(),
            quota_precheck_refs: Vec::new(),
            local_counter_refs: Vec::new(),
            grant_placeholder_refs: Vec::new(),
            policy_decision_refs: Vec::new(),
            phase6_precheck_summary: None,
            redaction_state: "private_metadata_redacted_phase9",
            dependency_refs: vec!["service:overwatch", "service:overgate"],
            incident_hook_refs: incident_hook_refs(ADMIN_ROUTE_IDEMPOTENCY_LOOKUP, &trace_id),
            operator_runbook_steps: idempotency_runbook_steps(),
            client_response_shape: ClientResponseShape::new(
                ADMIN_ROUTE_IDEMPOTENCY_LOOKUP,
                "overgate.admin_idempotency_lookup_phase9",
                vec![
                    "visibility",
                    "idempotency_records",
                    "retention_class",
                    "conflict_reason",
                    "audit_hook_ref",
                ],
            ),
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
    let operator_tenant = operator.tenant_id.clone();
    let idempotency_records = match state
        .idempotency
        .expire_record(&operator_tenant, &record_id)
    {
        IdempotencyMutation::Applied(record) => vec![record],
        IdempotencyMutation::Forbidden => {
            return Err(admin_denial(
                StatusCode::FORBIDDEN,
                &trace_id,
                ADMIN_ROUTE_IDEMPOTENCY_EXPIRE,
                "auth.cross_tenant_denied",
                "cross_tenant_idempotency_expire_denied",
            ));
        }
        IdempotencyMutation::Protected(reason) => {
            return Err(admin_denial(
                StatusCode::CONFLICT,
                &trace_id,
                ADMIN_ROUTE_IDEMPOTENCY_EXPIRE,
                "overgate.admin_idempotency_expiration_refused_phase9",
                reason,
            ));
        }
        IdempotencyMutation::Missing => {
            return Err(admin_denial(
                StatusCode::NOT_FOUND,
                &trace_id,
                ADMIN_ROUTE_IDEMPOTENCY_EXPIRE,
                "overgate.admin_idempotency_record_not_found",
                "idempotency_record_not_found",
            ));
        }
    };
    Ok(Json(ApiResponse::new_with_schema_version(
        PHASE9_RESPONSE_SCHEMA_VERSION,
        trace_id.clone(),
        "accepted",
        "overgate.admin_idempotency_expire_phase9",
        AdminLookupData {
            route: ADMIN_ROUTE_IDEMPOTENCY_EXPIRE,
            requested_ref: record_id,
            operator_tenant: operator.tenant_id,
            operator_role: operator.role,
            operator_admission: operator.admission,
            visibility: "signed_operator_expire_audited_phase9",
            audit_hook_ref,
            idempotency_records,
            rate_limit_buckets: Vec::new(),
            quota_precheck_refs: Vec::new(),
            local_counter_refs: Vec::new(),
            grant_placeholder_refs: Vec::new(),
            policy_decision_refs: Vec::new(),
            phase6_precheck_summary: None,
            redaction_state: "private_metadata_redacted_phase9",
            dependency_refs: vec!["service:overwatch", "service:overgate"],
            incident_hook_refs: incident_hook_refs(ADMIN_ROUTE_IDEMPOTENCY_EXPIRE, &trace_id),
            operator_runbook_steps: idempotency_runbook_steps(),
            client_response_shape: ClientResponseShape::new(
                ADMIN_ROUTE_IDEMPOTENCY_EXPIRE,
                "overgate.admin_idempotency_expire_phase9",
                vec![
                    "visibility",
                    "idempotency_records",
                    "audit_hook_ref",
                    "redaction_state",
                ],
            ),
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
    let precheck_summary = state.prechecks.limit_summary(&operator.tenant_id);
    Ok(Json(ApiResponse::new_with_schema_version(
        PHASE9_RESPONSE_SCHEMA_VERSION,
        trace_id.clone(),
        "ok",
        "overgate.admin_rate_limits_phase9",
        AdminLookupData {
            route: ADMIN_ROUTE_RATE_LIMITS,
            requested_ref: "rate_limits:phase9_operator_view".to_owned(),
            operator_tenant: operator.tenant_id,
            operator_role: operator.role,
            operator_admission: operator.admission,
            visibility: "signed_operator_tenant_scoped_redacted_phase9",
            audit_hook_ref,
            idempotency_records: Vec::new(),
            rate_limit_buckets: precheck_summary.buckets.clone(),
            quota_precheck_refs: precheck_summary.quota_precheck_refs.clone(),
            local_counter_refs: precheck_summary.local_counter_refs.clone(),
            grant_placeholder_refs: precheck_summary.grant_placeholder_refs.clone(),
            policy_decision_refs: precheck_summary.policy_decision_refs.clone(),
            phase6_precheck_summary: Some(precheck_summary),
            redaction_state: "budget_grant_and_tenant_private_values_redacted_phase9",
            dependency_refs: vec![
                "service:overwatch",
                "service:overguard",
                "service:overmeter",
                "service:oru_account_service",
            ],
            incident_hook_refs: incident_hook_refs(ADMIN_ROUTE_RATE_LIMITS, &trace_id),
            operator_runbook_steps: rate_limit_runbook_steps(),
            client_response_shape: ClientResponseShape::new(
                ADMIN_ROUTE_RATE_LIMITS,
                "overgate.admin_rate_limits_phase9",
                vec![
                    "rate_limit_buckets",
                    "quota_precheck_refs",
                    "local_counter_refs",
                    "grant_placeholder_refs",
                    "denial_reason_distribution",
                    "redaction_state",
                ],
            ),
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
            "overgate_admin",
            "auth.operator_signature_required",
            "missing_operator_signature",
        ));
    };
    if !signature.starts_with("signature:") {
        return Err(admin_denial(
            StatusCode::UNAUTHORIZED,
            trace_id,
            "overgate_admin",
            "auth.operator_signature_malformed",
            "malformed_operator_signature",
        ));
    }

    let role = header_value(headers, OPERATOR_ROLE_HEADER).unwrap_or_default();
    if role != "operator" && role != "system_service" {
        return Err(admin_denial(
            StatusCode::FORBIDDEN,
            trace_id,
            "overgate_admin",
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
                "overgate_admin",
                "auth.cross_tenant_denied",
                "cross_tenant_denied",
            ));
        }
    }

    if !overwatch_ready(dependencies) {
        return Err(admin_denial(
            StatusCode::SERVICE_UNAVAILABLE,
            trace_id,
            "overgate_admin",
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
    route_scope: &'static str,
    reason_code: &'static str,
    denial: &'static str,
) -> (StatusCode, Json<ApiResponse<AdminDeniedData>>) {
    let audit_hook_ref = admin_audit_ref(route_scope, trace_id);
    (
        status,
        Json(ApiResponse::new_with_schema_version(
            PHASE9_RESPONSE_SCHEMA_VERSION,
            trace_id.to_owned(),
            "denied",
            reason_code,
            AdminDeniedData {
                route_scope,
                denial,
                audit_hook_ref,
                client_response_shape: ClientResponseShape::new(
                    route_scope,
                    reason_code,
                    vec!["route_scope", "denial", "audit_hook_ref"],
                ),
            },
        )),
    )
}

fn incident_hook_refs(route: &str, trace_id: &str) -> Vec<String> {
    vec![
        format!(
            "incident_hook:overgate:{}:{}",
            stable_short_token(&[route]),
            stable_short_token(&[trace_id])
        ),
        "overwatch.event:overgate.admin.phase9".to_owned(),
    ]
}

fn ingress_runbook_steps() -> Vec<&'static str> {
    vec![
        "verify_operator_signature_and_tenant_scope",
        "inspect_redacted_ingress_request_refs",
        "follow_audit_hook_ref_to_overwatch",
        "confirm_dependency_refs_before_downstream_triage",
    ]
}

fn idempotency_runbook_steps() -> Vec<&'static str> {
    vec![
        "verify_idempotency_scope_and_request_hash",
        "inspect_conflict_reason_and_retention_class",
        "refuse_active_disputed_incident_retry_or_finality_records",
        "emit_audit_evidence_for_any_expiration",
    ]
}

fn rate_limit_runbook_steps() -> Vec<&'static str> {
    vec![
        "inspect_rate_limit_bucket_without_raw_private_values",
        "review_quota_precheck_refs_and_local_counter_refs",
        "compare_denial_reason_distribution",
        "escalate_grant_or_budget_questions_to_accounting_services",
    ]
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
