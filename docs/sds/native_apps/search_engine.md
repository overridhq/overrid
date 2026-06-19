SDS #72

# Search Engine SDS

## Purpose

Build permission-aware search across public Overrid content, directory listings, public app pages, public-interest datasets, and user-authorized private content.

Search Engine is the native discovery utility for Overrid. It owns source registrations, crawl/index jobs, index records, permission filter snapshots, query sessions, result sets, ranking explanations, removal/tombstone refs, abuse/spam refs, and usage refs. It is not an ad auction, canonical data store, private-data bypass, or central AI decision service. Search must make useful information discoverable without forcing people and small organizations into paid visibility traps.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [search_engine.md](../../service_catalog/native_apps/search_engine.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |

## Service Family

- Family: Native applications
- Owning layer: Native public utility application layer for permission-aware discovery, indexing, ranking explanation, and search handoffs
- Primary data scope: source registrations, crawler/index jobs, index records, permission snapshots, query sessions, result sets, ranking features/explanations, removal/tombstone refs, spam/abuse refs, usage refs, and audit refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md)

## Problem Statement

Search is a public utility, but dominant search systems often become advertising toll roads and surveillance ranking engines. Overrid needs search across public content, native directory listings, public app pages, public-interest datasets, workspace content, and user-authorized private context without turning ranking into a pay-to-survive marketplace or leaking private data.

The key design issue is authority separation. Source apps own their records and permissions; Search owns discovery indexes and query-time filtering. Search must never decide that a private record is public, never let paid placement override public-utility ranking, and never create a hidden dependency where small businesses must buy ads to be visible.

## Goals

- Define source registration, source policy, index record, index job, permission filter, query session, result set, ranking explanation, removal/tombstone, spam/abuse, and usage records.
- Index public Overrid content, Directory Listings, public app pages, public-interest datasets, Maps/places where allowed, Social public posts where allowed, and Workspace/private content only with explicit source authorization.
- Enforce permission filters before result visibility, snippets, previews, ranking features, and assistant handoffs.
- Provide transparent ranking explanation fields and inspectable ranking controls.
- Support redaction, tombstones, removals, source pausing, and freshness checks.
- Integrate Personal AI Assistant through explicit search tool calls and private context permissions.
- Emit usage and receipt refs without pricing or revenue assumptions.

## Non-Goals

- Do not become an advertising auction, paid ranking surface, marketplace promotion engine, or surveillance ranking service.
- Do not own canonical app content, workspace documents, message bodies, listing truth, map/place truth, social media objects, vault secrets, or ledger truth.
- Do not bypass source-service permissions, Overvault grants, Overguard policy, user/org ownership, or tenant boundaries.
- Do not expose private snippets, embeddings, metadata, or ranking features to unauthorized callers.
- Do not let Personal AI Assistant, Central AI Service, or any native app query private indexes without explicit permission refs.
- Do not replace Overclaim, Fraud Control, Reputation/Anti-Sybil, or app-level moderation.
- Do not add pricing, customer-count, revenue, blockchain, NFT, or per-transaction fee assumptions.

## Primary Actors And Clients

- Users searching public Overrid content, directory listings, app pages, maps, workspace content, social content, and authorized private sources.
- Organizations publishing public pages, directory pages, workspace-public docs, public-interest datasets, or app content.
- Native apps registering sources, emitting index update refs, receiving result handoffs, or requesting removal/tombstone behavior.
- Personal AI Assistant issuing user-approved search queries and receiving result refs/citations.
- Moderators and stewards reviewing spam, malicious content, impersonation, ranking abuse, and removal requests.
- Directory Listings, Workspace and Office Suite, Messaging Center, Maps and Navigation, Social Photo/Video App, Wallet and Usage Center, Mobile SDK, and Central AI Stewardship Interface.
- Overpass, Overtenant, Overkey, Universal Namespace Service, Overbase, Overstore, Overvault, Overguard, Overwatch, Overmeter, Overclaim, Fraud Control Service, Reputation and Anti-Sybil Service, ORU Account Service, and Seal Ledger.

## Dependencies

- [Directory Listings](directory_listings.md) for public/authorized listing records, organization pages, category/locality refs, and anti-ad-trap discovery constraints.
- [Workspace and Office Suite](workspace_office_suite.md) for permission-aware workspace/document indexes, private search refs, and export/import handoffs.
- [Messaging Center](messaging_center.md) for authorized message metadata search and notification/search handoffs.
- [Maps and Navigation](maps_navigation.md) for place/local discovery refs, route-safe public map records, and location privacy boundaries.
- [Social Photo/Video App](social_photo_video_app.md) for public/authorized media post search and moderation/takedown refs.
- [Personal AI Assistant](../ai_rag_model_routing/personal_ai_assistant.md) and [Encrypted Docdex RAG Adapter](../ai_rag_model_routing/encrypted_docdex_rag_adapter.md) for user-approved private context search and citation refs.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), [Overkey](../control_plane/overkey.md), and [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) for identity, tenant, namespace, source ownership, and public route refs.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), and [Overvault](../data_storage_namespace/overvault.md) for index metadata, object refs, authorized private refs, and source-owned data boundaries.
- [Overguard](../trust_policy_verification/overguard.md), [Overwatch](../control_plane/overwatch.md), [Overclaim](../trust_policy_verification/overclaim.md), [Fraud Control Service](../federation_public/fraud_control_service.md), and [Reputation and Anti-Sybil Service](../trust_policy_verification/reputation_anti_sybil_service.md) for indexing policy, audit, disputes, spam, impersonation, and ranking-abuse signals.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), and [Wallet and Usage Center](wallet_usage_center.md) for usage visibility and receipt refs.

## Owned Responsibilities

Search Engine owns:

- Source registration records and source-specific indexing policy refs.
- Crawl, ingest, index, reindex, removal, freshness, and tombstone jobs.
- Public and permission-aware index records with redacted text, metadata, vector refs, source refs, and freshness.
- Query sessions, filters, result sets, ranking explanations, and citation/result handoff refs.
- Permission filter snapshots applied at indexing and query time.
- Ranking policy refs that exclude paid-placement dependency and expose user-visible explanations.
- Spam, malicious-indexing, impersonation, cloaking, source-abuse, and removal-request records.
- Usage refs for indexing, querying, result delivery, private source access, assistant handoffs, exports, replay, compute, storage, and bandwidth.

Search Engine does not own canonical source content, app state, message bodies, workspace documents, private vault material, final moderation decisions, final fraud/reputation verdicts, balance truth, or provider payouts.

## Data Model

- `search_source`: source id, owning service/app refs, owner actor/org refs, source type, namespace route refs, allowed data classes, crawl mode, visibility defaults, permission contract refs, and state.
- `source_policy`: source ref, indexable fields, snippet rules, vectorization allowance, private-field redaction, freshness window, removal behavior, ranking constraints, and policy refs.
- `index_job`: job id, source refs, job type, cursor/checkpoint refs, requested scope, policy refs, worker refs, counts, errors, usage refs, and state.
- `index_record`: source record refs, canonical owner refs, title/summary refs, redacted content refs, metadata refs, vector/text index refs, visibility class, permission snapshot refs, freshness, tombstone refs, and state.
- `permission_filter_snapshot`: source refs, owner refs, tenant/org roles, audience rules, vault grant refs, app permission refs, redaction class, expiry, and version.
- `query_session`: query id, actor/app/client refs, raw query hash/ref, normalized query refs, filters, privacy mode, source scopes, policy refs, and usage refs.
- `result_set`: query ref, result refs, ranking version, permission filter version, omitted/denied counts, pagination refs, explanation refs, and state.
- `ranking_explanation`: result ref, matched fields, source freshness, authority/source signals, locality/category signals, permission redaction class, safety modifiers, and paid-placement absence attestation.
- `search_handoff_ref`: query/result refs, target native app, result source refs, redaction class, user/app permission refs, and handoff state.
- `removal_tombstone`: source refs, record refs, removal reason, requester refs, claim refs, policy refs, retained audit refs, expiry/retention, and state.
- `search_abuse_report`: reporter refs, source/result/query refs, report type, evidence refs, spam/fraud/reputation refs, moderation refs, claim refs, and state.
- `search_usage_ref`: indexing/query/private-search/result-delivery/assistant-handoff/removal/replay usage, Overmeter refs, and wallet receipt refs.

Common envelope fields: `id`, `tenant_id`, `actor_id`, `organization_id`, `app_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

- `POST /search/sources`: registers a source service/app with indexing and permission contracts.
- `GET /search/sources/{source_id}`: returns source metadata visible to the caller.
- `PATCH /search/sources/{source_id}`: updates source policy, paused state, freshness, or removal behavior.
- `POST /search/index-jobs`: queues crawl, ingest, reindex, tombstone, or removal work.
- `GET /search/index-jobs/{job_id}`: returns index job state, counts, failures, and usage refs.
- `POST /search/index-records`: ingests source-owned records through an authorized service account.
- `POST /search/index-records/{record_id}/remove`: tombstones or removes a search record under source/removal policy.
- `POST /search/query`: runs a permission-aware query and returns result refs, snippets, denied counts, and explanations.
- `POST /search/query/preview-permissions`: previews source scopes and denied private sources before query execution.
- `GET /search/results/{result_set_id}`: reads a result set with pagination and explanation refs.
- `GET /search/results/{result_id}/explain`: returns ranking explanation fields visible to the caller.
- `POST /search/handoffs`: creates a result handoff to a native app or Personal AI Assistant.
- `POST /search/reports`: records spam, malicious result, impersonation, cloaking, privacy leak, or ranking-abuse reports.
- `GET /search/replay/{record_id}`: reconstructs source, index job, query, permission, ranking, removal, abuse, and usage decisions.

Mutating APIs require signed actor/service identity, tenant scope, source authority, trace id, idempotency key, source policy refs, and permission/filter refs where applicable. Stable errors include `source_not_registered`, `source_paused`, `source_authority_required`, `index_policy_denied`, `private_source_permission_required`, `permission_snapshot_stale`, `snippet_redaction_required`, `result_visibility_denied`, `ranking_explanation_unavailable`, `tombstone_required`, `removal_policy_denied`, and `query_scope_conflict`.

## Event Surface

- `search_engine.source_registered`: source registered with policy refs.
- `search_engine.source_policy_updated`: source policy changed.
- `search_engine.index_job_queued`: index job queued.
- `search_engine.index_job_started`: index job started.
- `search_engine.index_record_updated`: index record created or refreshed.
- `search_engine.index_record_tombstoned`: index record tombstoned/removed from results.
- `search_engine.index_job_completed`: index job completed with counts and usage refs.
- `search_engine.index_job_failed`: index job failed with retry/error refs.
- `search_engine.query_submitted`: query session accepted.
- `search_engine.permission_filter_applied`: permission filter snapshot applied.
- `search_engine.result_set_created`: result set created.
- `search_engine.result_explained`: ranking explanation generated or requested.
- `search_engine.handoff_created`: result handoff to app/assistant recorded.
- `search_engine.abuse_reported`: search abuse/spam/privacy leak report recorded.
- `search_engine.usage_emitted`: usage refs emitted.

Events include source refs, index refs, query refs, result refs, permission snapshot refs, ranking version, redaction class, reason codes, claim refs, audit refs, and usage refs. Events must not include unauthorized private content, private embeddings, raw secrets, private message bodies, exact private locations, or payment data.

## Core Workflow

1. A source service registers an indexing contract naming owner refs, indexable fields, redaction rules, freshness, removal policy, and permission filter behavior.
2. Source emits index update or crawl refs. Search queues bounded index jobs and records usage/freshness state.
3. Index workers ingest only allowed fields, create redacted text/vector refs, attach permission snapshots, and store index records in Overbase/index substrates.
4. User or app submits a query with source scopes, filters, privacy mode, and permission refs.
5. Search normalizes the query, applies source scopes, applies permission filters before snippets/results, ranks allowed results, and emits ranking explanations.
6. Result refs can hand off to Directory, Workspace, Maps, Messaging, Social, Personal AI Assistant, or mobile clients without copying source truth.
7. Removals, tombstones, disputes, abuse reports, or source policy changes update index state and preserve replay evidence.
8. Usage and audit records flow to Overmeter, Wallet/Usage Center, Overwatch, and accounting views.

## State Machine

Source lifecycle:

1. `requested`
2. `registered`
3. `active`
4. `paused`
5. `policy_update_pending`
6. `disabled`
7. `removed`

Index job lifecycle:

1. `queued`
2. `policy_checked`
3. `running`
4. `checkpointed`
5. `completed`
6. `retry_pending`
7. `failed`
8. `cancelled`

Index record lifecycle:

1. `candidate`
2. `permission_checked`
3. `indexed`
4. `stale`
5. `redaction_required`
6. `tombstoned`
7. `removed`

Query lifecycle:

1. `submitted`
2. `normalized`
3. `permission_filtered`
4. `ranked`
5. `result_ready`
6. `denied`
7. `failed`
8. `expired`

Removal/report lifecycle:

1. `submitted`
2. `triaged`
3. `source_review_required`
4. `action_pending`
5. `tombstoned`
6. `rejected`
7. `under_claim`
8. `corrected`
9. `closed`

State transitions are append-only. Tombstones retain minimum audit/removal refs where policy requires, but results stop exposing removed content.

## Policy And Security

- Source services are the permission authorities for their records; Search must enforce source contracts and query-time permission filters.
- Private content indexing requires explicit source permission, user/org/app permission refs, and redaction/vectorization rules.
- Permission filters apply before snippets, result titles where sensitive, embeddings, explanations, handoffs, and assistant citations.
- Ranking must expose explanation fields and must not depend on paid placement, ad auctions, or opaque business suppression.
- Removal/tombstone requests require source, owner, policy, or claim authority and must preserve replay evidence.
- Search cannot expand location precision, message visibility, workspace access, or social post audience beyond source permissions.
- Index workers must avoid storing vault secrets, private decrypted bodies, raw payment data, or unrelated behavioral profiles.
- Abuse controls must handle spam, cloaking, impersonation, malicious indexing, privacy leaks, source poisoning, and ranking manipulation.
- Assistant and central AI handoffs require explicit result refs and permission scope; private search cannot silently become AI training data.

## Metering And Accounting

- Emit usage refs for source registration, crawling, indexing, vector/text index updates, private permission checks, queries, result delivery, ranking explanations, handoffs, removals, abuse reports, replay, compute, storage, and bandwidth.
- Link usage to actor, org, app, source, query, result set, index job, permission snapshot, handoff, Overmeter refs, and wallet receipt refs.
- Search Engine does not maintain balances, bids, ads, invoices, provider payouts, or ledger truth.
- Native-service economics remain structural and near-cost; surplus routing and grants are outside ranking.
- Do not encode hardcoded prices, revenue forecasts, paid visibility, or per-transaction fees.

## Observability And Operations

- Expose source count, active/paused sources, index lag, job failure rate, stale record count, query latency, denied-result count, private-source denial rate, ranking explanation coverage, tombstone latency, spam/abuse volume, handoff failure rate, and usage emission status.
- Alert on private-result leaks, permission snapshot failures, sudden denied-count changes, source poisoning bursts, malicious crawl spikes, ranking-abuse reports, stale index floods, tombstone failures, and missing usage refs.
- Provide source-owner diagnostics for index health, denied fields, removal state, and ranking explanations without exposing other sources.
- Provide user-visible search privacy audit for private-source searches, assistant handoffs, and permission previews.
- Provide replay for source registration, index jobs, query filtering, ranking, removal, report handling, and usage refs.

## Failure Modes And Recovery

- Source unavailable: use last indexed public/authorized records only if freshness policy allows; otherwise mark source stale or excluded.
- Permission snapshot stale: deny private results or rerun filter before returning snippets.
- Index job fails: checkpoint progress, retry bounded segments, and surface job failure to source owner.
- Removal/tombstone fails: suppress result from serving path until consistency is restored.
- Query route unavailable: return retryable failure without widening source scopes or privacy.
- Ranking explanation missing: return result only if policy allows, with explanation unavailable reason code.
- Abuse false positive: restore result through source correction refs and preserve prior action evidence.
- Usage emission fails: mark query/index job usage pending and reconcile before final receipt visibility.

## Validation Plan

- Public sources can register, index, query, explain, hand off, tombstone, and replay records through explicit APIs.
- Private Workspace, Messaging, Social, or personal data appears only when source permission and caller permission allow it.
- Permission filters apply before snippets, previews, embeddings, explanations, and assistant handoffs.
- Directory and public app search remains useful without paid placement or ad-ranking mechanics.
- Source pause, source policy change, tombstone, and removal operations affect results within defined freshness windows.
- Ranking explanations show inspectable non-ad signals and omit private internals when not authorized.
- Abuse reports create evidence refs and moderation/fraud/claim handoffs.
- Usage refs flow to Overmeter and Wallet/Usage Center.
- Replay reconstructs source, index, query, filter, ranking, result, handoff, removal, and usage decisions.

## Build Breakdown

1. Define source, policy, index job, index record, permission filter, query session, result set, explanation, handoff, removal, abuse report, and usage schemas.
2. Implement source registration, policy update, index job, ingest, removal/tombstone, query, explanation, handoff, report, and replay APIs.
3. Add source adapters for Directory Listings, public app pages, Workspace permitted content, Maps/place refs, Social public posts, and public-interest datasets.
4. Add permission filter snapshots with query-time enforcement and private-source previews.
5. Add ranking explanation model and anti-ad-trap constraints.
6. Add spam/cloaking/source poisoning/privacy leak detection handoffs to Fraud Control, Reputation/Anti-Sybil, Overclaim, and moderation.
7. Add Personal AI Assistant search tool integration, citations, and private-context audit refs.
8. Add Overmeter usage refs, Wallet/Usage Center receipts, Overwatch replay, SDK, mobile client, and operator/source-owner diagnostics.

## Handoff And Downstream Use

- Directory Listings depends on Search for public and local listing discovery without ad-trap visibility.
- Workspace and Office Suite depends on Search for permission-aware document discovery and private workspace search.
- Personal AI Assistant depends on Search for user-approved retrieval, result refs, and citations.
- Maps and Navigation uses Search for local discovery refs while preserving location privacy.
- Social Photo/Video App uses Search for public/authorized post and media discovery after moderation.
- Messaging Center uses Search only for authorized metadata or explicit user-approved message search.
- Wallet and Usage Center shows search/indexing/query usage and permissions.
- Central AI Stewardship Interface can inspect public-interest search health and abuse evidence without private content leakage.

## Open Design Questions

Resolved decisions:

- The first Phase 12 grid-resident substrate should be Search-owned index schemas on Overbase: Rust service code using Overbase document/secondary indexes for lexical search and Overbase `vector_index` collections for embeddings, with large chunks and artifacts referenced through Overstore and private or sensitive material referenced through Overvault grants. Embedded Rust indexing engines may be evaluated behind that boundary, but the product contract is Search plus Overbase, Overstore, and Overvault, not Elasticsearch, PostgreSQL extensions, S3-style object stores, or Encrypted Docdex RAG as the general search substrate. The first slice should index public and source-authorized low-sensitivity records with lexical, category, locality, and freshness ranking, then add vector search only where source policy permits vectorization and query-time permission filters are proven.
- Ranking explanations should expose audience-safe classes: matched field class, source type, source policy, freshness band, source authority or provenance band, category and locality fit, explicit user filter matches, permission/redaction class, safety or moderation modifier class, diversity/deduplication reason codes, public-interest purpose refs where relevant, ranking policy version, omitted/denied counts, and paid-placement absence attestation. They must not expose exact weights, thresholds, spam/fraud features, private embeddings, sensitive query expansion terms, raw private fields, or adversarial signals. Public users get reason-code summaries; source owners and stewards get stronger replay refs through Overwatch and Overguard under audience-scoped policy.
- The default private-search retention period should be no more than 24 hours for raw private query text, normalized query payloads, private result-set payloads, snippets, and assistant handoff bundles, expiring sooner on grant revocation or stricter source policy. Durable records default to redacted refs only: query hash, result ids/classes, permission snapshot version, denied/omitted counts, usage refs, privacy-audit refs, and Overwatch replay refs for 30 days unless the user or organization pins the search, the source requires shorter retention, or Overclaim/Compliance Boundary hold policy requires longer sealed retention.
- Public-interest dataset indexing requires a registered source policy naming dataset steward refs, owner/source refs, Purpose Tag Registry tag version refs, data class, evidence refs, license/rights/Overasset or namespace refs where available, and Overguard/Compliance Boundary checks before indexing. Shared ownership uses explicit owner-service or steward-quorum approvals and audience-specific redaction. Contested ownership moves to limited visibility or metadata-only indexing, pauses new full-content/vector indexing, attaches Overclaim dispute refs and Overwatch evidence, and shows source-review/contested reason codes without letting Search adjudicate ownership itself.
