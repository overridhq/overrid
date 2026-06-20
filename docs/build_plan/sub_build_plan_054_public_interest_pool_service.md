# SUB BUILD PLAN #54 - Public-Interest Pool Service

Attached SDS: [docs/sds/federation_public/public_interest_pool_service.md](../sds/federation_public/public_interest_pool_service.md)

## Purpose

This sub-build plan turns SDS #54 into an implementation sequence for Public-Interest Pool Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Public-Interest Pool Service is the Phase 10 service for accountable donated, sponsored, federation, and stewardship-directed capacity. It owns pool definitions, contribution refs, eligible grantee scopes, purpose requirements, quota windows, fairness windows, allocation requests, abuse throttle refs, renewal/revocation records, usage report refs, outcome report refs, and redacted public summaries. It does not define purpose tags globally, create grant authorizations directly, maintain ORU balances, append Seal Ledger entries, schedule work, execute work, decide stewardship priority alone, or turn public-interest capacity into informal free capacity.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #54: Public-Interest Pool Service](../sds/federation_public/public_interest_pool_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Public-Interest Pool Service plan](../service_catalog/federation_public/public_interest_pool_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant boundaries, Overkey signing refs, Overgate ingress, Overregistry service facts, Overwatch audit refs, and Overqueue primitives. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overmeter raw usage facts and execution refs used by pool usage reports without making this service a scheduler or runner. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard decisions, Workload Classifier facts, Oververify evidence, Overclaim correction paths, and challenge/public-provider hardening prerequisites. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overgrant allocation refs, ORU Account projections, Overmeter rollups, Seal Ledger refs, Overbill receipt/settlement context, and accounting evidence without pool-service mutation. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Controls the first build point: trusted federation capacity, stewarded purpose tags, cross-tenant grants, public-interest grant pools, fairness, usage reporting, and accountable public-interest proof. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies later hardening for unknown or semi-trusted public-provider capacity, public sandbox constraints, public-provider fraud refs, payout holds, and throttles. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies later native client and stewardship surfaces that read public-interest pool state, reports, wallet/usage refs, and outcome summaries through APIs. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies public reporting, stewardship review, compliance retention, threat review, incident response, central-AI boundaries, audit export, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #54 first build work aligned to master Phase 10, with earlier phases as prerequisites, Phase 11 as public-provider hardening, and Phase 13 as governance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 10, 11, and 13 | Attach SDS #54, preserve Phase 10 as first build, freeze authority boundaries, and identify prerequisite plus downstream owner-service gates. |
| 2 | Master Phases 0, 1, 4, 5, and 10 | Define Rust contracts, canonical schemas, lifecycle states, reason codes, signed refs, fairness rules, report profiles, and deterministic fixtures. |
| 3 | Master Phases 1, 5, and 10 | Implement pool draft, activation, read, contribution, and lifecycle APIs without owning grants, balances, ledger entries, scheduling, or execution. |
| 4 | Master Phases 4, 5, and 10 | Implement Purpose Tag Registry and Overguard preflight checks for purpose tags, workload/data classes, eligibility evidence, policy refs, and activation gates. |
| 5 | Master Phases 4, 5, and 10 | Implement eligibility, quota, fairness, allocation request, and Overgrant handoff behavior as the core Phase 10 allocation path. |
| 6 | Master Phases 3, 5, 10, and 13 | Implement usage, accounting, Overwatch, and report refs while leaving raw usage, ORU projections, Seal Ledger entries, and audit truth with owner services. |
| 7 | Master Phases 4, 10, 11, and 13 | Implement abuse throttle refs, renewal, revocation, corrections, appeals, and public-provider hardening handoffs without deleting history or punishing directly. |
| 8 | Master Phases 10, 12, and 13 | Implement redacted reports, participant views, outcome hooks, and bounded Central AI recommendation records without direct allocation authority. |
| 9 | Master Phases 10, 11, and 13 | Prove the first Phase 10 pool and define later Phase 11/13 expansion gates for public-provider capacity, governance, and scale. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Public-Interest Pool Service core is a Rust service/module using shared contract crates, Tokio for bounded lifecycle/report workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Pool definitions, contribution refs, grantee eligibility records, allocation requests, fairness windows, usage reports, outcome refs, public-interest recommendation records, events, fixtures, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed service, stewardship, or operator envelopes, tenant or pool scope, trace id, idempotency key, purpose tag refs, policy refs, evidence refs, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for pool snapshots, contribution projections, eligibility evidence snapshots, fairness-window replay bundles, redacted report projections, outcome refs, audit exports, and deterministic fixtures.
- Public-Interest Pool Service may point to Federation Template Service, Purpose Tag Registry, Overgrant, Overguard, Overmeter, ORU Account Service, Seal Ledger, Overbill, Fraud Control Service, Overclaim, Overwatch, Workload Classifier, Central AI Service, Stewardship Reporting Service, SDK, CLI, admin UI, and native client surfaces, but it must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw secret storage, grant authorization, ORU balance mutation, Seal Ledger mutation, workload scheduling, workload execution, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Phase 10 Scope, And Authority Boundaries

### Work Items

- **1.1 Attach the build plan to SDS #54.**
  - Design: Link this document from the Public-Interest Pool Service SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/federation_public/public_interest_pool_service.md`, `docs/service_catalog/federation_public/public_interest_pool_service.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #54 returns both the Public-Interest Pool Service SDS and this sub-build plan.

- **1.2 Preserve master Phase 10 as the first build point.**
  - Design: Keep first implementation in Phase 10 because the service needs trusted federation capacity, stewarded purpose tags, Overgrant allocation refs, ORU/Seal Ledger evidence, and policy verification before public-interest pools can be accountable.
  - Output: Phase-gate note that earlier phases are prerequisites, Phase 10 builds public-interest pool mechanics, Phase 11 adds unknown public-provider hardening, and Phase 13 hardens governance/reporting.
  - Validation: Review proves the plan does not move public-interest pool core work into Phases 0 through 9 or treat Phase 11 public-provider onboarding as the first build point.

- **1.3 Freeze pool-service ownership boundaries.**
  - Design: Record that the service owns pool definitions, contribution refs, eligibility scopes, quota/fairness windows, allocation requests, abuse throttle refs, usage report refs, outcome refs, renewal/revocation records, and redacted summaries.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the service does not define global purpose tags, authorize grants directly, maintain ORU balances, write ledger history, schedule workloads, execute workloads, adjudicate disputes alone, or decide stewardship priorities alone.

- **1.4 Carry forward resolved SDS #54 decisions.**
  - Design: Preserve the decisions for proof-pool purpose-tag scope, grantee eligibility threshold, append-only Central AI recommendation records, audience-specific reporting, and weighted max-min fair-share oversubscription.
  - Output: Resolved-decision checklist tied to proof tags, eligibility evidence, Central AI recommendation authority, public/participant/operator report redaction, and fairness-window policy.
  - Validation: Review rejects inactive-tag omission, opaque central-AI overrides, public leakage of private workload/payment/fraud data, and allocation decisions without Overguard/Overgrant owner-service handoff.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Federation Template Service, Purpose Tag Registry, Overgrant, Overguard, Overmeter, ORU Account Service, Seal Ledger, Overbill, Fraud Control Service, Overclaim, Overwatch, Workload Classifier, Central AI Service, Stewardship Reporting Service, SDK, CLI, admin UI, and native client surfaces.
  - Output: Boundary matrix listing owner, input refs, output refs, freshness rule, redaction class, policy refs, evidence refs, downstream consumer, and rejection behavior.
  - Validation: Review confirms every handoff uses explicit APIs, immutable refs, signed evidence, stable reason codes, trace ids, idempotency keys, policy refs, and Overwatch events rather than privileged shared records or hidden control paths.

## Phase 2: Rust Contracts, Schemas, Lifecycles, And Fixtures

### Work Items

- **2.1 Create the Public-Interest Pool Rust contract module.**
  - Design: Add contract types for pools, contributions, grantee eligibility, allocation requests, fairness windows, usage reports, outcome refs, recommendation records, throttles, renewals, revocations, corrections, events, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, purpose/ref enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overgrant, ORU Account Service, Seal Ledger, scheduling, execution, Purpose Tag Registry, and Overguard internals.

- **2.2 Define pool and contribution schemas.**
  - Design: Model `public_interest_pool` and `pool_contribution` with purpose tag refs, source refs, federation or native-service surplus refs, resource dimensions, availability windows, restrictions, lifecycle state, policy refs, audit refs, and snapshot hashes.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and canonical pool/contribution fixtures.
  - Validation: Schema tests reject missing pool id, source ref, purpose tag ref, resource dimension, availability window, lifecycle state, policy ref, trace id, idempotency key, or contribution audit ref.

- **2.3 Define grantee eligibility and allocation request schemas.**
  - Design: Model eligibility records and allocation requests with grantee identity refs, tenant/org refs, purpose evidence refs, signed attestation refs, workload/data class, requested dimensions, quota windows, Overguard dry-run refs, grant handoff refs, and state.
  - Output: Eligibility schema, allocation request schema, lifecycle examples, denial examples, reason-code catalog, and negative fixtures.
  - Validation: Tests reject records that omit verified identity refs, active purpose-tag validation refs, signed attestation refs, evidence refs, Overguard dry-run refs, quota/fairness facts, or stable denial reasons.

- **2.4 Define fairness, report, recommendation, and correction schemas.**
  - Design: Model `fairness_window`, `pool_usage_report`, `outcome_report_ref`, `public_interest_recommendation`, throttle refs, renewal refs, revocation refs, and correction refs as versioned append-only records.
  - Output: Fairness schema, usage report schema, outcome-ref schema, recommendation schema, correction schema, redaction profiles, replay examples, and supersession examples.
  - Validation: Tests prove fairness windows require cap, burst cap, refill/expiry behavior, weight version, usage refs, throttle overrides, and replay hashes; recommendation records require model/run provenance, confidence, expiry, review state, owner target, and appeal/correction path.

- **2.5 Create deterministic public-interest pool fixtures.**
  - Design: Build fixtures for proof pool creation, contribution attachment, science/education/opensource allocation, inactive medical/climate/public-infrastructure tags, missing evidence, quota exhaustion, fairness conflict, Overguard denial, Overgrant handoff failure, abuse throttle, renewal, revocation, report redaction, and correction.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected reason codes, report hashes, redaction outputs, and replay bundles.
  - Validation: Fixture tests produce stable ids, BLAKE3 hashes, lifecycle states, denial reason codes, audit refs, redacted views, and replay outputs across repeated runs.

## Phase 3: Pool Lifecycle APIs And Contribution Refs

### Work Items

- **3.1 Implement pool draft and read APIs.**
  - Design: Add `POST /public-interest-pools` and `GET /public-interest-pools/{pool_id}` for draft creation and scoped reads of pool status, purpose scope, contribution summaries, eligibility rules, quota status, and redacted reports.
  - Output: API handlers, request/response schemas, signed envelope checks, idempotency behavior, stable errors, and `public_interest_pool.drafted` events.
  - Validation: API tests cover valid drafts, duplicate idempotency keys, missing purpose refs, invalid source refs, unauthorized tenant/stewardship scope, and audience-specific redacted reads.

- **3.2 Implement activation pre-state and lifecycle transitions.**
  - Design: Enforce pool lifecycle transitions from draft to preflighting, active, paused, exhausted, renewal pending, retired, or revoked with explicit policy and evidence refs.
  - Output: State machine module, transition command/API, invalid-transition errors, lifecycle events, timeline projection, and replay fixtures.
  - Validation: Tests prove pools cannot activate without Purpose Tag Registry validation, Overguard preflight, contribution refs, quota/fairness rules, reporting requirements, and accounting-boundary refs.

- **3.3 Implement contribution attachment.**
  - Design: Add `POST /public-interest-pools/{pool_id}/contributions` for donated, sponsored, federation, grant-source, or native-service surplus contribution refs with availability windows and restrictions.
  - Output: Contribution API, contribution state, restriction validator, source-ref validator, contribution availability projection, and `public_interest_pool.contribution_added` events.
  - Validation: Tests prove contribution records store refs rather than balances, prices, payment instructions, raw provider secrets, or direct resource-control state.

- **3.4 Implement pool pause, exhaustion, retirement, and revocation.**
  - Design: Add lifecycle commands for pausing exhausted or unsafe pools, retiring completed pools, revoking invalid policy pools, and preserving final report requirements.
  - Output: Lifecycle APIs or commands, state transition refs, final-report obligations, reason codes, and audit events.
  - Validation: Tests prove paused/revoked/retired pools block new allocations, preserve historical usage/report refs, and do not delete contribution, eligibility, allocation, accounting, or correction history.

- **3.5 Publish pool query projections for operators and participants.**
  - Design: Add query projections for active pools, contribution availability, quota exhaustion, fairness windows, report freshness, renewal deadlines, throttle state, and participant-specific contribution/allocation status.
  - Output: Query APIs, filters, pagination, participant-safe views, operator diagnostics, and Overwatch timeline refs.
  - Validation: Contract tests prove contributors, grantees, operators, stewardship readers, and public readers see only fields allowed by their redaction profile.

## Phase 4: Purpose Tag And Overguard Preflight Gates

### Work Items

- **4.1 Integrate Purpose Tag Registry validation.**
  - Design: Require verified purpose tag version refs and evidence requirements before pool activation, eligibility evaluation, allocation request creation, or public report publication.
  - Output: Purpose Tag Registry adapter, tag version validator, inactive/review-only tag behavior, missing evidence summaries, and `purpose_tag_not_verified` errors.
  - Validation: Tests prove the proof pool registers the full stewarded tag set but only enables `science`, `education`, and `opensource` allocation at first while `medical`, `climate`, and `public_infrastructure` remain inactive or review-only.

- **4.2 Integrate Overguard pool preflight.**
  - Design: Ask Overguard to approve pool eligibility rules, workload/data-class restrictions, quota rules, fairness rules, abuse controls, reporting requirements, and contribution restrictions before activation.
  - Output: Overguard adapter, preflight fact bundle, policy decision refs, stale-policy behavior, denial reason codes, and preflight audit events.
  - Validation: Tests prove policy denial, stale policy refs, missing fact bundles, or Overguard unavailability prevent pool activation rather than allowing unsafe defaults.

- **4.3 Implement grantee/workload eligibility precheck.**
  - Design: Add `POST /public-interest-pools/{pool_id}/eligibility/evaluate` for verified identity, purpose evidence, signed attestation, workload/data class, dry-run policy, abuse state, and dispute state.
  - Output: Eligibility evaluation API, reason-coded result, missing-evidence checklist, review-needed state, and `public_interest_pool.eligibility_evaluated` events.
  - Validation: Tests prove public artifacts can satisfy low-risk proof tags, while private, regulated, sensitive, disputed, or weak-evidence claims return `needs_more_evidence` or `grantee_not_eligible`.

- **4.4 Implement workload and data-class restrictions.**
  - Design: Bind Workload Classifier and Overguard facts so pools only authorize workloads/data classes explicitly allowed by pool policy, purpose tags, contribution restrictions, and trust level.
  - Output: Workload/data-class rule map, restriction errors, review markers, policy refs, and negative fixtures.
  - Validation: Tests prove private, regulated, secret-bearing, or system-service workloads are denied unless a trusted template and specific policy allow them under trusted capacity.

- **4.5 Publish preflight diagnostics and simulation.**
  - Design: Provide side-effect-free diagnostics for draft pools, proposed contributions, grantee requests, purpose tags, quota/fairness rules, and reporting settings before activation or allocation.
  - Output: Simulation API, missing-prerequisite summaries, expected denial reasons, policy refs, and replay packs.
  - Validation: Tests prove simulation is side-effect-free and cannot create pools, activate pools, attach contributions, authorize grants, or mutate eligibility state.

## Phase 5: Eligibility, Quota, Fairness, And Overgrant Handoff

### Work Items

- **5.1 Implement allocation request creation.**
  - Design: Add `POST /public-interest-pools/{pool_id}/allocation-requests` to create request records with grantee refs, purpose tag refs, requested dimensions, time window, policy refs, eligibility refs, fairness refs, and grant refs.
  - Output: Allocation request API, lifecycle state, idempotency behavior, stable errors, and `public_interest_pool.allocation_requested` events.
  - Validation: API tests cover valid requests, duplicate idempotency keys, inactive pools, missing evidence, invalid purpose tags, disallowed workload/data class, exhausted pool, and active throttle denial.

- **5.2 Implement quota and fairness evaluation.**
  - Design: Evaluate per-pool, per-purpose, per-grantee, and per-window quotas with weighted max-min fair-share, equal initial weights, burst caps, refill/expiry behavior, and newcomer/starvation protection.
  - Output: Fairness evaluator, quota evaluator, fairness-window records, replayable calculations, reason-coded denial, and exhaustion events.
  - Validation: Tests prove no grantee can exceed fair share while other eligible grantees remain unsatisfied, unused allocations return after reservation expiry, and abuse throttles override fairness access.

- **5.3 Implement Overgrant handoff.**
  - Design: Hand accepted allocation requests to Overgrant for grant authorization, preserving pool refs, purpose refs, eligibility refs, quota/fairness facts, contribution refs, policy refs, and requested resource dimensions.
  - Output: Overgrant handoff contract, authorization pending state, grant refs, handoff failure behavior, and `public_interest_pool.allocation_authorized` or `allocation_denied` events.
  - Validation: Tests prove Public-Interest Pool Service never creates grant authorizations directly and never invents authorization when Overgrant rejects or is unavailable.

- **5.4 Implement allocation reads and denial explanations.**
  - Design: Add `GET /public-interest-pools/{pool_id}/allocation-requests/{request_id}` for allocation state, quota/fairness status, grant refs, denial reasons, throttle refs, and missing evidence refs.
  - Output: Read API, provider/grantee/operator/stewardship redactions, denial explanation summaries, and timeline refs.
  - Validation: Tests prove public and participant reads are explainable without exposing private workload data, sensitive fraud internals, raw stewardship reasoning, or payment details.

- **5.5 Implement reservation expiry and allocation correction.**
  - Design: Release unused allocation windows back into the pool after reservation expiry and create correction records when eligibility, quota, grant, or usage facts change.
  - Output: Expiry worker contract, correction command, superseded allocation refs, fairness-window updates, and audit events.
  - Validation: Tests prove corrections are append-only, old decisions remain replayable, and expired unused allocations return without deleting grant, usage, or report history.

## Phase 6: Usage, Accounting, Overwatch, And Report Refs

### Work Items

- **6.1 Link Overmeter usage refs.**
  - Design: Connect allocation requests and grant refs to Overmeter usage rollups for CPU/GPU/storage/network/memory/data dimensions without storing raw execution state locally.
  - Output: Overmeter adapter, usage-ref validator, stale/missing usage behavior, usage summary projection, and report fixtures.
  - Validation: Tests prove usage reports cite Overmeter refs and do not schedule, run, meter raw execution directly, or invent usage when owner refs are missing.

- **6.2 Link ORU Account Service and Seal Ledger refs.**
  - Design: Use ORU Account Service and Seal Ledger refs for balance projection and immutable accounting evidence while storing only pool, grant, contribution, usage, and report refs.
  - Output: Accounting-ref adapter, projection summary, ledger-ref validator, reconciliation status, and `report_blocked` reason codes.
  - Validation: Tests prove the service does not maintain balances, append ledger entries, mutate ledger history, create invoices, create payouts, or encode speculative financial assumptions.

- **6.3 Implement usage report generation.**
  - Design: Build `pool_usage_report` records from allocation, grant, usage, accounting, purpose, grantee, contribution, and redaction refs.
  - Output: Report builder, report lifecycle, report hash, redaction profile, stale-report alerts, and `public_interest_pool.report_published` events.
  - Validation: Tests prove reports reconcile with Overmeter and Seal Ledger refs or block publication with stable reason codes when reconciliation fails.

- **6.4 Implement Overwatch audit and trace projection.**
  - Design: Emit audit events for pool lifecycle, contribution changes, eligibility evaluations, allocation requests, grant handoffs, quota exhaustion, throttles, reports, renewals, revocations, and corrections.
  - Output: Overwatch event contracts, trace refs, event fixture set, operator timeline, and audit-export hooks.
  - Validation: Tests prove every mutating API creates an append-only event with actor, tenant/pool scope, trace id, idempotency key, policy refs, evidence refs, and stable reason codes.

- **6.5 Implement reconciliation diagnostics.**
  - Design: Provide diagnostics for missing Overmeter refs, stale grant refs, unreconciled Seal Ledger refs, blocked reports, inconsistent contribution summaries, and quota/report mismatch.
  - Output: Diagnostic API, reason-code catalog, operator dashboard fields, alert refs, and replay bundles.
  - Validation: Tests prove diagnostics are role-scoped and report blockers remain explicit instead of silently publishing inaccurate public-interest summaries.

## Phase 7: Abuse Throttles, Renewal, Revocation, And Corrections

### Work Items

- **7.1 Implement abuse throttle refs.**
  - Design: Add `POST /public-interest-pools/{pool_id}/throttles` to record Fraud Control, Overguard, Overclaim, or stewardship throttle refs for a grantee, purpose, pool, contribution, or allocation scope.
  - Output: Throttle API, throttle lifecycle, duration caps, proportionality refs, correction refs, and `public_interest_pool.throttle_applied` events.
  - Validation: Tests prove throttles limit or deny allocation without deleting usage history, grant refs, accounting refs, or eligibility evidence.

- **7.2 Implement renewal and eligibility expiry.**
  - Design: Add renewal behavior for pool status, contribution windows, purpose tag versions, grantee evidence, quota windows, fairness windows, reporting obligations, and outcome evidence deadlines.
  - Output: Renewal command/API, renewal-required state, expiry worker contract, evidence freshness checks, and renewal events.
  - Validation: Tests prove expired evidence moves eligibility to `renewal_required` and blocks new allocation until evidence and policy checks refresh.

- **7.3 Implement revocation and pool pause flows.**
  - Design: Revoke pool or grantee eligibility when purpose tags, Overguard policy, abuse evidence, contribution availability, reporting obligations, or stewardship decisions invalidate participation.
  - Output: Revocation command/API, pause behavior, final-report obligations, affected allocation query, and correction handoff refs.
  - Validation: Tests prove revocation prevents new allocations while preserving historical accounting, usage, report, and correction records.

- **7.4 Implement correction and appeal handoffs.**
  - Design: Record corrections for false eligibility denials, stale purpose evidence, incorrect usage/report facts, defective fairness rules, throttle mistakes, and accepted Overclaim appeals.
  - Output: Correction API, Overclaim handoff refs, superseded record refs, recommendation/retraction needs, provider/grantee notices, and audit events.
  - Validation: Tests prove corrections are append-only, linked to original records, visible to affected participants through safe summaries, and do not mutate owner-service truth directly.

- **7.5 Integrate Phase 11 public-provider hardening refs.**
  - Design: Prepare handoffs for Public Provider Onboarding, Public Sandbox Profile, Fraud Control Service, Reputation and Anti-Sybil Service, Challenge Task Service, payout holds, and public low-sensitivity workload controls.
  - Output: Public-provider hardening matrix, pool eligibility impact refs, throttle/hold/reputation handoff refs, and public-provider fixtures.
  - Validation: Review confirms Phase 11 hardening consumes pool refs and fraud/throttle refs without moving unknown public-provider onboarding into Phase 10 pool core.

## Phase 8: Redacted Reports, Outcome Hooks, And Central AI Recommendations

### Work Items

- **8.1 Implement public-safe report projection.**
  - Design: Publish active pool status, enabled purpose tag versions, aggregate contribution and consumption dimensions, aggregate grant/source classes, quota/exhaustion summaries, public-safe denial totals, outcome refs, correction notices, and redacted Overwatch/Stewardship Reporting refs.
  - Output: Public report schema, public projection API, redaction profiles, aggregate counters, report hashes, and examples.
  - Validation: Tests prove public reports hide raw private workload data, secrets, payment details, sensitive fraud internals, private identities, and raw central-AI reasoning.

- **8.2 Implement participant report projection.**
  - Design: Provide contributor and grantee views for their own contributions, allocation state, quota/fairness window, grant authorization refs, usage rollups, reporting obligations, outcome evidence refs, throttle refs, and appeal refs.
  - Output: Participant report API, audience-specific field map, pagination/filtering, stale-report markers, and fixtures.
  - Validation: Tests prove participants cannot see other participants' private contribution, workload, grantee, payment, fraud, or stewardship evidence fields.

- **8.3 Implement operator and steward report projection.**
  - Design: Provide stronger evidence refs, identity/contact refs, abuse/fraud refs, central-AI evidence-package details, report freshness, review queues, and correction state under audience-specific policy.
  - Output: Operator/steward report API, authorization checks, redaction policy refs, review-state summaries, and audit refs.
  - Validation: Tests prove operator/steward reports remain policy-bound and do not leak raw private workload data into public artifacts.

- **8.4 Implement outcome-report hooks.**
  - Design: Attach grantee-supplied or stewardship-supplied outcome refs with visibility controls, due dates, review state, public-safe summaries, correction paths, and report inclusion rules.
  - Output: Outcome report API or command, outcome-ref schema, review lifecycle, public-summary projection, and missing-outcome alerts.
  - Validation: Tests prove outcome refs are optional but tracked, public summaries are redacted, and missing outcomes affect reporting/renewal state without deleting allocation history.

- **8.5 Implement append-only Central AI recommendation records.**
  - Design: Store `public_interest_recommendation` records for stewardship suggestions with pool refs, grantee/candidate refs, purpose tag version refs, evidence package refs, fairness/quota facts, usage/outcome refs, model/run provenance, route refs, confidence, proportionality summary, policy threshold refs, expiry, review state, owner target, and appeal/correction path.
  - Output: Recommendation API or ingestion contract, review lifecycle, redacted reasoning summary, owner-service target refs, and recommendation fixtures.
  - Validation: Tests prove Central AI recommendations cannot activate pools, authorize grants, allocate capacity, change quotas, override fairness, revoke eligibility, or bypass Overwatch replay and human/steward review.

## Phase 9: Phase 10 Proof Pool And Later Expansion Gates

### Work Items

- **9.1 Configure the first proof pool.**
  - Design: Create a Phase 10 proof pool that registers the full stewarded purpose-tag set but enables allocation only for `science`, `education`, and `opensource` tag versions.
  - Output: Proof pool fixture, pool config, enabled/inactive tag matrix, contribution refs, quota/fairness policy, report requirements, and expected denial paths.
  - Validation: Scenario tests prove inactive `medical`, `climate`, and `public_infrastructure` tags exist as review-only or inactive records and cannot allocate without later policy approval.

- **9.2 Prove eligible allocation and grant handoff.**
  - Design: Run an end-to-end proof where a verified grantee with acceptable public evidence, signed attestation, Overguard dry-run, no active throttle, and available quota receives an Overgrant handoff.
  - Output: Scenario fixture, eligibility record, allocation request, quota/fairness decision, Overgrant handoff ref, initial usage-report ref, and replay bundle.
  - Validation: Scenario tests prove the path works without informal free capacity, central-AI command authority, local grant authorization, scheduling, execution, balance mutation, or ledger mutation.

- **9.3 Prove denial, throttle, and correction paths.**
  - Design: Run denial scenarios for missing evidence, unverified purpose tag, disallowed workload/data class, quota exhaustion, fairness-window exhaustion, pool exhaustion, unavailable contribution, active abuse throttle, and failed grant handoff.
  - Output: Denial fixtures, stable reason codes, throttle record, correction record, participant-safe explanation, and audit timeline.
  - Validation: Tests prove denials are explainable, correctable, and replayable while preserving historical usage, grant, and report refs.

- **9.4 Define Phase 11 public-provider expansion gate.**
  - Design: Specify what must exist before unknown/semi-trusted public-provider capacity can contribute to public-interest pools: public onboarding, anti-Sybil, public sandbox profile, fraud control, challenge tasks, payout holds, strict workload eligibility, and low-sensitivity proof.
  - Output: Phase 11 readiness checklist, blocked public-provider contribution state, integration targets, and public-provider hardening tests.
  - Validation: Review proves Phase 10 pools accept trusted federation/stewardship capacity first and do not route private, regulated, secret-bearing, system-service, or broad public-provider work prematurely.

- **9.5 Define Phase 13 governance and scale hardening gate.**
  - Design: Specify governance work for stewardship reporting, compliance boundaries, threat modeling, incident response, audit export, report retention, central-AI review boundaries, fairness rule changes, and purpose-tag policy evolution.
  - Output: Governance checklist, PIP hook refs, report retention classes, audit export refs, threat-review targets, incident/compliance handoff matrix, and scale-readiness gate.
  - Validation: Review confirms public-interest allocation is explainable, appealable, replayable, privacy-preserving, policy-versioned, and proportional before broader scale.

## Phase 10: Validation, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack and authority guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, grant-authorization ownership, ORU-balance mutation, Seal-Ledger mutation, scheduling ownership, workload-execution ownership, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools, local stubs, owner-service refs, or downstream APIs into Public-Interest Pool Service's product boundary.

- **10.3 Validate SDS #54 build-breakdown coverage.**
  - Design: Map every SDS #54 build-breakdown item to this plan: schemas, pool APIs, Purpose Tag Registry and Overguard checks, eligibility/quota/fairness, Overgrant handoff, usage/accounting refs, throttles/renewals/revocations/corrections, reports/outcomes, and proof pool.
  - Output: Coverage matrix, API checklist, workflow checklist, fixture checklist, and integration-test targets.
  - Validation: Review proves no SDS #54 build-breakdown item is missing and the plan preserves Public-Interest Pool Service as a Phase 10 pool-allocation and reporting service.

- **10.4 Validate SDS, service catalog, master plan, crosswalk, and queue alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, build-plan crosswalk, queue state, queue progress, and build-plan progress.
  - Output: Updated backlinks and index rows for SDS #54, queue state update, queue progress update, and build-plan progress evidence.
  - Validation: JSON validation passes; local link checks pass; queue validation confirms `054-build-plan` is complete, no materialized task is running, and `055-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders source alignment, contracts, schemas, lifecycle APIs, preflight gates, eligibility/fairness, grant handoff, usage/accounting refs, reports, corrections, proof scenarios, expansion gates, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.

## Alignment Review

- The sub-build plan keeps Public-Interest Pool Service first build work in master Phase 10 because trusted federation capacity, purpose tags, cross-tenant grants, Overguard policy, ORU/Seal Ledger evidence, and public-interest reporting need to exist before accountable public-interest capacity can operate.
- The plan treats Phase 11 as later hardening for unknown or semi-trusted public-provider capacity, public sandbox constraints, fraud controls, challenge tasks, payout holds, and anti-Sybil/reputation signals. It does not move unknown public-provider onboarding into the Phase 10 first build.
- The plan treats Purpose Tag Registry as purpose-tag definition owner; Public-Interest Pool Service stores purpose tag refs and pool-specific requirements without defining the global taxonomy.
- The plan treats Overgrant as grant authorization owner; Public-Interest Pool Service creates allocation requests and handoff refs without authorizing grants locally.
- The plan treats Overguard and Workload Classifier as policy and workload/data-class owners; Public-Interest Pool Service consumes their decision refs and blocks unsafe allocations.
- The plan treats Overmeter, ORU Account Service, Seal Ledger, and Overbill as metering/accounting owners; Public-Interest Pool Service stores usage/accounting refs and report projections without mutating balances, invoices, payouts, or ledger entries.
- The plan treats Fraud Control Service, Overclaim, Public Provider Onboarding, Reputation and Anti-Sybil Service, Challenge Task Service, and Public Sandbox Profile as Phase 11 public-provider hardening owners; Public-Interest Pool Service consumes their refs for throttles, holds, eligibility impact, corrections, and public-provider readiness.
- The plan treats Central AI as a bounded append-only recommendation source; Central AI cannot directly activate pools, authorize grants, allocate capacity, override fairness, revoke eligibility, or bypass Overwatch replay and human/steward review.
- The plan preserves master Phase 0 through Phase 13 ordering and uses earlier phases as prerequisites, Phase 10 as the first public-interest pool build, Phase 11 as public-provider expansion hardening, and Phase 13 as governance/security/compliance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service/contracts, native Overrid boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.
