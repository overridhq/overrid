# SUB BUILD PLAN #7 - Shared Schema Package

Attached SDS: [docs/sds/foundation/shared_schema_package.md](../sds/foundation/shared_schema_package.md)

## Purpose

This sub-build plan turns SDS #7 into an implementation sequence for the Shared Schema Package. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

The Shared Schema Package is a canonical contract package, not a deployed microservice, registry authority, policy engine, accounting service, or production state store. Its job is to define language-neutral schema sources, generated Rust types and validators, deterministic fixtures, stable reason codes, compatibility metadata, and generated documentation for Overrid service boundaries.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #7: Shared Schema Package](../sds/foundation/shared_schema_package.md) | Controls package purpose, object families, owned responsibilities, APIs, lifecycle, security rules, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Shared Schema Package service plan](../service_catalog/foundation/shared_schema_package.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Controls the first build point for repository layout, `packages/schemas`, API/event discipline, local fixtures, and strict schema validation. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies the first signed command, tenant, identity, credential, registry, audit, and queue consumers. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies workload, resource, lease, result, overpack, scheduling, and metering contract consumers. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage, ORU, Seal Ledger, receipt, correction, hold, grant, asset, and dispute contract consumers. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, namespace, route, ownership, and rights contract consumers. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #7 first build work aligned to master Phase 0, with downstream schema expansion gated by owning service phases. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires canonical JSON plus JSON Schema as the docs-facing source of truth, Rust-first generation and validation, TypeScript/web second, and Protobuf only for compact internal RPC/event contracts. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 0 | Attach SDS #7 to the build-plan layer, freeze package boundaries, and preserve resolved schema-source decisions. |
| 2 | Master Phase 0 | Define canonical schema source layout, common primitives, naming rules, and version metadata before service logic. |
| 3 | Master Phase 0 and Phase 1 | Define identity, tenant, command, API error, event, audit, manifest, queue, lease, and key metadata modules for the first control-plane path. |
| 4 | Master Phase 0 and Phase 1 | Build Rust-first generation outputs, validator packages, documentation generation, and bounded TypeScript/web target metadata. |
| 5 | Master Phase 0 and Phase 1 | Implement strict validation, parse helpers, envelope assertions, reason-code registries, and security-sensitive unknown-field rejection. |
| 6 | Master Phase 0 with SDS #3 and SDS #4 | Build deterministic fixtures, golden negative cases, local-stack contract checks, and integration-harness artifacts. |
| 7 | Master Phase 0 through Phase 6 | Add compatibility reports, deprecation metadata, migration gates, and current-plus-previous stable major support before external consumers rely on the package. |
| 8 | Master Phases 3, 4, 5, 8, and 12 | Expand schema families for execution, policy, accounting, rights, namespace, storage, native apps, AI, mobile, Docdex RAG, and ADES only as owning phases mature. |
| 9 | Master Phase 0 with Phase 6 hardening | Release the package through CI gates, consumer registry metadata, generated docs, and public API untyped-payload blockers. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, tech-stack alignment, cross-document links, queue state, and downstream handoff rules. |

## Tech Stack Guardrails

- Canonical JSON plus JSON Schema files under `packages/schemas` are the source of truth for commands, manifests, policy declarations, fixtures, signed payloads, public API objects, view models, generated documentation, and compatibility reports.
- Rust-owned validation, fixture, documentation, diff, and generation tooling consumes those schemas first because Overrid core, CLI, services, node agents, and local tooling are Rust-first.
- TypeScript/web models and validators are generated from the same contracts after Rust generation, validator behavior, fixtures, and compatibility checks are stable enough for Phase 6 UI, adapter, and browser-facing surfaces.
- Protobuf may be generated or introduced only for compact internal service/RPC/event contracts where binary compatibility and transport efficiency matter. It must not replace JSON Schema as the docs-facing and fixture-facing contract authority.
- Security-sensitive and accounting-sensitive families reject unknown fields completely by default, including command envelopes, identity refs, tenant refs, credential refs, signatures, API errors, audit records, policy decisions, usage records, ORU objects, Seal Ledger refs, Overasset refs, dispute refs, and public namespace ownership refs.
- Extension maps are allowed only for explicitly named low-risk metadata surfaces with typed values, namespace prefixes, privacy classification, and compatibility classification.
- The package defines object shapes, validators, fixtures, docs, and compatibility metadata. Runtime authority stays with Overgate, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, Overguard, Overmeter, ORU, Seal Ledger, Overasset, Overbase, Overstore, Overvault, and other owning services.
- The package must not introduce PostgreSQL, Redis, S3, MinIO, Kafka, NATS, Vault, blockchain, NFT, pricing, revenue, customer-count, or conventional SaaS control-plane assumptions.

## Phase 1: SDS Attachment, Package Boundary, And Authority Rules

### Work Items

- **1.1 Attach the build plan to SDS #7.**
  - Design: Link this document from the numbered Shared Schema Package SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/foundation/shared_schema_package.md`, `docs/service_catalog/foundation/shared_schema_package.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #7 returns both the SDS and this sub-build plan.

- **1.2 Freeze the package boundary.**
  - Design: Record that the Shared Schema Package is a versioned contract package and library, not a runtime registry, policy engine, audit authority, accounting authority, or production record store.
  - Output: Boundary guardrails documented in this plan and mirrored by implementation review checklists.
  - Validation: Architecture review confirms no work item requires service-owned production state, runtime policy decisions, ledger finality, or direct registry authority inside the package.

- **1.3 Preserve master Phase 0 as the first build point.**
  - Design: Keep the first implementation under Phase 0 because services, CLI, SDK, local stack, test harness, and later product adapters need shared contracts before their own logic stabilizes.
  - Output: Phase-gate note that SDS #7 starts in Phase 0 and expands later only through owning service phase gates.
  - Validation: Review proves this plan does not move schema authority into Phase 1 services or change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve canonical JSON plus JSON Schema as the source of truth, Rust generation first, TypeScript/web second, Protobuf only for compact internal RPC/event contracts, strict unknown-field rejection for sensitive objects, typed extension maps only for low-risk metadata, current-plus-previous stable major support, and formal migration plans for authority-sensitive fields.
  - Output: Resolved-decision checklist tied to SDS #7 open-question answers.
  - Validation: Review proves the plan does not re-open or contradict the resolved open design questions in SDS #7.

- **1.5 Define ownership and escalation rules.**
  - Design: Require each schema module to name the owning service family, downstream consumers, release status, privacy class, compatibility class, and review authority before it can become released.
  - Output: Schema ownership metadata model and review checklist.
  - Validation: Schema lint fails draft modules without owner, status, privacy class, compatibility metadata, and consumer notes.

## Phase 2: Canonical Schema Source Layout And Common Primitives

### Work Items

- **2.1 Define `packages/schemas` source layout.**
  - Design: Create a source tree that separates common primitives, object-family modules, fixtures, generated docs, compatibility reports, generated outputs, and internal binary-contract projections.
  - Output: `packages/schemas` layout map with approved source and generated-output paths.
  - Validation: Layout checks reject generated files in source directories and reject public schema sources outside approved schema paths.

- **2.2 Define common identifiers and references.**
  - Design: Model person, organization, tenant, node, app, native service, service account, system service, central AI actor, Overasset, namespace, route, package, workload, queue, lease, receipt, evidence, and secret-reference ids as typed schema primitives.
  - Output: Common id/ref schema module with stable naming and examples.
  - Validation: Fixture tests prove id/ref fields are typed, non-empty, versioned where needed, and cannot be replaced by unstructured strings in public objects.

- **2.3 Define time, trace, version, and idempotency primitives.**
  - Design: Standardize timestamp, logical sequence, schema version, command id, trace id, idempotency key, request hash, payload hash, compatibility window, and deprecation metadata primitives.
  - Output: Common lifecycle and envelope primitive module.
  - Validation: Lint checks require mutating commands and external records to carry the expected primitive fields.

- **2.4 Define privacy and data-class primitives.**
  - Design: Distinguish public refs, tenant-private refs, regulated data refs, encrypted/private refs, user-content refs, system-service-only refs, and redacted diagnostic refs.
  - Output: Privacy classification schema and allowed-field rules.
  - Validation: Tests reject event, audit, AI, Docdex RAG, payment-like, namespace, or system-service objects without privacy classification.

- **2.5 Define reason-code and correction primitives.**
  - Design: Create stable reason-code, retryability, correction-field, dependency-name, policy-ref, audit-ref, and unsupported-version primitives for service errors and denials.
  - Output: Reason-code registry schema and generated enum source metadata.
  - Validation: Error fixtures prove reason codes are stable, documented, generated into Rust first, and available to SDK/CLI without free-form parsing.

## Phase 3: Phase 0 And Phase 1 Contract Modules

### Work Items

- **3.1 Define identity and tenant modules.**
  - Design: Create schemas for identity refs, actor refs, organization refs, tenant refs, membership, role binding, delegated access, quota scope, suspension state, catalog visibility, and audit context.
  - Output: `identity` and `tenant` schema modules with valid and invalid fixtures.
  - Validation: Tests reject missing tenant/actor references on mutating command objects and reject raw secret or credential values in identity records.

- **3.2 Define command and API error modules.**
  - Design: Define command envelopes with tenant id, actor id, command type, trace id, idempotency key, timestamp, schema version, payload type, payload hash, signature metadata, and stable error objects.
  - Output: `command` and `api_error` modules with reason-code links and canonicalization expectations.
  - Validation: Envelope checks fail commands missing required fields, accepting unknown sensitive fields, or returning errors without reason code, trace id, retryability, and correction shape.

- **3.3 Define event and audit modules.**
  - Design: Define event envelopes and audit records with event id, source service, subject id, sequence, occurred time, actor, action, decision, policy refs, signature refs, evidence refs, privacy class, and schema version.
  - Output: `event` and `audit` modules with Overwatch-compatible shapes.
  - Validation: Audit/event fixtures prove events are append-only records with privacy classification and no raw private payload leakage.

- **3.4 Define manifest and registry modules.**
  - Design: Define workload manifest, resource manifest, package manifest refs, capability records, image/package refs, resource requirements, data refs, secret refs, policy refs, network intent, retry policy, and schema-ref metadata.
  - Output: `workload_manifest`, `resource_manifest`, and registry metadata modules.
  - Validation: Manifest validators reject missing schema versions, malformed secret refs, forbidden untyped capability blobs, and private package internals exposed as public schema authority.

- **3.5 Define queue, lease, credential, and key metadata modules.**
  - Design: Define queued command, lease grant, heartbeat, retry, timeout, cancellation, completion, dead-letter refs, credential metadata, signer refs, key rotation, and revocation records.
  - Output: `queue_and_lease` and `credential_key_metadata` schema modules.
  - Validation: Fixtures prove queue/lease records preserve trace, tenant, actor, command, retry, timeout, cancellation, and credential metadata without embedding private key material.

## Phase 4: Generation Toolchain, Rust Outputs, And Docs Projections

### Work Items

- **4.1 Choose and document the schema-generation toolchain.**
  - Design: Select a Rust-first generation pipeline that reads JSON Schema source, emits Rust types/validators/docs/fixture metadata, and can later emit TypeScript/web outputs from the same source.
  - Output: Toolchain decision record, generation command contract, and reproducibility notes.
  - Validation: Generation dry run proves outputs are deterministic from source schemas and do not depend on hand-edited generated files.

- **4.2 Generate Rust types and validators first.**
  - Design: Generate Rust structs/enums/newtypes, parse helpers, validator entrypoints, stable reason-code enums, schema-version constants, and redaction metadata from canonical source schemas.
  - Output: Rust generated package outputs under approved generated paths.
  - Validation: Rust compile and fixture validation checks pass once implementation exists; generation checks fail when schema changes are not reflected in outputs.

- **4.3 Generate schema-derived documentation.**
  - Design: Produce docs for object families, required fields, privacy classes, reason codes, extension-map rules, compatibility status, and example payloads without making prose docs the contract authority.
  - Output: Generated reference documentation and source-to-doc trace metadata.
  - Validation: Docs generation fails if public schemas lack description, privacy class, version, owner, reason-code links, or examples where required.

- **4.4 Prepare TypeScript/web projection rules.**
  - Design: Define how TypeScript/web models, validators, and error objects will be generated after Rust outputs and golden fixtures stabilize, including generated-output path and browser-safe redaction rules.
  - Output: TypeScript/web generation plan and blocked-until checklist.
  - Validation: Review confirms TypeScript/web output is generated from canonical schemas and cannot become the source of truth.

- **4.5 Define Protobuf projection boundaries.**
  - Design: Allow Protobuf only as an internal binary projection for compact service/RPC/event contracts, preserving JSON Schema authority for docs, fixtures, command payloads, policy declarations, manifests, and public API examples.
  - Output: Binary projection rules and source-of-truth warning metadata.
  - Validation: Checks reject Protobuf-only public object definitions and require a canonical JSON Schema source for every public object.

## Phase 5: Strict Validators, Parse Helpers, Envelope Assertions, And Reason Codes

### Work Items

- **5.1 Implement strict validation defaults.**
  - Design: Reject unknown fields for sensitive families and allow extension maps only where SDS #7 explicitly permits them with typed values, namespace prefixes, privacy class, and compatibility class.
  - Output: Strict validator configuration and extension-map rule table.
  - Validation: Negative fixtures prove unknown sensitive fields fail for commands, identity, tenant, credentials, signatures, errors, audit, policy, usage, ORU, Seal Ledger, Overasset, dispute, and namespace ownership objects.

- **5.2 Implement parse helpers.**
  - Design: Provide parse helpers that combine schema validation, typed object construction, error normalization, privacy classification checks, and reason-code references.
  - Output: `parse(schema_name, payload)` style API for Rust package consumers and later generated bindings.
  - Validation: Parse tests prove malformed, unsupported-version, missing-required-field, wrong-privacy-class, and unknown-field cases return stable validation errors.

- **5.3 Implement common envelope assertions.**
  - Design: Provide assertions for command, event, audit, usage, ledger, and public response envelopes so services cannot publish partially shaped objects.
  - Output: `assertCommonEnvelope` style validators and lint checks.
  - Validation: Service contract tests fail when mutating command schemas lack tenant, actor, trace id, idempotency key, command type, timestamp, schema version, or signature metadata.

- **5.4 Implement reason-code registries.**
  - Design: Build domain-specific reason-code registries for validation, identity, tenancy, credentials, policy, queue, execution, accounting, storage, namespace, AI, and compatibility errors.
  - Output: `listReasonCodes(domain)` output and generated enum bindings.
  - Validation: Tests reject errors and denial objects with undocumented reason codes or free-form replacement strings.

- **5.5 Implement redaction and privacy-aware diagnostics.**
  - Design: Attach redaction hints to sensitive fields and force diagnostic fixtures to prove private payloads, secrets, credential material, signatures, and user content do not leak into generated docs, logs, or validation artifacts.
  - Output: Redaction metadata and diagnostic fixture set.
  - Validation: Sentinel-secret tests fail if private values appear in generated docs, compatibility reports, fixture reports, logs, or validation outputs.

## Phase 6: Fixtures, Golden Tests, And Integration-Harness Contracts

### Work Items

- **6.1 Build valid fixture builders.**
  - Design: Generate deterministic valid fixtures for primitives, identity, tenant, command, error, event, audit, manifest, resource, queue, lease, usage, policy, and key metadata families.
  - Output: Fixture builders consumed by the integration test harness, SDK, CLI, service contract tests, and local stack.
  - Validation: Valid fixtures pass validators deterministically from a clean checkout and stable seed.

- **6.2 Build invalid and boundary fixture builders.**
  - Design: Generate invalid, boundary, deprecated, unsupported-version, unknown-field, missing-envelope, privacy-missing, malformed-ref, stale-reason-code, and migration-needed payloads.
  - Output: Negative fixture corpus for validator, parse, compatibility, and service contract tests.
  - Validation: Invalid fixtures fail with expected stable reason codes and do not produce ambiguous parser errors.

- **6.3 Build golden envelope fixtures.**
  - Design: Store canonical command, event, audit, usage, ledger, and API error fixture payloads that Rust, CLI, SDK, TypeScript/web, and later language bindings must preserve.
  - Output: Golden fixture corpus under approved specs/test paths.
  - Validation: Cross-target tests prove generated outputs round-trip golden fixtures without field loss, ordering drift, or error-shape changes.

- **6.4 Connect fixtures to SDS #3 and SDS #4 workflows.**
  - Design: Make fixtures resettable for the local development stack and reusable by the integration test harness without production credentials, real secrets, or private payload defaults.
  - Output: Local-stack fixture bundle and integration-harness contract.
  - Validation: Local reset tests prove fixture state is deterministic, redacted, and structurally impossible to enable as production data.

- **6.5 Publish validation artifacts.**
  - Design: Produce CI artifacts for schema lint, generated output diff, fixture pass/fail counts, redaction checks, and compatibility reports without treating those artifacts as Overwatch runtime events.
  - Output: Artifact naming, retention, and Docdex indexing expectations.
  - Validation: Progress evidence records artifact generation status and any blocker from missing test-runner configuration.

## Phase 7: Compatibility, Deprecation, And Migration Gates

### Work Items

- **7.1 Implement schema comparison.**
  - Design: Compare previous and next schema sets and classify changes as additive, deprecated, breaking, blocked, or migration-required per object family and field.
  - Output: `compareSchemas(previous, next)` style compatibility report.
  - Validation: Compatibility tests prove field removal, type narrowing, envelope change, signing-input change, and privacy-class changes are breaking unless migration metadata exists.

- **7.2 Implement deprecation metadata.**
  - Design: Require replacement mapping, consumer list, first deprecated version, last supported version, migration reason, owner, and support window for deprecated fields or objects.
  - Output: Deprecation metadata schema and generated documentation.
  - Validation: Release checks reject deprecated public fields without migration metadata and reject active consumers without support-window notes.

- **7.3 Enforce current-plus-previous stable major support.**
  - Design: Support current stable schema major plus one previous stable major once external native apps, adapters, mobile clients, or other external consumers depend on the package.
  - Output: Compatibility policy and unsupported-version reason-code mapping.
  - Validation: Contract tests require stable `schema_version_unsupported` or equivalent reason codes instead of silent downgrade behavior.

- **7.4 Gate authority-sensitive migrations.**
  - Design: Require formal migration plans before deprecating modules that affect runtime authority, auditability, rights, accounting finality, privacy, or cross-client compatibility.
  - Output: Migration-plan requirement for identity, tenant, command, API error, event, audit, queue, usage, ORU, Seal Ledger, policy, asset, namespace, credential, secret-ref, Overvault, Overbase, Overstore, native-app, mobile, AI, Docdex RAG, and ADES-facing contracts.
  - Validation: Release checks block deprecations in authority-sensitive modules without migration plan, consumer impact, rollback guidance, and stable error behavior.

- **7.5 Build consumer impact reports.**
  - Design: Track which services, SDKs, CLI surfaces, adapters, native apps, and test fixtures consume each schema module and field.
  - Output: Consumer registry view and impact report for schema changes.
  - Validation: CI fails when a breaking schema change lacks identified consumers, owner signoff, compatibility report, and migration notes.

## Phase 8: Downstream Domain Schema Expansion

### Work Items

- **8.1 Add execution and scheduling schemas.**
  - Design: Expand workload, resource, queue, lease, heartbeat, placement, cancellation, completion, result, package, benchmark, cache, and meter refs as Phase 3 services mature.
  - Output: Execution and scheduling schema modules with fixtures.
  - Validation: Downstream tests prove Overpack, Oversched, Overlease, Overrun, Overmeter, Overcell, and Overcache consume generated contracts rather than private duplicate types.

- **8.2 Add trust, policy, and verification schemas.**
  - Design: Add workload classification, policy dry-run, policy decision, challenge task, verification evidence, claim, reputation, anti-sybil, appeal, and data-class label schemas as Phase 4 services mature.
  - Output: Trust, policy, and verification schema modules.
  - Validation: Fixtures prove policy and verification decisions preserve reason codes, evidence refs, appeal refs, privacy class, and owner service refs.

- **8.3 Add accounting, rights, and settlement schemas.**
  - Design: Add usage rollups, metering dimensions, ORU refs, Seal Ledger refs, reversal refs, dispute refs, payout hold refs, grant refs, Overasset refs, receipt refs, and finality status as Phase 5 services mature.
  - Output: Accounting and rights schema modules with strict unknown-field rejection.
  - Validation: Tests prove accounting schemas stay structural, consume owning-service evidence refs, and do not embed charge tables, price schedules, revenue projections, or blockchain/NFT assumptions.

- **8.4 Add data, storage, namespace, and secret-ref schemas.**
  - Design: Add Overbase, Overstore, Overvault, namespace, route, directory listing, ownership evidence, upload/download grant, retention, backup, restore, and secret-reference schemas as Phase 8 services mature.
  - Output: Data/storage/namespace schema modules.
  - Validation: Validators reject raw secret values, untyped storage refs, conventional object-store assumptions, and namespace ownership records without evidence refs.

- **8.5 Add AI, Docdex, mobile, native-app, and ADES-facing schemas.**
  - Design: Add personal assistant request refs, model-routing decisions, encrypted Docdex index refs, RAG source refs, ADES extraction refs, mobile session refs, native-app view models, and central AI stewardship refs as Phases 6 and 12 mature.
  - Output: AI/RAG/mobile/native-app schema modules and TypeScript/web projection requirements.
  - Validation: Tests prove user content and AI context schemas use refs and privacy classes rather than raw private payload leakage.

## Phase 9: Package Release, CI Enforcement, And Consumer Registry

### Work Items

- **9.1 Define package release workflow.**
  - Design: Release a versioned package only after schema lint, generation, fixture, compatibility, redaction, docs, and consumer-impact checks pass.
  - Output: Release checklist, version metadata, generated artifacts, and blocked-release states.
  - Validation: CI prevents release when validators accept invalid payloads, generated output is stale, compatibility is unresolved, or public object docs are missing.

- **9.2 Enforce generated contract consumption.**
  - Design: Make service, SDK, CLI, worker, node-agent, UI, adapter, and test surfaces consume generated contracts for public objects instead of hand-written duplicate types.
  - Output: Contract-consumption lint and review checklist.
  - Validation: Checks fail public APIs, workers, or SDK surfaces that use untyped payloads, private schema forks, or ad hoc string parsing for structured objects.

- **9.3 Build schema coverage reports.**
  - Design: Report schema coverage by service, object family, build phase, privacy class, validation status, fixture coverage, and generated target.
  - Output: Coverage report used by Phase 0 exit and later phase gates.
  - Validation: Phase gates fail when a public boundary has no schema coverage or when a downstream service consumes unreleased draft contracts.

- **9.4 Build generated documentation publishing.**
  - Design: Publish schema-derived reference docs, reason-code docs, migration notes, compatibility reports, fixture examples, and consumer registry views as build artifacts.
  - Output: Generated docs bundle linked from service docs and implementation plans.
  - Validation: Markdown/link checks prove generated docs link back to source schemas, owning services, SDS docs, and build-plan phase gates.

- **9.5 Define Phase 6 product hardening.**
  - Design: During product integration, verify Docdex, Mcoda, Codali, admin/developer UI, CLI, SDK, adapters, AI gateway, and encrypted Docdex RAG consume generated schema outputs safely.
  - Output: Product-integration schema readiness checklist.
  - Validation: Phase 6 tests fail if product adapters or UI surfaces bypass generated contracts, Overgate command envelopes, stable errors, privacy classifications, or redaction rules.

## Phase 10: Validation, Documentation Alignment, And Downstream Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #7`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for canonical JSON plus JSON Schema authority, Rust-first generation, TypeScript/web second, Protobuf projection limits, strict validation, and no conventional cloud/product-boundary drift.
  - Output: Tech-stack alignment checklist for the Shared Schema Package.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #7 is represented as a Phase 0 foundation package with downstream expansion gated by owning service phases.
  - Output: Updated master-plan and crosswalk rows for SDS #7.
  - Validation: Review confirms only per-SDS sub-build indexing changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #7 and the Shared Schema Package service plan link back to this sub-build plan and preserve package-only, Phase 0, schema-source authority.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare downstream handoff rules.**
  - Design: Document how control-plane services, execution services, trust/policy services, accounting services, data/storage/namespace services, adapters, SDK, CLI, admin/developer UI, native apps, mobile, AI, and Docdex RAG consume schemas without bypassing their owning service authority.
  - Output: Downstream consumer rules and owner-gated schema expansion checklist.
  - Validation: Handoff review confirms later consumers can request new schemas through the package while runtime authority, audit finality, policy truth, accounting truth, storage truth, and secret custody remain in owning services.

## Alignment Review

- The sub-build plan keeps the Shared Schema Package's first build point in master Phase 0, matching SDS #7, the service catalog entry, Phase 0 plan, and the build-plan crosswalk.
- The plan treats master Phase 1 as the first major downstream consumer path for signed commands, tenant/identity/key metadata, Overregistry manifests, Overwatch audit/event records, and Overqueue records.
- The plan treats later phases as schema expansion gates owned by their services: execution in Phase 3, trust/policy in Phase 4, accounting/rights in Phase 5, product integration in Phase 6, data/storage/namespace in Phase 8, and native/mobile/AI clients in Phase 12.
- The plan carries forward SDS #7 resolved decisions: canonical JSON plus JSON Schema source, Rust generation first, TypeScript/web second, Protobuf only for compact internal RPC/event contracts, strict unknown-field rejection for security/accounting families, typed low-risk extension maps only, current-plus-previous stable major compatibility, and formal migration plans for authority-sensitive modules.
- The plan keeps schema authority narrow: no deployed runtime registry, no service-owned production state, no policy truth, no accounting finality, no audit finality, no secret custody, no storage ownership, and no conventional cloud product assumptions.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #7 is complete when a builder can implement the Shared Schema Package as a Phase 0 canonical contract package under `packages/schemas` with JSON plus JSON Schema source authority, Rust-first generated types and strict validators, deterministic fixtures and golden tests, stable reason codes and envelope assertions, generated docs, compatibility/deprecation/migration reports, consumer-impact tracking, TypeScript/web projections gated behind Rust and fixture stability, Protobuf projections limited to internal compact contracts, and downstream schema expansion rules that let later services consume shared contracts without moving runtime authority out of their owning services.
