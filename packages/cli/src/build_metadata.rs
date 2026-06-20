use overrid_contracts::{
    cli_contract_set, CONTRACT_SOURCE_ROOT, GENERATED_CONTRACT_STATUS, SUPPORTED_SCHEMA_VERSION,
};

pub const CLI_NAME: &str = "overrid";
pub const CLI_PACKAGE: &str = env!("CARGO_PKG_NAME");
pub const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionInfo {
    pub cli_name: &'static str,
    pub cli_package: &'static str,
    pub cli_version: &'static str,
    pub contract_source_root: &'static str,
    pub contract_status: &'static str,
    pub schema_version: &'static str,
    pub sdk_target: &'static str,
}

pub fn version_info() -> VersionInfo {
    let contracts = cli_contract_set();
    VersionInfo {
        cli_name: CLI_NAME,
        cli_package: CLI_PACKAGE,
        cli_version: CLI_VERSION,
        contract_source_root: CONTRACT_SOURCE_ROOT,
        contract_status: GENERATED_CONTRACT_STATUS,
        schema_version: contracts.schema_version,
        sdk_target: "overgate_only",
    }
}

pub fn human_version_lines() -> Vec<String> {
    let info = version_info();
    vec![
        format!("{} {}", info.cli_name, info.cli_version),
        format!("package: {}", info.cli_package),
        format!("schema: {}", SUPPORTED_SCHEMA_VERSION),
        format!("contract_source: {}", info.contract_source_root),
        format!("sdk_target: {}", info.sdk_target),
    ]
}
