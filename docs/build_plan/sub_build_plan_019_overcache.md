# SUB BUILD PLAN #19 - Overcache

Attached SDS: [docs/sds/execution_scheduling/overcache.md](../sds/execution_scheduling/overcache.md)

## Purpose

This sub-build plan turns SDS #19 into an implementation sequence for Overcache. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overcache is the policy-scoped reuse layer for cache metadata, provenance, grants, invalidation, warming, eviction, and usage facts. It reduces repeated compute, storage, model, package, dataset, index, static-asset, API-response, and runtime-snapshot work without becoming canonical storage or a shortcut around tenant, policy, privacy, rights, or trust boundaries. The first useful implementation is metadata-first trust-scope work in master Phase 4; broad byte-backed reuse waits for Overstore, Overvault, namespace, and data-class controls in master Phase 8 or later.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #19: Overcache](../sds/execution_scheduling/overcache.md) | Controls Overcache purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, policy rules, metering, operations, failure modes, validation, build breakdown, and resolved open-question decisions. |
| [Overcache service plan](../service_catalog/execution_scheduling/overcache.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, contract validation, fixture discipline, signed envelopes, local-stack stubs, and integration harnesses. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies first Overpack, Oversched, Overrun, Overlease, and Overmeter consumers that need cache hints and artifact refs without cache-policy bypass. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Controls Overcache's first build point: cache trust scopes, replayable policy decisions, reuse denial, and metadata-first private/trusted reuse classes. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes cache hit, miss, storage, egress, warming, eviction, and saved-work usage facts through Overmeter and accounting rollups. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overstore, Overvault, data-class, namespace, route, rights, retention, object integrity, and storage refs for broad byte-backed reuse. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies known-organization and grant-scoped reuse gates for federation-grant cache classes. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public low-sensitivity provider/workload limits, anti-abuse controls, challenge evidence, and public reuse guardrails. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies retention, incident, compliance, threat-model, migration, and reporting hardening for cache evidence and reuse decisions. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #19 first useful build work aligned to master Phase 4, with later handoffs through usage accountability, Overstore/Overvault storage, federation/public reuse, native clients, and governance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first services, Tokio, Axum/Tower/Hyper-style HTTP, canonical JSON plus JSON Schema, signed command envelopes, Ed25519 signatures, BLAKE3/content hashes, Overrid-native storage/queue/service boundaries, and TypeScript only for generated web/client/admin surfaces. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 3, 4, and 8 | Attach SDS #19, freeze Overcache scope, and preserve Phase 4 as the first useful build point while recognizing Phase 3 consumers and Phase 8 storage expansion. |
| 2 | Master Phases 0 and 4 | Define Rust service, schemas, APIs, fixtures, state machine, and local metadata stores for policy-scoped cache records. |
| 3 | Master Phase 4 | Build private-tenant and trusted-private-swarm metadata-first lookup, reservation, commit, and grant paths. |
| 4 | Master Phase 4 | Integrate Overguard policy decisions, redacted cache keys, provenance, scope denial, and Overwatch evidence. |
| 5 | Master Phases 4 and 8 | Implement invalidation, quarantine, stale/grace rules, eviction, retention, and object-integrity gates. |
| 6 | Master Phases 3, 4, and 5 | Add Overrun/Overpack/Oversched consumers, node-local pressure reports, cache-locality hints, and Overmeter usage facts. |
| 7 | Master Phase 8 | Expand to Overstore-backed refs, Overvault encrypted refs, namespace/rights metadata, and data-class-aware retention. |
| 8 | Master Phases 6, 10, 11, and 12 | Add product, AI/RAG, deployment, federation-grant, public low-sensitivity, and native-app reuse handoffs. |
| 9 | Master Phases 7 and 13 | Prepare grid-resident operations, maintenance jobs, evidence retention, incident response, compliance reporting, and migration behavior. |
| 10 | Master Phase 4 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, and implementation gates. |

## Tech Stack Guardrails

- Overcache core is a Rust service module using shared contract types, Tokio where async service calls are required, and Axum/Tower/Hyper-style HTTP for cache APIs.
- Cache keys, cache entries, provenance records, access grants, invalidations, warming jobs, eviction records, usage events, API errors, and fixtures use canonical JSON plus JSON Schema. Compact Protobuf contracts may be added only where the shared contract layer requires them.
- Mutating APIs require signed command or service-account envelopes, idempotency keys, trace ids, tenant/provider/app scope, actor/service refs, stable reason codes, schema versions, policy/evidence refs, and append-only Overwatch events.
- Cache keys, package/artifact refs, model refs, dataset refs, context-set refs, stored objects, diagnostics, integrity checks, and provenance links use BLAKE3/content hashes. Ed25519 is used where actor, service, node, bundle, or policy signatures are required.
- Overcache stores metadata, lifecycle state, policy refs, object refs, grants, invalidations, and usage facts. Overstore owns durable object bytes after Phase 8; Overvault owns restricted encrypted refs; node-local stores are bounded and authorized by Overcache decisions.
- Phase 4 implementation is metadata-first: private-tenant lookup metadata, trusted-private-swarm package/runtime compatibility metadata, policy-decision and validation-result refs, cache-locality hints, and bounded same-scope node-local package/artifact refs.
- Phase 4 must not enable production cross-tenant reuse, federation-grant byte reuse, public low-sensitivity byte reuse, broad model-output reuse, regulated caching, secret-bearing caching, or uncontrolled dataset reuse.
- Model-output cache keys must hash redacted envelopes. They must never contain raw prompts, retrieved chunks, private documents, generated outputs, object bytes, private object ids, exact tenant content descriptions, or secrets.
- Node-local cache pressure reports are signed aggregate measurements by storage class, trust-scope bucket, data-class bucket, retention class, age bucket, and coarse artifact class; they are not content inventories.
- PostgreSQL, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, blockchain, NFT, pricing, revenue, customer-count, or per-operation external payment mechanics must not become Overcache's product boundary.

## Phase 1: SDS Attachment, Cache Scope, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #19.**
  - Design: Link this document from the numbered Overcache SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/execution_scheduling/overcache.md`, `docs/service_catalog/execution_scheduling/overcache.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #19 returns both the Overcache SDS and this sub-build plan.

- **1.2 Freeze Overcache as policy-scoped reuse metadata and access control.**
  - Design: Record that Overcache owns cache keys, metadata, trust scopes, lifecycle states, access grants, invalidation, quarantine, warming, eviction, provenance refs, and usage facts, but not canonical bytes or canonical data ownership.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overcache does not own Overstore bytes, Overbase records, Overvault secrets, Overasset rights, scheduler placement authority, policy finality, billing, payouts, or public-provider admission.

- **1.3 Preserve master Phase 4 as the first useful build point.**
  - Design: Keep Overcache's first implementation aligned to master Phase 4 because cache reuse must be policy-scoped and replayable before it can influence multi-tenant execution.
  - Output: Phase-gate note that master Phase 0 and Phase 3 supply prerequisites, Phase 4 starts metadata-first trust scopes, Phase 5 consumes usage facts, and Phase 8 unlocks broader byte-backed reuse.
  - Validation: Review proves this plan does not move broad cache storage into Phase 3 or Phase 4 and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve decisions for Phase 4 safe cache classes, redacted model-output keys, public low-sensitivity provenance, immediate blocking versus grace staleness, and signed aggregate node-local pressure reporting.
  - Output: Resolved-decision checklist tied to SDS #19 open-question answers.
  - Validation: Review rejects raw prompt keys, broad public/federation byte reuse before Phase 8, unverified public provenance, stale reads after safety or trust failures, and node-local content inventories.

- **1.5 Define runtime authority boundaries.**
  - Design: Create a boundary matrix for Overguard, Overwatch, Overstore, Overvault, Overrun, Overpack, Oversched, Overmeter, Overtenant, Overkey, Overmesh, Overasset, Overclaim, Oververify, SDK, CLI, admin UI, AI/RAG adapters, deployment tooling, native apps, and public-provider services.
  - Output: Boundary matrix listing read/write authority, evidence refs, policy refs, redaction level, storage owner, usage owner, and ownership exclusions.
  - Validation: Design review rejects direct storage-byte ownership, direct ORU mutation, policy bypass, cross-tenant file-path exposure, scheduler-forced cache hits, and hidden rights transfer through cache entries.

## Phase 2: Rust Service, Schemas, APIs, Fixtures, And State Machine

### Work Items

- **2.1 Create the Overcache Rust service module.**
  - Design: Add a Rust service module with cache metadata repositories, policy clients, object-ref clients, Overmeter event clients, Overwatch event clients, idempotency handling, trace propagation, and stable reason-code mapping.
  - Output: Service crate or module skeleton, API boundary, repository interfaces, client interfaces, error types, and integration-test hooks.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Overcache remains separate from Overstore, Overvault, Overguard, Overmeter, Overrun, and Oversched.

- **2.2 Define cache contract schemas.**
  - Design: Add schemas for `cache_scope`, `cache_key`, `cache_entry`, `cache_provenance`, `cache_access_grant`, `cache_invalidation`, `cache_warming_job`, `cache_eviction_record`, `cache_usage_event`, API errors, and lifecycle events.
  - Output: JSON Schema files, Rust types, fixtures, lifecycle enums, reason-code enums, redaction metadata, schema-version rules, and compatibility rules.
  - Validation: Schema tests reject missing tenant/scope, workload class, data class, package/model/schema refs, input hash refs, policy refs, audit refs, provenance refs, state, expiry, retention class, and redaction class.

- **2.3 Implement cache state machine foundations.**
  - Design: Model `candidate`, `reserved`, `warming`, `available`, `stale`, `quarantined`, `evicting`, `evicted`, and `purged` states with legal transitions, idempotent producer attempts, and terminal evidence.
  - Output: State transition engine, illegal-transition reasons, append-only transition records, and fixture states.
  - Validation: State tests reject silent scope promotion, invalid commit from stale/quarantined state, purge-before-eviction, duplicate producer conflicts, and metadata deletion without retention authority.

- **2.4 Define cache API surface.**
  - Design: Implement or specify `POST /cache/lookups`, `POST /cache/reservations`, `POST /cache/entries/{entry_id}/commit`, `GET /cache/entries/{entry_id}`, `POST /cache/access-grants`, `POST /cache/invalidations`, `POST /cache/warming-jobs`, `POST /cache/entries/{entry_id}/quarantine`, and `POST /cache/entries/{entry_id}/evictions/run`.
  - Output: API contracts, request/response schemas, idempotency behavior, authorization filters, pagination/read filters, and Overwatch event payloads.
  - Validation: API tests cover valid calls, duplicate idempotency keys, wrong tenant/scope, stale policy refs, missing trace id, missing data class, invalid object refs, unauthorized provenance reads, and denied broader reuse.

- **2.5 Build deterministic fixtures and local metadata stubs.**
  - Design: Provide fixtures for lookup hit, lookup miss, policy denial, reservation conflict, commit conflict, invalidation, quarantine, eviction, warming, node-local pressure, and model-output redacted envelope hashing.
  - Output: Valid/invalid fixtures, local metadata store abstraction, Overguard stub, Overwatch stub, Overmeter stub, and object-ref stub.
  - Validation: Local harness scenarios prove deterministic results, stable reason codes, no raw private payloads in fixtures, and no dependency on conventional database, queue, or object-store product boundaries.

## Phase 3: Metadata-First Private And Trusted-Swarm Reuse

### Work Items

- **3.1 Implement private-tenant cache lookup.**
  - Design: Allow a tenant-scoped workload to request reuse only when tenant, workload class, data class, policy refs, package/model/schema refs, input hashes, and provenance match the approved private boundary.
  - Output: Lookup path, private-tenant key derivation, denial reasons, bounded access grant, and hit/miss usage dimensions.
  - Validation: Tenant isolation tests prove another tenant, actor, workload, or app cannot read private cache metadata or object refs without explicit authority.

- **3.2 Implement trusted-private-swarm metadata classes.**
  - Design: Support Phase 4 trusted-private-swarm package/runtime compatibility metadata, validation-result refs, policy-decision refs, cache-locality hints, and bounded node-local package/artifact refs.
  - Output: Trusted-swarm scope rules, eligibility checks, compatible metadata classes, and blocked broad-reuse classes.
  - Validation: Tests prove Phase 4 rejects broad model-output reuse, public dataset reuse, federation-grant reuse, regulated caching, secret-bearing caching, cross-tenant reuse, and production byte reuse.

- **3.3 Implement reservations and idempotent commits.**
  - Design: Reserve canonical keys before producing or warming content, then commit object refs, checksums, provenance, and policy refs only after output validation.
  - Output: Reservation records, producer attempt refs, commit flow, conflict behavior, and available-state transition.
  - Validation: Commit tests prove first valid commit wins unless idempotency proves the same producer attempt, and conflicting producers do not overwrite content or provenance.

- **3.4 Implement access grants.**
  - Design: Return short-lived actor/workload-bound grants with allowed operations, expiry, policy refs, provenance summary, object refs, and redaction class.
  - Output: Grant schema, grant repository, read filters, expiry behavior, and denial reason mapping.
  - Validation: Grant tests reject expired grants, wrong actor, wrong workload, wrong scope, missing policy refs, stale policy refs, and attempts to use grants as ownership or rights records.

- **3.5 Implement scope-promotion denial.**
  - Design: Deny implicit movement from private tenant to trusted swarm, federation grant, or public low-sensitivity, and require a new policy-approved entry or grant for broader scope.
  - Output: Scope compatibility matrix, denial events, reason codes, and operator explanations.
  - Validation: Scope tests prove broader reuse requires explicit Overguard approval and evidence; entries never silently move to a broader cache scope.

## Phase 4: Policy, Provenance, Redacted Keys, And Evidence

### Work Items

- **4.1 Integrate Overguard cache policy decisions.**
  - Design: Ask or apply Overguard policy for lookup, grant, commit, invalidation, warming, and scope transition decisions using data class, workload class, cache scope, tenant, package/model/schema refs, rights refs, and trust evidence.
  - Output: Policy client, policy snapshot refs, decision cache rules, replay metadata, and `overcache.reuse_denied` events.
  - Validation: Policy tests prove decisions are replayable from facts and policy version and that policy-engine unavailability denies broader reuse.

- **4.2 Implement redacted model-output cache keys.**
  - Design: Hash canonical redacted envelopes containing tenant, actor/service-account scope, workload/app id, cache scope, data class, prompt-template/version refs, model/tool/runtime/schema refs, retrieval bundle refs, BLAKE3 context-set hashes, policy-decision refs, and output schema refs.
  - Output: Redacted-envelope schema, key derivation library, fixture vectors, and forbidden-field checks.
  - Validation: Tests prove raw prompts, retrieved chunks, private documents, generated outputs, object bytes, private object ids, tenant content descriptions, and secrets never enter cache keys or logs.

- **4.3 Implement provenance records.**
  - Design: Link cached material to producing workload, package id/version, model/tool id/version, input refs and BLAKE3 hashes, environment refs, output schema, policy decision refs, validation/checksum refs, rights/retention refs, and evidence refs.
  - Output: Provenance schema, provenance write path, redacted read model, and unknown-provenance denial rule.
  - Validation: Provenance tests default unknown, mixed-scope, private, or unverified provenance to deny rather than public or broader reuse.

- **4.4 Emit Overwatch evidence.**
  - Design: Emit append-only events for lookup hit, lookup miss, reuse denial, reservation, commit, invalidation, quarantine, eviction, scope-promotion denial, usage emission, and restricted evidence access.
  - Output: Event payload contracts, severity mapping, evidence refs, restricted-event read policy, and local pending-event spool.
  - Validation: Audit tests prove every state-changing command has trace id, actor/service ref, tenant/scope, idempotency key where applicable, reason code, policy refs, and no raw cached content.

- **4.5 Implement restricted provenance read rules.**
  - Design: Redact provenance details that reveal private tenant information while allowing authorized operators to inspect enough evidence for incidents, disputes, and replay.
  - Output: Redaction profiles, authorized read model, restricted evidence API contract, and denial reason mapping.
  - Validation: Read tests prove tenants cannot infer other tenants' prompts, chunks, object ids, node paths, producers, or private package/model details from cache metadata.

## Phase 5: Invalidation, Quarantine, Staleness, Eviction, And Retention

### Work Items

- **5.1 Implement immediate read-block invalidation.**
  - Design: Immediately block reads for tenant suspension, role/key revocation, Overguard emergency block, policy revocation, data-class privacy upgrade, package/model/schema security revocation, input/object mismatch, corrupted object refs, quarantine, malware suspicion, trust disqualification, rights/grant revocation, and serious Overclaim disputes.
  - Output: Invalidation rules, block reasons, propagation state, and `overcache.entry_invalidated` events.
  - Validation: Invalidation replay tests prove blocked entries cannot return cache hits even when metadata exists.

- **5.2 Implement grace-period stale handling.**
  - Design: Allow grace only for low-risk freshness changes: normal TTL expiry, newer non-breaking package/model versions, soft benchmark age, storage pressure, planned retention migration, or policy-approved public static-content refresh.
  - Output: Grace policy schema, stale state, freshness requirements, and no-promotion constraints.
  - Validation: Grace tests prove stale entries cannot satisfy workloads requiring fresh policy/package/model/input evidence and cannot be promoted to broader scopes.

- **5.3 Implement quarantine workflow.**
  - Design: Isolate entries for suspected poisoning, malware, impossible provenance, checksum mismatch, disputed output, incident refs, or public-provider trust failure.
  - Output: Quarantine command, provenance-graph matching, incident refs, release criteria, and operator read model.
  - Validation: Quarantine tests prove quarantined entries are not reusable, warming jobs skip them, and release requires policy/evidence authority.

- **5.4 Implement eviction and retention controls.**
  - Design: Evict under storage pressure while preserving metadata/audit summaries and respecting data class, scope, retention class, evidence requirements, and pending disputes.
  - Output: Eviction planner, dry-run report, pressure rules, freed-byte record, evicted/purged behavior, and retention policy records.
  - Validation: Eviction tests prove metadata is not purged before bytes, retained evidence remains explainable, and restricted or disputed records cannot be destroyed by generic pressure cleanup.

- **5.5 Implement integrity and missing-object recovery.**
  - Design: Detect metadata hit with missing object, checksum mismatch, stale object ref, or Overstore/Overvault access failure; mark entry stale or quarantine and return miss.
  - Output: Integrity check, recovery state, repair hooks, object-ref health event, and operator remediation.
  - Validation: Recovery tests prove missing objects do not return hits, checksum mismatches quarantine, and retry/rebuild paths preserve evidence.

## Phase 6: Execution Consumers, Scheduler Hints, Node-Local Pressure, And Usage Facts

### Work Items

- **6.1 Integrate Overrun and Overpack producers.**
  - Design: Let Overrun and Overpack reserve and commit package layers, input/output artifacts, runtime snapshots, validation refs, and execution artifacts through policy-approved cache records.
  - Output: Producer client contracts, artifact class mapping, reservation behavior, commit fixtures, and result-state integration.
  - Validation: Integration tests prove execution can use cache refs without bypassing package verification, lease limits, output validation, or policy decisions.

- **6.2 Integrate Oversched cache-locality hints.**
  - Design: Expose scheduling-safe locality hints, hit-likelihood summaries, and pressure buckets without allowing Oversched to force a cache hit when Overguard denies reuse.
  - Output: Scheduler hint schema, cache-locality read API, staleness markers, and denial semantics.
  - Validation: Scheduler tests prove cache hints affect placement scoring only after policy eligibility and never override tenant, data-class, trust, or freshness requirements.

- **6.3 Implement signed node-local pressure reports.**
  - Design: Accept aggregate Overcell/Overcache pressure reports for total bytes, reserved bytes, reclaimable bytes, high-water marks, eviction backlog, storage class, trust-scope bucket, data-class bucket, retention class, age bucket, and coarse artifact class.
  - Output: Pressure report schema, signature checks, aggregation path, redacted tenant view, and operator view.
  - Validation: Pressure tests reject raw paths, filenames, prompts, object bytes, private object ids, exact tenant descriptions, per-entry metadata outside authorized views, and unsigned reports.

- **6.4 Emit Overmeter usage facts.**
  - Design: Emit hit, miss, write, read, egress, storage-byte, retention, invalidation, warming, eviction, and saved-upstream-work dimensions without encoding prices or external payment calls.
  - Output: Usage event contract, attribution fields, Overmeter client, and saved-work rollup hints.
  - Validation: Metering tests prove cache savings are visible by tenant/workload/app/service/scope/producer/locality while billing, ORU mutation, settlement, and payout logic remain outside Overcache.

- **6.5 Add workload-result replay fixtures.**
  - Design: Build fixtures showing workload result with hit, miss, stale, denied, invalidated, quarantined, and object-missing cache outcomes.
  - Output: End-to-end fixture set for private execution, policy replay, metering, audit, and result-state reporting.
  - Validation: Replay tests prove successful, denied, stale, invalidated, quarantined, and failed cache paths are explainable from stored facts and events.

## Phase 7: Overstore, Overvault, Namespace, Rights, And Phase 8 Expansion

### Work Items

- **7.1 Integrate Overstore-backed object refs.**
  - Design: After Phase 8 storage primitives exist, store object bytes in Overstore and keep Overcache focused on metadata, policy, grants, lifecycle, integrity refs, and usage facts.
  - Output: Overstore ref schema, checksum validation, retention refs, repair refs, and content-addressed object integration.
  - Validation: Object tests prove Overcache cannot read or mutate bytes outside authorized Overstore refs and treats missing or corrupted objects as misses or quarantines.

- **7.2 Integrate Overvault restricted encrypted refs.**
  - Design: Support later restricted encrypted-cache classes only through Overvault and Overstore refs when policy explicitly permits encrypted private/regulated/secret-bearing reuse.
  - Output: Encrypted ref schema, restricted policy flags, key-policy refs, deny-by-default behavior, and audit refs.
  - Validation: Restricted-cache tests prove private, regulated, and secret-bearing material is denied unless a later policy explicitly allows encrypted refs with Overvault authority.

- **7.3 Integrate namespace and rights metadata.**
  - Design: Use namespace, purpose tags, license/rights refs, Overasset refs, grant refs, and route refs to decide whether object/model/dataset/package reuse is allowed.
  - Output: Rights and namespace compatibility matrix, public-eligibility checks, and redacted provenance reads.
  - Validation: Rights tests reject expired grants, revoked rights, disputed ownership, misleading namespace routes, unknown purpose tags, and public reuse without explicit eligible refs.

- **7.4 Implement Phase 8 data-class-aware retention.**
  - Design: Apply data class, storage class, trust scope, retention class, tenant/app policy, incident refs, dispute refs, and governance refs to retention and purge behavior.
  - Output: Retention engine, retention dry-run report, purge authority checks, and migration records.
  - Validation: Retention tests prove private, regulated, secret-bearing, disputed, incident-linked, and governance-linked entries cannot be purged by generic TTL.

- **7.5 Implement storage migration and backfill.**
  - Design: Migrate Phase 4 metadata-only and node-local refs into Phase 8 Overstore/Overvault-backed refs where policy permits, preserving hash/evidence continuity.
  - Output: Migration jobs, backfill records, replacement refs, old-ref tombstones, and replay compatibility.
  - Validation: Migration tests prove old entries remain explainable, old refs cannot be reused after replacement when invalidated, and evidence continuity survives schema/storage upgrades.

## Phase 8: Product, AI/RAG, Federation, Public, And Native-App Reuse

### Work Items

- **8.1 Integrate product and adapter clients.**
  - Design: Provide Overcache client paths for Docdex encrypted RAG, Mcoda/Codali jobs, AI Gateway Router, deployment tooling, SDK, CLI, admin UI, and runtime bridge consumers through normal Overgate/admin APIs.
  - Output: Product integration contracts, client examples, redaction requirements, and adapter fixtures.
  - Validation: Product tests prove clients consume cache decisions and refs without direct metadata-store access, raw private payload leakage, or policy bypass.

- **8.2 Support model-routing and RAG reuse safely.**
  - Design: Allow only eligible model/routing/RAG reuse classes using redacted envelope keys, retrieval bundle refs, model/tool/schema refs, context-set hashes, policy decision refs, and output schema refs.
  - Output: AI/RAG cache profile, model-output eligibility matrix, private-tenant defaults, and encrypted-ref placeholders.
  - Validation: AI/RAG tests prove raw prompts, raw retrieved chunks, private documents, generated outputs, and secrets never appear in keys, provenance logs, pressure reports, or public summaries.

- **8.3 Support federation-grant reuse.**
  - Design: Add federation-grant cache scope after known-organization, grant, purpose-tag, tenant, policy, rights, and evidence controls exist.
  - Output: Federation-grant scope rules, grant refs, purpose-tag refs, provider verification refs, and dispute behavior.
  - Validation: Federation tests prove grant-scoped reuse cannot become public reuse, cannot cross unauthorized tenants, and remains revocable and reportable.

- **8.4 Support public low-sensitivity reuse.**
  - Design: Permit public low-sensitivity reuse only when every input and output is eligible and provenance includes producing workload, package/model/tool refs, input refs and hashes, data-class refs, purpose-tag refs, verified producer/provider refs, Overguard allow refs, Overwatch evidence, validation/checksum evidence, rights/retention refs, and no active quarantine/dispute.
  - Output: Public low-sensitivity eligibility engine, evidence checklist, denial reasons, and public read model.
  - Validation: Public tests default unknown, private, mixed-scope, unverified, disputed, quarantined, or rights-revoked provenance to deny.

- **8.5 Support native-app and deployment reuse.**
  - Design: Add reuse profiles for package artifacts, static assets, public content, app deployment layers, media derivatives, search indexes, directory listings, and workspace artifacts with app/tenant/data-class policy.
  - Output: Native/deployment cache profiles, route/namespace refs, app ownership refs, and user-visible invalidation behavior.
  - Validation: Native-app tests prove user-private, tenant-private, regulated, and secret-bearing app data cannot leak into public or cross-tenant cache entries.

## Phase 9: Grid-Resident Operations, Maintenance, Incidents, And Governance

### Work Items

- **9.1 Prepare Overcache as a protected grid-resident service.**
  - Design: Define Overcache control-plane components as protected system-service workloads with placement policy, backup, restore, failover, rolling update, rollback, maintenance mode, and restricted operator actions.
  - Output: System-service manifest requirements, backup/restore fields, failover refs, and maintenance-state behavior.
  - Validation: Grid-readiness review confirms Overcache can move off founder hardware without changing cache keys, policy decisions, grants, invalidations, usage events, or evidence contracts.

- **9.2 Implement maintenance dry runs.**
  - Design: Provide dry-run invalidation, dry-run eviction, dry-run retention migration, dry-run quarantine by provenance graph, and dry-run warming plans.
  - Output: Maintenance command contracts, estimated impact, affected refs, policy reasons, and operator evidence.
  - Validation: Maintenance tests prove dry-run commands do not change cache state, bytes, grants, or evidence and produce enough detail for safe operator approval.

- **9.3 Implement incident-response hooks.**
  - Design: Let incident response quarantine by package/model/input/policy/evidence refs, freeze scope promotion, block reads, export redacted evidence, and notify downstream owners.
  - Output: Incident refs, quarantine-by-graph operation, restricted evidence export, notification events, and recovery states.
  - Validation: Incident tests prove suspected poisoned artifacts, malware, impossible provenance, public-provider fraud, rights disputes, and policy emergency blocks stop reads immediately.

- **9.4 Implement compliance and stewardship reporting.**
  - Design: Produce reports for cache reuse by scope, data class, saved-work class, invalidation latency, quarantine backlog, public low-sensitivity evidence, grant reuse, and retention/purge actions.
  - Output: Reporting schemas, export filters, privacy-preserving aggregate views, and governance evidence refs.
  - Validation: Reporting tests prove exports are explainable and auditable without leaking private prompts, chunks, object ids, tenant content, raw paths, or secret-bearing values.

- **9.5 Implement policy/version migration.**
  - Design: Support schema, policy, key-derivation, data-class, rights-ref, and retention-rule migration without silently making old entries eligible under new broader rules.
  - Output: Migration plan schema, compatibility checks, migration events, stale markers, and replay notes.
  - Validation: Migration tests prove old entries either remain valid under equivalent policy evidence or become stale/denied until rebuilt with new refs.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #19`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first service ownership, Tokio, Axum/Tower/Hyper-style HTTP, canonical JSON plus JSON Schema, signed envelopes, Ed25519, BLAKE3/content hashes, Overrid-native storage/queue/service boundaries, Overstore/Overvault ownership boundaries, and TypeScript only for generated/client/admin surfaces.
  - Output: Tech-stack alignment checklist for Overcache.
  - Validation: Scans find no PostgreSQL, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, blockchain, NFT, pricing, revenue, customer-count, or per-operation external payment assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #19 is represented as a Phase 4 trust/policy cache-scope service with Phase 5 usage-fact handoff and Phase 8 storage/namespace expansion.
  - Output: Updated master plan, Phase 5 usage-fact wording, and crosswalk row for SDS #19.
  - Validation: Review confirms only per-SDS sub-build indexing and explicit Overcache usage-fact wording changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #19 and the Overcache service plan link back to this sub-build plan and preserve Overcache as policy-scoped reuse metadata and access control.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare Overcache implementation gates.**
  - Design: Require tests for cache scopes, keys, entries, provenance, grants, invalidation, warming, eviction, usage facts, APIs, policy replay, Overwatch evidence, Overmeter handoff, Overstore refs, Overvault refs, namespace/rights refs, public low-sensitivity provenance, node-local pressure reports, maintenance dry runs, incident hooks, retention, migration, and documentation links.
  - Output: Final validation checklist for Overcache implementation.
  - Validation: Handoff review confirms Overrun, Overpack, Oversched, Overguard, Overwatch, Overmeter, Overstore, Overvault, Overtenant, Overkey, Overmesh, Overasset, Overclaim, Oververify, SDK, CLI, admin UI, adapters, native apps, federation services, public-provider services, and governance services can depend on Overcache decisions without moving their runtime authority into Overcache.

## Alignment Review

- The sub-build plan keeps Overcache's first useful build work in master Phase 4, matching SDS #19, the service catalog entry, Phase 4 cache trust scopes, the master build plan, and the build-plan crosswalk.
- The plan treats master Phase 0 and Phase 3 as prerequisites for shared schemas, local harnesses, signed envelopes, private execution artifacts, scheduler hints, and raw usage events, not as broad Overcache byte-reuse phases.
- The plan treats master Phase 5 as a usage-fact consumer: Overcache emits hit/miss/storage/egress/warming/eviction/saved-work events, while Overmeter, ORU, Seal Ledger, and Overbill own accounting and settlement.
- The plan treats master Phase 8 as the point where Overstore, Overvault, data classes, namespace, rights refs, and retention policies make broader byte-backed reuse possible.
- The plan carries forward SDS #19 resolved decisions for metadata-first Phase 4 classes, redacted model-output cache keys, strict public low-sensitivity provenance, immediate safety/privacy/policy/trust/rights/integrity blocking, grace staleness only for low-risk freshness, and signed aggregate node-local pressure reporting.
- The plan keeps Overcache narrow: no canonical durable storage, no secrets/product-boundary ownership, no scheduler placement authority, no policy finality, no ORU/Seal Ledger mutation, no billing/payout ownership, no public-provider admission, no uncontrolled file paths, and no cross-tenant reuse without explicit policy evidence.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #19 is complete when a builder can implement Overcache as the Phase 4 Rust policy-scoped reuse metadata service with cache key, entry, provenance, grant, invalidation, warming, eviction, usage, and event schemas; metadata-first private-tenant and trusted-private-swarm lookup/commit/grant paths; explicit Overguard decisions; redacted model-output key derivation; append-only Overwatch evidence; immediate invalidation and quarantine blocking; grace staleness only for low-risk freshness; signed aggregate node-local pressure reports; Overmeter hit/miss/storage/egress/saved-work usage facts; Phase 8 Overstore/Overvault/namespace/rights expansion; federation-grant and public low-sensitivity gates; product, AI/RAG, deployment, native-app, grid-resident, incident, governance, retention, and migration handoffs; implementation validation gates; and documentation links that preserve the master Phase 0 through Phase 13 order.
