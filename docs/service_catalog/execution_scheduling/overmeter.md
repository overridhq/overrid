# Overmeter Implementation Plan

## Objective

Record usage events and signed rollups for compute, storage, bandwidth, requests, execution time, RAG retrieval, model inference, and app services.

## First Build Phase

Raw events in [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md); signed rollups in [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Dependencies

- Overrun result and metric events.
- Overlease usage windows.
- Shared usage schemas.
- Seal Ledger.

## Development Order

1. Emit raw usage events from private workloads.
2. Normalize CPU, GPU, storage, network, memory, data, and service dimensions.
3. Add rollup windows and signatures.
4. Add retention and dispute windows.
5. Feed signed rollups into Seal Ledger and Overbill.

## Contracts And Interfaces

- Raw usage event schema.
- Signed rollup schema.
- Resource dimension definitions.
- Retention and dispute-window metadata.

## Detailed SDS

The detailed design contract lives in [Overmeter SDS](../../sds/execution_scheduling/overmeter.md).

- [SUB BUILD PLAN #23 - Overmeter](../../build_plan/sub_build_plan_023_overmeter.md)

## Design Alignment

- Treat Overmeter as raw usage and signed rollup infrastructure, not billing, pricing, settlement, or ledger mutation.
- Tie usage to tenant, actor/app, provider, node, workload, queue item, lease, source refs, policy refs, and trace refs.
- Keep resource dimensions explicit so CPU, GPU, memory, storage, network, data, model, RAG, cache, and service units do not collapse into a vague balance.
- Preserve corrections as append-only records and keep disputed usage visible with hold state before settlement finality.

## Validation

- Workloads produce usage records tied to tenant, provider, node, workload, and lease.
- Rollups are reproducible from raw events.
- Disputed usage can be held before settlement finality.

## Handoff

Overmeter is the bridge from execution to ORU, Seal Ledger, billing, disputes, and native app usage displays.
