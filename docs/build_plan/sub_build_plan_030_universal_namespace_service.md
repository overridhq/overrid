# SUB BUILD PLAN #30 - Universal Namespace Service

Attached SDS: [docs/sds/data_storage_namespace/universal_namespace_service.md](../sds/data_storage_namespace/universal_namespace_service.md)

## Purpose

This sub-build plan turns SDS #30 into an implementation sequence for the Universal Namespace Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

The Universal Namespace Service is the readable naming, route-binding, delegation, transfer, verification, dispute, tombstone, and privacy-aware resolution layer for Overrid. It owns normalized names and namespace records. It does not own identity, asset rights, ledger state, object bytes, private vault material, private connectivity, billing mutation, or a public DNS product boundary.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #30: Universal Namespace Service](../sds/data_storage_namespace/universal_namespace_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Universal Namespace Service plan](../service_catalog/data_storage_namespace/universal_namespace_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, canonical JSON/JSON Schema discipline, signed envelopes, idempotency, trace ids, stable reason codes, local fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, identity, tenant, key, command, and audit primitives that namespace operations consume. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy checks, abuse controls, workload/data-class policy, verification evidence, challenge/dispute hooks, and Overclaim correction flow. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes namespace raw usage through Overmeter while keeping rollups, ORU balances, Seal Ledger mutation, billing, settlement, payout, and external payment rails outside namespace logic. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service operation, backup/restore patterns, failover/recovery expectations, and grid-resident readiness prerequisites. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Controls the first full build point for Universal Namespace after Overbase, Overstore, Overvault, Overmesh, Overasset, data-class, and ownership primitives are ready. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies app identity, service route, release, package, and deployment-intent route refs without moving deployment semantics into the namespace service. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Adds public-interest purpose tags, federation-scoped names, stewarded namespaces, and cross-tenant evidence gates. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Adds public-provider exposure only for approved public or public low-sensitivity route/target classes without leaking private routes or identities. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Consumes namespace records for wallet, personal AI, workspace, directory, search, messaging, social, maps, central AI, and mobile client addressability. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies retention, compliance hold, incident, threat-model, audit-export, migration, PIP, appeal, and governance hardening for names, verification, disputes, and route changes. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #30 first full build work aligned to master Phase 8, with earlier prerequisites and later federation/public/native/governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, native Overrid service boundaries, and no conventional DNS, database, object-store, vault/KMS, queue, blockchain, NFT, or SaaS product-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 7, 8, 10, 11, 12, and 13 | Attach SDS #30, freeze namespace scope, preserve Phase 8 as first full build point, and record prerequisite plus later public/native/governance gates. |
| 2 | Master Phases 0, 1, 4, 7, and 8 | Build Rust contracts, JSON Schemas, normalization rules, state machines, reserved-name fixtures, and deterministic harnesses before namespace side effects. |
| 3 | Master Phases 1, 4, 5, and 8 | Implement claims, reservations, uniqueness checks, owner refs, policy-denial evidence, raw usage facts, and namespace record activation. |
| 4 | Master Phases 4, 8, 9, and 12 | Implement target refs, route bindings, privacy-aware resolution, Overmesh handoff, route diagnostics, and cache invalidation. |
| 5 | Master Phases 4, 5, 8, and 13 | Implement delegation, revocation, signed transfers, rights/ledger evidence refs, releases, tombstones, and correction-preserving history. |
| 6 | Master Phases 4, 8, 10, 12, and 13 | Implement verification markers, reserved-name enforcement, issuer policy, anti-squatting, impersonation controls, and stewarded markers. |
| 7 | Master Phases 4, 8, 11, and 13 | Implement disputes, restrictions, appeal paths, route-hijack handling, Overclaim integration, incidents, and correction records. |
| 8 | Master Phases 6, 8, 9, 10, 11, and 12 | Integrate messaging, search, maps, directory listings, native apps, AI agents, app routes, SDK/CLI/admin clients, and federation/public scopes. |
| 9 | Master Phases 5, 7, 8, 12, and 13 | Integrate Overwatch, Overmeter, dependency health, operations views, backup/restore, migration, and governance handoffs. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, tech-stack alignment, queue state, progress evidence, negative controls, and implementation handoff gates. |

## Tech Stack Guardrails

- Universal Namespace core is a Rust service/module using shared contract types, Tokio for async cache/dispute/cleanup workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Namespace contracts use canonical JSON plus JSON Schema for docs-facing examples, fixtures, claims, records, owner refs, target refs, route bindings, delegations, transfers, verification markers, disputes, tombstones, cache entries, events, reason codes, and replay reports. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating calls use signed envelopes, tenant/org/app/community scope, actor or service-account identity, idempotency keys, trace ids, schema versions, stable reason codes, policy refs, data-class refs, visibility refs, and append-only audit events.
- Ed25519 is used where signatures are required. BLAKE3/content fingerprints are used for claim evidence, route-binding evidence, fixture commitments, cache invalidation commitments, dispute bundles, and replay reports.
- Overbase stores namespace records when Phase 8 primitives exist, but Universal Namespace owns normalized-name uniqueness, namespace lifecycle, owner refs, route-binding semantics, verification markers, disputes, tombstones, and resolution behavior.
- Overpass owns identity records and usernames. Overasset and Seal Ledger own rights and ownership evidence. Overmesh owns private connectivity and endpoint health. Overguard owns policy finality. Overclaim owns dispute/correction workflow. Overwatch owns audit/evidence retention. Overmeter owns usage rollups. Accounting services own ORU, billing, settlement, payout, and external payment rails.
- Public resolution must not expose private identities, tenant-private routes, secret-bearing endpoints, private storage refs, private vault refs, non-public asset refs, or cross-tenant existence signals.
- PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, external DNS product boundaries, blockchain, NFT, pricing, revenue, customer-count, or external-payment assumptions must not become the namespace service product boundary.
- TypeScript is limited to generated bindings and operator/developer UI surfaces. The core namespace runtime, normalization, policy checks, resolution, cache invalidation, dispute integration, and validation stay Rust-first.

## Phase 1: SDS Attachment, Namespace Scope, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #30.**
  - Design: Link this document from the numbered Universal Namespace SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/data_storage_namespace/universal_namespace_service.md`, `docs/service_catalog/data_storage_namespace/universal_namespace_service.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #30 returns both the Universal Namespace SDS and this sub-build plan.

- **1.2 Freeze Universal Namespace as the naming and resolution authority.**
  - Design: Record that the service owns normalized names, namespace records, claim records, owner refs, target refs, route bindings, delegation grants, transfer records, verification markers, dispute refs, reservation records, tombstones, and resolution cache entries.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Universal Namespace does not own identity, credentials, asset-right mutation, Seal Ledger entries, object bytes, private vault payloads, private connectivity, ORU balances, invoices, payouts, or billing state.

- **1.3 Preserve master Phase 8 as the first full build point.**
  - Design: Keep first full implementation in master Phase 8 because namespace depends on signed identity/tenant/request/audit rails, policy controls, raw usage emission, grid-resident readiness, Overbase storage, Overasset evidence, and Overmesh route resolution.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, and 7 are prerequisites and Phase 8 proves native namespace behavior before broad native-app consumption.
  - Validation: Review proves this plan does not move full namespace implementation into earlier master phases and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #30 decisions for classed uniqueness, day-one reserved names, evidence-threshold anti-squatting/impersonation controls, issuer-scoped verification markers, and tiered route-binding approvals.
  - Output: Resolved-decision checklist tied to SDS #30 open-question answers.
  - Validation: Review rejects global-only name semantics, unreviewable automatic deletion, verification markers as ownership, route broadening without policy, NFT-like speculative name assets, and private-route leakage through public resolution.

- **1.5 Define namespace authority boundaries.**
  - Design: Create a boundary matrix for Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, Overbase, Overstore, Overvault, Overmesh, Overasset, Seal Ledger, Overclaim, Oververify, Challenge Task Service, Overpack, native apps, SDK, CLI, admin UI, compliance services, and governance services.
  - Output: Boundary matrix listing consumed refs, emitted refs, denial behavior, retry owner, redaction profile, cache owner, dispute owner, and replay evidence for each dependency.
  - Validation: Review confirms every dependency has an owning service and Universal Namespace receives or emits facts through explicit APIs, refs, policy decisions, and audit evidence rather than privileged direct state access.

## Phase 2: Rust Contracts, Schemas, Normalization, And Fixtures

### Work Items

- **2.1 Create the Universal Namespace Rust contract module.**
  - Design: Add contract types for namespace records, name claims, owner refs, target refs, route bindings, delegation records, transfer records, verification markers, dispute records, reservations, tombstones, resolution cache entries, state enums, event payload refs, API errors, and reason codes.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, reason-code catalog, state enums, API error types, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms namespace contracts remain separate from identity, asset-rights, route-health, storage, vault, policy, audit, and accounting logic.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for claim request, namespace record, owner ref, target ref, route binding, resolve response, metadata read, delegation grant, delegation revoke, transfer request, verification marker, dispute request, restriction, release, tombstone, cache entry, and API errors.
  - Output: Schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing tenant/org/app/community scope, actor/service-account identity, trace id, idempotency key, namespace type, normalized name, owner refs, target refs, visibility, data class, policy refs, ttl, and audit refs where required.

- **2.3 Define normalization and reserved-name rules.**
  - Design: Model raw name capture, normalized name output, separator folding, case folding, Unicode-equivalent detection, confusing-name variants, reserved root names, canonical service/native-app names, scoped exceptions, and safe conflict reason codes.
  - Output: Normalization library contract, reserved-name registry schema, test vectors, exception policy shape, and conflict response contract.
  - Validation: Normalization tests prove equivalent names collide safely, reserved names deny by default, scoped exceptions require explicit policy refs, and public errors do not leak cross-tenant private namespace existence.

- **2.4 Define lifecycle and state transitions.**
  - Design: Model claim_requested, reserved, active, delegated, pending_transfer, transferred, verified, disputed, restricted, suspended, released, tombstoned, proposed, policy_checked, degraded, replaced, revoked, and disputed route-binding states.
  - Output: State transition table, legal transition rules, terminal/overlay state semantics, route-binding lifecycle rules, reason codes, and event payload refs.
  - Validation: State tests reject activation before policy allow, transfer without required signatures, verification without issuer authority, public resolution of private targets, route changes under blocking dispute, and deletion that rewrites history.

- **2.5 Create deterministic fixtures and harness scenarios.**
  - Design: Build fixtures for global, tenant-local, app-local, and community-local names; reserved-name denial; claim acceptance; duplicate claim conflict; route bind; private resolution redaction; delegation; transfer; marker issue/revoke; dispute restriction; release; tombstone; and cache invalidation.
  - Output: Fixture directory, expected API responses, events, audit refs, usage facts, redacted diagnostics, and replay hashes.
  - Validation: Fixture tests produce stable output and prove ambiguous or missing facts fail closed as denied, restricted, review-required, degraded, suspended, or tombstoned states.

## Phase 3: Claims, Reservations, Uniqueness, And Owner Records

### Work Items

- **3.1 Implement name claim submission.**
  - Design: Accept namespace type, raw name, scope, claimant ref, target intent, evidence refs, reservation type, data class, visibility, policy refs, idempotency key, and trace id through a signed command envelope.
  - Output: `POST /namespace/claims`, claim record, normalized-name preview, policy-check request, metadata-only response, and `namespace.claim_requested` event.
  - Validation: Claim tests reject missing scope, unsupported namespace type, unauthorized actor, malformed raw name, missing target intent, missing evidence where required, duplicate incompatible idempotency keys, and hidden cross-tenant leakage.

- **3.2 Implement uniqueness and reservation checks.**
  - Design: Check global, tenant-local, app-local, and community-local uniqueness rules plus reserved-name registry, tombstone/no-reuse rules, active disputes, and scoped exceptions before activation.
  - Output: Uniqueness index contract, reservation lookup, conflict response, denial reason codes, and `namespace.claim_denied` event path.
  - Validation: Tests prove globally unique names collide globally, scoped names collide only inside authority boundary, reserved names deny safely, and tombstoned names follow reuse policy.

- **3.3 Implement namespace record activation.**
  - Design: Convert allowed claims into namespace records with namespace id, raw name, normalized name, type, scope, owner ref, initial target refs, visibility, data class, current state, policy refs, audit refs, and usage refs.
  - Output: Namespace record writer, owner-ref creation, first target-ref record, `namespace.claim_accepted` event, and raw usage fact.
  - Validation: Activation tests confirm records are append-only, owner refs are explicit, current pointers are versioned, and policy-denied or stale claims cannot become active.

- **3.4 Implement owner ref management.**
  - Design: Store owner type, owner id/ref, rights refs, effective time, transfer eligibility, audit refs, and policy refs without moving identity or asset-right authority into namespace logic.
  - Output: Owner-ref schema implementation, owner history view, metadata read fields, and ownership evidence replay bundle.
  - Validation: Tests prove owner refs can reference Overpass, Overtenant, Overasset, and Seal Ledger evidence while Universal Namespace never mutates identity records, asset rights, ledger entries, balances, or settlement state.

- **3.5 Emit raw claim and owner usage events.**
  - Design: Emit usage facts for claim attempts, accepted claims, denied claims, reservation checks, owner-ref writes, policy-denied operations, and metadata reads.
  - Output: Overmeter raw usage contract with dimensions for tenant, actor, namespace type, scope, data class, operation kind, decision state, and policy refs.
  - Validation: Usage tests prove namespace emits raw facts only and never mutates ORU balances, Seal Ledger entries, invoices, payout holds, pricing, or external payment systems.

## Phase 4: Targets, Route Bindings, Resolution, And Cache Invalidation

### Work Items

- **4.1 Implement target ref binding.**
  - Design: Add or update target refs with target type, target id/ref, visibility, priority, route policy refs, disclosure rule, data class, and current-state metadata through signed owner/delegate commands.
  - Output: `POST /namespace/{namespace_id}/targets`, target-ref records, versioned target history, redacted metadata response, and `namespace.target_bound` event.
  - Validation: Target tests reject unauthorized actors, unsupported target types, private target exposure, data-class broadening without policy, missing disclosure rules, and silent overwrite of previous target history.

- **4.2 Implement route binding workflow.**
  - Design: Bind route kind, Overmesh route ref, app/service route, storage ref, api route, native app page ref, ttl, health refs, policy refs, and review/hold requirements.
  - Output: `POST /namespace/{namespace_id}/routes`, route-binding record, policy report, active route pointer, and `namespace.route_bound` or `namespace.route_changed` event.
  - Validation: Route tests prove low-risk scoped changes activate after signed policy checks, while public/global roots, verified names, target-type changes, visibility broadening, private/regulated/secret-bearing routes, disputed names, and system-service roots require delay, review, or multi-signature gates.

- **4.3 Implement privacy-aware resolution.**
  - Design: Resolve by normalized name or namespace id while applying tenant, role, data-class, target visibility, route-disclosure, dispute, suspension, and public/private policy.
  - Output: `GET /namespace/{name}`, `GET /namespace/{namespace_id}`, resolve response, metadata response, redaction matrix, and safe not-found/conflict behavior.
  - Validation: Resolution tests prove public callers cannot see private identities, tenant-private routes, secret-bearing endpoints, private storage refs, non-public asset refs, or cross-tenant existence signals.

- **4.4 Implement resolution cache and invalidation.**
  - Design: Cache lookup key, resolved target refs, route refs, policy refs, visibility, ttl, invalidation refs, cache state, and safe stale-serving rules tied to target/route/visibility/dispute changes.
  - Output: Resolution cache entry records, invalidation worker, `namespace.resolution_cache_invalidated` event, and replayable cache invalidation report.
  - Validation: Cache tests prove every target, route, visibility, data-class, marker, dispute, suspension, transfer, and tombstone change invalidates affected entries and stale data is served only when policy permits.

- **4.5 Integrate Overmesh route diagnostics.**
  - Design: Query Overmesh route state and endpoint health refs without making Universal Namespace the private connectivity or endpoint-health authority.
  - Output: Route diagnostic view, degraded route state, safe response hints, dependency health fields, and Overmesh handoff contract.
  - Validation: Tests prove Overmesh unavailable marks route resolution degraded where appropriate, keeps namespace records intact, and does not leak private endpoint details to unauthorized callers.

## Phase 5: Delegation, Transfers, Rights Evidence, And Tombstones

### Work Items

- **5.1 Implement delegation grant creation.**
  - Design: Grant limited namespace management rights to delegates with delegator ref, delegate ref, namespace scope, allowed operations, ttl, revocation state, policy refs, and audit refs.
  - Output: `POST /namespace/{namespace_id}/delegations`, delegation record, operation capability matrix, and `namespace.delegation_created` event.
  - Validation: Delegation tests reject overbroad scopes, missing ttl where required, unauthorized delegators, cross-tenant leakage, unsupported operations, and delegated changes that bypass owner or policy gates.

- **5.2 Implement delegation revocation.**
  - Design: Revoke delegation by owner/delegator/policy decision with revocation reason, effective time, affected operations, route/cache invalidation refs, and audit evidence.
  - Output: `POST /namespace/{namespace_id}/delegations/{delegation_id}/revoke`, revocation record, invalidation report, and `namespace.delegation_revoked` event.
  - Validation: Revocation tests prove revoked delegates cannot perform later mutations, existing pending route/transfer actions are rechecked, and revocation does not erase previous delegation evidence.

- **5.3 Implement signed transfer workflow.**
  - Design: Collect source owner authorization, target owner authorization, required signatures, rights refs, policy refs, ledger/evidence refs, review/hold windows, rollback/tombstone behavior, and cache invalidation.
  - Output: `POST /namespace/{namespace_id}/transfer`, transfer record, signature checklist, decision state, and `namespace.transfer_requested` or `namespace.transfer_completed` events.
  - Validation: Transfer tests reject missing source/target authorization, insufficient rights evidence, active blocking disputes, route-hijack indicators, stale signatures, and silent owner reassignment.

- **5.4 Implement release and tombstone behavior.**
  - Design: Release names according to reuse policy while preserving tombstone reason, visibility, former namespace id, dispute refs, retention, policy refs, and appeal/finality refs.
  - Output: `POST /namespace/{namespace_id}/release`, namespace tombstone record, release record, cache invalidation, and `namespace.tombstoned` event.
  - Validation: Release tests prove normal reuse follows policy, high-risk names can remain blocked, tombstones preserve history, and deletion never erases audit or dispute evidence.

- **5.5 Implement correction-preserving history.**
  - Design: Append correction records for owner, target, route, verification, dispute, and tombstone changes while preserving prior versions and current pointers.
  - Output: Versioned history view, correction ref model, replay report, and safe rollback notes.
  - Validation: History tests prove corrections create new records instead of rewriting prior ownership or route history and replay can explain current namespace state from stored facts.

## Phase 6: Verification Markers, Reserved Names, And Abuse Controls

### Work Items

- **6.1 Implement issuer-scoped verification markers.**
  - Design: Issue markers with namespace id, marker kind, issuer ref, evidence refs, expiration, revocation state, display policy, policy refs, and audit refs.
  - Output: `POST /namespace/{namespace_id}/verification`, marker record, marker display view, and `namespace.verification_added` event.
  - Validation: Marker tests prove markers are time-bounded, issuer-scoped, revocable evidence records and do not become identity, ownership, transfer authority, or asset rights by themselves.

- **6.2 Implement marker revocation and expiry.**
  - Design: Revoke or expire markers through issuer action, lost issuer authority, credential compromise, abuse policy, Overclaim resolution, or Overguard emergency block.
  - Output: Revocation record, expiry worker, marker state update, cache invalidation, and `namespace.verification_revoked` event.
  - Validation: Revocation tests prove display policy updates immediately, resolution cache invalidates, prior marker evidence remains replayable, and revoked markers cannot authorize transfers or public claims.

- **6.3 Enforce day-one reserved names and variants.**
  - Design: Protect canonical Overrid brand, protocol, system-service, route-root, native-app, operator/admin words, canonical service ids, native app roots, and confusing/Unicode/separator variants.
  - Output: Reserved-name registry, policy exception workflow, reason-code mapping, and validation fixtures.
  - Validation: Tests deny protected names by default, allow only explicit scoped exceptions, and preserve safe denial messages without publicizing protected internal variants beyond policy-approved displays.

- **6.4 Implement anti-squatting and impersonation controls.**
  - Design: Use graduated evidence thresholds for hard denials, temporary restrictions, review windows, disputes, public-interest names, high-traffic names, confusing names, abandoned names, and misleading markers.
  - Output: Abuse policy interface, evidence checklist, restriction records, reason codes, and review queue fields.
  - Validation: Abuse tests prove hard denials apply to active/reserved/tombstoned/protected collisions, temporary restrictions require credible evidence refs, and permanent transfer/suspension/tombstone requires Overclaim/Overguard/Overwatch evidence.

- **6.5 Implement community and public-interest marker policies.**
  - Design: Support organization markers, community markers, public-interest markers, stewardship-issued markers, renewal rules, local display boundaries, and promotion rules without collapsing scopes.
  - Output: Marker kind matrix, issuer authority checks, renewal jobs, display-policy examples, and policy fixtures.
  - Validation: Tests prove community markers display only within their community unless policy promotes them, public-interest markers require authorized stewardship evidence, and expired/compromised issuer credentials revoke marker authority.

## Phase 7: Disputes, Restrictions, Appeals, And Corrections

### Work Items

- **7.1 Implement dispute opening.**
  - Design: Open disputes for impersonation, squatting, misleading names, abandoned names, unauthorized transfer, route hijack, trademark/community conflicts where applicable, and correction requests.
  - Output: `POST /namespace/{namespace_id}/disputes`, dispute record, affected route refs, temporary restriction options, evidence refs, and `namespace.dispute_opened` event.
  - Validation: Dispute tests reject missing claimant refs, missing evidence where required, unsupported dispute types, duplicate incompatible idempotency keys, and unauthorized visibility into private target details.

- **7.2 Implement evidence-based temporary restrictions.**
  - Design: Restrict route changes, visibility broadening, transfers, marker display, or resolution only when credible evidence and policy allow a temporary hold.
  - Output: Restriction record, affected operation matrix, cache invalidation, operator review fields, and `namespace.dispute_restricted` event.
  - Validation: Restriction tests prove restrictions are proportionate, reason-coded, appealable, ttl/review bounded where appropriate, and never silently delete or reassign names.

- **7.3 Integrate Overclaim resolution and appeal refs.**
  - Design: Consume Overclaim decisions, appeal refs, finality refs, correction refs, and affected-party notice/response evidence to resolve disputes or maintain restrictions.
  - Output: Overclaim handoff contract, decision importer, finality/ref matrix, and `namespace.dispute_resolved` event.
  - Validation: Tests prove permanent transfer, reassignment, suspension, or tombstone requires Overclaim resolution, Overguard policy decision, appeal/finality refs, and Overwatch audit evidence.

- **7.4 Implement route-hijack and incident handling.**
  - Design: Preserve previous safe route, restrict risky route change, open dispute or incident refs, separate namespace problems from Overmesh endpoint-health problems, and maintain active approved binding.
  - Output: Route-hijack workflow, safe-route selection rules, incident evidence refs, degraded response behavior, and operator diagnostics.
  - Validation: Hijack tests prove suspicious route changes cannot replace active approved bindings without review and prior versions remain replayable.

- **7.5 Implement suspension and correction paths.**
  - Design: Suspend names through policy/dispute decisions and append correction records for owner, target, marker, route, visibility, tombstone, and dispute outcomes.
  - Output: `POST /namespace/{namespace_id}/suspend`, suspension record, correction record, current-pointer update, and audit replay bundle.
  - Validation: Tests prove suspended names block normal mutation/resolution according to policy, corrections preserve prior evidence, and release/tombstone behavior respects dispute outcome and retention.

## Phase 8: Native App, Search, Messaging, Maps, And Agent Integration

### Work Items

- **8.1 Integrate messaging and identity-addressed clients.**
  - Design: Provide resolution flows for usernames, organization names, service accounts, conversation targets, mobile clients, and private/public visibility classes without replacing Overpass identity authority.
  - Output: Messaging resolution contract, privacy matrix, contact-display fields, blocked/degraded states, and fixture scenarios.
  - Validation: Messaging tests prove names resolve to authorized identity or app refs and do not expose private identities, tenant-local aliases, or cross-tenant existence signals.

- **8.2 Integrate search and directory listings.**
  - Design: Expose authorized namespace metadata for search, directory listings, business/place names, community names, tags, and public-interest names while respecting marker, dispute, and data-class policy.
  - Output: Search projection contract, directory metadata view, redaction policy, stale projection invalidation, and reindex hooks.
  - Validation: Search tests prove private namespaces are not globally indexed, disputed/restricted names adjust display safely, and search projections rebuild or tombstone without rewriting history.

- **8.3 Integrate maps, social, workspace, wallet, and native app routes.**
  - Design: Bind public place/business names, app routes, workspace pages, wallet usage center entries, social handles, maps/navigation pages, and native app page refs through policy-checked route bindings.
  - Output: Native app route contract, target-kind matrix, app-route examples, route ownership refs, and display rules.
  - Validation: Native app tests prove route roots, high-traffic names, verified organization/public-interest names, and visibility broadening require review or multi-signature gates where SDS #30 requires them.

- **8.4 Integrate agents, swarms, services, and Overpack deployments.**
  - Design: Support agent names, swarm names, service names, system-service route roots, app/service route roots, Overpack deployment route refs, and package/deployment handoffs without moving deployment or scheduling semantics into namespace logic.
  - Output: Agent/swarm/service namespace schemas, Overpack route-binding handoff, system-service protected route policy, and deployment fixture set.
  - Validation: Tests prove system-service and native-app public roots require multi-signature approval and active Overmesh-approved binding while package/deployment semantics stay in Overpack/deployment services.

- **8.5 Integrate SDK, CLI, admin UI, and generated bindings.**
  - Design: Provide generated client contracts for claim, resolve, bind, delegate, transfer, verify, dispute, suspend, release, cache invalidation, and redacted metadata reads.
  - Output: SDK/CLI/admin capability matrix, command/API examples, UI-safe fields, role-based actions, and audit views.
  - Validation: Client tests prove surfaces call Overgate/namespace APIs, preserve signed envelopes/idempotency/trace ids, and never become privileged bypasses for private route/target disclosure.

## Phase 9: Observability, Metering, Operations, And Dependency Handoffs

### Work Items

- **9.1 Integrate Overwatch audit and evidence records.**
  - Design: Emit append-only events for claim, denial, target bind, route bind/change, delegation, transfer, verification, dispute, restriction, suspension, release, tombstone, and cache invalidation operations.
  - Output: Overwatch event contract, redaction profile, evidence-ref matrix, replay report, and operator audit views.
  - Validation: Audit tests prove all mutating operations emit events and unauthorized/public event views redact private target, route, identity, storage, and dispute-sensitive details.

- **9.2 Integrate Overmeter raw namespace usage.**
  - Design: Emit usage facts for claim attempts, accepted/denied claims, resolution reads, metadata reads, target changes, route changes, delegations, transfers, verification changes, disputes, restrictions, releases, tombstones, cache invalidations, and policy-denied operations.
  - Output: Overmeter usage contract, operation dimensions, data-class dimensions, policy refs, decision states, and retention notes.
  - Validation: Usage tests prove denied and dispute operations remain visible for audit/risk review and Universal Namespace never mutates ORU balances, Seal Ledger entries, provider payouts, invoices, or payment systems.

- **9.3 Integrate dependency health checks.**
  - Design: Report health for normalization, uniqueness index, Overbase storage, Overguard, Overpass, Overtenant, Overkey, Overasset, Seal Ledger evidence refs where used, Overmesh route checks, Overwatch emission, Overmeter emission, Overclaim handoff, cache invalidation, and async workers.
  - Output: Health endpoint/schema, metrics/events, dashboard fields, dependency outage behavior, and safe diagnostics.
  - Validation: Health tests prove dependency outages block or degrade operations safely and diagnostics include evidence refs and reason codes without leaking private target/route details.

- **9.4 Prepare backup, restore, and migration handoffs.**
  - Design: Document how Backup and Restore Service, Migration Tooling, Incident Response Service, Compliance Boundary Service, Overbase, Overwatch, Overmesh, Overasset, and native apps preserve namespace records, history, cache state, tombstones, disputes, restrictions, and evidence.
  - Output: Handoff matrix, dependency owners, ref lifecycles, restore notes, migration notes, retention notes, and phased readiness checklist.
  - Validation: Handoff review confirms downstream consumers can use namespace refs without weakening Overpass, Overasset, Seal Ledger, Overmesh, Overguard, Overwatch, Overmeter, Overclaim, or governance boundaries.

- **9.5 Implement operational review surfaces.**
  - Design: Provide operator views for claim queues, denied claims, reserved-name hits, active disputes, route changes, transfer workflows, verification markers, cache invalidations, abuse reason trends, and degraded route resolution.
  - Output: Admin/API read models, filtering fields, redaction policy, audit links, queue/retry views, and incident hooks.
  - Validation: Operations tests prove views are role-scoped, private fields redact by default, and operators can distinguish namespace issues from Overmesh endpoint-health issues.

## Phase 10: Validation, Documentation, Queue State, And Handoff

### Work Items

- **10.1 Validate contract and schema coverage.**
  - Design: Run focused checks for namespace records, claims, owner refs, target refs, route bindings, delegations, transfers, verification markers, disputes, restrictions, reservations, tombstones, resolution cache entries, APIs, events, and reason codes.
  - Output: Schema-test report, state-machine test report, fixture coverage matrix, failure notes, and remediation list.
  - Validation: Tests pass before implementation advances beyond each documented gate; blockers are recorded in build-plan progress.

- **10.2 Validate Phase 8 namespace behavior end to end.**
  - Design: Prove one signed tenant/app flow claims a name, activates it, binds a target, binds an Overmesh route, resolves it privately and publicly, delegates management, transfers ownership, issues a marker, opens a dispute, restricts a route, resolves the dispute, releases the name, and tombstones history.
  - Output: End-to-end namespace fixture, source-ref bundle, operation records, usage refs, audit trail, cache invalidation report, and replay report.
  - Validation: Replay confirms successful, denied, conflict, private-redacted, delegated, transferred, verified, disputed, restricted, suspended, released, tombstoned, degraded, and cache-invalidated paths produce distinct auditable states.

- **10.3 Validate security, privacy, and tech-stack alignment.**
  - Design: Scan implementation and docs for private-route leakage, private-identity leakage, cross-tenant existence leakage, external DNS product-boundary drift, identity-authority drift, asset-right drift, ledger/accounting mutation, object-storage drift, vault drift, policy-authority drift, blockchain/NFT mechanics, pricing/revenue/customer-count assumptions, and TypeScript core-runtime drift.
  - Output: Security/privacy checklist, tech-stack alignment report, negative-control scan results, and remediation notes.
  - Validation: Review confirms Universal Namespace remains Rust-first/native-Overrid infrastructure and uses canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content fingerprints, signed envelopes, policy refs, audit refs, privacy-aware resolution, and native service boundaries.

- **10.4 Validate master-plan and downstream handoff alignment.**
  - Design: Confirm SDS #30, the Universal Namespace service plan, master build plan, build-plan crosswalk, Phase 8 plan, Phase 9 plan, Phase 10 plan, Phase 11 plan, Phase 12 plan, Phase 13 plan, queue state, and progress docs link to this plan and preserve the Phase 8 first full build point.
  - Output: Updated source-document links, sub-build-plan index entries, progress evidence, queue status, and downstream handoff matrix.
  - Validation: Markdown link checks pass and review confirms no master Phase 0 through Phase 13 ordering change was required.

- **10.5 Validate implementation handoff readiness.**
  - Design: Prepare the handoff for builders by listing required crates/modules, schemas, reason-code catalogs, normalization library, reserved-name registry, policy hooks, route handoff, fixture groups, cache scenarios, dispute scenarios, acceptance tests, and phase gates.
  - Output: Implementation handoff checklist, validation command list, known blockers, dependency owners, and first namespace-backed native app fixture target.
  - Validation: Handoff review confirms a builder can start Universal Namespace Phase 8 implementation without reading informal agent notes or weakening SDS boundaries.

## Alignment Review

- The sub-build plan keeps Universal Namespace first full build work in master Phase 8, matching SDS #30, the service catalog entry, Phase 8 plan, master build plan, and build-plan crosswalk.
- The plan treats Phases 0, 1, 4, 5, and 7 as prerequisites for shared schemas, signed identity/tenant/request/audit rails, policy controls, raw usage emission, and protected system-service readiness rather than as full namespace implementation phases.
- The plan keeps Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, Overbase, Overstore, Overvault, Overmesh, Overasset, Seal Ledger, Overclaim, Oververify, Challenge Task Service, Overpack, native apps, SDK, CLI, admin UI, compliance services, and governance authority outside Universal Namespace while defining the refs and evidence Universal Namespace consumes or emits.
- The plan preserves SDS #30 resolved decisions: classed uniqueness by authority boundary, day-one reserved canonical names and variants, graduated evidence thresholds for anti-squatting and impersonation, issuer-scoped verification markers, and tiered route-binding approvals.
- The plan gates public-provider and public resolution behavior through strict visibility, data-class, route-disclosure, dispute, and Phase 11 low-sensitivity constraints so private routes, private identities, secret-bearing endpoints, and non-public storage refs are not exposed.
- The plan keeps long-running cache invalidation, dispute review, route review, marker expiry, transfer review, correction, migration, backup/restore, projection rebuild, and cleanup work in queued or resumable jobs instead of ordinary request budgets.
- The plan preserves the master Phase 0 through Phase 13 order and uses later phases only for deployment-route provisioning, federation/public-interest scopes, public-provider constraints, native-app consumption, and governance/compliance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first Universal Namespace core, native Overrid boundaries, Tokio, Axum/Tower/Hyper-style service boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content fingerprints, signed envelopes, privacy-aware resolution, audit refs, policy refs, and no conventional DNS, database, object-store, vault/KMS, queue, blockchain, NFT, pricing, revenue, customer-count, or external-payment assumptions.
