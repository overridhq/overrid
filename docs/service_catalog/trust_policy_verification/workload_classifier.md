# Workload Classifier Implementation Plan

## Objective

Normalize workload sensitivity and allowed execution environments before scheduling.

## First Build Phase

[Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md).

## Dependencies

- Workload manifest.
- Overguard policy.
- Overregistry package records.
- Data classification rules.

## Development Order

1. Define workload classes: system-service, private tenant, trusted federation, public low-sensitivity, public-interest, regulated, and secret-bearing.
2. Validate declared class against manifest facts.
3. Add downgrade or denial reason codes.
4. Expose class decisions in policy dry-run and admin UI.

## Contracts And Interfaces

- Workload class enum.
- Classification decision schema.
- Reason code registry.
- Policy input fact contract.

## Detailed SDS

The detailed design contract lives in [Workload Classifier SDS](../../sds/trust_policy_verification/workload_classifier.md).

- [SUB BUILD PLAN #37 - Workload Classifier](../../build_plan/sub_build_plan_037_workload_classifier.md)

## Design Alignment

- Treat Workload Classifier as a deterministic classification-fact producer, not the policy engine, scheduler, runner, Overvault, or billing service.
- Store workload/data class definitions, classification input snapshots, rule matches, classification decisions, reason codes, policy input facts, overrides, and replay bundles.
- Use the strictest applicable class when declared class conflicts with manifest, data, secret, egress, tenant, or package evidence.
- Feed Overguard, Policy Dry-Run API, Oversched, Overrun, Overmesh, Overcache, Overvault, compliance boundaries, SDK, CLI, and admin UI with versioned classification decisions.

## Validation

- Secret-bearing workloads cannot be classified as public low-sensitivity.
- System-service workloads require trusted placement.
- Classification decisions are visible in scheduler reasoning.

## Handoff

Workload classification feeds Overguard, Oversched, Overrun, public pool controls, and compliance boundaries.
