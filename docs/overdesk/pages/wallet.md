# Wallet

## Slug

`wallet`

## Title

Wallet

## Navigation Group

Wallet, Credits, And Ownership

## Description

Wallet is the Overdesk surface for ORU balance projections, usage, receipts, grants, holds, refunds, statements, app permissions, privacy audit, provider contribution summaries, and dispute handoffs. It is a visibility and control page, not an accounting ledger or payment authority.

Wallet must present Overrid as an ORU-first economy: users can buy ORU, earn ORU by sharing approved resources or running legitimate services, and spend ORU on native services, third-party apps, subscriptions, one-time charges, resource usage, and machine-to-machine calls. Cash-out appears only as a separate provider-eligibility path.

## Primary Users

- Regular users
- Organization admins
- Institution users
- App owners
- Resource providers
- Delegated account managers
- Stewards with authorized accounting views

## Primary User Goals

- See account balances and usage in understandable ORU dimensions.
- Review receipts, holds, grants, sponsored credits, refunds, and corrections.
- Understand which apps and services consumed credits.
- Understand which balances are spendable ORU, earned ORU, sponsored ORU, held ORU, or provider payout candidates.
- Inspect and clean up app permissions.
- Export statements.
- Open disputes or corrections.
- Move to Buy Credits when balance is low.

## Entry Points

- Wallet navigation group.
- Home Dashboard wallet summary.
- Buy Credits return path.
- Activity And Receipts Timeline.
- Notifications Center low-balance or hold alert.
- Personal AI Assistant usage explanation.
- Provider Earnings And Payouts.
- Address bar command: `/wallet`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Wallet freshness state.
- Balance projection source.
- Primary action: Buy Credits.
- Secondary actions: Export Statement, Permissions, Disputes.

### Account Selector

Content:

- Personal accounts.
- Organization accounts.
- Institution accounts.
- App-owner accounts.
- Delegated accounts.
- Role and visibility marker.
- Default account setting.
- Redaction state.

### Balance Overview

Content:

- ORU dimensions.
- Available, reserved, held, spent, earned, sponsored, refunded, and corrected projections.
- Seal Ledger checkpoint refs.
- ORU Account Service projection refs.
- Stale or reconciliation warning.
- Low-balance warning bands.

Links and handoffs:

- Buy Credits.
- Activity And Receipts Timeline.
- Grants And Public-Interest Projects.

### Usage Summary

Content:

- Time window selector.
- Usage by native app.
- Usage by service.
- Usage by model route.
- Usage by storage, compute, GPU, bandwidth, and data dimensions.
- Usage trend.
- Source-service refs.
- Receipt refs.

Links and handoffs:

- Personal AI Assistant.
- Owned Apps.
- Provider Earnings And Payouts.

### Receipts List

Content:

- Receipt date.
- Source app/service.
- Operation class.
- ORU dimension.
- Status.
- Refund/correction marker.
- Dispute marker.
- Export/open actions.

Links and handoffs:

- Activity And Receipts Timeline.
- Disputes And Appeals.

### Grants, Holds, And Sponsored Credits

Content:

- Active grants.
- Sponsored credits.
- Purpose scopes.
- Holds.
- Reservations.
- Release or expiry state.
- Source refs.
- Eligibility or reporting requirements.

Links and handoffs:

- Grants And Public-Interest Projects.
- Central AI Stewardship.
- Disputes And Appeals.

### Permissions And Privacy Audit

Content:

- App/service permissions.
- AI context grants.
- Location grants.
- Workspace grants.
- Messaging grants.
- Vault grants.
- RAG grants.
- Revoke, narrow, expire, or review actions.
- Privacy audit refs.

Links and handoffs:

- Privacy And Permissions Center.
- Overvault Secure Storage Center.
- Docdex And RAG Index Manager.

### Provider Contribution Summary

Content:

- This computer contribution summary.
- Owned node contribution summary.
- Earned projections.
- Pending holds.
- Active leases count.
- Recent provider receipts.
- Payout status marker.

Links and handoffs:

- Add This Computer To Overrid.
- Resource Sharing Rules.
- Provider Earnings And Payouts.
- Node Fleet Manager.

### Statements, Exports, And Disputes

Content:

- Statement request controls.
- Export format selector.
- Redaction profile.
- Statement job state.
- Open dispute action.
- Existing dispute refs.
- Refund/correction refs.

## Primary Actions

- Buy Credits.
- Change Account.
- Open Receipt.
- Export Statement.
- Review Permissions.
- Open Dispute.

## Secondary Actions

- Ask AI to explain usage.
- Filter usage.
- Copy receipt ref.
- Open grant refs.
- Revoke permission.
- Open provider summary.
- Refresh wallet view.

## States

- Empty account.
- Loading.
- Live.
- Stale projection.
- Reconciliation pending.
- Low balance.
- Account restricted.
- Permission denied.
- Receipt unavailable.
- Statement building.
- Statement ready.
- Dispute open.
- Partial accounting-service outage.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Wallet must cite ORU Account Service projections and Seal Ledger checkpoints for balance views.
- Overdesk must not create ledger entries, mutate balances, store payment secrets, or invent usage truth.
- Permission cleanup requests route to owning services and must show pending, accepted, denied, or failed state.
- Statements and exports require redaction profiles for provider-sensitive, fraud-sensitive, organization-only, and private-app details.
- Wallet and Usage Center owns wallet views; ORU Account Service, Seal Ledger, Overbill, Overgrant, Overmeter, Overclaim, Provider Payout Service, and owner services own authoritative accounting and dispute state.

## Design Notes

- Use an account-scoped dashboard with compact balance tiles, tables for receipts, and a detail side panel.
- Keep Buy Credits available without making the page feel like a sales funnel.
- Show source refs and checkpoint refs in details, not in the main scan path.
- Make stale/reconciliation states visually strong because users must not treat stale projections as final truth.
- Keep permission cleanup close to the usage that caused concern.
