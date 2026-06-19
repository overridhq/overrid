SDS #79

# Protocol Improvement Proposal Registry SDS

## Purpose

Make Overrid protocol, service-contract, governance, accounting, security, compliance, and migration changes evidence-backed, reviewable, implementable, migratable, reversible, and publicly traceable.

Protocol Improvement Proposal Registry is the durable registry for Overrid PIPs. It stores proposal records, versions, review evidence, decision refs, implementation links, migration plans, rollback plans, supersession records, deprecation records, and public-reporting refs. It does not implement protocol changes, run a token-voting system, mutate service state, replace security review, or turn governance into hidden admin decisions.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [pip_registry.md](../../service_catalog/governance_ops/pip_registry.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |

## Service Family

- Family: Governance, compliance, and operations
- Owning layer: Protocol and service-contract change governance, review records, implementation traceability, migration refs, rollback refs, and public proposal archive
- Primary data scope: PIP records, proposal versions, author/owner refs, motivation/specification/security/privacy/economic-impact/compatibility/migration/rollback sections, review assignments, review findings, acceptance/rejection decisions, implementation refs, rollout refs, deprecation/supersession refs, public report refs, and replay bundles
- First build phase from service plan: [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md)

## Problem Statement

Overrid must evolve across protocol core, schemas, service APIs, native apps, AI routing, accounting, governance, compliance, federation, public capacity, and mobile access. Without a formal PIP path, important changes will be hidden in code, agent edits, operator decisions, or ad hoc documents. That creates unreviewable central authority and breaks downstream builders.

The registry must make non-trivial protocol and platform changes explicit. A PIP should describe the change, why it exists, what it affects, which risks it creates, how it migrates, how it can be rolled back, and where implementation evidence lives.

## Goals

- Provide a stable PIP schema for motivation, specification, security impact, privacy impact, structural economic impact, compatibility, migration, rollout, rollback, alternatives, and evidence refs.
- Track proposal lifecycle from draft through review, accepted/rejected, implemented, superseded, deprecated, corrected, or rolled back.
- Require security, privacy, compliance, accounting, migration, and stewardship reviews when affected domains demand them.
- Link accepted PIPs to protocol versions, schema versions, service SDS updates, service catalog updates, implementation refs, migration plans, rollout refs, tests, and public reports.
- Make public-readable proposal summaries available without exposing private security, fraud, or personal data.
- Support corrections, retractions, deprecations, and supersession instead of deleting governance history.
- Keep PIPs evidence-backed and reviewable without blockchain, token voting, or hidden private governance.

## Non-Goals

- Do not implement code, deploy releases, migrate services, or mutate protocol/service state directly.
- Do not replace Overrid Protocol Core, service SDS files, service catalog plans, Release Strategy Service, Migration Tooling, Security Review Tracker, Compliance Boundary Service, or Stewardship Reporting.
- Do not run speculative voting, token governance, DAO mechanics, blockchain governance, NFT ownership, or pay-to-influence mechanisms.
- Do not expose embargoed vulnerability details, private user data, fraud heuristics, raw AI context, or sensitive compliance evidence in public PIP views.
- Do not treat acceptance as deployment. Owning implementation and release services must still validate and execute changes.
- Do not add revenue projections, pricing assumptions, customer-count math, or market claims.

## Primary Actors And Clients

- Authors proposing protocol, schema, service contract, native app, AI routing, accounting, governance, compliance, federation, mobile, or migration changes.
- Stewards, maintainers, security reviewers, privacy reviewers, compliance reviewers, accounting reviewers, migration owners, and affected service owners.
- Overrid Protocol Core, Shared Schema Package, service SDS files, service catalog, build-plan crosswalk, and implementation workstreams linking to accepted PIPs.
- Threat Modeling and Security Review Tracker, Compliance Boundary Service, Incident Response Service, Migration Tooling, Release Strategy Service, and Stewardship Reporting Service.
- Central AI Service and Central AI Stewardship Interface producing evidence summaries, review recommendations, and public views.
- SDK, CLI, admin/developer UI, native apps, and external builders reading accepted changes and migration guidance.
- Public users reading published PIP summaries and change history.

## Dependencies

- [Overrid Protocol Core](../control_plane/overrid_protocol_core.md) for protocol versioning, compatibility rules, conformance fixtures, and protocol change refs.
- [Shared Schema Package](../foundation/shared_schema_package.md), service SDS files, service catalog files, and build-plan crosswalk for contract and documentation updates.
- [Threat Modeling and Security Review Tracker](threat_modeling_security_review_tracker.md) for security reviews, threat model refs, findings, and remediation evidence.
- [Compliance Boundary Service](compliance_boundary_service.md) for regulated boundary changes, jurisdiction impacts, privacy classes, and public-reporting constraints.
- [Migration Tooling](migration_tooling.md) and [Release Strategy Service](../deployment_grid/release_strategy_service.md) for migration, rollout, rollback, and implementation evidence.
- [Overwatch](../control_plane/overwatch.md) for audit events, review evidence, publication events, and replay bundles.
- [Stewardship Reporting Service](stewardship_reporting_service.md) and [Central AI Service](../ai_rag_model_routing/central_ai_service.md) for public reports, evidence summaries, and stewardship review.

## Owned Responsibilities

Protocol Improvement Proposal Registry owns:

- PIP id allocation, record storage, version history, and public/private view metadata.
- Proposal section schema and completeness checks.
- Review state, reviewer assignments, comments, findings, waivers, and decision refs.
- Acceptance, rejection, withdrawal, supersession, deprecation, correction, and rollback records.
- Links from accepted PIPs to protocol/schema/service/document versions, implementation tasks, tests, migration plans, rollout refs, and report refs.
- Compatibility, migration, rollback, and deprecation metadata for builder consumption.
- Public proposal archive with redaction controls.
- Replay bundles for proposal review and acceptance decisions.

The registry records governance decisions and links; it does not implement them.

## Data Model

- `pip_record`: PIP id, title, summary, domain, author refs, steward refs, affected services, affected schemas, proposal type, visibility class, state, and audit refs.
- `pip_version`: version number, content hash, section refs, change summary, author signature, created-at, supersedes prior version, and review-reset flags.
- `pip_section`: section type, body ref, required/optional state, completeness status, evidence refs, redaction class, and reviewer notes.
- `review_assignment`: reviewer role, reviewer refs, required/optional flag, due window, conflict-of-interest refs, state, and outcome.
- `review_finding`: finding id, review type, severity, affected section, evidence refs, required change, remediation refs, waiver refs, and final status.
- `pip_decision`: decision type, decision body ref, deciding body refs, required review refs, vote/consensus record where applicable, reason codes, conditions, and effective state.
- `implementation_link`: source PIP, repo/task/doc/service refs, schema/protocol version refs, test refs, release refs, migration refs, and verification state.
- `migration_and_rollback_ref`: migration plan refs, rollout refs, rollback plan refs, fallback refs, compatibility window, and status.
- `supersession_record`: replaced PIP refs, replacement refs, reason, migration notes, deprecation window, and public notice refs.
- `public_pip_view`: redacted summary, publication state, audience, public URL/namespace ref, correction refs, and withdrawal refs.
- `pip_replay_bundle`: PIP versions, section hashes, review assignments, findings, decisions, implementation links, migration refs, publication refs, and Overwatch events.

Common envelope fields: `id`, `system_scope`, `actor_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

- `POST /pips`: create a draft PIP with title, domain, author refs, and initial sections.
- `GET /pips/{pip_id}`: read authorized PIP metadata, latest version, state, review status, and publication state.
- `POST /pips/{pip_id}/versions`: submit a new proposal version or correction.
- `POST /pips/{pip_id}/submit`: submit a draft into triage/review.
- `POST /pips/{pip_id}/reviews`: assign or submit review findings by role.
- `POST /pips/{pip_id}/decisions`: accept, reject, withdraw, return for changes, supersede, deprecate, or mark rollback-required.
- `POST /pips/{pip_id}/implementation-links`: attach implementation, schema, service SDS, service catalog, migration, release, test, or report refs.
- `POST /pips/{pip_id}/publish`: publish, correct, withhold, or retract public views.
- `GET /pips`: list public or authorized PIPs by state, domain, affected service, version, reviewer, or publication status.
- `GET /pips/{pip_id}/replay`: reconstruct proposal versions, reviews, decisions, links, and publication events.

Mutating APIs require signed identity, author/steward/reviewer role refs, trace id, idempotency key, and Overwatch audit refs. Stable errors include `pip_section_missing`, `review_required`, `security_review_required`, `migration_plan_required`, `rollback_plan_required`, `affected_service_missing`, `implementation_link_missing`, `publication_redaction_required`, and `decision_not_authorized`.

## Event Surface

- `pip_registry.pip_created`: draft PIP created.
- `pip_registry.version_submitted`: new proposal version submitted.
- `pip_registry.review_requested`: review assignment created.
- `pip_registry.review_submitted`: reviewer finding or approval submitted.
- `pip_registry.decision_recorded`: accept, reject, withdraw, supersede, deprecate, rollback, or correction decision recorded.
- `pip_registry.implementation_linked`: implementation, schema, SDS, migration, release, test, or report refs linked.
- `pip_registry.publication_changed`: public view published, corrected, withheld, or retracted.
- `pip_registry.supersession_recorded`: PIP supersession or deprecation recorded.
- `pip_registry.replay_completed`: replay bundle generated.
- `pip_registry.usage_emitted`: usage emitted for review, publication, replay, or reporting.

Events include PIP id, proposal version, affected domain, review refs, decision refs, implementation refs, migration refs, publication refs, trace id, and audit refs. Events must not expose embargoed security details, private user data, fraud internals, or restricted compliance evidence.

## Core Workflow

1. Author creates a draft PIP with motivation, specification, impact sections, compatibility notes, migration plan, and rollback plan.
2. Registry validates schema completeness and affected-service declarations.
3. Author submits the PIP into triage; stewards assign required reviews based on affected domains.
4. Reviewers submit security, privacy, compliance, accounting, migration, performance, implementation, and stewardship findings.
5. Author revises proposal versions until required findings are resolved, waived, or marked as accepted risk.
6. Decision body accepts, rejects, withdraws, returns, supersedes, deprecates, or requests rollback with reason codes and evidence.
7. Accepted PIP links to implementation changes, schema/protocol versions, SDS/catalog updates, migration plans, release plans, tests, and public reporting refs.
8. Public view is published with redaction profile and correction/retraction paths.
9. Later PIPs can supersede or deprecate prior PIPs without deleting history.

## State Machine

PIP lifecycle:

1. `draft`
2. `submitted`
3. `triage`
4. `review_required`
5. `security_review`
6. `privacy_review`
7. `compliance_review`
8. `migration_review`
9. `changes_requested`
10. `accepted`
11. `rejected`
12. `withdrawn`
13. `implemented`
14. `superseded`
15. `deprecated`
16. `rollback_required`
17. `rolled_back`

Review finding lifecycle:

1. `open`
2. `acknowledged`
3. `remediation_submitted`
4. `resolved`
5. `waived`
6. `accepted_risk`
7. `blocked`

Publication lifecycle:

1. `private_draft`
2. `redaction_review`
3. `published`
4. `corrected`
5. `withheld`
6. `retracted`

Proposal versions are immutable after submission. Revisions, decisions, implementation links, and public corrections are append-only linked records.

## Policy And Security

- Require complete motivation, specification, security, privacy, compatibility, migration, and rollback sections before review.
- Require domain-specific reviews for changes touching identity, tenancy, policy, accounting, ledger, payouts, vault, AI context, public providers, native apps, compliance, mobile access, or backbone operations.
- Embargoed security details stay private until public release policy allows disclosure.
- Public views must redact private data, fraud heuristics, exploit details, sensitive topology, payment details, and regulated evidence.
- Conflict-of-interest markers must be visible to reviewers and public summaries where appropriate.
- Accepted PIPs must link to implementation, test, migration, rollback, and documentation refs before being marked implemented.
- Emergency changes can be accepted with shortened review only when incident or break-glass policy allows, and must receive retrospective review.
- Governance history is append-only; rejected or flawed PIPs remain visible according to redaction rules.

## Metering And Accounting

- Emit usage refs for proposal storage, review work, public publication, report generation, replay, and governance dashboards.
- Structural economic impact means resource, accounting, grant, payout, cost-visibility, and native-service surplus implications; it must not include pricing forecasts or revenue projections.
- Accounting-related PIPs must link to ORU, Seal Ledger, Overbill, Overgrant, Provider Payout, or Overmark impact refs without mutating those services.
- Review and publication overhead should be visible as stewardship/system-service usage.
- No pay-to-propose, token-vote, or paid influence mechanism belongs in this registry.

## Observability And Operations

- Expose PIPs by state, domain, affected service, reviewer, blocked finding, implementation status, migration status, rollback status, publication state, and age.
- Alert on accepted PIPs without implementation links, missing rollback plan, overdue required reviews, emergency PIPs without retrospective review, and implemented changes without public view where publication is required.
- Provide builder views for accepted changes, migration windows, compatibility windows, deprecations, and schema/service version refs.
- Provide stewardship views for conflict-of-interest, review coverage, security/compliance status, and public reporting status.
- Keep searchable public archive and redaction-aware private archive.

## Failure Modes And Recovery

- Missing required section: keep draft/submitted state blocked with actionable section refs.
- Required reviewer unavailable: reassign reviewer and preserve assignment history.
- Conflicting reviews: require steward decision or PIP revision with accepted-risk refs.
- Security embargo needed: split public summary from private finding refs.
- Accepted PIP lacks implementation evidence: block `implemented` state.
- Migration plan fails: link rollback or corrective PIP and update implementation status.
- Public view published with bad redaction: retract/correct public view, record incident if needed, and preserve original publication evidence.
- Duplicate proposal: link related PIPs or mark supersession instead of deleting records.

## Validation Plan

- Non-trivial protocol and service-contract changes require a PIP before acceptance.
- Accepted PIPs link to implementation refs, tests, schema/protocol versions, migration refs, rollback refs, and relevant SDS/catalog updates.
- Security, privacy, compliance, accounting, and migration reviews are required for affected domains.
- Public PIP views are redacted and do not expose private evidence or embargoed vulnerability detail.
- Rollback plans are explicit before acceptance for changes that can affect running systems.
- Deprecated or superseded PIPs remain traceable through public/private views.
- Replay reconstructs proposal versions, findings, decisions, implementation links, migration refs, and publication history.

## Build Breakdown

1. Define PIP, version, section, review assignment, finding, decision, implementation link, migration/rollback, supersession, publication, and replay schemas.
2. Implement PIP creation, version submission, completeness validation, and authorized reads.
3. Add triage, review assignment, review finding, and reviewer decision APIs.
4. Add accept/reject/withdraw/supersede/deprecate/rollback decision records.
5. Add implementation, schema, SDS, catalog, migration, release, test, and report links.
6. Add public/private publication, redaction review, correction, and retraction workflows.
7. Integrate Security Review Tracker, Compliance Boundary Service, Migration Tooling, Release Strategy Service, Stewardship Reporting, and Overwatch.
8. Add dashboards, public archive, replay, validation fixtures, and Phase 13 governance reporting.

## Handoff And Downstream Use

PIP Registry hands accepted change records, review findings, implementation links, migration refs, rollback refs, and publication refs to Overrid Protocol Core, Shared Schema Package, service SDS/catalog maintainers, Release Strategy Service, Migration Tooling, Security Review Tracker, Compliance Boundary Service, Stewardship Reporting Service, SDK, CLI, admin/developer UI, and public documentation.

Downstream services should cite PIP ids for non-trivial protocol or contract changes. They must not treat a PIP as deployed until owning implementation and release services return evidence refs.

## Open Design Questions

Resolved decisions:

- A PIP is required for any non-trivial protocol, service-contract, schema, state-machine, compatibility, authority-boundary, security, privacy, compliance, accounting, AI-governance, federation, mobile, public-reporting, migration, rollback, or public-builder-facing change. Small implementation issues, typo fixes, generated documentation refreshes, new tests/fixtures, clarifying text that does not change normative behavior, additive optional non-authority fields, and bug fixes that preserve accepted contracts may proceed without a full PIP, but they still need ordinary issue/task evidence, affected docs or schemas updated where relevant, and passing conformance checks. Before the Phase 13 registry is active, non-trivial changes are captured through explicit SDS/schema/build-plan updates; after activation, those same changes must route through a PIP with compatibility, migration, rollback, security, privacy, and affected-service sections.
- Required review roles are derived from affected domains, with conflict-of-interest markers and signed Overwatch evidence for every mandatory review. Baseline review requires the PIP steward, affected service owner, implementation/release owner, and protocol/schema owner where contracts change. Protocol/schema changes require Protocol Core and Shared Schema Package review; security, identity, tenancy, policy, vault, public-provider, package, or system-service changes require Security Review Tracker and Overguard/security review; privacy, retention, deletion, child-safety, jurisdiction, mobile, user-content, or public-reporting changes require Compliance Boundary and privacy/redaction review; ORU, Seal Ledger, Overbill, Overgrant, Overasset, payout, dispute, or surplus-routing changes require accounting/stewardship/legal review; migration, route, backup, restore, release, or founder-hardware changes require Migration Tooling, Release Strategy, Backup/Restore, Failover/Recovery, and owning-service review. Central AI, native-app, and stewardship-surface changes require Central AI Service, Central AI Stewardship Interface, and Stewardship Reporting review where their evidence or public views are affected.
- Emergency incident-driven changes may ship first only under an Incident Response or break-glass record that names the incident, affected scope, owning service, temporary authority, expiry, rollback/fallback path, and Overwatch evidence refs. The urgent path should prefer reversible containment, route shifts, throttles, validation fixes, redaction fixes, or narrow compatibility patches; it must not silently create permanent protocol or governance rules. A retrospective PIP is required before the emergency change becomes permanent, before the next broad rollout, or before founder/backbone fallback is removed. That PIP must link incident timelines, containment refs, affected-service decisions, security/privacy/compliance/accounting findings, tests, migration and rollback evidence, public-report refs where required, and any rejected alternatives; unresolved severe findings keep the PIP in review, rollback-required, or accepted-risk state rather than implemented.
- Stable public archive fields are limited to interoperability and governance-history fields external builders and mobile clients can safely depend on: PIP id, slug, title, redacted summary, domain, current state, proposal type, affected service and schema/protocol refs, compatibility class, public reason codes, accepted/deprecated/superseded/rolled-back markers, effective window, migration and deprecation windows, replacement refs, public implementation/release/test refs, public migration and rollback refs, public report/correction/retraction refs, schema version, updated-at timestamp, public URL or namespace ref, and content or artifact hashes. Public archives must not expose tenant ids, actor ids, raw command/event traces, private reviewer notes, unredacted security findings, exploit details, fraud heuristics, payment/compliance evidence, private topology, vault refs, encrypted Docdex context, or private user data. Internal records may retain stronger refs for stewards, auditors, and legal/compliance audiences under audience policy.
- Central AI may recommend non-mutating governance support: section completeness checks, affected-service discovery, duplicate/supersession suggestions, compatibility-risk classification, missing-evidence prompts, reviewer-routing suggestions, redacted public-summary drafts, migration/rollback checklist review, retrospective incident evidence summaries, and stewardship-report draft material. Steward-only or owner-service-only decisions include accepting, rejecting, withdrawing, superseding, deprecating, rolling back, waiving required reviews, accepting risk, publishing/correcting/retracting public PIP views, approving emergency exceptions, changing protocol authority boundaries, changing security/privacy/compliance/accounting policy, authorizing grant/payout/surplus effects, and any decision touching identity, tenancy, ledger, vault, public-provider eligibility, regulated data, private AI context, or appealable sanctions. Central AI output must cite evidence, confidence, model/run provenance, and appeal/correction paths, and it must never become hidden final governance authority.
