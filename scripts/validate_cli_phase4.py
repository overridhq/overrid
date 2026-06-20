#!/usr/bin/env python3
"""Validate CLI Phase 4 lifecycle, envelope, diagnostics, and capability gates."""

from __future__ import annotations

from pathlib import Path
import json
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

CONTRACTS = Path("packages/schemas/overrid_contracts")
SDK = Path("packages/sdk")
CLI = Path("packages/cli")
CONTRACTS_SCHEMA = CONTRACTS / "v0/cli_command.schema.json"
CONTRACTS_MANIFEST = CONTRACTS / "codegen_manifest.json"
VERSION_FIXTURE = CLI / "fixtures/valid/version_output.valid.json"
SUB_PLAN = Path("docs/build_plan/sub_build_plan_002_cli.md")
SERVICE = Path("docs/service_catalog/foundation/cli.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/cli_phase_04_plan.md")
PHASE_PROGRESS = Path("docs/planning/cli_phase_04_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

PHASE4_CONTRACTS = {
    "command_lifecycle",
    "exit_code_registry",
    "diagnostic_bundle",
    "capability_snapshot",
    "command_context",
    "output_envelope",
}

LIFECYCLE_STATES = [
    "parsed",
    "profile_loaded",
    "credential_ready",
    "payload_validated",
    "signed",
    "submitted",
    "accepted",
    "waiting",
    "completed",
    "denied",
    "failed",
]

EXIT_CODE_PAIRS = [
    ("success", 0),
    ("usage", 2),
    ("config", 3),
    ("credential", 4),
    ("schema", 5),
    ("policy", 6),
    ("phase", 7),
    ("idempotency", 8),
    ("transport", 9),
    ("timeout", 10),
    ("platform", 11),
    ("local_io", 12),
]

ENVELOPE_FIELDS = {
    "schema_version",
    "ok",
    "result",
    "error",
    "trace_id",
    "reason_code",
    "retry_class",
    "exit_code",
    "exit_class",
    "timing_ms",
    "lifecycle",
    "diagnostic_bundle",
    "capabilities",
    "audit_refs",
    "warnings",
}

SECRET_MARKERS = [
    "-----begin",
    "private key",
    "raw_key",
    "secret=",
    "token=",
    "decrypted_payload",
    "private_payload=",
    "raw prompt",
    "private file",
]

LOCAL_PROFILE_ARGS = [
    "--profile",
    "local-dev",
    "--environment",
    "local",
    "--endpoint",
    "http://127.0.0.1:18080/overgate",
    "--endpoint-fingerprint",
    "fp_local",
    "--tenant",
    "tenant_local",
    "--actor",
    "actor_local",
    "--credential-namespace",
    "local-dev",
    "--credential-class",
    "fixture",
    "--credential-ref",
    "fixture://local-dev/key-1",
    "--key-id",
    "key-1",
    "--fixture-allowance",
    "local_only",
]

REQUIRED_FILES = [
    CONTRACTS_SCHEMA,
    CONTRACTS_MANIFEST,
    CONTRACTS / "src/lib.rs",
    SDK / "src/lib.rs",
    CLI / "src/parser.rs",
    CLI / "src/runner.rs",
    VERSION_FIXTURE,
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


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} is missing expected text: {expected}")


def assert_secret_free(text: str, source: str) -> None:
    lowered = text.lower()
    for marker in SECRET_MARKERS:
        if marker in lowered:
            raise AssertionError(f"{source} leaked secret marker: {marker}")


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
    assert_secret_free(output, " ".join(args))
    try:
        return json.loads(output)
    except json.JSONDecodeError as exc:
        raise AssertionError(f"CLI output is not JSON for {' '.join(args)}: {output}") from exc


def validate_required_files() -> None:
    for path in REQUIRED_FILES:
        if not (REPO_ROOT / path).is_file():
            raise AssertionError(f"Missing required file: {path}")


def validate_schema_and_manifest() -> None:
    schema = json.loads(read(CONTRACTS_SCHEMA))
    manifest = json.loads(read(CONTRACTS_MANIFEST))

    actual_contracts = set(schema["properties"]["contracts"]["items"]["enum"])
    missing_contracts = PHASE4_CONTRACTS - actual_contracts
    if missing_contracts:
        raise AssertionError(f"CLI schema is missing Phase 4 contracts: {sorted(missing_contracts)}")

    defs = schema["$defs"]
    for definition in [
        "command_lifecycle",
        "exit_code_registry",
        "diagnostic_bundle",
        "capability_snapshot",
        "command_context",
        "output_envelope",
    ]:
        if definition not in defs:
            raise AssertionError(f"CLI schema is missing $defs.{definition}")

    lifecycle_states = defs["command_lifecycle_state"]["enum"]
    if lifecycle_states != LIFECYCLE_STATES:
        raise AssertionError(f"Lifecycle states changed: {lifecycle_states}")

    exit_classes = defs["exit_code_class"]["enum"]
    exit_codes = defs["exit_code"]["enum"]
    if list(zip(exit_classes, exit_codes)) != EXIT_CODE_PAIRS:
        raise AssertionError("Exit-code registry schema no longer matches Phase 4 numeric contract")

    output_required = set(defs["output_envelope"]["required"])
    missing_envelope_fields = ENVELOPE_FIELDS - output_required
    if missing_envelope_fields:
        raise AssertionError(f"output_envelope missing fields: {sorted(missing_envelope_fields)}")

    diagnostic = defs["diagnostic_bundle"]
    if diagnostic["properties"]["redaction_policy"].get("const") != "secret_free_refs_only":
        raise AssertionError("diagnostic_bundle must require secret_free_refs_only")

    if set(manifest.get("phase4_contracts", [])) != PHASE4_CONTRACTS:
        raise AssertionError("Manifest phase4_contracts does not match the schema extensions")
    if manifest["rust_projection"]["non_authoritative"] is not True:
        raise AssertionError("Rust projection must remain non-authoritative")


def validate_rust_phase4_surface() -> None:
    contracts = read(CONTRACTS / "src/lib.rs")
    for expected in [
        "CommandLifecycleState",
        "ALL_COMMAND_LIFECYCLE_STATES",
        "CommandLifecycle",
        "RetryClass",
        "ExitCodeClass",
        "EXIT_CODE_REGISTRY",
        "CapabilitySnapshot",
        "CommandContext",
        "OutputEnvelope",
        "DiagnosticBundle",
        "secret_free_refs_only",
    ]:
        assert_contains(contracts, expected, CONTRACTS / "src/lib.rs")

    parser = read(CLI / "src/parser.rs")
    for expected in [
        "Doctor",
        '"doctor"',
        "PolicyCommand",
        "PackageCommand",
        "UsageCommand",
        "PlannedCommand::Package",
    ]:
        assert_contains(parser, expected, CLI / "src/parser.rs")

    runner = read(CLI / "src/runner.rs")
    for expected in [
        "render_success_json",
        "render_error_json",
        "render_envelope_json",
        "render_diagnostic_bundle_json",
        "render_capabilities_json",
        "render_doctor",
        "exit_class",
        "fail_closed",
        "trace_cli_local",
    ]:
        assert_contains(runner, expected, CLI / "src/runner.rs")


def validate_envelope(payload: dict[str, Any], *, ok: bool, exit_class: str) -> None:
    missing = ENVELOPE_FIELDS - set(payload)
    if missing:
        raise AssertionError(f"Envelope missing fields: {sorted(missing)}")
    if payload["schema_version"] != "cli-command.v0.1":
        raise AssertionError("Envelope must use cli-command.v0.1")
    if payload["ok"] is not ok:
        raise AssertionError(f"Envelope ok mismatch: {payload['ok']}")
    if payload["exit_class"] != exit_class:
        raise AssertionError(f"Envelope exit_class mismatch: {payload['exit_class']}")
    expected_code = dict(EXIT_CODE_PAIRS)[exit_class]
    if payload["exit_code"] != expected_code:
        raise AssertionError(f"Envelope exit_code mismatch: {payload['exit_code']}")
    if payload["retry_class"] not in {"not_retryable", "safe_retry", "retry_after", "operator_review"}:
        raise AssertionError(f"Unexpected retry_class: {payload['retry_class']}")
    if not isinstance(payload["timing_ms"], int) or payload["timing_ms"] < 0:
        raise AssertionError("timing_ms must be a non-negative integer")

    lifecycle = payload["lifecycle"]
    if not lifecycle.get("states"):
        raise AssertionError("lifecycle.states must not be empty")
    if lifecycle["terminal_state"] not in {"completed", "denied", "failed"}:
        raise AssertionError(f"Invalid terminal state: {lifecycle['terminal_state']}")
    if ok and lifecycle["terminal_state"] != "completed":
        raise AssertionError("Successful envelopes must end in completed")
    if not ok and lifecycle["terminal_state"] not in {"denied", "failed"}:
        raise AssertionError("Failed envelopes must end in denied or failed")

    bundle = payload["diagnostic_bundle"]
    if bundle["redaction_policy"] != "secret_free_refs_only":
        raise AssertionError("Diagnostic bundle must be secret_free_refs_only")
    assert_secret_free(json.dumps(bundle, sort_keys=True), "diagnostic_bundle")
    if "signature_ref" in json.dumps(bundle).lower() or "sigref:" in json.dumps(bundle).lower():
        raise AssertionError("Diagnostic bundle must not contain signature refs")
    if "cli-command.v0.1" not in bundle["schema_versions"]:
        raise AssertionError("Diagnostic bundle must cite schema version")

    capabilities = payload["capabilities"]
    if capabilities["source"] != "local_capability_cache":
        raise AssertionError("Capability metadata must use local_capability_cache source")
    if not isinstance(capabilities["stale_age_ms"], int) or capabilities["stale_age_ms"] < 0:
        raise AssertionError("Capability stale age must be non-negative")
    if not isinstance(capabilities["routes"], list):
        raise AssertionError("Capability routes must be a list")
    for route in capabilities["routes"]:
        if "cli-command.v0.1" not in route["schema_versions"]:
            raise AssertionError("Capability route must cite schema version")


def validate_cli_behavior() -> None:
    version = run_cli(["version", "--json"])
    validate_envelope(version, ok=True, exit_class="success")
    if version["diagnostic_bundle"]["command_name"] != "version":
        raise AssertionError("version diagnostic command name mismatch")

    fixture = json.loads(read(VERSION_FIXTURE))
    if version != fixture:
        raise AssertionError("overrid version --json output must match valid fixture")

    doctor = run_cli(["doctor", "--json"])
    validate_envelope(doctor, ok=True, exit_class="success")
    if doctor["result"]["redaction_policy"] != "secret_free_refs_only":
        raise AssertionError("doctor result must expose redaction policy")
    if "capability_cache_static" not in doctor["result"]["dependency_status"]:
        raise AssertionError("doctor must expose capability cache status")

    profile = run_cli(["profile", "inspect", "--json", *LOCAL_PROFILE_ARGS])
    validate_envelope(profile, ok=True, exit_class="success")
    if profile["result"]["profile_name"] != "local-dev":
        raise AssertionError("profile inspect result changed")
    if profile["diagnostic_bundle"]["profile_name"] != "local-dev":
        raise AssertionError("profile diagnostic bundle should carry profile name")

    credential = run_cli(["credential", "enroll", "--json", *LOCAL_PROFILE_ARGS])
    validate_envelope(credential, ok=True, exit_class="success")
    if "signed" not in credential["lifecycle"]["states"]:
        raise AssertionError("credential enroll lifecycle must include signed")
    if not credential["result"]["signature_ref"].startswith("sigref:"):
        raise AssertionError("credential enroll must retain signer handoff refs")

    phase_gated = run_cli(["deployment", "--json"], expected_exit=7)
    validate_envelope(phase_gated, ok=False, exit_class="phase")
    if phase_gated["error"]["reason_code"] != "not_available_in_phase":
        raise AssertionError("phase-gated deployment command must return not_available_in_phase")
    if phase_gated["capabilities"]["fail_closed"] is not True:
        raise AssertionError("phase-gated deployment command must fail closed")
    routes = phase_gated["capabilities"]["routes"]
    if not routes or routes[0]["available"] is not False:
        raise AssertionError("phase-gated route must be marked unavailable")
    if phase_gated["trace_id"] != "trace_cli_local":
        raise AssertionError("phase-gated failures must include trace_cli_local")
    if "trace_cli_local" not in phase_gated["diagnostic_bundle"]["trace_ids"]:
        raise AssertionError("diagnostic bundle must retain failure trace id")

    parse_error = run_cli(["--json", "unknown"], expected_exit=2)
    validate_envelope(parse_error, ok=False, exit_class="usage")
    if parse_error["error"]["reason_code"] != "usage_error":
        raise AssertionError("JSON parser failures must use the usage_error reason code")
    if parse_error["diagnostic_bundle"]["command_name"] != "parser":
        raise AssertionError("JSON parser failures must identify the parser command path")
    if parse_error["capabilities"]["fail_closed"] is not True:
        raise AssertionError("JSON parser failures must fail closed")
    if parse_error["trace_id"] != "trace_cli_local":
        raise AssertionError("JSON parser failures must include trace_cli_local")


def validate_docs_and_wrapper() -> None:
    sub_plan = read(SUB_PLAN)
    service = read(SERVICE)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    wrapper = read(VALIDATION_WRAPPER)

    for expected in [
        "## Phase 4: Command Lifecycle, Output Envelope, Exit Codes, And Diagnostics",
        "### Phase 4 Gate Outputs",
        "Lifecycle gate",
        "Output-envelope gate",
        "Exit-registry gate",
        "Diagnostic-bundle gate",
        "Capability-discovery gate",
        "scripts/validate_cli_phase4.py",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    for expected in [
        "## Phase 4 Implementation Gates",
        "Lifecycle gate",
        "Output-envelope gate",
        "Exit-registry gate",
        "Diagnostic gate",
        "Capability gate",
        "scripts/validate_cli_phase4.py",
    ]:
        assert_contains(service, expected, SERVICE)

    assert_contains(tech_stack, "| CLI | Rust CLI using generated contracts", TECH_STACK)
    assert_contains(phase_plan, "CLI Phase 4 Execution Plan", PHASE_PLAN)
    assert_contains(phase_progress, "CLI Phase 4 Progress", PHASE_PROGRESS)
    assert_contains(wrapper, 'Path("scripts/validate_cli_phase4.py")', VALIDATION_WRAPPER)


def validate_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-contracts", "-p", "overrid-sdk", "-p", "overrid-cli"])


def main() -> int:
    checks = [
        validate_required_files,
        validate_schema_and_manifest,
        validate_rust_phase4_surface,
        validate_cli_behavior,
        validate_docs_and_wrapper,
        validate_cargo_tests,
    ]
    for check in checks:
        check()
    print("CLI Phase 4 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
