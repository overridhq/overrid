# SUB BUILD PLAN #61 - Encrypted Docdex RAG Adapter

Attached SDS: [docs/sds/ai_rag_model_routing/encrypted_docdex_rag_adapter.md](../sds/ai_rag_model_routing/encrypted_docdex_rag_adapter.md)

## Purpose

This sub-build plan turns SDS #61 into an implementation sequence for Encrypted Docdex RAG Adapter. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Encrypted Docdex RAG Adapter is the Phase 6 authorization and context-assembly boundary between encrypted Docdex indexes and Overrid AI flows, with Phase 12 expansion for personal AI, workspace, search, code-agent, and native-app context use and Phase 13 hardening for leakage, revocation, privacy, replay, incident, retention, and compliance controls. It owns index authorization records, context-scope manifests, retrieval requests and result refs, leakage profiles, context grants, redaction profiles, context bundle refs, RAG usage refs, audit refs, and replay bundles. It does not run Docdex itself, decrypt private repositories for routers, choose final models, run inference, bypass Overvault or Overguard, expose raw private content, or hide encrypted-index leakage from users, routers, replay, or audit.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #61: Encrypted Docdex RAG Adapter](../sds/ai_rag_model_routing/encrypted_docdex_rag_adapter.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Encrypted Docdex RAG Adapter plan](../service_catalog/ai_rag_model_routing/encrypted_docdex_rag_adapter.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Policy Dry-Run previews, Challenge Task evidence, Oververify facts, and deny-by-default behavior. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill accounting handoffs, and the rule that RAG usage is structural and auditable without hardcoded pricing. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Controls the first build point for Docdex encrypted RAG jobs, retrieval, context authorization, AI Gateway Router route metadata, usage, and product proof. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, namespace, private refs, protected content refs, encrypted context refs, redaction boundaries, and metadata-only replay substrates. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Expands the adapter into personal AI, workspace, search, messaging, code-agent, mobile, and native-app context flows through bounded context grants and bundles. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies leakage threat models, grant-revocation hardening, privacy/compliance retention, incident response, security review, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #61 first build work aligned to master Phase 6, with Phase 12 native-app expansion and Phase 13 governance/security/compliance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, raw-private-content, hidden-leakage, or final-model-selection drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 12, and 13 | Attach SDS #61, preserve Phase 6 as first build, record prerequisites, and freeze context-authorization boundaries. |
| 2 | Master Phases 0, 1, 4, 5, 6, and 8 | Define Rust contracts, canonical schemas, state machines, stable errors, signed refs, hashes, fixtures, and redaction-safe examples. |
| 3 | Master Phases 1, 6, and 8 | Implement encrypted index ref registration, Docdex capability snapshots, owner/access bindings, and leakage profiles. |
| 4 | Master Phases 1, 4, 6, and 8 | Implement dry-run scope evaluation, context grants, key checks, policy checks, and fail-closed denial behavior. |
| 5 | Master Phases 4, 5, 6, and 8 | Execute bounded retrieval through Docdex Adapter, capture result refs, redaction status, usage refs, and replay evidence. |
| 6 | Master Phases 5, 6, 8, and 12 | Create route-bound context bundles, retention rules, redacted projections, and grant revocation behavior. |
| 7 | Master Phases 6 and 12 | Integrate AI Gateway Router, Personal AI Assistant, Central AI Service, Codali Adapter, Docdex Adapter, and native apps without bypassing context grants. |
| 8 | Master Phases 1, 5, 6, 8, 12, and 13 | Implement usage, audit, replay, observability, SDK/CLI/admin diagnostics, and support views. |
| 9 | Master Phase 13, with prerequisites from Phases 0 through 12 | Harden leakage, key/index degradation, grant revocation, incident handling, privacy, retention, and scale behavior. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Encrypted Docdex RAG Adapter core is a Rust service/module using shared contract crates, Tokio for bounded retrieval/redaction/replay workers, and Axum/Tower/Hyper-style HTTP only where a service boundary is needed.
- Index refs, context scope manifests, retrieval requests/results, context grants, leakage profiles, redaction profiles, context bundle refs, usage refs, audit records, events, fixtures, replay bundles, and diagnostics use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant and owner scope, trace id, idempotency key, data-class refs, purpose refs, key refs, policy refs, context grant refs where applicable, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for index capability snapshots, retrieval requests, snippet hashes, context bundle snapshots, redaction plans, leakage profiles, replay bundles, audit exports, and deterministic fixtures.
- Storage, queueing, private records, object refs, namespace, audit, usage, policy, identity, keys, and accounting handoffs must use native Overrid service boundaries such as Overbase, Overqueue, Overstore, Overvault, Universal Namespace Service, Overpass, Overtenant, Overkey, Overwatch, Overmeter, Overguard, ORU Account Service, and Seal Ledger.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw private content in logs/events, hidden plaintext term-index leakage, direct Docdex runtime ownership, final model selection, or model inference the adapter boundary.

## Phase 1: SDS Attachment, Phase 6 Scope, And Privacy Boundary

### Work Items

- **1.1 Attach the build plan to SDS #61.**
  - Design: Link this document from the Encrypted Docdex RAG Adapter SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/ai_rag_model_routing/encrypted_docdex_rag_adapter.md`, `docs/service_catalog/ai_rag_model_routing/encrypted_docdex_rag_adapter.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #61 returns both the Encrypted Docdex RAG Adapter SDS and this sub-build plan.

- **1.2 Preserve master Phase 6 as the first build point.**
  - Design: Keep first implementation in Phase 6 because Docdex encrypted indexing/search/retrieval and AI Gateway Router context plans are the first product proof for private RAG workloads.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, and 8 supply contracts, identity, policy, metering, and protected refs; Phase 6 builds the product RAG adapter proof; Phase 12 expands native-app use; and Phase 13 hardens governance/security/compliance.
  - Validation: Review proves the plan does not move the adapter into Phase 12-only native app work, does not delay the adapter to Phase 13, and does not reorder the master Phase 0 through Phase 13 sequence.

- **1.3 Freeze the adapter ownership boundary.**
  - Design: Record that the adapter owns context authorization, scope manifests, retrieval requests/result refs, leakage profiles, context grants, context bundles, usage refs, audit refs, and replay evidence.
  - Output: Ownership checklist for architecture, API, and implementation reviews.
  - Validation: Review confirms the adapter does not implement Docdex indexing/search internals, choose final models, run inference, persist raw key material, mutate accounting state, or bypass policy/key/tenant checks.

- **1.4 Carry forward resolved leakage and grant decisions.**
  - Design: Preserve the SDS decisions for `encrypted_metadata_only`, explicitly accepted `encrypted_term_index_limited`, route/request-bound grants, reusable grant narrowing, audience-classed retrieval metadata, ephemeral route-bound bundles, and immediate revocation triggers.
  - Output: Resolved-decision checklist covering accepted leakage profiles, grant scope widening, visible metadata, route-bound retention, capability-snapshot changes, and owner/user revocation.
  - Validation: Review rejects hidden term-index leakage, cross-owner context widening, reusable grants without expiry/scope/purpose/data-class/leakage constraints, and route bundle payloads retained beyond allowed policy.

- **1.5 Define downstream consumer boundaries.**
  - Design: Record how AI Gateway Router, Personal AI Assistant, Central AI Service, Codali Adapter, Docdex Adapter, workspace/search/messaging/native apps, Wallet/Usage Center, SDK, CLI, and admin UI consume context through refs instead of raw index access.
  - Output: Consumer-boundary matrix naming allowed inputs, denied direct reads, expected output refs, usage refs, and replay requirements.
  - Validation: Review confirms downstream services use context bundle/grant refs and cannot directly open Docdex indexes, Overvault private records, Overstore private payloads, or unauthorized workspace/repo paths.

## Phase 2: Rust Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the Rust contract module.**
  - Design: Add contract types for encrypted index ref, context scope manifest, retrieval request, retrieval result ref, context bundle ref, context grant, leakage profile, redaction profile, RAG usage ref, audit record, event payload, replay bundle, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, denial reason enums, operation-class enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Docdex runtime internals, AI Gateway route selection, model inference, and private storage internals.

- **2.2 Define index ref and capability snapshot schemas.**
  - Design: Model `encrypted_index_ref` and capability snapshots with Docdex repo/index refs, owner scope, tenant/person/org/workspace/repo refs, encryption mode, key ref, access bindings, leakage profile, last indexed time, disabled feature flags, and capability version.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and canonical fixture snapshots.
  - Validation: Schema tests reject missing owner scope, missing tenant, missing encryption mode, missing leakage profile, missing key status, unversioned capability snapshots, and raw key material.

- **2.3 Define retrieval, grant, and context bundle schemas.**
  - Design: Model `context_scope_manifest`, `retrieval_request`, `retrieval_result_ref`, `context_grant`, and `context_bundle_ref` with purpose refs, data classes, result caps, snippet caps, redaction profile, retention class, allowed clients/tools, denied scopes, expiry, revocation state, and route refs.
  - Output: Schema set, lifecycle rules, denial examples, redacted view examples, idempotency fixtures, route-bound bundle fixtures, and grant-revocation fixtures.
  - Validation: Tests reject requests without actor/tenant/owner/purpose/data-class checks, grants without expiry or allowed use, bundles without retention class, and results with raw unauthorized content.

- **2.4 Define leakage, usage, audit, and replay schemas.**
  - Design: Model `encrypted_index_leakage_profile`, `rag_usage_ref`, `rag_audit_record`, and replay bundles with plaintext term-index status, visible path/token/result classes, disabled operations, semantic/web/full-open flags, operation counters, usage refs, policy refs, key refs, denied scopes, and replay hashes.
  - Output: Leakage profile schema, usage schema, audit schema, replay schema, stable error catalog, redacted examples, and negative fixtures.
  - Validation: Tests prove plaintext term-index leakage cannot be omitted, disabled structural features cannot be advertised as available, and replay can reconstruct decisions without raw private payloads.

- **2.5 Create deterministic fixtures.**
  - Design: Build fixtures for valid index registration, missing key ref, term-index leakage denied, accepted limited leakage, valid dry-run, denied scope, valid retrieval, unauthorized result discard, redaction failure, bundle creation, grant revocation, stale index, Docdex Adapter unavailable, usage reconciliation, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, stable errors, BLAKE3 hashes, redacted projections, usage refs, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, denial codes, audit refs, redacted outputs, usage refs, revocation effects, and replay outputs across repeated runs.

## Phase 3: Index Ref Registry, Capability Snapshots, And Leakage Profiles

### Work Items

- **3.1 Implement encrypted index ref registration.**
  - Design: Add `POST /docdex-rag/index-refs` with signed envelope checks, actor/service identity, tenant/owner scope, Docdex repo/index refs, encryption mode, key refs, access bindings, idempotency, trace id, and Overwatch audit refs.
  - Output: Request handler, index ref records, idempotency behavior, stable errors, and `encrypted_docdex_rag_adapter.index_ref_registered` events.
  - Validation: API tests cover valid registration, duplicate idempotency keys, missing owner scope, missing tenant, missing key ref, malformed index refs, and audience-safe errors.

- **3.2 Implement index ref reads and projections.**
  - Design: Add `GET /docdex-rag/index-refs/{index_ref_id}` with role-scoped views for owner, app, router, steward/operator, and support contexts.
  - Output: Read handler, redacted projections, owner-visible metadata, router-visible capability/leakage facts, and admin diagnostics without raw private content or key material.
  - Validation: Tests prove callers cannot see unauthorized repo/workspace tree details, exact private paths where only classes are allowed, other users' refs, raw snippets, vault refs, or key material.

- **3.3 Capture Docdex Adapter capability snapshots.**
  - Design: Integrate with Docdex Adapter for repo binding, index/search/retrieval capabilities, encrypted-search compatibility, disabled structural features, stale-index status, and service-admin ingest capabilities.
  - Output: Capability snapshot records, versioned capability refs, degraded capability states, snapshot hashes, and `encrypted_docdex_rag_adapter.leakage_profile_changed` events when relevant.
  - Validation: Tests prove stale, degraded, missing, or widened capability snapshots cannot silently preserve stronger grants and active routes must reauthorize before using changed facts.

- **3.4 Implement leakage profile derivation.**
  - Design: Derive explicit leakage profiles for metadata-only encrypted retrieval, limited plaintext term indexing, visible path classes, token-count/result-count classes, unavailable semantic/full-open/structural operations, and residual leakage notes.
  - Output: Leakage evaluator, profile records, policy inputs, router-facing summaries, user-facing summaries, and replay refs.
  - Validation: Tests prove term-presence leakage is visible to AI Gateway Router, Personal AI Assistant, user privacy UI, Overwatch, and replay rather than hidden as implementation detail.

- **3.5 Implement index ref lifecycle.**
  - Design: Track index refs through `proposed`, `owner_verified`, `key_checked`, `capability_loaded`, `leakage_profiled`, `active`, `degraded`, `revoked`, and `archived`.
  - Output: Lifecycle state machine, transition rules, degraded-state reason codes, revocation records, archived projections, and event payloads.
  - Validation: Tests prove invalid transitions are rejected, revoked refs cannot start retrieval, degraded refs require explicit policy handling, and archived refs remain replayable without reopening content.

## Phase 4: Dry-Run Scope Evaluation, Context Grants, And Fail-Closed Policy

### Work Items

- **4.1 Implement retrieval dry-run.**
  - Design: Add `POST /docdex-rag/retrievals/dry-run` to preview allowed scopes, denied scopes, leakage facts, key status, policy reasons, result limits, expected usage class, and retention constraints without retrieving content.
  - Output: Dry-run handler, scope evaluator, denied-scope records, leakage acceptance prompts, expected usage refs, and `encrypted_docdex_rag_adapter.scope_evaluated` events.
  - Validation: Tests prove dry-run never retrieves raw snippets, exposes only authorized metadata, and returns stable reason codes for scope, key, policy, leakage, and result-limit denials.

- **4.2 Implement actor, tenant, and owner-scope checks.**
  - Design: Validate Overpass actor refs, Overtenant scope, owner/person/org/workspace/repo bindings, service identity, allowed client, allowed tool/native-service use, and route/session context.
  - Output: Scope-check adapter, owner-binding records, caller matrix, denied-scope projections, and replay refs.
  - Validation: Tests prove cross-person, cross-workspace, cross-org, cross-repo, cross-tenant, and unauthorized service requests deny before Docdex Adapter calls.

- **4.3 Implement policy and data-class checks.**
  - Design: Use Overguard, Workload Classifier, Policy Dry-Run, purpose refs, data-class refs, regulated/secret/fraud-sensitive flags, and caller app policy before allowing retrieval or grant creation.
  - Output: Policy-check adapter, data-class inputs, policy refs, review-required outcomes, and stable `policy_denied` behavior.
  - Validation: Tests prove fallback cannot widen context scope, weaken privacy, skip review-required classes, or convert denied retrievals into degraded success.

- **4.4 Implement key and grant checks.**
  - Design: Use Overvault and Overkey refs to validate key status, key availability, key rotation/revocation, access-binding freshness, context grant scope, expiry, allowed use, and revocation state without persisting key material.
  - Output: Key/grant checker, grant status records, revocation awareness, key-failure reason codes, and audit refs.
  - Validation: Tests prove missing/unavailable/revoked keys deny retrieval, grants cannot survive key or access-binding changes, and raw key material never appears in durable adapter records.

- **4.5 Implement context grant creation and renewal.**
  - Design: Add `POST /docdex-rag/context-grants` for route-bound grants by default and reusable time-bounded grants only when scope, purpose, app/session, data class, leakage profile, result cap, retention class, and allowed tool/native-service use are explicit.
  - Output: Grant writer, renewal/narrowing rules, widening confirmation requirements, user-visible grant metadata, and audit events.
  - Validation: Tests prove widening a grant requires explicit confirmation or policy authority, reusable grants remain revocable, and grant renewal cannot hide changed leakage or capability facts.

## Phase 5: Bounded Retrieval Through Docdex Adapter

### Work Items

- **5.1 Implement retrieval request acceptance.**
  - Design: Add `POST /docdex-rag/retrievals` to accept signed, tenant-scoped, grant-aware requests with query hashes, requested scopes, task class, privacy mode, purpose refs, trace id, idempotency key, and policy refs.
  - Output: Retrieval request records, idempotent acceptance, `encrypted_docdex_rag_adapter.retrieval_requested` events, and initial lifecycle state.
  - Validation: API tests cover valid requests, duplicate idempotency, missing grant, missing purpose, missing data class, missing trace id, and malformed query metadata.

- **5.2 Implement scope narrowing before retrieval.**
  - Design: Narrow each request to allowed indexes, paths, document classes, result count, snippet size, redaction profile, retention class, allowed clients, and denied scopes before calling Docdex Adapter.
  - Output: Narrowed retrieval plan, denied-scope records, redaction requirements, result caps, and replay refs.
  - Validation: Tests prove broad requests are narrowed or denied, denied scopes are recorded, and no Docdex Adapter call receives unauthorized paths or owner refs.

- **5.3 Integrate Docdex Adapter retrieval-only calls.**
  - Design: Call Docdex Adapter for retrieval-only search/RAG work using capability snapshot refs, authorized index refs, bounded query metadata, result caps, and retry/degradation policy.
  - Output: Adapter client, job/search refs, retrieval state transitions, retry records, degraded-state handling, and `docdex_adapter_unavailable` behavior.
  - Validation: Tests prove the adapter does not implement Docdex internals, does not bypass Docdex Adapter, handles unavailable/stale indexes safely, and preserves retry/failure reasons.

- **5.4 Capture retrieval result refs.**
  - Design: Store result refs, citation refs, snippet hashes, result count classes, redaction status, leakage profile ref, retrieval timestamp, and Docdex job/search ref without durable raw private content.
  - Output: `retrieval_result_ref` records, redacted reads, hashable result snapshots, and `encrypted_docdex_rag_adapter.retrieval_completed` events.
  - Validation: Tests prove unauthorized result scope is discarded, result refs remain replayable, raw private snippets do not appear in logs/events/metrics, and result count/token details are classed where policy requires.

- **5.5 Implement retrieval lifecycle and failure handling.**
  - Design: Track retrievals through `submitted`, `scope_checked`, `key_checked`, `leakage_checked`, `policy_checked`, `retrieving`, `redacting`, `bundle_created`, `completed`, `denied`, `failed`, and `cancelled`.
  - Output: Retrieval state machine, terminal reason codes, retry/failure rules, cancellation behavior, and audit refs.
  - Validation: Tests prove invalid transitions are rejected, policy/key/leakage denials fail closed, redaction failures do not create bundles, and usage reconciliation records failed/denied attempts correctly.

## Phase 6: Context Bundles, Redaction, Retention, And Revocation

### Work Items

- **6.1 Implement context bundle creation.**
  - Design: Add `POST /docdex-rag/context-bundles` to assemble selected result refs into bounded context payload refs with redaction plan, context-window budget, prompt-inclusion rules, expiry, retention class, route refs, and usage refs.
  - Output: Bundle writer, Overstore payload refs, selected-result refs, bundle metadata, and `encrypted_docdex_rag_adapter.context_bundle_created` events.
  - Validation: Tests prove bundles require authorized retrieval results, cannot include denied scopes, and cannot be created when redaction, key, policy, or grant state is invalid.

- **6.2 Implement redaction and truncation.**
  - Design: Apply redaction profile, snippet caps, result caps, citation refs, data-class rules, owner/app route rules, and audience-classed projection before bundle payload creation.
  - Output: Redaction worker, truncation records, snippet hashes, citation refs, redaction status, and failure reason codes.
  - Validation: Tests prove redaction failure blocks bundle creation, raw unauthorized snippets never appear in public/router/admin views, and citation/hash refs remain sufficient for replay.

- **6.3 Implement route-bound retention.**
  - Design: Treat a context bundle created for one AI route as a route-bound ephemeral artifact with readability limited to the terminal route state plus bounded retry/fallback window and payload expiry no later than policy permits.
  - Output: Retention policy config, expiry worker, payload tombstones, retained metadata refs, and replay-safe archives.
  - Validation: Tests prove route payloads expire, pinned exceptions require allowed support/dispute/incident/legal/compliance/user-history policy, and long-term archives reduce to hashes/ids/versions/reason codes.

- **6.4 Implement bundle reads and projections.**
  - Design: Add `GET /docdex-rag/context-bundles/{bundle_id}` with caller permission checks, grant status, route/session state, retention policy, redacted metadata, and payload ref access only where still allowed.
  - Output: Bundle read handler, role-scoped projections, expired/revoked behavior, and access audit refs.
  - Validation: Tests prove expired or revoked bundles do not return payload refs, unauthorized callers receive stable denials, and metadata views never reveal raw private content.

- **6.5 Implement grant revocation and downstream invalidation.**
  - Design: Add `POST /docdex-rag/context-grants/{grant_id}/revoke` and automatic revocation triggers for key revocation, owner/access-binding changes, tenant/repo membership changes, encryption-mode changes, widened leakage, cap increases, weakened redaction/retention, Docdex instance compromise, and policy denial.
  - Output: Revocation handler, revocation events, route invalidation refs, degraded-grant rules for narrowing changes, and downstream notification records.
  - Validation: Tests prove revoked grants block future access, active routes reauthorize before using changed snapshots, and `encrypted_docdex_rag_adapter.context_grant_revoked` reaches downstream consumers.

## Phase 7: AI Gateway, Personal AI, Central AI, Codali, And Native App Integrations

### Work Items

- **7.1 Integrate AI Gateway Router context plans.**
  - Design: Provide AI Gateway Router with allowed/denied context scopes, leakage profiles, context bundle refs, grant refs, route/session constraints, result caps, privacy facts, and replay refs before model/resource selection.
  - Output: Router-facing context plan contract, dry-run bridge, route metadata refs, fallback constraints, and denial behavior.
  - Validation: Tests prove AI Gateway Router cannot read indexes directly, cannot hide leakage facts, cannot select a route that requires denied context, and cannot weaken grant/privacy constraints during fallback.

- **7.2 Integrate Personal AI Assistant permissions.**
  - Design: Support assistant sessions with route/request-bound grants by default, user-visible retrieval metadata, explicit confirmation for scope widening, and revocation from Wallet/Usage Center or assistant controls.
  - Output: Assistant grant flow, permission prompts, user-visible retrieval metadata, revocation hooks, and privacy-audit refs.
  - Validation: Tests prove assistant sessions do not reuse grants across new repos/workspaces/person/org scopes without confirmation and do not expose raw snippets outside authorized bundles.

- **7.3 Integrate Central AI Service evidence context.**
  - Design: Let Central AI Service request evidence-bounded context only where policy permits, with strict data-class/purpose/review refs and no direct private-content ingestion.
  - Output: Central AI context contract, denied-context records, evidence-analysis bundle refs, and review/audit refs.
  - Validation: Tests prove Central AI cannot bypass owner permissions, ingest private context without authorization, hide denied context, or use bundle refs beyond retention/review rules.

- **7.4 Integrate Codali and code-agent repository context.**
  - Design: Provide Codali Adapter with repository context refs, citation refs, scope manifests, disabled encrypted-repo structural feature flags, result caps, logs/artifact refs, and replay constraints.
  - Output: Code-agent context contract, repo-context grant rules, disabled-feature propagation, result/ref mapping, and usage refs.
  - Validation: Tests prove code agents cannot access unauthorized repositories, cannot simulate disabled structural AST/symbol/impact features from unauthorized plaintext, and cannot leak private paths outside allowed projections.

- **7.5 Integrate workspace, search, messaging, mobile, SDK, CLI, and admin consumers.**
  - Design: Provide ordinary Overrid client flows for workspace AI, search, messaging triage, mobile sync/assistant use, SDK commands, CLI diagnostics, and admin troubleshooting through the same bundle/grant/ref contracts.
  - Output: Consumer contracts, generated bindings, CLI/admin command shapes, role-scoped projections, and operator docs.
  - Validation: Contract tests prove each consumer uses Overgate/Overpass/Overtenant/Overkey/Overguard/Overvault/Overwatch/Overmeter rails and never receives private context through side channels.

## Phase 8: Usage, Audit, Replay, Observability, And Diagnostics

### Work Items

- **8.1 Emit RAG usage refs.**
  - Design: Emit usage refs for index ref registration, dry-run checks, retrieval calls, result count classes, context assembly, redaction, bundle storage, retries, failed attempts, and denied attempts.
  - Output: Usage writer, operation classes, actor/tenant/owner/caller/index/retrieval/route/bundle refs, reconciliation records, and Overmeter handoff.
  - Validation: Tests prove successful, failed, denied, retried, redacted, and bundle-storage operations produce structural usage refs without hardcoded prices or provider payout logic.

- **8.2 Implement audit and replay APIs.**
  - Design: Add `GET /docdex-rag/replay/{record_id}` to reconstruct authorization, key, leakage, policy, retrieval, redaction, bundle, grant, revocation, usage, and downstream route decisions.
  - Output: Replay API, role-scoped replay bundles, pagination for linked records, audit export hooks, and deterministic replay fixtures.
  - Validation: Tests prove replay is deterministic, privacy bounded, can explain allowed/denied decisions, and does not fetch raw private payloads after expiry.

- **8.3 Implement observability.**
  - Design: Expose counts and latency for scope checks, retrievals, denials, redactions, bundle creation, grant revocations, Docdex Adapter failures, key failures, stale indexes, leakage-profile changes, and replay reads.
  - Output: Metrics, tracing spans, alert rules, dashboard projections, operator views, and Overwatch event links.
  - Validation: Tests prove metrics are role-scoped, alerts fire for leakage widening/key failure/revocation mismatch/private-context leakage attempts, and dashboards avoid private payload leakage.

- **8.4 Implement SDK, CLI, and admin diagnostics.**
  - Design: Provide typed SDK/CLI/admin commands for index refs, capability snapshots, leakage profiles, dry-runs, retrieval status, grants, revocations, bundle metadata, replay exports, and degraded encrypted indexes.
  - Output: Command contracts, stable JSON output, generated bindings, signed envelope examples, idempotency behavior, and operator docs.
  - Validation: Contract tests prove commands require identity/tenant/owner scope refs, produce stable JSON, preserve idempotency, and match service API authority boundaries.

- **8.5 Implement usage reconciliation and support views.**
  - Design: Reconcile usage emission failures, denied retrieval low-cost policy/audit usage, stale retrieval/bundle refs, support review refs, and private replay retention without storing raw private content unnecessarily.
  - Output: Reconciliation worker, support-safe projections, pending usage records, retry/dead-letter behavior, and audit refs.
  - Validation: Tests prove terminal completion waits for usage reconciliation where required, denied retrievals are not recorded as successful context retrieval, and support views remain redacted.

## Phase 9: Leakage, Revocation, Security, Incident, And Scale Hardening

### Work Items

- **9.1 Harden encrypted-index leakage handling.**
  - Design: Threat-model plaintext term indexes, visible path classes, token/result count leakage, disabled structural features, semantic/web/full-open capability flags, query-hash inference, and metadata correlation.
  - Output: Leakage threat model, mitigation checklist, policy gates, redacted user/router/admin summaries, and abuse-case fixtures.
  - Validation: Security tests prove leakage widening blocks or reauthorizes grants, user/router/replay views show leakage class, and hidden leakage cannot pass validation.

- **9.2 Harden key, grant, and capability revocation.**
  - Design: Enforce immediate revocation or reauthorization when key material, repo binding, owner/access binding, tenant membership, encryption mode, leakage profile, capability flags, result caps, redaction profile, retention guarantees, or Docdex trust status changes.
  - Output: Revocation detector, active-route invalidation, degraded-grant rules, owner notifications, downstream event records, and replay evidence.
  - Validation: Tests prove active routes cannot reuse changed grants, narrowing changes can degrade only where safe, and widened capabilities or weakened privacy always force reauthorization.

- **9.3 Harden privacy, compliance, and retention.**
  - Design: Apply audience-class retention for owner-visible summaries, private replay details, route-bound payload expiry, support/dispute/incident/legal/compliance pins, deletion/redaction rules, and long-term hash/id/version archives.
  - Output: Retention policy config, legal/compliance hold refs, redaction tombstones, archive compaction rules, and audit export fixtures.
  - Validation: Tests prove payload expiry follows policy, pins require allowed reason classes, deletion does not break audit chains, and public/operator archives reduce to safe refs.

- **9.4 Harden incident and abuse response.**
  - Design: Add incident playbooks for unauthorized context leakage, wrong-scope retrieval, leaked snippets in logs/events, Docdex instance compromise, grant revocation failure, key failure, redaction failure, and router misuse of context.
  - Output: Incident playbook refs, escalation records, route freeze behavior, affected-owner notification refs, correction/retraction workflows, and post-incident report refs.
  - Validation: Drills prove each incident path creates audit evidence, freezes risky grants/routes, notifies owning services/users where policy requires, and preserves replay/correction paths.

- **9.5 Harden reliability and scale behavior.**
  - Design: Add bounded concurrency, queue depth controls, retry/dead-letter behavior, bundle expiry batches, replay pagination, backpressure, load shedding, degraded-mode reads, stale-index refresh behavior, and Docdex Adapter failover rules.
  - Output: Reliability config, worker limits, retry/dead-letter records, backpressure metrics, load test fixtures, and scale dashboards.
  - Validation: Load tests prove the adapter preserves idempotency, does not drop audit/usage evidence, degrades by denying or queuing sensitive retrieval rather than bypassing policy, and handles native-app review spikes safely.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, 10 phase headings numbered 1 through 10, five work items per phase, Design/Output/Validation blocks, local links, final newline, and tab-free Markdown.
  - Output: Targeted validation script output and recorded evidence in `docs/build_plan/progress.md`.
  - Validation: Script passes for this file and the linked SDS/service/build-plan docs.

- **10.2 Validate documentation alignment.**
  - Design: Confirm SDS #61, the service catalog plan, master build plan, service catalog alignment crosswalk, Phase 6, Phase 12, Phase 13, and tech-stack decision all preserve the same first build phase, native expansion, hardening gates, ownership boundaries, and non-goals.
  - Output: Alignment review notes and updated backlinks/index rows.
  - Validation: Review confirms there is no master Phase 0 through Phase 13 reorder, no Phase 12-only or Phase 13-only first-build drift, and no missing sub-build-plan link.

- **10.3 Validate stack and privacy guardrails.**
  - Design: Scan the changed docs for conventional database, queue, object store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, customer-count, raw private content, hidden leakage, final model selection, model inference, and direct Docdex runtime ownership drift.
  - Output: Guardrail scan evidence with only expected negative-control references and native Overrid service-name references.
  - Validation: Review confirms Encrypted Docdex RAG Adapter remains Rust-first, native-Overrid-boundary-based, authorization-scoped, privacy-bounded, leakage-explicit, usage-metered, and replayable.

- **10.4 Refresh Docdex, queue, and progress state.**
  - Design: Update the Codex55 queue state/progress for `061-build-plan`, append build-plan progress evidence, index changed docs, and search for the new plan/backlinks.
  - Output: `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, `docs/build_plan/progress.md`, Docdex index refresh, Docdex search results, and DAG/session evidence where available.
  - Validation: JSON validation passes, queue counts advance from SDS #61 to SDS #62, Docdex search returns the new #61 plan with SDS/service backlinks, and no unrelated queue tasks are changed.

- **10.5 Prepare implementation handoff.**
  - Design: Summarize the ordered implementation entry points, required prerequisites, first usable Phase 6 proof, Phase 12 expansion gates, Phase 13 hardening gates, test expectations, and privacy/authorization boundaries for builders.
  - Output: Implementation handoff checklist for contracts, index refs, dry-run scope checks, retrieval execution, bundles/grants, integrations, usage/audit/replay, leakage/revocation hardening, and validation.
  - Validation: Handoff review confirms a builder can start with contracts and index refs, prove retrieval dry-run/bounded retrieval/context bundles in Phase 6, expand native consumers in Phase 12, and avoid raw private-content or hidden-leakage drift.

## Alignment Review

- SDS #61 remains attached to master Phase 6 as the first build point for encrypted Docdex RAG product integration, retrieval authorization, context grants, context bundles, leakage profiles, usage refs, audit refs, and replay bundles.
- Phase 12 remains native-app expansion for personal AI, workspace, search, messaging, code-agent, and mobile use through the same context grant and bundle contracts.
- Phase 13 remains governance, compliance, security, incident, retention, leakage, revocation, and scale hardening rather than the first delivery point.
- Phases 0, 1, 4, 5, and 8 remain prerequisites; this plan does not reorder master Phase 0 through Phase 13.
- The SDS/service docs only need backlink and Phase 13 alignment wording corrections; the SDS content itself already preserves the correct authorization-scoped, leakage-explicit, no-raw-private-content boundary.
- The master plan and service catalog alignment crosswalk need a new SDS #61 row and a Phase 12 crosswalk expansion entry so Encrypted Docdex RAG Adapter is explicitly tied to both Phase 6 product proof and Phase 12 native AI consumers.
