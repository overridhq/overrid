#!/usr/bin/env python3
"""Validate Local Development Stack Phase 9 diagnostics and CI evidence."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LOCAL_STACK_LIB = Path("packages/local_stack/src/lib.rs")
CLI_PARSER = Path("packages/cli/src/parser.rs")
CLI_RUNNER = Path("packages/cli/src/runner.rs")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_004_local_development_stack.md")
PHASE_PLAN = Path("docs/planning/local_development_stack_phase_09_plan.md")
PHASE_PROGRESS = Path("docs/planning/local_development_stack_phase_09_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

REQUIRED_FILES = [
    LOCAL_STACK_LIB,
    CLI_PARSER,
    CLI_RUNNER,
    SUB_PLAN,
    PHASE_PLAN,
    PHASE_PROGRESS,
    TECH_STACK,
    SUITE_VALIDATOR,
]

PHASE9_GATE = "phase_9_diagnostics_artifacts_ci_flake"
EXPECTED_LOG_SERVICES = {
    "service:api",
    "service:worker",
    "service:embedded_state",
    "service:overqueue_jobs",
    "service:overstore_stub",
    "service:event_audit",
    "service:node_agent_simulator",
}
EXPECTED_CI_COMMANDS = {
    "dev:start",
    "dev:reset",
    "dev:seed",
    "dev:smoke",
    "dev:prune",
    "schema:check",
    "layout:check",
    "docs:check",
    "harness:smoke",
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
    assert_true(isinstance(body, dict), "CLI envelope did not contain result or error body")
    return body


def assert_secret_free(payload: dict[str, Any], context: str) -> None:
    rendered = json.dumps(payload, sort_keys=True).lower()
    for marker in RAW_SECRET_MARKERS:
        assert_true(marker not in rendered, f"{context} exposes raw secret marker {marker}")
    body = envelope_body(payload)
    for event in body.get("lifecycle_events", []):
        assert_true(event["contains_raw_secret"] is False, f"{context} event exposes raw secret")
        assert_true(event["redaction_summary"] == "secret_free", f"{context} event redaction drifted")
        assert_true(event["local_only"] is True and event["test_only"] is True, f"{context} event scope drifted")


def check_required_files() -> None:
    for path in REQUIRED_FILES:
        assert_true((REPO_ROOT / path).is_file(), f"missing required file: {path}")


def check_docs_and_source() -> None:
    source = read_text(LOCAL_STACK_LIB)
    cli_parser = read_text(CLI_PARSER)
    cli_runner = read_text(CLI_RUNNER)
    sub_plan = read_text(SUB_PLAN)
    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    tech_stack = read_text(TECH_STACK)
    suite = read_text(SUITE_VALIDATOR)

    assert_contains(sub_plan, "## Phase 9: Diagnostics, Artifacts, CI, And Flake Evidence", SUB_PLAN)
    for item in ["**9.1", "**9.2", "**9.3", "**9.4", "**9.5"]:
        assert_contains(sub_plan, item, SUB_PLAN)
    for expected in [
        "redacted log export evidence",
        "diagnostic artifact bundle records",
        "clean-checkout CI entrypoint evidence",
        "flake evidence fields",
        "cleanup command integration and retention policy records",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)
    assert_contains(phase_progress, "Local Development Stack Phase 9 Progress", PHASE_PROGRESS)
    assert_contains(tech_stack, "Rust-first", TECH_STACK)
    assert_contains(
        suite,
        'Path("scripts/validate_local_development_stack_phase9.py")',
        SUITE_VALIDATOR,
    )

    for expected in [
        "LOCAL_STACK_PHASE9_DIAGNOSTICS_GATE",
        "LOCAL_STACK_PHASE9_CI_RUNNER_REF",
        "LocalRedactedLogExportRecord",
        "LocalDiagnosticArtifactBundleRecord",
        "LocalCleanCheckoutCiEntrypointRecord",
        "LocalFlakeEvidenceRecord",
        "LocalArtifactRetentionPolicyRecord",
        "redacted_log_export_records",
        "diagnostic_artifact_bundle_records",
        "clean_checkout_ci_entries",
        "flake_evidence_records_for_profile",
        "artifact_retention_policies",
        "render_redacted_log_exports_json",
        "render_diagnostic_artifact_bundles_json",
        "render_clean_checkout_ci_entries_json",
        "render_flake_evidence_records_json",
        "render_artifact_retention_policies_json",
        "phase9_redacted_log_exports_cover_local_surfaces_and_sensitive_material",
        "phase9_diagnostic_bundle_captures_reproduction_context_without_secrets",
        "phase9_clean_checkout_ci_entrypoint_is_loopback_only_and_reproducible",
        "phase9_flake_evidence_records_repeat_variance_retries_and_timeout_classes",
        "phase9_retention_policy_prune_keeps_user_production_and_non_local_state_safe",
        "phase9_prune_command_integrates_cleanup_without_deleting_non_local_state",
        "DevCommand::Prune",
        "cleanup_prune_command_integrated",
        "local_stack.artifact_retention_prune_verified",
        PHASE9_GATE,
    ]:
        assert_contains(source, expected, LOCAL_STACK_LIB)
    for expected in [
        "DevCommand::Prune",
        '"prune" => Ok(Command::Dev(DevCommand::Prune))',
        'Self::Prune => "dev prune"',
    ]:
        assert_contains(cli_parser, expected, CLI_PARSER)
    for expected in [
        "DevCommand::Prune => LocalStackDevCommand::Prune",
        "dev start|stop|restart|status|reset|seed|smoke|logs|doctor|prune",
        "dev_prune_reports_marker_gated_retention_policy",
    ]:
        assert_contains(cli_runner, expected, CLI_RUNNER)


def check_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-local-stack", "phase9"])


def check_redacted_logs() -> None:
    output = run_cli_json(
        [
            "dev",
            "logs",
            "--json",
            "--trace-id",
            "trace_phase9_validator_logs",
        ]
    )
    assert_true(output["ok"] is True, "phase 9 logs should pass")
    result = output["result"]
    assert_true(result["diagnostics_phase_gate"] == PHASE9_GATE, "phase 9 diagnostics gate drifted")
    services = {record["service_id"] for record in result["redacted_log_exports"]}
    assert_true(EXPECTED_LOG_SERVICES.issubset(services), "redacted log surfaces missing")
    for record in result["redacted_log_exports"]:
        assert_true(record["redacts_secrets"] is True, "log export must redact secrets")
        assert_true(record["redacts_tokens"] is True, "log export must redact tokens")
        assert_true(record["redacts_signatures"] is True, "log export must redact signatures")
        assert_true(record["redacts_private_payloads"] is True, "log export must redact private payloads")
        assert_true(record["redacts_encrypted_content"] is True, "log export must redact encrypted content")
        assert_true(record["export_blocked_until_secret_free"] is True, "log export must fail closed")
        assert_true(record["scanner_passed"] is True, "log export scanner must pass")
        assert_true(record["local_only"] is True and record["test_only"] is True, "log export scope drifted")
    assert_true(
        any(reference.endswith("/redacted") for reference in result["artifact_refs"]),
        "logs output missing redacted log artifact refs",
    )
    assert_secret_free(output, "phase 9 logs")


def check_diagnostic_bundle_and_retention() -> None:
    output = run_cli_json(
        [
            "dev",
            "smoke",
            "--json",
            "--trace-id",
            "trace_phase9_validator_smoke",
        ]
    )
    assert_true(output["ok"] is True, "phase 9 smoke should pass")
    result = output["result"]
    bundle = result["diagnostic_artifact_bundles"][0]
    assert_true(bundle["trace_id"] == "trace_phase9_validator_smoke", "bundle trace id drifted")
    assert_true(bundle["retention_class"] == "failure_evidence", "bundle retention class drifted")
    assert_true(bundle["contains_raw_secret"] is False, "bundle exposes raw secret")
    assert_true(bundle["reproduction_command"].startswith("overrid dev smoke"), "reproduction command missing")
    for key in ["health_snapshot_refs", "local_event_refs", "queue_state_refs", "object_refs"]:
        assert_true(len(bundle[key]) > 0, f"bundle missing {key}")
    assert_true(
        any(reference == bundle["bundle_ref"] for reference in result["artifact_refs"]),
        "smoke artifact refs missing diagnostic bundle",
    )

    policies = result["artifact_retention_policies"]
    assert_true(any(policy["retention_class"] == "success_summary" for policy in policies), "success retention missing")
    assert_true(any(policy["retention_class"] == "failure_evidence" for policy in policies), "failure retention missing")
    for policy in policies:
        assert_true(
            "command://local_stack/dev/prune" in policy["prune_command_ref"],
            "retention prune command must point at dev prune",
        )
        assert_true(policy["requires_test_state_marker"] is True, "retention must require test marker")
        assert_true(policy["deletes_unmarked_user_dirs"] is False, "retention may not delete unmarked user dirs")
        assert_true(policy["deletes_production_like_state"] is False, "retention may not delete production-like state")
        assert_true(policy["deletes_non_local_artifacts"] is False, "retention may not delete non-local artifacts")
    assert_true(
        "artifact_retention_policy_enforced" in output["diagnostic_bundle"]["dependency_status"],
        "dependency status missing retention evidence",
    )
    assert_secret_free(output, "phase 9 smoke")


def check_cleanup_prune_command() -> None:
    output = run_cli_json(
        [
            "dev",
            "prune",
            "--json",
            "--trace-id",
            "trace_phase9_validator_prune",
        ]
    )
    assert_true(output["ok"] is True, "phase 9 prune should pass")
    result = output["result"]
    assert_true(result["command"] == "dev prune", "prune command name drifted")
    assert_true(
        result["reason_code"] == "local_stack.artifact_retention_prune_verified",
        "prune reason code drifted",
    )
    assert_true(
        any(
            reference == "artifact://local_stack/prune/trace_phase9_validator_prune/retention_report"
            for reference in result["artifact_refs"]
        ),
        "prune output missing retention report artifact ref",
    )
    policies = result["artifact_retention_policies"]
    assert_true(len(policies) >= 2, "prune output missing retention policies")
    for policy in policies:
        assert_true(
            "command://local_stack/dev/prune" in policy["prune_command_ref"],
            "prune policy command ref drifted",
        )
        assert_true(policy["requires_test_state_marker"] is True, "prune must require test marker")
        assert_true(policy["deletes_unmarked_user_dirs"] is False, "prune may not delete unmarked user dirs")
        assert_true(policy["deletes_production_like_state"] is False, "prune may not delete production-like state")
        assert_true(policy["deletes_non_local_artifacts"] is False, "prune may not delete non-local artifacts")
    assert_true(
        "cleanup_prune_command_integrated" in output["diagnostic_bundle"]["dependency_status"],
        "dependency status missing prune command integration",
    )
    assert_secret_free(output, "phase 9 prune")


def check_clean_checkout_ci(status_payload: dict[str, Any]) -> None:
    result = status_payload["result"]
    entry = result["clean_checkout_ci_entries"][0]
    assert_true(entry["runner_ref"].endswith("/ubuntu_24_04_equivalent"), "CI runner ref drifted")
    assert_true(entry["os_family"] == "linux", "CI runner OS drifted")
    assert_true(entry["arch"] == "x86_64", "CI runner arch drifted")
    assert_true(entry["ubuntu_24_04_equivalent"] is True, "CI runner distro evidence missing")
    assert_true(entry["repo_pinned_rust_toolchain"] is True, "CI runner must pin Rust toolchain")
    assert_true(entry["loopback_networking"] is True, "CI runner must be loopback only")
    assert_true(entry["cloud_credentials_allowed"] is False, "CI runner may not allow cloud credentials")
    assert_true(entry["external_database_allowed"] is False, "CI runner may not allow external DB")
    assert_true(entry["external_queue_allowed"] is False, "CI runner may not allow external queue")
    assert_true(entry["external_object_store_allowed"] is False, "CI runner may not allow external object store")
    assert_true(EXPECTED_CI_COMMANDS.issubset(set(entry["commands"])), "CI command coverage missing")
    assert_true({"success", "blocked", "failed"}.issubset(set(entry["allowed_outcomes"])), "CI outcomes missing")
    assert_true(entry["machine_readable_output"] is True, "CI output must be stable and machine-readable")


def check_flake_evidence() -> None:
    output = run_cli_json(
        [
            "dev",
            "status",
            "--json",
            "--profile",
            "local-nondeterministic-fixture-ids-unstable-event-ordering-health-timeout",
            "--trace-id",
            "trace_phase9_validator_flake",
        ],
        expect_code=None,
    )
    body = envelope_body(output)
    evidence = body["flake_evidence_records"][0]
    assert_true(evidence["trace_id"] == "trace_phase9_validator_flake", "flake trace id drifted")
    assert_true(evidence["repeated_run_count"] >= 3, "flake repeated run count missing")
    assert_true(evidence["startup_timing_variance_ms"] >= 1000, "flake timing variance missing")
    assert_true(evidence["nondeterministic_fixture_ids"] is True, "fixture id flake signal missing")
    assert_true(evidence["unstable_event_ordering"] is True, "event ordering flake signal missing")
    assert_true(evidence["retry_count"] > 0, "flake retry count missing")
    assert_true(evidence["health_timeout_class"] == "required_service_timeout", "health timeout class missing")
    assert_true(evidence["flake_detected"] is True, "flake signal not detected")
    assert_secret_free(output, "phase 9 flake status")


def main() -> int:
    check_required_files()
    check_docs_and_source()
    check_cargo_tests()
    status = run_cli_json(["dev", "status", "--json", "--trace-id", "trace_phase9_validator_status"])
    assert_secret_free(status, "phase 9 status")
    check_clean_checkout_ci(status)
    check_redacted_logs()
    check_diagnostic_bundle_and_retention()
    check_cleanup_prune_command()
    check_flake_evidence()
    print("Local Development Stack Phase 9 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
