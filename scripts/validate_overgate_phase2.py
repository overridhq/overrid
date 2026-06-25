#!/usr/bin/env python3
"""Validate Overgate Phase 2 service skeleton, route surface, and fixtures."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

WORKSPACE = Path("Cargo.toml")
WORKSPACE_LAYOUT = Path("overrid.workspace.toml")
CRATE_TOML = Path("packages/overgate/Cargo.toml")
README = Path("packages/overgate/README.md")
LIB = Path("packages/overgate/src/lib.rs")
SERVICE = Path("packages/overgate/src/service.rs")
ROUTES = Path("packages/overgate/src/routes.rs")
ADMIN = Path("packages/overgate/src/admin.rs")
DEPENDENCIES = Path("packages/overgate/src/dependencies.rs")
MAIN = Path("packages/overgate/src/main.rs")
LOCAL_STACK = Path("packages/local_stack/src/lib.rs")
HARNESS_LOCAL_STACK = Path("packages/integration_harness/src/local_stack.rs")
VALID_FIXTURE = Path("packages/overgate/fixtures/valid/phase2_local_command.valid.json")
INVALID_FIXTURE = Path("packages/overgate/fixtures/invalid/admin_unsigned.invalid.json")
LOCAL_STACK_DEFAULT_FIXTURE = Path(
    "packages/schemas/overrid_contracts/fixtures/valid/"
    "local_development_stack_phase2_default_local.valid.json"
)
SUB_PLAN = Path("docs/build_plan/sub_build_plan_008_overgate.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overgate_phase_02_plan.md")
PHASE_PROGRESS = Path("docs/planning/overgate_phase_02_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

REQUIRED_PUBLIC_ROUTES = [
    "POST /v1/commands",
    "GET /v1/commands/{command_id}",
    "GET /v1/traces/{trace_id}",
    "GET /v1/limits",
    "POST /v1/policy/dry-run",
    "GET /v1/healthz",
    "GET /v1/readyz",
]

REQUIRED_ADMIN_ROUTES = [
    "GET /v1/admin/ingress/{request_id}",
    "GET /v1/admin/idempotency/{tenant_id}/{idempotency_key}",
    "POST /v1/admin/idempotency/{record_id}/expire",
    "GET /v1/admin/rate-limits",
]

REQUIRED_DEPENDENCIES = [
    "schema_validation",
    "overkey_lite",
    "overpass",
    "overtenant",
    "overwatch",
    "overqueue",
    "forwarding_targets",
    "later_overguard",
    "later_overmeter",
    "later_oru",
]

RAW_SECRET_MARKERS = (
    "password=",
    "token=",
    "secret=",
    "api_key=",
    "private key",
    "-----begin",
    "raw_secret_value",
)


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def load_json(path: Path) -> Any:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required JSON fixture: {path}")
    with full_path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} is missing expected text: {expected}")


def validate_workspace_and_crate() -> None:
    workspace = read(WORKSPACE)
    workspace_layout = read(WORKSPACE_LAYOUT)
    crate = read(CRATE_TOML)
    main = read(MAIN)
    lib = read(LIB)

    assert_contains(workspace, '"packages/overgate"', WORKSPACE)
    for expected in (
        'name = "overgate"',
        'type = "rust_service_crate"',
        'owner_layer = "runtime_service_contracts"',
        'path = "packages/overgate"',
        'cargo_member = "packages/overgate"',
        'cargo_package = "overrid-overgate"',
        '"cargo test -p overrid-overgate"',
        '"python3 scripts/validate_overgate_phase2.py"',
        'local_stack_participation = "profile_consumer"',
    ):
        assert_contains(workspace_layout, expected, WORKSPACE_LAYOUT)
    assert_contains(crate, 'name = "overrid-overgate"', CRATE_TOML)
    for dependency in ("axum", "tokio", "serde", "serde_json", "tracing", "tracing-subscriber"):
        assert_contains(crate, dependency, CRATE_TOML)
    assert_contains(crate, 'name = "overgate"', CRATE_TOML)
    assert_contains(lib, "#![forbid(unsafe_code)]", LIB)
    for module in ("admin", "dependencies", "routes", "service"):
        assert_contains(lib, f"pub mod {module};", LIB)
    assert_contains(main, "OvergateConfig::from_env()", MAIN)
    assert_contains(main, "tokio::net::TcpListener::bind", MAIN)
    assert_contains(main, "axum::serve", MAIN)
    assert_contains(main, "tracing_subscriber::fmt()", MAIN)


def validate_routes_and_admin_guards() -> None:
    routes = read(ROUTES)
    admin = read(ADMIN)
    service = read(SERVICE)
    dependencies = read(DEPENDENCIES)

    for route in REQUIRED_PUBLIC_ROUTES:
        assert_contains(routes, route, ROUTES)
    for route in REQUIRED_ADMIN_ROUTES:
        assert_contains(admin, route, ADMIN)
    for axum_path in (
        '/v1/commands"',
        '/v1/commands/:command_id"',
        '/v1/traces/:trace_id"',
        '/v1/limits"',
        '/v1/policy/dry-run"',
        '/v1/healthz"',
        '/v1/readyz"',
        '/v1/admin/ingress/:request_id"',
        '/v1/admin/idempotency/:tenant_id/:idempotency_key"',
        '/v1/admin/idempotency/:record_id/expire"',
        '/v1/admin/rate-limits"',
    ):
        source = admin if "admin" in axum_path else routes
        assert_contains(source, axum_path, ADMIN if source is admin else ROUTES)

    assert_contains(routes, "schema_version: \"overgate.phase2.response.v0\"", ROUTES)
    assert_contains(routes, "StatusCode::ACCEPTED", ROUTES)
    assert_contains(routes, "StatusCode::SERVICE_UNAVAILABLE", ROUTES)
    assert_contains(routes, "x-overrid-trace-id", ROUTES)
    assert_contains(routes, "../fixtures/valid/phase2_local_command.valid.json", ROUTES)
    assert_contains(routes, 'uri("/overgate/v1/commands")', ROUTES)

    for guard in (
        "x-overrid-operator-signature",
        "x-overrid-operator-role",
        "auth.operator_signature_required",
        "auth.operator_role_required",
        "auth.cross_tenant_denied",
    ):
        assert_contains(admin, guard, ADMIN)

    for dependency in REQUIRED_DEPENDENCIES:
        assert_contains(dependencies, dependency, DEPENDENCIES)
    assert_contains(dependencies, "DependencyRequirement::Required", DEPENDENCIES)
    assert_contains(dependencies, "DependencyRequirement::OptionalFuture", DEPENDENCIES)
    assert_contains(service, 'public_base_path: "/overgate"', SERVICE)
    assert_contains(service, "nest(&base_path, route_tree())", SERVICE)


def validate_fixtures_and_docs() -> None:
    readme = read(README)
    sub_plan = read(SUB_PLAN)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    suite = read(SUITE_VALIDATOR)
    local_stack = read(LOCAL_STACK)
    harness_local_stack = read(HARNESS_LOCAL_STACK)
    valid = load_json(VALID_FIXTURE)
    invalid = load_json(INVALID_FIXTURE)
    local_stack_default = load_json(LOCAL_STACK_DEFAULT_FIXTURE)

    assert_contains(sub_plan, "## Phase 2: Rust Service Skeleton, Routes, And Dependency Readiness", SUB_PLAN)
    assert_contains(tech_stack, "Axum/Tower/Hyper-style Rust HTTP services", TECH_STACK)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #8 Phase 2", PHASE_PLAN)
    assert_contains(phase_progress, "Overgate Phase 2 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)
    assert_contains(suite, 'Path("scripts/validate_overgate_phase2.py")', SUITE_VALIDATOR)
    assert_contains(local_stack, 'service_id: "service:api"', LOCAL_STACK)
    assert_contains(local_stack, "port: 18080", LOCAL_STACK)
    assert_contains(harness_local_stack, '("service:overgate", true)', HARNESS_LOCAL_STACK)

    for route in REQUIRED_PUBLIC_ROUTES + REQUIRED_ADMIN_ROUTES:
        assert_contains(readme, route, README)
    assert_contains(readme, "cargo run -p overrid-overgate --bin overgate", README)
    assert_contains(readme, "service:overgate", README)
    assert_contains(readme, "/overgate", README)

    local_stack_service = valid["local_stack_service"]
    if local_stack_service["service_id"] != "service:overgate":
        raise AssertionError("valid fixture must reference service:overgate")
    if local_stack_service["port_owner_service_id"] != "service:api":
        raise AssertionError("valid fixture must preserve service:api as the port 18080 owner")
    if local_stack_service["bind_host"] != "127.0.0.1" or local_stack_service["port"] != 18080:
        raise AssertionError("valid fixture must keep Overgate on loopback port 18080")
    if local_stack_service["base_path"] != "/overgate":
        raise AssertionError("valid fixture must use /overgate base path")
    if not local_stack_service["local_only"] or not local_stack_service["test_only"]:
        raise AssertionError("valid fixture must be local/test scoped")
    if valid["command_envelope"]["payload_ref"].startswith("fixture://") is False:
        raise AssertionError("valid fixture must use payload refs instead of private payload bodies")
    if valid["harness_scenario_ref"] != "scenario:overgate_phase2_command_smoke":
        raise AssertionError("valid fixture must expose the phase 2 Overgate smoke scenario ref")
    for dependency in REQUIRED_DEPENDENCIES[:7]:
        if dependency not in valid["dependency_refs"]:
            raise AssertionError(f"valid fixture missing dependency ref: {dependency}")

    local_stack_services = local_stack_default["stack_profile"]["enabled_services"]
    if "service:api" not in local_stack_services:
        raise AssertionError("local-stack default fixture must preserve service:api as port owner")
    port_bindings = local_stack_default["port_registry"]["bindings"]
    if not any(binding["service_id"] == "service:api" and binding["port"] == 18080 for binding in port_bindings):
        raise AssertionError("local-stack default fixture must preserve service:api on port 18080")

    denial = invalid["expected_denial"]
    if denial["status"] != 401 or denial["reason_code"] != "auth.operator_signature_required":
        raise AssertionError("invalid fixture must prove unsigned admin denial")

    for path in (README, VALID_FIXTURE, INVALID_FIXTURE, PHASE_PLAN, PHASE_PROGRESS):
        text = read(path).lower()
        for marker in RAW_SECRET_MARKERS:
            if marker in text:
                raise AssertionError(f"{path} contains raw secret marker: {marker}")


def validate_tests_exist() -> None:
    routes = read(ROUTES)
    tests = re.findall(r"#\[tokio::test\]\s+async fn ([a-z0-9_]+)", routes)
    required_tests = {
        "public_routes_register_and_preserve_trace_json",
        "local_base_path_routes_to_same_public_surface",
        "local_fixture_command_smoke_submits_through_overgate_base_path",
        "readyz_separates_liveness_from_dependency_authority",
        "admin_routes_deny_unsigned_non_operator_and_cross_tenant_requests",
    }
    missing = sorted(required_tests - set(tests))
    if missing:
        raise AssertionError(f"{ROUTES} missing expected route tests: {', '.join(missing)}")


def main() -> int:
    checks = [
        validate_workspace_and_crate,
        validate_routes_and_admin_guards,
        validate_fixtures_and_docs,
        validate_tests_exist,
    ]
    for check in checks:
        try:
            check()
        except AssertionError as exc:
            print(f"Overgate Phase 2 validation failed: {exc}", file=sys.stderr)
            return 1
    print("Overgate Phase 2 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
