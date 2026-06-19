# SUB BUILD PLAN #24 - Overpack

Attached SDS: [docs/sds/execution_scheduling/overpack.md](../sds/execution_scheduling/overpack.md)

## Purpose

This sub-build plan turns SDS #24 into an implementation sequence for Overpack. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overpack is the manifest and package contract for Overrid workloads and later application deployments. It starts in master Phase 3 as a strict workload manifest for private execution. It expands in master Phase 9 into an application-intent manifest for repeatable app deployment. It is not the runner, scheduler, storage service, deployment planner, package validator, policy authority, pricing engine, or accounting service.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #24: Overpack](../sds/execution_scheduling/overpack.md) | Controls Overpack purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering links, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overpack service plan](../service_catalog/execution_scheduling/overpack.md) | Controls the service-catalog objective, first build phases, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, canonical JSON/JSON Schema discipline, signed envelopes, trace ids, idempotency, local fixtures, deterministic validation harnesses, and generated Rust/TypeScript contract bindings. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, identity, tenant, key, registry, command, queue, and audit primitives that Overpack uses for validation and registration. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies registered Overcell nodes, capability records, runtime evidence, benchmark facts, and private node inventory used by Overpack compatibility checks. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Controls Overpack's first build point: a strict workload manifest for command jobs, supported OCI/container jobs, WASI where tested, model/artifact refs, resource cards, data class, egress, secrets, timeouts, retries, hashes, signatures, and provenance. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, dry-run reason codes, workload classifier facts, Oververify trust evidence, Overclaim dispute refs, cache trust scopes, and verification constraints that Overpack validation must reference without owning. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes Overpack workload/app ids, resource cards, billing-rule refs, model refs, service refs, and package refs as accounting dimensions while keeping pricing, settlement, and ledger mutation outside Overpack. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Adds SDK, CLI, admin/developer UI, Docdex, Mcoda, Codali, AI gateway, and product integration surfaces that create, validate, sign, submit, inspect, and diff manifests. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, Universal Namespace, Overmesh route resolution, Overasset refs, artifact storage, storage scopes, secret refs, private data refs, namespace routes, and route ownership facts used by Phase 9 app manifests. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Controls application-intent manifests, package validation, provenance, deployment planner compatibility, provisioning intent, health checks, release strategies, rollback, and AI-generated deployment compatibility. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Consumes purpose-scoped, grant-funded, and federation-safe manifest metadata for known external swarms without weakening private execution package integrity. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Adds public-provider manifest constraints, sandbox class declarations, low-sensitivity workload boundaries, fraud/challenge refs, and public-provider eligibility checks. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Consumes application-intent manifests through native apps, wallet usage views, personal AI surfaces, workspace apps, search, maps, directory, messaging, and media utilities as ordinary Overrid clients. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies schema migration, revocation, incident, compliance, threat-model, stewardship, and governance hardening for manifest history, provenance, compatibility, and deployment evidence. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #24 first workload-manifest build work aligned to master Phase 3, with application-intent deployment expansion in master Phase 9 and later federation, native-app, and governance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema for Overpack manifests, optional Protobuf for compact contracts, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and native Overrid service boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, and 9 | Attach SDS #24, freeze Overpack scope, preserve Phase 3 workload-manifest first build, and preserve Phase 9 application-intent expansion. |
| 2 | Master Phases 0, 1, and 3 | Build Rust contract modules, JSON Schemas, validators, fixtures, API error shape, and compatibility test harnesses. |
| 3 | Master Phase 3 | Implement Overpack v0 workload manifest fields needed before the first real private workload can execute. |
| 4 | Master Phases 1, 3, 4, and 9 | Add artifact refs, hashes, signatures, dependency locks, SBOM refs, provenance, revocation hooks, and Overregistry registration evidence. |
| 5 | Master Phases 3, 4, and 5 | Add resource cards, permissions, policy dry-run compatibility, budget refs, and accounting dimensions without moving policy or accounting ownership into Overpack. |
| 6 | Master Phases 1, 3, and 6 | Add validation/registration APIs, SDK/CLI flows, admin/developer validation reports, stable reason codes, and client automation outputs. |
| 7 | Master Phase 3 | Prove Overrun, Oversched, Overlease, Overguard, Overmeter, Overqueue, and Overwatch can consume the workload manifest without parsing informal instructions. |
| 8 | Master Phases 8 and 9 | Expand to application-intent manifests backed by storage, namespace, route, data, model, billing, scaling, health, and deployment-planner compatibility refs. |
| 9 | Master Phases 9, 11, and 13 | Add AI-generated manifest diffs, compatibility gates, schema migration, deprecation, revocation, public-provider constraints, and governance evidence. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, and implementation handoff gates. |

## Tech Stack Guardrails

- Overpack core is a Rust contract and validation service/module using shared contract types, Tokio where async validation/registration workers are required, and Axum/Tower/Hyper-style HTTP where an HTTP service boundary exists.
- Workload manifests, application-intent manifests, validation reports, compatibility reports, permission declarations, resource cards, API errors, lifecycle events, examples, and fixtures use canonical JSON plus JSON Schema.
- Ed25519 signatures are used for manifest/package signatures where signatures are required. BLAKE3/content hashes are used for manifest content, artifacts, dependency locks, SBOM refs, source-set evidence, validation reports, and replayable compatibility evidence.
- Overpack owns schema shape, semantic versions, compatibility rules, validation reports, manifest lifecycle state, artifact/hash/signature/provenance field definitions, runtime contracts, permission declarations, and resource cards.
- Overpack does not execute workloads, decide placement, store raw object bytes, decide policy, mutate ledgers, set prices, issue payouts, allocate grants, own secrets, or become a deployment planner.
- PostgreSQL, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFT, pricing, revenue, customer-count, or external payment assumptions must not become Overpack's product boundary.

## Phase 1: SDS Attachment, Scope, And Phase-Split Rules

### Work Items

- **1.1 Attach the build plan to SDS #24.**
  - Design: Link this document from the numbered Overpack SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/execution_scheduling/overpack.md`, `docs/service_catalog/execution_scheduling/overpack.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #24 returns both the Overpack SDS and this sub-build plan.

- **1.2 Freeze Overpack as the manifest and package contract.**
  - Design: Record that Overpack owns manifest schemas, semantic versions, compatibility rules, validation reports, artifact refs, hashes, signatures, provenance refs, runtime contracts, permission declarations, resource cards, and lifecycle state.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overpack does not become the runner, scheduler, storage service, deployment planner, package validator, policy authority, accounting mutator, pricing engine, or secret store.

- **1.3 Preserve master Phase 3 as the workload-manifest first build point.**
  - Design: Keep first implementation in master Phase 3 because Overrun, Oversched, Overlease, Overguard, and Overmeter need strict workload manifests before private workloads can execute safely.
  - Output: Phase-gate note that Phase 0 through Phase 2 are prerequisites and Phase 3 builds the execution-eligible workload manifest subset.
  - Validation: Review proves the Phase 3 subset supports only tested runtime cards and does not claim full app deployment, route provisioning, storage provisioning, release strategy, or public-provider readiness.

- **1.4 Preserve master Phase 9 as the application-intent expansion gate.**
  - Design: Keep application-intent manifests, deployment planner compatibility, provisioning intent, health, scaling, route binding, release strategy, rollback, and AI-generated deployment compatibility behind Phase 9.
  - Output: Phase 9 gate checklist for app identity, services, runtime/data/storage/model needs, permissions, wallet budget refs, billing-rule refs, routes, geography, scaling, health, observability, and rollback behavior.
  - Validation: Review confirms Phase 3 workload manifests remain backward-compatible when Phase 9 app manifests are added.

- **1.5 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #24 decisions for execution-eligible command and OCI/container jobs first, WASI only where tested, unknown canonical fields fail closed, dependency evidence uses lock refs plus BLAKE3/content hashes, CycloneDX JSON is the first normalized SBOM family, policy dry-run summaries remain compact, and AI-generated diffs are deterministic and risk-first.
  - Output: Resolved-decision checklist tied to SDS #24 open-question answers.
  - Validation: Review rejects unsupported runtimes becoming execution-eligible, unknown execution-affecting fields, raw secret values, permissive policy-affecting extension maps, private rule traces inside validation reports, and AI-generated manifests that bypass approval or policy rails.

## Phase 2: Rust Contracts, Schemas, Validators, And Fixtures

### Work Items

- **2.1 Create the Overpack Rust contract module.**
  - Design: Add a Rust module or crate for manifest envelopes, workload manifests, application-intent manifests, artifact refs, provenance records, runtime contracts, permission declarations, resource cards, validation reports, compatibility reports, lifecycle state, and stable reason codes.
  - Output: Contract module skeleton, Rust types, serde bindings, schema-version enums, lifecycle enums, error types, reason-code mapping, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms Overpack contracts remain separate from Overrun, Oversched, Overlease, Overguard, Overregistry, Package Validator, and Deployment Planner logic.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add JSON Schemas for workload manifest v0, manifest envelope, artifact refs, dependency locks, SBOM refs, package provenance, runtime contract, permission declaration, resource card, validation report, compatibility report, deprecation, revocation, and future app-intent manifest scaffolding.
  - Output: Versioned schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing manifest id, kind, schema version, tenant/app scope, actor, trace id, idempotency key, runtime contract, resource card, permissions, artifact hash, signature metadata, and validation state where required.

- **2.3 Define manifest lifecycle and state transitions.**
  - Design: Model draft, submitted, schema_valid, integrity_checked, policy_checked, registered, rejected, deprecated, revoked, and superseded states with legal transitions and append-only refs.
  - Output: Lifecycle state machine, transition reason codes, event payload schemas, illegal-transition behavior, and read-model fields.
  - Validation: State tests reject registration before schema/integrity checks, execution eligibility before required signatures and hashes, deprecation without reason, revocation without evidence refs, and mutation of registered versions.

- **2.4 Build deterministic manifest fixtures.**
  - Design: Create fixtures for signed command job, OCI/container job, unsupported WASI intent, model inference through OCI runtime card, missing signature, missing hash, raw secret leak, overbroad egress, missing resource card, invalid dependency lock, invalid SBOM, and future app-intent manifest.
  - Output: Fixture directory, expected validation reports, compatibility reports, policy dry-run stubs, reason-code snapshots, and replay hashes.
  - Validation: Fixture tests produce deterministic validation results and prove invalid packages fail before execution.

- **2.5 Define compatibility and migration primitives.**
  - Design: Add schema compatibility rules for additive non-execution metadata, breaking field changes, deprecated versions, migration hints, supported runtime profiles, validator profiles, and target service capability checks.
  - Output: Compatibility matrix, migration report schema, target-capability input schema, deprecation metadata, and fixture coverage.
  - Validation: Compatibility tests prove unknown execution-affecting fields fail closed while explicitly typed non-execution metadata can be accepted without altering policy, scheduling, leasing, metering, or deployment semantics.

## Phase 3: Workload Manifest V0

### Work Items

- **3.1 Define the workload manifest envelope.**
  - Design: Require manifest id, schema version, manifest kind, tenant id, actor/app scope, author, trace id, idempotency key, signature refs, compatibility version, created_at, policy refs, audit refs, and deprecation/supersession refs where applicable.
  - Output: `manifest_envelope` schema, Rust type, fixture set, envelope validator, and stable error reasons.
  - Validation: Envelope tests reject missing tenant scope, actor, trace id, idempotency key, schema version, manifest kind, author, signature refs where required, and unsupported compatibility versions.

- **3.2 Define execution-eligible runtime cards.**
  - Design: Support command jobs and OCI/container jobs first; allow WASI only where the Phase 3 Overrun adapter is tested; allow model inference through supported runtime cards with explicit model/artifact refs and GPU resource refs when needed.
  - Output: Runtime contract schemas for command, container, tested WASI, model-via-runtime, entrypoint, args, environment refs, mount rules, timeout, cleanup expectation, and allowed capability fields.
  - Validation: Runtime tests reject unsupported native-service, standalone model, untested WASI, hidden shell execution, missing entrypoint, unbounded timeout, unsafe mounts, and undeclared accelerator needs.

- **3.3 Define inputs, outputs, and artifact refs.**
  - Design: Model input refs, output refs, content-addressed artifact refs, object refs, media/runtime type, size, content hash, signature ref, storage class, retention hint, and protected/private artifact visibility.
  - Output: `artifact_ref`, input ref, output ref, storage-ref compatibility rules, fixture examples, and redacted read views.
  - Validation: Artifact tests reject missing content hashes, mutable tags without immutable digest refs, inaccessible artifact refs, raw private object content, undeclared output scopes, and output refs that bypass Overstore/Overvault ownership.

- **3.4 Define resource cards and workload classification.**
  - Design: Require CPU, memory, storage, network, GPU/accelerator where needed, model/runtime, queue priority, locality/region hints, cache hints, workload class, data sensitivity, deadline, timeout, retry, and cleanup requirements.
  - Output: Resource-card schema, workload-class enum, sensitivity enum, scheduler/lease compatibility notes, and required-field validators.
  - Validation: Resource-card tests prove Oversched and Overlease have enough explicit data to reason about placement and reservations without parsing informal package instructions.

- **3.5 Define egress, secrets, and permission declarations.**
  - Design: Require least-privilege egress policy, secret policy refs, storage refs, data refs, namespace refs, service-call refs, model/RAG refs, route refs, and explicit denial/default behavior.
  - Output: Permission declaration schema, secret-ref schema, egress profile schema, raw-secret rejection tests, and redaction profile.
  - Validation: Security tests prove raw secrets are rejected, undeclared egress fails closed, overbroad permissions produce risk reason codes, and Overpack declares intent without granting authority itself.

## Phase 4: Integrity, Signatures, Provenance, Dependency Locks, And SBOMs

### Work Items

- **4.1 Add manifest and artifact integrity checks.**
  - Design: Hash canonical manifest payloads, artifact refs, dependency refs, validation reports, and compatibility evidence with BLAKE3/content hashes and stable canonicalization rules.
  - Output: Hashing contract, canonicalization test vectors, content-hash fields, integrity result schema, and tamper fixtures.
  - Validation: Integrity tests reject changed manifests, changed artifacts, mismatched dependency refs, non-canonical hashes, and tampered validation reports.

- **4.2 Add signature verification through Overkey.**
  - Design: Verify Ed25519 manifest/package signatures, signer identity, key status, rotation metadata, revocation state, tenant scope, actor/app authority, and signature coverage over the canonical manifest payload.
  - Output: Signature verifier client, signature refs, signer authority checks, error reasons, and `overpack.manifest_rejected` event payloads.
  - Validation: Signature tests reject missing signatures, wrong tenant signer, revoked key, stale key metadata, partial signature coverage, and modified payloads after signing.

- **4.3 Define package provenance records.**
  - Design: Record source repo/build refs, builder identity, generated-by refs, base image/module refs, dependency locks, SBOM refs, policy compatibility refs, artifact refs, and validation report refs without storing private source content.
  - Output: `package_provenance` schema, provenance fixture set, builder identity checks, source-ref redaction rules, and provenance event payloads.
  - Validation: Provenance tests prove operators can answer what was submitted, by whom, from which artifact, with which dependency/SBOM refs, without exposing private source or provider-private topology.

- **4.4 Add dependency lock and SBOM refs.**
  - Design: Use `dependency_lock` objects that record native lock-file refs and artifact digests; require Cargo.lock evidence for Rust-built packages and immutable image/base/layer refs for OCI/container packages; accept CycloneDX JSON as the first normalized SBOM family.
  - Output: Dependency lock schema, CycloneDX JSON normalization contract, OCI digest fixture, Cargo.lock ref fixture, and Package Validator handoff notes.
  - Validation: Dependency tests reject missing lock refs, mutable image tags without digests, unsupported SBOM attachments without normalization, and dependency-private details in public reports.

- **4.5 Add revocation and unsafe provenance hooks.**
  - Design: Allow security, incident, Overwatch, Package Validator, Oververify, Overguard, or governance evidence to mark a manifest/dependency/artifact unsafe without editing registered history.
  - Output: Revocation record schema, blocked-use state, dependency-unsafe reason codes, notification event payloads, and affected-manifest lookup behavior.
  - Validation: Revocation tests prove revoked dependencies block new execution/deployment, prior registered records remain readable, and consumers receive explicit blocked or superseded states.

## Phase 5: Policy, Permissions, Resource Cards, Budget Refs, And Accounting Dimensions

### Work Items

- **5.1 Add Overguard policy dry-run compatibility.**
  - Design: Send workload class, data sensitivity, egress, secrets, storage, route, model/RAG, resource card, budget refs, runtime type, artifact trust, and package provenance to Overguard for advisory dry-runs.
  - Output: `POST /overpack/manifests/{manifest_id}/dry-run-policy` behavior, dry-run summary schema, stable reason codes, expiry fields, and policy bundle/evaluator version refs.
  - Validation: Policy tests prove full matched-rule traces, fact snapshots, private evidence refs, replay bundles, and operator-only explanations remain external refs and are not embedded in public validation reports.

- **5.2 Define compact validation reports.**
  - Design: Produce reports with schema result, signature result, artifact integrity result, policy dry-run summary, budget compatibility, route ownership checks, warnings, errors, severity, remediation hints, and validation refs.
  - Output: `manifest_validation_report` schema, report API response, CLI/admin rendering fields, redaction rules, and report fixture set.
  - Validation: Report tests prove automation receives stable reason codes while private manifests, secret refs, private artifact details, provider topology, and full policy traces are redacted or externalized.

- **5.3 Keep resource cards scheduler-ready and lease-ready.**
  - Design: Ensure resource cards include dimensions and constraints consumed by Oversched and Overlease, including CPU/GPU/memory/storage/network, accelerator runtime, region/locality, cache hints, runtime profile, deadline, timeout, and cleanup overhead.
  - Output: Scheduler/lease compatibility report, required field matrix, missing-resource reason codes, and Phase 3 placement fixtures.
  - Validation: Contract tests prove scheduling and leasing consumers can reject incomplete manifests without interpreting free-form text.

- **5.4 Keep accounting dimensions non-pricing.**
  - Design: Include workload/app ids, package refs, model refs, service ids, billing-rule refs, resource-card dimensions, and budget declarations as Overmeter/ORU/Overbill inputs without setting rates, balances, invoices, or payouts.
  - Output: Accounting-dimension schema notes, budget-ref validation, billing-rule ref contract, and Overmeter handoff fields.
  - Validation: Review rejects pricing, revenue, customer-count, settlement mutation, provider payout, ORU balance movement, or direct Seal Ledger writes inside Overpack.

- **5.5 Add route, namespace, storage, and secret compatibility refs.**
  - Design: For later phases, validate that route refs, namespace refs, Overstore artifact refs, Overbase data refs, Overvault secret refs, model/RAG refs, and service-call refs are declared explicitly and can be checked by owning services.
  - Output: Compatibility input schema, redacted ref summaries, owner-service lookup notes, and validation warnings for unavailable later-phase primitives.
  - Validation: Compatibility tests prove Overpack can report missing prerequisites without pretending to own route, storage, vault, namespace, model-routing, or data-platform authority.

## Phase 6: Validation And Registration APIs, SDK/CLI, And UI Reports

### Work Items

- **6.1 Implement manifest validation APIs.**
  - Design: Specify `POST /overpack/manifests/validate`, `POST /overpack/compatibility`, and `GET /overpack/schemas/{kind}/{version}` with signed caller identity, tenant scope, trace id, idempotency key, schema version, and stable error format.
  - Output: API request/response schemas, validation pipeline, compatibility target inputs, schema read endpoint, pagination/authorization rules, and Overwatch event payloads.
  - Validation: API tests cover valid validation, invalid schema, unsupported version, missing idempotency key, wrong tenant, unauthorized schema reads where applicable, and compatibility target mismatch.

- **6.2 Implement manifest registration through Overregistry.**
  - Design: Specify `POST /overpack/manifests` and `GET /overpack/manifests/{manifest_id}` so accepted manifest versions are immutable records registered through Overregistry with validation refs and provenance refs.
  - Output: Registration flow, Overregistry client contract, immutable version behavior, duplicate submission behavior, authorized read model, and `overpack.manifest_registered` event payloads.
  - Validation: Registration tests reject changes to registered versions, return stable ids for idempotent retries, and require new versions for manifest changes.

- **6.3 Add deprecation, supersession, and read APIs.**
  - Design: Specify `POST /overpack/manifests/{manifest_id}/deprecate`, supersession refs, revocation read status, schema compatibility reads, and redacted manifest metadata reads.
  - Output: Deprecation API schema, lifecycle transition logic, read filters, visibility matrix, and event payloads.
  - Validation: Lifecycle tests reject deprecating unknown manifests, deprecated versions becoming execution-eligible when policy blocks them, and unauthorized callers reading private artifact details.

- **6.4 Add SDK and CLI flows.**
  - Design: Provide generated SDK/CLI commands for manifest lint, validate, sign, dry-run policy, compare compatibility, register, deprecate, print schemas, and render validation reports.
  - Output: CLI command contract, SDK method names, stable JSON output fixtures, exit-code rules, and shell-friendly report examples.
  - Validation: CLI/SDK tests prove scripts can create, validate, sign, register, and inspect manifests without manual internal API calls or privileged UI shortcuts.

- **6.5 Add admin/developer UI report contracts.**
  - Design: Define report fields for manifest status, schema paths, error reason codes, permission risk, resource-card completeness, signature state, provenance warnings, policy dry-run summary, route/storage/secret refs, and AI diff groups.
  - Output: UI read-model contract, redaction matrix, sorted error/warning sections, and product-integration fixture set.
  - Validation: UI contract tests prove developers see actionable remediation hints while secrets, private source details, provider-private topology, raw policy evidence, and unauthorized tenant data stay hidden.

## Phase 7: Consumer Contracts And Execution Eligibility

### Work Items

- **7.1 Wire Overrun package verification contracts.**
  - Design: Ensure Overrun can verify manifest signature, artifact hashes, runtime contract, inputs, output refs, egress policy, secret refs, timeout, retry policy, cleanup expectation, and execution eligibility before preparing the sandbox.
  - Output: Overrun consumer contract, verification request/response schemas, failure reason codes, and first private workload fixture.
  - Validation: Integration tests prove invalid package, missing hash, revoked signer, unsupported runtime, raw secret, denied egress, and untested WASI fail before execution.

- **7.2 Wire Oversched placement contracts.**
  - Design: Ensure Oversched can consume resource cards, workload class, data sensitivity, locality/region, cache hints, runtime profile, accelerator requirements, policy summary refs, and provider/node compatibility requirements.
  - Output: Scheduler compatibility report, placement-input contract, missing-field errors, and deterministic selection fixture.
  - Validation: Scheduler tests prove placement is explainable and does not depend on free-form manifest text.

- **7.3 Wire Overlease reservation contracts.**
  - Design: Ensure Overlease can consume resource card dimensions, lease scope, deadline, timeout, renewal expectation, cleanup overhead, provider/node constraints, and idempotent workload refs.
  - Output: Lease request contract, reservation compatibility report, stale/missing field reason codes, and lease-window fixture.
  - Validation: Lease tests prove work cannot become execution-ready without resource reservations compatible with the manifest.

- **7.4 Wire Overguard, Overmeter, Overqueue, and Overwatch refs.**
  - Design: Ensure policy dry-run refs, queue item refs, trace/audit refs, workload/app ids, model refs, service ids, billing-rule refs, and package refs are stable enough for policy, queueing, metering, and audit consumers.
  - Output: Cross-service field matrix, event payload refs, audit trace mapping, and metering dimension notes.
  - Validation: Contract tests prove consumers use stable refs instead of parsing informal package instructions or private manifest payloads.

- **7.5 Prove the first execution-eligible manifest.**
  - Design: Use one signed command/OCI workload manifest that passes schema, integrity, signature, policy dry-run, resource-card completeness, lease compatibility, and Overrun verification.
  - Output: Golden manifest fixture, validation report, compatibility report, Overregistry record, queue submission ref, lease ref, run ref, result ref, and usage ref.
  - Validation: End-to-end test proves a known private node runs a real job through queue, scheduler, lease, runner, metering, result return, and audit using the manifest contract.

## Phase 8: Application-Intent Manifest And Deployment Compatibility

### Work Items

- **8.1 Define the application-intent manifest.**
  - Design: Add app identity, services, runtime cards, data needs, storage needs, model needs, permissions, wallet budget refs, billing-rule refs, routes, geography, scaling, security, health checks, observability, and rollback intent.
  - Output: `application_intent_manifest` schema, examples, invalid fixtures, compatibility notes, and app-lifecycle state mapping.
  - Validation: Schema tests prove the app manifest expresses intent rather than exact machine placement or manual infrastructure edits.

- **8.2 Add data, storage, vault, namespace, and route refs.**
  - Design: Reference Overbase collections, Overstore artifact/object scopes, Overvault secret scopes, Universal Namespace records, Overmesh routes, route ownership, storage retention, backup, repair, and private data needs.
  - Output: Phase 8 compatibility refs, route ownership summary, storage/vault/namespace validation stubs, and redacted report fields.
  - Validation: Compatibility tests prove missing Phase 8 prerequisites produce clear blocked states instead of moving storage, secret, namespace, or route ownership into Overpack.

- **8.3 Add deployment planner compatibility.**
  - Design: Generate a compatibility report that Deployment Planner can turn into ordered steps for authorization, budget reservation, runtime allocation, data/store allocation, route binding, service deployment, traffic activation, health observation, and settlement hooks.
  - Output: Planner compatibility schema, ordered-step hints, reversible-step markers, prerequisite refs, and deployment-ready state.
  - Validation: Planner contract tests prove Deployment Planner remains the owner of execution steps while Overpack owns declarative manifest shape and compatibility evidence.

- **8.4 Add package validation and release-strategy hooks.**
  - Design: Hand off dependency locks, SBOM refs, artifact refs, signatures, permissions, health checks, route changes, scaling rules, rollback behavior, canary/blue-green/rolling intent, and version pins to Package Validator and Release Strategy Service.
  - Output: Package Validator input schema, release strategy refs, rollback fixture, and health-gate compatibility notes.
  - Validation: Tests prove invalid packages fail before deployment and release strategy services consume manifest intent without Overpack deploying services itself.

- **8.5 Prove one signed app deployment manifest.**
  - Design: Create a signed application-intent fixture for one app that binds runtime, data, storage, routes, policy, metering, billing refs, health, observability, and rollback through normal Overrid service contracts.
  - Output: Golden app manifest, validation report, package validation ref, planner compatibility report, route/storage refs, health check refs, and rollback refs.
  - Validation: Phase 9 test proves a developer can deploy and update an app by changing a signed manifest rather than hand-building infrastructure.

## Phase 9: AI-Generated Manifests, Diffs, Public Constraints, And Governance Hardening

### Work Items

- **9.1 Add deterministic AI-generated manifest diffs.**
  - Design: Group diffs by permissions, budget/ORU exposure, routes and traffic, resources/runtime, storage, vault, model/RAG refs, data class, egress, provenance, health/scaling, rollback behavior, and effective privilege expansion.
  - Output: Diff schema, severity enum, old/new effective scope fields, reason codes, validation refs, dry-run refs, route impact, estimated budget-impact refs, and human review view.
  - Validation: Diff tests prove high-risk permission, budget, route, secret, privileged-runtime, data-class, or egress expansion requires explicit signed approval or revision.

- **9.2 Add permission minimization and proposal repair hints.**
  - Design: Detect overbroad egress, secret, storage, route, data, model/RAG, service-call, runtime, geography, scaling, and budget requests and return minimized alternatives where safe.
  - Output: Minimization reason codes, repair hint schema, before/after report fields, and AI proposal negative fixtures.
  - Validation: Tests prove AI-generated manifests must pass the same schema, provenance, validation, policy, and budget checks as human-authored manifests.

- **9.3 Add public-provider and low-sensitivity constraints.**
  - Design: Prepare manifest fields for public sandbox profile, provider eligibility, challenge refs, fraud-control refs, public low-sensitivity workload class, payout hold refs, and reputation/anti-Sybil constraints without enabling sensitive workloads too early.
  - Output: Public constraint schema fields, Phase 11 compatibility notes, sandbox profile refs, challenge refs, and policy-blocked fixture set.
  - Validation: Public-provider tests prove sensitive, secret-bearing, private-data, privileged-runtime, or unknown-provider workloads remain blocked until owning policy and verification services allow them.

- **9.4 Add schema migration and version governance.**
  - Design: Support migration reports, deprecated version warnings, compatibility windows, schema registry refs, PIP refs where required, validator profile versions, and non-breaking metadata extension rules.
  - Output: Migration report schema, version policy, deprecation behavior, PIP linkage notes, and compatibility test matrix.
  - Validation: Migration tests prove old registered versions remain readable, execution eligibility follows current policy, and breaking changes require explicit compatibility reports.

- **9.5 Add incident, revocation, and compliance evidence.**
  - Design: Preserve manifest history, provenance refs, signature refs, validation report refs, policy dry-run refs, revocation refs, incident refs, compliance boundary refs, and threat-model review refs for audit replay.
  - Output: Governance evidence bundle schema, incident/refocation read model, compliance export fields, and stewardship reporting inputs.
  - Validation: Governance tests prove an operator can trace a deployment or workload back through manifest version, signatures, artifacts, dependencies, SBOM, validation, policy dry-run, revocation, and compatibility evidence.

## Phase 10: Validation, Documentation, Queue State, And Handoff

### Work Items

- **10.1 Validate contract and schema coverage.**
  - Design: Run focused checks for manifest envelope, workload manifest, application-intent manifest, runtime contracts, permission declarations, resource cards, artifact refs, dependency locks, SBOM refs, provenance, validation reports, compatibility reports, lifecycle states, and reason codes.
  - Output: Schema-test report, fixture coverage matrix, compatibility report, lifecycle test report, and failure notes.
  - Validation: Tests pass before implementation advances beyond the documented gate; any blocker is recorded in build-plan progress.

- **10.2 Validate Phase 3 execution eligibility.**
  - Design: Prove a signed workload manifest flows through validation, integrity, signature verification, Overregistry registration, Overguard dry-run, Overqueue submission, Oversched placement, Overlease reservation, Overrun verification/execution, Overmeter usage, and Overwatch audit.
  - Output: End-to-end private-workload fixture, source-ref bundle, validation report, compatibility report, run result, usage ref, and audit trail.
  - Validation: Replay confirms successful, rejected, unsupported-runtime, revoked-artifact, denied-egress, missing-secret-ref, and timed-out workloads produce distinct auditable states.

- **10.3 Validate Phase 9 deployment compatibility.**
  - Design: Prove one signed application-intent manifest can drive package validation and deployment planner compatibility for runtime, data, storage, routes, policy, metering, billing refs, health, release strategy, and rollback.
  - Output: App-deployment fixture, package validation report, planner compatibility report, route/storage/namespace refs, health refs, rollback refs, and AI diff report.
  - Validation: Phase 9 checks prove Overpack declares app intent while Package Validator, Deployment Planner, Release Strategy Service, Overstore, Overvault, Overbase, Overmesh, Overmeter, and accounting services retain their authority.

- **10.4 Validate security, privacy, and tech-stack alignment.**
  - Design: Scan implementation and docs for raw secret leakage, private payload leakage, unsupported runtime execution, permissive unknown fields, conventional cloud-product boundary assumptions, blockchain/NFT mechanics, pricing/revenue/customer-count assumptions, and TypeScript core-runtime drift.
  - Output: Security/privacy checklist, tech-stack alignment report, negative-control scan results, and remediation notes.
  - Validation: Review confirms Overpack remains Rust-first/native-Overrid infrastructure and uses canonical JSON/JSON Schema, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and native Overrid service boundaries.

- **10.5 Validate documentation alignment and handoff.**
  - Design: Ensure SDS #24, the Overpack service plan, master build plan, build-plan crosswalk, phase docs, queue state, and progress docs link to this sub-build plan and preserve the Phase 3/Phase 9 split.
  - Output: Updated source-document links, sub-build-plan index entries, progress evidence, queue status, and downstream handoff matrix.
  - Validation: Markdown link checks pass and review confirms no master Phase 0 through Phase 13 ordering change was required.

## Alignment Review

- The sub-build plan keeps Overpack first workload-manifest build work in master Phase 3, matching SDS #24, the service catalog entry, Phase 3 plan, master build plan, and build-plan crosswalk.
- The plan keeps application-intent manifests, deployment planner compatibility, package validation handoff, release strategy refs, route/storage/data provisioning intent, and AI-generated deployment compatibility in master Phase 9, matching SDS #24 and the Phase 9 deployment-platform plan.
- The plan treats Phase 0 through Phase 2 as prerequisites for shared schemas, local fixtures, identity/tenant/key/audit/registry primitives, private-node capability facts, and runtime profiles rather than as Overpack's first implementation phase.
- The plan treats Phase 4, Phase 5, Phase 6, Phase 8, Phase 10, Phase 11, Phase 12, and Phase 13 as downstream policy, accounting, product, storage/namespace, federation, public-provider, native-app, and governance gates rather than reasons to weaken the Phase 3 workload manifest contract.
- The plan preserves the master Phase 0 through Phase 13 order and uses later phases only for application deployment expansion, product integration, storage/namespace refs, federation/public constraints, native app consumption, and governance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service/contracts, native Overrid boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.
