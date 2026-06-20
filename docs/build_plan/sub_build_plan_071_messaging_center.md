# SUB BUILD PLAN #71 - Messaging Center

Attached SDS: [SDS #71 - Messaging Center](../sds/native_apps/messaging_center.md)

## Purpose

This sub-build plan turns SDS #71 into an implementation sequence for Messaging Center. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Messaging Center is the Phase 12 native public utility for username-addressed inboxes, organization inboxes, app/service notifications, message envelopes, delivery/read/recall/tombstone records, encrypted payload refs, attachment refs, contact preferences, AI triage permissions, abuse reports, usage refs, audit refs, and replay evidence. It does not own identity truth, namespace ownership, object storage, vault secrets, payment/accounting truth, search ranking, social feed state, final fraud authority, final reputation scores, final dispute outcomes, or external bridge authority.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #71: Messaging Center](../sds/native_apps/messaging_center.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoffs, and resolved open-question decisions. |
| [Messaging Center service plan](../service_catalog/native_apps/messaging_center.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency keys, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Overclaim disputes, Fraud Control evidence, Challenge Task facts, Reputation/Anti-Sybil recommendations, and deny-by-default behavior. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill accounting truth, Wallet/Usage Center receipt refs, and the rule that Messaging emits usage refs but never mutates balances or payment records. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase state, Overstore attachment/object refs, Overvault encrypted payload grants, Universal Namespace refs, retention, backup/restore, and replay substrates. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider fraud, reputation, anti-Sybil, challenge, appeal, abuse-control, and throttling prerequisites used by public contact, app notifications, and spam controls. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first build point for Messaging Center and its first useful inbox, message, org routing, notification, encrypted payload, attachment, abuse, triage, usage, audit, and replay slice. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal messaging privacy, retention, compliance-hold, bridge, spam/abuse, incident response, threat review, public reporting, reliability, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #71 first build work aligned to master Phase 12, with Phase 13 as governance/security/compliance/reporting hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and contracts, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where a service boundary exists, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript for client surfaces only, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, ad/paid-priority, pricing, revenue, customer-count, search-ranking, social-feed, or raw-message-surveillance drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 8, 11, 12, and 13 | Attach SDS #71, preserve Phase 12 as first build, record prerequisites, and freeze Messaging ownership boundaries. |
| 2 | Master Phases 0, 1, 4, 5, 8, and 12 | Define Rust contracts, canonical schemas, lifecycle enums, event surfaces, stable errors, and deterministic fixtures. |
| 3 | Master Phases 1, 4, 8, and 12 | Implement inboxes, threads, participants, contact preferences, block/mute/archive state, and tenant/app/org isolation. |
| 4 | Master Phases 1, 4, 8, 11, and 12 | Implement username resolution, organization inbox routing, first-contact policy, app/service inboxes, and notification preference checks. |
| 5 | Master Phases 1, 4, 8, 12, and 13 | Implement message submission, delivery queues, read cursors, recall, retry, tombstones, and local-first bridge reconciliation. |
| 6 | Master Phases 4, 5, 8, 12, and 13 | Implement encrypted payload refs, attachment refs, app notifications, metadata-only search projections, and safety scans. |
| 7 | Master Phases 4, 6, 8, 12, and 13 | Implement AI triage permissions, assistant proposals, native app handoffs, mobile bindings, SDK/CLI/admin/support surfaces, and delegated tool boundaries. |
| 8 | Master Phases 4, 11, 12, and 13 | Implement spam, abuse, harassment, impersonation, unsafe attachment, unwanted-contact, Fraud/Reputation/Overclaim, moderation, and appeal handoffs. |
| 9 | Master Phases 5, 8, 12, and 13 | Implement usage refs, Wallet/Usage Center projections, audit exports, retention classes, compliance holds, replay, and data portability. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, security/privacy/compliance boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Messaging Center uses Rust-first shared contracts and service-facing APIs for inbox records, org routes, threads, participants, message envelopes, delivery/read/recall/tombstone records, encrypted payload refs, attachment refs, notification records, contact preferences, AI triage permissions, abuse reports, usage refs, audit refs, and replay bundles. TypeScript is acceptable for generated client bindings and native/web UI surfaces, but it must call Overrid APIs and must not become a privileged message authority.
- Messaging contracts, fixtures, event records, policy snapshots, replay bundles, retention classes, redaction profiles, abuse reports, triage permissions, export manifests, and compliance-hold records use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/app/org/device scope where applicable, sender authority, recipient refs, trace id, idempotency key, policy refs, storage/vault refs where applicable, reason codes, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for message envelope display hashes, payload/object refs, fixture inputs, delivery/replay bundles, retention manifests, export manifests, audit bundles, and deterministic comparisons.
- Structured state, encrypted payloads, attachments, queueing, identity, namespace, key status, policy, audit, usage, accounting refs, dispute refs, diagnostics, AI route refs, and replay must use native Overrid service boundaries such as Overbase, Overstore, Overvault, Overqueue, Overpass, Overtenant, Overkey, Universal Namespace Service, Overgate, Overguard, Overwatch, Overmeter, ORU Account Service, Seal Ledger, Wallet/Usage Center, Personal AI Assistant, AI Gateway Router, Directory Listings, Workspace and Office Suite, Search Engine, Maps and Navigation, Social Photo/Video App, Fraud Control, Reputation and Anti-Sybil Service, Overclaim, Mobile SDK, Mobile Backend Gateway, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, external email/phone/chat bridges, paid priority, ad targeting, hidden profiling, search ranking, social feeds, pricing, revenue forecasts, customer-count assumptions, raw vault secrets, raw decrypted message bodies, private attachments, global identity truth, final fraud/reputation/dispute authority, direct ORU/Seal Ledger mutation, or hidden moderation action the Messaging Center boundary.

## Phase 1: SDS Attachment, Phase 12 Scope, And Messaging Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #71.**
  - Design: Link this document from the Messaging Center SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/native_apps/messaging_center.md`, `docs/service_catalog/native_apps/messaging_center.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #71 returns both the Messaging Center SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep first implementation in Phase 12 because Messaging is a native application utility built on identity, namespace, storage, vault, policy, metering, accounting, mobile, abuse-control, and dispute rails that earlier phases provide.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 8, and 11 supply prerequisites; Phase 12 builds the first useful messaging utility; Phase 13 hardens privacy, retention, bridges, abuse controls, incident response, reporting, compliance, and scale.
  - Validation: Review proves the plan does not move Messaging into Phase 1 identity, Phase 8 storage/vault, Phase 11 abuse-control-only work, Phase 13-only governance, Search ranking, Social feed, or payment/accounting ownership.

- **1.3 Freeze the Messaging ownership boundary.**
  - Design: Record that Messaging owns inbox records, org routes, threads, participant state, message envelopes, delivery/read/recall/tombstone records, notifications, contact preferences, triage permissions, abuse reports, usage refs, audit refs, and replay projections.
  - Output: Ownership checklist for architecture, API, UI, implementation, operations, and review gates.
  - Validation: Review confirms Messaging does not own identity truth, namespace ownership, object storage, vault secrets, search ranking, social feed state, payment balances, final fraud verdicts, final reputation scores, final claim outcomes, or bridge authority.

- **1.4 Carry forward resolved SDS #71 decisions.**
  - Design: Preserve post-native opt-in bridge boundaries, classed retention defaults, encrypted-message metadata-only search projections, and compliance-hold/delete/recall handling.
  - Output: Resolved-decision checklist covering bridge classes, bridge denials, retention classes, encrypted-search projection rules, legal/compliance holds, user delete/recall tombstone cleanup, sealed refs, and appealability.
  - Validation: Review rejects bridge-owned identity/message/payment/search authority, raw encrypted body indexing, one-size retention, unscoped holds, hidden permanent holds, and hold-driven content exposure.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Record how Directory Listings, Workspace and Office Suite, Search Engine, Maps and Navigation, Social Photo/Video App, Personal AI Assistant, AI Gateway Router, Wallet/Usage Center, Mobile SDK, Mobile Backend Gateway, Overpass, Universal Namespace Service, Overbase, Overstore, Overvault, Overguard, Overwatch, Overmeter, Overclaim, Fraud Control, Reputation/Anti-Sybil, and Compliance Boundary interact through refs.
  - Output: Boundary matrix naming allowed reads, owned writes, handoff refs, denied direct mutation, audience rules, notification rules, usage refs, audit refs, replay refs, moderation paths, and owner-service finality.
  - Validation: Review confirms each downstream service keeps its authority and Messaging exchanges refs/events rather than copying raw private content or inventing canonical truth owned elsewhere.

## Phase 2: Contracts, Schemas, Events, Stable Errors, And Fixtures

### Work Items

- **2.1 Create the Messaging Rust contract module.**
  - Design: Add contract types for inboxes, org routes, threads, participants, message envelopes, encrypted payload refs, attachment refs, delivery records, notifications, contact preferences, AI triage permissions, abuse reports, usage refs, replay bundles, retention classes, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, inbox-type enums, delivery-state enums, retention-class enums, redaction-class enums, abuse-class enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from identity, object storage, vault secret handling, search ranking, accounting mutation, and final dispute authority.

- **2.2 Define canonical JSON and JSON Schema contracts.**
  - Design: Model all Messaging records with trace ids, idempotency keys, actor/org/app/service refs, tenant scope, state, policy refs, audit refs, reason codes, and stable schema versions.
  - Output: JSON Schema files, valid examples, invalid examples, signed command examples, event examples, replay examples, export examples, and retention/compliance examples.
  - Validation: Schema tests reject records without required owner refs, policy refs, retention class, audit refs, visibility/redaction class, state, trace id, idempotency key where mutating, and schema version.

- **2.3 Define event and replay contracts.**
  - Design: Model inbox, preference, org-route, thread, participant, message, policy-check, delivery, read, recall, notification, triage-permission, abuse-report, usage, export, retention, and replay events without raw private payloads.
  - Output: Event schema set, replay bundle schema, BLAKE3 display hash rules, redaction profile refs, stable ordering, and fixture-backed event streams.
  - Validation: Tests prove events include necessary refs and reason codes while excluding raw decrypted message bodies, private attachments, vault secrets, payment data, and unrelated profile data.

- **2.4 Define stable error taxonomy.**
  - Design: Preserve SDS stable errors and add implementation-ready mapping for username, inbox, sender, recipient, org role, first-contact, attachment, encryption, triage, policy, spam, thread conflict, notification preference, retention, hold, and bridge denials.
  - Output: Stable error registry, HTTP/API mapping, client-facing messages, support-safe diagnostics, retryability flags, and replay refs.
  - Validation: Tests prove denials are deterministic, support-safe, tenant-safe, and replayable without exposing private recipient settings or hidden abuse thresholds.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for inbox creation, org route creation, thread creation, message send/read, encrypted payload refs, attachments, notification preferences, first-contact denial, AI triage grant/revoke, abuse report, usage emission, retention, hold, export, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected errors, BLAKE3 hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, redacted outputs, and replay output across repeated runs.

## Phase 3: Inbox, Thread, Participant, Contact Preference, And Isolation Foundations

### Work Items

- **3.1 Implement inbox records.**
  - Design: Support person, organization, app, service, and system-notification inboxes with owner refs, namespace refs, inbox type, visibility class, role refs, notification prefs, state, retention policy, and audit refs.
  - Output: `messaging_inbox` schema, create/read/update APIs, lifecycle states, visibility projections, owner-role rules, and audit events.
  - Validation: Tests prove inboxes cannot exist without owner identity/namespace refs, tenant scope, retention policy, visibility class, and Overwatch audit refs.

- **3.2 Implement thread and participant records.**
  - Design: Model thread refs, participant refs, inbox refs, subject/topic refs, native app source refs, retention class, muted/archived/blocked flags, read cursor, roles, and state.
  - Output: `message_thread` and `thread_participant` schemas, create/read/participant-update APIs, lifecycle transitions, role-specific projections, and event fixtures.
  - Validation: Tests prove thread participants cannot bypass owner, recipient, org-role, block, mute, archive, visibility, or tenant checks.

- **3.3 Implement contact preferences and block/mute rules.**
  - Design: Support allowed sender classes, first-contact rules, quiet hours, block/mute refs, spam thresholds, notification prefs, assistant triage allowance, and revocation.
  - Output: `contact_preference` schema, update APIs, preference projection API, enforcement hooks, and audit events.
  - Validation: Tests prove first-contact denial, block/mute, quiet hours, and notification preferences stop delivery or notification without leaking private recipient settings.

- **3.4 Implement tenant, actor, app, org, and device isolation.**
  - Design: Enforce actor, organization, tenant, app, device, moderator, operator, auditor, and service-account scopes for all inbox, thread, preference, message, notification, triage, report, export, and replay operations.
  - Output: Access-control adapter, role-specific projections, tenant/app/device ownership checks, stronger-role gates, denial refs, and Overwatch audit events.
  - Validation: Tests prove apps cannot read unrelated inboxes, org users cannot access unrelated org inboxes, support views cannot bypass redaction, and devices cannot use revoked grants.

- **3.5 Implement retention class assignment at record creation.**
  - Design: Assign personal, organization, app notification, system-message, dispute/hold, and export retention classes at creation time with policy refs and later mutation constraints.
  - Output: Retention resolver, retention-class refs on inbox/thread/message/notification/report/export records, lifecycle hooks, and retention fixtures.
  - Validation: Tests prove records without retention class fail, class changes are audited, and app notification payload summaries can expire separately from delivery/preference/usage evidence.

## Phase 4: Username Resolution, Organization Routing, App Inboxes, And Notifications

### Work Items

- **4.1 Implement username and namespace resolution.**
  - Design: Resolve recipient usernames, org inboxes, app inboxes, service inboxes, and system targets through Overpass and Universal Namespace Service without Messaging inventing identity truth.
  - Output: Resolution adapter, recipient validation, namespace ref caching policy, stable `username_unresolved` and `inbox_not_found` errors, and audit refs.
  - Validation: Tests prove unresolved or stale usernames do not create message envelopes and resolution failures do not leak private namespace records.

- **4.2 Implement organization inbox routing.**
  - Design: Route organization messages by rule, sender class, topic/category refs, assignment group refs, escalation refs, availability refs, role refs, and state.
  - Output: `org_inbox_route` schema, route create/update APIs, assignment records, escalation refs, internal note refs, and `messaging_center.org_route_updated` events.
  - Validation: Tests prove org inbox actions require role refs, preserve assignment history, and cannot expose private personal inbox state to unauthorized org members.

- **4.3 Implement first-contact and sender authority policy.**
  - Design: Validate sender authority, recipient rules, first-contact settings, block state, spam thresholds, org roles, tenant boundaries, and Overguard decisions before thread or message creation.
  - Output: Policy precheck adapter, contact-denial records, spam threshold refs, sender-authority errors, missing-fact errors, and policy snapshot refs.
  - Validation: Tests prove deny-by-default behavior for missing sender authority, blocked recipients, denied first contact, org-role mismatch, tenant mismatch, and policy facts.

- **4.4 Implement app, service, and system notification inboxes.**
  - Design: Support app/service/system notification targets with notification class, payload summary refs, priority policy, delivery channels, preference refs, quiet hours, abuse throttles, and state.
  - Output: `notification_record` schema, `POST /messaging/notifications`, delivery channel refs, preference enforcement, throttling state, and event fixtures.
  - Validation: Tests prove notification priority cannot override preferences, quiet hours, block rules, abuse throttles, or payload redaction.

- **4.5 Implement native app contact handoff records.**
  - Design: Accept contact/share/request handoffs from Directory Listings, Workspace and Office Suite, Search Engine, Maps and Navigation, Social Photo/Video App, Wallet/Usage Center, and Mobile Backend Gateway through refs and redacted projections.
  - Output: Handoff schema, source app refs, target inbox/thread refs, consent refs, redaction class, denied handoff state, audit refs, and replay projections.
  - Validation: Tests prove handoffs do not copy private listing contact data, route geometry, workspace content, search ranking internals, social feed payloads, or wallet/accounting truth into Messaging.

## Phase 5: Message Envelopes, Delivery, Read State, Recall, Tombstones, And Bridge Reconciliation

### Work Items

- **5.1 Implement message envelope submission.**
  - Design: Create append-only message envelopes with thread refs, sender inbox refs, recipient inbox refs, body ref or encrypted payload ref, attachment refs, reply/forward refs, delivery state, policy refs, and audit refs.
  - Output: `message_envelope` schema, `POST /messaging/messages`, idempotent submission, accepted/denied states, stable errors, usage refs, and events.
  - Validation: API tests prove signed envelopes, tenant scope, sender authority, recipient refs, trace id, idempotency key, policy refs, and storage/vault refs where applicable are required.

- **5.2 Implement delivery queue integration and retries.**
  - Design: Queue delivery records with route refs, notification refs, delivery attempt, failure reason, retry schedule, delivered/read timestamps, and bounded backoff.
  - Output: `delivery_record` schema, delivery worker contract, retry states, failed-delivery state, notification split handling, and `messaging_center.delivery_queued` events.
  - Validation: Tests prove delivery failures preserve pending/failure state, retry without widening recipient permissions, and replay all delivery decisions.

- **5.3 Implement read cursor, archive, mute, and recall behavior.**
  - Design: Advance participant read cursors, archive or mute threads per participant/org role, and request recall/tombstone behavior where policy allows.
  - Output: Read API, archive API, mute API, recall API, participant state updates, tombstone records, delivery/read events, and state transitions.
  - Validation: Tests prove read/archive/mute/recall operations are participant-scoped, role-scoped, policy-scoped, audited, and do not delete required evidence.

- **5.4 Implement tombstone and deletion cleanup queues.**
  - Design: Record user delete/recall requests immediately, stop future delivery where policy allows, hide or tombstone normal views, and queue grant revocation, projection rebuild, and cleanup work.
  - Output: Tombstone records, cleanup jobs, projection rebuild jobs, retained audit refs, legal/dispute hold markers, and idempotent cleanup state.
  - Validation: Tests prove deletion/recall cleanup is idempotent, preserves required evidence, and does not expose held content or silently erase compliance records.

- **5.5 Implement bounded local-first bridge reconciliation.**
  - Design: Use mSwarm Runtime Bridge only for optional local-first delivery/read-receipt queues where grants, first-contact policy, recipient policy, and abuse controls still validate on reconnect.
  - Output: Bridge handoff refs, offline delivery receipts, reconnect validation, conflict states, stale markers, failure refs, and replay refs.
  - Validation: Tests prove the bridge cannot widen contact grants, keep raw payloads as canonical truth, skip first-contact checks, bypass revocation, or override Messaging/Overvault/Overguard decisions.

## Phase 6: Encryption, Attachments, Notifications, Search Projections, And Safety

### Work Items

- **6.1 Implement encrypted payload refs.**
  - Design: Store encrypted personal message payload material outside Messaging through Overstore/Overvault-protected refs with recipient grants, sender signing refs, redaction metadata, expiry refs, and access audit refs.
  - Output: `encrypted_message_ref` schema, grant validation, access audit hooks, revoke/expire handling, and stable `encryption_grant_missing` errors.
  - Validation: Tests prove decrypted bodies are unavailable to operators, search, analytics, AI triage, support, or unauthorized recipients without explicit active grants.

- **6.2 Implement attachment refs and safety checks.**
  - Design: Use Overstore object refs, media/size class, preview refs, retention class, safety scan refs, policy refs, and attachment delivery state before preview or delivery.
  - Output: `message_attachment_ref` schema, attachment validation, scan pending/denied states, preview policy, unsafe attachment reports, and stable errors.
  - Validation: Tests prove attachments cannot preview or deliver without valid Overstore refs, safety scan refs, policy refs, retention class, and audience-safe labels.

- **6.3 Implement app/service notification payload redaction.**
  - Design: Redact notification summaries by audience, device, app, quiet-hour state, preference state, abuse throttle, and payload class.
  - Output: Notification projection API, redaction profiles, device-safe payloads, denial states, and replay refs.
  - Validation: Tests prove notification payloads do not leak private message bodies, private attachments, vault refs, hidden sender state, or private recipient settings.

- **6.4 Implement metadata-only encrypted-message search projections.**
  - Design: Keep encrypted-message search metadata-only by default and create access-scoped local projections owned by the inbox, user, organization, or authorized app.
  - Output: Search projection schema, allowed fields, redacted subject/summary token rules, grant inheritance, revocation rebuild hooks, tombstone rules, and denied projection states.
  - Validation: Tests reject raw body text, decrypted attachments, private filenames, vault grants, embeddings, and AI summaries as shared-index material.

- **6.5 Implement plaintext-access and unsafe-content monitors.**
  - Design: Alert on unexpected plaintext access attempts, missing encryption grants, unsafe attachments, notification abuse, search projection overexposure, and raw-content export attempts.
  - Output: Metrics, alerts, audit events, support-safe diagnostics, incident refs, and test fixtures.
  - Validation: Drills prove plaintext and unsafe-content signals produce evidence without storing or displaying raw private message content.

## Phase 7: AI Triage, Native App Handoffs, Mobile, SDK, CLI, Admin, And Support Surfaces

### Work Items

- **7.1 Implement AI triage permissions.**
  - Design: Allow Personal AI Assistant triage, summaries, reply drafts, routing recommendations, and automation only through explicit permission records with allowed operations, context redaction class, confirmation rules, expiry, revocation state, AI Gateway Router route refs where model execution is needed, and audit refs.
  - Output: `ai_triage_permission` schema, grant/update/revoke APIs, permission lifecycle, assistant-access checks, AI Gateway route/request refs, cached-summary invalidation, and replay events.
  - Validation: Tests prove assistant access is denied by default, Messaging does not choose model routes, and revoked permissions stop future triage and invalidate cached summaries where required.

- **7.2 Implement assistant proposal and approval records.**
  - Design: Record assistant proposals for summary, draft, route, archive, report, notification preference, and org assignment actions as proposals requiring user/org approval where side effects exist.
  - Output: Proposal schema, approval/refusal refs, side-effect confirmation refs, redaction badges, usage refs, and audit events.
  - Validation: Tests prove the assistant cannot send messages, expose encrypted content, change preferences, report users, assign org queues, or export content without required approval.

- **7.3 Implement native app handoff ergonomics.**
  - Design: Provide typed handoff flows for Directory contact, Workspace comments/share requests, Maps place/route sharing, Social private contact/moderation notices, Search authorized result handoffs, and Wallet usage/permission controls.
  - Output: Handoff APIs, client projections, authority badges, redaction badges, denied-state explanations, replay links, and fixture-backed examples.
  - Validation: Tests prove the UI does not imply Messaging owns listings, workspace content, map truth, search ranking, social feed content, wallet balances, or final moderation authority.

- **7.4 Implement Mobile SDK and Mobile Backend Gateway bindings.**
  - Design: Add mobile-safe projections for inboxes, threads, messages, notifications, offline command intake, push notification refs, sync cursors, compact deltas, device revocation, and idempotent replay.
  - Output: Mobile contracts, generated bindings, compact projections, offline queue refs, push payload rules, stable error mappings, and mobile fixtures.
  - Validation: Mobile contract tests prove signing, device scope, revocation, first-contact policy, redaction, usage/audit refs, stable errors, and replay survive binding generation.

- **7.5 Implement SDK, CLI, admin, and support bindings.**
  - Design: Generate clients and operational views for inboxes, org routes, threads, messages, notifications, triage permissions, abuse reports, usage, exports, retention, holds, and replay.
  - Output: SDK bindings, CLI commands, admin/support projections, moderator-safe queues, fixture-based examples, and documentation.
  - Validation: Contract tests prove generated clients preserve signing, idempotency, trace ids, tenant/app/device scope, stable errors, redaction states, usage refs, and audit refs.

## Phase 8: Spam, Abuse, Moderation, Fraud, Reputation, Claims, And Appeals

### Work Items

- **8.1 Implement abuse report intake.**
  - Design: Capture spam, harassment, impersonation, unsafe attachment, unwanted contact, notification flooding, org inbox abuse, bridge abuse, and AI triage misuse with evidence refs and redaction rules.
  - Output: `message_abuse_report` schema, `POST /messaging/reports`, evidence refs, risk summary refs, abuse state machine, public-safe summaries, and events.
  - Validation: Tests prove reports preserve reporter protections, target refs, reason codes, policy refs, audit evidence, and redacted reviewer projections.

- **8.2 Implement spam and unwanted-contact controls.**
  - Design: Combine first-contact rules, block/mute refs, rate limits, spam thresholds, Reputation/Anti-Sybil signals, Fraud Control recommendations, notification throttles, and bounded consequence proposals.
  - Output: Spam-control adapter, throttle records, denied delivery states, report/block affordances, appeal refs, and replay fixtures.
  - Validation: Tests prove spam controls do not expose hidden thresholds and cannot silently suppress legitimate delivery without reason-coded, replayable evidence.

- **8.3 Implement fraud, reputation, and anti-Sybil handoffs.**
  - Design: Send redacted messaging abuse, spam, impersonation, notification abuse, bridge abuse, and unsafe attachment refs to Fraud Control and Reputation/Anti-Sybil for recommendations rather than final Messaging outcomes.
  - Output: Fraud handoff refs, reputation signal refs, recommendation refs, policy application refs, redacted evidence summaries, and replay bundles.
  - Validation: Tests prove private fraud internals and anti-abuse thresholds remain redacted and Messaging applies recommendations only through policy, reviewer, or owning-service finality rules.

- **8.4 Implement Overclaim and moderation workflows.**
  - Design: Connect contested abuse actions, org inbox disputes, impersonation disputes, wrongful delivery/denial claims, attachment safety disputes, and compliance-hold appeals to Overclaim and moderation refs.
  - Output: Claim refs, moderation action refs, reviewer refs, appeal refs, correction refs, action history, and support-safe views.
  - Validation: Tests prove moderation actions are role-scoped, auditable, reversible where policy permits, and cannot expose raw private messages beyond authorized review scope.

- **8.5 Implement reviewer, steward, and support queues.**
  - Design: Provide redacted queues for spam bursts, abuse reports, harassment, impersonation, unsafe attachments, notification abuse, org inbox issues, triage misuse, compliance holds, and bridge abuse.
  - Output: Queue APIs, redaction badges, missing-evidence states, action eligibility reasons, audit links, replay links, and stable denial reasons.
  - Validation: Tests prove reviewer/support views never expose raw decrypted bodies, private attachments, vault secrets, unrelated profile data, other-tenant evidence, or hidden anti-abuse thresholds.

## Phase 9: Usage, Wallet Projections, Audit, Export, Retention, Holds, And Replay

### Work Items

- **9.1 Emit usage refs for Messaging operations.**
  - Design: Meter inbox creation, message send, delivery, reads, notifications, storage refs, attachment refs, encrypted payload refs, AI triage, moderation, reports, exports, replay, bandwidth, and retention operations.
  - Output: Messaging usage events, Overmeter refs, wallet receipt refs, operation dimensions, actor/org/app/service/inbox/thread/message/attachment/notification/report tags, and reconciliation refs.
  - Validation: Tests prove usage refs are emitted for successful and policy-denied operations where required and no usage event mutates ORU balances directly.

- **9.2 Implement Wallet/Usage Center projections.**
  - Design: Show users and orgs messaging usage, receipts, app notification controls, triage permissions, attachment/storage usage, export usage, pending usage, and revocation controls without pricing or revenue forecasts.
  - Output: Wallet projection API, receipt refs, pending usage states, permission history, notification preference links, triage revocation shortcuts, and support-safe explanations.
  - Validation: Tests prove Messaging does not maintain balances, payment records, ad bids, paid priority, invoices, payout truth, hardcoded prices, revenue forecasts, or per-operation external payment calls.

- **9.3 Implement audit exports and data portability.**
  - Design: Export user/org visible inbox, thread, message envelope, delivery, notification, triage, abuse, usage, retention, and replay data with audience-specific redaction and deterministic ordering.
  - Output: Export API, export manifest, BLAKE3 hash manifest, redaction profile refs, retention labels, legal/dispute hold markers, and replay links.
  - Validation: Tests prove exports include authorized refs and state while excluding raw private payloads, raw vault secrets, private attachments beyond grants, fraud internals, hidden thresholds, and unrelated user data.

- **9.4 Implement legal/compliance hold handling.**
  - Design: Send hold/deletion facts to Compliance Boundary Service and Overguard, keep held payload refs sealed through Overvault/Overstore retention or escrow policy, and preserve Overwatch/Overclaim/incident evidence where applicable.
  - Output: Hold records, reason codes, scope refs, time bounds, redaction profiles, notice/appeal refs, release/narrowing workflows, and cleanup resume jobs.
  - Validation: Tests prove holds are scoped, reason-coded, time-bounded, redacted, appealable where policy permits, and do not let Messaging erase or expose content on its own.

- **9.5 Implement replay and retention cleanup.**
  - Design: Reconstruct inbox, thread, message, delivery, notification, triage, abuse, usage, retention, hold, export, delete, recall, and bridge decisions from append-only refs.
  - Output: `GET /messaging/replay/{record_id}`, replay bundle format, retention cleanup jobs, tombstone projection rebuilds, event ordering, and redacted replay projections.
  - Validation: Tests prove replay can explain key decisions without raw private content and retention cleanup preserves required evidence while removing or hiding data according to policy.

## Phase 10: Phase 13 Hardening, Validation, Documentation, And Handoff

### Work Items

- **10.1 Harden messaging privacy, retention, and compliance boundaries.**
  - Design: Connect encrypted messages, metadata projections, first-contact rules, notification preferences, AI triage, deletion/recall, legal holds, retention classes, bridge boundaries, exports, and replay to Compliance Boundary, Overguard, Overclaim, Fraud Control, and incident paths.
  - Output: Readiness matrix, compliance-boundary refs, disabled/review-gated bridge checklist, retention rules, notice/appeal rules, public-safe summaries, and explicit disabled states.
  - Validation: Tests prove sensitive modes cannot be enabled by config drift, UI-only changes, bridge adapters, search indexing, assistant tooling, or moderator action without required boundary refs.

- **10.2 Harden security, abuse, and incident response.**
  - Design: Threat-model inbox takeover, org inbox role bypass, first-contact bypass, spam floods, notification abuse, encrypted payload leakage, attachment malware, plaintext access attempts, AI triage misuse, metadata search leakage, bridge replay, retention bypass, hold abuse, and replay/audit gaps.
  - Output: Threat model entries, security review checklist, incident runbooks, privacy review checklist, mitigations, monitoring, owner assignments, and accepted-risk records where needed.
  - Validation: Security review confirms each listed threat has tests, monitoring, controls, incident paths, or explicit accepted risk.

- **10.3 Harden reliability and scale behavior.**
  - Design: Run drills for delivery backlog, notification outage, bridge reconnect storm, spam flood, org assignment backlog, encrypted grant failure spike, attachment scan backlog, triage permission revocation burst, abuse queue surge, usage reconciliation lag, retention cleanup lag, hold release backlog, export backlog, and replay backlog.
  - Output: Drill plans, expected behavior, actual behavior, evidence refs, metrics thresholds, remediation tasks, and release gates.
  - Validation: Drills prove Messaging degrades without losing envelopes, leaking private content, skipping usage/audit refs, bypassing recipient policy, serving stale projections, or granting downstream services final authority.

- **10.4 Validate documentation and implementation handoff readiness.**
  - Design: Recheck SDS/service/build-plan links, 10 sub-build phases, work-item structure, local Markdown links, queue state, stack guardrails, privacy/abuse/compliance boundary scans, and Docdex retrieval.
  - Output: Validation evidence in `docs/build_plan/progress.md`, queue progress update, Docdex index refresh, Docdex search evidence, and run-test blocker note if unchanged.
  - Validation: Focused scripts pass for title prefix, attached SDS link, phases 1 through 10, Design/Output/Validation work-item structure, local links, final newlines, no unresolved markers, and stack guardrails.

- **10.5 Hand off to Phase 12 implementation and Phase 13 hardening.**
  - Design: Summarize the minimum buildable Phase 12 slice and the later Phase 13 hardening scope so implementation can start without re-litigating ownership, stack choices, external bridges, retention classes, encrypted-message search, legal holds, AI triage permissions, or abuse-control boundaries.
  - Output: Handoff checklist covering contracts, schemas, APIs, inboxes, threads, messages, delivery, org routing, notifications, encrypted payloads, attachments, triage, abuse, usage, audit, replay, client surfaces, and hardening gates.
  - Validation: Review confirms the plan remains internally consistent, consistent with SDS #71, consistent with service catalog and master Phase 0-13 docs, and consistent with `docs/overrid_tech_stack_choice.md`.
