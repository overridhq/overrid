# AI Gateway Router Implementation Plan

## Objective

Route AI requests to the right model, node, provider, or local resource based on request nature, privacy, budget, availability, and capability.

## First Build Phase

[Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md) for product routing; [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) for the native assistant.

## Dependencies

- Overregistry model/resource records.
- Oversched.
- Overguard.
- Overmeter.
- Lightweight classifier.

## Development Order

1. Define request classification and route decision schema.
2. Add model capability matching and resource availability lookup.
3. Add privacy, budget, and policy checks.
4. Add fallback and retry policies.
5. Emit routing audit and usage events.

## Contracts And Interfaces

- AI route request.
- Route decision record.
- Model/provider capability refs.
- Fallback policy.

## Validation

- Router avoids hardcoded model names.
- Privacy-restricted requests do not route to disallowed resources.
- Route decisions are auditable and metered.

## Handoff

AI gateway router supports personal AI, Docdex, Mcoda, Codali, central AI, and native apps.

## Detailed SDS

- [AI Gateway Router SDS](../../sds/ai_rag_model_routing/ai_gateway_router.md)

## Design Alignment

The SDS defines AI Gateway Router as the route-decision authority for AI requests, not a model runner, conversation store, RAG index reader, policy engine, or accounting service. It owns route requests, classification fact bundles, context access plans, capability snapshots, route decisions, route attempts, fallback policies, usage refs, and replay evidence.

Build routing around registry/capability refs rather than hardcoded model names. Fallback must never weaken tenant, role, privacy, data-class, context authorization, locality, or budget constraints.
