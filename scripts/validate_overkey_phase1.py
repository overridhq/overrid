#!/usr/bin/env python3
"""Validate Overkey Phase 1 authority and documentation gates."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from urllib.parse import unquote, urlparse


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_009_overkey.md")
SDS = Path("docs/sds/control_plane/overkey.md")
SERVICE = Path("docs/service_catalog/control_plane/overkey.md")
MASTER = Path("docs/build_plan/master_plan.md")
CROSSWALK = Path("docs/build_plan/service_catalog_alignment.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overkey_phase_01_plan.md")
PHASE_PROGRESS = Path("docs/planning/overkey_phase_01_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

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
    "#### Frozen Credential Metadata Authority",
    "#### Master Phase Gate Matrix",
    "#### Resolved SDS Decision Checklist",
    "#### Runtime Authority Boundary Matrix",
    "#### Documentation Update Rule",
]

REQUIRED_GATE_STATES = [
    "`attached`",
    "`credential_metadata_authority_frozen`",
    "`master_phase_1_owned`",
    "`resolved_decision_carried`",
    "`overkey_owned`",
    "`downstream_owned`",
    "`forbidden_in_overkey`",
]

REQUIRED_ATTACHMENT_PATHS = [
    "docs/sds/control_plane/overkey.md",
    "docs/service_catalog/control_plane/overkey.md",
    "docs/build_plan/master_plan.md",
    "docs/build_plan/service_catalog_alignment.md",
    "docs/overrid_tech_stack_choice.md",
    "docs/planning/overkey_phase_01_plan.md",
    "docs/planning/overkey_phase_01_progress.md",
]

REQUIRED_RESOLVED_DECISIONS = [
    "Ed25519 command signatures",
    "BLAKE3 canonical hashes and fingerprints",
    "Explicit non-secret credential and key ids",
    "Production protection classes",
    "Short verification caches",
    "Revocation epoch invalidation",
    "Signed break-glass revocation through Overgate",
]

REQUIRED_SDS_DECISION_SNIPPETS = [
    "Ed25519",
    "BLAKE3",
    "explicit non-secret identifiers",
    "Verification caches",
    "break-glass command",
]

REQUIRED_OWNERSHIP_TERMS = [
    "Credential records",
    "API key hashes",
    "public signing key metadata",
    "service-account key records",
    "delegated access metadata",
    "rotation links",
    "revocation records",
    "last-used metadata",
    "verification results",
    "secret-ref contracts",
]

REQUIRED_FORBIDDEN_TERMS = [
    "raw secret custody",
    "business authorization",
    "tenant membership",
    "policy finality",
    "request admission",
    "accounting settlement",
    "raw private keys",
    "bearer tokens",
    "raw API keys",
]

REQUIRED_TECH_GUARDRAILS = [
    "Rust service using Tokio and Axum/Tower/Hyper-style HTTP",
    "canonical JSON plus JSON Schema",
    "Ed25519",
    "BLAKE3",
    "Overvault owns protected secret material",
    "PostgreSQL, Redis, Kafka, NATS, S3, MinIO, Vault",
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
    assert_contains(text, "# SUB BUILD PLAN #9 - Overkey", SUB_PLAN)
    assert_contains(text, "Attached SDS: [docs/sds/control_plane/overkey.md]", SUB_PLAN)

    phase_headings = re.findall(r"^## Phase (\d+):", text, flags=re.MULTILINE)
    if phase_headings != [str(number) for number in range(1, 11)]:
        raise AssertionError(f"{SUB_PLAN} must contain Phase 1 through Phase 10 in order")

    phase_1 = section(
        text,
        "## Phase 1: SDS Attachment, Authority, And Boundary Rules",
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
    for path in REQUIRED_ATTACHMENT_PATHS:
        assert_contains(phase_1, path, SUB_PLAN)
    for decision in REQUIRED_RESOLVED_DECISIONS:
        assert_contains(phase_1, decision, SUB_PLAN)
    for term in REQUIRED_OWNERSHIP_TERMS:
        assert_contains(phase_1, term, SUB_PLAN)
    for term in REQUIRED_FORBIDDEN_TERMS:
        assert_contains(phase_1, term, SUB_PLAN)
    for guardrail in REQUIRED_TECH_GUARDRAILS:
        assert_contains(text, guardrail, SUB_PLAN)


def validate_cross_doc_alignment() -> None:
    sds = read(SDS)
    service = read(SERVICE)
    master = read(MASTER)
    crosswalk = read(CROSSWALK)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    suite_validator = read(SUITE_VALIDATOR)

    assert_contains(sds, "[SUB BUILD PLAN #9 - Overkey]", SDS)
    assert_contains(sds, "Resolved decisions:", SDS)
    for decision in REQUIRED_SDS_DECISION_SNIPPETS:
        assert_contains(sds, decision, SDS)
    assert_contains(
        sds,
        "Overkey does not own raw secret storage, business authorization, tenant membership, or request admission.",
        SDS,
    )

    assert_contains(service, "[SUB BUILD PLAN #9 - Overkey]", SERVICE)
    assert_contains(service, "## Phase 1 Implementation Gates", SERVICE)
    for state in REQUIRED_GATE_STATES:
        assert_contains(service, state, SERVICE)
    for decision in REQUIRED_RESOLVED_DECISIONS:
        assert_contains(service, decision, SERVICE)
    for term in REQUIRED_FORBIDDEN_TERMS:
        assert_contains(service, term, SERVICE)

    assert_contains(master, "SDS #9: [Overkey]", MASTER)
    assert_contains(master, "[SUB BUILD PLAN #9 - Overkey]", MASTER)
    assert_contains(master, "First build point remains Phase 1", MASTER)

    assert_contains(crosswalk, "| SDS #9 | [Overkey]", CROSSWALK)
    assert_contains(crosswalk, "[SUB BUILD PLAN #9 - Overkey]", CROSSWALK)
    assert_contains(crosswalk, "[Phase 1: Control-Plane Skeleton]", CROSSWALK)
    assert_contains(crosswalk, "[Phase 8: Data, Storage, and Namespace Platform]", CROSSWALK)

    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)
    assert_contains(tech_stack, "Axum/Tower/Hyper-style Rust HTTP services", TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)
    assert_contains(tech_stack, "Ed25519 signatures", TECH_STACK)
    assert_contains(tech_stack, "BLAKE3/content hashes", TECH_STACK)
    assert_contains(tech_stack, "Overkey-lite", TECH_STACK)

    assert_contains(phase_plan, "Complete SUB BUILD PLAN #9 Phase 1", PHASE_PLAN)
    assert_contains(phase_plan, "Do not introduce the Overkey Rust service crate", PHASE_PLAN)
    assert_contains(phase_progress, "Overkey Phase 1 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)

    assert_contains(
        suite_validator,
        'Path("scripts/validate_overkey_phase1.py")',
        SUITE_VALIDATOR,
    )


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
        try:
            check()
        except AssertionError as exc:
            print(f"{check.__name__} failed: {exc}", file=sys.stderr)
            return 1
    print("Overkey Phase 1 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
