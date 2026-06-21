#!/usr/bin/env python3
"""Validate Repository Layout Phase 10 alignment and downstream handoff gates."""

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
MASTER = Path("docs/build_plan/master_plan.md")
CROSSWALK = Path("docs/build_plan/service_catalog_alignment.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")
PHASE_PLAN = Path("docs/planning/repository_layout_phase_10_plan.md")
PHASE_PROGRESS = Path("docs/planning/repository_layout_phase_10_progress.md")
CLI_RUNNER = Path("packages/cli/src/runner.rs")

REQUIRED_PHASE10_STATES = [
    "`sub_build_plan_structure_validated`",
    "`tech_stack_alignment_validated`",
    "`master_plan_alignment_validated`",
    "`service_catalog_sds_alignment_validated`",
    "`downstream_phase_handoff_defined`",
]

REQUIRED_PHASE10_HEADINGS = [
    "#### Sub-Build-Plan Structure Validation",
    "#### Tech-Stack Alignment Validation",
    "#### Master-Plan Alignment Validation",
    "#### Service-Catalog And SDS Alignment",
    "#### Downstream Phase Handoff",
]

REQUIRED_PHASE10_ARTIFACTS = [
    "sub_build_plan_structure_violation",
    "tech_stack_alignment_violation",
    "master_plan_alignment_violation",
    "source_document_alignment_violation",
    "downstream_handoff_violation",
]

STRUCTURE_CHECKS = [
    "title_prefix",
    "attached_sds_link",
    "phase_headings_1_through_10",
    "work_item_design_output_validation_fields",
    "exit_gate_present",
    "local_markdown_links",
]

TECH_STACK_ALIGNMENT_CHECKS = [
    "rust_first_workspace",
    "language_neutral_schema_authority",
    "overrid_shaped_local_primitives",
    "modular_control_plane_through_phase_3",
    "no_conventional_cloud_runtime_authority",
    "no_blockchain_or_pricing_assumptions",
]

MASTER_PLAN_ALIGNMENT_CHECKS = [
    "phase_0_through_13_order_unchanged",
    "sds_5_phase_0_foundation_alignment",
    "service_catalog_crosswalk_phase_0_alignment",
    "later_phase_consumer_rules_preserved",
]

SOURCE_ALIGNMENT_DOCUMENTS = [
    "docs/sds/foundation/repository_layout.md",
    "docs/service_catalog/foundation/repository_layout.md",
    "docs/build_plan/sub_build_plan_005_repository_layout.md",
    "docs/build_plan/master_plan.md",
    "docs/build_plan/service_catalog_alignment.md",
    "docs/overrid_tech_stack_choice.md",
]

PHASE_PLANNING_DOCUMENTS = [
    "docs/planning/repository_layout_phase_10_plan.md",
    "docs/planning/repository_layout_phase_10_progress.md",
]

DOWNSTREAM_RULES = [
    "phase_1_control_plane_modules",
    "phase_2_node_agent_modules",
    "phase_3_execution_loop_modules",
    "phase_4_trust_policy_modules",
    "phase_5_accounting_modules",
    "phase_6_product_integration_clients",
    "phase_7_grid_resident_services",
    "phase_8_storage_namespace_primitives",
    "phase_9_deployment_platform_modules",
    "phase_10_federation_public_interest_modules",
    "phase_11_public_provider_modules",
    "phase_12_native_app_clients",
    "phase_13_governance_hardening_modules",
]

MASTER_PHASE_NAMES = [
    "Foundation",
    "Control-Plane Skeleton",
    "Seed Private Swarm",
    "Private Execution Loop",
    "Trust, Policy, and Verification",
    "Metering, ORU, Seal Ledger, and Overbill",
    "First Product Integration",
    "Grid-Resident Backbone",
    "Data, Storage, and Namespace Platform",
    "Overpack Deployment Platform",
    "Trusted Federation and Public-Interest Pools",
    "Limited Public Low-Sensitivity Pool",
    "Native Application Layer",
    "Governance, Compliance, and Scale Hardening",
]

FORBIDDEN_ASSUMPTIONS = [
    "PostgreSQL",
    "Redis",
    "S3",
    "MinIO",
    "NATS",
    "Kafka",
    "Vault",
    "blockchain",
    "NFT",
    "pricing",
    "revenue",
    "customer-count",
]

REJECTION_WORDS = (
    "no ",
    "no_",
    "not ",
    "without",
    "reject",
    "free of",
    "avoid",
    "must not",
    "non-choice",
    "non-choices",
    "instead of",
    "rather than",
    "not as",
    "never",
    "cannot",
)

SCOPED_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    MASTER,
    CROSSWALK,
    TECH_STACK,
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


def validate_phase10_source_docs() -> None:
    sub_plan = read(SUB_PLAN)
    if not sub_plan.startswith("# SUB BUILD PLAN #5 - Repository Layout\n"):
        raise AssertionError(f"{SUB_PLAN} title prefix is wrong")
    assert_contains(
        sub_plan,
        "Attached SDS: [docs/sds/foundation/repository_layout.md](../sds/foundation/repository_layout.md)",
        SUB_PLAN,
    )
    for phase in range(1, 11):
        assert_contains(sub_plan, f"## Phase {phase}:", SUB_PLAN)
    assert_contains(sub_plan, "## Alignment Review", SUB_PLAN)
    assert_contains(sub_plan, "## Exit Gate", SUB_PLAN)

    phase_10 = section(
        sub_plan,
        "## Phase 10: Validation, Documentation Alignment, And Downstream Handoff",
    )
    for item in range(1, 6):
        assert_contains(phase_10, f"**10.{item} ", SUB_PLAN)
    for work_item in re.finditer(
        r"- \*\*10\.[1-5] .+?(?=\n- \*\*10\.|\n### Phase 10 Gate Outputs)",
        phase_10,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                raise AssertionError(f"{item_text.splitlines()[0]} is missing {field}")

    for heading in REQUIRED_PHASE10_HEADINGS:
        assert_contains(phase_10, heading, SUB_PLAN)
    for state in REQUIRED_PHASE10_STATES:
        assert_contains(phase_10, state, SUB_PLAN)
    for artifact in REQUIRED_PHASE10_ARTIFACTS:
        assert_contains(phase_10, f"`{artifact}`", SUB_PLAN)

    sds = read(SDS)
    assert_contains(sds, "## Phase 10 Alignment And Handoff Decisions", SDS)
    for state in REQUIRED_PHASE10_STATES:
        assert_contains(sds, state, SDS)
    for artifact in REQUIRED_PHASE10_ARTIFACTS:
        assert_contains(sds, artifact, SDS)
    assert_contains(sds, "existing_layout_with_sds_backed_expansion_no_top_level_sprawl", SDS)

    service = read(SERVICE)
    assert_contains(service, "## Phase 10 Implementation Gates", SERVICE)
    for state in [*REQUIRED_PHASE10_STATES, "`phase10_validation_defined`"]:
        assert_contains(service, state, SERVICE)
    assert_contains(service, "`scripts/validate_repository_layout_phase10.py`", SERVICE)

    tech_stack = read(TECH_STACK)
    for expected in (
        "Rust-first infrastructure stack",
        "Cargo workspace",
        "Canonical JSON plus JSON Schema",
        "Overrid-shaped local stubs or embedded engines",
        "not as the core runtime",
        "Explicit Non-Choices",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)


def validate_manifest_alignment_handoff() -> None:
    manifest = load_toml(MANIFEST)

    manifest_version = manifest.get("manifest_version")
    manifest_match = (
        re.search(r"repository-layout-phase-(\d+)", manifest_version)
        if isinstance(manifest_version, str)
        else None
    )
    if manifest_match is None or int(manifest_match.group(1)) < 10:
        raise AssertionError(f"{MANIFEST} manifest_version must identify phase 10 or later")

    layout_phase = manifest.get("validation_metadata", {}).get("layout_phase")
    if not isinstance(layout_phase, int) or layout_phase < 10:
        raise AssertionError(f"{MANIFEST} validation_metadata.layout_phase must be at least 10")

    registry_artifacts = manifest.get("root_command_registry", {}).get("validation_artifacts")
    list_contains_all(
        registry_artifacts,
        REQUIRED_PHASE10_ARTIFACTS,
        "root_command_registry.validation_artifacts",
    )

    alignment = manifest.get("alignment_handoff")
    if not isinstance(alignment, dict):
        raise AssertionError(f"{MANIFEST} must define [alignment_handoff]")
    if alignment.get("phase_gate") != "repository-layout-phase-10":
        raise AssertionError("alignment_handoff.phase_gate is wrong")
    if (
        alignment.get("downstream_handoff_boundary")
        != "existing_layout_with_sds_backed_expansion_no_top_level_sprawl"
    ):
        raise AssertionError("alignment_handoff.downstream_handoff_boundary is wrong")

    list_contains_all(
        alignment.get("states"),
        [state.strip("`") for state in REQUIRED_PHASE10_STATES],
        "alignment_handoff.states",
    )
    list_contains_all(
        alignment.get("structure_checks"),
        STRUCTURE_CHECKS,
        "alignment_handoff.structure_checks",
    )
    list_contains_all(
        alignment.get("tech_stack_alignment_checks"),
        TECH_STACK_ALIGNMENT_CHECKS,
        "alignment_handoff.tech_stack_alignment_checks",
    )
    list_contains_all(
        alignment.get("master_plan_alignment_checks"),
        MASTER_PLAN_ALIGNMENT_CHECKS,
        "alignment_handoff.master_plan_alignment_checks",
    )
    list_contains_all(
        alignment.get("source_alignment_documents"),
        SOURCE_ALIGNMENT_DOCUMENTS,
        "alignment_handoff.source_alignment_documents",
    )
    list_contains_all(
        alignment.get("phase_planning_documents"),
        PHASE_PLANNING_DOCUMENTS,
        "alignment_handoff.phase_planning_documents",
    )
    list_contains_all(
        alignment.get("downstream_phase_handoff_rules"),
        DOWNSTREAM_RULES,
        "alignment_handoff.downstream_phase_handoff_rules",
    )
    list_contains_all(
        alignment.get("violation_artifacts"),
        REQUIRED_PHASE10_ARTIFACTS,
        "alignment_handoff.violation_artifacts",
    )
    validate_path_list(alignment.get("source_alignment_documents"), "alignment_handoff.source_alignment_documents")
    validate_path_list(alignment.get("phase_planning_documents"), "alignment_handoff.phase_planning_documents")
    if not path_exists(alignment.get("validation_script")):
        raise AssertionError("alignment_handoff.validation_script is missing")

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
        REQUIRED_PHASE10_ARTIFACTS,
        "root_commands.layout:check.outputs",
    )


def validate_master_plan_alignment() -> None:
    master = read(MASTER)
    crosswalk = read(CROSSWALK)

    for index, name in enumerate(MASTER_PHASE_NAMES):
        assert_contains(master, f"| {index} |", MASTER)
        assert_contains(master, name, MASTER)
    assert_contains(master, "Phase 0 through Phase 13", MASTER)
    assert_contains(
        master,
        "| SDS #5: [Repository Layout](../sds/foundation/repository_layout.md) | [SUB BUILD PLAN #5 - Repository Layout](sub_build_plan_005_repository_layout.md) | First build point remains Phase 0",
        MASTER,
    )

    assert_contains(crosswalk, "Phase 0 through Phase 13 names must match", CROSSWALK)
    assert_contains(
        crosswalk,
        "| SDS #5 | [Repository Layout](../service_catalog/foundation/repository_layout.md) | [SUB BUILD PLAN #5 - Repository Layout](sub_build_plan_005_repository_layout.md) | [Phase 0: Foundation](phase_00_foundation.md), with later workspace expansion gated by owning service phases and layout-change governance |",
        CROSSWALK,
    )


def validate_cli_evidence() -> None:
    runner = read(CLI_RUNNER)
    for expected in (
        "phase10_alignment_handoff",
        "sub_build_plan_structure_checks",
        "tech_stack_alignment_checks",
        "master_plan_alignment_checks",
        "source_alignment_documents",
        "phase10_planning_documents",
        "downstream_phase_handoff_rules",
        "downstream_handoff_boundary",
        "phase10_validation_script",
        "layout_check_emits_phase10_alignment_handoff_records",
        "layout_check_rejects_phase10_alignment_handoff_violations",
        "sub_build_plan_structure_violation",
        "tech_stack_alignment_violation",
        "master_plan_alignment_violation",
        "source_document_alignment_violation",
        "downstream_handoff_violation",
    ):
        assert_contains(runner, expected, CLI_RUNNER)


def validate_local_planning_trail() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in (
        "Validation, Documentation Alignment, And Downstream Handoff",
        "overrid.workspace.toml",
        "packages/cli/src/runner.rs",
        "scripts/validate_repository_layout_phase10.py",
        "python3 scripts/validate_repository_layout_phase10.py",
        "docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid --target scripts/validate_repository_layout_phase10.py",
    ):
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in (
        "Loaded Docdex profile and repo memory",
        "Confirmed Repository Layout Phase 10 scope",
        "Validation Evidence",
    ):
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_suite_registration() -> None:
    suite = read(SUITE_VALIDATOR)
    assert_contains(
        suite,
        'Path("scripts/validate_repository_layout_phase10.py")',
        SUITE_VALIDATOR,
    )


def validate_rejected_assumption_scans() -> None:
    scoped_sections = {
        SUB_PLAN: section(
            read(SUB_PLAN),
            "## Phase 10: Validation, Documentation Alignment, And Downstream Handoff",
        ),
        SDS: section(read(SDS), "## Phase 10 Alignment And Handoff Decisions"),
        SERVICE: section(read(SERVICE), "## Phase 10 Implementation Gates"),
        PHASE_PLAN: read(PHASE_PLAN),
        PHASE_PROGRESS: read(PHASE_PROGRESS),
        MANIFEST: read(MANIFEST),
    }
    for path, text in scoped_sections.items():
        for line_number, line in enumerate(text.splitlines(), start=1):
            lowered = line.lower()
            found = [term for term in FORBIDDEN_ASSUMPTIONS if term.lower() in lowered]
            if not found:
                continue
            if any(word in lowered for word in REJECTION_WORDS):
                continue
            raise AssertionError(
                f"{path} line {line_number} contains unqualified assumption terms: {found}"
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
        validate_phase10_source_docs,
        validate_manifest_alignment_handoff,
        validate_master_plan_alignment,
        validate_cli_evidence,
        validate_local_planning_trail,
        validate_suite_registration,
        validate_rejected_assumption_scans,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("Repository Layout Phase 10 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
