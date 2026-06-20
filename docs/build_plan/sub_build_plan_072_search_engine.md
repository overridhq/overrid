# SUB BUILD PLAN #72 - Search Engine

Attached SDS: [SDS #72 - Search Engine](../sds/native_apps/search_engine.md)

## Purpose

This sub-build plan turns SDS #72 into an implementation sequence for Search Engine. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Search Engine is the Phase 12 native discovery utility for public Overrid content, Directory Listings, public app pages, public-interest datasets, workspace/private content where explicitly authorized, result handoffs, and Personal AI Assistant search tool calls. It owns source registrations, source policies, crawl/index jobs, index records, permission filter snapshots, query sessions, result sets, ranking explanations, removal/tombstone refs, abuse/spam refs, usage refs, audit refs, and replay projections. It does not own canonical source content, app state, message bodies, workspace documents, private vault material, final moderation decisions, final fraud or reputation verdicts, balance truth, paid ranking, ad auctions, or provider payouts.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #72: Search Engine](../sds/native_apps/search_engine.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoffs, and resolved open-question decisions. |
| [Search Engine service plan](../service_catalog/native_apps/search_engine.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency keys, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Overclaim disputes, Fraud Control evidence, Challenge Task facts, Reputation/Anti-Sybil recommendations, and deny-by-default behavior. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill accounting truth, Wallet/Usage Center receipt refs, and the rule that Search emits usage refs but never mutates balances, pricing, paid placement, or payment records. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies AI/RAG, adapter, product, and runtime-bridge groundwork used when Personal AI Assistant or approved native apps issue explicit search tool calls or receive bounded result refs. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase lexical/document/secondary/vector indexes, Overstore chunk/artifact refs, Overvault private grants, Universal Namespace refs, retention, backup/restore, and replay substrates. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider fraud, reputation, anti-Sybil, challenge, appeal, abuse-control, and throttling prerequisites for public-source indexing, spam controls, and ranking-abuse handling. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first build point for Search Engine and its first useful source registration, indexing, query, permission-filter, explanation, handoff, usage, audit, and replay slice. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal search privacy, ranking-abuse, source-poisoning, retention, compliance, threat review, incident response, public reporting, reliability, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #72 first build work aligned to master Phase 12, with Phase 13 as governance/security/compliance/reporting hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and contracts, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where a service boundary exists, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript for client surfaces only, native Overrid service boundaries, and no conventional database/search engine, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, ad auction, paid ranking, pricing, revenue, customer-count, surveillance ranking, raw private-content indexing, or private-data bypass drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 11, 12, and 13 | Attach SDS #72, preserve Phase 12 as first build, record prerequisites, and freeze Search ownership boundaries. |
| 2 | Master Phases 0, 1, 4, 8, and 12 | Define Rust contracts, canonical schemas, lifecycle enums, event surfaces, stable errors, and deterministic fixtures. |
| 3 | Master Phases 1, 4, 8, and 12 | Implement source registration, source policy, source authority, crawl/index job, and source-owner diagnostics foundations. |
| 4 | Master Phases 4, 8, 12, and 13 | Implement Search-owned Overbase lexical/document/secondary/vector index records, Overstore refs, Overvault grant refs, permission snapshots, freshness, and revocation behavior. |
| 5 | Master Phases 1, 4, 8, and 12 | Implement query sessions, source scopes, query-time permission filters, result sets, snippets, previews, pagination, and ranking explanations. |
| 6 | Master Phases 6, 8, 12, and 13 | Implement native app handoffs, Personal AI Assistant tool calls, private-context preview, citations, mobile bindings, and client/admin/SDK surfaces. |
| 7 | Master Phases 4, 8, 10, 11, 12, and 13 | Implement removals, tombstones, public-interest dataset governance, contested ownership, Purpose Tag refs, Overclaim holds, and source pause behavior. |
| 8 | Master Phases 4, 11, 12, and 13 | Implement spam, cloaking, malicious indexing, source poisoning, privacy leak, impersonation, ranking-abuse, Fraud Control/Reputation/Overclaim, and moderation handoffs. |
| 9 | Master Phases 5, 8, 12, and 13 | Implement usage refs, Wallet/Usage Center projections, retention classes, privacy audits, observability, replay, exports, and compliance holds. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, privacy/ranking/compliance boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Search Engine uses Rust-first shared contracts and service-facing APIs for source records, source policies, crawl/index jobs, index records, permission filter snapshots, query sessions, result sets, ranking explanations, handoff refs, removal/tombstone refs, abuse reports, usage refs, audit refs, and replay bundles. TypeScript is acceptable for native/web client surfaces and generated bindings, but it must call Overrid APIs and must not become a privileged search authority.
- Source policies, index jobs, index records, permission snapshots, query sessions, result sets, ranking explanations, handoff refs, removal/tombstone records, abuse reports, usage refs, deterministic fixtures, replay bundles, retention classes, redaction profiles, public-interest dataset manifests, and exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/app/source scope, trace id, idempotency key, source authority, source policy refs, permission/filter refs where applicable, visibility class, redaction profile refs, reason codes, downstream owner refs, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for query hashes, index snapshots, source snapshots, redacted content refs, vectorization input manifests, public-interest dataset manifests, tombstones, handoff envelopes, replay bundles, audit exports, fixture inputs, and deterministic comparisons.
- Structured index state, source metadata, large chunks/artifacts, private grants, queueing where needed, identity, namespace, key status, policy, audit, usage, accounting refs, dispute refs, diagnostics, and replay must use native Overrid service boundaries such as Overbase, Overstore, Overvault, Overqueue, Overpass, Overtenant, Overkey, Universal Namespace Service, Overgate, Overguard, Overwatch, Overmeter, ORU Account Service, Seal Ledger, Wallet/Usage Center, Directory Listings, Workspace and Office Suite, Messaging Center, Maps and Navigation, Social Photo/Video App, Personal AI Assistant, AI Gateway Router, Encrypted Docdex RAG Adapter, Fraud Control, Reputation and Anti-Sybil Service, Overclaim, Mobile SDK, Mobile Backend Gateway, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Elasticsearch, OpenSearch, Solr, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, external ad networks, paid placement, ad auctions, bidding, pricing, revenue forecasts, customer-count assumptions, raw private content, raw message bodies, raw workspace documents, raw exact private locations, private embeddings, vault secrets, final source truth, final fraud authority, final reputation scores, final dispute outcomes, direct ledger/accounting mutation, or hidden moderation action the Search Engine boundary.

## Phase 1: SDS Attachment, Phase 12 Scope, And Search Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #72.**
  - Design: Link this document from the Search Engine SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/native_apps/search_engine.md`, `docs/service_catalog/native_apps/search_engine.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #72 returns both the Search Engine SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep first implementation in Phase 12 because Search is a native utility built on identity, namespace, storage, vault, policy, metering, accounting, public abuse-control, AI/tool-call, mobile, and native app rails that earlier phases provide.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 6, 8, and 11 supply prerequisites; Phase 12 builds the first useful search utility; Phase 13 hardens privacy, ranking-abuse, source-poisoning, retention, incident response, reporting, compliance, and scale.
  - Validation: Review proves the plan does not move Search into Phase 8 storage, Phase 11 public-provider controls, Phase 13-only governance, Directory ownership, Workspace ownership, Messaging ownership, Maps ownership, Social ownership, AI authority, or payment/accounting ownership.

- **1.3 Freeze the Search ownership boundary.**
  - Design: Record that Search owns source registrations, source policies, crawl/index jobs, index records, permission snapshots, query sessions, result sets, ranking explanations, result handoffs, removals, tombstones, abuse reports, usage refs, audit refs, and replay projections.
  - Output: Ownership checklist for architecture, API, UI, implementation, operations, and review gates.
  - Validation: Review confirms Search does not own canonical source content, source-service permissions, message bodies, workspace documents, listing truth, map/place truth, social objects, vault secrets, final moderation decisions, final fraud/reputation verdicts, balances, bids, ads, invoices, or provider payouts.

- **1.4 Carry forward resolved SDS #72 decisions.**
  - Design: Preserve Search-owned Overbase lexical/document/secondary and `vector_index` collections, Overstore chunk/artifact refs, Overvault private grants, audience-safe ranking explanations, default private-search retention, and public-interest dataset governance.
  - Output: Resolved-decision checklist covering lexical-first indexing, vectorization permission gates, explanation audience classes, paid-placement absence attestation, 24-hour private raw payload retention, 30-day redacted audit refs, Purpose Tag refs, Overguard/Compliance checks, Overclaim contested-source handling, and source/steward approvals.
  - Validation: Review rejects external search products as the product boundary, raw private-body indexing, private embeddings in shared indexes, exact ranking weights, hidden spam thresholds, indefinite private query retention, ungoverned public-interest datasets, and Search-owned ownership adjudication.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Record how Directory Listings, Workspace and Office Suite, Messaging Center, Maps and Navigation, Social Photo/Video App, Wallet/Usage Center, Personal AI Assistant, AI Gateway Router, Encrypted Docdex RAG Adapter, Mobile SDK, Mobile Backend Gateway, Overbase, Overstore, Overvault, Overpass, Universal Namespace Service, Overguard, Overwatch, Overmeter, Overclaim, Fraud Control, Reputation/Anti-Sybil, and Central AI Stewardship Interface interact through refs.
  - Output: Boundary matrix naming allowed reads, owned writes, handoff refs, denied direct mutation, audience rules, redaction rules, usage refs, audit refs, replay refs, moderation paths, and owner-service finality.
  - Validation: Review confirms each upstream/downstream service keeps its authority and Search exchanges refs/events rather than copying raw private content or inventing canonical truth owned elsewhere.

## Phase 2: Contracts, Schemas, Events, Stable Errors, And Fixtures

### Work Items

- **2.1 Create the Search Rust contract module.**
  - Design: Add contract types for sources, policies, index jobs, index records, permission snapshots, query sessions, result sets, ranking explanations, handoffs, removals, tombstones, abuse reports, public-interest dataset manifests, usage refs, replay bundles, retention classes, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, source-type enums, crawl-mode enums, visibility-class enums, query-privacy enums, ranking-reason enums, lifecycle enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from source-service content, external search products, AI route authority, accounting mutation, and final dispute authority.

- **2.2 Define canonical JSON and JSON Schema contracts.**
  - Design: Model all Search records with source refs, owner refs, tenant/app scope, visibility class, policy refs, permission snapshot refs, redaction class, state, trace id, idempotency key where mutating, audit refs, reason codes, and stable schema versions.
  - Output: JSON Schema files, valid examples, invalid examples, signed command examples, event examples, query examples, result examples, explanation examples, public-interest examples, retention examples, and replay examples.
  - Validation: Schema tests reject records without required source authority, policy refs, permission/redaction class, visibility class, audit refs, state, trace id, idempotency key where mutating, and schema version.

- **2.3 Define event and replay contracts.**
  - Design: Model source, policy, index job, index record, permission-filter, query, result, explanation, handoff, removal, tombstone, abuse-report, usage, export, retention, and replay events without raw unauthorized private payloads.
  - Output: Event schema set, replay bundle schema, BLAKE3 display hash rules, redaction profile refs, stable ordering, and fixture-backed event streams.
  - Validation: Tests prove events include necessary refs and reason codes while excluding raw private content, private embeddings, vault secrets, exact private locations, private message bodies, private workspace text, payment data, and unrelated behavioral profiles.

- **2.4 Define stable error taxonomy.**
  - Design: Preserve SDS stable errors and add implementation-ready mapping for source, authority, index policy, private permission, permission snapshot, snippet redaction, result visibility, ranking explanation, tombstone, removal policy, query scope, vectorization, public-interest dataset, and abuse-denial failures.
  - Output: Stable error registry, HTTP/API mapping, client-facing messages, support-safe diagnostics, retryability flags, and replay refs.
  - Validation: Tests prove denials are deterministic, support-safe, tenant-safe, audience-safe, and replayable without exposing private recipient settings, hidden abuse thresholds, raw ranking weights, or adversarial signals.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for source registration, source policy update, public indexing, private indexing denial, vectorization denial, query permission filtering, ranking explanation, handoff, tombstone, removal, public-interest dataset registration, abuse report, usage emission, retention, export, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected errors, BLAKE3 hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, redacted outputs, omitted/denied counts, explanation classes, and replay output across repeated runs.

## Phase 3: Source Registration, Source Policies, Index Jobs, And Owner Diagnostics

### Work Items

- **3.1 Implement source registration records.**
  - Design: Support source services/apps with owner refs, namespace route refs, source type, allowed data classes, crawl mode, visibility defaults, permission contract refs, public-interest tags where applicable, and lifecycle state.
  - Output: `search_source` schema, create/read/update APIs, lifecycle transitions, source authority checks, source visibility projections, and audit events.
  - Validation: Tests prove sources cannot register without owner refs, source authority, namespace or service refs, visibility defaults, source policy refs, tenant/app scope, and Overwatch audit refs.

- **3.2 Implement source policy records.**
  - Design: Model indexable fields, snippet rules, vectorization allowance, private-field redaction, freshness windows, removal behavior, ranking constraints, audience-specific projection rules, and policy refs.
  - Output: `source_policy` schema, policy update APIs, versioning, compatibility checks, source pause/disable hooks, redaction rules, and policy-change events.
  - Validation: Tests prove policy changes are versioned, replayable, audited, and cannot widen private visibility, vectorization, snippets, assistant citations, or handoff scope without explicit authority.

- **3.3 Implement crawl, ingest, reindex, and removal job records.**
  - Design: Queue bounded index jobs with source refs, job type, cursor/checkpoint refs, requested scope, source policy refs, worker refs, counts, errors, usage refs, retry policy, and state.
  - Output: `index_job` schema, `POST /search/index-jobs`, job status API, checkpoint state, bounded retry state, failure refs, and `search_engine.index_job_queued` events.
  - Validation: Tests prove failed jobs checkpoint safely, retries do not widen source scope, removal jobs suppress serving paths where required, and every job emits audit and usage refs.

- **3.4 Implement authorized index-record ingest.**
  - Design: Accept source-owned records through authorized service accounts with canonical owner refs, redacted title/summary/content refs, metadata refs, source record refs, freshness, permission snapshot refs, and state.
  - Output: `POST /search/index-records`, ingest validation, deduplication keys, BLAKE3 source snapshot hashes, visibility projections, and stable `source_authority_required` errors.
  - Validation: API tests prove only authorized source services can ingest records and cannot send raw unauthorized private fields, raw secrets, raw exact private locations, or unredacted private message/workspace/social content.

- **3.5 Implement source-owner diagnostics.**
  - Design: Provide source owners with index health, denied fields, stale records, policy mismatch, removal state, ranking explanation coverage, usage refs, and replay refs without exposing other sources.
  - Output: Diagnostics API, source-owner projections, stale-source summaries, failure summaries, policy mismatch summaries, and support-safe diagnostics.
  - Validation: Tests prove diagnostics are source-scoped, tenant-scoped, audience-safe, and cannot reveal competing source internals, private user queries, private result sets, or hidden abuse signals.

## Phase 4: Overbase Index Substrate, Permission Snapshots, Vector Gates, And Revocation

### Work Items

- **4.1 Implement Search-owned Overbase lexical and document indexes.**
  - Design: Store public and source-authorized low-sensitivity records first with lexical, category, locality, freshness, source-type, and source-authority fields behind the Search plus Overbase boundary.
  - Output: Index collection schemas, index lifecycle APIs, lexical index manifests, field redaction manifests, freshness state, and fixture-backed index snapshots.
  - Validation: Tests prove Search uses Overbase as the product boundary, not Elasticsearch/OpenSearch/Solr/PostgreSQL extensions, and rejects fields not allowed by source policy.

- **4.2 Implement secondary and vector index gates.**
  - Design: Add secondary indexes and Overbase `vector_index` collections only where source policy permits vectorization and query-time permission filters are already proven.
  - Output: Vectorization policy checks, vector index manifests, embedding input refs, vector lifecycle state, rebuild hooks, denied-vectorization records, and stable errors.
  - Validation: Tests reject private embeddings, raw private field vectors, assistant-generated private summaries, and shared vector indexes for records without explicit source and caller permission.

- **4.3 Implement permission filter snapshots.**
  - Design: Capture source refs, owner refs, tenant/org roles, audience rules, vault grant refs, app permission refs, redaction class, expiry, and version at indexing and query time.
  - Output: `permission_filter_snapshot` schema, snapshot creation, query-time refresh, stale-snapshot denial, denied/omitted counts, and `search_engine.permission_filter_applied` events.
  - Validation: Tests prove permission filters apply before snippets, previews, embeddings, explanations, ranking features, result handoffs, and assistant citations.

- **4.4 Implement Overstore and Overvault refs.**
  - Design: Reference large chunks and artifacts through Overstore and private/sensitive material through Overvault grants without storing raw vault material or decrypted bodies in Search.
  - Output: Overstore chunk/artifact ref handling, Overvault grant ref handling, grant expiry hooks, redacted content refs, access audit hooks, and denial states.
  - Validation: Tests prove private source access fails closed when grants expire, keys fail, or source policy changes, and Search never stores raw vault secrets or decrypted private bodies.

- **4.5 Implement revocation, tombstone, and reindex propagation.**
  - Design: Rebuild or suppress affected index records when source policy, owner grants, Overvault grants, namespace refs, or deletion/tombstone refs change.
  - Output: Revocation queue, stale marker state, suppression state, rebuild jobs, tombstone propagation, replay refs, and audit events.
  - Validation: Tests prove revoked private content disappears before serving snippets/results/explanations/handoffs and replay shows the exact source policy and permission snapshot used.

## Phase 5: Query Sessions, Permission Filtering, Result Sets, And Ranking Explanations

### Work Items

- **5.1 Implement query session intake.**
  - Design: Accept signed query requests with actor/app/client refs, raw query hash/ref, normalized query refs, filters, privacy mode, source scopes, policy refs, trace id, and usage refs.
  - Output: `query_session` schema, `POST /search/query`, query normalization, source-scope validation, privacy-mode handling, idempotency behavior, and stable `query_scope_conflict` errors.
  - Validation: API tests prove queries require signed envelopes, tenant/app scope, source scopes, privacy mode, trace id, policy refs, and usage/audit refs without storing raw private query text beyond the configured retention class.

- **5.2 Implement private-source permission preview.**
  - Design: Provide a pre-query endpoint that previews allowed, denied, stale, and permission-required private source scopes without leaking private content or private source internals.
  - Output: `POST /search/query/preview-permissions`, preview result schema, denied counts, permission-required refs, stale snapshot refs, and user-visible privacy audit refs.
  - Validation: Tests prove previews do not expose private snippets, titles, embeddings, exact private metadata, hidden block/abuse thresholds, or unauthorized source names.

- **5.3 Implement result set creation and pagination.**
  - Design: Build result sets from permission-filtered records with result refs, ranking version, permission filter version, omitted/denied counts, pagination refs, explanation refs, and state.
  - Output: `result_set` schema, `GET /search/results/{result_set_id}`, pagination state, redacted snippets, denied counts, and result-set events.
  - Validation: Tests prove denied records cannot appear in result ids, titles, snippets, previews, explanations, pagination, handoffs, exports, or replay views visible to unauthorized callers.

- **5.4 Implement ranking explanation fields.**
  - Design: Expose audience-safe reason classes for matched field class, source type, source policy, freshness band, authority/provenance band, category/locality fit, explicit filters, permission/redaction class, safety modifier, diversity/deduplication, public-interest purpose refs, ranking policy version, omitted/denied counts, and paid-placement absence.
  - Output: `ranking_explanation` schema, `GET /search/results/{result_id}/explain`, explanation policy, audience-specific projections, and `search_engine.result_explained` events.
  - Validation: Tests prove explanations do not expose exact weights, thresholds, spam/fraud features, private embeddings, sensitive query expansion terms, raw private fields, or adversarial signals.

- **5.5 Implement non-ad ranking guardrails.**
  - Design: Keep ranking policy transparent and utility-oriented so public content, directory listings, public app pages, public-interest datasets, and small organizations remain discoverable without paid placement.
  - Output: Ranking policy refs, paid-placement absence attestation, anti-ad-trap validation fixtures, source fairness checks, public-interest boost reason codes where policy allows, and replay refs.
  - Validation: Tests reject ranking inputs that encode bids, ad budgets, paid priority, hidden marketplace suppression, pricing, revenue projections, customer-count assumptions, or opaque business suppression.

## Phase 6: Native App Handoffs, Assistant Tool Calls, Private Search, And Client Surfaces

### Work Items

- **6.1 Implement native app result handoffs.**
  - Design: Create result handoff refs to Directory Listings, Workspace and Office Suite, Messaging Center, Maps and Navigation, Social Photo/Video App, Wallet/Usage Center, mobile clients, and other approved native apps without copying source truth.
  - Output: `search_handoff_ref` schema, `POST /search/handoffs`, target app refs, source refs, redaction class, permission refs, handoff state, and audit events.
  - Validation: Tests prove handoffs cannot widen listing contact visibility, workspace access, message metadata, map/location precision, social post audience, wallet/accounting truth, or source ownership.

- **6.2 Implement Personal AI Assistant search tool integration.**
  - Design: Allow Personal AI Assistant to issue explicit user-approved search tool calls with source scopes, private-context permissions, result refs, citations, route refs, and privacy audit refs.
  - Output: Assistant search-tool contract, citation refs, result refs, private-context preview refs, AI Gateway Router handoff refs, Encrypted Docdex RAG Adapter boundary refs, usage refs, and replay refs.
  - Validation: Tests prove assistant calls cannot query private indexes without permission refs, silently train on private search data, keep raw private bundles past retention, or turn Search into central AI decision authority.

- **6.3 Implement workspace, messaging, maps, and social private-search adapters.**
  - Design: Integrate only through source-owned permission contracts and redacted projections for permitted workspace documents, authorized message metadata, map/place refs, and public/authorized social media posts.
  - Output: Source adapter contracts, permission contract fixtures, redacted projection fixtures, denied-private-source fixtures, stale-permission fixtures, and app-specific handoff refs.
  - Validation: Tests prove Search cannot bypass Workspace permissions, Messaging encrypted payload rules, Maps exact-location rules, Social audience rules, or source tombstones.

- **6.4 Implement client, SDK, CLI, and mobile bindings.**
  - Design: Expose typed generated bindings for source registration, index jobs, query, permission preview, results, explanations, handoffs, reports, usage, replay, and diagnostics through normal Overrid APIs.
  - Output: Rust SDK bindings, TypeScript generated bindings for client surfaces, CLI command specs, Mobile SDK/Mobile Backend Gateway projections, and examples.
  - Validation: Tests prove clients use signed Overgate APIs, propagate trace ids and idempotency keys, respect redaction profiles, and cannot use privileged internal index APIs.

- **6.5 Implement source-owner and operator/admin surfaces.**
  - Design: Provide source-owner diagnostics, moderator queues, steward public-interest dataset views, support-safe query/replay views, and operator health views without exposing raw private content.
  - Output: Admin/Developer UI projections, source-owner dashboards, moderation/referral queues, support-safe replay links, query privacy audit views, and source policy edit flows.
  - Validation: Tests prove operators, support staff, stewards, and moderators see only audience-allowed projections and cannot inspect raw private queries, raw private result payloads, private embeddings, or vault material.

## Phase 7: Removals, Tombstones, Public-Interest Dataset Governance, And Contested Sources

### Work Items

- **7.1 Implement removal and tombstone workflows.**
  - Design: Support source, owner, policy, claim, privacy, abuse, and compliance-driven removal requests with replay evidence and serving-path suppression.
  - Output: `removal_tombstone` schema, `POST /search/index-records/{record_id}/remove`, tombstone states, retained audit refs, source-owner notifications, and tombstone events.
  - Validation: Tests prove removed records stop appearing in results, snippets, previews, explanations, handoffs, exports, and assistant citations while required audit/removal refs remain replayable.

- **7.2 Implement source pause, stale, and policy-change behavior.**
  - Design: Pause or mark sources stale when freshness, policy, source availability, ownership, permission, or compliance conditions require suppression or revalidation.
  - Output: Source pause state, stale-source state, freshness checks, policy-update-pending state, reindex jobs, suppression markers, and source-owner diagnostics.
  - Validation: Tests prove stale or paused private sources deny results by default and public sources only serve last indexed records when source policy allows.

- **7.3 Implement public-interest dataset registration.**
  - Design: Require registered source policy naming dataset steward refs, owner/source refs, Purpose Tag Registry tag version refs, data class, evidence refs, license/rights/Overasset or namespace refs where available, and Overguard/Compliance Boundary checks before indexing.
  - Output: Public-interest dataset manifest schema, steward approval refs, Purpose Tag refs, source evidence refs, license refs, compliance check refs, Overguard decision refs, and dataset indexing state.
  - Validation: Tests reject public-interest datasets without source/steward approval, data class, rights/license evidence, Purpose Tag refs where applicable, Overguard/Compliance checks, and replay evidence.

- **7.4 Implement contested ownership and shared stewardship flows.**
  - Design: Move contested datasets or source records to limited visibility or metadata-only indexing, pause new full-content/vector indexing, attach Overclaim dispute refs, and preserve source-review reason codes.
  - Output: Contested-source state, metadata-only state, source-review state, shared steward quorum refs, Overclaim refs, Overwatch evidence refs, and denied-indexing reason codes.
  - Validation: Tests prove Search does not adjudicate ownership, does not continue full-content/vector indexing during contested states, and does not expose private or disputed material as authoritative.

- **7.5 Implement retention and cleanup for removal artifacts.**
  - Design: Apply retention classes to tombstones, deletion requests, private raw payloads, redacted audit refs, pinned user searches, source policy overrides, and Compliance Boundary holds.
  - Output: Retention resolver, cleanup jobs, sealed hold refs, export/delete interaction rules, redacted replay refs, and cleanup audit events.
  - Validation: Tests prove private raw query/result payloads expire within 24 hours by default, redacted audit refs default to 30 days unless pinned/source policy/hold changes them, and holds do not expose held content.

## Phase 8: Abuse, Spam, Source Poisoning, Ranking Abuse, And Moderation Handoffs

### Work Items

- **8.1 Implement search abuse reports.**
  - Design: Record spam, malicious result, impersonation, cloaking, privacy leak, source poisoning, ranking abuse, unsafe content, and removal-failure reports with evidence refs.
  - Output: `search_abuse_report` schema, `POST /search/reports`, report lifecycle, reporter refs, source/result/query refs, evidence refs, moderation refs, and audit events.
  - Validation: Tests prove reports are replayable, audience-safe, source-scoped where needed, and do not leak private queries, denied result payloads, or hidden detection thresholds.

- **8.2 Implement source poisoning and cloaking detection hooks.**
  - Design: Detect suspicious source behavior through policy-compatible signals, crawl/index consistency checks, source history, freshness anomalies, and Fraud Control/Reputation handoffs.
  - Output: Source-poisoning refs, cloaking suspicion refs, risk reason codes, throttling recommendations, review queue refs, and source-owner notifications.
  - Validation: Tests prove detection hooks propose bounded actions and cannot become final Fraud Control, final Reputation, final Overclaim, or hidden moderation authority.

- **8.3 Implement ranking-abuse controls.**
  - Design: Track manipulation attempts, duplicate/source farm patterns, misleading metadata, keyword spam, public-interest tag misuse, locality/category abuse, and explanation gaming without exposing adversarial internals.
  - Output: Ranking-abuse records, safety modifier refs, source throttles, explanation redaction rules, moderation queue refs, and replay refs.
  - Validation: Tests prove public users get safe reason-code summaries while source owners/stewards receive stronger replay refs only under audience-scoped policy.

- **8.4 Implement moderation, Fraud Control, Reputation, and Overclaim handoffs.**
  - Design: Route abuse reports, contested sources, scam reports, impersonation, privacy leaks, source poisoning, and ranking manipulation to the owner service and relevant fraud/dispute services through refs.
  - Output: Handoff contracts, Fraud Control refs, Reputation/Anti-Sybil refs, Overclaim refs, moderation refs, appeal refs, source action refs, and audit refs.
  - Validation: Tests prove Search does not own final moderation, final fraud, final reputation, final claim outcome, or final appeal authority.

- **8.5 Implement abuse observability and incident triggers.**
  - Design: Alert on private-result leaks, sudden denied-count changes, malicious crawl spikes, source poisoning bursts, ranking-abuse reports, stale index floods, tombstone failures, and missing usage refs.
  - Output: Metrics, alert rules, incident trigger refs, source-owner notifications, operator dashboards, and replay links.
  - Validation: Tests prove alerts include enough evidence for incident response without exposing raw private data or adversarial detection details to unauthorized audiences.

## Phase 9: Usage, Retention, Observability, Replay, Exports, And Compliance Holds

### Work Items

- **9.1 Implement search usage refs.**
  - Design: Emit usage refs for source registration, crawling, indexing, vector/text updates, private permission checks, queries, result delivery, ranking explanations, handoffs, removals, abuse reports, replay, compute, storage, and bandwidth.
  - Output: `search_usage_ref` schema, Overmeter refs, Wallet/Usage Center receipt refs, usage pending/reconciliation state, and `search_engine.usage_emitted` events.
  - Validation: Tests prove Search emits usage refs without maintaining balances, bids, ads, invoices, provider payouts, ledger truth, hardcoded prices, revenue forecasts, or per-transaction external payment rails.

- **9.2 Implement private-search retention and privacy audit.**
  - Design: Apply default retention of no more than 24 hours for raw private query text, normalized query payloads, private result payloads, snippets, and assistant handoff bundles, expiring sooner on revocation or stricter source policy.
  - Output: Retention classes, privacy audit refs, raw-payload expiry jobs, redacted audit refs, pinned-search exceptions, source policy overrides, and revocation cleanup hooks.
  - Validation: Tests prove private raw payloads expire on schedule, revocation triggers earlier cleanup, redacted audit refs remain only as allowed, and privacy audit views do not expose raw private material.

- **9.3 Implement observability and source health metrics.**
  - Design: Expose source count, active/paused sources, index lag, job failure rate, stale record count, query latency, denied-result count, private-source denial rate, ranking explanation coverage, tombstone latency, abuse volume, handoff failure rate, and usage emission status.
  - Output: Metrics, dashboards, source-owner diagnostics, operator diagnostics, alert thresholds, and replay refs.
  - Validation: Tests prove metrics are tenant-safe, source-safe, and do not leak private queries, denied result content, private source names, exact private locations, or hidden abuse thresholds.

- **9.4 Implement replay and audit export.**
  - Design: Reconstruct source registration, source policy, index jobs, permission filtering, ranking, result creation, handoff, removal, report handling, retention, and usage decisions from stored refs.
  - Output: `GET /search/replay/{record_id}`, replay bundle schema, Overwatch audit export refs, redacted replay profiles, and support-safe replay views.
  - Validation: Tests prove replay is deterministic, audience-scoped, redacted by policy, and sufficient to explain allow/deny/tombstone/handoff/usage decisions without raw unauthorized content.

- **9.5 Implement compliance holds, exports, and portability boundaries.**
  - Design: Coordinate Compliance Boundary holds, user/org exports, source-owner exports, deletion requests, pinned searches, public-interest report extracts, and replay retention without making Search a canonical source store.
  - Output: Hold refs, export manifests, portability bundles, source-scoped exports, redacted public-interest summaries, deletion interaction rules, and audit refs.
  - Validation: Tests prove exports contain only authorized source/result refs, redacted snippets where allowed, usage/audit refs, and no raw vault secrets, raw private source content, private embeddings, or unrelated profile data.

## Phase 10: Validation, Governance Handoff, Scale Readiness, And Documentation Closure

### Work Items

- **10.1 Validate sub-build plan structure and backlinks.**
  - Design: Check title prefix, attached SDS link, 10 phases numbered 1 through 10, five work items per phase, Design/Output/Validation fields, and local Markdown links.
  - Output: Validation evidence in `docs/build_plan/progress.md`, queue state/progress updates, and Docdex search/index evidence.
  - Validation: Focused checks pass for structure, links, final newline, no tabs, queue JSON, and Docdex retrieval of SDS #72 plus the sub-build plan.

- **10.2 Validate stack and authority guardrails.**
  - Design: Check that the plan preserves Rust-first core services, canonical JSON plus JSON Schema, Ed25519, BLAKE3, native Overrid boundaries, Phase 12 first build, Phase 13 hardening, non-ad ranking, and no conventional search/database/object-store/vault boundary drift.
  - Output: Guardrail scan results and explicit negative-control review for forbidden product-boundary, pricing, revenue, customer-count, blockchain/NFT, ad-ranking, paid-placement, raw-private-content, and final-authority terms.
  - Validation: Review passes with only expected negative-control wording that explicitly rejects those assumptions.

- **10.3 Validate master Phase 0 through Phase 13 alignment.**
  - Design: Confirm master plan, Phase 12, Phase 13, service catalog crosswalk, SDS, and service catalog entry all agree that Search first builds in Phase 12 with earlier prerequisites and Phase 13 governance/security/compliance/scale hardening.
  - Output: Updated master/crosswalk rows, Phase 12 Search workstream details, Phase 13 Search threat/security/reliability/reporting coverage, and backlinks from SDS/service files.
  - Validation: Review confirms no master phase reordering is required and no Search work is incorrectly moved into storage, AI, public-provider, governance-only, or accounting ownership.

- **10.4 Validate implementation handoff readiness.**
  - Design: Ensure implementation teams can derive target contracts, APIs, events, fixtures, policy checks, privacy checks, source adapters, handoffs, abuse flows, retention, usage, replay, and client surfaces from the plan without inventing missing authority boundaries.
  - Output: Handoff checklist covering Rust modules, schemas, APIs, events, fixtures, storage/vault refs, permission filters, ranking explanations, assistant/tool boundaries, abuse handoffs, retention, observability, and replay.
  - Validation: Review confirms each SDS build-breakdown item maps to at least one phase/work item and each work item has a testable output.

- **10.5 Validate documentation and queue closure.**
  - Design: Refresh Docdex index for changed docs, search for SDS #72 and the sub-build plan, export DAG evidence where available, update progress notes, and preserve the known `docdexd run-tests` blocker if no runner is configured.
  - Output: Docdex index/search evidence, DAG evidence, validation command evidence, queue state marking `072-build-plan` complete, and progress notes listing the next incomplete build-plan task.
  - Validation: `git diff --check`, local link checks, JSON checks, stale-marker checks, stack guardrail checks, Docdex index/search, and configured repo test command status are recorded before handoff.
