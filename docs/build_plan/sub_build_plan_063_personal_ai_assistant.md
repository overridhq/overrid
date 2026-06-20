# SUB BUILD PLAN #63 - Personal AI Assistant

Attached SDS: [docs/sds/ai_rag_model_routing/personal_ai_assistant.md](../sds/ai_rag_model_routing/personal_ai_assistant.md)

## Purpose

This sub-build plan turns SDS #63 into an implementation sequence for Personal AI Assistant. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Personal AI Assistant is the Phase 12 user-facing native AI surface that coordinates assistant sessions, user permissions, context-source selection, encrypted Docdex RAG, model/resource routing, tool-call proposals, native-app delegation, response refs, privacy audit, and ORU usage visibility. It uses Central AI Service only for bounded stewardship/evidence handoffs, and it delegates to owning native apps instead of replacing their data, permissions, policy, or state boundaries. It does not store raw user data as canonical state, bypass Overvault or Encrypted Docdex RAG grants, mutate accounting or ledger state, choose one hardcoded model/provider, or become Central AI Service.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #63: Personal AI Assistant](../sds/ai_rag_model_routing/personal_ai_assistant.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Personal AI Assistant service plan](../service_catalog/ai_rag_model_routing/personal_ai_assistant.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry app/service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Policy Dry-Run previews, deny-by-default behavior, and review-required signals that assistant convenience cannot override. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, Wallet/Usage Center handoffs, ORU/Seal Ledger receipt refs, and the rule that the assistant displays usage but does not mutate accounting truth. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies AI Gateway Router product route requests, encrypted Docdex RAG jobs, context authorization, route metadata, usage, replay, and first product proof for personal/repo context. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase assistant state, Overstore response/artifact refs, Overvault private refs and permission grants, namespace refs, protected content refs, retention classes, and metadata-only replay. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first build point for Wallet/Usage Center, Personal AI Assistant, native app delegation, encrypted context use, advisory classification, model/resource routing, and mobile handoffs. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies hardening for assistant permission abuse, tool-delegation side effects, private-context leakage, revocation bypass, unsafe-output repair, mobile offline replay, privacy audit, incident response, security review, and compliance controls. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #63 first build work aligned to master Phase 12 with Phase 6 product/RAG/model-routing groundwork and Phase 13 governance/security/compliance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, hardcoded model/provider, raw-private-prompt retention, or hidden data-extraction drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 12, and 13 | Attach SDS #63, preserve Phase 12 as first build, record prerequisites, and freeze the assistant delegation/permission boundary. |
| 2 | Master Phases 0, 1, 4, 5, 6, 8, and 12 | Define Rust contracts, canonical schemas, state machines, stable errors, signed refs, hashes, fixtures, and redaction-safe examples. |
| 3 | Master Phases 1, 4, 5, 8, and 12 | Implement assistant profiles, sessions, turns, privacy defaults, permission manifests, and user-visible lifecycle state. |
| 4 | Master Phases 4, 6, 8, and 12 | Integrate Lightweight Classifier, ADES hints, AI Gateway Router, and Encrypted Docdex RAG context planning without widening permissions. |
| 5 | Master Phases 4, 6, 8, and 12 | Implement context-source preview, route/request-bound grants, revocation, RAG bundle refs, citations, and privacy audit records. |
| 6 | Master Phases 4, 5, 8, and 12 | Implement tool proposal, confirmation, cancellation, delegated native-app calls, Wallet permission-control proof, and owning-service handoffs. |
| 7 | Master Phases 5, 6, 8, and 12 | Capture response refs, usage receipt refs, route/model/tool provenance, Overwatch replay, wallet visibility, and client projections. |
| 8 | Master Phases 1, 5, 8, 12, and 13 | Implement SDK, CLI, admin/support diagnostics, mobile/offline intent envelopes, sync handoffs, and operator-safe diagnostics. |
| 9 | Master Phase 13, with prerequisites from Phases 0 through 12 | Harden privacy, revocation, unsafe-output repair, fallback, prompt/context retention, tool-loop abuse, mobile replay, and incident behavior. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Personal AI Assistant core is a Rust service/module using shared contract crates, Tokio for bounded turn/context/tool/replay workers, and Axum/Tower/Hyper-style HTTP only where a service boundary is needed.
- Assistant profiles, sessions, turns, permission manifests, context-source selections, tool proposals, delegated call records, response refs, privacy audit records, usage receipt refs, events, fixtures, replay bundles, mobile intent envelopes, and diagnostics use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant scope, assistant/app/client refs, trace id, idempotency key, privacy mode, permission refs, context grant refs, policy refs, route refs where applicable, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for user input refs, redacted text refs, context-source selections, context bundle snapshots, tool proposals, delegated call results, response refs, mobile offline intent envelopes, replay bundles, audit exports, and deterministic fixtures.
- Storage, queueing, private records, object refs, namespace, audit, usage, policy, identity, keys, context grants, and accounting handoffs must use native Overrid service boundaries such as Overbase, Overqueue, Overstore, Overvault, Universal Namespace Service, Overpass, Overtenant, Overkey, Overwatch, Overmeter, Overguard, ORU Account Service, Seal Ledger, AI Gateway Router, Encrypted Docdex RAG Adapter, Lightweight Classifier, ADES Enrichment Adapter, Central AI Service, and Wallet/Usage Center.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw private prompts in logs/events/metrics/default durable records, hardcoded model/provider names, direct private-store reads, final policy authority, final accounting mutation, or addictive engagement loops the assistant boundary.

## Phase 1: SDS Attachment, Phase 12 Scope, And Assistant Boundary

### Work Items

- **1.1 Attach the build plan to SDS #63.**
  - Design: Link this document from the Personal AI Assistant SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/ai_rag_model_routing/personal_ai_assistant.md`, `docs/service_catalog/ai_rag_model_routing/personal_ai_assistant.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #63 returns both the Personal AI Assistant SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep first implementation in Phase 12 because the assistant depends on Wallet/Usage Center controls, encrypted RAG context grants, product AI routing, policy rails, usage visibility, and native-app clients.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 6, and 8 supply contracts, identity, policy, metering, AI routing/RAG, and protected storage prerequisites; Phase 12 builds the assistant; Phase 13 hardens it.
  - Validation: Review proves the plan does not move the assistant into Phase 6 routing, Phase 8 storage, Central AI governance, or Phase 13-only hardening, and does not reorder master Phase 0 through Phase 13.

- **1.3 Freeze the assistant ownership boundary.**
  - Design: Record that the assistant owns profiles, sessions, turns, permission manifests, context-source selections, tool proposals, delegated call records, confirmation records, response refs, privacy audit records, usage receipt refs, and user-visible controls.
  - Output: Ownership checklist for architecture, API, implementation, and review gates.
  - Validation: Review confirms the assistant does not store raw app data as canonical truth, bypass context grants, mutate ORU/Seal Ledger/grants/payouts/disputes, enforce policy, choose final governance outcomes, or replace owning native apps.

- **1.4 Carry forward resolved SDS #63 decisions.**
  - Design: Preserve the resolved rules for Wallet/Usage Center permission-control as the first side-effecting action, per-turn grants by default, user-visible privacy audit fields, mobile offline intent envelopes, and bounded unsafe-output repair.
  - Output: Resolved-decision checklist covering permission revocation proposals, reusable grant narrowing, audit redaction, offline revalidation, and repair-before-confirmation limits.
  - Validation: Review rejects broad persistent context grants, hidden raw prompt/decrypted snippet exposure, offline execution without revalidation, assistant-controlled accounting mutation, and repair paths that widen privacy/tool/model permissions without confirmation.

- **1.5 Define downstream and owning-service boundaries.**
  - Design: Record how AI Gateway Router, Encrypted Docdex RAG Adapter, Lightweight Classifier, ADES Enrichment Adapter, Central AI Service, Wallet/Usage Center, native apps, mobile, SDK, CLI, admin UI, Overguard, Overwatch, and Overmeter interact with assistant refs.
  - Output: Consumer-boundary matrix naming allowed inputs, owned outputs, denied direct authority, usage refs, audit refs, and replay requirements.
  - Validation: Review confirms downstream services keep final routing, context authorization, native-app permissions, side-effect execution, policy, usage/accounting, and governance boundaries.

## Phase 2: Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the Personal AI Assistant Rust contract module.**
  - Design: Add contract types for assistant profile, session, turn, permission manifest, context-source selection, tool proposal, delegated call record, assistant response ref, privacy audit record, usage receipt ref, mobile intent envelope, event payload, replay bundle, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, privacy-mode enums, tool side-effect enums, confirmation enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from AI Gateway final routing, encrypted RAG internals, native-app state mutation, policy enforcement, and accounting internals.

- **2.2 Define profile, permission, and context-source schemas.**
  - Design: Model assistant profile settings, privacy mode, allowed context source classes, allowed tool classes, confirmation policy, retention defaults, accessibility/app prefs, permission manifests, grant refs, expiry, revocation state, and context-source selections.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, route/request-bound grant fixtures, reusable grant fixtures, and denial fixtures.
  - Validation: Schema tests reject missing user/tenant refs, unscoped reusable grants, missing expiry, missing purpose/data-class refs, direct private-store reads, and permission manifests that authorize side effects without policy/confirmation rules.

- **2.3 Define session, turn, response, and audit schemas.**
  - Design: Model assistant sessions, turns, request hashes, redacted text refs, classifier refs, ADES hint refs, context plan refs, route request refs, response refs, citations, route/model provenance, safety/quality flags, privacy audit records, and retention decisions.
  - Output: Schema set, lifecycle state machines, redacted examples, BLAKE3 hash examples, stable error catalog, and replay fixtures.
  - Validation: Tests reject turns without privacy mode, tenant/actor refs, trace id, idempotency key, permission refs, classifier/route/context refs where required, response refs without citations/provenance, and audit records that expose raw private prompts or unauthorized snippets.

- **2.4 Define tool proposal and delegated call schemas.**
  - Design: Model target app/service, operation, parameter refs, side-effect class, confirmation requirement, policy refs, rollback/cancel behavior, confirmed actor refs, result refs, failure reason, and usage refs.
  - Output: Tool proposal schema, delegated call record schema, confirmation/rejection/cancellation examples, first Wallet permission-control fixtures, native-app handoff fixtures, and invalid side-effect fixtures.
  - Validation: Tests reject proposals without target owner, side-effect class, policy refs, confirmation requirements where needed, rollback/cancel behavior, usage refs, and Overwatch audit refs.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for simple chat, private RAG, repo-context coding help, workspace summarization, messaging triage, wallet permission revocation, native-app delegation, rejected tool proposal, route unavailable, grant revoked, offline turn replay, unsafe output repair, and Central AI handoff.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected stable errors, BLAKE3 hashes, redacted projections, usage refs, audit refs, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, audit refs, redacted outputs, usage refs, confirmation outcomes, and replay output across repeated runs.

## Phase 3: Sessions, Turns, Privacy Defaults, And Permission Manifests

### Work Items

- **3.1 Implement assistant profile APIs.**
  - Design: Add profile create/read/update APIs for privacy defaults, allowed context classes, allowed tool classes, confirmation defaults, retention defaults, accessibility preferences, and app integration preferences.
  - Output: `POST /personal-ai/profiles`, profile read/update handlers, profile records, idempotency behavior, stable errors, and `personal_ai_assistant.profile_updated` events.
  - Validation: API tests cover valid profiles, duplicate idempotency, missing actor/tenant refs, unsupported privacy modes, broad tool defaults, retention-denied profiles, and audience-safe errors.

- **3.2 Implement session lifecycle APIs.**
  - Design: Add `POST /personal-ai/sessions` and `GET /personal-ai/sessions/{session_id}` with signed envelope checks, tenant/user/client refs, privacy mode, permission manifest refs, active grant refs, route history refs, usage summaries, and retention class.
  - Output: Session handlers, state transitions, idempotent creation, closed/expired/deleted behavior, audit refs, and `personal_ai_assistant.session_created` events.
  - Validation: Tests prove sessions require actor/tenant/client refs, privacy mode, permission manifest refs, trace id, idempotency key, and cannot reopen expired/deleted sessions without new authorization.

- **3.3 Implement turn submission and state tracking.**
  - Design: Add `POST /personal-ai/sessions/{session_id}/turns` and `GET /personal-ai/turns/{turn_id}` with request hashing, redacted input refs, normalized request metadata, current permissions, lifecycle state, cancellation, and visible response state.
  - Output: Turn handlers, request records, lifecycle state machine, cancellation rules, stable errors, and `personal_ai_assistant.turn_submitted` events.
  - Validation: Tests cover valid turns, missing permission manifest, revoked grants, duplicate idempotency, cancelled sessions, privacy-mode mismatch, oversized input, redacted storage behavior, and terminal-state reads.

- **3.4 Implement permission manifest creation and revocation.**
  - Design: Add `POST /personal-ai/permissions` and `POST /personal-ai/permissions/{permission_id}/revoke` with scope, purpose, data classes, app/session, tool/native-service use, retention, expiry, revocation status, and confirmation requirements.
  - Output: Permission manifest writer, revocation handler, active-session invalidation records, reauthorization prompts, audit refs, and `personal_ai_assistant.permission_revoked` events.
  - Validation: Tests prove revocation blocks future context/tool access, active sessions move to waiting/denied/cancelled states where required, broad grants require explicit confirmation, and revocation never mutates Wallet/Usage Center state directly.

- **3.5 Implement privacy-default enforcement.**
  - Design: Apply privacy mode, retention class, raw prompt handling, redacted text refs, route/request-bound grants, reusable grant limits, and user/organization policy defaults before classification, RAG, routing, or tool planning.
  - Output: Privacy default evaluator, denied/waiting states, retention records, redaction/hash records, and stable `permission_required`, `privacy_mode_unsatisfied`, and `grant_revoked` behavior.
  - Validation: Tests prove raw private prompts are absent from logs/events/metrics/default records, privacy defaults cannot be weakened by caller metadata, and denied context/tool access can produce useful limited responses where allowed.

## Phase 4: Classification, ADES Hints, Context Planning, And Route Requests

### Work Items

- **4.1 Integrate Lightweight Classifier.**
  - Design: Attach advisory intent, privacy/data-class, RAG need, tool/native-app delegation, model-size/capability, confidence, uncertainty, and escalation facts to each turn before context planning or routing.
  - Output: Classifier client adapter, hint refs, escalation markers, user-visible safe summaries, usage refs, audit refs, and `personal_ai_assistant.classification_attached` events.
  - Validation: Tests prove classifier hints cannot authorize context, suppress confirmations, choose final models, weaken Workload Classifier or Overguard facts, or bypass final safety review.

- **4.2 Integrate optional ADES hints.**
  - Design: Accept local-only ADES entity/topic/warning/domain-pack hints where policy allows, mark degraded or stale hints, and use them only as advisory context-planning and route-planning inputs.
  - Output: ADES client adapter, hint-normalization records, degraded-hint behavior, stale-hint rejection, usage refs, and replay refs.
  - Validation: Tests prove missing ADES does not block the turn, stale/invalid hints cannot lower sensitivity, private/regulated content stays local-only, and ADES cannot make final policy/routing/tool decisions.

- **4.3 Implement context planning before retrieval.**
  - Design: Build context-source plans from request metadata, classifier/ADES hints, permission manifests, privacy mode, owner scopes, purpose refs, data classes, leakage constraints, result caps, retention class, and route needs.
  - Output: Context plan record, selected and denied source refs, redaction plan, required grant refs, denied reason codes, and replay snapshots.
  - Validation: Tests prove context plans do not retrieve content, do not widen permissions, record denied refs safely, and require fresh authorization for new owner/source/data/leakage/retention scope.

- **4.4 Integrate AI Gateway Router route requests.**
  - Design: Send route requests with classifier facts, context plan refs, authorized context bundle refs where available, privacy mode, model/resource capability needs, budget/latency class, fallback rules, usage refs, and replay refs.
  - Output: Router client adapter, route request refs, route decision refs, route unavailable behavior, fallback records, and `personal_ai_assistant.route_requested` events.
  - Validation: Tests prove the assistant does not hardcode model/provider names, bypass route decisions, widen fallback privacy constraints, or run model work without route evidence.

- **4.5 Implement route-decision handling and degradation.**
  - Design: Handle selected routes, denied routes, route unavailable, fallback denied, timeout, cancellation, stale route snapshots, and repair-required outcomes without changing context/tool permissions.
  - Output: Route-state updater, degradation records, retry/fallback limits, user-facing summaries, operator-safe diagnostics, and audit refs.
  - Validation: Tests prove degradation is replayable, role-scoped, and never weakens privacy, locality, policy, budget, model/resource, or tool-permission envelopes.

## Phase 5: Context Source Preview, Grants, RAG Bundles, And Privacy Audit

### Work Items

- **5.1 Implement context-source preview.**
  - Design: Add `POST /personal-ai/context-sources/preview` to show available, denied, degraded, and grant-required context sources before retrieval using owner scopes, data classes, leakage profiles, permission refs, and result caps.
  - Output: Preview handler, source alias projections, denied refs, grant-required prompts, leakage summaries, expected usage class, and `personal_ai_assistant.context_requested` events.
  - Validation: Tests prove preview never retrieves raw snippets, exposes only authorized metadata, and returns stable reason codes for scope, policy, key, leakage, and permission denials.

- **5.2 Integrate Encrypted Docdex RAG Adapter.**
  - Design: Request authorized context only through Encrypted Docdex RAG Adapter using context grants, leakage profiles, retrieval dry-run refs, retrieval result refs, context bundle refs, redaction profiles, and route refs.
  - Output: RAG adapter client, context grant refs, retrieval refs, bundle refs, citations, denied-context records, usage refs, and replay refs.
  - Validation: Tests prove the assistant does not decrypt Docdex indexes, read private storage directly, bypass grants, suppress leakage profiles, retain raw unauthorized snippets, or hide denied context.

- **5.3 Implement grant lifecycle and revocation awareness.**
  - Design: Track route/request-bound grants by default and reusable grants only when explicitly scoped by owner, purpose, app/session, data class, leakage profile, result/snippet caps, retention class, allowed tool/native-service use, expiry, and revocation state.
  - Output: Grant lifecycle records, renewal/narrowing behavior, widening confirmation prompts, active turn invalidation, and `grant_revoked` handling.
  - Validation: Tests prove widening requires fresh authorization, revocation blocks future access, active turns cancel or wait where policy requires, and grant renewal cannot hide changed leakage/capability facts.

- **5.4 Implement privacy audit reads.**
  - Design: Add `GET /personal-ai/privacy-audit/{session_or_turn_id}` with session/turn refs, purpose/privacy mode, context source aliases, owner scopes, data classes, grants, classifier/ADES categories, route/resource class, tool refs, usage refs, retention state, and Overwatch refs.
  - Output: Privacy audit handler, user-visible projections, support/operator projections, redaction rules, and audit export hooks.
  - Validation: Tests prove audit views omit raw private prompts, decrypted snippets outside authorized bundles, vault/key material, unauthorized paths, other users' records, hidden fraud/policy thresholds, model/provider secrets, and operator-only diagnostics.

- **5.5 Implement context failure and missing-permission recovery.**
  - Design: Handle missing permission, context authorization denied, stale index, revoked grant, redaction failure, route-bound bundle expiry, and partial context availability with useful constrained responses where allowed.
  - Output: Recovery state rules, reauthorization prompts, degraded response markers, denied context summaries, retry records, and audit refs.
  - Validation: Tests prove failures do not silently widen context, leak denied source names beyond allowed aliases, block unrelated safe responses unnecessarily, or create context bundles when redaction/grant/key/policy state is invalid.

## Phase 6: Tool Proposals, Confirmations, And Native-App Delegation

### Work Items

- **6.1 Implement tool proposal creation.**
  - Design: Create tool proposals for native-app or service actions with target service, operation class, parameter refs, side-effect class, confirmation requirement, policy refs, rollback/cancel behavior, expected usage, and audit refs.
  - Output: Proposal writer, `tool_confirmation_required` behavior, user-visible proposal summaries, policy input projections, and `personal_ai_assistant.tool_proposed` events.
  - Validation: Tests prove side-effecting, destructive, permission-changing, identity/credential, wallet/accounting, publishing, messaging, regulated, or cross-tenant actions require confirmation or owning-service review.

- **6.2 Implement Wallet/Usage Center permission-control proof.**
  - Design: Build the first side-effecting delegated native-app action as a confirmed request to narrow or revoke assistant, AI context, tool, or native-app permission through Wallet/Usage Center's owning-service handoff path.
  - Output: Wallet permission-control proposal, confirmation view, delegated call record, expected active-session effects, rollback/reauthorization path, policy refs, usage refs, and Overwatch audit refs.
  - Validation: Tests prove the proposal cannot mutate balances, ledger entries, grants, payouts, disputes, resource rates, or accounting truth, and must show scope, owner, effect, and confirmation before handoff.

- **6.3 Implement confirmation, rejection, and cancellation APIs.**
  - Design: Add `POST /personal-ai/tool-proposals/{proposal_id}/confirm` and `/reject` with signed actor refs, current permission checks, policy refs, idempotency, state transitions, cancellation behavior, and replay refs.
  - Output: Confirmation/rejection handlers, state machine transitions, terminal reason codes, `personal_ai_assistant.tool_confirmed` events, and cancelled proposal records.
  - Validation: Tests prove invalid transitions are rejected, revoked permissions block confirmation, duplicate idempotency is safe, rejected proposals do not execute, and cancelled actions remain replayable.

- **6.4 Implement delegated native-app call execution.**
  - Design: Execute confirmed or policy-allowed delegated calls through owning native-app APIs with command refs, result refs, usage refs, failure reasons, cancellation rules, and no direct app storage access.
  - Output: Delegated call executor, app/service adapters, result refs, failure records, retry rules, and `personal_ai_assistant.tool_executed` events.
  - Validation: Contract tests prove native apps still own domain permissions and side effects, assistant calls cannot mutate app data directly, and failures preserve proposal/result/reason/usage refs.

- **6.5 Implement tool-loop and side-effect guardrails.**
  - Design: Bound repeated tool proposals, automatic calls, confirmation prompts, retries, route/tool loops, external egress attempts, and cross-app delegations with user preference, policy, usage, and audit limits.
  - Output: Loop detector, retry limits, escalation records, operator diagnostics, user-facing warnings, and incident handoff hooks.
  - Validation: Tests prove the assistant cannot create hidden automation loops, repeatedly prompt for denied side effects, bypass confirmation by splitting actions, or hide tool failures from privacy audit.

## Phase 7: Responses, Usage, Wallet Visibility, Replay, And Client Projections

### Work Items

- **7.1 Implement response refs and citation handling.**
  - Design: Capture assistant response refs with output refs, citations, route decisions, model/run provenance, safety/quality flags, redaction profiles, degraded-context markers, and user-visible explanation refs.
  - Output: Response writer, citation refs, route/model provenance refs, redacted projections, response-ready/completed states, and `personal_ai_assistant.response_created` events.
  - Validation: Tests prove responses cannot expose unauthorized snippets, missing citations are flagged, unsafe output is blocked before display, and response refs remain replayable from stored refs.

- **7.2 Implement usage and wallet receipt handoffs.**
  - Design: Emit usage refs for classification, ADES enrichment, context retrieval, model routes, tool calls, storage, bandwidth, and response generation, then hand receipt refs to Wallet/Usage Center.
  - Output: Usage receipt writer, Overmeter handoff, Wallet display refs, pending receipt reconciliation, and `personal_ai_assistant.usage_emitted` events.
  - Validation: Tests prove failed, denied, cancelled, confirmation-rejected, and completed operations are visible with reason codes, and the assistant never mutates balances, ledger entries, payment refs, payout state, or grant truth.

- **7.3 Implement Overwatch replay bundles.**
  - Design: Build replay for a turn from permissions, classifier refs, ADES refs, context refs, route decision, tool confirmations, response refs, privacy audit, usage refs, and retention decisions.
  - Output: Replay bundle schema, replay API/projection, hashable snapshots, role-scoped export, and operator-safe diagnostics.
  - Validation: Replay tests reconstruct successful, denied, degraded, grant-revoked, route-unavailable, rejected-tool, failed-tool, unsafe-output, and offline-replayed turns without raw private payloads.

- **7.4 Implement user-facing client projections.**
  - Design: Provide web/desktop/mobile client projections for session state, turns, permissions, source previews, proposal confirmations, privacy audit, usage summaries, citations, and response status.
  - Output: Client projection APIs, TypeScript/web binding targets from generated contracts where needed, accessibility fields, pagination, and stable error decoding.
  - Validation: Tests prove clients receive only user-authorized projections, long labels fit structured fields, pagination is deterministic, and support/operator-only diagnostics are absent from user views.

- **7.5 Implement admin and support-safe diagnostics.**
  - Design: Provide admin/support views for route failures, context denials, permission errors, tool proposal outcomes, usage reconciliation, latency, and audit refs without raw private prompts or decrypted context.
  - Output: Admin/support diagnostics, metrics fields, redaction profiles, Overwatch links, and incident handoff summaries.
  - Validation: Tests prove diagnostics include enough reason codes for support while excluding raw prompts, decrypted snippets, private app payloads, hidden thresholds, key material, and model/provider secrets.

## Phase 8: SDK, CLI, Mobile, Offline Intent, And Native-App Expansion

### Work Items

- **8.1 Add SDK and CLI bindings.**
  - Design: Generate SDK/CLI bindings for sessions, turns, permissions, context preview, tool proposal confirmation/rejection, privacy audit, usage, and replay using shared contracts and signing/idempotency helpers.
  - Output: Rust SDK bindings, TypeScript/web bindings where required for clients, CLI commands, stable JSON output, fixture-backed examples, and docs snippets.
  - Validation: Contract tests prove SDK/CLI commands preserve signing, tenant scope, idempotency, trace ids, privacy mode, stable errors, pagination, and redacted projections.

- **8.2 Implement mobile backend handoff.**
  - Design: Integrate Mobile Backend Gateway and Mobile SDK for assistant sessions, compact turns, sync cursors, push notification refs, media/context handoffs, wallet/usage reads, and AI/RAG flow state.
  - Output: Mobile projection contract, sync/delta records, redacted push payload refs, device/session refs, route-bound bundle expiry awareness, and mobile usage refs.
  - Validation: Tests prove mobile clients cannot bypass Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, storage, AI routing, context grants, or native-app permission checks.

- **8.3 Implement offline intent envelopes.**
  - Design: Queue mobile offline assistant turns only as signed, idempotent, expiring intent envelopes with request hashes, privacy mode, permission snapshot refs, and payload refs or hashes.
  - Output: Offline intent envelope schema, enqueue handler, reconnect revalidation, expired/revoked behavior, discard rules, and audit refs.
  - Validation: Tests prove reconnect revalidates device/session state, actor/tenant authority, permission grants, context capability snapshots, policy refs, and route constraints before classification, retrieval, routing, or tool execution.

- **8.4 Expand native-app delegation targets safely.**
  - Design: Add first read-only or low-risk integrations for workspace, messaging, search, directory, maps, social, wallet, and central AI stewardship views after each owning app exposes permissioned APIs.
  - Output: Native-app adapter registry, allowed operation matrix, per-app permission scopes, side-effect classes, confirmation rules, fixture set, and app-specific usage refs.
  - Validation: Contract tests prove each native app remains owner of its data and operations, and the assistant cannot infer write permission from read permission or route convenience.

- **8.5 Implement rollout, feature flags, and compatibility gates.**
  - Design: Roll out assistant features by capability refs, app readiness, permission scopes, route support, context adapter support, mobile client version, safety gates, and audit/replay support.
  - Output: Capability/feature gate records, canary flags, compatibility windows, rollback flags, migration notes, and blocked-reason projections.
  - Validation: Tests prove unavailable or blocked capabilities produce explicit degraded states, not hidden behavior; rollback preserves sessions, permissions, audit refs, and usage reconciliation.

## Phase 9: Privacy, Security, Revocation, Fallback, And Unsafe-Output Hardening

### Work Items

- **9.1 Harden private-context and prompt retention.**
  - Design: Audit raw prompt handling, context bundle expiry, retained example gates, redaction profiles, privacy audit projections, support exports, incident/legal/compliance retention exceptions, and deletion paths.
  - Output: Retention hardening checklist, privacy tests, export controls, tombstone behavior, incident retention profiles, and redaction review records.
  - Validation: Tests prove raw private prompts and decrypted snippets are absent from default durable records and exports unless explicit policy allows a bounded retained ref.

- **9.2 Harden permission revocation and active-turn cancellation.**
  - Design: Make revocation interrupt future context/tool access, cancel or pause active work where policy requires, invalidate route/context bundles, and force reauthorization for widened or changed scope.
  - Output: Revocation propagation worker, active-turn cancellation rules, route/bundle invalidation records, reauthorization prompts, and Overwatch audit refs.
  - Validation: Tests prove revoked grants cannot be reused by retries, fallbacks, offline envelopes, tool proposals, context bundles, or mobile sync after revocation.

- **9.3 Harden unsafe-output repair.**
  - Design: Limit pre-confirmation repair to bounded non-side-effecting schema/format repair, citation/ref alignment, redaction cleanup, refusal/safety text normalization, or one route-approved retry under same or stricter envelope.
  - Output: Unsafe-output blocker, repair state machine, allowed repair matrix, review-required records, blocked display/execution behavior, and audit refs.
  - Validation: Tests prove repair cannot widen context, change permissions, use a larger/less-local/higher-cost resource class, invoke tools/native apps/Central AI, affect identity/wallet/accounting/disputes, publish/send/edit content, or run other side effects without confirmation or owning-service review.

- **9.4 Harden fallback, degradation, and route/tool loops.**
  - Design: Bound route fallback, context retry, tool retry, model repair, missing receipt reconciliation, repeated confirmation prompts, native-app failures, and degraded response behavior.
  - Output: Fallback policy, retry budgets, loop detector, escalation records, incident handoff hooks, and operator diagnostics.
  - Validation: Tests prove fallback cannot widen privacy, context, tenant, role, locality, budget, model/resource, or tool-permission envelopes, and loop detection prevents repeated hidden automation.

- **9.5 Run Phase 13 threat and security review.**
  - Design: Threat-model assistant-specific risks: prompt/context leakage, permission confusion, native-app side effects, tool-loop abuse, mobile offline replay, unsafe-output repair drift, wallet permission-control misuse, Central AI handoff overreach, and addiction/dark-pattern pressure.
  - Output: Threat model records, security review findings, mitigations, tracked remediation work, acceptance records, incident playbook hooks, and compliance boundary notes.
  - Validation: Review proves every threat has mitigation, test coverage, monitoring, accepted-risk record, or incident response path before broad native-app launch.

## Phase 10: Documentation, Validation, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build plan structure.**
  - Design: Check that this document starts with `SUB BUILD PLAN #63`, links the attached SDS immediately after the title, uses phases 1 through 10, and gives each phase internally consistent Design/Output/Validation work items.
  - Output: Structure validation evidence covering title, attached SDS link, phase headings, work-item count, work-item shape, local links, final newline, and tab checks.
  - Validation: Focused validation script passes for title prefix, attached SDS link, phase headings 1 through 10, 50 work items, five work items per phase, Design/Output/Validation structure, and local Markdown links.

- **10.2 Validate documentation alignment.**
  - Design: Confirm SDS #63, the service catalog plan, master build plan, crosswalk, Phase 12, Phase 13, and tech-stack decision agree on first build phase, dependencies, boundaries, and guardrails.
  - Output: Alignment evidence in `docs/build_plan/progress.md`, including no required master Phase 0 through Phase 13 reordering and any targeted wording corrections.
  - Validation: Review confirms Phase 12 first build, Phase 6 groundwork, Phase 13 hardening, no Central AI or accounting authority drift, and no conventional-cloud or speculative-economics drift.

- **10.3 Validate stack guardrails and stale markers.**
  - Design: Scan active #63 docs for tech-stack non-choices, raw-private-prompt retention, hardcoded model/provider names, hidden data extraction, final policy/accounting authority drift, and unresolved stale markers.
  - Output: Stack guardrail scan results, stale marker scan results, and exception notes for negative-control terms from tech-stack non-choice wording.
  - Validation: Scans pass with only expected native Overrid service names, accepted tech-stack non-choice terms, or explicit authority-boundary wording.

- **10.4 Update queue and progress evidence.**
  - Design: Mark `063-build-plan` complete in `.codex55_sds_queue/state.json`, update `.codex55_sds_queue/progress.md`, add build-plan progress evidence, and refresh Docdex indexing for the new/changed docs.
  - Output: `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, `docs/build_plan/progress.md`, Docdex index refresh, Docdex search results, and DAG/session evidence where available.
  - Validation: JSON validation passes, queue counts advance from SDS #63 to SDS #64, Docdex search returns the new #63 plan with SDS/service backlinks, and no unrelated queue tasks are changed.

- **10.5 Prepare implementation handoff.**
  - Design: Summarize implementation order, first contracts, first APIs, first Wallet permission-control proof, native-app delegation boundaries, mobile/offline gates, hardening gates, and validation fixtures for the future builder.
  - Output: Handoff checklist in this plan, progress evidence, repo memory, and Docdex-indexed retrieval entry.
  - Validation: A future implementation agent can identify first files/contracts/APIs/tests from this plan without changing master phase order or violating `docs/overrid_tech_stack_choice.md`.
