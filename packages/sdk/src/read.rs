use crate::{check_sdk_compatibility, OvergateEndpoint, SdkError, SDK_CURRENT_STABLE_MAJOR};
use overrid_contracts::{BootstrapCommandFamily, CapabilitySnapshot, SchemaVersion};

pub const SDK_PHASE3_DEFAULT_PAGE_LIMIT: u16 = 100;
pub const SDK_PHASE3_MAX_PAGE_LIMIT: u16 = 500;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkControlPlaneReadKind {
    Tenant,
    Identity,
    KeyMetadata,
    Manifest,
    QueueStatus,
    AuditRef,
}

impl SdkControlPlaneReadKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Tenant => "tenant",
            Self::Identity => "identity",
            Self::KeyMetadata => "key_metadata",
            Self::Manifest => "manifest",
            Self::QueueStatus => "queue_status",
            Self::AuditRef => "audit_ref",
        }
    }

    pub fn route(self) -> &'static str {
        match self {
            Self::Tenant => "/v1/control-plane/tenants",
            Self::Identity => "/v1/control-plane/identities",
            Self::KeyMetadata => "/v1/control-plane/keys",
            Self::Manifest => "/v1/control-plane/manifests",
            Self::QueueStatus => "/v1/control-plane/queue/status",
            Self::AuditRef => "/v1/control-plane/audit-refs",
        }
    }

    pub fn command_family(self) -> BootstrapCommandFamily {
        match self {
            Self::Tenant => BootstrapCommandFamily::Tenant,
            Self::Identity => BootstrapCommandFamily::Identity,
            Self::KeyMetadata => BootstrapCommandFamily::Key,
            Self::Manifest => BootstrapCommandFamily::Manifest,
            Self::QueueStatus => BootstrapCommandFamily::Workload,
            Self::AuditRef => BootstrapCommandFamily::Auth,
        }
    }

    pub fn phase_gate(self) -> &'static str {
        "phase_1_control_plane_bootstrap"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPagination {
    pub limit: u16,
    pub cursor: Option<String>,
}

impl SdkPagination {
    pub fn new(limit: Option<u16>, cursor: Option<String>) -> Result<Self, SdkError> {
        let limit = limit.unwrap_or(SDK_PHASE3_DEFAULT_PAGE_LIMIT);
        if limit == 0 || limit > SDK_PHASE3_MAX_PAGE_LIMIT {
            return Err(SdkError::MissingRequiredField("valid pagination limit"));
        }
        if cursor.as_ref().is_some_and(|value| value.trim().is_empty()) {
            return Err(SdkError::MissingRequiredField("pagination cursor"));
        }

        Ok(Self { limit, cursor })
    }
}

impl Default for SdkPagination {
    fn default() -> Self {
        Self {
            limit: SDK_PHASE3_DEFAULT_PAGE_LIMIT,
            cursor: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkReadRequest {
    pub endpoint: String,
    pub route: &'static str,
    pub read_kind: SdkControlPlaneReadKind,
    pub object_ref: String,
    pub request_id: String,
    pub trace_id: String,
    pub schema_version: SchemaVersion,
    pub pagination: SdkPagination,
    pub audit_refs: Vec<String>,
    pub headers: Vec<(String, String)>,
    pub read_only: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn build_control_plane_read_request(
    endpoint: &OvergateEndpoint,
    read_kind: SdkControlPlaneReadKind,
    object_ref: impl Into<String>,
    request_id: impl Into<String>,
    trace_id: impl Into<String>,
    schema_version: &str,
    pagination: SdkPagination,
    audit_refs: Vec<String>,
) -> Result<SdkReadRequest, SdkError> {
    let object_ref = object_ref.into();
    let request_id = request_id.into();
    let trace_id = trace_id.into();

    if object_ref.trim().is_empty() {
        return Err(SdkError::MissingRequiredField("read object reference"));
    }
    if request_id.trim().is_empty() {
        return Err(SdkError::MissingRequiredField("request id"));
    }
    if trace_id.trim().is_empty() {
        return Err(SdkError::MissingRequiredField("trace id"));
    }

    let schema_version = check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, schema_version)?;

    Ok(SdkReadRequest {
        endpoint: endpoint.raw().to_owned(),
        route: read_kind.route(),
        read_kind,
        object_ref,
        request_id: request_id.clone(),
        trace_id: trace_id.clone(),
        schema_version: schema_version.clone(),
        pagination,
        audit_refs,
        headers: vec![
            (
                "x-overrid-schema-version".to_owned(),
                schema_version.raw().to_owned(),
            ),
            ("x-overrid-request-id".to_owned(), request_id),
            ("x-overrid-trace-id".to_owned(), trace_id),
            ("x-overrid-read-only".to_owned(), "true".to_owned()),
        ],
        read_only: true,
    })
}

pub fn control_plane_reader_capability(
    read_kind: SdkControlPlaneReadKind,
    schema_version: &str,
) -> Result<CapabilitySnapshot, SdkError> {
    let schema_version = check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, schema_version)?;
    Ok(CapabilitySnapshot {
        route: read_kind.route().to_owned(),
        available: true,
        phase_gate: read_kind.phase_gate().to_owned(),
        schema_versions: vec![schema_version.raw().to_owned()],
        stale_age_ms: 0,
        fail_closed: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use overrid_contracts::SUPPORTED_SCHEMA_VERSION;

    #[test]
    fn phase3_read_request_preserves_request_trace_schema_pagination_and_audit_refs() {
        let config = ClientConfig::local_overgate("http://localhost:8080/overgate").unwrap();
        let pagination = SdkPagination::new(Some(25), Some("cursor_1".to_owned())).unwrap();

        let request = build_control_plane_read_request(
            &config.endpoint,
            SdkControlPlaneReadKind::Tenant,
            "tenant:local",
            "request_1",
            "trace_1",
            SUPPORTED_SCHEMA_VERSION,
            pagination,
            vec!["audit:tenant:local".to_owned()],
        )
        .unwrap();

        assert!(request.read_only);
        assert_eq!(request.route, "/v1/control-plane/tenants");
        assert_eq!(request.pagination.limit, 25);
        assert_eq!(request.pagination.cursor.as_deref(), Some("cursor_1"));
        assert_eq!(request.request_id, "request_1");
        assert_eq!(request.trace_id, "trace_1");
        assert_eq!(request.audit_refs, vec!["audit:tenant:local"]);
        assert!(request.headers.iter().any(|(name, value)| {
            name == "x-overrid-schema-version" && value == SUPPORTED_SCHEMA_VERSION
        }));
    }

    #[test]
    fn phase3_reader_capability_is_phase1_and_fail_closed() {
        let capability = control_plane_reader_capability(
            SdkControlPlaneReadKind::QueueStatus,
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap();

        assert_eq!(capability.route, "/v1/control-plane/queue/status");
        assert_eq!(capability.phase_gate, "phase_1_control_plane_bootstrap");
        assert!(capability.fail_closed);
    }
}
