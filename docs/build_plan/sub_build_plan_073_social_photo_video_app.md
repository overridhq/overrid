# SUB BUILD PLAN #73 - Social Photo/Video App

Attached SDS: [SDS #73 - Social Photo/Video App](../sds/native_apps/social_photo_video_app.md)

## Purpose

This sub-build plan turns SDS #73 into an implementation sequence for Social Photo/Video App. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Social Photo/Video App is the Phase 12 native media and community utility for upload intents, media asset refs, processing refs, post records, albums/collections, follows, groups, feed state, visibility controls, comments, reactions, rights/attribution refs, recommendation controls, moderation refs, abuse reports, usage refs, audit refs, and replay projections. It does not own raw object storage, private direct messages, identity truth, search ranking authority, payment/accounting truth, final fraud authority, final reputation scores, final claim outcomes, ad marketplaces, paid reach, or behavioral-profiling engines.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #73: Social Photo/Video App](../sds/native_apps/social_photo_video_app.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoffs, and resolved open-question decisions. |
| [Social Photo/Video App service plan](../service_catalog/native_apps/social_photo_video_app.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency keys, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Overclaim disputes, Fraud Control evidence, Challenge Task facts, Reputation/Anti-Sybil recommendations, deny-by-default behavior, and replayable reason codes. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill accounting truth, Wallet/Usage Center receipt refs, and the rule that Social emits usage refs but never mutates balances, bids, paid reach, pricing, invoices, or payout records. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies AI Gateway Router, Personal AI Assistant, encrypted RAG, adapter, and runtime-bridge groundwork used for permissioned caption/accessibility/report assistance, mobile/local-first proposals, and bounded tool handoffs. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase app state, Overstore media/object refs, Overvault private group/sensitive media grants, Universal Namespace refs, retention, backup/restore, and replay substrates. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public abuse-control, public-provider fraud, reputation, anti-Sybil, challenge, appeal, throttling, and low-sensitivity public-pool prerequisites used before broad social exposure. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first build point for Social Photo/Video App and its first useful upload, post, visibility, following/chronological feed, group, rights, moderation, search/messaging handoff, usage, audit, and replay slice. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal social privacy, age/safety, moderation, rights, retention, recommendation, incident response, threat review, public reporting, reliability, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #73 first build work aligned to master Phase 12, with Phase 13 as governance/security/compliance/reporting hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and contracts, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where a service boundary exists, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript for client surfaces only, native Overrid service boundaries, and no conventional database/object-store/vault/search/social platform, Kubernetes-first, blockchain, NFT, ad-marketplace, paid reach, pricing, revenue, customer-count, hidden profiling, private-content bypass, or raw-media storage drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 11, 12, and 13 | Attach SDS #73, preserve Phase 12 as first build, record prerequisites, and freeze Social ownership boundaries. |
| 2 | Master Phases 0, 1, 4, 5, 8, and 12 | Define Rust contracts, canonical schemas, lifecycle enums, event surfaces, stable errors, and deterministic fixtures. |
| 3 | Master Phases 1, 4, 5, 8, and 12 | Implement upload intents, Overstore media refs, processing callbacks, drafts, publish/edit/archive/remove/read APIs, post versions, and audit/usage refs. |
| 4 | Master Phases 1, 4, 8, 11, and 12 | Implement follows, groups, memberships, visibility rules, comments, reactions, block/mute/hide controls, private grants, and tenant/app isolation. |
| 5 | Master Phases 4, 8, 11, 12, and 13 | Implement following-only and chronological v0 feeds, profile/group/album feeds, recommendation controls, explanation refs, volume limits, anti-addiction policy, and no-paid-reach guardrails. |
| 6 | Master Phases 6, 8, 12, and 13 | Implement Search index update refs, Messaging notification/contact handoffs, Personal AI permissions, mobile upload/playback/offline draft surfaces, SDK/CLI/admin views, and client projections. |
| 7 | Master Phases 4, 5, 8, 10, 12, and 13 | Implement rights_attribution_ref, repost/remix permissions, public-interest media manifests, Purpose Tag refs, Overasset/namespace refs, Overclaim holds, takedowns, exports, and contested-source behavior. |
| 8 | Master Phases 4, 11, 12, and 13 | Implement age/safety profiles, safety scans, quarantine, reports, moderation actions, Fraud/Reputation/Overclaim handoffs, appeal/correction paths, spam/follow/comment/reaction abuse controls, and steward queues. |
| 9 | Master Phases 5, 8, 12, and 13 | Implement usage refs, Wallet/Usage Center projections, classed retention, Overvault grant revocation, exports/imports, observability, audit exports, replay, compliance holds, and cleanup. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, social/privacy/rights/moderation boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Social Photo/Video App uses Rust-first shared contracts and service-facing APIs for upload intents, media asset refs, processing jobs, post records, albums/collections, follows, groups, memberships, feeds, visibility controls, comments, reactions, rights/attribution refs, recommendation controls, moderation records, abuse reports, usage refs, audit refs, and replay bundles. TypeScript is acceptable for generated client bindings and native/web UI surfaces, but it must call Overrid APIs and must not become a privileged social authority.
- Social contracts, fixtures, event records, policy snapshots, replay bundles, retention classes, redaction profiles, age/safety profiles, rights/takedown records, recommendation controls, export manifests, and compliance-hold records use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/app/org/group scope where applicable, owner or role authority, trace id, idempotency key, media/storage refs where applicable, visibility refs, rights/consent refs where needed, policy refs, reason codes, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for upload/session hashes, media ref display hashes, processing output refs, fixture inputs, post/version/replay bundles, retention manifests, export manifests, audit bundles, and deterministic comparisons.
- Structured state, raw media objects, encrypted private grants, identity, namespace, key status, policy, audit, usage, accounting refs, dispute refs, diagnostics, AI route refs, search refs, notification/contact refs, and replay must use native Overrid service boundaries such as Overbase, Overstore, Overvault, Overpass, Overtenant, Overkey, Universal Namespace Service, Overgate, Overguard, Overwatch, Overmeter, ORU Account Service, Seal Ledger, Wallet/Usage Center, Search Engine, Messaging Center, Personal AI Assistant, AI Gateway Router, Fraud Control, Reputation and Anti-Sybil Service, Overclaim, Purpose Tag Registry, Compliance Boundary Service, Mobile SDK, Mobile Backend Gateway, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Elasticsearch, OpenSearch, Solr, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, external social networks, external ad networks, paid reach, ad auctions, creator payout scheduling, hidden behavioral profiling, engagement-maximization ranking, pricing, revenue forecasts, customer-count assumptions, blockchain, NFTs, raw media blobs, raw private group material, direct messages, global identity truth, final fraud/reputation/dispute authority, direct ORU/Seal Ledger mutation, or hidden moderation action the Social boundary.

## Phase 1: SDS Attachment, Phase 12 Scope, And Social Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #73.**
  - Design: Link this document from the Social Photo/Video SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/native_apps/social_photo_video_app.md`, `docs/service_catalog/native_apps/social_photo_video_app.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #73 returns both the Social Photo/Video SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep first implementation in Phase 12 because Social is a native utility built on identity, namespace, storage, vault, policy, metering, accounting, public abuse-control, search, messaging, mobile, and AI/tool rails that earlier phases provide.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 6, 8, and 11 supply prerequisites; Phase 12 builds the first useful social media utility; Phase 13 hardens age/safety, moderation, rights, retention, recommendations, incident response, reporting, compliance, and scale.
  - Validation: Review proves the plan does not move Social into Phase 8 storage, Phase 11 public-provider controls, Phase 13-only governance, Search ranking, Messaging private messages, identity, or payment/accounting ownership.

- **1.3 Freeze the Social ownership boundary.**
  - Design: Record that Social owns upload intents, media processing refs, post drafts, published posts, post versions, albums, follows, groups, feed state, visibility controls, comments, reactions, rights/attribution refs, recommendation controls, moderation refs, abuse reports, usage refs, audit refs, and replay projections.
  - Output: Ownership checklist for architecture, API, UI, implementation, operations, and review gates.
  - Validation: Review confirms Social does not own raw media object storage, private direct messages, global identity truth, search ranking authority, payment balances, paid reach, final fraud verdicts, final reputation scores, final claim outcomes, or central AI governance decisions.

- **1.4 Carry forward resolved SDS #73 decisions.**
  - Design: Preserve following-only and chronological v0 feeds, jurisdictional age/safety launch profiles, rights_attribution_ref requirements for repost/remix/public-interest media, and classed retention defaults.
  - Output: Resolved-decision checklist covering feed launch order, local/community pilot gates, recommendation-control requirements, age/guardian/consent policy profiles, minor-safe defaults, Search exclusion for restricted material, rights refs, Overclaim holds, and deletion/quarantine/retention classes.
  - Validation: Review rejects broad algorithmic launch without controls, broad public launch without jurisdictional age/safety profiles, repost/remix/public-interest media without rights refs, and one hardcoded deletion window.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Record how Overstore, Overbase, Overvault, Overpass, Universal Namespace Service, Overguard, Overwatch, Overmeter, Wallet/Usage Center, Search Engine, Messaging Center, Personal AI Assistant, AI Gateway Router, Fraud Control, Reputation/Anti-Sybil, Overclaim, Purpose Tag Registry, Compliance Boundary, Mobile SDK, Mobile Backend Gateway, SDK, CLI, and Admin/Developer UI interact through refs.
  - Output: Boundary matrix naming allowed reads, owned writes, handoff refs, denied direct mutation, audience rules, visibility rules, usage refs, audit refs, replay refs, moderation paths, and owner-service finality.
  - Validation: Review confirms each upstream/downstream service keeps its authority and Social exchanges refs/events rather than copying raw private content or inventing canonical truth owned elsewhere.

## Phase 2: Contracts, Schemas, Events, Stable Errors, And Fixtures

### Work Items

- **2.1 Create the Social Rust contract module.**
  - Design: Add contract types for upload intents, media asset refs, processing jobs, posts, post versions, albums, follows, groups, memberships, feeds, comments, reactions, rights/attribution refs, recommendation controls, moderation records, abuse reports, usage refs, replay bundles, retention classes, redaction profiles, age/safety profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, visibility-class enums, media-type enums, feed-type enums, recommendation-control enums, report-class enums, retention-class enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from object storage, vault secret handling, identity, search ranking, messaging payloads, accounting mutation, and final dispute authority.

- **2.2 Define canonical JSON and JSON Schema contracts.**
  - Design: Model all Social records with trace ids, idempotency keys, actor/org/group/app refs, tenant scope, state, visibility class, policy refs, rights refs where needed, audit refs, reason codes, usage refs where applicable, and stable schema versions.
  - Output: JSON Schema files, valid examples, invalid examples, signed command examples, event examples, replay examples, export examples, recommendation-control examples, age/safety profile examples, and retention/hold examples.
  - Validation: Schema tests reject records without required owner refs, visibility refs, policy refs, audit refs, state, trace id, idempotency key where mutating, and schema version.

- **2.3 Define event and replay contracts.**
  - Design: Model upload, processing, draft, publish, version, remove, restore, feed, follow, group, membership, comment, reaction, recommendation-control, rights, report, moderation, search update, notification handoff, usage, export, retention, and replay events without unauthorized private media or direct-message content.
  - Output: Event schema set, replay bundle schema, BLAKE3 display hash rules, redaction profile refs, stable ordering, and fixture-backed event streams.
  - Validation: Tests prove events include necessary refs and reason codes while excluding unauthorized private media, vault secrets, exact private location, direct messages, payment data, raw hidden profiles, and hidden moderation thresholds.

- **2.4 Define stable error taxonomy.**
  - Design: Preserve SDS stable errors and add implementation-ready mapping for media refs, processing state, visibility, group role, post policy, comment policy, rights claim, recommendation control, unsafe media, moderation, search visibility, retention, age/safety profile, and post state conflicts.
  - Output: Stable error registry, HTTP/API mapping, client-facing messages, support-safe diagnostics, retryability flags, and replay refs.
  - Validation: Tests prove denials are deterministic, support-safe, tenant-safe, audience-safe, and replayable without exposing private membership state, hidden abuse signals, raw media, or adversarial thresholds.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for upload intent, media processing, draft/publish/edit/archive/remove/restore/read, follow, group, comment, reaction, chronological feed, recommendation-control update, rights claim, abuse report, moderation action, search update, notification handoff, AI permission, usage emission, retention, export, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected errors, BLAKE3 hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, redacted outputs, omitted/denied counts, and replay output across repeated runs.

## Phase 3: Upload Intents, Media Processing, Posts, Versions, And Media Lifecycle

### Work Items

- **3.1 Implement media upload intent records.**
  - Design: Create upload intents with actor/org/client refs, media type, expected size/duration, destination visibility, processing profile, storage policy refs, safety policy refs, trace id, idempotency key, and state.
  - Output: `media_upload_intent` schema, `POST /social/media/upload-intents`, state transitions, Overstore upload session refs, cancellation/resume behavior, policy snapshot refs, and audit events.
  - Validation: Tests prove upload intents require signed actor/service envelopes, tenant/app scope, storage policy refs, media class, expected size/duration, trace id, idempotency key, and Overwatch audit refs.

- **3.2 Implement media asset refs and processing callbacks.**
  - Design: Reference media through Overstore object refs and capture thumbnails, transcodes, accessibility assets, hash refs, safety scan refs, retention class, processing worker refs, usage refs, and failure refs.
  - Output: `media_asset_ref` and `media_processing_job` schemas, authorized callback API, processing states, output variant refs, unsafe/quarantine states, and usage emission hooks.
  - Validation: Tests prove Social never stores raw media blobs, processing callbacks require authorized worker refs, unsafe media cannot preview/publish, and every processing decision is replayable.

- **3.3 Implement draft and publish APIs.**
  - Design: Create post drafts with media refs, caption/text refs, album/group/profile refs, visibility rules, rights refs, recommendation settings, and policy refs, then publish only after identity, ownership, group role, visibility, safety, rights, policy, and media readiness checks pass.
  - Output: `media_post` draft/publish schemas, `POST /social/posts/drafts`, `POST /social/posts/{post_id}/publish`, publish events, search/feed/notification refs, and stable errors.
  - Validation: API tests prove missing media readiness, missing rights refs, visibility conflicts, group-role mismatch, unsafe media, and policy denials block publish without widening visibility.

- **3.4 Implement post version, edit, archive, remove, restore, and read behavior.**
  - Design: Make edits append-only through `post_version`, preserve previous refs, record changed fields, allow archive/remove/restore where policy permits, and serve reads through role/visibility/redaction checks.
  - Output: `post_version` schema, `PATCH /social/posts/{post_id}`, archive/remove/restore APIs, tombstone states, owner/moderator authority checks, and redacted read projections.
  - Validation: Tests prove post mutations are idempotent, audited, versioned, policy-scoped, and cannot erase required tombstone, rights, usage, moderation, or audit refs.

- **3.5 Implement album and collection records.**
  - Design: Support owner-scoped albums/collections with post refs, visibility, ordering, collaboration refs, group/profile refs, state, and audit refs without turning albums into object storage.
  - Output: `album_collection` schema, create/update/reorder/archive APIs, collaborator role checks, visibility projections, and feed/search update refs.
  - Validation: Tests prove albums cannot expose posts outside post visibility, collaborators require explicit refs, deleted posts are suppressed, and album replay reconstructs ordering and visibility decisions.

## Phase 4: Visibility, Follows, Groups, Comments, Reactions, And Isolation

### Work Items

- **4.1 Implement visibility rule evaluation.**
  - Design: Evaluate public, restricted, group, private, profile, organization, hidden, and under-review visibility through actor/org/group refs, Overvault grants where needed, policy refs, redaction profiles, and deny-by-default missing-fact handling.
  - Output: Visibility evaluator, policy snapshot refs, `visibility_denied` and `search_visibility_denied` behavior, redacted projections, and audit events.
  - Validation: Tests prove private groups, restricted posts, comments, reactions, and media refs cannot be indexed, recommended, read, exported, or handed off outside allowed audience.

- **4.2 Implement follows and social graph records.**
  - Design: Support follow/unfollow/mute/block notification preferences with follower refs, followee actor/org/profile refs, visibility, abuse throttle refs, state, and audit refs without making the graph the global identity system.
  - Output: `social_follow` schema, `POST /social/follows`, mute/block states, notification preference refs, follow events, and abuse throttling hooks.
  - Validation: Tests prove follows cannot bypass identity, block, mute, tenant, profile, or abuse throttles and cannot expose hidden follow state to unauthorized users.

- **4.3 Implement groups and membership records.**
  - Design: Create groups with owner/admin refs, membership rules, visibility, posting/comment rules, moderation policy refs, Overvault grant refs for private groups, lifecycle state, and audit refs.
  - Output: `social_group` and `group_membership` schemas, group create/update API, member invite/accept/update/remove/mute/ban API, role projections, and group events.
  - Validation: Tests prove private group content requires active membership/grants, role changes are audited, banned users lose access, and stale grants deny future reads and comments.

- **4.4 Implement comments and threaded replies.**
  - Design: Support comments and replies with post refs, author refs, parent refs, text refs, visibility, moderation refs, edit history refs, state, and policy checks.
  - Output: `comment_record` schema, `POST /social/posts/{post_id}/comments`, edit/remove behavior, comment events, redacted projections, and moderation hooks.
  - Validation: Tests prove comments obey post/group visibility, block/mute state, comment policy, safety checks, and moderation states without becoming private messages.

- **4.5 Implement reactions and interaction abuse throttles.**
  - Design: Support reactions with post/comment refs, actor refs, reaction type, visibility, abuse throttle refs, state, and replayable updates.
  - Output: `reaction_record` schema, `POST /social/posts/{post_id}/reactions`, update/remove behavior, abuse throttle refs, reaction events, and usage hooks.
  - Validation: Tests prove reaction spam, duplicate idempotency, hidden actors, restricted posts, blocked users, and private groups are handled without leaking private audience membership.

## Phase 5: Feeds, Recommendation Controls, Transparency, And Anti-Addiction Guardrails

### Work Items

- **5.1 Implement following-only and chronological v0 feeds.**
  - Design: Build v0 feeds around following-only and chronological profile/group/album sources with explicit visibility filtering before ranking, snippets, previews, usage, and replay.
  - Output: `feed_record` schema, `GET /social/feeds/{feed_id}`, source refs, visibility-filter refs, chronological ordering, pagination refs, denied/hidden refs, and feed events.
  - Validation: Tests prove feed fallback never expands visibility, denied posts do not appear in pagination or replay views for unauthorized callers, and chronological ordering is deterministic.

- **5.2 Implement profile, group, and album feed surfaces.**
  - Design: Serve profile, group, and album feeds from owned Social state with explicit profile/group/album refs, audience rules, blocked/hidden refs, stale markers, and redaction profiles.
  - Output: Feed source contracts, profile/group/album feed APIs, source-specific pagination, hidden/muted filtering, and replay refs.
  - Validation: Tests prove profile/group/album feeds cannot bypass post, group, membership, block/mute, moderation, rights, or Search exclusion rules.

- **5.3 Implement recommendation controls as opt-in records.**
  - Design: Require active `recommendation_control` records before transparent recommendations can run, with allowed signals, blocked signals, time/volume limits, sensitive-topic controls, explanation settings, expiry, and revocation state.
  - Output: `recommendation_control` schema, `POST /social/recommendation-controls`, default-off state, control update events, user-visible settings, and replay refs.
  - Validation: Tests prove recommendations deny by default without active controls and revocation stops future recommended feed generation and invalidates stale recommendation refs.

- **5.4 Implement ranking explanations and anti-addiction policy.**
  - Design: Expose audience-safe explanation classes for feed source, follow/group/profile relation, recency, user controls, hidden/muted state, safety/moderation class, diversity/deduplication, and no-paid-reach/no-addiction policy without exact weights or adversarial internals.
  - Output: Ranking explanation schema, `GET /social/feeds/{feed_id}/explain`, explanation policy, no-paid-reach attestation, anti-addiction policy refs, and audit events.
  - Validation: Tests reject paid reach, ad budgets, hidden engagement-maximization goals, outrage amplification, hidden behavioral profiles, exact threshold leaks, and explanation data that reveals private audience membership.

- **5.5 Implement local/community feed pilot gates.**
  - Design: Allow local/community feed pilots only when Directory, Maps, Search, coarse locality refs, source attribution, Overguard policy, locality privacy, recommendation controls, and replay evidence are present.
  - Output: Pilot gate contract, locality/ref requirements, approval refs, denied pilot state, monitoring refs, and rollback controls.
  - Validation: Tests prove missing, stale, contested, or precise/private locality refs deny broad local/community feed exposure and fall back to following/chronological feeds.

## Phase 6: Search, Messaging, Assistant, Mobile, SDK, CLI, And Admin Handoffs

### Work Items

- **6.1 Implement Search Engine index update refs.**
  - Design: Emit search update refs for public/authorized posts, profiles, albums, groups, and tombstones only after visibility, moderation, rights, and Search source policy checks pass.
  - Output: Search handoff schema, index update events, tombstone/removal refs, redaction class refs, denied search states, and retry/reconciliation jobs.
  - Validation: Tests prove restricted posts, private groups, unsafe media, rights-held media, and moderation-limited content cannot appear in Search without explicit authorized source policy.

- **6.2 Implement Messaging Center notification and contact handoffs.**
  - Design: Route notifications, private contact requests, reports, moderation notices, group invites, organization handoffs, and appeal notices through Messaging refs without storing private messages in Social.
  - Output: Notification/contact handoff schema, target inbox/thread refs, consent refs, redacted payload refs, denied handoff state, and audit events.
  - Validation: Tests prove handoffs cannot copy private message bodies, bypass first-contact preferences, expose private group membership, or override Messaging delivery/retention authority.

- **6.3 Implement Personal AI Assistant permissions.**
  - Design: Allow caption drafting, accessibility text, feed/post summarization, search assistance, report drafting, rights/takedown drafting, and moderation appeal drafting only through explicit permission records with source scopes, redaction class, expiry, revocation, and approval rules.
  - Output: Assistant permission schema, tool-call refs, AI Gateway Router route refs where model execution is needed, cached-output invalidation, usage refs, and replay events.
  - Validation: Tests prove AI access is denied by default, revoked permissions stop future access, and media/post/comment/social graph data is not used for training or broad context without explicit source and user permission.

- **6.4 Implement mobile upload, playback, offline drafts, and notification surfaces.**
  - Design: Expose Mobile SDK and Mobile Backend Gateway projections for upload sessions, offline drafts, playback refs, compact feed deltas, notification refs, privacy controls, and idempotent replay.
  - Output: Mobile contracts, offline intent envelopes, upload/playback refs, sync cursor refs, redacted push payloads, stale markers, and reconnect validation.
  - Validation: Tests prove mobile/offline paths cannot widen media, group, location, AI, notification, or post visibility and all replayed commands preserve original policy snapshots.

- **6.5 Implement SDK, CLI, source-owner, moderator, and admin surfaces.**
  - Design: Provide generated bindings and support/admin projections for Social APIs, diagnostics, moderation queues, rights queues, age/safety profile state, usage refs, replay refs, and support-safe audit views.
  - Output: Rust SDK bindings, TypeScript generated bindings for client surfaces, CLI command specs, Admin/Developer UI projections, moderator/source-owner dashboards, and support-safe replay links.
  - Validation: Tests prove clients use signed Overgate APIs, propagate trace ids and idempotency keys, respect redaction profiles, and cannot use privileged internal media, vault, moderation, search, messaging, or accounting APIs.

## Phase 7: Rights, Attribution, Reposts, Remixes, Public-Interest Media, And Claims

### Work Items

- **7.1 Implement rights_attribution_ref records.**
  - Design: Attach `rights_attribution_ref` to source `media_asset_ref`, `media_post`, and derived post records with creator/source refs, license/consent refs, repost/remix permissions, takedown refs, claim refs, and state.
  - Output: Rights/attribution schema, attach/update APIs, rights events, source linkage, license/consent validation, and replay refs.
  - Validation: Tests prove publish/repost/remix/export paths fail when required rights refs are missing, stale, revoked, or contested.

- **7.2 Implement repost and share behavior.**
  - Design: Support reposts with source post refs, creator/owner refs, source audience refs, repost permission refs, revocation refs, takedown refs, and visibility no-widening checks.
  - Output: Repost schema, repost API, source/derived linkage, revocation propagation, takedown propagation, search/feed update refs, and events.
  - Validation: Tests prove reposts cannot widen source audience, strip attribution, evade takedowns, bypass blocks, or continue serving after revocation.

- **7.3 Implement remix and derivative media behavior.**
  - Design: Support remixes only with explicit derivative permission, transform provenance, source linkage, creator/owner refs, claim refs, safety scans, and policy checks.
  - Output: Remix schema, derivative permission validator, transform provenance refs, source/derived replay links, claim refs, and moderation hooks.
  - Validation: Tests prove remixes cannot publish without derivative permission, cannot hide source linkage, and inherit rights/claim/moderation constraints where policy requires.

- **7.4 Implement public-interest media governance.**
  - Design: Require steward/source refs, Purpose Tag Registry tag versions, data class, evidence refs, license/rights evidence, Overasset or namespace refs where available, and Overguard/Compliance checks before public-interest media can publish or be broadly discovered.
  - Output: Public-interest media manifest schema, steward/source approvals, Purpose Tag refs, evidence refs, compliance check refs, publication state, and search/feed policy refs.
  - Validation: Tests reject public-interest media without required steward/source approval, data class, rights evidence, Purpose Tag refs where applicable, Overguard/Compliance checks, and replay evidence.

- **7.5 Implement takedown, contested ownership, and claim holds.**
  - Design: Move contested or claim-held media to limited visibility, metadata-only, review, or hold states through Overclaim, Overguard, Overwatch, and Compliance Boundary refs without Social adjudicating final ownership.
  - Output: Claim/takedown state machine, limited visibility behavior, metadata-only state, Overclaim refs, Overwatch evidence refs, appeal refs, source-owner notices, and denied serving states.
  - Validation: Tests prove Social does not decide final claim outcomes, does not continue broad serving during contested states, and preserves audit/appeal/correction evidence.

## Phase 8: Moderation, Abuse, Age/Safety, Fraud, Reputation, And Appeals

### Work Items

- **8.1 Implement jurisdictional age and safety profiles.**
  - Design: Require jurisdictional age/safety policy profiles before broad public launch with age/guardian/consent checks where legally required, minor-safe visibility defaults, sensitive-media labels, rate limits, report paths, appeal paths, Messaging contact restrictions, Search exclusion rules, and stable Overguard reason codes.
  - Output: `social_age_safety_profile` schema, profile activation state, launch gate checks, profile stale/contested states, denial reasons, and audit events.
  - Validation: Tests prove broad public discovery and recommendations deny when the local profile is missing, stale, or contested, while private/group-only or review-gated pilots can remain policy-scoped.

- **8.2 Implement media safety scan and quarantine workflows.**
  - Design: Gate previews and publish on safety scan refs, media type, sensitive labels, age/safety profile, policy refs, reviewer refs, and quarantine states.
  - Output: Safety scan refs, quarantine states, review queue refs, limited visibility states, appeal/correction refs, and media safety events.
  - Validation: Tests prove quarantined media cannot publish/preview/search/recommend and restores require authorized review, policy refs, and replay evidence.

- **8.3 Implement abuse, spam, impersonation, privacy, and unsafe-content reports.**
  - Design: Record reports with reporter refs, target refs, report class, evidence refs, safety/fraud/reputation refs, Overclaim refs where relevant, state, and audience-specific redaction.
  - Output: `social_abuse_report` schema, `POST /social/reports`, report lifecycle, evidence refs, reporter protections, and audit events.
  - Validation: Tests prove reports are replayable, audience-safe, tenant-safe, and do not leak private group content, hidden reporter state, direct messages, or hidden detection thresholds.

- **8.4 Implement moderation actions and appeal/correction paths.**
  - Design: Support hide, limit, quarantine, remove, tombstone, restore, warn, mute, ban, group restrict, profile restrict, and correction actions with reviewer refs, reason codes, evidence refs, appeal refs, and replay.
  - Output: `social_moderation_record` schema, `POST /social/moderation-actions`, state transitions, appeal/correction refs, reviewer projections, and events.
  - Validation: Tests prove moderation actions are authorized, reason-coded, reversible where policy allows, audience-redacted, and not hidden final authority without appeal/correction paths.

- **8.5 Implement Fraud Control, Reputation, Anti-Sybil, and Overclaim handoffs.**
  - Design: Route suspicious follows, comment/reaction spam, coordinated abuse, impersonation, rights disputes, public-interest misuse, recommendation manipulation, and unsafe media evidence to owning fraud/reputation/dispute services through refs.
  - Output: Fraud Control refs, Reputation/Anti-Sybil refs, Overclaim refs, challenge refs, bounded recommendation refs, moderation refs, source action refs, appeal refs, and audit refs.
  - Validation: Tests prove Social does not own final fraud, final reputation, final claim outcome, final challenge consequence, or final appeal authority.

## Phase 9: Usage, Retention, Export, Observability, Replay, And Compliance Holds

### Work Items

- **9.1 Implement Social usage refs.**
  - Design: Emit usage refs for uploads, storage, processing, thumbnails/transcodes, post publish/edit, feed generation, comments, reactions, search updates, notifications, moderation, reports, AI assistance, exports, replay, compute, bandwidth, and retention.
  - Output: `social_usage_ref` schema, Overmeter refs, Wallet/Usage Center receipt refs, usage pending/reconciliation state, and `social_photo_video.usage_emitted` events.
  - Validation: Tests prove Social emits usage refs without maintaining balances, bids, paid reach, creator payout schedules, invoices, provider payouts, ledger truth, hardcoded prices, revenue forecasts, or per-transaction external payment rails.

- **9.2 Implement classed media retention and deletion behavior.**
  - Design: Apply classed retention defaults for owner-deleted posts, moderation removals, unsafe media, rights disputes, private-group media, sensitive media, exports, and compliance holds.
  - Output: Retention resolver, immediate serving revocation, feed/search/notification revocation refs, redacted tombstones, Overstore purge eligibility refs, Overvault grant revocation, sealed hold refs, and cleanup jobs.
  - Validation: Tests prove owner deletion stops serving immediately, moderation removals retain only redacted evidence until review/appeal closure, rights disputes hold required refs, private/sensitive media revoke grants immediately, and one hardcoded deletion window is rejected.

- **9.3 Implement export, import, and portability boundaries.**
  - Design: Support user/org exports and imports for posts, captions, refs, rights records, albums, follows, groups, comments, reactions, controls, reports, usage refs, and replay refs without exporting unauthorized raw media or third-party private data.
  - Output: Export manifest schema, portability bundle refs, import preflight, redaction profiles, rights/export constraints, hold interactions, and audit events.
  - Validation: Tests prove exports contain only authorized refs and redacted payloads, respect rights/claims/holds, and do not expose vault secrets, private groups, direct messages, or unrelated profile data.

- **9.4 Implement observability and operations metrics.**
  - Design: Expose upload success, processing latency, safety scan denials, publish latency, feed latency, recommendation-control usage, visibility denials, group moderation backlog, report volume, rights-claim volume, search update lag, notification failure rate, usage emission status, and replay backlog.
  - Output: Metrics, dashboards, alert rules, moderator/source-owner diagnostics, support-safe diagnostics, incident trigger refs, and replay links.
  - Validation: Tests prove metrics are tenant-safe and audience-safe and do not leak private media, hidden group membership, precise private location, direct messages, payment data, or hidden abuse thresholds.

- **9.5 Implement replay, audit export, and compliance-hold boundaries.**
  - Design: Reconstruct upload, processing, publish, feed, visibility, recommendation controls, comments, reactions, moderation, reports, rights, retention, handoffs, and usage decisions from stored refs while coordinating sealed holds through Compliance Boundary.
  - Output: `GET /social/replay/{record_id}`, replay bundle schema, Overwatch audit export refs, redacted replay profiles, hold refs, support-safe replay views, and cleanup interactions.
  - Validation: Tests prove replay is deterministic, audience-scoped, redacted by policy, sufficient to explain decisions, and never exposes held or unauthorized raw content.

## Phase 10: Validation, Governance Handoff, Scale Readiness, And Documentation Closure

### Work Items

- **10.1 Validate sub-build plan structure and backlinks.**
  - Design: Check title prefix, attached SDS link, 10 phases numbered 1 through 10, five work items per phase, Design/Output/Validation fields, and local Markdown links.
  - Output: Validation evidence in `docs/build_plan/progress.md`, queue state/progress updates, and Docdex search/index evidence.
  - Validation: Focused checks pass for structure, links, final newline, no tabs, queue JSON, and Docdex retrieval of SDS #73 plus the sub-build plan.

- **10.2 Validate stack and authority guardrails.**
  - Design: Check that the plan preserves Rust-first core services, canonical JSON plus JSON Schema, Ed25519, BLAKE3, native Overrid boundaries, Phase 12 first build, Phase 13 hardening, anti-addiction/no-paid-reach policy, and no external social/database/object-store/vault/search boundary drift.
  - Output: Guardrail scan results and explicit negative-control review for forbidden product-boundary, pricing, revenue, customer-count, blockchain/NFT, ad-marketplace, paid reach, raw-media-storage, direct-message, private-content, and final-authority terms.
  - Validation: Review passes with only expected negative-control wording that explicitly rejects those assumptions.

- **10.3 Validate master Phase 0 through Phase 13 alignment.**
  - Design: Confirm master plan, Phase 12, Phase 13, service catalog crosswalk, SDS, and service catalog entry all agree that Social first builds in Phase 12 with earlier prerequisites and Phase 13 governance/security/compliance/scale hardening.
  - Output: Updated master/crosswalk rows, Phase 12 Social workstream details, Phase 13 Social threat/security/reliability/reporting coverage, and backlinks from SDS/service files.
  - Validation: Review confirms no master phase reordering is required and no Social work is incorrectly moved into storage, messaging, search, AI, public-provider, governance-only, or accounting ownership.

- **10.4 Validate implementation handoff readiness.**
  - Design: Ensure implementation teams can derive target contracts, APIs, events, fixtures, policy checks, visibility checks, storage/vault refs, feed controls, rights flows, handoffs, abuse flows, retention, usage, replay, and client surfaces from the plan without inventing missing authority boundaries.
  - Output: Handoff checklist covering Rust modules, schemas, APIs, events, fixtures, Overstore/Overbase/Overvault refs, visibility filters, feeds, recommendations, rights, age/safety, moderation, assistant/tool boundaries, retention, observability, and replay.
  - Validation: Review confirms each SDS build-breakdown item maps to at least one phase/work item and each work item has a testable output.

- **10.5 Validate documentation and queue closure.**
  - Design: Refresh Docdex index for changed docs, search for SDS #73 and the sub-build plan, export DAG evidence where available, update progress notes, and preserve the known `docdexd run-tests` blocker if no runner is configured.
  - Output: Docdex index/search evidence, DAG evidence, validation command evidence, queue state marking `073-build-plan` complete, and progress notes listing the next incomplete build-plan task.
  - Validation: `git diff --check`, local link checks, JSON checks, stale-marker checks, stack guardrail checks, Docdex index/search, and configured repo test command status are recorded before handoff.
