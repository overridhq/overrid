#!/usr/bin/env python3
"""Validate Repository Layout Phase 5 root command registry and layout check gates."""

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
PHASE_PLAN = Path("docs/planning/repository_layout_phase_05_plan.md")
PHASE_PROGRESS = Path("docs/planning/repository_layout_phase_05_progress.md")
CLI_README = Path("packages/cli/README.md")
CLI_PARSER = Path("packages/cli/src/parser.rs")
CLI_RUNNER = Path("packages/cli/src/runner.rs")
PHASE4_VALIDATOR = Path("scripts/validate_repository_layout_phase4.py")

REQUIRED_ROOT_COMMANDS = [
    "build",
    "test",
    "test:integration",
    "dev:start",
    "dev:stop",
    "dev:reset",
    "dev:seed",
    "dev:status",
    "schema:check",
    "docs:check",
    "layout:check",
]

REQUIRED_PHASE5_STATES = [
    "`root_command_registry_defined`",
    "`rust_owned_command_execution_defined`",
    "`layout_check_defined`",
    "`schema_docs_check_orchestration_defined`",
    "`validation_artifacts_defined`",
]

REQUIRED_PHASE5_HEADINGS = [
    "#### Semantic Root Command Registry",
    "#### Rust-Owned Command Execution",
    "#### Layout Check Contract",
    "#### Schema And Docs Check Orchestration",
    "#### Validation Artifact Schema",
]

REQUIRED_ARTIFACTS = [
    "layout_check.passed",
    "layout_check.failed",
    "package_boundary_violation",
    "missing_service_contract",
    "missing_test_target",
    "generated_file_committed",
    "secret_file_committed",
]

REQUIRED_ROOT_COMMAND_FIELDS = [
    "name",
    "purpose",
    "inputs",
    "outputs",
    "owning_tool",
    "phase_gate",
    "canonical_invocation",
    "machine_readable_result_envelope",
    "failure_classes",
    "aliases",
]

SCOPED_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    CLI_README,
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


def path_exists(value: object) -> bool:
    if not isinstance(value, str) or not value:
        return False
    path = Path(value)
    if path.is_absolute() or ".." in path.parts:
        return False
    return (REPO_ROOT / path).exists()


def validate_phase5_source_docs() -> None:
    sub_plan = read(SUB_PLAN)
    phase_5 = section(sub_plan, "## Phase 5: Root Command Registry And Layout Check")

    for item in range(1, 6):
        assert_contains(phase_5, f"**5.{item} ", SUB_PLAN)

    for work_item in re.finditer(
        r"- \*\*5\.[1-5] .+?(?=\n- \*\*5\.|\n### Phase 5 Gate Outputs)",
        phase_5,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                raise AssertionError(f"{item_text.splitlines()[0]} is missing {field}")

    for heading in REQUIRED_PHASE5_HEADINGS:
        assert_contains(phase_5, heading, SUB_PLAN)
    for state in REQUIRED_PHASE5_STATES:
        assert_contains(phase_5, state, SUB_PLAN)
    for command in REQUIRED_ROOT_COMMANDS:
        assert_contains(phase_5, f"`{command}`", SUB_PLAN)
    for artifact in REQUIRED_ARTIFACTS:
        assert_contains(phase_5, f"`{artifact}`", SUB_PLAN)

    sds = read(SDS)
    assert_contains(sds, "## Phase 5 Root Command Registry And Layout Check Decisions", SDS)
    for state in REQUIRED_PHASE5_STATES:
        assert_contains(sds, state, SDS)

    service = read(SERVICE)
    assert_contains(service, "## Phase 5 Implementation Gates", SERVICE)
    for state in [*REQUIRED_PHASE5_STATES, "`phase5_validation_defined`"]:
        assert_contains(service, state, SERVICE)
    assert_contains(service, "`scripts/validate_repository_layout_phase5.py`", SERVICE)

    tech_stack = read(TECH_STACK)
    for expected in (
        "Rust-first infrastructure stack",
        "Rust CLI",
        "Cargo workspace",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)


def validate_manifest_registry() -> None:
    manifest = load_toml(MANIFEST)
    if manifest.get("schema_version") != 1:
        raise AssertionError(f"{MANIFEST} schema_version must be 1")

    manifest_version = manifest.get("manifest_version")
    manifest_match = (
        re.search(r"repository-layout-phase-(\d+)", manifest_version)
        if isinstance(manifest_version, str)
        else None
    )
    if manifest_match is None or int(manifest_match.group(1)) < 5:
        raise AssertionError(f"{MANIFEST} manifest_version must identify phase 5 or later")

    validation_metadata = manifest.get("validation_metadata", {})
    layout_phase = validation_metadata.get("layout_phase")
    if not isinstance(layout_phase, int) or layout_phase < 5:
        raise AssertionError(f"{MANIFEST} validation_metadata.layout_phase must be at least 5")
    command_consumers = validation_metadata.get("command_consumers", [])
    for command in REQUIRED_ROOT_COMMANDS:
        if command not in command_consumers:
            raise AssertionError(f"{MANIFEST} command_consumers missing {command}")

    registry = manifest.get("root_command_registry")
    if not isinstance(registry, dict):
        raise AssertionError(f"{MANIFEST} must define [root_command_registry]")
    if registry.get("registry_version") != "repository-layout-phase-5":
        raise AssertionError(f"{MANIFEST} root_command_registry.registry_version is wrong")
    if registry.get("owner") != "overrid-cli":
        raise AssertionError(f"{MANIFEST} root_command_registry.owner must be overrid-cli")
    if registry.get("alias_policy") != "thin_alias_only":
        raise AssertionError(f"{MANIFEST} root_command_registry.alias_policy must be thin_alias_only")
    if registry.get("machine_readable_result_envelope") is not True:
        raise AssertionError(f"{MANIFEST} root command registry must require JSON envelopes")
    for artifact in REQUIRED_ARTIFACTS:
        if artifact not in registry.get("validation_artifacts", []):
            raise AssertionError(f"{MANIFEST} registry missing validation artifact {artifact}")

    commands = manifest.get("root_commands")
    if not isinstance(commands, list):
        raise AssertionError(f"{MANIFEST} must define [[root_commands]] records")
    by_name = {}
    for command in commands:
        if not isinstance(command, dict):
            raise AssertionError(f"{MANIFEST} root command records must be TOML tables")
        name = command.get("name")
        if name in by_name:
            raise AssertionError(f"{MANIFEST} has duplicate root command: {name}")
        by_name[name] = command
        for field in REQUIRED_ROOT_COMMAND_FIELDS:
            if field not in command:
                raise AssertionError(f"{MANIFEST} root command {name} missing {field}")
        if command.get("owning_tool") != "overrid-cli":
            raise AssertionError(f"{MANIFEST} root command {name} must be owned by overrid-cli")
        if command.get("phase_gate") != "phase_0":
            raise AssertionError(f"{MANIFEST} root command {name} must remain phase_0")
        if command.get("machine_readable_result_envelope") is not True:
            raise AssertionError(f"{MANIFEST} root command {name} must declare JSON envelope support")
        if not isinstance(command.get("inputs"), list) or not command["inputs"]:
            raise AssertionError(f"{MANIFEST} root command {name} must declare inputs")
        if not isinstance(command.get("outputs"), list) or not command["outputs"]:
            raise AssertionError(f"{MANIFEST} root command {name} must declare outputs")
        if not command.get("failure_classes"):
            raise AssertionError(f"{MANIFEST} root command {name} must declare failure classes")

    missing_commands = [command for command in REQUIRED_ROOT_COMMANDS if command not in by_name]
    if missing_commands:
        raise AssertionError(
            f"{MANIFEST} root commands missing required phase 5 commands: {missing_commands}"
        )

    layout_outputs = by_name["layout:check"].get("outputs", [])
    for artifact in REQUIRED_ARTIFACTS:
        if artifact not in layout_outputs:
            raise AssertionError(f"{MANIFEST} layout:check outputs missing {artifact}")

    for command in commands:
        for input_path in command.get("inputs", []):
            # Inputs may be command-level virtual roots only if they are documented CLI records.
            if isinstance(input_path, str) and input_path.startswith("overrid "):
                continue
            if not path_exists(input_path):
                raise AssertionError(
                    f"{MANIFEST} root command {command['name']} input path missing: {input_path}"
                )


def validate_cli_wiring() -> None:
    parser = read(CLI_PARSER)
    runner = read(CLI_RUNNER)
    readme = read(CLI_README)

    for expected in (
        "CommandRegistry",
        "RootCommand",
        "RootCommand::Test",
        "RootCommand::LayoutCheck",
        '"command-registry"',
        '"test"',
        '"layout:check"',
        '"test:integration"',
        '"dev:start"',
    ):
        assert_contains(parser, expected, CLI_PARSER)

    for expected in (
        "ROOT_COMMAND_REGISTRY",
        "LAYOUT_VALIDATION_ARTIFACTS",
        "command_registry_result",
        "layout_check_result",
        "collect_layout_check_records",
        "secret_file_absence",
        "secret_file_committed",
        "layout_check.passed",
        "layout_check.failed",
    ):
        assert_contains(runner, expected, CLI_RUNNER)

    for command in REQUIRED_ROOT_COMMANDS:
        assert_contains(runner, f'name: "{command}"', CLI_RUNNER)
        assert_contains(readme, command, CLI_README)

    for artifact in REQUIRED_ARTIFACTS:
        assert_contains(runner, artifact, CLI_RUNNER)
    if "-----BEGIN" in runner or "raw_secret" in runner:
        raise AssertionError(f"{CLI_RUNNER} must not include raw secret material")
    assert_contains(readme, "Shell, Make, just, or npm aliases may only be thin wrappers", CLI_README)


def validate_suite_and_phase4_forward_compatibility() -> None:
    suite = read(SUITE_VALIDATOR)
    assert_contains(suite, 'Path("scripts/validate_repository_layout_phase5.py")', SUITE_VALIDATOR)

    phase4 = read(PHASE4_VALIDATOR)
    assert_contains(phase4, "phase 4 or later", PHASE4_VALIDATOR)
    assert_contains(phase4, "layout_phase must be at least 4", PHASE4_VALIDATOR)


def validate_local_planning_trail() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in (
        "Root Command Registry",
        "layout:check",
        "packages/cli/src/parser.rs",
        "scripts/validate_repository_layout_phase5.py",
    ):
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in (
        "Loaded Docdex profile and repo memory",
        "Confirmed Phase 5 scope",
        "Validation Evidence",
    ):
        assert_contains(progress, expected, PHASE_PROGRESS)


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
        validate_phase5_source_docs,
        validate_manifest_registry,
        validate_cli_wiring,
        validate_suite_and_phase4_forward_compatibility,
        validate_local_planning_trail,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("Repository Layout Phase 5 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
