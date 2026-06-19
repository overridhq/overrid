# Integration Test Harness Implementation Plan

## Objective

Prove Overrid behavior across service boundaries instead of relying only on unit tests.

## First Build Phase

[Phase 0: Foundation](../../build_plan/phase_00_foundation.md).

## Detailed SDS

[Integration Test Harness SDS](../../sds/foundation/integration_test_harness.md).

## Sub-Build Plan

[SUB BUILD PLAN #3 - Integration Test Harness](../../build_plan/sub_build_plan_003_integration_test_harness.md).

## Dependencies

- Local development stack.
- Shared schema package.
- Deterministic fixtures.

## Development Order

1. Build a test runner that starts and resets the local stack.
2. Add fixture builders for tenants, identities, keys, nodes, workloads, packages, and ledger accounts.
3. Add golden event trace support.
4. Add command-path tests for signed requests and idempotency.
5. Expand into queue, lease, execution, metering, policy, and ledger tests as phases progress.

## Contracts And Interfaces

- Test fixture API.
- Stack lifecycle hooks.
- Golden trace format.
- Failure assertion helpers.
- Scenario manifest and test run record schemas.
- Redacted artifact bundle format.
- Phase-gated scenario tags.

## Design Alignment

- The harness is a development and release gate, not a production service.
- It should assert behavior through public contracts and local test hooks.
- Direct storage inspection is limited to reset/diagnostic behavior, not normal service assertions.

## Validation

- Tests are deterministic.
- A failed integration test leaves enough logs and event records for diagnosis.
- The harness can test success, retry, cancellation, timeout, policy denial, and stale lease behavior.

## Handoff

This becomes the main validation path for every service implementation plan.
