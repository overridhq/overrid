#!/usr/bin/env python3
"""Validate CLI Phase 3 profile, credential, and environment guardrails."""

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
PHASE_PLAN = Path("docs/planning/cli_phase_03_plan.md")
PHASE_PROGRESS = Path("docs/planning/cli_phase_03_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

PHASE3_CONTRACTS = {
    "environment_class",
    "cli_profile",
    "credential_reference",
    "confirmation_policy",
    "profile_storage_policy",
    "signer_handoff",
}

CREDENTIAL_CLASSES = {
    "keychain",
    "secret_service",
    "encrypted_store",
    "signing_agent",
    "hardware_token",
    "fixture",
    "ci_reference",
}

PROFILE_REQUIRED_FIELDS = {
    "profile_name",
    "endpoint",
    "endpoint_fingerprint",
    "environment_class",
    "tenant_id",
    "actor_id",
    "credential_namespace",
    "allowed_credential_classes",
    "fixture_allowance",
    "default_output_mode",
    "confirmation_policy",
    "schema_pins",
}

SECRET_MARKERS = [
    "-----begin",
    "private key",
    "raw_key",
    "secret=",
    "token=",
    "decrypted_payload",
    "private_payload=",
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

SEED_PROFILE_ARGS = [
    "--profile",
    "seed",
    "--environment",
    "seed",
    "--endpoint",
    "https://overgate.seed.overrid.local",
    "--endpoint-fingerprint",
    "fp_seed",
    "--tenant",
    "tenant_seed",
    "--actor",
    "actor_seed",
    "--credential-namespace",
    "seed",
    "--credential-class",
    "keychain",
    "--credential-ref",
    "keychain://overrid/seed/key-1",
    "--key-id",
    "seed-key-1",
]

REQUIRED_FILES = [
    CONTRACTS_SCHEMA,
    CONTRACTS_MANIFEST,
    CONTRACTS / "src/lib.rs",
    SDK / "src/lib.rs",
    CLI / "src/parser.rs",
    CLI / "src/runner.rs",
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
        joined = " ".join(command)
        raise AssertionError(f"Command failed with exit {result.returncode}: {joined}")


def run_capture(command: list[str], expected_exit: int = 0) -> str:
    result = subprocess.run(command, cwd=REPO_ROOT, text=True, capture_output=True)
    if result.returncode != expected_exit:
        joined = " ".join(command)
        raise AssertionError(
            f"Command failed with exit {result.returncode}, expected {expected_exit}: "
            f"{joined}\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
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
    missing = PHASE3_CONTRACTS - actual_contracts
    if missing:
        raise AssertionError(f"CLI schema is missing Phase 3 contracts: {sorted(missing)}")

    defs = schema["$defs"]
    for definition in [
        "environment_class",
        "cli_profile",
        "credential_reference",
        "signer_handoff",
        "confirmation_policy",
    ]:
        if definition not in defs:
            raise AssertionError(f"CLI schema is missing $defs.{definition}")

    credential_classes = set(defs["credential_reference_class"]["enum"])
    if credential_classes != CREDENTIAL_CLASSES:
        raise AssertionError(f"Credential class enum mismatch: {sorted(credential_classes)}")

    profile_required = set(defs["cli_profile"]["required"])
    missing_profile_fields = PROFILE_REQUIRED_FIELDS - profile_required
    if missing_profile_fields:
        raise AssertionError(f"cli_profile missing required fields: {sorted(missing_profile_fields)}")

    exposes = defs["credential_reference"]["properties"]["exposes_key_material"]
    if exposes.get("const") is not False:
        raise AssertionError("credential_reference must require exposes_key_material:false")

    handoff = defs["signer_handoff"]
    if handoff["properties"]["signature_ref"].get("pattern") != "^sigref:[A-Za-z0-9._:-]+$":
        raise AssertionError("signer_handoff.signature_ref must use the sigref pattern")
    if handoff["properties"]["exposes_key_material"].get("const") is not False:
        raise AssertionError("signer_handoff must require exposes_key_material:false")

    if set(manifest.get("phase3_contracts", [])) != PHASE3_CONTRACTS:
        raise AssertionError("Manifest phase3_contracts does not match the schema extensions")
    if manifest["rust_projection"]["non_authoritative"] is not True:
        raise AssertionError("Rust projection must remain non-authoritative")


def validate_rust_phase3_surface() -> None:
    contracts = read(CONTRACTS / "src/lib.rs")
    for expected in [
        "EnvironmentClass",
        "CredentialReferenceClass",
        "FixtureAllowance",
        "ConfirmationPolicy",
        "CliProfile",
        "CredentialReference",
        "SignerHandoff",
        "validate_owner_only_file_mode",
        "CredentialRevoked",
        "CredentialExpired",
        "RawKeyMaterialInReference",
        "UnsafeFilePermissions",
    ]:
        assert_contains(contracts, expected, CONTRACTS / "src/lib.rs")

    sdk = read(SDK / "src/lib.rs")
    for expected in [
        "CommandSafetyInput",
        "enforce_profile_environment",
        "prepare_signer_handoff",
        "Credential(ProfileValidationError)",
        "UnsafeEndpointOverride",
        "MissingProfileConfirmation",
        "MissingExplicitProfile",
        "MissingReason",
    ]:
        assert_contains(sdk, expected, SDK / "src/lib.rs")

    parser = read(CLI / "src/parser.rs")
    for expected in [
        "--environment",
        "--endpoint",
        "--endpoint-fingerprint",
        "--credential-class",
        "--credential-ref",
        "--credential-namespace",
        "--confirm-profile",
        "--reason",
        "--test-harness-profile",
        "ProfileCommand::Create",
        "CredentialCommand::Enroll",
    ]:
        assert_contains(parser, expected, CLI / "src/parser.rs")

    runner = read(CLI / "src/runner.rs")
    for expected in [
        "profile_command_result",
        "credential_command_result",
        "owner_only_file_backed_config",
        "signature_ref",
        "exposes_key_material",
        "missing_profile_confirmation",
        "credential_validation_failed",
    ]:
        assert_contains(runner, expected, CLI / "src/runner.rs")


def validate_cli_behavior() -> None:
    profile = run_cli(["profile", "inspect", "--json", *LOCAL_PROFILE_ARGS])
    if profile["ok"] is not True:
        raise AssertionError("profile inspect should succeed")
    result = profile["result"]
    expected_profile = {
        "profile_name": "local-dev",
        "environment_class": "local",
        "endpoint_fingerprint": "fp_local",
        "tenant_id": "tenant_local",
        "actor_id": "actor_local",
        "credential_namespace": "local-dev",
        "storage_policy": "owner_only_file_backed_config",
    }
    for key, expected in expected_profile.items():
        if result.get(key) != expected:
            raise AssertionError(f"profile inspect wrong {key}: {result.get(key)}")

    profile_list = run_cli(["profile", "list", "--json"])
    if profile_list["result"]["storage_policy"] != "owner_only_file_backed_config":
        raise AssertionError("profile list must expose owner-only storage policy")

    credential = run_cli(["credential", "enroll", "--json", *LOCAL_PROFILE_ARGS])
    credential_result = credential["result"]
    if not credential_result["signature_ref"].startswith("sigref:"):
        raise AssertionError("credential enroll must return a sigref signer handoff")
    if credential_result["exposes_key_material"] is not False:
        raise AssertionError("credential enroll must not expose key material")

    missing_confirmation = run_cli(
        ["credential", "enroll", "--json", *SEED_PROFILE_ARGS],
        expected_exit=2,
    )
    assert_error(missing_confirmation, "missing_profile_confirmation", "phase_3")

    endpoint_override = run_cli(
        [
            "credential",
            "inspect",
            "--json",
            *SEED_PROFILE_ARGS,
            "--endpoint-override",
            "https://overgate.other.example",
        ],
        expected_exit=3,
    )
    assert_error(endpoint_override, "unsafe_endpoint_override", "phase_3")

    revoked = run_cli(
        ["credential", "inspect", "--json", *LOCAL_PROFILE_ARGS, "--revoked"],
        expected_exit=4,
    )
    assert_error(revoked, "credential_validation_failed", "phase_3")
    if "credential is revoked" not in revoked["error"]["message"]:
        raise AssertionError("revoked credential error must explain revocation")


def assert_error(payload: dict[str, Any], reason_code: str, phase_gate: str) -> None:
    if payload["ok"] is not False:
        raise AssertionError(f"{reason_code} path should fail")
    error = payload["error"]
    if error["reason_code"] != reason_code:
        raise AssertionError(f"Wrong reason_code: {error['reason_code']}")
    if error["phase_gate"] != phase_gate:
        raise AssertionError(f"Wrong phase gate: {error['phase_gate']}")


def validate_docs_and_wrapper() -> None:
    sub_plan = read(SUB_PLAN)
    service = read(SERVICE)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    wrapper = read(VALIDATION_WRAPPER)

    for expected in [
        "## Phase 3: Profiles, Credential References, And Environment Separation",
        "Profile contract gate",
        "Credential-reference gate",
        "Environment guard gate",
        "Signer handoff gate",
        "scripts/validate_cli_phase3.py",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    for expected in [
        "## Phase 3 Implementation Gates",
        "Profile gate",
        "Credential gate",
        "Environment gate",
        "Signer handoff gate",
        "scripts/validate_cli_phase3.py",
    ]:
        assert_contains(service, expected, SERVICE)

    assert_contains(tech_stack, "| CLI | Rust CLI using generated contracts", TECH_STACK)
    assert_contains(phase_plan, "CLI Phase 3 Execution Plan", PHASE_PLAN)
    assert_contains(phase_progress, "CLI Phase 3 Progress", PHASE_PROGRESS)
    assert_contains(wrapper, "Path(\"scripts/validate_cli_phase3.py\")", VALIDATION_WRAPPER)


def validate_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-contracts", "-p", "overrid-sdk", "-p", "overrid-cli"])


def main() -> int:
    checks = [
        validate_required_files,
        validate_schema_and_manifest,
        validate_rust_phase3_surface,
        validate_cli_behavior,
        validate_docs_and_wrapper,
        validate_cargo_tests,
    ]
    for check in checks:
        check()
    print("CLI Phase 3 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
