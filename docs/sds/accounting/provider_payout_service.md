SDS #43

# Provider Payout Service SDS

## Purpose

Batch provider earnings from contributed resources and legitimate services, apply dispute/fraud/verification holds, determine payout eligibility, create payout batch refs, track external payout status, and preserve correction evidence.

Provider Payout Service is a payout-coordination layer. It does not create usage truth, decide trust alone, rewrite Seal Ledger entries, or directly become a payment processor. It consumes provider earning refs and hold signals, then coordinates batched payout attempts through Overbill/payment-provider refs so Overrid can pay contributors safely without per-operation payment friction.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [provider_payout_service.md](../../service_catalog/accounting/provider_payout_service.md) |
| Sub-build plan | [SUB BUILD PLAN #43 - Provider Payout Service](../../build_plan/sub_build_plan_043_provider_payout_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md), [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md) |

## Service Family

- Family: Accounting, credits, billing, and rights
- Owning layer: provider earning views, payout eligibility, payout holds, payout batch refs, payout status, payout failures, reversals, and correction handoffs
- Primary data scope: provider earning snapshots, payout eligibility snapshots, hold records, payout batches, payout items, payment-provider refs, payout result events, reversal refs, correction refs, and compliance export refs
- First build phase from service plan: [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) for private providers; [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md) for public providers.

## Problem Statement

Overrid needs to compensate providers without letting fraud, disputes, challenge failures, chargebacks, public-provider Sybil behavior, or external payment failure corrupt internal accounting. Provider earnings originate from usage, resource contribution, app/service charges, subscription charges, one-time service charges, and ledger evidence, but payout eligibility depends on dispute windows, verification status, fraud signals, public-pool risk, and compliance constraints.

The ill-design to avoid is a direct "ledger earning equals pay now" rule. Earnings must pass through hold and eligibility checks, then move into bounded payout batches with idempotent payment refs and correction paths.

## Goals

- Aggregate provider earning refs from Seal Ledger into payout-period views.
- Support legitimate ORU earnings from approved resource contribution, native services, third-party apps, subscriptions, one-time charges, and machine-to-machine service usage.
- Apply dispute-window, Overclaim, Oververify, anti-Sybil, fraud-control, compliance, and policy holds.
- Produce payout eligibility snapshots with stable reason codes.
- Build payout batches that can be submitted through Overbill/payment-provider rails.
- Track submitted, paid, failed, reversed, chargeback-affected, and corrected payout states.
- Preserve evidence for every hold, release, failure, reversal, and correction.
- Enforce the hard rule: do not let users cash out bought ORU.
- Support stricter Phase 11 public-provider holds and throttles without weakening Phase 5 private-provider payouts.

## Non-Goals

- Do not create provider earning truth. Seal Ledger owns provider earning entries.
- Do not create usage truth. Overmeter and Seal Ledger own usage/accounting refs.
- Do not mutate balances directly. ORU Account Service projects balances from ledger entries.
- Do not adjudicate disputes. Overclaim owns dispute state and finality.
- Do not produce invoices or receipts. Overbill owns billing documents and external payment-provider refs.
- Do not cash out bought ORU for the buyer. Payout candidates must be provider-earned ORU backed by delivered service evidence, dispute finality, tax/compliance refs, and KYC/KYB/AML eligibility.
- Do not treat every ORU spend as payout-eligible. Spending is allowed inside the ORU economy; cash-out requires provider-earned ORU and the full eligibility path.
- Do not decide provider trust alone. Oververify, Reputation/Anti-Sybil Service, Fraud Control Service, and Overguard provide trust/risk facts.
- Do not pay every workload operation individually through an external provider.

## Primary Actors And Clients

- Private provider operators, reading payout status and hold reasons.
- Public providers, reading limited payout status and appeal paths under Phase 11.
- Overbill, receiving payout batch inputs and returning payment-provider refs/status.
- Seal Ledger, supplying provider earning, hold, release, correction, and reversal refs.
- Overclaim, supplying dispute windows, hold requests, settlement finality, and correction/refund proposals.
- Oververify, Reputation/Anti-Sybil Service, Fraud Control Service, and Challenge Task Service, supplying trust/fraud/challenge facts.
- Internal KYC Service, supplying KYC/KYB, beneficial-owner, payout destination ownership, source-of-funds, cooling-period, and cash-out eligibility facts.
- Wallet and Usage Center, showing provider payout status and held earnings where authorized.
- Admin UI, CLI, SDK, Overwatch, and central AI stewardship, inspecting payout evidence and reporting.

## Dependencies

- [Seal Ledger](seal_ledger.md) for provider earning entries, hold/release/correction entries, and settlement history.
- [Overbill](overbill.md) for payment-provider refs, payout document refs, refunds, chargebacks, and external rail events.
- [Overclaim](../trust_policy_verification/overclaim.md) for disputes, hold requests, finality refs, and correction proposals.
- [Oververify](../trust_policy_verification/oververify.md) for provider eligibility and evidence-backed trust signals.
- [Reputation and Anti-Sybil Service](../trust_policy_verification/reputation_anti_sybil_service.md) for public-provider risk recommendations, throttles, and payout hold triggers.
- [Fraud Control Service](../federation_public/fraud_control_service.md) and [Challenge Task Service](../trust_policy_verification/challenge_task_service.md) for fraud/challenge evidence.
- [ORU Account Service](oru_account_service.md) for provider account projections and available/held views.
- [Overwatch](../control_plane/overwatch.md) for audit evidence.
- [Overvault](../data_storage_namespace/overvault.md) for sensitive payout compliance refs and payment-token refs where needed.
- [Internal KYC Service](../governance_ops/internal_kyc_service.md) for KYC/KYB, payout destination ownership, source-of-funds, cooling-period, and cash-out eligibility facts.

## Owned Responsibilities

Provider Payout Service owns:

- Provider earning period views derived from Seal Ledger refs.
- Payout eligibility snapshots and reason codes.
- Hold records for dispute, fraud, challenge, anti-Sybil, compliance, KYC/KYB, AML, cooling-period, verification, chargeback, and operator/stewardship reasons.
- Hold release records linked to finality/evidence refs.
- Payout batch records, payout item records, and batch submission attempts.
- Idempotent payment instruction refs passed to Overbill/payment-provider adapters.
- Payout result state derived from Overbill/payment-provider events.
- Payout failure, retry, reversal, and correction coordination.
- Provider-facing payout-status views with redaction.
- Audit and compliance export bundles for payout periods.

It does not own raw payment credentials, direct payment execution, trust scoring, dispute adjudication, or historical ledger mutation.

## Data Model

The first implementation must define:

- `provider_earning_view`: provider id, tenant/scope, period, dimensions, earning ledger refs, deductions/holds refs, and derived payable candidate totals.
- `payout_eligibility_snapshot`: provider id, period, trust refs, dispute refs, fraud refs, compliance refs, policy decision refs, state, reason codes, and generated_at.
- `payout_hold`: hold id, provider id, earning refs, hold type, trigger refs, effective window, release requirements, appeal refs, and state.
- `hold_release`: release id, hold refs, finality refs, actor/service account, and released earning refs.
- `payout_batch`: batch id, period, included payout item ids, total by currency/rail/ref, submission state, Overbill refs, and audit refs.
- `payout_item`: provider id, earning refs, hold refs excluded, payable amount/ref, destination ref, status, and correction refs.
- `payment_instruction_ref`: idempotent instruction passed through Overbill, with no raw payment credential material.
- `payout_result`: external rail status ref, paid/failed/reversed/chargeback state, timestamps, provider-visible summary, and private evidence refs.
- `payout_correction`: correction, reversal, retry, refund, or chargeback-linked adjustment refs.
- `compliance_export_ref`: bounded export metadata and sensitive-data redaction refs.

Common envelope fields:

- `id`
- `tenant_id` or provider scope
- `provider_id`
- `period_id`
- `actor_id` or `service_account_id`
- `trace_id`
- `idempotency_key`
- `state`
- `schema_version`
- `policy_refs`
- `audit_refs`
- `created_at`
- `updated_at`

## API Surface

Phase 5 should expose:

- `POST /earning-views/build`: derive a provider earning view from Seal Ledger refs for a bounded period.
- `POST /payout-eligibility/evaluate`: create eligibility snapshots for provider/period candidates.
- `POST /holds`: create a payout hold from dispute, fraud, verification, compliance, challenge, or operator refs.
- `POST /holds/{id}/release`: release a hold after finality/evidence refs satisfy requirements.
- `POST /payout-batches`: create an idempotent batch from eligible payout items.
- `POST /payout-batches/{id}/submit`: submit payout instructions through Overbill/payment-provider refs.
- `POST /payout-batches/{id}/results`: ingest payout result refs from Overbill.
- `POST /payout-items/{id}/corrections`: attach reversal, retry, chargeback, or correction refs.
- `GET /providers/{id}/payout-status`: return provider-visible payout status with redaction.
- `GET /payout-batches/{id}/replay`: return source refs and eligibility decisions for audit.
- `GET /payout-periods/{id}/export`: create an authorized audit/compliance export ref.

API rules:

- Payout batch creation must be deterministic for a provider/period/idempotency key.
- A payout item cannot be submitted while any blocking hold is active.
- A payout item cannot be submitted without a current Internal KYC Service cash-out eligibility allow fact.
- Payment instructions must reference tokenized/approved payout destination refs only.
- Public-provider reads must reveal reason-code summaries and appeal refs without exposing fraud heuristics or other providers.
- Batch submission failures must be recorded before retry.

## Event Surface

- `provider_payout.earning_view_built`
- `provider_payout.eligibility_evaluated`
- `provider_payout.hold_created`
- `provider_payout.hold_released`
- `provider_payout.item_marked_eligible`
- `provider_payout.item_blocked`
- `provider_payout.batch_created`
- `provider_payout.batch_submitted`
- `provider_payout.payment_result_recorded`
- `provider_payout.payout_failed`
- `provider_payout.reversal_recorded`
- `provider_payout.correction_recorded`
- `provider_payout.export_created`

Events must include provider id, period id, payout item/batch id, reason codes, source refs, trace id, idempotency key, and audit refs. Sensitive payout destination data must stay behind tokenized refs.

## Core Workflow

1. Seal Ledger provider earning entries close for a payout period.
2. Provider Payout Service builds provider earning views from ledger refs.
3. Eligibility evaluation collects dispute, verification, fraud, challenge, compliance, and anti-Sybil facts.
4. Blocking holds are created or maintained where evidence requires.
5. Eligible payout items are grouped into deterministic payout batches.
6. Batches are submitted through Overbill/payment-provider refs.
7. External result events update payout batch/item state.
8. Failed or reversed payouts preserve evidence and create retry/correction records.
9. Provider-facing status and audit exports are generated from the same evidence.

## State Machine

Payout item states:

1. `earning_observed`: earning refs exist in Seal Ledger.
2. `in_dispute_window`: payout is waiting for dispute-window expiry or finality.
3. `hold_pending`: one or more hold triggers require evaluation.
4. `held`: blocking hold is active.
5. `eligible`: no blocking holds remain and policy checks passed.
6. `batched`: item is included in a payout batch.
7. `submitted`: payment instruction was submitted through Overbill.
8. `paid`: external result confirms payment.
9. `failed`: external result or validation failed.
10. `reversed`: paid item was reversed or clawed back.
11. `corrected`: correction refs supersede prior payout facts.

Payout batch states:

- `draft`
- `ready`
- `submitted`
- `partially_paid`
- `paid`
- `failed`
- `cancelled`
- `corrected`

No state transition may remove the old payout item, batch, or result. Corrections create new refs.

## Policy And Security

- Public-provider payouts are denied or held by default until Phase 11 eligibility, anti-Sybil, and sandbox facts are present.
- Private-provider payouts still require dispute-window, verification, KYC/KYB, AML, cooling-period, payout destination ownership, and policy checks.
- Hold triggers from Overclaim, Reputation/Anti-Sybil Service, Fraud Control Service, Challenge Task Service, Oververify, chargeback events, or operator action must be explainable.
- Payment destination refs must be tokenized or stored through approved vault/payment-provider paths.
- Raw bank, card, tax, or identity secrets must not be stored in payout records.
- Providers can see their own payout state and reason-code summaries, not other provider evidence or sensitive fraud heuristics.
- Signed operator overrides require Overwatch evidence and cannot bypass ledger correction requirements.

## Metering And Accounting

- Seal Ledger owns provider earning, hold, release, correction, and reversal entries.
- Provider Payout Service derives views and coordinates payout state; it does not mutate account balances directly.
- Overbill owns billing documents, payout document refs, and external payment-provider event refs.
- Payout periods should batch external interactions and avoid per-operation external payment calls.
- Payout corrections must reference the old item/batch/result and produce new accounting/billing refs rather than editing history.
- Provider payout views should reconcile `earned`, `held`, `eligible`, `submitted`, `paid`, `failed`, `reversed`, and `corrected`.

## Observability And Operations

Expose:

- Earning views built by provider/period.
- Held amount/count by reason code and source service.
- Eligibility denial rates and missing-fact rates.
- Batch size, submission latency, and success/failure rates.
- Payment-provider error categories.
- Retry and reversal counts.
- Public-provider hold rates and fraud/challenge-linked hold counts.
- Reconciliation gaps between Seal Ledger, Overbill, and payout state.
- Provider-facing status freshness.

Operators need drill-down by provider, period, batch, hold reason, source evidence, and external payment-provider ref.

## Failure Modes And Recovery

- Earning refs missing or inconsistent: block view creation with `earning_refs_incomplete`.
- Dispute window not closed: keep item `in_dispute_window`.
- Fraud or anti-Sybil signal present: create or maintain hold with source refs.
- Payment destination invalid: block item with `payout_destination_invalid`.
- Batch submit timeout: keep batch `submitted` or `failed` with retry-safe idempotency key.
- Partial batch success: mark item states individually; do not roll back paid items by deleting evidence.
- External reversal/chargeback: create reversal/correction refs and hold future eligible payouts if policy requires.
- Overbill/payment-provider unavailable: stop submission, preserve batch, retry after readiness returns.
- Provider appeal succeeds: release or correct holds with finality refs.

## Validation Plan

Required tests:

- Disputed earnings cannot be batched while a blocking dispute hold exists.
- Fraud/challenge/anti-Sybil hold blocks public-provider payout in Phase 11 scenarios.
- Eligibility snapshot records all source refs and reason codes.
- Payout batch creation is idempotent for the same period and candidates.
- Failed payout preserves evidence and can be retried without duplicate payment instructions.
- Partial payout success leaves per-item state accurate.
- Reversal or chargeback creates correction refs without editing historical paid state.
- Provider-facing status redacts sensitive fraud and payment details.
- Seal Ledger, Overbill, and Provider Payout views reconcile for a test payout period.
- Internal KYC cash-out eligibility facts block payout for missing KYC/KYB, active cool-off, source-of-funds review, related-party app spend, payout destination mismatch, and recent funding risk.

## Build Breakdown

1. Define provider earning view and payout period schemas.
2. Implement deterministic earning-view derivation from Seal Ledger refs.
3. Add dispute-window and verification hold rules for private providers.
4. Add payout eligibility snapshots and reason codes.
5. Add payout batch and payout item records.
6. Integrate Overbill payment instruction/result refs.
7. Add failure, retry, reversal, and correction flows.
8. Add provider-facing payout status and audit export.
9. Add Phase 11 public-provider hold inputs from anti-Sybil, fraud, and challenge services behind explicit feature gates.

## Handoff And Downstream Use

Provider Payout Service feeds:

- Overbill with payout batch inputs and payment instruction refs.
- Wallet and Usage Center with provider payout status.
- Overclaim with correction and appeal evidence.
- Reputation/Anti-Sybil and Fraud Control services with payout-hold outcomes.
- Central AI stewardship and reporting with aggregate payout/hold evidence.

Downstream services must treat payout state as evidence-backed coordination, not as replacement ledger truth.

## Open Design Questions

Resolved decisions:

- Mandatory payout destination and KYC/KYB checks are ref-based and deny-by-default. A provider cannot become payout-eligible until Provider Payout Service has a current provider account ref, Oververify/provider eligibility ref, Overtenant/Overpass ownership scope, Internal KYC Service cash-out eligibility allow fact, tokenized payout destination ref, destination ownership/consent ref, supported currency/rail/region ref, signed Overbill adapter capability ref, Compliance Boundary payout fact bundle, active Overguard allow decision, and a clean reconciliation checkpoint. Raw bank, card, tax, identity, or credential material stays in Overvault, Internal KYC Service evidence refs, or the approved payment-provider/tokenization path; this service only validates freshness, ownership, allowed use, and reason-coded blockers.
- Phase 5 private payouts should evaluate eligibility daily but submit external payouts by closed payout period, not by workload operation. The default payout period is seven days after Seal Ledger settlement and the applicable dispute window close, with Overbill rail minimums configured per adapter/policy rather than hardcoded in this SDS. Eligible items below the rail minimum roll into the next period unless an operator-approved correction, provider exit, or compliance-driven exception requires earlier settlement. Phase 11 public-provider payouts use the same batching contract but may extend the waiting period, reduce earning velocity, or require extra challenge/anti-Sybil finality before submission.
- Public-provider hold explanations expose only stable, remediable, coarse reason codes: `public_provider_verification_incomplete`, `challenge_review_pending`, `dispute_window_active`, `payout_destination_review_required`, `compliance_review_required`, `public_pool_throttle_active`, `payout_safety_review`, `appeal_or_correction_pending`, and `external_rail_unavailable`. Provider-facing views may include affected period, current state, safe remediation steps, policy/evaluator versions, redacted evidence refs, and Overclaim appeal refs. They must not expose exact thresholds, fraud heuristics, model weights, cluster membership, other-provider identities, exact payout/account hashes, raw graph edges, private tenant evidence, operator notes, or incident-response details.
- Cross-border tax and compliance state is represented as owner-service refs, not jurisdiction logic inside Provider Payout Service. The payout record stores jurisdiction profile refs, Compliance Boundary fact-bundle refs, tax profile or exemption refs, withholding/reporting marker refs, currency/rail refs, retention/redaction class, effective window, expiry, and audit/export refs. Raw tax forms, identity documents, payout credentials, bank/card data, and private compliance payloads remain in Overvault, Overtenant/Overpass, Compliance Boundary, or the approved payment-provider path. Jurisdiction changes create new fact-bundle and policy refs; existing payout records remain append-only and replayable.
- Automatic future-payout blocks are narrow, evidence-backed, and scoped. Confirmed duplicate payment, unresolved external reversal or chargeback, payout destination revocation, payout-provider idempotency conflict, Seal Ledger/Overbill/ORU reconciliation mismatch, tamper/checkpoint failure, Overclaim finality requiring clawback, confirmed fraud/challenge/sandbox/security violation, illegal or expired compliance fact bundle, and unresolved high-severity operator hold block future payouts for the affected provider, destination, rail, period, or payout items until release/correction refs arrive. Low-confidence fraud clusters, region-restricted signals, heuristic-only suspicion, or broad multi-provider impact require stewardship review before blocking beyond the specific risky items.
