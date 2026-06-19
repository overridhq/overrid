# Phase 3: Private Execution Loop

## Objective

Run real private workloads from signed request to placement, lease, execution, result, metering, retry, and audit.

This is the first phase where Overrid proves it can do useful work, not just describe resources.

## Depends On

- Phase 1 queued signed workloads.
- Phase 2 live private nodes and capability records.
- Basic package and workload manifest schema.

## Build Order

1. Define Overpack v0 workload manifest.
2. Add scheduler candidate selection.
3. Add Overlease reservations.
4. Add Overrun execution supervision.
5. Add result capture.
6. Add Overmeter raw usage events.
7. Add failure states, retry, timeout, cancellation, and dead-letter flow.
8. Run first real workload.

## Workstream 1: Overpack V0 Workload Manifest

Define a workload manifest that can describe:

- Command job.
- Container job.
- WASI job where feasible.
- Model inference job.
- Input references.
- Output references.
- Required resource card.
- Workload class.
- Data sensitivity.
- Egress policy.
- Secrets policy.
- Runtime timeout.
- Retry policy.

The manifest should be strict enough for scheduling and policy but small enough to implement quickly.

## Workstream 2: Oversched V0

Build placement logic for private swarm:

- Filter by tenant visibility.
- Filter by resource class.
- Filter by capability record.
- Filter by node state.
- Filter by trust class.
- Filter by locality or region if requested.
- Consider cache hints.
- Choose candidate and emit placement decision event.

The first scheduler should be deterministic and explainable. Reason codes are more important than sophisticated optimization at this stage.

## Workstream 3: Overlease V0

Add short-lived reservations:

- Lease id.
- Queue item id.
- Node id.
- Resource reservation.
- Expiration.
- Renewal.
- Release.
- Stale cleanup.

A node may only execute work when it holds a valid lease for that work.

## Workstream 4: Overrun V0

Build the node-side runner:

- Fetch workload assignment.
- Verify package or command.
- Prepare sandbox.
- Mount inputs.
- Apply egress policy.
- Start execution.
- Stream logs or structured progress.
- Enforce timeout.
- Capture output.
- Report final state.
- Cleanup sandbox.

The first runner should support a narrow workload class well rather than a broad set poorly.

## Workstream 5: Result Capture

Define result records:

- Success payload refs.
- Error type.
- Logs refs.
- Output artifact refs.
- Runtime metrics.
- Final state.
- Audit event refs.

Results should be returned through control-plane state, not by ad hoc node callbacks with no audit chain.

## Workstream 6: Overmeter Raw Events

Emit usage facts:

- CPU time.
- GPU time.
- Memory peak.
- Storage read/write.
- Network bytes.
- Queue wait.
- Wall time.
- Model inference count where applicable.

These are raw events, not billing. Phase 5 will convert signed rollups into ORU transitions.

## Workstream 7: Failure Handling

Implement states for:

- Pending.
- Scheduled.
- Leased.
- Starting.
- Running.
- Succeeded.
- Failed retryable.
- Failed final.
- Cancelled.
- Timed out.
- Dead-lettered.

Controlled failure is part of the product. Every failed state needs a reason code and audit trail.

## Validation

- A known private node runs a real job through queue, scheduler, lease, runner, metering, result return, and audit.
- Invalid package fails before execution.
- Node disconnect during lease causes retry or dead-letter based on policy.
- Cancellation interrupts a running job and records final state.
- Timeout terminates work and records usage up to termination.

## Exit Gate

Phase 3 is complete when a real private workload can run end to end with deterministic state transitions and auditable success and failure paths.

## Handoff To Phase 4

Phase 4 hardens the execution loop with trust, policy, verification, disputes, cache scopes, and safer multi-tenant boundaries.
