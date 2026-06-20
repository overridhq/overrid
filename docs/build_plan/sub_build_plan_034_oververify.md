# SUB BUILD PLAN #34 - Oververify

Attached SDS: [docs/sds/trust_policy_verification/oververify.md](../sds/trust_policy_verification/oververify.md)

## Purpose

This sub-build plan turns SDS #34 into an implementation sequence for Oververify. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Oververify is the evidence-backed verification and eligibility publication service for providers and nodes. It validates and normalizes evidence refs, computes explainable trust and eligibility signals, publishes workload-class-specific status, and records certification and recomputation history. It does not run benchmarks, orchestrate challenges, schedule workloads, enforce admission policy directly, mutate payouts, bill, settle, or hide trust behind opaque global scores.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #34: Oververify](../sds/trust_policy_verification/oververify.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Oververify service plan](../service_catalog/trust_policy_verification/oververify.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, signed envelopes, idempotency, trace ids, stable reason codes, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overtenant, Overregistry, Overkey, Overwatch, Overqueue, identity, tenant, credential, audit, and command primitives used by verification evidence and state-changing APIs. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies founder seed nodes, Overcell enrollment, hardware discovery, benchmark evidence, heartbeat refs, command-acceptance refs, and private bootstrap verification evidence. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies workload outcomes, raw usage facts, scheduler/lease context, result consistency refs, and execution evidence that can affect reliability and eligibility windows. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Controls Oververify's first build point for provider/node verification records, evidence validation, explainable signals, eligibility publication, certification, and recompute history. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies accounting consumers for payout eligibility and hold recommendation refs while keeping actual holds, settlement, corrections, and ledger mutation in accounting owners. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service placement, failover, restore, maintenance, and grid-resident hardening for Oververify as a backbone trust service. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase persistence, Overstore evidence artifacts, Overvault private/compliance refs, and namespace/storage references used by replay and long-term evidence access. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Controls public-provider verification hardening, anti-Sybil refs, public sandbox eligibility, duplicate-execution fraud refs, payout-risk refs, and strict low-sensitivity boundaries. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies SDK, CLI, admin UI, wallet/usage, native app, provider-facing, and stewardship consumers of verification status, explanations, reason codes, and remediation hints. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies stewardship, compliance, incident, threat-model, PIP, retention, migration, audit-export, and scale hardening for verification policies and evidence handling. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #34 first build work aligned to master Phase 4, with Phase 11 public-provider expansion and later grid, storage, native-app, accounting, and governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid evidence/storage/accounting boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, or customer-count drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 2, 3, 4, 5, 7, 8, 11, 12, and 13 | Attach SDS #34, freeze Oververify authority, preserve Phase 4 as first build point, and record later public, accounting, grid, storage, native-app, and governance gates. |
| 2 | Master Phases 0, 1, and 4 | Build Rust contracts, canonical schemas, state enums, reason codes, evidence categories, and deterministic fixtures before evidence affects eligibility. |
| 3 | Master Phases 1, 2, and 4 | Implement provider/node verification records and authenticated evidence intake from Overregistry, Overcell, Hardware Discovery, Benchmark Runner, and Overwatch refs. |
| 4 | Master Phases 2, 3, 4, 5, and 11 | Ingest benchmark, challenge, reliability, dispute, abuse, and public-provider risk refs without owning benchmark execution, challenge orchestration, scheduling, or accounting effects. |
| 5 | Master Phases 3, 4, 5, 11, and 13 | Compute policy-versioned trust and eligibility signals, freshness decay, payout-hold recommendation refs, and workload-class restrictions. |
| 6 | Master Phases 4, 7, 11, and 13 | Manage certification, probation, degraded, recheck-required, suspension, revocation, reinstatement, and public-provider eligibility lifecycle. |
| 7 | Master Phases 4, 12, and 13 | Expose authorized status, eligibility, explanation, remediation, redaction, SDK, CLI, admin UI, and stewardship-facing projections. |
| 8 | Master Phases 4, 7, 8, and 13 | Implement recompute, replay, scoped backfill, native persistence handoff, retention, operational dashboards, alerts, and evidence export. |
| 9 | Master Phases 4, 5, 11, 12, and 13 | Wire Oververify into Overguard, Oversched, Challenge Task Service, Benchmark Runner, Overclaim, payout/accounting, public-provider, native-app, and central AI consumers. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, phase boundaries, negative controls, queue state, and implementation handoff gates. |

## Tech Stack Guardrails

- Oververify core is a Rust service/module using shared contract types, Tokio for async validation/recompute workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Provider records, node records, evidence refs, benchmark refs, challenge outcome refs, reliability windows, dispute markers, abuse markers, trust signals, eligibility signals, certification records, explanation bundles, recompute jobs, events, API errors, fixtures, and reason-code catalogs use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating APIs require signed actor or service envelopes, tenant/provider/node scope where applicable, idempotency keys, trace ids, schema versions, policy refs, evidence refs, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for evidence commitments, replay bundles, schema fixtures, explanation exports, and deterministic golden tests.
- Oververify stores and publishes evidence-backed refs and may later persist durable records through native Overbase, evidence artifacts through Overstore, and private/compliance refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, or external workflow products the platform boundary.
- Oververify consumes benchmark and challenge results; it does not run benchmarks, select challenge payloads, execute challenges, or own duplicate-execution orchestration.
- Oververify emits eligibility, payout-eligibility, and payout-hold recommendation refs. It does not reserve, hold, release, settle, refund, correct, bill, invoice, price, or mutate ledger/accounting state.
- Trust output is a set of evidence-backed, workload-class-specific signals with reason codes, confidence, freshness, and policy/evaluator version. A single unexplained trust score is not an acceptable product boundary.
- Public-provider eligibility stays restricted to explicit public low-sensitivity classes with no private data, no secrets, no regulated data, no system-service work, anti-Sybil/fraud refs, sandbox constraints, rate limits, and volatile eligibility windows.
- Provider-facing explanations must be actionable and appealable while hiding private tenant data, fraud heuristics, challenge payload selection, comparator internals, other-provider data, raw identity/payout material, operator notes, exact topology, and sensitive incident details.
- Planning and implementation must avoid blockchain, NFT, speculative token mechanics, pricing tables, revenue projections, customer-count assumptions, per-signal external payment calls, and hidden dependency reads.

## Phase 1: SDS Attachment, Verification Authority, And Evidence Boundaries

### Work Items

- **1.1 Attach the build plan to SDS #34.**
  - Design: Link this document from the numbered Oververify SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/trust_policy_verification/oververify.md`, `docs/service_catalog/trust_policy_verification/oververify.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #34 returns both the Oververify SDS and this sub-build plan.

- **1.2 Freeze Oververify as verification and eligibility publication authority only.**
  - Design: Record that Oververify owns provider/node verification records, evidence intake validation, trust signals, eligibility signals, certification records, explanation bundles, lifecycle actions, and recompute history.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Oververify does not own benchmark execution, challenge orchestration, scheduler placement, admission-policy enforcement, queue state, lease reservation, runner execution, payout holds, billing, settlement, ledger mutation, or opaque global reputation scoring.

- **1.3 Preserve master Phase 4 as the first build point.**
  - Design: Keep first implementation in Phase 4 because Oververify needs Phase 0 contracts, Phase 1 identities/audit, Phase 2 node and benchmark evidence, and Phase 3 execution/reliability facts before its signals can affect placement and policy.
  - Output: Phase-gate note that Phase 5 supplies accounting consumers, Phase 7 grid-resident operation, Phase 8 native persistence/evidence storage, Phase 11 public-provider hardening, Phase 12 client consumers, and Phase 13 governance hardening.
  - Validation: Review proves this plan does not move public-provider broadening, settlement mutation, native persistence, native-app surfaces, or governance overrides into Phase 4 prematurely.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #34 decisions for workload-class-specific mandatory evidence, founder seed versus federation eligibility, policy-versioned evidence decay, payout-hold recommendation limits, and actionable redacted explanations.
  - Output: Resolved-decision checklist tied to SDS #34 open-question answers.
  - Validation: Review rejects one-size-fits-all evidence, permanent founder trust, public-provider inheritance into private/regulated/system work, unexplained global scores, low-severity payout holds without payable risk, and forensic provider-facing disclosures.

- **1.5 Define fact-owner and consumer boundaries.**
  - Design: Create a dependency matrix for Overregistry, Overcell, Hardware Discovery, Benchmark Runner, Challenge Task Service, Overwatch, Overclaim, Overguard, Oversched, Provider Payout Service, Overbill, Seal Ledger, Overgrant, Overmark, Public Provider Onboarding, Fraud Control Service, Reputation and Anti-Sybil Service, central AI stewardship, SDK, CLI, admin UI, and native apps.
  - Output: Boundary matrix listing consumed refs, emitted signal refs, final authority owner, freshness owner, redaction profile, replay evidence, policy version, lifecycle effect, and later phase gate for each dependency.
  - Validation: Review confirms every handoff uses explicit APIs, source-authenticated evidence refs, policy/evaluator versions, reason codes, trace ids, and Overwatch audit rather than privileged direct state reads.

## Phase 2: Rust Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the Oververify Rust contract module.**
  - Design: Add contract types for provider verification records, node verification records, attestation evidence, benchmark evidence refs, challenge outcome refs, reliability windows, dispute markers, abuse markers, trust signals, eligibility signals, certification records, explanation bundles, recompute jobs, API errors, and events.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, state enums, reason-code enums, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from benchmark runner, challenge service, scheduler, policy-engine, and accounting internals.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for provider evidence ingest, node evidence ingest, benchmark evidence ingest, challenge outcome ingest, dispute ingest, recompute, provider status read, node status read, eligibility read, explanation, certification, and state-change APIs.
  - Output: Schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing source service, target id, evidence refs, signature or integrity hash where required, policy version, evaluator version, trace id, idempotency key, reason code, or redaction class.

- **2.3 Model verification, evidence, signal, and certification state machines.**
  - Design: Encode provider/node lifecycle states, evidence lifecycle states, certification lifecycle states, recompute job states, signal replacement links, terminal-state behavior, and legal transitions.
  - Output: State transition tables, Rust enums, JSON Schema enums, event emission rules, and invalid transition fixtures.
  - Validation: State tests reject mutable signal updates, reinstatement without evidence refs, certification renewal without policy refs, evidence acceptance after revocation, and signal consumption after expiry.

- **2.4 Define evidence category and reason-code catalogs.**
  - Design: Model identity, ownership, enrollment, heartbeat, command acceptance, software version, hardware inventory, benchmark, challenge, reliability, dispute, abuse, anti-Sybil, payout, compliance, certification, and operator-action categories with user-safe and operator-only reason codes.
  - Output: Catalog files, stable identifiers, remediation metadata, severity bands, redaction classes, deprecation rules, and generated client fixtures.
  - Validation: Tests prove code identifiers remain stable when wording changes and operator-only categories still expose wrapper code, trace id, appeal/support path where allowed, and redacted evidence refs.

- **2.5 Create deterministic verification fixtures.**
  - Design: Build fixtures for unverified provider, founder seed node, trusted federation node, eligible private tenant node, stale benchmark, challenge failure, active dispute, abuse marker, certification renewal, payout-hold recommendation, public-provider probation, and accepted Overclaim correction.
  - Output: Fixture directory, expected API responses, Overwatch events, signal hashes, replay bundles, redacted explanations, and invalid examples.
  - Validation: Fixture tests produce stable signals, reason codes, signal hashes, evidence refs, redacted explanations, and lifecycle states across repeated runs.

## Phase 3: Provider, Node, And Attestation Evidence Intake

### Work Items

- **3.1 Implement provider verification records from Overregistry refs.**
  - Design: Create provider records from provider identity, organization, ownership, contact, payout eligibility, accepted workload classes, jurisdiction, policy acknowledgement, and verification state refs supplied by owning services.
  - Output: Provider record repository interface, create/read/update command handlers, source-ref validator, immutable history links, and `oververify.provider_record_created` events.
  - Validation: API tests reject unknown provider refs, missing policy acknowledgement, unauthorized service source, conflicting identity refs, and broad eligibility without workload-class-specific evidence.

- **3.2 Implement node verification records from Overcell and registry refs.**
  - Design: Create node records from node id, provider id, enrollment refs, heartbeat refs, command-acceptance refs, hardware inventory refs, benchmark refs, software version refs, trust class, workload eligibility, recheck requirements, and node state.
  - Output: Node record repository interface, enrollment binding validator, provider-node ownership check, freshness metadata, and `oververify.node_record_created` events.
  - Validation: Tests reject orphan nodes, stale enrollment refs, provider/node mismatch, unsupported node class, missing Overcell command-acceptance refs for active placement, and public-provider inheritance into private or system-service eligibility.

- **3.3 Implement authenticated evidence intake.**
  - Design: Support evidence attachment from approved source services with source service id, evidence type, target id, policy/evaluator refs, integrity hash or signature, expiry, revocation refs, and validation state.
  - Output: Evidence intake handlers, source-service registry, signature/hash verifier, idempotent evidence writes, and `oververify.evidence_attached` or `oververify.evidence_rejected` events.
  - Validation: Evidence tests reject unauthenticated sources, replayed idempotency keys with conflicting bodies, target mismatch, unsupported evidence type, expired evidence, revoked evidence, and missing trace ids.

- **3.4 Validate freshness, target identity, and policy compatibility.**
  - Design: Apply source-specific freshness windows, target binding rules, policy-version compatibility, evidence expiry, supersession, and stricter-rule conflict handling before evidence can affect signals.
  - Output: Freshness evaluator, target identity checker, policy compatibility checker, supersession links, rejection reason codes, and review-required flags.
  - Validation: Tests prove stale heartbeat, stale benchmark, stale challenge pass, conflicting provider identity, incompatible policy version, and superseded evidence cannot silently increase eligibility.

- **3.5 Publish intake audit trails and authorized reads.**
  - Design: Emit audit events for record creation, evidence acceptance, rejection, supersession, expiry, revocation, and target-link updates while exposing only authorized projections to providers, operators, auditors, and internal consumers.
  - Output: Event schemas, read projections, redaction profiles, audit export fixtures, and trace propagation.
  - Validation: Authorization tests prove provider views hide other-provider evidence, private tenant refs, raw identity/payout material, fraud heuristics, operator notes, and challenge internals.

## Phase 4: Benchmark, Challenge, Reliability, Dispute, And Abuse Evidence

### Work Items

- **4.1 Ingest benchmark evidence refs without running benchmarks.**
  - Design: Accept Benchmark Runner refs for suite id/version, node id, measured resource dimensions, raw sample refs, normalized result refs, anomaly refs, invalidation refs, and freshness windows.
  - Output: Benchmark ingest handler, resource-dimension freshness model, anomaly/invalidation mapping, benchmark-accepted events, and capacity-confidence inputs.
  - Validation: Tests prove Oververify rejects fabricated or impossible benchmark claims, expired samples, wrong-node samples, missing raw sample refs, and attempts to invoke benchmark execution from Oververify.

- **4.2 Ingest Challenge Task Service outcome refs without orchestrating challenges.**
  - Design: Accept challenge assignment id, challenge type, result state, severity, evidence refs, consequence proposal refs, dispute refs, expiry, and recheck windows from Challenge Task Service.
  - Output: Challenge outcome ingest handler, severity mapper, recheck requirement mapper, consequence proposal reader, challenge-accepted events, and challenge-failure reason codes.
  - Validation: Tests prove Oververify consumes challenge outcomes only after source validation and does not select payloads, issue challenge assignments, execute checks, or expose comparator internals.

- **4.3 Build reliability windows from execution and incident refs.**
  - Design: Aggregate completed workloads, failures, timeouts, cancellations, no-shows, result inconsistencies, incident refs, and confidence explanation fields into provider or node reliability windows.
  - Output: Reliability window model, aggregation worker, window freshness rules, incident binding, reason-code mapping, and reliability-change events.
  - Validation: Tests prove reliability windows are append-only, policy-versioned, bounded by source facts, and cannot erase prior failures or inflate confidence without fresh evidence.

- **4.4 Ingest Overclaim dispute outcomes and corrections.**
  - Design: Accept claim id, affected refs, status, resolution, severity, hold refs, correction refs, appeal state, and impact-on-verification from Overclaim without owning dispute adjudication.
  - Output: Dispute marker model, correction marker model, recompute trigger, appeal-aware visibility, and dispute-impact reason codes.
  - Validation: Tests prove active high-severity disputes restrict affected eligibility, accepted corrections trigger recompute, prior evidence remains linked, and Oververify does not mutate claims, holds, finality, or settlement records.

- **4.5 Ingest abuse and public-provider risk markers.**
  - Design: Accept abuse markers from approved sources with marker type, severity, confidence, evidence refs, expiry, appeal refs, visibility class, anti-Sybil refs, and public-provider risk refs.
  - Output: Abuse marker model, public-provider risk adapter, redaction rules, marker expiry behavior, and review-required mappings.
  - Validation: Tests prove severe abuse blocks or suspends affected classes, low-confidence markers do not over-trigger payout holds, provider-facing output hides fraud heuristics, and public-provider markers never grant private or secret-bearing eligibility.

## Phase 5: Trust Signals, Eligibility Signals, Decay, And Hold Recommendations

### Work Items

- **5.1 Implement policy-versioned signal computation.**
  - Design: Evaluate evidence categories against verification policy and evaluator versions to create append-only trust signals with target id, signal domain, signal value, allowed/denied workload classes, confidence, reason codes, evidence refs, and computed timestamp.
  - Output: Signal evaluator, evaluator-version registry, reason-code mapper, signal replacement links, and `oververify.signals_recomputed` events.
  - Validation: Tests prove repeated recompute with identical inputs is deterministic, live dependency drift cannot mutate prior signals, and signals cite evidence categories rather than opaque global scores.

- **5.2 Publish workload-class-specific eligibility signals.**
  - Design: Produce eligibility records for private tenant, trusted federation, public low-sensitivity, research/public-interest, regulated/secret-bearing, system-service, payout, grant, and review-required consumers.
  - Output: Eligibility evaluator, consumer-specific projections, restriction fields, recheck requirements, expiry fields, and `oververify.eligibility_changed` events.
  - Validation: Integration tests prove Overguard and Oversched consume eligibility changes and that public-provider eligibility cannot satisfy private, regulated, secret-bearing, or system-service work.

- **5.3 Implement evidence decay and stricter-conflict behavior.**
  - Design: Apply policy-versioned freshness windows for heartbeat, command acceptance, benchmark dimensions, challenge families, reliability windows, dispute markers, abuse markers, and anti-Sybil refs.
  - Output: Decay evaluator, freshness windows, class-specific downgrade rules, stricter-conflict resolver, recheck-required flags, and reason-code mapping.
  - Validation: Tests prove stale heartbeat restricts active placement fastest, stale benchmark caps only affected dimensions, recent severe challenge failures override stale passes, and conflicting evidence publishes the stricter workload-class eligibility.

- **5.4 Emit payout-hold recommendation refs within SDS limits.**
  - Design: Recommend payout holds only for trust changes that affect earned value, settlement safety, fraud exposure, or public-provider payout risk, and include evidence refs, reason codes, severity, scope, release conditions, and owner-service handoff.
  - Output: Payout-hold recommendation schema, Provider Payout Service handoff, Overclaim/Seal Ledger/Overbill refs, and no-mutation guardrail tests.
  - Validation: Tests prove stale or inconclusive evidence normally creates scheduler restrictions or recheck signals, while actual hold/release/correction/finality records stay in accounting and dispute services.

- **5.5 Expose signal reads for internal consumers.**
  - Design: Implement authorized internal reads for current provider status, node status, eligibility by target, signal history, certification status, and recompute provenance with consumer service and redaction class declared.
  - Output: Read handlers, consumer-specific redaction, pagination/versioning rules, stale-signal headers, and audit events.
  - Validation: API tests prove unauthorized consumers cannot read restricted evidence, stale/expired signals are labeled, and all reads carry policy/evaluator version, trace id, evidence refs, and redaction class.

## Phase 6: Verification Lifecycle, Certification, Recheck, And Reinstatement

### Work Items

- **6.1 Implement provider and node lifecycle transitions.**
  - Design: Support unverified, pending_evidence, verified, eligible, probation, degraded, recheck_required, suspended, revoked, disputed, and retired states with legal transitions and replacement signal refs.
  - Output: Lifecycle command handlers, state transition validator, state-change events, reason-code mapping, and terminal-state behavior.
  - Validation: State tests reject illegal allowlisting, revocation without evidence refs, reinstatement without correction or fresh evidence, and broad eligibility after a narrow class-specific repair.

- **6.2 Implement certification issue, renewal, suspension, and revocation.**
  - Design: Model provider/node certification type, scope, issuing policy, evidence refs, validity window, renewal rule, revocation refs, public visibility flag, and state-change effects.
  - Output: Certification APIs, certification repository interface, lifecycle events, expiry monitoring, and public/private projection rules.
  - Validation: Tests prove certifications expire, renew only under current evidence, revoke under blocking evidence, and never override stronger workload-class eligibility requirements.

- **6.3 Implement recheck requirement workflows.**
  - Design: Produce recheck requirements for benchmark, challenge, identity, enrollment, heartbeat, software version, public-provider anti-Sybil, payout eligibility, or compliance evidence without running the recheck itself.
  - Output: Recheck requirement schema, source-service handoff refs, scheduler restriction mapping, provider-facing remediation hints, and audit events.
  - Validation: Tests prove recheck requirements block or restrict only affected classes, point to the owning service, and do not call Benchmark Runner or Challenge Task Service as an executor.

- **6.4 Implement signed operator and stewardship actions.**
  - Design: Allow authorized probation, suspension, revocation, certification change, manual review-required, and reinstatement actions with signed operator identity, evidence refs, expiry where applicable, policy version, and Overwatch audit.
  - Output: Signed state-change API, operator-action records, approval rules, expiry behavior, and redacted action explanations.
  - Validation: Authorization tests prove unsigned actions fail, automated service identities cannot widen access beyond policy, and operator-only rationale is redacted from provider-facing views.

- **6.5 Implement correction and appeal recompute paths.**
  - Design: Link accepted Overclaim corrections, appeal outcomes, revoked evidence, superseded evidence, or source-service corrections to targeted signal recomputation without deleting previous evidence.
  - Output: Correction trigger, appeal-aware recompute job, signal replacement links, explanation delta, and audit trail.
  - Validation: Tests prove corrections restore only justified classes, preserve prior evidence history, publish new policy/evaluator refs, and expose actionable appeal status without leaking private evidence.

## Phase 7: Status APIs, Explanations, Redaction, And Client Projections

### Work Items

- **7.1 Implement provider and node status APIs.**
  - Design: Support `GET /verify/providers/{provider_id}` and `GET /verify/nodes/{node_id}` with authorized status, eligibility, certification, recheck, probation, suspension, revocation, and stale-signal projections.
  - Output: Status read handlers, projection contracts, redaction classes, freshness metadata, and read audit events.
  - Validation: API tests prove providers see only their authorized records while operators, stewards, auditors, Overguard, Oversched, and payout consumers receive scoped projections.

- **7.2 Implement eligibility and consumer signal APIs.**
  - Design: Support `GET /verify/eligibility/{target_id}` and bounded history reads that declare consumer service, target scope, desired workload class, and redaction class.
  - Output: Eligibility read handler, consumer-specific response shapes, cache-control/expiry semantics, pagination, and compatibility fixtures.
  - Validation: Integration tests prove Overguard, Oversched, Provider Payout Service, Overgrant, Public Provider Onboarding, Fraud Control Service, and central AI review receive only the fields they are allowed to act on.

- **7.3 Implement provider-facing explanations and remediation hints.**
  - Design: Support explanation bundles with current verification state, affected provider/node/resource/workload classes, user-safe reason codes, policy/evaluator versions, freshness state, metric bands or confidence deltas, redacted evidence refs, remediation steps, and appeal/recheck windows.
  - Output: `GET /verify/{target_id}/explain`, explanation bundle schema, remediation hint catalog, export events, and provider-facing fixtures.
  - Validation: Redaction tests prove provider-facing explanations hide challenge payload selection, randomization strategy, comparator internals, fraud heuristics, other-provider data, private tenant evidence, secret/regulated data, exact topology, raw payout/identity material, operator notes, and sensitive incident details.

- **7.4 Implement operator, steward, auditor, and central AI projections.**
  - Design: Provide deeper authorized views through Overwatch redaction profiles, access-decision refs, audit exports, and stewardship interfaces without creating uncontrolled raw-evidence access.
  - Output: Projection profiles, access decision checks, audit export metadata, central AI summary shape, and traceable dereference rules.
  - Validation: Authorization tests prove privileged views are logged, scoped, redacted by policy, and never bypass Overwatch audit or tenant/provider isolation.

- **7.5 Publish SDK, CLI, admin UI, and native-app contracts.**
  - Design: Generate Rust SDK bindings first and TypeScript/web bindings only for UI/client surfaces for status reads, eligibility reads, explanations, recheck requests, and remediation workflows.
  - Output: Generated contract fixtures, CLI output schema, admin UI projection notes, native-app read models, and compatibility tests.
  - Validation: Client tests prove generated bindings carry stable reason codes, schema versions, trace ids, redaction classes, and no privileged mutation path from UI or TypeScript surfaces.

## Phase 8: Recompute, Replay, Native Persistence, And Operations

### Work Items

- **8.1 Implement idempotent signal recompute jobs.**
  - Design: Support recompute by provider, node, cohort, evidence ref, policy version, evaluator version, or trigger ref with idempotency, bounded scope, replay metadata, and previous/new signal links.
  - Output: `POST /verify/recompute`, recompute worker, job state model, dedupe keys, failure behavior, and recompute events.
  - Validation: Tests prove duplicate recompute requests do not create conflicting signal sets and failed recompute preserves previous signals while marking review or recheck where policy requires.

- **8.2 Implement deterministic replay bundles.**
  - Design: Capture input evidence refs, source metadata, policy refs, evaluator version, old signal refs, new signal refs, reason codes, and redaction metadata so signal decisions can be reproduced.
  - Output: Replay bundle contract, replay command, mismatch record, BLAKE3 commitments, and audit export fixtures.
  - Validation: Replay tests reproduce signals from stored refs and mark mismatches as integrity incidents without deleting or mutating prior signals.

- **8.3 Implement scoped backfill and dry-run recompute.**
  - Design: Allow operators to preview policy/evaluator changes, evidence corrections, freshness-window changes, public-provider rules, and certification updates before activating new signals.
  - Output: Dry-run recompute API, backfill planner, impact summary, redacted diff views, canary fixtures, and rollback notes.
  - Validation: Dry-run tests prove previews are side-effect-free, redacted for non-operators, bounded by target scope, and auditable through Overwatch.

- **8.4 Prepare native persistence and evidence artifact handoff.**
  - Design: Keep early repository abstraction Overrid-shaped and define migration hooks for Overbase records, Overstore evidence artifacts, Overvault private/compliance refs, retention metadata, and backup/restore.
  - Output: Persistence interface, migration contract, evidence artifact ref model, retention tags, restore fixtures, and no-conventional-store guardrails.
  - Validation: Architecture tests prove no PostgreSQL, Redis, S3, MinIO, Vault, cloud KMS, or external workflow product becomes the service boundary, while persistence adapters remain replaceable behind Overrid contracts.

- **8.5 Add observability, alerts, and operator runbooks.**
  - Design: Track evidence freshness, benchmark freshness, challenge outcomes, reliability windows, dispute impact, abuse markers, recompute failures, certification expiry, eligibility distribution, suspension/reinstatement counts, and public-provider anomaly rates.
  - Output: Metrics contract, dashboard definitions, alert rules, health/readiness checks, runbook notes, and incident handoff refs.
  - Validation: Observability tests prove dashboards derive from Overwatch and signal refs, alerts include trace/evidence refs, and broad views do not expose raw private payloads or fraud internals.

## Phase 9: Cross-Service Integrations And Public-Provider Hardening

### Work Items

- **9.1 Integrate Overguard and Oversched consumers.**
  - Design: Provide stable eligibility and trust-signal reads for policy and scheduler decisions, including stale-signal handling, recheck-required restrictions, allowed workload classes, denied workload classes, and reason-coded restrictions.
  - Output: Consumer contract, adapter fixtures, integration tests, cache/expiry semantics, and error mappings.
  - Validation: End-to-end tests prove policy and scheduler decisions change when Oververify evidence changes and fail closed when required signals are missing, expired, or incompatible.

- **9.2 Integrate Benchmark Runner and Challenge Task Service producers.**
  - Design: Accept benchmark and challenge evidence from owning services with clear source authentication, target identity, freshness, anomaly, invalidation, severity, consequence proposal, and replay bundle refs.
  - Output: Producer contract docs, source validation tests, rejection reason codes, and producer-side fixture requirements.
  - Validation: Integration tests prove Oververify ingests results but cannot trigger benchmark execution, challenge selection, duplicate-execution orchestration, or comparator disclosure.

- **9.3 Integrate accounting and payout consumers without mutation.**
  - Design: Publish payout eligibility, payout-hold recommendation, grant eligibility, provider risk, verification class, dispute, and challenge refs to Provider Payout Service, Overgrant, Overmark, Seal Ledger, Overbill, and Overmeter.
  - Output: Accounting handoff schema, read projections, recommendation release conditions, usage-fact refs, and no-mutation tests.
  - Validation: Tests prove accounting services own holds, releases, corrections, settlement, billing, payout, and ledger state while Oververify only emits evidence-backed recommendations and visibility facts.

- **9.4 Harden Phase 11 public-provider verification.**
  - Design: Add public-provider onboarding refs, public sandbox compatibility, anti-Sybil/risk refs, duplicate-execution/fraud-control refs, payout/hold visibility, rate-limit context, abuse markers, challenge history, and strict low-sensitivity caps.
  - Output: Public-provider eligibility matrix, public-risk adapter, volatile TTLs, public explanation redaction, and Phase 11 test fixtures.
  - Validation: Phase 11 tests prove public nodes cannot receive private data, secrets, regulated data, or system-service workloads, and public-provider trust decay cannot widen eligibility.

- **9.5 Integrate client, native-app, and central AI consumers.**
  - Design: Provide provider-facing remediation, admin/operator timelines, wallet/usage center verification views, central AI stewardship summaries, CLI/SDK reads, and native-app projections with stable reason codes.
  - Output: Client projection fixtures, central AI summary contract, admin UI data model, CLI examples, and native-app compatibility notes.
  - Validation: Client integration tests prove user-safe outputs are actionable, appealable, localized by stable codes where needed, and free of operator-only fraud or private evidence details.

## Phase 10: Validation, Documentation, Queue State, And Governance Handoff

### Work Items

- **10.1 Run contract and evidence validation suites.**
  - Design: Validate provider evidence, node evidence, benchmark ingest, challenge ingest, dispute ingest, recompute, eligibility read, explanation, certification, state-change, freshness, supersession, expiry, rejection, and target-identity behavior.
  - Output: Contract test suite, fixture coverage report, negative controls, and validation evidence.
  - Validation: Tests pass for accepted paths and reject fabricated evidence, stale evidence, unsupported sources, target mismatch, missing signatures/hashes, broad public-provider eligibility, and opaque trust-only output.

- **10.2 Run cross-service integration tests.**
  - Design: Validate Overguard consumption, Oversched consumption, Challenge Task Service and Benchmark Runner producer handoffs, Overclaim corrections, payout-hold recommendations, public-provider hardening, and client projections.
  - Output: Integration tests, simulated evidence timelines, expected eligibility diffs, and audit traces.
  - Validation: Tests prove signal changes affect policy/scheduler eligibility, disputes/corrections recompute signals, accounting mutation boundaries hold, and public-provider restrictions remain strict.

- **10.3 Run boundary, redaction, and stack guardrail checks.**
  - Design: Scan for forbidden ownership drift, privileged direct state reads, conventional cloud-product boundaries, blockchain/NFT/pricing/revenue/customer-count assumptions, and provider-facing leakage of fraud, challenge, topology, private tenant, identity, payout, or operator-note details.
  - Output: Boundary scan report, redaction test evidence, stack guardrail evidence, and unresolved-risk list.
  - Validation: Checks pass with no unauthorized benchmark/challenge/scheduler/payout/accounting ownership, no conventional-product boundary drift, and no provider-facing disclosure of operator-only details.

- **10.4 Validate documentation links and queue alignment.**
  - Design: Keep SDS #34, service catalog, master plan, crosswalk, build-plan progress, and Codex55 queue state/progress linked to this sub-build plan with phases numbered 1 through 10.
  - Output: Markdown link validation, phase-heading validation, work-item structure validation, queue JSON validation, queue progress update, and Docdex index refresh evidence.
  - Validation: Local link checks, JSON checks, `git diff --check`, Docdex search, Docdex stats, and queue next-task checks pass.

- **10.5 Prepare governance and implementation handoff.**
  - Design: Document acceptance criteria, open engineering choices, PIP/governance dependencies, incident response hooks, retention/export needs, and phase-gated implementation order for future builders.
  - Output: Handoff checklist, acceptance gates, governance notes, validation evidence summary, and risks requiring Phase 13 review.
  - Validation: Architecture review confirms the implementation plan is internally consistent, aligned with master Phase 0 through Phase 13, aligned with SDS #34 and service catalog boundaries, and ready for downstream phase-work tasks.
