use axum::Router;

use crate::dependencies::DependencyMatrix;
use crate::repository::InMemoryCredentialRepository;
use crate::routes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverkeyConfig {
    pub service_id: String,
    pub bind_addr: String,
    pub public_base_path: String,
    pub local_stack_profile: String,
    pub fixture_set_ref: String,
    pub harness_scenario_ref: String,
}

impl OverkeyConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();
        if let Ok(bind_addr) = std::env::var("OVERRID_OVERKEY_BIND_ADDR") {
            config.bind_addr = bind_addr;
        }
        if let Ok(base_path) = std::env::var("OVERRID_OVERKEY_BASE_PATH") {
            config.public_base_path = normalize_base_path(&base_path);
        }
        if let Ok(profile) = std::env::var("OVERRID_LOCAL_PROFILE") {
            config.local_stack_profile = profile;
        }
        config
    }
}

impl Default for OverkeyConfig {
    fn default() -> Self {
        Self {
            service_id: "service:overkey".to_owned(),
            bind_addr: "127.0.0.1:18080".to_owned(),
            public_base_path: "/overkey".to_owned(),
            local_stack_profile: "local".to_owned(),
            fixture_set_ref: "fixture:overkey_phase2_credential_smoke".to_owned(),
            harness_scenario_ref: "scenario:overkey_phase2_credential_smoke".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OverkeyState {
    pub config: OverkeyConfig,
    pub dependencies: DependencyMatrix,
    pub repository: InMemoryCredentialRepository,
}

#[derive(Debug, Clone)]
pub struct OverkeyService {
    state: OverkeyState,
}

impl OverkeyService {
    pub fn new(config: OverkeyConfig) -> Self {
        Self::with_dependencies(config, DependencyMatrix::default())
    }

    pub fn with_dependencies(config: OverkeyConfig, dependencies: DependencyMatrix) -> Self {
        Self {
            state: OverkeyState {
                config,
                dependencies,
                repository: InMemoryCredentialRepository::default(),
            },
        }
    }

    pub fn state(&self) -> &OverkeyState {
        &self.state
    }

    pub fn router(&self) -> Router {
        let base_path = self.state.config.public_base_path.clone();
        route_tree()
            .nest(&base_path, route_tree())
            .with_state(self.state.clone())
    }
}

impl Default for OverkeyService {
    fn default() -> Self {
        Self::new(OverkeyConfig::default())
    }
}

fn route_tree() -> Router<OverkeyState> {
    routes::public_routes()
}

fn normalize_base_path(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "/" {
        return "/overkey".to_owned();
    }
    if trimmed.starts_with('/') {
        trimmed.trim_end_matches('/').to_owned()
    } else {
        format!("/{}", trimmed.trim_end_matches('/'))
    }
}
