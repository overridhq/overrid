# SUB BUILD PLAN #44 - Seal Ledger

Attached SDS: [docs/sds/accounting/seal_ledger.md](../sds/accounting/seal_ledger.md)

## Purpose

This sub-build plan turns SDS #44 into an implementation sequence for Seal Ledger. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Seal Ledger is the append-only accounting truth layer for ORU reservations, settlements, holds, releases, refunds, corrections, provider earnings, grant allocations, native service usage, system-service usage, dispute refs, external payment refs, migration imports, checkpoints, replay, reconciliation, and audit exports. It must preserve signed evidence and deterministic replay while avoiding mutable balance tables, blockchain consensus, mining, NFTs, speculative token mechanics, gas fees, and per-operation external payment calls.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #44: Seal Ledger](../sds/accounting/seal_ledger.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, accounting boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Seal Ledger plan](../service_catalog/accounting/seal_ledger.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical schemas, signed command envelopes, idempotency keys, trace ids, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies identity, tenant, service-account, signing-key, Overgate ingress, Overwatch audit, Overregistry refs, and command discipline prerequisites. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies workload execution refs, Overmeter raw usage facts, provider refs, workload refs, and settlement prerequisites. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy refs, Overclaim dispute/finality refs, Oververify trust facts, challenge evidence, and policy dry-run prerequisites. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Controls the first build point: append-only accounting entries, reservations, settlements, holds, releases, refunds, corrections, provider earnings, grants, native/system-service usage, checkpoints, reconciliation, replay, and audit exports. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service operation, backup/restore hooks, failover readiness, checkpoint promotion, maintenance controls, and grid-resident hardening. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase persistence, Overstore replay/export artifacts, Overvault private refs, retention, migration handoffs, and backup/restore storage paths. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies later cross-tenant grant, public-interest, and federation contexts that may create explicit bridge/import refs rather than changing Phase 5 ledger semantics. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider earning, hold, throttle, challenge, fraud, and anti-Sybil contexts that remain source refs consumed by Seal Ledger, not ledger-owned risk decisions. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies Wallet and Usage Center, admin/developer UI, SDK, CLI, native apps, and stewardship views that consume redacted ledger-derived views through owner services. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance boundaries, incident handoffs, threat-model reviews, stewardship reports, audit exports, retention controls, migration governance, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #44 first build work aligned to master Phase 5, with Phase 7 backup/restore/grid-resident promotion, Phase 8 native persistence, and later public/native/governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 7, 8, 10, 11, 12, and 13 | Attach SDS #44, freeze Seal Ledger authority, preserve Phase 5 as the first build point, and record later restore/persistence/federation/public/client/governance gates. |
| 2 | Master Phases 0, 1, and 5 | Build Rust contracts, canonical schemas, entry-type catalogs, source-ref catalogs, reason codes, state machines, and deterministic fixtures. |
| 3 | Master Phases 1, 5, and 7 | Implement tenant/system stream partitioning, writer authority, idempotency, expected-sequence control, stream epochs, segment metadata, and split-brain prevention. |
| 4 | Master Phase 5, with prerequisites from Phases 0, 1, 3, and 4 | Implement authorized append and append-batch APIs with atomicity, validation, denial evidence, and append-only events. |
| 5 | Master Phase 5 | Implement Phase 5 entry types, paired debit/credit-style accounting rules, indexes, query APIs, ORU derivation inputs, provider earning refs, grant refs, native/system-service usage refs, and external payment refs as references only. |
| 6 | Master Phase 5, with Phase 7 and Phase 13 hardening | Implement checkpoints, independent verification readiness, replay, audit exports, redaction profiles, checksum/signature evidence, and tamper detection. |
| 7 | Master Phases 4 and 5, with Phase 13 governance | Implement reconciliation, correction-by-new-entry, dispute/refund/reversal handoffs, operator-signed corrections, and downstream derivation contracts. |
| 8 | Master Phases 7, 8, and 13 | Prepare backup/restore verification hooks, migration_import batches, founder-hardware cutover, grid-resident failover promotion, native persistence, and retention controls. |
| 9 | Master Phases 6, 7, 8, 11, 12, and 13 | Add operations, lag metrics, redacted consumer views, export/reporting handoffs, public-provider source-ref readiness, native persistence adapters, and stewardship visibility. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Seal Ledger core is a Rust service/module using shared contract crates, Tokio for bounded async append/checkpoint/reconciliation workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Ledger streams, entries, entry batches, source evidence refs, account refs, index cursors, checkpoints, reconciliation reports, audit exports, migration imports, API objects, events, fixtures, reason-code catalogs, redaction profiles, and replay bundles use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating endpoints require signed service envelopes, accepted producer authority, tenant or system scope, trace id, idempotency key, schema version, source evidence refs, policy refs, expected sequence when required, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for entry fingerprints, checkpoint ranges, export bundles, migration snapshots, replay proofs, schema fixtures, and deterministic comparison tests.
- Seal Ledger may later persist streams through Overbase, replay/export artifacts through Overstore, and private refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative markets, payment processors, or external payment providers the platform boundary.
- Seal Ledger records external payment credit/debit refs only as accounting references supplied by Overbill or approved owner services. It must never trigger external payment-provider calls, create invoices/receipts, adjudicate disputes, score trust, calculate pricing/reference bands, mutate ORU balance truth, or expose mutable wallet counters as accounting truth.
- Planning and implementation must avoid mutable historical edits, delete-and-rewrite repair flows, global-ledger bottlenecks, account-per-stream fragmentation, raw secret storage, private workload evidence leakage, public blockchain language, speculative ownership, revenue projections, customer-count assumptions, direct payment execution, and per-operation external payment calls.

## Phase 1: SDS Attachment, Ledger Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #44.**
  - Design: Link this document from the Seal Ledger SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/accounting/seal_ledger.md`, `docs/service_catalog/accounting/seal_ledger.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #44 returns both the Seal Ledger SDS and this sub-build plan.

- **1.2 Freeze Seal Ledger authority boundaries.**
  - Design: Record that Seal Ledger owns append-only streams, ledger entries, entry batches, sequence invariants, producer validation, idempotency, source evidence refs, checkpoints, indexes, reconciliation reports, replay proofs, audit exports, correction-by-new-entry, and migration_import records.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Seal Ledger does not own mutable wallet state, ORU balance projection, pricing/reference bands, raw usage measurement, invoices, receipts, payment-provider calls, payout execution, dispute adjudication, trust scoring, scheduling, backup execution, or raw secret storage.

- **1.3 Preserve master Phase 5 as the first build point.**
  - Design: Keep first implementation in Phase 5 after Phase 0 contracts, Phase 1 identity/audit, Phase 3 usage facts, and Phase 4 policy/dispute prerequisites exist.
  - Output: Phase-gate note that Phase 5 builds private accounting truth, Phase 7 promotes restore/failover readiness, Phase 8 moves persistence into native primitives, Phase 10/11 add federation/public source contexts, Phase 12 exposes client views through owner services, and Phase 13 hardens governance.
  - Validation: Review proves this plan does not move backup/restore finality, public-provider risk decisions, native persistence, or governance hardening into the Phase 5 minimum.

- **1.4 Carry forward resolved SDS #44 decisions.**
  - Design: Preserve tenant-scoped streams plus limited system_scope streams, stream sequence order, expected-sequence checks, BLAKE3 checkpoint ranges, paired debit/credit-style entries for value movement, redacted audit bundles, and migration_import cutovers.
  - Output: Resolved-decision checklist tied to contract and implementation reviews.
  - Validation: Review rejects one global stream, account-per-stream fragmentation, blockchain consensus, mining, mutable repair, raw private workload export, and founder-hardware cutover by historical edits.

- **1.5 Define producer, owner-service, and consumer boundaries.**
  - Design: Create a dependency matrix for Overmeter, ORU Account Service, Overguard, Overclaim, Overbill, Overgrant, Provider Payout Service, Overwatch, Overkey, Backup and Restore Service, Wallet and Usage Center, admin UI, SDK, CLI, central AI stewardship, and compliance tools.
  - Output: Boundary matrix listing accepted entry types, source refs, signing authority, final owner, redaction class, replay evidence, downstream consumer, and later phase gate.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, trace ids, policy refs, and Overwatch audit rather than direct privileged state reads or mutable shared tables.

## Phase 2: Rust Contracts, Ledger Schemas, Reason Codes, And Fixtures

### Work Items

- **2.1 Create the Seal Ledger Rust contract module.**
  - Design: Add contract types for ledger streams, ledger entries, entry batches, account refs, source evidence refs, indexes, checkpoint records, reconciliation reports, audit exports, correction refs, migration imports, API errors, events, redaction profiles, and replay bundles.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, entry-type enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from ORU Account Service, Overmeter, Overbill, Overgrant, Overclaim, Provider Payout Service, Backup and Restore Service, Overbase, Overstore, and Overvault internals.

- **2.2 Define canonical ledger schemas and examples.**
  - Design: Add versioned schemas for stream creation, append command, append-batch command, ledger entry, entry query, checkpoint creation, reconciliation request/report, correction command, replay response, audit export, and migration_import batch.
  - Output: Schema files, valid examples, invalid examples, generated validators, compatibility notes, and docs-facing examples.
  - Validation: Schema tests reject missing stream id, tenant/system scope, trace id, idempotency key, schema version, entry type, source evidence refs, account refs where required, policy refs where required, expected sequence where required, or reason codes where required.

- **2.3 Define entry-type and transition catalogs.**
  - Design: Encode reservation, settlement, hold, release, refund, correction, provider_earning, grant_allocation, native_service_usage, system_service_usage, external_payment_credit_ref, external_payment_debit_ref, migration_import, checkpoint, audit_export marker, reconciliation marker, and zero-amount evidence marker rules.
  - Output: Entry-type catalog, validation matrix, required source refs, required paired-entry behavior, state-transition refs, invalid transition fixtures, and review notes.
  - Validation: Tests reject unsupported entry types, value-moving singletons without an allowed exception, zero-amount records that move value, payment refs that imply payment execution, and corrections that do not cite prior entries plus finality/evidence refs.

- **2.4 Model source-ref and envelope discipline.**
  - Design: Define source refs for Overmeter rollups, Overguard decisions, Overclaim finality, Overgrant authorizations, Overbill document/payment refs, Provider Payout refs, ORU account refs, operator actions, migration snapshots, Backup and Restore verification refs, Overwatch audit refs, and Overkey signer refs.
  - Output: Source-ref catalog, accepted freshness markers, owner-service map, evidence envelope rules, error catalog, and fixture references.
  - Validation: Tests reject unsigned, stale, wrong-scope, wrong-owner, missing-finality, missing-policy, missing-account, duplicate, or raw-secret-bearing refs with stable reason codes.

- **2.5 Create deterministic ledger fixtures.**
  - Design: Build fixtures for reservation, settlement, hold, release, refund, correction, provider earning, grant allocation, native service usage, system-service usage, external payment refs, sequence conflicts, duplicate idempotency, partial batch failure, checkpoint creation, reconciliation mismatch, audit export, and migration import.
  - Output: Fixture directory, expected entries, expected indexes, reason codes, Overwatch events, checkpoint hashes, replay examples, export examples, redacted views, and invalid examples.
  - Validation: Fixture tests produce stable entry ids, batch ids, sequence numbers, checkpoint hashes, index keys, denial reason codes, replay bundles, and idempotency outcomes across repeated runs.

## Phase 3: Stream Partitioning, Idempotency, And Sequence Control

### Work Items

- **3.1 Implement tenant and system stream definitions.**
  - Design: Create tenant-scoped streams plus a small number of system_scope streams with explicit stream ids, schema versions, checkpoint policy, retention policy, writer key refs, stream epoch, segment id, and previous checkpoint metadata.
  - Output: Stream registry model, stream creation contract, writer authority map, stream epoch records, segment metadata, and stream state queries.
  - Validation: Tests prove Phase 5 avoids one global stream, avoids account-per-stream fragmentation, and rejects appends to unknown, closed, wrong-scope, wrong-epoch, or unauthorized streams.

- **3.2 Implement idempotency storage and duplicate resolution.**
  - Design: Store idempotency keys by producer, scope, command ref, stream, entry type, source evidence ref, and expected result.
  - Output: Idempotency resolver, duplicate result projection, conflict reason codes, and audit events.
  - Validation: Tests prove duplicate append and append-batch commands return the original entry/batch refs, while same keys with different payloads reject with a stable conflict reason.

- **3.3 Implement expected-sequence control.**
  - Design: Require expected stream sequence for append paths that must protect order, reserve sequence ranges for batches, and reject or retry on stream head mismatch.
  - Output: Sequence head read, expected-sequence validator, range reservation model, conflict records, and retry guidance.
  - Validation: Concurrency tests prove out-of-order writes, split-brain writer attempts, stale stream heads, duplicate range reservations, and partial sequence writes cannot overwrite history.

- **3.4 Implement writer key and producer authority checks.**
  - Design: Bind each producer service account to allowed entry types, tenant/system scopes, writer key refs, signing key metadata, and source-ref requirements.
  - Output: Producer authority table, writer key verification, entry-type allowlist, denial events, and Overwatch audit refs.
  - Validation: Tests prove Overmeter can submit usage settlement candidates, Overclaim can drive hold/release/refund/correction refs, Overbill can supply external payment refs, Overgrant can supply grant refs, Provider Payout can supply payout refs, and unauthorized producers cannot append outside their authority.

- **3.5 Implement stream close, tombstone, and migration readiness.**
  - Design: Model read-only/tombstoned streams for cutover, restore, or migration while preserving audit replay and preventing new normal writes.
  - Output: Stream close records, tombstone refs, replacement stream refs, migration source metadata, and replay continuity notes.
  - Validation: Tests prove closed streams reject new appends, remain queryable/replayable, and can be cited by migration_import records without editing historical entries.

## Phase 4: Authorized Append And Atomic Batch APIs

### Work Items

- **4.1 Implement `POST /ledger/append`.**
  - Design: Validate signed envelope, producer authority, stream state, entry type, account refs, source evidence refs, idempotency key, expected sequence, paired-entry requirements, policy refs, and redaction class before writing.
  - Output: Append endpoint, validator pipeline, denial evidence, durable entry write, index scheduling hook, and `seal_ledger.entry_appended` or `seal_ledger.append_denied` events.
  - Validation: API tests prove valid appends commit once, invalid appends create denial evidence, duplicate appends return the original result, and no historical entry can be updated in place.

- **4.2 Implement `POST /ledger/append-batch`.**
  - Design: Validate atomic groups of entries where a transition needs multiple account-side records, reserve sequence ranges, and fail all-or-nothing when any entry is invalid.
  - Output: Batch endpoint, batch validator, sequence range reservation, batch result model, rollback-free rejection behavior, and `seal_ledger.batch_appended` events.
  - Validation: Tests prove partial batch failure appends nothing, duplicate batch idempotency returns original refs, and cross-account tenant/system transitions remain balanced through paired entries.

- **4.3 Implement durable append and index scheduling.**
  - Design: Persist entries before index updates, record append state, and make index rebuild possible from stream truth after any index failure.
  - Output: Append storage abstraction, append state model, index lag marker, rebuild cursor, and event emission discipline.
  - Validation: Failure tests prove index update failure after durable append marks lag and rebuilds from stream without reappending or changing the entry sequence.

- **4.4 Implement rejection evidence and stable errors.**
  - Design: Return stable denial reasons for producer_not_authorized, account_ref_invalid, source_evidence_missing, expected_sequence_mismatch, idempotency_conflict, paired_entry_required, batch_validation_failed, stream_closed, and schema_version_unsupported.
  - Output: Error catalog, denial event schema, redacted internal details, public-safe reason mapping, and operator diagnostics.
  - Validation: Tests prove every rejection emits a traceable denial event, cites safe reason codes, and never leaks private workload details, raw secrets, or other-tenant evidence.

- **4.5 Implement append read-after-write guarantees.**
  - Design: Make newly appended entries and batches readable by id immediately, with indexes allowed to lag behind stream truth.
  - Output: Entry lookup path, batch lookup path, index-lag status, checkpoint coverage field, and redaction-aware response model.
  - Validation: Tests prove entry id reads work before secondary indexes catch up, role-filtered redaction applies, and checkpoint status is absent or pending until a checkpoint covers the range.

## Phase 5: Phase 5 Entry Types, Paired Accounting, And Indexes

### Work Items

- **5.1 Implement reservation, settlement, hold, release, refund, and correction entries.**
  - Design: Encode ORU movement between available, reserved, held, spent, earned, refunded, and corrected states through append-only entries and paired-accounting rules.
  - Output: Entry validators, paired-entry builder, state-transition refs, account refs, source refs, and query fixtures.
  - Validation: Tests prove balance-affecting transitions are paired, historical entries are not edited, corrections cite prior entries, and ORU Account Service can derive projections from the ledger.

- **5.2 Implement provider earning and payout-linked refs.**
  - Design: Record provider_earning entries and later Provider Payout refs as ledger facts without letting Seal Ledger execute payouts or own provider eligibility.
  - Output: Provider earning entry schema, payout source-ref slots, hold/release links, correction links, and provider/workload indexes.
  - Validation: Tests prove Provider Payout Service derives earning and hold views from entries, payout results create refs/corrections only, and Seal Ledger never becomes the payout processor.

- **5.3 Implement grant allocation, native-service usage, and system-service usage entries.**
  - Design: Record grant source/authorization usage, native service usage, and system service usage by explicit resource dimensions and source owner refs.
  - Output: Grant allocation entries, native/system-service entries, dimension validators, reporting indexes, and replay examples.
  - Validation: Tests prove grant, native-service, and system-service entries are queryable by grant, native service, system service, account, tenant, resource dimension, and time window.

- **5.4 Implement external payment credit/debit refs.**
  - Design: Record tokenized Overbill-owned external payment credit/debit refs as ledger entries that link accounting history to external payment documents without making external calls.
  - Output: External payment ref entry schema, Overbill source-ref validator, no-call boundary check, audit events, and replay fields.
  - Validation: Tests prove external payment refs do not trigger payment-provider calls, do not store raw payment credentials, and do not create invoices, receipts, refunds, or chargeback state inside Seal Ledger.

- **5.5 Implement bounded indexes and query API.**
  - Design: Build indexes by account, tenant, workload, provider, dispute, grant, native service, system service, source rollup, resource dimension, time window, entry type, and correction state.
  - Output: `GET /ledger/query`, pagination, bounded filters, redaction classes, index lag behavior, and rebuild hooks.
  - Validation: Query tests prove bounded filters return ordered entries, role-filtering applies, unbounded scans are rejected, and indexes can be rebuilt deterministically from stream truth.

## Phase 6: Checkpoints, Replay, Integrity, And Audit Export

### Work Items

- **6.1 Implement checkpoint creation.**
  - Design: Create BLAKE3/checksum checkpoint records over stream sequence ranges using the Seal Ledger stream writer key managed through Overkey.
  - Output: `POST /ledger/checkpoints`, checkpoint record model, sequence-range validation, signer refs, verification status, and `seal_ledger.checkpoint_created` events.
  - Validation: Tests prove checkpoints cover contiguous ranges, reject missing or reordered entries, cite signer refs, and do not claim blockchain consensus or mining semantics.

- **6.2 Implement checkpoint verification and countersignature readiness.**
  - Design: Verify checkpoint ranges locally and support independent verifier signatures from Overwatch, Backup and Restore, or system-verifier service accounts before backup/restore finality, audit-export finality, migration cutover, or grid-resident failover promotion.
  - Output: Verification endpoint/worker, verifier ref model, countersignature records, mismatch reason codes, and incident hooks.
  - Validation: Tests prove tampered entries, missing entries, wrong sequence ranges, wrong signer refs, and stale schema versions produce verification failures and quarantine evidence.

- **6.3 Implement ledger replay.**
  - Design: Return ordered entries, checkpoints, source refs, correction refs, redaction metadata, and replay proof material for bounded stream/account/workload/provider/dispute/grant/time queries.
  - Output: `GET /ledger/replay`, replay bundle schema, checksum fields, redaction profile, and consumer authorization checks.
  - Validation: Replay tests prove deterministic ordering, stable hashes, correction history visibility, role-filtered redaction, and no raw private workload payload leakage.

- **6.4 Implement audit export.**
  - Design: Create authorized bounded export bundles with accounting truth retained indefinitely and private workload evidence exported only through redacted refs unless consumer authorization allows deeper Overwatch/Overvault-backed access.
  - Output: `POST /ledger/audit-exports`, export metadata, checksum, signer refs, redaction policy, retention class, consumer authorization refs, and `seal_ledger.audit_export_created` events.
  - Validation: Export tests prove ordering, checksums, source refs, redaction, retention metadata, and authorization checks are preserved while raw prompts, outputs, logs, secrets, exact node addresses, provider-private topology, and private tenant data are excluded by default.

- **6.5 Implement checkpoint and audit observability.**
  - Design: Expose checkpoint lag, verification lag, export count/size, failed verification count, tamper/mismatch incidents, redaction failures, and consumer export activity.
  - Output: Metrics schema, dashboard fields, alert rules, Overwatch event aggregation, and runbook refs.
  - Validation: Operations tests prove alerts fire for checkpoint lag, verification mismatch, stale verifier signatures, export authorization failure, redaction failure, and replay checksum mismatch.

## Phase 7: Reconciliation, Corrections, And Downstream Derivations

### Work Items

- **7.1 Implement reconciliation jobs.**
  - Design: Compare bounded ledger state against Overmeter rollups, ORU Account Service projections, Overbill refs, Overclaim refs, Overgrant refs, and Provider Payout refs.
  - Output: `POST /ledger/reconcile`, reconciliation report records, expected/observed totals, mismatch reason codes, correction refs, and `seal_ledger.reconciliation_created` or `seal_ledger.reconciliation_mismatch` events.
  - Validation: Tests prove mismatches are durable, queryable, replayable, and cannot be silently bypassed by ORU projections, billing documents, payout batches, grant reports, or dispute finality.

- **7.2 Implement correction-by-new-entry flow.**
  - Design: Append correction entries referencing prior entries, finality refs, operator/service-account refs, reason codes, and replacement accounting effects without editing or deleting the original entry.
  - Output: `POST /ledger/corrections`, correction entry schema, supersession refs, operator-signed evidence, affected index updates, and `seal_ledger.correction_appended` events.
  - Validation: Tests prove corrections require prior entry refs, finality/evidence refs, paired entries when value moves, replay visibility of old and new entries, and no historical mutation.

- **7.3 Implement dispute, refund, reversal, and hold handoffs.**
  - Design: Link Overclaim finality, refund refs, payout reversal refs, chargeback refs, hold release refs, and operator actions through source evidence rather than letting Seal Ledger adjudicate disputes.
  - Output: Handoff schemas, allowed owner-service refs, reason codes, correction flows, and reconciliation hooks.
  - Validation: Tests prove Overclaim owns dispute finality, Overbill owns billing/payment documents, Provider Payout owns payout coordination, and Seal Ledger only records append-only accounting effects from accepted refs.

- **7.4 Implement downstream derivation contracts.**
  - Design: Provide stable read/query/replay contracts for ORU Account Service, Overbill, Provider Payout Service, Overgrant, Overclaim, Wallet and Usage Center, central AI stewardship, and reporting tools.
  - Output: Consumer contract matrix, redacted view schemas, freshness fields, checkpoint refs, replay refs, and derived-state caveats.
  - Validation: Tests prove downstream services store ledger entry refs rather than copying mutable accounting state and cannot treat derived views as independent accounting truth.

- **7.5 Implement reconciliation repair governance.**
  - Design: Route high-impact mismatches, correction waves, checkpoint mismatches, migration errors, and operator-signed corrections through Overwatch, incident response, compliance boundary, and stewardship reporting refs.
  - Output: Governance checklist, escalation refs, review states, retention rules, and signed action requirements.
  - Validation: Governance tests prove repair actions are scoped, evidence-backed, signed, replayable, and exportable for audit without raw private evidence leakage.

## Phase 8: Backup/Restore, Migration Import, And Grid-Resident Readiness

### Work Items

- **8.1 Implement backup/restore verification hooks.**
  - Design: Expose checkpoint ranges, verifier refs, replay proofs, stream epochs, segment ids, and stream close/tombstone metadata to Backup and Restore Service.
  - Output: Backup/restore hook schema, verification API, restore drill fixtures, checkpoint promotion fields, and incident refs.
  - Validation: Restore tests prove restored streams verify against checkpoint ranges, preserve sequence order, reject tampered imports, and remain replay-compatible.

- **8.2 Implement `migration_import` batches.**
  - Design: Move founder-hardware/bootstrap streams into grid-resident stores through explicit import batches with source stream id, source sequence/checkpoint range, export snapshot ref, cutoff time, source/destination stream epochs, checksum refs, signer/verifier refs, import reason, and old-to-new account/stream mapping.
  - Output: Migration import schema, paired opening-balance behavior, migration_control/opening_balance accounts, cutover checklist, and audit events.
  - Validation: Migration tests prove imports append new records, original entries remain read-only and replayable, opening balances stay balanced, and no historical bootstrap entry is edited.

- **8.3 Prepare grid-resident writer and failover promotion.**
  - Design: Add stream epoch, writer key rotation, segment promotion, leader/failover readiness, maintenance mode, and split-brain prevention fields without changing Phase 5 entry semantics.
  - Output: Grid-resident operation notes, writer promotion contract, failover fixtures, service identity refs, maintenance controls, and replay pause/resume behavior.
  - Validation: Failover tests prove only one writer epoch can append, old writers are rejected after promotion, sequence continuity is preserved, and no duplicate external payment refs or accounting entries are created.

- **8.4 Prepare native persistence handoffs.**
  - Design: Move ledger stream storage toward native Overbase, replay/export artifacts toward Overstore, and private evidence refs toward Overvault while keeping public service contracts stable.
  - Output: Persistence adapter interfaces, migration plan, artifact refs, private-ref handoffs, retention hooks, and export compatibility notes.
  - Validation: Migration tests prove no conventional database/object/vault boundary becomes canonical, APIs remain stable, and private refs stay behind owning-service access controls.

- **8.5 Implement retention and redaction policy hooks.**
  - Design: Preserve accounting truth indefinitely while applying Overmeter-aligned private workload evidence retention of at least 90 days after signed rollup and receipt creation, extended for disputes, corrections, fraud/security review, payout holds, grants, compliance export, or incident finality.
  - Output: Retention class fields, redaction profile refs, finality margin fields, export behavior, and cleanup eligibility reports.
  - Validation: Tests prove accounting entries remain durable, private evidence exports use redacted refs after retention windows, and active disputes/incidents/compliance exports prevent premature private-evidence reduction.

## Phase 9: Operations, Redaction, Native Persistence, And Client Handoffs

### Work Items

- **9.1 Build ledger dashboards and alerts.**
  - Design: Track append success/denial counts by producer and entry type, stream lag, checkpoint lag, index lag, idempotency duplicates, sequence conflicts, reconciliation mismatches, unresolved mismatch age, correction counts, query latency, audit export count/size, and backup/restore verification status.
  - Output: Metrics schema, dashboard definitions, alert rules, Overwatch aggregation, and runbook links.
  - Validation: Operations tests prove alerts fire for stale indexes, sequence conflicts, append denial spikes, duplicate idempotency conflicts, checkpoint mismatch, reconciliation gaps, redaction failure, export failure, and restore verification failure.

- **9.2 Implement role-aware entry and replay reads.**
  - Design: Provide redacted internal/operator/client views that expose enough ledger-derived evidence for debugging and client status without leaking private workload details or raw secret-bearing refs.
  - Output: Redaction classes, response profiles, authorization checks, consumer-specific field maps, and replay drill-down links.
  - Validation: Tests prove Wallet and Usage Center, admin UI, SDK, CLI, stewardship, and reporting consumers see only authorized data and cannot infer private prompts, outputs, logs, exact node addresses, provider-private topology, or other-tenant facts.

- **9.3 Feed ORU, billing, payout, grant, dispute, wallet, and stewardship consumers.**
  - Design: Provide explicit consumer examples for ORU Account balance projections, Overbill receipts/statements/payment refs, Provider Payout earning/hold views, Overgrant reports, Overclaim correction/finality flows, Wallet and Usage Center views, and central AI stewardship summaries.
  - Output: Consumer examples, contract notes, owner-service refs, freshness fields, and replay refs.
  - Validation: Tests prove consumers use ledger entry refs, not mutable copied accounting state, and cannot write Seal Ledger entries unless they are authorized producers for specific entry types.

- **9.4 Add public-provider and federation source-ref readiness.**
  - Design: Prepare source-ref slots for Phase 10 public-interest/federation grant contexts and Phase 11 public-provider hold/throttle/challenge/fraud contexts without changing private Phase 5 ledger semantics.
  - Output: Source-ref compatibility notes, feature-gated fixtures, public/federation reason codes, and redaction rules.
  - Validation: Tests prove public/federation refs are rejected before prerequisites exist, accepted only from owner services after gate enablement, and never cause Seal Ledger to own public risk decisions.

- **9.5 Add incident, threat-model, retention, and scale handoffs.**
  - Design: Integrate incident response refs, threat-model findings, compliance boundary policy refs, stewardship reports, migration controls, retention/export policy, and scale-hardening reviews.
  - Output: Governance checklist, threat-model test list, incident handoff refs, stewardship report fields, scale test plan, and retention policy.
  - Validation: Governance tests prove checkpoint mismatches, correction waves, migration cutovers, audit export changes, redaction repairs, and compliance policy changes require signed action, evidence refs, Overwatch audit, and retention-compliant exports.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #44`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, work-item sections, final newline, and tab-free formatting.

- **10.2 Validate tech-stack alignment.**
  - Design: Scan for accidental conventional cloud, SaaS-admin, blockchain/NFT, speculative-market, external-payment-as-boundary, pricing, revenue, customer-count, raw-secret storage, direct payment execution, mutable ledger truth, mutable wallet truth, direct dispute adjudication, trust scoring, and raw private evidence exposure assumptions.
  - Output: Stack-alignment scan result and any corrected wording.
  - Validation: Scan shows only approved negative-control references, native Overrid service names, or explicit non-choice guardrails from `docs/overrid_tech_stack_choice.md`.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk links.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated docs and link-check evidence.
  - Validation: Local Markdown link validation returns no missing local targets for changed docs.

- **10.4 Validate Seal Ledger authority and phase gates.**
  - Design: Verify every planned behavior preserves Phase 5 as the first implementation point for append-only accounting and keeps backup/restore promotion, native persistence, federation/public contexts, client surfaces, and governance hardening behind their owning later phase gates.
  - Output: Authority-boundary checklist and implementation handoff notes.
  - Validation: Review confirms Seal Ledger does not own mutable balance truth, pricing/reference bands, raw usage measurement, invoices, receipts, payment-provider calls, payout execution, dispute adjudication, trust scoring, scheduling, backup execution, raw payment/tax/identity secrets, blockchain consensus, NFTs, mining, speculative tokens, or per-operation external payment calls.

- **10.5 Reindex and verify retrieval handoff.**
  - Design: Refresh Docdex for the new plan and linked docs, then verify search returns the SDS #44 plan and source docs in the aligned result set.
  - Output: Docdex index refresh evidence, retrieval evidence, queue/progress notes, repo memory, and implementation handoff note.
  - Validation: Docdex index succeeds, Docdex search for `SUB BUILD PLAN #44 Seal Ledger append-only accounting checkpoints reconciliation correction migration import` returns the new sub-build plan in the result set, and `docdexd run-tests` blocker is recorded if no test runner is configured.

## Alignment Review

- SDS #44 already contains resolved open-question decisions for tenant/system streams, stream sequence order, expected-sequence checks, BLAKE3 checkpoint ranges, paired debit/credit-style entries, redacted audit bundle retention, and migration_import cutovers. This pass adds the sub-build-plan backlink and does not require SDS content correction.
- The service catalog already matches the SDS and master plan: Seal Ledger starts in Phase 5 for append-only internal accounting, entries, batches, indexes, checkpoints, replay, reconciliation, audit export, and correction-by-new-entry. This pass adds the sub-build-plan backlink.
- The master Phase 0 through Phase 13 order remains valid. This pass adds SDS #44 to the per-SDS index and keeps Seal Ledger in Phase 5 while preserving Phase 7 as backup/restore/grid-resident promotion, Phase 8 as native persistence, Phase 10/11 as later federation/public source contexts, Phase 12 as client/native consumption, and Phase 13 as governance hardening.
- The build-plan crosswalk remains valid. This pass adds SDS #44 to the sub-build-plan index with Phase 5 first-build alignment and later restore/persistence/public/native/governance gates.
- The accepted Rust-first/native-Overrid stack remains authoritative. The plan uses Rust, Tokio, shared contracts, canonical JSON plus JSON Schema, signed envelopes, Ed25519, BLAKE3/content hashes, and native Overrid persistence/storage/private-ref boundaries.
- Local delegation was attempted for an advisory outline and rejected as authoritative because it produced 11 phases, included a Phase 0, and missed the established work-item structure.

## Exit Gate

SUB BUILD PLAN #44 is complete when the new plan is linked from the SDS, service catalog, master build plan, and service-catalog alignment docs; Phase 5 remains the first build point for append-only internal accounting streams, ledger entries, entry batches, authorized append APIs, idempotency, expected-sequence checks, paired entries, bounded indexes, checkpoints, replay, reconciliation, audit exports, and correction-by-new-entry; Phase 7 remains the backup/restore and grid-resident promotion gate; Phase 8 remains the native persistence gate; later federation/public/client/governance contexts remain behind their owning phases; the plan has 10 phases with well-designed Design/Output/Validation work items; queue/progress docs record the pass; local link and structure validation pass; stack guardrail scans show no accidental conventional-cloud, NFT, speculative-market, external-payment-as-boundary, pricing, revenue, customer-count, raw-secret-storage, mutable-ledger-truth, direct-payment-execution, dispute-adjudication, trust-scoring, raw-private-evidence, or per-operation external-payment drift; and Docdex retrieval can find the new plan with SDS #44 context.
