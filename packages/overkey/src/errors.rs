use axum::http::StatusCode;
use serde::Serialize;

use crate::repository::RepositoryError;

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

    pub fn repository_rejected(trace_id: impl Into<String>, error: RepositoryError) -> Self {
        match error {
            RepositoryError::DuplicateCredential => Self {
                http_status: StatusCode::CONFLICT,
                trace_id: trace_id.into(),
                reason_code: "overkey.duplicate_credential_rejected",
                data: ApiErrorData {
                    error_code: "duplicate_credential_rejected",
                    message: "credential metadata append would overwrite an existing credential",
                    retryability: Retryability::Terminal,
                    field_refs: vec!["credential_record.credential_id"],
                },
            },
            RepositoryError::DuplicateActiveKey => Self {
                http_status: StatusCode::CONFLICT,
                trace_id: trace_id.into(),
                reason_code: "overkey.duplicate_active_key_rejected",
                data: ApiErrorData {
                    error_code: "duplicate_active_key_rejected",
                    message: "an active key with this tenant-scoped key id already exists",
                    retryability: Retryability::Terminal,
                    field_refs: vec!["credential_record.key_id"],
                },
            },
            RepositoryError::CredentialNotFound => Self {
                http_status: StatusCode::NOT_FOUND,
                trace_id: trace_id.into(),
                reason_code: "overkey.credential_not_found",
                data: ApiErrorData {
                    error_code: "credential_not_found",
                    message: "credential metadata was not found in the tenant scope",
                    retryability: Retryability::Terminal,
                    field_refs: vec!["credential_id", "header:x-overrid-tenant-id"],
                },
            },
            RepositoryError::InvalidStatusTransition => Self {
                http_status: StatusCode::CONFLICT,
                trace_id: trace_id.into(),
                reason_code: "overkey.invalid_lifecycle_transition",
                data: ApiErrorData {
                    error_code: "invalid_lifecycle_transition",
                    message: "credential lifecycle transition is not allowed",
                    retryability: Retryability::Terminal,
                    field_refs: vec!["credential_record.status"],
                },
            },
            RepositoryError::BroadServiceAccountScope => Self {
                http_status: StatusCode::BAD_REQUEST,
                trace_id: trace_id.into(),
                reason_code: "overkey.broad_service_account_scope_rejected",
                data: ApiErrorData {
                    error_code: "broad_service_account_scope_rejected",
                    message: "service-account credentials require narrow service and command class scopes",
                    retryability: Retryability::Terminal,
                    field_refs: vec![
                        "service_account_key.allowed_services",
                        "service_account_key.allowed_command_classes",
                    ],
                },
            },
            RepositoryError::UnsignedServiceAccountCall => Self {
                http_status: StatusCode::FORBIDDEN,
                trace_id: trace_id.into(),
                reason_code: "overkey.service_account_signature_required",
                data: ApiErrorData {
                    error_code: "service_account_signature_required",
                    message: "service-account credential enrollment requires a signed service-account call",
                    retryability: Retryability::Terminal,
                    field_refs: vec![
                        "header:x-overrid-service-account",
                        "header:x-overrid-service-signature",
                    ],
                },
            },
            RepositoryError::RawSecretMaterial => Self {
                http_status: StatusCode::BAD_REQUEST,
                trace_id: trace_id.into(),
                reason_code: "overkey.raw_secret_material_rejected",
                data: ApiErrorData {
                    error_code: "raw_secret_material_rejected",
                    message: "credential metadata cannot include direct key material",
                    retryability: Retryability::Terminal,
                    field_refs: vec!["credential_record.secret_ref"],
                },
            },
            RepositoryError::DuplicateLifecycleRecord => Self {
                http_status: StatusCode::CONFLICT,
                trace_id: trace_id.into(),
                reason_code: "overkey.duplicate_lifecycle_record_rejected",
                data: ApiErrorData {
                    error_code: "duplicate_lifecycle_record_rejected",
                    message: "lifecycle record append would duplicate an existing record",
                    retryability: Retryability::Terminal,
                    field_refs: vec!["rotation_record.rotation_id", "revocation_record.revocation_id"],
                },
            },
        }
    }

    pub fn invalid_enrollment(
        trace_id: impl Into<String>,
        reason_code: &'static str,
        message: &'static str,
        field_refs: Vec<&'static str>,
    ) -> Self {
        Self {
            http_status: StatusCode::BAD_REQUEST,
            trace_id: trace_id.into(),
            reason_code,
            data: ApiErrorData {
                error_code: reason_code,
                message,
                retryability: Retryability::Terminal,
                field_refs,
            },
        }
    }
}
