# Public Provider Onboarding Implementation Plan

## Objective

Admit unknown or semi-trusted providers only into bounded public low-sensitivity capacity.

## First Build Phase

[Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md).

## Dependencies

- Overpass provider identity.
- Overcell node agent.
- Oververify.
- Overbill payout eligibility.
- Anti-Sybil service.

## Development Order

1. Collect provider identity level, node identity, contact, payout eligibility, resource claims, software version, and region.
2. Enforce accepted workload class restrictions.
3. Add enrollment policy acknowledgement.
4. Trigger verification and challenge tasks.
5. Publish only allowed capability records.

## Contracts And Interfaces

- Public provider record.
- Provider verification tier.
- Payout eligibility refs.
- Public workload acceptance contract.

## Validation

- New public providers cannot receive private, secret-bearing, regulated, or system workloads.
- Failed verification reduces eligibility.
- Onboarding records are auditable.

## Handoff

Public provider onboarding feeds anti-Sybil, Oververify, public sandbox, fraud control, and payout holds.

## Detailed SDS

The detailed design contract lives in [Public Provider Onboarding SDS](../../sds/federation_public/public_provider_onboarding.md).

## Sub-Build Plan

Implementation sequencing lives in [SUB BUILD PLAN #55 - Public Provider Onboarding](../../build_plan/sub_build_plan_055_public_provider_onboarding.md).

## Design Alignment

- Treat Public Provider Onboarding as the limited public-provider front door: enrollment records, node refs, policy acknowledgements, resource claims, verification refs, payout eligibility refs, public workload acceptance contracts, and capability publication refs.
- Keep identity verification, anti-Sybil scoring, benchmarking, node supervision, scheduling, payout execution, fraud adjudication, and reputation updates in their owning services.
- Hard-limit new providers to public low-sensitivity workloads with public sandbox profile requirements; private, regulated, secret-bearing, and system-service workloads must remain denied.
- Feed Overregistry, Overguard, Oversched, Public Sandbox Profile, Fraud Control Service, Reputation and Anti-Sybil Service, Oververify, Challenge Task Service, Overbill, Provider Payout Service, Overclaim, SDK, CLI, and admin UI with published eligibility refs.
