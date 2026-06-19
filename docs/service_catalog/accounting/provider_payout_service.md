# Provider Payout Service Implementation Plan

## Objective

Batch and hold provider earnings safely before external payout.

## First Build Phase

[Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) for private providers; [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md) for public providers.

## Dependencies

- Seal Ledger provider earning entries.
- Overbill payment rails.
- Overclaim dispute state.
- Oververify trust evidence.
- Anti-Sybil service for public providers.

## Development Order

1. Aggregate provider earnings from ledger entries.
2. Apply dispute-window and verification holds.
3. Add payout eligibility checks.
4. Create payout batches and payment refs.
5. Handle payout failure, reversal, and correction.

## Contracts And Interfaces

- Provider earning view.
- Payout batch schema.
- Hold and release rules.
- Payment provider refs.

## Validation

- Disputed or risky earnings cannot be paid out early.
- Failed payouts preserve accounting evidence.
- Public-provider holds can be triggered by fraud signals.

## Handoff

Provider payouts support private swarms, federation, public pools, and Overbill.

## Detailed SDS

The detailed design contract lives in [Provider Payout Service SDS](../../sds/accounting/provider_payout_service.md).

## Design Alignment

- Treat Provider Payout Service as payout eligibility, hold, batch, result, failure, reversal, and correction coordination, not the source of earning truth or a payment processor.
- Derive provider earning views from Seal Ledger refs and submit external payout work through Overbill/payment-provider refs.
- Preserve dispute, verification, fraud, challenge, anti-Sybil, compliance, and chargeback holds before any payout batch can be submitted.
- Keep Phase 11 public-provider payout controls stricter than private-provider Phase 5 payout controls; public supply is adversarial by default.
