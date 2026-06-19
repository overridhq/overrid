SDS #53

# Fraud Control Service SDS

## Purpose

Detect and coordinate response to provider fraud, workload abuse, payout abuse, result manipulation, challenge evasion, and policy evasion in federation and public-capacity pools.

Fraud Control Service is an evidence and recommendation layer. It creates fraud signals, risk cases, hold/throttle/challenge recommendations, and review evidence. It does not adjudicate disputes, mutate payouts, change ledger history, assign final reputation, or permanently punish providers without a correction path.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [fraud_control_service.md](../../service_catalog/federation_public/fraud_control_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md) |

## Service Family

- Family: Federation and public capacity
- Owning layer: Fraud signal aggregation, evidence packaging, hold/throttle recommendations, and correctable risk cases
- Primary data scope: fraud signals, anomaly windows, risk cases, evidence packages, hold triggers, throttle recommendations, challenge-task requests, payout-risk facts, appeal/correction records, and eligibility-impact refs
- First build phase from service plan: [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md)

## Problem Statement

Public supply is adversarial by default. Unknown or semi-trusted providers may fake resources, manipulate benchmark results, disappear during work, return inconsistent outputs, collude across identities, abuse payout windows, or evade workload restrictions. At the same time, Overrid must avoid opaque irreversible punishment because false positives can harm honest providers and public-interest pools.

Fraud Control Service gives the ecosystem a bounded, explainable, correctable response path: collect signals, open risk cases, recommend holds or throttles, trigger challenges, package evidence, and hand disputes or corrections to the right owner.

## Goals

- Define fraud signal schemas for provider behavior, workload behavior, result consistency, payout risk, challenge results, benchmark anomalies, abuse reports, and policy evasion.
- Aggregate signals into risk cases with severity, confidence, subject refs, evidence refs, and recommended actions.
- Recommend payout holds, public-pool throttles, challenge tasks, eligibility reductions, or manual review without directly mutating payouts or reputation.
- Preserve evidence packages for central AI assistance, human review, Overclaim disputes, and governance reporting.
- Provide false-positive correction and appeal integration.
- Make every fraud action replayable from stored signals, rule versions, evidence refs, and policy decisions.
- Keep private, regulated, secret-bearing, and system-service workloads away from public providers through hard policy signals.

## Non-Goals

- Do not decide final disputes or appeals; Overclaim owns dispute process and finality markers.
- Do not execute payout holds, payout releases, reversals, or external payments; Provider Payout Service and Overbill own those flows.
- Do not maintain final reputation or anti-Sybil identity scores; Reputation and Anti-Sybil Service owns public-provider risk recommendations.
- Do not run challenge workloads; Challenge Task Service owns challenge manifests, assignments, and results.
- Do not schedule, run, or duplicate workloads; scheduling/execution services own those actions.
- Do not rewrite Seal Ledger history, ORU balances, billing docs, or provider earning truth.
- Do not add pricing, revenue forecasts, customer counts, blockchain mechanics, NFT mechanics, or per-transaction fee economics.

## Primary Actors And Clients

- Public Provider Onboarding and Reputation and Anti-Sybil Service submitting provider identity and behavior signals.
- Oververify submitting verification and challenge outcome facts.
- Challenge Task Service submitting challenge results and requesting fraud-driven challenge targets.
- Overrun, Overcell, Overmeter, Overwatch, and Oversched submitting workload, execution, usage, and scheduling anomalies.
- Provider Payout Service, Overbill, and Overclaim receiving hold, evidence, and dispute refs.
- Overguard consuming risk facts for policy decisions.
- Public-Interest Pool Service and Federation Template Service consuming eligibility and abuse-control facts.
- Central AI or human reviewers reading evidence packages where allowed.

## Dependencies

- [Oververify](../trust_policy_verification/oververify.md) for provider/node verification facts and challenge outcome refs.
- [Reputation and Anti-Sybil Service](../trust_policy_verification/reputation_anti_sybil_service.md) for anti-Sybil signals and eligibility recommendations.
- [Challenge Task Service](../trust_policy_verification/challenge_task_service.md) for challenge task creation and challenge results.
- [Overwatch](../control_plane/overwatch.md) for audit events, incident refs, trace evidence, and signal provenance.
- [Overmeter](../execution_scheduling/overmeter.md) for usage anomalies and signed usage rollups.
- [Overbill](../accounting/overbill.md), [Provider Payout Service](../accounting/provider_payout_service.md), and [Seal Ledger](../accounting/seal_ledger.md) for billing/earning/ledger refs without direct mutation.
- [Overclaim](../trust_policy_verification/overclaim.md) for disputes, appeals, holds, and correction/finality paths.
- [Overguard](../trust_policy_verification/overguard.md) for policy effects and deny-by-default handling.

## Owned Responsibilities

Fraud Control Service owns:

- Fraud signal schemas and ingestion rules.
- Signal normalization and deduplication by subject, workload, provider, tenant, pool, and time window.
- Risk case records with rule version, severity, confidence, evidence, status, and recommended action.
- Hold trigger and throttle recommendation contracts.
- Challenge-task request recommendations.
- Evidence package format for central AI, human review, Overclaim, and governance.
- False-positive correction, appeal handoff, and action retraction records.
- Rule bundles for fraud signal interpretation and severity mapping.

## Data Model

- `fraud_signal`: `signal_id`, `signal_type`, `subject_type`, `subject_ref`, `source_service`, `source_event_ref`, `time_window`, `severity`, `confidence`, `data_class`, `rule_version`, and `evidence_refs`.
- `risk_case`: `case_id`, `subject_refs`, `case_type`, `opened_from_signal_refs`, `severity`, `confidence`, `state`, `recommended_actions`, `policy_refs`, `claim_refs`, `correction_refs`, and `audit_refs`.
- `hold_trigger`: payout or grant hold recommendation with `target_ref`, `hold_scope`, `reason_codes`, `expiry`, `release_conditions`, `provider_payout_ref`, `overbill_ref`, and `overclaim_ref`.
- `throttle_recommendation`: public-pool or provider throttle proposal with `target_ref`, `resource_scope`, `limit_change`, `duration`, `reason_codes`, and `policy_ref`.
- `challenge_request_recommendation`: target and challenge rationale for Challenge Task Service.
- `evidence_package`: curated refs and summaries with `package_id`, `case_id`, `audience`, `redaction_profile`, `signal_refs`, `trace_refs`, `usage_refs`, `challenge_refs`, `billing_refs`, `claim_refs`, and `summary_hash`.
- `fraud_correction`: false-positive or appeal correction with `correction_id`, `case_id`, `submitted_by`, `evidence_refs`, `decision_ref`, `actions_to_retract`, and `audit_refs`.

Risk cases and signals are append-only. Corrections create new records that supersede action recommendations without deleting original evidence.

## API Surface

- `POST /fraud/signals`: ingests one or more fraud signals with source refs and idempotency keys.
- `POST /fraud/evaluate`: evaluates a provider, workload, payout, pool, or tenant scope and returns risk case and recommendation summaries.
- `GET /fraud/cases/{case_id}`: returns risk case state, reason codes, action recommendations, and redacted evidence.
- `POST /fraud/cases/{case_id}/recommend-hold`: creates or updates a payout/grant hold recommendation for downstream services.
- `POST /fraud/cases/{case_id}/recommend-throttle`: creates a public-pool or provider throttle recommendation.
- `POST /fraud/cases/{case_id}/request-challenge`: asks Challenge Task Service to run a challenge based on case evidence.
- `GET /fraud/cases/{case_id}/evidence-packages/{audience}`: returns a redacted evidence package for allowed reviewers.
- `POST /fraud/cases/{case_id}/corrections`: records false-positive correction or appeal evidence and downstream retraction needs.
- `GET /fraud/subjects/{subject_ref}/summary`: returns active cases and current recommendations subject to role and data-class filtering.

Mutating APIs require service identity, trace id, idempotency key, source event refs, and policy context. Stable errors include `signal_source_untrusted`, `signal_duplicate`, `evidence_missing`, `case_closed`, `hold_not_allowed`, `throttle_not_allowed`, `challenge_not_available`, and `correction_requires_claim`.

## Event Surface

- `fraud_control.signal_ingested`: signal accepted.
- `fraud_control.signal_rejected`: signal rejected with reason code.
- `fraud_control.case_opened`: risk case opened from signals.
- `fraud_control.case_updated`: case severity, confidence, or evidence changed.
- `fraud_control.hold_recommended`: payout or grant hold recommended.
- `fraud_control.throttle_recommended`: throttle recommended.
- `fraud_control.challenge_recommended`: challenge request recommended.
- `fraud_control.evidence_package_created`: redacted evidence package created.
- `fraud_control.correction_recorded`: false-positive or appeal correction recorded.
- `fraud_control.recommendation_retracted`: prior recommendation superseded or retracted.
- `fraud_control.case_closed`: case closed with final state and evidence refs.

Events include case id, subject refs, severity, confidence, reason codes, rule version, policy refs, redacted evidence refs, and downstream action refs.

## Core Workflow

1. Ingest signals from Overwatch, Overmeter, Oververify, Challenge Task Service, Provider Payout Service, Overbill, onboarding, and anti-Sybil systems.
2. Normalize signals and deduplicate by subject, time window, workload, provider, pool, and source.
3. Apply fraud rule bundle to open or update risk cases.
4. Request Overguard policy effects for recommended holds, throttles, challenges, or eligibility changes.
5. Emit recommendations to downstream services without directly mutating payout, eligibility, or reputation state.
6. Package evidence for central AI/human review, Overclaim, or governance reporting according to redaction profile.
7. Accept corrections and appeals, retract or revise recommendations, and preserve the full evidence trail.

## State Machine

Risk case lifecycle:

1. `opened`
2. `collecting_signals`
3. `recommendation_pending`
4. `recommended_action_active`
5. `awaiting_challenge`
6. `awaiting_review`
7. `under_claim`
8. `corrected`
9. `closed_no_action`
10. `closed_action_confirmed`
11. `superseded`

Recommendation lifecycle:

1. `proposed`
2. `policy_checked`
3. `sent`
4. `accepted_by_downstream`
5. `rejected_by_downstream`
6. `expired`
7. `retracted`

Correction lifecycle:

1. `submitted`
2. `evidence_review`
3. `accepted`
4. `denied`
5. `downstream_retraction_sent`
6. `closed`

## Policy And Security

- Treat public-provider signals as sensitive and potentially adversarial; require source trust and evidence refs.
- Do not expose raw provider private data, tenant data, payout data, or challenge internals beyond authorized audiences.
- Require Overguard decisions before recommending holds, throttles, eligibility changes, or challenge escalation.
- Keep fraud rules versioned and explainable; opaque permanent punishment is not allowed.
- Keep false-positive correction and appeal paths available for affected providers and tenants.
- Do not use fraud signals to place private, regulated, secret-bearing, or system-service workloads on public nodes.
- Preserve immutable evidence and correction history for disputes and governance.

## Metering And Accounting

- Emit fraud evaluation, evidence packaging, review, and challenge-recommendation usage dimensions to Overmeter where material.
- Link signals and risk cases to provider, tenant, workload, public pool, payout batch, grant, claim, and policy refs where applicable.
- Hold recommendations must reference Provider Payout Service, Overbill, Overclaim, or Overgrant records; this service does not move credits or funds.
- Seal Ledger entries remain append-only; fraud corrections use correction refs and downstream accounting records.
- Do not encode fines, revenue targets, pricing, or per-transaction fees.

## Observability And Operations

- Expose signal ingestion rate, rejected signal rate, case counts by state/severity, active hold/throttle recommendations, correction rates, false-positive rates, and stale review queues.
- Provide redacted reviewer views for risk cases, evidence packages, rule versions, and downstream action state.
- Track rule precision through confirmed fraud, corrected false positives, and appeal outcomes.
- Support simulation mode for fraud rules against recorded signals without issuing recommendations.
- Alert on sudden volume anomalies, high-severity cases, challenge failure bursts, payout-risk spikes, and rule-version regressions.

## Failure Modes And Recovery

- Untrusted signal source: reject signal and emit reason code.
- Duplicate signal: deduplicate and link to existing case.
- Evidence refs unavailable: keep case in `collecting_signals` or `awaiting_review`.
- Policy denies hold/throttle: record denial and do not send recommendation.
- Downstream hold rejected: keep case active and record downstream reason code.
- Challenge service unavailable: keep recommendation pending and avoid escalating severity solely because a challenge could not run.
- False-positive correction accepted: retract recommendations and send correction refs to downstream services.
- Rule bundle found defective: mark affected cases for reevaluation and preserve original decisions.

## Validation Plan

- Volume anomaly, challenge failure, result inconsistency, and payout-risk signals can open risk cases with stable reason codes.
- Fraud signals can recommend payout holds without deleting or rewriting history.
- Hold, throttle, and challenge recommendations are idempotent and policy-checked.
- False positives can be corrected through evidence and downstream retraction refs.
- Public pool eligibility changes are auditable through risk case, policy, and evidence refs.
- Private, regulated, secret-bearing, and system-service workloads remain excluded from public-provider decisions.
- Replay of a risk case reconstructs signal inputs, rule version, recommended action, policy refs, and corrections.

## Build Breakdown

1. Define fraud signal, risk case, hold trigger, throttle recommendation, challenge recommendation, evidence package, and correction schemas.
2. Implement signal ingestion and deduplication from Overwatch, Overmeter, Oververify, Challenge Task Service, payout, billing, and onboarding sources.
3. Add rule bundle evaluation and risk case lifecycle.
4. Add hold/throttle/challenge recommendation APIs with Overguard policy checks.
5. Add redacted evidence package generation for central AI, human review, Overclaim, and governance.
6. Add correction and appeal handoff with recommendation retraction.
7. Prove fraud signals affect payout holds and public-pool eligibility without mutating historical accounting records.

## Handoff And Downstream Use

Fraud Control Service hands reason-coded recommendations and evidence to Provider Payout Service, Overbill, Overclaim, Overguard, Public-Interest Pool Service, Public Provider Onboarding, Reputation and Anti-Sybil Service, Challenge Task Service, stewardship reporting, and operator tooling.

Downstream services should store fraud recommendation refs with their own state changes rather than copying fraud rule logic.

## Open Design Questions

Resolved decisions:

- Automatic hold recommendations in the first public low-sensitivity pool are limited to high-confidence, source-trusted, pre-finality evidence with Overguard policy approval, release conditions, and Overclaim/Provider Payout refs. The initial allowlist is active Overclaim payout or settlement disputes, the Phase 11 public-provider payout/dispute window, fabricated evidence, impossible benchmark or capacity claims, high-confidence challenge failure or duplicate-execution mismatch tied to payable work, repeated no-shows or result inconsistency inside a payout period, sandbox escape, unauthorized egress, attempted secret/private-data access, confirmed duplicate payout-destination clusters where legally usable, payout-destination or compliance blockers, and severe multi-source fraud cases accepted by policy. Fraud Control Service only emits hold recommendations; Provider Payout Service, Overbill, Seal Ledger, and Overclaim create, release, correct, or finalize actual holds.
- Manual review or challenge tasks are required for softer, broader, or more sensitive signals: single-source abuse reports, sudden volume changes, first low-severity challenge failures, inconclusive challenges, benchmark drift, resource-claim variance, central-AI-only or heuristic-only suspicion, suspected collusion or Sybil clusters, region-restricted identity/payout signals, cross-provider correlation, broad pool-wide throttles, public-report-sensitive allegations, post-finality reversals, and any action that could materially affect many honest providers if wrong. Challenge Task Service should receive liveness, capability, sandbox, duplicate-execution, and result-consistency questions. Stewardship or operator review is required when evidence is ambiguous, compliance-sensitive, broad in scope, or dependent on private fraud heuristics.
- Central AI review is bounded to evidence-package analysis, summary generation, risk assessment, missing-evidence prompts, and intervention recommendations. Every analysis must carry data-class refs, redaction profile, allowed analysis scope, policy thresholds, model/run provenance, evidence refs, confidence/proportionality summary, expiry, owning-service target, and appeal/correction path. Central AI cannot open or close fraud cases by itself, mutate holds, change provider eligibility, alter payouts or ledger entries, decide disputes, or serve as the only evidence source for sanctions. Severe sanctions, broad eligibility changes, public reports, compliance-sensitive cases, and privacy-sensitive evidence require signed steward/operator review and owning-service action.
- Public-provider payout scale requires a fast false-positive correction path before payout batches become large. Affected providers must receive a provider-safe reason-code summary and Overclaim appeal/correction ref when a hold or throttle is recommended. Correction intake must be acknowledged within 24 hours, triaged within 48 hours, and clear evidence mistakes must trigger recommendation retraction or release requests within 24 hours after the correcting evidence is accepted. Standard Phase 11 fraud/payout corrections must resolve before the affected payout batch reaches finality and no later than the 7-day public-provider challenge or hold window; appeals remain available for the 14-day Overclaim appeal window. Provider payout scaling should pause when correction queues, stale holds, or false-positive rates exceed policy thresholds.
- Public fraud case summaries may expose only redacted stewardship facts: case id or report ref, time window, subject class, pool or workload class, severity band, confidence band, stable public-safe reason-code categories, current state, recommended action class, challenge or hold existence, correction/appeal status, aggregate outcome counts, and redacted Overwatch or Stewardship Reporting refs. Public summaries must not expose raw signals, detection thresholds, challenge payloads/templates, cluster heuristics, provider-private identity/contact/payout details, tenant/workload private data, topology, payment refs, central-AI prompts/context, exploit details, or unreviewed severe allegations. Provider-specific and operator summaries can expose more detail only through audience-scoped redaction profiles and owning-service authorization.
