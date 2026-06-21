# Control Plane Service Contract

`services/control-plane` is reserved for the initial modular Overrid API and worker process. The default shape is one modular Rust process through master Phase 3 with internal crates/modules for domains such as Overpass-lite, Overtenant, Overkey-lite, Overgate, Overregistry, Overwatch, and Overqueue.

Ownership rules:

- Keep the process boundary modular but not split into deployable microservices without measured operational or security pressure and updated SDS evidence.
- Public command, API, event, audit, and error shapes must cite `packages/schemas` and `docs/specs` contracts before service logic consumes them.
- Local development behavior must use `infra/local` contracts and stay loopback-only.
- Tests that span services belong under `tests/integration`; package-local tests stay with their crate/module.
- Repository Layout Phase 6 preserves this as one modular Rust process through master Phase 3 by default. Future splits require measured API-load, failure-isolation, security-boundary, operational, or grid-resident backbone pressure plus updated SDS, service catalog, build-plan crosswalk, docs/specs contracts, and validation evidence.
- Runtime-facing control-plane code must not depend on integration harness internals, local stack internals, fixture writers, integration artifacts, local simulator internals, or docs files as executable configuration.

This scaffold does not create runtime behavior. It records the Phase 2 path contract for later Rust implementation.
