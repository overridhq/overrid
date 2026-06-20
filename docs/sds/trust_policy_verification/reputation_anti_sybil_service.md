SDS #36

# Reputation and Anti-Sybil Service SDS

## Purpose

Protect the public low-sensitivity pool from fake providers, identity farming, node duplication, payout fraud, coordinated manipulation, abuse clusters, and repeated bad behavior.

Reputation and Anti-Sybil Service is a Phase 11 public-supply hardening service. It turns provider onboarding facts, verification evidence, challenge outcomes, behavior history, payout refs, dispute outcomes, and fraud-control signals into explainable risk, eligibility, throttle, and payout-hold recommendations.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [reputation_anti_sybil_service.md](../../service_catalog/trust_policy_verification/reputation_anti_sybil_service.md) |
| Sub-build plan | [SUB BUILD PLAN #36 - Reputation and Anti-Sybil Service](../../build_plan/sub_build_plan_036_reputation_anti_sybil_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md) |

## Service Family

- Family: Trust, policy, verification, and disputes
- Owning layer: Public-provider reputation, anti-Sybil evidence, risk scoring, eligibility recommendations, throttles, hold triggers, and correction history
- Primary data scope: provider reputation records, node uniqueness signals, payout uniqueness signals, network/behavior correlation refs, challenge history, abuse reports, risk windows, eligibility recommendations, throttle records, payout-hold trigger refs, appeal/correction records, and explanation bundles
- First build phase from service plan: [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md).

## Problem Statement

Public supply is adversarial by default. A public provider can register many identities, clone nodes, share payout rails across fake accounts, farm onboarding incentives, fake capacity, pass one challenge and fail real work, coordinate with other providers, or disappear before disputes settle.

Overrid must use layered signals rather than a single identity check. Reputation and Anti-Sybil Service must make public-provider restrictions explainable and correctable while still protecting private data, system-service workloads, regulated workloads, provider payouts, and user trust.

## Goals

- Track provider, node, payout, network, behavior, challenge, dispute, and abuse signals for public-provider risk.
- Produce explainable anti-Sybil and reputation signals with evidence refs, confidence, expiry, and correction state.
- Restrict new or risky public providers by default until evidence justifies broader low-sensitivity eligibility.
- Feed Overguard and Oversched with public-provider eligibility, throttle, duplicate-execution, and sandbox recommendations.
- Trigger payout hold recommendations when risk crosses configured thresholds.
- Support appeal, correction, expiry, and reversal without deleting signal history.
- Keep fraud heuristics protected while providing providers with actionable reason codes.
- Preserve Phase 11 limits: public nodes only run public low-sensitivity workloads with no secrets, no private tenant data, no regulated data, no system-service work, capped runtime, capped resources, and deny-by-default egress.

## Non-Goals

- Do not onboard providers. Public Provider Onboarding owns initial registration and onboarding workflow.
- Do not replace Oververify trust records or provider/node eligibility. This service publishes risk and anti-Sybil signals that Oververify and policy consumers use.
- Do not execute fraud investigations or incidents alone. Fraud Control Service and Incident Response Service handle broader cases.
- Do not directly hold payouts or edit ledger records. Provider Payout Service, Overbill, Seal Ledger, and Overclaim own financial state and disputes.
- Do not reveal raw fraud heuristics, private tenant data, or provider-private correlation data in provider-facing output.
- Do not permit public providers to receive private, regulated, secret-bearing, or system-service workloads.
- Do not create speculative reputation markets, tokenized reputation, NFTs, blockchain mechanics, or per-operation external payment calls.

## Primary Actors And Clients

- Public Provider Onboarding, submitting identity, node, contact, payout, jurisdiction, and policy-acknowledgement refs.
- Oververify, consuming risk signals and publishing final verification/eligibility changes.
- Overguard, consuming public-provider policy signals for workload admission and provider eligibility.
- Oversched, consuming scheduling throttle, duplicate-execution, sandbox, and public workload eligibility refs.
- Challenge Task Service, supplying challenge history and receiving required challenge cadence.
- Fraud Control Service, supplying abuse reports and consuming cluster/risk evidence.
- Provider Payout Service, Overbill, Seal Ledger, and Overclaim, consuming hold recommendations, dispute refs, correction refs, and release refs.
- Overwatch, supplying behavior history and storing evidence/audit events.
- Central AI stewardship, admin UI, CLI, and SDK, reading authorized summaries, explanations, and remediation steps.

## Dependencies

- [Public Provider Onboarding](../federation_public/public_provider_onboarding.md) for provider identity, node identity, contact, payout, jurisdiction, accepted workload class, and policy acknowledgement refs.
- [Oververify](oververify.md) for provider/node verification records, challenge outcomes, trust signals, and eligibility state.
- [Challenge Task Service](challenge_task_service.md) for challenge cadence, failure refs, duplicate-execution findings, and consequence proposals.
- [Overwatch](../control_plane/overwatch.md) for behavior history, workload outcomes, audit events, incidents, and evidence bundles.
- [Overclaim](overclaim.md) for appeal, correction, dispute, reversal, and finality refs.
- [Overguard](overguard.md) for public-provider policy decisions and workload-class restrictions.
- [Oversched](../execution_scheduling/oversched.md) for placement throttle, no-candidate reason, duplicate-run, and sandbox hints.
- [Provider Payout Service](../accounting/provider_payout_service.md), [Overbill](../accounting/overbill.md), and [Seal Ledger](../accounting/seal_ledger.md) for payout, hold, release, refund, and correction refs.
- [Fraud Control Service](../federation_public/fraud_control_service.md) for fraud reports, abuse clusters, and escalation outcomes.

## Owned Responsibilities

Reputation and Anti-Sybil Service owns:

- Public-provider reputation records and risk windows.
- Anti-Sybil signal references and evidence weights.
- Node uniqueness, payout uniqueness, network correlation, and behavior correlation summaries where legally allowed.
- Risk state transitions, throttle recommendations, duplicate-execution requirements, and public workload eligibility recommendations.
- Payout-hold trigger recommendations with evidence refs.
- Appeal, correction, signal expiry, and reversal records.
- Redacted provider-facing explanations and operator-facing evidence timelines.

The service must keep signals as evidence-backed records. It must not publish unexplained global reputation numbers as the only eligibility input.

## Data Model

The first implementation should define:

- `public_provider_reputation_record`: provider id, onboarding refs, verification refs, current reputation state, allowed workload classes, restriction refs, confidence, last recompute, and explanation bundle ref.
- `anti_sybil_signal`: signal id, target provider/node/payout ref, signal domain, evidence refs, confidence, severity, expiry, visibility class, correction state, and source service.
- `node_uniqueness_signal`: node id, hardware fingerprint refs where legally allowed, enrollment refs, duplicate suspicion, confidence, challenge refs, and privacy redaction profile.
- `payout_uniqueness_signal`: provider id, payout account hash/ref, duplicate payout cluster ref, compliance visibility, hold recommendation refs, and correction refs.
- `network_behavior_signal`: network cluster ref, behavior window, correlated timing, no-show/failure pattern, challenge-response pattern, abuse refs, and confidence.
- `reputation_window`: target id, time window, completed workloads, failed workloads, challenge outcomes, disputes, reversals, responsiveness, payout events, and abuse markers.
- `eligibility_recommendation`: target id, recommended eligibility state, allowed workload classes, required sandbox profile, duplicate-execution requirement, throttle limit, challenge cadence, hold trigger refs, reason codes, and expiry.
- `public_provider_throttle`: target id, throttle type, limit, cooldown, triggering evidence, appeal refs, expiry, and downstream consumer refs.
- `payout_hold_trigger`: provider id, payout refs, severity, evidence refs, proposed hold scope, dispute window, release condition, and downstream response refs.
- `appeal_correction_record`: target signal/ref, claimant, claim refs, correction reason, old state, new state, resolver refs, and audit refs.
- `reputation_explanation_bundle`: target id, user-safe reason codes, operator evidence refs, redaction profile, remediation hints, and replay bundle ref.
- `signal_replay_bundle`: input signals, policy version, evaluator version, risk calculation, recommendation refs, and Overwatch event refs.

Common envelope fields:

- `id`, `provider_id`, `node_id` where applicable, `tenant_id` where scoped, `trace_id`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

The first API surface should be internal and operator-facing:

- `POST /reputation/public-providers/{provider_id}/signals`: ingest anti-Sybil, abuse, payout, challenge, dispute, or behavior signal refs.
- `POST /reputation/public-providers/{provider_id}/recompute`: recompute reputation and eligibility recommendations.
- `GET /reputation/public-providers/{provider_id}`: read authorized reputation state and restrictions.
- `GET /reputation/public-providers/{provider_id}/eligibility`: read Overguard/Oversched-ready eligibility recommendations.
- `GET /reputation/public-providers/{provider_id}/explain`: read redacted provider-facing or operator-facing explanation.
- `POST /reputation/public-providers/{provider_id}/appeals`: open an appeal or correction request through Overclaim refs.
- `POST /reputation/public-providers/{provider_id}/throttles`: create, update, expire, or release throttle recommendations.
- `POST /reputation/public-providers/{provider_id}/hold-triggers`: create payout-hold trigger recommendations.
- `POST /reputation/replay/{recommendation_id}`: reconstruct recommendation from stored refs and policy version.

API requirements:

- Mutating endpoints require service identity, trace id, idempotency key, and evidence refs.
- Provider-facing reads must redact sensitive fraud heuristics and correlation data.
- Eligibility reads must be deterministic and include expiry.
- Signal ingest must reject unsupported or unauthenticated signal sources.

## Event Surface

- `reputation.signal_ingested`: new signal ref accepted.
- `reputation.signal_rejected`: signal rejected with reason code.
- `reputation.recomputed`: reputation state and eligibility recommendation recomputed.
- `reputation.eligibility_changed`: public-provider eligibility recommendation changed.
- `reputation.throttle_recommended`: throttle recommendation created or changed.
- `reputation.payout_hold_recommended`: payout hold trigger emitted.
- `reputation.challenge_cadence_changed`: required challenge cadence changed.
- `reputation.appeal_opened`: appeal or correction started.
- `reputation.correction_applied`: corrected signal or recommendation recorded.
- `reputation.replay_completed`: replay completed for audit.

Events must include target id, signal refs, reason codes, policy/evaluator version, visibility class, and trace id.

## Core Workflow

1. Public provider onboarding creates provider/node/payout refs.
2. Verification, challenge, behavior, payout, dispute, and abuse services submit signal refs.
3. Reputation and Anti-Sybil Service validates signal authenticity, legal/visibility scope, target identity, and freshness.
4. The service computes risk windows and anti-Sybil signal clusters.
5. It publishes eligibility, throttle, challenge-cadence, duplicate-execution, and payout-hold recommendations.
6. Oververify consumes the recommendation and publishes final trust/eligibility signals.
7. Overguard and Oversched consume final eligibility and restriction refs for public low-sensitivity work only.
8. Provider Payout Service and Overbill consume hold recommendations through their own hold/dispute contracts.
9. Providers may appeal or correct signals through Overclaim, producing new signal versions instead of deleting history.

## State Machine

Reputation state lifecycle:

1. `new_public_provider`: provider has limited initial eligibility.
2. `collecting_signals`: onboarding, verification, behavior, payout, and challenge refs are accumulating.
3. `restricted`: provider is allowed only narrow public low-sensitivity work or test work.
4. `eligible_public_low_sensitivity`: provider can receive bounded public low-sensitivity work.
5. `throttled`: provider is rate-limited because of risk or immature evidence.
6. `duplicate_execution_required`: selected work requires duplicate execution or result checks.
7. `hold_recommended`: payout hold trigger is active.
8. `probation`: provider remains usable only under strict restrictions.
9. `suspended`: provider is not eligible for new work.
10. `appeal_open`: provider has an active correction or appeal.
11. `corrected`: prior signal was corrected and recommendations were recomputed.
12. `expired`: recommendation expired and must be recomputed.

Signal lifecycle:

1. `received`: signal ref submitted.
2. `validated`: source, target, freshness, and legal scope passed.
3. `active`: signal contributes to recommendations.
4. `disputed`: signal is under appeal or correction.
5. `corrected`: replacement signal or reversal recorded.
6. `expired`: signal no longer contributes unless policy retains it as history.
7. `revoked`: source withdrew or invalidated the signal.

## Policy And Security

- Public providers are restricted by default until evidence supports eligibility.
- A single anti-Sybil signal must not be treated as final truth; recommendations should cite layered evidence and confidence.
- Public providers can never become eligible for private tenant, regulated, secret-bearing, or system-service workloads through this service.
- Payout-account and identity signals must be stored as hashes or refs where possible, not raw sensitive data.
- Fraud heuristics and cluster details require operator-only visibility.
- Provider-facing explanations must include appealable reason codes and safe remediation hints.
- Corrections must append new signal/recommendation records rather than deleting history.
- Manual risk overrides require signed action, evidence refs, expiry, and Overwatch audit.

## Metering And Accounting

The service does not bill, settle, or hold funds. It emits accounting-relevant recommendations:

- Payout hold trigger refs with evidence and release conditions.
- Throttle refs that may reduce public-provider earning velocity.
- Risk and eligibility refs for Provider Payout Service, Overbill, Seal Ledger, and Overclaim.
- Internal usage facts for signal ingest, recompute, explanation export, and replay.

Financial effects must be executed by the owning accounting service through append-only records.

## Observability And Operations

- Dashboards should show new public providers, restricted providers, eligible providers, throttle counts, hold recommendations, appeal rates, corrected signals, cluster risk, and recompute freshness.
- Operators need evidence timelines that join onboarding, verification, challenge, workload, payout, dispute, and abuse refs.
- Alerts should fire on suspected Sybil clusters, sudden public-provider volume spikes, repeated challenge failures, payout-account reuse clusters, and high false-positive correction rates.
- Recompute jobs must be replayable and support scoped backfills after rule changes.
- Retention policy must preserve enough signal history for fraud review without retaining unnecessary sensitive raw data.

## Failure Modes And Recovery

- Missing onboarding refs: keep provider restricted.
- Conflicting signals: apply the stricter recommendation or require review depending on severity.
- Signal source outage: keep existing recommendation until expiry, then restrict or require review.
- False positive cluster: open correction, create replacement signal, recompute recommendations, and preserve history.
- Payout hold downstream denial: record denial refs and let Overclaim or Provider Payout Service determine remedy.
- Abuse report flood: rate-limit signal source and require authenticated evidence refs.
- Recompute failure: keep prior recommendation, mark stale, and alert operators.

## Validation Plan

The service implementation plan lists these requirements:

- New public providers are restricted by default.
- Abuse signals reduce eligibility and can trigger payout holds.
- Corrections can update reputation without deleting history.

Additional SDS-level validation:

- Contract tests for signal ingest, recompute, eligibility read, explain, appeal, throttle, hold trigger, and replay APIs.
- Fixture tests for identity tier, node uniqueness, payout account, network behavior, challenge history, dispute history, and abuse marker signals.
- Public-provider restriction tests proving no signal path can allow private, regulated, secret-bearing, or system-service placement.
- Redaction tests for provider-facing and operator-facing explanations.
- Correction tests proving updated recommendations are appended and old signals remain auditable.
- Payout integration tests proving hold triggers are recommendations and do not mutate balances directly.
- Replay tests proving stored signal refs and policy versions reproduce recommendations.

## Build Breakdown

1. Define provider reputation, anti-Sybil signal, risk window, eligibility recommendation, throttle, hold trigger, appeal/correction, explanation, and replay schemas.
2. Ingest onboarding, verification, challenge, behavior, payout, dispute, and abuse signal refs.
3. Implement new-provider restricted defaults.
4. Add risk window recomputation and eligibility recommendation output.
5. Add throttle, duplicate-execution, challenge-cadence, and payout-hold trigger recommendations.
6. Add provider-facing redacted explanations and operator evidence views.
7. Add appeal/correction flow through Overclaim.
8. Add replay, recompute backfill, and dashboards.

## Handoff And Downstream Use

Reputation and Anti-Sybil Service hands recommendations to Oververify, Overguard, Oversched, Challenge Task Service, Fraud Control Service, Provider Payout Service, Overbill, Seal Ledger, Overclaim, admin UI, CLI, SDK, and central AI stewardship.

## Open Design Questions

Resolved decisions:

- Anti-Sybil signal legality is an operating-region policy fact, not hardcoded service logic. The Phase 11 portable baseline may use first-party operational evidence: onboarding identity-tier refs, public-provider policy acknowledgements, Overcell/node enrollment refs, software-version refs, Hardware Discovery and Benchmark Runner refs, Challenge Task Service outcomes, public sandbox compatibility, workload success/failure windows, Overclaim dispute/correction outcomes, Overwatch abuse/audit refs, and provider-owned payout eligibility or hold refs. Higher-risk correlation signals - device or hardware uniqueness, payout-account uniqueness, network/IP/ASN/geography correlation, behavioral timing clusters, shared artifact/result patterns, and cross-provider graph links - are usable only when Compliance Boundary and Overguard policy refs mark the signal allowed or review-required for the provider's operating region and purpose. Those signals must be stored as hashes, redacted summaries, or owner-service refs with purpose, expiry, visibility class, and appeal state. Where a region disallows a signal, the service records `signal_unavailable_by_region` and keeps the provider in the stricter public-limited class rather than substituting hidden collection. Raw identity documents, raw bank/card/tax data, exact fingerprints, exact IP histories, biometrics, private payloads, and other-provider identities stay in owning services or are not collected.
- Restricted eligibility versus suspension uses policy-versioned confidence bands plus severity, not a single opaque reputation number. The default Phase 11 bands are: below `0.50`, collect evidence, require recheck, or keep the new-provider default; `0.50` through `0.75`, publish restricted eligibility, throttles, shorter challenge cadence, duplicate-execution, or reduced earning velocity; `0.75` through `0.90`, publish probation or severe restriction and recommend holds or stewardship review when payable work is in scope; `0.90` and above, allow suspension only when supported by layered evidence or a critical direct signal. Missing, stale, inconclusive, or single non-critical signals should restrict or require review rather than suspend. Critical direct signals include sandbox escape, unauthorized egress, attempted secret or private-data access, fabricated identity/resource/challenge evidence, impossible benchmark claims, high-confidence duplicate-execution fraud, or confirmed payout-fraud refs. Oververify, Overguard, Oversched, Provider Payout Service, Overbill, Seal Ledger, and Overclaim still own final downstream effects.
- Automatic payout-hold recommendations are limited to high-confidence, policy-allowed, pre-finality triggers with evidence refs and release conditions: active Overclaim payout or settlement disputes, required Phase 11 public-provider dispute windows, fabricated evidence, impossible benchmark claims, high-confidence challenge or duplicate-execution fraud, repeated no-shows or result inconsistency inside a payable window, sandbox escape, unauthorized egress, attempted secret/private-data access, confirmed duplicate payout-account clusters where legally usable, payout destination or compliance blockers, and severe Fraud Control Service cases accepted by Overguard policy. Stewardship review is required for low-confidence or single-source clusters, region-restricted signals, central-AI-only or heuristic-only suspicion, first low-severity challenge failures, broad cluster-wide holds, post-finality reversals, regulated or secret-bearing evidence, sensitive identity/payout material, cross-region correlation, or any hold that could materially affect many honest providers if false positive. This service only emits hold trigger recommendations; Provider Payout Service, Overbill, Seal Ledger, and Overclaim create, release, correct, or finalize actual holds.
- Provider-facing cluster explanations may show the affected provider/node/payout period or public workload class, high-level cluster category, confidence band, severity, time window, stable reason codes, policy and evaluator versions, redacted evidence refs, current restriction or hold recommendation state, remediation steps, recheck options, and Overclaim appeal refs. Safe categories include `node_uniqueness_overlap`, `payout_uniqueness_overlap`, `challenge_pattern_anomaly`, `result_consistency_cluster`, `network_behavior_risk`, `abuse_report_cluster`, and `public_sandbox_violation`. Provider-facing output must not reveal other-provider identities, exact cluster membership, exact IP/device/fingerprint/payout hashes, raw graph edges, model weights, thresholds beyond coarse bands, random challenge internals, fraud heuristics, private tenant evidence, raw payout or identity material, operator notes, or incident-response details. Operator and stewardship views can dereference deeper evidence through Overwatch redaction profiles and signed access decisions.
- Corrected and expired signals remain append-only audit history but stop contributing to active recommendations according to policy. Accepted Overclaim corrections immediately mark prior signals `corrected`, append replacement refs, recompute eligibility, remove active provider-facing penalty where policy allows, and keep only a redacted correction trail in provider views. Expired signals become `expired` and no longer contribute unless policy retains them as historical risk context. Default retention is classed: non-payout low-severity operational signals retain redacted history for 180 days; eligibility, throttle, challenge-cadence, duplicate-execution, and public-provider correction signals retain redacted history for two years; payout-hold, confirmed fraud, dispute, compliance, suspension, and finality-related refs retain audit history for seven years or the stricter region/accounting requirement. Raw correlation inputs should be minimized sooner when owner-service retention permits, while hashes, reason codes, policy versions, correction refs, replay bundle refs, and Overwatch audit refs remain long enough to explain decisions, support appeals, and protect accounting finality.
