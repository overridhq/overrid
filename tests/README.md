# Tests Directory Contract

`tests/` owns repo-level tests that cross package or service boundaries.

Phase 0 starts with:

- `tests/integration`: cross-service scenarios, scenario manifests, and ignored integration artifacts.

Rules:

- Package-local unit tests stay inside their Rust crate or package.
- Cross-service behavior belongs in `tests/integration` and should use shared schemas, local stack profiles, and stable scenario manifests.
- Runtime services must not import tests or test-only helpers.
