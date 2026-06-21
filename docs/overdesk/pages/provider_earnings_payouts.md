# Provider Earnings And Payouts

## Slug

`provider-earnings-payouts`

## Title

Provider Earnings And Payouts

## Navigation Group

Wallet, Credits, And Ownership

## Description

Provider Earnings And Payouts is the Overdesk surface for contributed-resource earnings, payout eligibility, holds, payout batches, payout destination refs, failed payouts, reversals, corrections, and provider-visible reason codes. It must help providers understand what happened without making Overdesk the source of payout, usage, ledger, trust, or payment truth.

## Primary Users

- Resource providers
- Node owners
- Organization admins
- Institution resource managers
- Delegated provider account managers
- Support operators with authorized provider views
- Stewards reviewing provider payout evidence

## Primary User Goals

- See earnings by node, time window, resource class, and account.
- Understand what is payable, pending, held, failed, corrected, or paid.
- See why a payout item is blocked without exposing fraud internals.
- Review payout batches and external payout status.
- Manage payout destination refs through the proper owner-service flow.
- Open disputes or appeals for held, failed, or corrected payout items.
- Export provider-safe statements and audit summaries.

## Entry Points

- Wallet.
- Home Dashboard provider snapshot.
- Resource Sharing Rules.
- Node Fleet Manager.
- Node Detail.
- Notifications Center payout or hold alert.
- Activity And Receipts Timeline.
- Disputes And Appeals.
- Address bar command: `/provider-payouts`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Provider account selector.
- Current payout period.
- Provider visibility state.
- Primary action: Review Payouts.
- Secondary actions: Export Statement, Open Dispute, Payout Settings.

### Provider Account Selector

Content:

- Personal provider account.
- Organization provider account.
- Institution provider account.
- Delegated provider account.
- Payout role marker.
- Payout destination setup state.
- Compliance requirement marker.

Links and handoffs:

- Wallet.
- Identity And Profile Center.
- Settings And Security.

### Earnings Overview

Content:

- Current-period earning projection.
- Closed-period earning view.
- Eligible amount.
- Held amount.
- Batched amount.
- Paid amount.
- Failed or reversed amount.
- Correction amount.
- Source refs from Overmeter, Seal Ledger, ORU Account Service, Overmark, Overlease, and Provider Payout Service.

Links and handoffs:

- Wallet.
- Activity And Receipts Timeline.

### Node Contribution Breakdown

Content:

- Node list.
- Resource classes used.
- Lease count.
- Workload class summary.
- Uptime and availability summary.
- Usage dimensions.
- Earnings by node.
- Policy denials and pause/drain markers.
- Suspicious workload report shortcut.

Links and handoffs:

- Node Fleet Manager.
- Node Detail.
- Resource Sharing Rules.
- Access Rules.

### Holds And Eligibility

Content:

- Active holds.
- Hold type.
- Safe reason code.
- Source service refs.
- Dispute window.
- Challenge or verification requirement.
- Compliance requirement.
- Release condition.
- Appeal path.

Links and handoffs:

- Disputes And Appeals.
- Security And Compliance Reviews.
- Central AI Stewardship where stewardship review is involved.

### Payout Batches

Content:

- Batch list.
- Payout item count.
- Period.
- Included and excluded items.
- Batch state.
- Overbill/payment-provider refs.
- Submitted, paid, failed, reversed, or corrected markers.
- Retry or support path where allowed.

### Payout Destination And Compliance

Content:

- Tokenized payout destination refs.
- Destination verification state.
- Missing setup requirements.
- Tax/compliance refs where visible.
- Overvault-backed sensitive refs marker.
- Last destination update audit ref.

Links and handoffs:

- Overvault Secure Storage Center.
- Settings And Security.
- Security And Compliance Reviews.

### Disputes, Corrections, And Reversals

Content:

- Open payout disputes.
- Correction refs.
- Reversal refs.
- Chargeback-affected items.
- Failed payout retries.
- Evidence summary by audience.
- Open dispute action.

Links and handoffs:

- Disputes And Appeals.
- Activity And Receipts Timeline.

### Statements And Exports

Content:

- Statement period selector.
- Export format selector.
- Redaction profile.
- Statement job state.
- Compliance export eligibility.
- Download/open refs where allowed.

## Primary Actions

- Review payout period.
- Open payout batch.
- Export statement.
- Open dispute.
- Set up payout destination.
- View node contribution.

## Secondary Actions

- Filter by node.
- Filter by payout state.
- Copy payout refs.
- View hold reason.
- Open activity refs.
- Ask AI to explain payout status.
- Refresh provider projections.

## States

- No provider account.
- No joined nodes.
- Loading.
- Live.
- Closed period.
- In dispute window.
- Held.
- Eligible.
- Batched.
- Submitted.
- Paid.
- Failed.
- Reversed.
- Corrected.
- Compliance required.
- Destination setup required.
- Permission denied.
- Partial owner-service outage.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Provider payout views must show only the current provider's authorized payout, node, and earning data.
- Raw bank, card, tax, identity, payment-token, fraud heuristic, and other provider-private data must not be displayed in Overdesk.
- Hold explanations should use safe reason-code summaries and appeal refs instead of exposing sensitive fraud or trust internals.
- Earnings and payout projections must cite owner-service refs and must never look like final ledger or payment truth when stale.
- Provider Payout Service, Overbill, Seal Ledger, ORU Account Service, Overmeter, Overclaim, Oververify, Overguard, Overvault, and payment-provider integrations own authoritative payout state; Overdesk owns display, filters, and signed action drafts.

## Design Notes

- Use a period-based dashboard with compact summary tiles and tables for payout items and batches.
- Keep held, failed, reversed, and compliance-required states visually clear.
- Keep the payout destination flow one level deeper because it is sensitive.
- Put dispute and appeal actions directly beside affected holds or failed payout items.
- Do not present projected earnings as guaranteed payout.
