# Resource Sharing Rules

## Slug

`resource-sharing-rules`

## Title

Resource Sharing Rules

## Navigation Group

Network Contribution

## Description

Resource Sharing Rules is the Overdesk surface for deciding when and how much of a computer, server, lab machine, GPU rig, or organization node can be used by Overrid. It must make provider control obvious: schedules, resource percentages, safety caps, workload classes, pause/drain controls, and usage/earning projections are all visible before a signed rule update is submitted.

## Primary Users

- Individual device owners
- Resource providers
- University lab admins
- Organization resource managers
- Institution IT operators
- Delegated device managers
- Support operators helping a provider

## Primary User Goals

- Enable or disable resource sharing for one node or a selected node group.
- Define day, night, exact-hour, date-range, and recurring sharing windows.
- Set CPU, GPU, RAM, storage, network, I/O, concurrency, thermal, battery, and idle-only limits.
- Allow or deny workload classes without needing to understand scheduler internals.
- Keep the local device usable by pausing on battery, calls, meetings, active screen use, selected local apps, metered network, or high temperature.
- Preview policy effects before saving.
- See display-only usage and earning projections.
- Pause immediately, stop accepting new leases, drain active leases, or report suspicious workloads.

## Entry Points

- Network Contribution navigation.
- Add This Computer To Overrid first sharing rule step.
- Home Dashboard This Computer card.
- Node Fleet Manager bulk rules action.
- Node Detail sharing summary.
- Provider Earnings And Payouts node contribution breakdown.
- Activity And Receipts Timeline resource-rule event.
- Notifications Center policy-denial or safety alert.
- Address bar command: `/resource-rules`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account/scope.
- Selected node or node group.
- Sharing state.
- Unsaved draft marker.
- Last policy preview state.
- Primary action: Save Rules.
- Secondary actions: Preview Policy, Pause Now, Drain Leases, View Activity.

### Node Scope Selector

Content:

- Current computer selector.
- Personal node selector.
- Organization node group selector.
- Institution/lab node group selector.
- Server room, GPU rig, classroom, or cloud/edge group selector.
- Current rule template marker.
- Rule drift marker when selected nodes do not share identical rules.
- Selected node count.

Links and handoffs:

- Node Fleet Manager.
- Node Detail.
- Add This Computer To Overrid.

### Master Sharing State

Content:

- Master enable/disable switch.
- Accept new leases toggle.
- Drain existing leases toggle.
- Paused state and pause reason.
- Emergency stop state.
- Owner-service acknowledgement state.
- Last saved rule ref.
- Effective rule summary.

### Schedule Editor

Content:

- Day/night preset controls.
- Specific hour windows.
- Weekday/weekend controls.
- Date range controls.
- Holiday or blackout day controls.
- One-time sharing window.
- Recurring sharing window.
- Maintenance window overlay.
- Time zone marker.
- Conflict and gap warnings.

### Resource Caps

Content:

- CPU percentage.
- GPU percentage.
- RAM cap.
- Storage cap.
- Network bandwidth cap.
- I/O cap.
- Concurrent lease count.
- Idle-only threshold.
- Thermal cap.
- Battery cap.
- Minimum local-reserve values.
- Warning when caps make the node unusable for selected workload classes.

### Device Safety Rules

Content:

- Pause when on battery.
- Pause when screen active.
- Pause while selected local apps are running.
- Pause during calls or meetings.
- Pause on high fan or temperature.
- Pause on metered network.
- Pause when local user activity crosses a threshold.
- Resume behavior after safety pause.
- Local safety event history.

Links and handoffs:

- Local Device Settings.
- Diagnostics And Support Bundles.

### Workload Class Controls

Content:

- Allowed workload classes.
- Denied workload classes.
- Public low-sensitivity work toggle.
- Institution/private work toggle.
- AI/model jobs allowed/denied.
- Storage jobs allowed/denied.
- Bandwidth-only jobs allowed/denied.
- RAG/index jobs allowed/denied.
- Compliance-bound workload marker.
- Explanation of why a class is blocked.

Links and handoffs:

- Access Rules.
- Security And Compliance Reviews.
- Grants And Public-Interest Projects.

### Usage And Earnings Preview

Content:

- Projected usage dimensions.
- Display-only earnings projection.
- Estimated utilization by schedule.
- Expected ORU dimensions.
- Resource class breakdown.
- Payout-hold warning where applicable.
- Source refs from Overmeter, Overmark, ORU Account Service, Seal Ledger, and Provider Payout Service.

Links and handoffs:

- Wallet.
- Provider Earnings And Payouts.
- Activity And Receipts Timeline.

### Policy Preview

Content:

- Current draft result.
- Allowed and denied workload examples.
- Missing prerequisite reason codes.
- Access-rule interaction summary.
- Scheduler feasibility marker.
- Verification requirement marker.
- Effective rule diff against current saved rule.
- Owner-service refs for preview output.

Links and handoffs:

- Access Rules.
- Node Detail.
- Security And Compliance Reviews.

### Emergency Controls

Content:

- Pause now.
- Stop accepting new leases.
- Drain active leases.
- Emergency stop.
- Report suspicious workload.
- Open dispute.
- Export support bundle.
- Current lease impact preview.
- Confirmation dialog for high-impact controls.

Links and handoffs:

- Disputes And Appeals.
- Diagnostics And Support Bundles.
- Activity And Receipts Timeline.

### Rule History

Content:

- Saved rule versions.
- Draft timestamps.
- Actor refs.
- Policy preview refs.
- Approval or denial refs.
- Pause/drain history.
- Safety override history.
- Replay refs.

## Primary Actions

- Save sharing rules.
- Preview policy.
- Enable sharing.
- Disable sharing.
- Pause now.
- Drain leases.
- Stop accepting new leases.
- Report suspicious workload.

## Secondary Actions

- Select node group.
- Apply rule template.
- Copy rule from another node.
- Reset draft.
- Open node detail.
- View usage and earning projections.
- Export redacted support bundle.
- Ask AI to explain the rule effect.

## States

- No joined node.
- Loading.
- Live.
- Enabled.
- Disabled.
- Draft with unsaved changes.
- Policy preview running.
- Policy preview failed.
- Rule save pending.
- Rule accepted.
- Rule denied.
- Paused.
- Draining.
- Emergency stopped.
- Safety paused.
- Partial owner-service outage.
- Permission denied.
- Offline cached view.
- Error with retry.

## Permissions And Privacy Behavior

- Resource Sharing Rules may store local drafts and submit signed rule updates, but it must not schedule workloads locally.
- Overguard, Oversched, Overlease, Overcell, Overmeter, and Oververify own admission, placement, lease, measurement, and verification truth.
- Earning and usage projections must be marked as projections until owner-service refs confirm them.
- Local app names, meeting state, battery state, and activity signals must be minimized and shown only to authorized local/device managers.
- Support bundles must be redacted by default and reviewed before export.
- High-impact actions such as emergency stop and draining leases require clear confirmation with affected node, leases, and owner-service refs.

## Design Notes

- Use a split layout: rule controls on the left, live policy/usage preview on the right.
- Use toggles for binary safety rules, sliders or numeric inputs for caps, calendars/time grids for schedules, and segmented controls for workload class modes.
- Keep pause/drain/emergency controls visually distinct from normal editing.
- Show the effective saved rule separately from the current draft so providers never confuse preview with live policy.
