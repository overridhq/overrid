# Benchmark Runner Implementation Plan

## Objective

Measure useful node capacity so scheduling and verification do not rely only on provider claims.

## First Build Phase

[Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md).

## Dependencies

- Hardware discovery.
- Overcell node-agent execution.
- Oververify benchmark validation.

## Development Order

1. Add safe CPU, GPU, disk, network, cold-start, and sustained reliability benchmarks.
2. Normalize benchmark outputs.
3. Sign benchmark records with node credentials.
4. Publish benchmark evidence to Overregistry.
5. Re-run benchmarks on schedule or after runtime changes.

## Contracts And Interfaces

- Benchmark job manifest.
- Benchmark result schema.
- Signed evidence record.
- Capability update contract.

## Detailed SDS

The detailed design contract lives in [Benchmark Runner SDS](../../sds/execution_scheduling/benchmark_runner.md).

## Design Alignment

- Treat Benchmark Runner as the measured-capacity evidence producer, not as a scheduler, trust scorer, or generic workload runner.
- Keep Hardware Discovery claims, benchmark samples, normalized results, anomalies, and invalidations as separate records.
- Publish signed benchmark evidence to Overregistry so Oversched, Oververify, and provider reputation can consume versioned results without direct private-storage access.
- Bound every suite with safety limits so bootstrap benchmarks cannot harm founder hardware or disguise production workloads.

## Validation

- Benchmarks run without harming node availability.
- Results are stable enough for scheduler tiers.
- Impossible or degraded results can be challenged by Oververify.

## Handoff

Benchmark records become evidence for Oververify, Oversched, and provider reputation.
