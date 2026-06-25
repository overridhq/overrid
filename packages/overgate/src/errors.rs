use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Retryability {
    NotRetryable,
    SafeRetry,
    RetryAfter,
    OperatorReview,
}

impl Retryability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotRetryable => "not_retryable",
            Self::SafeRetry => "safe_retry",
            Self::RetryAfter => "retry_after",
            Self::OperatorReview => "operator_review",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OvergateError {
    pub status: StatusCode,
    pub reason_code: &'static str,
    pub retryability: Retryability,
    pub correction_fields: Vec<&'static str>,
    pub correction_hint: &'static str,
    pub dependency_name: Option<&'static str>,
    pub diagnostic: &'static str,
    pub client_denial_refs: Vec<String>,
}

impl OvergateError {
    pub fn wrong_content_type() -> Self {
        Self::new(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "schema.wrong_content_type",
            Retryability::NotRetryable,
            vec!["content_type"],
            "Send the command envelope as application/json.",
            "wrong_content_type",
        )
    }

    pub fn oversized() -> Self {
        Self::new(
            StatusCode::PAYLOAD_TOO_LARGE,
            "schema.payload_too_large",
            Retryability::NotRetryable,
            vec!["body"],
            "Send a smaller command envelope and move private payloads behind refs.",
            "command_envelope_too_large",
        )
    }

    pub fn malformed_json() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "schema.parse_malformed_payload",
            Retryability::NotRetryable,
            vec!["body"],
            "Send a well-formed JSON command envelope.",
            "malformed_json",
        )
    }

    pub fn missing_required_field(field: &'static str) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "schema.missing_required_field",
            Retryability::NotRetryable,
            vec![field],
            "Add the missing required command-envelope field.",
            "missing_required_field",
        )
    }

    pub fn unknown_sensitive_field() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "schema.unknown_sensitive_field",
            Retryability::NotRetryable,
            vec!["command_envelope"],
            "Remove fields that are not part of the strict command-envelope schema.",
            "unknown_sensitive_field",
        )
    }

    pub fn unsupported_schema_version() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "schema.unsupported_version",
            Retryability::NotRetryable,
            vec!["schema_version"],
            "Use the supported shared-schema package version for Overgate command envelopes.",
            "unsupported_schema_version",
        )
    }

    pub fn unsupported_canonicalization_version() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "schema.unsupported_version",
            Retryability::NotRetryable,
            vec!["signature_metadata.canonicalization_version"],
            "Use the Overgate canonicalization version for signed command envelopes.",
            "unsupported_canonicalization_version",
        )
    }

    pub fn unsupported_command_type() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "command.unsupported_type",
            Retryability::NotRetryable,
            vec!["command_type"],
            "Use an Overgate-supported command type.",
            "unsupported_command_type",
        )
    }

    pub fn wrong_privacy_class() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "schema.wrong_privacy_class",
            Retryability::NotRetryable,
            vec!["privacy_class"],
            "Use a private command privacy class such as tenant_private or system_service_only.",
            "wrong_privacy_class",
        )
    }

    pub fn malformed_ref(field: &'static str) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "schema.malformed_ref",
            Retryability::NotRetryable,
            vec![field],
            "Use a typed ref or hash ref with a stable prefix.",
            "malformed_ref",
        )
    }

    pub fn raw_secret_rejected() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "schema.raw_secret_rejected",
            Retryability::NotRetryable,
            vec!["payload_ref", "secret_ref"],
            "Replace raw secrets, private payloads, or key material with typed refs.",
            "raw_secret_rejected",
        )
    }

    pub fn dependency_unavailable(dependency_name: &'static str) -> Self {
        Self::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "dependency.schema_validation_unavailable",
            Retryability::RetryAfter,
            vec!["schema_validation"],
            "Retry after the shared schema validation dependency is available.",
            "schema_validation_dependency_unavailable",
        )
        .with_dependency(dependency_name)
    }

    pub fn credential_denied(
        reason_code: &'static str,
        field: &'static str,
        diagnostic: &'static str,
    ) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            reason_code,
            Retryability::NotRetryable,
            vec![field],
            "Use an active Overkey-lite credential and current signature metadata.",
            diagnostic,
        )
        .with_dependency("overkey_lite")
    }

    pub fn actor_denied(
        reason_code: &'static str,
        field: &'static str,
        diagnostic: &'static str,
    ) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            reason_code,
            Retryability::OperatorReview,
            vec![field],
            "Resolve the actor through Overpass before retrying the protected command.",
            diagnostic,
        )
        .with_dependency("overpass")
    }

    pub fn tenant_denied(
        reason_code: &'static str,
        field: &'static str,
        diagnostic: &'static str,
    ) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            reason_code,
            Retryability::OperatorReview,
            vec![field],
            "Use a tenant, role, membership, or delegated-access binding that authorizes this command.",
            diagnostic,
        )
        .with_dependency("overtenant")
    }

    pub fn service_account_denied(
        reason_code: &'static str,
        field: &'static str,
        diagnostic: &'static str,
    ) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            reason_code,
            Retryability::OperatorReview,
            vec![field],
            "Use a signed service-account or node-agent command with narrow scoped permission and audit context.",
            diagnostic,
        )
        .with_dependency("overtenant")
    }

    pub fn idempotency_conflict() -> Self {
        Self::new(
            StatusCode::CONFLICT,
            "overgate.idempotency_conflict",
            Retryability::NotRetryable,
            vec!["idempotency_key", "request_hash"],
            "Reuse an idempotency key only with the exact same request hash and credential context.",
            "idempotency_request_hash_conflict",
        )
    }

    pub fn status_not_found() -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            "overgate.status_not_found",
            Retryability::NotRetryable,
            vec!["command_id"],
            "Use a command id visible to this tenant after the command has reached Overgate.",
            "status_record_not_found",
        )
    }

    pub fn status_visibility_denied() -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            "overgate.status_visibility_denied",
            Retryability::OperatorReview,
            vec!["tenant_id", "command_id"],
            "Use tenant-scoped credentials that are authorized to read this command status.",
            "cross_tenant_status_denied",
        )
    }

    pub fn rate_limited(client_denial_refs: Vec<String>) -> Self {
        Self::new(
            StatusCode::TOO_MANY_REQUESTS,
            "overgate.rate_limited",
            Retryability::RetryAfter,
            vec!["rate_limit_bucket", "reset_ref"],
            "Wait for the deterministic rate-limit reset window or reduce command frequency.",
            "rate_limit_bucket_exhausted_phase6",
        )
        .with_client_denial_refs(client_denial_refs)
    }

    pub fn quota_precheck_denied(client_denial_refs: Vec<String>) -> Self {
        Self::new(
            StatusCode::PAYMENT_REQUIRED,
            "overgate.quota_precheck_denied",
            Retryability::OperatorReview,
            vec!["quota_precheck_ref", "budget_ref", "grant_ref"],
            "Resolve quota, budget, or grant eligibility before retrying this command.",
            "quota_precheck_denied_phase6_no_settlement",
        )
        .with_dependency("overmeter_or_oru_placeholder")
        .with_client_denial_refs(client_denial_refs)
    }

    pub fn policy_denied(client_denial_refs: Vec<String>) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            "overgate.policy_denied",
            Retryability::OperatorReview,
            vec!["policy_decision_ref", "policy_rule_ref"],
            "Use the Overguard policy decision ref to inspect the denial before retrying.",
            "overguard_policy_dry_run_denied_phase6",
        )
        .with_dependency("overguard")
        .with_client_denial_refs(client_denial_refs)
    }

    pub fn command_class_matrix_denied() -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "overgate.command_class_matrix_invalid",
            Retryability::NotRetryable,
            vec!["command_type"],
            "Use a command type with a complete Phase 6 command-class matrix entry.",
            "command_class_matrix_missing_required_gate",
        )
    }

    pub fn audit_fail_closed(client_denial_refs: Vec<String>) -> Self {
        Self::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "overgate.audit_fail_closed",
            Retryability::RetryAfter,
            vec!["overwatch", "emergency_audit_wal"],
            "Retry after Overwatch audit evidence is available, or use an explicitly configured low-risk emergency audit WAL path.",
            "overwatch_unavailable_fail_closed_phase7",
        )
        .with_dependency("overwatch")
        .with_client_denial_refs(client_denial_refs)
    }

    fn new(
        status: StatusCode,
        reason_code: &'static str,
        retryability: Retryability,
        correction_fields: Vec<&'static str>,
        correction_hint: &'static str,
        diagnostic: &'static str,
    ) -> Self {
        Self {
            status,
            reason_code,
            retryability,
            correction_fields,
            correction_hint,
            dependency_name: None,
            diagnostic,
            client_denial_refs: Vec::new(),
        }
    }

    fn with_dependency(mut self, dependency_name: &'static str) -> Self {
        self.dependency_name = Some(dependency_name);
        self
    }

    fn with_client_denial_refs(mut self, client_denial_refs: Vec<String>) -> Self {
        self.client_denial_refs = client_denial_refs;
        self
    }

    pub fn to_data(&self, request_id: Option<String>) -> ApiErrorData {
        ApiErrorData {
            request_id,
            reason_code: self.reason_code,
            retryability: self.retryability.as_str(),
            correction_fields: self.correction_fields.clone(),
            correction_hint: self.correction_hint,
            dependency_name: self.dependency_name,
            client_denial_refs: self.client_denial_refs.clone(),
            diagnostics: RedactedDiagnostics {
                privacy_class: "redacted_diagnostic",
                redacted: true,
                diagnostic: self.diagnostic,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiErrorData {
    pub request_id: Option<String>,
    pub reason_code: &'static str,
    pub retryability: &'static str,
    pub correction_fields: Vec<&'static str>,
    pub correction_hint: &'static str,
    pub dependency_name: Option<&'static str>,
    pub client_denial_refs: Vec<String>,
    pub diagnostics: RedactedDiagnostics,
}

#[derive(Debug, Clone, Serialize)]
pub struct RedactedDiagnostics {
    pub privacy_class: &'static str,
    pub redacted: bool,
    pub diagnostic: &'static str,
}
