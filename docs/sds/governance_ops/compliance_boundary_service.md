SDS #76

# Compliance Boundary Service SDS

## Purpose

Keep payment, custody-like, privacy, child-safety, regulated-workload, jurisdiction, retention, deletion, payout, and public-reporting boundaries explicit and replayable.

Compliance Boundary Service is the compliance fact and boundary-definition layer for Overrid. It turns jurisdiction, data-class, workload-class, accounting, vault, tenant, payout, and reporting constraints into versioned boundary records that Overguard and other services can consume. It does not give legal advice, certify external compliance, process payments, hold custody, store secrets, make final policy decisions, or mutate billing, ledger, vault, tenant, payout, or incident records.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [compliance_boundary_service.md](../../service_catalog/governance_ops/compliance_boundary_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |

## Service Family

- Family: Governance, compliance, and operations
- Owning layer: Compliance boundary facts, regulated-scope markers, export packages, and policy-input contracts
- Primary data scope: jurisdiction records, boundary rulesets, payment/custody/payout markers, privacy and child-safety markers, regulated-workload markers, retention/deletion constraints, data-residency constraints, boundary evaluations, compliance export refs, signed exceptions, and update histories
- First build phase from service plan: [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md)

## Problem Statement

Overrid intentionally mixes infrastructure, native apps, AI, accounting, public providers, grants, and stewardship. Some flows are low-risk utility usage; others cross payment, payout, custody-like, privacy, child-safety, regulated workload, data residency, deletion, retention, or public-reporting boundaries. If those boundaries are hidden in service code, the platform will either over-isolate harmless flows or let high-compliance work contaminate the rest of the system.

The service must provide an explicit boundary map that Overguard, Overbill, Overvault, Overtenant, Overwatch, Central AI, native apps, and governance reports can cite. The design fix is to make compliance facts versioned, specific, and auditable while keeping final policy enforcement in Overguard and domain truth in the owning services.

## Goals

- Define versioned boundary rulesets for jurisdiction, payment, custody-like behavior, payouts, refunds/disputes, privacy, child safety, regulated workloads, data residency, retention, deletion, and public reporting.
- Produce signed compliance fact bundles that Overguard can evaluate before admission, payout, export, model context use, public-provider placement, and native-app publication.
- Mark high-compliance workloads and data classes so they can be isolated without forcing every low-risk flow into the same operating mode.
- Provide evidence-backed export packages for auditors, stewards, incident responders, and public reports.
- Track jurisdiction and policy changes with effective windows, supersession, migration notes, and rollback or remediation refs.
- Support signed exceptions where policy allows them, with expiry, evidence, approver refs, and public/private reporting boundaries.
- Keep compliance decisions replayable from ruleset version, input facts, and stored boundary evaluations.

## Non-Goals

- Do not provide legal advice, become a law firm, or claim external regulatory certification.
- Do not process payments, custody assets, hold user funds, issue refunds, approve payouts, or mutate ORU, Seal Ledger, Overbill, Overgrant, or Provider Payout records.
- Do not enforce policy directly; Overguard consumes compliance facts and produces admission or denial decisions.
- Do not store raw secrets, payment credentials, identity documents, private user content, encrypted Docdex context, or raw child-safety evidence.
- Do not replace Overtenant identity/tenant truth, Overvault secret policy, Overclaim dispute records, Incident Response cases, or Stewardship Reporting publications.
- Do not add pricing forecasts, customer counts, revenue projections, blockchain mechanics, NFT mechanics, or speculative economics.

## Primary Actors And Clients

- Overguard, requesting compliance facts for admission, dry-run, payout, public-provider, native-app, and export decisions.
- Overbill, ORU Account Service, Seal Ledger, Provider Payout Service, and Overgrant, supplying accounting refs and consuming payment/custody/payout boundary markers.
- Overvault and Encrypted Docdex RAG Adapter, supplying secret, encrypted-context, and private-data class refs.
- Overtenant, Overpass, and Universal Namespace Service, supplying tenant, identity, namespace, region, and ownership facts.
- Native apps, AI Gateway Router, Central AI Service, and Personal AI Assistant, requesting data-use, public-reporting, and AI-context boundary checks.
- Overwatch, Incident Response Service, Stewardship Reporting Service, PIP Registry, and auditors consuming compliance evidence and export refs.
- Authorized compliance stewards and operators defining rulesets, reviewing exceptions, and validating reports.

## Dependencies

- [Overguard](../trust_policy_verification/overguard.md) for policy evaluation, reason codes, replay, rollout, and deny-by-default behavior.
- [Overbill](../accounting/overbill.md), [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), [Provider Payout Service](../accounting/provider_payout_service.md), and [Overgrant](../accounting/overgrant.md) for accounting, payment, custody-like, payout, grant, and refund/dispute refs.
- [Overvault](../data_storage_namespace/overvault.md), [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), and [Encrypted Docdex RAG Adapter](../ai_rag_model_routing/encrypted_docdex_rag_adapter.md) for secret, private-data, encrypted-index, retention, deletion, and export refs.
- [Overtenant](../control_plane/overtenant.md), [Overpass](../control_plane/overpass.md), and [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) for tenant, actor, region, ownership, and namespace facts.
- [Overwatch](../control_plane/overwatch.md) for audit events, evidence bundles, export integrity, and compliance timelines.
- [Overclaim](../trust_policy_verification/overclaim.md), [Incident Response Service](incident_response_service.md), and [Stewardship Reporting Service](stewardship_reporting_service.md) for disputes, incidents, public reports, corrections, and publication boundaries.

## Owned Responsibilities

Compliance Boundary Service owns:

- Boundary ruleset records, schema versions, effective windows, supersession history, and rollout notes.
- Boundary marker taxonomy for payment, custody-like, payout, refund/dispute, privacy, child safety, regulated workload, data residency, retention, deletion, AI context, public report, and export classes.
- Boundary evaluation records that bind explicit input facts to ruleset versions and result refs.
- Compliance fact bundles supplied to Overguard, Stewardship Reporting, Central AI, Overbill, Provider Payout, native apps, and auditors.
- Signed exception records, exception expiry, exception revocation, and required evidence/approval refs.
- Jurisdiction update records and migration notes for affected services, tenants, apps, or workload classes.
- Compliance export jobs and redacted evidence packages.
- Replay bundles proving which boundary rules and input facts produced a result.

The service does not own legal outcomes, policy enforcement, billing truth, ledger truth, vault contents, tenant identity, or incident finality.

## Data Model

- `boundary_ruleset`: ruleset id, domain, semantic version, jurisdiction refs, effective window, scope selectors, signer refs, rollout state, superseded-by refs, compatibility notes, and audit refs.
- `boundary_marker`: stable marker code, domain, severity, public/private visibility, required input facts, affected services, default action hint, redaction class, and deprecation state.
- `jurisdiction_profile`: jurisdiction id, supported service domains, data-residency constraints, payment/payout constraints, retention/deletion constraints, regulated workload markers, and source/evidence refs.
- `regulated_scope`: workload class, data class, tenant/app/native-service refs, region refs, model-context refs, secret refs, reporting class, and isolation requirements.
- `boundary_evaluation`: request id, actor/service account, target refs, input fact bundle refs, ruleset version, matched markers, result state, reason codes, policy handoff refs, and replay bundle ref.
- `compliance_fact_bundle`: signed facts exported to Overguard or another service, with fact owners, marker refs, freshness window, trust level, and allowed use.
- `exception_record`: requested exception, target refs, markers being waived or narrowed, evidence refs, approver refs, expiry, revocation refs, reporting class, and resulting evaluation refs.
- `jurisdiction_update`: changed profile refs, affected markers, affected tenants/services/apps, migration notes, effective-at, rollback notes, notification refs, and acceptance evidence.
- `compliance_export_job`: export purpose, audience, included fact bundles, redaction profile, evidence refs, artifact refs, integrity hash, expiry, and publication refs.
- `compliance_replay_bundle`: ruleset, input facts, marker matches, exception refs, generated fact bundle, Overguard decision refs, event refs, and export refs.

Common envelope fields: `id`, `tenant_id` or `system_scope`, `actor_id` or `service_account_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

- `POST /compliance/rulesets`: create or revise a boundary ruleset draft with signed source and evidence refs.
- `POST /compliance/rulesets/{ruleset_id}/publish`: publish, stage, activate, supersede, pause, or revoke a ruleset version.
- `GET /compliance/rulesets/{ruleset_id}`: read authorized ruleset metadata, marker taxonomy, effective windows, and rollout state.
- `POST /compliance/evaluations`: evaluate target refs and explicit input fact bundles against active or staged rulesets.
- `POST /compliance/fact-bundles`: create a signed fact bundle for Overguard, Stewardship Reporting, Central AI, accounting, or export use.
- `GET /compliance/markers`: list stable marker definitions and data-class-safe explanations.
- `POST /compliance/exceptions`: request a signed exception with evidence, expiry, and target refs.
- `POST /compliance/exceptions/{exception_id}/review`: approve, reject, revoke, or expire an exception according to policy.
- `POST /compliance/jurisdiction-updates`: record jurisdiction/profile changes and affected service refs.
- `POST /compliance/exports`: generate an export package with declared audience and redaction profile.
- `GET /compliance/replay/{evaluation_or_export_id}`: reconstruct boundary input facts, ruleset version, markers, exception refs, and outputs.

Mutating APIs require actor or service identity, role refs, trace id, idempotency key, policy refs, and Overwatch audit refs. Stable errors include `boundary_scope_missing`, `ruleset_not_active`, `ruleset_signature_invalid`, `input_fact_stale`, `marker_unknown`, `exception_not_allowed`, `jurisdiction_profile_missing`, `redaction_required`, and `export_audience_denied`.

## Event Surface

- `compliance_boundary.ruleset_drafted`: ruleset draft created.
- `compliance_boundary.ruleset_published`: ruleset version published, staged, activated, paused, superseded, or revoked.
- `compliance_boundary.marker_versioned`: marker taxonomy changed.
- `compliance_boundary.evaluation_requested`: evaluation accepted for processing.
- `compliance_boundary.evaluation_completed`: evaluation produced marker and fact refs.
- `compliance_boundary.evaluation_denied`: evaluation could not proceed due to stale facts, scope, or policy.
- `compliance_boundary.fact_bundle_created`: signed fact bundle created for a consumer.
- `compliance_boundary.exception_requested`: exception request opened.
- `compliance_boundary.exception_reviewed`: exception approved, rejected, revoked, or expired.
- `compliance_boundary.jurisdiction_update_recorded`: jurisdiction profile changed with affected refs.
- `compliance_boundary.export_created`: redacted compliance export generated.
- `compliance_boundary.usage_emitted`: usage emitted for evaluation, export, review, or replay work.

Events include ruleset ids, marker refs, target refs, audience/redaction class, policy refs, evidence refs, trace id, and audit refs. Events must not include raw private data, secret material, identity documents, payment credentials, or unredacted child-safety evidence.

## Core Workflow

1. Steward or authorized service drafts a boundary ruleset with domain, markers, jurisdiction refs, evidence refs, and rollout plan.
2. Ruleset passes schema, signature, compatibility, and redaction checks before publication.
3. Overguard, accounting, vault, AI, native app, reporting, or incident workflows submit explicit input facts for boundary evaluation.
4. Compliance Boundary Service matches ruleset and marker versions, applies allowed exceptions, and emits a signed fact bundle.
5. Consuming service uses the fact bundle through its own authority: Overguard enforces policy, Overbill/Provider Payout handles payment flow, Overvault controls secrets, and Stewardship Reporting publishes redacted summaries.
6. Jurisdiction or policy changes create update records, impacted-scope lists, notification refs, migration notes, and optional staged evaluations.
7. Auditors or stewards request export packages assembled from fact bundles, Overwatch evidence, redaction profiles, and integrity refs.
8. Replay reconstructs rules, input facts, exceptions, and outputs for disputes, incidents, compliance review, or public reports.

## State Machine

Ruleset lifecycle:

1. `draft`
2. `schema_checked`
3. `review_pending`
4. `staged`
5. `active`
6. `paused`
7. `superseded`
8. `revoked`
9. `retired`

Boundary evaluation lifecycle:

1. `submitted`
2. `facts_validated`
3. `ruleset_selected`
4. `markers_matched`
5. `exception_checked`
6. `fact_bundle_created`
7. `completed`
8. `denied`
9. `expired`

Exception lifecycle:

1. `requested`
2. `evidence_review`
3. `approved`
4. `rejected`
5. `active`
6. `revoked`
7. `expired`
8. `superseded`

Export lifecycle:

1. `requested`
2. `scope_checked`
3. `evidence_collecting`
4. `redaction_review`
5. `ready`
6. `delivered`
7. `failed`
8. `expired`

Rulesets and evaluations are append-only after publication. Corrections, exceptions, and jurisdiction changes create linked records rather than editing prior conclusions.

## Policy And Security

- Deny or block evaluation when identity, tenant, jurisdiction, data class, workload class, or evidence refs are missing.
- Treat boundary facts as policy inputs; the service must not bypass Overguard to allow or deny execution.
- Keep high-compliance markers narrow and specific so low-risk utility flows are not overconstrained.
- Require signed steward/operator action for ruleset activation, exception approval, jurisdiction-profile change, and export release.
- Store sensitive evidence as refs through Overwatch, Overvault, Overbase, or Overstore according to data class.
- Export jobs must declare audience, purpose, redaction profile, expiry, and integrity refs before artifact creation.
- Public reports may use aggregate or redacted facts only.
- Exception records require expiry and must be easy to revoke.
- Replays use stored ruleset versions and stored fact refs, not the current live policy state.

## Metering And Accounting

- Emit usage refs for evaluations, fact bundle creation, export assembly, replay, stewardship review, and jurisdiction-update processing.
- Link usage to service account, tenant/system scope, ruleset, marker domain, evaluation id, export id, and Overwatch trace.
- Accounting services remain authoritative for usage, balances, receipts, payout batches, grants, and ledger entries.
- Boundary records may mark a flow as payment, custody-like, payout, refund, grant, or public-interest related; they do not calculate charges or mutate balances.
- Keep native-service economics structural and near-cost without forecasts or customer-count assumptions.

## Observability And Operations

- Expose ruleset rollout state, evaluation latency, denied evaluation counts, stale fact counts, exception volume, export backlog, jurisdiction updates, and impacted-scope counts.
- Alert on ruleset signature failure, sudden marker spikes, high-compliance workload without isolation marker, export without redaction review, active exception past expiry, and missing Overguard consumption.
- Provide operator diagnostics for target refs, matched markers, policy handoffs, evidence refs, export artifacts, and replay outcomes.
- Provide dry-run comparison for staged rulesets against recorded evaluations.
- Keep compatibility fixtures for every marker family and jurisdiction profile.

## Failure Modes And Recovery

- Missing or stale input facts: deny or block evaluation with required fact refs.
- Ruleset conflict: use stricter marker or require review according to domain policy.
- Jurisdiction update invalidates prior boundary: create affected-scope list and require reevaluation or migration notes.
- Exception expiry missed: revoke on next evaluation and alert operators.
- Export redaction failure: keep export blocked and record failed redaction refs.
- Consuming service unavailable: create fact bundle and retry handoff through idempotent request refs.
- Replay mismatch: mark compliance integrity incident and preserve evaluator version evidence.
- Accidental sensitive-data capture: restrict query, create redaction marker, open incident, and fix the emitting path.

## Validation Plan

- Regulated workloads are isolated by policy using Compliance Boundary fact bundles consumed by Overguard.
- Payment, custody-like, payout, refund, and dispute boundaries are explicit in accounting and billing flows.
- Compliance exports cite stored evidence, redaction profile, ruleset version, and integrity refs.
- Boundary evaluations are deterministic from stored input facts and ruleset versions.
- Ruleset activation, exception approval, jurisdiction updates, and export release require signed steward/operator action.
- Public reports cannot expose raw private data, payment credentials, child-safety evidence, secret refs, or encrypted Docdex context.
- Low-risk flows without high-compliance markers remain possible under normal policy.
- Jurisdiction updates produce affected-scope lists, reevaluation refs, and migration notes.

## Build Breakdown

1. Define marker taxonomy, ruleset, jurisdiction profile, evaluation, fact bundle, exception, update, export, and replay schemas.
2. Implement ruleset draft, validation, publication, staged rollout, and marker registry APIs.
3. Implement boundary evaluation and signed fact bundle creation for Overguard.
4. Add accounting boundary markers for payment, custody-like behavior, payouts, refunds, disputes, grants, and public-interest reports.
5. Add privacy, child-safety, data-residency, retention, deletion, and encrypted-context markers.
6. Add exception review, expiry, revocation, and jurisdiction update workflows.
7. Add redacted export packages, replay APIs, Overwatch evidence, and Stewardship Reporting handoff.
8. Add staged-ruleset dry-run comparison, compatibility fixtures, and Phase 13 governance validation.

## Handoff And Downstream Use

Compliance Boundary Service hands signed boundary facts to Overguard, Overbill, Provider Payout Service, Overvault, Overtenant, Central AI Service, native apps, Incident Response Service, Stewardship Reporting Service, PIP Registry, and auditors.

Downstream services must consume boundary facts through documented APIs and policy decisions. They must not read private compliance storage or treat compliance markers as permission to mutate accounting, secrets, identity, reports, or incident records.

## Open Design Questions

Resolved decisions:

- Legal-steward review is required before activation for marker families that can change external legal/compliance posture: payment processing, custody-like behavior, provider payouts, refunds, chargebacks, tax/compliance metadata, privacy, child safety, regulated workloads, data residency, retention, deletion, public reporting, auditor/legal exports, and any exception that narrows or waives those markers. Service owners may steward operational marker families inside an already active boundary domain, such as source freshness, stale input facts, redaction-profile missing, service-specific compatibility, rollout/canary state, low-risk public/utility classification, and consumer handoff failures. Service-owned markers still need stable codes, versioned schemas, Overwatch audit refs, and Overguard consumption; they cannot grant permission where a legal-steward marker is missing, stale, denied, or review-required.
- The minimum jurisdiction profile for first public native apps and provider payouts must include a jurisdiction id, effective window, source/evidence refs, supported service domains, data-class allow/deny rules, data-residency constraints, user deletion and retention defaults, child-safety/public-content obligations where applicable, public-reporting redaction class, payment/refund/chargeback constraints, provider-payout constraints, dispute/hold windows, and required owning-service refs. Public native apps may proceed only in jurisdictions whose profiles cover public content, account/wallet visibility, privacy, deletion, retention, AI-context use, and reporting redaction. Provider payouts additionally require Overbill/Provider Payout/Overvault-compatible refs for payout eligibility, tokenized payment destination or payment-provider refs, tax/compliance metadata refs, provider region, dispute/fraud/anti-Sybil hold requirements, and allowed payout state. Missing profile facts should block or hold the relevant public native-app or payout flow rather than defaulting to a permissive mode.
- Public boundary facts should be limited to marker taxonomy descriptions, active ruleset ids and versions, effective windows, supported jurisdiction/domain summaries, public low-sensitivity allow/deny categories, redaction classes, public-report templates, aggregate compliance/reporting metrics, and public correction/supersession notices. Affected parties may see their own evaluation ids, matched marker summaries, user-safe reason codes, action requirements, hold/deletion/retention status, appeal or dispute refs, and exported artifacts allowed for their role. Stewards, auditors, and authorized operators may see private evidence refs, ruleset source refs, exception records, input-fact ownership, replay/export bundles, reviewer notes, and redaction decisions according to audience policy. Raw secrets, payment credentials, identity documents, private user content, encrypted Docdex context, child-safety evidence, fraud heuristics, and exploit details must remain in the owning services and appear in Compliance Boundary only as redacted refs with access policy.
- Staged rulesets should compare old and new decisions by replaying stored boundary evaluations and explicit input fact bundles through the staged ruleset in dry-run mode, then writing a redacted comparison record. The comparison output should include ruleset versions, marker-code deltas, decision-state deltas, reason-code deltas, affected-service or affected-scope counts, representative redacted refs, and migration/blocker notes. It must not copy raw private facts into the comparison. Private detail remains behind Overwatch, Overvault, Overbase, or Overstore refs and is visible only through authorized replay. Public and affected-party comparison views should use aggregate counts, self-scoped refs, thresholds, and stable reason codes so staged rollout can be audited without leaking tenant, provider, child-safety, fraud, security, payment, or private-content data.
- Founder-hardware bootstrap may allow only narrow, expiry-bound exceptions for private seed operations: staged ruleset testing, temporary non-sensitive source-freshness gaps, service-owner operational markers, local development/test exports, private-swarm migration windows, and break-glass continuity actions that preserve Overguard, Overwatch, Overbill, Provider Payout, Overvault, and owning-service authority. These exceptions require signed actor refs, evidence refs, expiry, affected scope, revocation behavior, and post-action review. Exceptions that must wait for full Phase 13 governance include public-provider payout waivers, custody-like or external payment boundary waivers, child-safety or regulated-workload waivers, reductions to deletion/retention/data-residency obligations, public-report redaction waivers, Overvault emergency access to user/private content beyond approved break-glass policy, exceptions allowing public nodes to run private/regulated/secret-bearing/system-service workloads, and any long-lived exception that would weaken audit, replay, or evidence requirements.
