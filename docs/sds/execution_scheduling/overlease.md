SDS #21

# Overlease SDS

## Purpose

Provide short-lived, auditable resource reservations that prevent double booking and prove that a node may execute a specific workload for a bounded time window.

Overlease is the reservation authority for the Phase 3 private execution loop. It converts an Oversched placement decision into a lease record that Overcell and Overrun can verify before work starts. It owns lease creation, renewal, release, cancellation, expiration, stale cleanup, and atomic lease sets.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overlease.md](../../service_catalog/execution_scheduling/overlease.md) |
| Sub-build plan | [SUB BUILD PLAN #21 - Overlease](../../build_plan/sub_build_plan_021_overlease.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md) |

## Service Family

- Family: Execution and scheduling
- Owning layer: Reservation, concurrency control, and execution eligibility
- Primary data scope: lease requests, lease records, resource reservations, lease tokens, renewals, releases, expirations, cancellation records, lease sets, stale cleanup records, and usage-window refs
- First build phase from service plan: [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md)

## Problem Statement

Once Oversched selects a node, the system still needs a hard reservation boundary. Without it, a node can be double-booked, stale work can start after placement is no longer valid, cancellation and drain behavior becomes ambiguous, and metering cannot reliably tie usage to a valid execution window.

Overlease gives the execution loop a short-lived, signed reservation record. A node may only run a workload when it can prove that a lease is active, belongs to the workload, belongs to the node, has not expired, and has not been cancelled or revoked.

## Goals

- Create single-node leases from accepted placement decisions.
- Tie each lease to queue item, workload id, node id, tenant id, provider id, resource reservation, policy refs, and expiration.
- Support explicit renewal, release, cancellation, expiration, and stale cleanup.
- Prevent double booking through atomic resource checks and compare-and-swap style state transitions.
- Provide lease tokens or proofs that Overcell/Overrun can verify before execution.
- Link usage windows to lease ids so Overmeter can produce reproducible raw usage and rollups.
- Add atomic multi-node lease sets after single-node behavior is proven.

## Non-Goals

- Do not choose the node. Oversched owns placement.
- Do not execute work. Overrun executes lease-bound workloads through Overcell.
- Do not perform billing or settlement. Overmeter, ORU, Seal Ledger, and Overbill consume lease-linked usage.
- Do not override Overguard policy. Leases must reference the policy decision that allowed placement.
- Do not keep leases alive indefinitely; leases are short-lived execution windows, not durable entitlements.
- Do not hide resource conflict or expiration behind implicit retries.

## Primary Actors And Clients

- Oversched, which requests leases after placement decisions.
- Overqueue, which keeps workload state aligned with scheduled, leased, retry, cancelled, and dead-letter states.
- Overcell and Overrun, which validate active leases before starting work.
- Overmeter, which links raw usage to lease windows.
- Overwatch, which records lifecycle, conflict, expiration, and cleanup events.
- Operators inspecting active, stale, expired, cancelled, and conflicted reservations.

## Dependencies

- [Oversched](oversched.md) for placement decisions and candidate resource cards.
- [Overqueue](../control_plane/overqueue.md) for queued workload state and retry/cancellation handoff.
- [Overcell](overcell.md) for node lifecycle state, drain state, and assignment acceptance.
- [Overrun](overrun.md) for execution start validation and final release.
- [Overmeter](overmeter.md) for usage-window linkage.
- [Overwatch](../control_plane/overwatch.md) for audit events.
- [Overguard](../trust_policy_verification/overguard.md) for placement policy decision refs.
- Shared schema package for lease record and token schemas.

Early Phase 3 may implement a simple single-node lease store, but the record shape must support later multi-node atomic sets.

## Owned Responsibilities

Overlease owns:

- Lease request validation against placement decision, node state, resource reservation, workload id, and policy refs.
- Durable lease records with clear lifecycle transitions.
- Resource conflict detection for node-local reservations.
- Lease token/proof issuance and verification metadata.
- Renewal rules, maximum extension limits, and expiration behavior.
- Release, cancellation, and drain-aware state changes.
- Stale lease sweeps and cleanup evidence.
- Atomic lease-set semantics for later multi-node workloads.

Overlease must not write directly into Overrun execution state or Overmeter rollups. It emits records and events that those services consume.

## Data Model

The first implementation should define:

- `lease_request`: request id, placement decision id, queue item id, workload id, tenant id, provider id, node id, requested resource reservation, requested ttl, policy refs, and idempotency key.
- `lease_record`: lease id, workload id, queue item id, node id, tenant id, provider id, resource reservation, state, issued_at, expires_at, max_renew_until, renewal count, release refs, and audit refs.
- `resource_reservation`: CPU, GPU, memory, storage, network, accelerator, model runtime, and locality reservations with units and source resource card.
- `lease_token`: token id, lease id, node id, workload id, expiry, signature/key refs, audience, and verification hash.
- `lease_renewal`: lease id, requested extension, accepted extension, requester, reason, previous expiry, new expiry, and denial reason if rejected.
- `lease_release`: lease id, releasing actor/service, final execution state ref, release reason, released_at, and usage-window refs.
- `lease_cancellation`: lease id, canceller, cancellation reason, policy/operator refs, and expected node behavior.
- `lease_set`: set id, member lease ids, workload group id, atomicity rule, state, failure reason, and rollback refs.
- `stale_cleanup_run`: sweep id, matched leases, expired leases, conflicted leases, cleanup actions, and Overwatch refs.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

Phase 3 should expose narrow reservation commands:

- `POST /leases`: create a lease from a placement decision and resource reservation.
- `GET /leases/{lease_id}`: read lease state, resource reservation, expiry, and refs.
- `POST /leases/{lease_id}/renew`: extend a valid lease within policy limits.
- `POST /leases/{lease_id}/release`: release a lease after completion, cancellation, or failure.
- `POST /leases/{lease_id}/cancel`: cancel an active lease before or during execution.
- `POST /leases/{lease_id}/verify`: internal verification endpoint for Overcell/Overrun lease proofs.
- `POST /lease-cleanups`: run a bounded stale/expired lease cleanup sweep.
- `POST /lease-sets`: create an atomic multi-node lease set after single-node leasing is stable.

API requirements:

- Mutating requests require actor/service identity, tenant context, trace id, idempotency key, and policy refs.
- Lease creation must be idempotent for the same placement decision and queue item.
- Lease verification must return explicit valid, expired, cancelled, revoked, wrong-node, wrong-workload, and unknown states.
- Cleanup endpoints must be bounded and replayable.
- Reads must respect tenant/provider/operator visibility.

## Event Surface

- `overlease.requested`: lease requested from a placement decision.
- `overlease.created`: lease became active.
- `overlease.renewed`: lease expiry changed.
- `overlease.released`: lease ended normally.
- `overlease.cancelled`: lease cancelled by actor, policy, or queue state.
- `overlease.expired`: lease passed expiry and is no longer valid.
- `overlease.conflict_detected`: resource reservation conflict found.
- `overlease.cleanup_completed`: stale cleanup sweep completed.
- `overlease.set_committed`: atomic lease set reserved every required member.
- `overlease.set_rolled_back`: atomic lease set failed and released all members.

Events must include lease id, workload id, node id, tenant id, state, reason code, and trace refs. Payloads should not include workload secrets or private input content.

## Core Workflow

1. Oversched emits a placement decision with selected node, resource card, and policy refs.
2. Overlease validates the node state, resource reservation, workload id, tenant visibility, and idempotency key.
3. Overlease atomically reserves resources for a short TTL and issues a signed lease proof.
4. Overqueue moves the workload into leased state.
5. Overcell/Overrun verify the lease before execution starts.
6. Overrun reports progress and final state while Overmeter records usage under the lease window.
7. Overrun or Overqueue releases, renews, cancels, or lets the lease expire.
8. Cleanup sweeps reclaim stale leases and emit evidence.

## State Machine

Lease lifecycle:

1. `requested`: lease request accepted for validation.
2. `active`: reservation committed and lease proof issued.
3. `renewing`: renewal is being evaluated.
4. `released`: work completed or failed and resources were released.
5. `cancelled`: queue, policy, operator, or tenant cancelled the lease.
6. `expired`: TTL elapsed without renewal or release.
7. `revoked`: trust, policy, incident, or node state invalidated the lease early.
8. `conflicted`: reservation conflict was detected and lease is not valid for execution.
9. `cleanup_pending`: stale cleanup has matched the lease.
10. `cleaned`: cleanup completed and evidence was recorded.

Lease-set lifecycle:

1. `assembling`: member reservations are being checked.
2. `committed`: every member lease is active.
3. `rolled_back`: at least one member failed and all reserved members were released.
4. `partial_failure`: rollback needs operator or automated recovery because a member could not be released cleanly.

Published lease history is correction-based. A lease can be superseded by a new lease; it is not silently edited.

## Policy And Security

- A node may not execute work without a valid lease matching node id, workload id, queue item id, and expiry.
- Lease proofs must be signed or verifiable through Overkey/service credentials.
- Lease creation requires a policy-approved placement decision.
- Drain, maintenance, suspended, stale, offline, revoked, or incompatible node states block new leases.
- Renewals must have maximum duration and maximum count limits.
- Cancellation must propagate to Overcell/Overrun and preserve evidence.
- Clocks must be handled defensively; server-side expiry is authoritative and node-side checks must tolerate bounded skew only.
- Lease records must not contain workload secrets or raw private inputs.
- Multi-node lease sets must be all-or-none to avoid partial distributed work unless a later manifest explicitly permits partial execution.

## Metering And Accounting

Overlease is not a billing service, but it defines usage windows:

- Emit lease lifecycle events with timestamps and resource reservations for Overmeter.
- Link raw usage events to lease id, workload id, node id, provider id, tenant id, and resource dimensions.
- Record idle leased time separately from active execution time when possible.
- Preserve cancelled, expired, and retry windows for dispute and accounting analysis.
- Do not call external payment rails or apply pricing inside lease logic.

## Observability And Operations

- Operators need active leases, nearing-expiry leases, stale leases, conflicting reservations, renewal failures, cancellation propagation, and cleanup sweep results.
- Health checks should verify the lease store, signer/verifier availability, Overwatch event emission, queue handoff, and cleanup workers.
- Dashboards should expose lease pressure by node, tenant, provider, workload class, and resource dimension.
- Maintenance commands should support bounded cleanup dry-runs and forced cancellation with signed operator action.
- Migration tooling must preserve lease history, token verification refs, and usage-window refs.

## Failure Modes And Recovery

- Placement decision missing or stale: deny lease creation with reason code.
- Node becomes stale before execution starts: revoke or cancel lease and hand workload back to queue policy.
- Lease renewal lost in transit: Overrun must stop before expiry unless a renewed proof is received.
- Cleanup race with release: use idempotent state transitions and return final state rather than double releasing.
- Multi-node set member conflict: roll back all reserved members and emit set rollback evidence.
- Token verification failure: reject execution and emit security event.
- Lease store unavailable: deny new leases; existing lease proofs remain valid only until expiry.

## Validation Plan

The service implementation plan lists these requirements:

- A node cannot run work without a valid lease.
- Expired leases are reclaimed safely.
- Multi-node lease either reserves every required node or none.

Additional SDS-level validation:

- Contract tests for create, renew, release, cancel, verify, cleanup, and lease-set APIs.
- Concurrency tests proving the same node/resource cannot be double-booked.
- Expiry and clock-skew tests.
- Idempotency tests for repeated placement decisions and repeated release/cancel calls.
- Drain/maintenance tests proving new leases are blocked and active leases are handled by policy.
- Overrun verification tests for valid, expired, revoked, wrong-node, and wrong-workload proofs.
- Replay tests proving usage windows can be reconstructed from lease events.

## Build Breakdown

1. Define lease, reservation, token, renewal, release, cancellation, and cleanup schemas.
2. Implement single-node lease create, verify, renew, release, cancel, and expire.
3. Integrate Oversched placement decisions and Overqueue state transitions.
4. Add Overcell/Overrun lease verification before execution.
5. Add stale cleanup sweeps and operator diagnostics.
6. Link lease windows to Overmeter raw usage events.
7. Add atomic lease sets for multi-node workloads after the single-node loop is stable.

The Phase 3 target is a reliable reservation boundary for one private workload running on one private node.

## Handoff And Downstream Use

Overlease gates Overrun execution, protects resources from double booking, and creates the usage window that Overmeter and later accounting services need. Downstream services should verify leases through the Overlease contract rather than reading private lease storage.

## Open Design Questions

Resolved decisions:

- Phase 3 uses classed, server-authoritative lease windows tied to the Overpack runtime timeout and the Overcell private-node heartbeat profile. Simple command/probe jobs default to a five-minute initial lease, 60-second renewal increments, and a 15-minute maximum renew-until. Container jobs default to a 15-minute initial lease, five-minute renewal increments, and a maximum renew-until equal to the manifest runtime timeout plus a five-minute cleanup grace, capped at two hours unless a later policy profile explicitly raises it. Model jobs in Phase 3 are short batch or inference jobs only, not resident model-serving services; they default to a 10-minute initial lease, two-minute renewal increments, and a 60-minute maximum renew-until. Any workload whose runtime timeout exceeds its class cap must use an explicit signed policy profile before scheduling, and Overrun must stop or renew before expiry rather than relying on node-local grace.
- V0 atomic reservation must cover the dimensions that can cause immediate double booking on one private node: node id, resource-instance refs, CPU core/share allocation, memory bytes, GPU device or partition refs, GPU memory, accelerator/runtime slot, local scratch-storage reservation, active-lease/concurrency slot, and the workload class/runtime adapter needed to execute the job. Network locality, bandwidth class, cache hints, and route constraints are eligibility and metering facts in Phase 3 unless the Overpack manifest explicitly requires an exclusive transfer or route window; those exclusive cases become lease-set or Overmesh-scoped reservations after the single-node loop is stable.
- V0 uses control-plane lease verification through Overlease before Overcell hands work to Overrun and before Overrun starts side effects. Lease records and tokens still use canonical signed proof fields, Overkey signer refs, audience, expiry, and verification hashes so later offline verification can be added without changing the public shape. During temporary control-plane disconnect, an already verified running attempt may continue only until the signed `valid_until`/`expires_at` boundary and must stop before expiry unless a renewed proof was received; a node may not start a new attempt from an unverified cached proof.
- Cancellation is resolved by action severity, while preserving every observed cause in the lease history. Policy revocation, trust revocation, credential revocation, incident response, or legal/compliance hold wins and moves the lease to `revoked` with forced stop semantics. Tenant or workload-owner cancellation is next and moves the lease to `cancelled` unless a higher-severity revocation already owns the terminal reason. Node drain or maintenance blocks new leases and requests graceful release or reschedule; it does not override a tenant cancellation or policy revocation. Expiry and stale cleanup are fallback terminal paths when no explicit cancellation/revocation arrived. The primary terminal reason follows that priority, and secondary simultaneous causes are recorded as evidence refs rather than discarded.
- Distributed or replicated jobs require an all-or-none lease-set contract before they are allowed. The minimum accepted lease-set semantics are: one set id, one workload group id, one manifest/policy version, per-member resource reservations, a shared validity window, prepare/commit/rollback phases, bounded commit timeout, rollback evidence for every prepared member, set-level renewal/cancellation/revocation, and Overwatch evidence linking every member lease. Partial execution is denied by default; a later manifest may opt into degraded or quorum execution only after Oversched, Overlease, Overrun, Overmeter, and policy docs define the quorum rule, accounting treatment, failure behavior, and user-visible result semantics.
