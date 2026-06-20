# SUB BUILD PLAN #68 - Central AI Stewardship Interface

Attached SDS: [SDS #68 - Central AI Stewardship Interface](../sds/native_apps/central_ai_stewardship_interface.md)

## Purpose

This sub-build plan turns SDS #68 into an implementation sequence for Central AI Stewardship Interface. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Central AI Stewardship Interface is the Phase 12 native app surface that makes central AI stewardship inspectable and correctable. It owns role-aware dashboard preferences, redacted view models, stewardship work queues, signed review action envelopes, audit display records, usage refs, report views, correction/withdrawal notices, and replay views. It does not own Central AI recommendations, grants, ORU or Seal Ledger truth, fraud cases, claims, public-interest allocations, reports, PIPs, incidents, policy decisions, or final dispute outcomes. Those records remain with their owning services and review bodies.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #68: Central AI Stewardship Interface](../sds/native_apps/central_ai_stewardship_interface.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Central AI Stewardship Interface service plan](../service_catalog/native_apps/central_ai_stewardship_interface.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency keys, stable errors, local stack, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Policy Dry-Run previews, Overclaim appeal/dispute refs, Fraud Control evidence, Challenge Task facts, Workload Classifier facts, and deny-by-default behavior. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill/Overgrant accounting truth, receipt refs, statement refs, and the rule that the interface displays accounting refs but never mutates balances or allocations. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies AI Gateway Router, encrypted RAG, product-adapter, admin, CLI, SDK, usage, audit, and replay groundwork consumed by Central AI Service and stewardship views. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, namespace, private refs, protected content refs, redaction boundaries, report artifacts, and replay substrates. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies Public-Interest Pool Service, Purpose Tag Registry, Overgrant expansion, trusted public-interest records, donation/sponsor refs, quota/fairness windows, and public-interest evidence. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies Fraud Control, Public Provider Onboarding, Public Sandbox Profile, public-provider fraud evidence, challenge refs, payout-risk refs, appeal/correction refs, and public low-sensitivity proof boundaries. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Controls the first build point for the read-heavy, role-aware stewardship surface and the first narrow signed-review action set. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal authority boundaries, stewardship reporting, compliance boundaries, incident escalation, threat review, retention, legal holds, redaction review, report publication hardening, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #68 first build work aligned to master Phase 12, with Phase 13 as governance/security/compliance/reporting hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and contracts, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where a service boundary exists, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript for web/client surfaces only, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, hidden-admin, direct-mutation, or raw-private-evidence drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 10, 11, 12, and 13 | Attach SDS #68, preserve Phase 12 as first build, record prerequisites, and freeze interface authority boundaries. |
| 2 | Master Phases 0, 1, 4, 5, 8, 10, 11, and 12 | Define Rust contracts, canonical schemas, TypeScript-facing bindings, state machines, stable errors, signed refs, hashes, events, and fixtures. |
| 3 | Master Phases 1, 4, 8, 12, and 13 | Implement audience, role, redaction, view assembly, saved views, and display-history foundations. |
| 4 | Master Phases 4, 6, 8, 10, 11, and 12 | Implement dashboard, recommendation, work-queue, evidence summary, provenance, review state, and action eligibility views. |
| 5 | Master Phases 5, 10, 11, and 12 | Implement public-interest project, grant/pool, donation, surplus-routing, purpose-tag, accounting-ref, and outcome views. |
| 6 | Master Phases 4, 5, 10, 11, 12, and 13 | Implement fraud evidence, appeals, disputes, signed review action envelopes, owner-service routing, denial handling, and correction paths. |
| 7 | Master Phases 8, 12, and 13 | Implement report publication views, correction/withdrawal timelines, system-health views, public-safe reports, replay, and report export refs. |
| 8 | Master Phases 1, 5, 6, 8, 12, and 13 | Implement usage, audit, observability, diagnostics, SDK/CLI/admin projections, Personal AI handoffs, and mobile-safe client contracts. |
| 9 | Master Phase 13, with prerequisites from Phases 0 through 12 | Harden authority, privacy, compliance, retention, incident, threat-model, security-review, report-publication, and scale behavior. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Central AI Stewardship Interface uses Rust-first shared contracts and service-facing APIs for view models, signed action envelopes, policy/audience checks, usage refs, audit refs, and replay bundles. TypeScript is acceptable for the web/native client surface and generated bindings, but it must call Overrid APIs and must not become a privileged control plane.
- Dashboard views, recommendation views, work items, action envelopes, report views, public-interest views, fraud summaries, appeal views, usage refs, events, deterministic fixtures, replay bundles, redaction profiles, and report exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/steward/operator/service envelopes, tenant or system scope, trace id, idempotency key, role refs, audience class, evidence refs, policy refs, reason codes, redaction profile refs, downstream owning-service refs, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for view snapshots, action envelopes, report artifacts, replay bundles, audit exports, fixture inputs, and deterministic view/action comparisons.
- Storage, queueing where needed, object refs, protected private records, namespace routes, identity, key status, policy, audit, usage, accounting refs, report publication, and diagnostics must use native Overrid service boundaries such as Overbase, Overqueue, Overstore, Overvault, Universal Namespace Service, Overpass, Overtenant, Overkey, Overgate, Overguard, Overwatch, Overmeter, ORU Account Service, Seal Ledger, Overbill, Overgrant, Public-Interest Pool Service, Fraud Control, Overclaim, Stewardship Reporting Service, Compliance Boundary Service, Incident Response Service, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue forecasts, customer-count assumptions, raw private evidence, raw encrypted Docdex context, vault secrets, direct ledger mutation, direct grant allocation, final fraud authority, final dispute authority, direct report publication, or hidden admin action the interface boundary.

## Phase 1: SDS Attachment, Phase 12 Scope, And Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #68.**
  - Design: Link this document from the Central AI Stewardship Interface SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/native_apps/central_ai_stewardship_interface.md`, `docs/service_catalog/native_apps/central_ai_stewardship_interface.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #68 returns both the Central AI Stewardship Interface SDS and this sub-build plan.

- **1.2 Preserve master Phase 12 as the first build point.**
  - Design: Keep the first implementation in Phase 12 because the useful first slice is a read-heavy native stewardship surface over already-built identity, policy, accounting, public-interest, fraud, AI routing, storage, audit, and replay substrates.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 6, 8, 10, and 11 supply prerequisites; Phase 12 builds the interface proof; Phase 13 hardens formal governance, compliance, security, reporting, and scale.
  - Validation: Review proves the plan does not move the interface into Phase 6 AI routing, Phase 10 pool authority, Phase 11 fraud finality, or Phase 13-only governance delivery.

- **1.3 Freeze the interface ownership boundary.**
  - Design: Record that the interface owns role-aware views, saved filters, redacted view models, signed review action envelopes, work-queue display state, audit display records, usage refs, report views, and replay projections.
  - Output: Ownership checklist for architecture, UI, API, implementation, operations, and review gates.
  - Validation: Review confirms the interface does not own canonical Central AI recommendations, grants, public-interest allocations, ORU/Seal Ledger entries, fraud cases, claims, report publication truth, PIPs, incidents, policy finality, or dispute finality.

- **1.4 Carry forward resolved SDS #68 decisions.**
  - Design: Preserve the resolved audience classes, steward-review requirements, append-only report correction/withdrawal timeline, structural surplus-routing public fields, and minimum Phase 12 slice.
  - Output: Resolved-decision checklist covering public, affected-party, steward/operator, auditor/legal/compliance audiences; severe-action review; report versioning; surplus routing; and Phase 12 minimum surface.
  - Validation: Review rejects raw private evidence publication, Central-AI-only final actions, silent report rewrites, speculative financial fields, and Phase 13-only deferral of the first interface.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Record how Central AI Service, Overgrant, Public-Interest Pool Service, Purpose Tag Registry, Fraud Control, Overclaim, Stewardship Reporting Service, PIP Registry, Compliance Boundary Service, Incident Response Service, ORU Account Service, Seal Ledger, Overbill, Wallet/Usage Center, Personal AI Assistant, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, Overbase, Overstore, and Overvault interact through refs.
  - Output: Boundary matrix naming allowed reads, view refs, action routes, denied direct mutation, audience rules, usage refs, audit refs, replay refs, and owner-service correction paths.
  - Validation: Review confirms downstream services keep final identity, policy, redaction, grant, accounting, fraud, dispute, report, incident, compliance, and governance authority.

## Phase 2: Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the stewardship interface Rust contract module.**
  - Design: Add contract types for dashboard views, recommendation view refs, work items, review action envelopes, public-interest project views, surplus-routing views, fraud evidence summaries, appeal/dispute views, report publication views, system-health views, usage refs, event payloads, replay bundles, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, review-action enums, audience-class enums, report-state enums, work-item-state enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Central AI analysis, accounting mutation, fraud finality, dispute finality, report publication authority, and governance finality.

- **2.2 Define dashboard, work-item, and recommendation schemas.**
  - Design: Model role-aware dashboard sections, counts, filters, saved views, recommendation refs, evidence package refs, provenance refs, review state, downstream owner refs, action eligibility, appeal path, redaction audience, and display snapshot hash.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, public/steward/operator/auditor examples, missing-evidence fixtures, and stale-recommendation fixtures.
  - Validation: Schema tests reject recommendation views without evidence refs, owner refs, review state, appeal path, audience class, redaction profile, policy refs, or display snapshot refs.

- **2.3 Define public-interest, grant, donation, and surplus schemas.**
  - Design: Model project refs, purpose tags, grant/pool refs, donation/source refs, usage/outcome refs, structural surplus refs, contribution/allocation refs, state, review refs, correction paths, and redaction profiles.
  - Output: Public-interest schema set, grant/pool view schema, donation view schema, surplus-routing schema, aggregate-public examples, affected-party examples, steward examples, and negative fixtures for forbidden financial fields.
  - Validation: Tests reject hardcoded prices, revenue forecasts, customer counts, payout rules, internal cost models, private donor facts, per-user usage leakage, provider-private payout data, and speculative financial projections.

- **2.4 Define fraud, appeal, report, and replay schemas.**
  - Design: Model fraud evidence summary views, appeal/dispute views, report publication views, correction/withdrawal timelines, system-health summaries, replay bundles, redaction states, publication refs, review refs, and public URL/namespace refs.
  - Output: Schema set, report-state state machine, appeal/dispute examples, redacted fraud examples, correction/withdrawal fixtures, replay fixtures, and stable error catalog.
  - Validation: Tests prove private fraud internals, payment details, workspace data, encrypted Docdex context, vault secrets, private tenant data, and sensitive compliance facts are absent from public fixtures.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for public dashboard, affected-party appeal, steward review queue, fraud evidence summary, grant view, surplus routing, report publication, redaction denial, signed action routing, downstream denial, correction, withdrawal, usage emission, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected stable errors, BLAKE3 hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, usage refs, audit refs, redacted outputs, report timelines, and replay output across repeated runs.

## Phase 3: Audience, Redaction, View Assembly, And Display History

### Work Items

- **3.1 Implement audience and role resolution.**
  - Design: Resolve public, affected-party, steward, operator, auditor, legal, compliance, and service-account audiences before fetching or rendering evidence summaries.
  - Output: Audience resolver, role-policy adapter, tenant/system scope checks, stronger-role gates, denial refs, and `central_ai_stewardship_interface.dashboard_viewed` audit events.
  - Validation: Tests prove public users cannot access affected-party/steward/operator/auditor views, affected parties only see their own scoped records, and stronger roles require explicit policy authority.

- **3.2 Implement redaction profiles and evidence-summary fetches.**
  - Design: Request redacted summaries and refs from owning services instead of raw private evidence, and block views when a redaction profile is missing or failed.
  - Output: Redaction profile registry, owner-service query adapter, public-safe summary projection, affected-party projection, steward/operator projection, auditor/legal/compliance projection, and redaction-denial records.
  - Validation: Tests prove raw private evidence, encrypted Docdex context, private workspace data, vault secrets, payment details, fraud heuristics, and other-tenant data never enter public or broad logs.

- **3.3 Implement saved views, filters, and dashboard preferences.**
  - Design: Store actor/role dashboard preferences, saved filters, sort rules, visibility state, notification preferences, and query refs without persisting private evidence content.
  - Output: Dashboard preference APIs, saved-query refs, filter schemas, preference usage refs, and role-scoped display projections.
  - Validation: Tests prove preferences cannot widen permissions, persist raw evidence, expose other-tenant identifiers, or override redaction/policy decisions.

- **3.4 Implement display snapshot and replay foundations.**
  - Design: Hash what was displayed, which refs were fetched, which redaction profile applied, which action buttons were enabled, and which downstream owner was named.
  - Output: Display snapshot writer, replay metadata refs, BLAKE3 hash records, trace ids, idempotency refs, and `view_snapshot_recorded` internal event.
  - Validation: Tests prove replay can reconstruct the displayed refs and denied/allowed actions without storing raw private evidence or mutable UI-only state as authority.

- **3.5 Implement view authorization failure handling.**
  - Design: Return redacted/public-safe alternatives, missing-permission explanations, correction paths, and audit refs instead of blank or leaking error states.
  - Output: Stable errors such as `view_not_authorized`, `private_evidence_redacted`, `redaction_required`, `review_role_missing`, and user-safe denial views.
  - Validation: Tests prove denial states are visible, auditable, and actionable without leaking sensitive details or hiding policy failures.

## Phase 4: Dashboard, Recommendation, Work Queue, And Evidence Views

### Work Items

- **4.1 Implement role-aware stewardship dashboard.**
  - Design: Add `GET /stewardship/dashboard` with sections, counts, filters, saved views, work-queue summaries, report states, system-health bands, and role-scoped data.
  - Output: Dashboard API, query projections, owner-service adapters, empty/error states, redaction state badges, usage refs, and audit refs.
  - Validation: API/UI tests prove dashboard counts respect audience, tenant/system scope, redaction, ownership, and stale owner-service state.

- **4.2 Implement recommendation list and detail views.**
  - Design: Add recommendation list/detail views with evidence refs, policy thresholds, model/run provenance refs, confidence/proportionality summary, review state, downstream owner, action eligibility, appeal path, correction state, and expiry.
  - Output: `GET /stewardship/recommendations`, `GET /stewardship/recommendations/{recommendation_id}`, view models, filters, pagination, denial states, and `recommendation_viewed` events.
  - Validation: Tests reject action-enabled recommendation views without evidence refs, owner refs, appeal path, review state, redaction profile, and display snapshot refs.

- **4.3 Implement stewardship work queues.**
  - Design: Add queue views for recommendations, missing evidence, grants, public-interest projects, fraud cases, appeals, reports, incidents, and PIP-linked changes.
  - Output: `stewardship_work_item` projections, priority/severity sorting, due windows, reviewer assignment refs, policy refs, state badges, and audit refs.
  - Validation: Tests prove work queues remain views over owning-service refs and cannot create final outcomes by moving UI state alone.

- **4.4 Implement provenance and missing-evidence surfaces.**
  - Design: Show model/run provenance refs, route refs, context-plan hashes, source inventories, missing-evidence prompts, stale-evidence flags, and owner-service correction paths according to audience.
  - Output: Provenance panel, missing-evidence view model, stale-recommendation warnings, source-service summaries, and redacted replay links.
  - Validation: Tests prove private prompts, decrypted RAG content, fraud internals, private source evidence, and raw workspace data do not appear outside authorized roles.

- **4.5 Implement action eligibility gates.**
  - Design: Disable or hide review actions until required evidence, owner refs, role checks, appeal paths, redaction profile, policy checks, and display snapshot refs exist.
  - Output: Action-eligibility matrix, disabled-state reasons, stable errors, owner-service requirement hints, and audit refs.
  - Validation: Tests prove severe actions cannot be enabled from incomplete recommendations, missing appeals, missing owner refs, stale evidence, failed redaction, or unreviewed private evidence.

## Phase 5: Public-Interest, Grant, Donation, And Surplus-Routing Views

### Work Items

- **5.1 Implement public-interest project views.**
  - Design: Add `GET /stewardship/public-interest/projects` with project refs, purpose tags, grant/pool refs, outcome refs, eligibility state, public/private redaction profiles, reporting summaries, correction paths, and review state.
  - Output: Project list/detail projections, purpose-tag filters, public-safe pages, affected-party pages, steward pages, and `public_interest_viewed` events.
  - Validation: Tests prove private candidate facts, donor-private details, unresolved allegations, regulated-work details, and fraud-sensitive information remain redacted.

- **5.2 Implement grant, pool, and donation views.**
  - Design: Add `GET /stewardship/grants/{grant_or_pool_ref}` with authorized grant, pool, donation, allocation, usage, outcome, fairness, statement, receipt, and correction refs.
  - Output: Grant/pool view models, donation-source summaries, statement refs, Overgrant/Public-Interest Pool/ORU/Seal/Overbill handoff refs, and role-scoped projections.
  - Validation: Tests prove the interface does not authorize allocations, mutate grants, change ledger entries, expose payment details, or invent financial assumptions.

- **5.3 Implement native-service surplus-routing views.**
  - Design: Add `GET /stewardship/surplus-routing` for native service ref/class, reporting period, purpose tag or pool ref, contribution/allocation ref, resource class, measured usage/capacity band, routed/pending/withheld state, Central AI recommendation ref, steward review state, conflict marker, outcome/report refs, and correction path.
  - Output: Surplus-routing list/detail projections, public summaries, steward review summaries, correction links, and no-forecast guardrails.
  - Validation: Tests reject hardcoded prices, forecasts, customer counts, payout rules, internal cost models, donor-private facts, per-user usage, provider-private payout data, and speculative projections.

- **5.4 Implement purpose and outcome reporting views.**
  - Design: Show purpose tag refs, quota/fairness window refs, outcome/report refs, eligibility state, objection path, and correction state without turning summaries into final allocation commands.
  - Output: Purpose/outcome panels, public-safe filters, report links, review refs, and correction/objection affordances.
  - Validation: Tests prove purpose/outcome views cite owning-service refs, remain redacted by audience, and do not bypass Public-Interest Pool Service or Overgrant authority.

- **5.5 Implement public and steward comparison views.**
  - Design: Provide side-by-side public-safe and steward-authorized projections so reviewers can catch redaction gaps before publication.
  - Output: Comparison API, redaction-diff projection, report-publication blockers, privacy review flags, and audit refs.
  - Validation: Tests prove public comparison output never includes private refs while steward output remains policy-scoped and fully auditable.

## Phase 6: Fraud, Appeals, Signed Review Actions, And Owner-Service Routing

### Work Items

- **6.1 Implement fraud evidence summary views.**
  - Design: Add `GET /stewardship/fraud-cases/{case_id}` with case refs, subject refs, severity/confidence summary, redacted signal refs, hold/throttle/challenge refs, appeal refs, allowed audience, and correction state.
  - Output: Fraud evidence summary view, public/affected/steward/operator/auditor projections, `fraud_evidence_viewed` events, and private-signal redaction guards.
  - Validation: Tests prove fraud heuristics, exploit details, raw signals, payment details, private tenant data, and sensitive anti-fraud internals do not leak to public or affected-party views beyond policy.

- **6.2 Implement appeal and dispute views.**
  - Design: Add `GET /stewardship/appeals/{claim_id}` with Overclaim refs, parties, current state, evidence refs, deadlines, review body refs, decisions, correction/retraction refs, and finality markers.
  - Output: Appeal/dispute projections, deadline banners, decision refs, correction/retraction links, and `appeal_viewed` events.
  - Validation: Tests prove affected parties can see permitted records, public users cannot inspect private claims, and severe recommendations cannot proceed when appeal paths are missing.

- **6.3 Implement signed review action envelopes.**
  - Design: Add `POST /stewardship/recommendations/{recommendation_id}/review-actions` for accept, reject, request-more-evidence, correct, retract, publish, withhold, or escalate requests with signed steward/operator identity, reason codes, evidence refs, downstream command refs, idempotency key, and display snapshot hash.
  - Output: `review_action_envelope` writer, signing checks, role checks, action-state machine, `review_action_created` events, and stable errors.
  - Validation: Tests prove unsigned, wrong-role, stale-display, missing-reason, missing-evidence, missing-owner, missing-appeal, duplicate, and forbidden actions are rejected.

- **6.4 Implement owning-service routing and denial handling.**
  - Design: Route signed review actions to Central AI Service, Overgrant, Public-Interest Pool Service, Fraud Control, Overclaim, Stewardship Reporting Service, Incident Response Service, Compliance Boundary Service, or PIP Registry according to target refs.
  - Output: Routing adapter, `review_action_routed` events, downstream accepted/denied refs, retry behavior, pending state, and `review_action_rejected` events.
  - Validation: Tests prove the interface never mutates canonical records locally and records downstream denial reasons plus user-visible correction paths.

- **6.5 Implement correction and retraction request flows.**
  - Design: Support correction/retraction envelopes for recommendations, reports, public-interest views, fraud summaries, appeal views, and display histories while preserving original records.
  - Output: Correction request API, retraction request API, owner-service handoff refs, timeline links, notification refs, and audit refs.
  - Validation: Tests prove corrections create linked records, report/list views prefer current versions, old public artifacts are not silently rewritten, and private evidence remains audience-scoped.

## Phase 7: Reports, System Health, Replay, And Public Publication Views

### Work Items

- **7.1 Implement report list and detail views.**
  - Design: Add `GET /stewardship/reports` and report detail projections with report type, period, redaction profile, review state, publication state, correction refs, withdrawal refs, public URL/namespace refs, and replay id.
  - Output: Report list/detail API, report-state filters, public-safe summaries, steward review views, and `report_viewed` events.
  - Validation: Tests prove reports are withheld when redaction/profile/review refs are missing and public views include only allowed summaries, hashes, reason codes, and publication refs.

- **7.2 Implement report review actions.**
  - Design: Add `POST /stewardship/reports/{report_id}/review-actions` for publish, withhold, correct, retract, request-redaction, and request-more-evidence actions routed to Stewardship Reporting Service.
  - Output: Report action envelope, route adapter, publication blockers, correction/withdrawal refs, and `report_action_created` events.
  - Validation: Tests prove report publication cannot bypass redaction review, steward review, owner-service publication state, or policy checks.

- **7.3 Implement append-only correction and withdrawal timelines.**
  - Design: Show `current`, `corrected`, `withdrawn`, and `superseded` states with timestamp, public-safe reason-code summary, replacement/correction refs, review refs, publication refs, and replay id.
  - Output: Timeline component contract, public list preference for latest current version, correction/withdrawal notices, and redacted diff projection.
  - Validation: Tests prove original reports remain addressable as no-longer-current, private diffs stay hidden, and report lists do not silently rewrite history.

- **7.4 Implement system-health and incident views.**
  - Design: Add `GET /stewardship/system-health` with system-health bands, incident status bands, report freshness, governance status, public-capacity state, abuse/fraud trend summaries, and owner-service refs.
  - Output: System-health dashboard, incident/report correlation refs, public-safe status bands, operator diagnostics, and alert inputs.
  - Validation: Tests prove private incident internals, exploit details, compliance-sensitive facts, exact infrastructure weaknesses, and other-tenant data remain role-scoped.

- **7.5 Implement replay API and public-safe export refs.**
  - Design: Add `GET /stewardship/replay/{view_or_action_id}` to reconstruct displayed refs, redaction profile, action signer, policy decision, downstream command, result, correction, and report state.
  - Output: Replay API, role-scoped replay bundles, public-safe export refs, steward/auditor projections, and deterministic replay fixtures.
  - Validation: Tests prove replay is deterministic, authorization-aware, complete enough for support/dispute/security review, and does not expose private content outside allowed scopes.

## Phase 8: Usage, Audit, Observability, Diagnostics, And Client Handoffs

### Work Items

- **8.1 Emit interface usage refs.**
  - Design: Emit usage refs for dashboard queries, evidence summary reads, recommendation detail views, report exports, public report serving, review actions, replay reads, storage, bandwidth, compute, denials, and corrections where material.
  - Output: Overmeter integration, `interface_usage_ref` writer, `usage_emitted` events, usage reconciliation records, and Wallet/Usage Center/ORU/Seal/Overbill handoff refs.
  - Validation: Tests prove usage links actor/role, tenant/system scope, view id, report id, recommendation id, grant/pool ref, claim/case ref, action id, receipt refs where applicable, and does not maintain balances locally.

- **8.2 Emit Overwatch audit evidence.**
  - Design: Record audit refs for dashboard views, recommendation views, evidence views, appeal views, report views, review action creation/routing/denial, correction/withdrawal requests, replay reads, usage emission, and redaction failures.
  - Output: Audit event mapping, Overwatch refs, redacted projections, immutable decision evidence, audit completeness checks, and support-safe exports.
  - Validation: Tests prove every mutating API and publication-affecting state has audit evidence and broad events omit private evidence, payment details, vault secrets, fraud internals, and private personal data.

- **8.3 Implement observability and diagnostics.**
  - Design: Expose dashboard latency, evidence-view denials, redaction failures, review queue age, action acceptance/denial rate, report publication state, public report traffic, system-health freshness, replay gaps, and usage by view/action type.
  - Output: Metrics, tracing spans, alert rules, operator diagnostics, stale owner-service refs, failed report generation diagnostics, and missing-appeal alerts.
  - Validation: Tests prove alerts fire for private evidence exposure attempts, missing appeal paths, recommendations without evidence, severe actions without signed review, report publication without redaction review, repeated downstream denials, and replay gaps.

- **8.4 Implement SDK, CLI, admin, and client projections.**
  - Design: Provide typed Rust SDK and generated TypeScript/web bindings for dashboard reads, recommendation reads, review actions, report actions, replay reads, and diagnostics; provide CLI/admin commands with stable JSON.
  - Output: Contract examples, SDK bindings, TypeScript client types, CLI JSON examples, admin panel projections, idempotency examples, and redacted error examples.
  - Validation: Contract tests prove clients use normal Overrid APIs, signed envelopes, stable JSON, role/audience scopes, idempotency keys, and no privileged UI shortcuts.

- **8.5 Implement Personal AI, mobile, and public-client handoffs.**
  - Design: Allow Personal AI Assistant, Mobile SDK, Mobile Backend Gateway, public pages, and native apps to navigate or summarize permitted stewardship views under the current user's permissions.
  - Output: Handoff contract, delegated-view refs, permission checks, public URL/namespace refs, mobile-safe redaction, push/export payload redaction rules, and replay refs.
  - Validation: Tests prove Personal AI cannot silently approve actions, mobile/public clients cannot bypass redaction, delegated native-app calls require user permissions, and exported payloads remain public-safe.

## Phase 9: Governance, Compliance, Security, Incident, And Scale Hardening

### Work Items

- **9.1 Harden formal authority boundaries.**
  - Design: Encode Phase 13 policies for severe sanctions, provider suspension, payout holds after finality, grant/pool allocation, native surplus routing, public report publication, report correction/withdrawal, compliance-sensitive action, broad eligibility change, policy change, incident containment, and public communication.
  - Output: Authority boundary matrix, signed-review gates, owner-service routing requirements, steward/legal/compliance review requirements, low-risk automation allowlist, and stable denial reason codes.
  - Validation: Tests prove severe actions, allocations, report publication/correction/withdrawal, compliance-sensitive changes, and policy changes require correct review and owning-service paths.

- **9.2 Harden privacy, retention, and legal-hold behavior.**
  - Design: Apply retention classes for public summaries, affected-party views, steward/operator views, auditor/legal/compliance replay bundles, private refs, report timelines, corrections, withdrawals, incidents, claims, and legal holds.
  - Output: Retention policy config, legal hold refs, redaction tombstones, archive compaction rules, export fixtures, deletion/redaction rules, and audit continuity checks.
  - Validation: Tests prove legal holds override expiry, public archives reduce to safe refs, deletion does not break audit/replay chains, and private evidence remains protected.

- **9.3 Harden threat models and security reviews.**
  - Design: Threat-model signed review action forgery, audience-class confusion, owner-service routing bypass, report redaction failure, private evidence leakage, appeal path suppression, report rewrite abuse, native surplus disclosure, replay/audit gaps, fraud-summary leakage, steward override abuse, and public-report scraping.
  - Output: Threat model entries, security review checklist, abuse fixtures, mitigation records, incident hooks, and accepted-risk records where necessary.
  - Validation: Security tests prove forged actions fail, private refs stay scoped, redaction failures block publication, report history is append-only, replay remains authorized, and owner-service routing cannot be bypassed.

- **9.4 Harden incident and compliance response.**
  - Design: Add incident playbooks for wrong public report, privacy leak suspicion, fraud-summary leakage, mistaken recommendation display, steward action abuse, downstream denial spike, appeal path outage, report correction/withdrawal, public traffic surge, and replay gap.
  - Output: Incident playbook refs, escalation records, quarantine/freeze behavior, owner-service notification refs, correction/retraction workflows, compliance export projections, and post-incident reports.
  - Validation: Drills prove incidents preserve evidence, stop unsafe actions/publication, notify owning services/users where policy requires, reconcile usage, and produce compliance-safe reports.

- **9.5 Harden reliability and scale behavior.**
  - Design: Add bounded concurrency, queue-depth controls, read-heavy caching by redaction profile, backpressure, report pagination, replay pagination, load shedding, stale-owner-service fallback, retry/dead-letter behavior, and degraded-mode reads.
  - Output: Reliability config, worker limits, retry/dead-letter records, backpressure metrics, load test fixtures, report/replay pagination, and scale dashboards.
  - Validation: Load tests prove review queue spikes, report traffic, public health reads, replay reads, owner-service outages, and downstream denials degrade safely without dropping audit evidence or bypassing policy.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate SDS #68 build-breakdown coverage.**
  - Design: Map each SDS build-breakdown item to sub-build phases covering view models, read-only views, redaction/audience enforcement, signed review action envelopes, report publication/correction/withdrawal views, usage/audit/replay/admin diagnostics, and Phase 13 governance hardening.
  - Output: Coverage checklist in review notes and implementation handoff records.
  - Validation: Review proves no SDS #68 build-breakdown item is missing and the plan preserves Central AI Stewardship Interface as a Phase 12 native app surface.

- **10.2 Validate structure and work-item quality.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, five work items per phase, Design/Output/Validation bullets, final newline, and no tab/format drift.
  - Output: Focused validation script evidence for this file.
  - Validation: Script passes for `SUB BUILD PLAN #68`, attached SDS link, phase headings 1 through 10, 50 work items, and complete work-item structure.

- **10.3 Validate links and source alignment.**
  - Design: Check local Markdown links across this plan, the SDS, service catalog entry, master plan, crosswalk, Phase 12, Phase 13, tech-stack decision, progress docs, and queue docs.
  - Output: Link-check evidence and corrected backlinks where needed.
  - Validation: Link checker reports no missing local targets and Docdex search returns aligned SDS/service/sub-build-plan/crosswalk results.

- **10.4 Validate tech-stack and authority guardrails.**
  - Design: Scan changed docs for accepted Rust-first, TypeScript-client-only, canonical JSON/JSON Schema, signed envelopes, Ed25519, BLAKE3, native Overrid boundaries, no conventional database/queue/object-store/vault/KMS, no Kubernetes-first, no blockchain/NFT, no pricing/revenue/customer-count, no direct mutation, no hidden admin, and no raw private evidence.
  - Output: Guardrail scan evidence and any required wording fixes.
  - Validation: Scan output contains only accepted stack language, native Overrid service names, or explicit non-goal/authority-boundary statements.

- **10.5 Update queue, progress, index, and handoff evidence.**
  - Design: Mark `068-build-plan` complete, update progress docs, run targeted Docdex index refresh, run retrieval checks, record the `docdexd run-tests` blocker if still present, and save repo memory.
  - Output: Updated `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, `docs/build_plan/progress.md`, Docdex index/search evidence, and implementation handoff note.
  - Validation: Queue JSON validates, next incomplete build-plan task is `069-build-plan`, Docdex search finds the new sub-build plan with SDS/service backlinks, and repo-wide test execution status is recorded.

## Alignment Review

- SDS #68 already correctly defines Central AI Stewardship Interface as a native app surface, not as Central AI Service, an accounting service, Fraud Control, Overclaim, Stewardship Reporting Service, or a governance authority.
- First implementation remains master Phase 12 because the minimum useful slice is a read-heavy, role-aware dashboard, recommendation/work-queue, public-interest/grant, fraud, appeal, system-health, report, usage, audit, replay, and narrow signed-action interface. Phase 13 remains governance, compliance, security, public-reporting, incident, retention, and scale hardening.
- The master Phase 0 through Phase 13 order remains unchanged. Required alignment updates are backlinks/index rows for SDS #68, service catalog linkage, master/crosswalk rows, Phase 12 wording that states the minimum interface slice, Phase 13 wording that explicitly covers interface threat/security/reliability/reporting hardening, queue/progress evidence, and Docdex index/search refresh.
- The plan respects the accepted Rust-first/native-Overrid stack: Rust contracts and service APIs, TypeScript only for web/client surfaces, canonical JSON and JSON Schema, signed envelopes, Ed25519, BLAKE3, Overrid-native state/storage/private/audit/usage/accounting boundaries, and no conventional cloud product boundary or speculative financial assumptions.
