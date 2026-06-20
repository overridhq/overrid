# Phase 12: Native Application Layer

## Objective

Build native public utilities on top of Overrid as ordinary clients of the same identity, policy, storage, routing, metering, privacy, and accounting rails.

Native apps are non-profit oriented public utilities. They should charge for resource usage where needed, keep cost and revenue close, and route surplus through central AI stewardship rules rather than private extraction.

## Depends On

- Phase 9 deployment platform.
- Phase 8 data, storage, vault, namespace, and route substrate.
- Phase 5 ORU, Seal Ledger, and Overbill.
- Phase 4 policy and abuse controls.
- Phase 6 product, AI/RAG, and mSwarm Runtime Bridge groundwork where local-first native apps need runtime sessions, sync, discovery, collaboration, cloud coordination hooks, or offline handoffs.
- Phase 11 public low-sensitivity pool where appropriate.

## Build Order

1. Wallet and usage center.
2. Personal native AI assistant.
3. Workspace and office suite.
4. Directory listings.
5. Search engine.
6. Messaging center.
7. Social photo/video app.
8. Maps and navigation.
9. Central AI stewardship interface and governance console.
10. Mobile service layer for approved native and third-party mobile apps.

## Workstream 1: Wallet And Usage Center

Build first because every native service needs user-visible account, usage, receipt, permission, privacy, and dispute controls.

The first useful Wallet and Usage Center slice includes:

- Wallet profiles, account selectors, balance views, usage dashboards, usage line items, receipt collections, statement/export jobs, app permission controls, revocation requests, privacy audit views, dispute handoffs, notification prefs, usage refs, audit refs, and replay bundles.
- ORU Account Service balance projections, Seal Ledger checkpoint refs, Overbill receipt/statement/refund refs, Overgrant grant refs, Overmeter rollup refs, Overclaim dispute refs, and Overmark resource-card refs displayed through wallet-safe projections.
- Account, organization, delegated-accountant, app-owner, support, and steward redaction classes enforced by source-service policy rather than UI-only filtering.
- High-risk permission revocation that fails closed immediately for private-data, spend, credential/session/device, secret/Overvault, AI/RAG context, location/contact/message/workspace/private-media, payment/grant, push-sensitive, child/safety, and compromised access.
- Queued owner-service revocation only for low-risk cleanup permissions with expiry, idempotency, pending state, owner-service refs, user-visible status, and fail-closed escalation if risk changes.
- Read-only mobile/offline snapshots for previously authorized account selectors, last successful projections, receipts/statements, permission inventory, notification prefs, and privacy-audit summaries; live revalidation remains required for budget prechecks, statements, disputes, permission expansion, spend, or accounting-changing actions.
- Personal AI Assistant wallet tools for usage summaries, receipt explanations, permission cleanup suggestions, and dispute drafts only with explicit user permission, redaction classes, expiry, usage refs, audit refs, and replay.
- Usage refs for wallet reads, exports, statements, permission changes, dispute handoffs, privacy audit views, replay, compute, storage, and bandwidth.

Wallet should be the user's control panel for the ecosystem, not ORU Account Service, Seal Ledger, Overbill, Overgrant, Overmeter, Overclaim, payment processing, or resource-rate authority.

## Workstream 2: Personal Native AI Assistant

Build the assistant using:

- Central AI Service evidence-bound recommendations and Central AI Stewardship Interface review queues.
- Encrypted Docdex RAG Adapter context grants, leakage profiles, and bounded context bundles for personal, organization, and repo RAG.
- AI Gateway Router request classification, route decisions, fallback plans, and replay evidence.
- ADES Enrichment Adapter for optional local entity, topic, warning, and domain-pack hints where useful.
- Lightweight Classifier advisory labels for request intent, privacy/data-class hints, RAG need, tool/native-app delegation, model-size/capability hints, confidence, uncertainty, and escalation requirements.
- Larger model routing for complex work.
- Available Overrid model resources.
- User permission and audit controls.
- ORU metering per model/resource operation.

The assistant should direct calls when needed rather than forcing every request through one expensive model.

## Workstream 3: Workspace And Office Suite

Build the Phase 12 Workspace and Office Suite slice from [SUB BUILD PLAN #75 - Workspace and Office Suite](sub_build_plan_075_workspace_office_suite.md):

- Workspace, folder, object, document, structured table, page/presentation, editor session, share permission, version, comment, approval, search handoff, AI assist, import/export, usage, audit, and replay records.
- Overrid-native canonical authoring formats: `workspace_document_v0` for rich text blocks, headings, lists, embeds, comments, and version refs; `workspace_table_v0` for typed rows, columns, formulas, validation, views, and import/export refs; and `workspace_page_v0` for simple pages, slide-like sections, layouts, themes, and embedded Overstore media refs.
- Versioned editing with short-lived editor sessions, optimistic base-version checks, object or section locks for high-conflict/sensitive scopes, append-only patch/proposal records, snapshots, restore refs, conflict branches, and import-review drafts. Full OT/CRDT support remains a later internal engine under the Workspace contract.
- Explicit sharing, workspace roles, invitations, scoped public links, revocation refs, vault/search/cache/AI/offline invalidation, user/org-visible audit, and replayable permission decisions.
- Search Engine source handoffs using source policy, indexable fields, redaction profiles, permission snapshots, tombstones, and update refs. Search owns final indexes/ranking; Workspace owns source policy and object visibility refs.
- Permissioned Personal AI Assistant, AI Gateway Router, and Encrypted Docdex RAG Adapter flows for writing, summarization, table work, document search, and edit proposals. AI output remains proposal/apply/reject state and cannot become hidden training data or direct edits without user/role/policy approval.
- Import/export portability with Overrid-native formats as canonical state, Markdown/plain text, CSV/TSV, HTML, and PDF as low-risk renderings, and OOXML/ODF/legacy office formats as conversion jobs with manifests, redaction profiles, and no executable macros.
- Mobile and offline support limited to permission-snapshot-bound drafts, cached authorized metadata/content refs, local text-block/comment/simple table-cell patches against known base versions, and reconnect revalidation through Mobile Backend Gateway before authoritative state changes.
- Overbase structured state, Overstore object/bundle refs, Overvault private grants, Messaging Center notifications, Wallet/Usage Center usage projections, Overmeter usage refs, Overwatch audit, Compliance Boundary holds, and replay bundles.

This exercises Overbase, Overstore, Overvault, encrypted Docdex RAG, namespace routes, sharing, metering, Wallet/Usage Center, Search Engine, Messaging Center, Mobile SDK, Mobile Backend Gateway, and mSwarm Runtime Bridge handoffs for app-owned local-first collaboration or offline proposals where appropriate. Workspace must remain a native Overrid service boundary, not a conventional office-suite wrapper, raw object store, vault, search engine, direct-message system, AI model provider, or accounting authority.

## Workstream 4: Directory Listings

Build a Craigslist-like native directory:

- Phase 12 low-compliance category allowlist before regulated/high-risk categories.
- Listing categories and category policy refs.
- Locality privacy classes for public place, approximate area, private-to-contact exact refs, and hidden locality.
- Classifieds.
- Local services.
- Jobs.
- Housing.
- Events.
- Community groups.
- Organization and business pages.
- Reputation and dispute records.
- Messaging Center first-contact handoff and organization inbox refs.
- Search Engine index update refs with anti-ad-trap ranking constraints.
- Maps and Navigation locality/place handoff refs without map truth ownership.
- Abuse reports, moderation actions, Fraud Control, Reputation/Anti-Sybil, and Overclaim dispute handoffs.
- Usage refs, wallet handoffs, audit refs, retention/tombstones, and replay.

Start with directory listings before broad social media because the abuse surface is smaller and the utility is immediate.

## Workstream 5: Search Engine

Build search across:

- Public Overrid content.
- Native directory listings.
- Public app pages.
- Public-interest datasets.
- User-authorized private content.
- Workspace content where permitted.
- Messaging metadata where explicitly authorized.
- Maps/place records where precision and source policy allow it.
- Public/authorized social content after source moderation policy permits it.

Search must respect permissions. Ranking should not become an ad trap where small businesses are forced to pay escalating marketplace fees for visibility.

The first useful Search slice includes:

- Source registrations and source policies for Directory Listings, public app pages, public-interest datasets, Workspace permitted content, Messaging metadata, Maps/place refs, and Social public posts.
- Crawl, ingest, reindex, removal, freshness, tombstone, and replay jobs.
- Search-owned Overbase lexical/document/secondary indexes first, with Overbase `vector_index` collections only where source policy permits vectorization and query-time permission filters are proven.
- Overstore chunk/artifact refs for large indexed material and Overvault grant refs for private or sensitive source material.
- Permission filter snapshots applied before snippets, previews, embeddings, explanations, ranking features, result handoffs, and Personal AI Assistant citations.
- Query sessions, source scopes, private-source permission previews, result sets, denied/omitted counts, pagination, and audience-safe ranking explanations.
- Ranking explanation fields for matched field class, source type, source policy, freshness band, authority/provenance band, category/locality fit, explicit filters, permission/redaction class, safety modifiers, diversity/deduplication, public-interest purpose refs, ranking policy version, omitted/denied counts, and paid-placement absence.
- Native app and Personal AI Assistant handoffs through explicit result refs, source refs, redaction class, permission refs, usage refs, audit refs, and replay refs.
- Public-interest dataset manifests with steward/source refs, Purpose Tag refs, license/rights evidence, Overguard and Compliance Boundary checks, and Overclaim contested-source handling.
- Spam, cloaking, malicious indexing, source poisoning, impersonation, privacy leak, and ranking-abuse reports routed to Fraud Control, Reputation/Anti-Sybil, Overclaim, and source moderation owners.
- Usage refs for indexing, query, private permission checks, result delivery, explanations, handoffs, removals, abuse reports, replay, compute, storage, and bandwidth.
- Private-search retention where raw private query/result payloads default to no more than 24 hours and durable records default to redacted audit refs unless pinned, source policy says otherwise, or a sealed hold applies.

Search should be an Overrid-native discovery service, not an external search product wrapper, source-of-truth database, ad auction, paid placement system, private-data bypass, or central AI decision service.

## Workstream 6: Messaging Center

Build a username-addressed replacement for fragmented email, phone, and chat identities:

- Direct username addressing through Overpass and Universal Namespace Service, without Messaging owning identity truth.
- Person, organization, app, service, and system-notification inbox records.
- Thread, participant, message envelope, delivery, read, recall, tombstone, archive, mute, and block records.
- Organization inbox routes, role-based assignment, escalation, internal notes, and audit refs.
- App/service notifications with preference, quiet-hour, redaction, and abuse-throttle checks.
- Encrypted personal messages through Overvault/Overstore payload refs, grant refs, expiry, revocation, and access audit.
- Attachments through Overstore refs with safety scans, retention class, previews, and policy refs.
- Optional local-first delivery/read-receipt queues through mSwarm Runtime Bridge where grants, revocation, and first-contact policy still validate on reconnect.
- Spam, harassment, impersonation, unsafe attachment, unwanted-contact, notification-flooding, Fraud Control/Reputation/Overclaim handoffs, moderation, appeals, and redacted reviewer queues.
- AI assistant triage with explicit permission manifests, allowed operations, redaction classes, AI Gateway Router route refs where model execution is needed, expiry, revocation, user/org approval for side effects, and replay.
- Metadata-only encrypted-message search projections scoped to the owning inbox, user, organization, or authorized app.
- Usage refs, Wallet/Usage Center receipt projections, Overwatch audit, export/data-portability refs, retention/tombstones, compliance-hold refs, and replay.

Messaging should be a protocol-level utility, not a lock-in product.

## Workstream 7: Social Photo/Video

Build only after stronger moderation and storage controls exist:

- Upload intents and Overstore media asset refs for photos, videos, thumbnails, transcodes, accessibility assets, safety scan refs, retention class, and replay.
- Overbase app state for posts, post versions, albums/collections, follows, groups, memberships, feeds, visibility controls, comments, reactions, moderation records, reports, usage refs, and replay bundles.
- Overvault grants for private groups, sensitive media, private metadata refs, grant expiry, revocation, and sealed hold behavior.
- Following-only and chronological v0 feeds, with profile, group, and album feeds scoped by explicit visibility refs.
- Local/community feeds limited to opt-in policy-compatible pilots that use source-attributed Directory, Maps, Search, and coarse locality refs.
- Transparent recommendation controls only after active `recommendation_control` records, explanation refs, mute/block/hide controls, volume limits, Overwatch replay evidence, and explicit no-paid-reach/no-addiction ranking policy exist.
- Rights/attribution refs attached to media, posts, reposts, remixes, public-interest media, exports, takedowns, and dispute flows.
- Repost/remix permissions, source linkage, transform provenance, revocation, takedown, Overclaim, Overwatch, Purpose Tag, Overasset/namespace, and Compliance Boundary refs where applicable.
- Abuse controls for spam, impersonation, unsafe media, privacy reports, coordinated follows/comments/reactions, rights claims, recommendation manipulation, and appeal/correction paths.
- Jurisdictional age and safety policy profiles before broad public launch, including required age/guardian/consent checks, minor-safe defaults, restricted-material Search exclusion, Messaging contact restrictions, and stable Overguard reason codes.
- Search Engine handoffs for public/authorized media posts and profiles without widening visibility.
- Messaging Center handoffs for notifications, private contact, reports, moderation notices, and organization inbox flows without Social owning direct messages.
- Personal AI Assistant permissions for captions, accessibility text, summaries, search help, reports, and appeals, with explicit source scopes, redaction classes, expiry, revocation, usage, and replay.
- Mobile SDK and Mobile Backend Gateway support for uploads, offline drafts, playback, notifications, privacy controls, compact deltas, and idempotent replay.
- Usage refs for uploads, storage, processing, feeds, comments, reactions, search updates, notifications, moderation, AI assistance, exports, retention, replay, compute, and bandwidth.
- Classed retention for owner deletion, moderation removal, unsafe media quarantine, rights disputes, private groups, sensitive media, exports, and compliance holds.

Avoid addiction-driven design. The app should serve people and communities, not maximize compulsive engagement.

## Workstream 8: Maps And Navigation

Build:

- Place records.
- Source-attributed, policy-compatible map inputs with visible attribution, license/source refs, freshness, confidence, precision class, and Overwatch audit refs.
- Routes for walking and driving first, with accessibility preferences; cycling and transit only when source coverage and authority are adequate; delivery and emergency routing remain disabled or review-gated until later safety/compliance controls exist.
- Local discovery.
- Business and organization listing handoffs through Directory Listings without Maps owning listing truth.
- Community map layers with risk-tiered moderation, evidence refs, correction history, report links, and reversible state.
- Transit or mobility integrations where available.
- Privacy-preserving location controls where denied/coarse is the default and exact/current/background/offline access requires explicit purpose-bound consent, expiry, revocation, and user-visible audit.
- Offline or cached map areas where feasible, starting with route-corridor packs, neighborhood/small-town packs, and selected public-interest layers under storage, freshness, consent, encryption, invalidation, and mobile capability budgets.
- mSwarm Runtime Bridge handoffs for offline area manifests, stale markers, source freshness, and exact/current location expiry without bridge-owned location authority.

Maps should integrate with directory listings, search, and messaging.

## Workstream 9: Central AI Stewardship Interface

Build an interface for:

- Role-aware stewardship dashboard counts, saved filters, and work queues.
- Redacted Central AI recommendation views with evidence refs, provenance refs, review state, downstream owner, and appeal paths.
- Public-interest project, grant, pool, donation, purpose-tag, and outcome views.
- Native app surplus-routing views with structural refs only, not speculative financial assumptions.
- Fraud and abuse evidence summaries by audience class.
- Appeals and disputes through Overclaim and owning-service refs.
- Signed review action envelopes routed to Central AI Service, Overgrant, Public-Interest Pool Service, Fraud Control, Overclaim, Stewardship Reporting Service, and other owners.
- System health, incident bands, report lists, governance status, and public-safe reporting summaries.
- Report publication, correction, withdrawal, and replay views.
- Usage refs, audit refs, redaction profile refs, and display snapshots for review and support.

The minimum Phase 12 slice is read-heavy and role-aware. It may create a narrow signed-review action envelope, but every mutating action must route to the owning service and retain evidence, privacy, audit, and appeal paths. Formal compliance, threat review, public-reporting governance, and scale hardening continue in Phase 13.

## Workstream 10: Mobile Service Layer

Build the Phase 12 mobile service layer from [SUB BUILD PLAN #82 - Mobile Backend Gateway](sub_build_plan_082_mobile_backend_gateway.md) and [SUB BUILD PLAN #83 - Mobile SDK](sub_build_plan_083_mobile_sdk.md):

- Mobile API contracts, device registration records, mobile session refs, capability profiles, sync cursors, offline command envelopes, push token refs, notification delivery refs, media upload sessions, wallet/usage reads, AI/RAG route handoffs, native app handoffs, Mobile SDK contracts, usage refs, audit refs, and redacted replay bundles.
- Generated mobile client bindings, configuration, secure storage provider interfaces, signed request helpers, device/session helpers, bounded offline queueing, sync cursor helpers, push registration helpers, media upload helpers, wallet/usage readers, native-app helpers, AI/RAG request helpers, permission prompts, redacted diagnostics, compatibility handling, and contract fixtures.
- Explicit handoffs to Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, Overqueue, Overbase, Overstore, Overvault, Wallet/Usage Center, AI Gateway Router, Personal AI Assistant, Encrypted Docdex RAG Adapter, native apps, Mobile SDK, mSwarm Runtime Bridge, Fraud Control, and support/operator views without mutating owner-service truth.
- Reference-only push payloads, bounded replay-safe offline command classes, advisory attestation refs, app/SDK compatibility profiles, privacy-safe diagnostics, usage/audit spooling, and Phase 13 mobile privacy, security-review, incident, compliance, reporting, reliability, and scale hardening.

Build the mobile access layer using:

- Mobile backend gateway.
- Mobile SDK.
- Device registration and revocation.
- Mobile sessions through Overrid identity and credential rails.
- Sync cursors and compact delta responses.
- Offline command intake and idempotent replay.
- Push notification refs with redacted payload rules.
- Media upload session coordination.
- Wallet and usage readers.
- Personal AI, AI gateway, and encrypted Docdex RAG handoffs.
- mSwarm Runtime Bridge handoffs for local-first sync, opt-in discovery, collaboration, cloud coordination, and offline replay where app-owned policies permit.
- Context grant revocation, leakage-profile visibility, and route-bound bundle expiry for AI/RAG flows.
- Abuse, fraud, rate-limit, and mobile replay evidence.

Mobile apps should use Overrid as a backend/resource plane without bypassing Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, ORU, Seal Ledger, storage, native app services, or AI routing.

## Validation

- Each native app uses normal Overrid APIs rather than private shortcuts.
- Wallet shows account selectors, balances, usage, holds, grants, refunds, receipts, statements, permissions, privacy audits, disputes, notifications, and replay through authoritative refs.
- Personal AI assistant uses encrypted Docdex RAG and model/resource routing.
- Directory listing supports identity, listing, search, messaging handoff, and dispute basics.
- Native app surplus routing is represented structurally without speculative financial assumptions.
- Mobile SDK and Mobile Backend Gateway support session, sync, offline replay, push, media, wallet/usage, and AI/RAG flows through normal Overrid rails.
- Native apps and mobile clients use mSwarm Runtime Bridge only for bounded local-first coordination; app-owned data, permissions, conflict semantics, public discovery, and side effects remain in the owning services.

## Exit Gate

Phase 12 is complete when the first native services operate as real Overrid applications using shared rails for identity, storage, policy, usage, accounting, and governance.

## Handoff To Phase 13

Phase 13 hardens governance, compliance, security, operations, and scale so public participation can grow responsibly.
