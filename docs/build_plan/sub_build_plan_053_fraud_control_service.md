# SUB BUILD PLAN #53 - Fraud Control Service

Attached SDS: [docs/sds/federation_public/fraud_control_service.md](../sds/federation_public/fraud_control_service.md)

## Purpose

This sub-build plan turns SDS #53 into an implementation sequence for Fraud Control Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Fraud Control Service is the Phase 11 evidence and recommendation layer for provider fraud, workload abuse, payout abuse, result manipulation, challenge evasion, and policy evasion in federation and public-capacity pools. It owns fraud signal ingestion, signal normalization, risk cases, rule bundles, evidence packages, hold/throttle/challenge recommendations, false-positive corrections, and recommendation retractions. It does not adjudicate disputes, mutate payouts, rewrite Seal Ledger history, assign final reputation, run challenge tasks, schedule work, execute workloads, or expose raw fraud heuristics.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #53: Fraud Control Service](../sds/federation_public/fraud_control_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Fraud Control Service plan](../service_catalog/federation_public/fraud_control_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant boundaries, Overkey signing refs, Overgate ingress, Overregistry service facts, Overwatch audit refs, and Overqueue primitives. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overrun, Overcell, Overmeter, Overpack, Overlease, and Oversched execution facts consumed as anomaly evidence. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Oververify, Overguard, Overclaim, Workload Classifier, Policy Dry-Run API, Challenge Task Service, and private-trust evidence prerequisites. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter rollups, ORU/Overbill/Provider Payout/Seal Ledger refs, Overgrant refs, and accounting evidence without fraud-service mutation. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service placement, failover/restore evidence, audit retention, and operational reliability prerequisites for public-pool safety. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase/Overstore/Overvault/Universal Namespace refs, retention hooks, and protected evidence-storage boundaries used by risk-case history. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies federation/public-interest pool context and known-participant boundaries that Fraud Control can protect without becoming federation ownership. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Controls the first build point: public-provider fraud signals, challenge recommendations, payout-hold recommendations, throttles, public-pool abuse controls, and correction paths. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies public reporting, stewardship review, compliance retention, threat review, incident response, central-AI boundaries, audit export, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #53 first build work aligned to master Phase 11, with earlier phases as prerequisites and Phase 13 as later hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 7, 8, 10, 11, and 13 | Attach SDS #53, freeze recommendation-only authority, preserve Phase 11 as first build, and identify prerequisite and downstream owner-service gates. |
| 2 | Master Phases 0, 1, 4, 5, and 11 | Define Rust contracts, canonical schemas, lifecycle states, rule versions, reason codes, signed refs, redaction classes, and deterministic fixtures. |
| 3 | Master Phases 1, 3, 4, 5, 10, and 11 | Implement source-trusted signal ingestion, normalization, idempotency, deduplication, and source-quality handling from execution, trust, accounting, federation, and public-provider systems. |
| 4 | Master Phases 4, 5, 10, and 11 | Implement fraud rule bundles, risk-case lifecycle, severity/confidence evaluation, replay, and rule-version supersession without irreversible punishment. |
| 5 | Master Phases 4, 5, 10, and 11 | Implement Overguard-checked hold, throttle, challenge, and eligibility-impact recommendations without mutating owner-service state. |
| 6 | Master Phases 4, 8, 11, and 13 | Implement redacted evidence packages, reviewer views, Central AI bounded analysis inputs, and audience-specific evidence release. |
| 7 | Master Phases 4, 5, 11, and 13 | Implement false-positive correction, Overclaim appeal handoff, recommendation retraction, release requests, and post-finality boundaries. |
| 8 | Master Phases 3, 4, 5, 10, and 11 | Integrate public-provider onboarding, anti-Sybil, challenge, payout, billing, public-interest pool, and workload-execution consumers around a public low-sensitivity proof. |
| 9 | Master Phases 11 and 13 | Add operations, simulation, reviewer queues, metrics, privacy-preserving reports, incident/compliance hooks, and stewardship reporting. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Fraud Control Service core is a Rust service/module using shared contract crates, Tokio for bounded signal evaluation and case maintenance workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Fraud signals, risk cases, rule bundles, hold triggers, throttle recommendations, challenge recommendations, evidence packages, correction records, events, fixtures, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed service or operator envelopes, tenant/provider/pool/workload context where applicable, trace id, idempotency key, source event refs, schema version, rule version, policy context, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for evidence package snapshots, rule bundles, risk-case replay bundles, redaction projections, report projections, audit exports, and deterministic fixtures.
- Fraud Control Service may point to Oververify, Reputation and Anti-Sybil Service, Challenge Task Service, Overwatch, Overmeter, Overbill, Provider Payout Service, Seal Ledger, Overclaim, Overguard, Overrun, Overcell, Oversched, Public Provider Onboarding, Public-Interest Pool Service, Federation Template Service, Central AI Service, Incident Response, Compliance Boundary, and Stewardship Reporting, but it must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, revenue projections, customer-count assumptions, raw secret storage, payout mutation, ledger rewriting, final reputation scoring, dispute adjudication, challenge execution, scheduling, workload execution, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Fraud Authority, And Phase 11 Gates

### Work Items

- **1.1 Attach the build plan to SDS #53.**
  - Design: Link this document from the Fraud Control Service SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/federation_public/fraud_control_service.md`, `docs/service_catalog/federation_public/fraud_control_service.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #53 returns both the Fraud Control Service SDS and this sub-build plan.

- **1.2 Freeze recommendation-only authority boundaries.**
  - Design: Record that the service owns fraud signals, normalization, risk cases, rule bundles, recommendation records, evidence packages, correction records, and retraction refs, but not final owner-service mutations.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the service does not adjudicate disputes, execute payout holds/releases, rewrite Seal Ledger history, assign final reputation, run challenges, schedule workloads, execute workloads, or decide final policy.

- **1.3 Preserve master Phase 11 as the first build point.**
  - Design: Keep first implementation in Phase 11 because fraud control only becomes useful after identity, verification, policy, accounting, public-provider onboarding, public sandboxing, and public low-sensitivity workload boundaries exist.
  - Output: Phase-gate note that earlier phases are prerequisites, Phase 10 supplies federation/public-interest context, Phase 11 builds public-pool fraud controls, and Phase 13 hardens governance/reporting.
  - Validation: Review proves the plan does not move public-provider fraud control into Phases 0 through 10 and does not reorder master Phase 0 through Phase 13.

- **1.4 Carry forward resolved SDS #53 decisions.**
  - Design: Preserve the decisions for high-confidence automatic hold recommendations, manual review and challenge requirements for softer signals, bounded Central AI review, fast false-positive correction before payout finality, and redacted public fraud summaries.
  - Output: Resolved-decision checklist tied to hold allowlists, manual/challenge gates, Central AI analysis scope, correction timing, payout scaling pauses, and public/private redaction profiles.
  - Validation: Review rejects opaque sanctions, heuristic-only severe consequences, raw heuristic disclosure, central-AI-only enforcement, missing correction paths, and public reports exposing private provider, tenant, payout, challenge, topology, or exploit details.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Oververify, Reputation and Anti-Sybil Service, Challenge Task Service, Overwatch, Overmeter, Overbill, Provider Payout Service, Seal Ledger, Overclaim, Overguard, Overrun, Overcell, Oversched, Public Provider Onboarding, Public-Interest Pool Service, Federation Template Service, Central AI Service, Incident Response, Compliance Boundary, and Stewardship Reporting.
  - Output: Boundary matrix listing owner, input refs, output refs, source-trust rule, freshness rule, redaction class, policy refs, evidence refs, downstream consumer, and rejection behavior.
  - Validation: Review confirms every handoff uses explicit APIs, immutable refs, signed evidence, stable reason codes, trace ids, idempotency keys, policy refs, and Overwatch events rather than privileged shared records or hidden control paths.

## Phase 2: Rust Contracts, Schemas, Rule Bundles, And Fixtures

### Work Items

- **2.1 Create the Fraud Control Rust contract module.**
  - Design: Add contract types for fraud signals, risk cases, hold triggers, throttle recommendations, challenge recommendations, evidence packages, corrections, recommendation retractions, events, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, signal-type enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overguard, Overclaim, Challenge Task Service, Provider Payout Service, Overbill, Seal Ledger, scheduling, and execution internals.

- **2.2 Define fraud signal and source-trust schemas.**
  - Design: Model `fraud_signal` with signal type, subject refs, source service, source event ref, time window, severity, confidence, data class, rule version, evidence refs, source trust, dedup key, and ingestion idempotency.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and canonical signal fixtures.
  - Validation: Schema tests reject missing signal id, subject ref, source service, source event ref, time window, severity, confidence, data class, rule version, evidence refs, trace id, idempotency key, or source-trust classification.

- **2.3 Define risk case, recommendation, and correction schemas.**
  - Design: Model `risk_case`, `hold_trigger`, `throttle_recommendation`, `challenge_request_recommendation`, `evidence_package`, `fraud_correction`, and `recommendation_retraction` with append-only refs and lifecycle state.
  - Output: Risk-case schema, recommendation schemas, evidence package schema, correction schema, retraction schema, lifecycle examples, and negative fixtures.
  - Validation: Tests reject records that omit case id, subject refs, opened-from signal refs, severity, confidence, state, recommended actions, policy refs, claim refs, correction refs, audit refs, or downstream owner-service refs where required.

- **2.4 Define rule bundle and reason-code schemas.**
  - Design: Model rule bundles as signed, versioned, replayable mappings from normalized signals to severity, confidence, case type, recommended action, manual-review gate, challenge request, hold allowlist match, and expiry.
  - Output: Rule bundle schema, rule-version lifecycle, reason-code catalog, rule fixture set, compatibility examples, and supersession examples.
  - Validation: Tests prove rule bundles require signatures, schema versions, effective windows, owner refs, severity mappings, reason codes, replay hashes, and supersession refs while rejecting opaque unversioned rules.

- **2.5 Create deterministic fraud-control fixtures.**
  - Design: Build fixtures for volume anomaly, challenge failure, result inconsistency, payout-risk signal, duplicate execution mismatch, source-untrusted signal, duplicate signal, evidence missing, Overguard denial, downstream rejection, false-positive correction, and defective rule bundle.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected reason codes, evidence hashes, redaction outputs, and replay bundles.
  - Validation: Fixture tests produce stable ids, BLAKE3 hashes, lifecycle states, denial reason codes, audit refs, redacted views, and replay outputs across repeated runs.

## Phase 3: Signal Ingestion, Normalization, And Deduplication

### Work Items

- **3.1 Implement fraud signal ingestion.**
  - Design: Add `POST /fraud/signals` for source-authenticated batches from Overwatch, Overmeter, Oververify, Challenge Task Service, payout, billing, onboarding, anti-Sybil, and execution services.
  - Output: API handler, request/response schemas, signed envelope checks, idempotency behavior, source trust checks, stable errors, and `fraud_control.signal_ingested` events.
  - Validation: API tests cover valid signal batches, duplicate idempotency keys, untrusted source services, missing evidence refs, unsupported signal types, invalid subject refs, and redacted rejection events.

- **3.2 Normalize heterogeneous signal payloads.**
  - Design: Convert workload, provider, payout, challenge, benchmark, sandbox, egress, policy, public-pool, and abuse-report signals into canonical subject refs and reason-coded dimensions.
  - Output: Normalization pipeline, subject-ref parser, source-specific adapters, data-class mapper, reason-code mapper, and normalized-signal fixtures.
  - Validation: Tests prove source-specific fields do not leak into downstream rules without canonical mapping, and malformed or unsupported source facts fail with stable reason codes.

- **3.3 Deduplicate by subject, window, and evidence.**
  - Design: Deduplicate signals by provider, node, payout, workload, tenant, pool, time window, source event ref, evidence hash, and dedup key while preserving provenance.
  - Output: Deduplication module, duplicate-link records, idempotency state, duplicate event behavior, and dedup replay fixtures.
  - Validation: Tests prove duplicate signals link to existing cases or signal records without opening duplicate recommendations or dropping provenance needed for audit.

- **3.4 Validate source trust and evidence freshness.**
  - Design: Check source-service identity, signing refs, evidence freshness, trace continuity, data-class permissions, and policy context before allowing a signal into evaluation.
  - Output: Source-trust checker, evidence freshness checker, rejection reason codes, audit refs, and source trust fixtures.
  - Validation: Tests prove stale evidence, unsigned source refs, mismatched tenant/pool scope, untrusted reports, and unauthorized data-class access are rejected or held for review.

- **3.5 Publish signal query and audit projections.**
  - Design: Add role-scoped reads for accepted, rejected, duplicate, pending-review, and source-untrusted signals with audience-specific redaction.
  - Output: Query APIs, filters, pagination, redacted signal summaries, operator diagnostics, and Overwatch timeline refs.
  - Validation: Contract tests prove providers, tenants, operators, and stewardship readers see only the signal fields allowed by their redaction profile.

## Phase 4: Risk Case Lifecycle And Rule Evaluation

### Work Items

- **4.1 Implement rule bundle evaluation.**
  - Design: Apply versioned fraud rules to normalized signals to open, update, or suppress risk cases with explainable severity, confidence, case type, and reason codes.
  - Output: Rule engine, evaluation snapshot, rule-version refs, severity/confidence mapping, suppression reasons, and `fraud_control.case_opened`/`case_updated` events.
  - Validation: Tests prove volume anomaly, challenge failure, result inconsistency, payout-risk, duplicate-execution, sandbox escape, and policy-evasion signals produce expected case outcomes and replay hashes.

- **4.2 Implement risk case state machine.**
  - Design: Enforce risk case states from `opened` through collecting signals, recommendation pending, action active, awaiting challenge, awaiting review, under claim, corrected, closed, and superseded.
  - Output: State machine module, transition APIs or commands, lifecycle events, invalid-transition errors, timeline projections, and case fixtures.
  - Validation: Tests prove cases cannot skip required review/challenge/policy states and cannot close with active unretracted recommendations unless downstream owner-service finality refs exist.

- **4.3 Implement severity, confidence, and manual-review gates.**
  - Design: Separate high-confidence source-trusted automatic recommendations from softer, broader, privacy-sensitive, or ambiguous signals that require challenge, stewardship, or operator review.
  - Output: Gate matrix, severity/confidence thresholds, manual-review reason codes, review queue refs, challenge-needed markers, and privacy-sensitive signal handling.
  - Validation: Tests prove heuristic-only, single-source, compliance-sensitive, collusion, Sybil cluster, central-AI-only, and broad pool-wide cases cannot produce severe sanctions without review or challenge gates.

- **4.4 Implement rule-version supersession and reevaluation.**
  - Design: Support defective rule bundle marking, case reevaluation, recommendation supersession, and audit-preserving correction without deleting original signals or decisions.
  - Output: Rule supersession API or command, reevaluation job contract, affected-case query, superseded recommendation refs, and audit events.
  - Validation: Tests prove old decisions remain replayable, reevaluation creates new records, superseded recommendations are distinguishable, and downstream retraction needs are emitted.

- **4.5 Publish risk case reads and summaries.**
  - Design: Add `GET /fraud/cases/{case_id}` and `GET /fraud/subjects/{subject_ref}/summary` with operator, provider-safe, tenant-safe, stewardship, and public-safe projections.
  - Output: Query handlers, redaction profiles, state summaries, reason-code summaries, active recommendation summaries, correction status, and evidence refs.
  - Validation: Contract tests prove public and provider-safe summaries hide raw signals, thresholds, challenge payloads, private identities, tenant data, payout refs, topology, exploit details, and central-AI prompts.

## Phase 5: Hold, Throttle, Challenge, And Eligibility Recommendations

### Work Items

- **5.1 Implement Overguard policy checks for recommendations.**
  - Design: Require Overguard decisions before recommending payout holds, grant holds, public-pool throttles, challenge escalation, or eligibility-impact actions.
  - Output: Overguard adapter, policy fact bundle, policy decision refs, stale-policy behavior, denial reason codes, and `policy_checked` recommendation state.
  - Validation: Tests prove policy denial, stale policy refs, missing fact bundles, or Overguard unavailability prevent recommendation dispatch rather than defaulting to punitive action.

- **5.2 Implement payout and grant hold recommendations.**
  - Design: Add `POST /fraud/cases/{case_id}/recommend-hold` for high-confidence, pre-finality, source-trusted, policy-approved hold recommendations with release conditions and owner-service refs.
  - Output: Hold recommendation API, hold allowlist checks, Provider Payout/Overbill/Overclaim/Overgrant refs, expiry handling, release conditions, and `fraud_control.hold_recommended` events.
  - Validation: Tests prove Fraud Control only emits hold recommendations and never creates, releases, corrects, finalizes, or mutates actual holds, payouts, invoices, balances, grants, or ledger entries.

- **5.3 Implement public-pool and provider throttle recommendations.**
  - Design: Add `POST /fraud/cases/{case_id}/recommend-throttle` for bounded public-provider or public-pool throttles with duration, resource scope, reason codes, proportionality, and owner refs.
  - Output: Throttle recommendation API, proportionality checks, duration caps, pool/provider scopes, policy refs, and `fraud_control.throttle_recommended` events.
  - Validation: Tests prove broad pool-wide throttles require review, all throttles expire or renew through explicit evidence, and Public-Interest Pool Service/Public Provider Onboarding remain state owners.

- **5.4 Implement challenge recommendation handoff.**
  - Design: Add `POST /fraud/cases/{case_id}/request-challenge` to ask Challenge Task Service for liveness, capability, sandbox, duplicate-execution, and result-consistency checks.
  - Output: Challenge request recommendation API, challenge rationale schema, target snapshot refs, rate-limit checks, Challenge Task Service handoff refs, and `fraud_control.challenge_recommended` events.
  - Validation: Tests prove Fraud Control does not run challenges, define challenge payload internals, or escalate severity solely because Challenge Task Service is unavailable.

- **5.5 Implement eligibility-impact recommendations.**
  - Design: Emit reason-coded eligibility reduction or review recommendations for Overguard, Public Provider Onboarding, Reputation and Anti-Sybil Service, and Public-Interest Pool Service without assigning final reputation.
  - Output: Eligibility-impact recommendation contract, current-risk summary, owner-service target refs, correction refs, and public-low-sensitivity scope checks.
  - Validation: Tests prove final reputation, final eligibility, and onboarding state remain owner-service decisions and private/regulated/secret-bearing/system-service workloads remain excluded from public-provider decisions.

## Phase 6: Evidence Packages, Reviewer Views, And Central AI Bounds

### Work Items

- **6.1 Implement redacted evidence package generation.**
  - Design: Add `GET /fraud/cases/{case_id}/evidence-packages/{audience}` for provider-safe, tenant-safe, operator, Overclaim, Central-AI, governance, and public-summary audiences.
  - Output: Evidence package builder, redaction profiles, audience authorization checks, package hashes, trace refs, and `fraud_control.evidence_package_created` events.
  - Validation: Tests prove evidence packages include required signal, trace, usage, challenge, billing, claim, and policy refs while excluding raw private data outside the audience scope.

- **6.2 Implement reviewer case work queues.**
  - Design: Provide operator/stewardship queues for ambiguous, high-severity, public-report-sensitive, compliance-sensitive, broad-impact, and private-heuristic cases.
  - Output: Review queue projections, case assignment refs, stale review timers, review outcome refs, audit events, and escalation rules.
  - Validation: Tests prove manual-review-required cases cannot bypass review, stale queues emit alerts, and reviewer actions remain evidence refs rather than hidden state mutation.

- **6.3 Bound Central AI review.**
  - Design: Provide Central AI with redacted evidence packages, allowed analysis scope, missing-evidence prompts, risk summaries, confidence/proportionality context, expiry, provenance, and owner-service target refs.
  - Output: Central-AI evidence contract, analysis result schema, model/run provenance fields, human/steward signoff requirements, and denial of direct mutation authority.
  - Validation: Tests prove Central AI cannot open/close cases by itself, mutate holds, change provider eligibility, alter payouts, rewrite ledger entries, decide disputes, or serve as sole evidence for sanctions.

- **6.4 Implement public fraud summary redaction.**
  - Design: Publish public-safe summaries with report refs, time windows, subject classes, pool/workload class, severity/confidence bands, stable public-safe reason categories, current state, action class, correction status, and aggregate outcomes.
  - Output: Public summary schema, public report examples, aggregate outcome counters, stewardship refs, and redaction tests.
  - Validation: Tests prove public summaries hide raw signals, thresholds, challenge payloads/templates, heuristics, provider-private identity/contact/payout details, tenant/workload private data, topology, central-AI prompts, exploit details, and unreviewed severe allegations.

- **6.5 Implement evidence retention and protected storage refs.**
  - Design: Store evidence refs and package hashes using Overbase/Overstore/Overvault/Overwatch boundaries as appropriate without turning Fraud Control into primary storage, vault, or audit ownership.
  - Output: Evidence retention schema, protected ref model, package hash fixtures, retention class mappings, deletion/redaction behavior, and audit-export hooks.
  - Validation: Tests prove append-only case history stays replayable, redacted projections can be regenerated, and protected data remains behind owner-service or Overvault access decisions.

## Phase 7: Corrections, Appeals, Retractions, And Owner-Service Handoffs

### Work Items

- **7.1 Implement correction intake.**
  - Design: Add `POST /fraud/cases/{case_id}/corrections` for false-positive evidence, appeal refs, corrected source facts, provider-safe reason-code summaries, and downstream retraction needs.
  - Output: Correction API, acknowledgement state, evidence review state, Overclaim refs, provider notification refs, and `fraud_control.correction_recorded` events.
  - Validation: Tests prove corrections are append-only, linked to original cases, acknowledged within policy windows, and visible to affected providers through safe reason-code summaries.

- **7.2 Implement recommendation retraction workflows.**
  - Design: Retract or supersede hold, throttle, challenge, and eligibility-impact recommendations when corrections, appeals, defective rules, or downstream owner decisions invalidate them.
  - Output: Retraction command, superseded recommendation refs, downstream release/review request refs, audit events, and case timeline updates.
  - Validation: Tests prove retractions do not delete original evidence, downstream owner services receive correction refs, and Fraud Control does not directly release holds or mutate payout/reputation state.

- **7.3 Implement payout-finality and correction timing gates.**
  - Design: Enforce Phase 11 correction timing: 24-hour acknowledgement, 48-hour triage, 24-hour retraction/release request after accepted correcting evidence, standard correction before payout-batch finality and no later than the 7-day public-provider challenge/hold window.
  - Output: Timing policy config, stale correction alerts, payout-finality refs, queue health metrics, and payout-scaling pause signals.
  - Validation: Tests prove stale holds, correction backlogs, false-positive rates, or missed timing thresholds emit payout-scaling pause recommendations rather than silently widening public-provider payout volume.

- **7.4 Integrate Overclaim appeal and dispute handoff.**
  - Design: Hand disputes, appeal refs, claim refs, correction evidence, hold-release requests, and finality windows to Overclaim without deciding claims locally.
  - Output: Overclaim handoff contract, claim-ref validator, correction-to-claim link, appeal status projection, and finality markers.
  - Validation: Integration tests prove Overclaim remains the dispute owner and Fraud Control only attaches evidence, recommendations, correction refs, and retraction needs.

- **7.5 Publish correction and appeal summaries.**
  - Design: Provide provider, operator, stewardship, and public aggregate views for correction status, appeal status, accepted corrections, denied corrections, retractions, and false-positive trends.
  - Output: Summary APIs, redacted correction views, aggregate counters, provider-safe notices, and stewardship report refs.
  - Validation: Tests prove correction summaries are explainable and correctable while hiding private fraud heuristics, provider-private data, payout details, tenant data, and raw evidence outside authorized audiences.

## Phase 8: Public-Pool Integrations And Low-Sensitivity Fraud Proof

### Work Items

- **8.1 Integrate public-provider onboarding and anti-Sybil signals.**
  - Design: Consume onboarding, identity tier, node uniqueness, payout eligibility, behavior correlation, challenge history, reputation, and rate-limit signals from Public Provider Onboarding and Reputation and Anti-Sybil Service.
  - Output: Integration contracts, source trust rules, signal adapters, eligibility-impact refs, and public-provider fixtures.
  - Validation: Integration tests prove Fraud Control consumes anti-Sybil and onboarding refs without assigning final reputation, final eligibility, or onboarding state.

- **8.2 Integrate execution, scheduling, and sandbox anomaly sources.**
  - Design: Consume public low-sensitivity execution anomalies from Overrun, Overcell, Overmeter, Overwatch, Oversched, Workload Classifier, Public Sandbox Profile, and Challenge Task Service.
  - Output: Execution anomaly adapters, sandbox violation signals, result inconsistency signals, duplicate-execution mismatch signals, benchmark drift signals, and public workload-class guards.
  - Validation: Tests prove signals can open cases without scheduling or executing workloads and private, regulated, secret-bearing, and system-service workloads remain ineligible for public placement.

- **8.3 Integrate payout, billing, grant, and ledger refs.**
  - Design: Consume Provider Payout Service, Overbill, Overgrant, ORU Account Service, Seal Ledger, and Overmeter refs for payout-risk, hold-scope, billing evidence, grant-scope, earning-window, and correction refs.
  - Output: Accounting evidence adapters, hold-target refs, payout-window refs, grant refs, ledger stream refs, and correction handoff refs.
  - Validation: Tests prove Fraud Control never mutates ORU balances, invoices, payout batches, grant state, payment instructions, or Seal Ledger entries.

- **8.4 Integrate federation and public-interest pool consumers.**
  - Design: Provide fraud refs and redacted risk summaries to Federation Template Service and Public-Interest Pool Service for public-interest abuse controls, pool eligibility, purpose-tag abuse review, and participant reports.
  - Output: Pool/federation handoff contract, public-interest abuse signals, eligibility recommendation refs, stewardship summary refs, and redacted report fixtures.
  - Validation: Tests prove pool and federation services store fraud refs with their own state changes rather than copying fraud rule logic or using Fraud Control as final pool authority.

- **8.5 Prove the first low-sensitivity public fraud scenario.**
  - Design: Run an end-to-end Phase 11 proof where a public provider returns inconsistent low-sensitivity results, receives a challenge recommendation, triggers a policy-approved payout-hold recommendation, gets a provider-safe explanation, submits correction evidence, and receives a retraction request if accepted.
  - Output: Scenario fixture, signal bundle, risk case, rule evaluation, Overguard policy ref, challenge recommendation, hold recommendation, evidence package, correction record, retraction record, and replay bundle.
  - Validation: Scenario tests prove public low-sensitivity fraud controls work without leaking private data, mutating owner-service state, bypassing Overclaim, or exposing raw heuristics.

## Phase 9: Operations, Simulation, Reporting, And Governance Hooks

### Work Items

- **9.1 Implement operational metrics and diagnostics.**
  - Design: Track signal ingestion rate, rejected signal rate, case counts by state/severity, active recommendations, correction rates, false-positive rates, stale review queues, challenge recommendation volume, payout-risk spikes, and rule-version regressions.
  - Output: Health endpoint, metrics/events, operator diagnostics, stale review queries, rule-regression alerts, and Overwatch refs.
  - Validation: Tests prove diagnostics are role-scoped, redacted by audience, and tied to stable reason codes and replayable evidence.

- **9.2 Implement fraud-rule simulation mode.**
  - Design: Evaluate recorded signals, alternate rule bundles, severity thresholds, hold allowlists, throttle gates, challenge routing, redaction profiles, and correction outcomes without issuing recommendations.
  - Output: Simulation API, fixture input schema, simulated timeline, missing-prerequisite summaries, expected recommendation diffs, and replay packs.
  - Validation: Tests prove simulation is side-effect-free and cannot emit downstream hold, throttle, challenge, eligibility, payout, claim, or reputation actions.

- **9.3 Implement incident and compliance handoffs.**
  - Design: Hand high-severity fraud bursts, payout-risk spikes, public-pool abuse, sandbox escape, compliance-sensitive evidence, defective rule bundles, and public-report-sensitive cases to Incident Response and Compliance Boundary Service.
  - Output: Incident trigger refs, compliance export refs, severity classes, retention pins, redaction requirements, and escalation events.
  - Validation: Review proves incident/compliance hooks are evidence refs and workflow handoffs, not hidden authority for Fraud Control to punish providers or modify owner-service truth.

- **9.4 Implement stewardship and public reporting feeds.**
  - Design: Produce aggregate fraud statistics, correction outcomes, false-positive trends, challenge/hold volumes, public-pool abuse summaries, and redacted case summaries for Stewardship Reporting Service.
  - Output: Stewardship report contract, aggregate counters, public-safe reason categories, redacted examples, and report hash fixtures.
  - Validation: Tests prove reports are specific enough for accountability while hiding raw signals, provider-private data, tenant data, payout refs, private topology, exploit details, and central-AI prompts.

- **9.5 Implement governance hardening checkpoints.**
  - Design: Add Phase 13 checkpoints for threat modeling, central-AI boundaries, privacy review, payment/custody boundary review, formal security review, reliability drills, and rule-governance procedures.
  - Output: Governance checklist, threat-model inputs, security-review targets, rule-change review process, audit export refs, and PIP hooks where required.
  - Validation: Review confirms public-provider fraud enforcement is explainable, appealable, replayable, privacy-preserving, and proportional before scale expansion.

## Phase 10: Validation, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack and authority guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, fraud-owned-payout-mutation, fraud-owned-ledger-history, fraud-owned-final-reputation, fraud-owned-dispute-adjudication, fraud-owned-challenge-execution, fraud-owned-scheduling, fraud-owned-workload-execution, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools, local stubs, owner-service refs, or downstream APIs into Fraud Control Service's product boundary.

- **10.3 Validate SDS #53 build-breakdown coverage.**
  - Design: Map every SDS #53 build-breakdown item to this plan: schemas, signal ingestion/deduplication, rule bundle evaluation, risk case lifecycle, hold/throttle/challenge recommendation APIs, Overguard policy checks, evidence packages, corrections/appeals, and payout/public-pool proof.
  - Output: Coverage matrix, API checklist, workflow checklist, fixture checklist, and integration-test targets.
  - Validation: Review proves no SDS #53 build-breakdown item is missing and the plan preserves Fraud Control as a Phase 11 recommendation and evidence service.

- **10.4 Validate SDS, service catalog, master plan, crosswalk, and queue alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, build-plan crosswalk, queue state, queue progress, and build-plan progress.
  - Output: Updated backlinks and index rows for SDS #53, queue state update, queue progress update, and build-plan progress evidence.
  - Validation: JSON validation passes; local link checks pass; queue validation confirms `053-build-plan` is complete, no materialized task is running, and `054-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders source alignment, contracts, schemas, ingestion, rules, case lifecycle, recommendations, evidence, corrections, integrations, operations, governance hooks, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.

## Alignment Review

- The sub-build plan keeps Fraud Control Service first build work in master Phase 11 because public-provider fraud controls require identity, verification, policy, accounting, public-provider onboarding, payout holds, challenge tasks, and public low-sensitivity workload boundaries to exist first.
- The plan treats Phase 10 trusted federation and public-interest pools as upstream context and downstream consumers, not as the first build point for Fraud Control. Known-participant federation remains separate from adversarial unknown public-provider supply.
- The plan treats Overguard as policy owner; Fraud Control provides fraud facts, recommendation requests, and policy-context bundles but does not replace final policy decisions.
- The plan treats Overclaim as dispute and appeal owner; Fraud Control records evidence, correction refs, and retraction needs without adjudicating disputes or finality.
- The plan treats Provider Payout Service, Overbill, ORU Account Service, Overgrant, and Seal Ledger as accounting owners; Fraud Control recommends holds or releases but does not mutate payouts, invoices, balances, grants, or ledger entries.
- The plan treats Reputation and Anti-Sybil Service and Public Provider Onboarding as eligibility/reputation owners; Fraud Control emits reason-coded risk refs and recommendations without assigning final reputation.
- The plan treats Challenge Task Service, Workload Classifier, Public Sandbox Profile, Oversched, Overlease, Overcell, and Overrun as challenge/classification/scheduling/execution owners; Fraud Control consumes evidence and recommends challenges without running or placing work.
- The plan treats Central AI as a bounded redacted evidence reviewer; Central AI cannot directly open/close cases, mutate holds, alter eligibility, alter payouts, decide disputes, or serve as the sole evidence source for sanctions.
- The plan preserves master Phase 0 through Phase 13 ordering and uses earlier phases as prerequisites, Phase 11 as the first public-fraud build, and Phase 13 as governance/security/compliance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service/contracts, native Overrid boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.
