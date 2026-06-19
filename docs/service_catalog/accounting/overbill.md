# Overbill Implementation Plan

## Objective

Handle fiat billing, invoices, payment-provider integration, taxes, refunds, chargebacks, provider payouts, payout holds, and audit export.

## First Build Phase

[Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Dependencies

- Seal Ledger.
- ORU account service.
- Overclaim.
- Provider payout service.

## Development Order

1. Define invoice, receipt, payment reference, refund, chargeback, and payout batch schemas.
2. Integrate payment providers behind a narrow adapter contract.
3. Add receipts for usage and ORU transitions.
4. Add refund and chargeback handling.
5. Add audit and compliance export hooks.

## Contracts And Interfaces

- Invoice API.
- Receipt API.
- Payment provider adapter.
- Refund and chargeback event contract.

## Detailed SDS

The detailed design contract lives in [Overbill SDS](../../sds/accounting/overbill.md).

## Design Alignment

- Treat Overbill as the billing-document and external-payment-ref bridge, not usage truth, ORU balance storage, provider payout execution, or dispute adjudication.
- Store receipts, invoices, line items, payment provider refs, signed external payment events, refunds, chargebacks, payout batch inputs, payout hold views, statements, audit exports, and reconciliation jobs.
- Link external payment state to Seal Ledger and ORU refs without rewriting ledger history or creating per-operation external payment calls.
- Feed Wallet and Usage Center, Provider Payout Service, Overclaim, compliance exports, native apps, SDK, CLI, admin UI, and central AI stewardship with auditable billing views.

## Validation

- Internal ORU transitions work without per-operation external payment calls.
- External payment state is linked but does not rewrite Seal Ledger history.
- Refunds and chargebacks create auditable corrections.

## Handoff

Overbill connects Seal Ledger to compliant external payment rails and provider payouts.
