# Lightweight Classifier Implementation Plan

## Objective

Cheaply classify request intent, sensitivity, tool needs, model size, RAG need, and safety/policy hints before using larger resources.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md).

## Dependencies

- AI Gateway Router.
- Overguard policy labels and Workload Classifier facts.
- Encrypted Docdex RAG Adapter context and RAG-need handoffs.
- Personal AI Assistant and native app callers.
- ADES Enrichment Adapter where useful.
- Overregistry classifier/version refs, Overmeter usage refs, Overwatch audit refs, and Overbase/Overstore/Overvault state and artifact refs.

## Development Order

1. Define intent, sensitivity, tool, model-size, RAG, and safety labels.
2. Implement deterministic Rust heuristic classification with optional policy-allowed small local model classification.
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

## Sub-Build Plan

- [SUB BUILD PLAN #62 - Lightweight Classifier](../../build_plan/sub_build_plan_062_lightweight_classifier.md)

## Design Alignment

The SDS refines this implementation plan as an advisory pre-routing service. It owns label taxonomy, classifier versions, confidence thresholds, escalation records, evaluation fixtures, calibration reports, drift signals, rollout state, replay refs, and usage refs, while explicitly preventing classifier output from bypassing Overguard, Workload Classifier, AI Gateway Router, context authorization, user permissions, native app authority, or final safety review. Phase 13 hardening must cover false-negative sensitive classes, escalation bypass, rollout drift, privacy retention, and optional ADES/small-local-model supply-chain risks.
