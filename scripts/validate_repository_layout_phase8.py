#!/usr/bin/env python3
"""Validate Repository Layout Phase 8 service-contract and lifecycle gates."""

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
PHASE_PLAN = Path("docs/planning/repository_layout_phase_08_plan.md")
PHASE_PROGRESS = Path("docs/planning/repository_layout_phase_08_progress.md")
SPECS_README = Path("docs/specs/README.md")
SERVICE_TEMPLATE = Path("docs/specs/service_contract_template.md")
NEW_MODULE_CHECKLIST = Path("docs/specs/new_module_checklist.md")
CLI_README = Path("packages/cli/README.md")
PACKAGES_README = Path("packages/README.md")
CONTROL_PLANE_README = Path("services/control-plane/README.md")
NODE_AGENT_README = Path("services/node-agent/README.md")
CLI_RUNNER = Path("packages/cli/src/runner.rs")

REQUIRED_PHASE8_STATES = [
    "`service_contract_template_implemented`",
    "`new_service_checklist_defined`",
    "`module_addition_workflow_defined`",
    "`deprecation_removal_workflow_defined`",
    "`cross_document_maintenance_rules_defined`",
]

REQUIRED_PHASE8_HEADINGS = [
    "#### Service Contract Template Usage",
    "#### New-Service Checklist",
    "#### Module Addition Workflow",
    "#### Deprecation And Removal Workflow",
    "#### Cross-Document Maintenance Rules",
]

REQUIRED_LIFECYCLE_STATES = [
    "proposed",
    "scaffolded",
    "contracted",
    "wired",
    "validated",
    "accepted",
    "deprecated",
    "removed",
]

REQUIRED_ACCEPTANCE_EVIDENCE = [
    "sds_link",
    "service_catalog_link",
    "build_plan_crosswalk_link",
    "docs_specs_contract",
    "schema_entry_or_no_public_contract_reason",
    "test_target",
    "local_stack_participation",
    "module_record",
    "validation_evidence",
]

REQUIRED_DEPRECATION_EVIDENCE = [
    "replacement_or_removal_note",
    "stale_reference_scan",
    "workspace_manifest_update",
    "test_target_update",
    "local_stack_update",
    "harness_scenario_update",
    "generated_output_update",
    "docdex_reference_update",
]

REQUIRED_PHASE8_ARTIFACTS = [
    "missing_service_contract",
    "missing_test_target",
    "module_lifecycle_violation",
    "stale_layout_reference",
]

SCOPED_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    SPECS_README,
    SERVICE_TEMPLATE,
    NEW_MODULE_CHECKLIST,
    CLI_README,
    PACKAGES_README,
    CONTROL_PLANE_README,
    NODE_AGENT_README,
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


def validate_phase8_source_docs() -> None:
    sub_plan = read(SUB_PLAN)
    phase_8 = section(
        sub_plan,
        "## Phase 8: Service Contract Templates And New-Module Checklist",
    )

    for item in range(1, 6):
        assert_contains(phase_8, f"**8.{item} ", SUB_PLAN)

    for work_item in re.finditer(
        r"- \*\*8\.[1-5] .+?(?=\n- \*\*8\.|\n### Phase 8 Gate Outputs)",
        phase_8,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                raise AssertionError(f"{item_text.splitlines()[0]} is missing {field}")

    for heading in REQUIRED_PHASE8_HEADINGS:
        assert_contains(phase_8, heading, SUB_PLAN)
    for state in REQUIRED_PHASE8_STATES:
        assert_contains(phase_8, state, SUB_PLAN)
    for lifecycle_state in REQUIRED_LIFECYCLE_STATES:
        assert_contains(phase_8, f"`{lifecycle_state}`", SUB_PLAN)
    for artifact in REQUIRED_PHASE8_ARTIFACTS:
        assert_contains(phase_8, f"`{artifact}`", SUB_PLAN)

    sds = read(SDS)
    assert_contains(sds, "## Phase 8 Service Contract And Lifecycle Decisions", SDS)
    for state in REQUIRED_PHASE8_STATES:
        assert_contains(sds, state, SDS)

    service = read(SERVICE)
    assert_contains(service, "## Phase 8 Implementation Gates", SERVICE)
    for state in [*REQUIRED_PHASE8_STATES, "`phase8_validation_defined`"]:
        assert_contains(service, state, SERVICE)
    assert_contains(service, "`scripts/validate_repository_layout_phase8.py`", SERVICE)

    tech_stack = read(TECH_STACK)
    for expected in (
        "Rust-first infrastructure stack",
        "Cargo workspace",
        "Canonical JSON plus JSON Schema",
        "Generated Rust SDK first",
        "not as the core runtime",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)


def validate_specs_and_checklist() -> None:
    specs = read(SPECS_README)
    for expected in (
        "[Service contract template](service_contract_template.md)",
        "[New module checklist](new_module_checklist.md)",
        "New modules must follow the lifecycle",
    ):
        assert_contains(specs, expected, SPECS_README)

    template = read(SERVICE_TEMPLATE)
    for expected in (
        "Downstream Dependencies",
        "Usage Notes",
        "new_module_checklist.md",
        "Do not treat generated Rust, TypeScript declarations, examples, or fixture output as the contract source of truth.",
    ):
        assert_contains(template, expected, SERVICE_TEMPLATE)

    checklist = read(NEW_MODULE_CHECKLIST)
    for expected in (
        "Required Lifecycle States",
        "Addition Checklist",
        "Deprecation And Removal Checklist",
        "Cross-Document Maintenance",
        "`proposed`",
        "`scaffolded`",
        "`contracted`",
        "`wired`",
        "`validated`",
        "`accepted`",
        "`deprecated`",
        "`removed`",
        "service_catalog_alignment",
        "overrid.workspace.toml",
    ):
        assert_contains(checklist, expected, NEW_MODULE_CHECKLIST)


def validate_manifest_lifecycle() -> None:
    manifest = load_toml(MANIFEST)

    manifest_version = manifest.get("manifest_version")
    manifest_match = (
        re.search(r"repository-layout-phase-(\d+)", manifest_version)
        if isinstance(manifest_version, str)
        else None
    )
    if manifest_match is None or int(manifest_match.group(1)) < 8:
        raise AssertionError(f"{MANIFEST} manifest_version must identify phase 8 or later")

    layout_phase = manifest.get("validation_metadata", {}).get("layout_phase")
    if not isinstance(layout_phase, int) or layout_phase < 8:
        raise AssertionError(f"{MANIFEST} validation_metadata.layout_phase must be at least 8")

    schema = manifest.get("module_record_schema", {})
    list_contains_all(
        schema.get("accepted_lifecycle_states"),
        REQUIRED_LIFECYCLE_STATES,
        "module_record_schema.accepted_lifecycle_states",
    )

    lifecycle = manifest.get("module_lifecycle")
    if not isinstance(lifecycle, dict):
        raise AssertionError(f"{MANIFEST} must define [module_lifecycle]")
    if lifecycle.get("phase_gate") != "repository-layout-phase-8":
        raise AssertionError("module_lifecycle.phase_gate is wrong")
    list_contains_all(
        lifecycle.get("states"),
        [state.strip("`") for state in REQUIRED_PHASE8_STATES],
        "module_lifecycle.states",
    )
    list_contains_all(
        lifecycle.get("addition_states"),
        REQUIRED_LIFECYCLE_STATES[:6],
        "module_lifecycle.addition_states",
    )
    list_contains_all(
        lifecycle.get("deprecation_states"),
        REQUIRED_LIFECYCLE_STATES[6:],
        "module_lifecycle.deprecation_states",
    )
    list_contains_all(
        lifecycle.get("required_acceptance_evidence"),
        REQUIRED_ACCEPTANCE_EVIDENCE,
        "module_lifecycle.required_acceptance_evidence",
    )
    list_contains_all(
        lifecycle.get("deprecation_removal_evidence"),
        REQUIRED_DEPRECATION_EVIDENCE,
        "module_lifecycle.deprecation_removal_evidence",
    )
    list_contains_all(
        lifecycle.get("violation_artifacts"),
        REQUIRED_PHASE8_ARTIFACTS,
        "module_lifecycle.violation_artifacts",
    )

    for path_field in (
        "service_contract_template",
        "new_module_checklist",
        "validation_script",
    ):
        if not path_exists(lifecycle.get(path_field)):
            raise AssertionError(f"module_lifecycle.{path_field} is missing")

    for doc_path in lifecycle.get("maintenance_documents", []):
        if not path_exists(doc_path):
            raise AssertionError(f"module_lifecycle.maintenance_documents missing {doc_path}")

    modules = [module for module in manifest.get("modules", []) if isinstance(module, dict)]
    for module in modules:
        state = module.get("lifecycle_state")
        if state not in REQUIRED_LIFECYCLE_STATES:
            raise AssertionError(f"module {module.get('name')} has invalid lifecycle_state {state}")
        if state == "accepted" and not module.get("test_targets"):
            raise AssertionError(f"accepted module {module.get('name')} needs test_targets")

    root_commands = manifest.get("root_commands", [])
    if not isinstance(root_commands, list):
        raise AssertionError(f"{MANIFEST} root_commands must be a list")
    layout_commands = [
        command
        for command in root_commands
        if isinstance(command, dict) and command.get("name") == "layout:check"
    ]
    if len(layout_commands) != 1:
        raise AssertionError(f"{MANIFEST} must define exactly one layout:check root command")
    list_contains_all(
        layout_commands[0].get("outputs"),
        REQUIRED_PHASE8_ARTIFACTS,
        "root_commands.layout:check.outputs",
    )


def validate_cli_and_readme_evidence() -> None:
    cli = read(CLI_README)
    for expected in (
        "Repository Layout` Phase 8",
        "service_contract_template_implemented",
        "new_service_checklist_defined",
        "module_addition_workflow_defined",
        "deprecation_removal_workflow_defined",
        "cross_document_maintenance_rules_defined",
        "module_lifecycle_violation",
        "stale_layout_reference",
    ):
        assert_contains(cli, expected, CLI_README)

    packages = read(PACKAGES_README)
    for expected in (
        "Repository Layout Phase 8 module lifecycle rules",
        "`proposed`",
        "`removed`",
        "docs/specs/new_module_checklist.md",
    ):
        assert_contains(packages, expected, PACKAGES_README)

    for path in (CONTROL_PLANE_README, NODE_AGENT_README):
        text = read(path)
        assert_contains(text, "Repository Layout Phase 8", path)
        assert_contains(text, "docs/specs/new_module_checklist.md", path)

    runner = read(CLI_RUNNER)
    for expected in (
        "phase8_module_lifecycle",
        "service_contract_template_usage",
        "new_module_checklist",
        "module_addition_workflow",
        "deprecation_removal_workflow",
        "cross_document_maintenance_rules",
        "push_lifecycle_state_validity",
        "push_accepted_module_validation_evidence",
        "layout_check_emits_phase8_lifecycle_records",
        "layout_check_rejects_phase8_lifecycle_violations",
        "module_lifecycle_violation",
        "stale_layout_reference",
    ):
        assert_contains(runner, expected, CLI_RUNNER)


def validate_local_planning_trail() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in (
        "Service Contract Templates And New-Module Checklist",
        "docs/specs/new_module_checklist.md",
        "packages/cli/src/runner.rs",
        "scripts/validate_repository_layout_phase8.py",
        "docdexd run-tests --repo . --target scripts/validate_repository_layout_phase8.py",
    ):
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in (
        "Loaded Docdex profile and repo memory",
        "Confirmed Phase 8 scope",
        "Validation Evidence",
    ):
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_suite_registration() -> None:
    suite = read(SUITE_VALIDATOR)
    assert_contains(
        suite,
        'Path("scripts/validate_repository_layout_phase8.py")',
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
        validate_phase8_source_docs,
        validate_specs_and_checklist,
        validate_manifest_lifecycle,
        validate_cli_and_readme_evidence,
        validate_local_planning_trail,
        validate_suite_registration,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("Repository Layout Phase 8 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
