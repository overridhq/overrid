SDS #11

# Overqueue SDS

## Purpose

Build durable workload queueing with priority lanes, retry orchestration, deadlines, cancellation, backpressure, and dead-letter handling.

Overqueue is the control-plane persistence layer for accepted work that cannot or should not execute synchronously. It receives commands admitted by Overgate, stores queue items as durable state, exposes scheduler-safe fetch and acknowledgement paths, and preserves retry, cancellation, timeout, and dead-letter evidence for audit and recovery.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overqueue.md](../../service_catalog/control_plane/overqueue.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md), [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md) |

## Service Family

- Family: Control plane.
- Owning layer: Durable command and workload queue state.
- Primary data scope: queue items, lane definitions, scheduler claims, retry records, cancellation records, dead-letter records, backpressure records, and audit refs.
- First build phase from service plan: skeleton in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); execution integration in [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md).

## Problem Statement

Overrid needs a durable handoff between API admission and execution. Without Overqueue, accepted workload commands can vanish during failures, retry without evidence, execute twice without detection, or overload scheduler and worker services. Overqueue must make pending work explicit, queryable, bounded, and auditable before the system runs real workloads.

## Goals

- Persist accepted workload commands as queue items with stable ids and traceability.
- Support priority lanes, deadlines, retry metadata, cancellation, backpressure, and dead-letter handling.
- Provide at-least-once delivery to scheduler consumers with idempotent workload ids and acknowledgement rules.
- Preserve every queue state transition as Overwatch-compatible evidence.
- Support Phase 1 synthetic workload flow into `pending` state.
- Support Phase 3 scheduler, lease, runner, result, retry, timeout, cancellation, and dead-letter flows.
- Keep queue state tenant-scoped, policy-aware, and safe to inspect through operator tools.

## Non-Goals

- Do not choose placement candidates; Oversched owns placement.
- Do not grant resource reservations; Overlease owns resource leases.
- Do not execute workloads; Overrun and node agents own execution.
- Do not store full private payloads when manifest refs, hashes, and evidence refs are enough.
- Do not make queue priority a hidden override of policy, tenant suspension, or quota state.
- Do not encode pricing, customer-count, or market assumptions.

## Primary Actors And Clients

- Overgate forwarding accepted workload commands.
- Overguard providing policy admission refs.
- Oversched consuming ready queue items.
- Overlease and Overrun reporting execution lifecycle refs in later phases.
- SDK, CLI, admin UI, and native apps reading caller-visible status through approved APIs.
- Operators investigating stuck, retrying, or dead-lettered work.
- Overwatch storing queue evidence.

## Dependencies

- Overgate accepted commands and trace ids.
- Overregistry workload, resource, package, provider, and capability refs.
- Overtenant tenant state, quota scope, and suspension state.
- Overpass actor, service account, node, app, and native service refs.
- Overkey service-account credentials for internal scheduler consumers.
- Overguard admission decisions and policy refs once policy is active.
- Shared queue item schema and API error schema.
- Overwatch append-only event log.
- Oversched in Phase 3 for fetch, claim, and acknowledgement integration.

## Owned Responsibilities

Overqueue owns:

- Queue item creation from admitted commands.
- Queue item state transitions and transition guards.
- Lane, priority, deadline, and backpressure metadata.
- Scheduler claim and acknowledgement records.
- Retry scheduling and retry budget state.
- Cancellation intent and cancellation acknowledgement state.
- Dead-letter records with reason codes and evidence refs.
- Query surfaces for caller-visible and operator-visible queue status.
- Queue health metrics and events.

Overqueue must not mutate manifests, policy decisions, leases, execution results, or accounting records directly.

## Data Model

The first implementation should define these records:

- `queue_item`: queue id, workload id, tenant id, actor id, command id, trace id, idempotency key, workload manifest ref, resource card ref, lane id, priority, deadline, state, attempt count, retry policy ref, cancellation ref, dead-letter ref, and audit refs.
- `queue_lane`: lane id, workload class, tenant scope, priority class, max depth, concurrency hint, backpressure state, and policy refs.
- `scheduler_claim`: claim id, queue item id, scheduler service account id, claim token hash, claim time, expires time, heartbeat time, acknowledgement state, and audit refs.
- `retry_record`: queue item id, attempt number, retryable reason code, next eligible time, max attempts, backoff class, prior failure refs, and policy refs.
- `cancellation_record`: queue item id, requested by, reason code, requested time, effective state, scheduler acknowledgement ref, and audit refs.
- `dead_letter_record`: queue item id, terminal reason code, final attempt, failed dependency, evidence refs, operator action refs, and requeue eligibility.
- `backpressure_record`: lane id, depth, oldest ready time, blocked reason, dependency state, shed or deny reason, and event refs.

Queue item payloads should use manifest refs and hashes. Private input data belongs in the storage or vault services, not in queue rows.

## API Surface

Phase 1 should expose a small internal API plus caller-visible status:

- `POST /v1/internal/queue-items`: enqueue an Overgate-admitted command. Requires service-account auth and an admission ref.
- `GET /v1/queue-items/{queue_item_id}`: read caller-visible queue state with tenant filtering.
- `GET /v1/queue-items:by-trace`: resolve queue state by trace id or command id.
- `POST /v1/internal/scheduler/fetch`: scheduler fetches ready items by lane, workload class, and capacity hint.
- `POST /v1/internal/scheduler/{claim_id}/ack`: acknowledge scheduler acceptance, retry, cancellation, or release.
- `POST /v1/internal/queue-items/{queue_item_id}/retry`: schedule retry from an execution or scheduler failure ref.
- `POST /v1/queue-items/{queue_item_id}/cancel`: request cancellation through an admitted command.
- `GET /v1/admin/queue/lanes`: inspect lane depth, oldest ready time, retry counts, and dead-letter counts.
- `GET /v1/admin/queue/dead-letter`: inspect dead-lettered items with tenant and role filtering.

External callers should normally submit work through Overgate and read status through SDK or CLI helpers; they should not write queue state directly.

## Event Surface

Overqueue should emit:

- `overqueue.item_created`: durable queue item created from an admitted command.
- `overqueue.item_ready`: item eligible for scheduler fetch.
- `overqueue.item_claimed`: scheduler claim opened.
- `overqueue.item_dispatched`: scheduler accepted the item for placement.
- `overqueue.item_retry_scheduled`: retryable failure returned item to a future ready time.
- `overqueue.item_cancel_requested`: authorized cancellation requested.
- `overqueue.item_cancelled`: cancellation reached a terminal queue state.
- `overqueue.item_dead_lettered`: item moved to dead-letter state.
- `overqueue.backpressure_changed`: lane pressure changed enough to affect admission or fetch.
- `overqueue.claim_expired`: scheduler claim expired and item returned to ready or retry state.

Events must include queue item id, tenant id, trace id, workload ref, reason code, prior state, next state, and audit refs. Private payloads should be referenced, not embedded.

## Core Workflow

1. Overgate accepts a signed workload command and forwards a queue request with trace id, tenant id, actor id, command id, idempotency key, manifest refs, and audit refs.
2. Overqueue validates the request against shared schemas and verifies that the admission ref is usable.
3. Overqueue creates or replays the queue item based on workload id and idempotency context.
4. The item enters the correct lane with priority, deadline, retry policy, and backpressure metadata.
5. In Phase 1 the item only needs to reach durable `pending` or `ready` state.
6. In Phase 3 Oversched fetches ready items, opens scheduler claims, and acknowledges dispatch, retry, or release.
7. Failures return with reason codes and evidence refs; Overqueue either schedules retry, cancels, or dead-letters the item.
8. Operators and callers inspect state through filtered query APIs.

## State Machine

Queue item lifecycle:

1. `accepted_for_queue`: Overgate admission is valid and enqueue has started.
2. `pending`: item is durable but waiting for prerequisites, lane capacity, or future eligibility.
3. `ready`: item can be fetched by an authorized scheduler.
4. `claimed`: scheduler has a time-limited claim token.
5. `dispatched`: scheduler accepted responsibility and is moving the item toward placement or lease.
6. `retry_wait`: retryable failure returned the item to a future ready time.
7. `cancellation_requested`: authorized actor requested cancellation while work may still be in another subsystem.
8. `cancelled`: queue item reached terminal cancellation before or during dispatch.
9. `dead_lettered`: item cannot proceed without operator action, manifest correction, or policy change.
10. `completed`: downstream execution produced a terminal success ref.
11. `failed_final`: downstream execution or policy produced terminal failure without requeue.

Allowed transitions must be explicit. Claim expiry may return `claimed` to `ready` or `retry_wait`, but only with an event and reason code.

## Policy And Security

- Only Overgate or approved internal service accounts can enqueue work.
- Scheduler fetch and acknowledgement APIs require service-account credentials and lane permissions.
- Tenant suspension, quota holds, policy denial, or manifest revocation must prevent new ready work and may freeze existing work.
- Queue status reads must filter by tenant, actor, role, workload class, and data sensitivity.
- Cancellation must be an admitted command with actor, tenant, trace id, and reason code.
- Dead-letter requeue requires an explicit signed operator or service command and must preserve prior evidence.
- Backpressure should deny or defer safely; it must not silently drop accepted work.
- Queue records must not contain raw secrets or private payload bodies.

## Metering And Accounting

Overqueue does not settle usage. It should:

- Emit queue wait, retry count, lane pressure, cancellation, and dead-letter facts for Overmeter.
- Preserve tenant, actor, app, workload, and service account refs for usage attribution.
- Provide raw queue timing dimensions that Phase 5 can roll into ORU and Seal Ledger evidence.
- Avoid payment or ledger mutation in queue code.
- Keep native-service and mobile-service queue usage visible through receipt refs returned by downstream accounting services.

## Observability And Operations

Overqueue should expose:

- Lane depth, oldest ready item age, pending count, claimed count, retry count, cancellation count, and dead-letter count.
- Scheduler claim expiry and acknowledgement latency.
- Backpressure state and admission impact by lane.
- Retry distribution by reason code.
- Tenant-scoped stuck-work views.
- Dead-letter inspection and controlled requeue tooling.
- Backup, restore, replay, and migration checks for queue records.
- Readiness checks for storage, schema package, Overwatch, Overregistry, and scheduler consumers.

## Failure Modes And Recovery

- Duplicate enqueue with same idempotency context: return prior queue item.
- Duplicate enqueue with conflicting payload hash: deny with conflict reason.
- Overwatch unavailable: fail closed for new enqueue unless an audited local buffer mode is explicitly enabled.
- Scheduler fetch succeeds but ack is lost: claim expires and item returns to `ready` or `retry_wait`.
- Scheduler or node fails after dispatch: downstream failure ref drives retry or dead-letter.
- Tenant suspended while item is pending: item moves to blocked pending state until unsuspended or cancelled.
- Manifest revoked while queued: item moves to dead-letter or failed final with registry evidence.
- Backpressure threshold reached: admission can defer or deny new work according to policy, but existing durable items remain queryable.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Synthetic workload reaches pending state in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md).
- Retryable failures return to queue with reason codes.
- Dead-letter state preserves evidence and is queryable.

Additional SDS-level validation:

- Contract tests cover enqueue, status, fetch, ack, retry, cancellation, dead-letter, and lane health APIs.
- Idempotency tests cover duplicate enqueue replay and conflicting request hashes.
- Tenant isolation tests prove queue status and admin views filter correctly.
- Claim tests cover expiry, double-ack, stale scheduler, and claim-token mismatch.
- Backpressure tests prove new admission is denied or deferred predictably while durable items remain intact.
- Replay tests reconstruct the queue item history from Overwatch events.

## Build Breakdown

1. Define queue item, lane, scheduler claim, retry, cancellation, dead-letter, and backpressure schemas.
2. Implement enqueue from Overgate accepted commands and persist synthetic workload items.
3. Add caller-visible status by queue item id, command id, and trace id.
4. Emit Overwatch events for create, ready, state change, and dead-letter transitions.
5. Add scheduler fetch and acknowledgement APIs for Phase 3.
6. Add retry, cancellation, claim expiry, and backpressure controls.
7. Add admin diagnostics for lanes, stuck items, retry loops, and dead letters.

The Phase 1 exit gate is durable pending state with audit evidence; Phase 3 expands the same records into real execution handoff.

## Handoff And Downstream Use

Overqueue hands accepted workloads to Oversched and preserves queue state for audit, replay, recovery, and later accounting.

Downstream services should consume queue records through scheduler APIs and event refs, not direct table reads. If a new execution path needs queue behavior, update this SDS, the service implementation plan, shared schemas, and the build-plan crosswalk together.

## Open Design Questions

Resolved decisions:

- Phase 1 should implement two concrete writable lanes: `tenant_workload_default` for Overgate-admitted synthetic workload commands and later Phase 3 private jobs, and `control_plane_internal` for approved service-account maintenance, replay, and recovery work needed to prove audit-safe queue behavior. `system_service`, `native_app_task`, and `background_low_priority` may exist as reserved lane or workload-class enum values, but they are not active scheduling lanes in Phase 1; grid-resident system-service work belongs to Phase 7, native-app task lanes belong to Phase 12, and low-priority background work starts as a priority/backoff class inside an existing lane rather than a separate queue product.
- Phase 1 does not need live scheduler claims because synthetic work only needs to reach durable `pending` or `ready` state. Once Phase 3 scheduler fetch is enabled, the initial claim timeout should be short and scheduling-only: 60 seconds by default, with a 15-second heartbeat expectation and at most three consecutive renewals before the item is released or moved to `retry_wait` with evidence. This claim covers Oversched evaluation and lease-request handoff only; real execution duration belongs to Overlease and Overrun timeout telemetry, not to an Overqueue claim token.
- Before Overmeter, ORU, and Seal Ledger prechecks are online, Overqueue backpressure should feed conservative Overgate admission hints from native lane depth, oldest-ready age, retry pressure, dead-letter pressure, tenant state, and configured Overtenant quota placeholders. Soft pressure may let Overgate accept the command and place it into an explicit blocked or delayed pending state with a `retry_after` hint; hard pressure should deny new queue-producing commands before enqueue with stable reason codes such as `queue_backpressure_hard_limit`, while preserving every already durable item. Overqueue must not mutate balances, reserve funds, or pretend to settle usage during this phase; it only emits queue pressure refs for later Overmeter and ORU processing.
- Dead-letter reasons are user-correctable only when the submitting actor can fix the condition through a new signed Overgate command without private state repair: missing artifact refs, invalid or superseded workload manifest refs, unsupported resource-card values, expired caller credentials, safe package-validation failures, insufficient placeholder quota/grant refs, or user-requested cancellation/retry choices. Reasons are operator-only when they involve queue invariant violations, idempotency conflicts, Overwatch evidence gaps, storage corruption, tenant suspension, suspected abuse or compromise, protected service-account/system-service state, policy override, lease/scheduler consistency bugs, private evidence, secret refs, accounting/rights state, or compliance/legal holds. Requeue always creates new signed evidence and never edits the prior dead-letter record in place.
- Queue history remains hot-queryable while an item is nonterminal and for 30 days after terminal success, cancellation, final failure, or resolved dead-letter. Unresolved dead-letter, active dispute, incident, retry investigation, accounting hold, tenant offboarding, or legal/compliance hold keeps the item in the hot or warm query set until the owning workflow closes. After the hot window, Overqueue may archive detailed transition rows into Overwatch/backup-backed evidence storage while retaining a 90-day operator summary index by tenant, trace id, command id, queue item id, workload id, terminal reason, and evidence refs; public or caller-facing reads after archival should resolve through redacted trace/evidence summaries rather than raw queue rows.
