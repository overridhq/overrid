#!/usr/bin/env python3
"""Validate Repository Layout Phase 9 foundation integration gates."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from urllib.parse import unquote, urlparse

try:
    import tomllib
except ModuleNotFoundError as exc:  # pragma: no cover - Python < 3.11 guard.
    raise SystemExit("Python 3.11+ is required for TOML validation") from exc


REPO_ROOT = Path(__file__).resolve().parents[1]

MANIFEST = Path("overrid.workspace.toml")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_005_repository_layout.md")
SDS = Path("docs/sds/foundation/repository_layout.md")
SERVICE = Path("docs/service_catalog/foundation/repository_layout.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")
PHASE_PLAN = Path("docs/planning/repository_layout_phase_09_plan.md")
PHASE_PROGRESS = Path("docs/planning/repository_layout_phase_09_progress.md")
CLI_README = Path("packages/cli/README.md")
PACKAGES_README = Path("packages/README.md")
LOCAL_INFRA_README = Path("infra/local/README.md")
INTEGRATION_TESTS_README = Path("tests/integration/README.md")
CLI_RUNNER = Path("packages/cli/src/runner.rs")

REQUIRED_PHASE9_STATES = [
    "`local_stack_discovery_metadata_defined`",
    "`harness_discovery_metadata_defined`",
    "`clean_checkout_ci_defined`",
    "`validation_evidence_defined`",
    "`validation_artifact_consumers_defined`",
]

REQUIRED_PHASE9_HEADINGS = [
    "#### Local-Stack Discovery Metadata",
    "#### Harness Discovery Metadata",
    "#### Clean-Checkout CI Behavior",
    "#### Docs And Layout Validation Evidence",
    "#### Validation Artifact Consumers",
]

REQUIRED_PHASE9_ARTIFACTS = [
    "local_stack_discovery_violation",
    "harness_discovery_violation",
    "ci_command_sequence_violation",
    "validation_evidence_missing",
    "artifact_consumer_violation",
]

LOCAL_STACK_DISCOVERY_FIELDS = [
    "service_definition_roots",
    "profile_roots",
    "local_state_roots",
    "generated_env_paths",
    "port_binding_source",
    "safe_reset_markers",
]

HARNESS_DISCOVERY_FIELDS = [
    "scenario_roots",
    "fixture_roots",
    "artifact_roots",
    "schema_refs",
    "local_stack_commands",
    "test_targets",
]

CI_COMMANDS = [
    "overrid layout:check",
    "overrid schema:check",
    "overrid docs:check",
    "overrid test",
    "overrid dev smoke",
    "overrid test integration",
]

VALIDATION_EVIDENCE = [
    "markdown_link_checks",
    "phase_heading_checks",
    "work_item_structure_checks",
    "stale_note_scans",
    "rejected_assumption_scans",
    "docdex_indexing_checks",
    "queue_progress_updates",
]

ARTIFACT_CONSUMERS = [
    "ci",
    "local_developer_commands",
    "docdex",
    "future_agents",
]

SCOPED_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    CLI_README,
    PACKAGES_README,
    LOCAL_INFRA_README,
    INTEGRATION_TESTS_README,
    PHASE_PLAN,
    PHASE_PROGRESS,
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def load_toml(path: Path) -> dict:
    with (REPO_ROOT / path).open("rb") as handle:
        return tomllib.load(handle)


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} is missing expected text: {expected}")


def section(text: str, heading: str, next_heading_prefix: str = "## ") -> str:
    marker = f"{heading}\n"
    start = text.find(marker)
    if start == -1:
        raise AssertionError(f"Missing heading: {heading}")
    body_start = start + len(marker)
    end = text.find(f"\n{next_heading_prefix}", body_start)
    if end == -1:
        end = len(text)
    return text[body_start:end]


def list_contains_all(values: object, expected: list[str], source: str) -> None:
    if not isinstance(values, list) or not all(isinstance(item, str) for item in values):
        raise AssertionError(f"{source} must be a list of strings")
    missing = [item for item in expected if item not in values]
    if missing:
        raise AssertionError(f"{source} missing {missing}")


def path_exists(value: object) -> bool:
    if not isinstance(value, str) or not value:
        return False
    path = Path(value)
    if path.is_absolute() or ".." in path.parts:
        return False
    return (REPO_ROOT / path).exists()


def validate_path_list(values: object, source: str) -> None:
    if not isinstance(values, list) or not values:
        raise AssertionError(f"{source} must be a non-empty list")
    for value in values:
        if not path_exists(value):
            raise AssertionError(f"{source} has missing path: {value}")


def validate_phase9_source_docs() -> None:
    sub_plan = read(SUB_PLAN)
    phase_9 = section(
        sub_plan,
        "## Phase 9: Foundation Integration With Local Stack, Harness, And CI",
    )

    for item in range(1, 6):
        assert_contains(phase_9, f"**9.{item} ", SUB_PLAN)

    for work_item in re.finditer(
        r"- \*\*9\.[1-5] .+?(?=\n- \*\*9\.|\n### Phase 9 Gate Outputs)",
        phase_9,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                raise AssertionError(f"{item_text.splitlines()[0]} is missing {field}")

    for heading in REQUIRED_PHASE9_HEADINGS:
        assert_contains(phase_9, heading, SUB_PLAN)
    for state in REQUIRED_PHASE9_STATES:
        assert_contains(phase_9, state, SUB_PLAN)
    for artifact in REQUIRED_PHASE9_ARTIFACTS:
        assert_contains(phase_9, f"`{artifact}`", SUB_PLAN)

    sds = read(SDS)
    assert_contains(sds, "## Phase 9 Foundation Integration Decisions", SDS)
    for state in REQUIRED_PHASE9_STATES:
        assert_contains(sds, state, SDS)
    for artifact in REQUIRED_PHASE9_ARTIFACTS:
        assert_contains(sds, artifact, SDS)

    service = read(SERVICE)
    assert_contains(service, "## Phase 9 Implementation Gates", SERVICE)
    for state in [*REQUIRED_PHASE9_STATES, "`phase9_validation_defined`"]:
        assert_contains(service, state, SERVICE)
    assert_contains(service, "`scripts/validate_repository_layout_phase9.py`", SERVICE)

    tech_stack = read(TECH_STACK)
    for expected in (
        "Rust-first infrastructure stack",
        "Cargo workspace",
        "Canonical JSON plus JSON Schema",
        "Local development",
        "not as the core runtime",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)


def validate_manifest_foundation_integration() -> None:
    manifest = load_toml(MANIFEST)

    manifest_version = manifest.get("manifest_version")
    manifest_match = (
        re.search(r"repository-layout-phase-(\d+)", manifest_version)
        if isinstance(manifest_version, str)
        else None
    )
    if manifest_match is None or int(manifest_match.group(1)) < 9:
        raise AssertionError(f"{MANIFEST} manifest_version must identify phase 9 or later")

    layout_phase = manifest.get("validation_metadata", {}).get("layout_phase")
    if not isinstance(layout_phase, int) or layout_phase < 9:
        raise AssertionError(f"{MANIFEST} validation_metadata.layout_phase must be at least 9")

    command_consumers = manifest.get("validation_metadata", {}).get("command_consumers")
    list_contains_all(command_consumers, ["dev:smoke"], "validation_metadata.command_consumers")

    registry_artifacts = manifest.get("root_command_registry", {}).get("validation_artifacts")
    list_contains_all(
        registry_artifacts,
        REQUIRED_PHASE9_ARTIFACTS,
        "root_command_registry.validation_artifacts",
    )

    integration = manifest.get("foundation_integration")
    if not isinstance(integration, dict):
        raise AssertionError(f"{MANIFEST} must define [foundation_integration]")
    if integration.get("phase_gate") != "repository-layout-phase-9":
        raise AssertionError("foundation_integration.phase_gate is wrong")

    list_contains_all(
        integration.get("states"),
        [state.strip("`") for state in REQUIRED_PHASE9_STATES],
        "foundation_integration.states",
    )
    list_contains_all(
        integration.get("local_stack_discovery_fields"),
        LOCAL_STACK_DISCOVERY_FIELDS,
        "foundation_integration.local_stack_discovery_fields",
    )
    list_contains_all(
        integration.get("harness_discovery_fields"),
        HARNESS_DISCOVERY_FIELDS,
        "foundation_integration.harness_discovery_fields",
    )
    list_contains_all(
        integration.get("clean_checkout_ci_commands"),
        CI_COMMANDS,
        "foundation_integration.clean_checkout_ci_commands",
    )
    list_contains_all(
        integration.get("clean_checkout_ci_statuses"),
        ["pass", "fail", "blocked"],
        "foundation_integration.clean_checkout_ci_statuses",
    )
    list_contains_all(
        integration.get("validation_evidence_entries"),
        VALIDATION_EVIDENCE,
        "foundation_integration.validation_evidence_entries",
    )
    list_contains_all(
        integration.get("artifact_consumers"),
        ARTIFACT_CONSUMERS,
        "foundation_integration.artifact_consumers",
    )
    list_contains_all(
        integration.get("violation_artifacts"),
        REQUIRED_PHASE9_ARTIFACTS,
        "foundation_integration.violation_artifacts",
    )

    if integration.get("artifact_consumer_boundary") != "build_ci_evidence_not_overwatch_runtime_events":
        raise AssertionError("foundation_integration.artifact_consumer_boundary is wrong")
    if not path_exists(integration.get("local_stack_port_binding_source")):
        raise AssertionError("foundation_integration.local_stack_port_binding_source is missing")
    if not path_exists(integration.get("validation_script")):
        raise AssertionError("foundation_integration.validation_script is missing")

    for field in (
        "local_stack_service_definition_roots",
        "local_stack_profile_roots",
        "local_stack_local_state_roots",
        "local_stack_generated_env_paths",
        "local_stack_safe_reset_markers",
        "harness_scenario_roots",
        "harness_fixture_roots",
        "harness_artifact_roots",
        "harness_schema_refs",
    ):
        validate_path_list(integration.get(field), f"foundation_integration.{field}")

    root_commands = manifest.get("root_commands", [])
    if not isinstance(root_commands, list):
        raise AssertionError(f"{MANIFEST} root_commands must be a list")
    by_name = {
        command.get("name"): command
        for command in root_commands
        if isinstance(command, dict) and isinstance(command.get("name"), str)
    }
    if "layout:check" not in by_name:
        raise AssertionError(f"{MANIFEST} must define layout:check root command")
    list_contains_all(
        by_name["layout:check"].get("outputs"),
        REQUIRED_PHASE9_ARTIFACTS,
        "root_commands.layout:check.outputs",
    )
    if by_name.get("dev:smoke", {}).get("canonical_invocation") != "overrid dev smoke":
        raise AssertionError(f"{MANIFEST} must define dev:smoke root command")


def validate_cli_and_readme_evidence() -> None:
    cli = read(CLI_README)
    for expected in (
        "Repository Layout` Phase 9",
        "local_stack_discovery_metadata_defined",
        "harness_discovery_metadata_defined",
        "clean_checkout_ci_defined",
        "validation_evidence_defined",
        "validation_artifact_consumers_defined",
        "local_stack_discovery_violation",
        "artifact_consumer_violation",
    ):
        assert_contains(cli, expected, CLI_README)

    packages = read(PACKAGES_README)
    for expected in (
        "Repository Layout Phase 9 foundation integration metadata",
        "local-stack discovery",
        "harness discovery",
        "clean-checkout CI sequencing",
    ):
        assert_contains(packages, expected, PACKAGES_README)

    local_infra = read(LOCAL_INFRA_README)
    for expected in (
        "Repository Layout Phase 9",
        "Local Development Stack",
        "service definitions",
        "safe reset markers",
    ):
        assert_contains(local_infra, expected, LOCAL_INFRA_README)

    tests = read(INTEGRATION_TESTS_README)
    for expected in (
        "Repository Layout Phase 9",
        "Integration Test Harness",
        "scenario roots",
        "fixture roots",
    ):
        assert_contains(tests, expected, INTEGRATION_TESTS_README)

    runner = read(CLI_RUNNER)
    for expected in (
        "phase9_foundation_integration",
        "local_stack_discovery_metadata",
        "harness_discovery_metadata",
        "clean_checkout_ci_behavior",
        "validation_evidence_model",
        "validation_artifact_consumers",
        "layout_check_emits_phase9_foundation_integration_records",
        "layout_check_rejects_phase9_foundation_integration_violations",
        "local_stack_discovery_violation",
        "harness_discovery_violation",
        "ci_command_sequence_violation",
        "validation_evidence_missing",
        "artifact_consumer_violation",
    ):
        assert_contains(runner, expected, CLI_RUNNER)


def validate_local_planning_trail() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in (
        "Foundation Integration With Local Stack, Harness, And CI",
        "overrid.workspace.toml",
        "packages/cli/src/runner.rs",
        "scripts/validate_repository_layout_phase9.py",
        "docdexd run-tests --repo . --target scripts/validate_repository_layout_phase9.py",
    ):
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in (
        "Loaded Docdex profile and repo memory",
        "Confirmed Phase 9 scope",
        "Validation Evidence",
    ):
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_suite_registration() -> None:
    suite = read(SUITE_VALIDATOR)
    assert_contains(
        suite,
        'Path("scripts/validate_repository_layout_phase9.py")',
        SUITE_VALIDATOR,
    )


def iter_markdown_links(text: str):
    pattern = re.compile(r"(?<!!)\[[^\]]+\]\(([^)]+)\)")
    for match in pattern.finditer(text):
        target = match.group(1).split("#", 1)[0].strip()
        if not target or target.startswith(("http://", "https://", "mailto:")):
            continue
        parsed = urlparse(target)
        if parsed.scheme and parsed.scheme != "file":
            continue
        yield unquote(parsed.path if parsed.scheme == "file" else target)


def validate_local_markdown_links() -> None:
    for doc in SCOPED_DOCS:
        text = read(doc)
        for target in iter_markdown_links(text):
            candidate = (REPO_ROOT / doc.parent / target).resolve()
            try:
                candidate.relative_to(REPO_ROOT.resolve())
            except ValueError as exc:
                raise AssertionError(f"{doc} link escapes repo: {target}") from exc
            if not candidate.exists():
                raise AssertionError(f"{doc} has missing link target: {target}")


def main() -> int:
    checks = [
        validate_phase9_source_docs,
        validate_manifest_foundation_integration,
        validate_cli_and_readme_evidence,
        validate_local_planning_trail,
        validate_suite_registration,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("Repository Layout Phase 9 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
