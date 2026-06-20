# Seal Ledger Implementation Plan

## Objective

Build append-only internal accounting for ORU balances, usage rollups, holds, corrections, disputes, and settlement history.

## First Build Phase

[Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Dependencies

- ORU account service.
- Overmeter signed rollups.
- Overclaim dispute records.
- Overbill payment refs.

## Development Order

1. Define ledger entry schema and invariants.
2. Add reservation, settlement, hold, release, refund, correction, provider earning, grant allocation, and native service usage entries.
3. Add account, workload, tenant, provider, and dispute indexes.
4. Add balance derivation views.
5. Add audit export.

## Contracts And Interfaces

- Ledger entry schema.
- Append API.
- Balance view API.
- Evidence refs.
- Audit export format.

## Validation

- Ledger entries are append-only.
- Balance views reconcile with entries.
- Disputes and corrections never mutate historical entries.

## Handoff

Seal Ledger is the accounting backbone for ORU, Overbill, Overclaim, wallet, grants, and stewardship reporting.

## Detailed SDS

The detailed design contract lives in [Seal Ledger SDS](../../sds/accounting/seal_ledger.md).

## Sub-Build Plan

[SUB BUILD PLAN #44 - Seal Ledger](../../build_plan/sub_build_plan_044_seal_ledger.md).

## Design Alignment

- Treat Seal Ledger as append-only internal accounting truth with entry streams, batches, checkpoints, replay, reconciliation, and correction-by-new-entry.
- Do not expose mutable balances as ledger truth; ORU Account Service derives balance projections from entries.
- Do not implement blockchain, mining, NFT records, speculative token mechanics, gas fees, or per-operation external payment tolls.
- Feed ORU Account Service, Overbill, Provider Payout Service, Overgrant, Overclaim, Wallet and Usage Center, stewardship reporting, and Backup and Restore Service with stable ledger entry refs.
