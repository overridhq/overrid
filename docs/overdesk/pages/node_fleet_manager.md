# Node Fleet Manager

## Slug

`node-fleet-manager`

## Title

Node Fleet Manager

## Navigation Group

Network Contribution

## Description

Node Fleet Manager is the Overdesk page for managing multiple contributed computers from one desktop product. It gives providers and organizations a dense, operational view of node health, sharing state, access rules, active leases, updates, incidents, benchmarks, usage, earnings, and bulk actions without turning Overdesk into the scheduler or node authority.

## Primary Users

- Resource providers with multiple devices
- University lab admins
- Organization admins
- Institution IT operators
- Server room operators
- GPU rig owners
- Delegated fleet managers
- Support operators with authorized fleet views

## Primary User Goals

- See every contributed node by account, organization, institution, lab, class, office, server room, GPU rig, or cloud/edge group.
- Detect unhealthy, stale, paused, draining, blocked, or out-of-date nodes quickly.
- Compare resource sharing state, access rules, leases, usage, earnings, benchmarks, and incidents.
- Apply bulk sharing or access rule drafts to selected nodes.
- Stage updates and maintenance windows.
- Export redacted support bundles for selected nodes.
- Open a single Node Detail page for deeper inspection.

## Entry Points

- Network Contribution navigation.
- Home Dashboard provider snapshot.
- Add This Computer To Overrid joined state.
- Resource Sharing Rules node scope selector.
- Access Rules node scope selector.
- Provider Earnings And Payouts node contribution breakdown.
- Notifications Center fleet alert.
- Diagnostics And Support Bundles.
- Address bar command: `/nodes`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Fleet visibility state.
- Node count.
- Healthy/unhealthy summary.
- Active lease count.
- Primary action: Add Node.
- Secondary actions: Bulk Rules, Bulk Access, Maintenance, Export.

### Fleet Scope And Filters

Content:

- Personal fleet selector.
- Organization fleet selector.
- Institution/lab fleet selector.
- Server room, classroom, GPU rig, or edge group selector.
- Node tag filter.
- Health filter.
- Sharing state filter.
- Access policy filter.
- Lease state filter.
- Update state filter.
- Incident state filter.
- Benchmark freshness filter.

### Fleet Summary

Content:

- Total nodes.
- Online nodes.
- Offline nodes.
- Paused nodes.
- Draining nodes.
- Active leases.
- Current utilization.
- Benchmark freshness.
- Policy denial count.
- Update required count.
- Open incident count.
- Display-only earning projection.

Links and handoffs:

- Provider Earnings And Payouts.
- Wallet.
- Activity And Receipts Timeline.

### Node List

Content:

- Node name.
- Node owner scope.
- Node group/tag.
- Online/offline state.
- Health state.
- Hardware summary.
- Resource sharing state.
- Access rule state.
- Active lease count.
- Current utilization.
- Earnings projection.
- Benchmark state.
- Update state.
- Incident marker.
- Last seen timestamp.

Links and handoffs:

- Node Detail.
- Resource Sharing Rules.
- Access Rules.

### Health And Lease Monitor

Content:

- Health timeline summary.
- Active leases.
- Lease class.
- Lease start/end.
- Scheduler denial summary.
- Execution failure summary.
- Overwatch alert markers.
- Overrun state where visible.
- Drain readiness.
- Local service outage marker.

Links and handoffs:

- Node Detail.
- Activity And Receipts Timeline.
- Disputes And Appeals.

### Bulk Actions

Content:

- Selected node count.
- Apply sharing template.
- Apply access template.
- Pause selected.
- Resume selected where allowed.
- Drain selected.
- Stage update.
- Set maintenance window.
- Add or remove tags.
- Export selected support bundles.
- Preview result before submit.
- Per-node failure handling.

Links and handoffs:

- Resource Sharing Rules.
- Access Rules.
- Diagnostics And Support Bundles.

### Rule And Access Drift

Content:

- Nodes with inconsistent sharing rules.
- Nodes with inconsistent access rules.
- Nodes missing required purpose tags.
- Nodes with stale private UUID grants.
- Nodes with expired access grants.
- Nodes blocked by policy prerequisites.
- Suggested review actions.

Links and handoffs:

- Resource Sharing Rules.
- Access Rules.
- Security And Compliance Reviews.

### Updates And Maintenance

Content:

- Overcell version.
- Node Installer version.
- Local service version.
- Update availability.
- Staged update state.
- Maintenance window.
- Failed update marker.
- Rollback or support handoff where allowed.
- Required restart marker.

Links and handoffs:

- Local Device Settings.
- Updates And Release Notes.
- Diagnostics And Support Bundles.

### Incidents And Support

Content:

- Open node incidents.
- Suspicious workload reports.
- Support bundle jobs.
- Dispute refs.
- Owner-service outage refs.
- Compliance/security review refs.
- Last support export state.

Links and handoffs:

- Disputes And Appeals.
- Diagnostics And Support Bundles.
- Security And Compliance Reviews.

## Primary Actions

- Add node.
- Open node detail.
- Apply bulk sharing rules.
- Apply bulk access rules.
- Pause selected nodes.
- Drain selected nodes.
- Stage update.
- Set maintenance window.
- Export support bundles.

## Secondary Actions

- Filter nodes.
- Group nodes.
- Add tags.
- Copy node refs.
- Refresh fleet state.
- View provider earnings.
- Open activity refs.
- Ask AI to summarize fleet health.

## States

- Empty fleet.
- Loading.
- Live.
- Partial fleet visibility.
- Online.
- Offline.
- Stale heartbeat.
- Paused.
- Draining.
- Update required.
- Maintenance scheduled.
- Benchmark stale.
- Policy blocked.
- Incident open.
- Bulk action preview.
- Bulk action running.
- Bulk action partially failed.
- Permission denied.
- Partial owner-service outage.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Node Fleet Manager displays fleet projections and drafts signed bulk actions, but authoritative node, scheduling, execution, health, usage, and earning truth belongs to owner services.
- Node Installer, Overcell, Hardware Discovery, Overregistry, Oververify, Overguard, Oversched, Overlease, Overrun, Overmeter, Overwatch, and Provider Payout Service own authoritative node state.
- Bulk actions must show affected node count, affected scopes, policy preview, failure handling, and owner-service refs before submit.
- Hardware details, local service details, and support bundles must be redacted according to viewer role.
- Visitor, workload, fraud, and requester internals must not be exposed through fleet summaries.

## Design Notes

- Use a dense table as the primary surface with stable columns, compact status chips, and a persistent details drawer.
- Keep bulk actions disabled until a valid node selection exists.
- Show rule/access drift as actionable warnings, not as a separate operational maze.
- Make Node Detail the single-click path from every row for deep inspection.
