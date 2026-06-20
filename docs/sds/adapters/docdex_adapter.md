SDS #65

# Docdex Adapter SDS

## Purpose

Make Docdex encrypted indexing, search, retrieval, and RAG context assembly first-class Overrid workloads.

Docdex Adapter translates Docdex daemon and encrypted-repository operations into Overrid jobs. It owns Docdex repo bindings, workload manifests, index/search/retrieval/admin-ingest jobs, capability snapshots, result refs, deletion/deprovision jobs, and usage records. It does not authorize assistant context bundles, choose models, run final RAG answers, or become the source of Overrid identity/accounting truth.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [docdex_adapter.md](../../service_catalog/adapters/docdex_adapter.md) |
| Sub-build plan | [SUB BUILD PLAN #65 - Docdex Adapter](../../build_plan/sub_build_plan_065_docdex_adapter.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md) |
| Docdex implementation context | `/Users/bekirdag/Documents/apps/docdex` |

## Service Family

- Family: Ecosystem adapters
- Owning layer: Docdex workload translation and product integration
- Primary data scope: Docdex instance refs, repo bindings, encrypted repo config refs, index job manifests, search/retrieval job refs, admin ingest jobs, capability snapshots, result refs, cleanup/deletion jobs, usage refs, and audit evidence
- First build phase from service plan: [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md)

## Problem Statement

Docdex is a natural first product integration because it already turns repositories and documents into private, local-first search context for AI agents. Overrid needs to run Docdex indexing, search, retrieval, and encrypted repository flows as normal grid workloads with identity, policy, queues, storage refs, metering, and audit.

This adapter must keep the boundary clean. Docdex Adapter should operate Docdex jobs and expose capability/result refs. Encrypted Docdex RAG Adapter should decide which context is authorized for an assistant/model route. AI Gateway Router should choose the model/resource route. Overrid accounting and audit services should remain the durable usage truth.

## Goals

- Register Docdex instances, repos, encrypted repo configs, and capability snapshots as Overrid-visible resources.
- Submit Docdex index, search, retrieval-only RAG, memory/profile alias, and admin-ingest jobs through Overrid workload rails where allowed.
- Support encrypted-repository constraints from Docdex: application-managed key refs, access checks, bounded results/snippets, optional plaintext term-index leakage, and disabled structural features.
- Store index/search/retrieval outputs as result refs, not broad raw content dumps.
- Coordinate with Encrypted Docdex RAG Adapter for context authorization and bundle creation.
- Emit usage refs for index, search, retrieval, admin ingest, storage, and deletion/deprovision work.
- Prove Phase 6 product integration through SDK/CLI/admin UI without direct internal API calls.

## Non-Goals

- Do not reimplement Docdex indexer, search engine, MCP server, memory system, or encrypted repository internals.
- Do not authorize final RAG context for personal AI; Encrypted Docdex RAG Adapter owns that boundary.
- Do not choose models or run final LLM inference.
- Do not store raw Docdex encryption keys or bypass Overvault key refs.
- Do not expose encrypted repo plaintext, snippets, logs, or indexes to unauthorized services.
- Do not make Docdex the owner of Overrid identities, tenants, ORU balances, Seal Ledger records, or policy decisions.
- Do not add pricing forecasts, customer-count assumptions, blockchain mechanics, NFT mechanics, or per-transaction fee economics.

## Primary Actors And Clients

- Docdex runtime/daemon or managed Docdex service submitting and receiving workload refs.
- Encrypted Docdex RAG Adapter requesting authorized retrieval jobs and capability snapshots.
- Personal AI Assistant, Codali Adapter, Mcoda Adapter, AI Gateway Router, and native apps consuming Docdex result refs through proper authorization layers.
- SDK, CLI, and admin UI submitting index/search/retrieval work and inspecting status.
- Overpack, Overqueue, Oversched, Overlease, Overrun, Overstore, Overvault, Overmeter, and Overwatch providing workload, execution, storage, secret, usage, and audit rails.
- Overguard and Overtenant enforcing tenant, data-class, and workload policy.

## Dependencies

- Docdex daemon/API, including index, search, retrieval-only RAG, tree/files/stats/repo-inspect, encrypted-search compatibility, and service-admin ingest surfaces.
- [Encrypted Docdex RAG Adapter](../ai_rag_model_routing/encrypted_docdex_rag_adapter.md) for authorized context use.
- [AI Gateway Router](../ai_rag_model_routing/ai_gateway_router.md) for model/resource route decisions when Docdex results feed AI work.
- [Overpack](../execution_scheduling/overpack.md), [Overqueue](../control_plane/overqueue.md), [Oversched](../execution_scheduling/oversched.md), [Overlease](../execution_scheduling/overlease.md), and [Overrun](../execution_scheduling/overrun.md) for workload packaging and execution.
- [Overstore](../data_storage_namespace/overstore.md), [Overbase](../data_storage_namespace/overbase.md), and [Overvault](../data_storage_namespace/overvault.md) for result refs, state, and key/secret refs.
- [Overguard](../trust_policy_verification/overguard.md), [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), and [Overwatch](../control_plane/overwatch.md) for policy, usage, accounting refs, and audit.

## Owned Responsibilities

Docdex Adapter owns:

- Docdex instance registry refs and health/capability snapshots.
- Repo binding records connecting Overrid tenant/owner scopes to Docdex repo ids.
- Encrypted repo config refs, key-ref status, access-binding refs, and leakage/capability facts surfaced from Docdex.
- Index job manifests and async job status mapping.
- Search and retrieval-only RAG job records and result refs.
- Service-admin document ingest records for managed encrypted repos, including plaintext cleanup evidence where applicable.
- Deletion/deprovision job records for repo roots, state roots, and access bindings where policy allows.
- Usage/audit refs for Docdex operations.

## Data Model

- `docdex_instance_ref`: endpoint/service ref, version, health, supported surfaces, auth mode, tenant visibility, runtime placement, and capability snapshot refs.
- `docdex_repo_binding`: Overrid tenant/owner refs, Docdex repo id, repo root/storage ref, access binding refs, data classes, policy refs, and state.
- `encrypted_repo_config_ref`: encryption mode, key ref, key id, shared-token status, plaintext term-index status, semantic/web/full-open flags, max result/snippet caps, and residual leakage notes.
- `docdex_workload_manifest`: operation class, repo binding, paths/query/context, expected result shape, queue/runtime needs, idempotency key, trace id, and policy refs.
- `docdex_index_job`: job id, paths total/indexed/failed, queued/started/finished times, errors, state, and result refs.
- `docdex_search_job`: query hash, search parameters, include_libs flag, result refs, limits, ranking surface, and failure reason.
- `docdex_retrieval_job`: retrieval-only RAG request, context metadata, result refs, context consumer refs, and handoff refs to Encrypted Docdex RAG Adapter.
- `admin_ingest_job`: service-admin source refs, target repo binding, content encoding class, ingest result, plaintext cleanup status, and audit refs.
- `docdex_result_ref`: indexed file refs, ranked hit refs, snippet refs, stats refs, capability refs, error refs, and Overstore refs where materialized.
- `docdex_usage_ref`: operation class, repo id, bytes/classes, path count, query/result counts, index time, retrieval time, storage refs, and Overmeter refs.

## API Surface

- `POST /docdex/instances`: registers a Docdex instance or managed Docdex connector.
- `GET /docdex/instances/{instance_id}/capabilities`: returns health, supported operations, encryption capability, and limits.
- `POST /docdex/repos`: creates or imports a Docdex repo binding for an Overrid tenant/owner scope.
- `GET /docdex/repos/{repo_binding_id}`: returns repo binding, encrypted config, status, and visible capability facts.
- `POST /docdex/index-jobs`: submits an index job manifest.
- `GET /docdex/index-jobs/{job_id}`: returns async index status and errors.
- `POST /docdex/search-jobs`: submits a bounded search job.
- `POST /docdex/retrieval-jobs`: submits retrieval-only RAG work and returns result refs.
- `POST /docdex/admin-ingest-jobs`: submits service-admin document ingest for managed encrypted repos.
- `POST /docdex/repos/{repo_binding_id}/deprovision`: requests bounded repo/state/access-binding cleanup.
- `GET /docdex/jobs/{job_id}`: returns state, result refs, errors, usage refs, and audit refs for any Docdex job type.
- `GET /docdex/replay/{job_id}`: reconstructs manifest, policy, capability, execution, result, cleanup, and usage decisions.

Mutating APIs require actor/service identity, tenant scope, repo binding, trace id, idempotency key, policy refs, and key/secret refs where applicable. Stable errors include `docdex_instance_unavailable`, `repo_binding_missing`, `repo_access_denied`, `repo_encryption_key_unavailable`, `repo_encryption_key_invalid`, `operation_disabled_for_encrypted_repo`, `plaintext_term_leakage_unaccepted`, `index_job_failed`, `retrieval_limit_exceeded`, `admin_ingest_cleanup_failed`, `deprovision_not_allowed`, and `policy_denied`.

## Event Surface

- `docdex_adapter.instance_registered`: Docdex instance/connector registered.
- `docdex_adapter.repo_bound`: Overrid scope bound to Docdex repo id.
- `docdex_adapter.capability_snapshot_updated`: capabilities, limits, health, or encryption facts changed.
- `docdex_adapter.index_job_submitted`: index job submitted.
- `docdex_adapter.index_job_completed`: index job completed or failed.
- `docdex_adapter.search_job_completed`: search job completed with result refs.
- `docdex_adapter.retrieval_job_completed`: retrieval-only RAG job completed with result refs.
- `docdex_adapter.admin_ingest_completed`: admin ingest completed with cleanup evidence.
- `docdex_adapter.repo_deprovisioned`: repo/state/access binding cleanup completed.
- `docdex_adapter.job_failed`: job failed with reason code.
- `docdex_adapter.usage_emitted`: usage refs emitted.

Events include instance refs, repo binding refs, capability refs, job refs, result refs, policy refs, cleanup refs, failure reason, and usage refs. They must not include raw encrypted repo content, raw keys, unredacted snippets, or private source documents.

## Core Workflow

1. Register a Docdex instance and repo binding with tenant/owner scope, access policy, and encrypted repo config refs.
2. Caller submits an index, search, retrieval, admin-ingest, or cleanup job through SDK/CLI/API.
3. Adapter validates identity, tenant, repo binding, operation class, encrypted-repo capability, key refs, idempotency, and policy.
4. Adapter creates an Overpack-compatible workload manifest or direct connector call according to the phase/runtime profile.
5. Overqueue/Overrun or the connector executes the Docdex operation and returns status/result refs.
6. Adapter records result refs, capability/leakage facts, errors, cleanup status, usage refs, and audit evidence.
7. Encrypted Docdex RAG Adapter consumes retrieval/capability refs for context authorization where AI use is requested.

## State Machine

Repo binding lifecycle:

1. `proposed`
2. `policy_checked`
3. `key_checked`
4. `bound`
5. `active`
6. `degraded`
7. `suspended`
8. `deprovisioning`
9. `deprovisioned`

Job lifecycle:

1. `submitted`
2. `validated`
3. `queued`
4. `running`
5. `result_captured`
6. `usage_recorded`
7. `completed`
8. `retryable_failed`
9. `terminal_failed`
10. `cancelled`

Admin ingest lifecycle:

1. `submitted`
2. `service_admin_checked`
3. `source_staged`
4. `indexed`
5. `plaintext_cleanup_verified`
6. `completed`
7. `cleanup_failed`
8. `failed`

## Policy And Security

- Repo bindings require tenant/owner scope, access policy, and service-account authorization.
- Encrypted repository operations require key refs and access checks; shared bearer tokens are not sufficient when encryption is enabled.
- Surface Docdex residual leakage facts, including plaintext term-index leakage and visible metadata, to RAG and route layers.
- Respect disabled encrypted-repo operations such as structural AST/symbol/impact features unless Docdex later exposes safe support.
- Admin ingest must remove or avoid durable plaintext source artifacts after encrypted indexing where the Docdex mode requires it.
- Result refs must be scoped and redacted before visibility beyond the owning tenant/user/service.
- Deprovision jobs require explicit policy and audit because they remove repo roots, state roots, or access bindings.
- Operator overrides require signed action and Overwatch evidence.

## Metering And Accounting

- Emit usage refs for indexing, search, retrieval, admin ingest, stats/files/tree calls where material, storage, cleanup, retries, and failed jobs.
- Link usage to tenant, owner, actor/service account, repo binding, job id, operation class, result refs, and downstream context/route refs.
- Failed or cleanup-failed jobs must be visible for dispute and support.
- ORU Account Service and Wallet/Usage Center consume rollups downstream; Docdex Adapter does not keep balances.
- Do not encode hardcoded prices, forecasts, provider payouts, or per-call financial assumptions.

## Observability And Operations

- Expose instance health, repo binding count, encrypted repo key status, index backlog, job latency, search/retrieval errors, admin-ingest cleanup failures, deprovision failures, and usage by operation class.
- Alert on missing key material, invalid key material, encrypted-repo operation mismatch, plaintext cleanup failure, unauthorized repo binding access, and repeated Docdex instance failures.
- Provide replay for manifests, capability snapshots, policy checks, encrypted config, execution state, results, cleanup, and usage refs.
- Provide SDK/CLI/admin diagnostics for repo status, index jobs, search/retrieval jobs, encryption capabilities, and degraded operations.

## Failure Modes And Recovery

- Docdex instance unavailable: queue retry if allowed or fail job with instance reason code.
- Repo binding missing or stale: reject before side effects and request rebinding.
- Key material missing/invalid: block encrypted operation and emit key-status event.
- Operation disabled for encrypted repo: fail closed and return capability reason code.
- Index job partial failure: preserve path counts, failed path refs, errors, and retry manifest.
- Admin ingest cleanup fails: mark cleanup failed and block completed status until resolved.
- Retrieval result exceeds cap: truncate according to policy or fail if truncation would be misleading.
- Usage emission fails: retain job/result refs for later reconciliation before completion.

## Validation Plan

- Docdex can submit real jobs through SDK/CLI and receive status/result refs without direct internal API calls.
- Encrypted repo operations enforce access checks, key refs, and operation capability limits.
- Plaintext term-index leakage and disabled structural features are visible in capability snapshots.
- Retrieval-only RAG results feed Encrypted Docdex RAG Adapter instead of bypassing context authorization.
- Admin ingest cleans up plaintext source artifacts or fails closed when cleanup is required.
- Usage records show index, search, retrieval, admin ingest, storage, retry, and failure dimensions.
- Replay reconstructs repo binding, manifest, capability, policy, execution, result, cleanup, and usage decisions.

## Build Breakdown

1. Define Docdex instance, repo binding, encrypted config, workload manifest, index/search/retrieval/admin-ingest job, result, cleanup, and usage schemas.
2. Implement instance and repo-binding APIs with policy, key-ref, and capability checks.
3. Implement index/search/retrieval job submission through Overpack/Overqueue or connector calls, preserving final interface shape.
4. Integrate encrypted repo capability snapshots, leakage facts, and disabled operation handling.
5. Implement admin ingest and cleanup evidence for managed encrypted repos.
6. Integrate Encrypted Docdex RAG Adapter handoff, Overmeter usage refs, Overwatch replay, SDK/CLI/admin diagnostics.
7. Harden retry, deprovision, key-failure, and cleanup-failure validation before broad product use.

## Handoff And Downstream Use

Docdex Adapter hands repo binding refs, capability snapshots, index/search/retrieval job refs, result refs, encrypted config facts, admin-ingest cleanup refs, usage refs, and replay bundles to Encrypted Docdex RAG Adapter, Personal AI Assistant, Codali Adapter, Mcoda Adapter, AI Gateway Router, SDK, CLI, admin UI, Overmeter, and Overwatch.

Downstream AI flows must obtain context authorization from Encrypted Docdex RAG Adapter before passing Docdex retrieval output to a model route.

## Open Design Questions

Resolved decisions:

- Phase 6 should start with a local Docdex daemon on founder seed/private hardware, reached through a Rust Docdex Adapter connector that uses the same signed, tenant-scoped, idempotent job envelopes the later grid path will use. This proves real encrypted index, search, retrieval, admin-ingest, capability, usage, and replay flows without pretending Docdex is already a grid-resident system service. A managed connector may wrap that same local/seed daemon for controlled product demos and service-admin ingest, but it must preserve Overrid-shaped manifests, result refs, Overvault key refs, Overmeter usage refs, and Overwatch evidence. A grid-resident Docdex package is a later Phase 7/Phase 9 hardening target after system-service workload placement, package validation, backup/restore, and release strategy are proven.
- User-facing privacy UI must represent plaintext term-index leakage as an explicit encrypted-index leakage profile, not as a generic encrypted/on-off badge. The default visible mode is `encrypted_metadata_only`; `encrypted_term_index_limited` is available only when the owner or tenant accepts term-presence leakage, visible path class, token-count class, result-count class, capability limits, retention/expiry, and affected operations. The UI should show safe aliases, owner scope, data class, leakage class, selected caps, disabled features, key/capability status, citation/snippet-hash refs, denial reasons, usage refs, and Overwatch audit refs, while hiding raw plaintext snippets outside authorized bundles, exact private paths where only classes are permitted, key material, vault refs, other users' queries, and admin-only Docdex instance details.
- Phase 6 exposes Docdex memory/profile/conversation surfaces through this adapter only as capability facts, owner-scoped aliases, job/result refs, and explicitly authorized retrieval or admin-ingest refs. Repo-scoped search/retrieval can feed Encrypted Docdex RAG Adapter context bundles when the owner grants access, and service-admin ingest may import approved documents into managed encrypted repos with cleanup evidence. Raw profile preference stores, conversation transcripts, diary entries, temporal-graph internals, global agent preferences, and memory-evolution controls remain local-only unless the owning user or tenant imports a bounded subset into a managed encrypted repo or grants a route/request-bound context bundle through Encrypted Docdex RAG Adapter. Docdex memory/profile data never becomes Overrid identity, tenant, policy, ledger, or permission truth.
- Deprovision requires a two-phase evidence bundle before deleting repo state, state roots, encrypted index material, or access bindings. The prepare phase records signed actor/service authority, tenant/owner scope, repo binding id, policy and retention decisions, legal hold/dispute/support checks, pending job cancellation or drain state, key/access-binding revocation plan, export or explicit-discard choice, usage/receipt reconciliation, and a dry-run manifest of paths/state roots/access refs to remove. The execute phase records deletion receipts, BLAKE3 hashes or tombstone refs for removed state, Overstore/Overvault cleanup refs, failed-item refs, usage/audit refs, and replay evidence. If any cleanup, retention, hold, key, or access-binding condition is unresolved, the repo binding moves to `suspended` or `deprovisioning` rather than `deprovisioned`.
- Encrypted repositories keep structural code-intelligence features disabled until Docdex provides encrypted-safe support with explicit capability snapshots and leakage evidence. The disabled set includes AST extraction, symbol indexes, dependency or impact graphs, cross-file call graphs, broad tree/path enumeration beyond authorized classes, full-open file/snippet reads, raw plaintext index export, semantic or web expansion outside the registered encrypted index scope, and any batch/rerank/debug surface that would materialize unauthorized plaintext or exact private structure. Safe support must be opt-in by leakage profile, constrained by owner/tenant policy, visible to Encrypted Docdex RAG Adapter, AI Gateway Router, Personal AI Assistant privacy audit, and Overwatch replay, and must fail closed when key, capability, redaction, retention, or access-binding facts change.
