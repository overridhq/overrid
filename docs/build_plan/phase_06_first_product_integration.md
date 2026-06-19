# Phase 6: First Product Integration

## Objective

Connect real ecosystem products to Overrid before expanding the infrastructure surface.

This phase proves that the control plane, private swarm, execution loop, policy, and ORU accounting can serve actual demand from Docdex, Mcoda, Codali, and the developer/admin toolchain.

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
6. Build admin and developer visibility.
7. Run product-driven retry, cancellation, metering, and receipt tests.

## Workstream 1: First Workload Selection

Prefer workloads that exercise the platform without requiring public marketplace trust:

- Docdex encrypted indexing.
- Docdex search/retrieval jobs.
- Model-routing support for personal or repo context.
- Mcoda agent execution.
- Codali code-agent package execution.

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

- Encrypted repository index jobs.
- Search and retrieval jobs.
- RAG context assembly.
- Model-routing metadata.
- Cloud-capable encrypted index storage where appropriate.
- Tenant and repo scoped access.
- Usage rollups linked to index, search, retrieval, and model operations.

Docdex integration should prove the personal AI assistant path later, because encrypted Docdex indexes are the RAG substrate for people, organizations, and repos.

## Workstream 4: Mcoda Integration

Integrate Mcoda agent workloads:

- Agent task packaging.
- Model or resource selection.
- Tool-use boundary declaration.
- Execution result capture.
- Usage reporting.
- Failure reason propagation.
- Agent-level budget and policy checks.

The goal is to prove that Overrid can become the resource plane behind agent execution without hardcoding one model or provider.

## Workstream 5: Codali Integration

Integrate Codali/code-agent workloads:

- Package execution.
- Repository context references.
- Logs and artifacts.
- Structured result capture.
- Retry or repair loops where safe.
- Resource usage per agent phase.

This should demonstrate practical code-agent execution on private nodes and provide the first clear developer productivity proof.

## Workstream 6: Developer And Admin UI

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

## Workstream 7: Product Reliability Tests

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

Each case should leave readable audit, usage, and receipt trails.

## Validation

- At least one real product submits jobs through SDK/CLI.
- Product can retrieve result without direct internal API calls.
- Usage appears in ORU/Seal Ledger records.
- Admin can inspect job state, policy, node, audit, and receipt.
- Product failure paths produce actionable reason codes.

## Exit Gate

Phase 6 is complete when Docdex, Mcoda, or Codali can use Overrid for real private work and the operator can trace each job from request to execution to accounting.

## Handoff To Phase 7

Phase 7 moves core backbone services from founder seed machines into protected grid-resident system workloads.
