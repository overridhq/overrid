# Protocol Improvement Proposal Registry Implementation Plan

## Objective

Make Overrid protocol changes evidence-backed, reviewable, migratable, and traceable.

## First Build Phase

[Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md).

## Dependencies

- Overrid protocol core.
- Overregistry.
- Stewardship governance.
- Security review tracker.

## Development Order

1. Define proposal schema: motivation, specification, security, privacy, economic impact, compatibility, migration, and rollback.
2. Add proposal lifecycle states.
3. Add review and acceptance records.
4. Link accepted PIPs to schema/service versions.
5. Add public reporting hooks.

## Contracts And Interfaces

- PIP record schema.
- Review state API.
- Acceptance and rollback refs.
- Migration plan refs.

## Validation

- Non-trivial protocol changes have a PIP.
- Accepted PIPs link to implementation and migration evidence.
- Rollback plans are explicit.

## Handoff

PIP registry becomes the long-term governance path for protocol evolution.

## Detailed SDS

The design contract is expanded in [Protocol Improvement Proposal Registry SDS](../../sds/governance_ops/pip_registry.md).

## SDS Design Alignment

- Treat the registry as append-only protocol and service-contract governance records, not implementation, deployment, token voting, or hidden private governance.
- Build PIP records, versions, sections, review assignments, findings, decisions, implementation links, migration/rollback refs, supersession records, public views, and replay bundles.
- Require affected-domain reviews for security, privacy, compliance, accounting, migration, and stewardship impacts before acceptance.
- Link accepted PIPs to implementation, schema/protocol versions, tests, migration, rollback, release, SDS/catalog updates, and public-report refs.
