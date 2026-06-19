# Challenge Task Service Implementation Plan

## Objective

Actively test providers and nodes instead of trusting self-reported capacity or behavior.

## First Build Phase

[Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md) for trusted nodes; [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md) for public providers.

## Dependencies

- Oververify.
- Overcell.
- Benchmark runner.
- Overwatch evidence.

## Development Order

1. Define liveness, GPU capability, benchmark recheck, and result consistency challenges.
2. Schedule safe challenges through the private execution loop.
3. Record challenge outcomes and evidence.
4. Apply eligibility, throttling, or payout-hold consequences.
5. Add public-provider challenge policy in [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md).

## Contracts And Interfaces

- Challenge manifest.
- Challenge result schema.
- Consequence rule mapping.
- Evidence refs.

## Detailed SDS

The detailed design contract lives in [Challenge Task Service SDS](../../sds/trust_policy_verification/challenge_task_service.md).

## Design Alignment

- Treat Challenge Task Service as active verification-work orchestration, not the trust scorer, scheduler, runner, or payout authority.
- Store challenge templates, manifests, target snapshots, assignments, run refs, results, rate-limit records, consequence proposals, and replay bundles.
- Keep challenge payloads synthetic or public and route execution through Overqueue, Oversched, Overlease, Overcell, and Overrun.
- Feed outcome refs to Oververify, Overclaim, payout holds, fraud controls, reputation, and central AI review without inspecting private workload data.

## Validation

- Failed challenge produces visible trust and eligibility change.
- Challenge tasks cannot access private workload data.
- Repeated challenges are rate-limited and auditable.

## Handoff

Challenge outcomes feed Oververify, Oversched, payout holds, fraud controls, and reputation.
