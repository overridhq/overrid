SDS #77

# Incident Response Service SDS

## Purpose

Coordinate investigation, containment, recovery, communication, evidence retention, drills, post-incident review, and follow-up work for operational, abuse, fraud, security, compliance, and stewardship incidents.

Incident Response Service is the case-management and coordination layer for incidents. It turns Overwatch evidence, fraud signals, disputes, policy decisions, billing/accounting refs, recovery actions, and stewardship reports into a durable incident timeline with owners, containment requests, recovery refs, communications, and lessons learned. It does not replace Overwatch as the event log, directly execute containment in other services, mutate ledger or billing records, adjudicate disputes, or make opaque central AI enforcement decisions.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [incident_response_service.md](../../service_catalog/governance_ops/incident_response_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md), with simple incident records earlier in [Overwatch](../control_plane/overwatch.md) |
| Sub-build plan | [SUB BUILD PLAN #77 - Incident Response Service](../../build_plan/sub_build_plan_077_incident_response_service.md) |

## Service Family

- Family: Governance, compliance, and operations
- Owning layer: Incident case coordination, containment workflow, recovery tracking, communications, drills, post-incident reports, and follow-up actions
- Primary data scope: incident cases, severity, scope snapshots, affected tenants/services/providers/apps, evidence refs, timeline entries, role assignments, containment requests, recovery refs, communication records, drill reports, post-incident reports, action items, and replay bundles
- First build phase from service plan: [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md), with simple incident records earlier in [Overwatch](../control_plane/overwatch.md)

## Problem Statement

Overrid will face node failures, control-plane outages, public-provider abuse, fraud spikes, payment-provider outages, AI routing failures, policy mistakes, privacy issues, security findings, and native-app abuse. If each team handles these as private chat threads or ad hoc operator notes, the ecosystem cannot prove what happened, who acted, which users were affected, what was contained, and how it was fixed.

The service must make incidents operationally disciplined without granting itself unchecked power. It should coordinate containment and recovery by asking the owning services to act through policy-checked, auditable commands. Overwatch remains the evidence log; this service owns the incident case and timeline.

## Goals

- Create durable incident cases with severity, type, affected scope, owners, timeline, evidence refs, and current state.
- Connect operational, abuse, fraud, security, compliance, accounting, native-app, and central-AI incidents to the same coordination model.
- Request containment through Overguard and owning services rather than directly mutating service state.
- Track recovery steps, verification evidence, user/steward/operator communications, and follow-up work.
- Support drills for node failure, provider abuse, payment outage, control-plane outage, route failure, queue backlog, data restore, AI route failure, and public-report correction.
- Produce post-incident reports for Stewardship Reporting with public/private redaction profiles.
- Preserve enough evidence to replay decisions, audit operator actions, and support disputes or appeals.

## Non-Goals

- Do not replace Overwatch event storage, traces, health events, or raw evidence bundles.
- Do not execute route shifts, failover, backups, restores, policy blocks, payout holds, billing corrections, ledger entries, or tenant suspensions directly.
- Do not decide disputes, refunds, final fraud outcomes, or provider payout finality; Overclaim, Fraud Control, Overbill, Seal Ledger, and Provider Payout own those records.
- Do not expose raw private evidence, vault secrets, payment details, encrypted Docdex context, security exploit details, or fraud heuristics in broad incident views.
- Do not become a generic project-management system outside incident response.
- Do not add pricing, revenue, customer-count, blockchain, NFT, or speculative transaction economics.

## Primary Actors And Clients

- Overwatch opening or linking simple incident records, traces, health events, audit evidence, and evidence bundles.
- Operators, incident commanders, security reviewers, compliance stewards, fraud reviewers, and recovery owners.
- Failover and Recovery Coordinator, Backup and Restore Service, Migration Tooling, Release Strategy Service, and Deployment Planner for operational recovery refs.
- Fraud Control Service, Overclaim, Overguard, Challenge Task Service, Oververify, and Reputation and Anti-Sybil Service for fraud, abuse, policy, challenge, and dispute evidence.
- Overbill, Seal Ledger, ORU Account Service, Provider Payout Service, and Overgrant for accounting-impact evidence and downstream action refs.
- Central AI Service and Central AI Stewardship Interface for evidence-bounded analysis, recommendations, review actions, and public-facing stewardship views.
- Stewardship Reporting Service and Compliance Boundary Service for public/private reports and regulated boundary implications.
- Affected users, tenants, providers, organizations, grantees, and native app owners through authorized status and communication views.

## Dependencies

- [Overwatch](../control_plane/overwatch.md) for incident seed records, event evidence, audit refs, traces, health signals, and export bundles.
- [Overguard](../trust_policy_verification/overguard.md) for containment authorization, operator action policy, and reason-coded denials.
- [Overclaim](../trust_policy_verification/overclaim.md), [Fraud Control Service](../federation_public/fraud_control_service.md), [Challenge Task Service](../trust_policy_verification/challenge_task_service.md), and [Oververify](../trust_policy_verification/oververify.md) for dispute, fraud, challenge, and trust evidence.
- [Failover and Recovery Coordinator](../deployment_grid/failover_recovery_coordinator.md), [Backup and Restore Service](../deployment_grid/backup_restore_service.md), [Migration Tooling](migration_tooling.md), and [Release Strategy Service](../deployment_grid/release_strategy_service.md) for operational recovery actions and drill evidence.
- [Overbill](../accounting/overbill.md), [Seal Ledger](../accounting/seal_ledger.md), [ORU Account Service](../accounting/oru_account_service.md), [Provider Payout Service](../accounting/provider_payout_service.md), and [Overgrant](../accounting/overgrant.md) for accounting impact refs, holds, corrections, grants, and payout evidence.
- [Central AI Service](../ai_rag_model_routing/central_ai_service.md), [Stewardship Reporting Service](stewardship_reporting_service.md), and [Compliance Boundary Service](compliance_boundary_service.md) for analysis, reports, regulated boundaries, and redaction profiles.

## Owned Responsibilities

Incident Response Service owns:

- Incident case records, case ids, severity, incident type, affected-scope snapshots, and current state.
- Incident role assignments: commander, technical lead, communications owner, evidence owner, recovery owner, security reviewer, compliance reviewer, and post-incident owner.
- Timeline entries that reference Overwatch events, owning-service refs, operator decisions, user communications, and recovery evidence.
- Containment request records and downstream response refs.
- Recovery step records and verification refs.
- Communication records, audience classes, redaction profiles, and publication state.
- Drill scenarios, drill runs, findings, follow-up refs, and pass/fail evidence.
- Post-incident reports, corrective action items, recurrence prevention refs, and closure evidence.
- Replay bundles for incident decisions and operator actions.

The service coordinates and records action; it does not directly alter state owned by other services.

## Data Model

- `incident_case`: incident id, type, severity, priority, status, summary, originating refs, affected services, affected tenants/users/providers/apps, data classes, created-by, commander refs, and audit refs.
- `affected_scope_snapshot`: time-bound snapshot of affected resources, tenant/user/provider counts as refs or bounded estimates, service refs, route refs, workload refs, accounting refs, data class, and confidence.
- `incident_timeline_entry`: timestamp, entry type, actor/service, source refs, evidence refs, decision refs, command refs, communication refs, redaction class, and correction refs.
- `incident_role_assignment`: role, assignee, start/end, escalation path, backup owner, authority boundary, and audit refs.
- `containment_request`: requested action, target owning service, policy decision refs, reason codes, evidence refs, idempotency key, downstream response refs, expiry, and rollback refs.
- `recovery_step_record`: recovery action, owner service, expected result, verification refs, state, retry policy, rollback refs, and final outcome.
- `communication_record`: audience, channel, message/template refs, approval refs, redaction profile, sent/published state, correction refs, and receipt refs.
- `incident_drill`: scenario, target scope, expected behavior, safety bounds, participants, started/completed timestamps, evidence refs, findings, and follow-up refs.
- `post_incident_report`: incident refs, timeline summary, root-cause refs, contributing factors, impact refs, containment/recovery refs, corrective actions, redaction profile, publication refs, correction refs, and closure refs.
- `follow_up_action`: action owner, target service, priority, due window, linked incident, verification requirement, state, and completion evidence.
- `incident_replay_bundle`: case, timeline, evidence refs, policy decisions, containment requests, recovery refs, communications, post-incident report, and follow-up refs.

Common envelope fields: `id`, `tenant_id` or `system_scope`, `actor_id` or `service_account_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

- `POST /incidents`: declare an incident or promote an Overwatch incident seed into a managed case.
- `GET /incidents/{incident_id}`: read authorized incident status, scope, role assignments, timeline summary, and next actions.
- `POST /incidents/{incident_id}/severity`: change severity or priority with evidence and signer refs.
- `POST /incidents/{incident_id}/scope`: add or correct affected-scope snapshots.
- `POST /incidents/{incident_id}/timeline`: add timeline entries with source refs and redaction class.
- `POST /incidents/{incident_id}/roles`: assign, hand off, or remove incident roles.
- `POST /incidents/{incident_id}/containment-requests`: request a hold, throttle, route change, block, quarantine, suspension, challenge, rollback, or recovery action from an owning service.
- `POST /incidents/{incident_id}/recovery-steps`: record recovery work and verification refs.
- `POST /incidents/{incident_id}/communications`: create, approve, send, correct, or retract incident communications.
- `POST /incidents/{incident_id}/post-incident-reports`: create, review, publish, correct, or retract a report.
- `POST /incident-drills`: schedule or start a controlled incident drill.
- `GET /incident-drills/{drill_id}`: read drill state, evidence, findings, and follow-up work.
- `GET /incidents/{incident_id}/replay`: reconstruct incident timeline, decisions, containment, recovery, and communications.

Mutating APIs require signed identity, role authority, trace id, idempotency key, Overguard policy refs where relevant, and Overwatch audit refs. Stable errors include `incident_scope_missing`, `severity_change_not_authorized`, `evidence_required`, `containment_owner_required`, `policy_denied`, `communication_redaction_required`, `postmortem_not_ready`, and `incident_not_closable`.

## Event Surface

- `incident_response.incident_declared`: managed incident case opened.
- `incident_response.severity_changed`: severity or priority changed with evidence.
- `incident_response.scope_updated`: affected scope added or corrected.
- `incident_response.timeline_entry_added`: timeline entry appended.
- `incident_response.role_assigned`: incident role assignment changed.
- `incident_response.containment_requested`: request sent to owning service.
- `incident_response.containment_acknowledged`: owning service accepted, denied, or completed containment.
- `incident_response.recovery_step_recorded`: recovery step added or updated.
- `incident_response.communication_published`: communication sent, published, corrected, or retracted.
- `incident_response.post_incident_report_ready`: report ready for review or publication.
- `incident_response.drill_completed`: drill completed with findings and follow-up refs.
- `incident_response.incident_closed`: incident closed with report or closure evidence.
- `incident_response.usage_emitted`: usage emitted for response, reporting, drills, or replay.

Events include incident id, severity, affected refs, source refs, timeline refs, role refs, policy refs, audit refs, and redaction class. Events must not include private payloads, raw exploit details, vault secrets, private AI context, payment credentials, or unredacted fraud evidence.

## Core Workflow

1. Overwatch, an operator, Central AI, Fraud Control, or an owning service declares or promotes an incident.
2. Incident Response validates actor authority, incident type, initial evidence refs, and affected-scope hints.
3. The case receives severity, commander, role assignments, timeline, and communication mode.
4. The team requests containment through Overguard and owning service APIs, such as route shift, payout hold, policy block, challenge, quarantine, restore, rollback, or throttling.
5. Owning services return accepted, denied, completed, or failed refs; the incident timeline records results without editing downstream truth.
6. Recovery owners record verification steps, monitoring windows, and service health evidence.
7. Communications are drafted, reviewed, redacted, published, corrected, or retracted according to audience.
8. Post-incident review identifies root cause, contributing factors, lessons, corrective actions, tests, monitoring, policy changes, PIPs, and report publication refs.
9. Closure requires containment/recovery verification, required communications, follow-up ownership, and closure evidence.

## State Machine

Incident lifecycle:

1. `suspected`
2. `declared`
3. `triaging`
4. `contained`
5. `recovering`
6. `monitoring`
7. `resolved`
8. `post_review`
9. `closed`
10. `reopened`
11. `cancelled`

Containment request lifecycle:

1. `draft`
2. `policy_pending`
3. `requested`
4. `accepted_by_owner`
5. `denied_by_owner`
6. `applied`
7. `failed`
8. `rolled_back`
9. `expired`

Communication lifecycle:

1. `draft`
2. `redaction_review`
3. `approved`
4. `published`
5. `corrected`
6. `retracted`
7. `archived`

Drill lifecycle:

1. `planned`
2. `approved`
3. `running`
4. `verifying`
5. `completed`
6. `failed`
7. `cancelled`

State changes append timeline entries. Corrections create new entries linked to prior entries; incident history is not rewritten.

## Policy And Security

- Require role-bound signed actions for declaration, severity change, containment request, communication publication, report publication, and closure.
- Require Overguard policy decisions before containment requests that affect routes, billing, payouts, tenant state, provider eligibility, secret access, public-provider capacity, or public reports.
- Keep raw private evidence in owning services; incident records store refs, summaries, hashes, and redaction profiles.
- Separate internal, affected-party, steward, auditor, and public views.
- Public incident communications must be redacted and must not expose exploit details, private user data, internal topology, fraud heuristics, or sensitive compliance facts.
- Severe AI-related or fraud-related incident recommendations require evidence refs and appeal/correction paths.
- Closure cannot happen while critical containment, recovery, communication, or follow-up requirements remain unowned.

## Metering And Accounting

- Emit usage refs for investigation work, evidence package generation, containment coordination, recovery work, drills, report generation, communications, and replay.
- Link usage to incident id, affected services, system-service account, actor role, tenant/system scope, resource dimensions, and report refs.
- Accounting-impact actions are refs to owning services: holds, refunds, corrections, payout delays, grant pauses, or ledger entries remain outside Incident Response.
- Incident costs and stewardship overhead should be reportable structurally without customer-count or revenue assumptions.
- Drills and recovery work should distinguish system-service overhead from ordinary user workload usage.

## Observability And Operations

- Expose open incidents by severity, service, type, age, owner, containment state, recovery state, communication state, and follow-up status.
- Track time to declare, assign, contain, recover, communicate, close, and complete follow-up work.
- Alert on incidents without commander, severe incidents without containment decision, stale severity, missing affected scope, missing communication owner, unresolved public-provider abuse spikes, and overdue follow-up actions.
- Provide timelines joining Overwatch traces, Overguard decisions, Overclaim disputes, recovery refs, billing refs, public-report refs, and Central AI recommendations.
- Support simulation and drill mode without production side effects.
- Preserve post-incident reports and closure evidence for Stewardship Reporting and PIP follow-up.

## Failure Modes And Recovery

- Incident declared without evidence: keep `suspected` and require evidence before containment.
- Wrong affected scope: append corrected scope snapshot and link the superseded entry.
- Containment owner unclear: block containment request until owning service is identified.
- Policy denies containment: record denial, reason codes, and alternate safe actions.
- Owning service unavailable: keep containment request pending with retry/incident escalation.
- Communication published with wrong redaction: retract or correct, create incident entry, and trigger privacy review.
- Recovery verification fails: reopen `recovering` or create a new incident linked to the original.
- Post-incident follow-up overdue: escalate owner and report status to Stewardship Reporting.
- Overwatch degraded: use a bounded local spool and block closure until evidence is reconciled.

## Validation Plan

- Incidents cite evidence refs, affected scope, severity, owner assignments, and current state.
- Containment actions are routed to owning services through policy-checked, audited requests.
- Drills for node failure, provider abuse, payment outage, control-plane outage, queue backlog, restore failure, and AI route failure produce findings and follow-up work.
- Public incident communications and reports are redacted by audience and never expose private evidence.
- Incident closure is denied while containment, recovery verification, required communication, or follow-up ownership is missing.
- Replay reconstructs timeline, policy decisions, containment requests, recovery refs, communications, and report refs.
- Accounting-impact incidents link to Overclaim, Overbill, Seal Ledger, Provider Payout, or Overgrant records without mutating them.

## Build Breakdown

1. Define incident case, affected scope, timeline, role, containment request, recovery step, communication, drill, post-incident report, follow-up, and replay schemas.
2. Implement incident declaration, read, severity, scope, timeline, and role APIs.
3. Integrate Overwatch incident seeds, traces, health events, and evidence bundles.
4. Add containment request workflow through Overguard and owning service refs.
5. Add recovery step tracking with Failover, Backup/Restore, Migration, Release Strategy, and owning-service integrations.
6. Add communication review, redaction, publication, correction, and retraction workflows.
7. Add drill runner, drill evidence, post-incident reports, follow-up action tracking, and Stewardship Reporting handoff.
8. Add dashboards, replay, validation fixtures, and Phase 13 governance/security hardening.

## Handoff And Downstream Use

Incident Response Service hands incident status, containment refs, recovery refs, communication refs, drill findings, post-incident reports, and follow-up actions to Overwatch, Stewardship Reporting Service, Central AI Service, Compliance Boundary Service, PIP Registry, Threat Modeling and Security Review Tracker, owning services, SDK, CLI, and admin/stewardship UIs.

Downstream services should treat incident requests as coordinated, evidence-backed commands. They must still enforce their own policy and return accepted or denied refs instead of letting Incident Response edit their private state.

## Open Design Questions

Resolved decisions:

- Overwatch may auto-declare `suspected` or `triaging` incident cases for objective, source-trusted signals: service or route outage, queue backlog, failed backup/restore/failover drill, event append or evidence-integrity failure, policy-denial spike, stale system-service health, private-data redaction escape, public-provider abuse spike, payout-risk spike, challenge failure burst, or public report publication error. Steward or operator confirmation is required before promoting ambiguous fraud/abuse signals, security exploit findings, compliance or child-safety issues, public-report allegations, broad provider or tenant sanctions, payout/finality actions, regulated-workload incidents, or any case whose first external communication could expose private, security, fraud, or compliance-sensitive facts.
- Incident severity should use one shared classed model across domains: `sev_0_emergency`, `sev_1_severe`, `sev_2_significant`, `sev_3_minor`, and `sev_4_info_or_drill`. Classification combines user/provider impact, service criticality, data class, evidence confidence, affected-scope size, reversibility, accounting or compliance exposure, public visibility, and recurrence risk. Severe security, privacy, compliance, ledger, vault, public-provider, central-AI, or public-report cases may not be downgraded only because runtime availability is healthy; operational outages may not be inflated by speculation without evidence refs.
- During seed hardware operation, automation may request only reversible, expiry-bound, policy-checked containment through Overguard and the owning service: fresh evidence capture, health rechecks, local audit spool reconciliation, temporary admission pause for an affected queue lane, route drain away from an unhealthy seed instance, stateless worker restart or replacement, cache/artifact quarantine by hash, known-bad package block, challenge request, temporary public-pool throttle, or pre-finality hold recommendation where the owning accounting or payout service policy allows it. Human or steward review is required for tenant or provider suspension, permanent eligibility changes, payout release or reversal, refund/correction finality, secret access, data deletion or retention waiver, stateful restore or promotion, broad policy change, public report publication, and any containment that touches private, regulated, secret-bearing, fraud-sensitive, or appealable evidence.
- Public reporting is required for `sev_0_emergency` and normally required for `sev_1_severe` once immediate containment and redaction review are complete. `sev_2_significant` incidents are public when they affect public/native-app availability, user-visible privacy/security/accounting/compliance behavior, public-provider or public-interest pool trust, published report correctness, or enough affected parties that private notification alone would hide ecosystem risk. `sev_3_minor`, `sev_4_info_or_drill`, and low-confidence suspected cases stay in private steward/operator views or aggregate periodic reports unless a correction, retraction, or public trust obligation applies. Public artifacts must be produced through Stewardship Reporting with Compliance Boundary and security redaction, never from raw incident timelines.
- Founder hardware can leave the normal production path only after Phase 7 recovery evidence shows two consecutive full-backbone cutover drills, at least one planned and one failure-injection, plus per-critical-service backup restore, failover, rollback, queue recovery, Overwatch reconciliation, and route-shift drills with no unresolved `sev_0` or `sev_1` follow-up actions. During migration, each critical service needs a targeted drill before cutover and another after grid-resident promotion; backup integrity should be verified continuously with monthly restore samples until removal. After founder hardware is emergency-only, the baseline cadence is quarterly full-backbone drills, monthly targeted restore/failover/rollback drills for critical services, and an extra drill after major release, migration, PIP, policy, or topology changes.
