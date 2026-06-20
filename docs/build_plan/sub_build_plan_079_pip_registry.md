# SUB BUILD PLAN #79 - Protocol Improvement Proposal Registry

Attached SDS: [SDS #79 - Protocol Improvement Proposal Registry](../sds/governance_ops/pip_registry.md)

## Purpose

This sub-build plan turns SDS #79 into an implementation sequence for the Protocol Improvement Proposal Registry. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Protocol Improvement Proposal Registry is the durable governance registry for non-trivial protocol, schema, service-contract, state-machine, compatibility, security, privacy, compliance, accounting, migration, rollback, public-builder, native-app, AI-governance, federation, mobile, and public-reporting changes. It records proposals, immutable versions, review evidence, decisions, implementation links, migration and rollback refs, supersession/deprecation/correction records, public redacted views, and replay bundles. It does not implement changes, deploy releases, run token voting, mutate owner-service state, replace security or compliance review, or hide governance decisions inside private admin paths.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #79: Protocol Improvement Proposal Registry](../sds/governance_ops/pip_registry.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflow, state machines, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoff, and resolved open-question decisions. |
| [Protocol Improvement Proposal Registry plan](../service_catalog/governance_ops/pip_registry.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order and keeps PIP Registry as a Phase 13 governance service. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies Rust workspace rules, shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, stable errors, signed envelopes, trace ids, idempotency keys, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry records, Overwatch audit refs, Overqueue primitives, and service account boundaries. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Policy Dry-Run previews, Oververify evidence, Overclaim dispute refs, Challenge Task refs, Workload Classifier facts, and security/trust review prerequisites. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill/Overgrant/Overasset/Provider Payout impact refs, dispute refs, and structural economic-impact boundaries. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies product, adapter, AI routing, SDK, CLI, and admin/developer client consumers that may create, read, or link PIPs through authorized APIs. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies grid-resident system-service evidence, founder-hardware fallback evidence, restore/failover/rollback drill refs, and backbone change inputs for PIPs. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase records, Overstore artifact refs, Overvault grants, Universal Namespace refs, Overmesh routes, native storage/vault boundaries, and data-retention implications for PIP changes. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies Deployment Planner, Release Strategy, Package Validator, rollout/rollback refs, version pins, package evidence, release gates, and implementation-link targets. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies federation template, public-interest pool, purpose tag, cross-tenant grant, stewardship, and public-benefit implications for PIP review. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider onboarding, public sandbox, fraud, reputation, challenge, payout-hold, and public-safe workload constraints. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies native app, wallet, assistant, search, messaging, social, maps, workspace, mobile, and stewardship-interface consumers of accepted changes, notices, and public PIP views. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Controls the first full build point for PIP Registry, including governance process, domain reviews, public reporting, threat/security review, compliance, emergency retrospectives, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #79 aligned to Phase 13 with prerequisites from earlier phases, without changing master phase order. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first services, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where service boundaries exist, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript only for client surfaces, native Overrid service boundaries, and no conventional database/object-store/vault/queue/compliance SaaS product boundary, blockchain, NFT, hardcoded pricing, revenue, or customer-count assumptions. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 9, 12, and 13 | Attach SDS #79, freeze PIP Registry authority, and map the affected-domain governance surface. |
| 2 | Master Phases 0, 1, 4, 5, 8, 9, and 13 | Define Rust contracts, canonical schemas, immutable records, state machines, stable errors, and fixtures. |
| 3 | Master Phases 0, 1, 4, and 13 | Implement draft creation, immutable versions, section completeness, author identity, and lifecycle gates. |
| 4 | Master Phases 4, 5, 8, 9, 10, 11, 12, and 13 | Implement affected-domain routing, review assignments, findings, waivers, conflict markers, and reviewer evidence. |
| 5 | Master Phases 1, 4, 5, 9, and 13 | Implement decisions, acceptance/rejection/withdrawal, implementation readiness, and status transitions. |
| 6 | Master Phases 0, 1, 4, 5, 8, 9, 10, 11, 12, and 13 | Link accepted PIPs to protocol/schema/service/docs, tests, release, migration, rollback, public reports, and evidence refs. |
| 7 | Master Phases 10, 11, 12, and 13 | Implement redacted public views, publication review, corrections, retractions, builder archive, and public-history projections. |
| 8 | Master Phases 7, 9, and 13 | Implement supersession, deprecation, rollback-required states, emergency retrospective PIPs, and break-glass evidence. |
| 9 | Master Phase 13 with evidence from Phases 0 through 12 | Implement metering, observability, replay, reports, integrations, threat/security review, and scale hardening. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, queue/progress metadata, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- PIP Registry uses Rust-first contracts and service APIs for PIP records, versions, sections, review assignments, findings, decisions, implementation links, migration/rollback refs, supersession records, public views, and replay bundles. TypeScript is acceptable only for generated client, admin, stewardship, SDK, CLI, or native-app surfaces that call authorized Overrid APIs.
- Contracts, lifecycle states, stable errors, replay bundles, redacted views, section schemas, review evidence, implementation links, public summaries, and deterministic fixtures use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/system scope, trace id, idempotency key, author/steward/reviewer role refs, conflict-of-interest refs where applicable, policy refs, evidence refs, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for immutable versions, section bodies, evidence manifests, implementation-link manifests, public-view snapshots, replay bundles, fixture inputs, and deterministic comparison artifacts. BLAKE3 must not be described as encryption.
- PIP Registry stores governance records and refs through native Overrid boundaries such as Overbase, Overstore, Overvault, Overqueue, Overmesh, Overregistry, Overwatch, Overguard, Compliance Boundary Service, Threat Modeling and Security Review Tracker, Migration Tooling, Release Strategy Service, Incident Response Service, Stewardship Reporting Service, ORU/Seal Ledger/Overbill/Overgrant/Overasset/Provider Payout, Central AI Service, Central AI Stewardship Interface, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Elasticsearch, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, compliance SaaS, ticketing SaaS, DAO mechanics, token voting, blockchain, NFTs, external payment custody, hardcoded pricing, revenue forecasts, customer-count assumptions, direct protocol mutation, direct schema mutation, direct service mutation, direct deployment ownership, direct migration execution, or direct public-report publication the PIP Registry authority.

## Phase 1: SDS Attachment, Scope Boundary, And Governance Authority

### Work Items

- **1.1 Attach the build plan to SDS #79.**
  - Design: Link this document from the PIP Registry SDS, service plan, master build plan, Phase 13 plan, and build-plan crosswalk so builders can move from governance scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/governance_ops/pip_registry.md`, `docs/service_catalog/governance_ops/pip_registry.md`, `docs/build_plan/master_plan.md`, `docs/build_plan/phase_13_governance_compliance_scale_hardening.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #79 returns both the PIP Registry SDS and this sub-build plan.

- **1.2 Preserve Phase 13 as the first full build point.**
  - Design: Keep PIP Registry in Phase 13 because it depends on accepted protocol/schema contracts, identity, tenancy, policy, accounting refs, storage/private refs, release/migration tooling, incident evidence, compliance boundaries, native-app surfaces, and public reporting.
  - Output: Phase-gate note that earlier phases provide evidence inputs and client surfaces, while full PIP lifecycle authority starts in Phase 13.
  - Validation: Review proves the plan does not move governance acceptance, public publication, emergency permanence, or normative protocol-change authority into earlier phases.

- **1.3 Freeze PIP Registry ownership boundaries.**
  - Design: Record that PIP Registry owns proposal records, immutable versions, sections, completeness checks, review assignment state, findings, decisions, implementation links, migration/rollback refs, supersession/deprecation/correction records, public views, and replay bundles.
  - Output: Ownership checklist for APIs, records, events, operations, review gates, public projections, and handoffs.
  - Validation: Review confirms the registry does not implement protocol changes, mutate schemas or services, deploy releases, execute migrations, replace owner-service decisions, publish raw private evidence, or run voting/token-governance systems.

- **1.4 Carry forward resolved SDS #79 decisions.**
  - Design: Preserve the resolved rules for when a PIP is required, domain-derived reviews, conflict-of-interest evidence, emergency break-glass scope, retrospective PIPs, public redaction, and permanent-change gates.
  - Output: Resolved-decision checklist covering non-trivial change triggers, ordinary issue/task exceptions, required review roles, emergency incident records, retrospective PIP deadlines, and accepted-risk handling.
  - Validation: Review rejects silent permanent emergency rules, missing compatibility/migration/rollback sections, token-vote influence, hidden admin decisions, and public exposure of private or embargoed evidence.

- **1.5 Map upstream and downstream service boundaries.**
  - Design: Record how Protocol Core, Shared Schema Package, SDS/catalog maintainers, Security Review Tracker, Compliance Boundary, Migration Tooling, Release Strategy, Incident Response, Stewardship Reporting, Overwatch, Central AI, native apps, SDK, CLI, and Admin/Developer UI interact through refs and signed commands.
  - Output: Boundary matrix naming allowed reads, owned writes, required refs, denied direct mutations, policy refs, evidence refs, redaction classes, usage refs, audit refs, and owner-service finality.
  - Validation: Review confirms PIP Registry exchanges records, refs, reports, and events without taking over implementation, release, migration, security, compliance, accounting, or public-report owner authority.

## Phase 2: Contracts, Schemas, State Machines, Stable Errors, And Fixtures

### Work Items

- **2.1 Create the PIP Registry Rust contract module.**
  - Design: Add contract types for PIP records, versions, sections, review assignments, findings, decisions, implementation links, migration/rollback refs, supersession records, public views, replay bundles, stable errors, lifecycle states, and event payloads.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, domain enums, review-role enums, visibility/redaction classes, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from protocol mutation, release execution, migration execution, raw security evidence, and public-report publication.

- **2.2 Define proposal, version, and section schemas.**
  - Design: Model `pip_record`, `pip_version`, and `pip_section` with author refs, steward refs, affected domains, affected services, affected schemas, content hashes, section types, completeness status, evidence refs, redaction class, and reviewer notes.
  - Output: JSON Schema files, valid examples, invalid examples, stable error mapping, and fixtures for protocol, schema, service-contract, native-app, AI-governance, accounting, compliance, federation, mobile, and migration changes.
  - Validation: Schema tests reject missing motivation/specification/security/privacy/compatibility/migration/rollback sections when required, missing affected-service refs, unversioned sections, mutable submitted versions, and raw private payloads in public sections.

- **2.3 Define review, finding, decision, and implementation-link schemas.**
  - Design: Model review assignments, reviewer findings, decision records, implementation links, migration/rollback refs, release refs, test refs, report refs, waiver refs, accepted-risk refs, and conflict-of-interest markers.
  - Output: Schema set, lifecycle enums, role-derived review requirements, HTTP/API mapping, event mapping, deterministic examples, and golden replay fixtures.
  - Validation: Tests prove affected domains require the right review roles and that decisions cannot become accepted or implemented without required reviews, reason codes, implementation refs, migration refs, rollback refs, and Overwatch evidence.

- **2.4 Define state machines and stable errors.**
  - Design: Preserve SDS #79 PIP, finding, and publication lifecycles and map every transition to allowed actors, required evidence, idempotency behavior, redaction requirements, public-view behavior, and rollback/supersession safety.
  - Output: Transition matrix for `draft`, `submitted`, `triage`, `review_required`, domain review states, `changes_requested`, `accepted`, `rejected`, `withdrawn`, `implemented`, `superseded`, `deprecated`, `rollback_required`, and `rolled_back`, plus stable errors.
  - Validation: Tests reject invalid transitions, duplicate side effects, missing idempotency keys, finalization without audit refs, public publication without redaction review, and `implemented` without implementation/migration/rollback/test refs.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for draft creation, version submission, completeness failure, review assignment, finding remediation, accepted-risk waiver, acceptance, implementation linking, public redaction, correction, supersession, emergency retrospective PIP, rollback-required state, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected stable errors, BLAKE3/content hashes, usage refs, audit refs, public projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, redaction behavior, required-review behavior, usage refs, audit refs, and replay output across repeated runs.

## Phase 3: Draft Intake, Immutable Versions, Completeness, And Authorized Reads

### Work Items

- **3.1 Implement draft PIP creation.**
  - Design: Accept draft PIPs only with title, summary, domain, author refs, affected domains, affected services/schemas where known, initial sections, visibility class, trace id, idempotency key, and author signature.
  - Output: `POST /pips`, draft records, section records, author audit events, stable errors, and authorized read projections.
  - Validation: Tests reject missing author refs, unsigned envelopes, missing scope, unsupported domain, invalid visibility class, duplicate idempotency conflicts, and draft content that includes raw secrets or private evidence in public fields.

- **3.2 Implement immutable version submission.**
  - Design: Store each submitted version as append-only content with section hashes, version numbers, change summaries, author signatures, supersedes-prior refs, review-reset flags, and completeness results.
  - Output: `POST /pips/{pip_id}/versions`, immutable version records, hash manifests, version timeline projections, audit refs, and stable errors.
  - Validation: Tests prove submitted versions cannot be edited, version ids are stable, content hash mismatches fail, and new versions reset or preserve reviews according to changed section and affected-domain rules.

- **3.3 Implement section completeness validation.**
  - Design: Evaluate required sections based on affected domains and change type, including motivation, specification, security impact, privacy impact, structural economic impact, compatibility, migration, rollout, rollback, alternatives, and evidence refs.
  - Output: Completeness evaluator, missing-section diagnostics, stable errors such as `pip_section_missing`, and review-readiness summaries.
  - Validation: Tests prove protocol/schema/service-contract changes require compatibility, migration, rollback, and affected-service sections, while small typo/test-only changes can remain ordinary task evidence outside full PIP flow.

- **3.4 Implement authorized reads and list filters.**
  - Design: Provide private, reviewer, steward, owner-service, public-summary, and builder-facing projections with redaction classes, state filters, domain filters, affected-service filters, implementation-status filters, and publication-state filters.
  - Output: `GET /pips/{pip_id}`, `GET /pips`, projection schemas, access-denied errors, pagination, and deterministic sort behavior.
  - Validation: Tests prove each audience sees only allowed fields and that embargoed vulnerability detail, private user data, fraud heuristics, raw AI context, and sensitive compliance evidence are not exposed.

- **3.5 Implement submission and triage entry.**
  - Design: Move complete drafts into submitted/triage states, assign steward ownership, freeze submitted content, and create triage tasks for affected-domain review routing.
  - Output: `POST /pips/{pip_id}/submit`, triage records, steward assignment refs, Overqueue task refs, Overwatch audit refs, and stable errors.
  - Validation: Tests prove incomplete PIPs cannot enter triage, duplicate submit calls are idempotent, and submitted PIPs preserve immutable version history.

## Phase 4: Affected-Domain Routing, Review Assignments, Findings, And Waivers

### Work Items

- **4.1 Implement affected-domain review routing.**
  - Design: Derive review roles from affected domains including protocol/schema, security, identity, tenancy, policy, vault, public-provider, package, privacy, retention, deletion, jurisdiction, mobile, user content, accounting, migration, release, AI, native apps, and stewardship surfaces.
  - Output: Routing evaluator, required/optional review-role records, conflict-of-interest requirements, owner-service refs, and review-readiness blockers.
  - Validation: Tests prove every SDS #79 resolved review rule maps to required reviewers and that missing or stale affected-domain declarations block review.

- **4.2 Implement review assignment APIs.**
  - Design: Allow stewards to assign reviewers by role with due windows, required/optional flags, conflict-of-interest markers, replacement history, and signed assignment envelopes.
  - Output: `POST /pips/{pip_id}/reviews`, assignment records, due-window metrics, notification refs, audit events, and stable errors.
  - Validation: Tests reject unauthorized assignment, duplicate required-role assignment conflicts, missing conflict markers, invalid due windows, and assignment of reviewers who lack the required scope.

- **4.3 Implement review findings and approvals.**
  - Design: Allow reviewers to submit findings, approvals, required changes, remediations, waived findings, accepted-risk records, severity, affected section refs, evidence refs, and final status.
  - Output: Review finding records, finding lifecycle transitions, remediation refs, waiver refs, accepted-risk refs, audit events, and stable errors.
  - Validation: Tests prove open required findings block acceptance, waived or accepted-risk findings require steward authority and reason codes, and final finding history remains append-only.

- **4.4 Implement cross-domain conflict handling.**
  - Design: Detect conflicting reviews, reviewer unavailability, stale evidence, mutually incompatible findings, and review-scope gaps, then route them to steward decision or author revision.
  - Output: Conflict diagnostics, reviewer replacement records, changes-requested state, accepted-risk escalation refs, and stable errors.
  - Validation: Tests prove conflicting required reviews cannot silently pass and that replacements preserve assignment history and audit refs.

- **4.5 Implement reviewer and author collaboration projections.**
  - Design: Expose role-aware views for comments, findings, missing evidence, section diffs, impacted services, review status, and allowed next actions without leaking private review evidence to public audiences.
  - Output: Reviewer dashboard projection, author action list, steward overview, redacted public status, and SDK/CLI response shapes.
  - Validation: Tests prove review dashboards include actionable blockers and public summaries reveal status without embargoed or private details.

## Phase 5: Decisions, Acceptance, Rejection, Withdrawal, And Implementation Readiness

### Work Items

- **5.1 Implement decision records.**
  - Design: Record accept, reject, withdraw, return-for-changes, supersede, deprecate, rollback-required, and correction decisions with deciding body refs, required review refs, reason codes, conditions, effective state, and audit refs.
  - Output: `POST /pips/{pip_id}/decisions`, decision records, state transitions, reason-code catalog, audit events, and stable errors.
  - Validation: Tests reject unauthorized decisions, decisions with unresolved required findings, missing reason codes, missing deciding body refs, and transitions that conflict with immutable version state.

- **5.2 Implement acceptance readiness gates.**
  - Design: Require complete reviews, compatibility analysis, migration plan refs, rollback plan refs, affected-service owner acknowledgement, security/privacy/compliance/accounting findings where needed, and release/implementation owner acknowledgement before acceptance.
  - Output: Acceptance-readiness evaluator, blocker report, go/no-go checklist, required ref list, and stable errors such as `review_required`, `migration_plan_required`, and `rollback_plan_required`.
  - Validation: Tests prove accepted PIPs cannot bypass required domain reviews or lack migration, rollback, compatibility, implementation, or public-reporting conditions where affected domains require them.

- **5.3 Implement rejection, withdrawal, and return-for-changes flows.**
  - Design: Preserve rejected and withdrawn proposals as historical records, allow return-for-changes without deleting evidence, and link replacement versions or alternatives when authors revise.
  - Output: State transitions, author/steward reason records, alternative refs, public/private status projections, and replay events.
  - Validation: Tests prove rejected, withdrawn, and returned PIPs remain traceable and do not lose review, decision, or evidence history.

- **5.4 Implement implementation-ready and implemented gates.**
  - Design: Separate acceptance from deployment by requiring implementation refs, schema/protocol version refs, SDS/catalog updates, test refs, release refs, migration refs, rollback refs, and public reporting refs before `implemented`.
  - Output: Implementation-readiness state, `implementation_link_missing` errors, owner-service ack refs, and implemented-state transition.
  - Validation: Tests prove accepted PIPs with no implementation evidence cannot be marked implemented and that implementation evidence is append-only and replayable.

- **5.5 Implement decision event emission and audit timeline.**
  - Design: Emit decision, review, acceptance, rejection, implementation-link, rollback-required, supersession, deprecation, and public-view events with trace ids, PIP ids, version ids, affected domains, and audit refs.
  - Output: Event payloads, Overwatch audit timeline, replay inputs, usage refs, and durable event fixtures.
  - Validation: Tests prove events exclude private/embargoed details and can reconstruct decision history from stored refs.

## Phase 6: Implementation Links, Migration/Rollback Refs, Release Evidence, And Documentation Updates

### Work Items

- **6.1 Implement implementation-link APIs.**
  - Design: Attach implementation tasks, repo refs, schema/protocol version refs, service SDS refs, service catalog refs, tests, release refs, migration refs, rollback refs, and report refs to accepted PIPs.
  - Output: `POST /pips/{pip_id}/implementation-links`, link records, verification state, duplicate-link handling, and audit events.
  - Validation: Tests reject links from unauthorized owners, malformed refs, stale schema versions, missing evidence hashes, and links that imply deployment before owning release services confirm.

- **6.2 Implement migration and rollback reference handling.**
  - Design: Link PIPs to Migration Tooling, Release Strategy, Backup and Restore, Failover and Recovery, Deployment Planner, Package Validator, Overmesh, Overbase, Overstore, Overvault, Overqueue, and accounting refs without executing those services.
  - Output: Migration/rollback ref records, compatibility-window metadata, fallback refs, release gate refs, and stable errors.
  - Validation: Tests prove PIP Registry stores refs only, requires owning-service evidence, and blocks implemented state when rollback or migration refs are missing for changes that can affect running systems.

- **6.3 Implement documentation update evidence.**
  - Design: Require accepted PIPs that change contracts to link updated protocol docs, shared schema, SDS files, service catalog files, build-plan crosswalk entries, and public builder docs where applicable.
  - Output: Documentation-link records, stale-doc blockers, compatibility notes, builder notice refs, and verification state.
  - Validation: Tests prove service-contract PIPs cannot become implemented when SDS/catalog/schema docs remain missing or stale.

- **6.4 Implement test and conformance evidence.**
  - Design: Link accepted changes to conformance fixtures, integration tests, migration tests, rollback tests, security tests, privacy/redaction tests, accounting reconciliation tests, and release-gate evidence.
  - Output: Test evidence records, fixture hash refs, result refs, failure refs, and readiness summaries.
  - Validation: Tests prove acceptance and implementation reports include required test refs and cannot mark failed or stale tests as passing evidence.

- **6.5 Implement downstream handoff exports.**
  - Design: Export accepted change records, compatibility notes, migration windows, rollback refs, implementation refs, and public summaries for SDK, CLI, admin/developer UI, native apps, external builders, and documentation pipelines.
  - Output: Handoff APIs, projection schemas, builder timeline views, SDK/CLI response fixtures, and redacted public data.
  - Validation: Tests prove builder-facing exports are complete enough to act on while excluding private, embargoed, or restricted evidence.

## Phase 7: Public Archive, Redaction Review, Corrections, Retractions, And Builder Views

### Work Items

- **7.1 Implement redaction review workflow.**
  - Design: Require public views to pass redaction review for private data, fraud heuristics, exploit details, sensitive topology, payment details, regulated evidence, raw AI context, and private review notes.
  - Output: `POST /pips/{pip_id}/publish`, redaction review state, publication blockers, redaction profiles, and public-view manifests.
  - Validation: Tests prove public publication fails when restricted evidence appears in public fields or redaction review is missing.

- **7.2 Implement public PIP archive.**
  - Design: Publish public-readable proposal summaries, accepted/rejected/withdrawn/superseded/deprecated states, correction notices, migration windows, compatibility notes, public implementation status, and builder guidance.
  - Output: Public archive projections, filters by state/domain/affected service/version/publication status, stable URLs or namespace refs, and pagination.
  - Validation: Tests prove public archive queries are deterministic, searchable, redaction-aware, and free of private evidence.

- **7.3 Implement correction and retraction flows.**
  - Design: Allow public-view correction, withdrawal, withholding, or retraction with reason codes, prior publication refs, corrected public snapshots, and incident handoff when redaction fails.
  - Output: Correction/retraction records, publication timeline, Overwatch events, Incident Response handoff refs, and public notices.
  - Validation: Tests prove bad public redaction can be retracted/corrected without deleting the historical evidence chain.

- **7.4 Implement builder and owner-service views.**
  - Design: Provide builder-facing accepted-change views, migration windows, compatibility windows, deprecations, schema/service version refs, implementation status, and affected-service ownership.
  - Output: Builder APIs, owner-service dashboards, SDK/CLI projections, and native-app notification payloads.
  - Validation: Tests prove builder views distinguish accepted, implemented, superseded, deprecated, rollback-required, and rolled-back states.

- **7.5 Implement public-reporting handoff.**
  - Design: Link public PIP views to Stewardship Reporting Service, Central AI Stewardship Interface, Compliance Boundary Service, Incident Response, and public documentation without giving PIP Registry report-publication ownership.
  - Output: Report refs, publication-state refs, redacted summary bundles, and report handoff events.
  - Validation: Tests prove report owners receive sufficient refs and PIP Registry does not publish external reports or expose restricted evidence directly.

## Phase 8: Supersession, Deprecation, Rollback-Required States, And Emergency Retrospectives

### Work Items

- **8.1 Implement supersession and deprecation records.**
  - Design: Link replaced PIPs to replacements, reasons, migration notes, deprecation windows, compatibility notes, public notice refs, and implementation status.
  - Output: Supersession records, deprecation records, replacement graphs, public/private projections, and replay events.
  - Validation: Tests prove superseded and deprecated PIPs remain traceable and cannot be deleted or overwritten.

- **8.2 Implement rollback-required and rolled-back states.**
  - Design: Record rollback-required decisions, rollback plan refs, owner-service rollback evidence, corrective PIP refs, implementation status, public summary status, and incident links.
  - Output: Rollback-required state transitions, rollback records, corrective PIP links, Incident Response handoff refs, and stable errors.
  - Validation: Tests prove rollback state cannot be cleared without owner-service rollback evidence or accepted corrective action.

- **8.3 Implement emergency break-glass records.**
  - Design: Support incident-driven emergency changes that name the incident, affected scope, owning service, temporary authority, expiry, rollback/fallback path, and Overwatch evidence refs.
  - Output: Emergency records, temporary-effectiveness windows, expiry checks, incident refs, retrospective-PIP requirements, and audit events.
  - Validation: Tests prove emergency changes cannot silently become permanent and must link retrospective PIPs before broad rollout or founder/backbone fallback removal.

- **8.4 Implement retrospective PIP flow.**
  - Design: Create retrospective PIPs from emergency records with incident timelines, containment refs, affected-service decisions, security/privacy/compliance/accounting findings, tests, migration/rollback evidence, public-report refs, and rejected alternatives.
  - Output: Retrospective PIP drafts, required-section prefill, review requirements, blocker states, and accepted-risk refs.
  - Validation: Tests prove unresolved severe findings keep retrospective PIPs in review, rollback-required, or accepted-risk state rather than implemented.

- **8.5 Implement duplicate and related-PIP handling.**
  - Design: Detect duplicate proposals, overlapping affected services, related compatibility windows, replacement candidates, and conflicting deprecation notes.
  - Output: Related-PIP links, duplicate diagnostics, supersession recommendations, steward review tasks, and public related-change projections.
  - Validation: Tests prove duplicate proposals are linked or superseded instead of deleted and that public/private history remains consistent.

## Phase 9: Metering, Observability, Reporting, Integrations, Security, And Scale Hardening

### Work Items

- **9.1 Implement usage metering and accounting refs.**
  - Design: Emit usage refs for proposal storage, version submission, review work, publication, public archive reads, report generation, replay, governance dashboards, and stewardship/system-service operations.
  - Output: Usage event schema, Overmeter refs, plan/session attribution, structural economic-impact refs, and reconciliation fixtures.
  - Validation: Tests prove governance overhead is visible as system-service usage where applicable and PIP Registry does not create pricing, revenue, or customer-count assumptions.

- **9.2 Implement operational metrics and alerts.**
  - Design: Track draft age, missing sections, overdue required reviews, blocked findings, accepted PIPs without implementation links, implemented changes without public views, emergency records nearing expiry, redaction failures, and replay failures.
  - Output: Metrics schema, alert rules, operator/steward dashboard refs, escalation records, and runbook notes.
  - Validation: Drills prove overdue reviews, missing rollback plans, emergency expiry, redaction failure, and implementation-link gaps alert and create timeline evidence.

- **9.3 Implement replay bundles and audit reconstruction.**
  - Design: Reconstruct proposal versions, section hashes, review assignments, findings, decisions, implementation links, migration refs, publication refs, usage refs, and Overwatch events.
  - Output: `GET /pips/{pip_id}/replay`, replay bundle schema, deterministic replay fixtures, audit exports, and stable errors.
  - Validation: Tests prove replay reconstructs the full history without needing mutable live state and excludes restricted evidence from unauthorized replay views.

- **9.4 Integrate governance services and clients.**
  - Design: Connect PIP Registry with Threat Modeling and Security Review Tracker, Compliance Boundary, Incident Response, Migration Tooling, Release Strategy, Stewardship Reporting, Central AI, Central AI Stewardship Interface, SDK, CLI, admin/developer UI, and native-app status consumers through refs and authorized APIs.
  - Output: Adapter contracts, event subscriptions, projection endpoints, handoff records, and integration fixtures.
  - Validation: Integration tests prove each service receives only refs it owns or can read and PIP Registry does not become a private evidence sink.

- **9.5 Run threat modeling and security review gates.**
  - Design: Review governance capture, hidden admin acceptance, forged reviewer signatures, stale affected-domain declarations, redaction failure, public archive leakage, emergency authority abuse, token-vote drift, implementation-link spoofing, and replay tampering.
  - Output: Threat model entries, security review findings, mitigations, owners, acceptance criteria, tests, and residual-risk records.
  - Validation: Review confirms each high-risk finding has a mitigation, test, monitor, accepted-risk record, or blocker before broad PIP Registry use.

## Phase 10: Validation, Documentation, Queue State, And Implementation Handoff

### Work Items

- **10.1 Validate sub-build-plan structure and backlinks.**
  - Design: Check the `SUB BUILD PLAN #79` title, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, and backlinks across SDS, service catalog, master plan, crosswalk, and Phase 13 docs.
  - Output: Validation report covering local Markdown links, heading order, work-item counts, backlink presence, final newlines, and tab checks.
  - Validation: Focused structure checks, Markdown link checks, JSON validation, and `git diff --check` pass.

- **10.2 Validate tech-stack guardrails and authority boundaries.**
  - Design: Scan the changed docs for accidental conventional product boundaries, direct mutation authority, compliance-SaaS assumptions, token governance, blockchain/NFT wording, pricing/revenue/customer-count assumptions, and BLAKE3-as-encryption mistakes.
  - Output: Guardrail scan report with negative-control explanations for allowed prohibition wording.
  - Validation: Review proves the plan respects `docs/overrid_tech_stack_choice.md` and native Overrid service boundaries.

- **10.3 Validate queue and progress metadata.**
  - Design: Mark `079-build-plan` complete, move the next incomplete build-plan task to SDS #80, and record validation evidence in build-plan progress and queue progress.
  - Output: Updated `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, and `docs/build_plan/progress.md`.
  - Validation: JSON validation passes and queue progress counts match the state file.

- **10.4 Refresh Docdex and verify retrieval.**
  - Design: Reindex the new sub-build-plan file and linked SDS/service/build-plan docs so future agents retrieve the aligned plan.
  - Output: Docdex targeted index refresh, search result for SDS #79, symbol extraction for the new plan, and DAG export for the final search session.
  - Validation: Docdex search returns the SDS #79 sub-build plan in the result set with SDS/service/master/crosswalk/Phase 13 evidence, and Docdex stats reflect the indexed document.

- **10.5 Prepare implementation handoff.**
  - Design: Summarize the SDS #79 implementation order, prerequisites, authority boundaries, test strategy, known blockers, and validation evidence for the next build or phase-execution task.
  - Output: Handoff note in build-plan progress and Docdex memory covering the final aligned scope.
  - Validation: Review confirms the handoff names Phase 13 first build, required owner services, no direct mutation boundaries, no token-voting mechanics, no public evidence leakage, and test/validation prerequisites without unresolved alignment gaps.
