#!/usr/bin/env python3
"""Validate Admin and Developer UI Phase 6 workload timeline artifacts."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from typing import Any

from validate_admin_ui_phase2 import load_json, read_text


SHELL_ROOT = Path("packages/admin_ui_shell")
SRC_ROOT = SHELL_ROOT / "src"
INDEX_TS = SRC_ROOT / "index.ts"
WORKLOAD_TIMELINE_TS = SRC_ROOT / "workload_timeline.ts"

VALID_TIMELINE = SHELL_ROOT / "fixtures/valid/workload_timeline_phase6.valid.json"
INVALID_DIRECT_OVERWATCH = SHELL_ROOT / "fixtures/invalid/workload_timeline_direct_overwatch.invalid.json"
INVALID_MISSING_GAP = SHELL_ROOT / "fixtures/invalid/workload_timeline_missing_gap_reason.invalid.json"
INVALID_PRIVATE_PAYLOAD = SHELL_ROOT / "fixtures/invalid/workload_timeline_private_payload.invalid.json"

SHELL_README = SHELL_ROOT / "README.md"
SCHEMA_README = Path("packages/schemas/admin_ui/README.md")
MANIFEST_PATH = Path("packages/schemas/admin_ui/codegen_manifest.json")
SUITE_VALIDATOR = Path("scripts/validate_admin_ui.py")
PHASE_PLAN = Path("docs/planning/admin_developer_ui_phase_06_plan.md")
PHASE_PROGRESS = Path("docs/planning/admin_developer_ui_phase_06_progress.md")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_001_admin_developer_ui.md")
MASTER_PHASE6 = Path("docs/build_plan/phase_06_first_product_integration.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")

REQUIRED_STAGES = {
    "request",
    "command_acceptance",
    "queue_item",
    "scheduler_decision",
    "lease",
    "node_assignment",
    "runner_lifecycle",
    "policy_decision",
    "usage_rollup",
    "receipt",
    "dispute",
    "correction_ref",
}
REQUIRED_OUTCOMES = {"successful", "failed", "retryable", "cancelled", "timed_out", "denied", "disputed"}
REQUIRED_GAP_CASES = {
    "missing_overmeter",
    "missing_receipt",
    "delayed_overwatch_event",
    "unavailable_dispute_service",
}
REQUIRED_OVERLAY_LAYERS = {"policy", "usage", "accounting", "dispute"}
REQUIRED_FOLLOW_CASES = {"overgate_event_stream", "bounded_polling", "disabled_without_capability"}
GAP_REQUIRED_FIELDS = {"case", "owning_service", "retry_class", "stale_age_ms", "diagnostic_reason", "refs"}
ACTIVE_DISPUTE_SIGNALS = {
    "disputed",
    "dispute_hold",
    "dispute_open",
    "dispute_active",
    "dispute_pending",
    "settlement_hold",
    "payout_hold",
}

FORBIDDEN_TIMELINE_PATTERNS = [
    re.compile(r"\bpassword\s*=", re.IGNORECASE),
    re.compile(r"\bsecret\b", re.IGNORECASE),
    re.compile(r"\bcredential\b", re.IGNORECASE),
    re.compile(r"\bprivate[_ -]?payload\b", re.IGNORECASE),
    re.compile(r"\bdecrypted\b", re.IGNORECASE),
    re.compile(r"\bprompt\b", re.IGNORECASE),
    re.compile(r"\bkey[_ -]?material\b", re.IGNORECASE),
    re.compile(r"/Users/|/home/|[A-Za-z]:\\"),
]


def assert_contains(content: str, snippet: str, path: Path) -> None:
    if snippet not in content:
        raise AssertionError(f"{path} is missing expected text: {snippet}")


def iter_strings_and_keys(value: Any, path: str = "$") -> list[tuple[str, str]]:
    if isinstance(value, str):
        return [(path, value)]
    if isinstance(value, list):
        strings: list[tuple[str, str]] = []
        for index, item in enumerate(value):
            strings.extend(iter_strings_and_keys(item, f"{path}[{index}]"))
        return strings
    if isinstance(value, dict):
        strings = []
        for key, item in value.items():
            strings.append((f"{path}.{key}<key>", str(key)))
            strings.extend(iter_strings_and_keys(item, f"{path}.{key}"))
        return strings
    return []


def private_content_paths(payload: Any) -> list[str]:
    matches: list[str] = []
    for path, value in iter_strings_and_keys(payload):
        for pattern in FORBIDDEN_TIMELINE_PATTERNS:
            if pattern.search(value):
                matches.append(path)
    return matches


def resolve_stage(node: dict[str, Any]) -> str:
    kind = node.get("kind")
    haystack = " ".join(
        [str(kind), *[str(ref) for ref in node.get("refs", [])], *[str(code) for code in node.get("reason_codes", [])]]
    ).lower()
    if kind == "gap":
        return "gap"
    if "correction" in haystack:
        return "correction_ref"
    if kind == "overgate_request" and ("command:" in haystack or "command." in haystack):
        return "command_acceptance"
    if kind == "overgate_request":
        return "request"
    if kind == "overqueue_item":
        return "queue_item"
    if kind == "oversched_placement":
        return "scheduler_decision"
    if kind == "overlease_reservation":
        return "lease"
    if kind == "overcell_execution":
        return "node_assignment"
    if kind == "overrun_result":
        return "runner_lifecycle"
    if kind == "overguard_decision":
        return "policy_decision"
    if kind == "overmeter_rollup":
        return "usage_rollup"
    if kind == "seal_ledger_receipt":
        return "receipt"
    if kind == "overclaim_dispute":
        return "dispute"
    return "gap"


def has_active_dispute_signal(haystack: str) -> bool:
    return any(signal in haystack for signal in ACTIVE_DISPUTE_SIGNALS)


def classify_fixture_outcome(case: dict[str, Any]) -> str:
    if case.get("status") == "unavailable":
        return "unavailable"
    outcome_signals = [str(case.get("status", ""))]
    for node in case.get("nodes", []):
        outcome_signals.append(str(node.get("status", "")))
        outcome_signals.extend(str(code) for code in node.get("reason_codes", []))
    for dependency in case.get("partial_dependencies", []):
        outcome_signals.append(str(dependency.get("status", "")))
        outcome_signals.append(str(dependency.get("reason_code", "")))

    haystack = " ".join(outcome_signals).lower()
    if has_active_dispute_signal(haystack):
        return "disputed"
    if "cancel" in haystack:
        return "cancelled"
    if "timeout" in haystack or "timed_out" in haystack:
        return "timed_out"
    if "denied" in haystack or "policy_denied" in haystack:
        return "denied"
    if "retryable" in haystack:
        return "retryable"
    if "failed" in haystack or "terminal" in haystack:
        return "failed"
    if case.get("status") == "partial" or case.get("partial_dependencies"):
        return "partial"
    return "successful"


def validate_source_files() -> None:
    source = read_text(WORKLOAD_TIMELINE_TS)
    for snippet in [
        "REQUIRED_TIMELINE_STAGES",
        "REQUIRED_TIMELINE_OUTCOMES",
        "REQUIRED_GAP_DEPENDENCY_CASES",
        "TIMELINE_OVERLAY_LAYERS",
        "buildWorkloadTimelineState",
        "renderTimelineNode",
        "buildTimelineGaps",
        "buildTimelineOverlays",
        "createTimelineDiagnosticBundle",
        "planTimelineFollowMode",
        "classifyTimelineOutcome",
        "hasRequiredTimelineStageCoverage",
        "hasImmutableOverlayRefs",
        "containsUnsafeDiagnosticText",
        "ACTIVE_DISPUTE_SIGNALS",
        "hasActiveDisputeSignal",
        "uniqueAuditRefs",
        "assertTimelineRoute",
        "directOverwatchConnection: false",
        "key[_ -]?material",
        '"bounded_polling"',
        '"overgate_event_stream"',
        '"admin.timeline.capability_unavailable"',
    ]:
        assert_contains(source, snippet, WORKLOAD_TIMELINE_TS)
    classifier_body = source.split("export function classifyTimelineOutcome", 1)[1].split(
        "export function hasRequiredTimelineStageCoverage", 1
    )[0]
    for forbidden in ['node.stage', 'haystack.includes("dispute")', 'haystack.includes("hold")']:
        if forbidden in classifier_body:
            raise AssertionError(
                f"{WORKLOAD_TIMELINE_TS} classifier must not use broad dispute/stage matching: {forbidden}"
            )

    index = read_text(INDEX_TS)
    assert_contains(index, "./workload_timeline", INDEX_TS)


def validate_valid_fixture() -> None:
    payload = load_json(VALID_TIMELINE)
    private_paths = private_content_paths(payload)
    if private_paths:
        raise AssertionError(f"{VALID_TIMELINE} contains private timeline content at {private_paths}")
    if payload.get("route") != "/admin/workloads/{id}/timeline":
        raise AssertionError(f"{VALID_TIMELINE} must use the Overgate admin timeline route")
    if set(payload.get("required_stages", [])) != REQUIRED_STAGES:
        raise AssertionError(f"{VALID_TIMELINE} required_stages are incomplete")
    if set(payload.get("outcomes", [])) != REQUIRED_OUTCOMES:
        raise AssertionError(f"{VALID_TIMELINE} outcomes must cover {sorted(REQUIRED_OUTCOMES)}")

    cases = payload.get("cases", [])
    outcomes = {case.get("expected_outcome") for case in cases}
    if outcomes != REQUIRED_OUTCOMES:
        raise AssertionError(f"{VALID_TIMELINE} cases do not cover outcomes: {sorted(outcomes)}")
    for case in cases:
        if case.get("status") not in {"complete", "partial", "unavailable"}:
            raise AssertionError(f"{VALID_TIMELINE} case has invalid status: {case}")
        if not case.get("workload_id") or not str(case.get("workload_id")).startswith("workload:"):
            raise AssertionError(f"{VALID_TIMELINE} case must have workload_id ref: {case}")
        if not case.get("trace_id") or not str(case.get("trace_id")).startswith("trace_"):
            raise AssertionError(f"{VALID_TIMELINE} case must have trace_id: {case}")
        if not case.get("nodes"):
            raise AssertionError(f"{VALID_TIMELINE} case must include nodes: {case}")
        classified = classify_fixture_outcome(case)
        if classified != case.get("expected_outcome"):
            raise AssertionError(
                f"{VALID_TIMELINE} case {case.get('name')} classifies as {classified}, "
                f"expected {case.get('expected_outcome')}"
            )
        for node in case.get("nodes", []):
            if not node.get("refs") or not node.get("reason_codes") or not node.get("audit_refs"):
                raise AssertionError(f"{VALID_TIMELINE} timeline nodes must cite refs, reason_codes, and audit_refs: {node}")
            if node.get("trace_id") != case.get("trace_id"):
                raise AssertionError(f"{VALID_TIMELINE} node trace id must match case trace id: {node}")

    successful_case = next(case for case in cases if case.get("expected_outcome") == "successful")
    successful_stages = {resolve_stage(node) for node in successful_case.get("nodes", [])}
    if successful_stages != REQUIRED_STAGES:
        raise AssertionError(f"{VALID_TIMELINE} successful trace stages are incomplete: {sorted(successful_stages)}")

    gap_cases = payload.get("gap_cases", [])
    if {gap.get("case") for gap in gap_cases} != REQUIRED_GAP_CASES:
        raise AssertionError(f"{VALID_TIMELINE} gap cases are incomplete")
    for gap in gap_cases:
        missing = sorted(field for field in GAP_REQUIRED_FIELDS if field not in gap)
        if missing:
            raise AssertionError(f"{VALID_TIMELINE} gap case misses {missing}: {gap}")
        if not isinstance(gap.get("stale_age_ms"), int) or gap.get("stale_age_ms") <= 0:
            raise AssertionError(f"{VALID_TIMELINE} gap case must have positive stale_age_ms: {gap}")
        if gap.get("retry_class") not in {"retryable", "waiting", "terminal", "none"}:
            raise AssertionError(f"{VALID_TIMELINE} gap case has invalid retry_class: {gap}")

    overlays = payload.get("overlays", [])
    if {overlay.get("layer") for overlay in overlays} != REQUIRED_OVERLAY_LAYERS:
        raise AssertionError(f"{VALID_TIMELINE} overlays must cover {sorted(REQUIRED_OVERLAY_LAYERS)}")
    for overlay in overlays:
        if not overlay.get("refs") or not overlay.get("audit_refs") or not overlay.get("node_ids"):
            raise AssertionError(f"{VALID_TIMELINE} overlays must cite immutable refs and audit refs: {overlay}")
        if overlay.get("redacted") is not False:
            raise AssertionError(f"{VALID_TIMELINE} overlays should be copy-safe and unredacted refs: {overlay}")

    bundle = payload.get("diagnostic_bundle", {})
    for field in ["workload_id", "trace_id", "command_refs", "reason_codes", "schema_versions", "safe_refs", "redacted_fields"]:
        if field not in bundle:
            raise AssertionError(f"{VALID_TIMELINE} diagnostic bundle is missing {field}")
    if len(bundle.get("audit_refs", [])) < 3:
        raise AssertionError(f"{VALID_TIMELINE} diagnostic bundle must include node-level audit refs")
    if not all(str(ref).startswith("command:") for ref in bundle.get("command_refs", [])):
        raise AssertionError(f"{VALID_TIMELINE} diagnostic command_refs must be command refs")
    if private_content_paths(bundle):
        raise AssertionError(f"{VALID_TIMELINE} diagnostic bundle must be copy-safe")

    follow_modes = payload.get("follow_modes", [])
    if {mode.get("case") for mode in follow_modes} != REQUIRED_FOLLOW_CASES:
        raise AssertionError(f"{VALID_TIMELINE} follow mode cases are incomplete")
    for mode in follow_modes:
        if mode.get("route") != "/admin/workloads/{id}/timeline":
            raise AssertionError(f"{VALID_TIMELINE} follow mode must stay on timeline route: {mode}")
        if mode.get("uses_overgate_only") is not True or mode.get("direct_overwatch_connection") is not False:
            raise AssertionError(f"{VALID_TIMELINE} follow mode must not bypass Overgate: {mode}")
        if mode.get("refresh_window_ms", 0) < 5000 or mode.get("stale_after_ms", 0) <= 0:
            raise AssertionError(f"{VALID_TIMELINE} follow mode must be bounded: {mode}")
    event_stream = next(mode for mode in follow_modes if mode.get("case") == "overgate_event_stream")
    polling = next(mode for mode in follow_modes if mode.get("case") == "bounded_polling")
    if event_stream.get("manual_refresh") is not False or polling.get("manual_refresh") is not True:
        raise AssertionError(f"{VALID_TIMELINE} follow modes must distinguish stream and polling refresh")


def validate_invalid_fixtures() -> None:
    direct = load_json(INVALID_DIRECT_OVERWATCH)
    direct_modes = direct.get("follow_modes", [])
    if not any(mode.get("direct_overwatch_connection") is True for mode in direct_modes):
        raise AssertionError(f"{INVALID_DIRECT_OVERWATCH} must contain direct Overwatch connection")
    if not any(mode.get("uses_overgate_only") is False for mode in direct_modes):
        raise AssertionError(f"{INVALID_DIRECT_OVERWATCH} must bypass Overgate")

    missing_gap = load_json(INVALID_MISSING_GAP)
    gaps = missing_gap.get("gap_cases", [])
    if not any(sorted(field for field in GAP_REQUIRED_FIELDS if field not in gap) for gap in gaps):
        raise AssertionError(f"{INVALID_MISSING_GAP} must contain a gap missing required fields")

    private_payload = load_json(INVALID_PRIVATE_PAYLOAD)
    if not private_content_paths(private_payload):
        raise AssertionError(f"{INVALID_PRIVATE_PAYLOAD} must fail private-content validation")


def validate_manifest_and_docs() -> None:
    manifest = load_json(MANIFEST_PATH)
    entries = [
        entry
        for entry in manifest.get("phase_artifacts", [])
        if entry.get("phase") == "admin_ui_phase6"
    ]
    if len(entries) != 1:
        raise AssertionError(f"{MANIFEST_PATH} must contain one admin_ui_phase6 phase_artifacts entry")
    entry = entries[0]
    if entry.get("validator") != "scripts/validate_admin_ui_phase6.py":
        raise AssertionError(f"{MANIFEST_PATH} Phase 6 validator path is wrong")
    if str(WORKLOAD_TIMELINE_TS) not in entry.get("source_files", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 6 source_files must include {WORKLOAD_TIMELINE_TS}")
    if str(VALID_TIMELINE) not in entry.get("fixtures", {}).get("valid", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 6 valid fixtures must include {VALID_TIMELINE}")

    required_text = {
        SHELL_README: [
            "Phase 6 trace-linked workload timeline artifacts",
            "src/workload_timeline.ts",
            "scripts/validate_admin_ui_phase6.py",
        ],
        SCHEMA_README: [
            "Phase 6 trace-linked workload timeline artifacts",
            "workload timeline fixtures",
            "validate_admin_ui_phase6.py",
        ],
        PHASE_PLAN: [
            "Complete the Phase 6 trace-linked workload timeline",
            "packages/admin_ui_shell/src/workload_timeline.ts",
            "scripts/validate_admin_ui_phase6.py",
        ],
        PHASE_PROGRESS: [
            "Admin and Developer UI Phase 6 Progress",
            "Validation Evidence",
        ],
        SUB_PLAN: [
            "## Phase 6: Trace-Linked Workload Timeline",
            "Build timeline assembly UI",
            "Build follow-mode event updates",
        ],
        MASTER_PHASE6: [
            "Developer And Admin UI",
            "operator can trace each job",
        ],
        TECH_STACK: [
            "TypeScript web UI is acceptable",
            "Overgate",
            "never become a privileged backdoor",
        ],
        SUITE_VALIDATOR: [
            "scripts/validate_admin_ui_phase6.py",
        ],
    }
    for path, snippets in required_text.items():
        content = read_text(path)
        for snippet in snippets:
            assert_contains(content, snippet, path)


def validate_file_hygiene() -> None:
    checked_paths = [
        INDEX_TS,
        WORKLOAD_TIMELINE_TS,
        VALID_TIMELINE,
        INVALID_DIRECT_OVERWATCH,
        INVALID_MISSING_GAP,
        INVALID_PRIVATE_PAYLOAD,
        SHELL_README,
        SCHEMA_README,
        MANIFEST_PATH,
        PHASE_PLAN,
        PHASE_PROGRESS,
    ]
    for path in checked_paths:
        content = read_text(path)
        if not content.endswith("\n"):
            raise AssertionError(f"{path} must end with a newline")
        if "\t" in content:
            raise AssertionError(f"{path} must not contain tabs")


def main() -> int:
    validate_source_files()
    validate_valid_fixture()
    validate_invalid_fixtures()
    validate_manifest_and_docs()
    validate_file_hygiene()
    print("Admin UI Phase 6 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
