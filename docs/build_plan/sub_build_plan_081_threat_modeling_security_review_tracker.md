# SUB BUILD PLAN #81 - Threat Modeling and Security Review Tracker

Attached SDS: [SDS #81 - Threat Modeling and Security Review Tracker](../sds/governance_ops/threat_modeling_security_review_tracker.md)

## Purpose

This sub-build plan turns SDS #81 into an implementation sequence for the Threat Modeling and Security Review Tracker. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Threat Modeling and Security Review Tracker is the durable security-review record system for Overrid. It records threat models, assets, trust boundaries, data flows, review assignments, security findings, mitigations, verification evidence, accepted risks, review gates, redacted reports, and replay bundles. It does not scan systems, run incident response, execute containment, block releases by itself, decide policy alone, patch services, mutate ledgers, or store raw exploit payloads, secrets, private keys, decrypted private content, or unredacted vulnerability details in broadly visible records.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #81: Threat Modeling and Security Review Tracker](../sds/governance_ops/threat_modeling_security_review_tracker.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflow, state machines, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoff, and resolved open-question decisions. |
| [Threat Modeling and Security Review Tracker plan](../service_catalog/governance_ops/threat_modeling_security_review_tracker.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order and keeps the tracker as a Phase 13 governance/compliance/security-review service. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies Rust workspace rules, shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, stable errors, signed envelopes, trace ids, idempotency keys, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry records, Overwatch audit refs, Overqueue primitives, and service account boundaries. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Policy Dry-Run previews, Oververify evidence, Overclaim dispute refs, Challenge Task refs, Workload Classifier facts, and trust-review prerequisites. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage facts, accounting evidence refs, grant/payout/ledger boundaries, and system-service overhead visibility without making the tracker an accounting authority. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies product adapter, AI routing, SDK, CLI, admin/developer client, and code/RAG adapter surfaces that need baseline threat models before production exposure. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies grid-resident service eligibility, backup/restore/failover drill refs, founder-hardware exit evidence, and system-service migration risks. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase records, Overstore evidence artifacts, Overvault private refs, Universal Namespace refs, Overmesh route facts, and retention/redaction boundaries. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies package validation, deployment plans, release strategy, rollback refs, version pins, migration refs, and release/package gate consumers. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies federation, public-interest, purpose-tag, cross-tenant grant, and public-benefit review surfaces that need security-impact records. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider onboarding, sandbox, fraud, reputation, challenge, payout-hold, and public-safe workload constraints. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies native app, mobile, wallet, assistant, search, messaging, social, maps, workspace, and Central AI Stewardship Interface review consumers. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Controls the first full build point for formal threat models, security reviews, tracked remediation, accepted risk, review gates, redacted reporting, replay, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #81 aligned to Phase 13 with prerequisites from earlier phases, without changing master phase order. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first services, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where service boundaries exist, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript only for client surfaces, native Overrid service boundaries, and no conventional database/object-store/vault/queue/security SaaS product boundary, blockchain, NFT, hardcoded pricing, revenue, or customer-count assumptions. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, and 13 | Attach SDS #81, freeze authority boundaries, and map prerequisites without moving formal security-review authority before Phase 13. |
| 2 | Master Phases 0, 1, 4, 8, and 13 | Define Rust contracts, canonical schemas, taxonomy records, severity/gate mappings, stable errors, events, and fixtures. |
| 3 | Master Phases 0, 1, 4, 6, 7, 8, 9, 12, and 13 | Implement threat-model, asset, boundary, data-flow, and threat APIs plus baseline scoping records for high-risk services. |
| 4 | Master Phases 0, 1, 4, 9, 12, and 13 | Implement review assignment, finding intake, triage, owner/due-window state, conflicts, signoff, and downstream links. |
| 5 | Master Phases 1, 4, 8, 9, and 13 | Implement mitigation, verification, accepted-risk, expiry, revocation, reopening, and evidence-ref lifecycle. |
| 6 | Master Phases 4, 7, 9, 10, 11, 12, and 13 | Implement review gates and cross-service handoffs to Release Strategy, Package Validator, PIP Registry, Migration Tooling, Overguard, Compliance, Incident Response, and Reporting. |
| 7 | Master Phases 8, 12, and 13 | Implement redaction profiles, authorized report views, replay bundles, public shells, correction/retraction records, and Central AI/operator views. |
| 8 | Master Phases 1, 5, 7, 8, 9, and 13 | Implement observability, retention, alerts, evidence import normalization, failure recovery, metering, and operational dashboards. |
| 9 | Master Phase 13 with evidence from Phases 0 through 12 | Harden integrations, access controls, scale behavior, reliability drills, and the tracker threat model itself. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, queue/progress metadata, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Threat Modeling and Security Review Tracker uses Rust-first contracts and service APIs for threat models, assets, trust boundaries, data flows, threat records, reviews, findings, mitigations, verification records, accepted risks, review gates, redaction profiles, report bundles, replay bundles, imports, metrics, alerts, and usage refs. TypeScript is acceptable only for generated client, SDK, CLI, admin, stewardship, native-app, or mobile surfaces that call authorized Overrid APIs.
- Contracts, event payloads, taxonomy entries, severity mappings, review-gate results, accepted-risk records, report views, stable errors, replay bundles, and deterministic fixtures use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant or system scope, trace id, idempotency key, role refs, policy refs, evidence refs, taxonomy version, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for model snapshots, evidence manifests, report shells, replay bundles, artifact refs, fixture inputs, and deterministic comparison artifacts. BLAKE3 must not be described as encryption.
- Sensitive exploit evidence stays in owning evidence systems such as Overwatch, Overvault, Overstore, or service-local evidence stores. The tracker stores refs, hashes, summaries, redaction classes, audience policies, retention metadata, and gate state.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Elasticsearch, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, SIEM SaaS, ticketing SaaS, compliance SaaS, analytics SaaS, external scanner infrastructure, DAO mechanics, token voting, blockchain, NFTs, external payment custody, hardcoded pricing, revenue forecasts, customer-count assumptions, release authority, policy authority, incident containment authority, ledger mutation authority, or raw private-evidence publication the tracker boundary.

## Phase 1: SDS Attachment, Security-Review Charter, And Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #81.**
  - Design: Link this document from the Threat Modeling and Security Review Tracker SDS, service plan, master build plan, Phase 13 plan, and build-plan crosswalk so builders can move from security-review scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/governance_ops/threat_modeling_security_review_tracker.md`, `docs/service_catalog/governance_ops/threat_modeling_security_review_tracker.md`, `docs/build_plan/master_plan.md`, `docs/build_plan/phase_13_governance_compliance_scale_hardening.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #81 returns both the tracker SDS and this sub-build plan.

- **1.2 Preserve Phase 13 as the first full build point.**
  - Design: Keep formal security-review workflow in Phase 13 because it depends on identity, tenant scope, policy, audit, release/package/migration refs, evidence systems, storage/private refs, incident/compliance/reporting handoffs, native-app exposure, and public-provider gates.
  - Output: Phase-gate note that earlier phases may create scoped baseline threat-model records, while full review workflow, accepted-risk finality, gate publication, and redacted reporting start in Phase 13.
  - Validation: Review proves the plan does not move formal risk acceptance, public security posture reporting, review-gate finality, release/package blocking, incident containment, or policy decision authority into earlier phases.

- **1.3 Freeze tracker ownership boundaries.**
  - Design: Record that the service owns threat-model records, asset and boundary records, data-flow and threat records, review assignments, findings, mitigation refs, verification refs, accepted-risk records, review gates, redaction profiles, report bundles, replay bundles, imports, metrics, and usage refs.
  - Output: Ownership checklist for APIs, records, events, reports, replay bundles, operations, and downstream handoffs.
  - Validation: Review confirms the service does not run scanners, execute containment, deploy patches, mutate ledgers, accept PIPs, adjudicate incidents, enforce policy alone, or publish raw exploit/private evidence.

- **1.4 Carry forward resolved SDS #81 decisions.**
  - Design: Preserve resolved decisions for required baseline threat models, deterministic severity-to-gate mapping, component-specific risk acceptance authority, exploit-sensitive evidence retention, public redaction shells, and append-only taxonomy versioning.
  - Output: Resolved-decision checklist covering critical/high blockers, medium blocker conditions, accepted-risk roles, evidence retention windows, public shell retention, taxonomy v1 domains, and policy mapping separation.
  - Validation: Review rejects silent severity downgrade, AI-only risk acceptance, unscoped accepted risk, missing expiry, missing owner, missing evidence, mutable taxonomy history, and public exposure of exploit-sensitive content.

- **1.5 Map prerequisites and consumers.**
  - Design: Identify the upstream evidence sources and downstream consumers for tracker state before implementing schemas so field names and refs align with owner-service boundaries.
  - Output: Dependency matrix covering Overwatch, PIP Registry, Incident Response, Release Strategy, Package Validator, Compliance Boundary, Overguard, Stewardship Reporting, Central AI, SDK, CLI, Admin/Developer UI, native apps, and mobile.
  - Validation: Matrix review confirms every dependency is used through APIs/events/refs and no implementation plan reads private storage directly or treats missing tracker records as approval.

## Phase 2: Contracts, Taxonomy, Severity Mapping, And Fixtures

### Work Items

- **2.1 Define canonical record envelopes.**
  - Design: Create shared envelope fields for tenant or system scope, actor or service account, trace id, idempotency key, schema version, taxonomy version, policy refs, evidence refs, audit refs, created/updated times, and append-only correction refs.
  - Output: Rust contract modules and JSON Schema fixtures for shared tracker envelopes.
  - Validation: Contract tests reject missing scope, missing actor/service account, missing trace id, missing idempotency key on mutating commands, and incompatible schema versions.

- **2.2 Model core tracker schemas.**
  - Design: Define schemas for `threat_model`, `security_asset`, `trust_boundary`, `data_flow`, `threat_record`, `security_review`, `security_finding`, `mitigation_plan`, `verification_record`, `accepted_risk`, `review_gate`, and `security_replay_bundle`.
  - Output: Canonical JSON Schema files, Rust typed contracts, stable field docs, examples, and fixture snapshots.
  - Validation: Schema fixtures round-trip through Rust serializers and reject ownerless findings, unscoped risks, boundaryless review starts, and verification records without evidence refs.

- **2.3 Ship Overrid-native threat taxonomy v1.**
  - Design: Encode taxonomy categories for identity/tenant escape, key/secret handling, policy bypass, package/supply-chain compromise, execution sandbox escape, scheduler/lease manipulation, node/provider fraud, public-node leakage, Overmesh/namespace hijack, data exposure, ledger/accounting/payout manipulation, AI/agentic enforcement abuse, native/mobile abuse, federation/public-provider abuse, compliance/reporting failure, incident/recovery failure, and governance/PIP abuse.
  - Output: Append-only taxonomy records with stable codes, aliases, deprecations, migration notes, and example mappings.
  - Validation: Taxonomy tests prove edits do not rewrite prior codes and policy/gate behavior stays in separately versioned mappings.

- **2.4 Define severity and review-gate mappings.**
  - Design: Convert SDS severity decisions into versioned review-gate rules for critical/high, medium, low, informational, missing severity, missing owner, missing evidence, stale cadence, expired risk, and unresolved redaction states.
  - Output: Mapping schema and fixtures for pass, warning, blocked, accepted-risk, waived, stale, expired-risk, and superseded states.
  - Validation: Gate fixtures prove critical/high findings block protected subjects unless remediated or actively accepted, medium findings block high-compliance or public exposure, and missing mandatory fields block instead of downgrade.

- **2.5 Define events, stable errors, and fixtures.**
  - Design: Align API errors and events with SDS names so downstream services can rely on stable reason codes and replayable event state.
  - Output: Event schemas for model creation, scoping, threat changes, review start, findings, triage, mitigations, remediation verification, risk acceptance, risk expiry, gate changes, report exports, and usage emission plus stable errors such as `scope_required`, `boundary_required`, `owner_required`, `severity_required`, `evidence_required`, `mitigation_required`, `verification_required`, `risk_acceptance_authority_required`, `risk_expired`, `review_gate_blocked`, and `redaction_required`.
  - Validation: Event and error fixtures match the SDS event surface and no event contains raw secrets, exploit payloads, decrypted context, private user data, or fraud heuristics.

## Phase 3: Threat-Model Records, Assets, Boundaries, And Threat APIs

### Work Items

- **3.1 Implement threat-model lifecycle APIs.**
  - Design: Build `POST /threat-models` and `GET /threat-models/{model_id}` with signed identity, scope, subject refs, lifecycle states, review cadence, linked PIPs/releases/incidents, and authorized views.
  - Output: Rust service handlers, contract fixtures, policy checks, audit events, and read models for `draft`, `scoping`, `ready_for_review`, `in_review`, `approved`, `approved_with_accepted_risk`, `superseded`, and `archived`.
  - Validation: Integration tests prove authorized creation/read paths, idempotent duplicate handling, state validation, audit emission, and rejection of missing subject scope.

- **3.2 Implement asset and trust-boundary capture.**
  - Design: Build asset, boundary, and correction endpoints that preserve asset ownership, data classes, trust levels, exposure classes, authentication/authorization expectations, encryption expectations, network/storage/context boundaries, and known exceptions.
  - Output: `POST /threat-models/{model_id}/assets` and `POST /threat-models/{model_id}/boundaries` handlers plus append-only correction events.
  - Validation: Tests reject review start for models without minimum assets and boundaries, prove corrections append new records, and confirm boundary exceptions require evidence refs.

- **3.3 Implement data-flow records.**
  - Design: Capture source, destination, data classes, transport, storage refs, vault refs, retention class, transformation summary, leakage concerns, and owner-service refs without duplicating owner-service private evidence.
  - Output: Data-flow API, schema fixtures, and read projections for reviewers.
  - Validation: Data-flow tests prove protected/private data classes require redaction and evidence refs, while public views hide private topology and sensitive refs.

- **3.4 Implement threat-record APIs.**
  - Design: Build threat creation/update/supersession/closure workflows with category, attacker model, affected assets, scenario summary, likelihood, impact, confidence, linked evidence, and review status.
  - Output: `POST /threat-models/{model_id}/threats` handler, taxonomy validation, threat lifecycle events, and supersession links.
  - Validation: Tests prove taxonomy version is recorded, superseded threats remain replayable, closed threats require mitigation, verification, accepted risk, or explicit non-applicability evidence.

- **3.5 Seed baseline threat-model records.**
  - Design: Create scoped baseline records for production, system-service, public-provider, native-app, mobile, AI-routing, secret-bearing, identity, ledger, package, release, migration, and policy authority boundaries before formal Phase 13 review.
  - Output: Seed fixtures for the first required SDS #81 baseline set and import-ready subject refs.
  - Validation: Fixture tests prove baseline records contain assets, trust boundaries, top threats, owners, and review cadence, while clearly marking formal review state as Phase 13 work.

## Phase 4: Security Reviews, Findings, Triage, And Assignments

### Work Items

- **4.1 Implement security-review intake.**
  - Design: Build `POST /security-reviews` for threat model, PIP, release, incident, package, migration, native app, mobile, or system-service subjects with review type, scope, owners, reviewers, phase gate, due window, blocked reasons, and signoff refs.
  - Output: Review lifecycle handlers for `planned`, `assigned`, `in_progress`, `blocked`, `finding_remediation`, `verification`, `passed`, `failed`, `waived_with_risk`, and `superseded`.
  - Validation: Tests reject unscoped reviews, missing reviewers, missing owner, stale model refs, and review start before minimum scoping fields exist.

- **4.2 Implement assignment, conflict, and waiver controls.**
  - Design: Enforce reviewer assignment, role authority, conflict-of-interest evidence, reassignment, and signed waiver records without letting automation or Central AI approve risk alone.
  - Output: Assignment APIs, conflict records, waiver records, audit events, and role-policy fixtures.
  - Validation: Tests prove conflicted reviewers cannot sign off without reassignment or waiver, and role-bound actions emit Overwatch audit refs.

- **4.3 Implement finding recording and triage.**
  - Design: Build `POST /security-reviews/{review_id}/findings` with severity, affected subject refs, owner, due window, exploitability class, evidence refs, redaction class, and downstream work refs.
  - Output: Finding lifecycle handlers for `open`, `triaged`, `remediation_planned`, `remediation_in_progress`, `remediated`, `verification_pending`, `verified`, `accepted_risk`, `expired_risk`, `reopened`, and `closed`.
  - Validation: Tests reject missing severity, missing owner, missing evidence, and invalid redaction class; triage events update review-gate projections deterministically.

- **4.4 Link findings to downstream work.**
  - Design: Attach PIP, implementation task, release, package, migration, incident, compliance, Overguard policy, stewardship report, and Central AI evidence refs without mutating those owner systems.
  - Output: Downstream-link records, link validation, and read projections for owners.
  - Validation: Tests prove links are refs only, missing downstream owners keep findings open, and deleted or unavailable owner refs do not produce false approval.

- **4.5 Build reviewer and owner views.**
  - Design: Provide authorized query views by owner, reviewer, severity, due window, service, release, PIP, incident, accepted-risk expiry, and gate state for admin/developer and stewardship clients.
  - Output: Query handlers, pagination, filtering fixtures, redacted public/steward/operator projections, and access-policy tests.
  - Validation: View tests prove least-privilege access, stable pagination, redaction per audience, and no raw exploit/private evidence in public or broad operator views.

## Phase 5: Mitigation, Verification, Accepted Risk, And Expiry

### Work Items

- **5.1 Implement mitigation-plan workflow.**
  - Design: Build `POST /findings/{finding_id}/mitigations` for mitigation type, owning service, expected control, test refs, monitoring refs, policy refs, implementation refs, target release, and fallback.
  - Output: Mitigation handlers, schema fixtures, status projections, and events.
  - Validation: Tests reject mitigation plans without owner, control summary, or at least one test, monitoring, policy, or implementation ref.

- **5.2 Implement verification records.**
  - Design: Build `POST /findings/{finding_id}/verification` for verifier, method, test run refs, package validation refs, audit refs, result, residual risk, and verification time.
  - Output: Verification API, replayable verification events, result projections, and residual-risk handling.
  - Validation: Tests prove verification cannot close a finding without evidence refs, verifier authority, and result state; failed verification reopens or keeps remediation pending.

- **5.3 Implement accepted-risk lifecycle.**
  - Design: Build `POST /findings/{finding_id}/accept-risk` and `POST /accepted-risks/{risk_id}/review` with component-specific authority, reason, scope, compensating controls, expiry, review window, affected scope, public/private report class, and reopening triggers.
  - Output: Accepted-risk records, authority-policy fixtures, expiry jobs, review/renew/revoke/supersede handlers, and gate projections.
  - Validation: Tests prove accepted risks require correct role bundles, expiry, compensating controls, evidence, and review cadence; expired risks block or warn according to mapping.

- **5.4 Implement automatic reopening triggers.**
  - Design: Reopen or refresh risks when evidence changes, policy changes, taxonomy changes, release scope widens, incident history changes, mitigation fails, or review cadence expires.
  - Output: Trigger evaluator, events for reopened risks, owner notifications, and gate-state updates.
  - Validation: Scenario tests prove stale reviews and expired risks never remain silently passing.

- **5.5 Preserve evidence refs and retention metadata.**
  - Design: Store exploit-sensitive refs, hashes, redaction class, audience policy, and retention metadata without copying secret-bearing material into broad tracker records.
  - Output: Evidence-ref schema, retention policies for critical/high, medium, low/informational, public shells, private evidence maps, incidents, PIPs, audits, disputes, compliance holds, and reports.
  - Validation: Retention tests prove public shells are append-only and private evidence maps follow the stricter linked retention rule.

## Phase 6: Review Gates And Cross-Service Handoffs

### Work Items

- **6.1 Implement review-gate evaluation.**
  - Design: Build `POST /review-gates/{subject_ref}/evaluate` for release, package, migration, PIP, native-app publication, mobile gateway, public-provider, identity, ledger, vault, AI-enforcement, and system-service subjects.
  - Output: Deterministic gate states for `not_required`, `required`, `waiting_for_review`, `blocked_by_findings`, `blocked_by_expired_risk`, `passed`, `passed_with_accepted_risk`, `waived_by_authority`, and `superseded`.
  - Validation: Tests prove protected subjects block on high/critical findings, expired risks, missing owner/severity/evidence, stale cadence, or unresolved redaction.

- **6.2 Integrate Release Strategy and Package Validator.**
  - Design: Expose gate state, finding summaries, mitigation refs, package validation refs, release refs, rollback requirements, and accepted-risk state through APIs/events.
  - Output: Handoff contracts for Release Strategy Service and Package Validator plus fixtures for rollout, freeze, package validation, and rollback cases.
  - Validation: Integration tests prove the tracker does not directly block deployments but returns deterministic state that release/package owners can enforce.

- **6.3 Integrate PIP Registry and Migration Tooling.**
  - Design: Link proposed protocol/service changes, security-impact sections, implementation refs, migration plans, cutover windows, rollback refs, accepted risks, and review findings.
  - Output: PIP and migration handoff contracts, review requirements, and replay links.
  - Validation: Tests prove PIPs and migrations cannot treat missing tracker records as approval and accepted security risks remain visible in proposal/migration histories.

- **6.4 Integrate Overguard and Compliance Boundary.**
  - Design: Provide review facts, compliance markers, regulated data classes, policy refs, exception refs, jurisdiction facts, and security evidence to policy/compliance consumers without making the tracker a policy engine.
  - Output: Fact-bundle events, Overguard input refs, Compliance Boundary export refs, and access policy.
  - Validation: Tests prove policy/compliance consumers receive evidence-backed refs and the tracker never mutates policy bundles or compliance rulesets.

- **6.5 Integrate Incident Response, Stewardship Reporting, and Central AI.**
  - Design: Hand off prior threats, missed mitigations, security findings, accepted risks, redacted posture summaries, and evidence refs to incident, reporting, and AI review consumers.
  - Output: Incident links, reporting bundle refs, Central AI redacted evidence views, and stewardship report projections.
  - Validation: Tests prove incident/report/AI consumers receive redacted, audience-limited state and cannot silently escalate recommendations into enforcement.

## Phase 7: Redaction, Reporting, Replay, And Public Views

### Work Items

- **7.1 Implement redaction profiles.**
  - Design: Define internal, steward, auditor, affected-party, and public profiles with explicit rules for exploit steps, topology, fraud heuristics, security scanner details, secret-bearing refs, private user data, and evidence maps.
  - Output: Redaction profile records, profile tests, report-view policy, and failure codes.
  - Validation: Tests prove public and broad operator views cannot expose raw exploit details, secrets, private data, decrypted context, fraud heuristics, or unredacted evidence refs.

- **7.2 Implement security report views.**
  - Design: Build `GET /security-reports/{subject_ref}` for authorized posture summaries, finding counts, open risks, accepted risks, review cadence, blocked gates, and redacted evidence refs.
  - Output: Report API, audience-filtered projections, pagination, export refs, and report events.
  - Validation: Report tests prove aggregate and redacted views are accurate, access-scoped, and safe for stewardship reporting.

- **7.3 Implement replay bundles.**
  - Design: Build `GET /security-reviews/{review_id}/replay` to reconstruct model, review, findings, mitigations, accepted risks, policy refs, audit refs, redacted evidence maps, and taxonomy versions.
  - Output: Replay API, BLAKE3/content hashes for bundle members, deterministic replay fixtures, and audit-ready output.
  - Validation: Replay tests reconstruct outcomes from stored refs and fail loudly on missing evidence, inconsistent policy refs, or non-deterministic ordering.

- **7.4 Implement public shells, corrections, and retractions.**
  - Design: Preserve public redaction bundles and report shells as append-only artifacts with correction, retraction, supersession, artifact hash, safe reason-code history, and private evidence map references.
  - Output: Public shell records, correction/retraction APIs, supersession events, and namespace/reporting refs.
  - Validation: Tests prove report corrections never rewrite history silently and private evidence maps follow stricter retention than public shells.

- **7.5 Build operator, steward, and Central AI views.**
  - Design: Provide authorized dashboards and API projections for open findings, overdue work, stale threat models, expired risks, blocked gates, severe findings, redaction status, report exports, and replay health.
  - Output: Query/read-model contracts for Admin/Developer UI, Central AI Stewardship Interface, SDK, CLI, native apps, and mobile gateway.
  - Validation: View tests prove least-privilege access and no client surface becomes a privileged backdoor or policy authority.

## Phase 8: Operations, Retention, Metering, And Failure Recovery

### Work Items

- **8.1 Implement observability and alerts.**
  - Design: Track open findings by severity, owner, service, release, PIP, incident, due window, review state, accepted-risk expiry, stale models, report redaction status, and replay health.
  - Output: Rust tracing spans, OpenTelemetry-compatible metrics, Overwatch audit refs, alert events, and operational dashboards.
  - Validation: Alert tests prove severe findings without owners, risks nearing expiry, overdue verification, stale review cadence, and blocked gates produce deterministic alerts.

- **8.2 Implement retention and cleanup jobs.**
  - Design: Apply SDS retention decisions for critical/high, medium, low/informational, public shells, private evidence maps, accepted risks, reports, incidents, PIPs, audits, disputes, and compliance holds.
  - Output: Retention scheduler, retention policy records, hold records, cleanup events, and proof fixtures.
  - Validation: Retention tests prove evidence refs survive required windows, compliance holds block cleanup, and public shell artifacts remain append-only.

- **8.3 Normalize external review imports.**
  - Design: Allow imports of external review reports only as structured normalized summaries with evidence refs, redaction classes, provenance, owner assignment, and review state.
  - Output: Import API, normalization schema, duplicate detection, validation errors, and audit events.
  - Validation: Import tests reject raw secret-bearing payloads, raw exploit details in broad records, ambiguous owners, missing evidence refs, and unsupported report formats.

- **8.4 Implement failure-mode recovery.**
  - Design: Encode SDS failure modes for incomplete scoping, ownerless findings, mitigation without tests, expired accepted risks, reviewer conflicts, evidence system outages, stale review gates, and redaction mistakes.
  - Output: Recovery handlers, state transitions, report retraction flow, blocked verification states, and operator runbooks.
  - Validation: Failure tests prove the service blocks safely rather than guessing, downgrading, or publishing unsafe evidence.

- **8.5 Emit metering and usage refs.**
  - Design: Emit usage refs for threat-model creation, review work, evidence packaging, report export, replay, Central AI analysis, and long-running review automation without hardcoded pricing or market assumptions.
  - Output: Usage events linked to model id, review id, subject refs, actor role, tenant/system scope, service account, evidence package refs, and report refs.
  - Validation: Metering tests prove usage emits through Overmeter-compatible refs and never mutates ORU balances, Seal Ledger entries, bills, payouts, or grants.

## Phase 9: Service Integrations, Security Hardening, And Scale Drills

### Work Items

- **9.1 Build full integration scenarios.**
  - Design: Exercise end-to-end flows across Overwatch, PIP Registry, Incident Response, Release Strategy, Package Validator, Compliance Boundary, Overguard, Stewardship Reporting, Central AI, SDK, CLI, Admin/Developer UI, native apps, and mobile.
  - Output: Integration scenarios for release gate, package gate, PIP review, migration gate, incident follow-up, accepted risk, redacted report, and replay.
  - Validation: Integration tests prove each consumer uses APIs/events/refs and no service reads tracker storage directly.

- **9.2 Harden tracker access and mutation controls.**
  - Design: Threat-model the tracker itself for identity takeover, tenant escape, reviewer role abuse, finding tampering, evidence-ref spoofing, accepted-risk forgery, redaction bypass, replay mismatch, and report overexposure.
  - Output: Tracker self-threat model, mitigation checklist, access-policy tests, signature checks, tamper-evidence checks, and replay tests.
  - Validation: Security tests prove signed authority, idempotency, append-only correction, audit refs, and least-privilege views hold under negative scenarios.

- **9.3 Run reliability and scale drills.**
  - Design: Drill review queue surge, evidence-system outage, Overwatch outage, redaction failure, report export backlog, accepted-risk expiry surge, stale model fanout, gate evaluation spike, and replay backlog.
  - Output: Drill records with expected behavior, actual behavior, evidence refs, follow-up findings, and report handoffs.
  - Validation: Drill validation proves safe degraded states, no false approval, no private evidence leakage, and bounded recovery steps.

- **9.4 Validate performance and deterministic ordering.**
  - Design: Test large threat models, many findings, concurrent reviewer updates, bulk gate evaluations, report exports, replay bundles, and filtered dashboard queries.
  - Output: Load-test fixtures, benchmark results, pagination limits, deterministic sort keys, and backpressure behavior.
  - Validation: Performance tests meet documented bounds and concurrent writes remain append-only, idempotent, and replayable.

- **9.5 Complete Phase 13 hardening handoff.**
  - Design: Feed tracker findings, accepted risks, report outputs, replay health, and drill evidence into Phase 13 governance/security/compliance completion criteria.
  - Output: Phase 13 readiness checklist and remediation backlog for remaining severe findings.
  - Validation: Phase 13 cannot mark security review complete while high/critical findings lack mitigation, active accepted risk, or explicit blocker status.

## Phase 10: Validation, Documentation Alignment, And Handoff

### Work Items

- **10.1 Validate documentation alignment.**
  - Design: Confirm this plan links from the SDS, service catalog plan, master build plan, Phase 13 plan, and build-plan crosswalk, and that all docs preserve Phase 13 as the first full build point.
  - Output: Alignment evidence for SDS #81 docs and the 0-13 master plan.
  - Validation: Link and search checks prove all backlinks resolve and no doc moves formal security-review authority before Phase 13.

- **10.2 Validate tech stack guardrails.**
  - Design: Scan the plan and linked docs for conventional cloud-product, scanner/SIEM/ticketing SaaS, blockchain, NFT, token-voting, pricing, revenue, customer-count, or core TypeScript runtime drift.
  - Output: Stack guardrail scan evidence and negative-control notes for explicitly rejected assumptions.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, SIEM SaaS, ticketing SaaS, blockchain, NFT, token voting, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate sub-build plan structure.**
  - Design: Check title prefix, attached SDS link, source alignment table, master phase mapping, tech-stack guardrails, phases 1 through 10, five work items per phase, and Design/Output/Validation structure for every work item.
  - Output: Structure validation evidence for this file.
  - Validation: Focused checks pass for 10 phase headings, 50 work items, 50 Design lines, 50 Output lines, 50 Validation lines, local Markdown links, no broken fences, final newline, and no tabs.

- **10.4 Update queue and progress metadata.**
  - Design: Mark `081-build-plan` complete, move the next incomplete build-plan task to SDS #82, and record validation evidence in build-plan progress and queue progress.
  - Output: Updated `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, and `docs/build_plan/progress.md`.
  - Validation: JSON validation passes and queue progress reports `082-build-plan` as the next incomplete build-plan task with counts adjusted for one additional completed task and zero running tasks.

- **10.5 Reindex and hand off implementation readiness.**
  - Design: Refresh Docdex index for the new plan and linked docs, run retrieval checks, run pre-commit validation, and document the test-runner status.
  - Output: Docdex index/search/DAG evidence, symbols evidence, pre-commit result, test-runner result or blocker, and final handoff summary.
  - Validation: Docdex search returns the new #81 sub-build plan and linked SDS/service/Phase 13/crosswalk evidence; pre-commit passes; `docdexd run-tests` either passes or records the known missing `.docdex/run-tests.json` blocker.
