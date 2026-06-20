use overrid_contracts::{
    FixtureKey, FixtureManifest, HarnessRunStatus, SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
};

pub const DEFAULT_FIXTURE_SEED: &str = "seed_phase0_smoke_0001";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicFixtureSeed {
    pub scenario_id: String,
    pub seed: String,
    pub fixture_id: String,
}

impl DeterministicFixtureSeed {
    pub fn new(scenario_id: impl Into<String>, seed: impl Into<String>) -> Self {
        let scenario_id = scenario_id.into();
        let seed = seed.into();
        let fixture_id = format!("fixture_{}", stable_token(&[&scenario_id, &seed]));
        Self {
            scenario_id,
            seed,
            fixture_id,
        }
    }
}

pub fn stable_token(parts: &[&str]) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

pub fn stable_short_token(parts: &[&str]) -> String {
    stable_token(parts).chars().take(12).collect()
}

pub fn sanitize_identifier(value: &str) -> String {
    let sanitized = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    let compact = sanitized
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_");
    if compact.is_empty() {
        "value".to_owned()
    } else {
        compact
    }
}

pub fn fixture_id_from_ref(value: &str) -> String {
    if let Some(rest) = value.strip_prefix("fixture:") {
        format!("fixture_{}", sanitize_identifier(rest))
    } else {
        sanitize_identifier(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureIdentityKind {
    Person,
    Organization,
    ServiceAccount,
    SystemService,
    TestActor,
}

impl FixtureIdentityKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Person => "person",
            Self::Organization => "organization",
            Self::ServiceAccount => "service_account",
            Self::SystemService => "system_service",
            Self::TestActor => "test_actor",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureAccessState {
    Active,
    Suspended,
}

impl FixtureAccessState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Suspended => "suspended",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleBindingFixture {
    pub binding_ref: String,
    pub tenant_ref: String,
    pub actor_ref: String,
    pub role_ref: String,
    pub allowed: bool,
}

impl RoleBindingFixture {
    pub fn new(
        tenant_ref: impl Into<String>,
        actor_ref: impl Into<String>,
        role_ref: impl Into<String>,
        allowed: bool,
    ) -> Self {
        let tenant_ref = tenant_ref.into();
        let actor_ref = actor_ref.into();
        let role_ref = role_ref.into();
        let binding_ref = format!(
            "role_binding:{}",
            stable_short_token(&[&tenant_ref, &actor_ref, &role_ref])
        );
        Self {
            binding_ref,
            tenant_ref,
            actor_ref,
            role_ref,
            allowed,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotaFixture {
    pub quota_ref: String,
    pub tenant_ref: String,
    pub limit: u64,
    pub used: u64,
}

impl QuotaFixture {
    pub fn new(tenant_ref: impl Into<String>, limit: u64, used: u64) -> Self {
        let tenant_ref = tenant_ref.into();
        let quota_ref = format!(
            "quota:{}:{}",
            sanitize_identifier(&tenant_ref),
            stable_short_token(&[&tenant_ref, &limit.to_string(), &used.to_string()])
        );
        Self {
            quota_ref,
            tenant_ref,
            limit,
            used,
        }
    }

    pub fn is_exhausted(&self) -> bool {
        self.used >= self.limit
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantFixture {
    pub tenant_ref: String,
    pub organization_ref: String,
    pub state: FixtureAccessState,
    pub quota: QuotaFixture,
    pub test_only: bool,
}

impl TenantFixture {
    pub fn local_alpha(seed: &str) -> Self {
        let tenant_ref = format!(
            "tenant:local:alpha:{}",
            stable_short_token(&[seed, "tenant"])
        );
        let organization_ref = format!(
            "organization:local:alpha:{}",
            stable_short_token(&[seed, "organization"])
        );
        Self {
            quota: QuotaFixture::new(&tenant_ref, 1_000, 0),
            tenant_ref,
            organization_ref,
            state: FixtureAccessState::Active,
            test_only: true,
        }
    }

    pub fn suspended(mut self) -> Self {
        self.state = FixtureAccessState::Suspended;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityFixture {
    pub identity_ref: String,
    pub tenant_ref: String,
    pub actor_ref: String,
    pub kind: FixtureIdentityKind,
    pub role_bindings: Vec<RoleBindingFixture>,
    pub state: FixtureAccessState,
    pub test_only: bool,
}

impl IdentityFixture {
    pub fn new(
        tenant_ref: impl Into<String>,
        actor_ref: impl Into<String>,
        kind: FixtureIdentityKind,
        seed: &str,
    ) -> Self {
        let tenant_ref = tenant_ref.into();
        let actor_ref = actor_ref.into();
        let identity_ref = format!(
            "identity:{}:{}",
            kind.as_str(),
            stable_short_token(&[&tenant_ref, &actor_ref, kind.as_str(), seed])
        );
        let role_bindings = vec![RoleBindingFixture::new(
            &tenant_ref,
            &actor_ref,
            "role:local:harness_builder",
            true,
        )];
        Self {
            identity_ref,
            tenant_ref,
            actor_ref,
            kind,
            role_bindings,
            state: FixtureAccessState::Active,
            test_only: true,
        }
    }

    pub fn person(tenant_ref: impl Into<String>, actor_ref: impl Into<String>, seed: &str) -> Self {
        Self::new(tenant_ref, actor_ref, FixtureIdentityKind::Person, seed)
    }

    pub fn organization(
        tenant_ref: impl Into<String>,
        actor_ref: impl Into<String>,
        seed: &str,
    ) -> Self {
        Self::new(
            tenant_ref,
            actor_ref,
            FixtureIdentityKind::Organization,
            seed,
        )
    }

    pub fn service_account(
        tenant_ref: impl Into<String>,
        actor_ref: impl Into<String>,
        seed: &str,
    ) -> Self {
        Self::new(
            tenant_ref,
            actor_ref,
            FixtureIdentityKind::ServiceAccount,
            seed,
        )
    }

    pub fn system_service(
        tenant_ref: impl Into<String>,
        actor_ref: impl Into<String>,
        seed: &str,
    ) -> Self {
        Self::new(
            tenant_ref,
            actor_ref,
            FixtureIdentityKind::SystemService,
            seed,
        )
    }

    pub fn test_actor(
        tenant_ref: impl Into<String>,
        actor_ref: impl Into<String>,
        seed: &str,
    ) -> Self {
        Self::new(tenant_ref, actor_ref, FixtureIdentityKind::TestActor, seed)
    }

    pub fn suspended(mut self) -> Self {
        self.state = FixtureAccessState::Suspended;
        self
    }

    pub fn denied_role(mut self) -> Self {
        for binding in &mut self.role_bindings {
            binding.allowed = false;
        }
        self
    }

    pub fn expected_admission(&self, tenant: &TenantFixture) -> FixtureAdmissionOutcome {
        if !self.test_only || !tenant.test_only {
            return FixtureAdmissionOutcome::blocked("safety.fixture_not_test_only");
        }
        if tenant.state == FixtureAccessState::Suspended {
            return FixtureAdmissionOutcome::blocked("fixture.tenant_suspended");
        }
        if self.state == FixtureAccessState::Suspended {
            return FixtureAdmissionOutcome::blocked("fixture.actor_suspended");
        }
        if tenant.quota.is_exhausted() {
            return FixtureAdmissionOutcome::blocked("fixture.quota_exhausted");
        }
        if self.role_bindings.iter().all(|binding| !binding.allowed) {
            return FixtureAdmissionOutcome::blocked("fixture.role_denied");
        }
        FixtureAdmissionOutcome::accepted("fixture.identity_accepted")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureAdmissionOutcome {
    pub accepted: bool,
    pub reason_code: String,
}

impl FixtureAdmissionOutcome {
    pub fn accepted(reason_code: impl Into<String>) -> Self {
        Self {
            accepted: true,
            reason_code: reason_code.into(),
        }
    }

    pub fn blocked(reason_code: impl Into<String>) -> Self {
        Self {
            accepted: false,
            reason_code: reason_code.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigningKeyVariant {
    Active,
    Rotated,
    Revoked,
    WrongKey,
    WrongTenant,
    Expired,
    InvalidSignature,
    OutsideLocalProfile,
}

impl SigningKeyVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Rotated => "rotated",
            Self::Revoked => "revoked",
            Self::WrongKey => "wrong_key",
            Self::WrongTenant => "wrong_tenant",
            Self::Expired => "expired",
            Self::InvalidSignature => "invalid_signature",
            Self::OutsideLocalProfile => "outside_local_profile",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestSigningKeyFixture {
    pub key: FixtureKey,
    pub tenant_ref: String,
    pub actor_ref: String,
    pub variant: SigningKeyVariant,
    pub local_test_profile_only: bool,
}

impl TestSigningKeyFixture {
    pub fn new(
        tenant_ref: impl Into<String>,
        actor_ref: impl Into<String>,
        variant: SigningKeyVariant,
        seed: &str,
    ) -> Self {
        let tenant_ref = tenant_ref.into();
        let actor_ref = actor_ref.into();
        let key_id = format!(
            "test_key_{}_{}",
            variant.as_str(),
            stable_short_token(&[&tenant_ref, &actor_ref, variant.as_str(), seed])
        );
        let key_ref = format!("key:fixture:{key_id}");
        Self {
            key: FixtureKey::test_only(key_ref, key_id),
            tenant_ref,
            actor_ref,
            variant,
            local_test_profile_only: true,
        }
    }

    pub fn active(tenant_ref: impl Into<String>, actor_ref: impl Into<String>, seed: &str) -> Self {
        Self::new(tenant_ref, actor_ref, SigningKeyVariant::Active, seed)
    }

    pub fn expected_signature_result(&self, profile: &str) -> FixtureAdmissionOutcome {
        if !is_fixture_local_profile(profile) {
            return FixtureAdmissionOutcome::blocked("safety.fixture_key_outside_local");
        }
        if self.key.validate().is_err() {
            return FixtureAdmissionOutcome::blocked("safety.fixture_not_test_only");
        }
        match self.variant {
            SigningKeyVariant::Active | SigningKeyVariant::Rotated => {
                FixtureAdmissionOutcome::accepted("signature.accepted")
            }
            SigningKeyVariant::Revoked => FixtureAdmissionOutcome::blocked("signature.revoked_key"),
            SigningKeyVariant::WrongKey => FixtureAdmissionOutcome::blocked("signature.wrong_key"),
            SigningKeyVariant::WrongTenant => {
                FixtureAdmissionOutcome::blocked("signature.wrong_tenant")
            }
            SigningKeyVariant::Expired => FixtureAdmissionOutcome::blocked("signature.expired_key"),
            SigningKeyVariant::InvalidSignature => {
                FixtureAdmissionOutcome::blocked("signature.invalid")
            }
            SigningKeyVariant::OutsideLocalProfile => {
                FixtureAdmissionOutcome::blocked("safety.fixture_key_outside_local")
            }
        }
    }

    pub fn artifact_safe_refs(&self) -> Vec<String> {
        vec![
            self.key.key_ref.clone(),
            format!("signature_ref:{}:redacted", self.key.key_id),
        ]
    }
}

pub fn is_fixture_local_profile(profile: &str) -> bool {
    matches!(
        profile,
        "local"
            | "local-dev"
            | "test"
            | "ci"
            | "local-already-running"
            | "local-health-timeout"
            | "local-port-conflict"
            | "local-degraded-optional"
            | "local-reset-incomplete"
            | "local-unmarked-state"
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceCardFixture {
    pub resource_card_ref: String,
    pub cpu_units: u16,
    pub memory_mb: u32,
    pub gpu_profile_ref: Option<String>,
}

impl ResourceCardFixture {
    pub fn synthetic_cpu(seed: &str) -> Self {
        Self {
            resource_card_ref: format!(
                "resource:local:synthetic_cpu:{}",
                stable_short_token(&[seed, "resource"])
            ),
            cpu_units: 2,
            memory_mb: 2048,
            gpu_profile_ref: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkloadFixture {
    pub command_ref: String,
    pub workload_ref: String,
    pub package_ref: String,
    pub resource_card: ResourceCardFixture,
    pub node_simulator_ref: String,
    pub queue_item_ref: String,
    pub lease_placeholder_ref: String,
    pub runner_state_ref: String,
    pub phase_tag: u8,
    pub required_services: Vec<String>,
    pub pending_queue_state_only: bool,
    pub execution_claimed: bool,
}

impl WorkloadFixture {
    pub fn phase1_pending_noop(seed: &str) -> Self {
        let token = stable_short_token(&[seed, "phase1_pending_noop"]);
        Self {
            command_ref: format!("command:local:no_op:{token}"),
            workload_ref: format!("workload:local:no_op:{token}"),
            package_ref: format!("package:local:no_op:{token}"),
            resource_card: ResourceCardFixture::synthetic_cpu(seed),
            node_simulator_ref: format!("node:simulator:local:{token}"),
            queue_item_ref: format!("queue:pending:{token}"),
            lease_placeholder_ref: format!("lease:placeholder:{token}"),
            runner_state_ref: format!("runner_state:not_started:{token}"),
            phase_tag: 1,
            required_services: vec![
                "service:overgate".to_owned(),
                "service:overwatch".to_owned(),
                "component:overqueue_durable_state".to_owned(),
            ],
            pending_queue_state_only: true,
            execution_claimed: false,
        }
    }

    pub fn validate_phase1_boundary(&self) -> FixtureAdmissionOutcome {
        if self.phase_tag != 1 {
            return FixtureAdmissionOutcome::blocked("dependency.phase_tag_unsupported");
        }
        if !self.pending_queue_state_only || self.execution_claimed {
            return FixtureAdmissionOutcome::blocked("fixture.execution_not_available_in_phase1");
        }
        FixtureAdmissionOutcome::accepted("fixture.pending_queue_state_only")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountingPolicyFixture {
    pub local_oru_account_ref: String,
    pub usage_dimension_ref: String,
    pub receipt_ref: String,
    pub budget_exhausted: bool,
    pub policy_context_ref: String,
    pub denied_class_ref: String,
    pub test_usage: bool,
    pub external_payment_provider_reachable: bool,
    pub provider_payout_reachable: bool,
    pub owning_service_refs: Vec<String>,
}

impl AccountingPolicyFixture {
    pub fn local_test(seed: &str) -> Self {
        let token = stable_short_token(&[seed, "accounting_policy"]);
        Self {
            local_oru_account_ref: format!("ledger:local:oru_account:{token}"),
            usage_dimension_ref: format!("usage:local:test_dimension:{token}"),
            receipt_ref: format!("receipt:local:test_only:{token}"),
            budget_exhausted: false,
            policy_context_ref: format!("policy:local:allow_smoke:{token}"),
            denied_class_ref: format!("policy:local:deny_external_payment:{token}"),
            test_usage: true,
            external_payment_provider_reachable: false,
            provider_payout_reachable: false,
            owning_service_refs: vec![
                "service:overmeter".to_owned(),
                "service:oru_account".to_owned(),
                "service:seal_ledger".to_owned(),
                "service:overbill".to_owned(),
            ],
        }
    }

    pub fn budget_exhausted(mut self) -> Self {
        self.budget_exhausted = true;
        self.denied_class_ref = format!(
            "policy:local:budget_exhausted:{}",
            stable_short_token(&[&self.local_oru_account_ref])
        );
        self
    }

    pub fn validate_test_isolation(&self) -> FixtureAdmissionOutcome {
        if !self.test_usage {
            return FixtureAdmissionOutcome::blocked("accounting.not_test_usage");
        }
        if self.external_payment_provider_reachable || self.provider_payout_reachable {
            return FixtureAdmissionOutcome::blocked("safety.external_payment_reachable");
        }
        if self.budget_exhausted {
            return FixtureAdmissionOutcome::blocked("policy.budget_exhausted");
        }
        FixtureAdmissionOutcome::accepted("accounting.test_usage_isolated")
    }

    pub fn validate_owning_services_available(
        &self,
        available_service_refs: &[&str],
    ) -> FixtureAdmissionOutcome {
        let missing_owning_service = self.owning_service_refs.iter().any(|required| {
            !available_service_refs
                .iter()
                .any(|available| *available == required.as_str())
        });
        if missing_owning_service {
            return FixtureAdmissionOutcome::blocked("dependency.phase_contract_not_ready");
        }
        FixtureAdmissionOutcome::accepted("accounting.owning_services_available")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureLibrary {
    pub manifest: FixtureManifest,
    pub tenant: TenantFixture,
    pub identities: Vec<IdentityFixture>,
    pub signing_keys: Vec<TestSigningKeyFixture>,
    pub workload: WorkloadFixture,
    pub accounting_policy: AccountingPolicyFixture,
    pub golden_trace_refs: Vec<String>,
}

impl FixtureLibrary {
    pub fn phase5_local(seed: &str) -> Result<Self, overrid_contracts::HarnessContractError> {
        let deterministic = DeterministicFixtureSeed::new("scenario_phase1_control_spine", seed);
        let tenant = TenantFixture::local_alpha(seed);
        let builder_actor = format!(
            "actor:local:builder:{}",
            stable_short_token(&[seed, "actor"])
        );
        let identities = vec![
            IdentityFixture::person(&tenant.tenant_ref, &builder_actor, seed),
            IdentityFixture::organization(&tenant.tenant_ref, "actor:local:organization", seed),
            IdentityFixture::service_account(&tenant.tenant_ref, "actor:local:service", seed),
            IdentityFixture::system_service(&tenant.tenant_ref, "actor:local:system", seed),
            IdentityFixture::test_actor(&tenant.tenant_ref, "actor:local:test_actor", seed),
        ];
        let signing_keys = vec![
            TestSigningKeyFixture::active(&tenant.tenant_ref, &builder_actor, seed),
            TestSigningKeyFixture::new(
                &tenant.tenant_ref,
                &builder_actor,
                SigningKeyVariant::Rotated,
                seed,
            ),
            TestSigningKeyFixture::new(
                &tenant.tenant_ref,
                &builder_actor,
                SigningKeyVariant::Revoked,
                seed,
            ),
            TestSigningKeyFixture::new(
                "tenant:local:wrong",
                &builder_actor,
                SigningKeyVariant::WrongTenant,
                seed,
            ),
            TestSigningKeyFixture::new(
                &tenant.tenant_ref,
                &builder_actor,
                SigningKeyVariant::Expired,
                seed,
            ),
            TestSigningKeyFixture::new(
                &tenant.tenant_ref,
                &builder_actor,
                SigningKeyVariant::WrongKey,
                seed,
            ),
            TestSigningKeyFixture::new(
                &tenant.tenant_ref,
                &builder_actor,
                SigningKeyVariant::InvalidSignature,
                seed,
            ),
        ];
        let manifest = FixtureManifest::new(
            deterministic.fixture_id,
            tenant.tenant_ref.clone(),
            builder_actor,
            deterministic.seed,
            signing_keys
                .iter()
                .map(|fixture| fixture.key.clone())
                .collect(),
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
        )?;
        Ok(Self {
            manifest,
            tenant,
            identities,
            signing_keys,
            workload: WorkloadFixture::phase1_pending_noop(seed),
            accounting_policy: AccountingPolicyFixture::local_test(seed),
            golden_trace_refs: vec![
                "golden_trace:phase0:exact_signed_noop".to_owned(),
                "golden_trace:phase1:pending_queue_spine".to_owned(),
            ],
        })
    }

    pub fn drift_snapshot(&self) -> FixtureDriftSnapshot {
        FixtureDriftSnapshot {
            schema_version: self.manifest.schema_version.raw().to_owned(),
            fixture_id: self.manifest.fixture_id.clone(),
            expected_ids: vec![
                self.tenant.tenant_ref.clone(),
                self.manifest.actor_ref.clone(),
                self.workload.workload_ref.clone(),
                self.accounting_policy.local_oru_account_ref.clone(),
            ],
            generated_refs: self.generated_refs(),
            golden_trace_refs: self.golden_trace_refs.clone(),
        }
    }

    pub fn generated_refs(&self) -> Vec<String> {
        let mut refs = vec![
            self.manifest.fixture_id.clone(),
            self.tenant.tenant_ref.clone(),
            self.tenant.organization_ref.clone(),
            self.workload.command_ref.clone(),
            self.workload.workload_ref.clone(),
            self.workload.package_ref.clone(),
            self.workload.resource_card.resource_card_ref.clone(),
            self.workload.queue_item_ref.clone(),
            self.accounting_policy.local_oru_account_ref.clone(),
            self.accounting_policy.policy_context_ref.clone(),
        ];
        refs.extend(
            self.identities
                .iter()
                .map(|identity| identity.identity_ref.clone()),
        );
        refs.extend(
            self.signing_keys
                .iter()
                .map(|signing_key| signing_key.key.key_ref.clone()),
        );
        refs.sort();
        refs
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureDriftSnapshot {
    pub schema_version: String,
    pub fixture_id: String,
    pub expected_ids: Vec<String>,
    pub generated_refs: Vec<String>,
    pub golden_trace_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureDriftDifference {
    pub field_path: String,
    pub expected_value: String,
    pub actual_value: String,
    pub reason_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureDriftReport {
    pub status: HarnessRunStatus,
    pub reason_code: String,
    pub stable_diff_fields: Vec<String>,
    pub differences: Vec<FixtureDriftDifference>,
}

impl FixtureDriftReport {
    pub fn has_drift(&self) -> bool {
        !self.differences.is_empty()
    }
}

pub fn compare_fixture_drift(
    expected: &FixtureDriftSnapshot,
    actual: &FixtureDriftSnapshot,
) -> FixtureDriftReport {
    let mut differences = Vec::new();
    push_diff(
        &mut differences,
        "$.schema_version",
        &expected.schema_version,
        &actual.schema_version,
    );
    push_diff(
        &mut differences,
        "$.fixture_id",
        &expected.fixture_id,
        &actual.fixture_id,
    );
    push_vec_diff(
        &mut differences,
        "$.expected_ids",
        &expected.expected_ids,
        &actual.expected_ids,
    );
    push_vec_diff(
        &mut differences,
        "$.generated_refs",
        &expected.generated_refs,
        &actual.generated_refs,
    );
    push_vec_diff(
        &mut differences,
        "$.golden_trace_refs",
        &expected.golden_trace_refs,
        &actual.golden_trace_refs,
    );

    FixtureDriftReport {
        status: if differences.is_empty() {
            HarnessRunStatus::Passed
        } else {
            HarnessRunStatus::Failed
        },
        reason_code: if differences.is_empty() {
            "fixture.drift_absent".to_owned()
        } else {
            "fixture.drift_detected".to_owned()
        },
        stable_diff_fields: vec![
            "$.schema_version".to_owned(),
            "$.fixture_id".to_owned(),
            "$.expected_ids".to_owned(),
            "$.generated_refs".to_owned(),
            "$.golden_trace_refs".to_owned(),
        ],
        differences,
    }
}

fn push_diff(
    differences: &mut Vec<FixtureDriftDifference>,
    field_path: &str,
    expected: &str,
    actual: &str,
) {
    if expected != actual {
        differences.push(FixtureDriftDifference {
            field_path: field_path.to_owned(),
            expected_value: expected.to_owned(),
            actual_value: actual.to_owned(),
            reason_code: "fixture.field_drift".to_owned(),
        });
    }
}

fn push_vec_diff(
    differences: &mut Vec<FixtureDriftDifference>,
    field_path: &str,
    expected: &[String],
    actual: &[String],
) {
    if expected != actual {
        differences.push(FixtureDriftDifference {
            field_path: field_path.to_owned(),
            expected_value: expected.join(","),
            actual_value: actual.join(","),
            reason_code: "fixture.ref_set_drift".to_owned(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_seed_is_deterministic() {
        let first = DeterministicFixtureSeed::new("scenario_phase0_smoke", DEFAULT_FIXTURE_SEED);
        let second = DeterministicFixtureSeed::new("scenario_phase0_smoke", DEFAULT_FIXTURE_SEED);
        assert_eq!(first, second);
        assert!(first.fixture_id.starts_with("fixture_"));
    }

    #[test]
    fn fixture_refs_normalize_to_fixture_ids() {
        assert_eq!(
            fixture_id_from_ref("fixture:phase0_smoke"),
            "fixture_phase0_smoke"
        );
        assert_eq!(
            fixture_id_from_ref("fixture_phase0_smoke"),
            "fixture_phase0_smoke"
        );
    }

    #[test]
    fn phase5_library_emits_schema_checked_test_only_fixture_manifest() {
        let library = FixtureLibrary::phase5_local(DEFAULT_FIXTURE_SEED).unwrap();

        assert!(library.manifest.test_only);
        assert_eq!(
            library.manifest.schema_version.raw(),
            SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION
        );
        assert_eq!(library.identities.len(), 5);
        assert!(library.identities.iter().all(|identity| identity.test_only));
        assert!(library.signing_keys.iter().all(|fixture| {
            fixture.key.test_only
                && fixture.key.signature_ref_only
                && !fixture.key.raw_key_material_present
        }));
        assert!(library
            .manifest
            .local_ledger_account_refs
            .iter()
            .all(|value| value.starts_with("ledger:")));
        library.manifest.validate().unwrap();
    }

    #[test]
    fn identity_and_tenant_variants_have_stable_accept_deny_behavior() {
        let tenant = TenantFixture::local_alpha(DEFAULT_FIXTURE_SEED);
        let actor = IdentityFixture::test_actor(
            &tenant.tenant_ref,
            "actor:local:builder",
            DEFAULT_FIXTURE_SEED,
        );

        assert_eq!(
            actor.expected_admission(&tenant),
            FixtureAdmissionOutcome::accepted("fixture.identity_accepted")
        );
        assert_eq!(
            actor.expected_admission(&tenant.clone().suspended()),
            FixtureAdmissionOutcome::blocked("fixture.tenant_suspended")
        );
        assert_eq!(
            actor.clone().suspended().expected_admission(&tenant),
            FixtureAdmissionOutcome::blocked("fixture.actor_suspended")
        );
        assert_eq!(
            actor.denied_role().expected_admission(&tenant),
            FixtureAdmissionOutcome::blocked("fixture.role_denied")
        );
    }

    #[test]
    fn signing_key_variants_fail_closed_without_raw_key_material() {
        let tenant = TenantFixture::local_alpha(DEFAULT_FIXTURE_SEED);
        let active = TestSigningKeyFixture::active(
            &tenant.tenant_ref,
            "actor:local:builder",
            DEFAULT_FIXTURE_SEED,
        );

        assert_eq!(
            active.expected_signature_result("local"),
            FixtureAdmissionOutcome::accepted("signature.accepted")
        );
        assert_eq!(
            active.expected_signature_result("production"),
            FixtureAdmissionOutcome::blocked("safety.fixture_key_outside_local")
        );
        assert!(active
            .artifact_safe_refs()
            .iter()
            .all(|value| !value.contains("private") && !value.contains("secret")));

        for (variant, reason_code) in [
            (SigningKeyVariant::Revoked, "signature.revoked_key"),
            (SigningKeyVariant::WrongKey, "signature.wrong_key"),
            (SigningKeyVariant::WrongTenant, "signature.wrong_tenant"),
            (SigningKeyVariant::Expired, "signature.expired_key"),
            (SigningKeyVariant::InvalidSignature, "signature.invalid"),
        ] {
            let fixture = TestSigningKeyFixture::new(
                &tenant.tenant_ref,
                "actor:local:builder",
                variant,
                DEFAULT_FIXTURE_SEED,
            );
            assert_eq!(
                fixture.expected_signature_result("local"),
                FixtureAdmissionOutcome::blocked(reason_code)
            );
        }
    }

    #[test]
    fn workload_fixture_keeps_phase1_at_pending_queue_state() {
        let workload = WorkloadFixture::phase1_pending_noop(DEFAULT_FIXTURE_SEED);

        assert_eq!(workload.phase_tag, 1);
        assert!(workload.pending_queue_state_only);
        assert!(!workload.execution_claimed);
        assert_eq!(
            workload.validate_phase1_boundary(),
            FixtureAdmissionOutcome::accepted("fixture.pending_queue_state_only")
        );
        assert!(workload
            .required_services
            .contains(&"component:overqueue_durable_state".to_owned()));
    }

    #[test]
    fn accounting_policy_fixture_is_test_usage_and_payment_isolated() {
        let fixture = AccountingPolicyFixture::local_test(DEFAULT_FIXTURE_SEED);

        assert!(fixture.test_usage);
        assert!(!fixture.external_payment_provider_reachable);
        assert!(!fixture.provider_payout_reachable);
        assert_eq!(
            fixture.validate_test_isolation(),
            FixtureAdmissionOutcome::accepted("accounting.test_usage_isolated")
        );
        assert_eq!(
            fixture.clone().budget_exhausted().validate_test_isolation(),
            FixtureAdmissionOutcome::blocked("policy.budget_exhausted")
        );
        assert_eq!(
            fixture.validate_owning_services_available(&[
                "service:overmeter",
                "service:oru_account",
                "service:seal_ledger",
                "service:overbill",
            ]),
            FixtureAdmissionOutcome::accepted("accounting.owning_services_available")
        );
        assert_eq!(
            fixture.validate_owning_services_available(&["service:overmeter"]),
            FixtureAdmissionOutcome::blocked("dependency.phase_contract_not_ready")
        );
    }

    #[test]
    fn fixture_drift_report_uses_stable_diff_fields() {
        let library = FixtureLibrary::phase5_local(DEFAULT_FIXTURE_SEED).unwrap();
        let expected = library.drift_snapshot();
        let mut actual = expected.clone();
        actual
            .expected_ids
            .push("tenant:local:unexpected".to_owned());
        actual
            .golden_trace_refs
            .push("golden_trace:unexpected".to_owned());

        let report = compare_fixture_drift(&expected, &actual);

        assert!(report.has_drift());
        assert_eq!(report.status, HarnessRunStatus::Failed);
        assert_eq!(report.reason_code, "fixture.drift_detected");
        assert!(report
            .stable_diff_fields
            .contains(&"$.golden_trace_refs".to_owned()));
        assert!(report
            .differences
            .iter()
            .any(|difference| difference.field_path == "$.expected_ids"));
    }
}
