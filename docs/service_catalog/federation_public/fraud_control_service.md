# Fraud Control Service Implementation Plan

## Objective

Detect provider fraud, workload abuse, payout abuse, result manipulation, and policy evasion.

## First Build Phase

[Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md).

## Dependencies

- Oververify.
- Overwatch.
- Overbill.
- Overclaim.
- Anti-Sybil service.

## Development Order

1. Define fraud signals and evidence records.
2. Add volume anomaly, challenge failure, result inconsistency, and payout-risk signals.
3. Trigger throttles, challenge tasks, and payout holds.
4. Add evidence packages for central AI and human review where needed.
5. Add correction and appeal integration.

## Contracts And Interfaces

- Fraud signal schema.
- Hold trigger contract.
- Evidence package format.
- Abuse throttle API.

## Validation

- Fraud signals can hold payouts without deleting history.
- False positives can be corrected through evidence.
- Public pool eligibility changes are auditable.

## Handoff

Fraud control protects public pools, provider payouts, native apps, and stewardship governance.

## Detailed SDS

The detailed design contract lives in [Fraud Control Service SDS](../../sds/federation_public/fraud_control_service.md).

## Design Alignment

- Treat Fraud Control Service as evidence-backed signal, risk-case, hold/throttle recommendation, challenge recommendation, and correction infrastructure.
- Keep final dispute adjudication, payout execution, ledger history, final reputation, challenge execution, scheduling, and workload execution in their owning services.
- Require source-trusted signals, rule versions, redacted evidence packages, Overguard policy refs, and correction/appeal paths before downstream action.
- Feed Provider Payout Service, Overbill, Overclaim, Overguard, Public-Interest Pool Service, Public Provider Onboarding, Reputation and Anti-Sybil Service, Challenge Task Service, and stewardship reporting with reason-coded fraud refs.
