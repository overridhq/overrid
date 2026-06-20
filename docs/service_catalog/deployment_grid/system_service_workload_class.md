# System-Service Workload Class Implementation Plan

## Objective

Define the protected workload class that allows Overrid backbone services to run on trusted grid nodes.

## First Build Phase

[Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md).

## Dependencies

- Overguard workload class policy.
- Oververify trusted node evidence.
- Oversched placement.
- Overwatch audit.

## Development Order

1. Define system-service class schema and policy.
2. Mark eligible services and node requirements.
3. Add trusted placement restrictions.
4. Add stricter logging, backup, update, and rollback requirements.
5. Add break-glass and signed operator action controls.

## Contracts And Interfaces

- System-service workload class.
- Eligibility policy.
- Placement requirement contract.
- Signed operator action event.

## Validation

- Unknown public nodes cannot host system services.
- System workload placement is replayable from trust evidence.
- Maintenance and rollback actions are audited.

## Handoff

This enables grid-resident backbone migration and protected Overrid self-hosting.

## Detailed SDS

The detailed design contract lives in [System-Service Workload Class SDS](../../sds/deployment_grid/system_service_workload_class.md).

## Sub-Build Plan

Implementation sequencing lives in [SUB BUILD PLAN #51 - System-Service Workload Class](../../build_plan/sub_build_plan_051_system_service_workload_class.md).

## Design Alignment

- Treat System-Service Workload Class as the versioned eligibility and guardrail contract for protected backbone placement, not as scheduler, runner, package validator, release service, or failover system.
- Hard-deny unknown public nodes and public sandbox providers for system-service workloads.
- Require verified operator/node evidence, package command contracts, backup/restore/failover controls, signed operator actions, and Overwatch evidence before eligibility advances.
- Feed Overguard, Oversched, Deployment Planner, Release Strategy Service, Backup and Restore Service, Failover and Recovery Coordinator, and Grid-Resident Service Packager with replayable class/evaluation refs.
