SDS #25

# Overrun SDS

## Purpose

Execute lease-bound workloads on Overcell nodes with verified packages, prepared sandboxes, controlled inputs and secrets, supervised runtime, result capture, usage emission, cancellation, timeout, and cleanup.

Overrun is the node-side runner for the Phase 3 private execution loop. It does not schedule work and it does not invent workload instructions. It runs only assignments that reference a valid lease, an accepted Overpack manifest, and policy-compatible execution context.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overrun.md](../../service_catalog/execution_scheduling/overrun.md) |
| Sub-build plan | [SUB BUILD PLAN #25 - Overrun](../../build_plan/sub_build_plan_025_overrun.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md) |

## Service Family

- Family: Execution and scheduling
- Owning layer: Lease-bound sandbox execution and result capture
- Primary data scope: run assignments, execution attempts, preflight reports, sandbox specs, input mounts, secret mount refs, progress records, result records, logs/artifact refs, usage refs, cancellation refs, timeout refs, and cleanup records
- First build phase from service plan: [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md)

## Problem Statement

The scheduler can choose a node and Overlease can reserve it, but useful work only happens if the node can verify the assignment, prepare a safe runtime, mount authorized inputs and secrets, enforce policy, supervise execution, capture results, report usage, and clean up every time.

Overrun is that execution boundary. It must support a narrow workload class well before broad runtime coverage, because controlled failure and cleanup are part of the product.

## Goals

- Fetch and accept assignments only when a valid lease exists.
- Verify Overpack manifest integrity, artifact hashes, signatures, runtime contract, policy refs, egress policy, data class, secret policy, and timeout.
- Prepare a sandbox and mount authorized inputs, output locations, and secret refs.
- Supervise execution with structured progress, logs, cancellation, timeout, and final-state capture.
- Emit raw usage events to Overmeter and audit events to Overwatch.
- Store output, log, and artifact refs through phase-appropriate storage contracts.
- Cleanup sandbox, mounts, credentials, temp files, and local cache refs after success or failure.
- Produce distinct final states for success, failure, retryable failure, timeout, cancellation, and cleanup failure.

## Non-Goals

- Do not pick placement or reserve resources. Oversched and Overlease own those boundaries.
- Do not run work without a valid lease and accepted manifest.
- Do not bypass Overguard policy or Overvault secret controls.
- Do not store raw secrets in run records, logs, or diagnostics.
- Do not implement every runtime type in v0; support one narrow workload class well first.
- Do not perform settlement or provider payout logic.
- Do not allow arbitrary operator shell commands under the workload runner path.

## Primary Actors And Clients

- Overcell node agent delivering signed assignments to the local runner.
- Overlease verifying execution eligibility and lease windows.
- Overpack supplying manifest and package integrity contracts.
- Overguard supplying policy decision refs for data, egress, secrets, workload class, and package trust.
- Overvault supplying secret refs or phase-appropriate secret stubs.
- Overstore supplying input/output/log/artifact refs once storage exists.
- Overmeter consuming raw usage facts.
- Overwatch and Overclaim consuming execution evidence, failure reasons, and dispute refs.

## Dependencies

- [Overcell](overcell.md) for signed command delivery, node identity, local health, and audit spool.
- [Overlease](overlease.md) for valid lease verification.
- [Overpack](overpack.md) for manifest, artifact, runtime, and permission contracts.
- [Oversched](oversched.md) for placement decisions, candidate reason refs, and lease-request context.
- [Overguard](../trust_policy_verification/overguard.md) for policy decisions and egress/secret/data-class rules.
- [Overvault](../data_storage_namespace/overvault.md) for secret refs and mount policy once available.
- [Overstore](../data_storage_namespace/overstore.md) for inputs, outputs, logs, and artifacts once available.
- [Overmeter](overmeter.md) for raw usage events.
- [Overwatch](../control_plane/overwatch.md) for audit and trace events.

Phase 3 can use narrow local filesystem/object-ref stubs for storage and secrets, but the final refs must preserve the same contract shape.

Storage and secret adapters are explicit readiness gates. A run may use a Phase 3 stub only when the assignment records the stub profile, data class, redaction policy, cleanup policy, and migration path to Overstore or Overvault. After policy requires Overstore or Overvault, Overrun must fail preflight instead of silently falling back to local files.

## Owned Responsibilities

Overrun owns:

- Assignment acceptance and local preflight checks.
- Lease, manifest, package, policy, secret, and egress verification before side effects.
- Sandbox preparation, runtime adapter invocation, resource limits, timeout, cancellation, and cleanup.
- Input, output, log, and artifact ref handling.
- Structured progress and final result records.
- Raw usage event emission for execution dimensions.
- Execution attempt history and retry/failure evidence.

Overrun must not mutate queue state directly except through documented result/reporting APIs. It reports facts; queue and policy services decide retry/dead-letter behavior.

## Data Model

The first implementation should define:

- `run_assignment`: assignment id, queue item id, workload id, manifest id/version, lease id, node id, tenant id, resource reservation, policy refs, timeout, and trace id.
- `execution_attempt`: attempt id, assignment id, node id, state, start/end timestamps, retry index, runtime adapter, sandbox id, and final reason code.
- `preflight_report`: lease verification, manifest verification, package hash/signature status, runtime availability, input availability, secret policy status, egress policy status, and denial reasons.
- `sandbox_spec`: runtime type, isolation mode, resource limits, filesystem layout, network policy, environment refs, mount refs, and cleanup policy.
- `input_mount_plan`: input refs, data class, read/write mode, checksum expectations, and mount paths.
- `secret_mount_plan`: secret refs, allowed process scope, ttl, redaction rule, and unmount behavior.
- `storage_adapter_readiness`: adapter kind, supported URI schemes, allowed data classes, local stub profile, failure mode, and Overstore migration requirement.
- `secret_adapter_readiness`: adapter kind, secret ref schemes, mount mechanism, redaction guarantees, ttl enforcement, and Overvault migration requirement.
- `run_progress`: attempt id, sequence, timestamp, progress kind, log ref, metric refs, and state hint.
- `run_result`: final state, output refs, log refs, artifact refs, error type, reason code, runtime metrics, usage refs, and audit refs.
- `cleanup_record`: resources removed, failures, retries, leftover refs, and operator follow-up if needed.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

Most Overrun calls are internal node/control-plane calls:

- `POST /runs/assignments/{assignment_id}/accept`: node runner accepts or rejects a signed assignment after preflight.
- `POST /runs/{run_id}/start`: mark execution attempt as starting after lease and manifest verification.
- `POST /runs/{run_id}/progress`: stream structured progress, log refs, and metric refs.
- `POST /runs/{run_id}/cancel`: request cancellation with reason and deadline.
- `POST /runs/{run_id}/complete`: submit final result, output refs, usage refs, and cleanup state.
- `GET /runs/{run_id}`: read authorized run state and evidence refs.
- `POST /runs/{run_id}/cleanup`: retry cleanup or record cleanup final state.

API requirements:

- Start requires a valid lease proof and accepted manifest refs.
- Every node-originated report is signed through Overcell/node credentials.
- Cancellation must be idempotent and safe if the process already exited.
- Result reports must distinguish execution failure from cleanup failure.
- Reads must redact logs, inputs, outputs, secrets, and private data by tenant/data-class rules.

## Event Surface

- `overrun.assignment_accepted`: runner accepted assignment after preflight.
- `overrun.assignment_rejected`: preflight rejected assignment with reason.
- `overrun.sandbox_prepared`: sandbox and mounts prepared.
- `overrun.execution_started`: workload process started.
- `overrun.progress_reported`: structured progress/log refs emitted.
- `overrun.cancel_requested`: cancellation requested.
- `overrun.execution_timed_out`: timeout enforced.
- `overrun.execution_completed`: workload reached terminal success.
- `overrun.execution_failed`: workload failed with reason and retryability.
- `overrun.cleanup_completed`: sandbox cleanup completed.
- `overrun.cleanup_failed`: cleanup left residual work or evidence.

Events must include trace, lease, workload, node, attempt, and policy refs without embedding secret values or private payloads.

## Core Workflow

1. Overcell receives a signed assignment for a node.
2. Overrun verifies lease validity through Overlease.
3. Overrun verifies Overpack manifest, artifact hashes, signatures, runtime contract, egress policy, secret policy, timeout, and resource reservation.
4. Overrun prepares the sandbox and mounts authorized inputs, output refs, and secret refs.
5. Runner starts the workload under resource, timeout, network, and policy controls.
6. Runner streams structured progress and logs refs.
7. Cancellation, timeout, process failure, or success moves the attempt to a terminal execution state.
8. Runner captures outputs, logs, artifacts, metrics, final state, and usage refs.
9. Runner cleans up sandbox, mounts, temp files, credentials, and local refs.
10. Runner reports final result to the control plane and emits Overmeter/Overwatch events.

## State Machine

Execution attempt lifecycle:

1. `assigned`: assignment delivered to node.
2. `preflight`: lease, manifest, package, policy, runtime, and input checks are running.
3. `rejected`: assignment failed preflight and did not execute.
4. `sandboxing`: sandbox and mounts are being prepared.
5. `starting`: runtime is launching the workload.
6. `running`: workload process is active.
7. `cancelling`: cancellation is being enforced.
8. `timing_out`: timeout termination is being enforced.
9. `succeeded`: workload completed successfully.
10. `failed_retryable`: workload failed and policy may retry.
11. `failed_final`: workload failed terminally.
12. `cancelled`: workload stopped by authorized cancellation.
13. `timed_out`: workload exceeded timeout and was terminated.
14. `cleaning`: sandbox cleanup is in progress.
15. `cleanup_failed`: execution has terminal state but cleanup needs retry/operator follow-up.
16. `reported`: final result and usage refs were reported.

State history is append-only. Cleanup failure must not erase the execution final state.

## Policy And Security

- A valid lease is mandatory before execution.
- Overpack manifest integrity and artifact hashes must be verified before sandbox preparation.
- Secret values must come through Overvault or a phase-appropriate secret reference mechanism and must never be written to logs/results.
- Egress policy must be applied before process start.
- Input and output mounts must enforce data class and read/write permissions.
- Runtime adapters must run with least privilege and bounded resources.
- Cancellation and timeout must be enforced even when workload code misbehaves.
- Logs and diagnostics must redact secret refs, private inputs, and sensitive outputs.
- Operator overrides require signed action and Overwatch evidence.

## Metering And Accounting

Overrun emits raw execution usage; it does not bill:

- CPU time, GPU time, memory peak, storage read/write, network bytes, wall time, queue wait, start delay, model inference count where applicable, and cleanup overhead.
- Link every usage event to tenant, workload, queue item, lease, node, provider, package, run attempt, and final state.
- Emit usage up to cancellation or timeout.
- Preserve failed and cleanup-failed usage for disputes and accounting review.
- Do not call external payment rails or produce settlement entries.

## Observability And Operations

- Operators need views for assignments, preflight failures, active runs, cancellations, timeouts, failed cleanup, output/log refs, and usage emission status.
- Health checks should cover runtime adapter availability, sandbox creation, lease verification, manifest verification, storage refs, secret refs, egress controls, and Overmeter/Overwatch reporting.
- Diagnostics should include reason codes and evidence refs, not raw private content.
- Runtime adapters should expose version and compatibility info.
- Cleanup retries and residual-resource warnings must be visible.

## Failure Modes And Recovery

- Invalid lease: reject assignment before sandbox setup.
- Manifest or artifact hash mismatch: reject before execution and emit security evidence.
- Secret mount denied: reject before process start.
- Input unavailable: fail preflight with retryability based on storage policy.
- Sandbox setup failure: fail before workload start and cleanup partial resources.
- Runtime crash: capture logs/metrics, mark failed retryable or final based on manifest/policy.
- Timeout: terminate process tree, capture usage up to termination, cleanup, and report timed_out.
- Cancellation: stop workload, capture partial outputs if policy allows, cleanup, and report cancelled.
- Cleanup failure: keep final execution state and schedule cleanup retry/operator follow-up.
- Result report failure: spool result through Overcell local audit until reconnect.

## Validation Plan

The service implementation plan lists these requirements:

- Successful, failed, cancelled, and timed-out jobs produce distinct final states.
- Secrets are mounted only when policy allows.
- Cleanup runs even after execution failure.

Additional SDS-level validation:

- Contract tests for assignment accept/reject, start, progress, cancel, complete, read, and cleanup APIs.
- Lease validation tests for valid, expired, cancelled, wrong-node, and wrong-workload leases.
- Manifest/package verification tests for good, missing, tampered, unsigned, and revoked artifacts.
- Sandbox tests for mount permissions, egress policy, timeout, cancellation, and cleanup.
- Secret redaction tests across logs, results, progress, and diagnostics.
- Usage emission tests for success, failure, timeout, and cancellation.
- Replay tests proving final result and usage can be reconstructed from assignment, lease, manifest, and event refs.

## Build Breakdown

1. Define assignment, attempt, preflight, sandbox, result, progress, usage, and cleanup schemas.
2. Implement one narrow workload class for Phase 3, preferably command or container jobs on the private swarm.
3. Verify valid lease and Overpack manifest before execution.
4. Prepare sandbox, mount inputs, apply egress policy, and start workload.
5. Enforce timeout and cancellation.
6. Capture outputs, logs, metrics, final state, and usage refs.
7. Cleanup sandbox and report final result through control-plane state.
8. Add explicit storage and secret adapter readiness checks before enabling Overstore and Overvault integrations.
9. Retire Phase 3 local stubs only after contract-compatible Overstore and Overvault paths pass validation.

The first runner should do one workload class well rather than provide broad but unsafe runtime coverage.

## Handoff And Downstream Use

Overrun produces execution evidence, results, logs/artifact refs, final states, and usage events for Overmeter, Overwatch, Overclaim, Overqueue retry/dead-letter policy, product adapters, and later native apps. Consumers should use run result records and event refs rather than scraping node logs.

## Open Design Questions

- Resolved: Phase 3 should prove the runner with a signed command-contract job as the first fixture and make signed command-contract jobs plus OCI/container jobs the v0 execution-eligible workload class before the first real private workload. Model inference may run only through those supported runtime kinds with explicit model/artifact, GPU, and metering refs; WASI/Wasmtime remains schema-recognized but execution-ineligible unless the Phase 3 adapter is tested. No workload class may degrade into arbitrary operator shell execution.
- Resolved: OCI-compatible sandboxing is mandatory for execution-eligible founder and provider workloads: rootless container runtime where possible, Linux namespaces, cgroups/resource limits, seccomp or equivalent syscall filtering, read-only base filesystem, per-run writable scratch, explicit egress policy, and controlled NVIDIA/ROCm GPU adapter hooks when accelerators are exposed. Founder hardware may keep a command-contract bootstrap profile for no-secret, low-sensitivity fixtures, but provider hardware and secret-bearing workloads require the full sandbox profile; WASI may be added only as a tested adapter, not as the default escape hatch.
- Resolved: Partial outputs from cancelled or timed-out workloads are publishable only when the manifest declares a partial-output or checkpoint policy, the data class permits partial retention, output refs are content-hashed, and cleanup can prove no secret-bearing temp state leaked. Otherwise Overrun must quarantine or discard partial outputs, keep only redacted evidence/log refs for Overwatch and Overclaim, mark the result as non-authoritative, and exclude the partial output from normal product handoff until policy explicitly releases it.
- Resolved: Cleanup policy is an initial synchronous cleanup attempt before final reporting, followed by two bounded automatic retries with backoff through the Overcell local spool. If secret mounts, network namespaces, writable sandboxes, GPU contexts, or provider-billable resources remain after three total cleanup attempts or 15 minutes, Overrun must mark `cleanup_failed`, preserve the execution terminal state, emit residual-resource evidence, block new secret-bearing work on that node profile, and require operator or incident workflow intervention. Non-sensitive cache/temp residue may continue through a background sweeper only when policy labels it safe.
- Resolved: Settlement-ready metrics are the lease-bound run window, wall-clock start/end, final state, CPU time, memory peak, storage read/write bytes, network bytes, cleanup overhead, and GPU/model dimensions only when they come from named runtime/container, cgroup, NVIDIA/ROCm, or model-router sources with source refs, confidence metadata, and Overmeter validation. Diagnostic-only metrics include raw host load averages, uncorroborated node self-reports, stdout-parsed counters, unverifiable GPU utilization/power samples, and cache or network observations not tied to lease/run/source refs; these can support debugging and disputes but cannot become Phase 5 signed settlement totals without corroboration.
- Resolved: Before Overstore and Overvault are fully available, founder hardware may use only explicit `founder_local_object_ref` and `founder_local_secret_ref` adapter profiles for private Phase 3 workloads. The storage profile must be content-addressed with BLAKE3 hashes, per-run directories, retention and cleanup policy, migration refs to Overstore, and no cross-node durability promise. The secret profile must use provisioned local secret refs with TTL, least-scope mount, redaction, unmount/zeroization evidence, and migration refs to Overvault. These stubs are forbidden for public/provider pools, regulated or third-party secret workloads, or any assignment whose policy requires Overstore or Overvault; in those cases Overrun fails preflight instead of falling back.
