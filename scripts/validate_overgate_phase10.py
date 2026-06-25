#!/usr/bin/env python3
"""Validate Overgate Phase 10 closure and downstream handoff artifacts."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any
from urllib.parse import unquote


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_008_overgate.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
MASTER_PLAN = Path("docs/build_plan/master_plan.md")
CROSSWALK = Path("docs/build_plan/service_catalog_alignment.md")
SDS = Path("docs/sds/control_plane/overgate.md")
SERVICE = Path("docs/service_catalog/control_plane/overgate.md")
README = Path("packages/overgate/README.md")
PHASE_PLAN = Path("docs/planning/overgate_phase_10_plan.md")
PHASE_PROGRESS = Path("docs/planning/overgate_phase_10_progress.md")
STRUCTURE_ARTIFACT = Path("packages/overgate/handoff/phase10/structure_validation.valid.json")
ALIGNMENT_ARTIFACT = Path("packages/overgate/handoff/phase10/alignment_checklist.valid.json")
HANDOFF_ARTIFACT = Path("packages/overgate/handoff/phase10/downstream_handoff_rules.valid.json")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

REQUIRED_STRUCTURE_GATES = {
    "title_prefix",
    "attached_sds_link",
    "phase_headings",
    "work_item_structure",
    "exit_gate",
    "local_markdown_links",
}
REQUIRED_ALIGNMENT_DOMAINS = {
    "tech_stack",
    "master_plan",
    "crosswalk",
    "sds",
    "service_catalog",
}
REQUIRED_ALIGNMENT_CHECKS = {
    "rust_first_control_plane_service",
    "signed_command_envelopes",
    "canonical_json_plus_json_schema",
    "native_overqueue_boundary",
    "native_overwatch_boundary",
    "no_conventional_cloud_product_boundary",
    "no_downstream_finality_or_lifecycle_authority",
}
REQUIRED_CONSUMERS = {
    "overpass",
    "overtenant",
    "overkey",
    "overregistry",
    "overwatch",
    "overqueue",
    "overguard",
    "overmeter",
    "oru_account_service",
    "seal_ledger",
    "overpack",
    "overrun",
    "sdk",
    "cli",
    "admin_developer_ui",
    "adapters",
    "native_apps",
    "mobile_clients",
    "grid_resident_system_services",
}
REQUIRED_OUTPUTS = {
    "admission_record",
    "trace_id",
    "request_id",
    "command_ref",
    "audit_refs",
    "idempotency_ref",
    "quota_ref",
    "policy_decision_ref",
    "forwarding_record",
    "client_denial_refs",
}
SECRET_MARKERS = (
    "password=",
    "token=",
    "secret=",
    "api_key=",
    "private key",
    "-----begin",
    "raw_secret_value",
    "raw_private_key",
    "private_payload_value",
)
FORBIDDEN_TRUE_MARKERS = (
    '"runtime_authority": true',
    '"overgate_runtime_authority": true',
    '"direct_downstream_state_write": true',
    '"owner_authority_retained": false',
    '"finality_stays_with_owner": false',
)


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(read(path))


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} is missing expected text: {expected}")


def assert_true(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


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


def assert_no_markdown_link_gaps(path: Path) -> None:
    text = read(path)
    for match in re.finditer(r"\[[^\]]+\]\(([^)#][^)#]+)(?:#[^)]+)?\)", text):
        raw_target = unquote(match.group(1))
        if "://" in raw_target or raw_target.startswith("mailto:"):
            continue
        target = raw_target.split("#", 1)[0]
        if not target or target.startswith("#"):
            continue
        target_path = (REPO_ROOT / path.parent / target).resolve()
        if not target_path.exists():
            raise AssertionError(f"{path} has missing local markdown link target: {raw_target}")


def assert_artifact_safe(path: Path) -> None:
    lowered = read(path).lower()
    for marker in SECRET_MARKERS:
        if marker in lowered:
            raise AssertionError(f"{path} contains secret marker: {marker}")
    for marker in FORBIDDEN_TRUE_MARKERS:
        if marker in lowered:
            raise AssertionError(f"{path} contains forbidden authority marker: {marker}")


def validate_sub_plan_phase10() -> None:
    text = read(SUB_PLAN)
    assert_contains(text, "# SUB BUILD PLAN #8 - Overgate", SUB_PLAN)
    assert_contains(
        text,
        "Attached SDS: [docs/sds/control_plane/overgate.md](../sds/control_plane/overgate.md)",
        SUB_PLAN,
    )
    for phase in range(1, 11):
        assert_contains(text, f"## Phase {phase}:", SUB_PLAN)

    phase_10 = section(
        text,
        "## Phase 10: Validation, Documentation Alignment, And Downstream Handoff",
    )
    for item in range(1, 6):
        assert_contains(phase_10, f"**10.{item} ", SUB_PLAN)

    work_items = list(
        re.finditer(
            r"- \*\*10\.[1-5] .+?(?=\n- \*\*10\.|\n### Phase 10 Gate Outputs|\n## Alignment Review|\n## Exit Gate|\Z)",
            phase_10,
            re.S,
        )
    )
    if len(work_items) != 5:
        raise AssertionError("Phase 10 must contain five structured work items")
    for work_item in work_items:
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")

    for expected in [
        "### Phase 10 Gate Outputs",
        "packages/overgate/handoff/phase10/structure_validation.valid.json",
        "packages/overgate/handoff/phase10/alignment_checklist.valid.json",
        "packages/overgate/handoff/phase10/downstream_handoff_rules.valid.json",
        "Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, Overguard, Overmeter, ORU, Seal Ledger, Overpack, Overrun, SDK, CLI, admin UI, adapters, native apps, mobile clients, and grid-resident system services",
        "without moving runtime authority into Overgate",
        "## Exit Gate",
    ]:
        assert_contains(text, expected, SUB_PLAN)
    assert_no_markdown_link_gaps(SUB_PLAN)


def validate_phase_docs() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in [
        "Complete SUB BUILD PLAN #8 Phase 10",
        "packages/overgate/handoff/phase10",
        "scripts/validate_overgate_phase10.py",
        "no downstream state ownership",
        "PYTHONDONTWRITEBYTECODE=1 python3 -B scripts/validate_overgate_phase10.py",
        "docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid --target packages/overgate",
    ]:
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in [
        "Loaded Docdex profile memory and repo memory",
        "Docdex impact graph",
        "Local delegation was attempted",
        "Validation Evidence",
        "Blockers",
    ]:
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_alignment_sources() -> None:
    tech_stack = read(TECH_STACK)
    master = read(MASTER_PLAN)
    crosswalk = read(CROSSWALK)
    sds = read(SDS)
    service = read(SERVICE)

    for expected in [
        "Rust-first infrastructure stack",
        "Axum/Tower/Hyper-style Rust HTTP services",
        "Signed command envelopes",
        "Canonical JSON plus JSON Schema",
        "Native Overqueue durable state and events",
        "Overwatch as the authoritative audit/evidence layer",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)
    for expected in [
        "SDS #8: [Overgate]",
        "SUB BUILD PLAN #8 - Overgate",
        "First build point remains Phase 1, with Phase 0 prerequisites and later hardening through policy, metering, product integration, and grid-resident operation.",
        "Phase 0 through Phase 13",
    ]:
        assert_contains(master, expected, MASTER_PLAN)
    for expected in [
        "| SDS #8 | [Overgate]",
        "sub_build_plan_008_overgate.md",
        "Phase 1: Control-Plane Skeleton",
        "later hardening through policy, metering, product integration, and grid-resident operation",
    ]:
        assert_contains(crosswalk, expected, CROSSWALK)
    for expected in [
        "Sub-build plan",
        "SUB BUILD PLAN #8 - Overgate",
        "Downstream services own their domain state. Overgate must not write their private records directly.",
        "Forward accepted commands through explicit downstream APIs or queues, never by direct storage writes.",
    ]:
        assert_contains(sds, expected, SDS)
    for expected in [
        "SUB BUILD PLAN #8",
        "Overgate must not write private downstream records directly.",
        "Handoff",
        "Overgate is the required entry point for SDK, CLI, admin UI, adapters, and native apps.",
    ]:
        assert_contains(service, expected, SERVICE)

    for path in [SDS, SERVICE, SUB_PLAN]:
        assert_no_markdown_link_gaps(path)


def validate_readme_and_wrapper() -> None:
    readme = read(README)
    for expected in [
        "Phase 10 Validation, Documentation Alignment, And Downstream Handoff",
        "packages/overgate/handoff/phase10/structure_validation.valid.json",
        "packages/overgate/handoff/phase10/alignment_checklist.valid.json",
        "packages/overgate/handoff/phase10/downstream_handoff_rules.valid.json",
        "scripts/validate_overgate_phase10.py",
        "without moving owner-service runtime authority into Overgate",
    ]:
        assert_contains(readme, expected, README)
    wrapper = read(SUITE_VALIDATOR)
    assert_contains(wrapper, 'Path("scripts/validate_overgate_phase10.py")', SUITE_VALIDATOR)


def validate_structure_artifact() -> None:
    sub_plan = read(SUB_PLAN)
    artifact = read_json(STRUCTURE_ARTIFACT)
    assert_true(
        artifact.get("artifact_id") == "overgate_phase10_structure_validation",
        "wrong structure artifact id",
    )
    assert_true(artifact.get("runtime_authority") is False, "structure artifact authority drifted")
    gates = artifact.get("gates")
    assert_true(isinstance(gates, list), "structure gates must be a list")
    assert_true({entry["gate"] for entry in gates} == REQUIRED_STRUCTURE_GATES, "structure gate set drifted")
    for entry in gates:
        assert_true(entry["source_path"] == str(SUB_PLAN), f"structure gate source drifted: {entry['gate']}")
        assert_contains(sub_plan, entry["required_marker"], SUB_PLAN)
        assert_true(entry["passed"] is True, f"structure gate did not pass: {entry['gate']}")
        assert_true(entry["runtime_authority"] is False, f"structure gate authority drifted: {entry['gate']}")


def validate_alignment_artifact() -> None:
    artifact = read_json(ALIGNMENT_ARTIFACT)
    assert_true(
        artifact.get("artifact_id") == "overgate_phase10_alignment_checklist",
        "wrong alignment artifact id",
    )
    assert_true(
        artifact.get("master_alignment") == "phase1_control_plane_with_phase0_prerequisites_and_later_hardening",
        "master alignment marker drifted",
    )
    assert_true(artifact.get("runtime_authority") is False, "alignment artifact authority drifted")
    assert_true(set(artifact.get("checks", [])) == REQUIRED_ALIGNMENT_CHECKS, "alignment check set drifted")
    items = artifact.get("items")
    assert_true(isinstance(items, list), "alignment items must be a list")
    assert_true({entry["domain"] for entry in items} == REQUIRED_ALIGNMENT_DOMAINS, "alignment domain set drifted")
    for entry in items:
        for key in (
            "preserves_phase_order",
            "preserves_rust_first",
            "preserves_signed_envelopes",
            "owner_authority_retained",
            "no_conventional_cloud_boundary",
        ):
            assert_true(entry[key] is True, f"alignment item must keep {key}=true: {entry['domain']}")
        assert_true(entry["runtime_authority"] is False, f"alignment item authority drifted: {entry['domain']}")
        source_path = Path(entry["source_path"])
        assert_contains(read(source_path), entry["expected_marker"], source_path)


def validate_handoff_artifact() -> None:
    artifact = read_json(HANDOFF_ARTIFACT)
    assert_true(
        artifact.get("artifact_id") == "overgate_phase10_downstream_handoff_rules",
        "wrong handoff artifact id",
    )
    assert_true(
        artifact.get("runtime_authority_profile") == "owner_services_not_overgate",
        "runtime authority profile drifted",
    )
    assert_true(artifact.get("runtime_authority") is False, "handoff artifact authority drifted")
    assert_true(set(artifact.get("admission_outputs", [])) == REQUIRED_OUTPUTS, "admission output set drifted")
    consumers = artifact.get("consumers")
    assert_true(isinstance(consumers, list), "handoff consumers must be a list")
    assert_true({entry["consumer"] for entry in consumers} == REQUIRED_CONSUMERS, "handoff consumer set drifted")
    for entry in consumers:
        assert_true(entry.get("consumed_outputs"), f"missing consumed outputs for {entry['consumer']}")
        assert_true(
            set(entry["consumed_outputs"]) <= REQUIRED_OUTPUTS,
            f"unknown consumed output for {entry['consumer']}",
        )
        assert_true(entry.get("required_handoff_ref"), f"missing handoff ref for {entry['consumer']}")
        for key in ("owner_authority_retained", "finality_stays_with_owner"):
            assert_true(entry[key] is True, f"{entry['consumer']} must keep {key}=true")
        for key in ("overgate_runtime_authority", "direct_downstream_state_write"):
            assert_true(entry[key] is False, f"{entry['consumer']} must keep {key}=false")


def validate_artifact_safety() -> None:
    for path in [STRUCTURE_ARTIFACT, ALIGNMENT_ARTIFACT, HANDOFF_ARTIFACT]:
        assert_artifact_safe(path)


def main() -> int:
    validate_sub_plan_phase10()
    validate_phase_docs()
    validate_alignment_sources()
    validate_readme_and_wrapper()
    validate_structure_artifact()
    validate_alignment_artifact()
    validate_handoff_artifact()
    validate_artifact_safety()
    print("Overgate Phase 10 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as exc:
        print(f"Overgate Phase 10 validation failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
