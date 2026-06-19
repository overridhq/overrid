SDS #1

# Admin and Developer UI SDS

## Purpose

Provide a utilitarian operational interface for tenants, identities, nodes, jobs, queue state, policy decisions, verification evidence, usage, ORU/Seal Ledger views, disputes, receipts, and system health.

This SDS defines the Admin and Developer UI as an Overrid client surface, not a privileged backdoor. The UI must read and mutate platform state only through Overgate-admin APIs, signed operator commands, shared schemas, and Overwatch-backed audit evidence.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [admin_developer_ui.md](../../service_catalog/foundation/admin_developer_ui.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md) |
| SDS sub-build plan | [SUB BUILD PLAN #1 - Admin and Developer UI](../../build_plan/sub_build_plan_001_admin_developer_ui.md) |

## Service Family

- Family: Foundation and developer tooling.
- Owning layer: Operator and developer experience.
- Primary data scope: UI view models, saved filters, operator action envelopes, audit receipt references, status panels, and local UI diagnostics.
- First build phase from service plan: [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).
- Upstream phase dependencies: Phase 1 identity/tenant/key/audit primitives, Phase 3 job execution state, Phase 4 policy/verification/dispute evidence, and Phase 5 usage/accounting views.

## Problem Statement

By Phase 6, Overrid must support real product workloads from Docdex, Mcoda, Codali, and the developer toolchain. Operators need a single place to trace a job from signed request to queued work, lease, node execution, policy decisions, usage rollups, receipt records, and disputes. Without this UI, support and debugging would fall back to ad hoc database queries and hidden operator scripts, which would break Overrid's auditability and make early product integration unsafe.

## Goals

- Build read-only operational visibility before any mutating admin action.
- Show a complete job timeline from Overgate request through Overqueue, Oversched, Overlease, Overcell/Overrun, Overmeter, Seal Ledger, and Overwatch.
- Expose policy denials with stable reason codes, input facts, policy versions, and appeal/correction paths where applicable.
- Show tenant, identity, key, node, package, workload, usage, receipt, hold, and dispute records through schema-checked view models.
- Route every mutating admin action through signed Overgate commands with idempotency keys and Overwatch audit evidence.
- Keep the first UI quiet and operational: dense tables, timeline views, drilldowns, filters, and diagnostics, not marketing pages.

## Non-Goals

- Do not give the UI direct database, queue, object-store, ledger, or node-agent access.
- Do not create an unaudited "super admin" path.
- Do not make the UI the source of truth for policy, identity, billing, disputes, or workload state.
- Do not expose private payloads, encrypted RAG inputs, secrets, raw credentials, or unnecessary personal data.
- Do not add public native-app features here; this is an operator/developer surface.
- Do not add pricing, customer-count, revenue, or market-volume assumptions.

## Primary Actors And Clients

- Founder/operator running seed hardware and early product integrations.
- Service implementers debugging control-plane, execution, policy, accounting, and adapter behavior.
- Product integrators for Docdex, Mcoda, Codali, and later native apps.
- Security and incident responders reviewing audit, policy, and dispute evidence.
- Central AI stewardship workflows that need bounded read access to operational evidence in later phases.

## Dependencies

- Overgate admin APIs for every read and write.
- Overpass, Overtenant, and Overkey for actor, tenant, role, service-account, and key context.
- Overwatch for append-only audit and event timelines.
- Overqueue, Oversched, Overlease, Overcell, Overrun, and Overpack for job state.
- Overguard, Oververify, Challenge Task Service, and Overclaim for policy, verification, and dispute evidence.
- Overmeter, ORU Account Service, Seal Ledger, Overbill, Overgrant, and Overasset for usage/accounting/rights views.
- Shared schema package and SDK for typed API clients and view-model validation.

Dependencies must be consumed through explicit API contracts. During early builds, missing later-phase dependencies should appear as disabled panels with fixture-backed contract tests, not as direct storage reads or hardcoded mock behavior in production code.

## Owned Responsibilities

Admin and Developer UI is responsible for:

- Rendering operator/developer views from schema-checked API responses.
- Maintaining local UI state such as selected tenant, filters, table layouts, expanded timeline nodes, and saved dashboard presets.
- Building signed admin action requests and passing them to Overgate.
- Showing action outcomes, audit references, denial reason codes, and retry/correction paths.
- Redacting sensitive fields before display according to data class, role, and tenant context.
- Providing actionable diagnostics when an integration fails without requiring private storage access.
- Keeping frontend telemetry separate from platform audit events.

The UI does not own platform business state. Source-of-truth state remains in the underlying Overrid services.

## Data Model

The first implementation should define these records and schemas:

- `admin_session_context`: local view context containing active tenant, actor id, role bindings, selected environment, and redaction profile.
- `operator_view_preset`: saved filters, table columns, sort order, and dashboard layout scoped to actor and tenant.
- `resource_summary_view`: normalized summary for tenants, identities, nodes, keys, manifests, packages, workloads, queue items, leases, usage, receipts, disputes, and policy decisions.
- `job_timeline_view`: ordered events and state transitions keyed by `trace_id`, command id, queue item id, lease id, node id, workload id, usage rollup id, and receipt id.
- `policy_decision_view`: policy version, matched rules, input fact references, reason codes, denial class, and appeal/correction options.
- `admin_action_request`: signed command envelope for bounded mutations such as cancel job, retry job, pause node, annotate dispute, rotate key, or acknowledge incident.
- `admin_action_receipt`: Overwatch event refs, command outcome, affected records, and follow-up links for each admin action.
- `ui_diagnostic_event`: local-only browser/app diagnostic with no private payload content.

All records that cross service boundaries must use the common Overrid envelope where applicable: `id`, `tenant_id`, `actor_id`, `trace_id`, `idempotency_key`, `state`, `schema_version`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

The UI must use typed SDK clients generated from shared schemas. It should call Overgate routes conceptually equivalent to:

- `GET /admin/tenants`: list tenant summaries visible to the actor.
- `GET /admin/identities`: list people, organizations, nodes, apps, service accounts, native services, and system-service identities visible to the actor.
- `GET /admin/keys`: list key metadata, rotation state, revocation state, and last-used data without exposing private key material.
- `GET /admin/nodes`: list node capability, trust class, region, health, leases, and verification state.
- `GET /admin/workloads`: list workload requests, manifests, queue state, execution state, retry state, and terminal outcome.
- `GET /admin/workloads/{id}/timeline`: assemble trace-linked Overwatch, queue, lease, execution, policy, usage, receipt, and dispute events.
- `GET /admin/policy-decisions`: inspect policy decisions with reason codes and input fact references.
- `GET /admin/usage`: inspect Overmeter rollups by tenant, actor, workload, app, provider, and time window.
- `GET /admin/ledger`: inspect ORU/Seal Ledger balances, holds, receipts, corrections, and disputes through read-only views.
- `GET /admin/disputes`: inspect Overclaim cases and linked evidence.
- `POST /admin/actions`: submit a signed `admin_action_request` through Overgate.

API requirements:

- Read APIs must enforce tenant, role, data-class, and redaction filtering server-side.
- Mutating APIs must require signed command envelope, idempotency key, actor id, tenant id, reason, target, expected current state, and trace id.
- Batch reads must be bounded by server-controlled limits and cursor pagination.
- UI actions must be safe to retry through idempotency.
- Error responses must include stable reason codes, trace id, and audit refs where available.

## Event Surface

The UI must not write directly to Overwatch. Platform events are emitted by Overgate and downstream services in response to UI-originated commands.

Relevant event families:

- `admin_ui.view_opened`: optional local diagnostic only; never a platform audit source.
- `admin_action.requested`: emitted by Overgate after receiving a signed admin action request.
- `admin_action.accepted`: emitted after actor, tenant, role, idempotency, and policy checks pass.
- `admin_action.denied`: emitted when the action fails authorization, policy, schema, trust, quota, or state-precondition checks.
- `admin_action.state_changed`: emitted by the target service when a valid action changes state.
- `admin_action.completed`: emitted when the action reaches a terminal outcome.

Event payloads must reference private records rather than embedding sensitive content. UI-local diagnostics should be disabled or redacted in production builds unless explicitly enabled by operator policy.

## Core Workflow

1. Actor opens the UI and selects an environment and tenant context.
2. UI fetches actor role bindings, visible tenants, and redaction rules through Overgate.
3. UI renders read-only summary tables for tenants, identities, keys, nodes, workloads, policy decisions, usage, ledger views, and disputes.
4. Operator opens a workload timeline and follows `trace_id` from request to queue, lease, execution, policy, usage, receipt, and dispute evidence.
5. Operator optionally submits a bounded admin action with reason, expected current state, idempotency key, and signature.
6. Overgate validates, audits, and forwards the action to the owning service.
7. UI displays the action receipt and refreshes affected timelines through read APIs.

Read-only visibility must be useful before step 5 exists.

## State Machine

The UI has two distinct lifecycles.

View lifecycle:

1. `uninitialized`: UI has no environment or actor context.
2. `context_loaded`: actor, tenant, role, and redaction profile are loaded.
3. `querying`: one or more read models are loading.
4. `rendered`: current view model is valid and visible.
5. `stale`: the view model is older than the configured refresh window or affected by an action.
6. `failed`: the view cannot render because a query failed with a visible reason code.

Admin action lifecycle:

1. `draft`: operator selects target, action, reason, and expected current state.
2. `signed`: command envelope is signed locally or by an approved signing provider.
3. `submitted`: command is sent to Overgate with an idempotency key.
4. `accepted`: Overgate accepted the action and emitted an audit event.
5. `denied`: Overgate or policy denied the action before side effects.
6. `applied`: owning service changed state and emitted evidence.
7. `completed`: final receipt is visible to the operator.
8. `failed`: action failed with reason code, retry class, and audit refs.

No UI state transition may silently rewrite platform history.

## Policy And Security

- Enforce least-privilege roles for every view and action.
- Do server-side redaction before data reaches the browser or desktop shell.
- Require signed admin actions for mutations; no cookie-only mutation path.
- Require explicit operator reason and expected current state for mutating actions.
- Separate local UI diagnostics from platform audit evidence.
- Do not persist secrets, API keys, private payloads, encrypted RAG inputs, or raw credentials in UI storage.
- Bind sessions to tenant context and show the active tenant prominently in every action confirmation.
- Use CSRF, CORS, content-security-policy, secure cookie, and origin checks for web deployments.
- Support hardware-key or strong signing flows once Overkey supports them.
- Treat break-glass access as a signed, time-bounded, policy-audited action with extra evidence.

## Metering And Accounting

The UI is not a revenue surface and must not encode pricing. It should still participate in usage visibility:

- Expensive admin queries should emit usage dimensions through the normal API layer when they consume significant compute, storage, or query capacity.
- UI-originated admin actions must link to the actor, tenant, trace id, command id, and target record.
- Operator activity that affects provider payouts, ledger corrections, disputes, holds, grants, or resource state must produce an audit receipt.
- Read-only dashboards should avoid creating tiny external payment events; internal usage visibility belongs in Overmeter and Seal Ledger where relevant.
- Usage views must distinguish observed resource usage from billed/settled accounting state.

## Observability And Operations

The UI should expose:

- Environment status and Overgate reachability.
- Dependency status for identity, tenant, key, queue, execution, policy, usage, ledger, and dispute panels.
- Query latency, error rate, stale-data age, and pagination limits.
- Action submission outcomes by reason code.
- Client-side errors with sensitive fields redacted.
- Build version, schema version, API compatibility version, and feature flags.
- A "copy diagnostic bundle" action that includes trace ids and reason codes but excludes private payload content.

## Failure Modes And Recovery

- Overgate unavailable: show environment outage and preserve local draft actions without submitting them.
- Actor lacks permission: hide unavailable actions and show denial reason on attempted access.
- Stale view model: refresh before mutation and require expected-current-state checks.
- Duplicate action submit: reuse idempotency key and display original outcome.
- Partial downstream failure: show owning service, retry class, audit refs, and safe next action.
- Policy denial: display reason code, policy version, input fact references, and correction path.
- Redaction failure: fail closed and hide the affected field or panel.
- UI schema mismatch: disable affected panel and show expected/actual schema versions.

## Validation Plan

Service-plan validation:

- Operator can trace a job from request through execution and accounting.
- Policy denials show reason codes and input facts.
- Admin actions emit signed audit events.

Additional SDS-level validation:

- Read-only mode works with Phase 6 product workload fixtures before any admin action exists.
- UI cannot read directly from database, queue, object store, node agent, or ledger storage.
- Tenant isolation tests prove an actor cannot see another tenant's jobs, ledger views, disputes, or identities.
- Redaction tests prove secret-bearing and private payload fields are not rendered.
- Admin-action tests cover cancel, retry, node pause, dispute annotation, and key-rotation request with signed envelopes and idempotency.
- Stale-state tests prove a mutation fails when expected current state no longer matches.
- Accessibility and dense-table usability checks cover keyboard navigation, focus order, loading states, and long reason-code text.
- Product-reliability cases from Phase 6 can be inspected end to end.

## Build Breakdown

1. Define admin view-model schemas in the shared schema package.
2. Build read-only shell with environment, actor, tenant, role, and redaction context.
3. Add tenant, identity, key, node, workload, and queue summary tables.
4. Add workload timeline view across Overwatch, queue, lease, execution, policy, usage, ledger, and dispute refs.
5. Add policy decision and reason-code explorer.
6. Add usage, ORU, receipt, hold, correction, and dispute views.
7. Add signed admin action drafts and confirmation UX.
8. Route first bounded admin actions through Overgate with Overwatch receipts.
9. Add diagnostics bundle, schema-version checks, and panel-level dependency health.

## Handoff And Downstream Use

This UI becomes the main operational surface for [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md), especially Docdex, Mcoda, Codali, SDK, and CLI integrations.

Later native apps and governance tooling should reuse the same read-model and signed-action discipline. If the UI needs a new capability, update the owning service SDS/API first, then add the UI panel.

## Open Design Questions

Resolved decisions:

- Phase 6 UI admin actions should be limited to contract-backed actions needed for first product operations: cancel workload, retry workload from a retryable state, pause or drain a private-swarm node, annotate a dispute or incident, request credential rotation, and acknowledge an action receipt. These actions must use signed `admin_action_request` envelopes through Overgate with idempotency keys, explicit reasons, expected current state, trace ids, policy checks where applicable, and Overwatch receipts. Backbone maintenance, forced rollback, break-glass route activation, ledger correction execution, provider payout overrides, direct data repair, raw secret/key recovery, and any action without a stable owning-service command contract remain CLI/runbook-only until Phase 7 or later hardening.
- Real-time UI should mean an Overgate-filtered event stream built from authorized Overwatch and service read models, not direct browser access to service storage or event stores. Workload timelines, action receipts, queue state, node health, dependency health, and incident status may use subscriptions or follow-mode streaming when the backend contract exists. Tenant, identity, key, manifest, policy-decision history, usage, ORU/Seal Ledger, receipt, dispute, and verification-evidence panels should remain paginated polling views with visible stale-data age, cursor limits, and manual refresh.
- The first encrypted Docdex/RAG redaction policy is deny-by-default for raw private context. The UI may show tenant-authorized metadata such as job class, encrypted index ref, owner-scope class, capability/degraded status, leakage profile, context grant state, result-count class, context bundle ref, usage class, trace id, reason code, and replay/audit refs. It must not render raw prompts, decrypted snippets, file paths, query text, secret/key material, private payloads, raw RAG inputs, or unredacted result contents unless the actor already has the underlying context grant and the owning RAG service returns a deliberately redacted view.
- Before full governance roles exist, the UI should support a small tenant-scoped role set: `platform_owner`, `tenant_owner`, `tenant_admin`, `product_integrator`, `support_viewer`, `incident_responder`, `accounting_viewer`, `service_account`, and `system_service`. These roles map onto the Overtenant distinction between owner, admin, member, service account, system service, and external viewer, but high-risk capabilities must be separate action scopes rather than implied by a broad admin label.
- Break-glass should not be a normal navigation item or reusable shortcut. In Phase 6 the UI may show break-glass readiness, policy status, and incident evidence, but execution stays disabled unless the dedicated signed command class, Overkey signing support, Overguard policy, expiry rules, and Overwatch evidence bundle exist. When enabled in later phases, break-glass must require an incident ref, target scope, explicit expiry, strong signing, expected current state, extra reason/evidence fields, no silent retry, visible active-emergency state, automatic expiry, and post-action review records.
