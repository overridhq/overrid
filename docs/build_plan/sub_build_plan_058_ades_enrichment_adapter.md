# SUB BUILD PLAN #58 - ADES Enrichment Adapter

Attached SDS: [docs/sds/ai_rag_model_routing/ades_enrichment_adapter.md](../sds/ai_rag_model_routing/ades_enrichment_adapter.md)

## Purpose

This sub-build plan turns SDS #58 into an implementation sequence for ADES Enrichment Adapter. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

ADES Enrichment Adapter is the Phase 12 optional local semantic enrichment bridge for Personal AI Assistant, AI Gateway Router, Lightweight Classifier, Encrypted Docdex RAG Adapter, native apps, SDK, CLI, admin UI, Overwatch, and Overmeter. It owns ADES connector configuration, local service health, approved domain-pack metadata, enrichment request envelopes, privacy guard records, local-only input handling, normalized entity/topic/warning outputs, timing metadata, routing hint bundles, degradation records, usage refs, and audit refs. It does not make ADES a required Overrid protocol dependency, persist raw private text, fetch unapproved packs during private requests, send private or regulated content to non-local services, make final policy/safety/fraud/routing/model decisions, replace encrypted Docdex RAG, replace Lightweight Classifier, replace AI Gateway Router, replace Central AI Service, or introduce conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #58: ADES Enrichment Adapter](../sds/ai_rag_model_routing/ades_enrichment_adapter.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [ADES Enrichment Adapter plan](../service_catalog/ai_rag_model_routing/ades_enrichment_adapter.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [ADES README](https://raw.githubusercontent.com/bekirdag/ades/refs/heads/main/README.md) | Supplies only upstream ADES capability context: local-first tagging, local package/CLI/local HTTP service modes, domain packs, millisecond timing fields, and `/healthz`; Overrid policy and contracts remain controlled by this repo. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service facts, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard privacy/data-class decisions, Workload Classifier facts, Policy Dry-Run previews, and deny-by-default policy semantics for enrichment mode selection. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger accounting handoff boundaries, and no per-call pricing or direct accounting mutation inside the adapter. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies encrypted Docdex RAG and model-routing product context used as earlier integration substrate, without moving ADES first build work out of Phase 12. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overstore and Overvault refs for authorized content refs, protected context, hash refs, retention classes, and metadata-only replay. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first build point: Personal AI Assistant and native-app enrichment using optional local ADES hints. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies central AI boundaries, compliance, threat review, security review, incident handling, reporting, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #58 first build work aligned to master Phase 12, with earlier phases as prerequisites and Phase 13 as governance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 12, and 13 | Attach SDS #58, preserve Phase 12 as first build, freeze optional-adapter authority boundaries, and identify owner-service gates. |
| 2 | Master Phases 0, 1, 4, 8, and 12 | Define Rust contracts, canonical schemas, connector modes, lifecycle states, reason codes, signed refs, redaction profiles, and deterministic fixtures. |
| 3 | Master Phases 1, 4, and 12 | Implement ADES connector configuration, package/CLI/local HTTP health checks, `/healthz` behavior, pack inventory, approval records, and diagnostics. |
| 4 | Master Phases 4, 8, and 12 | Implement privacy guard records, local-only input handling, redaction, metadata-only retention, and policy denial semantics. |
| 5 | Master Phases 4, 8, and 12 | Implement bounded enrichment execution, normalization, timing metadata, result lifecycle, and safe retained outputs. |
| 6 | Master Phases 4, 6, 8, and 12 | Implement advisory routing hint bundles and downstream handoffs to AI Gateway Router, Lightweight Classifier, Personal AI Assistant, and Encrypted Docdex RAG Adapter. |
| 7 | Master Phases 4, 8, and 12 | Implement degradation, fallback behavior, stable errors, timeout handling, invalid-output handling, and confidence lowering. |
| 8 | Master Phases 1, 5, 8, 12, and 13 | Implement usage refs, Overwatch audit events, replay bundles, health/pack diagnostics, operator visibility, and reporting hooks. |
| 9 | Master Phases 6, 8, 12, and 13 | Prove the Phase 12 personal assistant/native app enrichment path and define later governance, regulated-domain, mobile, and central-AI hardening gates. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- ADES Enrichment Adapter core is a Rust service/module using shared contract crates, Tokio for bounded connector/timeout workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Connector configs, pack refs, enrichment requests/results, routing hint bundles, privacy guard records, degradation records, usage refs, events, fixtures, replay bundles, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating APIs require signed actor, app, service, or operator envelopes, tenant scope, privacy mode, data class, trace id, idempotency key, policy refs, pack refs, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for ephemeral input hashes, normalized output hashes, pack inventory snapshots, config snapshots, degradation records, routing hint bundles, replay bundles, audit exports, and deterministic fixtures.
- The adapter may call ADES in approved local package, CLI, or local HTTP service mode. ADES remains optional and local-first; it must not become the Overrid protocol boundary, policy authority, routing authority, model authority, safety authority, fraud authority, or final classifier.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw private text storage, remote private enrichment, unapproved pack fetches, final policy/routing/model/safety/fraud decisions, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Phase 12 Scope, And Authority Boundaries

### Work Items

- **1.1 Attach the build plan to SDS #58.**
  - Design: Link this document from the ADES Enrichment Adapter SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/ai_rag_model_routing/ades_enrichment_adapter.md`, `docs/service_catalog/ai_rag_model_routing/ades_enrichment_adapter.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #58 returns both the ADES Enrichment Adapter SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep first implementation in Phase 12 because ADES enrichment exists to improve native assistant, native app, and model-routing flows after identity, policy, metering, storage, RAG, and deployment rails are available.
  - Output: Phase-gate note that earlier phases supply contracts, policy, metering, product integrations, and protected refs, Phase 12 builds the adapter, and Phase 13 hardens governance/security/compliance.
  - Validation: Review proves the plan does not move ADES core work into Phases 0 through 11, does not delay the adapter to Phase 13, and does not reorder the master Phase 0 through Phase 13 sequence.

- **1.3 Freeze adapter ownership boundaries.**
  - Design: Record that the adapter owns connector config, local health, approved pack metadata, request envelopes, privacy guards, local-only input handling, normalized enrichment outputs, hint bundles, degradations, usage refs, and audit refs.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the adapter does not own final policy decisions, final route decisions, model selection authority, safety adjudication, fraud adjudication, encrypted RAG truth, central AI authority, accounting state, or raw private content retention.

- **1.4 Carry forward resolved SDS #58 decisions.**
  - Design: Preserve `general-en` as the only default fallback pack, require finance/medical/specialized packs to be local-only and opt-in behind explicit policy, default private-request retention to metadata-only, return label-level hints by default, cap raw ephemeral text at 8 KiB UTF-8, and avoid sensitive-pack substitution.
  - Output: Resolved-decision checklist tied to pack fallback, pack policy, retention, span disclosure, input caps, large-document handoff, and sensitive-domain behavior.
  - Validation: Review rejects silent truncation, raw span emission to routers/logs, medical/finance pack auto-substitution, remote private enrichment, unapproved pack fetching, and raw private text persistence.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for ADES local package/CLI/local HTTP service, AI Gateway Router, Lightweight Classifier, Personal AI Assistant, Encrypted Docdex RAG Adapter, Overguard, Overvault, Overwatch, Overmeter, SDK, CLI, admin UI, Mobile SDK, Mobile Backend Gateway, Central AI Service, and Compliance Boundary Service.
  - Output: Boundary matrix listing owner, input refs, output refs, privacy mode, pack policy, retention rule, redaction class, policy refs, usage refs, downstream consumer, and rejection behavior.
  - Validation: Review confirms every handoff uses explicit APIs, immutable refs or hashes, signed evidence, stable reason codes, trace ids, idempotency keys, policy refs, and Overwatch events rather than privileged shared records or hidden control paths.

## Phase 2: Rust Contracts, Schemas, Connector Modes, And Fixtures

### Work Items

- **2.1 Create the ADES Enrichment Adapter Rust contract module.**
  - Design: Add contract types for connector config, domain pack ref, enrichment request, enrichment result, routing hint bundle, privacy guard record, degradation record, usage ref, event payload, redaction profile, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, connector-mode enums, privacy-mode enums, lifecycle enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from AI Gateway Router internals, Lightweight Classifier internals, encrypted Docdex RAG storage, Overguard policy internals, Overmeter accounting mutation, and Central AI authority.

- **2.2 Define connector and domain-pack schemas.**
  - Design: Model `ades_connector_config` and `domain_pack_ref` with connector mode, executable/service refs, local endpoint, version, timeout, max input size, allowed pack refs, pack name, pack version, language, domain, dependency refs, approval status, local install state, and policy refs.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and canonical connector/pack fixtures.
  - Validation: Schema tests reject missing connector mode, non-local private endpoint, missing `/healthz` status for local HTTP mode, missing timeout, missing max input size, missing pack approval, missing policy refs, or unknown pack lifecycle state.

- **2.3 Define request, result, and hint schemas.**
  - Design: Model `enrichment_request`, `enrichment_result`, and `routing_hint_bundle` with actor refs, tenant refs, privacy mode, input ref or ephemeral hash, requested packs, max output size, entities, topics, warnings, timing in milliseconds, confidence fields, output hash, model-size hints, tool-need hints, RAG-scope hints, and explanation refs.
  - Output: Request schema, result schema, hint schema, stable error catalog, redacted examples, and negative fixtures.
  - Validation: Tests reject raw private text persistence, missing privacy mode, missing data class, missing pack refs, missing trace id, missing idempotency key, timing fields not expressed in milliseconds, unbounded output, and hint payloads that imply final route or model authority.

- **2.4 Define privacy, degradation, usage, event, and replay schemas.**
  - Design: Model `privacy_guard_record`, `degradation_record`, `usage_ref`, events, and replay bundles with data class, locality requirement, redaction outcome, policy refs, denied fields, missing ADES, failed health check, missing pack, timeout, invalid output, connector mode, duration class, input/output size class, and audit refs.
  - Output: Privacy schema, degradation schema, usage schema, event schema, replay schema, role-scoped redaction profiles, and replay fixtures.
  - Validation: Tests prove events never include raw private text or raw files, usage refs cannot be interpreted as price schedules, replay reconstructs decisions from refs and hashes, and degradation outputs cannot be charged as successful enrichment work.

- **2.5 Create deterministic ADES adapter fixtures.**
  - Design: Build fixtures for local package mode, CLI mode, local HTTP mode, `/healthz` success, health failure, `general-en` default fallback, finance pack opt-in, medical pack opt-in, missing pack, unapproved pack, invalid ADES output, timeout, 8 KiB cap, metadata-only retention, redacted span return, and route hint emission.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected reason codes, BLAKE3 hashes, redacted views, policy refs, usage refs, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, denial/degradation reason codes, audit refs, redacted outputs, and replay outputs across repeated runs.

## Phase 3: Connector Health, Pack Inventory, And Approval Controls

### Work Items

- **3.1 Implement connector configuration APIs.**
  - Design: Add APIs or commands for registering approved local ADES connector modes with executable refs, local HTTP endpoint, timeout, max input size, allowed pack refs, version facts, signed envelope, trace id, idempotency key, and audit refs.
  - Output: Connector config handler, request/response schemas, signed envelope checks, idempotency behavior, config snapshot hashes, stable errors, and `ades_enrichment_adapter.connector_configured` events.
  - Validation: API tests cover valid package/CLI/local HTTP config, duplicate idempotency key, non-local endpoint for private flows, missing timeout, missing max input size, invalid mode, stale version refs, and audience-safe errors.

- **3.2 Implement local health checks.**
  - Design: Check ADES package availability, CLI invocation readiness, local HTTP `/healthz`, version facts, timeout budget, and fallback mode eligibility without attempting remote enrichment.
  - Output: Health checker, health status records, connector version refs, failure reason codes, operator diagnostics, and `ades_enrichment_adapter.health_checked` events.
  - Validation: Tests cover healthy package mode, healthy CLI mode, healthy local HTTP mode, missing executable, failed `/healthz`, version mismatch, timeout, invalid health payload, and switch-to-approved-local-mode behavior.

- **3.3 Implement pack inventory refresh.**
  - Design: Add `GET /ades/packs` and `POST /ades/packs/refresh` behavior for listing installed, approved, unavailable, missing, deprecated, blocked, and dependency packs without pulling unapproved packs during private requests.
  - Output: Pack inventory projection, refresh command, local install-state records, dependency refs, approval status, stale inventory alerts, and `ades_enrichment_adapter.pack_inventory_updated` events.
  - Validation: Tests prove refresh never fetches unapproved packs during private enrichment, reports missing dependencies, marks deprecated packs, preserves approval refs, and emits stable reason codes for unavailable packs.

- **3.4 Implement pack approval policy checks.**
  - Design: Require explicit pack approval, local-only flags, domain restrictions, language/locale constraints, user/tenant permission, Overguard data-class checks, and default fallback eligibility before enrichment uses a pack.
  - Output: Pack approval checker, policy fact adapter, fallback matrix, denial reason codes, confidence lowering rules, and approval fixtures.
  - Validation: Tests prove `general-en` is the only default fallback, finance/medical/specialized packs are opt-in, locale-specific packs stay disabled until approved fixtures exist, and missing sensitive packs do not widen routing or policy authority.

- **3.5 Publish connector and pack diagnostics.**
  - Design: Provide diagnostics for connector mode, health status, version, timeout, pack inventory, pack approval, pack missing/deprecated/blocked state, policy denials, fallback behavior, and high-use pack health.
  - Output: Diagnostic APIs, operator projections, admin UI inputs, alert refs, public-safe summaries, and Overwatch timeline refs.
  - Validation: Tests prove diagnostics are role-scoped, actionable, and free of raw private text, raw files, sensitive pack contents, private entity values, or internal safety/fraud facts.

## Phase 4: Privacy Guards, Local-Only Input Handling, And Redaction

### Work Items

- **4.1 Implement privacy preflight.**
  - Design: Check actor/app/tenant refs, privacy mode, data class, locality requirement, retention rule, requested pack refs, max input size, and Overguard policy refs before any ADES invocation.
  - Output: Privacy preflight engine, policy input facts, allow/deny/degrade decisions, stable reason codes, and `ades_enrichment_adapter.enrichment_denied` events.
  - Validation: Tests prove private and regulated content can run only in approved local modes, missing privacy mode denies, remote/non-local paths deny with `locality_required`, and policy denials happen before ADES sees input.

- **4.2 Implement ephemeral input handling.**
  - Design: Accept ephemeral text or authorized content refs, compute BLAKE3 hashes, enforce 8 KiB UTF-8 raw text cap, route larger documents through Encrypted Docdex RAG Adapter or explicit local batch/chunking grants, and drop raw input after processing.
  - Output: Input handler, size checker, content-ref guard, hash records, large-input handoff records, `input_too_large` errors, and retention cleanup hooks.
  - Validation: Tests prove no silent truncation, no raw private input persistence, no unapproved file reads, no content-ref bypass, and no raw payload leakage into logs, events, metrics, usage refs, or replay bundles.

- **4.3 Implement redaction profiles.**
  - Design: Return label-level hints by default, expose raw spans only to the original local/private caller under the same permission and retention policy, and provide redacted spans or span offsets when highlighting/replay/correction needs them.
  - Output: Redaction profile catalog, role-scoped result views, span-disclosure checker, public/operator event redaction, and safe highlight outputs.
  - Validation: Contract tests prove AI Gateway Router, Lightweight Classifier, logs, metrics, public events, and operator views receive labels/categories/confidence refs rather than raw spans or private entity values.

- **4.4 Implement metadata-only retention.**
  - Design: Persist request/result ids, actor/tenant/app refs, privacy mode, data class, input/output hashes, pack refs and versions, connector mode/version, timing, size classes, label/topic/warning categories, confidence classes, degradation/denial reason codes, policy refs, Overwatch refs, and usage refs.
  - Output: Metadata retention records, expiry/redaction lifecycle, owner-encrypted artifact handoff option, retention policy refs, and replay-safe projections.
  - Validation: Tests prove raw private text, raw files, raw spans, secrets, unredacted private entity values, and unauthorized content refs are ephemeral unless the owner explicitly stores an encrypted, time-bounded artifact through the owning storage/context service.

- **4.5 Implement privacy diagnostics and correction paths.**
  - Design: Provide diagnostics for locality denials, unsupported retention, unsafe span requests, input too large, private pack mismatch, missing policy refs, and attempted remote/non-local handling.
  - Output: Diagnostic API, caller remediation summaries, operator-safe summaries, correction refs, and Overwatch audit refs.
  - Validation: Tests prove diagnostics help callers correct requests without exposing private content or turning denied private enrichment into a route/model decision.

## Phase 5: Enrichment Execution, Normalization, And Result Lifecycle

### Work Items

- **5.1 Implement bounded enrichment execution.**
  - Design: Invoke ADES through approved local package, CLI, or local HTTP mode with bounded input, timeout budget, max output size, selected pack refs, privacy context, and cancellation behavior.
  - Output: Execution adapter, mode-specific invocation wrappers, timeout enforcement, cancellation path, output capture, and `ades_enrichment_adapter.enrichment_requested` events.
  - Validation: Tests cover successful package/CLI/local HTTP execution, timeout, cancellation, max-output breach, invalid connector mode, unhealthy connector, missing pack, and no execution after privacy denial.

- **5.2 Implement output normalization.**
  - Design: Normalize ADES entities, topics, warnings, timing metadata, confidence fields, pack refs, labels, spans or redacted spans, output hash, and explanation refs into the stable Overrid envelope.
  - Output: Normalizer, schema validators, ordered output fields, timing in milliseconds, output hashes, confidence classes, and `ades_enrichment_adapter.enrichment_completed` events.
  - Validation: Tests prove normalized output is deterministic, timing fields are milliseconds, invalid ADES output is discarded, spans follow redaction policy, and warnings remain advisory signals.

- **5.3 Implement result lifecycle transitions.**
  - Design: Move results through `normalized`, `hint_emitted`, `retained_metadata_only`, `expired`, and `redacted` states with owner permissions, retention policy, and audit refs.
  - Output: Result lifecycle engine, status reads, expiration jobs, redaction jobs, metadata-only projections, and lifecycle events.
  - Validation: Tests prove lifecycle transitions are replayable, expired/redacted results cannot leak raw input, and retained metadata still supports route-decision replay.

- **5.4 Implement result reads and role-specific views.**
  - Design: Add `GET /ades/results/{result_id}` for redacted enrichment output where retention is allowed, with caller, owner, downstream service, operator, and public-report views.
  - Output: Result read API, role-scoped projection builder, safe error behavior, pagination for bounded batch results, and redaction tests.
  - Validation: Contract tests prove unauthorized callers cannot read results, downstream services receive only needed hints, operators receive diagnostic categories, and public/reporting views never expose private text or private entity values.

- **5.5 Implement batch enrichment guardrails.**
  - Design: Add bounded `POST /ades/enrich/batch` support for small local items with per-item privacy checks, pack approval, size caps, timeout budget, partial degradation, and deterministic item-level results.
  - Output: Batch API, per-item lifecycle records, partial success/degradation outputs, aggregate usage refs, and replay bundle.
  - Validation: Tests cover mixed allowed/denied items, item size breach, batch size breach, missing pack, connector timeout, partial degradation, and no cross-item raw-content leakage.

## Phase 6: Routing Hint Bundles And Downstream Advisory Handoff

### Work Items

- **6.1 Implement routing hint bundle creation.**
  - Design: Build advisory hint bundles with model-size hints, tool-need hints, RAG-scope hints, domain hints, warning hints, confidence/explanation refs, pack refs, policy refs, and degradation refs.
  - Output: Hint builder, hint schema, confidence classes, explanation refs, role-scoped views, and `ades_enrichment_adapter.routing_hints_emitted` events.
  - Validation: Tests prove hints never become final model selection, policy allow/deny, safety decision, fraud decision, resource allocation, or route authority.

- **6.2 Integrate AI Gateway Router.**
  - Design: Provide ADES hint bundles to AI Gateway Router as optional advisory facts for model, tool, RAG, and resource selection while leaving final route decisions with the router and policy gates.
  - Output: Router adapter, hint ingestion contract, missing-hint behavior, degraded-hint behavior, route replay refs, and stale-hint rejection.
  - Validation: Tests prove missing ADES hints do not break routing, stale/invalid hints are ignored or downgraded, and router decisions still cite Overguard/policy and router-owned facts.

- **6.3 Integrate Lightweight Classifier.**
  - Design: Feed entities, topics, warnings, domain hints, confidence classes, and pack refs to Lightweight Classifier as supporting facts without replacing classifier-owned data-class or intent decisions.
  - Output: Classifier handoff contract, input fact bundle, confidence lowering on degradation, missing-pack behavior, and replay fixtures.
  - Validation: Tests prove Lightweight Classifier can use hints but must make its own classifier decision, cannot treat ADES warnings as final safety outcomes, and handles missing ADES gracefully.

- **6.4 Integrate Personal AI Assistant and native apps.**
  - Design: Provide local, user-authorized enrichment for assistant prompts and native app text, with permission checks, privacy guards, label-level hints, optional local span highlights, user-visible degradation, and usage/audit refs.
  - Output: Assistant/native handoff contract, user permission adapter, local highlight path, degraded assistant behavior, and native-app fixtures.
  - Validation: Tests prove user-authorized private content stays local, missing ADES degrades visibly but non-fatally, span highlights are local/private only, and native apps cannot bypass Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, storage, or AI routing.

- **6.5 Integrate Encrypted Docdex RAG Adapter.**
  - Design: Use ADES hints to narrow authorized context scopes where policy allows, while Encrypted Docdex RAG Adapter remains the owner of encrypted context retrieval, context grants, chunking, and private corpus boundaries.
  - Output: RAG hint contract, context-scope hint refs, large-document handoff, policy refs, replay refs, and missing-hint behavior.
  - Validation: Tests prove ADES cannot read unauthorized context, cannot widen RAG scope, cannot persist raw repository/workspace content, and cannot replace encrypted Docdex RAG retrieval decisions.

## Phase 7: Degradation, Fallbacks, Stable Errors, And Recovery

### Work Items

- **7.1 Implement degradation records.**
  - Design: Record missing ADES, failed health check, missing pack, unapproved pack, deprecated pack, timeout, invalid output, policy denial, input too large, local service unhealthy, and fallback behavior with caller-visible reason codes.
  - Output: Degradation record schema, degradation API, event payload, reason-code catalog, caller-facing messages, and `ades_enrichment_adapter.degraded` events.
  - Validation: Tests prove every failure mode has a stable reason code, no raw private input appears in degradation records, and degraded calls are not counted as successful enrichment work.

- **7.2 Implement safe fallback behavior.**
  - Design: Allow router/classifier/assistant flows to continue without ADES hints; use `general-en` only when policy allows a generic baseline, never substitute sensitive or locale-specific packs, and lower confidence when domain packs are unavailable.
  - Output: Fallback matrix, generic fallback guard, confidence lowering rules, downstream degraded-hint contract, and user/operator messages.
  - Validation: Tests prove missing finance/medical/locale packs emit `pack_missing` or degradation rather than substituting other sensitive packs or widening authority.

- **7.3 Implement timeout and retry policy.**
  - Design: Enforce bounded retries inside timeout budgets for local health/execution paths, switch only between approved local modes, and emit degradation when the budget is exhausted.
  - Output: Retry policy, timeout budget config, approved-mode switcher, cancellation hooks, and timeout fixtures.
  - Validation: Tests prove no unbounded retries, no remote fallback, no execution after cancellation, no partial unsafe hint emission, and deterministic timeout reason codes.

- **7.4 Implement invalid-output handling.**
  - Design: Validate ADES output shape, pack refs, timing fields, entity/topic/warning arrays, span policy, output size, and confidence fields before passing hints downstream.
  - Output: Output validator, discard path, invalid-output degradation, diagnostic summaries, and replay-safe failure refs.
  - Validation: Tests prove malformed output is discarded, no unsafe hint escapes, downstream services see degradation instead of partial untrusted data, and route replay can cite invalid-output refs without content leakage.

- **7.5 Implement degradation reads and recovery diagnostics.**
  - Design: Add `GET /ades/degradations/{degradation_id}` and operator diagnostics for service startup, `/healthz`, pack inventory, timeout tuning, missing permissions, and fallback behavior.
  - Output: Degradation read API, operator view, caller-safe view, remediation checklist, health drill fixtures, and Overwatch refs.
  - Validation: Tests prove diagnostics are useful, role-scoped, and safe for private requests, with no raw text, raw files, raw spans, private entity values, or sensitive pack payloads.

## Phase 8: Metering, Audit, Replay, And Operational Diagnostics

### Work Items

- **8.1 Implement usage refs without pricing logic.**
  - Design: Emit usage refs for material enrichment operations with connector mode, pack count, input size class, output size class, duration class, user/app/workload refs where allowed, and Overmeter refs.
  - Output: Usage-ref handoff, usage events, accounting-friendly redacted summaries, and owner-service boundary notes.
  - Validation: Tests prove the adapter does not create ORU balances, grants, receipts, invoices, price records, fee records, payout records, ledger entries, or per-call price schedules.

- **8.2 Implement Overwatch audit events.**
  - Design: Emit health, inventory, request, denial, completion, hint, degradation, redaction, and retention events with connector mode, pack refs, privacy mode, data class, timing class, result refs, degradation refs, policy refs, and usage refs.
  - Output: Event emitters, event schemas, redaction policy, audit refs, and event replay fixtures.
  - Validation: Tests prove events include enough evidence for replay and operations while excluding raw private text, raw files, raw spans, secrets, unredacted private entity values, and unauthorized content refs.

- **8.3 Implement route-decision replay support.**
  - Design: Reconstruct enrichment influence from input/output hashes, pack refs, connector version, policy refs, normalized hint refs, degradation refs, and router/classifier decision refs.
  - Output: Replay bundle builder, replay API or export, deterministic fixtures, redacted explanation view, and mismatch detection.
  - Validation: Tests prove route replay can cite ADES pack refs and normalized hints without exposing private text, and replay remains valid after result payload expiration.

- **8.4 Implement operational health and pack alerts.**
  - Design: Alert when private requests attempt non-local enrichment, high-use packs are missing/deprecated/unhealthy, invalid output rate spikes, timeouts rise, local service `/healthz` fails, or pack approval drifts.
  - Output: Alert rules, diagnostic counters, operator dashboard inputs, admin UI data, and Overwatch alert refs.
  - Validation: Tests prove alerts trigger on expected thresholds, remain role-scoped, avoid sensitive payloads, and produce actionable remediation without becoming policy or routing authority.

- **8.5 Implement reporting and compliance hooks.**
  - Design: Provide aggregate, redacted enrichment health, degradation, pack usage, privacy denial, timeout, and invalid-output summaries to Central AI Service, Stewardship Reporting Service, Compliance Boundary Service, and Phase 13 review surfaces.
  - Output: Reporting adapter, redacted aggregate summaries, compliance-boundary refs, governance hardening refs, and export fixtures.
  - Validation: Tests prove reports exclude raw private content, private entity values, raw spans, unapproved pack details, sensitive user/app context, and per-call pricing or revenue assumptions.

## Phase 9: Phase 12 Proof, Native Integration, And Governance Expansion Gates

### Work Items

- **9.1 Prove the first Personal AI Assistant enrichment path.**
  - Design: Run a Phase 12 assistant request through privacy preflight, local ADES enrichment, label-level hints, router/classifier handoff, user-visible degradation handling, usage refs, and audit refs.
  - Output: End-to-end assistant fixture, successful enrichment trace, missing-ADES trace, missing-pack trace, local span-highlight trace, and replay bundle.
  - Validation: Tests prove assistant behavior improves with hints but still functions without ADES, and ADES output remains advisory.

- **9.2 Prove native app local text enrichment.**
  - Design: Add native app fixtures for user-authorized text tagging, domain hints, warning hints, local-only private handling, redacted result reads, and metadata-only retention.
  - Output: Native app fixture set, app permission checks, user-facing degradation messages, result projections, and audit refs.
  - Validation: Tests prove native apps cannot bypass identity, tenant, policy, storage, AI routing, usage, or audit rails, and cannot persist raw private text through adapter records.

- **9.3 Define regulated-domain expansion gates.**
  - Design: Gate finance, medical, locale-specific, child-safety, regulated-workload, and public-infrastructure pack use behind explicit pack policy, Compliance Boundary facts, Overguard data-class checks, local-only mode, validation fixtures, steward approval, and redaction review.
  - Output: Expansion gate checklist, compliance refs, pack approval refs, validation fixture requirements, steward review refs, and blocked-state behavior.
  - Validation: Review rejects regulated-domain pack activation without policy refs, compliance facts, fixture coverage, local-only enforcement, reviewer scope, redaction rules, and Phase 13 review where required.

- **9.4 Define mobile and third-party app gates.**
  - Design: Gate Mobile SDK, Mobile Backend Gateway, and third-party app usage through device/session refs, compact local enrichment limits, offline replay, push redaction, media/content grants, and normal Overrid rails.
  - Output: Mobile handoff checklist, device/session constraints, offline command behavior, compact result shapes, redacted push rules, and handoff fixtures.
  - Validation: Tests prove mobile clients cannot send private text to non-local services, bypass Overgate/Overpass/Overtenant/Overkey/Overguard/Overwatch/Overmeter, or persist raw private content through sync/push paths.

- **9.5 Define Phase 13 hardening gates.**
  - Design: Prepare threat modeling, security review, incident response, compliance boundary review, governance reporting, performance/timeout drills, and central AI authority boundaries for ADES enrichment.
  - Output: Phase 13 checklist, threat model inputs, security review scope, incident playbooks, compliance report refs, performance drill plan, and central AI boundary notes.
  - Validation: Review proves broad public/native use is blocked until security/privacy/compliance findings are tracked, central AI cannot mutate adapter policy alone, and incident/reporting surfaces are redacted.

## Phase 10: Validation, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Verify title prefix, attached SDS link, phase headings 1 through 10, work-item numbering, Design/Output/Validation fields, source alignment, master-phase mapping, tech-stack guardrails, and alignment review.
  - Output: Focused validation result for this file.
  - Validation: Scripted/text checks pass for title prefix, attached SDS, exactly 10 phase headings, five work items per phase, required work-item fields, local links, final newline, and no tabs.

- **10.2 Validate documentation alignment.**
  - Design: Check SDS backlink, service catalog backlink, master plan row, crosswalk row, Phase 12 wording, tech-stack consistency, and no master Phase 0 through Phase 13 reordering.
  - Output: Alignment evidence in `docs/build_plan/progress.md` and queue progress.
  - Validation: Local Markdown link checks pass and review confirms no SDS correction or master-plan reordering is needed beyond backlink/index-row and Phase 12 wording updates.

- **10.3 Validate authority and stack guardrails.**
  - Design: Scan active changed docs for forbidden drift: raw private text persistence, remote private enrichment, final policy/routing/model/safety/fraud authority, conventional database/queue/object-store/vault/KMS/Kubernetes-first boundaries, blockchain/NFT mechanics, pricing, revenue, customer-count assumptions, and external-provider-as-core-boundary behavior.
  - Output: Guardrail scan evidence and any required correction patches.
  - Validation: Guardrail scan passes with only expected negative-control, native Overrid service-name, or explicit authority-boundary references.

- **10.4 Validate queue state and Docdex retrieval.**
  - Design: Mark `058-build-plan` complete, clear running state if present, advance queue progress to `059-build-plan`, refresh Docdex index for changed indexed docs, and verify retrieval.
  - Output: Updated `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, targeted Docdex index refresh, and Docdex search evidence.
  - Validation: JSON validation passes, queue progress counts are consistent, Docdex search returns the SDS #58 sub-build plan with SDS/service/master/crosswalk context, and DAG export succeeds for retrieval traces when available.

- **10.5 Record final implementation handoff.**
  - Design: Summarize deliverables, aligned docs, validation evidence, known blockers, local delegation outcome, impact results, and implementation handoff notes.
  - Output: Progress entry, repo memory note, optional diary note, and concise final response.
  - Validation: Progress evidence names commands/checks, `docdexd run-tests` blocker is recorded if still present, and final response accurately reports created/updated files and validation status.

## Alignment Review

- SDS #58 already aligns with Phase 12 as the first build phase. No SDS content correction is required beyond adding the sub-build-plan backlink.
- The service catalog entry already aligns with Phase 12 and optional local enrichment boundaries. It only needs the sub-build-plan backlink.
- The master Phase 0 through Phase 13 order remains valid. No phase reordering is required.
- Phase 12 benefits from a small wording correction: replace the generic "ADES-style entity extraction" wording with an explicit ADES Enrichment Adapter reference so the master plan names the SDS #58 owner.
- The plan respects `docs/overrid_tech_stack_choice.md`: Rust-first core, canonical contracts, native Overrid boundaries, optional local adapter behavior, and no conventional cloud/product-boundary drift.
