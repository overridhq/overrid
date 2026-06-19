# SUB BUILD PLAN #15 - Overwatch

Attached SDS: [docs/sds/control_plane/overwatch.md](../sds/control_plane/overwatch.md)

## Purpose

This sub-build plan turns SDS #15 into an implementation sequence for Overwatch. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overwatch is the append-only evidence backbone for Overrid. It stores events, audit records, traces, health signals, incidents, evidence bundles, retention/export metadata, and tamper-evidence checkpoints so services can prove what happened without turning private service memory into the source of truth.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #15: Overwatch](../sds/control_plane/overwatch.md) | Controls Overwatch purpose, owned evidence records, API surface, events, state machines, policy/security boundaries, validation, resolved open-question decisions, and downstream handoff. |
| [Overwatch service plan](../service_catalog/control_plane/overwatch.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared event/audit schemas, canonical JSON fixtures, API/event discipline, local stack, and integration harnesses required before Overwatch implementation. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Controls the first build point for Overwatch as the append-only event log, request trace, audit, health, and early policy-decision evidence primitive. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies queue, scheduler, lease, runner, result, retry, cancellation, timeout, and raw usage events that must become replayable evidence. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard, policy dry-run, Oververify, challenge, dispute, private mesh, and cache-trust evidence without moving those decisions into Overwatch. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage, ledger, ORU, billing, grant, payout, dispute, refund, and correction evidence refs while keeping settlement outside Overwatch. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies SDK, CLI, admin UI, Docdex, Mcoda, Codali, and adapter clients that inspect traces, audit refs, node health, usage evidence, and dispute evidence. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies system-service workload, replicated state, backup, restore, failover, rolling update, rollback, maintenance, and break-glass evidence needs. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, namespace, route, private data, secret-ref, retention, archive, and deletion-proof handoffs. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies federation, public-interest, purpose-tag, grant, and partner-swarm evidence requirements. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider, sandbox, fraud, reputation, challenge, payout-hold, and abuse evidence requirements. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies wallet, personal AI, workspace, directory, search, messaging, social, maps, mobile, and stewardship client trace and evidence requirements. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance exports, incident response, stewardship reporting, threat modeling, migration, PIP governance, and public-report evidence hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #15 first build work aligned to master Phase 1, with later hardening through trust, accounting, product, grid-resident, data/storage, federation/public, native-app, and governance phases. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first control-plane services, Axum/Tower/Hyper-style HTTP, signed command and service-account envelopes, Ed25519, BLAKE3 refs, canonical JSON plus JSON Schema, native Overwatch evidence, and no conventional cloud product-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 1 | Attach SDS #15 to the build-plan layer, freeze Overwatch as append-only evidence infrastructure, and preserve resolved checkpoint, retention, buffering, bundle, and trace-summary decisions. |
| 2 | Master Phases 0 and 1 | Build the Rust service skeleton, shared event/audit schemas, local Overrid-shaped storage, deterministic fixtures, and service-account append contracts. |
| 3 | Master Phase 1 | Implement append-only event and audit storage, idempotent append handling, sequence checks, payload hashes, prior-event hashes, and Phase 1 checkpoints. |
| 4 | Master Phases 1, 3, and 4 | Implement trace reconstruction, event/audit query APIs, tenant/role/data-class/evidence-purpose filtering, and caller-safe trace summaries. |
| 5 | Master Phases 4, 5, and 13 | Implement evidence bundles, retention classes, redaction markers, export manifests, public-report-safe projections, and legal/compliance handoff refs. |
| 6 | Master Phases 3, 4, and 5 | Add policy, dispute, verification, challenge, execution, usage, ledger, grant, payout, and correction evidence handoffs without owning final decisions. |
| 7 | Master Phases 4, 7, and 13 | Add health events, incident timelines, observability, dependency readiness, operator runbooks, and governance-grade incident evidence. |
| 8 | Master Phases 7, 8, and 13 | Add bounded append buffering, rebuild/replay recovery, backup/restore drills, archive refs, grid-resident operation, and corruption/checkpoint validation. |
| 9 | Master Phases 6, 10, 11, 12, and 13 | Harden SDK, CLI, admin, product, adapter, federation, public-provider, native-app, mobile, central AI, and reporting handoffs. |
| 10 | Master Phase 1 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, migration/governance handoff, and final implementation gates. |

## Tech Stack Guardrails

- Overwatch core is a Rust control-plane service using Tokio and Axum/Tower/Hyper-style HTTP, with rustls/mTLS where early seed control-plane transport requires it.
- Event, audit, trace, evidence bundle, incident, health, retention, export, API error, fixture, and compatibility records use canonical JSON plus JSON Schema from the shared schema package; compact Protobuf service/RPC/event contracts may be added only where the shared contract layer requires them.
- Service-account append requests and mutating admin commands require signed envelopes, idempotency keys, trace ids, tenant ids, actor or service refs, schema versions, stable reason codes, privacy classes, and append-only Overwatch meta-events.
- Ed25519 is used for signatures where service-account or operator signatures are required. BLAKE3 is used for payload hashes, prior-event hashes, content refs, export hashes, checkpoint hashes, and hash-linked audit evidence.
- Phase 1 integrity uses monotonic per-source sequences, BLAKE3 payload hashes, prior-event hashes, and signed Overwatch-owned checkpoints. Merkle range proofs, cross-node checkpoints, restore attestations, and replicated checkpoint authorities are later Phase 7+ hardening.
- Overwatch records are append-only. Corrections, redaction markers, state changes, retention changes, archive moves, and export changes append new records or derived indexes; they never rewrite prior evidence.
- Append buffering is allowed only as a bounded Rust-owned service-local durable buffer behind Overrid abstractions. Critical tenant, credential, manifest, queue, policy, and accounting-adjacent mutations fail closed unless the source service can atomically record the domain mutation and matching append-buffer entry.
- Overwatch persists state through Overrid-owned abstractions or Overrid-shaped local stubs during early phases. It must not make PostgreSQL, Redis, Kafka, NATS, RabbitMQ, S3, MinIO, Vault, cloud KMS, or similar products the platform boundary.
- Overwatch stores refs, hashes, summaries, redaction metadata, and event payloads allowed by evidence policy. It is not an unrestricted data lake for raw private payloads, raw secrets, private service memory, payment details, or native-app business content.
- Overwatch does not own tenant truth, identity proof, credential custody, manifest truth, queue state, policy finality, dispute judgment, reputation scoring, accounting settlement, grant allocation, payout eligibility, compliance conclusions, or governance approval.
- Evidence docs stay structural. They must not encode pricing tables, revenue projections, customer-count assumptions, blockchain mechanics, NFT mechanics, or public-market shortcut behavior.

## Phase 1: SDS Attachment, Evidence Authority, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #15.**
  - Design: Link this document from the numbered Overwatch SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/control_plane/overwatch.md`, `docs/service_catalog/control_plane/overwatch.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #15 returns both the Overwatch SDS and this sub-build plan.

- **1.2 Freeze Overwatch as append-only evidence infrastructure.**
  - Design: Record that Overwatch owns event append, audit append, sequence metadata, trace/correlation metadata, evidence bundles, incident timelines, health records, retention/export metadata, and tamper-evidence checkpoints.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overwatch owns evidence infrastructure but not domain truth, policy finality, dispute judgment, accounting settlement, reputation, compliance decisions, or raw private data custody.

- **1.3 Preserve master Phase 1 as the first build point.**
  - Design: Keep first implementation in master Phase 1 because signed control-plane commands need append-only request, manifest, queue, key, tenant, and health evidence before the private swarm and execution loop start.
  - Output: Phase-gate note that SDS #15 starts in Phase 1 and expands later through trust, execution, accounting, product, grid-resident, storage, federation, public-provider, native-app, and governance gates.
  - Validation: Review proves this plan does not move Overwatch into Phase 0 and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #15 decisions for BLAKE3/prior-hash signed checkpoints, event-family/privacy-class retention, bounded Rust-owned append buffering, canonical JSON/JSON Schema evidence bundles, and caller-safe trace summaries.
  - Output: Resolved-decision checklist tied to SDS #15 open-question answers.
  - Validation: Review rejects plans that require mature consensus in Phase 1, use a single global retention rule, make Redis/Kafka/NATS the append buffer, treat human-readable reports as authoritative bundle data, or expose raw private fields in user traces.

- **1.5 Define runtime authority boundaries.**
  - Design: Require append APIs to be service-account restricted while query and bundle APIs enforce tenant, role, data-class, and evidence-purpose filtering.
  - Output: Boundary matrix for Overgate, Overpass, Overtenant, Overkey, Overregistry, Overqueue, Overguard, Oververify, Overclaim, Overmeter, ORU, Seal Ledger, Overbill, central AI, compliance, admin UI, and public reports.
  - Validation: Design review rejects direct event mutation by domain services, unrestricted event queries, raw private payload dumps, public evidence leaks, and Overwatch-owned policy/accounting/dispute conclusions.

## Phase 2: Rust Service Skeleton, Schemas, And Local Fixtures

### Work Items

- **2.1 Create the Overwatch Rust service crate.**
  - Design: Add an Overwatch service crate under the control-plane workspace using Tokio, Axum/Tower/Hyper-style HTTP, shared config loading, tracing setup, and dependency injection for storage, schema validation, Overkey service-account verification, Overtenant query filtering, and internal clients.
  - Output: Service crate, module layout, local-stack service entrypoint, handler boundaries, repository traits, and test harness hooks.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Overwatch stays separate from Overgate, Overpass, Overtenant, Overkey, Overregistry, Overqueue, Overguard, Oververify, Overclaim, Overmeter, and accounting internals.

- **2.2 Define canonical evidence schemas.**
  - Design: Add shared schemas for `event_record`, `audit_record`, `trace_record`, `evidence_bundle`, `incident_record`, `health_record`, `retention_record`, `export_record`, checkpoint records, append requests, query filters, API errors, and Overwatch meta-events.
  - Output: JSON Schema files, Rust generated or hand-written types, compatibility fixtures, and stable reason-code enums.
  - Validation: Schema tests reject missing event id, source service, event type, tenant id where required, trace id, command id, schema version, sequence number, privacy class, payload hash, prior hash, and evidence refs where required.

- **2.3 Define Phase 1 source-service fixtures.**
  - Design: Model Overgate, Overpass, Overtenant, Overkey, Overregistry, and Overqueue as Phase 1 appenders with deterministic service ids, service-account refs, event families, privacy classes, source sequences, and failure cases.
  - Output: Valid and invalid append fixtures for accepted request, denied request, key change, tenant state, manifest accepted, queue pending, health event, duplicate event, sequence gap, and invalid schema.
  - Validation: Fixture tests prove Phase 1 event families can reconstruct a signed API request to manifest and pending queue state without relying on private service logs.

- **2.4 Implement Overrid-owned evidence storage boundaries.**
  - Design: Define repositories for immutable event bodies, sequence indexes, trace indexes, audit indexes, health indexes, incident timelines, checkpoints, retention markers, export records, and local Overrid-shaped storage stubs.
  - Output: Repository traits, local storage adapter, migration hooks, append-only write model, derived-index rebuild model, and checkpoint-read model.
  - Validation: Tests prove prior event bodies cannot be overwritten or deleted through normal APIs and storage never persists raw secrets, unrestricted private payloads, payment details, mutable hidden overrides, or unmanaged private service memory.

- **2.5 Connect local development and integration harness fixtures.**
  - Design: Wire Overwatch into the loopback-only local stack and integration harness with deterministic tenants, identities, service accounts, source services, event sequences, traces, bundles, incidents, health records, and retention scenarios.
  - Output: Local service config, seed evidence fixtures, checkpoint fixtures, redaction fixtures, and harness scenario names.
  - Validation: Local smoke tests can append an Overgate request event, append an Overregistry manifest event, append an Overqueue pending event, query a caller-safe trace, and verify checkpoint metadata without production credentials or non-Overrid product dependencies.

## Phase 3: Append-Only Event Storage, Audit Records, And Tamper Evidence

### Work Items

- **3.1 Implement service-account event append.**
  - Design: Support `POST /v1/events` for authorized service accounts with source service, source sequence, event type, tenant scope, actor ref, subject ref, trace id, command id, schema version, privacy class, payload refs, payload hash, prior event hash, and idempotency key.
  - Output: Event append handler, service-account authorization, schema validator, idempotent response behavior, and `overwatch.event_appended` or `overwatch.event_rejected` meta-event payloads.
  - Validation: API tests cover valid append, missing service account, unknown source service, invalid schema, duplicate event id, conflicting duplicate payload, missing source sequence, missing payload hash, and missing privacy class.

- **3.2 Implement audit record append.**
  - Design: Support `POST /v1/audit-records` for admitted command decisions and protected state transitions with command id, actor id, target ref, action, decision, reason code, policy refs, state-transition refs, evidence refs, signature refs, trace id, and tenant scope.
  - Output: Audit append handler, audit schema validator, audit index, and audit append events.
  - Validation: Audit tests prove every mutating command fixture has actor, target, decision, reason code, trace id, tenant scope, evidence refs, schema version, and service signature where required.

- **3.3 Implement per-source sequence checks.**
  - Design: Track monotonic per-source sequence heads, duplicate ids, sequence gaps, conflict state, and block/degrade behavior according to source criticality.
  - Output: Sequence repository, source-head read model, duplicate handling, sequence-gap markers, and blocked-source reason codes.
  - Validation: Sequence tests prove identical duplicates return prior append result, conflicting duplicates are rejected, critical source gaps block the source stream, and allowed gap markers remain auditable.

- **3.4 Implement Phase 1 checkpointing.**
  - Design: Create Overwatch-owned checkpoint records covering source stream heads, global checkpoint ranges, schema versions, service identities, BLAKE3 payload/prior-hash continuity, generated time, and signature refs.
  - Output: Checkpoint creation job, checkpoint query, checkpoint event, and checkpoint coverage report.
  - Validation: Integrity tests verify payload hashes, prior-event hashes, source head coverage, global range coverage, checkpoint signature refs, duplicate handling, and trace continuity.

- **3.5 Implement append failure behavior.**
  - Design: Return stable reason codes for invalid schema, unknown source service, unauthorized service account, duplicate conflict, sequence gap, privacy-class error, checkpoint error, and storage unavailable.
  - Output: Error contract, retryability flags, source-service handoff guidance, and failure-mode fixtures.
  - Validation: Failure tests prove critical mutation append failures force fail-closed behavior unless a valid atomic local append buffer is available, while low-risk telemetry can degrade only to signed loss counters or summaries.

## Phase 4: Trace Reconstruction, Query APIs, And Privacy Filters

### Work Items

- **4.1 Implement trace reconstruction.**
  - Design: Build trace summaries from append-only events using root request id, command refs, tenant id, actor id, service-hop events, manifest refs, queue refs, policy refs, execution refs, current status, first seen, last seen, and terminal reason.
  - Output: Trace indexer, trace read model, terminal-state detector, and `overwatch.trace_completed` event payload.
  - Validation: Trace tests reconstruct Overgate to Overregistry to Overqueue Phase 1 flow and later queue/scheduler/lease/runner/metering paths from event refs.

- **4.2 Implement caller-safe trace reads.**
  - Design: Support `GET /v1/traces/{trace_id}` with field filtering so ordinary users see only request id, command id, visible tenant/app context, submitted time, last updated time, lifecycle state, terminal reason, user-facing explanation, allowed refs, retry/idempotency outcome, and next allowed action.
  - Output: Trace read handler, caller-visible field policy, admin/operator extension policy, and redaction fixtures.
  - Validation: Privacy tests prove user trace summaries hide raw payloads, private refs, service-account secrets, other tenant members, operator notes, private policy inputs, node/provider details unless authorized, and cross-tenant correlation data.

- **4.3 Implement event reads and event queries.**
  - Design: Support `GET /v1/events/{event_id}` and `GET /v1/events:query` filtered by tenant, actor, subject, command, trace, source service, event type, privacy class, reason code, and time window.
  - Output: Event read handler, query handler, pagination, field policy matrix, and query reason-code mapping.
  - Validation: Query tests prove tenant isolation, role filtering, data-class filtering, evidence-purpose filtering, pagination stability, time-window correctness, and denial of unrestricted event scans.

- **4.4 Implement audit query APIs.**
  - Design: Provide audit query surfaces for command decisions, policy decisions, state transitions, incident refs, dispute refs, and operator actions without exposing raw private payloads.
  - Output: Audit query handler, admin/operator filter set, tenant-visible audit summary, and evidence-ref response shape.
  - Validation: Audit query tests prove users can see their own request decisions while operators and compliance callers require explicit role, tenant, data-class, and evidence-purpose checks for deeper refs.

- **4.5 Implement derived-index rebuilds.**
  - Design: Rebuild trace, event-query, audit-query, incident, health, and retention indexes from append-only event bodies and checkpoint records.
  - Output: Rebuild command, rebuild progress records, corruption reports, and verification summaries.
  - Validation: Recovery tests destroy derived indexes in a fixture, rebuild them from append-only records, and verify trace, event, audit, incident, health, and checkpoint query equivalence.

## Phase 5: Evidence Bundles, Retention, Redaction, And Exports

### Work Items

- **5.1 Implement evidence bundle generation.**
  - Design: Support `POST /v1/evidence-bundles` with purpose, requester id, tenant scope, included event refs, redaction rules, export format, generated time, integrity hash, and audit refs.
  - Output: Evidence bundle handler, canonical JSON bundle manifest, JSON Schema validation, optional deterministic NDJSON event stream, optional human-readable rendering, and `overwatch.evidence_bundle_created` event payload.
  - Validation: Bundle tests prove canonical JSON plus JSON Schema is authoritative and optional NDJSON/renderings are generated from the same bundle manifest.

- **5.2 Implement purpose-specific bundle profiles.**
  - Design: Define dispute, central AI review, compliance, public report, incident, stewardship, migration, and governance bundle profiles with allowed event families, redaction requirements, provenance fields, confidence limits, and private-data exclusions.
  - Output: Bundle-profile schemas, profile validation, and fixture bundles.
  - Validation: Tests prove dispute bundles include timeline and decision refs, central AI bundles exclude raw private data by default, compliance bundles include retention/export refs, and public-report bundles expose only aggregates or redacted timelines.

- **5.3 Implement event-family and privacy-class retention.**
  - Design: Store retention rules by event family, privacy class, tenant scope, legal hold refs, archive refs, redaction markers, expiry rules, and summary-rollup rules for high-volume health/readiness/metric/debug families.
  - Output: Retention schema, policy application job, retention reports, and `overwatch.retention_policy_applied` event payload.
  - Validation: Retention tests prove permanent refs and integrity metadata remain for protocol, identity, tenant, key, registry, queue, checkpoint, security, incident, dispute, accounting, grant, payout, and governance evidence.

- **5.4 Implement redaction markers and restricted views.**
  - Design: Represent accidental private-data append handling as redaction markers, access restrictions, incident evidence, and emitting-service correction refs without deleting prior event existence or hashes.
  - Output: Redaction marker schema, query transformation rules, incident handoff refs, and correction guidance.
  - Validation: Redaction tests prove restricted callers cannot read private payload details while authorized incident/compliance paths can prove event existence, hash continuity, redaction reason, and corrective follow-up refs.

- **5.5 Implement export records and artifact refs.**
  - Design: Store export id, requester, purpose, scope, evidence bundle ref, generated artifact ref, redaction policy, audit refs, integrity hash, format, and retry/failure state.
  - Output: Export record schema, export job status, artifact-ref handoff to Overstore/Overvault where appropriate, and audit-safe status views.
  - Validation: Export tests prove exports cite bundle manifests, retain integrity hashes, enforce purpose filters, reject public private-data leaks, and never become a payment, billing, or governance approval mechanism.

## Phase 6: Policy, Dispute, Execution, Usage, And Accounting Evidence Handoff

### Work Items

- **6.1 Add policy and dry-run evidence handoff.**
  - Design: Store Overguard and policy dry-run evidence refs for matched rules, denied conditions, data sensitivity, egress limits, package trust, provider eligibility, quota checks, and estimated placement class.
  - Output: Policy evidence event family, decision-ref schema, and policy replay fixture set.
  - Validation: Policy tests prove Overwatch stores replayable evidence but does not decide policy, modify policy versions, or silently override Overguard decisions.

- **6.2 Add verification, challenge, reputation, and dispute evidence refs.**
  - Design: Store Oververify, challenge task, reputation, anti-Sybil, Overclaim, dispute, hold, correction, and settlement-window refs without adjudicating outcomes.
  - Output: Trust/dispute event families, challenge evidence refs, dispute timeline refs, and hold/correction evidence refs.
  - Validation: Trust/dispute tests prove Overwatch can assemble evidence for Oververify and Overclaim while leaving trust scoring, challenge adjudication, dispute outcomes, and payout holds with owning services.

- **6.3 Add execution-loop evidence handoff.**
  - Design: Store queue, scheduler, lease, runner, package, result, retry, cancellation, timeout, dead-letter, cache, mesh, and execution state events from the private execution loop.
  - Output: Execution event family schemas, trace-link rules, runner handoff refs, and execution replay fixtures.
  - Validation: Execution tests prove successful, failed, cancelled, timed-out, retried, and dead-lettered workloads have distinct replayable event chains.

- **6.4 Add usage and accounting evidence refs.**
  - Design: Store Overmeter raw usage refs, rollup refs, ORU account refs, Seal Ledger refs, Overbill refs, Overgrant refs, Overasset refs, provider payout refs, refund refs, and correction refs without mutating balances or invoices.
  - Output: Accounting evidence schemas, usage-relevant event checklist, and accounting handoff contract.
  - Validation: Accounting tests prove Overwatch provides event refs to Overmeter, ORU, Seal Ledger, Overbill, Overgrant, Overasset, and provider payout services while never settling usage or embedding charge logic.

- **6.5 Implement evidence consistency checks.**
  - Design: Add reports for missing policy refs, missing audit refs, orphaned trace refs, missing usage refs, missing ledger refs, unmatched dispute refs, and bundle refs that cite inaccessible event families.
  - Output: Consistency-check jobs, audit-safe reports, remediation reason codes, and incident handoff rules.
  - Validation: Consistency tests prove high-severity missing evidence creates Overwatch incident or health events and low-risk report gaps are retryable without blocking unrelated tenants.

## Phase 7: Health, Incidents, Observability, And Operator Workflows

### Work Items

- **7.1 Implement health event records.**
  - Design: Support service id, instance id, grid node ref, health state, readiness state, dependency state, version, event refs, observed time, tenant impact class, and source service signature.
  - Output: Health record schema, health append path, health query, health summary, and health event fixtures.
  - Validation: Health tests prove liveness and readiness are distinct, dependency state is explicit, labels avoid private data, and health summaries never expose raw tenant payloads.

- **7.2 Implement incident records and timelines.**
  - Design: Support incident id, severity, affected services, affected tenants, timeline refs, current state, owner refs, mitigation refs, closure refs, reopened state, and evidence bundle refs.
  - Output: Incident record schema, `GET /v1/admin/incidents`, `POST /v1/admin/incidents`, timeline append rules, and `overwatch.incident_opened` / `incident_updated` events.
  - Validation: Incident tests cover opened, triaged, mitigating, monitoring, resolved, closed, and reopened states without editing prior timeline evidence.

- **7.3 Implement observability metrics and traces.**
  - Design: Expose event append rate, rejection rate, query latency, trace completion rate, checkpoint coverage, per-service event lag, schema error counts, storage growth, incident counts, retention status, and export job state.
  - Output: Rust tracing/OpenTelemetry-compatible metrics, audit-safe labels, dashboard query shapes, and Overwatch-as-authoritative-evidence guidance.
  - Validation: Metrics tests prove labels avoid private data, tenant leakage to low-privilege callers, raw hashes where not public, secrets, payment details, and unbounded high-cardinality values.

- **7.4 Implement operator diagnostics.**
  - Design: Provide operator views for stuck traces, source sequence gaps, append-buffer activation, failed appends, checkpoint gaps, retention drift, index rebuilds, export failures, and incident timelines.
  - Output: Operator diagnostic endpoints or admin UI contract, reason-code mapping, and runbook refs.
  - Validation: Operator tests prove diagnostics require role, tenant, data-class, and evidence-purpose checks and do not expose raw private payloads by default.

- **7.5 Implement health-to-incident automation hooks.**
  - Design: Convert sustained dependency failures, append failures, checkpoint gaps, query index corruption, storage corruption, and repeated privacy violations into incident candidates with evidence refs.
  - Output: Incident-candidate rules, severity mapping, dedupe behavior, and notification/event refs.
  - Validation: Automation tests prove repeated failures create bounded incident candidates, dedupe correctly, preserve evidence refs, and avoid noisy unbounded incident creation.

## Phase 8: Buffering, Recovery, Backup, Restore, And Grid-Resident Hardening

### Work Items

- **8.1 Implement bounded append buffering contract.**
  - Design: Define source-service local durable buffer requirements for original trace id, command id, source sequence, payload hash, privacy class, service signature, atomic domain mutation link, replay idempotency, sequence-gap handling, and buffer limit events.
  - Output: Buffer contract, buffer status events, replay contract, drain behavior, and source-service integration fixtures.
  - Validation: Buffer tests prove critical mutations fail closed without atomic buffer support, low-risk telemetry can degrade only to signed loss counters, and buffer start/drain/limit events are visible to operators.

- **8.2 Implement replay and rebuild recovery.**
  - Design: Reconstruct derived indexes, trace summaries, incident timelines, health summaries, retention reports, and export status from append-only records and checkpoints after corruption or restore.
  - Output: Replay command, rebuild reports, checkpoint verification, source-sequence verification, and recovery evidence bundle.
  - Validation: Recovery tests prove replay detects missing records, conflicting duplicates, broken prior hashes, checkpoint mismatch, source gaps, and derived-index drift.

- **8.3 Define backup and restore handoff.**
  - Design: Coordinate with backup/restore services, Overstore, Overvault, and grid-resident storage to preserve append-only records, checkpoints, archive refs, retention refs, export refs, and restore attestations.
  - Output: Backup manifest fields, restore manifest fields, restore drill checklist, and integrity-attestation refs.
  - Validation: Restore tests prove restored Overwatch records pass hash, sequence, checkpoint, schema, trace, retention, and export integrity checks.

- **8.4 Prepare grid-resident operations behavior.**
  - Design: Define system-service workload needs for Overwatch, including protected placement, replicated state, backup, restore, failover, rolling update, rollback, maintenance mode, break-glass controls, and incident runbooks.
  - Output: Phase 7 operations checklist for Overwatch, readiness/degraded-state behavior, and failover evidence requirements.
  - Validation: Grid-readiness review confirms founder seed hardware can later be removed from the normal path without changing append, query, checkpoint, trace, or evidence-bundle contracts.

- **8.5 Add archive, retention, and deletion-proof hardening.**
  - Design: Connect retention and archive state to Overbase/Overstore/Overvault/namespace/accounting/compliance refs while preserving permanent evidence refs and privacy-class restrictions.
  - Output: Archive refs, deletion-proof refs, redaction-proof refs, retained-evidence reports, and governance handoff fields.
  - Validation: Archive tests prove old health/debug data can roll into signed summaries while permanent security, dispute, accounting, governance, and checkpoint refs remain replayable.

## Phase 9: SDK, CLI, Admin, Product, Federation, Native-App, And AI Handoff

### Work Items

- **9.1 Harden SDK and CLI evidence bindings.**
  - Design: Provide generated Rust-first SDK/CLI flows for event append diagnostics, trace reads, event queries, evidence bundles, incident reads, health summaries, checkpoint reports, and export status.
  - Output: SDK/CLI contract examples, stable JSON output shapes, reason-code mappings, and troubleshooting flows.
  - Validation: SDK/CLI tests prove clients pass trace ids, decode stable reason codes, honor pagination, respect tenant filters, and do not call privileged append APIs without service-account authority.

- **9.2 Implement admin and developer UI evidence views.**
  - Design: Expose tenant-isolated views for traces, audit records, service health, event lag, failed appends, sequence gaps, incidents, bundles, exports, retention, redaction markers, and checkpoint coverage.
  - Output: Admin read-model requirements, UI endpoint contracts, field policy matrix, and operator workflow checklist.
  - Validation: Admin tests prove authorized operators can diagnose system evidence while tenant users cannot see cross-tenant private metadata, raw payloads, service-account refs, private hashes, or unrestricted evidence bundles.

- **9.3 Define product and adapter evidence handoff.**
  - Design: Document how Docdex, Mcoda, Codali, AI gateway, encrypted RAG, runtime bridge, node agents, workers, and product clients emit and consume trace/evidence refs.
  - Output: Product evidence checklist, adapter event-family contracts, and integration fixtures for Phase 6 and later consumers.
  - Validation: Product integration tests fail when clients omit tenant ids, trace ids, idempotency keys, service-account signatures, privacy classes, stable reason codes, or audit refs.

- **9.4 Define federation, public-provider, and native-app evidence rules.**
  - Design: Provide evidence rules for trusted federation, public-interest pools, public provider onboarding, public sandbox, fraud controls, purpose tags, wallet, personal AI, workspace, directory, search, messaging, social, maps, mobile, and stewardship clients.
  - Output: Federation/native/public evidence checklist, public-safe redaction rules, user-facing trace projection schema, and public-report handoff refs.
  - Validation: Federation/native tests prove public and native clients use normal Overwatch query/bundle APIs, respect private/public scope, and cannot bypass tenant, role, data-class, or evidence-purpose filtering.

- **9.5 Define central AI and governance evidence access.**
  - Design: Give central AI, stewardship reporting, compliance, threat modeling, incident response, migration tooling, and PIP governance purpose-scoped evidence refs and summaries rather than raw private data.
  - Output: Central AI bundle profile, governance bundle profile, compliance export profile, and private-data exclusion checklist.
  - Validation: Governance tests prove central AI and reports can cite evidence refs, provenance, confidence limits, and redaction metadata without receiving raw private data by default.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #15`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first control-plane implementation, Axum/Tower/Hyper-style HTTP, Ed25519, BLAKE3, signed command/service-account envelopes, canonical JSON plus JSON Schema, native Overwatch evidence, bounded Rust-owned buffering, and Overrid-owned storage boundaries.
  - Output: Tech-stack alignment checklist for Overwatch.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #15 is represented as a Phase 1 control-plane service with later hardening through trust/policy, accounting evidence, product clients, grid-resident operation, storage/archive refs, federation/public pools, native apps, and governance/compliance.
  - Output: Updated master-plan and crosswalk rows for SDS #15.
  - Validation: Review confirms only per-SDS sub-build indexing changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #15 and the Overwatch service plan link back to this sub-build plan and preserve Overwatch as append-only evidence infrastructure.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare Overwatch implementation gates.**
  - Design: Require tests for event append, audit append, service-account auth, schema validation, duplicate handling, source sequences, BLAKE3 payload/prior hashes, checkpoints, traces, caller-safe summaries, event/audit queries, evidence bundles, retention, redaction, exports, policy/dispute/execution/usage evidence, health, incidents, metrics, buffering, recovery, backup/restore, grid operation, SDK/CLI/admin/product/federation/native/governance handoffs, and documentation links.
  - Output: Final validation checklist for Overwatch implementation.
  - Validation: Handoff review confirms Overgate, Overpass, Overtenant, Overkey, Overregistry, Overqueue, Overguard, Oververify, Overclaim, Overmeter, ORU, Seal Ledger, Overbill, Overgrant, Overasset, SDK, CLI, admin UI, adapters, federation services, public-provider services, native apps, mobile services, central AI, governance, compliance, and grid-resident system services can depend on Overwatch evidence without moving their runtime authority into Overwatch.

## Alignment Review

- The sub-build plan keeps Overwatch first build work in master Phase 1, matching SDS #15, the service catalog entry, Phase 1 plan, master build plan, and build-plan crosswalk.
- The plan treats master Phase 0 as prerequisite work for shared event/audit schemas, local stack, fixtures, test harness, and shared API/event discipline, not as the Overwatch implementation phase.
- The plan treats later phases as expansion or hardening gates: execution evidence in Phase 3, policy/trust/dispute evidence in Phase 4, usage/accounting refs in Phase 5, product clients in Phase 6, grid-resident health/failover/restore in Phase 7, storage/archive/private-data refs in Phase 8, federation and public-provider evidence in Phases 10 and 11, native/mobile/AI trace consumers in Phase 12, and governance/compliance/public reports in Phase 13.
- The plan carries forward SDS #15 resolved decisions for Phase 1 BLAKE3/prior-hash signed checkpoints, event-family/privacy-class retention, bounded Rust-owned append buffering, canonical JSON/JSON Schema evidence bundles, optional deterministic NDJSON/renderings, and caller-safe user trace summaries.
- The plan keeps Overwatch narrow: no tenant truth, no identity proof ownership, no credential custody, no manifest truth ownership, no queue ownership, no policy finality, no dispute adjudication, no reputation scoring, no ORU or Seal Ledger mutation, no billing or payout ownership, no compliance conclusion ownership, no raw private data lake behavior, and no conventional cloud product-boundary assumptions.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #15 is complete when a builder can implement Overwatch as the Phase 1 Rust control-plane append-only evidence backbone with canonical event/audit/trace/evidence/incident/health/retention/export schemas, service-account append APIs, audit append APIs, idempotent duplicate handling, per-source sequences, BLAKE3 payload and prior-event hashes, signed Phase 1 checkpoints, stable append failure behavior, trace reconstruction, caller-safe trace summaries, tenant/role/data-class/evidence-purpose filtered event and audit queries, derived-index rebuilds, canonical JSON evidence bundles, purpose-specific bundle profiles, event-family/privacy-class retention, redaction markers, export records, policy/dispute/execution/usage/accounting evidence refs without decision ownership, health records, incident timelines, observability, operator diagnostics, bounded Rust-owned append buffering, replay/rebuild recovery, backup/restore handoffs, grid-resident operation requirements, SDK/CLI/admin/product/federation/native/mobile/central-AI/governance handoff rules, implementation validation gates, and documentation links that preserve the master Phase 0 through Phase 13 order.
