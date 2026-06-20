# SUB BUILD PLAN #43 - Provider Payout Service

Attached SDS: [docs/sds/accounting/provider_payout_service.md](../sds/accounting/provider_payout_service.md)

## Purpose

This sub-build plan turns SDS #43 into an implementation sequence for Provider Payout Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Provider Payout Service is a Phase 5 accounting coordination service that derives provider earning views from Seal Ledger refs, evaluates payout eligibility, applies dispute, verification, fraud, challenge, compliance, anti-Sybil, chargeback, and operator holds, creates payout batches, submits idempotent payment instruction refs through Overbill, tracks payment result refs, preserves failure/retry/reversal/correction evidence, and exposes redacted provider-facing payout status. It must not create usage truth, create earning truth, adjudicate disputes, decide trust alone, mutate ORU balances, rewrite Seal Ledger history, store raw payment or tax secrets, or become the external payment processor.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #43: Provider Payout Service](../sds/accounting/provider_payout_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Provider Payout Service plan](../service_catalog/accounting/provider_payout_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical schemas, signed envelopes, idempotency, trace ids, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identity refs, Overtenant tenant/provider scope, Overkey signing/service refs, Overgate request discipline, Overwatch audit, Overregistry refs, and Overqueue-safe command context. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies private workload execution refs, Overmeter raw usage facts, provider ids, workload ids, and settlement prerequisites. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overclaim dispute/finality refs, Oververify eligibility facts, Overguard decisions, Challenge Task evidence, Workload Classifier facts, and policy dry-run prerequisites. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Controls the first build point: private-provider earning views, eligibility snapshots, holds, payout batches, Overbill instruction/result refs, failures, reversals, corrections, and audit exports. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service operation, failover, restore, maintenance, and grid-resident hardening for payout workers and reconciliation jobs. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase persistence, Overstore replay/export artifacts, Overvault private payout/compliance refs, retention, backup/restore, and migration handoffs. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies federation and public-interest pool prerequisites that may later produce provider earning and grant/payment contexts. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Controls public-provider payout holds, throttles, anti-Sybil and fraud inputs, challenge finality, low-sensitivity public workload limits, and redacted public-provider explanations. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies Wallet and Usage Center, admin UI, SDK, CLI, native apps, and central AI stewardship views that consume redacted payout status and aggregate evidence. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance boundaries, incident handoffs, threat-model reviews, stewardship reports, audit exports, retention controls, migration governance, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #43 first build work aligned to master Phase 5, with Phase 11 public-provider expansion and later native-app/governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 11, 12, and 13 | Attach SDS #43, freeze Provider Payout Service authority, preserve Phase 5 as private-provider first build point, and record public/native/governance gates. |
| 2 | Master Phases 0, 1, 4, and 5 | Build Rust contracts, canonical schemas, reason codes, signed envelopes, state machines, source-ref catalogs, and deterministic fixtures. |
| 3 | Master Phases 3, 4, and 5 | Build provider earning views, payout periods, dispute windows, earning-ref validation, ORU/Seal Ledger/Overbill reconciliation, and replayable period queries. |
| 4 | Master Phases 4, 5, and 11 | Implement payout eligibility snapshots, hold creation, hold release, destination/compliance checks, explainability, and deny-by-default missing-fact behavior. |
| 5 | Master Phase 5 | Implement payout item selection, deterministic batches, payment instruction refs, Overbill submission, idempotent retries, and duplicate-suppression controls. |
| 6 | Master Phase 5, with Phase 13 evidence hardening | Implement payment result ingestion, failed payout handling, partial success, reversals, chargeback-linked corrections, and reconciliation checkpoints. |
| 7 | Master Phases 5, 6, 11, 12, and 13 | Implement provider-facing payout status, admin/operator views, Wallet and Usage Center handoffs, audit exports, redaction, and appeal/correction links. |
| 8 | Master Phase 11, with Phase 10 prerequisites | Add public-provider hold and throttle behavior using anti-Sybil, fraud, challenge, public sandbox, federation/public-interest, and stewardship evidence. |
| 9 | Master Phases 7, 8, and 13 | Harden operations, native persistence, grid-resident packaging, compliance/tax fact refs, incident handoffs, retention, migration, and scale behavior. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Provider Payout Service core is a Rust service/module using shared contract types, Tokio for bounded async workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Provider earning views, eligibility snapshots, payout holds, hold releases, payout batches, payout items, payment instruction refs, payout results, payout corrections, compliance export refs, API objects, events, fixtures, reason-code catalogs, redaction profiles, and replay bundles use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating endpoints require signed actor or service envelopes, tenant/provider scope, trace id, idempotency key, schema version, source evidence refs, policy refs, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for evidence fingerprints, replay/export bundles, schema fixtures, reconciliation checkpoints, and deterministic comparison tests.
- Raw bank, card, tax, identity, payout credential, and private compliance payloads stay behind Overvault, Overtenant/Overpass, Compliance Boundary, Overbill, or approved payment-provider/tokenization refs. Provider Payout Service stores only refs, freshness markers, ownership/consent markers, policy decisions, redaction classes, state, and audit evidence.
- Provider Payout Service may later persist payout records through Overbase, replay/export artifacts through Overstore, and private payout/compliance refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative markets, direct payment processors, or external payment providers the platform boundary.
- Phase 5 allows private-provider payout periods, earning views, eligibility snapshots, holds, batches, Overbill instruction/result refs, failures, reversals, corrections, and provider-visible status. Phase 11 owns public-provider extra holds, throttles, challenge/anti-Sybil/fraud finality, and public-safe explanations.
- Provider Payout Service coordinates payout state. It never creates usage truth, creates provider earning truth, adjudicates disputes, scores trust alone, mutates ORU balances, appends Seal Ledger entries, creates invoices or receipts, stores raw payment secrets, or directly executes external payments.
- Planning and implementation must avoid per-operation external payment calls, speculative token pricing, NFT mechanics, revenue projections, customer-count assumptions, direct ledger mutation, irreversible silent corrections, broad heuristic-only public-provider blocking, and public exposure of fraud heuristics, raw graph signals, operator notes, account hashes, or private tenant evidence.

## Phase 1: SDS Attachment, Provider Payout Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #43.**
  - Design: Link this document from the Provider Payout Service SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/accounting/provider_payout_service.md`, `docs/service_catalog/accounting/provider_payout_service.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #43 returns both the Provider Payout Service SDS and this sub-build plan.

- **1.2 Freeze Provider Payout Service authority boundaries.**
  - Design: Record that the service owns earning-period views, payout eligibility snapshots, payout holds, hold releases, payout batches, payout items, payment instruction refs, payment result state, failure/retry/reversal/correction coordination, provider-facing status, and audit/compliance export refs.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms the service does not own usage truth, provider earning truth, ORU balance mutation, Seal Ledger mutation, Overbill invoices/receipts, external payment execution, dispute adjudication, trust scoring, raw payment credentials, or raw compliance secrets.

- **1.3 Preserve master Phase 5 as the first build point.**
  - Design: Keep first implementation in Phase 5 after Phase 0 contracts, Phase 1 identity/audit, Phase 3 usage facts, and Phase 4 dispute/verification/policy prerequisites exist.
  - Output: Phase-gate note that Phase 5 builds private-provider payout primitives, Phase 11 adds public-provider extra holds/throttles, Phase 12 exposes native/client views, and Phase 13 hardens compliance/governance.
  - Validation: Review proves this plan does not move public-provider payout relaxation into Phase 5 and does not defer core private-provider payout coordination behind later phases.

- **1.4 Carry forward resolved SDS #43 decisions.**
  - Design: Preserve ref-based deny-by-default destination checks, seven-day default closed payout periods after settlement/dispute-window close, public-provider safe explanation codes, owner-service refs for cross-border compliance, and narrow evidence-backed future-payout blocks.
  - Output: Resolved-decision checklist tied to implementation reviews.
  - Validation: Review rejects raw payment/tax storage, per-workload external payouts, exact fraud-threshold disclosure, jurisdiction logic embedded in this service, and broad heuristic-only provider bans.

- **1.5 Define owner-service and consumer boundaries.**
  - Design: Create a dependency matrix for Seal Ledger, Overbill, Overclaim, Oververify, Reputation and Anti-Sybil Service, Fraud Control Service, Challenge Task Service, ORU Account Service, Overguard, Overwatch, Overvault, Compliance Boundary Service, Wallet and Usage Center, admin UI, CLI, SDK, and central AI stewardship.
  - Output: Boundary matrix listing consumed refs, emitted refs, final authority owner, redaction class, replay evidence, blocking behavior, and later phase gate.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, trace ids, policy refs, and Overwatch audit rather than direct privileged state reads.

## Phase 2: Rust Contracts, Schemas, Reason Codes, And Fixtures

### Work Items

- **2.1 Create the Provider Payout Rust contract module.**
  - Design: Add contract types for provider earning views, payout eligibility snapshots, payout holds, hold releases, payout batches, payout items, payment instruction refs, payout results, payout corrections, compliance export refs, API errors, events, redaction profiles, and replay bundles.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, hold-type enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Seal Ledger, Overbill, Overclaim, Oververify, Reputation/Anti-Sybil, Fraud Control, ORU Account Service, and Overvault internals.

- **2.2 Define canonical payout schemas and examples.**
  - Design: Add versioned schemas for earning-view build requests, eligibility evaluation, hold create/release, payout batch create/submit, payment result ingest, payout correction, provider status, replay, and compliance export.
  - Output: Schema files, valid examples, invalid examples, generated validators, compatibility notes, and docs-facing examples.
  - Validation: Schema tests reject missing provider id, period id, tenant/scope, trace id, idempotency key, state, schema version, policy refs, source evidence refs, audit refs, redaction class, or reason codes where required.

- **2.3 Define state machines and reason-code catalogs.**
  - Design: Encode payout item states from `earning_observed` through dispute/hold/eligible/batched/submitted/paid/failed/reversed/corrected and payout batch states from draft through ready/submitted/partially_paid/paid/failed/cancelled/corrected.
  - Output: Transition tables, Rust enums, JSON Schema enums, invalid transition fixtures, reason-code catalog, and lifecycle review notes.
  - Validation: State tests reject silent deletion, historical edits, paid-state overwrite, correction without old refs, batch submission while a blocking hold is active, and result ingestion without Overbill/payment-provider refs.

- **2.4 Model source-ref and envelope discipline.**
  - Design: Define source refs for Seal Ledger earning entries, Overmeter rollups, Overclaim disputes/finality, Oververify eligibility, Reputation/Anti-Sybil risk, Fraud Control evidence, Challenge Task results, ORU account projections, Overbill payment refs, Overvault tokenized refs, Compliance Boundary fact bundles, Overguard allow decisions, and Overwatch audit.
  - Output: Source-ref catalog, accepted freshness markers, owner-service map, evidence envelope rules, error catalog, and fixture references.
  - Validation: Tests reject unsigned, stale, wrong-scope, wrong-owner, missing-finality, missing-policy, or raw-secret-bearing refs with stable reason codes.

- **2.5 Create deterministic payout fixtures.**
  - Design: Build fixtures for disputed earnings, clean private-provider eligibility, missing destination refs, compliance holds, verification holds, deterministic batch creation, Overbill submission, external failure, partial success, reversal, correction, public-provider challenge hold, and redacted status reads.
  - Output: Fixture directory, expected projections, reason codes, Overwatch events, replay examples, export examples, redacted views, and invalid examples.
  - Validation: Fixture tests produce stable batch ids, item states, hold states, result states, correction refs, replay hashes, and idempotency outcomes across repeated runs.

## Phase 3: Provider Earning Views, Payout Periods, And Reconciliation Inputs

### Work Items

- **3.1 Implement Seal Ledger earning-ref intake.**
  - Design: Resolve provider earning entries from Seal Ledger for a bounded period without copying or mutating ledger truth.
  - Output: Earning-ref resolver interface, period query contract, missing/inconsistent earning reason codes, and `provider_payout.earning_view_built` event wiring.
  - Validation: Tests reject incomplete earning refs, wrong provider scope, wrong period, unsigned refs, corrected/superseded refs without replacement context, and ledger entries outside the requested period.

- **3.2 Implement payout-period policy.**
  - Design: Model closed payout periods, dispute-window expiry, default seven-day submission timing, rail minimum rollover, operator-approved exceptions, provider exits, and compliance-driven early settlement without hardcoding rail-specific amounts in this SDS boundary.
  - Output: Payout period contract, policy adapter fields, closed-period state, rollover records, exception records, and period replay metadata.
  - Validation: Tests prove Phase 5 private payouts evaluate daily but submit only closed eligible periods unless an approved correction, exit, or compliance exception exists.

- **3.3 Build provider earning views.**
  - Design: Derive candidate payable totals from earning refs, deductions, holds, correction refs, dimensions, tenant/scope, provider id, and period id.
  - Output: `POST /earning-views/build`, idempotent view creation, view projection, source-ref list, derived totals by dimension/rail/ref, and audit events.
  - Validation: API tests prove earning views are deterministic for the same inputs, cite all source refs, and never create ledger entries, ORU balance transitions, invoices, receipts, or payment instructions.

- **3.4 Add ORU Account Service and Overbill reconciliation inputs.**
  - Design: Compare earning views against ORU account projections, Seal Ledger state, Overbill payout-hold/payment refs, and prior correction/reversal refs before eligibility proceeds.
  - Output: Reconciliation checkpoint model, mismatch reason codes, blocker records, and replay fields.
  - Validation: Tests prove reconciliation mismatches block payout eligibility with `reconciliation_mismatch`-style reasons and cannot be bypassed by batch creation.

- **3.5 Implement period and provider queries.**
  - Design: Provide operator/internal queries for earning views by provider, period, tenant/scope, state, missing facts, hold exposure, and reconciliation checkpoint.
  - Output: `GET /earning-views`, `GET /payout-periods/{id}`, pagination fields, redaction controls, and drill-down links.
  - Validation: Tests prove internal queries are scope-limited, provider-facing queries remain unavailable until Phase 7 status redaction is in place, and replay uses the same source refs as eligibility.

## Phase 4: Eligibility Snapshots, Holds, Release Rules, And Explainability

### Work Items

- **4.1 Implement eligibility evaluation.**
  - Design: Collect dispute, verification, fraud, challenge, compliance, anti-Sybil, chargeback, destination, and operator/stewardship facts into payout eligibility snapshots.
  - Output: `POST /payout-eligibility/evaluate`, snapshot records, missing-fact records, eligible/blocked states, reason codes, and `provider_payout.eligibility_evaluated` events.
  - Validation: Tests prove missing mandatory refs deny eligibility by default and every allow/block decision carries source refs, policy refs, trace id, idempotency key, and audit refs.

- **4.2 Implement payout hold creation.**
  - Design: Create holds for dispute windows, Overclaim requests, Oververify gaps, fraud/challenge evidence, anti-Sybil recommendations, compliance review, chargeback/reversal risk, destination review, and signed operator/stewardship action.
  - Output: `POST /holds`, hold records, trigger refs, effective windows, release requirements, appeal refs, and `provider_payout.hold_created` events.
  - Validation: Tests prove active blocking holds prevent item eligibility and batch submission, and non-blocking observation refs cannot silently become blocking without a hold record.

- **4.3 Implement hold release rules.**
  - Design: Release holds only when finality/evidence refs satisfy the hold type's release requirements and old hold records remain replayable.
  - Output: `POST /holds/{id}/release`, release records, finality refs, service/account actor refs, released earning refs, and `provider_payout.hold_released` events.
  - Validation: Tests reject release without Overclaim finality, verification clearance, compliance clearance, challenge/fraud resolution, destination clearance, or signed operator/stewardship action where required.

- **4.4 Implement mandatory payout destination and compliance checks.**
  - Design: Validate current provider account refs, provider eligibility refs, ownership/consent refs, tokenized payout destination refs, supported currency/rail/region refs, Overbill adapter capability refs, Compliance Boundary fact bundles, Overguard allow decisions, and reconciliation checkpoints.
  - Output: Destination/compliance checker, freshness markers, ownership/consent markers, missing-ref reason codes, and deny-by-default policy.
  - Validation: Tests prove raw bank/card/tax/identity/credential data is never stored and missing/expired fact bundles block eligibility rather than being treated as allow.

- **4.5 Implement explainable reason-code summaries.**
  - Design: Produce coarse, stable, remediable reason summaries for internal and provider-facing use while keeping fraud heuristics, model weights, graph edges, exact thresholds, operator notes, and private evidence hidden.
  - Output: Explanation schema, audience map, remediation steps, Overclaim appeal refs, policy/evaluator version refs, and redacted evidence refs.
  - Validation: Tests prove public/provider explanations expose safe reason codes only and never leak other-provider identities, raw graph signals, exact account hashes, private tenant evidence, or incident-response details.

## Phase 5: Payout Items, Deterministic Batches, And Overbill Submission

### Work Items

- **5.1 Implement payout item selection.**
  - Design: Select eligible earning refs into payout items only when eligibility snapshots are current, blocking holds are absent, reconciliation checkpoints are clean, and destination/compliance checks passed.
  - Output: Payout item builder, excluded-hold refs, payable amount/ref, destination ref, status, correction refs, and `provider_payout.item_marked_eligible` or `provider_payout.item_blocked` events.
  - Validation: Tests prove disputed, held, missing-destination, compliance-blocked, reconciliation-mismatched, or stale-snapshot items cannot enter a payout batch.

- **5.2 Implement deterministic payout batch creation.**
  - Design: Group eligible payout items by period, rail/currency/ref, policy scope, provider constraints, and idempotency key without making per-operation external payment calls.
  - Output: `POST /payout-batches`, batch records, item membership, totals by currency/rail/ref, draft/ready state, audit refs, and `provider_payout.batch_created` events.
  - Validation: Tests prove the same provider/period/candidates/idempotency key creates the same batch and duplicate creation cannot add or drop items silently.

- **5.3 Build payment instruction refs.**
  - Design: Create idempotent payment instruction refs for Overbill/payment-provider adapters that carry only tokenized destination refs, batch/item refs, scope, rail/currency refs, and audit metadata.
  - Output: `payment_instruction_ref` model, instruction builder, Overbill handoff schema, duplicate-suppression fields, and private-ref boundaries.
  - Validation: Tests prove instructions contain no raw payment credentials and cannot be built for unsupported rails, expired destination refs, missing consent refs, or blocked items.

- **5.4 Submit payout batches through Overbill.**
  - Design: Submit ready batches to Overbill/payment-provider refs, record submission state before external retry, and preserve idempotency across timeouts.
  - Output: `POST /payout-batches/{id}/submit`, Overbill submission adapter, submitted state, failure-before-retry records, and `provider_payout.batch_submitted` events.
  - Validation: Tests prove submission failure is recorded before retry, timeouts do not duplicate payment instructions, and Provider Payout Service never directly executes external payments.

- **5.5 Implement batch cancellation and pre-submit corrections.**
  - Design: Allow ready/draft batches to be cancelled or corrected before submission when new holds, disputes, reconciliation mismatches, destination revocation, or compliance refs arrive.
  - Output: Cancel/correct behavior, replacement batch refs, affected item states, audit refs, and downstream warning events.
  - Validation: Tests prove cancelled batches cannot be submitted, replacement batches cite the old batch, and old batch records remain replayable.

## Phase 6: Payment Results, Failures, Reversals, Corrections, And Reconciliation

### Work Items

- **6.1 Ingest Overbill payment result refs.**
  - Design: Receive signed Overbill/payment-provider result refs and update payout item and batch state without trusting raw external callbacks directly.
  - Output: `POST /payout-batches/{id}/results`, result resolver, paid/failed/reversed/chargeback states, timestamps, provider-visible summary, private evidence refs, and `provider_payout.payment_result_recorded` events.
  - Validation: Tests reject unsigned, duplicate, wrong-batch, wrong-item, stale, missing-Overbill, or raw-secret-bearing result refs with stable reason codes.

- **6.2 Implement failed payout retry coordination.**
  - Design: Preserve failure evidence, classify rail/provider/config/temporary errors, and retry only with safe idempotency and current eligibility/destination/compliance checks.
  - Output: Failure records, retry policy, backoff state, retry attempt refs, `provider_payout.payout_failed` events, and replay metadata.
  - Validation: Tests prove failed payouts can be retried without duplicate payment instructions and cannot retry after destination revocation, active hold, or compliance expiry.

- **6.3 Implement partial success handling.**
  - Design: Track item-level paid/failed states for partially successful batches while preserving batch-level summary state.
  - Output: Partial result projection, item-level reconciliation fields, affected-provider summaries, and retry/correction routing for failed items.
  - Validation: Tests prove paid items are not rolled back by deleting evidence, failed items remain retry/correction candidates, and provider-visible status is accurate per item.

- **6.4 Implement reversal and chargeback-linked correction flows.**
  - Design: Create reversal/correction refs for external reversals, chargebacks, Overclaim finality, fraud/challenge outcomes, or compliance corrections without editing historical paid state.
  - Output: `POST /payout-items/{id}/corrections`, correction records, old item/batch/result refs, new accounting/billing refs, future-hold triggers, and `provider_payout.reversal_recorded`/`provider_payout.correction_recorded` events.
  - Validation: Tests prove reversals and chargebacks create new refs, cite prior evidence, optionally hold future eligible payouts, and never overwrite old paid/result records.

- **6.5 Reconcile Seal Ledger, Overbill, ORU, and payout state.**
  - Design: Run reconciliation by provider, period, batch, item, rail/currency/ref, hold reason, correction state, and external result.
  - Output: Reconciliation job, gap reports, checkpoint refs, operator drill-downs, and repair task records.
  - Validation: Tests prove mismatches block future payout for affected scopes until release/correction refs arrive and reconciliation records remain append-only.

## Phase 7: Provider Status, Redaction, Audit Export, And Client Handoffs

### Work Items

- **7.1 Implement provider-facing payout status.**
  - Design: Return provider-visible status across earned, held, eligible, batched, submitted, paid, failed, reversed, and corrected states with coarse reason-code summaries.
  - Output: `GET /providers/{id}/payout-status`, status view model, freshness fields, safe remediation steps, and Overclaim appeal refs.
  - Validation: Tests prove providers see only their own status and cannot infer other-provider evidence, exact fraud thresholds, raw graph data, account hashes, operator notes, or private tenant evidence.

- **7.2 Implement role-aware admin/operator views.**
  - Design: Provide internal drill-down by provider, period, batch, item, hold reason, source evidence, external payment-provider ref, reconciliation state, and policy/evaluator version.
  - Output: Admin/operator query schemas, pagination, redaction classes, Overwatch links, and audit export hooks.
  - Validation: Tests prove privileged views require signed service/operator identity, tenant/provider scope, policy refs, and audit events.

- **7.3 Implement audit and compliance exports.**
  - Design: Create authorized export refs for payout periods, batches, holds, releases, corrections, reconciliation checkpoints, and compliance fact bundles.
  - Output: `GET /payout-periods/{id}/export`, compliance export refs, redaction metadata, retention class, export hash, and private-ref boundaries.
  - Validation: Tests prove exports cite source refs and never include raw payment credentials, tax forms, identity docs, private compliance payloads, or fraud heuristics outside authorized scope.

- **7.4 Feed Wallet and Usage Center, SDK, CLI, and admin UI.**
  - Design: Provide client examples for payout status reads, hold explanations, batch/result histories, appeals, exports, and operator diagnostics.
  - Output: SDK/CLI examples, Wallet and Usage Center view model, admin UI field list, native-service examples, and central AI stewardship summary fields.
  - Validation: Tests prove clients cannot create payout truth, override holds, submit batches directly, read raw payment refs, or convert status views into ledger/billing authority.

- **7.5 Implement Overclaim and correction handoffs.**
  - Design: Link provider appeals, hold disputes, correction requests, payout reversal evidence, and finality refs to Overclaim without letting this service adjudicate disputes.
  - Output: Overclaim handoff schema, appeal refs, correction refs, status links, and redacted explanation fields.
  - Validation: Tests prove Overclaim owns finality, Provider Payout Service owns payout-state coordination, and updates happen through evidence refs rather than direct state edits.

## Phase 8: Phase 11 Public-Provider Holds, Throttles, And Safety Gates

### Work Items

- **8.1 Add explicit Phase 11 public-provider gates.**
  - Design: Keep public-provider payout logic behind feature gates requiring public onboarding, public sandbox profile, low-sensitivity workload class, anti-Sybil evidence, challenge/fraud inputs, and payout-hold policy.
  - Output: Public-provider gate flags, missing-prerequisite reason codes, rollout checklist, and public-provider fixture set.
  - Validation: Tests prove public-provider payout paths are unavailable before Phase 11 prerequisites exist and private-provider Phase 5 behavior remains unchanged.

- **8.2 Ingest anti-Sybil, fraud, and challenge signals.**
  - Design: Consume Reputation/Anti-Sybil recommendations, Fraud Control refs, Challenge Task outcomes, duplicate-execution facts, public sandbox violations, and Oververify updates as hold/throttle inputs.
  - Output: Public-risk source-ref resolver, hold trigger mapping, throttle refs, challenge finality fields, and public-provider audit events.
  - Validation: Tests prove risk signals can hold or throttle affected items/providers/scopes but do not expose heuristics or become broad unreviewed bans.

- **8.3 Implement public-provider waiting periods and earning velocity controls.**
  - Design: Extend waiting periods, reduce earning velocity, roll items forward, or require extra finality for new/risky public providers without changing the private-provider closed-period contract.
  - Output: Public payout-period policy, throttle state, rollover records, safety-review records, and reason-code summaries.
  - Validation: Tests prove public-provider payouts can be delayed or throttled based on evidence while still remaining explainable, appealable, and replayable.

- **8.4 Implement public-safe hold explanations.**
  - Design: Use only stable, remediable reason codes such as verification incomplete, challenge review pending, dispute window active, destination review required, compliance review required, public pool throttle active, payout safety review, appeal/correction pending, and external rail unavailable.
  - Output: Public/provider explanation catalog, remediation steps, redacted evidence refs, policy/evaluator version refs, and Overclaim appeal links.
  - Validation: Tests prove explanations do not reveal exact thresholds, fraud heuristics, model weights, cluster membership, other-provider identities, raw graph edges, account hashes, private tenant evidence, operator notes, or incident details.

- **8.5 Add stewardship review for high-impact public holds.**
  - Design: Require review before low-confidence clusters, region-restricted signals, heuristic-only suspicion, or broad multi-provider impact blocks beyond the specific risky items.
  - Output: Stewardship review queue, escalation refs, review outcomes, scoped block records, and release/correction requirements.
  - Validation: Tests prove high-impact public-provider blocks are narrow, evidence-backed, scoped, reviewable, and correctable.

## Phase 9: Operations, Native Persistence, Grid Residency, And Governance

### Work Items

- **9.1 Build payout dashboards and alerts.**
  - Design: Track earning views, held amount/count by reason, eligibility denial rates, missing-fact rates, batch size, submission latency, payment-provider errors, retries, reversals, public-provider hold rates, reconciliation gaps, and status freshness.
  - Output: Metrics schema, dashboard definitions, alert rules, Overwatch aggregation, and runbook links.
  - Validation: Operations tests prove alerts fire for stale eligibility snapshots, unexpected hold spikes, submission timeouts, duplicate idempotency conflicts, reconciliation mismatches, redaction failures, and export failures.

- **9.2 Prepare native Overbase, Overstore, and Overvault persistence handoffs.**
  - Design: Move payout records to native Overbase when available, replay/export artifacts to Overstore where appropriate, and private payout/compliance refs to Overvault without changing API semantics.
  - Output: Persistence adapter interfaces, migration plan, artifact refs, private-ref handoffs, retention hooks, and export format.
  - Validation: Migration tests prove no conventional database/object/vault boundary becomes canonical, records remain replayable, and private payout/compliance refs stay behind owning-service access controls.

- **9.3 Prepare grid-resident protected operation.**
  - Design: Package payout workers and reconciliation jobs as protected grid-resident system workloads with service identity, config contracts, secret/private refs, health checks, failover behavior, restore drills, maintenance mode, and signed operator actions.
  - Output: System-service packaging notes, runtime config contract, backup/restore hooks, health endpoints, maintenance controls, and break-glass audit rules.
  - Validation: Grid tests prove restart, failover, restore, replay pause/resume, and maintenance mode preserve append-only payout state and do not submit duplicate external instructions.

- **9.4 Add compliance, tax, and jurisdiction fact-ref governance.**
  - Design: Represent jurisdiction profile refs, tax profile/exemption refs, withholding/reporting marker refs, currency/rail refs, retention/redaction class, effective windows, expiry, and audit/export refs as owner-service facts.
  - Output: Compliance Boundary adapter, fact-bundle schema, jurisdiction-change handling, expiry blockers, and export metadata.
  - Validation: Tests prove jurisdiction changes create new fact-bundle/policy refs, existing payout records remain append-only, and raw tax/identity/payment data stays outside Provider Payout Service.

- **9.5 Add incident, threat-model, retention, and migration handoffs.**
  - Design: Integrate incident response refs, threat-model findings, compliance boundary policy refs, stewardship reports, migration controls, retention/export policy, and scale-hardening reviews.
  - Output: Governance checklist, threat-model test list, incident handoff refs, stewardship report fields, migration plan, and retention policy.
  - Validation: Governance tests prove high-impact blocks, reversal/correction waves, payment-provider incidents, export changes, redaction repairs, and compliance policy changes require signed action, evidence refs, Overwatch audit, and retention-compliant exports.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #43`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, work-item sections, final newline, and tab-free formatting.

- **10.2 Validate tech-stack alignment.**
  - Design: Scan for accidental conventional cloud, SaaS-admin, blockchain/NFT, speculative-market, external-payment-as-boundary, pricing, revenue, customer-count, raw-secret storage, direct-ledger-mutation, ORU-balance-mutation, dispute-adjudication, trust-scoring, and broad public-provider blocking assumptions.
  - Output: Stack-alignment scan result and any corrected wording.
  - Validation: Scan shows only approved negative-control references, native Overrid service names, or explicit non-choice guardrails from `docs/overrid_tech_stack_choice.md`.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk links.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated docs and link-check evidence.
  - Validation: Local Markdown link validation returns no missing local targets for changed docs.

- **10.4 Validate Provider Payout authority and phase gates.**
  - Design: Verify every planned behavior preserves Phase 5 as the first implementation point for private-provider payout coordination and Phase 11 as the public-provider extra hold/throttle gate.
  - Output: Authority-boundary checklist and implementation handoff notes.
  - Validation: Review confirms Provider Payout Service does not own usage truth, provider earning truth, ORU balance mutation, Seal Ledger mutation, Overbill invoices/receipts, external payment execution, dispute adjudication, trust scoring, raw payment secrets, raw compliance secrets, revenue projections, customer-count assumptions, or per-operation external payments.

- **10.5 Reindex and verify retrieval handoff.**
  - Design: Refresh Docdex for the new plan and linked docs, then verify search returns the SDS #43 plan and source docs in the aligned result set.
  - Output: Docdex index refresh evidence, retrieval evidence, queue/progress notes, repo memory, and implementation handoff note.
  - Validation: Docdex index succeeds, Docdex search for `SUB BUILD PLAN #43 Provider Payout Service Phase 5 payout eligibility holds Overbill payment refs Phase 11 public holds` returns the new sub-build plan in the result set, and `docdexd run-tests` blocker is recorded if no test runner is configured.

## Alignment Review

- SDS #43 already contains resolved open-question decisions for deny-by-default payout destination checks, closed-period payout timing, public-provider explanation boundaries, owner-service compliance refs, and narrow evidence-backed future-payout blocks. This pass adds the sub-build-plan backlink and does not require SDS content correction.
- The service catalog already matches the SDS and master plan: Provider Payout Service starts in Phase 5 for private-provider earning views, holds, eligibility, batches, Overbill refs, results, failures, reversals, corrections, and status; Phase 11 adds stricter public-provider controls. This pass adds the sub-build-plan backlink.
- The master Phase 0 through Phase 13 order remains valid. This pass adds SDS #43 to the per-SDS index and keeps Provider Payout Service in Phase 5 while preserving Phase 11 as the public-provider hold/throttle gate, Phase 12 as client/native consumption, and Phase 13 as governance hardening.
- The build-plan crosswalk remains valid. This pass adds SDS #43 to the sub-build-plan index with Phase 5 first-build alignment and later public/native/governance gates.
- The accepted Rust-first/native-Overrid stack remains authoritative. The plan uses Rust, Tokio, shared contracts, canonical JSON plus JSON Schema, signed envelopes, Ed25519, BLAKE3/content hashes, and native Overrid persistence/storage/private-ref boundaries.

## Exit Gate

SUB BUILD PLAN #43 is complete when the new plan is linked from the SDS, service catalog, master build plan, and service-catalog alignment docs; Phase 5 remains the first build point for private-provider earning views, eligibility snapshots, payout holds, deterministic payout batches, Overbill payment instruction/result refs, failures, reversals, corrections, provider-facing status, and audit exports; Phase 11 remains the public-provider hold/throttle gate; the plan has 10 phases with well-designed Design/Output/Validation work items; queue/progress docs record the pass; local link and structure validation pass; stack guardrail scans show no accidental conventional-cloud, NFT, speculative-market, external-payment-as-boundary, pricing, revenue, customer-count, raw-secret-storage, direct-ledger-mutation, ORU-balance-mutation, dispute-adjudication, trust-scoring, broad public-provider-blocking, or per-operation external-payment drift; and Docdex retrieval can find the new plan with SDS #43 context.
