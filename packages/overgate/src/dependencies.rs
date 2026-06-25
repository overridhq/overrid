use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyRequirement {
    Required,
    OptionalFuture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyState {
    Ready,
    Unavailable,
    NotConfigured,
}

impl DependencyState {
    pub fn reason_code(self, dependency_id: &str) -> String {
        match self {
            Self::Ready => format!("{dependency_id}.ready"),
            Self::Unavailable => format!("{dependency_id}.unavailable"),
            Self::NotConfigured => format!("{dependency_id}.not_configured"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DependencyCheck {
    pub dependency_id: String,
    pub service_ref: String,
    pub requirement: DependencyRequirement,
    pub state: DependencyState,
    pub reason_code: String,
}

impl DependencyCheck {
    pub fn required(dependency_id: &str, service_ref: &str) -> Self {
        Self::new(
            dependency_id,
            service_ref,
            DependencyRequirement::Required,
            DependencyState::Ready,
        )
    }

    pub fn optional_future(dependency_id: &str, service_ref: &str) -> Self {
        Self::new(
            dependency_id,
            service_ref,
            DependencyRequirement::OptionalFuture,
            DependencyState::NotConfigured,
        )
    }

    pub fn with_state(mut self, state: DependencyState) -> Self {
        self.state = state;
        self.reason_code = state.reason_code(&self.dependency_id);
        self
    }

    fn new(
        dependency_id: &str,
        service_ref: &str,
        requirement: DependencyRequirement,
        state: DependencyState,
    ) -> Self {
        Self {
            dependency_id: dependency_id.to_owned(),
            service_ref: service_ref.to_owned(),
            requirement,
            state,
            reason_code: state.reason_code(dependency_id),
        }
    }

    pub fn is_required_ready(&self) -> bool {
        self.requirement != DependencyRequirement::Required || self.state == DependencyState::Ready
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DependencyMatrix {
    pub checks: Vec<DependencyCheck>,
}

impl DependencyMatrix {
    pub fn phase2_local_stack() -> Self {
        Self {
            checks: vec![
                DependencyCheck::required("schema_validation", "service:shared_schema_package"),
                DependencyCheck::required("overkey_lite", "service:overkey"),
                DependencyCheck::required("overpass", "service:overpass"),
                DependencyCheck::required("overtenant", "service:overtenant"),
                DependencyCheck::required("overwatch", "service:overwatch"),
                DependencyCheck::required("overqueue", "service:overqueue"),
                DependencyCheck::required("forwarding_targets", "service:downstream_contracts"),
                DependencyCheck::optional_future("later_overguard", "service:overguard"),
                DependencyCheck::optional_future("later_overmeter", "service:overmeter"),
                DependencyCheck::optional_future("later_oru", "service:oru_account_service"),
            ],
        }
    }

    pub fn with_dependency_state(mut self, dependency_id: &str, state: DependencyState) -> Self {
        for check in &mut self.checks {
            if check.dependency_id == dependency_id {
                *check = check.clone().with_state(state);
            }
        }
        self
    }

    pub fn required_ready(&self) -> bool {
        self.checks.iter().all(DependencyCheck::is_required_ready)
    }

    pub fn required_failures(&self) -> Vec<String> {
        self.checks
            .iter()
            .filter(|check| !check.is_required_ready())
            .map(|check| check.dependency_id.clone())
            .collect()
    }

    pub fn readiness_reason_code(&self) -> &'static str {
        if self.required_ready() {
            "overgate.ready"
        } else {
            "overgate.dependency_unavailable"
        }
    }
}

impl Default for DependencyMatrix {
    fn default() -> Self {
        Self::phase2_local_stack()
    }
}
