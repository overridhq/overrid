# Local Infrastructure Contract

`infra/local` describes the loopback-only local development stack shape for Overrid.

Tracked source contracts:

- `profiles/`: local development profiles and profile metadata.
- `service-definitions/`: Overrid-shaped service definitions and dependency-order notes.

Ignored local-only markers:

- `state/`: local durable state and resettable volumes.
- `job-tables/`: local durable job-table experiments.
- `artifacts/`: object/artifact stub outputs and temporary chunks.

Rules:

- Keep local state explicitly test/development scoped and resettable.
- Use Overrid-shaped local durable state, job tables, artifact stubs, service definitions, and profiles.
- Do not make PostgreSQL, Redis, S3, MinIO, Kafka, NATS, Vault, or cloud KMS the product boundary.
- Never commit raw secrets, private keys, tokens, local runtime logs, or generated artifacts.
- Repository Layout Phase 7 requires `state/`, `job-tables/`, and `artifacts/` to remain ignored marker-gated local-state paths.
- Reset and validation artifacts must redact secrets, keys, tokens, signatures, private payloads, encrypted content, and fixture credentials.
