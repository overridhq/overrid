#!/usr/bin/env python3
"""Validate Repository Layout Phase 3 manifest and inventory contracts."""

from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass
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
CARGO_WORKSPACE = Path("Cargo.toml")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_005_repository_layout.md")
SDS = Path("docs/sds/foundation/repository_layout.md")
SERVICE = Path("docs/service_catalog/foundation/repository_layout.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")
PHASE_PLAN = Path("docs/planning/repository_layout_phase_03_plan.md")
PHASE_PROGRESS = Path("docs/planning/repository_layout_phase_03_progress.md")

REQUIRED_PHASE3_HEADINGS = [
    "#### Workspace Manifest Contract",
    "#### Module Record Schema",
    "#### Inventory Discovery Rules",
    "#### Phase And Owner Metadata",
    "#### Manifest Drift Reason Codes",
]

REQUIRED_PHASE3_STATES = [
    "`workspace_manifest_defined`",
    "`module_records_defined`",
    "`inventory_discovery_defined`",
    "`phase_owner_metadata_defined`",
    "`manifest_drift_checks_defined`",
]

REQUIRED_MODULE_FIELDS = [
    "`name`",
    "`type`",
    "`owner_layer`",
    "`path`",
    "`master_phase`",
    "`public_contract_path`",
    "`allowed_dependency_groups`",
    "`generated_output_paths`",
    "`test_targets`",
    "`local_stack_participation`",
    "`documentation_links`",
]

REQUIRED_DRIFT_CODES = [
    "missing_schema_version",
    "duplicate_module_name",
    "missing_path",
    "unknown_module_type",
    "stale_documentation_link",
    "missing_test_target",
    "forbidden_generated_path",
    "unlisted_module",
    "cargo_member_drift",
]

STANDARD_IGNORE_MARKER = "*\n!.gitignore\n"

SCOPED_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    TECH_STACK,
    Path("packages/README.md"),
    Path("docs/specs/README.md"),
]


@dataclass(frozen=True)
class ValidationFinding:
    code: str
    message: str


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


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


def load_toml(path: Path) -> dict:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required TOML file: {path}")
    with full_path.open("rb") as handle:
        return tomllib.load(handle)


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


def actual_cargo_members() -> list[str]:
    cargo = load_toml(CARGO_WORKSPACE)
    members = cargo.get("workspace", {}).get("members", [])
    if not isinstance(members, list):
        raise AssertionError("Cargo.toml workspace.members must be a list")
    return sorted(str(member) for member in members)


def actual_package_roots() -> list[str]:
    packages_root = REPO_ROOT / "packages"
    return sorted(
        str(path.relative_to(REPO_ROOT))
        for path in packages_root.iterdir()
        if path.is_dir() and not path.name.startswith(".")
    )


def has_standard_ignore_marker(path: Path) -> bool:
    marker = REPO_ROOT / path / ".gitignore"
    return marker.is_file() and marker.read_text(encoding="utf-8") == STANDARD_IGNORE_MARKER


def normalize_string_list(value: object) -> list[str] | None:
    if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
        return None
    return value


def collect_manifest_findings(
    manifest: dict,
    *,
    discovered_cargo_members: list[str] | None = None,
    discovered_package_roots: list[str] | None = None,
) -> list[ValidationFinding]:
    findings: list[ValidationFinding] = []

    def add(code: str, message: str) -> None:
        findings.append(ValidationFinding(code, message))

    if manifest.get("schema_version") != 1:
        add("missing_schema_version", "manifest schema_version must be 1")

    schema = manifest.get("module_record_schema")
    if not isinstance(schema, dict):
        add("missing_required_field", "module_record_schema is required")
        schema = {}

    required_fields = normalize_string_list(schema.get("required_fields")) or []
    accepted_types = set(normalize_string_list(schema.get("accepted_module_types")) or [])
    accepted_owner_layers = set(
        normalize_string_list(schema.get("accepted_owner_layers")) or []
    )
    accepted_dependency_groups = set(
        normalize_string_list(schema.get("accepted_dependency_groups")) or []
    )
    accepted_lifecycle_states = set(
        normalize_string_list(schema.get("accepted_lifecycle_states")) or []
    )
    accepted_local_stack = set(
        normalize_string_list(schema.get("accepted_local_stack_participation")) or []
    )

    drift_codes = set(
        normalize_string_list(manifest.get("drift_report", {}).get("reason_codes")) or []
    )
    for code in REQUIRED_DRIFT_CODES:
        if code not in drift_codes:
            add("missing_required_field", f"drift_report.reason_codes is missing {code}")

    modules = manifest.get("modules")
    if not isinstance(modules, list) or not modules:
        add("missing_module_records", "manifest must define at least one module record")
        modules = []

    seen_names: set[str] = set()
    manifest_module_paths: set[str] = set()
    manifest_cargo_members: set[str] = set()

    for index, module in enumerate(modules):
        if not isinstance(module, dict):
            add("missing_required_field", f"module #{index + 1} must be a table")
            continue

        name = module.get("name")
        label = name if isinstance(name, str) else f"module #{index + 1}"

        for field in required_fields:
            if field not in module:
                add("missing_required_field", f"{label} is missing {field}")

        if isinstance(name, str):
            if name in seen_names:
                add("duplicate_module_name", f"duplicate module name: {name}")
            seen_names.add(name)

        module_type = module.get("type")
        if module_type not in accepted_types:
            add("unknown_module_type", f"{label} has unknown type {module_type!r}")

        owner_layer = module.get("owner_layer")
        if owner_layer not in accepted_owner_layers:
            add("unknown_owner_layer", f"{label} has unknown owner_layer {owner_layer!r}")

        lifecycle_state = module.get("lifecycle_state")
        if lifecycle_state is not None and lifecycle_state not in accepted_lifecycle_states:
            add(
                "missing_required_field",
                f"{label} has unknown lifecycle_state {lifecycle_state!r}",
            )

        master_phase = module.get("master_phase")
        if not isinstance(master_phase, int) or not 0 <= master_phase <= 13:
            add("invalid_master_phase", f"{label} has invalid master_phase {master_phase!r}")

        path = safe_relative_path(module.get("path"))
        if path is None:
            add("invalid_path", f"{label} path must be a safe relative path")
        else:
            manifest_module_paths.add(str(path))
            if not (REPO_ROOT / path).exists():
                add("missing_path", f"{label} path does not exist: {path}")

        public_contract = module.get("public_contract_path")
        if public_contract:
            contract_path = safe_relative_path(public_contract)
            if contract_path is None:
                add("invalid_path", f"{label} public_contract_path is invalid")
            elif not (REPO_ROOT / contract_path).is_file():
                add(
                    "missing_public_contract",
                    f"{label} public contract is missing: {contract_path}",
                )
        elif not module.get("no_public_contract_reason"):
            add(
                "missing_public_contract",
                f"{label} must define public_contract_path or no_public_contract_reason",
            )

        allowed_groups = normalize_string_list(module.get("allowed_dependency_groups"))
        if allowed_groups is None:
            add("missing_required_field", f"{label} allowed_dependency_groups must be a list")
        else:
            for group in allowed_groups:
                if group not in accepted_dependency_groups:
                    add("unknown_dependency_group", f"{label} has unknown dependency group {group}")

        generated_paths = normalize_string_list(module.get("generated_output_paths"))
        if generated_paths is None:
            add("missing_required_field", f"{label} generated_output_paths must be a list")
        else:
            for generated in generated_paths:
                generated_path = safe_relative_path(generated)
                if generated_path is None:
                    add("invalid_path", f"{label} generated path is invalid: {generated!r}")
                elif not has_standard_ignore_marker(generated_path):
                    add(
                        "forbidden_generated_path",
                        f"{label} generated path is not protected by standard ignore marker: {generated_path}",
                    )

        test_targets = normalize_string_list(module.get("test_targets"))
        if not test_targets:
            add("missing_test_target", f"{label} must define at least one test target")
        elif any(not target.strip() for target in test_targets):
            add("missing_test_target", f"{label} has a blank test target")

        local_stack = module.get("local_stack_participation")
        if local_stack not in accepted_local_stack:
            add(
                "missing_local_stack_participation",
                f"{label} has invalid local_stack_participation {local_stack!r}",
            )

        doc_links = normalize_string_list(module.get("documentation_links"))
        if doc_links is None:
            add("missing_required_field", f"{label} documentation_links must be a list")
        else:
            for link in doc_links:
                link_path = safe_relative_path(link)
                if link_path is None:
                    add("invalid_path", f"{label} documentation link is invalid: {link!r}")
                elif not (REPO_ROOT / link_path).is_file():
                    add("stale_documentation_link", f"{label} stale documentation link: {link_path}")

        cargo_member = module.get("cargo_member")
        if cargo_member is not None:
            cargo_path = safe_relative_path(cargo_member)
            if cargo_path is None:
                add("invalid_path", f"{label} cargo_member path is invalid")
            else:
                manifest_cargo_members.add(str(cargo_path))
                if not (REPO_ROOT / cargo_path / "Cargo.toml").is_file():
                    add("cargo_member_drift", f"{label} cargo_member lacks Cargo.toml: {cargo_path}")

    actual_members = set(discovered_cargo_members or actual_cargo_members())
    if manifest_cargo_members != actual_members:
        missing = sorted(actual_members - manifest_cargo_members)
        stale = sorted(manifest_cargo_members - actual_members)
        if missing:
            add("cargo_member_drift", f"Cargo members missing from manifest: {missing}")
        if stale:
            add("cargo_member_drift", f"Manifest cargo_member entries absent from Cargo.toml: {stale}")

    actual_packages = set(discovered_package_roots or actual_package_roots())
    unlisted = sorted(actual_packages - manifest_module_paths)
    if unlisted:
        add("unlisted_module", f"Package roots missing from manifest: {unlisted}")

    inventory = manifest.get("workspace_inventory", {})
    if isinstance(inventory, dict):
        manifest_inventory_members = normalize_string_list(inventory.get("cargo_members")) or []
        if sorted(manifest_inventory_members) != sorted(actual_members):
            add("cargo_member_drift", "workspace_inventory.cargo_members differs from Cargo.toml")
        manifest_inventory_packages = normalize_string_list(inventory.get("package_roots")) or []
        if sorted(manifest_inventory_packages) != sorted(actual_packages):
            add("unlisted_module", "workspace_inventory.package_roots differs from packages/")
    else:
        add("missing_required_field", "workspace_inventory table is required")

    return findings


def fail_on_findings(findings: list[ValidationFinding]) -> None:
    if not findings:
        return
    details = "\n".join(f"- {finding.code}: {finding.message}" for finding in findings)
    raise AssertionError(f"manifest validation failed:\n{details}")


def validate_phase3_source_docs() -> None:
    sub_plan = read(SUB_PLAN)
    phase_3 = section(
        sub_plan,
        "## Phase 3: Workspace Manifest And Module Inventory",
    )

    for item in range(1, 6):
        assert_contains(phase_3, f"**3.{item} ", SUB_PLAN)

    for work_item in re.finditer(
        r"- \*\*3\.[1-5] .+?(?=\n- \*\*3\.|\n### Phase 3 Gate Outputs)",
        phase_3,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")

    for heading in REQUIRED_PHASE3_HEADINGS:
        assert_contains(phase_3, heading, SUB_PLAN)
    for state in REQUIRED_PHASE3_STATES:
        assert_contains(phase_3, state, SUB_PLAN)
    for field in REQUIRED_MODULE_FIELDS:
        assert_contains(phase_3, field, SUB_PLAN)
    for code in REQUIRED_DRIFT_CODES:
        assert_contains(phase_3, f"`{code}`", SUB_PLAN)
    assert_contains(phase_3, "`overrid.workspace.toml`", SUB_PLAN)
    assert_contains(phase_3, "`module_record`", SUB_PLAN)

    sds = read(SDS)
    assert_contains(sds, "## Phase 3 Workspace Manifest Decisions", SDS)
    for state in REQUIRED_PHASE3_STATES:
        assert_contains(sds, state, SDS)
    for field in REQUIRED_MODULE_FIELDS:
        assert_contains(sds, field, SDS)
    for code in REQUIRED_DRIFT_CODES:
        assert_contains(sds, f"`{code}`", SDS)

    service = read(SERVICE)
    assert_contains(service, "## Phase 3 Implementation Gates", SERVICE)
    for state in REQUIRED_PHASE3_STATES:
        assert_contains(service, state, SERVICE)
    assert_contains(service, "`scripts/validate_repository_layout_phase3.py`", SERVICE)

    tech_stack = read(TECH_STACK)
    for expected in (
        "Rust-first infrastructure stack",
        "Cargo workspace",
        "JSON Schema",
        "Overrid-shaped local stubs",
        "Node.js/TypeScript as the core control-plane",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)

    suite = read(SUITE_VALIDATOR)
    assert_contains(
        suite,
        'Path("scripts/validate_repository_layout_phase3.py")',
        SUITE_VALIDATOR,
    )


def validate_manifest() -> None:
    manifest = load_toml(MANIFEST)
    fail_on_findings(collect_manifest_findings(manifest))

    for path in (
        manifest.get("source_sds"),
        manifest.get("source_build_plan"),
        manifest.get("tech_stack_decision"),
        manifest.get("workspace_inventory", {}).get("cargo_workspace_manifest"),
    ):
        if not path_exists(path):
            raise AssertionError(f"{MANIFEST} references a missing source path: {path}")

    command_consumers = manifest.get("validation_metadata", {}).get("command_consumers", [])
    for command in ("layout:check", "build", "test", "schema:check", "docs:check"):
        if command not in command_consumers:
            raise AssertionError(f"{MANIFEST} validation_metadata is missing {command}")


def validate_fixture_drift_cases() -> None:
    base = load_toml(MANIFEST)
    actual_members = actual_cargo_members()
    actual_packages = actual_package_roots()

    def assert_code(mutator, expected_code: str) -> None:
        fixture = deepcopy(base)
        mutator(fixture)
        findings = collect_manifest_findings(
            fixture,
            discovered_cargo_members=actual_members,
            discovered_package_roots=actual_packages,
        )
        codes = {finding.code for finding in findings}
        if expected_code not in codes:
            detail = ", ".join(sorted(codes)) or "no findings"
            raise AssertionError(
                f"fixture drift case expected {expected_code}, observed {detail}"
            )

    assert_code(lambda data: data.pop("schema_version", None), "missing_schema_version")

    def duplicate_module(data: dict) -> None:
        data["modules"].append(deepcopy(data["modules"][0]))

    assert_code(duplicate_module, "duplicate_module_name")

    def missing_path(data: dict) -> None:
        data["modules"][0]["path"] = "missing/path"

    assert_code(missing_path, "missing_path")

    def unknown_type(data: dict) -> None:
        data["modules"][0]["type"] = "unknown_module_type_for_fixture"

    assert_code(unknown_type, "unknown_module_type")

    def stale_link(data: dict) -> None:
        data["modules"][0]["documentation_links"].append("docs/missing.md")

    assert_code(stale_link, "stale_documentation_link")

    def missing_target(data: dict) -> None:
        data["modules"][0]["test_targets"] = []

    assert_code(missing_target, "missing_test_target")

    def forbidden_generated_path(data: dict) -> None:
        data["modules"][0]["generated_output_paths"] = ["packages/cli/src"]

    assert_code(forbidden_generated_path, "forbidden_generated_path")

    findings = collect_manifest_findings(
        base,
        discovered_cargo_members=actual_members,
        discovered_package_roots=[*actual_packages, "packages/unlisted_fixture"],
    )
    if "unlisted_module" not in {finding.code for finding in findings}:
        raise AssertionError("fixture drift case expected unlisted_module")


def validate_local_planning_trail() -> None:
    existing = [(REPO_ROOT / path).is_file() for path in (PHASE_PLAN, PHASE_PROGRESS)]
    if not any(existing):
        return
    if not all(existing):
        raise AssertionError(
            f"Local planning trail must include both {PHASE_PLAN} and {PHASE_PROGRESS}"
        )

    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #5 Phase 3", PHASE_PLAN)
    assert_contains(phase_plan, "root workspace manifest", PHASE_PLAN)
    assert_contains(phase_progress, "Repository Layout Phase 3 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)


def validate_local_markdown_links() -> None:
    link_pattern = re.compile(r"(?<!!)\[[^\]]+\]\(([^)]+)\)")
    missing: list[str] = []

    scoped_docs = [
        *SCOPED_DOCS,
        *[path for path in (PHASE_PLAN, PHASE_PROGRESS) if (REPO_ROOT / path).is_file()],
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
        validate_phase3_source_docs,
        validate_manifest,
        validate_fixture_drift_cases,
        validate_local_planning_trail,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("Repository Layout Phase 3 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
