# Overqueue Implementation Plan

## Objective

Build durable workload queueing with priority lanes, retry orchestration, deadlines, cancellation, backpressure, and dead-letter handling.

## First Build Phase

Skeleton in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); execution integration in [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md).

## Dependencies

- Overgate accepted commands.
- Overguard admission.
- Shared queue item schema.
- Overwatch event log.

## Development Order

1. Implement persistent pending queue items.
2. Add priority, deadline, retry, cancellation, and dead-letter fields.
3. Add at-least-once delivery with idempotent workload ids.
4. Add backpressure and queue health events.
5. Connect Oversched consumption in [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md).

## Contracts And Interfaces

- Queue item schema.
- Enqueue API.
- Scheduler fetch/ack API.
- Retry and dead-letter event contracts.

## Detailed SDS

- [Overqueue SDS](../../sds/control_plane/overqueue.md)

## Sub-Build Plan

- [SUB BUILD PLAN #11 - Overqueue](../../build_plan/sub_build_plan_011_overqueue.md)

## Design Alignment

- Treat Overqueue as durable queue state and scheduler handoff, not as scheduler, lease, or runner logic.
- Accept enqueue writes only from Overgate-admitted commands or approved internal service accounts.
- Preserve priority, lane, retry, cancellation, backpressure, scheduler claim, and dead-letter state with reason codes and Overwatch refs.
- Keep at-least-once delivery safe through idempotent workload ids and scheduler claim acknowledgement.
- Ensure Phase 1 stops at durable pending state while Phase 3 expands the same records into real execution handoff.

## Validation

- Synthetic workload reaches pending state in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md).
- Retryable failures return to queue with reason codes.
- Dead-letter state preserves evidence and is queryable.

## Handoff

Overqueue hands accepted workloads to Oversched and preserves execution state for audit and recovery.
