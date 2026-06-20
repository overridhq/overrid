# Docdex Adapter Implementation Plan

## Objective

Make Docdex encrypted indexing, search, retrieval, and RAG context assembly first-class Overrid workloads.

## First Build Phase

[Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Dependencies

- Overpack.
- Overqueue.
- Overrun.
- Overstore.
- Overvault.
- Overmeter.

## Development Order

1. Define Docdex index, search, and retrieval workload manifests.
2. Add encrypted index storage refs.
3. Capture retrieval results and RAG context metadata.
4. Add model-routing metadata.
5. Emit usage rollups for index/search/retrieval operations.

## Contracts And Interfaces

- Docdex workload manifests.
- Encrypted index refs.
- Retrieval result schema.
- Usage event fields.

## Validation

- Docdex can submit real jobs through SDK/CLI.
- Results return without direct internal API calls.
- Usage records show index, search, and retrieval costs.

## Handoff

Docdex adapter proves data-and-AI workloads and feeds personal AI RAG.

## Detailed SDS

- [Docdex Adapter SDS](../../sds/adapters/docdex_adapter.md)

## Sub-Build Plan

- [SUB BUILD PLAN #65 - Docdex Adapter](../../build_plan/sub_build_plan_065_docdex_adapter.md)

## Design Alignment

The SDS refines this implementation plan as the Docdex workload adapter for instance refs, repo bindings, encrypted repo configs, index/search/retrieval/admin-ingest jobs, result refs, cleanup/deprovision jobs, usage refs, audit refs, and replay bundles. It keeps final context authorization in Encrypted Docdex RAG Adapter, model/resource decisions in AI Gateway Router, and Overrid identity/accounting truth in the core platform services. Phase 13 hardening must cover encrypted repo leakage facts, key failures, disabled structural features, admin-ingest plaintext cleanup, two-phase deprovision evidence, unauthorized repo binding access, usage reconciliation, incident response, and replay/audit gaps.
