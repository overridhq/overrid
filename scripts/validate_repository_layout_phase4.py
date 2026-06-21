#!/usr/bin/env python3
"""Validate Repository Layout Phase 4 contract authority gates."""

from __future__ import annotations

from pathlib import Path
import json
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
PHASE_PLAN = Path("docs/planning/repository_layout_phase_04_plan.md")
PHASE_PROGRESS = Path("docs/planning/repository_layout_phase_04_progress.md")
SPECS_README = Path("docs/specs/README.md")
SCHEMA_README = Path("packages/schemas/README.md")
CONTRACT_AUTHORITY = Path("docs/specs/contract_authority.md")
SERVICE_TEMPLATE = Path("docs/specs/service_contract_template.md")
REASON_EVENTS = Path("docs/specs/reason_codes_and_events.md")

CODEGEN_MANIFESTS = [
    Path("packages/schemas/admin_ui/codegen_manifest.json"),
    Path("packages/schemas/overrid_contracts/codegen_manifest.json"),
]

REQUIRED_PHASE4_STATES = [
    "`schema_authority_defined`",
    "`generated_binding_boundaries_defined`",
    "`protobuf_placement_defined`",
    "`service_contract_template_defined`",
    "`reason_event_contracts_defined`",
]

REQUIRED_PHASE4_HEADINGS = [
    "#### Schema Authority Contract",
    "#### Generated Binding Boundary",
    "#### Optional Protobuf Placement",
    "#### Service Contract Stub Template",
    "#### Reason-Code And Event Contract Placement",
]

REQUIRED_SERVICE_CONTRACT_SECTIONS = [
    "Purpose",
    "Owned Data",
    "Public API",
    "Events Emitted",
    "Events Consumed",
    "Security Boundary",
    "Operational Checks",
    "Test Expectations",
    "Schema Refs",
    "Owning Phase",
]

REQUIRED_CONTRACT_AUTHORITY_PATHS = [
    "packages/schemas/admin_ui/v0",
    "packages/schemas/overrid_contracts/v0",
    "packages/schemas/admin_ui/codegen_manifest.json",
    "packages/schemas/overrid_contracts/codegen_manifest.json",
    "docs/specs/service_contract_template.md",
    "docs/specs/reason_codes_and_events.md",
    "docs/specs/contract_authority.md",
]

SCOPED_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    SPECS_README,
    SCHEMA_README,
    CONTRACT_AUTHORITY,
    SERVICE_TEMPLATE,
    REASON_EVENTS,
    PHASE_PLAN,
    PHASE_PROGRESS,
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def load_json(path: Path) -> object:
    with (REPO_ROOT / path).open(encoding="utf-8") as handle:
        return json.load(handle)


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


def safe_relative_path(value: object) -> Path | None:
    if not isinstance(value, str) or not value:
        return None
    path = Path(value)
    if path.is_absolute() or ".." in path.parts:
        return None
    return path


def path_exists(value: object) -> bool:
    path = safe_relative_path(value)
    return path is not None and (REPO_ROOT / path).exists()


def iter_codegen_entries(value: object):
    if isinstance(value, dict):
        if "canonical_schema" in value:
            yield value
        for nested in value.values():
            yield from iter_codegen_entries(nested)
    elif isinstance(value, list):
        for item in value:
            yield from iter_codegen_entries(item)


def iter_projection_entries(value: object):
    if isinstance(value, dict):
        for key in ("rust_projection", "typescript_projection"):
            projection = value.get(key)
            if isinstance(projection, dict):
                yield projection
        for nested in value.values():
            yield from iter_projection_entries(nested)
    elif isinstance(value, list):
        for item in value:
            yield from iter_projection_entries(item)


def validate_phase4_source_docs() -> None:
    sub_plan = read(SUB_PLAN)
    phase_4 = section(
        sub_plan,
        "## Phase 4: Contracts, Specs, And Schema Source Of Truth",
    )

    for item in range(1, 6):
        assert_contains(phase_4, f"**4.{item} ", SUB_PLAN)

    for work_item in re.finditer(
        r"- \*\*4\.[1-5] .+?(?=\n- \*\*4\.|\n### Phase 4 Gate Outputs)",
        phase_4,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                raise AssertionError(f"{item_text.splitlines()[0]} is missing {field}")

    for heading in REQUIRED_PHASE4_HEADINGS:
        assert_contains(phase_4, heading, SUB_PLAN)
    for state in REQUIRED_PHASE4_STATES:
        assert_contains(phase_4, state, SUB_PLAN)
    for path in REQUIRED_CONTRACT_AUTHORITY_PATHS:
        assert_contains(phase_4, path, SUB_PLAN)
    for section_name in REQUIRED_SERVICE_CONTRACT_SECTIONS:
        assert_contains(phase_4, section_name, SUB_PLAN)

    sds = read(SDS)
    assert_contains(sds, "## Phase 4 Contracts And Schema Authority Decisions", SDS)
    for state in REQUIRED_PHASE4_STATES:
        assert_contains(sds, state, SDS)

    service = read(SERVICE)
    assert_contains(service, "## Phase 4 Implementation Gates", SERVICE)
    for state in [*REQUIRED_PHASE4_STATES, "`phase4_validation_defined`"]:
        assert_contains(service, state, SERVICE)
    assert_contains(service, "`scripts/validate_repository_layout_phase4.py`", SERVICE)

    tech_stack = read(TECH_STACK)
    for expected in (
        "Rust-first infrastructure stack",
        "JSON Schema",
        "Protobuf",
        "Generated Rust SDK first",
        "TypeScript/web bindings from the same contracts",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)

    suite = read(SUITE_VALIDATOR)
    assert_contains(
        suite,
        'Path("scripts/validate_repository_layout_phase4.py")',
        SUITE_VALIDATOR,
    )


def validate_spec_docs() -> None:
    specs = read(SPECS_README)
    for required in (
        "[Contract authority](contract_authority.md)",
        "[Service contract template](service_contract_template.md)",
        "[Reason codes and events](reason_codes_and_events.md)",
        "Generated specs belong under `docs/specs/generated/`",
        "Protobuf specs are optional internal compact contracts",
    ):
        assert_contains(specs, required, SPECS_README)

    authority = read(CONTRACT_AUTHORITY)
    for required in (
        "## Canonical Schema Sources",
        "## Generated Binding Boundaries",
        "## Optional Protobuf Placement",
        "source-of-truth status",
        "non-authoritative status",
        "packages/schemas/admin_ui/generated",
        "packages/schemas/overrid_contracts/src/lib.rs",
    ):
        assert_contains(authority, required, CONTRACT_AUTHORITY)

    template = read(SERVICE_TEMPLATE)
    for section_name in REQUIRED_SERVICE_CONTRACT_SECTIONS:
        assert_contains(template, section_name, SERVICE_TEMPLATE)
    assert_contains(template, "Generated/projection code is not enough", SERVICE_TEMPLATE)

    reason_events = read(REASON_EVENTS)
    for required in (
        "## Placement Rules",
        "## Required Contract Families",
        "packages/schemas",
        "event envelopes",
        "audit records",
        "error shapes",
    ):
        assert_contains(reason_events, required, REASON_EVENTS)


def validate_schema_package() -> None:
    schema_readme = read(SCHEMA_README)
    for required in (
        "## Schema Naming Conventions",
        "## Generated Binding Boundaries",
        "## Optional Protobuf Placement",
        "## Reason Codes, Events, Audit, And Errors",
        "`*.schema.json`",
        "`fixtures/valid`",
        "`fixtures/invalid`",
        "`codegen_manifest.json`",
        "non-authoritative",
    ):
        assert_contains(schema_readme, required, SCHEMA_README)

    for manifest_path in CODEGEN_MANIFESTS:
        manifest = load_json(manifest_path)
        entries = list(iter_codegen_entries(manifest))
        if not entries:
            raise AssertionError(f"{manifest_path} must declare canonical schema entries")
        for entry in entries:
            if entry.get("source_of_truth") != "json_schema":
                raise AssertionError(f"{manifest_path} has non-json_schema source of truth")
            if not path_exists(entry.get("canonical_schema")):
                raise AssertionError(
                    f"{manifest_path} references missing schema: {entry.get('canonical_schema')}"
                )

        projections = list(iter_projection_entries(manifest))
        if not projections:
            raise AssertionError(f"{manifest_path} must declare generated/projection metadata")
        for projection in projections:
            if projection.get("non_authoritative") is not True:
                raise AssertionError(f"{manifest_path} projection must be non_authoritative")
            if not path_exists(projection.get("path")):
                raise AssertionError(
                    f"{manifest_path} projection path is missing: {projection.get('path')}"
                )
            if not path_exists(projection.get("source_schema")):
                raise AssertionError(
                    f"{manifest_path} projection source schema is missing: {projection.get('source_schema')}"
                )

    proto_files = list(REPO_ROOT.rglob("*.proto"))
    allowed_proto_root = REPO_ROOT / "packages/schemas/protobuf"
    for proto_file in proto_files:
        try:
            proto_file.relative_to(allowed_proto_root)
        except ValueError as exc:
            raise AssertionError(f"Protobuf file outside approved root: {proto_file}") from exc


def validate_manifest() -> None:
    manifest = load_toml(MANIFEST)
    if manifest.get("schema_version") != 1:
        raise AssertionError(f"{MANIFEST} schema_version must be 1")
    if "repository-layout-phase-4" not in str(manifest.get("manifest_version")):
        raise AssertionError(f"{MANIFEST} manifest_version must identify phase 4")
    if manifest.get("validation_metadata", {}).get("layout_phase") != 4:
        raise AssertionError(f"{MANIFEST} validation_metadata.layout_phase must equal 4")

    contract_authority = manifest.get("contract_authority")
    if not isinstance(contract_authority, dict):
        raise AssertionError(f"{MANIFEST} must define [contract_authority]")
    if contract_authority.get("source_of_truth") != "json_schema":
        raise AssertionError(f"{MANIFEST} contract_authority.source_of_truth must be json_schema")
    if contract_authority.get("optional_protobuf_requires_sds_and_spec") is not True:
        raise AssertionError(f"{MANIFEST} must gate optional Protobuf on SDS/spec evidence")

    for key in (
        "schema_source_roots",
        "fixture_roots",
        "codegen_manifests",
        "approved_generated_roots",
    ):
        values = contract_authority.get(key)
        if not isinstance(values, list) or not values:
            raise AssertionError(f"{MANIFEST} contract_authority.{key} must be a non-empty list")
        for value in values:
            if not path_exists(value):
                raise AssertionError(f"{MANIFEST} contract_authority.{key} path missing: {value}")

    for key in (
        "service_contract_template",
        "reason_event_contracts",
        "contract_authority_spec",
    ):
        if not path_exists(contract_authority.get(key)):
            raise AssertionError(f"{MANIFEST} contract_authority.{key} is missing")

    modules = manifest.get("modules", [])
    if not any(
        module.get("name") == "shared-schemas"
        and "python3 scripts/validate_repository_layout_phase4.py"
        in module.get("test_targets", [])
        for module in modules
        if isinstance(module, dict)
    ):
        raise AssertionError(f"{MANIFEST} shared-schemas module must run phase4 validation")
    if not any(
        module.get("name") == "docs-specs"
        and "python3 scripts/validate_repository_layout_phase4.py"
        in module.get("test_targets", [])
        for module in modules
        if isinstance(module, dict)
    ):
        raise AssertionError(f"{MANIFEST} docs-specs module must run phase4 validation")


def validate_local_planning_trail() -> None:
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    for expected in (
        "Complete SUB BUILD PLAN #5 Phase 4",
        "packages/schemas",
        "docs/specs",
        "scripts/validate_repository_layout_phase4.py",
    ):
        assert_contains(phase_plan, expected, PHASE_PLAN)
    for expected in (
        "Repository Layout Phase 4 Progress",
        "Validation Evidence",
        "packages/schemas",
        "docs/specs",
    ):
        assert_contains(phase_progress, expected, PHASE_PROGRESS)


def validate_local_markdown_links() -> None:
    link_pattern = re.compile(r"(?<!!)\[[^\]]+\]\(([^)]+)\)")
    missing: list[str] = []

    for path in SCOPED_DOCS:
        text = read(path)
        for raw_target in link_pattern.findall(text):
            parsed = urlparse(raw_target)
            if parsed.scheme or raw_target.startswith("#"):
                continue
            target_without_anchor = raw_target.split("#", 1)[0]
            if not target_without_anchor:
                continue
            decoded = unquote(target_without_anchor)
            target = (REPO_ROOT / path.parent / decoded).resolve()
            try:
                target.relative_to(REPO_ROOT)
            except ValueError as exc:
                raise AssertionError(f"{path} links outside repo: {raw_target}") from exc
            if not target.exists():
                missing.append(f"{path}: {raw_target}")

    if missing:
        joined = "\n".join(f"- {item}" for item in missing)
        raise AssertionError(f"Missing local Markdown link targets:\n{joined}")


def main() -> int:
    checks = [
        validate_phase4_source_docs,
        validate_spec_docs,
        validate_schema_package,
        validate_manifest,
        validate_local_planning_trail,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("Repository Layout Phase 4 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
