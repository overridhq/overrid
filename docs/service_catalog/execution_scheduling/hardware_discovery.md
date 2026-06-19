# Hardware Discovery Implementation Plan

## Objective

Discover usable node capacity beyond hardware names so placement can be evidence-backed.

## First Build Phase

[Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md).

## Dependencies

- Overcell node agent.
- Host runtime access.
- Overregistry capability schema.

## Development Order

1. Detect CPU, memory, GPU, storage, network, OS, container/runtime, and region/locality facts.
2. Normalize hardware facts into capability records.
3. Mark unsupported or degraded runtime features.
4. Publish capability changes through Overcell.
5. Emit discovery errors and changes to Overwatch.

## Contracts And Interfaces

- Hardware fact schema.
- Capability record schema.
- Discovery error codes.
- Runtime support flags.

## Detailed SDS

The detailed design contract lives in [Hardware Discovery SDS](../../sds/execution_scheduling/hardware_discovery.md).

## Design Alignment

- Treat Hardware Discovery as observed inventory normalization, not measured performance, trust scoring, or scheduling authority.
- Preserve raw probe evidence refs while publishing normalized capability observations through Overregistry.
- Keep unsupported, unknown, degraded, and stale states explicit instead of guessing from incomplete host data.
- Redact host-sensitive identifiers and never collect user files, secrets, process memory, or unrelated private data.

## Validation

- CPU-only and GPU nodes produce normalized records.
- Missing runtime dependencies are reported clearly.
- Capability changes update Overregistry without losing history.

## Handoff

Hardware discovery feeds benchmark runner, Oververify, and Oversched.
