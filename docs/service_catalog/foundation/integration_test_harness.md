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

## Phase 1 Implementation Gates

- Phase 1 freezes the harness boundary as a non-production development/release gate and local test orchestrator.
- Gate classes are `smoke`, `contract_spine`, `regression`, `extended`, and `release_candidate`; each master phase inherits earlier mandatory gates before adding its current contract spine.
- Missing local stack profiles, missing schemas, unavailable services, wrong phase tags, missing manifests, non-local profiles, non-test fixture keys, and not-yet-implemented later-phase contracts must produce `blocked` test runs rather than partial false passes.
- Stable blocked reason-code families include `dependency.local_stack_unavailable`, `dependency.schema_missing`, `dependency.service_unavailable`, `dependency.phase_tag_unsupported`, `dependency.manifest_missing`, `safety.non_local_profile`, `safety.fixture_not_test_only`, and `dependency.phase_contract_not_ready`.
- A service plan may claim integration readiness only after its public APIs, events, schemas, reason codes, or state transitions have corresponding scenario manifests, fixture expectations, golden traces, and redacted artifact rules.

## Validation

- Tests are deterministic.
- A failed integration test leaves enough logs and event records for diagnosis.
- The harness can test success, retry, cancellation, timeout, policy denial, and stale lease behavior.

## Handoff

This becomes the main validation path for every service implementation plan.
