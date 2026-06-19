# SUB BUILD PLAN #20 - Overcell

Attached SDS: [docs/sds/execution_scheduling/overcell.md](../sds/execution_scheduling/overcell.md)

## Purpose

This sub-build plan turns SDS #20 into an implementation sequence for Overcell. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overcell is the supervised Rust node agent and resource-state boundary for participant-owned servers, GPUs, storage, network, model, and service capacity. It enrolls a node, preserves node identity, sends signed heartbeats, reports resource snapshots, accepts only signed authorized control-plane commands, hands bounded assignments to Hardware Discovery, Benchmark Runner, and later Overrun, and preserves tamper-evident local audit evidence. It is not the scheduler, lease creator, trust scorer, billing engine, payout service, provider-admission authority, or arbitrary remote shell.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #20: Overcell](../sds/execution_scheduling/overcell.md) | Controls Overcell purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, state machines, policy/security, metering facts, operations, failure modes, validation, build breakdown, downstream handoff, and resolved open-question decisions. |
| [Overcell service plan](../service_catalog/execution_scheduling/overcell.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, fixture discipline, local stack stubs, signed envelopes, idempotency, trace ids, integration harnesses, and Rust workspace prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, identity, tenant, key, registry, and audit primitives that Overcell consumes but does not own. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Controls the first build point for Overcell as the seed private swarm node agent alongside Node Installer, Hardware Discovery, Benchmark Runner, capability publication, heartbeat, and node lifecycle visibility. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Consumes registered, healthy, capability-backed nodes for lease-bound private execution, Overrun handoff, result state, cancellation, timeout, retry, and raw usage-event reporting. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies policy, verification, challenge, trust, abuse, and eligibility consumers for node evidence without moving policy finality or trust scoring into Overcell. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes Overcell raw availability, command, resource, overhead, drain, maintenance, offline, and execution-duration facts through Overmeter rollups; accounting decisions stay outside Overcell. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies system-service workload class requirements, trusted placement constraints, maintenance, rolling update, rollback, failover, backup/restore, and founder-hardware removal readiness. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies known-organization and purpose-scoped provider expansion that consumes Overcell enrollment, evidence, and node-state records. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider sandbox, anti-abuse, fraud, challenge, payout-hold, and low-sensitivity workload constraints that require stricter Overcell evidence and diagnostics behavior. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies threat-model, incident, compliance, retention, migration, reporting, and governance hardening for node evidence, command history, and lifecycle records. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #20 first build work aligned to master Phase 2, with later handoffs through private execution, trust/verification, raw usage facts, grid-resident operation, public-provider hardening, and governance. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and node agent, Tokio, Axum/Tower/Hyper-style HTTP, authenticated HTTP/2, canonical JSON plus JSON Schema, optional Protobuf for compact contracts, Ed25519 signatures, BLAKE3/content hashes, explicit GPU/runtime adapters, and native Overrid service boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 2, 3, and 7 | Attach SDS #20, freeze Overcell as the supervised node-agent boundary, preserve first build in Phase 2, and record later execution and system-service gates. |
| 2 | Master Phases 0, 1, and 2 | Build Rust agent/service contracts, schemas, APIs, state machines, fixtures, reason codes, and local harness scenarios. |
| 3 | Master Phases 1 and 2 | Implement registration, credential binding, heartbeat, lifecycle state, reconnect, stale/offline detection, and tenant/provider visibility. |
| 4 | Master Phase 2 | Implement resource snapshots, node topology reports, Hardware Discovery trigger, Benchmark Runner command handoff, capability refs, and Overregistry publication boundaries. |
| 5 | Master Phases 2 and 3 | Implement signed command delivery, idempotency, assignment views, compatibility checks, local adapter boundaries, and command-result evidence. |
| 6 | Master Phases 2, 3, and 4 | Implement drain, maintenance, shutdown, offline audit spool, diagnostics, local health, upgrade, rollback, and incident-ready lifecycle controls. |
| 7 | Master Phases 3 and 5 | Add Phase 3 lease-bound Overrun handoff and Phase 5 raw usage-fact emission without making Overcell own leases, scheduling, metering rollups, billing, or settlement. |
| 8 | Master Phases 4 and 7 | Add policy/trust evidence integration and strict system-service eligibility gates for grid-resident backbone placement. |
| 9 | Master Phases 6, 10, 11, 12, and 13 | Harden SDK, CLI, admin, product, trusted-provider, public-provider, native-client, incident, and governance handoffs. |
| 10 | Master Phase 2 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, and implementation gates. |

## Tech Stack Guardrails

- Overcell core is a Rust supervised node-agent process using shared contract types, Tokio where async control-plane calls are required, and Axum/Tower/Hyper-style HTTP for control-plane APIs when an HTTP service boundary is needed.
- Node registration, credential binding, heartbeats, resource snapshots, node topology reports, capability reports, agent commands, assignment views, runtime health, local audit spool records, diagnostics manifests, state transitions, API errors, and fixtures use canonical JSON plus JSON Schema. Compact Protobuf contracts may be added only where the shared contract layer requires them.
- Mutating control-plane calls require signed node or service-account envelopes, idempotency keys, trace ids, node ids, tenant/provider scope, actor/service refs, stable reason codes, schema versions, policy/evidence refs, and append-only Overwatch events.
- Node-originated calls use the current non-revoked node credential. Control-plane commands are signed by authorized service accounts and bound to node id, command id, payload hash, expiry, and policy refs.
- Ed25519 is used for node, service, operator, bundle, and command signatures where signatures are required. BLAKE3/content hashes are used for command payloads, config refs, diagnostics manifests, package refs, evidence checkpoints, and audit-spool integrity.
- GPU/runtime support is explicit and adapter-controlled through the same normalized runtime contracts consumed by Hardware Discovery and Benchmark Runner. Hidden shell execution and opaque host mutation are not allowed.
- Overcell local paths, cache scopes, logs, diagnostics, and audit spool records must keep tenant/provider data separated and redacted by policy.
- Overcell reports raw node-side facts. Oversched owns placement, Overlease owns reservations, Overrun owns workload execution, Overmeter owns rollups, Oververify owns verification/trust consumers, Overguard owns policy finality, and accounting services own ORU, Seal Ledger, billing, payouts, and holds.
- PostgreSQL, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFT, pricing, revenue, customer-count, or external payment mechanics must not become Overcell's product boundary.

## Phase 1: SDS Attachment, Node-Agent Scope, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #20.**
  - Design: Link this document from the numbered Overcell SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/execution_scheduling/overcell.md`, `docs/service_catalog/execution_scheduling/overcell.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #20 returns both the Overcell SDS and this sub-build plan.

- **1.2 Freeze Overcell as the supervised node-agent and resource-state boundary.**
  - Design: Record that Overcell owns node-agent lifecycle, local config loading, credential binding, heartbeat, reconnect, shutdown, resource snapshots, command acceptance, command idempotency, command results, lifecycle state, assignment handoff, and local audit spooling.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overcell does not own scheduling, lease creation, trust scoring, billing, provider payouts, ORU mutation, public-provider admission, raw key storage, or arbitrary remote shell.

- **1.3 Preserve master Phase 2 as the first build point.**
  - Design: Keep first implementation in master Phase 2 because founder servers and GPUs need a supervised node agent before private execution, policy, metering, product integration, or public-provider expansion can consume node facts.
  - Output: Phase-gate note that master Phase 0 and Phase 1 are prerequisites, Phase 2 starts Overcell, Phase 3 consumes lease-bound handoff, and Phase 7 uses stricter system-service eligibility.
  - Validation: Review proves this plan does not move Overcell into Phase 0 or Phase 1 and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #20 decisions for versioned heartbeat profiles, source-attributed scheduling hints, one `node_id` per supervised installation with child resource-instance records, bounded audit-spool retention, and strict system-service eligibility.
  - Output: Resolved-decision checklist tied to SDS #20 open-question answers.
  - Validation: Review rejects hardcoded unversioned heartbeat windows, raw self-reported performance as eligibility, early identity splitting, unbounded audit logs, silent evidence drops, and public or unknown nodes as system-service candidates.

- **1.5 Define runtime authority boundaries.**
  - Design: Create a boundary matrix for Node Installer, Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Hardware Discovery, Benchmark Runner, Overqueue, Oversched, Overlease, Overrun, Overmeter, Overguard, Oververify, Overclaim, Overcache, SDK, CLI, admin UI, provider onboarding, and deployment-grid services.
  - Output: Boundary matrix listing read/write authority, local privilege requirements, command authority, event ownership, evidence refs, policy refs, restricted evidence, and ownership exclusions.
  - Validation: Design review rejects direct scheduler state writes, direct lease mutation, direct trust-score mutation, direct ORU/Seal Ledger mutation, direct payout holds, unrestricted diagnostics, and hidden host command execution inside Overcell.

## Phase 2: Rust Agent, Schemas, APIs, Fixtures, And State Machines

### Work Items

- **2.1 Create the Overcell Rust agent module.**
  - Design: Add a Rust agent/service module with local config loading, supervisor integration, control-plane clients, credential provider boundary, heartbeat loop, command loop, local adapter registry, event emitter, reason-code mapping, and integration-test hooks.
  - Output: Agent crate or module skeleton, client interfaces, adapter traits, config loader, lifecycle controller, error types, and test harness entry points.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Overcell remains separate from Node Installer, Hardware Discovery, Benchmark Runner, Oversched, Overlease, Overrun, Overmeter, and accounting services.

- **2.2 Define Overcell contract schemas.**
  - Design: Add schemas for `node_agent_registration`, `node_credential_binding`, `heartbeat_payload`, `resource_snapshot`, `node_topology_report`, `resource_instance`, `capability_report`, `agent_command`, `assignment_view`, `runtime_health`, `local_audit_spool`, API errors, lifecycle events, and reason codes.
  - Output: JSON Schema files, Rust types, fixtures, lifecycle enums, reason-code enums, redaction metadata, schema-version rules, and compatibility rules.
  - Validation: Schema tests reject missing node id, provider/tenant scope, agent instance id, version, credential ref, state, trace id, idempotency key, signature refs, policy refs, audit refs, timestamp, payload hash, and redaction class where required.

- **2.3 Define control-plane API contracts.**
  - Design: Implement or specify `POST /nodes/register`, `POST /nodes/{node_id}/heartbeat`, `POST /nodes/{node_id}/capability-reports`, `GET /nodes/{node_id}/commands`, `POST /nodes/{node_id}/commands/{command_id}/ack`, `POST /nodes/{node_id}/commands/{command_id}/result`, and `POST /nodes/{node_id}/state-transitions`.
  - Output: API request/response schemas, signed-envelope rules, idempotency behavior, command expiry handling, pagination/streaming semantics, tenant/provider read filters, and Overwatch event payloads.
  - Validation: API tests cover valid calls, duplicate idempotency keys, wrong node id, wrong tenant/provider scope, expired command, missing trace id, revoked credential, unauthorized service account, stale heartbeat, and restricted diagnostics reads.

- **2.4 Build deterministic local harness fixtures.**
  - Design: Model clean seed server, clean GPU node, unsupported runtime, revoked credential, stale heartbeat, offline reconnect, duplicate command, invalid command signature, failed local adapter, benchmark result, audit-spool replay, and host reboot identity preservation.
  - Output: Valid and invalid fixtures with expected state transitions, local spool entries, Overwatch events, API responses, command results, and reason codes.
  - Validation: Local harness scenarios produce deterministic outputs and prove identity, heartbeat, commands, capability refs, diagnostics, and audit replay do not depend on conventional database, queue, object-store, or remote-shell product boundaries.

- **2.5 Implement node and command state machines.**
  - Design: Model legal node transitions across installing, enrolling, registered, active, draining, maintenance, stale, offline, suspended, revoked, and retired, plus command transitions across offered, accepted, running, reporting, completed, failed, and rejected.
  - Output: State transition engine, illegal-transition reasons, append-only transition records, terminal-state evidence requirements, and replay fixtures.
  - Validation: State tests reject active-from-unregistered, active-with-revoked-credential, assignment-on-stale-node, command-run-before-accepted, result-without-running-state, retired-node-reactivation without new enrollment, and silent terminal-state mutation.

## Phase 3: Registration, Credential Binding, Heartbeat, And Lifecycle State

### Work Items

- **3.1 Implement registration through the installed node path.**
  - Design: Accept registration only after Node Installer has enrolled local credentials and produced install refs, then bind node id, provider/tenant scope, agent instance id, version, platform, endpoint mode, and initial state.
  - Output: Registration client/server path, install-ref validation, duplicate-node handling, registered-state record, and `overcell.node_registered` event.
  - Validation: Registration tests reject offline prepared hosts, duplicate node ids, missing install refs, wrong provider scope, revoked enrollment, missing credential binding, stale version, and scheduler-visible capability before heartbeat/discovery evidence.

- **3.2 Implement credential binding, rotation, and revocation behavior.**
  - Design: Use Overkey-managed credential refs, local protected key material, current credential metadata, rotation state, last verification time, revocation state, and hard command rejection after revocation.
  - Output: Credential binding model, current-key selector, rotation placeholder, revocation detector, local diagnostic behavior, and command-denial reasons.
  - Validation: Security tests prove private keys never appear in control-plane records, diagnostics, logs, heartbeat payloads, command results, or audit uploads, and revoked nodes stop accepting commands.

- **3.3 Implement signed heartbeat profiles.**
  - Design: Send signed heartbeat payloads with agent version, monotonic counter, observed resources, active lease count, local load summary, last contact state, lifecycle state, and profile/version refs.
  - Output: Heartbeat scheduler, private seed profile defaults, public-provider profile placeholders, system-service stricter profile hook, stale/offline timers, and heartbeat acceptance events.
  - Validation: Heartbeat tests cover private 15-second target with stale after 60 seconds and offline after five minutes, public-provider later profile placeholders, system-service stricter class hooks, expired signatures, wrong node id, replayed counters, and stale-state placement blocking.

- **3.4 Implement lifecycle visibility and state transitions.**
  - Design: Record active, stale, offline, draining, maintenance, suspended, revoked, and retired state with reason codes, actor refs, policy refs, timestamps, and tenant/provider/operator visibility filters.
  - Output: Lifecycle transition API, read model, Overwatch events, operator summaries, tenant/provider filtered views, and blocked-eligibility reasons.
  - Validation: State tests prove stale and offline block new scheduling eligibility, active lease behavior is delegated to Overlease/Overrun policy, retired nodes preserve history, and tenant/provider reads cannot inspect unrelated nodes.

- **3.5 Implement reconnect and control-plane contact behavior.**
  - Design: Continue local service during control-plane outage, mark disconnected locally, back off retries, avoid duplicate registration, preserve pending events, and resync heartbeat/state after reconnect.
  - Output: Reconnect loop, control-plane contact state, pending upload queue, duplicate-registration guard, and recovered-state event.
  - Validation: Outage tests prove network loss does not create duplicate node identity, rejected heartbeat reasons are handled without busy-looping, and reconnect uploads pending state/evidence in order.

## Phase 4: Resource Snapshots, Topology, Discovery, Benchmark, And Capability Refs

### Work Items

- **4.1 Implement resource snapshot reporting.**
  - Design: Report CPU, memory, GPU, storage, network, runtime health, pressure metrics, maintenance flags, clock drift, audit-spool backlog, active lease count, and timestamp with redaction and source attribution.
  - Output: Snapshot collector, redaction rules, source field, aggregate pressure buckets, and snapshot event refs.
  - Validation: Snapshot tests reject raw tenant/cache contents, exact private file paths, raw workload payloads, raw prompts, secrets, unavailable sensors treated as healthy, and unsigned aggregate reports.

- **4.2 Implement node topology and resource-instance records.**
  - Design: Keep one `node_id` per supervised Overcell installation and represent compound hardware through child `resource_instance` records for CPUs, memory pools, GPU devices or partitions, storage pools, network interfaces, runtimes, and optional service-role facets.
  - Output: `node_topology_report`, resource-instance refs, topology hash, lease-target refs, visibility filters, and failure-domain metadata.
  - Validation: Topology tests prove multi-GPU, mixed compute/storage/cache/gateway roles, and virtualized slices do not require premature identity splitting and can still be targeted by leases, policy, and metering.

- **4.3 Trigger Hardware Discovery and publish capability refs.**
  - Design: Trigger Hardware Discovery through bounded Overcell adapter hooks after registration and heartbeat, then publish discovery snapshot refs and normalized capability refs through Overregistry without Overcell owning normalization.
  - Output: Discovery command adapter, trigger state, capability-report refs, Overregistry publication client boundary, and handoff events.
  - Validation: Integration tests prove Hardware Discovery is not triggered before enrollment/heartbeat, failed discovery marks capability incomplete, Overregistry owns published records, and scheduler eligibility blocks until required evidence exists.

- **4.4 Execute Benchmark Runner commands safely.**
  - Design: Accept Benchmark Runner commands only when signed, current, compatible, and allowed for the node/runtime profile, then execute through bounded local adapter hooks and report evidence refs.
  - Output: Benchmark command adapter, compatibility checks, result refs, failure reasons, and capability-report update path.
  - Validation: Benchmark tests reject expired commands, invalid signatures, unsupported runtime, missing GPU runtime, duplicate command ids, incompatible payloads, and benchmark commands that request arbitrary shell access.

- **4.5 Implement capability visibility and eligibility boundaries.**
  - Design: Expose capability status to operators and allowed tenant/provider readers while preserving that heartbeat and self-reported resources are evidence inputs, not sufficient scheduling or trust eligibility.
  - Output: Capability read model, current/stale evidence markers, blocked-eligibility reasons, operator summaries, and tenant/provider filters.
  - Validation: Review proves raw self-reported performance, stale benchmark tiers, unavailable sensors, unknown runtime facts, and missing source attribution downgrade or block candidates rather than silently granting placement.

## Phase 5: Signed Commands, Assignment Views, Idempotency, And Results

### Work Items

- **5.1 Implement signed command delivery.**
  - Design: Poll or stream authorized commands bound to node id, command id, expiry, issuer, policy refs, payload hash, command type, and service-account signature.
  - Output: Command client, signature verifier, command envelope schema, command availability handling, and `overcell.command_delivered` event.
  - Validation: Command tests reject wrong-node commands, expired commands, invalid signatures, revoked service accounts, missing policy refs, tampered payload hash, and unknown command types.

- **5.2 Implement command idempotency and replay protection.**
  - Design: Track command id, payload hash, attempt, accepted/rejected state, execution state, result refs, and duplicate behavior so retries do not execute twice unless explicitly safe.
  - Output: Command ledger, idempotency store, duplicate detector, replay reasons, and stable result replay path.
  - Validation: Tests prove duplicate command ids with same payload return prior result, duplicate ids with different payload reject, reconnect does not double-run commands, and command expiry blocks late execution.

- **5.3 Implement assignment views and local compatibility checks.**
  - Design: Model assigned discovery, benchmark, and workload refs with expected resource card, lease id if required, timeout, sandbox refs, data sensitivity, local state, and compatibility result.
  - Output: `assignment_view` record, compatibility matrix, accepted/rejected command reasons, and local preflight result refs.
  - Validation: Assignment tests reject mismatched resource cards, missing required runtime, maintenance/drain state, stale heartbeat state, missing lease for Phase 3 workloads, insufficient local pressure headroom, and incompatible policy refs.

- **5.4 Implement bounded local adapter execution.**
  - Design: Invoke Hardware Discovery, Benchmark Runner, and later Overrun only through declared adapter interfaces with sandbox, resource, timeout, egress, secret, and evidence rules.
  - Output: Adapter registry, adapter permission model, timeout handling, local execution state, and adapter result refs.
  - Validation: Adapter tests prove Overcell cannot execute arbitrary user/operator commands, cannot mount secrets outside declared policy, cannot bypass Overrun sandbox checks, and reports missing runtime as a command failure.

- **5.5 Implement command result and evidence reporting.**
  - Design: Report accepted, rejected, running, failed, completed, and evidence-upload states with trace id, reason code, evidence refs, result refs, local runtime summary, and upload state.
  - Output: Command result API client, evidence refs, retryable upload behavior, terminal-state records, and Overwatch command events.
  - Validation: Result tests cover success, retryable failure, final failure, rejected command, invalid signature, incompatible payload, failed local runtime, offline result upload, and no raw workload payload leakage.

## Phase 6: Lifecycle Controls, Audit Spool, Diagnostics, Upgrade, And Recovery

### Work Items

- **6.1 Implement drain, maintenance, shutdown, and restart controls.**
  - Design: Support local and control-plane drain, maintenance, shutdown, restart, disabled, revoked, and retired behavior with active-assignment awareness and clear authority boundaries.
  - Output: Lifecycle command handlers, drain state, maintenance state, forced-stop policy, active-assignment summary, and state-change events.
  - Validation: Lifecycle tests prove drain blocks new assignments without killing current safe work, maintenance removes eligibility, forced local stop requires explicit local authority, and scheduler/lease authority remains outside Overcell.

- **6.2 Implement bounded tamper-evident local audit spool.**
  - Design: Store signed event envelopes, hashes, sequence checkpoints, redacted evidence refs, retry state, upload state, and integrity summaries for offline periods under class-specific retention caps.
  - Output: Audit-spool schema, sequence checkpointing, upload/retry loop, cap policy, priority-pinned event classes, degraded/draining-on-cap behavior, and drop summaries.
  - Validation: Spool tests prove private seed nodes can retain critical unsent events for policy defaults, public-provider caps are smaller, security/credential/lease/cancellation/usage/system-service events are priority-pinned, and caps produce degraded/draining state instead of silent critical-evidence loss.

- **6.3 Implement diagnostics with redaction boundaries.**
  - Design: Provide local diagnostics for service supervisor state, credential validity, control-plane reachability, disk pressure, clock drift, runtime dependencies, audit-spool backlog, command failures, config hashes, and redacted host facts.
  - Output: Diagnostics manifest, public/provider/operator redaction profiles, restricted evidence refs, local command output, and read API requirements.
  - Validation: Redaction tests prove diagnostics exclude tokens, private keys, secret refs, workload data, prompts, private files, raw logs, unrelated host details, and tenant-private cache/log paths outside authority.

- **6.4 Implement runtime health and operator views.**
  - Design: Report runtime health for supervisor status, disk pressure, clock drift, network reachability, credential validity, queue depth, runtime adapters, GPU runtime health, local errors, version distribution, and last contact.
  - Output: Runtime-health record, health read model, operator dashboard requirements, state severity mapping, and event refs.
  - Validation: Health tests prove operators can distinguish live, stale, offline, draining, maintenance, disabled, revoked, degraded, incompatible runtime, and audit-spool-blocked states without reading sensitive node payloads.

- **6.5 Integrate upgrade and rollback through Node Installer.**
  - Design: Preserve identity across upgrades, coordinate drain before restart when needed, apply signed package/config changes through Node Installer, report version/state after recovery, and support rollback on failed upgrade.
  - Output: Upgrade handoff contract, version-reporting heartbeat fields, rollback event refs, config migration hook, and recovery state.
  - Validation: Upgrade tests prove identity persists, rollback restores prior working agent, failed upgrade reports degraded state, revoked credentials block restart eligibility, and no duplicate node records are created.

## Phase 7: Lease-Bound Execution Handoff And Raw Usage Facts

### Work Items

- **7.1 Validate Overlease before workload handoff.**
  - Design: Require a valid lease id, resource reservation, expiration, node id, queue/workload ref, and current policy refs before Overcell invokes Overrun for Phase 3 workloads.
  - Output: Lease validation client boundary, missing/expired/stale lease denial reasons, active lease count reporting, and lease-observation events.
  - Validation: Execution tests prove workload assignments without valid lease are rejected, expired leases stop new execution, and lease creation/renewal/release remains owned by Overlease.

- **7.2 Hand off to Overrun under local checks.**
  - Design: Pass workload refs, package refs, sandbox refs, input/output refs, resource card, egress policy, secret refs, timeout, trace id, and evidence refs to Overrun only after local compatibility and lease checks pass.
  - Output: Overrun adapter contract, local preflight summary, sandbox readiness refs, timeout/cancellation hooks, and Overrun result refs.
  - Validation: Handoff tests prove Overcell does not verify packages as final authority, does not own sandbox internals, does not bypass egress/secret policy, and rejects missing or incompatible runtime adapters.

- **7.3 Report active assignment and workload state.**
  - Design: Track local assignment state across accepted, running, reporting, completed, failed, rejected, cancelled, timed out, and disconnected states without becoming the queue or scheduler source of truth.
  - Output: Assignment-state read model, active lease count, local-progress events, cancellation propagation refs, and terminal result summaries.
  - Validation: Tests prove queue state remains owned by Overqueue, placement remains owned by Oversched, lease state remains owned by Overlease, execution finality remains owned by Overrun/control-plane result state, and Overcell reports local facts consistently.

- **7.4 Emit raw usage facts for Overmeter.**
  - Design: Emit raw node-side facts for availability duration, heartbeat state, command execution duration, discovery/benchmark/workload runtime duration, CPU/GPU time where available, memory pressure, disk/network bytes, active lease count, agent overhead, failures, retries, drain, maintenance, offline time, and audit-spool upload.
  - Output: Overmeter raw event contract, attribution fields, local usage emitter, evidence refs, and backpressure behavior.
  - Validation: Metering tests prove Overcell emits facts but does not calculate ORU balances, Seal Ledger transitions, invoices, payouts, pricing, provider earnings, or billing decisions.

- **7.5 Preserve failure, retry, cancellation, and dead-letter evidence.**
  - Design: Capture local evidence for disconnects, lease expiry, command cancellation, timeout, runtime missing, sandbox preparation failure, result upload failure, and terminal failure with stable reason codes.
  - Output: Failure evidence refs, retryable/final reason mapping, cancellation report, timeout report, and dead-letter handoff notes.
  - Validation: Failure tests prove controlled failures are explainable from stored facts, local retry does not duplicate work, cancellation is propagated, and dead-letter decisions remain outside Overcell.

## Phase 8: Policy, Trust Evidence, And System-Service Eligibility

### Work Items

- **8.1 Integrate policy prechecks without owning policy finality.**
  - Design: Apply Overguard allow/deny refs for command acceptance, workload class, data sensitivity, tenant quota, package trust, sandboxing, egress, secret access, provider eligibility, cache scope, and budget precheck.
  - Output: Policy-ref fields, local deny behavior, stale-policy denial reasons, and policy-decision event refs.
  - Validation: Policy tests prove Overcell denies missing/stale policy refs but does not author policy rules or override Overguard decisions.

- **8.2 Provide trust and verification evidence to consumers.**
  - Design: Expose signed heartbeat, discovery, benchmark, command, failure, uptime, reconnect, audit-spool, diagnostics, and capability refs for Oververify, Overwatch, Overguard, Oversched, Overlease, and incident workflows.
  - Output: Evidence export/read model, trust-consumer refs, redacted summaries, replay metadata, and restricted evidence access rules.
  - Validation: Trust tests prove Overcell evidence influences consumers but Overcell does not compute final trust scores, provider reputation, anti-Sybil classification, or challenge outcomes.

- **8.3 Implement system-service eligibility reporting.**
  - Design: Report explicit `system_service_eligible` class/version approval only when verified identity, non-revoked credentials, current heartbeat/discovery/benchmark evidence, uptime/reconnect history, private-network or Overmesh reachability, security baseline, no active dispute/abuse/quarantine markers, healthy Overwatch path, audit spool health, backup/restore capability, command support, Overguard allow refs, and replayable placement evidence exist.
  - Output: System-service eligibility schema, denial reasons, eligibility event, operator read model, and grid-resident readiness checklist.
  - Validation: Eligibility tests hard-deny unknown public nodes, public sandbox nodes, stale evidence, missing backup/failover controls, missing Overguard refs, unsigned operator actions, revoked credentials, unhealthy audit path, and active dispute markers.

- **8.4 Prepare grid-resident operational hooks.**
  - Design: Support health/readiness, drain, maintenance, rollback, package readiness, command readiness, audit-spool status, backup-capability refs, and controlled update hooks needed before backbone workloads move into the grid.
  - Output: Phase 7 readiness contract, system-service command set, readiness fields, rollback refs, and operator action audit.
  - Validation: Grid-readiness review confirms Overcell can prove candidate node state without changing Overgate, Overregistry, Overqueue, Oversched, Overmeter, Overwatch, Overguard, Overpass, or supporting-store contracts.

- **8.5 Add incident response and quarantine hooks.**
  - Design: Support suspend, revoke, quarantine, drain, restricted diagnostics, evidence export, and command blocking when incidents, abuse, credential problems, runtime compromise, or impossible evidence appear.
  - Output: Incident command handling, quarantine state, restricted evidence refs, notification events, and recovery conditions.
  - Validation: Incident tests prove compromised, revoked, suspicious, quarantined, or under-investigation nodes stop accepting new commands and stop presenting as eligible until explicit authority clears them.

## Phase 9: Product, Provider, Public Pool, Admin, And Governance Handoffs

### Work Items

- **9.1 Harden SDK, CLI, and admin read models.**
  - Design: Provide generated Rust-first SDK and CLI flows plus admin/operator surfaces for node registration, heartbeat age, lifecycle state, capability status, command history, diagnostics, audit-spool backlog, version distribution, active leases, and system-service eligibility.
  - Output: SDK/CLI/admin contract examples, pagination, tenant/provider filters, reason-code mappings, redaction profiles, and troubleshooting flows.
  - Validation: Client tests prove normal users cannot see cross-tenant node metadata, providers cannot see restricted operator evidence, and operators can diagnose node state without raw secrets or payloads.

- **9.2 Define product and adapter consumption boundaries.**
  - Design: Document how Docdex, Mcoda, Codali, AI gateway, encrypted RAG, runtime bridge, SDK, CLI, and admin surfaces consume Overcell state through Overregistry, Overwatch, Overgate/admin APIs, Overmeter, and policy refs rather than private node-agent tables.
  - Output: Product readiness checklist, adapter fixture contracts, node-state prerequisites, degraded-state behavior, and integration scenarios.
  - Validation: Product integration tests fail when consumers bypass Overregistry/Overcell status, treat stale/offline nodes as eligible, ignore degraded health, or depend on local private Overcell state.

- **9.3 Support trusted-provider and public-interest expansion.**
  - Design: Preserve provider refs, purpose/grant refs, capability evidence, channel policy, diagnostics profiles, verification refs, and node-state history for known organizations and public-interest pools.
  - Output: Trusted-provider readiness records, grant-aware node summaries, purpose-scope refs, provider evidence refs, and reporting hooks.
  - Validation: Trusted-provider tests prove capacity remains policy-bound, purpose-scoped, evidence-backed, and explainable without exposing raw provider host details or granting system-service eligibility by default.

- **9.4 Harden public-provider node controls.**
  - Design: Add stricter public-provider rules for stable-channel defaults, no prepared/offline scheduler visibility, random verification, fraud refs, challenge refs, payout-hold refs, sandbox-class limits, diagnostic minimization, and fast revocation.
  - Output: Public-provider Overcell hardening checklist, sandbox compatibility matrix, anti-abuse handoff refs, fraud/payout-hold refs, and public diagnostics profile.
  - Validation: Public-provider tests prove unknown nodes cannot run private, regulated, secret-bearing, or system-service workloads and suspicious node evidence reduces eligibility before payout-sensitive flows.

- **9.5 Add governance, retention, and migration handoffs.**
  - Design: Preserve node evidence, command history, lifecycle tombstones, diagnostics summaries, audit-spool checkpoints, schema-version transitions, and migration records for compliance, incident, threat-model, reporting, and PIP-driven changes.
  - Output: Retention policy records, migration plan schema, compact evidence summaries, governance report fields, and historical replay notes.
  - Validation: Governance tests prove historical node decisions remain explainable after schema, policy, key, heartbeat-profile, eligibility, retention, or diagnostics-redaction upgrades.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #20`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first node agent, Tokio, Axum/Tower/Hyper-style HTTP, authenticated HTTP/2, canonical JSON plus JSON Schema, optional Protobuf only for compact shared contracts, signed envelopes, Ed25519, BLAKE3/content hashes, explicit GPU/runtime adapters, Rust supervised process behavior, and native Overrid boundaries.
  - Output: Tech-stack alignment checklist for Overcell.
  - Validation: Scans find no PostgreSQL, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, customer-count, or external payment assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #20 is represented as a Phase 2 execution/scheduling node-agent service with Phase 3 execution handoff, Phase 5 raw usage-fact emission, Phase 7 grid-resident eligibility hardening, public-provider hardening, and governance.
  - Output: Updated master-plan and crosswalk rows for SDS #20.
  - Validation: Review confirms only per-SDS sub-build indexing and explicit Overcell references changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #20 and the Overcell service plan link back to this sub-build plan and preserve Overcell as the supervised node-agent and resource-state boundary.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare Overcell implementation gates.**
  - Design: Require tests for registration, credential binding, heartbeat profiles, lifecycle state, topology, resource snapshots, discovery trigger, benchmark handoff, command signature/idempotency, assignment compatibility, local adapter boundaries, audit spool, diagnostics, drain, maintenance, upgrade/rollback, Overrun handoff, raw usage facts, policy refs, trust evidence, system-service eligibility, public-provider hardening, governance retention, and documentation links.
  - Output: Final validation checklist for Overcell implementation.
  - Validation: Handoff review confirms Node Installer, Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Hardware Discovery, Benchmark Runner, Overqueue, Oversched, Overlease, Overrun, Overmeter, Overguard, Oververify, Overclaim, Overcache, SDK, CLI, admin UI, provider services, deployment-grid services, and governance services can depend on Overcell facts without moving their runtime authority into Overcell.

## Alignment Review

- The sub-build plan keeps Overcell's first implementation in master Phase 2, matching SDS #20, the service catalog entry, Phase 2 seed-private-swarm workstreams, the master build plan, and the build-plan crosswalk.
- The plan treats master Phase 0 and Phase 1 as prerequisites for shared contracts, local harnesses, signed envelopes, identity, tenancy, keys, registry, audit, queue, and ingress, not as Overcell runtime implementation phases.
- The plan treats master Phase 3 as the point where Overcell hands lease-bound workload assignments to Overrun after local compatibility checks; Overcell still does not own scheduling, reservations, package verification finality, sandbox internals, result finality, or retry/dead-letter policy.
- The plan treats master Phase 5 as a raw usage-fact consumer: Overcell emits heartbeat, availability, command, resource, failure, drain, maintenance, offline, and execution-duration facts while Overmeter, ORU, Seal Ledger, Overbill, provider payout, and accounting services own rollups and settlement.
- The plan treats master Phase 7 as the point where Overcell evidence supports strict system-service eligibility for grid-resident backbone placement, never unknown public nodes or stale evidence.
- The plan carries forward SDS #20 resolved decisions for heartbeat profiles, source-attributed scheduling hints, one supervised-installation node identity with child resource-instance records, bounded tamper-evident audit-spool retention, and strict system-service eligibility.
- The plan keeps Overcell narrow: no arbitrary remote shell, no scheduler placement authority, no lease creation, no trust-score finality, no policy finality, no billing/payout/ORU ownership, no raw private key storage, no hidden tenant data leakage, and no public-provider admission authority.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #20 is complete when a builder can implement Overcell as the Phase 2 Rust supervised node-agent and resource-state boundary with node registration, credential binding, signed heartbeat profiles, lifecycle state, resource snapshots, topology/resource-instance reports, Hardware Discovery trigger, Benchmark Runner handoff, Overregistry capability refs, signed command delivery, command idempotency, assignment compatibility checks, bounded local adapters, command result evidence, drain/maintenance/offline handling, bounded tamper-evident audit spool, redacted diagnostics, upgrade/rollback coordination through Node Installer, Phase 3 lease-bound Overrun handoff, Phase 5 raw usage facts, Overguard policy refs, Oververify/Overwatch evidence handoffs, strict Phase 7 system-service eligibility, product/admin/provider/public-pool/governance handoffs, implementation validation gates, and documentation links that preserve the master Phase 0 through Phase 13 order.
