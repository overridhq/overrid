# Oversched Implementation Plan

## Objective

Build the policy-aware scheduler that chooses placements from availability, trust, cost, locality, cache, grant, and lease facts.

## First Build Phase

[Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md).

## Dependencies

- Overqueue.
- Overregistry capability records.
- Hardware Discovery and Benchmark Runner evidence.
- Overcell heartbeat, load, and node-state signals.
- Overpack workload manifests and resource cards.
- Overguard policy decisions.
- Oververify trust signals.
- Overgrant and Overmark signals.
- Overcache scoped cache hints.
- Overmesh locality and private route hints.
- Overlease.
- Overwatch audit and replay evidence.

## Development Order

1. Implement deterministic candidate filtering for private nodes.
2. Add workload class, data class, resource card, node state, trust, grant, cost-class, cache, locality, and policy filters.
3. Add explainable placement decision records, scheduler policy versions, reason codes, deterministic tie-breaks, and replay bundles.
4. Add Overlease reservation calls and queue/Overrun handoff after single-node decisions are durable.
5. Add no-candidate, blocked, cancellation, lease-denial, and retry/dead-letter flows.
6. Add gang scheduling for multi-node jobs only after single-node scheduling is stable.

## Contracts And Interfaces

- Scheduler fetch API.
- Scheduling request and candidate snapshot schema.
- Placement decision schema.
- Filter result and score explanation schema.
- Candidate reason codes.
- Lease request contract.
- Replay bundle contract.

## Detailed SDS

The detailed design contract lives in [Oversched SDS](../../sds/execution_scheduling/oversched.md).

- [SUB BUILD PLAN #26 - Oversched](../../build_plan/sub_build_plan_026_oversched.md)

## Design Alignment

- Treat Oversched as the placement decision and lease-request handoff authority, not the runner, lease store, trust scorer, or accounting service.
- Store candidate snapshots, filter results, candidate scores, placement decisions, lease requests, gang lease plans, and replay bundles with fact-version refs.
- Keep scheduling deterministic and explainable before adding optimization: stable reason codes and replayability are part of the v0 contract.
- Use Overlease for reservations, Overqueue for retry/dead-letter flow, Overguard/Oververify/Overgrant/Overmark for policy/trust/grant/cost-class refs, and Overrun only after lease binding.
- Keep user-facing no-candidate explanations coarse and remediation-oriented while provider-private capacity, topology, trust internals, dispute state, and route details remain behind owning-service redaction profiles.

## Validation

- Scheduler rejects ineligible nodes with reason codes.
- Scheduler produces replayable decisions from stored facts.
- No-candidate, blocked, cancellation, and lease-denial paths remain auditable and redacted.
- Multi-node lease requests are atomic when gang scheduling is enabled.

## Handoff

Oversched hands leased assignments to Overrun through Overqueue and Overcell, emits scheduler usage facts to Overmeter, and depends on Overwatch/Overclaim for audit and dispute evidence.
