#!/usr/bin/env python3
"""Validate SDK Phase 6 workload, manifest, status, and policy helper gates."""

from __future__ import annotations

from pathlib import Path
import re
import subprocess
import sys


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_006_sdk.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/sdk_phase_06_plan.md")
PHASE_PROGRESS = Path("docs/planning/sdk_phase_06_progress.md")
SDK_CARGO = Path("packages/sdk/Cargo.toml")
SDK_LIB = Path("packages/sdk/src/lib.rs")
SDK_WORKLOAD = Path("packages/sdk/src/workload.rs")
SDK_README = Path("packages/sdk/README.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

REQUIRED_PHASE6_SYMBOLS = [
    "pub mod workload;",
    "pub use workload::*;",
    "SDK_PHASE6_CAPABILITY_PROFILE",
    "SDK_PHASE6_WORKLOAD_COMMAND_TYPE",
    "SDK_PHASE6_POLICY_DRY_RUN_ROUTE",
    "SDK_PHASE6_RUNTIME_AUTHORITY_OWNERS",
    "SdkWorkloadClass",
    "SdkResourceDeclaration",
    "SdkDataDeclaration",
    "SdkPolicyDeclaration",
    "SdkEgressDeclaration",
    "SdkSecretReferenceDeclaration",
    "SdkWorkloadManifestInput",
    "SdkWorkloadManifestValidation",
    "SdkWorkloadManifest",
    "build_workload_manifest",
    "SdkWorkloadSubmissionInput",
    "SdkWorkloadSubmission",
    "submit_workload",
    "decode_workload_submission_response",
    "SdkWorkloadReadKind",
    "build_workload_read_request",
    "build_command_status_request",
    "build_workload_status_request",
    "build_job_status_request",
    "build_workload_result_request",
    "build_cancellation_status_request",
    "SdkWorkloadServiceState",
    "SdkWorkloadStatusRecord",
    "build_workload_cancellation_request",
    "SdkPolicyDryRunInput",
    "build_policy_dry_run_request",
    "decode_policy_dry_run_result",
    "SdkPhase6AuthorityBoundary",
    "sdk_phase6_authority_review",
    "validate_phase6_authority_review",
    "SdkPhase6Error",
]

REQUIRED_PHASE6_TESTS = [
    "phase6_manifest_builder_validates_local_shape_without_runtime_acceptance",
    "phase6_submit_workload_wraps_manifest_command_signing_and_overgate_submission",
    "phase6_status_result_and_cancellation_helpers_preserve_service_states",
    "phase6_policy_dry_run_fails_closed_and_never_caches_policy_truth",
    "phase6_authority_review_blocks_runtime_ownership_inside_sdk",
    "phase6_policy_and_status_errors_preserve_stable_reason_refs",
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def run(command: list[str]) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(command, cwd=REPO_ROOT, text=True, capture_output=True)
    if result.returncode != 0:
        sys.stdout.write(result.stdout)
        sys.stderr.write(result.stderr)
        raise AssertionError(f"Command failed: {' '.join(command)}")
    return result


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


def validate_sub_plan_phase6() -> None:
    text = read(SUB_PLAN)
    phase_6 = section(text, "## Phase 6: Workload, Manifest, Status, And Policy Helpers")
    for item in range(1, 6):
        assert_contains(phase_6, f"**6.{item} ", SUB_PLAN)
    for work_item in re.finditer(
        r"- \*\*6\.[1-5] .+?(?=\n- \*\*6\.|\n## Phase 7:)",
        phase_6,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")
    for expected in [
        "buildWorkloadManifest(input)",
        "submitWorkload(manifest)",
        "getCommandStatus",
        "dryRunPolicy(input)",
        "runtime authority outside the SDK",
    ]:
        assert_contains(phase_6, expected, SUB_PLAN)


def validate_phase_docs() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in [
        "Complete the Phase 6 gate",
        "workload manifest builders",
        "workload submission descriptors",
        "status/result/cancellation readers",
        "policy dry-run helpers",
        "runtime-authority boundary",
        "cargo test -p overrid-sdk",
        "Keep secret refs reference-only",
    ]:
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in [
        "Loaded Docdex profile memory and repo memory",
        "Docdex impact graph",
        "Local delegation was attempted",
        "Saved the repo-scoped Phase 6 session context",
        "Validation Evidence",
    ]:
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_sdk_code_and_readme() -> None:
    cargo = read(SDK_CARGO)
    lib = read(SDK_LIB)
    workload = read(SDK_WORKLOAD)
    readme = read(SDK_README)
    combined = f"{lib}\n{workload}"

    for expected in REQUIRED_PHASE6_SYMBOLS + REQUIRED_PHASE6_TESTS:
        assert_contains(combined, expected, SDK_WORKLOAD)
    for expected in [
        "runtime_acceptance_claimed: false",
        "runtime_authority_claimed: false",
        "runtime_completion_invented: false",
        'acceptance.pending_state.contains("queue")',
        "pending_review",
        "sdk_invented_terminal_state: false",
        "cacheable_as_policy_truth: false",
        "mutates_runtime_state: false",
        "requires_service_state_for_cancelled: true",
        "ServiceEvidenceRequired",
        "ForbiddenEgress",
        "MalformedSecretRef",
        "UnsupportedWorkloadClass",
        "Overgate",
        "Overguard",
        "Overqueue",
        "Overrun",
        "Overmeter",
        "Overwatch",
    ]:
        assert_contains(workload, expected, SDK_WORKLOAD)
    if 'acceptance.pending_state.contains("pending")' in workload:
        raise AssertionError(
            f"{SDK_WORKLOAD} should not treat generic pending states as pending-queue proof"
        )
    for expected in [
        "WorkloadManifest",
        "WorkloadSubmission",
        "StatusReaders",
        "PolicyDryRun",
        '"workload_manifest"',
        '"workload_submission"',
        '"status_readers"',
        '"policy_dry_run"',
    ]:
        assert_contains(lib, expected, SDK_LIB)
    for expected in [
        "## Phase 6 Workload, Manifest, Status, And Policy Helpers",
        "`build_workload_manifest()`",
        "`submit_workload()`",
        "`decode_workload_submission_response()`",
        "`build_command_status_request()`",
        "`build_workload_status_request()`",
        "`build_job_status_request()`",
        "`build_workload_result_request()`",
        "`build_cancellation_status_request()`",
        "`SdkWorkloadStatusRecord::from_service()`",
        "`build_workload_cancellation_request()`",
        "`build_policy_dry_run_request()`",
        "`decode_policy_dry_run_result()`",
        "`sdk_phase6_authority_review()`",
        "`validate_phase6_authority_review()`",
        "never cache dry-run output as policy truth",
    ]:
        assert_contains(readme, expected, SDK_README)
    if "[dependencies]\noverrid-contracts" not in cargo:
        raise AssertionError(f"{SDK_CARGO} should remain dependency-light for Phase 6")


def validate_tech_stack_alignment() -> None:
    tech_stack = read(TECH_STACK)
    workload = read(SDK_WORKLOAD)
    for expected in [
        "| SDKs | Generated Rust SDK first",
        "Signed command envelopes",
        "idempotency keys",
        "trace ids",
        "stable reason codes",
        "Overrid-owned storage abstraction",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)
    for forbidden in [
        "postgres",
        "redis",
        "kafka",
        "nats",
        "s3",
        "typescript",
        "blockchain",
        "nft",
        "pricing",
        "revenue",
        "customer-count",
    ]:
        if forbidden in workload.lower():
            raise AssertionError(f"{SDK_WORKLOAD} contains forbidden stack term: {forbidden}")


def validate_wrapper() -> None:
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_sdk_phase6.py")', VALIDATION_WRAPPER)


def validate_rust_tests() -> None:
    result = run(["cargo", "test", "-p", "overrid-sdk", "phase6_"])
    if "test result: ok" not in result.stdout and "test result: ok" not in result.stderr:
        raise AssertionError("Phase 6 Rust tests did not report success")


def main() -> int:
    validate_sub_plan_phase6()
    validate_phase_docs()
    validate_sdk_code_and_readme()
    validate_tech_stack_alignment()
    validate_wrapper()
    validate_rust_tests()
    print("SDK Phase 6 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
