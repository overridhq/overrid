# SUB BUILD PLAN #62 - Lightweight Classifier

Attached SDS: [docs/sds/ai_rag_model_routing/lightweight_classifier.md](../sds/ai_rag_model_routing/lightweight_classifier.md)

## Purpose

This sub-build plan turns SDS #62 into an implementation sequence for Lightweight Classifier. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Lightweight Classifier is the Phase 12 advisory pre-routing service that cheaply labels AI requests before larger resources are used. It owns the versioned request-classification taxonomy, classifier versions, request/result records, confidence policies, escalation records, evaluation fixtures, calibration reports, drift signals, advisory hint bundles, usage refs, audit refs, and replay evidence. It does not enforce policy, authorize context, choose final models, run user-facing conversations, mutate accounting state, replace Workload Classifier, or bypass AI Gateway Router, Overguard, Encrypted Docdex RAG Adapter, Personal AI Assistant, user permissions, or final safety review.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #62: Lightweight Classifier](../sds/ai_rag_model_routing/lightweight_classifier.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Lightweight Classifier service plan](../service_catalog/ai_rag_model_routing/lightweight_classifier.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service/classifier records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Policy Dry-Run previews, deny-by-default behavior, and review-required signals that classifier hints cannot override. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs and the rule that classifier usage is structural/auditable without hardcoded pricing, revenue, provider payout, or customer-count assumptions. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies AI Gateway Router product route requests, encrypted Docdex RAG context plans, classification-required placeholders, route metadata, usage, and replay prerequisites. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase state, Overstore artifact refs, Overvault protected example refs, namespace refs, private-data handling, and native storage boundaries for classifier versions, fixtures, reports, and retained examples. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first build point for Personal AI Assistant, native app classification, advisory routing hints, local ADES hints, model/resource routing, wallet/usage visibility, and mobile handoffs. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies false-negative threat models, privacy/retention review, classifier rollout security, drift/abuse handling, incident response, compliance, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #62 first build work aligned to master Phase 12 with earlier product-routing prerequisites and Phase 13 hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, customer-count, hardcoded model/provider, raw-private-prompt retention, or final-authority drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 12, and 13 | Attach SDS #62, preserve Phase 12 as first build, record prerequisites, and freeze the advisory-only boundary. |
| 2 | Master Phases 0, 1, 4, 5, 8, and 12 | Define Rust contracts, taxonomy, schemas, state machines, stable errors, signed refs, hashes, and deterministic fixtures. |
| 3 | Master Phases 1, 4, 6, 8, and 12 | Implement request intake, metadata normalization, privacy constraints, taxonomy compatibility, and safe dry-run behavior. |
| 4 | Master Phases 0, 4, 6, and 12 | Implement deterministic Rust heuristic classification and confidence policy before optional enrichment. |
| 5 | Master Phases 4, 6, 8, and 12 | Integrate optional ADES hints and policy-allowed small local model classification without making either mandatory authority. |
| 6 | Master Phases 4, 6, 8, and 12 | Implement hard escalation, advisory handoff bundles, AI Gateway Router, Personal AI, encrypted RAG, and native-app integration. |
| 7 | Master Phases 8, 12, and 13 | Build evaluation fixtures, calibration reports, correction records, retention-safe examples, and drift detection. |
| 8 | Master Phases 1, 5, 8, 12, and 13 | Implement classifier version rollout, usage refs, audit, replay, observability, SDK/CLI/admin diagnostics, and support views. |
| 9 | Master Phase 13, with prerequisites from Phases 0 through 12 | Harden false negatives, escalation bypass, privacy, retention, rollout abuse, model drift, incident response, and scale behavior. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Lightweight Classifier core is a Rust service/module using shared contract crates, Tokio for bounded classification/evaluation/rollout workers, and Axum/Tower/Hyper-style HTTP only where a service boundary is needed.
- Taxonomy, classifier version, request/result, confidence policy, escalation record, evaluation fixture, calibration report, drift signal, usage ref, audit record, replay bundle, events, and diagnostics use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant scope, caller app, trace id, idempotency key, privacy mode, policy refs, classifier version refs, taxonomy version refs, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for request hashes, redacted text refs, fixture artifacts, calibration snapshots, classifier outputs, replay bundles, and audit exports.
- Storage, queueing, private records, retained examples, artifact refs, audit, usage, policy, identity, keys, and accounting handoffs must use native Overrid service boundaries such as Overbase, Overqueue, Overstore, Overvault, Overpass, Overtenant, Overkey, Overwatch, Overmeter, Overguard, ORU Account Service, and Seal Ledger.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw private prompts in logs/events/metrics/default durable records, hardcoded model/provider names, final policy authority, final model selection, or user-facing conversation execution the classifier boundary.

## Phase 1: SDS Attachment, Phase 12 Scope, And Advisory Boundary

### Work Items

- **1.1 Attach the build plan to SDS #62.**
  - Design: Link this document from the Lightweight Classifier SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/ai_rag_model_routing/lightweight_classifier.md`, `docs/service_catalog/ai_rag_model_routing/lightweight_classifier.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #62 returns both the Lightweight Classifier SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep first implementation in Phase 12 because the classifier first becomes useful when Personal AI Assistant and native apps need cheap request labels before routing, RAG, tool, and native-app handoff decisions.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 6, and 8 supply contracts, identity, policy, metering, product AI routing, and protected storage prerequisites; Phase 12 builds the classifier; Phase 13 hardens it.
  - Validation: Review proves the plan does not move Lightweight Classifier into Phase 4 Workload Classifier, Phase 6 final route authority, or Phase 13-only hardening, and does not reorder master Phase 0 through Phase 13.

- **1.3 Freeze the advisory ownership boundary.**
  - Design: Record that Lightweight Classifier owns advisory label taxonomy, versions, classification records, confidence policies, escalation records, evaluation fixtures, calibration reports, drift signals, hint bundles, usage refs, audit refs, and replay evidence.
  - Output: Ownership checklist for architecture, API, and implementation reviews.
  - Validation: Review confirms the classifier does not enforce Overguard policy, authorize context, choose final model routes, run conversations, mutate ledger/accounting/grant state, or bypass owning-service permission checks.

- **1.4 Carry forward resolved open-question decisions.**
  - Design: Preserve the compact Phase 12 taxonomy, hard escalation classes, hybrid deterministic Rust plus optional ADES/small-local-model approach, retention-safe evaluation artifacts, and audience-safe metrics split.
  - Output: Resolved-decision checklist covering labels, hard escalation, baseline classifier architecture, calibration retention, user-visible summaries, and operator-only aggregate quality views.
  - Validation: Review rejects model-only decisions, remote-provider requirements, raw private prompt retention by default, hidden sensitive false negatives, and user/operator views that leak private payloads or exact abuse thresholds.

- **1.5 Define downstream consumer boundaries.**
  - Design: Record how AI Gateway Router, Personal AI Assistant, Encrypted Docdex RAG Adapter, ADES Enrichment Adapter, native apps, mobile, SDK, CLI, admin UI, Overguard, Workload Classifier, Overwatch, and Overmeter consume hints.
  - Output: Consumer-boundary matrix naming allowed inputs, advisory outputs, denied direct authority, usage refs, audit refs, and replay requirements.
  - Validation: Review confirms downstream services treat classifier output as hints and still run their own policy, context, permission, route, model, tool, and native-app authority checks.

## Phase 2: Contracts, Taxonomy, Schemas, And Fixtures

### Work Items

- **2.1 Create the Rust contract module.**
  - Design: Add contract types for classifier taxonomy, classifier version, classification request, classification result, confidence policy, escalation record, evaluation fixture, calibration report, drift signal, classification usage ref, event payload, replay bundle, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, label enums, confidence/uncertainty enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from AI Gateway Router final routing, model inference, policy enforcement, and storage internals.

- **2.2 Define the Phase 12 label taxonomy.**
  - Design: Model compact first-build labels for intent/purpose class, privacy/data-class hint, RAG/context-source need, native-app/tool-delegation need, side-effect class, model-size/capability hint, output-mode hint, latency/budget class, confidence class, uncertainty class, escalation requirement, and reason codes.
  - Output: Taxonomy schema, compatibility version, allowed values, deprecated-label rules, visible docs, valid examples, invalid examples, and canonical fixtures for common assistant/native-app requests.
  - Validation: Schema tests reject unversioned labels, unknown active labels, missing confidence/uncertainty, missing escalation requirement, and labels that imply final permission or final model authority.

- **2.3 Define classifier version and rollout schemas.**
  - Design: Model implementation type, deterministic heuristic refs, optional small-model capability refs, prompt/template refs where applicable, optional ADES dependency refs, rollout state, compatibility window, evaluation gate refs, canary settings, pause/rollback flags, and blocked reasons.
  - Output: JSON Schema files, Rust validators, active/canary/deprecated/blocked examples, version-lifecycle fixtures, and Overregistry capability refs.
  - Validation: Tests reject hardcoded model names, missing compatibility window, missing eval gate, unavailable version promotion, and version records that require a remote provider.

- **2.4 Define request/result, confidence, and escalation schemas.**
  - Design: Model actor/tenant refs, caller app, request hash, redacted text ref, metadata refs, ADES hint refs, privacy mode, max latency class, trace id, idempotency key, labels, confidence, uncertainty, reason codes, fallback requirements, and escalation records.
  - Output: Request/result schemas, confidence-policy schema, escalation schema, stable error catalog, idempotency examples, redacted examples, and denied/escalated fixtures.
  - Validation: Tests reject requests without tenant/caller/trace/privacy refs, results without taxonomy/classifier version, confidence policies without fallback behavior, and escalation records without downstream policy refs.

- **2.5 Create deterministic fixtures.**
  - Design: Build fixtures for simple chat, search/RAG, writing/editing, code help, workspace assistance, messaging triage, wallet read, native-app delegation, tool proposal, central-AI handoff, private context request, accounting mutation, destructive tool action, ambiguous request, unavailable classifier, and invalid output.
  - Output: Fixture directory, canonical inputs, redacted projections, expected labels, expected confidence bands, expected escalation decisions, BLAKE3 hashes, usage refs, audit refs, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, labels, confidence bands, escalation outcomes, reason codes, usage refs, and replay output across repeated runs.

## Phase 3: Request Intake, Metadata Normalization, And Privacy Validation

### Work Items

- **3.1 Implement classification request intake.**
  - Design: Add `POST /lightweight-classifier/classify` and `POST /lightweight-classifier/classify/batch` with signed envelope checks, actor/service identity, tenant scope, caller app, trace id, idempotency key, privacy mode, classifier/taxonomy compatibility, and bounded input size.
  - Output: Request handlers, batch bounds, request records, idempotency behavior, stable errors, and `lightweight_classifier.classification_requested` events.
  - Validation: API tests cover valid single/batch requests, duplicate idempotency keys, missing tenant, missing caller app, missing privacy mode, unsupported taxonomy version, unavailable classifier version, oversized input, and audience-safe errors.

- **3.2 Normalize caller metadata and ADES refs.**
  - Design: Normalize caller-provided task class, app/native-service refs, tool proposal refs, side-effect markers, route/session refs, context/RAG hints, ADES hint refs, privacy mode, budget/latency class, and prior classifier facts.
  - Output: Metadata normalizer, canonical request snapshot, degraded-hint markers, stale-hint rejection, missing-hint behavior, and replay refs.
  - Validation: Tests prove stale/invalid ADES refs are ignored or downgraded, missing ADES hints do not fail basic classification, and caller metadata cannot force lower sensitivity or bypass escalation.

- **3.3 Validate privacy and retention constraints.**
  - Design: Enforce default ephemeral raw prompt handling, redacted text refs, hashes, owner-approved retained example refs, purpose/scope/expiry, and retention-denied behavior before classification persists any durable record.
  - Output: Privacy validator, retention-class config, retained-example gate, redaction/hash records, and `private_input_retention_denied` behavior.
  - Validation: Tests prove raw private prompts are absent from logs/events/metrics/default records, retained examples require explicit bounded refs, and retention denial does not block ephemeral classification unless policy requires denial.

- **3.4 Implement dry-run preview.**
  - Design: Add `POST /lightweight-classifier/dry-run` to preview labels, confidence, uncertainty, escalation, and fallback requirements without affecting route state or storing raw input beyond allowed ephemeral processing.
  - Output: Dry-run handler, side-effect-free result records, redacted projections, stable errors, and replay-safe preview refs.
  - Validation: Tests prove dry-run does not promote classifier versions, trigger route changes, retain raw input by default, or authorize context/tool/model decisions.

- **3.5 Implement taxonomy and version reads.**
  - Design: Add `GET /lightweight-classifier/taxonomy` and `GET /lightweight-classifier/versions` with role-scoped visibility for active/canary/deprecated/blocked classifier versions and compatibility windows.
  - Output: Read handlers, public-safe taxonomy docs, operator diagnostics, active/canary views, and redacted blocked-reason projections.
  - Validation: Tests prove ordinary clients see safe label semantics while operator views remain scoped and do not reveal prompt secrets, abuse thresholds, model secrets, or private fixtures.

## Phase 4: Deterministic Baseline And Confidence Policy

### Work Items

- **4.1 Implement deterministic Rust heuristics.**
  - Design: Build deterministic rules for input size, privacy mode, tool/side-effect markers, permission widening, context/RAG need, accounting/ledger/grant/payout terms, destructive actions, external egress, regulated/safety-sensitive classes, native-app targets, and unknown/ambiguous requests.
  - Output: Heuristic engine, rule version refs, reason-code catalog, strictest-class behavior, rule-match evidence, and replay snapshots.
  - Validation: Tests prove deterministic gates catch hard escalation classes before optional model classification and never downgrade strict caller or policy facts.

- **4.2 Implement label normalization.**
  - Design: Normalize heuristic outputs into the Phase 12 taxonomy with confidence, uncertainty, reason codes, data-class hints, native-app target hints, model-size/capability hints, output-mode hints, latency/budget class, and fallback requirements.
  - Output: Normalization module, output validators, canonical result builder, invalid-output handling, and `lightweight_classifier.classification_completed` events.
  - Validation: Tests reject malformed labels, missing reason codes, unbounded confidence, taxonomy/version mismatch, and output that implies final route/model/context authority.

- **4.3 Implement confidence policy.**
  - Design: Apply confidence thresholds by label, privacy mode, caller app, data class, tool/side-effect class, RAG need, native-app target, and required fallback behavior.
  - Output: Confidence-policy evaluator, threshold config, fallback records, escalation requirement writer, and replay refs.
  - Validation: Tests prove low confidence, ambiguous intent, private/regulatory data, side-effecting tools, accounting mutations, and conflicting route facts produce escalation-required outcomes.

- **4.4 Implement hard escalation records.**
  - Design: Record hard escalations for low-confidence/ambiguous requests, private context widening, secret/regulated data, destructive or mutating tools, identity/credential/permission changes, ORU/Seal Ledger/Overbill/Overgrant/payout/dispute actions, external egress, safety guidance, abuse/fraud/moderation/stewardship paths, and route/context conflicts.
  - Output: Escalation writer, downstream service refs, user/app confirmation flags, policy refs, route refs, and `lightweight_classifier.classification_escalated` events.
  - Validation: Tests prove classifier may still emit hints for escalated requests but downstream flow must require stronger analysis, Overguard or owning-service policy checks, and confirmation where applicable.

- **4.5 Implement unavailable and invalid classifier behavior.**
  - Design: Fail closed when classifier versions are unavailable, deterministic rules cannot parse input, optional model output is invalid, taxonomy changes are unsupported, or usage/audit prerequisites are missing.
  - Output: Stable errors, fallback requirement records, denied/escalated terminal states, audit refs, and `lightweight_classifier.classification_denied` events.
  - Validation: Tests prove unavailable or invalid classifiers return escalation-required or denied outcomes rather than silent cheap-route approval.

## Phase 5: Optional ADES Hints And Small Local Model Classification

### Work Items

- **5.1 Integrate optional ADES hints.**
  - Design: Accept ADES entity/topic/warning/domain-pack hints only as advisory input, with local-only/private guards, hint version refs, stale/degraded markers, and no requirement that ADES be available for classification.
  - Output: ADES hint adapter, hint-normalization contract, stale-hint rejection, degraded-mode behavior, usage refs, and replay refs.
  - Validation: Tests prove missing ADES hints do not break classification, stale or invalid hints cannot lower sensitivity, and ADES cannot make final policy/routing/model decisions.

- **5.2 Implement small local model connector.**
  - Design: Add optional connector support for policy-allowed small local model classification of non-sensitive ambiguous language, using Overregistry capability refs instead of hardcoded model/provider names.
  - Output: Local model connector interface, capability-ref lookup, bounded prompt/template refs, timeout behavior, invalid-output discard, and degraded fallback.
  - Validation: Tests prove remote providers are not required, model/provider names are not hardcoded, unavailable model output escalates or falls back, and sensitive hard-gate classes do not depend on model output.

- **5.3 Merge deterministic, ADES, and model facts.**
  - Design: Merge deterministic baseline facts, caller metadata, optional ADES hints, and optional small-model output into one normalized advisory result while preserving provenance and strictest escalation.
  - Output: Merge engine, source provenance refs, conflict resolution rules, downgraded-confidence behavior, and replayable source snapshots.
  - Validation: Tests prove deterministic hard escalation wins conflicts, optional hints cannot override policy/data-class facts, and merged outputs remain explainable from stored refs.

- **5.4 Bound latency and resource use.**
  - Design: Apply max latency class, batch size, local-model timeout, concurrency limits, retry policy, degraded-mode return, and usage classes so cheap classification remains cheap.
  - Output: Runtime limits, worker config, timeout records, bounded batch executor, load-shedding behavior, and Overmeter usage refs.
  - Validation: Tests prove large batches are capped, timeouts produce degraded/escalated outputs, failed local model calls do not block safe deterministic escalation, and resource use emits structural usage refs.

- **5.5 Validate optional path replay.**
  - Design: Preserve enough version, hint, prompt/template, capability, hash, confidence, and reason-code refs to replay why optional enrichment changed or did not change a result.
  - Output: Optional-path replay bundle, redacted source refs, model capability refs, ADES refs, decision trace, and audit export hooks.
  - Validation: Replay tests reconstruct deterministic-only, ADES-degraded, model-unavailable, model-invalid, and merged-success outcomes without raw private prompts.

## Phase 6: Escalation, Handoff Bundles, And Native AI Integrations

### Work Items

- **6.1 Implement AI Gateway Router handoff.**
  - Design: Provide AI Gateway Router with advisory labels, confidence, uncertainty, escalation requirement, reason codes, taxonomy/classifier version refs, privacy/data-class hints, RAG/tool/model-size/native-app hints, usage refs, and replay refs.
  - Output: Router-facing hint bundle contract, route metadata refs, fallback flags, classification-required behavior, and handoff events.
  - Validation: Tests prove AI Gateway Router owns final route decisions, cannot treat classifier confidence as permission, and must escalate incomplete/low-confidence/context-conflicting facts.

- **6.2 Implement Personal AI Assistant handoff.**
  - Design: Provide assistant sessions with cheap classification before context planning, tool planning, native-app delegation, output-mode selection, and user confirmation prompts.
  - Output: Assistant-facing hint bundle, confirmation reason codes, context widening flags, tool/native-app side-effect flags, and user-visible safe summaries.
  - Validation: Tests prove the assistant cannot reuse classifier hints to widen private context, call mutating tools, skip user confirmation, or bypass encrypted RAG grants.

- **6.3 Implement Encrypted Docdex RAG Adapter handoff.**
  - Design: Provide RAG/context-source need, privacy/data-class hints, result-scope hints, escalation flags, and route refs to the RAG adapter while keeping context authorization inside Encrypted Docdex RAG Adapter.
  - Output: RAG hint contract, context-source flags, grant-required markers, leakage-awareness refs, and retrieval dry-run triggers.
  - Validation: Tests prove classifier output cannot authorize encrypted Docdex retrieval, suppress leakage profiles, bypass context grants, or expose raw private content.

- **6.4 Implement native app and mobile handoffs.**
  - Design: Provide workspace, messaging, search, directory, wallet/usage, mobile, and other native apps with advisory task labels, tool/native-app target hints, side-effect classes, confidence bands, and confirmation/escalation reasons.
  - Output: Native-app hint contract, mobile-safe projections, SDK bindings, app-specific allowed values, and redacted user-visible summaries.
  - Validation: Contract tests prove native apps still own domain permissions and side effects, and classifier hints cannot mutate app data or hide policy requirements.

- **6.5 Implement Overguard and Workload Classifier comparison.**
  - Design: Send classifier hints to Overguard and Workload Classifier only as advisory facts where useful, while preserving Workload Classifier authority over execution/data classes and Overguard authority over policy decisions.
  - Output: Policy input projection, comparison records, conflict reason codes, review-required behavior, and audit refs.
  - Validation: Tests prove conflicts with Workload Classifier or Overguard force escalation/review and classifier hints cannot weaken workload/data sensitivity.

## Phase 7: Evaluation, Calibration, Drift, And Retention-Safe Learning

### Work Items

- **7.1 Build evaluation fixture sets.**
  - Design: Create fixture sets for assistant tasks, RAG decisions, native app delegation, tool calls, accounting/ledger/grant/payout/dispute requests, safety-sensitive questions, abuse/moderation/stewardship paths, and sensitive false-negative classes.
  - Output: Synthetic fixtures, redacted golden examples, expected labels, expected escalation, reason-code expectations, privacy classes, and fixture-source refs.
  - Validation: Tests prove fixture coverage catches false-negative sensitive classes, hard escalation misses, taxonomy drift, and side-effect/tool misclassification before rollout.

- **7.2 Implement evaluation run APIs.**
  - Design: Add `POST /lightweight-classifier/evaluations` and `GET /lightweight-classifier/evaluations/{eval_id}` for bounded fixture runs, calibration reports, regression gates, reviewer refs, and rollout decisions.
  - Output: Evaluation runner, result records, calibration report writer, confusion-matrix summaries, false-negative class reports, and `lightweight_classifier.evaluation_completed` events.
  - Validation: API tests prove failed evaluation gates block promotion, evaluation results remain replayable, and reports avoid raw private prompts.

- **7.3 Implement label-only corrections and retained example governance.**
  - Design: Support label-only correction records, owner-approved encrypted evaluation artifacts, explicit purpose/scope/expiry/revocation, and deletion/redaction on expiry, revocation, dispute closure, or policy change.
  - Output: Correction schema, retained-example refs, Overvault/Overstore handoff, expiry worker, redaction tombstones, and audit evidence.
  - Validation: Tests prove raw prompts, raw files, secrets, and private app payloads stay ephemeral by default, and retained examples cannot outlive purpose/scope/expiry/revocation rules.

- **7.4 Implement calibration and drift detection.**
  - Design: Track confidence distribution, label distribution, escalation rate, false-negative rates, fixture regressions, ADES/model degradation, route outcome mismatches, and canary drift indicators.
  - Output: Calibration report, drift detector, alert inputs, rollout pause suggestions, and `lightweight_classifier.drift_detected` events.
  - Validation: Tests prove drift or false-negative spikes pause rollout or require review instead of silently promoting a weak classifier.

- **7.5 Implement privacy-safe quality views.**
  - Design: Provide user-visible status summaries and operator-only aggregate quality views without raw private prompts, decrypted context, other-tenant facts, exact abuse thresholds, or hidden policy heuristics.
  - Output: User metrics projection, operator metrics projection, redaction rules, access checks, and support-safe views.
  - Validation: Tests prove user views show classification status, confidence band, escalation reason, route/usage/privacy refs, while operator views remain aggregate and scoped.

## Phase 8: Version Rollout, Usage, Audit, Replay, And Diagnostics

### Work Items

- **8.1 Implement classifier version rollout.**
  - Design: Add `POST /lightweight-classifier/versions/{version_id}/rollout` to start, pause, promote, rollback, deprecate, or block versions with signed action, evaluation refs, compatibility windows, and Overwatch evidence.
  - Output: Rollout handler, canary controls, pause/rollback records, blocked-version records, and `lightweight_classifier.version_rollout_changed` events.
  - Validation: Tests prove versions cannot promote without passing gates, rollback restores the prior active version, and all results cite the classifier/taxonomy version used.

- **8.2 Emit classification usage refs.**
  - Design: Emit usage refs for single calls, batch calls, deterministic-only calls, ADES hint normalization, small local model invocations, evaluation runs, canary diagnostics, denied calls, failed calls, and escalated calls.
  - Output: Usage writer, operation classes, actor/tenant/caller/version/request/route refs, reconciliation records, and Overmeter handoff.
  - Validation: Tests prove successful, failed, denied, escalated, deterministic-only, optional-model, and evaluation operations produce structural usage refs without hardcoded prices or provider payout logic.

- **8.3 Implement audit and replay APIs.**
  - Design: Add replay reads that reconstruct taxonomy, classifier version, input hash, redacted text ref, metadata refs, ADES refs, deterministic rule matches, optional model capability refs, confidence policy, escalation, usage, and downstream handoff facts.
  - Output: Replay API, role-scoped replay bundles, pagination for linked records, audit export hooks, and deterministic replay fixtures.
  - Validation: Tests prove replay is deterministic, privacy bounded, can explain allowed/escalated/denied decisions, and does not fetch raw private payloads after expiry.

- **8.4 Implement observability.**
  - Design: Expose request volume, latency, confidence distribution, escalation rate, label distribution, error rate, optional model health, ADES hint health, drift signals, rollout state, and evaluation results.
  - Output: Metrics, tracing spans, alert rules, dashboard projections, operator views, and Overwatch event links.
  - Validation: Tests prove alerts fire for false-negative sensitive classes, high escalation loops, stale classifier versions, drift/regression signals, privacy leakage attempts, and model/ADES degradation.

- **8.5 Implement SDK, CLI, and admin diagnostics.**
  - Design: Provide typed SDK/CLI/admin commands for classify, dry-run, batch classify, taxonomy reads, version reads, evaluation runs, rollout actions, replay reads, usage refs, and degraded classifier diagnostics.
  - Output: Command contracts, stable JSON output, generated bindings, signed envelope examples, idempotency behavior, and operator docs.
  - Validation: Contract tests prove commands require identity/tenant/caller scope refs, produce stable JSON, preserve idempotency, and match service API authority boundaries.

## Phase 9: Phase 13 Hardening, Security, Privacy, And Scale

### Work Items

- **9.1 Harden false-negative and escalation-bypass risks.**
  - Design: Threat-model missed sensitive classes, accounting/tool side effects, hidden context widening, safety-sensitive prompts, policy conflicts, native-app mutation bypass, and route fallback misuse.
  - Output: Threat model, mitigation checklist, blocking tests, review-required gates, and incident playbook refs.
  - Validation: Security tests prove hard escalation classes cannot be route-approved by classifier confidence alone and false-negative fixture failures block rollout.

- **9.2 Harden privacy, retention, and support access.**
  - Design: Apply audience-class retention for user summaries, operator aggregates, private replay details, retained examples, support/dispute/incident/legal/compliance pins, deletion/redaction, and long-term hash/id/version archives.
  - Output: Retention policy config, legal/compliance hold refs, redaction tombstones, archive compaction rules, support-safe projections, and audit export fixtures.
  - Validation: Tests prove raw private payloads expire or remain ephemeral by default, pins require allowed reason classes, deletion does not break audit chains, and archives reduce to safe refs.

- **9.3 Harden rollout and model/ADES supply-chain safety.**
  - Design: Review classifier version provenance, heuristic rule changes, prompt/template refs, optional model capability refs, ADES domain-pack refs, evaluation gates, canary promotion, rollback, and stale dependency handling.
  - Output: Rollout security checklist, provenance refs, dependency health checks, stale/blocked version handling, and supply-chain review records.
  - Validation: Tests prove stale, unreviewed, blocked, or evaluation-failed classifier/model/ADES refs cannot silently promote or weaken escalation.

- **9.4 Harden incident and abuse response.**
  - Design: Add incident playbooks for false-negative sensitive classes, raw prompt leakage, wrong native-app handoff, side-effect/tool misclassification, classifier outage, drift spike, model invalid-output spike, ADES bad-hint spike, and replay/audit gaps.
  - Output: Incident playbook refs, freeze/pause behavior, affected-route review refs, owner/operator notifications, correction/retraction workflows, and post-incident report refs.
  - Validation: Drills prove each incident path creates audit evidence, pauses risky versions or routes, notifies owning services/users where policy requires, and preserves correction/replay paths.

- **9.5 Harden reliability and scale behavior.**
  - Design: Add bounded concurrency, batch caps, queue depth controls, retry/dead-letter behavior, rollout canary percentages, backpressure, load shedding, degraded deterministic-only mode, and replay pagination.
  - Output: Reliability config, worker limits, retry/dead-letter records, backpressure metrics, load-test fixtures, and scale dashboards.
  - Validation: Load tests prove classifier preserves idempotency, emits usage/audit evidence, degrades by escalating or denying risky requests rather than bypassing policy, and handles native-app traffic spikes safely.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, 10 phase headings numbered 1 through 10, five work items per phase, Design/Output/Validation blocks, local links, final newline, and tab-free Markdown.
  - Output: Targeted validation script output and recorded evidence in `docs/build_plan/progress.md`.
  - Validation: Script passes for this file and the linked SDS/service/build-plan docs.

- **10.2 Validate documentation alignment.**
  - Design: Confirm SDS #62, the service catalog plan, master build plan, service catalog alignment crosswalk, Phase 12, Phase 13, and tech-stack decision all preserve the same first build phase, prerequisites, hardening gates, ownership boundaries, and non-goals.
  - Output: Alignment review notes and updated backlinks/index rows.
  - Validation: Review confirms there is no master Phase 0 through Phase 13 reorder, no Phase 4/6/13 first-build drift, and no missing sub-build-plan link.

- **10.3 Validate stack and authority guardrails.**
  - Design: Scan the changed docs for conventional database, queue, object store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, customer-count, raw private prompts, hardcoded model/provider, final policy authority, final model selection, context authorization, and user-facing conversation drift.
  - Output: Guardrail scan evidence with only expected negative-control references and native Overrid service-name references.
  - Validation: Review confirms Lightweight Classifier remains Rust-first, native-Overrid-boundary-based, advisory-only, privacy-bounded, versioned, usage-metered, evaluated, and replayable.

- **10.4 Refresh Docdex, queue, and progress state.**
  - Design: Update the Codex55 queue state/progress for `062-build-plan`, append build-plan progress evidence, index changed docs, and search for the new plan/backlinks.
  - Output: `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, `docs/build_plan/progress.md`, Docdex index refresh, Docdex search results, and DAG/session evidence where available.
  - Validation: JSON validation passes, queue counts advance from SDS #62 to SDS #63, Docdex search returns the new #62 plan with SDS/service backlinks, and no unrelated queue tasks are changed.

- **10.5 Prepare implementation handoff.**
  - Design: Summarize the ordered implementation entry points, required prerequisites, first usable Phase 12 proof, Phase 13 hardening gates, test expectations, and advisory/privacy boundaries for builders.
  - Output: Implementation handoff checklist for contracts, taxonomy, intake, deterministic baseline, optional ADES/model paths, escalation, handoffs, evaluation, rollout, usage/audit/replay, hardening, and validation.
  - Validation: Handoff review confirms a builder can start with contracts and taxonomy, prove deterministic cheap classification in Phase 12, integrate advisory handoffs safely, harden in Phase 13, and avoid raw-private-prompt, final-authority, and hardcoded-model drift.

## Alignment Review

- SDS #62 remains attached to master Phase 12 as the first build point for advisory AI request classification, compact taxonomy, deterministic Rust heuristics, optional ADES hints, optional policy-allowed small local model classification, confidence policies, hard escalation, evaluation, rollout, usage, audit, and replay.
- Phase 6 remains a prerequisite/product-routing consumer through AI Gateway Router classification fact bundles, context plans, and route metadata; this plan does not move Lightweight Classifier's first implementation into Phase 6.
- Phase 4 Workload Classifier remains the execution/data sensitivity authority for workload safety; Lightweight Classifier does not replace it.
- Phase 13 remains governance, compliance, security, incident, privacy, retention, false-negative, drift, rollout, and scale hardening rather than the first delivery point.
- Phases 0, 1, 4, 5, 6, and 8 remain prerequisites; this plan does not reorder master Phase 0 through Phase 13.
- The SDS content already preserves the correct advisory-only and privacy-bounded boundary; the SDS and service catalog only need backlink and Phase 13 alignment wording updates.
- The master plan and service catalog alignment crosswalk need a new SDS #62 row and Phase 13 crosswalk inclusion so Lightweight Classifier is explicitly tied to Phase 12 delivery and Phase 13 hardening.
