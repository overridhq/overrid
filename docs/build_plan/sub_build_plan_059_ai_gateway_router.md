# SUB BUILD PLAN #59 - AI Gateway Router

Attached SDS: [docs/sds/ai_rag_model_routing/ai_gateway_router.md](../sds/ai_rag_model_routing/ai_gateway_router.md)

## Purpose

This sub-build plan turns SDS #59 into an implementation sequence for AI Gateway Router. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

AI Gateway Router is the Phase 6 product-routing authority for AI route requests from Docdex, Mcoda, Codali, central AI workflows, SDK/CLI/admin flows, and later Phase 12 native assistant/native app clients. It owns AI route request envelopes, classification fact bundles, context access plans, model/resource capability snapshots, route decisions, route attempts, fallback policies, routing usage refs, audit refs, and replay evidence. It does not run model inference, store full conversations, decrypt Docdex indexes, bypass Encrypted Docdex RAG Adapter authorization, replace Overguard policy decisions, mutate ORU/Seal Ledger accounting state, replace Central AI stewardship, hardcode model/provider names, or introduce conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #59: AI Gateway Router](../sds/ai_rag_model_routing/ai_gateway_router.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [AI Gateway Router plan](../service_catalog/ai_rag_model_routing/ai_gateway_router.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry control-plane records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies workload packaging, queue, lease, runner, raw usage, retry, cancellation, timeout, and dead-letter primitives for routed AI work that executes on grid resources. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Workload Classifier facts, Overguard policy decisions, Policy Dry-Run previews, trust/verification facts, public-provider eligibility guards, and deny-by-default semantics. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger accounting handoff boundaries, and the rule that the router emits usage evidence but does not maintain balances or price schedules. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Controls the first build point for product AI routing across Docdex, Mcoda, Codali, SDK, CLI, and admin diagnostics. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, namespace, private refs, encrypted context refs, model/artifact refs, and metadata-only replay substrates. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls native assistant and native app expansion that uses the same router contracts after product routing is proven. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance, central AI boundaries, threat review, incident handling, reporting, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #59 first build work aligned to master Phase 6, with Phase 12 as native-app expansion and Phase 13 as governance/security/compliance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 12, and 13 | Attach SDS #59, preserve Phase 6 as first build, define Phase 12 expansion, and freeze router authority boundaries. |
| 2 | Master Phases 0, 1, 4, 5, 6, and 8 | Define Rust contracts, canonical schemas, state machines, stable reason codes, signed refs, hashes, fixtures, and redaction-safe examples. |
| 3 | Master Phases 1, 4, 5, and 6 | Implement route request intake, dry-run, idempotency, deterministic facts, classification hints, and minimum product-routing envelopes. |
| 4 | Master Phases 4, 5, 6, and 8 | Implement context access planning, privacy gates, Overguard checks, budget prechecks, and deny-before-execution behavior. |
| 5 | Master Phases 1, 2, 3, 4, 5, 6, and 8 | Implement capability snapshot loading, candidate evaluation, immutable route decisions, and selected/rejected candidate explanations. |
| 6 | Master Phases 3, 4, 5, 6, and 8 | Implement route attempts, execution handoff, fallback, retry, cancellation, timeout, and stale-decision replacement behavior. |
| 7 | Master Phase 6, with prerequisites from Phases 0 through 5 and 8 | Prove product integrations for Docdex, Mcoda, Codali, SDK, CLI, admin diagnostics, and developer/operator views. |
| 8 | Master Phase 12, with prerequisites from Phases 4, 5, 6, and 8 | Expand the same router contracts to Personal AI Assistant, native apps, mobile handoffs, and local/native resource routing. |
| 9 | Master Phases 5, 6, 12, and 13 | Implement usage refs, Overwatch events, replay, audit, health, diagnostics, reporting, threat-review inputs, and governance hooks. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- AI Gateway Router core is a Rust service/module using shared contract crates, Tokio for bounded routing/fallback workers, and Axum/Tower/Hyper-style HTTP only where a service boundary is needed.
- Route requests, classification fact bundles, context access plans, capability snapshots, decisions, attempts, fallback policies, usage refs, events, fixtures, replay bundles, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating APIs require signed actor, app, service, or operator envelopes, tenant scope, privacy mode, data class, trace id, idempotency key, policy refs, context authorization refs, budget refs, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for request snapshots, capability candidate snapshots, context access plans, route decisions, attempt records, fallback records, replay bundles, audit exports, and deterministic fixtures.
- Model/provider/node choices must come from Overregistry capability refs and current availability snapshots. The router must not hardcode model names, provider names, price tables, private extraction logic, or external-provider products as platform boundaries.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw private prompt storage, direct private-context reads, final policy authority, final stewardship authority, or direct accounting mutation the router boundary.

## Phase 1: SDS Attachment, Phase 6 Scope, And Router Authority Boundaries

### Work Items

- **1.1 Attach the build plan to SDS #59.**
  - Design: Link this document from the AI Gateway Router SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/ai_rag_model_routing/ai_gateway_router.md`, `docs/service_catalog/ai_rag_model_routing/ai_gateway_router.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #59 returns both the AI Gateway Router SDS and this sub-build plan.

- **1.2 Preserve master Phase 6 as the first build point.**
  - Design: Keep the first implementation in Phase 6 because product integrations need product route requests, dry-run routing, model/resource capability matching, policy-aware context planning, route decisions, attempt refs, fallback refs, and usage refs before native apps expand the surface.
  - Output: Phase-gate note that Phases 0 through 5 supply contracts, identity, execution, policy, and accounting, Phase 6 builds product routing, Phase 8 supplies protected data/context refs, Phase 12 expands native clients, and Phase 13 hardens governance/security/compliance.
  - Validation: Review proves the plan does not move the router first build into Phase 12, does not delay core routing to Phase 13, and does not reorder the master Phase 0 through Phase 13 sequence.

- **1.3 Define Phase 12 as expansion, not first build.**
  - Design: Record that Personal AI Assistant, native apps, mobile handoffs, and user-facing route explanations reuse Phase 6 router contracts after product routing and protected-context prerequisites exist.
  - Output: Native expansion checklist covering user permission, encrypted Docdex RAG, ADES hints, Lightweight Classifier facts, local/native resource routing, wallet/usage visibility, and mobile handoff constraints.
  - Validation: Review confirms Phase 12 work does not fork a separate native-only router, bypass Overgate/Overpass/Overtenant/Overkey/Overguard/Overwatch/Overmeter, or weaken privacy/fallback constraints.

- **1.4 Freeze router ownership boundaries.**
  - Design: Record that the router owns request envelopes, fact aggregation, context access plans, model/resource candidate snapshots, decisions, attempts, fallback policies, usage refs, and replay evidence.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the router does not run inference, store conversations, decrypt RAG indexes, read private storage directly, replace Overguard, mutate accounting state, decide central AI stewardship, decide fraud/safety outcomes, or own downstream model workers.

- **1.5 Carry forward resolved SDS #59 decisions.**
  - Design: Preserve the minimum Phase 6 classification set, confirmation rules for widening beyond permission/budget/locality envelopes, and quality expectation representation as explicit classes and acceptance constraints rather than hidden ranking formulas.
  - Output: Resolved-decision checklist covering route request facts, classifier confidence, escalation requirements, confirmation requirements, route quality class, output mode, required capabilities, citation/context constraints, determinism/replay needs, and selected/rejected reason codes.
  - Validation: Review rejects silent route widening, hidden score-only decisions, model/provider hardcoding, unbounded fallback, and user-visible explanations that expose private ranking internals instead of satisfied/rejected requirements.

## Phase 2: Rust Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the AI Gateway Router Rust contract module.**
  - Design: Add contract types for route request, classification fact bundle, context access plan, capability snapshot, route decision, route attempt, fallback policy, usage ref, replay bundle, event payload, redaction profile, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, route lifecycle enums, attempt lifecycle enums, privacy-mode enums, quality/latency/budget classes, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from model worker internals, Encrypted Docdex RAG storage internals, Overguard policy internals, Overmeter accounting mutation, and Central AI authority.

- **2.2 Define route request and dry-run schemas.**
  - Design: Model `ai_route_request` and dry-run envelopes with caller refs, actor/tenant refs, app/workload refs, task type, requested output mode, privacy mode, data class, latency class, quality class, budget/limit refs, context refs, trace id, idempotency key, and confirmation state.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and canonical product-routing fixtures.
  - Validation: Schema tests reject missing actor/tenant refs, missing privacy mode, missing data class, missing budget/limit refs where required, unknown task/output classes, missing trace id, missing idempotency key, and unbounded context refs.

- **2.3 Define classification and context-plan schemas.**
  - Design: Model `classification_fact_bundle` and `context_access_plan` with deterministic caller facts, Lightweight Classifier refs, ADES hint refs, Workload Classifier refs, tool/RAG need facts, sensitivity facts, authorized Docdex index refs, retrieval scopes, vault grants, native service refs, denied context refs, redaction requirements, and explanation refs.
  - Output: Fact bundle schema, context plan schema, stable error catalog, redacted examples, denied-context examples, and negative fixtures.
  - Validation: Tests reject raw private prompt storage, unauthorized context refs, direct vault/object reads, missing policy refs, missing redaction requirements, and context plans that widen scope without explicit authorization.

- **2.4 Define capability, decision, attempt, fallback, and usage schemas.**
  - Design: Model `model_capability_snapshot`, `route_decision`, `route_attempt`, `fallback_policy`, and `routing_usage_ref` with registry refs, modality, context length class, tool support, privacy/locality class, adapter type, health, availability, quota/capacity, policy refs, selected target refs, rejected candidate reason codes, fallback order, attempt result class, retry/fallback state, and usage rollup refs.
  - Output: Capability schema, decision schema, attempt schema, fallback schema, usage schema, route replay schema, and role-scoped redaction profiles.
  - Validation: Tests prove decisions are immutable, candidate snapshots are hashable, rejected candidates have reason codes, fallback cannot weaken constraints, usage refs are not price schedules, and replay reconstructs decisions from stored facts.

- **2.5 Create deterministic router fixtures.**
  - Design: Build fixtures for product route success, dry-run denial, missing classification, authorized RAG context, unauthorized RAG context, ADES degraded, classifier unavailable, capability match, no capability match, unhealthy selected target, allowed fallback, disallowed fallback, budget limit exceeded, cancellation, timeout, stale capability snapshot, and route replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected reason codes, BLAKE3 hashes, redacted views, policy refs, usage refs, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, denial/degradation reason codes, audit refs, redacted outputs, usage refs, and replay outputs across repeated runs.

## Phase 3: Route Intake, Dry-Run, Classification Facts, And Minimum Product Routing

### Work Items

- **3.1 Implement route request intake.**
  - Design: Add `POST /ai-routes` with signed envelope checks, tenant scope, actor/service identity, schema validation, privacy mode, data class, context refs, budget refs, trace id, idempotency key, and audit refs.
  - Output: Request handler, request persistence/projection, idempotency behavior, stable errors, and `ai_gateway_router.route_requested` events.
  - Validation: API tests cover valid product requests, duplicate idempotency keys, missing identity, missing tenant, missing privacy mode, missing data class, unknown context refs, missing budget refs, malformed request, and audience-safe errors.

- **3.2 Implement side-effect-free dry-run.**
  - Design: Add `POST /ai-routes/dry-run` to preview likely route classes, denied candidates, context plan, confirmation requirements, missing prerequisites, budget class, and usage class without starting execution.
  - Output: Dry-run handler, dry-run response schema, missing-prerequisite facts, redacted candidate explanations, and no-execution audit refs.
  - Validation: Tests prove dry-run creates no route attempt, no queue job, no lease, no model call, no accounting mutation, and no raw private context leakage.

- **3.3 Implement deterministic request fact extraction.**
  - Design: Extract caller/product/workload family, actor/tenant/service-account refs, purpose/task class, privacy mode, data-class/sensitivity class, required context/RAG scope, tool/native-app/side-effect class, model-size/capability/modality hint, locality/trust class, latency class, budget/limit refs, escalation requirement, and stable reason codes.
  - Output: Fact extractor, fact bundle records, low-confidence flags, missing-fact errors, and deterministic fixture outputs.
  - Validation: Tests prove incomplete or low-confidence facts force `classification_required`, `context_not_authorized`, or escalation instead of silent routing.

- **3.4 Integrate Lightweight Classifier and optional ADES hints.**
  - Design: Ingest Lightweight Classifier outputs and optional ADES hint bundles as advisory facts while preserving deterministic facts and router-owned final route decisions.
  - Output: Classifier/hint ingestion adapters, confidence classes, degradation refs, stale-hint rejection, missing-hint behavior, and replay refs.
  - Validation: Tests prove missing ADES does not break routing, classifier unavailability follows degradation/denial policy, hints do not become final model/policy/safety/fraud authority, and stale hints are ignored or downgraded.

- **3.5 Implement minimum Phase 6 product-routing envelope.**
  - Design: Support Docdex, Mcoda, Codali, SDK, CLI, admin UI, and central AI service-to-service callers through the same route envelope and signed permission/budget manifests.
  - Output: Product caller adapters, manifest refs, route request examples, service-to-service confirmation profiles, and product route fixtures.
  - Validation: Tests prove product integrations do not bypass Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, route idempotency, or budget/context confirmation gates.

## Phase 4: Context Access Planning, Privacy, Policy, And Budget Gates

### Work Items

- **4.1 Implement context access planning.**
  - Design: Build context access plans through Encrypted Docdex RAG Adapter, Overvault, Overstore, tools, and native services where authorized, recording allowed scopes, denied scopes, redaction requirements, and context-plan hashes.
  - Output: Context planner, context plan records, denied-context records, retrieval-scope refs, redaction profiles, and `ai_gateway_router.context_plan_created` events.
  - Validation: Tests prove the router never decrypts Docdex indexes, reads private storage directly, widens RAG scope, persists raw private content, or hides denied context from route replay.

- **4.2 Implement privacy and locality gates.**
  - Design: Evaluate privacy mode, data class, locality requirement, trust class, model/resource locality, adapter type, and protected-context requirements before candidate selection or execution.
  - Output: Privacy gate, locality gate, failure reason codes, redacted denial responses, and `privacy_constraint_unsatisfied` behavior.
  - Validation: Tests prove private or regulated requests cannot route to resources lacking required trust/locality/encryption/sandbox profile and fallback cannot weaken privacy/locality.

- **4.3 Integrate Overguard and Policy Dry-Run.**
  - Design: Ask Overguard to evaluate tenant, role, data-class, context, tool, egress, budget, workload, and provider/resource constraints; expose Policy Dry-Run refs where callers need previews.
  - Output: Policy input adapter, policy decision refs, dry-run refs, matched rule refs, missing prerequisite refs, and `policy_denied` behavior.
  - Validation: Tests prove policy denial happens before execution, route decisions cite policy refs, dry-run stays side-effect-free, and router logic does not duplicate or override Overguard.

- **4.4 Implement budget and confirmation gates.**
  - Design: Require user, app, or signed service-manifest confirmation when a route widens beyond current permission, budget, locality, context bundle, leakage profile, side-effect class, or resource class.
  - Output: Confirmation policy, pre-authorized product profile, confirmation-required errors, budget class checks, and equivalent-health substitution rules.
  - Validation: Tests prove larger/higher-cost resource classes, weaker locality, expanded context, side-effecting tools, and central AI handoffs require confirmation, while equivalent-health substitutions inside the same class can proceed with reason-code evidence.

- **4.5 Implement denial diagnostics and remediation paths.**
  - Design: Provide bounded diagnostics for missing classification, unauthorized context, privacy failure, budget limit, capability gaps, stale registry data, and policy denial.
  - Output: Diagnostic response schema, caller-safe remediation summaries, admin-safe summaries, Overwatch audit refs, and support replay refs.
  - Validation: Tests prove diagnostics are actionable, role-scoped, and free of raw prompts, private documents, decrypted RAG content, model secrets, hidden scoring formulas, or sensitive policy internals.

## Phase 5: Capability Snapshots, Candidate Evaluation, And Route Decisions

### Work Items

- **5.1 Implement Overregistry capability lookup.**
  - Design: Load model/provider/node/adapter capability records from Overregistry with modality, context length class, tool support, privacy/locality class, adapter type, health, availability, quota/capacity, policy refs, and freshness metadata.
  - Output: Capability lookup adapter, visibility filters, freshness checks, candidate list records, and `capability_not_found` behavior.
  - Validation: Tests cover visible/invisible capabilities, stale registry snapshots, unavailable resources, unhealthy models, quota/capacity exhaustion, missing policy refs, and no hardcoded model/provider names.

- **5.2 Implement candidate snapshotting.**
  - Design: Freeze the candidate set used for a decision into a hashable `model_capability_snapshot` so replay can show selected and rejected candidates without exposing private capability internals.
  - Output: Candidate snapshot records, BLAKE3 hashes, redacted candidate views, snapshot freshness policies, and replay fixture hooks.
  - Validation: Tests prove route replay uses the original candidate snapshot, not later registry state, and stale snapshots force refresh or replacement decision behavior.

- **5.3 Implement least-sufficient route evaluation.**
  - Design: Evaluate candidates against required capabilities, privacy, locality, quality class, latency class, budget class, context/citation requirements, determinism/replay needs, tool/native-service needs, and policy refs.
  - Output: Evaluation engine, selected target refs, rejected candidate reason codes, quality expectation records, and route scoring evidence as explicit satisfied/rejected constraints.
  - Validation: Tests prove decisions are explainable by requirements, not opaque scores, and selected routes satisfy all required privacy, context, policy, capability, budget, and quality constraints.

- **5.4 Implement immutable route decisions.**
  - Design: Create `route_decision` records with selected target refs, candidate list hash, rejected candidate reason codes, fallback order, context plan ref, usage plan ref, policy decision refs, and replay refs.
  - Output: Decision writer, immutable decision records, decision reads, redacted decision summaries, and `ai_gateway_router.decision_created` events.
  - Validation: Tests prove decisions cannot be mutated after creation; new facts, expired snapshots, user confirmation, retries, or fallback changes create replacement decisions or route attempt records.

- **5.5 Implement route decision read and replay APIs.**
  - Design: Add `GET /ai-routes/{route_id}` and `GET /ai-routes/replay/{decision_id}` with role-scoped summaries of request, classification, context plan, candidates, selected route, attempts, fallback, policy refs, and usage refs.
  - Output: Read APIs, replay API, redacted projections, pagination for attempts, and support/admin views.
  - Validation: Tests prove replay reconstructs selected route, rejected candidates, policy refs, context plan, attempts, fallback refs, and usage refs without leaking raw prompts or private context.

## Phase 6: Route Attempts, Execution Handoff, Fallbacks, And Cancellation

### Work Items

- **6.1 Implement route attempt creation.**
  - Design: Add `POST /ai-routes/{route_id}/attempts` to record or start an execution attempt through the selected adapter/resource path with decision refs, target refs, queued job refs, lease/execution refs, trace ids, and idempotency keys.
  - Output: Attempt handler, attempt state records, queue handoff adapter, direct-local connector handoff where allowed, and `ai_gateway_router.attempt_started` events.
  - Validation: Tests cover valid attempt start, stale decision, missing selected target, duplicate attempt idempotency, resource unavailable, queue unavailable, and no execution when policy/context is denied.

- **6.2 Integrate Overqueue, Oversched, Overlease, and Overrun handoff.**
  - Design: Hand routed work to queue/scheduler/lease/runner for grid execution while preserving route id, context plan ref, target ref, package/adapter refs, policy refs, cancellation hooks, and usage refs.
  - Output: Queue command envelope, scheduling facts, lease refs, runner refs, route/attempt correlation ids, and handoff fixtures.
  - Validation: Tests prove routed work remains traceable from request through queue, scheduler, lease, runner, result, usage, and audit, with no internal API shortcuts.

- **6.3 Implement fallback policy and execution.**
  - Design: Add `POST /ai-routes/{route_id}/fallback` to evaluate next allowed fallback only inside the original or replacement decision constraints, with retry limits, degradation rules, user confirmation requirements, and privacy-preserving fallback constraints.
  - Output: Fallback evaluator, fallback records, fallback events, confirmation-required behavior, degraded route summaries, and `ai_gateway_router.fallback_selected` events.
  - Validation: Tests prove fallback cannot weaken privacy, context authorization, tenant, role, data-class, locality, budget, or quality constraints and cannot silently route to public/untrusted resources.

- **6.4 Implement cancellation, timeout, and failure handling.**
  - Design: Add `POST /ai-routes/{route_id}/cancel` and timeout/failure handling that propagates to pending route attempts, queue jobs, leases, execution refs, fallback state, and usage reconciliation.
  - Output: Cancellation handler, timeout jobs, failure reason records, retryable/terminal result classes, dead-letter refs, and `ai_gateway_router.route_cancelled` / `ai_gateway_router.attempt_failed` events.
  - Validation: Tests cover pending cancellation, running cancellation, timeout, retryable failure, terminal failure, selected target unhealthy, fallback loop prevention, and usage reconciliation before final completion.

- **6.5 Implement attempt lifecycle reads and reconciliation.**
  - Design: Expose attempt lifecycle through `queued`, `leased`, `running`, `succeeded`, `retryable_failed`, `terminal_failed`, `timed_out`, and `cancelled` states with usage refs and result refs.
  - Output: Attempt read projections, reconciliation jobs, final route summaries, result-class refs, and `ai_gateway_router.route_completed` / `ai_gateway_router.usage_emitted` events.
  - Validation: Tests prove route completion waits for required usage refs or preserves reconciliation refs, failure classes remain distinct, and downstream systems can explain attempt state.

## Phase 7: Phase 6 Product Integration Proof

### Work Items

- **7.1 Integrate Docdex encrypted RAG routing.**
  - Design: Route Docdex indexing, search, retrieval, context assembly, model-routing support, and encrypted index operations through product route envelopes and authorized RAG context plans.
  - Output: Docdex route request profiles, RAG context-plan refs, model/resource routing examples, usage refs, route replay fixtures, and admin/SDK examples.
  - Validation: Tests prove Docdex work uses Encrypted Docdex RAG Adapter for context authorization, never exposes decrypted index content to the router, and produces route, attempt, usage, and audit refs.

- **7.2 Integrate Mcoda agent workload routing.**
  - Design: Route agent tasks with explicit model/resource needs, tool-use boundaries, actor/tenant/repo refs, budget refs, context refs, and result expectations.
  - Output: Mcoda route adapter, agent task route examples, tool boundary refs, fallback constraints, and usage/audit refs.
  - Validation: Tests prove agent route decisions avoid hardcoded model/provider names, preserve tool-use boundaries, enforce policy/budget/context constraints, and propagate failure reasons.

- **7.3 Integrate Codali/code-agent routing.**
  - Design: Route code-agent phases, package execution, repository context refs, logs/artifacts, result capture, safe retry/repair loops, and resource usage per agent phase.
  - Output: Codali route adapter, package/phase route examples, artifact/log refs, repair-loop constraints, usage refs, and replay fixtures.
  - Validation: Tests prove repo/private context remains authorized, package execution goes through Overpack/queue/runner paths where required, retries remain bounded, and route replay captures selected/rejected candidates.

- **7.4 Integrate SDK, CLI, and admin diagnostics.**
  - Design: Expose route submission, dry-run, decision read, attempt read, fallback, cancellation, capability read, and replay through generated SDK/CLI bindings and admin diagnostics.
  - Output: SDK/CLI command contracts, admin route views, capability views, denial diagnostics, redacted replay views, and operator examples.
  - Validation: Tests prove SDK/CLI wrap signing, idempotency, trace ids, retries, error decoding, route reads, and dry-run without requiring direct internal API calls.

- **7.5 Prove product reliability cases.**
  - Design: Run product-driven route cases for success, retryable failure, terminal failure, cancellation, timeout, policy denial, budget exhaustion, node disconnect, stale capability, and disputed usage.
  - Output: Product reliability matrix, fixture cases, route replay outputs, usage refs, audit refs, and receipt/statement handoff refs.
  - Validation: Tests prove each case leaves readable route decision, attempt, policy, context, usage, and audit trails and can be inspected by product, operator, and support views.

## Phase 8: Phase 12 Native Assistant, Native Apps, And Mobile Expansion

### Work Items

- **8.1 Integrate Personal AI Assistant routing.**
  - Design: Route assistant requests with user permission, encrypted Docdex RAG context, ADES hints, Lightweight Classifier facts, local/native resource preferences, output mode, quality class, budget refs, and audit controls.
  - Output: Assistant route profiles, user confirmation profiles, local/private route examples, assistant fallback rules, and wallet/usage refs.
  - Validation: Tests prove the assistant uses the same AI Gateway Router contracts as products, not a native-only shortcut, and user private content stays within authorized context/locality rules.

- **8.2 Integrate native app AI route profiles.**
  - Design: Define route profiles for workspace, directory, search, messaging, social media, maps, wallet/usage, and central AI stewardship interface clients where AI or RAG assistance is needed.
  - Output: Native app route profile catalog, side-effect class mappings, context/scope refs, redaction profiles, fallback constraints, and usage refs.
  - Validation: Tests prove native apps cannot bypass identity, tenancy, policy, storage, usage, accounting, route replay, or user permission gates.

- **8.3 Integrate local and on-device/native resource routing.**
  - Design: Route to local resources, trusted private nodes, specialized adapters, or grid model resources based on privacy/locality, capability, quality, latency, budget, and availability constraints.
  - Output: Local resource capability refs, locality-aware candidate evaluation, on-device/native route examples, fallback rules, and degraded route summaries.
  - Validation: Tests prove local/private routes are preferred where required, fallback never leaves locality/privacy envelopes, and missing local capacity produces confirmation or denial rather than silent weakening.

- **8.4 Integrate mobile backend and mobile SDK handoffs.**
  - Design: Expose compact mobile route submission, dry-run, decision read, cancellation, replay summaries, sync cursors, push-notification refs, and offline replay constraints through Mobile Backend Gateway and Mobile SDK.
  - Output: Mobile route contracts, redacted push refs, sync cursor refs, offline command replay behavior, mobile denial diagnostics, and mobile fixture cases.
  - Validation: Tests prove mobile flows use Overrid identity/session rails, keep payloads redacted, preserve idempotency, and cannot bypass AI routing, policy, storage, usage, or audit systems.

- **8.5 Implement native user-facing route explanations.**
  - Design: Provide user-visible explanations based on satisfied/rejected requirements, confirmation requirements, selected resource class, privacy/locality class, context scope, fallback state, and usage class.
  - Output: Explanation schema, redacted assistant/native views, operator/debug views, localization-safe reason codes, and dispute/support refs.
  - Validation: Tests prove explanations are understandable, deterministic, and safe; they do not reveal raw private prompts, private docs, hidden ranking formulas, model secrets, or sensitive policy internals.

## Phase 9: Metering, Audit, Replay, Operations, And Governance Hooks

### Work Items

- **9.1 Implement routing usage refs.**
  - Design: Emit usage refs for classification, ADES enrichment, RAG retrieval, queue/scheduler work, model attempts, retries, fallbacks, cancellations, and degraded attempts without maintaining balances or encoding price schedules.
  - Output: Usage ref writer, Overmeter handoff, route/attempt usage summaries, reconciliation jobs, and `ai_gateway_router.usage_emitted` events.
  - Validation: Tests prove usage refs link to tenant, actor, app, route id, model/resource refs, context refs, result class, and Overmeter rollup refs without exposing private content or pricing assumptions.

- **9.2 Implement Overwatch audit and route timeline.**
  - Design: Emit route requested, classification completed, context plan created, route denied, decision created, attempt started, attempt failed, fallback selected, route completed, route cancelled, and usage emitted events.
  - Output: Event payloads, Overwatch integration, route timeline projections, redaction profiles, and operator/support views.
  - Validation: Tests prove events include refs, reason codes, policy refs, context plan refs, selected target refs, fallback refs, result class, and usage refs while excluding raw prompts, private docs, decrypted RAG content, and secrets.

- **9.3 Implement health, capability, and fallback diagnostics.**
  - Design: Track route volume by task/privacy/target/denial/fallback/failure class, model/provider health, registry freshness, classifier errors, ADES degradation, RAG authorization denials, queue/resource unavailability, and fallback-loop risks.
  - Output: Diagnostics APIs, operator projections, alerts, admin UI inputs, support summaries, and Overwatch refs.
  - Validation: Tests prove diagnostics are role-scoped, actionable, and free of raw private content, hidden model secrets, and speculative financial assumptions.

- **9.4 Implement governance and threat-review hooks.**
  - Design: Feed route replay, privacy constraint failures, fallback attempts, central AI handoffs, operator overrides, policy denial patterns, and suspicious route behavior into Phase 13 threat, compliance, incident, and reporting workflows.
  - Output: Governance export refs, incident trigger refs, threat-review fixture cases, central AI boundary refs, and compliance-safe summaries.
  - Validation: Tests prove governance hooks are evidence-backed, appeal/dispute-friendly, redacted, and do not let central AI or operators bypass router, policy, context, or accounting constraints.

- **9.5 Implement operator override boundaries.**
  - Design: Allow only signed, policy-referenced, audited operator overrides for bounded diagnostics or incident response, and never for weakening privacy/context/data-class/fallback constraints without explicit governed authority.
  - Output: Override schema, approval refs, expiration refs, Overwatch evidence, replay refs, and denial behavior.
  - Validation: Tests prove overrides require signer authority, reason codes, policy refs, expiry, audit refs, and replay evidence, and cannot silently hardcode model/provider routes or weaken protected context.

## Phase 10: Validation, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate sub-build plan structure.**
  - Design: Check the title prefix, attached SDS link, source alignment, master phase mapping, tech-stack guardrails, phase headings 1 through 10, and Design/Output/Validation work-item structure.
  - Output: Validation evidence in `docs/build_plan/progress.md` and queue progress.
  - Validation: Focused scripts pass for title, attached SDS, phase count, work-item count, local links, final newline, tabs, and stale markers.

- **10.2 Validate SDS, service catalog, master, and crosswalk backlinks.**
  - Design: Confirm the SDS points to this plan, the service catalog points to this plan, master plan has the SDS #59 row, and the crosswalk has the SDS #59 row.
  - Output: Backlink validation report and Docdex search evidence.
  - Validation: Link checks pass and Docdex search returns the SDS, service catalog, sub-build plan, master row, and crosswalk row in the aligned result set.

- **10.3 Validate stack and authority guardrails.**
  - Design: Scan changed active docs for conventional cloud/product-boundary drift, hardcoded model/provider decisions, raw private context storage, policy/accounting/stewardship ownership drift, and prohibited financial/speculative assumptions.
  - Output: Stack guardrail scan evidence and negative-control note for explicit non-choice lines.
  - Validation: Guardrail scan passes with only native Overrid service names, expected phase wording, or explicit non-choice/authority-boundary references.

- **10.4 Close queue and progress evidence.**
  - Design: Mark `059-build-plan` complete, clear any running state, make `060-build-plan` the next incomplete build-plan task, and record Docdex/validation/test-runner evidence.
  - Output: Updated `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, and `docs/build_plan/progress.md`.
  - Validation: JSON validation passes, queue totals are internally consistent, and progress docs list the correct next incomplete task.

- **10.5 Refresh Docdex and record handoff memory.**
  - Design: Refresh the Docdex index for the new and changed docs, run search/DAG evidence for SDS #59, save concise repo memory, and record any persistent validation blockers.
  - Output: Docdex index/search evidence, DAG export evidence, repo memory, and implementation handoff summary.
  - Validation: Docdex search for SDS #59 sub-build plan returns this file in the aligned result set; `docdexd run-tests` outcome is recorded, including any existing missing-runner blocker.

## Alignment Review

This plan keeps AI Gateway Router as a Phase 6 first-build service for product AI route decisions, dry-run previews, route decisions, fallbacks, attempts, usage refs, and replay evidence. Phase 12 is the native assistant/native app expansion of the same contracts, not a separate router. Phase 13 is governance, compliance, incident, security, and scale hardening.

No SDS correction is required for SDS #59 beyond linking this plan. The SDS already preserves the router's authority boundaries, explicitly rejects hardcoded model/provider names, and keeps routing separate from inference, conversation storage, encrypted RAG ownership, policy authority, accounting authority, and central AI stewardship.
