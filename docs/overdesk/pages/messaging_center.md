# Messaging Center

## Slug

`messaging-center`

## Title

Messaging Center

## Navigation Group

Daily Apps

## Description

Messaging Center is the unified username-addressed communication page for Overdesk. It replaces scattered email, phone, and chat identifiers with Overrid identities, organization inboxes, app inboxes, service notifications, and direct username messaging. It must be fast for daily conversation while still making identity, privacy, attachments, AI triage, and organization routing clear.

## Primary Users

- Regular users
- Organization members
- App owners
- Support teams
- Institution users
- Stewards handling official inboxes

## Primary User Goals

- Send a message directly to a username, organization, app, or service inbox.
- Read and respond to personal and organization threads.
- Search, archive, mute, block, report, or assign messages.
- Understand identity, scope, and delivery state.
- Manage attachments and sensitive content safely.
- Use AI triage only with explicit permission.

## Entry Points

- Daily Apps navigation.
- Home Dashboard fast app shortcut.
- Notifications Center.
- Global Search result.
- Directory listing contact action.
- App support action.
- Address bar command: `/messages`.

## Sections To Have

### Page Header

Content:

- Page title.
- Active account and inbox scope.
- Online/sync state.
- Primary action: New Message.
- Secondary actions: Search Messages, Inbox Settings, AI Triage.

### Inbox Switcher

Content:

- Personal inboxes.
- Organization inboxes.
- Institution inboxes.
- App-owner inboxes.
- Service/system inboxes.
- Delegated inboxes.
- Unread and urgent counts per inbox.

Links and handoffs:

- Identity And Profile Center.
- Settings And Security.
- Privacy And Permissions Center.

### Thread List

Content:

- Sender or channel identity.
- Verified/unknown marker.
- Subject or first message line.
- Last activity.
- Unread count.
- Assignment state.
- Attachment marker.
- Mute/archive/block state.
- Risk or report marker where relevant.

Links and handoffs:

- Global Search.
- Activity And Receipts Timeline.

### Thread View

Content:

- Message history.
- Sender identity details.
- Delivery/read state.
- Reactions where supported.
- Attachments.
- Redaction markers.
- Tombstone/recall markers.
- Thread-level permissions.

Links and handoffs:

- Identity And Profile Center.
- Overvault Secure Storage Center.
- Disputes And Appeals.

### Compose Panel

Content:

- Recipient field with username, organization, app, or service inbox lookup.
- Message body.
- Attachment picker.
- Visibility/sensitivity setting.
- Send account/scope.
- Delivery estimate where usage applies.
- Confirmation state for high-risk or external handoffs.

Links and handoffs:

- Overrid Browser for profile route lookup.
- Wallet for usage/receipt details.
- Overvault for private attachment refs.

### Organization Inbox Panel

Content:

- Assignee.
- Internal notes.
- Role visibility.
- Escalation state.
- SLA or response target where the organization defines one.
- Related app, listing, dispute, or receipt refs.

Links and handoffs:

- Developer Console.
- Owned Apps.
- Directory Listings.
- Disputes And Appeals.

### Search And Filters

Content:

- Text search.
- Sender/recipient filters.
- Inbox filter.
- Attachment filter.
- Date filter.
- Read/unread filter.
- Assigned/unassigned filter.
- Muted/archived/reported filter.

### AI Triage Permission Panel

Content:

- AI triage status.
- Scope being granted.
- Data classes included.
- Redaction rules.
- Expiry.
- Estimated usage.
- Last triage receipt.

Links and handoffs:

- Personal AI Assistant.
- Docdex And RAG Index Manager.
- Privacy And Permissions Center.
- Activity And Receipts Timeline.

### Safety And Reporting Panel

Content:

- Block sender.
- Report abuse.
- Report spam.
- Open dispute.
- View identity/verification details.
- View moderation or appeal state where available.

## Primary Actions

- New Message.
- Reply.
- Search Messages.
- Archive.
- Mute.
- Assign.
- Report.

## Secondary Actions

- Add reaction.
- Forward/share route where policy allows.
- Download attachment.
- Revoke attachment access.
- Start AI triage.
- Export thread where allowed.
- Open receipt/replay.

## States

- Empty inbox.
- Loading.
- Live.
- Syncing.
- Offline cached.
- Stale thread.
- Permission-denied thread.
- Unknown recipient.
- Delivery failed.
- Partial attachment outage.
- Organization role required.

## Permissions And Privacy Behavior

- The page must never reveal private thread content outside the active inbox role.
- AI triage must be opt-in by scope, data class, and expiry.
- Attachments must use owner-service and Overvault permissions rather than local raw storage.
- Block/report actions must route to owner services.
- Organization internal notes must never appear to external participants.
- Desktop notifications must redact message content by default unless the user changes settings.

## Design Notes

- The page should use a three-column desktop pattern: inboxes, thread list, thread detail.
- Compose should be fast but must expose sender scope clearly.
- Organization routing should feel like a normal inbox, not a separate admin tool.
- Spam/report controls should be easy to find without dominating normal conversations.
