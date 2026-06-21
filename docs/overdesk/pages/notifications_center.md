# Notifications Center

## Slug

`notifications-center`

## Title

Notifications Center

## Navigation Group

Home And Fast Access

## Description

Notifications Center is the unified page for user-visible alerts, messages, reminders, system prompts, wallet events, node events, deployment events, disputes, grants, governance assignments, and security prompts. It must help users act on what matters without becoming an addictive feed.

## Primary Users

- Regular users
- Resource providers
- App owners
- Builders
- Institution users
- Stewards

## Primary User Goals

- See what needs attention.
- Separate urgent action from normal updates.
- Open the right owner page quickly.
- Mute, snooze, archive, or configure notification classes.
- Review security and high-risk prompts safely.

## Entry Points

- Bell/notification control in global shell.
- Home Dashboard recent activity and alerts.
- Command palette: `notifications`.
- Push or local desktop notification click.
- Settings And Security notification preferences.

## Sections To Have

### Notification Header

Content:

- Page title.
- Unread count.
- Critical count.
- Active account/scope.
- Global mark-read action.
- Notification settings link.

### Priority Inbox

Content:

- Security prompts.
- Payment/credit issues.
- Node emergency events.
- Deployment failures.
- Dispute deadlines.
- Governance review assignments.
- Permission changes requiring review.

### Notification Feed

Content:

- Grouped notifications by time and source.
- Source icon.
- Severity.
- Read/unread state.
- Action required marker.
- Short message.
- Owner page link.
- Snooze/mute/archive controls.

### Source Filters

Content:

- Messages.
- Wallet.
- Nodes.
- Apps.
- Deployments.
- Assets.
- Permissions.
- Grants.
- Disputes.
- Governance.
- System.

### Notification Detail Panel

Content:

- Full notification text.
- Source service.
- Affected account, app, node, route, receipt, grant, dispute, or review.
- Audit/replay refs where available.
- Available actions.
- Privacy and redaction status.

### Preferences Shortcut Panel

Content:

- Quiet hours.
- Delivery channels.
- Per-source mute.
- Severity thresholds.
- Desktop push settings.
- Digest options.

Links and handoffs:

- Settings And Security.
- Privacy And Permissions Center.

## Primary Actions

- Open notification target.
- Mark as read.
- Snooze.
- Mute source.
- Archive.
- Review required action.

## Secondary Actions

- Open replay.
- Export notification receipt where allowed.
- Report suspicious notification.
- Change notification preferences.
- Clear read notifications.

## States

- Empty.
- Loading.
- Live.
- Offline cached.
- Permission-restricted.
- Partial source outage.
- All caught up.
- Critical action required.

## Permissions And Privacy Behavior

- Notifications must be redacted by viewer role.
- Desktop notifications must avoid sensitive content by default.
- Security, fraud, and compliance notifications must show safe categories, not hidden internals.
- Notification actions must route through owner services.
- Muting must not hide legally or security-required notices unless policy allows it.

## Design Notes

- The page should prioritize actionability over volume.
- Critical items should be distinct but not manipulative.
- Feed grouping should reduce noise.
- Users should be able to clean the inbox quickly.
