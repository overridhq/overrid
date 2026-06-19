# Overguard Implementation Plan

## Objective

Enforce policy for workload admission, data sensitivity, sandboxing, compliance, egress, secret access, provider eligibility, quota, and abuse prevention.

## First Build Phase

[Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md).

## Dependencies

- Overgate command context.
- Overregistry manifests.
- Overtenant quotas.
- Overvault secret refs.
- Oververify trust evidence.

## Development Order

1. Define policy input facts and decision schema.
2. Implement workload class, data sensitivity, quota, package trust, egress, secret access, and provider eligibility rules.
3. Add policy versioning and reason codes.
4. Add budget reservation prechecks.
5. Add replay support from stored facts.

## Contracts And Interfaces

- Policy decision schema.
- Reason code registry.
- Policy version metadata.
- Admission API.

## Detailed SDS

The detailed design contract lives in [Overguard SDS](../../sds/trust_policy_verification/overguard.md).

## Design Alignment

- Treat Overguard as the policy decision engine, not the queue, scheduler, runner, vault, trust scorer, or accounting service.
- Store policy bundles, input fact bundles, immutable decisions, matched rules, stable reason codes, rollout records, overrides, and replay bundles.
- Deny by default when critical facts are missing, stale, or unauthorized, and make decisions replayable from fact refs and policy versions.
- Use the same evaluator for real admission and Policy Dry-Run API while keeping dry runs side-effect-free.

## Validation

- Denials happen before execution.
- Decisions are replayable from stored facts and policy version.
- Reason codes are stable and visible to SDK/CLI/admin UI.

## Handoff

Overguard gates Overqueue, Oversched, Overrun, Overvault, Overgrant, and public-provider execution.
