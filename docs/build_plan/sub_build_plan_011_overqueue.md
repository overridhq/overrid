# SUB BUILD PLAN #11 - Overqueue

Attached SDS: [SDS #11 - Overqueue](../sds/control_plane/overqueue.md)

## Purpose

This sub-build plan turns SDS #11 into an implementation sequence for Overqueue.

Overqueue is the durable queue state and scheduler handoff layer for accepted Overrid commands and workloads. It stores accepted work after Overgate admission, keeps queue state tenant-scoped and auditable, exposes scheduler-safe fetch and acknowledgement paths, and preserves retry, cancellation, deadline, timeout, backpressure, and dead-letter evidence. It must not become the scheduler, lease authority, runner, policy engine, metering authority, or ledger.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #11: Overqueue](../sds/control_plane/overqueue.md) | Controls purpose, non-goals, actors, dependencies, ownership, queue data model, APIs, events, workflows, state machine, security, metering handoff, validation, build breakdown, and resolved open questions. |
| [Overqueue implementation plan](../service_catalog/control_plane/overqueue.md) | Controls service-catalog objective, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Keeps Overqueue skeleton work in Phase 1 and execution integration in Phase 3. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies accepted command intake, durable pending state, Overwatch evidence, queue schemas, and minimal synthetic workload flow. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies scheduler fetch, claim, acknowledgement, retry, cancellation, timeout, result, and dead-letter integration. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #11 discoverable in the numbered service set. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0 and 1 | Freeze queue ownership, schemas, lane/state model, idempotency behavior, and evidence requirements before accepted commands can be durable. |
| 2 | Master Phase 1 | Implement enqueue, replay/conflict, pending/ready state, cancellation request state, and caller-visible status for synthetic workload flow. |
| 3 | Master Phase 1 | Implement queue health, backpressure hints, retention, operator queries, and Overwatch evidence. |
| 4 | Master Phase 3 | Add scheduler fetch, claim, heartbeat, ack, release, retry, timeout, and dead-letter behavior. |
| 5 | Master Phases 3, 4, and 5 | Add policy, verification, usage, accounting, and dispute refs without moving authority into Overqueue. |
| 6 | Master Phases 7, 11, 12, and 13 | Harden queue behavior for grid-resident backbone, public-provider constraints, native-app clients, compliance, incident, and scale. |

## Tech Stack Guardrails

- Overqueue is native Overrid durable queue state. It should start inside the modular Rust control-plane boundary until service split criteria are proven.
- Queue records, APIs, events, and tests use shared schemas, canonical JSON/JSON Schema, stable errors, trace ids, idempotency keys, and Overwatch-compatible evidence refs.
- Queue storage must be behind Overrid-owned abstractions. Do not describe Overqueue as a conventional Kafka, RabbitMQ, SQS, NATS, Redis queue, or cloud queue product.
- Overqueue may hold payload refs, manifest refs, hashes, command classes, reason codes, and evidence refs. It must avoid raw private payloads, secrets, vault material, decrypted RAG context, private messages, raw media, payment secrets, or sensitive app data.
- Overqueue must preserve at-least-once delivery safety through idempotent workload ids, request hashes, claim tokens, acknowledgements, and stable duplicate/conflict behavior.

## Phase 1: Charter, Schemas, Lanes, And State Model

### Work Items

- **1.1 Freeze Overqueue ownership boundaries.**
  - Design: Document that Overqueue owns queue item state, lane state, claims, retries, cancellations, backpressure records, and dead-letter records.
  - Output: Boundary matrix for Overgate, Overguard, Overregistry, Overwatch, Oversched, Overlease, Overrun, Overmeter, ORU, Seal Ledger, SDK, CLI, Admin UI, native apps, and operators.
  - Validation: Review confirms no work item gives Overqueue placement, lease, execution, policy, usage, or accounting authority.

- **1.2 Define queue schemas and stable errors.**
  - Design: Model `queue_item`, `queue_lane`, `scheduler_claim`, `queue_transition`, `retry_record`, `cancellation_record`, `backpressure_record`, and `dead_letter_record`.
  - Output: Shared schema additions, stable errors, positive fixtures, negative fixtures, and state-transition fixtures.
  - Validation: Schema tests reject missing tenant refs, workload refs, trace ids, idempotency keys, command classes, deadline refs, and audit refs.

- **1.3 Define queue state machine.**
  - Design: Preserve SDS states from accepted through pending, ready, claimed, running handoff, retry wait, cancelled, terminal success, terminal failure, and dead-letter.
  - Output: State reducer, transition guard table, reason-code table, and replay fixtures.
  - Validation: Invalid transitions fail with stable reason codes and do not delete prior evidence.

## Phase 2: Phase 1 Durable Pending Queue

### Work Items

- **2.1 Implement admitted enqueue path.**
  - Design: Accept enqueue only from Overgate-admitted commands or approved internal service accounts.
  - Output: Enqueue API, idempotency/replay handling, duplicate-conflict handling, and Overwatch append refs.
  - Validation: Synthetic workload reaches durable `pending` or `ready` state in Phase 1.

- **2.2 Implement caller-visible status.**
  - Design: Return queue state, stable reason codes, retry-after hints, cancellation eligibility, trace refs, and redacted evidence refs.
  - Output: Status API and SDK/CLI/admin-compatible response fixtures.
  - Validation: Caller-visible reads never expose private payloads, unrelated tenant state, fraud internals, or operator-only details.

- **2.3 Implement cancellation request path.**
  - Design: Support cancellation request records before scheduler claim, with later Phase 3 propagation to lease/runner state.
  - Output: Cancellation API, cancellation state, reason refs, and audit refs.
  - Validation: Cancellation is idempotent and cannot remove queue history.

## Phase 3: Health, Backpressure, Retention, And Evidence

### Work Items

- **3.1 Implement queue health and backpressure records.**
  - Design: Track lane depth, oldest-ready age, retry pressure, dead-letter pressure, tenant pressure, and hard/soft admission hints.
  - Output: Queue health API, backpressure events, Overgate admission hints, and operator views.
  - Validation: Hard pressure denies new queue-producing commands before enqueue while preserving existing durable items.

- **3.2 Implement retention and archival policy.**
  - Design: Keep nonterminal items hot-queryable, terminal records hot for the configured window, and unresolved dead-letter/dispute/incident/accounting-hold records queryable until closure.
  - Output: Retention evaluator, archival markers, Overwatch evidence refs, summary indexes, and replay records.
  - Validation: Retention tests prove hot/warm/archive behavior preserves replay and does not leak private payloads.

- **3.3 Implement operator investigation views.**
  - Design: Provide stuck, delayed, retrying, cancelled, and dead-letter views with redaction by role.
  - Output: Operator query APIs, redacted detail views, support refs, and diagnostics.
  - Validation: Tenant/operator redaction tests pass for caller, support, operator, and steward roles.

## Phase 4: Phase 3 Scheduler Integration

### Work Items

- **4.1 Implement scheduler fetch and claim.**
  - Design: Let Oversched fetch ready items by lane, tenant, workload class, deadline, and policy-compatible filters, then issue short scheduler claims.
  - Output: Fetch API, claim token, claim expiry, heartbeat, release, and conflict handling.
  - Validation: Expired claims release safely and duplicate claims cannot cause duplicate execution without idempotency evidence.

- **4.2 Implement acknowledgement and result handoff.**
  - Design: Accept scheduler/runner lifecycle refs for lease requested, lease denied, execution started, result accepted, retry requested, cancellation propagated, timeout, and terminal states.
  - Output: Ack API, result handoff refs, terminal transition records, and replay bundles.
  - Validation: Ack tests prove terminal state is idempotent and previous queue evidence remains queryable.

- **4.3 Implement retry, timeout, and dead-letter behavior.**
  - Design: Apply retry policy, deadlines, max attempts, terminal failure rules, dead-letter thresholds, and operator remediation hooks.
  - Output: Retry scheduler, timeout sweeper, dead-letter records, remediation refs, and events.
  - Validation: Retryable failures return to queue with reason codes; nonretryable failures create final evidence.

## Phase 5: Policy, Usage, Accounting, And Dispute Handoffs

### Work Items

- **5.1 Add policy and verification refs.**
  - Design: Preserve Overguard, Workload Classifier, Oververify, Challenge Task, and reputation refs as queue facts, not queue-owned decisions.
  - Output: Policy ref fields, denial/blocked states, and recheck triggers.
  - Validation: Queue items cannot execute when policy refs are denied, stale, or missing where required.

- **5.2 Add usage and accounting handoff refs.**
  - Design: Emit queue operation usage refs and preserve downstream Overmeter/ORU/Seal Ledger refs without mutating accounting truth.
  - Output: Usage refs for enqueue, fetch, claim, retry, cancellation, dead-letter, status reads, and replay.
  - Validation: Accounting tests prove Overqueue never reserves funds, mutates balances, or appends ledger entries.

- **5.3 Add dispute and incident handoff refs.**
  - Design: Let Overclaim and Incident Response cite queue evidence for failed, duplicated, delayed, or disputed work.
  - Output: Claim refs, incident refs, hold refs, and replay bundles.
  - Validation: Dispute/incident reads are redacted and preserve queue evidence.

## Phase 6: Grid, Public, Native-App, And Governance Hardening

### Work Items

- **6.1 Harden grid-resident operation.**
  - Design: Support backup/restore, failover, single-writer fencing, queue drains, route freezes, and protected system-service workload behavior.
  - Output: Grid readiness checklist, backup refs, failover refs, restore drill refs, and drain controls.
  - Validation: Restore drills preserve queue ordering, claim state, replay evidence, and idempotency behavior.

- **6.2 Harden public-provider and native-app traffic.**
  - Design: Apply public sandbox, fraud, anti-Sybil, payout-hold, native-app side-effect, and mobile/offline replay constraints.
  - Output: Public/native lane policies, rate limits, abuse handoffs, and redacted app-facing status.
  - Validation: Public and native-app tests prove high-risk commands cannot bypass queue policy or owner-service checks.

- **6.3 Harden compliance, incident, and scale operation.**
  - Design: Attach threat/security review, retention, incident drills, PIP/governance refs, and scale limits.
  - Output: Security review entries, incident runbooks, scale tests, migration notes, and public-safe reporting aggregates.
  - Validation: Queue scale tests and incident drills pass without losing or duplicating accepted work.

## Exit Gates

- Accepted commands can be durably queued with idempotency, trace, tenant, policy, and audit refs.
- Scheduler fetch/claim/ack/release/retry/dead-letter behavior works against deterministic fixtures.
- Queue health and backpressure feed Overgate admission hints without dropping durable items.
- Queue status and operator views are redacted by role.
- Overqueue emits usage/audit refs but never mutates policy, placement, lease, execution, usage, accounting, or dispute truth directly.

## Downstream Handoff

Overqueue is ready when Overgate can safely hand accepted work to durable queue state, Oversched can consume that work through claims and acknowledgements, Overwatch can reconstruct the queue history, and later execution, metering, billing, dispute, native app, public-provider, and governance services can cite queue refs without moving their authority into Overqueue.
