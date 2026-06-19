# SUB BUILD PLAN #1 - Admin and Developer UI

Attached SDS: [docs/sds/foundation/admin_developer_ui.md](../sds/foundation/admin_developer_ui.md)

## Purpose

This sub-build plan turns SDS #1 into an implementation sequence for the Admin and Developer UI. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

The UI is an Overrid client surface. It must use Overgate admin APIs, signed command envelopes, shared schemas, generated SDK bindings, and Overwatch evidence. It must never become a privileged backdoor into databases, queues, object stores, node agents, ledgers, vaults, or private RAG data.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #1: Admin and Developer UI](../sds/foundation/admin_developer_ui.md) | Controls detailed UI scope, actors, records, API surface, security, validation, and open-question decisions. |
| [Admin and Developer UI service plan](../service_catalog/foundation/admin_developer_ui.md) | Controls service-catalog objective, first build phase, dependencies, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Controls the first implementation point for the UI. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps Admin and Developer UI aligned to master Phase 6. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and TypeScript only for the operator/developer UI surface and generated web bindings. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 0 and Phase 6 | Establish UI-specific scope, links, and phase gates before implementation. |
| 2 | Master Phase 0 and Phase 1 | Define shared schemas and generated bindings before UI code depends on them. |
| 3 | Master Phase 1 | Create Overgate-admin read contracts and server-side authorization boundaries. |
| 4 | Master Phase 6 | Build the first TypeScript operator shell on top of generated clients. |
| 5 | Master Phase 6 with dependencies on Phases 1, 2, and 3 | Expose read-only operational summaries for tenants, identities, nodes, packages, workloads, and queues. |
| 6 | Master Phase 6 with dependencies on Phases 3, 4, and 5 | Add trace-linked workload timelines across execution, policy, usage, receipts, and disputes. |
| 7 | Master Phase 6 with dependency on Phase 4 | Add policy, verification, dispute, and incident evidence views. |
| 8 | Master Phase 6 with dependency on Phase 5 | Add usage, ORU, Seal Ledger, Overbill, Overgrant, Overasset, and receipt views. |
| 9 | Master Phase 6, with later hardening in Phases 7 and 13 | Add bounded signed admin actions only after owning-service command contracts exist. |
| 10 | Master Phase 6, with handoff to Phases 7 and 13 | Validate product reliability, diagnostics, security posture, accessibility, and handoff. |

## Tech Stack Guardrails

- Core admin APIs, authorization filters, audit emission, policy checks, and read-model assembly belong in Rust services behind Overgate.
- The UI surface may be TypeScript, using generated bindings from canonical Rust-owned contracts.
- The UI must not invent its own platform state model; it renders schema-checked view models returned by Overgate-admin APIs.
- Local development stubs may be used only when they preserve final Overrid contract shapes.
- No direct PostgreSQL, Redis, S3, Vault, node-agent, Seal Ledger, or Overwatch storage access may appear in UI code.

## Phase 1: SDS Alignment, Scope Freeze, And Implementation Gates

### Work Items

- **1.1 Attach the build plan to SDS #1.**
  - Design: Keep this document linked from the SDS, service catalog entry, master build plan, and crosswalk so builders can move from numbered SDS to implementation work without guessing the phase.
  - Output: Stable links between this file, `docs/sds/foundation/admin_developer_ui.md`, and `docs/service_catalog/foundation/admin_developer_ui.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #1 returns both the SDS and this sub-build plan.

- **1.2 Freeze client-surface boundaries.**
  - Design: Record that the UI is a TypeScript client surface using Overgate-admin APIs and generated bindings, while server-side filters, policy decisions, command acceptance, and audit emission remain Rust service responsibilities.
  - Output: Implementation guardrail documented in this plan and enforced in later work items.
  - Validation: Architecture review confirms no UI work item requires direct service storage or node-agent access.

- **1.3 Define phase entry prerequisites.**
  - Design: Treat master Phase 6 as the first UI build point, but require Phase 0 schemas, Phase 1 identity/tenant/key/audit APIs, Phase 3 workload state, Phase 4 policy/dispute evidence, and Phase 5 accounting views for complete panels.
  - Output: A prerequisite matrix for every UI panel and action.
  - Validation: Each later work item names the upstream service contract it depends on.

- **1.4 Establish read-only-first delivery.**
  - Design: Deliver operational visibility before any mutation, and keep action workflows disabled until signed action contracts and owning-service commands exist.
  - Output: Feature flags or capability checks that expose panels independently from actions.
  - Validation: The UI can run in read-only mode against Phase 6 product fixtures with all mutation controls disabled.

- **1.5 Define documentation-update rules.**
  - Design: When a UI capability needs a new backend contract, update the owning service SDS/API first, then update this sub-build plan and only then implement the UI panel.
  - Output: Cross-document maintenance rule in this plan.
  - Validation: Review checklist rejects UI-only changes that invent backend state.

## Phase 2: Shared Admin Schemas And Generated Client Contracts

### Work Items

- **2.1 Define admin session and redaction schemas.**
  - Design: Add `admin_session_context` with active tenant, actor id, role bindings, selected environment, schema version, redaction profile, and visible capability flags.
  - Output: Canonical schema plus valid and invalid fixtures.
  - Validation: Schema tests prove missing tenant, actor, role, or redaction fields fail closed.

- **2.2 Define reusable summary view models.**
  - Design: Add `resource_summary_view` variants for tenants, identities, keys, nodes, packages, workloads, queue items, leases, usage, receipts, disputes, and policy decisions.
  - Output: Shared schema package entries and generated TypeScript bindings.
  - Validation: Fixture validation covers pagination, stale-data age, redaction markers, reason codes, trace ids, and audit refs.

- **2.3 Define job timeline view contracts.**
  - Design: Add `job_timeline_view` as an ordered, trace-linked read model across Overgate request, Overqueue item, Oversched placement, Overlease reservation, Overcell/Overrun execution, Overguard decision, Overmeter rollup, Seal Ledger receipt, and Overclaim dispute refs.
  - Output: Timeline schema with typed node kinds and edge/ref fields.
  - Validation: Fixtures prove a successful job, denied job, cancelled job, timed-out job, and disputed job can all render without private payloads.

- **2.4 Define admin action request and receipt schemas.**
  - Design: Add `admin_action_request` and `admin_action_receipt` with command id, tenant id, actor id, target, action type, reason, expected current state, idempotency key, trace id, signature refs, policy refs, and audit refs.
  - Output: Contract for later bounded actions without implementing actions early.
  - Validation: Schema tests reject unsigned requests, missing reasons, missing expected state, and unsupported action types.

- **2.5 Define local UI diagnostic event schema.**
  - Design: Keep `ui_diagnostic_event` local-only and separate from platform audit events, with trace ids and reason codes but no private payloads.
  - Output: Redacted diagnostic bundle schema.
  - Validation: Test fixtures prove secrets, raw credentials, decrypted RAG snippets, prompts, and private file paths are not allowed.

## Phase 3: Overgate Admin Read API Contracts

### Work Items

- **3.1 Specify admin list APIs.**
  - Design: Define read-only Overgate-admin routes for tenants, identities, keys, nodes, packages, workloads, queue items, policy decisions, usage, ledger views, receipts, and disputes.
  - Output: Contract specs for `GET /admin/*` routes with cursor pagination, bounded filters, stable error shapes, and schema versions.
  - Validation: Contract tests prove server-controlled limits, cursor behavior, tenant scoping, and stable reason codes.

- **3.2 Specify workload timeline API.**
  - Design: Define `GET /admin/workloads/{id}/timeline` as a Rust-side read model assembler that joins authorized refs by trace id without exposing private service storage.
  - Output: Timeline endpoint contract and fixtures for complete and partial timelines.
  - Validation: Timeline tests prove unavailable dependencies produce explicit panel status rather than broken or fabricated events.

- **3.3 Specify server-side authorization and redaction filters.**
  - Design: Enforce tenant, actor, role, data class, policy scope, and redaction before data reaches the TypeScript client.
  - Output: Authorization matrix for initial roles: `platform_owner`, `tenant_owner`, `tenant_admin`, `product_integrator`, `support_viewer`, `incident_responder`, `accounting_viewer`, `service_account`, and `system_service`.
  - Validation: Negative tests prove cross-tenant access and high-risk fields fail closed.

- **3.4 Specify API compatibility and feature discovery.**
  - Design: Add compatibility metadata so the UI can disable panels when the backend contract is absent, too old, or intentionally unavailable.
  - Output: Admin capabilities response naming route availability, schema versions, limits, and feature flags.
  - Validation: UI contract tests prove disabled panels display dependency status and do not fall back to direct storage reads.

- **3.5 Specify read-model audit refs.**
  - Design: Ensure admin reads return relevant Overwatch refs, but do not treat UI view openings as authoritative platform audit events.
  - Output: Audit-ref fields on every sensitive read model.
  - Validation: Contract tests prove action receipts and policy decisions include traceable Overwatch refs where available.

## Phase 4: TypeScript Operator Shell And Session Context

### Work Items

- **4.1 Build the generated-client integration layer.**
  - Design: Use generated TypeScript bindings and an Overgate client wrapper for signing-aware requests, trace propagation, retry decoding, pagination, and error reason display.
  - Output: UI client package or module consumed by all panels.
  - Validation: Unit tests prove the wrapper includes trace ids, handles cursor pagination, and never accepts raw service URLs outside Overgate.

- **4.2 Build environment and tenant context loading.**
  - Design: Load active environment, actor, visible tenants, role bindings, redaction profile, and compatibility flags before rendering operational panels.
  - Output: Session context provider and environment selector.
  - Validation: Component tests cover uninitialized, context-loaded, failed, stale, and permission-denied states.

- **4.3 Build the dense operational layout.**
  - Design: Use a quiet, utilitarian interface with navigation for Overview, Tenants, Identities, Keys, Nodes, Workloads, Policy, Usage, Ledger, Disputes, Receipts, and Diagnostics.
  - Output: Responsive shell with stable table dimensions, filters, empty states, and loading states.
  - Validation: Accessibility checks cover keyboard navigation, focus order, visible active tenant, and long reason-code text.

- **4.4 Build redaction-aware rendering primitives.**
  - Design: Create field components that display redacted, unavailable, denied, stale, and degraded values consistently.
  - Output: Reusable redaction badges, reason-code labels, audit-ref links, stale-age indicators, and copy-safe diagnostic refs.
  - Validation: Tests prove raw private fields cannot render through default table/detail components.

- **4.5 Build saved view preset local state.**
  - Design: Store `operator_view_preset` data for filters, columns, sorting, and panel layout scoped to actor and tenant, without storing secrets or private payloads.
  - Output: Local preset module with schema validation and reset behavior.
  - Validation: Tests prove tenant switches do not leak presets across unauthorized scopes.

## Phase 5: Read-Only Operational Summary Panels

### Work Items

- **5.1 Build tenant, identity, and key panels.**
  - Design: Render visible tenant state, role bindings, identity summaries, service accounts, system services, key metadata, rotation state, revocation state, and last-used metadata without exposing key material.
  - Output: Read-only tables and detail drawers backed by Overgate-admin APIs.
  - Validation: Tenant isolation tests prove actors cannot see unauthorized identities or key metadata.

- **5.2 Build node and capability panels.**
  - Design: Render node health, heartbeat age, capability records, trust class, region, current leases, verification state, benchmark refs, and drain/maintenance readiness as read-only data.
  - Output: Node inventory view with health filters and stale-state indicators.
  - Validation: Fixture tests cover live, stale, expired, draining, denied, and unverified node states.

- **5.3 Build package, workload, and queue summary panels.**
  - Design: Render workload requests, manifests, package refs, queue state, priority, retry metadata, cancellation eligibility, timeout state, and terminal outcomes.
  - Output: Workload and queue tables with trace-id search and stable filters.
  - Validation: Product fixtures cover Docdex, Mcoda, Codali, CLI, and SDK workload families.

- **5.4 Build panel dependency health.**
  - Design: Show explicit dependency state for Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, Oversched, Overlease, Overcell, and Overrun.
  - Output: Dependency health strip and per-panel degraded-state messages.
  - Validation: Tests prove absent dependencies disable only affected panels.

- **5.5 Build bounded query behavior.**
  - Design: Use server-side pagination, filter validation, query limits, refresh windows, manual refresh, and stale-age display instead of unbounded dashboard polling.
  - Output: Shared table query controller.
  - Validation: Tests prove queries respect server limits and display stale data clearly.

## Phase 6: Trace-Linked Workload Timeline

### Work Items

- **6.1 Build timeline assembly UI.**
  - Design: Render timeline nodes for request, command acceptance, queue item, scheduler decision, lease, node assignment, runner lifecycle, policy decision, usage rollup, receipt, dispute, and correction refs.
  - Output: Workload timeline detail view keyed by workload id and trace id.
  - Validation: Fixtures prove successful, failed, retryable, cancelled, timed-out, denied, and disputed paths render distinctly.

- **6.2 Build timeline gap and partial-data handling.**
  - Design: Treat missing refs as explicit gaps with owning service, retry class, stale age, and diagnostic reason, not as hidden errors.
  - Output: Timeline gap components and dependency status overlays.
  - Validation: Tests cover missing Overmeter, missing receipt, delayed Overwatch event, and unavailable dispute service.

- **6.3 Build policy and usage timeline overlays.**
  - Design: Overlay policy decision refs, reason codes, usage dimensions, ORU state transitions, receipt refs, and dispute holds in the same trace path.
  - Output: Toggleable timeline layers for policy, usage, accounting, and dispute evidence.
  - Validation: Timeline tests prove each overlay cites immutable refs and hides private payloads.

- **6.4 Build trace and diagnostic bundle exports.**
  - Design: Let operators copy a diagnostic bundle with trace ids, command ids, reason codes, schema versions, dependency status, and audit refs while excluding secrets and private payloads.
  - Output: Copy-safe diagnostic bundle action.
  - Validation: Redaction tests scan bundles for raw prompts, decrypted RAG snippets, secrets, key material, credentials, and private file paths.

- **6.5 Build follow-mode event updates.**
  - Design: Use Overgate-filtered event streams only where the backend contract exists; otherwise use paginated polling with visible stale age and manual refresh.
  - Output: Follow-mode timeline refresh with capability gating.
  - Validation: Tests prove no direct browser connection to Overwatch or service event stores exists.

## Phase 7: Policy, Verification, Dispute, And Incident Evidence Views

### Work Items

- **7.1 Build policy decision explorer.**
  - Design: Render policy version, matched rules, input fact refs, denied class, allowed class, reason codes, expected placement class, and correction options from Overguard and Policy Dry-Run API read models.
  - Output: Policy explorer linked from workloads and standalone filters.
  - Validation: Tests cover denied egress, insufficient trust, quota exhaustion, package trust failure, wrong tenant, and budget precheck failure.

- **7.2 Build verification evidence panels.**
  - Design: Render Oververify provider verification, challenge refs, benchmark validation, trust class changes, and stale verification indicators without exposing private challenge payloads.
  - Output: Verification evidence panel linked to nodes, providers, and workload decisions.
  - Validation: Fixture tests cover verified, degraded, challenged, expired, disputed, and untrusted states.

- **7.3 Build dispute and correction views.**
  - Design: Render Overclaim cases, evidence refs, hold state, challenge windows, correction outcomes, refunds, and settlement impacts through read-only views first.
  - Output: Dispute list and detail panel linked from timeline, usage, and receipts.
  - Validation: Tests prove disputes can hold settlement visibility without allowing direct ledger mutation.

- **7.4 Build incident and break-glass readiness views.**
  - Design: Show incident refs, readiness status, policy state, dependency health, and disabled break-glass execution until dedicated signed command classes exist in later hardening.
  - Output: Incident readiness panel with action-disabled states.
  - Validation: Tests prove break-glass controls cannot execute in Phase 6 without required Overkey, Overguard, expiry, and Overwatch evidence contracts.

- **7.5 Build evidence-link consistency checks.**
  - Design: Ensure every evidence panel links to trace ids, audit refs, policy refs, or owning-service refs rather than embedding mutable or private data.
  - Output: Shared evidence-link renderer and validation helper.
  - Validation: Component tests reject evidence cards without stable refs.

## Phase 8: Usage, ORU, Seal Ledger, Receipt, Grant, And Rights Views

### Work Items

- **8.1 Build usage rollup panels.**
  - Design: Render Overmeter rollups by tenant, actor, workload, app, provider, resource class, time window, and trace id while distinguishing observed usage from settled accounting.
  - Output: Usage dashboards with filters, drilldowns, and timeline links.
  - Validation: Fixtures cover CPU-ORU, GPU-ORU, STOR-ORU, NET-ORU, MEM-ORU, and DATA-ORU dimensions.

- **8.2 Build ORU and Seal Ledger read views.**
  - Design: Render available, reserved, held, spent, earned, sponsored, refunded/corrected, expired/revoked, and disputed states from authorized ledger read models.
  - Output: Account and workload ledger views with immutable ledger refs.
  - Validation: Tests prove balances are derived from read models and cannot be edited by the UI.

- **8.3 Build Overbill receipt and invoice views.**
  - Design: Render receipt refs, invoice status, payment-provider refs, refund refs, correction refs, and payout-hold refs without encoding pricing assumptions in the UI.
  - Output: Receipt/invoice panels linked to workload and ledger views.
  - Validation: Revenue-assumption scans find no customer-count, pricing, or market-volume projections.

- **8.4 Build Overgrant and Overasset visibility.**
  - Design: Show grant scope, sponsored resource allocation, purpose scope, resource rights refs, storage/namespace/route bindings, entitlement refs, and expiration/correction status as read-only evidence.
  - Output: Grant and rights panels linked to usage and namespace/storage refs where available.
  - Validation: Tests prove the UI does not represent Overasset as blockchain or NFT ownership.

- **8.5 Build accounting redaction and access controls.**
  - Design: Apply accounting-specific roles so `accounting_viewer` can inspect authorized ledger/receipt data without receiving unrelated tenant, private payload, or secret-bearing records.
  - Output: Accounting authorization matrix and UI permission states.
  - Validation: Cross-role tests cover platform owner, tenant owner, tenant admin, support viewer, product integrator, incident responder, and accounting viewer access.

## Phase 9: Signed Admin Actions And Receipts

### Work Items

- **9.1 Implement action drafting and confirmation.**
  - Design: Add a draft lifecycle for bounded actions: cancel workload, retry workload from retryable state, pause or drain private-swarm node, annotate dispute or incident, request credential rotation, and acknowledge receipt.
  - Output: Action draft forms with target, reason, expected current state, trace id, and visible active tenant.
  - Validation: Component tests prove missing reason, missing expected state, unsupported action, or tenant mismatch blocks submission.

- **9.2 Implement local signing handoff.**
  - Design: Route actions through approved signing flow or signing provider support, then submit signed envelopes to Overgate with idempotency keys.
  - Output: Signed `admin_action_request` submission through generated client bindings.
  - Validation: Contract tests prove unsigned or malformed signatures are rejected by Overgate before side effects.

- **9.3 Implement action receipt rendering.**
  - Design: Display Overgate acceptance, denial, owning-service state change, terminal outcome, affected refs, reason codes, retry class, and Overwatch refs.
  - Output: Action receipt detail panel and timeline insertion behavior.
  - Validation: Tests cover accepted, denied, duplicate idempotency key, stale expected state, downstream failure, applied, completed, and failed outcomes.

- **9.4 Implement stale-state protection.**
  - Design: Require refresh and expected-current-state checks before mutation, and block silent retries for high-risk or break-glass-like actions.
  - Output: Stale-state confirmation flow.
  - Validation: Tests prove a changed target state invalidates the draft and requires operator review.

- **9.5 Keep high-risk actions out of Phase 6.**
  - Design: Keep backbone maintenance, forced rollback, break-glass activation, ledger correction execution, provider payout overrides, direct data repair, and raw secret/key recovery disabled until owning Phase 7 or Phase 13 contracts exist.
  - Output: Explicit action denylist and capability gates.
  - Validation: Security review confirms disabled actions cannot be reached by route, feature flag, or direct component invocation.

## Phase 10: Product Reliability, Diagnostics, Accessibility, And Handoff

### Work Items

- **10.1 Run Phase 6 product reliability cases.**
  - Design: Use Docdex, Mcoda, Codali, SDK, and CLI fixtures to prove the UI can inspect successful jobs, retryable failures, final failures, cancellations, timeouts, policy denials, budget exhaustion, node disconnects, and disputed usage.
  - Output: Product-driven test matrix and evidence screenshots or logs.
  - Validation: Every product case leaves readable audit, usage, receipt, and reason-code trails.

- **10.2 Run security and redaction validation.**
  - Design: Exercise cross-tenant access, role limits, redaction profiles, encrypted Docdex/RAG metadata, secret-bearing fields, key metadata, and diagnostic bundles.
  - Output: Security test report tied to SDS validation items.
  - Validation: Tests prove raw prompts, decrypted snippets, file paths, query text, key material, credentials, secrets, private payloads, and unredacted result contents do not render unless explicitly authorized by owning services.

- **10.3 Run accessibility and dense-table usability validation.**
  - Design: Validate keyboard navigation, focus order, screen-reader labels, long reason-code wrapping, stable table dimensions, loading states, empty states, and responsive behavior.
  - Output: Accessibility checklist and UI regression coverage.
  - Validation: Automated checks plus focused manual review pass for the main panels and action confirmation flow.

- **10.4 Run integration and contract validation.**
  - Design: Validate generated bindings, schema compatibility, Overgate admin routes, read-only mode, action submission, idempotency, stale-state blocking, and timeline assembly.
  - Output: Integration test suite tied to this sub-build plan.
  - Validation: `docdexd run-tests` or the repo's canonical full test command passes once implementation exists.

- **10.5 Prepare Phase 7 and Phase 13 handoff.**
  - Design: Document which UI surfaces remain disabled or readiness-only until grid-resident backbone operations, stronger governance, incident response, and compliance hardening exist.
  - Output: Handoff notes for system-service operations, incident readiness, break-glass execution, governance reporting, and compliance views.
  - Validation: Handoff review confirms Phase 6 UI completion does not imply permission for later high-risk operations.

## Alignment Review

- The sub-build plan keeps Admin and Developer UI in master Phase 6, matching the SDS, service catalog entry, and build-plan crosswalk.
- The plan explicitly depends on earlier master phases for schemas, identity, queue/execution state, policy, verification, disputes, metering, ledger, billing, grants, and rights.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.
- The plan adds only a more detailed per-SDS implementation layer under `docs/build_plan`.
- The plan respects the accepted tech stack: Rust services own Overgate-admin APIs and TypeScript owns only the operator/developer UI surface.

## Exit Gate

SUB BUILD PLAN #1 is complete when a builder can implement the Admin and Developer UI as a read-only-first Phase 6 operator surface, then add bounded signed admin actions without violating tenant isolation, redaction, auditability, tech-stack boundaries, or the canonical master build order.
