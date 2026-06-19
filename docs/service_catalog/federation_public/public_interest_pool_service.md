# Public-Interest Pool Service Implementation Plan

## Objective

Manage donated or sponsored resources for approved public-interest work.

## First Build Phase

[Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md).

## Dependencies

- Overgrant.
- ORU account service.
- Seal Ledger.
- Purpose tag registry.
- Overguard.

## Development Order

1. Define pool accounts, contributed resources, eligible grantees, quotas, and fairness rules.
2. Add grant allocation and usage reporting.
3. Add abuse throttles.
4. Add public-interest outcome report hooks.
5. Add native app surplus routing integration.

## Contracts And Interfaces

- Pool account schema.
- Grant allocation record.
- Eligibility and quota API.
- Public-interest report format.

## Validation

- Eligible grantees can use pool capacity within quota.
- Ineligible usage is denied with reason codes.
- Reports reconcile with Seal Ledger usage.

## Handoff

Public-interest pools turn Overrid surplus and donated capacity into accountable support for useful projects.

## Detailed SDS

The detailed design contract lives in [Public-Interest Pool Service SDS](../../sds/federation_public/public_interest_pool_service.md).

## Design Alignment

- Treat Public-Interest Pool Service as purpose-aware allocation and reporting infrastructure: pool definitions, contributions, eligible grantees, quotas, fairness windows, allocation requests, abuse throttles, usage reports, and outcome refs.
- Keep purpose tag definition, grant authorization, ORU balances, Seal Ledger entries, scheduling, execution, and stewardship priority decisions in their owning services.
- Require verified purpose tags, Overguard eligibility, Overgrant handoff, usage/accounting refs, and public-safe redaction before allocation/reporting.
- Feed Overgrant, Overguard, Overmeter, ORU Account Service, Seal Ledger, Federation Template Service, Fraud Control Service, stewardship reporting, central AI stewardship mechanisms, SDK, CLI, and admin UI with pool refs.
