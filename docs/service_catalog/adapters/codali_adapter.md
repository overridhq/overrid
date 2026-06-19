# Codali Adapter Implementation Plan

## Objective

Run Codali/code-agent workloads through Overrid with repository context refs, logs, artifacts, structured results, and usage capture.

## First Build Phase

[Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Dependencies

- Overpack.
- Overrun.
- Overstore.
- Overmeter.
- Overwatch.
- Encrypted Docdex RAG adapter where repo context is needed.

## Development Order

1. Define code-agent package manifest.
2. Add repository context refs and artifact refs.
3. Capture logs and structured results.
4. Bound retry/repair loops with policy and budget checks.
5. Meter resource usage per agent phase.

## Contracts And Interfaces

- Code-agent manifest.
- Repository context ref.
- Artifact/log refs.
- Structured result schema.

## Validation

- Codali can execute a real private code-agent job.
- Logs and artifacts are retrievable.
- Agent phase usage is metered and auditable.

## Handoff

Codali adapter proves developer-productivity workloads and exercises Overrid's AI/code execution loop.

## Detailed SDS

- [Codali Adapter SDS](../../sds/adapters/codali_adapter.md)

## Design Alignment

The SDS refines this implementation plan as code-agent workload packaging and result capture. It owns code-agent manifests, repo context refs, sandbox/tool boundaries, phase logs, artifacts, structured results, bounded repair loops, and usage refs, while preventing arbitrary repo writes, direct Git ownership, context bypass, and unsandboxed agent execution.
