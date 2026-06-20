#!/usr/bin/env python3
"""Validate the CLI Phase 2 Rust crate skeleton and guardrails."""

from __future__ import annotations

from pathlib import Path
import json
import subprocess
import sys


REPO_ROOT = Path(__file__).resolve().parents[1]

ROOT_CARGO = Path("Cargo.toml")
CONTRACTS = Path("packages/schemas/overrid_contracts")
SDK = Path("packages/sdk")
CLI = Path("packages/cli")
CONTRACTS_SCHEMA = CONTRACTS / "v0/cli_command.schema.json"
CONTRACTS_MANIFEST = CONTRACTS / "codegen_manifest.json"
CONTRACTS_README = CONTRACTS / "README.md"
SUB_PLAN = Path("docs/build_plan/sub_build_plan_002_cli.md")
SERVICE = Path("docs/service_catalog/foundation/cli.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/cli_phase_02_plan.md")
PHASE_PROGRESS = Path("docs/planning/cli_phase_02_progress.md")

REQUIRED_FILES = [
    ROOT_CARGO,
    CONTRACTS_README,
    CONTRACTS_MANIFEST,
    CONTRACTS_SCHEMA,
    CONTRACTS / "Cargo.toml",
    CONTRACTS / "src/lib.rs",
    SDK / "Cargo.toml",
    SDK / "src/lib.rs",
    CLI / "Cargo.toml",
    CLI / "README.md",
    CLI / "src/build_metadata.rs",
    CLI / "src/lib.rs",
    CLI / "src/main.rs",
    CLI / "src/parser.rs",
    CLI / "src/runner.rs",
    CLI / "fixtures/valid/version_output.valid.json",
    CLI / "fixtures/invalid/private_service_endpoint.invalid.json",
    PHASE_PLAN,
    PHASE_PROGRESS,
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} is missing expected text: {expected}")


def assert_not_contains(text: str, rejected: str, source: Path) -> None:
    if rejected in text:
        raise AssertionError(f"{source} contains rejected text: {rejected}")


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=REPO_ROOT)
    if result.returncode != 0:
        joined = " ".join(command)
        raise AssertionError(f"Command failed with exit {result.returncode}: {joined}")


def run_capture(command: list[str]) -> str:
    result = subprocess.run(command, cwd=REPO_ROOT, text=True, capture_output=True)
    if result.returncode != 0:
        joined = " ".join(command)
        raise AssertionError(
            f"Command failed with exit {result.returncode}: {joined}\n{result.stderr}"
        )
    return result.stdout


def validate_required_files() -> None:
    for path in REQUIRED_FILES:
        if not (REPO_ROOT / path).is_file():
            raise AssertionError(f"Missing required file: {path}")


def validate_workspace() -> None:
    root = read(ROOT_CARGO)
    for member in [
        '"packages/schemas/overrid_contracts"',
        '"packages/sdk"',
        '"packages/cli"',
        "resolver = \"2\"",
        "unsafe_code = \"forbid\"",
    ]:
        assert_contains(root, member, ROOT_CARGO)

    contracts_manifest = read(CONTRACTS / "Cargo.toml")
    assert_contains(contracts_manifest, "name = \"overrid-contracts\"", CONTRACTS / "Cargo.toml")

    sdk_manifest = read(SDK / "Cargo.toml")
    assert_contains(sdk_manifest, "name = \"overrid-sdk\"", SDK / "Cargo.toml")
    assert_contains(sdk_manifest, "overrid-contracts", SDK / "Cargo.toml")

    cli_manifest = read(CLI / "Cargo.toml")
    assert_contains(cli_manifest, "name = \"overrid-cli\"", CLI / "Cargo.toml")
    assert_contains(cli_manifest, "name = \"overrid\"", CLI / "Cargo.toml")
    assert_contains(cli_manifest, "overrid-contracts", CLI / "Cargo.toml")
    assert_contains(cli_manifest, "overrid-sdk", CLI / "Cargo.toml")


def validate_contracts() -> None:
    readme = read(CONTRACTS_README)
    for expected in [
        "JSON Schema files under `v0/` are the canonical docs-facing source",
        "src/",
        "generated/projection layer",
    ]:
        assert_contains(readme, expected, CONTRACTS_README)

    manifest = json.loads(read(CONTRACTS_MANIFEST))
    expected_manifest_fields = {
        "schema_family": "cli-command",
        "schema_version": "cli-command.v0.1",
        "canonical_schema": "packages/schemas/overrid_contracts/v0/cli_command.schema.json",
        "source_of_truth": "json_schema",
    }
    for key, expected in expected_manifest_fields.items():
        if manifest.get(key) != expected:
            raise AssertionError(f"{CONTRACTS_MANIFEST} has wrong {key}: {manifest.get(key)}")
    if manifest["rust_projection"]["non_authoritative"] is not True:
        raise AssertionError("Rust projection must be marked non-authoritative")

    schema = json.loads(read(CONTRACTS_SCHEMA))
    if schema["properties"]["schema_family"]["const"] != "cli-command":
        raise AssertionError("CLI schema must use cli-command schema family")
    if "output_envelope" not in schema["$defs"]:
        raise AssertionError("CLI schema must define output_envelope")
    expected_phase2_contracts = {
        "output_envelope",
        "api_error",
        "trace_context",
        "idempotency_record",
        "diagnostic_bundle",
        "overgate_request_metadata",
    }
    expected_phase3_extensions = {
        "environment_class",
        "cli_profile",
        "credential_reference",
        "confirmation_policy",
        "profile_storage_policy",
        "signer_handoff",
    }
    expected_phase4_extensions = {
        "command_lifecycle",
        "exit_code_registry",
        "capability_snapshot",
        "command_context",
    }
    expected_phase5_extensions = {
        "phase1_bootstrap_command",
        "signed_command_envelope",
        "bootstrap_acceptance_record",
        "manifest_bootstrap_ref",
        "synthetic_workload_pending_state",
    }
    actual_contracts = set(schema["properties"]["contracts"]["items"]["enum"])
    missing_phase2_contracts = expected_phase2_contracts - actual_contracts
    if missing_phase2_contracts:
        raise AssertionError(
            f"CLI schema is missing Phase 2 contracts: {sorted(missing_phase2_contracts)}"
        )
    allowed_contracts = (
        expected_phase2_contracts
        | expected_phase3_extensions
        | expected_phase4_extensions
        | expected_phase5_extensions
    )
    unexpected_contracts = actual_contracts - allowed_contracts
    if unexpected_contracts:
        raise AssertionError(
            f"CLI schema contains unexpected contracts: {sorted(unexpected_contracts)}"
        )

    contracts = read(CONTRACTS / "src/lib.rs")
    for expected in [
        "SUPPORTED_SCHEMA_VERSION",
        "cli-command.v0.1",
        "CONTRACT_SOURCE_ROOT",
        "packages/schemas",
        "rust_projection_from_json_schema_source",
        "GENERATED_CONTRACT_STATUS",
        "ensure_supported_schema_version",
        "UnsupportedSchemaVersion",
        "TraceContext",
        "IdempotencyRecord",
        "OutputEnvelope",
        "DiagnosticBundle",
    ]:
        assert_contains(contracts, expected, CONTRACTS / "src/lib.rs")


def validate_sdk() -> None:
    sdk = read(SDK / "src/lib.rs")
    for expected in [
        "OvergateEndpoint",
        "validate_overgate_target",
        "PRIVATE_SERVICE_TARGETS",
        "overqueue",
        "overvault",
        "node-agent",
        "MissingOvergateTarget",
        "PrivateServiceTarget",
        "OverridSdkClient",
        "build_request",
        "x-overrid-target",
        "overgate",
    ]:
        assert_contains(sdk, expected, SDK / "src/lib.rs")


def validate_cli_runtime_boundary() -> None:
    readme = read(CLI / "README.md")
    assert_contains(readme, "Rust CLI runtime", CLI / "README.md")
    assert_contains(readme, "TypeScript", CLI / "README.md")
    assert_contains(readme, "not this CLI's runtime", CLI / "README.md")

    cli_paths = [path for path in (REPO_ROOT / CLI).rglob("*") if path.is_file()]
    for path in cli_paths:
        rel = path.relative_to(REPO_ROOT)
        if rel.name == "package.json" or rel.suffix in {".ts", ".tsx", ".js", ".jsx"}:
            raise AssertionError(f"CLI runtime contains non-Rust runtime artifact: {rel}")

    parser = read(CLI / "src/parser.rs")
    for expected in [
        "--json",
        "--no-color",
        "--verbose",
        "--profile",
        "--all-phases",
        "--output",
        "ConflictingOutputMode",
        "set_output_mode",
        "PlannedCommand::Node",
    ]:
        assert_contains(parser, expected, CLI / "src/parser.rs")

    runner = read(CLI / "src/runner.rs")
    for expected in [
        "not_available_in_phase",
        "EXIT_NOT_AVAILABLE_IN_PHASE",
        "render_version_json",
        "sdk_target",
        "overgate_only",
        "node register|inspect|health",
        "governance|incident|compliance|migration",
    ]:
        assert_contains(runner, expected, CLI / "src/runner.rs")


def validate_fixtures() -> None:
    valid = json.loads(read(CLI / "fixtures/valid/version_output.valid.json"))
    if valid["schema_version"] != "cli-command.v0.1":
        raise AssertionError("valid version fixture must use cli-command.v0.1")
    if valid["result"]["sdk_target"] != "overgate_only":
        raise AssertionError("valid version fixture must preserve Overgate-only SDK target")
    if valid["result"]["contract_source_root"] != "packages/schemas":
        raise AssertionError("valid version fixture must cite packages/schemas")

    invalid = json.loads(read(CLI / "fixtures/invalid/private_service_endpoint.invalid.json"))
    if invalid["overgate_only"] is not False:
        raise AssertionError("invalid fixture must explicitly fail Overgate-only validation")
    if "overqueue" not in invalid["endpoint"]:
        raise AssertionError("invalid fixture must exercise a private service endpoint")

    emitted = json.loads(
        run_capture(["cargo", "run", "-p", "overrid-cli", "--", "version", "--json"])
    )
    if emitted != valid:
        raise AssertionError("overrid version --json output must match valid fixture")


def validate_docs() -> None:
    sub_plan = read(SUB_PLAN)
    service = read(SERVICE)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)

    for expected in [
        "## Phase 2: Rust CLI Crate, Generated Contracts, And SDK Integration",
        "**2.1 Create the Rust CLI crate and package boundary.**",
        "**2.2 Integrate generated contracts.**",
        "**2.3 Integrate SDK transport middleware.**",
        "**2.4 Define command parser conventions.**",
        "**2.5 Define fixture strategy.**",
        "### Phase 2 Gate Outputs",
        "packages/cli",
        "packages/sdk",
        "packages/schemas/overrid_contracts",
        "scripts/validate_cli_phase2.py",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    for expected in [
        "Build the Phase 2 Rust CLI crate skeleton",
        "## Phase 2 Implementation Gates",
        "packages/cli",
        "packages/sdk",
        "packages/schemas/overrid_contracts",
    ]:
        assert_contains(service, expected, SERVICE)

    assert_contains(tech_stack, "| CLI | Rust CLI using generated contracts", TECH_STACK)
    assert_contains(phase_plan, "Complete the Phase 2 implementation slice", PHASE_PLAN)
    assert_contains(progress, "# CLI Phase 2 Progress", PHASE_PROGRESS)


def validate_no_forbidden_runtime_terms() -> None:
    scoped_text = "\n".join(
        read(path)
        for path in [
            ROOT_CARGO,
            CONTRACTS / "src/lib.rs",
            SDK / "src/lib.rs",
            CLI / "src/build_metadata.rs",
            CLI / "src/lib.rs",
            CLI / "src/main.rs",
            CLI / "src/parser.rs",
            CLI / "src/runner.rs",
        ]
    )
    for rejected in [
        "postgres",
        "redis",
        "kafka",
        "rabbitmq",
        "minio",
        "s3://",
        "vault token",
        "blockchain",
        "nft",
    ]:
        assert_not_contains(scoped_text.lower(), rejected, Path("CLI Phase 2 scoped files"))


def validate_cargo() -> None:
    run(["cargo", "check", "-p", "overrid-cli"])
    run(["cargo", "test", "-p", "overrid-contracts", "-p", "overrid-sdk", "-p", "overrid-cli"])


def main() -> int:
    checks = [
        validate_required_files,
        validate_workspace,
        validate_contracts,
        validate_sdk,
        validate_cli_runtime_boundary,
        validate_fixtures,
        validate_docs,
        validate_no_forbidden_runtime_terms,
        validate_cargo,
    ]
    for check in checks:
        try:
            check()
        except AssertionError as exc:
            print(f"{check.__name__} failed: {exc}", file=sys.stderr)
            return 1
    print("CLI Phase 2 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
