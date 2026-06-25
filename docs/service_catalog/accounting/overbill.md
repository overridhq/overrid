# Overbill Implementation Plan

## Objective

Handle external payment boundary records, invoices, payment-provider integration, taxes, refunds, chargebacks, provider payouts, payout holds, and audit export for an ORU-first internal economy.

## First Build Phase

[Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Dependencies

- Seal Ledger.
- ORU account service.
- Overclaim.
- Provider payout service.
- Internal KYC Service.

## Development Order

1. Define invoice, receipt, payment reference, manual high-credit request, funding AML hold, refund, chargeback, and payout batch schemas.
2. Integrate payment providers behind a narrow adapter contract.
3. Add receipts for usage and ORU transitions.
4. Add refund and chargeback handling.
5. Add audit and compliance export hooks.

## Contracts And Interfaces

- Invoice API.
- Receipt API.
- Payment provider adapter.
- Manual high-credit and AML funding hold contract.
- Refund and chargeback event contract.

## Detailed SDS

The detailed design contract lives in [Overbill SDS](../../sds/accounting/overbill.md).

## Sub-Build Plan

[SUB BUILD PLAN #40 - Overbill](../../build_plan/sub_build_plan_040_overbill.md).

## Design Alignment

- Treat Overbill as the billing-document and external-payment-ref bridge, not usage truth, ORU balance storage, provider payout execution, or dispute adjudication.
- Store receipts, invoices, line items, payment provider refs, signed external payment events, refunds, chargebacks, payout batch inputs, payout hold views, statements, audit exports, and reconciliation jobs.
- Link external payment state to Seal Ledger and ORU refs without rewriting ledger history or creating per-operation external payment calls.
- Enforce active policy caps for automated credit purchases and route larger requests to Internal KYC/source-of-funds review.
- Keep bought ORU out of cash-out eligibility. Do not let users cash out bought ORU.
- Keep ORU as the only internal payment medium for native services, third-party apps, subscriptions, in-app purchases, one-time service charges, paid unlocks, paid listings, resource usage, and machine-to-machine calls.
- Treat external payment rails as boundary rails for ORU funding, refund, chargeback, tax, and eligible provider payout, not as separate in-system checkout paths for each app.
- Preserve app monetization terms refs and block payout eligibility for app fees collected through external checkout/payment bypass.
- Feed Wallet and Usage Center, Provider Payout Service, Overclaim, compliance exports, native apps, SDK, CLI, admin UI, and central AI stewardship with auditable billing views.

## Validation

- Internal ORU transitions work without per-operation external payment calls.
- External payment state is linked but does not rewrite Seal Ledger history.
- Credit purchases above active policy caps are denied or held for manual high-credit review before credits become spendable.
- External checkout/payment bypass cannot create payout-eligible app earnings.
- Refunds and chargebacks create auditable corrections.

## Handoff

Overbill connects Seal Ledger to compliant external payment rails and provider payouts.
