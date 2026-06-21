#!/usr/bin/env python3
"""Validate Shared Schema Package Phase 1 boundary and authority gates."""

from __future__ import annotations

from pathlib import Path
import json
import re
import sys
from urllib.parse import unquote, urlparse


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_007_shared_schema_package.md")
SDS = Path("docs/sds/foundation/shared_schema_package.md")
SERVICE = Path("docs/service_catalog/foundation/shared_schema_package.md")
MASTER = Path("docs/build_plan/master_plan.md")
CROSSWALK = Path("docs/build_plan/service_catalog_alignment.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SCHEMAS_README = Path("packages/schemas/README.md")
CONTRACTS_MANIFEST = Path("packages/schemas/overrid_contracts/codegen_manifest.json")
ADMIN_UI_MANIFEST = Path("packages/schemas/admin_ui/codegen_manifest.json")
PHASE_PLAN = Path("docs/planning/shared_schema_package_phase_01_plan.md")
PHASE_PROGRESS = Path("docs/planning/shared_schema_package_phase_01_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

MARKDOWN_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    MASTER,
    CROSSWALK,
    TECH_STACK,
    SCHEMAS_README,
]

OPTIONAL_PLANNING_DOCS = [
    PHASE_PLAN,
    PHASE_PROGRESS,
]

REQUIRED_GATE_HEADINGS = [
    "#### Link Attachment Matrix",
    "#### Frozen Package Boundary",
    "#### Master Phase Gate Matrix",
    "#### Resolved SDS Decision Checklist",
    "#### Schema Ownership Metadata Model",
    "#### Schema Review Lifecycle",
    "#### Documentation Update Rule",
]

REQUIRED_GATE_STATES = [
    "`attached`",
    "`boundary_frozen`",
    "`phase_0_authority`",
    "`downstream_phase_gated`",
    "`resolved_decision_carried`",
    "`metadata_required`",
]

REQUIRED_LIFECYCLE_STATES = [
    "`draft`",
    "`reviewed`",
    "`validated`",
    "`compatible`",
    "`released`",
    "`deprecated`",
    "`retired`",
    "`blocked`",
]

REQUIRED_BOUNDARY_TEXT = [
    "not a deployed microservice",
    "not a runtime registry",
    "not a policy engine",
    "not an audit authority",
    "not an accounting authority",
    "not a production record store",
    "Runtime authority stays with the owning Overrid services",
    "must not own runtime policy decisions",
    "production queue state",
    "production registry state",
    "production storage",
    "secret storage",
    "TypeScript core runtime authority",
]

REQUIRED_ATTACHMENT_PATHS = [
    "docs/sds/foundation/shared_schema_package.md",
    "docs/service_catalog/foundation/shared_schema_package.md",
    "docs/build_plan/master_plan.md",
    "docs/build_plan/service_catalog_alignment.md",
    "docs/overrid_tech_stack_choice.md",
    "packages/schemas/README.md",
    "docs/planning/shared_schema_package_phase_01_plan.md",
    "docs/planning/shared_schema_package_phase_01_progress.md",
]

REQUIRED_RESOLVED_DECISIONS = [
    "Canonical JSON plus JSON Schema",
    "Rust generation",
    "TypeScript/web projections",
    "Protobuf",
    "strict unknown-field rejection",
    "Extension maps",
    "Current-plus-previous stable major support",
    "formal migration plans",
]

REQUIRED_METADATA_FIELDS = [
    "owning_service_family",
    "downstream_consumers",
    "release_status",
    "privacy_class",
    "compatibility_class",
    "review_authority",
    "consumer_notes",
]

ALLOWED_RELEASE_STATES = {
    "draft",
    "reviewed",
    "validated",
    "compatible",
    "released",
    "deprecated",
    "retired",
    "blocked",
}

CONTRACTS_MANIFEST_MODULE_KEYS = [
    "local_development_stack_phase2",
    "integration_harness_phase2",
    "integration_harness_phase6",
    "integration_harness_phase7",
    "integration_harness_phase8",
    "integration_harness_phase9",
    "integration_harness_phase10",
]

ADMIN_UI_MANIFEST_COLLECTION_KEYS = [
    "additional_schemas",
    "phase_artifacts",
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def section(text: str, heading: str, next_heading_level: str = "## ") -> str:
    marker = f"{heading}\n"
    start = text.find(marker)
    if start == -1:
        raise AssertionError(f"Missing heading: {heading}")
    body_start = start + len(marker)
    end = text.find(f"\n{next_heading_level}", body_start)
    if end == -1:
        end = len(text)
    return text[body_start:end]


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} is missing expected text: {expected}")


def validate_sub_plan() -> None:
    text = read(SUB_PLAN)
    assert_contains(text, "# SUB BUILD PLAN #7 - Shared Schema Package", SUB_PLAN)
    assert_contains(
        text,
        "Attached SDS: [docs/sds/foundation/shared_schema_package.md]",
        SUB_PLAN,
    )

    phase_headings = re.findall(r"^## Phase (\d+):", text, flags=re.MULTILINE)
    if phase_headings != [str(number) for number in range(1, 11)]:
        raise AssertionError(f"{SUB_PLAN} must contain Phase 1 through Phase 10 in order")

    phase_1 = section(
        text,
        "## Phase 1: SDS Attachment, Package Boundary, And Authority Rules",
    )

    for item in range(1, 6):
        assert_contains(phase_1, f"**1.{item} ", SUB_PLAN)

    for work_item in re.finditer(
        r"- \*\*1\.[1-5] .+?(?=\n- \*\*1\.|\n### Phase 1 Gate Outputs)",
        phase_1,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")

    for heading in REQUIRED_GATE_HEADINGS:
        assert_contains(phase_1, heading, SUB_PLAN)
    for state in REQUIRED_GATE_STATES:
        assert_contains(phase_1, state, SUB_PLAN)
    for state in REQUIRED_LIFECYCLE_STATES:
        assert_contains(phase_1, state, SUB_PLAN)
    for boundary in REQUIRED_BOUNDARY_TEXT:
        assert_contains(phase_1, boundary, SUB_PLAN)
    for path in REQUIRED_ATTACHMENT_PATHS:
        assert_contains(phase_1, path, SUB_PLAN)
    for decision in REQUIRED_RESOLVED_DECISIONS:
        assert_contains(phase_1, decision, SUB_PLAN)

    for phase in range(0, 14):
        if not re.search(rf"^\| {phase} \|", phase_1, flags=re.MULTILINE):
            raise AssertionError(f"{SUB_PLAN} master phase gate matrix is missing phase {phase}")


def validate_cross_doc_alignment() -> None:
    sds = read(SDS)
    service = read(SERVICE)
    master = read(MASTER)
    crosswalk = read(CROSSWALK)
    tech_stack = read(TECH_STACK)
    schemas_readme = read(SCHEMAS_README)
    suite_validator = read(SUITE_VALIDATOR)

    assert_contains(sds, "[SUB BUILD PLAN #7 - Shared Schema Package]", SDS)
    assert_contains(sds, "## Phase-Gate Boundary Decisions", SDS)
    for state in REQUIRED_GATE_STATES:
        assert_contains(sds, state, SDS)
    for state in REQUIRED_LIFECYCLE_STATES:
        assert_contains(sds, state, SDS)
    for boundary in REQUIRED_BOUNDARY_TEXT[:6]:
        assert_contains(sds, boundary, SDS)
    for decision in REQUIRED_RESOLVED_DECISIONS:
        assert_contains(sds, decision, SDS)

    assert_contains(service, "[SUB BUILD PLAN #7 - Shared Schema Package]", SERVICE)
    assert_contains(service, "## Phase 1 Implementation Gates", SERVICE)
    for state in REQUIRED_GATE_STATES:
        assert_contains(service, state, SERVICE)
    for boundary in REQUIRED_BOUNDARY_TEXT[:6]:
        assert_contains(service, boundary, SERVICE)
    for field in REQUIRED_METADATA_FIELDS:
        assert_contains(service, field, SERVICE)

    assert_contains(master, "SDS #7: [Shared Schema Package]", MASTER)
    assert_contains(master, "[SUB BUILD PLAN #7 - Shared Schema Package]", MASTER)
    assert_contains(master, "First build point remains Phase 0", MASTER)

    assert_contains(crosswalk, "| SDS #7 | [Shared Schema Package]", CROSSWALK)
    assert_contains(crosswalk, "[SUB BUILD PLAN #7 - Shared Schema Package]", CROSSWALK)
    assert_contains(crosswalk, "[Phase 0: Foundation]", CROSSWALK)
    assert_contains(crosswalk, "downstream schema expansion gated by owning service phases", CROSSWALK)

    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)
    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)
    assert_contains(tech_stack, "TypeScript/web bindings from the same contracts", TECH_STACK)
    assert_contains(tech_stack, "Internal binary contracts", TECH_STACK)
    assert_contains(tech_stack, "Protobuf where compact", TECH_STACK)
    assert_contains(tech_stack, "Node.js/TypeScript as the core control-plane", TECH_STACK)

    assert_contains(schemas_readme, "## Schema Ownership Metadata", SCHEMAS_README)
    for field in REQUIRED_METADATA_FIELDS:
        assert_contains(schemas_readme, field, SCHEMAS_README)
    assert_contains(schemas_readme, "metadata_required", SCHEMAS_README)
    assert_contains(schemas_readme, "canonical JSON Schema authority", SCHEMAS_README)
    assert_contains(schemas_readme, "Rust-first validation", SCHEMAS_README)

    planning_docs_present = [(REPO_ROOT / path).is_file() for path in OPTIONAL_PLANNING_DOCS]
    if any(planning_docs_present):
        if not all(planning_docs_present):
            raise AssertionError(
                f"Local planning trail must include both {PHASE_PLAN} and {PHASE_PROGRESS}"
            )
        phase_plan = read(PHASE_PLAN)
        phase_progress = read(PHASE_PROGRESS)
        assert_contains(phase_plan, "Complete SUB BUILD PLAN #7 Phase 1", PHASE_PLAN)
        assert_contains(phase_plan, "must not introduce runtime registry behavior", PHASE_PLAN)
        assert_contains(phase_progress, "Shared Schema Package Phase 1 Progress", PHASE_PROGRESS)
        assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)

    assert_contains(
        suite_validator,
        'Path("scripts/validate_shared_schema_package_phase1.py")',
        SUITE_VALIDATOR,
    )


def validate_schema_ownership(ownership: object, path: Path, label: str) -> None:
    if not isinstance(ownership, dict):
        raise AssertionError(f"{path} {label} is missing schema_ownership metadata")

    for field in REQUIRED_METADATA_FIELDS:
        value = ownership.get(field)
        if value in (None, "", []):
            raise AssertionError(f"{path} {label}.schema_ownership.{field} is required")

    if ownership["release_status"] not in ALLOWED_RELEASE_STATES:
        raise AssertionError(f"{path} {label} has invalid release_status")
    if not isinstance(ownership["downstream_consumers"], list):
        raise AssertionError(f"{path} {label} downstream_consumers must be a list")


def validate_projection_metadata(entry: dict, path: Path, label: str) -> None:
    for projection_key in ("rust_projection", "typescript_projection"):
        projection = entry.get(projection_key)
        if isinstance(projection, dict):
            if projection.get("non_authoritative") is not True:
                raise AssertionError(f"{path} {label}.{projection_key} must be non-authoritative")


def validate_manifest_entry(entry: object, path: Path, label: str) -> set[str]:
    if not isinstance(entry, dict):
        raise AssertionError(f"{path} {label} must be an object")

    validate_schema_ownership(entry.get("schema_ownership"), path, label)

    covered_schemas: set[str] = set()
    canonical_schema = entry.get("canonical_schema")
    if canonical_schema is not None:
        if not isinstance(canonical_schema, str) or not canonical_schema:
            raise AssertionError(f"{path} {label}.canonical_schema must be a non-empty string")
        covered_schemas.add(canonical_schema)
        if not (REPO_ROOT / canonical_schema).is_file():
            raise AssertionError(f"{path} {label}.canonical_schema does not exist: {canonical_schema}")
        if entry.get("source_of_truth") != "json_schema":
            raise AssertionError(f"{path} {label} must keep JSON Schema as source_of_truth")
        if entry.get("rust_first_validation") is not True:
            raise AssertionError(f"{path} {label} must keep rust_first_validation true")

    validate_projection_metadata(entry, path, label)
    return covered_schemas


def validate_manifest_metadata(path: Path) -> set[str]:
    raw = read(path)
    try:
        data = json.loads(raw)
    except json.JSONDecodeError as exc:
        raise AssertionError(f"{path} must be valid JSON: {exc}") from exc

    covered_schemas = validate_manifest_entry(data, path, "top-level")

    if path == CONTRACTS_MANIFEST:
        for key in CONTRACTS_MANIFEST_MODULE_KEYS:
            if key not in data:
                raise AssertionError(f"{path} is missing expected manifest module: {key}")
            covered_schemas.update(validate_manifest_entry(data[key], path, key))

    if path == ADMIN_UI_MANIFEST:
        for collection_key in ADMIN_UI_MANIFEST_COLLECTION_KEYS:
            collection = data.get(collection_key)
            if not isinstance(collection, list) or not collection:
                raise AssertionError(f"{path} {collection_key} must be a non-empty list")
            for index, entry in enumerate(collection):
                label = f"{collection_key}[{index}]"
                covered_schemas.update(validate_manifest_entry(entry, path, label))

    return covered_schemas


def validate_schema_metadata() -> None:
    covered_schemas: set[str] = set()
    covered_schemas.update(validate_manifest_metadata(CONTRACTS_MANIFEST))
    covered_schemas.update(validate_manifest_metadata(ADMIN_UI_MANIFEST))

    schema_sources = {
        str(path.relative_to(REPO_ROOT))
        for path in (REPO_ROOT / "packages/schemas").glob("**/*.schema.json")
    }
    missing_metadata = sorted(schema_sources - covered_schemas)
    if missing_metadata:
        missing = "\n".join(f"- {path}" for path in missing_metadata)
        raise AssertionError(
            "Schema sources missing schema_ownership metadata in codegen manifests:\n"
            f"{missing}"
        )


def validate_local_markdown_links() -> None:
    link_pattern = re.compile(r"(?<!!)\[[^\]]+\]\(([^)]+)\)")
    missing: list[str] = []

    scoped_docs = [
        *MARKDOWN_DOCS,
        *[path for path in OPTIONAL_PLANNING_DOCS if (REPO_ROOT / path).is_file()],
    ]

    for path in scoped_docs:
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
        validate_sub_plan,
        validate_cross_doc_alignment,
        validate_schema_metadata,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("Shared Schema Package Phase 1 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
