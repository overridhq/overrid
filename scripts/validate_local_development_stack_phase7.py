#!/usr/bin/env python3
"""Validate Local Development Stack Phase 7 reset/seed fixture guarantees."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LOCAL_STACK_LIB = Path("packages/local_stack/src/lib.rs")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_004_local_development_stack.md")
PHASE_PLAN = Path("docs/planning/local_development_stack_phase_07_plan.md")
PHASE_PROGRESS = Path("docs/planning/local_development_stack_phase_07_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

REQUIRED_FILES = [
    LOCAL_STACK_LIB,
    SUB_PLAN,
    PHASE_PLAN,
    PHASE_PROGRESS,
    TECH_STACK,
    SUITE_VALIDATOR,
]

PHASE7_GATE = "phase_7_reset_seed_fixtures"
FIXTURE_VERSION = "fixture:phase7_control_plane_seed.v1"
DETERMINISTIC_SEED = "seed:local_stack:phase7:0001"
EXPECTED_FIXTURE_KINDS = [
    "tenant",
    "actor",
    "key",
    "node",
    "manifest",
    "package",
    "workload",
    "oru_account",
    "seal_ledger_ref",
    "policy_context",
]
EXPECTED_PREREQUISITES = [
    "tenant",
    "actor",
    "key",
    "manifest",
    "idempotency_key",
    "trace_root",
    "pending_work_target",
]
REJECTED_PROFILE_CLASSES = [
    "seed",
    "staging",
    "production_like",
    "federation",
    "public_provider",
    "non_local",
]
DRIFT_PROFILE_REASONS = {
    "nondeterministic-fixture-ids": "local_stack.fixture_id_drift",
    "fixture-schema-drift": "local_stack.fixture_schema_drift",
    "missing-fixture-ref": "local_stack.fixture_ref_missing",
    "extra-fixture-record": "local_stack.fixture_extra_record",
}
RAW_SECRET_MARKERS = (
    "password=",
    "token=",
    "api_key=",
    "private key",
    "-----begin",
)


def read_text(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def assert_true(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def assert_contains(text: str, expected: str, source: Path) -> None:
    assert_true(expected in text, f"{source} missing required text: {expected}")


def run(command: list[str], check: bool = True) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(
        command,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if check and result.returncode != 0:
        raise AssertionError(
            f"{' '.join(command)} failed with {result.returncode}\n"
            f"stdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    return result


def run_cli_json(args: list[str], expect_code: int | None = 0) -> dict[str, Any]:
    result = run(["cargo", "run", "-q", "-p", "overrid-cli", "--", *args], check=False)
    if expect_code is not None and result.returncode != expect_code:
        raise AssertionError(
            f"overrid {' '.join(args)} exited {result.returncode}, expected {expect_code}\n"
            f"stdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as error:
        raise AssertionError(
            f"overrid {' '.join(args)} did not emit JSON: {error}\n{result.stdout}"
        ) from error


def envelope_body(payload: dict[str, Any]) -> dict[str, Any]:
    body = payload.get("result") or payload.get("error")
    assert_true(isinstance(body, dict), "CLI envelope did not contain a result or error body")
    return body


def assert_secret_free(payload: dict[str, Any], context: str) -> None:
    rendered = json.dumps(payload, sort_keys=True).lower()
    for marker in RAW_SECRET_MARKERS:
        assert_true(marker not in rendered, f"{context} exposes raw secret marker {marker}")

    body = envelope_body(payload)
    for event in body.get("lifecycle_events", []):
        assert_true(event["contains_raw_secret"] is False, f"{context} event exposes a raw secret")
        assert_true(event["redaction_summary"] == "secret_free", f"{context} event redaction drifted")
        assert_true(event["local_only"] is True and event["test_only"] is True, f"{context} event scope drifted")


def check_required_files() -> None:
    for path in REQUIRED_FILES:
        assert_true((REPO_ROOT / path).is_file(), f"missing required file: {path}")


def check_docs_and_source() -> None:
    source = read_text(LOCAL_STACK_LIB)
    sub_plan = read_text(SUB_PLAN)
    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    tech_stack = read_text(TECH_STACK)
    suite = read_text(SUITE_VALIDATOR)

    assert_contains(sub_plan, "## Phase 7: Reset, Seed, Fixtures, And Test-Only Identity", SUB_PLAN)
    for item in ["**7.1", "**7.2", "**7.3", "**7.4", "**7.5"]:
        assert_contains(sub_plan, item, SUB_PLAN)
    for expected in [
        "reset safety evidence",
        "deterministic seed fixture evidence",
        "fixture credential isolation",
        "Phase 1 control-plane seed prerequisites",
        "fixture drift detection",
        "scripts/validate_local_development_stack_phase7.py",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)
    assert_contains(phase_progress, "Local Development Stack Phase 7 Progress", PHASE_PROGRESS)
    assert_contains(tech_stack, "Rust-first", TECH_STACK)
    assert_contains(
        suite,
        'Path("scripts/validate_local_development_stack_phase7.py")',
        SUITE_VALIDATOR,
    )

    for expected in [
        "LOCAL_STACK_PHASE7_FIXTURE_GATE",
        "LOCAL_STACK_PHASE7_FIXTURE_VERSION",
        "LOCAL_STACK_PHASE7_DETERMINISTIC_SEED",
        "LocalResetSafetyCheck",
        "LocalSeedFixtureRecord",
        "LocalFixtureIsolationCheck",
        "LocalControlPlaneSeedPrerequisite",
        "LocalFixtureDriftReport",
        "reset_safety_checks_for_profile",
        "seed_fixture_records",
        "fixture_isolation_checks",
        "phase1_seed_prerequisites",
        "fixture_drift_reports_for_profile",
        "render_reset_safety_checks_json",
        "render_seed_fixture_records_json",
        "render_fixture_isolation_checks_json",
        "render_phase1_seed_prerequisites_json",
        "render_fixture_drift_reports_json",
        "local_stack.reset_unsafe_state",
        "local_stack.fixture_drift_detected",
        PHASE7_GATE,
    ]:
        assert_contains(source, expected, LOCAL_STACK_LIB)


def check_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-local-stack", "phase7"])


def check_reset_safety() -> None:
    output = run_cli_json(
        [
            "dev",
            "reset",
            "--json",
            "--dry-run",
            "--trace-id",
            "trace_phase7_reset_safe",
        ]
    )
    assert_true(output["ok"] is True, "local dry-run reset should pass")
    result = output["result"]
    assert_true(result["fixture_phase_gate"] == PHASE7_GATE, "phase 7 gate drifted")
    reset_checks = result["reset_safety_checks"]
    assert_true(len(reset_checks) >= 4, "reset safety checks missing expected targets")
    assert_true(
        all(
            check["safe_to_delete"] is True
            and check["marker_state"] == "verified"
            and check["planned_action"] == "dry_run_delete"
            and ".overrid-local-test-state" in check["marker_ref"]
            for check in reset_checks
        ),
        "safe reset checks must require verified local test markers",
    )
    assert_secret_free(output, "safe reset")

    marker_profiles = {
        "local-missing-reset-marker": "missing",
        "local-stale-reset-marker": "stale",
        "local-inconsistent-reset-marker": "inconsistent",
    }
    for profile, marker_state in marker_profiles.items():
        failed = run_cli_json(
            [
                "dev",
                "reset",
                "--json",
                "--profile",
                profile,
                "--trace-id",
                f"trace_phase7_reset_{marker_state}",
            ],
            expect_code=None,
        )
        assert_true(failed["ok"] is False, f"{profile} reset should fail closed")
        error = failed["error"]
        assert_true(error["reason_code"] == "local_stack.reset_unsafe_state", "reset reason drifted")
        unsafe = [check for check in error["reset_safety_checks"] if check["safe_to_delete"] is False]
        assert_true(len(unsafe) == 1, f"{profile} should report one unsafe reset target")
        assert_true(unsafe[0]["marker_state"] == marker_state, f"{profile} marker state drifted")
        assert_true("reset_safety_checked" in failed["diagnostic_bundle"]["dependency_status"], "reset diagnostic status missing")
        assert_secret_free(failed, f"{profile} reset")


def check_seed_determinism_and_prerequisites() -> None:
    first = run_cli_json(
        [
            "dev",
            "seed",
            "--json",
            "--trace-id",
            "trace_phase7_seed_first",
        ]
    )
    second = run_cli_json(
        [
            "dev",
            "seed",
            "--json",
            "--trace-id",
            "trace_phase7_seed_second",
        ]
    )
    assert_true(first["ok"] is True and second["ok"] is True, "local seed should pass")
    first_result = first["result"]
    second_result = second["result"]
    assert_true(first_result["fixture_phase_gate"] == PHASE7_GATE, "seed phase gate drifted")
    assert_true(
        first_result["seed_fixture_records"] == second_result["seed_fixture_records"],
        "seed fixture records must be deterministic across runs",
    )

    fixtures = first_result["seed_fixture_records"]
    assert_true([fixture["fixture_kind"] for fixture in fixtures] == EXPECTED_FIXTURE_KINDS, "fixture order drifted")
    assert_true([fixture["apply_order"] for fixture in fixtures] == list(range(1, 11)), "fixture apply order drifted")
    assert_true(
        all(
            fixture["fixture_version"] == FIXTURE_VERSION
            and fixture["deterministic_seed"] == DETERMINISTIC_SEED
            and fixture["schema_version"] == "local-development-stack.v0.1"
            and fixture["local_only"] is True
            and fixture["test_only"] is True
            for fixture in fixtures
        ),
        "seed fixture metadata drifted",
    )

    prerequisites = first_result["phase1_seed_prerequisites"]
    assert_true(
        [prerequisite["prerequisite_kind"] for prerequisite in prerequisites] == EXPECTED_PREREQUISITES,
        "Phase 1 prerequisite order drifted",
    )
    assert_true(
        all(
            prerequisite["signing_required"] is True
            and prerequisite["id_preserved_for_phase1"] is True
            and prerequisite["bypasses_signing"] is False
            and prerequisite["local_only"] is True
            and prerequisite["test_only"] is True
            for prerequisite in prerequisites
        ),
        "Phase 1 seed prerequisites must preserve ids and require signing",
    )
    assert_true(
        "deterministic_seed_verified" in first["diagnostic_bundle"]["dependency_status"],
        "deterministic seed status missing",
    )
    assert_true(
        "phase1_seed_prerequisites_ready" in first["diagnostic_bundle"]["dependency_status"],
        "Phase 1 prerequisite status missing",
    )
    assert_secret_free(first, "deterministic seed")


def check_fixture_isolation() -> None:
    output = run_cli_json(
        [
            "dev",
            "status",
            "--json",
            "--trace-id",
            "trace_phase7_isolation",
        ]
    )
    result = output["result"]
    checks = {check["profile_class"]: check for check in result["fixture_isolation_checks"]}
    assert_true(checks["local"]["accepted"] is True, "local fixture profile should be accepted")
    assert_true(checks["ci"]["accepted"] is True, "ci fixture profile should be accepted")
    for profile_class in REJECTED_PROFILE_CLASSES:
        check = checks[profile_class]
        assert_true(check["accepted"] is False, f"{profile_class} fixture profile should be rejected")
        assert_true(check["reason_code"] == "profile.not_local_test", f"{profile_class} rejection reason drifted")
        assert_true(check["contains_raw_secret"] is False, f"{profile_class} check exposes a raw secret")

    for profile_class in REJECTED_PROFILE_CLASSES:
        profile_arg = profile_class.replace("_", "-")
        failed = run_cli_json(
            [
                "dev",
                "seed",
                "--json",
                "--profile",
                profile_arg,
                "--trace-id",
                f"trace_phase7_reject_{profile_class}",
            ],
            expect_code=None,
        )
        assert_true(failed["ok"] is False, f"{profile_class} seed should fail closed")
        assert_true(failed["error"]["reason_code"] == "profile.not_local_test", f"{profile_class} error reason drifted")
        assert_secret_free(failed, f"{profile_class} fixture isolation")


def check_fixture_drift_detection() -> None:
    clean = run_cli_json(
        [
            "dev",
            "seed",
            "--json",
            "--trace-id",
            "trace_phase7_drift_clean",
        ]
    )
    assert_true(clean["ok"] is True, "clean seed should pass drift checks")
    assert_true(
        all(report["drift_detected"] is False and report["blocks_seed"] is False for report in clean["result"]["fixture_drift_reports"]),
        "clean seed should report no fixture drift",
    )
    assert_true(
        "fixture_drift_report_clean" in clean["diagnostic_bundle"]["dependency_status"],
        "clean fixture drift dependency status missing",
    )

    for profile, reason in DRIFT_PROFILE_REASONS.items():
        failed = run_cli_json(
            [
                "dev",
                "seed",
                "--json",
                "--profile",
                profile,
                "--trace-id",
                f"trace_phase7_drift_{profile}",
            ],
            expect_code=None,
        )
        assert_true(failed["ok"] is False, f"{profile} seed should fail on fixture drift")
        error = failed["error"]
        assert_true(error["reason_code"] == "local_stack.fixture_drift_detected", f"{profile} drift error reason changed")
        drifted = [report for report in error["fixture_drift_reports"] if report["drift_detected"]]
        assert_true(drifted, f"{profile} did not report a drifted fixture")
        assert_true(drifted[0]["reason_code"] == reason, f"{profile} drift reason drifted")
        assert_true(drifted[0]["expected_ref"] != drifted[0]["actual_ref"], f"{profile} drift must expose a stable diff")
        assert_true(drifted[0]["blocks_seed"] is True, f"{profile} drift should block seed")
        assert_true(
            "fixture_drift_report_collected" in failed["diagnostic_bundle"]["dependency_status"],
            f"{profile} drift diagnostic status missing",
        )
        assert_secret_free(failed, f"{profile} drift detection")


def main() -> int:
    check_required_files()
    check_docs_and_source()
    check_cargo_tests()
    check_reset_safety()
    check_seed_determinism_and_prerequisites()
    check_fixture_isolation()
    check_fixture_drift_detection()
    print("Local Development Stack Phase 7 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
