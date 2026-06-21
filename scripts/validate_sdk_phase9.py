#!/usr/bin/env python3
"""Validate SDK Phase 9 TypeScript/web binding and product hardening gates."""

from __future__ import annotations

from pathlib import Path
import json
import re
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_006_sdk.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/sdk_phase_09_plan.md")
PHASE_PROGRESS = Path("docs/planning/sdk_phase_09_progress.md")
SDK_CARGO = Path("packages/sdk/Cargo.toml")
SDK_LIB = Path("packages/sdk/src/lib.rs")
SDK_BINDINGS = Path("packages/sdk/src/bindings.rs")
SDK_README = Path("packages/sdk/README.md")
TS_WEB_MANIFEST = Path("packages/sdk/bindings/phase9/typescript_web_binding_manifest.valid.json")
PRODUCT_MODULES = Path("packages/sdk/bindings/phase9/product_convenience_modules.valid.json")
READINESS_GATES = Path("packages/sdk/bindings/phase9/binding_readiness_gates.valid.json")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

REQUIRED_PHASE9_SYMBOLS = [
    "pub mod bindings;",
    "pub use bindings::*;",
    "SDK_PHASE9_CAPABILITY_PROFILE",
    "SDK_PHASE9_BINDING_ROOT",
    "SDK_PHASE9_TYPESCRIPT_WEB_MANIFEST_PATH",
    "SDK_PHASE9_PRODUCT_MODULES_PATH",
    "SDK_PHASE9_READINESS_GATES_PATH",
    "SdkPhase9TypeScriptWebArtifactKind",
    "SdkPhase9TypeScriptWebBindingDescriptor",
    "sdk_phase9_typescript_web_bindings",
    "validate_phase9_typescript_web_bindings",
    "SdkPhase9AdapterUiSafetyBoundary",
    "sdk_phase9_adapter_ui_safety_boundary",
    "validate_phase9_adapter_ui_safety_boundary",
    "SdkPhase9ProductKind",
    "SdkPhase9ProductConvenienceModule",
    "sdk_phase9_product_convenience_modules",
    "validate_phase9_product_convenience_modules",
    "SdkPhase9MobileExtensionKind",
    "SdkPhase9MobileExtensionInterface",
    "sdk_phase9_mobile_extension_interfaces",
    "validate_phase9_mobile_extension_interfaces",
    "SdkPhase9BindingTarget",
    "SdkPhase9BindingReadinessGate",
    "sdk_phase9_binding_readiness_gates",
    "validate_phase9_binding_readiness_gates",
    "validate_phase9_artifact_files",
    "SdkPhase9Error",
]

REQUIRED_PHASE9_TESTS = [
    "phase9_typescript_web_bindings_are_generated_from_shared_contracts",
    "phase9_adapter_ui_boundary_rejects_internal_authority",
    "phase9_product_modules_are_capability_gated_and_cover_failure_cases",
    "phase9_mobile_extensions_do_not_own_mobile_runtime_behavior",
    "phase9_binding_readiness_gates_block_drift_and_handwritten_objects",
    "phase9_artifact_files_match_declared_paths_and_markers",
]

REQUIRED_TS_WEB_KINDS = {
    "models",
    "validators",
    "error_objects",
    "request_helpers",
    "manifest_helpers",
    "idempotency_helpers",
}

REQUIRED_PRODUCTS = {
    "docdex_encrypted_rag",
    "mcoda_agent_workload",
    "codali_code_agent",
    "package_validation",
}

REQUIRED_FAILURE_CASES = {
    "successful_job",
    "retryable_failure",
    "final_failure",
    "cancellation",
    "timeout",
    "policy_denial",
    "budget_exhaustion",
    "node_disconnect",
    "disputed_usage",
}

REQUIRED_MOBILE_EXTENSIONS = {
    "session_refresh_primitive",
    "secure_storage_adapter_interface",
    "offline_queue_interface",
    "sync_cursor_primitive",
    "conflict_state_primitive",
    "redacted_diagnostics_hook",
}

REQUIRED_BINDING_TARGETS = {
    "typescript_web",
    "swift_ios",
    "kotlin_android",
    "python",
    "other",
}

FORBIDDEN_MARKERS = [
    "raw_private_key",
    "private_key_value",
    "bearer_token_value",
    "seed_phrase_value",
    '"signature_value"',
    '"raw_request_body"',
    "private_payload_value",
    "fixture_credential_material",
    '"runtime_authority": true',
    '"bypasses_internal_services": true',
    '"stores_private_keys": true',
    '"direct_storage_access": true',
    '"hardcoded_model_or_provider": true',
    '"paid_service_assumption": true',
    '"foundation_sdk_implements_runtime": true',
    '"provides_default_offline_queue": true',
    '"handwritten_public_objects_allowed": true',
    "postgresql",
    "redis",
    "s3://",
    "minio",
    "kafka",
    "nats",
    "blockchain",
    "nft",
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(read(path))


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=REPO_ROOT, text=True, capture_output=True)
    if result.returncode != 0:
        sys.stdout.write(result.stdout)
        sys.stderr.write(result.stderr)
        raise AssertionError(f"Command failed: {' '.join(command)}")


def section(text: str, heading: str, next_heading_level: str = "## ") -> str:
    marker = f"{heading}\n"
    start = text.find(marker)
    if start == -1:
        raise AssertionError(f"Missing heading: {heading}")
    body_start = start + len(marker)
    end = text.find(f"\n{next_heading_level}", body_start)
    if end == -1:
        end = len(text)
    return text[body_start:end]


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} is missing expected text: {expected}")


def assert_no_forbidden_markers(text: str, source: Path) -> None:
    lowered = text.lower()
    for marker in FORBIDDEN_MARKERS:
        if marker in lowered:
            raise AssertionError(f"{source} contains forbidden Phase 9 marker: {marker}")


def validate_sub_plan_phase9() -> None:
    text = read(SUB_PLAN)
    phase_9 = section(
        text,
        "## Phase 9: TypeScript/Web Bindings And Product Integration Hardening",
    )
    for item in range(1, 6):
        assert_contains(phase_9, f"**9.{item} ", SUB_PLAN)
    for work_item in re.finditer(
        r"- \*\*9\.[1-5] .+?(?=\n- \*\*9\.|\n## Phase 10:)",
        phase_9,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")
    for expected in [
        "TypeScript/web models",
        "adapter and UI safety boundaries",
        "Docdex encrypted RAG jobs",
        "Mcoda agent workloads",
        "Codali code-agent workloads",
        "Mobile SDK",
        "Binding-readiness checklist",
    ]:
        assert_contains(phase_9, expected, SUB_PLAN)


def validate_phase_docs() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in [
        "Complete the Phase 9 gate",
        "TypeScript/web binding projections",
        "packages/sdk/bindings/phase9",
        "adapter/UI safety descriptors",
        "product convenience module descriptors",
        "Phase 12 Mobile SDK",
        "Do not add new SDK dependencies",
        "cargo test -p overrid-sdk phase9_",
    ]:
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in [
        "Loaded Docdex profile memory and repo memory",
        "Docdex impact graph",
        "Local delegation was attempted",
        "Saved the repo-scoped Phase 9 session scope",
        "Validation Evidence",
    ]:
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_sdk_code_and_readme() -> None:
    cargo = read(SDK_CARGO)
    lib = read(SDK_LIB)
    bindings = read(SDK_BINDINGS)
    readme = read(SDK_README)
    combined = f"{lib}\n{bindings}"

    for expected in REQUIRED_PHASE9_SYMBOLS + REQUIRED_PHASE9_TESTS:
        assert_contains(combined, expected, SDK_BINDINGS)
    for expected in [
        'include_str!("../bindings/phase9/typescript_web_binding_manifest.valid.json")',
        'include_str!("../bindings/phase9/product_convenience_modules.valid.json")',
        'include_str!("../bindings/phase9/binding_readiness_gates.valid.json")',
        "SDK_PHASE9_CAPABILITY_PROFILE",
        "SDK_PHASE9_TYPESCRIPT_WEB_TARGET",
        "SDK_PHASE9_MOBILE_SDK_OWNER_PHASE",
        "bypasses_internal_services: false",
        "runtime_authority: false",
        "handwritten_public_objects_allowed: false",
    ]:
        assert_contains(bindings, expected, SDK_BINDINGS)
    if "serde" in cargo or "tokio" in cargo:
        raise AssertionError("SDK Phase 9 must not add new runtime dependencies")
    for expected in [
        "Phase 9 TypeScript/Web Bindings And Product Integration Hardening",
        "sdk_phase9_typescript_web_bindings()",
        "validate_phase9_product_convenience_modules()",
        "validate_phase9_mobile_extension_interfaces()",
        "validate_phase9_binding_readiness_gates()",
        "scripts/validate_sdk_phase9.py",
    ]:
        assert_contains(readme, expected, SDK_README)


def validate_ts_web_manifest() -> None:
    manifest = read_json(TS_WEB_MANIFEST)
    if manifest["artifact_id"] != "sdk_phase9_typescript_web_binding_manifest":
        raise AssertionError("wrong TypeScript/web manifest id")
    if manifest["language_target"] != "typescript_web":
        raise AssertionError("wrong TypeScript/web language target")
    for key in [
        "generated_from_shared_contracts",
        "passes_phase8_golden_corpus",
        "request_envelope_matches_rust",
        "error_objects_match_rust",
        "idempotency_matches_rust",
        "browser_surface_only",
    ]:
        if manifest[key] is not True:
            raise AssertionError(f"TypeScript/web manifest must keep {key}=true")
    for key in ["runtime_authority", "handwritten_public_objects_allowed"]:
        if manifest[key] is not False:
            raise AssertionError(f"TypeScript/web manifest must keep {key}=false")
    bindings = manifest["bindings"]
    if {entry["kind"] for entry in bindings} != REQUIRED_TS_WEB_KINDS:
        raise AssertionError("TypeScript/web binding kinds drifted")
    for entry in bindings:
        if not entry["output_path"].startswith("packages/sdk/bindings/typescript_web/generated/"):
            raise AssertionError(f"binding output path drifted: {entry['name']}")
        if entry["generated_from_shared_contracts"] is not True:
            raise AssertionError(f"binding is not generated from contracts: {entry['name']}")
        if entry["runtime_authority"] is not False:
            raise AssertionError(f"binding became runtime authority: {entry['name']}")
        if entry["handwritten_public_objects_allowed"] is not False:
            raise AssertionError(f"binding allows handwritten objects: {entry['name']}")


def validate_product_modules() -> None:
    manifest = read_json(PRODUCT_MODULES)
    safety = manifest["adapter_ui_safety"]
    for key in [
        "generated_client_required",
        "uses_overgate_admin_api_only",
        "redacted_diagnostics_required",
    ]:
        if safety[key] is not True:
            raise AssertionError(f"adapter/UI safety must keep {key}=true")
    for key in [
        "privileged_internal_endpoints_allowed",
        "private_key_storage_allowed",
        "hidden_service_shortcuts_allowed",
        "direct_worker_storage_api_allowed",
        "browser_runtime_authority",
    ]:
        if safety[key] is not False:
            raise AssertionError(f"adapter/UI safety must keep {key}=false")

    if set(manifest["required_failure_cases"]) != REQUIRED_FAILURE_CASES:
        raise AssertionError("required failure cases drifted")
    modules = manifest["modules"]
    if {entry["product_kind"] for entry in modules} != REQUIRED_PRODUCTS:
        raise AssertionError("Phase 9 product module set drifted")
    for entry in modules:
        if set(entry["failure_cases"]) != REQUIRED_FAILURE_CASES:
            raise AssertionError(f"failure case set drifted for {entry['module_name']}")
        if not entry["helpers"]:
            raise AssertionError(f"missing helpers for {entry['module_name']}")
        for helper in ["read_usage_rollup", "read_usage_receipt"]:
            if helper not in entry["helpers"]:
                raise AssertionError(
                    f"{entry['module_name']} must expose explicit {helper} helper"
                )
        if not all(
            route.startswith(("/v1/overgate/", "/v1/control-plane/", "/v1/accounting/"))
            for route in entry["public_routes"]
        ):
            raise AssertionError(f"unsafe public route for {entry['module_name']}")
        for key in [
            "capability_checks_required",
            "uses_overgate_admin_api_only",
            "authorized_refs_only",
            "service_returned_usage_receipts_only",
        ]:
            if entry[key] is not True:
                raise AssertionError(f"{entry['module_name']} must keep {key}=true")
        for key in [
            "bypasses_internal_services",
            "stores_private_keys",
            "direct_storage_access",
            "hardcoded_model_or_provider",
            "paid_service_assumption",
        ]:
            if entry[key] is not False:
                raise AssertionError(f"{entry['module_name']} must keep {key}=false")


def validate_readiness_gates() -> None:
    manifest = read_json(READINESS_GATES)
    mobile = manifest["mobile_extension_interfaces"]
    if {entry["kind"] for entry in mobile} != REQUIRED_MOBILE_EXTENSIONS:
        raise AssertionError("mobile extension kinds drifted")
    for entry in mobile:
        if entry["owner_phase"] != "phase12_mobile_sdk":
            raise AssertionError(f"mobile owner phase drifted for {entry['name']}")
        if entry["extension_point_only"] is not True:
            raise AssertionError(f"mobile extension must be extension-only: {entry['name']}")
        for key in [
            "foundation_sdk_implements_runtime",
            "provides_default_offline_queue",
            "owns_os_secure_storage",
            "owns_push_registration",
            "owns_background_behavior",
            "owns_media_upload_state",
        ]:
            if entry[key] is not False:
                raise AssertionError(f"{entry['name']} must keep {key}=false")

    gates = manifest["binding_readiness_gates"]
    if {entry["target"] for entry in gates} != REQUIRED_BINDING_TARGETS:
        raise AssertionError("binding readiness targets drifted")
    for entry in gates:
        for key in [
            "schema_compatibility_required",
            "golden_request_envelope_required",
            "generated_error_objects_required",
            "cross_language_contract_tests_required",
            "no_handwritten_public_objects_required",
            "release_blocking",
            "matches_rust_behavior_required",
        ]:
            if entry[key] is not True:
                raise AssertionError(f"{entry['target']} must keep {key}=true")
        if entry["target"] in {"swift_ios", "kotlin_android"}:
            if entry["mobile_sdk_phase12_gate"] is not True:
                raise AssertionError(f"{entry['target']} must remain Phase 12 gated")


def validate_artifact_safety() -> None:
    for path in [
        TS_WEB_MANIFEST,
        PRODUCT_MODULES,
        READINESS_GATES,
        SDK_README,
        PHASE_PLAN,
        PHASE_PROGRESS,
    ]:
        assert_no_forbidden_markers(read(path), path)


def validate_wrapper() -> None:
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_sdk_phase9.py")', VALIDATION_WRAPPER)


def validate_rust_tests() -> None:
    run(["cargo", "test", "-p", "overrid-sdk", "phase9_"])


def main() -> int:
    validate_sub_plan_phase9()
    validate_phase_docs()
    validate_sdk_code_and_readme()
    validate_ts_web_manifest()
    validate_product_modules()
    validate_readiness_gates()
    validate_artifact_safety()
    validate_wrapper()
    validate_rust_tests()
    print("SDK Phase 9 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
