SDS #7

# Shared Schema Package SDS

## Purpose

Create the canonical schemas, generated types, validators, fixtures, and compatibility rules used across Overrid service boundaries.

The shared schema package is a build-time and runtime library, not a deployed microservice. It is the contract source for control-plane APIs, worker messages, event envelopes, SDK objects, CLI input, audit records, queue records, usage records, ORU objects, Seal Ledger refs, and native-service integration payloads.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [shared_schema_package.md](../../service_catalog/foundation/shared_schema_package.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Sub-build plan | [SUB BUILD PLAN #7 - Shared Schema Package](../../build_plan/sub_build_plan_007_shared_schema_package.md) |
| Build phase alignment | [Phase 0: Foundation](../../build_plan/phase_00_foundation.md) |

## Service Family

- Family: Foundation and developer tooling.
- Owning layer: Platform contract foundation.
- Runtime shape: versioned package with schema definitions, generated types, validators, fixtures, and compatibility metadata.
- Primary data scope: protocol objects and validation rules, not service-owned production state.
- First build phase from service plan: [Phase 0: Foundation](../../build_plan/phase_00_foundation.md).

## Problem Statement

Overrid has many services that must agree on identity, tenant, command, workload, event, usage, ledger, policy, and dispute objects. Without a single schema package, each service will invent incompatible fields, weak validation, and divergent state names. That would break SDK generation, mobile-service integration, audit replay, and the user-owned infrastructure model. The package must make contract drift visible before runtime.

## Goals

- Define the canonical object families required by Phase 0 and Phase 1.
- Generate types and validators for services, SDK, CLI, workers, node agents, and tests.
- Enforce common envelope fields for mutating commands and externally visible records.
- Provide fixtures that prove valid and invalid payload behavior deterministically.
- Version schemas with explicit compatibility and deprecation metadata.
- Keep ORU, Seal Ledger, Overasset, native service, AI assistant, Docdex RAG, and mobile-service objects on the same contract path as core control-plane objects.

## Non-Goals

- Do not make the package a runtime registry or policy engine.
- Do not store production records or emit authoritative platform events.
- Do not let individual services fork private schemas for public objects.
- Do not hide breaking changes behind permissive validation.
- Do not encode charge tables, customer-count assumptions, or economic projections.
- Do not use ad hoc string parsing when typed schemas can describe the object.

## Primary Actors And Clients

- Service implementers defining APIs and events.
- SDK, CLI, and admin UI builders generating typed clients.
- Worker and node-agent implementers exchanging queue, lease, manifest, and result objects.
- Native app and mobile-service builders needing stable public contracts.
- Integration-test harnesses generating valid and invalid fixtures.
- Operators reviewing schema versions during migrations and incident response.

## Dependencies

- Repository layout for package location, code generation outputs, and validation fixtures.
- Initial protocol object list from [master_services.md](../../service_catalog/master_services.md).
- Build-plan API and event conventions from [Phase 0: Foundation](../../build_plan/phase_00_foundation.md).
- Service SDS files for service-specific object ownership and state names.
- Schema versioning and deprecation policy maintained with the package.

## Owned Responsibilities

Shared Schema Package owns:

- Canonical schema source files.
- Generated types for approved implementation languages.
- Runtime validators and parse helpers.
- Fixture builders for integration and service contract tests.
- Stable error shape definitions and reason-code typing.
- Common envelope definitions for command, event, audit, usage, and ledger objects.
- Compatibility metadata for additive, deprecated, and breaking changes.
- Documentation generated from schemas for service teams and API consumers.

It does not own service state transitions, policy decisions, audit finality, or accounting finality.

## Phase-Gate Boundary Decisions

SDS #7 Phase 1 freezes the Shared Schema Package as a versioned contract package and library. It is not a deployed microservice, not a runtime registry, not a policy engine, not an audit authority, not an accounting authority, and not a production record store.

Required Phase 1 gate states:

- `attached`: the SDS, service catalog, master build plan, crosswalk, tech-stack decision, package docs, and sub-build plan point to each other.
- `boundary_frozen`: schema authority stays in canonical schema sources, Rust-first validation, fixtures, generated docs, compatibility reports, and generated/projection outputs.
- `phase_0_authority`: the first build point remains Phase 0.
- `downstream_phase_gated`: later schema families expand only through owning service phase gates.
- `resolved_decision_carried`: resolved SDS decisions remain closed unless a later formal plan changes them.
- `metadata_required`: draft schema modules cannot release without owner, status, privacy class, compatibility metadata, and consumer notes.

Resolved decisions:

- Canonical JSON plus JSON Schema remains the docs-facing, fixture-facing, command, manifest, signed-payload, and public API source of truth.
- Rust generation, Rust validators, Rust fixture checks, and Rust docs/diff tooling are first.
- TypeScript/web projections are generated second from the same contracts and must never become the source of truth.
- Protobuf is allowed only for compact internal service/RPC/event contracts and never as a Protobuf-only public object definition.
- Security-sensitive and accounting-sensitive families use strict unknown-field rejection by default.
- Extension maps are allowed only for explicitly named low-risk metadata surfaces with typed values, namespace prefixes, privacy class, and compatibility class.
- Current-plus-previous stable major support applies once external consumers depend on the package.
- Authority-sensitive schema changes require formal migration plans, consumer impact, rollback guidance, and stable unsupported-version reason codes.

Schema modules move through `draft`, `reviewed`, `validated`, `compatible`, `released`, `deprecated`, `retired`, or `blocked`. Release metadata must name the owning service family, downstream consumers, release status, privacy class, compatibility class, review authority, and consumer notes before a schema module can become released.

## Data Model

The first package should define these schema modules:

- `identity`: person, organization, node, app, native service, service account, system service, and central AI actor references.
- `tenant`: tenant, membership, role binding, app ownership, delegated access, and tenant state references.
- `command`: actor id, tenant id, command type, trace id, idempotency key, timestamp, schema version, payload, and signature metadata.
- `api_error`: reason code, message, trace id, retryability, correction fields, dependency id, and audit refs.
- `event`: event id, source service, subject id, event type, schema version, sequence, occurred time, trace id, and privacy classification.
- `audit`: actor, action, target, decision, reason code, policy refs, signature refs, and evidence refs.
- `workload_manifest`: workload class, image/package refs, resource requirements, data refs, secret refs, policy refs, network intent, and retry policy.
- `resource_manifest`: CPU, memory, GPU, storage, bandwidth, queue, lease, and locality requirements.
- `queue_and_lease`: queued command, lease grant, heartbeat, retry, timeout, cancellation, and completion records.
- `usage_and_metering`: measured resource dimensions, attribution refs, rollup window, receipt refs, and dispute refs.
- `oru_and_seal_ledger`: ORU account ref, transfer intent, settlement ref, reversal ref, evidence hash, and finality status.
- `policy_and_guard`: policy id, policy version, decision, matched rule refs, appeal refs, and data-class labels.
- `asset_and_namespace`: Overasset refs, namespace refs, route refs, directory listing refs, and ownership evidence.
- `ai_and_docdex`: personal assistant request refs, model-routing decision refs, encrypted Docdex index refs, RAG source refs, and ADES extraction refs.

Every externally visible schema must include a `schema_version` field. Mutating command schemas must require tenant, actor, trace id, and idempotency fields.

## API Surface

The package exposes library and build-tool interfaces:

- `validate(schemaName, payload)`: strict validation with typed success or stable validation error.
- `parse(schemaName, payload)`: validation plus typed object construction.
- `buildFixture(schemaName, variant)`: valid and invalid test payload generation.
- `generateTypes(targetLanguage)`: generated type outputs for approved implementation languages.
- `generateApiDocs()`: schema-derived reference documentation.
- `compareSchemas(previous, next)`: compatibility classification for additive, deprecated, or breaking changes.
- `listReasonCodes(domain)`: typed reason-code registry for service errors and denials.
- `assertCommonEnvelope(payload, envelopeKind)`: guardrail for command, event, audit, and usage records.

Services must import generated validators instead of copying schema fragments.

## Event Surface

The package has no runtime event authority. It defines event schemas that runtime services use.

- Runtime services emit `service.event_type` events through Overwatch-compatible pipelines.
- Schema package build jobs may publish build artifacts such as generated docs, compatibility reports, and fixture reports.
- Compatibility reports must identify breaking changes before a package version can be consumed by SDK or service code.
- Event schemas must include privacy classification so services know when to store references instead of private payloads.

## Core Workflow

1. Define or update canonical schema source.
2. Run schema linting for naming, envelope requirements, version fields, reason-code references, and privacy labels.
3. Generate types, validators, fixtures, and docs.
4. Run compatibility checks against the previous released schema set.
5. Run service contract tests that import the generated package.
6. Publish a versioned package only after tests and compatibility checks pass.
7. Update SDS and implementation-plan references when a schema change changes service responsibility.

## State Machine

Schema definitions should use a package lifecycle:

1. `draft`: proposed schema exists in source but is not consumed by runtime code.
2. `reviewed`: schema has owner, object family, privacy labels, and envelope checks.
3. `validated`: generated validators and fixtures pass.
4. `compatible`: compatibility report confirms the change is additive or explicitly managed.
5. `released`: package version is published for service, SDK, CLI, and test consumption.
6. `deprecated`: schema or field remains supported but has removal guidance and replacement mapping.
7. `retired`: schema or field is no longer accepted after a documented migration.
8. `blocked`: schema cannot release because ownership, validation, or compatibility checks failed.

Runtime business objects may have their own service-specific state machines; the package only defines the allowed values and transition object shapes.

## Policy And Security

- Strict validation is the default for runtime boundaries.
- Unknown fields should be rejected for security-sensitive objects unless an explicit forward-compatible extension map is defined.
- Secret-bearing fields must use secret-reference schemas rather than raw secret values.
- Privacy classification is required for user content, identity data, encrypted Docdex refs, AI assistant context refs, payment-like ORU objects, and system-service records.
- Reason codes must be stable enough for SDKs, users, operators, and central AI governance workflows.
- Schemas must distinguish public refs, tenant-private refs, regulated data refs, and system-service-only refs.
- Deprecations must be explicit; silent type changes are breaking changes.

## Metering And Accounting

The package defines accounting object shapes but does not perform accounting.

- Usage schemas must support compute, storage, bandwidth, model resources, queue capacity, cache usage, and operator action dimensions.
- ORU schemas must support low-friction internal accounting and HTTP 402-style machine-to-machine settlement.
- Seal Ledger schemas must capture evidence refs, settlement refs, reversal refs, and finality state without blockchain or NFT assumptions.
- Native-service schemas must support near-cost operation and stewardship surplus routing without embedding charge tables.

## Observability And Operations

The package should provide:

- Schema coverage reports by service and build phase.
- Compatibility reports for each package version.
- Fixture coverage for valid, invalid, boundary, and deprecated payloads.
- Generated documentation for service implementers and SDK consumers.
- Migration notes for deprecated fields and object versions.
- CI checks that fail when public APIs use objects without schema coverage.
- A registry view showing which services consume each schema module.

## Failure Modes And Recovery

- Missing schema for a public object: block the dependent service or SDK release.
- Breaking change without migration metadata: block package release.
- Divergent hand-written type in a service: fail contract tests and replace with generated type.
- Validator accepts invalid command envelope: treat as a release blocker.
- Fixture drift from schema source: regenerate and review fixtures before release.
- Deprecated field used by active service without migration plan: keep support and mark the consumer until it is updated.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Valid fixtures pass and invalid fixtures fail deterministically.
- API, worker, node agent, SDK, and CLI use generated types.
- Mutating command schemas include tenant, actor, trace id, and idempotency fields.

Additional SDS-level validation:

- Schema lint tests enforce naming, version, envelope, privacy, and reason-code rules.
- Compatibility tests classify every schema change.
- Generated type compilation succeeds for all supported targets.
- Runtime validators reject unknown or malformed security-sensitive payloads.
- Integration tests submit at least one command using generated SDK and service validators.
- Audit, usage, ORU, Seal Ledger, policy, and dispute fixtures prove the package covers later platform rails before those services depend on them.

## Build Breakdown

1. Choose the schema source format and code generation toolchain.
2. Define common primitives: ids, timestamps, refs, privacy labels, reason codes, trace ids, idempotency keys, and schema versions.
3. Add Phase 0 and Phase 1 modules: identity, tenant, command, event, audit, API error, manifest, queue, lease, and key metadata.
4. Add usage, ORU, Seal Ledger, policy, dispute, asset, namespace, AI assistant, Docdex RAG, and ADES-related schemas as downstream plans require them.
5. Build validators, generated types, fixture builders, and generated docs.
6. Add compatibility checks and deprecation metadata before the first external SDK release.
7. Make CI fail when public APIs, workers, or SDK surfaces use untyped payloads.

This package must land early enough for seed-hardware development to avoid later rewrites.

## Handoff And Downstream Use

This becomes the source of truth for [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md) control-plane APIs and all later service contracts.

Downstream services should import generated types and validators instead of reading private package internals or duplicating schema fragments. If a downstream service needs a new public object, update this SDS, the service implementation plan, and the build-plan crosswalk together.

## Open Design Questions

Resolved decisions:

- The first implementation should use language-neutral canonical JSON plus JSON Schema files under `packages/schemas` as the source of truth for commands, manifests, policy declarations, fixtures, signed payloads, public API objects, view models, generated documentation, and compatibility reports. Rust-owned validation, fixture, documentation, and diff tooling should consume those schemas. Protobuf may be generated or introduced only for compact internal service/RPC/event contracts where binary compatibility and transport efficiency matter; it must not replace JSON Schema as the docs-facing and fixture-facing contract authority.
- The required generated targets before Phase 6 are Rust first and a bounded TypeScript/web target second. Rust types and validators are mandatory for Phase 0/1 services, CLI, SDK, node-agent, worker, and integration-harness code because the accepted core stack is Rust-first. TypeScript/web models and validators should be generated from the same contracts before Phase 6 product integration needs UI, adapter, and browser-facing surfaces. Swift, Kotlin, Python, and other bindings wait until the schema compatibility checker, golden fixtures, and cross-language contract tests are stable.
- Extension maps are allowed only for explicitly named low-risk metadata surfaces: workload/resource capability annotations, package or manifest labels, docs-facing examples, adapter metadata, AI/RAG source hints, and future native-app view metadata. Security-sensitive and accounting-sensitive families must reject unknown fields completely by default, including command envelopes, identity and tenant refs, credential and secret refs, signatures, API errors, audit records, policy decisions, usage records, ORU objects, Seal Ledger refs, Overasset rights refs, dispute refs, and public namespace ownership refs. Any extension map must have a typed value schema, namespace prefix, privacy classification, and compatibility classification.
- Services and SDKs should support the current stable schema major version plus one previous stable major version once external native apps, adapters, or mobile clients depend on the package. Additive fields may remain within the current major version only when validators, generated types, fixtures, docs, and compatibility reports classify them as additive. Breaking changes to envelopes, signing, tenant/actor identity, policy, privacy, secret refs, accounting, ledger, ownership, or namespace objects require a new major version, migration metadata, and stable `schema_version_unsupported` or equivalent reason codes rather than silent downgrade behavior.
- Formal migration plans are required before deprecating fields in modules that can affect runtime authority, auditability, user rights, accounting finality, privacy, or cross-client compatibility: `identity`, `tenant`, `command`, `api_error`, `event`, `audit`, `queue_and_lease`, `usage_and_metering`, `oru_and_seal_ledger`, `policy_and_guard`, `asset_and_namespace`, and any credential, secret-ref, dispute, Overvault, Overbase, Overstore, Overasset, native-service, mobile, AI assistant, encrypted Docdex RAG, or ADES-facing public contract. Purely internal draft schemas may deprecate faster only while they remain unreleased and unconsumed by services, SDKs, CLI, or fixtures.
