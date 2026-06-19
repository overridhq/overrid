# SUB BUILD PLAN #21 - Overlease

Attached SDS: [docs/sds/execution_scheduling/overlease.md](../sds/execution_scheduling/overlease.md)

## Purpose

This sub-build plan turns SDS #21 into an implementation sequence for Overlease. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overlease is the reservation, concurrency-control, and execution-eligibility authority for the Phase 3 private execution loop. It converts an Oversched placement decision into a short-lived, auditable, verifiable lease that Overcell and Overrun must check before a workload starts. It owns lease creation, renewal, release, cancellation, expiration, stale cleanup, lease proof metadata, and later all-or-none lease sets. It is not the scheduler, runner, policy engine, metering rollup service, billing service, settlement service, provider trust scorer, or queue owner.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #21: Overlease](../sds/execution_scheduling/overlease.md) | Controls Overlease purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering links, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overlease service plan](../service_catalog/execution_scheduling/overlease.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, fixture discipline, local stack stubs, signed envelopes, idempotency, trace ids, integration harnesses, and Rust workspace prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, identity, tenant, key, registry, queue, and audit primitives that Overlease consumes. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies registered Overcell nodes, heartbeat state, capability records, benchmark evidence, resource-instance refs, drain state, and private-swarm visibility used before a lease can be created. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Controls the first build point for Overlease as the reservation boundary between Oversched placement and Overrun execution. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy refs, Oververify trust evidence, challenge/dispute inputs, cancellation/revocation causes, and replayable eligibility decisions consumed by leases. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes Overlease usage-window refs through Overmeter rollups; accounting and settlement decisions stay outside Overlease. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Uses stricter lease, cancellation, renewal, rollback, and evidence requirements for grid-resident system workloads. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies capped public-provider lease windows, sandbox constraints, challenge/fraud refs, and payout-hold signals for low-sensitivity public work. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies retention, migration, incident, compliance, reporting, threat-model, and PIP governance for lease history and proof semantics. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #21 first build work aligned to master Phase 3, with prerequisites from Phases 0 through 2 and later handoffs through policy, metering/accounting, grid-resident operation, public-provider constraints, and governance. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, authenticated HTTP/2, canonical JSON plus JSON Schema, optional Protobuf for compact contracts, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and native Overrid service boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 2, 3, 4, 5, 7, 11, and 13 | Attach SDS #21, freeze Overlease as the reservation authority, preserve first build in Phase 3, and record later hardening gates. |
| 2 | Master Phases 0, 1, and 3 | Build Rust service contracts, lease schemas, API shapes, state machines, fixtures, reason codes, and local harness scenarios. |
| 3 | Master Phases 2 and 3 | Implement single-node lease creation from Oversched placement using registered Overcell node state and atomic resource reservation checks. |
| 4 | Master Phase 3 | Implement lease verification and proof/token semantics before Overcell hands work to Overrun and before Overrun starts side effects. |
| 5 | Master Phase 3 | Implement renewal, expiration, release, stale cleanup, idempotent race handling, and bounded lease-window behavior. |
| 6 | Master Phases 3 and 4 | Implement cancellation, revocation, drain, maintenance, retry, and dead-letter handoffs while preserving policy and queue authority. |
| 7 | Master Phases 3 and 5 | Emit lease lifecycle events and usage-window refs for Overwatch and Overmeter without owning rollups, ORU, Seal Ledger, billing, or payouts. |
| 8 | Master Phases 3, 7, and 9 | Add future-compatible all-or-none lease-set semantics for multi-node, replicated, deployment, and grid-resident workloads after single-node leasing is stable. |
| 9 | Master Phases 4, 6, 7, 10, 11, 12, and 13 | Harden product, SDK, CLI, admin, policy, trust, public-provider, system-service, native-client, and governance handoffs. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, and implementation gates. |

## Tech Stack Guardrails

- Overlease core is a Rust control-plane service using shared contract types, Tokio where async storage, cleanup, and service calls are required, and Axum/Tower/Hyper-style HTTP for service APIs where an HTTP boundary exists.
- Lease requests, lease records, resource reservations, lease tokens, renewals, releases, cancellations, cleanup runs, lease sets, API errors, reason codes, fixtures, and lifecycle events use canonical JSON plus JSON Schema. Compact Protobuf contracts may be added only where the shared contract layer requires them.
- Mutating calls require signed service or actor envelopes, tenant id, provider id where relevant, workload id, queue item id, node id, trace id, idempotency key, policy refs, audit refs, schema version, stable reason codes, and append-only Overwatch events.
- Lease proofs use Overkey signer refs, audience, expiry, verification hashes, and canonical signed proof fields. Phase 3 uses control-plane verification first while preserving a proof shape that can support later offline verification.
- Ed25519 is used for signatures where signatures are required. BLAKE3/content hashes are used for payload hashes, resource-card hashes, proof verification hashes, cleanup checkpoints, and evidence refs.
- Overlease may use an internal Overrid-owned durable lease store or embedded engine behind native service boundaries. PostgreSQL, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFT, pricing, revenue, customer-count, or external payment mechanics must not become the Overlease product boundary.
- Overlease owns reservations, proof metadata, and lease history. Oversched owns placement; Overqueue owns workload queue state; Overcell owns node agent state; Overrun owns execution; Overmeter owns usage rollups; Overguard owns policy finality; Oververify owns trust evidence; accounting services own ORU, Seal Ledger, billing, payouts, holds, and settlement.

## Phase 1: SDS Attachment, Reservation Authority, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #21.**
  - Design: Link this document from the numbered Overlease SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/execution_scheduling/overlease.md`, `docs/service_catalog/execution_scheduling/overlease.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #21 returns both the Overlease SDS and this sub-build plan.

- **1.2 Freeze Overlease as the reservation authority.**
  - Design: Record that Overlease owns lease request validation, resource reservation, lease record lifecycle, proof metadata, renewal, release, cancellation, expiration, stale cleanup, conflict detection, and later lease-set semantics.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overlease does not choose nodes, execute work, mutate queue finality directly, compute usage rollups, bill users, settle providers, override policy, or score trust.

- **1.3 Preserve master Phase 3 as the first build point.**
  - Design: Keep first implementation in master Phase 3 because Overlease depends on Phase 1 queued signed workloads and Phase 2 live Overcell nodes/capability records, then gates Overrun execution.
  - Output: Phase-gate note that Phase 0 through Phase 2 are prerequisites, Phase 3 starts Overlease, Phase 5 consumes usage-window refs, and later phases harden trust, public supply, and governance.
  - Validation: Review proves this plan does not move Overlease into Phase 0, Phase 1, or Phase 2 and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #21 decisions for classed lease windows, v0 atomic reservation dimensions, control-plane verification first with future-compatible signed proof fields, cancellation priority by severity, and all-or-none lease-set semantics.
  - Output: Resolved-decision checklist tied to SDS #21 open-question answers.
  - Validation: Review rejects indefinite leases, unbounded renewals, node-local grace after expiry, cached proof starts during disconnect, implicit partial distributed execution, and cancellation logic that discards secondary causes.

- **1.5 Define runtime authority boundaries.**
  - Design: Create a boundary matrix for Oversched, Overqueue, Overcell, Overrun, Overmeter, Overwatch, Overguard, Oververify, Overclaim, Overkey, Overregistry, Overgate, SDK, CLI, admin UI, public-provider controls, and governance services.
  - Output: Boundary matrix listing read/write authority, command authority, state ownership, evidence refs, policy refs, restricted evidence, and ownership exclusions.
  - Validation: Design review rejects direct scheduler ownership, runner side effects, raw metering rollup mutation, ORU/Seal Ledger mutation, policy override, trust-score mutation, and private workload payload storage inside Overlease.

## Phase 2: Rust Service, Schemas, APIs, Fixtures, And State Machines

### Work Items

- **2.1 Create the Overlease Rust service module.**
  - Design: Add a Rust service module with lease command handlers, durable lease repository boundary, proof signer/verifier boundary, cleanup worker, conflict detector, Overwatch emitter, Overqueue/Oversched/Overcell/Overrun/Overmeter clients, and integration-test hooks.
  - Output: Service crate or module skeleton, repository traits, client interfaces, cleanup worker entry point, error types, reason-code mapping, and test harness entry points.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Overlease remains separate from Oversched, Overcell, Overrun, Overmeter, Overguard, Oververify, and accounting services.

- **2.2 Define lease contract schemas.**
  - Design: Add schemas for `lease_request`, `lease_record`, `resource_reservation`, `lease_token`, `lease_renewal`, `lease_release`, `lease_cancellation`, `lease_set`, `lease_set_member`, and `stale_cleanup_run`.
  - Output: JSON Schema files, Rust types, fixtures, lifecycle enums, reason-code enums, schema-version rules, redaction metadata, and compatibility rules.
  - Validation: Schema tests reject missing lease id, placement decision id, queue item id, workload id, node id, tenant id, provider id, resource reservation, expiry, max renew-until, trace id, idempotency key, policy refs, audit refs, signature refs, and state where required.

- **2.3 Define reservation API contracts.**
  - Design: Implement or specify `POST /leases`, `GET /leases/{lease_id}`, `POST /leases/{lease_id}/verify`, `POST /leases/{lease_id}/renew`, `POST /leases/{lease_id}/release`, `POST /leases/{lease_id}/cancel`, `POST /lease-cleanups`, and `POST /lease-sets`.
  - Output: API request/response schemas, signed-envelope rules, idempotency behavior, pagination/read filters, verification states, cleanup bounds, and Overwatch event payloads.
  - Validation: API tests cover valid calls, duplicate idempotency keys, stale placement decisions, wrong node/workload/tenant scope, missing policy refs, expired commands, unauthorized service accounts, and restricted reads.

- **2.4 Build deterministic local harness fixtures.**
  - Design: Model valid single-node lease, duplicate placement decision, resource conflict, stale node, drain node, cancelled queue item, expired lease, renewal lost in transit, release-cleanup race, wrong-node proof, and lease-set rollback.
  - Output: Valid and invalid fixtures with expected state transitions, API responses, Overwatch events, queue handoff refs, proof verification results, and reason codes.
  - Validation: Local harness scenarios produce deterministic outputs and prove Overlease behavior does not depend on conventional database, queue, object-store, or external payment product boundaries.

- **2.5 Implement lease and lease-set state machines.**
  - Design: Model legal lease transitions across requested, active, renewing, released, cancelled, expired, revoked, conflicted, cleanup_pending, and cleaned, plus lease-set transitions across assembling, committed, rolled_back, and partial_failure.
  - Output: State transition engine, illegal-transition reasons, append-only transition records, terminal-state evidence requirements, and replay fixtures.
  - Validation: State tests reject active-from-missing-placement, renewal-after-terminal, release-before-active, execution-verification-after-expiry, silent terminal mutation, partial lease-set commit, and cleanup that deletes history.

## Phase 3: Single-Node Lease Creation And Atomic Resource Reservation

### Work Items

- **3.1 Validate placement decisions before lease creation.**
  - Design: Accept lease requests only from authorized Oversched placement decisions that bind queue item, workload id, tenant id, provider id, node id, resource card, policy refs, and idempotency key.
  - Output: Placement-decision validator, stale/missing/unauthorized denial reasons, request audit event, and accepted/rejected request state.
  - Validation: Creation tests reject missing placement, stale placement, wrong tenant, wrong node, missing policy refs, incompatible workload class, cancelled queue item, and direct user-created lease attempts.

- **3.2 Check Overcell node state and resource facts.**
  - Design: Require current registered Overcell node state, non-revoked credentials, non-stale heartbeat, non-draining/maintenance state, compatible runtime adapter, capability refs, and source-attributed resource facts before reservation.
  - Output: Node eligibility check, Overcell read boundary, resource-instance compatibility checks, stale evidence markers, and denial reasons.
  - Validation: Eligibility tests prove stale, offline, revoked, maintenance, draining, incompatible, missing capability, missing runtime, and unsupported accelerator states block new leases.

- **3.3 Implement v0 atomic reservation dimensions.**
  - Design: Reserve the dimensions that can cause immediate double booking on one private node: node id, resource-instance refs, CPU share/core allocation, memory bytes, GPU device or partition refs, GPU memory, accelerator/runtime slot, local scratch storage, active lease/concurrency slot, and workload class/runtime adapter.
  - Output: Resource-reservation model, conflict index, compare-and-swap reservation transaction, reservation snapshot, and conflict reason codes.
  - Validation: Concurrency tests prove simultaneous requests cannot double-book CPU, memory, GPU, GPU memory, runtime slot, scratch storage, concurrency slot, or same exclusive resource-instance refs.

- **3.4 Create active leases and proof metadata.**
  - Design: Commit active lease records with issued_at, expires_at, max_renew_until, renewal count, resource reservation, proof metadata, Overwatch refs, queue refs, workload refs, and policy refs.
  - Output: Active lease record, lease proof/token metadata, verification hash, signer refs, audit event, and read model.
  - Validation: Creation tests prove active leases include complete refs, server-side expiry, signed proof fields, no workload secrets, no raw private inputs, and stable verification hash.

- **3.5 Align Overqueue scheduled-to-leased handoff.**
  - Design: Emit state-change refs that let Overqueue move work from scheduled to leased without letting Overlease become the queue source of truth.
  - Output: Queue handoff event, leased-state refs, idempotent duplicate response, and failure handoff reason codes.
  - Validation: Integration tests prove repeated placement decisions return the same lease result, failed lease creation returns the workload to queue policy, and queue state remains owned by Overqueue.

## Phase 4: Lease Verification, Proofs, And Execution Start Gates

### Work Items

- **4.1 Implement control-plane lease verification.**
  - Design: Provide an internal verification endpoint that checks lease id, node id, workload id, queue item id, tenant id, expiry, cancellation/revocation state, resource reservation, audience, signature refs, and policy refs.
  - Output: Verification service, explicit valid/expired/cancelled/revoked/wrong-node/wrong-workload/unknown/conflicted states, and verification audit events.
  - Validation: Verification tests cover valid proofs, expired leases, cancelled leases, revoked leases, wrong node, wrong workload, wrong audience, unknown lease, tampered hash, and missing policy refs.

- **4.2 Preserve signed proof shape for later offline verification.**
  - Design: Keep canonical signed proof fields, signer refs, audience, expiry, payload hash, resource reservation hash, and verification hash even while Phase 3 verification remains control-plane based.
  - Output: Lease-token schema, proof metadata, signer selection boundary, hash canonicalization rules, and proof fixture set.
  - Validation: Compatibility tests prove later offline verification can be added without changing public lease-token shape or Overcell/Overrun request contracts.

- **4.3 Gate Overcell assignment handoff.**
  - Design: Require Overcell to verify an active lease before handing Phase 3 workload assignments to Overrun and to deny starts when verification is missing, stale, expired, revoked, cancelled, wrong-node, or wrong-workload.
  - Output: Overcell verification client contract, denial reason mapping, active lease count refs, and lease-observation event.
  - Validation: Integration tests prove Overcell does not create leases, renew leases, release leases, run assignments without valid leases, or start new attempts from cached unverified proofs.

- **4.4 Gate Overrun side effects before execution.**
  - Design: Require Overrun to verify the active lease before sandbox preparation, input mount, secret access, egress opening, runtime start, or output side effects.
  - Output: Overrun start-gate contract, side-effect boundary checklist, timeout/valid-until handling, and final verification refs.
  - Validation: Execution tests prove Overrun rejects expired, cancelled, revoked, wrong-node, wrong-workload, wrong-resource, and unknown leases before side effects begin.

- **4.5 Handle clock skew and control-plane disconnects defensively.**
  - Design: Treat server-side expiry as authoritative, allow only bounded skew in node-side checks, and permit already verified running attempts to continue only until signed valid_until/expires_at when disconnected.
  - Output: Clock-skew policy, disconnect behavior, valid-until enforcement, renewal-required state, and stop-before-expiry hook.
  - Validation: Tests prove nodes cannot start new attempts from cached proofs during disconnect, running attempts stop before expiry without renewal, and skew cannot extend lease validity silently.

## Phase 5: Renewal, Expiration, Release, And Cleanup

### Work Items

- **5.1 Implement classed renewal rules.**
  - Design: Enforce SDS #21 lease-window decisions: simple command/probe jobs default to five-minute initial leases, 60-second renewal increments, and 15-minute max renew-until; container jobs default to 15-minute initial leases, five-minute renewals, and manifest timeout plus five-minute cleanup grace capped at two hours; Phase 3 model jobs default to 10-minute initial leases, two-minute renewals, and 60-minute max renew-until.
  - Output: Lease profile rules, renewal evaluator, denied-renewal reasons, workload-class caps, and policy-profile override hook.
  - Validation: Renewal tests prove unprofiled workloads cannot exceed class caps, explicit signed policy profiles are required above caps, renewal count/duration limits are enforced, and renewals after terminal states fail.

- **5.2 Implement expiration enforcement.**
  - Design: Expire leases when TTL elapses without renewal or release, mark them invalid for execution, emit events, and trigger queue/runner follow-up through the proper owning services.
  - Output: Expiration worker, invalidation state, Overwatch event, Overqueue handoff refs, and Overrun stop-required signal refs.
  - Validation: Expiry tests prove expired leases cannot verify as valid, new execution starts are blocked, running work receives stop-before-expiry behavior, and history remains replayable.

- **5.3 Implement normal and failure release.**
  - Design: Support release after success, failure, cancellation, timeout, retry, dead-letter, or cleanup with final execution state refs, usage-window refs, releasing actor/service, release reason, and idempotent duplicate behavior.
  - Output: Release API, final-state refs, resource-freeing transaction, usage-window refs, release event, and duplicate-release response.
  - Validation: Release tests prove resources are freed once, duplicate releases return final state, missing execution refs are rejected where required, and releases do not rewrite prior active history.

- **5.4 Implement bounded stale cleanup sweeps.**
  - Design: Run bounded cleanup sweeps that match expired, stale, conflicted, orphaned, or cleanup_pending leases, then emit replayable cleanup evidence without deleting lease history.
  - Output: Cleanup worker, dry-run mode, sweep bounds, matched lease list, cleanup actions, checkpoint hashes, and `overlease.cleanup_completed` events.
  - Validation: Cleanup tests prove sweeps are bounded, idempotent, replayable, safe under retries, safe during release races, and do not remove evidence needed for disputes or usage reconstruction.

- **5.5 Resolve release, renewal, cleanup, and cancellation races.**
  - Design: Use compare-and-swap style state transitions and terminal-state priority rules so concurrent renew/release/cancel/expire/cleanup calls converge to one valid final state with secondary causes preserved.
  - Output: Race-handling rules, transition conflicts, final-state response behavior, secondary-cause evidence refs, and concurrency tests.
  - Validation: Stress tests prove cleanup-vs-release, cancel-vs-renew, expire-vs-release, revoke-vs-release, and duplicate request races do not double-free resources or lose terminal evidence.

## Phase 6: Cancellation, Revocation, Drain, Maintenance, And Retry Handoffs

### Work Items

- **6.1 Implement severity-ordered cancellation and revocation.**
  - Design: Apply SDS #21 priority: policy revocation, trust revocation, credential revocation, incident response, or legal/compliance hold wins with revoked/forced-stop semantics; tenant/workload-owner cancellation is next; node drain/maintenance blocks new leases and requests graceful release; expiry and cleanup are fallback terminal paths.
  - Output: Terminal priority resolver, revoked/cancelled/expired state handling, secondary-cause evidence refs, and cancellation event payloads.
  - Validation: Cancellation tests prove high-severity revocation wins over tenant cancellation, tenant cancellation wins over drain, expiry does not override explicit cancellation, and all observed causes remain attached as evidence refs.

- **6.2 Propagate cancellation to Overcell and Overrun.**
  - Design: Emit cancellation/revocation refs that Overcell and Overrun can consume to stop, drain, release, retry, or report final state through their own authority boundaries.
  - Output: Cancellation propagation contract, expected node behavior, Overrun stop refs, Overcell command refs, and acknowledgement handling.
  - Validation: Integration tests prove cancelled leases stop new starts, running work receives stop/cancel signals, results after cancellation are handled deterministically, and Overlease does not execute local node commands directly.

- **6.3 Integrate drain and maintenance state.**
  - Design: Block new leases on draining or maintenance nodes, preserve active lease handling through policy and runner state, and record graceful release or reschedule expectations.
  - Output: Drain/maintenance eligibility check, blocked-new-lease reasons, graceful-release refs, and operator-visible state.
  - Validation: Tests prove drain/maintenance blocks new reservations, active leases are not silently killed by Overlease alone, and queue/scheduler retry behavior remains outside Overlease.

- **6.4 Hand retry and dead-letter decisions back to owning services.**
  - Design: Return explicit lease-denial, cancellation, revocation, expiry, or conflict reasons to Overqueue and Oversched so they can decide retry, reschedule, dead-letter, or operator escalation.
  - Output: Retry/dead-letter handoff schema, reason-code mapping, workload refs, and audit event refs.
  - Validation: Tests prove Overlease supplies reasons and final lease state but does not own queue retry policy, scheduling candidate selection, or dead-letter finality.

- **6.5 Preserve dispute and incident evidence.**
  - Design: Preserve lease history, cancellation causes, revocation refs, conflict evidence, cleanup records, usage-window refs, and proof verification results for Overclaim, Overwatch, incident response, and compliance.
  - Output: Evidence export/read model, restricted evidence classes, incident refs, dispute refs, and retention placeholders.
  - Validation: Evidence tests prove disputes can reconstruct why a lease existed, why it ended, who acted, which policy/evidence refs applied, and what secondary causes occurred without exposing workload secrets.

## Phase 7: Usage Windows, Events, Observability, And Operations

### Work Items

- **7.1 Emit complete lease lifecycle events.**
  - Design: Emit requested, created, renewed, released, cancelled, expired, revoked, conflict_detected, cleanup_completed, set_committed, and set_rolled_back events with stable state, reason, trace, tenant, workload, node, and policy refs.
  - Output: Overwatch event schemas, event emitter, lifecycle fixture set, and replay metadata.
  - Validation: Event tests prove every state transition emits one append-only event, events contain no workload secrets/private inputs, and event replay reconstructs lease history.

- **7.2 Link usage windows to Overmeter raw events.**
  - Design: Provide lease id, workload id, queue item id, node id, provider id, tenant id, resource reservation, start/end timestamps, idle leased time, active execution time refs, and terminal state refs for Overmeter.
  - Output: Usage-window schema, Overmeter handoff contract, idle/active time markers, and raw usage attribution refs.
  - Validation: Metering tests prove usage can be reconstructed from lease events and Overmeter raw events while Overlease does not compute rollups, balances, invoices, payouts, rates, or settlements.

- **7.3 Build operator observability views.**
  - Design: Expose active leases, nearing-expiry leases, stale leases, conflicts, renewal failures, cancellations, revocations, cleanup sweeps, per-node pressure, per-tenant pressure, per-provider pressure, and resource-dimension pressure.
  - Output: Operator read models, filters, pagination, summaries, reason-code mapping, health checks, and alert thresholds.
  - Validation: Observability tests prove operators can diagnose pressure and failures without seeing workload secrets, private input content, raw private logs, or cross-tenant details outside authority.

- **7.4 Add health checks and maintenance commands.**
  - Design: Check lease store availability, signer/verifier availability, Overwatch emission, Overqueue handoff health, cleanup worker health, clock skew, and storage pressure; provide cleanup dry-runs and signed forced-cancel maintenance commands.
  - Output: Health/readiness endpoints, maintenance command contract, dry-run cleanup output, signed operator action refs, and audit events.
  - Validation: Operational tests prove lease-store outage denies new leases, existing proofs expire naturally, maintenance actions are signed/audited, and forced cancellation preserves evidence.

- **7.5 Preserve migration and history safety.**
  - Design: Treat published lease history as correction-based and never silently edited; preserve token verification refs, usage-window refs, cleanup refs, and schema-version transition records during migrations.
  - Output: Migration contract, historical replay notes, correction record shape, token-ref preservation, and migration validation checklist.
  - Validation: Migration tests prove old lease records remain explainable after schema, proof, policy, signer, cleanup, or resource-reservation changes.

## Phase 8: Atomic Lease Sets And Distributed Workload Readiness

### Work Items

- **8.1 Define all-or-none lease-set contracts.**
  - Design: Add one set id, one workload group id, one manifest/policy version, per-member resource reservations, shared validity window, prepare/commit/rollback phases, bounded commit timeout, rollback evidence, set-level renewal/cancellation/revocation, and Overwatch refs.
  - Output: Lease-set schema, member lease schema, atomicity rules, state machine, and fixture set.
  - Validation: Contract tests reject missing shared validity window, missing rollback semantics, mixed policy versions, partial untracked members, and lease-set creation before all members are eligible.

- **8.2 Implement prepare, commit, and rollback flow.**
  - Design: Reserve each member in prepare state, commit only when all members are prepared, and roll back every prepared member when any member fails before commit.
  - Output: Prepare/commit/rollback transaction flow, set-level conflict detection, rollback event refs, and partial_failure handling.
  - Validation: Concurrency tests prove member conflict rolls back all prepared reservations, successful commit activates every member, and rollback failures move to partial_failure with recovery evidence.

- **8.3 Add set-level renewal, cancellation, and revocation.**
  - Design: Renew, cancel, revoke, expire, and release lease sets as a coordinated group unless a later manifest explicitly defines safe degraded or quorum behavior.
  - Output: Set-level operations, member propagation rules, terminal reason priority, and secondary-cause evidence refs.
  - Validation: Tests prove set renewal preserves shared validity, set cancellation propagates to every member, revocation wins by severity, and partial execution remains denied by default.

- **8.4 Deny distributed or replicated execution without lease sets.**
  - Design: Require all distributed, replicated, multi-node, or workload-group manifests to use lease sets before execution eligibility.
  - Output: Execution eligibility rule, Overpack/Oversched/Overrun integration notes, denial reasons, and fixture cases.
  - Validation: Integration tests prove distributed jobs without committed lease sets are rejected and cannot start as independent uncoordinated single-node leases.

- **8.5 Prepare deployment and grid-resident lease-set handoffs.**
  - Design: Preserve lease-set semantics for later deployment planner, release strategy, package validator, system-service workloads, grid-resident backbone services, and Overpack deployment platform work.
  - Output: Deployment-readiness checklist, system-service lease-set constraints, rollback evidence refs, and operational recovery notes.
  - Validation: Review confirms Phase 8 does not prematurely implement deployment orchestration but gives Phase 7 and Phase 9 services the lease-set contract they will need.

## Phase 9: Product, Policy, Public Provider, And Governance Handoffs

### Work Items

- **9.1 Harden SDK, CLI, and admin consumption.**
  - Design: Provide generated Rust-first SDK and CLI flows plus admin/operator surfaces for create, read, verify, renew, release, cancel, cleanup, lease-set operations, active leases, conflict reasons, expiration status, and usage-window refs.
  - Output: SDK/CLI/admin contract examples, pagination, tenant/provider filters, reason-code mappings, redaction profiles, and troubleshooting flows.
  - Validation: Client tests prove normal users cannot see cross-tenant leases, providers see only authorized provider-scoped summaries, and operators can diagnose lease state without raw workload payloads.

- **9.2 Define product and adapter handoffs.**
  - Design: Document how Docdex, Mcoda, Codali, encrypted RAG, AI gateway, runtime bridge, and first product integrations depend on leases through Overgate/admin APIs, Overqueue state, Overrun verification, Overmeter usage refs, and Overwatch events.
  - Output: Product readiness checklist, adapter fixture contracts, lease-state prerequisites, degraded-state behavior, and integration scenarios.
  - Validation: Product integration tests fail when consumers bypass Overlease verification, treat scheduled work as runnable without active lease, ignore expiry, or read private lease storage directly.

- **9.3 Integrate policy, verification, disputes, and public-provider signals.**
  - Design: Consume Overguard policy refs, Oververify trust evidence, Overclaim dispute refs, public sandbox profile refs, challenge refs, fraud refs, payout-hold refs, workload sensitivity, egress/secrets policy, and eligibility decisions without owning those services.
  - Output: Policy/trust/dispute/public-provider handoff fields, denied-lease reasons, public-provider cap hooks, and evidence refs.
  - Validation: Tests prove public low-sensitivity leases are capped, private/regulated/secret-bearing/system-service work cannot lease unknown public nodes, and trust/policy disputes affect lease eligibility through refs.

- **9.4 Support grid-resident and system-service lease requirements.**
  - Design: Add stricter lease windows, renewal, cancellation, rollback, maintenance, signer, evidence, backup, readiness, and failover requirements for grid-resident system workloads.
  - Output: System-service lease profile, readiness refs, rollback hooks, stricter verification requirements, and operator action audit.
  - Validation: System-service tests prove unknown public nodes cannot receive backbone leases, stale evidence blocks leases, missing failover/backup refs block leases, and signed operator actions are required for forced changes.

- **9.5 Add governance, retention, migration, and incident handoffs.**
  - Design: Preserve lease records, proof metadata, cleanup checkpoints, conflict evidence, cancellation/revocation history, usage-window refs, migration records, incident refs, compliance exports, and PIP-driven schema changes.
  - Output: Retention policy records, migration plan schema, compact evidence summaries, compliance report fields, and historical replay notes.
  - Validation: Governance tests prove historical lease decisions remain explainable after schema, policy, proof, signer, retention, migration, or incident-response changes.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #21`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first service implementation, Tokio, Axum/Tower/Hyper-style HTTP, authenticated HTTP/2, canonical JSON plus JSON Schema, optional Protobuf only for compact shared contracts, signed envelopes, Ed25519, BLAKE3/content hashes, and native Overrid service boundaries.
  - Output: Tech-stack alignment checklist for Overlease.
  - Validation: Scans find no PostgreSQL, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, customer-count, or external payment assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #21 is represented as a Phase 3 execution/scheduling reservation service with Phase 0 through Phase 2 prerequisites and later handoffs through policy, metering/accounting, grid-resident operation, public-provider constraints, and governance.
  - Output: Updated master-plan and crosswalk rows for SDS #21.
  - Validation: Review confirms only per-SDS sub-build indexing and explicit Overlease references changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #21 and the Overlease service plan link back to this sub-build plan and preserve Overlease as the reservation, concurrency-control, and execution-eligibility authority.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare Overlease implementation gates.**
  - Design: Require tests for schema validation, API contracts, placement validation, node-state checks, atomic conflict detection, lease proof metadata, verification, Overcell/Overrun gates, renewal, expiration, release, cleanup, cancellation priority, revocation, drain/maintenance, retry/dead-letter handoff, usage windows, Overwatch events, operator views, lease sets, public-provider caps, system-service leases, governance retention, and documentation links.
  - Output: Final validation checklist for Overlease implementation.
  - Validation: Handoff review confirms Oversched, Overqueue, Overcell, Overrun, Overmeter, Overwatch, Overguard, Oververify, Overclaim, Overkey, Overregistry, SDK, CLI, admin UI, public-provider services, deployment-grid services, and governance services can depend on lease facts without moving their runtime authority into Overlease.

## Alignment Review

- The sub-build plan keeps Overlease's first implementation in master Phase 3, matching SDS #21, the service catalog entry, Phase 3 private execution loop workstreams, the master build plan, and the build-plan crosswalk.
- The plan treats master Phase 0 through Phase 2 as prerequisites for shared contracts, signed envelopes, identity, tenancy, keys, audit, queue state, registered Overcell nodes, capability records, heartbeat state, and benchmark evidence, not as Overlease runtime implementation phases.
- The plan treats master Phase 3 as the point where Overlease converts Oversched placement into active lease records and proof metadata that Overcell and Overrun must verify before work starts.
- The plan treats master Phase 4 as a hardening consumer and input provider: policy, trust, disputes, challenge evidence, and revocation refs influence lease eligibility, but Overlease does not author policy rules or trust scores.
- The plan treats master Phase 5 as a usage-window consumer: Overlease emits lease lifecycle and window refs while Overmeter, ORU, Seal Ledger, Overbill, provider payout, and accounting services own rollups and settlement.
- The plan treats master Phase 7 and Phase 9 as stricter consumers of lease-set, rollback, readiness, and system-service lease semantics, without moving deployment orchestration into Overlease.
- The plan carries forward SDS #21 resolved decisions for classed lease windows, v0 atomic resource reservations, control-plane verification first, cancellation priority by severity, and all-or-none lease sets before distributed/replicated jobs.
- The plan keeps Overlease narrow: no scheduler placement authority, no runner side effects, no queue finality, no trust-score finality, no policy finality, no metering rollups, no billing/payout/ORU ownership, no private workload payload storage, and no external payment mechanics.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #21 is complete when a builder can implement Overlease as the Phase 3 Rust reservation and execution-eligibility authority with schemas, APIs, deterministic fixtures, single-node lease creation, atomic resource-reservation checks, signed proof metadata, control-plane verification, Overcell/Overrun start gates, classed renewal limits, server-authoritative expiry, idempotent release, stale cleanup, severity-ordered cancellation/revocation, drain/maintenance handling, retry/dead-letter handoff refs, usage-window refs, Overwatch lifecycle events, operator observability, health/maintenance commands, migration-safe history, all-or-none lease sets, SDK/CLI/admin/product handoffs, policy/trust/public-provider/system-service/governance hardening, implementation validation gates, and documentation links that preserve the master Phase 0 through Phase 13 order.
