# Local Development Stack Implementation Plan

## Objective

Let developers run a complete minimal Overrid environment locally without external cloud services.

## First Build Phase

[Phase 0: Foundation](../../build_plan/phase_00_foundation.md).

## Detailed SDS

[Local Development Stack SDS](../../sds/foundation/local_development_stack.md).

## Sub-Build Plan

[SUB BUILD PLAN #4 - Local Development Stack](../../build_plan/sub_build_plan_004_local_development_stack.md).

## Dependencies

- Repository layout.
- Shared schema package.
- Local database, queue, and object-store choices.

## Development Order

1. Add local service definitions for API, worker, database, queue, object store, and node-agent simulator.
2. Add seed fixtures for tenant, identity, key, node, workload, and package.
3. Add reset and seed commands.
4. Add local observability output for logs, events, and job state.
5. Add smoke tests that run against the local stack.

## Contracts And Interfaces

- `start`, `stop`, `reset`, and `seed` commands.
- Local environment variables.
- Fixture ids and deterministic seed data.
- Health endpoints for local services.
- Local stack profile, service definition, port registry, reset plan, and seed manifest schemas.
- Loopback-only default service binding and test-only fixture markers.

## Design Alignment

- The local stack is not a production deployment mechanism.
- It must run without external cloud services or paid SaaS dependencies.
- Reset/seed behavior must be deterministic and guarded by local/test state markers.

## Validation

- A clean checkout can start the local stack.
- Reset removes local state and reseed recreates deterministic fixtures.
- Smoke test creates a tenant, actor, command, and audit event.

## Handoff

This enables repeatable control-plane and execution-loop development.
