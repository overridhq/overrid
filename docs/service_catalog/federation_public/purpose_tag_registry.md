# Purpose Tag Registry Implementation Plan

## Objective

Define stewarded purpose tags for science, education, medical, opensource, climate, public infrastructure, and later approved categories.

## First Build Phase

[Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md).

## Dependencies

- Overregistry.
- Overgrant.
- Overguard.
- Stewardship governance.

## Development Order

1. Define tag records and eligibility criteria.
2. Add evidence requirements and steward records.
3. Add tag review and update flow.
4. Integrate tags with Overgrant and policy dry-run.
5. Add reporting hooks for public-interest pools.

## Contracts And Interfaces

- Purpose tag schema.
- Eligibility criteria schema.
- Evidence requirement refs.
- Stewardship review events.

## Validation

- Workloads claiming a purpose tag must supply required evidence.
- Ineligible workloads are denied before grant use.
- Tag changes are versioned and auditable.

## Handoff

Purpose tags power public-interest pools, central AI stewardship, and grant reporting.

## Detailed SDS

- [Purpose Tag Registry SDS](../../sds/federation_public/purpose_tag_registry.md)

## Sub-Build Plan

- [SUB BUILD PLAN #57 - Purpose Tag Registry](../../build_plan/sub_build_plan_057_purpose_tag_registry.md)

## Design Alignment

The SDS defines Purpose Tag Registry as the stewarded, versioned purpose taxonomy and evidence-requirement authority. It owns tag definitions, tag versions, eligibility criteria, evidence requirement bundles, steward assignments, review proposals, deprecations, purpose-claim validation refs, and policy export refs.

Build this service as a vocabulary and validation layer for Overgrant, Public-Interest Pool Service, Overguard, Central AI Service, and reporting. It must not authorize grants, allocate resources, decide project merit, adjudicate disputes, or turn purpose tags into speculative rights.
