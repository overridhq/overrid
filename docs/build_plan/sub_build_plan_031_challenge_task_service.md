# SUB BUILD PLAN #31 - Challenge Task Service

Attached SDS: [docs/sds/trust_policy_verification/challenge_task_service.md](../sds/trust_policy_verification/challenge_task_service.md)

## Purpose

This sub-build plan turns SDS #31 into an implementation sequence for Challenge Task Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Challenge Task Service is the active verification workload orchestration layer for Overrid. It owns challenge templates, safe challenge manifests, assignments, target snapshots, result normalization, replay bundles, rate limits, evidence refs, and bounded consequence proposals. It does not own final trust scores, scheduling, reservations, execution, payout mutation, ledger state, dispute judgment, private workload inspection, or public-provider eligibility finality.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #31: Challenge Task Service](../sds/trust_policy_verification/challenge_task_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machine, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Challenge Task Service plan](../service_catalog/trust_policy_verification/challenge_task_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, canonical JSON/JSON Schema discipline, signed envelopes, idempotency, trace ids, stable reason codes, local fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies identity, tenant, key, Overgate, Overregistry, Overwatch, Overqueue, service-account, audit, and pending-work primitives consumed by challenge issuance. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies Overcell enrollment, heartbeat, hardware discovery, benchmark evidence, and capability records that become challenge target facts. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack manifests, Overqueue, Oversched, Overlease, Overrun, Overmeter raw usage, retry, cancellation, timeout, and execution refs used by challenge runs. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Controls the first build point for trusted-node challenge orchestration, Overguard admission, Oververify outcome consumption, Overclaim handoff, private mesh/cache trust interactions, and replayable policy evidence. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes challenge usage, hold, refund, correction, and payout-hold proposal refs while keeping ORU, Seal Ledger, Overbill, provider payout, and accounting mutation outside Challenge Task Service. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service operation, dependency readiness, backup/restore, failover, and challenge-service recovery hardening. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Controls public-provider challenge profiles, duplicate execution, stricter cadence, public sandbox challenge classes, fraud controls, payout holds, and low-sensitivity workload limits. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies incident, threat-model, audit-export, compliance, public-report, appeal, migration, and governance hardening for challenge evidence and consequence proposals. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #31 first build work aligned to master Phase 4, with Phase 11 public-provider expansion and later governance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid queue/execution/evidence/accounting boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, or external-payment drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 11, and 13 | Attach SDS #31, freeze Challenge Task Service authority, preserve Phase 4 as first build point, and record public-provider and governance gates. |
| 2 | Master Phases 0, 1, 3, and 4 | Build Rust contracts, JSON Schemas, template/manifest safety rules, state machines, reason codes, and deterministic fixtures before challenge side effects. |
| 3 | Master Phases 2, 3, and 4 | Implement target fact intake, Overguard admission, dry-run, idempotency, cadence, cooldown, and denial/explain records from existing capability and execution facts. |
| 4 | Master Phases 3 and 4 | Execute trusted-node liveness, command-acceptance, capability, benchmark, and controlled-result challenges through normal private execution rails. |
| 5 | Master Phases 4, 5, and 13 | Normalize results, bind Overwatch evidence, create replay bundles, produce consequence proposals, and preserve appealable provider-visible explanations. |
| 6 | Master Phases 4, 5, 11, and 13 | Hand outcome refs to Oververify, scheduler trust consumers, Overclaim, payout-hold consumers, and correction/recheck flows without taking final authority. |
| 7 | Master Phases 4, 7, 11, and 13 | Expand challenge portfolio, anti-gaming randomization, reliability checks, rollout controls, anomaly escalation, and system-service readiness. |
| 8 | Master Phase 11, with Phase 4 and Phase 5 prerequisites | Add public-provider profiles, duplicate execution, stricter cadence, fraud/reputation handoffs, public sandbox profiles, and payout-hold triggers. |
| 9 | Master Phases 4, 5, 7, 11, and 13 | Add dashboards, health checks, operator timelines, outage recovery, evidence exports, and governance-safe reporting. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, tech-stack alignment, queue state, safety controls, integration behavior, and implementation handoff gates. |

## Tech Stack Guardrails

- Challenge Task Service core is a Rust service/module using shared contract types, Tokio for async workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Challenge templates, manifests, target snapshots, assignments, run refs, results, consequence proposals, rate-limit records, public-provider profiles, replay bundles, events, fixtures, API errors, and reason codes use canonical JSON plus JSON Schema for docs-facing and test fixtures. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating APIs require signed service or operator envelopes, tenant/system-service scope, idempotency keys, trace ids, schema versions, stable reason codes, Overguard admission refs, policy refs, evidence refs, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for payload commitments, manifest refs, expected-result commitments, evidence bundle refs, randomization seed commitments, and replay reports.
- Challenge payloads are synthetic, public, or purpose-built. They must never include private tenant workloads, secrets, raw private service memory, regulated data, or secret-bearing endpoint details.
- Challenge Task Service routes execution through native Overrid rails: Overqueue, Oversched, Overlease, Overcell, Overrun, Overpack, Overmeter, Overwatch, Overguard, Oververify, and Overclaim. It must not make Redis, NATS, Kafka, RabbitMQ, PostgreSQL, S3, MinIO, Vault, cloud KMS, Kubernetes-first orchestration, or external scheduler products the platform boundary.
- Oververify owns trust and eligibility changes. Overguard owns admission policy. Oversched and Overlease own placement and reservation. Overcell and Overrun own node command and execution. Overwatch owns evidence retention. Overclaim owns disputes. Provider Payout Service, Seal Ledger, Overbill, and ORU services own accounting effects.
- Provider-visible explanations must disclose stable reason codes, policy/verifier versions, remediation hints, recheck windows, dispute windows, and redacted evidence refs without revealing exact payload selection, randomization strategy, comparator internals, fraud heuristics, other-provider data, tenant-private facts, secrets, or operator-only notes.
- TypeScript is limited to generated bindings, SDK/web clients, and operator/developer UI surfaces. The challenge runtime, policy admission integration, result normalization, replay, and consequence proposal logic stay Rust-first.
- Challenge planning must avoid blockchain, NFT, speculative token mechanics, pricing tables, revenue projections, customer-count assumptions, or per-challenge external payment calls. Cost and holds are represented through ORU, Seal Ledger, Overbill, Provider Payout Service, Overgrant, and payout-hold refs.

## Phase 1: SDS Attachment, Challenge Authority, And Safety Gates

### Work Items

- **1.1 Attach the build plan to SDS #31.**
  - Design: Link this document from the numbered Challenge Task Service SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/trust_policy_verification/challenge_task_service.md`, `docs/service_catalog/trust_policy_verification/challenge_task_service.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #31 returns both the Challenge Task Service SDS and this sub-build plan.

- **1.2 Freeze Challenge Task Service as active challenge orchestration.**
  - Design: Record that the service owns templates, manifests, target snapshots, assignments, run refs, normalized results, replay bundles, rate limits, evidence refs, provider-visible explanations, and bounded consequence proposals.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms the service does not own trust scores, scheduler placement, reservations, execution, payout mutation, ledger state, billing, dispute judgment, fraud scoring, or private workload inspection.

- **1.3 Preserve master Phase 4 as the first build point.**
  - Design: Keep first implementation in Phase 4 because challenges require Phase 2 node/capability evidence and Phase 3 queue/scheduler/lease/runner rails before they can influence trust and eligibility safely.
  - Output: Phase-gate note that Phase 11 expands public-provider challenges, while Phase 5 accounting and Phase 13 governance consume consequence/evidence refs later.
  - Validation: Review proves this plan does not move full challenge behavior into Phases 0 through 3, does not make Phase 11 public-provider policy available early, and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #31 decisions for required private-work challenge refs, classed and risk-driven cadence, high-confidence automatic suspension boundaries, provider-visible redacted verifier details, and reason-based challenge cost ownership.
  - Output: Resolved-decision checklist tied to SDS #31 open-question answers.
  - Validation: Review rejects private eligibility without current challenge refs, a single fixed cadence, irreversible punishment for ambiguous failures, provider explanations that leak challenge internals, and per-challenge external payment mechanics.

- **1.5 Define challenge authority boundaries.**
  - Design: Create a dependency matrix for Oververify, Overguard, Overqueue, Oversched, Overlease, Overcell, Overrun, Overpack, Benchmark Runner, Hardware Discovery, Overwatch, Overclaim, Overmeter, Provider Payout Service, Seal Ledger, Overbill, Fraud Control Service, Reputation and Anti-Sybil Service, SDK, CLI, and admin UI.
  - Output: Boundary matrix listing consumed refs, emitted refs, final authority owner, denial behavior, retry owner, redaction profile, replay evidence, and later phase gate for each dependency.
  - Validation: Review confirms every handoff uses explicit APIs, refs, reason codes, evidence bundles, and policy versions rather than privileged direct state access.

## Phase 2: Rust Contracts, Schemas, Templates, And Fixtures

### Work Items

- **2.1 Create the Challenge Task Service Rust contract module.**
  - Design: Add contract types for challenge templates, challenge manifests, target snapshots, assignments, run refs, results, consequence proposals, rate-limit records, public-provider profiles, replay bundles, state enums, API errors, events, and reason codes.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, stable reason-code catalog, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms challenge contracts remain separate from Oververify trust scoring, Overguard policy, scheduler, runner, dispute, and accounting internals.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for template create/update, issue request, dry-run response, target snapshot, manifest, assignment, run ref, result submission, explain response, cancellation, replay report, rate-limit record, and consequence proposal.
  - Output: Schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing service identity, trace id, idempotency key, target ref, challenge type, policy version, Overguard admission ref, no-private-data attestation, execution refs, evidence refs, and stable reason codes where required.

- **2.3 Define safe challenge template and manifest rules.**
  - Design: Model challenge type, version, risk class, allowed workload classes, required resource class, synthetic or public payload refs, expected result shape, timeout, retry policy, sandbox profile, egress rule, and safety notes.
  - Output: Template schema, manifest schema, no-private-data attestation, allowed-payload registry, sandbox policy refs, and egress-denial fixtures.
  - Validation: Tests prove challenge manifests cannot request tenant secrets, private workload payloads, raw private service memory, unauthorized egress, unsupported resource classes, or public-provider-ineligible workloads.

- **2.4 Define lifecycle, replay, and rate-limit state machines.**
  - Design: Model requested, checking_policy, denied, prepared, queued, scheduled, leased, running, result_pending, passed, failed, inconclusive, consequence_proposed, disputed, closed, and cancelled assignment states plus cooldown and replay states.
  - Output: State transition table, legal transition rules, terminal/overlay state semantics, replay bundle shape, rate-limit bucket states, and event refs.
  - Validation: State tests reject result submission before execution refs, consequence proposals without result refs, closure before handoff refs, retry without idempotency, and history rewriting after dispute or appeal.

- **2.5 Create deterministic fixtures and harness scenarios.**
  - Design: Build fixtures for liveness pass, liveness fail, command acceptance fail, GPU mismatch, stale benchmark, impossible claim, result inconsistency, platform-caused inconclusive, policy denial, cooldown denial, dispute, recheck, and public-provider duplicate execution.
  - Output: Fixture directory, expected API responses, events, Overwatch refs, usage refs, consequence proposals, redacted explain responses, and replay hashes.
  - Validation: Fixture tests produce stable output and prove fail, inconclusive, disputed, and platform-error paths do not mutate trust, payout, ledger, or dispute state directly.

## Phase 3: Policy Admission, Target Facts, And Rate Limits

### Work Items

- **3.1 Implement challenge issue and dry-run APIs.**
  - Design: Support issue and dry-run requests for providers, nodes, verification records, incidents, rechecks, and public-provider onboarding flows using signed service/operator envelopes.
  - Output: `POST /challenges/issue`, `POST /challenges/issue:dry-run`, request validators, idempotent responses, denial responses, and `challenge.issue_requested` or `challenge.issue_denied` events.
  - Validation: API tests reject unauthorized issuers, missing target refs, missing Overguard refs, duplicate conflicting idempotency keys, unsupported challenge types, and public-provider-only challenge classes before Phase 11.

- **3.2 Gather target facts from owning services.**
  - Design: Read provider id, node id, trust class, capability refs, benchmark refs, heartbeat refs, previous challenge refs, public-provider risk level, payout-hold refs, and collected-at timestamp through explicit dependency clients.
  - Output: `challenge_target_snapshot` creation, target fact adapter traits, freshness checks, missing-fact denial reasons, and target snapshot events.
  - Validation: Tests prove missing or stale target facts produce blocked, denied, or recheck outcomes and never cause Challenge Task Service to invent hardware, benchmark, heartbeat, trust, or payout facts.

- **3.3 Integrate Overguard challenge-admission policy.**
  - Design: Submit target facts, challenge class, workload class, sandbox profile, egress profile, rate-limit context, public-provider state, and issuer reason to Overguard before assignment creation.
  - Output: Overguard admission client, policy refs, matched rule refs, deny reason mapping, and audit-safe dry-run response.
  - Validation: Policy tests prove private, regulated, secret-bearing, and system-service challenge classes deny public-provider placement and that denied challenges do not create runnable assignments.

- **3.4 Implement classed cadence and cooldown records.**
  - Design: Rate-limit by target, provider, challenge type, issuer reason, policy version, tenant/system account, public-provider risk level, recent failures, disputes, and incidents.
  - Output: `challenge_rate_limit_record`, cooldown evaluator, exception refs, randomized freshness windows, and cooldown explain fields.
  - Validation: Tests prove repeated challenges are auditable, cooldowns prevent harassment, incident/recheck exceptions are explicit, and public-provider cadence is stricter and less predictable only after Phase 11 gates open.

- **3.5 Implement denial and explanation records.**
  - Design: Store policy denials, missing facts, cooldowns, invalid targets, unsupported challenge classes, public-provider phase gates, and safety denials as replayable records.
  - Output: Denial record, explain response, Overwatch evidence refs, stable reason codes, and redaction policy.
  - Validation: Explain tests prove providers receive appealable high-level reasons without payload selection details, comparator internals, private fraud heuristics, other-provider data, tenant-private facts, secrets, or operator-only notes.

## Phase 4: Trusted-Node Challenge Execution Through Private Rails

### Work Items

- **4.1 Implement liveness and command-acceptance challenges.**
  - Design: Create low-risk synthetic challenge templates that verify a target can receive commands, acknowledge leases, start a sandbox, emit heartbeats, and return controlled evidence.
  - Output: Liveness template, command-acceptance template, assignment creation, queue submission, result expectations, and evidence refs.
  - Validation: Integration tests prove failed liveness or command acceptance can produce high-confidence failure proposals for active targets while platform errors remain inconclusive.

- **4.2 Implement resource-class and GPU capability challenges.**
  - Design: Validate claimed CPU/GPU/memory/storage/runtime class with controlled probes tied to Hardware Discovery and Benchmark Runner refs.
  - Output: Capability challenge templates, expected metric bands, result comparators, hardware ref linkage, and mismatch reason codes.
  - Validation: Tests prove falsified resource class, missing accelerator runtime, impossible GPU claim, or material mismatch creates bounded eligibility downgrade or review proposals without mutating Oververify directly.

- **4.3 Implement benchmark freshness and impossible-claim rechecks.**
  - Design: Trigger recheck challenges when benchmark or heartbeat evidence is stale, impossible, incident-flagged, disputed, or inconsistent with recent execution behavior.
  - Output: Benchmark recheck template, freshness policy refs, impossible-claim anomaly codes, and recheck windows.
  - Validation: Tests prove stale evidence triggers liveness or recheck before stronger consequences, and impossible claims produce stronger proposals only when evidence is high confidence.

- **4.4 Route challenge execution through normal private execution rails.**
  - Design: Package safe challenge manifests and submit them through Overpack, Overqueue, Oversched, Overlease, Overcell, and Overrun with ordinary policy, lease, audit, retry, timeout, and metering behavior.
  - Output: Queue item refs, scheduling request refs, lease refs, Overrun execution refs, timeout/cancel handlers, and usage rollup refs.
  - Validation: Scheduler integration tests prove challenges use normal queue, scheduler, lease, runner, and metering refs and cannot bypass Overguard, Overlease, Overrun sandbox, or Overwatch evidence.

- **4.5 Attach run refs and Overwatch evidence.**
  - Design: Record queue, scheduler, lease, runner, start/end, terminal state, usage, event, log, verifier, and audit refs as a `challenge_run_ref`.
  - Output: Run-ref writer, evidence attachment, event emissions, and replay bundle shell.
  - Validation: Evidence tests prove every queued, scheduled, leased, running, completed, failed, cancelled, timed-out, and inconclusive challenge can be reconstructed from refs without raw private payloads.

## Phase 5: Result Normalization, Evidence, And Consequence Proposals

### Work Items

- **5.1 Implement trusted verifier result submission.**
  - Design: Accept results only from trusted challenge verifier paths with assignment id, execution refs, observed outputs, normalized metrics, expected result hash or comparator, verifier version, confidence, and evidence refs.
  - Output: `POST /challenges/{assignment_id}/result`, verifier authorization, schema validator, idempotent result storage, and `challenge.result_recorded` events.
  - Validation: API tests reject unauthorized verifiers, missing execution refs, malformed outputs, stale verifier versions, conflicting duplicate results, and result submissions for cancelled or closed assignments.

- **5.2 Normalize pass, fail, and inconclusive outcomes.**
  - Design: Map observed outputs to pass, fail, inconclusive, platform error, verifier error, target fault, and policy-blocked states using versioned normalization rules.
  - Output: `challenge_result` records, anomaly codes, confidence levels, terminal state transitions, and normalized result events.
  - Validation: Determinism tests prove the same assignment, execution refs, verifier version, observed facts, and comparator version produce the same outcome and reason codes.

- **5.3 Build replay bundles and evidence commitments.**
  - Design: Bind assignment, target snapshot, manifest, execution refs, result refs, policy version, verifier version, randomization seed commitment, and consequence mapping version.
  - Output: `challenge_replay_bundle`, BLAKE3 replay report hash, replay API, and audit-mode reconstruction output.
  - Validation: Replay tests prove outcome and consequence proposal can be reconstructed from stored refs and that missing evidence yields blocked or inconclusive replay rather than fabricated success or failure.

- **5.4 Create bounded consequence proposals.**
  - Design: Map outcomes to eligibility downgrade, throttle, recheck, manual review, payout-hold proposal, dispute opening, probation, or no-action proposals without mutating owning systems.
  - Output: `consequence_proposal`, severity, proposed action, target refs, recheck window, dispute window, approver requirements, and `challenge.consequence_proposed` event.
  - Validation: Tests prove automatic suspension is reserved for high-confidence safety/integrity/fraud failures and ambiguous, inconclusive, platform-caused, first low-severity reliability, or disputed failures create bounded proposals instead.

- **5.5 Implement redacted provider-visible explanations.**
  - Design: Return challenge family, affected resource/workload class, pass/fail/inconclusive state, stable reason codes, policy/verifier versions, non-sensitive metric bands, remediation steps, recheck windows, dispute windows, and redacted evidence refs.
  - Output: `GET /challenges/{assignment_id}/explain`, provider redaction profile, operator extension profile, and evidence links.
  - Validation: Redaction tests prove explanations are appealable but hide exact payload selection, randomization strategy, comparator internals, private fraud heuristics, other-provider data, tenant-private evidence, secrets, and operator notes.

## Phase 6: Oververify, Scheduler Trust, Disputes, And Accounting Handoffs

### Work Items

- **6.1 Feed outcome refs to Oververify.**
  - Design: Publish outcome refs, confidence, challenge class, policy version, verifier version, and consequence proposals to Oververify for final trust and eligibility decisions.
  - Output: Oververify handoff event, outcome-ref API client, retry behavior, and handoff status on the assignment.
  - Validation: Integration tests prove Oververify can consume challenge outcomes and Challenge Task Service cannot directly set trust score, provider eligibility, scheduler eligibility, or workload-class eligibility.

- **6.2 Emit scheduler trust-signal refs without scheduling authority.**
  - Design: Make challenge outcomes available to Oversched through Oververify-owned trust/eligibility refs and Overwatch evidence rather than direct scheduler mutation.
  - Output: Scheduler-readable handoff refs, eligibility-change correlation ids, stale-trust warnings, and dependency health fields.
  - Validation: Tests prove Oversched placement changes only through owning trust/policy refs and that Challenge Task Service cannot reserve, place, or run work directly.

- **6.3 Add Overclaim dispute and appeal handoff.**
  - Design: Open or update dispute/appeal windows with assignment refs, result refs, explanation refs, evidence refs, consequence refs, and policy-defined freeze behavior.
  - Output: Overclaim handoff client, dispute-window records, challenged-outcome status, and `challenge.disputed` transition support.
  - Validation: Dispute tests prove challenges hand off to Overclaim without rewriting original result records and consequence escalation freezes where policy requires.

- **6.4 Emit payout-hold and correction proposal refs.**
  - Design: Produce hold, correction, refund, or payout review refs for Provider Payout Service, Seal Ledger, Overbill, and Overmeter after Phase 5 accounting rails exist.
  - Output: Accounting handoff record, hold proposal ref, correction trigger ref, issuer/system account cost refs, and retry behavior.
  - Validation: Tests prove Phase 4 builds can stub payout outputs while preserving final semantics and that Challenge Task Service never mutates ORU balances, Seal Ledger entries, invoices, payouts, receipts, pricing, or external payment calls.

- **6.5 Implement recheck and correction-preserving follow-up.**
  - Design: Convert remediation, reinstatement, repeated no-shows, impossible claims, platform-correction, and appeal outcomes into new assignments or correction refs rather than rewriting prior results.
  - Output: Recheck workflow, superseding assignment links, correction record refs, and history view.
  - Validation: History tests prove rechecks, appeals, and corrections append new records with traceable refs and leave old outcomes replayable.

## Phase 7: Reliability, Anti-Gaming, And Challenge Portfolio Expansion

### Work Items

- **7.1 Add controlled result-consistency challenges.**
  - Design: Run deterministic or comparator-backed challenge jobs that detect material output divergence for selected workload classes without using private customer workloads.
  - Output: Result-consistency templates, comparator refs, expected-output commitments, duplicate-safe result records, and anomaly codes.
  - Validation: Tests prove controlled-result inconsistencies can produce trust proposals while private tenant payloads, secrets, and regulated data are never used as challenge material.

- **7.2 Add reliability and repeated controlled-job challenges.**
  - Design: Track repeated controlled jobs, no-shows, timeouts, flaky behavior, degraded runtime, lease violations, and repeated inconclusive outcomes against target risk classes.
  - Output: Reliability templates, rolling challenge history view, severity mapping, and escalation thresholds.
  - Validation: Tests prove first low-severity reliability failures create probation, recheck, or review proposals, while repeated material failures can trigger stricter consequences.

- **7.3 Add auditable anti-gaming randomization.**
  - Design: Randomize time, target, payload, verifier route, duplicate path, and recheck window where policy allows, while storing commitments that can be audited after the fact.
  - Output: Randomization seed commitment, selection report, replay-safe redaction, and anti-gaming reason codes.
  - Validation: Tests prove providers cannot see enough to train against challenges, while operators/auditors can reconstruct the selection from stored refs and commitments.

- **7.4 Implement challenge template rollout controls.**
  - Design: Support staged activation, canary target groups, policy-gated rollout, rollback, version pinning, historical replay, and verifier compatibility checks.
  - Output: Template rollout state, activation events, rollback events, canary reports, and compatibility warnings.
  - Validation: Rollout tests prove a bad template can be halted or rolled back without rewriting previous results and historical snapshots can be replayed against new mapping versions in audit mode.

- **7.5 Add anomaly escalation without final fraud authority.**
  - Design: Emit anomaly clusters, verifier inconsistency, impossible benchmark claims, repeated challenge failures, public-provider anomalies, and abuse patterns to owning fraud/reputation/review services.
  - Output: Fraud/reputation handoff refs, central AI review refs, operator review queue refs, and escalation reason codes.
  - Validation: Tests prove Challenge Task Service emits evidence-backed signals but does not own final fraud score, reputation score, public-provider eligibility, or stewardship decision.

## Phase 8: Public-Provider Phase 11 Expansion

### Work Items

- **8.1 Implement public-provider challenge profiles.**
  - Design: Add onboarding tier, anti-Sybil refs, required challenge cadence, duplicate-execution policy, payout-hold trigger thresholds, maximum public workload class, and public sandbox profile refs.
  - Output: `public_provider_challenge_profile`, profile APIs, Phase 11 gating checks, and public-provider events.
  - Validation: Tests prove public-provider profiles cannot be used to admit private, regulated, tenant-sensitive, secret-bearing, or system-service workloads.

- **8.2 Add stricter public-provider onboarding and freshness challenges.**
  - Design: Require enrollment, resource claim, benchmark freshness, liveness, command acceptance, sandbox smoke, and result-consistency challenge refs before public providers receive low-sensitivity work.
  - Output: Onboarding challenge set, freshness policy, public-provider cooldowns, and eligibility handoff refs.
  - Validation: Tests prove public providers receive stricter cadence, shorter freshness windows, and less predictable timing than founder/private hardware while still respecting anti-harassment cooldowns.

- **8.3 Add duplicate execution and public result-consistency controls.**
  - Design: Route selected public low-sensitivity workloads or challenge-only jobs through duplicate execution and controlled comparators to detect inconsistent or fabricated results.
  - Output: Duplicate challenge templates, result comparison refs, anomaly events, and public-provider consequence proposals.
  - Validation: Integration tests prove duplicate-execution fraud can reduce eligibility or trigger holds while public nodes still cannot receive private, regulated, secret-bearing, or system-service work.

- **8.4 Integrate public sandbox challenge limits.**
  - Design: Require hardened public sandbox profiles with no secret injection, capped runtime, capped resource allocation, deny-by-default egress, output validation, artifact quarantine, and privacy-protected logs.
  - Output: Public sandbox profile refs, challenge manifest constraints, sandbox failure codes, and public-provider explain fields.
  - Validation: Safety tests prove public-provider challenge manifests deny secrets, private data, broad egress, uncapped runtime, privileged filesystem, and sensitive logs.

- **8.5 Integrate fraud, reputation, and payout-hold triggers.**
  - Design: Send public-provider challenge failures, duplicate-execution anomalies, repeated no-shows, impossible claims, abuse reports, and sudden volume changes to fraud, reputation, and payout-hold consumers.
  - Output: Fraud-control handoff, reputation handoff, payout-hold proposal refs, dispute-window refs, and escalation events.
  - Validation: Tests prove public-provider failures can hold payouts and reduce eligibility through owning services while Challenge Task Service stays an evidence/proposal producer.

## Phase 9: Observability, Operations, And Governance Evidence

### Work Items

- **9.1 Build challenge dashboards and metrics.**
  - Design: Track issuance rate, pass/fail/inconclusive ratio, cooldown blocks, execution failures, verifier errors, consequence proposals, dispute rate, public-provider cadence, and template rollout status.
  - Output: Metrics contract, dashboard queries, Overwatch event families, and operator status views.
  - Validation: Observability tests prove metrics are derived from stored challenge/evidence refs and avoid raw private payloads, secrets, or provider-private details in broad views.

- **9.2 Add dependency health and readiness checks.**
  - Design: Check Overguard, Overqueue, Oversched, Overlease, Overrun, Oververify, Overwatch, Benchmark Runner, Hardware Discovery, Overclaim, Overmeter, and verifier freshness.
  - Output: Health endpoint, readiness endpoint, dependency status records, degraded-mode reason codes, and alert triggers.
  - Validation: Health tests prove missing critical dependencies block issuance safely and non-critical downstream handoff failures preserve retryable refs without fabricating success.

- **9.3 Build operator target timelines and evidence views.**
  - Design: Join verification records, challenge history, benchmark evidence, heartbeat evidence, disputes, payout-hold refs, anomaly clusters, and template versions in authorized operator views.
  - Output: Target timeline API, evidence bundle refs, redaction profiles, pagination, and audit events.
  - Validation: Authorization tests prove operator views require explicit role, tenant, data-class, and evidence-purpose checks and provider-visible views remain redacted.

- **9.4 Implement outage recovery and replay operations.**
  - Design: Resume assignments from append-only state, reconcile queue/scheduler/lease/runner refs, retry downstream handoffs, rebuild derived indexes, and quarantine malformed verifier output.
  - Output: Recovery command, reconciliation reports, quarantine records, replay summaries, and recovery events.
  - Validation: Recovery tests prove challenge system outage, partial downstream failure, malformed verifier output, and derived-index loss recover without duplicate consequences or rewritten outcomes.

- **9.5 Add governance, compliance, and public-report evidence paths.**
  - Design: Produce redacted exports, aggregate public reports, incident evidence bundles, threat-model evidence, retention refs, and stewardship review bundles from Overwatch refs.
  - Output: Governance bundle profile, compliance export refs, public-report-safe aggregates, incident handoff refs, and retention metadata.
  - Validation: Governance tests prove exports include provenance and integrity refs while excluding private workload data, secrets, raw fraud heuristics, unrelated provider data, and payment details.

## Phase 10: Validation, Documentation Alignment, And Build Handoff

### Work Items

- **10.1 Run contract and API validation.**
  - Design: Validate template creation/update, issue, dry-run, read, explain, result submission, cancel, replay, target history, rate-limit, and consequence proposal APIs against schemas.
  - Output: Contract tests, API tests, invalid-fixture tests, and compatibility reports.
  - Validation: Tests pass for required fields, stable reason codes, idempotency, trace ids, service identities, Overguard refs, evidence refs, and schema-version compatibility.

- **10.2 Run safety and privacy negative controls.**
  - Design: Prove challenge payloads cannot access tenant secrets, private workload payloads, private storage refs, protected vault refs, regulated data, unsupported egress, or public-provider-ineligible workload classes.
  - Output: Safety test suite, redaction test suite, public-provider denial tests, and no-private-data attestations.
  - Validation: Negative tests fail closed and provider-visible explanations remain useful without exposing sensitive internals.

- **10.3 Run execution-loop integration validation.**
  - Design: Exercise challenges through Overqueue, Oversched, Overlease, Overcell, Overrun, Overmeter, Overwatch, Oververify, Overclaim, and accounting handoff stubs.
  - Output: Integration scenario set, deterministic run refs, replay bundles, and consequence proposal records.
  - Validation: Successful, failed, inconclusive, cancelled, disputed, platform-error, and public-provider-gated scenarios produce distinct states and replayable evidence.

- **10.4 Validate documentation and phase alignment.**
  - Design: Check this plan against SDS #31, the service catalog entry, master Phase 0 through Phase 13 order, service_catalog_alignment, Phase 4, Phase 11, and `docs/overrid_tech_stack_choice.md`.
  - Output: Link-check results, phase-table verification, work-item count verification, stale-note scan, stack-guardrail scan, and queue/progress updates.
  - Validation: Validation proves the plan has 10 phases numbered 1 through 10, five work items per phase, Design/Output/Validation fields, no external product-boundary drift, and no required master phase reordering.

- **10.5 Hand off implementation gates.**
  - Design: Convert this documentation into build-entry criteria for contracts, schemas, local fixtures, Rust service skeleton, policy admission, execution integration, evidence replay, public-provider gating, and governance readiness.
  - Output: Implementation gate checklist, dependency readiness checklist, risk register, and first coding-task candidate list.
  - Validation: Handoff review confirms Phase 4 work can start after Phase 0 through Phase 3 prerequisites exist and Phase 11 public-provider controls remain gated until public low-sensitivity rails are ready.
