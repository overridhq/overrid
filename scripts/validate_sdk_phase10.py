#!/usr/bin/env python3
"""Validate SDK Phase 10 validation, documentation, and handoff gates."""

from __future__ import annotations

import json
from pathlib import Path
import re
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_006_sdk.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
MASTER_PLAN = Path("docs/build_plan/master_plan.md")
CROSSWALK = Path("docs/build_plan/service_catalog_alignment.md")
SDK_SDS = Path("docs/sds/foundation/sdk.md")
SDK_SERVICE = Path("docs/service_catalog/foundation/sdk.md")
PHASE_PLAN = Path("docs/planning/sdk_phase_10_plan.md")
PHASE_PROGRESS = Path("docs/planning/sdk_phase_10_progress.md")
SDK_CARGO = Path("packages/sdk/Cargo.toml")
SDK_LIB = Path("packages/sdk/src/lib.rs")
SDK_HANDOFF = Path("packages/sdk/src/handoff.rs")
SDK_README = Path("packages/sdk/README.md")
STRUCTURE_ARTIFACT = Path("packages/sdk/handoff/phase10/structure_validation.valid.json")
ALIGNMENT_ARTIFACT = Path("packages/sdk/handoff/phase10/alignment_checklist.valid.json")
HANDOFF_ARTIFACT = Path("packages/sdk/handoff/phase10/downstream_handoff_rules.valid.json")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

REQUIRED_STRUCTURE_GATES = {
    "title_prefix",
    "attached_sds_link",
    "phase_headings",
    "work_item_structure",
    "design_output_validation_fields",
    "exit_gate",
}
REQUIRED_ALIGNMENT_DOMAINS = {
    "tech_stack",
    "master_plan",
    "sds",
    "service_catalog",
    "crosswalk",
}
REQUIRED_ALIGNMENT_CHECKS = {
    "rust_first_sdk_generation",
    "typescript_web_generated_second_target",
    "language_neutral_schema_authority",
    "overgate_only_mutating_calls",
    "credential_provider_signing",
}
REQUIRED_DOWNSTREAM_CONSUMERS = {
    "cli",
    "admin_developer_ui",
    "docdex_adapter",
    "mcoda_adapter",
    "codali_adapter",
    "native_apps",
    "mobile_services",
    "overpack_deployment",
    "accounting_services",
    "policy_services",
    "future_language_bindings",
}
ALLOWED_PUBLIC_ROUTE_PREFIXES = (
    "/v1/overgate/",
    "/v1/admin/",
    "/v1/control-plane/",
    "/v1/accounting/",
    "/v1/policy/",
    "/v1/overpack/",
)
FORBIDDEN_ARTIFACT_MARKERS = [
    "raw_private_key",
    "private_key_value",
    "bearer_token_value",
    "seed_phrase_value",
    '"signature_value"',
    '"raw_request_body"',
    "private_payload_value",
    "fixture_credential_material",
    '"runtime_authority": true',
    '"allowed_to_bypass_owner_services": true',
    '"allowed_to_store_private_keys": true',
    '"direct_storage_access": true',
    '"client_side_policy_truth": true',
    '"client_side_accounting_truth": true',
    '"owner_authority_retained": false',
    '"generated_contracts_required": false',
    '"capability_checks_required": false',
    "postgresql",
    "redis",
    "s3://",
    "minio",
    "kafka",
    "nats",
    "blockchain",
    "nft",
    "pricing",
    "revenue",
    "customer_count",
    "customer-count",
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(read(path))


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=REPO_ROOT, text=True, capture_output=True)
    if result.returncode != 0:
        sys.stdout.write(result.stdout)
        sys.stderr.write(result.stderr)
        raise AssertionError(f"Command failed: {' '.join(command)}")


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


def assert_true(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def assert_no_forbidden_markers(text: str, source: Path) -> None:
    lowered = text.lower()
    for marker in FORBIDDEN_ARTIFACT_MARKERS:
        if marker in lowered:
            raise AssertionError(f"{source} contains forbidden Phase 10 marker: {marker}")


def assert_no_markdown_link_gaps(path: Path) -> None:
    text = read(path)
    for match in re.finditer(r"\[[^\]]+\]\(([^)#][^)#]+)(?:#[^)]+)?\)", text):
        raw_target = match.group(1)
        if "://" in raw_target or raw_target.startswith("mailto:"):
            continue
        target = raw_target.split("#", 1)[0]
        if not target or target.startswith("#"):
            continue
        target_path = (REPO_ROOT / path.parent / target).resolve()
        if not target_path.exists():
            raise AssertionError(f"{path} has missing local markdown link target: {raw_target}")


def validate_sub_plan_phase10() -> None:
    text = read(SUB_PLAN)
    assert_contains(text, "# SUB BUILD PLAN #6 - SDK", SUB_PLAN)
    assert_contains(
        text,
        "Attached SDS: [docs/sds/foundation/sdk.md](../sds/foundation/sdk.md)",
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
            r"- \*\*10\.[1-5] .+?(?=\n- \*\*10\.|\n## Alignment Review|\n## Exit Gate|\Z)",
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
        "title prefix",
        "attached SDS link",
        "phase headings 1 through 10",
        "Rust-first SDK generation",
        "TypeScript/web second target",
        "Overgate-only mutating calls",
        "credential-provider signing",
        "Phase 1 foundation SDK with Phase 6 product-integration hardening",
        "CLI, admin/developer UI, adapters, Docdex, Mcoda, Codali",
        "without bypassing owning services",
        "## Exit Gate",
    ]:
        assert_contains(text, expected, SUB_PLAN)


def validate_phase_docs() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)

    for expected in [
        "Complete the Phase 10 gate",
        "packages/sdk/src/handoff.rs",
        "packages/sdk/handoff/phase10",
        "scripts/validate_sdk_phase10.py",
        "Do not add new SDK dependencies",
        "cargo test -p overrid-sdk phase10_",
        "docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid --target scripts/validate_sdk_phase10.py",
    ]:
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in [
        "Loaded Docdex profile memory and repo memory",
        "Docdex impact graph",
        "Local delegation was attempted",
        "Saved the repo-scoped Phase 10 session scope",
        "Validation Evidence",
    ]:
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_alignment_sources() -> None:
    tech_stack = read(TECH_STACK)
    master = read(MASTER_PLAN)
    crosswalk = read(CROSSWALK)
    sds = read(SDK_SDS)
    service = read(SDK_SERVICE)

    for expected in [
        "Rust-first infrastructure stack",
        "TypeScript should be used for web-facing client surfaces",
        "not as the core runtime for the grid",
        "Third-party software may be used as libraries",
        "It must not become the product boundary for core Overrid primitives.",
        "Explicit Non-Choices",
        "Native Overqueue durable state and events",
        "Native Overstore",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)
    for expected in [
        "SDS #6: [SDK]",
        "SUB BUILD PLAN #6 - SDK",
        "First build point remains Phase 1, with Phase 0 prerequisites and Phase 6 product-integration hardening.",
        "Phase 0 through Phase 13",
    ]:
        assert_contains(master, expected, MASTER_PLAN)
    for expected in [
        "| SDS #6 | [SDK]",
        "sub_build_plan_006_sdk.md",
        "Phase 1: Control-Plane Skeleton",
        "Phase 6: First Product Integration",
        "with Phase 0 prerequisites",
    ]:
        assert_contains(crosswalk, expected, CROSSWALK)
    for expected in [
        "The SDK is a versioned developer package, not a standalone runtime service.",
        "sub_build_plan_006_sdk.md",
        "Phase 1: Control-Plane Skeleton",
        "Phase 6: First Product Integration",
        "must not persist private keys or long-lived secrets",
        "must never infer `accepted` or `completed` without a service response",
    ]:
        assert_contains(sds, expected, SDK_SDS)
    for expected in [
        "SDK Implementation Plan",
        "SUB BUILD PLAN #6 - SDK",
        "Phase 1: Control-Plane Skeleton",
        "Phase 6: First Product Integration",
        "Treat the SDK as a versioned client package, not a runtime authority or deployed service.",
        "Use Overkey-compatible credential references for signing without storing private keys in SDK state.",
    ]:
        assert_contains(service, expected, SDK_SERVICE)

    for path in [SUB_PLAN, SDK_SDS, SDK_SERVICE]:
        assert_no_markdown_link_gaps(path)


def validate_sdk_code_and_readme() -> None:
    cargo = read(SDK_CARGO)
    lib = read(SDK_LIB)
    handoff = read(SDK_HANDOFF)
    readme = read(SDK_README)
    combined = f"{lib}\n{handoff}"

    for expected in [
        "pub mod handoff;",
        "pub use handoff::*;",
        "SDK_PHASE10_CAPABILITY_PROFILE",
        "SDK_PHASE10_HANDOFF_ROOT",
        "SDK_PHASE10_STRUCTURE_VALIDATION_PATH",
        "SDK_PHASE10_ALIGNMENT_CHECKLIST_PATH",
        "SDK_PHASE10_DOWNSTREAM_HANDOFF_RULES_PATH",
        "SdkPhase10StructureGate",
        "SdkPhase10StructureValidation",
        "sdk_phase10_structure_validation",
        "validate_phase10_structure_validation",
        "SdkPhase10AlignmentDomain",
        "SdkPhase10AlignmentChecklistItem",
        "sdk_phase10_alignment_checklist",
        "validate_phase10_alignment_checklist",
        "SdkPhase10DownstreamConsumerKind",
        "SdkPhase10DownstreamHandoffRule",
        "sdk_phase10_downstream_handoff_rules",
        "validate_phase10_downstream_handoff_rules",
        "validate_phase10_artifact_files",
        "SdkPhase10Error",
        "phase10_structure_validation_covers_sub_build_plan_exit_gate",
        "phase10_alignment_checklist_preserves_master_and_tech_stack",
        "phase10_downstream_handoff_rules_keep_authority_in_owner_services",
        "phase10_artifact_files_match_declared_paths_and_markers",
    ]:
        assert_contains(combined, expected, SDK_HANDOFF)
    for expected in [
        'include_str!("../handoff/phase10/structure_validation.valid.json")',
        'include_str!("../handoff/phase10/alignment_checklist.valid.json")',
        'include_str!("../handoff/phase10/downstream_handoff_rules.valid.json")',
        "owner_authority_retained: true",
        "allowed_to_bypass_owner_services: false",
        "allowed_to_store_private_keys: false",
        "runtime_authority: false",
    ]:
        assert_contains(handoff, expected, SDK_HANDOFF)
    if any(dep in cargo for dep in ("serde", "tokio", "reqwest", "axum")):
        raise AssertionError("SDK Phase 10 must not add new runtime dependencies")
    for expected in [
        "Phase 10 Validation, Documentation Alignment, And Downstream Handoff",
        "sdk_phase10_structure_validation()",
        "validate_phase10_alignment_checklist()",
        "sdk_phase10_downstream_handoff_rules()",
        "validate_phase10_artifact_files()",
        "scripts/validate_sdk_phase10.py",
        "Phase 10 adds no new SDK dependencies",
    ]:
        assert_contains(readme, expected, SDK_README)


def validate_structure_artifact() -> None:
    sub_plan = read(SUB_PLAN)
    artifact = read_json(STRUCTURE_ARTIFACT)
    if artifact.get("artifact_id") != "sdk_phase10_structure_validation":
        raise AssertionError("wrong Phase 10 structure artifact id")
    if artifact.get("runtime_authority") is not False:
        raise AssertionError("structure artifact must keep runtime_authority=false")
    gates = artifact.get("gates")
    assert_true(isinstance(gates, list), "structure artifact gates must be a list")
    if {entry["gate"] for entry in gates} != REQUIRED_STRUCTURE_GATES:
        raise AssertionError("Phase 10 structure gate set drifted")
    for entry in gates:
        if entry["source_path"] != str(SUB_PLAN):
            raise AssertionError(f"structure gate source drifted: {entry['gate']}")
        if entry["required_marker"] not in sub_plan:
            raise AssertionError(
                f"structure gate marker is not present in {SUB_PLAN}: {entry['gate']}"
            )
        for key in ("passed",):
            if entry[key] is not True:
                raise AssertionError(f"structure gate must keep {key}=true: {entry['gate']}")
        if entry["runtime_authority"] is not False:
            raise AssertionError(f"structure gate became runtime authority: {entry['gate']}")


def validate_alignment_artifact() -> None:
    artifact = read_json(ALIGNMENT_ARTIFACT)
    if artifact.get("artifact_id") != "sdk_phase10_alignment_checklist":
        raise AssertionError("wrong Phase 10 alignment artifact id")
    if artifact.get("master_alignment") != "phase1_with_phase0_prerequisites_and_phase6_hardening":
        raise AssertionError("SDK master-plan alignment marker drifted")
    if set(artifact.get("checks", [])) != REQUIRED_ALIGNMENT_CHECKS:
        raise AssertionError("Phase 10 alignment check set drifted")
    items = artifact.get("items")
    assert_true(isinstance(items, list), "alignment artifact items must be a list")
    if {entry["domain"] for entry in items} != REQUIRED_ALIGNMENT_DOMAINS:
        raise AssertionError("Phase 10 alignment domain set drifted")
    for entry in items:
        for key in (
            "preserves_phase_order",
            "preserves_rust_first",
            "generated_bindings_only",
            "owner_authority_retained",
            "no_conventional_cloud_boundary",
        ):
            if entry[key] is not True:
                raise AssertionError(f"alignment item must keep {key}=true: {entry['domain']}")
        if entry["runtime_authority"] is not False:
            raise AssertionError(f"alignment item became runtime authority: {entry['domain']}")
        source_path = Path(entry["source_path"])
        assert_contains(read(source_path), entry["expected_marker"], source_path)


def validate_downstream_handoff_artifact() -> None:
    artifact = read_json(HANDOFF_ARTIFACT)
    if artifact.get("artifact_id") != "sdk_phase10_downstream_handoff_rules":
        raise AssertionError("wrong Phase 10 downstream handoff artifact id")
    if artifact.get("runtime_authority_profile") != "owner_services_not_sdk":
        raise AssertionError("runtime authority profile drifted")
    if artifact.get("runtime_authority") is not False:
        raise AssertionError("handoff artifact must keep runtime_authority=false")
    consumers = artifact.get("consumers")
    assert_true(isinstance(consumers, list), "handoff artifact consumers must be a list")
    if {entry["consumer"] for entry in consumers} != REQUIRED_DOWNSTREAM_CONSUMERS:
        raise AssertionError("Phase 10 downstream consumer set drifted")
    for entry in consumers:
        if not entry.get("allowed_sdk_primitives"):
            raise AssertionError(f"missing SDK primitives for {entry['consumer']}")
        routes = entry.get("required_public_api_families")
        if not routes:
            raise AssertionError(f"missing public API families for {entry['consumer']}")
        if not all(route.startswith(ALLOWED_PUBLIC_ROUTE_PREFIXES) for route in routes):
            raise AssertionError(f"unsafe public route for {entry['consumer']}: {routes}")
        for key in (
            "owner_authority_retained",
            "generated_contracts_required",
            "capability_checks_required",
        ):
            if entry[key] is not True:
                raise AssertionError(f"{entry['consumer']} must keep {key}=true")
        for key in (
            "allowed_to_bypass_owner_services",
            "allowed_to_store_private_keys",
            "runtime_authority",
        ):
            if entry[key] is not False:
                raise AssertionError(f"{entry['consumer']} must keep {key}=false")


def validate_artifact_safety() -> None:
    for path in [STRUCTURE_ARTIFACT, ALIGNMENT_ARTIFACT, HANDOFF_ARTIFACT, SDK_README]:
        assert_no_forbidden_markers(read(path), path)


def validate_wrapper() -> None:
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_sdk_phase10.py")', VALIDATION_WRAPPER)


def validate_rust_tests() -> None:
    run(["cargo", "test", "-p", "overrid-sdk", "phase10_"])


def main() -> int:
    validate_sub_plan_phase10()
    validate_phase_docs()
    validate_alignment_sources()
    validate_sdk_code_and_readme()
    validate_structure_artifact()
    validate_alignment_artifact()
    validate_downstream_handoff_artifact()
    validate_artifact_safety()
    validate_wrapper()
    validate_rust_tests()
    print("SDK Phase 10 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
