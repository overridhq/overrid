#!/usr/bin/env python3
"""Validate CLI Phase 10 release-readiness, security, and handoff gates."""

from __future__ import annotations

from pathlib import Path
import json
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

CONTRACTS = Path("packages/schemas/overrid_contracts")
CLI = Path("packages/cli")
CONTRACTS_SCHEMA = CONTRACTS / "v0/cli_command.schema.json"
CONTRACTS_MANIFEST = CONTRACTS / "codegen_manifest.json"
PHASE10_FIXTURE = CLI / "fixtures/valid/phase10_release_readiness.valid.json"
SUB_PLAN = Path("docs/build_plan/sub_build_plan_002_cli.md")
SERVICE = Path("docs/service_catalog/foundation/cli.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/cli_phase_10_plan.md")
PHASE_PROGRESS = Path("docs/planning/cli_phase_10_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

PHASE10_CONTRACTS = {
    "cli_security_review_report",
    "cli_phase_availability_matrix",
    "cli_release_readiness_report",
}
SNAPSHOT_CHECKS = {
    "schema_contracts",
    "output_envelope",
    "exit_code_registry",
    "help_text",
    "human_output",
    "json_output",
    "error_decode_records",
    "backward_compatible_json",
}
HELP_COMMANDS = {
    "version",
    "doctor",
    "profile",
    "credential",
    "auth",
    "tenant",
    "identity",
    "key",
    "manifest",
    "node",
    "workload",
    "policy",
    "package",
    "usage",
    "receipt",
    "ledger",
    "dispute",
    "federation/public-interest/purpose-tag",
    "release-readiness",
}
EXIT_CLASSES = {
    "success",
    "usage",
    "config",
    "credential",
    "schema",
    "policy",
    "phase",
    "idempotency",
    "transport",
    "timeout",
    "platform",
    "local_io",
}
REASON_FAMILIES = {
    "not_available_in_phase",
    "profile_validation_failed",
    "credential_validation_failed",
    "missing_profile_confirmation",
    "missing_reason",
    "policy.egress_denied",
    "package.invalid",
    "result.failed",
    "budget.exhausted",
    "transport.unavailable",
    "timeout.waiting",
}
INTEGRATION_CHECKS = {
    "tenant_setup",
    "identity_key_lifecycle",
    "manifest_submit",
    "synthetic_workload",
    "real_private_job",
    "policy_dry_run",
    "package_validation",
    "usage_receipt_lookup",
    "cancellation",
    "timeout_retry",
    "docdex_workflow",
    "mcoda_workflow",
    "codali_workflow",
}
AUTOMATION_CHECKS = {
    "stable_json_output",
    "stable_human_output",
    "exit_class_branching",
    "trace_id_presence",
    "audit_ref_presence",
    "bounded_retry_timeout",
    "ci_non_interactive_credentials",
}
SECURITY_FALSE_FLAGS = {
    "raw_keys_exposed",
    "tokens_exposed",
    "signatures_exposed",
    "secrets_exposed",
    "private_payloads_exposed",
    "decrypted_content_exposed",
    "unsafe_endpoints_allowed",
    "cross_tenant_access_allowed",
}
DENIED_HANDOFF_COMMANDS = {
    "federation/public-interest/purpose-tag": "phase_10",
    "deployment": "phase_9",
    "governance/incident/compliance/migration": "phase_7_or_phase_13",
}
SECRET_VALUE_MARKERS = [
    "-----begin",
    "private key",
    "raw_key=",
    "secret=",
    "token=",
    "signature=",
    "private_payload=",
    "decrypted_content=",
    "node-agent://",
    "postgres://",
    "redis://",
    "s3://",
    "minio://",
    "http://overbase.",
]
FORBIDDEN_STACK_MARKERS = [
    "direct_internal_api_access\":true",
    "direct_storage_access\":true",
    "raw_http_required\":true",
    "high_risk_phase7_phase13_enabled\":true",
    "direct_private_shortcut\":true",
]

REQUIRED_FILES = [
    CONTRACTS_SCHEMA,
    CONTRACTS_MANIFEST,
    PHASE10_FIXTURE,
    CONTRACTS / "src/lib.rs",
    CONTRACTS / "README.md",
    CLI / "src/parser.rs",
    CLI / "src/runner.rs",
    CLI / "README.md",
    SUB_PLAN,
    SERVICE,
    TECH_STACK,
    PHASE_PLAN,
    PHASE_PROGRESS,
    VALIDATION_WRAPPER,
]


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


def assert_no_forbidden_stack(text: str, source: str) -> None:
    lowered = text.lower()
    for marker in FORBIDDEN_STACK_MARKERS:
        if marker in lowered:
            raise AssertionError(f"{source} contains forbidden stack shortcut: {marker}")


def assert_runtime_secret_free(value: Any, source: str, path: str = "$") -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            if key == "forbidden_output_markers":
                continue
            assert_runtime_secret_free(child, source, f"{path}.{key}")
        return
    if isinstance(value, list):
        for index, child in enumerate(value):
            assert_runtime_secret_free(child, source, f"{path}[{index}]")
        return
    if isinstance(value, str):
        lowered = value.lower()
        for marker in SECRET_VALUE_MARKERS:
            if marker in lowered:
                raise AssertionError(f"{source} leaked forbidden marker at {path}: {marker}")


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=REPO_ROOT)
    if result.returncode != 0:
        raise AssertionError(f"Command failed with exit {result.returncode}: {' '.join(command)}")


def run_capture(command: list[str], expected_exit: int = 0) -> str:
    result = subprocess.run(command, cwd=REPO_ROOT, text=True, capture_output=True)
    if result.returncode != expected_exit:
        raise AssertionError(
            f"Command failed with exit {result.returncode}, expected {expected_exit}: "
            f"{' '.join(command)}\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    if result.stderr:
        raise AssertionError(f"Command emitted stderr unexpectedly: {' '.join(command)}\n{result.stderr}")
    return result.stdout.strip()


def run_cli(args: list[str], expected_exit: int = 0) -> dict[str, Any]:
    output = run_capture(["cargo", "run", "-q", "-p", "overrid-cli", "--", *args], expected_exit)
    assert_no_forbidden_stack(output, " ".join(args))
    try:
        parsed = json.loads(output)
    except json.JSONDecodeError as exc:
        raise AssertionError(f"CLI output is not JSON for {' '.join(args)}: {output}") from exc
    assert_runtime_secret_free(parsed, " ".join(args))
    return parsed


def validate_required_files() -> None:
    for path in REQUIRED_FILES:
        if not (REPO_ROOT / path).is_file():
            raise AssertionError(f"Missing required file: {path}")


def validate_schema_and_manifest() -> None:
    schema = read_json(CONTRACTS_SCHEMA)
    manifest = read_json(CONTRACTS_MANIFEST)
    defs = schema["$defs"]

    if "Phase 10" not in schema.get("description", ""):
        raise AssertionError("CLI schema description is missing Phase 10")

    actual_contracts = set(schema["properties"]["contracts"]["items"]["enum"])
    missing_contracts = PHASE10_CONTRACTS - actual_contracts
    if missing_contracts:
        raise AssertionError(f"CLI schema is missing Phase 10 contracts: {sorted(missing_contracts)}")
    missing_defs = PHASE10_CONTRACTS - set(defs)
    if missing_defs:
        raise AssertionError(f"CLI schema is missing Phase 10 $defs: {sorted(missing_defs)}")

    security = defs["cli_security_review_report"]["properties"]
    for key in SECURITY_FALSE_FLAGS:
        if security[key].get("const") is not False:
            raise AssertionError(f"cli_security_review_report must keep {key}=false")

    availability = defs["cli_phase_availability_record"]["properties"]
    if availability["direct_private_shortcut"].get("const") is not False:
        raise AssertionError("cli_phase_availability_record must forbid private shortcuts")
    if defs["cli_phase_availability_matrix"].get("minItems", 0) < 21:
        raise AssertionError("cli_phase_availability_matrix must cover the full command matrix")

    release = defs["cli_release_readiness_report"]["properties"]
    for key in ["release_ready", "sdk_overgate_only"]:
        if release[key].get("const") is not True:
            raise AssertionError(f"cli_release_readiness_report must keep {key}=true")
    for key in ["direct_private_shortcut", "high_risk_phase7_phase13_enabled"]:
        if release[key].get("const") is not False:
            raise AssertionError(f"cli_release_readiness_report must keep {key}=false")

    if set(manifest.get("phase10_contracts", [])) != PHASE10_CONTRACTS:
        raise AssertionError("Manifest phase10_contracts does not match schema extensions")
    fixture_paths = manifest.get("fixtures", {}).get("phase10_valid", [])
    if str(PHASE10_FIXTURE) not in fixture_paths:
        raise AssertionError("Manifest is missing Phase 10 fixture path")

    guardrails = "\n".join(manifest.get("guardrails", []))
    for expected in [
        "Phase 10 release-readiness output",
        "Phase 10 security validation",
        "Phase 10 phase-availability validation",
        "Phase 10 Phase 7/13 handoff evidence",
    ]:
        if expected not in guardrails:
            raise AssertionError(f"Manifest guardrails missing Phase 10 text: {expected}")


def validate_report_payload(report: dict[str, Any], source: str) -> None:
    if set(report["contract_snapshot_suite"]) != SNAPSHOT_CHECKS:
        raise AssertionError(f"{source} contract snapshot suite drifted")
    if set(report["help_snapshot_commands"]) != HELP_COMMANDS:
        raise AssertionError(f"{source} help snapshot commands drifted")
    if set(report["exit_code_classes"]) != EXIT_CLASSES:
        raise AssertionError(f"{source} exit classes drifted")
    if set(report["reason_code_families"]) != REASON_FAMILIES:
        raise AssertionError(f"{source} reason families drifted")
    if set(report["integration_validation_matrix"]) != INTEGRATION_CHECKS:
        raise AssertionError(f"{source} integration matrix drifted")
    if set(report["automation_compatibility_matrix"]) != AUTOMATION_CHECKS:
        raise AssertionError(f"{source} automation matrix drifted")
    for key in ["release_ready", "sdk_overgate_only"]:
        if report[key] is not True:
            raise AssertionError(f"{source} must keep {key}=true")
    for key in ["direct_private_shortcut", "high_risk_phase7_phase13_enabled"]:
        if report[key] is not False:
            raise AssertionError(f"{source} must keep {key}=false")

    security = report["security_review_report"]
    for key in SECURITY_FALSE_FLAGS:
        if security[key] is not False:
            raise AssertionError(f"{source} security report must keep {key}=false")
    for expected in ["credential_storage", "signer_handoff", "diagnostics", "execution_results"]:
        if expected not in security["reviewed_surfaces"]:
            raise AssertionError(f"{source} security review is missing {expected}")

    matrix = {entry["command"]: entry for entry in report["phase_availability_matrix"]}
    if len(matrix) < 20:
        raise AssertionError(f"{source} phase availability matrix is incomplete")
    if matrix["release-readiness"]["availability"] != "available":
        raise AssertionError(f"{source} release-readiness must be available")
    for command, phase_gate in DENIED_HANDOFF_COMMANDS.items():
        entry = matrix[command]
        if entry["phase_gate"] != phase_gate:
            raise AssertionError(f"{source} {command} phase gate drifted")
        if entry["availability"] != "denied":
            raise AssertionError(f"{source} {command} must stay denied")
        if entry["stable_reason_code"] != "not_available_in_phase":
            raise AssertionError(f"{source} {command} must use stable not_available_in_phase")
        if entry["hidden_in_normal_help"] is not True:
            raise AssertionError(f"{source} {command} must stay hidden from normal help")
        if entry["direct_private_shortcut"] is not False:
            raise AssertionError(f"{source} {command} must forbid private shortcuts")


def validate_fixture_and_docs() -> None:
    fixture = read_json(PHASE10_FIXTURE)
    if fixture["phase"] != "cli_phase_10":
        raise AssertionError("Phase 10 fixture has wrong phase marker")
    if set(fixture["contracts"]) != PHASE10_CONTRACTS:
        raise AssertionError("Phase 10 fixture contract set drifted")
    validate_report_payload(
        {
            **fixture,
            "release_ready": fixture["guardrails"]["release_ready"],
            "sdk_overgate_only": fixture["guardrails"]["sdk_overgate_only"],
            "direct_private_shortcut": fixture["guardrails"]["direct_private_shortcut"],
            "high_risk_phase7_phase13_enabled": fixture["guardrails"][
                "high_risk_phase7_phase13_enabled"
            ],
        },
        "Phase 10 fixture",
    )

    docs = {
        CONTRACTS / "README.md": [
            "Phase 10",
            "cli_security_review_report",
            "cli_phase_availability_matrix",
            "cli_release_readiness_report",
        ],
        CLI / "README.md": ["Phase 10", "release-readiness", "not_available_in_phase"],
        SERVICE: ["Phase 10 Implementation Gates", "release-readiness", "scripts/validate_cli_phase10.py"],
        SUB_PLAN: ["Phase 10: Validation", "10.1", "10.5"],
        PHASE_PLAN: ["CLI Phase 10 Plan", "release-readiness", "Validation Plan"],
        PHASE_PROGRESS: ["CLI Phase 10 Progress", "Validation Evidence"],
        TECH_STACK: ["Rust CLI", "Overgate"],
        VALIDATION_WRAPPER: ["scripts/validate_cli_phase10.py"],
    }
    for path, expected_values in docs.items():
        text = read(path)
        for expected in expected_values:
            assert_contains(text, expected, path)


def validate_rust_surfaces() -> None:
    contracts = read(CONTRACTS / "src/lib.rs")
    for expected in [
        "CliSecurityReviewReport",
        "CliPhaseAvailabilityRecord",
        "CliReleaseReadinessReport",
        "phase10_release_readiness_report_keeps_handoff_gated_and_secret_free",
    ]:
        assert_contains(contracts, expected, CONTRACTS / "src/lib.rs")

    parser = read(CLI / "src/parser.rs")
    for expected in ["ReleaseReadiness", "release-readiness", "phase_10"]:
        assert_contains(parser, expected, CLI / "src/parser.rs")

    runner = read(CLI / "src/runner.rs")
    for expected in [
        "release_readiness_result",
        "render_release_readiness_report_json",
        "phase10_release_readiness_emits_safe_validation_evidence",
        "security_redaction_validated",
    ]:
        assert_contains(runner, expected, CLI / "src/runner.rs")


def validate_cli_outputs() -> None:
    envelope = run_cli(["release-readiness", "--json"])
    if envelope["ok"] is not True:
        raise AssertionError("release-readiness JSON output must be ok")
    if envelope["diagnostic_bundle"]["command_name"] != "release-readiness":
        raise AssertionError("release-readiness command name drifted")
    if envelope["exit_class"] != "success" or envelope["exit_code"] != 0:
        raise AssertionError("release-readiness must use success exit metadata")
    if envelope["reason_code"] is not None:
        raise AssertionError("release-readiness success must not set a reason_code")
    validate_report_payload(envelope["result"], "release-readiness output")

    human = run_capture(["cargo", "run", "-q", "-p", "overrid-cli", "--", "release-readiness"])
    for expected in [
        "release_readiness: ready",
        "security_review: secret_free",
        "phase_10_federation_handoff_denied",
        "phase_7_or_phase_13_handoff_denied",
        "high_risk_phase7_phase13_operations_disabled",
    ]:
        if expected not in human:
            raise AssertionError(f"release-readiness human output missing {expected}")

    help_text = run_capture(["cargo", "run", "-q", "-p", "overrid-cli", "--", "help"])
    if "release-readiness" not in help_text:
        raise AssertionError("normal help must show release-readiness")
    if "governance|incident|compliance|migration" in help_text:
        raise AssertionError("normal help must not show Phase 7/13 handoff commands")

    all_help = run_capture(["cargo", "run", "-q", "-p", "overrid-cli", "--", "help", "--all-phases"])
    for expected in [
        "deployment helpers",
        "federation|public-interest|purpose-tag",
        "governance|incident|compliance|migration",
    ]:
        if expected not in all_help:
            raise AssertionError(f"all-phases help is missing {expected}")

    for args, command_name, phase_gate in [
        (
            ["federation", "--json"],
            "federation/public-interest/purpose-tag",
            "phase_10",
        ),
        (
            ["public-interest", "--json"],
            "federation/public-interest/purpose-tag",
            "phase_10",
        ),
        (
            ["purpose-tag", "--json"],
            "federation/public-interest/purpose-tag",
            "phase_10",
        ),
        (["deployment", "--json"], "deployment", "phase_9"),
        (["governance", "--json"], "governance/incident/compliance", "phase_7_or_phase_13"),
        (["incident", "--json"], "governance/incident/compliance", "phase_7_or_phase_13"),
    ]:
        denied = run_cli(args, expected_exit=7)
        if denied["ok"] is not False:
            raise AssertionError(f"{command_name} must fail closed")
        if denied["diagnostic_bundle"]["command_name"] != command_name:
            raise AssertionError(f"{command_name} denial command name drifted")
        if denied["reason_code"] != "not_available_in_phase":
            raise AssertionError(f"{command_name} must use not_available_in_phase")
        if denied["exit_class"] != "phase":
            raise AssertionError(f"{command_name} must use phase exit class")
        if denied["error"]["phase_gate"] != phase_gate:
            raise AssertionError(f"{command_name} phase gate drifted")


def validate_rust_tests() -> None:
    run(["cargo", "test", "-p", "overrid-contracts", "-p", "overrid-cli", "phase10"])


def main() -> int:
    try:
        validate_required_files()
        validate_schema_and_manifest()
        validate_fixture_and_docs()
        validate_rust_surfaces()
        validate_cli_outputs()
        validate_rust_tests()
    except AssertionError as exc:
        print(f"validate_cli_phase10.py failed: {exc}", file=sys.stderr)
        return 1
    print("validate_cli_phase10.py passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
