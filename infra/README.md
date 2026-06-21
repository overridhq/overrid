# Infrastructure Directory Contract

`infra/` owns local and later deployment-support artifacts that describe Overrid-shaped infrastructure behavior without making conventional cloud products the platform boundary.

Phase 0 starts with:

- `infra/local`: loopback-only local development stack contracts, source-controlled profiles/service definitions, and ignored local state markers.

Rules:

- Local infrastructure may use embedded engines, stubs, and adapters for development, but public boundaries remain Overrid services and protocols.
- Do not encode PostgreSQL, Redis, S3, MinIO, Kafka, NATS, Vault, Kubernetes, or cloud accounts as required Overrid primitives.
- Secret-bearing files, local state, artifacts, logs, and generated outputs stay ignored by default.
