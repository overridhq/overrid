# Activity And Receipts Timeline

## Slug

`activity-receipts-timeline`

## Title

Activity And Receipts Timeline

## Navigation Group

Home And Fast Access

## Description

Activity And Receipts Timeline is the replayable, role-redacted history of important Overrid actions. It gives users one place to inspect receipts, usage events, permission changes, node activity, app activity, deployments, disputes, payouts, grants, governance actions, support exports, and security prompts.

## Primary Users

- Regular users
- Resource providers
- App owners
- Builders
- Institution users
- Stewards
- Support users with explicit permission

## Primary User Goals

- Understand what happened and when.
- Find receipts and replay refs.
- Filter activity by account, app, node, service, time, type, and risk.
- Export user-visible records.
- Open disputes or corrections from activity items.
- Verify that sensitive actions have audit trails.

## Entry Points

- Home Dashboard recent activity.
- Wallet receipts.
- Notifications Center.
- Provider Earnings And Payouts.
- Deploy New App.
- Release And Rollback Manager.
- Governance Center.
- Command palette: `activity`.

## Sections To Have

### Timeline Header

Content:

- Page title.
- Active account/scope.
- Time range selector.
- Export action.
- Filter summary.
- Live/stale/offline indicator.

### Filter Bar

Content:

- Time range.
- Account/scope.
- Type.
- Source service.
- App.
- Node.
- Namespace.
- Asset.
- Receipt class.
- Risk class.
- Dispute state.

### Timeline Feed

Content:

- Chronological grouped events.
- Event title.
- Source service.
- Affected object.
- Amount/usage summary where applicable.
- Policy/audit/replay markers.
- Status.
- Open detail action.

### Receipt Detail Panel

Content:

- Receipt id.
- Event id.
- Source refs.
- Usage refs.
- Policy refs.
- Audit refs.
- Replay refs.
- ORU/Seal Ledger/Overbill refs where relevant.
- Redaction class.
- Export eligibility.

### Replay And Evidence Panel

Content:

- Safe replay summary.
- Owner-service decision refs.
- Denial reason codes.
- Correction refs.
- Dispute/appeal links.
- Support bundle links where allowed.

### Export Panel

Content:

- Export type.
- Time range.
- Redaction level.
- Included sources.
- File format options.
- Confirmation state.

## Primary Actions

- Filter timeline.
- Open receipt detail.
- Open replay.
- Export records.
- Open dispute or correction.

## Secondary Actions

- Copy receipt id.
- Open source page.
- Save filter.
- Compare related events.
- Report suspicious activity.

## States

- Loading.
- Empty timeline.
- Live.
- Stale.
- Offline cached.
- Partial source outage.
- Permission-restricted.
- Export preparing.
- Export ready.
- Export failed.

## Permissions And Privacy Behavior

- Timeline content must be redacted by viewer role.
- Raw private content, secrets, exact location trails, raw RAG context, fraud internals, and provider-sensitive internals must not appear.
- Export must show redaction level and included sources before creation.
- Dispute and correction actions must route to owning services.
- Offline cached timeline must be marked as stale and read-only.

## Design Notes

- Timeline scanning should be fast, with strong filters.
- The detail panel should make receipts understandable without exposing internal service complexity.
- Export should feel deliberate because it can include sensitive history.
- Events from different services should use consistent type labels.
