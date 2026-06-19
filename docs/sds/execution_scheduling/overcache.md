SDS #19

# Overcache SDS

## Purpose

Provide a policy-scoped reuse layer for artifacts, model outputs, dataset chunks, indexes, package layers, static assets, API responses, and runtime snapshots without turning cached data into an uncontrolled source of truth.

Overcache exists to reduce repeated compute, storage, and network waste. It is useful only if cache reuse is governed by tenant boundaries, workload sensitivity, provenance, expiration, invalidation, and trust scopes.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overcache.md](../../service_catalog/execution_scheduling/overcache.md) |
| Sub-build plan | [SUB BUILD PLAN #19 - Overcache](../../build_plan/sub_build_plan_019_overcache.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md), [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md), [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md), [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md) |

## Service Family

- Family: Execution and scheduling
- Owning layer: Policy-governed reuse and cache metadata
- Primary data scope: cache keys, cache entries, trust scopes, provenance, artifact refs, access grants, invalidations, warming jobs, eviction records, and cache usage events
- First build phase from service plan: cache trust scopes in [Phase 4](../../build_plan/phase_04_trust_policy_verification.md), usage-fact handoff in [Phase 5](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md), and broader reuse after [Phase 8](../../build_plan/phase_08_data_storage_namespace_platform.md)

## Problem Statement

Distributed infrastructure becomes wasteful if every node repeatedly downloads the same package layers, recomputes the same public model output, rebuilds the same index, or transfers the same dataset chunk. But naive caching is dangerous: a cached artifact can cross tenant boundaries, leak private data, bypass policy, reuse stale results, or hide untrusted provenance.

Overcache fixes this by making reuse explicit, content-addressed where possible, policy-bound, auditable, invalidatable, and metered.

## Goals

- Define cache keys and metadata that include tenant, trust scope, data class, workload class, package/model/version refs, input hashes, policy refs, and provenance.
- Implement private tenant and trusted swarm scopes first.
- Add federation grant and public low-sensitivity scopes only after policy, verification, and namespace controls mature.
- Prevent cached artifacts from crossing into broader trust scopes without explicit Overguard approval.
- Keep cached bytes in Overstore or node-local stores while Overcache owns metadata, policy, and lifecycle.
- Emit usage events for cache hits, misses, writes, storage, egress, invalidation, and saved upstream work.
- Support deterministic invalidation, quarantine, retention, and eviction.

## Non-Goals

- Do not become the canonical data store. Overstore, Overbase, and Overvault own durable data/storage contracts.
- Do not cache secrets or regulated data unless a later policy explicitly permits a restricted encrypted form.
- Do not bypass Overguard, Overtenant, Overkey, or Overwatch.
- Do not let scheduler hints force a cache hit when policy denies reuse.
- Do not use cache entries as ownership or asset rights. Overasset handles rights and bindings.
- Do not encode pricing, revenue, or per-transaction fee behavior in cache logic.

## Primary Actors And Clients

- Overrun and Overpack for workload package layers, input artifacts, output artifacts, and runtime snapshots.
- Oversched for cache-locality hints, never as a policy override.
- Overguard for cache trust-scope decisions and reuse approval.
- Overstore for object refs and byte storage.
- Overmeter for cache usage facts and efficiency rollups.
- Encrypted Docdex RAG adapter, AI Gateway Router, native apps, and deployment tooling for high-value reuse.
- Operators inspecting cache pressure, invalidation, quarantine, and cross-scope denials.

## Dependencies

- [Overguard](../trust_policy_verification/overguard.md) for cache policy, data class, trust scope, and reuse decisions.
- [Overwatch](../control_plane/overwatch.md) for audit, invalidation, quarantine, and access events.
- [Overstore](../data_storage_namespace/overstore.md) for object references, retention, and byte storage after Phase 8.
- [Overrun](overrun.md) and [Overpack](overpack.md) for artifact production and consumption.
- [Overmeter](overmeter.md) for cache hit/miss, storage, egress, and saved-work usage events.
- [Overtenant](../control_plane/overtenant.md) for tenant and private-swarm boundaries.
- [Overkey](../control_plane/overkey.md) and [Overvault](../data_storage_namespace/overvault.md) for encrypted refs where restricted cache material becomes allowed.

Phase 4 should define trust scopes and metadata contracts before broad data caching exists.

## Owned Responsibilities

Overcache owns:

- Cache key schema, metadata schema, trust-scope enum, and compatibility rules.
- Cache entry lifecycle from candidate/reserved through available, stale, quarantined, evicted, or purged.
- Cache lookup, reserve, commit, access-grant, invalidate, warm, and purge contracts.
- Provenance checks linking cached material to producing workload, package/model/schema versions, input hashes, policy decisions, and evidence refs.
- Scope-transition checks and denials when a caller tries to reuse broader or incompatible cached material.
- Eviction and retention policy execution with auditable reason codes.

Overcache should not store large bytes directly when Overstore or node-local content stores are available. It stores metadata and controlled refs.

## Data Model

The first implementation should define:

- `cache_scope`: enum and policy object for `private_tenant`, `trusted_swarm`, `federation_grant`, and `public_low_sensitivity`.
- `cache_key`: canonical hash over tenant/scope, workload class, data class, package/model/schema refs, input refs or hashes, runtime compatibility, policy version, and producer identity.
- `cache_entry`: entry id, key hash, state, scope, owner tenant, producer workload, object refs, size, checksum, content type, created/last-hit timestamps, expiry, retention class, and audit refs.
- `cache_provenance`: producing workload id, package id/version, model id/version, input hashes, environment refs, result schema, policy decision refs, and evidence refs.
- `cache_access_grant`: caller, tenant, workload id, decision, allowed operations, expiry, policy refs, and denial reason if rejected.
- `cache_invalidation`: entry ids, invalidation reason, invalidating actor/service, matching rule, replacement refs, and propagation state.
- `cache_warming_job`: requested key set, target node/scope, priority, state, object refs, budget guard, and outcome.
- `cache_eviction_record`: entry id, eviction reason, pressure metrics, retention class, freed bytes, and audit refs.
- `cache_usage_event`: hit/miss/write/read/egress/storage/saved-work dimensions for Overmeter.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

The API should make policy decisions explicit:

- `POST /cache/lookups`: ask whether a caller may reuse an entry for a canonical cache key and workload context.
- `POST /cache/reservations`: reserve a cache key before producing or warming content.
- `POST /cache/entries/{entry_id}/commit`: commit object refs, checksum, provenance, and policy refs after production.
- `GET /cache/entries/{entry_id}`: read authorized metadata and access status.
- `POST /cache/access-grants`: create a bounded read/write grant after Overguard approval.
- `POST /cache/invalidations`: invalidate entries by key, provenance, version, policy, tenant, or incident ref.
- `POST /cache/warming-jobs`: request policy-approved cache warming near selected nodes or scopes.
- `POST /cache/entries/{entry_id}/quarantine`: isolate suspicious cached material pending review.
- `POST /cache/evictions/run`: execute bounded eviction under retention policy.

API requirements:

- Every lookup includes tenant, actor, workload class, data class, requested scope, package/model/schema refs, and policy context.
- Cache hit responses must include the decision, scope, object refs, expiry, provenance summary, and metering dimensions.
- Store/commit operations must be idempotent by canonical key and producer attempt.
- Access to broader scopes requires explicit policy approval; implicit promotion is forbidden.
- Reads must redact provenance details that reveal private tenant information.

## Event Surface

- `overcache.lookup_hit`: authorized hit returned.
- `overcache.lookup_miss`: no eligible entry found.
- `overcache.reuse_denied`: matching entry existed but policy denied reuse.
- `overcache.entry_reserved`: key reserved for production or warming.
- `overcache.entry_committed`: entry made available with provenance and object refs.
- `overcache.entry_invalidated`: entry made stale or unavailable by rule or actor.
- `overcache.entry_quarantined`: suspicious entry isolated.
- `overcache.entry_evicted`: entry removed under retention/pressure policy.
- `overcache.scope_promotion_denied`: attempted broader reuse rejected.
- `overcache.usage_emitted`: cache usage facts emitted for Overmeter.

Events should use object refs and hashes rather than embedding cached content.

## Core Workflow

1. A workload or service computes a canonical cache key from inputs, versions, scope, and policy context.
2. Caller asks Overcache for lookup with tenant, workload, and data-class context.
3. Overcache asks or applies Overguard policy to determine eligible scope and reuse rights.
4. If an eligible entry exists, Overcache returns a bounded access grant and emits hit usage.
5. If no entry exists, a producer may reserve the key and run the workload.
6. Producer commits object refs, checksums, and provenance after successful output validation.
7. Overcache marks the entry available only for the approved scope.
8. Invalidation rules mark entries stale or quarantined when inputs, packages, models, policies, or incidents change.
9. Eviction removes entries under pressure while preserving audit and usage summaries.

## State Machine

Cache entry lifecycle:

1. `candidate`: key/provenance identified but no stored content exists.
2. `reserved`: a producer or warming job owns a bounded write reservation.
3. `warming`: content is being copied or prepositioned under policy.
4. `available`: entry may be reused by authorized callers.
5. `stale`: entry exists but is not eligible because expiry, input, package, model, or policy changed.
6. `quarantined`: entry is isolated pending incident, dispute, or malware/provenance review.
7. `evicting`: eviction is in progress.
8. `evicted`: bytes were removed but metadata/audit summary remains.
9. `purged`: metadata and bytes were removed under retention or compliance rule.

Scope changes require a new policy-approved entry or grant. Entries do not silently move from narrow to broad scopes.

## Policy And Security

- Trust scopes are mandatory on every lookup, commit, and grant.
- Cache scope order is not a free promotion ladder; broader reuse needs explicit Overguard approval and evidence.
- Cache keys must include policy-relevant inputs so incompatible workloads cannot collide.
- Private tenant, regulated, secret-bearing, and personal data must stay isolated unless a later restricted encrypted-cache policy explicitly permits reuse.
- Cached outputs must carry provenance; unknown provenance defaults to deny.
- Invalidation must run when package, model, input, schema, policy, or trust evidence changes.
- Quarantine is required for suspected poisoned artifacts, malware, impossible provenance, or disputed outputs.
- Access grants must be short-lived, actor/workload-bound, and traceable.
- Node-local cache stores must not expose content across tenants through filesystem paths, shared layers, or side channels.

## Metering And Accounting

Overcache is an efficiency layer and must make savings visible:

- Emit hit, miss, write, read, egress, storage-byte, retention, invalidation, warming, eviction, and saved-upstream-work events.
- Attribute usage to tenant, workload, app/service account, cache scope, producer, and node locality where relevant.
- Keep native-service economics structural and near-cost; do not encode price or revenue forecasts.
- Support Overmeter rollups that compare cache cost against avoided compute/model/storage/network work.
- Avoid per-operation external payment calls; accounting later flows through ORU and Seal Ledger rollups.

## Observability And Operations

- Operators need views for hit rate by scope, storage pressure, stale entries, invalidation latency, quarantine backlog, cross-scope denials, and top saved-work classes.
- Health checks should verify metadata store, object ref access, Overguard policy path, Overmeter events, Overwatch events, and eviction workers.
- Maintenance jobs should support dry-run invalidation and dry-run eviction before destructive operations.
- Retention policy must be inspectable by data class and scope.
- Incident tooling must quickly quarantine by package/model/input/policy/evidence refs.
- Cache warming must expose budget, target node/scope, and failed artifact refs.

## Failure Modes And Recovery

- Metadata hit but object missing: mark entry stale, emit integrity event, and return miss.
- Object checksum mismatch: quarantine entry and block reuse.
- Policy engine unavailable: deny broader reuse and optionally allow narrow private-tenant reuse only if cached policy decision remains valid.
- Commit conflict: preserve first committed content for the key unless idempotency proves the same producer attempt.
- Invalidation worker failure: keep entries unavailable once invalidation is recorded, then retry propagation.
- Eviction failure: leave entry state unchanged or `evicting` with retry evidence; do not purge metadata first.
- Poisoned cache suspicion: quarantine by provenance graph and notify Overwatch/incident response.

## Validation Plan

The service implementation plan lists these requirements:

- Cache entries never cross into broader trust scopes without policy approval.
- Cache hits are auditable and metered.
- Invalidated entries are not reused by later workloads.

Additional SDS-level validation:

- Contract tests for lookup, reserve, commit, grant, invalidation, quarantine, eviction, and warming APIs.
- Tenant isolation tests proving private entries cannot be read by other tenants.
- Scope tests for private tenant, trusted swarm, federation grant, and public low-sensitivity behavior.
- Collision tests proving cache keys include policy-relevant refs.
- Provenance tests for package/model/input/schema/policy changes.
- Invalidation replay tests proving stale entries cannot become hits.
- Metering tests for hits, misses, writes, egress, storage, and saved-work rollups.

## Build Breakdown

1. Define cache key schema, metadata schema, trust-scope enum, provenance schema, and invalidation contract.
2. Implement metadata-only private tenant and trusted swarm lookup/commit over local or simple object refs.
3. Add Overguard policy checks and Overwatch audit events for reuse decisions.
4. Add invalidation, stale state, and restricted provenance reads.
5. Add Overmeter cache hit/miss/storage/egress events.
6. Add quarantine and eviction controls.
7. Expand to federation grant and public low-sensitivity scopes after trust and namespace controls are ready.
8. Integrate Overstore-backed object refs and broader AI/native-app/deployment reuse after Phase 8.

The first useful design work is Phase 4 trust scopes. Large-scale byte storage belongs after the data/storage platform exists.

## Handoff And Downstream Use

Overcache improves efficiency for Overrun, Overpack, Docdex RAG, model routing, native apps, deployment artifacts, and storage-heavy services. Downstream clients must treat cache hits as policy decisions with evidence, not as uncontrolled file paths.

## Open Design Questions

Resolved decisions:

- Phase 4 may implement only metadata-first cache classes whose bytes are either absent, already tenant-local, or already held by an execution artifact store under the same private boundary. Safe Phase 4 classes are private-tenant lookup metadata, trusted-private-swarm package/runtime compatibility metadata, policy-decision and validation-result refs, cache-locality hints for Oversched, and bounded node-local package/artifact refs for the same tenant or private swarm. Phase 4 must not enable broad model-output reuse, public dataset reuse, cross-tenant reuse, federation-grant reuse, regulated/secret-bearing caching, or public low-sensitivity byte reuse as a production path. Those broader classes require Overstore/Overvault/namespace policy, transfer, retention, and integrity controls from Phase 8 or later.
- Model-output cache keys must never contain raw prompts, retrieved chunks, private documents, generated outputs, or secret-bearing values. The key is a canonical hash over a redacted input envelope: tenant id, actor or service-account scope, workload/app id, cache scope, data class, prompt-template/version refs, model id/version, tool/runtime/schema refs, retrieval bundle refs, BLAKE3 hashes of context sets computed inside the authorized tenant/private scope, policy-decision refs, and output schema/version refs. Private, regulated, secret-bearing, and user-private model outputs stay `private_tenant` or are not cache-eligible unless a later restricted encrypted-cache policy explicitly allows encrypted refs through Overvault and Overstore.
- Public low-sensitivity reuse requires provenance that proves every input and output is eligible for public low-sensitivity treatment. The minimum record is: producing workload id, package id/version, model or tool id/version where applicable, input object/dataset refs and BLAKE3 hashes, data-class and purpose-tag refs, producer/provider identity and verification refs, Overguard allow decision with policy bundle/version and reason codes, Overwatch evidence refs, validation or checksum evidence, retention/license/rights refs where applicable, and a no-active-quarantine/no-active-dispute state. Unknown, private, mixed-scope, or unverified provenance defaults to deny rather than public reuse.
- Immediate read blocking is required for tenant suspension or role/key revocation, Overguard emergency blocks or policy revocation, data-class changes to private/regulated/secret-bearing, package/model/schema security revocation, input/object hash mismatch, missing or corrupted object refs, quarantine, malware/poisoning suspicion, producer/provider trust disqualification, rights or grant revocation, and Overclaim disputes that challenge correctness, ownership, privacy, or settlement. Grace-period staleness is allowed only for low-risk freshness changes such as normal TTL expiry, newer non-breaking package/model versions, soft benchmark age, storage pressure, planned retention migration, or public-static-content refresh where policy marks old content acceptable. During grace, entries cannot be promoted to broader scopes and cannot satisfy workloads that require fresh policy, package, model, or input evidence.
- Node-local cache pressure reports must be signed aggregate measurements, not content inventories. Overcell/Overcache may report total cache bytes, reserved bytes, reclaimable bytes, high-water marks, eviction backlog, storage class, trust-scope bucket, data-class bucket, retention class, age bucket, and coarse artifact class. Reports must not include raw paths, filenames, prompts, object bytes, private object ids, exact tenant content descriptions, or per-entry metadata visible outside the authorized tenant/operator view. Tenant-specific pressure views use redacted cache-entry refs through Overcache authorization, while Overmeter and Overwatch receive only aggregate dimensions and evidence refs needed for usage, audit, and eviction replay.
