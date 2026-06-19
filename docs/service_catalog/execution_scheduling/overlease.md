# Overlease Implementation Plan

## Objective

Provide short-lived reservations, concurrency locks, atomic lease sets, renewal, cancellation, and stale-lease cleanup.

## First Build Phase

[Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md).

## Dependencies

- Oversched placement decisions.
- Overcell node state.
- Shared lease schemas.
- Overwatch events.

## Development Order

1. Implement single-node lease creation, renewal, release, and expiration.
2. Add stale lease cleanup.
3. Add cancellation and drain behavior.
4. Add atomic multi-node lease sets.
5. Link usage windows to lease ids for metering.

## Contracts And Interfaces

- Lease record schema.
- Lease create/renew/release APIs.
- Stale cleanup event contract.
- Atomic lease-set contract.

## Detailed SDS

The detailed design contract lives in [Overlease SDS](../../sds/execution_scheduling/overlease.md).

- [SUB BUILD PLAN #21 - Overlease](../../build_plan/sub_build_plan_021_overlease.md)

## Design Alignment

- Treat Overlease as the reservation and execution-eligibility authority, not the scheduler, runner, or billing service.
- Require every runnable workload to hold an active lease matching node, workload, queue item, resource reservation, and expiry.
- Keep lease creation, renewal, release, cancellation, expiration, stale cleanup, and future multi-node lease sets as append-only auditable state.
- Link usage windows to lease ids so Overmeter can produce reproducible raw usage and signed rollups.

## Validation

- A node cannot run work without a valid lease.
- Expired leases are reclaimed safely.
- Multi-node lease either reserves every required node or none.

## Handoff

Overlease gates Overrun execution and protects resources from double booking.
