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
