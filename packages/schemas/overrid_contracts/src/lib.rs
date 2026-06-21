#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

pub const CONTRACT_SOURCE_ROOT: &str = "packages/schemas";
pub const CLI_SCHEMA_FAMILY: &str = "cli-command";
pub const SUPPORTED_SCHEMA_VERSION: &str = "cli-command.v0.1";
pub const INTEGRATION_HARNESS_SCHEMA_FAMILY: &str = "integration-harness";
pub const SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION: &str = "integration-harness.v0.1";
pub const LOCAL_DEVELOPMENT_STACK_SCHEMA_FAMILY: &str = "local-development-stack";
pub const SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION: &str = "local-development-stack.v0.1";
pub const SHARED_SCHEMA_PACKAGE_SCHEMA_FAMILY: &str = "shared-schema-package";
pub const SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION: &str = "shared-schema-package.v0.1";
pub const GENERATED_CONTRACT_STATUS: &str = "rust_projection_from_json_schema_source";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedContractSet {
    pub schema_family: &'static str,
    pub schema_version: &'static str,
    pub source_root: &'static str,
    pub projection_status: &'static str,
}

pub fn cli_contract_set() -> GeneratedContractSet {
    GeneratedContractSet {
        schema_family: CLI_SCHEMA_FAMILY,
        schema_version: SUPPORTED_SCHEMA_VERSION,
        source_root: CONTRACT_SOURCE_ROOT,
        projection_status: GENERATED_CONTRACT_STATUS,
    }
}

pub fn integration_harness_contract_set() -> GeneratedContractSet {
    GeneratedContractSet {
        schema_family: INTEGRATION_HARNESS_SCHEMA_FAMILY,
        schema_version: SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        source_root: CONTRACT_SOURCE_ROOT,
        projection_status: GENERATED_CONTRACT_STATUS,
    }
}

pub fn local_development_stack_contract_set() -> GeneratedContractSet {
    GeneratedContractSet {
        schema_family: LOCAL_DEVELOPMENT_STACK_SCHEMA_FAMILY,
        schema_version: SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
        source_root: CONTRACT_SOURCE_ROOT,
        projection_status: GENERATED_CONTRACT_STATUS,
    }
}

pub fn shared_schema_package_contract_set() -> GeneratedContractSet {
    GeneratedContractSet {
        schema_family: SHARED_SCHEMA_PACKAGE_SCHEMA_FAMILY,
        schema_version: SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION,
        source_root: CONTRACT_SOURCE_ROOT,
        projection_status: GENERATED_CONTRACT_STATUS,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaVersion {
    raw: String,
    family: String,
    major: u16,
    minor: u16,
}

impl SchemaVersion {
    pub fn parse(raw: &str) -> Result<Self, ContractError> {
        if raw.trim().is_empty() {
            return Err(ContractError::MissingSchemaVersion);
        }

        let (family, version) = raw
            .split_once(".v")
            .ok_or_else(|| ContractError::InvalidSchemaVersion(raw.to_owned()))?;
        let (major, minor) = version
            .split_once('.')
            .ok_or_else(|| ContractError::InvalidSchemaVersion(raw.to_owned()))?;

        let major = major
            .parse::<u16>()
            .map_err(|_| ContractError::InvalidSchemaVersion(raw.to_owned()))?;
        let minor = minor
            .parse::<u16>()
            .map_err(|_| ContractError::InvalidSchemaVersion(raw.to_owned()))?;

        Ok(Self {
            raw: raw.to_owned(),
            family: family.to_owned(),
            major,
            minor,
        })
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn minor(&self) -> u16 {
        self.minor
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractError {
    MissingSchemaVersion,
    InvalidSchemaVersion(String),
    UnsupportedSchemaVersion {
        provided: String,
        supported: &'static str,
    },
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSchemaVersion => formatter.write_str("schema version is required"),
            Self::InvalidSchemaVersion(raw) => write!(formatter, "invalid schema version: {raw}"),
            Self::UnsupportedSchemaVersion {
                provided,
                supported,
            } => write!(
                formatter,
                "unsupported schema version {provided}; supported version is {supported}"
            ),
        }
    }
}

impl std::error::Error for ContractError {}

pub fn ensure_supported_schema_version(raw: &str) -> Result<SchemaVersion, ContractError> {
    let parsed = SchemaVersion::parse(raw)?;
    let supported = SchemaVersion::parse(SUPPORTED_SCHEMA_VERSION)?;
    if parsed.family() != CLI_SCHEMA_FAMILY
        || parsed.major() != supported.major()
        || parsed.minor() > supported.minor()
    {
        return Err(ContractError::UnsupportedSchemaVersion {
            provided: raw.to_owned(),
            supported: SUPPORTED_SCHEMA_VERSION,
        });
    }
    Ok(parsed)
}

pub fn ensure_supported_integration_harness_schema_version(
    raw: &str,
) -> Result<SchemaVersion, ContractError> {
    let parsed = SchemaVersion::parse(raw)?;
    let supported = SchemaVersion::parse(SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION)?;
    if parsed.family() != INTEGRATION_HARNESS_SCHEMA_FAMILY
        || parsed.major() != supported.major()
        || parsed.minor() > supported.minor()
    {
        return Err(ContractError::UnsupportedSchemaVersion {
            provided: raw.to_owned(),
            supported: SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        });
    }
    Ok(parsed)
}

pub fn ensure_supported_local_development_stack_schema_version(
    raw: &str,
) -> Result<SchemaVersion, ContractError> {
    let parsed = SchemaVersion::parse(raw)?;
    let supported = SchemaVersion::parse(SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION)?;
    if parsed.family() != LOCAL_DEVELOPMENT_STACK_SCHEMA_FAMILY
        || parsed.major() != supported.major()
        || parsed.minor() > supported.minor()
    {
        return Err(ContractError::UnsupportedSchemaVersion {
            provided: raw.to_owned(),
            supported: SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
        });
    }
    Ok(parsed)
}

pub fn ensure_supported_shared_schema_package_schema_version(
    raw: &str,
) -> Result<SchemaVersion, ContractError> {
    let parsed = SchemaVersion::parse(raw)?;
    let supported = SchemaVersion::parse(SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION)?;
    if parsed.family() != SHARED_SCHEMA_PACKAGE_SCHEMA_FAMILY
        || parsed.major() != supported.major()
        || parsed.minor() > supported.minor()
    {
        return Err(ContractError::UnsupportedSchemaVersion {
            provided: raw.to_owned(),
            supported: SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION,
        });
    }
    Ok(parsed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SharedSchemaPrivacyClass {
    Public,
    TenantPrivate,
    Regulated,
    EncryptedPrivate,
    UserContent,
    SystemServiceOnly,
    RedactedDiagnostic,
}

impl SharedSchemaPrivacyClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::TenantPrivate => "tenant_private",
            Self::Regulated => "regulated",
            Self::EncryptedPrivate => "encrypted_private",
            Self::UserContent => "user_content",
            Self::SystemServiceOnly => "system_service_only",
            Self::RedactedDiagnostic => "redacted_diagnostic",
        }
    }

    pub fn allows_public_object(self) -> bool {
        matches!(self, Self::Public | Self::RedactedDiagnostic)
    }

    pub fn requires_redaction(self) -> bool {
        !matches!(self, Self::Public)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedSchemaRetryability {
    NotRetryable,
    SafeRetry,
    RetryAfter,
    OperatorReview,
}

impl SharedSchemaRetryability {
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
pub struct SharedSchemaTypedRefPrimitive {
    pub primitive_name: String,
    pub object_family: String,
    pub ref_prefix: String,
    pub privacy_class: SharedSchemaPrivacyClass,
    pub public_object_required: bool,
    pub unstructured_string_allowed: bool,
    pub versioned_when_external: bool,
}

impl SharedSchemaTypedRefPrimitive {
    pub fn new(
        primitive_name: impl Into<String>,
        object_family: impl Into<String>,
        ref_prefix: impl Into<String>,
        privacy_class: SharedSchemaPrivacyClass,
        public_object_required: bool,
        versioned_when_external: bool,
    ) -> Self {
        Self {
            primitive_name: primitive_name.into(),
            object_family: object_family.into(),
            ref_prefix: ref_prefix.into(),
            privacy_class,
            public_object_required,
            unstructured_string_allowed: false,
            versioned_when_external,
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPackageContractError> {
        if self.primitive_name.trim().is_empty() {
            return Err(SharedSchemaPackageContractError::MissingPrimitiveName);
        }
        if self.object_family.trim().is_empty() {
            return Err(SharedSchemaPackageContractError::MissingObjectFamily);
        }
        if self.ref_prefix.trim().is_empty()
            || !(self.ref_prefix.ends_with(':') || self.ref_prefix.ends_with("://"))
        {
            return Err(SharedSchemaPackageContractError::UntypedRefPrimitive(
                self.primitive_name.clone(),
            ));
        }
        if self.unstructured_string_allowed {
            return Err(SharedSchemaPackageContractError::UntypedRefPrimitive(
                self.primitive_name.clone(),
            ));
        }
        if self.public_object_required && !self.versioned_when_external {
            return Err(SharedSchemaPackageContractError::UnversionedPublicRef(
                self.primitive_name.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaLifecyclePrimitives {
    pub schema_version: SchemaVersion,
    pub timestamp: &'static str,
    pub logical_sequence: &'static str,
    pub command_id_prefix: &'static str,
    pub trace_id_prefix: &'static str,
    pub idempotency_key_prefix: &'static str,
    pub request_hash_prefix: &'static str,
    pub payload_hash_prefix: &'static str,
    pub previous_major_supported: bool,
    pub migration_plan_ref: &'static str,
}

impl SharedSchemaLifecyclePrimitives {
    pub fn phase2_default() -> Result<Self, ContractError> {
        Ok(Self {
            schema_version: ensure_supported_shared_schema_package_schema_version(
                SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION,
            )?,
            timestamp: "rfc3339_utc_ms",
            logical_sequence: "monotonic_u64",
            command_id_prefix: "command:",
            trace_id_prefix: "trace_",
            idempotency_key_prefix: "idem_",
            request_hash_prefix: "hash_",
            payload_hash_prefix: "hash_",
            previous_major_supported: true,
            migration_plan_ref: "migration:phase2:none",
        })
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPackageContractError> {
        if self.trace_id_prefix != "trace_" {
            return Err(SharedSchemaPackageContractError::InvalidLifecyclePrimitive(
                "trace_id",
            ));
        }
        if self.idempotency_key_prefix != "idem_" {
            return Err(SharedSchemaPackageContractError::InvalidLifecyclePrimitive(
                "idempotency_key",
            ));
        }
        if self.request_hash_prefix != "hash_" || self.payload_hash_prefix != "hash_" {
            return Err(SharedSchemaPackageContractError::InvalidLifecyclePrimitive(
                "hash",
            ));
        }
        if !self.previous_major_supported {
            return Err(SharedSchemaPackageContractError::InvalidLifecyclePrimitive(
                "compatibility_window",
            ));
        }
        if !self.migration_plan_ref.starts_with("migration:") {
            return Err(SharedSchemaPackageContractError::InvalidLifecyclePrimitive(
                "migration_plan_ref",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaReasonCodeEntry {
    pub reason_code: String,
    pub retryability: SharedSchemaRetryability,
    pub correction_fields: Vec<String>,
    pub dependency_name: String,
    pub policy_ref: String,
    pub audit_ref: String,
    pub unsupported_version: bool,
    pub documented: bool,
    pub rust_first_enum_source: bool,
}

impl SharedSchemaReasonCodeEntry {
    pub fn new(
        reason_code: impl Into<String>,
        retryability: SharedSchemaRetryability,
        correction_fields: Vec<String>,
        unsupported_version: bool,
    ) -> Self {
        Self {
            reason_code: reason_code.into(),
            retryability,
            correction_fields,
            dependency_name: "shared_schema_package".to_owned(),
            policy_ref: "policy:shared_schema_package".to_owned(),
            audit_ref: "audit:schema:phase2".to_owned(),
            unsupported_version,
            documented: true,
            rust_first_enum_source: true,
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPackageContractError> {
        if !self.reason_code.contains('.') {
            return Err(SharedSchemaPackageContractError::InvalidReasonCode(
                self.reason_code.clone(),
            ));
        }
        if self.correction_fields.is_empty() {
            return Err(SharedSchemaPackageContractError::MissingCorrectionFields(
                self.reason_code.clone(),
            ));
        }
        if !self.documented || !self.rust_first_enum_source {
            return Err(SharedSchemaPackageContractError::UndocumentedReasonCode(
                self.reason_code.clone(),
            ));
        }
        if !self.policy_ref.starts_with("policy:") || !self.audit_ref.starts_with("audit:") {
            return Err(SharedSchemaPackageContractError::InvalidReasonCode(
                self.reason_code.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase2Contract {
    pub schema_version: SchemaVersion,
    pub source_roots: Vec<String>,
    pub generated_output_roots: Vec<String>,
    pub fixture_roots: Vec<String>,
    pub compatibility_report_roots: Vec<String>,
    pub internal_binary_projection_roots: Vec<String>,
    pub generated_in_source_directories: bool,
    pub typed_ref_primitives: Vec<SharedSchemaTypedRefPrimitive>,
    pub lifecycle_primitives: SharedSchemaLifecyclePrimitives,
    pub privacy_classes: Vec<SharedSchemaPrivacyClass>,
    pub reason_codes: Vec<SharedSchemaReasonCodeEntry>,
}

impl SharedSchemaPhase2Contract {
    pub fn canonical() -> Result<Self, ContractError> {
        Ok(Self {
            schema_version: ensure_supported_shared_schema_package_schema_version(
                SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION,
            )?,
            source_roots: vec![
                "packages/schemas/README.md".to_owned(),
                "packages/schemas/overrid_contracts/v0".to_owned(),
                "packages/schemas/overrid_contracts/codegen_manifest.json".to_owned(),
                "packages/schemas/overrid_contracts/README.md".to_owned(),
            ],
            generated_output_roots: vec![
                "packages/schemas/overrid_contracts/src/lib.rs".to_owned(),
                "packages/schemas/admin_ui/generated".to_owned(),
            ],
            fixture_roots: vec![
                "packages/schemas/overrid_contracts/fixtures/valid".to_owned(),
                "packages/schemas/overrid_contracts/fixtures/invalid".to_owned(),
            ],
            compatibility_report_roots: vec![
                "packages/schemas/overrid_contracts/compatibility".to_owned(),
                "docs/specs/generated".to_owned(),
            ],
            internal_binary_projection_roots: vec![
                "packages/schemas/overrid_contracts/protobuf/internal".to_owned(),
            ],
            generated_in_source_directories: false,
            typed_ref_primitives: required_shared_schema_typed_refs(),
            lifecycle_primitives: SharedSchemaLifecyclePrimitives::phase2_default()?,
            privacy_classes: vec![
                SharedSchemaPrivacyClass::Public,
                SharedSchemaPrivacyClass::TenantPrivate,
                SharedSchemaPrivacyClass::Regulated,
                SharedSchemaPrivacyClass::EncryptedPrivate,
                SharedSchemaPrivacyClass::UserContent,
                SharedSchemaPrivacyClass::SystemServiceOnly,
                SharedSchemaPrivacyClass::RedactedDiagnostic,
            ],
            reason_codes: vec![
                SharedSchemaReasonCodeEntry::new(
                    "schema.unsupported_version",
                    SharedSchemaRetryability::NotRetryable,
                    vec!["schema_version".to_owned()],
                    true,
                ),
                SharedSchemaReasonCodeEntry::new(
                    "validation.typed_ref_required",
                    SharedSchemaRetryability::NotRetryable,
                    vec!["ref_prefix".to_owned()],
                    false,
                ),
                SharedSchemaReasonCodeEntry::new(
                    "policy.privacy_class_missing",
                    SharedSchemaRetryability::OperatorReview,
                    vec!["privacy_class".to_owned()],
                    false,
                ),
                SharedSchemaReasonCodeEntry::new(
                    "dependency.generated_source_misplaced",
                    SharedSchemaRetryability::OperatorReview,
                    vec!["generated_output_roots".to_owned()],
                    false,
                ),
            ],
        })
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPackageContractError> {
        if !self
            .source_roots
            .iter()
            .any(|path| path == "packages/schemas/overrid_contracts/v0")
        {
            return Err(SharedSchemaPackageContractError::MissingSourceRoot(
                "packages/schemas/overrid_contracts/v0",
            ));
        }
        if self.generated_in_source_directories {
            return Err(SharedSchemaPackageContractError::GeneratedOutputInsideSource);
        }
        if self.generated_output_roots.iter().any(|path| {
            path.starts_with("packages/schemas/overrid_contracts/v0/")
                || path.ends_with(".schema.json")
        }) {
            return Err(SharedSchemaPackageContractError::GeneratedOutputInsideSource);
        }
        for required_root in [
            "packages/schemas/overrid_contracts/compatibility",
            "docs/specs/generated",
        ] {
            if !self
                .compatibility_report_roots
                .iter()
                .any(|path| path == required_root)
            {
                return Err(
                    SharedSchemaPackageContractError::MissingCompatibilityReportRoot(required_root),
                );
            }
        }
        if !self
            .internal_binary_projection_roots
            .iter()
            .any(|path| path == "packages/schemas/overrid_contracts/protobuf/internal")
        {
            return Err(
                SharedSchemaPackageContractError::MissingInternalBinaryProjectionRoot(
                    "packages/schemas/overrid_contracts/protobuf/internal",
                ),
            );
        }
        for primitive in &self.typed_ref_primitives {
            primitive.validate()?;
        }
        for required_family in REQUIRED_SHARED_SCHEMA_OBJECT_FAMILIES {
            if !self
                .typed_ref_primitives
                .iter()
                .any(|primitive| primitive.object_family == *required_family)
            {
                return Err(SharedSchemaPackageContractError::MissingObjectFamily);
            }
        }
        let privacy_classes: BTreeSet<_> = self.privacy_classes.iter().copied().collect();
        for required_class in REQUIRED_SHARED_SCHEMA_PRIVACY_CLASSES {
            if !privacy_classes.contains(required_class) {
                return Err(SharedSchemaPackageContractError::MissingPrivacyClass(
                    required_class.as_str(),
                ));
            }
        }
        self.lifecycle_primitives.validate()?;
        for reason in &self.reason_codes {
            reason.validate()?;
        }
        if !self
            .reason_codes
            .iter()
            .any(|reason| reason.unsupported_version)
        {
            return Err(SharedSchemaPackageContractError::MissingUnsupportedVersionReason);
        }
        Ok(())
    }

    pub fn has_typed_ref_family(&self, object_family: &str) -> bool {
        self.typed_ref_primitives
            .iter()
            .any(|primitive| primitive.object_family == object_family)
    }

    pub fn has_reason_code(&self, reason_code: &str) -> bool {
        self.reason_codes
            .iter()
            .any(|entry| entry.reason_code == reason_code)
    }
}

pub const REQUIRED_SHARED_SCHEMA_OBJECT_FAMILIES: &[&str] = &[
    "person",
    "organization",
    "tenant",
    "node",
    "app",
    "native_service",
    "service_account",
    "system_service",
    "central_ai_actor",
    "overasset",
    "namespace",
    "route",
    "package",
    "workload",
    "queue",
    "lease",
    "receipt",
    "evidence",
    "secret_reference",
];

pub const REQUIRED_SHARED_SCHEMA_PRIVACY_CLASSES: &[SharedSchemaPrivacyClass] = &[
    SharedSchemaPrivacyClass::Public,
    SharedSchemaPrivacyClass::TenantPrivate,
    SharedSchemaPrivacyClass::Regulated,
    SharedSchemaPrivacyClass::EncryptedPrivate,
    SharedSchemaPrivacyClass::UserContent,
    SharedSchemaPrivacyClass::SystemServiceOnly,
    SharedSchemaPrivacyClass::RedactedDiagnostic,
];

pub fn required_shared_schema_typed_refs() -> Vec<SharedSchemaTypedRefPrimitive> {
    vec![
        SharedSchemaTypedRefPrimitive::new(
            "person_ref",
            "person",
            "person:",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "organization_ref",
            "organization",
            "org:",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "tenant_ref",
            "tenant",
            "tenant:",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "node_ref",
            "node",
            "node:",
            SharedSchemaPrivacyClass::SystemServiceOnly,
            false,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "app_ref",
            "app",
            "app:",
            SharedSchemaPrivacyClass::Public,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "native_service_ref",
            "native_service",
            "native_service:",
            SharedSchemaPrivacyClass::SystemServiceOnly,
            false,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "service_account_ref",
            "service_account",
            "service_account:",
            SharedSchemaPrivacyClass::SystemServiceOnly,
            false,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "system_service_ref",
            "system_service",
            "system_service:",
            SharedSchemaPrivacyClass::SystemServiceOnly,
            false,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "central_ai_actor_ref",
            "central_ai_actor",
            "central_ai:",
            SharedSchemaPrivacyClass::Regulated,
            false,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "overasset_ref",
            "overasset",
            "overasset:",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "namespace_ref",
            "namespace",
            "namespace:",
            SharedSchemaPrivacyClass::Public,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "route_ref",
            "route",
            "route:",
            SharedSchemaPrivacyClass::Public,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "package_ref",
            "package",
            "package:",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "workload_ref",
            "workload",
            "workload:",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "queue_ref",
            "queue",
            "queue:",
            SharedSchemaPrivacyClass::SystemServiceOnly,
            false,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "lease_ref",
            "lease",
            "lease:",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "receipt_ref",
            "receipt",
            "receipt:",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "evidence_ref",
            "evidence",
            "evidence:",
            SharedSchemaPrivacyClass::RedactedDiagnostic,
            true,
            true,
        ),
        SharedSchemaTypedRefPrimitive::new(
            "secret_ref",
            "secret_reference",
            "secret:",
            SharedSchemaPrivacyClass::EncryptedPrivate,
            false,
            true,
        ),
    ]
}

pub const REQUIRED_SHARED_SCHEMA_PHASE3_MODULES: &[&str] = &[
    "identity",
    "tenant",
    "command",
    "api_error",
    "event",
    "audit",
    "workload_manifest",
    "resource_manifest",
    "registry_metadata",
    "queue_and_lease",
    "credential_key_metadata",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SharedSchemaPhase3ModuleFamily {
    IdentityTenant,
    CommandApiError,
    EventAudit,
    ManifestRegistry,
    QueueLeaseCredentialKey,
}

impl SharedSchemaPhase3ModuleFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IdentityTenant => "identity_tenant",
            Self::CommandApiError => "command_api_error",
            Self::EventAudit => "event_audit",
            Self::ManifestRegistry => "manifest_registry",
            Self::QueueLeaseCredentialKey => "queue_lease_credential_key",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase3ContractRule {
    pub rule_id: String,
    pub reason_code: String,
    pub correction_fields: Vec<String>,
    pub enforcement: String,
}

impl SharedSchemaPhase3ContractRule {
    pub fn new(
        rule_id: impl Into<String>,
        reason_code: impl Into<String>,
        correction_fields: Vec<String>,
        enforcement: impl Into<String>,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            reason_code: reason_code.into(),
            correction_fields,
            enforcement: enforcement.into(),
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase3ContractError> {
        if self.rule_id.trim().is_empty() {
            return Err(SharedSchemaPhase3ContractError::MissingRuleId);
        }
        if !self.reason_code.contains('.') {
            return Err(SharedSchemaPhase3ContractError::InvalidRuleReasonCode(
                self.reason_code.clone(),
            ));
        }
        if self.correction_fields.is_empty() {
            return Err(
                SharedSchemaPhase3ContractError::MissingRuleCorrectionFields(self.rule_id.clone()),
            );
        }
        match self.enforcement.as_str() {
            "require" | "reject" | "preserve_append_only" | "require_typed_ref" => Ok(()),
            _ => Err(SharedSchemaPhase3ContractError::InvalidRuleEnforcement(
                self.enforcement.clone(),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase3ContractModule {
    pub module_name: String,
    pub module_family: SharedSchemaPhase3ModuleFamily,
    pub schema_version: SchemaVersion,
    pub owning_service_family: String,
    pub downstream_consumers: Vec<String>,
    pub source_of_truth: String,
    pub rust_projection_non_authoritative: bool,
    pub runtime_authority: String,
    pub privacy_class: SharedSchemaPrivacyClass,
    pub strict_unknown_field_rejection: bool,
    pub raw_secret_values_allowed: bool,
    pub private_key_material_allowed: bool,
    pub tenant_actor_refs_required: bool,
    pub append_only_record: bool,
    pub reason_code_required: bool,
    pub correction_shape_required: bool,
    pub privacy_class_required: bool,
    pub typed_secret_refs_required: bool,
    pub untyped_capability_blobs_allowed: bool,
    pub required_refs: Vec<String>,
    pub required_fields: Vec<String>,
    pub validation_rules: Vec<SharedSchemaPhase3ContractRule>,
}

impl SharedSchemaPhase3ContractModule {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        module_name: impl Into<String>,
        module_family: SharedSchemaPhase3ModuleFamily,
        owning_service_family: impl Into<String>,
        downstream_consumers: Vec<String>,
        privacy_class: SharedSchemaPrivacyClass,
        tenant_actor_refs_required: bool,
        append_only_record: bool,
        reason_code_required: bool,
        correction_shape_required: bool,
        typed_secret_refs_required: bool,
        required_refs: Vec<String>,
        required_fields: Vec<String>,
        validation_rules: Vec<SharedSchemaPhase3ContractRule>,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            module_name: module_name.into(),
            module_family,
            schema_version: ensure_supported_shared_schema_package_schema_version(
                SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION,
            )?,
            owning_service_family: owning_service_family.into(),
            downstream_consumers,
            source_of_truth: "json_schema".to_owned(),
            rust_projection_non_authoritative: true,
            runtime_authority: "owning_service".to_owned(),
            privacy_class,
            strict_unknown_field_rejection: true,
            raw_secret_values_allowed: false,
            private_key_material_allowed: false,
            tenant_actor_refs_required,
            append_only_record,
            reason_code_required,
            correction_shape_required,
            privacy_class_required: true,
            typed_secret_refs_required,
            untyped_capability_blobs_allowed: false,
            required_refs,
            required_fields,
            validation_rules,
        })
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase3ContractError> {
        if self.module_name.trim().is_empty() {
            return Err(SharedSchemaPhase3ContractError::MissingModuleName);
        }
        if self.owning_service_family.trim().is_empty() || self.downstream_consumers.is_empty() {
            return Err(SharedSchemaPhase3ContractError::MissingOwnership(
                self.module_name.clone(),
            ));
        }
        if self.source_of_truth != "json_schema"
            || !self.rust_projection_non_authoritative
            || self.runtime_authority != "owning_service"
        {
            return Err(SharedSchemaPhase3ContractError::AuthorityDrift(
                self.module_name.clone(),
            ));
        }
        if !self.strict_unknown_field_rejection {
            return Err(
                SharedSchemaPhase3ContractError::UnknownSensitiveFieldsAllowed(
                    self.module_name.clone(),
                ),
            );
        }
        if self.raw_secret_values_allowed {
            return Err(SharedSchemaPhase3ContractError::RawSecretValuesAllowed(
                self.module_name.clone(),
            ));
        }
        if self.private_key_material_allowed {
            return Err(SharedSchemaPhase3ContractError::PrivateKeyMaterialAllowed(
                self.module_name.clone(),
            ));
        }
        if self.untyped_capability_blobs_allowed {
            return Err(
                SharedSchemaPhase3ContractError::UntypedCapabilityBlobAllowed(
                    self.module_name.clone(),
                ),
            );
        }
        if self.tenant_actor_refs_required
            && (!self.has_required_ref("tenant_ref") || !self.has_required_ref("actor_ref"))
        {
            return Err(SharedSchemaPhase3ContractError::MissingTenantActorRefs(
                self.module_name.clone(),
            ));
        }
        if self.append_only_record && !self.has_required_field("sequence") {
            return Err(SharedSchemaPhase3ContractError::MissingAppendOnlySequence(
                self.module_name.clone(),
            ));
        }
        if self.privacy_class_required && !self.has_required_field("privacy_class") {
            return Err(SharedSchemaPhase3ContractError::MissingPrivacyClassField(
                self.module_name.clone(),
            ));
        }
        if self.reason_code_required && !self.has_required_field("reason_code") {
            return Err(SharedSchemaPhase3ContractError::MissingReasonCode(
                self.module_name.clone(),
            ));
        }
        if self.correction_shape_required && !self.has_required_field("correction_fields") {
            return Err(SharedSchemaPhase3ContractError::MissingCorrectionShape(
                self.module_name.clone(),
            ));
        }
        if self.typed_secret_refs_required && !self.has_required_ref("secret_ref") {
            return Err(SharedSchemaPhase3ContractError::MissingTypedSecretRef(
                self.module_name.clone(),
            ));
        }
        if self.module_name == "command" {
            for field in [
                "trace_id",
                "idempotency_key",
                "payload_hash",
                "signature_metadata",
            ] {
                if !self.has_required_field(field) {
                    return Err(SharedSchemaPhase3ContractError::IncompleteCommandEnvelope(
                        field,
                    ));
                }
            }
        }
        if self.module_name == "api_error" {
            for field in [
                "reason_code",
                "trace_id",
                "retryability",
                "correction_fields",
            ] {
                if !self.has_required_field(field) {
                    return Err(SharedSchemaPhase3ContractError::IncompleteApiError(field));
                }
            }
        }
        if self.module_name == "credential_key_metadata"
            && (!self.has_required_field("key_rotation")
                || !self.has_required_field("revocation")
                || !self.has_required_ref("signer_ref"))
        {
            return Err(SharedSchemaPhase3ContractError::IncompleteCredentialKeyMetadata);
        }
        for rule in &self.validation_rules {
            rule.validate()?;
        }
        Ok(())
    }

    pub fn has_required_ref(&self, required_ref: &str) -> bool {
        self.required_refs.iter().any(|item| item == required_ref)
    }

    pub fn has_required_field(&self, required_field: &str) -> bool {
        self.required_fields
            .iter()
            .any(|item| item == required_field)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase3Contract {
    pub schema_version: SchemaVersion,
    pub modules: Vec<SharedSchemaPhase3ContractModule>,
}

impl SharedSchemaPhase3Contract {
    pub fn canonical() -> Result<Self, ContractError> {
        Ok(Self {
            schema_version: ensure_supported_shared_schema_package_schema_version(
                SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION,
            )?,
            modules: shared_schema_phase3_contract_modules()?,
        })
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase3ContractError> {
        let module_names: BTreeSet<_> = self
            .modules
            .iter()
            .map(|module| module.module_name.as_str())
            .collect();
        for required_module in REQUIRED_SHARED_SCHEMA_PHASE3_MODULES {
            if !module_names.contains(required_module) {
                return Err(SharedSchemaPhase3ContractError::MissingModule(
                    required_module,
                ));
            }
        }
        for module in &self.modules {
            module.validate()?;
        }
        Ok(())
    }

    pub fn module(&self, module_name: &str) -> Option<&SharedSchemaPhase3ContractModule> {
        self.modules
            .iter()
            .find(|module| module.module_name == module_name)
    }
}

pub fn shared_schema_phase3_contract_modules(
) -> Result<Vec<SharedSchemaPhase3ContractModule>, ContractError> {
    Ok(vec![
        phase3_module(
            "identity",
            SharedSchemaPhase3ModuleFamily::IdentityTenant,
            "SDS #10 Overpass and SDS #14 Overtenant",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            false,
            false,
            false,
            false,
            &[
                "identity_ref",
                "actor_ref",
                "tenant_ref",
                "organization_ref",
                "audit_context_ref",
            ],
            &[
                "membership",
                "role_binding",
                "delegated_access",
                "quota_scope",
                "suspension_state",
                "catalog_visibility",
                "privacy_class",
            ],
            &[(
                "identity.requires_tenant_actor_audit_context",
                "identity.tenant_actor_required",
                &["tenant_ref", "actor_ref", "audit_context_ref"][..],
                "require",
            )],
        )?,
        phase3_module(
            "tenant",
            SharedSchemaPhase3ModuleFamily::IdentityTenant,
            "SDS #14 Overtenant",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            false,
            false,
            false,
            false,
            &[
                "tenant_ref",
                "actor_ref",
                "organization_ref",
                "audit_context_ref",
            ],
            &[
                "membership",
                "role_binding",
                "delegated_access",
                "quota_scope",
                "suspension_state",
                "catalog_visibility",
                "privacy_class",
            ],
            &[(
                "tenant.requires_quota_and_catalog_visibility",
                "tenant.scope_required",
                &["quota_scope", "catalog_visibility"][..],
                "require",
            )],
        )?,
        phase3_module(
            "command",
            SharedSchemaPhase3ModuleFamily::CommandApiError,
            "SDS #8 Overgate",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            false,
            false,
            false,
            false,
            &["tenant_ref", "actor_ref", "credential_ref", "signature_ref"],
            &[
                "command_id",
                "command_type",
                "trace_id",
                "idempotency_key",
                "timestamp",
                "schema_version",
                "payload_type",
                "payload_hash",
                "signature_metadata",
                "privacy_class",
            ],
            &[(
                "command.requires_signed_tenant_actor_envelope",
                "command.envelope_incomplete",
                &["tenant_ref", "actor_ref", "trace_id", "idempotency_key"][..],
                "require",
            )],
        )?,
        phase3_module(
            "api_error",
            SharedSchemaPhase3ModuleFamily::CommandApiError,
            "SDS #8 Overgate",
            SharedSchemaPrivacyClass::RedactedDiagnostic,
            false,
            false,
            true,
            true,
            false,
            &["trace_ref", "audit_ref"],
            &[
                "reason_code",
                "trace_id",
                "retryability",
                "correction_fields",
                "schema_version",
                "privacy_class",
            ],
            &[(
                "api_error.requires_reason_trace_retry_correction",
                "api_error.reason_trace_required",
                &["reason_code", "trace_id", "correction_fields"][..],
                "require",
            )],
        )?,
        phase3_module(
            "event",
            SharedSchemaPhase3ModuleFamily::EventAudit,
            "SDS #15 Overwatch",
            SharedSchemaPrivacyClass::RedactedDiagnostic,
            true,
            true,
            false,
            false,
            false,
            &[
                "event_ref",
                "tenant_ref",
                "actor_ref",
                "subject_ref",
                "policy_ref",
                "evidence_ref",
                "signature_ref",
            ],
            &[
                "event_id",
                "source_service",
                "subject_id",
                "action",
                "decision",
                "sequence",
                "occurred_time",
                "privacy_class",
                "schema_version",
            ],
            &[(
                "event.append_only_privacy_classified",
                "event.append_only_required",
                &["sequence", "privacy_class", "evidence_ref"][..],
                "preserve_append_only",
            )],
        )?,
        phase3_module(
            "audit",
            SharedSchemaPhase3ModuleFamily::EventAudit,
            "SDS #15 Overwatch",
            SharedSchemaPrivacyClass::RedactedDiagnostic,
            true,
            true,
            false,
            false,
            false,
            &[
                "audit_ref",
                "tenant_ref",
                "actor_ref",
                "policy_ref",
                "evidence_ref",
                "signature_ref",
            ],
            &[
                "audit_id",
                "source_service",
                "subject_id",
                "action",
                "decision",
                "sequence",
                "occurred_time",
                "privacy_class",
                "schema_version",
            ],
            &[(
                "audit.append_only_no_private_payload",
                "audit.private_payload_rejected",
                &["privacy_class", "evidence_ref"][..],
                "reject",
            )],
        )?,
        phase3_module(
            "workload_manifest",
            SharedSchemaPhase3ModuleFamily::ManifestRegistry,
            "SDS #24 Overpack",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            false,
            false,
            false,
            true,
            &[
                "workload_ref",
                "tenant_ref",
                "actor_ref",
                "package_ref",
                "secret_ref",
                "policy_ref",
                "schema_ref",
            ],
            &[
                "schema_version",
                "resource_requirements",
                "data_refs",
                "network_intent",
                "retry_policy",
                "privacy_class",
            ],
            &[(
                "workload_manifest.requires_typed_secret_refs",
                "manifest.secret_ref_required",
                &["secret_ref", "schema_version"][..],
                "require_typed_ref",
            )],
        )?,
        phase3_module(
            "resource_manifest",
            SharedSchemaPhase3ModuleFamily::ManifestRegistry,
            "SDS #24 Overpack and SDS #26 Oversched",
            SharedSchemaPrivacyClass::TenantPrivate,
            false,
            false,
            false,
            false,
            true,
            &[
                "resource_manifest_ref",
                "node_ref",
                "policy_ref",
                "schema_ref",
                "secret_ref",
            ],
            &[
                "schema_version",
                "capability_records",
                "resource_requirements",
                "data_refs",
                "privacy_class",
            ],
            &[(
                "resource_manifest.rejects_untyped_capabilities",
                "manifest.untyped_capability_rejected",
                &["capability_records"][..],
                "reject",
            )],
        )?,
        phase3_module(
            "registry_metadata",
            SharedSchemaPhase3ModuleFamily::ManifestRegistry,
            "SDS #12 Overregistry",
            SharedSchemaPrivacyClass::TenantPrivate,
            false,
            false,
            false,
            false,
            false,
            &[
                "package_ref",
                "capability_ref",
                "image_ref",
                "schema_ref",
                "policy_ref",
            ],
            &[
                "schema_version",
                "capability_records",
                "review_status",
                "visibility",
                "privacy_class",
            ],
            &[(
                "registry_metadata_no_public_private_internals",
                "registry.private_internal_rejected",
                &["visibility", "schema_ref"][..],
                "reject",
            )],
        )?,
        phase3_module(
            "queue_and_lease",
            SharedSchemaPhase3ModuleFamily::QueueLeaseCredentialKey,
            "SDS #11 Overqueue and SDS #21 Overlease",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            true,
            true,
            true,
            false,
            &[
                "queue_ref",
                "lease_ref",
                "tenant_ref",
                "actor_ref",
                "command_ref",
                "dead_letter_ref",
                "credential_ref",
            ],
            &[
                "trace_id",
                "retry_policy",
                "timeout_policy",
                "heartbeat",
                "cancellation",
                "completion",
                "reason_code",
                "correction_fields",
                "sequence",
                "privacy_class",
            ],
            &[(
                "queue_lease_preserves_trace_and_terminal_refs",
                "queue_lease.trace_required",
                &["trace_id", "queue_ref", "lease_ref"][..],
                "preserve_append_only",
            )],
        )?,
        phase3_module(
            "credential_key_metadata",
            SharedSchemaPhase3ModuleFamily::QueueLeaseCredentialKey,
            "SDS #9 Overkey",
            SharedSchemaPrivacyClass::EncryptedPrivate,
            true,
            true,
            true,
            true,
            true,
            &[
                "credential_ref",
                "signer_ref",
                "key_ref",
                "tenant_ref",
                "actor_ref",
                "secret_ref",
                "audit_ref",
            ],
            &[
                "credential_metadata",
                "key_rotation",
                "revocation",
                "reason_code",
                "trace_id",
                "correction_fields",
                "sequence",
                "privacy_class",
            ],
            &[(
                "credential_key_metadata_refs_only",
                "credential.private_key_material_rejected",
                &["credential_ref", "signer_ref", "secret_ref"][..],
                "reject",
            )],
        )?,
    ])
}

#[allow(clippy::too_many_arguments)]
fn phase3_module(
    module_name: &str,
    module_family: SharedSchemaPhase3ModuleFamily,
    owning_service_family: &str,
    privacy_class: SharedSchemaPrivacyClass,
    tenant_actor_refs_required: bool,
    append_only_record: bool,
    reason_code_required: bool,
    correction_shape_required: bool,
    typed_secret_refs_required: bool,
    required_refs: &[&str],
    required_fields: &[&str],
    rules: &[(&str, &str, &[&str], &str)],
) -> Result<SharedSchemaPhase3ContractModule, ContractError> {
    SharedSchemaPhase3ContractModule::new(
        module_name,
        module_family,
        owning_service_family,
        vec![
            "Shared Schema Package".to_owned(),
            "CLI".to_owned(),
            "SDK".to_owned(),
            "Integration Test Harness".to_owned(),
        ],
        privacy_class,
        tenant_actor_refs_required,
        append_only_record,
        reason_code_required,
        correction_shape_required,
        typed_secret_refs_required,
        required_refs
            .iter()
            .map(|item| (*item).to_owned())
            .collect(),
        required_fields
            .iter()
            .map(|item| (*item).to_owned())
            .collect(),
        rules
            .iter()
            .map(|(rule_id, reason_code, correction_fields, enforcement)| {
                SharedSchemaPhase3ContractRule::new(
                    *rule_id,
                    *reason_code,
                    correction_fields
                        .iter()
                        .map(|item| (*item).to_owned())
                        .collect(),
                    *enforcement,
                )
            })
            .collect(),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedSchemaPhase3ContractError {
    MissingModule(&'static str),
    MissingModuleName,
    MissingOwnership(String),
    AuthorityDrift(String),
    UnknownSensitiveFieldsAllowed(String),
    RawSecretValuesAllowed(String),
    PrivateKeyMaterialAllowed(String),
    UntypedCapabilityBlobAllowed(String),
    MissingTenantActorRefs(String),
    MissingAppendOnlySequence(String),
    MissingPrivacyClassField(String),
    MissingReasonCode(String),
    MissingCorrectionShape(String),
    MissingTypedSecretRef(String),
    IncompleteCommandEnvelope(&'static str),
    IncompleteApiError(&'static str),
    IncompleteCredentialKeyMetadata,
    MissingRuleId,
    InvalidRuleReasonCode(String),
    MissingRuleCorrectionFields(String),
    InvalidRuleEnforcement(String),
}

impl fmt::Display for SharedSchemaPhase3ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingModule(module) => write!(formatter, "missing Phase 3 module: {module}"),
            Self::MissingModuleName => formatter.write_str("module name is required"),
            Self::MissingOwnership(module) => write!(formatter, "missing ownership: {module}"),
            Self::AuthorityDrift(module) => write!(formatter, "authority drift: {module}"),
            Self::UnknownSensitiveFieldsAllowed(module) => {
                write!(formatter, "unknown sensitive fields allowed: {module}")
            }
            Self::RawSecretValuesAllowed(module) => {
                write!(formatter, "raw secret values allowed: {module}")
            }
            Self::PrivateKeyMaterialAllowed(module) => {
                write!(formatter, "private key material allowed: {module}")
            }
            Self::UntypedCapabilityBlobAllowed(module) => {
                write!(formatter, "untyped capability blobs allowed: {module}")
            }
            Self::MissingTenantActorRefs(module) => {
                write!(formatter, "missing tenant/actor refs: {module}")
            }
            Self::MissingAppendOnlySequence(module) => {
                write!(formatter, "missing append-only sequence: {module}")
            }
            Self::MissingPrivacyClassField(module) => {
                write!(formatter, "missing privacy class field: {module}")
            }
            Self::MissingReasonCode(module) => write!(formatter, "missing reason code: {module}"),
            Self::MissingCorrectionShape(module) => {
                write!(formatter, "missing correction shape: {module}")
            }
            Self::MissingTypedSecretRef(module) => {
                write!(formatter, "missing typed secret ref: {module}")
            }
            Self::IncompleteCommandEnvelope(field) => {
                write!(formatter, "incomplete command envelope field: {field}")
            }
            Self::IncompleteApiError(field) => write!(formatter, "incomplete API error: {field}"),
            Self::IncompleteCredentialKeyMetadata => {
                formatter.write_str("incomplete credential/key metadata")
            }
            Self::MissingRuleId => formatter.write_str("rule id is required"),
            Self::InvalidRuleReasonCode(code) => write!(formatter, "invalid rule reason: {code}"),
            Self::MissingRuleCorrectionFields(rule) => {
                write!(formatter, "missing rule correction fields: {rule}")
            }
            Self::InvalidRuleEnforcement(enforcement) => {
                write!(formatter, "invalid rule enforcement: {enforcement}")
            }
        }
    }
}

impl std::error::Error for SharedSchemaPhase3ContractError {}

pub const PHASE4_CANONICAL_SCHEMA_SOURCE: &str =
    "packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json";
pub const PHASE4_MANIFEST_SOURCE: &str = "packages/schemas/overrid_contracts/codegen_manifest.json";
pub const PHASE4_BUILD_PLAN_SOURCE: &str =
    "docs/build_plan/sub_build_plan_007_shared_schema_package.md";
pub const PHASE4_TECH_STACK_SOURCE: &str = "docs/overrid_tech_stack_choice.md";
pub const PHASE4_RUST_OUTPUT_PATH: &str = "packages/schemas/overrid_contracts/src/lib.rs";
pub const PHASE4_GENERATED_DOCS_PATH: &str =
    "packages/schemas/overrid_contracts/generated/docs/shared_schema_package_phase4_reference.md";
pub const PHASE4_TYPESCRIPT_OUTPUT_ROOT: &str = "packages/schemas/admin_ui/generated";
pub const PHASE4_PROTOBUF_INTERNAL_ROOT: &str =
    "packages/schemas/overrid_contracts/protobuf/internal";

pub const REQUIRED_SHARED_SCHEMA_PHASE4_REASON_CODES: &[&str] = &[
    "schema.generation_not_reproducible",
    "schema.generated_output_hand_edited",
    "schema.docs_trace_missing",
    "schema.typescript_source_authority",
    "schema.protobuf_public_only",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedSchemaPhase4ProjectionTarget {
    Rust,
    Docs,
    TypeScriptWeb,
    ProtobufInternal,
}

impl SharedSchemaPhase4ProjectionTarget {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Docs => "docs",
            Self::TypeScriptWeb => "typescript_web",
            Self::ProtobufInternal => "protobuf_internal",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase4GenerationCommand {
    pub command_name: String,
    pub command: String,
    pub deterministic: bool,
    pub source_inputs: Vec<String>,
    pub output_paths: Vec<String>,
    pub dry_run_reason_code: String,
}

impl SharedSchemaPhase4GenerationCommand {
    pub fn new(
        command_name: impl Into<String>,
        command: impl Into<String>,
        source_inputs: Vec<String>,
        output_paths: Vec<String>,
        dry_run_reason_code: impl Into<String>,
    ) -> Self {
        Self {
            command_name: command_name.into(),
            command: command.into(),
            deterministic: true,
            source_inputs,
            output_paths,
            dry_run_reason_code: dry_run_reason_code.into(),
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase4ContractError> {
        if self.command_name.trim().is_empty() || self.command.trim().is_empty() {
            return Err(SharedSchemaPhase4ContractError::MissingGenerationCommand);
        }
        if !self.deterministic {
            return Err(SharedSchemaPhase4ContractError::NonDeterministicCommand(
                self.command_name.clone(),
            ));
        }
        if self.source_inputs.is_empty() || self.output_paths.is_empty() {
            return Err(
                SharedSchemaPhase4ContractError::IncompleteGenerationCommand(
                    self.command_name.clone(),
                ),
            );
        }
        if !self.dry_run_reason_code.contains('.') {
            return Err(SharedSchemaPhase4ContractError::MissingReasonCode(
                self.command_name.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase4ProjectionOutput {
    pub target: SharedSchemaPhase4ProjectionTarget,
    pub path: String,
    pub source_schema: String,
    pub non_authoritative: bool,
    pub validator_entrypoint: String,
    pub contains_redaction_metadata: bool,
    pub contains_reason_code_metadata: bool,
}

impl SharedSchemaPhase4ProjectionOutput {
    pub fn rust() -> Self {
        Self {
            target: SharedSchemaPhase4ProjectionTarget::Rust,
            path: PHASE4_RUST_OUTPUT_PATH.to_owned(),
            source_schema: PHASE4_CANONICAL_SCHEMA_SOURCE.to_owned(),
            non_authoritative: true,
            validator_entrypoint: "SharedSchemaPhase4GenerationContract::canonical().validate()"
                .to_owned(),
            contains_redaction_metadata: true,
            contains_reason_code_metadata: true,
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase4ContractError> {
        if self.source_schema != PHASE4_CANONICAL_SCHEMA_SOURCE {
            return Err(SharedSchemaPhase4ContractError::AuthorityDrift);
        }
        if !self.non_authoritative {
            return Err(
                SharedSchemaPhase4ContractError::GeneratedOutputAuthoritative(self.path.clone()),
            );
        }
        if self.target == SharedSchemaPhase4ProjectionTarget::Rust {
            if self.path != PHASE4_RUST_OUTPUT_PATH {
                return Err(SharedSchemaPhase4ContractError::MissingRustOutput);
            }
            if !self
                .validator_entrypoint
                .contains("SharedSchemaPhase4GenerationContract")
            {
                return Err(SharedSchemaPhase4ContractError::MissingRustValidator);
            }
            if !self.contains_redaction_metadata || !self.contains_reason_code_metadata {
                return Err(SharedSchemaPhase4ContractError::IncompleteRustOutput);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase4DocsProjection {
    pub output_path: String,
    pub source_to_doc_trace: bool,
    pub required_sections: Vec<String>,
    pub requires_descriptions: bool,
    pub requires_examples: bool,
    pub requires_privacy_class: bool,
    pub requires_reason_code_links: bool,
}

impl SharedSchemaPhase4DocsProjection {
    pub fn canonical() -> Self {
        Self {
            output_path: PHASE4_GENERATED_DOCS_PATH.to_owned(),
            source_to_doc_trace: true,
            required_sections: vec![
                "Object Families".to_owned(),
                "Required Fields".to_owned(),
                "Privacy And Redaction".to_owned(),
                "Reason Codes".to_owned(),
                "Compatibility And Authority".to_owned(),
            ],
            requires_descriptions: true,
            requires_examples: true,
            requires_privacy_class: true,
            requires_reason_code_links: true,
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase4ContractError> {
        if self.output_path != PHASE4_GENERATED_DOCS_PATH || !self.source_to_doc_trace {
            return Err(SharedSchemaPhase4ContractError::MissingDocsTrace);
        }
        for required in [
            "Object Families",
            "Required Fields",
            "Privacy And Redaction",
            "Reason Codes",
            "Compatibility And Authority",
        ] {
            if !self
                .required_sections
                .iter()
                .any(|section| section == required)
            {
                return Err(SharedSchemaPhase4ContractError::MissingDocsSection(
                    required,
                ));
            }
        }
        if !self.requires_descriptions
            || !self.requires_examples
            || !self.requires_privacy_class
            || !self.requires_reason_code_links
        {
            return Err(SharedSchemaPhase4ContractError::MissingDocsTrace);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase4TypeScriptProjection {
    pub status: String,
    pub source_schema: String,
    pub output_root: String,
    pub blocked_until: Vec<String>,
    pub browser_safe_redaction: bool,
    pub source_of_truth_allowed: bool,
}

impl SharedSchemaPhase4TypeScriptProjection {
    pub fn canonical() -> Self {
        Self {
            status: "blocked_until_rust_and_fixtures_stable".to_owned(),
            source_schema: PHASE4_CANONICAL_SCHEMA_SOURCE.to_owned(),
            output_root: PHASE4_TYPESCRIPT_OUTPUT_ROOT.to_owned(),
            blocked_until: vec![
                "rust_projection_validated".to_owned(),
                "golden_fixtures_stable".to_owned(),
                "docs_trace_validated".to_owned(),
                "compatibility_checks_stable".to_owned(),
            ],
            browser_safe_redaction: true,
            source_of_truth_allowed: false,
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase4ContractError> {
        if self.source_of_truth_allowed
            || self.status != "blocked_until_rust_and_fixtures_stable"
            || self.source_schema != PHASE4_CANONICAL_SCHEMA_SOURCE
        {
            return Err(SharedSchemaPhase4ContractError::TypeScriptSourceAuthority);
        }
        for required in [
            "rust_projection_validated",
            "golden_fixtures_stable",
            "docs_trace_validated",
            "compatibility_checks_stable",
        ] {
            if !self.blocked_until.iter().any(|gate| gate == required) {
                return Err(SharedSchemaPhase4ContractError::MissingTypeScriptGate(
                    required,
                ));
            }
        }
        if self.output_root != PHASE4_TYPESCRIPT_OUTPUT_ROOT || !self.browser_safe_redaction {
            return Err(SharedSchemaPhase4ContractError::TypeScriptSourceAuthority);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase4ProtobufProjection {
    pub status: String,
    pub scope: String,
    pub output_root: String,
    pub json_schema_source_required: bool,
    pub public_object_definition_allowed: bool,
    pub public_payload_families_blocked: Vec<String>,
}

impl SharedSchemaPhase4ProtobufProjection {
    pub fn canonical() -> Self {
        Self {
            status: "internal_only_when_owning_sds_approves".to_owned(),
            scope: "compact_internal_service_rpc_event_contracts".to_owned(),
            output_root: PHASE4_PROTOBUF_INTERNAL_ROOT.to_owned(),
            json_schema_source_required: true,
            public_object_definition_allowed: false,
            public_payload_families_blocked: vec![
                "command_payloads".to_owned(),
                "signed_payloads".to_owned(),
                "manifests".to_owned(),
                "policy_declarations".to_owned(),
                "fixtures".to_owned(),
                "docs_examples".to_owned(),
                "api_errors".to_owned(),
                "reason_codes".to_owned(),
                "audit_records".to_owned(),
            ],
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase4ContractError> {
        if self.status != "internal_only_when_owning_sds_approves"
            || self.scope != "compact_internal_service_rpc_event_contracts"
            || self.output_root != PHASE4_PROTOBUF_INTERNAL_ROOT
            || !self.json_schema_source_required
            || self.public_object_definition_allowed
        {
            return Err(SharedSchemaPhase4ContractError::ProtobufPublicAuthority);
        }
        for required in [
            "command_payloads",
            "signed_payloads",
            "manifests",
            "policy_declarations",
            "fixtures",
            "docs_examples",
            "api_errors",
            "reason_codes",
            "audit_records",
        ] {
            if !self
                .public_payload_families_blocked
                .iter()
                .any(|family| family == required)
            {
                return Err(SharedSchemaPhase4ContractError::ProtobufPublicAuthority);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase4Reproducibility {
    pub deterministic: bool,
    pub source_to_output_trace_required: bool,
    pub generated_diff_required: bool,
    pub hand_edit_policy: String,
    pub failure_reason_codes: Vec<String>,
}

impl SharedSchemaPhase4Reproducibility {
    pub fn canonical() -> Self {
        Self {
            deterministic: true,
            source_to_output_trace_required: true,
            generated_diff_required: true,
            hand_edit_policy: "prohibited".to_owned(),
            failure_reason_codes: REQUIRED_SHARED_SCHEMA_PHASE4_REASON_CODES
                .iter()
                .map(|reason| (*reason).to_owned())
                .collect(),
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase4ContractError> {
        if !self.deterministic
            || !self.source_to_output_trace_required
            || !self.generated_diff_required
            || self.hand_edit_policy != "prohibited"
        {
            return Err(SharedSchemaPhase4ContractError::GeneratedOutputHandEditAllowed);
        }
        for required in REQUIRED_SHARED_SCHEMA_PHASE4_REASON_CODES {
            if !self
                .failure_reason_codes
                .iter()
                .any(|reason| reason == required)
            {
                return Err(SharedSchemaPhase4ContractError::MissingReasonCode(
                    (*required).to_owned(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase4GenerationContract {
    pub schema_version: SchemaVersion,
    pub toolchain_name: String,
    pub canonical_source: String,
    pub source_of_truth: String,
    pub rust_first: bool,
    pub typescript_second: bool,
    pub generated_outputs_non_authoritative: bool,
    pub hand_edited_generated_files_allowed: bool,
    pub protobuf_public_authority_allowed: bool,
    pub generation_commands: Vec<SharedSchemaPhase4GenerationCommand>,
    pub source_hash_inputs: Vec<String>,
    pub rust_outputs: Vec<SharedSchemaPhase4ProjectionOutput>,
    pub docs_projection: SharedSchemaPhase4DocsProjection,
    pub typescript_web_projection: SharedSchemaPhase4TypeScriptProjection,
    pub protobuf_projection: SharedSchemaPhase4ProtobufProjection,
    pub reproducibility: SharedSchemaPhase4Reproducibility,
}

impl SharedSchemaPhase4GenerationContract {
    pub fn canonical() -> Result<Self, ContractError> {
        Ok(Self {
            schema_version: ensure_supported_shared_schema_package_schema_version(
                SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION,
            )?,
            toolchain_name: "rust-json-schema-projection-v0".to_owned(),
            canonical_source: PHASE4_CANONICAL_SCHEMA_SOURCE.to_owned(),
            source_of_truth: "json_schema".to_owned(),
            rust_first: true,
            typescript_second: true,
            generated_outputs_non_authoritative: true,
            hand_edited_generated_files_allowed: false,
            protobuf_public_authority_allowed: false,
            generation_commands: vec![
                SharedSchemaPhase4GenerationCommand::new(
                    "rust_projection_check",
                    "cargo test -p overrid-contracts shared_schema_phase4",
                    vec![
                        PHASE4_CANONICAL_SCHEMA_SOURCE.to_owned(),
                        PHASE4_MANIFEST_SOURCE.to_owned(),
                    ],
                    vec![PHASE4_RUST_OUTPUT_PATH.to_owned()],
                    "schema.generation_not_reproducible",
                ),
                SharedSchemaPhase4GenerationCommand::new(
                    "docs_trace_check",
                    "python3 scripts/validate_shared_schema_package_phase4.py",
                    vec![
                        PHASE4_CANONICAL_SCHEMA_SOURCE.to_owned(),
                        PHASE4_MANIFEST_SOURCE.to_owned(),
                        PHASE4_BUILD_PLAN_SOURCE.to_owned(),
                    ],
                    vec![PHASE4_GENERATED_DOCS_PATH.to_owned()],
                    "schema.docs_trace_missing",
                ),
            ],
            source_hash_inputs: vec![
                PHASE4_CANONICAL_SCHEMA_SOURCE.to_owned(),
                PHASE4_MANIFEST_SOURCE.to_owned(),
                PHASE4_BUILD_PLAN_SOURCE.to_owned(),
                PHASE4_TECH_STACK_SOURCE.to_owned(),
            ],
            rust_outputs: vec![SharedSchemaPhase4ProjectionOutput::rust()],
            docs_projection: SharedSchemaPhase4DocsProjection::canonical(),
            typescript_web_projection: SharedSchemaPhase4TypeScriptProjection::canonical(),
            protobuf_projection: SharedSchemaPhase4ProtobufProjection::canonical(),
            reproducibility: SharedSchemaPhase4Reproducibility::canonical(),
        })
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase4ContractError> {
        if self.toolchain_name != "rust-json-schema-projection-v0"
            || self.canonical_source != PHASE4_CANONICAL_SCHEMA_SOURCE
            || self.source_of_truth != "json_schema"
            || !self.rust_first
            || !self.typescript_second
            || !self.generated_outputs_non_authoritative
        {
            return Err(SharedSchemaPhase4ContractError::AuthorityDrift);
        }
        if self.hand_edited_generated_files_allowed {
            return Err(SharedSchemaPhase4ContractError::GeneratedOutputHandEditAllowed);
        }
        if self.protobuf_public_authority_allowed {
            return Err(SharedSchemaPhase4ContractError::ProtobufPublicAuthority);
        }
        for required in [
            PHASE4_CANONICAL_SCHEMA_SOURCE,
            PHASE4_MANIFEST_SOURCE,
            PHASE4_BUILD_PLAN_SOURCE,
            PHASE4_TECH_STACK_SOURCE,
        ] {
            if !self
                .source_hash_inputs
                .iter()
                .any(|input| input == required)
            {
                return Err(SharedSchemaPhase4ContractError::MissingSourceInput(
                    required,
                ));
            }
        }
        if self.generation_commands.len() < 2 {
            return Err(SharedSchemaPhase4ContractError::MissingGenerationCommand);
        }
        for command in &self.generation_commands {
            command.validate()?;
        }
        if !self.generation_commands.iter().any(|command| {
            command
                .command
                .contains("cargo test -p overrid-contracts shared_schema_phase4")
        }) {
            return Err(SharedSchemaPhase4ContractError::MissingGenerationCommand);
        }
        if self.rust_outputs.is_empty()
            || !self
                .rust_outputs
                .iter()
                .any(|output| output.target == SharedSchemaPhase4ProjectionTarget::Rust)
        {
            return Err(SharedSchemaPhase4ContractError::MissingRustOutput);
        }
        for output in &self.rust_outputs {
            output.validate()?;
        }
        self.docs_projection.validate()?;
        self.typescript_web_projection.validate()?;
        self.protobuf_projection.validate()?;
        self.reproducibility.validate()?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedSchemaPhase4ContractError {
    AuthorityDrift,
    GeneratedOutputHandEditAllowed,
    MissingSourceInput(&'static str),
    MissingGenerationCommand,
    NonDeterministicCommand(String),
    IncompleteGenerationCommand(String),
    MissingReasonCode(String),
    MissingRustOutput,
    MissingRustValidator,
    IncompleteRustOutput,
    GeneratedOutputAuthoritative(String),
    MissingDocsTrace,
    MissingDocsSection(&'static str),
    TypeScriptSourceAuthority,
    MissingTypeScriptGate(&'static str),
    ProtobufPublicAuthority,
}

impl fmt::Display for SharedSchemaPhase4ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthorityDrift => formatter.write_str("Phase 4 authority drift"),
            Self::GeneratedOutputHandEditAllowed => {
                formatter.write_str("generated output hand edits are allowed")
            }
            Self::MissingSourceInput(path) => write!(formatter, "missing source input: {path}"),
            Self::MissingGenerationCommand => formatter.write_str("missing generation command"),
            Self::NonDeterministicCommand(name) => {
                write!(formatter, "non-deterministic generation command: {name}")
            }
            Self::IncompleteGenerationCommand(name) => {
                write!(formatter, "incomplete generation command: {name}")
            }
            Self::MissingReasonCode(reason) => write!(formatter, "missing reason code: {reason}"),
            Self::MissingRustOutput => formatter.write_str("missing Rust output"),
            Self::MissingRustValidator => formatter.write_str("missing Rust validator entrypoint"),
            Self::IncompleteRustOutput => formatter.write_str("incomplete Rust output metadata"),
            Self::GeneratedOutputAuthoritative(path) => {
                write!(formatter, "generated output is authoritative: {path}")
            }
            Self::MissingDocsTrace => formatter.write_str("missing generated docs trace"),
            Self::MissingDocsSection(section) => {
                write!(formatter, "missing generated docs section: {section}")
            }
            Self::TypeScriptSourceAuthority => {
                formatter.write_str("TypeScript/web projection became source authority")
            }
            Self::MissingTypeScriptGate(gate) => {
                write!(
                    formatter,
                    "missing TypeScript/web blocked-until gate: {gate}"
                )
            }
            Self::ProtobufPublicAuthority => {
                formatter.write_str("Protobuf projection became public authority")
            }
        }
    }
}

impl std::error::Error for SharedSchemaPhase4ContractError {}

pub const PHASE5_CANONICAL_SCHEMA_SOURCE: &str =
    "packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json";
pub const PHASE5_MANIFEST_SOURCE: &str = "packages/schemas/overrid_contracts/codegen_manifest.json";
pub const PHASE5_BUILD_PLAN_SOURCE: &str =
    "docs/build_plan/sub_build_plan_007_shared_schema_package.md";
pub const PHASE5_TECH_STACK_SOURCE: &str = "docs/overrid_tech_stack_choice.md";
pub const PHASE5_RUST_OUTPUT_PATH: &str = "packages/schemas/overrid_contracts/src/lib.rs";
pub const PHASE5_VALIDATOR_SCRIPT: &str = "scripts/validate_shared_schema_package_phase5.py";
pub const PHASE5_SENTINEL_SECRET: &str = "OVERRID_SENTINEL_SECRET_NEVER_EMIT";

pub const REQUIRED_SHARED_SCHEMA_PHASE5_SENSITIVE_FAMILIES: &[&str] = &[
    "command",
    "identity",
    "tenant",
    "credential",
    "signature",
    "api_error",
    "audit",
    "policy",
    "usage",
    "oru",
    "seal_ledger",
    "overasset",
    "dispute",
    "namespace_ownership",
];

pub const REQUIRED_SHARED_SCHEMA_PHASE5_ENVELOPE_FAMILIES: &[&str] = &[
    "command",
    "event",
    "audit",
    "usage",
    "ledger",
    "public_response",
];

pub const REQUIRED_SHARED_SCHEMA_PHASE5_REASON_DOMAINS: &[&str] = &[
    "validation",
    "identity",
    "tenancy",
    "credentials",
    "policy",
    "queue",
    "execution",
    "accounting",
    "storage",
    "namespace",
    "ai",
    "compatibility",
];

pub const REQUIRED_SHARED_SCHEMA_PHASE5_PARSE_ERRORS: &[&str] = &[
    "schema.parse_malformed_payload",
    "schema.unsupported_version",
    "schema.missing_required_field",
    "schema.wrong_privacy_class",
    "schema.unknown_sensitive_field",
];

pub const REQUIRED_SHARED_SCHEMA_PHASE5_VALIDATION_REASON_CODES: &[&str] = &[
    "schema.unknown_sensitive_field",
    "schema.extension_map_not_permitted",
    "schema.parse_malformed_payload",
    "schema.unsupported_version",
    "schema.missing_required_field",
    "schema.wrong_privacy_class",
    "schema.envelope_incomplete",
    "schema.reason_code_undocumented",
    "schema.diagnostic_secret_leak",
];

fn owned_values(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase5ExtensionMapRule {
    pub surface: String,
    pub namespace_prefix_required: bool,
    pub typed_values_required: bool,
    pub privacy_class_required: bool,
    pub compatibility_class_required: bool,
    pub privacy_class: SharedSchemaPrivacyClass,
    pub compatibility_class: String,
}

impl SharedSchemaPhase5ExtensionMapRule {
    pub fn low_risk_metadata() -> Self {
        Self {
            surface: "low_risk_metadata".to_owned(),
            namespace_prefix_required: true,
            typed_values_required: true,
            privacy_class_required: true,
            compatibility_class_required: true,
            privacy_class: SharedSchemaPrivacyClass::Public,
            compatibility_class: "additive".to_owned(),
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase5ContractError> {
        if self.surface != "low_risk_metadata"
            || !self.namespace_prefix_required
            || !self.typed_values_required
            || !self.privacy_class_required
            || !self.compatibility_class_required
        {
            return Err(SharedSchemaPhase5ContractError::ExtensionMapRuleIncomplete(
                self.surface.clone(),
            ));
        }
        if self.privacy_class.requires_redaction() {
            return Err(SharedSchemaPhase5ContractError::ExtensionMapRuleIncomplete(
                self.surface.clone(),
            ));
        }
        if self.compatibility_class != "additive" && self.compatibility_class != "phase-gated" {
            return Err(SharedSchemaPhase5ContractError::ExtensionMapRuleIncomplete(
                self.surface.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase5StrictValidationDefaults {
    pub unknown_fields_rejected_for_sensitive_families: bool,
    pub extension_maps_default_denied: bool,
    pub sensitive_families: Vec<String>,
    pub allowed_extension_maps: Vec<SharedSchemaPhase5ExtensionMapRule>,
}

impl SharedSchemaPhase5StrictValidationDefaults {
    pub fn canonical() -> Self {
        Self {
            unknown_fields_rejected_for_sensitive_families: true,
            extension_maps_default_denied: true,
            sensitive_families: owned_values(REQUIRED_SHARED_SCHEMA_PHASE5_SENSITIVE_FAMILIES),
            allowed_extension_maps: vec![SharedSchemaPhase5ExtensionMapRule::low_risk_metadata()],
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase5ContractError> {
        if !self.unknown_fields_rejected_for_sensitive_families {
            return Err(SharedSchemaPhase5ContractError::UnknownSensitiveFieldsAllowed);
        }
        if !self.extension_maps_default_denied {
            return Err(SharedSchemaPhase5ContractError::ExtensionMapsDefaultAllowed);
        }
        for required in REQUIRED_SHARED_SCHEMA_PHASE5_SENSITIVE_FAMILIES {
            if !self
                .sensitive_families
                .iter()
                .any(|family| family == required)
            {
                return Err(SharedSchemaPhase5ContractError::MissingSensitiveFamily(
                    required,
                ));
            }
        }
        for rule in &self.allowed_extension_maps {
            rule.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase5ParseHelper {
    pub helper_name: String,
    pub schema_name: String,
    pub combines_schema_validation: bool,
    pub constructs_typed_object: bool,
    pub normalizes_errors: bool,
    pub checks_privacy_class: bool,
    pub references_reason_codes: bool,
    pub stable_error_reasons: Vec<String>,
}

impl SharedSchemaPhase5ParseHelper {
    pub fn canonical() -> Self {
        Self {
            helper_name: "parse_shared_schema_payload".to_owned(),
            schema_name: SHARED_SCHEMA_PACKAGE_SCHEMA_FAMILY.to_owned(),
            combines_schema_validation: true,
            constructs_typed_object: true,
            normalizes_errors: true,
            checks_privacy_class: true,
            references_reason_codes: true,
            stable_error_reasons: owned_values(REQUIRED_SHARED_SCHEMA_PHASE5_PARSE_ERRORS),
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase5ContractError> {
        if self.helper_name.trim().is_empty()
            || self.schema_name != SHARED_SCHEMA_PACKAGE_SCHEMA_FAMILY
            || !self.combines_schema_validation
            || !self.constructs_typed_object
            || !self.normalizes_errors
            || !self.checks_privacy_class
            || !self.references_reason_codes
        {
            return Err(SharedSchemaPhase5ContractError::IncompleteParseHelper(
                self.helper_name.clone(),
            ));
        }
        for required in REQUIRED_SHARED_SCHEMA_PHASE5_PARSE_ERRORS {
            if !self
                .stable_error_reasons
                .iter()
                .any(|reason| reason == required)
            {
                return Err(SharedSchemaPhase5ContractError::MissingStableErrorReason(
                    required,
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase5EnvelopeAssertion {
    pub envelope_family: String,
    pub required_fields: Vec<String>,
    pub required_refs: Vec<String>,
    pub reason_code: String,
}

impl SharedSchemaPhase5EnvelopeAssertion {
    pub fn new(
        envelope_family: impl Into<String>,
        required_fields: Vec<String>,
        required_refs: Vec<String>,
    ) -> Self {
        Self {
            envelope_family: envelope_family.into(),
            required_fields,
            required_refs,
            reason_code: "schema.envelope_incomplete".to_owned(),
        }
    }

    pub fn command() -> Self {
        Self::new(
            "command",
            owned_values(&[
                "tenant_id",
                "actor_id",
                "trace_id",
                "idempotency_key",
                "command_type",
                "timestamp",
                "schema_version",
                "signature_metadata",
            ]),
            owned_values(&["tenant_ref", "actor_ref", "signature_ref"]),
        )
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase5ContractError> {
        if !REQUIRED_SHARED_SCHEMA_PHASE5_ENVELOPE_FAMILIES
            .iter()
            .any(|family| self.envelope_family == *family)
        {
            return Err(SharedSchemaPhase5ContractError::IncompleteEnvelope(
                self.envelope_family.clone(),
            ));
        }
        if self.reason_code != "schema.envelope_incomplete" || self.required_fields.is_empty() {
            return Err(SharedSchemaPhase5ContractError::IncompleteEnvelope(
                self.envelope_family.clone(),
            ));
        }
        if self.envelope_family == "command" {
            for required in [
                "tenant_id",
                "actor_id",
                "trace_id",
                "idempotency_key",
                "command_type",
                "timestamp",
                "schema_version",
                "signature_metadata",
            ] {
                if !self.required_fields.iter().any(|field| field == required) {
                    return Err(SharedSchemaPhase5ContractError::MissingEnvelopeField {
                        envelope: self.envelope_family.clone(),
                        field: required,
                    });
                }
            }
            for required in ["tenant_ref", "actor_ref", "signature_ref"] {
                if !self.required_refs.iter().any(|field| field == required) {
                    return Err(SharedSchemaPhase5ContractError::MissingEnvelopeField {
                        envelope: self.envelope_family.clone(),
                        field: required,
                    });
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase5ReasonCodeRegistry {
    pub domain: String,
    pub reason_codes: Vec<String>,
    pub generated_enum_binding: String,
    pub free_form_replacements_allowed: bool,
}

impl SharedSchemaPhase5ReasonCodeRegistry {
    pub fn new(domain: impl Into<String>, reason_codes: Vec<String>) -> Self {
        Self {
            domain: domain.into(),
            reason_codes,
            generated_enum_binding: "SharedSchemaPhase5ReasonCode".to_owned(),
            free_form_replacements_allowed: false,
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase5ContractError> {
        if !REQUIRED_SHARED_SCHEMA_PHASE5_REASON_DOMAINS
            .iter()
            .any(|domain| self.domain == *domain)
        {
            return Err(SharedSchemaPhase5ContractError::MissingReasonDomain(
                self.domain.clone(),
            ));
        }
        if self.generated_enum_binding != "SharedSchemaPhase5ReasonCode"
            || self.free_form_replacements_allowed
        {
            return Err(SharedSchemaPhase5ContractError::FreeFormReasonCodesAllowed(
                self.domain.clone(),
            ));
        }
        if self.reason_codes.is_empty() {
            return Err(SharedSchemaPhase5ContractError::MissingReasonDomain(
                self.domain.clone(),
            ));
        }
        for code in &self.reason_codes {
            if !code.contains('.') {
                return Err(SharedSchemaPhase5ContractError::InvalidReasonCode(
                    code.clone(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase5RedactionDiagnostic {
    pub surface: String,
    pub redaction_hints: Vec<String>,
    pub sentinel_secret: String,
    pub private_payload_leak_allowed: bool,
    pub diagnostic_outputs: Vec<String>,
}

impl SharedSchemaPhase5RedactionDiagnostic {
    pub fn generated_docs() -> Self {
        Self {
            surface: "generated_docs".to_owned(),
            redaction_hints: owned_values(&[
                "secret_ref",
                "credential_ref",
                "signature_ref",
                "private_payload",
            ]),
            sentinel_secret: PHASE5_SENTINEL_SECRET.to_owned(),
            private_payload_leak_allowed: false,
            diagnostic_outputs: owned_values(&[
                "generated_docs",
                "validation_output",
                "fixture_report",
                "compatibility_report",
                "logs",
            ]),
        }
    }

    pub fn validation_output() -> Self {
        Self {
            surface: "validation_output".to_owned(),
            redaction_hints: owned_values(&[
                "user_content",
                "regulated_payload",
                "encrypted_private_payload",
            ]),
            sentinel_secret: PHASE5_SENTINEL_SECRET.to_owned(),
            private_payload_leak_allowed: false,
            diagnostic_outputs: owned_values(&["validation_output", "fixture_report", "logs"]),
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase5ContractError> {
        if self.private_payload_leak_allowed || self.sentinel_secret != PHASE5_SENTINEL_SECRET {
            return Err(SharedSchemaPhase5ContractError::DiagnosticLeakAllowed(
                self.surface.clone(),
            ));
        }
        if self.redaction_hints.is_empty() || self.diagnostic_outputs.is_empty() {
            return Err(SharedSchemaPhase5ContractError::MissingRedactionHint(
                self.surface.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase5RustProjection {
    pub path: String,
    pub validator_entrypoint: String,
    pub non_authoritative: bool,
}

impl SharedSchemaPhase5RustProjection {
    pub fn canonical() -> Self {
        Self {
            path: PHASE5_RUST_OUTPUT_PATH.to_owned(),
            validator_entrypoint: "SharedSchemaPhase5ValidationContract::canonical().validate()"
                .to_owned(),
            non_authoritative: true,
        }
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase5ContractError> {
        if self.path != PHASE5_RUST_OUTPUT_PATH
            || !self
                .validator_entrypoint
                .contains("SharedSchemaPhase5ValidationContract")
            || !self.non_authoritative
        {
            return Err(SharedSchemaPhase5ContractError::RustProjectionAuthorityDrift);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedSchemaPhase5ValidationContract {
    pub schema_version: SchemaVersion,
    pub strict_validation_defaults: SharedSchemaPhase5StrictValidationDefaults,
    pub parse_helpers: Vec<SharedSchemaPhase5ParseHelper>,
    pub envelope_assertions: Vec<SharedSchemaPhase5EnvelopeAssertion>,
    pub reason_code_registries: Vec<SharedSchemaPhase5ReasonCodeRegistry>,
    pub redaction_diagnostics: Vec<SharedSchemaPhase5RedactionDiagnostic>,
    pub source_hash_inputs: Vec<String>,
    pub rust_projection: SharedSchemaPhase5RustProjection,
}

impl SharedSchemaPhase5ValidationContract {
    pub fn canonical() -> Result<Self, ContractError> {
        Ok(Self {
            schema_version: ensure_supported_shared_schema_package_schema_version(
                SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION,
            )?,
            strict_validation_defaults: SharedSchemaPhase5StrictValidationDefaults::canonical(),
            parse_helpers: vec![SharedSchemaPhase5ParseHelper::canonical()],
            envelope_assertions: vec![
                SharedSchemaPhase5EnvelopeAssertion::command(),
                SharedSchemaPhase5EnvelopeAssertion::new(
                    "event",
                    owned_values(&[
                        "event_id",
                        "source_service",
                        "subject_id",
                        "sequence",
                        "occurred_at",
                        "privacy_class",
                        "schema_version",
                    ]),
                    owned_values(&["actor_ref", "evidence_ref"]),
                ),
                SharedSchemaPhase5EnvelopeAssertion::new(
                    "audit",
                    owned_values(&[
                        "audit_id",
                        "source_service",
                        "subject_id",
                        "sequence",
                        "actor_id",
                        "action",
                        "decision",
                        "privacy_class",
                        "schema_version",
                    ]),
                    owned_values(&["actor_ref", "policy_ref", "evidence_ref"]),
                ),
                SharedSchemaPhase5EnvelopeAssertion::new(
                    "usage",
                    owned_values(&[
                        "usage_id",
                        "tenant_id",
                        "actor_id",
                        "resource_ref",
                        "trace_id",
                        "metered_at",
                        "schema_version",
                    ]),
                    owned_values(&["tenant_ref", "resource_ref"]),
                ),
                SharedSchemaPhase5EnvelopeAssertion::new(
                    "ledger",
                    owned_values(&[
                        "ledger_ref",
                        "tenant_id",
                        "entry_sequence",
                        "reason_code",
                        "trace_id",
                        "schema_version",
                    ]),
                    owned_values(&["tenant_ref", "ledger_ref"]),
                ),
                SharedSchemaPhase5EnvelopeAssertion::new(
                    "public_response",
                    owned_values(&["request_id", "trace_id", "schema_version", "reason_code"]),
                    owned_values(&["evidence_ref"]),
                ),
            ],
            reason_code_registries: phase5_reason_code_registries(),
            redaction_diagnostics: vec![
                SharedSchemaPhase5RedactionDiagnostic::generated_docs(),
                SharedSchemaPhase5RedactionDiagnostic::validation_output(),
            ],
            source_hash_inputs: owned_values(&[
                PHASE5_CANONICAL_SCHEMA_SOURCE,
                PHASE5_MANIFEST_SOURCE,
                PHASE5_BUILD_PLAN_SOURCE,
                PHASE5_TECH_STACK_SOURCE,
            ]),
            rust_projection: SharedSchemaPhase5RustProjection::canonical(),
        })
    }

    pub fn validate(&self) -> Result<(), SharedSchemaPhase5ContractError> {
        self.strict_validation_defaults.validate()?;
        if self.parse_helpers.is_empty() {
            return Err(SharedSchemaPhase5ContractError::IncompleteParseHelper(
                "missing".to_owned(),
            ));
        }
        for helper in &self.parse_helpers {
            helper.validate()?;
        }
        for required in REQUIRED_SHARED_SCHEMA_PHASE5_ENVELOPE_FAMILIES {
            if !self
                .envelope_assertions
                .iter()
                .any(|assertion| assertion.envelope_family == *required)
            {
                return Err(SharedSchemaPhase5ContractError::MissingEnvelopeFamily(
                    required,
                ));
            }
        }
        for assertion in &self.envelope_assertions {
            assertion.validate()?;
        }
        for required in REQUIRED_SHARED_SCHEMA_PHASE5_REASON_DOMAINS {
            if !self
                .reason_code_registries
                .iter()
                .any(|registry| registry.domain == *required)
            {
                return Err(SharedSchemaPhase5ContractError::MissingReasonDomain(
                    (*required).to_owned(),
                ));
            }
        }
        for registry in &self.reason_code_registries {
            registry.validate()?;
        }
        let validation_registry = self
            .reason_code_registries
            .iter()
            .find(|registry| registry.domain == "validation")
            .ok_or_else(|| {
                SharedSchemaPhase5ContractError::MissingReasonDomain("validation".to_owned())
            })?;
        for required in REQUIRED_SHARED_SCHEMA_PHASE5_VALIDATION_REASON_CODES {
            if !validation_registry
                .reason_codes
                .iter()
                .any(|reason| reason == required)
            {
                return Err(SharedSchemaPhase5ContractError::InvalidReasonCode(
                    (*required).to_owned(),
                ));
            }
        }
        if self.redaction_diagnostics.is_empty() {
            return Err(SharedSchemaPhase5ContractError::MissingRedactionHint(
                "redaction_diagnostics".to_owned(),
            ));
        }
        for diagnostic in &self.redaction_diagnostics {
            diagnostic.validate()?;
        }
        for required in [
            PHASE5_CANONICAL_SCHEMA_SOURCE,
            PHASE5_MANIFEST_SOURCE,
            PHASE5_BUILD_PLAN_SOURCE,
            PHASE5_TECH_STACK_SOURCE,
        ] {
            if !self
                .source_hash_inputs
                .iter()
                .any(|input| input == required)
            {
                return Err(SharedSchemaPhase5ContractError::MissingSourceInput(
                    required,
                ));
            }
        }
        self.rust_projection.validate()?;
        Ok(())
    }
}

pub fn phase5_reason_code_registries() -> Vec<SharedSchemaPhase5ReasonCodeRegistry> {
    vec![
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "validation",
            owned_values(REQUIRED_SHARED_SCHEMA_PHASE5_VALIDATION_REASON_CODES),
        ),
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "identity",
            owned_values(&["identity.invalid_actor_ref"]),
        ),
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "tenancy",
            owned_values(&["tenant.missing_scope"]),
        ),
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "credentials",
            owned_values(&["credential.raw_material_rejected"]),
        ),
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "policy",
            owned_values(&["policy.denied_by_rule"]),
        ),
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "queue",
            owned_values(&["queue.invalid_transition"]),
        ),
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "execution",
            owned_values(&["execution.lease_missing"]),
        ),
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "accounting",
            owned_values(&["accounting.ledger_ref_missing"]),
        ),
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "storage",
            owned_values(&["storage.private_ref_required"]),
        ),
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "namespace",
            owned_values(&["namespace.ownership_ref_missing"]),
        ),
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "ai",
            owned_values(&["ai.private_context_redacted"]),
        ),
        SharedSchemaPhase5ReasonCodeRegistry::new(
            "compatibility",
            owned_values(&["compatibility.unsupported_schema_version"]),
        ),
    ]
}

pub fn list_reason_codes(domain: &str) -> Vec<String> {
    phase5_reason_code_registries()
        .into_iter()
        .find(|registry| registry.domain == domain)
        .map(|registry| registry.reason_codes)
        .unwrap_or_default()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedSchemaPhase5ContractError {
    UnknownSensitiveFieldsAllowed,
    ExtensionMapsDefaultAllowed,
    MissingSensitiveFamily(&'static str),
    ExtensionMapRuleIncomplete(String),
    IncompleteParseHelper(String),
    MissingStableErrorReason(&'static str),
    IncompleteEnvelope(String),
    MissingEnvelopeFamily(&'static str),
    MissingEnvelopeField {
        envelope: String,
        field: &'static str,
    },
    MissingReasonDomain(String),
    FreeFormReasonCodesAllowed(String),
    InvalidReasonCode(String),
    DiagnosticLeakAllowed(String),
    MissingRedactionHint(String),
    MissingSourceInput(&'static str),
    RustProjectionAuthorityDrift,
}

impl fmt::Display for SharedSchemaPhase5ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSensitiveFieldsAllowed => {
                formatter.write_str("unknown sensitive fields are allowed")
            }
            Self::ExtensionMapsDefaultAllowed => {
                formatter.write_str("extension maps are allowed by default")
            }
            Self::MissingSensitiveFamily(family) => {
                write!(formatter, "missing sensitive family: {family}")
            }
            Self::ExtensionMapRuleIncomplete(surface) => {
                write!(formatter, "extension map rule is incomplete: {surface}")
            }
            Self::IncompleteParseHelper(helper) => {
                write!(formatter, "incomplete parse helper: {helper}")
            }
            Self::MissingStableErrorReason(reason) => {
                write!(formatter, "missing stable parse error reason: {reason}")
            }
            Self::IncompleteEnvelope(envelope) => {
                write!(formatter, "incomplete envelope: {envelope}")
            }
            Self::MissingEnvelopeFamily(family) => {
                write!(formatter, "missing envelope family: {family}")
            }
            Self::MissingEnvelopeField { envelope, field } => {
                write!(formatter, "missing envelope field {field} on {envelope}")
            }
            Self::MissingReasonDomain(domain) => {
                write!(formatter, "missing reason domain: {domain}")
            }
            Self::FreeFormReasonCodesAllowed(domain) => {
                write!(formatter, "free-form reason codes allowed for {domain}")
            }
            Self::InvalidReasonCode(code) => {
                write!(formatter, "invalid or missing reason code: {code}")
            }
            Self::DiagnosticLeakAllowed(surface) => {
                write!(formatter, "diagnostic leak allowed for {surface}")
            }
            Self::MissingRedactionHint(surface) => {
                write!(formatter, "missing redaction hint for {surface}")
            }
            Self::MissingSourceInput(path) => write!(formatter, "missing source input: {path}"),
            Self::RustProjectionAuthorityDrift => {
                formatter.write_str("Phase 5 Rust projection authority drift")
            }
        }
    }
}

impl std::error::Error for SharedSchemaPhase5ContractError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedSchemaPackageContractError {
    MissingSourceRoot(&'static str),
    GeneratedOutputInsideSource,
    MissingCompatibilityReportRoot(&'static str),
    MissingInternalBinaryProjectionRoot(&'static str),
    MissingPrimitiveName,
    MissingObjectFamily,
    UntypedRefPrimitive(String),
    UnversionedPublicRef(String),
    MissingPrivacyClass(&'static str),
    InvalidLifecyclePrimitive(&'static str),
    InvalidReasonCode(String),
    MissingCorrectionFields(String),
    UndocumentedReasonCode(String),
    MissingUnsupportedVersionReason,
}

impl fmt::Display for SharedSchemaPackageContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSourceRoot(path) => write!(formatter, "missing source root: {path}"),
            Self::GeneratedOutputInsideSource => {
                formatter.write_str("generated output is inside schema source")
            }
            Self::MissingCompatibilityReportRoot(path) => {
                write!(formatter, "missing compatibility report root: {path}")
            }
            Self::MissingInternalBinaryProjectionRoot(path) => {
                write!(formatter, "missing internal binary projection root: {path}")
            }
            Self::MissingPrimitiveName => formatter.write_str("primitive name is required"),
            Self::MissingObjectFamily => formatter.write_str("object family is required"),
            Self::UntypedRefPrimitive(name) => write!(formatter, "untyped ref primitive: {name}"),
            Self::UnversionedPublicRef(name) => write!(formatter, "unversioned public ref: {name}"),
            Self::MissingPrivacyClass(class) => write!(formatter, "missing privacy class: {class}"),
            Self::InvalidLifecyclePrimitive(name) => {
                write!(formatter, "invalid lifecycle primitive: {name}")
            }
            Self::InvalidReasonCode(code) => write!(formatter, "invalid reason code: {code}"),
            Self::MissingCorrectionFields(code) => {
                write!(formatter, "missing correction fields: {code}")
            }
            Self::UndocumentedReasonCode(code) => {
                write!(formatter, "undocumented reason code: {code}")
            }
            Self::MissingUnsupportedVersionReason => {
                formatter.write_str("missing unsupported-version reason code")
            }
        }
    }
}

impl std::error::Error for SharedSchemaPackageContractError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentClass {
    Local,
    Seed,
    Staging,
    ProductionLike,
    Ci,
}

impl EnvironmentClass {
    pub fn parse(raw: &str) -> Result<Self, ProfileValidationError> {
        match raw {
            "local" => Ok(Self::Local),
            "seed" => Ok(Self::Seed),
            "staging" => Ok(Self::Staging),
            "production_like" => Ok(Self::ProductionLike),
            "ci" => Ok(Self::Ci),
            other => Err(ProfileValidationError::UnknownEnvironment(other.to_owned())),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Seed => "seed",
            Self::Staging => "staging",
            Self::ProductionLike => "production_like",
            Self::Ci => "ci",
        }
    }

    pub fn locks_endpoint_override(self) -> bool {
        matches!(self, Self::Seed | Self::ProductionLike)
    }

    pub fn requires_profile_confirmation(self) -> bool {
        matches!(self, Self::Seed | Self::ProductionLike)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialReferenceClass {
    Keychain,
    SecretService,
    EncryptedStore,
    SigningAgent,
    HardwareToken,
    Fixture,
    CiReference,
}

impl CredentialReferenceClass {
    pub fn parse(raw: &str) -> Result<Self, ProfileValidationError> {
        match raw {
            "keychain" => Ok(Self::Keychain),
            "secret_service" | "secret-service" => Ok(Self::SecretService),
            "encrypted_store" | "encrypted-store" => Ok(Self::EncryptedStore),
            "signing_agent" | "signing-agent" => Ok(Self::SigningAgent),
            "hardware_token" | "hardware-token" => Ok(Self::HardwareToken),
            "fixture" => Ok(Self::Fixture),
            "ci_reference" | "ci-reference" => Ok(Self::CiReference),
            other => Err(ProfileValidationError::UnknownCredentialClass(
                other.to_owned(),
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Keychain => "keychain",
            Self::SecretService => "secret_service",
            Self::EncryptedStore => "encrypted_store",
            Self::SigningAgent => "signing_agent",
            Self::HardwareToken => "hardware_token",
            Self::Fixture => "fixture",
            Self::CiReference => "ci_reference",
        }
    }

    pub fn is_fixture(self) -> bool {
        matches!(self, Self::Fixture)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureAllowance {
    Denied,
    LocalOnly,
    TestHarnessOnly,
}

impl FixtureAllowance {
    pub fn parse(raw: &str) -> Result<Self, ProfileValidationError> {
        match raw {
            "denied" => Ok(Self::Denied),
            "local_only" => Ok(Self::LocalOnly),
            "test_harness_only" => Ok(Self::TestHarnessOnly),
            other => Err(ProfileValidationError::UnknownFixtureAllowance(
                other.to_owned(),
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Denied => "denied",
            Self::LocalOnly => "local_only",
            Self::TestHarnessOnly => "test_harness_only",
        }
    }

    pub fn allows_fixture(self, environment: EnvironmentClass, test_harness_profile: bool) -> bool {
        match self {
            Self::Denied => false,
            Self::LocalOnly => environment == EnvironmentClass::Local,
            Self::TestHarnessOnly => {
                test_harness_profile && environment != EnvironmentClass::ProductionLike
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationPolicy {
    Standard,
    ConfirmSensitive,
    AlwaysConfirm,
}

impl ConfirmationPolicy {
    pub fn parse(raw: &str) -> Result<Self, ProfileValidationError> {
        match raw {
            "standard" => Ok(Self::Standard),
            "confirm_sensitive" => Ok(Self::ConfirmSensitive),
            "always_confirm" => Ok(Self::AlwaysConfirm),
            other => Err(ProfileValidationError::UnknownConfirmationPolicy(
                other.to_owned(),
            )),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::ConfirmSensitive => "confirm_sensitive",
            Self::AlwaysConfirm => "always_confirm",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandLifecycleState {
    Parsed,
    ProfileLoaded,
    CredentialReady,
    PayloadValidated,
    Signed,
    Submitted,
    Accepted,
    Waiting,
    Completed,
    Denied,
    Failed,
}

impl CommandLifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Parsed => "parsed",
            Self::ProfileLoaded => "profile_loaded",
            Self::CredentialReady => "credential_ready",
            Self::PayloadValidated => "payload_validated",
            Self::Signed => "signed",
            Self::Submitted => "submitted",
            Self::Accepted => "accepted",
            Self::Waiting => "waiting",
            Self::Completed => "completed",
            Self::Denied => "denied",
            Self::Failed => "failed",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Denied | Self::Failed)
    }
}

pub const ALL_COMMAND_LIFECYCLE_STATES: [CommandLifecycleState; 11] = [
    CommandLifecycleState::Parsed,
    CommandLifecycleState::ProfileLoaded,
    CommandLifecycleState::CredentialReady,
    CommandLifecycleState::PayloadValidated,
    CommandLifecycleState::Signed,
    CommandLifecycleState::Submitted,
    CommandLifecycleState::Accepted,
    CommandLifecycleState::Waiting,
    CommandLifecycleState::Completed,
    CommandLifecycleState::Denied,
    CommandLifecycleState::Failed,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandLifecycle {
    pub states: Vec<CommandLifecycleState>,
}

impl CommandLifecycle {
    pub fn new(states: Vec<CommandLifecycleState>) -> Self {
        Self { states }
    }

    pub fn terminal_state(&self) -> Option<CommandLifecycleState> {
        self.states
            .iter()
            .rev()
            .copied()
            .find(|state| state.is_terminal())
    }

    pub fn has_terminal_state(&self) -> bool {
        self.terminal_state().is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryClass {
    NotRetryable,
    SafeRetry,
    RetryAfter,
    OperatorReview,
}

impl RetryClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotRetryable => "not_retryable",
            Self::SafeRetry => "safe_retry",
            Self::RetryAfter => "retry_after",
            Self::OperatorReview => "operator_review",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCodeClass {
    Success,
    Usage,
    Config,
    Credential,
    Schema,
    Policy,
    Phase,
    Idempotency,
    Transport,
    Timeout,
    Platform,
    LocalIo,
}

impl ExitCodeClass {
    pub const fn code(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::Usage => 2,
            Self::Config => 3,
            Self::Credential => 4,
            Self::Schema => 5,
            Self::Policy => 6,
            Self::Phase => 7,
            Self::Idempotency => 8,
            Self::Transport => 9,
            Self::Timeout => 10,
            Self::Platform => 11,
            Self::LocalIo => 12,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Usage => "usage",
            Self::Config => "config",
            Self::Credential => "credential",
            Self::Schema => "schema",
            Self::Policy => "policy",
            Self::Phase => "phase",
            Self::Idempotency => "idempotency",
            Self::Transport => "transport",
            Self::Timeout => "timeout",
            Self::Platform => "platform",
            Self::LocalIo => "local_io",
        }
    }
}

pub const EXIT_CODE_REGISTRY: [ExitCodeClass; 12] = [
    ExitCodeClass::Success,
    ExitCodeClass::Usage,
    ExitCodeClass::Config,
    ExitCodeClass::Credential,
    ExitCodeClass::Schema,
    ExitCodeClass::Policy,
    ExitCodeClass::Phase,
    ExitCodeClass::Idempotency,
    ExitCodeClass::Transport,
    ExitCodeClass::Timeout,
    ExitCodeClass::Platform,
    ExitCodeClass::LocalIo,
];

pub fn exit_code_class_for_code(code: i32) -> Option<ExitCodeClass> {
    EXIT_CODE_REGISTRY
        .iter()
        .copied()
        .find(|exit_class| exit_class.code() == code)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilitySnapshot {
    pub route: String,
    pub available: bool,
    pub phase_gate: String,
    pub schema_versions: Vec<String>,
    pub stale_age_ms: u64,
    pub fail_closed: bool,
}

impl CapabilitySnapshot {
    pub fn local_phase_gate(route: impl Into<String>, phase_gate: impl Into<String>) -> Self {
        Self {
            route: route.into(),
            available: false,
            phase_gate: phase_gate.into(),
            schema_versions: vec![SUPPORTED_SCHEMA_VERSION.to_owned()],
            stale_age_ms: 0,
            fail_closed: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandContext {
    pub command_name: String,
    pub trace_context: TraceContext,
    pub lifecycle: CommandLifecycle,
    pub capability_snapshot: Option<CapabilitySnapshot>,
}

impl CommandContext {
    pub fn new(
        command_name: impl Into<String>,
        trace_id: impl Into<String>,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            command_name: command_name.into(),
            trace_context: TraceContext::new(trace_id, SUPPORTED_SCHEMA_VERSION)?,
            lifecycle: CommandLifecycle::new(vec![CommandLifecycleState::Parsed]),
            capability_snapshot: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliProfile {
    pub name: String,
    pub endpoint: String,
    pub endpoint_fingerprint: String,
    pub environment: EnvironmentClass,
    pub tenant_id: String,
    pub actor_id: String,
    pub credential_namespace: String,
    pub allowed_credential_classes: Vec<CredentialReferenceClass>,
    pub fixture_allowance: FixtureAllowance,
    pub default_output_mode: String,
    pub confirmation_policy: ConfirmationPolicy,
    pub schema_pins: Vec<String>,
    pub test_harness_profile: bool,
}

impl CliProfile {
    pub fn validate(&self) -> Result<(), ProfileValidationError> {
        require_non_empty(&self.name, "profile name")?;
        require_non_empty(&self.endpoint, "endpoint")?;
        require_non_empty(&self.endpoint_fingerprint, "endpoint fingerprint")?;
        require_non_empty(&self.tenant_id, "tenant id")?;
        require_non_empty(&self.actor_id, "actor id")?;
        require_non_empty(&self.credential_namespace, "credential namespace")?;

        if self.allowed_credential_classes.is_empty() {
            return Err(ProfileValidationError::MissingCredentialClass);
        }

        match self.default_output_mode.as_str() {
            "human" | "json" => {}
            other => return Err(ProfileValidationError::InvalidOutputMode(other.to_owned())),
        }

        if self.schema_pins.is_empty() {
            return Err(ProfileValidationError::MissingSchemaPin);
        }
        for schema_pin in &self.schema_pins {
            ensure_supported_schema_version(schema_pin)
                .map_err(ProfileValidationError::UnsupportedSchemaPin)?;
        }

        let fixture_allowed = self
            .fixture_allowance
            .allows_fixture(self.environment, self.test_harness_profile);
        if self
            .allowed_credential_classes
            .iter()
            .any(|class| class.is_fixture())
            && !fixture_allowed
        {
            return Err(ProfileValidationError::FixtureCredentialNotAllowed {
                environment: self.environment.as_str(),
            });
        }

        if self.environment.locks_endpoint_override()
            && self.endpoint_fingerprint.eq_ignore_ascii_case("unknown")
        {
            return Err(ProfileValidationError::MissingEndpointFingerprint);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialReference {
    pub reference_id: String,
    pub class: CredentialReferenceClass,
    pub namespace: String,
    pub key_id: String,
    pub revoked: bool,
    pub expired: bool,
}

impl CredentialReference {
    pub fn validate_for_profile(&self, profile: &CliProfile) -> Result<(), ProfileValidationError> {
        profile.validate()?;
        require_non_empty(&self.reference_id, "credential reference")?;
        require_non_empty(&self.namespace, "credential namespace")?;
        require_non_empty(&self.key_id, "key id")?;

        for value in [&self.reference_id, &self.namespace, &self.key_id] {
            if contains_raw_secret_marker(value) {
                return Err(ProfileValidationError::RawKeyMaterialInReference);
            }
        }

        if self.namespace != profile.credential_namespace {
            return Err(ProfileValidationError::CredentialNamespaceMismatch {
                expected: profile.credential_namespace.clone(),
                actual: self.namespace.clone(),
            });
        }

        if !profile.allowed_credential_classes.contains(&self.class) {
            return Err(ProfileValidationError::CredentialClassNotAllowed {
                class: self.class.as_str(),
            });
        }

        if self.class.is_fixture()
            && !profile
                .fixture_allowance
                .allows_fixture(profile.environment, profile.test_harness_profile)
        {
            return Err(ProfileValidationError::FixtureCredentialNotAllowed {
                environment: profile.environment.as_str(),
            });
        }

        if self.revoked {
            return Err(ProfileValidationError::CredentialRevoked);
        }
        if self.expired {
            return Err(ProfileValidationError::CredentialExpired);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignerHandoff {
    pub credential_reference_id: String,
    pub key_id: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub signature_ref: String,
    pub exposes_key_material: bool,
}

impl SignerHandoff {
    pub fn new(
        profile: &CliProfile,
        credential: &CredentialReference,
        command_type: &str,
    ) -> Result<Self, ProfileValidationError> {
        require_non_empty(command_type, "command type")?;
        credential.validate_for_profile(profile)?;
        let normalized_command = command_type.replace([' ', '/'], "_");
        Ok(Self {
            credential_reference_id: credential.reference_id.clone(),
            key_id: credential.key_id.clone(),
            tenant_id: profile.tenant_id.clone(),
            actor_id: profile.actor_id.clone(),
            signature_ref: format!(
                "sigref:{}:{}:{}",
                credential.namespace, credential.key_id, normalized_command
            ),
            exposes_key_material: false,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootstrapCommandFamily {
    Auth,
    Tenant,
    Identity,
    Key,
    Manifest,
    Workload,
    Node,
    Policy,
    Package,
    Usage,
    Receipt,
    Ledger,
    Dispute,
}

impl BootstrapCommandFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auth => "auth",
            Self::Tenant => "tenant",
            Self::Identity => "identity",
            Self::Key => "key",
            Self::Manifest => "manifest",
            Self::Workload => "workload",
            Self::Node => "node",
            Self::Policy => "policy",
            Self::Package => "package",
            Self::Usage => "usage",
            Self::Receipt => "receipt",
            Self::Ledger => "ledger",
            Self::Dispute => "dispute",
        }
    }

    pub fn phase_gate(self) -> &'static str {
        match self {
            Self::Node => "phase_2_seed_private_swarm",
            Self::Policy => "phase_4_trust_policy_verification",
            Self::Package => "phase_9_overpack_deployment_platform",
            Self::Usage | Self::Receipt | Self::Ledger | Self::Dispute => {
                "phase_5_metering_accounting"
            }
            _ => "phase_1_control_plane_bootstrap",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedCommandEnvelope {
    pub family: BootstrapCommandFamily,
    pub command_type: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub target_ref: String,
    pub payload_type: String,
    pub expected_state: Option<String>,
    pub reason: Option<String>,
    pub idempotency: IdempotencyRecord,
    pub trace_context: TraceContext,
    pub signature_ref: String,
    pub exposes_key_material: bool,
}

impl SignedCommandEnvelope {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        family: BootstrapCommandFamily,
        command_type: impl Into<String>,
        tenant_id: impl Into<String>,
        actor_id: impl Into<String>,
        target_ref: impl Into<String>,
        payload_type: impl Into<String>,
        expected_state: Option<String>,
        reason: Option<String>,
        idempotency_key: impl Into<String>,
        trace_id: impl Into<String>,
        signature_ref: impl Into<String>,
    ) -> Result<Self, ContractError> {
        let command_type = command_type.into();
        Ok(Self {
            family,
            tenant_id: tenant_id.into(),
            actor_id: actor_id.into(),
            target_ref: target_ref.into(),
            payload_type: payload_type.into(),
            expected_state,
            reason,
            idempotency: IdempotencyRecord::new(
                idempotency_key,
                command_type.clone(),
                SUPPORTED_SCHEMA_VERSION,
            )?,
            trace_context: TraceContext::new(trace_id, SUPPORTED_SCHEMA_VERSION)?,
            signature_ref: signature_ref.into(),
            command_type,
            exposes_key_material: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapAcceptanceRecord {
    pub command_type: String,
    pub accepted_ref: String,
    pub phase_gate: String,
    pub pending_state: String,
    pub audit_refs: Vec<String>,
}

impl BootstrapAcceptanceRecord {
    pub fn new(
        command_type: impl Into<String>,
        accepted_ref: impl Into<String>,
        pending_state: impl Into<String>,
        audit_refs: Vec<String>,
    ) -> Self {
        Self {
            command_type: command_type.into(),
            accepted_ref: accepted_ref.into(),
            phase_gate: "phase_1_control_plane_bootstrap".to_owned(),
            pending_state: pending_state.into(),
            audit_refs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestBootstrapRef {
    pub manifest_ref: String,
    pub manifest_kind: String,
    pub immutable_ref: String,
    pub submitted_via: String,
}

impl ManifestBootstrapRef {
    pub fn new(manifest_ref: impl Into<String>, manifest_kind: impl Into<String>) -> Self {
        let manifest_ref = manifest_ref.into();
        let manifest_kind = manifest_kind.into();
        Self {
            immutable_ref: format!("manifest:{}:immutable", manifest_ref),
            manifest_ref,
            manifest_kind,
            submitted_via: "sdk_overgate_contract".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntheticWorkloadPendingState {
    pub workload_ref: String,
    pub workload_kind: String,
    pub queue_state: String,
    pub execution_implied: bool,
    pub timeline_refs: Vec<String>,
}

impl SyntheticWorkloadPendingState {
    pub fn pending(workload_ref: impl Into<String>, workload_kind: impl Into<String>) -> Self {
        let workload_ref = workload_ref.into();
        Self {
            timeline_refs: vec![format!("timeline:{}:pending", workload_ref)],
            workload_ref,
            workload_kind: workload_kind.into(),
            queue_state: "pending".to_owned(),
            execution_implied: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Live,
    Stale,
    Expired,
    Draining,
    Disabled,
}

impl NodeState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Draining => "draining",
            Self::Disabled => "disabled",
        }
    }

    pub fn accepts_work(self) -> bool {
        matches!(self, Self::Live)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeStatusRecord {
    pub node_ref: String,
    pub state: NodeState,
    pub profile_name: String,
    pub credential_ref: String,
    pub credential_checked: bool,
    pub capability_refs: Vec<String>,
    pub heartbeat_ref: String,
    pub registered_via: String,
    pub direct_node_access: bool,
}

impl NodeStatusRecord {
    pub fn new(
        node_ref: impl Into<String>,
        state: NodeState,
        profile_name: impl Into<String>,
        credential_ref: impl Into<String>,
    ) -> Self {
        let node_ref = node_ref.into();
        Self {
            heartbeat_ref: format!("overwatch:heartbeat:{node_ref}"),
            capability_refs: vec![
                format!("overcell:node:{node_ref}:capabilities"),
                format!("overregistry:node:{node_ref}:profile"),
            ],
            node_ref,
            state,
            profile_name: profile_name.into(),
            credential_ref: credential_ref.into(),
            credential_checked: true,
            registered_via: "sdk_overgate_contract".to_owned(),
            direct_node_access: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadExecutionState {
    Scheduled,
    Leased,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
    DeadLettered,
}

impl WorkloadExecutionState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Scheduled => "scheduled",
            Self::Leased => "leased",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::DeadLettered => "dead_lettered",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded | Self::Failed | Self::Cancelled | Self::TimedOut | Self::DeadLettered
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionDiagnosticEvent {
    pub state: WorkloadExecutionState,
    pub service_ref: String,
    pub reason_code: String,
    pub evidence_ref: String,
}

impl ExecutionDiagnosticEvent {
    pub fn new(
        state: WorkloadExecutionState,
        service_ref: impl Into<String>,
        reason_code: impl Into<String>,
        evidence_ref: impl Into<String>,
    ) -> Self {
        Self {
            state,
            service_ref: service_ref.into(),
            reason_code: reason_code.into(),
            evidence_ref: evidence_ref.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionTimeline {
    pub workload_ref: String,
    pub states: Vec<WorkloadExecutionState>,
    pub diagnostic_events: Vec<ExecutionDiagnosticEvent>,
    pub owning_service_refs: Vec<String>,
    pub trace_id: String,
    pub direct_node_access: bool,
}

impl ExecutionTimeline {
    pub fn new(
        workload_ref: impl Into<String>,
        states: Vec<WorkloadExecutionState>,
        trace_id: impl Into<String>,
    ) -> Self {
        let workload_ref = workload_ref.into();
        Self {
            diagnostic_events: vec![
                ExecutionDiagnosticEvent::new(
                    WorkloadExecutionState::Scheduled,
                    "oversched:scheduler",
                    "scheduler.accepted",
                    format!("trace:{workload_ref}:scheduler"),
                ),
                ExecutionDiagnosticEvent::new(
                    WorkloadExecutionState::Leased,
                    "overlease:lease",
                    "lease.active",
                    format!("trace:{workload_ref}:lease"),
                ),
                ExecutionDiagnosticEvent::new(
                    WorkloadExecutionState::Running,
                    "overrun:runner",
                    "runner.started",
                    format!("trace:{workload_ref}:runner"),
                ),
                ExecutionDiagnosticEvent::new(
                    WorkloadExecutionState::Running,
                    "overcell:node-heartbeat",
                    "node.heartbeat.live",
                    format!("trace:{workload_ref}:node_heartbeat"),
                ),
                ExecutionDiagnosticEvent::new(
                    WorkloadExecutionState::Running,
                    "overpack:package",
                    "package.resolved",
                    format!("trace:{workload_ref}:package"),
                ),
                ExecutionDiagnosticEvent::new(
                    states
                        .last()
                        .copied()
                        .unwrap_or(WorkloadExecutionState::Succeeded),
                    "overstore:result-state-ref",
                    result_state_reason_code(
                        states
                            .last()
                            .copied()
                            .unwrap_or(WorkloadExecutionState::Succeeded),
                    ),
                    format!("trace:{workload_ref}:result_state"),
                ),
            ],
            owning_service_refs: vec![
                "overgate:execution-command".to_owned(),
                "overqueue:workload-state".to_owned(),
                "oversched:scheduler".to_owned(),
                "overlease:lease".to_owned(),
                "overrun:runner".to_owned(),
                "overcell:node-heartbeat".to_owned(),
                "overpack:package".to_owned(),
                "overstore:result-state-ref".to_owned(),
                "overwatch:trace".to_owned(),
            ],
            workload_ref,
            states,
            trace_id: trace_id.into(),
            direct_node_access: false,
        }
    }
}

fn result_state_reason_code(state: WorkloadExecutionState) -> &'static str {
    match state {
        WorkloadExecutionState::Scheduled => "result.scheduled",
        WorkloadExecutionState::Leased => "result.leased",
        WorkloadExecutionState::Running => "result.pending",
        WorkloadExecutionState::Succeeded => "result.ref.available",
        WorkloadExecutionState::Failed => "result.failed",
        WorkloadExecutionState::Cancelled => "result.cancelled",
        WorkloadExecutionState::TimedOut => "result.timed_out",
        WorkloadExecutionState::DeadLettered => "result.dead_lettered",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionLogBundle {
    pub workload_ref: String,
    pub log_ref: String,
    pub redaction_policy: String,
    pub bounded_streaming: bool,
    pub trace_linked_ref: String,
    pub contains_private_payload: bool,
    pub direct_node_path_exposed: bool,
}

impl ExecutionLogBundle {
    pub fn new(workload_ref: impl Into<String>, trace_id: impl Into<String>) -> Self {
        let workload_ref = workload_ref.into();
        Self {
            log_ref: format!("overwatch:logs:{workload_ref}:redacted"),
            trace_linked_ref: format!("trace:{}:logs", trace_id.into()),
            workload_ref,
            redaction_policy: "secret_free_refs_only".to_owned(),
            bounded_streaming: true,
            contains_private_payload: false,
            direct_node_path_exposed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResultRef {
    pub workload_ref: String,
    pub result_ref: String,
    pub authorized_control_plane_ref: String,
    pub trace_linked_ref: String,
    pub contains_private_payload: bool,
    pub direct_object_store_path_exposed: bool,
}

impl ExecutionResultRef {
    pub fn new(workload_ref: impl Into<String>, trace_id: impl Into<String>) -> Self {
        let workload_ref = workload_ref.into();
        Self {
            result_ref: format!("overstore:result:{workload_ref}:authorized_ref"),
            authorized_control_plane_ref: format!("overgate:result:{workload_ref}"),
            trace_linked_ref: format!("trace:{}:result", trace_id.into()),
            workload_ref,
            contains_private_payload: false,
            direct_object_store_path_exposed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PollingPlan {
    pub wait: bool,
    pub follow: bool,
    pub timeout_ms: u64,
    pub poll_interval_ms: u64,
    pub event_stream_preferred: bool,
    pub fallback_polling: bool,
    pub interruptible: bool,
}

impl PollingPlan {
    pub fn bounded(wait: bool, follow: bool, timeout_ms: u64, poll_interval_ms: u64) -> Self {
        Self {
            wait,
            follow,
            timeout_ms: timeout_ms.clamp(1, 600_000),
            poll_interval_ms: poll_interval_ms.clamp(100, 60_000),
            event_stream_preferred: true,
            fallback_polling: true,
            interruptible: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDryRunDecisionState {
    Accepted,
    Denied,
}

impl PolicyDryRunDecisionState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Denied => "denied",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyDryRunDecision {
    pub target_ref: String,
    pub decision: PolicyDryRunDecisionState,
    pub reason_codes: Vec<String>,
    pub workload_class: String,
    pub data_sensitivity: String,
    pub quota_ref: String,
    pub package_trust_ref: String,
    pub egress_policy_ref: String,
    pub provider_eligibility_ref: String,
    pub budget_ref: String,
    pub evaluated_via: String,
    pub mutates_platform_state: bool,
    pub direct_policy_service_access: bool,
}

impl PolicyDryRunDecision {
    pub fn new(target_ref: impl Into<String>, reason_codes: Vec<String>) -> Self {
        let target_ref = target_ref.into();
        let decision = if reason_codes.is_empty() {
            PolicyDryRunDecisionState::Accepted
        } else {
            PolicyDryRunDecisionState::Denied
        };
        Self {
            quota_ref: format!("overmeter:quota:{target_ref}"),
            package_trust_ref: format!("overguard:package_trust:{target_ref}"),
            egress_policy_ref: format!("overguard:egress:{target_ref}"),
            provider_eligibility_ref: format!("overguard:provider:{target_ref}"),
            budget_ref: format!("oru:budget:{target_ref}"),
            target_ref,
            decision,
            reason_codes,
            workload_class: "standard_compute".to_owned(),
            data_sensitivity: "low_sensitivity".to_owned(),
            evaluated_via: "sdk_overgate_contract".to_owned(),
            mutates_platform_state: false,
            direct_policy_service_access: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageValidationState {
    Accepted,
    InvalidPackage,
    UnsupportedVersion,
    MissingProvenance,
    PolicyIncompatible,
}

impl PackageValidationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::InvalidPackage => "invalid_package",
            Self::UnsupportedVersion => "unsupported_version",
            Self::MissingProvenance => "missing_provenance",
            Self::PolicyIncompatible => "policy_incompatible",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageValidationSummary {
    pub package_ref: String,
    pub validation_state: PackageValidationState,
    pub reason_codes: Vec<String>,
    pub schema_checked: bool,
    pub signature_checked: bool,
    pub hash_checked: bool,
    pub dependency_checked: bool,
    pub permission_checked: bool,
    pub provenance_available: bool,
    pub sbom_ref: String,
    pub policy_compatibility_checked: bool,
    pub submitted_via: String,
    pub direct_package_store_access: bool,
}

impl PackageValidationSummary {
    pub fn new(
        package_ref: impl Into<String>,
        validation_state: PackageValidationState,
        reason_codes: Vec<String>,
    ) -> Self {
        let package_ref = package_ref.into();
        Self {
            sbom_ref: format!("overpack:sbom:{package_ref}"),
            package_ref,
            validation_state,
            reason_codes,
            schema_checked: true,
            signature_checked: true,
            hash_checked: true,
            dependency_checked: true,
            permission_checked: true,
            provenance_available: !matches!(
                validation_state,
                PackageValidationState::MissingProvenance
            ),
            policy_compatibility_checked: true,
            submitted_via: "sdk_overgate_contract".to_owned(),
            direct_package_store_access: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageOruRollup {
    pub tenant_ref: String,
    pub usage_ref: String,
    pub units: Vec<String>,
    pub budget_state: String,
    pub disputed_usage: bool,
    pub read_via: String,
    pub payment_behavior_created: bool,
    pub direct_meter_access: bool,
}

impl UsageOruRollup {
    pub fn new(tenant_ref: impl Into<String>, usage_ref: impl Into<String>) -> Self {
        let usage_ref = usage_ref.into();
        let normalized = usage_ref.to_ascii_lowercase();
        Self {
            tenant_ref: tenant_ref.into(),
            usage_ref,
            units: [
                "CPU-ORU", "GPU-ORU", "STOR-ORU", "NET-ORU", "MEM-ORU", "DATA-ORU",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            budget_state: if normalized.contains("budget_exhausted") {
                "budget_exhausted"
            } else {
                "within_budget"
            }
            .to_owned(),
            disputed_usage: normalized.contains("disputed"),
            read_via: "sdk_overgate_contract".to_owned(),
            payment_behavior_created: false,
            direct_meter_access: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptLedgerRead {
    pub receipt_ref: String,
    pub ledger_refs: Vec<String>,
    pub invoice_status: String,
    pub refund_ref: String,
    pub correction_ref: String,
    pub payout_hold_ref: String,
    pub audit_refs: Vec<String>,
    pub read_via: String,
    pub pricing_assumptions_present: bool,
    pub revenue_assumptions_present: bool,
    pub customer_count_assumptions_present: bool,
    pub market_volume_assumptions_present: bool,
    pub direct_ledger_access: bool,
}

impl ReceiptLedgerRead {
    pub fn new(receipt_ref: impl Into<String>) -> Self {
        let receipt_ref = receipt_ref.into();
        Self {
            ledger_refs: vec![
                format!("seal-ledger:entry:{receipt_ref}"),
                format!("overbill:receipt:{receipt_ref}"),
            ],
            refund_ref: format!("overbill:refund:{receipt_ref}"),
            correction_ref: format!("seal-ledger:correction:{receipt_ref}"),
            payout_hold_ref: format!("seal-ledger:payout-hold:{receipt_ref}"),
            audit_refs: vec![format!("overwatch:audit:{receipt_ref}")],
            receipt_ref,
            invoice_status: "issued".to_owned(),
            read_via: "sdk_overgate_contract".to_owned(),
            pricing_assumptions_present: false,
            revenue_assumptions_present: false,
            customer_count_assumptions_present: false,
            market_volume_assumptions_present: false,
            direct_ledger_access: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisputeReadModel {
    pub dispute_ref: String,
    pub case_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub hold_status: String,
    pub correction_refs: Vec<String>,
    pub resolution_state: String,
    pub tenant_role_filtered: bool,
    pub read_via: String,
    pub direct_dispute_mutation: bool,
    pub direct_ledger_mutation: bool,
}

impl DisputeReadModel {
    pub fn new(dispute_ref: impl Into<String>) -> Self {
        let dispute_ref = dispute_ref.into();
        let normalized = dispute_ref.to_ascii_lowercase();
        Self {
            case_refs: vec![
                format!("overclaim:case:{dispute_ref}:primary"),
                format!("overclaim:case:{dispute_ref}:appeal"),
            ],
            evidence_refs: vec![format!("overclaim:evidence:{dispute_ref}:bundle")],
            hold_status: if normalized.contains("released") {
                "released"
            } else {
                "held"
            }
            .to_owned(),
            correction_refs: vec![format!("seal-ledger:correction:{dispute_ref}")],
            resolution_state: if normalized.contains("resolved") {
                "resolved"
            } else {
                "open"
            }
            .to_owned(),
            dispute_ref,
            tenant_role_filtered: true,
            read_via: "sdk_overgate_contract".to_owned(),
            direct_dispute_mutation: false,
            direct_ledger_mutation: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductKind {
    Docdex,
    Mcoda,
    Codali,
}

impl ProductKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Docdex => "docdex",
            Self::Mcoda => "mcoda",
            Self::Codali => "codali",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductWorkflowRecipe {
    pub product: ProductKind,
    pub workflow_ref: String,
    pub workload_kind: String,
    pub command_recipes: Vec<String>,
    pub required_refs: Vec<String>,
    pub expected_failure_modes: Vec<String>,
    pub safe_retry_patterns: Vec<String>,
    pub submitted_via: String,
    pub sdk_overgate_only: bool,
    pub authorized_refs_only: bool,
    pub secret_free_json_output: bool,
    pub dynamic_model_resource_selection: bool,
    pub direct_internal_api_access: bool,
    pub direct_storage_access: bool,
    pub raw_http_required: bool,
    pub hardcoded_model_or_provider: bool,
    pub hardcoded_node_assumption: bool,
    pub paid_service_assumption: bool,
}

impl ProductWorkflowRecipe {
    pub fn new(
        product: ProductKind,
        workflow_ref: impl Into<String>,
        workload_kind: impl Into<String>,
    ) -> Self {
        let workflow_ref = workflow_ref.into();
        let workload_kind = workload_kind.into();
        let (command_recipes, required_refs, expected_failure_modes, safe_retry_patterns) =
            match product {
                ProductKind::Docdex => (
                    vec![
                        format!(
                            "overrid workload submit --workload-kind {workload_kind} --workload-ref {workflow_ref} --json"
                        ),
                        format!("overrid workload status --workload-ref {workflow_ref} --json"),
                        format!("overrid workload result --workload-ref {workflow_ref} --json"),
                        format!(
                            "overrid workload cancel --workload-ref {workflow_ref} --reason operator_requested --json"
                        ),
                        format!("overrid usage show --target-ref usage:{workflow_ref} --json"),
                        format!("overrid receipt show --target-ref receipt:{workflow_ref} --json"),
                    ],
                    vec![
                        "encrypted_index_ref",
                        "retrieval_job_ref",
                        "search_result_ref",
                        "usage_rollup_ref",
                        "receipt_ref",
                    ],
                    vec![
                        "policy.egress_denied",
                        "budget.exhausted",
                        "credential.scope_denied",
                        "result.cancelled",
                    ],
                    vec![
                        "retry_same_idempotency_key_for_transport",
                        "resume_with_workload_ref",
                        "cancel_then_resubmit_after_policy_fix",
                    ],
                ),
                ProductKind::Mcoda => (
                    vec![
                        format!(
                            "overrid workload submit --workload-kind {workload_kind} --workload-ref {workflow_ref} --json"
                        ),
                        format!("overrid workload status --workload-ref {workflow_ref} --json"),
                        format!("overrid workload logs --workload-ref {workflow_ref} --json"),
                        format!("overrid workload result --workload-ref {workflow_ref} --json"),
                        format!(
                            "overrid workload cancel --workload-ref {workflow_ref} --reason operator_requested --json"
                        ),
                        format!("overrid usage show --target-ref usage:{workflow_ref} --json"),
                    ],
                    vec![
                        "agent_workload_ref",
                        "dynamic_model_metadata_ref",
                        "resource_metadata_ref",
                        "tool_boundary_ref",
                        "budget_ref",
                        "usage_rollup_ref",
                    ],
                    vec![
                        "policy.unsupported_workload_class",
                        "budget.exhausted",
                        "tool.boundary_denied",
                        "provider.unavailable",
                    ],
                    vec![
                        "retry_same_idempotency_key_for_safe_transport",
                        "switch_model_via_metadata_ref",
                        "operator_review_for_policy_denial",
                    ],
                ),
                ProductKind::Codali => (
                    vec![
                        format!("overrid package validate --target-ref package:{workflow_ref} --json"),
                        format!(
                            "overrid workload submit --workload-kind {workload_kind} --workload-ref {workflow_ref} --json"
                        ),
                        format!("overrid workload logs --workload-ref {workflow_ref} --json"),
                        format!("overrid workload result --workload-ref {workflow_ref} --json"),
                        format!(
                            "overrid workload cancel --workload-ref {workflow_ref} --reason operator_requested --json"
                        ),
                        format!("overrid usage show --target-ref usage:{workflow_ref} --json"),
                    ],
                    vec![
                        "code_agent_package_ref",
                        "repository_context_ref",
                        "execution_log_ref",
                        "artifact_refs",
                        "repair_boundary_ref",
                        "phase_usage_ref",
                    ],
                    vec![
                        "policy.resource_denied",
                        "package.invalid",
                        "repo_context.ref_denied",
                        "repair.boundary_exceeded",
                    ],
                    vec![
                        "retry_failed_phase_after_repair_ref",
                        "preserve_repo_context_ref",
                        "operator_review_for_policy_denial",
                    ],
                ),
            };

        Self {
            product,
            workflow_ref,
            workload_kind,
            command_recipes,
            required_refs: required_refs.into_iter().map(str::to_owned).collect(),
            expected_failure_modes: expected_failure_modes
                .into_iter()
                .map(str::to_owned)
                .collect(),
            safe_retry_patterns: safe_retry_patterns.into_iter().map(str::to_owned).collect(),
            submitted_via: "sdk_overgate_contract".to_owned(),
            sdk_overgate_only: true,
            authorized_refs_only: true,
            secret_free_json_output: true,
            dynamic_model_resource_selection: true,
            direct_internal_api_access: false,
            direct_storage_access: false,
            raw_http_required: false,
            hardcoded_model_or_provider: false,
            hardcoded_node_assumption: false,
            paid_service_assumption: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CiAutomationProfile {
    pub profile_kind: String,
    pub environment_class: EnvironmentClass,
    pub credential_reference_class: String,
    pub credential_reference_id: String,
    pub allowed_credential_ref_kinds: Vec<String>,
    pub submitted_via: String,
    pub short_lived_service_account_required: bool,
    pub ambient_persistent_keychain_allowed: bool,
    pub requires_non_interactive_confirmation: bool,
    pub json_output_stable: bool,
    pub secret_free_output: bool,
    pub branch_on_exit_class: bool,
}

impl CiAutomationProfile {
    pub fn new(
        environment_class: EnvironmentClass,
        credential_reference_class: impl Into<String>,
        credential_reference_id: impl Into<String>,
    ) -> Self {
        Self {
            profile_kind: "ci".to_owned(),
            environment_class,
            credential_reference_class: credential_reference_class.into(),
            credential_reference_id: credential_reference_id.into(),
            allowed_credential_ref_kinds: [
                "ci_reference",
                "signing_agent",
                "fixture",
                "hardware_token",
                "mounted_secret_ref",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            submitted_via: "sdk_overgate_contract".to_owned(),
            short_lived_service_account_required: true,
            ambient_persistent_keychain_allowed: false,
            requires_non_interactive_confirmation: true,
            json_output_stable: true,
            secret_free_output: true,
            branch_on_exit_class: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliPhaseAvailabilityRecord {
    pub command: String,
    pub phase_gate: String,
    pub availability: String,
    pub stable_reason_code: String,
    pub hidden_in_normal_help: bool,
    pub direct_private_shortcut: bool,
}

impl CliPhaseAvailabilityRecord {
    pub fn available(command: impl Into<String>, phase_gate: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            phase_gate: phase_gate.into(),
            availability: "available".to_owned(),
            stable_reason_code: "available".to_owned(),
            hidden_in_normal_help: false,
            direct_private_shortcut: false,
        }
    }

    pub fn read_only(command: impl Into<String>, phase_gate: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            phase_gate: phase_gate.into(),
            availability: "read_only".to_owned(),
            stable_reason_code: "read_model_only".to_owned(),
            hidden_in_normal_help: false,
            direct_private_shortcut: false,
        }
    }

    pub fn denied(command: impl Into<String>, phase_gate: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            phase_gate: phase_gate.into(),
            availability: "denied".to_owned(),
            stable_reason_code: "not_available_in_phase".to_owned(),
            hidden_in_normal_help: true,
            direct_private_shortcut: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliSecurityReviewReport {
    pub reviewed_surfaces: Vec<String>,
    pub redaction_probes: Vec<String>,
    pub forbidden_output_markers: Vec<String>,
    pub raw_keys_exposed: bool,
    pub tokens_exposed: bool,
    pub signatures_exposed: bool,
    pub secrets_exposed: bool,
    pub private_payloads_exposed: bool,
    pub decrypted_content_exposed: bool,
    pub unsafe_endpoints_allowed: bool,
    pub cross_tenant_access_allowed: bool,
}

impl CliSecurityReviewReport {
    pub fn new() -> Self {
        Self {
            reviewed_surfaces: [
                "credential_storage",
                "signer_handoff",
                "file_permissions",
                "environment_separation",
                "debug_output",
                "diagnostics",
                "logs",
                "execution_results",
                "phase_handoff",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            redaction_probes: [
                "raw_private_key",
                "token",
                "signature",
                "secret",
                "private_payload",
                "decrypted_content",
                "unsafe_endpoint",
                "cross_tenant_access",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            forbidden_output_markers: [
                "raw_private_key=",
                "token=",
                "signature=",
                "secret=",
                "private_payload=",
                "decrypted_content=",
                "http://overbase.",
                "database_endpoint=",
                "cache_endpoint=",
                "object_storage_endpoint=",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            raw_keys_exposed: false,
            tokens_exposed: false,
            signatures_exposed: false,
            secrets_exposed: false,
            private_payloads_exposed: false,
            decrypted_content_exposed: false,
            unsafe_endpoints_allowed: false,
            cross_tenant_access_allowed: false,
        }
    }
}

impl Default for CliSecurityReviewReport {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliReleaseReadinessReport {
    pub contract_snapshot_suite: Vec<String>,
    pub help_snapshot_commands: Vec<String>,
    pub exit_code_classes: Vec<String>,
    pub reason_code_families: Vec<String>,
    pub security_review_report: CliSecurityReviewReport,
    pub phase_availability_matrix: Vec<CliPhaseAvailabilityRecord>,
    pub integration_validation_matrix: Vec<String>,
    pub automation_compatibility_matrix: Vec<String>,
    pub handoff_notes: Vec<String>,
    pub release_ready: bool,
    pub sdk_overgate_only: bool,
    pub direct_private_shortcut: bool,
    pub high_risk_phase7_phase13_enabled: bool,
}

impl CliReleaseReadinessReport {
    pub fn new() -> Self {
        Self {
            contract_snapshot_suite: [
                "schema_contracts",
                "output_envelope",
                "exit_code_registry",
                "help_text",
                "human_output",
                "json_output",
                "error_decode_records",
                "backward_compatible_json",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            help_snapshot_commands: [
                "version",
                "doctor",
                "profile",
                "credential",
                "auth",
                "tenant",
                "identity",
                "key",
                "manifest",
                "node",
                "workload",
                "policy",
                "package",
                "usage",
                "receipt",
                "ledger",
                "dispute",
                "federation/public-interest/purpose-tag",
                "release-readiness",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            exit_code_classes: EXIT_CODE_REGISTRY
                .iter()
                .map(|exit_class| exit_class.as_str().to_owned())
                .collect(),
            reason_code_families: [
                "not_available_in_phase",
                "profile_validation_failed",
                "credential_validation_failed",
                "missing_profile_confirmation",
                "missing_reason",
                "policy.egress_denied",
                "package.invalid",
                "result.failed",
                "budget.exhausted",
                "transport.unavailable",
                "timeout.waiting",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            security_review_report: CliSecurityReviewReport::new(),
            phase_availability_matrix: vec![
                CliPhaseAvailabilityRecord::available("version", "phase_4"),
                CliPhaseAvailabilityRecord::available("doctor", "phase_4"),
                CliPhaseAvailabilityRecord::available("profile", "phase_3"),
                CliPhaseAvailabilityRecord::available("credential", "phase_3"),
                CliPhaseAvailabilityRecord::available("auth", "phase_1"),
                CliPhaseAvailabilityRecord::available("tenant", "phase_1"),
                CliPhaseAvailabilityRecord::available("identity", "phase_1"),
                CliPhaseAvailabilityRecord::available("key", "phase_1"),
                CliPhaseAvailabilityRecord::available("manifest", "phase_1"),
                CliPhaseAvailabilityRecord::available("node", "phase_7"),
                CliPhaseAvailabilityRecord::available("workload", "phase_7"),
                CliPhaseAvailabilityRecord::available("policy dry-run", "phase_8"),
                CliPhaseAvailabilityRecord::available("package validate", "phase_8"),
                CliPhaseAvailabilityRecord::read_only("usage show", "phase_8"),
                CliPhaseAvailabilityRecord::read_only("receipt show", "phase_8"),
                CliPhaseAvailabilityRecord::read_only("ledger inspect", "phase_8"),
                CliPhaseAvailabilityRecord::read_only("dispute inspect", "phase_8"),
                CliPhaseAvailabilityRecord::available("release-readiness", "phase_10"),
                CliPhaseAvailabilityRecord::denied(
                    "federation/public-interest/purpose-tag",
                    "phase_10",
                ),
                CliPhaseAvailabilityRecord::denied("deployment", "phase_9"),
                CliPhaseAvailabilityRecord::denied(
                    "governance/incident/compliance/migration",
                    "phase_7_or_phase_13",
                ),
            ],
            integration_validation_matrix: [
                "tenant_setup",
                "identity_key_lifecycle",
                "manifest_submit",
                "synthetic_workload",
                "real_private_job",
                "policy_dry_run",
                "package_validation",
                "usage_receipt_lookup",
                "cancellation",
                "timeout_retry",
                "docdex_workflow",
                "mcoda_workflow",
                "codali_workflow",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            automation_compatibility_matrix: [
                "stable_json_output",
                "stable_human_output",
                "exit_class_branching",
                "trace_id_presence",
                "audit_ref_presence",
                "bounded_retry_timeout",
                "ci_non_interactive_credentials",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            handoff_notes: [
                "phase10_federation_public_interest_commands_remain_disabled_until_contracts_exist",
                "phase7_backbone_commands_remain_disabled_until_contracts_exist",
                "phase13_governance_compliance_incident_migration_remain_disabled_until_contracts_exist",
                "completion_does_not_authorize_high_risk_operations_early",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            release_ready: true,
            sdk_overgate_only: true,
            direct_private_shortcut: false,
            high_risk_phase7_phase13_enabled: false,
        }
    }
}

impl Default for CliReleaseReadinessReport {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessRunStatus {
    Planned,
    Running,
    Passed,
    Failed,
    Blocked,
}

impl HarnessRunStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Running => "running",
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Blocked => "blocked",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Passed | Self::Failed | Self::Blocked)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioActionKind {
    Cli,
    Sdk,
    Api,
    LocalHelper,
    Assertion,
}

impl ScenarioActionKind {
    pub fn parse(raw: &str) -> Result<Self, HarnessContractError> {
        match raw {
            "cli" => Ok(Self::Cli),
            "sdk" => Ok(Self::Sdk),
            "api" => Ok(Self::Api),
            "local_helper" => Ok(Self::LocalHelper),
            "assertion" => Ok(Self::Assertion),
            other => Err(HarnessContractError::InvalidActionKind(other.to_owned())),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cli => "cli",
            Self::Sdk => "sdk",
            Self::Api => "api",
            Self::LocalHelper => "local_helper",
            Self::Assertion => "assertion",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoldenTraceMode {
    Exact,
    Dag,
}

impl GoldenTraceMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Dag => "dag",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactRetentionClass {
    SmokeCompact,
    FailureEvidence,
    ReleaseCandidate,
    PhaseGateEvidence,
}

impl ArtifactRetentionClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SmokeCompact => "smoke_compact",
            Self::FailureEvidence => "failure_evidence",
            Self::ReleaseCandidate => "release_candidate",
            Self::PhaseGateEvidence => "phase_gate_evidence",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedactionScanReport {
    pub policy: String,
    pub redacted_fields: Vec<String>,
    pub scanner_passed: bool,
    pub rejected_markers: Vec<String>,
}

impl RedactionScanReport {
    pub fn passed(redacted_fields: Vec<String>) -> Self {
        Self {
            policy: "secret_free_refs_only".to_owned(),
            redacted_fields,
            scanner_passed: true,
            rejected_markers: Vec::new(),
        }
    }

    pub fn failed(rejected_markers: Vec<String>) -> Self {
        Self {
            policy: "secret_free_refs_only".to_owned(),
            redacted_fields: Vec::new(),
            scanner_passed: false,
            rejected_markers,
        }
    }

    pub fn validate(&self) -> Result<(), HarnessContractError> {
        harness_require_non_empty(&self.policy, "redaction policy")?;
        if self.policy != "secret_free_refs_only" {
            return Err(HarnessContractError::RawSecretInArtifact);
        }
        if !self.scanner_passed || !self.rejected_markers.is_empty() {
            return Err(HarnessContractError::RawSecretInArtifact);
        }
        for value in self.redacted_fields.iter().chain(&self.rejected_markers) {
            if contains_raw_secret_marker(value) {
                return Err(HarnessContractError::RawSecretInArtifact);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlakeMetadata {
    pub repeated_run_count: u32,
    pub timing_variance_ms: u64,
    pub nondeterministic_assertion_markers: Vec<String>,
    pub unstable_event_ordering: bool,
    pub tolerance_window_used: bool,
}

impl FlakeMetadata {
    pub fn stable() -> Self {
        Self {
            repeated_run_count: 1,
            timing_variance_ms: 0,
            nondeterministic_assertion_markers: Vec::new(),
            unstable_event_ordering: false,
            tolerance_window_used: false,
        }
    }

    pub fn unstable_event_ordering(markers: Vec<String>) -> Self {
        Self {
            repeated_run_count: 3,
            timing_variance_ms: 125,
            nondeterministic_assertion_markers: markers,
            unstable_event_ordering: true,
            tolerance_window_used: true,
        }
    }

    pub fn is_nondeterministic(&self) -> bool {
        self.unstable_event_ordering || !self.nondeterministic_assertion_markers.is_empty()
    }

    pub fn validate(&self) -> Result<(), HarnessContractError> {
        if self.repeated_run_count == 0 {
            return Err(HarnessContractError::MissingRequiredField(
                "repeated run count",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactRetentionPolicy {
    pub retention_class: ArtifactRetentionClass,
    pub minimum_retention_days: u16,
    pub prune_after_days: u16,
    pub compact_success_summary: bool,
}

impl ArtifactRetentionPolicy {
    pub fn for_class(retention_class: ArtifactRetentionClass) -> Self {
        match retention_class {
            ArtifactRetentionClass::SmokeCompact => Self {
                retention_class,
                minimum_retention_days: 1,
                prune_after_days: 7,
                compact_success_summary: true,
            },
            ArtifactRetentionClass::FailureEvidence => Self {
                retention_class,
                minimum_retention_days: 30,
                prune_after_days: 90,
                compact_success_summary: false,
            },
            ArtifactRetentionClass::ReleaseCandidate => Self {
                retention_class,
                minimum_retention_days: 90,
                prune_after_days: 365,
                compact_success_summary: false,
            },
            ArtifactRetentionClass::PhaseGateEvidence => Self {
                retention_class,
                minimum_retention_days: 14,
                prune_after_days: 180,
                compact_success_summary: false,
            },
        }
    }

    pub fn validate(&self) -> Result<(), HarnessContractError> {
        if self.minimum_retention_days == 0 || self.prune_after_days < self.minimum_retention_days {
            return Err(HarnessContractError::MissingArtifactPolicy);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureKey {
    pub key_ref: String,
    pub key_id: String,
    pub test_only: bool,
    pub signature_ref_only: bool,
    pub raw_key_material_present: bool,
}

impl FixtureKey {
    pub fn test_only(key_ref: impl Into<String>, key_id: impl Into<String>) -> Self {
        Self {
            key_ref: key_ref.into(),
            key_id: key_id.into(),
            test_only: true,
            signature_ref_only: true,
            raw_key_material_present: false,
        }
    }

    pub fn validate(&self) -> Result<(), HarnessContractError> {
        harness_require_non_empty(&self.key_ref, "key ref")?;
        harness_require_non_empty(&self.key_id, "key id")?;
        if !self.test_only || !self.signature_ref_only || self.raw_key_material_present {
            return Err(HarnessContractError::FixtureKeyNotTestOnly);
        }
        if contains_raw_secret_marker(&self.key_ref) || contains_raw_secret_marker(&self.key_id) {
            return Err(HarnessContractError::RawSecretInArtifact);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureManifest {
    pub schema_version: SchemaVersion,
    pub fixture_id: String,
    pub tenant_ref: String,
    pub actor_ref: String,
    pub deterministic_seed: String,
    pub keys: Vec<FixtureKey>,
    pub resource_card_refs: Vec<String>,
    pub workload_refs: Vec<String>,
    pub package_refs: Vec<String>,
    pub local_ledger_account_refs: Vec<String>,
    pub policy_context_refs: Vec<String>,
    pub test_only: bool,
}

impl FixtureManifest {
    pub fn new(
        fixture_id: impl Into<String>,
        tenant_ref: impl Into<String>,
        actor_ref: impl Into<String>,
        deterministic_seed: impl Into<String>,
        keys: Vec<FixtureKey>,
        schema_version: &str,
    ) -> Result<Self, HarnessContractError> {
        let manifest = Self {
            schema_version: ensure_supported_integration_harness_schema_version(schema_version)?,
            fixture_id: fixture_id.into(),
            tenant_ref: tenant_ref.into(),
            actor_ref: actor_ref.into(),
            deterministic_seed: deterministic_seed.into(),
            keys,
            resource_card_refs: vec!["resource:local:synthetic_cpu".to_owned()],
            workload_refs: vec!["workload:local:no_op".to_owned()],
            package_refs: vec!["package:local:no_op".to_owned()],
            local_ledger_account_refs: vec!["ledger:local:oru_account".to_owned()],
            policy_context_refs: vec!["policy:local:allow_smoke".to_owned()],
            test_only: true,
        };
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), HarnessContractError> {
        harness_require_non_empty(&self.fixture_id, "fixture id")?;
        harness_require_non_empty(&self.tenant_ref, "tenant ref")?;
        harness_require_non_empty(&self.actor_ref, "actor ref")?;
        harness_require_non_empty(&self.deterministic_seed, "deterministic seed")?;
        if !self.test_only {
            return Err(HarnessContractError::FixtureNotTestOnly);
        }
        if self.keys.is_empty() {
            return Err(HarnessContractError::MissingRequiredField("fixture key"));
        }
        for key in &self.keys {
            key.validate()?;
        }
        if self.resource_card_refs.is_empty()
            || self.workload_refs.is_empty()
            || self.package_refs.is_empty()
            || self.local_ledger_account_refs.is_empty()
            || self.policy_context_refs.is_empty()
        {
            return Err(HarnessContractError::MissingRequiredField(
                "fixture compatibility refs",
            ));
        }
        for value in &self.workload_refs {
            ensure_harness_ref_prefix("workload ref", value, "workload:")?;
        }
        for value in &self.package_refs {
            ensure_harness_ref_prefix("package ref", value, "package:")?;
        }
        for value in &self.local_ledger_account_refs {
            ensure_harness_ref_prefix("local ledger account ref", value, "ledger:")?;
        }
        for value in &self.policy_context_refs {
            ensure_harness_ref_prefix("policy context ref", value, "policy:")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioStep {
    pub step_id: String,
    pub action_kind: ScenarioActionKind,
    pub input_refs: Vec<String>,
    pub timeout_ms: u64,
    pub retry_expectation: String,
    pub expected_result_class: HarnessRunStatus,
    pub assertion_refs: Vec<String>,
    pub cleanup_rule: String,
}

impl ScenarioStep {
    pub fn new(
        step_id: impl Into<String>,
        action_kind: ScenarioActionKind,
        assertion_refs: Vec<String>,
        timeout_ms: u64,
    ) -> Result<Self, HarnessContractError> {
        let step = Self {
            step_id: step_id.into(),
            action_kind,
            input_refs: Vec::new(),
            timeout_ms,
            retry_expectation: "none".to_owned(),
            expected_result_class: HarnessRunStatus::Passed,
            assertion_refs,
            cleanup_rule: "collect_artifacts_then_reset".to_owned(),
        };
        step.validate()?;
        Ok(step)
    }

    pub fn validate(&self) -> Result<(), HarnessContractError> {
        harness_require_non_empty(&self.step_id, "step id")?;
        if self.timeout_ms == 0 || self.timeout_ms > 600_000 {
            return Err(HarnessContractError::UnsafeTimeoutMs(self.timeout_ms));
        }
        if self.assertion_refs.is_empty() {
            return Err(HarnessContractError::MissingAssertion);
        }
        harness_require_non_empty(&self.retry_expectation, "retry expectation")?;
        harness_require_non_empty(&self.cleanup_rule, "cleanup rule")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioManifest {
    pub schema_version: SchemaVersion,
    pub scenario_id: String,
    pub master_phase: u8,
    pub gate_class: String,
    pub tags: Vec<String>,
    pub required_services: Vec<String>,
    pub setup_fixture_refs: Vec<String>,
    pub steps: Vec<ScenarioStep>,
    pub cleanup_rules: Vec<String>,
}

impl ScenarioManifest {
    pub fn new(
        scenario_id: impl Into<String>,
        master_phase: u8,
        steps: Vec<ScenarioStep>,
        schema_version: &str,
    ) -> Result<Self, HarnessContractError> {
        let manifest = Self {
            schema_version: ensure_supported_integration_harness_schema_version(schema_version)?,
            scenario_id: scenario_id.into(),
            master_phase,
            gate_class: "smoke".to_owned(),
            tags: vec!["phase0".to_owned(), "smoke".to_owned()],
            required_services: vec!["service:local_stack".to_owned()],
            setup_fixture_refs: vec!["fixture:phase0_smoke".to_owned()],
            steps,
            cleanup_rules: vec!["collect_artifacts_then_reset".to_owned()],
        };
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), HarnessContractError> {
        harness_require_non_empty(&self.scenario_id, "scenario id")?;
        if self.master_phase > 13 {
            return Err(HarnessContractError::UnsupportedPhase(self.master_phase));
        }
        if self.tags.is_empty() {
            return Err(HarnessContractError::MissingRequiredField("scenario tag"));
        }
        if self.required_services.is_empty() {
            return Err(HarnessContractError::MissingRequiredField(
                "required service",
            ));
        }
        if self.setup_fixture_refs.is_empty() {
            return Err(HarnessContractError::MissingRequiredField(
                "setup fixture ref",
            ));
        }
        if self.steps.is_empty() {
            return Err(HarnessContractError::MissingRequiredField("scenario step"));
        }
        for step in &self.steps {
            step.validate()?;
        }
        if self.cleanup_rules.is_empty() {
            return Err(HarnessContractError::MissingRequiredField("cleanup rule"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssertionResult {
    pub assertion_id: String,
    pub scenario_id: String,
    pub status: HarnessRunStatus,
    pub reason_code: String,
    pub field_path: String,
    pub expected_value: String,
    pub actual_value: String,
    pub artifact_refs: Vec<String>,
}

impl AssertionResult {
    pub fn passed(assertion_id: impl Into<String>, scenario_id: impl Into<String>) -> Self {
        Self {
            assertion_id: assertion_id.into(),
            scenario_id: scenario_id.into(),
            status: HarnessRunStatus::Passed,
            reason_code: "assertion.passed".to_owned(),
            field_path: "$.events".to_owned(),
            expected_value: "expected".to_owned(),
            actual_value: "expected".to_owned(),
            artifact_refs: vec!["artifact:bundle:phase0_smoke".to_owned()],
        }
    }

    pub fn validate(&self) -> Result<(), HarnessContractError> {
        harness_require_non_empty(&self.assertion_id, "assertion id")?;
        harness_require_non_empty(&self.scenario_id, "scenario id")?;
        if !self.status.is_terminal() {
            return Err(HarnessContractError::MissingRunStatus);
        }
        harness_require_non_empty(&self.reason_code, "reason code")?;
        harness_require_non_empty(&self.field_path, "field path")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestRunRecord {
    pub schema_version: SchemaVersion,
    pub run_id: String,
    pub scenario_ids: Vec<String>,
    pub stack_profile: String,
    pub workspace_fingerprint: String,
    pub status: HarnessRunStatus,
    pub started_at_ms: u64,
    pub ended_at_ms: u64,
    pub reason_code: String,
    pub reason_class: String,
    pub assertion_refs: Vec<String>,
    pub artifact_policy: ArtifactRetentionClass,
    pub artifact_refs: Vec<String>,
    pub flake_metadata: FlakeMetadata,
}

impl TestRunRecord {
    pub fn terminal(
        run_id: impl Into<String>,
        status: HarnessRunStatus,
        schema_version: &str,
    ) -> Result<Self, HarnessContractError> {
        let record = Self {
            schema_version: ensure_supported_integration_harness_schema_version(schema_version)?,
            run_id: run_id.into(),
            scenario_ids: vec!["scenario_phase0_smoke".to_owned()],
            stack_profile: "local".to_owned(),
            workspace_fingerprint: "workspace_local_0001".to_owned(),
            status,
            started_at_ms: 1,
            ended_at_ms: 2,
            reason_code: match status {
                HarnessRunStatus::Passed => "run.passed".to_owned(),
                HarnessRunStatus::Failed => "run.failed".to_owned(),
                HarnessRunStatus::Blocked => "run.blocked".to_owned(),
                HarnessRunStatus::Planned | HarnessRunStatus::Running => String::new(),
            },
            reason_class: match status {
                HarnessRunStatus::Passed => "success".to_owned(),
                HarnessRunStatus::Failed => "assertion".to_owned(),
                HarnessRunStatus::Blocked => "dependency".to_owned(),
                HarnessRunStatus::Planned | HarnessRunStatus::Running => String::new(),
            },
            assertion_refs: vec!["assertion:phase0:trace_order".to_owned()],
            artifact_policy: ArtifactRetentionClass::PhaseGateEvidence,
            artifact_refs: vec!["artifact:bundle:phase0_smoke".to_owned()],
            flake_metadata: FlakeMetadata::stable(),
        };
        record.validate()?;
        Ok(record)
    }

    pub fn validate(&self) -> Result<(), HarnessContractError> {
        harness_require_non_empty(&self.run_id, "run id")?;
        if self.scenario_ids.is_empty() {
            return Err(HarnessContractError::MissingRequiredField("scenario id"));
        }
        harness_require_non_empty(&self.stack_profile, "stack profile")?;
        harness_require_non_empty(&self.workspace_fingerprint, "workspace fingerprint")?;
        if !self.status.is_terminal() {
            return Err(HarnessContractError::MissingRunStatus);
        }
        if self.started_at_ms == 0 || self.ended_at_ms == 0 || self.ended_at_ms < self.started_at_ms
        {
            return Err(HarnessContractError::MissingTiming);
        }
        harness_require_non_empty(&self.reason_code, "reason code")?;
        harness_require_non_empty(&self.reason_class, "reason class")
            .map_err(|_| HarnessContractError::MissingReasonClass)?;
        if self.assertion_refs.is_empty() {
            return Err(HarnessContractError::MissingAssertion);
        }
        if self.artifact_refs.is_empty() {
            return Err(HarnessContractError::MissingArtifactPolicy);
        }
        self.flake_metadata.validate()?;
        if self.status == HarnessRunStatus::Passed && self.flake_metadata.is_nondeterministic() {
            return Err(HarnessContractError::RawSecretInArtifact);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoldenTrace {
    pub schema_version: SchemaVersion,
    pub trace_id: String,
    pub mode: GoldenTraceMode,
    pub required_nodes: Vec<String>,
    pub required_causal_edges: Vec<(String, String)>,
    pub forbidden_transitions: Vec<(String, String)>,
    pub stable_reason_codes: Vec<String>,
}

impl GoldenTrace {
    pub fn new(
        trace_id: impl Into<String>,
        mode: GoldenTraceMode,
        required_nodes: Vec<String>,
        required_causal_edges: Vec<(String, String)>,
        schema_version: &str,
    ) -> Result<Self, HarnessContractError> {
        let trace = Self {
            schema_version: ensure_supported_integration_harness_schema_version(schema_version)?,
            trace_id: trace_id.into(),
            mode,
            required_nodes,
            required_causal_edges,
            forbidden_transitions: Vec::new(),
            stable_reason_codes: vec!["command.accepted".to_owned(), "audit.written".to_owned()],
        };
        trace.validate()?;
        Ok(trace)
    }

    pub fn validate(&self) -> Result<(), HarnessContractError> {
        harness_require_non_empty(&self.trace_id, "trace id")?;
        if self.required_nodes.is_empty() {
            return Err(HarnessContractError::MissingRequiredField("trace node"));
        }
        if self.mode == GoldenTraceMode::Dag && self.required_causal_edges.is_empty() {
            return Err(HarnessContractError::GoldenTraceMissingEdge);
        }
        if self.mode == GoldenTraceMode::Exact && self.required_nodes.len() < 2 {
            return Err(HarnessContractError::MissingRequiredField(
                "exact trace order",
            ));
        }
        if self.stable_reason_codes.is_empty() {
            return Err(HarnessContractError::MissingRequiredField(
                "stable reason code",
            ));
        }
        Ok(())
    }

    pub fn assert_observed_nodes(
        &self,
        observed_nodes: &[String],
    ) -> Result<(), HarnessContractError> {
        for node in &self.required_nodes {
            if !observed_nodes.contains(node) {
                return Err(HarnessContractError::GoldenTraceMissingEvent);
            }
        }

        if observed_nodes
            .iter()
            .any(|node| node.starts_with("event_") && !self.required_nodes.contains(node))
        {
            return Err(HarnessContractError::GoldenTraceExtraStateEvent);
        }

        if self.mode == GoldenTraceMode::Exact
            && observed_nodes.len() >= self.required_nodes.len()
            && observed_nodes[..self.required_nodes.len()] != self.required_nodes[..]
        {
            return Err(HarnessContractError::GoldenTraceExactOrderMismatch);
        }

        for (from, to) in &self.forbidden_transitions {
            if observed_nodes.windows(2).any(|window| {
                window[0].as_str() == from.as_str() && window[1].as_str() == to.as_str()
            }) {
                return Err(HarnessContractError::GoldenTraceForbiddenTransition);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactBundle {
    pub schema_version: SchemaVersion,
    pub bundle_id: String,
    pub run_id: String,
    pub retention_class: ArtifactRetentionClass,
    pub privacy_classifications: Vec<String>,
    pub redaction_policy: String,
    pub redaction_report: RedactionScanReport,
    pub redacted_log_refs: Vec<String>,
    pub overwatch_export_refs: Vec<String>,
    pub cli_output_refs: Vec<String>,
    pub api_payload_envelope_refs: Vec<String>,
    pub stack_health_refs: Vec<String>,
    pub fixture_version_refs: Vec<String>,
    pub schema_version_refs: Vec<String>,
    pub assertion_diff_refs: Vec<String>,
    pub redaction_report_ref: String,
    pub reproduction_command: String,
    pub artifact_refs: Vec<String>,
    pub flake_metadata: FlakeMetadata,
    pub retention_policy: ArtifactRetentionPolicy,
    pub contains_raw_secret: bool,
    pub contains_private_key: bool,
    pub contains_token: bool,
    pub contains_signature: bool,
    pub contains_encrypted_rag_content: bool,
    pub contains_private_payload: bool,
    pub contains_fixture_key_material: bool,
}

impl ArtifactBundle {
    pub fn new(
        bundle_id: impl Into<String>,
        run_id: impl Into<String>,
        contains_raw_secret: bool,
        contains_private_payload: bool,
        schema_version: &str,
    ) -> Result<Self, HarnessContractError> {
        let bundle = Self {
            schema_version: ensure_supported_integration_harness_schema_version(schema_version)?,
            bundle_id: bundle_id.into(),
            run_id: run_id.into(),
            retention_class: ArtifactRetentionClass::PhaseGateEvidence,
            privacy_classifications: vec![
                "public_test_metadata".to_owned(),
                "redacted_local_log".to_owned(),
            ],
            redaction_policy: "secret_free_refs_only".to_owned(),
            redaction_report: RedactionScanReport::passed(vec![
                "headers.authorization".to_owned(),
                "payload.private".to_owned(),
            ]),
            redacted_log_refs: vec!["artifact:logs:phase0_smoke_redacted".to_owned()],
            overwatch_export_refs: vec!["artifact:overwatch:phase0_smoke".to_owned()],
            cli_output_refs: vec!["artifact:cli_output:phase0_smoke".to_owned()],
            api_payload_envelope_refs: vec!["artifact:api_envelope:phase0_smoke".to_owned()],
            stack_health_refs: vec!["artifact:health:local_stack".to_owned()],
            fixture_version_refs: vec!["artifact:fixture_version:phase0_smoke".to_owned()],
            schema_version_refs: vec!["artifact:schema_version:integration_harness_v0_1".to_owned()],
            assertion_diff_refs: vec!["artifact:assertion_diff:phase0_smoke".to_owned()],
            redaction_report_ref: "artifact:redaction_report:phase0_smoke".to_owned(),
            reproduction_command:
                "overrid test scenario scenario_phase0_smoke --profile local --json".to_owned(),
            artifact_refs: vec!["artifact:bundle:phase0_smoke".to_owned()],
            flake_metadata: FlakeMetadata::stable(),
            retention_policy: ArtifactRetentionPolicy::for_class(
                ArtifactRetentionClass::PhaseGateEvidence,
            ),
            contains_raw_secret,
            contains_private_key: false,
            contains_token: false,
            contains_signature: false,
            contains_encrypted_rag_content: false,
            contains_private_payload,
            contains_fixture_key_material: false,
        };
        bundle.validate()?;
        Ok(bundle)
    }

    pub fn validate(&self) -> Result<(), HarnessContractError> {
        harness_require_non_empty(&self.bundle_id, "artifact bundle id")?;
        harness_require_non_empty(&self.run_id, "run id")?;
        if self.privacy_classifications.is_empty() {
            return Err(HarnessContractError::MissingRequiredField(
                "privacy classification",
            ));
        }
        harness_require_non_empty(&self.redaction_policy, "redaction policy")?;
        self.redaction_report.validate()?;
        self.flake_metadata.validate()?;
        self.retention_policy.validate()?;
        if self.retention_policy.retention_class != self.retention_class {
            return Err(HarnessContractError::MissingArtifactPolicy);
        }
        for (field, refs) in [
            ("redacted log refs", &self.redacted_log_refs),
            ("Overwatch export refs", &self.overwatch_export_refs),
            ("CLI output refs", &self.cli_output_refs),
            ("API payload envelope refs", &self.api_payload_envelope_refs),
            ("stack health refs", &self.stack_health_refs),
            ("fixture version refs", &self.fixture_version_refs),
            ("schema version refs", &self.schema_version_refs),
            ("assertion diff refs", &self.assertion_diff_refs),
        ] {
            if refs.is_empty() {
                return Err(HarnessContractError::MissingRequiredField(field));
            }
            for value in refs {
                ensure_harness_ref_prefix(field, value, "artifact:")?;
            }
        }
        ensure_harness_ref_prefix(
            "redaction report ref",
            &self.redaction_report_ref,
            "artifact:",
        )?;
        harness_require_non_empty(&self.reproduction_command, "reproduction command")?;
        if self.artifact_refs.is_empty() {
            return Err(HarnessContractError::MissingArtifactPolicy);
        }
        if self.contains_raw_secret
            || self.contains_private_key
            || self.contains_token
            || self.contains_signature
            || self.contains_encrypted_rag_content
            || self.contains_private_payload
            || self.contains_fixture_key_material
        {
            return Err(HarnessContractError::RawSecretInArtifact);
        }
        if contains_raw_secret_marker(&self.reproduction_command) {
            return Err(HarnessContractError::RawSecretInArtifact);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HarnessContractError {
    UnsupportedSchemaVersion(ContractError),
    MissingRequiredField(&'static str),
    InvalidHarnessRef {
        field: &'static str,
        value: String,
        expected_prefix: &'static str,
    },
    FixtureNotTestOnly,
    FixtureKeyNotTestOnly,
    UnsupportedPhase(u8),
    InvalidActionKind(String),
    MissingAssertion,
    UnsafeTimeoutMs(u64),
    MissingRunStatus,
    MissingTiming,
    MissingReasonClass,
    MissingArtifactPolicy,
    GoldenTraceMissingEdge,
    GoldenTraceMissingEvent,
    GoldenTraceExtraStateEvent,
    GoldenTraceExactOrderMismatch,
    GoldenTraceForbiddenTransition,
    RawSecretInArtifact,
}

impl From<ContractError> for HarnessContractError {
    fn from(error: ContractError) -> Self {
        Self::UnsupportedSchemaVersion(error)
    }
}

impl fmt::Display for HarnessContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion(error) => error.fmt(formatter),
            Self::MissingRequiredField(field) => write!(formatter, "{field} is required"),
            Self::InvalidHarnessRef {
                field,
                value,
                expected_prefix,
            } => write!(
                formatter,
                "{field} has invalid ref {value}; expected prefix {expected_prefix}"
            ),
            Self::FixtureNotTestOnly => formatter.write_str("fixture manifest is not test-only"),
            Self::FixtureKeyNotTestOnly => {
                formatter.write_str("fixture key must be test-only and signature-ref-only")
            }
            Self::UnsupportedPhase(phase) => write!(formatter, "unsupported master phase: {phase}"),
            Self::InvalidActionKind(kind) => write!(formatter, "invalid action kind: {kind}"),
            Self::MissingAssertion => formatter.write_str("at least one assertion is required"),
            Self::UnsafeTimeoutMs(timeout_ms) => {
                write!(formatter, "unsafe scenario timeout: {timeout_ms}ms")
            }
            Self::MissingRunStatus => formatter.write_str("terminal run status is required"),
            Self::MissingTiming => formatter.write_str("started and ended timing is required"),
            Self::MissingReasonClass => formatter.write_str("run reason class is required"),
            Self::MissingArtifactPolicy => {
                formatter.write_str("artifact policy and refs are required")
            }
            Self::GoldenTraceMissingEdge => {
                formatter.write_str("DAG golden trace requires at least one causal edge")
            }
            Self::GoldenTraceMissingEvent => {
                formatter.write_str("observed trace is missing a required event")
            }
            Self::GoldenTraceExtraStateEvent => {
                formatter.write_str("observed trace contains an extra state-changing event")
            }
            Self::GoldenTraceExactOrderMismatch => {
                formatter.write_str("observed trace does not match exact golden order")
            }
            Self::GoldenTraceForbiddenTransition => {
                formatter.write_str("observed trace contains a forbidden transition")
            }
            Self::RawSecretInArtifact => {
                formatter.write_str("artifact or fixture contains raw secret material")
            }
        }
    }
}

impl std::error::Error for HarnessContractError {}

fn harness_require_non_empty(value: &str, field: &'static str) -> Result<(), HarnessContractError> {
    if value.trim().is_empty() {
        Err(HarnessContractError::MissingRequiredField(field))
    } else {
        Ok(())
    }
}

fn ensure_harness_ref_prefix(
    field: &'static str,
    value: &str,
    expected_prefix: &'static str,
) -> Result<(), HarnessContractError> {
    harness_require_non_empty(value, field)?;
    if value.starts_with(expected_prefix) {
        Ok(())
    } else {
        Err(HarnessContractError::InvalidHarnessRef {
            field,
            value: value.to_owned(),
            expected_prefix,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileValidationError {
    MissingRequiredField(&'static str),
    UnknownEnvironment(String),
    UnknownCredentialClass(String),
    UnknownFixtureAllowance(String),
    UnknownConfirmationPolicy(String),
    InvalidOutputMode(String),
    MissingCredentialClass,
    MissingSchemaPin,
    UnsupportedSchemaPin(ContractError),
    MissingEndpointFingerprint,
    FixtureCredentialNotAllowed { environment: &'static str },
    CredentialNamespaceMismatch { expected: String, actual: String },
    CredentialClassNotAllowed { class: &'static str },
    CredentialRevoked,
    CredentialExpired,
    RawKeyMaterialInReference,
    UnsafeFilePermissions(u32),
}

impl fmt::Display for ProfileValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredField(field) => write!(formatter, "{field} is required"),
            Self::UnknownEnvironment(value) => write!(formatter, "unknown environment: {value}"),
            Self::UnknownCredentialClass(value) => {
                write!(formatter, "unknown credential class: {value}")
            }
            Self::UnknownFixtureAllowance(value) => {
                write!(formatter, "unknown fixture allowance: {value}")
            }
            Self::UnknownConfirmationPolicy(value) => {
                write!(formatter, "unknown confirmation policy: {value}")
            }
            Self::InvalidOutputMode(value) => write!(formatter, "invalid output mode: {value}"),
            Self::MissingCredentialClass => formatter.write_str("credential class is required"),
            Self::MissingSchemaPin => formatter.write_str("at least one schema pin is required"),
            Self::UnsupportedSchemaPin(error) => error.fmt(formatter),
            Self::MissingEndpointFingerprint => {
                formatter.write_str("endpoint fingerprint is required for locked environments")
            }
            Self::FixtureCredentialNotAllowed { environment } => write!(
                formatter,
                "fixture credential is not allowed for {environment} profile"
            ),
            Self::CredentialNamespaceMismatch { expected, actual } => write!(
                formatter,
                "credential namespace mismatch: expected {expected}, got {actual}"
            ),
            Self::CredentialClassNotAllowed { class } => {
                write!(formatter, "credential class is not allowed: {class}")
            }
            Self::CredentialRevoked => formatter.write_str("credential is revoked"),
            Self::CredentialExpired => formatter.write_str("credential is expired"),
            Self::RawKeyMaterialInReference => {
                formatter.write_str("credential reference contains raw key material")
            }
            Self::UnsafeFilePermissions(mode) => {
                write!(formatter, "file mode {mode:o} is not owner-only")
            }
        }
    }
}

impl std::error::Error for ProfileValidationError {}

pub fn validate_owner_only_file_mode(mode: u32) -> Result<(), ProfileValidationError> {
    if mode & 0o077 == 0 {
        Ok(())
    } else {
        Err(ProfileValidationError::UnsafeFilePermissions(mode))
    }
}

fn require_non_empty(value: &str, field: &'static str) -> Result<(), ProfileValidationError> {
    if value.trim().is_empty() {
        Err(ProfileValidationError::MissingRequiredField(field))
    } else {
        Ok(())
    }
}

fn contains_raw_secret_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    value.contains("-----BEGIN")
        || lower.contains("private key")
        || lower.contains("secret=")
        || lower.contains("token=")
        || lower.contains("raw_key")
}

pub const LOCAL_STACK_RESERVED_PORT_MIN: u16 = 18_080;
pub const LOCAL_STACK_RESERVED_PORT_MAX: u16 = 18_085;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalStackHealthState {
    NotStarted,
    Starting,
    Ready,
    Degraded,
    Failed,
    Resetting,
    Seeding,
    RunningTests,
}

impl LocalStackHealthState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Starting => "starting",
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::Failed => "failed",
            Self::Resetting => "resetting",
            Self::Seeding => "seeding",
            Self::RunningTests => "running_tests",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Ready | Self::Degraded | Self::Failed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalStackPhaseGateState {
    BuildablePhase0,
    LocalSmokePrerequisite,
    OwningServiceRequired,
    PlannedDisabled,
    NotLocalStackOwned,
}

impl LocalStackPhaseGateState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BuildablePhase0 => "buildable_phase_0",
            Self::LocalSmokePrerequisite => "local_smoke_prerequisite",
            Self::OwningServiceRequired => "owning_service_required",
            Self::PlannedDisabled => "planned_disabled",
            Self::NotLocalStackOwned => "not_local_stack_owned",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackProfileContract {
    pub schema_version: SchemaVersion,
    pub profile_id: String,
    pub environment_class: EnvironmentClass,
    pub enabled_services: Vec<String>,
    pub required_phase_gates: Vec<LocalStackPhaseGateState>,
    pub local_only: bool,
    pub test_only: bool,
    pub default_bind_host: String,
    pub future_services_require_phase_gate: bool,
}

impl LocalStackProfileContract {
    pub fn local_default(profile_id: impl Into<String>) -> Result<Self, LocalStackContractError> {
        let profile = Self {
            schema_version: ensure_supported_local_development_stack_schema_version(
                SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
            )?,
            profile_id: profile_id.into(),
            environment_class: EnvironmentClass::Local,
            enabled_services: vec![
                "service:api".to_owned(),
                "service:worker".to_owned(),
                "service:embedded_state".to_owned(),
            ],
            required_phase_gates: vec![
                LocalStackPhaseGateState::BuildablePhase0,
                LocalStackPhaseGateState::LocalSmokePrerequisite,
                LocalStackPhaseGateState::OwningServiceRequired,
            ],
            local_only: true,
            test_only: true,
            default_bind_host: "127.0.0.1".to_owned(),
            future_services_require_phase_gate: true,
        };
        profile.validate()?;
        Ok(profile)
    }

    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.profile_id, "profile id")?;
        if !matches!(
            self.environment_class,
            EnvironmentClass::Local | EnvironmentClass::Ci
        ) {
            return Err(LocalStackContractError::UnsupportedEnvironment(
                self.environment_class.as_str(),
            ));
        }
        if self.enabled_services.is_empty() {
            return Err(LocalStackContractError::MissingRequiredField(
                "enabled service",
            ));
        }
        if self.required_phase_gates.is_empty() {
            return Err(LocalStackContractError::MissingRequiredField("phase gate"));
        }
        if !self.local_only || !self.test_only {
            return Err(LocalStackContractError::MissingLocalTestMarker);
        }
        ensure_loopback_bind_host(&self.default_bind_host)?;
        for service in &self.enabled_services {
            local_stack_require_non_empty(service, "enabled service")?;
            if is_future_service_ref(service)
                && !self.required_phase_gates.iter().any(|gate| {
                    matches!(
                        gate,
                        LocalStackPhaseGateState::OwningServiceRequired
                            | LocalStackPhaseGateState::PlannedDisabled
                    )
                })
            {
                return Err(LocalStackContractError::FutureServiceMissingPhaseGate(
                    service.clone(),
                ));
            }
        }
        if !self.future_services_require_phase_gate {
            return Err(LocalStackContractError::FutureServiceMissingPhaseGate(
                "future services".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackHealthCheck {
    pub kind: String,
    pub endpoint: String,
    pub expected_state: LocalStackHealthState,
    pub timeout_ms: u64,
}

impl LocalStackHealthCheck {
    pub fn http(endpoint: impl Into<String>) -> Self {
        Self {
            kind: "http".to_owned(),
            endpoint: endpoint.into(),
            expected_state: LocalStackHealthState::Ready,
            timeout_ms: 60_000,
        }
    }

    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.kind, "health check kind")?;
        local_stack_require_non_empty(&self.endpoint, "health check endpoint")?;
        if self.timeout_ms == 0 || self.timeout_ms > 600_000 {
            return Err(LocalStackContractError::InvalidHealthCheck);
        }
        if !endpoint_is_loopback_or_local_ref(&self.endpoint) {
            return Err(LocalStackContractError::NonLoopbackEndpoint(
                self.endpoint.clone(),
            ));
        }
        if contains_raw_secret_marker(&self.endpoint) {
            return Err(LocalStackContractError::RawSecretInDiagnostic);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackServiceDefinitionContract {
    pub service_id: String,
    pub service_kind: String,
    pub command: Vec<String>,
    pub working_directory: String,
    pub env_refs: Vec<String>,
    pub dependency_order: Option<u16>,
    pub depends_on: Vec<String>,
    pub health_check: Option<LocalStackHealthCheck>,
    pub shutdown_behavior: String,
    pub log_target: String,
    pub restart_class: String,
    pub local_only: bool,
    pub test_only: bool,
}

impl LocalStackServiceDefinitionContract {
    pub fn local_api() -> Result<Self, LocalStackContractError> {
        let service = Self {
            service_id: "service:api".to_owned(),
            service_kind: "api".to_owned(),
            command: vec![
                "cargo".to_owned(),
                "run".to_owned(),
                "-p".to_owned(),
                "overrid-local-api".to_owned(),
            ],
            working_directory: "repo://.".to_owned(),
            env_refs: vec!["env://OVERRID_LOCAL_API_PORT".to_owned()],
            dependency_order: Some(4),
            depends_on: vec!["service:embedded_state".to_owned()],
            health_check: Some(LocalStackHealthCheck::http(
                "http://127.0.0.1:18080/healthz",
            )),
            shutdown_behavior: "graceful_then_kill".to_owned(),
            log_target: "log://local_stack/api.log".to_owned(),
            restart_class: "required".to_owned(),
            local_only: true,
            test_only: true,
        };
        service.validate()?;
        Ok(service)
    }

    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.service_id, "service id")?;
        local_stack_require_non_empty(&self.service_kind, "service kind")?;
        if self.command.is_empty() {
            return Err(LocalStackContractError::MissingRequiredField("command"));
        }
        local_stack_require_non_empty(&self.working_directory, "working directory")?;
        if self.env_refs.is_empty() {
            return Err(LocalStackContractError::MissingRequiredField("env ref"));
        }
        if self.dependency_order.unwrap_or_default() == 0 {
            return Err(LocalStackContractError::MissingServiceDependencyOrder);
        }
        self.health_check
            .as_ref()
            .ok_or(LocalStackContractError::MissingServiceHealthCheck)?
            .validate()?;
        local_stack_require_non_empty(&self.shutdown_behavior, "shutdown behavior")
            .map_err(|_| LocalStackContractError::MissingServiceShutdownBehavior)?;
        local_stack_require_non_empty(&self.log_target, "log target")
            .map_err(|_| LocalStackContractError::MissingServiceLogTarget)?;
        local_stack_require_non_empty(&self.restart_class, "restart class")?;
        if !self.local_only || !self.test_only {
            return Err(LocalStackContractError::MissingLocalTestMarker);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackPortBinding {
    pub service_id: String,
    pub port: u16,
    pub bind_host: String,
    pub purpose: String,
}

impl LocalStackPortBinding {
    pub fn new(service_id: impl Into<String>, port: u16, purpose: impl Into<String>) -> Self {
        Self {
            service_id: service_id.into(),
            port,
            bind_host: "127.0.0.1".to_owned(),
            purpose: purpose.into(),
        }
    }

    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.service_id, "service id")?;
        local_stack_require_non_empty(&self.purpose, "port purpose")?;
        ensure_loopback_bind_host(&self.bind_host)?;
        if !(LOCAL_STACK_RESERVED_PORT_MIN..=LOCAL_STACK_RESERVED_PORT_MAX).contains(&self.port) {
            return Err(LocalStackContractError::PortOutsideReservedRange(self.port));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackPortRegistry {
    pub registry_id: String,
    pub collision_policy: String,
    pub bindings: Vec<LocalStackPortBinding>,
    pub local_only: bool,
    pub test_only: bool,
}

impl LocalStackPortRegistry {
    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.registry_id, "port registry id")?;
        local_stack_require_non_empty(&self.collision_policy, "collision policy")?;
        if self.bindings.is_empty() {
            return Err(LocalStackContractError::MissingRequiredField(
                "port binding",
            ));
        }
        if !self.local_only || !self.test_only {
            return Err(LocalStackContractError::MissingLocalTestMarker);
        }
        let mut seen = BTreeSet::new();
        for binding in &self.bindings {
            binding.validate()?;
            if !seen.insert(binding.port) {
                return Err(LocalStackContractError::DuplicatePort(binding.port));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackVolumeRef {
    pub volume_id: String,
    pub path_ref: String,
    pub resettable: bool,
    pub local_test_state_marker: Option<String>,
}

impl LocalStackVolumeRef {
    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.volume_id, "volume id")?;
        local_stack_require_non_empty(&self.path_ref, "volume path ref")?;
        if self.resettable
            && self
                .local_test_state_marker
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
        {
            return Err(LocalStackContractError::ResetTargetMissingMarker);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackVolumeRegistry {
    pub registry_id: String,
    pub volumes: Vec<LocalStackVolumeRef>,
    pub local_only: bool,
    pub test_only: bool,
}

impl LocalStackVolumeRegistry {
    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.registry_id, "volume registry id")?;
        if self.volumes.is_empty() {
            return Err(LocalStackContractError::MissingRequiredField("volume"));
        }
        if !self.local_only || !self.test_only {
            return Err(LocalStackContractError::MissingLocalTestMarker);
        }
        for volume in &self.volumes {
            volume.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackResetOperation {
    pub operation_id: String,
    pub target_ref: String,
    pub requires_marker: bool,
    pub marker_ref: Option<String>,
}

impl LocalStackResetOperation {
    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.operation_id, "reset operation id")?;
        local_stack_require_non_empty(&self.target_ref, "reset target ref")?;
        if !self.requires_marker
            || self
                .marker_ref
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
        {
            return Err(LocalStackContractError::ResetTargetMissingMarker);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackResetPlan {
    pub plan_id: String,
    pub operations: Vec<LocalStackResetOperation>,
    pub deterministic: bool,
    pub requires_local_profile: bool,
    pub local_only: bool,
    pub test_only: bool,
}

impl LocalStackResetPlan {
    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.plan_id, "reset plan id")?;
        if self.operations.is_empty() {
            return Err(LocalStackContractError::MissingRequiredField(
                "reset operation",
            ));
        }
        if !self.deterministic
            || !self.requires_local_profile
            || !self.local_only
            || !self.test_only
        {
            return Err(LocalStackContractError::MissingLocalTestMarker);
        }
        for operation in &self.operations {
            operation.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackSeedManifest {
    pub manifest_id: String,
    pub fixture_version: String,
    pub deterministic_seed: String,
    pub fixture_refs: Vec<String>,
    pub local_only: bool,
    pub test_only: bool,
}

impl LocalStackSeedManifest {
    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.manifest_id, "seed manifest id")?;
        local_stack_require_non_empty(&self.fixture_version, "fixture version")?;
        local_stack_require_non_empty(&self.deterministic_seed, "deterministic seed")?;
        if self.fixture_refs.is_empty() {
            return Err(LocalStackContractError::MissingRequiredField("fixture ref"));
        }
        if !self.local_only || !self.test_only {
            return Err(LocalStackContractError::SeedManifestNotTestOnly);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackServiceHealth {
    pub service_id: String,
    pub state: LocalStackHealthState,
    pub last_error: Option<String>,
    pub health_check_ref: String,
}

impl LocalStackServiceHealth {
    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.service_id, "service health service id")?;
        local_stack_require_non_empty(&self.health_check_ref, "health check ref")?;
        if self
            .last_error
            .as_deref()
            .is_some_and(contains_raw_secret_marker)
        {
            return Err(LocalStackContractError::RawSecretInDiagnostic);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStackHealthSnapshot {
    pub schema_version: SchemaVersion,
    pub snapshot_id: String,
    pub profile_id: String,
    pub state: LocalStackHealthState,
    pub service_health: Vec<LocalStackServiceHealth>,
    pub redaction_summary: String,
    pub local_only: bool,
    pub test_only: bool,
}

impl LocalStackHealthSnapshot {
    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.snapshot_id, "health snapshot id")?;
        local_stack_require_non_empty(&self.profile_id, "profile id")?;
        if self.service_health.is_empty() {
            return Err(LocalStackContractError::MissingRequiredField(
                "service health",
            ));
        }
        for service in &self.service_health {
            service.validate()?;
        }
        if self.redaction_summary != "secret_free" {
            return Err(LocalStackContractError::RawSecretInDiagnostic);
        }
        if !self.local_only || !self.test_only {
            return Err(LocalStackContractError::MissingLocalTestMarker);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSecretRecordContract {
    pub secret_ref: String,
    pub secret_kind: String,
    pub redaction_ref: String,
    pub raw_secret_present: bool,
    pub local_only: bool,
    pub test_only: bool,
}

impl LocalSecretRecordContract {
    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.secret_ref, "secret ref")?;
        local_stack_require_non_empty(&self.secret_kind, "secret kind")?;
        local_stack_require_non_empty(&self.redaction_ref, "redaction ref")?;
        if self.raw_secret_present || contains_raw_secret_marker(&self.secret_ref) {
            return Err(LocalStackContractError::RawSecretInDiagnostic);
        }
        if !self.local_only || !self.test_only {
            return Err(LocalStackContractError::MissingLocalTestMarker);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDiagnosticEventContract {
    pub event_type: String,
    pub trace_id: String,
    pub service_id: String,
    pub reason_code: String,
    pub redaction_summary: String,
    pub contains_raw_secret: bool,
    pub local_only: bool,
    pub test_only: bool,
}

impl LocalDiagnosticEventContract {
    pub fn validate(&self) -> Result<(), LocalStackContractError> {
        local_stack_require_non_empty(&self.event_type, "diagnostic event type")?;
        local_stack_require_non_empty(&self.trace_id, "trace id")?;
        local_stack_require_non_empty(&self.service_id, "service id")?;
        local_stack_require_non_empty(&self.reason_code, "reason code")?;
        if self.contains_raw_secret
            || self.redaction_summary != "secret_free"
            || contains_raw_secret_marker(&self.reason_code)
        {
            return Err(LocalStackContractError::RawSecretInDiagnostic);
        }
        if !self.local_only || !self.test_only {
            return Err(LocalStackContractError::MissingLocalTestMarker);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalStackContractError {
    UnsupportedSchemaVersion(ContractError),
    MissingRequiredField(&'static str),
    MissingLocalTestMarker,
    UnsupportedEnvironment(&'static str),
    NonLoopbackBindHost(String),
    NonLoopbackEndpoint(String),
    FutureServiceMissingPhaseGate(String),
    MissingServiceDependencyOrder,
    MissingServiceHealthCheck,
    MissingServiceLogTarget,
    MissingServiceShutdownBehavior,
    InvalidHealthCheck,
    DuplicatePort(u16),
    PortOutsideReservedRange(u16),
    ResetTargetMissingMarker,
    SeedManifestNotTestOnly,
    RawSecretInDiagnostic,
}

impl From<ContractError> for LocalStackContractError {
    fn from(error: ContractError) -> Self {
        Self::UnsupportedSchemaVersion(error)
    }
}

impl fmt::Display for LocalStackContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion(error) => error.fmt(formatter),
            Self::MissingRequiredField(field) => write!(formatter, "{field} is required"),
            Self::MissingLocalTestMarker => formatter.write_str("local/test marker is required"),
            Self::UnsupportedEnvironment(environment) => {
                write!(
                    formatter,
                    "unsupported local-stack environment: {environment}"
                )
            }
            Self::NonLoopbackBindHost(host) => {
                write!(formatter, "bind host is not loopback: {host}")
            }
            Self::NonLoopbackEndpoint(endpoint) => {
                write!(formatter, "endpoint is not loopback/local: {endpoint}")
            }
            Self::FutureServiceMissingPhaseGate(service) => {
                write!(formatter, "future service lacks phase gate: {service}")
            }
            Self::MissingServiceDependencyOrder => {
                formatter.write_str("service dependency order is required")
            }
            Self::MissingServiceHealthCheck => {
                formatter.write_str("service health check is required")
            }
            Self::MissingServiceLogTarget => formatter.write_str("service log target is required"),
            Self::MissingServiceShutdownBehavior => {
                formatter.write_str("service shutdown behavior is required")
            }
            Self::InvalidHealthCheck => formatter.write_str("local health check is invalid"),
            Self::DuplicatePort(port) => write!(formatter, "duplicate local stack port: {port}"),
            Self::PortOutsideReservedRange(port) => {
                write!(
                    formatter,
                    "local stack port is outside reserved range: {port}"
                )
            }
            Self::ResetTargetMissingMarker => {
                formatter.write_str("resettable local target must have a test-state marker")
            }
            Self::SeedManifestNotTestOnly => {
                formatter.write_str("seed manifest must remain local/test only")
            }
            Self::RawSecretInDiagnostic => {
                formatter.write_str("local stack diagnostic contains raw secret material")
            }
        }
    }
}

impl std::error::Error for LocalStackContractError {}

fn local_stack_require_non_empty(
    value: &str,
    field: &'static str,
) -> Result<(), LocalStackContractError> {
    if value.trim().is_empty() {
        Err(LocalStackContractError::MissingRequiredField(field))
    } else {
        Ok(())
    }
}

fn ensure_loopback_bind_host(host: &str) -> Result<(), LocalStackContractError> {
    if matches!(host, "127.0.0.1" | "localhost" | "::1") {
        Ok(())
    } else {
        Err(LocalStackContractError::NonLoopbackBindHost(
            host.to_owned(),
        ))
    }
}

fn endpoint_is_loopback_or_local_ref(endpoint: &str) -> bool {
    endpoint.starts_with("http://127.0.0.1:")
        || endpoint.starts_with("http://localhost:")
        || endpoint.starts_with("http://[::1]:")
        || endpoint.starts_with("local-state://")
        || endpoint.starts_with("log://")
        || endpoint.starts_with("artifact://")
        || endpoint.starts_with("env://")
        || endpoint.starts_with("fixture://")
        || endpoint.starts_with("secret://")
}

fn is_future_service_ref(service_ref: &str) -> bool {
    !matches!(
        service_ref,
        "service:api"
            | "service:worker"
            | "service:embedded_state"
            | "service:overqueue_jobs"
            | "service:overstore_stub"
            | "service:event_audit"
            | "service:node_agent_simulator"
            | "service:developer_ui"
    )
}

fn idempotency_component(value: &str) -> String {
    let mut rendered = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | ':' | '-') {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    while rendered.contains("__") {
        rendered = rendered.replace("__", "_");
    }
    rendered.trim_matches('_').to_owned()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceContext {
    pub trace_id: String,
    pub schema_version: SchemaVersion,
}

impl TraceContext {
    pub fn new(trace_id: impl Into<String>, schema_version: &str) -> Result<Self, ContractError> {
        Ok(Self {
            trace_id: trace_id.into(),
            schema_version: ensure_supported_schema_version(schema_version)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdempotencyRecord {
    pub key: String,
    pub command_type: String,
    pub schema_version: SchemaVersion,
}

impl IdempotencyRecord {
    pub fn new(
        key: impl Into<String>,
        command_type: impl Into<String>,
        schema_version: &str,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            key: key.into(),
            command_type: command_type.into(),
            schema_version: ensure_supported_schema_version(schema_version)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalIdempotencyFingerprint {
    pub environment_class: EnvironmentClass,
    pub endpoint_identity: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub command_type: String,
    pub target_ref: String,
    pub canonical_payload_hash: String,
    pub expected_current_state: Option<String>,
    pub reason: Option<String>,
    pub schema_version: SchemaVersion,
    pub fingerprint: String,
}

impl CanonicalIdempotencyFingerprint {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        environment_class: EnvironmentClass,
        endpoint_identity: impl Into<String>,
        tenant_id: impl Into<String>,
        actor_id: impl Into<String>,
        command_type: impl Into<String>,
        target_ref: impl Into<String>,
        canonical_payload_hash: impl Into<String>,
        expected_current_state: Option<String>,
        reason: Option<String>,
        schema_version: &str,
        fingerprint: impl Into<String>,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            environment_class,
            endpoint_identity: endpoint_identity.into(),
            tenant_id: tenant_id.into(),
            actor_id: actor_id.into(),
            command_type: command_type.into(),
            target_ref: target_ref.into(),
            canonical_payload_hash: canonical_payload_hash.into(),
            expected_current_state,
            reason,
            schema_version: ensure_supported_schema_version(schema_version)?,
            fingerprint: fingerprint.into(),
        })
    }

    pub fn idempotency_key(&self) -> String {
        format!("idem_{}", idempotency_component(&self.fingerprint))
    }

    pub fn new_operation_idempotency_key(&self, trace_id: &str) -> String {
        format!(
            "idem_new_{}_{}",
            idempotency_component(&self.fingerprint),
            idempotency_component(trace_id)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryTimeoutPolicy {
    pub max_retries: u8,
    pub timeout_ms: u64,
    pub bounded: bool,
    pub retryable_classes: Vec<RetryClass>,
    pub non_retryable_reason_families: Vec<String>,
}

impl RetryTimeoutPolicy {
    pub fn bounded(max_retries: u8, timeout_ms: u64) -> Self {
        Self {
            max_retries: max_retries.min(5),
            timeout_ms: timeout_ms.clamp(1, 600_000),
            bounded: true,
            retryable_classes: vec![RetryClass::SafeRetry, RetryClass::RetryAfter],
            non_retryable_reason_families: [
                "schema",
                "auth",
                "policy",
                "phase",
                "credential",
                "idempotency_duplicate",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorDecodeRecord {
    pub source_family: String,
    pub reason_code: String,
    pub retry_class: RetryClass,
    pub exit_class: ExitCodeClass,
    pub remediation_hint: String,
    pub raw_internal_error_exposed: bool,
}

impl ErrorDecodeRecord {
    pub fn new(
        source_family: impl Into<String>,
        reason_code: impl Into<String>,
        retry_class: RetryClass,
        exit_class: ExitCodeClass,
        remediation_hint: impl Into<String>,
    ) -> Self {
        Self {
            source_family: source_family.into(),
            reason_code: reason_code.into(),
            retry_class,
            exit_class,
            remediation_hint: remediation_hint.into(),
            raw_internal_error_exposed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalIdempotencyCacheRecord {
    pub cache_scope: String,
    pub profile_name: String,
    pub environment_class: EnvironmentClass,
    pub command_fingerprint: String,
    pub idempotency_key: String,
    pub owner_only: bool,
    pub contains_private_payload: bool,
    pub resettable: bool,
    pub inspectable: bool,
}

impl LocalIdempotencyCacheRecord {
    pub fn new(
        profile_name: impl Into<String>,
        environment_class: EnvironmentClass,
        command_fingerprint: impl Into<String>,
        idempotency_key: impl Into<String>,
    ) -> Self {
        Self {
            cache_scope: "profile_environment".to_owned(),
            profile_name: profile_name.into(),
            environment_class,
            command_fingerprint: command_fingerprint.into(),
            idempotency_key: idempotency_key.into(),
            owner_only: true,
            contains_private_payload: false,
            resettable: true,
            inspectable: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    pub reason_code: String,
    pub message: String,
    pub phase_gate: String,
    pub retry_class: RetryClass,
    pub trace_id: Option<String>,
    pub schema_version: SchemaVersion,
}

impl ApiError {
    pub fn new(
        reason_code: impl Into<String>,
        message: impl Into<String>,
        phase_gate: impl Into<String>,
        retry_class: RetryClass,
        trace_id: Option<String>,
        schema_version: &str,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            reason_code: reason_code.into(),
            message: message.into(),
            phase_gate: phase_gate.into(),
            retry_class,
            trace_id,
            schema_version: ensure_supported_schema_version(schema_version)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputEnvelope {
    pub ok: bool,
    pub schema_version: SchemaVersion,
    pub reason_code: Option<String>,
    pub retry_class: RetryClass,
    pub trace_id: Option<String>,
    pub audit_refs: Vec<String>,
    pub warnings: Vec<String>,
    pub exit_code: i32,
    pub exit_class: ExitCodeClass,
    pub lifecycle: CommandLifecycle,
    pub timing_ms: u64,
    pub diagnostic_bundle: Option<DiagnosticBundle>,
    pub capability_snapshot: Option<CapabilitySnapshot>,
}

impl OutputEnvelope {
    pub fn success(schema_version: &str) -> Result<Self, ContractError> {
        Ok(Self {
            ok: true,
            schema_version: ensure_supported_schema_version(schema_version)?,
            reason_code: None,
            retry_class: RetryClass::NotRetryable,
            trace_id: None,
            audit_refs: Vec::new(),
            warnings: Vec::new(),
            exit_code: ExitCodeClass::Success.code(),
            exit_class: ExitCodeClass::Success,
            lifecycle: CommandLifecycle::new(vec![
                CommandLifecycleState::Parsed,
                CommandLifecycleState::Completed,
            ]),
            timing_ms: 0,
            diagnostic_bundle: None,
            capability_snapshot: None,
        })
    }

    pub fn failure(
        schema_version: &str,
        reason_code: impl Into<String>,
    ) -> Result<Self, ContractError> {
        Self::failure_with_exit(
            schema_version,
            reason_code,
            ExitCodeClass::Config,
            RetryClass::NotRetryable,
        )
    }

    pub fn failure_with_exit(
        schema_version: &str,
        reason_code: impl Into<String>,
        exit_class: ExitCodeClass,
        retry_class: RetryClass,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            ok: false,
            schema_version: ensure_supported_schema_version(schema_version)?,
            reason_code: Some(reason_code.into()),
            retry_class,
            trace_id: None,
            audit_refs: Vec::new(),
            warnings: Vec::new(),
            exit_code: exit_class.code(),
            exit_class,
            lifecycle: CommandLifecycle::new(vec![
                CommandLifecycleState::Parsed,
                CommandLifecycleState::Failed,
            ]),
            timing_ms: 0,
            diagnostic_bundle: None,
            capability_snapshot: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticBundle {
    pub command_name: String,
    pub schema_version: SchemaVersion,
    pub profile_name: Option<String>,
    pub endpoint_fingerprint: Option<String>,
    pub schema_versions: Vec<String>,
    pub trace_ids: Vec<String>,
    pub reason_codes: Vec<String>,
    pub retry_count: u8,
    pub dependency_status: Vec<String>,
    pub redaction_policy: String,
}

impl DiagnosticBundle {
    pub fn new(
        command_name: impl Into<String>,
        schema_version: &str,
    ) -> Result<Self, ContractError> {
        let schema_version = ensure_supported_schema_version(schema_version)?;
        Ok(Self {
            command_name: command_name.into(),
            schema_versions: vec![schema_version.raw().to_owned()],
            schema_version,
            profile_name: None,
            endpoint_fingerprint: None,
            trace_ids: Vec::new(),
            reason_codes: Vec::new(),
            retry_count: 0,
            dependency_status: vec!["local_contracts_available".to_owned()],
            redaction_policy: "secret_free_refs_only".to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local_profile() -> CliProfile {
        CliProfile {
            name: "local-dev".to_owned(),
            endpoint: "http://127.0.0.1:18080/overgate".to_owned(),
            endpoint_fingerprint: "fp_local_dev".to_owned(),
            environment: EnvironmentClass::Local,
            tenant_id: "tenant_local".to_owned(),
            actor_id: "actor_local".to_owned(),
            credential_namespace: "local-dev".to_owned(),
            allowed_credential_classes: vec![CredentialReferenceClass::Fixture],
            fixture_allowance: FixtureAllowance::LocalOnly,
            default_output_mode: "json".to_owned(),
            confirmation_policy: ConfirmationPolicy::ConfirmSensitive,
            schema_pins: vec![SUPPORTED_SCHEMA_VERSION.to_owned()],
            test_harness_profile: false,
        }
    }

    fn fixture_credential() -> CredentialReference {
        CredentialReference {
            reference_id: "fixture://local-dev/key-1".to_owned(),
            class: CredentialReferenceClass::Fixture,
            namespace: "local-dev".to_owned(),
            key_id: "key-1".to_owned(),
            revoked: false,
            expired: false,
        }
    }

    #[test]
    fn accepts_supported_cli_schema_version() {
        let parsed = ensure_supported_schema_version(SUPPORTED_SCHEMA_VERSION).unwrap();
        assert_eq!(parsed.family(), CLI_SCHEMA_FAMILY);
        assert_eq!(parsed.major(), 0);
        assert_eq!(parsed.minor(), 1);
    }

    #[test]
    fn rejects_missing_schema_version() {
        assert_eq!(
            ensure_supported_schema_version("").unwrap_err(),
            ContractError::MissingSchemaVersion
        );
    }

    #[test]
    fn rejects_unknown_schema_family() {
        assert!(matches!(
            ensure_supported_schema_version("admin-ui.v0.1"),
            Err(ContractError::UnsupportedSchemaVersion { .. })
        ));
    }

    #[test]
    fn rejects_future_minor_schema_version() {
        assert!(matches!(
            ensure_supported_schema_version("cli-command.v0.2"),
            Err(ContractError::UnsupportedSchemaVersion { .. })
        ));
    }

    #[test]
    fn rejects_malformed_schema_version() {
        assert!(matches!(
            ensure_supported_schema_version("cli-command-0-1"),
            Err(ContractError::InvalidSchemaVersion(_))
        ));
    }

    #[test]
    fn exposes_contract_projection_metadata() {
        let set = cli_contract_set();
        assert_eq!(set.source_root, CONTRACT_SOURCE_ROOT);
        assert_eq!(set.projection_status, GENERATED_CONTRACT_STATUS);
    }

    fn harness_fixture_key() -> FixtureKey {
        FixtureKey::test_only("key:fixture:local_builder", "fixture_local_builder")
    }

    fn harness_step() -> ScenarioStep {
        ScenarioStep::new(
            "step_cli_noop",
            ScenarioActionKind::Cli,
            vec!["assertion:phase0:trace_order".to_owned()],
            30_000,
        )
        .unwrap()
    }

    #[test]
    fn exposes_integration_harness_contract_projection_metadata() {
        let set = integration_harness_contract_set();
        assert_eq!(set.schema_family, INTEGRATION_HARNESS_SCHEMA_FAMILY);
        assert_eq!(
            set.schema_version,
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION
        );
        assert_eq!(set.source_root, CONTRACT_SOURCE_ROOT);
        assert_eq!(set.projection_status, GENERATED_CONTRACT_STATUS);
    }

    #[test]
    fn accepts_supported_integration_harness_schema_version() {
        let parsed = ensure_supported_integration_harness_schema_version(
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();

        assert_eq!(parsed.family(), INTEGRATION_HARNESS_SCHEMA_FAMILY);
        assert_eq!(parsed.major(), 0);
        assert_eq!(parsed.minor(), 1);
    }

    #[test]
    fn rejects_incompatible_integration_harness_schema_version() {
        assert!(matches!(
            ensure_supported_integration_harness_schema_version("integration-harness.v1.0"),
            Err(ContractError::UnsupportedSchemaVersion { .. })
        ));
        assert!(matches!(
            ensure_supported_integration_harness_schema_version(SUPPORTED_SCHEMA_VERSION),
            Err(ContractError::UnsupportedSchemaVersion { .. })
        ));
    }

    #[test]
    fn exposes_local_stack_contract_projection_metadata() {
        let set = local_development_stack_contract_set();
        assert_eq!(set.schema_family, LOCAL_DEVELOPMENT_STACK_SCHEMA_FAMILY);
        assert_eq!(
            set.schema_version,
            SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION
        );
        assert_eq!(set.source_root, CONTRACT_SOURCE_ROOT);
        assert_eq!(set.projection_status, GENERATED_CONTRACT_STATUS);
    }

    #[test]
    fn local_stack_schema_version_accepts_only_local_stack_family() {
        let parsed = ensure_supported_local_development_stack_schema_version(
            SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
        )
        .unwrap();

        assert_eq!(parsed.family(), LOCAL_DEVELOPMENT_STACK_SCHEMA_FAMILY);
        assert_eq!(parsed.major(), 0);
        assert_eq!(parsed.minor(), 1);
        assert!(matches!(
            ensure_supported_local_development_stack_schema_version("local-development-stack.v0.2"),
            Err(ContractError::UnsupportedSchemaVersion { .. })
        ));
        assert!(matches!(
            ensure_supported_local_development_stack_schema_version(
                SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION
            ),
            Err(ContractError::UnsupportedSchemaVersion { .. })
        ));
    }

    #[test]
    fn exposes_shared_schema_package_contract_projection_metadata() {
        let set = shared_schema_package_contract_set();
        assert_eq!(set.schema_family, SHARED_SCHEMA_PACKAGE_SCHEMA_FAMILY);
        assert_eq!(
            set.schema_version,
            SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION
        );
        assert_eq!(set.source_root, CONTRACT_SOURCE_ROOT);
        assert_eq!(set.projection_status, GENERATED_CONTRACT_STATUS);
    }

    #[test]
    fn shared_schema_package_schema_version_accepts_only_shared_schema_family() {
        let parsed = ensure_supported_shared_schema_package_schema_version(
            SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION,
        )
        .unwrap();

        assert_eq!(parsed.family(), SHARED_SCHEMA_PACKAGE_SCHEMA_FAMILY);
        assert_eq!(parsed.major(), 0);
        assert_eq!(parsed.minor(), 1);
        assert!(matches!(
            ensure_supported_shared_schema_package_schema_version("shared-schema-package.v0.2"),
            Err(ContractError::UnsupportedSchemaVersion { .. })
        ));
        assert!(matches!(
            ensure_supported_shared_schema_package_schema_version(
                SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION
            ),
            Err(ContractError::UnsupportedSchemaVersion { .. })
        ));
    }

    #[test]
    fn shared_schema_phase2_contract_covers_layout_and_common_primitives() {
        let contract = SharedSchemaPhase2Contract::canonical().unwrap();
        contract.validate().unwrap();

        assert!(contract
            .source_roots
            .contains(&"packages/schemas/overrid_contracts/v0".to_owned()));
        assert!(contract
            .generated_output_roots
            .contains(&"packages/schemas/overrid_contracts/src/lib.rs".to_owned()));
        assert!(contract
            .fixture_roots
            .contains(&"packages/schemas/overrid_contracts/fixtures/valid".to_owned()));
        assert!(contract
            .compatibility_report_roots
            .contains(&"packages/schemas/overrid_contracts/compatibility".to_owned()));
        assert!(contract
            .compatibility_report_roots
            .contains(&"docs/specs/generated".to_owned()));
        assert!(contract
            .internal_binary_projection_roots
            .contains(&"packages/schemas/overrid_contracts/protobuf/internal".to_owned()));
        assert!(!contract.generated_in_source_directories);

        for family in REQUIRED_SHARED_SCHEMA_OBJECT_FAMILIES {
            assert!(contract.has_typed_ref_family(family), "missing {family}");
        }
        assert!(contract.has_reason_code("schema.unsupported_version"));
        assert!(contract.has_reason_code("validation.typed_ref_required"));
        assert!(contract.has_reason_code("policy.privacy_class_missing"));
        assert!(contract.has_reason_code("dependency.generated_source_misplaced"));
    }

    #[test]
    fn shared_schema_phase2_rejects_generated_outputs_inside_source_roots() {
        let mut contract = SharedSchemaPhase2Contract::canonical().unwrap();
        contract.generated_output_roots =
            vec!["packages/schemas/overrid_contracts/v0/generated_types.rs".to_owned()];

        assert_eq!(
            contract.validate().unwrap_err(),
            SharedSchemaPackageContractError::GeneratedOutputInsideSource
        );

        let mut contract = SharedSchemaPhase2Contract::canonical().unwrap();
        contract.generated_in_source_directories = true;

        assert_eq!(
            contract.validate().unwrap_err(),
            SharedSchemaPackageContractError::GeneratedOutputInsideSource
        );
    }

    #[test]
    fn shared_schema_phase2_rejects_untyped_or_unversioned_public_refs() {
        let mut untyped = SharedSchemaTypedRefPrimitive::new(
            "tenant_ref",
            "tenant",
            "",
            SharedSchemaPrivacyClass::TenantPrivate,
            true,
            true,
        );
        assert!(matches!(
            untyped.validate(),
            Err(SharedSchemaPackageContractError::UntypedRefPrimitive(name))
                if name == "tenant_ref"
        ));

        untyped.ref_prefix = "tenant:".to_owned();
        untyped.unstructured_string_allowed = true;
        assert!(matches!(
            untyped.validate(),
            Err(SharedSchemaPackageContractError::UntypedRefPrimitive(name))
                if name == "tenant_ref"
        ));

        let unversioned_public = SharedSchemaTypedRefPrimitive::new(
            "app_ref",
            "app",
            "app:",
            SharedSchemaPrivacyClass::Public,
            true,
            false,
        );
        assert!(matches!(
            unversioned_public.validate(),
            Err(SharedSchemaPackageContractError::UnversionedPublicRef(name))
                if name == "app_ref"
        ));
    }

    #[test]
    fn shared_schema_phase2_lifecycle_privacy_and_reason_codes_are_stable() {
        let lifecycle = SharedSchemaLifecyclePrimitives::phase2_default().unwrap();
        lifecycle.validate().unwrap();
        assert_eq!(lifecycle.trace_id_prefix, "trace_");
        assert_eq!(lifecycle.idempotency_key_prefix, "idem_");
        assert_eq!(lifecycle.request_hash_prefix, "hash_");
        assert!(lifecycle.previous_major_supported);

        assert!(SharedSchemaPrivacyClass::Public.allows_public_object());
        assert!(SharedSchemaPrivacyClass::RedactedDiagnostic.allows_public_object());
        assert!(SharedSchemaPrivacyClass::EncryptedPrivate.requires_redaction());

        let mut reason = SharedSchemaReasonCodeEntry::new(
            "schema.unsupported_version",
            SharedSchemaRetryability::NotRetryable,
            vec!["schema_version".to_owned()],
            true,
        );
        reason.validate().unwrap();
        assert_eq!(reason.retryability.as_str(), "not_retryable");

        reason.correction_fields.clear();
        assert!(matches!(
            reason.validate(),
            Err(SharedSchemaPackageContractError::MissingCorrectionFields(code))
                if code == "schema.unsupported_version"
        ));
    }

    #[test]
    fn shared_schema_phase3_contract_covers_phase0_phase1_modules() {
        let contract = SharedSchemaPhase3Contract::canonical().unwrap();
        contract.validate().unwrap();

        for module in REQUIRED_SHARED_SCHEMA_PHASE3_MODULES {
            assert!(contract.module(module).is_some(), "missing {module}");
        }

        let command = contract.module("command").unwrap();
        assert_eq!(
            command.module_family,
            SharedSchemaPhase3ModuleFamily::CommandApiError
        );
        assert!(command.has_required_ref("tenant_ref"));
        assert!(command.has_required_ref("actor_ref"));
        assert!(command.has_required_field("trace_id"));
        assert!(command.has_required_field("idempotency_key"));
        assert!(command.has_required_field("payload_hash"));
        assert!(command.has_required_field("signature_metadata"));
        assert!(!command.raw_secret_values_allowed);
        assert!(!command.private_key_material_allowed);
        assert!(command.strict_unknown_field_rejection);

        let api_error = contract.module("api_error").unwrap();
        assert!(api_error.has_required_field("reason_code"));
        assert!(api_error.has_required_field("trace_id"));
        assert!(api_error.has_required_field("retryability"));
        assert!(api_error.has_required_field("correction_fields"));

        let tenant = contract.module("tenant").unwrap();
        assert!(tenant.has_required_field("delegated_access"));
        assert!(tenant.has_required_field("privacy_class"));

        let event = contract.module("event").unwrap();
        assert!(event.has_required_ref("policy_ref"));
        assert!(event.has_required_field("action"));
        assert!(event.has_required_field("decision"));
        assert!(event.append_only_record);
        assert!(event.has_required_field("sequence"));
        assert!(event.has_required_field("privacy_class"));

        let audit = contract.module("audit").unwrap();
        assert!(audit.has_required_field("source_service"));
        assert!(audit.has_required_field("subject_id"));

        let manifest = contract.module("workload_manifest").unwrap();
        assert!(manifest.typed_secret_refs_required);
        assert!(manifest.has_required_ref("secret_ref"));
        assert!(!manifest.untyped_capability_blobs_allowed);

        let credential = contract.module("credential_key_metadata").unwrap();
        assert!(credential.has_required_ref("signer_ref"));
        assert!(credential.has_required_ref("secret_ref"));
        assert!(credential.has_required_field("key_rotation"));
        assert!(credential.has_required_field("revocation"));
    }

    #[test]
    fn shared_schema_phase3_rejects_authority_or_security_drift() {
        let mut command = SharedSchemaPhase3Contract::canonical()
            .unwrap()
            .module("command")
            .unwrap()
            .clone();
        command.required_refs.retain(|item| item != "tenant_ref");
        assert!(matches!(
            command.validate(),
            Err(SharedSchemaPhase3ContractError::MissingTenantActorRefs(module))
                if module == "command"
        ));

        let mut identity = SharedSchemaPhase3Contract::canonical()
            .unwrap()
            .module("identity")
            .unwrap()
            .clone();
        identity.raw_secret_values_allowed = true;
        assert!(matches!(
            identity.validate(),
            Err(SharedSchemaPhase3ContractError::RawSecretValuesAllowed(module))
                if module == "identity"
        ));

        let mut event = SharedSchemaPhase3Contract::canonical()
            .unwrap()
            .module("event")
            .unwrap()
            .clone();
        event.strict_unknown_field_rejection = false;
        assert!(matches!(
            event.validate(),
            Err(SharedSchemaPhase3ContractError::UnknownSensitiveFieldsAllowed(module))
                if module == "event"
        ));

        let mut manifest = SharedSchemaPhase3Contract::canonical()
            .unwrap()
            .module("workload_manifest")
            .unwrap()
            .clone();
        manifest.required_refs.retain(|item| item != "secret_ref");
        assert!(matches!(
            manifest.validate(),
            Err(SharedSchemaPhase3ContractError::MissingTypedSecretRef(module))
                if module == "workload_manifest"
        ));

        let mut credential = SharedSchemaPhase3Contract::canonical()
            .unwrap()
            .module("credential_key_metadata")
            .unwrap()
            .clone();
        credential.private_key_material_allowed = true;
        assert!(matches!(
            credential.validate(),
            Err(SharedSchemaPhase3ContractError::PrivateKeyMaterialAllowed(module))
                if module == "credential_key_metadata"
        ));
    }

    #[test]
    fn shared_schema_phase3_rejects_incomplete_api_error_and_append_only_modules() {
        let mut api_error = SharedSchemaPhase3Contract::canonical()
            .unwrap()
            .module("api_error")
            .unwrap()
            .clone();
        api_error.required_fields.retain(|item| item != "trace_id");
        assert!(matches!(
            api_error.validate(),
            Err(SharedSchemaPhase3ContractError::IncompleteApiError(field))
                if field == "trace_id"
        ));

        let mut audit = SharedSchemaPhase3Contract::canonical()
            .unwrap()
            .module("audit")
            .unwrap()
            .clone();
        audit.required_fields.retain(|item| item != "sequence");
        assert!(matches!(
            audit.validate(),
            Err(SharedSchemaPhase3ContractError::MissingAppendOnlySequence(module))
                if module == "audit"
        ));

        let mut credential = SharedSchemaPhase3Contract::canonical()
            .unwrap()
            .module("credential_key_metadata")
            .unwrap()
            .clone();
        credential
            .required_fields
            .retain(|item| item != "revocation");
        assert!(matches!(
            credential.validate(),
            Err(SharedSchemaPhase3ContractError::IncompleteCredentialKeyMetadata)
        ));
    }

    #[test]
    fn shared_schema_phase4_contract_covers_generation_outputs() {
        let contract = SharedSchemaPhase4GenerationContract::canonical().unwrap();
        contract.validate().unwrap();

        assert_eq!(contract.toolchain_name, "rust-json-schema-projection-v0");
        assert_eq!(contract.canonical_source, PHASE4_CANONICAL_SCHEMA_SOURCE);
        assert!(contract.rust_first);
        assert!(contract.typescript_second);
        assert!(contract.generated_outputs_non_authoritative);
        assert!(!contract.hand_edited_generated_files_allowed);
        assert!(!contract.protobuf_public_authority_allowed);
        assert!(contract
            .source_hash_inputs
            .contains(&PHASE4_TECH_STACK_SOURCE.to_owned()));
        assert!(contract.generation_commands.iter().any(|command| {
            command.command_name == "rust_projection_check"
                && command
                    .command
                    .contains("cargo test -p overrid-contracts shared_schema_phase4")
        }));

        let rust = contract
            .rust_outputs
            .iter()
            .find(|output| output.target == SharedSchemaPhase4ProjectionTarget::Rust)
            .unwrap();
        assert_eq!(rust.path, PHASE4_RUST_OUTPUT_PATH);
        assert!(rust.non_authoritative);
        assert!(rust.contains_redaction_metadata);
        assert!(rust.contains_reason_code_metadata);
        assert_eq!(SharedSchemaPhase4ProjectionTarget::Docs.as_str(), "docs");

        assert_eq!(
            contract.docs_projection.output_path,
            PHASE4_GENERATED_DOCS_PATH
        );
        assert!(contract.docs_projection.source_to_doc_trace);
        assert!(contract
            .docs_projection
            .required_sections
            .contains(&"Reason Codes".to_owned()));
        assert_eq!(
            contract.typescript_web_projection.output_root,
            PHASE4_TYPESCRIPT_OUTPUT_ROOT
        );
        assert!(!contract.typescript_web_projection.source_of_truth_allowed);
        assert_eq!(
            contract.protobuf_projection.output_root,
            PHASE4_PROTOBUF_INTERNAL_ROOT
        );
        assert!(contract.protobuf_projection.json_schema_source_required);
        assert!(
            !contract
                .protobuf_projection
                .public_object_definition_allowed
        );
    }

    #[test]
    fn shared_schema_phase4_rejects_authority_and_generation_drift() {
        let mut contract = SharedSchemaPhase4GenerationContract::canonical().unwrap();
        contract.hand_edited_generated_files_allowed = true;
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase4ContractError::GeneratedOutputHandEditAllowed)
        ));

        let mut contract = SharedSchemaPhase4GenerationContract::canonical().unwrap();
        contract
            .source_hash_inputs
            .retain(|input| input != "packages/schemas/overrid_contracts/codegen_manifest.json");
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase4ContractError::MissingSourceInput(path))
                if path == "packages/schemas/overrid_contracts/codegen_manifest.json"
        ));

        let mut contract = SharedSchemaPhase4GenerationContract::canonical().unwrap();
        contract.generation_commands[0].deterministic = false;
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase4ContractError::NonDeterministicCommand(name))
                if name == "rust_projection_check"
        ));

        let mut contract = SharedSchemaPhase4GenerationContract::canonical().unwrap();
        contract.rust_outputs[0].validator_entrypoint.clear();
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase4ContractError::MissingRustValidator)
        ));
    }

    #[test]
    fn shared_schema_phase4_rejects_docs_typescript_and_protobuf_drift() {
        let mut contract = SharedSchemaPhase4GenerationContract::canonical().unwrap();
        contract.docs_projection.source_to_doc_trace = false;
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase4ContractError::MissingDocsTrace)
        ));

        let mut contract = SharedSchemaPhase4GenerationContract::canonical().unwrap();
        contract
            .typescript_web_projection
            .blocked_until
            .retain(|gate| gate != "docs_trace_validated");
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase4ContractError::MissingTypeScriptGate(gate))
                if gate == "docs_trace_validated"
        ));

        let mut contract = SharedSchemaPhase4GenerationContract::canonical().unwrap();
        contract.typescript_web_projection.source_of_truth_allowed = true;
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase4ContractError::TypeScriptSourceAuthority)
        ));

        let mut contract = SharedSchemaPhase4GenerationContract::canonical().unwrap();
        contract
            .protobuf_projection
            .public_object_definition_allowed = true;
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase4ContractError::ProtobufPublicAuthority)
        ));
    }

    #[test]
    fn shared_schema_phase5_contract_covers_strict_validation_parse_and_envelopes() {
        let contract = SharedSchemaPhase5ValidationContract::canonical().unwrap();
        contract.validate().unwrap();

        assert!(
            contract
                .strict_validation_defaults
                .unknown_fields_rejected_for_sensitive_families
        );
        assert!(
            contract
                .strict_validation_defaults
                .extension_maps_default_denied
        );
        for family in REQUIRED_SHARED_SCHEMA_PHASE5_SENSITIVE_FAMILIES {
            assert!(contract
                .strict_validation_defaults
                .sensitive_families
                .contains(&(*family).to_owned()));
        }
        assert!(contract.parse_helpers[0]
            .stable_error_reasons
            .contains(&"schema.unsupported_version".to_owned()));

        let command = contract
            .envelope_assertions
            .iter()
            .find(|assertion| assertion.envelope_family == "command")
            .unwrap();
        assert!(command.required_fields.contains(&"trace_id".to_owned()));
        assert!(command
            .required_fields
            .contains(&"idempotency_key".to_owned()));
        assert!(command.required_refs.contains(&"signature_ref".to_owned()));

        let validation_registry = contract
            .reason_code_registries
            .iter()
            .find(|registry| registry.domain == "validation")
            .unwrap();
        assert!(validation_registry
            .reason_codes
            .contains(&"schema.diagnostic_secret_leak".to_owned()));
        assert!(!validation_registry.free_form_replacements_allowed);

        assert_eq!(
            contract.rust_projection.validator_entrypoint,
            "SharedSchemaPhase5ValidationContract::canonical().validate()"
        );
        assert!(contract
            .source_hash_inputs
            .contains(&PHASE5_TECH_STACK_SOURCE.to_owned()));
    }

    #[test]
    fn shared_schema_phase5_lists_reason_codes_by_domain() {
        let validation_codes = list_reason_codes("validation");
        assert_eq!(
            validation_codes.len(),
            REQUIRED_SHARED_SCHEMA_PHASE5_VALIDATION_REASON_CODES.len()
        );
        assert!(validation_codes.contains(&"schema.unknown_sensitive_field".to_owned()));
        assert!(validation_codes.contains(&"schema.diagnostic_secret_leak".to_owned()));

        for domain in REQUIRED_SHARED_SCHEMA_PHASE5_REASON_DOMAINS {
            assert!(
                !list_reason_codes(domain).is_empty(),
                "missing Phase 5 reason codes for domain {domain}"
            );
        }
    }

    #[test]
    fn shared_schema_phase5_reason_code_lookup_returns_empty_for_unknown_domain() {
        assert!(list_reason_codes("unknown").is_empty());
        assert!(list_reason_codes("").is_empty());
    }

    #[test]
    fn shared_schema_phase5_rejects_unknown_fields_and_untyped_extension_maps() {
        let mut contract = SharedSchemaPhase5ValidationContract::canonical().unwrap();
        contract
            .strict_validation_defaults
            .unknown_fields_rejected_for_sensitive_families = false;
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase5ContractError::UnknownSensitiveFieldsAllowed)
        ));

        let mut contract = SharedSchemaPhase5ValidationContract::canonical().unwrap();
        contract
            .strict_validation_defaults
            .extension_maps_default_denied = false;
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase5ContractError::ExtensionMapsDefaultAllowed)
        ));

        let mut contract = SharedSchemaPhase5ValidationContract::canonical().unwrap();
        contract.strict_validation_defaults.allowed_extension_maps[0].typed_values_required = false;
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase5ContractError::ExtensionMapRuleIncomplete(surface))
                if surface == "low_risk_metadata"
        ));
    }

    #[test]
    fn shared_schema_phase5_rejects_parse_envelope_and_reason_code_drift() {
        let mut contract = SharedSchemaPhase5ValidationContract::canonical().unwrap();
        contract.parse_helpers[0]
            .stable_error_reasons
            .retain(|reason| reason != "schema.unsupported_version");
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase5ContractError::MissingStableErrorReason(reason))
                if reason == "schema.unsupported_version"
        ));

        let mut contract = SharedSchemaPhase5ValidationContract::canonical().unwrap();
        let command = contract
            .envelope_assertions
            .iter_mut()
            .find(|assertion| assertion.envelope_family == "command")
            .unwrap();
        command.required_fields.retain(|field| field != "trace_id");
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase5ContractError::MissingEnvelopeField { envelope, field })
                if envelope == "command" && field == "trace_id"
        ));

        let mut contract = SharedSchemaPhase5ValidationContract::canonical().unwrap();
        let registry = contract
            .reason_code_registries
            .iter_mut()
            .find(|registry| registry.domain == "validation")
            .unwrap();
        registry.free_form_replacements_allowed = true;
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase5ContractError::FreeFormReasonCodesAllowed(domain))
                if domain == "validation"
        ));
    }

    #[test]
    fn shared_schema_phase5_rejects_diagnostic_and_projection_leaks() {
        let mut contract = SharedSchemaPhase5ValidationContract::canonical().unwrap();
        contract.redaction_diagnostics[0].private_payload_leak_allowed = true;
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase5ContractError::DiagnosticLeakAllowed(surface))
                if surface == "generated_docs"
        ));

        let mut contract = SharedSchemaPhase5ValidationContract::canonical().unwrap();
        contract.redaction_diagnostics[0].redaction_hints.clear();
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase5ContractError::MissingRedactionHint(surface))
                if surface == "generated_docs"
        ));

        let mut contract = SharedSchemaPhase5ValidationContract::canonical().unwrap();
        contract.rust_projection.non_authoritative = false;
        assert!(matches!(
            contract.validate(),
            Err(SharedSchemaPhase5ContractError::RustProjectionAuthorityDrift)
        ));
    }

    #[test]
    fn local_stack_profile_rejects_non_loopback_and_future_service_without_gate() {
        let mut profile =
            LocalStackProfileContract::local_default("profile:local_default").unwrap();
        assert_eq!(profile.environment_class.as_str(), "local");
        assert!(profile
            .required_phase_gates
            .iter()
            .any(|gate| gate.as_str() == "owning_service_required"));

        profile.default_bind_host = "0.0.0.0".to_owned();
        assert!(matches!(
            profile.validate(),
            Err(LocalStackContractError::NonLoopbackBindHost(host)) if host == "0.0.0.0"
        ));

        let mut future_service =
            LocalStackProfileContract::local_default("profile:future").unwrap();
        future_service.enabled_services = vec!["service:overvault".to_owned()];
        future_service.required_phase_gates = vec![LocalStackPhaseGateState::BuildablePhase0];
        assert!(matches!(
            future_service.validate(),
            Err(LocalStackContractError::FutureServiceMissingPhaseGate(service))
                if service == "service:overvault"
        ));

        let mut missing_marker =
            LocalStackProfileContract::local_default("profile:missing_marker").unwrap();
        missing_marker.local_only = false;
        assert!(matches!(
            missing_marker.validate(),
            Err(LocalStackContractError::MissingLocalTestMarker)
        ));
    }

    #[test]
    fn local_stack_service_definition_requires_metadata_and_loopback_health() {
        let service = LocalStackServiceDefinitionContract::local_api().unwrap();
        assert_eq!(service.service_id, "service:api");
        assert_eq!(service.dependency_order, Some(4));
        service.validate().unwrap();

        let mut missing_health = service.clone();
        missing_health.health_check = None;
        assert!(matches!(
            missing_health.validate(),
            Err(LocalStackContractError::MissingServiceHealthCheck)
        ));

        let mut missing_order = service.clone();
        missing_order.dependency_order = None;
        assert!(matches!(
            missing_order.validate(),
            Err(LocalStackContractError::MissingServiceDependencyOrder)
        ));

        let mut missing_log = service.clone();
        missing_log.log_target.clear();
        assert!(matches!(
            missing_log.validate(),
            Err(LocalStackContractError::MissingServiceLogTarget)
        ));

        let mut missing_shutdown = service.clone();
        missing_shutdown.shutdown_behavior.clear();
        assert!(matches!(
            missing_shutdown.validate(),
            Err(LocalStackContractError::MissingServiceShutdownBehavior)
        ));

        let mut non_loopback = service;
        non_loopback.health_check = Some(LocalStackHealthCheck::http(
            "http://192.0.2.4:18080/healthz",
        ));
        assert!(matches!(
            non_loopback.validate(),
            Err(LocalStackContractError::NonLoopbackEndpoint(endpoint))
                if endpoint == "http://192.0.2.4:18080/healthz"
        ));
    }

    #[test]
    fn local_stack_port_registry_rejects_collisions_and_non_reserved_ports() {
        let registry = LocalStackPortRegistry {
            registry_id: "port_registry:reserved_18080_18085".to_owned(),
            collision_policy: "fail_before_startup".to_owned(),
            bindings: vec![
                LocalStackPortBinding::new("service:api", 18_080, "api_health"),
                LocalStackPortBinding::new("service:worker", 18_081, "worker_health"),
            ],
            local_only: true,
            test_only: true,
        };
        registry.validate().unwrap();

        let mut duplicate = registry.clone();
        duplicate.bindings.push(LocalStackPortBinding::new(
            "service:overstore_stub",
            18_080,
            "artifact_stub",
        ));
        assert!(matches!(
            duplicate.validate(),
            Err(LocalStackContractError::DuplicatePort(18_080))
        ));

        let mut outside_range = registry.clone();
        outside_range.bindings[0].port = 18_090;
        assert!(matches!(
            outside_range.validate(),
            Err(LocalStackContractError::PortOutsideReservedRange(18_090))
        ));

        let mut non_loopback = registry;
        non_loopback.bindings[0].bind_host = "0.0.0.0".to_owned();
        assert!(matches!(
            non_loopback.validate(),
            Err(LocalStackContractError::NonLoopbackBindHost(host)) if host == "0.0.0.0"
        ));
    }

    #[test]
    fn local_stack_reset_seed_secret_and_diagnostic_contracts_are_safe() {
        let reset_plan = LocalStackResetPlan {
            plan_id: "reset_plan:local_default".to_owned(),
            operations: vec![LocalStackResetOperation {
                operation_id: "reset:embedded_state".to_owned(),
                target_ref: "local-state://embedded_state".to_owned(),
                requires_marker: true,
                marker_ref: Some(
                    "local-state://embedded_state/.overrid-local-test-state".to_owned(),
                ),
            }],
            deterministic: true,
            requires_local_profile: true,
            local_only: true,
            test_only: true,
        };
        reset_plan.validate().unwrap();

        let volume_registry = LocalStackVolumeRegistry {
            registry_id: "volume_set:local_default".to_owned(),
            volumes: vec![LocalStackVolumeRef {
                volume_id: "volume:embedded_state".to_owned(),
                path_ref: "local-state://embedded_state".to_owned(),
                resettable: true,
                local_test_state_marker: Some(".overrid-local-test-state".to_owned()),
            }],
            local_only: true,
            test_only: true,
        };
        volume_registry.validate().unwrap();

        let mut unmarked_volume = volume_registry.clone();
        unmarked_volume.volumes[0].local_test_state_marker = None;
        assert!(matches!(
            unmarked_volume.validate(),
            Err(LocalStackContractError::ResetTargetMissingMarker)
        ));

        let mut unmarked_reset = reset_plan.clone();
        unmarked_reset.operations[0].marker_ref = None;
        assert!(matches!(
            unmarked_reset.validate(),
            Err(LocalStackContractError::ResetTargetMissingMarker)
        ));

        let mut unsafe_seed = LocalStackSeedManifest {
            manifest_id: "seed_manifest:local_default".to_owned(),
            fixture_version: "fixture.local_stack.phase2.v1".to_owned(),
            deterministic_seed: "local_stack_phase2_seed_0001".to_owned(),
            fixture_refs: vec!["fixture:phase2_default_local".to_owned()],
            local_only: true,
            test_only: false,
        };
        assert!(matches!(
            unsafe_seed.validate(),
            Err(LocalStackContractError::SeedManifestNotTestOnly)
        ));
        unsafe_seed.test_only = true;
        unsafe_seed.validate().unwrap();

        let mut secret = LocalSecretRecordContract {
            secret_ref: "secret://local_stack/fixture_key_ref".to_owned(),
            secret_kind: "fixture_key".to_owned(),
            redaction_ref: "redaction:fixture_key_ref".to_owned(),
            raw_secret_present: false,
            local_only: true,
            test_only: true,
        };
        secret.validate().unwrap();
        secret.raw_secret_present = true;
        assert!(matches!(
            secret.validate(),
            Err(LocalStackContractError::RawSecretInDiagnostic)
        ));

        let mut diagnostic = LocalDiagnosticEventContract {
            event_type: "local_stack.service_ready".to_owned(),
            trace_id: "trace_local_stack_phase2_ready".to_owned(),
            service_id: "service:api".to_owned(),
            reason_code: "local_stack.service_ready".to_owned(),
            redaction_summary: "secret_free".to_owned(),
            contains_raw_secret: false,
            local_only: true,
            test_only: true,
        };
        diagnostic.validate().unwrap();
        diagnostic.contains_raw_secret = true;
        assert!(matches!(
            diagnostic.validate(),
            Err(LocalStackContractError::RawSecretInDiagnostic)
        ));
    }

    #[test]
    fn local_stack_health_states_cover_phase2_lifecycle() {
        let states = [
            LocalStackHealthState::NotStarted,
            LocalStackHealthState::Starting,
            LocalStackHealthState::Ready,
            LocalStackHealthState::Degraded,
            LocalStackHealthState::Failed,
            LocalStackHealthState::Resetting,
            LocalStackHealthState::Seeding,
            LocalStackHealthState::RunningTests,
        ];
        assert_eq!(
            states.map(LocalStackHealthState::as_str),
            [
                "not_started",
                "starting",
                "ready",
                "degraded",
                "failed",
                "resetting",
                "seeding",
                "running_tests",
            ]
        );
        assert!(LocalStackHealthState::Ready.is_terminal());
        assert!(!LocalStackHealthState::Starting.is_terminal());

        let mut snapshot = LocalStackHealthSnapshot {
            schema_version: ensure_supported_local_development_stack_schema_version(
                SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION,
            )
            .unwrap(),
            snapshot_id: "health_snapshot:local_ready".to_owned(),
            profile_id: "profile:local_default".to_owned(),
            state: LocalStackHealthState::Ready,
            service_health: vec![LocalStackServiceHealth {
                service_id: "service:api".to_owned(),
                state: LocalStackHealthState::Ready,
                last_error: None,
                health_check_ref: "health:api".to_owned(),
            }],
            redaction_summary: "secret_free".to_owned(),
            local_only: true,
            test_only: true,
        };
        snapshot.validate().unwrap();
        snapshot.service_health[0].last_error = Some("token=leaked".to_owned());
        assert!(matches!(
            snapshot.validate(),
            Err(LocalStackContractError::RawSecretInDiagnostic)
        ));
    }

    #[test]
    fn validates_phase2_fixture_manifest_contracts() {
        let manifest = FixtureManifest::new(
            "fixture_phase0_smoke",
            "tenant:local:alpha",
            "actor:local:builder",
            "seed_phase0_smoke_0001",
            vec![harness_fixture_key()],
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();

        assert!(manifest.test_only);
        assert_eq!(manifest.schema_version.raw(), "integration-harness.v0.1");
        assert_eq!(manifest.keys[0].key_id, "fixture_local_builder");
        assert!(manifest
            .resource_card_refs
            .contains(&"resource:local:synthetic_cpu".to_owned()));
        assert!(manifest
            .workload_refs
            .contains(&"workload:local:no_op".to_owned()));
        assert!(manifest
            .package_refs
            .contains(&"package:local:no_op".to_owned()));
        assert!(manifest
            .local_ledger_account_refs
            .contains(&"ledger:local:oru_account".to_owned()));
        assert!(manifest
            .policy_context_refs
            .contains(&"policy:local:allow_smoke".to_owned()));
    }

    #[test]
    fn rejects_invalid_phase2_fixture_manifests() {
        assert!(matches!(
            FixtureManifest::new(
                "fixture_missing_seed",
                "tenant:local:alpha",
                "actor:local:builder",
                "",
                vec![harness_fixture_key()],
                SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
            ),
            Err(HarnessContractError::MissingRequiredField(
                "deterministic seed"
            ))
        ));

        assert!(matches!(
            FixtureManifest::new(
                "fixture_missing_tenant",
                "",
                "actor:local:builder",
                "seed_phase0_smoke_0001",
                vec![harness_fixture_key()],
                SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
            ),
            Err(HarnessContractError::MissingRequiredField("tenant ref"))
        ));

        let mut non_test_key = harness_fixture_key();
        non_test_key.test_only = false;
        assert!(matches!(
            FixtureManifest::new(
                "fixture_non_test_key",
                "tenant:local:alpha",
                "actor:local:builder",
                "seed_phase0_smoke_0001",
                vec![non_test_key],
                SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
            ),
            Err(HarnessContractError::FixtureKeyNotTestOnly)
        ));
    }

    #[test]
    fn rejects_invalid_phase2_fixture_ref_prefixes() {
        let mut manifest = FixtureManifest::new(
            "fixture_bad_ref",
            "tenant:local:alpha",
            "actor:local:builder",
            "seed_phase0_smoke_0001",
            vec![harness_fixture_key()],
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();
        manifest.workload_refs = vec!["package:local:not_a_workload".to_owned()];

        assert!(matches!(
            manifest.validate(),
            Err(HarnessContractError::InvalidHarnessRef {
                field: "workload ref",
                expected_prefix: "workload:",
                ..
            })
        ));
    }

    #[test]
    fn validates_phase2_scenario_manifest_contracts() {
        let manifest = ScenarioManifest::new(
            "scenario_phase0_smoke",
            0,
            vec![harness_step()],
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();

        assert_eq!(manifest.master_phase, 0);
        assert_eq!(manifest.steps[0].action_kind.as_str(), "cli");
        assert!(manifest
            .required_services
            .contains(&"service:local_stack".to_owned()));
        assert!(manifest
            .cleanup_rules
            .contains(&"collect_artifacts_then_reset".to_owned()));
    }

    #[test]
    fn rejects_invalid_phase2_scenario_variants() {
        assert!(matches!(
            ScenarioActionKind::parse("direct_storage"),
            Err(HarnessContractError::InvalidActionKind(kind)) if kind == "direct_storage"
        ));

        assert!(matches!(
            ScenarioManifest::new(
                "scenario_phase14",
                14,
                vec![harness_step()],
                SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
            ),
            Err(HarnessContractError::UnsupportedPhase(14))
        ));

        assert!(matches!(
            ScenarioStep::new(
                "step_missing_assertion",
                ScenarioActionKind::Cli,
                Vec::new(),
                30_000,
            ),
            Err(HarnessContractError::MissingAssertion)
        ));

        assert!(matches!(
            ScenarioStep::new(
                "step_unsafe_timeout",
                ScenarioActionKind::Cli,
                vec!["assertion:timeout:bounded".to_owned()],
                900_000,
            ),
            Err(HarnessContractError::UnsafeTimeoutMs(900_000))
        ));
    }

    #[test]
    fn validates_phase2_run_and_assertion_records() {
        let run = TestRunRecord::terminal(
            "run_phase0_smoke_passed",
            HarnessRunStatus::Passed,
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();
        let assertion =
            AssertionResult::passed("assertion_phase0_trace_order", "scenario_phase0_smoke");

        assert_eq!(run.status.as_str(), "passed");
        assert!(run.status.is_terminal());
        assert_eq!(run.reason_code, "run.passed");
        assert_eq!(run.artifact_policy.as_str(), "phase_gate_evidence");
        assertion.validate().unwrap();
    }

    #[test]
    fn rejects_non_terminal_or_incomplete_run_records() {
        assert!(matches!(
            TestRunRecord::terminal(
                "run_still_planned",
                HarnessRunStatus::Planned,
                SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
            ),
            Err(HarnessContractError::MissingRunStatus)
        ));
    }

    #[test]
    fn validates_phase2_golden_trace_modes() {
        let exact = GoldenTrace::new(
            "golden_trace_phase0_noop",
            GoldenTraceMode::Exact,
            vec![
                "event_command_accepted".to_owned(),
                "event_audit_written".to_owned(),
            ],
            vec![(
                "event_command_accepted".to_owned(),
                "event_audit_written".to_owned(),
            )],
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();

        let dag = GoldenTrace::new(
            "golden_trace_phase3_execution",
            GoldenTraceMode::Dag,
            vec![
                "event_queue_pending".to_owned(),
                "event_lease_created".to_owned(),
            ],
            vec![(
                "event_queue_pending".to_owned(),
                "event_lease_created".to_owned(),
            )],
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();

        assert_eq!(exact.mode.as_str(), "exact");
        assert_eq!(dag.mode.as_str(), "dag");
        assert!(dag
            .stable_reason_codes
            .contains(&"command.accepted".to_owned()));
    }

    #[test]
    fn detects_observed_golden_trace_drift() {
        let exact = GoldenTrace::new(
            "golden_trace_phase0_noop",
            GoldenTraceMode::Exact,
            vec![
                "event_command_accepted".to_owned(),
                "event_audit_written".to_owned(),
            ],
            vec![(
                "event_command_accepted".to_owned(),
                "event_audit_written".to_owned(),
            )],
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();

        exact
            .assert_observed_nodes(&[
                "event_command_accepted".to_owned(),
                "event_audit_written".to_owned(),
            ])
            .unwrap();
        assert!(matches!(
            exact.assert_observed_nodes(&["event_command_accepted".to_owned()]),
            Err(HarnessContractError::GoldenTraceMissingEvent)
        ));
        assert!(matches!(
            exact.assert_observed_nodes(&[
                "event_command_accepted".to_owned(),
                "event_unexpected_state_change".to_owned(),
                "event_audit_written".to_owned(),
            ]),
            Err(HarnessContractError::GoldenTraceExtraStateEvent)
        ));
        assert!(matches!(
            exact.assert_observed_nodes(&[
                "event_audit_written".to_owned(),
                "event_command_accepted".to_owned(),
            ]),
            Err(HarnessContractError::GoldenTraceExactOrderMismatch)
        ));

        let forbidden = GoldenTrace::new(
            "golden_trace_forbidden_transition",
            GoldenTraceMode::Exact,
            vec![
                "event_audit_written".to_owned(),
                "event_command_accepted".to_owned(),
            ],
            vec![(
                "event_audit_written".to_owned(),
                "event_command_accepted".to_owned(),
            )],
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .map(|mut trace| {
            trace.forbidden_transitions = vec![(
                "event_audit_written".to_owned(),
                "event_command_accepted".to_owned(),
            )];
            trace
        })
        .unwrap();
        assert!(matches!(
            forbidden.assert_observed_nodes(&[
                "event_audit_written".to_owned(),
                "event_command_accepted".to_owned(),
            ]),
            Err(HarnessContractError::GoldenTraceForbiddenTransition)
        ));
    }

    #[test]
    fn rejects_dag_golden_trace_without_edges() {
        assert!(matches!(
            GoldenTrace::new(
                "golden_trace_missing_edge",
                GoldenTraceMode::Dag,
                vec![
                    "event_queue_pending".to_owned(),
                    "event_lease_created".to_owned()
                ],
                Vec::new(),
                SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
            ),
            Err(HarnessContractError::GoldenTraceMissingEdge)
        ));
    }

    #[test]
    fn validates_phase2_artifact_bundle_contracts() {
        let bundle = ArtifactBundle::new(
            "artifact_bundle_phase0_smoke",
            "run_phase0_smoke_passed",
            false,
            false,
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();

        assert_eq!(bundle.retention_class.as_str(), "phase_gate_evidence");
        assert_eq!(bundle.redaction_policy, "secret_free_refs_only");
        assert!(bundle
            .reproduction_command
            .contains("overrid test scenario scenario_phase0_smoke"));
        assert!(bundle
            .redacted_log_refs
            .contains(&"artifact:logs:phase0_smoke_redacted".to_owned()));
        assert!(bundle
            .overwatch_export_refs
            .contains(&"artifact:overwatch:phase0_smoke".to_owned()));
        assert!(bundle
            .cli_output_refs
            .contains(&"artifact:cli_output:phase0_smoke".to_owned()));
        assert!(bundle
            .api_payload_envelope_refs
            .contains(&"artifact:api_envelope:phase0_smoke".to_owned()));
        assert!(bundle
            .stack_health_refs
            .contains(&"artifact:health:local_stack".to_owned()));
        assert!(bundle
            .fixture_version_refs
            .contains(&"artifact:fixture_version:phase0_smoke".to_owned()));
        assert!(bundle
            .schema_version_refs
            .contains(&"artifact:schema_version:integration_harness_v0_1".to_owned()));
        assert!(bundle
            .assertion_diff_refs
            .contains(&"artifact:assertion_diff:phase0_smoke".to_owned()));
        assert_eq!(
            bundle.redaction_report_ref,
            "artifact:redaction_report:phase0_smoke"
        );
        assert!(bundle.redaction_report.scanner_passed);
        assert!(!bundle.flake_metadata.is_nondeterministic());
        assert_eq!(
            bundle.retention_policy.retention_class,
            ArtifactRetentionClass::PhaseGateEvidence
        );
        assert!(!bundle.contains_raw_secret);
        assert!(!bundle.contains_private_key);
        assert!(!bundle.contains_token);
        assert!(!bundle.contains_signature);
        assert!(!bundle.contains_encrypted_rag_content);
        assert!(!bundle.contains_private_payload);
        assert!(!bundle.contains_fixture_key_material);
    }

    #[test]
    fn rejects_artifact_bundles_with_raw_or_private_material() {
        assert!(matches!(
            ArtifactBundle::new(
                "artifact_bundle_raw_secret",
                "run_raw_secret",
                true,
                false,
                SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
            ),
            Err(HarnessContractError::RawSecretInArtifact)
        ));

        assert!(matches!(
            ArtifactBundle::new(
                "artifact_bundle_private_payload",
                "run_private_payload",
                false,
                true,
                SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
            ),
            Err(HarnessContractError::RawSecretInArtifact)
        ));

        let mut signature_bundle = ArtifactBundle::new(
            "artifact_bundle_signature",
            "run_signature",
            false,
            false,
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();
        signature_bundle.contains_signature = true;
        assert!(matches!(
            signature_bundle.validate(),
            Err(HarnessContractError::RawSecretInArtifact)
        ));

        let mut missing_ref_bundle = ArtifactBundle::new(
            "artifact_bundle_missing_refs",
            "run_missing_refs",
            false,
            false,
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();
        missing_ref_bundle.overwatch_export_refs.clear();
        assert!(matches!(
            missing_ref_bundle.validate(),
            Err(HarnessContractError::MissingRequiredField(
                "Overwatch export refs"
            ))
        ));
    }

    #[test]
    fn validates_phase8_flake_redaction_and_retention_contracts() {
        let mut flake_run = TestRunRecord::terminal(
            "run_phase8_flake_detection",
            HarnessRunStatus::Failed,
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )
        .unwrap();
        flake_run.flake_metadata = FlakeMetadata::unstable_event_ordering(vec![
            "assertion:phase8:unstable_event_ordering".to_owned(),
        ]);
        flake_run.validate().unwrap();
        assert!(flake_run.flake_metadata.is_nondeterministic());

        let mut passed_run = flake_run.clone();
        passed_run.status = HarnessRunStatus::Passed;
        assert!(matches!(
            passed_run.validate(),
            Err(HarnessContractError::RawSecretInArtifact)
        ));

        let retention = ArtifactRetentionPolicy::for_class(ArtifactRetentionClass::FailureEvidence);
        assert_eq!(retention.minimum_retention_days, 30);
        assert_eq!(retention.prune_after_days, 90);
        retention.validate().unwrap();

        let redaction = RedactionScanReport::passed(vec![
            "headers.authorization".to_owned(),
            "payload.private".to_owned(),
        ]);
        redaction.validate().unwrap();

        assert!(matches!(
            RedactionScanReport::failed(vec!["token=".to_owned()]).validate(),
            Err(HarnessContractError::RawSecretInArtifact)
        ));
    }

    #[test]
    fn validates_local_fixture_profile_and_credential() {
        let profile = local_profile();
        let credential = fixture_credential();
        profile.validate().unwrap();
        credential.validate_for_profile(&profile).unwrap();
    }

    #[test]
    fn rejects_missing_tenant_before_profile_use() {
        let mut profile = local_profile();
        profile.tenant_id.clear();
        assert!(matches!(
            profile.validate(),
            Err(ProfileValidationError::MissingRequiredField("tenant id"))
        ));
    }

    #[test]
    fn rejects_fixture_credential_for_seed_profile_without_test_harness() {
        let mut profile = local_profile();
        profile.environment = EnvironmentClass::Seed;
        profile.fixture_allowance = FixtureAllowance::Denied;
        assert!(matches!(
            profile.validate(),
            Err(ProfileValidationError::FixtureCredentialNotAllowed { .. })
        ));
    }

    #[test]
    fn rejects_raw_key_material_in_credential_reference() {
        let profile = local_profile();
        let mut credential = fixture_credential();
        credential.reference_id = "-----BEGIN PRIVATE KEY-----".to_owned();
        assert!(matches!(
            credential.validate_for_profile(&profile),
            Err(ProfileValidationError::RawKeyMaterialInReference)
        ));
    }

    #[test]
    fn rejects_revoked_and_expired_credential_references() {
        let profile = local_profile();
        let mut revoked = fixture_credential();
        revoked.revoked = true;
        assert!(matches!(
            revoked.validate_for_profile(&profile),
            Err(ProfileValidationError::CredentialRevoked)
        ));

        let mut expired = fixture_credential();
        expired.expired = true;
        assert!(matches!(
            expired.validate_for_profile(&profile),
            Err(ProfileValidationError::CredentialExpired)
        ));
    }

    #[test]
    fn signer_handoff_returns_refs_without_key_material() {
        let profile = local_profile();
        let credential = fixture_credential();
        let handoff = SignerHandoff::new(&profile, &credential, "tenant create").unwrap();
        assert_eq!(handoff.credential_reference_id, credential.reference_id);
        assert!(!handoff.exposes_key_material);
        assert!(handoff.signature_ref.starts_with("sigref:local-dev:key-1:"));
        assert!(!handoff.signature_ref.contains("PRIVATE KEY"));
    }

    #[test]
    fn signed_bootstrap_envelope_carries_phase5_refs() {
        let envelope = SignedCommandEnvelope::new(
            BootstrapCommandFamily::Tenant,
            "tenant create",
            "tenant_local",
            "actor_local",
            "tenant_local",
            "tenant_bootstrap_command",
            Some("absent".to_owned()),
            Some("bootstrap tenant".to_owned()),
            "idem_local_tenant_create_tenant_local",
            "trace_cli_local",
            "sigref:local-dev:key-1:tenant_create",
        )
        .unwrap();

        assert_eq!(envelope.family.as_str(), "tenant");
        assert_eq!(
            envelope.family.phase_gate(),
            "phase_1_control_plane_bootstrap"
        );
        assert_eq!(
            envelope.idempotency.key,
            "idem_local_tenant_create_tenant_local"
        );
        assert_eq!(envelope.trace_context.trace_id, "trace_cli_local");
        assert!(!envelope.exposes_key_material);
    }

    #[test]
    fn canonical_idempotency_fingerprint_carries_phase6_inputs() {
        let fingerprint = CanonicalIdempotencyFingerprint::new(
            EnvironmentClass::Local,
            "fp_local_dev",
            "tenant_local",
            "actor_local",
            "tenant create",
            "tenant_local",
            "hash_1234",
            Some("absent".to_owned()),
            Some("bootstrap tenant".to_owned()),
            SUPPORTED_SCHEMA_VERSION,
            "local:fp_local_dev:tenant:create",
        )
        .unwrap();

        assert_eq!(fingerprint.endpoint_identity, "fp_local_dev");
        assert_eq!(
            fingerprint.expected_current_state.as_deref(),
            Some("absent")
        );
        assert_eq!(fingerprint.reason.as_deref(), Some("bootstrap tenant"));
        assert_eq!(fingerprint.schema_version.raw(), SUPPORTED_SCHEMA_VERSION);
        assert!(fingerprint.idempotency_key().starts_with("idem_"));
        assert!(fingerprint
            .new_operation_idempotency_key("trace_cli_new")
            .starts_with("idem_new_"));
    }

    #[test]
    fn retry_timeout_policy_is_bounded_and_classified() {
        let policy = RetryTimeoutPolicy::bounded(9, 900_000);

        assert_eq!(policy.max_retries, 5);
        assert_eq!(policy.timeout_ms, 600_000);
        assert!(policy.bounded);
        assert!(policy.retryable_classes.contains(&RetryClass::SafeRetry));
        assert!(policy
            .non_retryable_reason_families
            .contains(&"phase".to_owned()));
    }

    #[test]
    fn error_decode_record_hides_internal_errors() {
        let record = ErrorDecodeRecord::new(
            "platform",
            "transport_unavailable",
            RetryClass::SafeRetry,
            ExitCodeClass::Transport,
            "retry with the same idempotency key",
        );

        assert_eq!(record.retry_class, RetryClass::SafeRetry);
        assert_eq!(record.exit_class, ExitCodeClass::Transport);
        assert!(!record.raw_internal_error_exposed);
    }

    #[test]
    fn local_idempotency_cache_record_is_refs_only() {
        let record = LocalIdempotencyCacheRecord::new(
            "local-dev",
            EnvironmentClass::Local,
            "fingerprint_local",
            "idem_fingerprint_local",
        );

        assert_eq!(record.cache_scope, "profile_environment");
        assert!(record.owner_only);
        assert!(!record.contains_private_payload);
        assert!(record.inspectable);
        assert!(record.resettable);
    }

    #[test]
    fn node_status_record_is_profile_scoped_and_refs_only() {
        let record = NodeStatusRecord::new(
            "node_live",
            NodeState::Live,
            "local-dev",
            "fixture://local-dev/key-1",
        );

        assert_eq!(record.state.as_str(), "live");
        assert!(record.state.accepts_work());
        assert!(record.credential_checked);
        assert_eq!(record.registered_via, "sdk_overgate_contract");
        assert!(!record.direct_node_access);
        assert!(record
            .capability_refs
            .contains(&"overcell:node:node_live:capabilities".to_owned()));
        assert_eq!(NodeState::Draining.as_str(), "draining");
        assert!(!NodeState::Disabled.accepts_work());
    }

    #[test]
    fn workload_execution_contracts_are_refs_only_and_terminal_aware() {
        let timeline = ExecutionTimeline::new(
            "workload_local",
            vec![
                WorkloadExecutionState::Scheduled,
                WorkloadExecutionState::Leased,
                WorkloadExecutionState::Running,
                WorkloadExecutionState::Succeeded,
            ],
            "trace_cli_local",
        );
        let logs = ExecutionLogBundle::new("workload_local", "trace_cli_local");
        let result = ExecutionResultRef::new("workload_local", "trace_cli_local");
        let polling = PollingPlan::bounded(true, true, 900_000, 10);

        assert!(WorkloadExecutionState::Succeeded.is_terminal());
        assert!(WorkloadExecutionState::TimedOut.is_terminal());
        assert!(!WorkloadExecutionState::Running.is_terminal());
        assert_eq!(timeline.states[0].as_str(), "scheduled");
        assert_eq!(timeline.states[3].as_str(), "succeeded");
        assert_eq!(timeline.diagnostic_events.len(), 6);
        assert!(timeline
            .owning_service_refs
            .contains(&"overwatch:trace".to_owned()));
        assert!(timeline
            .owning_service_refs
            .contains(&"overcell:node-heartbeat".to_owned()));
        assert!(timeline
            .owning_service_refs
            .contains(&"overpack:package".to_owned()));
        assert!(timeline
            .owning_service_refs
            .contains(&"overstore:result-state-ref".to_owned()));
        assert!(timeline
            .diagnostic_events
            .iter()
            .any(|event| event.reason_code == "result.ref.available"));
        let failed_timeline = ExecutionTimeline::new(
            "workload_failed",
            vec![
                WorkloadExecutionState::Scheduled,
                WorkloadExecutionState::Leased,
                WorkloadExecutionState::Running,
                WorkloadExecutionState::Failed,
            ],
            "trace_cli_local",
        );
        assert!(failed_timeline
            .diagnostic_events
            .iter()
            .any(|event| event.reason_code == "result.failed"));
        assert!(!timeline.direct_node_access);
        assert_eq!(logs.redaction_policy, "secret_free_refs_only");
        assert!(logs.bounded_streaming);
        assert!(!logs.contains_private_payload);
        assert!(!logs.direct_node_path_exposed);
        assert!(result.result_ref.starts_with("overstore:result:"));
        assert!(!result.contains_private_payload);
        assert!(!result.direct_object_store_path_exposed);
        assert_eq!(polling.timeout_ms, 600_000);
        assert_eq!(polling.poll_interval_ms, 100);
        assert!(polling.event_stream_preferred);
        assert!(polling.fallback_polling);
    }

    #[test]
    fn manifest_and_workload_phase5_refs_are_pending_only() {
        let manifest = ManifestBootstrapRef::new("manifest_local", "workload");
        assert_eq!(manifest.submitted_via, "sdk_overgate_contract");
        assert_eq!(manifest.immutable_ref, "manifest:manifest_local:immutable");

        let workload = SyntheticWorkloadPendingState::pending("workload_local", "synthetic");
        assert_eq!(workload.queue_state, "pending");
        assert!(!workload.execution_implied);
        assert_eq!(
            workload.timeline_refs,
            vec!["timeline:workload_local:pending".to_owned()]
        );
    }

    #[test]
    fn phase9_product_workflow_recipe_keeps_products_on_sdk_overgate_refs() {
        let docdex = ProductWorkflowRecipe::new(
            ProductKind::Docdex,
            "workload_docdex_index",
            "docdex_encrypted_index",
        );
        assert_eq!(docdex.product.as_str(), "docdex");
        assert_eq!(docdex.submitted_via, "sdk_overgate_contract");
        assert!(docdex.sdk_overgate_only);
        assert!(docdex.authorized_refs_only);
        assert!(docdex.secret_free_json_output);
        assert!(docdex
            .required_refs
            .contains(&"encrypted_index_ref".to_owned()));
        assert!(!docdex.direct_internal_api_access);
        assert!(!docdex.direct_storage_access);
        assert!(!docdex.raw_http_required);

        let mcoda = ProductWorkflowRecipe::new(
            ProductKind::Mcoda,
            "workload_mcoda_agent",
            "mcoda_agent_workload",
        );
        assert!(mcoda.dynamic_model_resource_selection);
        assert!(mcoda
            .required_refs
            .contains(&"dynamic_model_metadata_ref".to_owned()));
        assert!(!mcoda.hardcoded_model_or_provider);
        assert!(!mcoda.hardcoded_node_assumption);
        assert!(!mcoda.paid_service_assumption);

        let codali = ProductWorkflowRecipe::new(
            ProductKind::Codali,
            "workload_codali_package",
            "codali_code_agent_package",
        );
        assert!(codali.required_refs.contains(&"artifact_refs".to_owned()));
        assert!(codali
            .expected_failure_modes
            .contains(&"policy.resource_denied".to_owned()));
        assert!(codali
            .safe_retry_patterns
            .contains(&"retry_failed_phase_after_repair_ref".to_owned()));
    }

    #[test]
    fn phase9_ci_automation_profile_requires_explicit_short_lived_refs() {
        let profile = CiAutomationProfile::new(
            EnvironmentClass::Ci,
            "ci_reference",
            "ci://overrid/service-account/short-lived",
        );

        assert_eq!(profile.profile_kind, "ci");
        assert_eq!(profile.environment_class.as_str(), "ci");
        assert_eq!(profile.submitted_via, "sdk_overgate_contract");
        assert!(profile.short_lived_service_account_required);
        assert!(profile.requires_non_interactive_confirmation);
        assert!(profile.json_output_stable);
        assert!(profile.secret_free_output);
        assert!(profile.branch_on_exit_class);
        assert!(!profile.ambient_persistent_keychain_allowed);
        assert!(profile
            .allowed_credential_ref_kinds
            .contains(&"mounted_secret_ref".to_owned()));
    }

    #[test]
    fn phase10_release_readiness_report_keeps_handoff_gated_and_secret_free() {
        let report = CliReleaseReadinessReport::new();

        assert!(report.release_ready);
        assert!(report.sdk_overgate_only);
        assert!(!report.direct_private_shortcut);
        assert!(!report.high_risk_phase7_phase13_enabled);
        assert!(report
            .contract_snapshot_suite
            .contains(&"backward_compatible_json".to_owned()));
        assert!(report
            .exit_code_classes
            .contains(&ExitCodeClass::Phase.as_str().to_owned()));
        assert!(report
            .reason_code_families
            .contains(&"not_available_in_phase".to_owned()));
        assert!(report
            .integration_validation_matrix
            .contains(&"real_private_job".to_owned()));
        assert!(report
            .automation_compatibility_matrix
            .contains(&"ci_non_interactive_credentials".to_owned()));
        assert!(!report.security_review_report.raw_keys_exposed);
        assert!(!report.security_review_report.tokens_exposed);
        assert!(!report.security_review_report.signatures_exposed);
        assert!(!report.security_review_report.private_payloads_exposed);
        assert!(report.phase_availability_matrix.iter().any(|record| {
            record.command == "federation/public-interest/purpose-tag"
                && record.phase_gate == "phase_10"
                && record.availability == "denied"
                && record.stable_reason_code == "not_available_in_phase"
                && record.hidden_in_normal_help
                && !record.direct_private_shortcut
        }));
        assert!(report.phase_availability_matrix.iter().any(|record| {
            record.command == "governance/incident/compliance/migration"
                && record.availability == "denied"
                && record.stable_reason_code == "not_available_in_phase"
                && record.hidden_in_normal_help
                && !record.direct_private_shortcut
        }));
    }

    #[test]
    fn bootstrap_acceptance_record_tracks_audit_refs() {
        let record = BootstrapAcceptanceRecord::new(
            "workload submit",
            "accepted:workload_local",
            "pending",
            vec!["audit_cli_bootstrap_workload_submit".to_owned()],
        );

        assert_eq!(record.phase_gate, "phase_1_control_plane_bootstrap");
        assert_eq!(record.pending_state, "pending");
        assert_eq!(record.audit_refs.len(), 1);
    }

    #[test]
    fn command_lifecycle_tracks_terminal_states() {
        let lifecycle = CommandLifecycle::new(vec![
            CommandLifecycleState::Parsed,
            CommandLifecycleState::ProfileLoaded,
            CommandLifecycleState::CredentialReady,
            CommandLifecycleState::PayloadValidated,
            CommandLifecycleState::Completed,
        ]);

        assert!(lifecycle.has_terminal_state());
        assert_eq!(
            lifecycle.terminal_state(),
            Some(CommandLifecycleState::Completed)
        );
        assert_eq!(CommandLifecycleState::Denied.as_str(), "denied");
        assert_eq!(ALL_COMMAND_LIFECYCLE_STATES.len(), 11);
    }

    #[test]
    fn exit_code_registry_locks_phase4_numeric_classes() {
        let pairs = EXIT_CODE_REGISTRY
            .iter()
            .map(|class| (class.as_str(), class.code()))
            .collect::<Vec<_>>();

        assert_eq!(
            pairs,
            vec![
                ("success", 0),
                ("usage", 2),
                ("config", 3),
                ("credential", 4),
                ("schema", 5),
                ("policy", 6),
                ("phase", 7),
                ("idempotency", 8),
                ("transport", 9),
                ("timeout", 10),
                ("platform", 11),
                ("local_io", 12),
            ]
        );
        assert_eq!(exit_code_class_for_code(7), Some(ExitCodeClass::Phase));
        assert_eq!(exit_code_class_for_code(1), None);
    }

    #[test]
    fn output_envelope_carries_phase4_fields() {
        let success = OutputEnvelope::success(SUPPORTED_SCHEMA_VERSION).unwrap();
        assert_eq!(success.exit_class, ExitCodeClass::Success);
        assert_eq!(success.retry_class, RetryClass::NotRetryable);
        assert_eq!(
            success.lifecycle.terminal_state(),
            Some(CommandLifecycleState::Completed)
        );

        let failure = OutputEnvelope::failure_with_exit(
            SUPPORTED_SCHEMA_VERSION,
            "not_available_in_phase",
            ExitCodeClass::Phase,
            RetryClass::NotRetryable,
        )
        .unwrap();
        assert_eq!(failure.exit_code, 7);
        assert_eq!(failure.exit_class.as_str(), "phase");
        assert_eq!(
            failure.reason_code.as_deref(),
            Some("not_available_in_phase")
        );
    }

    #[test]
    fn diagnostic_bundle_is_secret_free_and_refs_only() {
        let bundle = DiagnosticBundle::new("doctor", SUPPORTED_SCHEMA_VERSION).unwrap();
        assert_eq!(bundle.redaction_policy, "secret_free_refs_only");
        assert_eq!(
            bundle.schema_versions,
            vec![SUPPORTED_SCHEMA_VERSION.to_owned()]
        );
        assert!(bundle.trace_ids.is_empty());
        assert!(bundle.reason_codes.is_empty());
        assert!(bundle
            .dependency_status
            .contains(&"local_contracts_available".to_owned()));
    }

    #[test]
    fn capability_snapshot_fails_closed_when_route_unavailable() {
        let capability = CapabilitySnapshot::local_phase_gate("policy dry-run", "phase_4");
        assert!(!capability.available);
        assert!(capability.fail_closed);
        assert_eq!(capability.phase_gate, "phase_4");
        assert_eq!(
            capability.schema_versions,
            vec![SUPPORTED_SCHEMA_VERSION.to_owned()]
        );
    }

    #[test]
    fn owner_only_file_mode_rejects_group_or_world_bits() {
        validate_owner_only_file_mode(0o600).unwrap();
        validate_owner_only_file_mode(0o700).unwrap();
        assert!(matches!(
            validate_owner_only_file_mode(0o644),
            Err(ProfileValidationError::UnsafeFilePermissions(0o644))
        ));
    }
}
