# Central AI Stewardship Interface Implementation Plan

## Objective

Build the public and administrative interface for central AI stewardship, grants, donations, fraud evidence, system health, appeals, and governance reports.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) and [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md).

## Dependencies

- Central AI service.
- Overgrant.
- Seal Ledger.
- Overwatch.
- Overclaim.
- Stewardship reporting.

## Development Order

1. Add public-interest project and grant views.
2. Add native app surplus routing views.
3. Add fraud and abuse evidence summaries.
4. Add appeals and dispute visibility.
5. Add system health and public reporting exports.

## Contracts And Interfaces

- Stewardship dashboard API.
- Recommendation record view.
- Appeal/dispute view.
- Public report refs.

## Validation

- Recommendations cite evidence.
- Private user data is not exposed in public reports.
- Appeals and disputes are visible where policy allows.

## Handoff

This interface makes central AI stewardship inspectable and accountable.

## Detailed SDS

The detailed design contract is [Central AI Stewardship Interface SDS](../../sds/native_apps/central_ai_stewardship_interface.md).

## Sub-Build Plan

- [SUB BUILD PLAN #68 - Central AI Stewardship Interface](../../build_plan/sub_build_plan_068_central_ai_stewardship_interface.md)

## Design Alignment

- Treat the interface as a redacted view and signed-action surface, not as Central AI Service or a direct mutation path for funds, grants, fraud cases, disputes, or reports.
- Require recommendation views, work queues, review action envelopes, public-interest project views, surplus-routing views, fraud evidence summaries, appeal views, report publication views, and usage refs.
- Route every mutating stewardship action to the owning service with evidence refs, role checks, reason codes, appeal paths, and Overwatch audit.
- Public reporting must be useful without exposing private evidence or speculative financial assumptions.
