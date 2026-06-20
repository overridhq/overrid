# Phase 6: First Product Integration

## Objective

Connect real ecosystem products to Overrid before expanding the infrastructure surface.

This phase proves that the control plane, private swarm, execution loop, policy, and ORU accounting can serve actual demand from Docdex, Mcoda, Codali, mSwarm runtime bridging, and the developer/admin toolchain.

## Depends On

- Phase 5 accountable workload execution.
- Product candidates with real workloads.
- SDK/CLI skeleton from earlier phases.

## Build Order

1. Choose first workload family.
2. Harden SDK and CLI for external product use.
3. Integrate Docdex encrypted RAG jobs.
4. Integrate Mcoda agent workloads.
5. Integrate Codali/code-agent workloads.
6. Integrate mSwarm runtime bridge sessions, sync, discovery, collaboration, and cloud coordination hooks.
7. Build admin and developer visibility.
8. Run product-driven retry, cancellation, metering, and receipt tests.

## Workstream 1: First Workload Selection

Prefer workloads that exercise the platform without requiring public marketplace trust:

- Docdex encrypted indexing.
- Docdex search/retrieval jobs.
- Docdex Adapter instance/repo binding, admin-ingest, capability, result, cleanup, usage, and replay refs.
- Encrypted Docdex RAG Adapter authorization, leakage profiles, context grants, context bundle refs, and replay evidence.
- AI Gateway Router model-routing support for personal, organization, workspace, or repo context.
- Mcoda agent execution.
- Codali code-agent package execution.
- mSwarm Runtime Bridge sessions, capability snapshots, sync manifests, private discovery, collaboration refs, and cloud hooks for local-first product/runtime coordination.

The first workload should have clear inputs, outputs, resource usage, and success criteria.

## Workstream 2: SDK And CLI

Create stable developer commands:

- Login or credential enrollment.
- Submit workload.
- Validate manifest.
- Inspect job status.
- Cancel job.
- Fetch result.
- Show usage.
- Show receipts.
- Register node.
- Inspect node health.

The SDK should wrap signing, idempotency, trace ids, retries, and error decoding so products do not reimplement protocol details.

## Workstream 3: Docdex Integration

Use Docdex as the first serious data-and-AI workload:

- Docdex Adapter instance refs, repo bindings, encrypted repo config refs, capability snapshots, job refs, result refs, admin-ingest refs, cleanup/deprovision refs, usage refs, audit refs, and replay bundles.
- Encrypted repository index jobs.
- Search and retrieval jobs.
- Retrieval-only RAG result refs handed to Encrypted Docdex RAG Adapter for authorization and context-bundle creation.
- Encrypted Docdex RAG Adapter authorization, leakage profiles, context grants, context bundle refs, and replay evidence.
- AI Gateway Router route requests, dry-runs, decisions, and model-routing metadata.
- Protected encrypted index/result refs through native Overrid storage and vault boundaries where appropriate.
- Tenant and repo scoped access.
- Usage rollups linked to index, search, retrieval, context assembly, and model operations.

Docdex integration should prove the personal AI assistant path later, because encrypted Docdex indexes are the RAG substrate for people, organizations, and repos.

## Workstream 4: Mcoda Integration

Integrate Mcoda agent workloads:

- Agent task manifests with actor, tenant, Mcoda task, requested agent, output schema, budget, timeout, retry, and trace refs.
- Agent capability snapshots accepted from Mcoda inventory, AI Gateway Router, Overregistry, Overguard, scheduler, or runtime health refs.
- Tool-boundary declarations, grant refs, per-call confirmations, denied-tool records, and side-effect class handling.
- Context-access plans for repo, Docdex, encrypted RAG, workspace/app, vault, native-app, and external-resource refs through owning services.
- AI Gateway Router route requests, route decisions, fallback refs, model/resource candidates, and route replay evidence.
- Overpack workload refs submitted through Overqueue, Oversched, Overlease, Overrun, and Overcell.
- Agent phase records, tool-call records, structured result refs, artifact/log refs, failure refs, cancellation refs, and bounded repair/retry refs.
- Usage reporting per validation, route selection, queue wait, execution, tool call, retry, repair, storage, bandwidth, cleanup, denial, and cancellation.
- Replay bundles that reconstruct manifest validation, policy, route, tool, execution, result, and usage decisions.

The goal is to prove that Overrid can become the resource plane behind agent execution without hardcoding one model or provider.

## Workstream 5: Codali Integration

Integrate Codali/code-agent workloads:

- Overpack-compatible code-agent manifests with declared tool, sandbox, model-capability, validation, and output boundaries.
- Authorized repository context references through Encrypted Docdex RAG Adapter or Docdex Adapter, not direct raw repository reads.
- Private/trusted-node sandbox execution for the first bounded test-generation proof.
- Patch, validation, log, artifact, structured result, approval-handoff, and replay refs.
- Bounded retry or repair loops where policy, budget, context, and approval allow.
- Resource usage per agent phase, including context retrieval, model routes, queue, lease/runtime, validation, storage, artifacts, and repairs.

This should demonstrate practical code-agent execution on private nodes and provide the first clear developer productivity proof.

## Workstream 6: mSwarm Runtime Bridge

Integrate mSwarm local-first runtime coordination without bypassing Overrid rails:

- Bridge session records that bind mSwarm runtime sessions to Overrid actor, tenant, device, client, app, credential, route, and audit refs.
- Runtime capability snapshots with feature-level freshness, compatibility, supported sync modes, encryption modes, storage adapters, route classes, offline support, health refs, and degraded states.
- Sync manifest validation, sync cursors, checkpoints, conflict summaries, offline reconciliation bounds, policy denial refs, and app-owned resolution handoffs.
- Private Phase 6 discovery announcements for tenant/app/device scoped endpoints, capability refs, private route refs, session health, and retryable cloud-hook targets.
- Collaboration session refs and cloud coordination hooks with permission manifests, participant refs, idempotency keys, retry limits, redacted payload refs, failure refs, usage refs, audit refs, and replay bundles.

This should prove local-first product/runtime coordination while Overrid remains the authority for identity, tenancy, permissions, policy, storage, vault access, usage, and audit.

## Workstream 7: Developer And Admin UI

Build a simple operational interface for:

- Tenants.
- Identities.
- Nodes.
- Workloads.
- Queue state.
- Policy decisions.
- Verification evidence.
- Usage rollups.
- ORU balances and holds.
- Disputes.
- Receipts.

This UI can be utilitarian. It exists to debug and operate the system, not to market it.

## Workstream 8: Product Reliability Tests

Run product-driven cases:

- Successful job.
- Retryable failure.
- Final failure.
- Cancellation.
- Timeout.
- Policy denial.
- Budget exhaustion.
- Node disconnect.
- Disputed usage.
- Stale runtime capability snapshot.
- Offline reconciliation denial.
- Discovery visibility denial.
- Collaboration participant denial.
- Cloud hook retry exhaustion.

Each case should leave readable audit, usage, and receipt trails.

## Validation

- At least one real product submits jobs through SDK/CLI.
- Product can retrieve result without direct internal API calls.
- Usage appears in ORU/Seal Ledger records.
- Admin can inspect job state, policy, node, audit, and receipt.
- Product failure paths produce actionable reason codes.
- mSwarm bridge sessions, sync/discovery/collaboration/hook records are auditable and replayable without raw private payload exposure.

## Exit Gate

Phase 6 is complete when Docdex, Mcoda, Codali, or mSwarm runtime bridge flows can use Overrid for real private work and the operator can trace each job or runtime coordination record from request to execution or handoff to accounting.

## Handoff To Phase 7

Phase 7 moves core backbone services from founder seed machines into protected grid-resident system workloads.
