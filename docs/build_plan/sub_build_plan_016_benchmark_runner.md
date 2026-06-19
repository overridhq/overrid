# SUB BUILD PLAN #16 - Benchmark Runner

Attached SDS: [docs/sds/execution_scheduling/benchmark_runner.md](../sds/execution_scheduling/benchmark_runner.md)

## Purpose

This sub-build plan turns SDS #16 into an implementation sequence for Benchmark Runner. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Benchmark Runner is the measured-capacity evidence producer for Overrid nodes. It defines approved benchmark suites, coordinates bounded runs through Overcell-managed nodes, ingests signed samples, normalizes results, publishes capability evidence to Overregistry, emits Overwatch events, and preserves enough evidence for Oversched, Oververify, Challenge Task Service, provider reputation, and operators to make later decisions without trusting provider claims.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #16: Benchmark Runner](../sds/execution_scheduling/benchmark_runner.md) | Controls Benchmark Runner purpose, responsibilities, data model, APIs, events, state machine, safety/security rules, metering facts, validation, resolved open-question decisions, and downstream handoff. |
| [Benchmark Runner service plan](../service_catalog/execution_scheduling/benchmark_runner.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schema, fixture, local-stack, idempotency, trace, signed-envelope, and integration-harness prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies identity, tenant, key, registry, queue, audit, and Overwatch primitives that Benchmark Runner depends on but does not own. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Controls the first build point for Benchmark Runner as seed-node capacity evidence, alongside Overcell, Node Installer, Hardware Discovery, and Overregistry capability publication. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Consumes measured capability records for Oversched placement, Overlease reservations, Overrun execution eligibility, and Overmeter raw usage attribution. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Oververify, challenge checks, Overclaim, Overmesh private connectivity, and cache-trust hardening that consume or trigger benchmark evidence. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage rollup and accounting visibility for benchmark overhead without moving billing or settlement into Benchmark Runner. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies system-service workload, backup, restore, failover, upgrade, and grid-resident operation requirements for the benchmark coordinator. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies known-organization and public-interest capacity policies that later consume measured capability evidence. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider anti-gaming, challenge, payout-hold, fraud, sandbox, and reputation hardening that requires stronger benchmark validation. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance, threat modeling, migration, incident, reporting, and governance hardening for benchmark evidence retention and explainability. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #16 first build work aligned to master Phase 2, with later expansion and hardening through execution, verification, accounting, grid-resident, federation, public-provider, and governance phases. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP, signed envelopes, Ed25519, BLAKE3/content hashes, canonical JSON plus JSON Schema, explicit GPU/runtime adapters, Rust Overcell agent coordination, and native Overrid boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, and 2 | Attach SDS #16, freeze Benchmark Runner as measured-capacity evidence, and preserve first build in Phase 2. |
| 2 | Master Phases 0 and 2 | Build Rust service skeleton, suite schemas, local fixtures, and deterministic benchmark contract tests. |
| 3 | Master Phase 2 | Implement suite registry, approved runner artifact contracts, safety limits, compatibility checks, and suite version immutability. |
| 4 | Master Phase 2 | Implement run lifecycle, Overcell assignment, signed node sample uploads, and coordinator completion. |
| 5 | Master Phases 2 and 3 | Normalize samples into capability tiers, confidence, expiry, and Overregistry publication records consumed by Oversched. |
| 6 | Master Phases 2 and 4 | Add safety enforcement, anomaly detection, invalidation, Oververify rerun hooks, and challenge-triggered supersession. |
| 7 | Master Phases 3 and 5 | Wire scheduler-facing eligibility summaries, resource-class validity windows, and system-service usage facts without owning scheduling or accounting. |
| 8 | Master Phases 4, 7, and 13 | Add operations, observability, retention, restricted evidence views, rebuild/replay, backup/restore, and grid-resident readiness. |
| 9 | Master Phases 6, 10, 11, and 12 | Harden SDK, CLI, admin, adapter, federation, public-provider, native-app, and product-reader handoffs. |
| 10 | Master Phase 2 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, and final implementation gates. |

## Tech Stack Guardrails

- Benchmark Runner core is a Rust execution/scheduling service using Tokio and Axum/Tower/Hyper-style HTTP, with rustls/mTLS where early seed control-plane or node-agent paths require it.
- Benchmark suite manifests, run records, environment snapshots, samples, normalized results, evidence bundles, anomalies, invalidations, API errors, and fixtures use canonical JSON plus JSON Schema. Compact Protobuf contracts may be added only where the shared contract layer requires them.
- Mutating commands require signed command or service-account envelopes, idempotency keys, trace ids, tenant or system scope, actor/service refs, stable reason codes, schema versions, policy/evidence refs, and append-only Overwatch events.
- Node sample uploads require Overcell assignment binding, node credential verification through Overkey, suite hash binding, environment snapshot binding, and coordinator verification before normalization.
- Ed25519 is used where node, coordinator, service-account, or operator signatures are required. BLAKE3/content hashes are used for suite artifacts, sample bundles, normalized result records, evidence bundles, and Overregistry publication refs.
- GPU/runtime integration is explicit and adapter-controlled for NVIDIA/ROCm or future accelerator runtimes. Benchmark Runner must not hide arbitrary shell execution behind a benchmark label.
- Benchmark storage and queues use Overrid-owned boundaries or Overrid-shaped local stubs during early phases. PostgreSQL, Redis, Kafka, NATS, RabbitMQ, S3, MinIO, Vault, cloud KMS, blockchain, NFT, or market-token mechanics must not become Benchmark Runner's product boundary.
- Benchmark Runner does not own Hardware Discovery inventory, scheduling placement, trust scoring, policy decisions, provider reputation, disputes, ORU balances, Seal Ledger transitions, billing, payout eligibility, or public-provider admission decisions.
- Benchmark jobs are bounded system-service workloads with declared CPU, GPU memory, disk write, network target, duration, and cooldown limits. Seed hardware protection wins over benchmark completeness.
- Raw host details remain restricted evidence. Scheduler-facing and tenant-visible reads expose normalized resource class, capability tier, confidence, freshness, redacted reason codes, and eligibility summaries only.

## Phase 1: SDS Attachment, Evidence Scope, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #16.**
  - Design: Link this document from the numbered Benchmark Runner SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/execution_scheduling/benchmark_runner.md`, `docs/service_catalog/execution_scheduling/benchmark_runner.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #16 returns both the Benchmark Runner SDS and this sub-build plan.

- **1.2 Freeze Benchmark Runner as measured-capacity evidence.**
  - Design: Record that Benchmark Runner owns suite definitions, run lifecycle, signed sample ingestion, normalization, anomaly records, invalidation records, evidence bundles, and Overregistry publication refs.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Benchmark Runner does not own Hardware Discovery inventory, scheduler private state, trust scoring, policy finality, dispute judgment, ORU accounting, billing, or provider payout state.

- **1.3 Preserve master Phase 2 as the first build point.**
  - Design: Keep first implementation in master Phase 2 because seed nodes need measured CPU, GPU, disk, network, cold-start, and sustained-safety evidence before Phase 3 private workload placement.
  - Output: Phase-gate note that master Phase 0 and Phase 1 are prerequisites, Phase 2 is first implementation, and later phases consume or harden benchmark evidence.
  - Validation: Review proves this plan does not move Benchmark Runner into Phase 0 or Phase 1 and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #16 decisions for resource-class-specific eligibility, validity windows, restricted host details, confidence thresholds, and anomaly severity eligibility effects.
  - Output: Resolved-decision checklist tied to SDS #16 open-question answers.
  - Validation: Review rejects plans that allow stale, missing, or unknown benchmark confidence to pass scheduling, expose raw host details in general reads, or treat one benchmark result as permanent truth.

- **1.5 Define runtime authority boundaries.**
  - Design: Create a boundary matrix for Overcell, Hardware Discovery, Overregistry, Overwatch, Oversched, Oververify, Challenge Task Service, Overmeter, Overguard, Overclaim, provider reputation, admin UI, SDK, and CLI interactions.
  - Output: Boundary matrix listing read/write authority, allowed events, restricted evidence, and ownership exclusions.
  - Validation: Design review rejects direct scheduler state writes, arbitrary customer code execution, benchmark-labeled production work, unrestricted raw sample reads, and billing or payout decisions inside Benchmark Runner.

## Phase 2: Rust Service Skeleton, Schemas, And Fixtures

### Work Items

- **2.1 Create the Benchmark Runner Rust service crate.**
  - Design: Add a Rust service module using Tokio, Axum/Tower/Hyper-style HTTP, shared config loading, tracing setup, dependency injection, and clients for Overcell, Overkey, Overregistry, Overwatch, Overmeter, and Oververify.
  - Output: Service crate, handler modules, repository traits, local-stack entrypoint, background worker boundary, and integration-test hooks.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Benchmark Runner remains separate from Overcell, Hardware Discovery, Overregistry, Oversched, Oververify, and accounting services.

- **2.2 Define benchmark contract schemas.**
  - Design: Add schemas for `benchmark_suite`, `benchmark_run`, `benchmark_environment_snapshot`, `benchmark_sample`, `benchmark_result`, `benchmark_evidence_bundle`, `benchmark_anomaly`, `benchmark_invalidation`, API errors, and benchmark events.
  - Output: JSON Schema files, Rust types, fixtures, stable reason-code enums, and compatibility metadata.
  - Validation: Schema tests reject missing suite id/version, node id, provider id, trace id, idempotency key, environment snapshot refs, sample hashes, result hash, expiry, confidence, signatures, and reason codes where required.

- **2.3 Define seed-node benchmark fixtures.**
  - Design: Model CPU-only, GPU-capable, missing-driver, throttled, slow-disk, unstable-network, offline-node, stale-agent, and safety-cancelled fixtures.
  - Output: Valid and invalid run fixtures with deterministic sample values, environment snapshots, normalized results, anomalies, and expected publication outcomes.
  - Validation: Fixture tests prove the same raw samples, suite version, and environment snapshot reconstruct the same normalized result and evidence hash.

- **2.4 Implement Overrid-owned storage boundaries.**
  - Design: Define repositories for suite versions, run state, raw sample refs, normalized results, evidence bundle refs, anomaly records, invalidations, derived indexes, and local Overrid-shaped storage stubs.
  - Output: Repository traits, local adapter, append-only evidence model, derived-index rebuild model, and migration hooks.
  - Validation: Storage tests prove published results cannot be silently edited, supersession and invalidation append new records, and restricted host details are stored only as protected refs.

- **2.5 Wire local development and harness scenarios.**
  - Design: Add local-stack and integration-harness scenarios for one CPU node, one GPU node, assignment acceptance, sample upload, normalization, Overregistry publication, Overwatch events, and Oververify rerun request.
  - Output: Local service config, deterministic fixtures, fake Overcell node-agent channel, fake Overregistry/Overwatch clients, and scenario names.
  - Validation: Local smoke tests can register suites, request a run, receive signed samples, normalize a result, publish capability evidence, and inspect redacted latest results without production credentials.

## Phase 3: Suite Registry, Runner Artifacts, And Safety Limits

### Work Items

- **3.1 Implement suite registration.**
  - Design: Support immutable suite registration with suite id, semantic version, resource class, workload class, runner artifact ref, expected output schema, safety limits, timeout, sampling policy, and deprecation state.
  - Output: `POST /benchmark-suites`, `GET /benchmark-suites`, suite hash calculation, compatibility filters, and suite lifecycle events.
  - Validation: API tests cover valid registration, duplicate version, mutable change rejection, missing safety limits, missing output schema, unknown resource class, and deprecated suite filtering.

- **3.2 Define approved runner artifact contracts.**
  - Design: Require runner artifact refs, content hashes, platform/runtime compatibility, explicit command args, output schema, resource cap declarations, and no arbitrary customer payload.
  - Output: Runner artifact contract, suite artifact verification, local artifact fixtures, and artifact-ref handoff to Overstore or early Overrid-shaped stubs.
  - Validation: Tests reject mutable runner artifacts, missing hashes, broad shell execution, unbounded output, undeclared network targets, and output that cannot validate against the suite schema.

- **3.3 Implement compatibility and suite selection.**
  - Design: Select the smallest compatible suite based on Hardware Discovery prerequisites, node class, driver/runtime facts, trigger type, resource class, current cooldown, and existing valid evidence.
  - Output: Suite compatibility evaluator, selection reason codes, missing-prerequisite reasons, and dry-run selection result.
  - Validation: Fixture tests prove CPU, GPU, disk, private-network, cold-start, and sustained-load suites are selected only when prerequisites and safety windows allow them.

- **3.4 Implement safety policy enforcement.**
  - Design: Enforce maximum CPU load, GPU memory, disk writes, network targets, duration, concurrent runs, cooldowns, and seed-hardware protection before assignment.
  - Output: Safety policy validator, cap schemas, cooldown tracker, assignment blocker reasons, and `benchmark_runner.run_rejected` event payloads.
  - Validation: Safety tests prove the runner rejects overbroad suites, production-looking workloads, unsafe network targets, excessive disk writes, excessive GPU memory pressure, and cooldown violations.

- **3.5 Implement suite deprecation and migration rules.**
  - Design: Allow suite versions to be deprecated, replaced, or blocked while retaining historical results with their original suite version and hash.
  - Output: Deprecation state model, replacement refs, suite migration report, and compatibility warning events.
  - Validation: Tests prove deprecated suites are not selected for new runs unless explicitly authorized, historical results stay explainable, and replacement suites trigger rerun planning instead of rewriting old evidence.

## Phase 4: Run Lifecycle, Overcell Assignment, And Signed Samples

### Work Items

- **4.1 Implement benchmark run requests.**
  - Design: Support `POST /benchmark-runs` with node id, suite id/version or resource-class request, trigger type, requester, idempotency key, trace id, policy/evidence refs, and bounded batch behavior.
  - Output: Run request handler, planned/queued state, idempotent responses, rate limits, and `benchmark_runner.run_requested` events.
  - Validation: API tests cover valid request, duplicate idempotency key, unsupported suite, missing node, invalid trigger, unauthorized requester, unbounded batch, and blocked resource class.

- **4.2 Implement Overcell assignment.**
  - Design: Send signed benchmark assignments to Overcell with suite version, suite hash, runner artifact ref, safety limits, timeout, cooldown, output schema, trace id, and node-specific assignment binding.
  - Output: Assignment client, assignment state transition, run-assigned event, retry rules, and assignment failure reasons.
  - Validation: Integration tests prove a run moves from queued to assigned only after Overcell accepts the signed assignment and rejects mismatched suite hash, expired assignment, wrong node id, or unsupported runtime.

- **4.3 Implement signed sample upload.**
  - Design: Support internal `POST /benchmark-runs/{run_id}/samples` for node-originated signed samples bound to assigned run id, suite hash, environment snapshot, sample index, metric name, unit, value, and capture timestamp.
  - Output: Sample upload handler, node signature verification, sample range validation, sample hash storage, and `benchmark_runner.sample_received` events.
  - Validation: Signature tests reject unsigned samples, wrong node credential, replayed samples, out-of-order required samples, mismatched suite hash, invalid units, impossible values, and samples for unassigned runs.

- **4.4 Implement environment snapshot binding.**
  - Design: Capture discovery snapshot refs, agent version, OS/kernel, driver/runtime versions, thermal/power hints, sandbox/container info, network peer set, locality tag, and restricted host-detail refs.
  - Output: Environment snapshot schema, snapshot validator, redacted read model, restricted evidence refs, and snapshot consistency checks.
  - Validation: Tests prove driver/runtime changes, agent upgrades, material Hardware Discovery deltas, prolonged offline intervals, and safety cancellations force rerun or block affected resource classes.

- **4.5 Implement run completion and terminal states.**
  - Design: Support completion, failure, cancellation, timeout, sampled, normalizing, published, superseded, invalidated, and failed states with append-only transitions and stable reason codes.
  - Output: Completion handler, state machine validator, terminal-state events, retry eligibility rules, and failure records.
  - Validation: State tests prove illegal transitions are rejected, published results are superseded or invalidated rather than edited, and every terminal state has reason, trace, and evidence refs.

## Phase 5: Normalization, Confidence, And Overregistry Publication

### Work Items

- **5.1 Implement unit normalization and result calculation.**
  - Design: Convert raw samples into normalized score, unit, resource tier, confidence, stability marker, expiry timestamp, capability delta, and scheduler-facing summary.
  - Output: Normalization engine, unit conversion table, result schema, and deterministic fixture cases.
  - Validation: Replay tests prove normalized results can be reconstructed from suite version, environment snapshot, raw samples, and normalization version.

- **5.2 Implement confidence and validity windows.**
  - Design: Apply SDS #16 confidence thresholds and validity windows for baseline CPU, disk, private-network, cold-start, GPU/runtime, sustained thermal, private-test, private-standard, resource-intensive, and system-service-candidate eligibility.
  - Output: Confidence calculator, expiry policy, blocked/missing/expired state markers, and resource-class eligibility summaries.
  - Validation: Tests prove Phase 3 placement blocks missing, expired, unknown, or below-threshold results and only permits lower classes when baseline suites and Oververify eligibility pass.

- **5.3 Implement anomaly detection.**
  - Design: Detect impossible capability, degraded performance, instability, safety-limit bypass, signature mismatch, tampered evidence, repeated runner crashes, and identity-binding suspicion.
  - Output: Anomaly rules, severity mapping, affected resource class, required follow-up, and `benchmark_runner.anomaly_detected` events.
  - Validation: Tests prove critical anomalies suspend all new placement, high/suspicious anomalies suspend affected resource class, degraded anomalies downgrade tier, warnings shorten validity, and info anomalies remain trend-only.

- **5.4 Publish capability evidence to Overregistry.**
  - Design: Publish benchmark evidence bundles, normalized result refs, capability deltas, expiry, confidence, suite hash, sample hashes, and environment refs to Overregistry without overwriting historical evidence.
  - Output: Overregistry publication client, publication state, stale capability report, and `benchmark_runner.evidence_published` events.
  - Validation: Integration tests prove Overregistry receives versioned evidence refs, keeps historical results, distinguishes active/latest from superseded/invalidated results, and exposes only scheduler-safe summaries.

- **5.5 Implement latest-result reads.**
  - Design: Support `GET /nodes/{node_id}/benchmark-results` and `GET /benchmark-runs/{run_id}` with public summary, operator detail, restricted evidence, and tenant visibility filtering.
  - Output: Result read handlers, field-policy matrix, redaction fixtures, pagination, and stable reason-code mapping.
  - Validation: Privacy tests prove general reads hide serials, MACs, IPs, hostnames, rack/facility details, GPU UUIDs/serials, raw network peers, route tables, mount paths, local usernames, raw logs, and identifying thermal/power details.

## Phase 6: Safety, Invalidation, And Verification Hooks

### Work Items

- **6.1 Implement invalidation and supersession.**
  - Design: Support `POST /benchmark-runs/{run_id}/invalidate` with reason code, actor/service account, evidence refs, replacement run id, scope, and affected resource class.
  - Output: Invalidation handler, supersession records, latest-result recomputation, and `benchmark_runner.result_invalidated` events.
  - Validation: Tests prove invalidated results are not scheduler-eligible, superseded results remain historically explainable, and unauthorized invalidation is rejected.

- **6.2 Add rerun triggers.**
  - Design: Schedule reruns for initial enrollment, periodic refresh, agent upgrade, driver/runtime change, material inventory change, warning/degraded anomaly, offline interval, safety cancellation, suspicious result, operator request, and Oververify challenge.
  - Output: Rerun scheduler, trigger reason codes, cooldown-aware queueing, and rerun request events.
  - Validation: Tests prove volatile resources rerun sooner than baseline resources and challenge-triggered reruns supersede or invalidate previous results according to outcome.

- **6.3 Implement Oververify challenge hooks.**
  - Design: Let Oververify and Challenge Task Service request validation reruns, inspect anomalies, receive evidence refs, and mark resource-class eligibility outcomes without Benchmark Runner owning trust score.
  - Output: Challenge request API/client, challenge evidence handoff, response reason codes, and challenge-run linkage.
  - Validation: Integration tests prove Oververify can trigger reruns, consume evidence, and change eligibility through its own authority while Benchmark Runner remains evidence producer only.

- **6.4 Implement safety cancellation and cooldown behavior.**
  - Design: Cancel runs when local safety reports exceed caps, node availability would be harmed, network targets are unsafe, GPU memory pressure is excessive, disk write caps are reached, or sustained-load windows breach policy.
  - Output: Safety cancellation states, cooldown records, anomaly events, and operator-visible reason codes.
  - Validation: Safety tests prove cancellations produce evidence and block immediate retry where required, while unrelated resource classes remain eligible when their baseline evidence is valid.

- **6.5 Implement restricted evidence bundles.**
  - Design: Build evidence bundles that link claimed inventory, environment snapshots, samples, normalized results, signatures, suite hashes, Overwatch events, Overregistry refs, anomalies, invalidations, and challenge refs.
  - Output: Bundle manifest schema, content hash, node signature refs, coordinator signature refs, redaction rules, and restricted reader policy.
  - Validation: Bundle tests prove raw host details stay restricted, public summaries are redacted, and Oververify/operator/dispute paths can prove evidence integrity without exposing unnecessary private host data.

## Phase 7: Scheduler, Usage, And Accounting Handoffs

### Work Items

- **7.1 Define scheduler-facing result summaries.**
  - Design: Expose measured resource class, tier, confidence, expiry, anomaly state, validity reason, freshness, and blocked/downgraded class markers through Overregistry, not direct Benchmark Runner private tables.
  - Output: Scheduler summary schema, Overregistry read contract, and blocked eligibility reason map.
  - Validation: Oversched contract tests prove placement uses Overregistry-published benchmark evidence and blocks missing, expired, below-threshold, challenged, or critical-anomaly results.

- **7.2 Connect resource cards and Overmark signals.**
  - Design: Map measured capability tiers to resource cards and bounded reference-rate inputs without encoding prices, customer counts, revenue assumptions, or speculative market behavior.
  - Output: Resource-card mapping contract, Overmark handoff refs, and resource-class capability fixtures.
  - Validation: Review confirms Benchmark Runner provides measured capability facts only and does not calculate pricing, provider earnings, market rates, ORU transitions, or payouts.

- **7.3 Emit benchmark usage facts.**
  - Design: Emit system-service usage events for benchmark duration, CPU/GPU time, disk writes, network bytes, object/evidence storage, suite id/version, trigger type, requester, node id, and provider id.
  - Output: Overmeter raw usage event contract, usage event client, and benchmark-overhead dashboards.
  - Validation: Metering tests prove benchmark overhead is visible to operators and future accounting while no per-event charge, settlement, billing, or payout logic lives inside Benchmark Runner.

- **7.4 Support private-workload eligibility gates.**
  - Design: Enforce SDS #16 resource-class eligibility rules for Phase 3 private test, private standard, GPU/storage-heavy, and future system-service-candidate placement through published evidence.
  - Output: Eligibility gate contract, reason-code mapping, and resource-class fixture matrix.
  - Validation: Tests prove nodes lacking class-specific suites remain eligible only for lower classes when baseline suites, heartbeat, node state, and Oververify eligibility all pass.

- **7.5 Add failure-state handoffs to execution and accounting.**
  - Design: Surface benchmark stale, invalidated, challenged, degraded, and blocked states to Overqueue/Oversched/Overlease/Overrun/Overmeter consumers without changing their domain state directly.
  - Output: Handoff events, consumer contract docs, and failure-reason fixture set.
  - Validation: End-to-end tests prove failed, cancelled, timed-out, challenged, invalidated, and superseded benchmark states produce distinct scheduling and metering effects.

## Phase 8: Operations, Retention, Recovery, And Grid-Resident Readiness

### Work Items

- **8.1 Implement operational health and metrics.**
  - Design: Expose suite registry health, active/queued run counts, node assignment lag, sample rejection rate, normalization errors, publication lag, signer availability, anomaly counts, cooldowns, and latest-result freshness.
  - Output: Rust tracing/OpenTelemetry-compatible metrics, health/readiness endpoints, dashboard query shapes, and Overwatch health events.
  - Validation: Metrics tests prove labels avoid raw host details, tenant leakage, secrets, raw hashes where not public, payment details, and unbounded high-cardinality values.

- **8.2 Implement operator controls.**
  - Design: Provide safe controls to pause scheduling, disable a suite, rerun a node, invalidate a result, inspect failure reasons, inspect queue pressure, and view restricted evidence according to role and purpose.
  - Output: Admin API contract, CLI/admin UI requirements, reason-code docs, and runbook links.
  - Validation: Operator tests prove privileged controls require explicit role, tenant/system scope, data-class, evidence-purpose, idempotency, audit, and service-account checks.

- **8.3 Implement retention and restricted evidence policy.**
  - Design: Retain raw samples long enough for disputes and challenge windows while preserving compact normalized results, content hashes, signatures, invalidations, and summary history for scheduling and governance.
  - Output: Retention policy records, archive refs, redaction marker behavior, and restricted evidence access matrix.
  - Validation: Retention tests prove raw host details are not leaked through general reads and historical evidence remains explainable after schema, suite, or environment upgrades.

- **8.4 Implement replay, rebuild, and migration support.**
  - Design: Rebuild latest-result indexes, anomaly summaries, scheduler summaries, and publication refs from append-only run, sample, result, invalidation, and Overregistry evidence records.
  - Output: Rebuild command, migration hooks, replay reports, corruption reports, and verification summaries.
  - Validation: Recovery tests destroy derived indexes in fixtures, rebuild them from source records, and verify result, anomaly, publication, invalidation, and latest-summary equivalence.

- **8.5 Prepare grid-resident operations.**
  - Design: Define Benchmark Runner as a system-service workload with protected placement, backup, restore, failover, rolling update, rollback, maintenance mode, break-glass controls, signer handling, and incident runbooks.
  - Output: Phase 7 readiness checklist, system-service manifest requirements, backup/restore fields, and failover evidence refs.
  - Validation: Grid-readiness review confirms founder seed hardware can later be removed from the normal path without changing suite, run, sample, result, evidence, or publication contracts.

## Phase 9: Product, Federation, Public Provider, And Client Handoffs

### Work Items

- **9.1 Harden SDK and CLI benchmark flows.**
  - Design: Provide generated Rust-first SDK and CLI flows for suite listing, run requests, run inspection, latest result reads, rerun requests, invalidation diagnostics, and redacted evidence summaries.
  - Output: SDK/CLI contract examples, stable JSON output, pagination, reason-code mappings, and troubleshooting flows.
  - Validation: SDK/CLI tests prove clients pass trace ids and idempotency keys, decode reason codes, respect tenant filters, and cannot invoke privileged sample upload or invalidation without authority.

- **9.2 Implement admin and operator visibility.**
  - Design: Expose dashboards for suite version coverage, active/queued runs, per-node last valid result, anomalies, invalidations, failed uploads, publication lag, cooldowns, challenge reruns, and safety cancellations.
  - Output: Admin read-model requirements, UI endpoint contracts, restricted evidence policy, and operator workflow checklist.
  - Validation: Admin tests prove operators can diagnose benchmark evidence while tenants cannot see cross-tenant private host metadata or raw restricted evidence.

- **9.3 Define product and adapter handoffs.**
  - Design: Document how Docdex, Mcoda, Codali, AI gateway, encrypted RAG, runtime bridge, node agents, and product workloads consume scheduler-safe capability evidence through Overregistry.
  - Output: Product capability evidence checklist, adapter fixture contracts, and integration scenarios.
  - Validation: Product integration tests fail when consumers bypass Overregistry, treat Hardware Discovery claims as measured results, ignore expiry/confidence/anomaly state, or depend on Benchmark Runner private tables.

- **9.4 Add trusted federation and public-interest capacity rules.**
  - Design: Support known-organization and public-interest pools by preserving suite provenance, purpose scope, grant refs, provider refs, and evidence freshness for donated or shared capacity.
  - Output: Federation/public-interest evidence checklist, grant-aware result summaries, and reporting refs.
  - Validation: Federation tests prove shared capacity remains policy-bound, evidence-backed, purpose-scoped, and explainable without exposing raw provider host details.

- **9.5 Harden public-provider anti-gaming controls.**
  - Design: Add stronger public-provider controls for random challenge timing, repeated-run consistency, impossible-result detection, identity-binding suspicion, payout-hold refs, fraud refs, sandbox class limits, and low-sensitivity placement only.
  - Output: Public-provider benchmark hardening checklist, anti-gaming rule set, and challenge/fraud/payout-hold handoff refs.
  - Validation: Public-provider tests prove unknown nodes cannot become eligible for private, regulated, secret-bearing, or system-service workloads and suspicious benchmark behavior reduces eligibility before payout-sensitive flows.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #16`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first implementation, Tokio, Axum/Tower/Hyper-style HTTP, Overcell coordination, explicit GPU/runtime adapters, signed envelopes, Ed25519, BLAKE3/content hashes, canonical JSON plus JSON Schema, native Overrid boundaries, and restricted evidence handling.
  - Output: Tech-stack alignment checklist for Benchmark Runner.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #16 is represented as a Phase 2 execution/scheduling service with later handoffs through private execution, trust/verification, accounting visibility, grid-resident operation, federation/public-provider hardening, product clients, and governance.
  - Output: Updated master-plan and crosswalk rows for SDS #16.
  - Validation: Review confirms only per-SDS sub-build indexing changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #16 and the Benchmark Runner service plan link back to this sub-build plan and preserve Benchmark Runner as the measured-capacity evidence producer.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare Benchmark Runner implementation gates.**
  - Design: Require tests for suite registration, suite compatibility, safety caps, runner artifact verification, run request, Overcell assignment, signed sample upload, environment snapshot binding, normalization, confidence, expiry, anomaly severity, Overregistry publication, latest result reads, invalidation, rerun triggers, Oververify challenge hooks, evidence bundles, scheduler summaries, usage events, retention, recovery, operator controls, SDK/CLI/admin/product/federation/public-provider handoffs, and documentation links.
  - Output: Final validation checklist for Benchmark Runner implementation.
  - Validation: Handoff review confirms Overcell, Hardware Discovery, Overregistry, Oversched, Overlease, Overrun, Overmeter, Overwatch, Oververify, Challenge Task Service, Overguard, Overclaim, Overmark, provider reputation, SDK, CLI, admin UI, adapters, federation services, public-provider services, native apps, and governance services can depend on Benchmark Runner evidence without moving their runtime authority into Benchmark Runner.

## Alignment Review

- The sub-build plan keeps Benchmark Runner first build work in master Phase 2, matching SDS #16, the service catalog entry, Phase 2 plan, master build plan, and build-plan crosswalk.
- The plan treats master Phase 0 and Phase 1 as prerequisites for shared schemas, local harnesses, identity, tenant, key, registry, queue, audit, and Overwatch primitives, not as the Benchmark Runner implementation phase.
- The plan treats later phases as consumer or hardening gates: scheduler eligibility in Phase 3, verification/challenge and private mesh hardening in Phase 4, benchmark overhead metering in Phase 5, grid-resident operation in Phase 7, trusted federation/public-interest consumption in Phase 10, public-provider anti-gaming in Phase 11, client/product reads in Phases 6 and 12, and compliance/governance hardening in Phase 13.
- The plan carries forward SDS #16 resolved decisions for resource-class-specific eligibility, validity windows, restricted raw host details, benchmark confidence thresholds, and anomaly severity effects.
- The plan keeps Benchmark Runner narrow: no Hardware Discovery inventory ownership, no scheduler placement ownership, no policy finality, no trust score ownership, no dispute adjudication, no reputation scoring, no ORU or Seal Ledger mutation, no billing or payout ownership, no arbitrary-code execution, and no public-provider admission authority.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #16 is complete when a builder can implement Benchmark Runner as the Phase 2 Rust measured-capacity evidence producer with immutable suite manifests, approved runner artifact contracts, suite compatibility checks, safety caps, run lifecycle APIs, Overcell assignment, signed node sample upload, environment snapshot binding, deterministic normalization, confidence and validity windows, anomaly severity effects, Overregistry capability publication, redacted latest-result reads, invalidation and supersession, rerun triggers, Oververify challenge hooks, restricted evidence bundles, scheduler-facing summaries, benchmark overhead usage facts, retention and recovery behavior, operator controls, SDK/CLI/admin/product/federation/public-provider handoffs, implementation validation gates, and documentation links that preserve the master Phase 0 through Phase 13 order.
