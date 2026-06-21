# Packages Directory Contract

`packages/` owns shared buildable packages used by Overrid services, tools, SDKs, local development, and integration validation.

Required Phase 0 package contracts:

- `packages/schemas`: language-neutral contract authority. Canonical JSON Schema owns human-readable commands, manifests, fixtures, signed payloads, view models, events, audit records, and docs-facing examples.
- `packages/sdk`: Rust SDK first, generated or validated from shared contracts and used by tools and client bindings.
- `packages/cli`: Rust developer/operator CLI with stable JSON output, signing/idempotency/trace propagation, and SDK/Overgate routing.

Implemented companion package contracts:

- `packages/admin_ui_shell`: TypeScript operator/developer UI shell package for later client surfaces only.
- `packages/integration_harness`: Rust integration validation gate package for cross-service scenarios and phase evidence.
- `packages/local_stack`: Rust local development helper package for loopback-only stack records, reset/seed behavior, and deterministic local fixtures.

Rules:

- Generated code is never the source of truth.
- TypeScript is allowed for web/client bindings and UI packages, not core grid/runtime authority.
- Package additions require owner-layer, master-phase, contract refs, test targets, and docs links before they are accepted.
- Repository Layout Phase 6 dependency direction groups keep `contracts` as the shared boundary path, `sdk` and `cli` as consumers, and `local_stack`, `integration_harness`, `local_infra`, fixture writers, and integration artifacts as local/test-only helpers.
- Runtime-facing modules must not import local/test-only helpers or docs files as executable configuration; boundary payloads must cite `packages/schemas`, `overrid-contracts`, or `docs/specs`.
- Repository Layout Phase 7 generated-output rules keep package caches, generated projections, generated docs/types, coverage, logs, fixture outputs, and temporary object chunks ignored by default.
- Approved generated projection files must be explicitly listed in `overrid.workspace.toml`; generated projections remain non-authoritative consumers of canonical schema sources.
- Package validation artifacts must report secret, key, token, signature, private payload, encrypted content, and fixture-credential findings by safe refs only.
- Repository Layout Phase 8 module lifecycle rules require new packages to move through `proposed`, `scaffolded`, `contracted`, `wired`, `validated`, and `accepted` states, with `deprecated` and `removed` states for retirement.
- Accepted package modules must keep service/module contracts, schema refs or no-public-contract reasons, test targets, documentation links, local-stack participation metadata, and validation evidence aligned with `docs/specs/new_module_checklist.md`.
- Repository Layout Phase 9 foundation integration metadata exposes local-stack discovery, harness discovery, clean-checkout CI sequencing, validation evidence, and artifact-consumer rules through `overrid.workspace.toml`; package roots remain consumers of those build/CI records, not runtime configuration sources.
