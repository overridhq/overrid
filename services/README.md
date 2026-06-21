# Services Directory Contract

`services/` is the source-controlled root for Overrid runtime service code when an owning build phase creates it. It starts with narrow Phase 0 contracts for the initial control-plane process and node-agent boundary; it is not a dumping ground for every future whitepaper concept.

Allowed Phase 0 children:

- `services/control-plane`: initial modular Rust API and worker process boundary.
- `services/node-agent`: Overcell node agent and simulator boundary.

Rules:

- Service code must remain Rust-first unless a later SDS explicitly defines a client/UI surface.
- Shared boundary objects must flow through `packages/schemas` and generated or validated bindings.
- Future service folders require SDS, service-catalog, build-plan, specs, tests, and layout-governance updates before acceptance.
- This directory must not become hidden service discovery, production config, deployment orchestration, or a shortcut around service contracts.
