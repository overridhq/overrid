# Admin and Developer UI Implementation Plan

## Objective

Provide operational visibility into tenants, identities, nodes, jobs, policy decisions, usage, disputes, and system health.

## First Build Phase

[Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Detailed SDS

[Admin and Developer UI SDS](../../sds/foundation/admin_developer_ui.md).

## Detailed Build Plan

[SUB BUILD PLAN #1 - Admin and Developer UI](../../build_plan/sub_build_plan_001_admin_developer_ui.md).

## Dependencies

- Overgate admin APIs.
- Overwatch event log.
- Overmeter usage records.
- Seal Ledger views.
- Overclaim dispute records.

## Development Order

1. Build read-only tenant, identity, node, and job views.
2. Add queue, lease, and execution state timelines.
3. Add policy decision and reason-code explorer.
4. Add usage, ORU, receipt, hold, and dispute views.
5. Add carefully scoped admin actions with signed operator audit.

## Contracts And Interfaces

- Overgate admin API.
- Overwatch event query API.
- Seal Ledger account and workload views.
- Overclaim dispute views.
- Signed admin action envelope routed through Overgate.
- Read-only view models with server-side tenant, role, and data-class filtering.
- Job timeline view joining request, queue, lease, execution, policy, usage, receipt, and dispute refs by trace id.

## Design Alignment

- The UI is an Overrid client surface, not a privileged control-plane service.
- The first build is read-only. Mutating admin actions are added only after signed operator action, idempotency, expected-current-state, and Overwatch receipt contracts exist.
- The UI must not read private service storage directly.

## Validation

- Operator can trace a job from request through execution and accounting.
- Policy denials show reason codes and input facts.
- Admin actions emit signed audit events.

## Handoff

This becomes the main operational surface for [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md) product integrations and later backbone migration.
