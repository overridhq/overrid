#!/usr/bin/env python3
"""Validate CLI Phase 5 bootstrap command gates."""

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
SUB_PLAN = Path("docs/build_plan/sub_build_plan_002_cli.md")
SERVICE = Path("docs/service_catalog/foundation/cli.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/cli_phase_05_plan.md")
PHASE_PROGRESS = Path("docs/planning/cli_phase_05_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

PHASE5_CONTRACTS = {
    "phase1_bootstrap_command",
    "signed_command_envelope",
    "bootstrap_acceptance_record",
    "manifest_bootstrap_ref",
    "synthetic_workload_pending_state",
}

BOOTSTRAP_FAMILIES = ["auth", "tenant", "identity", "key", "manifest", "workload"]

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

SIGNED_ENVELOPE_REQUIRED = {
    "family",
    "command_type",
    "tenant_id",
    "actor_id",
    "target_ref",
    "payload_type",
    "idempotency_key",
    "trace_id",
    "signature_ref",
    "exposes_key_material",
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
    CLI / "README.md",
    CONTRACTS / "README.md",
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
    defs = schema["$defs"]

    actual_contracts = set(schema["properties"]["contracts"]["items"]["enum"])
    missing_contracts = PHASE5_CONTRACTS - actual_contracts
    if missing_contracts:
        raise AssertionError(f"CLI schema is missing Phase 5 contracts: {sorted(missing_contracts)}")

    missing_defs = PHASE5_CONTRACTS - set(defs)
    if missing_defs:
        raise AssertionError(f"CLI schema is missing Phase 5 $defs: {sorted(missing_defs)}")

    if defs["bootstrap_command_family"]["enum"] != BOOTSTRAP_FAMILIES:
        raise AssertionError("Phase 5 bootstrap command family enum changed")

    bootstrap = defs["phase1_bootstrap_command"]
    if bootstrap["properties"]["phase_gate"].get("const") != "phase_1_control_plane_bootstrap":
        raise AssertionError("phase1_bootstrap_command must remain Phase 1 gated")
    if bootstrap["properties"]["sdk_target"].get("const") != "overgate_only":
        raise AssertionError("phase1_bootstrap_command must target Overgate through the SDK")

    signed = defs["signed_command_envelope"]
    missing_signed = SIGNED_ENVELOPE_REQUIRED - set(signed["required"])
    if missing_signed:
        raise AssertionError(f"signed_command_envelope missing required fields: {sorted(missing_signed)}")
    for optional_field in ["expected_state", "reason"]:
        if optional_field not in signed["properties"]:
            raise AssertionError(f"signed_command_envelope missing optional field: {optional_field}")
    if signed["properties"]["signature_ref"].get("pattern") != "^sigref:[A-Za-z0-9._:-]+$":
        raise AssertionError("signed_command_envelope signature_ref must stay a signature reference")
    if signed["properties"]["exposes_key_material"].get("const") is not False:
        raise AssertionError("signed_command_envelope must forbid raw key material")

    acceptance = defs["bootstrap_acceptance_record"]
    if acceptance["properties"]["phase_gate"].get("const") != "phase_1_control_plane_bootstrap":
        raise AssertionError("bootstrap_acceptance_record must stay Phase 1 gated")
    if acceptance["properties"]["audit_refs"].get("minItems", 0) < 1:
        raise AssertionError("bootstrap_acceptance_record must carry at least one audit ref")

    manifest_ref = defs["manifest_bootstrap_ref"]
    if manifest_ref["properties"]["submitted_via"].get("const") != "sdk_overgate_contract":
        raise AssertionError("manifest_bootstrap_ref must submit through the SDK/Overgate contract")

    workload = defs["synthetic_workload_pending_state"]
    if workload["properties"]["queue_state"].get("const") != "pending":
        raise AssertionError("synthetic workload state must remain pending in Phase 5")
    if workload["properties"]["execution_implied"].get("const") is not False:
        raise AssertionError("synthetic workload state must not imply execution")
    if workload["properties"]["timeline_refs"].get("minItems", 0) < 1:
        raise AssertionError("synthetic workload state must carry timeline refs")

    if set(manifest.get("phase5_contracts", [])) != PHASE5_CONTRACTS:
        raise AssertionError("Manifest phase5_contracts does not match the schema extensions")
    guardrails = "\n".join(manifest.get("guardrails", []))
    for expected in [
        "Phase 1 bootstrap commands",
        "signed SDK/Overgate envelopes",
        "must not call Overtenant",
    ]:
        if expected not in guardrails:
            raise AssertionError(f"Manifest guardrails missing Phase 5 text: {expected}")


def validate_rust_phase5_surface() -> None:
    contracts = read(CONTRACTS / "src/lib.rs")
    for expected in [
        "BootstrapCommandFamily",
        "SignedCommandEnvelope",
        "BootstrapAcceptanceRecord",
        "ManifestBootstrapRef",
        "SyntheticWorkloadPendingState",
        "phase_1_control_plane_bootstrap",
        "sdk_overgate_contract",
        "execution_implied",
    ]:
        assert_contains(contracts, expected, CONTRACTS / "src/lib.rs")

    parser = read(CLI / "src/parser.rs")
    for expected in [
        "AuthCommand",
        "TenantCommand",
        "IdentityCommand",
        "KeyCommand",
        "ManifestCommand",
        "WorkloadCommand",
        '"auth"',
        '"tenant"',
        '"identity"',
        '"key"',
        '"manifest"',
        '"workload"',
        "--trace-id",
        "--idempotency-key",
        "--expected-state",
        "--target-ref",
        "--manifest-kind",
        "--manifest-ref",
        "--workload-kind",
        "--workload-ref",
        "--dry-run",
    ]:
        assert_contains(parser, expected, CLI / "src/parser.rs")

    runner = read(CLI / "src/runner.rs")
    for expected in [
        "auth_command_result",
        "tenant_command_result",
        "identity_command_result",
        "key_command_result",
        "manifest_command_result",
        "workload_command_result",
        "prepare_bootstrap_context",
        "render_bootstrap_result",
        "render_signed_command_envelope_json",
        "render_acceptance_json",
        "manifest_ref",
        "workload_pending_state",
        "deterministic_idempotency_key",
        "phase1_bootstrap_contracts_available",
        "sdk_overgate_contract",
        "credential_reference_checked",
        "signed_command_envelope",
        "synthetic_workload_pending_state",
        "execution_implied",
    ]:
        assert_contains(runner, expected, CLI / "src/runner.rs")

    for source in [parser, runner, read(SDK / "src/lib.rs")]:
        lowered = source.lower()
        for forbidden in [
            "postgres://",
            "redis://",
            "nats://",
            "kafka://",
            "s3://",
            "vault://",
            "minio://",
            "node-agent://",
        ]:
            if forbidden in lowered:
                raise AssertionError(f"Phase 5 CLI/SDK source contains forbidden direct target: {forbidden}")


def validate_envelope(
    payload: dict[str, Any],
    *,
    ok: bool = True,
    exit_class: str = "success",
    exit_code: int = 0,
) -> None:
    missing = ENVELOPE_FIELDS - set(payload)
    if missing:
        raise AssertionError(f"JSON envelope missing fields: {sorted(missing)}")
    if payload["ok"] is not ok:
        raise AssertionError(f"Envelope ok mismatch: {payload['ok']}")
    if payload["exit_class"] != exit_class:
        raise AssertionError(f"Envelope exit_class mismatch: {payload['exit_class']}")
    if payload["exit_code"] != exit_code:
        raise AssertionError(f"Envelope exit_code mismatch: {payload['exit_code']}")
    if payload["schema_version"] != "cli-command.v0.1":
        raise AssertionError("Envelope schema version changed")
    if payload["retry_class"] != "not_retryable":
        raise AssertionError("Phase 5 smoke outputs must remain not_retryable")
    assert_secret_free(json.dumps(payload, sort_keys=True), "CLI JSON envelope")

    lifecycle = payload["lifecycle"]
    if "states" not in lifecycle or "terminal_state" not in lifecycle:
        raise AssertionError("Envelope lifecycle missing states or terminal_state")
    if lifecycle["terminal_state"] not in {"completed", "failed", "denied"}:
        raise AssertionError(f"Unexpected terminal lifecycle state: {lifecycle['terminal_state']}")

    diagnostic = payload["diagnostic_bundle"]
    if diagnostic["redaction_policy"] != "secret_free_refs_only":
        raise AssertionError("Diagnostic bundle must remain secret-free")


def assert_phase5_result(payload: dict[str, Any], command: str, family: str, signed: bool) -> dict[str, Any]:
    validate_envelope(payload)
    result = payload["result"]
    if result["command"] != command:
        raise AssertionError(f"Unexpected command result: {result['command']}")
    if result["family"] != family:
        raise AssertionError(f"Unexpected command family: {result['family']}")
    if result["phase_gate"] != "phase_1_control_plane_bootstrap":
        raise AssertionError("Phase 5 command must stay Phase 1 bootstrap gated")
    if result["sdk_target"] != "overgate_only":
        raise AssertionError("Phase 5 command must use Overgate-only SDK target")
    if result["submitted_via"] != "sdk_overgate_contract":
        raise AssertionError("Phase 5 command must submit through SDK/Overgate contract")
    if result["signed"] is not signed:
        raise AssertionError(f"Signed flag mismatch for {command}: {result['signed']}")
    if result["trace_id"] != payload["trace_id"]:
        raise AssertionError("Result trace id must match envelope trace id")

    routes = payload["capabilities"]["routes"]
    if not routes or routes[0]["route"] != family or not routes[0]["available"]:
        raise AssertionError(f"Capability route missing or unavailable for {family}")

    if signed:
        signed_envelope = result["signed_command_envelope"]
        if not signed_envelope:
            raise AssertionError(f"{command} did not render signed_command_envelope")
        if signed_envelope["family"] != family or signed_envelope["command_type"] != command:
            raise AssertionError(f"Signed envelope command identity mismatch for {command}")
        if not signed_envelope["signature_ref"].startswith("sigref:"):
            raise AssertionError(f"Signed envelope signature ref is not a ref for {command}")
        if signed_envelope["exposes_key_material"] is not False:
            raise AssertionError(f"Signed envelope exposed key material for {command}")
        if result["signature_ref"] != signed_envelope["signature_ref"]:
            raise AssertionError(f"Result signature ref mismatch for {command}")
        if not result["acceptance"] or not result["acceptance"]["audit_refs"]:
            raise AssertionError(f"Signed command missing acceptance audit refs for {command}")
        if not payload["audit_refs"]:
            raise AssertionError(f"Signed command missing top-level audit refs for {command}")
    else:
        if result["signed_command_envelope"] is not None or result["signature_ref"] is not None:
            raise AssertionError(f"Read-only command rendered signing data for {command}")
        if result["acceptance"] is not None:
            raise AssertionError(f"Read-only command rendered acceptance data for {command}")

    return result


def validate_cli_phase5_behavior() -> None:
    profile = LOCAL_PROFILE_ARGS

    whoami = run_cli(["auth", "whoami", "--json", *profile])
    whoami_result = assert_phase5_result(whoami, "auth whoami", "auth", signed=False)
    if whoami_result["auth_status"] != "authenticated":
        raise AssertionError("auth whoami must report authenticated actor state")

    tenant_create_a = run_cli(["tenant", "create", "--json", "--expected-state", "absent", *profile])
    tenant_a = assert_phase5_result(tenant_create_a, "tenant create", "tenant", signed=True)
    tenant_create_b = run_cli(["tenant", "create", "--json", "--expected-state", "absent", *profile])
    tenant_b = tenant_create_b["result"]
    if tenant_a["idempotency_key"] != tenant_b["idempotency_key"]:
        raise AssertionError("Default tenant create idempotency key must be deterministic")
    if tenant_a["signed_command_envelope"]["expected_state"] != "absent":
        raise AssertionError("Expected state did not flow into signed tenant envelope")
    if tenant_a["acceptance"]["pending_state"] != "accepted":
        raise AssertionError("Tenant create acceptance must be accepted")

    missing_reason = run_cli(["key", "revoke", "--json", *profile], expected_exit=2)
    validate_envelope(missing_reason, ok=False, exit_class="usage", exit_code=2)
    if missing_reason["reason_code"] != "missing_reason":
        raise AssertionError("key revoke without reason must fail with missing_reason")

    key_revoke = run_cli(["key", "revoke", "--json", "--reason", "rotate compromised test ref", *profile])
    key_result = assert_phase5_result(key_revoke, "key revoke", "key", signed=True)
    if key_result["key_state"] != "revoked":
        raise AssertionError("key revoke must report revoked state")
    if key_result["signed_command_envelope"]["reason"] != "rotate compromised test ref":
        raise AssertionError("Admin reason did not flow into signed key envelope")

    manifest_validate = run_cli(
        [
            "manifest",
            "validate",
            "--json",
            "--manifest-kind",
            "workload",
            "--manifest-ref",
            "manifest_local",
            *profile,
        ]
    )
    manifest_validate_result = assert_phase5_result(
        manifest_validate, "manifest validate", "manifest", signed=False
    )
    if manifest_validate_result["local_validation"] != "accepted":
        raise AssertionError("manifest validate must stay local accepted validation")

    manifest_submit = run_cli(
        [
            "manifest",
            "submit",
            "--json",
            "--manifest-kind",
            "workload",
            "--manifest-ref",
            "manifest_local",
            *profile,
        ]
    )
    manifest_submit_result = assert_phase5_result(
        manifest_submit, "manifest submit", "manifest", signed=True
    )
    manifest_ref = manifest_submit_result["manifest_bootstrap_ref"]
    if manifest_ref["immutable_ref"] != "manifest:manifest_local:immutable":
        raise AssertionError("manifest submit immutable ref changed")
    if manifest_ref["submitted_via"] != "sdk_overgate_contract":
        raise AssertionError("manifest submit did not use SDK/Overgate contract")

    workload_submit = run_cli(
        [
            "workload",
            "submit",
            "--json",
            "--workload-kind",
            "synthetic",
            "--workload-ref",
            "workload_local",
            *profile,
        ]
    )
    workload_submit_result = assert_phase5_result(
        workload_submit, "workload submit", "workload", signed=True
    )
    pending = workload_submit_result["synthetic_workload_pending_state"]
    if pending["queue_state"] != "pending" or pending["execution_implied"] is not False:
        raise AssertionError("Synthetic workload submit must stop at pending without implying execution")
    if "waiting" not in workload_submit["lifecycle"]["states"]:
        raise AssertionError("Synthetic workload submit must expose waiting lifecycle state")
    if workload_submit_result["acceptance"]["pending_state"] != "pending":
        raise AssertionError("Synthetic workload submit acceptance must be pending")

    workload_timeline = run_cli(
        [
            "workload",
            "timeline",
            "--json",
            "--workload-kind",
            "synthetic",
            "--workload-ref",
            "workload_local",
            *profile,
        ]
    )
    workload_timeline_result = assert_phase5_result(
        workload_timeline, "workload timeline", "workload", signed=False
    )
    if workload_timeline_result["synthetic_workload_pending_state"]["queue_state"] != "pending":
        raise AssertionError("Synthetic workload timeline must stay pending")
    if not workload_timeline_result.get("timeline_events"):
        raise AssertionError("Synthetic workload timeline must include timeline events")

    package_build = run_cli(["package", "build", "--json"], expected_exit=7)
    validate_envelope(package_build, ok=False, exit_class="phase", exit_code=7)
    if package_build["reason_code"] != "not_available_in_phase":
        raise AssertionError("Later package commands must stay phase gated")
    if package_build["error"]["phase_gate"] != "phase_9":
        raise AssertionError("Package commands must point at Phase 9")
    if package_build["capabilities"]["fail_closed"] is not True:
        raise AssertionError("Phase-gated package commands must fail closed")

    custom = run_cli(
        [
            "tenant",
            "create",
            "--json",
            "--idempotency-key",
            "idem_custom_test",
            "--trace-id",
            "trace_custom_test",
            "--expected-state",
            "absent",
            "--target-ref",
            "tenant_custom",
            *profile,
        ]
    )
    custom_result = assert_phase5_result(custom, "tenant create", "tenant", signed=True)
    if custom_result["idempotency_key"] != "idem_custom_test":
        raise AssertionError("Explicit idempotency key did not flow into result")
    if custom_result["trace_id"] != "trace_custom_test" or custom["trace_id"] != "trace_custom_test":
        raise AssertionError("Explicit trace id did not flow through envelope")
    signed = custom_result["signed_command_envelope"]
    if signed["target_ref"] != "tenant_custom" or signed["idempotency_key"] != "idem_custom_test":
        raise AssertionError("Explicit target/idempotency did not flow into signed envelope")


def validate_docs_and_wrapper() -> None:
    sub_plan = read(SUB_PLAN)
    for expected in [
        "### Phase 5 Gate Outputs",
        "Bootstrap parser gate",
        "Signed envelope gate",
        "Manifest bootstrap gate",
        "Synthetic workload gate",
        "Validation gate",
        "scripts/validate_cli_phase5.py",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    service = read(SERVICE)
    for expected in [
        "## Phase 5 Implementation Gates",
        "Bootstrap parser gate",
        "Signed envelope gate",
        "Manifest bootstrap gate",
        "Synthetic workload gate",
        "scripts/validate_cli_phase5.py",
    ]:
        assert_contains(service, expected, SERVICE)

    cli_readme = read(CLI / "README.md")
    for expected in [
        "Phase 5 Rust CLI bootstrap",
        "auth, tenant, identity, key, manifest, and synthetic workload",
        "signed command envelopes",
        "pending state without implying execution",
    ]:
        assert_contains(cli_readme, expected, CLI / "README.md")

    contracts_readme = read(CONTRACTS / "README.md")
    for expected in [
        "Phase 2 through Phase 5",
        "phase1_bootstrap_command",
        "signed_command_envelope",
        "bootstrap_acceptance_record",
        "manifest_bootstrap_ref",
        "synthetic_workload_pending_state",
    ]:
        assert_contains(contracts_readme, expected, CONTRACTS / "README.md")

    phase_plan = read(PHASE_PLAN)
    for expected in ["# CLI Phase 05 Plan", "scripts/validate_cli_phase5.py"]:
        assert_contains(phase_plan, expected, PHASE_PLAN)

    phase_progress = read(PHASE_PROGRESS)
    for expected in ["# CLI Phase 05 Progress", "Run focused and full validation"]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)

    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_cli_phase5.py")', VALIDATION_WRAPPER)

    tech_stack = read(TECH_STACK)
    assert_contains(tech_stack, "| CLI | Rust CLI using generated contracts", TECH_STACK)


def validate_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-contracts", "-p", "overrid-sdk", "-p", "overrid-cli"])


def main() -> int:
    try:
        validate_required_files()
        validate_schema_and_manifest()
        validate_rust_phase5_surface()
        validate_docs_and_wrapper()
        validate_cli_phase5_behavior()
        validate_cargo_tests()
    except AssertionError as exc:
        print(f"CLI Phase 5 validation failed: {exc}", file=sys.stderr)
        return 1
    print("CLI Phase 5 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
