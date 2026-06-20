SDS #64

# Codali Adapter SDS

## Purpose

Run Codali/code-agent workloads through Overrid with repository context refs, logs, artifacts, structured results, and usage capture.

Codali Adapter translates code-agent tasks into Overrid workload packages. It owns code-agent manifests, repo context refs, sandbox/tool boundaries, phase logs, artifact/result refs, bounded repair loops, and usage/audit records. It does not give agents arbitrary repository write access, own Git state, select final model routes, or bypass policy and sandbox rules.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [codali_adapter.md](../../service_catalog/adapters/codali_adapter.md) |
| Sub-build plan | [SUB BUILD PLAN #64 - Codali Adapter](../../build_plan/sub_build_plan_064_codali_adapter.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md), [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md) |

## Service Family

- Family: Ecosystem adapters
- Owning layer: Code-agent workload packaging, execution handoff, and result capture
- Primary data scope: code-agent manifests, repo context refs, task packages, sandbox policies, execution phases, logs, artifacts, structured results, repair-loop records, usage refs, and audit evidence
- First build phase from service plan: [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md)

## Problem Statement

Codali/code-agent workloads are a strong first proof for Overrid: they need private repo context, model routing, sandboxed execution, artifact capture, logs, retries, and usage accounting. They also have a high blast radius if designed loosely. A code agent that can write anywhere, run arbitrary tools, or ignore policy would undermine the whole platform.

The adapter must turn code-agent work into explicit packages and phases. Repository context must arrive through authorized refs. Execution must happen through Overpack/Overrun and lease-bound sandboxes. Results must be structured enough for humans and downstream tools to review, not just opaque text blobs.

## Goals

- Accept Codali/code-agent tasks through SDK/CLI/API with signed actor, tenant, repo, and task context.
- Build Overpack-compatible code-agent manifests with tool permissions, sandbox profile, resource needs, and expected outputs.
- Request repository context through Encrypted Docdex RAG Adapter or Docdex Adapter without direct raw repo reads.
- Execute code-agent phases through Overqueue, Oversched, Overlease, Overrun, and AI Gateway Router where model resources are needed.
- Capture logs, artifacts, patches, structured results, validation outputs, and failure reasons.
- Bound retry/repair loops with policy, budget, context, and user/operator approval where needed.
- Meter usage by agent phase and preserve audit/replay evidence.

## Non-Goals

- Do not own Codali's product UI or full code-agent reasoning engine.
- Do not own Git truth, branch protection, merge decisions, or repository hosting.
- Do not allow arbitrary repo writes outside declared sandbox/output refs.
- Do not bypass Encrypted Docdex RAG authorization for private repo context.
- Do not bypass Overpack, Overrun, Overguard, Overvault, Overmeter, or Overwatch for platform work.
- Do not choose models directly when AI Gateway Router is required.
- Do not add pricing forecasts, customer-count assumptions, blockchain mechanics, NFT mechanics, or per-transaction fee economics.

## Primary Actors And Clients

- Codali product submitting code-agent tasks.
- Developers and organizations using SDK/CLI/admin UI to submit, inspect, approve, or cancel code-agent jobs.
- AI Gateway Router selecting model/resource routes for reasoning, generation, review, and repair phases.
- Encrypted Docdex RAG Adapter and Docdex Adapter providing authorized repo context and retrieval jobs.
- Overpack, Overqueue, Oversched, Overlease, Overrun, Overstore, Overvault, Overmeter, and Overwatch executing and recording the workload.
- Overguard and Workload Classifier applying policy, data-class, tool, sandbox, and budget checks.

## Dependencies

- [Overpack](../execution_scheduling/overpack.md) for code-agent workload manifests and artifact refs.
- [Overqueue](../control_plane/overqueue.md), [Oversched](../execution_scheduling/oversched.md), [Overlease](../execution_scheduling/overlease.md), and [Overrun](../execution_scheduling/overrun.md) for queued, leased, sandboxed execution.
- [AI Gateway Router](../ai_rag_model_routing/ai_gateway_router.md) for model/resource route decisions.
- [Encrypted Docdex RAG Adapter](../ai_rag_model_routing/encrypted_docdex_rag_adapter.md) and [Docdex Adapter](docdex_adapter.md) for repository context and Docdex retrieval/index jobs.
- [Overstore](../data_storage_namespace/overstore.md), [Overbase](../data_storage_namespace/overbase.md), and [Overvault](../data_storage_namespace/overvault.md) for artifacts, structured state, and secret refs.
- [Overguard](../trust_policy_verification/overguard.md), [Workload Classifier](../trust_policy_verification/workload_classifier.md), [Overmeter](../execution_scheduling/overmeter.md), and [Overwatch](../control_plane/overwatch.md) for policy, classification, usage, and audit.

## Owned Responsibilities

Codali Adapter owns:

- Code-agent task intake, validation, idempotency, and task package refs.
- Code-agent manifest normalization for Overpack.
- Repository context requests and context ref attachment.
- Sandbox/tool permission profiles for code-agent phases.
- Execution phase records for plan, edit, test, review, repair, and summarize phases.
- Log streams, artifact refs, patch refs, structured results, and validation summaries.
- Retry/repair-loop state, approval gates, cancellation, and failure reason codes.
- Usage refs, audit refs, and replay bundles for code-agent jobs.

## Data Model

- `code_agent_task`: actor/tenant/org refs, repo refs, task description hash/ref, requested phase set, privacy mode, data class, budget/limit refs, trace id, and idempotency key.
- `code_agent_manifest`: Overpack manifest ref, runtime/tool permissions, sandbox profile, model capability needs, repo context requirements, expected artifacts, validation commands, and output schema.
- `repo_context_ref`: Docdex index ref, context bundle ref, allowed path scopes, redaction profile, grant refs, and context expiry.
- `code_agent_phase`: job ref, phase name, route ref, lease/run refs, input refs, output refs, state, start/end times, and reason codes.
- `agent_log_ref`: stdout/stderr/tool log refs, redaction status, retention class, and Overwatch refs.
- `artifact_ref`: patch/diff refs, generated file refs, test result refs, report refs, package refs, hashes, and storage refs.
- `structured_result`: summary, changed files, validation status, risk notes, follow-up refs, confidence, and reviewer/approval state.
- `repair_loop_record`: trigger reason, max attempts, current attempt, prior artifact refs, policy/budget checks, approval status, and terminal outcome.
- `codali_usage_ref`: model, compute, storage, bandwidth, test execution, queue, retry, and repair usage dimensions.

## API Surface

- `POST /codali/jobs`: submits a code-agent task and creates a job package.
- `POST /codali/jobs/validate`: validates a code-agent manifest without executing.
- `GET /codali/jobs/{job_id}`: returns state, phases, route refs, context refs, artifacts, logs, and usage refs.
- `POST /codali/jobs/{job_id}/start`: queues execution after policy and package checks.
- `POST /codali/jobs/{job_id}/cancel`: cancels queued/running phases and preserves partial artifacts.
- `POST /codali/jobs/{job_id}/repair`: starts an allowed repair loop or requests approval if required.
- `GET /codali/jobs/{job_id}/logs`: returns redacted phase logs visible to the caller.
- `GET /codali/jobs/{job_id}/artifacts`: returns artifact refs and structured result refs.
- `POST /codali/jobs/{job_id}/approve-artifact`: records user/org approval for a patch/result handoff.
- `GET /codali/replay/{job_id}`: reconstructs package, context, route, execution, repair, result, usage, and audit decisions.

Mutating APIs require actor identity, tenant/org scope, repo scope, trace id, idempotency key, manifest refs, context grants, policy refs, and budget refs. Stable errors include `repo_context_required`, `repo_context_denied`, `manifest_invalid`, `sandbox_profile_missing`, `tool_not_allowed`, `route_unavailable`, `lease_failed`, `artifact_validation_failed`, `repair_limit_exceeded`, `approval_required`, and `policy_denied`.

## Event Surface

- `codali_adapter.job_submitted`: code-agent job accepted.
- `codali_adapter.manifest_validated`: manifest passed or failed validation.
- `codali_adapter.context_attached`: authorized repo context refs attached.
- `codali_adapter.phase_started`: execution phase started.
- `codali_adapter.phase_completed`: execution phase completed.
- `codali_adapter.phase_failed`: execution phase failed with reason code.
- `codali_adapter.repair_requested`: repair loop requested or started.
- `codali_adapter.artifact_created`: artifact/log/result refs created.
- `codali_adapter.job_completed`: job completed with structured result.
- `codali_adapter.job_cancelled`: job cancelled by user, policy, or operator.
- `codali_adapter.usage_emitted`: usage refs emitted.

Events include job refs, manifest refs, context bundle refs, route refs, phase refs, artifact refs, policy refs, result classes, and usage refs. They must not include raw private repo content, secrets, unredacted logs, or private prompts outside authorized audit scopes.

## Core Workflow

1. Codali or developer submits a code-agent task with repo scope, task metadata, expected outputs, and budget/limit refs.
2. Adapter validates identity, tenant, repo access, idempotency, manifest shape, data class, and policy.
3. Adapter requests authorized repo context from Encrypted Docdex RAG Adapter or Docdex Adapter.
4. Adapter creates an Overpack-compatible code-agent manifest with sandbox/tool/resource constraints.
5. AI Gateway Router selects model/resource routes for agent phases where needed.
6. Overqueue/Scheduler/Lease/Runner execute phases in sandboxed jobs and return logs/artifacts/results.
7. Adapter validates outputs, records structured result, and starts bounded repair only if policy and budget allow.
8. Adapter emits usage/audit refs and hands artifacts to Codali or user/org approval flows.

## State Machine

Job lifecycle:

1. `submitted`
2. `validated`
3. `context_attached`
4. `packaged`
5. `queued`
6. `running`
7. `repair_pending`
8. `completed`
9. `approval_pending`
10. `failed`
11. `cancelled`

Phase lifecycle:

1. `planned`
2. `route_selected`
3. `queued`
4. `leased`
5. `running`
6. `artifact_captured`
7. `validated`
8. `succeeded`
9. `retryable_failed`
10. `terminal_failed`

Repair lifecycle:

1. `proposed`
2. `policy_checked`
3. `approval_required`
4. `approved`
5. `running`
6. `succeeded`
7. `limit_reached`
8. `rejected`

## Policy And Security

- Repo context must be authorized through Docdex/RAG refs and constrained by repo/path/purpose/data class.
- Tool permissions must be explicit in the manifest and enforced by sandbox profiles.
- Code-agent jobs must not access raw secrets; secret use must flow through Overvault refs and mount leases where allowed.
- Patch/artifact creation is not merge authority. Branch protection and repository owner approval remain outside this adapter.
- Repair loops require max attempts, budget checks, output validation, and approval gates for risky changes.
- Logs and artifacts require redaction before broad visibility.
- Public or low-trust nodes cannot run private code-agent workloads unless policy explicitly allows the workload class.
- Operator overrides require signed action and Overwatch evidence.

## Metering And Accounting

- Emit usage refs for context retrieval, model routes, queue time, lease/runtime, test execution, storage, artifact transfer, retries, repair loops, and cancellation.
- Link usage to tenant/org, actor, repo, job, phase, route, artifact, and result refs.
- Failed and repaired phases remain visible for accounting and dispute review.
- Wallet/Usage Center and ORU Account Service consume rollups downstream; Codali Adapter does not maintain balances.
- Do not encode hardcoded prices, revenue projections, or provider payout rules.

## Observability And Operations

- Expose job volume, phase latency, queue/lease/runtime failures, context-denial rate, repair-loop rate, artifact validation failure rate, and model-route failure rate.
- Alert on sandbox escapes, unauthorized repo scope, secret access denial spikes, excessive repair loops, unredacted private logs, and artifact validation regressions.
- Provide replay for task package, context grant, manifest validation, route decisions, phase execution, artifacts, repair attempts, and usage.
- Provide SDK/CLI/admin diagnostics for job state, logs, artifacts, policy denials, and usage.

## Failure Modes And Recovery

- Repo context denied: fail before execution or request narrower permission.
- Manifest invalid: reject with schema and policy reason codes.
- Model route unavailable: retry allowed fallback through AI Gateway Router or fail phase.
- Lease/run failure: preserve package refs, retry state, logs, and reason code before retrying.
- Artifact validation fails: start repair only when budget/policy allow; otherwise mark job failed.
- Repair loop exceeds limit: stop and return latest artifacts with risk notes.
- Logs contain private content: quarantine log refs until redaction passes.
- Usage emission fails: retain phase/job refs for reconciliation before terminal completion.

## Validation Plan

- Codali can execute a private code-agent job through Overrid without direct internal API calls.
- Private repo context is authorized through Encrypted Docdex RAG Adapter or Docdex Adapter and bounded by scope.
- Agents cannot write outside declared artifact/output paths.
- Sandbox/tool permissions are enforced for every phase.
- Logs and artifacts are retrievable and redacted according to policy.
- Retry/repair loops stop at configured limits and require approval for risky changes.
- Agent phase usage is metered and auditable.
- Replay reconstructs task, context, package, route, execution, artifacts, repair, and usage decisions.

## Build Breakdown

1. Define code-agent task, manifest, repo context, phase, log, artifact, structured result, repair loop, and usage schemas.
2. Implement job submission, manifest validation, and policy precheck APIs.
3. Integrate Encrypted Docdex RAG Adapter and Docdex Adapter for repo context refs.
4. Generate Overpack manifests and route model phases through AI Gateway Router.
5. Execute phases through queue/scheduler/lease/runner and capture logs/artifacts/results.
6. Implement bounded repair loops, output validation, approval gates, cancellation, and replay.
7. Add Overmeter/Overwatch integration, SDK/CLI/admin diagnostics, and private-code workload tests.

## Handoff And Downstream Use

Codali Adapter hands code-agent manifests, job/phase refs, repo context refs, route refs, logs, artifacts, structured results, repair-loop refs, usage refs, and replay bundles to Codali, developers, SDK, CLI, admin UI, AI Gateway Router, Encrypted Docdex RAG Adapter, Overpack, Overrun, Overmeter, Overwatch, and repository-owner approval flows.

Downstream systems must treat adapter artifacts as proposed outputs until repository owners or their configured automation approve them.

## Open Design Questions

Resolved decisions:

- The first Phase 6 Codali proof is a bounded test-generation job, expressed as a patch-proposal artifact rather than a direct repository write. It may read authorized repo context through Encrypted Docdex RAG Adapter or Docdex Adapter refs, create a temporary sandbox checkout, generate tests under declared path scopes, run declared validation commands, and return diff/test artifacts for approval. Read-only analysis can be a dry-run precheck, while general repair waits until the test-generation path proves context, sandbox, artifact, metering, and replay behavior.
- The minimum sandbox profile is `private_code_agent_test_gen_v0`: private seed or trusted node only, accepted Overpack command/OCI manifest, valid Overlease, non-root Overrun sandbox, read-only repo/context input refs, write access only to an ephemeral workspace and declared output/artifact refs, deny-by-default network with an allowlist limited to Overrid control, artifact, context, and model-route endpoints, no raw secret mounts by default, explicit tool/command allowlist, resource/time limits, redacted logs, BLAKE3-hashed artifacts, and mandatory cleanup evidence.
- Required approval artifacts are an immutable job manifest/replay bundle, authorized context/ref summary, structured result JSON, unified diff or git-format patch, changed-file manifest with hashes, validation report with command ids, exit codes, and JUnit/TAP or equivalent test output where available, redacted logs, risk/follow-up notes, usage refs, and Overwatch evidence refs. Binary or generated assets must be Overstore artifact refs with hashes and media/runtime metadata; approval sees refs and summaries, not unchecked host writes.
- Early product integration allows at most one automatic repair attempt per job for retryable validation or artifact-shape failures inside the same or stricter context, sandbox, model/resource, tool, network, budget, and output scopes. One additional repair may run only after explicit user/org approval and must still avoid repo-host side effects; any permission widening, secret access, new context scope, accounting/native-app side effect, or third attempt requires a new signed job.
- Repo-host approval/merge handoff is not part of the Phase 6 execution proof. The first interface is a host-neutral `repo_approval_handoff` contract that emits patch/result refs and records approval state; first-class later adapters should cover Forgejo/Gitea and self-managed GitLab for open/self-hosted paths plus GitHub pull requests for common external repos. Bitbucket and Azure DevOps can follow demand. In all cases the repo host remains the merge and branch-protection authority, and Codali Adapter only proposes handoff refs.
