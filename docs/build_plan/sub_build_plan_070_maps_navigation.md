# SUB BUILD PLAN #70 - Maps and Navigation

Attached SDS: [SDS #70 - Maps and Navigation](../sds/native_apps/maps_navigation.md)

## Purpose

This sub-build plan turns SDS #70 into an implementation sequence for Maps and Navigation. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Maps and Navigation is the Phase 12 native public utility for places, routes, local discovery, map layers, offline areas, community corrections, and privacy-preserving location controls. It owns place records, geometry refs, map layer records, route requests/results, location permission records, offline area manifests, correction reports, directory/search/messaging/assistant handoff records, usage refs, audit refs, and replay projections. It does not own listing truth, broad search ranking truth, direct messages, payments, identity truth, vault secret material, final disputes, final fraud authority, final reputation scores, or final accounting truth.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #70: Maps and Navigation](../sds/native_apps/maps_navigation.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Maps and Navigation service plan](../service_catalog/native_apps/maps_navigation.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency keys, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Overclaim disputes, Fraud Control evidence, Challenge Task facts, Reputation/Anti-Sybil recommendations, and deny-by-default behavior. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill accounting truth, Wallet/Usage Center receipt refs, and the rule that Maps displays usage/accounting refs but never mutates balances or payment records. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase place/layer state, Overstore tile/object/offline refs, Overvault private location grants, Universal Namespace place/route refs, retention, backup/restore, and replay substrates. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider fraud, reputation, anti-Sybil, challenge, appeal, correction, and abuse-control prerequisites used by public layers, public place corrections, and low-sensitivity local discovery. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first build point for the Maps and Navigation native app and its first useful place, route, location-permission, local-discovery, offline-area, community-layer, handoff, usage, audit, and replay slice. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal location privacy, route safety, retention, compliance, threat review, incident response, public reporting, reliability, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #70 first build work aligned to master Phase 12, with Phase 13 as governance/security/compliance/reporting hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and contracts, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where a service boundary exists, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript for client surfaces only, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, proprietary-map-product-boundary, surveillance, ad-rank, paid-placement, pricing, revenue, or raw-mobility-history drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 8, 11, 12, and 13 | Attach SDS #70, preserve Phase 12 as first build, record prerequisites, and freeze Maps ownership boundaries. |
| 2 | Master Phases 0, 1, 4, 8, 12, and 13 | Define Rust contracts, source provenance, attribution, map layers, tile/object refs, import records, stable errors, and fixtures. |
| 3 | Master Phases 1, 4, 8, and 12 | Implement place, geometry, location permission, precision, redaction, policy, and tenant/app/device isolation foundations. |
| 4 | Master Phases 1, 4, 8, and 12 | Implement place/local-discovery APIs and Directory/Search/Messaging/Assistant handoffs without ownership drift. |
| 5 | Master Phases 4, 8, 12, and 13 | Implement privacy-preserving routing, tiered route modes, redacted route results, sharing, and route-source failure recovery. |
| 6 | Master Phases 5, 8, 12, and 13 | Implement offline area manifests, cache freshness, storage/encryption/invalidation, mobile handoffs, and revocation cleanup. |
| 7 | Master Phases 4, 8, 11, 12, and 13 | Implement community map layers, corrections, dispute paths, Fraud Control/Reputation/Overclaim handoffs, and moderation views. |
| 8 | Master Phases 1, 5, 8, 12, and 13 | Implement usage, Wallet/Usage Center location audit, observability, audit exports, retention, and replay. |
| 9 | Master Phases 6, 8, 12, and 13 | Implement native client flows, admin/support surfaces, SDK/CLI bindings, Mobile SDK/Gateway projections, AI tool permissions, and attribution UX. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, location/privacy/compliance boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Maps and Navigation uses Rust-first shared contracts and service-facing APIs for place records, geometry refs, map layers, route requests/results, location permissions, offline area manifests, local discovery queries, corrections, handoff refs, usage refs, audit refs, and replay bundles. TypeScript is acceptable for native/web client surfaces and generated bindings, but it must call Overrid APIs and must not become a privileged map authority.
- Place records, geometry refs, route refs, map layers, location permissions, offline manifests, source/attribution records, import jobs, correction reports, handoff records, usage refs, deterministic fixtures, replay bundles, redaction profiles, and exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/app/device scope, trace id, idempotency key, source refs, permission refs where location is involved, policy refs, visibility class, redaction profile refs, reason codes, downstream owner refs, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for place versions, layer manifests, tile/object refs, offline manifests, source snapshots, policy snapshots, handoff envelopes, correction actions, replay bundles, audit exports, fixture inputs, and deterministic comparisons.
- Structured state, tile/object refs, private location grants, queueing where needed, search/listing/messaging refs, identity, namespace, key status, policy, audit, usage, accounting refs, dispute refs, diagnostics, and replay must use native Overrid service boundaries such as Overbase, Overstore, Overvault, Overqueue, Overpass, Overtenant, Overkey, Universal Namespace Service, Overgate, Overguard, Overwatch, Overmeter, ORU Account Service, Seal Ledger, Wallet/Usage Center, Directory Listings, Search Engine, Messaging Center, Personal AI Assistant, Fraud Control, Reputation and Anti-Sybil Service, Overclaim, Mobile SDK, Mobile Backend Gateway, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, proprietary map APIs, scraped private mobility traces, opaque commercial ranking datasets, ad-tech location feeds, paid placement, hidden promotion, ad auctions, bidding, pricing, revenue forecasts, customer-count assumptions, raw exact private locations, raw route histories, raw mobility histories, vault secrets, direct listing truth, direct search ranking authority, direct messaging authority, direct reputation finality, final dispute authority, or hidden moderation action the Maps and Navigation boundary.

## Phase 1: SDS Attachment, Phase 12 Scope, And Map Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #70.**
  - Design: Link this document from the Maps and Navigation SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/native_apps/maps_navigation.md`, `docs/service_catalog/native_apps/maps_navigation.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #70 returns both the Maps and Navigation SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep first implementation in Phase 12 because Maps is a native app built on identity, namespace, storage, vault, policy, search, directory, messaging, mobile, usage, accounting, abuse-control, and dispute rails that earlier phases provide.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 8, and 11 supply prerequisites; Phase 12 builds the first useful maps utility; Phase 13 hardens location privacy, route safety, community-layer abuse, incident response, reporting, compliance, and scale.
  - Validation: Review proves the plan does not move Maps into Phase 8 storage, Phase 11 public-provider controls, Phase 13-only governance, Directory ownership, Search ranking ownership, Messaging ownership, or payment/accounting ownership.

- **1.3 Freeze the Maps ownership boundary.**
  - Design: Record that Maps and Navigation owns place records, geometry refs, map layers, route requests/results, location permissions, offline area manifests, correction reports, handoff refs, usage refs, audit refs, and replay projections.
  - Output: Ownership checklist for architecture, API, UI, implementation, operations, and review gates.
  - Validation: Review confirms Maps does not own listing truth, broad search ranking, direct messages, payments, identity truth, vault secret material, final fraud authority, final reputation scores, final disputes, or final accounting truth.

- **1.4 Carry forward resolved SDS #70 decisions.**
  - Design: Preserve source-attributed map inputs, tiered route modes, coarse-or-denied location defaults, bounded offline areas, and risk-tiered community map layer moderation.
  - Output: Resolved-decision checklist covering accepted data sources, rejected source classes, route mode readiness, precision defaults, offline cache classes, revocation triggers, community layer risk tiers, attribution visibility, and audit refs.
  - Validation: Review rejects scraped private mobility traces, ad-tech feeds, opaque commercial ranking datasets, proprietary APIs as the product boundary, default exact/background location, broad offline retention, and blanket community-layer pre-approval.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Record how Directory Listings, Search Engine, Messaging Center, Personal AI Assistant, Mobile SDK, Mobile Backend Gateway, Overpass, Universal Namespace Service, Overbase, Overstore, Overvault, Overguard, Overwatch, Overmeter, Wallet/Usage Center, Overclaim, Fraud Control, Reputation/Anti-Sybil, and Central AI Stewardship Interface interact through refs.
  - Output: Boundary matrix naming allowed reads, owned writes, handoff refs, denied direct mutation, audience rules, precision rules, usage refs, audit refs, replay refs, correction paths, and owner-service finality.
  - Validation: Review confirms each downstream service keeps its authority and Maps exchanges refs/events rather than copying private map data or inventing canonical truth owned elsewhere.

## Phase 2: Source Provenance, Map Layers, Contracts, And Fixtures

### Work Items

- **2.1 Create the Maps Rust contract module.**
  - Design: Add contract types for place records, place geometry refs, route requests/results, map layers, source refs, import jobs, location permissions, offline area manifests, local discovery queries, correction reports, handoff refs, usage refs, replay bundles, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, source-class enums, precision-class enums, route-mode enums, layer-type enums, offline-state enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Directory listing truth, Search ranking, Messaging payloads, accounting mutation, and final dispute authority.

- **2.2 Define source, attribution, and license schemas.**
  - Design: Model open/public-interest base-map extracts, public civic datasets, verified organization/place submissions, Directory handoffs, Search refs, user/community corrections, license/attribution refs, freshness, confidence, precision class, redaction policy, and import audit refs.
  - Output: JSON Schema files, accepted-source examples, attribution examples, freshness-window examples, source-confidence examples, rejected-source examples, and attribution projection fixtures.
  - Validation: Schema tests reject place, layer, tile/object, offline, and correction records without source refs, license or attribution refs where needed, freshness, confidence, precision class, redaction policy, import job refs, and Overwatch audit refs.

- **2.3 Define map layer and tile/object ref manifests.**
  - Design: Model base layers, public-interest layers, community layers, directory overlays, search overlays, private overlays, tile/object refs, layer ownership, visibility scope, freshness, storage refs, attribution refs, and moderation state.
  - Output: `map_layer` and tile/object schemas, valid layer fixtures, invalid over-precise layer fixtures, source-attribution fixtures, storage ref fixtures, and public/private projection examples.
  - Validation: Tests prove layers cannot expose private overlays, hidden listing fields, exact private homes, vulnerable-person contexts, unsafe emergency claims, or unlicensed/opaque data.

- **2.4 Implement import validation and unsafe-source rejection.**
  - Design: Create a validation path that accepts source-attributed, policy-compatible inputs and rejects scraped private mobility traces, ad-tech feeds, opaque commercial ranking data, or proprietary APIs that become the product boundary.
  - Output: Import validation API, source review state, rejection reason codes, quarantine refs, attribution requirements, audit events, and stable errors such as `place_source_untrusted`.
  - Validation: Tests prove unsafe sources fail before public place/layer/offline records exist and all rejection decisions are replayable from source refs and policy snapshots.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for source import, place creation, geometry update, layer creation, attribution display, rejected source, offline pack source selection, correction intake, route source fallback, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected stable errors, BLAKE3 hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, redacted outputs, source attribution, and replay output across repeated runs.

## Phase 3: Place, Geometry, Location Permission, And Policy Foundations

### Work Items

- **3.1 Implement place and geometry records.**
  - Design: Model place ids, names/aliases, namespace refs, category refs, locality refs, geometry refs, source refs, directory/listing refs, verification refs, visibility, state, correction refs, and audit refs.
  - Output: Place and geometry schemas, lifecycle state machine, create/update validation, merge/deprecate/remove states, precision classes, confidence classes, and fixture-backed projections.
  - Validation: Tests reject place records without source refs, geometry precision class, visibility class, policy refs, audit refs, and correction path.

- **3.2 Implement location permission records.**
  - Design: Support denied, coarse, exact, current, background, route, saved-place, sharing, and offline scopes with purpose, expiry, retention, revocation state, app/device refs, and audit refs.
  - Output: `location_permission` schema, grant/update/revoke APIs, permission lifecycle, user-visible audit refs, stable errors, and revocation event fixtures.
  - Validation: Tests prove exact, current, background, route, saved-place, and offline access is impossible without explicit purpose-bound consent and visible revocation.

- **3.3 Implement precision and redaction profiles.**
  - Design: Keep coarse precision as the default and route exact private location through Overvault private refs, recipient-scoped grants, expiry, redaction profiles, and policy refs.
  - Output: Precision resolver, redaction profile schema, downgrade rules, private ref storage adapter, denial states, and projection fixtures for public, owner, recipient, moderator, and auditor views.
  - Validation: Tests prove public views do not expose exact private location, saved places, private route geometry, private offline areas, or private contact data.

- **3.4 Implement deny-by-default policy prechecks.**
  - Design: Run Overguard and Workload Classifier-derived policy facts before place, route, layer, offline, location grant, handoff, correction, export, and replay operations.
  - Output: Policy precheck adapter, missing-fact errors, precision denial errors, source denial errors, route denial errors, layer visibility denial errors, and policy snapshot refs on state transitions.
  - Validation: Tests prove missing source, location permission, precision, ownership, visibility, redaction, freshness, or policy facts keep operations denied or limited rather than silently expanding access.

- **3.5 Implement actor, tenant, app, and device isolation.**
  - Design: Enforce actor, organization, tenant, app, device, steward, moderator, operator, auditor, and service-account roles for all place, route, layer, location, offline, correction, export, and replay reads/writes.
  - Output: Access-control adapter, role-specific projections, tenant/app/device ownership checks, stronger-role gates, denial refs, and Overwatch audit events.
  - Validation: Tests prove apps cannot read another app's location grant, devices cannot use revoked grants, organizations cannot hijack unrelated places, and support views cannot bypass redaction rules.

## Phase 4: Place APIs, Local Discovery, And Native Handoffs

### Work Items

- **4.1 Implement place read/create/update APIs.**
  - Design: Add place candidate creation, verified place ref creation, place update, correction proposal, place listing by locality/category/source, and replayable lifecycle transitions.
  - Output: `GET /maps/places/{place_id}`, `POST /maps/places`, `PATCH /maps/places/{place_id}`, `GET /maps/places`, stable errors, events, usage refs, and audit refs.
  - Validation: API tests prove signed envelopes, tenant/app/device context, trace id, idempotency key, source refs, policy refs, and audit refs are required for mutating operations.

- **4.2 Implement local discovery with Directory and Search refs.**
  - Design: Support local discovery by locality/place refs, category filters, directory/search refs, visibility constraints, ranking guardrails, privacy mode, and source freshness.
  - Output: `POST /maps/local-discovery`, `local_discovery_query` records, result refs, handoff refs, redacted result projections, and `maps_navigation.local_discovery_run` events.
  - Validation: Tests prove local discovery can display public Directory/Search results without making Maps the listing authority, broad ranking authority, or hidden ad-ranking surface.

- **4.3 Implement handoff records across native apps.**
  - Design: Record handoffs from Directory Listings, Search Engine, Messaging Center, Personal AI Assistant, and other native apps with redaction class, consent refs, source app/service refs, target place/route/layer refs, and state.
  - Output: `POST /maps/handoffs`, `map_handoff_ref` records, denied handoff states, result refs, audit refs, and handoff replay projections.
  - Validation: Tests prove handoffs pass refs and redacted projections rather than copying private route geometry, exact locations, hidden listing fields, message contents, or assistant context.

- **4.4 Implement organization/business listing integration.**
  - Design: Link verified organization/business pages and intentionally published venues to place refs while keeping Directory Listings the organization/listing/page authority.
  - Output: Directory place-link records, organization page place refs, verification marker refs, stale ref handling, source confidence, and disputed place/listing state.
  - Validation: Tests prove Directory updates cannot silently overwrite map place truth and map corrections cannot silently rewrite listing/page truth.

- **4.5 Implement replay for place and discovery decisions.**
  - Design: Reconstruct place, local discovery, source, handoff, policy, precision, correction, usage, and audit decisions from append-only refs.
  - Output: `GET /maps/replay/{record_id}`, replay bundle format, BLAKE3 display hashes, event ordering, actor/service refs, and redacted replay projections.
  - Validation: Tests prove replay can explain place creation, discovery results, handoff denials, precision downgrades, corrections, usage refs, and audit refs without raw private location data.

## Phase 5: Privacy-Preserving Routing, Route Modes, And Sharing

### Work Items

- **5.1 Implement route request and result contracts.**
  - Design: Model requester refs, origin/destination refs or private location refs, route mode, accessibility needs, constraints, route preferences, privacy mode, policy refs, result geometry refs, freshness, redaction profile, expiry, and usage refs.
  - Output: Route schemas, walking and driving v0 fixtures, accessibility preference fixtures, route explanation refs, route lifecycle states, and stable errors.
  - Validation: Tests reject route requests without permission refs, origin/destination refs, route mode, policy refs, privacy mode, trace id, and idempotency key.

- **5.2 Implement route APIs with permission validation.**
  - Design: Validate current permission, exact/coarse precision, origin/destination audience, route preference refs, source availability, and route policy before route computation or request dispatch.
  - Output: `POST /maps/routes`, `GET /maps/routes/{route_id}`, `route_requested`, `route_completed`, `route_denied`, and `route_unavailable` paths.
  - Validation: Tests prove missing permission, revoked grant, exact precision denial, private destination denial, and route source failure produce stable, replayable errors.

- **5.3 Implement tiered route-mode policy.**
  - Design: Keep Phase 12 v0 required route modes to walking and driving, allow cycling only after source coverage/safety metadata is adequate, treat transit as source-backed display/handoff unless authoritative feeds exist, and keep delivery/emergency routing disabled or review-gated.
  - Output: Route-mode readiness matrix, disabled/review-gated mode records, source authority checks, safety metadata requirements, and compliance gate refs.
  - Validation: Tests prove cycling, transit, delivery, and emergency modes cannot be enabled by UI or config drift without required source, policy, replay, and abuse-control refs.

- **5.4 Implement route expiry, redaction, and sharing.**
  - Design: Expire route results and private route geometry, redact route projections by audience, and share route/place refs through Messaging Center only with consent, expiry, and revocation.
  - Output: Route expiry jobs, redacted route projections, Messaging Center sharing refs, recipient-scoped exact location grants, revoked-share states, and audit refs.
  - Validation: Tests prove exact live location requires recipient-scoped consent and expiry, is revocable, and is never broadly indexed in threads or search.

- **5.5 Implement route failure recovery and fallbacks.**
  - Design: Return retryable failures, fallback route provider refs only where allowed, manual direction alternatives where useful, and no privacy expansion during outage.
  - Output: Retry states, fallback-source refs, partial route states, degraded projections, stable errors, and operator diagnostics.
  - Validation: Tests prove route outages do not leak private origins/destinations, skip usage/audit refs, invent untrusted route sources, or widen location permissions.

## Phase 6: Offline Areas, Cache Freshness, Mobile, And Revocation

### Work Items

- **6.1 Implement offline area manifests.**
  - Design: Support Phase 12 v0 route-corridor packs, neighborhood/small-town packs, and selected public-interest layer packs within Mobile Backend Gateway and Mobile SDK budgets.
  - Output: `POST /maps/offline-areas`, `offline_area_manifest` schema, area refs, layer refs, tile/object refs, freshness window, storage refs, encryption profile, consent refs, invalidation refs, and usage refs.
  - Validation: Tests prove broad city, regional, country, continuous-background, sensitive private-layer, and over-budget offline packs remain disabled until explicit evidence and policy gates exist.

- **6.2 Implement storage, encryption, freshness, and invalidation refs.**
  - Design: Store public tile/layer refs through Overstore/Overbase, private offline refs through consent-scoped Overvault grants, and every offline manifest with freshness and invalidation policy.
  - Output: Storage adapter refs, encryption profile refs, freshness checks, invalidation events, stale states, repair refs, and deletion refs.
  - Validation: Tests prove stale, revoked, logged-out, device-revoked, app-uninstalled, or policy-invalidated offline areas cannot serve sensitive or private data.

- **6.3 Implement mobile capability budgets.**
  - Design: Route offline area builds, location permission reads, and mobile-safe projections through Mobile Backend Gateway and Mobile SDK without creating mobile-only authority or bypasses.
  - Output: Mobile-safe schemas, capability budgets, compact projections, offline progress refs, device scope refs, and mobile error mappings.
  - Validation: Mobile contract tests prove signing, device scope, revocation, usage/audit refs, redaction, and stable errors survive mobile client binding generation.

- **6.4 Implement mSwarm Runtime Bridge offline handoffs.**
  - Design: Use the bridge for offline area manifests, stale markers, source freshness, sync hints, and exact/current location expiry without bridge-owned location authority or private payload ownership.
  - Output: Bridge handoff refs, offline sync cursors, stale marker refs, expiration checks, conflict states, and replay refs.
  - Validation: Tests prove the bridge cannot widen location grants, keep raw payloads as canonical truth, skip revocation, or override Maps/Overvault/Overguard decisions.

- **6.5 Implement revocation, deletion, and cleanup behavior.**
  - Design: Delete or invalidate user-specific offline manifests and private location refs on permission revocation, logout, device revocation, stale freshness, app uninstall, or policy change.
  - Output: Cleanup jobs, deletion events, invalidation events, retained public shard rules, retention labels, legal-hold markers where required, and diagnostics.
  - Validation: Tests prove public seed shards may remain only when source/attribution policy allows, while user-specific private manifests and private location refs remain consent-scoped and revocable.

## Phase 7: Community Layers, Corrections, Moderation, And Disputes

### Work Items

- **7.1 Implement community map layer workflows.**
  - Design: Allow low-risk public-interest layers such as amenities, community resources, event maps, trails, accessibility notes, and neighborhood corrections with owner identity, source refs, confidence labels, rate limits, report links, and reversible state.
  - Output: Community layer create/update APIs, layer ownership records, source confidence, publish state, reversible state, report refs, and `maps_navigation.layer_created` events.
  - Validation: Tests prove community layers remain overlays and cannot become canonical place/listing/search truth.

- **7.2 Implement risk-tiered moderation.**
  - Design: Require review, limited visibility, Overclaim, or Fraud Control evidence for higher-risk layers involving private homes, vulnerable groups, safety hazards, emergency/disaster claims, regulated services, disputed boundaries, identity claims, exact sensitive locations, or harassment risk.
  - Output: Layer risk classifier facts, review queues, limited visibility states, deny states, reason codes, reviewer refs, and audit refs.
  - Validation: Tests prove high-risk layers cannot publish broadly without required review, evidence, policy, visibility, and dispute refs.

- **7.3 Implement correction and issue report intake.**
  - Design: Capture place, route, layer, locality, source, attribution, and privacy issue reports with evidence refs, source confidence, reporter protections, moderation refs, and state transitions.
  - Output: `POST /maps/corrections`, correction records, evidence refs, confidence updates, correction state machine, public-safe correction summaries, and `maps_navigation.place_correction_reported` events.
  - Validation: Tests prove corrections preserve prior state, source history, reporter refs, reviewer refs, reason codes, and audit evidence.

- **7.4 Implement Overclaim, Fraud Control, and Reputation handoffs.**
  - Design: Send redacted place/layer/correction/report refs to Overclaim, Fraud Control, and Reputation/Anti-Sybil for disputes, fraud recommendations, abuse controls, and risk recommendations rather than final Maps outcomes.
  - Output: Dispute refs, fraud handoff refs, reputation signal refs, recommendation refs, correction paths, appeal refs, and redacted evidence summaries.
  - Validation: Tests prove private fraud internals and anti-abuse thresholds remain redacted and Maps applies recommendations only through policy, reviewer, or owning-service finality rules.

- **7.5 Implement moderator, steward, and support views.**
  - Design: Provide redacted queues for corrections, source disputes, community-layer reports, location-privacy issues, route issues, attribution issues, Fraud/Reputation recommendations, and Overclaim refs.
  - Output: Moderator queue API, steward/operator projections, redaction badges, missing-evidence states, action eligibility reasons, audit links, replay links, and stable denial reasons.
  - Validation: Tests prove support and moderator views are role-scoped and never expose raw exact private locations, private route geometry, private offline manifests, vault secrets, unrelated profile data, or other-tenant evidence.

## Phase 8: Usage, Location Audit, Observability, Retention, And Replay

### Work Items

- **8.1 Emit usage refs for Maps operations.**
  - Design: Meter place reads/writes, tile/object access, local discovery, route computation, offline cache build/refresh, layer updates, corrections, handoffs, exports, replay, bandwidth, storage, and compute.
  - Output: Maps usage events, Overmeter refs, wallet receipt refs, operation dimensions, actor/app/device/place/route/layer/offline/source tags, and reconciliation refs.
  - Validation: Tests prove usage refs are emitted for successful and policy-denied operations where required and no usage event mutates ORU balances directly.

- **8.2 Implement Wallet/Usage Center location audit projections.**
  - Design: Show users which apps used location, precision, purpose, time, route/place refs, offline refs, usage refs, receipt refs, and revocation controls without pricing or revenue forecasts.
  - Output: Location audit projection API, wallet/usage refs, receipt refs, pending usage states, permission history, revocation shortcuts, and support-safe explanations.
  - Validation: Tests prove Maps does not maintain balances, payment records, ad bids, listing promotion, invoices, payout truth, hardcoded prices, revenue forecasts, or per-transaction fees.

- **8.3 Implement observability and operational metrics.**
  - Design: Track place volume, correction backlog, route latency, route denial rate, permission grant/revocation rate, precision-denial rate, local discovery latency, offline cache build failures, layer freshness, handoff failures, usage by resource class, and replay backlog.
  - Output: Metrics, traces, dashboards, alert rules, redacted logs, operational events, and runbook hooks.
  - Validation: Tests and drills prove alerts exist for unexpected background location attempts, exact-location access without consent, stale offline cache bursts, route failures, search/directory handoff denials, public layer abuse, correction spam, missing usage refs, and replay delays.

- **8.4 Implement audit exports and redacted replay bundles.**
  - Design: Export audit and replay evidence for users, moderators, stewards, auditors, legal/compliance, and support without exposing raw private data beyond audience policy.
  - Output: Audit export API, replay bundle export, BLAKE3 hash manifests, redaction profile refs, stable ordering, retention labels, and legal-hold markers.
  - Validation: Tests prove exports reconstruct permissions, route/place/layer decisions, handoffs, corrections, source refs, usage refs, and evidence refs while excluding raw mobility history, exact private location, private route geometry, vault secrets, fraud internals, and unrelated user data.

- **8.5 Implement retention and cleanup behavior.**
  - Design: Handle expired, revoked, stale, disputed, corrected, hidden, removed, and deleted place/route/layer/offline/location records through explicit retention classes and deletion/visibility rules.
  - Output: Retention policy refs, cleanup jobs, tombstone records where required, purge-denial errors, legal hold refs, and operator diagnostics.
  - Validation: Tests prove retention preserves required dispute/audit/source evidence but does not keep raw private location, route geometry, or offline private refs longer than policy permits.

## Phase 9: Native Client, Admin, SDK, Mobile, And AI Tool Surfaces

### Work Items

- **9.1 Implement Phase 12 native map user flows.**
  - Design: Build user-facing flows for map browsing, place search, place save, route request, route share, location permission grant/revoke, offline area management, correction reporting, community layer display, and replay/audit inspection.
  - Output: TypeScript client views or generated bindings, map views, route panels, permission controls, offline pack controls, correction/report affordances, attribution display, and usage/receipt links.
  - Validation: UI/API tests prove user flows call Overrid APIs and cannot bypass source policy, location permission, precision redaction, offline revocation, usage/audit emission, or moderation state.

- **9.2 Implement Directory, Search, and Messaging handoff ergonomics.**
  - Design: Show Directory listings, Search results, place refs, route refs, and Messaging shares as handoff projections with authority badges and privacy controls.
  - Output: Handoff UI components, authority badges, redaction badges, contact/share affordances, denied-state explanations, source attribution, and replay links.
  - Validation: Tests prove the UI does not imply Maps owns listing truth, Search ranking truth, direct messages, or exact private recipient location.

- **9.3 Implement Personal AI Assistant tool permission flows.**
  - Design: Allow Personal AI Assistant to request map/search/route/offline/correction tools only through explicit delegated native-app calls with just-in-time purpose-bound grants and visible audit.
  - Output: Delegated tool proposal schemas, permission prompts, grant expiry refs, per-turn coarse locality refs, exact/current/background/offline grant refs, denial states, and replay links.
  - Validation: Tests prove the assistant receives no location by default and cannot widen precision, publish corrections, save places, share routes, or download offline areas without explicit Maps and user-permission checks.

- **9.4 Implement SDK, CLI, admin, and support bindings.**
  - Design: Generate clients for places, routes, permissions, layers, offline areas, corrections, handoffs, usage, exports, and replay with stable JSON output.
  - Output: SDK bindings, CLI commands, admin/support projections, mobile-safe compact projections, fixture-based examples, and docs.
  - Validation: Contract tests prove generated clients preserve signing, idempotency, trace ids, tenant/app/device scope, stable errors, redaction states, source attribution, usage refs, and audit refs.

- **9.5 Implement accessibility, localization, and attribution UX.**
  - Design: Include route accessibility needs, attribution visibility, source freshness, confidence labels, localization-ready place/route strings, and accessible controls for permission and revocation.
  - Output: Accessibility preference refs, localized projection shapes, attribution components, confidence/freshness labels, keyboard/screen-reader-safe controls, and test fixtures.
  - Validation: Tests prove route preference explanations, source attribution, freshness/confidence, and permission states are visible without relying on hidden ad-ranking or surveillance cues.

## Phase 10: Phase 13 Hardening, Validation, Documentation, And Handoff

### Work Items

- **10.1 Harden location privacy, route safety, and compliance boundaries.**
  - Design: Connect exact/background/live/offline location, route modes, source licensing, emergency/delivery routing, public layers, sensitive places, retention, child-safety, and jurisdiction markers to Compliance Boundary, Overguard, Overclaim, Fraud Control, and incident paths.
  - Output: Readiness matrix, compliance-boundary refs, disabled/review-gated mode checklist, public notices, retention rules, and explicit disabled states.
  - Validation: Tests prove sensitive modes cannot be enabled by config drift, UI-only changes, search indexing, map layer import, or moderator action without required boundary refs.

- **10.2 Harden security, abuse, and incident response.**
  - Design: Threat-model source poisoning, exact-location leakage, background tracking, route privacy leakage, route manipulation, offline cache staleness, community-layer harassment, correction spam, attribution stripping, map handoff bypass, AI location misuse, and audit/replay gaps.
  - Output: Threat model entries, security review checklist, incident runbooks, privacy review checklist, mitigations, monitoring, owner assignments, and accepted-risk records where needed.
  - Validation: Security review confirms each listed threat has tests, monitoring, controls, incident paths, or explicit accepted risk.

- **10.3 Harden reliability and scale behavior.**
  - Design: Run drills for route service outage, source import backlog, tile/object repair, offline invalidation storm, location revocation burst, exact-location denial spikes, handoff lag, correction flood, community-layer abuse, usage reconciliation lag, retention cleanup lag, and replay backlog.
  - Output: Drill plans, expected behavior, actual behavior, evidence refs, metrics thresholds, remediation tasks, and release gates.
  - Validation: Drills prove Maps degrades without losing place truth, leaking private location, skipping usage/audit refs, serving stale sensitive offline data, or granting downstream services final authority.

- **10.4 Validate documentation and implementation handoff readiness.**
  - Design: Recheck SDS/service/build-plan links, 10 sub-build phases, work-item structure, local Markdown links, queue state, stack guardrails, location/privacy/source/revenue/ad-trap scans, and Docdex retrieval.
  - Output: Validation evidence in `docs/build_plan/progress.md`, queue progress update, Docdex index refresh, Docdex search evidence, and run-test blocker note if unchanged.
  - Validation: Focused scripts pass for title prefix, attached SDS link, phases 1 through 10, Design/Output/Validation work-item structure, local links, final newlines, no unresolved markers, and stack guardrails.

- **10.5 Hand off to Phase 12 implementation and Phase 13 hardening.**
  - Design: Summarize the minimum buildable Phase 12 slice and the later Phase 13 hardening scope so implementation can start without re-litigating ownership, stack choices, source policy, route modes, precision defaults, or offline/community-layer boundaries.
  - Output: Handoff checklist covering contracts, schemas, APIs, source provenance, place/layer/route/location/offline/correction records, Directory/Search/Messaging/AI handoffs, usage/audit/replay, client surfaces, and hardening gates.
  - Validation: Review confirms the plan remains internally consistent, consistent with SDS #70, consistent with service catalog and master Phase 0-13 docs, and consistent with `docs/overrid_tech_stack_choice.md`.
