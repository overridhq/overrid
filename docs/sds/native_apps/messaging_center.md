SDS #71

# Messaging Center SDS

## Purpose

Build a username-addressed replacement for fragmented email, phone, and chat identities.

Messaging Center is the native communication utility for Overrid. It owns inboxes, threads, message envelopes, delivery state, organization inbox routing, notification records, attachment refs, encrypted-message refs, spam/abuse refs, AI triage permission records, and usage refs. It is not a social feed, advertising surface, identity authority, storage system, payment service, or global policy judge. Its job is to let people, organizations, apps, and native services communicate through stable Overrid usernames and inbox refs without creating another captive messaging silo.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [messaging_center.md](../../service_catalog/native_apps/messaging_center.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |

## Service Family

- Family: Native applications
- Owning layer: Native public utility application layer for username-addressed messages, org inboxes, app notifications, and communication handoffs
- Primary data scope: inbox refs, thread refs, message envelopes, delivery receipts, notification refs, org routing rules, attachment refs, encrypted payload refs, block/mute/contact preferences, AI triage permissions, spam/abuse reports, usage refs, and audit refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md)

## Problem Statement

Internet communication is split across email addresses, phone numbers, private chat handles, workplace tools, and platform-specific DMs. This fragmentation creates lock-in, spam, identity confusion, weak portability, and excessive dependence on corporate account systems. Overrid needs a native username-addressed communication layer that works across people, organizations, apps, native services, mobile clients, and assistants.

The hard part is boundary discipline. Messaging Center must provide delivery and inbox semantics without becoming the identity system, object store, vault, social graph, ad engine, or fraud authority. It must also support encrypted personal messages, organization inbox roles, app notifications, AI triage with permission, and strong abuse controls without reading private message content unnecessarily.

## Goals

- Define inbox, thread, message envelope, delivery, read, notification, attachment, contact preference, block/mute, AI triage permission, abuse report, and usage records.
- Route messages directly to Overrid usernames, organization inboxes, app/service inboxes, and native app notification targets.
- Support encrypted personal messages where appropriate through Overvault grants and payload refs.
- Support attachments through Overstore refs without making Messaging Center the object store.
- Provide organization inbox routing, role-based access, assignment, and audit refs.
- Provide spam, abuse, impersonation, harassment, unsafe attachment, and unwanted-contact controls.
- Allow Personal AI Assistant triage only under explicit permission and redacted context rules.
- Emit usage and receipt refs to Wallet/Usage Center without hardcoded pricing or revenue assumptions.

## Non-Goals

- Do not become Overpass, Universal Namespace Service, Overkey, Overvault, Overstore, Search Engine, Directory Listings, Social Photo/Video App, ORU Account Service, Seal Ledger, Overclaim, or Fraud Control.
- Do not store raw vault secrets, payment data, ledger truth, unrelated personal profiles, or continuous behavioral tracking data.
- Do not require email, phone, or external chat compatibility in the first build; bridges can be later adapters if policy allows.
- Do not expose encrypted personal messages to operators, AI triage, search, or analytics without explicit authorized refs.
- Do not implement an engagement feed, follower graph, ad targeting, paid inbox priority, or dark-pattern notification loops.
- Do not bypass identity, org-role, tenant, vault, storage, policy, metering, audit, or dispute rails.
- Do not add pricing, customer-count, revenue, blockchain, NFT, or per-transaction fee assumptions.

## Primary Actors And Clients

- People sending and receiving direct messages through Overrid usernames.
- Organizations managing shared inboxes, support/contact queues, team routing, and public contact surfaces.
- Native apps delivering app notifications, contact handoffs, receipts, moderation notices, and system messages.
- Personal AI Assistant triaging, summarizing, drafting, or routing messages with permission.
- Directory Listings, Workspace and Office Suite, Search Engine, Maps and Navigation, Social Photo/Video App, Wallet and Usage Center, Mobile SDK, and Mobile Backend Gateway.
- Moderators, fraud reviewers, and stewards handling spam, abuse, impersonation, unsafe attachments, and disputes.
- Overpass, Overtenant, Overkey, Universal Namespace Service, Overbase, Overstore, Overvault, Overguard, Overwatch, Overmeter, Overclaim, Fraud Control Service, Reputation and Anti-Sybil Service, ORU Account Service, and Seal Ledger.

## Dependencies

- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), [Overkey](../control_plane/overkey.md), and [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) for username resolution, organization refs, service accounts, app refs, tenant scope, and identity markers.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), and [Overvault](../data_storage_namespace/overvault.md) for structured message state, attachment object refs, encrypted payload refs, grants, and private metadata.
- [Overguard](../trust_policy_verification/overguard.md), [Workload Classifier](../trust_policy_verification/workload_classifier.md), [Overwatch](../control_plane/overwatch.md), [Overclaim](../trust_policy_verification/overclaim.md), [Fraud Control Service](../federation_public/fraud_control_service.md), and [Reputation and Anti-Sybil Service](../trust_policy_verification/reputation_anti_sybil_service.md) for policy, audit, abuse triage, disputes, spam, impersonation, and Sybil signals.
- [Personal AI Assistant](../ai_rag_model_routing/personal_ai_assistant.md) for permissioned triage, summaries, reply drafts, routing recommendations, and user-facing automation.
- [Directory Listings](directory_listings.md), [Workspace and Office Suite](workspace_office_suite.md), [Social Photo/Video App](social_photo_video_app.md), [Maps and Navigation](maps_navigation.md), and [Wallet and Usage Center](wallet_usage_center.md) for native app handoffs.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), and [Overbill](../accounting/overbill.md) for usage refs, receipts, accounting projections, and statement visibility.

## Owned Responsibilities

Messaging Center owns:

- Inbox records for persons, organizations, apps, services, and system-notification targets.
- Thread records, participant refs, org assignment refs, membership state, mute/archive/block state, and retention refs.
- Message envelope records with sender, recipient, delivery, encryption, attachment, policy, and audit refs.
- Delivery, read, archive, mute, recall, failed-delivery, and retry records.
- Organization inbox routing rules, role bindings, assignment state, internal note refs, and escalation refs.
- App notification records, notification preference refs, and delivery channel refs for native clients.
- Attachment refs, virus/safety check refs, and allowed preview metadata.
- AI triage permission records, assistant action proposals, summary refs, and user approval refs.
- Spam, abuse, harassment, impersonation, unsafe attachment, and unwanted-contact report refs.
- Usage refs for message send, delivery, storage, attachment, notification, triage, moderation, export, and replay operations.

Messaging Center does not own global identity truth, namespace ownership, secret material, binary object storage, search ranking, social feed state, payment balances, final fraud verdicts, or final dispute resolutions.

## Data Model

- `messaging_inbox`: inbox id, owner actor/org/app/service refs, username/namespace refs, inbox type, visibility class, role refs, notification prefs, state, retention policy, and audit refs.
- `org_inbox_route`: org inbox ref, routing rule, sender class, topic/category refs, assignment group refs, escalation refs, availability refs, and state.
- `message_thread`: thread id, participant refs, inbox refs, subject/topic refs, native app source refs, visibility class, retention class, muted/archived/blocked flags, latest message refs, and state.
- `thread_participant`: thread ref, participant actor/org/app refs, role, delivery preference, read cursor, mute/archive state, block state, and permission refs.
- `message_envelope`: message id, thread ref, sender inbox ref, recipient inbox refs, body ref or encrypted payload ref, attachment refs, reply/forward refs, delivery state, policy refs, and audit refs.
- `encrypted_message_ref`: payload object ref, vault grant refs, encryption profile, recipient grant refs, sender signing refs, redaction metadata, expiry refs, and access audit refs.
- `message_attachment_ref`: Overstore object refs, filename/display label, media type, size class, safety scan refs, preview refs, retention class, and policy refs.
- `delivery_record`: message ref, recipient inbox ref, route refs, notification refs, delivery attempt, failure reason, retry schedule, delivered/read timestamps, and state.
- `notification_record`: source service/app refs, target inbox/device refs, notification class, payload summary refs, priority policy, delivery channel refs, preference refs, and state.
- `contact_preference`: actor/org refs, allowed sender classes, first-contact rules, quiet hours, block/mute refs, spam threshold refs, and assistant triage allowance.
- `ai_triage_permission`: inbox/thread refs, assistant refs, allowed operations, context redaction class, confirmation rules, expiry, revocation state, and audit refs.
- `message_abuse_report`: reporter refs, target thread/message/inbox refs, report class, evidence refs, risk summary refs, fraud/reputation refs, claim refs, action refs, and state.
- `messaging_usage_ref`: send/read/deliver/attachment/notification/triage/moderation/export/replay usage, Overmeter refs, and wallet receipt refs.

Common envelope fields: `id`, `tenant_id`, `actor_id`, `organization_id`, `app_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

- `POST /messaging/inboxes`: creates a person, organization, app, service, or notification inbox.
- `GET /messaging/inboxes/{inbox_id}`: returns inbox metadata according to owner, role, and visibility rules.
- `PATCH /messaging/inboxes/{inbox_id}/preferences`: updates contact, notification, privacy, and triage preferences.
- `POST /messaging/org-inboxes/{inbox_id}/routes`: creates or updates organization routing rules.
- `POST /messaging/threads`: creates a thread with participant inbox refs and native app source refs.
- `GET /messaging/threads/{thread_id}`: reads thread state, participant refs, visible messages, and delivery summaries.
- `POST /messaging/threads/{thread_id}/participants`: adds or updates participants according to policy and org roles.
- `POST /messaging/messages`: submits a message envelope with body ref, encrypted payload ref, attachment refs, and recipient refs.
- `GET /messaging/messages/{message_id}`: reads a visible message envelope and authorized body/payload refs.
- `POST /messaging/messages/{message_id}/read`: advances read cursor for a participant.
- `POST /messaging/messages/{message_id}/recall`: requests recall/tombstone behavior where policy allows.
- `POST /messaging/threads/{thread_id}/archive`: archives a thread for the caller or allowed org role.
- `POST /messaging/threads/{thread_id}/mute`: mutes notification delivery for the caller or allowed org route.
- `POST /messaging/blocks`: creates block or first-contact restriction refs.
- `POST /messaging/reports`: creates spam, abuse, harassment, impersonation, unsafe attachment, or unwanted-contact report refs.
- `POST /messaging/ai-triage/permissions`: grants or updates permissioned AI triage for inboxes or threads.
- `POST /messaging/ai-triage/permissions/{permission_id}/revoke`: revokes future AI triage access.
- `POST /messaging/notifications`: records app/service notification delivery requests.
- `GET /messaging/replay/{record_id}`: reconstructs inbox, thread, message, delivery, triage, abuse, and usage decisions.

Mutating APIs require signed actor/service identity, tenant scope, trace id, idempotency key, sender authority, recipient refs, policy refs, and vault/storage refs where applicable. Stable errors include `username_unresolved`, `inbox_not_found`, `sender_not_authorized`, `recipient_denied`, `org_role_required`, `first_contact_denied`, `attachment_ref_invalid`, `encryption_grant_missing`, `triage_permission_required`, `message_policy_denied`, `spam_limit_exceeded`, `thread_state_conflict`, and `notification_preference_denied`.

## Event Surface

- `messaging_center.inbox_created`: inbox created for an actor, org, app, service, or notification target.
- `messaging_center.inbox_preferences_updated`: contact, notification, or triage preferences changed.
- `messaging_center.org_route_updated`: organization inbox routing changed.
- `messaging_center.thread_created`: thread created with participant refs.
- `messaging_center.participant_updated`: thread participant role/read/mute/archive/block state changed.
- `messaging_center.message_submitted`: message envelope accepted.
- `messaging_center.message_policy_checked`: policy, sender, attachment, encryption, and first-contact checks completed.
- `messaging_center.delivery_queued`: delivery attempt queued.
- `messaging_center.message_delivered`: delivery record completed.
- `messaging_center.message_read`: read cursor advanced.
- `messaging_center.message_recalled`: recall/tombstone request accepted where allowed.
- `messaging_center.notification_requested`: app/service notification accepted.
- `messaging_center.ai_triage_permission_changed`: triage permission granted, narrowed, expired, or revoked.
- `messaging_center.abuse_reported`: spam/abuse/harassment/impersonation/unsafe attachment report recorded.
- `messaging_center.usage_emitted`: usage refs emitted.

Events include inbox/thread/message refs, participant refs, sender/recipient refs, native app source refs, encryption/storage refs, policy refs, delivery state, reason codes, audit refs, and usage refs. Events must not include raw decrypted message bodies, vault secrets, private attachments, payment data, or unrelated user profile data.

## Core Workflow

1. Sender resolves recipient username, organization inbox, app inbox, or service inbox through Overpass and Universal Namespace Service.
2. Messaging validates sender authority, tenant scope, recipient rules, contact preferences, block/mute state, attachment refs, encryption grants, and policy refs.
3. Message submission creates an append-only message envelope, delivery records, notification refs, and usage refs.
4. For encrypted personal messages, payload material remains in Overstore/Overvault-protected refs; Messaging stores only envelope, grant, and access-audit refs.
5. Recipient clients read visible thread/message refs, advance read cursors, mute/archive threads, or report abuse.
6. Organization inboxes route messages by rule, role, assignment, and escalation refs.
7. Personal AI Assistant may summarize, triage, draft, or route messages only when explicit AI triage permission is active.
8. Spam, abuse, impersonation, harassment, or unsafe attachment reports create evidence refs for Fraud Control, Reputation/Anti-Sybil, Overclaim, and moderation flows.
9. Usage and audit records flow to Overmeter, Wallet/Usage Center, Overwatch, and accounting views.

## State Machine

Inbox lifecycle:

1. `requested`
2. `active`
3. `restricted`
4. `suspended`
5. `archived`
6. `deleted`

Thread lifecycle:

1. `created`
2. `active`
3. `muted`
4. `archived`
5. `blocked`
6. `under_review`
7. `closed`
8. `tombstoned`

Message lifecycle:

1. `draft`
2. `submitted`
3. `policy_checked`
4. `queued`
5. `delivering`
6. `delivered`
7. `read`
8. `failed`
9. `recalled`
10. `tombstoned`
11. `under_review`

Abuse report lifecycle:

1. `submitted`
2. `triaged`
3. `evidence_requested`
4. `action_pending`
5. `action_applied`
6. `rejected`
7. `under_claim`
8. `corrected`
9. `closed`

AI triage permission lifecycle:

1. `requested`
2. `active`
3. `narrowed`
4. `paused`
5. `revoked`
6. `expired`

State transitions are append-only. Deleted or recalled content retains tombstone, delivery, report, and audit refs where policy requires.

## Policy And Security

- Username and org-inbox routing must use Overpass and Universal Namespace Service; Messaging does not invent identity truth.
- Sender authority, recipient allowance, first-contact settings, block rules, org roles, and tenant boundaries are deny-by-default.
- Encrypted personal messages keep payload refs outside Messaging; decrypted content is available only to authorized recipients and approved tools.
- AI triage cannot inspect private messages unless an active permission manifest covers the inbox/thread, operation, context redaction class, and expiry.
- Attachments must use Overstore refs and safety scan/policy refs before preview or delivery.
- Notification priority cannot override user preferences, quiet hours, block rules, or abuse throttles.
- Organization inbox actions require role refs and produce audit events.
- Abuse controls must handle spam, harassment, impersonation, unwanted contact, malicious attachments, and notification flooding.
- Search indexing of message content is off by default and requires explicit app/user authorization; metadata-only search must be redacted.
- Messaging must avoid addictive loops, paid priority, ad targeting, hidden profiling, and surveillance metrics.

## Metering And Accounting

- Emit usage refs for inbox creation, message send, delivery, reads, notifications, storage refs, attachment refs, encrypted payload refs, AI triage, moderation, reports, exports, replay, bandwidth, and retention.
- Link usage to actor, org, app, service, inbox, thread, message, attachment, notification, triage permission, report, Overmeter refs, and wallet receipt refs.
- Messaging Center does not maintain balances, payment instruments, ad bids, invoices, provider payouts, ORU projection truth, or ledger entries.
- Native-service economics remain structural and near-cost; surplus routing is handled by stewardship/accounting services, not by inbox priority.
- Do not encode hardcoded prices, revenue forecasts, paid placement, or per-operation external payment calls.

## Observability And Operations

- Expose message send latency, delivery latency, delivery failure rates, queue depth, notification failure rates, encrypted-message grant failures, attachment-scan denials, first-contact denial rates, spam bursts, abuse backlog, org assignment backlog, AI triage usage, permission revocations, and usage emission status.
- Alert on spam floods, repeated failed delivery, unexpected plaintext access attempts, missing encryption grants, attachment safety failures, notification abuse, org inbox role violations, high report volume, missing usage refs, and replay gaps.
- Provide user-visible privacy audit for AI triage, message access, encrypted payload access, attachment access, and notification delivery.
- Provide operator diagnostics using refs, reason codes, and redacted summaries rather than raw private message content.
- Support export and retention jobs for user-controlled data portability, legal/compliance boundaries, and tombstone preservation.

## Failure Modes And Recovery

- Username or inbox unresolved: return stable reason code and do not create a message envelope.
- Recipient blocks or first-contact denies sender: stop delivery and record denial refs without leaking private recipient settings.
- Encryption grant missing: keep message draft/submitted pending until grant is fixed or sender cancels.
- Attachment scan pending or denied: hold message or strip attachment according to policy and preserve reason refs.
- Delivery route unavailable: retry with bounded backoff and show pending/failure state to sender.
- Notification delivery fails: preserve message delivery state and retry notification separately.
- Organization assignment unavailable: deliver to org inbox queue with escalation refs.
- AI triage permission revoked: stop future triage and invalidate cached assistant summaries where required.
- Abuse false positive: restore allowed delivery/visibility through correction refs and preserve review evidence.
- Usage emission fails: mark operation usage pending and reconcile before final receipt visibility.

## Validation Plan

- Users can create inboxes, send username-addressed messages, receive messages, read messages, mute/archive threads, and revoke triage permissions.
- Organization inboxes route by role and rule without leaking private user inboxes or bypassing org access checks.
- Encrypted messages store payload refs and grant refs without exposing decrypted bodies to operators, search, or AI triage.
- Attachments flow through Overstore refs and safety/policy refs before delivery or preview.
- App notifications obey user preferences, quiet hours, and abuse throttles.
- AI triage works only with active permission and produces audit refs for context access and actions.
- Spam/abuse reports create evidence refs, moderation refs, and claim handoffs without exposing private data beyond authorized reviewers.
- Usage refs flow to Overmeter and Wallet/Usage Center.
- Replay reconstructs inbox, thread, message, delivery, notification, triage, report, and usage decisions.

## Build Breakdown

1. Define inbox, org route, thread, participant, message envelope, encrypted payload ref, attachment ref, delivery, notification, contact preference, AI triage permission, abuse report, and usage schemas.
2. Implement inbox, thread, message send/read/archive/mute/block/report, notification, AI triage permission, and replay APIs.
3. Add Overpass/namespace username resolution, org-role checks, Overguard policy checks, Overvault encrypted payload grants, and Overstore attachment refs.
4. Add delivery queue integration with retry, failure, read, recall, and tombstone records.
5. Add organization inbox routing, assignment, internal notes, and escalation refs.
6. Add Personal AI Assistant triage proposal/permission flows.
7. Add abuse, spam, fraud, reputation, Overclaim, and moderation handoffs.
8. Add Overmeter usage, Wallet/Usage Center receipt visibility, Overwatch replay, export, retention, SDK, and mobile client flows.

## Handoff And Downstream Use

- Directory Listings uses Messaging Center for contact handoffs without exposing private contact data.
- Workspace and Office Suite uses Messaging Center for comments, share notifications, approval requests, and team inbox handoffs.
- Social Photo/Video App uses Messaging Center for private contact and moderation notices while keeping feed content separate.
- Maps and Navigation uses Messaging Center for place/route sharing and local business contact handoffs.
- Search Engine indexes only authorized message metadata or explicit user-approved content refs.
- Personal AI Assistant uses Messaging Center through permissioned tool calls and triage manifests.
- Wallet and Usage Center shows messaging usage, receipts, triage permissions, and app notification controls.
- Mobile SDK and Mobile Backend Gateway use Messaging Center as the native messaging/notification channel.

## Open Design Questions

Resolved decisions:

- External bridges are allowed only after the native username protocol, inbox model, abuse controls, encrypted payload refs, and replay evidence are stable. The minimum bridge set is standards-style email import/forwarding, organization webhook or support-system handoff, and narrowly scoped SMS/phone notification relay for recovery or critical delivery notices. External chat/social bridges remain import/export or explicit per-thread relay adapters until separate policy approves richer interop. No bridge may become an identity authority, canonical message store, payment surface, search authority, or privileged bypass; bridges must use Overpass/namespace refs, Overkey service credentials, Overvault/Overstore refs, Overguard policy checks, Overwatch audit, and user/org opt-in grants.
- Retention defaults are classed policies, not one global duration. Personal inbox content defaults to user-controlled retention with recall/delete tombstones, vault grant revocation, and participant-visible state changes while preserving required audit refs. Organization inboxes default to org-configured retention classes with role-scoped access, assignment history, export refs, and legal/dispute hold compatibility. App notification inboxes default to short operational retention: payload summaries expire first, while delivery, preference, abuse, and usage refs can remain as redacted evidence. System-message inboxes default to durable audit/evidence retention for security, accounting, policy, compliance, and incident messages, with raw private payloads minimized or excluded.
- Encrypted-message search is metadata-only by default and must use access-scoped local projections owned by the inbox, user, organization, or authorized app. Searchable fields can include message/thread/inbox refs, authorized participant refs, sender/recipient inbox refs visible to the caller, timestamp or time-bucket, delivery/read/retry state, retention class, encryption profile id, attachment media/size class, safety-scan status, user-created labels, and explicit redacted subject/summary tokens when the owning participant or org grants them. Raw body text, decrypted attachments, private filenames, vault grants, embeddings, and AI summaries are not shared-index material; if projections or embeddings are needed, they inherit the encrypted record's data class, grant, revocation, retention, and rebuild/tombstone rules.
- Cross-organization legal or compliance holds do not let Messaging Center erase or expose content on its own. Messaging records deletion and recall requests immediately, stops future delivery where policy allows, hides or tombstones normal views, and sends the hold/deletion facts to Compliance Boundary Service and Overguard. Held payload refs remain sealed under Overvault/Overstore retention or escrow policy with Overwatch evidence and Overclaim/incident refs when applicable. Holds must be scoped, reason-coded, time-bounded, redacted, and appealable unless a specific law or emergency policy blocks notice. When the hold expires or is narrowed, queued user deletion, grant revocation, projection rebuild, and recall/tombstone cleanup proceed idempotently.
