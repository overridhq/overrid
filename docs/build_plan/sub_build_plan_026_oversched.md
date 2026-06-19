# SUB BUILD PLAN #26 - Oversched

Attached SDS: [docs/sds/execution_scheduling/oversched.md](../sds/execution_scheduling/oversched.md)

## Purpose

This sub-build plan turns SDS #26 into an implementation sequence for Oversched. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Oversched is the policy-aware placement decision authority for the Phase 3 private execution loop. It claims accepted queue items, assembles traceable fact bundles, filters candidate nodes, produces stable reason codes, scores eligible candidates, writes replayable placement decisions, and requests reservations from Overlease. It does not execute workloads, create leases, mint trust or policy facts, mutate accounting balances, own cache/storage facts, or expose provider-private capacity details in user explanations.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #26: Oversched](../sds/execution_scheduling/oversched.md) | Controls Oversched purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machine, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Oversched service plan](../service_catalog/execution_scheduling/oversched.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, canonical JSON/JSON Schema discipline, signed envelopes, idempotency, trace ids, stable reason codes, local fixtures, deterministic harnesses, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, identity, tenant, key, registry, queue, command, and audit primitives that Oversched consumes. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies Overcell nodes, node identity, heartbeat/load state, Hardware Discovery facts, Benchmark Runner capacity evidence, and capability publication prerequisites. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Controls Oversched's first build point: deterministic private-swarm placement, candidate snapshots, reason codes, scoring, Overlease reservation requests, queue/Overrun handoff, and replayable placement decisions. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard, Oververify, Overclaim, policy dry-run, workload classification, challenge, trust, dispute, and cache-scope decisions consumed by scheduler filters. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes scheduler effort and placement facts through Overmeter while keeping billing, rollups, ledger mutation, payout, and settlement outside Oversched. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Adds Docdex, Mcoda, Codali, SDK, CLI, and admin/developer UI callers that submit workloads, dry-run placement, inspect explanations, cancel requests, and diagnose scheduler blocks. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Requires system-service placement rules, maintenance/drain awareness, protected scheduler operation, failover, backup/restore, leader/owner safety, and grid-resident control-plane readiness. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase/Overstore/Overvault refs, Universal Namespace refs, Overmesh route resolution, storage locality, private data/secret refs, and native-platform readiness signals. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies richer workload/app deployment manifests, package validation, deployment-planner intent, release strategy refs, and rollback health constraints that Oversched can consume but not own. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Adds trusted federation and public-interest candidate-pool separation, purpose tags, grant constraints, partner capacity, and replayable fairness profiles. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Adds public-provider candidate pools only for eligible low-sensitivity, no-secret workloads with sandbox, fraud, challenge, payout-hold, and redaction constraints. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Consumes placement status, no-candidate explanations, usage refs, and redacted decision summaries through normal Overrid APIs for native app workflows. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies retention, compliance, threat-model, incident, PIP, migration, and governance hardening for scheduler contracts, policy rollout, replay evidence, and explanation redaction. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #26 first build work aligned to master Phase 3, with later trust, metering, product, grid-resident, storage/namespace, federation, public-provider, native-app, and governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, authenticated HTTP/2 with mTLS/rustls for seed paths, canonical JSON plus JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and native Overrid service boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 2, 3, 4, 5, 10, 11, and 13 | Attach SDS #26, freeze Oversched scope, preserve Phase 3 as the first build point, and record later trust, fairness, public-provider, and governance gates. |
| 2 | Master Phases 0, 1, and 3 | Build Rust contracts, schemas, reason-code catalogs, policy-version records, fixtures, and local harnesses before placement side effects. |
| 3 | Master Phases 1, 2, 3, 4, and 8 | Claim queue items and assemble versioned fact bundles from owning services without reading private dependency storage directly. |
| 4 | Master Phases 2, 3, and 4 | Implement deterministic private-swarm hard filters for tenant visibility, node state, resource fit, capability, workload/data class, policy, trust, grant, cache, and locality. |
| 5 | Master Phases 3, 4, 5, and 10 | Add explainable scoring, deterministic tie-breaks, fairness profiles, scheduler policy versions, and reason-code evidence. |
| 6 | Master Phase 3 | Request single-node leases through Overlease, bind lease proofs back to queue/Overrun handoff, and keep reservation authority outside Oversched. |
| 7 | Master Phases 3, 4, and 5 | Handle no-candidate, blocked, cancellation, dependency outage, retry, lease denial, and dead-letter handoff with explicit reason codes. |
| 8 | Master Phases 3, 4, 6, and 13 | Add replay, dry-run, explanation redaction, scheduler-policy rollout evidence, admin/CLI/SDK reads, and governance-friendly audit views. |
| 9 | Master Phases 7, 8, 9, 10, 11, 12, and 13 | Expand only after single-node private scheduling is stable: system-service scheduling, namespace/storage locality, deployment intent, federation/public pools, native-app consumption, and governance. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, tech-stack alignment, queue state, progress evidence, and implementation handoff gates. |

## Tech Stack Guardrails

- Oversched core is a Rust control-plane service/module using shared contract types, Tokio for async service calls, and Axum/Tower/Hyper-style HTTP only where an internal service boundary exists.
- Scheduler contracts use canonical JSON plus JSON Schema for docs-facing examples, fixtures, command payloads, placement decisions, explanations, events, and replay bundles. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating calls use signed envelopes, service identity, tenant context, idempotency keys, trace ids, scheduler policy versions, stable reason codes, and append-only events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for manifest refs, fact bundles, replay bundles, policy bundles, candidate snapshots, and decision evidence.
- Oversched consumes Overqueue, Overregistry, Hardware Discovery, Benchmark Runner, Overcell, Overpack, Overguard, Oververify, Overgrant, Overmark, Overcache, Overmesh, Overlease, and Overwatch APIs. It must not read their private storage directly or create their facts.
- PostgreSQL, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFT, pricing, revenue, customer-count, opaque global optimizer, or external-payment assumptions must not become Oversched's product boundary.

## Phase 1: SDS Attachment, Scheduler Scope, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #26.**
  - Design: Link this document from the numbered Oversched SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/execution_scheduling/oversched.md`, `docs/service_catalog/execution_scheduling/oversched.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #26 returns both the Oversched SDS and this sub-build plan.

- **1.2 Freeze Oversched as placement decision authority.**
  - Design: Record that Oversched owns scheduler request intake, fact collection orchestration, deterministic filtering, candidate scoring, placement decisions, reason codes, replay bundles, and lease-request handoff.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Oversched does not execute workloads, reserve resources directly, mutate node facts, mint trust/policy/grant/cache facts, own billing/ORU state, or expose provider-private capacity.

- **1.3 Preserve master Phase 3 as the first build point.**
  - Design: Keep first implementation in master Phase 3 because Oversched depends on Phase 1 signed queue/audit/registry primitives and Phase 2 live node/capability facts.
  - Output: Phase-gate note that Phase 0 through Phase 2 are prerequisites and Phase 3 proves deterministic private-swarm placement before broader pools.
  - Validation: Review proves this plan does not move Oversched into Phase 0, Phase 1, or Phase 2 and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #26 decisions for deterministic private fairness in Phase 3, bounded tenant soft preferences, no-candidate explanation redaction, all-or-none gang scheduling after single-node stability, and mandatory dry-run replay before decision-affecting policy rollout.
  - Output: Resolved-decision checklist tied to SDS #26 open-question answers.
  - Validation: Review rejects public/federation fairness before Phase 10/11 profiles, raw provider-capacity exposure, tenant overrides of hard policy/grant/trust gates, partial gang scheduling, and policy rollout without replay evidence.

- **1.5 Define scheduler authority boundaries.**
  - Design: Create a boundary matrix for Overqueue, Overregistry, Hardware Discovery, Benchmark Runner, Overcell, Overpack, Overguard, Oververify, Overgrant, Overmark, Overcache, Overmesh, Overlease, Overrun, Overmeter, Overwatch, Overclaim, SDK, CLI, admin UI, product adapters, and governance services.
  - Output: Boundary matrix listing consumed refs, emitted refs, denial behavior, retry owner, redaction profile, and replay evidence for each dependency.
  - Validation: Review confirms every dependency has an owning service and every private fact reaches Oversched through an explicit API/ref.

## Phase 2: Rust Contracts, Schemas, Reason Codes, And Fixtures

### Work Items

- **2.1 Create the Oversched Rust contract module.**
  - Design: Add contract types for scheduling requests, scheduler policy versions, candidate snapshots, filter results, candidate scores, placement decisions, lease requests, gang lease plans, scheduler backoff, and replay bundles.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, state enums, API error types, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms scheduler contracts remain separate from runner, lease, policy, trust, grant, cache, storage, and accounting logic.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for claim, evaluate, reserve, explain, cancel, replay, candidate snapshot, filter result, score result, placement decision, lease request, no-candidate result, and replay bundle payloads.
  - Output: Schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing tenant, queue, workload, manifest, node, policy, scheduler-policy-version, trace, idempotency, input-fact-version, decision-state, and reason-code fields where required.

- **2.3 Build the reason-code and redaction catalog.**
  - Design: Define stable reason codes for tenant visibility, node state, resource fit, capability, workload/data class, policy denial, trust/grant/cost/cache/locality block, stale facts, fairness block, no-candidate, cancellation, lease denial, and dependency outage.
  - Output: Reason-code catalog, severity/retryability mapping, remediation hint rules, user/operator/audit redaction profiles, and localization-safe identifiers.
  - Validation: Catalog tests prove every candidate rejection and terminal scheduler state maps to a stable reason code and redaction profile.

- **2.4 Define scheduler policy version records.**
  - Design: Model filter chain version, scoring rule version, tie-break algorithm, fairness profile, candidate-pool mode, gang-scheduling mode, redaction profile, and compatibility date as explicit decision inputs.
  - Output: Scheduler policy schema, rollout metadata, compatibility matrix, fixture policies, and policy-version refs in placement decisions.
  - Validation: Determinism tests prove the same fact bundle and scheduler policy version produce the same decision and explanation.

- **2.5 Create deterministic fixtures and harness scenarios.**
  - Design: Build fixtures for valid private placement, no eligible nodes, stale heartbeat, policy denial, missing manifest, grant block, cache/locality hint, lease denial, cancellation, dependency outage, fairness block, and first gang scheduling denial.
  - Output: Fixture directory, expected candidate snapshots, filter results, score results, placement decisions, lease requests, replay hashes, and harness scenario notes.
  - Validation: Fixture tests produce stable output and prove ambiguous or missing facts fail closed as blocked or denied states.

## Phase 3: Queue Intake, Fact Collection, And Snapshot Assembly

### Work Items

- **3.1 Implement queue claim and scheduling request intake.**
  - Design: Claim accepted Overqueue scheduler-lane items idempotently with queue item id, workload id, manifest id/version, tenant id, actor/service account, priority class, retry index, deadline, idempotency key, and trace id.
  - Output: Claim handler/client contract, scheduling request record, duplicate-claim behavior, queue conflict response, and `oversched.request_claimed` event.
  - Validation: Intake tests reject missing tenant/trace/idempotency refs, incompatible duplicate claims, cancelled queue items, and unauthorized non-Overqueue scheduling requests.

- **3.2 Load workload, manifest, and tenant facts.**
  - Design: Fetch Overpack manifest refs, resource cards, workload class, data class, egress/secret declarations, timeout/deadline, tenant visibility, quota scope, and policy input refs through owning services.
  - Output: Workload fact bundle, manifest compatibility report, tenant scope snapshot, missing-fact reason codes, and audit refs.
  - Validation: Fact-loading tests reject missing manifests, stale manifest versions, wrong tenant scope, incompatible data class, and missing policy inputs before candidate collection.

- **3.3 Collect node and capacity facts.**
  - Design: Fetch Overregistry node/capability facts, Hardware Discovery inventory refs, Benchmark Runner capacity refs, Overcell heartbeat/load state, node state, accelerator runtime flags, and collected-at timestamps.
  - Output: Candidate node fact snapshot, source-service refs, freshness metadata, stale-fact handling, and fact-version index.
  - Validation: Snapshot tests prove stale, missing, unauthorized, or cross-tenant node facts are excluded or blocked according to scheduler policy.

- **3.4 Collect policy, trust, grant, cost, cache, and route refs.**
  - Design: Fetch Overguard policy decisions, Oververify trust refs, Overgrant grant constraints, Overmark cost-class signals, Overcache cache hints, and Overmesh locality/route hints without taking ownership of those facts.
  - Output: Auxiliary fact bundle, dependency freshness fields, blocked dependency states, and redacted source summaries.
  - Validation: Integration tests prove missing required refs block scheduling and optional hint providers cannot silently grant permission.

- **3.5 Build immutable candidate snapshots.**
  - Design: Combine request, manifest, tenant, node, policy, trust, grant, cache, route, and freshness refs into immutable candidate snapshots with source service, version, timestamp, and traceable ref for every input fact.
  - Output: Candidate snapshot store, replay bundle seed, snapshot hash, event payload, and redaction-safe summary.
  - Validation: Replay tests reconstruct candidate snapshots from stored refs and prove Oversched never reads dependency private storage directly.

## Phase 4: Deterministic Candidate Filtering

### Work Items

- **4.1 Implement tenant visibility and node-state filters first.**
  - Design: Filter candidates by tenant visibility, provider/node authorization, node lifecycle state, heartbeat age, maintenance/drain state, capacity freshness, and cancellation status before scoring.
  - Output: Filter functions, pass/fail/unknown records, reason codes, remediation hints, and operator diagnostics.
  - Validation: Filter tests prove tenant-invisible, stale, drained, expired, unauthorized, or cancelled candidates cannot reach scoring.

- **4.2 Implement resource and capability fit filters.**
  - Design: Match Overpack resource cards against observed CPU, memory, GPU, storage, bandwidth, runtime support, accelerator adapter, region/locality, and capability records.
  - Output: Resource-fit report, capability mismatch reasons, freshness requirements, and scheduler-policy controlled thresholds.
  - Validation: Resource tests reject insufficient capacity, unsupported runtime, missing accelerator hook, stale benchmark evidence, and region/locality hard mismatches.

- **4.3 Implement workload, data, and package policy filters.**
  - Design: Apply workload class, data class, secret-bearing status, egress declarations, package trust, runtime class, public/private/system-service separation, and Overguard policy refs before scoring.
  - Output: Policy filter chain, matched policy refs, denial reason mapping, and redacted explanation fields.
  - Validation: Policy tests prove denied egress, forbidden data class, missing package trust, secret-bearing mismatch, and system/public pool violations fail closed.

- **4.4 Implement trust, grant, cost-class, cache, and locality filters.**
  - Design: Apply Oververify trust refs, Overgrant grant constraints, Overmark cost-class signals, Overcache scoped cache hints, Overmesh route/locality constraints, and dependency freshness rules.
  - Output: Auxiliary filter results, grant/cost/cache/locality reason codes, optional versus required hint semantics, and blocked-state handoff.
  - Validation: Tests prove expired grants, trust failures, cost-class incompatibility, cache-scope violations, stale route refs, and missing required dependencies produce explicit rejection or blocked states.

- **4.5 Record full filter histories.**
  - Design: Store pass/fail/unknown filter results for every candidate with filter name, input refs, severity, reason code, retryability, remediation hint, and safe explanation summary.
  - Output: Append-only filter result records, candidate rejection events, no-candidate input summary, and replay bundle entries.
  - Validation: Tests prove rejected candidates have enough evidence for audit replay without exposing private provider topology or raw capacity details to user views.

## Phase 5: Explainable Scoring, Fairness, And Tie-Breaks

### Work Items

- **5.1 Implement candidate scoring for eligible private nodes.**
  - Design: Score only candidates that passed hard filters using resource fit, locality, cache affinity, trust class, grant fit, cost-class signal, deadline sensitivity, and scheduler policy version.
  - Output: Candidate score records, score explanation schema, score component weights, confidence/freshness notes, and event payloads.
  - Validation: Score tests prove ineligible candidates are never resurrected by scoring and every component has an input ref.

- **5.2 Implement deterministic tie-breaks.**
  - Design: Define stable tie-break keys that use scheduler policy version, tenant/request identifiers, candidate ids, priority class, retry/age ordering, lease availability, and deterministic sorting without random or opaque optimization.
  - Output: Tie-break algorithm spec, Rust implementation, fixtures for equal scores, and explanation refs.
  - Validation: Determinism tests prove equal inputs always choose the same candidate and tie-break reasons appear in the placement decision.

- **5.3 Implement Phase 3 private fairness profile.**
  - Design: Start with queue priority, retry/age ordering, lease availability, max attempt windows, no-starvation aging, and stable tie-breaks inside tenant-visible private candidate pools.
  - Output: Private fairness policy version, fairness reason codes, blocked-by-fairness state, and replay fixtures.
  - Validation: Fairness tests prove Phase 3 fairness remains deterministic and does not claim public/federation fairness readiness.

- **5.4 Bound tenant soft preferences.**
  - Design: Allow tenant preferences only inside already-allowed pools, such as locality preference, cache affinity, speed-versus-cost-class preference, batch versus interactive priority, deadline sensitivity, and tenant-visible provider allow/avoid preferences.
  - Output: Soft-preference schema, validation rules, precedence matrix, and rejection reasons for invalid overrides.
  - Validation: Tests prove tenant preferences cannot override Overguard denials, grant limits, trust requirements, candidate-pool separation, lease safety, fairness caps, or explanation redaction.

- **5.5 Write placement decision records.**
  - Design: Store selected candidate or no-candidate result, selected reason summary, rejected candidate reason refs, input fact versions, scheduler policy version, score/tie-break refs, lease request refs, and replay bundle ref.
  - Output: Append-only placement decision record, `oversched.placement_selected` or `oversched.no_candidate` event, redacted read model, and replay seed.
  - Validation: Decision tests prove later corrections create new decision versions rather than mutating historical decisions.

## Phase 6: Overlease Reservation And Assignment Handoff

### Work Items

- **6.1 Build single-node lease request contracts.**
  - Design: Convert selected placement decisions into Overlease reservation requests with node id, resource reservation, lease window, renewal policy, workload refs, tenant refs, atomicity mode, and placement decision id.
  - Output: Lease request schema, Overlease client contract, idempotency key rules, and lease request event.
  - Validation: Contract tests reject missing decision ids, wrong tenant/workload refs, unsupported reservation windows, and non-idempotent duplicate requests.

- **6.2 Request reservations only after placement decisions.**
  - Design: Call Overlease only after candidate filters, score, tie-break, placement decision, and replay bundle seed are written.
  - Output: Reservation workflow, state transition from selected to reserving, Overlease response mapping, and trace refs.
  - Validation: Workflow tests prove dry-run evaluation never creates leases and live reservation cannot bypass decision records.

- **6.3 Bind lease proofs to placement decisions.**
  - Design: Store lease proof refs, lease expiry, Overlease denial reasons, selected node refs, and queue assignment refs without taking ownership of lease state.
  - Output: `oversched.lease_requested`, `oversched.lease_bound`, and `oversched.lease_denied` events; decision update version; read-model fields.
  - Validation: Lease tests prove expired, denied, stale, wrong-node, wrong-workload, and duplicate lease responses produce explicit states.

- **6.4 Hand off leased assignments through Overqueue and Overcell.**
  - Design: Bind successful lease refs back to queue assignment state and hand leased assignment refs toward Overcell/Overrun while keeping Overrun responsible for lease and manifest verification before execution.
  - Output: Assignment handoff contract, queue state update refs, Overrun handoff refs, and audit events.
  - Validation: Handoff tests prove Overrun receives only lease-bound assignments and no direct runner command is created by Oversched.

- **6.5 Emit scheduler usage and audit facts.**
  - Design: Emit candidate count, filter count, score count, scheduling attempt, blocked duration, no-candidate count, lease request count, replay count, and final scheduling state as raw usage/diagnostic facts.
  - Output: Overmeter raw scheduler usage event contract, Overwatch audit mapping, idempotent retry/spool behavior, and source refs.
  - Validation: Usage tests prove scheduler usage is reconstructable without becoming billing, pricing, settlement, or ORU balance mutation.

## Phase 7: No-Candidate, Blocked, Cancellation, And Lease-Denial Flows

### Work Items

- **7.1 Implement no-candidate decisions.**
  - Design: Produce no-candidate decisions when all candidates fail or are unknown, with coarse user-safe remediation categories and detailed operator/audit refs behind redaction profiles.
  - Output: No-candidate record, explanation summary, rejected candidate reason refs, Overqueue retry/dead-letter handoff, and events.
  - Validation: Redaction tests prove user explanations do not expose node ids, hostnames, exact provider capacity, private topology, raw benchmark values, competing tenant counts, payout/dispute internals, or private route details.

- **7.2 Implement blocked-state handling.**
  - Design: Block scheduling when dependency facts are stale/unavailable, policy/grant/trust state is unresolved, fairness windows require delay, capacity is temporarily unavailable, or lease race retry is appropriate.
  - Output: Scheduler backoff record, retry-after rules, blocked lane, max retry, dependency status refs, and `oversched.request_blocked` event.
  - Validation: Blocked-state tests prove missing facts are not treated as permission and blocked work resumes only from durable request/state refs.

- **7.3 Implement authorized cancellation.**
  - Design: Stop scheduling when queue state, tenant policy, actor authorization, or deadline cancellation says the request should not continue.
  - Output: Cancel API, cancellation state transition, reason codes, idempotent duplicate behavior, and `oversched.request_cancelled` event.
  - Validation: Cancellation tests cover queued, claimed, loading, filtering, scoring, selected, reserving, leased, blocked, and expired states.

- **7.4 Implement lease-denial recovery.**
  - Design: On Overlease denial, classify stale capacity, concurrency race, policy mismatch, expired decision, invalid request, or terminal denial and choose retry-with-fresh-facts, blocked, or dead-letter handoff.
  - Output: Lease-denial classification, retry policy, fresh-fact reload path, new decision version rules, and audit refs.
  - Validation: Lease-race tests prove concurrent reservations and stale capacity produce safe retry or blocked states without mutating historical decisions.

- **7.5 Implement crash and dependency outage recovery.**
  - Design: Resume from append-only scheduling request, candidate snapshot, decision, lease request, and idempotency keys after scheduler crash or dependency outage.
  - Output: Recovery workflow, idempotent replay order, duplicate suppression, corruption detection, and health signal refs.
  - Validation: Recovery tests cover crash before decision, after decision, during lease request, after lease denial, during cancellation, and during final queue handoff.

## Phase 8: Replay, Explanations, Redaction, And Policy Rollout

### Work Items

- **8.1 Implement dry-run evaluation.**
  - Design: Let authorized callers collect facts, filter candidates, score candidates, and produce placement decisions without creating leases or mutating queue finality beyond audit events.
  - Output: Dry-run API behavior, dry-run decision record, redacted explanation view, and audit event.
  - Validation: Dry-run tests prove no Overlease call or queue assignment mutation occurs during dry-run evaluation.

- **8.2 Implement replay from stored fact versions.**
  - Design: Rebuild decisions from stored request, candidate snapshots, filter results, score results, scheduler policy version, lease response refs, and replay bundle refs, not current live facts.
  - Output: Replay endpoint, replay result schema, hash comparison, mismatch report, and audit refs.
  - Validation: Replay tests prove historical decisions remain reproducible even when live node, policy, trust, grant, cache, or route facts have changed.

- **8.3 Build explanation read models.**
  - Design: Provide user, operator, audit, Overwatch, Overclaim, admin UI, CLI, SDK, and central-AI review explanation views with appropriate redaction and source refs.
  - Output: Explanation API/read model, redaction profiles, remediation hint rules, and view-specific fixtures.
  - Validation: Redaction tests prove user views expose only coarse remediation categories while operator/audit views follow refs through their owning services.

- **8.4 Gate scheduler policy rollout with replay evidence.**
  - Design: Require dry-run replay before rollout for changes affecting eligibility, selected node, no-candidate state, lease request shape, reason-code output, or explanation content.
  - Output: Rollout checklist, replay sample set, approval evidence refs, compatibility report, and scheduler policy version update event.
  - Validation: Rollout tests reject active scheduler policy changes without traceable replay evidence unless the change is dashboard-only and decision-neutral.

- **8.5 Expose operations dashboards and health checks.**
  - Design: Publish queue depth by scheduler lane, active scheduling requests, no-candidate reason distribution, blocked dependencies, lease-denial rate, latency, replay health, policy version rollout status, and fairness window state.
  - Output: Health endpoint/schema, metrics/events, redacted diagnostics, dashboard fields, and operator follow-up refs.
  - Validation: Operations tests prove diagnostics include evidence refs and reason codes but not raw private capacity, private payloads, or dependency internals outside authority.

## Phase 9: Gang Scheduling, Federation, Public, And System-Service Expansion

### Work Items

- **9.1 Add all-or-none gang scheduling after single-node stability.**
  - Design: Require group id, member roles, required node count, per-member resource cards, shared manifest, scheduler policy version, locality/anti-affinity constraints, shared lease window, and rollback behavior.
  - Output: Gang lease plan schema, candidate-set scorer, all-or-none placement decision, Overlease atomic set request, and rollback refs.
  - Validation: Gang tests prove Overrun starts no member until Overlease commits the full set and partial/quorum/degraded/elastic modes remain denied.

- **9.2 Prepare system-service scheduling gates.**
  - Design: Add stricter candidate pools, trust rules, maintenance/drain behavior, update/rollback windows, backup/restore refs, failover constraints, and protected placement policy for Phase 7 backbone workloads.
  - Output: System-service scheduler profile, protected candidate-pool rules, maintenance hooks, and rollback/failover evidence refs.
  - Validation: System-service tests prove backbone workloads cannot run on untrusted, public, or policy-incompatible nodes.

- **9.3 Integrate native storage, secret, namespace, and route locality.**
  - Design: Consume Overstore, Overvault, Overbase, Universal Namespace, Overmesh, and Overasset refs as scheduler facts for locality, data class, secret policy, storage placement, namespace route, and entitlement-aware placement.
  - Output: Phase 8 fact adapters, locality/ref compatibility checks, blocked-state rules, and migration fixtures.
  - Validation: Integration tests deny cross-tenant refs, wrong data class, stale route/storage/vault refs, and unsupported namespace or entitlement conditions.

- **9.4 Add trusted federation and public-interest candidate pools.**
  - Design: Before Phase 10 eligibility, require versioned fairness profiles backed by Overgrant and Overguard, separate private/trusted-federation/public-interest/system-service pools, purpose-tag quotas, partner capacity refs, and replayable explanations.
  - Output: Federation/public-interest scheduler profile, candidate-pool separation rules, grant/purpose-tag filters, fairness windows, and explanation redaction.
  - Validation: Federation tests prove external capacity cannot become eligible without explicit profile, grant, purpose, trust, accounting, and replay evidence.

- **9.5 Add limited public low-sensitivity scheduling gates.**
  - Design: For Phase 11 public providers, allow only low-sensitivity, no-secret, capped workloads with public sandbox profile, fraud/challenge refs, payout-hold evidence, deny-by-default egress, and provider-private redaction.
  - Output: Public candidate-pool profile, low-sensitivity eligibility filters, challenge/fraud/payout-hold refs, and denial reasons.
  - Validation: Public-provider tests prove private, regulated, secret-bearing, system-service, privileged-runtime, or unknown-provider workloads cannot reach public nodes.

## Phase 10: Validation, Documentation, Queue State, And Handoff

### Work Items

- **10.1 Validate contract, schema, and reason-code coverage.**
  - Design: Run focused checks for scheduling requests, policy versions, candidate snapshots, filter results, score results, placement decisions, lease requests, gang plans, backoff records, replay bundles, APIs, events, and reason codes.
  - Output: Schema-test report, state-machine test report, fixture coverage matrix, failure notes, and remediation list.
  - Validation: Tests pass before implementation advances beyond each documented gate; any blocker is recorded in build-plan progress.

- **10.2 Validate Phase 3 scheduling end to end.**
  - Design: Prove one signed private workload flows through queue claim, manifest/resource-card loading, candidate fact collection, deterministic filtering, scoring, Overlease reservation, queue/Overrun handoff, raw scheduler usage, audit, and replay.
  - Output: End-to-end private scheduling fixture, source-ref bundle, placement decision, lease request/response, assignment refs, usage ref, audit trail, and replay report.
  - Validation: Replay confirms selected, no-candidate, blocked, cancelled, lease-denied, retry, dead-letter, and dependency-outage paths produce distinct auditable states.

- **10.3 Validate security, privacy, and tech-stack alignment.**
  - Design: Scan implementation and docs for raw provider-private capacity leakage, private payload leakage, direct dependency storage reads, policy/trust/grant fact minting, opaque optimizer behavior, conventional cloud-product boundaries, blockchain/NFT mechanics, pricing/revenue/customer-count assumptions, and TypeScript core-runtime drift.
  - Output: Security/privacy checklist, tech-stack alignment report, negative-control scan results, and remediation notes.
  - Validation: Review confirms Oversched remains Rust-first/native-Overrid infrastructure and uses canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and native service boundaries.

- **10.4 Validate master-plan and downstream handoff alignment.**
  - Design: Confirm SDS #26, the Oversched service plan, master build plan, build-plan crosswalk, Phase 3 plan, Phase 4 plan, Phase 5 plan, Phase 7 plan, Phase 8 plan, Phase 10/11 plans, queue state, and progress docs link to this plan and preserve the Phase 3 first build point.
  - Output: Updated source-document links, sub-build-plan index entries, progress evidence, queue status, and downstream handoff matrix.
  - Validation: Markdown link checks pass and review confirms no master Phase 0 through Phase 13 ordering change was required.

- **10.5 Validate implementation handoff readiness.**
  - Design: Prepare the handoff for builders by listing required crates/modules, schemas, reason-code catalogs, service clients, fixture groups, local harness scenarios, acceptance tests, and phase gates.
  - Output: Implementation handoff checklist, validation command list, known blockers, dependency owners, and first-placement fixture target.
  - Validation: Handoff review confirms a builder can start Oversched Phase 3 implementation without reading informal agent notes or weakening SDS boundaries.

## Alignment Review

- The sub-build plan keeps Oversched first build work in master Phase 3, matching SDS #26, the service catalog entry, Phase 3 plan, master build plan, and build-plan crosswalk.
- The plan treats Phase 0 through Phase 2 as prerequisites for shared schemas, local fixtures, signed command/queue/audit/registry primitives, live Overcell nodes, capability facts, and measured capacity evidence rather than as Oversched's first implementation phase.
- The plan keeps Overqueue, Overregistry, Hardware Discovery, Benchmark Runner, Overcell, Overpack, Overguard, Oververify, Overgrant, Overmark, Overcache, Overmesh, Overlease, Overrun, Overmeter, and Overwatch authority outside Oversched while defining the refs and evidence Oversched consumes or emits.
- The plan preserves the Phase 3 constraint that private-swarm fairness is deterministic and narrow, then gates federation/public/system-service expansion behind later master phases.
- The plan keeps user-facing no-candidate explanations coarse and remediation-oriented while preserving richer operator/audit evidence through owning-service refs.
- The plan introduces gang scheduling only after single-node scheduling is stable and keeps it all-or-none until every participating service defines partial/quorum/degraded semantics.
- The plan preserves the master Phase 0 through Phase 13 order and uses later phases only for trust/policy expansion, signed rollups, product integration, grid-resident system-service placement, native storage/namespace locality, deployment-platform compatibility, federation/public constraints, native app consumption, and governance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first scheduler core, native Overrid boundaries, Tokio, Axum/Tower/Hyper-style service boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, customer-count, opaque optimizer, or external-payment assumptions.
