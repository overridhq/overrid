#!/usr/bin/env python3
"""Validate CLI Phase 8 policy, package, usage, receipt, ledger, and dispute gates."""

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
PHASE8_FIXTURE = CLI / "fixtures/valid/phase8_policy_package_accounting.valid.json"
SUB_PLAN = Path("docs/build_plan/sub_build_plan_002_cli.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/cli_phase_08_plan.md")
PHASE_PROGRESS = Path("docs/planning/cli_phase_08_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

PHASE8_CONTRACTS = {
    "policy_dry_run_decision",
    "package_validation_summary",
    "usage_oru_rollup",
    "receipt_ledger_read",
    "dispute_read_model",
}

POLICY_TARGETS = {
    "policy_accept": ("accepted", None),
    "policy_denied_egress": ("denied", "policy.egress_denied"),
    "policy_wrong_tenant": ("denied", "policy.wrong_tenant"),
    "policy_insufficient_trust": ("denied", "policy.insufficient_trust"),
    "policy_quota_exhausted": ("denied", "policy.quota_exhausted"),
    "policy_unsupported_workload": ("denied", "policy.unsupported_workload_class"),
}

PACKAGE_TARGETS = {
    "package_accept": ("accepted", None),
    "package_invalid": ("invalid_package", "package.invalid"),
    "package_unsupported_version": ("unsupported_version", "package.unsupported_version"),
    "package_missing_provenance": ("missing_provenance", "package.missing_provenance"),
    "package_policy_incompatible": ("policy_incompatible", "package.policy_incompatible"),
}

ORU_UNITS = {"CPU-ORU", "GPU-ORU", "STOR-ORU", "NET-ORU", "MEM-ORU", "DATA-ORU"}

SECRET_MARKERS = [
    "-----begin",
    "private key",
    "raw_key",
    "secret=",
    "token=",
    "decrypted_payload",
    "private_payload=",
    "node-agent://",
    "s3://",
    "minio://",
]

FORBIDDEN_ASSUMPTION_MARKERS = [
    "price_per",
    "revenue_projection",
    "customer_count_projection",
    "market_volume_projection",
    "blockchain_tx",
    "nft_ref",
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
    PHASE8_FIXTURE,
    CONTRACTS / "src/lib.rs",
    CONTRACTS / "README.md",
    CLI / "src/parser.rs",
    CLI / "src/runner.rs",
    CLI / "README.md",
    SUB_PLAN,
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


def assert_secret_free(text: str, source: str) -> None:
    lowered = text.lower()
    for marker in SECRET_MARKERS:
        if marker in lowered:
            raise AssertionError(f"{source} leaked forbidden marker: {marker}")


def assert_no_forbidden_assumptions(text: str, source: str) -> None:
    lowered = text.lower()
    for marker in FORBIDDEN_ASSUMPTION_MARKERS:
        if marker in lowered:
            raise AssertionError(f"{source} contains forbidden assumption marker: {marker}")


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
    assert_no_forbidden_assumptions(output, " ".join(args))
    try:
        return json.loads(output)
    except json.JSONDecodeError as exc:
        raise AssertionError(f"CLI output is not JSON for {' '.join(args)}: {output}") from exc


def validate_required_files() -> None:
    for path in REQUIRED_FILES:
        if not (REPO_ROOT / path).is_file():
            raise AssertionError(f"Missing required file: {path}")


def validate_schema_and_manifest() -> None:
    schema = read_json(CONTRACTS_SCHEMA)
    manifest = read_json(CONTRACTS_MANIFEST)
    defs = schema["$defs"]

    actual_contracts = set(schema["properties"]["contracts"]["items"]["enum"])
    missing_contracts = PHASE8_CONTRACTS - actual_contracts
    if missing_contracts:
        raise AssertionError(f"CLI schema is missing Phase 8 contracts: {sorted(missing_contracts)}")
    missing_defs = PHASE8_CONTRACTS - set(defs)
    if missing_defs:
        raise AssertionError(f"CLI schema is missing Phase 8 $defs: {sorted(missing_defs)}")

    policy = defs["policy_dry_run_decision"]["properties"]
    if policy["evaluated_via"].get("const") != "sdk_overgate_contract":
        raise AssertionError("policy dry-run decisions must route through SDK/Overgate")
    if policy["mutates_platform_state"].get("const") is not False:
        raise AssertionError("policy dry-run decisions must be non-mutating")
    if policy["direct_policy_service_access"].get("const") is not False:
        raise AssertionError("policy dry-run must forbid direct policy-service access")

    package = defs["package_validation_summary"]["properties"]
    for key in [
        "schema_checked",
        "signature_checked",
        "hash_checked",
        "dependency_checked",
        "permission_checked",
        "policy_compatibility_checked",
    ]:
        if package[key].get("const") is not True:
            raise AssertionError(f"package validation must require {key}")
    if package["direct_package_store_access"].get("const") is not False:
        raise AssertionError("package validation must forbid direct package-store access")

    usage = defs["usage_oru_rollup"]["properties"]
    if set(usage["units"]["items"]["enum"]) != ORU_UNITS:
        raise AssertionError("usage_oru_rollup must cover every ORU unit")
    if usage["payment_behavior_created"].get("const") is not False:
        raise AssertionError("usage reads must not create payment behavior")
    if usage["direct_meter_access"].get("const") is not False:
        raise AssertionError("usage reads must forbid direct meter access")

    receipt = defs["receipt_ledger_read"]["properties"]
    for key in [
        "pricing_assumptions_present",
        "revenue_assumptions_present",
        "customer_count_assumptions_present",
        "market_volume_assumptions_present",
        "direct_ledger_access",
    ]:
        if receipt[key].get("const") is not False:
            raise AssertionError(f"receipt/ledger reads must keep {key}=false")

    dispute = defs["dispute_read_model"]["properties"]
    if dispute["tenant_role_filtered"].get("const") is not True:
        raise AssertionError("dispute reads must be tenant/role filtered")
    if dispute["direct_dispute_mutation"].get("const") is not False:
        raise AssertionError("dispute reads must not mutate disputes")
    if dispute["direct_ledger_mutation"].get("const") is not False:
        raise AssertionError("dispute reads must not mutate ledger state")

    if set(manifest.get("phase8_contracts", [])) != PHASE8_CONTRACTS:
        raise AssertionError("Manifest phase8_contracts does not match schema extensions")
    fixture_paths = manifest.get("fixtures", {}).get("phase8_valid", [])
    if str(PHASE8_FIXTURE) not in fixture_paths:
        raise AssertionError("Manifest is missing Phase 8 fixture path")
    guardrails = "\n".join(manifest.get("guardrails", []))
    for expected in [
        "Phase 8 policy dry-run commands",
        "Phase 8 package validation commands",
        "Phase 8 usage, receipt, ledger, and dispute read commands",
        "Phase 8 accounting outputs must not encode pricing",
    ]:
        if expected not in guardrails:
            raise AssertionError(f"Manifest guardrails missing Phase 8 text: {expected}")


def validate_fixture_and_docs() -> None:
    fixture = read_json(PHASE8_FIXTURE)
    if set(fixture["usage_units"]) != ORU_UNITS:
        raise AssertionError("Phase 8 fixture must cover all ORU units")
    if set(fixture["policy_targets"]) != set(POLICY_TARGETS):
        raise AssertionError("Phase 8 fixture policy targets drifted")
    if set(fixture["package_targets"]) != set(PACKAGE_TARGETS):
        raise AssertionError("Phase 8 fixture package targets drifted")
    guardrails = fixture["guardrails"]
    for key in [
        "mutates_platform_state",
        "direct_policy_service_access",
        "direct_package_store_access",
        "direct_meter_access",
        "direct_ledger_access",
        "direct_dispute_mutation",
        "direct_ledger_mutation",
        "payment_behavior_created",
        "pricing_assumptions_present",
        "revenue_assumptions_present",
        "customer_count_assumptions_present",
        "market_volume_assumptions_present",
    ]:
        if guardrails[key] is not False:
            raise AssertionError(f"Phase 8 fixture guardrail must keep {key}=false")

    for path in [SUB_PLAN, TECH_STACK, PHASE_PLAN, PHASE_PROGRESS, CLI / "README.md", CONTRACTS / "README.md"]:
        text = read(path)
        assert_no_forbidden_assumptions(text, str(path))

    sub_plan = read(SUB_PLAN)
    for expected in ["policy dry-run", "package validation", "usage", "receipt", "dispute"]:
        assert_contains(sub_plan, expected, SUB_PLAN)
    plan = read(PHASE_PLAN)
    for expected in ["policy dry-run", "package validation", "usage/ORU", "dispute read"]:
        assert_contains(plan, expected, PHASE_PLAN)
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, "scripts/validate_cli_phase8.py", VALIDATION_WRAPPER)


def validate_cli_outputs() -> None:
    for target_ref, (expected_decision, expected_reason) in POLICY_TARGETS.items():
        output = run_cli(["policy", "dry-run", "--json", "--target-ref", target_ref, *LOCAL_PROFILE_ARGS])
        assert output["ok"] is True
        result = output["result"]["policy_dry_run_decision"]
        assert result["decision"] == expected_decision
        assert result["evaluated_via"] == "sdk_overgate_contract"
        assert result["mutates_platform_state"] is False
        assert result["direct_policy_service_access"] is False
        if expected_reason is None:
            assert result["reason_codes"] == []
        else:
            assert expected_reason in result["reason_codes"]

    for target_ref, (expected_state, expected_reason) in PACKAGE_TARGETS.items():
        output = run_cli(["package", "validate", "--json", "--target-ref", target_ref, *LOCAL_PROFILE_ARGS])
        result = output["result"]["package_validation_summary"]
        assert result["validation_state"] == expected_state
        for key in [
            "schema_checked",
            "signature_checked",
            "hash_checked",
            "dependency_checked",
            "permission_checked",
            "policy_compatibility_checked",
        ]:
            assert result[key] is True
        assert result["direct_package_store_access"] is False
        if expected_state == "missing_provenance":
            assert result["provenance_available"] is False
        if expected_reason is None:
            assert result["reason_codes"] == []
        else:
            assert expected_reason in result["reason_codes"]

    usage = run_cli(["usage", "show", "--json", "--target-ref", "usage_budget_exhausted", *LOCAL_PROFILE_ARGS])
    rollup = usage["result"]["usage_oru_rollup"]
    assert set(rollup["units"]) == ORU_UNITS
    assert rollup["budget_state"] == "budget_exhausted"
    assert rollup["payment_behavior_created"] is False
    assert rollup["direct_meter_access"] is False

    disputed = run_cli(["usage", "show", "--json", "--target-ref", "usage_disputed", *LOCAL_PROFILE_ARGS])
    assert disputed["result"]["usage_oru_rollup"]["disputed_usage"] is True

    receipt = run_cli(["receipt", "show", "--json", "--target-ref", "receipt_local", *LOCAL_PROFILE_ARGS])
    read_model = receipt["result"]["receipt_ledger_read"]
    for prefix in ["seal-ledger:entry", "overbill:receipt"]:
        if not any(ref.startswith(prefix) for ref in read_model["ledger_refs"]):
            raise AssertionError(f"receipt output missing {prefix} ref")
    for key in [
        "pricing_assumptions_present",
        "revenue_assumptions_present",
        "customer_count_assumptions_present",
        "market_volume_assumptions_present",
        "direct_ledger_access",
    ]:
        assert read_model[key] is False

    ledger = run_cli(["ledger", "inspect", "--json", "--target-ref", "ledger_local", *LOCAL_PROFILE_ARGS])
    assert ledger["result"]["receipt_ledger_read"]["direct_ledger_access"] is False

    dispute = run_cli(["dispute", "inspect", "--json", "--target-ref", "dispute_resolved_released", *LOCAL_PROFILE_ARGS])
    model = dispute["result"]["dispute_read_model"]
    assert model["hold_status"] == "released"
    assert model["resolution_state"] == "resolved"
    assert model["tenant_role_filtered"] is True
    assert model["direct_dispute_mutation"] is False
    assert model["direct_ledger_mutation"] is False

    deploy = run_cli(["deployment", "--json"], expected_exit=7)
    assert deploy["ok"] is False
    assert deploy["reason_code"] == "not_available_in_phase"
    assert deploy["error"]["phase_gate"] == "phase_9"


def validate_rust_tests() -> None:
    run(["cargo", "test", "-p", "overrid-contracts", "-p", "overrid-sdk", "-p", "overrid-cli"])


def main() -> int:
    try:
        validate_required_files()
        validate_schema_and_manifest()
        validate_fixture_and_docs()
        validate_cli_outputs()
        validate_rust_tests()
    except AssertionError as exc:
        print(f"validate_cli_phase8.py failed: {exc}", file=sys.stderr)
        return 1
    print("validate_cli_phase8.py passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
