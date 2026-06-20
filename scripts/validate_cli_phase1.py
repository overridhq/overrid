#!/usr/bin/env python3
"""Validate the CLI Phase 1 documentation gate."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from urllib.parse import unquote, urlparse


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_002_cli.md")
SDS = Path("docs/sds/foundation/cli.md")
SERVICE = Path("docs/service_catalog/foundation/cli.md")
MASTER = Path("docs/build_plan/master_plan.md")
CROSSWALK = Path("docs/build_plan/service_catalog_alignment.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/cli_phase_01_plan.md")
PHASE_PROGRESS = Path("docs/planning/cli_phase_01_progress.md")

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

REQUIRED_GATE_HEADINGS = [
    "#### Link Attachment Matrix",
    "#### Frozen CLI Boundary",
    "#### Command Availability Matrix",
    "#### Resolved SDS Decision Checklist",
    "#### Documentation Update Rule",
]

REQUIRED_COMMAND_STATES = [
    "`available`",
    "`hidden`",
    "`documented_planned`",
    "`not_available_in_phase`",
]

REQUIRED_PHASE1_COMMANDS = [
    "`overrid version`",
    "`overrid doctor`",
    "`overrid profile create|list|select|inspect|reset`",
    "`overrid auth login|whoami`",
    "`overrid credential enroll|inspect`",
    "`overrid tenant create|list|inspect`",
    "`overrid identity create|list|inspect|disable`",
    "`overrid key enroll|list|rotate|revoke`",
    "`overrid manifest validate|submit|inspect`",
    "`overrid workload submit|status|timeline`",
    "`overrid dev start|stop|reset|seed|smoke`",
]

REQUIRED_LATER_COMMANDS = [
    "`overrid node register|inspect|health`",
    "`overrid workload logs|cancel|result|follow`",
    "`overrid policy dry-run`",
    "`overrid usage show`, `overrid receipt show`, `overrid dispute list|inspect`",
    "`overrid package validate`",
]

REQUIRED_SDS_DECISIONS = [
    "Platform-native credential storage by default.",
    "Phase 1 command limits.",
    "Deterministic idempotency fingerprints.",
    "Small numeric exit-code registry.",
    "Profile environment separation.",
]

REQUIRED_SERVICE_DEVELOPMENT_ORDER = [
    "Freeze Phase 1 command availability",
    "Build the Phase 2 Rust CLI crate skeleton",
    "Implement Phase 1 runtime command groups: version, doctor, profile, auth, credential, tenant, identity, key, manifest, synthetic workload submit/status/timeline, and local-only dev helpers.",
    "Phase 2 node commands; Phase 3 real workload logs, cancellation, result, and follow; Phase 4 policy dry-run; Phase 5/6 usage, receipt, and dispute reads; Phase 9 package and deployment-adjacent commands.",
    "Harden product integration, stable JSON output, diagnostics, security/redaction, and release readiness.",
]

REJECTED_UNQUALIFIED_SERVICE_ORDER = [
    "Add workload submit, inspect, cancel, result, and logs commands.",
    "Add node registration and node health commands.",
    "Add usage, receipt, policy dry-run, and package validation commands.",
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


def assert_not_contains(text: str, rejected: str, source: Path) -> None:
    if rejected in text:
        raise AssertionError(f"{source} still contains rejected text: {rejected}")


def validate_sub_plan() -> None:
    text = read(SUB_PLAN)
    assert_contains(text, "# SUB BUILD PLAN #2 - CLI", SUB_PLAN)
    assert_contains(text, "Attached SDS: [docs/sds/foundation/cli.md]", SUB_PLAN)

    phase_headings = re.findall(r"^## Phase (\d+):", text, flags=re.MULTILINE)
    if phase_headings != [str(number) for number in range(1, 11)]:
        raise AssertionError(f"{SUB_PLAN} must contain Phase 1 through Phase 10 in order")

    phase_1 = section(text, "## Phase 1: SDS Alignment, Command Scope, And Phase Gates")
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
    for state in REQUIRED_COMMAND_STATES:
        assert_contains(phase_1, state, SUB_PLAN)
    for command in REQUIRED_PHASE1_COMMANDS:
        assert_contains(phase_1, command, SUB_PLAN)
    for command in REQUIRED_LATER_COMMANDS:
        assert_contains(phase_1, command, SUB_PLAN)
    for decision in REQUIRED_SDS_DECISIONS:
        assert_contains(phase_1, decision, SUB_PLAN)

    guardrails = [
        "Rust binary crate using generated contracts",
        "Direct reads or writes against Overbase, Overstore, Overvault, Overqueue, Overwatch, Seal Ledger, node-agent state, or service-local files.",
        "Raw private keys, ambient CI keychains, world-readable config secrets, printed signatures, or decrypted payload output.",
        "Reject any CLI-only command behavior that is not backed by SDK/Overgate contracts and an owning service document.",
    ]
    for guardrail in guardrails:
        assert_contains(phase_1, guardrail, SUB_PLAN)


def validate_cross_doc_alignment() -> None:
    sds = read(SDS)
    service = read(SERVICE)
    master = read(MASTER)
    crosswalk = read(CROSSWALK)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)

    assert_contains(sds, "[SUB BUILD PLAN #2 - CLI]", SDS)
    assert_contains(sds, "schema-checked client on top of the SDK and Overgate", SDS)
    assert_contains(sds, "not_available_in_phase", SDS)
    assert_contains(sds, "Resolved decisions:", SDS)

    assert_contains(service, "[SUB BUILD PLAN #2 - CLI]", SERVICE)
    assert_contains(service, "Basic commands in [Phase 1: Control-Plane Skeleton]", SERVICE)
    assert_contains(service, "Every platform call must go through the SDK and Overgate path.", SERVICE)
    assert_contains(service, "## Phase 1 Implementation Gates", SERVICE)
    assert_contains(service, "direct storage, queue, ledger, vault, object-store, node-agent, or service-local state access", SERVICE)
    development_order = section(service, "## Development Order")
    for expected in REQUIRED_SERVICE_DEVELOPMENT_ORDER:
        assert_contains(development_order, expected, SERVICE)
    for rejected in REJECTED_UNQUALIFIED_SERVICE_ORDER:
        assert_not_contains(development_order, rejected, SERVICE)

    assert_contains(master, "SDS #2: [CLI]", MASTER)
    assert_contains(master, "[SUB BUILD PLAN #2 - CLI]", MASTER)
    assert_contains(master, "First build point remains Phase 1", MASTER)

    assert_contains(crosswalk, "| SDS #2 | [CLI]", CROSSWALK)
    assert_contains(crosswalk, "[SUB BUILD PLAN #2 - CLI]", CROSSWALK)
    assert_contains(crosswalk, "[Phase 1: Control-Plane Skeleton]", CROSSWALK)
    assert_contains(crosswalk, "[Phase 6: First Product Integration]", CROSSWALK)

    assert_contains(tech_stack, "| CLI | Rust CLI using generated contracts", TECH_STACK)
    assert_contains(tech_stack, "not as the core runtime for the grid", TECH_STACK)
    assert_contains(tech_stack, "Node.js/TypeScript as the core control-plane", TECH_STACK)

    assert_contains(phase_plan, "Complete the Phase 1 gate work", PHASE_PLAN)
    assert_contains(phase_plan, "Do not introduce a Rust CLI crate in this phase", PHASE_PLAN)


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
        validate_sub_plan,
        validate_cross_doc_alignment,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("CLI Phase 1 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
