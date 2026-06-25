# App Detail

## Slug

`app-detail`

## Title

App Detail

## Navigation Group

Apps, Deployment, And Operations

## Description

App Detail is the focused operational page for one Overrid app. It gives an app owner or delegated operator a complete view of identity, namespace routes, release state, deployments, usage, earnings, costs, visitor/source-safe analytics, permissions, data/storage usage, app-bound assets, grants, incidents, support, disputes, and replay refs without making Overdesk the authority for app, deployment, metering, accounting, or visitor truth.

## Primary Users

- App owners
- Delegated app managers
- Developers
- Organization admins
- Institution app operators
- Support operators with authorized app views
- Grant/project managers
- Stewards reviewing public-interest app evidence

## Primary User Goals

- Understand the current state of one app quickly.
- Open the app route, namespace route, or catalog listing.
- Inspect release, deployment, health, route, and rollback state.
- Track credit usage, credit earnings, resource costs, grants, holds, receipts, and ORU-only monetization compliance.
- Review privacy-safe visitor and referral analytics.
- See permissions, data classes, storage usage, vault grants, and RAG/index grants.
- Respond to incidents, support items, abuse reports, disputes, and policy reviews.
- Export privacy-safe analytics, statements, and replay bundles where allowed.

## Entry Points

- Owned Apps app row.
- Native App Catalog owner/developer action.
- Deploy New App launch monitor.
- Release And Rollback Manager release row.
- Developer Console project row.
- App Incidents And Support case.
- Wallet app usage breakdown.
- Overasset Assets app-bound right.
- Grants And Public-Interest Projects app link.
- Address bar command: `/app-detail`.

## Sections To Have

### Page Header

Content:

- App name.
- Namespace route.
- Active owner scope.
- Environment.
- App state.
- Health state.
- Current release version.
- Incident count.
- Primary action: Open App.
- Secondary actions: Deploy, Release Manager, Developer Console, Export.

### App Identity And Routes

Content:

- App id/ref.
- App display name.
- Owner account/scope.
- Namespace route.
- App route and subroutes.
- Catalog visibility.
- Environment.
- Risk class.
- Package identity.
- Route trust marker.
- Dispute/contact info.

Links and handoffs:

- Overrid Browser.
- Native App Catalog.
- Namespace Manager.

### Health And Runtime Summary

Content:

- Deployment health.
- App session health.
- Route health.
- Error rate summary.
- Latency summary where available.
- Active deployment targets.
- Overwatch alert refs.
- Owner-service health refs.
- Stale or partial health marker.

Links and handoffs:

- App Incidents And Support.
- Activity And Receipts Timeline.

### Usage, Earnings, Costs, And Receipts

Content:

- Usage by ORU dimension.
- Compute, GPU, RAM, storage, bandwidth, model-route, and data-transfer usage.
- Credit earnings.
- Credit costs.
- Net projection.
- Monetization mode and active ORU-only policy version.
- Subscription, in-app purchase, one-time purchase, paid unlock, listing, and service-unit ORU receipts.
- External checkout/payment bypass state.
- Monetization suspension and payout-hold markers.
- Sponsored usage/grant consumption.
- Wallet receipt refs.
- Seal Ledger checkpoint refs where visible.
- Holds, refunds, corrections, and disputes.

Links and handoffs:

- Wallet.
- Grants And Public-Interest Projects.
- Disputes And Appeals.

### Monetization Compliance

Content:

- Accepted publisher terms version.
- Active app monetization policy version.
- ORU-only fee collection attestation.
- Current monetization state: compliant, warning, suspended, payout-held, or appeal-open.
- Declared subscription, in-app purchase, one-time purchase, paid unlock, paid listing, service-unit, and machine-to-machine billing rules.
- Overbill and ORU Account Service refs for paid actions.
- External checkout scan results across manifest, package metadata, catalog content, outbound links, UI declarations, support text, and routes.
- Bypass reason codes for card, bank-transfer, crypto/stablecoin, payment-link, QR-code, external subscription, or private payment collection.
- Required fixes and appeal refs.

Links and handoffs:

- Deploy New App.
- Wallet.
- Security And Compliance Reviews.
- Disputes And Appeals.

### Deployment And Release

Content:

- Current release version.
- Package refs.
- Active deployment plan refs.
- Rollout state.
- Health gate status.
- Backup/restore readiness.
- Rollback point.
- Migration marker.
- Pending reviews.
- Policy blockers.
- Last deployment audit refs.

Links and handoffs:

- Deploy New App.
- Release And Rollback Manager.
- Developer Console.

### Analytics And Referrals

Content:

- Visitor/source-safe summary.
- App opens and sessions.
- Traffic source categories.
- Search referrals.
- Directory referrals.
- Social referrals.
- Maps/place referrals.
- App-to-app handoffs.
- Coarse region where policy allows.
- Conversion/action summary where allowed.
- Redaction profile.

Links and handoffs:

- Global Search.
- Directory Listings.
- Social Photo/Video.
- Maps And Navigation.

### Permissions, Data, And Storage

Content:

- Declared app permissions.
- Data classes.
- Storage usage.
- Workspace/document access refs.
- Vault grant marker.
- RAG/index grant marker.
- Exact location permission marker.
- Messaging/social/media permission markers where applicable.
- Expiring permission warnings.
- Revocation and review paths.

Links and handoffs:

- Privacy And Permissions Center.
- Overvault Secure Storage Center.
- Docdex And RAG Index Manager.
- Workspace.

### Assets, Grants, And Ownership

Content:

- App-bound Overasset rights.
- Namespace rights.
- Storage/resource rights.
- Grant-linked rights.
- Delegated operators.
- Ownership proof refs.
- Expiring rights.
- Disputed rights.
- Transfer or binding restrictions.

Links and handoffs:

- Overasset Assets.
- Grants And Public-Interest Projects.
- Identity And Profile Center.

### Incidents, Support, And Disputes

Content:

- Open incidents.
- Support inbox summary.
- Abuse reports.
- Policy/compliance reviews.
- Dispute refs.
- Public status marker.
- Support bundle refs.
- Replay refs.
- Last communication marker.

Links and handoffs:

- App Incidents And Support.
- Disputes And Appeals.
- Security And Compliance Reviews.

### Activity And Replay

Content:

- Deployment events.
- Release events.
- Route changes.
- Permission grants/revokes.
- Wallet receipts.
- Analytics export jobs.
- Support bundle exports.
- Incident and dispute events.
- Replay refs.

Links and handoffs:

- Activity And Receipts Timeline.

## Primary Actions

- Open app.
- Deploy new version.
- Open release manager.
- Open developer console.
- Export analytics.
- Export app statement.
- Open incident/support.
- Open wallet usage.

## Secondary Actions

- Copy namespace route.
- Open catalog page.
- Refresh projections.
- Review permissions.
- View assets.
- View grants.
- Open dispute.
- Ask AI to summarize app state.

## States

- Loading.
- Live.
- App not found.
- Permission denied.
- Owner scope required.
- Deployment healthy.
- Deployment degraded.
- Release pending.
- Policy blocked.
- Incident open.
- Dispute open.
- Analytics stale.
- Wallet projection stale.
- Grant active.
- Grant exhausted.
- Partial owner-service outage.
- Offline cached view.
- Export queued.
- Error with retry.

## Permissions And Privacy Behavior

- App Detail displays app-owner projections and drafts handoffs; owner services own app records, deployment truth, metering truth, visitor identity truth, accounting truth, grants, incidents, and disputes.
- Visitor analytics must be source-safe and aggregated. Raw visitor identity, raw private content, precise location trails, raw RAG context, vault secrets, payment secrets, and fraud internals must not appear.
- Monetization compliance must show policy state and reason codes, but it must not expose private fraud heuristics, suspicious-transaction reporting status, raw payment-provider secrets, or hidden AML thresholds.
- Permission, vault, and RAG/index sections must show revocation and inspection paths without revealing raw protected content.
- Export actions must show redaction profile, scope, time range, included data classes, and owner-service refs before export.
- Offline cached app detail must be clearly marked and must not allow release, deployment, wallet, permission, or policy changes without online revalidation.

## Design Notes

- Use a tabbed detail layout: Overview, Usage, Release, Analytics, Permissions, Assets, Incidents, and Activity.
- Keep the page header operational: route, version, health, incidents, and primary actions must be visible without scrolling.
- Use compact status chips and tables for repeated operational facts.
- Keep raw refs in drawers so the main page remains readable but audit/replay data is still reachable.
