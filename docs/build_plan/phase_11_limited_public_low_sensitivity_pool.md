# Phase 11: Limited Public Low-Sensitivity Pool

## Objective

Allow unknown or semi-trusted providers only where risk is bounded.

This phase is intentionally narrow. Public nodes should not run private, regulated, secret-bearing, or system-service workloads.

## Depends On

- Phase 10 trusted federation.
- Mature policy and verification systems.
- Dispute, hold, and payout controls.
- Public low-sensitivity workload class.

## Build Order

1. Define public provider onboarding.
2. Add anti-Sybil verification levels.
3. Create strict public workload eligibility rules.
4. Add public-node sandbox profile.
5. Add fraud controls and challenge tasks.
6. Add payout holds and throttles.
7. Add reputation and eligibility updates.
8. Prove low-sensitivity public workload execution.

## Workstream 1: Public Provider Onboarding

Collect and verify:

- Provider identity level.
- Node identity.
- Contact and payout eligibility where allowed.
- Resource claims.
- Software version.
- Jurisdiction or region.
- Accepted workload classes.
- Policy acknowledgement.

Onboarding should not imply broad trust. It only creates a starting eligibility level.

## Workstream 2: Anti-Sybil Controls

Add layered controls:

- Identity verification tiers.
- Device or node uniqueness checks where feasible.
- Payout account uniqueness signals.
- Network and behavior correlation.
- Challenge history.
- Reputation history.
- Rate limits for new providers.

Do not rely on a single anti-Sybil signal. Public supply is adversarial by default.

## Workstream 3: Strict Workload Eligibility

Public nodes may only receive:

- Public low-sensitivity workloads.
- No secrets.
- No private tenant data.
- No regulated data.
- No system-service workloads.
- Capped runtime.
- Capped resource allocation.
- Deny-by-default egress unless explicitly allowed.

Policy must make accidental leakage impossible through normal scheduler behavior.

## Workstream 4: Public Sandbox Profile

Create a hardened profile:

- No secret injection.
- Minimal filesystem.
- Network restrictions.
- Runtime cap.
- Memory cap.
- Output validation.
- Artifact quarantine where needed.
- Logs with privacy protection.

Assume public nodes can fail, cheat, or disappear.

## Workstream 5: Fraud And Challenge Controls

Use:

- Challenge workloads.
- Duplicate execution for selected jobs.
- Result consistency checks.
- Benchmark revalidation.
- Impossible-result detection.
- Abuse reports.
- Automated throttles.
- Manual/stewardship escalation for severe cases.

Fraud signals should affect eligibility and payout holds.

## Workstream 6: Payout Holds

Add hold logic based on:

- New provider status.
- Dispute window.
- Challenge failure.
- Result inconsistency.
- Abuse reports.
- Sudden volume changes.
- Policy violation.

Holds protect the ecosystem while disputes and verification mature.

## Workstream 7: Reputation Updates

Track:

- Successful workloads.
- Failed workloads.
- Timeliness.
- Challenge results.
- Disputes.
- Abuse signals.
- Provider responsiveness.
- Reversal or appeal outcomes.

Reputation must be explainable and correctable.

## Validation

- Unknown public node can register only into public provider class.
- Scheduler cannot place private, regulated, secret-bearing, or system-service work on public nodes.
- Public low-sensitivity job can run with sandbox restrictions.
- Failed verification reduces eligibility.
- Payout hold is created and released or corrected by evidence.

## Exit Gate

Phase 11 is complete when public supply can be used for limited low-sensitivity work without risking private data, backbone services, or user trust.

## Handoff To Phase 12

Phase 12 builds native applications on top of proven identity, storage, policy, metering, deployment, and trust rails.
