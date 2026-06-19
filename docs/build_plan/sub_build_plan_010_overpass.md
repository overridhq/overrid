# SUB BUILD PLAN #10 - Overpass

Attached SDS: [docs/sds/control_plane/overpass.md](../sds/control_plane/overpass.md)

## Purpose

This sub-build plan turns SDS #10 into an implementation sequence for Overpass. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overpass is the durable identity and namespace authority for Overrid. It creates stable identity records, subject refs, lifecycle history, namespace records, route bindings, verification markers, tombstones, merge refs, recovery refs, and dispute refs while leaving credentials, tenant authorization, accounting, policy finality, profile content, and private evidence with their owning services.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #10: Overpass](../sds/control_plane/overpass.md) | Controls Overpass purpose, data model, API surface, lifecycle states, namespace scope, verification marker rules, recovery/dispute decisions, validation, and resolved open-question decisions. |
| [Overpass service plan](../service_catalog/control_plane/overpass.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, local stack, deterministic fixtures, API/event conventions, and integration harnesses required before Overpass implementation. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Controls the first build point for Overpass-lite as the identity primitive in the signed control-plane path. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy, disputes, abuse controls, and verification evidence that later constrain identity and namespace mutations. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies accounting refs and usage-relevant event consumers without moving accounting authority into Overpass. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies SDK, CLI, admin UI, Docdex, Mcoda, Codali, and product-client identity-resolution use cases. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service workload, backup, restore, failover, rolling update, and break-glass operation requirements. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies universal namespace, route binding, Overmesh route resolution, Overasset namespace/storage bindings, transfer, delegation, anti-squatting, and dispute expansion. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies wallet, AI assistant, workspace, directory, search, messaging, social, maps, and mobile clients that consume identity and namespace rails. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies governance, threat modeling, privacy, compliance, incident, migration, and public-reporting hardening for identity and namespace authority. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #10 first build work aligned to master Phase 1, with broader namespace expansion in Phase 8 and later hardening phases. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first control-plane services, Axum/Tower/Hyper-style HTTP, signed command envelopes, Ed25519, BLAKE3 refs, canonical JSON plus JSON Schema, native Overwatch evidence, and no conventional cloud product-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 1 | Attach SDS #10 to the build-plan layer, freeze Overpass as the identity and namespace authority, and preserve resolved identity decisions. |
| 2 | Master Phases 0 and 1 | Build the Rust service skeleton, canonical identity/namespace schemas, local Overrid-shaped storage, and deterministic fixtures. |
| 3 | Master Phase 1 | Implement Phase 1 Overpass-lite identity creation, subject refs, lifecycle state, append-only history, and state transitions. |
| 4 | Master Phase 1 | Implement filtered identity resolution and integrate Overgate, Overtenant, Overkey, and Overregistry around stable subject refs. |
| 5 | Master Phase 8 | Expand into universal namespace records, route bindings, normalization, transfer/delegation, and anti-squatting controls. |
| 6 | Master Phases 4, 8, and 13 | Add verification markers, identity links, merge, recovery, and dispute refs with policy and governance handoff. |
| 7 | Master Phases 1, 5, and 7 | Emit Overwatch events, provide usage-relevant refs, expose observability, and prepare grid-resident operation. |
| 8 | Master Phases 6 and 12 | Harden SDK, CLI, admin UI, product, mobile, and native-app handoff contracts that consume Overpass refs. |
| 9 | Master Phases 7 and 13 | Harden privacy, security, migration, incident, stewardship, and scale behavior for identity and namespace authority. |
| 10 | Master Phase 1 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, contract tests, and phase-order consistency. |

## Tech Stack Guardrails

- Overpass core is a Rust service using Tokio and Axum/Tower/Hyper-style HTTP, with rustls/mTLS where early seed control-plane transport requires it.
- Identity records, namespace records, route bindings, verification markers, tombstones, dispute refs, API errors, events, and fixtures use canonical JSON plus JSON Schema from the shared schema package.
- Signed Overpass lifecycle commands flow through Overgate and use Ed25519 command envelopes, idempotency keys, trace ids, tenant ids, stable reason codes, schema versions, and append-only events.
- BLAKE3 is used for evidence refs, content hashes, canonical body hashes, route-binding evidence refs, fixture hashes, and hash-linked audit references where hashes are needed.
- Overpass persists state through Overrid-owned abstractions or Overrid-shaped local stubs during early phases. It must not make PostgreSQL, Redis, Kafka, NATS, S3, MinIO, Vault, cloud KMS, or similar products the platform boundary.
- Overpass stores stable identity and namespace refs. It must not store raw private keys, raw API keys, credentials, unrestricted private evidence, payment details, raw profile content, or secret material.
- Overpass does not own tenant authorization, credential verification, policy finality, usage accounting, wallet balances, Seal Ledger mutation, content moderation decisions, profile records, or native-app content.
- Names, handles, routes, and namespace labels are resolvable records, not primary identity keys and not speculative assets. Overpass must not implement blockchain, NFT mechanics, pricing tables, revenue projections, or customer-count assumptions.

## Phase 1: SDS Attachment, Authority, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #10.**
  - Design: Link this document from the numbered Overpass SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/control_plane/overpass.md`, `docs/service_catalog/control_plane/overpass.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #10 returns both the Overpass SDS and this sub-build plan.

- **1.2 Freeze Overpass as the identity and namespace authority.**
  - Design: Record that Overpass owns identity records, subject refs, lifecycle state, public and tenant-local namespace records, route-binding refs, verification markers, tombstones, identity links, merge refs, recovery refs, and dispute refs.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overpass owns stable identity and namespace refs but not credentials, tenant authorization, policy finality, accounting mutation, profile content, or private evidence custody.

- **1.3 Preserve master Phase 1 as the first build point.**
  - Design: Keep first implementation in master Phase 1 because Overpass-lite is required before Overtenant, Overkey, Overgate, Overregistry, and Overqueue can share stable actor, tenant, service-account, manifest-owner, and system-service refs.
  - Output: Phase-gate note that SDS #10 starts in Phase 1 and expands later through namespace, route, policy, metering, product, grid-resident, native-app, and governance gates.
  - Validation: Review proves this plan does not move Overpass into Phase 0 or change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve the SDS decisions for the Phase 1 identity set, classed namespace scope, no immediate reuse for tombstoned names, verification marker fields, and append-only merge/recovery behavior.
  - Output: Resolved-decision checklist tied to SDS #10 open-question answers.
  - Validation: Review proves the plan does not reopen settled decisions or replace them with username-primary identity, immediate name reuse, raw private evidence payloads, direct edits for merge/recovery, or speculative namespace ownership.

- **1.5 Define runtime authority boundaries.**
  - Design: Require Overpass mutating APIs to flow through Overgate for external and operator-facing commands while internal resolution helpers remain restricted to approved service accounts with explicit caller purpose.
  - Output: Boundary matrix for Overgate admission, Overtenant membership refs, Overkey credential refs, Overregistry manifest ownership, Overwatch audit refs, Overguard policy refs, Overasset ownership refs, and downstream read caches.
  - Validation: Design review rejects direct identity mutation by downstream services and rejects resolution paths that leak fields outside tenant, role, data-class, or policy scope.

## Phase 2: Rust Service Skeleton, Schemas, And Local Fixtures

### Work Items

- **2.1 Create the Overpass Rust service crate.**
  - Design: Add an Overpass service crate under the control-plane workspace using Tokio, Axum/Tower/Hyper-style HTTP, shared config loading, tracing setup, and dependency injection for storage, schema validation, Overgate, Overtenant, Overkey, Overregistry, Overwatch, and policy callbacks.
  - Output: Service crate, module layout, local-stack service entrypoint, and testable handler boundaries.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Overpass stays separate from Overkey, Overtenant, Overgate, Overregistry, Overwatch, Overguard, and accounting internals.

- **2.2 Define canonical identity schemas.**
  - Design: Add shared schemas for `identity_record`, `identity_subject_ref`, `membership_ref`, `identity_link`, lifecycle state, API errors, and Overwatch event payloads.
  - Output: JSON Schema files, Rust generated or hand-written types, compatibility fixtures, and stable reason-code enums.
  - Validation: Schema tests reject missing identity type, tenant scope where required, lifecycle state, created-by refs, updated-time refs, audit refs, and schema versions.

- **2.3 Define namespace and route schemas behind phase gates.**
  - Design: Model `namespace_record`, `route_binding`, `verification_marker`, `tombstone_record`, and `dispute_ref` early while keeping broad mutating namespace and route APIs gated until master Phase 8.
  - Output: Schema definitions, reserved enum values, phase-availability metadata, and fixture records for later expansion.
  - Validation: Schema tests prove Phase 1 can validate reserved namespace fields without exposing Phase 8 mutating surfaces too early.

- **2.4 Implement Overrid-owned storage boundaries.**
  - Design: Define append-friendly repositories for identity, namespace, route, tombstone, marker, link, and dispute records with local Overrid-shaped storage stubs for early phases.
  - Output: Repository traits, local storage adapter, migration hooks, and append-only history model.
  - Validation: Tests prove lifecycle updates append state transitions rather than erasing identity history and never persist credentials, raw secrets, raw private evidence, payment details, or unrestricted profile data.

- **2.5 Connect local development and integration harness fixtures.**
  - Design: Wire Overpass into the loopback-only local stack and integration harness with deterministic person, organization, node, app, native-service, service-account, system-service, tenant, manifest-owner, tombstone, and namespace fixtures.
  - Output: Local service config, fixture references, and harness scenario names.
  - Validation: Local smoke tests can create fixture identities and resolve subject refs through Overgate without production credentials or non-Overrid product dependencies.

## Phase 3: Identity Creation, Lifecycle, And State History

### Work Items

- **3.1 Implement Phase 1 identity creation APIs.**
  - Design: Support `POST /v1/identities` for person, organization, node, app, native-service, service-account, and system-service identities using signed commands, idempotency keys, tenant scope where applicable, and schema validation.
  - Output: Identity creation handler, identity-type validator, idempotent response behavior, and creation event payload.
  - Validation: API tests cover valid creation, duplicate idempotency keys, duplicate subject refs, missing tenant scope, invalid type, wrong actor, and schema-version denial.

- **3.2 Implement stable subject refs.**
  - Design: Generate or accept subject refs that remain stable across key rotation, tenant changes, app migration, namespace changes, route changes, and recovery flows.
  - Output: Subject-ref module, ref format fixtures, and lookup indexes.
  - Validation: Tests prove downstream records can store subject refs without depending on mutable display handles, route names, key ids, profile names, or tenant-local aliases.

- **3.3 Implement lifecycle state transitions.**
  - Design: Support `pending`, `active`, `disabled`, `suspended`, `merged`, and `tombstoned` with explicit command type, actor ref, reason code, audit refs, effective time, and transition rules.
  - Output: `POST /v1/identities/{identity_id}/state` handler and lifecycle state machine.
  - Validation: State tests reject invalid transitions, protected mutations by disabled/suspended/tombstoned identities, silent resurrection, direct state edits, and missing audit evidence.

- **3.4 Implement membership and owner reference mirroring.**
  - Design: Mirror only the Overtenant membership and owner refs needed for identity resolution while keeping tenant membership authority in Overtenant.
  - Output: `membership_ref` read model, dependency update handler, and stale-ref behavior.
  - Validation: Tenant tests prove Overpass cannot grant roles, change tenant membership, or override suspension state; it can only resolve stored refs and filtered identity fields.

- **3.5 Implement append-only identity history.**
  - Design: Preserve creation, state changes, links, tombstones, merge refs, display-handle reservations, and audit refs in append-only history so old evidence remains explainable.
  - Output: Identity history query model and retention rules.
  - Validation: History tests prove updates create new history entries, reads can reconstruct current state, and tombstoned or merged identities preserve replacement or no-reuse refs.

## Phase 4: Resolution APIs And Control-Plane Integrations

### Work Items

- **4.1 Implement filtered identity reads.**
  - Design: Implement `GET /v1/identities/{identity_id}` with caller-aware field filtering for public, tenant-local, operator, internal-service, and system-service contexts.
  - Output: Identity read handler, field-policy matrix, and redaction fixtures.
  - Validation: Tests prove callers see only allowed fields and cannot inspect cross-tenant private fields, hidden markers, raw evidence refs, or protected recovery data.

- **4.2 Implement identity resolution helpers.**
  - Design: Implement `GET /v1/identities:resolve` for subject refs, identity ids, minimal display handles, and later namespace or route refs according to caller permissions and phase availability.
  - Output: Resolution endpoint, query normalization, denial reason codes, and cache guidance.
  - Validation: Resolution tests cover active, pending, disabled, suspended, merged, tombstoned, wrong-tenant, missing, stale, and unauthorized refs.

- **4.3 Integrate Overgate, Overtenant, and Overkey checks.**
  - Design: Provide dependency contracts so Overgate can resolve actor state, Overtenant can resolve identity and membership refs, and Overkey can attach credential metadata to stable subjects.
  - Output: Client contracts, fixture calls, and denial mapping for dependency failures.
  - Validation: Integration tests prove Overgate blocks protected mutations for suspended identities, Overkey binds credentials to stable subject refs, and Overtenant remains membership authority.

- **4.4 Integrate Overregistry and manifest ownership.**
  - Design: Require resource manifests, workload manifests, package manifests, provider records, native app records, and system-service records to reference stable Overpass identities.
  - Output: Manifest-owner ref contract and Overregistry handoff checklist.
  - Validation: Manifest tests reject owner refs that are missing, tombstoned, suspended for protected actions, wrong tenant, or not allowed for the requested manifest class.

- **4.5 Define resolution cache and invalidation behavior.**
  - Design: Treat Overpass resolution caches as short-lived optimizations keyed by ref, tenant scope, caller purpose, field policy, and identity state epoch.
  - Output: Cache guidance in resolution results and invalidation events for state, merge, tombstone, namespace, and route changes.
  - Validation: Cache tests prove suspension, tombstone, merge, membership change, marker change, and route/namespace rebind events invalidate or bypass stale positive reads before protected mutations.

## Phase 5: Namespace Records, Route Bindings, And Phase 8 Expansion

### Work Items

- **5.1 Implement namespace scope classes.**
  - Design: Model global, tenant-local, app-local, and community-local namespace classes exactly as SDS #10 defines them, with authority boundary, allowed target class, transfer rules, and privacy defaults.
  - Output: Namespace-class schema, policy matrix, and phase-availability matrix.
  - Validation: Tests prove Phase 1 exposes only minimal display-handle resolution while Phase 8 enables broader namespace records behind explicit feature gates.

- **5.2 Implement name normalization and collision protection.**
  - Design: Normalize names consistently and defend against case collisions, reserved names, lookalike classes where supported, route-root conflicts, tenant-local collisions, and misleading public names.
  - Output: Normalization module, collision reason codes, golden fixtures, and migration hooks.
  - Validation: Normalization tests cover case folding, reserved words, ambiguous labels, route collisions, tenant-local duplicates, public-name conflicts, and Unicode lookalike handling where supported.

- **5.3 Implement namespace reservation, binding, and resolution.**
  - Design: Add `POST /v1/namespaces`, `GET /v1/namespaces:resolve`, and state transitions for `reserved`, `active`, `locked`, `transferring`, `released`, and `tombstoned`.
  - Output: Namespace handlers, state machine, reservation refs, target refs, and no-reuse metadata.
  - Validation: Namespace tests prove names resolve only when active, locked or disputed names cannot be rebound silently, and released names remain protected by classed no-reuse rules.

- **5.4 Implement route-binding records.**
  - Design: Bind routes to apps, native services, messaging handles, service endpoints, Overmesh routes, and approved asset or storage refs with explicit owner identity, tenant scope, route class, verification refs, and conflict refs.
  - Output: `route_binding` schema, `POST /v1/routes` handler, and Overmesh handoff contract.
  - Validation: Route tests reject missing owner authority, tombstoned targets, suspended actors, stale endpoints, unapproved service refs, route hijacks, and unaudited route target changes.

- **5.5 Implement anti-squatting, transfer, release, and no-reuse controls.**
  - Design: Apply ordinary tenant-local, public/global, native-app route, high-risk, abuse, and dispute-related no-reuse windows from SDS #10 without speculative sale or asset behavior.
  - Output: No-reuse policy table, transfer/release handlers, and tombstone reservation records.
  - Validation: Tests prove public/global names and native-app route names remain indefinitely reserved unless signed recovery or dispute outcome explicitly rebinds them, and abuse-related names remain permanently locked.

## Phase 6: Verification Markers, Identity Links, Recovery, And Disputes

### Work Items

- **6.1 Implement verification markers.**
  - Design: Allow central AI and operators to add marker type, issuer identity, target ref, evidence ref, decision class where needed, expiry or renewal rule, revocation path, and stable reason codes without exposing private evidence.
  - Output: `verification_marker` schema, marker handler, filtered marker read model, and Overwatch events.
  - Validation: Marker tests prove payloads use evidence refs, hashes, redacted summaries, and trace ids rather than private documents, secrets, payment details, credential material, or unrestricted profile data.

- **6.2 Implement identity links, delegation refs, and merge records.**
  - Design: Add `identity_link` records for verified relationships, delegated refs, recovery links, and merges while preserving source identity history and target replacement refs.
  - Output: `POST /v1/identities/{identity_id}/links` handler, link state machine, and merge read behavior.
  - Validation: Link tests prove merges are append-only, source identities move to `merged`, replacement refs are returned on reads, and direct identity edits are rejected.

- **6.3 Implement recovery and compromise flows.**
  - Design: Support lost-credential recovery when a valid protected credential remains, and break-glass/operator-assisted recovery with offline evidence refs when all credentials are lost.
  - Output: Recovery request schema, freeze behavior, dependency handoff to Overkey and Overgate, and recovery outcome records.
  - Validation: Recovery tests prove suspected compromise freezes identity, credentials, namespace bindings, and high-risk service-account actions until signed outcome records unblock them.

- **6.4 Implement identity and namespace disputes.**
  - Design: Add `dispute_ref` records for identity ownership, impersonation, namespace collision, route hijack, recovery challenge, abuse marker challenge, and transfer dispute.
  - Output: Dispute schema, dispute-opened event, state model, evidence refs, claimant refs, and current-state reads.
  - Validation: Dispute tests prove evidence refs and reason codes are preserved, locked records cannot be silently rebound, and policy outcomes are traceable without exposing private data dumps.

- **6.5 Add Overguard and governance handoff.**
  - Design: Route suspension, abuse, verification, dispute, namespace protection, high-risk transfer, and recovery policy questions to Overguard and later governance/stewardship systems without moving policy finality into Overpass.
  - Output: Policy-decision ref contract, governance handoff checklist, and fallback deny-by-default behavior.
  - Validation: Policy tests prove Overpass honors deny or lock decisions but does not become the policy engine, abuse classifier, central AI governance authority, or dispute adjudication authority.

## Phase 7: Events, Observability, Metering Refs, And Grid Operations

### Work Items

- **7.1 Emit Overwatch-compatible events.**
  - Design: Emit `overpass.identity_requested`, `overpass.identity_created`, `overpass.identity_state_changed`, `overpass.identity_suspended`, `overpass.identity_tombstoned`, `overpass.identity_merged`, `overpass.namespace_reserved`, `overpass.namespace_bound`, `overpass.route_bound`, `overpass.verification_marker_added`, and `overpass.dispute_opened`.
  - Output: Event builder, Overwatch client, event schemas, and event-to-state transition map.
  - Validation: Audit tests prove events include refs, hashes, trace ids, actor ids, tenant ids, target refs, and reason codes while excluding hidden profile data and raw private evidence.

- **7.2 Implement audit-safe metrics and traces.**
  - Design: Record identity counts by type/state, namespace counts by scope/state, route-binding health, marker counts by allowed class, suspension counts, tombstone counts, dispute counts, stale-target reports, and dependency failures.
  - Output: Rust tracing and OpenTelemetry-compatible metric hooks with Overwatch as authoritative audit evidence.
  - Validation: Metrics tests prove labels avoid private data, tenant leakage, raw evidence, secrets, raw profile content, and high-cardinality unbounded values.

- **7.3 Emit usage-relevant refs without accounting mutation.**
  - Design: Emit events for identity creation, namespace reservation, route binding, verification marker changes, dispute actions, and operator actions so Overmeter, ORU accounts, wallets, native services, and Seal Ledger can attribute usage externally.
  - Output: Usage-relevant event contract and Overmeter handoff notes.
  - Validation: Accounting-boundary tests prove Overpass does not mutate ORU balances, Seal Ledger entries, invoices, wallet accounts, provider payouts, grant pools, or Overasset rights.

- **7.4 Implement readiness, failure behavior, and dependency state.**
  - Design: Separate liveness from readiness for storage, shared schemas, Overgate, Overtenant, Overkey, Overregistry, Overwatch, Overguard, Overasset refs, and route/namespace dependencies.
  - Output: `GET /v1/healthz`, `GET /v1/readyz`, dependency matrix, degraded-state reason codes, and emergency-buffer rules.
  - Validation: Readiness tests fail closed when Overwatch is unavailable for lifecycle mutation and reject unsafe identity or namespace mutations when required authority dependencies are missing.

- **7.5 Prepare grid-resident operations behavior.**
  - Design: Define system-service workload needs for Overpass, including protected placement, backup, restore, failover, rolling update, rollback, maintenance mode, break-glass controls, and incident runbooks.
  - Output: Phase 7 operations checklist for Overpass.
  - Validation: Grid-readiness review confirms founder seed hardware can later be removed from the normal path without changing Overpass public or internal resolution contracts.

## Phase 8: SDK, CLI, Admin, Product, And Native-App Handoff

### Work Items

- **8.1 Harden SDK and CLI identity bindings.**
  - Design: Provide generated contract bindings and Rust-first SDK/CLI flows for identity creation, resolution, state transitions, namespace reservation, namespace resolution, route binding, marker reads, and dispute refs.
  - Output: SDK/CLI contract examples, stable JSON output shapes, and error/reason-code mappings.
  - Validation: SDK/CLI tests prove clients use generated contracts, pass idempotency and trace ids, decode stable reason codes, and never hardcode username-primary identity assumptions.

- **8.2 Implement admin and operator views.**
  - Design: Expose tenant-isolated views for identity state, suspension, disabled identities, tombstones, merged refs, namespace conflicts, route health, verification markers, disputes, and stale-target diagnostics.
  - Output: Admin read-model requirements and operator diagnostic endpoints or UI contract.
  - Validation: Admin tests prove authorized operators can diagnose identity and namespace issues while tenant users cannot see cross-tenant private metadata, hidden markers, or unrestricted evidence refs.

- **8.3 Define product and adapter identity handoff.**
  - Design: Document how Docdex, Mcoda, Codali, adapters, AI routing, Overrun workers, node agents, and product clients attach stable Overpass refs to jobs, repos, packages, models, tools, service accounts, and result artifacts.
  - Output: Product identity checklist and integration fixtures for Phase 6 and later consumers.
  - Validation: Product integration tests fail when clients bypass Overgate, store mutable handles as primary keys, omit subject refs, omit trace ids, skip idempotency, or use unaudited local identity shortcuts.

- **8.4 Define native-app identity and namespace handoff.**
  - Design: Provide handoff rules for wallet, personal AI assistant, workspace, directory listings, search, messaging, social, maps, mobile backend gateway, and mobile SDK.
  - Output: Native-app checklist for person/org/app/native-service identity refs, route refs, public handle refs, tenant-local aliases, permission-aware resolution, and dispute hooks.
  - Validation: Native-app tests prove apps use normal Overpass APIs, resolve display names through Overpass, respect tenant and app-local scopes, and do not store private profile data inside Overpass.

- **8.5 Define downstream migration and backfill paths.**
  - Design: Provide migration steps for moving placeholder usernames, manifest owners, route names, wallet refs, service-account refs, app owners, and system-service refs into stable Overpass subject refs.
  - Output: Migration checklist, compatibility fixtures, and rollback rules.
  - Validation: Migration tests prove old refs map to stable subject refs without identity takeover, name reuse, route hijack, or audit loss.

## Phase 9: Governance, Security, Privacy, And Scale Hardening

### Work Items

- **9.1 Run identity and namespace threat modeling.**
  - Design: Cover identity takeover, tenant escape through identity refs, service-account impersonation, username squatting, route hijack, marker abuse, recovery abuse, merge fraud, stale cache misuse, and supply-chain identity confusion.
  - Output: Threat model, mitigations, tests, accepted-risk entries, and owner refs.
  - Validation: Security review confirms every high-risk threat has a mitigation, monitoring rule, test, or explicit accepted-risk entry.

- **9.2 Harden privacy and data-class filtering.**
  - Design: Apply data-class filters to identity fields, markers, namespace records, route refs, recovery refs, dispute refs, operator views, native-app reads, mobile reads, search, messaging, and public APIs.
  - Output: Privacy matrix, redaction tests, and field-policy fixtures.
  - Validation: Privacy tests prove public, tenant, app-local, community-local, operator, and internal service callers cannot see fields outside their authority.

- **9.3 Formalize central AI and stewardship boundaries.**
  - Design: Limit central AI and operator verification marker use to evidence-backed refs, stable reason codes, expiration or revocation rules, and appeal/dispute paths.
  - Output: Central AI marker policy, stewardship reporting inputs, and appeal/dispute handoff rules.
  - Validation: Governance tests prove central AI cannot silently delete, transfer, tombstone, or rebind identities, names, or routes without approved policy evidence and auditable state transitions.

- **9.4 Add migration, compatibility, and PIP requirements.**
  - Design: Require protocol improvement proposals or migration records for identity type changes, namespace-class changes, route-binding behavior changes, marker class changes, no-reuse policy changes, and normalization changes.
  - Output: Compatibility matrix, migration tool hooks, and PIP trigger checklist.
  - Validation: Migration review proves schema and normalization changes preserve old evidence, audit replay, route compatibility, and no-reuse protections.

- **9.5 Define incident and recovery runbooks.**
  - Design: Document response for compromised identities, malicious rotations, wrong tombstones, false suspensions, route hijacks, namespace disputes, marker mistakes, Overwatch outages, stale resolution caches, and cross-tenant leakage.
  - Output: Operator runbook checklist tied to Overwatch events, Overpass admin views, affected-subject inventory, and recovery tasks.
  - Validation: Incident drills prove operators can trace each failure from request to identity record, namespace record, route binding, marker, dispute, audit ref, dependency state, and recovery outcome.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #10`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first control-plane implementation, Axum/Tower/Hyper-style HTTP, Ed25519, BLAKE3, signed command envelopes, canonical JSON plus JSON Schema, Overrid-owned storage boundaries, and native Overwatch evidence.
  - Output: Tech-stack alignment checklist for Overpass.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #10 is represented as a Phase 1 control-plane service with broader Phase 8 namespace expansion and later hardening through policy, metering, product, grid-resident, native app, and governance phases.
  - Output: Updated master-plan and crosswalk rows for SDS #10.
  - Validation: Review confirms only per-SDS sub-build indexing changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #10 and the Overpass service plan link back to this sub-build plan and preserve Overpass identity/namespace authority boundaries.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare identity, namespace, contract, and downstream handoff gates.**
  - Design: Require tests for identity creation, subject refs, lifecycle transitions, resolution, field filtering, membership refs, manifest-owner refs, namespace reservation, route binding, tombstones, no-reuse policy, verification markers, identity links, recovery, disputes, audit events, readiness, cache invalidation, and downstream consumers.
  - Output: Final validation checklist for Overpass implementation.
  - Validation: Handoff review confirms Overgate, Overtenant, Overkey, Overregistry, Overwatch, Overguard, Overmeter, ORU, Seal Ledger, Overasset, Overmesh, SDK, CLI, admin UI, adapters, native apps, mobile services, and grid-resident system services can depend on Overpass refs without moving their runtime authority into Overpass.

## Alignment Review

- The sub-build plan keeps Overpass first build work in master Phase 1, matching SDS #10, the service catalog entry, Phase 1 plan, master build plan, and build-plan crosswalk.
- The plan treats master Phase 0 as prerequisite work for schemas, local stack, fixtures, test harness, and shared API/event discipline, not as the Overpass implementation phase.
- The plan treats master Phase 8 as the broad namespace and route-binding expansion point, matching SDS #10 and the Phase 8 data/storage/namespace platform.
- The plan treats later phases as hardening or consumer gates: trust/policy in Phase 4, metering/accounting refs in Phase 5, product clients in Phase 6, grid-resident operations in Phase 7, native/mobile apps in Phase 12, and governance/compliance hardening in Phase 13.
- The plan carries forward SDS #10 resolved decisions for Phase 1 identity types, reserved later enum values, classed namespace scopes, no immediate name reuse, evidence-ref verification markers, and append-only merge/recovery flows.
- The plan keeps Overpass narrow: no credential custody, no tenant authorization ownership, no policy finality, no accounting or Seal Ledger mutation, no content/profile ownership, no raw private evidence storage, no direct downstream private-record writes, and no conventional cloud product boundary assumptions.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #10 is complete when a builder can implement Overpass as the Phase 1 Rust control-plane identity authority with canonical identity schemas, Phase 1 identity creation for people, organizations, nodes, apps, native services, service accounts, and system services, stable subject refs, append-only lifecycle history, pending/active/disabled/suspended/merged/tombstoned transitions, filtered resolution APIs, Overgate/Overtenant/Overkey/Overregistry integrations, cache guidance, Phase 8 namespace and route-binding expansion, normalization and no-reuse controls, verification markers, identity links, recovery/dispute refs, Overwatch-compatible events, usage-relevant refs without accounting mutation, audit-safe observability, readiness/degraded-state behavior, SDK/CLI/admin/product/native/mobile handoff rules, governance/security hardening, and documentation links that preserve the master Phase 0 through Phase 13 order.
