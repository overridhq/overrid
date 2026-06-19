SDS #15

# Overwatch SDS

## Purpose

Build the event, audit, observability, incident, health, reputation, and compliance evidence layer.

Overwatch is the append-only evidence backbone for Overrid. It records accepted and denied requests, state transitions, policy decisions, manifest changes, queue events, execution events, usage facts, incidents, health signals, and later compliance exports so operators, users, central AI, dispute services, and governance processes can replay what happened without trusting hidden service memory.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overwatch.md](../../service_catalog/control_plane/overwatch.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md), [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md), [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md), [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |

## Service Family

- Family: Control plane.
- Owning layer: Append-only events, audit evidence, traces, incidents, health signals, and evidence packages.
- Primary data scope: event records, audit records, trace records, evidence bundles, incident records, health records, retention records, export records, and tamper-evidence metadata.
- First build phase from service plan: event log in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); mature observability through [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md), [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md), and [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md).

## Problem Statement

Overrid cannot govern fraud, disputes, billing, central AI interventions, native service stewardship, or grid-resident operations without durable evidence. If events are mutable, incomplete, or private to each service, the system cannot prove why it accepted, denied, executed, charged, suspended, or corrected anything. Overwatch makes evidence a first-class platform surface.

## Goals

- Store append-only event and audit records for Phase 1 command flow.
- Support trace reconstruction from API request to queue item and later execution result.
- Preserve policy, dispute, usage, health, and incident evidence for later phases.
- Provide tenant-filtered and role-filtered query APIs.
- Provide tamper-evident sequencing strong enough for operational audit and later governance.
- Support evidence packages for Overclaim, Oververify, central AI, compliance, and public reports without exposing private data.
- Support grid-resident backbone operations with health, readiness, failover, migration, and restore evidence.

## Non-Goals

- Do not replace each service's source-of-truth records.
- Do not become an unrestricted data lake of private payloads.
- Do not decide policy, disputes, reputation, or compliance outcomes; Overwatch stores evidence those services cite.
- Do not allow services to edit or delete prior events to hide mistakes.
- Do not expose private tenant evidence in public reports.
- Do not encode pricing, customer-count, or market assumptions.

## Primary Actors And Clients

- Overgate, Overtenant, Overpass, Overkey, Overregistry, and Overqueue emitting Phase 1 events.
- Oversched, Overlease, Overrun, and Overmeter emitting execution and usage events later.
- Overguard, Oververify, Overclaim, reputation, anti-Sybil, and challenge services citing evidence.
- Admin UI, CLI, and SDK reading traces and audit refs.
- Central AI governance workflows reviewing fraud, abuse, grant, and intervention evidence.
- Compliance, incident response, stewardship reporting, and public reporting services.
- Operators investigating health, outages, stuck queues, disputes, and migrations.

## Dependencies

- Shared event and audit schemas.
- Protocol Core event, reason-code, trace, privacy classification, and state-transition rules.
- Overgate ingress events.
- Overqueue, registry, tenant, key, identity, and execution state events.
- Service-account credentials from Overkey for event appenders.
- Overtenant for tenant-filtered event reads.
- Storage and backup layers in later phases for retention, export, and restore.

## Owned Responsibilities

Overwatch owns:

- Event append API and append-only storage.
- Event sequence, trace, and correlation metadata.
- Audit record storage and query.
- Evidence bundle construction from event refs.
- Incident record and timeline storage.
- Health and readiness event storage.
- Retention, redaction marker, and export metadata.
- Tamper-evidence metadata such as hashes, sequence checks, and checkpoint refs.
- Tenant-filtered and operator-filtered query surfaces.

Overwatch does not own the domain truth of tenant roles, queue state, manifests, credentials, accounting finality, or policy decisions.

## Data Model

The first implementation should define these records:

- `event_record`: event id, source service, event type, tenant id, actor id, subject id, trace id, command id, schema version, sequence number, occurred time, received time, privacy class, payload refs, payload hash, and prior event hash.
- `audit_record`: audit id, command id, actor id, target ref, action, decision, reason code, policy refs, state transition refs, evidence refs, and signature refs.
- `trace_record`: trace id, root request id, tenant id, actor id, command refs, event refs, current status, first seen time, last seen time, and terminal reason.
- `evidence_bundle`: bundle id, purpose, requester id, tenant scope, included event refs, redaction rules, export format, generated time, and integrity hash.
- `incident_record`: incident id, severity, affected services, affected tenants, timeline refs, current state, owner refs, mitigation refs, and closure refs.
- `health_record`: service id, instance id, grid node ref, health state, readiness state, dependency state, version, and event refs.
- `retention_record`: event family, tenant scope, retention class, legal hold refs, redaction markers, archive refs, and expiry rules.
- `export_record`: export id, requester, purpose, scope, evidence bundle ref, generated artifact ref, redaction policy, and audit refs.

Payloads should prefer structured refs and hashes. Private content belongs in the owning service or storage layer unless an explicit evidence contract requires a redacted copy.

## API Surface

Phase 1 should expose:

- `POST /v1/events`: append an event from an authorized service account.
- `POST /v1/audit-records`: append an audit record from an authorized service account.
- `GET /v1/traces/{trace_id}`: read caller-visible trace summary and event refs.
- `GET /v1/events/{event_id}`: read allowed event fields.
- `GET /v1/events:query`: query by tenant, actor, subject, command, trace, service, event type, and time window.
- `POST /v1/evidence-bundles`: generate a filtered evidence package.
- `GET /v1/admin/incidents`: query incident timelines.
- `POST /v1/admin/incidents`: create or update incident records through signed operator commands.
- `GET /v1/health/events`: read service health event summaries.

Append APIs must be service-account restricted. Query APIs must filter by tenant, role, data class, and evidence purpose.

## Event Surface

Overwatch stores events from the platform and also emits limited meta-events:

- `overwatch.event_appended`: event accepted into append-only storage.
- `overwatch.event_rejected`: event rejected for schema, auth, duplicate, or sequence reasons.
- `overwatch.trace_completed`: trace reached a terminal state.
- `overwatch.evidence_bundle_created`: bundle generated with redaction policy.
- `overwatch.incident_opened`: incident record opened.
- `overwatch.incident_updated`: incident state or timeline changed.
- `overwatch.retention_policy_applied`: retention, archive, legal hold, or redaction marker applied.
- `overwatch.checkpoint_created`: integrity checkpoint created for a sequence range.

Overwatch must never emit an event that rewrites prior evidence. Corrections are new events.

## Core Workflow

1. A service performs a mutating action or denial and creates a schema-checked event or audit record.
2. The service appends the record to Overwatch using service-account credentials.
3. Overwatch validates schema, source service, sequence expectations, privacy class, trace id, and payload refs.
4. Overwatch stores the record with integrity metadata.
5. Trace summaries and event indexes update from append-only records.
6. Query callers receive filtered views according to tenant, role, data class, and evidence purpose.
7. Evidence bundles assemble selected refs, hashes, and redaction metadata for disputes, incidents, compliance, central AI, or public reports.

## State Machine

Event lifecycle:

1. `received`: append request reached Overwatch.
2. `validated`: schema, source service, auth, and privacy checks passed.
3. `appended`: event is durable and included in sequence.
4. `indexed`: event is queryable through trace and search indexes.
5. `checkpointed`: event is covered by integrity checkpoint.
6. `archived`: event remains durable but is in archival storage.
7. `redaction_marked`: access is restricted or transformed by policy while original evidence handling remains auditable.

Trace lifecycle:

1. `open`
2. `active`
3. `blocked`
4. `terminal_success`
5. `terminal_denied`
6. `terminal_failed`
7. `archived`

Incident lifecycle:

1. `opened`
2. `triaged`
3. `mitigating`
4. `monitoring`
5. `resolved`
6. `closed`
7. `reopened`

Prior records are never edited to move between states; new events and derived indexes reflect state changes.

## Policy And Security

- Append requires service-account authentication and source-service authorization.
- Event schemas must include privacy classification.
- Query APIs must enforce tenant, role, data-class, and evidence-purpose filtering.
- Private payloads should be stored as refs and hashes unless explicit evidence policy allows capture.
- Redaction must be represented as a new policy/evidence marker, not silent deletion.
- Integrity checks should cover ordering, payload hashes, prior event hashes, and checkpoint refs.
- Public reports must use aggregate or redacted evidence bundles.
- Central AI may consume evidence refs and summaries according to policy; it must not receive raw private data by default.

## Metering And Accounting

Overwatch does not settle usage. It should:

- Store raw usage, queue, execution, policy, dispute, and ledger evidence refs for later accounting services.
- Provide event refs to Overmeter, ORU, Seal Ledger, Overbill, Overgrant, and provider payout services.
- Emit storage and query usage dimensions for its own resource consumption.
- Preserve evidence for low-friction ORU and Seal Ledger settlement without blockchain or NFT assumptions.
- Keep accounting and native-service stewardship reports evidence-backed without embedding charge logic.

## Observability And Operations

Overwatch should expose:

- Event append rate, rejection rate, query latency, trace completion rate, and checkpoint coverage.
- Per-service event lag and schema error counts.
- Storage growth by event family, tenant, and privacy class.
- Incident timeline views and current mitigation state.
- Health event dashboards for control-plane and grid-resident services.
- Retention and archive reports.
- Backup, restore, corruption detection, and replay drills.
- Export job status for evidence bundles and public reports.

## Failure Modes And Recovery

- Invalid event schema: reject with reason code and do not append.
- Unknown source service: reject unless explicitly registered.
- Duplicate event id: return prior append result if identical, reject if conflicting.
- Sequence gap: accept with gap marker only if protocol allows; otherwise block source stream.
- Overwatch unavailable: source service must fail closed for critical mutating actions or use a documented local append buffer.
- Index corruption: rebuild derived indexes from append-only records.
- Storage corruption: restore from backup and validate checkpoints.
- Accidental private data append: create redaction marker, restrict query, preserve incident evidence, and fix emitting service.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Every mutating command can be traced from request to final state.
- Policy and dispute decisions cite stored evidence.
- Event records are append-only and tamper-evident enough for operational audit.

Additional SDS-level validation:

- Contract tests cover event append, audit append, trace query, event query, evidence bundle, incident, and health APIs.
- Append-only tests prove prior event records cannot be overwritten or deleted through normal APIs.
- Trace tests reconstruct Overgate to Overregistry to Overqueue flow from event refs.
- Privacy tests prove tenant and data-class filters hide restricted payload refs.
- Integrity tests verify hashes, sequence checks, duplicate handling, and checkpoint coverage.
- Recovery tests rebuild query indexes from append-only records.
- Evidence bundle tests prove central AI, dispute, compliance, and public-report bundles use the correct redaction policy.

## Build Breakdown

1. Define event, audit, trace, health, incident, evidence bundle, retention, and export schemas.
2. Implement append-only event and audit storage for Phase 1 services.
3. Add trace reconstruction from Overgate, Overregistry, and Overqueue events.
4. Add tenant-filtered event and trace query APIs.
5. Add integrity metadata and checkpoint reporting.
6. Add policy, dispute, execution, and usage evidence retention in Phase 4 and later phases.
7. Add health, incident, backup, restore, and grid-resident evidence in Phase 7.
8. Add compliance and public reporting exports in [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md).

The Phase 1 exit gate is an auditable chain from signed API request to manifest and queue item.

## Handoff And Downstream Use

Overwatch is evidence infrastructure for Overclaim, Oververify, Overguard, Overmeter, governance, central AI, compliance, admin UI, and public reporting.

Downstream services should append events and cite event refs rather than copying evidence into private logs. If a service needs a new event family or evidence bundle, update this SDS, the service implementation plan, shared schemas, and the build-plan crosswalk together.

## Open Design Questions

- Phase 1 integrity should use a simple Overwatch-owned checkpoint scheme: every source service writes a monotonic per-source sequence, every event carries a BLAKE3 payload hash and prior-event hash, and Overwatch periodically writes a signed checkpoint record covering the source stream head, global checkpoint range, schema version, and service identity. This is sufficient for the Phase 1 signed command path because it proves ordering, duplicate handling, payload integrity, and trace continuity without requiring mature Overbase, Overstore, backup, consensus, or grid-resident replication. Merkle range proofs, cross-node checkpoints, restore attestations, and replicated checkpoint authorities belong to Phase 7 and later hardening.
- Retention is classed by event family and privacy class, not by a single global rule. Protocol, identity, tenant, key, registry, queue, checkpoint, security, incident, dispute, accounting, grant, payout, and governance evidence must retain permanent refs and integrity metadata, with raw private payloads kept out of Overwatch by default. High-volume health, readiness, metric, trace-span, and debug families may roll into signed summaries after their operational window while preserving enough refs to prove service behavior. Accidental private-data appends are handled by redaction markers, access restriction, and incident evidence; prior event existence and hashes remain auditable.
- Append buffering is allowed only as a bounded Rust-owned service-local durable buffer behind Overrid abstractions, never as Redis, Kafka, or another external queue boundary. Critical tenant, credential, manifest, queue, policy, and accounting-adjacent mutations must fail closed unless the source service can durably record the domain mutation and matching append-buffer entry atomically; low-risk health or telemetry events may degrade to signed loss counters or summaries. Buffered events keep the original trace id, command id, source sequence, payload hash, privacy class, and service signature; replay is idempotent, sequence gaps block that source stream, and operators get an Overwatch incident or health event when buffering starts, drains, or exceeds its limit.
- Evidence bundles use canonical JSON plus JSON Schema as the authoritative portable format, with optional deterministic NDJSON event streams and human-readable renderings generated from the same bundle manifest. Dispute bundles include actor, tenant, subject, timeline, command, event, reason-code, policy, state-transition, and signature refs with redaction metadata. Central AI review bundles include purpose-scoped summaries, evidence refs, provenance, confidence limits, and private-data exclusions. Compliance bundles include retention class, export scope, legal hold or deletion refs, integrity hashes, and audit refs. Public-report bundles expose only aggregates, redacted timelines, and stewardship-safe summaries.
- User-visible trace summaries should expose only the fields needed to understand the caller's own request: trace id, request or command id, tenant/app context visible to the caller, submitted time, last updated time, current lifecycle state, terminal reason code, user-facing explanation, relevant manifest or queue refs, service-hop names, retry/idempotency outcome, and next allowed action. They must hide raw payloads, private payload refs, service-account secrets, other tenant members, internal operator notes, private policy inputs, node/provider details unless explicitly authorized, and cross-tenant correlation data. Admin and operator views may add deeper refs only through tenant, role, data-class, and evidence-purpose checks.
