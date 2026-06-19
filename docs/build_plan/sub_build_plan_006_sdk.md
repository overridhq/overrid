# SUB BUILD PLAN #6 - SDK

Attached SDS: [docs/sds/foundation/sdk.md](../sds/foundation/sdk.md)

## Purpose

This sub-build plan turns SDS #6 into an implementation sequence for the Overrid SDK. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

The SDK is a versioned client package, not a deployed runtime service, policy engine, secret store, accounting authority, or privileged API backdoor. Its job is to make Overrid calls safe and consistent by generating clients from shared contracts, enforcing request envelope rules, routing mutating calls through Overgate, delegating signing to credential providers, preserving trace and audit evidence, and giving product builders stable helpers for workloads, policy dry-runs, usage, receipts, and errors.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #6: SDK](../sds/foundation/sdk.md) | Controls SDK purpose, non-goals, actors, dependencies, local data model, API surface, state machine, security rules, validation, build breakdown, and resolved open-question decisions. |
| [SDK service plan](../service_catalog/foundation/sdk.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, API conventions, local development, and integration harness prerequisites; it does not make the SDK's first thin client build point Phase 0. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Controls the first build point for the thin Rust SDK because Overgate, Overkey-lite, Overtenant, Overpass-lite, Overwatch, and Overqueue define the first signed command path. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Controls hardening for real product use by Docdex, Mcoda, Codali, CLI, admin/developer UI, adapters, and product reliability tests. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDK first build work aligned to master Phase 1 with Phase 6 product-integration hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first generated SDK work first, TypeScript/web bindings after contract stability, and Overrid-owned primitives rather than conventional cloud product boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 1 with Phase 0 prerequisites | Attach SDS #6 to the build-plan layer, freeze the SDK boundary, and preserve resolved SDS decisions before implementation. |
| 2 | Master Phase 0 prerequisites and Master Phase 1 | Consume shared schemas, API descriptions, reason codes, and capability metadata needed by the first thin client. |
| 3 | Master Phase 1 | Generate the first Rust SDK package and typed client skeleton for the control-plane command path. |
| 4 | Master Phase 1 | Implement request context, command envelopes, idempotency, retries, state transitions, and stable error decoding. |
| 5 | Master Phase 1 | Implement credential-provider interfaces, canonical signing, redaction, and production/test separation without storing private material. |
| 6 | Master Phase 1 with later gates from Phases 3 and 4 | Add manifest, workload, status, result, and policy dry-run helpers without claiming runtime authority before owning services exist. |
| 7 | Master Phase 5 and Master Phase 6 | Add usage, ORU, Seal Ledger, receipt, and dispute-reference readers after accounting objects become available. |
| 8 | Master Phase 0 through Phase 6 validation | Build fixtures, golden tests, local-stack contract tests, cross-language gates, and validation artifacts. |
| 9 | Master Phase 6 with later Phase 12/mobile gates | Add TypeScript/web bindings and product convenience modules after stable schema generation; keep mobile/offline behavior in the Mobile SDK. |
| 10 | Master Phase 6 with handoff to later phases | Validate alignment, update documentation links, and define downstream expansion rules for adapters, native apps, mobile, and future bindings. |

## Tech Stack Guardrails

- The first binding is a generated Rust SDK because Overrid core, CLI, services, control-plane skeleton, and local tooling are Rust-first.
- TypeScript/web bindings are generated next for browser-facing UI, adapters, and product integrations only after shared schemas, generated models, error objects, and golden fixtures are stable.
- Swift/iOS, Kotlin/Android, Python, and other bindings remain blocked until cross-language contract tests prove request envelopes, signing inputs, idempotency behavior, error decoding, and manifest validation are identical.
- `packages/schemas` remains the source of truth for public objects. SDK generated code is an output, not a contract authority.
- Mutating calls must route through Overgate and carry actor id, tenant id, trace id, idempotency key, command type, timestamp, schema version, and signature metadata.
- Signing uses credential-provider interfaces and Overkey-compatible credential refs. The SDK must not persist private keys, bearer tokens, seed phrases, or vault values.
- Overvault-backed secret refs are reference-only for the general SDK until Phase 8 introduces Overvault. The SDK may pass declared secret refs and mount intents but must not emulate vault storage.
- Policy decisions belong to Overguard, policy dry-run services, and runtime admission flows; accounting truth belongs to Overmeter, ORU, Seal Ledger, Overbill, Overgrant, and Overasset.
- Local fake signers, fixture credentials, and bypasses are test-harness-only and must be structurally impossible to enable in production config.
- The SDK must not introduce PostgreSQL, Redis, S3, Vault, Kafka, NATS, blockchain, NFT, pricing, revenue, or customer-count assumptions.

## Phase 1: SDS Attachment, Boundary, And Version Gates

### Work Items

- **1.1 Attach the build plan to SDS #6.**
  - Design: Link this document from the numbered SDK SDS, SDK service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/foundation/sdk.md`, `docs/service_catalog/foundation/sdk.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #6 returns both the SDK SDS and this sub-build plan.

- **1.2 Freeze the SDK boundary.**
  - Design: Record that the SDK is a generated/versioned client package and local helper layer, not a runtime service, policy authority, secret store, billing service, queue, scheduler, or storage layer.
  - Output: Boundary guardrails documented in this plan and mirrored by implementation reviews.
  - Validation: Architecture review confirms no work item requires private internal endpoints, direct service storage, policy decisions inside the client, secret persistence, or direct payment behavior.

- **1.3 Preserve master Phase 1 as the first SDK build point.**
  - Design: Treat Phase 0 shared schemas, local stack, and harness as prerequisites, then start the thin SDK when Phase 1 Overgate, Overkey-lite, tenant, identity, audit, and queue contracts exist.
  - Output: Phase-gate note that the first Rust SDK becomes buildable in Phase 1 and receives Phase 6 product-integration hardening.
  - Validation: Review proves this plan does not move SDK product hardening into Phase 0 or bypass the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve the Rust-first binding, TypeScript/web second target, credential-provider-only signing, bounded idempotency cache retention, separate Mobile SDK boundary, and current-plus-previous stable major compatibility window.
  - Output: Implementation checklist tied to the SDS resolved decisions.
  - Validation: Review proves the plan does not re-open or contradict resolved open design questions in SDS #6.

- **1.5 Define SDK release and compatibility gates.**
  - Design: Require every released SDK major version to name schema versions, supported service capability profiles, deprecation behavior, upgrade guidance, and security-critical emergency break handling.
  - Output: Compatibility metadata and release checklist for current and previous stable major support.
  - Validation: Contract tests reject silent downgrade behavior and require stable `unsupported_sdk_version` or `schema_version_unsupported` errors.

## Phase 2: Contract Intake And Local Client Data Model

### Work Items

- **2.1 Consume shared schema and API descriptions.**
  - Design: Generate SDK inputs only from `packages/schemas`, approved API descriptions, and docs/specs contract records for command envelopes, API objects, events, errors, manifests, usage, receipts, and audit refs.
  - Output: Contract intake manifest naming source schema files, API descriptions, owning phase, generated output paths, and version metadata.
  - Validation: `schema:check` and SDK generation fail when a public SDK object has no source schema or when generated output is newer than source contracts.

- **2.2 Define local SDK configuration records.**
  - Design: Model `sdk_config`, environment, base URL, timeout budget, retry policy, feature flags, default tenant, client identity reference, and service capability profile as local typed records.
  - Output: Versioned config schema with production/test environment separation and explicit live-endpoint confirmation.
  - Validation: Tests reject missing environment, implicit production endpoint use, unknown feature flags, and config that enables test fixtures in production.

- **2.3 Define request and credential reference records.**
  - Design: Model `credential_ref`, `request_context`, `signed_request`, and replay-window metadata without raw private material or bearer tokens.
  - Output: Typed records for actor id, tenant id, trace id, idempotency key, command type, schema version, credential id, signature metadata, body hash, timestamp, and replay window.
  - Validation: Serialization tests prove required mutating-call fields are present before signing and redaction tests prove secret-like fields are absent from durable SDK state.

- **2.4 Define idempotency and error records.**
  - Design: Model `idempotency_entry` and `overrid_error` with request hash, terminal response digest, trace id, audit refs, retryability, correction fields, dependency name, policy refs, and stable reason codes.
  - Output: Local cache and typed error schemas with command-class retention fields.
  - Validation: Fixture tests cover accepted replay, timeout retry, conflicting request hash, terminal denial, and unsupported schema version.

- **2.5 Define capability and schema-version negotiation.**
  - Design: Let clients read gateway capability profiles and compare supported schema, SDK major, signing, idempotency, dry-run, and accounting features before using optional helpers.
  - Output: Capability reader and compatibility decision table.
  - Validation: Tests prove unsupported optional helpers fail with stable local errors before unsafe network calls.

## Phase 3: Generated Rust SDK Skeleton

### Work Items

- **3.1 Create the Rust SDK package boundary.**
  - Design: Implement the first SDK under the Rust-first workspace as a generated-and-handwritten split: generated models/validators from contracts plus thin handwritten client orchestration.
  - Output: `packages/sdk` Rust crate structure, module ownership, generated-output paths, and public API entrypoints.
  - Validation: Layout checks confirm generated code is not the source of truth and SDK modules depend on schemas rather than private service structs.

- **3.2 Generate typed models and validators.**
  - Design: Generate Rust types, validators, stable enum mappings, reason-code objects, and schema-version constants from contract inputs.
  - Output: Generated Rust models for initial Phase 1 command, tenant, identity, key, manifest, queue, audit, and error objects.
  - Validation: Golden schema fixtures validate successfully and negative fixtures fail with stable reason codes.

- **3.3 Implement client construction.**
  - Design: Provide `configureClient(config)` semantics in Rust with base URL, environment, retry policy, credential provider, timeout budget, trace policy, and redaction defaults.
  - Output: Typed client builder and immutable client configuration object.
  - Validation: Unit tests prove invalid config fails before network use and valid local/test config can target only loopback local stack unless explicitly configured otherwise.

- **3.4 Implement read helpers for Phase 1 public control-plane APIs.**
  - Design: Add typed readers for accepted control-plane objects that are safe before execution exists, such as identity, tenant, key metadata, manifest, queue status, and audit refs where exposed.
  - Output: Read-only API helpers with stable pagination, response decoding, and trace propagation.
  - Validation: Contract tests prove readers preserve schema versions, pagination cursors, request ids, and audit refs.

- **3.5 Implement SDK version reporting.**
  - Design: Make the client report SDK name, major/minor/patch, schema set, generated-contract revision, supported feature flags, and language binding.
  - Output: Version metadata surfaced on requests and through local diagnostics.
  - Validation: Gateway compatibility tests can reject unsupported SDK versions with stable upgrade guidance.

## Phase 4: Command Pipeline, Idempotency, Retry, And Errors

### Work Items

- **4.1 Implement command-envelope builders.**
  - Design: Provide `buildCommand(input)` semantics that require tenant, actor, command type, schema version, trace id, idempotency key, timestamp, and payload validation for mutating calls.
  - Output: Typed command builder with deterministic canonical payload and validation results.
  - Validation: Golden tests prove envelope canonicalization is stable and required fields cannot be omitted before signing.

- **4.2 Implement Overgate submission.**
  - Design: Provide `submitCommand(command)` semantics that route all mutating calls through Overgate and decode accepted, denied, duplicate, retryable, and terminal failure responses.
  - Output: Submission client that never calls private service storage or internal worker endpoints.
  - Validation: Local-stack contract tests submit a signed synthetic workload command and prove audit refs, queue refs, and trace ids are preserved.

- **4.3 Implement bounded idempotency cache behavior.**
  - Design: Use SDS-defined command-class retention: no cache for read-only calls, 24-hour successful terminal digests for Phase 1 mutating commands, shorter in-flight retry records, 7-day workload submission refs, and stricter security-sensitive retention.
  - Output: Local idempotency cache with request hashes, terminal digests, cleanup controls, and user-clearable state.
  - Validation: Tests cover successful replay, retry after timeout, conflicting request hash, cache discard, and local `dev reset` cleanup.

- **4.4 Implement retry classification and state transitions.**
  - Design: Apply bounded retries only for safe transport failures or service responses explicitly marked retryable; preserve the SDS state machine from prepared through completed.
  - Output: Retry classifier and request lifecycle state machine.
  - Validation: Tests prove denied, failed, duplicate, retry_wait, accepted, and completed states cannot be inferred without service responses.

- **4.5 Implement stable error decoding.**
  - Design: Decode service errors into `overrid_error` objects that preserve reason code, message, trace id, audit refs, retryable flag, correction fields, policy refs, and dependency name.
  - Output: Error decoder and caller-facing error types.
  - Validation: Contract tests prove SDK error objects preserve reason codes and trace ids for invalid signature, duplicate idempotency conflict, unsupported schema, revoked credential, and policy denial fixtures.

## Phase 5: Credential Providers, Signing, And Security Guardrails

### Work Items

- **5.1 Define credential-provider interfaces.**
  - Design: Implement provider traits for file-backed local/test credentials, host signing-agent sockets, platform keychains where available, hardware-backed signing devices or secure enclaves, and Overkey credential refs.
  - Output: Provider interface returning credential id, public metadata, signing capability, rotation/revocation hints, and redaction class.
  - Validation: Tests prove raw private keys, bearer tokens, seed phrases, and vault values never enter SDK durable state or default logs.

- **5.2 Implement canonical request signing.**
  - Design: Provide `signRequest(command, credentialRef)` semantics that canonicalize method, path, headers, body hash, timestamp, schema version, credential id, and replay metadata before delegating signing.
  - Output: Signing adapter and canonicalization fixtures.
  - Validation: Golden tests verify stable signing input bytes and reject mutated payload, wrong tenant, expired timestamp, mismatched credential id, and unsupported algorithm fixtures.

- **5.3 Separate test signers from production configuration.**
  - Design: Keep fake signers and fixture credentials visually and structurally separate from production config and require explicit local/test harness installation.
  - Output: Test-only signer module and production guard that refuses fixture providers outside local/test modes.
  - Validation: Security tests prove test signers cannot be enabled in production configuration or through ambiguous environment defaults.

- **5.4 Implement redacted diagnostics.**
  - Design: Emit local diagnostics for request_built, request_signed, request_sent, response_received, retry_scheduled, request_denied, request_failed, and duplicate_resolved with trace and idempotency refs but no secret or private payload content.
  - Output: Structured logging and metrics hooks with redaction defaults.
  - Validation: Redaction tests inject sentinel secrets, signatures, private payloads, and tokens and fail if any appear in logs, metrics, diagnostics, or validation artifacts.

- **5.5 Handle credential lifecycle failures.**
  - Design: Treat expired, revoked, mismatched, unknown, or insufficient credentials as terminal client or service errors unless Overgate returns an explicit retryable correction path.
  - Output: Credential error mapping and refresh guidance hooks.
  - Validation: Contract tests cover revoked credential, rotated credential, missing credential ref, host signer unavailable, and retry-prohibited signing failure.

## Phase 6: Workload, Manifest, Status, And Policy Helpers

### Work Items

- **6.1 Implement workload manifest builders.**
  - Design: Provide `buildWorkloadManifest(input)` semantics for workload, resource, data, policy, egress, output, and secret-reference declarations while validating locally without claiming runtime acceptance.
  - Output: Typed builder and local validation result types.
  - Validation: Manifest tests prove invalid resources, missing schema version, malformed secret refs, forbidden egress declarations, and unsupported workload classes fail before submission.

- **6.2 Implement workload submission helpers.**
  - Design: Provide `submitWorkload(manifest)` semantics that wrap manifest validation, command envelope creation, signing, Overgate submission, and accepted/denied response decoding.
  - Output: Workload submission client for Phase 1 pending-queue flow and later Phase 3 execution flow.
  - Validation: Local-stack contract tests prove a signed workload reaches pending queue state without manual internal API calls.

- **6.3 Implement status, result, and cancellation readers.**
  - Design: Add `getCommandStatus`, job/workload status readers, result readers, and cancellation helpers only where owning service APIs expose public control-plane paths.
  - Output: Typed readers for accepted command refs, trace ids, queue state, result refs, cancellation decisions, and final states.
  - Validation: Tests prove readers do not invent completed states and preserve failed, cancelled, timed-out, duplicate, and dead-letter distinctions returned by services.

- **6.4 Implement policy dry-run helper.**
  - Design: Provide `dryRunPolicy(input)` only after the Phase 4 policy dry-run API exists, returning matched policy refs, reason codes, estimated placement class, and correction fields without mutating runtime state.
  - Output: Dry-run client behind capability checks.
  - Validation: Tests prove dry-run helpers fail closed before Phase 4 capability is available and never cache dry-run results as policy truth.

- **6.5 Keep runtime authority outside the SDK.**
  - Design: Ensure workload, manifest, status, result, cancellation, and dry-run helpers are ergonomic wrappers over approved public APIs, not alternate schedulers, policy engines, or direct storage readers.
  - Output: Authority boundary review checklist.
  - Validation: Architecture review confirms no helper bypasses Overgate, Overguard, Overqueue, Overrun, Overmeter, or Overwatch ownership.

## Phase 7: Usage, Receipt, ORU, And Dispute Readers

### Work Items

- **7.1 Add usage receipt view models.**
  - Design: Introduce `usage_receipt_view` readers for usage rollups, ORU charge previews, Seal Ledger references, dispute refs, and receipt refs once Phase 5 accounting APIs exist.
  - Output: Generated accounting response models and typed reader helpers.
  - Validation: Schema tests prove accounting helpers consume service-returned objects and do not embed local charge tables or pricing logic.

- **7.2 Preserve metering and accounting refs.**
  - Design: Attach caller app id, actor id, tenant id, trace id, and idempotency refs so runtime services can meter accurately, and surface Overmeter, ORU, Seal Ledger, Overbill, Overgrant, and Overasset refs unchanged.
  - Output: Request metadata propagation and accounting ref decoders.
  - Validation: Contract tests prove usage, charge preview, receipt, hold, refund/correction, grant, and asset refs round-trip without SDK rewriting.

- **7.3 Add dispute and correction-reference helpers.**
  - Design: Provide readers for dispute refs, correction refs, challenge windows, provider payout hold refs, and refund refs when runtime services return them.
  - Output: Dispute-reference reader and typed error/correction surfaces.
  - Validation: Tests prove disputed usage surfaces stable refs and does not hide holds, corrections, or denied settlement behind generic errors.

- **7.4 Block direct payment and authority leakage.**
  - Design: Keep accounting helpers structural and read-oriented; the SDK must not call external payment providers for platform operations or mutate accounting state outside approved Overgate commands.
  - Output: Guardrail documented in SDK API review and code ownership checks.
  - Validation: Review rejects direct payment-provider SDK dependencies, embedded price tables, revenue forecasts, and client-side settlement decisions.

- **7.5 Gate accounting helpers behind Phase 5 and Phase 6 readiness.**
  - Design: Make usage and receipt helpers optional until accounting services are present, then harden them during Phase 6 product integration.
  - Output: Capability-gated accounting module.
  - Validation: Product tests prove Docdex, Mcoda, or Codali can show usage and receipts only through service-returned accounting evidence.

## Phase 8: Fixtures, Contract Tests, And Validation Artifacts

### Work Items

- **8.1 Build local SDK fixtures.**
  - Design: Provide local/test fixtures for tenants, actors, credentials, command envelopes, manifests, signatures, idempotency entries, errors, usage refs, and audit refs.
  - Output: Fixture package aligned with SDS #3 Integration Test Harness and SDS #4 Local Development Stack.
  - Validation: Fixture tests prove generated fixtures are deterministic, redacted, resettable, and never production defaults.

- **8.2 Build SDK contract tests against the local stack.**
  - Design: Run SDK tests through canonical local stack and public Overgate/control-plane APIs, not internal service mocks, when the owning services exist.
  - Output: Contract-test suite for signed command submission, duplicate idempotency, stable error preservation, and status reads.
  - Validation: `test:integration` or equivalent root command passes for the SDK slice once the repo has test-runner wiring.

- **8.3 Build golden cross-language fixtures.**
  - Design: Store golden request envelopes, canonical signing inputs, response errors, manifest validations, idempotency cases, and redaction cases that every language binding must pass.
  - Output: Golden fixture corpus under approved specs/test paths.
  - Validation: Rust passes first; TypeScript/web and later bindings cannot release until they pass the same corpus.

- **8.4 Build security and redaction tests.**
  - Design: Test production/test separation, fake signer isolation, secret redaction, private payload logging, credential lifecycle failures, unsupported schema, and unsafe downgrade attempts.
  - Output: Security regression tests and validation artifact schemas.
  - Validation: Tests fail if secrets, raw request bodies, signatures, private content, or fixture credentials appear in logs or artifacts.

- **8.5 Define SDK validation artifacts.**
  - Design: Produce build/CI artifacts for schema generation, contract tests, signing golden checks, idempotency behavior, redaction checks, compatibility checks, and docs alignment without treating them as Overwatch runtime events.
  - Output: Artifact names, retention rules, and Docdex-indexing expectations.
  - Validation: Progress evidence records which checks ran, which are blocked by missing test-runner config, and which docs were refreshed in Docdex.

## Phase 9: TypeScript/Web Bindings And Product Integration Hardening

### Work Items

- **9.1 Generate TypeScript/web bindings after Rust stability.**
  - Design: Generate TypeScript/web models, validators, error objects, and request helpers from the same shared contracts after Rust fixtures and schema generation stabilize.
  - Output: TypeScript/web binding package for UI, adapter, and browser-facing client surfaces.
  - Validation: Cross-language fixture tests prove TypeScript/web request envelopes, errors, manifests, and idempotency behavior match Rust.

- **9.2 Define adapter and UI safety boundaries.**
  - Design: Let adapters, admin/developer UI, and browser surfaces call only approved Overgate/admin APIs through generated clients, without privileged internal endpoints or hidden service shortcuts.
  - Output: Browser/UI safety profile and adapter helper boundary.
  - Validation: Architecture review rejects UI/adapters that bypass Overgate, store private keys, or depend on internal worker/storage APIs.

- **9.3 Harden product convenience modules.**
  - Design: Add convenience helpers for Docdex encrypted RAG jobs, Mcoda agent workloads, Codali code-agent workloads, package validation, job status, cancellation, results, usage, receipts, and product failure cases during Phase 6.
  - Output: Product-facing SDK module set behind feature flags and capability checks.
  - Validation: Product-driven tests cover successful job, retryable failure, final failure, cancellation, timeout, policy denial, budget exhaustion, node disconnect, and disputed usage.

- **9.4 Keep mobile/offline behavior in the Mobile SDK.**
  - Design: Provide extension points and shared primitives for mobile session refresh, secure storage adapters, bounded offline queues, sync cursors, conflict states, and redacted diagnostics, but leave mobile-specific packaging to Phase 12 Mobile SDK.
  - Output: Mobile extension interfaces and fixture hooks without default offline queues in the foundation SDK.
  - Validation: Review confirms the foundation SDK does not embed mobile session management, push registration, OS background behavior, media upload state, or offline command queues.

- **9.5 Gate additional language bindings.**
  - Design: Add Swift/iOS, Kotlin/Android, Python, or other bindings only after schema compatibility checks, golden request-envelope fixtures, generated error objects, and cross-language contract tests are stable.
  - Output: Binding-readiness checklist and release gate.
  - Validation: Release checks reject handwritten public objects, partial schema coverage, missing golden tests, or language-specific behavior drift.

## Phase 10: Validation, Documentation Alignment, And Downstream Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #6`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first SDK generation, TypeScript/web second target, language-neutral schema authority, Overgate-only mutating calls, credential-provider signing, and no conventional cloud/product-boundary drift.
  - Output: Tech-stack alignment checklist for SDK.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #6 is represented as a Phase 1 foundation SDK with Phase 6 product-integration hardening.
  - Output: Updated master-plan and crosswalk rows for SDS #6.
  - Validation: Review confirms only per-SDS sub-build indexing changed; no phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #6 and the SDK service plan link back to this sub-build plan and preserve thin-client Phase 1 plus Phase 6 product-integration hardening.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare downstream phase handoff.**
  - Design: Document how CLI, admin/developer UI, adapters, Docdex, Mcoda, Codali, native apps, mobile services, Overpack deployment, accounting, policy, and future language bindings consume SDK primitives without bypassing owning services.
  - Output: Handoff rules for downstream consumers and later master phases.
  - Validation: Handoff review confirms later consumers can use SDK helpers through public APIs while policy, accounting, secrets, storage, scheduling, and runtime authority remain in their owning services.

## Alignment Review

- The sub-build plan keeps the SDK's first thin-client build point in master Phase 1, matching the SDS, service catalog entry, Phase 1 plan, and build-plan crosswalk.
- The plan treats master Phase 0 as a prerequisite provider for schemas, local stack, integration harness, and API conventions rather than the SDK's first implementation phase.
- The plan treats master Phase 6 as product-integration hardening for Docdex, Mcoda, Codali, CLI, adapters, and admin/developer UI.
- The plan carries forward SDS #6 resolved decisions: Rust SDK first, TypeScript/web second after schema stability, no handwritten public-object drift, credential-provider interfaces instead of key storage, bounded idempotency retention, separate Phase 12 Mobile SDK behavior, and current-plus-previous stable major compatibility.
- The plan keeps SDK authority narrow: no policy truth, no secret persistence, no direct accounting authority, no internal service endpoint shortcuts, no production local-dev bypasses, and no conventional cloud product assumptions.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #6 is complete when a builder can implement the SDK as a Rust-first, schema-generated, capability-gated client package that submits signed tenant-scoped commands through Overgate, delegates signing to credential providers, handles idempotency and stable errors predictably, validates workload manifests locally without claiming runtime authority, surfaces audit, usage, receipt, and dispute refs from owning services, provides deterministic fixtures and contract tests, gates TypeScript/web and later bindings behind cross-language compatibility, and gives downstream products a safe public API path without service shortcuts or tech-stack drift.
