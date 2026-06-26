use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Retryability {
    Retryable,
    Terminal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorData {
    pub error_code: &'static str,
    pub message: &'static str,
    pub retryability: Retryability,
    pub field_refs: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverkeyError {
    pub http_status: StatusCode,
    pub trace_id: String,
    pub reason_code: &'static str,
    pub data: ApiErrorData,
}

impl OverkeyError {
    pub fn missing_tenant(trace_id: impl Into<String>) -> Self {
        Self {
            http_status: StatusCode::BAD_REQUEST,
            trace_id: trace_id.into(),
            reason_code: "overkey.tenant_context_required",
            data: ApiErrorData {
                error_code: "tenant_context_required",
                message: "tenant context is required",
                retryability: Retryability::Terminal,
                field_refs: vec!["header:x-overrid-tenant-id"],
            },
        }
    }

    pub fn unsupported_media_type(trace_id: impl Into<String>) -> Self {
        Self {
            http_status: StatusCode::UNSUPPORTED_MEDIA_TYPE,
            trace_id: trace_id.into(),
            reason_code: "overkey.content_type_required",
            data: ApiErrorData {
                error_code: "content_type_required",
                message: "application/json content type is required",
                retryability: Retryability::Terminal,
                field_refs: vec!["header:content-type"],
            },
        }
    }

    pub fn missing_service_account(trace_id: impl Into<String>) -> Self {
        Self {
            http_status: StatusCode::FORBIDDEN,
            trace_id: trace_id.into(),
            reason_code: "auth.service_account_required",
            data: ApiErrorData {
                error_code: "service_account_required",
                message: "internal verification requires service account credentials",
                retryability: Retryability::Terminal,
                field_refs: vec![
                    "header:x-overrid-service-account",
                    "header:x-overrid-service-signature",
                ],
            },
        }
    }

    pub fn repository_rejected(trace_id: impl Into<String>) -> Self {
        Self {
            http_status: StatusCode::BAD_REQUEST,
            trace_id: trace_id.into(),
            reason_code: "overkey.raw_secret_material_rejected",
            data: ApiErrorData {
                error_code: "raw_secret_material_rejected",
                message: "credential metadata cannot include direct key material",
                retryability: Retryability::Terminal,
                field_refs: vec!["credential_record.secret_ref"],
            },
        }
    }
}
