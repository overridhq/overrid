# Owned Apps

## Slug

`owned-apps`

## Title

Owned Apps

## Navigation Group

Apps, Deployment, And Operations

## Description

Owned Apps is the Overdesk dashboard for people and organizations that own, operate, or help maintain Overrid apps. It shows app inventory, namespace routes, release state, deployment health, credit usage, credit earnings, resource costs, visitor/source-safe analytics, incidents, disputes, grants, app assets, and operational handoffs while leaving app, deployment, metering, accounting, and visitor truth with the owner services.

## Primary Users

- App owners
- Organization admins
- Institution app operators
- Developers
- Support operators with authorized app views
- Grant/project managers
- Stewards reviewing public-interest app evidence
- Delegated app managers

## Primary User Goals

- See every app owned or delegated to the active account/scope.
- Understand deployment state, route state, release version, health, incidents, and support needs.
- Track credit usage, credit earnings, resource costs, grants, and wallet receipts.
- Review privacy-safe analytics without exposing raw visitor identity.
- Open app detail, deployment, release, incidents, assets, wallet, or developer handoffs.
- Export privacy-safe analytics or app statements.
- Detect apps that need a release, rollback, policy review, namespace fix, grant update, or support response.

## Entry Points

- Apps, Deployment, And Operations navigation.
- Home Dashboard app owner summary.
- Native App Catalog owned/developer view.
- Deploy New App completion state.
- Wallet app-usage breakdown.
- Overasset Assets app-bound asset.
- Grants And Public-Interest Projects project app link.
- App Incidents And Support.
- Address bar command: `/owned-apps`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Owner visibility state.
- Owned app count.
- Deployment alert count.
- Incident count.
- Current usage period.
- Primary action: Deploy New App.
- Secondary actions: Export Analytics, Developer Console, Release Manager.

### Owner Scope Selector

Content:

- Personal owner account.
- Organization owner account.
- Institution owner account.
- Delegated owner account.
- Grant/project-linked owner scope.
- App-owner role marker.
- Visibility and permission marker.

Links and handoffs:

- Identity And Profile Center.
- Wallet.
- Grants And Public-Interest Projects.

### App Inventory

Content:

- App name.
- Namespace route.
- Owner account/scope.
- Environment.
- Current version.
- Release state.
- Deployment state.
- Health state.
- Usage summary.
- Earnings summary.
- Cost summary.
- Visitor/source-safe analytics marker.
- Incident/dispute marker.
- Asset/grant marker.
- Last release timestamp.

Links and handoffs:

- App Detail.
- Overrid Browser.
- Native App Catalog.

### Filters And Grouping

Content:

- Owner scope filter.
- Organization/institution filter.
- Namespace filter.
- Environment filter.
- Release state filter.
- Deployment state filter.
- Health state filter.
- Risk class filter.
- Grant/public-interest filter.
- Incident/dispute filter.
- Asset-bound filter.

### Performance And Usage Summary

Content:

- App opens/sessions summary.
- Resource usage by dimension.
- Compute, GPU, RAM, storage, bandwidth, and model-route cost summaries.
- Usage trend.
- Budget or grant consumption.
- Overmeter refs.
- Owner-service usage refs.
- Stale or partial analytics marker.

Links and handoffs:

- Wallet.
- Grants And Public-Interest Projects.
- Activity And Receipts Timeline.

### Earnings, Costs, And Receipts

Content:

- Credit earnings.
- Credit costs.
- Net projection.
- Wallet receipt refs.
- ORU dimensions.
- Seal Ledger checkpoint refs where visible.
- Sponsored usage/grant refs.
- Refund, correction, hold, or dispute marker.
- Statement export shortcut.

Links and handoffs:

- Wallet.
- Provider Earnings And Payouts where app also contributes resources.
- Disputes And Appeals.

### Deployment And Release Health

Content:

- Deployment state.
- Active deployment targets.
- Current release version.
- Package refs.
- Rollout state.
- Health checks.
- Route/namespace binding state.
- Backup/rollback readiness.
- Failed deployment marker.
- Pending review or policy blocker.

Links and handoffs:

- Deploy New App.
- Release And Rollback Manager.
- Developer Console.
- App Detail.

### Analytics And Referrals

Content:

- Visitor/source-safe summary.
- Traffic source categories.
- Coarse region where policy allows.
- Search referrals.
- Directory referrals.
- Social referrals.
- App-to-app handoffs.
- Conversion or action summaries where allowed.
- Redaction and privacy markers.
- Analytics export job state.

Links and handoffs:

- Global Search.
- Directory Listings.
- Social Photo/Video.
- Maps And Navigation.

### Incidents, Support, And Disputes

Content:

- Open app incidents.
- Support inbox summary.
- Abuse report summary.
- Policy or compliance review marker.
- Dispute refs.
- Correction refs.
- User-visible status marker.
- Support bundle/replay refs.

Links and handoffs:

- App Incidents And Support.
- Disputes And Appeals.
- Security And Compliance Reviews.
- Activity And Receipts Timeline.

### Assets, Grants, And Permissions

Content:

- App-bound Overasset rights.
- Namespace rights.
- Storage/resource rights.
- Grant-linked rights.
- Delegated operators.
- Expiring permissions.
- Vault grant marker.
- RAG/index permission marker where the app uses private context.
- Policy review marker.

Links and handoffs:

- Overasset Assets.
- Namespace Manager.
- Privacy And Permissions Center.
- Overvault Secure Storage Center.
- Docdex And RAG Index Manager.

## Primary Actions

- Deploy new app.
- Open app detail.
- Open release manager.
- Open developer console.
- Export analytics.
- Export app statement.
- Open incident.
- Open wallet usage.

## Secondary Actions

- Filter apps.
- Group apps.
- Copy namespace route.
- Open in Overrid Browser.
- View assets.
- View grants.
- Refresh app projections.
- Ask AI to summarize app performance.

## States

- No owned apps.
- Loading.
- Live.
- Partial owner-service outage.
- Analytics stale.
- Deployment healthy.
- Deployment degraded.
- Release pending.
- Rollback available.
- Policy blocked.
- Incident open.
- Dispute open.
- Grant active.
- Grant exhausted.
- Permission denied.
- Offline cached view.
- Export queued.
- Error with retry.

## Permissions And Privacy Behavior

- Owned Apps displays app-owner projections and drafts handoffs, but owner services own app records, deployment truth, metering truth, visitor identity truth, accounting truth, grants, incidents, and disputes.
- Deployment Planner, Overpack, Package Validator, Release Strategy Service, Overregistry, Overmeter, Wallet, ORU Account Service, Seal Ledger, Overclaim, Overasset, Universal Namespace Service, Overvault, and owner services own the authoritative records behind this page.
- Visitor analytics must be source-safe, aggregated, and redacted. Raw visitor identity, raw private content, raw RAG context, and precise location trails must not be exposed.
- Export actions must show redaction profile, account/scope, time window, included data classes, and owner-service refs before the export runs.
- Offline mode may show cached app summaries but must not allow release, deployment, policy, wallet, or permission-changing actions without online revalidation.

## Design Notes

- Use a dense app table with a persistent app summary drawer.
- Keep deployment, release, usage, earnings, and incident state in separate compact columns so app owners can scan quickly.
- Use tabs or segmented filters for Owned, Delegated, Grant-Linked, Needs Attention, and Archived where supported.
- Make Deploy New App and App Detail the primary forward paths; keep analytics exports and incident handling available but not visually dominant.
