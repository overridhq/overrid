# Lightweight Classifier Implementation Plan

## Objective

Cheaply classify request intent, sensitivity, tool needs, model size, RAG need, and safety/policy hints before using larger resources.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md).

## Dependencies

- AI gateway router.
- Overguard policy labels.
- ADES enrichment adapter where useful.

## Development Order

1. Define intent, sensitivity, tool, model-size, RAG, and safety labels.
2. Implement local or cheap model classification path.
3. Add confidence thresholds and fallback to larger routing analysis.
4. Add classifier evidence to routing decisions.
5. Add evaluation fixtures from real assistant tasks.

## Contracts And Interfaces

- Classification request.
- Classification result.
- Confidence and fallback fields.
- Routing hint contract.

## Validation

- Low-risk simple tasks route cheaply.
- Low-confidence or sensitive tasks escalate.
- Classification output does not bypass policy.

## Handoff

The classifier lowers compute waste for personal AI, gateway routing, and native app AI features.

## Detailed SDS

- [Lightweight Classifier SDS](../../sds/ai_rag_model_routing/lightweight_classifier.md)

## Design Alignment

The SDS refines this implementation plan as an advisory pre-routing service. It owns label taxonomy, classifier versions, confidence thresholds, escalation records, evaluation fixtures, calibration reports, and usage refs, while explicitly preventing classifier output from bypassing Overguard, AI Gateway Router, context authorization, user permissions, or final safety review.
