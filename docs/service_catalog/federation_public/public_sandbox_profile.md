# Public Sandbox Profile Implementation Plan

## Objective

Prevent public nodes from receiving secrets, private data, regulated data, or backbone workloads.

## First Build Phase

[Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md).

## Dependencies

- Overguard.
- Overrun.
- Workload classifier.
- Public provider onboarding.

## Development Order

1. Define public low-sensitivity sandbox rules.
2. Deny secret injection and private data mounts.
3. Add filesystem, network, memory, and runtime caps.
4. Add output validation and artifact quarantine.
5. Add privacy-preserving log handling.

## Contracts And Interfaces

- Sandbox profile schema.
- Runtime restriction contract.
- Output validation record.
- Artifact quarantine refs.

## Validation

- Private workloads cannot be scheduled to public sandbox.
- Secret refs are denied before execution.
- Public jobs stay inside caps and egress policy.

## Handoff

Public sandbox profile enables safe limited use of public supply.

## Detailed SDS

- [Public Sandbox Profile SDS](../../sds/federation_public/public_sandbox_profile.md)

## Design Alignment

The SDS defines Public Sandbox Profile as the versioned public low-sensitivity runtime safety contract, not an onboarding, scheduling, runner, trust, or payout service. It owns sandbox profile versions, restriction sets, workload/data-class bindings, secret and mount denials, output validation, artifact quarantine, log redaction, and replayable profile evaluations.

Build this service so Overguard, Oversched, Overrun, Overcell, Public Provider Onboarding, Fraud Control, and Overwatch consume profile/evaluation refs instead of duplicating sandbox rules. The first build must hard-deny private, regulated, secret-bearing, and system-service workloads before public placement.
