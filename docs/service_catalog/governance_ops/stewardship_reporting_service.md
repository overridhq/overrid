# Stewardship Reporting Service Implementation Plan

## Objective

Publish structured system, grant, surplus, fraud, security, incident, and protocol reports without exposing private user data.

## First Build Phase

[Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md).

## Dependencies

- Overwatch.
- Seal Ledger.
- Overgrant.
- Overclaim.
- Central AI service.

## Development Order

1. Define report types and data minimization rules.
2. Add system health and grant usage reports.
3. Add native service surplus routing reports.
4. Add abuse/fraud statistics and incident summaries.
5. Add public export pipeline.

## Contracts And Interfaces

- Report schema.
- Privacy redaction rules.
- Public export format.
- Evidence refs.

## Validation

- Reports reconcile with ledger and event data.
- Private user data is excluded or redacted.
- Report generation is repeatable.

## Handoff

Stewardship reporting makes the ecosystem accountable to users and public-interest stakeholders.

## Detailed SDS

The design contract is expanded in [Stewardship Reporting Service SDS](../../sds/governance_ops/stewardship_reporting_service.md).

## SDS Design Alignment

- Treat reporting as evidence aggregation, redaction, review, publication, correction, retraction, and replay infrastructure, not as grant, ledger, dispute, fraud, or policy authority.
- Build report templates, periods, build jobs, source inventories, metric snapshots, evidence manifests, redaction profiles, review records, artifacts, and correction/retraction records.
- Reconcile reports with Overwatch, Seal Ledger, Overgrant, Overclaim, Central AI, PIP, compliance, incident, migration, and security-review refs before publication.
- Keep public reports accountable and repeatable without exposing private users, encrypted context, secrets, payment details, fraud internals, or security-sensitive evidence.
