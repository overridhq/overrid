# ORU Account Service Implementation Plan

## Objective

Manage Overrid Resource Unit credits as the internal non-speculative utility credit unit for resource usage, grants, holds, refunds, service charges, subscriptions, one-time app payments, provider earnings, and machine-to-machine settlement.

## First Build Phase

[Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Dependencies

- Overmeter rollups.
- Seal Ledger.
- Overpass identities.
- Overtenant scope.
- Internal KYC Service for cash-out eligibility refs.

## Development Order

1. Define person, organization, app, native service, provider, grant pool, hold, reserve, and system-service account types.
2. Implement ORU state machine: available, reserved, held, spent, earned, sponsored, refunded, corrected, expired, revoked.
3. Add resource dimensions: CPU, GPU, storage, network, memory, data, and service units.
4. Expose wallet and admin read APIs.
5. Link every transition to ledger evidence.

## Contracts And Interfaces

- ORU account schema.
- Balance query API.
- State transition refs.
- Wallet display contract.

## Detailed SDS

The detailed design contract lives in [ORU Account Service SDS](../../sds/accounting/oru_account_service.md).

## Detailed Build Plan

- [SUB BUILD PLAN #38 - ORU Account Service](../../build_plan/sub_build_plan_038_oru_account_service.md)

## Design Alignment

- Treat ORU Account Service as the account and balance projection layer, not Seal Ledger, Overbill, Overmark, Overgrant, Provider Payout Service, or a speculative token system.
- Store ORU accounts, owner refs, resource dimensions, balance projections, transition refs, reservations, holds, grants, budget prechecks, wallet views, statements, and replay bundles.
- Derive balances from append-only Seal Ledger refs and signed source records; direct mutable balance counters are not accounting truth.
- Distinguish bought credits from provider-earned ORU; bought credits can be spent on real usage but cannot become direct cash-out eligibility for the buyer. Do not let users cash out bought ORU.
- Enforce ORU-first settlement: Overrid apps and native services accept ORU inside the system, including resource usage, third-party services, subscriptions, in-app purchases, one-time charges, paid unlocks, service units, and machine-to-machine calls.
- Reject app-fee settlement paths that depend on card, bank-transfer, crypto, stablecoin, payment-link, external subscription, QR-code, or private payment bypass.
- Support accounts that both earn ORU from contributed resources or legitimate services and spend ORU elsewhere in the network.
- Keep machine-to-machine settlement low-friction through budget prechecks, reservations, holds, rollup settlement, and receipts without per-operation external payment calls.

## Validation

- Balances derive from append-only ledger entries.
- Held and reserved credits cannot be double-spent.
- Refunds and corrections append new transitions.

## Handoff

ORU accounts power wallet, Overgrant, native app usage, provider earnings, and HTTP 402-style settlement.
