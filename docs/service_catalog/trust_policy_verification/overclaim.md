# Overclaim Implementation Plan

## Objective

Handle disputes, evidence, holds, challenge windows, refunds, corrections, and settlement-finality boundaries.

## First Build Phase

Records in [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md); settlement integration in [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Dependencies

- Overwatch evidence.
- Overmeter usage.
- Oververify trust records.
- Seal Ledger holds.
- Overbill refunds.

## Development Order

1. Define dispute and claim record schemas.
2. Add evidence links and claim states.
3. Add hold and challenge-window integration.
4. Add refund and correction proposals.
5. Add resolution records and audit export.

## Contracts And Interfaces

- Dispute API.
- Evidence link schema.
- Hold/refund/correction refs.
- Resolution event contract.

## Detailed SDS

The detailed design contract lives in [Overclaim SDS](../../sds/trust_policy_verification/overclaim.md).

- [SUB BUILD PLAN #32 - Overclaim](../../build_plan/sub_build_plan_032_overclaim.md)

## Design Alignment

- Treat Overclaim as the append-only dispute, evidence, hold, correction, refund-proposal, appeal, and finality coordination layer.
- Store claim records, parties, evidence links, challenge windows, hold requests, correction/refund proposals, resolution records, appeal records, and audit exports.
- Keep ledger, billing, payout, usage, verification, and namespace changes in their owning services; Overclaim sends refs and records downstream responses.
- Preserve evidence redaction and finality boundaries so disputes can hold or correct settlement without rewriting history.

## Validation

- Disputed jobs can hold settlement.
- Corrections append new ledger entries instead of editing history.
- Final resolution cites evidence.

## Handoff

Overclaim is the dispute layer for execution, billing, provider payouts, namespace disputes, and native apps.
