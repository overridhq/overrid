SDS #20

# Overcell SDS

## Purpose

Build the node agent and resource abstraction that lets participant-owned servers, GPUs, storage, network, model, and service capacity join the Overrid grid under controlled identity, heartbeat, capability, assignment, and lifecycle rules.

Overcell is the node-side backbone for Phase 2. It enrolls a node, maintains live state, reports inventory and health, accepts authorized assignments, invokes local runtime components, and preserves audit evidence. It is not the scheduler, trust scorer, billing engine, or arbitrary remote shell.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overcell.md](../../service_catalog/execution_scheduling/overcell.md) |
| Sub-build plan | [SUB BUILD PLAN #20 - Overcell](../../build_plan/sub_build_plan_020_overcell.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md), [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md), [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md) |

## Service Family

- Family: Execution and scheduling
- Owning layer: Node agent, resource state, and local execution handoff
- Primary data scope: node registration, credential binding, heartbeat, resource snapshot, capability report, local assignment view, lease observation, runtime health, drain/maintenance state, and agent command history
- First build phase from service plan: [Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md)

## Problem Statement

Overrid cannot become a distributed grid until real machines can join the system safely. The first founder servers and GPUs need a long-running agent that preserves identity, reports useful capacity, survives restarts, receives assignments, enforces local guardrails, and emits evidence.

Overcell provides that agent boundary. It turns hardware into an accountable grid node without giving the control plane uncontrolled shell access or letting nodes self-declare trust.

## Goals

- Register seed servers and GPU nodes with stable node identities and credential bindings.
- Maintain heartbeat and lifecycle state: live, stale, expired, draining, maintenance, disabled, revoked, and retired.
- Report local resource snapshots, agent version, active leases, load summary, and control-plane contact state.
- Trigger Hardware Discovery and Benchmark Runner flows and publish capability reports through Overregistry.
- Accept only signed, authorized control-plane commands and workload assignments.
- Enforce local assignment checks before invoking Overrun or benchmark tasks.
- Preserve node identity across restarts, upgrades, and reconnects.
- Provide the path for grid-resident system workloads to run on verified resources later.

## Non-Goals

- Do not choose placement. Oversched owns scheduling.
- Do not create leases. Overlease owns reservations.
- Do not execute arbitrary commands from users or operators.
- Do not decide trust scores. Oververify and policy services consume evidence.
- Do not perform billing or provider payout logic.
- Do not store raw private keys in control-plane records.
- Do not hide node health or resource pressure from operators.

## Primary Actors And Clients

- Node Installer, which installs and enrolls the agent.
- Overgate, Overpass, Overtenant, and Overkey for registration, identity, tenant/provider scope, and credentials.
- Hardware Discovery and Benchmark Runner, which run node-local probes and benchmarks through Overcell.
- Overqueue, Oversched, and Overlease, which eventually assign work through controlled reservations.
- Overrun, which executes accepted workloads locally under policy.
- Overwatch, which receives heartbeat, state, command, failure, and health events.
- Operators and provider dashboards inspecting node state.

## Dependencies

- [Node Installer](node_installer.md) for local install, config, service supervision, and diagnostics.
- [Overpass](../control_plane/overpass.md) for node identity and route binding.
- [Overtenant](../control_plane/overtenant.md) for provider/tenant ownership and visibility.
- [Overkey](../control_plane/overkey.md) for node credential enrollment, rotation, and revocation.
- [Overregistry](../control_plane/overregistry.md) for capability records and node manifests.
- [Overwatch](../control_plane/overwatch.md) for event, trace, health, and audit evidence.
- [Hardware Discovery](hardware_discovery.md) and [Benchmark Runner](benchmark_runner.md) for capacity evidence.
- [Overlease](overlease.md) and [Overrun](overrun.md) in Phase 3 for workload execution.

Phase 2 should implement the node agent as a supervised process with narrow command support, not as an ad hoc script.

## Owned Responsibilities

Overcell owns:

- Node-agent process lifecycle, local config loading, credential binding, heartbeat, reconnect, and shutdown behavior.
- Node registration and version reporting.
- Local resource snapshots and health summaries used by the control plane.
- Agent command acceptance, signature verification, command idempotency, and command result reporting.
- Local state transitions for active, stale, draining, maintenance, disabled, revoked, and retired nodes.
- Assignment handoff to Hardware Discovery, Benchmark Runner, and later Overrun only when the command is authorized and compatible.
- Local audit spool for offline periods and replay after reconnect.

Overcell must call control-plane APIs through defined contracts and avoid direct writes to registry, queue, lease, or accounting storage.

## Data Model

The first implementation should define:

- `node_agent_registration`: node id, provider/tenant scope, agent instance id, install session ref, version, platform, endpoint mode, and state.
- `node_credential_binding`: credential id/ref, public key metadata, rotation state, last verified time, revocation state, and Overkey refs.
- `heartbeat_payload`: node id, agent version, monotonic counter, observed resources, active leases count, local load summary, last contact, state, and signature.
- `resource_snapshot`: CPU/memory/GPU/storage/network availability, pressure metrics, runtime health, maintenance flags, and timestamp.
- `capability_report`: discovery snapshot refs, benchmark result refs, manually restricted capabilities, and Overregistry publication refs.
- `agent_command`: command id, command type, issuer, policy refs, payload hash, state, attempt, received/executed timestamps, and result refs.
- `assignment_view`: assigned benchmark/discovery/workload id, lease id if required, expected resource card, timeout, sandbox refs, and current local state.
- `runtime_health`: service supervisor status, disk pressure, clock drift, network reachability, credential validity, queue depth, and local errors.
- `local_audit_spool`: offline event refs, retry state, upload status, and integrity hash.

Common envelope fields:

- `id`, `node_id`, `provider_id`, `tenant_id` or system scope.
- `actor_id` or service account for control-plane commands.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

Overcell exposes node-agent control-plane APIs and local command hooks.

Control-plane facing:

- `POST /nodes/register`: register an enrolled node-agent with credentials and install refs.
- `POST /nodes/{node_id}/heartbeat`: submit signed heartbeat and resource summary.
- `POST /nodes/{node_id}/capability-reports`: submit discovery and benchmark publication refs.
- `GET /nodes/{node_id}/commands`: poll or stream authorized commands for the agent.
- `POST /nodes/{node_id}/commands/{command_id}/ack`: acknowledge accepted or rejected command.
- `POST /nodes/{node_id}/commands/{command_id}/result`: report command result and evidence refs.
- `POST /nodes/{node_id}/state-transitions`: request drain, maintenance, disable, revoke, or retire transition.

Local agent hooks:

- `overcell status`: local status for installer and diagnostics.
- `overcell drain`: local operator request to stop accepting new assignments.
- `overcell diagnostics`: local redacted agent diagnostics.
- Internal adapters for Hardware Discovery, Benchmark Runner, and later Overrun.

API requirements:

- Every node-originated call is signed by the current node credential.
- Every control-plane command is signed by an authorized service account and bound to node id, command id, and expiry.
- Commands are idempotent by command id and payload hash.
- Heartbeat acceptance must not imply scheduler eligibility unless capability, trust, and policy also allow it.
- Reads and operator actions must respect tenant/provider visibility.

## Event Surface

- `overcell.node_registered`: node-agent registration accepted.
- `overcell.heartbeat_received`: heartbeat accepted and node state refreshed.
- `overcell.node_stale`: heartbeat age exceeded stale threshold.
- `overcell.node_expired`: node exceeded expiration threshold.
- `overcell.state_changed`: lifecycle state changed with reason.
- `overcell.command_delivered`: signed command made available to node.
- `overcell.command_rejected`: node rejected command with reason.
- `overcell.command_completed`: command completed with result refs.
- `overcell.capability_reported`: discovery or benchmark evidence refs reported.
- `overcell.agent_error`: local agent error emitted with severity and retryability.

Events should include trace ids and evidence refs but avoid leaking host secrets or workload payloads.

## Core Workflow

1. Node Installer enrolls credentials and starts the supervised Overcell agent.
2. Overcell registers node identity, version, provider/tenant scope, and install refs.
3. Agent sends signed heartbeat with state, load summary, and active lease count.
4. Agent triggers Hardware Discovery and reports normalized capability refs.
5. Benchmark Runner assigns safe benchmark commands; Overcell executes and reports evidence.
6. Phase 3 adds lease-bound workload assignments, which Overcell hands to Overrun after local checks.
7. Agent handles drain, maintenance, reconnect, upgrade, shutdown, and disabled/revoked states.
8. Offline audit spool uploads missed events after reconnect.

## State Machine

Node lifecycle:

1. `installing`: installer is preparing local files and service.
2. `enrolling`: credentials and node id are being established.
3. `registered`: control plane knows the node but it is not yet live.
4. `active`: heartbeats are current and node may be considered by later policy/scheduler filters.
5. `draining`: node is finishing current assignments and refusing new ones.
6. `maintenance`: operator or policy removed node from normal eligibility.
7. `stale`: heartbeat missed the stale threshold.
8. `offline`: heartbeat expired or node disconnected beyond policy window.
9. `suspended`: policy, trust, incident, or provider state blocks eligibility.
10. `revoked`: credential or node identity is revoked; commands are rejected.
11. `retired`: node intentionally removed while preserving history.

Command lifecycle:

1. `offered`: command is available for the node.
2. `accepted`: node verified command signature and compatibility.
3. `running`: local adapter is executing the command.
4. `reporting`: result/evidence is being uploaded.
5. `completed`: command reached terminal success.
6. `failed`: command reached terminal failure with reason.
7. `rejected`: command was invalid, expired, incompatible, or unauthorized.

## Policy And Security

- Node-agent credentials must be enrolled, rotated, and revoked through Overkey.
- Agent accepts only signed commands from authorized service accounts.
- Workload execution requires a valid lease once Overlease exists.
- Local runtime adapters must enforce sandbox, resource, timeout, egress, and secret policies before invoking Overrun.
- Node heartbeat and local resource reports are evidence inputs, not sufficient trust.
- Agent must keep tenant data separated in local paths, cache scopes, logs, and diagnostics.
- Offline audit spool must be tamper-evident and bounded.
- Revoked nodes must stop accepting commands and stop presenting as eligible.
- Local diagnostics must redact credentials, secret refs, workload data, and private host details by policy.

## Metering And Accounting

Overcell emits raw node-side facts that later accounting depends on:

- Heartbeat and availability duration by node, provider, state, and version.
- Command execution duration for discovery, benchmark, and later workload tasks.
- Local resource summaries such as CPU/GPU time, memory pressure, disk/network bytes, and active lease count where available.
- Agent overhead and failed/retry command counts.
- Node drain/maintenance/offline time for provider reliability and reputation.

These are raw Overmeter events, not billing decisions. ORU and Seal Ledger integration happens after Phase 5 accounting services consume signed rollups.

## Observability And Operations

- Operators need views for live/stale/offline/draining/maintenance nodes, heartbeat age, version distribution, capability status, command failures, and last contact.
- Health checks should cover credential validity, control-plane reachability, local service supervisor, disk pressure, clock drift, runtime dependencies, and audit spool backlog.
- Agent logs must use stable reason codes and trace ids.
- Upgrade orchestration must preserve identity and support rollback through Node Installer.
- Drain should be visible and should block new assignments without killing current safe work.
- Incident response must support suspend/revoke commands with clear propagation state.

## Failure Modes And Recovery

- Control plane unreachable: continue local service, mark disconnected locally, spool bounded audit events, and retry with backoff.
- Credential revoked: stop accepting commands, emit local diagnostic, and require re-enrollment or uninstall.
- Heartbeat rejected: inspect reason, backoff, and avoid duplicating registration.
- Command expired or signature invalid: reject and report reason without executing.
- Local runtime missing: mark command failed with prerequisite reason and update capability/health.
- Disk pressure: enter degraded or draining state before local evidence/cache causes host harm.
- Agent crash: supervisor restarts agent; identity persists from local protected config.
- Upgrade failure: Node Installer rolls back and Overcell reports version/state after recovery.

## Validation Plan

The service implementation plan lists these requirements:

- Seed server and GPU node register successfully.
- Restarted node-agent preserves identity.
- Stale and expired node states are detected.

Additional SDS-level validation:

- Registration and heartbeat contract tests with valid, expired, revoked, and wrong-node credentials.
- Host reboot test proving node identity and config persist.
- State transition tests for active, stale, offline, draining, maintenance, suspended, revoked, and retired.
- Command tests for valid assignment, expired command, invalid signature, incompatible payload, duplicate command id, and failed local runtime.
- Hardware Discovery and Benchmark Runner handoff tests.
- Offline audit spool replay tests.
- Tenant/provider visibility tests for node status reads.
- Resource pressure tests proving drain/degraded behavior triggers before host harm.

## Build Breakdown

1. Define node registration, heartbeat, resource snapshot, command, and state-transition schemas.
2. Implement supervised agent skeleton with local config loading and stable identity.
3. Add registration through Overgate/Overkey/Overpass/Overtenant contracts.
4. Add signed heartbeat and stale/expired lifecycle detection.
5. Add Hardware Discovery trigger and capability report publication refs.
6. Add Benchmark Runner command acceptance and result reporting.
7. Add drain, maintenance, reconnect, shutdown, diagnostics, and local audit spool.
8. Add Phase 3 lease-bound Overrun handoff and workload execution checks.
9. Add grid-resident system-service eligibility hooks for Phase 7.

The Phase 2 target is one server and one GPU node in a controlled private swarm. The design must already avoid founder-server assumptions so later backbone services can migrate into grid-resident workloads.

## Handoff And Downstream Use

Overcell supplies live node state and capability evidence to Overregistry, Benchmark Runner, Oversched, Overlease, Overrun, Overmeter, Oververify, and grid-resident backbone planning. Downstream services must consume signed reports and published records rather than trusting ad hoc node claims.

## Open Design Questions

Resolved decisions:

- Heartbeat policy is profile-based and versioned, not hardcoded into the node agent. Phase 2 founder/private nodes should default to a 15-second heartbeat target, `stale` after 60 seconds without an accepted heartbeat, and `offline`/expired after five minutes. Later verified public-provider nodes should default to a 30-second heartbeat target, `stale` after two minutes, and `offline`/expired after ten minutes. Future system-service candidates use the stricter private profile or a tighter class profile, because backbone failover depends on fast detection. Any stale state blocks new scheduling eligibility; active lease behavior is governed by Overlease and Overrun cancellation/expiry policy.
- Phase 3 scheduling hints may use only signed, current, and source-attributed node-local facts: lifecycle state, heartbeat age, active lease count, drain/maintenance flags, coarse CPU load, available memory and pressure, visible GPU count and memory pressure, GPU runtime health, scratch-storage free/pressure state, private-network reachability and coarse bandwidth/latency class, runtime adapter readiness, clock drift, audit-spool backlog, and command failure/backoff state. Hard resource fit still comes from Overpack resource cards, Hardware Discovery observations, Benchmark Runner measured evidence, and Overregistry records. Raw self-reported performance, exact tenant/cache contents, precise provider topology, stale benchmark tiers, unavailable sensors, and unknown runtime facts must downgrade or block candidates rather than silently granting placement.
- Keep one `node_id` per supervised Overcell installation and credential binding. Represent compound hardware through child records in a `node_topology_report`: `resource_instance` records for CPUs, memory pools, GPU devices or partitions, storage pools, network interfaces, runtimes, and optional service-role facets. Multi-GPU, mixed compute/storage/cache/gateway roles, and virtualized slices should be leased by resource-instance refs under the same node identity unless the operator creates a separately enrolled logical node with its own supervisor, credential binding, audit spool, and failure domain. This avoids splitting identity too early while still allowing leases, metering, and policy to target specific device groups or VM/MIG-like partitions.
- Local audit spool retention is bounded by node class and must store signed event envelopes, hashes, sequence checkpoints, and redacted evidence refs rather than secrets, workload payloads, prompts, private files, or raw logs. Phase 2 private seed nodes should retain unsent critical events for at least seven days or until acknowledged upload, capped by a policy default such as 1 GiB or 100,000 events. Later public-provider nodes should default to at least seventy-two hours, capped by a smaller policy default such as 256 MiB. Security, credential, lease, cancellation, usage-window, and system-service events are priority-pinned until upload or explicit operator intervention; when the cap is reached, Overcell enters degraded/draining state and preserves integrity checkpoints plus drop summaries instead of silently discarding critical evidence.
- Before core Overrid services move from founder hardware into the grid, Overcell must expose only nodes that pass the System-Service Workload Class requirements: verified operator and node identity, current non-revoked Overkey credential binding, explicit `system_service_eligible` class/version approval, current heartbeat/discovery/benchmark evidence, strong uptime and reconnect history, stable private-network or Overmesh reachability, security baseline for OS/runtime/isolation, no active dispute/abuse/quarantine markers, healthy Overwatch event path and audit spool, backup/restore capability for stateful roles, package health/readiness/drain/rollback command support, Overguard allow decisions, and replayable placement evidence. Unknown public nodes, public sandbox nodes, stale evidence, missing backup/failover controls, or unsigned operator actions remain hard denials for backbone placement.
