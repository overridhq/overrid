#!/usr/bin/env python3
"""Validate Repository Layout Phase 2 workspace-shape contracts."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from urllib.parse import unquote, urlparse


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_005_repository_layout.md")
SDS = Path("docs/sds/foundation/repository_layout.md")
SERVICE = Path("docs/service_catalog/foundation/repository_layout.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")
PHASE_PLAN = Path("docs/planning/repository_layout_phase_02_plan.md")
PHASE_PROGRESS = Path("docs/planning/repository_layout_phase_02_progress.md")

REQUIRED_TOP_LEVEL_DIRS = [
    Path("services"),
    Path("packages"),
    Path("infra"),
    Path("tests"),
    Path("docs/specs"),
    Path("docs/build_plan"),
    Path("docs/service_catalog"),
    Path("docs/sds"),
]

REQUIRED_CONTRACT_FILES = [
    Path("services/README.md"),
    Path("services/control-plane/README.md"),
    Path("services/node-agent/README.md"),
    Path("packages/README.md"),
    Path("packages/admin_ui_shell/README.md"),
    Path("packages/schemas/README.md"),
    Path("packages/sdk/README.md"),
    Path("packages/cli/README.md"),
    Path("packages/integration_harness/README.md"),
    Path("packages/local_stack/README.md"),
    Path("infra/README.md"),
    Path("infra/local/README.md"),
    Path("infra/local/profiles/README.md"),
    Path("infra/local/service-definitions/README.md"),
    Path("tests/README.md"),
    Path("tests/integration/README.md"),
    Path("tests/integration/scenarios/README.md"),
    Path("docs/specs/README.md"),
]

IGNORED_LOCAL_MARKERS = [
    Path("infra/local/state/.gitignore"),
    Path("infra/local/job-tables/.gitignore"),
    Path("infra/local/artifacts/.gitignore"),
    Path("tests/integration/artifacts/.gitignore"),
    Path("docs/specs/generated/.gitignore"),
]

REQUIRED_PHASE2_HEADINGS = [
    "#### Phase 2 Directory Contract Matrix",
    "#### Service Path Ownership",
    "#### Package Path Ownership",
    "#### Local Infrastructure And Integration Test Paths",
    "#### Specs Placement Contract",
]

REQUIRED_PHASE2_STATES = [
    "`top_level_contracts_scaffolded`",
    "`service_path_rules_defined`",
    "`package_path_rules_defined`",
    "`local_infra_test_paths_defined`",
    "`specs_contract_defined`",
]

REQUIRED_PHASE2_PATHS = [
    "`services`",
    "`services/control-plane`",
    "`services/node-agent`",
    "`packages/schemas`",
    "`packages/sdk`",
    "`packages/cli`",
    "`infra/local`",
    "`tests/integration`",
    "`docs/specs`",
]

IMPLEMENTED_COMPANION_PACKAGE_PATHS = [
    "`packages/admin_ui_shell`",
    "`packages/integration_harness`",
    "`packages/local_stack`",
]

SCOPED_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    TECH_STACK,
    Path("services/README.md"),
    Path("services/control-plane/README.md"),
    Path("services/node-agent/README.md"),
    Path("packages/README.md"),
    Path("packages/admin_ui_shell/README.md"),
    Path("packages/schemas/README.md"),
    Path("packages/sdk/README.md"),
    Path("packages/integration_harness/README.md"),
    Path("packages/local_stack/README.md"),
    Path("infra/README.md"),
    Path("infra/local/README.md"),
    Path("tests/README.md"),
    Path("tests/integration/README.md"),
    Path("docs/specs/README.md"),
]


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


def validate_phase2_source_docs() -> None:
    sub_plan = read(SUB_PLAN)
    phase_2 = section(
        sub_plan,
        "## Phase 2: Top-Level Directory Contracts And Workspace Shape",
    )

    for item in range(1, 6):
        assert_contains(phase_2, f"**2.{item} ", SUB_PLAN)

    for work_item in re.finditer(
        r"- \*\*2\.[1-5] .+?(?=\n- \*\*2\.|\n### Phase 2 Gate Outputs)",
        phase_2,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")

    for heading in REQUIRED_PHASE2_HEADINGS:
        assert_contains(phase_2, heading, SUB_PLAN)
    for state in REQUIRED_PHASE2_STATES:
        assert_contains(phase_2, state, SUB_PLAN)
    for path in REQUIRED_PHASE2_PATHS:
        assert_contains(phase_2, path, SUB_PLAN)
    for path in IMPLEMENTED_COMPANION_PACKAGE_PATHS:
        assert_contains(phase_2, path, SUB_PLAN)

    sds = read(SDS)
    assert_contains(sds, "## Phase 2 Workspace Shape Decisions", SDS)
    for state in REQUIRED_PHASE2_STATES:
        assert_contains(sds, state, SDS)
    for path in REQUIRED_PHASE2_PATHS:
        assert_contains(sds, path, SDS)
    for path in IMPLEMENTED_COMPANION_PACKAGE_PATHS:
        assert_contains(sds, path, SDS)

    service = read(SERVICE)
    assert_contains(service, "## Phase 2 Implementation Gates", SERVICE)
    for state in REQUIRED_PHASE2_STATES:
        assert_contains(service, state, SERVICE)
    for path in REQUIRED_PHASE2_PATHS:
        assert_contains(service, path, SERVICE)
    for path in IMPLEMENTED_COMPANION_PACKAGE_PATHS:
        assert_contains(service, path, SERVICE)

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
        'Path("scripts/validate_repository_layout_phase2.py")',
        SUITE_VALIDATOR,
    )


def validate_directory_contracts() -> None:
    for path in REQUIRED_TOP_LEVEL_DIRS:
        if not (REPO_ROOT / path).is_dir():
            raise AssertionError(f"Missing required Phase 2 directory: {path}")

    for path in REQUIRED_CONTRACT_FILES:
        text = read(path)
        if not text.strip().startswith("# "):
            raise AssertionError(f"{path} must start with a Markdown H1")

    expected_text = {
        Path("services/control-plane/README.md"): [
            "one modular Rust process",
            "through master Phase 3",
            "not split into deployable microservices",
        ],
        Path("services/node-agent/README.md"): ["Overcell node agent", "simulator"],
        Path("packages/README.md"): [
            "`packages/admin_ui_shell`",
            "`packages/schemas`",
            "`packages/sdk`",
            "`packages/cli`",
            "`packages/integration_harness`",
            "`packages/local_stack`",
            "Generated code is never the source of truth",
        ],
        Path("packages/admin_ui_shell/README.md"): [
            "TypeScript operator shell",
            "client-side surface only",
            "generated TypeScript projections",
            "must not become the source of truth",
        ],
        Path("packages/schemas/README.md"): [
            "canonical JSON Schema",
            "Generated code is not the source of truth",
        ],
        Path("packages/sdk/README.md"): [
            "Rust SDK first",
            "`packages/schemas`",
            "SDK/Overgate routing",
        ],
        Path("packages/integration_harness/README.md"): [
            "Rust integration validation gate",
            "Integration Test Harness SDS",
            "cargo test -p overrid-integration-harness",
            "non-production validation gate",
            "secret-free",
        ],
        Path("packages/local_stack/README.md"): [
            "Rust helper types",
            "Local Development Stack SDS",
            "cargo test -p overrid-local-stack",
            "Overrid-shaped local state",
            "PostgreSQL, Redis, S3, MinIO, Kafka, NATS, Vault",
        ],
        Path("infra/local/README.md"): [
            "Overrid-shaped local",
            "`state/`",
            "`job-tables/`",
            "`artifacts/`",
            "PostgreSQL, Redis, S3, MinIO, Kafka, NATS, Vault",
        ],
        Path("tests/integration/README.md"): [
            "cross-service",
            "`scenarios/`",
            "`artifacts/`",
            "secret-free",
        ],
        Path("docs/specs/README.md"): [
            "protocol",
            "reason-code",
            "`docs/specs/generated/`",
            "must not be executed as runtime configuration",
        ],
    }

    for path, snippets in expected_text.items():
        text = read(path)
        for snippet in snippets:
            assert_contains(text, snippet, path)

    package_roots = sorted(
        child
        for child in (REPO_ROOT / "packages").iterdir()
        if child.is_dir() and not child.name.startswith(".")
    )
    for package_root in package_roots:
        readme = package_root / "README.md"
        rel_readme = readme.relative_to(REPO_ROOT)
        if not readme.is_file():
            raise AssertionError(
                f"Implemented package root is missing ownership README: {rel_readme}"
            )


def validate_ignored_markers() -> None:
    expected = "*\n!.gitignore\n"
    for path in IGNORED_LOCAL_MARKERS:
        marker = REPO_ROOT / path
        if not marker.is_file():
            raise AssertionError(f"Missing ignored-path marker: {path}")
        if marker.read_text(encoding="utf-8") != expected:
            raise AssertionError(f"{path} must contain the standard ignore marker")

        parent = marker.parent
        tracked_files = [child.name for child in parent.iterdir() if child.name != ".gitignore"]
        if tracked_files:
            raise AssertionError(
                f"{parent.relative_to(REPO_ROOT)} should contain only .gitignore in Phase 2"
            )


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
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #5 Phase 2", PHASE_PLAN)
    assert_contains(phase_plan, "source-controlled contract files", PHASE_PLAN)
    assert_contains(phase_progress, "Repository Layout Phase 2 Progress", PHASE_PROGRESS)
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
        validate_phase2_source_docs,
        validate_directory_contracts,
        validate_ignored_markers,
        validate_local_planning_trail,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("Repository Layout Phase 2 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
