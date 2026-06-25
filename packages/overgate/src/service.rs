use axum::Router;

use crate::admin;
use crate::dependencies::DependencyMatrix;
use crate::idempotency::IdempotencyStore;
use crate::prechecks::PrecheckStore;
use crate::routes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OvergateConfig {
    pub service_id: String,
    pub bind_addr: String,
    pub public_base_path: String,
    pub local_stack_profile: String,
    pub fixture_set_ref: String,
    pub harness_scenario_ref: String,
}

impl OvergateConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();
        if let Ok(bind_addr) = std::env::var("OVERRID_OVERGATE_BIND_ADDR") {
            config.bind_addr = bind_addr;
        }
        if let Ok(base_path) = std::env::var("OVERRID_OVERGATE_BASE_PATH") {
            config.public_base_path = normalize_base_path(&base_path);
        }
        if let Ok(profile) = std::env::var("OVERRID_LOCAL_PROFILE") {
            config.local_stack_profile = profile;
        }
        config
    }
}

impl Default for OvergateConfig {
    fn default() -> Self {
        Self {
            service_id: "service:overgate".to_owned(),
            bind_addr: "127.0.0.1:18080".to_owned(),
            public_base_path: "/overgate".to_owned(),
            local_stack_profile: "local".to_owned(),
            fixture_set_ref: "fixture:overgate_phase2_command_smoke".to_owned(),
            harness_scenario_ref: "scenario:overgate_phase2_command_smoke".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OvergateState {
    pub config: OvergateConfig,
    pub dependencies: DependencyMatrix,
    pub idempotency: IdempotencyStore,
    pub prechecks: PrecheckStore,
}

#[derive(Debug, Clone)]
pub struct OvergateService {
    state: OvergateState,
}

impl OvergateService {
    pub fn new(config: OvergateConfig) -> Self {
        Self::with_dependencies(config, DependencyMatrix::default())
    }

    pub fn with_dependencies(config: OvergateConfig, dependencies: DependencyMatrix) -> Self {
        Self {
            state: OvergateState {
                config,
                dependencies,
                idempotency: IdempotencyStore::default(),
                prechecks: PrecheckStore::default(),
            },
        }
    }

    pub fn state(&self) -> &OvergateState {
        &self.state
    }

    pub fn router(&self) -> Router {
        let base_path = self.state.config.public_base_path.clone();
        route_tree()
            .nest(&base_path, route_tree())
            .with_state(self.state.clone())
    }
}

impl Default for OvergateService {
    fn default() -> Self {
        Self::new(OvergateConfig::default())
    }
}

fn route_tree() -> Router<OvergateState> {
    routes::public_routes().merge(admin::admin_routes())
}

fn normalize_base_path(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "/" {
        return "/overgate".to_owned();
    }
    if trimmed.starts_with('/') {
        trimmed.trim_end_matches('/').to_owned()
    } else {
        format!("/{}", trimmed.trim_end_matches('/'))
    }
}
