#!/usr/bin/env python3
"""Validate Repository Layout Phase 7 artifact hygiene and indexing gates."""

from __future__ import annotations

from pathlib import Path
import re
import subprocess
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
PHASE_PLAN = Path("docs/planning/repository_layout_phase_07_plan.md")
PHASE_PROGRESS = Path("docs/planning/repository_layout_phase_07_progress.md")
GITIGNORE = Path(".gitignore")
DOCDEXIGNORE = Path(".docdexignore")
PACKAGES_README = Path("packages/README.md")
CLI_README = Path("packages/cli/README.md")
LOCAL_INFRA_README = Path("infra/local/README.md")
INTEGRATION_README = Path("tests/integration/README.md")
CLI_RUNNER = Path("packages/cli/src/runner.rs")

REQUIRED_PHASE7_STATES = [
    "`generated_output_ignore_rules_defined`",
    "`local_state_ignore_rules_defined`",
    "`secret_file_rules_defined`",
    "`docdex_indexing_hygiene_defined`",
    "`artifact_redaction_expectations_defined`",
]

REQUIRED_PHASE7_HEADINGS = [
    "#### Generated-Output Ignore Rules",
    "#### Local-State Ignore Rules",
    "#### Secret-File Rules",
    "#### Docdex Indexing Hygiene",
    "#### Artifact Redaction Expectations",
]

REQUIRED_PHASE7_ARTIFACTS = [
    "generated_file_committed",
    "secret_file_committed",
    "local_state_committed",
    "docdex_index_hygiene_violation",
    "artifact_redaction_violation",
]

REQUIRED_GENERATED_OUTPUT_ROOTS = [
    "target",
    "node_modules",
    "coverage",
    "logs",
    "docs/specs/generated",
    "packages/schemas/admin_ui/generated",
    "infra/local/artifacts",
    "tests/integration/artifacts",
]

REQUIRED_LOCAL_STATE_ROOTS = [
    ".overrid",
    "infra/local/state",
    "infra/local/job-tables",
    "infra/local/artifacts",
    "tests/integration/artifacts",
]

REQUIRED_LOCAL_MARKERS = [
    "infra/local/state/.gitignore",
    "infra/local/job-tables/.gitignore",
    "infra/local/artifacts/.gitignore",
    "tests/integration/artifacts/.gitignore",
    "docs/specs/generated/.gitignore",
]

REQUIRED_SECRET_PATTERNS = [
    ".env",
    ".env.*",
    "*.local.*",
    "*.secret.*",
    "*.key",
    "*.pem",
    "*.p12",
    "*.pfx",
    "*.token",
    "secrets.*",
    "id_ed25519",
]

REQUIRED_DOCDEX_INCLUDE_ROOTS = [
    "docs/build_plan",
    "docs/sds",
    "docs/service_catalog",
    "docs/specs",
    "packages/schemas/admin_ui/fixtures",
    "packages/schemas/overrid_contracts/v0",
    "packages/schemas/overrid_contracts/fixtures",
]

REQUIRED_DOCDEX_EXCLUDE_ROOTS = [
    "target",
    "node_modules",
    "coverage",
    "logs",
    ".overrid",
    "infra/local/state",
    "infra/local/job-tables",
    "infra/local/artifacts",
    "tests/integration/artifacts",
    "docs/specs/generated",
    "packages/schemas/admin_ui/generated",
]

REQUIRED_REDACTION_CLASSES = [
    "secret",
    "key",
    "token",
    "signature",
    "private_payload",
    "encrypted_content",
    "fixture_credential",
]

APPROVED_GENERATED_PROJECTIONS = [
    "packages/schemas/admin_ui/generated/typescript/admin_ui_contracts.d.ts",
    "packages/schemas/admin_ui/generated/typescript/admin_read_api_contracts.d.ts",
    "packages/schemas/overrid_contracts/src/lib.rs",
]

REQUIRED_GITIGNORE_MARKERS = [
    "/.overrid/",
    ".env.*",
    "!.env.example",
    "*.secret.*",
    "*.key",
    "target/",
    "node_modules/",
    ".pnpm-store/",
    "packages/**/generated/**",
    "!packages/schemas/admin_ui/generated/typescript/admin_ui_contracts.d.ts",
    "infra/local/state/*",
    "!infra/local/state/.gitignore",
    "tests/integration/artifacts/*",
    "!tests/integration/artifacts/.gitignore",
    "!docs/planning/repository_layout_phase_07_plan.md",
    "!docs/planning/repository_layout_phase_07_progress.md",
]

REQUIRED_DOCDEXIGNORE_MARKERS = [
    "target/",
    "node_modules/",
    ".overrid/",
    "infra/local/state/",
    "infra/local/job-tables/",
    "infra/local/artifacts/",
    "tests/integration/artifacts/",
    "docs/specs/generated/",
    "packages/**/generated/",
]

GIT_IGNORED_PATHS = [
    "target/debug/overrid",
    "node_modules/pkg/index.js",
    "coverage/lcov.info",
    "logs/run.log",
    ".overrid/state.json",
    "infra/local/state/local.db",
    "infra/local/job-tables/jobs.db",
    "infra/local/artifacts/chunk.bin",
    "tests/integration/artifacts/run.json",
    "docs/specs/generated/openapi.json",
    "packages/schemas/admin_ui/generated/new_projection.d.ts",
    ".env",
    ".env.local",
    "config.local.toml",
    "fixture.secret.toml",
    "private.key",
]

SCOPED_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    PHASE_PLAN,
    PHASE_PROGRESS,
    PACKAGES_README,
    CLI_README,
    LOCAL_INFRA_README,
    INTEGRATION_README,
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


def validate_phase7_source_docs() -> None:
    sub_plan = read(SUB_PLAN)
    phase_7 = section(
        sub_plan,
        "## Phase 7: Generated Artifacts, Secrets, Local State, And Index Hygiene",
    )

    for item in range(1, 6):
        assert_contains(phase_7, f"**7.{item} ", SUB_PLAN)

    for work_item in re.finditer(
        r"- \*\*7\.[1-5] .+?(?=\n- \*\*7\.|\n### Phase 7 Gate Outputs)",
        phase_7,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                raise AssertionError(f"{item_text.splitlines()[0]} is missing {field}")

    for heading in REQUIRED_PHASE7_HEADINGS:
        assert_contains(phase_7, heading, SUB_PLAN)
    for state in REQUIRED_PHASE7_STATES:
        assert_contains(phase_7, state, SUB_PLAN)
    for artifact in REQUIRED_PHASE7_ARTIFACTS:
        assert_contains(phase_7, f"`{artifact}`", SUB_PLAN)

    sds = read(SDS)
    assert_contains(sds, "## Phase 7 Artifact Hygiene And Indexing Decisions", SDS)
    for state in REQUIRED_PHASE7_STATES:
        assert_contains(sds, state, SDS)

    service = read(SERVICE)
    assert_contains(service, "## Phase 7 Implementation Gates", SERVICE)
    for state in [*REQUIRED_PHASE7_STATES, "`phase7_validation_defined`"]:
        assert_contains(service, state, SERVICE)
    assert_contains(service, "`scripts/validate_repository_layout_phase7.py`", SERVICE)

    tech_stack = read(TECH_STACK)
    for expected in (
        "Rust-first infrastructure stack",
        "Generated Rust SDK first",
        "same contracts",
        "Local development",
        "Overrid-shaped local stubs",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)


def validate_manifest_hygiene() -> None:
    manifest = load_toml(MANIFEST)

    manifest_version = manifest.get("manifest_version")
    manifest_match = (
        re.search(r"repository-layout-phase-(\d+)", manifest_version)
        if isinstance(manifest_version, str)
        else None
    )
    if manifest_match is None or int(manifest_match.group(1)) < 7:
        raise AssertionError(f"{MANIFEST} manifest_version must identify phase 7 or later")

    validation_metadata = manifest.get("validation_metadata", {})
    layout_phase = validation_metadata.get("layout_phase")
    if not isinstance(layout_phase, int) or layout_phase < 7:
        raise AssertionError(f"{MANIFEST} validation_metadata.layout_phase must be at least 7")

    hygiene = manifest.get("artifact_hygiene")
    if not isinstance(hygiene, dict):
        raise AssertionError(f"{MANIFEST} must define [artifact_hygiene]")
    if hygiene.get("phase_gate") != "repository-layout-phase-7":
        raise AssertionError("artifact_hygiene.phase_gate is wrong")
    list_contains_all(hygiene.get("states"), [state.strip("`") for state in REQUIRED_PHASE7_STATES], "artifact_hygiene.states")
    list_contains_all(hygiene.get("generated_output_roots"), REQUIRED_GENERATED_OUTPUT_ROOTS, "artifact_hygiene.generated_output_roots")
    list_contains_all(hygiene.get("local_state_roots"), REQUIRED_LOCAL_STATE_ROOTS, "artifact_hygiene.local_state_roots")
    list_contains_all(hygiene.get("local_state_marker_files"), REQUIRED_LOCAL_MARKERS, "artifact_hygiene.local_state_marker_files")
    list_contains_all(hygiene.get("secret_file_deny_patterns"), REQUIRED_SECRET_PATTERNS, "artifact_hygiene.secret_file_deny_patterns")
    list_contains_all(hygiene.get("docdex_index_include_roots"), REQUIRED_DOCDEX_INCLUDE_ROOTS, "artifact_hygiene.docdex_index_include_roots")
    list_contains_all(hygiene.get("docdex_index_exclude_roots"), REQUIRED_DOCDEX_EXCLUDE_ROOTS, "artifact_hygiene.docdex_index_exclude_roots")
    list_contains_all(hygiene.get("redaction_classes"), REQUIRED_REDACTION_CLASSES, "artifact_hygiene.redaction_classes")
    list_contains_all(hygiene.get("violation_artifacts"), REQUIRED_PHASE7_ARTIFACTS, "artifact_hygiene.violation_artifacts")
    list_contains_all(hygiene.get("approved_generated_projection_files"), APPROVED_GENERATED_PROJECTIONS, "artifact_hygiene.approved_generated_projection_files")

    for path_field in ("validation_script", "git_ignore_file", "docdex_ignore_file"):
        if not path_exists(hygiene.get(path_field)):
            raise AssertionError(f"artifact_hygiene.{path_field} is missing")

    registry = manifest.get("root_command_registry", {})
    list_contains_all(registry.get("validation_artifacts"), REQUIRED_PHASE7_ARTIFACTS, "root_command_registry.validation_artifacts")
    layout_commands = [
        command
        for command in manifest.get("root_commands", [])
        if isinstance(command, dict) and command.get("name") == "layout:check"
    ]
    if len(layout_commands) != 1:
        raise AssertionError(f"{MANIFEST} must contain exactly one layout:check command")
    list_contains_all(layout_commands[0].get("outputs"), REQUIRED_PHASE7_ARTIFACTS, "layout:check.outputs")


def git_check_ignored(path: str) -> bool:
    result = subprocess.run(
        ["git", "check-ignore", "--no-index", "-q", path],
        cwd=REPO_ROOT,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    return result.returncode == 0


def validate_ignore_files() -> None:
    gitignore = read(GITIGNORE)
    docdexignore = read(DOCDEXIGNORE)
    for marker in REQUIRED_GITIGNORE_MARKERS:
        assert_contains(gitignore, marker, GITIGNORE)
    for marker in REQUIRED_DOCDEXIGNORE_MARKERS:
        assert_contains(docdexignore, marker, DOCDEXIGNORE)

    for marker in REQUIRED_LOCAL_MARKERS:
        marker_path = REPO_ROOT / marker
        text = marker_path.read_text(encoding="utf-8")
        if "*" not in text or "!.gitignore" not in text:
            raise AssertionError(f"{marker} must ignore contents and keep .gitignore visible")

    for path in GIT_IGNORED_PATHS:
        if not git_check_ignored(path):
            raise AssertionError(f"{path} should be ignored by .gitignore")

    for path in APPROVED_GENERATED_PROJECTIONS:
        if not (REPO_ROOT / path).is_file():
            raise AssertionError(f"approved generated projection is missing: {path}")
        if git_check_ignored(path):
            raise AssertionError(f"approved generated projection should remain visible: {path}")

    for forbidden in ("docs/build_plan/", "docs/sds/", "docs/service_catalog/"):
        if forbidden in docdexignore:
            raise AssertionError(f"{DOCDEXIGNORE} must not exclude {forbidden}")


def validate_readme_hygiene_evidence() -> None:
    packages = read(PACKAGES_README)
    for expected in (
        "Repository Layout Phase 7 generated-output rules",
        "Approved generated projection files",
        "secret, key, token, signature, private payload",
    ):
        assert_contains(packages, expected, PACKAGES_README)

    cli = read(CLI_README)
    for expected in (
        "Repository Layout` Phase 7",
        "generated_output_ignore_rules_defined",
        "local_state_ignore_rules_defined",
        "secret_file_rules_defined",
        "docdex_indexing_hygiene_defined",
        "artifact_redaction_expectations_defined",
        "local_state_committed",
        "docdex_index_hygiene_violation",
        "artifact_redaction_violation",
    ):
        assert_contains(cli, expected, CLI_README)

    local = read(LOCAL_INFRA_README)
    for expected in (
        "Repository Layout Phase 7",
        "`state/`, `job-tables/`, and `artifacts/`",
        "redact secrets, keys, tokens, signatures",
    ):
        assert_contains(local, expected, LOCAL_INFRA_README)

    integration = read(INTEGRATION_README)
    for expected in (
        "Repository Layout Phase 7",
        "`artifacts/`",
        "path/reason refs only",
    ):
        assert_contains(integration, expected, INTEGRATION_README)


def validate_cli_wiring() -> None:
    runner = read(CLI_RUNNER)
    for expected in (
        "phase7_artifact_hygiene",
        "generated_output_ignore_rules",
        "local_state_ignore_rules",
        "secret_file_rules",
        "docdex_indexing_hygiene",
        "artifact_redaction_expectations",
        "required_hygiene_file",
        "push_marker_only_directory",
        "push_ignore_file_contains",
        "push_secret_like_paths_absent",
        "first_forbidden_secret_like_path",
        "layout_check_emits_phase7_hygiene_records",
        "layout_check_rejects_phase7_hygiene_violations",
    ):
        assert_contains(runner, expected, CLI_RUNNER)
    for artifact in REQUIRED_PHASE7_ARTIFACTS:
        assert_contains(runner, artifact, CLI_RUNNER)
    if "OVERRID_PHASE7_SENTINEL_SECRET" not in runner:
        raise AssertionError(f"{CLI_RUNNER} must include a redaction sentinel regression test")


def validate_local_planning_trail() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in (
        "Generated Artifacts, Secrets, Local State, And Index Hygiene",
        "packages/cli/src/runner.rs",
        "scripts/validate_repository_layout_phase7.py",
        "docdexd run-tests --repo . --target scripts/validate_repository_layout_phase7.py",
    ):
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in (
        "Loaded Docdex profile and repo memory",
        "Confirmed Phase 7 scope",
        "Validation Evidence",
    ):
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_suite_registration() -> None:
    suite = read(SUITE_VALIDATOR)
    assert_contains(
        suite,
        'Path("scripts/validate_repository_layout_phase7.py")',
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
        validate_phase7_source_docs,
        validate_manifest_hygiene,
        validate_ignore_files,
        validate_readme_hygiene_evidence,
        validate_cli_wiring,
        validate_local_planning_trail,
        validate_suite_registration,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("Repository Layout Phase 7 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
