# Disputes And Appeals

## Slug

`disputes-appeals`

## Title

Disputes And Appeals

## Navigation Group

Wallet, Credits, And Ownership

## Description

Disputes And Appeals is the Overdesk surface for usage disputes, asset disputes, namespace disputes, moderation appeals, payout holds, provider complaints, fraud reports, grant disputes, incident-linked appeals, correction proposals, refund proposals, and finality markers. It should give users a clear evidence path without rewriting history or exposing private internals.

## Primary Users

- Regular users
- Providers
- App owners
- Organization admins
- Institution admins
- Moderators and support operators
- Stewards and reviewers
- Affected parties in claims

## Primary User Goals

- See all disputes and appeals in one place.
- Open a new dispute from a receipt, payout, listing, asset, namespace, app, grant, or incident ref.
- Understand deadlines, parties, evidence, holds, and requested remedy.
- Attach evidence safely.
- Track owner-service responses and resolution state.
- Appeal, withdraw, or export a claim where allowed.
- See finality and correction paths clearly.

## Entry Points

- Wallet receipt dispute action.
- Provider Earnings And Payouts hold or failed payout.
- Overasset Assets disputed right.
- Directory Listings report or moderation state.
- Social Photo/Video moderation appeal.
- Namespace Manager conflict.
- Grants And Public-Interest Projects denial or revocation.
- Incident Reports.
- Activity And Receipts Timeline.
- Address bar command: `/disputes`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Case count.
- Urgent deadline marker.
- Redaction state.
- Primary action: Open Dispute.
- Secondary actions: Evidence Uploads, Exports, Filters.

### Case List

Content:

- Claim title.
- Claim type.
- Affected object refs.
- State.
- Deadline.
- Hold/effect marker.
- Requested remedy.
- Last owner-service update.
- Visibility/redaction marker.

Links and handoffs:

- Wallet.
- Provider Earnings And Payouts.
- Overasset Assets.
- Namespace Manager.
- Incident Reports.

### Case Detail

Content:

- Claim id.
- Claim type.
- Parties.
- Affected refs.
- Requested remedy.
- Severity.
- Current state.
- Deadlines and windows.
- Policy version refs.
- Owner-service refs.
- Downstream action refs.
- Finality marker.

### Evidence Panel

Content:

- Evidence links.
- Source service refs.
- Evidence kind.
- Visibility class.
- Redaction profile.
- Completeness status.
- Add evidence action.
- Party statements.
- Export-safe summary.

Links and handoffs:

- Overvault Secure Storage Center.
- Activity And Receipts Timeline.

### Timeline And Windows

Content:

- Intake event.
- Hold request/acknowledgement.
- Challenge window.
- Evidence window.
- Review state.
- Resolution proposal.
- Appeal window.
- Finality state.
- Owner-service messages.

### Create Dispute Draft

Content:

- Dispute type.
- Affected object picker.
- Requested remedy.
- Initial evidence refs.
- Party visibility preview.
- Claim window validation.
- Sensitive-data warning.
- Submit confirmation.

### Holds, Refunds, And Corrections

Content:

- Hold requests.
- Hold status refs.
- Refund proposal refs.
- Correction proposal refs.
- Release requests.
- Downstream accounting or owner-service response.
- Effect denied reason refs.

Links and handoffs:

- Wallet.
- Provider Earnings And Payouts.
- Activity And Receipts Timeline.

### Appeals And Finality

Content:

- Resolution record.
- Appeal eligibility.
- Appeal reason.
- New evidence refs.
- Appeal deadline.
- Finality marker.
- Withdraw action where policy allows.
- Export claim action.

## Primary Actions

- Open dispute.
- Open case.
- Add evidence.
- Submit statement.
- Request hold.
- Appeal resolution.
- Export case.

## Secondary Actions

- Withdraw claim.
- Filter cases.
- Copy claim ref.
- Open affected object.
- Ask AI to summarize public-safe case data.
- Report missing evidence.
- Open owner-service detail.

## States

- Empty case list.
- Loading.
- Live.
- Draft.
- Submitted.
- Triaging.
- Rejected.
- Evidence open.
- Hold requested.
- Held.
- Under review.
- Challenge window.
- Resolution proposed.
- Resolved.
- Appealed.
- Final.
- Withdrawn.
- Expired.
- Permission denied.
- Partial owner-service outage.
- Offline draft only.

## Permissions And Privacy Behavior

- Evidence visibility is separate from claim visibility; users may see reason codes without raw private evidence.
- Overdesk must not display raw workload payloads, private tenant data, provider-private capacity facts, fraud heuristics, vault secrets, payment secrets, or unrelated-party information.
- Dispute actions must preserve trace ids, source refs, policy refs, idempotency, and redaction profiles.
- Accounting effects must remain append-only through owner services; Overdesk must not mutate balances, rewrite receipts, or edit historical events.
- Overclaim owns claim coordination, evidence links, holds, appeals, and finality records; affected owner services own their own downstream corrections, refunds, releases, namespace actions, moderation actions, or payout changes.

## Design Notes

- Use a case inbox with a strong deadline column and a detail panel.
- Keep evidence, timeline, and remedy sections as tabs in the case detail.
- Make redaction and party visibility visible before evidence submission.
- Put finality states in plain language so users know when a case can no longer change.
- Do not turn disputes into a generic support inbox; each case must have affected refs and requested remedy.
