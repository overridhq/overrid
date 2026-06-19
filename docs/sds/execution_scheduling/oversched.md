SDS #26

# Oversched SDS

## Purpose

Choose auditable workload placements from queue, node, capability, policy, trust, grant, cache, locality, and lease facts, then request reservations from Overlease.

Oversched is the placement decision authority for the execution loop. It does not execute workloads, create resource facts, mint trust scores, or reserve resources by itself. It turns accepted queued work into explainable placement decisions and lease requests.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [oversched.md](../../service_catalog/execution_scheduling/oversched.md) |
| Sub-build plan | [SUB BUILD PLAN #26 - Oversched](../../build_plan/sub_build_plan_026_oversched.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md) |

## Service Family

- Family: Execution and scheduling
- Owning layer: Policy-aware placement, candidate filtering, placement explanation, and lease-request handoff
- Primary data scope: scheduling requests, queue item refs, workload resource cards, candidate snapshots, filter results, scoring inputs, placement decisions, lease requests, gang lease plans, replay bundles, and scheduling policy versions
- First build phase from service plan: [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md)

## Problem Statement

The execution loop needs a deterministic component that can decide where accepted work should run without bypassing policy, lease, trust, or audit rails. A node can be available and still be ineligible because of tenant visibility, workload class, data class, package trust, location, cache scope, grant constraints, resource pressure, or stale capability facts.

Oversched must make those decisions explainable. Operators, users, Overwatch, Overclaim, and later central AI review should be able to reconstruct why a node was selected, why alternatives were rejected, and which facts were used.

## Goals

- Fetch eligible queued work and required facts through explicit service contracts.
- Filter candidate nodes by tenant visibility, node state, resource class, capability records, workload class, data class, trust class, locality, cache hints, grant constraints, and policy decisions.
- Produce stable reason codes for both selected and rejected candidates.
- Create replayable placement decision records with input fact versions and scheduler policy version.
- Request short-lived reservations from Overlease and bind successful leases to queue assignments.
- Support single-node jobs first, then atomic multi-node lease plans after the v0 path is stable.
- Preserve founder-hardware bootstrap assumptions while keeping the contract compatible with grid-resident scheduling.

## Non-Goals

- Do not execute workloads. Overrun owns node-side execution.
- Do not reserve resources directly. Overlease owns lease records and lease proofs.
- Do not mutate node inventory, benchmark, trust, grant, or cache facts.
- Do not generate trust scores or policy decisions; consume Overguard, Oververify, Overgrant, and Overmark outputs.
- Do not bill users, mutate ORU balances, or perform settlement.
- Do not optimize for opaque global efficiency at the cost of replayability and reason codes.
- Do not use missing dependency facts as permission to schedule; ambiguous state is a deny or blocked state.

## Primary Actors And Clients

- Overqueue, supplying accepted workload commands.
- Overgate, CLI, SDK, admin UI, and product adapters, reading placement status and explanations.
- Overregistry, Hardware Discovery, Benchmark Runner, and Overcell, supplying node and capability facts.
- Overguard, Oververify, Overgrant, Overmark, and Overcache, supplying policy, trust, grant, cost-class, and cache hints.
- Overlease, receiving reservation requests and returning lease proofs or denial reasons.
- Overrun, receiving leased assignments after scheduling succeeds.
- Overwatch and Overclaim, consuming decision evidence for audits, disputes, and incident review.

## Dependencies

- [Overqueue](../control_plane/overqueue.md) for queue item claims, retry lanes, cancellation, and dead-letter handoff.
- [Overregistry](../control_plane/overregistry.md) for registered node, provider, capability, package, and workload facts.
- [Hardware Discovery](hardware_discovery.md) and [Benchmark Runner](benchmark_runner.md) for observed inventory and measured capacity evidence.
- [Overcell](overcell.md) for node heartbeat, command acceptance, and live availability signals.
- [Overpack](overpack.md) for workload manifests, resource cards, runtime contracts, and permission refs.
- [Overguard](../trust_policy_verification/overguard.md) for workload, data, egress, tenant, and policy decisions.
- [Oververify](../trust_policy_verification/oververify.md) for provider and node verification signals when available.
- [Overgrant](../accounting/overgrant.md) and [Overmark](../accounting/overmark.md) for grant constraints and resource cost-class signals.
- [Overcache](overcache.md) for policy-scoped cache hints and locality value.
- [Overmesh](overmesh.md) for route locality and private connectivity constraints once Phase 4/8 routing matures.
- [Overlease](overlease.md) for reservation creation, renewal boundaries, and atomic lease sets.
- [Overwatch](../control_plane/overwatch.md) for decision events and replay evidence.

Phase 3 may use deterministic fixtures for trust, grant, cost-class, cache, and route facts, but the fixture records must preserve the final interface shape and be visible in the placement replay bundle.

## Owned Responsibilities

Oversched owns:

- Scheduler request intake from durable queue lanes.
- Candidate fact collection through dependency APIs.
- Deterministic filtering and reason-code production.
- Resource-fit calculation against workload resource cards and live capacity.
- Placement scoring rules that are versioned, explainable, and replayable.
- Single-node placement decisions and lease requests.
- Gang scheduling plans for multi-node work after v0.
- Scheduler backoff, blocked-state, no-candidate, and cancellation records.
- Scheduling replay bundles for audits, disputes, and tests.

Oversched must not read dependency private storage directly. Every input fact must have a source service, version, timestamp, and traceable ref.

## Data Model

The first implementation should define:

- `scheduling_request`: queue item id, workload id, manifest id/version, tenant id, actor id or service account, priority class, requested resource card, workload class, data class, locality hints, cache hints, deadline, retry index, and trace id.
- `scheduler_policy_version`: filter chain version, scoring rule version, tie-break rule, gang-scheduling mode, fairness window, and compatibility date.
- `candidate_snapshot`: candidate node id, provider id, tenant visibility, node state, resource availability, capability refs, benchmark refs, heartbeat age, trust refs, policy refs, cache refs, route refs, and collected-at timestamp.
- `filter_result`: candidate id, filter name, input refs, pass/fail/unknown result, reason code, severity, and remediation hint where safe.
- `candidate_score`: candidate id, scheduler policy version, resource fit, locality score, cache score, trust class, grant fit, cost-class signal, tie-break key, and score explanation.
- `placement_decision`: decision id, request id, selected candidate or no-candidate result, selected reason summary, rejected candidate reason refs, input fact versions, policy refs, lease request refs, and replay bundle ref.
- `lease_request`: placement decision id, node id or node set, resource reservation, lease window, renewal policy, workload refs, tenant refs, and atomicity mode.
- `gang_lease_plan`: node set, per-node resource reservation, all-or-nothing requirement, rollback rule, and partial failure reason refs.
- `scheduler_backoff`: request id, dependency reason, retry-after, max retry, blocked lane, and dead-letter threshold.
- `scheduling_replay_bundle`: immutable snapshot of the request, candidate snapshots, filter results, score results, scheduler policy version, lease response, and event refs.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

Oversched APIs are mostly internal but must remain explicit and testable:

- `POST /scheduler/requests/{queue_item_id}/claim`: claim a queue item for scheduling with an idempotency key.
- `POST /scheduler/requests/{request_id}/evaluate`: collect facts, filter candidates, and create a placement decision without creating a lease when dry-run is requested.
- `POST /scheduler/decisions/{decision_id}/reserve`: submit the selected candidate or node set to Overlease.
- `GET /scheduler/requests/{request_id}`: read scheduling state, selected candidate, no-candidate reason, lease refs, and redacted decision summary.
- `GET /scheduler/decisions/{decision_id}/explain`: read reason codes, filter chain, score explanation, input refs, and replay bundle refs.
- `POST /scheduler/requests/{request_id}/cancel`: stop scheduling when the queue item or tenant policy cancels the work.
- `POST /scheduler/replay/{decision_id}`: rebuild the decision from stored refs in test or audit mode.

API requirements:

- Mutating endpoints require service identity, tenant context, trace id, and idempotency key.
- Dry-run evaluation must never create leases or mutate queue state beyond an audit event.
- Explanation reads must redact private provider, tenant, and user data while preserving reason codes.
- Lease reservation calls must be idempotent and must include the placement decision id.
- Replays must use stored fact versions, not current live facts.

## Event Surface

- `oversched.request_claimed`: queue item accepted for scheduling.
- `oversched.candidates_collected`: candidate snapshot set captured.
- `oversched.candidate_rejected`: candidate rejected with stable reason code.
- `oversched.placement_selected`: selected candidate or node set chosen.
- `oversched.no_candidate`: no eligible candidate found with blocked or terminal reason.
- `oversched.lease_requested`: lease request sent to Overlease.
- `oversched.lease_bound`: lease proof bound to placement decision.
- `oversched.lease_denied`: Overlease rejected reservation.
- `oversched.request_blocked`: dependency, capacity, policy, or fairness state blocks scheduling.
- `oversched.request_cancelled`: scheduling stopped by authorized cancellation.

Events must include trace, request, queue, workload, policy, scheduler-policy-version, and decision refs. They must not expose private node details beyond authorized operator and audit views.

## Core Workflow

1. Overqueue exposes an accepted workload item in a scheduler lane.
2. Oversched claims the queue item and records a scheduling request.
3. Oversched loads workload manifest, resource card, tenant scope, policy refs, and scheduling policy version.
4. Oversched collects candidate node facts from registry, node heartbeats, benchmark evidence, trust refs, grant refs, cache hints, and route hints.
5. Oversched applies deterministic filters and records pass/fail/unknown reason codes for every candidate.
6. Oversched scores eligible candidates with versioned rules and deterministic tie-breaks.
7. Oversched writes a placement decision and replay bundle.
8. Oversched requests a lease from Overlease.
9. On lease success, Overqueue and Overrun receive leased assignment refs.
10. On no-candidate, lease denial, cancellation, or dependency failure, Oversched records reason codes and hands retry, block, or dead-letter refs back to Overqueue.

## State Machine

Scheduling request lifecycle:

1. `queued`: queue item is visible for scheduling.
2. `claimed`: Oversched has claimed the item idempotently.
3. `loading_facts`: manifest, tenant, policy, node, trust, grant, cache, and route refs are being collected.
4. `filtering`: candidate filters are running.
5. `scoring`: eligible candidates are being ranked.
6. `selected`: a placement decision has selected a candidate or node set.
7. `reserving`: lease request is in flight.
8. `leased`: Overlease returned a valid lease proof.
9. `no_candidate`: no eligible node exists with recorded reason codes.
10. `blocked`: scheduling waits for dependency, capacity, fairness, or policy state.
11. `lease_denied`: reservation failed after selection.
12. `cancelled`: queue or policy cancelled scheduling.
13. `failed`: unrecoverable scheduler error with evidence refs.
14. `expired`: scheduling deadline passed before lease binding.

Decision and replay history are append-only. A later correction creates a new placement decision version rather than mutating the old decision.

## Policy And Security

- Tenant visibility is the first candidate filter.
- Workload class, data class, secret-bearing status, egress requirements, package trust, and provider trust must be checked before scoring.
- Missing or stale facts produce `unknown` or `blocked` reason codes, not implicit permission.
- Scheduler policy versions must be stored with decisions.
- Operator overrides require signed action, Overwatch evidence, and a new decision version.
- Public, private, regulated, secret-bearing, public-interest, and system-service workloads must not share candidate pools unless policy explicitly allows it.
- Provider-private capacity data must be redacted from user-facing explanations.
- Gang scheduling must be all-or-nothing when the workload requires atomic node sets.

## Metering And Accounting

Oversched does not bill or settle, but it emits scheduler usage facts:

- Candidate count, filter count, score count, scheduling attempts, blocked duration, no-candidate count, lease request count, and replay count.
- Link scheduler usage to tenant, workload, queue item, app/service account, scheduler policy version, and final scheduling state.
- Preserve scheduling effort for diagnostics and capacity planning without charging per tiny internal operation.
- Carry grant and cost-class refs into placement decisions for later Overmark and accounting review.

## Observability And Operations

- Dashboards should show queue depth by scheduler lane, active scheduling requests, no-candidate reason distribution, blocked dependencies, lease-denial rate, scheduling latency, and replay health.
- Operators need redacted decision explanations, dependency freshness, scheduler policy version rollout status, and fairness window state.
- Health checks should cover Overqueue, Overregistry, Overcell heartbeat freshness, Overguard, Overlease, Overwatch, and optional trust/grant/cache/route providers.
- Scheduler policy changes must be staged, test-replayed, and auditable before production rollout.
- Replay tools must support sampled decisions and incident-specific decisions.

## Failure Modes And Recovery

- Queue claim conflict: keep the existing owner and return current claim state.
- Missing manifest or invalid resource card: fail or dead-letter through Overqueue with reason code.
- Stale node facts: exclude or block based on scheduler policy.
- Policy denial: record matched policy refs and candidate reason codes.
- No eligible candidates: produce no-candidate decision with safe remediation hints.
- Lease race or denial: retry with fresh facts or mark blocked according to Overlease reason.
- Dependency outage: keep request blocked with retry-after and traceable dependency status.
- Scheduler crash: resume from append-only scheduling request and idempotency keys.
- Gang scheduling partial failure: release any partial reservations and record rollback evidence.

## Validation Plan

The service implementation plan lists these requirements:

- Scheduler rejects ineligible nodes with reason codes.
- Scheduler produces replayable decisions from stored facts.
- Multi-node lease requests are atomic when gang scheduling is enabled.

Additional SDS-level validation:

- Contract tests for claim, evaluate, reserve, explain, cancel, and replay APIs.
- Fixture tests for tenant visibility, resource class, capability, node state, trust class, locality, cache, grant, and policy filters.
- Determinism tests proving the same fact bundle and policy version produce the same decision.
- Redaction tests for user-facing and operator-facing explanation reads.
- Lease-race tests covering stale capacity and concurrent reservations.
- Gang scheduling tests covering all-or-nothing success, partial denial, rollback, and retry.
- Replay tests using stored fact versions rather than current live facts.

## Build Breakdown

1. Define scheduling request, candidate snapshot, reason code, placement decision, lease request, and replay bundle schemas.
2. Implement deterministic candidate filtering for private nodes.
3. Add resource-card fit, workload class, data class, node state, trust class, region, and cache filters.
4. Add explainable scoring and deterministic tie-breaks.
5. Add Overlease reservation calls for single-node jobs.
6. Bind leased assignments back to queue and Overrun handoff.
7. Add no-candidate, blocked, cancellation, and lease-denial flows.
8. Add replay endpoint and decision validation tests.
9. Add gang scheduling only after single-node scheduling is stable.

## Handoff And Downstream Use

Oversched hands lease-bound assignment refs to Overrun through Overqueue and Overcell. Overrun must verify the lease and manifest before execution. Overwatch, Overclaim, admin UI, CLI, SDK, and later central AI should use placement decisions and replay bundles rather than scraping scheduler logs.

## Open Design Questions

Resolved decisions:

- Resolved: Phase 3 needs only deterministic private-swarm fairness: queue priority class, retry/age ordering, lease availability, and stable tie-breaks inside tenant-visible private candidate pools. Before Phase 10 trusted federation or Phase 11 public capacity is introduced, Oversched must require a versioned fairness profile backed by Overgrant and Overguard that separates private, trusted-federation, public-interest, limited-public, and system-service candidate pools; enforces per-tenant, per-app/service-account, per-grant, per-provider, and per-purpose-tag quota windows; applies max-share, burst, aging/no-starvation, abuse-throttle, and dispute/hold constraints; and records reason codes when fairness rather than raw capacity blocks placement. Public and federation fairness profiles must be replayable and visible in placement decisions before any external capacity can be scheduler-eligible.
- Resolved: user-facing no-candidate explanations may expose only coarse, remediation-oriented facts: missing workload/resource class, incompatible runtime or sandbox, no eligible GPU/memory/storage/locality class, policy/trust/grant/cache/route block, stale prerequisite evidence, deadline/capacity class unavailable, or public/federation class not allowed for the workload. They must not expose exact provider capacity, node ids, hostnames, addresses, topology, precise free resources, raw benchmark values, competing tenant/workload counts, payout or dispute state, fraud or anti-Sybil internals, private route details, or provider contact/identity details outside the caller's authority. Operator and audit views can follow Overwatch, Oververify, Hardware Discovery, Benchmark Runner, and Overlease refs through their own redaction profiles.
- Resolved: globally governed scoring rules include every hard filter, workload/data/system/public candidate-pool separation, provider and node trust gates, policy-denial semantics, reason-code taxonomy, deterministic tie-break algorithm, grant and quota semantics, fairness windows, abuse throttles, public-interest purpose handling, redaction profiles, dry-run/replay requirements, and any weight that affects shared, public, federation, or system-service supply. Tenants may configure only bounded soft preferences inside an already-allowed candidate pool, such as locality preference, cache affinity, speed-versus-cost-class preference, batch versus interactive priority, deadline sensitivity, and tenant-visible provider allow/avoid preferences. Tenant configuration cannot override Overguard denials, public/private/system-service separation, grant limits, fairness caps, trust requirements, lease safety, or explanation redaction.
- Resolved: the minimum useful gang scheduling model is an all-or-none placement group after single-node scheduling is stable. A gang request must declare a fixed workload group id, member roles, required node count, per-member resource cards, shared manifest and scheduler policy version, locality or anti-affinity constraints, a shared lease validity window, and rollback behavior. Oversched selects the full node set as one placement decision and asks Overlease for an atomic lease set with prepare, commit, rollback, and bounded commit timeout semantics; Overrun may start no member until `overlease.set_committed`, and Overmeter links usage to both the set id and member lease ids. Partial, quorum, degraded, or elastic execution stays denied until Overpack, Oversched, Overlease, Overrun, Overmeter, and policy docs define the manifest contract, accounting treatment, failure behavior, and user-visible result semantics.
- Resolved: dry-run replay is mandatory before rollout for scheduler policy changes that can alter candidate eligibility, selected node, no-candidate state, lease request shape, reason-code output, or user/operator explanation content. This includes filter-chain changes, hard-denial changes, candidate-pool separation, fairness windows or weights, deterministic tie-breaks, trust/grant/cost/cache/locality input semantics, public/federation/system-service eligibility, gang scheduling rules, redaction-profile changes, and any policy bundle used by Overguard decisions consumed by Oversched. Low-risk observability wording or dashboard-only changes that do not affect decisions, stored refs, or explanation reason codes can use normal review, but the active scheduler policy version still needs traceable rollout evidence.
