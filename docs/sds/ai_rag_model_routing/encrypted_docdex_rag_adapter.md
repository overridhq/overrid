SDS #61

# Encrypted Docdex RAG Adapter SDS

## Purpose

Connect encrypted Docdex indexes to Overrid workloads and personal, organization, and repository AI context retrieval.

Encrypted Docdex RAG Adapter is the authorization and context-assembly boundary between Docdex indexes and Overrid AI flows. It decides which encrypted indexes may be used, converts retrieval output into privacy-bounded context refs, records leakage and policy facts, and emits usage/audit evidence. It does not run Docdex itself, decrypt private repositories for the router, choose the final model, or run model inference.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [encrypted_docdex_rag_adapter.md](../../service_catalog/ai_rag_model_routing/encrypted_docdex_rag_adapter.md) |
| Sub-build plan | [SUB BUILD PLAN #61 - Encrypted Docdex RAG Adapter](../../build_plan/sub_build_plan_061_encrypted_docdex_rag_adapter.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md), [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |
| Docdex implementation context | `/Users/bekirdag/Documents/apps/docdex` |

## Service Family

- Family: AI, RAG, and model routing
- Owning layer: Authorized encrypted-index retrieval and context bundle assembly
- Primary data scope: index refs, retrieval scopes, context grants, leakage profiles, retrieval requests/results, redaction plans, context bundle refs, usage refs, and audit evidence
- First build phase from service plan: [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md), expanded for native AI in [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), with leakage, revocation, retention, privacy, incident, and compliance hardening in [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md)

## Problem Statement

The personal AI assistant, code agents, workspace, and native apps need private context without uploading raw personal or repository content to a generic cloud RAG system. Docdex already provides local-first indexing, encrypted-repository compatibility paths, retrieval-only RAG, memory surfaces, and repo isolation. Overrid needs a service boundary that can safely connect those capabilities to the grid.

The risky design would let the router or a model worker read raw encrypted index contents directly. This adapter prevents that. It forces every context request through tenant/person/org/repo authorization, records what leakage remains, returns scoped context refs rather than broad storage access, and lets AI Gateway Router choose models only after context policy has been resolved.

## Goals

- Register Docdex encrypted indexes as Overrid context resources with explicit owner, scope, encryption, and leakage metadata.
- Submit or reference Docdex index/search/retrieval work through the Docdex Adapter without becoming the Docdex runtime.
- Authorize retrieval against person, organization, workspace, repo, and tenant boundaries before context is exposed.
- Produce bounded RAG context bundles with selected snippets, redaction profile, citation refs, result hashes, and retention rules.
- Expose encrypted-repository constraints to AI Gateway Router, including key status, plaintext term-index leakage, unavailable structural features, and result limits.
- Support personal AI, workspace AI, code agents, central AI evidence analysis, and native apps through one context authorization contract.
- Meter index/search/retrieval/context-assembly usage and produce Overwatch evidence without storing raw private content unnecessarily.

## Non-Goals

- Do not implement Docdex indexing/search internals; Docdex Adapter and Docdex runtime own those operations.
- Do not choose the final model or execution resource; AI Gateway Router owns route decisions.
- Do not run model inference or final answer generation.
- Do not bypass Overvault key refs, Overguard policy checks, tenant scope, user permissions, or repo access bindings.
- Do not expose raw repository, workspace, or personal content to routers, classifiers, logs, metrics, or unrelated services.
- Do not claim encrypted search has zero leakage when plaintext term indexes, paths, token counts, result counts, or metadata remain visible.
- Do not add blockchain, NFT, per-transaction fee, revenue forecast, or customer-count assumptions.

## Primary Actors And Clients

- Personal AI Assistant requesting authorized user context.
- AI Gateway Router requesting context plans before model routing.
- Docdex Adapter executing index/search/retrieval work against Docdex.
- Codali Adapter requesting repository context for code-agent tasks.
- Workspace, Search, Messaging, Directory, and other native apps requesting user-approved context.
- Central AI Service requesting evidence-bounded context where policy permits.
- Overguard, Overpass, Overtenant, and Overkey providing identity, tenant, and policy facts.
- Overvault and Overstore providing key refs, encrypted object refs, and context artifact storage.
- Overmeter, Wallet/Usage Center, and Overwatch consuming usage and audit refs.

## Dependencies

- [Docdex Adapter](../adapters/docdex_adapter.md) for Docdex daemon/job translation, repo binding, index/search/retrieval calls, and capability snapshots.
- [AI Gateway Router](ai_gateway_router.md) for downstream model/resource route decisions.
- [Lightweight Classifier](lightweight_classifier.md) and [ADES Enrichment Adapter](ades_enrichment_adapter.md) for optional advisory hints.
- [Overguard](../trust_policy_verification/overguard.md), [Workload Classifier](../trust_policy_verification/workload_classifier.md), and [Policy Dry-Run API](../trust_policy_verification/policy_dry_run_api.md) for policy and data-class decisions.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), and [Overkey](../control_plane/overkey.md) for user, tenant, and credential facts.
- [Overvault](../data_storage_namespace/overvault.md) for key refs, grants, and secret access decisions.
- [Overstore](../data_storage_namespace/overstore.md) and [Overbase](../data_storage_namespace/overbase.md) for context bundle refs, retrieval metadata, and state.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), and [Overwatch](../control_plane/overwatch.md) for usage and audit.

## Owned Responsibilities

Encrypted Docdex RAG Adapter owns:

- Index authorization records connecting Docdex repo/index refs to Overrid actor, tenant, person, organization, workspace, and repository scopes.
- Context-scope manifests describing what an assistant, app, or code agent may retrieve.
- Retrieval requests, result refs, redaction profiles, and context bundle refs.
- Leakage profiles for encrypted-index modes, including plaintext term-index leakage and disabled structural features.
- Context grants and denial records used by AI Gateway Router and Personal AI Assistant.
- RAG usage refs for index, search, retrieval, context assembly, redaction, and bundle storage.
- Replay bundles proving which policy, key, access, and leakage facts allowed or denied context.

## Data Model

- `encrypted_index_ref`: Docdex repo id, index id, owner scope, tenant id, repository/workspace/person refs, encryption mode, key ref, key id, access-binding refs, last indexed time, and capability snapshot ref.
- `context_scope_manifest`: allowed repos/folders/doc classes, purpose tag refs, data classes, max result count, max snippet size, redaction profile, retention class, allowed clients, and denied scopes.
- `retrieval_request`: actor/tenant refs, caller app, query hash, requested scopes, task class, privacy mode, purpose refs, idempotency key, trace id, and policy refs.
- `retrieval_result_ref`: Docdex job/search ref, ranked result refs, citation refs, snippet hashes, result count, redaction status, leakage profile ref, and retrieval timestamp.
- `context_bundle_ref`: bounded context payload ref in Overstore, selected result refs, redaction plan, context-window budget, prompt-inclusion rules, expiry, retention class, and downstream route refs.
- `context_grant`: grant id, actor refs, scope refs, allowed client/service refs, allowed use, expiry, revocation state, and Overvault/Overguard decision refs.
- `encrypted_index_leakage_profile`: plaintext term-index status, visible paths/token-count classes, disabled operations, semantic/web/full-open capability flags, and residual leakage notes.
- `rag_usage_ref`: operation class, index/search/retrieval/context assembly counters, bytes/classes, model-route refs where available, and Overmeter rollup refs.
- `rag_audit_record`: policy decision refs, key-status refs, denied scope refs, selected scope refs, operator override refs, and replay hash.

Raw private content is not a durable service record unless a user or tenant explicitly stores a context bundle under an allowed retention policy. Durable records prefer refs, hashes, redaction metadata, result IDs, and citations.

## API Surface

- `POST /docdex-rag/index-refs`: registers or updates an encrypted Docdex index ref visible to Overrid callers.
- `GET /docdex-rag/index-refs/{index_ref_id}`: returns capability, owner, leakage, and authorization metadata visible to the caller.
- `POST /docdex-rag/retrievals/dry-run`: previews allowed scopes, denied scopes, leakage facts, and expected usage class without retrieving content.
- `POST /docdex-rag/retrievals`: authorizes and executes a bounded retrieval through Docdex Adapter.
- `GET /docdex-rag/retrievals/{retrieval_id}`: returns redacted retrieval metadata, result refs, state, and usage refs.
- `POST /docdex-rag/context-bundles`: creates a context bundle from authorized retrieval results.
- `GET /docdex-rag/context-bundles/{bundle_id}`: returns bundle metadata and payload ref if the caller still has permission.
- `POST /docdex-rag/context-grants`: creates, narrows, or renews an explicit context grant.
- `POST /docdex-rag/context-grants/{grant_id}/revoke`: revokes future access and emits downstream invalidation events.
- `GET /docdex-rag/replay/{record_id}`: reconstructs authorization, leakage, policy, retrieval, redaction, and usage decisions.

Mutating APIs require actor/service identity, tenant scope, context owner scope, idempotency key, trace id, data class, purpose refs, and policy refs. Stable errors include `index_ref_not_found`, `context_scope_denied`, `key_ref_missing`, `key_material_unavailable`, `encrypted_search_leakage_unaccepted`, `structural_feature_unavailable`, `retrieval_limit_exceeded`, `context_bundle_expired`, `grant_revoked`, `policy_denied`, and `docdex_adapter_unavailable`.

## Event Surface

- `encrypted_docdex_rag_adapter.index_ref_registered`: encrypted index ref registered or updated.
- `encrypted_docdex_rag_adapter.scope_evaluated`: context scope was allowed, narrowed, or denied.
- `encrypted_docdex_rag_adapter.retrieval_requested`: retrieval request accepted for authorization.
- `encrypted_docdex_rag_adapter.retrieval_denied`: retrieval denied by scope, key, leakage, policy, or adapter state.
- `encrypted_docdex_rag_adapter.retrieval_completed`: retrieval completed with result refs.
- `encrypted_docdex_rag_adapter.context_bundle_created`: bounded context bundle created.
- `encrypted_docdex_rag_adapter.context_grant_revoked`: grant revoked and downstream invalidation requested.
- `encrypted_docdex_rag_adapter.leakage_profile_changed`: encrypted-index leakage facts changed.
- `encrypted_docdex_rag_adapter.usage_emitted`: RAG usage refs emitted to Overmeter.

Events include index refs, scope refs, policy refs, leakage profile refs, bundle refs, route refs, result classes, and usage refs. They must not include raw private content, raw prompts, decrypted files, secrets, or unredacted snippets unless the event stream is explicitly scoped to a private owner and retention policy permits it.

## Core Workflow

1. Register a Docdex encrypted index with owner scope, key refs, access bindings, capability snapshot, and leakage profile.
2. Caller requests context through Personal AI Assistant, AI Gateway Router, Codali Adapter, native app, or Central AI.
3. Adapter validates actor, tenant, owner scope, purpose, data class, context grant, key status, leakage acceptance, and policy.
4. Adapter narrows the retrieval scope to allowed indexes, paths, document classes, and result limits.
5. Adapter calls Docdex Adapter for retrieval-only search/RAG work and receives result refs.
6. Adapter redacts, truncates, ranks, and packages allowed snippets into a context bundle ref.
7. AI Gateway Router receives the context plan and bundle ref, then selects a model/resource route.
8. Adapter emits usage and audit refs and retains replay evidence without leaking raw private content.

## State Machine

Index ref lifecycle:

1. `proposed`
2. `owner_verified`
3. `key_checked`
4. `capability_loaded`
5. `leakage_profiled`
6. `active`
7. `degraded`
8. `revoked`
9. `archived`

Retrieval lifecycle:

1. `submitted`
2. `scope_checked`
3. `key_checked`
4. `leakage_checked`
5. `policy_checked`
6. `retrieving`
7. `redacting`
8. `bundle_created`
9. `completed`
10. `denied`
11. `failed`
12. `cancelled`

Context bundle lifecycle:

1. `created`
2. `route_attached`
3. `consumed`
4. `expired`
5. `revoked`
6. `redacted`

## Policy And Security

- Every retrieval requires actor, tenant, owner scope, purpose, data-class, and context-grant checks.
- Overvault key refs must be used for key access; the adapter must not persist raw key material.
- The adapter must surface encrypted-index leakage facts to callers and route decisions. If plaintext term indexing is enabled, treat term-presence leakage as an explicit capability and policy fact.
- Structural code intelligence that Docdex disables for encrypted repos must not be silently simulated from unauthorized plaintext.
- Context bundles should be least-context, time-bounded, and bound to a route/session/tool use.
- Fallback cannot widen context scope or weaken privacy rules.
- User revocation invalidates future context access and forces downstream route/tool calls to reauthorize.
- Operator overrides require signed action, policy refs, user/tenant scope, and Overwatch evidence.

## Metering And Accounting

- Emit usage refs for index ref registration, retrieval calls, result count classes, context assembly, redaction, bundle storage, retries, and failed/denied attempts.
- Link usage to actor, tenant, owner scope, caller app, index ref, retrieval id, route id, and context bundle ref.
- Keep usage structural and auditable; ORU Account Service and Wallet/Usage Center consume rollups downstream.
- Denied retrievals may emit low-cost policy/audit usage but must not be recorded as successful context retrieval.
- Do not encode hardcoded prices, revenue projections, model-provider payout logic, or per-transaction fee assumptions.

## Observability And Operations

- Expose counts and latency for scope checks, retrievals, denials, redactions, bundle creation, grant revocations, Docdex adapter failures, and key failures.
- Track leakage-profile distribution across encrypted index refs.
- Alert on private-context leakage attempts, unknown key refs, stale access bindings, repeated denied scopes, and bundle access after revocation.
- Provide replay for context authorization, leakage acceptance, policy decisions, retrieval results, and context bundle creation.
- Provide SDK/CLI/admin diagnostics for index refs, capability snapshots, degraded encrypted indexes, and denied context scopes.

## Failure Modes And Recovery

- Key ref missing or unavailable: deny retrieval and emit `key_ref_missing` or `key_material_unavailable`.
- Leakage profile not accepted: deny retrieval until caller or policy accepts/narrows the leakage mode.
- Docdex Adapter unavailable: keep retrieval pending if retryable; otherwise fail with adapter reason code.
- Index stale: return degraded result or require reindex depending on caller quality policy.
- Retrieval result contains unauthorized scope: discard result, fail closed, and emit an incident-grade audit event.
- Redaction fails: do not create context bundle; preserve metadata for retry or support review.
- Grant revoked during request: cancel or deny before route execution unless policy permits the already-created bundle to expire naturally.
- Usage emission fails: retain retrieval and bundle refs for later reconciliation before terminal completion.

## Validation Plan

- Authorized retrieval succeeds only for indexes and scopes explicitly allowed by actor, tenant, owner, purpose, and data-class policy.
- Unauthorized repo/person/org/workspace scope cannot be retrieved through dry-run, retrieval, bundle, replay, or admin paths.
- Encrypted-index leakage facts are visible to AI Gateway Router and preserved in replay.
- Plaintext term-index leakage cannot be hidden by the adapter.
- Disabled encrypted-repo structural features are not exposed as available capabilities.
- Raw private content does not appear in router logs, classifier records, metrics, public events, or service catalog audit exports.
- Context grants revoke future access and invalidate downstream bundle use.
- Replay reconstructs scope, key, leakage, policy, retrieval, redaction, bundle, and usage decisions.

## Build Breakdown

1. Define encrypted index ref, context scope, retrieval request/result, leakage profile, context grant, bundle, usage, and audit schemas.
2. Integrate Docdex Adapter capability snapshots and retrieval-only RAG calls.
3. Implement dry-run scope evaluation with Overguard, Overvault, Overpass, Overtenant, and owner-scope checks.
4. Implement bounded retrieval and context bundle creation with redaction, citation refs, result hashes, and retention policy.
5. Integrate AI Gateway Router context plans, Lightweight Classifier/ADES hints where useful, and Personal AI Assistant permissions.
6. Add usage refs, Overwatch replay, grant revocation, SDK/CLI/admin diagnostics, and validation fixtures.
7. Harden encrypted-index leakage handling and degraded-key/index behavior before broad native app use.

## Handoff And Downstream Use

Encrypted Docdex RAG Adapter hands context plans, authorized retrieval result refs, context bundle refs, leakage profiles, context grants, denial reason codes, usage refs, and replay bundles to Personal AI Assistant, AI Gateway Router, Central AI Service, Codali Adapter, Docdex Adapter, native apps, Wallet/Usage Center, Overwatch, SDK, CLI, and admin UI.

Downstream services must consume context through bundle/grant refs and must not directly open Docdex indexes or private storage.

## Open Design Questions

Resolved decisions:

- The first Phase 6 product integration allows only owner-scoped, retrieval-only encrypted-index profiles needed to prove Docdex indexing, search, retrieval, and model-routing support. The allowed set is `encrypted_metadata_only` for key-gated retrieval with no plaintext term index, plus `encrypted_term_index_limited` only when the owner or tenant explicitly accepts term-presence, visible-path class, token-count class, result-count, and capability leakage. Phase 6 does not allow broad full-open encrypted repository access, generic web/semantic expansion outside the registered index scope, unauthorized structural code intelligence, raw plaintext index export, or cross-person/workspace/org context widening. Any plaintext term-index use must be visible to the AI Gateway Router, Personal AI Assistant, user privacy UI, replay records, and Overwatch evidence as an explicit leakage profile, not hidden as an implementation detail.
- Personal assistant sessions use route/request-bound grants by default, with reusable time-bounded grants allowed only for a specific owner scope, purpose, app/session, data class, leakage profile, result cap, retention class, and allowed tool/native-service use. Per-turn or explicit confirmation is required when a request would widen scope, include a new repo/workspace/person/org, raise result or snippet limits, change leakage profile, cross tenant or organization boundaries, touch regulated/secret-bearing/fraud-sensitive content, invoke a side-effecting tool, or use Central AI/stewardship handoff. Reusable grants must remain revocable from Wallet/Usage Center or assistant controls and cannot survive key revocation, access-binding changes, or policy denial.
- User-visible retrieval metadata is audience-classed. The requesting owner can see source aliases, owner scope, authorized repo/workspace labels, data class, purpose, leakage profile, query hash or user-entered query ref, coarse result-count and size classes, selected citation refs, snippet hashes, redaction status, key/capability status, denial reason codes, usage refs, and expiry. Users must not see private repo/workspace tree structure they are not authorized to view, raw snippets outside an authorized context bundle, exact private paths where policy exposes only classes, raw token counts where classes are enough, other users' queries, vault refs, secret/key material, hidden fraud/policy heuristics, or admin-only Docdex instance details. Public or operator views receive aggregates, refs, hashes, and reason-code summaries unless an owning-service policy grants stronger visibility.
- A context bundle created only for one AI route is a route-bound ephemeral artifact. It remains readable only until the route reaches a terminal state plus the bounded retry/fallback window, and its raw snippet payload expires no later than 24 hours after creation unless the owner explicitly pins it for support, dispute, incident, legal, compliance, or user-requested history. After payload expiry, replay keeps only bundle id, hashes, citation/result refs, leakage profile, redaction plan, route refs, reason codes, usage refs, and Overwatch evidence under the AI Gateway/Overwatch classed retention pattern: caller-visible summaries for 30 days, private replay details for 7 days unless pinned, redacted mismatch or rollout evidence for 90 days, then long-term hash/id/version/reason-code archives.
- Immediate grant revocation is required when a Docdex capability snapshot shows key revocation, rotation, missing or invalid key material, repo binding suspension/deprovision, owner or access-binding changes, tenant/workspace/repo membership changes, encryption-mode changes, plaintext term indexing newly enabled, visible-path/token/result metadata leakage widened, semantic/web/full-open or structural AST/symbol/impact features newly enabled for an encrypted repo, result/snippet caps increased beyond the grant, redaction or retention guarantees weakened, admin-ingest plaintext cleanup failure, Docdex instance compromise or untrusted relocation, or Overguard/Overvault policy denial. Purely narrowing changes may mark a grant degraded instead of owner-wide revoked, but any active route must reauthorize before using the changed capability snapshot.
