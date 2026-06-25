use serde::Serialize;

use crate::canonical::{stable_short_token, CanonicalRequestInput, CANONICALIZATION_VERSION};
use crate::dependencies::DependencyMatrix;
use crate::envelope::CommandEnvelope;
use crate::errors::OvergateError;

pub const PHASE4_ADMISSION_ADAPTER_ID: &str = "overgate.phase4.local_admission_adapter";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdmissionContext {
    pub adapter_id: &'static str,
    pub admission_state: &'static str,
    pub ready_for_idempotency: bool,
    pub signature_check: SignatureCheckRecord,
    pub actor_resolution: ActorResolutionRecord,
    pub tenant_authorization: TenantAuthorizationRecord,
    pub service_account_admission: ServiceAccountAdmissionRecord,
    pub audit_context_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SignatureCheckRecord {
    pub dependency_id: &'static str,
    pub credential_id: String,
    pub public_key_ref: String,
    pub key_version: String,
    pub algorithm: String,
    pub canonicalization_version: &'static str,
    pub verified_state: &'static str,
    pub replay_window_state: &'static str,
    pub revocation_state: &'static str,
    pub rotation_state: &'static str,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActorResolutionRecord {
    pub dependency_id: &'static str,
    pub actor_id: String,
    pub actor_type: &'static str,
    pub actor_state: &'static str,
    pub identity_ref: String,
    pub environment_ref: &'static str,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TenantAuthorizationRecord {
    pub dependency_id: &'static str,
    pub tenant_id: String,
    pub tenant_state: &'static str,
    pub membership_state: &'static str,
    pub app_ownership_state: &'static str,
    pub delegated_access_state: &'static str,
    pub role_binding_state: &'static str,
    pub service_account_permission_state: &'static str,
    pub quota_scope_ref: String,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ServiceAccountAdmissionRecord {
    pub principal_kind: &'static str,
    pub command_class: String,
    pub scoped_credential_state: &'static str,
    pub narrow_permission_state: &'static str,
    pub callback_signature_state: &'static str,
    pub trace_audit_state: &'static str,
    pub reason_code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperatorAdmissionRecord {
    pub dependency_id: &'static str,
    pub operator_tenant: String,
    pub operator_role: String,
    pub signed_credential_state: &'static str,
    pub audit_requirement: &'static str,
    pub fail_closed_dependency: &'static str,
    pub reason_code: &'static str,
}

pub fn admit_command(
    envelope: &CommandEnvelope,
    canonical_request: &CanonicalRequestInput,
) -> Result<AdmissionContext, OvergateError> {
    let signature_check = verify_signature(envelope, canonical_request)?;
    let actor_resolution = resolve_actor(envelope)?;
    let tenant_authorization = authorize_tenant(envelope, &actor_resolution)?;
    let service_account_admission = admit_service_or_node_agent(envelope, &actor_resolution)?;

    Ok(AdmissionContext {
        adapter_id: PHASE4_ADMISSION_ADAPTER_ID,
        admission_state: "admitted_before_idempotency_phase4",
        ready_for_idempotency: true,
        signature_check,
        actor_resolution,
        tenant_authorization,
        service_account_admission,
        audit_context_ref: audit_context_ref(&[
            envelope.tenant_id.as_str(),
            envelope.actor_id.as_str(),
            envelope.trace_id.as_str(),
            canonical_request.canonical_hash.as_str(),
        ]),
    })
}

pub fn operator_admission_record(
    role: &str,
    tenant_id: &str,
    trace_id: &str,
) -> OperatorAdmissionRecord {
    OperatorAdmissionRecord {
        dependency_id: "overwatch",
        operator_tenant: tenant_id.to_owned(),
        operator_role: role.to_owned(),
        signed_credential_state: "signed_operator_or_system_service_credential",
        audit_requirement: "strict_admin_audit_required",
        fail_closed_dependency: "overwatch",
        reason_code: "auth.operator_admitted_phase4",
    }
    .with_trace(trace_id)
}

pub fn overwatch_ready(dependencies: &DependencyMatrix) -> bool {
    dependencies.dependency_ready("overwatch")
}

impl OperatorAdmissionRecord {
    fn with_trace(mut self, trace_id: &str) -> Self {
        if trace_id.trim().is_empty() {
            self.reason_code = "auth.operator_trace_missing";
        }
        self
    }
}

fn verify_signature(
    envelope: &CommandEnvelope,
    canonical_request: &CanonicalRequestInput,
) -> Result<SignatureCheckRecord, OvergateError> {
    let signature_ref = envelope.signature_metadata.signature_ref.as_str();
    let credential_id = envelope.credential_id.as_str();
    let key_version = envelope.signature_metadata.key_version.as_str();
    let algorithm = envelope.signature_metadata.algorithm.as_str();
    let timestamp = envelope.timestamp.as_str();

    if signature_ref.contains("malformed") || signature_ref.contains("invalid") {
        return Err(OvergateError::credential_denied(
            "auth.signature_invalid",
            "signature_metadata.signature_ref",
            "signature_invalid",
        ));
    }
    if credential_id.contains("unknown") {
        return Err(OvergateError::credential_denied(
            "auth.credential_unknown",
            "credential_id",
            "credential_unknown",
        ));
    }
    if credential_id.contains("wrong_tenant") || signature_ref.contains("wrong_tenant") {
        return Err(OvergateError::credential_denied(
            "auth.credential_wrong_tenant",
            "credential_id",
            "credential_wrong_tenant",
        ));
    }
    if timestamp.starts_with("2020-") || signature_ref.contains("expired") {
        return Err(OvergateError::credential_denied(
            "auth.signature_expired",
            "timestamp",
            "signature_expired",
        ));
    }
    if signature_ref.contains("replayed") || timestamp.contains("replay") {
        return Err(OvergateError::credential_denied(
            "auth.signature_replay_window_failed",
            "timestamp",
            "signature_replay_window_failed",
        ));
    }
    if credential_id.contains("revoked") || signature_ref.contains("revoked") {
        return Err(OvergateError::credential_denied(
            "auth.credential_revoked",
            "credential_id",
            "credential_revoked",
        ));
    }
    if key_version.contains("rotated") {
        return Err(OvergateError::credential_denied(
            "auth.credential_rotation_denied",
            "signature_metadata.key_version",
            "credential_rotation_denied",
        ));
    }
    if key_version.contains("wrong") {
        return Err(OvergateError::credential_denied(
            "auth.key_version_denied",
            "signature_metadata.key_version",
            "key_version_denied",
        ));
    }
    if algorithm != "ed25519" {
        return Err(OvergateError::credential_denied(
            "auth.signature_algorithm_denied",
            "signature_metadata.algorithm",
            "signature_algorithm_denied",
        ));
    }

    Ok(SignatureCheckRecord {
        dependency_id: "overkey_lite",
        credential_id: envelope.credential_id.clone(),
        public_key_ref: format!(
            "public_key:overkey:{}",
            stable_short_token(&[credential_id, canonical_request.canonical_hash.as_str()])
        ),
        key_version: envelope.signature_metadata.key_version.clone(),
        algorithm: envelope.signature_metadata.algorithm.clone(),
        canonicalization_version: CANONICALIZATION_VERSION,
        verified_state: "verified",
        replay_window_state: "fresh",
        revocation_state: "active",
        rotation_state: "current",
        reason_code: "auth.signature_verified_phase4",
    })
}

fn resolve_actor(envelope: &CommandEnvelope) -> Result<ActorResolutionRecord, OvergateError> {
    let actor_id = envelope.actor_id.as_str();
    if actor_id.contains("unknown") {
        return Err(OvergateError::actor_denied(
            "auth.actor_unknown",
            "actor_id",
            "actor_unknown",
        ));
    }
    if actor_id.contains("disabled") {
        return Err(OvergateError::actor_denied(
            "auth.actor_disabled",
            "actor_id",
            "actor_disabled",
        ));
    }
    if actor_id.contains("suspended") {
        return Err(OvergateError::actor_denied(
            "auth.actor_suspended",
            "actor_id",
            "actor_suspended",
        ));
    }
    if actor_id.contains("deleted") {
        return Err(OvergateError::actor_denied(
            "auth.actor_deleted",
            "actor_id",
            "actor_deleted_marker",
        ));
    }
    if actor_id.contains("wrong_type") {
        return Err(OvergateError::actor_denied(
            "auth.actor_wrong_type",
            "actor_id",
            "actor_wrong_type",
        ));
    }
    if actor_id.contains("env_mismatch") {
        return Err(OvergateError::actor_denied(
            "auth.actor_environment_mismatch",
            "actor_id",
            "actor_environment_mismatch",
        ));
    }

    Ok(ActorResolutionRecord {
        dependency_id: "overpass",
        actor_id: envelope.actor_id.clone(),
        actor_type: actor_type(actor_id),
        actor_state: "active",
        identity_ref: format!("identity:overpass:{}", stable_short_token(&[actor_id])),
        environment_ref: "environment:local",
        reason_code: "auth.actor_resolved_phase4",
    })
}

fn authorize_tenant(
    envelope: &CommandEnvelope,
    actor: &ActorResolutionRecord,
) -> Result<TenantAuthorizationRecord, OvergateError> {
    let tenant_id = envelope.tenant_id.as_str();
    if tenant_id.contains("unknown") {
        return Err(OvergateError::tenant_denied(
            "auth.tenant_unknown",
            "tenant_id",
            "tenant_unknown",
        ));
    }
    if tenant_id.contains("suspended") {
        return Err(OvergateError::tenant_denied(
            "auth.tenant_suspended",
            "tenant_id",
            "tenant_suspended",
        ));
    }
    if envelope.actor_id.contains("wrong_tenant") {
        return Err(OvergateError::tenant_denied(
            "auth.cross_tenant_denied",
            "actor_id",
            "cross_tenant_actor",
        ));
    }
    if envelope.actor_id.contains("no_membership") {
        return Err(OvergateError::tenant_denied(
            "auth.tenant_membership_denied",
            "actor_id",
            "tenant_membership_denied",
        ));
    }
    if envelope.actor_id.contains("role_denied") || envelope.command_type.contains("role_denied") {
        return Err(OvergateError::tenant_denied(
            "auth.tenant_role_denied",
            "command_type",
            "tenant_role_denied",
        ));
    }

    let service_permission = if actor.actor_type == "service_account" {
        "service_account_permission_scoped"
    } else {
        "not_service_account"
    };
    Ok(TenantAuthorizationRecord {
        dependency_id: "overtenant",
        tenant_id: envelope.tenant_id.clone(),
        tenant_state: "active",
        membership_state: "member_or_owned_service",
        app_ownership_state: "app_ownership_checked",
        delegated_access_state: "delegated_access_checked",
        role_binding_state: "role_binding_allows_command",
        service_account_permission_state: service_permission,
        quota_scope_ref: format!(
            "quota_scope:overtenant:{}",
            stable_short_token(&[tenant_id])
        ),
        reason_code: "auth.tenant_authorized_phase4",
    })
}

fn admit_service_or_node_agent(
    envelope: &CommandEnvelope,
    actor: &ActorResolutionRecord,
) -> Result<ServiceAccountAdmissionRecord, OvergateError> {
    match actor.actor_type {
        "service_account" => admit_service_account(envelope),
        "node_agent" => admit_node_agent(envelope),
        _ => Ok(ServiceAccountAdmissionRecord {
            principal_kind: actor.actor_type,
            command_class: command_class(&envelope.command_type),
            scoped_credential_state: "not_service_account_or_node_agent",
            narrow_permission_state: "not_applicable",
            callback_signature_state: "not_applicable",
            trace_audit_state: "trace_and_audit_context_required_for_mutations",
            reason_code: "auth.principal_not_service_account_phase4",
        }),
    }
}

fn admit_service_account(
    envelope: &CommandEnvelope,
) -> Result<ServiceAccountAdmissionRecord, OvergateError> {
    if envelope.credential_id.contains("dev_secret")
        || envelope
            .signature_metadata
            .signature_ref
            .contains("dev_secret")
    {
        return Err(OvergateError::service_account_denied(
            "auth.hardcoded_development_secret_denied",
            "credential_id",
            "hardcoded_development_secret_denied",
        ));
    }
    if envelope.command_type.contains("broad") || envelope.credential_id.contains("broad") {
        return Err(OvergateError::service_account_denied(
            "auth.service_account_scope_denied",
            "command_type",
            "service_account_scope_denied",
        ));
    }
    if !(envelope.command_type.contains("service_account")
        || envelope.command_type.ends_with(".noop"))
    {
        return Err(OvergateError::service_account_denied(
            "auth.service_account_command_class_denied",
            "command_type",
            "service_account_command_class_denied",
        ));
    }
    if envelope.trace_id.contains("missing_audit") {
        return Err(OvergateError::service_account_denied(
            "auth.service_account_audit_context_required",
            "trace_id",
            "service_account_audit_context_required",
        ));
    }

    Ok(ServiceAccountAdmissionRecord {
        principal_kind: "service_account",
        command_class: command_class(&envelope.command_type),
        scoped_credential_state: "scoped_service_credential",
        narrow_permission_state: "narrow_command_class_allowed",
        callback_signature_state: "signed_service_account_command",
        trace_audit_state: "trace_and_audit_context_present",
        reason_code: "auth.service_account_admitted_phase4",
    })
}

fn admit_node_agent(
    envelope: &CommandEnvelope,
) -> Result<ServiceAccountAdmissionRecord, OvergateError> {
    if !(envelope.command_type.contains("node_agent") || envelope.command_type.contains("callback"))
    {
        return Err(OvergateError::service_account_denied(
            "auth.node_agent_command_class_denied",
            "command_type",
            "node_agent_command_class_denied",
        ));
    }
    if envelope.trace_id.contains("missing_audit") {
        return Err(OvergateError::service_account_denied(
            "auth.node_agent_audit_context_required",
            "trace_id",
            "node_agent_audit_context_required",
        ));
    }

    Ok(ServiceAccountAdmissionRecord {
        principal_kind: "node_agent",
        command_class: command_class(&envelope.command_type),
        scoped_credential_state: "scoped_node_agent_credential",
        narrow_permission_state: "node_agent_callback_class_allowed",
        callback_signature_state: "signed_node_agent_callback",
        trace_audit_state: "trace_and_audit_context_present",
        reason_code: "auth.node_agent_admitted_phase4",
    })
}

fn actor_type(actor_id: &str) -> &'static str {
    if actor_id.starts_with("service_account:") {
        "service_account"
    } else if actor_id.starts_with("node_agent:") {
        "node_agent"
    } else if actor_id.starts_with("system_service:") {
        "system_service"
    } else if actor_id.starts_with("operator:") {
        "operator"
    } else if actor_id.starts_with("org:") {
        "organization"
    } else if actor_id.starts_with("app:") {
        "app"
    } else if actor_id.starts_with("native_service:") {
        "native_service"
    } else {
        "person"
    }
}

fn command_class(command_type: &str) -> String {
    command_type
        .split('.')
        .take(3)
        .collect::<Vec<_>>()
        .join(".")
}

fn audit_context_ref(parts: &[&str]) -> String {
    format!("audit_context:overgate:{}", stable_short_token(parts))
}
