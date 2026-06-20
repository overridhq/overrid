# SUB BUILD PLAN #55 - Public Provider Onboarding

Attached SDS: [docs/sds/federation_public/public_provider_onboarding.md](../sds/federation_public/public_provider_onboarding.md)

## Purpose

This sub-build plan turns SDS #55 into an implementation sequence for Public Provider Onboarding. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Public Provider Onboarding is the Phase 11 front door for unknown or semi-trusted public providers. It owns enrollment applications, provider contact refs, policy acknowledgements, node enrollment refs, resource claims, public workload acceptance contracts, verification request refs, payout eligibility refs, capability publication refs, and correction/offboarding records. It does not verify identity directly, assign broad trust, supervise nodes, benchmark resources, schedule workloads, execute workloads, decide payouts, decide final reputation, adjudicate fraud, or weaken public sandbox restrictions.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #55: Public Provider Onboarding](../sds/federation_public/public_provider_onboarding.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Public Provider Onboarding plan](../service_catalog/federation_public/public_provider_onboarding.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant boundaries, Overkey signing refs, Overgate ingress, Overregistry service facts, Overwatch audit refs, and Overqueue primitives. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies Overcell/node-agent enrollment, Node Installer version evidence, Hardware Discovery facts, Benchmark Runner evidence, and capability publication prerequisites. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack, Oversched, Overlease, Overrun, and Overmeter execution facts consumed later for public low-sensitivity proof without making onboarding a scheduler or runner. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Oververify, Overguard, Workload Classifier, Challenge Task Service, Overclaim, and policy dry-run inputs needed before public eligibility can be published. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overbill, Provider Payout Service, Overmeter, ORU, Seal Ledger, and payout/hold refs without onboarding mutating balances, payouts, invoices, or ledger entries. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies known-participant federation and public-interest context that remains separate from unknown public-provider onboarding. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Controls the first build point: public-provider onboarding, public sandbox requirements, anti-Sybil/fraud/challenge gates, payout holds, throttles, and bounded low-sensitivity public capacity. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies public reporting, stewardship review, compliance retention, threat review, incident response, audit export, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #55 first build work aligned to master Phase 11, with earlier phases as prerequisites and Phase 13 as later hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 2, 4, 5, 10, 11, and 13 | Attach SDS #55, preserve Phase 11 as first build, freeze onboarding authority boundaries, and identify prerequisite owner-service gates. |
| 2 | Master Phases 0, 1, 2, 4, 5, and 11 | Define Rust contracts, canonical schemas, lifecycle states, reason codes, signed refs, redaction profiles, and deterministic fixtures. |
| 3 | Master Phases 1, 2, and 11 | Implement enrollment, contact capture, policy acknowledgement, provider identity refs, idempotency, and safe provider-facing status. |
| 4 | Master Phases 2, 4, and 11 | Implement node attachment, Overcell/Node Installer refs, software version checks, Hardware Discovery and Benchmark Runner handoffs, and candidate capability handling. |
| 5 | Master Phases 4, 5, and 11 | Implement verification, anti-Sybil, fraud, challenge, payout eligibility, and hold refs without replacing owner-service decisions. |
| 6 | Master Phases 4, 10, and 11 | Implement public workload acceptance contracts and Public Sandbox Profile gates that keep unknown providers limited to public low-sensitivity work. |
| 7 | Master Phases 1, 4, and 11 | Implement Overguard eligibility evaluation and Overregistry capability publication for restricted, expiring, redacted public capability records. |
| 8 | Master Phases 4, 5, 11, and 13 | Implement eligibility reduction, suspension, correction, appeal, offboarding, and correction/retraction handoffs without deleting history. |
| 9 | Master Phases 3, 4, 5, 10, 11, and 13 | Prove an unknown public node can register only into bounded public low-sensitivity capacity and define later scale/governance gates. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Public Provider Onboarding core is a Rust service/module using shared contract crates, Tokio for bounded enrollment/evaluation workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Enrollment records, node enrollment refs, resource claims, public workload acceptance contracts, verification tier refs, payout eligibility refs, capability publications, corrections, events, fixtures, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed provider, service, or operator envelopes, provider/tenant/node scope, trace id, idempotency key, policy refs, evidence refs, schema version, stable reason codes, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for enrollment snapshots, acknowledgement snapshots, verification request bundles, capability publication snapshots, redacted provider summaries, replay bundles, audit exports, and deterministic fixtures.
- Public Provider Onboarding may point to Overpass, Overtenant, Overkey, Overgate, Overcell, Node Installer, Hardware Discovery, Benchmark Runner, Oververify, Reputation and Anti-Sybil Service, Public Sandbox Profile, Workload Classifier, Overguard, Overregistry, Fraud Control Service, Challenge Task Service, Overbill, Provider Payout Service, Overclaim, Overwatch, Oversched, SDK, CLI, and admin UI, but it must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, pricing, revenue projections, customer-count assumptions, raw secret storage, private workload hosting, system-service hosting, direct verification, benchmark ownership, payout mutation, final reputation scoring, fraud adjudication, scheduling, workload execution, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Phase 11 Scope, And Authority Boundaries

### Work Items

- **1.1 Attach the build plan to SDS #55.**
  - Design: Link this document from the Public Provider Onboarding SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/federation_public/public_provider_onboarding.md`, `docs/service_catalog/federation_public/public_provider_onboarding.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #55 returns both the Public Provider Onboarding SDS and this sub-build plan.

- **1.2 Preserve master Phase 11 as the first build point.**
  - Design: Keep first implementation in Phase 11 because unknown public providers require identity, verification, public sandboxing, fraud controls, challenge tasks, payout holds, and strict workload sensitivity gates.
  - Output: Phase-gate note that earlier phases are prerequisites, Phase 10 remains trusted federation/public-interest context, Phase 11 builds onboarding/public-pool controls, and Phase 13 hardens governance.
  - Validation: Review proves the plan does not move unknown public-provider onboarding into Phase 10 or allow public-provider work before Phase 11 gates exist.

- **1.3 Freeze onboarding ownership boundaries.**
  - Design: Record that onboarding owns enrollment applications, contact/policy acknowledgement records, node enrollment refs, declared resource claims, public workload acceptance contracts, payout eligibility refs, capability publication refs, and correction/offboarding records.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms onboarding does not verify identity directly, assign broad trust, install/supervise nodes, benchmark resources, schedule workloads, execute workloads, decide payouts, decide final reputation, adjudicate fraud, or weaken public sandbox restrictions.

- **1.4 Carry forward resolved SDS #55 decisions.**
  - Design: Preserve the `public_provider_limited` first-workload tier, restricted pre-benchmark candidate publication, acknowledgement renewal cadence, payout-delay-only continuation rules, blocking hold states, and safe provider-facing explanation policy.
  - Output: Resolved-decision checklist tied to provider tier, candidate capability confidence, acknowledgement freshness, payout/hold behavior, blocking states, safe reason categories, redaction, and appeal refs.
  - Validation: Review rejects lower-tier requester work, authoritative self-declared capacity, stale acknowledgements, unsafe payout continuation, raw fraud heuristic leakage, and missing appeal/correction paths.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Overpass, Overtenant, Overkey, Overgate, Overcell, Node Installer, Hardware Discovery, Benchmark Runner, Oververify, Reputation and Anti-Sybil Service, Public Sandbox Profile, Workload Classifier, Overguard, Overregistry, Fraud Control Service, Challenge Task Service, Overbill, Provider Payout Service, Overclaim, Overwatch, Oversched, SDK, CLI, and admin UI.
  - Output: Boundary matrix listing owner, input refs, output refs, freshness rule, redaction class, policy refs, evidence refs, downstream consumer, and rejection behavior.
  - Validation: Review confirms every handoff uses explicit APIs, immutable refs, signed evidence, stable reason codes, trace ids, idempotency keys, policy refs, and Overwatch events rather than privileged shared records or hidden control paths.

## Phase 2: Rust Contracts, Schemas, Lifecycles, And Fixtures

### Work Items

- **2.1 Create the Public Provider Onboarding Rust contract module.**
  - Design: Add contract types for provider enrollments, node enrollments, resource claims, public workload acceptance contracts, verification tier refs, payout eligibility refs, capability publications, corrections, events, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, reason-code enums, confidence bands, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Oververify, Reputation and Anti-Sybil, Hardware Discovery, Benchmark Runner, Fraud Control, Overbill, Provider Payout, Overregistry, scheduling, and execution internals.

- **2.2 Define provider enrollment and acknowledgement schemas.**
  - Design: Model `public_provider_enrollment` and acknowledgement records with provider refs, tenant refs, identity level refs, contact refs, region/jurisdiction facts, policy acknowledgement refs, state, submitted time, audit refs, and renewal state.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and canonical enrollment fixtures.
  - Validation: Schema tests reject missing provider ref, tenant ref, identity level ref, required contact refs, region/jurisdiction facts where policy requires them, current acknowledgement refs, trace id, idempotency key, or audit refs.

- **2.3 Define node enrollment and resource claim schemas.**
  - Design: Model `public_node_enrollment` and `resource_claim` with node refs, Overcell refs, software version refs, hardware discovery refs, benchmark refs, sandbox compatibility, declared dimensions, observed evidence refs, confidence, publication state, and expiry.
  - Output: Node schema, resource-claim schema, confidence rules, publication-state examples, and negative fixtures.
  - Validation: Tests reject trusted/schedulable/earning-capable claims without required Overcell, discovery, benchmark, challenge, or policy evidence.

- **2.4 Define eligibility, publication, payout, and correction schemas.**
  - Design: Model `public_workload_acceptance_contract`, `verification_tier_ref`, `payout_eligibility_ref`, `capability_publication`, `onboarding_correction`, suspension, reduction, appeal, and offboarding records as append-only versioned facts.
  - Output: Contract schemas, lifecycle examples, reason-code catalog, redaction profiles, supersession examples, and replay examples.
  - Validation: Tests prove workload acceptance contracts carry allowed workload/data classes, runtime caps, memory caps, egress policy, no-secret/no-private-data/no-system-service flags, sandbox refs, and policy refs.

- **2.5 Create deterministic public-provider onboarding fixtures.**
  - Design: Build fixtures for draft enrollment, missing acknowledgement, insufficient identity, stale software, missing node agent, unverified resource claim, sandbox incompatibility, high anti-Sybil risk, payout hold, candidate capability, publishable public-limited node, correction, appeal, suspension, and offboarding.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected reason codes, redacted summaries, BLAKE3 hashes, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, denial reason codes, audit refs, redacted views, and replay outputs across repeated runs.

## Phase 3: Enrollment, Identity Refs, And Policy Acknowledgement

### Work Items

- **3.1 Implement provider enrollment creation.**
  - Design: Add `POST /public-providers/enrollments` for provider identity refs, tenant refs, contact refs, region/jurisdiction facts, payout eligibility refs, initial policy refs, and traceable submission state.
  - Output: API handler, request/response schemas, signed envelope checks, idempotency behavior, stable errors, and `public_provider_onboarding.enrollment_submitted` events.
  - Validation: API tests cover valid enrollment, duplicate idempotency key, missing identity ref, missing contact ref, missing region where required, invalid tenant scope, and provider-safe error summaries.

- **3.2 Implement policy acknowledgement intake.**
  - Design: Add `POST /public-providers/{provider_id}/acknowledgements` for public-provider, sandbox, fraud-control, payout-hold, dispute, offboarding, and data-restriction terms.
  - Output: Acknowledgement API, acknowledgement snapshot hashes, freshness state, renewal schedule, material-change markers, and `policy_acknowledged` events.
  - Validation: Tests prove missing or stale acknowledgements block first capability publication and new work while preserving historical eligibility, usage, and accounting evidence.

- **3.3 Implement enrollment validation and blocked states.**
  - Design: Validate required fields, identity tier refs, contact refs, region facts, policy refs, source evidence refs, and provider/tenant ownership before moving out of draft/submitted states.
  - Output: Validation module, blocked-state projection, missing-field checklist, stable errors, and provider-facing remediation summaries.
  - Validation: Tests prove `identity_level_insufficient`, `policy_ack_missing`, and related reason codes are stable, explainable, and redacted by audience.

- **3.4 Implement provider-facing status reads.**
  - Design: Add `GET /public-providers/{provider_id}/eligibility` and enrollment status projections with state, blockers, safe reason-code categories, severity/confidence bands, acknowledgement freshness, sandbox refs, and appeal/correction refs.
  - Output: Read API, provider-safe redaction profile, operator redaction profile, pagination/filtering, and status fixtures.
  - Validation: Contract tests prove provider-facing explanations do not expose raw identity or payout material, other-provider identities, exact fingerprints, fraud thresholds, model weights, challenge payloads, topology, operator notes, or internal heuristics.

- **3.5 Publish enrollment audit and support views.**
  - Design: Provide operator and support views for enrollment queues, missing acknowledgements, blocked enrollments, policy version drift, correction requests, and recheck needs.
  - Output: Query APIs, filters, queue projections, Overwatch timeline refs, support-safe summaries, and stale-state alerts.
  - Validation: Tests prove every mutating enrollment or acknowledgement action emits an append-only event with actor, provider/tenant scope, trace id, idempotency key, policy refs, evidence refs, and stable reason codes.

## Phase 4: Node Attachment, Software Version, Discovery, And Benchmark Handoffs

### Work Items

- **4.1 Implement public node attachment.**
  - Design: Add `POST /public-providers/{provider_id}/nodes` for node enrollment refs, Overcell refs, software version refs, heartbeat readiness, network facts, sandbox compatibility, and declared resource claims.
  - Output: Node attachment API, node enrollment state machine, signed envelope checks, idempotency behavior, and `node_attached` events.
  - Validation: Tests cover valid node attachment, missing Overcell ref, mismatched provider ownership, stale heartbeat, stale software, duplicate node refs, and unauthorized tenant scope.

- **4.2 Implement software version and node-agent readiness checks.**
  - Design: Validate Node Installer/Overcell enrollment refs, current heartbeat, supported public-provider software version, update requirements, and rollback/incompatible-version blockers.
  - Output: Readiness checker, software-version policy refs, stale-version errors, remediation summaries, and node readiness projections.
  - Validation: Tests prove `node_agent_missing` and `software_version_stale` block capability publication without deleting the enrollment or node record.

- **4.3 Implement Hardware Discovery handoff.**
  - Design: Request or consume Hardware Discovery refs for coarse CPU, memory, GPU, storage, network, runtime, locality, and sandbox-compatibility facts while treating declared provider claims as untrusted.
  - Output: Discovery handoff contract, source-trust rules, candidate fact projection, confidence cap behavior, and `verification_requested` events where discovery is required.
  - Validation: Tests prove Hardware Discovery or Overcell-observed facts can publish only restricted, expiring, non-authoritative candidate capability presence at confidence no higher than the SDS limit.

- **4.4 Implement Benchmark Runner handoff.**
  - Design: Request or consume Benchmark Runner evidence for measured schedulable capacity, earning-capable capacity, higher confidence publication, and normalized resource cards.
  - Output: Benchmark handoff contract, pending benchmark state, stale benchmark behavior, measured-capacity projection, and benchmark-needed reason codes.
  - Validation: Tests prove self-declared claims remain confidence-capped and cannot become schedulable/earning-capable public capacity without benchmark and challenge evidence.

- **4.5 Publish candidate capability diagnostics.**
  - Design: Expose candidate capability status, missing discovery/benchmark/challenge evidence, confidence bands, expiry, sandbox compatibility, and publication blockers for providers and operators.
  - Output: Diagnostic API, candidate capability fixtures, expiry worker contract, redacted summaries, and audit refs.
  - Validation: Tests prove candidate capability facts may be used only for onboarding, verification, challenges, bounded probes, or heavily capped public test work, never for normal requester workloads.

## Phase 5: Verification, Anti-Sybil, Fraud, Challenge, And Payout Refs

### Work Items

- **5.1 Implement Oververify and anti-Sybil request orchestration.**
  - Design: Add `POST /public-providers/{provider_id}/verification-request` for Oververify, Reputation and Anti-Sybil Service, challenge, benchmark, and discovery follow-up requests based on risk tier.
  - Output: Verification request API, request planner, owner-service target refs, freshness rules, retry behavior, and `verification_requested` events.
  - Validation: Tests prove onboarding requests evidence but does not verify identity, assign reputation, decide final anti-Sybil outcomes, or bypass owner-service refs.

- **5.2 Implement verification tier intake and freshness checks.**
  - Design: Consume Oververify and anti-Sybil outcomes with tier refs, confidence, freshness, appeal status, risk windows, and eligibility recommendation refs.
  - Output: Verification tier projection, freshness checker, appeal status projection, stale evidence reason codes, and replay fixtures.
  - Validation: Tests prove first requester workloads require `public_provider_limited` or stronger eligible refs and Overguard allow decisions for the exact public-low-sensitivity workload.

- **5.3 Implement Fraud Control and Challenge Task handoffs.**
  - Design: Attach Fraud Control refs and Challenge Task Service refs for high-risk enrollment, duplicate execution, result consistency, liveness, capability, sandbox, or public-provider fraud scenarios.
  - Output: Fraud/challenge handoff contract, request rationale schema, active challenge state, fraud-risk blockers, and challenge-result intake.
  - Validation: Tests prove onboarding does not run challenges, expose challenge payload internals, adjudicate fraud, or escalate eligibility solely because a downstream service is unavailable.

- **5.4 Implement payout eligibility and hold refs.**
  - Design: Capture Overbill and Provider Payout Service refs, payout hold refs, payout-delay-only status, blocking hold states, dispute windows, and external payout eligibility without storing payment details.
  - Output: Payout eligibility projection, hold-state validator, continuation policy checker, blocking/non-blocking reason codes, and provider-safe payout status.
  - Validation: Tests prove payout-delay-only states may allow bounded public low-sensitivity work only when Provider Payout Service, Overguard, Oververify, and Reputation/Anti-Sybil agree; blocking states stop new work.

- **5.5 Publish evidence freshness and blocker diagnostics.**
  - Design: Provide diagnostics for stale verification, high anti-Sybil risk, active fraud signals, unresolved challenges, payout holds, missing discovery/benchmark refs, and appeal/correction needs.
  - Output: Diagnostic API, operator queues, provider-safe blockers, Overwatch timeline refs, and remediation summaries.
  - Validation: Tests prove diagnostics are explainable, role-scoped, replayable, and do not leak raw fraud, anti-Sybil, payout, challenge, topology, or private tenant evidence.

## Phase 6: Public Workload Acceptance And Sandbox Gates

### Work Items

- **6.1 Implement public workload acceptance contracts.**
  - Design: Define allowed workload classes, data classes, runtime caps, memory caps, egress policy, no-secret flag, no-private-data flag, no-system-service flag, sandbox profile ref, and policy refs for each public provider/node.
  - Output: Contract builder, acceptance API or command, lifecycle state, stable errors, and `public_workload_acceptance_contract` fixtures.
  - Validation: Tests prove private, regulated, secret-bearing, tenant-private, and system-service workloads are denied for public providers regardless of provider claim quality.

- **6.2 Integrate Public Sandbox Profile compatibility.**
  - Design: Require Public Sandbox Profile refs for filesystem, network, runtime, memory, output validation, artifact quarantine, log privacy, and no-secret/no-private-data controls before capability publication.
  - Output: Sandbox profile adapter, compatibility checker, missing profile errors, remediation summaries, and sandbox readiness projection.
  - Validation: Tests prove `sandbox_not_compatible` prevents capability publication and normal work until fixed.

- **6.3 Integrate Workload Classifier and Overguard class checks.**
  - Design: Bind workload/data-class facts to Overguard policy decisions so public providers can accept only approved public low-sensitivity workloads under current policy.
  - Output: Class-check adapter, policy fact bundle, denial reason codes, stale-policy behavior, and workload acceptance replay fixtures.
  - Validation: Tests prove normal scheduler behavior cannot accidentally place private, regulated, secret-bearing, or system-service workloads on public nodes.

- **6.4 Implement acknowledgement change and renewal enforcement.**
  - Design: Block new capability publication and new work when material public-provider policy, sandbox, payout/dispute, region/compliance, or workload-acceptance contract changes are not reacknowledged.
  - Output: Reacknowledgement scheduler, freshness policy, stricter probationary cadence, stale acknowledgement state, and provider-facing renewal prompts.
  - Validation: Tests prove active providers reacknowledge on the SDS cadence and stricter states use the shorter cadence while historical evidence remains append-only.

- **6.5 Publish side-effect-free eligibility simulations.**
  - Design: Provide previews for proposed public workload acceptance, sandbox compatibility, class restrictions, region policy, evidence freshness, and provider-specific blockers before publication.
  - Output: Simulation API, missing-prerequisite summaries, expected denial reasons, policy refs, and replay packs.
  - Validation: Tests prove simulation cannot create enrollments, publish capabilities, alter eligibility, request payouts, schedule work, or mutate owner-service state.

## Phase 7: Overguard Evaluation And Overregistry Capability Publication

### Work Items

- **7.1 Implement eligibility evaluation.**
  - Design: Add `POST /public-providers/{provider_id}/eligibility/evaluate` to combine identity tier, node readiness, software, discovery/benchmark/challenge evidence, payout state, fraud/anti-Sybil refs, sandbox compatibility, workload contract, and Overguard policy refs.
  - Output: Eligibility evaluator, fact bundle schema, stable allow/deny/review outcomes, reason-code catalog, and `eligibility_evaluated` events.
  - Validation: Tests cover eligible public-limited providers, missing acknowledgement, stale software, unverified resource claims, sandbox incompatibility, payout hold blockers, anti-Sybil high risk, and disallowed workload class.

- **7.2 Implement capability publication preflight.**
  - Design: Require current Overguard allow decision, verified/restricted capability scope, redaction profile, expiry, registry target refs, policy refs, evidence refs, and publication idempotency before publishing.
  - Output: Publication preflight checker, missing-prerequisite errors, publication snapshot hash, and preflight replay fixtures.
  - Validation: Tests prove failed Overguard, stale evidence, missing sandbox refs, unsafe workload classes, or public-private data ambiguity block publication.

- **7.3 Implement Overregistry capability publication.**
  - Design: Add `POST /public-providers/{provider_id}/publish-capabilities` for allowed public provider/node/capability facts with public-limited scopes, expiry, policy refs, registry refs, and redacted provider facts.
  - Output: Publication API, Overregistry adapter, capability lifecycle state, supersession behavior, and `capabilities_published` events.
  - Validation: Tests prove published records include only public low-sensitivity scopes and redacted facts, never raw contact, identity, payout, private workload, secret, topology, or fraud internals.

- **7.4 Implement publication expiry, reduction, and revocation.**
  - Design: Expire, reduce, suspend, or revoke capability publications when evidence freshness, software version, sandbox compatibility, fraud, anti-Sybil, payout, policy, or correction state changes.
  - Output: Expiry worker contract, reduction command, revocation command, registry update refs, and `eligibility_reduced`/`suspended` events.
  - Validation: Tests prove changes create new records and downstream update refs without deleting original publication history.

- **7.5 Publish registry and scheduler handoff projections.**
  - Design: Provide Oversched, Overguard, Public Sandbox Profile, Fraud Control, Reputation and Anti-Sybil, SDK, CLI, and admin UI with published eligibility/capability refs rather than provider-submitted claims.
  - Output: Handoff projections, consumer field map, freshness/expiry semantics, redaction profiles, and integration fixtures.
  - Validation: Integration tests prove downstream consumers cannot treat raw provider claims as trusted capacity and cannot use expired/revoked capability refs for placement.

## Phase 8: Suspension, Reduction, Correction, Appeal, And Offboarding

### Work Items

- **8.1 Implement suspension and eligibility reduction.**
  - Design: Add `POST /public-providers/{provider_id}/suspend` and reduction commands for fraud signals, failed verification, anti-Sybil risk, stale software, missing acknowledgements, sandbox incompatibility, payout blockers, or policy denial.
  - Output: Suspension API, reduction command, lifecycle transitions, affected publication refs, provider notices, and `suspended`/`eligibility_reduced` events.
  - Validation: Tests prove suspension/reduction blocks new work or reduces published scope while preserving enrollment, publication, usage, payout, and audit history.

- **8.2 Implement correction and appeal intake.**
  - Design: Add `POST /public-providers/{provider_id}/corrections` for provider correction, appeal, offboarding evidence, false fraud signals, incorrect identity facts, stale software fixes, payout status corrections, and accepted owner-service decisions.
  - Output: Correction API, appeal status projection, owner-service handoff refs, superseded evaluation refs, provider-safe response, and `correction_recorded` events.
  - Validation: Tests prove corrections are append-only, linked to original records, visible to affected providers through safe summaries, and do not mutate owner-service truth directly.

- **8.3 Implement eligibility restoration and re-publication.**
  - Design: Reevaluate corrected providers/nodes with current policy, verification, sandbox, payout, fraud, anti-Sybil, challenge, discovery, and benchmark refs before restoring or republishing capabilities.
  - Output: Restoration workflow, republish command, superseded publication refs, Overregistry update refs, and replay fixtures.
  - Validation: Tests prove restoration requires current evidence and Overguard allow decisions; stale correction evidence cannot silently restore public eligibility.

- **8.4 Implement offboarding flows.**
  - Design: Support provider-initiated and operator-initiated offboarding with publication revocation, pending work block, payout/hold visibility, correction/appeal preservation, and downstream notice refs.
  - Output: Offboarding command/API, final state projection, downstream update refs, redacted summaries, and `offboarded` events.
  - Validation: Tests prove offboarding blocks new work, preserves audit/accounting/correction history, and does not delete evidence needed for disputes, reports, or governance review.

- **8.5 Publish correction, appeal, and offboarding reports.**
  - Design: Provide provider, operator, stewardship, and aggregate views for blocked reasons, active appeals, accepted corrections, denied corrections, restored eligibility, offboarded providers, and false-positive trends.
  - Output: Summary APIs, redacted report profiles, aggregate counters, Overwatch refs, and stewardship report hooks.
  - Validation: Tests prove reports are explainable and correctable while hiding raw identity, payout, fraud, anti-Sybil, challenge, topology, private tenant, and operator-note details outside authorized audiences.

## Phase 9: Public-Limited Proof And Scale/Governance Gates

### Work Items

- **9.1 Configure the first public-limited provider proof.**
  - Design: Build a proof fixture for a new public provider with verified account control, reachable contact refs, required region refs, current acknowledgements, Overcell node, current heartbeat/software, sandbox compatibility, public-low-sensitivity Oververify eligibility, acceptable anti-Sybil risk, and Overguard allow decision.
  - Output: Proof provider fixture, node fixture, acknowledgement refs, verification refs, sandbox refs, payout refs, capability publication refs, and expected denial paths.
  - Validation: Scenario tests prove lower tiers can enroll and run onboarding checks but cannot receive requester workloads.

- **9.2 Prove candidate capability handling before benchmark evidence.**
  - Design: Run a scenario where declared resource claims publish only restricted, expiring, non-authoritative candidate facts until discovery/benchmark/challenge evidence raises confidence.
  - Output: Candidate capability scenario, confidence cap records, expiration behavior, blocked schedulable capacity, and replay bundle.
  - Validation: Tests prove self-declared claims do not become trusted schedulable/earning-capable capacity and Oversched can use candidate facts only for probes, challenges, or heavily capped public test work.

- **9.3 Prove first bounded public low-sensitivity workload eligibility.**
  - Design: Run an end-to-end eligibility proof where a provider reaches `public_provider_limited`, publishes redacted capability refs, and becomes eligible only for a capped public low-sensitivity workload through policy and sandbox rails.
  - Output: Scenario fixture, eligibility evaluation, Overguard allow ref, Overregistry publication, scheduler-readable refs, and audit timeline.
  - Validation: Scenario tests prove private, regulated, secret-bearing, and system-service workloads remain denied and no owner-service boundary is bypassed.

- **9.4 Prove fraud, hold, correction, and appeal paths.**
  - Design: Run scenarios for failed verification, high anti-Sybil risk, stale software, sandbox incompatibility, payout hold, fraud signal after publication, provider correction, accepted appeal, and offboarding.
  - Output: Scenario fixtures, stable reason codes, suspension/reduction records, correction records, appeal refs, republish refs, and replay bundles.
  - Validation: Tests prove denials are explainable, correctable, and replayable while preserving historical usage, payout, registry, and audit refs.

- **9.5 Define Phase 13 scale and governance hardening gates.**
  - Design: Specify governance work for public-provider reporting, compliance boundaries, threat modeling, incident response, audit export, redaction review, fraud/anti-Sybil model review, payout/custody boundary review, and provider-support operations.
  - Output: Governance checklist, public-provider report classes, retention classes, incident/compliance handoff matrix, threat-review targets, and scale-readiness gate.
  - Validation: Review confirms public-provider onboarding is bounded, explainable, appealable, replayable, privacy-preserving, and proportional before wider public-provider scale.

## Phase 10: Validation, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack and authority guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, private-workload hosting, system-service hosting, direct-verification ownership, benchmark ownership, payout mutation, final reputation, fraud adjudication, scheduling, workload execution, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools, local stubs, owner-service refs, or downstream APIs into Public Provider Onboarding's product boundary.

- **10.3 Validate SDS #55 build-breakdown coverage.**
  - Design: Map every SDS #55 build-breakdown item to this plan: schemas, enrollment/acknowledgement APIs, node attachment, software/discovery/benchmark handoff, verification/anti-Sybil/fraud/challenge/payout refs, sandbox gates, Overguard evaluation, Overregistry publication, suspension/reduction/correction/appeal/offboarding, and bounded public-node proof.
  - Output: Coverage matrix, API checklist, workflow checklist, fixture checklist, and integration-test targets.
  - Validation: Review proves no SDS #55 build-breakdown item is missing and the plan preserves Public Provider Onboarding as a Phase 11 public-provider eligibility and publication service.

- **10.4 Validate SDS, service catalog, master plan, crosswalk, and queue alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, build-plan crosswalk, queue state, queue progress, and build-plan progress.
  - Output: Updated backlinks and index rows for SDS #55, queue state update, queue progress update, and build-plan progress evidence.
  - Validation: JSON validation passes; local link checks pass; queue validation confirms `055-build-plan` is complete, no materialized task is running, and `056-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders source alignment, contracts, schemas, enrollment APIs, node evidence handoffs, verification refs, sandbox gates, eligibility/publication behavior, corrections, proof scenarios, scale gates, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.

## Alignment Review

- The sub-build plan keeps Public Provider Onboarding first build work in master Phase 11 because unknown or semi-trusted public providers require strict low-sensitivity workload boundaries, public sandboxing, fraud controls, challenge tasks, payout holds, anti-Sybil signals, verification refs, and correction paths.
- The plan treats Phase 10 trusted federation and public-interest pools as upstream context and downstream consumers, not as the first build point for unknown public-provider onboarding. Known-participant federation remains separate from adversarial unknown public-provider supply.
- The plan treats Oververify and Reputation and Anti-Sybil Service as verification and reputation owners; Public Provider Onboarding consumes their refs without assigning final trust or reputation.
- The plan treats Hardware Discovery and Benchmark Runner as evidence owners; Public Provider Onboarding requests and consumes evidence without benchmarking or trusting self-declared claims.
- The plan treats Public Sandbox Profile, Workload Classifier, and Overguard as sandbox, workload/data-class, and policy owners; Public Provider Onboarding consumes their decisions and blocks unsafe publication.
- The plan treats Overregistry as capability publication owner-of-record; Public Provider Onboarding publishes only redacted, restricted, expiring public-limited capability refs.
- The plan treats Overbill and Provider Payout Service as payout/hold owners; Public Provider Onboarding stores payout eligibility and hold refs without mutating payouts, invoices, balances, or payment details.
- The plan treats Fraud Control Service, Challenge Task Service, Overclaim, and Overwatch as fraud/challenge/dispute/audit owners; Public Provider Onboarding stores refs and handoffs without adjudicating fraud, running challenges, or deleting history.
- The plan preserves master Phase 0 through Phase 13 ordering and uses earlier phases as prerequisites, Phase 11 as the first public-provider onboarding build, and Phase 13 as governance/security/compliance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service/contracts, native Overrid boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.
