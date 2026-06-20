SDS #66

# Mcoda Adapter SDS

## Purpose

Run Mcoda agent workloads through Overrid resource, policy, execution, and metering rails.

Mcoda Adapter is the translation boundary between Mcoda agent tasks and Overrid workload execution. It packages agent requests into explicit task manifests, attaches tool and data boundaries, asks AI Gateway Router or scheduler-facing services for resource routes, submits executable work through normal queue/scheduler/runner paths, and returns structured results, failure reasons, logs, artifacts, and usage refs. It does not own Mcoda agent definitions, choose final policy outcomes, bypass tool grants, store private repo data, or hardcode one model/provider.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [mcoda_adapter.md](../../service_catalog/adapters/mcoda_adapter.md) |
| Sub-build plan | [SUB BUILD PLAN #66 - Mcoda Adapter](../../build_plan/sub_build_plan_066_mcoda_adapter.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md) |

## Service Family

- Family: Ecosystem adapters
- Owning layer: Product integration layer for agent workload translation
- Primary data scope: Mcoda task manifests, agent capability refs, route refs, tool-boundary declarations, tool grants, workload refs, phase records, result refs, failure refs, logs/artifact refs, usage refs, and replay bundles
- First build phase from service plan: [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md)

## Problem Statement

Overrid needs to prove that real agent execution can run on the grid without turning each product into a special-case integration. Mcoda can bring practical agent workloads, but agent tasks are dangerous if they arrive as opaque prompts with broad tool access, hidden model selection, unbounded loops, or unmetered resource use.

The adapter must convert Mcoda work into Overrid-native contracts: explicit task packages, resource requirements, data permissions, tool boundaries, policy checks, queueable execution, failure semantics, and usage accounting. The important design correction is that Mcoda Adapter must not become a privileged side door. Mcoda tasks should look like ordinary Overrid work with extra agent-specific metadata.

## Goals

- Define a stable Mcoda agent task manifest that Overpack, Overqueue, Oversched, Overrun, Overmeter, Overwatch, and AI Gateway Router can understand.
- Attach explicit tool-use boundary declarations before any task is admitted.
- Preserve actor, tenant, repo, model, tool, data-class, budget, and trace context across the full agent run.
- Let AI Gateway Router and scheduler-facing services select available model and compute resources without hardcoding one provider.
- Capture agent phase state, logs, artifacts, result refs, failure reasons, cancellation state, and usage refs.
- Expose replay and diagnostics so operators can explain why a task was accepted, denied, routed, retried, or failed.
- Keep Phase 6 small enough to run on seed hardware while preserving the path to grid-resident agent execution.

## Non-Goals

- Do not implement Mcoda's agent marketplace, local agent inventory, agent rating system, or user-facing agent UI.
- Do not bypass Overgate, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmeter, ORU Account Service, or Seal Ledger.
- Do not grant arbitrary shell, network, file, repo, wallet, messaging, vault, or native-app access just because an agent asks for it.
- Do not store raw private repo content, decrypted Docdex context, vault secrets, or opaque prompts as canonical adapter data.
- Do not mutate ORU balances, Seal Ledger entries, grants, payouts, disputes, package validation, or policy decisions directly.
- Do not treat a successful model response as proof that side effects are allowed.
- Do not add pricing, customer-count, revenue, blockchain, NFT, or per-transaction fee assumptions.

## Primary Actors And Clients

- Mcoda clients submitting agent tasks.
- Mcoda agent runtimes or local/cloud agent providers referenced by capability snapshots.
- SDK, CLI, and Admin/Developer UI showing validation, submission, status, result, logs, and usage.
- AI Gateway Router selecting model/resource routes for agent phases.
- Overpack, Overqueue, Oversched, Overlease, Overrun, and Overcell executing packaged agent work.
- Overguard, Workload Classifier, Policy Dry-Run API, and Overvault checking workload/data/tool permission.
- Docdex Adapter, Encrypted Docdex RAG Adapter, and Codali Adapter when Mcoda tasks need repo context or code-agent handoff.
- Overmeter, ORU Account Service, Seal Ledger, Overbill, Overwatch, and Overclaim receiving usage, audit, receipt, and dispute evidence.

## Dependencies

- [Overpack](../execution_scheduling/overpack.md) for executable workload manifest and artifact contracts.
- [Overqueue](../control_plane/overqueue.md), [Oversched](../execution_scheduling/oversched.md), [Overlease](../execution_scheduling/overlease.md), [Overrun](../execution_scheduling/overrun.md), and [Overcell](../execution_scheduling/overcell.md) for queueing, placement, leases, node-side execution, and supervision.
- [AI Gateway Router](../ai_rag_model_routing/ai_gateway_router.md) for model/resource route decisions and fallback policy.
- [Overguard](../trust_policy_verification/overguard.md), [Workload Classifier](../trust_policy_verification/workload_classifier.md), and [Policy Dry-Run API](../trust_policy_verification/policy_dry_run_api.md) for admission, class facts, and preflight explanation.
- [Docdex Adapter](docdex_adapter.md), [Encrypted Docdex RAG Adapter](../ai_rag_model_routing/encrypted_docdex_rag_adapter.md), and [Overvault](../data_storage_namespace/overvault.md) for authorized context refs and secrets-by-reference.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), [Seal Ledger](../accounting/seal_ledger.md), and [Overbill](../accounting/overbill.md) for usage and accounting refs.
- [Overwatch](../control_plane/overwatch.md) and [Overclaim](../trust_policy_verification/overclaim.md) for audit, evidence, and disputes.

## Owned Responsibilities

Mcoda Adapter owns:

- Mcoda task intake normalization and schema validation.
- `agent_task_manifest` records that bind Mcoda task refs to Overrid workload refs.
- Agent capability snapshots accepted from Mcoda inventory or route-selection metadata.
- Tool boundary declarations, tool grant refs, and denied-tool records.
- Data/context access declarations for repo, Docdex, vault, app, and external-resource refs.
- Phase records for planning, context attach, route selection, execution, tool calls, repair loops, result capture, and cleanup.
- Result refs, artifact refs, log refs, failure reason refs, and cancellation refs.
- Usage handoff records that link agent phases to Overmeter and wallet-visible receipts.
- Replay bundles for policy, route, tool, execution, and result decisions.

Mcoda Adapter does not own agent source code, model inference, model inventory truth, scheduling, sandbox enforcement, policy authority, vault secret material, ledger truth, or final dispute resolution.

## Data Model

- `agent_task_manifest`: task id, Mcoda task ref, actor/tenant refs, requested agent ref, task type, prompt/input refs, expected output schema, data classes, tool classes, route hints, resource hints, timeouts, retry limits, and state.
- `agent_capability_snapshot`: agent/provider refs, advertised usage class, model/resource refs, context window class, tool support flags, health refs, max complexity, trust refs, and freshness timestamp.
- `tool_boundary_declaration`: allowed tool names/classes, denied tool classes, side-effect classes, confirmation requirements, network/file/repo scopes, vault grants, native-app tool refs, and policy refs.
- `context_access_plan`: Docdex index refs, repo refs, encrypted RAG context refs, workspace/app refs, redaction profile, leakage profile, and authorization refs.
- `route_request_ref`: AI Gateway route request, model/resource candidates, fallback policy, selected route ref, denial/refusal refs, and usage estimate refs.
- `agent_workload_ref`: Overpack manifest ref, queue item ref, lease ref, runner assignment ref, sandbox profile, package validation ref, and execution state.
- `agent_phase_record`: phase name, input refs, output refs, tool calls, route refs, policy refs, started/ended timestamps, status, and failure reason.
- `agent_result_ref`: structured result refs, artifact refs, log refs, model/run provenance, validation status, user-visible summary ref, and checksum/hash refs.
- `agent_failure_ref`: phase, stable reason code, retryability, downstream dependency, policy refs, user/actionable message, and recovery path.
- `agent_usage_ref`: model tokens/units, compute time, storage, bandwidth, tool calls, retries, queue time, operator review, Overmeter refs, ORU/receipt refs, and dispute refs.

Common envelope fields: `id`, `tenant_id`, `actor_id`, `service_account_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

- `POST /adapters/mcoda/tasks/validate`: validates a task manifest, tool boundaries, output schema, and data-access plan without submitting work.
- `POST /adapters/mcoda/tasks`: creates an adapter-owned task record and returns manifest, policy-precheck, route-precheck, and queue-readiness refs.
- `POST /adapters/mcoda/tasks/{task_id}/start`: submits the prepared task to Overqueue/Overpack after policy and budget checks pass.
- `GET /adapters/mcoda/tasks/{task_id}`: returns task state, phase states, route refs, tool-boundary refs, result refs, usage refs, and failure refs subject to role filtering.
- `POST /adapters/mcoda/tasks/{task_id}/cancel`: requests cancellation and forwards it to queue, scheduler, lease, or runner state as applicable.
- `POST /adapters/mcoda/tasks/{task_id}/tool-grants`: attaches, narrows, or revokes explicit tool grants before or during execution.
- `POST /adapters/mcoda/tasks/{task_id}/retry`: retries a retryable failed phase under the same or narrower task boundaries.
- `GET /adapters/mcoda/tasks/{task_id}/results`: returns structured result, artifact, and log refs.
- `GET /adapters/mcoda/tasks/{task_id}/usage`: returns Overmeter and receipt refs by phase.
- `GET /adapters/mcoda/tasks/{task_id}/replay`: reconstructs validation, policy, route, tool, execution, and result decisions.

Mutating APIs require signed actor or service-account identity, tenant scope, trace id, idempotency key, task manifest version, policy refs, and tool/data boundary refs. Stable errors include `manifest_invalid`, `tool_boundary_missing`, `context_not_authorized`, `agent_capability_stale`, `route_unavailable`, `policy_denied`, `budget_precheck_failed`, `sandbox_profile_missing`, `task_not_retryable`, `cancel_not_possible`, and `result_validation_failed`.

## Event Surface

- `mcoda_adapter.task_validated`: manifest and boundary validation passed.
- `mcoda_adapter.task_rejected`: task rejected before side effects with reason codes.
- `mcoda_adapter.task_created`: task record created with manifest refs.
- `mcoda_adapter.tool_boundary_attached`: tool boundaries or grants attached.
- `mcoda_adapter.route_requested`: route request sent to AI Gateway Router or scheduler-facing services.
- `mcoda_adapter.workload_submitted`: Overrid workload refs created.
- `mcoda_adapter.phase_started`: agent phase started.
- `mcoda_adapter.tool_call_requested`: agent requested a bounded tool call.
- `mcoda_adapter.tool_call_denied`: tool call denied by policy, grant, or missing confirmation.
- `mcoda_adapter.phase_completed`: agent phase completed with result refs.
- `mcoda_adapter.task_completed`: task reached successful terminal state.
- `mcoda_adapter.task_failed`: task reached terminal failure with reason refs.
- `mcoda_adapter.task_cancelled`: cancellation completed or terminal cancellation recorded.
- `mcoda_adapter.usage_emitted`: usage refs emitted for Overmeter and wallet/receipt display.

Events carry refs, phase ids, reason codes, route ids, policy ids, tool ids/classes, usage ids, and redacted summaries. Events must not include raw private prompts, decrypted context, vault secrets, private repo contents, or unredacted logs.

## Core Workflow

1. Mcoda client submits a task package through SDK/CLI/API with expected output schema, requested tools, context refs, and resource hints.
2. Adapter normalizes the task, attaches actor/tenant/trace/idempotency envelope, and validates manifest compatibility.
3. Workload Classifier and Overguard evaluate workload class, data classes, tool boundaries, trust level, and budget precheck.
4. Adapter requests model/resource route from AI Gateway Router when model selection is needed.
5. Adapter creates an Overpack workload ref and submits the job through Overqueue.
6. Oversched, Overlease, Overcell, and Overrun handle placement and execution under the declared sandbox/tool boundaries.
7. Adapter receives phase updates, tool-call requests, logs/artifact refs, result refs, failures, and usage refs.
8. Adapter validates result schema, emits task completion or failure, and exposes results and replay to authorized clients.
9. Wallet/Usage Center and accounting services receive usage/receipt refs; Overclaim can use replay/evidence refs for disputes.

## State Machine

Task lifecycle:

1. `received`
2. `validated`
3. `policy_prechecked`
4. `route_selected`
5. `queued`
6. `leased`
7. `running`
8. `waiting_for_tool_grant`
9. `waiting_for_confirmation`
10. `result_validating`
11. `completed`
12. `failed`
13. `cancel_requested`
14. `cancelled`
15. `expired`

Phase lifecycle:

1. `planned`
2. `ready`
3. `running`
4. `waiting_for_tool`
5. `succeeded`
6. `retryable_failed`
7. `terminal_failed`
8. `skipped`
9. `cancelled`

Tool-call lifecycle:

1. `requested`
2. `policy_checked`
3. `grant_required`
4. `approved`
5. `executing`
6. `succeeded`
7. `denied`
8. `failed`

State transitions are append-only. Replays must reconstruct the effective task boundary and route from versioned refs rather than reading mutable current configuration.

## Policy And Security

- Tool access is deny-by-default; every allowed tool class requires a boundary declaration and policy decision.
- Side-effecting tool calls require explicit confirmation unless a narrow durable grant and policy allow unattended execution.
- Repo, Docdex, workspace, vault, wallet, messaging, and native-app access must use refs from owning services; the adapter cannot read private stores directly.
- Route fallback cannot widen tool, data, model, network, or filesystem permissions.
- Agent outputs that request side effects must be treated as proposals until tool policy and confirmation gates pass.
- Raw prompts, private context, decrypted RAG, secrets, and private logs must be redacted or stored by owning services as refs.
- Repair/retry loops require bounded attempt counts, unchanged or narrower permissions, and distinct usage records.
- Operator overrides require signed action envelopes and Overwatch evidence.

## Metering And Accounting

- Emit usage refs for validation, classification, route selection, queue time, model/resource execution, tool calls, storage, bandwidth, retries, result validation, and cleanup.
- Link usage to tenant, actor, Mcoda task ref, agent ref, route ref, workload ref, phase id, tool-call id, result ref, and wallet receipt ref.
- Failed, denied, cancelled, and retryable phases should produce visible reason and usage records where material.
- The adapter does not maintain balances, prices, ledger truth, grants, payouts, or invoices; it emits refs for Overmeter, ORU Account Service, Seal Ledger, Overbill, and Wallet/Usage Center.
- Do not encode hardcoded prices, provider payouts, financial projections, or per-transaction economics.

## Observability And Operations

- Expose task volume, validation failures, policy denials, route failures, queue latency, lease latency, phase latency, tool-denial rate, retry rate, cancellation rate, result-validation failures, and usage by resource class.
- Provide operator-safe logs with raw private content removed and refs preserved.
- Alert on tool-boundary bypass attempts, repeated route fallback loops, high policy-denial rates, missing usage refs, missing result refs, and tasks running past declared limits.
- Provide task replay views showing manifest, boundary declarations, policy decisions, route decisions, execution refs, tool-call refs, result refs, and usage refs.
- Support migration/backfill of manifest schema versions with explicit compatibility reports.

## Failure Modes And Recovery

- Invalid manifest: reject before side effects with field-level reason codes.
- Missing tool boundary: hold task in `waiting_for_tool_grant` or deny if the tool is required.
- Stale agent capability snapshot: refresh capability or require caller to resubmit under current facts.
- AI route unavailable: queue, retry, or fail according to fallback policy without widening permissions.
- Policy denies a tool or data class: record denial and continue only if the task can proceed without it.
- Overqueue/Oversched/Overrun failure: preserve idempotency, phase state, and retryability before retrying.
- Result schema invalid: mark result validation failed and allow bounded repair if policy permits.
- Usage emission failure: hold completion visibility until reconciliation refs exist or mark usage pending.
- Cancellation race: preserve current phase state and record whether cancellation reached queue, lease, runner, or completed task.

## Validation Plan

- Mcoda can submit a real task through the adapter and receive a structured result ref without direct internal API calls.
- Tool boundaries are required and visible to policy before execution.
- Missing, broad, or revoked tool grants block side effects.
- Route selection uses AI Gateway Router or scheduler-facing refs rather than hardcoded provider logic.
- Agent phases capture logs/artifacts/results/failure reasons with redaction and replay refs.
- Retry/cancel flows remain idempotent and bounded.
- Usage refs are emitted per phase and visible through wallet/accounting paths.
- Tenant isolation tests prove one tenant cannot read another tenant's task, context, logs, artifacts, or usage.
- Replay reconstructs manifest validation, policy decisions, route selection, tool decisions, execution refs, and result validation.

## Build Breakdown

1. Define the `agent_task_manifest`, `tool_boundary_declaration`, result schema, and stable error codes.
2. Implement `validate`, `create`, `start`, `status`, `cancel`, `results`, `usage`, and `replay` APIs against local fixtures.
3. Integrate Workload Classifier, Overguard prechecks, and Policy Dry-Run preview.
4. Integrate AI Gateway Router for model/resource route refs.
5. Generate Overpack workload refs and submit through Overqueue for a seed-hardware Phase 6 path.
6. Capture phase updates, tool-call decisions, result refs, failure refs, and usage refs.
7. Add Admin/Developer UI and CLI visibility for task state, tool grants, replay, and receipts.
8. Harden cancellation, retry, redaction, schema migration, and missing-dependency handling.

## Handoff And Downstream Use

Mcoda Adapter proves that agent work can run through Overrid without a privileged product-specific path. It gives Phase 6 a practical agent workload and gives later native apps, personal AI, and developer tooling a pattern for safe agent integration.

Downstream systems should call this adapter through its APIs and events, not by reading adapter storage. If Mcoda adds new agent capabilities, the catalog plan, this SDS, policy facts, and validation fixtures must be updated together.

## Open Design Questions

Resolved decisions:

- Route selection can treat only fresh, signed, policy-visible facts as authoritative: agent/provider identity refs, advertised usage class, model/resource capability class, modality/tool-support flags, context-window class, max-complexity class, health/quota/capacity refs, trust/locality class, compatible sandbox/runtime profile, and freshness timestamp from Overregistry, AI Gateway Router, Overguard, scheduler, or Mcoda inventory snapshots accepted into the adapter. Agent rating, historical success rate, cost hints, latency observations, user preference, popularity, self-described strengths, and free-form Mcoda metadata remain advisory; they may rank otherwise eligible candidates but cannot override missing capability, stale health, privacy, data-class, budget, trust, locality, or tool-boundary constraints.
- Durable unattended grants are allowed only for narrow, reversible, non-destructive tool classes whose boundaries are fully declared, time-bounded, tenant/actor scoped, resource capped, and replayable: read-only status/result/usage inspection, route/capability refresh, result-schema validation, redacted log/artifact fetch, cancellation of the same task, and deterministic cleanup of the adapter-owned sandbox or temporary refs. Per-call confirmation is always required for shell or code execution outside an accepted Overpack manifest, network egress widening, repo/file writes, vault/secret access, wallet/accounting/grant changes, messaging/native-app side effects, external publication, permission widening, deletion outside adapter-owned temporary state, policy/operator override, and any action that changes another service's authoritative state.
- Phase 6 minimum output support is a stable `agent_result_ref` envelope with `schema_version`, `task_id`, `phase_id`, selected route and capability snapshot refs, status, stable reason code, structured summary, machine-readable result payload refs, artifact/log refs, validation status, redaction profile, checksum/hash refs, usage refs, and replay refs. The initial supported payload shapes are `text_summary`, `json_object`, `artifact_set`, `tool_proposal`, and `failure_report`; richer multimodal, streaming, multi-agent, or native-app outputs must attach as typed extension refs without changing the base envelope or bypassing result validation.
- A repair inside the same Mcoda task is model self-correction only when it is triggered by retryable output-shape, validation, route-attempt, or transient execution failure and stays inside the same or stricter actor, tenant, context, data-class, tool, sandbox, route/fallback, budget, timeout, and output scopes. It receives a new `agent_phase_record`, route/attempt refs, result refs, and incremental usage refs under the original task. Any retry that widens permissions, adds context or tools, changes side-effect class, uses a less-local or higher-risk route, exceeds the configured automatic repair count, follows terminal policy denial, or changes the user-requested objective becomes a new signed billable task attempt with its own manifest, idempotency key, budget precheck, and receipt trail.
- Redacted log visibility is classed by audience. Users may see task state, selected route class, phase summaries, tool approvals/denials, result/artifact aliases, actionable failure messages, usage/receipt refs, and redacted excerpts for their own task. Operators may see cross-service reason codes, policy refs, sandbox/lease/route evidence, health and reconciliation diagnostics, and redacted stack traces under tenant and incident scope. Mcoda developers may see schema validation, adapter translation, capability mismatch, route/fallback, and agent-interface diagnostics with tenant/user/repo/vault/native-app details removed. Dispute reviewers may see immutable replay bundles, policy/usage/receipt refs, grant history, selected redacted logs, hashes, timestamps, and reason-code evidence. No audience gets raw private prompts, decrypted RAG context, private repo contents, vault secrets, exact hidden paths beyond authorized classes, other-tenant data, provider secrets, or unredacted native-app payloads through this adapter.
