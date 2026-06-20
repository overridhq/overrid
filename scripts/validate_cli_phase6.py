#!/usr/bin/env python3
"""Validate CLI Phase 6 idempotency, retry, trace, and error hardening."""

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
PHASE_PLAN = Path("docs/planning/cli_phase_06_plan.md")
PHASE_PROGRESS = Path("docs/planning/cli_phase_06_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

PHASE6_CONTRACTS = {
    "canonical_idempotency_fingerprint",
    "retry_timeout_policy",
    "error_decode_record",
    "local_idempotency_cache_record",
}

FINGERPRINT_FIELDS = {
    "environment_class",
    "endpoint_identity",
    "tenant_id",
    "actor_id",
    "command_type",
    "target_ref",
    "canonical_payload_hash",
    "expected_current_state",
    "reason",
    "schema_version",
    "fingerprint",
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
    missing_contracts = PHASE6_CONTRACTS - actual_contracts
    if missing_contracts:
        raise AssertionError(f"CLI schema is missing Phase 6 contracts: {sorted(missing_contracts)}")

    missing_defs = PHASE6_CONTRACTS - set(defs)
    if missing_defs:
        raise AssertionError(f"CLI schema is missing Phase 6 $defs: {sorted(missing_defs)}")

    fingerprint = defs["canonical_idempotency_fingerprint"]
    if set(fingerprint["required"]) != FINGERPRINT_FIELDS:
        raise AssertionError("canonical_idempotency_fingerprint required fields changed")
    if fingerprint["properties"]["canonical_payload_hash"].get("pattern") != "^hash_[A-Fa-f0-9]{16}$":
        raise AssertionError("canonical payload hash must stay stable and bounded")

    retry_policy = defs["retry_timeout_policy"]
    if retry_policy["properties"]["max_retries"].get("maximum") != 5:
        raise AssertionError("retry_timeout_policy must cap max_retries at 5")
    if retry_policy["properties"]["timeout_ms"].get("maximum") != 600000:
        raise AssertionError("retry_timeout_policy must cap timeout_ms at 600000")
    if retry_policy["properties"]["bounded"].get("const") is not True:
        raise AssertionError("retry_timeout_policy must be bounded")

    error_decode = defs["error_decode_record"]
    if error_decode["properties"]["raw_internal_error_exposed"].get("const") is not False:
        raise AssertionError("error_decode_record must forbid raw internal errors")

    cache = defs["local_idempotency_cache_record"]
    if cache["properties"]["cache_scope"].get("const") != "profile_environment":
        raise AssertionError("local idempotency cache must be scoped by profile/environment")
    if cache["properties"]["contains_private_payload"].get("const") is not False:
        raise AssertionError("local idempotency cache must not contain private payloads")
    if cache["properties"]["owner_only"].get("const") is not True:
        raise AssertionError("local idempotency cache must be owner-only")

    if set(manifest.get("phase6_contracts", [])) != PHASE6_CONTRACTS:
        raise AssertionError("Manifest phase6_contracts does not match the schema extensions")
    guardrails = "\n".join(manifest.get("guardrails", []))
    for expected in [
        "Canonical idempotency fingerprints",
        "Retry and timeout policy",
        "Error decode records",
        "Local idempotency cache records",
    ]:
        if expected not in guardrails:
            raise AssertionError(f"Manifest guardrails missing Phase 6 text: {expected}")


def validate_rust_phase6_surface() -> None:
    contracts = read(CONTRACTS / "src/lib.rs")
    for expected in [
        "CanonicalIdempotencyFingerprint",
        "RetryTimeoutPolicy",
        "ErrorDecodeRecord",
        "LocalIdempotencyCacheRecord",
        "new_operation_idempotency_key",
        "non_retryable_reason_families",
        "raw_internal_error_exposed: false",
        "contains_private_payload: false",
    ]:
        assert_contains(contracts, expected, CONTRACTS / "src/lib.rs")

    sdk = read(SDK / "src/lib.rs")
    for expected in [
        "retry_timeout_policy",
        "decode_phase6_error",
        "RetryTimeoutPolicy::bounded",
        "remediation_hint",
        "retry with the same idempotency key",
        "wait for the owning phase",
    ]:
        assert_contains(sdk, expected, SDK / "src/lib.rs")

    parser = read(CLI / "src/parser.rs")
    for expected in [
        "IdempotencyCacheCommand",
        "--new-idempotency-key",
        "--timeout-ms",
        "--max-retries",
        "idempotency-cache",
    ]:
        assert_contains(parser, expected, CLI / "src/parser.rs")

    runner = read(CLI / "src/runner.rs")
    for expected in [
        "canonical_fingerprint",
        "canonical_payload_hash",
        "retry_timeout_policy",
        "render_retry_timeout_policy_json",
        "render_error_decode_record_json",
        "render_local_idempotency_cache_json",
        "idempotency_key_source",
        "new_operation",
        "canonical_fingerprint",
    ]:
        assert_contains(runner, expected, CLI / "src/runner.rs")


def validate_cli_phase6_behavior() -> None:
    base = run_cli(["tenant", "create", "--json", "--expected-state", "absent", *LOCAL_PROFILE_ARGS])
    repeat = run_cli(["tenant", "create", "--json", "--expected-state", "absent", *LOCAL_PROFILE_ARGS])
    base_result = base["result"]
    repeat_result = repeat["result"]

    if base_result["idempotency_key"] != repeat_result["idempotency_key"]:
        raise AssertionError("Deterministic idempotency key changed across identical safe retry")
    if base_result["idempotency_key_source"] != "canonical_fingerprint":
        raise AssertionError("Default idempotency key must come from canonical fingerprint")
    if set(base_result["canonical_idempotency_fingerprint"]) != FINGERPRINT_FIELDS:
        raise AssertionError("CLI output omitted canonical fingerprint fields")
    if base_result["canonical_idempotency_fingerprint"]["expected_current_state"] != "absent":
        raise AssertionError("Expected state did not enter canonical fingerprint")
    if not base_result["canonical_idempotency_fingerprint"]["canonical_payload_hash"].startswith("hash_"):
        raise AssertionError("Canonical payload hash must use stable hash_ form")

    signed = base_result["signed_command_envelope"]
    if signed["idempotency_key"] != base_result["idempotency_key"]:
        raise AssertionError("Signed envelope idempotency key drifted from result")
    if signed["trace_id"] != base["trace_id"]:
        raise AssertionError("Signed envelope trace id drifted from output envelope")
    if signed["exposes_key_material"] is not False:
        raise AssertionError("Signed envelope exposed key material")
    if not base["audit_refs"] or not base_result["acceptance"]["audit_refs"]:
        raise AssertionError("Mutating command did not emit audit refs")

    policy = base_result["retry_timeout_policy"]
    if policy["bounded"] is not True:
        raise AssertionError("Retry/timeout policy must be bounded")
    if set(policy["retryable_classes"]) != {"safe_retry", "retry_after"}:
        raise AssertionError("Retryable classes must stay transport/platform-safe only")
    for expected in ["schema", "auth", "policy", "phase", "credential", "idempotency_duplicate"]:
        if expected not in policy["non_retryable_reason_families"]:
            raise AssertionError(f"Retry policy omitted non-retryable family: {expected}")

    cache = base_result["local_idempotency_cache"]
    if cache["idempotency_key"] != base_result["idempotency_key"]:
        raise AssertionError("Local cache did not store the rendered idempotency key")
    if cache["owner_only"] is not True or cache["contains_private_payload"] is not False:
        raise AssertionError("Local idempotency cache must be owner-only and payload-free")

    new_operation = run_cli(
        [
            "tenant",
            "create",
            "--json",
            "--expected-state",
            "absent",
            "--new-idempotency-key",
            "--trace-id",
            "trace_cli_new",
            *LOCAL_PROFILE_ARGS,
        ]
    )
    new_result = new_operation["result"]
    if new_result["idempotency_key"] == base_result["idempotency_key"]:
        raise AssertionError("--new-idempotency-key reused the canonical key")
    if new_result["idempotency_key_source"] != "new_operation":
        raise AssertionError("--new-idempotency-key must report new_operation source")
    if not new_result["idempotency_key"].startswith("idem_new_"):
        raise AssertionError("--new-idempotency-key must use idem_new_ prefix")
    if new_operation["trace_id"] != "trace_cli_new":
        raise AssertionError("Custom trace id did not reach output envelope")

    explicit = run_cli(
        [
            "tenant",
            "create",
            "--json",
            "--target-ref",
            "tenant_custom",
            "--idempotency-key",
            "idem_custom_test",
            "--trace-id",
            "trace_custom_test",
            *LOCAL_PROFILE_ARGS,
        ]
    )
    explicit_result = explicit["result"]
    if explicit_result["idempotency_key"] != "idem_custom_test":
        raise AssertionError("Explicit idempotency key did not flow into result")
    if explicit_result["signed_command_envelope"]["target_ref"] != "tenant_custom":
        raise AssertionError("Explicit target ref did not flow into signed envelope")
    if explicit["trace_id"] != "trace_custom_test":
        raise AssertionError("Explicit trace id did not flow through envelope")

    bounded = run_cli(
        [
            "tenant",
            "create",
            "--json",
            "--timeout-ms",
            "4500",
            "--max-retries",
            "3",
            *LOCAL_PROFILE_ARGS,
        ]
    )
    bounded_policy = bounded["result"]["retry_timeout_policy"]
    if bounded_policy["timeout_ms"] != 4500 or bounded_policy["max_retries"] != 3:
        raise AssertionError("Retry/timeout flags did not flow into policy")

    cache_inspect = run_cli(["idempotency-cache", "inspect", "--json", *LOCAL_PROFILE_ARGS])
    cache_reset = run_cli(["idempotency-cache", "reset", "--json", *LOCAL_PROFILE_ARGS])
    if cache_inspect["result"]["cache_action"] != "inspect":
        raise AssertionError("idempotency-cache inspect did not render inspect action")
    if cache_reset["result"]["cache_action"] != "reset":
        raise AssertionError("idempotency-cache reset did not render reset action")
    if cache_reset["result"]["local_idempotency_cache"]["resettable"] is not True:
        raise AssertionError("idempotency-cache reset must remain resettable")

    key_revoke = run_cli(["key", "revoke", "--json", *LOCAL_PROFILE_ARGS], expected_exit=2)
    error = key_revoke["error"]
    if error["reason_code"] != "missing_reason":
        raise AssertionError("Admin-impacting key revoke must fail with missing_reason")
    if error["error_decode_record"]["raw_internal_error_exposed"] is not False:
        raise AssertionError("Error decoder exposed raw internal error")
    if key_revoke["retry_class"] != "not_retryable":
        raise AssertionError("missing_reason must be non-retryable")

    phase_gated = run_cli(["package", "build", "--json"], expected_exit=7)
    if phase_gated["error"]["reason_code"] != "not_available_in_phase":
        raise AssertionError("Phase-gated command must use stable phase reason")
    if phase_gated["error"]["error_decode_record"]["source_family"] != "phase":
        raise AssertionError("Phase-gated error must decode as phase source family")
    if "wait for the owning phase" not in phase_gated["error"]["remediation_hint"]:
        raise AssertionError("Phase-gated remediation hint is not stable")


def validate_docs_and_wrapper() -> None:
    sub_plan = read(SUB_PLAN)
    for expected in [
        "### Phase 6 Gate Outputs",
        "Canonical idempotency gate",
        "Retry and timeout gate",
        "Trace and audit gate",
        "Error decoding gate",
        "Local idempotency cache gate",
        "scripts/validate_cli_phase6.py",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    service = read(SERVICE)
    for expected in [
        "## Phase 6 Implementation Gates",
        "Canonical idempotency gate",
        "Retry and timeout gate",
        "Trace and audit gate",
        "Error decoding gate",
        "Local idempotency cache gate",
        "scripts/validate_cli_phase6.py",
    ]:
        assert_contains(service, expected, SERVICE)

    cli_readme = read(CLI / "README.md")
    for expected in [
        "Phase 6 automation hardening",
        "canonical idempotency fingerprints",
        "bounded retry and timeout policy",
        "stable error decode records",
        "local idempotency cache",
    ]:
        assert_contains(cli_readme, expected, CLI / "README.md")

    contracts_readme = read(CONTRACTS / "README.md")
    for expected in [
        "Phase 6 hardening contracts",
        "canonical_idempotency_fingerprint",
        "retry_timeout_policy",
        "error_decode_record",
        "local_idempotency_cache_record",
    ]:
        assert_contains(contracts_readme, expected, CONTRACTS / "README.md")

    phase_plan = read(PHASE_PLAN)
    for expected in ["# CLI Phase 06 Plan", "scripts/validate_cli_phase6.py"]:
        assert_contains(phase_plan, expected, PHASE_PLAN)

    phase_progress = read(PHASE_PROGRESS)
    for expected in ["# CLI Phase 06 Progress", "Run focused and full validation"]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)

    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_cli_phase6.py")', VALIDATION_WRAPPER)

    tech_stack = read(TECH_STACK)
    assert_contains(tech_stack, "| CLI | Rust CLI using generated contracts", TECH_STACK)


def validate_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-contracts", "-p", "overrid-sdk", "-p", "overrid-cli"])


def main() -> int:
    try:
        validate_required_files()
        validate_schema_and_manifest()
        validate_rust_phase6_surface()
        validate_docs_and_wrapper()
        validate_cli_phase6_behavior()
        validate_cargo_tests()
    except AssertionError as exc:
        print(f"CLI Phase 6 validation failed: {exc}", file=sys.stderr)
        return 1
    print("CLI Phase 6 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
