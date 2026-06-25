use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use serde_json::{json, Value};

use crate::admission::{admit_command, AdmissionContext};
use crate::audit::{AuditGuardInput, Phase7AuditEvidence, Phase7AuditInput, Phase7DenialInput};
use crate::canonical::CanonicalRequestInput;
use crate::envelope::{trace_id_hint, CommandEnvelope};
use crate::errors::{ApiErrorData, OvergateError};
use crate::forwarding::{ForwardingInput, ForwardingOutcome};
use crate::idempotency::{
    CommandLookup, IdempotencyForwardingProjection, IdempotencyLimitSummary, IdempotencyOutcome,
    IdempotencyRecord, IdempotencyReservationInput, TraceSummary,
};
use crate::prechecks::{
    accepted_quota_refs, precheck_digest_ref, PrecheckInput, PrecheckLimitSummary, PrecheckOutcome,
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
pub const PHASE5_COMMAND_ACCEPTED_REASON: &str = "overgate.command_accepted_phase5";
pub const PHASE5_LIMITS_REASON: &str = "overgate.limits_phase5";
pub const PHASE6_RESPONSE_SCHEMA_VERSION: &str = "overgate.phase6.response.v0";
pub const PHASE6_COMMAND_ACCEPTED_REASON: &str = "overgate.command_accepted_phase6";
pub const PHASE7_RESPONSE_SCHEMA_VERSION: &str = "overgate.phase7.response.v0";
pub const PHASE8_RESPONSE_SCHEMA_VERSION: &str = "overgate.phase8.response.v0";
pub const PHASE8_COMMAND_ACCEPTED_REASON: &str = "overgate.command_accepted_phase8";
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
        Self::new_with_schema_version(
            PHASE6_RESPONSE_SCHEMA_VERSION,
            trace_id,
            status,
            reason_code,
            data,
        )
    }

    pub fn new_with_schema_version(
        schema_version: &'static str,
        trace_id: impl Into<String>,
        status: &'static str,
        reason_code: &'static str,
        data: T,
    ) -> Self {
        Self {
            schema_version,
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
    phase6_prechecks: PrecheckOutcome,
    accepted_command_quota_refs: Vec<String>,
    phase6_precheck_digest_ref: String,
    phase7_audit: Phase7AuditEvidence,
    phase8_forwarding: ForwardingOutcome,
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
    phase6_precheck_summary: PrecheckLimitSummary,
}

#[derive(Debug, Serialize)]
struct PolicyDryRunData {
    route: &'static str,
    policy_state: &'static str,
    mutation_allowed: bool,
    policy_ref: String,
    policy_check: crate::prechecks::PolicyCheckRecord,
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
        command_error_response(&state, &parsed.envelope, request_id.clone(), None, error)
    })?;
    let schema_validation = validate_command_envelope(&parsed.envelope).map_err(|error| {
        command_error_response(
            &state,
            &parsed.envelope,
            request_id.clone(),
            Some(admission.audit_context_ref.clone()),
            error,
        )
    })?;
    let retention = RetentionDecision::from_envelope(&parsed.envelope, &parsed.body_hash);
    let audit_ref = format!("audit:overgate:request_received:{request_id}");
    let mut idempotency = state
        .idempotency
        .reserve_or_replay(IdempotencyReservationInput::from_envelope(
            &parsed.envelope,
            &canonical_request,
            request_id.clone(),
            retention.clone(),
            audit_ref.clone(),
        ))
        .map_err(|error| {
            command_error_response(
                &state,
                &parsed.envelope,
                request_id.clone(),
                Some(admission.audit_context_ref.clone()),
                error,
            )
        })?;
    let phase6_prechecks = state
        .prechecks
        .precheck_command(PrecheckInput::from_parts(
            &parsed.envelope,
            &admission,
            &idempotency,
            body.len(),
        ))
        .map_err(|error| {
            command_error_response(
                &state,
                &parsed.envelope,
                request_id.clone(),
                Some(admission.audit_context_ref.clone()),
                error,
            )
        })?;
    let audit_decision = state
        .audit
        .guard_before_acceptance(AuditGuardInput {
            command_type: parsed.envelope.command_type.clone(),
            command_class: phase6_prechecks.command_class.clone(),
            request_hash_ref: parsed.envelope.request_hash.clone(),
            payload_hash_ref: parsed.envelope.payload_hash.clone(),
            trace_id: parsed.envelope.trace_id.clone(),
            overwatch_ready: state.dependencies.dependency_ready("overwatch"),
        })
        .map_err(|error| {
            command_error_response(
                &state,
                &parsed.envelope,
                request_id.clone(),
                Some(admission.audit_context_ref.clone()),
                error,
            )
        })?;
    let accepted_command_quota_refs = accepted_quota_refs(&phase6_prechecks);
    let phase6_precheck_digest_ref = precheck_digest_ref(&phase6_prechecks);
    let phase7_audit = state.audit.record_acceptance(Phase7AuditInput::from_parts(
        &parsed.envelope,
        request_id.clone(),
        admission.audit_context_ref.clone(),
        idempotency.record.record_id.clone(),
        idempotency.reason_code,
        idempotency.record.forwarding_state,
        audit_decision,
        phase6_prechecks.clone(),
    ));
    let phase7_audit_refs = phase7_audit
        .ordered_events
        .iter()
        .map(|event| event.event_ref.clone())
        .collect::<Vec<_>>();
    let phase8_forwarding = state
        .forwarding
        .forward_after_acceptance(ForwardingInput::from_parts(
            &parsed.envelope,
            request_id.clone(),
            &idempotency,
            &phase6_prechecks,
            phase7_audit_refs,
        ))
        .map_err(|error| {
            command_error_response(
                &state,
                &parsed.envelope,
                request_id.clone(),
                Some(admission.audit_context_ref.clone()),
                error,
            )
        })?;
    if let Some(projected_record) =
        state
            .idempotency
            .apply_forwarding_projection(IdempotencyForwardingProjection {
                record_id: idempotency.record.record_id.clone(),
                current_state: phase8_forwarding.status_projection.current_state,
                forwarding_state: phase8_forwarding.status_projection.forwarding_state,
                audit_refs: phase8_forwarding.record.audit_refs.clone(),
            })
    {
        idempotency.record = projected_record;
    }
    let (http_status, response_status, reason_code) = if idempotency.replayed {
        (StatusCode::OK, "ok", "overgate.idempotency_replayed")
    } else if phase8_forwarding.outcome_state == "failed_after_acceptance_phase8" {
        (
            StatusCode::ACCEPTED,
            "accepted",
            "overgate.forwarding_failed_after_acceptance",
        )
    } else {
        (
            StatusCode::ACCEPTED,
            "accepted",
            PHASE8_COMMAND_ACCEPTED_REASON,
        )
    };

    Ok((
        http_status,
        Json(ApiResponse::new_with_schema_version(
            PHASE8_RESPONSE_SCHEMA_VERSION,
            parsed.envelope.trace_id.clone(),
            response_status,
            reason_code,
            CommandAcceptedData {
                route: ROUTE_SUBMIT_COMMAND,
                request_id: request_id.clone(),
                command_id: parsed.envelope.command_id.clone(),
                audit_ref,
                forwarding_state: phase8_forwarding.status_projection.forwarding_state,
                payload_hash_ref: parsed.envelope.payload_hash.clone(),
                request_hash_ref: parsed.envelope.request_hash.clone(),
                body_hash_ref: parsed.body_hash.clone(),
                schema_validation,
                canonical_request,
                admission,
                retention,
                idempotency,
                phase6_prechecks,
                accepted_command_quota_refs,
                phase6_precheck_digest_ref,
                phase7_audit,
                phase8_forwarding,
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
    let phase6_precheck_summary = _state.prechecks.limit_summary(&tenant_scope);
    Json(ApiResponse::new(
        trace_id(&headers, "trace_overgate_phase6_limits"),
        "ok",
        "overgate.limits_phase6",
        LimitsData {
            route: ROUTE_LIMITS,
            tenant_scope: tenant_scope.clone(),
            rate_limit_refs: phase6_precheck_summary
                .buckets
                .iter()
                .map(|bucket| bucket.bucket_id.clone())
                .collect(),
            quota_precheck_refs: phase6_precheck_summary.quota_precheck_refs.clone(),
            idempotency_summary,
            phase6_precheck_summary,
        },
    ))
}

async fn policy_dry_run(
    State(state): State<OvergateState>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<ApiResponse<PolicyDryRunData>>) {
    let trace_id = trace_id(&headers, "trace_overgate_phase6_policy_dry_run");
    let tenant_scope =
        header_value(&headers, TENANT_HEADER).unwrap_or_else(|| "tenant:local:test".to_owned());
    let policy_check = state
        .prechecks
        .policy_dry_run(&tenant_scope, &payload, &trace_id);
    (
        StatusCode::ACCEPTED,
        Json(ApiResponse::new(
            trace_id,
            "accepted",
            "overgate.policy_dry_run_phase6",
            PolicyDryRunData {
                route: ROUTE_POLICY_DRY_RUN,
                policy_state: policy_check.handoff_state,
                mutation_allowed: policy_check.allowed,
                policy_ref: policy_check.decision_ref.clone(),
                policy_check,
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

fn command_error_response(
    state: &OvergateState,
    envelope: &CommandEnvelope,
    request_id: String,
    audit_context_ref: Option<String>,
    error: OvergateError,
) -> (StatusCode, Json<ApiResponse<ApiErrorData>>) {
    let audit_context_ref = audit_context_ref.unwrap_or_else(|| {
        format!(
            "audit_context:overgate:phase7:denied:{}",
            stable_short_token(&[envelope.command_type.as_str(), error.reason_code])
        )
    });
    let denial_evidence = state.audit.record_denial(Phase7DenialInput::from_error(
        envelope,
        request_id.clone(),
        audit_context_ref,
        &error,
    ));
    let audit_refs = denial_evidence
        .ordered_events
        .iter()
        .map(|event| event.event_ref.clone())
        .collect::<Vec<_>>();
    api_error_response_with_schema(
        PHASE8_RESPONSE_SCHEMA_VERSION,
        envelope.trace_id.clone(),
        Some(request_id),
        error.with_additional_client_denial_refs(audit_refs),
    )
}

fn api_error_response(
    trace_id: String,
    request_id: Option<String>,
    error: OvergateError,
) -> (StatusCode, Json<ApiResponse<ApiErrorData>>) {
    api_error_response_with_schema(PHASE6_RESPONSE_SCHEMA_VERSION, trace_id, request_id, error)
}

fn api_error_response_with_schema(
    schema_version: &'static str,
    trace_id: String,
    request_id: Option<String>,
    error: OvergateError,
) -> (StatusCode, Json<ApiResponse<ApiErrorData>>) {
    let status = error.status;
    (
        status,
        Json(ApiResponse::new_with_schema_version(
            schema_version,
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
    use crate::audit::AuditStore;
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
        assert_eq!(body["schema_version"], PHASE8_RESPONSE_SCHEMA_VERSION);
        assert_eq!(body["reason_code"], PHASE8_COMMAND_ACCEPTED_REASON);
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
        assert_eq!(
            body["data"]["phase6_prechecks"]["precheck_state"],
            "prechecked_before_forwarding_phase6"
        );
        assert_eq!(
            body["data"]["phase6_prechecks"]["quota_precheck"]["no_balance_mutation"],
            true
        );
        assert_eq!(
            body["data"]["phase6_prechecks"]["quota_precheck"]["no_seal_ledger_entry"],
            true
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
            body["data"]["forwarding_state"],
            "synchronous_completed_phase8"
        );
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
        assert_eq!(body["reason_code"], PHASE8_COMMAND_ACCEPTED_REASON);
        assert_eq!(body["data"]["route"], ROUTE_SUBMIT_COMMAND);
        assert_eq!(
            body["data"]["forwarding_state"],
            "synchronous_completed_phase8"
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
        assert_eq!(first_body["reason_code"], PHASE8_COMMAND_ACCEPTED_REASON);
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
        assert_eq!(
            conflict_body["schema_version"],
            PHASE8_RESPONSE_SCHEMA_VERSION
        );
        assert!(conflict_body["data"]["client_denial_refs"]
            .as_array()
            .expect("conflict denial refs should be present")
            .iter()
            .any(|value| value
                .as_str()
                .unwrap_or_default()
                .contains("overgate_idempotency_conflict")));
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
        assert_eq!(body["data"]["admission_state"], "completed_synchronously");
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
        assert_eq!(body["reason_code"], "overgate.limits_phase6");
        assert_eq!(
            body["data"]["idempotency_summary"]["visible_record_count"],
            1
        );
        assert_eq!(body["data"]["phase6_precheck_summary"]["bucket_count"], 1);

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
    async fn phase6_rate_limits_deny_before_forwarding_and_reset_by_window() {
        let app = OvergateService::default().router();
        for index in 0..2 {
            let mut envelope = valid_command_envelope(&format!("trace_phase6_rate_{index}"));
            envelope["command_id"] = json!(format!("command:overgate:phase6:rate:{index}"));
            envelope["command_type"] = json!("overgate.phase6.tenant.update");
            envelope["idempotency_key"] = json!(format!("idem:overgate:phase6:rate:{index}"));
            envelope["request_hash"] = json!(format!("hash:fixture:phase6_rate_request_{index}"));
            envelope["payload_hash"] = json!(format!("hash:fixture:phase6_rate_payload_{index}"));
            envelope["timestamp"] = json!("2026-06-25T00:10:00Z");
            let response = app
                .clone()
                .oneshot(command_post("/v1/commands", envelope))
                .await
                .expect("rate command should respond");
            assert_eq!(response.status(), StatusCode::ACCEPTED);
            let body = body_json(response).await;
            assert_eq!(body["reason_code"], PHASE8_COMMAND_ACCEPTED_REASON);
            assert_eq!(
                body["data"]["phase6_prechecks"]["rate_limit"]["reason_code"],
                "overgate.rate_limit_allowed_phase6"
            );
        }

        let mut exhausted = valid_command_envelope("trace_phase6_rate_exhausted");
        exhausted["command_id"] = json!("command:overgate:phase6:rate:exhausted");
        exhausted["command_type"] = json!("overgate.phase6.tenant.update");
        exhausted["idempotency_key"] = json!("idem:overgate:phase6:rate:exhausted");
        exhausted["request_hash"] = json!("hash:fixture:phase6_rate_request_exhausted");
        exhausted["payload_hash"] = json!("hash:fixture:phase6_rate_payload_exhausted");
        exhausted["timestamp"] = json!("2026-06-25T00:20:00Z");
        let response = app
            .clone()
            .oneshot(command_post("/v1/commands", exhausted))
            .await
            .expect("exhausted rate command should respond");
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        let body = body_json(response).await;
        assert_eq!(body["reason_code"], "overgate.rate_limited");
        assert!(body["data"]["client_denial_refs"]
            .as_array()
            .expect("client denial refs should be present")
            .iter()
            .any(|value| value
                .as_str()
                .unwrap_or_default()
                .starts_with("rate_limit_reset:overgate:phase6:")));

        let mut reset = valid_command_envelope("trace_phase6_rate_reset");
        reset["command_id"] = json!("command:overgate:phase6:rate:reset");
        reset["command_type"] = json!("overgate.phase6.tenant.update");
        reset["idempotency_key"] = json!("idem:overgate:phase6:rate:reset");
        reset["request_hash"] = json!("hash:fixture:phase6_rate_request_reset");
        reset["payload_hash"] = json!("hash:fixture:phase6_rate_payload_reset");
        reset["timestamp"] = json!("2026-06-25T01:00:00Z");
        let response = app
            .clone()
            .oneshot(command_post("/v1/commands", reset))
            .await
            .expect("reset-window rate command should respond");
        assert_eq!(response.status(), StatusCode::ACCEPTED);

        let admin_view = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/v1/admin/rate-limits")
                    .header(
                        crate::admin::OPERATOR_SIGNATURE_HEADER,
                        "signature:fixture:operator",
                    )
                    .header(crate::admin::OPERATOR_ROLE_HEADER, "operator")
                    .body(Body::empty())
                    .expect("admin rate-limit request should build"),
            )
            .await
            .expect("admin rate-limit route should respond");
        assert_eq!(admin_view.status(), StatusCode::OK);
        let body = body_json(admin_view).await;
        assert_eq!(body["reason_code"], "overgate.admin_rate_limits_phase6");
        assert!(
            body["data"]["rate_limit_buckets"]
                .as_array()
                .expect("rate buckets should be visible")
                .len()
                >= 2
        );
    }

    #[tokio::test]
    async fn phase6_idempotency_replays_reuse_prechecks_without_recounting_rate_limits() {
        let app = OvergateService::default().router();
        let mut original = valid_command_envelope("trace_phase6_replay_original");
        original["command_id"] = json!("command:overgate:phase6:replay:original");
        original["command_type"] = json!("overgate.phase6.tenant.update");
        original["idempotency_key"] = json!("idem:overgate:phase6:replay");
        original["request_hash"] = json!("hash:fixture:phase6_replay_request");
        original["payload_hash"] = json!("hash:fixture:phase6_replay_payload");
        original["timestamp"] = json!("2026-06-25T02:00:00Z");

        let response = app
            .clone()
            .oneshot(command_post("/v1/commands", original.clone()))
            .await
            .expect("original replay command should respond");
        assert_eq!(response.status(), StatusCode::ACCEPTED);

        let replay = app
            .clone()
            .oneshot(command_post("/v1/commands", original))
            .await
            .expect("idempotency replay command should respond");
        assert_eq!(replay.status(), StatusCode::OK);
        let body = body_json(replay).await;
        assert_eq!(body["reason_code"], "overgate.idempotency_replayed");
        assert_eq!(
            body["data"]["phase6_prechecks"]["rate_limit"]["consumed"],
            1
        );

        let mut second_real_command = valid_command_envelope("trace_phase6_replay_second_real");
        second_real_command["command_id"] = json!("command:overgate:phase6:replay:second_real");
        second_real_command["command_type"] = json!("overgate.phase6.tenant.update");
        second_real_command["idempotency_key"] = json!("idem:overgate:phase6:replay_second_real");
        second_real_command["request_hash"] = json!("hash:fixture:phase6_replay_second_request");
        second_real_command["payload_hash"] = json!("hash:fixture:phase6_replay_second_payload");
        second_real_command["timestamp"] = json!("2026-06-25T02:00:00Z");
        let response = app
            .clone()
            .oneshot(command_post("/v1/commands", second_real_command))
            .await
            .expect("second real command should respond");
        assert_eq!(response.status(), StatusCode::ACCEPTED);

        let mut third_real_command = valid_command_envelope("trace_phase6_replay_third_real");
        third_real_command["command_id"] = json!("command:overgate:phase6:replay:third_real");
        third_real_command["command_type"] = json!("overgate.phase6.tenant.update");
        third_real_command["idempotency_key"] = json!("idem:overgate:phase6:replay_third_real");
        third_real_command["request_hash"] = json!("hash:fixture:phase6_replay_third_request");
        third_real_command["payload_hash"] = json!("hash:fixture:phase6_replay_third_payload");
        third_real_command["timestamp"] = json!("2026-06-25T02:00:00Z");
        let response = app
            .oneshot(command_post("/v1/commands", third_real_command))
            .await
            .expect("third real command should respond");
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn phase6_quota_policy_and_dry_run_surfaces_are_structured() {
        let app = OvergateService::default().router();
        let mut quota_denied = valid_command_envelope("trace_phase6_quota_denied");
        quota_denied["command_id"] = json!("command:overgate:phase6:quota:denied");
        quota_denied["command_type"] = json!("overgate.phase6.quota.denied");
        quota_denied["idempotency_key"] = json!("idem:overgate:phase6:quota_denied");
        quota_denied["request_hash"] = json!("hash:fixture:phase6_quota_denied_request");
        quota_denied["payload_hash"] = json!("hash:fixture:phase6_quota_denied_payload");
        let response = app
            .clone()
            .oneshot(command_post("/v1/commands", quota_denied))
            .await
            .expect("quota-denied command should respond");
        assert_eq!(response.status(), StatusCode::PAYMENT_REQUIRED);
        let body = body_json(response).await;
        assert_eq!(body["reason_code"], "overgate.quota_precheck_denied");
        assert!(body["data"]["client_denial_refs"]
            .as_array()
            .expect("quota refs should be present")
            .iter()
            .any(|value| value
                .as_str()
                .unwrap_or_default()
                .starts_with("budget:overgate:phase6:")));

        let mut policy_denied = valid_command_envelope("trace_phase6_policy_denied");
        policy_denied["command_id"] = json!("command:overgate:phase6:policy:denied");
        policy_denied["command_type"] = json!("overgate.phase6.policy.deny");
        policy_denied["idempotency_key"] = json!("idem:overgate:phase6:policy_denied");
        policy_denied["request_hash"] = json!("hash:fixture:phase6_policy_denied_request");
        policy_denied["payload_hash"] = json!("hash:fixture:phase6_policy_denied_payload");
        let response = app
            .clone()
            .oneshot(command_post("/v1/commands", policy_denied))
            .await
            .expect("policy-denied command should respond");
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        let body = body_json(response).await;
        assert_eq!(body["reason_code"], "overgate.policy_denied");
        assert_eq!(body["data"]["dependency_name"], "overguard");
        assert!(body["data"]["client_denial_refs"]
            .as_array()
            .expect("policy refs should be present")
            .iter()
            .any(|value| value
                .as_str()
                .unwrap_or_default()
                .starts_with("policy_decision:overguard:phase6:")));

        let dry_run = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/policy/dry-run")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "command_type": "overgate.phase6.policy.evaluate",
                            "simulate_decision": "deny"
                        })
                        .to_string(),
                    ))
                    .expect("policy dry-run request should build"),
            )
            .await
            .expect("policy dry-run route should respond");
        assert_eq!(dry_run.status(), StatusCode::ACCEPTED);
        let body = body_json(dry_run).await;
        assert_eq!(body["reason_code"], "overgate.policy_dry_run_phase6");
        assert_eq!(body["data"]["mutation_allowed"], false);
        assert_eq!(
            body["data"]["policy_check"]["tenant_id"],
            "tenant:local:test"
        );
        assert_eq!(
            body["data"]["policy_check"]["stored_policy_truth_in_overgate"],
            false
        );
    }

    #[tokio::test]
    async fn phase6_policy_summaries_are_tenant_scoped() {
        let app = OvergateService::default().router();
        for (tenant, trace) in [
            ("tenant:phase6:alpha", "trace_phase6_policy_alpha"),
            ("tenant:phase6:beta", "trace_phase6_policy_beta"),
        ] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/v1/policy/dry-run")
                        .header(CONTENT_TYPE, "application/json")
                        .header(TENANT_HEADER, tenant)
                        .header(TRACE_HEADER, trace)
                        .body(Body::from(
                            json!({
                                "command_type": "overgate.phase6.policy.evaluate",
                                "simulate_decision": "allow"
                            })
                            .to_string(),
                        ))
                        .expect("tenant policy dry-run request should build"),
                )
                .await
                .expect("tenant policy dry-run should respond");
            assert_eq!(response.status(), StatusCode::ACCEPTED);
        }

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/v1/limits")
                    .header(TENANT_HEADER, "tenant:phase6:alpha")
                    .body(Body::empty())
                    .expect("tenant limits request should build"),
            )
            .await
            .expect("tenant limits should respond");
        assert_eq!(response.status(), StatusCode::OK);
        let body = body_json(response).await;
        let policy_refs = body["data"]["phase6_precheck_summary"]["policy_decision_refs"]
            .as_array()
            .expect("policy refs should be an array");
        assert_eq!(policy_refs.len(), 1);
    }

    #[tokio::test]
    async fn phase6_command_class_matrix_covers_required_classes() {
        assert!(crate::prechecks::validate_command_class_matrix());
        let classes = crate::prechecks::command_class_matrix();
        let names = classes
            .iter()
            .map(|entry| entry.command_class)
            .collect::<std::collections::HashSet<_>>();
        for expected in [
            "low_risk_read",
            "phase1_control_plane_mutation",
            "queue_producing_workload",
            "policy_heavy",
            "accounting_affecting",
            "storage_namespace",
            "native_app_side_effect",
            "admin",
            "break_glass",
        ] {
            assert!(names.contains(expected), "missing {expected}");
        }
    }

    #[tokio::test]
    async fn phase7_accepted_command_returns_overwatch_compatible_audit_evidence() {
        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_phase7_audit_response");
        envelope["command_id"] = json!("command:overgate:phase7:audit_response");
        envelope["command_type"] = json!("overgate.phase7.tenant.profile_update");
        envelope["idempotency_key"] = json!("idem:overgate:phase7:audit_response");
        envelope["request_hash"] = json!("hash:fixture:phase7_audit_response_request");
        envelope["payload_hash"] = json!("hash:fixture:phase7_audit_response_payload");
        envelope["timestamp"] = json!("2026-06-25T03:00:00Z");

        let response = app
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("phase7 audit response should respond");
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let body = body_json(response).await;
        assert_eq!(body["schema_version"], PHASE8_RESPONSE_SCHEMA_VERSION);
        assert_eq!(body["reason_code"], PHASE8_COMMAND_ACCEPTED_REASON);
        assert_eq!(
            body["data"]["phase7_audit"]["evidence_state"],
            "phase7_audit_evidence_recorded"
        );
        assert_eq!(
            body["data"]["phase7_audit"]["overwatch_client_ref"],
            "overwatch.audit.v0"
        );
        let event_types = body["data"]["phase7_audit"]["ordered_events"]
            .as_array()
            .expect("ordered audit events should be present")
            .iter()
            .map(|event| event["event_type"].as_str().expect("event type"))
            .collect::<Vec<_>>();
        assert_eq!(
            event_types,
            vec![
                "overgate.request_received",
                "overgate.signature_verified",
                "overgate.idempotency_reserved",
                "overgate.command_accepted",
            ]
        );
        assert_eq!(
            body["data"]["phase7_audit"]["emergency_wal"]["enabled"],
            false
        );
        assert_eq!(
            body["data"]["phase7_audit"]["metrics"]["labels"]["private_data_in_labels"],
            false
        );
        assert_eq!(
            body["data"]["phase7_audit"]["metrics"]["labels"]["secrets_in_labels"],
            false
        );
        assert_eq!(
            body["data"]["phase7_audit"]["grid_operations"]["system_service_workload_class"],
            "system_service_workload:overgate:phase7"
        );
        assert_eq!(
            body["data"]["phase7_audit"]["raw_private_payload_stored"],
            false
        );
        assert_eq!(body["data"]["phase7_audit"]["raw_secret_stored"], false);
    }

    #[tokio::test]
    async fn phase7_overwatch_unavailable_fails_closed_for_high_risk_commands() {
        let dependencies = DependencyMatrix::default()
            .with_dependency_state("overwatch", DependencyState::Unavailable);
        let service = OvergateService::with_dependencies(OvergateConfig::default(), dependencies);
        let mut envelope = valid_command_envelope("trace_phase7_fail_closed_route");
        envelope["command_id"] = json!("command:overgate:phase7:fail_closed");
        envelope["command_type"] = json!("overgate.phase7.accounting.ledger.transfer");
        envelope["idempotency_key"] = json!("idem:overgate:phase7:fail_closed");
        envelope["request_hash"] = json!("hash:fixture:phase7_fail_closed_request");
        envelope["payload_hash"] = json!("hash:fixture:phase7_fail_closed_payload");

        let response = service
            .router()
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("phase7 fail-closed route should respond");
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = body_json(response).await;
        assert_eq!(body["schema_version"], PHASE8_RESPONSE_SCHEMA_VERSION);
        assert_eq!(body["reason_code"], "overgate.audit_fail_closed");
        assert_eq!(body["data"]["dependency_name"], "overwatch");
        assert_eq!(body["data"]["retryability"], "retry_after");
        assert!(body["data"]["client_denial_refs"]
            .as_array()
            .expect("client denial refs should be present")
            .iter()
            .any(|value| value == "dependency:overwatch"));
        assert!(body["data"]["client_denial_refs"]
            .as_array()
            .expect("client denial refs should be present")
            .iter()
            .any(|value| value
                .as_str()
                .unwrap_or_default()
                .contains("overgate_audit_fail_closed")));
    }

    #[tokio::test]
    async fn phase7_denied_commands_return_overwatch_compatible_audit_refs() {
        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_phase7_denial_audit");
        envelope["command_id"] = json!("command:overgate:phase7:denial_audit");
        envelope["command_type"] = json!("overgate.phase7.tenant.profile_update");
        envelope["idempotency_key"] = json!("idem:overgate:phase7:denial_audit");
        envelope["request_hash"] = json!("hash:fixture:phase7_denial_request");
        envelope["payload_hash"] = json!("hash:fixture:phase7_denial_payload");

        let first = app
            .clone()
            .oneshot(command_post("/v1/commands", envelope.clone()))
            .await
            .expect("first phase7 denial setup request should respond");
        assert_eq!(first.status(), StatusCode::ACCEPTED);

        envelope["request_hash"] = json!("hash:fixture:phase7_denial_conflict");
        let conflict = app
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("phase7 denial audit request should respond");
        assert_eq!(conflict.status(), StatusCode::CONFLICT);
        let body = body_json(conflict).await;
        assert_eq!(body["schema_version"], PHASE8_RESPONSE_SCHEMA_VERSION);
        assert_eq!(body["reason_code"], "overgate.idempotency_conflict");
        let refs = body["data"]["client_denial_refs"]
            .as_array()
            .expect("denial audit refs should be present");
        assert!(refs.iter().any(|value| value
            .as_str()
            .unwrap_or_default()
            .contains("overgate_request_received")));
        assert!(refs.iter().any(|value| value
            .as_str()
            .unwrap_or_default()
            .contains("overgate_signature_verified")));
        assert!(refs.iter().any(|value| value
            .as_str()
            .unwrap_or_default()
            .contains("overgate_idempotency_conflict")));
    }

    #[tokio::test]
    async fn phase7_emergency_wal_allows_low_risk_phase1_mutation_only() {
        let dependencies = DependencyMatrix::default()
            .with_dependency_state("overwatch", DependencyState::Unavailable);
        let service = OvergateService::with_dependencies_and_audit(
            OvergateConfig::default(),
            dependencies,
            AuditStore::with_emergency_wal_enabled(4),
        );
        let mut envelope = valid_command_envelope("trace_phase7_emergency_route");
        envelope["command_id"] = json!("command:overgate:phase7:emergency_wal");
        envelope["command_type"] = json!("overgate.phase7.tenant.profile_update");
        envelope["idempotency_key"] = json!("idem:overgate:phase7:emergency_wal");
        envelope["request_hash"] = json!("hash:fixture:phase7_emergency_request");
        envelope["payload_hash"] = json!("hash:fixture:phase7_emergency_payload");

        let response = service
            .router()
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("phase7 emergency WAL route should respond");
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let body = body_json(response).await;
        let wal = &body["data"]["phase7_audit"]["emergency_wal"];
        assert_eq!(wal["enabled"], true);
        assert_eq!(wal["degraded_mode"], true);
        assert_eq!(wal["entries"].as_array().expect("entries").len(), 4);
        assert_eq!(wal["hash_chain_verified"], true);
        assert_eq!(wal["fsync_before_side_effect"], true);
        assert_eq!(wal["replay_to_overwatch_required"], true);
        assert_eq!(
            wal["readiness_state"],
            "degraded_until_replayed_to_overwatch"
        );
        assert_eq!(wal["external_log_dependency"], "none_rust_owned_local_wal");
    }

    #[tokio::test]
    async fn phase8_synchronous_forwarding_completes_narrow_phase1_commands() {
        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_phase8_sync_forwarding");
        envelope["command_id"] = json!("command:overgate:phase8:sync");
        envelope["command_type"] = json!("overgate.phase8.tenant.profile_update");
        envelope["idempotency_key"] = json!("idem:overgate:phase8:sync");
        envelope["request_hash"] = json!("hash:fixture:phase8_sync_request");
        envelope["payload_hash"] = json!("hash:fixture:phase8_sync_payload");

        let response = app
            .clone()
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("phase8 sync command should respond");
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let body = body_json(response).await;
        assert_eq!(body["schema_version"], PHASE8_RESPONSE_SCHEMA_VERSION);
        assert_eq!(body["reason_code"], PHASE8_COMMAND_ACCEPTED_REASON);
        assert_eq!(
            body["data"]["forwarding_state"],
            "synchronous_completed_phase8"
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["outcome_state"],
            "synchronous_forwarding_completed_phase8"
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["target"]["owner_service"],
            "service:overtenant"
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["synchronous_completion"]
                ["completed_before_response"],
            true
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["synchronous_completion"]["execution_side_effect"],
            false
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["direct_downstream_state_write"],
            false
        );

        let status = app
            .oneshot(empty_get("/v1/commands/command:overgate:phase8:sync"))
            .await
            .expect("phase8 sync status should respond");
        assert_eq!(status.status(), StatusCode::OK);
        let status_body = body_json(status).await;
        assert_eq!(
            status_body["data"]["admission_state"],
            "completed_synchronously"
        );
        assert_eq!(
            status_body["data"]["forwarding_state"],
            "synchronous_completed_phase8"
        );
    }

    #[tokio::test]
    async fn phase8_overqueue_dispatch_records_durable_pending_work() {
        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_phase8_overqueue_dispatch");
        envelope["command_id"] = json!("command:overgate:phase8:queue");
        envelope["command_type"] = json!("overgate.phase8.queue.workload.submit");
        envelope["idempotency_key"] = json!("idem:overgate:phase8:queue");
        envelope["request_hash"] = json!("hash:fixture:phase8_queue_request");
        envelope["payload_hash"] = json!("hash:fixture:phase8_queue_payload");

        let response = app
            .clone()
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("phase8 queue command should respond");
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let body = body_json(response).await;
        assert_eq!(body["reason_code"], PHASE8_COMMAND_ACCEPTED_REASON);
        assert_eq!(
            body["data"]["phase8_forwarding"]["outcome_state"],
            "overqueue_dispatch_recorded_phase8"
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["target"]["dispatch_mode"],
            "overqueue_durable_dispatch_phase8"
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["overqueue_item"]["durable_state"],
            "durable_pending_work_phase8"
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["overqueue_item"]["native_overqueue_boundary"],
            true
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["status_projection"]["current_state"],
            "pending_overqueue_dispatch"
        );
        assert!(body["data"]["phase8_forwarding"]["record"]["audit_refs"]
            .as_array()
            .expect("audit refs")
            .iter()
            .any(|value| value
                .as_str()
                .unwrap_or_default()
                .contains("overgate_command_forwarded")));

        let status = app
            .oneshot(empty_get("/v1/commands/command:overgate:phase8:queue"))
            .await
            .expect("phase8 queue status should respond");
        let status_body = body_json(status).await;
        assert_eq!(
            status_body["data"]["admission_state"],
            "pending_overqueue_dispatch"
        );
        assert_eq!(
            status_body["data"]["forwarding_state"],
            "overqueue_pending_phase8"
        );
    }

    #[tokio::test]
    async fn phase8_target_registry_rejects_unregistered_targets() {
        assert!(crate::forwarding::validate_target_registry());
        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_phase8_missing_target");
        envelope["command_id"] = json!("command:overgate:phase8:missing_target");
        envelope["command_type"] = json!("overgate.phase8.unregistered.operation");
        envelope["idempotency_key"] = json!("idem:overgate:phase8:missing_target");
        envelope["request_hash"] = json!("hash:fixture:phase8_missing_target_request");
        envelope["payload_hash"] = json!("hash:fixture:phase8_missing_target_payload");

        let response = app
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("phase8 missing target should respond");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = body_json(response).await;
        assert_eq!(body["schema_version"], PHASE8_RESPONSE_SCHEMA_VERSION);
        assert_eq!(
            body["reason_code"],
            "overgate.forwarding_target_unregistered"
        );
        assert!(body["data"]["client_denial_refs"]
            .as_array()
            .expect("client refs")
            .iter()
            .any(|value| value == "forwarding_target_registry:overgate:phase8"));
    }

    #[tokio::test]
    async fn phase8_failed_after_acceptance_status_preserves_retry_projection() {
        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_phase8_downstream_unavailable");
        envelope["command_id"] = json!("command:overgate:phase8:downstream_unavailable");
        envelope["command_type"] = json!("overgate.phase8.queue.workload.downstream.unavailable");
        envelope["idempotency_key"] = json!("idem:overgate:phase8:downstream_unavailable");
        envelope["request_hash"] = json!("hash:fixture:phase8_downstream_unavailable_request");
        envelope["payload_hash"] = json!("hash:fixture:phase8_downstream_unavailable_payload");

        let response = app
            .clone()
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("phase8 failed-after-acceptance command should respond");
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let body = body_json(response).await;
        assert_eq!(
            body["reason_code"],
            "overgate.forwarding_failed_after_acceptance"
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["outcome_state"],
            "failed_after_acceptance_phase8"
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["status_projection"]["current_state"],
            "failed_after_acceptance"
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["status_projection"]["forwarding_state"],
            "retry_scheduled_phase8"
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["retry"]["retryable"],
            true
        );
        assert_eq!(
            body["data"]["phase8_forwarding"]["retry"]["safe_overqueue_retry"],
            true
        );
        assert!(body["data"]["phase8_forwarding"]["record"]["audit_refs"]
            .as_array()
            .expect("audit refs")
            .iter()
            .any(|value| value
                .as_str()
                .unwrap_or_default()
                .contains("overgate_forwarding_failed")));

        let status = app
            .oneshot(empty_get(
                "/v1/commands/command:overgate:phase8:downstream_unavailable",
            ))
            .await
            .expect("phase8 failure status should respond");
        let status_body = body_json(status).await;
        assert_eq!(
            status_body["data"]["admission_state"],
            "failed_after_acceptance"
        );
        assert_eq!(
            status_body["data"]["forwarding_state"],
            "retry_scheduled_phase8"
        );
    }

    #[tokio::test]
    async fn phase8_product_clients_cannot_bypass_overgate_forwarding_contracts() {
        let app = OvergateService::default().router();
        let mut envelope = valid_command_envelope("trace_phase8_product_bypass");
        envelope["command_id"] = json!("command:overgate:phase8:product_bypass");
        envelope["command_type"] = json!("overgate.phase8.product.bypass_internal_api");
        envelope["idempotency_key"] = json!("idem:overgate:phase8:product_bypass");
        envelope["request_hash"] = json!("hash:fixture:phase8_product_bypass_request");
        envelope["payload_hash"] = json!("hash:fixture:phase8_product_bypass_payload");

        let response = app
            .oneshot(command_post("/v1/commands", envelope))
            .await
            .expect("phase8 product bypass denial should respond");
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        let body = body_json(response).await;
        assert_eq!(body["schema_version"], PHASE8_RESPONSE_SCHEMA_VERSION);
        assert_eq!(body["reason_code"], "overgate.product_client_bypass_denied");
        assert!(body["data"]["client_denial_refs"]
            .as_array()
            .expect("client refs")
            .iter()
            .any(|value| value == "product_client_flows:overgate:phase8"));
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
        assert_eq!(body["reason_code"], PHASE8_COMMAND_ACCEPTED_REASON);
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
