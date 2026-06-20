# Federation Template Service Implementation Plan

## Objective

Let known organizations share capacity under explicit tenant, policy, billing, and dispute boundaries.

## First Build Phase

[Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md).

## Dependencies

- Overtenant.
- Overgrant.
- Overguard.
- Overbill.
- Overclaim.

## Development Order

1. Define templates for universities, companies, research labs, nonprofits, family/community clouds, trusted partner swarms, and emergency pools.
2. Add participant identity and operator records.
3. Add capacity contribution scope and eligible workload rules.
4. Add billing, dispute, and reporting boundaries.
5. Add template validation and admin UI support.

## Contracts And Interfaces

- Federation template schema.
- Participant record.
- Capacity contribution record.
- Reporting and dispute boundary refs.

## Validation

- Known partner capacity is visible only under approved policy.
- Cross-tenant use cites the federation template.
- Disputes can be traced to participant, tenant, workload, and billing scope.

## Handoff

Federation templates prepare Overrid for trusted external capacity before public supply.

## Detailed SDS

The detailed design contract lives in [Federation Template Service SDS](../../sds/federation_public/federation_template_service.md).

## Sub-Build Plan

Implementation sequencing lives in [SUB BUILD PLAN #52 - Federation Template Service](../../build_plan/sub_build_plan_052_federation_template_service.md).

## Design Alignment

- Treat Federation Template Service as the trusted-partner capacity contract layer: template versions, participant roles, capacity scopes, workload/data-class eligibility, accounting boundaries, reporting, dispute boundaries, and concrete federation instances.
- Keep identity verification, grants, accounting, dispute adjudication, scheduling, and public-provider onboarding in their owning services.
- Require verified known participants, Overguard policy, accounting/grant refs, and dispute refs before any cross-tenant usage.
- Feed Overtenant, Overguard, Overgrant, Overmeter, Overbill, Overclaim, Purpose Tag Registry, Public-Interest Pool Service, SDK, CLI, and admin UI with federation boundary refs.
