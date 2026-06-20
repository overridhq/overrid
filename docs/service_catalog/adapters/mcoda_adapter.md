# Mcoda Adapter Implementation Plan

## Objective

Run Mcoda agent workloads through Overrid resource, policy, and metering rails.

## First Build Phase

[Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Dependencies

- Overpack.
- Overqueue.
- Oversched.
- Overrun.
- Overmeter.
- AI gateway router.

## Development Order

1. Define agent task package format.
2. Add model/resource selection and tool-use boundary declarations.
3. Submit agent workloads through Overrid.
4. Capture result, failure reason, and usage.
5. Add budget and policy checks.

## Contracts And Interfaces

- Agent task manifest.
- Tool boundary declaration.
- Result schema.
- Usage report fields.

## Validation

- Mcoda can run a real task through Overrid.
- Tool boundaries are visible to policy.
- Usage and failure reasons are captured.

## Handoff

Mcoda adapter proves agent execution and model routing on Overrid resources.

## Detailed SDS

The detailed design contract is [Mcoda Adapter SDS](../../sds/adapters/mcoda_adapter.md).

## Sub-Build Plan

- [SUB BUILD PLAN #66 - Mcoda Adapter](../../build_plan/sub_build_plan_066_mcoda_adapter.md)

## Design Alignment

- Treat the adapter as the Mcoda-to-Overrid workload translation boundary, not as the Mcoda agent runtime or marketplace.
- Require explicit agent task manifests, tool boundary declarations, context access plans, route refs, phase records, result refs, failure refs, and usage refs.
- Route model/resource selection through AI Gateway Router and normal execution services instead of hardcoding one model, provider, or privileged runtime.
- Enforce tool grants, policy decisions, redaction, replay, and wallet-visible usage before the adapter is considered production-ready.
