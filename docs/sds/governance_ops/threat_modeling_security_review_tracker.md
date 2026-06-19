SDS #81

# Threat Modeling and Security Review Tracker SDS

## Purpose

Turn threat modeling and security review into tracked remediation rather than static notes.

Threat Modeling and Security Review Tracker is the governance and security-review record system for Overrid. It records threat models, assets, trust boundaries, review assignments, security findings, mitigations, accepted risks, verification evidence, and reporting handoffs. It does not scan systems, run incident response, execute containment, block releases by itself, or decide policy alone. It creates the evidence and state that Release Strategy, PIP Registry, Incident Response, Overguard, Overwatch, and stewardship reporting can use.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [threat_modeling_security_review_tracker.md](../../service_catalog/governance_ops/threat_modeling_security_review_tracker.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |

## Service Family

- Family: Governance, compliance, and operations
- Owning layer: Security review records, threat-model lifecycle, remediation tracking, risk acceptance, verification evidence, and security-report handoff
- Primary data scope: threat models, reviewed assets, trust boundaries, review records, findings, mitigation plans, accepted risks, test/evidence refs, owners, due windows, release/PIP/incident links, and replay bundles
- First build phase from service plan: [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md)

## Problem Statement

Overrid will operate identity, scheduling, storage, accounting, AI routing, public-provider capacity, native apps, mobile access, and governance systems in one ecosystem. Security review cannot live as scattered documents because risks then lose owners, mitigations, tests, expiry dates, and release impact.

Phase 13 explicitly requires threat models for identity takeover, tenant escape, node fraud, scheduler manipulation, public-node leakage, ledger manipulation, native-app abuse, central AI abuse, namespace hijack, and supply-chain compromise. Every threat must produce a mitigation, a test, a monitoring requirement, or a recorded accepted risk. This tracker turns that requirement into durable workflow.

## Goals

- Maintain a versioned threat-model record for each service, native app, adapter, mobile component, protocol change, and major deployment pattern.
- Record assets, trust boundaries, actors, assumptions, threats, mitigations, tests, monitoring refs, and risk decisions as structured data.
- Coordinate security reviews with owners, reviewers, due windows, severity, status, verification refs, and release or PIP links.
- Link findings to implementation tasks, PIP records, incidents, release gates, migration plans, compliance facts, and public/private stewardship reports.
- Require evidence for remediation, verification, waiver, and accepted-risk decisions.
- Preserve replayable review history without exposing exploit details or private evidence to broad audiences.
- Give central AI and operators structured evidence for fraud, abuse, security, and governance review without making opaque enforcement decisions.

## Non-Goals

- Do not run scanners, fuzzers, vulnerability testing infrastructure, SIEM pipelines, or package validation jobs directly.
- Do not replace Package Validator, Overwatch, Incident Response Service, PIP Registry, Release Strategy Service, Compliance Boundary Service, or Overguard.
- Do not directly block deployments, suspend tenants, rotate credentials, freeze payouts, patch services, or execute containment.
- Do not store raw secrets, private keys, exploit payloads, decrypted private content, or unredacted vulnerability details in broad records.
- Do not create token voting, blockchain governance, NFT security badges, market-based bug priorities, or speculative economic mechanics.
- Do not hide accepted risks; every acceptance needs authority, evidence, expiry, and review state.

## Primary Actors And Clients

- Security reviewers, protocol maintainers, service owners, native app owners, mobile platform maintainers, and adapter owners.
- PIP Registry linking proposed protocol or service changes to required security review.
- Release Strategy Service and Package Validator consuming review-gate state before rollout.
- Incident Response Service linking incidents to prior threats, missed mitigations, security findings, and corrective actions.
- Overwatch providing event evidence, trace refs, health records, and audit refs.
- Overguard and Compliance Boundary Service consuming review facts where policy or compliance rules need security evidence.
- Central AI Service and Central AI Stewardship Interface reading redacted evidence for bounded recommendations and public reporting.
- Stewardship Reporting Service publishing aggregate and redacted security posture reports.

## Dependencies

- [Overwatch](../control_plane/overwatch.md) for audit refs, trace evidence, health signals, event bundles, and append-only review actions.
- [Protocol Improvement Proposal Registry](pip_registry.md) for PIP security-impact sections, proposal-review links, accepted decisions, implementation refs, and rollback refs.
- [Incident Response Service](incident_response_service.md) for incidents, containment evidence, post-incident follow-up, and recurrence-prevention refs.
- [Release Strategy Service](../deployment_grid/release_strategy_service.md) for release gates, freeze decisions, rollout approvals, and rollback requirements.
- [Package Validator](../deployment_grid/package_validator.md) for package validation reports, manifest checks, supply-chain findings, and validation evidence.
- [Compliance Boundary Service](compliance_boundary_service.md) for compliance markers, jurisdiction facts, regulated data classes, and export requirements.
- [Overguard](../trust_policy_verification/overguard.md) for policy decisions when a review gate affects execution, deployment, data access, or administrative authority.
- [Stewardship Reporting Service](stewardship_reporting_service.md) for redacted public/private security posture reporting.

## Owned Responsibilities

Threat Modeling and Security Review Tracker owns:

- Threat-model records for services, packages, native apps, adapters, mobile components, system-service workloads, and protocol proposals.
- Asset, actor, trust-boundary, data-flow, assumption, and threat records inside a threat model.
- Security-review records, review assignments, reviewer signoff, review scope, and review-gate state.
- Finding records with severity, affected component, owner, status, due window, evidence refs, and downstream work refs.
- Mitigation and verification records that tie findings to tests, monitoring, policy changes, release checks, incidents, or accepted risks.
- Accepted-risk records with authority, reason, evidence, expiry, review cadence, and reopening triggers.
- Redaction profiles for internal, steward, auditor, affected-party, and public views.
- Replay bundles that reconstruct a threat model, review decision, accepted risk, or finding lifecycle from stored refs and policy versions.

This service records and coordinates security review. Owning services remain responsible for implementation changes, runtime enforcement, rollout, incident response, and policy execution.

## Data Model

- `threat_model`: model id, subject type, subject refs, owning team, scope, version, status, review cadence, linked PIPs/releases/incidents, data classes, created-by, and audit refs.
- `security_asset`: asset id, asset type, owner, service/app refs, data class, trust level, exposure class, criticality, dependency refs, and assumptions.
- `trust_boundary`: source refs, target refs, boundary type, authentication and authorization expectations, encryption expectations, network/storage/context boundary, and known exceptions.
- `data_flow`: source, destination, data classes, transport, storage refs, vault refs, retention class, transformation summary, and leakage concerns.
- `threat_record`: threat id, category, attacker model, affected assets, scenario summary, likelihood class, impact class, confidence, linked evidence, and review status.
- `security_review`: review id, model refs, review type, reviewers, owner, phase gate, scope, due window, state, outcome, blocked reasons, and signoff refs.
- `security_finding`: finding id, review ref, affected subject refs, severity, status, owner, due window, exploitability class, evidence refs, redaction class, and downstream work refs.
- `mitigation_plan`: finding refs, mitigation type, owning service, expected control, test refs, monitoring refs, policy refs, implementation refs, target release, and fallback.
- `verification_record`: verifier, method, test run refs, package validation refs, audit refs, result, residual risk, and verification time.
- `accepted_risk`: finding/threat refs, accepting authority, reason, compensating controls, expiry, review window, affected scope, public/private report class, and reopening triggers.
- `review_gate`: subject ref, gate type, required review refs, finding threshold, accepted-risk requirements, current gate state, release refs, and policy refs.
- `security_replay_bundle`: threat model, review, findings, mitigations, accepted risks, policy refs, audit refs, and redacted evidence map.

Common envelope fields: `id`, `tenant_id` or `system_scope`, `actor_id` or `service_account_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

- `POST /threat-models`: create a draft threat model for a service, native app, adapter, mobile component, system-service workload, package, deployment pattern, or PIP.
- `GET /threat-models/{model_id}`: read the authorized model view with assets, boundaries, threats, mitigations, findings, accepted risks, and review state.
- `POST /threat-models/{model_id}/assets`: add or correct security assets.
- `POST /threat-models/{model_id}/boundaries`: add trust boundaries and data-flow refs.
- `POST /threat-models/{model_id}/threats`: add, update, close, or supersede threat records.
- `POST /security-reviews`: start a security review linked to a threat model, PIP, release, incident, package, or migration.
- `POST /security-reviews/{review_id}/findings`: record a security finding with owner, severity, evidence refs, and downstream links.
- `POST /findings/{finding_id}/mitigations`: attach mitigation plans, implementation refs, tests, monitoring refs, or policy refs.
- `POST /findings/{finding_id}/verification`: record remediation verification and residual-risk outcome.
- `POST /findings/{finding_id}/accept-risk`: accept residual risk with authority, reason, expiry, and compensating controls.
- `POST /accepted-risks/{risk_id}/review`: refresh, expire, revoke, or supersede an accepted risk.
- `POST /review-gates/{subject_ref}/evaluate`: return the current security gate state for release, package, PIP, migration, or app publication.
- `GET /security-reviews/{review_id}/replay`: reconstruct review decisions from model, finding, mitigation, accepted-risk, policy, and audit refs.
- `GET /security-reports/{subject_ref}`: return redacted review posture for stewardship reporting or authorized operators.

Mutating APIs require signed identity, review role authority, trace id, idempotency key, policy refs where gate state affects other services, and Overwatch audit refs. Stable errors include `scope_required`, `boundary_required`, `owner_required`, `severity_required`, `evidence_required`, `mitigation_required`, `verification_required`, `risk_acceptance_authority_required`, `risk_expired`, `review_gate_blocked`, and `redaction_required`.

## Event Surface

- `security_tracker.threat_model_created`: threat model draft opened.
- `security_tracker.threat_model_scoped`: assets, boundaries, and data flows are complete enough for review.
- `security_tracker.threat_recorded`: threat added, updated, superseded, or closed.
- `security_tracker.review_started`: security review opened with owner and reviewer assignments.
- `security_tracker.finding_recorded`: security finding added with severity, owner, evidence, and due window.
- `security_tracker.finding_triaged`: finding severity, owner, due window, or downstream link changed.
- `security_tracker.mitigation_planned`: mitigation, test, monitoring, policy, or implementation refs attached.
- `security_tracker.remediation_verified`: finding verification recorded.
- `security_tracker.risk_accepted`: accepted-risk record created with expiry and authority.
- `security_tracker.risk_expired`: accepted risk expired or was revoked.
- `security_tracker.review_gate_changed`: release/PIP/package/migration gate state changed.
- `security_tracker.report_exported`: redacted report bundle emitted for stewardship, audit, or operations.
- `security_tracker.usage_emitted`: usage emitted for review, report generation, evidence packaging, or replay.

Events include model, review, finding, risk, subject, policy, audit, and evidence refs. Events must not include exploit payloads, raw secrets, vault values, private user data, decrypted Docdex context, or unredacted vulnerability details.

## Core Workflow

1. A service owner, PIP author, release owner, package validator, incident responder, or central AI workflow creates or links a threat model.
2. The owner defines reviewed assets, trust boundaries, data flows, assumptions, attacker models, and affected Overrid services.
3. Reviewers add threats, categorize risk, require mitigations/tests/monitoring, and open findings where work is needed.
4. Findings receive owners, severity, due windows, downstream refs, and review-gate impact.
5. Owning services implement mitigations or propose accepted risks through signed, evidence-backed workflow.
6. Reviewers verify remediation through tests, Package Validator reports, policy refs, incident evidence, or monitoring refs.
7. Review gates produce pass, fail, warning, or accepted-risk states for Release Strategy, PIP Registry, Package Validator, or Migration Tooling.
8. Stewardship Reporting consumes redacted posture summaries; Overwatch preserves audit evidence for replay.

## State Machine

Threat model lifecycle:

1. `draft`
2. `scoping`
3. `ready_for_review`
4. `in_review`
5. `changes_required`
6. `approved`
7. `approved_with_accepted_risk`
8. `superseded`
9. `archived`

Security review lifecycle:

1. `planned`
2. `assigned`
3. `in_progress`
4. `blocked`
5. `finding_remediation`
6. `verification`
7. `passed`
8. `failed`
9. `waived_with_risk`
10. `superseded`

Finding lifecycle:

1. `open`
2. `triaged`
3. `remediation_planned`
4. `remediation_in_progress`
5. `remediated`
6. `verification_pending`
7. `verified`
8. `accepted_risk`
9. `expired_risk`
10. `reopened`
11. `closed`

Review-gate lifecycle:

1. `not_required`
2. `required`
3. `waiting_for_review`
4. `blocked_by_findings`
5. `blocked_by_expired_risk`
6. `passed`
7. `passed_with_accepted_risk`
8. `waived_by_authority`
9. `superseded`

Corrections append new records linked to prior records. Security history is never silently rewritten.

## Policy And Security

- Require role-bound signed actions for model creation, review assignment, finding severity changes, verification, gate evaluation, risk acceptance, and report export.
- Store sensitive exploit or evidence material in owning evidence systems; tracker records refs, hashes, summaries, severity, and redaction class.
- Accepted risks require explicit authority, expiry, scope, compensating controls, and review cadence.
- Critical or severe findings that affect release, package, migration, central AI, identity, ledger, vault, or public-provider boundaries must surface as review-gate state for owning services.
- Public views must redact exploit steps, private user data, internal topology, fraud heuristics, security scanner details, and secret-bearing refs.
- Central AI recommendations must cite evidence refs and cannot silently turn into enforcement.
- Review waivers require signed authority and Overwatch evidence.
- Supply-chain findings should link Package Validator and Overregistry refs instead of duplicating package truth.

## Metering And Accounting

- Emit usage refs for threat-model creation, review work, evidence packaging, report export, replay, central AI analysis, and long-running review automation.
- Link usage to model id, review id, subject refs, actor role, tenant/system scope, service account, evidence package refs, and report refs.
- Keep security-review overhead visible as system-service work without putting pricing or market assumptions in SDS records.
- Do not mutate ORU balances, Seal Ledger entries, bills, payouts, or grants.
- Native-service and mobile-app security reviews should expose enough refs for Wallet/Usage Center and stewardship reports to explain platform overhead structurally.

## Observability And Operations

- Expose open findings by severity, owner, service, release, PIP, incident, due window, review state, and accepted-risk expiry.
- Track time to scope, review, remediate, verify, accept risk, expire risk, and close findings.
- Alert on severe findings without owners, risks nearing expiry, overdue verification, release gates blocked by findings, threat models stale past cadence, and reports awaiting redaction review.
- Provide review replay for auditors, operators, central AI, PIP reviewers, release owners, and incident responders.
- Support import of external review reports only through structured normalization and evidence refs.
- Preserve redaction profiles for internal, steward, auditor, affected-party, and public views.

## Failure Modes And Recovery

- Threat model submitted without assets or boundaries: keep `scoping` and deny review start until minimum fields exist.
- Finding without owner or severity: reject with stable reason code and no gate effect.
- Mitigation lacks test, monitoring, policy, or implementation refs: keep finding open and block verification.
- Accepted risk expires: move linked gate to blocked or warning state according to policy and emit expiry event.
- Reviewer conflict of interest: require reassignment or signed waiver with evidence.
- Evidence system unavailable: accept draft notes only, block verification and report export until refs reconcile.
- Release gate checked while review state is stale: return `waiting_for_review` or `blocked_by_expired_risk` rather than guessing.
- Redaction mistake: retract report export, append correction, and require privacy/security review before republication.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Security findings have owner and status.
- Accepted risks are explicit and reviewable.
- Mitigations link to tests or evidence.

Additional SDS-level validation:

- Threat-model contract tests cover assets, boundaries, threats, findings, mitigations, verification, accepted risk, gate evaluation, report export, and replay.
- Review-gate tests prove Release Strategy, PIP Registry, Package Validator, and Migration Tooling receive deterministic pass, blocked, warning, waiver, and superseded states.
- Accepted-risk tests cover required authority, expiry, compensating controls, renewal, revocation, and expired-risk reopening.
- Redaction tests prove public/steward/auditor views do not expose raw exploit details, secrets, private user data, or fraud heuristics.
- Replay tests reconstruct review outcome from threat model, findings, mitigations, risk records, policy refs, and Overwatch refs.
- Failure tests cover missing evidence, missing owner, stale model, unavailable evidence system, conflicting reviewer updates, and expired risk.

## Build Breakdown

1. Define threat model, asset, trust boundary, data flow, threat, review, finding, mitigation, verification, accepted-risk, review-gate, and replay schemas.
2. Implement threat-model creation, scoping, read, asset, boundary, data-flow, and threat APIs.
3. Implement security review, reviewer assignment, finding, triage, mitigation, and verification workflow.
4. Add accepted-risk lifecycle with authority checks, expiry, renewal, revocation, and reopening.
5. Integrate Overwatch, PIP Registry, Package Validator, Release Strategy, Incident Response, and Compliance Boundary refs.
6. Add review-gate evaluation APIs for releases, packages, migrations, PIPs, and native/mobile app publication.
7. Add redacted reporting, replay bundles, metrics, alerts, and stewardship-report handoff.

The first build should prioritize Phase 13 threat modeling and security review needs while keeping the record model reusable for earlier services as they harden.

## Handoff And Downstream Use

This tracker hands review-gate state, finding refs, accepted-risk refs, mitigation refs, verification refs, and redacted report bundles to PIP Registry, Release Strategy Service, Package Validator, Incident Response Service, Compliance Boundary Service, Stewardship Reporting Service, Central AI Service, Overwatch, admin UI, and service owners.

Downstream services must consume the tracker through its API and events. They should not read tracker storage or treat a missing security record as approval.

## Open Design Questions

Resolved decisions:

- Baseline threat models are required before Phase 13 for any service that becomes a production, system-service, public-provider, native-app, mobile, AI-routing, secret-bearing, identity, ledger, package, release, migration, or policy authority boundary. The first required set is Overrid Protocol Core, Shared Schema Package, Overgate, Overpass, Overtenant, Overkey, Overregistry, Overqueue, Overwatch, Overcell, Overpack, Oversched, Overlease, Overrun, Overmeter, Overmesh, Overguard, Policy Dry-Run API, Oververify, Overclaim, Challenge Task Service, Workload Classifier, ORU Account Service, Seal Ledger, Overbill, Provider Payout Service, Overgrant, Overasset, Overbase, Overstore, Overvault, Universal Namespace Service, Package Validator, Deployment Planner, Release Strategy Service, Backup and Restore Service, Failover and Recovery Coordinator, System-Service Workload Class, Migration Tooling, AI Gateway Router, Central AI Service, native apps, and Mobile Backend Gateway. Before Phase 13 these can start as scoped baseline records with assets, trust boundaries, top threats, owners, and review cadence; they become formal Phase 13 security reviews as release, public exposure, grid-resident migration, or high-compliance scope increases.
- The first implementation should turn severity into review-gate state deterministically. `critical` and `high` findings block production, system-service, public-provider, ledger, identity, vault, AI-enforcement, native-app publication, mobile gateway, package, migration, and protocol-release gates unless a verified mitigation or active accepted-risk record exists. `medium` findings are warnings for internal/private bootstrap paths, but they block high-compliance, secret-bearing, public-reporting, payout/finality, cross-tenant, or public-provider exposure until mitigated or explicitly accepted. `low` and `informational` findings are warnings with owners, due windows, and monitoring or test refs. Missing severity, missing owner, missing evidence, stale review cadence, expired accepted risk, or unresolved exploit-sensitive redaction always blocks the affected gate instead of being downgraded silently.
- Risk acceptance is role-bound and component-specific; Central AI, automation, package validation, release strategy, or the tracker itself may recommend but never accept risk alone. System-service backbone risks require the service owner, security reviewer, release/operations steward, and governance steward; identity, tenant, key, and vault risks require the owning service, security steward, privacy/compliance steward, and protocol steward; ledger, ORU, billing, payout, and grant risks require the owning service, accounting steward, security reviewer, and compliance/stewardship reviewer; AI-routing or Central AI risks require the AI owner, security/privacy reviewer, central-AI steward, and affected-domain owner; native-app and mobile risks require the app/mobile owner plus security, privacy/compliance, and stewardship review when public users are affected. Accepted risks must be scoped, expiry-bound, compensating-control-backed, replayable from Overwatch refs, visible to Release Strategy or PIP Registry where relevant, and reopened automatically when evidence, policy, taxonomy, release scope, or incident history changes.
- Exploit-sensitive material stays in owning evidence systems such as Overwatch, Overvault, Overstore, or service-local evidence stores; the tracker stores refs, hashes, redaction class, audience policy, and retention metadata. Private exploit-sensitive evidence refs for `critical` or `high` findings, identity, vault, ledger, public-provider, system-service, AI-enforcement, or mobile access boundaries are retained for at least seven years after closure and longer while incidents, PIPs, accepted risks, audits, disputes, compliance holds, or public reports depend on them. `medium` finding refs are retained for at least three years after closure, and `low` or informational review refs for at least eighteen months unless linked to a broader report or incident. Public redaction bundles and public report shells are retained indefinitely as append-only artifacts with correction, retraction, supersession, artifact hash, and safe reason-code history; private evidence maps behind those bundles follow the stricter linked evidence retention rule.
- Ship an Overrid-native threat taxonomy v1 covering identity and tenant escape, key and secret handling, policy or authorization bypass, package and supply-chain compromise, execution sandbox escape, scheduler and lease manipulation, node/provider fraud, public-node data leakage, Overmesh route or namespace hijack, Overbase/Overstore/Overvault data exposure, ledger/accounting/payout manipulation, Central AI or agentic enforcement abuse, native-app and mobile abuse, public-provider/federation abuse, compliance/reporting failure, incident/recovery failure, and governance/PIP process abuse. Taxonomy versions are append-only schema/catalog records with stable codes, aliases, deprecations, and migration notes; each threat model records the taxonomy version used. Taxonomy edits do not change policy by themselves. Release-blocking behavior lives in separately versioned Overguard/review-gate mappings, and a PIP is required only when a taxonomy change changes authority, gate behavior, compatibility, public reporting, or required review roles.
