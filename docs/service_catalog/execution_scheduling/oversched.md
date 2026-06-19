# Oversched Implementation Plan

## Objective

Build the policy-aware scheduler that chooses placements from availability, trust, cost, locality, cache, grant, and lease facts.

## First Build Phase

[Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md).

## Dependencies

- Overqueue.
- Overregistry capability records.
- Overguard policy decisions.
- Overgrant and Overmark signals.
- Overlease.

## Development Order

1. Implement deterministic candidate filtering for private nodes.
2. Add trust class, workload class, resource card, region, and cache filters.
3. Add explainable placement decision records.
4. Add Overlease reservation calls.
5. Add gang scheduling for multi-node jobs after single-node scheduling is stable.

## Contracts And Interfaces

- Scheduler fetch API.
- Placement decision schema.
- Candidate reason codes.
- Lease request contract.

## Detailed SDS

The detailed design contract lives in [Oversched SDS](../../sds/execution_scheduling/oversched.md).

## Design Alignment

- Treat Oversched as the placement decision and lease-request handoff authority, not the runner, lease store, trust scorer, or accounting service.
- Store candidate snapshots, filter results, candidate scores, placement decisions, lease requests, gang lease plans, and replay bundles with fact-version refs.
- Keep scheduling deterministic and explainable before adding optimization: stable reason codes and replayability are part of the v0 contract.
- Use Overlease for reservations, Overqueue for retry/dead-letter flow, Overguard/Oververify/Overgrant/Overmark for policy/trust/grant/cost-class refs, and Overrun only after lease binding.

## Validation

- Scheduler rejects ineligible nodes with reason codes.
- Scheduler produces replayable decisions from stored facts.
- Multi-node lease requests are atomic when gang scheduling is enabled.

## Handoff

Oversched hands leased assignments to Overrun and depends on Oververify for trust signals.
