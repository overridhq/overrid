#!/usr/bin/env python3
"""Validate Repository Layout Phase 6 package-boundary and control-plane gates."""

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
PHASE_PLAN = Path("docs/planning/repository_layout_phase_06_plan.md")
PHASE_PROGRESS = Path("docs/planning/repository_layout_phase_06_progress.md")
PACKAGES_README = Path("packages/README.md")
CLI_README = Path("packages/cli/README.md")
CLI_RUNNER = Path("packages/cli/src/runner.rs")
CONTROL_PLANE_README = Path("services/control-plane/README.md")
NODE_AGENT_README = Path("services/node-agent/README.md")

REQUIRED_PHASE6_STATES = [
    "`dependency_direction_groups_defined`",
    "`shared_schema_dependency_paths_enforced`",
    "`modular_control_plane_shape_preserved`",
    "`split_review_criteria_defined`",
    "`local_test_only_separation_enforced`",
]

REQUIRED_PHASE6_HEADINGS = [
    "#### Dependency Direction Groups",
    "#### Shared-Schema Dependency Enforcement",
    "#### Modular Control-Plane Shape",
    "#### Split-Review Criteria",
    "#### Local/Test-Only Separation",
]

REQUIRED_GROUPS = [
    "contracts",
    "sdk",
    "cli",
    "local_stack",
    "integration_harness",
    "admin_ui_shell",
    "docs",
    "local_infra",
    "control_plane_modules",
    "node_agent_modules",
    "docs_specs",
]

REQUIRED_PHASE6_ARTIFACTS = [
    "package_boundary_violation",
    "schema_ref_missing",
    "premature_service_split",
    "split_review_missing",
    "local_test_boundary_violation",
]

REQUIRED_SPLIT_EVIDENCE = [
    "api_load",
    "failure_isolation",
    "security_boundary",
    "operational_pressure",
    "grid_resident_backbone_pressure",
]

RUST_PACKAGE_MANIFESTS = {
    "contracts": Path("packages/schemas/overrid_contracts/Cargo.toml"),
    "sdk": Path("packages/sdk/Cargo.toml"),
    "local_stack": Path("packages/local_stack/Cargo.toml"),
    "integration_harness": Path("packages/integration_harness/Cargo.toml"),
    "cli": Path("packages/cli/Cargo.toml"),
}

EXPECTED_INTERNAL_DEPS = {
    "contracts": set(),
    "sdk": {"overrid-contracts"},
    "local_stack": {"overrid-contracts"},
    "integration_harness": {"overrid-contracts"},
    "cli": {
        "overrid-contracts",
        "overrid-integration-harness",
        "overrid-local-stack",
        "overrid-sdk",
    },
}

SCOPED_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    PHASE_PLAN,
    PHASE_PROGRESS,
    PACKAGES_README,
    CLI_README,
    CONTROL_PLANE_README,
    NODE_AGENT_README,
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


def internal_dependencies(cargo_manifest: Path) -> set[str]:
    cargo = load_toml(cargo_manifest)
    dependencies = cargo.get("dependencies", {})
    if not isinstance(dependencies, dict):
        return set()
    return {name for name in dependencies if name.startswith("overrid-")}


def validate_phase6_source_docs() -> None:
    sub_plan = read(SUB_PLAN)
    phase_6 = section(
        sub_plan,
        "## Phase 6: Package Boundary Enforcement And Modular Control-Plane Shape",
    )

    for item in range(1, 6):
        assert_contains(phase_6, f"**6.{item} ", SUB_PLAN)

    for work_item in re.finditer(
        r"- \*\*6\.[1-5] .+?(?=\n- \*\*6\.|\n### Phase 6 Gate Outputs)",
        phase_6,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                raise AssertionError(f"{item_text.splitlines()[0]} is missing {field}")

    for heading in REQUIRED_PHASE6_HEADINGS:
        assert_contains(phase_6, heading, SUB_PLAN)
    for state in REQUIRED_PHASE6_STATES:
        assert_contains(phase_6, state, SUB_PLAN)
    for group in REQUIRED_GROUPS:
        assert_contains(phase_6, f"`{group}`", SUB_PLAN)
    for artifact in REQUIRED_PHASE6_ARTIFACTS:
        assert_contains(phase_6, f"`{artifact}`", SUB_PLAN)

    sds = read(SDS)
    assert_contains(sds, "## Phase 6 Package Boundary And Control-Plane Decisions", SDS)
    for state in REQUIRED_PHASE6_STATES:
        assert_contains(sds, state, SDS)

    service = read(SERVICE)
    assert_contains(service, "## Phase 6 Implementation Gates", SERVICE)
    for state in [*REQUIRED_PHASE6_STATES, "`phase6_validation_defined`"]:
        assert_contains(service, state, SERVICE)
    assert_contains(service, "`scripts/validate_repository_layout_phase6.py`", SERVICE)

    tech_stack = read(TECH_STACK)
    for expected in (
        "Rust-first infrastructure stack",
        "JSON Schema",
        "Overrid services and protocols",
        "not as the core runtime",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)


def validate_manifest_boundaries() -> None:
    manifest = load_toml(MANIFEST)

    manifest_version = manifest.get("manifest_version")
    manifest_match = (
        re.search(r"repository-layout-phase-(\d+)", manifest_version)
        if isinstance(manifest_version, str)
        else None
    )
    if manifest_match is None or int(manifest_match.group(1)) < 6:
        raise AssertionError(f"{MANIFEST} manifest_version must identify phase 6 or later")

    validation_metadata = manifest.get("validation_metadata", {})
    layout_phase = validation_metadata.get("layout_phase")
    if not isinstance(layout_phase, int) or layout_phase < 6:
        raise AssertionError(f"{MANIFEST} validation_metadata.layout_phase must be at least 6")

    accepted_groups = manifest.get("module_record_schema", {}).get(
        "accepted_dependency_groups"
    )
    list_contains_all(accepted_groups, REQUIRED_GROUPS, "accepted_dependency_groups")

    boundary = manifest.get("package_boundary_enforcement")
    if not isinstance(boundary, dict):
        raise AssertionError(f"{MANIFEST} must define [package_boundary_enforcement]")

    if boundary.get("phase_gate") != "repository-layout-phase-6":
        raise AssertionError("package_boundary_enforcement.phase_gate is wrong")
    list_contains_all(boundary.get("states"), [state.strip("`") for state in REQUIRED_PHASE6_STATES], "package_boundary_enforcement.states")
    list_contains_all(
        boundary.get("dependency_direction_groups"),
        REQUIRED_GROUPS,
        "package_boundary_enforcement.dependency_direction_groups",
    )
    list_contains_all(
        boundary.get("violation_artifacts"),
        REQUIRED_PHASE6_ARTIFACTS,
        "package_boundary_enforcement.violation_artifacts",
    )
    list_contains_all(
        boundary.get("split_review_required_evidence"),
        REQUIRED_SPLIT_EVIDENCE,
        "package_boundary_enforcement.split_review_required_evidence",
    )
    list_contains_all(
        boundary.get("runtime_forbidden_dependency_groups"),
        ["integration_harness", "local_stack", "local_infra", "admin_ui_shell", "docs"],
        "package_boundary_enforcement.runtime_forbidden_dependency_groups",
    )

    for path_field in (
        "shared_schema_dependency_source",
        "rust_contract_projection",
        "schema_ref_contract",
        "control_plane_path",
        "node_agent_path",
        "validation_script",
    ):
        if not path_exists(boundary.get(path_field)):
            raise AssertionError(f"package_boundary_enforcement.{path_field} is missing")

    if (
        boundary.get("control_plane_default_shape")
        != "single_modular_rust_process_through_phase_3"
    ):
        raise AssertionError("control_plane_default_shape must preserve Phase 3 shape")
    if boundary.get("split_review_first_allowed_phase") != 4:
        raise AssertionError("split_review_first_allowed_phase must be 4")
    if boundary.get("grid_resident_split_review_phase") != 7:
        raise AssertionError("grid_resident_split_review_phase must be 7")

    registry = manifest.get("root_command_registry", {})
    list_contains_all(
        registry.get("validation_artifacts"),
        REQUIRED_PHASE6_ARTIFACTS,
        "root_command_registry.validation_artifacts",
    )
    layout_commands = [
        command
        for command in manifest.get("root_commands", [])
        if isinstance(command, dict) and command.get("name") == "layout:check"
    ]
    if len(layout_commands) != 1:
        raise AssertionError(f"{MANIFEST} must contain exactly one layout:check command")
    list_contains_all(
        layout_commands[0].get("outputs"),
        REQUIRED_PHASE6_ARTIFACTS,
        "layout:check.outputs",
    )


def validate_cargo_dependency_direction() -> None:
    for group, manifest_path in RUST_PACKAGE_MANIFESTS.items():
        actual = internal_dependencies(manifest_path)
        expected = EXPECTED_INTERNAL_DEPS[group]
        if actual != expected:
            raise AssertionError(
                f"{manifest_path} internal dependency mismatch: {sorted(actual)} != {sorted(expected)}"
            )

    for service_root in (REPO_ROOT / "services/control-plane", REPO_ROOT / "services/node-agent"):
        nested_cargo = [path for path in service_root.rglob("Cargo.toml")]
        if nested_cargo:
            rendered = ", ".join(str(path.relative_to(REPO_ROOT)) for path in nested_cargo)
            raise AssertionError(
                f"Phase 6 must not create premature deployable service manifests: {rendered}"
            )


def validate_readme_boundary_evidence() -> None:
    packages = read(PACKAGES_README)
    for expected in (
        "Repository Layout Phase 6 dependency direction groups",
        "`contracts`",
        "`local_stack`",
        "`integration_harness`",
        "`local_infra`",
        "Runtime-facing modules must not import local/test-only helpers",
    ):
        assert_contains(packages, expected, PACKAGES_README)

    cli = read(CLI_README)
    for expected in (
        "Repository Layout` Phase 6",
        "dependency_direction_groups_defined",
        "shared_schema_dependency_paths_enforced",
        "modular_control_plane_shape_preserved",
        "split_review_criteria_defined",
        "local_test_only_separation_enforced",
        "schema_ref_missing",
        "premature_service_split",
        "local_test_boundary_violation",
    ):
        assert_contains(cli, expected, CLI_README)

    control_plane = read(CONTROL_PLANE_README)
    for expected in (
        "one modular Rust process through master Phase 3",
        "Future splits require measured API-load",
        "Runtime-facing control-plane code must not depend on integration harness internals",
    ):
        assert_contains(control_plane, expected, CONTROL_PLANE_README)

    node_agent = read(NODE_AGENT_README)
    for expected in (
        "node_agent_modules",
        "They may consume contracts",
        "Any future node-agent service split requires measured pressure",
    ):
        assert_contains(node_agent, expected, NODE_AGENT_README)


def validate_cli_wiring() -> None:
    runner = read(CLI_RUNNER)
    for expected in (
        "phase6_boundary_enforcement",
        "shared_schema_dependency_path",
        "modular_control_plane_shape",
        "split_review_criteria",
        "local_test_only_separation",
        "layout_check_emits_phase6_boundary_records",
        "layout_check_rejects_real_phase6_boundary_violations",
        "dependency_direction_group",
        "service_contract_root_not_deployable",
        "internal_dependency_names",
        "first_cargo_manifest_under",
        "single_modular_rust_process_through_phase_3",
        "runtime_forbidden_dependency_groups",
    ):
        assert_contains(runner, expected, CLI_RUNNER)
    for artifact in REQUIRED_PHASE6_ARTIFACTS:
        assert_contains(runner, artifact, CLI_RUNNER)


def validate_local_planning_trail() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in (
        "Package Boundary Enforcement",
        "packages/cli/src/runner.rs",
        "scripts/validate_repository_layout_phase6.py",
        "docdexd run-tests --repo . --target scripts/validate_repository_layout_phase6.py",
    ):
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in (
        "Loaded Docdex profile and repo memory",
        "Confirmed Phase 6 scope",
        "Validation Evidence",
    ):
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_suite_registration() -> None:
    suite = read(SUITE_VALIDATOR)
    assert_contains(
        suite,
        'Path("scripts/validate_repository_layout_phase6.py")',
        SUITE_VALIDATOR,
    )


def iter_markdown_links(text: str):
    pattern = re.compile(r"(?<!\!)\[[^\]]+\]\(([^)]+)\)")
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
        validate_phase6_source_docs,
        validate_manifest_boundaries,
        validate_cargo_dependency_direction,
        validate_readme_boundary_evidence,
        validate_cli_wiring,
        validate_local_planning_trail,
        validate_suite_registration,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("Repository Layout Phase 6 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
