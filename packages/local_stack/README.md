# Local Stack Package Contract

`packages/local_stack` owns Rust helper types for the loopback-only Overrid local development stack. It is a Cargo workspace member for local stack records, deterministic reset/seed planning, health snapshots, fixture references, and local artifact metadata.

Ownership metadata:

- Owner layer: local execution and developer environment.
- First master phase: Phase 0 foundation.
- Source SDS: [Local Development Stack SDS](../../docs/sds/foundation/local_development_stack.md).
- Source build plan: [SUB BUILD PLAN #4 - Local Development Stack](../../docs/build_plan/sub_build_plan_004_local_development_stack.md).
- Contract authority: `packages/schemas/overrid_contracts` and `infra/local` source-controlled contracts.
- Test target: `cargo test -p overrid-local-stack`.

Rules:

- This package must model Overrid-shaped local state, job tables, artifact stubs, service definitions, profiles, and deterministic fixtures.
- It must not introduce PostgreSQL, Redis, S3, MinIO, Kafka, NATS, Vault, external cloud accounts, or paid SaaS products as required platform boundaries.
- Local records, generated secrets, logs, chunks, job tables, and artifact output belong in ignored local-state paths.
- Local-only helpers must not bypass shared schemas or become production configuration loading.
