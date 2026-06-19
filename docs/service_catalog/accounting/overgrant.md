# Overgrant Implementation Plan

## Objective

Manage reservations, sponsorships, donations, purpose tags, quotas, priority allocation, and public-interest resource pools.

## First Build Phase

[Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) primitives; federation and public-interest expansion in [Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md).

## Dependencies

- ORU account service.
- Seal Ledger.
- Overguard.
- Oversched.
- Purpose tag registry.

## Development Order

1. Define grant source, eligible identities, purpose tags, resource dimensions, time windows, quotas, and fairness rules.
2. Add policy integration with Overguard.
3. Add scheduler signals for grant-backed workloads.
4. Add public-interest pool reporting in [Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md).
5. Add abuse and revocation controls.

## Contracts And Interfaces

- Grant record schema.
- Eligibility API.
- Purpose tag refs.
- Usage and report schema.

## Validation

- Eligible workloads can consume grant resources.
- Ineligible workloads are denied with reason codes.
- Grant usage is visible in Seal Ledger and reports.

## Handoff

Overgrant powers public-interest pools, native service surplus routing, central AI stewardship, and sponsored research/education work.

## Detailed SDS

The detailed design contract lives in [Overgrant SDS](../../sds/accounting/overgrant.md).

## Design Alignment

- Treat Overgrant as programmable resource allocation and grant authorization infrastructure, not a token drop, balance store, billing service, or speculative market.
- Keep Phase 5 focused on local/private grant primitives: source refs, eligibility, purpose tags, resource dimensions, quotas, fairness, authorization refs, and reporting.
- Gate Phase 10 cross-tenant grants and public-interest pool expansion behind explicit federation policies.
- Feed Overguard, Oversched, ORU Account Service, Seal Ledger, Wallet and Usage Center, Public-Interest Pool Service, and central AI stewardship with authorization/report refs instead of exposing private grant storage.
