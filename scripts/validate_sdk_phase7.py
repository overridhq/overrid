#!/usr/bin/env python3
"""Validate SDK Phase 7 usage, receipt, ORU, and dispute reader gates."""

from __future__ import annotations

from pathlib import Path
import re
import subprocess
import sys


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_006_sdk.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/sdk_phase_07_plan.md")
PHASE_PROGRESS = Path("docs/planning/sdk_phase_07_progress.md")
SDK_CARGO = Path("packages/sdk/Cargo.toml")
SDK_LIB = Path("packages/sdk/src/lib.rs")
SDK_ACCOUNTING = Path("packages/sdk/src/accounting.rs")
SDK_README = Path("packages/sdk/README.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

REQUIRED_PHASE7_SYMBOLS = [
    "pub mod accounting;",
    "pub use accounting::*;",
    "AccountingReaders",
    "SDK_PHASE7_CAPABILITY_PROFILE",
    "SDK_PHASE7_USAGE_RECEIPT_ROUTE",
    "SDK_PHASE7_USAGE_ROLLUP_ROUTE",
    "SDK_PHASE7_ORU_CHARGE_PREVIEW_ROUTE",
    "SDK_PHASE7_SEAL_LEDGER_REF_ROUTE",
    "SDK_PHASE7_DISPUTE_REFERENCE_ROUTE",
    "SDK_PHASE7_ACCOUNTING_AUTHORITY_OWNERS",
    "SdkAccountingReadKind",
    "SdkAccountingRequestMetadata",
    "SdkAccountingReadRequest",
    "build_accounting_read_request",
    "validate_accounting_reader_readiness",
    "SdkOruDimensionTotal",
    "SdkAccountingRefBundle",
    "verify_accounting_refs_unchanged",
    "SdkUsageReceiptViewInput",
    "SdkUsageReceiptView",
    "decode_usage_receipt_view",
    "SdkOruChargePreviewInput",
    "SdkOruChargePreview",
    "decode_oru_charge_preview",
    "SdkAccountingDisputeReferenceInput",
    "SdkAccountingDisputeReferenceView",
    "decode_dispute_reference_view",
    "SdkAccountingErrorSurface",
    "build_accounting_error_surface",
    "build_accounting_receipt_idempotency_entry",
    "SdkPhase7AuthorityBoundary",
    "sdk_phase7_authority_review",
    "validate_phase7_authority_review",
    "SdkPhase7Error",
]

REQUIRED_PHASE7_TESTS = [
    "phase7_accounting_readers_are_capability_gated_and_read_only",
    "phase7_usage_receipt_view_consumes_service_objects_without_charge_tables",
    "phase7_metering_and_accounting_refs_round_trip_without_rewriting",
    "phase7_dispute_and_correction_readers_surface_stable_refs",
    "phase7_authority_review_blocks_payment_calls_and_accounting_leaks",
    "phase7_accounting_helpers_fail_closed_before_readiness",
    "phase7_error_surfaces_preserve_refs_without_generic_masking",
    "phase7_accounting_receipt_idempotency_uses_receipt_retention",
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def run(command: list[str]) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(command, cwd=REPO_ROOT, text=True, capture_output=True)
    if result.returncode != 0:
        sys.stdout.write(result.stdout)
        sys.stderr.write(result.stderr)
        raise AssertionError(f"Command failed: {' '.join(command)}")
    return result


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


def validate_sub_plan_phase7() -> None:
    text = read(SUB_PLAN)
    phase_7 = section(text, "## Phase 7: Usage, Receipt, ORU, And Dispute Readers")
    for item in range(1, 6):
        assert_contains(phase_7, f"**7.{item} ", SUB_PLAN)
    for work_item in re.finditer(
        r"- \*\*7\.[1-5] .+?(?=\n- \*\*7\.|\n## Phase 8:)",
        phase_7,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")
    for expected in [
        "usage_receipt_view",
        "ORU charge previews",
        "Seal Ledger references",
        "dispute refs",
        "direct payment",
        "Phase 5 and Phase 6 readiness",
    ]:
        assert_contains(phase_7, expected, SUB_PLAN)


def validate_phase_docs() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in [
        "Complete the Phase 7 gate",
        "usage receipt views",
        "ORU dimension totals",
        "dispute, correction, challenge-window",
        "direct payment-provider calls",
        "Phase 5 accounting API readiness",
        "cargo test -p overrid-sdk",
        "Do not add new SDK dependencies",
    ]:
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in [
        "Loaded Docdex profile memory and repo memory",
        "Docdex impact graph",
        "Local delegation was attempted",
        "Saved the repo-scoped Phase 7 session context",
        "Validation Evidence",
    ]:
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_sdk_code_and_readme() -> None:
    cargo = read(SDK_CARGO)
    lib = read(SDK_LIB)
    accounting = read(SDK_ACCOUNTING)
    readme = read(SDK_README)
    combined = f"{lib}\n{accounting}"

    for expected in REQUIRED_PHASE7_SYMBOLS + REQUIRED_PHASE7_TESTS:
        assert_contains(combined, expected, SDK_ACCOUNTING)
    for expected in [
        "service_returned_evidence_only: true",
        "mutates_accounting_state: false",
        "embeds_charge_tables: false",
        "client_side_settlement_decision: false",
        "generic_error_masked: false",
        "hides_holds_or_corrections: false",
        "RefRewriteDetected",
        "GenericAccountingErrorMask",
        "ReadinessGateClosed",
        "AccountingAuthorityLeak",
        "Overmeter",
        "ORU Account Service",
        "Seal Ledger",
        "Overbill",
        "Overgrant",
        "Overasset",
        "Overclaim",
        "Provider Payout Service",
        "Overwatch",
    ]:
        assert_contains(accounting, expected, SDK_ACCOUNTING)
    for forbidden in [
        "external_payment_provider_calls: true",
        "embeds_charge_tables: true",
        "client_side_settlement_decision: true",
        "mutates_accounting_state: true",
        "stripe",
        "paypal",
        "payment_provider_sdk",
        "local_charge_table",
        "price_table",
        "revenue_forecast",
        "customer-count",
        "blockchain",
        "nft",
    ]:
        if forbidden in accounting.lower():
            raise AssertionError(f"{SDK_ACCOUNTING} contains forbidden authority leak: {forbidden}")
    for expected in [
        "## Phase 7 Usage, Receipt, ORU, And Dispute Readers",
        "`build_accounting_read_request()`",
        "`validate_accounting_reader_readiness()`",
        "`decode_usage_receipt_view()`",
        "`decode_oru_charge_preview()`",
        "`decode_dispute_reference_view()`",
        "`build_accounting_error_surface()`",
        "`verify_accounting_refs_unchanged()`",
        "`sdk_phase7_authority_review()`",
        "`validate_phase7_authority_review()`",
        "direct payment-provider calls",
        "embedded charge tables",
        "client-side settlement decisions",
    ]:
        assert_contains(readme, expected, SDK_README)
    if "[dependencies]\noverrid-contracts" not in cargo:
        raise AssertionError(f"{SDK_CARGO} should remain dependency-light for Phase 7")


def validate_tech_stack_alignment() -> None:
    tech_stack = read(TECH_STACK)
    accounting = read(SDK_ACCOUNTING)
    for expected in [
        "| SDKs | Generated Rust SDK first",
        "Signed command envelopes",
        "idempotency keys",
        "trace ids",
        "stable reason codes",
        "Overrid-owned storage abstraction",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)
    for forbidden in [
        "postgres",
        "redis",
        "kafka",
        "nats",
        "s3",
        "typescript",
    ]:
        if forbidden in accounting.lower():
            raise AssertionError(f"{SDK_ACCOUNTING} contains forbidden stack term: {forbidden}")


def validate_wrapper() -> None:
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_sdk_phase7.py")', VALIDATION_WRAPPER)


def validate_rust_tests() -> None:
    result = run(["cargo", "test", "-p", "overrid-sdk", "phase7_"])
    if "phase7_" not in result.stdout and "phase7_" not in result.stderr:
        raise AssertionError("Phase 7 focused Rust tests did not run")


def main() -> int:
    validate_sub_plan_phase7()
    validate_phase_docs()
    validate_sdk_code_and_readme()
    validate_tech_stack_alignment()
    validate_wrapper()
    validate_rust_tests()
    print("SDK Phase 7 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
