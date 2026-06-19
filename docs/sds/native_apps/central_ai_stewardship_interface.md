SDS #68

# Central AI Stewardship Interface SDS

## Purpose

Build the public and administrative interface for central AI stewardship, grants, donations, fraud evidence, system health, appeals, and governance reports.

Central AI Stewardship Interface is the inspectable native app surface for stewardship workflows. It lets authorized users, stewards, operators, reviewers, and the public see evidence-bounded recommendations, public-interest projects, grant and donation records, surplus-routing proposals, fraud/abuse evidence summaries, appeals, system health, and governance reports. It does not run the Central AI Service, mutate ledger or grant state directly, decide disputes, hide private evidence in public reports, or create unreviewable AI authority.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [central_ai_stewardship_interface.md](../../service_catalog/native_apps/central_ai_stewardship_interface.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |

## Service Family

- Family: Native applications
- Owning layer: Native public utility application layer and governance-facing stewardship console
- Primary data scope: dashboard preferences, stewardship work queues, recommendation views, review action envelopes, public-interest project views, grant/donation/surplus refs, fraud evidence summaries, appeal/dispute views, report publication refs, system-health views, redaction decisions, and usage refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md), with governance hardening in [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md)

## Problem Statement

Overrid's central AI mechanism is intended to help protect the ecosystem from fraud, abuse, extractive behavior, and misallocated resources while directing surplus and public-interest resources toward projects that benefit people. That power becomes dangerous if it is hidden behind opaque model outputs, private admin screens, missing appeal paths, or undocumented fund flows.

The stewardship interface must make central AI and stewardship machinery visible, bounded, and correctable. It should show what was recommended, which evidence supports it, who reviewed it, which owning service can act on it, what happened next, what can be appealed, and which public reports can be trusted without leaking private data.

## Goals

- Provide public and authorized stewardship views for recommendations, grants, donations, public-interest pools, surplus-routing proposals, fraud evidence, system health, appeals, and governance reports.
- Show evidence refs, policy thresholds, model/run provenance refs, confidence, proportionality, review state, downstream action refs, and appeal paths for central AI recommendations.
- Let authorized stewards/operators create signed review actions such as accept, reject, request-more-evidence, correct, retract, publish, withhold, or escalate.
- Route all mutating actions to owning services: Central AI Service, Overgrant, Public-Interest Pool Service, Fraud Control, Overclaim, Stewardship Reporting Service, Incident Response Service, ORU/Seal/Overbill services, or PIP Registry.
- Enforce redaction and audience controls so public reports never expose private user, tenant, workspace, Docdex, vault, payment, or fraud-sensitive data.
- Make native-service surplus routing structural and auditable without adding financial projections.
- Provide public reporting views that build accountability without turning central AI into an opaque ruler.

## Non-Goals

- Do not run model analysis, own central AI recommendations, or generate final AI assessments; Central AI Service owns those records.
- Do not directly mutate ORU balances, Seal Ledger entries, grant authorizations, public-interest allocations, payouts, holds, sanctions, or dispute finality.
- Do not expose raw private evidence, encrypted Docdex context, private workspace data, vault secrets, payment details, or sensitive fraud internals.
- Do not replace Overclaim appeals, Fraud Control cases, Overgrant authorization, Public-Interest Pool allocation rules, PIP Registry governance, or Stewardship Reporting publication authority.
- Do not create hidden admin powers, dark patterns, addiction loops, ad-ranking incentives, or private extraction mechanics.
- Do not add pricing, customer-count, revenue, blockchain, NFT, or per-transaction fee assumptions.

## Primary Actors And Clients

- Public users viewing published governance reports, public-interest projects, and stewardship summaries.
- Affected users, providers, organizations, grantees, donors, and project owners viewing their own evidence, appeals, or grant/surplus records.
- Stewards and authorized reviewers evaluating recommendations, grants, fraud evidence, appeals, and report drafts.
- Operators using system health, incident, fraud, and policy views.
- Central AI Service supplying recommendations, analysis refs, model/run provenance, and report draft refs.
- Overgrant, Public-Interest Pool Service, Fraud Control, Overclaim, Stewardship Reporting Service, PIP Registry, Incident Response Service, Compliance Boundary Service, ORU Account Service, Seal Ledger, Overbill, and Wallet/Usage Center supplying owning-service records and action endpoints.
- Personal AI Assistant helping users navigate public and authorized stewardship views with permission.

## Dependencies

- [Central AI Service](../ai_rag_model_routing/central_ai_service.md) for evidence packages, analysis jobs, recommendations, intervention proposals, report refs, corrections, and retractions.
- [Overgrant](../accounting/overgrant.md), [Public-Interest Pool Service](../federation_public/public_interest_pool_service.md), [Purpose Tag Registry](../federation_public/purpose_tag_registry.md), [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), and [Overbill](../accounting/overbill.md) for grants, pools, purpose tags, accounting refs, receipts, and statements.
- [Fraud Control Service](../federation_public/fraud_control_service.md), [Overclaim](../trust_policy_verification/overclaim.md), [Overguard](../trust_policy_verification/overguard.md), [Overwatch](../control_plane/overwatch.md), and [Challenge Task Service](../trust_policy_verification/challenge_task_service.md) for fraud evidence, policy decisions, audit events, challenge results, disputes, and appeals.
- [Stewardship Reporting Service](../governance_ops/stewardship_reporting_service.md), [Protocol Improvement Proposal Registry](../governance_ops/pip_registry.md), [Compliance Boundary Service](../governance_ops/compliance_boundary_service.md), [Incident Response Service](../governance_ops/incident_response_service.md), and [Threat Modeling and Security Review Tracker](../governance_ops/threat_modeling_security_review_tracker.md) for governance and compliance hardening.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), [Overkey](../control_plane/overkey.md), [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), [Overvault](../data_storage_namespace/overvault.md), [Overmeter](../execution_scheduling/overmeter.md), and [Overwatch](../control_plane/overwatch.md) for identity, state, objects, private refs, usage, and audit.

## Owned Responsibilities

Central AI Stewardship Interface owns:

- User-facing dashboard configuration, saved filters, role-aware views, and notification preferences for stewardship work.
- Work-queue views for recommendations, grants, public-interest projects, fraud/abuse cases, appeals, reports, incidents, and PIP-linked changes.
- Redacted recommendation view models assembled from owning-service refs.
- Signed review action envelopes and action-routing records for authorized stewards/operators.
- Evidence-summary view models by audience: public, affected party, steward, operator, auditor, or legal/compliance reviewer.
- Public report publication views, report correction views, and report withdrawal notices.
- Audit display records showing what the interface showed, what action was requested, and which owning service accepted or denied it.
- Usage records for dashboard queries, report exports, evidence package views, and review actions.

The interface does not own canonical recommendations, grants, ledger entries, fraud cases, claims, reports, PIPs, incidents, policy decisions, or central AI analysis jobs.

## Data Model

- `stewardship_dashboard_view`: actor/role refs, tenant/system scope, dashboard sections, filters, sort rules, redaction audience, saved query refs, and visibility state.
- `recommendation_view_ref`: Central AI recommendation ref, evidence package refs, recommendation type, confidence/proportionality summary, model/run provenance refs, review state, downstream owner refs, action eligibility, and appeal path.
- `stewardship_work_item`: work type, owning service, target refs, priority/severity, due window, reviewer assignment refs, policy refs, state, and audit refs.
- `review_action_envelope`: signed steward/operator action, target refs, action type, reason codes, evidence refs, requested downstream command, idempotency key, and result refs.
- `public_interest_project_view`: project refs, purpose tag refs, grant/pool refs, donation/source refs, outcome refs, eligibility state, reporting summaries, and public/private redaction profiles.
- `surplus_routing_view`: native app/service refs, structural surplus refs, proposed pool/grant/project refs, central AI recommendation refs, review refs, and public reporting refs.
- `fraud_evidence_summary_view`: risk case refs, subject refs, severity/confidence summary, redacted signal refs, hold/throttle/challenge refs, claim/appeal refs, and allowed audience.
- `appeal_dispute_view`: Overclaim ref, parties, current state, evidence refs, deadlines, review body refs, decisions, correction/retraction refs, and finality marker.
- `report_publication_view`: report ref, report type, period, redaction profile, review status, publication state, correction refs, withdrawal refs, and public URL/namespace refs.
- `interface_usage_ref`: dashboard query, evidence view, report export, review action, storage/bandwidth/compute usage, Overmeter refs, and wallet/receipt refs where applicable.

Common envelope fields: `id`, `tenant_id` or `system_scope`, `actor_id`, `role_refs`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

- `GET /stewardship/dashboard`: returns role-aware dashboard sections, counts, filters, and saved views.
- `GET /stewardship/recommendations`: lists recommendation view refs by status, type, owner, severity, purpose tag, or report period.
- `GET /stewardship/recommendations/{recommendation_id}`: returns redacted recommendation details, evidence refs, review state, provenance refs, and allowed actions.
- `POST /stewardship/recommendations/{recommendation_id}/review-actions`: creates a signed review action envelope and routes it to the owning service.
- `GET /stewardship/public-interest/projects`: lists public-interest project views, pool refs, purpose tags, outcome refs, and public reporting summaries.
- `GET /stewardship/grants/{grant_or_pool_ref}`: returns authorized grant, pool, donation, allocation, usage, and reporting refs.
- `GET /stewardship/surplus-routing`: lists native-service surplus-routing proposals and review state without financial forecasts.
- `GET /stewardship/fraud-cases/{case_id}`: returns a redacted fraud evidence summary according to audience.
- `GET /stewardship/appeals/{claim_id}`: returns allowed appeal/dispute state, evidence refs, deadlines, and decision refs.
- `GET /stewardship/system-health`: returns system-health, incident, public-capacity, report, and governance status summaries.
- `GET /stewardship/reports`: lists report publication views.
- `POST /stewardship/reports/{report_id}/review-actions`: routes publish, withhold, correct, retract, or request-redaction actions to Stewardship Reporting Service.
- `GET /stewardship/replay/{view_or_action_id}`: reconstructs what was displayed, which refs were used, and which downstream command was submitted.

Mutating APIs require signed identity, steward/operator role refs where applicable, tenant/system scope, trace id, idempotency key, reason codes, evidence refs, and policy refs. Stable errors include `view_not_authorized`, `private_evidence_redacted`, `recommendation_not_reviewable`, `owning_service_required`, `appeal_path_missing`, `redaction_required`, `review_role_missing`, `report_not_publishable`, `action_not_allowed`, and `downstream_action_denied`.

## Event Surface

- `central_ai_stewardship_interface.dashboard_viewed`: role-aware dashboard rendered.
- `central_ai_stewardship_interface.recommendation_viewed`: recommendation view rendered with audience/redaction refs.
- `central_ai_stewardship_interface.review_action_created`: signed review action envelope created.
- `central_ai_stewardship_interface.review_action_routed`: review action sent to owning service.
- `central_ai_stewardship_interface.review_action_rejected`: owning service or policy rejected the action.
- `central_ai_stewardship_interface.public_interest_viewed`: public-interest project or grant view rendered.
- `central_ai_stewardship_interface.fraud_evidence_viewed`: fraud evidence summary rendered under audience controls.
- `central_ai_stewardship_interface.appeal_viewed`: appeal/dispute view rendered.
- `central_ai_stewardship_interface.report_action_created`: report publish/correct/retract/withhold action created.
- `central_ai_stewardship_interface.report_viewed`: public or authorized report view rendered.
- `central_ai_stewardship_interface.usage_emitted`: dashboard/query/export/review usage emitted.

Events include view refs, actor/role refs, audience class, redaction profile, target refs, owning service refs, action type, result refs, policy refs, audit refs, and usage refs. Events must not include private evidence content, encrypted context, raw fraud signals, payment details, vault secrets, or private personal data outside an authorized audience.

## Core Workflow

1. User opens the stewardship interface; app resolves actor, role, tenant/system scope, and audience class.
2. Interface loads dashboard view refs from owning services and applies redaction/policy filters before display.
3. User selects recommendation, grant, public-interest project, fraud case, appeal, report, or system-health item.
4. Interface requests redacted summaries and evidence refs from owning services; raw private data remains with the owner.
5. Authorized reviewer creates a signed action envelope such as accept, reject, request-more-evidence, correct, retract, publish, withhold, or escalate.
6. Interface routes the action to the owning service and records result refs, denial refs, or follow-up work items.
7. Public reports and project pages show only approved summaries, publication refs, corrections, and withdrawal notices.
8. Usage and audit refs are emitted for dashboard access, evidence views, exports, and review actions.

## State Machine

Work item lifecycle:

1. `visible`
2. `triaged`
3. `review_pending`
4. `more_evidence_requested`
5. `action_submitted`
6. `accepted_by_owner`
7. `rejected_by_owner`
8. `appeal_pending`
9. `corrected`
10. `retracted`
11. `closed`

Review action lifecycle:

1. `draft`
2. `signed`
3. `policy_checked`
4. `routed`
5. `accepted`
6. `denied`
7. `superseded`
8. `cancelled`

Report publication lifecycle:

1. `draft_view`
2. `redaction_review`
3. `steward_review`
4. `publication_requested`
5. `published`
6. `withheld`
7. `corrected`
8. `withdrawn`

State transitions preserve view/action history. A corrected recommendation or report creates linked correction records rather than editing away the original displayed facts.

## Policy And Security

- Every view must resolve an audience class before evidence is fetched or rendered.
- Private evidence stays in owning services; the interface requests redacted summaries and refs.
- Severe interventions, grant decisions, public report publication, sanctions, or compliance-sensitive actions require role-bound signed review action envelopes.
- Central AI recommendations must show evidence refs, review state, downstream owner, and appeal/correction path before action buttons are enabled.
- Public views must default to aggregate or redacted summaries; affected-party and steward views require stronger identity/role checks.
- Report publication requires redaction profile, privacy review, and owning-service publication state.
- The interface cannot create hidden actions. Every action must have a user-visible reason code, signer, target refs, downstream result, and Overwatch event.
- Personal AI can help navigate the interface only under the user's current permissions and cannot silently approve stewardship actions.

## Metering And Accounting

- Emit usage refs for dashboard queries, evidence summary reads, report exports, review actions, public report serving, and storage/bandwidth/compute consumed by the interface.
- Link usage to actor/role, tenant/system scope, view id, report id, recommendation id, grant/pool ref, claim/case ref, action id, and receipt refs where applicable.
- Grant, donation, surplus, ORU, Seal Ledger, billing, and public-interest allocation truth remains in the owning accounting and pool services.
- Native-service surplus routing is displayed as structural refs and review state; the interface must not encode financial projections, hardcoded prices, forecasts, or payout rules.
- Public views should be efficient and cacheable when policy allows, but cache entries must carry redaction profile and invalidation refs.

## Observability And Operations

- Expose dashboard latency, evidence-view denials, redaction failures, review queue age, action acceptance/denial rate, report publication state, public report traffic, system-health view freshness, and usage by view/action type.
- Alert on private evidence exposure attempts, missing appeal paths, recommendations without evidence refs, severe actions without signed review, report publication without redaction review, and repeated downstream action denials.
- Provide operator diagnostics for stale work queues, missing owning-service refs, failed report generation, and unavailable evidence summaries.
- Provide replay for what an actor saw before taking an action.
- Support report correction and withdrawal flows without deleting prior publication evidence.

## Failure Modes And Recovery

- Actor lacks audience permission: show redacted or public-safe view and record denial refs.
- Recommendation lacks evidence refs: disable review action and request correction from Central AI Service.
- Appeal path missing: block severe action until owning service supplies an appeal/correction path.
- Redaction profile missing or failed: withhold public view/report until reviewed.
- Owning service unavailable: keep work item pending, preserve signed action envelope, and retry with idempotency key.
- Downstream service rejects review action: record denial and show reason/actionable correction path.
- Report already published with later correction: publish correction or withdrawal notice linked to original report.
- Usage emission fails: mark usage pending and reconcile before finalizing export-heavy or action-heavy flows.

## Validation Plan

- Recommendations shown in the interface cite evidence refs, route/provenance refs, review state, downstream owner, and appeal path.
- Public report and public-interest views do not expose private user, tenant, payment, workspace, Docdex, vault, or fraud-sensitive data.
- Review actions are signed, idempotent, role-checked, and routed to owning services rather than mutating records locally.
- Fraud cases and appeals show different redaction levels for public, affected party, steward, operator, and auditor audiences.
- Grant, donation, and surplus-routing views reference accounting/pool records without adding financial projections.
- Report publish/correct/retract flows require redaction and stewardship review refs.
- Replay reconstructs displayed refs, redaction profile, action signer, policy decision, downstream command, and result.
- Personal AI access to stewardship views follows the user's current permissions.

## Build Breakdown

1. Define view models for dashboard, recommendation, work item, review action, public-interest project, fraud evidence, appeal, report publication, and usage refs.
2. Implement read-only dashboard, recommendation, public-interest, fraud, appeal, report, and system-health views using fixtures from owning services.
3. Add redaction/audience enforcement and validation fixtures for public, affected-party, steward, operator, and auditor roles.
4. Add signed review action envelopes and route them to Central AI Service, Overgrant, Fraud Control, Overclaim, Public-Interest Pool Service, and Stewardship Reporting Service.
5. Add report publication/correction/withdrawal views and public report rendering.
6. Add usage metering, audit events, replay views, and Admin/Developer UI diagnostics.
7. Harden Phase 13 governance behavior: PIP links, compliance boundaries, incident links, threat-review links, and public reporting exports.

## Handoff And Downstream Use

Central AI Stewardship Interface makes central AI stewardship inspectable and accountable. It is the public and authorized review surface for recommendations, grants, fraud evidence, appeals, native-service surplus routing, system health, and governance reports.

Downstream services should expose explicit view/query/action endpoints for the interface. The interface must not read private service storage directly or treat a UI action as final mutation unless the owning service returns accepted result refs.

## Open Design Questions

Resolved decisions:

- Public-by-default records are limited to published stewardship report metadata and artifacts, public-interest project pages, public pool and purpose-tag summaries, aggregate grant/donation/surplus/usage/outcome summaries, system-health bands, incident status bands, correction or withdrawal notices, and public-safe central-AI recommendation summaries after redaction and review. Affected-party roles can see their own cases, appeals, grants, review state, provider-safe reason codes, and allowed evidence refs. Steward and operator roles can see work queues, signed review envelopes, richer evidence summaries, missing-evidence prompts, redaction failures, incident/fraud operational state, and downstream denial reasons. Auditor, legal, and compliance roles can see replay bundles, source inventories, boundary decisions, retained private refs, and stronger provenance only when policy, retention, and jurisdiction allow. Raw private evidence, encrypted Docdex or vault context, private workspace data, payment details, fraud heuristics, secret-bearing refs, and other-tenant data are never public.
- Human/steward review is required before downstream action for severe sanctions, provider suspension, payout holds after finality, grant or public-interest pool allocation, native-service surplus routing, public report publication, report correction or withdrawal, compliance-sensitive action, broad eligibility change, policy change, incident containment or public communication, or any recommendation based on private, regulated, secret-bearing, fraud-sensitive, or appealable evidence. Central AI may automate only non-mutating support work such as evidence validation, missing-evidence prompts, low-severity triage summaries, duplicate/stale recommendation expiry, public-safe report draft sections, and routing suggestions. A downstream owner may automate a bounded low-risk action only when its own policy explicitly permits it and the Central AI recommendation remains supporting evidence rather than final authority.
- Public reports show corrections and withdrawals through an append-only version timeline with `current`, `corrected`, `withdrawn`, or `superseded` state, effective timestamp, public-safe reason-code summary, replacement or correction refs, review refs, publication refs, and replay id. The interface must not delete or silently rewrite the original report; it marks the old artifact as no longer current, links the correction or withdrawal notice, and has search/list views prefer the latest current version while preserving history. Public diffs are limited to redacted statement, aggregate metric, source-class, and conclusion changes; private source evidence, fraud internals, payment details, exploit details, and sensitive compliance facts remain behind audience-scoped Stewardship Reporting or owning-service views.
- Public native-service surplus-routing summaries should show structural accountability fields only: native service ref/class, reporting period, purpose tag or pool ref, contribution/allocation ref, resource class, measured usage or capacity band, routed/pending/withheld state, central-AI recommendation ref, steward review state, conflict-of-interest marker, outcome/report refs, and correction or objection path. They must not include hardcoded prices, revenue forecasts, customer counts, payout rules, internal cost models, donor-private facts, per-user usage, provider-private payout data, or speculative financial projections. Accounting truth remains in ORU Account Service, Seal Ledger, Overbill, Overgrant, Provider Payout Service, and Public-Interest Pool Service.
- The minimum Phase 12 slice before Phase 13 hardening is a read-heavy, role-aware stewardship surface: dashboard counts and filters, recommendation/work-queue views, recommendation detail with evidence/provenance/review/appeal refs, public-interest project and grant/pool views, fraud evidence summaries, appeal/dispute views, system-health and report lists, correction/withdrawal notices, usage/audit refs, and replay for what the actor saw. Signed review action envelopes should exist for the first narrow action set and route to Central AI Service, Fraud Control, Overclaim, Overgrant, Public-Interest Pool Service, and Stewardship Reporting Service, but formal PIP, compliance, threat-review, incident, and public-reporting hardening remains Phase 13 work.
