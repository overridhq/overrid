# SUB BUILD PLAN #76 - Compliance Boundary Service

Attached SDS: [SDS #76 - Compliance Boundary Service](../sds/governance_ops/compliance_boundary_service.md)

## Purpose

This sub-build plan turns SDS #76 into an implementation sequence for Compliance Boundary Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Compliance Boundary Service is the Phase 13 compliance fact and boundary-definition layer for rulesets, marker taxonomy, jurisdiction profiles, regulated-scope records, boundary evaluations, signed fact bundles, exception records, jurisdiction updates, compliance exports, and replay bundles. It does not provide legal advice, process payments, hold custody, enforce policy directly, store raw secrets or private evidence, mutate accounting/vault/tenant/payout/incident records, or replace owner-service authority.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #76: Compliance Boundary Service](../sds/governance_ops/compliance_boundary_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, states, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoffs, and resolved open-question decisions. |
| [Compliance Boundary Service plan](../service_catalog/governance_ops/compliance_boundary_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed command envelopes, trace ids, idempotency keys, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, and Overqueue command/event primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Overclaim dispute refs, Policy Dry-Run previews, Oververify evidence, reason codes, deny-by-default behavior, and replayable decisions. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill accounting truth, Overgrant grant refs, Provider Payout refs, Overmark resource cards, Overclaim dispute refs, and the rule that Compliance Boundary marks boundary classes but never mutates accounting truth. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies AI Gateway Router, Personal AI Assistant, encrypted RAG, adapter, SDK, CLI, admin, and mSwarm Runtime Bridge groundwork used for AI-context and protected-context boundary checks. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase records, Overstore artifact/evidence refs, Overvault sensitive/private refs, Universal Namespace refs, retention/deletion substrates, and replay storage boundaries. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies federation templates, purpose tags, public-interest pools, and cross-tenant grant/public-interest boundary inputs. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider onboarding, fraud/reputation/challenge refs, public sandbox limits, payout holds, and public low-sensitivity boundary inputs. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies native app, wallet, AI assistant, search, messaging, social, maps, workspace, mobile, and stewardship-interface consumers of boundary facts. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Controls the first build point for Compliance Boundary Service, including governance, ruleset activation, exceptions, jurisdiction changes, exports, replay, reporting, threat review, incident response, migration, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #76 first build work aligned to master Phase 13 with earlier phases as prerequisites only. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and contracts, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where a service boundary exists, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript for client surfaces only, native Overrid service boundaries, and no conventional database/object-store/vault/queue/search/compliance SaaS product boundary, Kubernetes-first architecture, blockchain, NFT, hidden legal-advice service, hardcoded pricing, revenue, or customer-count assumptions. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 6, 8, 10, 11, 12, and 13 | Attach SDS #76, preserve Phase 13 as first build, record prerequisites, and freeze Compliance Boundary authority. |
| 2 | Master Phases 0, 1, 4, 5, 8, 11, 12, and 13 | Define Rust contracts, marker taxonomy, ruleset schemas, jurisdiction profiles, regulated scopes, stable errors, and fixtures. |
| 3 | Master Phases 0, 1, 4, and 13 | Implement ruleset lifecycle, marker registry, publication, staged rollout, signatures, compatibility checks, and rollback controls. |
| 4 | Master Phases 1, 4, 5, 6, 8, 11, 12, and 13 | Implement boundary evaluations, signed fact bundles, freshness windows, Overguard handoffs, owner-service handoffs, and policy-input replay. |
| 5 | Master Phases 4, 5, 10, 11, 12, and 13 | Implement accounting, payout, custody-like, refund/dispute, grant, and public-interest markers without mutating accounting truth. |
| 6 | Master Phases 4, 6, 8, 11, 12, and 13 | Implement privacy, child-safety, data-residency, retention, deletion, encrypted-context, AI-context, public-report, and export markers. |
| 7 | Master Phases 1, 4, 5, 7, 8, 10, 11, and 13 | Implement exception review, expiry, revocation, jurisdiction updates, migration notes, bootstrap exceptions, and affected-scope lists. |
| 8 | Master Phases 1, 8, 12, and 13 | Implement redacted exports, evidence refs, Stewardship Reporting handoffs, audit packages, replay APIs, and integrity bundles. |
| 9 | Master Phase 13 with evidence from Phases 0 through 12 | Implement operational metrics, threat/security review gates, reliability drills, incident hooks, migration hooks, and public reporting checks. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Compliance Boundary Service uses Rust-first shared contracts and service-facing APIs for boundary rulesets, markers, jurisdiction profiles, regulated scopes, evaluations, fact bundles, exceptions, jurisdiction updates, exports, and replay bundles. TypeScript is acceptable only for generated client/admin surfaces and must call Overrid APIs without becoming a compliance authority.
- Contracts, markers, ruleset metadata, jurisdiction profiles, input fact bundles, exception records, export manifests, redaction profiles, stable errors, replay bundles, and deterministic fixtures use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/system scope, trace id, idempotency key, role/steward refs, evidence refs, policy refs, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for ruleset bundles, marker registries, source/evidence refs, export manifests, replay bundles, fixture inputs, and deterministic comparison artifacts. BLAKE3 must not be described as encryption.
- Structured compliance state, evidence refs, sensitive records, private content refs, audit timelines, owner-service refs, usage refs, and replay must use native Overrid service boundaries such as Overbase, Overstore, Overvault, Overwatch, Overguard, Overbill, Provider Payout Service, ORU Account Service, Seal Ledger, Overgrant, Overtenant, Overpass, Universal Namespace Service, Incident Response Service, PIP Registry, Stewardship Reporting Service, Central AI Service, native apps, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Elasticsearch, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, a compliance SaaS, Kubernetes-first orchestration, blockchain, NFTs, external legal-advice systems, external payment custody, hardcoded pricing, revenue forecasts, customer-count assumptions, raw private-data exports, raw child-safety evidence, payment credentials, private user content, encrypted Docdex context, final policy enforcement, accounting mutation, vault mutation, tenant mutation, incident finality, or payout approval the Compliance Boundary authority.

## Phase 1: SDS Attachment, Phase 13 Scope, And Compliance Authority Boundary

### Work Items

- **1.1 Attach the build plan to SDS #76.**
  - Design: Link this document from the Compliance Boundary SDS, service plan, master build plan, Phase 13 plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/governance_ops/compliance_boundary_service.md`, `docs/service_catalog/governance_ops/compliance_boundary_service.md`, `docs/build_plan/master_plan.md`, `docs/build_plan/phase_13_governance_compliance_scale_hardening.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #76 returns both the Compliance Boundary Service SDS and this sub-build plan.

- **1.2 Preserve master Phase 13 as the first build point.**
  - Design: Keep first implementation in Phase 13 because compliance boundary facts require identity, tenancy, policy, accounting, storage/vault, federation, public-provider, native-app, reporting, and incident-response prerequisites from earlier phases.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, 6, 8, 10, 11, and 12 supply prerequisites while Phase 13 builds ruleset activation, exception review, jurisdiction changes, exports, replay, and governance hardening.
  - Validation: Review proves the plan does not move Compliance Boundary into Phase 4 policy enforcement, Phase 5 accounting mutation, Phase 8 vault/storage ownership, Phase 11 public-provider authority, or Phase 12 native-app ownership.

- **1.3 Freeze the Compliance Boundary ownership boundary.**
  - Design: Record that Compliance Boundary owns versioned rulesets, marker taxonomy, jurisdiction profiles, regulated scopes, boundary evaluations, signed fact bundles, exception records, jurisdiction updates, exports, and replay bundles.
  - Output: Ownership checklist for architecture, API, implementation, operations, governance, and review gates.
  - Validation: Review confirms the service does not own legal advice, external certification, payments, custody, refunds, payouts, ORU/Seal Ledger/Overbill truth, vault contents, tenant identity, incident finality, or Overguard enforcement decisions.

- **1.4 Carry forward resolved SDS #76 decisions.**
  - Design: Preserve legal-steward review for high-compliance marker activation, service-owner stewardship for lower-risk operational markers, minimum jurisdiction profiles, audience-classed fact visibility, dry-run staged ruleset comparison, and narrow bootstrap exceptions.
  - Output: Resolved-decision checklist covering marker families, profile minimums, public/affected-party/steward/auditor visibility, redacted staged comparisons, and exceptions that must wait for full Phase 13 governance.
  - Validation: Review rejects permissive missing-profile defaults, public leakage of private facts, raw comparison captures, long-lived bootstrap waivers, high-compliance public-node waivers, payout waivers, custody/payment waivers, and audit/replay waivers.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Record how Overguard, Workload Classifier, Policy Dry-Run API, Overbill, ORU Account Service, Seal Ledger, Provider Payout, Overgrant, Overvault, Overbase, Overstore, Overtenant, Overpass, Universal Namespace, Overwatch, Overclaim, Incident Response, Stewardship Reporting, PIP Registry, Central AI, and native apps interact through refs and fact bundles.
  - Output: Boundary matrix naming allowed reads, owned writes, required refs, denied direct mutations, redaction classes, evidence refs, usage refs, audit refs, replay refs, and owner-service finality.
  - Validation: Review confirms Compliance Boundary exchanges signed refs/events/fact bundles and never copies private internals or grants itself mutation authority owned by another service.

## Phase 2: Contracts, Marker Taxonomy, Rulesets, Jurisdiction Profiles, And Fixtures

### Work Items

- **2.1 Create the Compliance Boundary Rust contract module.**
  - Design: Add contract types for boundary rulesets, marker definitions, jurisdiction profiles, regulated scopes, boundary evaluations, compliance fact bundles, exceptions, jurisdiction updates, exports, replay bundles, stable errors, and lifecycle states.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, marker-domain enums, severity/action/redaction enums, lifecycle enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from legal advice, enforcement, payment processing, custody, vault contents, and incident finality.

- **2.2 Define marker taxonomy schemas.**
  - Design: Model stable marker codes for payment, custody-like behavior, payout, refund/dispute, grant, privacy, child safety, regulated workload, data residency, retention, deletion, AI context, public reporting, export, jurisdiction, stale input facts, and rollout state.
  - Output: JSON Schema files, marker-domain registry, severity rules, visibility classes, required-input-fact fields, affected-service fields, default action hints, deprecation states, valid examples, and invalid examples.
  - Validation: Schema tests reject unknown marker domains, missing stable codes, missing visibility class, missing required facts, unsupported severity, unversioned markers, raw secrets, payment credentials, private content, or child-safety evidence payloads.

- **2.3 Define ruleset and jurisdiction profile schemas.**
  - Design: Model `boundary_ruleset`, `jurisdiction_profile`, and `regulated_scope` with signer refs, effective windows, jurisdiction/domain selectors, source/evidence refs, supported service domains, residency/retention/deletion constraints, payment/payout constraints, and isolation requirements.
  - Output: Ruleset/profile/scope schemas, lifecycle state enums, signature metadata, source-ref examples, compatibility notes, migration notes, rollback notes, and fixture inputs for first public native apps and provider payouts.
  - Validation: Tests reject profiles missing jurisdiction id, effective window, source/evidence refs, privacy/deletion/retention rules, child-safety obligations where applicable, payment/refund/chargeback constraints, payout constraints, owning-service refs, or data-class allow/deny rules.

- **2.4 Define evaluation, fact bundle, exception, update, export, and replay schemas.**
  - Design: Model `boundary_evaluation`, `compliance_fact_bundle`, `exception_record`, `jurisdiction_update`, `compliance_export_job`, and `compliance_replay_bundle` with explicit input fact refs, owner-service refs, allowed use, freshness windows, approver refs, expiry, redaction profiles, integrity hashes, and replay refs.
  - Output: Schema set, stable error mapping, HTTP/API mapping, audience-classed projection schemas, deterministic examples, and golden replay fixtures.
  - Validation: Tests prove outputs are deterministic from stored facts/ruleset versions and cannot include raw secrets, payment credentials, identity documents, private user content, encrypted Docdex context, child-safety evidence, fraud heuristics, or exploit details.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for ruleset draft, marker registry update, jurisdiction profile creation, boundary evaluation, fact bundle creation, accounting-boundary handoff, privacy-boundary handoff, exception review, jurisdiction update, export, staged comparison, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected stable errors, BLAKE3 hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, redaction behavior, usage refs, audit refs, and replay output across repeated runs.

## Phase 3: Ruleset Lifecycle, Marker Registry, Publication, And Staged Rollout

### Work Items

- **3.1 Implement ruleset draft and schema checks.**
  - Design: Accept ruleset drafts only with domain, semantic version, jurisdiction refs, scope selectors, marker refs, source/evidence refs, signer refs, compatibility notes, rollout plan, and audit refs.
  - Output: `POST /compliance/rulesets`, draft state, schema-check state, validation errors, evidence-ref validation, signature metadata, and Overwatch audit events.
  - Validation: Tests reject missing source/evidence refs, missing signer refs, invalid marker references, missing jurisdiction/profile selectors, missing audit refs, and unscoped rulesets.

- **3.2 Implement marker registry versioning.**
  - Design: Version marker definitions append-only with deprecation/supersession state, affected services, required input facts, default action hints, public/private visibility, and redaction classes.
  - Output: Marker registry API, `GET /compliance/markers`, marker-version events, compatibility fixture updates, and migration notes for affected services.
  - Validation: Tests prove marker changes are signed, versioned, backward-compatible or explicitly breaking, and unable to silently alter prior evaluation replay.

- **3.3 Implement publication, staging, activation, pause, supersession, and revocation.**
  - Design: Publish ruleset versions through legal-steward/operator review where marker family requires it, with staged rollout windows and explicit activation controls.
  - Output: `POST /compliance/rulesets/{ruleset_id}/publish`, lifecycle transitions, review refs, staged/active/paused/superseded/revoked states, and Overwatch timeline refs.
  - Validation: Tests prove high-compliance marker families cannot activate without required steward review and signatures, and lower-risk operational markers cannot grant permission where legal-steward markers are missing, stale, denied, or review-required.

- **3.4 Implement staged ruleset dry-run comparison.**
  - Design: Replay stored boundary evaluations and explicit input fact bundles through staged rulesets in dry-run mode, then write redacted comparison records.
  - Output: Comparison job schema, marker-code deltas, decision-state deltas, reason-code deltas, affected-scope counts, representative redacted refs, migration notes, and blocker notes.
  - Validation: Tests prove comparison records do not copy raw private facts and expose only aggregate/self-scoped refs, thresholds, and stable reason codes to public or affected-party views.

- **3.5 Implement compatibility, rollback, and supersession controls.**
  - Design: Require compatibility fixtures, affected-service acceptance evidence, rollback notes, superseded-by refs, migration notes, and notification refs before broad activation.
  - Output: Compatibility gate, rollback metadata, acceptance evidence refs, notification handoff refs, and release-state diagnostics.
  - Validation: Tests prove breaking marker/ruleset changes require explicit affected-service acceptance or remain staged/blocked.

## Phase 4: Boundary Evaluations, Fact Bundles, And Overguard Handoffs

### Work Items

- **4.1 Implement boundary evaluation intake.**
  - Design: Accept evaluation requests only with actor/service identity, tenant/system scope, target refs, explicit input fact bundle refs, desired audience/consumer, trace id, idempotency key, and policy refs.
  - Output: `POST /compliance/evaluations`, submitted/facts_validated/ruleset_selected states, stale-fact errors, missing-scope errors, audit events, and usage refs.
  - Validation: Tests deny or block evaluation when identity, tenant, jurisdiction, data class, workload class, owner-service refs, or evidence refs are missing or stale.

- **4.2 Implement marker matching and exception checks.**
  - Design: Match active/staged ruleset versions and marker definitions against validated input facts, then apply only allowed active exceptions with expiry and target-scope checks.
  - Output: Matched marker refs, result states, reason codes, exception refs, denial refs, replay links, and Overwatch events.
  - Validation: Tests prove missing/expired/revoked exceptions fail closed, stricter markers win on conflict, and replays use stored ruleset versions rather than live policy state.

- **4.3 Implement signed compliance fact bundle creation.**
  - Design: Create fact bundles for Overguard and owner services with fact owners, marker refs, freshness windows, trust level, allowed use, redaction class, signature refs, and replay refs.
  - Output: `POST /compliance/fact-bundles`, signed fact bundle records, consumer-specific projections, freshness metadata, and stable errors for stale or denied facts.
  - Validation: Tests prove fact bundles are signed, bounded to allowed use, consumed as policy inputs, and unable to authorize execution without Overguard or owner-service authority.

- **4.4 Implement Overguard and policy dry-run handoffs.**
  - Design: Feed signed boundary facts to Overguard and Policy Dry-Run API as policy inputs for admission, payout, export, native-app publication, AI context use, public-provider placement, and regulated workload checks.
  - Output: Handoff envelope, Overguard decision refs, dry-run refs, reason-code mapping, denied/boundary-required states, and replay links.
  - Validation: Tests prove Compliance Boundary does not allow/deny execution directly and Overguard remains final policy-enforcement authority.

- **4.5 Implement consumer handoffs and failure recovery.**
  - Design: Deliver fact bundles to Overbill, Provider Payout, Overvault, Overtenant, Central AI, native apps, Incident Response, Stewardship Reporting, and auditors through idempotent handoff refs.
  - Output: Consumer handoff events, retry states, failure refs, owner-service response refs, and degraded diagnostics.
  - Validation: Tests simulate consuming-service outage and prove fact bundles are retained for retry without widening access or mutating owner-service records.

## Phase 5: Accounting, Payout, Custody-Like, Grant, And Public-Interest Boundary Markers

### Work Items

- **5.1 Implement payment and custody-like boundary markers.**
  - Design: Mark flows involving external payment-provider refs, account visibility, payment credentials, custody-like behavior, refunds, chargebacks, and payment-boundary reporting without processing payments or holding funds.
  - Output: Payment/custody marker family, required Overbill/ORU/Seal Ledger/Overwatch refs, reason codes, redaction classes, and compatibility fixtures.
  - Validation: Tests prove markers are explicit in accounting flows and cannot create charges, mutate balances, issue refunds, access payment credentials, or claim external regulatory certification.

- **5.2 Implement provider payout and payout-hold markers.**
  - Design: Mark provider payout eligibility, payment destination refs, tax/compliance metadata refs, provider region, fraud/anti-Sybil holds, dispute holds, allowed payout states, and public-provider payout constraints.
  - Output: Payout marker family, Provider Payout handoff refs, hold reason codes, jurisdiction profile checks, and replay fixtures.
  - Validation: Tests prove missing jurisdiction profile facts block or hold payout flows and no marker can approve payout, release holds, or mutate payout batch state.

- **5.3 Implement refund, dispute, and claim boundary markers.**
  - Design: Link Overclaim, Overbill, Seal Ledger, Provider Payout, and Overwatch refs to refund/dispute markers, correction states, hold windows, appeal refs, and public/private visibility rules.
  - Output: Refund/dispute marker family, hold-window fields, claim refs, correction refs, affected-party projections, and replay bundle fixtures.
  - Validation: Tests prove dispute markers preserve owner-service finality and cannot settle claims, rewrite ledger entries, or expose private evidence.

- **5.4 Implement grant and public-interest boundary markers.**
  - Design: Mark Overgrant, Purpose Tag Registry, Public-Interest Pool, Central AI, and Stewardship Reporting refs for public-interest funding, grant eligibility, purpose constraints, stewardship review, and redacted outcome reporting.
  - Output: Grant/public-interest marker family, purpose-tag refs, report redaction classes, stewardship handoff refs, and compatibility fixtures.
  - Validation: Tests prove public-interest markers do not allocate grants, mutate ORU accounts, decide Central AI recommendations, or expose private grant evidence.

- **5.5 Implement accounting-boundary usage and audit hooks.**
  - Design: Emit usage refs and audit refs for accounting-boundary evaluations, fact bundles, staged comparisons, exports, replays, steward review, and jurisdiction-update processing.
  - Output: Usage class registry, Overmeter handoff events, Overwatch audit events, pending reconciliation markers, and diagnostics.
  - Validation: Tests prove every billable/material accounting-boundary operation emits or reconciles usage refs without hardcoded prices, balances, invoices, resource rates, revenue forecasts, or customer-count assumptions.

## Phase 6: Privacy, Child Safety, Data Residency, Retention, Deletion, And AI Context Markers

### Work Items

- **6.1 Implement privacy and data-class markers.**
  - Design: Mark personal data, private user content, organization content, native-app content, encrypted context, identity document refs, secret refs, fraud-sensitive refs, and audience-specific visibility.
  - Output: Privacy marker family, data-class registry, owner-service refs, redaction profiles, affected-party projections, and steward/auditor projections.
  - Validation: Tests prove public facts remain limited to taxonomy summaries, active ruleset ids/versions, effective windows, supported summaries, redaction classes, templates, aggregate metrics, and correction/supersession notices.

- **6.2 Implement child-safety and regulated-workload markers.**
  - Design: Mark child-safety obligations, regulated workload classes, public-content obligations, isolation requirements, public-node denials, and special review gates without storing raw child-safety evidence.
  - Output: Child-safety/regulated marker family, strict isolation requirements, Overguard handoff refs, public-provider denial refs, and security fixtures.
  - Validation: Tests prove raw child-safety evidence, fraud heuristics, exploit details, and regulated private facts remain in owner services behind redacted refs and access policy.

- **6.3 Implement data residency, retention, deletion, and hold markers.**
  - Design: Mark residency constraints, deletion requirements, retention defaults, legal/compliance holds, public-report redaction, export expiry, tombstone behavior, and affected-scope migration requirements.
  - Output: Residency/retention/deletion marker family, cleanup/hold refs, jurisdiction profile compatibility fixtures, and replay preservation rules.
  - Validation: Tests prove reductions to deletion, retention, or data-residency obligations require legal-steward review and cannot be waived by bootstrap exceptions.

- **6.4 Implement AI context and encrypted RAG boundary markers.**
  - Design: Mark AI context use, encrypted Docdex context, RAG leakage profiles, context grants, assistant/native-app tool handoffs, Central AI review boundaries, and public/private reporting classes.
  - Output: AI-context marker family, Encrypted Docdex RAG/AI Gateway/Personal AI handoff refs, context-retention refs, leakage-profile refs, and replay fixtures.
  - Validation: Tests prove raw encrypted Docdex context is never copied into Compliance Boundary records and revoked or missing context grants fail closed.

- **6.5 Implement export and public-report redaction markers.**
  - Design: Mark export audience, purpose, included fact bundles, redaction profile, evidence refs, artifact refs, integrity hash, expiry, and public-report class before export creation.
  - Output: Export/public-report marker family, audience rules, redaction gates, public-safe summary fixtures, and denial errors.
  - Validation: Tests prove public reports cannot expose raw private data, payment credentials, child-safety evidence, secret refs, encrypted Docdex context, exploit details, or fraud heuristics.

## Phase 7: Exceptions, Jurisdiction Updates, Migration Notes, And Bootstrap Controls

### Work Items

- **7.1 Implement exception request and review workflows.**
  - Design: Accept exception requests with target refs, marker refs being waived or narrowed, evidence refs, approver refs, expiry, revocation behavior, reporting class, affected scope, and resulting evaluation refs.
  - Output: `POST /compliance/exceptions`, review API, requested/evidence_review/approved/rejected/active/revoked/expired/superseded states, and audit refs.
  - Validation: Tests prove exceptions require expiry, are easy to revoke, and cannot waive high-compliance marker families without required legal-steward review.

- **7.2 Implement exception expiry and revocation enforcement.**
  - Design: Revoke expired exceptions on next evaluation, alert operators for missed expiry, preserve exception history, and create linked correction records rather than editing prior conclusions.
  - Output: Expiry worker, revocation records, alert rules, correction refs, and replay evidence.
  - Validation: Tests prove expired/revoked exceptions fail closed and prior evaluations replay with the exception state that existed at evaluation time.

- **7.3 Implement jurisdiction update records and affected-scope lists.**
  - Design: Record changed jurisdiction profiles, affected markers, affected tenants/services/apps, migration notes, effective-at, rollback notes, notification refs, and acceptance evidence.
  - Output: `POST /compliance/jurisdiction-updates`, affected-scope reports, reevaluation refs, migration notes, rollback refs, and owner-service notifications.
  - Validation: Tests prove jurisdiction changes create affected-scope lists and block permissive defaults when required profile facts are missing.

- **7.4 Implement migration and owner-service acceptance workflows.**
  - Design: Require owner-service acceptance evidence for boundary changes that alter accounting, privacy, payout, native-app publication, public-provider eligibility, AI context, export, or reporting behavior.
  - Output: Acceptance records, migration status, blocked-service refs, notification timelines, and rollback handoff refs.
  - Validation: Tests prove affected services remain staged/blocked until acceptance evidence exists or an approved migration exception is active.

- **7.5 Implement founder-bootstrap exception controls.**
  - Design: Allow only narrow, expiry-bound private seed exceptions for staged testing, non-sensitive source freshness gaps, operational markers, local development/test exports, private-swarm migration windows, and break-glass continuity actions.
  - Output: Bootstrap exception policy, denied-exception registry, expiry defaults, post-action review refs, and evidence checklist.
  - Validation: Tests reject public-provider payout waivers, custody/payment waivers, child-safety/regulated waivers, deletion/retention/residency reductions, public-report redaction waivers, broad vault access, public-node private/regulated/secret/system-service workloads, and long-lived audit/replay waivers.

## Phase 8: Compliance Exports, Evidence Refs, Stewardship Reporting, And Replay

### Work Items

- **8.1 Implement compliance export jobs.**
  - Design: Generate export packages only after declared purpose, audience, included fact bundles, redaction profile, evidence refs, artifact refs, integrity hash, expiry, and publication refs are present.
  - Output: `POST /compliance/exports`, export lifecycle states, redaction review, artifact refs, failed-redaction refs, delivery refs, and audit events.
  - Validation: Tests block export creation when redaction profile, audience, purpose, evidence refs, expiry, or authorization is missing.

- **8.2 Implement evidence-ref collection and redacted packages.**
  - Design: Assemble evidence packages from Overwatch, Overbase, Overstore, Overvault refs, owner-service refs, fact bundles, exception records, jurisdiction updates, and redaction decisions without copying private payloads.
  - Output: Evidence package schema, redaction manifest, integrity hash, source-ref list, access policy, and public/affected-party/steward/auditor projections.
  - Validation: Tests prove exported artifacts include enough provenance for audit while excluding private payloads, raw secret material, payment credentials, identity documents, child-safety evidence, fraud heuristics, exploit details, and encrypted RAG context.

- **8.3 Implement Stewardship Reporting and PIP Registry handoffs.**
  - Design: Provide redacted compliance summaries, correction/supersession notices, active ruleset/version summaries, aggregate metrics, report refs, and PIP change refs to governance consumers.
  - Output: Stewardship Reporting handoff contract, PIP Registry handoff refs, public summary fixtures, correction/withdrawal refs, and replay links.
  - Validation: Tests prove governance consumers receive redacted facts only and cannot mutate boundary rulesets, exceptions, reports, PIPs, or owner-service records through the handoff.

- **8.4 Implement replay endpoints and bundles.**
  - Design: Reconstruct evaluations, exports, comparisons, exception decisions, jurisdiction updates, fact bundles, owner-service handoffs, and Overguard decisions from stored ruleset versions and input fact refs.
  - Output: `GET /compliance/replay/{evaluation_or_export_id}`, replay bundle schema, redaction snapshots, source refs, generated fact bundle refs, decision refs, event refs, export refs, and deterministic hashes.
  - Validation: Tests prove replay reconstructs decisions deterministically from stored versions and excludes unauthorized private payloads or secret-bearing refs.

- **8.5 Implement export/replay observability and integrity checks.**
  - Design: Track export backlog, failed redactions, replay mismatches, artifact expiry, evidence-ref freshness, ruleset/source hash mismatch, delivery failures, and audience-denied attempts.
  - Output: Metrics, alerts, diagnostics, runbook entries, integrity incident hooks, and support-safe summaries.
  - Validation: Drills prove redaction failure blocks export, replay mismatch opens compliance integrity incident, and evidence integrity checks never reveal private evidence.

## Phase 9: Operations, Threat Review, Security Review, Drills, And Governance Hardening

### Work Items

- **9.1 Implement operational metrics and alerting.**
  - Design: Track ruleset rollout state, evaluation latency, denied evaluation counts, stale fact counts, exception volume, export backlog, jurisdiction updates, impacted-scope counts, handoff failures, and replay backlog.
  - Output: Metrics schema, alert rules, dashboard refs, degraded-state summaries, runbook docs, and owner-service escalation refs.
  - Validation: Tests and drills prove alerts fire for ruleset signature failure, marker spikes, high-compliance workload without isolation marker, export without redaction review, active exception past expiry, missing Overguard consumption, stale fact spikes, and replay mismatch.

- **9.2 Implement threat-model gates.**
  - Design: Add threat-model entries for stale or forged rulesets, marker drift, jurisdiction profile gaps, exception overreach, export redaction failure, fact bundle replay abuse, evidence-ref spoofing, owner-service handoff confusion, bootstrap exception abuse, and low-risk/high-compliance overconstraint.
  - Output: Threat model checklist, mitigation mapping, monitoring refs, accepted-risk records, remediation issue templates, and validation fixtures.
  - Validation: Review proves each threat has mitigation, tests, monitoring, or explicit accepted risk before broad release.

- **9.3 Implement security review gates.**
  - Design: Review ruleset signatures, marker registry integrity, steward/operator authorization, exception approval, jurisdiction update access, export release access, replay access, redaction policy, evidence-ref access, and owner-service handoffs.
  - Output: Security review checklist, reviewer refs, remediation records, release blockers, and post-review evidence.
  - Validation: Release cannot pass if high-compliance marker activation, exception approval, jurisdiction changes, or export release bypass signed steward/operator controls.

- **9.4 Implement reliability and incident drills.**
  - Design: Run drills for rule activation rollback, jurisdiction update fanout, stale fact surge, exception expiry miss, export redaction failure, owner-service outage, Overguard outage, Overwatch evidence outage, replay mismatch, and accidental sensitive-data capture.
  - Output: Drill scenarios, expected behavior, actual behavior, evidence bundles, incident hooks, remediation refs, and report summaries.
  - Validation: Drills prove safe failure modes: block/deny evaluation, retain fact bundles for retry, restrict queries, create redaction markers, open incidents, and preserve evaluator version evidence.

- **9.5 Implement governance reporting and public summaries.**
  - Design: Produce aggregate/redacted reports for boundary rulesets, supported jurisdictions/domains, marker changes, exception volumes, jurisdiction updates, export health, redaction failures, public native-app boundary behavior, provider payout boundary behavior, usage reconciliation, incidents, and replay health.
  - Output: Reporting schema, redaction profiles, public-safe summaries, source refs, governance handoff refs, and report replay refs.
  - Validation: Tests prove reports are specific enough for trust while excluding raw private data, payment credentials, child-safety evidence, secret refs, encrypted Docdex context, fraud heuristics, exploit details, and unrelated-party data.

## Phase 10: Validation, Link Alignment, Queue, Index, And Handoff Readiness

### Work Items

- **10.1 Validate sub-build plan structure.**
  - Design: Check title prefix, attached SDS link, ten phase headings numbered 1 through 10, five work items per phase, and Design/Output/Validation fields.
  - Output: Structure validation evidence for `docs/build_plan/sub_build_plan_076_compliance_boundary_service.md`.
  - Validation: Scripted checks pass for phase count, work-item count, numbering, and required fields.

- **10.2 Validate cross-document alignment.**
  - Design: Confirm SDS, service catalog entry, master plan, crosswalk, Phase 13, progress doc, and tech-stack guardrails all agree that Compliance Boundary is Phase 13-first with earlier phases as prerequisites only.
  - Output: Alignment checklist and updated backlinks across changed docs.
  - Validation: Local Markdown link checks pass and review finds no mismatch with master Phase 0 through Phase 13 order.

- **10.3 Validate stack and authority guardrails.**
  - Design: Scan changed docs for prohibited external product boundaries, conventional database/object-store/vault/queue/compliance-SaaS authority drift, direct legal advice, direct policy enforcement, accounting mutation, vault mutation, tenant mutation, payout approval, incident finality, blockchain/NFT language, pricing/revenue/customer-count assumptions, and BLAKE3-as-encryption wording.
  - Output: Guardrail scan evidence and corrected wording where needed.
  - Validation: Matches are either absent or explicit negative-control lines rejecting the prohibited assumptions.

- **10.4 Validate Docdex retrieval, impact, and index state.**
  - Design: Use Docdex impact, symbols, diagnostics, search, DAG export, and targeted index refresh for the new plan and linked docs.
  - Output: Impact evidence, symbols/Markdown structure evidence, search result evidence, DAG export evidence, and updated index stats.
  - Validation: Docdex search for SDS #76 returns the new sub-build plan and backlinks; impact diagnostics remain empty; targeted index refresh succeeds.

- **10.5 Validate implementation handoff readiness.**
  - Design: Update queue/progress evidence and confirm builders can start with marker taxonomy, rulesets, profiles, evaluations, fact bundles, owner-service handoffs, accounting/privacy/AI markers, exceptions, jurisdiction updates, exports, replay, operations, and governance hardening gates.
  - Output: Queue/progress update, blocker notes, validation command notes, and handoff summary.
  - Validation: `docdexd hook pre-commit --repo /Users/bekirdag/Documents/apps/overrid` passes; `docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid` result is recorded, including the known missing test-runner blocker if unchanged.
