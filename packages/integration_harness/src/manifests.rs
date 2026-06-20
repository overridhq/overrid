use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use overrid_contracts::{
    FixtureKey, FixtureManifest, HarnessContractError, HarnessRunStatus, ScenarioActionKind,
    ScenarioManifest, ScenarioStep, SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION,
};

use crate::fixtures::fixture_id_from_ref;
use crate::phase_gate::scenario_matches_phase_filter;

pub const DEFAULT_MANIFEST_DIR: &str = "packages/schemas/overrid_contracts/fixtures/valid";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureManifestRef {
    pub fixture_id: String,
    pub tenant_ref: String,
    pub actor_ref: String,
    pub deterministic_seed: String,
    pub source_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioStepRef {
    pub step_id: String,
    pub action_kind: ScenarioActionKind,
    pub input_refs: Vec<String>,
    pub timeout_ms: u64,
    pub retry_expectation: String,
    pub expected_result_class: HarnessRunStatus,
    pub assertion_refs: Vec<String>,
    pub cleanup_rule: String,
}

impl ScenarioStepRef {
    pub fn action_kind_str(&self) -> &'static str {
        self.action_kind.as_str()
    }

    fn to_contract_step(&self) -> ScenarioStep {
        ScenarioStep {
            step_id: self.step_id.clone(),
            action_kind: self.action_kind,
            input_refs: self.input_refs.clone(),
            timeout_ms: self.timeout_ms,
            retry_expectation: self.retry_expectation.clone(),
            expected_result_class: self.expected_result_class,
            assertion_refs: self.assertion_refs.clone(),
            cleanup_rule: self.cleanup_rule.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioManifestRef {
    pub scenario_id: String,
    pub master_phase: u8,
    pub gate_class: String,
    pub tags: Vec<String>,
    pub required_services: Vec<String>,
    pub setup_fixture_refs: Vec<String>,
    pub steps: Vec<ScenarioStepRef>,
    pub source_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessManifestCatalog {
    pub fixtures: Vec<FixtureManifestRef>,
    pub scenarios: Vec<ScenarioManifestRef>,
    pub source_paths: Vec<String>,
}

impl HarnessManifestCatalog {
    pub fn scenario(&self, scenario_id: &str) -> Option<&ScenarioManifestRef> {
        self.scenarios
            .iter()
            .find(|scenario| scenario.scenario_id == scenario_id)
    }

    pub fn scenarios_for_phase(&self, phase_filter: Option<u8>) -> Vec<ScenarioManifestRef> {
        self.scenarios
            .iter()
            .filter(|scenario| scenario_matches_phase_filter(scenario.master_phase, phase_filter))
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestLoadError {
    Io { path: String, message: String },
    InvalidJson { path: String },
    IncompatibleVersion { path: String, found: Option<String> },
    UnsafeField { path: String, field: String },
    MissingField { path: String, field: &'static str },
    MissingFixture { path: String, fixture_ref: String },
    DuplicateScenarioId { scenario_id: String },
    Contract(String),
}

impl fmt::Display for ManifestLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, message } => write!(formatter, "{path}: {message}"),
            Self::InvalidJson { path } => write!(formatter, "{path}: invalid JSON document"),
            Self::IncompatibleVersion { path, found } => write!(
                formatter,
                "{path}: incompatible integration harness schema version {}",
                found.as_deref().unwrap_or("<missing>")
            ),
            Self::UnsafeField { path, field } => {
                write!(formatter, "{path}: unsafe field rejected: {field}")
            }
            Self::MissingField { path, field } => write!(formatter, "{path}: missing {field}"),
            Self::MissingFixture { path, fixture_ref } => {
                write!(formatter, "{path}: missing fixture for {fixture_ref}")
            }
            Self::DuplicateScenarioId { scenario_id } => {
                write!(formatter, "duplicate scenario id: {scenario_id}")
            }
            Self::Contract(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for ManifestLoadError {}

impl From<HarnessContractError> for ManifestLoadError {
    fn from(error: HarnessContractError) -> Self {
        Self::Contract(error.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessManifestLoader {
    root: PathBuf,
}

impl HarnessManifestLoader {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn canonical(repo_root: impl AsRef<Path>) -> Self {
        Self::new(repo_root.as_ref().join(DEFAULT_MANIFEST_DIR))
    }

    pub fn load_catalog(&self) -> Result<HarnessManifestCatalog, ManifestLoadError> {
        let mut paths = fs::read_dir(&self.root)
            .map_err(|error| ManifestLoadError::Io {
                path: self.root.to_string_lossy().into_owned(),
                message: error.to_string(),
            })?
            .map(|entry| {
                entry
                    .map(|entry| entry.path())
                    .map_err(|error| ManifestLoadError::Io {
                        path: self.root.to_string_lossy().into_owned(),
                        message: error.to_string(),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        paths.retain(|path| match path.extension() {
            Some(extension) => extension == "json",
            None => false,
        });
        paths.sort();

        let mut documents = Vec::new();
        for path in paths {
            let display_path = path.to_string_lossy().into_owned();
            let text = fs::read_to_string(&path).map_err(|error| ManifestLoadError::Io {
                path: display_path.clone(),
                message: error.to_string(),
            })?;
            documents.push((display_path, text));
        }

        let borrowed = documents
            .iter()
            .map(|(path, text)| (path.as_str(), text.as_str()))
            .collect::<Vec<_>>();
        Self::load_catalog_from_documents(&borrowed)
    }

    pub fn load_catalog_from_documents(
        documents: &[(&str, &str)],
    ) -> Result<HarnessManifestCatalog, ManifestLoadError> {
        let mut fixtures = Vec::new();
        let mut scenarios = Vec::new();
        let mut source_paths = Vec::new();
        let mut scenario_ids = BTreeSet::new();

        for (path, text) in documents {
            let document = validate_manifest_document(path, text)?;
            if !scenario_ids.insert(document.scenario.scenario_id.clone()) {
                return Err(ManifestLoadError::DuplicateScenarioId {
                    scenario_id: document.scenario.scenario_id,
                });
            }
            source_paths.push((*path).to_owned());
            fixtures.push(document.fixture);
            scenarios.push(document.scenario);
        }

        let fixture_ids = fixtures
            .iter()
            .map(|fixture| fixture.fixture_id.as_str())
            .collect::<BTreeSet<_>>();
        for scenario in &scenarios {
            for fixture_ref in &scenario.setup_fixture_refs {
                let fixture_id = fixture_id_from_ref(fixture_ref);
                if !fixture_ids.contains(fixture_id.as_str()) {
                    return Err(ManifestLoadError::MissingFixture {
                        path: scenario.source_path.clone(),
                        fixture_ref: fixture_ref.clone(),
                    });
                }
            }
        }

        Ok(HarnessManifestCatalog {
            fixtures,
            scenarios,
            source_paths,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidatedManifestDocument {
    fixture: FixtureManifestRef,
    scenario: ScenarioManifestRef,
}

fn validate_manifest_document(
    path: &str,
    text: &str,
) -> Result<ValidatedManifestDocument, ManifestLoadError> {
    let trimmed = text.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') || !balanced_json_delimiters(trimmed) {
        return Err(ManifestLoadError::InvalidJson {
            path: path.to_owned(),
        });
    }

    reject_unsafe_fields(path, text)?;

    let schema_version = extract_string_field(text, "schema_version").ok_or_else(|| {
        ManifestLoadError::MissingField {
            path: path.to_owned(),
            field: "schema_version",
        }
    })?;
    if schema_version != SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION {
        return Err(ManifestLoadError::IncompatibleVersion {
            path: path.to_owned(),
            found: Some(schema_version),
        });
    }

    let fixture_id = extract_string_field(text, "fixture_id").ok_or_else(|| {
        ManifestLoadError::MissingField {
            path: path.to_owned(),
            field: "fixture_id",
        }
    })?;
    let tenant_ref = extract_string_field(text, "tenant_ref").ok_or_else(|| {
        ManifestLoadError::MissingField {
            path: path.to_owned(),
            field: "tenant_ref",
        }
    })?;
    let actor_ref =
        extract_string_field(text, "actor_ref").ok_or_else(|| ManifestLoadError::MissingField {
            path: path.to_owned(),
            field: "actor_ref",
        })?;
    let deterministic_seed = extract_string_field(text, "deterministic_seed").ok_or_else(|| {
        ManifestLoadError::MissingField {
            path: path.to_owned(),
            field: "deterministic_seed",
        }
    })?;

    FixtureManifest::new(
        &fixture_id,
        &tenant_ref,
        &actor_ref,
        &deterministic_seed,
        vec![FixtureKey::test_only(
            "key:fixture:local_builder",
            "fixture_local_builder",
        )],
        &schema_version,
    )?;

    let scenario_id = extract_string_field(text, "scenario_id").ok_or_else(|| {
        ManifestLoadError::MissingField {
            path: path.to_owned(),
            field: "scenario_id",
        }
    })?;
    let master_phase =
        extract_u8_field(text, "master_phase").ok_or_else(|| ManifestLoadError::MissingField {
            path: path.to_owned(),
            field: "master_phase",
        })?;
    let gate_class = extract_string_field(text, "gate_class").unwrap_or_else(|| "smoke".to_owned());
    let tags = extract_string_array(text, "tags");
    let required_services = extract_string_array(text, "required_services");
    let setup_fixture_refs = extract_string_array(text, "setup_fixture_refs");
    if setup_fixture_refs.is_empty() {
        return Err(ManifestLoadError::MissingFixture {
            path: path.to_owned(),
            fixture_ref: "<missing>".to_owned(),
        });
    }

    if !setup_fixture_refs
        .iter()
        .any(|fixture_ref| fixture_id_from_ref(fixture_ref) == fixture_id)
    {
        return Err(ManifestLoadError::MissingFixture {
            path: path.to_owned(),
            fixture_ref: setup_fixture_refs.join(","),
        });
    }

    let steps = extract_scenario_steps(path, text)?;
    let contract_steps = steps
        .iter()
        .map(ScenarioStepRef::to_contract_step)
        .collect::<Vec<_>>();
    for step in &contract_steps {
        step.validate()?;
    }
    ScenarioManifest::new(&scenario_id, master_phase, contract_steps, &schema_version)?;

    Ok(ValidatedManifestDocument {
        fixture: FixtureManifestRef {
            fixture_id,
            tenant_ref,
            actor_ref,
            deterministic_seed,
            source_path: path.to_owned(),
        },
        scenario: ScenarioManifestRef {
            scenario_id,
            master_phase,
            gate_class,
            tags,
            required_services,
            setup_fixture_refs,
            steps,
            source_path: path.to_owned(),
        },
    })
}

fn extract_scenario_steps(
    path: &str,
    text: &str,
) -> Result<Vec<ScenarioStepRef>, ManifestLoadError> {
    let step_objects = extract_object_array(text, "steps");
    if step_objects.is_empty() {
        return Err(ManifestLoadError::MissingField {
            path: path.to_owned(),
            field: "scenario step",
        });
    }

    let mut steps = Vec::new();
    for step_text in step_objects {
        let step_id = extract_string_field(&step_text, "step_id").ok_or_else(|| {
            ManifestLoadError::MissingField {
                path: path.to_owned(),
                field: "step_id",
            }
        })?;
        let action_kind = extract_string_field(&step_text, "action_kind")
            .ok_or_else(|| ManifestLoadError::MissingField {
                path: path.to_owned(),
                field: "action_kind",
            })
            .and_then(|raw| {
                ScenarioActionKind::parse(&raw).map_err(ManifestLoadError::from)
            })?;
        let timeout_ms = extract_u64_field(&step_text, "timeout_ms").ok_or_else(|| {
            ManifestLoadError::MissingField {
                path: path.to_owned(),
                field: "timeout_ms",
            }
        })?;
        let retry_expectation = extract_string_field(&step_text, "retry_expectation")
            .unwrap_or_else(|| "none".to_owned());
        let expected_result_class = extract_string_field(&step_text, "expected_result_class")
            .map(|raw| parse_harness_status(path, &raw))
            .transpose()?
            .unwrap_or(HarnessRunStatus::Passed);
        let assertion_refs = extract_string_array(&step_text, "assertion_refs");
        let cleanup_rule = extract_string_field(&step_text, "cleanup_rule")
            .unwrap_or_else(|| "collect_artifacts_then_reset".to_owned());
        let step = ScenarioStepRef {
            step_id,
            action_kind,
            input_refs: extract_string_array(&step_text, "input_refs"),
            timeout_ms,
            retry_expectation,
            expected_result_class,
            assertion_refs,
            cleanup_rule,
        };
        step.to_contract_step().validate()?;
        steps.push(step);
    }
    Ok(steps)
}

fn parse_harness_status(
    path: &str,
    raw: &str,
) -> Result<HarnessRunStatus, ManifestLoadError> {
    match raw {
        "planned" => Ok(HarnessRunStatus::Planned),
        "running" => Ok(HarnessRunStatus::Running),
        "passed" => Ok(HarnessRunStatus::Passed),
        "failed" => Ok(HarnessRunStatus::Failed),
        "blocked" => Ok(HarnessRunStatus::Blocked),
        other => Err(ManifestLoadError::Contract(format!(
            "{path}: invalid expected_result_class: {other}"
        ))),
    }
}

fn reject_unsafe_fields(path: &str, text: &str) -> Result<(), ManifestLoadError> {
    for field in [
        "raw_key_material_present",
        "contains_raw_secret",
        "contains_private_key",
        "contains_token",
        "contains_private_payload",
        "contains_fixture_key_material",
    ] {
        if boolean_field_is_true(text, field) {
            return Err(ManifestLoadError::UnsafeField {
                path: path.to_owned(),
                field: field.to_owned(),
            });
        }
    }

    for field in [
        "raw_private_key",
        "private_key_material",
        "unknown_sensitive_field",
        "production_like_endpoint",
        "production_endpoint",
    ] {
        if text.contains(&format!("\"{field}\"")) {
            return Err(ManifestLoadError::UnsafeField {
                path: path.to_owned(),
                field: field.to_owned(),
            });
        }
    }

    if text.contains("production_like") {
        return Err(ManifestLoadError::UnsafeField {
            path: path.to_owned(),
            field: "production_like".to_owned(),
        });
    }

    Ok(())
}

fn balanced_json_delimiters(text: &str) -> bool {
    let mut braces = 0_i32;
    let mut brackets = 0_i32;
    let mut in_string = false;
    let mut escaped = false;

    for ch in text.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        if in_string && ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        match ch {
            '{' => braces += 1,
            '}' => braces -= 1,
            '[' => brackets += 1,
            ']' => brackets -= 1,
            _ => {}
        }
        if braces < 0 || brackets < 0 {
            return false;
        }
    }

    braces == 0 && brackets == 0 && !in_string
}

fn extract_string_field(text: &str, field: &str) -> Option<String> {
    let pattern = format!("\"{field}\"");
    let start = text.find(&pattern)? + pattern.len();
    let after_colon = text[start..].find(':')? + start + 1;
    let quote_start = text[after_colon..].find('"')? + after_colon + 1;
    parse_json_string_at(text, quote_start)
}

fn parse_json_string_at(text: &str, start: usize) -> Option<String> {
    let mut value = String::new();
    let mut escaped = false;
    for ch in text[start..].chars() {
        if escaped {
            value.push(match ch {
                '"' => '"',
                '\\' => '\\',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                other => other,
            });
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '"' => return Some(value),
            other => value.push(other),
        }
    }
    None
}

fn extract_u8_field(text: &str, field: &str) -> Option<u8> {
    let pattern = format!("\"{field}\"");
    let start = text.find(&pattern)? + pattern.len();
    let after_colon = text[start..].find(':')? + start + 1;
    let digits = text[after_colon..]
        .chars()
        .skip_while(|ch| ch.is_ascii_whitespace())
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    digits.parse::<u8>().ok()
}

fn extract_u64_field(text: &str, field: &str) -> Option<u64> {
    let pattern = format!("\"{field}\"");
    let start = text.find(&pattern)? + pattern.len();
    let after_colon = text[start..].find(':')? + start + 1;
    let digits = text[after_colon..]
        .chars()
        .skip_while(|ch| ch.is_ascii_whitespace())
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    digits.parse::<u64>().ok()
}

fn extract_string_array(text: &str, field: &str) -> Vec<String> {
    let pattern = format!("\"{field}\"");
    let Some(start) = text.find(&pattern).map(|index| index + pattern.len()) else {
        return Vec::new();
    };
    let Some(after_colon) = text[start..].find(':').map(|index| index + start + 1) else {
        return Vec::new();
    };
    let Some(array_start) = text[after_colon..]
        .find('[')
        .map(|index| index + after_colon + 1)
    else {
        return Vec::new();
    };

    let mut values = Vec::new();
    let mut index = array_start;
    while index < text.len() {
        let rest = &text[index..];
        let Some(ch) = rest.chars().next() else {
            break;
        };
        if ch == ']' {
            break;
        }
        if ch == '"' {
            let value_start = index + 1;
            if let Some(value) = parse_json_string_at(text, value_start) {
                index = value_start + value.len() + 1;
                values.push(value);
                continue;
            }
            break;
        }
        index += ch.len_utf8();
    }
    values
}

fn extract_object_array(text: &str, field: &str) -> Vec<String> {
    let pattern = format!("\"{field}\"");
    let Some(start) = text.find(&pattern).map(|index| index + pattern.len()) else {
        return Vec::new();
    };
    let Some(after_colon) = text[start..].find(':').map(|index| index + start + 1) else {
        return Vec::new();
    };
    let Some(array_start) = text[after_colon..]
        .find('[')
        .map(|index| index + after_colon)
    else {
        return Vec::new();
    };

    let mut objects = Vec::new();
    let mut depth = 0_i32;
    let mut object_start = None;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, ch) in text[array_start + 1..].char_indices() {
        let index = array_start + 1 + offset;
        if escaped {
            escaped = false;
            continue;
        }
        if in_string && ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        match ch {
            '{' => {
                if depth == 0 {
                    object_start = Some(index);
                }
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    if let Some(start) = object_start.take() {
                        objects.push(text[start..=index].to_owned());
                    }
                }
            }
            ']' if depth == 0 => break,
            _ => {}
        }
    }
    objects
}

fn boolean_field_is_true(text: &str, field: &str) -> bool {
    let pattern = format!("\"{field}\"");
    let Some(start) = text.find(&pattern).map(|index| index + pattern.len()) else {
        return false;
    };
    let Some(after_colon) = text[start..].find(':').map(|index| index + start + 1) else {
        return false;
    };
    text[after_colon..].trim_start().starts_with("true")
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = include_str!(
        "../../schemas/overrid_contracts/fixtures/valid/integration_harness_phase2.valid.json"
    );

    fn valid_pair() -> [(&'static str, &'static str); 1] {
        [("valid.json", VALID)]
    }

    #[test]
    fn loads_valid_manifest_from_canonical_repo_paths() {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .unwrap();
        let catalog = HarnessManifestLoader::canonical(repo_root)
            .load_catalog()
            .unwrap();
        assert!(catalog.scenario("scenario_phase0_smoke").is_some());
        assert!(catalog.scenario("scenario_blocked_dependency").is_some());
    }

    #[test]
    fn loads_valid_manifest_from_memory() {
        let catalog = HarnessManifestLoader::load_catalog_from_documents(&valid_pair()).unwrap();
        assert_eq!(catalog.scenarios[0].scenario_id, "scenario_phase0_smoke");
        assert_eq!(catalog.fixtures[0].fixture_id, "fixture_phase0_smoke");
        assert_eq!(catalog.scenarios[0].steps.len(), 2);
        assert_eq!(catalog.scenarios[0].steps[0].action_kind_str(), "cli");
        assert_eq!(catalog.scenarios[0].steps[1].action_kind_str(), "assertion");
    }

    #[test]
    fn rejects_invalid_json() {
        let error = HarnessManifestLoader::load_catalog_from_documents(&[(
            "invalid.json",
            "{\"schema_version\":\"integration-harness.v0.1\"",
        )])
        .unwrap_err();
        assert!(matches!(error, ManifestLoadError::InvalidJson { .. }));
    }

    #[test]
    fn rejects_incompatible_version() {
        let invalid = VALID.replace("integration-harness.v0.1", "integration-harness.v9.9");
        let error =
            HarnessManifestLoader::load_catalog_from_documents(&[("bad_version.json", &invalid)])
                .unwrap_err();
        assert!(matches!(
            error,
            ManifestLoadError::IncompatibleVersion { .. }
        ));
    }

    #[test]
    fn rejects_unknown_sensitive_fields() {
        let invalid = VALID.replace(
            "\"fixture_id\": \"fixture_phase0_smoke\"",
            "\"fixture_id\": \"fixture_phase0_smoke\", \"unknown_sensitive_field\": \"secret\"",
        );
        let error =
            HarnessManifestLoader::load_catalog_from_documents(&[("unsafe.json", &invalid)])
                .unwrap_err();
        assert!(matches!(error, ManifestLoadError::UnsafeField { .. }));
    }

    #[test]
    fn rejects_missing_fixture_refs() {
        let invalid = VALID.replace("fixture:phase0_smoke", "fixture:missing");
        let error = HarnessManifestLoader::load_catalog_from_documents(&[(
            "missing_fixture.json",
            &invalid,
        )])
        .unwrap_err();
        assert!(matches!(error, ManifestLoadError::MissingFixture { .. }));
    }

    #[test]
    fn rejects_duplicate_scenario_ids() {
        let error = HarnessManifestLoader::load_catalog_from_documents(&[
            ("a.json", VALID),
            ("b.json", VALID),
        ])
        .unwrap_err();
        assert!(matches!(
            error,
            ManifestLoadError::DuplicateScenarioId { .. }
        ));
    }
}
