# SUB BUILD PLAN #80 - Stewardship Reporting Service

Attached SDS: [SDS #80 - Stewardship Reporting Service](../sds/governance_ops/stewardship_reporting_service.md)

## Purpose

This sub-build plan turns SDS #80 into an implementation sequence for the Stewardship Reporting Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Stewardship Reporting Service is the evidence-backed reporting layer for Overrid governance. It defines report templates and periods, inventories source refs, builds metric snapshots and evidence manifests, applies audience-specific redaction profiles, coordinates privacy/security/compliance/accounting/stewardship review, publishes public or private artifacts, preserves corrections and retractions, and produces replay bundles. It does not decide grants, spend funds, mutate ledgers, adjudicate disputes, determine fraud outcomes, accept PIPs, contain incidents, expose private evidence, or replace owning-service truth.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #80: Stewardship Reporting Service](../sds/governance_ops/stewardship_reporting_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflow, state machines, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoff, and resolved open-question decisions. |
| [Stewardship Reporting Service plan](../service_catalog/governance_ops/stewardship_reporting_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order and keeps Stewardship Reporting Service as a Phase 13 governance/compliance service. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies Rust workspace rules, shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, stable errors, signed envelopes, trace ids, idempotency keys, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry records, Overwatch audit refs, Overqueue primitives, and service account boundaries. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard audience/access decisions, Policy Dry-Run previews, Oververify evidence, Overclaim dispute refs, Challenge Task refs, Workload Classifier facts, and trust-review prerequisites. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies ORU, Seal Ledger, Overmeter, Overbill, Overgrant, Overasset, Provider Payout, accounting reconciliation, usage refs, grant refs, payout refs, and native-service surplus reporting inputs. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies product adapter, AI routing, SDK, CLI, admin/developer client, and report-consumer surfaces that may create report jobs, list reports, or link report refs. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies grid-resident service health, founder-hardware fallback and exit evidence, backup/restore/failover drill refs, migration-readiness status, and backbone reporting inputs. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase records, Overstore report artifacts, Overvault private refs, Universal Namespace public report refs, Overmesh routes, native storage/vault boundaries, and retention implications. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies package validation, release strategy, deployment plans, rollout/rollback refs, version pins, migration refs, and release-health report inputs. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies federation template, public-interest pool, purpose tag, cross-tenant grant, pool allocation, and public-benefit reporting inputs. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider onboarding, public sandbox, fraud, reputation, challenge, payout-hold, provider-capacity, and public-safe workload reporting constraints. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies native app, wallet, assistant, search, messaging, social, maps, workspace, mobile, and Central AI Stewardship Interface consumers of published reports, private reports, notices, and replay. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Controls the first full build point for Stewardship Reporting Service, including report governance, public reporting, redaction review, privacy/security/compliance/accounting review, corrections, retractions, replay, threat/security review, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #80 aligned to Phase 13 with prerequisites from earlier phases, without changing master phase order. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first services, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where service boundaries exist, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript only for client surfaces, native Overrid service boundaries, and no conventional database/object-store/vault/queue/compliance SaaS product boundary, blockchain, NFT, hardcoded pricing, revenue, or customer-count assumptions. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 10, 11, 12, and 13 | Attach SDS #80, freeze reporting authority, and map report sources without moving owner-service truth. |
| 2 | Master Phases 0, 1, 4, 5, 8, 10, 11, 12, and 13 | Define Rust contracts, canonical schemas, report templates, periods, state machines, stable errors, and fixtures. |
| 3 | Master Phases 0, 1, 4, and 13 | Implement template and period lifecycle APIs with signed commands, role policy, source declarations, and audit refs. |
| 4 | Master Phases 1, 4, 5, 7, 8, 9, 10, 11, 12, and 13 | Implement source inventories, metric snapshots, freshness checks, aggregation thresholds, and evidence manifests from owner refs. |
| 5 | Master Phases 5, 7, 9, 10, 11, 12, and 13 | Implement report-type builders and reconciliation for health, grants, surplus, fraud, incidents, security, compliance, PIPs, migrations, and Central AI. |
| 6 | Master Phases 1, 4, 8, 11, 12, and 13 | Implement audience policy, redaction profiles, privacy thresholds, and required review workflows before artifact release. |
| 7 | Master Phases 8, 12, and 13 | Implement artifact generation, public/private report indexes, namespace refs, archive records, and authorized read APIs. |
| 8 | Master Phases 8, 9, 12, and 13 | Implement corrections, retractions, supersession, exports, replay bundles, and immutable public history. |
| 9 | Master Phase 13 with evidence from Phases 0 through 12 | Implement service integrations, metering, observability, dashboards, retention, incident hooks, threat/security review, and scale hardening. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, queue/progress metadata, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Stewardship Reporting Service uses Rust-first contracts and service APIs for templates, periods, build jobs, source inventories, metric snapshots, evidence manifests, redaction profiles, review records, artifacts, publication state, correction/retraction records, exports, indexes, and replay bundles. TypeScript is acceptable only for generated client, SDK, CLI, admin, stewardship, public-report, native-app, or mobile surfaces that call authorized Overrid APIs.
- Contracts, event payloads, report templates, period records, redaction profiles, review state, public summaries, private report metadata, stable errors, replay bundles, and deterministic fixtures use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant or system scope, trace id, idempotency key, role refs, audience class, policy refs, evidence refs, template version, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for templates, source inventories, metric snapshots, evidence manifests, artifacts, public-view snapshots, export packages, replay bundles, fixture inputs, and deterministic comparison artifacts. BLAKE3 must not be described as encryption.
- Reports, source refs, private evidence refs, artifacts, public namespace refs, usage refs, and audit refs use native Overrid service boundaries such as Overwatch, Overguard, Overtenant, Overpass, Overkey, Overbase, Overstore, Overvault, Universal Namespace Service, Overqueue, Overmesh, ORU Account Service, Seal Ledger, Overmeter, Overbill, Overgrant, Overasset, Provider Payout Service, Public-Interest Pool Service, Purpose Tag Registry, Fraud Control Service, Overclaim, Challenge Task Service, Oververify, Reputation and Anti-Sybil Service, PIP Registry, Compliance Boundary Service, Incident Response Service, Threat Modeling and Security Review Tracker, Migration Tooling, Release Strategy Service, Backup and Restore Service, Central AI Service, Central AI Stewardship Interface, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Elasticsearch, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, compliance SaaS, ticketing SaaS, SIEM SaaS, analytics SaaS, marketing dashboards, DAO mechanics, token voting, blockchain, NFTs, external payment custody, hardcoded pricing, revenue forecasts, customer-count assumptions, grant allocation authority, ledger mutation authority, fraud adjudication authority, incident containment authority, PIP acceptance authority, compliance-finding authority, or raw private-evidence publication the Stewardship Reporting Service boundary.

## Phase 1: SDS Attachment, Reporting Charter, And Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #80.**
  - Design: Link this document from the Stewardship Reporting SDS, service plan, master build plan, Phase 13 plan, and build-plan crosswalk so builders can move from reporting scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/governance_ops/stewardship_reporting_service.md`, `docs/service_catalog/governance_ops/stewardship_reporting_service.md`, `docs/build_plan/master_plan.md`, `docs/build_plan/phase_13_governance_compliance_scale_hardening.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #80 returns both the Stewardship Reporting SDS and this sub-build plan.

- **1.2 Preserve Phase 13 as the first full build point.**
  - Design: Keep full reporting authority in Phase 13 because it depends on identity, tenant scope, policy, audit, accounting refs, owner-service evidence, storage/private refs, release/migration refs, incident/compliance/security review, native-app consumers, and public-reporting governance.
  - Output: Phase-gate note that earlier phases provide evidence sources and client surfaces, while full public/private report production starts in Phase 13.
  - Validation: Review proves the plan does not move report publication, public archive authority, privacy review, security posture reporting, accounting reconciliation, or correction/retraction finality into earlier phases.

- **1.3 Freeze reporting ownership boundaries.**
  - Design: Record that the service owns report templates, reporting periods, build jobs, source inventories, metric snapshots, evidence manifests, redaction profiles, report reviews, artifacts, public/private indexes, publication state, corrections, retractions, supersession, exports, archives, usage refs, and replay bundles.
  - Output: Ownership checklist for APIs, records, events, operations, review gates, public projections, and handoffs.
  - Validation: Review confirms the service does not allocate grants, spend funds, mutate ORU or Seal Ledger records, adjudicate disputes, decide fraud outcomes, contain incidents, accept protocol changes, replace compliance findings, or publish raw private evidence.

- **1.4 Carry forward resolved SDS #80 decisions.**
  - Design: Preserve resolved decisions for mandatory recurring reports, privacy thresholds, Central AI draft authority, public archive and correction shape, seed-hardware reporting labels, and owner-service truth boundaries.
  - Output: Resolved-decision checklist covering public participation reports, aggregate privacy thresholds, sensitive-bucket thresholds, affected-party views, steward/auditor/private exports, Central AI provenance, public archive records, correction/retraction notices, and seed-hardware scope labels.
  - Validation: Review rejects missing report categories, single global privacy thresholds, Central AI hidden final authority, invisible corrections/retractions, overstated decentralization, and any report that invents values or source truth.

- **1.5 Map upstream and downstream service boundaries.**
  - Design: Record how Overwatch, accounting services, Overgrant, Public-Interest Pool, Fraud Control, Overclaim, PIP Registry, Compliance Boundary, Incident Response, Threat Modeling, Migration Tooling, Release Strategy, Central AI, Central AI Stewardship Interface, SDK, CLI, admin UI, and native apps interact through refs and signed commands.
  - Output: Boundary matrix naming allowed reads, owned writes, required refs, denied direct mutations, policy refs, evidence refs, redaction classes, usage refs, audit refs, and owner-service finality.
  - Validation: Review confirms Stewardship Reporting exchanges source refs, report artifacts, notices, and replay bundles without taking over grant, ledger, fraud, dispute, incident, compliance, migration, release, PIP, or Central AI owner authority.

## Phase 2: Contracts, Schemas, State Machines, Stable Errors, And Fixtures

### Work Items

- **2.1 Create the Stewardship Reporting Rust contract module.**
  - Design: Add contract types for report templates, periods, build jobs, source inventories, metric snapshots, evidence manifests, redaction profiles, report reviews, artifacts, publication records, corrections, retractions, exports, public/private indexes, replay bundles, stable errors, lifecycle states, and event payloads.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, report-type enums, audience-class enums, redaction-class enums, review-role enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from owner-service mutation, grant decisions, ledger truth, incident containment, PIP acceptance, and private-evidence storage.

- **2.2 Define template, period, and build-job schemas.**
  - Design: Model `report_template`, `report_period`, and `report_build_job` with report type, semantic version, required sections, required sources, metric definitions, freshness rules, audience class, review requirements, publication policy, state, retries, failures, and audit refs.
  - Output: JSON Schema files, valid examples, invalid examples, stable error mapping, and fixtures for system health, grants, surplus routing, abuse/fraud, incidents, security posture, compliance boundaries, PIPs, migrations, and Central AI recommendations.
  - Validation: Schema tests reject missing required sources, missing period scope, unversioned templates, open periods used for final reports, public audiences without redaction profile refs, missing idempotency keys, and hidden private source payloads.

- **2.3 Define source inventory, metric, evidence, and redaction schemas.**
  - Design: Model source refs, owner service, freshness, data class, privacy constraints, missing-source markers, aggregation method, confidence, reconciliation status, evidence inclusion/exclusion reason, redaction profile, aggregation thresholds, contribution caps, and reviewer refs.
  - Output: Schema set, fixture set, threshold examples, owner-service ref examples, redacted aggregate examples, private export metadata examples, and stable errors for missing/stale sources, reconciliation mismatch, and restricted data.
  - Validation: Tests prove public aggregates enforce `k >= 20` ordinary thresholds, sensitive buckets enforce `k >= 50` with contribution caps, private exports remain role-scoped, and raw private content, payment details, encrypted Docdex context, fraud heuristics, exploit details, or private topology cannot enter public artifacts.

- **2.4 Define review, artifact, publication, correction, export, and replay schemas.**
  - Design: Model reviewer decisions, findings, correction requirements, artifact metadata, public namespace refs, storage refs, integrity hashes, publication state, correction/retraction records, replacement refs, export packages, archive refs, and replay inputs.
  - Output: Schema files, lifecycle enums, HTTP/API mapping, event mapping, deterministic examples, public history examples, private export examples, and golden replay fixtures.
  - Validation: Tests prove reports cannot publish without required privacy, security, compliance, accounting, and stewardship reviews; corrections/retractions create linked visible records; exports preserve audience class; and replay reconstructs source refs, redaction, review, publication, and correction state.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for template creation, period close, source inventory success/failure, metric reconciliation success/failure, privacy-threshold suppression, redaction review, security review, accounting review, public publication, private export, correction, retraction, supersession, seed-hardware report, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected stable errors, BLAKE3/content hashes, usage refs, audit refs, public projections, private export metadata, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, threshold decisions, redaction behavior, review behavior, usage refs, audit refs, and replay output across repeated runs.

## Phase 3: Template And Reporting Period Lifecycle APIs

### Work Items

- **3.1 Implement template creation and revision APIs.**
  - Design: Accept template creation or revision only with report type, semantic version, required sources, metric definitions, section definitions, redaction defaults, review requirements, publication policy, signed actor/service envelope, trace id, idempotency key, and Overwatch audit refs.
  - Output: `POST /stewardship-reports/templates`, authorized read API, versioned template records, template validation errors, audit events, and deterministic tests.
  - Validation: Tests reject unversioned templates, missing source declarations, missing review requirements, invalid redaction defaults, duplicate idempotency side effects, unauthorized roles, and template changes without immutable version records.

- **3.2 Implement report period open and close APIs.**
  - Design: Create period lifecycle operations for planned, open, closing, closed, corrected, and archived states with scope, time window, freshness requirements, correction window, and owner-service source expectations.
  - Output: `POST /stewardship-reports/periods`, period records, close-state validation, freshness policy refs, period audit events, and period fixtures.
  - Validation: Tests reject final build jobs against open periods, closing without required freshness rules, duplicate period windows for the same scope where disallowed, unauthorized close attempts, and missing trace/idempotency keys.

- **3.3 Implement build-job request and state read APIs.**
  - Design: Start report build jobs for template, period, scope, and audience, then expose build state, source freshness, missing refs, reconciliation status, retries, failures, and stable errors.
  - Output: `POST /stewardship-reports/build-jobs`, `GET /stewardship-reports/build-jobs/{job_id}`, queued job records, state transitions, retry metadata, and event payloads.
  - Validation: Tests reject unknown template versions, unclosed periods, forbidden audiences, missing redaction profiles, stale source policy, duplicate side effects, and finalization without Overwatch audit refs.

- **3.4 Implement authorized template and period reads.**
  - Design: Provide role-aware reads for templates, periods, source requirements, redaction defaults, review requirements, build eligibility, and public-safe metadata without exposing private source refs.
  - Output: Read models for public, affected-party, steward, auditor, legal/compliance, service owner, and admin/developer audiences.
  - Validation: Tests prove public reads exclude private source refs and reviewer internals, affected-party reads remain self-scoped, steward/auditor reads require policy grants, and denied reads produce stable errors.

- **3.5 Emit lifecycle events and usage refs.**
  - Design: Emit template version, period open/close, build request, queue, read, validation failure, and usage events with report type, template version, period, audience, trace id, and audit refs.
  - Output: `stewardship_reporting.template_versioned`, `period_opened`, `period_closed`, `build_started`, validation-failure events, usage refs, and replay-ready event fixtures.
  - Validation: Event tests prove payloads include required refs and never include raw private evidence, secret material, encrypted context, payment details, exploit details, sensitive fraud internals, or private topology.

## Phase 4: Source Inventory, Metric Snapshots, Evidence Manifests, And Freshness

### Work Items

- **4.1 Implement source inventory collection.**
  - Design: Collect source refs from Overwatch, accounting/grant/payout services, Public-Interest Pool, Purpose Tag Registry, Fraud Control, Overclaim, Challenge Task, Oververify, PIP Registry, Compliance Boundary, Incident Response, Threat Modeling, Migration Tooling, Release Strategy, Backup/Restore, Central AI, and native service refs through owner-service APIs.
  - Output: `source_inventory_snapshot` records with required/optional source refs, owner service, scope, freshness, data classes, privacy constraints, missing-source markers, and source hashes.
  - Validation: Tests reject invented values, raw private payload ingestion, unknown owner-service refs, stale required refs without explicit partial/blocked status, and inventory finalization without audit refs.

- **4.2 Implement metric snapshot builders.**
  - Design: Build metric snapshots from owner-service refs using documented aggregation methods, threshold policy, confidence, reconciliation status, integrity hashes, and redaction class.
  - Output: `metric_snapshot` records for health, availability, grants, surplus routing, ORU/Seal/Overbill/provider payout summaries, public-provider capacity, fraud/abuse aggregates, incident summaries, PIP changes, compliance-boundary changes, migration status, and Central AI activity.
  - Validation: Tests prove metrics cite owner refs, enforce ordinary and sensitive aggregation thresholds, carry confidence/reconciliation status, suppress rare categories, and fail or mark partial when required source evidence is missing.

- **4.3 Implement evidence manifest construction.**
  - Design: Build evidence manifests that include allowed evidence refs, excluded refs with reason, source hashes, redaction classes, audience, export integrity refs, and owner-service constraints.
  - Output: `evidence_manifest` records, manifest hashes, excluded-evidence reason codes, public/private manifest projections, and replay links.
  - Validation: Tests prove public manifests never expose raw user, tenant, workspace, payment, vault, encrypted Docdex, fraud heuristic, exploit, private topology, or sensitive security evidence and that private manifests remain policy-scoped.

- **4.4 Implement freshness and reconciliation gates.**
  - Design: Compare inventory and metric snapshots against template freshness rules, period close state, source update refs, ledger/accounting refs, incident status, PIP status, compliance boundary refs, migration refs, and Central AI recommendation state.
  - Output: Freshness reports, reconciliation records, missing-source blockers, partial-report markers, stable errors, review-required records, and Overwatch audit events.
  - Validation: Tests reject reports with stale critical sources, unresolved reconciliation mismatches, missing accounting refs for financial/grant reports, missing incident refs for incident reports, or missing public-safe partial markers.

- **4.5 Persist source inventory and metric replay inputs.**
  - Design: Persist snapshots, hashes, policy refs, audit refs, and owner-service refs in native Overrid storage boundaries so reports can be reproduced without storing raw private payloads.
  - Output: Overbase record refs, Overstore manifest refs, Overvault private evidence refs where allowed, usage refs, audit refs, and replay inputs.
  - Validation: Replay tests reconstruct source inventory and metric snapshots from refs and hashes; privacy scans prove protected payloads remain in owning services or Overvault-granted private views.

## Phase 5: Report Type Builders And Owner-Service Reconciliation

### Work Items

- **5.1 Implement system health and seed-hardware reports.**
  - Design: Build system health, availability, seed/private-swarm, founder-hardware dependency, control-plane uptime, queue/execution/retry health, Overwatch evidence durability, package/security-review status, and migration-readiness reports with explicit scope labels.
  - Output: Report builders, template fixtures, metric mappings, public-safe labels, private steward detail views, and seed-hardware partial-status handling.
  - Validation: Tests prove seed reports do not overstate decentralization, redact topology/provider-sensitive detail, avoid unsupported public-provider payout/finality claims, and require partial/private-only status where evidence is not strong enough.

- **5.2 Implement accounting, grant, surplus, payout, and public-interest reports.**
  - Design: Build reports for native-service surplus routing, public-interest grants and pool allocations, ORU/Seal Ledger/Overbill/Provider Payout reconciliation summaries, donor/grantee public-safe outcomes, and purpose-tag outcome summaries.
  - Output: Report builders, reconciliation mappings, accounting review requirements, owner-service refs, grant/pool summary views, correction paths, and replay fixtures.
  - Validation: Tests prove reports cite ORU, Seal Ledger, Overbill, Overgrant, Provider Payout, Public-Interest Pool, and Purpose Tag refs without allocating funds, changing balances, determining payouts, forecasting revenue, or inventing financial values.

- **5.3 Implement abuse, fraud, dispute, public-provider, and trust reports.**
  - Design: Build aggregate abuse/fraud statistics, challenge volumes, hold/throttle summary refs, dispute/correction trends, public-provider capacity/trust posture, reputation/anti-Sybil summaries, and rare-category suppression.
  - Output: Fraud/trust report builders, sensitive threshold enforcement, review requirements, redacted public views, affected-party self views, steward/auditor private views, and replay fixtures.
  - Validation: Tests prove sensitive reports enforce `k >= 50` thresholds and contribution caps, avoid naming affected parties unless authorized, withhold fraud internals, carry correction/retraction paths, and do not adjudicate fraud outcomes.

- **5.4 Implement incident, security posture, compliance, PIP, and migration reports.**
  - Design: Build reports from Incident Response, Threat Modeling, Compliance Boundary, PIP Registry, Migration Tooling, Release Strategy, Backup/Restore, Package Validator, and Overwatch refs with required review roles.
  - Output: Incident report summaries, security posture reports, severe-finding summaries, compliance-boundary change reports, accepted/emergency PIP reports, migration/founder-exit reports, public archive refs, and replay fixtures.
  - Validation: Tests prove reports avoid exploit details and sensitive incident internals, require security/compliance/stewardship review, cite owner refs, preserve PIP/change-governance boundaries, and do not contain incidents, accept changes, or execute migrations.

- **5.5 Implement Central AI recommendation and stewardship activity reports.**
  - Design: Build reports for Central AI recommendation activity, provenance, confidence, model/run refs, anomaly notes, reviewer-routing suggestions, correction paths, and public-safe narrative summaries while preserving human/steward finality for sensitive conclusions.
  - Output: Central AI report builders, non-authoritative draft markers, provenance refs, confidence fields, model/run refs, steward approval requirements, public/private projections, and replay fixtures.
  - Validation: Tests prove Central AI drafts never become hidden final governance authority, sensitive narratives require steward approval, reputation/payout/provider/regulatory-impacting statements remain reviewed, and source refs are preserved.

## Phase 6: Redaction Profiles, Audience Policy, Privacy Thresholds, And Review Workflow

### Work Items

- **6.1 Implement audience classification and access policy.**
  - Design: Resolve audience class before assembling or serving reports, using Overtenant roles, Overpass identities, Overkey signatures, Overguard policies, affected-party scope, steward/auditor/legal/compliance grants, and public/private report type.
  - Output: Audience-class resolver, policy refs, access decisions, stable errors, audit events, and read/write enforcement tests.
  - Validation: Tests prove denied audiences cannot request private artifacts, affected-party views are self-scoped, public views are aggregate/redacted, and service accounts cannot bypass redaction or review gates.

- **6.2 Implement redaction profile lifecycle.**
  - Design: Create and revise redaction profiles with allowed fields, withheld fields, aggregation thresholds, contribution caps, time-window smoothing, sensitive classes, security/fraud/compliance restrictions, reviewer refs, and template bindings.
  - Output: Redaction profile records, versioned profile refs, profile validation, profile change events, test fixtures, and default profile bindings.
  - Validation: Tests reject public profiles without thresholds, sensitive profiles without stronger caps, profile revisions without versioning, public artifacts with restricted fields, and private exports without audience policy refs.

- **6.3 Implement privacy, security, compliance, accounting, and stewardship review workflows.**
  - Design: Assign required reviewers from report type, audience, source classes, data classes, incident/security/compliance/accounting refs, and Central AI-generated draft markers.
  - Output: `report_review` records, review assignment events, findings, changes-requested state, approval/rejection/waiver state, role-specific decisions, and stable errors.
  - Validation: Tests prove public reports cannot publish until required review roles pass, waivers carry reason and accepted-risk refs, conflicts are recorded where applicable, and rejected reports return to draft with findings.

- **6.4 Implement restricted-data detection before artifact generation.**
  - Design: Scan draft report contents, manifests, public summaries, metrics, links, and metadata for restricted classes, raw private payloads, payment details, encrypted context, security exploit details, fraud internals, private topology, and contribution-threshold violations.
  - Output: Restricted-data findings, block records, incident trigger hooks for escaped artifacts, correction requirements, and test fixtures.
  - Validation: Tests prove restricted public artifacts are blocked before publication, escaped restricted artifacts create incident/correction paths, and false positives can be reviewed without weakening default-deny behavior.

- **6.5 Implement review audit, usage, and replay capture.**
  - Design: Capture review assignments, decisions, findings, waived risks, profile versions, reviewer roles, audit refs, usage refs, and replay inputs without storing raw private reviewer notes in public artifacts.
  - Output: Review audit records, usage events, replay bundle segments, public-safe review summaries, private review metadata, and retention refs.
  - Validation: Replay reconstructs review state and decision gates; public views show review status and correction history without leaking private evidence or sensitive reviewer notes.

## Phase 7: Artifact Generation, Publication, Public/Private Indexes, And Archive

### Work Items

- **7.1 Implement public and private artifact generation.**
  - Design: Generate audience-specific artifacts from template version, period, source inventory, metric snapshots, evidence manifest, redaction profile, and review state.
  - Output: `report_artifact` records, public artifacts, private steward/auditor artifacts, affected-party views, artifact hashes, storage refs, expiry/archive refs, and deterministic output tests.
  - Validation: Tests prove artifacts are deterministic for the same inputs, public artifacts contain only allowed fields, private artifacts require grants, and artifact hashes change when template/source/redaction inputs change.

- **7.2 Implement publication, withholding, and archive workflow.**
  - Design: Publish, withhold, or archive report artifacts only after approval with explicit reason codes, public namespace refs, replacement refs where applicable, and audit refs.
  - Output: `POST /stewardship-reports/{report_id}/publish`, publication records, public URL or namespace refs, withheld-state records, archive records, and events.
  - Validation: Tests reject publication without approvals, unauthorized publishers, missing public-safe artifact refs, missing namespace refs where required, and publication state changes without audit refs.

- **7.3 Implement public report index and authorized private indexes.**
  - Design: Provide list/read APIs for public reports and authorized private reports by type, period, audience, state, owning service, correction state, and archive status.
  - Output: `GET /stewardship-reports`, `GET /stewardship-reports/{report_id}`, public index records, private index records, pagination, stable filters, and client fixtures.
  - Validation: Tests prove public index excludes private reports and restricted metadata, private indexes enforce roles, pagination is stable, correction/retraction state is visible, and stale indexes are invalidated after publication changes.

- **7.4 Implement public-safe summary and native-app handoff views.**
  - Design: Produce public summaries and native-app report views for Central AI Stewardship Interface, Wallet/Usage Center, Search, Directory, Messaging, Social, Maps, Workspace, mobile clients, SDK, CLI, and admin UI.
  - Output: Public summary schema, native-app read models, generated client bindings, CLI output fixtures, admin/stewardship view fixtures, and route refs.
  - Validation: Client tests prove surfaces consume report APIs through authorized Overrid boundaries, cannot mutate owner-service truth, and render correction/retraction/withheld status consistently.

- **7.5 Implement archive retention and invalidation rules.**
  - Design: Maintain public report archive and private steward/auditor archive with retention policies, invalidation refs, public-history preservation, private export expiry, and source-correction hooks.
  - Output: Archive records, retention refs, invalidation events, cache metadata with redaction profile/version refs, archive read APIs, and retention tests.
  - Validation: Tests prove public history remains visible after correction/retraction, private exports expire or remain role-scoped according to policy, and cache invalidation follows redaction/profile/report-state changes.

## Phase 8: Corrections, Retractions, Supersession, Exports, And Replay

### Work Items

- **8.1 Implement correction workflow.**
  - Design: Create correction records when source data changes, reviewer findings require change, metric reconciliation changes, redaction mistakes are found, or public summaries need safe correction.
  - Output: `correction_retraction_record` correction state, corrected artifact refs, public notices, replacement hashes, reason codes, source-ref classes, and audit events.
  - Validation: Tests prove corrections preserve original public shell, publish visible deltas, link replacement reports, keep private evidence scoped, and cannot silently rewrite published history.

- **8.2 Implement retraction and supersession workflow.**
  - Design: Support retractions for unsafe or invalid public reports and supersession for template/report replacements while preserving safe public reason, withheld status, replacement refs, archive refs, and audit refs.
  - Output: Retraction records, supersession records, public notices, replacement/withheld links, stable reason codes, and event payloads.
  - Validation: Tests prove retractions remain visible, original reports are not deleted, sensitive details are withheld, superseded reports link to replacements, and unauthorized roles cannot retract or supersede reports.

- **8.3 Implement export packages.**
  - Design: Export report packages for public, steward, auditor, legal/compliance, affected-party, SDK, CLI, and admin audiences with audience-specific fields, integrity hashes, retention refs, and replay refs.
  - Output: `POST /stewardship-reports/{report_id}/exports`, export package metadata, Overstore refs, Overvault-granted private refs where permitted, usage refs, and export fixtures.
  - Validation: Tests prove public exports contain only public fields, private exports require policy grants, export hashes are stable, private export expiry works, and raw restricted data is not copied into export packages.

- **8.4 Implement report replay.**
  - Design: Reconstruct report build inputs, source inventory, metric snapshots, redaction, review, artifact generation, publication, correction, retraction, export, and archive refs from stored versions and hashes.
  - Output: `GET /stewardship-reports/{report_id}/replay`, `report_replay_bundle` records, deterministic replay output, mismatch markers, and replay audit events.
  - Validation: Replay tests reconstruct public and private report histories from template, period, source refs, redaction profile, review records, artifact refs, publication events, correction refs, and export refs.

- **8.5 Implement owner-service source correction hooks.**
  - Design: React to owner-service corrections from accounting/grants, Fraud Control, Overclaim, Incident Response, PIP Registry, Compliance Boundary, Migration Tooling, Central AI, and Overwatch with correction-required, rebuild-required, or no-op decisions.
  - Output: Source-correction listener records, report impact scans, rebuild jobs, correction requirements, retraction requirements, and audit events.
  - Validation: Tests prove owner-service corrections do not mutate reports silently, impacted reports get explicit correction/retraction state, non-impacted reports remain unchanged, and owner-service truth remains authoritative.

## Phase 9: Integrations, Operations, Metering, Observability, And Scale Hardening

### Work Items

- **9.1 Integrate with owner services and client surfaces.**
  - Design: Connect through refs and authorized APIs with Overwatch, Overguard, accounting/grant services, Fraud Control, Overclaim, PIP Registry, Compliance Boundary, Incident Response, Threat Modeling, Migration Tooling, Release Strategy, Backup/Restore, Central AI, Central AI Stewardship Interface, SDK, CLI, admin UI, public pages, and native apps.
  - Output: Integration adapters, contract tests, owner-service ref mappings, client read/write capabilities, and denied-mutation tests.
  - Validation: Integration tests prove Stewardship Reporting reads refs and emits reports without mutating source services or bypassing owner-service finality.

- **9.2 Implement metering and accounting usage refs.**
  - Design: Emit usage refs for source inventory, metric aggregation, report generation, redaction review, publication, export, public serving, correction, retraction, replay, and archive serving.
  - Output: Usage events linked to report type, period, template, audience, source services, artifact refs, system-service account, and audit refs.
  - Validation: Tests prove usage refs reconcile with Overmeter/accounting expectations, public serving usage respects cache policy, and no hardcoded pricing/revenue/customer-count assumptions appear.

- **9.3 Implement observability, alerts, and steward dashboards.**
  - Design: Expose build backlog, source freshness failures, reconciliation failures, redaction failures, review queue age, publication state, correction/retraction count, public traffic, export volume, replay success, and alert rules.
  - Output: Metrics, logs, traces, alert definitions, steward/operator dashboards, Overwatch evidence refs, and runbook links.
  - Validation: Tests or simulations trigger alerts for publication without required review, public artifact restricted-data detection, reconciliation mismatch, source freshness failure, stale public report, overdue correction, and required retraction.

- **9.4 Implement retention, archival, and incident hooks.**
  - Design: Enforce retention for public archive, private reports, private exports, review records, replay bundles, correction records, and retraction notices, with incident hooks for escaped restricted data or publication errors.
  - Output: Retention policies, archive jobs, incident hook records, private export expiry jobs, replay retention refs, and audit evidence.
  - Validation: Retention tests prove public notices remain durable, private evidence refs remain policy-scoped, expired private exports are inaccessible, and escaped-artifact incidents create correction/retraction paths.

- **9.5 Perform threat/security/compliance/scale hardening.**
  - Design: Threat model report redaction failure, source spoofing, evidence-ref tampering, metric manipulation, reviewer bypass, publication forgery, correction rewrite, archive deletion, public index poisoning, export leakage, replay tampering, and denial-of-service against report serving.
  - Output: Threat model entries, mitigation refs, security review findings, compliance review findings, scale-test results, reliability drill evidence, and release gate refs.
  - Validation: Phase 13 hardening gates pass before broad public reporting; unresolved high-severity findings block publication expansion or require explicit accepted-risk records.

## Phase 10: Alignment, Validation, Queue Handoff, And Implementation Readiness

### Work Items

- **10.1 Validate SDS and service-catalog alignment.**
  - Design: Confirm SDS #80, service catalog plan, master build plan, Phase 13 plan, crosswalk, and this sub-build plan agree on purpose, first build phase, dependencies, non-goals, owner-service boundaries, data model, APIs, events, review gates, privacy thresholds, and handoffs.
  - Output: Alignment checklist and any required doc corrections.
  - Validation: Review finds no conflicting first-build phase, no missing sub-build backlinks, no stale open questions, and no contradiction between service authority and owner-service truth.

- **10.2 Validate tech-stack guardrails.**
  - Design: Scan the plan and linked docs for conventional cloud product boundaries, blockchain/NFT/token-voting assumptions, pricing/revenue/customer-count assumptions, grant/ledger/fraud/incident/PIP/compliance owner takeover, and BLAKE3-as-encryption mistakes.
  - Output: Stack guardrail evidence with allowed negative-control matches documented where terms are explicitly rejected.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, compliance SaaS, blockchain, NFT, token voting, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate sub-build plan structure.**
  - Design: Check title prefix, attached SDS link, source alignment table, master phase mapping, tech-stack guardrails, phases 1 through 10, five work items per phase, and Design/Output/Validation structure for every work item.
  - Output: Structure validation evidence for this file.
  - Validation: Focused checks pass for 10 phase headings, 50 work items, 50 Design lines, 50 Output lines, 50 Validation lines, local Markdown links, no broken fences, final newline, and no tabs.

- **10.4 Update queue and progress metadata.**
  - Design: Mark `080-build-plan` complete, move the next incomplete build-plan task to SDS #81, and record validation evidence in build-plan progress and queue progress.
  - Output: Updated `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, and `docs/build_plan/progress.md`.
  - Validation: JSON validation passes and queue progress reports `081-build-plan` as the next incomplete build-plan task with counts adjusted for one additional completed task and zero running tasks.

- **10.5 Reindex and hand off implementation readiness.**
  - Design: Refresh Docdex index for the new plan and linked docs, run retrieval checks, run pre-commit validation, and document the test-runner status.
  - Output: Docdex index/search/DAG evidence, symbols evidence, pre-commit result, test-runner result or blocker, and final handoff summary.
  - Validation: Docdex search returns the new #80 sub-build plan and linked SDS/service/Phase 13/crosswalk evidence; pre-commit passes; `docdexd run-tests` either passes or records the known missing `.docdex/run-tests.json` blocker.
