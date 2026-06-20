#!/usr/bin/env python3
"""Validate CLI Phase 9 product integration and CI automation gates."""

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
PHASE9_FIXTURE = CLI / "fixtures/valid/phase9_product_integration.valid.json"
SUB_PLAN = Path("docs/build_plan/sub_build_plan_002_cli.md")
SERVICE = Path("docs/service_catalog/foundation/cli.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/cli_phase_09_plan.md")
PHASE_PROGRESS = Path("docs/planning/cli_phase_09_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

PHASE9_CONTRACTS = {"product_workflow_recipe", "ci_automation_profile"}
PRODUCTS = {"docdex", "mcoda", "codali"}
CI_CREDENTIAL_REF_KINDS = {
    "ci_reference",
    "signing_agent",
    "fixture",
    "hardware_token",
    "mounted_secret_ref",
}
PRODUCT_WORKLOAD_KINDS = {
    "docdex": "docdex_encrypted_index",
    "mcoda": "mcoda_agent_workload",
    "codali": "codali_code_agent_package",
}
PRODUCT_REQUIRED_REFS = {
    "docdex": {
        "encrypted_index_ref",
        "retrieval_job_ref",
        "search_result_ref",
        "usage_rollup_ref",
        "receipt_ref",
    },
    "mcoda": {
        "agent_workload_ref",
        "dynamic_model_metadata_ref",
        "resource_metadata_ref",
        "tool_boundary_ref",
        "budget_ref",
        "usage_rollup_ref",
    },
    "codali": {
        "code_agent_package_ref",
        "repository_context_ref",
        "execution_log_ref",
        "artifact_refs",
        "repair_boundary_ref",
        "phase_usage_ref",
    },
}
PRODUCT_REQUIRED_COMMANDS = {
    "docdex": {
        "overrid workload submit",
        "overrid workload status",
        "overrid workload result",
        "overrid workload cancel",
        "overrid usage show",
        "overrid receipt show",
    },
    "mcoda": {
        "overrid workload submit",
        "overrid workload status",
        "overrid workload logs",
        "overrid workload result",
        "overrid workload cancel",
        "overrid usage show",
    },
    "codali": {
        "overrid package validate",
        "overrid workload submit",
        "overrid workload logs",
        "overrid workload result",
        "overrid workload cancel",
        "overrid usage show",
    },
}
EXPECTED_FAILURE_MODES = {
    "policy.egress_denied",
    "budget.exhausted",
    "tool.boundary_denied",
    "policy.resource_denied",
    "package.invalid",
    "repo_context.ref_denied",
    "repair.boundary_exceeded",
}

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
    "gpt-4",
    "gpt-5",
    "claude-",
    "gemini-",
    "openai-only",
    "anthropic-only",
    "paid_service_required",
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

CI_PROFILE_ARGS = [
    "--profile",
    "ci-automation",
    "--environment",
    "ci",
    "--endpoint",
    "http://127.0.0.1:18080/overgate",
    "--endpoint-fingerprint",
    "fp_ci",
    "--tenant",
    "tenant_ci",
    "--actor",
    "actor_ci",
    "--credential-namespace",
    "ci",
    "--credential-class",
    "ci_reference",
    "--credential-ref",
    "ci://overrid/service-account/short-lived",
    "--key-id",
    "ci-key-1",
    "--fixture-allowance",
    "test_harness_only",
]

REQUIRED_FILES = [
    CONTRACTS_SCHEMA,
    CONTRACTS_MANIFEST,
    PHASE9_FIXTURE,
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
    missing_contracts = PHASE9_CONTRACTS - actual_contracts
    if missing_contracts:
        raise AssertionError(f"CLI schema is missing Phase 9 contracts: {sorted(missing_contracts)}")
    missing_defs = PHASE9_CONTRACTS - set(defs)
    if missing_defs:
        raise AssertionError(f"CLI schema is missing Phase 9 $defs: {sorted(missing_defs)}")

    if set(defs["product_kind"]["enum"]) != PRODUCTS:
        raise AssertionError("product_kind enum drifted")
    if set(defs["ci_credential_reference_kind"]["enum"]) != CI_CREDENTIAL_REF_KINDS:
        raise AssertionError("ci_credential_reference_kind enum drifted")

    product = defs["product_workflow_recipe"]["properties"]
    for key in [
        "sdk_overgate_only",
        "authorized_refs_only",
        "secret_free_json_output",
        "dynamic_model_resource_selection",
    ]:
        if product[key].get("const") is not True:
            raise AssertionError(f"product_workflow_recipe must keep {key}=true")
    for key in [
        "direct_internal_api_access",
        "direct_storage_access",
        "raw_http_required",
        "hardcoded_model_or_provider",
        "hardcoded_node_assumption",
        "paid_service_assumption",
    ]:
        if product[key].get("const") is not False:
            raise AssertionError(f"product_workflow_recipe must keep {key}=false")
    if product["submitted_via"].get("const") != "sdk_overgate_contract":
        raise AssertionError("product workflows must route through SDK/Overgate")

    ci = defs["ci_automation_profile"]["properties"]
    if ci["profile_kind"].get("const") != "ci":
        raise AssertionError("CI automation profile must use profile_kind=ci")
    if ci["environment_class"].get("const") != "ci":
        raise AssertionError("CI automation profile must use environment_class=ci")
    for key in [
        "short_lived_service_account_required",
        "requires_non_interactive_confirmation",
        "json_output_stable",
        "secret_free_output",
        "branch_on_exit_class",
    ]:
        if ci[key].get("const") is not True:
            raise AssertionError(f"ci_automation_profile must keep {key}=true")
    if ci["ambient_persistent_keychain_allowed"].get("const") is not False:
        raise AssertionError("CI automation must forbid ambient persistent keychain defaults")
    if ci["submitted_via"].get("const") != "sdk_overgate_contract":
        raise AssertionError("CI automation must route through SDK/Overgate")

    if set(manifest.get("phase9_contracts", [])) != PHASE9_CONTRACTS:
        raise AssertionError("Manifest phase9_contracts does not match schema extensions")
    fixture_paths = manifest.get("fixtures", {}).get("phase9_valid", [])
    if str(PHASE9_FIXTURE) not in fixture_paths:
        raise AssertionError("Manifest is missing Phase 9 fixture path")
    guardrails = "\n".join(manifest.get("guardrails", []))
    for expected in [
        "Phase 9 product integration workflows",
        "Phase 9 Mcoda product workflows",
        "Phase 9 Codali product workflows",
        "Phase 9 CI automation profiles",
    ]:
        if expected not in guardrails:
            raise AssertionError(f"Manifest guardrails missing Phase 9 text: {expected}")


def validate_fixture_and_docs() -> None:
    fixture = read_json(PHASE9_FIXTURE)
    if fixture["phase"] != "cli_phase_09":
        raise AssertionError("Phase 9 fixture has wrong phase marker")
    if set(fixture["products"]) != PRODUCTS:
        raise AssertionError("Phase 9 fixture product set drifted")
    if set(fixture["expected_failure_modes"]) != EXPECTED_FAILURE_MODES:
        raise AssertionError("Phase 9 fixture expected failure modes drifted")

    for product, expected_kind in PRODUCT_WORKLOAD_KINDS.items():
        entry = fixture["products"][product]
        if entry["workload_kind"] != expected_kind:
            raise AssertionError(f"{product} fixture workload kind drifted")
        if set(entry["required_refs"]) != PRODUCT_REQUIRED_REFS[product]:
            raise AssertionError(f"{product} fixture required refs drifted")
        if set(entry["required_commands"]) != {
            command.replace("overrid ", "") for command in PRODUCT_REQUIRED_COMMANDS[product]
        }:
            raise AssertionError(f"{product} fixture required commands drifted")

    ci_profile = fixture["ci_profile"]
    if ci_profile["profile_kind"] != "ci" or ci_profile["environment_class"] != "ci":
        raise AssertionError("Phase 9 fixture CI profile must be explicit ci")
    if set(ci_profile["allowed_credential_ref_kinds"]) != CI_CREDENTIAL_REF_KINDS:
        raise AssertionError("Phase 9 fixture CI credential kinds drifted")

    guardrails = fixture["guardrails"]
    for key in [
        "submitted_via",
        "sdk_overgate_only",
        "authorized_refs_only",
        "secret_free_json_output",
        "dynamic_model_resource_selection",
        "direct_internal_api_access",
        "direct_storage_access",
        "raw_http_required",
        "hardcoded_model_or_provider",
        "hardcoded_node_assumption",
        "paid_service_assumption",
        "ambient_persistent_keychain_allowed",
        "requires_non_interactive_confirmation",
        "json_output_stable",
        "branch_on_exit_class",
    ]:
        if key not in guardrails:
            raise AssertionError(f"Phase 9 fixture guardrail missing {key}")
    if guardrails["submitted_via"] != "sdk_overgate_contract":
        raise AssertionError("Phase 9 fixture must use SDK/Overgate")
    for key in [
        "sdk_overgate_only",
        "authorized_refs_only",
        "secret_free_json_output",
        "dynamic_model_resource_selection",
        "requires_non_interactive_confirmation",
        "json_output_stable",
        "branch_on_exit_class",
    ]:
        if guardrails[key] is not True:
            raise AssertionError(f"Phase 9 fixture guardrail must keep {key}=true")
    for key in [
        "direct_internal_api_access",
        "direct_storage_access",
        "raw_http_required",
        "hardcoded_model_or_provider",
        "hardcoded_node_assumption",
        "paid_service_assumption",
        "ambient_persistent_keychain_allowed",
    ]:
        if guardrails[key] is not False:
            raise AssertionError(f"Phase 9 fixture guardrail must keep {key}=false")

    for path in [
        SUB_PLAN,
        SERVICE,
        TECH_STACK,
        PHASE_PLAN,
        PHASE_PROGRESS,
        CLI / "README.md",
        CONTRACTS / "README.md",
    ]:
        text = read(path)
        assert_no_forbidden_assumptions(text, str(path))

    sub_plan = read(SUB_PLAN)
    for expected in ["Docdex", "Mcoda", "Codali", "ci_automation_profile", "product_workflow_recipe"]:
        assert_contains(sub_plan, expected, SUB_PLAN)
    plan = read(PHASE_PLAN)
    for expected in ["Docdex", "Mcoda", "Codali", "CI automation", "scripts/validate_cli_phase9.py"]:
        assert_contains(plan, expected, PHASE_PLAN)
    service = read(SERVICE)
    for expected in ["Product workflow gate", "CI automation gate", "scripts/validate_cli_phase9.py"]:
        assert_contains(service, expected, SERVICE)
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, "scripts/validate_cli_phase9.py", VALIDATION_WRAPPER)


def validate_recipe(product: str, recipe: dict[str, Any]) -> None:
    if recipe["product"] != product:
        raise AssertionError(f"expected product {product}, got {recipe['product']}")
    if recipe["workload_kind"] != PRODUCT_WORKLOAD_KINDS[product]:
        raise AssertionError(f"{product} workload kind drifted")
    if set(recipe["required_refs"]) != PRODUCT_REQUIRED_REFS[product]:
        raise AssertionError(f"{product} recipe required refs drifted")
    command_blob = "\n".join(recipe["command_recipes"])
    for command in PRODUCT_REQUIRED_COMMANDS[product]:
        if command not in command_blob:
            raise AssertionError(f"{product} recipe is missing command {command}")
    if not set(recipe["expected_failure_modes"]):
        raise AssertionError(f"{product} recipe must expose expected failure modes")
    if not set(recipe["safe_retry_patterns"]):
        raise AssertionError(f"{product} recipe must expose safe retry patterns")
    for key in [
        "sdk_overgate_only",
        "authorized_refs_only",
        "secret_free_json_output",
        "dynamic_model_resource_selection",
    ]:
        if recipe[key] is not True:
            raise AssertionError(f"{product} recipe must keep {key}=true")
    for key in [
        "direct_internal_api_access",
        "direct_storage_access",
        "raw_http_required",
        "hardcoded_model_or_provider",
        "hardcoded_node_assumption",
        "paid_service_assumption",
    ]:
        if recipe[key] is not False:
            raise AssertionError(f"{product} recipe must keep {key}=false")
    if recipe["submitted_via"] != "sdk_overgate_contract":
        raise AssertionError(f"{product} recipe must route through SDK/Overgate")


def validate_cli_outputs() -> None:
    docdex = run_cli(
        [
            "workload",
            "submit",
            "--json",
            "--workload-kind",
            "docdex_encrypted_index",
            "--workload-ref",
            "workload_docdex_index",
            "--target-ref",
            "docdex_index",
            *LOCAL_PROFILE_ARGS,
        ]
    )
    validate_recipe("docdex", docdex["result"]["product_workflow_recipe"])
    if "ci_automation_profile" in docdex["result"]:
        raise AssertionError("local product workflow must not render ci_automation_profile")

    mcoda = run_cli(
        [
            "workload",
            "status",
            "--json",
            "--workload-kind",
            "mcoda_agent_workload",
            "--workload-ref",
            "workload_mcoda_agent",
            "--target-ref",
            "mcoda_model_metadata",
            *LOCAL_PROFILE_ARGS,
        ]
    )
    validate_recipe("mcoda", mcoda["result"]["product_workflow_recipe"])
    if "execution_timeline" not in mcoda["result"]:
        raise AssertionError("mcoda status must include traceable execution timeline context")

    codali = run_cli(
        [
            "workload",
            "result",
            "--json",
            "--workload-kind",
            "codali_code_agent_package",
            "--workload-ref",
            "workload_codali_package",
            "--target-ref",
            "codali_repo_context",
            *LOCAL_PROFILE_ARGS,
        ]
    )
    validate_recipe("codali", codali["result"]["product_workflow_recipe"])
    if codali["result"]["execution_result"] is None:
        raise AssertionError("codali result must include authorized execution_result refs")
    if "policy.resource_denied" not in codali["result"]["product_workflow_recipe"]["expected_failure_modes"]:
        raise AssertionError("codali recipe must expose policy/resource failure reason")

    ci = run_cli(
        [
            "workload",
            "submit",
            "--json",
            "--workload-kind",
            "docdex_encrypted_index",
            "--workload-ref",
            "workload_docdex_ci",
            "--target-ref",
            "docdex_ci_index",
            *CI_PROFILE_ARGS,
        ]
    )
    validate_recipe("docdex", ci["result"]["product_workflow_recipe"])
    ci_profile = ci["result"]["ci_automation_profile"]
    if ci_profile["profile_kind"] != "ci" or ci_profile["environment_class"] != "ci":
        raise AssertionError("CI output must render explicit ci profile kind/environment")
    if ci_profile["credential_reference_class"] != "ci_reference":
        raise AssertionError("CI output must use explicit ci_reference credential class")
    if set(ci_profile["allowed_credential_ref_kinds"]) != CI_CREDENTIAL_REF_KINDS:
        raise AssertionError("CI output allowed credential refs drifted")
    for key in [
        "short_lived_service_account_required",
        "requires_non_interactive_confirmation",
        "json_output_stable",
        "secret_free_output",
        "branch_on_exit_class",
    ]:
        if ci_profile[key] is not True:
            raise AssertionError(f"CI output must keep {key}=true")
    if ci_profile["ambient_persistent_keychain_allowed"] is not False:
        raise AssertionError("CI output must forbid ambient persistent keychain defaults")


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
        print(f"validate_cli_phase9.py failed: {exc}", file=sys.stderr)
        return 1
    print("validate_cli_phase9.py passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
