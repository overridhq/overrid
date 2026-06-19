# Overpack Implementation Plan

## Objective

Define workload and application package manifests for commands, services, containers, WASI modules, models, datasets, routes, storage, security, billing, and scaling intent.

## First Build Phase

Workload manifest in [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md); deployment platform in [Phase 9: Overpack Deployment Platform](../../build_plan/phase_09_overpack_deployment_platform.md).

## Dependencies

- Shared manifest schemas.
- Overregistry.
- Overguard.
- Package validator.
- Overrun.

## Development Order

1. Define Overpack v0 workload manifest.
2. Add artifact hashes, signatures, runtime contract, egress policy, and resource cards.
3. Add SBOM and dependency locks.
4. Add application-intent manifest in [Phase 9: Overpack Deployment Platform](../../build_plan/phase_09_overpack_deployment_platform.md).
5. Add compatibility checks for AI-generated package/deployment proposals.

## Contracts And Interfaces

- Workload manifest schema.
- Application-intent manifest schema.
- Package provenance record.
- Runtime contract and permission declarations.

## Detailed SDS

The detailed design contract lives in [Overpack SDS](../../sds/execution_scheduling/overpack.md).

- [SUB BUILD PLAN #24 - Overpack](../../build_plan/sub_build_plan_024_overpack.md)

## Design Alignment

- Treat Overpack as the manifest/package contract, not the runner, scheduler, storage service, or deployment planner.
- Start with a strict Phase 3 workload manifest that declares runtime, inputs, outputs, resource card, data class, egress, secrets, timeout, retry, artifacts, hashes, signatures, and provenance.
- Register accepted immutable manifest versions through Overregistry and require new versions for changes.
- Expand to Phase 9 application-intent manifests without weakening the execution-time package integrity and policy contracts.

## Validation

- Invalid packages are rejected before execution.
- Overrun can verify package integrity from the manifest.
- Deployment planner can provision resources from one signed application manifest.

## Handoff

Overpack feeds Overregistry, Overguard, Overrun, deployment planner, and package validator.
