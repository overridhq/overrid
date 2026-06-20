# Compliance Boundary Service Implementation Plan

## Objective

Keep payment, custody-like, privacy, child-safety, regulated-workload, jurisdiction, and payout boundaries explicit.

## First Build Phase

[Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md).

## Dependencies

- Overguard.
- Overbill.
- Overvault.
- Overtenant.
- Overwatch.

## Development Order

1. Define jurisdiction, payment, custody, retention, deletion, regulated workload, and payout boundary records.
2. Feed compliance facts into Overguard.
3. Add compliance export and audit refs.
4. Add isolation rules for high-compliance workloads.
5. Add update workflow for jurisdiction-specific changes.

## Contracts And Interfaces

- Compliance rule schema.
- Boundary marker refs.
- Export format.
- Policy fact contract.

## Validation

- Regulated workloads are isolated by policy.
- Payment/custody boundaries are explicit in billing flows.
- Compliance exports cite stored evidence.

## Handoff

Compliance boundaries protect Overrid as native apps, payments, and public participation grow.

## Detailed SDS

The design contract is expanded in [Compliance Boundary Service SDS](../../sds/governance_ops/compliance_boundary_service.md).

## Sub-Build Plan

- [SUB BUILD PLAN #76 - Compliance Boundary Service](../../build_plan/sub_build_plan_076_compliance_boundary_service.md)

## SDS Design Alignment

- Treat the service as the versioned compliance fact and boundary-marker authority, not as legal advice, payment processing, policy enforcement, or custody.
- Build concrete records for rulesets, jurisdiction profiles, boundary markers, evaluations, fact bundles, exceptions, updates, exports, and replay bundles.
- Feed signed boundary facts to Overguard and owning services through explicit APIs while preserving denial, redaction, exception, and jurisdiction-update evidence.
- Keep high-compliance markers narrow so low-risk utility flows stay possible and regulated flows are isolated by policy.
