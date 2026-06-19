SDS #16

# Benchmark Runner SDS

## Purpose

Measure useful node capacity with controlled, repeatable benchmark jobs so scheduling, verification, and provider reputation can cite evidence instead of trusting provider claims.

Benchmark Runner is the evidence-producing benchmark coordinator for Phase 2 private-swarm bootstrap and the later verification loop. It defines benchmark suites, triggers runs, receives signed samples from Overcell-managed nodes, normalizes results, publishes capability evidence, and preserves enough context for Oververify challenges.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [benchmark_runner.md](../../service_catalog/execution_scheduling/benchmark_runner.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md), [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md) |

## Service Family

- Family: Execution and scheduling
- Owning layer: Evidence-backed resource measurement
- Primary data scope: benchmark suites, benchmark runs, raw samples, normalized results, signatures, anomaly markers, capability evidence, and invalidation records
- First build phase from service plan: [Phase 2: Seed Private Swarm](../../build_plan/phase_02_seed_private_swarm.md)

## Problem Statement

Node providers can claim CPU, GPU, disk, network, locality, or runtime capability that does not match useful performance. Hardware names alone are also not enough: a GPU with a broken driver, thermal throttling, bad storage, or unstable networking can look eligible while failing workloads.

Overrid needs a benchmark system that is safe for seed hardware, hard to fake, and specific enough for scheduler tiers. Benchmark Runner fills that role by producing signed evidence records that Overregistry, Oversched, Oververify, and provider reputation can consume.

## Goals

- Produce measured CPU, GPU, disk, network, cold-start, and sustained-reliability evidence for Phase 2 seed nodes.
- Keep claimed inventory from Hardware Discovery separate from measured benchmark evidence.
- Normalize raw benchmark output into versioned capability tiers with units, confidence, and run context.
- Sign benchmark evidence with node and coordinator credentials so results can be challenged later.
- Publish capability deltas to Overregistry without overwriting historical evidence.
- Support scheduled reruns after agent upgrades, driver changes, hardware changes, trust challenges, or suspicious results.
- Keep benchmarks bounded so they do not harm node availability or disguise production workload execution.

## Non-Goals

- Do not schedule normal user workloads. Oversched owns placement.
- Do not score trust by itself. Oververify and reputation services consume benchmark evidence.
- Do not replace Hardware Discovery inventory. Discovery reports observed hardware; Benchmark Runner measures useful behavior.
- Do not execute arbitrary customer code under a benchmark label.
- Do not make provider payout, pricing, or business-volume projections.
- Do not treat one benchmark result as permanent truth; evidence can expire, be superseded, or be invalidated.

## Primary Actors And Clients

- Overcell node agents executing approved benchmark suites.
- Hardware Discovery, which provides claimed inventory and runtime prerequisites.
- Overregistry, which stores versioned capability and benchmark evidence records.
- Oververify and Challenge Task Service, which request validation reruns and inspect anomalies.
- Oversched, which reads measured tiers through Overregistry rather than direct benchmark storage.
- Operators and admin UI readers inspecting run history, failure reasons, and node readiness.

## Dependencies

- [Hardware Discovery](hardware_discovery.md) for normalized inventory, driver/runtime facts, and benchmark prerequisites.
- [Overcell](overcell.md) for signed node identity, agent command execution, heartbeat, and local safety checks.
- [Overkey](../control_plane/overkey.md) for node and coordinator signing credentials.
- [Overregistry](../control_plane/overregistry.md) for immutable benchmark evidence and capability record publication.
- [Overwatch](../control_plane/overwatch.md) for run lifecycle, anomaly, and invalidation events.
- [Oververify](../trust_policy_verification/oververify.md) and [Challenge Task Service](../trust_policy_verification/challenge_task_service.md) for later challenge-triggered reruns.

Early Phase 2 can use a narrow in-process coordinator and simple node-agent command channel, but the records and event contracts must already match the final service boundary.

## Owned Responsibilities

Benchmark Runner owns:

- Benchmark suite definitions, versions, safety limits, and deprecation state.
- Benchmark run lifecycle from trigger through assignment, sampling, normalization, signing, publication, and invalidation.
- Raw sample ingestion rules, unit normalization, confidence scoring, and anomaly detection.
- Evidence bundles that link claimed hardware, runtime environment, samples, normalized results, signatures, and Overwatch events.
- Run scheduling rules for initial enrollment, periodic refresh, runtime change, operator request, and Oververify challenge.
- Safety guards that cap duration, resource load, network targets, disk writes, and GPU memory pressure.

Benchmark Runner must not write directly into scheduler private state. It publishes measured evidence to Overregistry and emits events for consumers.

## Data Model

The first implementation should define these logical records:

- `benchmark_suite`: suite id, semantic version, target resource class, workload class, benchmark steps, runner artifact ref, expected output schema, safety limits, timeout, sampling policy, and deprecation state.
- `benchmark_run`: run id, node id, provider id, suite id/version, trigger type, requester, state, attempt number, idempotency key, trace id, started and ended timestamps, and policy/evidence refs.
- `benchmark_environment_snapshot`: discovery snapshot ref, agent version, kernel/OS, driver/runtime versions, thermal/power hints, container/sandbox info, network peer set, and locality tag.
- `benchmark_sample`: raw metric name, value, unit, duration, sample index, sensor/source, stderr/log ref, and capture timestamp.
- `benchmark_result`: normalized score, unit, confidence, resource tier, stability marker, capability delta, scheduler-facing summary, and expiry timestamp.
- `benchmark_evidence_bundle`: content hash, node signature, coordinator signature, suite hash, sample hashes, result hash, Overwatch event refs, and Overregistry publication refs.
- `benchmark_anomaly`: anomaly type, severity, matched rule, expected range, observed value, evidence refs, and required follow-up.
- `benchmark_invalidation`: invalidated run/result ids, reason code, actor/service account, replacement run id if any, and audit refs.

Common envelope fields on mutable records:

- `id`, `node_id`, `provider_id`, `tenant_id` or system scope.
- `actor_id` or service account.
- `trace_id` and `idempotency_key` for mutating commands.
- `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

Phase 2 should keep the API narrow and explicit:

- `POST /benchmark-suites`: register a new suite version. Operator/system-service only.
- `GET /benchmark-suites`: list active and deprecated suites by resource class and compatibility.
- `POST /benchmark-runs`: request a benchmark for a node, suite, and trigger reason.
- `GET /benchmark-runs/{run_id}`: read run state, evidence refs, failure reasons, and normalized result summary.
- `POST /benchmark-runs/{run_id}/samples`: internal signed node-agent upload for bounded raw samples.
- `POST /benchmark-runs/{run_id}/complete`: internal coordinator command to normalize, sign, and publish a terminal result.
- `POST /benchmark-runs/{run_id}/invalidate`: invalidate or supersede a result with a reason and evidence link.
- `GET /nodes/{node_id}/benchmark-results`: read latest valid measured results for a node.

API requirements:

- Mutating calls require actor identity, tenant/system scope, idempotency key, trace id, and service-account authorization.
- Node-originated sample uploads must be signed by the node credential and bound to an assigned run.
- Suite definitions must be immutable after activation; changes create a new version.
- Reads must distinguish raw samples, normalized public summaries, and operator-only evidence.
- Batch benchmark triggers must be bounded and rate-limited to protect seed hardware.

## Event Surface

- `benchmark_runner.suite_registered`: suite version accepted and hash recorded.
- `benchmark_runner.run_requested`: benchmark requested for a node and suite.
- `benchmark_runner.run_assigned`: Overcell accepted the run assignment.
- `benchmark_runner.sample_received`: signed raw sample accepted.
- `benchmark_runner.run_normalized`: samples converted into versioned normalized results.
- `benchmark_runner.evidence_published`: result published to Overregistry.
- `benchmark_runner.anomaly_detected`: impossible, suspicious, unstable, or degraded result found.
- `benchmark_runner.result_invalidated`: previous result invalidated or superseded.
- `benchmark_runner.run_failed`: run reached terminal failure with reason code.

Events must avoid embedding large logs or raw private host details when content hashes and object refs are enough.

## Core Workflow

1. Hardware Discovery publishes an inventory snapshot and benchmark prerequisites for a node.
2. Benchmark Runner selects the smallest compatible suite for the node class and trigger reason.
3. Overcell receives a signed benchmark assignment with suite version, safety limits, and timeout.
4. The node agent runs approved benchmark steps under local safety limits.
5. The node uploads signed samples and environment refs.
6. Benchmark Runner validates signatures, suite hashes, sample ranges, and environment consistency.
7. Results are normalized into measured tiers with confidence and expiry.
8. Evidence is signed and published to Overregistry.
9. Overwatch receives lifecycle, anomaly, and publication events.
10. Oververify or operators may trigger challenge reruns or invalidate stale results.

## State Machine

Benchmark run lifecycle:

1. `planned`: run request accepted but not yet assigned.
2. `queued`: run is waiting for node availability or rate-limit window.
3. `assigned`: Overcell has accepted the assignment for a specific node.
4. `running`: node agent is executing benchmark steps.
5. `sampled`: required raw samples were received.
6. `normalizing`: coordinator is validating and converting samples.
7. `published`: signed normalized evidence was written to Overregistry.
8. `superseded`: a newer valid run replaces the result for scheduling purposes.
9. `invalidated`: result is no longer eligible because evidence was challenged, expired, or contradicted.
10. `failed`: run failed with reason and retry eligibility.
11. `cancelled`: authorized actor or policy cancelled before terminal evidence.

Allowed transitions must be append-only. A published result can be superseded or invalidated, not silently edited.

## Policy And Security

- Only approved benchmark suites can run; arbitrary commands are rejected.
- Every node sample upload requires node credential verification and assignment binding.
- The coordinator signs final evidence separately from node signatures.
- Benchmark jobs must declare maximum CPU load, GPU memory, disk writes, network targets, duration, and cooldown windows.
- Seed hardware protection wins over benchmark completeness; safety violations cancel the run.
- Results that are too good, too unstable, or inconsistent with discovery facts produce anomaly records.
- Raw host details that could expose private infrastructure should be redacted from general reads and retained as restricted evidence refs.
- Benchmark evidence must include suite version and environment snapshot so results remain explainable after software or driver upgrades.
- Public provider phases must add anti-gaming controls, random challenge timing, and repeated-run consistency checks before unknown nodes are trusted.

## Metering And Accounting

Benchmarking consumes real compute, GPU, disk, and network resources, so it must emit usage facts even when no tenant is directly billed.

- Emit system-service usage events for benchmark duration, CPU/GPU time, disk writes, network bytes, and object storage used for evidence.
- Link usage to node id, provider id, suite id/version, trigger type, and requester.
- Keep benchmark overhead visible to operators and future provider settlement logic.
- Do not charge per tiny benchmark event; usage rollups should flow through Overmeter and later ORU/Seal Ledger accounting.
- Distinguish provider-eligibility benchmarks from tenant-requested performance tests so accounting can apply the right policy later.

## Observability And Operations

- Dashboards should show active runs, queued runs, per-node last valid result, anomaly counts, failure reasons, and suite version coverage.
- Health checks should confirm suite registry access, Overcell command path, Overregistry publication, Overwatch event emission, and signer availability.
- Operators need safe controls to pause benchmark scheduling, disable a suite, invalidate a result, and rerun a node.
- Retention should keep raw samples long enough for disputes while preserving compact normalized results for scheduling history.
- Migration tooling must preserve content hashes and signatures when evidence schema versions change.

## Failure Modes And Recovery

- Node offline: leave run queued until timeout, then fail with retry eligibility.
- Node rejects suite: fail with compatibility reason and mark missing prerequisite if appropriate.
- Safety threshold exceeded: cancel run, emit anomaly, and require cooldown before retry.
- Signature mismatch: reject samples, mark run failed, and notify Oververify for investigation.
- Normalization error: preserve raw samples and fail with schema/version reason.
- Overregistry unavailable: keep result in `sampled` or `normalizing` with retry state; do not mark published.
- Suspicious result: publish anomaly as restricted evidence and avoid scheduler eligibility until Oververify resolves it.

## Validation Plan

The service implementation plan lists these requirements:

- Benchmarks run without harming node availability.
- Results are stable enough for scheduler tiers.
- Impossible or degraded results can be challenged by Oververify.

Additional SDS-level validation:

- Contract tests for suite registration, run trigger, signed sample upload, result read, and invalidation APIs.
- Fixture tests for CPU-only node, GPU node, missing driver, thermal throttling, slow disk, and unstable network.
- Signature tests for node-signed samples and coordinator-signed evidence.
- Replay tests proving normalized results can be reconstructed from suite version, environment snapshot, and samples.
- Safety tests proving resource caps, timeouts, cooldowns, and network target allowlists work.
- Challenge tests proving Oververify-triggered reruns can supersede or invalidate results.

## Build Breakdown

1. Define suite manifest schema, result schema, unit conventions, and safety-limit schema.
2. Implement one CPU suite and one GPU suite for founder hardware, plus disk and private-network probes where feasible.
3. Add Overcell assignment and signed sample upload for a single node.
4. Normalize results into measured capability records and publish to Overregistry.
5. Emit Overwatch lifecycle and anomaly events.
6. Add scheduled reruns on agent version, driver/runtime, and inventory changes.
7. Add Oververify challenge hooks, invalidation records, and repeated-run stability checks.

The Phase 2 target is controlled private-swarm capacity evidence. Broader public-provider anti-gaming belongs to later trust and federation phases.

## Handoff And Downstream Use

Benchmark evidence feeds Overregistry capability records, Oversched placement filters, Oververify trust evidence, Challenge Task Service reruns, provider reputation, and operator visibility. Downstream consumers must read the published evidence contract rather than Benchmark Runner private tables.

## Open Design Questions

- Phase 3 private-workload eligibility is resource-class specific. Every schedulable private node must have fresh successful baseline suites for CPU throughput, local scratch disk read/write behavior, private-swarm network reachability/latency, runner cold-start overhead, and bounded sustained-load safety. GPU workloads additionally require the approved GPU inference or matrix-throughput suite and matching driver/runtime evidence. Storage-heavy, gateway/network-heavy, and future accelerator classes require their class-specific suites before Oversched can place workloads that request those resource cards. A node that lacks a class-specific suite may remain eligible for lower classes only when the required baseline suites, heartbeat, node state, and Oververify eligibility all pass.
- Scheduler-validity windows are stored on each benchmark result and are intentionally shorter for volatile resources. For Phase 3 private placement, baseline CPU, disk, private-network, and cold-start results remain valid for at most seven days; GPU/runtime and sustained thermal or throttling results remain valid for at most seventy-two hours; any warning, degraded state, agent upgrade, driver/runtime change, kernel/OS change, material Hardware Discovery delta, prolonged offline interval, safety cancellation, Oververify challenge, or suspicious result forces an immediate rerun or blocks the affected resource class. Later public, regulated, and system-service tiers may only tighten these windows.
- Raw host details that expose founder or provider infrastructure stay restricted evidence, not scheduler-facing fields: serial numbers, MAC addresses, public or private IP addresses, hostnames, rack/facility/precise location, provider resource-pool ids, motherboard/BIOS ids, disk WWNs, GPU UUIDs or serials, raw network peer lists, internal route tables, mount paths, kernel command lines, local usernames, service paths, raw command output, stderr/log refs, sensor ids, and power or thermal details that identify a machine or site. General reads expose normalized resource class, capability tier, confidence, freshness, and redacted reason codes; restricted operator, Oververify, Overwatch, and dispute views may cite hashed refs and evidence bundles.
- Benchmark confidence is a per-result scheduling input, not an opaque trust score. Phase 3 `private_test` placement requires at least `0.80` confidence on every required suite, `private_standard` requires at least `0.85`, resource-intensive GPU or storage tiers require at least `0.90` on the requested resource-class suite plus passing baseline suites, and future `system_service_candidate` placement requires at least `0.95` plus repeated-run stability, current Oververify eligibility, and stronger policy checks. Results below `0.80` are inventory evidence only; Oversched must treat missing, expired, or unknown confidence as blocked rather than permissive.
- Anomaly severity controls eligibility by blast radius. `critical` anomalies automatically suspend all new placement for the node until Oververify or an authorized operator clears the evidence; examples include signature mismatch, tampered evidence, impossible capabilities, safety-limit bypass, repeated runner crashes, or suspected identity binding failure. `high` or `suspicious` anomalies suspend the affected resource class and request challenge or rerun before scheduling resumes. `degraded` anomalies remove the affected scheduler tier or downgrade to a lower eligible class when baseline evidence remains valid. `warning` anomalies trigger review, shorter validity, and rerun scheduling without immediate suspension. `info` anomalies are retained for trend analysis only.
