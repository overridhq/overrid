# Overcell Implementation Plan

## Objective

Build the node-agent and resource abstraction for participant-owned compute, GPU, storage, network, data, model, and service capacity.

## First Build Phase

[Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md).

## Dependencies

- Overpass node identities.
- Overkey node credentials.
- Overregistry capability records.
- Node installer.

## Development Order

1. Implement node-agent registration and credential enrollment.
2. Add heartbeat, drain, shutdown, and reconnect flows.
3. Add local runtime and capability reporting.
4. Publish capability records into Overregistry.
5. Emit health and state events to Overwatch.

## Contracts And Interfaces

- Node registration API.
- Heartbeat payload.
- Capability record schema.
- Node state transition events.

## Detailed SDS

The detailed design contract lives in [Overcell SDS](../../sds/execution_scheduling/overcell.md).

## Design Alignment

- Treat Overcell as the supervised node agent and resource-state boundary, not as the scheduler, lease creator, trust scorer, or billing engine.
- Preserve stable node identity across restarts while accepting only signed, authorized control-plane commands.
- Report heartbeat, resource snapshots, capability refs, assignment results, and local audit-spool evidence through explicit contracts.
- Keep Phase 2 narrow enough for founder hardware while preserving the path to Phase 3 lease-bound execution and Phase 7 grid-resident backbone workloads.

## Validation

- Seed server and GPU node register successfully.
- Restarted node-agent preserves identity.
- Stale and expired node states are detected.

## Handoff

Overcell supplies verified resource inventory to Oversched, Oververify, and the grid-resident backbone.
