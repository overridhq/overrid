# SUB BUILD PLAN #65 - Docdex Adapter

Attached SDS: [SDS #65 - Docdex Adapter](../sds/adapters/docdex_adapter.md)

## Purpose

This sub-build plan turns SDS #65 into an implementation sequence for Docdex Adapter. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Docdex Adapter is the Phase 6 product-integration boundary that makes Docdex indexing, search, retrieval-only RAG work, service-admin ingest, capability snapshots, cleanup, usage, and replay visible as normal Overrid workloads. It owns Docdex instance refs, repo bindings, encrypted config refs, workload manifests, job refs, result refs, cleanup/deprovision refs, usage refs, audit refs, and replay bundles. It does not reimplement Docdex internals, authorize final AI context bundles, choose model routes, run final inference, own Overrid identities or accounting truth, store raw encryption keys, or expose unauthorized private content.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #65: Docdex Adapter](../sds/adapters/docdex_adapter.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Docdex Adapter service plan](../service_catalog/adapters/docdex_adapter.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency keys, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack manifests, Overqueue jobs, Oversched placement, Overlease reservations, Overrun lease-bound execution, Overmeter raw usage facts, retries, cancellation, timeouts, and dead-letter handling. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Workload Classifier facts, Overguard policy checks, Policy Dry-Run previews, deny-by-default behavior, data-class handling, and operation eligibility decisions. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill handoffs, dispute visibility, and the rule that the adapter emits usage but never maintains balances or pricing truth. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Controls the first build point for Docdex product workloads: instance/repo binding, encrypted index/search/retrieval/admin-ingest jobs, capability snapshots, result refs, usage, diagnostics, and product proof. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies later protected system-service placement, backup/restore, failover, rollout, and founder-hardware removal gates for any grid-resident Docdex package. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase state refs, Overstore object/result/artifact refs, Overvault key/secret refs, namespace refs, protected private records, retention classes, and replay substrates. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies later application-intent and system package deployment planning for managed or grid-resident Docdex connector/package promotion. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Consumes Docdex result/capability refs through Personal AI Assistant, workspace, search, Codali, mobile, and native app context flows, but does not move Docdex Adapter's first build out of Phase 6. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies hardening for encrypted repo leakage, key failure, cleanup failure, deprovision evidence, unauthorized repo access, admin-ingest plaintext cleanup, replay gaps, incident response, security review, and compliance controls. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #65 first build work aligned to master Phase 6 with earlier contract/execution/policy/accounting/protected-storage prerequisites and later Phase 7/9 packaging plus Phase 13 governance/security/compliance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, hidden plaintext, final-model-selection, or direct-Docdex-internals drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 6, 8, 12, and 13 | Attach SDS #65, preserve Phase 6 as first build, record prerequisites, and freeze the Docdex Adapter authority boundary. |
| 2 | Master Phases 0, 1, 3, 4, 5, 6, and 8 | Define Rust contracts, canonical schemas, state machines, stable errors, signed refs, hashes, and fixtures. |
| 3 | Master Phases 1, 4, 6, and 8 | Implement Docdex instance registry, repo binding, encrypted config, capability snapshot, and lifecycle APIs. |
| 4 | Master Phases 3, 4, 5, 6, and 8 | Implement workload manifests and index/search/retrieval job submission through connector or Overpack/Overqueue paths. |
| 5 | Master Phases 4, 6, 8, and 13 | Integrate encrypted repo capability facts, leakage profiles, disabled operation handling, and fail-closed behavior. |
| 6 | Master Phases 1, 4, 5, 6, 8, and 13 | Implement service-admin ingest, plaintext cleanup evidence, deprovision preparation, and cleanup state. |
| 7 | Master Phases 5, 6, 8, 12, and 13 | Integrate Encrypted Docdex RAG Adapter handoff, result refs, usage, audit, replay, diagnostics, and downstream consumers. |
| 8 | Master Phase 6, with prerequisites from Phases 0, 1, 3, 4, 5, and 8 | Prove a local/seed Docdex daemon through Overrid-shaped signed job envelopes and SDK/CLI/admin flows. |
| 9 | Master Phases 7, 9, and 13, with prerequisites from Phases 0 through 8 | Prepare grid-resident package readiness and harden retry, deprovision, key-failure, cleanup-failure, incident, and compliance behavior. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Docdex Adapter core is a Rust service/module using shared contract crates, Tokio for bounded job/capability/cleanup/replay workers, and Axum/Tower/Hyper-style HTTP only where a service boundary is needed.
- Instance refs, repo bindings, encrypted config refs, workload manifests, index/search/retrieval/admin-ingest jobs, result refs, cleanup/deprovision refs, usage refs, audit records, events, fixtures, replay bundles, and diagnostics use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant and owner scope, repo binding refs, trace id, idempotency key, policy refs, key/secret refs where applicable, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for capability snapshots, workload manifests, query/request snapshots, result refs, admin-ingest source snapshots, cleanup manifests, tombstone refs, replay bundles, audit exports, and deterministic fixtures.
- Storage, queueing, private records, object refs, secret refs, namespace refs, audit, usage, policy, identity, key status, execution, and accounting handoffs must use native Overrid service boundaries such as Overbase, Overqueue, Overstore, Overvault, Universal Namespace Service, Overpass, Overtenant, Overkey, Overwatch, Overmeter, Overguard, Overpack, Oversched, Overlease, Overrun, Encrypted Docdex RAG Adapter, AI Gateway Router, SDK, CLI, and Admin and Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw encrypted-repo content in broad logs/events/metrics/default records, raw encryption keys, direct model routing, final RAG authorization, or direct Docdex internals the adapter boundary.

## Phase 1: SDS Attachment, Phase 6 Scope, And Adapter Boundary

### Work Items

- **1.1 Attach the build plan to SDS #65.**
  - Design: Link this document from the Docdex Adapter SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/adapters/docdex_adapter.md`, `docs/service_catalog/adapters/docdex_adapter.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #65 returns both the Docdex Adapter SDS and this sub-build plan.

- **1.2 Preserve master Phase 6 as the first build point.**
  - Design: Keep first implementation in Phase 6 because Docdex is the first serious data-and-AI product workload after contracts, control plane, execution, policy, metering, and protected refs exist.
  - Output: Phase-gate note that Phases 0, 1, 3, 4, 5, and 8 supply prerequisites; Phase 6 builds the adapter proof; Phase 7/9 later package or promote managed/grid-resident forms; Phase 13 hardens it.
  - Validation: Review proves the plan does not move Docdex Adapter into Phase 8 storage, Phase 12 native apps, or Phase 13-only hardening, and does not reorder master Phase 0 through Phase 13.

- **1.3 Freeze the adapter ownership boundary.**
  - Design: Record that Docdex Adapter owns instance refs, repo bindings, encrypted config refs, workload manifests, job refs, result refs, cleanup/deprovision refs, usage refs, audit refs, and replay bundles.
  - Output: Ownership checklist for architecture, API, implementation, and review gates.
  - Validation: Review confirms the adapter does not own Docdex indexing/search internals, final context authorization, final model routing, model inference, Overrid identities, policy truth, ORU balances, raw keys, or raw unauthorized content.

- **1.4 Carry forward resolved SDS #65 decisions.**
  - Design: Preserve the resolved local/seed Docdex daemon proof, explicit leakage-profile UI, bounded memory/profile/conversation exposure, two-phase deprovision evidence, and disabled encrypted structural feature rules.
  - Output: Resolved-decision checklist covering local/seed connector proof, `encrypted_metadata_only`, accepted `encrypted_term_index_limited`, capability snapshots, service-admin ingest, cleanup evidence, deprovision prepare/execute, and fail-closed structural features.
  - Validation: Review rejects hidden term-index leakage, broad raw memory/profile export, direct encrypted structural AST/symbol/impact support, deletion without dry-run evidence, and premature grid-resident package assumptions.

- **1.5 Define upstream and downstream boundaries.**
  - Design: Record how Docdex daemon/runtime, SDK, CLI, Admin UI, Encrypted Docdex RAG Adapter, Personal AI Assistant, Codali Adapter, Mcoda Adapter, AI Gateway Router, Overpack, Overqueue, Oversched, Overlease, Overrun, Overstore, Overbase, Overvault, Overmeter, Overwatch, Overguard, and Overtenant interact through refs.
  - Output: Consumer-boundary matrix naming allowed inputs, owned outputs, denied direct authority, usage refs, audit refs, and replay requirements.
  - Validation: Review confirms downstream services keep final context authorization, model/resource routing, policy, usage/accounting, protected storage, secret, execution, and user-visible privacy boundaries.

## Phase 2: Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the Docdex Adapter Rust contract module.**
  - Design: Add contract types for Docdex instance refs, repo bindings, encrypted config refs, workload manifests, index/search/retrieval/admin-ingest jobs, result refs, cleanup jobs, usage refs, events, replay bundles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, operation-class enums, capability enums, leakage enums, cleanup enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Docdex runtime internals, Encrypted Docdex RAG authorization, AI Gateway routing, policy enforcement, and accounting internals.

- **2.2 Define instance, repo binding, and encrypted config schemas.**
  - Design: Model `docdex_instance_ref`, `docdex_repo_binding`, and `encrypted_repo_config_ref` with endpoint/service refs, version, health, supported surfaces, tenant visibility, owner scope, repo id, repo root/storage ref, key refs, access bindings, leakage status, caps, disabled features, and state.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and canonical capability fixtures.
  - Validation: Schema tests reject missing tenant/owner scope, raw endpoint secrets, raw key material, unversioned capabilities, missing leakage profile, missing access binding refs, and unclassed encrypted repo metadata.

- **2.3 Define workload, job, result, and usage schemas.**
  - Design: Model `docdex_workload_manifest`, `docdex_index_job`, `docdex_search_job`, `docdex_retrieval_job`, `admin_ingest_job`, `docdex_result_ref`, and `docdex_usage_ref` with actor/tenant/repo refs, operation class, query/request hashes, path scopes, result caps, idempotency, trace id, lifecycle state, storage refs, failure reasons, and usage dimensions.
  - Output: Schema set, lifecycle state machines, redacted examples, BLAKE3 hash examples, stable error catalog, usage fixtures, and failure fixtures.
  - Validation: Tests reject jobs without repo binding, policy refs, trace id, idempotency key, expected result shape, caps, usage refs where required, or redaction/visibility status.

- **2.4 Define cleanup, deprovision, audit, and replay schemas.**
  - Design: Model cleanup/deprovision prepare and execute records with authority, tenant/owner scope, policy/retention/hold checks, pending job drain, key/access revocation plan, export/discard choice, dry-run manifests, deletion receipts, tombstone refs, failed-item refs, usage refs, audit refs, and replay hashes.
  - Output: Cleanup schema, deprovision schema, audit schema, replay bundle schema, valid examples, failure examples, and tombstone fixtures.
  - Validation: Tests reject deletion records without prepare evidence, legal/hold/dispute/support decision refs, cleanup receipts, failed-item refs, usage reconciliation, or replay evidence.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for instance registration, repo binding, encrypted config accepted/denied, index job success, partial index failure, search success, retrieval-only handoff, admin ingest cleanup success/failure, disabled operation denial, key failure, deprovision prepare, deprovision execute, usage reconciliation, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, stable errors, hashes, redacted projections, usage refs, audit refs, cleanup refs, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, cleanup refs, redacted outputs, and replay output across repeated runs.

## Phase 3: Instance Registry, Repo Bindings, Capabilities, And Lifecycle APIs

### Work Items

- **3.1 Implement Docdex instance registration.**
  - Design: Add `POST /docdex/instances` with signed actor/service envelope, tenant visibility, endpoint/service ref, auth mode ref, version, health probe, supported surfaces, trace id, idempotency key, and Overwatch audit refs.
  - Output: Instance registration handler, instance records, idempotency behavior, health/capability placeholder refs, stable errors, and `docdex_adapter.instance_registered` events.
  - Validation: API tests cover valid registration, duplicate idempotency, unavailable instance, unsupported version, missing tenant visibility, missing auth ref, and audience-safe error responses.

- **3.2 Implement instance capability reads and health snapshots.**
  - Design: Add `GET /docdex/instances/{instance_id}/capabilities` and snapshot updates for index, search, retrieval-only RAG, tree/files/stats/repo-inspect, encrypted-search compatibility, admin ingest, limits, and degradation status.
  - Output: Capability snapshot records, health/degraded states, versioned capability refs, visibility projections, and `docdex_adapter.capability_snapshot_updated` events.
  - Validation: Tests prove stale, degraded, missing, or widened capability facts are explicit, versioned, replayable, and cannot be hidden from downstream RAG/router/audit consumers.

- **3.3 Implement repo binding create/import.**
  - Design: Add `POST /docdex/repos` to bind an Overrid tenant/owner scope to a Docdex repo id with repo root/storage ref, access binding refs, encrypted config refs, policy refs, key refs, and lifecycle state.
  - Output: Repo binding handler, binding records, policy/key precheck refs, idempotent import behavior, stable errors, and `docdex_adapter.repo_bound` events.
  - Validation: Tests prove cross-tenant binding is denied, missing owner/access/key refs fail closed, stale repo ids require rebinding, and raw repo paths are not broadly exposed.

- **3.4 Implement repo binding reads and role-scoped projections.**
  - Design: Add `GET /docdex/repos/{repo_binding_id}` with owner, service, RAG adapter, router, admin, and support projections for status, encrypted config, leakage/capability facts, result caps, and degraded operations.
  - Output: Read handler, redacted projections, visibility matrix, support-safe diagnostics, and replay refs.
  - Validation: Tests prove callers cannot see unauthorized repo details, exact private paths where only classes are allowed, raw snippets, raw keys, vault refs, other users' query history, or admin-only instance details.

- **3.5 Implement repo binding lifecycle transitions.**
  - Design: Track repo bindings through `proposed`, `policy_checked`, `key_checked`, `bound`, `active`, `degraded`, `suspended`, `deprovisioning`, and `deprovisioned`.
  - Output: Lifecycle state machine, transition rules, degraded reason codes, suspension behavior, deprovision prerequisites, and event payloads.
  - Validation: Tests prove invalid transitions are rejected, suspended bindings cannot start new jobs, degraded bindings require explicit policy handling, and deprovisioned bindings remain replayable without reopening content.

## Phase 4: Workload Manifests, Index/Search/Retrieval Jobs, And Connector Execution

### Work Items

- **4.1 Implement workload manifest validation.**
  - Design: Add manifest validation for operation class, repo binding, paths/query/context metadata, expected result shape, queue/runtime needs, result caps, idempotency key, trace id, policy refs, key refs, and encrypted capability constraints.
  - Output: Manifest validator, validation report refs, stable `manifest_invalid`, `operation_disabled_for_encrypted_repo`, `retrieval_limit_exceeded`, and `policy_denied` behavior.
  - Validation: Tests prove validation cannot queue work, call Docdex, materialize content, create artifacts, or bypass policy/key/capability checks.

- **4.2 Implement index job submission and status.**
  - Design: Add `POST /docdex/index-jobs` and `GET /docdex/index-jobs/{job_id}` with repo binding, path scopes, expected counts, connector or Overpack/Overqueue execution profile, cancellation/timeout policy, and result refs.
  - Output: Index job handler, status handler, lifecycle records, path-count refs, failed-path refs, error refs, retry records, and `docdex_adapter.index_job_submitted` and `docdex_adapter.index_job_completed` events.
  - Validation: Tests cover valid index, duplicate idempotency, partial failure, retryable failure, terminal failure, operation denied, cancellation, usage emission, and redacted error views.

- **4.3 Implement search job submission and result refs.**
  - Design: Add `POST /docdex/search-jobs` with query hash, bounded parameters, include_libs flag where allowed, result limits, encrypted capability refs, leakage refs, and redaction/visibility requirements.
  - Output: Search job handler, ranked hit refs, snippet refs or snippet-hash refs, stats refs, error refs, lifecycle records, and `docdex_adapter.search_job_completed` events.
  - Validation: Tests prove search results are bounded, role-scoped, redacted, hashable, and never leak unauthorized raw snippets, hidden exact paths, or disabled encrypted structural facts.

- **4.4 Implement retrieval-only RAG job submission.**
  - Design: Add `POST /docdex/retrieval-jobs` for retrieval-only work that returns result refs and handoff refs to Encrypted Docdex RAG Adapter rather than final model-ready context bundles.
  - Output: Retrieval job handler, retrieval result refs, capability/leakage refs, consumer refs, handoff refs, lifecycle records, and `docdex_adapter.retrieval_job_completed` events.
  - Validation: Tests prove the adapter does not authorize final context bundles, choose models, run inference, bypass Encrypted Docdex RAG Adapter, or expose raw unauthorized private content.

- **4.5 Preserve the final interface across connector and grid paths.**
  - Design: Route early Phase 6 calls to a local/seed Docdex daemon through a Rust connector while preserving the same signed, tenant-scoped, idempotent job envelopes and result refs expected by later Overpack/Overqueue/grid paths.
  - Output: Connector execution profile, Overpack-ready manifest shape, queue/deferred execution switch points, result normalization, and replay refs.
  - Validation: Integration tests prove local connector execution and future queue execution produce the same public job states, stable errors, result refs, usage refs, and replay shape.

## Phase 5: Encrypted Capabilities, Leakage Profiles, Disabled Operations, And Fail-Closed Behavior

### Work Items

- **5.1 Surface encrypted repo capability snapshots.**
  - Design: Capture encryption mode, key status, shared-token status, plaintext term-index status, semantic/web/full-open flags, max result/snippet caps, disabled features, and residual leakage notes from Docdex.
  - Output: Capability snapshot updater, leakage fact records, downstream visibility refs, and `docdex_adapter.capability_snapshot_updated` events.
  - Validation: Tests prove changed capabilities are versioned, active jobs record the snapshot used, and downstream grants/routes reauthorize or fail closed when facts weaken or widen leakage.

- **5.2 Implement explicit leakage profile projection.**
  - Design: Represent `encrypted_metadata_only` and accepted `encrypted_term_index_limited` with owner scope, data class, leakage class, visible path/token/result classes, caps, disabled features, retention/expiry, and affected operations.
  - Output: Leakage profile records, user/admin/router/RAG adapter summaries, redacted projections, and replay refs.
  - Validation: Tests prove plaintext term-index leakage is visible to users, Encrypted Docdex RAG Adapter, AI Gateway Router, Personal AI Assistant privacy audit, Overwatch, and replay.

- **5.3 Enforce disabled structural feature handling.**
  - Design: Fail closed for encrypted repo AST extraction, symbol indexes, dependency or impact graphs, broad tree/path enumeration, full-open reads, raw index export, semantic/web expansion outside scope, batch/rerank/debug surfaces that materialize unauthorized plaintext, and any future unsupported structural feature.
  - Output: Disabled operation matrix, stable `operation_disabled_for_encrypted_repo` errors, policy inputs, and test fixtures.
  - Validation: Tests prove disabled operations cannot be simulated through unauthorized plaintext, cannot be hidden as degraded success, and require explicit capability snapshots before any safe future support.

- **5.4 Implement key, access, and retention failure gates.**
  - Design: Validate Overvault key refs, Overkey status, access bindings, tenant/repo membership, retention class, deletion hold, and policy facts before encrypted operations start.
  - Output: Key/access checker, failure reason mapping, degraded/suspended state transitions, audit refs, and user-safe denial projections.
  - Validation: Tests prove missing/unavailable/revoked keys, stale access bindings, retention conflicts, legal/support holds, and policy denials stop jobs before side effects.

- **5.5 Propagate capability and leakage changes downstream.**
  - Design: Notify Encrypted Docdex RAG Adapter, AI Gateway Router, Personal AI Assistant, Codali Adapter, Mcoda Adapter, SDK, CLI, Admin UI, Overwatch, and Overmeter when capability/leakage/key/access facts change.
  - Output: Downstream event mapping, invalidation refs, reauthorization requirements, degraded-grant behavior, and replay links.
  - Validation: Tests prove downstream consumers cannot reuse stale stronger capabilities, suppress leakage changes, retain revoked result refs, or continue active routes without reauthorization where required.

## Phase 6: Admin Ingest, Cleanup Evidence, Deprovision, And Lifecycle Recovery

### Work Items

- **6.1 Implement service-admin ingest jobs.**
  - Design: Add `POST /docdex/admin-ingest-jobs` for approved service-admin document ingest into managed encrypted repos with source refs, target repo binding, encoding class, cleanup policy, trace id, idempotency key, and audit refs.
  - Output: Admin ingest handler, ingest lifecycle records, cleanup requirements, source staging refs, result refs, and `docdex_adapter.admin_ingest_completed` events.
  - Validation: Tests prove service-admin authority is required, source refs are bounded, raw plaintext is not retained beyond policy, unauthorized admins are denied, and failed ingest records are replayable.

- **6.2 Implement plaintext cleanup verification.**
  - Design: Verify removal or avoidance of durable plaintext source artifacts after encrypted indexing where Docdex mode requires cleanup, and fail closed when cleanup cannot be proven.
  - Output: Cleanup verification worker, cleanup evidence refs, cleanup-failed state, tombstone refs where applicable, and support/operator projections.
  - Validation: Tests prove cleanup failures block completed status, preserve evidence, trigger alerts, reconcile usage, and prevent broad result visibility.

- **6.3 Implement deprovision prepare.**
  - Design: Add a prepare phase that records signed authority, tenant/owner scope, repo binding id, policy/retention/legal/support/dispute decisions, pending job cancellation/drain state, key/access revocation plan, export/discard choice, usage reconciliation, and dry-run removal manifest.
  - Output: Deprovision prepare handler, dry-run manifest refs, pending job records, hold/reason refs, export/discard refs, and replay refs.
  - Validation: Tests prove deprovision cannot execute without prepare evidence, unresolved holds keep state `suspended` or `deprovisioning`, and dry-run manifests do not delete content.

- **6.4 Implement deprovision execute.**
  - Design: Execute allowed repo root, state root, encrypted index material, or access-binding cleanup with deletion receipts, BLAKE3 hashes or tombstone refs, Overstore/Overvault cleanup refs, failed-item refs, usage/audit refs, and replay evidence.
  - Output: Deprovision execute handler, cleanup receipts, tombstones, failed-item records, terminal states, and `docdex_adapter.repo_deprovisioned` events.
  - Validation: Tests prove failed-item refs keep the binding out of `deprovisioned`, deletion evidence is role-scoped, usage is reconciled, and replay remains possible without reopening removed content.

- **6.5 Implement lifecycle recovery for stuck jobs.**
  - Design: Recover index/search/retrieval/admin-ingest/deprovision jobs stuck in retryable, cleanup-failed, usage-pending, key-failed, instance-unavailable, or cancellation states.
  - Output: Recovery worker, retry/dead-letter records, operator action refs, alert inputs, support-safe projections, and reconciliation reports.
  - Validation: Tests prove recovery never retries with weaker policy/key/leakage constraints, never drops audit evidence, and separates retryable, terminal, cancelled, cleanup-failed, and support-required states.

## Phase 7: RAG Handoff, Usage, Audit, Replay, SDK, CLI, And Admin Diagnostics

### Work Items

- **7.1 Implement Encrypted Docdex RAG Adapter handoff.**
  - Design: Provide repo binding refs, capability snapshots, leakage profiles, retrieval job refs, result refs, citation/snippet-hash refs, usage refs, and replay refs to Encrypted Docdex RAG Adapter without authorizing final context bundles.
  - Output: Handoff contract, retrieval result projections, denied/degraded refs, stable errors, and downstream event records.
  - Validation: Tests prove final context grants, context bundles, route-bound retention, and AI prompt inclusion remain owned by Encrypted Docdex RAG Adapter and downstream AI layers.

- **7.2 Emit operation-level usage refs.**
  - Design: Emit usage refs for instance/capability checks where material, index jobs, search jobs, retrieval jobs, admin ingest, stats/files/tree calls where material, storage refs, retries, cleanup, deprovision, denied attempts, and failed jobs.
  - Output: Overmeter integration, `docdex_adapter.usage_emitted` events, operation classes, usage reconciliation records, and downstream Wallet/Usage Center or ORU Account Service handoff refs.
  - Validation: Tests prove usage links tenant, owner, actor/service account, repo binding, job id, operation class, result refs, downstream context/route refs, retries, failures, and cleanup without maintaining balances in the adapter.

- **7.3 Emit Overwatch audit evidence.**
  - Design: Record signed audit refs for instance registration, repo binding, capability snapshot updates, manifest validation, job submission, connector execution, result capture, admin ingest, cleanup, deprovision, usage, and replay reads.
  - Output: Audit event mapping, Overwatch refs, redacted projections, and immutable decision evidence.
  - Validation: Tests prove every mutating API and terminal state has an audit ref and audit events omit raw encrypted repo content, raw keys, unredacted snippets, raw private source documents, and unauthorized queries.

- **7.4 Implement replay API.**
  - Design: Add `GET /docdex/replay/{job_id}` to reconstruct manifest, policy, key/capability/leakage facts, execution path, result refs, cleanup refs, deprovision refs, usage refs, and audit decisions.
  - Output: Replay assembler, replay bundle schema, role-scoped projections, pagination for linked records, and deterministic replay fixtures.
  - Validation: Tests prove replay is deterministic, redacted, authorization-aware, complete enough for dispute/security/support review, and does not expose private content outside allowed scopes.

- **7.5 Implement SDK, CLI, and admin diagnostics.**
  - Design: Provide SDK/CLI/admin commands for instance registration, repo binding, capability snapshots, repo status, index/search/retrieval/admin-ingest jobs, cleanup/deprovision status, result refs, usage, diagnostics, and replay summaries.
  - Output: Generated Rust SDK bindings, TypeScript/web bindings where required for clients, CLI commands, stable JSON output, admin projections, error decoding, and docs-facing examples.
  - Validation: Contract tests prove commands preserve signing, tenant scope, idempotency, trace ids, stable errors, redaction, role-scoped projections, and no direct internal API calls.

## Phase 8: Phase 6 Local/Seed Connector Proof And Product Experience

### Work Items

- **8.1 Implement the first local/seed Docdex connector proof.**
  - Design: Use a local Docdex daemon on founder seed/private hardware through the Rust connector while preserving Overrid-shaped signed, tenant-scoped, idempotent job envelopes, result refs, Overvault key refs, Overmeter usage refs, and Overwatch evidence.
  - Output: Connector proof profile, instance registration fixture, repo binding fixture, index/search/retrieval/admin-ingest fixtures, and replay fixture.
  - Validation: End-to-end test proves a real Docdex operation can be submitted through SDK/CLI and observed through adapter job/result/usage/audit refs without direct internal API calls.

- **8.2 Prove encrypted index and search flows.**
  - Design: Run a bounded encrypted repo index job and search job with explicit leakage profile, result caps, disabled structural feature flags, key refs, access binding refs, and redacted results.
  - Output: Index proof record, search proof record, capability snapshot, leakage profile, result refs, usage refs, audit refs, and replay bundle.
  - Validation: Test proves encrypted repo operations enforce access checks, key refs, result caps, disabled feature denials, and redaction while preserving user-visible leakage facts.

- **8.3 Prove retrieval-only handoff flow.**
  - Design: Run a retrieval-only job that hands bounded result refs to Encrypted Docdex RAG Adapter for authorization and bundle creation rather than making Docdex Adapter a context-authorizing service.
  - Output: Retrieval proof record, RAG handoff refs, result/citation/snippet-hash refs, usage refs, audit refs, and replay bundle.
  - Validation: Test proves Docdex Adapter does not build final model context, choose routes, run inference, or bypass RAG grants; downstream authorization remains visible and replayable.

- **8.4 Prove admin-ingest and cleanup flow.**
  - Design: Run a managed encrypted repo service-admin ingest with source refs, target repo binding, cleanup policy, plaintext cleanup verification, result refs, and failure behavior.
  - Output: Admin-ingest proof record, cleanup evidence, cleanup-failed fixture, usage refs, audit refs, and replay bundle.
  - Validation: Test proves cleanup success reaches completed state, cleanup failure blocks completed state, and support/operator views receive evidence without broad plaintext exposure.

- **8.5 Prove product-facing SDK, CLI, and admin experience.**
  - Design: Exercise SDK/CLI/admin flows for instance status, repo binding, index/search/retrieval jobs, degraded operations, usage, replay, and troubleshooting.
  - Output: Product proof checklist, CLI JSON examples, admin projection examples, degraded-state examples, and support-safe diagnostics.
  - Validation: Review confirms product users can submit jobs, retrieve refs, see usage, inspect failures, and debug capabilities without manually calling internal APIs or violating stack guardrails.

## Phase 9: Grid-Resident Readiness, Security, Privacy, Incident, And Compliance Hardening

### Work Items

- **9.1 Prepare grid-resident Docdex package readiness.**
  - Design: Define later Phase 7/9 readiness for managed or grid-resident Docdex packages after system-service workload class, package validation, backup/restore, release strategy, and failover are proven.
  - Output: Package readiness checklist, Overpack manifest requirements, backup/restore refs, migration requirements, health/readiness commands, rollout/rollback requirements, and blocked-until gates.
  - Validation: Review proves Phase 6 does not pretend Docdex is already a grid-resident system service and later package promotion uses Phase 7/9 gates without changing adapter APIs.

- **9.2 Harden encrypted repo and key failure behavior.**
  - Design: Threat-model missing/invalid/revoked keys, access binding drift, leakage-profile widening, disabled operation bypass, stale capabilities, stale indexes, shared-token misuse, and unauthorized repo binding access.
  - Output: Threat model records, mitigation checklist, policy gates, alert inputs, tests, and accepted-risk records.
  - Validation: Security tests prove failures deny or suspend safely, active consumers reauthorize, raw keys never appear in adapter records, and disabled operations cannot be bypassed.

- **9.3 Harden admin ingest, cleanup, and deprovision.**
  - Design: Threat-model plaintext source retention, cleanup evidence forgery, deprovision without holds, partial deletion, failed cleanup, tombstone tampering, support/legal/dispute conflicts, and replay gaps.
  - Output: Cleanup hardening checklist, two-phase deprovision policy, tombstone integrity checks, incident hooks, and compliance export refs.
  - Validation: Tests prove unresolved cleanup or hold conditions block final deletion, tampering is detected by hashes/receipts, and replay can explain cleanup/deprovision outcomes.

- **9.4 Harden observability, retention, and privacy.**
  - Design: Expose counts and latency for index/search/retrieval/admin-ingest/deprovision work while redacting raw queries, snippets, paths, source docs, key refs, and private content according to audience and retention class.
  - Output: Metrics, tracing spans, alert rules, redaction profiles, retention policy config, support-safe dashboards, and Overwatch links.
  - Validation: Tests prove diagnostics are useful but role-scoped, raw private content is absent from broad logs/events/metrics, retention/deletion states remain replayable, and support exports are redacted.

- **9.5 Harden incident and compliance response.**
  - Design: Add incident playbooks for Docdex instance compromise, unauthorized repo binding, key leak attempt, plaintext cleanup failure, result leakage, disabled operation bypass, deprovision failure, usage emission failure, and replay/audit gaps.
  - Output: Incident playbooks, escalation records, route/job freeze behavior, affected-owner notification refs, correction/retraction workflows, compliance export projections, and remediation tracking refs.
  - Validation: Drills prove incidents preserve evidence, stop unsafe jobs, quarantine affected refs, reconcile usage, notify owning services/users where policy requires, and produce compliance-safe reports.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate SDS #65 build-breakdown coverage.**
  - Design: Map each SDS build-breakdown item to sub-build phases covering schemas, instance/repo APIs, job submission, capability snapshots, leakage facts, disabled operations, admin ingest, cleanup evidence, RAG handoff, usage, replay, diagnostics, and hardening.
  - Output: Coverage checklist in review notes and implementation handoff records.
  - Validation: Review proves no SDS #65 build-breakdown item is missing and the plan preserves Docdex Adapter as a Phase 6 product-integration service.

- **10.2 Validate structure and work-item quality.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, five work items per phase, Design/Output/Validation bullets, final newline, and no tab/format drift.
  - Output: Focused validation script evidence for this file.
  - Validation: Script passes for `SUB BUILD PLAN #65`, attached SDS link, phase headings 1 through 10, 50 work items, and complete work-item structure.

- **10.3 Validate links and source alignment.**
  - Design: Check local Markdown links across this plan, the SDS, service catalog entry, master plan, crosswalk, Phase 6, Phase 7, Phase 8, Phase 9, Phase 12, Phase 13, progress docs, and queue docs.
  - Output: Link-check evidence and corrected backlinks where needed.
  - Validation: Link checker reports no missing local targets and Docdex search returns aligned SDS/service/sub-build-plan/crosswalk results.

- **10.4 Validate tech-stack guardrails.**
  - Design: Scan the changed docs for accepted Rust-first, canonical JSON/JSON Schema, signed envelope, Ed25519, BLAKE3, native Overrid boundary, no conventional database/queue/object-store/vault/KMS, no Kubernetes-first, no blockchain/NFT, no pricing/revenue/customer-count, no raw key/content leakage, no final-model-routing ownership, and no direct Docdex-internals drift.
  - Output: Guardrail scan evidence and any required wording fixes.
  - Validation: Scan output contains only accepted tech-stack language, native Overrid service names, or explicit non-goal/authority-boundary statements.

- **10.5 Update queue, progress, index, and handoff evidence.**
  - Design: Mark `065-build-plan` complete, update progress docs, run targeted Docdex index refresh, run retrieval checks, record the `docdexd run-tests` blocker if still present, and save repo memory.
  - Output: Updated `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, `docs/build_plan/progress.md`, Docdex index/search evidence, and implementation handoff note.
  - Validation: Queue JSON validates, next incomplete build-plan task is `066-build-plan`, Docdex search finds the new sub-build plan with SDS/service backlinks, and repo-wide test execution status is recorded.

## Alignment Review

- SDS #65 already aligns with master Phase 6 as the first build point for Docdex instance/repo binding, encrypted index/search/retrieval/admin-ingest jobs, capability snapshots, result refs, usage refs, audit refs, and replay bundles.
- The SDS correctly separates Docdex Adapter from Encrypted Docdex RAG Adapter: Docdex Adapter returns capability/result/handoff refs, while Encrypted Docdex RAG Adapter authorizes final context grants and bundles for AI flows.
- Phase 7 and Phase 9 remain later package/grid-resident readiness gates; this plan does not pretend a local/seed Docdex daemon is already a grid-resident system service.
- Phase 12 remains downstream native-app and personal AI consumption, not Docdex Adapter's first build phase.
- Phase 13 remains governance, compliance, security, privacy, cleanup, deprovision, incident, retention, replay, and scale hardening rather than the first delivery point.
- The master Phase 0 through Phase 13 order remains unchanged. Required alignment updates are backlinks/index rows for SDS #65, Phase 6 wording that distinguishes Docdex Adapter jobs from Encrypted Docdex RAG authorization, Phase 13 hardening wording for Docdex Adapter, and local-stack wording cleanup to match `docs/overrid_tech_stack_choice.md`.
