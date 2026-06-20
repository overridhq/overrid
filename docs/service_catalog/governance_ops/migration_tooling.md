# Migration Tooling Implementation Plan

## Objective

Move services, tenants, data, routes, and workloads between seed, private, trusted federation, and grid-resident deployments.

## First Build Phase

[Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md), with grid migration tools starting in [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md).

## Dependencies

- Deployment planner.
- Overbase.
- Overstore.
- Overmesh.
- Seal Ledger.
- Overwatch.

## Development Order

1. Add service migration and route rebinding tools.
2. Add tenant workload and data store migration.
3. Add event replay and ledger state verification.
4. Add post-migration integrity checks.
5. Add rollback and audit records.

## Contracts And Interfaces

- Migration plan schema.
- Migration step records.
- Integrity check report.
- Rollback refs.

## Validation

- Services can migrate from seed hardware to grid nodes.
- Ledger and route integrity are verified after migration.
- Failed migration can roll back or resume safely.

## Handoff

Migration tooling supports grid independence, customer swarms, and long-term platform evolution.

## Detailed SDS

The design contract is expanded in [Migration Tooling SDS](../../sds/governance_ops/migration_tooling.md).

## Sub-Build Plan

- [SUB BUILD PLAN #78 - Migration Tooling](../../build_plan/sub_build_plan_078_migration_tooling.md)

## SDS Design Alignment

- Treat migration as planned, resumable, checkpointed coordination across owning services, not direct edits to stores, routes, ledgers, queues, or vaults.
- Build plan, inventory, preflight, graph, step, checkpoint, cutover, integrity-check, rollback, and replay records.
- Require backup/restore readiness, destination eligibility, route ownership, ledger checkpoints, queue drain, vault grants, policy decisions, and cutover approval before irreversible moves.
- Use the service to prove founder hardware can leave the normal path only after migration, restore, failover, rollback, and integrity drills pass.
