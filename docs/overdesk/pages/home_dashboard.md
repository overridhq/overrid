# Home Dashboard

## Slug

`home-dashboard`

## Title

Home Dashboard

## Navigation Group

Home And Fast Access

## Description

The Home Dashboard is the first screen after sign-in. It gives the user a role-aware summary of the account, computer, wallet, native apps, owned apps, assets, recent activity, and urgent actions. It should work as a calm command center, not as a news feed or engagement trap.

## Primary Users

- Regular users
- Resource providers
- App owners
- Builders
- Institution users
- Stewards with assigned reviews

## Primary User Goals

- Understand the current account and scope.
- Jump to the most common Overdesk pages.
- See whether this computer is part of the network.
- See wallet, usage, grant, and permission warnings.
- Resume recent work without hunting through navigation.
- Notice urgent but legitimate security, dispute, payout, deployment, or governance items.

## Entry Points

- Default page after sign-in.
- Home item in primary navigation.
- Command palette: `home`.
- Address bar command: `/home`.
- Return target after completing onboarding, purchase, deployment, or settings flow.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account and scope.
- Live/stale/offline indicator.
- Primary action: Open Overrid Browser.
- Secondary actions: Search, Ask AI, Add This Computer.

### Account Summary

Content:

- Selected identity.
- Current tenant, organization, institution, or delegated scope.
- Wallet summary.
- Active grants.
- Permission alerts.
- Required profile or security actions.

Links and handoffs:

- Wallet.
- Privacy And Permissions Center.
- Identity And Profile Center.
- Grants And Public-Interest Projects.
- Settings And Security.

### This Computer

Content:

- Joined/not joined state.
- Node health.
- Resource sharing state.
- Current active leases count.
- Last benchmark summary.
- Pause/drain status.
- Update required state.

Links and handoffs:

- Add This Computer To Overrid.
- Resource Sharing Rules.
- Access Rules.
- Node Detail.
- Node Fleet Manager.

### Fast Apps

Content:

- Pinned app shortcuts.
- Recently used apps.
- Suggested core apps for the current role.
- Restricted/unavailable markers when a scope cannot open an app.

Links and handoffs:

- Messaging Center.
- Personal AI Assistant.
- Workspace.
- Directory Listings.
- Maps And Navigation.
- Native App Catalog.
- Central AI Stewardship.

### Wallet And Usage Snapshot

Content:

- ORU balance projection by primary dimensions.
- Current month or selected-period usage.
- Holds, disputes, sponsored credits, and grants.
- Credit warning bands.
- Most recent receipts.

Links and handoffs:

- Wallet.
- Buy Credits.
- Activity And Receipts Timeline.
- Disputes And Appeals.

### App Owner Snapshot

Content:

- Owned app count.
- Deployment alerts.
- Credit usage.
- Credit earnings.
- Resource costs.
- Visitor/source-safe summary.
- Active incidents and disputes.

Links and handoffs:

- Owned Apps.
- Deploy New App.
- Release And Rollback Manager.
- App Incidents And Support.

### Asset Snapshot

Content:

- Overasset count.
- Delegated rights.
- Expiring rights.
- Pending asset disputes.
- App/resource ownership refs.

Links and handoffs:

- Overasset Assets.
- Namespace Manager.
- Disputes And Appeals.

### Recent Activity

Content:

- Messages.
- Receipts.
- Resource leases.
- Assistant sessions.
- Workspace edits.
- Listing updates.
- Namespace changes.
- Vault grants.
- Deployment events.
- Governance events.

Links and handoffs:

- Activity And Receipts Timeline.
- Notifications Center.
- Relevant owner page for each item.

## Primary Actions

- Open Overrid Browser.
- Search.
- Ask AI.
- Add This Computer.
- Buy Credits.

## Secondary Actions

- Resume recent item.
- Open wallet receipt.
- Review permission alert.
- Open dispute.
- Open deployment alert.
- Open governance review.

## States

- Loading.
- Empty new account.
- Live.
- Stale.
- Offline-limited.
- Restricted account.
- Partial owner-service outage.
- Security action required.
- Local cache unavailable.

## Permissions And Privacy Behavior

- The page must redact every card by the active viewer role.
- The page must not expose hidden visitor identity, raw private messages, raw AI/RAG context, raw vault data, exact location trails, or fraud internals.
- The page may show counts and summaries only when the owning service returns a permission-safe projection.
- Every warning must link to a clear inspection or correction path.

## Design Notes

- The dashboard should be dense but not noisy.
- The user should be able to reach common daily actions in one click.
- Urgent items must be visually distinct from normal activity, but must not use fake urgency.
- The primary layout should support role-based modules without changing the overall page structure.
