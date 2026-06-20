SDS #63

# Personal AI Assistant SDS

## Purpose

Build the user's everyday native AI surface using central AI coordination, encrypted Docdex RAG, model/resource routing, permissions, privacy, and ORU metering.

Personal AI Assistant is the user-facing native AI app. It manages sessions, permissions, context source selection, tool-call proposals, delegated native-app calls, route requests, response refs, privacy audit, and usage visibility. It uses central AI coordination and the grid's model resources where appropriate, but it is not Central AI Service, not a policy authority, not a data store, and not a single giant model endpoint.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [personal_ai_assistant.md](../../service_catalog/ai_rag_model_routing/personal_ai_assistant.md) |
| Sub-build plan | [SUB BUILD PLAN #63 - Personal AI Assistant](../../build_plan/sub_build_plan_063_personal_ai_assistant.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md), [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |

## Service Family

- Family: AI, RAG, and model routing
- Owning layer: User-facing assistant application and permissioned AI task orchestration
- Primary data scope: assistant profiles, sessions, turns, permission manifests, context-source refs, tool-call proposals, delegated calls, route refs, response refs, usage receipts, privacy audits, and user-visible controls
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), with integration groundwork in [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md)

## Problem Statement

Users need an AI assistant that can use their personal, organization, workspace, and repository context without handing their life to a corporate black box. The assistant must be efficient enough to route simple requests to small resources, powerful enough to delegate complex work to available grid models or tools, and disciplined enough to preserve privacy, permission boundaries, and user control.

The assistant also needs to become the natural interface for Overrid native apps: workspace, messaging, search, directory, wallet, maps, mobile clients, and central AI stewardship views. That cannot mean bypassing those apps. The assistant must call them through explicit tool/delegation contracts, with user consent and audit.

## Goals

- Provide the user's everyday AI interface for chat, task execution, search, summarization, writing, coding assistance, and native-app delegation.
- Use Lightweight Classifier, ADES hints, AI Gateway Router, and Encrypted Docdex RAG Adapter to select the least sufficient authorized path.
- Let users choose or approve context sources from personal, organization, workspace, repo, messaging, search, and native-app data.
- Route model work through available Overrid model resources instead of one hardcoded model/provider.
- Propose tool calls and delegated native-app actions before side effects where policy or user preference requires confirmation.
- Show usage, receipts, context access, and privacy audit in the Wallet/Usage Center.
- Support central AI coordination as bounded assistance for stewardship/reporting paths without making the personal assistant an enforcement authority.

## Non-Goals

- Do not store raw user data, workspace documents, repo content, or vault secrets as assistant-owned canonical data.
- Do not bypass app permissions, Overvault grants, encrypted Docdex RAG authorization, Overguard policy, or user confirmation.
- Do not run every request through a large model or one provider.
- Do not directly mutate ORU balances, Seal Ledger entries, grants, payouts, disputes, or sanctions.
- Do not become Central AI Service or make ecosystem governance decisions.
- Do not optimize for addiction, compulsive engagement, surveillance, ads, or hidden data extraction.
- Do not add pricing forecasts, customer counts, blockchain assumptions, NFT mechanics, or per-transaction fee economics.

## Primary Actors And Clients

- End users using the native assistant app on web, desktop, or mobile clients.
- AI Gateway Router selecting model/resource/tool routes.
- Encrypted Docdex RAG Adapter providing authorized context bundle refs.
- Lightweight Classifier and ADES Enrichment Adapter providing advisory request hints.
- Wallet and Usage Center showing usage, receipts, context access, permissions, and privacy audit.
- Native apps receiving delegated calls: workspace, messaging, search, directory, maps, social, wallet, and central AI stewardship interface.
- Overpass, Overtenant, Overkey, Overguard, Overvault, Overmeter, Overwatch, ORU Account Service, and Seal Ledger enforcing identity, permissions, privacy, usage, and accounting.
- Central AI Service receiving user-approved stewardship or evidence-analysis handoffs where applicable.

## Dependencies

- [AI Gateway Router](ai_gateway_router.md) for model/resource route decisions.
- [Encrypted Docdex RAG Adapter](encrypted_docdex_rag_adapter.md) for authorized personal/org/repo/workspace context.
- [Lightweight Classifier](lightweight_classifier.md) and [ADES Enrichment Adapter](ades_enrichment_adapter.md) for advisory classification and local enrichment.
- [Central AI Service](central_ai_service.md) for bounded stewardship/evidence workflows.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), and [Overkey](../control_plane/overkey.md) for identity, tenant, credentials, and service-account facts.
- [Overguard](../trust_policy_verification/overguard.md), [Overvault](../data_storage_namespace/overvault.md), [Overstore](../data_storage_namespace/overstore.md), and [Overbase](../data_storage_namespace/overbase.md) for policy, secret refs, object refs, and assistant state.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), and [Overwatch](../control_plane/overwatch.md) for usage, receipts, accounting refs, and audit.
- Native app SDS files for delegated actions and app-specific permission checks.

## Owned Responsibilities

Personal AI Assistant owns:

- Assistant profile settings, safe defaults, user preferences, and allowed tool/context policies.
- Assistant sessions, turns, user-visible message refs, and route request refs.
- Permission manifests for context sources, tools, native apps, retention, and confirmation rules.
- Context-source selection records and context grant requests.
- Tool-call proposals, delegated native-app call records, confirmation records, and cancellation records.
- Response refs, citation refs, route refs, model/result provenance, and user-visible explanation summaries.
- Privacy audit records showing what context was used, why, by which route, and under which permission.
- Usage receipt refs and wallet handoff records.

## Data Model

- `assistant_profile`: user refs, default privacy mode, allowed context source classes, allowed tool classes, confirmation policy, retention defaults, accessibility prefs, and app integration prefs.
- `assistant_session`: session id, user/tenant refs, client refs, session purpose, privacy mode, active context grants, route history refs, usage refs, state, and retention class.
- `assistant_turn`: session ref, user input hash/ref, normalized request metadata, classifier refs, ADES hint refs, context plan refs, route request refs, response ref, user feedback refs, and audit refs.
- `permission_manifest`: context-source permissions, native-app tool permissions, vault grant refs, delegated action policy, retention policy, expiry, revocation status, and confirmation requirements.
- `context_source_selection`: selected Docdex indexes, workspace/doc refs, message/search refs, app data refs, denied refs, redaction plan, and purpose refs.
- `tool_call_proposal`: target app/service, operation, parameters or parameter refs, side-effect class, confirmation requirement, policy refs, and rollback/cancel behavior.
- `delegated_call_record`: proposal ref, confirmed actor refs, target service, command refs, result refs, failure reason, and usage refs.
- `assistant_response_ref`: output ref, citations, route decision refs, model/run provenance, safety/quality flags, redaction profile, and user-visible explanation refs.
- `privacy_audit_record`: context accessed, permissions used, grants created/revoked, route refs, tool calls, retention decisions, and Overwatch refs.
- `assistant_usage_receipt`: route usage, context retrieval usage, tool usage, storage/bandwidth usage, receipt refs, and wallet display refs.

## API Surface

- `POST /personal-ai/sessions`: creates an assistant session with privacy mode and permission manifest refs.
- `GET /personal-ai/sessions/{session_id}`: returns session state, visible turns, permissions, and usage summaries.
- `POST /personal-ai/sessions/{session_id}/turns`: submits a user turn and starts classification, context planning, routing, and response generation.
- `GET /personal-ai/turns/{turn_id}`: returns turn state, response refs, citations, route refs, and privacy audit summary.
- `POST /personal-ai/context-sources/preview`: previews available context sources and denied refs before retrieval.
- `POST /personal-ai/permissions`: creates or updates a permission manifest.
- `POST /personal-ai/permissions/{permission_id}/revoke`: revokes future context or tool access.
- `POST /personal-ai/tool-proposals/{proposal_id}/confirm`: confirms a side-effecting tool or native-app action.
- `POST /personal-ai/tool-proposals/{proposal_id}/reject`: rejects a proposed tool/delegated action.
- `GET /personal-ai/privacy-audit/{session_or_turn_id}`: returns user-visible context, route, and tool-use audit.
- `GET /personal-ai/usage/{session_or_turn_id}`: returns usage and receipt refs.

Mutating APIs require user/service identity, tenant scope, trace id, idempotency key, privacy mode, permission refs, and policy refs. Stable errors include `permission_required`, `context_not_authorized`, `tool_confirmation_required`, `native_app_permission_denied`, `route_unavailable`, `privacy_mode_unsatisfied`, `usage_limit_exceeded`, `response_blocked`, `grant_revoked`, and `policy_denied`.

## Event Surface

- `personal_ai_assistant.session_created`: session created with privacy mode and permission refs.
- `personal_ai_assistant.turn_submitted`: user turn accepted.
- `personal_ai_assistant.classification_attached`: classifier and ADES hint refs attached.
- `personal_ai_assistant.context_requested`: context-source preview or retrieval requested.
- `personal_ai_assistant.route_requested`: AI Gateway route requested.
- `personal_ai_assistant.tool_proposed`: tool or native-app action proposed.
- `personal_ai_assistant.tool_confirmed`: user or policy confirmation recorded.
- `personal_ai_assistant.tool_executed`: delegated call completed or failed.
- `personal_ai_assistant.response_created`: response refs and citations produced.
- `personal_ai_assistant.permission_revoked`: permission or context grant revoked.
- `personal_ai_assistant.usage_emitted`: usage and receipt refs emitted.

Events include user/tenant refs, session/turn refs, privacy mode, permission refs, route refs, context bundle refs, tool refs, result class, and usage refs. They must not include raw private prompts, decrypted RAG content, vault secrets, or private app data outside the user's allowed audit scope.

## Core Workflow

1. User starts or resumes an assistant session with a privacy mode and permission manifest.
2. User submits a turn. Assistant validates identity, tenant, idempotency, privacy mode, and current permissions.
3. Lightweight Classifier and optional ADES enrichment produce advisory intent, sensitivity, RAG, tool, and model-size hints.
4. Assistant previews/selects context sources and requests authorized context from Encrypted Docdex RAG Adapter.
5. AI Gateway Router selects model/resource/tool route using context plan, classifier facts, policy, availability, and budget.
6. If a side-effecting tool or native-app action is needed, assistant creates a proposal and waits for confirmation when required.
7. Assistant executes the allowed route/delegated call, captures response refs, citations, tool results, usage refs, and privacy audit.
8. Wallet/Usage Center receives usage and receipt refs; users can inspect and revoke future permissions.

## State Machine

Session lifecycle:

1. `created`
2. `active`
3. `waiting_for_permission`
4. `waiting_for_user_confirmation`
5. `paused`
6. `closed`
7. `expired`
8. `deleted`

Turn lifecycle:

1. `submitted`
2. `classified`
3. `context_planned`
4. `route_selected`
5. `waiting_for_confirmation`
6. `running`
7. `response_ready`
8. `completed`
9. `denied`
10. `failed`
11. `cancelled`

Tool/delegation lifecycle:

1. `proposed`
2. `policy_checked`
3. `confirmation_required`
4. `confirmed`
5. `executing`
6. `succeeded`
7. `rejected`
8. `failed`
9. `cancelled`

## Policy And Security

- The assistant must be permission-first: context and tool access require explicit manifests, owner scope, purpose refs, and policy checks.
- Private context must flow through Encrypted Docdex RAG Adapter, native app APIs, Overvault grants, or other owning services; the assistant cannot read private stores directly.
- Side-effecting actions require confirmation unless the user has granted a narrow durable permission and policy allows it.
- Permission revocation blocks future access and forces reauthorization for active sessions where needed.
- Route fallback cannot widen context or tool permissions.
- Assistant logs and metrics must use hashes, refs, labels, and redacted summaries instead of raw private prompts by default.
- Central AI handoff requires explicit evidence scope and cannot silently become enforcement.
- Product behavior must avoid addictive loops, hidden profiling, dark patterns, or ad-ranking incentives.

## Metering And Accounting

- Emit usage refs for classification, enrichment, context retrieval, model routes, tool calls, storage, bandwidth, and response generation.
- Link usage to session, turn, user, tenant, app, route id, context bundle id, delegated call id, and wallet receipt refs.
- Wallet/Usage Center displays usage and receipts; the assistant does not maintain balances or ledger truth.
- Failed, denied, cancelled, and confirmation-rejected operations should be visible with reason codes for user trust and dispute support.
- Do not encode hardcoded prices, revenue projections, or provider payout rules.

## Observability And Operations

- Expose session volume, turn latency, classification escalation rate, context-denial rate, route failures, tool proposal/confirmation rates, permission revocations, and usage by resource class.
- Alert on unexpected private-context access, tool-call loops, route fallback loops, high denial rates, missing wallet receipts, and policy-sensitive actions without confirmation.
- Provide user-visible privacy audit and operator-safe diagnostics that do not leak private prompts or context.
- Provide replay for a turn: permissions, classifier refs, context refs, route decision, tool confirmations, response refs, and usage refs.

## Failure Modes And Recovery

- Permission missing: ask for narrow permission or continue without that context/tool if useful.
- Context authorization denied: answer without that context or explain the missing permission.
- Route unavailable: retry allowed fallback through AI Gateway Router without widening privacy constraints.
- Tool confirmation rejected: cancel the delegated action and continue the session.
- Native app call fails: preserve proposal, result, reason code, and retry/correction path.
- Usage receipt emission fails: complete user response only after durable reconciliation refs exist or mark receipt pending.
- Revocation during turn: stop future context/tool access and cancel active work if policy requires.
- Model output invalid or unsafe: block response, route to repair/retry where allowed, and preserve audit refs.

## Validation Plan

- Assistant uses authorized RAG context only and never bypasses context grants.
- Simple low-risk tasks route to smaller resources when classifier/router policy allows.
- Sensitive, low-confidence, private, destructive, or side-effecting tasks escalate or require confirmation.
- Tool calls to native apps use app APIs and permission manifests, not direct storage access.
- Usage is metered and visible through wallet/receipt refs.
- Privacy audit shows context sources, route refs, tool calls, and retention decisions.
- Permission revocation blocks future context/tool use.
- Central AI handoff is evidence-scoped and does not mutate governance/accounting state directly.

## Build Breakdown

1. Define assistant profile, session, turn, permission manifest, context source, tool proposal, delegated call, response, privacy audit, and usage receipt schemas.
2. Build user session/turn APIs and privacy/permission defaults.
3. Integrate Lightweight Classifier, ADES hints, AI Gateway Router, and Encrypted Docdex RAG Adapter.
4. Implement context-source preview, grant creation, grant revocation, and user-visible privacy audit.
5. Implement tool/delegation proposal and confirmation flows for first native apps.
6. Integrate Wallet/Usage Center, Overmeter, ORU/Seal Ledger receipt refs, Overwatch replay, SDK/mobile client surfaces.
7. Harden privacy, permission revocation, fallback, and unsafe-output repair before broad native app launch.

## Handoff And Downstream Use

Personal AI Assistant hands route requests, context-source selections, permission manifests, tool proposals, delegated call refs, response refs, privacy audit refs, and usage receipt refs to AI Gateway Router, Encrypted Docdex RAG Adapter, Lightweight Classifier, ADES Enrichment Adapter, Central AI Service, native apps, Wallet/Usage Center, Overwatch, SDK, CLI, mobile, and admin UI.

Native apps remain the owners of their data and operations. The assistant delegates; it does not replace their permission or state boundaries.

## Open Design Questions

Resolved decisions:

- The first side-effecting delegated native-app action should be Wallet and Usage Center permission control, starting with a confirmed request to narrow or revoke an assistant, AI context, tool, or native-app permission through Wallet's owning-service handoff path. This aligns with Phase 12's build order, exercises the user's control surface before broader app automation, and avoids pretending the assistant can mutate balances, ledger entries, grants, payouts, disputes, or resource rates. The proposal must show the permission ref, owning service, scope being narrowed or revoked, expected effect on active sessions, rollback or reauthorization path, policy refs, usage refs, and Overwatch audit refs before confirmation.
- Context-source permission defaults to route/request-bound per-turn grants. Session-scoped grants are allowed only when the user or organization explicitly creates a narrow, time-bounded reusable grant for a specific owner scope, purpose, app/session, data class, leakage profile, result and snippet caps, retention class, and allowed tool or native-service use. Any request that widens context, adds a new repo/workspace/person/org source, crosses tenant or organization boundaries, changes leakage/retention/cap limits, touches secret-bearing, regulated, fraud-sensitive, or stewardship data, or invokes a side-effecting tool must require per-turn confirmation or fresh authorization.
- User-visible privacy audit should show the session and turn refs, assistant/app/client refs, purpose and privacy mode, context source aliases and owner scopes, data-class and leakage profile, permission/grant refs with expiry and revocation status, classifier/ADES hint categories, context bundle/citation/snippet-hash refs, selected route/resource class, rejected or degraded reason codes, tool proposal/confirmation/execution refs, delegated native-app target and operation class, usage and wallet receipt refs, retention/expiry state, and Overwatch audit refs. It must not expose raw private prompts, decrypted RAG snippets outside an authorized bundle, vault/key material, unauthorized path or workspace structure, other users' queries or records, hidden fraud/policy thresholds, model/provider secrets, or operator-only diagnostics.
- Mobile offline mode may queue assistant turns only as signed, idempotent, expiring intent envelopes with request hashes, privacy mode, permission snapshot refs, and payload refs or hashes. On reconnect, Mobile Backend Gateway and the assistant must revalidate device/session state, actor and tenant authority, permission grants, context capability snapshots, policy refs, and route constraints before classification, retrieval, model routing, or tool execution. If a relevant permission, grant, device, session, or context source was revoked or expired while offline, the queued turn moves to `waiting_for_permission`, `denied`, `expired`, or `cancelled` with `grant_revoked` or equivalent reason codes; cached context and side-effecting actions are discarded until the user reauthorizes.
- Before user or human confirmation, unsafe-output repair is limited to a bounded, non-side-effecting repair inside the same or stricter privacy, context, tenant, role, locality, budget, model/resource, and tool-permission envelope: schema/format repair, citation/ref alignment, redaction cleanup, refusal or safety text normalization, or one route-approved retry that uses the same authorized context and no new tools. The assistant must block the unsafe output from display or execution while preserving audit refs. Any repair that would widen context, change permissions, use a larger/less-local/higher-cost resource class, invoke Central AI or a native app, affect identity, credentials, wallet/accounting/disputes, regulated or safety-sensitive guidance, publication, messaging, workspace edits, or other side effects requires explicit confirmation or owning-service review first.
