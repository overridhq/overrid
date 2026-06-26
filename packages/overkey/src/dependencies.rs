use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyRequirement {
    Required,
    Optional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyState {
    Available,
    Unavailable,
    Stubbed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DependencyCheck {
    pub dependency_id: &'static str,
    pub requirement: DependencyRequirement,
    pub state: DependencyState,
    pub contract_ref: &'static str,
    pub reason_code: &'static str,
}

impl DependencyCheck {
    pub const fn required_available(
        dependency_id: &'static str,
        contract_ref: &'static str,
    ) -> Self {
        Self {
            dependency_id,
            requirement: DependencyRequirement::Required,
            state: DependencyState::Available,
            contract_ref,
            reason_code: "dependency.available",
        }
    }

    pub const fn optional_stubbed(dependency_id: &'static str, contract_ref: &'static str) -> Self {
        Self {
            dependency_id,
            requirement: DependencyRequirement::Optional,
            state: DependencyState::Stubbed,
            contract_ref,
            reason_code: "dependency.stubbed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DependencyMatrix {
    pub service_id: &'static str,
    pub checks: Vec<DependencyCheck>,
}

impl DependencyMatrix {
    pub fn required_failures(&self) -> Vec<&'static str> {
        self.checks
            .iter()
            .filter(|check| {
                check.requirement == DependencyRequirement::Required
                    && check.state == DependencyState::Unavailable
            })
            .map(|check| check.dependency_id)
            .collect()
    }

    pub fn ready(&self) -> bool {
        self.required_failures().is_empty()
    }
}

impl Default for DependencyMatrix {
    fn default() -> Self {
        Self {
            service_id: "service:overkey",
            checks: vec![
                DependencyCheck::required_available(
                    "schema_validation",
                    "schema:overkey:credential_record:v0",
                ),
                DependencyCheck::required_available(
                    "credential_metadata_repository",
                    "repository:overkey:append_friendly_local_stub",
                ),
                DependencyCheck::required_available(
                    "overgate_callback",
                    "callback:overgate:command_admission:v0",
                ),
                DependencyCheck::required_available(
                    "overpass_callback",
                    "callback:overpass:actor_resolution:v0",
                ),
                DependencyCheck::required_available(
                    "overtenant_callback",
                    "callback:overtenant:tenant_context:v0",
                ),
                DependencyCheck::required_available(
                    "overwatch_event_sink",
                    "event:overwatch:credential_lifecycle:v0",
                ),
                DependencyCheck::required_available(
                    "overvault_secret_ref_resolver",
                    "secret_ref:overvault:local_stub:v0",
                ),
                DependencyCheck::optional_stubbed(
                    "later_overguard_policy",
                    "policy:overguard:credential_use:v0",
                ),
                DependencyCheck::optional_stubbed(
                    "later_overmeter_usage_events",
                    "metric:overmeter:credential_usage:v0",
                ),
            ],
        }
    }
}
