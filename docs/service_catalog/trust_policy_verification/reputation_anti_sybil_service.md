# Reputation and Anti-Sybil Service Implementation Plan

## Objective

Protect the public pool from fake providers, repeated abuse, identity farming, payout fraud, and coordinated manipulation.

## First Build Phase

[Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md).

## Dependencies

- Public provider onboarding.
- Oververify challenge outcomes.
- Overwatch behavior history.
- Overbill payout records.

## Development Order

1. Define verification tiers and reputation records.
2. Add node uniqueness, payout account, network, and behavior signals where legally allowed.
3. Add rate limits for new or risky providers.
4. Add abuse throttles and payout hold triggers.
5. Add appeal and correction records.

## Contracts And Interfaces

- Reputation record schema.
- Sybil signal refs.
- Eligibility signal API.
- Appeal/correction contract.

## Detailed SDS

The detailed design contract lives in [Reputation and Anti-Sybil Service SDS](../../sds/trust_policy_verification/reputation_anti_sybil_service.md).

## Design Alignment

- Treat Reputation and Anti-Sybil Service as public-provider risk and eligibility recommendation infrastructure, not provider onboarding, final trust scoring, payout mutation, or incident response.
- Store reputation records, anti-Sybil signals, node/payout/network/behavior signals, risk windows, eligibility recommendations, throttles, payout-hold triggers, appeal/correction records, and replay bundles.
- Keep public providers restricted by default and ensure no recommendation can allow private, regulated, secret-bearing, or system-service workloads.
- Feed Oververify, Overguard, Oversched, Fraud Control Service, Provider Payout Service, Overbill, Overclaim, and central AI governance with evidence-backed recommendations.

## Validation

- New public providers are restricted by default.
- Abuse signals reduce eligibility and can trigger payout holds.
- Corrections can update reputation without deleting history.

## Handoff

This service feeds public provider scheduling, fraud control, payouts, and central AI governance evidence.
