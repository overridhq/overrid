use std::fmt;

use crate::{
    check_sdk_compatibility, negotiate_sdk_capability, OvergateEndpoint, OverridErrorRecord,
    SdkCapabilityDecision, SdkCommandClass, SdkError, SdkIdempotencyEntry, SdkOptionalHelper,
    SdkRequestContextRecord, SdkServiceCapabilityProfile, SDK_CURRENT_STABLE_MAJOR,
    SDK_LANGUAGE_BINDING, SDK_NAME, SDK_VERSION,
};
use overrid_contracts::SchemaVersion;

pub const SDK_PHASE7_CAPABILITY_PROFILE: &str =
    "phase7-usage-receipt-oru-seal-ledger-dispute-readers";
pub const SDK_PHASE7_USAGE_RECEIPT_ROUTE: &str = "/v1/accounting/usage-receipts";
pub const SDK_PHASE7_USAGE_ROLLUP_ROUTE: &str = "/v1/accounting/usage-rollups";
pub const SDK_PHASE7_ORU_CHARGE_PREVIEW_ROUTE: &str = "/v1/accounting/oru/charge-previews";
pub const SDK_PHASE7_SEAL_LEDGER_REF_ROUTE: &str = "/v1/accounting/seal-ledger/refs";
pub const SDK_PHASE7_DISPUTE_REFERENCE_ROUTE: &str = "/v1/accounting/dispute-refs";
pub const SDK_PHASE7_ACCOUNTING_AUTHORITY_OWNERS: &[&str] = &[
    "Overmeter",
    "ORU Account Service",
    "Seal Ledger",
    "Overbill",
    "Overgrant",
    "Overasset",
    "Overclaim",
    "Provider Payout Service",
    "Overwatch",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdkAccountingReadKind {
    UsageReceipt,
    UsageRollup,
    OruChargePreview,
    SealLedgerReference,
    DisputeReference,
    ReceiptReference,
    HoldReference,
    RefundCorrectionReference,
    GrantReference,
    AssetReference,
}

impl SdkAccountingReadKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsageReceipt => "usage_receipt",
            Self::UsageRollup => "usage_rollup",
            Self::OruChargePreview => "oru_charge_preview",
            Self::SealLedgerReference => "seal_ledger_reference",
            Self::DisputeReference => "dispute_reference",
            Self::ReceiptReference => "receipt_reference",
            Self::HoldReference => "hold_reference",
            Self::RefundCorrectionReference => "refund_correction_reference",
            Self::GrantReference => "grant_reference",
            Self::AssetReference => "asset_reference",
        }
    }

    pub fn route(self) -> &'static str {
        match self {
            Self::UsageReceipt | Self::ReceiptReference => SDK_PHASE7_USAGE_RECEIPT_ROUTE,
            Self::UsageRollup => SDK_PHASE7_USAGE_ROLLUP_ROUTE,
            Self::OruChargePreview => SDK_PHASE7_ORU_CHARGE_PREVIEW_ROUTE,
            Self::SealLedgerReference => SDK_PHASE7_SEAL_LEDGER_REF_ROUTE,
            Self::DisputeReference
            | Self::HoldReference
            | Self::RefundCorrectionReference
            | Self::GrantReference
            | Self::AssetReference => SDK_PHASE7_DISPUTE_REFERENCE_ROUTE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkAccountingRequestMetadata {
    pub caller_app_id: Option<String>,
    pub actor_id: String,
    pub tenant_id: String,
    pub trace_id: String,
    pub idempotency_key: String,
    pub schema_version: SchemaVersion,
}

impl SdkAccountingRequestMetadata {
    pub fn new(
        caller_app_id: Option<String>,
        actor_id: impl Into<String>,
        tenant_id: impl Into<String>,
        trace_id: impl Into<String>,
        idempotency_key: impl Into<String>,
        schema_version: &str,
    ) -> Result<Self, SdkPhase7Error> {
        let actor_id = actor_id.into();
        let tenant_id = tenant_id.into();
        let trace_id = trace_id.into();
        let idempotency_key = idempotency_key.into();
        require_phase7_non_empty(&actor_id, "actor id")?;
        require_phase7_non_empty(&tenant_id, "tenant id")?;
        require_phase7_non_empty(&trace_id, "trace id")?;
        require_phase7_non_empty(&idempotency_key, "idempotency key")?;
        if caller_app_id
            .as_ref()
            .is_some_and(|caller| caller.trim().is_empty())
        {
            return Err(SdkPhase7Error::MissingField("caller app id"));
        }
        Ok(Self {
            caller_app_id,
            actor_id,
            tenant_id,
            trace_id,
            idempotency_key,
            schema_version: check_sdk_compatibility(SDK_CURRENT_STABLE_MAJOR, schema_version)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkAccountingReadRequest {
    pub endpoint: String,
    pub route: &'static str,
    pub read_kind: SdkAccountingReadKind,
    pub object_ref: String,
    pub metadata: SdkAccountingRequestMetadata,
    pub capability: SdkCapabilityDecision,
    pub headers: Vec<(String, String)>,
    pub read_only: bool,
    pub mutates_accounting_state: bool,
    pub service_returned_evidence_only: bool,
}

pub fn build_accounting_read_request(
    endpoint: &OvergateEndpoint,
    profile: &SdkServiceCapabilityProfile,
    read_kind: SdkAccountingReadKind,
    object_ref: impl Into<String>,
    metadata: SdkAccountingRequestMetadata,
    phase5_accounting_api_ready: bool,
    phase6_product_integration_ready: bool,
) -> Result<SdkAccountingReadRequest, SdkPhase7Error> {
    let object_ref = object_ref.into();
    require_phase7_non_empty(&object_ref, "accounting object ref")?;
    let capability = validate_accounting_reader_readiness(
        profile,
        phase5_accounting_api_ready,
        phase6_product_integration_ready,
        metadata.schema_version.raw(),
    )?;
    Ok(SdkAccountingReadRequest {
        endpoint: endpoint.raw().to_owned(),
        route: read_kind.route(),
        read_kind,
        object_ref,
        headers: phase7_headers(&metadata, read_kind.route()),
        metadata,
        capability,
        read_only: true,
        mutates_accounting_state: false,
        service_returned_evidence_only: true,
    })
}

pub fn validate_accounting_reader_readiness(
    profile: &SdkServiceCapabilityProfile,
    phase5_accounting_api_ready: bool,
    phase6_product_integration_ready: bool,
    schema_version: &str,
) -> Result<SdkCapabilityDecision, SdkPhase7Error> {
    if !phase5_accounting_api_ready {
        return Err(SdkPhase7Error::ReadinessGateClosed(
            "phase_5_accounting_api",
        ));
    }
    if !phase6_product_integration_ready {
        return Err(SdkPhase7Error::ReadinessGateClosed(
            "phase_6_product_integration",
        ));
    }
    Ok(negotiate_sdk_capability(
        profile,
        SdkOptionalHelper::AccountingReaders,
        schema_version,
        SDK_CURRENT_STABLE_MAJOR,
    )?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkOruDimensionTotal {
    pub dimension: String,
    pub quantity: u64,
    pub unit: String,
    pub source_ref: String,
}

impl SdkOruDimensionTotal {
    pub fn new(
        dimension: impl Into<String>,
        quantity: u64,
        unit: impl Into<String>,
        source_ref: impl Into<String>,
    ) -> Result<Self, SdkPhase7Error> {
        let dimension = dimension.into();
        let unit = unit.into();
        let source_ref = source_ref.into();
        require_phase7_non_empty(&dimension, "ORU dimension")?;
        require_phase7_non_empty(&unit, "ORU unit")?;
        require_phase7_non_empty(&source_ref, "ORU source ref")?;
        if quantity == 0 {
            return Err(SdkPhase7Error::InvalidDimensionQuantity(dimension));
        }
        Ok(Self {
            dimension,
            quantity,
            unit,
            source_ref,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SdkAccountingRefBundle {
    pub overmeter_rollup_refs: Vec<String>,
    pub oru_account_refs: Vec<String>,
    pub seal_ledger_refs: Vec<String>,
    pub overbill_refs: Vec<String>,
    pub overgrant_refs: Vec<String>,
    pub overasset_refs: Vec<String>,
    pub policy_refs: Vec<String>,
    pub audit_refs: Vec<String>,
}

impl SdkAccountingRefBundle {
    pub fn validate(&self) -> Result<(), SdkPhase7Error> {
        for (field, refs) in [
            ("overmeter rollup ref", &self.overmeter_rollup_refs),
            ("oru account ref", &self.oru_account_refs),
            ("seal ledger ref", &self.seal_ledger_refs),
            ("overbill ref", &self.overbill_refs),
            ("overgrant ref", &self.overgrant_refs),
            ("overasset ref", &self.overasset_refs),
            ("policy ref", &self.policy_refs),
            ("audit ref", &self.audit_refs),
        ] {
            validate_ref_list(field, refs)?;
        }
        if self.overmeter_rollup_refs.is_empty()
            && self.oru_account_refs.is_empty()
            && self.seal_ledger_refs.is_empty()
            && self.overbill_refs.is_empty()
            && self.overgrant_refs.is_empty()
            && self.overasset_refs.is_empty()
        {
            return Err(SdkPhase7Error::ServiceEvidenceRequired("accounting refs"));
        }
        Ok(())
    }
}

pub fn verify_accounting_refs_unchanged(
    expected: &SdkAccountingRefBundle,
    actual: &SdkAccountingRefBundle,
) -> Result<(), SdkPhase7Error> {
    if expected != actual {
        return Err(SdkPhase7Error::RefRewriteDetected("accounting refs"));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkUsageReceiptViewInput {
    pub service_reported: bool,
    pub receipt_ref: String,
    pub account_id: String,
    pub tenant_id: String,
    pub usage_rollup_refs: Vec<String>,
    pub oru_dimension_totals: Vec<SdkOruDimensionTotal>,
    pub accounting_refs: SdkAccountingRefBundle,
    pub dispute_window_ref: String,
    pub issued_at_ms: u64,
    pub trace_id: String,
    pub redaction_profile: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkUsageReceiptView {
    pub receipt_ref: String,
    pub account_id: String,
    pub tenant_id: String,
    pub usage_rollup_refs: Vec<String>,
    pub oru_dimension_totals: Vec<SdkOruDimensionTotal>,
    pub accounting_refs: SdkAccountingRefBundle,
    pub dispute_window_ref: String,
    pub issued_at_ms: u64,
    pub trace_id: String,
    pub redaction_profile: String,
    pub service_reported: bool,
    pub read_only: bool,
    pub mutates_accounting_state: bool,
    pub embeds_charge_tables: bool,
}

pub fn decode_usage_receipt_view(
    input: SdkUsageReceiptViewInput,
) -> Result<SdkUsageReceiptView, SdkPhase7Error> {
    if !input.service_reported {
        return Err(SdkPhase7Error::ServiceEvidenceRequired(
            "usage receipt service response",
        ));
    }
    require_phase7_non_empty(&input.receipt_ref, "receipt ref")?;
    require_phase7_non_empty(&input.account_id, "account id")?;
    require_phase7_non_empty(&input.tenant_id, "tenant id")?;
    require_phase7_non_empty(&input.dispute_window_ref, "dispute window ref")?;
    require_phase7_non_empty(&input.trace_id, "trace id")?;
    require_phase7_non_empty(&input.redaction_profile, "redaction profile")?;
    if input.issued_at_ms == 0 {
        return Err(SdkPhase7Error::MissingField("issued at"));
    }
    validate_non_empty_ref_list("usage rollup ref", &input.usage_rollup_refs)?;
    if input.oru_dimension_totals.is_empty() {
        return Err(SdkPhase7Error::ServiceEvidenceRequired(
            "ORU dimension totals",
        ));
    }
    input.accounting_refs.validate()?;

    Ok(SdkUsageReceiptView {
        receipt_ref: input.receipt_ref,
        account_id: input.account_id,
        tenant_id: input.tenant_id,
        usage_rollup_refs: input.usage_rollup_refs,
        oru_dimension_totals: input.oru_dimension_totals,
        accounting_refs: input.accounting_refs,
        dispute_window_ref: input.dispute_window_ref,
        issued_at_ms: input.issued_at_ms,
        trace_id: input.trace_id,
        redaction_profile: input.redaction_profile,
        service_reported: true,
        read_only: true,
        mutates_accounting_state: false,
        embeds_charge_tables: false,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkOruChargePreviewInput {
    pub service_reported: bool,
    pub preview_ref: String,
    pub workload_ref: String,
    pub tenant_id: String,
    pub oru_dimension_totals: Vec<SdkOruDimensionTotal>,
    pub accounting_refs: SdkAccountingRefBundle,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkOruChargePreview {
    pub preview_ref: String,
    pub workload_ref: String,
    pub tenant_id: String,
    pub oru_dimension_totals: Vec<SdkOruDimensionTotal>,
    pub accounting_refs: SdkAccountingRefBundle,
    pub trace_id: String,
    pub service_reported: bool,
    pub read_only: bool,
    pub embeds_charge_tables: bool,
    pub client_side_settlement_decision: bool,
}

pub fn decode_oru_charge_preview(
    input: SdkOruChargePreviewInput,
) -> Result<SdkOruChargePreview, SdkPhase7Error> {
    if !input.service_reported {
        return Err(SdkPhase7Error::ServiceEvidenceRequired(
            "ORU charge preview service response",
        ));
    }
    require_phase7_non_empty(&input.preview_ref, "charge preview ref")?;
    require_phase7_non_empty(&input.workload_ref, "workload ref")?;
    require_phase7_non_empty(&input.tenant_id, "tenant id")?;
    require_phase7_non_empty(&input.trace_id, "trace id")?;
    if input.oru_dimension_totals.is_empty() {
        return Err(SdkPhase7Error::ServiceEvidenceRequired(
            "ORU charge preview dimensions",
        ));
    }
    input.accounting_refs.validate()?;

    Ok(SdkOruChargePreview {
        preview_ref: input.preview_ref,
        workload_ref: input.workload_ref,
        tenant_id: input.tenant_id,
        oru_dimension_totals: input.oru_dimension_totals,
        accounting_refs: input.accounting_refs,
        trace_id: input.trace_id,
        service_reported: true,
        read_only: true,
        embeds_charge_tables: false,
        client_side_settlement_decision: false,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkAccountingDisputeReferenceInput {
    pub service_reported: bool,
    pub usage_ref: String,
    pub dispute_refs: Vec<String>,
    pub correction_refs: Vec<String>,
    pub challenge_window_ref: String,
    pub provider_payout_hold_refs: Vec<String>,
    pub refund_refs: Vec<String>,
    pub denied_settlement_reason_refs: Vec<String>,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkAccountingDisputeReferenceView {
    pub usage_ref: String,
    pub dispute_refs: Vec<String>,
    pub correction_refs: Vec<String>,
    pub challenge_window_ref: String,
    pub provider_payout_hold_refs: Vec<String>,
    pub refund_refs: Vec<String>,
    pub denied_settlement_reason_refs: Vec<String>,
    pub trace_id: String,
    pub service_reported: bool,
    pub generic_error_masked: bool,
    pub hides_holds_or_corrections: bool,
}

pub fn decode_dispute_reference_view(
    input: SdkAccountingDisputeReferenceInput,
) -> Result<SdkAccountingDisputeReferenceView, SdkPhase7Error> {
    if !input.service_reported {
        return Err(SdkPhase7Error::ServiceEvidenceRequired(
            "dispute reference service response",
        ));
    }
    require_phase7_non_empty(&input.usage_ref, "usage ref")?;
    require_phase7_non_empty(&input.challenge_window_ref, "challenge window ref")?;
    require_phase7_non_empty(&input.trace_id, "trace id")?;
    validate_non_empty_ref_list("dispute ref", &input.dispute_refs)?;
    validate_ref_list("correction ref", &input.correction_refs)?;
    validate_ref_list("provider payout hold ref", &input.provider_payout_hold_refs)?;
    validate_ref_list("refund ref", &input.refund_refs)?;
    validate_ref_list(
        "denied settlement reason ref",
        &input.denied_settlement_reason_refs,
    )?;

    Ok(SdkAccountingDisputeReferenceView {
        usage_ref: input.usage_ref,
        dispute_refs: input.dispute_refs,
        correction_refs: input.correction_refs,
        challenge_window_ref: input.challenge_window_ref,
        provider_payout_hold_refs: input.provider_payout_hold_refs,
        refund_refs: input.refund_refs,
        denied_settlement_reason_refs: input.denied_settlement_reason_refs,
        trace_id: input.trace_id,
        service_reported: true,
        generic_error_masked: false,
        hides_holds_or_corrections: false,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkAccountingErrorSurface {
    pub error: OverridErrorRecord,
    pub dispute_refs: Vec<String>,
    pub correction_refs: Vec<String>,
    pub hold_refs: Vec<String>,
    pub refund_refs: Vec<String>,
    pub denied_settlement_reason_refs: Vec<String>,
    pub generic_error_masked: bool,
}

pub fn build_accounting_error_surface(
    error: OverridErrorRecord,
    dispute_view: &SdkAccountingDisputeReferenceView,
) -> Result<SdkAccountingErrorSurface, SdkPhase7Error> {
    if dispute_view.generic_error_masked || dispute_view.hides_holds_or_corrections {
        return Err(SdkPhase7Error::GenericAccountingErrorMask);
    }
    Ok(SdkAccountingErrorSurface {
        error,
        dispute_refs: dispute_view.dispute_refs.clone(),
        correction_refs: dispute_view.correction_refs.clone(),
        hold_refs: dispute_view.provider_payout_hold_refs.clone(),
        refund_refs: dispute_view.refund_refs.clone(),
        denied_settlement_reason_refs: dispute_view.denied_settlement_reason_refs.clone(),
        generic_error_masked: false,
    })
}

pub fn build_accounting_receipt_idempotency_entry(
    context: &SdkRequestContextRecord,
    request_hash: impl Into<String>,
    terminal_response_digest: Option<String>,
    audit_refs: Vec<String>,
    correction_fields: Vec<String>,
) -> Result<Option<SdkIdempotencyEntry>, SdkPhase7Error> {
    Ok(SdkIdempotencyEntry::for_command_class(
        SdkCommandClass::AccountingReceiptOrDispute,
        context,
        request_hash,
        terminal_response_digest,
        audit_refs,
        overrid_contracts::RetryClass::NotRetryable,
        correction_fields,
    )?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdkPhase7AuthorityBoundary {
    pub helper: &'static str,
    pub approved_public_route: &'static str,
    pub read_only: bool,
    pub service_returned_evidence_only: bool,
    pub external_payment_provider_calls: bool,
    pub embeds_charge_tables: bool,
    pub client_side_settlement_decision: bool,
    pub mutates_accounting_state: bool,
    pub owning_services: &'static [&'static str],
}

pub fn sdk_phase7_authority_review() -> Vec<SdkPhase7AuthorityBoundary> {
    vec![
        SdkPhase7AuthorityBoundary {
            helper: "usage_receipt_view_reader",
            approved_public_route: SDK_PHASE7_USAGE_RECEIPT_ROUTE,
            read_only: true,
            service_returned_evidence_only: true,
            external_payment_provider_calls: false,
            embeds_charge_tables: false,
            client_side_settlement_decision: false,
            mutates_accounting_state: false,
            owning_services: &[
                "Overmeter",
                "ORU Account Service",
                "Seal Ledger",
                "Overbill",
            ],
        },
        SdkPhase7AuthorityBoundary {
            helper: "oru_charge_preview_reader",
            approved_public_route: SDK_PHASE7_ORU_CHARGE_PREVIEW_ROUTE,
            read_only: true,
            service_returned_evidence_only: true,
            external_payment_provider_calls: false,
            embeds_charge_tables: false,
            client_side_settlement_decision: false,
            mutates_accounting_state: false,
            owning_services: &["ORU Account Service", "Seal Ledger", "Overbill"],
        },
        SdkPhase7AuthorityBoundary {
            helper: "seal_ledger_reference_reader",
            approved_public_route: SDK_PHASE7_SEAL_LEDGER_REF_ROUTE,
            read_only: true,
            service_returned_evidence_only: true,
            external_payment_provider_calls: false,
            embeds_charge_tables: false,
            client_side_settlement_decision: false,
            mutates_accounting_state: false,
            owning_services: &["Seal Ledger", "Overwatch"],
        },
        SdkPhase7AuthorityBoundary {
            helper: "dispute_correction_reference_reader",
            approved_public_route: SDK_PHASE7_DISPUTE_REFERENCE_ROUTE,
            read_only: true,
            service_returned_evidence_only: true,
            external_payment_provider_calls: false,
            embeds_charge_tables: false,
            client_side_settlement_decision: false,
            mutates_accounting_state: false,
            owning_services: &[
                "Overclaim",
                "Overbill",
                "Provider Payout Service",
                "Overgrant",
                "Overasset",
            ],
        },
        SdkPhase7AuthorityBoundary {
            helper: "accounting_readiness_gate",
            approved_public_route: "capability_profile",
            read_only: true,
            service_returned_evidence_only: true,
            external_payment_provider_calls: false,
            embeds_charge_tables: false,
            client_side_settlement_decision: false,
            mutates_accounting_state: false,
            owning_services: SDK_PHASE7_ACCOUNTING_AUTHORITY_OWNERS,
        },
    ]
}

pub fn validate_phase7_authority_review(
    review: &[SdkPhase7AuthorityBoundary],
) -> Result<(), SdkPhase7Error> {
    for boundary in review {
        if !boundary.read_only
            || !boundary.service_returned_evidence_only
            || boundary.external_payment_provider_calls
            || boundary.embeds_charge_tables
            || boundary.client_side_settlement_decision
            || boundary.mutates_accounting_state
        {
            return Err(SdkPhase7Error::AccountingAuthorityLeak(boundary.helper));
        }
    }
    for owner in SDK_PHASE7_ACCOUNTING_AUTHORITY_OWNERS {
        if !review
            .iter()
            .any(|boundary| boundary.owning_services.contains(owner))
        {
            return Err(SdkPhase7Error::AccountingAuthorityLeak(owner));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkPhase7Error {
    Sdk(SdkError),
    MissingField(&'static str),
    InvalidDimensionQuantity(String),
    ServiceEvidenceRequired(&'static str),
    ReadinessGateClosed(&'static str),
    RefRewriteDetected(&'static str),
    GenericAccountingErrorMask,
    AccountingAuthorityLeak(&'static str),
}

impl fmt::Display for SdkPhase7Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sdk(error) => error.fmt(formatter),
            Self::MissingField(field) => write!(formatter, "{field} is required"),
            Self::InvalidDimensionQuantity(dimension) => {
                write!(
                    formatter,
                    "ORU dimension quantity must be positive: {dimension}"
                )
            }
            Self::ServiceEvidenceRequired(surface) => {
                write!(formatter, "service evidence is required for {surface}")
            }
            Self::ReadinessGateClosed(gate) => write!(formatter, "readiness gate closed: {gate}"),
            Self::RefRewriteDetected(surface) => {
                write!(formatter, "ref rewrite detected: {surface}")
            }
            Self::GenericAccountingErrorMask => {
                write!(
                    formatter,
                    "accounting refs cannot be hidden behind generic errors"
                )
            }
            Self::AccountingAuthorityLeak(helper) => {
                write!(
                    formatter,
                    "SDK helper claims accounting authority: {helper}"
                )
            }
        }
    }
}

impl std::error::Error for SdkPhase7Error {}

impl From<SdkError> for SdkPhase7Error {
    fn from(error: SdkError) -> Self {
        Self::Sdk(error)
    }
}

impl From<crate::SdkCompatibilityRejection> for SdkPhase7Error {
    fn from(error: crate::SdkCompatibilityRejection) -> Self {
        Self::Sdk(SdkError::Compatibility(error))
    }
}

fn phase7_headers(metadata: &SdkAccountingRequestMetadata, route: &str) -> Vec<(String, String)> {
    let mut headers = vec![
        ("x-overrid-target".to_owned(), "overgate".to_owned()),
        ("x-overrid-route".to_owned(), route.to_owned()),
        ("x-overrid-sdk-name".to_owned(), SDK_NAME.to_owned()),
        ("x-overrid-sdk-version".to_owned(), SDK_VERSION.to_owned()),
        (
            "x-overrid-sdk-language".to_owned(),
            SDK_LANGUAGE_BINDING.to_owned(),
        ),
        (
            "x-overrid-sdk-capability-profile".to_owned(),
            SDK_PHASE7_CAPABILITY_PROFILE.to_owned(),
        ),
        (
            "x-overrid-schema-version".to_owned(),
            metadata.schema_version.raw().to_owned(),
        ),
        ("x-overrid-actor-id".to_owned(), metadata.actor_id.clone()),
        ("x-overrid-tenant-id".to_owned(), metadata.tenant_id.clone()),
        ("x-overrid-trace-id".to_owned(), metadata.trace_id.clone()),
        (
            "x-overrid-idempotency-key".to_owned(),
            metadata.idempotency_key.clone(),
        ),
    ];
    if let Some(caller_app_id) = &metadata.caller_app_id {
        headers.push(("x-overrid-caller-app-id".to_owned(), caller_app_id.clone()));
    }
    headers
}

fn validate_non_empty_ref_list(field: &'static str, refs: &[String]) -> Result<(), SdkPhase7Error> {
    if refs.is_empty() {
        return Err(SdkPhase7Error::ServiceEvidenceRequired(field));
    }
    validate_ref_list(field, refs)
}

fn validate_ref_list(field: &'static str, refs: &[String]) -> Result<(), SdkPhase7Error> {
    for value in refs {
        require_phase7_non_empty(value, field)?;
    }
    Ok(())
}

fn require_phase7_non_empty(value: &str, field: &'static str) -> Result<(), SdkPhase7Error> {
    if value.trim().is_empty() {
        return Err(SdkPhase7Error::MissingField(field));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use overrid_contracts::{RetryClass, SUPPORTED_SCHEMA_VERSION};

    fn accounting_profile(accounting: bool) -> SdkServiceCapabilityProfile {
        SdkServiceCapabilityProfile {
            profile_name: SDK_PHASE7_CAPABILITY_PROFILE.to_owned(),
            supported_schema_versions: vec![SUPPORTED_SCHEMA_VERSION.to_owned()],
            supported_sdk_majors: vec![SDK_CURRENT_STABLE_MAJOR],
            signing: true,
            idempotency: true,
            dry_run: true,
            accounting,
        }
    }

    fn metadata() -> SdkAccountingRequestMetadata {
        SdkAccountingRequestMetadata::new(
            Some("app:docdex".to_owned()),
            "actor:alice",
            "tenant:local",
            "trace:phase7",
            "idem:phase7",
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap()
    }

    fn dimension() -> SdkOruDimensionTotal {
        SdkOruDimensionTotal::new("GPU-ORU", 42, "oru", "overmeter:rollup:usage-1").unwrap()
    }

    fn refs() -> SdkAccountingRefBundle {
        SdkAccountingRefBundle {
            overmeter_rollup_refs: vec!["overmeter:rollup:usage-1".to_owned()],
            oru_account_refs: vec!["oru:account:tenant:local".to_owned()],
            seal_ledger_refs: vec!["seal-ledger:entry:123".to_owned()],
            overbill_refs: vec!["overbill:receipt:123".to_owned()],
            overgrant_refs: vec!["overgrant:grant:compute".to_owned()],
            overasset_refs: vec!["overasset:right:compute".to_owned()],
            policy_refs: vec!["overguard:policy:egress-deny".to_owned()],
            audit_refs: vec!["overwatch:audit:phase7".to_owned()],
        }
    }

    fn dispute_view() -> SdkAccountingDisputeReferenceView {
        decode_dispute_reference_view(SdkAccountingDisputeReferenceInput {
            service_reported: true,
            usage_ref: "usage:rollup:1".to_owned(),
            dispute_refs: vec!["overclaim:dispute:1".to_owned()],
            correction_refs: vec!["seal-ledger:correction:1".to_owned()],
            challenge_window_ref: "overclaim:challenge-window:1".to_owned(),
            provider_payout_hold_refs: vec!["provider-payout:hold:1".to_owned()],
            refund_refs: vec!["overbill:refund:1".to_owned()],
            denied_settlement_reason_refs: vec!["settlement_denied_policy".to_owned()],
            trace_id: "trace:phase7".to_owned(),
        })
        .unwrap()
    }

    #[test]
    fn phase7_accounting_readers_are_capability_gated_and_read_only() {
        let endpoint = OvergateEndpoint::parse(
            "http://127.0.0.1:18080/overgate",
            overrid_contracts::EnvironmentClass::Local,
        )
        .unwrap();
        let profile = accounting_profile(true);

        let request = build_accounting_read_request(
            &endpoint,
            &profile,
            SdkAccountingReadKind::UsageReceipt,
            "overbill:receipt:123",
            metadata(),
            true,
            true,
        )
        .unwrap();

        assert_eq!(request.route, SDK_PHASE7_USAGE_RECEIPT_ROUTE);
        assert!(request.read_only);
        assert!(!request.mutates_accounting_state);
        assert!(request.service_returned_evidence_only);
        assert_eq!(
            request.capability.helper,
            SdkOptionalHelper::AccountingReaders
        );
        assert_eq!(
            request.capability.snapshot.phase_gate,
            "phase_5_metering_accounting"
        );
        assert!(request.headers.contains(&(
            "x-overrid-caller-app-id".to_owned(),
            "app:docdex".to_owned()
        )));
    }

    #[test]
    fn phase7_usage_receipt_view_consumes_service_objects_without_charge_tables() {
        let view = decode_usage_receipt_view(SdkUsageReceiptViewInput {
            service_reported: true,
            receipt_ref: "overbill:receipt:123".to_owned(),
            account_id: "oru:account:tenant:local".to_owned(),
            tenant_id: "tenant:local".to_owned(),
            usage_rollup_refs: vec!["overmeter:rollup:usage-1".to_owned()],
            oru_dimension_totals: vec![dimension()],
            accounting_refs: refs(),
            dispute_window_ref: "overclaim:window:receipt-123".to_owned(),
            issued_at_ms: 1_782_021_000_000,
            trace_id: "trace:phase7".to_owned(),
            redaction_profile: "receipt_default".to_owned(),
        })
        .unwrap();

        assert!(view.service_reported);
        assert!(view.read_only);
        assert!(!view.mutates_accounting_state);
        assert!(!view.embeds_charge_tables);
        assert_eq!(
            view.accounting_refs.seal_ledger_refs,
            vec!["seal-ledger:entry:123"]
        );
        assert!(matches!(
            decode_usage_receipt_view(SdkUsageReceiptViewInput {
                service_reported: false,
                receipt_ref: "overbill:receipt:123".to_owned(),
                account_id: "oru:account:tenant:local".to_owned(),
                tenant_id: "tenant:local".to_owned(),
                usage_rollup_refs: vec!["overmeter:rollup:usage-1".to_owned()],
                oru_dimension_totals: vec![dimension()],
                accounting_refs: refs(),
                dispute_window_ref: "overclaim:window:receipt-123".to_owned(),
                issued_at_ms: 1_782_021_000_000,
                trace_id: "trace:phase7".to_owned(),
                redaction_profile: "receipt_default".to_owned(),
            }),
            Err(SdkPhase7Error::ServiceEvidenceRequired(
                "usage receipt service response"
            ))
        ));
    }

    #[test]
    fn phase7_metering_and_accounting_refs_round_trip_without_rewriting() {
        let original = refs();
        let from_service = refs();

        verify_accounting_refs_unchanged(&original, &from_service).unwrap();

        let preview = decode_oru_charge_preview(SdkOruChargePreviewInput {
            service_reported: true,
            preview_ref: "oru:preview:workload-1".to_owned(),
            workload_ref: "workload:1".to_owned(),
            tenant_id: "tenant:local".to_owned(),
            oru_dimension_totals: vec![dimension()],
            accounting_refs: from_service,
            trace_id: "trace:phase7".to_owned(),
        })
        .unwrap();

        assert!(preview.read_only);
        assert!(!preview.embeds_charge_tables);
        assert!(!preview.client_side_settlement_decision);
        assert_eq!(
            preview.accounting_refs.overgrant_refs,
            vec!["overgrant:grant:compute"]
        );

        let mut rewritten = original.clone();
        rewritten.seal_ledger_refs = vec!["seal-ledger:entry:rewritten".to_owned()];
        assert!(matches!(
            verify_accounting_refs_unchanged(&original, &rewritten),
            Err(SdkPhase7Error::RefRewriteDetected("accounting refs"))
        ));
    }

    #[test]
    fn phase7_dispute_and_correction_readers_surface_stable_refs() {
        let dispute = dispute_view();

        assert_eq!(dispute.dispute_refs, vec!["overclaim:dispute:1"]);
        assert_eq!(dispute.correction_refs, vec!["seal-ledger:correction:1"]);
        assert_eq!(
            dispute.provider_payout_hold_refs,
            vec!["provider-payout:hold:1"]
        );
        assert_eq!(dispute.refund_refs, vec!["overbill:refund:1"]);
        assert_eq!(
            dispute.denied_settlement_reason_refs,
            vec!["settlement_denied_policy"]
        );
        assert!(!dispute.generic_error_masked);
        assert!(!dispute.hides_holds_or_corrections);
    }

    #[test]
    fn phase7_authority_review_blocks_payment_calls_and_accounting_leaks() {
        let review = sdk_phase7_authority_review();

        validate_phase7_authority_review(&review).unwrap();
        assert!(review.iter().all(|boundary| boundary.read_only));
        assert!(review
            .iter()
            .all(|boundary| !boundary.external_payment_provider_calls));
        assert!(review.iter().all(|boundary| !boundary.embeds_charge_tables));
        assert!(review
            .iter()
            .all(|boundary| !boundary.client_side_settlement_decision));
        assert!(review
            .iter()
            .all(|boundary| !boundary.mutates_accounting_state));
    }

    #[test]
    fn phase7_accounting_helpers_fail_closed_before_readiness() {
        let profile = accounting_profile(true);

        assert!(matches!(
            validate_accounting_reader_readiness(&profile, false, true, SUPPORTED_SCHEMA_VERSION),
            Err(SdkPhase7Error::ReadinessGateClosed(
                "phase_5_accounting_api"
            ))
        ));
        assert!(matches!(
            validate_accounting_reader_readiness(&profile, true, false, SUPPORTED_SCHEMA_VERSION),
            Err(SdkPhase7Error::ReadinessGateClosed(
                "phase_6_product_integration"
            ))
        ));
        assert!(matches!(
            validate_accounting_reader_readiness(
                &accounting_profile(false),
                true,
                true,
                SUPPORTED_SCHEMA_VERSION
            ),
            Err(SdkPhase7Error::Sdk(SdkError::CapabilityUnavailable {
                helper: "accounting_readers",
                ..
            }))
        ));
    }

    #[test]
    fn phase7_error_surfaces_preserve_refs_without_generic_masking() {
        let error = OverridErrorRecord::new(
            "settlement_denied_policy",
            "settlement denied by policy evidence",
            Some("trace:phase7".to_owned()),
            vec!["overwatch:audit:phase7".to_owned()],
            RetryClass::NotRetryable,
            vec!["review_dispute_ref".to_owned()],
            Some("overbill".to_owned()),
            vec!["overguard:policy:egress-deny".to_owned()],
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap();
        let surface = build_accounting_error_surface(error, &dispute_view()).unwrap();

        assert_eq!(surface.error.reason_code, "settlement_denied_policy");
        assert_eq!(surface.dispute_refs, vec!["overclaim:dispute:1"]);
        assert_eq!(surface.correction_refs, vec!["seal-ledger:correction:1"]);
        assert_eq!(surface.hold_refs, vec!["provider-payout:hold:1"]);
        assert_eq!(surface.refund_refs, vec!["overbill:refund:1"]);
        assert!(!surface.generic_error_masked);
    }

    #[test]
    fn phase7_accounting_receipt_idempotency_uses_receipt_retention() {
        let context = SdkRequestContextRecord::new(
            "actor:alice",
            "tenant:local",
            "trace:phase7",
            "idem:receipt:123",
            "receipt read",
            Some("app:docdex".to_owned()),
            1_782_021_000_000,
            SUPPORTED_SCHEMA_VERSION,
        )
        .unwrap();

        let entry = build_accounting_receipt_idempotency_entry(
            &context,
            "hash_0123456789abcdef",
            Some("digest_receipt_123".to_owned()),
            vec!["overwatch:audit:phase7".to_owned()],
            vec!["dispute_ref".to_owned()],
        )
        .unwrap()
        .unwrap();

        assert_eq!(
            entry.command_class,
            SdkCommandClass::AccountingReceiptOrDispute
        );
        assert_eq!(entry.retention_ms, 30 * 24 * 60 * 60 * 1_000);
        assert!(!entry.contains_raw_payload);
    }
}
