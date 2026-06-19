# Overrid Tech Stack Choice

Status: accepted

Decision date: 2026-06-18

## Purpose

This document records the implementation stack choice for Overrid after reviewing the whitepaper, master SDS, build plan, and the service SDS layer.

Overrid is not a conventional SaaS backend. It is a distributed infrastructure and native application ecosystem intended to replace centralized cloud primitives with native Overrid services: resource grid, identity, policy, execution, metering, accounting, structured state, object persistence, private storage, namespace, and public utility applications.

The stack must therefore build Overrid primitives instead of depending on external cloud-style products as the real architecture.

## Primary Decision

Overrid should use a Rust-first infrastructure stack.

Core grid, control-plane, execution, storage, policy, accounting, and backbone services should be implemented in Rust. TypeScript should be used for web-facing client surfaces, generated web SDK bindings, and later operator/developer UI work, not as the core runtime for the grid.

## Architecture Rule

Third-party software may be used as libraries, embedded engines, runtime adapters, or temporary local development stubs. It must not become the product boundary for core Overrid primitives.

The public boundary must remain Overrid services and protocols:

- Overbase for structured state.
- Overstore for content-addressed object and artifact persistence.
- Overvault for sensitive/private encrypted state and secrets.
- Overqueue for durable workload state.
- Overmesh for controlled connectivity and route resolution.
- ORU, Seal Ledger, Overmeter, Overbill, Overgrant, and Overasset for accounting and rights.

## Stack By Layer

| Layer | Choice |
| --- | --- |
| Core language | Rust |
| Build/workspace | Cargo workspace with crates for services, shared contracts, test utilities, CLI, and local tooling |
| Async runtime | Tokio |
| Service framework | Axum/Tower/Hyper-style Rust HTTP services |
| Transport bootstrap | Authenticated HTTP/2 with mTLS/rustls for early seed control-plane paths |
| Mesh/federation transport | QUIC and libp2p-style peer networking for Overmesh, node discovery, artifact transfer, and later federation |
| Internal binary contracts | Protobuf where compact, versioned service/RPC/event contracts are needed |
| Human-readable contracts | Canonical JSON plus JSON Schema for Overpack manifests, policy declarations, fixtures, signed command payloads, and docs-facing examples |
| API discipline | Signed command envelopes, idempotency keys, trace ids, tenant ids, stable reason codes, schema versions, and append-only events |
| Cryptography | Ed25519 signatures, BLAKE3/content hashes, rustls, encryption-before-placement for protected data |
| Node agent | Rust Overcell agent as a supervised process |
| Execution runner | Rust Overrun with lease-bound execution, OCI/container runtime support first, Wasmtime/WASI where portable sandboxing fits |
| GPU/runtime integration | Explicit NVIDIA/ROCm runtime integration through controlled adapters, not hidden shell execution |
| Scheduler | Rust Oversched consuming queue, node, lease, policy, trust, grant, cache, and locality facts |
| Queue | Native Overqueue durable state and events, not NATS/Kafka/Redis |
| Structured state | Native Overbase boundary with append-only records, schemas, indexes, replication metadata, consistency policy, and backup/restore plans |
| Embedded storage engine | Start with an Overrid-owned storage abstraction; benchmark redb for simple local/pure-Rust storage and RocksDB-like engines only as internal shard-engine candidates |
| Object persistence | Native Overstore: content-addressed chunks, BLAKE3 hashes, upload/download grants, replication or erasure coding, repair, retention, verification evidence |
| Private storage | Native Overvault: encrypted private records, secret refs, key policy metadata, deny-by-default access checks, and audit evidence |
| Observability | Rust tracing plus OpenTelemetry-compatible spans/metrics/logs, with Overwatch as the authoritative audit/evidence layer |
| CLI | Rust CLI using generated contracts, signing, idempotency, trace propagation, and stable JSON output |
| SDKs | Generated Rust SDK first; TypeScript/web bindings from the same contracts for apps, adapters, and UI surfaces |
| Operator/developer UI | Phase 6 client surface only; TypeScript web UI is acceptable, but it must call Overgate/admin APIs and never become a privileged backdoor |
| Local development | Loopback-only local stack with deterministic reset/seed and explicit test stubs that preserve final Overrid contract shapes |

## Explicit Non-Choices

These are not the core Overrid stack:

- PostgreSQL, MySQL, MongoDB, DynamoDB, or similar systems as the core database.
- S3, MinIO, Ceph, or cloud object storage as the object layer.
- Redis, NATS, Kafka, or RabbitMQ as required queue/event backbone.
- Vault or cloud KMS as the product boundary for secrets/private state.
- Node.js/TypeScript as the core control-plane, scheduler, storage, or node-agent runtime.
- Kubernetes-first architecture.
- Conventional admin-dashboard/SaaS control-plane framing.
- Blockchain/NFT mechanics for accounting, ownership, namespace, or rights.

Some of these may be useful for comparison, import/export bridges, tests, migration adapters, or benchmark baselines. They should not define the Overrid platform boundary.

## Phase Implications

### Phase 0

Build a Rust workspace with shared contracts, schema validation, a local stack, deterministic fixtures, and integration harnesses.

Local "database", "queue", and "object store" references in the Phase 0 docs should be treated as capability placeholders. They should be implemented as Overrid-shaped local stubs or embedded engines, not as commitments to PostgreSQL, Redis, S3, or similar products.

### Phase 1

Build the control-plane skeleton in Rust:

- Overpass-lite.
- Overtenant.
- Overkey-lite.
- Overgate.
- Overregistry.
- Overwatch.
- Overqueue.

The first goal is a signed, tenant-scoped, auditable command path that reaches durable pending-work state.

### Phases 2 and 3

Build the private swarm and execution loop in Rust:

- Overcell node agent.
- Hardware discovery and benchmark runner.
- Overpack workload manifest contract.
- Oversched placement.
- Overlease reservations.
- Overrun lease-bound execution.
- Overmeter usage facts.

The first execution path should support a narrow workload class well before expanding runtime coverage.

### Phase 7

Move the backbone into protected grid-resident system workloads. At this phase, consensus, replicated state, failover, restore, and leader-election choices become critical. Raft-style Rust implementations can be evaluated here, but they should not be forced into Phase 0 before the control-plane contract is proven.

### Phase 8

Build the native data, storage, and namespace platform:

- Overbase for structured state.
- Overstore for object/artifact persistence.
- Overvault for encrypted private records and secrets.
- Universal Namespace Service.
- Overasset bindings to storage, namespace, routes, and entitlements.
- Overmesh route resolution.

This is where Overrid starts replacing database, object storage, vault, and namespace products as native infrastructure.

## Decision Rationale

Rust is the best fit for the core because Overrid needs long-running infrastructure processes, node agents, storage engines, networking, sandbox execution, cryptography, low overhead, strong correctness pressure, and cross-platform deployment. The project is closer to a distributed operating substrate than a web app.

TypeScript remains useful for browser/client surfaces, generated SDK bindings, and the later operator/developer UI. It should not drive the core grid, scheduler, storage, node-agent, or accounting layers.

The main architectural goal is to avoid accidental dependency inversion. Overrid can use internal engines and adapters, but Overbase, Overstore, Overvault, Overqueue, Overmesh, Seal Ledger, and related services must remain the actual primitives exposed to apps, native services, adapters, and operators.

## Source Documents

- [Master SDS](sds/master_sds.md)
- [Master Build Plan](build_plan/master_plan.md)
- [Phase 0: Foundation](build_plan/phase_00_foundation.md)
- [Phase 1: Control-Plane Skeleton](build_plan/phase_01_control_plane_skeleton.md)
- [Phase 8: Data, Storage, and Namespace Platform](build_plan/phase_08_data_storage_namespace_platform.md)
- [Overbase SDS](sds/data_storage_namespace/overbase.md)
- [Overstore SDS](sds/data_storage_namespace/overstore.md)
- [Overvault SDS](sds/data_storage_namespace/overvault.md)
- [Overcell SDS](sds/execution_scheduling/overcell.md)
- [Overrun SDS](sds/execution_scheduling/overrun.md)
- [Oversched SDS](sds/execution_scheduling/oversched.md)
- [Overpack SDS](sds/execution_scheduling/overpack.md)
- [Overmesh SDS](sds/execution_scheduling/overmesh.md)
- [Shared Schema Package SDS](sds/foundation/shared_schema_package.md)
- [Local Development Stack SDS](sds/foundation/local_development_stack.md)

## Open Engineering Questions

- Exact schema/code-generation toolchain for Protobuf plus JSON Schema.
- First embedded storage engine for Overbase v0.
- First chunking, erasure-coding, and repair profile for Overstore v0.
- Exact QUIC/libp2p boundary for Overmesh v0.
- Initial OCI versus WASI workload split for Overrun.
- When to introduce consensus/leader-election mechanics for grid-resident backbone services.

These are implementation choices inside the accepted Rust-first/native-Overrid direction, not reasons to revert to conventional cloud primitives.
