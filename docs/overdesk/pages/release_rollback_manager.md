# Release And Rollback Manager

## Slug

`release-rollback-manager`

## Title

Release And Rollback Manager

## Navigation Group

Apps, Deployment, And Operations

## Description

Release And Rollback Manager is the Overdesk page for app and service owners to inspect release state, rollout health, deployment graph refs, health gates, backup/restore readiness, rollback points, route state, namespace bindings, migration status, incidents, and recovery handoffs. It must make release operations visible and controlled while leaving release, rollback, deployment, failover, and recovery authority with the owner services.

## Primary Users

- App owners
- Service owners
- Developers
- Release managers
- Organization admins
- Institution app operators
- Support/recovery operators with authorized views
- Stewards reviewing public-interest app changes

## Primary User Goals

- See current and pending releases by app/service/environment.
- Understand rollout strategy, health gates, route state, namespace binding, and deployment status.
- Know whether rollback is safe, blocked, stale, or unavailable.
- Promote, pause, resume, or rollback where authorized through signed action drafts.
- Review backup/restore, failover, migration, and route-shift readiness.
- Open incidents, support bundles, deployment previews, app details, and activity refs.
- See why a release or rollback is blocked in clear reason codes.

## Entry Points

- Apps, Deployment, And Operations navigation.
- Owned Apps deployment and release health.
- App Detail deployment and release section.
- Deploy New App launch monitor.
- Developer Console deployment preview.
- App Incidents And Support incident recovery action.
- Activity And Receipts Timeline release event.
- Address bar command: `/releases`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active owner scope.
- Environment selector.
- Release count.
- Health gate alert count.
- Rollback readiness summary.
- Primary action: New Release.
- Secondary actions: Pause Rollout, Rollback, Export.

### Release List

Content:

- App/service name.
- Environment.
- Version.
- Package ref.
- Rollout strategy.
- Rollout state.
- Health status.
- Incident status.
- Backup/restore readiness.
- Rollback point.
- Route state.
- Namespace binding state.
- Last action timestamp.

Links and handoffs:

- App Detail.
- Deploy New App.
- Developer Console.

### Release Detail

Content:

- Release id/ref.
- App/service refs.
- Version.
- Package refs.
- Deployment plan refs.
- Release Strategy refs.
- Rollout segments.
- Current traffic/route state where visible.
- Health gate history.
- Policy decision refs.
- Audit/replay refs.

Links and handoffs:

- Activity And Receipts Timeline.
- Security And Compliance Reviews.

### Health Gates

Content:

- Health checks.
- Error rate markers.
- Latency markers where available.
- Route health.
- Overwatch alert refs.
- User-impact marker.
- Compliance boundary marker.
- Required review state.
- Gate pass/fail reason codes.
- Retry state.

Links and handoffs:

- App Incidents And Support.
- Incident Reports.

### Rollout Controls

Content:

- Promote request draft.
- Pause rollout request draft.
- Resume rollout request draft.
- Route-shift request draft.
- Segment progression preview.
- Affected routes/apps/users summary as refs or bounded estimates.
- Policy dry-run state.
- Confirmation controls.

### Rollback Readiness

Content:

- Rollback point.
- Previous release.
- Backup/restore readiness.
- Data migration compatibility.
- Route rollback readiness.
- Failover/recovery refs.
- Unsafe rollback reasons.
- Stale backup marker.
- Rollback request draft.
- Post-rollback health gate plan.

Links and handoffs:

- Backup and restore detail where available.
- Failover and recovery detail where available.
- App Incidents And Support.

### Migration And Data Changes

Content:

- Migration refs.
- Data class changes.
- Storage changes.
- Vault permission changes.
- RAG/index changes.
- Compatibility warnings.
- Roll-forward requirements.
- Required approvals.
- Replay refs.

Links and handoffs:

- Overvault Secure Storage Center.
- Docdex And RAG Index Manager.
- Privacy And Permissions Center.

### Incidents, Support, And Recovery

Content:

- Release-linked incidents.
- Failed health gates.
- Rollback-related support items.
- Recovery step refs.
- Support bundle refs.
- Public/private communication marker.
- Post-incident follow-up marker.

Links and handoffs:

- App Incidents And Support.
- Diagnostics And Support Bundles.
- Incident Reports.

## Primary Actions

- Open release detail.
- Promote release.
- Pause rollout.
- Resume rollout.
- Request rollback.
- Request route shift.
- Open deployment plan.
- Export release record.

## Secondary Actions

- Filter releases.
- Compare versions.
- Copy release ref.
- Open app detail.
- Open developer console.
- View backup/restore refs.
- View activity refs.
- Ask AI to explain blocked release.

## States

- Empty release list.
- Loading.
- Live.
- Release pending.
- Rollout running.
- Rollout paused.
- Rollout healthy.
- Health gate failed.
- Rollback ready.
- Rollback blocked.
- Backup stale.
- Migration required.
- Policy blocked.
- Incident open.
- Action awaiting signature.
- Action running.
- Action partially failed.
- Permission denied.
- Partial owner-service outage.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Release And Rollback Manager displays state and drafts signed actions. Release Strategy Service, Deployment Planner, Backup and Restore Service, Failover and Recovery Coordinator, Overmesh, Overwatch, Package Validator, Compliance Boundary Service, and owner services own release and rollback authority.
- Promote, pause, resume, route-shift, and rollback requests must show affected account/scope, app/service, release, route, health gate result, backup/restore state, policy result, and audit refs before signing.
- Raw private payloads, topology internals, vault secrets, exploit details, and protected user data must not appear in release views or exports.
- Stale cached release state must not be presented as authority for rollback or promotion.
- Unsafe rollbacks must show clear reason codes instead of offering a risky action.

## Design Notes

- Use a release table with a right-side detail drawer and clear health/rollback columns.
- Keep rollback controls available but separated from normal promotion controls.
- Use compare views for version changes, permissions, data classes, and migration effects.
- Make blocked states actionable through reason codes and handoffs.
