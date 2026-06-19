# Overwatch Implementation Plan

## Objective

Build the event, audit, observability, incident, health, reputation, and compliance evidence layer.

## First Build Phase

Event log in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); mature observability through [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md), [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md), and [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md).

## Dependencies

- Shared event and audit schemas.
- Overgate ingress events.
- Overqueue and execution state events.

## Development Order

1. Implement append-only event storage.
2. Add request traces, state transitions, and health events.
3. Add policy decision and execution event retention.
4. Add incident records and evidence bundles.
5. Add compliance and public reporting exports in [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md).

## Contracts And Interfaces

- Event append API.
- Event query API.
- Trace and incident schemas.
- Evidence package format.
- Audit export format.

## Detailed SDS

- [Overwatch SDS](../../sds/control_plane/overwatch.md)
- [SUB BUILD PLAN #15 - Overwatch](../../build_plan/sub_build_plan_015_overwatch.md)

## Design Alignment

- Treat Overwatch as append-only evidence infrastructure, not as each service's domain source of truth.
- Store event, audit, trace, evidence bundle, incident, health, retention, export, and integrity metadata with privacy classification.
- Let services correct prior behavior by appending new evidence, never by rewriting old events.
- Filter query and evidence bundle APIs by tenant, role, data class, and evidence purpose.
- Support central AI, disputes, compliance, public reporting, and grid-resident operations through refs and redacted bundles rather than private data dumps.

## Validation

- Every mutating command can be traced from request to final state.
- Policy and dispute decisions cite stored evidence.
- Event records are append-only and tamper-evident enough for operational audit.

## Handoff

Overwatch is evidence infrastructure for Overclaim, Oververify, governance, central AI, compliance, and admin UI.
