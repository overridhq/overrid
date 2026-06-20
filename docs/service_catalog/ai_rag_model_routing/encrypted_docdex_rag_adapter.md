# Encrypted Docdex RAG Adapter Implementation Plan

## Objective

Connect encrypted Docdex indexes to Overrid workloads and personal, organization, and repository AI context retrieval.

## First Build Phase

[Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Dependencies

- Docdex encrypted index service.
- Overpack.
- Overqueue.
- Overstore.
- Overvault.
- Overmeter.

## Development Order

1. Add index job submission and result capture.
2. Add search and retrieval job adapters.
3. Store encrypted index and context refs.
4. Add tenant/person/org/repo scope checks.
5. Add usage rollups and access audit.

## Contracts And Interfaces

- Index job manifest.
- Retrieval request/result schema.
- Encrypted context refs.
- Usage rollup fields.

## Validation

- Retrieval only uses authorized encrypted indexes.
- Raw repo/personal content is not exposed to the router unnecessarily.
- Index/search/retrieval usage is metered.

## Handoff

Encrypted Docdex RAG becomes the knowledge substrate for personal AI, workspace, code-agent, and enterprise use cases.

## Detailed SDS

- [Encrypted Docdex RAG Adapter SDS](../../sds/ai_rag_model_routing/encrypted_docdex_rag_adapter.md)

## Sub-Build Plan

- [SUB BUILD PLAN #61 - Encrypted Docdex RAG Adapter](../../build_plan/sub_build_plan_061_encrypted_docdex_rag_adapter.md)

## Design Alignment

The SDS refines this implementation plan as the authorization and context-bundle boundary between Docdex indexes and Overrid AI flows. It keeps Docdex runtime work in Docdex Adapter, keeps final model selection in AI Gateway Router, records encrypted-index leakage facts, and requires context grants, redaction, usage refs, and replay evidence before private context reaches an assistant or code agent.
