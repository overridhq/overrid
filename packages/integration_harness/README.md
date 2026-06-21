# Integration Harness Package Contract

`packages/integration_harness` owns the Rust integration validation gate for Overrid phase evidence. It is a Cargo workspace member for test harness logic, scenario execution helpers, assertion records, artifact metadata, and phase-gate validation primitives.

Ownership metadata:

- Owner layer: quality, contract, and integration validation.
- First master phase: Phase 0 foundation.
- Source SDS: [Integration Test Harness SDS](../../docs/sds/foundation/integration_test_harness.md).
- Source build plan: [SUB BUILD PLAN #3 - Integration Test Harness](../../docs/build_plan/sub_build_plan_003_integration_test_harness.md).
- Contract authority: `packages/schemas/overrid_contracts` and `docs/specs` records as they are introduced.
- Test target: `cargo test -p overrid-integration-harness`.

Rules:

- This package is a non-production validation gate, not a runtime service or hidden service registry.
- It must exercise services through public contracts, the CLI, SDK, Overgate-compatible surfaces, and local test hooks only.
- Scenario manifests, golden traces, and failure artifacts must remain deterministic and secret-free.
- Generated artifacts and run output belong in ignored integration artifact paths unless a later acceptance path promotes redacted fixtures.
