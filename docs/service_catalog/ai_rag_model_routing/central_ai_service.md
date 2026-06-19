# Central AI Service Implementation Plan

## Objective

Coordinate ecosystem-level AI stewardship for fraud detection, grant recommendations, public-interest investment, policy evidence review, and governance reporting.

## First Build Phase

[Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) interface; stronger governance in [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md).

## Dependencies

- Overwatch evidence.
- Overclaim disputes.
- Overgrant pools.
- Seal Ledger.
- Stewardship reporting.

## Development Order

1. Define evidence package intake and privacy boundaries.
2. Add fraud and abuse risk scoring from evidence.
3. Add grant and public-interest recommendation support.
4. Add native app surplus routing recommendations.
5. Add appeal/dispute awareness and governance reports.

## Contracts And Interfaces

- Evidence package schema.
- Recommendation record.
- Intervention reason codes.
- Stewardship report refs.

## Validation

- Central AI decisions cite evidence and policy thresholds.
- Private data is not ingested without permission.
- Appeal/dispute paths are represented for interventions.

## Handoff

Central AI service supports bounded governance, anti-fraud work, public-interest investment, and stewardship reporting.

## Detailed SDS

- [Central AI Service SDS](../../sds/ai_rag_model_routing/central_ai_service.md)

## Design Alignment

The SDS defines Central AI Service as an evidence-bounded stewardship and recommendation layer, not an opaque enforcement authority. It owns evidence package intake, analysis jobs, risk assessments, grant/public-interest recommendations, intervention proposals, report refs, recommendation review state, corrections, retractions, and model/run provenance.

Build this service so recommendations cite evidence, policy thresholds, route refs, provenance, and appeal paths. Downstream services such as Fraud Control, Overclaim, Overgrant, Public-Interest Pool Service, Provider Payout Service, and Stewardship Reporting keep final mutation and dispute authority.
