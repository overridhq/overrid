#!/usr/bin/env python3
"""Validate Local Development Stack Phase 2 contract artifacts."""

from __future__ import annotations

import json
from pathlib import Path
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

SCHEMA = Path("packages/schemas/overrid_contracts/v0/local_development_stack.schema.json")
MANIFEST = Path("packages/schemas/overrid_contracts/codegen_manifest.json")
README = Path("packages/schemas/overrid_contracts/README.md")
RUST_PROJECTION = Path("packages/schemas/overrid_contracts/src/lib.rs")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_004_local_development_stack.md")
PHASE_PLAN = Path("docs/planning/local_development_stack_phase_02_plan.md")
PHASE_PROGRESS = Path("docs/planning/local_development_stack_phase_02_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

VALID_FIXTURES = [
    Path("packages/schemas/overrid_contracts/fixtures/valid/local_development_stack_phase2_default_local.valid.json"),
    Path("packages/schemas/overrid_contracts/fixtures/valid/local_development_stack_phase2_ci_smoke.valid.json"),
    Path("packages/schemas/overrid_contracts/fixtures/valid/local_development_stack_phase2_harness.valid.json"),
    Path("packages/schemas/overrid_contracts/fixtures/valid/local_development_stack_phase2_extended_profile.valid.json"),
]

INVALID_FIXTURES = {
    Path("packages/schemas/overrid_contracts/fixtures/invalid/local_stack_profile_missing_marker.invalid.json"): "safety.local_test_marker_missing",
    Path("packages/schemas/overrid_contracts/fixtures/invalid/local_stack_profile_non_loopback.invalid.json"): "profile.non_loopback_bind_host",
    Path("packages/schemas/overrid_contracts/fixtures/invalid/local_stack_profile_future_service_without_gate.invalid.json"): "profile.future_service_without_phase_gate",
    Path("packages/schemas/overrid_contracts/fixtures/invalid/local_stack_service_missing_health.invalid.json"): "service.health_check_missing",
    Path("packages/schemas/overrid_contracts/fixtures/invalid/local_stack_service_missing_dependency_order.invalid.json"): "service.dependency_order_missing",
    Path("packages/schemas/overrid_contracts/fixtures/invalid/local_stack_port_collision.invalid.json"): "port.duplicate",
    Path("packages/schemas/overrid_contracts/fixtures/invalid/local_stack_reset_unmarked_volume.invalid.json"): "reset.marker_missing",
    Path("packages/schemas/overrid_contracts/fixtures/invalid/local_stack_seed_non_test_fixture.invalid.json"): "seed.not_test_only",
    Path("packages/schemas/overrid_contracts/fixtures/invalid/local_stack_health_unknown_state.invalid.json"): "health.invalid_state",
    Path("packages/schemas/overrid_contracts/fixtures/invalid/local_stack_diagnostic_raw_secret.invalid.json"): "diagnostic.raw_secret",
}

SUPPORTED_SCHEMA = "local-development-stack.v0.1"
RESERVED_PORT_MIN = 18080
RESERVED_PORT_MAX = 18085
LOOPBACK_HOSTS = {"127.0.0.1", "localhost", "::1"}
ENVIRONMENT_CLASSES = {"local", "ci"}
PHASE_GATE_STATES = {
    "buildable_phase_0",
    "local_smoke_prerequisite",
    "owning_service_required",
    "planned_disabled",
    "not_local_stack_owned",
}
FOUNDATION_SERVICES = {
    "service:api",
    "service:worker",
    "service:embedded_state",
    "service:overqueue_jobs",
    "service:overstore_stub",
    "service:event_audit",
    "service:node_agent_simulator",
    "service:developer_ui",
}
SERVICE_KINDS = {
    "api",
    "worker",
    "embedded_state",
    "overqueue_job_table",
    "overstore_artifact_stub",
    "local_event_audit_query",
    "node_agent_simulator",
}
HEALTH_STATES = {
    "not_started",
    "starting",
    "ready",
    "degraded",
    "failed",
    "resetting",
    "seeding",
    "running_tests",
}
DIAGNOSTIC_EVENT_TYPES = {
    "local_stack.start_requested",
    "local_stack.service_starting",
    "local_stack.service_ready",
    "local_stack.reset_started",
    "local_stack.seed_started",
    "local_stack.seed_completed",
    "local_stack.smoke_started",
    "local_stack.smoke_completed",
    "local_stack.failed",
}
PATH_REF_PREFIXES = (
    "repo://",
    "local-state://",
    "log://",
    "artifact://",
    "env://",
    "fixture://",
    "secret://",
)
RAW_SECRET_MARKERS = (
    "password=",
    "token=",
    "secret=",
    "api_key=",
    "private key",
    "-----begin",
    "raw_secret",
)
CONTRACT_NAMES = [
    "stack_profile",
    "service_definition",
    "port_registry",
    "local_env_manifest",
    "volume_registry",
    "reset_plan",
    "seed_manifest",
    "health_snapshot",
    "local_secret_record",
    "local_diagnostic_event",
]


def load_json(path: Path) -> Any:
    with (REPO_ROOT / path).open("r", encoding="utf-8") as handle:
        return json.load(handle)


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


def has_raw_secret_marker(value: Any) -> bool:
    if isinstance(value, str):
        lowered = value.lower()
        return any(marker in lowered for marker in RAW_SECRET_MARKERS)
    if isinstance(value, list):
        return any(has_raw_secret_marker(item) for item in value)
    if isinstance(value, dict):
        return any(has_raw_secret_marker(item) for item in value.values())
    return False


def is_local_ref(value: str) -> bool:
    return value.startswith(PATH_REF_PREFIXES)


def is_loopback_endpoint(value: str) -> bool:
    return (
        value.startswith("http://127.0.0.1:")
        or value.startswith("http://localhost:")
        or value.startswith("http://[::1]:")
        or is_local_ref(value)
    )


def record_marker_errors(record: dict[str, Any], errors: list[str]) -> None:
    if record.get("local_only") is not True or record.get("test_only") is not True:
        errors.append("safety.local_test_marker_missing")


def check_required_files() -> None:
    for path in [
        SCHEMA,
        MANIFEST,
        README,
        RUST_PROJECTION,
        SUB_PLAN,
        PHASE_PLAN,
        PHASE_PROGRESS,
        TECH_STACK,
        SUITE_VALIDATOR,
        *VALID_FIXTURES,
        *INVALID_FIXTURES.keys(),
    ]:
        assert_true((REPO_ROOT / path).is_file(), f"missing required file: {path}")


def check_schema_surface(schema: dict[str, Any]) -> None:
    assert_true(
        schema.get("$schema") == "https://json-schema.org/draft/2020-12/schema",
        "local stack schema must use JSON Schema draft 2020-12",
    )
    assert_true(
        schema.get("$id", "").endswith("/local_development_stack.schema.json"),
        "local stack schema id drifted",
    )
    assert_true(schema.get("additionalProperties") is False, "root schema must reject unknown fields")
    defs = schema.get("$defs", {})
    props = schema.get("properties", {})

    for name in ["schema_version", *CONTRACT_NAMES]:
        assert_true(name in defs, f"schema missing $defs.{name}")
    for name in [
        "stack_profile",
        "service_definition",
        "service_definitions",
        "port_registry",
        "local_env_manifest",
        "volume_registry",
        "reset_plan",
        "seed_manifest",
        "health_snapshot",
        "local_secret_record",
        "local_secret_records",
        "local_diagnostic_event",
        "local_diagnostic_events",
    ]:
        assert_true(name in props, f"schema missing root property {name}")

    assert_true(defs["schema_version"].get("const") == SUPPORTED_SCHEMA, "wrong schema version")
    assert_true(set(defs["environment_class"].get("enum", [])) == ENVIRONMENT_CLASSES, "environment classes drifted")
    assert_true(set(defs["phase_gate_state"].get("enum", [])) == PHASE_GATE_STATES, "phase gate states drifted")
    assert_true(set(defs["service_kind"].get("enum", [])) == SERVICE_KINDS, "service kinds drifted")
    assert_true(set(defs["health_state"].get("enum", [])) == HEALTH_STATES, "health states drifted")

    stack_profile = defs["stack_profile"]
    for field in [
        "profile_id",
        "environment_class",
        "enabled_services",
        "required_phase_gates",
        "service_dependencies",
        "fixture_set",
        "feature_flags",
        "port_policy",
        "volume_set",
        "local_only",
        "test_only",
        "default_bind_host",
        "future_services_require_phase_gate",
    ]:
        assert_true(field in stack_profile.get("required", []), f"stack_profile must require {field}")

    service_definition = defs["service_definition"]
    for field in [
        "command",
        "working_directory",
        "env_refs",
        "dependency_order",
        "depends_on",
        "health_check",
        "shutdown_behavior",
        "log_target",
        "restart_class",
        "local_only",
        "test_only",
    ]:
        assert_true(field in service_definition.get("required", []), f"service_definition must require {field}")

    port_binding = defs["port_registry"]["properties"]["bindings"]["items"]["properties"]
    assert_true(port_binding["port"].get("minimum") == RESERVED_PORT_MIN, "port minimum drifted")
    assert_true(port_binding["port"].get("maximum") == RESERVED_PORT_MAX, "port maximum drifted")
    assert_true(set(port_binding["bind_host"].get("enum", [])) == LOOPBACK_HOSTS, "port bind hosts drifted")
    assert_true(
        defs["port_registry"]["properties"]["collision_policy"].get("const") == "fail_before_startup",
        "port collision policy must fail before startup",
    )

    local_env = defs["local_env_manifest"]
    assert_true(
        local_env["properties"]["contains_raw_secret"].get("$ref") == "#/$defs/no_raw_secret",
        "env manifest must reject raw secrets",
    )
    assert_true(
        defs["reset_plan"]["properties"]["deterministic"].get("$ref") == "#/$defs/local_marker",
        "reset plan must be deterministic",
    )
    assert_true(
        defs["local_secret_record"]["properties"]["raw_secret_present"].get("$ref") == "#/$defs/no_raw_secret",
        "local secret records must reject raw secrets",
    )
    assert_true(
        defs["local_diagnostic_event"]["properties"]["contains_raw_secret"].get("$ref")
        == "#/$defs/no_raw_secret",
        "diagnostic events must reject raw secrets",
    )


def validate_stack_profile(profile: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if profile.get("environment_class") not in ENVIRONMENT_CLASSES:
        errors.append("profile.unsupported_environment")
    if not profile.get("enabled_services"):
        errors.append("profile.enabled_services_missing")
    gates = profile.get("required_phase_gates", [])
    if not gates:
        errors.append("profile.phase_gate_missing")
    for gate in gates:
        if gate not in PHASE_GATE_STATES:
            errors.append("profile.phase_gate_unknown")
    record_marker_errors(profile, errors)
    if profile.get("default_bind_host") not in LOOPBACK_HOSTS:
        errors.append("profile.non_loopback_bind_host")
    if profile.get("future_services_require_phase_gate") is not True:
        errors.append("profile.future_service_without_phase_gate")
    for service in profile.get("enabled_services", []):
        if service not in FOUNDATION_SERVICES and not (
            "owning_service_required" in gates or "planned_disabled" in gates
        ):
            errors.append("profile.future_service_without_phase_gate")
    return errors


def validate_service_definition(service: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if service.get("service_kind") not in SERVICE_KINDS:
        errors.append("service.kind_unknown")
    if not service.get("command"):
        errors.append("service.command_missing")
    if not service.get("working_directory"):
        errors.append("service.working_directory_missing")
    if not service.get("env_refs"):
        errors.append("service.env_refs_missing")
    if not service.get("dependency_order"):
        errors.append("service.dependency_order_missing")
    if "depends_on" not in service:
        errors.append("service.depends_on_missing")
    health = service.get("health_check")
    if not isinstance(health, dict):
        errors.append("service.health_check_missing")
    else:
        if health.get("expected_state") not in HEALTH_STATES:
            errors.append("health.invalid_state")
        if not is_loopback_endpoint(str(health.get("endpoint", ""))):
            errors.append("service.non_loopback_endpoint")
        timeout = health.get("timeout_ms")
        if not isinstance(timeout, int) or timeout <= 0 or timeout > 600000:
            errors.append("service.health_timeout_invalid")
    if not service.get("shutdown_behavior"):
        errors.append("service.shutdown_behavior_missing")
    if not service.get("log_target"):
        errors.append("service.log_target_missing")
    if not service.get("restart_class"):
        errors.append("service.restart_class_missing")
    record_marker_errors(service, errors)
    return errors


def validate_port_registry(registry: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if registry.get("collision_policy") != "fail_before_startup":
        errors.append("port.collision_policy_invalid")
    record_marker_errors(registry, errors)
    seen: set[int] = set()
    for binding in registry.get("bindings", []):
        port = binding.get("port")
        if not isinstance(port, int) or not RESERVED_PORT_MIN <= port <= RESERVED_PORT_MAX:
            errors.append("port.outside_reserved_range")
        elif port in seen:
            errors.append("port.duplicate")
        else:
            seen.add(port)
        if binding.get("bind_host") not in LOOPBACK_HOSTS:
            errors.append("port.non_loopback_bind_host")
    if not seen:
        errors.append("port.binding_missing")
    return errors


def validate_env_manifest(manifest: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    record_marker_errors(manifest, errors)
    if manifest.get("redaction_policy") != "secret_free_refs_only":
        errors.append("env.redaction_policy_invalid")
    if manifest.get("contains_raw_secret") is not False:
        errors.append("env.raw_secret")
    for variable in manifest.get("variables", []):
        name = variable.get("name", "")
        value_ref = variable.get("value_ref", "")
        if not isinstance(name, str) or not name.startswith("OVERRID_"):
            errors.append("env.variable_name_invalid")
        if not isinstance(value_ref, str) or not is_local_ref(value_ref):
            errors.append("env.value_ref_invalid")
    return errors


def validate_volume_registry(registry: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    record_marker_errors(registry, errors)
    volumes = registry.get("volumes", [])
    if not volumes:
        errors.append("volume.missing")
    for volume in volumes:
        if not is_local_ref(str(volume.get("path_ref", ""))):
            errors.append("volume.path_ref_invalid")
        if volume.get("resettable") is True and not str(volume.get("local_test_state_marker", "")).strip():
            errors.append("reset.marker_missing")
    return errors


def validate_reset_plan(plan: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    record_marker_errors(plan, errors)
    if plan.get("deterministic") is not True or plan.get("requires_local_profile") is not True:
        errors.append("reset.local_deterministic_marker_missing")
    operations = plan.get("operations", [])
    if not operations:
        errors.append("reset.operation_missing")
    for operation in operations:
        if operation.get("requires_marker") is not True or not str(operation.get("marker_ref", "")).strip():
            errors.append("reset.marker_missing")
        if not is_local_ref(str(operation.get("target_ref", ""))):
            errors.append("reset.target_ref_invalid")
    return errors


def validate_seed_manifest(manifest: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    record_marker_errors(manifest, errors)
    if manifest.get("local_only") is not True or manifest.get("test_only") is not True:
        errors.append("seed.not_test_only")
    for field in ["fixture_version", "deterministic_seed"]:
        if not str(manifest.get(field, "")).strip():
            errors.append("seed.required_field_missing")
    for field, prefix in [
        ("tenant_refs", "tenant:"),
        ("actor_refs", "actor:"),
        ("key_refs", "key:"),
        ("node_refs", "node:"),
        ("manifest_refs", "manifest:"),
        ("package_refs", "package:"),
        ("workload_refs", "workload:"),
        ("local_oru_account_refs", "oru:"),
        ("ledger_refs", "ledger:"),
        ("policy_context_refs", "policy:"),
    ]:
        refs = manifest.get(field, [])
        if field in {"tenant_refs", "actor_refs", "key_refs"} and not refs:
            errors.append("seed.required_field_missing")
        for ref in refs:
            if not isinstance(ref, str) or not ref.startswith(prefix):
                errors.append("seed.ref_prefix_invalid")
    return errors


def validate_health_snapshot(snapshot: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    record_marker_errors(snapshot, errors)
    if snapshot.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append("schema.version_mismatch")
    if snapshot.get("state") not in HEALTH_STATES:
        errors.append("health.invalid_state")
    if snapshot.get("redaction_summary") != "secret_free":
        errors.append("diagnostic.raw_secret")
    service_health = snapshot.get("service_health", [])
    if not service_health:
        errors.append("health.service_missing")
    for item in service_health:
        if item.get("state") not in HEALTH_STATES:
            errors.append("health.invalid_state")
        if has_raw_secret_marker(item.get("last_error")):
            errors.append("diagnostic.raw_secret")
    return errors


def validate_secret_record(record: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    record_marker_errors(record, errors)
    if record.get("raw_secret_present") is not False or has_raw_secret_marker(record.get("secret_ref")):
        errors.append("diagnostic.raw_secret")
    if not is_local_ref(str(record.get("secret_ref", ""))):
        errors.append("secret.ref_invalid")
    return errors


def validate_diagnostic_event(event: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    record_marker_errors(event, errors)
    if event.get("event_type") not in DIAGNOSTIC_EVENT_TYPES:
        errors.append("diagnostic.event_type_unknown")
    if event.get("redaction_summary") != "secret_free" or event.get("contains_raw_secret") is not False:
        errors.append("diagnostic.raw_secret")
    if has_raw_secret_marker(event):
        errors.append("diagnostic.raw_secret")
    return errors


def validate_document(document: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if document.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append("schema.version_mismatch")
    if "stack_profile" in document:
        errors.extend(validate_stack_profile(document["stack_profile"]))
    if "service_definition" in document:
        errors.extend(validate_service_definition(document["service_definition"]))
    for service in document.get("service_definitions", []):
        errors.extend(validate_service_definition(service))
    if "port_registry" in document:
        errors.extend(validate_port_registry(document["port_registry"]))
    if "local_env_manifest" in document:
        errors.extend(validate_env_manifest(document["local_env_manifest"]))
    if "volume_registry" in document:
        errors.extend(validate_volume_registry(document["volume_registry"]))
    if "reset_plan" in document:
        errors.extend(validate_reset_plan(document["reset_plan"]))
    if "seed_manifest" in document:
        errors.extend(validate_seed_manifest(document["seed_manifest"]))
    if "health_snapshot" in document:
        errors.extend(validate_health_snapshot(document["health_snapshot"]))
    if "local_secret_record" in document:
        errors.extend(validate_secret_record(document["local_secret_record"]))
    for record in document.get("local_secret_records", []):
        errors.extend(validate_secret_record(record))
    if "local_diagnostic_event" in document:
        errors.extend(validate_diagnostic_event(document["local_diagnostic_event"]))
    for event in document.get("local_diagnostic_events", []):
        errors.extend(validate_diagnostic_event(event))
    return errors


def check_fixtures() -> None:
    for path in VALID_FIXTURES:
        errors = validate_document(load_json(path))
        assert_true(not errors, f"{path} should be valid, got {sorted(set(errors))}")

    for path, expected_error in INVALID_FIXTURES.items():
        errors = validate_document(load_json(path))
        assert_true(
            expected_error in errors,
            f"{path} should fail with {expected_error}, got {sorted(set(errors))}",
        )


def check_manifest(manifest: dict[str, Any]) -> None:
    section = manifest.get("local_development_stack_phase2")
    assert_true(isinstance(section, dict), "codegen manifest missing local_development_stack_phase2")
    assert_true(section.get("schema_family") == "local-development-stack", "wrong local stack schema family")
    assert_true(section.get("schema_version") == SUPPORTED_SCHEMA, "wrong local stack schema version")
    assert_true(section.get("canonical_schema") == str(SCHEMA), "wrong local stack schema path")
    assert_true(section.get("source_of_truth") == "json_schema", "local stack source of truth must be JSON Schema")
    projection = section.get("rust_projection", {})
    assert_true(projection.get("path") == str(RUST_PROJECTION), "wrong Rust projection path")
    assert_true(projection.get("non_authoritative") is True, "Rust projection must be non-authoritative")
    assert_true(set(section.get("contracts", [])) == set(CONTRACT_NAMES), "local stack contract list drifted")

    listed_valid = {Path(path) for path in section.get("fixtures", {}).get("valid", [])}
    listed_invalid = {Path(path) for path in section.get("fixtures", {}).get("invalid", [])}
    assert_true(listed_valid == set(VALID_FIXTURES), "local stack valid fixture list drifted")
    assert_true(listed_invalid == set(INVALID_FIXTURES), "local stack invalid fixture list drifted")

    guardrails = "\n".join(section.get("guardrails", []))
    for expected in [
        "local/test-only",
        "18080-18085",
        "Future services",
        "Reset and seed",
        "Health and diagnostic",
    ]:
        assert_contains(guardrails, expected, MANIFEST)


def check_docs_and_projection() -> None:
    readme = read_text(README)
    sub_plan = read_text(SUB_PLAN)
    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    tech_stack = read_text(TECH_STACK)
    rust = read_text(RUST_PROJECTION)
    suite = read_text(SUITE_VALIDATOR)

    for expected in [
        "Local Development Stack Phase 2 contracts define",
        "`stack_profile`",
        "`service_definition`",
        "loopback-only",
        "phase-gated",
    ]:
        assert_contains(readme, expected, README)
    for expected in ["**2.1", "**2.2", "**2.3", "**2.4", "**2.5"]:
        assert_contains(sub_plan, expected, SUB_PLAN)
    for expected in [
        "Canonical schema plus valid and invalid examples",
        "reserved-port collisions fail before startup",
        "test-only fixture markers",
        "not-started, starting, ready, degraded, failed, resetting, seeding, and running-tests",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)
    for expected in [
        "packages/schemas/overrid_contracts/v0/local_development_stack.schema.json",
        "scripts/validate_local_development_stack_phase2.py",
        "cargo test -p overrid-contracts local_stack",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)
    assert_contains(phase_progress, "Local Development Stack Phase 2 Progress", PHASE_PROGRESS)
    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)
    assert_contains(tech_stack, "Overrid-shaped local stubs", TECH_STACK)
    assert_contains(
        suite,
        'Path("scripts/validate_local_development_stack_phase2.py")',
        SUITE_VALIDATOR,
    )
    for expected in [
        "LOCAL_DEVELOPMENT_STACK_SCHEMA_FAMILY",
        "SUPPORTED_LOCAL_DEVELOPMENT_STACK_SCHEMA_VERSION",
        "LocalStackProfileContract",
        "LocalStackServiceDefinitionContract",
        "LocalStackPortRegistry",
        "LocalStackResetPlan",
        "LocalStackSeedManifest",
        "LocalStackHealthSnapshot",
        "LocalSecretRecordContract",
        "LocalDiagnosticEventContract",
        "DuplicatePort",
        "FutureServiceMissingPhaseGate",
    ]:
        assert_contains(rust, expected, RUST_PROJECTION)


def main() -> int:
    try:
        check_required_files()
        check_schema_surface(load_json(SCHEMA))
        check_manifest(load_json(MANIFEST))
        check_fixtures()
        check_docs_and_projection()
    except AssertionError as error:
        print(f"Local Development Stack Phase 2 validation failed: {error}", file=sys.stderr)
        return 1

    print("Local Development Stack Phase 2 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
