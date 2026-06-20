# SUB BUILD PLAN #69 - Directory Listings

Attached SDS: [SDS #69 - Directory Listings](../sds/native_apps/directory_listings.md)

## Purpose

This sub-build plan turns SDS #69 into an implementation sequence for Directory Listings. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Directory Listings is the Phase 12 native public utility for classifieds, local services, jobs, housing, events, community groups, organization/business pages, local discovery, and disputes. It owns listing records, category bindings, locality scopes, organization page records, listing lifecycle, media refs, contact handoff refs, search index update refs, map/place handoff refs, abuse report refs, moderation refs, dispute refs, usage refs, audit refs, and replay projections. It does not own direct messages, payments, escrow, final identity truth, map/place truth, global search ranking, final reputation scores, or final dispute resolution.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #69: Directory Listings](../sds/native_apps/directory_listings.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Directory Listings service plan](../service_catalog/native_apps/directory_listings.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency keys, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Overclaim disputes, Fraud Control evidence, Challenge Task facts, Reputation/Anti-Sybil recommendations, and deny-by-default behavior. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill accounting truth, wallet refs, receipts, and the rule that Directory Listings displays usage/accounting refs but never mutates balances, escrow, payments, or payouts. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase listing state, Overstore media refs, Overvault private contact/location refs, namespace records, route refs, protected content refs, tombstones, retention, backup/restore, and replay substrates. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider fraud, reputation, anti-Sybil, challenge, appeal, correction, and low-sensitivity abuse-control prerequisites used by listing moderation and public discovery. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first build point for the Directory Listings native app and its first useful category, listing, locality, contact, search, map, moderation, dispute, usage, and replay slice. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal compliance, category-boundary, threat-review, incident, retention, legal-hold, privacy, moderation, public-reporting, reliability, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #69 first build work aligned to master Phase 12, with Phase 13 as governance/security/compliance/reporting hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and contracts, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where a service boundary exists, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript for web/client surfaces only, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, hidden-ad, direct-message, direct-payment, direct-search-ranking, direct-map-truth, or raw-private-evidence drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 8, 11, 12, and 13 | Attach SDS #69, preserve Phase 12 as first build, record prerequisites, and freeze Directory ownership boundaries. |
| 2 | Master Phases 0, 1, 4, 5, 8, 11, and 12 | Define Rust contracts, canonical schemas, state machines, stable errors, signed refs, hashes, events, and fixtures. |
| 3 | Master Phases 1, 4, 8, 11, and 12 | Implement category, locality, organization page, verification marker, policy, and privacy foundations. |
| 4 | Master Phases 1, 4, 5, 8, and 12 | Implement listing lifecycle APIs, append-only versioning, media refs, ownership checks, usage refs, and replay basics. |
| 5 | Master Phases 4, 8, and 12 | Implement Search Engine and Maps handoffs with anti-ad-trap ranking constraints, redaction refs, and locality privacy. |
| 6 | Master Phases 1, 4, 6, 8, and 12 | Implement Messaging Center handoffs, organization inbox refs, first-contact policy, Personal AI delegation refs, and spam controls. |
| 7 | Master Phases 4, 5, 8, 11, 12, and 13 | Implement abuse reports, moderation actions, Fraud Control, Reputation/Anti-Sybil, Overclaim disputes, visibility limits, and corrections. |
| 8 | Master Phases 1, 5, 8, 12, and 13 | Implement usage, wallet, observability, audit, diagnostics, replay, exports, retention, and tombstone behavior. |
| 9 | Master Phases 6, 8, 12, and 13 | Implement native app UI/client flows, admin/moderator surfaces, SDK/CLI bindings, mobile-safe projections, and handoff ergonomics. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, category/compliance boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Directory Listings uses Rust-first shared contracts and service-facing APIs for listing records, category/locality policies, organization pages, contact handoffs, search/map handoffs, moderation reports, dispute refs, usage refs, audit refs, and replay bundles. TypeScript is acceptable for web/native client surfaces and generated bindings, but it must call Overrid APIs and must not become a privileged control plane.
- Listing records, category policies, locality scopes, organization pages, contact handoffs, search update requests, map handoffs, abuse reports, moderation actions, dispute refs, usage refs, deterministic fixtures, replay bundles, redaction profiles, and exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant or organization scope, trace id, idempotency key, category refs, locality refs, policy refs, ownership refs, visibility class, redaction profile refs, reason codes, downstream owner refs, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for listing versions, media refs, policy snapshots, handoff envelopes, moderation actions, replay bundles, audit exports, fixture inputs, and deterministic comparisons.
- Structured state, object/media refs, private contact/location refs, queueing where needed, search updates, map refs, identity, namespace, key status, policy, audit, usage, accounting refs, dispute refs, diagnostics, and replay must use native Overrid service boundaries such as Overbase, Overstore, Overvault, Overqueue, Overpass, Overtenant, Overkey, Universal Namespace Service, Overgate, Overguard, Overwatch, Overmeter, ORU Account Service, Seal Ledger, Wallet/Usage Center, Search Engine, Messaging Center, Maps and Navigation, Fraud Control, Reputation and Anti-Sybil Service, Overclaim, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, paid placement, hidden promotion, ad auctions, bidding, pricing, revenue forecasts, customer-count assumptions, escrow, payment settlement, raw private messages, raw exact private locations, vault secrets, direct map truth, direct search ranking authority, direct reputation finality, final dispute authority, or hidden moderation action the Directory Listings boundary.

## Phase 1: SDS Attachment, Phase 12 Scope, And Directory Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #69.**
  - Design: Link this document from the Directory Listings SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/native_apps/directory_listings.md`, `docs/service_catalog/native_apps/directory_listings.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #69 returns both the Directory Listings SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep first implementation in Phase 12 because Directory Listings is a native app built on identity, namespace, storage, policy, search, messaging, maps, usage, accounting, abuse-control, and dispute rails that earlier phases provide.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 8, and 11 supply prerequisites; Phase 12 builds the first useful listing utility; Phase 13 hardens compliance, security, governance, retention, incident response, and scale.
  - Validation: Review proves the plan does not move Directory Listings into Phase 8 storage, Phase 11 public-provider controls, Phase 13-only governance, Search Engine ownership, Messaging Center ownership, or Maps ownership.

- **1.3 Freeze the Directory ownership boundary.**
  - Design: Record that Directory Listings owns category bindings, listing records, organization page records, locality scopes, media refs, contact handoff refs, search update refs, map handoff refs, abuse report refs, moderation refs, dispute refs, usage refs, audit refs, and replay projections.
  - Output: Ownership checklist for architecture, API, UI, implementation, operations, and review gates.
  - Validation: Review confirms Directory Listings does not own direct messages, payment/escrow settlement, global search ranking, map/place canonical truth, final identity truth, final reputation scores, final fraud authority, or final dispute resolution.

- **1.4 Carry forward resolved SDS #69 decisions.**
  - Design: Preserve the Phase 12 low-compliance category allowlist, four locality privacy classes, Messaging Center first-contact handoff, Search Engine anti-ad-trap constraints, and immediate versus review-gated visibility rules.
  - Output: Resolved-decision checklist covering allowed first categories, disabled or review-gated categories, public/approximate/private/hidden locality, contact handoff records, search ranking constraints, and visibility transition rules.
  - Validation: Review rejects regulated-category default enablement, public phone/email exposure by default, exact private-location leakage, paid-placement ranking, hidden small-user suppression, and ad hoc final removal outside the owning resolution path.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Record how Overpass, Overtenant, Overkey, Universal Namespace Service, Overbase, Overstore, Overvault, Overguard, Workload Classifier, Overwatch, Overmeter, ORU Account Service, Seal Ledger, Wallet/Usage Center, Search Engine, Messaging Center, Maps and Navigation, Fraud Control, Reputation and Anti-Sybil Service, Overclaim, Personal AI Assistant, and Central AI Stewardship Interface interact through refs.
  - Output: Boundary matrix naming allowed reads, owned writes, handoff refs, denied direct mutation, audience rules, usage refs, audit refs, replay refs, correction paths, and owner-service finality.
  - Validation: Review confirms each downstream service keeps its authority and Directory Listings exchanges refs/events rather than reading private storage directly or inventing canonical truth owned elsewhere.

## Phase 2: Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the Directory Listings Rust contract module.**
  - Design: Add contract types for listing categories, locality scopes, listing records, listing versions, organization pages, contact handoffs, search update requests, map/place handoffs, abuse reports, moderation actions, dispute refs, usage refs, replay bundles, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, category-state enums, locality-privacy enums, moderation-action enums, contact-state enums, visibility-state enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Search Engine ranking, Messaging Center payloads, Maps place truth, accounting mutation, escrow/payment settlement, and final dispute authority.

- **2.2 Define listing, category, and locality schemas.**
  - Design: Model category ids, category requirements, prohibited fields, jurisdiction refs, policy refs, locality class, place/region refs, service-area refs, private exact-location refs, listing owner refs, title/summary/attributes, media refs, visibility state, moderation state, expiry, and audit refs.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, low-compliance category examples, disabled-category examples, approximate locality examples, hidden-locality examples, and exact-location denial fixtures.
  - Validation: Schema tests reject listings without category refs, locality refs, owner refs, policy refs, visibility class, media ref validation, redaction profile, audit refs, or required category fields.

- **2.3 Define organization page, contact, search, and map schemas.**
  - Design: Model organization/business/community pages, namespace refs, verification markers, claim/dispute refs, contact preference refs, Messaging Center thread/inbox refs, Search Engine index update refs, search visibility class, anti-ad-trap ranking constraint refs, map/place refs, route display permission, and offline/cache flags.
  - Output: Schema set, organization page examples, first-contact examples, organization inbox examples, search update examples, map handoff examples, negative fixtures for public private-contact and exact private-location exposure.
  - Validation: Tests prove schemas require explicit owner/service refs and cannot smuggle raw messages, public phone/email defaults, exact private home addresses, payment details, hidden paid promotion, or map canonical truth.

- **2.4 Define report, moderation, dispute, and usage schemas.**
  - Design: Model abuse reports, scam signals, impersonation reports, prohibited-content reports, moderation actions, visibility limits, Overclaim dispute refs, Fraud Control refs, Reputation/Anti-Sybil refs, usage refs, wallet receipt refs, tombstones, corrections, restoration refs, and replay state.
  - Output: Abuse/report schema set, moderation-action state machine, dispute-ref schema, usage-ref schema, tombstone/correction examples, false-positive correction fixtures, and stable error catalog.
  - Validation: Tests reject moderation actions without policy refs, reason codes, reviewer refs where required, correction path, audit refs, appeal/dispute refs where required, and owner-service finality refs.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for draft, validation failure, publish, edit/version, pause, renew, expire, archive, organization page creation, contact handoff, search update, map handoff, abuse report, moderation action, dispute, usage emission, tombstone, correction, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected stable errors, BLAKE3 hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, redacted outputs, dispute refs, tombstone refs, and replay output across repeated runs.

## Phase 3: Category, Locality, Organization Page, And Policy Foundations

### Work Items

- **3.1 Implement the Phase 12 category registry slice.**
  - Design: Start with low-compliance, high-utility categories such as community events, community groups, organization/business pages, general household classifieds, free/giveaway posts, and non-regulated local services.
  - Output: Category registry records, policy refs, required-field rules, disabled-category records, review-gated category records, jurisdiction refs, and category validation errors.
  - Validation: Tests prove jobs, housing, childcare, medical/legal/financial services, transportation, weapons, regulated goods, high-value asset sales, age-restricted categories, and other high-compliance categories remain disabled or review-gated until Compliance Boundary and category policy refs exist.

- **3.2 Implement locality privacy classes.**
  - Design: Support public place/venue refs, approximate locality, private-to-contact exact pickup/route/address refs, and hidden locality with strict default selection by category and actor context.
  - Output: Locality resolver, locality validation API, precision downgrade rules, private ref storage, Maps handoff constraints, hidden-locality denial states, and redaction profile refs.
  - Validation: Tests prove individual listings default to approximate locality, exact private location is never public by default, and hidden locality applies to sensitive, private-home, vulnerable-person, disputed, moderated, or over-precise listings.

- **3.3 Implement organization and business page foundations.**
  - Design: Link organization pages to Overpass identities, Universal Namespace Service names/routes, verification markers, claim/dispute refs, service categories, place/locality refs, and contact handoff refs.
  - Output: Organization page create/update/read APIs, namespace verification refs, page lifecycle states, claim/dispute refs, page media refs, and public-safe projections.
  - Validation: Tests prove pages cannot impersonate organizations, bypass namespace ownership, publish unverifiable claims as verified, expose private contact refs, or overwrite disputed page state without the owning claim path.

- **3.4 Implement deny-by-default policy prechecks.**
  - Design: Run Overguard and Workload Classifier-derived policy facts before publish, contact, search, map, report, moderation, and export operations.
  - Output: Policy precheck adapter, reason-code mapping, missing-fact errors, category/locality/media/contact checks, and policy snapshot refs on state transitions.
  - Validation: Tests prove missing category, locality, ownership, media, contact preference, redaction, or policy facts keep listings in draft or limited visibility rather than publishing.

- **3.5 Implement owner, tenant, and role isolation.**
  - Design: Enforce actor, organization, tenant, moderator, steward, operator, auditor, and service-account roles for all listing, page, contact, report, moderation, dispute, export, and replay reads/writes.
  - Output: Access-control adapter, role-specific projections, tenant/org ownership checks, stronger-role gates, denial refs, and Overwatch audit events.
  - Validation: Tests prove users cannot edit other owners' listings, organizations cannot claim unrelated pages, public users cannot inspect private reports, and moderators cannot bypass policy/audit requirements.

## Phase 4: Listing Lifecycle APIs, Versions, Media, And Replay Basis

### Work Items

- **4.1 Implement draft creation and validation.**
  - Design: Add draft creation for category, locality, media, contact preferences, organization page refs, structured attributes, and policy snapshots before public visibility exists.
  - Output: `POST /directory/listings/drafts`, draft view model, field-level validation errors, idempotency handling, media-ref checks, and `directory_listings.draft_created` event.
  - Validation: API tests prove drafts require signed envelopes, tenant/org context, trace id, idempotency key, category refs, locality refs, and policy refs.

- **4.2 Implement publish and visibility transitions.**
  - Design: Publish only after category, locality, ownership, media, contact, policy, redaction, and abuse prechecks pass; immediate limiting/removal only happens for high-confidence safety, privacy, legality, identity, or policy failures.
  - Output: `POST /directory/listings/{listing_id}/publish`, visibility state machine, limited-visibility state, publish events, search update request refs, map handoff refs, usage refs, and audit refs.
  - Validation: Tests prove high-confidence prohibited category, exact-location leak, credible impersonation, malware/scam payload, discrimination-sensitive violation, child-safety/violence threat, Overguard denial, or source-trusted Fraud Control evidence can trigger immediate blocking.

- **4.3 Implement edit, versioning, pause, renew, expire, archive, and removal flows.**
  - Design: Record every listing change as an append-only version with changed fields, editor refs, policy checks, previous version refs, rollback/correction refs, and lifecycle reason codes.
  - Output: `PATCH`, `pause`, `renew`, `expire`, `archive`, and removal/tombstone APIs, version records, state transition events, retention refs, and stable errors such as `listing_state_conflict`.
  - Validation: Tests prove edits cannot mutate history, renewals rerun current policy, removals preserve tombstones where policy requires, and archived/expired states retain dispute/audit refs.

- **4.4 Implement read/list APIs and role projections.**
  - Design: Return listing details by listing id, category, locality, organization, state, and visibility class while respecting owner, role, tenant, redaction, and policy state.
  - Output: `GET /directory/listings/{listing_id}`, `GET /directory/listings`, pagination, filters, public/owner/moderator projections, redaction badges, and denial states.
  - Validation: Tests prove public reads do not expose private contact refs, exact private locations, moderation internals, private fraud evidence, other-tenant data, or policy-hidden fields.

- **4.5 Implement lifecycle replay foundations.**
  - Design: Reconstruct listing state from versions, policy snapshots, media refs, search/map/messaging handoffs, report/moderation/dispute events, usage refs, and audit refs.
  - Output: `GET /directory/replay/{record_id}`, replay bundle format, BLAKE3 display hashes, event ordering, actor/service refs, and redacted replay projections.
  - Validation: Tests prove replay can reconstruct publish decisions, visibility changes, reports, moderation actions, disputes, handoffs, usage refs, and corrections without raw private messages or exact private locations.

## Phase 5: Search, Maps, Discovery, And Anti-Ad-Trap Handoffs

### Work Items

- **5.1 Implement Search Engine index update handoff.**
  - Design: Send redacted listing/page summaries, category/locality tokens, visibility class, safety state, freshness windows, owner diversity refs, and ranking-constraint refs to Search Engine.
  - Output: `search_index_update` records, update request events, retry state, lag metrics, redaction refs, and `directory_listings.search_update_requested` events.
  - Validation: Tests prove Search Engine receives refs and public-safe summaries only, not private contact data, exact private locations, payment facts, fraud internals, or unrelated profile data.

- **5.2 Enforce anti-ad-trap ranking constraints.**
  - Design: Represent allowed ranking signals as category match, locality fit, text/attribute relevance, freshness within capped renewal windows, verified organization/namespace markers, completeness, safety/moderation state, owner diversity, and user-selected filters.
  - Output: Ranking-constraint schema, search explanation refs, fairness/rotation constraint refs, invalid-signal denial fixtures, and no-paid-placement guardrail tests.
  - Validation: Tests reject bids, paid placement, hidden promotion, native-service surplus routing, payment history, engagement addiction metrics, private profile targeting, and opaque small-user suppression.

- **5.3 Implement Maps and Navigation handoff.**
  - Design: Send place/locality refs, coarse/exact location class, route-display permission, privacy class, source freshness, and offline/cache allowance while keeping Maps and Navigation the place/route display owner.
  - Output: `map_place_handoff` records, Maps request events, denied map display states, precision downgrade records, and map privacy audit refs.
  - Validation: Tests prove exact private pickup/address refs are released only through authorized Messaging Center or Maps handoff after listing policy and recipient contact policy pass.

- **5.4 Implement local discovery and directory browsing.**
  - Design: Support category/locality browsing, organization pages, public events, public place refs, and saved listing refs without turning Directory browsing into global Search Engine ranking.
  - Output: Browse APIs, category/locality landing projections, organization page listings, event/community views, saved-list refs, and public-safe filters.
  - Validation: Tests prove browsing respects visibility, category policy, locality privacy, ownership, moderation state, redaction, and fairness constraints.

- **5.5 Implement search/map failure recovery.**
  - Design: Allow listings to publish with direct URL visibility when Search Engine is unavailable and policy allows, while marking search update pending; degrade map display to coarser locality when map handoff fails or policy denies exact display.
  - Output: Pending update state, retry refs, contact-unavailable state where needed, direct URL visibility flags, coarser-locality fallback, stable errors, and operator diagnostics.
  - Validation: Tests prove outages do not drop listing truth, leak private fields, skip usage/audit refs, or make downstream services authoritative over Directory state.

## Phase 6: Messaging Handoff, Organization Inboxes, Spam Controls, And Personal AI Delegation

### Work Items

- **6.1 Implement first-contact handoff records.**
  - Design: Directory creates contact handoffs with listing refs, requester refs, owner/org inbox refs, consent refs, spam-risk refs, rate-limit state, report/block affordances, and Messaging Center owner refs.
  - Output: `POST /directory/listings/{listing_id}/contact`, `contact_handoff` records, Messaging Center thread/inbox refs, first-contact states, and `directory_listings.contact_handoff_created` events.
  - Validation: Tests prove Directory stores handoff refs only, not message bodies, attachments, encrypted payload refs, raw private contact data, or Messaging Center delivery state.

- **6.2 Implement organization inbox and verified page contact flows.**
  - Design: Let verified organizations route to organization inboxes while individual listings use two-step contact requests and redacted owner identity until policy allows more detail.
  - Output: Organization inbox refs, verification marker checks, owner identity redaction, contact preference rules, consent records, and denial states.
  - Validation: Tests prove unverified or disputed organization pages cannot present verified contact affordances and individual listings do not reveal owner identity or private contact details by default.

- **6.3 Implement spam, unsafe attachment, and off-platform pressure controls.**
  - Design: Detect repeated template spam, unsafe attachment attempts, private-contact harvesting, and off-platform payment pressure by creating fraud/report refs without blocking ordinary low-risk messages by default.
  - Output: Spam-risk refs, contact throttles, report/block paths, Fraud Control handoff refs, stable errors, and redacted user-facing states.
  - Validation: Tests prove low-risk first contact remains usable, while abusive patterns create auditable refs, throttles, reports, or review queues according to policy.

- **6.4 Implement Personal AI Assistant delegation refs.**
  - Design: Allow Personal AI Assistant to help draft listings, summarize allowed listing replies, search listings, and report abuse only through explicit delegated native-app calls with permission and audit refs.
  - Output: Delegated action refs, permission checks, tool proposal refs, listing draft assist refs, search assist refs, report assist refs, and replay links.
  - Validation: Tests prove the assistant cannot publish, contact, expose private contact data, bypass category policy, or widen locality precision without explicit Directory and user-permission checks.

- **6.5 Implement contact handoff replay and support diagnostics.**
  - Design: Reconstruct contact handoff decisions from listing state, contact preference, consent refs, spam-risk refs, policy refs, Messaging Center refs, report/block refs, usage refs, and audit refs.
  - Output: Contact replay projection, support-safe diagnostics, owner/requester views, moderator views, stable denial reasons, and redacted audit export.
  - Validation: Tests prove support diagnostics can explain contact failures or abuse throttles without revealing message content, exact private locations, private contact data, or abuse thresholds.

## Phase 7: Abuse Reports, Moderation, Fraud Control, Reputation, And Disputes

### Work Items

- **7.1 Implement abuse and scam report intake.**
  - Design: Support reports for prohibited content, scams, impersonation, unsafe meetings, private-contact harvesting, discrimination-sensitive listings, spam, malware, unsafe attachments, and map/location privacy violations.
  - Output: `POST /directory/listings/{listing_id}/reports`, report records, evidence refs, Fraud Control refs, Overwatch audit refs, reporter protections, stable errors, and `directory_listings.abuse_reported` events.
  - Validation: Tests prove reports create refs and audit evidence without exposing reporter identity to unauthorized parties or mutating listing finality by themselves.

- **7.2 Implement moderation actions and visibility controls.**
  - Design: Record warnings, labels, contact throttles, limited visibility, holds, removals, restorations, corrections, and tombstones with policy refs, reason codes, reviewer refs, duration, appeal path, and audit refs.
  - Output: Moderation action API, moderation queue projections, state transition events, reviewer assignment refs, correction paths, and `directory_listings.moderation_action_recorded` events.
  - Validation: Tests prove lower-confidence abuse reports, duplicate/spam suspicions, category corrections, quality issues, routine disputes, and contested organization claims move to limited visibility or review queues rather than final removal by default.

- **7.3 Implement Fraud Control and Reputation/Anti-Sybil handoffs.**
  - Design: Send redacted evidence refs and listing/page/contact refs to Fraud Control and Reputation/Anti-Sybil for risk recommendations, not final Directory outcomes.
  - Output: Fraud handoff refs, reputation signal refs, recommendation refs, hold/throttle recommendations, duplicate/spam refs, redacted summaries, and owner-service correction paths.
  - Validation: Tests prove private fraud internals and anti-abuse thresholds remain redacted and that Directory applies recommendations only through policy, reviewer, or owning-service finality rules.

- **7.4 Implement Overclaim dispute refs and challenge paths.**
  - Design: Attach Overclaim refs for contested listings, organization claims, moderation disputes, scam/fraud allegations, service delivery complaints, and restoration/correction requests.
  - Output: `POST /directory/listings/{listing_id}/disputes`, dispute refs, party refs, evidence refs, hold/hide state, deadlines, finality markers, and replay refs.
  - Validation: Tests prove disputes can limit visibility or contact according to policy but final removal, restoration, correction, or tombstone behavior follows the owning resolution path.

- **7.5 Implement moderator, steward, and operator review views.**
  - Design: Provide redacted work queues, policy summaries, evidence refs, prior actions, risk bands, dispute refs, correction paths, and owner-service links by authorized role.
  - Output: Moderator queue API, steward/operator projections, redaction badges, missing-evidence states, action eligibility reasons, and usage/audit refs.
  - Validation: Tests prove review views do not leak raw private messages, exact private locations, payment details, fraud internals, unrelated profile data, or other-tenant evidence.

## Phase 8: Usage, Wallet Visibility, Observability, Audit, Retention, And Replay

### Work Items

- **8.1 Emit usage refs for Directory operations.**
  - Design: Meter listing creation, validation, edits, publish/renew, media storage, search update, map handoff, contact handoff, abuse report, moderation work, dispute work, export, replay, and retention/tombstone operations.
  - Output: Directory usage events, Overmeter refs, wallet receipt refs, operation dimensions, actor/org/listing/category/locality tags, and reconciliation refs.
  - Validation: Tests prove usage refs are emitted for successful and policy-denied operations where required and no usage event mutates ORU balances directly.

- **8.2 Implement Wallet/Usage Center projections.**
  - Design: Show user and organization usage, holds where applicable, receipt refs, pending usage, reconciliation states, and operation-level explanations without pricing or revenue forecasts.
  - Output: Wallet/usage projection API, receipt refs, pending usage states, overuse warnings where policy provides them, and correction refs.
  - Validation: Tests prove Directory does not maintain balances, escrow, payments, bids, invoices, payout truth, hardcoded prices, revenue forecasts, or per-transaction fees.

- **8.3 Implement observability and operational metrics.**
  - Design: Track listing counts by category/locality/state, publish latency, policy denial rate, search update lag, map handoff lag, contact handoff failure rate, report volume, moderation backlog, scam signal rates, dispute rates, usage by operation, and replay backlog.
  - Output: Metrics, traces, dashboards, alert rules, redacted logs, operational events, and runbook hooks.
  - Validation: Tests and drills prove metrics exist for spam bursts, impersonation spikes, prohibited category attempts, repeated contact abuse, search update failures, map privacy violations, moderation backlog age, missing usage refs, and replay delays.

- **8.4 Implement audit exports and redacted replay bundles.**
  - Design: Export audit and replay evidence for owners, moderators, stewards, auditors, legal/compliance, and support without exposing raw private data beyond audience policy.
  - Output: Audit export API, replay bundle export, BLAKE3 hash manifests, redaction profile refs, stable ordering, retention labels, and legal-hold markers.
  - Validation: Tests prove exports reconstruct decisions and evidence refs while excluding raw private messages, exact private locations, vault secrets, payment details, fraud internals, and unrelated user profile data.

- **8.5 Implement retention, tombstones, and cleanup behavior.**
  - Design: Handle expired, archived, removed, disputed, corrected, restored, and deleted listings through explicit retention classes, tombstones, dispute/audit refs, and deletion/visibility rules.
  - Output: Retention policy refs, tombstone records, cleanup jobs, restore refs, purge-denial errors, legal hold refs, and operator diagnostics.
  - Validation: Tests prove deletion does not remove required dispute/audit/tombstone evidence and retention does not keep raw private contact/location data longer than policy permits.

## Phase 9: Native Client, Admin, SDK, CLI, And Mobile-Safe Surfaces

### Work Items

- **9.1 Implement Phase 12 native app user flows.**
  - Design: Build user-facing flows for browsing, creating drafts, publishing, editing, pausing, renewing, expiring, archiving, contacting, reporting, disputing, and replaying allowed listing history.
  - Output: TypeScript client views or generated bindings, listing forms, category/locality selectors, privacy controls, contact handoff UI, report/dispute affordances, and usage/receipt links.
  - Validation: UI/API tests prove user flows call Overrid APIs and cannot bypass category policy, locality privacy, contact policy, usage/audit emission, or moderation state.

- **9.2 Implement organization and business page management flows.**
  - Design: Provide page create/update, namespace claim, verification marker display, organization inbox routing, page dispute, service category selection, public page preview, and correction flows.
  - Output: Organization page UI/API projections, verification badges, claim/dispute views, contact settings, page audit history, and replay links.
  - Validation: Tests prove disputed/unverified pages cannot display misleading verification, expose private owner data, or bypass namespace/claim checks.

- **9.3 Implement moderator and support surfaces.**
  - Design: Provide queues for reports, policy denials, limited-visibility records, contested pages, spam/contact abuse, map privacy issues, search update failures, dispute refs, and correction/restoration work.
  - Output: Admin/Developer UI projections, moderator filters, action eligibility reasons, redacted evidence summaries, audit links, replay links, and stable error views.
  - Validation: Tests prove support and moderator views are role-scoped and never expose private data or final-authority actions beyond role and owning-service policy.

- **9.4 Implement SDK, CLI, and mobile-safe contract bindings.**
  - Design: Generate clients for listing lifecycle, organization pages, contact handoffs, reports, disputes, search/map refs, usage, replay, and exports with stable JSON output.
  - Output: SDK bindings, CLI commands, mobile-safe compact projections, offline-safe read shapes where permitted, fixture-based examples, and docs.
  - Validation: Contract tests prove generated clients preserve signing, idempotency, trace ids, tenant/org scope, stable errors, redaction states, and usage/audit refs.

- **9.5 Implement Personal AI and Central AI Stewardship Interface handoffs.**
  - Design: Expose safe refs for Personal AI listing assistance and Central AI Stewardship Interface review/reporting without making either service the listing authority.
  - Output: Delegated action schemas, public-safe summaries, review queue refs, surplus/reporting refs where applicable, usage refs, audit refs, and replay links.
  - Validation: Tests prove AI/stewardship surfaces cannot publish, remove, contact, rank, expose private data, or finalize disputes without Directory policy and owning-service authority.

## Phase 10: Phase 13 Hardening, Validation, Documentation, And Handoff

### Work Items

- **10.1 Harden category and compliance boundaries.**
  - Design: Connect disabled/review-gated categories to Compliance Boundary, policy refs, Fraud Control, Overclaim, jurisdiction markers, retention rules, and appeal paths before enabling higher-compliance categories.
  - Output: Category readiness matrix, compliance-boundary refs, enablement checklist, migration plan, public notices, and explicit disabled states.
  - Validation: Tests prove high-risk categories cannot be enabled by config drift, UI-only changes, search indexing, or moderator action without required boundary refs.

- **10.2 Harden security, privacy, abuse, and incident response.**
  - Design: Threat-model scams, impersonation, exact-location leakage, private-contact harvesting, map privacy bypass, search ranking manipulation, contact spam, moderation abuse, disputed page takeover, and replay/audit gaps.
  - Output: Threat model entries, security review checklist, incident runbooks, privacy review checklist, mitigations, monitoring, owner assignments, and accepted-risk records where needed.
  - Validation: Security review confirms each listed threat has tests, monitoring, controls, incident paths, or explicit accepted risk.

- **10.3 Harden reliability and scale behavior.**
  - Design: Run drills for listing publish bursts, search update lag, map handoff lag, contact handoff abuse, report floods, moderation backlog, dispute backlog, usage reconciliation lag, retention cleanup, replay backlog, and partial owner-service outage.
  - Output: Drill plans, expected behavior, actual behavior, evidence refs, metrics thresholds, remediation tasks, and release gates.
  - Validation: Drills prove Directory degrades without losing listing truth, leaking private data, skipping usage/audit refs, or granting downstream services final authority.

- **10.4 Validate documentation and implementation handoff readiness.**
  - Design: Recheck SDS/service/build-plan links, 10 sub-build phases, work-item structure, local Markdown links, queue state, stack guardrails, revenue/pricing/ad-trap scans, and Docdex retrieval.
  - Output: Validation evidence in `docs/build_plan/progress.md`, queue progress update, Docdex index refresh, Docdex search evidence, and run-test blocker note if unchanged.
  - Validation: Focused scripts pass for title prefix, attached SDS link, phases 1 through 10, Design/Output/Validation work-item structure, local links, final newlines, no unresolved markers, and stack guardrails.

- **10.5 Hand off to Phase 12 implementation and Phase 13 hardening.**
  - Design: Summarize the minimum buildable Phase 12 slice and the later Phase 13 hardening scope so implementation can start without re-litigating ownership, stack choices, or category/compliance boundaries.
  - Output: Handoff checklist covering contracts, schemas, APIs, category allowlist, locality privacy, contact/search/map handoffs, abuse/moderation/disputes, usage/audit/replay, client surfaces, and hardening gates.
  - Validation: Review confirms the plan remains internally consistent, consistent with SDS #69, consistent with service catalog and master Phase 0-13 docs, and consistent with `docs/overrid_tech_stack_choice.md`.
