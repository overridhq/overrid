# SUB BUILD PLAN #23 - Overmeter

Attached SDS: [docs/sds/execution_scheduling/overmeter.md](../sds/execution_scheduling/overmeter.md)

## Purpose

This sub-build plan turns SDS #23 into an implementation sequence for Overmeter. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overmeter is the usage-evidence bridge between execution and accounting. It starts in master Phase 3 by recording raw usage events for private workloads. It expands in master Phase 5 by producing deterministic signed rollups that ORU Account Service, Seal Ledger, Overbill, disputes, provider payout flows, wallet usage views, and stewardship reporting can consume. It is not a pricing engine, not a settlement authority, not a ledger mutator, not a billing policy service, and not a place to hide explicit resource dimensions behind a vague total.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #23: Overmeter](../sds/execution_scheduling/overmeter.md) | Controls Overmeter purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering/accounting boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overmeter service plan](../service_catalog/execution_scheduling/overmeter.md) | Controls the service-catalog objective, first build phases, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, canonical JSON/JSON Schema discipline, signed envelopes, idempotency, trace ids, local fixtures, and integration harness prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, identity, tenant, key, registry, queue, and audit primitives that Overmeter consumes. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies registered Overcell nodes, capability records, benchmark evidence, heartbeat/load facts, and node lifecycle state used to corroborate usage events. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Controls Overmeter's first build point: raw usage events for private workloads, tied to queue, lease, workload, run attempt, node, provider, tenant, source evidence, and final state. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies policy decisions, verification facts, Overclaim dispute holds, challenge evidence, and trust refs that determine settlement eligibility later without changing raw event history. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Controls signed rollups, retention/dispute windows, accounting export, ORU/Seal Ledger/Overbill handoff, holds, corrections, receipts, and provider earning inputs. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Adds Docdex, Mcoda, Codali, RAG, model-routing, SDK, CLI, and admin/developer UI usage consumers and producers. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Requires Overmeter to support protected grid-resident system-service workload usage, health, backup, restore, failover, maintenance, and service-runtime accounting evidence. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Adds Overbase, Overstore, Overvault, Universal Namespace, Overasset, storage refs, private data refs, object refs, and namespace-bound usage dimensions. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Consumes purpose-scoped and grant-funded usage evidence through Overgrant, public-interest pools, and stewardship reporting boundaries. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Adds public-provider usage constraints, fraud/anti-abuse holds, challenge evidence, payout hold inputs, and low-sensitivity public workload reporting. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Consumes wallet-visible usage summaries, native app/service usage units, user-facing receipts, holds, corrections, and redacted rollup refs. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies retention, migration, incident, compliance, reporting, threat-model, and governance hardening for usage evidence, rollup replay, and accounting exports. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #23 first raw-event build work aligned to master Phase 3, with signed rollups in master Phase 5 and later product, grid-resident, public-provider, native-app, and governance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, authenticated HTTP/2 with mTLS/rustls for early seed paths, canonical JSON plus JSON Schema, optional Protobuf for compact contracts, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and native Overrid service boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 2, 3, and 5 | Attach SDS #23, freeze Overmeter scope, preserve raw events in Phase 3, and preserve signed rollups/accounting handoff in Phase 5. |
| 2 | Master Phases 0, 1, 3, and 5 | Build Rust service contracts, schemas, APIs, state machines, reason codes, fixtures, and local harness scenarios. |
| 3 | Master Phase 3 | Implement trusted raw usage ingestion for the first private execution loop. |
| 4 | Master Phases 2, 3, and 4 | Add attribution, source refs, lease/run correlation, confidence metadata, and corroboration before settlement eligibility. |
| 5 | Master Phases 3 and 5 | Normalize resource dimensions with versioned replay records while preserving raw events as non-billing evidence. |
| 6 | Master Phase 5 | Add deterministic rollup windows, signed usage rollups, replay metadata, and settlement-eligibility state. |
| 7 | Master Phase 5 | Add accounting exports for Seal Ledger, ORU, Overbill, provider payout, receipts, and stewardship consumers without mutating their ledgers. |
| 8 | Master Phases 4, 5, 11, and 13 | Add dispute holds, append-only corrections, retention windows, fraud/public-provider holds, and governance/audit replay. |
| 9 | Master Phases 5, 6, 8, 10, 11, 12, and 13 | Expand usage sources for RAG/model routing, cache, mesh, storage, native apps, public-provider work, wallet views, and stewardship reports. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, and implementation gates. |

## Tech Stack Guardrails

- Overmeter core is a Rust service module using shared contract types, Tokio where async ingestion/export workers are required, and Axum/Tower/Hyper-style HTTP for usage APIs where an HTTP boundary exists.
- Phase 3 ingestion uses authenticated service-to-service calls over early Overrid transport conventions. Every mutating call carries a signed envelope, service identity, tenant scope, trace id, idempotency key, schema version, stable reason codes, source evidence refs, policy refs where applicable, and append-only Overwatch refs.
- Usage events, resource dimensions, normalization records, rollup windows, signed rollups, dispute markers, correction records, exports, API errors, fixtures, and events use canonical JSON plus JSON Schema. Compact Protobuf contracts may be added only where the shared contract layer requires them.
- Ed25519 is used for rollup, correction, export, service, and command signatures where signatures are required. BLAKE3/content hashes are used for raw-event batches, normalized-dimension sets, rollup source sets, export checkpoints, fixture hashes, and replay evidence.
- Overmeter owns usage evidence, normalization, deterministic aggregation, signatures, retention metadata, hold/correction refs, and export records. ORU Account Service, Seal Ledger, Overbill, Overclaim, Overmark, Provider Payout Service, Overgrant, and Overasset own their respective accounting, dispute, pricing/reference-rate, payout, grant, and rights states.
- Usage records must not include raw secrets, private documents, prompts, outputs, logs, provider-private topology, exact node addresses, or ledger internals where refs and redacted summaries are enough.
- PostgreSQL, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFT, pricing, revenue, customer-count, or per-operation external payment assumptions must not become Overmeter's product boundary.

## Phase 1: SDS Attachment, Scope, And Phase-Split Rules

### Work Items

- **1.1 Attach the build plan to SDS #23.**
  - Design: Link this document from the numbered Overmeter SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/execution_scheduling/overmeter.md`, `docs/service_catalog/execution_scheduling/overmeter.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #23 returns both the Overmeter SDS and this sub-build plan.

- **1.2 Freeze Overmeter as usage evidence and signed rollup infrastructure.**
  - Design: Record that Overmeter owns raw usage events, resource dimension definitions, attribution refs, normalization records, rollup windows, signed usage rollups, dispute markers, correction records, and exports.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overmeter does not become pricing, settlement, ledger mutation, billing policy, provider payout execution, grant allocation, fraud adjudication, or wallet UI logic.

- **1.3 Preserve master Phase 3 as the raw-event first build point.**
  - Design: Keep first implementation in master Phase 3 because raw private-workload usage must be captured before ORU, Seal Ledger, Overbill, or provider earning logic can consume it.
  - Output: Phase-gate note that Phase 0 through Phase 2 are prerequisites, Phase 3 records raw usage, Phase 4 adds trust/dispute inputs, and Phase 5 creates signed rollups.
  - Validation: Review proves this plan does not move signed settlement, provider payout, pricing, or ledger mutation into Phase 3.

- **1.4 Preserve master Phase 5 as the signed-rollup and accounting-handoff gate.**
  - Design: Keep deterministic signed rollups, retention/dispute windows, accounting exports, holds, corrections, receipts, and provider earning inputs behind Phase 5.
  - Output: Phase 5 gate checklist for rollup signatures, replay metadata, dispute timestamps, export refs, and downstream consumer contracts.
  - Validation: Review confirms raw events remain non-billing evidence until Phase 5 rollups are signed and exported through owning accounting services.

- **1.5 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #23 decisions for required private-workload raw dimensions, classed rollup windows, corroboration before settlement eligibility, retention at least through dispute/audit windows, and redacted wallet-visible summaries.
  - Output: Resolved-decision checklist tied to SDS #23 open-question answers.
  - Validation: Review rejects vague total-only metrics, uncorroborated node metrics as settlement-ready, early raw retention drops, hidden GPU/model/RAG/cache dimensions, and wallet views that expose provider-private or workload-private evidence.

## Phase 2: Rust Service, Schemas, APIs, Fixtures, And State Machines

### Work Items

- **2.1 Create the Overmeter Rust service module.**
  - Design: Add a Rust service module with ingestion handlers, event repository, dimension catalog, normalizer, rollup worker, signer boundary, export worker, dispute/correction repository, Overwatch emitter, Overkey verifier, and integration-test hooks.
  - Output: Service crate or module skeleton, repository traits, client interfaces, worker entry points, error types, reason-code mapping, and test harness entry points.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Overmeter remains separate from Overrun, Overlease, Overwatch, Overkey, Overclaim, ORU, Seal Ledger, Overbill, and payout services.

- **2.2 Define Overmeter contract schemas.**
  - Design: Add schemas for `usage_event`, `resource_dimension`, `usage_source_ref`, `usage_normalization_record`, `rollup_window`, `signed_usage_rollup`, `usage_dispute_marker`, `usage_correction`, export requests, API errors, lifecycle events, redaction classes, and reason codes.
  - Output: JSON Schema files, Rust types, fixtures, lifecycle enums, reason-code enums, schema-version rules, redaction metadata, and compatibility rules.
  - Validation: Schema tests reject missing tenant, source service, source event id, trace id, idempotency key, timestamp, resource dimensions, source evidence refs, confidence metadata, and state where required.

- **2.3 Define Phase 3 ingestion and query APIs.**
  - Design: Specify `POST /usage/events`, `GET /usage/events/{event_id}`, `GET /usage/events`, and `POST /usage/normalize` with signed service identity, idempotency, trace propagation, source evidence refs, pagination, filters, and visibility rules.
  - Output: API request/response schemas, signed-envelope rules, dedup behavior, read filters, normalization request limits, and Overwatch event payloads.
  - Validation: API tests cover valid ingestion, duplicate idempotency keys, wrong tenant, wrong service account, missing source refs, malformed dimensions, unauthorized reads, and bounded normalization requests.

- **2.4 Define Phase 5 rollup, hold, correction, and export APIs.**
  - Design: Specify `POST /usage/rollups`, `GET /usage/rollups/{rollup_id}`, `POST /usage/rollups/{rollup_id}/hold`, `POST /usage/corrections`, and `POST /usage/exports/seal-ledger` with deterministic source-set selection and downstream export refs.
  - Output: Rollup, hold, correction, and export API schemas with replay metadata, signature refs, dispute-window timestamps, export idempotency, and failure reason codes.
  - Validation: API tests reject unsigned rollups, missing raw-event refs, missing normalizer versions, export before signature, direct ledger mutation, correction without evidence, and hold release without authority.

- **2.5 Build deterministic local harness fixtures.**
  - Design: Model successful private workload usage, failed retryable usage, cancelled usage, timed-out usage, duplicate event retry, missing lease ref, suspect node metric, normalization failure, signed rollup success, disputed rollup hold, correction superseding a rollup, and export retry.
  - Output: Valid and invalid fixtures with expected state transitions, normalized dimensions, rollup totals, signatures, Overwatch events, export refs, redacted reads, and reason codes.
  - Validation: Local harness scenarios produce deterministic outputs and prove Overmeter behavior does not depend on conventional database, queue, object-store, SaaS billing, blockchain, NFT, pricing, or external payment product boundaries.

## Phase 3: Phase 3 Raw Usage Ingestion For Private Workloads

### Work Items

- **3.1 Ingest raw usage events from trusted services.**
  - Design: Accept events only from trusted Overrun, Overcell, Overcache, Overmesh, model routing, RAG, storage, native-service, or system-service accounts with allowed source types for the current phase.
  - Output: Ingestion handler, source-service allowlist by phase, envelope verifier, source-event id mapping, and `overmeter.usage_ingested` or `overmeter.usage_rejected` events.
  - Validation: Ingestion tests reject unsigned callers, unknown source services, missing source event ids, stale envelopes, wrong tenant scope, and unauthorized service accounts.

- **3.2 Capture the first private workload usage envelope.**
  - Design: Require the Phase 3 private-workload event to include tenant id, provider id, node id, workload id, queue item id, lease id, run attempt id, final state, trace/audit refs, start/end timestamps, wall time, queue wait/start delay, CPU time, memory peak, storage read/write bytes, network bytes, source refs, schema version, and confidence metadata.
  - Output: First workload usage fixture, required-field validator, missing-dimension reason codes, and private execution integration test hook.
  - Validation: Contract tests prove a private workload cannot be considered usage-valid without the required refs and dimensions; GPU/model dimensions are required when requested or used.

- **3.3 Implement idempotency and deduplication.**
  - Design: Deduplicate by source service, source event id, workload id, run attempt id, and idempotency key while preserving retry evidence and returning the original accepted event id.
  - Output: Idempotency store contract, duplicate response behavior, conflict reason codes, retry audit event, and duplicate metrics.
  - Validation: Retry tests prove duplicate source events do not double-count usage, conflicting payloads are rejected, and accepted event ids remain stable.

- **3.4 Store immutable raw events with query-safe indexes.**
  - Design: Persist raw usage events with tenant, workload, lease, provider, node, source, time range, state, confidence, and trace indexes without exposing private payload-adjacent data.
  - Output: Raw event repository, query filters, pagination model, visibility checks, retained-summary marker, and read models for tenant/provider/operator scopes.
  - Validation: Query tests prove tenants see only authorized workload usage, providers see only relevant resource usage, operators see restricted diagnostics, and raw secrets/logs/prompts/outputs are absent.

- **3.5 Emit raw-event observability without billing decisions.**
  - Design: Emit ingestion rate, rejected event rate, duplicate count, missing source refs, source-service count, raw dimension coverage, and raw event lag as operational metrics and Overwatch refs.
  - Output: Observability counters, structured traces, `overmeter.usage_ingested` and `overmeter.usage_rejected` event payloads, and dashboard seed queries.
  - Validation: Operations tests prove raw observability never calculates price, account balance, provider payout, settlement state, or billable total inside Overmeter.

## Phase 4: Attribution, Source Evidence, And Corroboration

### Work Items

- **4.1 Link usage to Overlease windows.**
  - Design: Correlate raw usage with lease id, lease window, reservation scope, expiration, renewal, release, and stale cleanup state before the event can feed settlement-eligible rollups.
  - Output: Lease attribution resolver, missing/late lease reason codes, non-settlement diagnostic usage state, and lease-window replay refs.
  - Validation: Tests prove accounting-relevant usage without a valid lease window is rejected or marked non-settlement diagnostic, and late lease corrections create explicit records.

- **4.2 Link usage to workload, queue, node, and provider facts.**
  - Design: Resolve Overqueue item id, Oversched placement decision, Overrun attempt state, Overcell node id, provider id, capability refs, and final workload state.
  - Output: Attribution resolver, source fact cache, stale fact handling, missing fact reason codes, and attribution audit refs.
  - Validation: Tests prove usage cannot silently shift tenant, provider, node, workload, queue item, or attempt ownership after acceptance.

- **4.3 Capture source evidence refs instead of payloads.**
  - Design: Store refs to Overrun result records, Overcell heartbeat/load facts, Overwatch timing evidence, runtime/container counters, Overcache entry refs, Overmesh transfer refs, model request refs, RAG retrieval refs, storage/object refs, and policy refs.
  - Output: `usage_source_ref` validators, evidence-ref type registry, redaction classes, and source-ref replay fixtures.
  - Validation: Security tests prove raw logs, prompts, outputs, private documents, secrets, node addresses, and provider-private topology do not enter usage events when refs are enough.

- **4.4 Corroborate node-reported settlement dimensions.**
  - Design: Cross-check CPU time, memory peak, GPU time/memory, storage bytes, network bytes, wall time, queue wait/start delay, model counts, and cleanup overhead against lease windows, Overrun attempt state, Overcell facts, Benchmark Runner evidence where applicable, Overwatch timing, and runtime counters.
  - Output: Corroboration worker, `needs_corroboration` and `suspect_source` reason codes, confidence updates, and settlement-eligibility flags.
  - Validation: Tests prove uncorroborated or contradictory metrics stay raw diagnostic usage and cannot become signed settlement-ready rollups until corrected, held, or backed by accepted evidence.

- **4.5 Preserve confidence and source-quality metadata.**
  - Design: Attach source confidence, measurement method, source service version, normalizer version readiness, corroboration status, and dispute eligibility to raw and normalized records.
  - Output: Confidence metadata schema, source-quality enum, read filters, event payload fields, and operator diagnostics.
  - Validation: Review confirms downstream consumers can distinguish accepted, diagnostic, suspect, held, corrected, and settlement-eligible usage without editing raw history.

## Phase 5: Normalization And Resource-Dimension Discipline

### Work Items

- **5.1 Create the resource dimension catalog.**
  - Design: Define CPU, GPU, memory, storage, network, data, wall time, queue wait, cleanup overhead, model inference, RAG retrieval, cache, mesh transfer, native app, and service-unit dimensions with explicit units, precision, aggregation rules, source types, and accounting mapping hints.
  - Output: `resource_dimension` catalog, unit enum, precision rules, aggregation rules, schema fixtures, and compatibility notes.
  - Validation: Dimension tests prove GPU, storage, network, model, RAG, cache, and service dimensions cannot collapse into a vague total.

- **5.2 Implement versioned normalization records.**
  - Design: Convert raw source metrics into explicit Overrid resource dimensions using normalizer versions, source refs, adjustment rules, dropped-value records, and replay metadata.
  - Output: Normalizer worker, `usage_normalization_record`, normalizer version registry, batch limits, replay refs, and `overmeter.usage_normalized` events.
  - Validation: Normalization tests prove the same raw refs and normalizer version produce the same normalized dimensions.

- **5.3 Record dropped, adjusted, and rejected values.**
  - Design: Preserve reason codes for unit mismatch, missing source ref, negative duration, impossible GPU use, stale lease, suspect node source, duplicate source event, and policy-ineligible usage.
  - Output: Adjustment reason-code registry, rejected-value records, diagnostic read model, and operator alert rules.
  - Validation: Tests prove normalization failures preserve raw events, fail bounded batches with explicit reasons, and do not silently discard accounting-relevant dimensions.

- **5.4 Keep accounting mapping hints non-pricing.**
  - Design: Map normalized dimensions to ORU dimension hints, accounting resource classes, wallet summary groups, and provider reporting groups without setting rates or mutating balances.
  - Output: Mapping-hint schema, ORU dimension hint table, compatibility tests, and policy note for Overmark/ORU ownership.
  - Validation: Review rejects pricing, reference-rate calculation, customer-count math, revenue assumptions, settlement mutation, and provider payout decisions inside Overmeter.

- **5.5 Add normalization backfill and drift checks.**
  - Design: Support bounded re-normalization by source/time range with dry-run previews, normalizer version comparison, raw-ref completeness checks, and export-impact summaries.
  - Output: Backfill API behavior, dry-run report, drift detector, replay checklist, and retention guard.
  - Validation: Backfill tests prove existing accepted events are not edited, new normalization records point to prior refs, and raw retention cannot drop data required for replay or disputes.

## Phase 6: Phase 5 Rollup Windows, Signatures, And Replay

### Work Items

- **6.1 Implement deterministic classed rollup windows.**
  - Design: Start with one rollup per lease/run attempt for workload receipts plus hourly tenant/provider/node aggregation for settlement and provider earning batches; long-running workloads emit hourly partial rollups and a terminal final rollup.
  - Output: Rollup-window selector, window id rules, partial/final window state, included raw-ref sets, and dispute-window timestamp rules.
  - Validation: Tests prove window selection is deterministic and no window collapses explicit CPU, GPU, MEM, STOR, NET, DATA, model/RAG/cache, or Service-ORU dimensions into a vague total.

- **6.2 Aggregate normalized dimensions reproducibly.**
  - Design: Aggregate only accepted normalized records using raw event refs, normalizer version, source-set hash, policy refs, confidence thresholds, and settlement-eligibility gates.
  - Output: Aggregation worker, source-set hash, replay input manifest, totals schema, incomplete-window reasons, and rollup draft state.
  - Validation: Replay tests prove rollup totals are reproducible from raw refs and normalizer versions.

- **6.3 Sign rollups with replay metadata.**
  - Design: Use Overkey-backed signing refs and Ed25519 signatures over canonical rollup payloads, source-set hashes, normalizer versions, policy refs, totals, generated_at, and dispute-window timestamps.
  - Output: Signed rollup schema, signer client, signature refs, BLAKE3 source-set hashes, export checkpoints, and `overmeter.rollup_created` events.
  - Validation: Signature tests reject tampered totals, missing source refs, missing normalizer version, stale signer credentials, unsigned rollups, and mismatched replay hashes.

- **6.4 Manage rollup lifecycle state.**
  - Design: Implement draft, computed, signed, exported, held, settlement_ready, and superseded states with legal transitions, idempotent retries, and append-only transition refs.
  - Output: Rollup state machine, illegal-transition reason codes, terminal-state rules, state-change events, and read models.
  - Validation: State tests reject export before signature, settlement_ready while held, supersede without correction refs, duplicate export side effects, and silent deletion of prior versions.

- **6.5 Validate signature and replay behavior at phase gate.**
  - Design: Add a phase gate that recomputes signed rollups from raw refs, normalizer versions, source-set hashes, and signature refs before allowing accounting export.
  - Output: Replay verification command, fixture set, phase-gate report, failure reason codes, and export block behavior.
  - Validation: Gate tests prove corrupted raw refs, changed normalizer behavior, missing source refs, and signature mismatch block export.

## Phase 7: Accounting Exports And Downstream Boundaries

### Work Items

- **7.1 Export signed rollups to Seal Ledger.**
  - Design: Send signed rollups, source refs, correction refs, hold state, dispute-window timestamps, and replay metadata to the accounting layer without writing Seal Ledger entries directly.
  - Output: `POST /usage/exports/seal-ledger` behavior, export idempotency, export refs, retry policy, and `overmeter.rollup_exported` events.
  - Validation: Export tests prove unsigned, held, incomplete, suspect, or non-settlement diagnostic usage cannot be exported as settlement-ready ledger input.

- **7.2 Feed ORU and Overbill consumers through signed rollup contracts.**
  - Design: Provide ORU Account Service and Overbill with resource dimensions, account/tenant/provider refs, receipt grouping refs, hold/correction state, and replay metadata.
  - Output: ORU export payload, Overbill receipt/invoice input payload, statement grouping refs, and consumer contract tests.
  - Validation: Contract tests prove Overmeter exports inputs only; ORU balances, invoices, refunds, and account statements remain owned by their services.

- **7.3 Support provider payout and earning batch inputs.**
  - Design: Include provider id, node id, workload id, resource dimensions, rollup window, dispute/hold state, public-provider risk refs, and export refs needed by Provider Payout Service.
  - Output: Provider earning input schema, payout-hold flags, batch grouping hints, and provider-visible read model.
  - Validation: Tests prove provider payout execution, external payout status, and payout policy remain outside Overmeter.

- **7.4 Prepare wallet-visible usage summaries.**
  - Design: Produce redacted summaries by account, tenant or organization scope, app/service/native-app/workload class, resource dimension, time window, receipt/refund/correction/hold state, ORU projection refs, and source rollup refs.
  - Output: Wallet summary export schema, redaction profiles, user/provider/operator visibility matrix, and wallet fixture set.
  - Validation: Privacy tests prove wallet views hide provider-private topology, exact node addresses, raw host metrics, fraud signals, unauthorized tenant identifiers, raw logs, prompts, outputs, secret refs, and ledger internals.

- **7.5 Handle export failures and downstream lag.**
  - Design: Retry failed exports idempotently, preserve failed-export reason codes, alert on export lag, block early retention for unexported settlement-ready rollups, and expose operator diagnostics.
  - Output: Export retry worker, failure state, lag metrics, dry-run repair command, and operator read model.
  - Validation: Failure tests prove exports are not duplicated, unsigned rollups are never exported by retry, and downstream outage does not corrupt raw or signed usage evidence.

## Phase 8: Disputes, Corrections, Retention, And Audit Replay

### Work Items

- **8.1 Implement dispute holds with Overclaim refs.**
  - Design: Allow authorized Overclaim, operator, or accounting flows to place event or rollup refs under hold with reason, dispute id, scope, actor/service, and audit refs.
  - Output: Hold API behavior, `usage_dispute_marker`, `overmeter.rollup_held` events, held read models, and settlement block flags.
  - Validation: Dispute tests prove held usage remains visible, blocks settlement finality, and cannot disappear from tenant/provider/operator views.

- **8.2 Implement append-only corrections.**
  - Design: Preserve original raw events and signed rollups while appending correction records with corrected dimensions, actor/service, reason, signature, affected export refs, and downstream correction refs.
  - Output: `usage_correction` schema, correction API, superseded rollup link, `overmeter.correction_recorded` event, and correction read model.
  - Validation: Correction tests prove accepted events and exported rollups are never edited in place and correction replay can reconstruct before/after state.

- **8.3 Enforce retention and retained-summary rules.**
  - Design: Keep accounting-relevant raw events at full detail until the longest applicable dispute, receipt, audit, correction, provider payout hold, fraud/security review, grant reporting, or compliance export window closes; Phase 5 private-workload default is at least 90 days after signed rollup and receipt creation.
  - Output: Retention policy evaluator, pinned-record reasons, dry-run retention report, retained-summary schema, and `overmeter.retention_applied` events.
  - Validation: Retention tests prove active disputes, corrections, payout holds, compliance exports, replay requirements, and receipt windows block raw-detail compaction.

- **8.4 Route post-export disputes into accounting correction flow.**
  - Design: When a dispute arrives after export, append hold/correction state, preserve original export refs, emit accounting correction refs, and require downstream services to reconcile via new entries.
  - Output: Post-export hold flow, correction export payload, ledger/Overbill correction refs, and operator diagnostics.
  - Validation: Tests prove post-export disputes do not mutate prior exported rollups and downstream accounting receives explicit correction inputs.

- **8.5 Harden audit replay and governance evidence.**
  - Design: Preserve raw refs, normalizer versions, source-set hashes, signature refs, hold/correction refs, export refs, retention state, and policy refs for compliance and governance review.
  - Output: Replay bundle export, governance report inputs, incident evidence refs, and migration compatibility checks.
  - Validation: Governance tests prove a rollup can be traced back through raw events, source evidence, normalizer records, signatures, holds, corrections, exports, and retained summaries.

## Phase 9: Expanded Usage Sources, Product Views, And Reporting

### Work Items

- **9.1 Add Overcache and Overmesh usage facts.**
  - Design: Ingest cache hits, misses, writes, reads, storage bytes, egress, warming, eviction, retention, invalidation, saved upstream work, route resolution counts, transfer bytes, endpoint locality, retry counts, and mesh transfer facts.
  - Output: Source schemas, attribution refs, dimension mappings, fixtures, and raw usage event producers for Overcache and Overmesh.
  - Validation: Tests prove cache/mesh facts become raw usage and rollup dimensions without making Overcache or Overmesh billing authorities.

- **9.2 Add RAG and model-routing usage facts.**
  - Design: Ingest AI Gateway Router, encrypted Docdex RAG adapter, model request, model runtime, retrieval count, token/count proxy where applicable, and service-route usage refs without storing prompts, outputs, private docs, or secrets.
  - Output: RAG/model source schemas, redaction rules, normalizer mappings, fixtures, and product integration contract tests.
  - Validation: Privacy tests prove prompts, outputs, private documents, and secret refs are excluded while authorized usage summaries remain visible.

- **9.3 Add storage, namespace, and data-platform usage facts.**
  - Design: After Phase 8 primitives exist, ingest Overbase structured-state usage, Overstore object/artifact bytes, Overvault private-record refs, namespace route refs, asset refs, and storage/backup/repair dimensions.
  - Output: Data/storage source schemas, namespace/storage attribution rules, Overasset binding refs, and Phase 8 integration fixtures.
  - Validation: Tests prove Overmeter records usage refs and dimensions while Overbase, Overstore, Overvault, Universal Namespace, and Overasset retain ownership of their state.

- **9.4 Add native app and wallet usage surfaces.**
  - Design: Feed native app usage, service-unit summaries, wallet usage center displays, receipt refs, correction state, refund refs, held state, and user-safe rollup refs through normal Overrid APIs.
  - Output: Native app summary schema, wallet view fixtures, read authorization matrix, and UI/API contract notes.
  - Validation: Native app tests prove users see authorized usage and hold/correction/refund state without raw ledger internals, provider-private topology, or cross-tenant identifiers.

- **9.5 Add stewardship, public-provider, and grant reporting dimensions.**
  - Design: Feed public-interest pool usage, grant-funded work, sponsored work, provider-public workload summaries, fraud/hold status, challenge refs, and stewardship report inputs without revenue or customer-count assumptions.
  - Output: Reporting export schemas, public-provider hold flags, Overgrant refs, stewardship summary fields, and governance report fixtures.
  - Validation: Reporting tests prove public/grant/stewardship consumers receive signed usage evidence while payout, grant allocation, and governance finality remain in their owning services.

## Phase 10: Validation, Documentation, Queue State, And Handoff

### Work Items

- **10.1 Validate contract and state-machine coverage.**
  - Design: Run focused checks for ingestion, query, normalization, rollup creation, hold, correction, export, event lifecycles, state transitions, idempotency, redaction, and error reason codes.
  - Output: Contract-test report, state-machine test report, fixture coverage matrix, and failure notes.
  - Validation: Tests pass before implementation advances beyond the documented gate; any blocker is recorded in build-plan progress.

- **10.2 Validate end-to-end private-workload replay.**
  - Design: Prove a known private workload flows through queue, scheduler, lease, runner, raw usage, normalization, signed rollup, export, hold/correction where applicable, and wallet/accounting summary refs.
  - Output: End-to-end replay scenario, source-ref bundle, signed rollup fixture, export fixture, and replay report.
  - Validation: Replay confirms successful, failed, cancelled, timed-out, held, and corrected workloads produce distinct auditable usage states.

- **10.3 Validate security, privacy, and tech-stack alignment.**
  - Design: Scan implementation and docs for raw secret leakage, private payload leakage, unauthorized reads, conventional cloud-product boundary assumptions, blockchain/NFT mechanics, pricing/revenue/customer-count assumptions, and TypeScript core-runtime drift.
  - Output: Security/privacy checklist, tech-stack alignment report, negative-control scan results, and remediation notes.
  - Validation: Review confirms Overmeter remains Rust-first/native-Overrid infrastructure and does not adopt prohibited product boundaries or economic assumptions.

- **10.4 Validate documentation alignment.**
  - Design: Ensure SDS #23, the Overmeter service plan, master build plan, build-plan crosswalk, phase docs, queue state, and progress docs link to this sub-build plan and preserve Overmeter's Phase 3/Phase 5 split.
  - Output: Updated source-document links, sub-build-plan index entries, progress evidence, queue status, and alignment notes.
  - Validation: Markdown link checks pass and review confirms no master Phase 0 through Phase 13 ordering change was required.

- **10.5 Hand off implementation gates to downstream services.**
  - Design: Publish what Overrun, Overlease, Overcell, Overcache, Overmesh, AI/RAG services, Overwatch, Overkey, Overclaim, ORU, Seal Ledger, Overbill, Provider Payout Service, Overgrant, Overasset, wallet/native apps, and governance services may depend on.
  - Output: Handoff matrix listing producer/consumer contracts, ownership boundaries, required refs, visibility rules, failure behavior, and phase gates.
  - Validation: Handoff review confirms downstream services consume signed rollups and correction records rather than reinterpreting raw logs or moving Overmeter authority into accounting, payout, policy, wallet, or governance services.

## Alignment Review

- The sub-build plan keeps Overmeter first raw-event build work in master Phase 3, matching SDS #23, the service catalog entry, Phase 3 plan, master build plan, and build-plan crosswalk.
- The plan keeps signed rollups, dispute windows, corrections, retention, and accounting exports in master Phase 5, matching SDS #23 and the Phase 5 metering/accounting plan.
- The plan treats Phase 0 through Phase 2 as prerequisites for schemas, local fixtures, identity/tenant/key/audit primitives, node facts, and benchmark evidence rather than as Overmeter implementation phases.
- The plan treats Phase 4 as a trust/dispute/corroboration input gate rather than the first Overmeter build phase.
- The plan preserves the master Phase 0 through Phase 13 order and uses later phases only for product, grid-resident, storage/namespace, federation/public-provider, native-app, and governance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service, native Overrid boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.
