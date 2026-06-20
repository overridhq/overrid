# ADES Enrichment Adapter Implementation Plan

## Objective

Use ADES as optional local semantic enrichment and domain-pack tagging for entity, topic, warning, and routing hints.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md).

## Dependencies

- ADES local service or package.
- AI gateway router.
- Personal AI assistant.
- Privacy policy.

## Development Order

1. Add connector for local ADES HTTP/service or library mode.
2. Add domain-pack selection.
3. Extract entities, topics, warnings, timing metadata, and routing hints.
4. Keep local execution mode for private text.
5. Feed hints into classifier/router without making ADES a protocol dependency.

## Contracts And Interfaces

- Enrichment request.
- Enrichment result.
- Domain pack metadata.
- Routing hint fields.

## Validation

- Private text can be enriched locally.
- Missing ADES is handled as optional degradation.
- Hints improve routing without replacing policy decisions.

## Handoff

ADES enrichment supports personal AI, model routing, and domain-specific assistant behavior.

## Detailed SDS

- [ADES Enrichment Adapter SDS](../../sds/ai_rag_model_routing/ades_enrichment_adapter.md)

## Sub-Build Plan

- [SUB BUILD PLAN #58 - ADES Enrichment Adapter](../../build_plan/sub_build_plan_058_ades_enrichment_adapter.md)

## Design Alignment

The SDS defines ADES Enrichment Adapter as an optional local semantic enrichment bridge, not a required protocol dependency or final routing/policy authority. It owns connector configuration, local health checks, approved domain-pack refs, enrichment request/result normalization, routing hint bundles, privacy guard records, degradation records, and usage refs.

Build this adapter so private text stays local, missing ADES or missing packs degrade cleanly, and ADES hints remain advisory inputs to Lightweight Classifier, AI Gateway Router, Personal AI Assistant, and encrypted Docdex RAG flows.
