SDS #59

# AI Gateway Router SDS

## Purpose

Route AI requests to the right model, node, provider, or local resource based on request nature, privacy, budget, availability, and capability.

AI Gateway Router owns route-decision infrastructure for Overrid AI work. It classifies the request, checks privacy and policy constraints, selects context and model-resource options, creates auditable route decisions, handles fallback, and emits usage refs. It does not own user conversations, model inference, encrypted RAG indexes, ADES enrichment, policy authority, or central AI stewardship decisions.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [ai_gateway_router.md](../../service_catalog/ai_rag_model_routing/ai_gateway_router.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md), [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |

## Service Family

- Family: AI, RAG, and model routing
- Owning layer: AI request classification, model/resource route selection, fallback, and audit
- Primary data scope: route requests, classification facts, privacy constraints, context access plans, model/resource capability snapshots, route decisions, route attempts, fallback policies, usage refs, and audit evidence
- First build phase from service plan: [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md) for product routing; [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) for the native assistant

## Problem Statement

Overrid cannot run a sustainable AI layer by sending every request to one expensive model or one provider. Different requests need different resources: local classification, ADES hints, encrypted Docdex RAG, small models, large models, code agents, central AI review, or native service calls. The grid should find available model resources and use the least sufficient path that satisfies privacy, policy, quality, latency, and budget constraints.

AI Gateway Router must make those choices explicit and replayable. A route decision must show why a model/resource was selected, what context was allowed, what fallbacks were permitted, and how usage will be accounted for.

## Goals

- Accept AI route requests from products, native apps, personal assistant, Docdex, Mcoda, Codali, and central AI workflows.
- Classify request nature using deterministic facts, Lightweight Classifier, optional ADES hints, and caller-supplied task metadata.
- Check privacy, tenant, role, data-class, context authorization, and budget constraints before routing.
- Match requests to model/resource capabilities from Overregistry and available grid capacity.
- Produce route decisions with selected target, fallback order, context access plan, usage plan, and reason codes.
- Avoid hardcoded model names and provider lock-in.
- Meter routing, model attempts, retries, fallbacks, and result metadata without exposing encrypted RAG inputs.

## Non-Goals

- Do not run model inference directly; model workers, adapters, Overrun, or provider connectors execute inference.
- Do not store full user conversations or private documents.
- Do not decrypt Docdex indexes or bypass Encrypted Docdex RAG Adapter authorization.
- Do not replace Overguard policy decisions, Overmeter usage truth, ORU accounting, or Central AI stewardship.
- Do not make ADES or any single classifier mandatory.
- Do not choose models based on financial projections or private extraction logic.
- Do not add blockchain, NFT, or per-transaction fee assumptions.

## Primary Actors And Clients

- Personal AI Assistant and native apps submitting user AI requests.
- Docdex Adapter, Mcoda Adapter, Codali Adapter, and mSwarm Runtime Bridge submitting product workloads.
- Encrypted Docdex RAG Adapter providing authorized context refs.
- Lightweight Classifier and ADES Enrichment Adapter providing request hints.
- Overregistry publishing model/resource capability refs.
- Overguard checking policy, privacy, and data-class constraints.
- Oversched and Overqueue placing route-selected work onto available resources.
- Overmeter, ORU Account Service, Wallet/Usage Center, and Overwatch consuming usage and audit refs.
- Central AI Service submitting bounded stewardship or evidence-analysis work.

## Dependencies

- [Overregistry](../control_plane/overregistry.md) for model, provider, node, adapter, and capability records.
- [Overguard](../trust_policy_verification/overguard.md), [Workload Classifier](../trust_policy_verification/workload_classifier.md), and [Policy Dry-Run API](../trust_policy_verification/policy_dry_run_api.md) for policy and data-class facts.
- [Overqueue](../control_plane/overqueue.md), [Oversched](../execution_scheduling/oversched.md), [Overlease](../execution_scheduling/overlease.md), and [Overrun](../execution_scheduling/overrun.md) for route execution on grid resources.
- [Encrypted Docdex RAG Adapter](encrypted_docdex_rag_adapter.md), [ADES Enrichment Adapter](ades_enrichment_adapter.md), and [Lightweight Classifier](lightweight_classifier.md) for context and routing hints.
- [Overvault](../data_storage_namespace/overvault.md), [Overstore](../data_storage_namespace/overstore.md), and [Overbase](../data_storage_namespace/overbase.md) for authorized context/data refs.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), and [Overwatch](../control_plane/overwatch.md) for usage and audit.

## Owned Responsibilities

AI Gateway Router owns:

- AI route request envelopes and normalized task metadata.
- Classification fact bundles and hint aggregation.
- Privacy, budget, latency, quality, locality, and capability constraints used for routing.
- Context access plans referencing authorized RAG, files, tools, and native services.
- Model/resource capability matching and candidate snapshots.
- Route decision records with stable reason codes and fallback order.
- Route attempt records and fallback execution state.
- Routing audit, replay bundles, and usage refs.

## Data Model

- `ai_route_request`: caller refs, actor/tenant refs, app/workload refs, task type, requested output mode, privacy mode, data class, latency class, quality class, budget/limit refs, context refs, trace id, and idempotency key.
- `classification_fact_bundle`: deterministic caller facts, Lightweight Classifier output refs, ADES hint refs, Workload Classifier refs, tool/RAG need facts, sensitivity facts, and confidence/explanation refs.
- `context_access_plan`: authorized Docdex index refs, retrieval scopes, vault grants, tool refs, native service refs, redaction requirements, and denied context refs.
- `model_capability_snapshot`: model/provider/node refs, modality, context length class, tool support, privacy/locality class, adapter type, health, availability, quota/capacity, and policy refs.
- `route_decision`: selected target refs, candidate list hash, rejected candidate reason codes, fallback order, context plan ref, usage plan ref, policy decision refs, and replay refs.
- `route_attempt`: decision ref, target ref, queued job ref, lease/execution refs, start/end times, result class, failure reason, retry/fallback status, and usage refs.
- `fallback_policy`: allowed fallback classes, retry limits, degradation rules, user confirmation requirements, and privacy-preserving fallback constraints.
- `routing_usage_ref`: classification usage, enrichment usage refs, RAG usage refs, model attempt refs, queue/compute refs, and Overmeter rollup refs.

Route decisions are immutable after creation. New conditions, retries, or fallbacks create route attempt records and, where needed, replacement decisions.

## API Surface

- `POST /ai-routes`: creates a route request and returns a route decision or queued route decision ref.
- `POST /ai-routes/dry-run`: previews likely route, denied candidates, context plan, and usage class without executing.
- `GET /ai-routes/{route_id}`: returns route request, decision summary, attempts, and redacted evidence.
- `POST /ai-routes/{route_id}/attempts`: records or starts an execution attempt through the selected adapter/resource.
- `POST /ai-routes/{route_id}/fallback`: evaluates and starts the next allowed fallback.
- `POST /ai-routes/{route_id}/cancel`: cancels pending route attempts and propagates cancellation to queue/execution refs.
- `GET /ai-routes/capabilities`: returns available model/resource capability summaries visible to the caller.
- `GET /ai-routes/replay/{decision_id}`: reconstructs a route decision from stored facts, policies, and capability snapshots.

Mutating APIs require actor/service identity, tenant scope, trace id, idempotency key, policy refs, and context authorization refs. Stable errors include `classification_required`, `context_not_authorized`, `privacy_constraint_unsatisfied`, `budget_limit_exceeded`, `capability_not_found`, `model_unhealthy`, `resource_unavailable`, `fallback_not_allowed`, `policy_denied`, and `route_expired`.

## Event Surface

- `ai_gateway_router.route_requested`: route request accepted.
- `ai_gateway_router.classification_completed`: classification/hint bundle completed.
- `ai_gateway_router.context_plan_created`: context access plan created.
- `ai_gateway_router.route_denied`: routing denied before execution.
- `ai_gateway_router.decision_created`: route decision created.
- `ai_gateway_router.attempt_started`: route attempt started.
- `ai_gateway_router.attempt_failed`: selected target failed or timed out.
- `ai_gateway_router.fallback_selected`: fallback target selected.
- `ai_gateway_router.route_completed`: route completed successfully.
- `ai_gateway_router.route_cancelled`: route cancelled.
- `ai_gateway_router.usage_emitted`: routing/model/RAG usage refs emitted.

Events include route refs, candidate snapshot refs, policy refs, context plan refs, selected target refs, fallback refs, result class, and usage refs. They must not include raw private prompts, private documents, decrypted RAG content, or model secrets.

## Core Workflow

1. Receive an AI route request from an app, assistant, adapter, or stewardship workflow.
2. Validate identity, tenant, idempotency, schema, privacy mode, and context refs.
3. Gather deterministic request facts, optional ADES hints, Lightweight Classifier output, Workload Classifier facts, and caller-supplied metadata.
4. Build a context access plan through Encrypted Docdex RAG Adapter, Overvault, Overstore, tools, and native services where authorized.
5. Query Overregistry and resource health for model/provider/node capabilities.
6. Ask Overguard to evaluate privacy, data-class, tenant, budget, tool, and context constraints.
7. Select the least sufficient allowed route, record rejected candidates and reason codes, and create fallback order.
8. Hand execution to the selected adapter/resource path through queue/scheduler/runner or direct local connector as allowed.
9. Record attempts, failures, fallbacks, usage refs, result refs, and replay evidence.

## State Machine

Route lifecycle:

1. `submitted`
2. `classified`
3. `context_authorized`
4. `candidates_loaded`
5. `policy_checked`
6. `decision_created`
7. `attempting`
8. `fallback_pending`
9. `completed`
10. `denied`
11. `cancelled`
12. `failed`

Attempt lifecycle:

1. `queued`
2. `leased`
3. `running`
4. `succeeded`
5. `retryable_failed`
6. `terminal_failed`
7. `timed_out`
8. `cancelled`

## Policy And Security

- Route decisions must honor data-class, privacy, locality, role, tenant, context authorization, and workload-class constraints.
- Private or regulated requests cannot route to resources that lack the required trust, locality, encryption, or sandbox profile.
- Context refs must be resolved through authorized adapters; the router must not read private storage directly.
- Model names and providers must come from registry/capability refs, not hardcoded routing tables.
- Fallback cannot weaken privacy, context authorization, or data-class constraints.
- Route replay must explain selected and rejected candidates without exposing raw prompts or private context.
- Operator overrides require signed action, policy refs, and Overwatch evidence.

## Metering And Accounting

- Emit usage refs for classification, ADES enrichment, RAG retrieval, queue/scheduler work, model attempts, retries, fallbacks, and cancellations.
- Link usage to tenant, actor, app, route id, model/resource refs, context refs, and result class.
- Failed or degraded attempts should be visible to accounting and dispute systems with reason codes.
- Wallet/Usage Center and ORU Account Service consume usage summaries downstream; the router does not maintain balances.
- Do not encode financial projections, hardcoded prices, or provider payout logic.

## Observability And Operations

- Expose route volume by task class, privacy class, selected target class, denial reason, fallback reason, latency class, and failure class.
- Track model/provider health, registry freshness, classifier errors, ADES degradation, RAG authorization denials, and queue/resource unavailability.
- Alert on privacy-constraint violations, fallback loops, hardcoded model refs, stale capability snapshots, and unusually high model failure rates.
- Provide route replay for support, disputes, and central AI stewardship review.
- Provide bounded dry-run diagnostics for SDK, CLI, admin UI, and native apps.

## Failure Modes And Recovery

- Classifier unavailable: route using deterministic facts and emit degradation if policy allows; otherwise deny.
- ADES unavailable: continue without ADES hints and record degradation.
- RAG context unauthorized: route without that context or deny if context is required.
- No model/resource capability matches: return denied with missing capability reason codes.
- Selected model unhealthy: attempt allowed fallback if privacy and policy constraints still hold.
- Queue or scheduler unavailable: keep route pending or fail with retryable reason.
- Model output failure: record attempt failure and choose fallback according to policy.
- Usage emission fails: preserve route and attempt refs for later usage reconciliation before final completion.

## Validation Plan

- Router avoids hardcoded model names and uses registry capability refs.
- Privacy-restricted requests do not route to disallowed resources.
- Fallback never weakens privacy, context, tenant, role, or data-class constraints.
- Route decisions are auditable and metered.
- Dry-run explains candidate acceptance/denial without side effects.
- ADES and Lightweight Classifier degradation do not break allowed routing.
- RAG authorization failures are visible and do not leak context.
- Replay reconstructs selected route, rejected candidates, policy refs, context plan, attempts, and usage refs.

## Build Breakdown

1. Define route request, classification facts, context plan, capability snapshot, decision, attempt, fallback, and usage schemas.
2. Implement route request and dry-run APIs with idempotency and stable errors.
3. Integrate Lightweight Classifier, ADES hints, Workload Classifier facts, and deterministic request metadata.
4. Integrate Encrypted Docdex RAG Adapter and context authorization refs.
5. Integrate Overregistry capability lookup, Overguard policy checks, and Overmeter usage refs.
6. Add route attempt/fallback state and execution handoff to product adapters or Overrid queue/scheduler.
7. Add observability, replay, validation fixtures, and SDK/CLI/admin diagnostics.

## Handoff And Downstream Use

AI Gateway Router hands route decisions, context plans, selected target refs, fallback refs, attempt refs, usage refs, and replay bundles to Personal AI Assistant, Central AI Service, Encrypted Docdex RAG Adapter, ADES Enrichment Adapter, Lightweight Classifier, Docdex/Mcoda/Codali adapters, Overqueue, Oversched, Overmeter, Wallet/Usage Center, Overwatch, SDK, CLI, and admin UI.

Downstream execution components must follow the selected route contract and report attempts back to the router.

## Open Design Questions

Resolved decisions:

- The minimum Phase 6 product-routing classification set is a normalized request envelope with caller/product/workload family, actor/tenant/service-account refs, purpose/task class, privacy mode, data-class/sensitivity class, required context/RAG scope and leakage facts, tool/native-app/side-effect class, model-size/capability/modality hint, locality/trust class, latency class, budget/limit refs, classifier confidence/uncertainty, escalation requirement, and stable reason codes. Phase 6 does not need the full Phase 12 native assistant taxonomy or ADES-specific domain packs to be mandatory; deterministic caller metadata plus Lightweight Classifier and optional ADES hints are enough when incomplete or low-confidence facts force `classification_required`, `context_not_authorized`, or escalation instead of silent routing.
- User or app confirmation is required before a route widens beyond the caller's current permission, budget, or locality envelope: using a larger or higher-cost resource class, leaving local/private capacity for less-local/provider/public capacity, sending private or regulated context to a resource with weaker trust/locality, expanding a context bundle or leakage profile, invoking side-effecting tool/native-app/central-AI handoff, or overriding session/profile constraints. Product integrations in Phase 6 may pre-authorize bounded service-to-service confirmations through signed permission manifests and budget refs, but an equivalent-health substitution inside the same privacy, data-class, locality, capability, and budget class can proceed automatically with reason-code evidence.
- Route decisions represent quality expectations as explicit classes and acceptance constraints, not an opaque score. The decision records requested output mode, quality class, required capabilities, context/citation requirements, determinism or replay need, latency/budget limits, safety/repair requirements, selected and rejected candidate reason codes, and the model/resource capability snapshot used at decision time. Classifier confidence and uncertainty remain advisory evidence; user-visible explanations should say which requirements were satisfied or rejected rather than exposing hidden ranking formulas.
- Automatic fallback is allowed only for retryable, non-side-effecting, policy-equivalent failures where the next target stays inside the same or stricter privacy, context, tenant, role, data-class, locality, budget, and tool-permission envelope: unhealthy selected target, transient adapter/queue/scheduler failure, timeout, unavailable ADES/classifier hints when deterministic facts suffice, or invalid/unsafe model output that can be repaired under the same constraints. Explicit user/app retry or new authorization is required for policy denial, budget exhaustion, missing or revoked context grants, privacy/locality mismatch, no matching capability, side-effecting or accounting/native-app actions, fallback loops, higher-cost/larger/less-local resources, or any fallback that would widen context or tool permissions.
- Private assistant route replay retention follows the same classed pattern as Policy Dry-Run and Overwatch rather than a single global window. Caller-visible route summaries, selected/rejected reason codes, policy refs, capability snapshot hashes, usage refs, and redacted context-plan refs are retained for 30 days by default for user audit and receipts. Private-data, secret-bearing, regulated, or native-app permission replay bundles keep request summaries and detailed replay facts for 7 days unless pinned by user support, Overclaim dispute, incident, policy rollout investigation, legal hold, or compliance policy; route/admission mismatch or rollout comparison evidence keeps redacted replay bundles for 90 days. Raw prompts, decrypted RAG content, vault secrets, and private app payloads are not copied into router storage; after expiry the archive keeps only hashes, ids, version refs, reason codes, usage refs, and Overwatch evidence refs.
