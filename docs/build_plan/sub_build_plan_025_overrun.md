# SUB BUILD PLAN #25 - Overrun

Attached SDS: [docs/sds/execution_scheduling/overrun.md](../sds/execution_scheduling/overrun.md)

## Purpose

This sub-build plan turns SDS #25 into an implementation sequence for Overrun. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overrun is the lease-bound node-side execution runner for the Phase 3 private execution loop. It verifies assignments, leases, manifests, packages, policies, inputs, secret refs, egress rules, runtime adapters, timeouts, and cleanup rules before side effects. It prepares and supervises sandboxes, captures results, emits raw usage and audit evidence, and cleans up local resources. It is not the scheduler, lease authority, package author, policy authority, storage service, vault service, queue owner, metering rollup service, billing service, settlement service, or arbitrary operator shell.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #25: Overrun](../sds/execution_scheduling/overrun.md) | Controls Overrun purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machine, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overrun service plan](../service_catalog/execution_scheduling/overrun.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, canonical JSON/JSON Schema discipline, signed envelopes, idempotency, trace ids, stable reason codes, local fixtures, and integration harness prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, identity, tenant, key, registry, queue, command, and audit primitives that Overrun consumes. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies Overcell node agents, node identity, signed command delivery, capability records, benchmark facts, runtime availability, heartbeat/load state, and local audit spool prerequisites. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Controls Overrun's first build point: lease-bound execution for one narrow workload class with manifest verification, sandboxing, supervision, result capture, raw usage events, cancellation, timeout, retry, and cleanup. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, workload-class rules, egress/data/secret constraints, Oververify evidence, Overclaim dispute refs, and verification gates that Overrun enforces without owning. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes raw execution usage through Overmeter while keeping billing, pricing, settlement, ledger mutation, provider payout, and ORU transitions outside Overrun. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Adds Docdex, Mcoda, Codali, AI gateway, SDK, CLI, and admin/developer UI workloads and views that submit, inspect, cancel, retry, and diagnose Overrun executions. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Requires stricter runner behavior for protected system-service workloads, maintenance, update, rollback, health, backup/restore, failover, and service-runtime eligibility. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overstore, Overvault, Overbase, Universal Namespace, Overmesh route resolution, storage refs, secret refs, private data refs, and migration paths that replace Phase 3 local stubs. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies richer app/deployment manifests, package validation, release strategy, rollback, health checks, and deployment-planner compatibility that Overrun executes only after owning services authorize them. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Adds known external-swarm execution boundaries, purpose-scoped grants, and federation policy refs without weakening lease, manifest, sandbox, or cleanup guarantees. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Adds public-provider sandbox restrictions, no-secret rules, low-sensitivity workload eligibility, challenge/fraud refs, and payout-hold evidence for public nodes. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Consumes run state, logs, output refs, usage refs, cancellation state, and evidence through normal Overrid APIs for native app and wallet/user-facing workflows. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies retention, migration, incident, compliance, threat-model, security-review, PIP, and governance hardening for runner contracts, sandbox guarantees, evidence, and cleanup semantics. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #25 first build work aligned to master Phase 3, with Phase 8 storage/secret integration, Phase 11 public-sandbox constraints, and later governance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, authenticated HTTP/2 with mTLS/rustls for early seed paths, canonical JSON plus JSON Schema, optional Protobuf for compact contracts, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, OCI/container execution first, and native Overrid service boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 2, 3, 4, 5, 8, 11, and 13 | Attach SDS #25, freeze Overrun scope, preserve Phase 3 as the first build point, and record later storage, public-provider, and governance gates. |
| 2 | Master Phases 0, 1, and 3 | Build Rust contracts, schemas, state machines, reason codes, fixtures, and local harnesses before runtime side effects. |
| 3 | Master Phases 1, 2, 3, and 4 | Implement assignment acceptance and preflight by verifying Overcell delivery, Overlease lease proof, Overpack manifest/package integrity, Overguard policy, and adapter readiness. |
| 4 | Master Phases 2, 3, 4, and 8 | Prepare sandboxes, runtime adapters, mounts, egress, secret refs, GPU/runtime hooks, and storage/secret readiness gates. |
| 5 | Master Phase 3 | Supervise runtime execution with progress, logs, metrics, cancellation, timeout, and deterministic state transitions. |
| 6 | Master Phases 3 and 5 | Capture results, output/log/artifact refs, raw usage, audit evidence, and report/spool behavior without owning rollups or billing. |
| 7 | Master Phases 3, 4, and 7 | Implement cleanup, retryability, cleanup-failure handling, residual-resource blocking, node safety, and reconnect/spool recovery. |
| 8 | Master Phase 8 | Replace founder-local storage and secret stubs with contract-compatible Overstore and Overvault paths after readiness validation. |
| 9 | Master Phases 6, 7, 9, 10, 11, 12, and 13 | Harden product, system-service, deployment, federation, public-provider, native-app, and governance execution surfaces. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, and implementation handoff gates. |

## Tech Stack Guardrails

- Overrun core is a Rust node-side service/module using shared contract types, Tokio for async supervision and adapter calls, and Axum/Tower/Hyper-style HTTP only where a service boundary exists.
- Overrun runs through the supervised Rust Overcell agent and accepts only signed assignments that reference valid leases, accepted Overpack manifests, and policy-compatible execution contexts.
- The first execution-eligible workload class is signed command-contract jobs plus OCI/container jobs on the private swarm. Model inference may run only through those supported runtime kinds with explicit model/artifact, GPU, and metering refs. WASI/Wasmtime remains schema-recognized but execution-ineligible until a tested adapter exists.
- OCI-compatible sandboxing is mandatory for founder/provider secret-bearing or real workloads: rootless where possible, namespaces, cgroups/resource limits, seccomp or equivalent syscall filtering, read-only base filesystem, per-run writable scratch, explicit egress policy, and controlled NVIDIA/ROCm hooks when accelerators are exposed.
- Founder Phase 3 local storage and secret stubs are allowed only through explicit `founder_local_object_ref` and `founder_local_secret_ref` profiles with data class, redaction, retention, cleanup, hash, TTL, and migration metadata. Policy-required Overstore or Overvault paths must fail preflight if unavailable.
- Run assignments, attempts, preflight reports, sandbox specs, mount plans, progress records, result records, cleanup records, usage refs, API errors, reason codes, events, and fixtures use canonical JSON plus JSON Schema. Compact Protobuf contracts may be added only where the shared contract layer requires them.
- Ed25519 is used for signatures where signatures are required. BLAKE3/content hashes are used for manifests, packages, artifacts, output refs, log/artifact refs, cleanup checkpoints, source sets, and replay evidence.
- Overrun must never log or persist raw secrets, private inputs, secret-bearing temp state, provider-private topology, or private payloads where refs and redacted evidence are enough.
- PostgreSQL, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFT, pricing, revenue, customer-count, arbitrary shell execution, or external payment assumptions must not become Overrun's product boundary.

## Phase 1: SDS Attachment, Runner Scope, And Phase-Gate Rules

### Work Items

- **1.1 Attach the build plan to SDS #25.**
  - Design: Link this document from the numbered Overrun SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/execution_scheduling/overrun.md`, `docs/service_catalog/execution_scheduling/overrun.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #25 returns both the Overrun SDS and this sub-build plan.

- **1.2 Freeze Overrun as the lease-bound node-side runner.**
  - Design: Record that Overrun owns assignment acceptance, preflight, sandbox prep, runtime invocation, supervision, result capture, usage emission, cleanup, and execution evidence.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overrun does not choose placement, create leases, author manifests, decide final policy, store authoritative objects/secrets, mutate queue finality, aggregate billing rollups, settle providers, or run arbitrary operator shell commands.

- **1.3 Preserve master Phase 3 as the first build point.**
  - Design: Keep first implementation in master Phase 3 because Overrun depends on Phase 1 signed command/queue/audit primitives, Phase 2 live Overcell nodes, Overpack manifests, Oversched placement, Overlease reservations, and Overmeter raw usage events.
  - Output: Phase-gate note that Phase 0 through Phase 2 are prerequisites and Phase 3 proves one safe private workload class before broad runtime coverage.
  - Validation: Review proves this plan does not move Overrun into Phase 0, Phase 1, or Phase 2 and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #25 decisions for signed command-contract jobs plus OCI/container jobs first, OCI-compatible sandboxing, partial-output quarantine/release rules, bounded cleanup retries, settlement-ready versus diagnostic metrics, and founder-local storage/secret stubs.
  - Output: Resolved-decision checklist tied to SDS #25 open-question answers.
  - Validation: Review rejects unsupported runtime eligibility, arbitrary shell execution, silent fallback from required Overstore/Overvault paths, secret-bearing public execution, and partial-output release without manifest and data-class permission.

- **1.5 Define runtime authority boundaries.**
  - Design: Create a boundary matrix for Overcell, Overqueue, Oversched, Overlease, Overpack, Overguard, Overvault, Overstore, Overmeter, Overwatch, Overclaim, Overkey, Overregistry, SDK, CLI, admin UI, product adapters, public-provider controls, and governance services.
  - Output: Authority matrix and handoff checklist for implementation reviews.
  - Validation: Review confirms every cross-service dependency has an owner, consumed refs, emitted refs, denial behavior, and replay evidence.

## Phase 2: Rust Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the Overrun Rust contract module.**
  - Design: Add contract types for run assignments, execution attempts, preflight reports, sandbox specs, input mounts, secret mounts, storage/secret adapter readiness, progress records, run results, cleanup records, usage refs, and stable reason codes.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, state enums, reason-code mapping, API error types, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms runner contracts remain separate from scheduler, lease, policy, storage, vault, and accounting logic.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add schemas for assignment accept/reject, run start, progress, cancellation, completion, cleanup retry, read-model, events, preflight reports, sandbox specs, mount plans, result refs, cleanup records, and adapter readiness.
  - Output: Versioned schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing assignment, lease, manifest, node, tenant, trace, idempotency, runtime, timeout, policy, adapter-readiness, final-state, and cleanup fields where required.

- **2.3 Define the execution attempt state machine.**
  - Design: Model assigned, preflight, rejected, sandboxing, starting, running, cancelling, timing_out, succeeded, failed_retryable, failed_final, cancelled, timed_out, cleaning, cleanup_failed, and reported as append-only state history.
  - Output: State transition table, legal transition rules, terminal-state semantics, cleanup overlay behavior, reason codes, and event payload refs.
  - Validation: State tests reject skipping preflight, starting without sandboxing, erasing terminal execution state after cleanup failure, reporting before cleanup attempt, and double terminal reports.

- **2.4 Build deterministic run fixtures.**
  - Design: Create fixtures for valid signed command-contract job, valid OCI/container job, unsupported WASI, missing lease, expired lease, wrong node lease, tampered artifact, denied egress, missing secret ref, timeout, cancellation, runtime crash, cleanup failure, and result-spool reconnect.
  - Output: Fixture directory, expected preflight reports, state histories, usage refs, audit refs, cleanup records, and replay hashes.
  - Validation: Fixture tests produce deterministic reports and prove invalid or unsafe work fails before sandbox side effects.

- **2.5 Define local harness and adapter readiness scenarios.**
  - Design: Model founder Phase 3 local object/secret refs, runtime adapter availability, GPU adapter availability, storage/vault unavailable behavior, Overmeter/Overwatch outage spool behavior, and migration-required readiness states.
  - Output: Harness scenarios, adapter-readiness matrix, negative fixtures, and expected failure-mode snapshots.
  - Validation: Harness tests prove policy-required Overstore/Overvault paths fail preflight when unavailable, while allowed founder-local stubs preserve final contract shape.

## Phase 3: Assignment Acceptance, Lease Verification, And Preflight

### Work Items

- **3.1 Implement signed assignment intake from Overcell.**
  - Design: Accept assignments only through Overcell/node credentials with signed command delivery, node identity, tenant scope, queue item refs, workload refs, manifest refs, lease refs, policy refs, timeout, and trace id.
  - Output: Assignment intake API/handler, signature verification hook, idempotency behavior, accept/reject response schema, and Overwatch event payloads.
  - Validation: Intake tests reject unsigned assignments, wrong-node assignments, duplicate incompatible idempotency keys, missing trace/tenant refs, and direct non-Overcell runner commands.

- **3.2 Verify active Overlease proof before side effects.**
  - Design: Verify lease id, queue item, workload, node id, resource reservation, tenant, expiry, cancellation/revocation state, lease proof signature/refs, and lease-window compatibility before manifest or sandbox side effects.
  - Output: Lease verification client contract, preflight fields, denial reasons, and stale/expired lease handling.
  - Validation: Lease tests reject missing, expired, cancelled, revoked, wrong-node, wrong-workload, wrong-tenant, stale, and malformed lease proofs before sandbox setup.

- **3.3 Verify Overpack manifest and package integrity.**
  - Design: Verify accepted immutable manifest refs, artifact hashes, signatures, runtime contract, resource cards, input/output declarations, permission declarations, timeout, retry policy, cleanup expectation, SBOM/dependency refs where present, and revocation state.
  - Output: Manifest/package verification pipeline, integrity result schema, Overpack/Overregistry client refs, and tamper reason codes.
  - Validation: Manifest tests reject missing hashes, unsigned packages, revoked signers, mutable image tags without digest refs, unsupported runtime cards, undeclared outputs, and manifest changes after registration.

- **3.4 Enforce Overguard policy preflight.**
  - Design: Check workload class, data sensitivity, egress policy, secret policy, storage policy, package trust, tenant quota refs, provider/node eligibility, runtime class, and policy-decision version before sandbox preparation.
  - Output: Policy preflight request/response contract, denial reason mapping, policy refs in run records, and redacted preflight summaries.
  - Validation: Policy tests prove denied egress, forbidden data class, secret policy mismatch, unsupported public/provider placement, package trust failure, and missing policy refs stop before process start.

- **3.5 Produce idempotent assignment accept/reject records.**
  - Design: Store accept/reject decisions as append-only preflight records with final reason codes, retryability hints, trace refs, audit refs, and no raw secret/private payloads.
  - Output: `overrun.assignment_accepted` and `overrun.assignment_rejected` events, preflight read model, duplicate-request behavior, and retry-safe response rules.
  - Validation: Replay tests reconstruct why an assignment was accepted or rejected from lease, manifest, policy, adapter, and assignment refs without reading node logs.

## Phase 4: Sandbox, Runtime Adapter, Mount, And Egress Preparation

### Work Items

- **4.1 Implement the first runtime adapter profile.**
  - Design: Support signed command-contract bootstrap fixtures and OCI/container jobs first; keep WASI/Wasmtime schema-recognized but execution-ineligible until a tested adapter exists.
  - Output: Runtime adapter trait, command-contract adapter, OCI/container adapter contract, unsupported-runtime denial reasons, and runtime version/compatibility fields.
  - Validation: Adapter tests prove command-contract fixtures cannot become arbitrary shell execution and unsupported runtimes fail closed.

- **4.2 Prepare OCI-compatible sandbox specs.**
  - Design: Build sandbox specs with rootless container preference, namespaces, cgroups/resource limits, seccomp or equivalent filtering, read-only base filesystem, writable scratch, process tree tracking, timeout hooks, and cleanup policy.
  - Output: Sandbox spec schema, sandbox creation workflow, resource-limit mapping, process supervisor hooks, and sandbox-prepared events.
  - Validation: Sandbox tests enforce read-only base, bounded scratch, CPU/memory/runtime caps, process tree kill, denied privileged mounts, and deterministic cleanup refs.

- **4.3 Mount authorized inputs and output locations.**
  - Design: Convert manifest input/output refs into mount plans with data class, checksum expectations, read/write mode, mount paths, content hash checks, quarantine rules, and storage adapter readiness.
  - Output: Input mount plan, output ref plan, content-hash validation, redacted mount summaries, and failure reason codes.
  - Validation: Mount tests reject undeclared inputs, wrong data class, hash mismatch, write access where read-only is required, output path escape, and storage profile mismatch.

- **4.4 Mount secret refs without exposing values.**
  - Design: Mount only allowed secret refs through Overvault or founder-local secret profiles with ttl, least-scope process access, redaction, unmount/zeroization evidence, and migration refs.
  - Output: Secret mount plan, secret adapter readiness checks, redaction rules, unmount evidence schema, and preflight denial reasons.
  - Validation: Secret tests prove raw secret values never appear in run records, logs, progress, result refs, diagnostics, or cleanup records.

- **4.5 Apply egress, accelerator, and environment controls.**
  - Design: Apply egress allow/deny rules, environment refs, DNS/network namespace restrictions, accelerator runtime hooks, NVIDIA/ROCm adapter metadata, model/artifact refs, and default-deny behavior before process start.
  - Output: Egress control plan, environment projection, accelerator adapter contract, policy evidence refs, and denied-egress fixtures.
  - Validation: Runtime tests prove undeclared egress fails closed, GPU hooks are explicit and metered, environment values are ref-based/redacted, and no hidden shell/network bypass exists.

## Phase 5: Runtime Supervision, Progress, Cancellation, And Timeout

### Work Items

- **5.1 Launch workloads through deterministic state transitions.**
  - Design: Transition from sandboxing to starting to running only after lease, manifest, policy, storage, secret, egress, and runtime adapter checks pass.
  - Output: Start API behavior, process supervisor, state history writes, `overrun.execution_started` event, and start failure reason codes.
  - Validation: Start tests reject process launch before preflight completion and prove every started process has assignment, lease, manifest, sandbox, policy, and trace refs.

- **5.2 Stream structured progress and log refs.**
  - Design: Emit sequence-numbered progress with timestamps, progress kind, redacted log refs, metric refs, state hints, and bounded buffering/spool behavior.
  - Output: Progress API/event schema, log-ref writer, redaction filter, sequence enforcement, and replay fixtures.
  - Validation: Progress tests reject out-of-order incompatible writes, raw secrets, private payloads, unbounded logs, and progress records without attempt/trace refs.

- **5.3 Supervise runtime metrics and resource bounds.**
  - Design: Track wall time, CPU time, memory peak, storage read/write, network bytes, process exits, cgroup/container metrics, accelerator/model refs, and confidence/source metadata.
  - Output: Runtime metric collector, metric source refs, diagnostic versus settlement-ready classification, and Overmeter raw-event staging.
  - Validation: Metric tests prove diagnostic-only counters do not become settlement-ready usage and every usage dimension has source refs and confidence metadata.

- **5.4 Enforce idempotent cancellation.**
  - Design: Accept authorized cancellation with reason and deadline, move to cancelling, terminate process tree, capture policy-allowed partial refs, and behave safely if process already exited.
  - Output: Cancellation API, cancellation state transition, process termination workflow, partial-output policy checks, and `overrun.cancel_requested` event.
  - Validation: Cancellation tests cover already exited processes, duplicate cancels, unauthorized cancels, partial-output quarantine, usage up to cancellation, and cleanup after cancellation.

- **5.5 Enforce timeout termination.**
  - Design: Enforce manifest/lease/policy timeout, move to timing_out, terminate process tree, capture metrics up to termination, quarantine/discard partial outputs by policy, and proceed to cleanup.
  - Output: Timeout supervisor, termination evidence refs, timeout result state, usage cutoff behavior, and timeout events.
  - Validation: Timeout tests prove misbehaving workloads cannot evade termination and timed_out remains distinct from failed_retryable, failed_final, cancelled, and cleanup_failed.

## Phase 6: Result Capture, Usage Emission, Audit, And Reporting

### Work Items

- **6.1 Capture final result records.**
  - Design: Produce final state, output refs, log refs, artifact refs, error type, reason code, retryability, runtime metrics, usage refs, audit refs, cleanup status, and redacted summaries.
  - Output: Run result schema, final-state write path, terminal event payloads, read model, and result fixtures.
  - Validation: Result tests prove success, retryable failure, final failure, cancellation, timeout, and cleanup failure produce distinct terminal evidence.

- **6.2 Capture output, log, and artifact refs safely.**
  - Design: Write refs through phase-appropriate storage contracts, quarantine or discard partial outputs by policy, content-hash refs, redact sensitive logs, and avoid raw private content in run records.
  - Output: Output capture workflow, log/artifact ref schema, content-hash validation, quarantine state, and redacted read behavior.
  - Validation: Output tests reject raw secret leakage, private payload leakage, output path escape, missing content hash, and unauthorized partial-output release.

- **6.3 Emit raw usage events to Overmeter.**
  - Design: Emit lease-bound run window, start/end wall time, final state, CPU, memory, storage bytes, network bytes, cleanup overhead, and GPU/model dimensions only from named trusted sources.
  - Output: Overmeter raw usage event contract, source refs, confidence metadata, idempotent emission, and retry/spool behavior.
  - Validation: Usage tests prove failed, cancelled, timed-out, cleanup-failed, and successful runs emit reconstructable usage without billing or settlement mutation.

- **6.4 Emit audit and evidence refs to Overwatch and Overclaim.**
  - Design: Emit assignment, preflight, sandbox, start, progress, cancellation, timeout, result, cleanup, residual-resource, and report-spool events with trace, lease, workload, node, attempt, and policy refs.
  - Output: Overwatch event mapping, Overclaim dispute evidence refs, audit spool integration, and redacted diagnostic fields.
  - Validation: Audit tests reconstruct execution from append-only evidence without scraping node logs or exposing private payloads.

- **6.5 Report final state through control-plane state.**
  - Design: Report final result and cleanup state through documented control-plane APIs, with Overcell local audit spool on network/report failure and idempotent retry behavior.
  - Output: Final report API behavior, local spool contract, reconnect retry policy, duplicate report handling, and read-after-report fixtures.
  - Validation: Report tests cover control-plane outage, duplicate report, partial report failure, reconnect, idempotent replay, and no direct queue-state mutation beyond documented reporting APIs.

## Phase 7: Cleanup, Retryability, Failure Recovery, And Node Safety

### Work Items

- **7.1 Implement cleanup state and evidence.**
  - Design: Cleanup sandbox, mounts, secret refs, temp files, process tree, credentials, network namespace, GPU contexts, local refs, and cache/temp residue according to policy after every terminal execution state.
  - Output: Cleanup workflow, cleanup_record schema, resource removal checklist, retry plan, cleanup events, and evidence refs.
  - Validation: Cleanup tests prove cleanup runs after success, failure, timeout, cancellation, sandbox setup failure, and partial start failure.

- **7.2 Enforce bounded cleanup retries and residual blocking.**
  - Design: Perform initial synchronous cleanup before final reporting, then two bounded retries through Overcell local spool; block new secret-bearing work on affected node profile after severe residual resources remain.
  - Output: Cleanup retry scheduler, backoff policy, residual-resource evidence, node-profile block flag, and operator/incident handoff.
  - Validation: Failure tests prove `cleanup_failed` preserves execution terminal state, emits evidence, blocks affected secret-bearing profiles, and does not hide residual-resource risk.

- **7.3 Classify retryable and final failures.**
  - Design: Classify failures by preflight denial, input unavailable, sandbox setup failure, runtime crash, policy revocation, output capture failure, report failure, cleanup failure, and node disconnect.
  - Output: Retryability matrix, reason-code catalog, queue/dead-letter handoff notes, and Overclaim evidence refs.
  - Validation: Retry tests prove Overrun reports facts while Overqueue and policy services decide retry/dead-letter behavior.

- **7.4 Recover from node disconnect and report outages.**
  - Design: Spool progress, result, usage, audit, and cleanup evidence locally through Overcell, then replay idempotently on reconnect without inventing missing final states.
  - Output: Local spool contract, replay order, duplicate suppression, corruption handling, and reconnect health signals.
  - Validation: Recovery tests cover disconnect before start, during running, during cleanup, during usage emission, and during final report.

- **7.5 Expose operational health and diagnostics.**
  - Design: Publish health checks for runtime adapter availability, sandbox creation, lease verification, manifest verification, storage refs, secret refs, egress controls, Overmeter/Overwatch reporting, cleanup backlog, and residual warnings.
  - Output: Health endpoint/schema, diagnostic reason codes, admin read model, redaction matrix, and operator follow-up refs.
  - Validation: Operations tests prove diagnostics include evidence refs and reason codes but not raw private content, secret values, or provider-private topology.

## Phase 8: Storage, Secret Adapter Readiness, And Native Platform Integration

### Work Items

- **8.1 Formalize founder-local object and secret profiles.**
  - Design: Restrict Phase 3 stubs to `founder_local_object_ref` and `founder_local_secret_ref` with BLAKE3 hashes, per-run directories, TTL, redaction, unmount/zeroization evidence, retention, cleanup, and migration metadata.
  - Output: Stub profile schemas, allowed policy matrix, migration refs, denial reasons, and redacted read views.
  - Validation: Stub tests reject public/provider pools, regulated workloads, third-party secrets, cross-node durability claims, and assignments whose policy requires Overstore or Overvault.

- **8.2 Integrate Overstore-backed inputs, outputs, logs, and artifacts.**
  - Design: Replace local object refs with Overstore contracts for content-addressed inputs, outputs, logs, artifacts, package refs, model refs, retention, replication/repair refs, and access-policy checks.
  - Output: Overstore adapter contract, URI scheme support matrix, migration tests, read/write grant checks, and content-hash verification.
  - Validation: Storage tests prove missing Overstore readiness fails preflight and Overrun does not become the object store.

- **8.3 Integrate Overvault-backed secret refs.**
  - Design: Replace local secret refs with Overvault contracts for secret scopes, mount mechanism, ttl enforcement, redaction guarantees, access policy, audit evidence, and unmount/zeroization.
  - Output: Overvault adapter contract, secret ref scheme support, mount/unmount evidence, access denial reasons, and migration tests.
  - Validation: Vault tests prove raw secrets remain outside run records/logs/results and Overrun fails preflight if required Overvault paths are unavailable.

- **8.4 Enforce data-class and namespace-aware refs.**
  - Design: Apply data class, tenant/app namespace, route/service refs, private storage refs, model/RAG refs, and Overmesh route compatibility once Phase 8 services exist.
  - Output: Data-class compatibility checks, namespace/ref validation hooks, route/service compatibility notes, and blocked-state reporting.
  - Validation: Integration tests deny cross-tenant refs, wrong data class, unauthorized namespace refs, and stale route/storage/vault refs.

- **8.5 Retire local stubs through explicit migration gates.**
  - Design: Define conditions for retiring founder-local object/secret profiles: contract-compatible Overstore/Overvault paths, fixture parity, cleanup parity, migration evidence, and policy update.
  - Output: Stub-retirement checklist, migration replay report, policy-version refs, compatibility matrix, and deprecation events.
  - Validation: Migration tests prove existing Phase 3 fixtures remain replayable and no workload silently falls back to local files after policy requires native storage/vault.

## Phase 9: Product, System-Service, Deployment, Public, And Governance Hardening

### Work Items

- **9.1 Harden product integration workloads.**
  - Design: Support Docdex, Mcoda, Codali, model-routing/RAG, SDK, CLI, and admin/developer UI run inspection using normal Overrid submission, cancellation, result, usage, and audit contracts.
  - Output: Product fixture set, SDK/CLI commands, admin read-model fields, redaction matrix, and product adapter handoff notes.
  - Validation: Product tests prove one real product workload survives retry, cancellation, timeout, usage emission, and redacted operator inspection.

- **9.2 Harden grid-resident system-service execution.**
  - Design: Add stricter requirements for system-service workload class, trusted placement, upgrade/rollback, maintenance, service health, backup/restore, failover, and protected runtime policy.
  - Output: System-service runner constraints, maintenance/drain hooks, rollback evidence, and Phase 7 compatibility tests.
  - Validation: System-service tests prove backbone workloads cannot run on untrusted/public nodes and produce stronger evidence before update/rollback.

- **9.3 Consume deployment-platform manifests safely.**
  - Design: Execute Phase 9 deployment outputs only when Overpack app/deployment manifests, Package Validator, Deployment Planner, Release Strategy Service, storage, namespace, route, policy, and budget refs are authorized.
  - Output: Deployment execution compatibility contract, release strategy refs, health check hooks, rollback run refs, and denied-deployment fixtures.
  - Validation: Deployment tests prove Overrun executes approved runtime steps without becoming the deployment planner, package validator, route owner, or storage/vault owner.

- **9.4 Enforce public low-sensitivity sandbox constraints.**
  - Design: For Phase 11 public providers, enforce no secrets, no private tenant data, no regulated data, capped runtime/resources, deny-by-default egress, public sandbox profile, challenge refs, fraud refs, and payout-hold evidence.
  - Output: Public sandbox profile, low-sensitivity eligibility checks, challenge/fraud evidence refs, and public-provider denial reasons.
  - Validation: Public-provider tests prove private, regulated, secret-bearing, system-service, privileged-runtime, or unknown-provider workloads cannot reach public nodes.

- **9.5 Add governance, incident, compliance, and threat-model hooks.**
  - Design: Preserve runner contract versions, sandbox evidence, incident refs, compliance boundary refs, threat-model findings, security-review refs, retention refs, migration refs, and PIP/governance refs for audit replay.
  - Output: Governance evidence bundle, compliance export fields, incident/revocation refs, and threat-model coverage matrix.
  - Validation: Governance tests prove an operator can trace a run from assignment through lease, manifest, policy, sandbox, execution, usage, cleanup, and final report.

## Phase 10: Validation, Documentation, Queue State, And Handoff

### Work Items

- **10.1 Validate contract, schema, and state-machine coverage.**
  - Design: Run focused checks for assignments, attempts, preflight reports, sandbox specs, mount plans, adapter readiness, progress, results, cleanup, events, API errors, state transitions, and reason codes.
  - Output: Schema-test report, state-machine test report, fixture coverage matrix, failure notes, and remediation list.
  - Validation: Tests pass before implementation advances beyond each documented gate; any blocker is recorded in build-plan progress.

- **10.2 Validate Phase 3 execution end to end.**
  - Design: Prove one signed command-contract or OCI/container workload flows through queue, Overpack validation, Oversched placement, Overlease reservation, Overcell delivery, Overrun preflight/execution, Overmeter usage, Overwatch audit, result return, and cleanup.
  - Output: End-to-end private workload fixture, source-ref bundle, run result, usage ref, audit trail, cleanup record, and replay report.
  - Validation: Replay confirms successful, rejected, retryable failure, final failure, cancelled, timed-out, cleanup-failed, and report-spooled paths produce distinct auditable states.

- **10.3 Validate security, privacy, and tech-stack alignment.**
  - Design: Scan implementation and docs for raw secret leakage, private payload leakage, arbitrary shell execution, unsupported runtime eligibility, silent local fallback, conventional cloud-product boundaries, blockchain/NFT mechanics, pricing/revenue/customer-count assumptions, and TypeScript core-runtime drift.
  - Output: Security/privacy checklist, tech-stack alignment report, negative-control scan results, and remediation notes.
  - Validation: Review confirms Overrun remains Rust-first/native-Overrid infrastructure and uses canonical JSON/JSON Schema, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and native service boundaries.

- **10.4 Validate master-plan and downstream handoff alignment.**
  - Design: Confirm SDS #25, the Overrun service plan, master build plan, build-plan crosswalk, Phase 3 plan, Phase 8 plan, Phase 11 plan, Phase 13 plan, queue state, and progress docs link to this plan and preserve the Phase 3 first build point.
  - Output: Updated source-document links, sub-build-plan index entries, progress evidence, queue status, and downstream handoff matrix.
  - Validation: Markdown link checks pass and review confirms no master Phase 0 through Phase 13 ordering change was required.

- **10.5 Validate implementation handoff readiness.**
  - Design: Prepare the handoff for builders by listing required crates/modules, schemas, adapter traits, fixture groups, service clients, local harness scenarios, acceptance tests, and phase gates.
  - Output: Implementation handoff checklist, validation command list, known blockers, dependency owners, and first-execution fixture target.
  - Validation: Handoff review confirms a builder can start Overrun Phase 3 implementation without reading informal agent notes or weakening SDS boundaries.

## Alignment Review

- The sub-build plan keeps Overrun first build work in master Phase 3, matching SDS #25, the service catalog entry, Phase 3 plan, master build plan, and build-plan crosswalk.
- The plan treats Phase 0 through Phase 2 as prerequisites for shared schemas, local fixtures, identity/tenant/key/audit/registry/queue primitives, live Overcell nodes, and runtime capability facts rather than as Overrun's first implementation phase.
- The plan keeps Overcell, Overlease, Overpack, Oversched, Overguard, Overvault, Overstore, Overmeter, and Overwatch authority outside Overrun while defining the refs and evidence Overrun consumes or emits.
- The plan keeps Phase 8 as the native Overstore/Overvault integration gate and forbids silent fallback to founder-local stubs when policy requires native storage or vault paths.
- The plan keeps Phase 11 public-provider execution strictly limited to low-sensitivity, no-secret, capped, deny-by-default public sandbox profiles.
- The plan preserves the master Phase 0 through Phase 13 order and uses later phases only for product integration, system-service hardening, native storage/vault integration, deployment-platform compatibility, federation/public constraints, native app consumption, and governance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service/contracts, native Overrid boundaries, OCI/container execution first, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, customer-count, arbitrary shell, or external-payment assumptions.
