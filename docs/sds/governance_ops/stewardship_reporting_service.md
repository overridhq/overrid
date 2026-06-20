SDS #80

# Stewardship Reporting Service SDS

## Purpose

Publish structured system, grant, surplus, fraud, abuse, security, incident, compliance, migration, protocol, public-interest, and central-AI stewardship reports without exposing private user data.

Stewardship Reporting Service is the accountable reporting layer for Overrid governance. It assembles redacted, evidence-backed reports from Overwatch, Seal Ledger, Overgrant, Overclaim, Central AI, PIP Registry, Incident Response, Compliance Boundary, and native service refs. It does not decide grants, spend funds, mutate ledgers, adjudicate disputes, expose private evidence, or turn reports into hidden governance actions.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [stewardship_reporting_service.md](../../service_catalog/governance_ops/stewardship_reporting_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |
| Sub-build plan | [SUB BUILD PLAN #80 - Stewardship Reporting Service](../../build_plan/sub_build_plan_080_stewardship_reporting_service.md) |

## Service Family

- Family: Governance, compliance, and operations
- Owning layer: Report definition, evidence aggregation, redaction, publication, correction, retraction, and public/private stewardship reporting
- Primary data scope: report templates, report periods, metric snapshots, evidence refs, redaction profiles, review assignments, publication jobs, report artifacts, public URLs/namespace refs, correction/retraction records, reconciliation refs, and replay bundles
- First build phase from service plan: [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md)

## Problem Statement

Overrid promises humanity-owned public utility infrastructure rather than private extraction. That promise cannot remain a slogan. Users, builders, providers, stewards, auditors, and the public need structured reports showing system health, native-service surplus routing, public-interest grants, abuse interventions, fraud statistics, security posture, incidents, protocol changes, compliance boundaries, and central AI recommendations.

The hard part is accountability without surveillance. Reports must reconcile to durable evidence and ledger/event data, but they must not expose private user data, workspace content, encrypted Docdex context, secrets, payment details, or sensitive fraud/security details.

## Goals

- Define report templates and reporting periods for system health, grants, public-interest pools, native-service surplus routing, abuse/fraud, incidents, security posture, compliance boundaries, PIPs, migrations, central AI recommendations, and stewardship operations.
- Assemble reports from evidence refs, metric snapshots, ledger/accounting refs, incident refs, PIP refs, compliance refs, and central AI refs.
- Apply audience-specific redaction profiles before publication or export.
- Reconcile reports with Overwatch event data, Seal Ledger/accounting refs, Overgrant/public-interest refs, and owning-service records.
- Publish public reports with correction, retraction, supersession, and archive support.
- Provide private steward/auditor reports with stronger evidence refs where permitted.
- Keep report generation repeatable from stored inputs and template versions.

## Non-Goals

- Do not decide grant allocations, public-interest investments, surplus routing, fraud outcomes, policy decisions, incident containment, protocol acceptance, or compliance findings.
- Do not mutate ORU balances, Seal Ledger entries, Overgrant authorizations, Overbill receipts, payout records, claims, incidents, PIPs, or central AI recommendations.
- Do not store raw private payloads, raw secrets, raw payment details, encrypted Docdex context, or unredacted security/fraud evidence in report artifacts.
- Do not use reports as a marketing dashboard with speculative financial claims, pricing projections, customer counts, or growth targets.
- Do not replace Central AI Stewardship Interface, PIP Registry, Incident Response, Compliance Boundary Service, Overclaim, or Overwatch.
- Do not publish unreviewed severe allegations or sensitive incident details without redaction and steward approval.

## Primary Actors And Clients

- Public users, builders, providers, organizations, grantees, donors, and stakeholders reading published stewardship reports.
- Stewards, auditors, compliance reviewers, incident responders, security reviewers, and operators reviewing draft/private reports.
- Central AI Service and Central AI Stewardship Interface supplying recommendation refs, risk summaries, report drafts, and review actions.
- Overwatch supplying event, audit, health, incident, and evidence bundle refs.
- Seal Ledger, ORU Account Service, Overbill, Overgrant, Provider Payout Service, Public-Interest Pool Service, and Purpose Tag Registry supplying accounting, grant, payout, surplus, and pool refs.
- Fraud Control Service, Overclaim, Challenge Task Service, Oververify, and Reputation and Anti-Sybil Service supplying fraud, dispute, challenge, and trust refs.
- PIP Registry, Compliance Boundary Service, Incident Response Service, Threat Modeling and Security Review Tracker, Migration Tooling, Release Strategy Service, and Backup and Restore Service supplying governance and operations refs.
- SDK, CLI, admin/developer UI, native apps, and public web/report surfaces consuming report listings and exports.

## Dependencies

- [Overwatch](../control_plane/overwatch.md) for audit events, traces, evidence bundles, health, incidents, exports, and integrity refs.
- [Seal Ledger](../accounting/seal_ledger.md), [ORU Account Service](../accounting/oru_account_service.md), [Overbill](../accounting/overbill.md), [Overgrant](../accounting/overgrant.md), [Provider Payout Service](../accounting/provider_payout_service.md), [Public-Interest Pool Service](../federation_public/public_interest_pool_service.md), and [Purpose Tag Registry](../federation_public/purpose_tag_registry.md) for accounting, grants, payouts, surplus routing, pools, and purpose refs.
- [Central AI Service](../ai_rag_model_routing/central_ai_service.md) and [Central AI Stewardship Interface](../native_apps/central_ai_stewardship_interface.md) for recommendations, evidence summaries, review actions, and public views.
- [Fraud Control Service](../federation_public/fraud_control_service.md), [Overclaim](../trust_policy_verification/overclaim.md), [Challenge Task Service](../trust_policy_verification/challenge_task_service.md), and [Oververify](../trust_policy_verification/oververify.md) for fraud, abuse, challenge, dispute, and verification refs.
- [PIP Registry](pip_registry.md), [Compliance Boundary Service](compliance_boundary_service.md), [Incident Response Service](incident_response_service.md), [Threat Modeling and Security Review Tracker](threat_modeling_security_review_tracker.md), and [Migration Tooling](migration_tooling.md) for governance reports, compliance facts, incident reports, security review status, and migration evidence.
- [Overguard](../trust_policy_verification/overguard.md) and [Overtenant](../control_plane/overtenant.md) for report access policy, audience classification, and tenant/system scope.

## Owned Responsibilities

Stewardship Reporting Service owns:

- Report template definitions, template versions, section definitions, metric definitions, and reporting periods.
- Report build jobs, source inventory snapshots, evidence manifests, metric snapshots, and reconciliation records.
- Redaction profiles, audience classes, privacy review state, and public/private artifact generation.
- Draft, review, approval, publication, correction, retraction, supersession, and archive state for reports.
- Public report index records and private steward/auditor report indexes.
- Report replay bundles proving how a report was generated from source refs and template versions.
- Publication event refs, report artifact refs, integrity hashes, export refs, and retention records.
- Usage refs for report generation, export, publication, and public serving.

The service does not own the domain facts being reported. It cites them and reconciles them.

## Data Model

- `report_template`: template id, report type, semantic version, section definitions, required sources, metric definitions, redaction defaults, review requirements, and publication policy.
- `report_period`: report type, time window, scope, source freshness requirements, close state, and correction window.
- `report_build_job`: job id, template version, period, requested audience, source inventory refs, state, retries, failures, and audit refs.
- `source_inventory_snapshot`: required and optional source refs, freshness, owner service, scope, data classes, privacy constraints, and missing-source markers.
- `metric_snapshot`: metric id, source refs, aggregation method, value refs or redacted aggregate, confidence, reconciliation status, and integrity hash.
- `evidence_manifest`: included evidence refs, excluded refs with reason, source hashes, redaction classes, audience, and export integrity refs.
- `redaction_profile`: audience class, fields allowed, fields withheld, aggregation thresholds, security/fraud/compliance restrictions, and reviewer refs.
- `report_artifact`: artifact id, report type, version, audience, format, storage refs, public namespace refs, integrity hash, state, and expiry/archive refs.
- `report_review`: reviewer role, decision, findings, required corrections, redaction notes, approval refs, and final state.
- `correction_retraction_record`: source report, reason, corrected refs, public notice refs, replacement report refs, and audit refs.
- `report_replay_bundle`: template version, period, source inventory, metrics, evidence manifest, redaction profile, review records, artifact refs, publication events, and correction refs.

Common envelope fields: `id`, `tenant_id` or `system_scope`, `actor_id` or `service_account_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

- `POST /stewardship-reports/templates`: create or revise a report template.
- `GET /stewardship-reports/templates/{template_id}`: read authorized template metadata, sections, required sources, and review policy.
- `POST /stewardship-reports/periods`: open or close a reporting period.
- `POST /stewardship-reports/build-jobs`: start a report build job for a template, period, scope, and audience.
- `GET /stewardship-reports/build-jobs/{job_id}`: read build state, source freshness, missing refs, reconciliation status, and errors.
- `POST /stewardship-reports/{report_id}/reviews`: submit redaction, stewardship, compliance, security, or accounting review.
- `POST /stewardship-reports/{report_id}/publish`: publish, withhold, archive, correct, retract, or supersede a report artifact.
- `GET /stewardship-reports`: list public or authorized reports by type, period, audience, state, or owning service.
- `GET /stewardship-reports/{report_id}`: read authorized report metadata, artifact refs, redacted contents, source refs, and correction state.
- `POST /stewardship-reports/{report_id}/exports`: export a report package for public, steward, auditor, or legal/compliance audience.
- `GET /stewardship-reports/{report_id}/replay`: reconstruct report build inputs, redaction, review, publication, and correction refs.

Mutating APIs require signed identity, role refs, tenant/system scope, trace id, idempotency key, and Overwatch audit refs. Stable errors include `template_source_missing`, `period_not_closed`, `source_freshness_failed`, `reconciliation_failed`, `redaction_required`, `review_required`, `publication_not_authorized`, `report_contains_private_data`, and `correction_required`.

## Event Surface

- `stewardship_reporting.template_versioned`: template created or revised.
- `stewardship_reporting.period_opened`: reporting period opened.
- `stewardship_reporting.period_closed`: reporting period closed and eligible for build.
- `stewardship_reporting.build_started`: report build job started.
- `stewardship_reporting.source_inventory_created`: source inventory snapshot created.
- `stewardship_reporting.reconciliation_completed`: source/metric reconciliation completed.
- `stewardship_reporting.redaction_review_completed`: redaction review completed.
- `stewardship_reporting.report_ready`: draft report artifact ready for review or publication.
- `stewardship_reporting.report_published`: report published to public or authorized audience.
- `stewardship_reporting.report_corrected`: correction published.
- `stewardship_reporting.report_retracted`: report retracted with notice refs.
- `stewardship_reporting.export_created`: export package created.
- `stewardship_reporting.usage_emitted`: usage emitted for build, export, publication, or serving.

Events include report id, template version, period, audience, source refs, metric refs, redaction profile, review refs, artifact refs, publication refs, trace id, and audit refs. Events must not include raw private evidence, secret material, encrypted context, payment details, exploit details, or sensitive fraud internals.

## Core Workflow

1. Steward defines or revises a report template with required sections, sources, metrics, redaction profile, and review requirements.
2. Reporting period opens and later closes with a stable scope and source freshness requirements.
3. Build job inventories required source refs from Overwatch, accounting, grants, fraud/dispute, PIP, compliance, incident, migration, security review, and central AI services.
4. Service builds metric snapshots and evidence manifests, marking missing or stale sources explicitly.
5. Report generation applies template version and audience redaction profile to produce draft artifacts.
6. Required reviewers check reconciliation, privacy, security, compliance, accounting, and stewardship correctness.
7. Authorized publisher publishes, withholds, corrects, retracts, or supersedes the report.
8. Public index and private report views expose only the allowed audience fields.
9. Replay reconstructs the report from template, period, source refs, metric snapshots, redaction profile, review records, and publication state.

## State Machine

Report period lifecycle:

1. `planned`
2. `open`
3. `closing`
4. `closed`
5. `corrected`
6. `archived`

Report build lifecycle:

1. `requested`
2. `source_inventory`
3. `reconciling`
4. `redacting`
5. `draft_ready`
6. `review_pending`
7. `approved`
8. `published`
9. `withheld`
10. `corrected`
11. `retracted`
12. `failed`
13. `archived`

Review lifecycle:

1. `assigned`
2. `in_review`
3. `changes_requested`
4. `approved`
5. `rejected`
6. `waived`

Publication lifecycle:

1. `not_public`
2. `publish_requested`
3. `published`
4. `correction_pending`
5. `corrected`
6. `retraction_pending`
7. `retracted`
8. `superseded`

Reports and artifacts are append-only after publication. Corrections and retractions create linked records and public notices rather than deleting history.

## Policy And Security

- Resolve audience class before assembling or serving report content.
- Public reports must use aggregate or redacted evidence, never raw private user, tenant, workspace, payment, vault, encrypted Docdex, security, or fraud data.
- Require privacy/redaction review before public publication.
- Require accounting reconciliation for reports that cite grants, surplus routing, payouts, receipts, ORU, Seal Ledger, or Overbill refs.
- Require security review for reports that include vulnerabilities, incidents, threat models, exploit classes, or recovery details.
- Require compliance review for jurisdiction, regulated-workload, child-safety, retention, deletion, data-residency, payment, custody-like, payout, or public-reporting boundaries.
- Public incident and fraud reports must avoid naming affected parties unless policy and authorization allow it.
- Corrections and retractions must be visible from public report history.

## Metering And Accounting

- Emit usage refs for source inventory, metric aggregation, report generation, redaction review, publication, export, public serving, correction, and replay.
- Link usage to report type, period, template, audience, source services, artifact refs, and system-service account.
- Native-service surplus routing is reported through accounting/stewardship refs without financial forecasts or hardcoded prices.
- Grants and public-interest allocations are reported from Overgrant, Seal Ledger, ORU, and Public-Interest Pool refs; this service does not allocate or spend.
- Public report serving should be cacheable where policy allows, with redaction profile and invalidation refs carried in cache metadata.

## Observability And Operations

- Expose build backlog, source freshness failures, reconciliation failures, redaction failures, review queue age, publication state, correction/retraction count, public traffic, export volume, and replay success.
- Alert on report publication without required review, public artifact containing restricted data class, reconciliation mismatch, source freshness failure, stale public report, correction overdue, and retraction required.
- Provide operator/steward dashboards for report templates, periods, source inventory, missing sources, metric definitions, redaction profiles, review findings, and publication state.
- Support repeatable builds from stored inputs and deterministic template versions.
- Maintain public report archive and private auditor/steward archive with retention policies.

## Failure Modes And Recovery

- Source missing or stale: mark report blocked or partial with explicit missing-source refs; do not invent values.
- Metric reconciliation mismatch: fail build or require review before publication.
- Redaction profile missing: block artifact generation for public audience.
- Restricted data detected in public artifact: block publication, create incident if artifact escaped, and require correction/retraction.
- Reviewer rejects report: return to draft with findings and correction refs.
- Publication fails: preserve approved artifact and retry publication idempotently.
- Published report later found wrong: publish correction or retraction linked to original report.
- Owning service corrects source data: create new report version or correction according to policy.
- Overwatch unavailable: do not finalize report until audit events and source refs are durable.

## Validation Plan

- Reports reconcile with ledger, grant, event, incident, PIP, compliance, fraud/dispute, and central AI data according to template requirements.
- Private user data is excluded, aggregated, or redacted according to audience.
- Report generation is repeatable from stored template version, report period, source inventory, metric snapshots, and redaction profile.
- Public reports cannot publish until required privacy, security, compliance, accounting, and stewardship reviews pass.
- Corrections and retractions preserve public history and link to original report.
- Native-service surplus, grant, public-interest, and central AI recommendation reports reference owning-service records rather than creating new truth.
- Replay reconstructs source refs, reconciliation, redaction, review, publication, and correction history.

## Build Breakdown

1. Define report template, period, build job, source inventory, metric snapshot, evidence manifest, redaction profile, artifact, review, correction/retraction, and replay schemas.
2. Implement template and period APIs with required source declarations.
3. Implement source inventory and metric snapshot collection for system health, Overwatch events, Seal Ledger/accounting, Overgrant, and public-interest refs.
4. Add native-service surplus routing, fraud/abuse, incident, security posture, compliance boundary, PIP, migration, and central AI report types.
5. Add redaction profiles, audience enforcement, review workflow, and public/private artifact generation.
6. Add publication, correction, retraction, export, public index, and archive workflows.
7. Add repeatable replay, reconciliation tests, dashboards, and Phase 13 public-reporting validation.

## Handoff And Downstream Use

Stewardship Reporting Service hands report artifacts, public indexes, private exports, source inventories, metric snapshots, redaction profiles, correction notices, retraction notices, and replay bundles to Central AI Stewardship Interface, public report pages, auditors, compliance reviewers, incident responders, PIP Registry, SDK, CLI, admin UI, and native apps.

Downstream services should contribute evidence through explicit refs and consume report APIs. They must not rely on report artifacts as a substitute for owning-service truth.

## Open Design Questions

Resolved decisions:

- Broad public participation requires, at minimum, recurring reports for system health and availability, public/native-app availability, native-service surplus routing, public-interest grants and pool allocations, ORU/Seal Ledger/Overbill/Provider Payout reconciliation summaries, public-provider capacity and trust posture, abuse and fraud aggregate statistics, security posture and unresolved severe findings, `sev_0` and normally `sev_1` incidents, accepted or emergency PIPs, compliance-boundary changes affecting public users/builders/providers, migration/founder-hardware dependency status, and Central AI recommendation activity. These reports must cite owning-service refs and Overwatch evidence; Stewardship Reporting may withhold a report or publish a partial/missing-source marker, but it must not invent values or become the authority for ledger, grant, fraud, incident, compliance, migration, or protocol truth.
- Public reports should use classed privacy thresholds rather than one global minimum. Ordinary public aggregates need at least `k >= 20` distinct eligible subjects and no single tenant, provider, grantee, incident, or native service contributing more than 25% of a published bucket unless that party is already intentionally public and publication is authorized. Sensitive buckets covering fraud, abuse, security, child safety, regulated workloads, payouts, disputes, or public-provider risk need at least `k >= 50`, provider/tenant contribution caps of 10%, time-window smoothing where feasible, and suppression or coarsening for rare categories. Affected-party views may show self-scoped refs; steward/auditor/private exports may carry stronger evidence refs under audience policy. Raw private content, payment details, vault refs, encrypted Docdex context, fraud heuristics, exploit details, private topology, and identities remain in owning services and appear only as authorized refs or redacted aggregates.
- Central AI may automatically draft non-authoritative material: source inventory summaries, missing-source lists, metric anomaly notes, redaction-risk warnings, audience-specific public-summary drafts, incident/PIP/migration/evidence summaries, trend comparisons, and reviewer-routing suggestions. Steward-authored or steward-approved reports are required from the start for grant and surplus conclusions, public-interest allocation narratives, fraud or abuse allegations, security posture statements, incident public reports, compliance-boundary interpretations, PIP/change-governance summaries, founder-hardware migration status, and any report that could affect reputation, payout, provider eligibility, regulated exposure, or public trust. Central AI drafts must carry provenance, confidence, model/run refs, source refs, and correction paths, and they never become hidden final governance authority.
- Public archives should use immutable report artifacts plus append-only correction, retraction, and supersession records. Each public entry includes report id, slug, type, period, audience, template version, redacted summary, public metrics, owning-service refs, evidence-manifest hash, artifact hash, publication timestamp, correction/retraction state, replacement refs, public URL or namespace ref, and stable reason codes. Corrections publish a visible delta with corrected fields, reason, replacement artifact/hash, source-ref classes, and effective timestamp; retractions keep the original public shell, explain the safe public reason, and link to replacement or withheld status. Private evidence manifests, reviewer notes, raw traces, fraud/security internals, payment/compliance evidence, and private user/tenant/provider refs remain available only through authorized steward, auditor, legal/compliance, or affected-party views.
- During seed hardware operation, publish bootstrap-safe reports that make founder dependency visible without overstating decentralization: seed/private-swarm health, registered node/capacity summaries, control-plane uptime, Overwatch evidence durability, queue/execution/retry health, ORU/Seal Ledger/Overbill reconciliation summaries for private workloads, grant and public-interest allocation summaries, incident/drill summaries, compliance-boundary readiness, PIP/change-log summaries, package/security-review status, migration-readiness and founder-hardware dependency reports, and Central AI recommendation activity. Public reports at this stage must label seed-hardware scope, redact topology and provider-sensitive detail, avoid public-provider payout/finality claims before those systems are live, and use partial or private-only status where evidence is not yet strong enough for broad public publication. Full public-provider, federation, native-app, and broad public-interest reporting expands only after the relevant Phase 10-13 controls and grid-resident migration evidence exist.
