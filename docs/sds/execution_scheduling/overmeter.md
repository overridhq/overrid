SDS #23

# Overmeter SDS

## Purpose

Record raw usage facts and produce signed usage rollups for compute, GPU, memory, storage, network, execution time, RAG retrieval, model inference, cache reuse, and app service activity.

Overmeter is the bridge between execution evidence and accounting. Phase 3 only needs raw usage events for private workloads. Phase 5 converts those events into signed rollups that ORU, Seal Ledger, Overbill, disputes, provider payouts, native app usage displays, and stewardship reporting can consume.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overmeter.md](../../service_catalog/execution_scheduling/overmeter.md) |
| Sub-build plan | [SUB BUILD PLAN #23 - Overmeter](../../build_plan/sub_build_plan_023_overmeter.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md), [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) |

## Service Family

- Family: Execution and scheduling
- Owning layer: Usage facts, normalization, signed rollups, and accounting handoff
- Primary data scope: raw usage events, resource dimensions, attribution refs, lease windows, normalized usage, rollup windows, signatures, dispute windows, correction refs, and export refs
- First build phase from service plan: raw events in [Phase 3](../../build_plan/phase_03_private_execution_loop.md); signed rollups in [Phase 5](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md)

## Problem Statement

Overrid cannot account fairly for distributed resources if usage is reconstructed from logs after the fact. Each workload needs durable raw usage events tied to tenant, provider, node, workload, lease, policy, and result evidence. Later, those raw events must roll up into signed accounting records without blockchain, NFT mechanics, or per-operation external payment friction.

Overmeter owns that usage-evidence path. It records what resources were consumed and how the numbers were derived; it does not decide prices, settle accounts, or edit ledger history.

## Goals

- Capture raw usage events from Overrun, Overcell, Overcache, Overmesh, model routing, RAG, storage, and native app services.
- Normalize CPU, GPU, memory, storage, network, data, request, model, and service dimensions with explicit units.
- Tie every workload usage event to tenant, actor/app, provider, node, workload, queue item, lease, policy refs, and audit refs where applicable.
- Produce deterministic rollup windows from raw events.
- Sign rollups with enough refs for Seal Ledger, Overbill, disputes, and audit replay.
- Support retention and dispute windows before settlement finality.
- Preserve corrections as new records rather than editing accepted usage history.

## Non-Goals

- Do not set pricing or market rates. Overmark and accounting policy handle bounded reference rates.
- Do not move ORU balances or write Seal Ledger settlement entries directly.
- Do not replace Overwatch audit events; Overmeter stores usage facts and refs.
- Do not trust unverified node-reported metrics without source and confidence metadata.
- Do not hide GPU, storage, network, or model costs inside a vague single metric.
- Do not call external payment rails for tiny usage events.

## Primary Actors And Clients

- Overrun emitting execution metrics, result state, logs refs, and output refs.
- Overlease supplying usage windows and lease ids.
- Overcell supplying node availability and agent overhead facts.
- Overcache and Overmesh emitting cache and network usage.
- AI Gateway Router, encrypted Docdex RAG adapter, and native apps emitting model/RAG/service usage.
- Seal Ledger, ORU Account Service, Overbill, Overclaim, Provider Payout Service, Wallet and Usage Center, and stewardship reporting consuming signed rollups.
- Operators and users inspecting usage, holds, disputes, and receipts.

## Dependencies

- [Overrun](overrun.md) for workload result and metric events.
- [Overlease](overlease.md) for lease usage windows.
- [Overwatch](../control_plane/overwatch.md) for audit/event refs and integrity evidence.
- [Overkey](../control_plane/overkey.md) for signer credentials and signature verification.
- [Seal Ledger](../accounting/seal_ledger.md), ORU Account Service, and Overbill for Phase 5 accounting consumption.
- Shared schema package for usage event and rollup schemas.
- Overclaim for dispute holds and correction refs.

Phase 3 should emit raw usage without settlement. Phase 5 should add signed rollups and accounting handoff.

## Owned Responsibilities

Overmeter owns:

- Raw usage event schema, validation, idempotency, deduplication, and retention.
- Resource dimension definitions and unit normalization.
- Attribution rules for tenant, actor, app/service, provider, node, workload, queue item, lease, package, model, and route refs.
- Rollup-window definition, deterministic aggregation, signatures, and replay metadata.
- Dispute-window metadata and correction records.
- Export surfaces for accounting, wallet, provider, operator, and stewardship views.

Overmeter must not mutate ledger balances, decide billing amounts, or suppress disputed usage without an explicit correction/hold record.

## Data Model

The first implementation should define:

- `usage_event`: event id, source service, event type, tenant id, actor/app id, provider id, node id, workload id, queue item id, lease id, resource dimensions, timestamp, trace id, source evidence refs, and confidence.
- `resource_dimension`: dimension name, unit, allowed precision, aggregation rule, source types, and accounting mapping hint.
- `usage_source_ref`: Overrun result ref, Overcell heartbeat/command ref, Overcache entry ref, Overmesh transfer ref, model request ref, RAG retrieval ref, storage/object ref, and Overwatch event refs.
- `usage_normalization_record`: raw event ids, normalizer version, normalized dimensions, dropped/adjusted values, reason codes, and replay refs.
- `rollup_window`: window id, tenant/provider/node/workload scope, start/end timestamps, status, included raw event refs, and dispute-window timestamps.
- `signed_usage_rollup`: rollup id, dimensions, totals, source refs, policy refs, signature refs, generated_at, settlement eligibility, and export refs.
- `usage_dispute_marker`: rollup or event refs, dispute id, hold status, reason, proposed correction, and final correction refs.
- `usage_correction`: original refs, corrected dimensions, actor/service, reason, signature, and ledger/export impact refs.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

Phase 3:

- `POST /usage/events`: ingest one or more raw usage events from trusted services.
- `GET /usage/events/{event_id}`: read authorized raw event metadata.
- `GET /usage/events`: query usage by tenant, workload, lease, node, provider, source, or time range.
- `POST /usage/normalize`: run bounded normalization for a source/time range.

Phase 5:

- `POST /usage/rollups`: create a deterministic signed rollup window.
- `GET /usage/rollups/{rollup_id}`: read rollup totals, source refs, and signatures.
- `POST /usage/rollups/{rollup_id}/hold`: mark a rollup under dispute or correction review.
- `POST /usage/corrections`: append a correction record with evidence.
- `POST /usage/exports/seal-ledger`: export signed rollups for accounting consumption.

API requirements:

- Mutating calls require service identity, schema version, trace id, idempotency key, and source evidence refs.
- Ingestion must deduplicate by source service, source event id, workload id, and idempotency key.
- Rollups must be reproducible from raw events and normalizer versions.
- Reads must apply tenant/provider/user/operator visibility rules.
- Disputed usage remains visible with hold state rather than disappearing.

## Event Surface

- `overmeter.usage_ingested`: raw usage event accepted.
- `overmeter.usage_rejected`: raw usage event rejected with reason.
- `overmeter.usage_normalized`: raw events normalized into accounting dimensions.
- `overmeter.rollup_created`: signed rollup created.
- `overmeter.rollup_exported`: rollup exported to accounting service.
- `overmeter.rollup_held`: rollup placed under dispute or correction hold.
- `overmeter.correction_recorded`: usage correction appended.
- `overmeter.retention_applied`: retention policy applied to raw events or exports.

Events should contain usage dimensions and refs, not private workload payloads.

## Core Workflow

1. Overrun emits raw usage for CPU, GPU, memory, storage, network, wall time, queue/lease refs, and final state.
2. Overmeter validates schema, identity, idempotency, source refs, and resource dimensions.
3. Raw usage is stored with trace, lease, workload, node, provider, tenant, and policy refs.
4. Normalization converts raw metrics into explicit Overrid resource dimensions.
5. Phase 5 rollup windows aggregate usage deterministically and sign totals.
6. Rollups enter dispute windows before settlement finality.
7. Seal Ledger/ORU/Overbill consume signed rollups and produce ledger/accounting records.
8. Corrections append new records and point to affected events/rollups.

## State Machine

Raw event lifecycle:

1. `received`: event accepted for validation.
2. `accepted`: schema, identity, and idempotency checks passed.
3. `normalized`: event contributed to normalized dimensions.
4. `rolled_up`: event included in a signed rollup.
5. `held`: event or derived rollup is under dispute/correction hold.
6. `corrected`: event remains immutable but has a correction ref.
7. `rejected`: event failed validation and cannot be used.
8. `retained_summary`: raw detail aged out while signed summary remains.

Rollup lifecycle:

1. `draft`: rollup window selected and source refs gathered.
2. `computed`: totals calculated from raw refs and normalizer versions.
3. `signed`: rollup signed and ready for accounting export.
4. `exported`: rollup handed to accounting layer.
5. `held`: dispute or correction prevents settlement finality.
6. `settlement_ready`: dispute window passed or hold released.
7. `superseded`: correction generated a replacement rollup.

## Policy And Security

- Only trusted service accounts may submit usage events.
- Source refs are mandatory for events that affect accounting.
- Node-reported metrics need confidence/source metadata and may require Overrun/control-plane corroboration.
- Usage records must not include raw secrets, private documents, prompts, outputs, or logs where refs are enough.
- Tenant/provider reads must be scoped; providers see usage relevant to their resources, tenants see their workloads, operators see restricted diagnostic views.
- Rollups must be signed and replayable from immutable raw event refs.
- Corrections must append new records instead of mutating accepted events.
- Retention must preserve enough data for disputes, audit, receipts, and stewardship reports.

## Metering And Accounting

Overmeter is the metering authority, so this section defines how its own output feeds accounting:

- Raw events are not billing decisions.
- Signed rollups are accounting inputs for ORU Account Service, Seal Ledger, Overbill, provider payouts, grants, and wallet displays.
- Rollups must separate dimensions such as CPU, GPU, memory, storage, network, data, model inference, RAG retrieval, cache, and service units.
- Dispute windows and holds must block settlement finality without deleting usage evidence.
- Internal accounting should use rollups and ORU/Seal Ledger transitions, not external payment calls for each small operation.

## Observability And Operations

- Operators need ingestion rate, rejected event rate, missing source refs, normalization failures, rollup backlog, held rollups, correction counts, and export lag.
- Health checks should confirm event store, schema registry, signer availability, Overwatch refs, and accounting export path.
- Backfills must be bounded, replayable, and tied to normalizer versions.
- Dashboards should compare raw event counts to rollup totals and identify drift.
- Retention jobs should support dry-run previews before dropping raw detail.

## Failure Modes And Recovery

- Duplicate event: deduplicate and return the original accepted event id.
- Missing lease or workload refs: accept only as non-settlement diagnostic usage or reject if accounting-relevant.
- Source service unavailable: continue ingesting other sources and mark rollup incomplete until required refs arrive.
- Normalization error: preserve raw events and fail the normalization batch with reason code.
- Signer unavailable: keep computed rollup unsigned and retry; do not export unsigned rollups.
- Dispute arrives after export: append hold/correction state and route to accounting correction flow.
- Raw retention runs too early: block retention until dispute and receipt requirements are satisfied.

## Validation Plan

The service implementation plan lists these requirements:

- Workloads produce usage records tied to tenant, provider, node, workload, and lease.
- Rollups are reproducible from raw events.
- Disputed usage can be held before settlement finality.

Additional SDS-level validation:

- Contract tests for event ingestion, event query, normalization, rollup creation, hold, correction, and export APIs.
- Deduplication and idempotency tests across source retries.
- Unit normalization tests for CPU, GPU, memory, storage, network, wall time, model inference, and RAG retrieval.
- Replay tests proving signed rollups are reproducible from raw refs.
- Tenant/provider visibility tests.
- Dispute tests proving held usage blocks settlement finality.
- Correction tests proving history is append-only.

## Build Breakdown

1. Define raw usage event schema, resource dimensions, attribution refs, and idempotency rules.
2. Ingest Overrun raw usage for the first private workload loop.
3. Link usage to Overlease windows and Overwatch evidence.
4. Add normalization for CPU, GPU, memory, storage, network, wall time, queue wait, and model inference count.
5. Add signed rollup windows in Phase 5.
6. Add dispute windows, holds, corrections, and Seal Ledger/Overbill export.
7. Add native app, RAG, cache, mesh, and provider reporting dimensions as services mature.

The Phase 3 target is raw usage visibility; Phase 5 turns it into signed accounting evidence.

## Handoff And Downstream Use

Overmeter feeds ORU accounts, Seal Ledger, Overbill, provider payouts, Overclaim, wallet usage displays, native service reporting, and stewardship reporting. Downstream accounting services must consume signed rollups and correction records rather than reinterpreting raw logs.

## Open Design Questions

- Resolved: the first private workload is usage-valid only when its raw event includes tenant id, provider id, node id, workload id, queue item id, lease id, run attempt id, final state, trace/audit refs, start/end timestamps, wall time, queue wait or start-delay duration, CPU time, memory peak, storage read/write bytes, network bytes, source evidence refs, source service id, idempotency key, schema version, and confidence/source metadata. GPU time and GPU memory are mandatory when the workload requested or used a GPU; model inference count is mandatory when the workload used a model runtime. Cleanup overhead, cache reuse, RAG retrieval, and mesh transfer dimensions may be absent only when the source contract proves they were not applicable.
- Resolved: Phase 5 should start with deterministic classed rollup windows. Private execution workloads use one rollup per lease/run attempt for workload receipts plus an hourly tenant/provider/node aggregation for settlement and provider-earning batches; long-running workloads also emit hourly partial rollups and a terminal final rollup. Native app and service usage uses short interactive buckets for wallet freshness, defaulting to 15-minute app/service/user-visible summaries and hourly accounting rollups, then daily statement aggregates through Overbill. All windows carry raw event refs, normalizer version, policy refs, signature refs, and dispute-window timestamps; no window may collapse explicit CPU, GPU, MEM, STOR, NET, DATA, model/RAG/cache, or Service-ORU dimensions into a vague total.
- Resolved: node-reported metrics that can affect settlement eligibility require corroboration before they are exported as signed settlement-ready rollups. CPU time, memory peak, GPU time/memory, storage bytes, network bytes, wall time, queue wait/start delay, model runtime counts, and cleanup overhead must be cross-checked against lease windows, Overrun attempt state, Overcell heartbeat/load facts, Hardware Discovery observations, Benchmark Runner measured-capacity evidence where applicable, Overwatch event timing, and source refs such as runtime/container counters. Uncorroborated or contradictory metrics remain raw diagnostic usage, produce `needs_corroboration` or `suspect_source` reason codes, and cannot settle until corrected, held, or backed by accepted evidence.
- Resolved: raw accounting-relevant events remain available at full detail until the longest applicable dispute, receipt, audit, and correction window closes, with a Phase 5 private-workload default of at least 90 days after signed rollup and receipt creation. Events tied to active disputes, corrections, fraud/security review, provider payout holds, grant reporting, or compliance export are pinned until finality plus the configured audit margin. After that point, raw payload-adjacent details may be compacted into retained summaries, but signed rollups, receipts, correction refs, normalizer versions, hashes, and replay metadata must remain available for ledger/accounting replay.
- Resolved: wallet-visible summaries should show redacted usage by account, tenant or organization scope, app/service/native-app/workload class, resource dimension, time window, receipt/refund/correction/hold state, ORU projection refs, and source rollup refs. Users may see their own workload/app names where authorized, coarse provider class or region when useful, and whether usage is settled, held, disputed, corrected, sponsored, grant-funded, or pending. Wallet views must hide provider-private topology, exact node addresses, raw host metrics, fraud signals, private tenant identifiers outside the viewer scope, raw logs, prompts, outputs, secret refs, and ledger internals; deeper evidence access goes through authorized Overbill, Overclaim, Overwatch, or operator export flows with redaction profiles.
