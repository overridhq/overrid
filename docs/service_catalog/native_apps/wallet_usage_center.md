# Wallet and Usage Center Implementation Plan

## Objective

Build the user-facing control panel for ORU balances, usage, grants, holds, refunds, receipts, app permissions, and service costs.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md).

## Dependencies

- ORU account service.
- Seal Ledger.
- Overbill.
- Overgrant.
- Overpass.

## Development Order

1. Build balance and usage views.
2. Add holds, refunds, grants, sponsored credits, and receipts.
3. Add account statements and app permissions.
4. Add privacy controls and export options.
5. Add mobile-friendly API surfaces.

## Contracts And Interfaces

- Balance view API.
- Receipt view API.
- Permission control API.
- Account statement export.

## Validation

- User can see resource usage and receipts by app/service.
- Holds and refunds reconcile with Seal Ledger.
- App permissions can be reviewed and revoked.

## Handoff

Wallet is the first native app because every other native service depends on usage, balances, and permissions.

## Detailed SDS

The detailed design contract is [Wallet and Usage Center SDS](../../sds/native_apps/wallet_usage_center.md).

## Design Alignment

- Treat Wallet and Usage Center as a user-facing projection and control app, not as ORU Account Service, Seal Ledger, Overbill, Overgrant, Overmeter, Overclaim, or an external payment processor.
- Require wallet profiles, account selectors, balance views, usage dashboards, receipt collections, statement/export jobs, app permission controls, revocation requests, privacy audit views, dispute handoffs, notification prefs, and usage refs.
- Read authoritative accounting and usage data from ORU Account Service, Seal Ledger, Overbill, Overgrant, Overmeter, and Overclaim instead of mutating balances or ledger truth directly.
- Make app permissions, privacy access, receipts, holds, grants, refunds, and usage visible and revocable without blockchain/NFT framing, hardcoded prices, or revenue projections.
