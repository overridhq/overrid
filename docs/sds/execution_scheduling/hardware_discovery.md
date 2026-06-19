SDS #17

# Hardware Discovery SDS

## Purpose

Discover and normalize node hardware, runtime, and locality facts so Overrid can understand what a node appears to have before measured benchmarks and scheduler policy decide how it may be used.

Hardware Discovery is an evidence collection and normalization component. It reports observed inventory and runtime support; it does not prove trust, schedule work, or decide provider eligibility by itself.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [hardware_discovery.md](../../service_catalog/execution_scheduling/hardware_discovery.md) |
| Sub-build plan | [SUB BUILD PLAN #17 - Hardware Discovery](../../build_plan/sub_build_plan_017_hardware_discovery.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md), [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md), [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md), [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md), [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md), [Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md), [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md), [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |

## Service Family

- Family: Execution and scheduling
- Owning layer: Node inventory and capability observation
- Primary data scope: discovery sessions, host facts, device records, runtime support flags, health probes, normalized capability observations, publication refs, and discovery errors
- First build phase from service plan: [Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md)

## Problem Statement

The first Overrid grid starts on founder-provided servers and GPUs, then expands to other providers. The control plane needs a consistent way to know what each node appears to offer: CPU, memory, GPU, disk, network, operating system, runtime support, locality, and degradation signals.

Raw host commands return inconsistent vendor-specific output, and hardware names can be misleading. Hardware Discovery turns raw observations into normalized records while preserving the original evidence and error context for Benchmark Runner, Overregistry, Oververify, and operators.

## Goals

- Detect CPU, memory, GPU, storage, network, OS, kernel, container/runtime, driver, and locality facts.
- Normalize facts into stable capability observations with schema versions and confidence markers.
- Separate observed inventory from measured benchmark results and trust scores.
- Mark missing, unsupported, degraded, or unknown runtime features explicitly.
- Publish capability changes through Overcell into Overregistry without losing history.
- Emit useful discovery error codes and Overwatch events for operators.
- Keep host-sensitive details redacted unless a restricted evidence view is authorized.

## Non-Goals

- Do not run performance benchmarks. Benchmark Runner owns measured capacity.
- Do not decide scheduling placement. Oversched consumes published capability records.
- Do not assign trust scores or provider eligibility. Oververify and policy services own those decisions.
- Do not collect secrets, user files, private keys, process contents, or unrelated personal data.
- Do not mutate host configuration except for explicitly approved prerequisite probes.
- Do not hide uncertainty; unknown and unsupported states are valid outputs.

## Primary Actors And Clients

- Overcell node agent, which runs local probes and submits signed discovery reports.
- Node Installer, which may invoke first discovery during enrollment.
- Overregistry, which stores versioned capability observations.
- Benchmark Runner, which uses discovery facts to choose compatible benchmark suites.
- Oversched and Overguard, which later filter placements from published capability and policy facts.
- Operators inspecting private-swarm inventory and degraded nodes.

## Dependencies

- [Overcell](overcell.md) for node identity, agent runtime, heartbeat, and signed report submission.
- [Node Installer](node_installer.md) for initial bootstrap and local prerequisite checks.
- [Overregistry](../control_plane/overregistry.md) for versioned capability records.
- [Overkey](../control_plane/overkey.md) for node report signature verification.
- [Overwatch](../control_plane/overwatch.md) for discovery lifecycle and error events.
- Host runtime access for read-only operating-system, device, driver, and network probes.

Phase 2 can implement Linux-first probes for founder hardware, but schemas must leave room for additional OSes, accelerators, and sandbox runtimes.

## Owned Responsibilities

Hardware Discovery owns:

- Probe definitions, probe versions, and supported platform matrix.
- Discovery session lifecycle and signed report ingestion.
- Raw-to-normalized mapping for device, runtime, and health facts.
- Error codes for missing permissions, missing drivers, unsupported devices, broken runtimes, or unavailable sensors.
- Change detection between snapshots, including newly found, missing, degraded, or materially changed capabilities.
- Publication of normalized observations through Overcell/Overregistry contracts.

It must preserve raw evidence refs where safe, but downstream services should consume normalized records rather than scraping raw command output.

## Data Model

The first implementation should define:

- `discovery_session`: session id, node id, trigger, requested probes, agent version, state, started/ended timestamps, and trace refs.
- `probe_definition`: probe id, version, platform selector, command/module used, output schema, permissions needed, timeout, and redaction rules.
- `hardware_inventory_snapshot`: node id, snapshot version, discovery session id, observed CPU, memory, GPU, storage, network, OS/kernel, and locality summaries.
- `device_record`: device type, vendor, model, stable device key, visible memory/capacity, driver/runtime binding, health flags, and redacted serial/hash fields.
- `runtime_support_record`: container runtime, sandbox runtime, GPU runtime, model-serving runtime, filesystem features, cgroup/isolation features, and support state.
- `capability_observation`: normalized capability name, value, unit, confidence, source probe, observed_at, previous value ref, and publication ref.
- `health_probe_result`: local load, thermal/power hints, disk health hints, network reachability, sensor availability, and warning flags.
- `discovery_error`: probe id, reason code, severity, retryability, stderr/log ref, and operator hint.

Common envelope fields:

- `id`, `node_id`, `provider_id`, `tenant_id` or system scope.
- `actor_id` or service account.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

The API should expose node-scoped discovery control and read-only inventory:

- `POST /nodes/{node_id}/discovery-sessions`: request discovery using an approved probe set and trigger reason.
- `POST /discovery-sessions/{session_id}/reports`: internal signed Overcell report upload.
- `GET /discovery-sessions/{session_id}`: read session state, errors, and publication refs.
- `GET /nodes/{node_id}/inventory`: read latest normalized hardware inventory for authorized operators/services.
- `GET /nodes/{node_id}/capability-observations`: query versioned capability observations and change history.
- `POST /discovery-sessions/{session_id}/publish`: internal command to publish validated observations to Overregistry.
- `POST /nodes/{node_id}/runtime-flags`: update operator-approved runtime support overrides with audit evidence.

API requirements:

- Discovery requests must be bounded by allowed probe sets and node trust/maintenance state.
- Node reports require node signatures and current enrollment binding.
- Reads must redact host-sensitive fields by role and data classification.
- Repeated discovery triggers must be idempotent when the same node, probe set, and trigger key are used.
- Unknown values must be represented explicitly rather than omitted.

## Event Surface

- `hardware_discovery.session_requested`: discovery requested for a node.
- `hardware_discovery.probe_started`: a probe began on a node.
- `hardware_discovery.probe_failed`: a probe failed with stable reason code.
- `hardware_discovery.report_received`: signed node report accepted.
- `hardware_discovery.normalized`: raw facts converted into schema-versioned observations.
- `hardware_discovery.capability_changed`: material capability change detected.
- `hardware_discovery.published`: capability observations published to Overregistry.
- `hardware_discovery.sensitive_field_redacted`: restricted host detail withheld from a read model.

Events should contain summaries and evidence refs, not raw full host command output.

## Core Workflow

1. Node Installer or Overcell triggers discovery for a newly enrolled or changed node.
2. Hardware Discovery selects approved probes for the node platform and runtime profile.
3. Overcell runs read-only local probes with timeout and permission limits.
4. The node signs and submits a discovery report with raw evidence refs.
5. Hardware Discovery validates the report, applies redaction, and normalizes facts.
6. Capability changes are detected against the previous snapshot.
7. Normalized observations are published to Overregistry.
8. Benchmark Runner uses the records to select safe benchmark suites.
9. Overwatch receives lifecycle, change, and error events.

## State Machine

Discovery session lifecycle:

1. `requested`: an authorized actor or service requested discovery.
2. `probing`: Overcell is running approved probes.
3. `reported`: signed report was received.
4. `normalizing`: raw facts are being redacted and normalized.
5. `published`: normalized observations were written to Overregistry.
6. `unchanged`: session completed without material capability changes.
7. `stale`: latest valid discovery is older than policy allows.
8. `suspect`: report conflicts with prior evidence or benchmark facts.
9. `failed`: session failed with reason and retryability.
10. `cancelled`: request was cancelled before publication.

Only new snapshots and correction records can change history. Published observations are not silently edited.

## Policy And Security

- Probe definitions must be allowlisted and versioned.
- Probes run with least privilege. Any elevated permission must be explicit, logged, and limited to required read-only data.
- Discovery must not read user home directories, application data, secret files, process memory, private keys, or unrelated network contents.
- Host identifiers such as serial numbers, MAC addresses, IPs, or precise physical location must be hashed or redacted unless a restricted operator view is authorized.
- Discovery findings must not grant scheduling eligibility by themselves; policy and benchmark evidence still apply.
- If required probes are missing or ambiguous, publish `unknown` or `unsupported` rather than guessing.
- Runtime overrides require signed operator action and Overwatch evidence.

## Metering And Accounting

Discovery is lightweight but still consumes node and control-plane resources:

- Emit usage facts for discovery duration, probe count, uploaded evidence bytes, normalized record writes, and retry count.
- Attribute usage to the node, provider, trigger type, and system service account.
- Keep discovery overhead visible during bootstrap so founder hardware pressure is understood.
- Do not create per-probe payment behavior; raw usage rolls up through Overmeter when accounting integration exists.
- Distinguish enrollment discovery from operator-requested or challenge-triggered discovery.

## Observability And Operations

- Operators need inventory views grouped by node class, stale state, missing prerequisites, degraded runtimes, and capability changes.
- Health checks should confirm probe registry availability, Overcell report path, signature verification, Overregistry publication, and Overwatch event emission.
- Diagnostics should show probe stderr refs, reason codes, and remediation hints without exposing private host data.
- Discovery schedules should be adjustable for enrollment, heartbeat drift, runtime upgrades, and suspected changes.
- Migration tooling must preserve old snapshot schema versions and conversion provenance.

## Failure Modes And Recovery

- Probe command unavailable: mark feature `unsupported` or `unknown`, emit reason code, and continue other probes.
- Permission denied: record missing permission and remediation hint; do not escalate automatically.
- Node disconnect: keep session retryable until timeout, then fail with audit trail.
- Signature mismatch: reject report and notify Oververify/Overwatch.
- Conflicting inventory: mark snapshot `suspect` and require benchmark or operator confirmation.
- Overregistry unavailable: keep normalized observations pending with retry state.
- Redaction failure: block publication of sensitive fields and emit restricted incident evidence.

## Validation Plan

The service implementation plan lists these requirements:

- CPU-only and GPU nodes produce normalized records.
- Missing runtime dependencies are reported clearly.
- Capability changes update Overregistry without losing history.

Additional SDS-level validation:

- Fixture tests for CPU-only server, GPU node, missing GPU driver, unsupported container runtime, multiple disks, and limited network visibility.
- Contract tests for discovery trigger, signed report upload, inventory read, capability history read, and publication.
- Redaction tests for serial numbers, MAC/IP fields, locality details, and raw command output.
- Change-detection tests proving additions, removals, upgrades, and degraded runtime changes are preserved.
- Replay tests proving a normalized snapshot can be regenerated from raw report, probe versions, and redaction rules.
- Tenant/operator isolation tests for inventory reads.

## Build Breakdown

1. Define the inventory schema, probe schema, runtime-support enum, error codes, and redaction rules.
2. Implement Linux CPU, memory, OS/kernel, storage, network, and basic container runtime probes.
3. Add GPU probe support for the first founder GPU stack.
4. Add signed Overcell report upload and normalization pipeline.
5. Publish normalized observations to Overregistry and lifecycle events to Overwatch.
6. Add stale detection, change detection, and operator diagnostics.
7. Add expanded accelerator/runtime probes as the grid grows.

The Phase 2 target is enough normalized inventory to register one server and one GPU node safely. Public-provider and regulated-host discovery hardening belongs to later phases.

## Handoff And Downstream Use

Hardware Discovery feeds Benchmark Runner, Overregistry, Oversched, Overguard, Oververify, and operator inventory views. Consumers should treat its records as observed facts with confidence, not proof of performance or trust.

## Open Design Questions

- Restricted host inventory may store only fields that are needed for placement, verification, repair, or operator diagnostics, and those fields must be role-gated and redacted from general reads. Useful restricted fields include salted host fingerprint hashes, CPU architecture/model family and core counts, memory totals, GPU vendor/model/memory and driver/runtime versions, disk class/capacity/filesystem and health hints, network interface class and private-swarm reachability hints, OS/kernel/runtime versions, cgroup/isolation feature support, locality at the approved region or zone granularity, and sensor summaries needed to explain degraded hardware. Raw serial numbers, MAC addresses, public or private IPs, hostnames, exact rack/facility/location data, GPU UUIDs, disk WWNs, route tables, mount paths, local usernames, raw command output, stderr, and sensor ids stay in restricted evidence bundles only when there is a clear diagnostic or verification need. Secrets, private keys, tokens, user home-directory contents, process memory, environment variable values, unrelated application data, packet contents, and raw personal files are never collected or persisted by Hardware Discovery.
- Phase 3 workload eligibility fails closed for the affected workload class when required discovery facts are missing, stale, unsigned, unverifiable, or marked `unknown` where the manifest requires certainty. Every schedulable private node needs current signed discovery for node identity binding, OS/kernel compatibility, CPU architecture and baseline cores, memory total, local scratch storage availability, private-swarm network reachability, Overcell/Overrun version and health, required container or sandbox runtime support, cgroup/isolation support, filesystem features, and any storage or secret adapter readiness required by the manifest. GPU probe failure blocks GPU workloads; storage, network, or accelerator probe failure blocks only workloads requesting that class unless baseline node safety is affected. Signature mismatch, redaction failure, identity conflict, missing required isolation, missing egress-control support, or runtime facts contradicted by benchmark evidence suspends all new Phase 3 placement for the node until corrected or challenged. Optional thermal, power, or vendor-detail probes may publish `degraded` or `unknown` without blocking lower-risk CPU-only work when all required baseline facts still pass.
- Refresh cadence is policy-driven and separate from Overcell heartbeat. Heartbeats carry live state and coarse resource pressure, but unchanged private nodes should run a lightweight discovery drift check at least every twenty-four hours and a full inventory refresh at least every seven days. GPU, driver, container runtime, kernel, sandbox, cgroup, network, and storage health subsets should refresh at least every seventy-two hours while the node is eligible for workloads that depend on them, matching the shorter validity window used by volatile benchmark evidence. Discovery runs immediately after install/enrollment, agent upgrade, OS/kernel/driver/runtime change, hardware change, prolonged offline interval, failed health check, operator request, Oververify challenge, or benchmark anomaly. Future system-service candidates require discovery evidence no older than twenty-four hours before placement evaluation, and public/regulated pools may only tighten these windows.
- Stable normalized capability names use a versioned namespaced taxonomy: `resource.<class>.<attribute>` for inventory, `runtime.<kind>.<feature>` for execution support, `locality.<scope>` for placement hints, `health.<class>.<signal>` for degraded-state summaries, and `accelerator.<class>.<attribute>` for future non-GPU devices. Vendor names, device models, versions, units, confidence, source probe, and evidence refs belong in values or metadata, not in the capability name itself. Initial stable names include `resource.cpu.arch`, `resource.cpu.logical_cores`, `resource.cpu.physical_cores`, `resource.cpu.features`, `resource.memory.bytes_total`, `resource.gpu.count`, `resource.gpu.memory_bytes`, `resource.gpu.driver.version`, `resource.storage.scratch.bytes_total`, `resource.storage.filesystem.kind`, `resource.storage.health.state`, `resource.network.private.reachable`, `resource.network.bandwidth_hint`, `runtime.container.oci`, `runtime.sandbox.wasi`, `runtime.gpu.cuda`, `runtime.gpu.rocm`, `runtime.isolation.cgroup_v2`, `runtime.isolation.namespaces`, `runtime.security.seccomp`, `runtime.egress.policy_supported`, `locality.region`, and `locality.zone`. New accelerators extend the taxonomy by class without renaming existing CPU, GPU, storage, or network facts.
- System-service workload eligibility requires a stricter runtime-support flag bundle before Phase 7 placement can proceed. Hardware Discovery must publish current `supported` evidence for the approved Overpack runtime profile, OCI/container execution or an approved system-service runtime, cgroup v2 or equivalent resource isolation, process/user/mount/network namespace isolation or equivalent, seccomp or comparable syscall filtering, read-only root filesystem support with explicit writable data mounts, durable local state paths, private control-plane/Overmesh reachability, clock synchronization, health/readiness probe support, supervised restart compatibility, node audit-spool availability, Overwatch event path availability, and secret-ref mounting through Overvault or a declared no-secret class. Stateful backbone candidates also require storage, backup, restore, and snapshot-readiness facts; secret-bearing candidates require Overvault readiness; network-facing candidates require egress and ingress policy enforcement. Missing, stale, or `unknown` mandatory flags return a system-service denial reason such as `system_runtime_fact_missing` or `node_not_system_eligible` rather than falling back to ordinary workload placement.
