SDS #62

# Lightweight Classifier SDS

## Purpose

Cheaply classify request intent, sensitivity, tool needs, model size, RAG need, and safety/policy hints before using larger resources.

Lightweight Classifier is an advisory pre-routing component for AI work. It produces cheap, bounded labels and confidence facts that help AI Gateway Router avoid wasteful model routes. It does not enforce policy, make final safety decisions, authorize data access, choose the final model, or run user-facing conversations.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [lightweight_classifier.md](../../service_catalog/ai_rag_model_routing/lightweight_classifier.md) |
| Sub-build plan | [SUB BUILD PLAN #62 - Lightweight Classifier](../../build_plan/sub_build_plan_062_lightweight_classifier.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |

## Service Family

- Family: AI, RAG, and model routing
- Owning layer: Advisory request classification and routing hints
- Primary data scope: label taxonomies, classifier versions, classification requests/results, confidence thresholds, escalation rules, evaluation fixtures, calibration reports, drift signals, and usage refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md)

## Problem Statement

The Overrid AI layer cannot afford to send every request to a large model just to decide whether the request is a simple command, a search query, a private-context task, a code-agent request, a policy-sensitive action, or a native-app delegation. That would waste grid resources and recreate the inefficient AI economics Overrid is trying to fix.

A cheap classifier can reduce load, but it becomes dangerous if treated as authority. Its output must be explainable, confidence-bounded, versioned, evaluated, and always subject to policy and route checks.

## Goals

- Produce low-cost labels for intent, sensitivity, RAG need, tool need, native-app delegation need, model-size hint, output mode, and escalation risk.
- Support deterministic heuristics, small local models, and optional ADES enrichment without requiring any one mechanism.
- Return confidence, uncertainty, reason codes, label version, and fallback requirements.
- Escalate low-confidence, private, regulated, destructive, financial/accounting, safety-sensitive, or tool-mutating requests to stronger analysis.
- Feed advisory facts to AI Gateway Router, Personal AI Assistant, Encrypted Docdex RAG Adapter, and native apps.
- Maintain evaluation fixtures and calibration reports so classifier changes are testable before rollout.
- Meter material classification work without creating a hidden revenue or pricing model.

## Non-Goals

- Do not replace Overguard, Workload Classifier, AI Gateway Router, Central AI Service, or user permissions.
- Do not authorize encrypted Docdex RAG, native app tool calls, vault access, ledger/accounting mutations, or model routing by itself.
- Do not persist raw private prompts unless the caller explicitly grants retention for evaluation.
- Do not make one model, provider, or classifier architecture mandatory.
- Do not treat confidence as truth; low confidence and sensitive classes must escalate.
- Do not optimize for engagement, manipulation, or addictive product loops.
- Do not add pricing, customer-count, blockchain, NFT, or per-transaction fee assumptions.

## Primary Actors And Clients

- AI Gateway Router requesting classification facts before route selection.
- Personal AI Assistant classifying user turns before context and tool planning.
- ADES Enrichment Adapter providing optional entities, topics, warnings, and domain hints.
- Encrypted Docdex RAG Adapter consuming RAG-need and scope hints.
- Native apps requesting cheap classification for assistant, search, workspace, messaging, directory, and mobile flows.
- Overguard and Workload Classifier consuming or comparing advisory facts with policy/data-class facts.
- Overmeter and Overwatch consuming usage, audit, and evaluation refs.

## Dependencies

- [AI Gateway Router](ai_gateway_router.md) for final route decisions.
- [ADES Enrichment Adapter](ades_enrichment_adapter.md) for optional local enrichment hints.
- [Encrypted Docdex RAG Adapter](encrypted_docdex_rag_adapter.md) for authorized context retrieval when RAG is needed.
- [Personal AI Assistant](personal_ai_assistant.md) for user-facing request flow.
- [Overguard](../trust_policy_verification/overguard.md) and [Workload Classifier](../trust_policy_verification/workload_classifier.md) for policy and workload/data-class checks.
- [Overregistry](../control_plane/overregistry.md) for model/classifier version and capability refs.
- [Overmeter](../execution_scheduling/overmeter.md), [Overwatch](../control_plane/overwatch.md), and [Overbase](../data_storage_namespace/overbase.md) for usage, audit, and state.

## Owned Responsibilities

Lightweight Classifier owns:

- Versioned label taxonomy for AI request pre-classification.
- Classifier configuration, prompt/heuristic/model refs, rollout state, and compatibility windows.
- Classification request/result records with confidence, uncertainty, reasons, and redacted inputs.
- Escalation thresholds and fallback requirements for low-confidence or sensitive requests.
- Evaluation fixture sets, gold labels, calibration reports, regression gates, and drift signals.
- Advisory hint bundles handed to AI Gateway Router and Personal AI Assistant.
- Usage and audit refs for classification work.

## Data Model

- `classifier_taxonomy`: label names, descriptions, allowed values, compatibility version, owner, and deprecated labels.
- `classifier_version`: implementation type (`heuristic`, `small_model`, `hybrid`), model/capability refs, prompt/template refs, ADES dependency refs, rollout state, and eval gate refs.
- `classification_request`: actor/tenant refs, caller app, request hash, redacted text ref, metadata refs, ADES hint refs, privacy mode, max latency class, trace id, and idempotency key.
- `classification_result`: intent labels, sensitivity label, data-class hints, RAG need, tool need, native-app target hint, model-size hint, output-mode hint, confidence, uncertainty, reason codes, and escalation requirement.
- `confidence_policy`: threshold by label, privacy mode, data class, caller app, and required fallback behavior.
- `escalation_record`: trigger reason, next service, route ref, user confirmation requirement, and downstream policy refs.
- `evaluation_fixture`: input class, gold labels, privacy class, expected escalation, allowed retained text status, and fixture source refs.
- `calibration_report`: version ref, fixture set, confusion matrix summary, false-negative classes, drift indicators, rollout decision, and reviewer refs.
- `classification_usage_ref`: classifier version, token/compute class, latency class, batch size, and Overmeter refs.

Private prompt text should be ephemeral by default. Durable evaluation records use redacted text, hashes, synthetic fixtures, or user-approved retained examples.

## API Surface

- `POST /lightweight-classifier/classify`: classifies one request and returns advisory labels.
- `POST /lightweight-classifier/classify/batch`: bounded batch classification for app workflows.
- `POST /lightweight-classifier/dry-run`: previews labels and escalation without affecting route state.
- `GET /lightweight-classifier/taxonomy`: returns visible label taxonomy and compatibility version.
- `GET /lightweight-classifier/versions`: lists active, canary, deprecated, and blocked classifier versions.
- `POST /lightweight-classifier/evaluations`: runs an evaluation fixture set for a classifier version.
- `GET /lightweight-classifier/evaluations/{eval_id}`: returns calibration and regression results.
- `POST /lightweight-classifier/versions/{version_id}/rollout`: starts, pauses, promotes, or rolls back a classifier rollout.

Mutating APIs require service identity, tenant/system scope where applicable, trace id, idempotency key, privacy mode, and policy refs. Stable errors include `taxonomy_version_unsupported`, `classifier_version_unavailable`, `input_too_large`, `private_input_retention_denied`, `classification_low_confidence`, `evaluation_gate_failed`, `rollout_not_allowed`, and `policy_denied`.

## Event Surface

- `lightweight_classifier.classification_requested`: classification request accepted.
- `lightweight_classifier.classification_completed`: advisory labels produced.
- `lightweight_classifier.classification_escalated`: low-confidence or sensitive request escalated.
- `lightweight_classifier.classification_denied`: request denied by size, privacy, policy, or unavailable classifier.
- `lightweight_classifier.taxonomy_changed`: label taxonomy updated.
- `lightweight_classifier.version_rollout_changed`: classifier rollout state changed.
- `lightweight_classifier.evaluation_completed`: evaluation fixture run completed.
- `lightweight_classifier.drift_detected`: drift or regression detected for a classifier version.
- `lightweight_classifier.usage_emitted`: usage refs emitted.

Events include classifier version refs, taxonomy version, label summaries, confidence classes, escalation reason, evaluation refs, and usage refs. They must not include raw private prompts or unredacted sensitive content.

## Core Workflow

1. Caller submits request metadata, optional redacted text ref, privacy mode, and any ADES hints.
2. Classifier validates input size, privacy constraints, taxonomy version, and classifier availability.
3. Classifier runs deterministic heuristics and/or a small local model within latency and cost bounds.
4. Classifier normalizes output into labels, confidence, uncertainty, and reason codes.
5. Confidence policy marks the result as usable, advisory-only, or escalation-required.
6. Result is handed to AI Gateway Router, Personal AI Assistant, or native app as a hint bundle.
7. Usage and audit refs are emitted; raw private input is dropped unless retention is explicitly allowed.

## State Machine

Classification lifecycle:

1. `submitted`
2. `validated`
3. `enriched`
4. `classified`
5. `confidence_checked`
6. `completed`
7. `escalated`
8. `denied`
9. `failed`

Classifier version lifecycle:

1. `draft`
2. `fixture_ready`
3. `evaluating`
4. `canary`
5. `active`
6. `paused`
7. `rolled_back`
8. `deprecated`
9. `blocked`

Evaluation lifecycle:

1. `queued`
2. `running`
3. `passed`
4. `failed`
5. `review_required`
6. `archived`

## Policy And Security

- Treat all classifier output as advisory facts, never as final permission or route authority.
- Escalate requests with low confidence, policy-sensitive classes, private/regulatory data, destructive tool actions, financial/accounting implications, or ambiguous target service.
- Do not retain raw private input for evaluation without explicit consent and retention policy.
- Redact or hash prompt text in durable records and events.
- Classifier versions must pass evaluation gates before promotion.
- Rollbacks must be fast and preserve replay of which classifier version produced a route hint.
- Operator overrides for rollout require signed action and Overwatch evidence.

## Metering And Accounting

- Emit usage refs for classifier calls, batch calls, small-model invocations, evaluation runs, and canary/rollback diagnostics.
- Link usage to caller app, tenant, actor where allowed, classifier version, request class, and route id when available.
- Tiny local heuristic calls may be aggregated rather than individually accounted, but still need operational visibility.
- Failed and escalated calls must be visible for capacity and quality review.
- Do not encode hardcoded prices, revenue projections, or model-provider payout rules.

## Observability And Operations

- Expose request volume, latency, confidence distribution, escalation rate, label distribution, error rate, model health, and drift signals.
- Alert on false-negative sensitive classes, sudden confidence shifts, high escalation loops, stale classifier versions, and evaluation regressions.
- Provide replay using taxonomy version, classifier version, ADES hint refs, input hash, output labels, confidence policy, and escalation rule.
- Provide SDK/CLI/admin diagnostics for taxonomy changes, active versions, canary state, and evaluation results.

## Failure Modes And Recovery

- Classifier unavailable: return escalation-required and let AI Gateway Router use deterministic facts or stronger analysis.
- ADES unavailable: classify without ADES hints and mark degraded.
- Input too large: request caller to provide metadata or a bounded summary; do not silently truncate policy-sensitive text.
- Low confidence: mark escalation-required and block cheap-route-only decisions.
- Invalid classifier output: discard result and escalate.
- Evaluation regression: pause rollout, keep prior active version, and emit drift/regression event.
- Usage emission fails: retain classification refs for reconciliation before marking completed.

## Validation Plan

- Simple low-risk requests classify cheaply and route with small-resource hints.
- Low-confidence, private, regulated, destructive, accounting, and tool-mutating requests escalate.
- Classifier output never bypasses Overguard, context grants, route decisions, or user permissions.
- Taxonomy and classifier version are included in every result and replay.
- Raw private prompts are absent from events, logs, metrics, and default durable records.
- Evaluation fixtures catch false-negative sensitive classes before rollout.
- Rollback restores the prior version and preserves audit of affected route hints.

## Build Breakdown

1. Define taxonomy, classifier version, request/result, confidence policy, escalation, evaluation, calibration, drift, and usage schemas.
2. Implement deterministic baseline classification and small local model connector.
3. Integrate optional ADES hints and caller metadata normalization.
4. Implement confidence thresholds, escalation records, dry-run, and AI Gateway Router handoff.
5. Build evaluation fixtures for assistant tasks, RAG decisions, native app delegation, tool calls, and sensitive classes.
6. Add rollout, canary, rollback, observability, usage, and replay.
7. Harden privacy retention and false-negative gates before Phase 12 native assistant launch.

## Handoff And Downstream Use

Lightweight Classifier hands advisory label bundles, confidence facts, escalation requirements, taxonomy/version refs, usage refs, and replay evidence to AI Gateway Router, Personal AI Assistant, ADES Enrichment Adapter, Encrypted Docdex RAG Adapter, native apps, Overguard, Overwatch, SDK, CLI, and admin UI.

Downstream services must treat classifier output as hints and must run their own policy, context, and route checks.

## Open Design Questions

Resolved decisions:

- The first personal AI assistant build should use a compact Phase 12 taxonomy rather than the full future routing ontology: intent or purpose class, privacy/data-class hint, RAG or context-source need, native-app or tool-delegation need, side-effect class, model-size/capability hint, output-mode hint, latency/budget class, confidence class, uncertainty class, escalation requirement, and stable reason codes. The allowed values should cover ordinary chat, summarization, search/RAG, writing/editing, code/repository help, workspace or messaging assistance, wallet/usage reads, native-app delegation, tool proposal, central-AI or stewardship handoff, and unknown/ambiguous requests. This is sufficient only because AI Gateway Router, Encrypted Docdex RAG Adapter, Personal AI Assistant, Overguard, Workload Classifier, and owning native apps still make the authority, context, route, and permission decisions.
- Hard escalation is required regardless of classifier confidence when a request is low-confidence or ambiguous, asks for new or widened private context, touches secret-bearing or regulated data, requests destructive or side-effecting tool/native-app actions, changes identity/credential/permission state, affects ORU, Seal Ledger, Overbill, Overgrant, payouts, disputes, or accounting records, attempts external egress, requests safety/medical/legal/financial guidance, enters abuse/fraud/moderation/stewardship review paths, or conflicts with current route, context, tenant, locality, budget, or policy facts. The classifier may emit hints for these requests, but downstream flow must require stronger analysis, Overguard or owning-service policy checks, and user/app confirmation where applicable.
- The first classifier should be hybrid: deterministic Rust heuristics and schema-normalized caller metadata form the baseline, optional ADES hints enrich the input, and a small local model may classify non-sensitive ambiguous language when policy permits. Deterministic gates own input size, privacy mode, known tool/side-effect markers, permission widening, and hard escalation classes; small-model output is advisory, versioned through classifier refs, and discarded or escalated when unavailable, invalid, stale, or low confidence. The first build must not require a remote provider, a hardcoded model name, or a model-only decision path.
- Evaluation quality should improve through synthetic fixtures, redacted golden examples, stable reason-code fixtures, BLAKE3 input/output hashes, label-only correction records, ADES and route refs, and owner-approved encrypted evaluation artifacts with explicit purpose, scope, expiry, and revocation. Raw private prompts, raw files, unredacted spans, secrets, and private app payloads stay ephemeral by default; retaining an example for calibration requires the owner or tenant to store a bounded encrypted artifact through the owning context/storage service, with Overwatch evidence and deletion/redaction on expiry, revocation, dispute closure, or policy change.
- User-visible metrics should be limited to safe request-level and account-level summaries: classification status, label categories, confidence band, escalation or confirmation reason, privacy/context/tool implications, route handoff refs, usage/receipt refs, and privacy-audit refs. Operator-only views may show aggregate label distribution, latency, escalation rate, confidence histograms, false-negative sensitive-class checks, evaluation/calibration outcomes, drift signals, version rollout state, degradation/error rates, and usage by resource class. Neither audience should receive raw private prompts, decrypted context, unredacted sensitive spans, other-tenant facts, model secrets, exact abuse/fraud thresholds, or hidden policy heuristics outside an authorized evidence view.
