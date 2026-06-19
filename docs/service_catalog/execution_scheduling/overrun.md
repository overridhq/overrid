# Overrun Implementation Plan

## Objective

Build sandbox preparation, pre-flight checks, execution supervision, credential mounting, result handoff, and safe termination.

## First Build Phase

[Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md).

## Dependencies

- Overlease.
- Overpack.
- Overvault.
- Overstore.
- Overmeter.
- Overwatch.

## Development Order

1. Fetch assignments only when a valid lease exists.
2. Verify package hashes, signatures, runtime contract, egress policy, and secret mounts.
3. Prepare sandbox and mount inputs.
4. Supervise execution with timeout and cancellation.
5. Capture outputs, logs, metrics, and final state.
6. Cleanup sandbox and emit usage/events.

## Contracts And Interfaces

- Assignment contract.
- Runner state machine.
- Result record schema.
- Log and artifact refs.
- Usage event contract.

## Detailed SDS

The detailed design contract lives in [Overrun SDS](../../sds/execution_scheduling/overrun.md).

## Design Alignment

- Treat Overrun as the lease-bound node-side sandbox runner, not the scheduler, lease authority, package author, or accounting system.
- Verify lease, Overpack manifest, package hashes/signatures, runtime contract, policy refs, egress rules, secret refs, and input refs before side effects.
- Treat storage and secret adapters as explicit readiness gates; fail preflight instead of silently falling back from required Overstore or Overvault paths to local stubs.
- Produce distinct final states for success, retryable failure, final failure, cancellation, timeout, and cleanup failure.
- Emit execution evidence, logs/artifact refs, final state, and raw usage for Overwatch, Overmeter, Overclaim, and product adapters.

## Validation

- Successful, failed, cancelled, and timed-out jobs produce distinct final states.
- Secrets are mounted only when policy allows.
- Cleanup runs even after execution failure.

## Handoff

Overrun produces the evidence, results, and metering data used by Overmeter, Overwatch, Overclaim, and product adapters.
