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
- Overrid-shaped local durable state, durable job table, and object/artifact stub choices.

## Development Order

1. Freeze Phase 1 SDS attachment, local-only boundary, first-build and expansion gates, resolved SDS decisions, and documentation-update rules.
2. Add local service definitions for API, worker, Overrid-shaped local durable state, durable job table, object/artifact stubs, and node-agent simulator.
3. Add seed fixtures for tenant, identity, key, node, workload, and package.
4. Add reset and seed commands.
5. Add local observability output for logs, events, and job state.
6. Add smoke tests that run against the local stack.

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

## Phase 1 Implementation Gates

- First build point remains [Phase 0: Foundation](../../build_plan/phase_00_foundation.md) with `buildable_phase_0` local lifecycle, fixtures, reset/seed, health, diagnostics, and smoke.
- Phase 1 control-plane work may use `local_smoke_prerequisite` fixtures and lifecycle hooks, but the local stack is not a production control plane and does not own Overgate, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, CLI, or SDK behavior.
- Phase 2 node identity and Phase 3 execution-loop support are `owning_service_required`; simulator expansion cannot bypass Overcell, Hardware Discovery, Benchmark Runner, Node Installer, Overpack, Oversched, Overlease, Overrun, and Overmeter contracts.
- Later local service families remain `planned_disabled` until the owning SDS/API defines local-test behavior; governance and compliance workflows are `not_local_stack_owned`.
- Direct storage, queue, object, event, or local file inspection is allowed only for reset, seed, health, and diagnostics.
- New local-stack profiles, services, simulators, fixture families, smoke scenarios, diagnostic events, commands, or health fields must update owning service SDS/API docs first, then shared schemas when serialized, then the sub-build plan, this service catalog entry, harness scenarios, and validation evidence.

Resolved decisions:

- Rust-owned embedded/local durable state behind Overbase-shaped and Overqueue-shaped contracts.
- Filesystem-backed content-addressed Overstore stub with BLAKE3/content hashes, object manifests, local/test markers, deterministic reset, and redacted artifact bundles.
- Exactly one local Overcell-like node simulator before Phase 2.
- Deterministic loopback ports: API `18080`, worker health/metrics `18081`, node-agent simulator `18082`, object/artifact stub `18083`, local event/audit query `18084`, and optional developer UI `18085`.
- Reproducible Linux x86_64 clean-checkout CI target with repository-pinned Rust tooling, loopback networking, no external database/queue/object-store services, no ambient keychain, and no cloud credentials.

## Validation

- A clean checkout can start the local stack.
- Reset removes local state and reseed recreates deterministic fixtures.
- Smoke test creates a tenant, actor, command, and audit event.

## Handoff

This enables repeatable control-plane and execution-loop development.
