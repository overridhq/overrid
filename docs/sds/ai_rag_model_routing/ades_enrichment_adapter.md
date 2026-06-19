SDS #58

# ADES Enrichment Adapter SDS

## Purpose

Use ADES as optional local semantic enrichment and domain-pack tagging for entity, topic, warning, and routing hints.

ADES Enrichment Adapter connects Overrid AI flows to ADES without making ADES a required protocol dependency. ADES is a local-first semantic enrichment tool that installs small domain libraries, tags text or files locally, and can also run as a local HTTP service with a health endpoint. The adapter wraps that capability with Overrid identity, privacy, metering, degradation, audit, and routing-hint contracts.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [ades_enrichment_adapter.md](../../service_catalog/ai_rag_model_routing/ades_enrichment_adapter.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |
| ADES README | [README.md](https://raw.githubusercontent.com/bekirdag/ades/refs/heads/main/README.md) |

## Service Family

- Family: AI, RAG, and model routing
- Owning layer: Optional local semantic enrichment and routing-hint adapter
- Primary data scope: enrichment requests, domain-pack refs, ADES connector health, privacy mode, enrichment results, entity/topic/warning outputs, timing metadata, routing hint bundles, degradation records, and usage refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md)

## Problem Statement

AI routing should not send every request to a large model just to discover basic entities, topics, domain hints, warnings, or simple routing clues. ADES can provide local semantic enrichment through domain packs such as general English, finance, medical, or region/domain-specific packs. That can reduce model load and improve routing quality.

The risk is making ADES too powerful. ADES hints are not policy decisions, final intent classification, safety judgments, or model selection authority. The adapter must keep private text local where required, degrade cleanly when ADES is unavailable, and pass only bounded hints to the AI Gateway Router or Personal AI Assistant.

## Goals

- Connect to ADES in local library/CLI mode or local HTTP service mode.
- Discover and select installed domain packs for enrichment.
- Produce entities, topics, warnings, timing metadata, and routing hints in a stable Overrid envelope.
- Keep private text local and prevent accidental external enrichment.
- Degrade gracefully when ADES, a domain pack, or a local service health check is unavailable.
- Feed hints to Lightweight Classifier and AI Gateway Router without replacing policy, routing, or model authority.
- Meter material enrichment work and preserve audit refs without storing raw private text.

## Non-Goals

- Do not make ADES a required Overrid protocol dependency.
- Do not treat ADES output as a final policy decision, safety decision, fraud decision, or route decision.
- Do not persist raw private text or files in the adapter.
- Do not fetch unapproved domain packs during a private request.
- Do not send private or regulated content to non-local services.
- Do not replace encrypted Docdex RAG, Lightweight Classifier, AI Gateway Router, or Central AI Service.
- Do not add pricing, financial projections, customer counts, blockchain mechanics, NFT mechanics, or per-transaction fee economics.

## Primary Actors And Clients

- Personal AI Assistant requesting local enrichment before model routing.
- AI Gateway Router requesting hints for model, tool, RAG, or resource selection.
- Lightweight Classifier consuming entities, topics, warnings, and domain hints.
- Encrypted Docdex RAG Adapter using enrichment hints to select context scopes where authorized.
- Native apps using local semantic tags for user-authorized text.
- Overguard and privacy policy checks constraining allowed enrichment modes.
- Overmeter and Wallet/Usage Center consuming usage refs where material.
- SDK, CLI, and admin UI inspecting degradation and health.

## Dependencies

- ADES local package, CLI, or local HTTP service with `/healthz`.
- Domain pack inventory from ADES, such as `general-en`, `finance-en`, `medical-en`, or future approved packs.
- [AI Gateway Router](ai_gateway_router.md) and [Lightweight Classifier](lightweight_classifier.md) for downstream routing and classification.
- [Personal AI Assistant](personal_ai_assistant.md) for user-facing request flow.
- [Encrypted Docdex RAG Adapter](encrypted_docdex_rag_adapter.md) for authorized context retrieval integration.
- [Overguard](../trust_policy_verification/overguard.md), [Overvault](../data_storage_namespace/overvault.md), and [Overwatch](../control_plane/overwatch.md) for privacy, secret refs, and audit evidence.
- [Overmeter](../execution_scheduling/overmeter.md) for usage visibility.

## Owned Responsibilities

ADES Enrichment Adapter owns:

- ADES connector configuration and local service health records.
- Domain pack inventory and approved pack metadata.
- Enrichment request envelopes and privacy mode selection.
- Local-only input handling and redaction rules.
- Enrichment result normalization into stable entities, topics, warnings, timing, and confidence fields.
- Routing hint bundle creation for downstream services.
- Degradation records when ADES, packs, or health checks are unavailable.
- Usage and audit refs for material enrichment operations.

## Data Model

- `ades_connector_config`: connector mode (`library`, `cli`, `local_http`), executable/service refs, local endpoint, health status, version, timeout, max input size, and allowed pack refs.
- `domain_pack_ref`: pack name, version, language, domain, dependency refs, approval status, local install state, and policy refs.
- `enrichment_request`: request id, actor/tenant refs, privacy mode, input ref or ephemeral text hash, requested pack refs, max output size, trace id, and idempotency key.
- `enrichment_result`: normalized entities, topics, warnings, labels, spans or redacted spans, timing fields in milliseconds, pack refs, confidence fields, and output hash.
- `routing_hint_bundle`: model-size hint, tool-need hints, RAG-scope hints, domain hints, warning hints, and confidence/explanation refs.
- `privacy_guard_record`: data class, locality requirement, redaction outcome, policy refs, denied fields, and audit refs.
- `degradation_record`: missing ADES, failed health check, missing pack, timeout, invalid output, policy denial, fallback behavior, and caller-visible reason.
- `usage_ref`: enrichment duration, pack count, input size class, output size class, and Overmeter refs.

Raw private text should be ephemeral. Persistent records store hashes, refs, normalized safe hints, and audit metadata rather than full private content.

## API Surface

- `POST /ades/enrich`: enriches ephemeral text or an authorized file/content ref with selected packs.
- `POST /ades/enrich/batch`: bounded batch enrichment for small local items.
- `GET /ades/packs`: lists installed, approved, unavailable, and deprecated domain packs.
- `POST /ades/packs/refresh`: refreshes local pack inventory without pulling unapproved packs.
- `GET /ades/health`: checks ADES package or local service health.
- `POST /ades/hints`: returns routing-hint bundles for AI Gateway Router or Lightweight Classifier.
- `GET /ades/results/{result_id}`: returns redacted enrichment output if retention is allowed.
- `GET /ades/degradations/{degradation_id}`: returns degradation reason and fallback behavior.

Mutating APIs require actor/service identity, tenant scope, trace id, idempotency key, privacy mode, data class, allowed pack refs, and policy refs. Stable errors include `ades_unavailable`, `pack_missing`, `pack_not_allowed`, `locality_required`, `input_too_large`, `private_text_persistence_denied`, `output_invalid`, `timeout`, and `policy_denied`.

## Event Surface

- `ades_enrichment_adapter.health_checked`: connector health check completed.
- `ades_enrichment_adapter.pack_inventory_updated`: local pack inventory changed.
- `ades_enrichment_adapter.enrichment_requested`: enrichment request accepted.
- `ades_enrichment_adapter.enrichment_denied`: privacy, pack, size, or policy denial.
- `ades_enrichment_adapter.enrichment_completed`: normalized result available.
- `ades_enrichment_adapter.routing_hints_emitted`: routing hint bundle emitted downstream.
- `ades_enrichment_adapter.degraded`: ADES unavailable, pack missing, timeout, or invalid output.

Events include connector mode, pack refs, privacy mode, data class, timing classes, result refs, degradation refs, policy refs, and usage refs. They must not include raw private text.

## Core Workflow

1. Caller requests enrichment with text or an authorized content ref, privacy mode, data class, and desired pack refs.
2. Adapter checks Overguard/privacy policy and rejects non-local handling for private or regulated content.
3. Adapter checks ADES health and local pack inventory.
4. Adapter invokes ADES in library, CLI, or local HTTP mode with bounded input.
5. Adapter normalizes ADES output into entities, topics, warnings, timing metadata, and confidence/explanation fields.
6. Adapter emits routing hints to AI Gateway Router or Lightweight Classifier.
7. Adapter records usage and audit metadata, then drops raw private input unless explicit retention is authorized.
8. If ADES is unavailable, adapter emits a degradation record and lets the router continue without ADES hints.

## State Machine

Request lifecycle:

1. `submitted`
2. `policy_checked`
3. `health_checked`
4. `pack_selected`
5. `running`
6. `completed`
7. `degraded`
8. `denied`
9. `failed`

Pack lifecycle:

1. `discovered`
2. `approved`
3. `installed`
4. `active`
5. `missing`
6. `deprecated`
7. `blocked`

Result lifecycle:

1. `normalized`
2. `hint_emitted`
3. `retained_metadata_only`
4. `expired`
5. `redacted`

## Policy And Security

- Private and regulated content must run only in local ADES modes approved by policy.
- Do not persist raw private text or raw file content in adapter records.
- Domain packs must be approved before use in platform flows.
- Hints are advisory and must not bypass Overguard, AI Gateway Router, or user permissions.
- Redact spans when returning them would expose private content to a downstream service that only needs labels.
- Enforce input size, timeout, and output size caps to prevent adapter abuse.
- Treat warnings as signals for downstream review, not as final safety outcomes.

## Metering And Accounting

- Emit usage for material enrichment operations: input size class, pack count, duration, connector mode, and output size class.
- Link enrichment usage to user/app/workload refs where allowed.
- Keep tiny local classification-like calls cheap and internal; accounting flows through Overmeter and ORU/Seal Ledger where relevant.
- Missing ADES or degraded calls should not be charged as successful enrichment work.
- Do not encode financial projections, per-call price schedules, or revenue logic.

## Observability And Operations

- Expose ADES health, connector mode, version, pack inventory, pack approval status, request latency, timeouts, invalid output counts, privacy denials, and degradation rate.
- Alert when private requests attempt non-local enrichment.
- Alert when a high-use pack becomes missing, deprecated, or unhealthy.
- Provide replay using input hashes, pack refs, connector version, policy refs, and normalized output refs without storing raw input.
- Provide operator diagnostics for ADES service startup, `/healthz`, pack inventory, and timeout tuning.

## Failure Modes And Recovery

- ADES missing: return degradation and allow router fallback without enrichment.
- Pack missing: degrade to approved baseline pack if policy allows; otherwise return `pack_missing`.
- Local service unhealthy: retry within timeout budget or switch to approved library/CLI mode.
- Private request attempts remote/non-local path: deny with `locality_required`.
- ADES output invalid: discard output, emit degradation, and do not pass hints downstream.
- Timeout: return partial safe metadata only if complete normalization exists; otherwise degrade.
- Pack update changes behavior: record version refs and allow route-decision replay.

## Validation Plan

- Private text is enriched only through local approved connector modes.
- Missing ADES is handled as optional degradation and does not break AI routing.
- Missing or unapproved packs return stable errors or allowed degradation.
- Hints improve routing without replacing policy decisions.
- Raw private input is not persisted in adapter records.
- ADES health checks support package, CLI, and local HTTP modes.
- Timing metadata is recorded in milliseconds.
- Route decision replay can cite ADES pack refs and normalized hints without exposing private text.

## Build Breakdown

1. Define connector config, pack ref, enrichment request/result, routing hint, privacy guard, degradation, and usage schemas.
2. Implement local health checks for package/CLI/local HTTP modes, including `/healthz` where the service is used.
3. Implement pack inventory and approval checks.
4. Implement local enrichment request flow with input limits, timeouts, privacy rules, and normalization.
5. Implement degradation behavior and routing-hint emission.
6. Integrate with AI Gateway Router, Lightweight Classifier, Personal AI Assistant, Encrypted Docdex RAG Adapter, Overguard, Overwatch, and Overmeter.
7. Add validation fixtures for private text, missing ADES, missing pack, invalid output, and timeout cases.

## Handoff And Downstream Use

ADES Enrichment Adapter hands normalized entities, topics, warnings, timing metadata, domain-pack refs, routing hints, privacy guard refs, degradation refs, and usage refs to AI Gateway Router, Lightweight Classifier, Personal AI Assistant, Encrypted Docdex RAG Adapter, Overwatch, Overmeter, SDK, CLI, and admin UI.

Downstream services must treat ADES output as advisory enrichment, not final authority.

## Open Design Questions

- The first native AI assistant build should approve `general-en` as the default baseline pack. `finance-en` and `medical-en` may be installed and approved only as local-only, opt-in advisory packs behind explicit pack policy, user/tenant permission, and Overguard data-class checks; they must escalate route and safety confidence rather than make final finance, medical, policy, or model decisions. `finance-de-en` and other locale or specialized packs should remain disabled until a locale-specific validation fixture set, policy refs, and steward approval exist.
- Private-request retention should default to metadata-only records: request/result ids, actor/tenant/app refs, privacy mode, data class, BLAKE3 input/output hashes, pack refs and versions, connector mode/version, timing in milliseconds, input/output size classes, label/topic/warning categories, confidence classes, redacted entity refs where needed, degradation/denial reason codes, policy refs, Overwatch refs, and usage refs. Raw private text, raw files, raw spans, secrets, and unredacted private entity values must be ephemeral unless the owner explicitly stores an encrypted, time-bounded artifact through the owning storage/context service.
- Return label-level hints by default. Raw spans should not be emitted to AI Gateway Router, Lightweight Classifier, logs, metrics, or public/operator events; span offsets or redacted spans may be returned only to the original local/private caller when needed for user-visible highlighting, replay, or correction, and only under the same permission and retention policy as the source text.
- Phase 12 interactive assistant enrichment should cap raw ephemeral text at 8 KiB UTF-8 per enrichment request, with `input_too_large` returned rather than silent truncation. Larger documents, repository context, workspace content, or multi-item analysis should flow through Encrypted Docdex RAG Adapter or an explicit local batch/chunking path with separate context grants, size classes, and timeout policy.
- `general-en` is the only default fallback pack when a domain-specific pack is missing, deprecated, or unhealthy, and only when policy says a generic baseline hint is still useful. Missing `finance-en`, `medical-en`, locale-specific, or regulated-domain packs must emit `pack_missing` or a degradation record, lower confidence, and let Lightweight Classifier or AI Gateway Router escalate; the adapter must not substitute a different sensitive pack or widen routing/policy authority.
