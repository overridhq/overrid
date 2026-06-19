# Phase 5: Metering, ORU, Seal Ledger, and Overbill

## Objective

Make resource usage accountable without blockchain, NFT speculation, or per-transaction fee friction.

ORU credits and Seal Ledger are internal utility infrastructure for resource accounting, settlement, grants, holds, corrections, and machine-to-machine payments. They are not speculative tokens.

## Depends On

- Phase 3 raw usage events.
- Phase 4 policy decisions, disputes, and verification evidence.
- Identity, tenant, provider, app, and native service records.

## Build Order

1. Define ORU account model.
2. Define ORU resource dimensions and state machine.
3. Sign Overmeter rollups.
4. Build Seal Ledger append-only accounting.
5. Add holds, corrections, refunds, and dispute links.
6. Add Overmark resource cards and bounded reference rates.
7. Add Overgrant primitives for sponsored, grant-funded, and purpose-scoped allocation.
8. Add Overasset utility records for non-speculative resource rights and operational ownership references.
9. Add Overbill receipts, invoices, payouts, and audit export.
10. Prove internal accounting without per-operation external payment calls.

## Workstream 1: ORU Account Model

Create account types for:

- Person.
- Organization.
- App.
- Native service.
- Provider.
- Grant pool.
- Escrow or hold.
- Reserve.
- System service.

Accounts must support tenant scope, owner identity, compliance flags, suspension state, and audit links.

## Workstream 2: ORU Dimensions

Track resource dimensions:

- CPU-ORU.
- GPU-ORU.
- STOR-ORU.
- NET-ORU.
- MEM-ORU.
- DATA-ORU.
- Service-ORU where native app service units need abstraction.

Keep dimensions explicit so resource accounting does not hide GPU, storage, network, and model costs inside one vague balance.

## Workstream 3: ORU State Machine

Implement states:

- Available.
- Reserved.
- Held.
- Spent.
- Earned.
- Sponsored.
- Refunded.
- Corrected.
- Expired.
- Revoked.

Every transition must reference a command, usage rollup, policy decision, dispute, or operator action.

## Workstream 4: Overmeter Rollups

Convert raw usage into signed rollups:

- Workload id.
- Tenant id.
- Provider id.
- Node id.
- Resource dimensions.
- Time window.
- Raw event refs.
- Policy refs.
- Signature.
- Dispute window.
- Cache dimensions from Overcache: hit, miss, write, read, storage bytes, egress, warming, eviction, retention, invalidation, and saved upstream work.
- Model inference, RAG retrieval, mesh transfer, storage, native app, and service-unit dimensions as those source services come online.

Rollups are the bridge between execution and accounting.

## Workstream 5: Seal Ledger

Build append-only ledger entries for:

- Reservation.
- Settlement.
- Hold.
- Release.
- Refund.
- Correction.
- Provider earning.
- Grant allocation.
- Native service usage.
- System-service usage.

Seal Ledger must be queryable by account, workload, tenant, provider, and dispute.

## Workstream 6: Overmark

Add bounded resource cards and valuation signals:

- Resource card version.
- Resource class.
- Capability tier.
- Trust tier.
- Locality.
- Availability.
- Reference rate band.
- Budget signal.

Do not encode speculative market behavior here. Overmark should help placement and accounting remain predictable.

## Workstream 7: Overgrant Primitives

Build the first Overgrant primitives for:

- Grant source account.
- Eligible identity, tenant, app, or workload class.
- Resource dimensions.
- Time window.
- Quota.
- Purpose scope.
- Abuse throttle.
- Reporting requirement.

This phase only needs local/private allocation primitives. Cross-tenant federation and public-interest pool expansion happens in Phase 10.

## Workstream 8: Overasset Utility Records

Build Overasset as non-speculative operational records for:

- Resource rights.
- Service ownership references.
- Storage entitlement references.
- Namespace-bound asset records where needed.
- Transfer and revocation audit references.
- Dispute and correction links.

Overasset must not behave like NFTs. It exists to represent rights and operational ownership inside Overrid without speculative token mechanics.

## Workstream 9: Overbill

Build billing records:

- Usage receipt.
- Invoice.
- Payment provider reference where needed.
- Refund record.
- Provider payout batch.
- Payout hold.
- Audit export.
- Account statement.

External payments should be batched where possible. Internal ORU transitions should not require external fees per small operation.

## Workstream 10: Machine-To-Machine Payments

Prepare low-friction service payment flows:

- Pre-authorized budget.
- Small usage holds.
- Rollup settlement.
- Receipt delivery.
- Budget exhaustion handling.
- Service-to-service usage attribution.

This is the path toward HTTP 402-style machine-to-machine payments without blockchain tolls.

## Validation

- Private workloads produce signed usage rollups.
- Rollups create ORU reservations, settlements, and receipts.
- Disputed workloads move settlement into hold state.
- Refund or correction produces a new ledger entry instead of editing history.
- Provider earnings are batchable and can be held.
- User can inspect usage, holds, refunds, and receipts.

## Exit Gate

Phase 5 is complete when Overrid can account for real private workloads through ORU and Seal Ledger without relying on blockchain, NFTs, or per-operation external payment rails.

## Handoff To Phase 6

Phase 6 integrates real ecosystem products so metering and accounting are proven by useful workloads instead of synthetic examples.
