# Repository Layout Implementation Plan

## Objective

Define the physical workspace for Overrid services, packages, SDKs, CLI tools, specs, tests, and local infrastructure so early development does not sprawl.

## First Build Phase

[Phase 0: Foundation](../../build_plan/phase_00_foundation.md).

## Detailed SDS

[Repository Layout SDS](../../sds/foundation/repository_layout.md).

## Sub-Build Plan

[SUB BUILD PLAN #5 - Repository Layout](../../build_plan/sub_build_plan_005_repository_layout.md).

## Dependencies

- Current whitepaper and build plan.
- Agreement that the first implementation is modular, not many premature microservices.
- Local development and test conventions.

## Development Order

1. Create top-level folders for `services`, `packages`, `infra`, `tests`, and `docs/specs`.
2. Define ownership boundaries for control plane, node agent, SDK, CLI, schemas, and integration tests.
3. Add placeholder service contract docs for each initial subsystem.
4. Add build/test command conventions and a root task runner.
5. Document how new services are added without bypassing shared schemas.

## Contracts And Interfaces

- Folder naming convention.
- Package dependency rules.
- Service contract template.
- Test and local-stack command names.
- Workspace manifest and module record conventions.
- Generated-artifact and secret-file exclusion rules.
- New-service checklist.

## Design Alignment

- Repository layout is a governance/build-contract artifact, not a runtime service.
- Phase 0 keeps the first implementation modular but not over-split.
- Shared schemas are the required dependency path for service boundaries.

## Validation

- Fresh checkout has all expected directories.
- Root commands can discover packages and tests.
- New service stubs can be added without inventing a new layout.

## Handoff

This enables the shared schema package, local development stack, SDK, CLI, and first control-plane modules.
