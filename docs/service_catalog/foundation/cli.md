# CLI Implementation Plan

## Objective

Let developers and operators use Overrid from the terminal without manually calling internal APIs.

## First Build Phase

Basic commands in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); hardened CLI in [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Detailed SDS

[CLI SDS](../../sds/foundation/cli.md).

## Detailed Build Plan

[SUB BUILD PLAN #2 - CLI](../../build_plan/sub_build_plan_002_cli.md).

## Dependencies

- SDK.
- Overgate.
- Overkey credential enrollment.
- Shared schemas.

## Development Order

1. Add login or credential enrollment.
2. Add tenant, identity, key, and manifest commands.
3. Add workload submit, inspect, cancel, result, and logs commands.
4. Add node registration and node health commands.
5. Add usage, receipt, policy dry-run, and package validation commands.
6. Add admin-safe JSON output for automation.

## Contracts And Interfaces

- Human-readable command output.
- Machine-readable `--json` output.
- Config and credential storage rules.
- Exit-code conventions.
- Signed command envelope construction through the SDK.
- Local profile and credential-reference rules.
- Stable reason-code, retry-class, trace-id, and audit-ref output.

## Design Alignment

- The CLI is a terminal client, not a private service interface.
- Every platform call must go through the SDK and Overgate path.
- Local/test fixture credentials must be isolated from seed or production-like endpoints.

## Validation

- CLI can complete the [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md) synthetic workload path.
- CLI can submit and inspect a [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md) real private job.
- CLI output includes trace ids for failures.

## Handoff

This becomes the operator and developer workflow until richer admin UI surfaces mature.
