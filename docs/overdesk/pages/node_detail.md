# Node Detail

## Slug

`node-detail`

## Title

Node Detail

## Navigation Group

Network Contribution

## Description

Node Detail is the Overdesk page for inspecting and operating one contributed node. It brings together identity, registration, hardware capabilities, live health, leases, sharing rules, access rules, benchmark and verification state, earnings, incidents, diagnostics, and support actions for a single computer or server.

## Primary Users

- Node owners
- Resource providers
- University lab admins
- Organization resource managers
- Delegated device managers
- Support operators with authorized node views
- Stewards reviewing node evidence

## Primary User Goals

- Understand whether a node is joined, healthy, usable, paused, draining, blocked, or out of date.
- See hardware capabilities and verification evidence without exposing unnecessary host details.
- Inspect active and recent leases.
- Review the effective sharing and access rules for this node.
- Run or schedule benchmark and verification checks.
- See usage, earnings, holds, and payout handoffs.
- Open incidents, disputes, diagnostics, or redacted support bundles.
- Pause, drain, update, or edit rules from one place.

## Entry Points

- Node Fleet Manager row.
- Resource Sharing Rules node selector.
- Access Rules node selector.
- Home Dashboard This Computer card.
- Provider Earnings And Payouts node contribution breakdown.
- Activity And Receipts Timeline node event.
- Notifications Center node alert.
- Diagnostics And Support Bundles.
- Address bar command: `/node`.

## Sections To Have

### Page Header

Content:

- Page title.
- Node name.
- Active account/scope.
- Node state.
- Health state.
- Current lease count.
- Last seen timestamp.
- Primary action: Manage Rules.
- Secondary actions: Pause, Drain, Run Benchmark, Export Support.

### Identity And Registration

Content:

- Node id/ref.
- Node display name.
- Owner account/scope.
- Institution or organization scope.
- Registration state.
- Overregistry refs.
- Installer refs.
- Local service refs.
- Created/updated timestamps.
- Current Overcell version.
- Signed command/audit refs.

Links and handoffs:

- Identity And Profile Center.
- Node Fleet Manager.
- Activity And Receipts Timeline.

### Hardware And Capabilities

Content:

- CPU summary.
- GPU summary.
- RAM.
- Storage.
- Network class.
- Runtime/sandbox capability.
- GPU runtime support.
- Uptime profile.
- Safety limits.
- Hardware Discovery refs.
- Redacted host-detail marker.

Links and handoffs:

- Local Device Settings.
- Diagnostics And Support Bundles.

### Live Health

Content:

- Online/offline state.
- Heartbeat freshness.
- CPU, GPU, memory, disk, network, and I/O utilization.
- Battery state where relevant.
- Thermal state.
- Local service state.
- Update state.
- Health warnings.
- Overwatch alert refs.
- Degraded capability markers.

### Active And Recent Leases

Content:

- Active lease list.
- Lease class.
- Workload class.
- Start and expected end.
- Resource allocation.
- Scheduler refs.
- Execution refs.
- Measurement refs.
- Verification refs.
- Drain impact.
- Safe requester visibility only.

Links and handoffs:

- Activity And Receipts Timeline.
- Disputes And Appeals.

### Sharing Rules Summary

Content:

- Master sharing state.
- Schedule summary.
- Resource caps.
- Safety rules.
- Workload class allow/deny summary.
- Pause/drain state.
- Last saved rule ref.
- Policy preview shortcut.

Links and handoffs:

- Resource Sharing Rules.

### Access Rules Summary

Content:

- Institution allowlist summary.
- Organization allowlist summary.
- User allowlist summary.
- Masked private UUID grant summary.
- Purpose tag summary.
- Deny rule summary.
- Expiring grant marker.
- Last dry-run ref.

Links and handoffs:

- Access Rules.
- Privacy And Permissions Center.

### Benchmark And Verification

Content:

- Last benchmark result.
- Benchmark freshness.
- Oververify state.
- Challenge-task readiness.
- Reputation/bootstrap state.
- Verification blockers.
- Required retest marker.
- Run benchmark action.
- Schedule verification action where allowed.

Links and handoffs:

- Security And Compliance Reviews.
- Activity And Receipts Timeline.

### Usage, Earnings, And Holds

Content:

- Usage by ORU dimension.
- Resource class usage.
- Current-period earning projection.
- Closed-period earning view.
- Holds and corrections affecting this node.
- Payout eligibility marker.
- Source refs from Overmeter, Overmark, ORU Account Service, Seal Ledger, Overlease, and Provider Payout Service.

Links and handoffs:

- Provider Earnings And Payouts.
- Wallet.

### Events, Incidents, And Diagnostics

Content:

- Node event timeline.
- Policy denials.
- Lease failures.
- Suspicious workload reports.
- Open incidents.
- Dispute refs.
- Update failures.
- Local service errors.
- Redacted diagnostics preview.
- Support bundle export state.

Links and handoffs:

- Diagnostics And Support Bundles.
- Disputes And Appeals.
- App Incidents And Support.

## Primary Actions

- Edit sharing rules.
- Edit access rules.
- Pause node.
- Drain leases.
- Resume where allowed.
- Run benchmark.
- Stage update.
- Export redacted support bundle.
- Open dispute.

## Secondary Actions

- Rename node.
- Add or remove tags.
- Copy node ref.
- Refresh node state.
- View activity refs.
- View provider earnings.
- Ask AI to summarize node health.
- Open local device settings.

## States

- Loading.
- Live.
- Joined.
- Registration pending.
- Registration denied.
- Online.
- Offline.
- Stale heartbeat.
- Healthy.
- Degraded.
- Paused.
- Draining.
- Emergency stopped.
- Benchmark stale.
- Verification required.
- Update required.
- Incident open.
- Policy blocked.
- Permission denied.
- Partial owner-service outage.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Node Detail displays node facts and drafts actions, but Node Installer, Overcell, Hardware Discovery, Overregistry, Oververify, Overguard, Oversched, Overlease, Overrun, Overmeter, Overwatch, and Provider Payout Service own authoritative node truth.
- Raw host details, local logs, private UUIDs, requester details, fraud/risk internals, and support bundles must be redacted by role.
- Pause, drain, update, benchmark, support export, and dispute actions require clear actor, scope, and affected-node confirmation.
- Offline cached node state must be clearly marked and cannot be used as authority for mutating actions.
- Earnings and usage must cite owner-service refs and remain display-only projections unless closed-period refs confirm them.

## Design Notes

- Use a header status band plus tabs for Overview, Leases, Rules, Access, Verification, Earnings, and Events.
- Keep the main overview dense enough for support and provider use, with deeper refs in the contextual drawer.
- Make Pause and Drain visible but not easy to trigger accidentally.
- Put private or raw diagnostic details behind explicit review/export flows.
