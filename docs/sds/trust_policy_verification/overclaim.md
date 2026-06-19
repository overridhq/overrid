SDS #32

# Overclaim SDS

## Purpose

Handle disputes, evidence, holds, challenge windows, refund proposals, correction proposals, appeals, and settlement-finality boundaries across execution, billing, provider payout, namespace, native app, and public-provider workflows.

Overclaim is the dispute and correction coordination layer. It does not rewrite history or directly mutate ledger balances. It creates append-only claim records, evidence links, hold requests, and resolution refs that downstream accounting and trust services can execute through their own contracts.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overclaim.md](../../service_catalog/trust_policy_verification/overclaim.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md), [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) |

## Service Family

- Family: Trust, policy, verification, and disputes
- Owning layer: Dispute intake, evidence coordination, challenge windows, holds, corrections, refunds, appeals, and finality records
- Primary data scope: claim records, parties, evidence links, challenge windows, hold requests, hold status refs, refund/correction proposals, resolution records, appeal records, finality markers, and audit exports
- First build phase from service plan: Records in [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md); settlement integration in [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Problem Statement

Overrid needs a clear dispute path before it can safely handle paid workloads, provider payouts, public supply, native app usage, or namespace conflicts. Execution can fail, providers can cheat, users can submit invalid claims, usage rollups can be disputed, and verification evidence can change after settlement is prepared.

The system must preserve finality where it matters while still allowing holds, corrections, refunds, and appeals. That requires a dispute service that links evidence across Overwatch, Overmeter, Oververify, Challenge Task Service, Seal Ledger, Overbill, and provider payout flows without editing their historical records.

## Goals

- Provide a single claim record format for execution disputes, billing disputes, provider payout disputes, namespace disputes, native app service disputes, and public-provider challenges.
- Link every claim to evidence refs, affected records, parties, policy versions, time windows, and requested remedies.
- Create hold requests before settlement or payout finality when claim policy allows it.
- Create refund and correction proposals that append new ledger and billing entries rather than changing old ones.
- Support challenge windows and appeal windows with explicit deadlines.
- Keep resolution decisions evidence-backed, reason-coded, replayable, and exportable.
- Preserve private data boundaries through evidence redaction and authorized views.
- Provide stable handoff contracts to Seal Ledger, Overbill, Provider Payout Service, Oververify, Universal Namespace Service, and native apps.

## Non-Goals

- Do not execute refunds, corrections, or payouts directly. Accounting services own financial state changes.
- Do not mutate Overmeter usage rollups, Seal Ledger entries, Overbill receipts, Oververify records, or namespace records in place.
- Do not act as a general customer-support inbox outside Overrid evidence and policy scope.
- Do not decide trust scores. Oververify and reputation services consume claim outcomes.
- Do not reveal private workload payloads, tenant secrets, provider-private capacity details, or fraud heuristics to unauthorized parties.
- Do not allow open-ended disputes with no deadlines, parties, evidence refs, or requested remedy.
- Do not add speculative pricing, business-volume projections, blockchain, NFT, or per-transaction fee behavior.

## Primary Actors And Clients

- Users, tenants, providers, native services, and system services opening claims through Overgate, SDK, CLI, admin UI, or service adapters.
- Overwatch, supplying trace, audit, incident, and evidence bundle refs.
- Overmeter, supplying usage rollups, raw event refs, and dispute windows.
- Oververify and Challenge Task Service, supplying verification outcomes and challenge evidence.
- Overbill, Seal Ledger, ORU Account Service, and Provider Payout Service, consuming hold, refund, release, and correction refs.
- Universal Namespace Service and Overpass, supplying identity, ownership, route, and namespace dispute refs.
- Overguard, checking claim permissions, data sensitivity, and policy boundaries.
- Central AI stewardship, fraud controls, and incident response, reviewing severe or systemic disputes.

## Dependencies

- [Overwatch](../control_plane/overwatch.md) for evidence bundles, trace history, audit records, incident refs, and export integrity.
- [Overmeter](../execution_scheduling/overmeter.md) for usage rollup refs, raw usage refs, dispute windows, and correction candidate refs.
- [Oververify](oververify.md) and [Challenge Task Service](challenge_task_service.md) for trust, eligibility, and challenge outcome evidence.
- [Overguard](overguard.md) for claim admission, party permissions, evidence visibility, workload sensitivity, and compliance policy.
- [Overbill](../accounting/overbill.md), [Seal Ledger](../accounting/seal_ledger.md), [ORU Account Service](../accounting/oru_account_service.md), and [Provider Payout Service](../accounting/provider_payout_service.md) for holds, refunds, corrections, releases, and payout state.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), and [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) for identity, tenant, route, and namespace ownership facts.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), and [Overvault](../data_storage_namespace/overvault.md) for claim records, evidence attachments, and redacted secret refs where implementation needs persistent storage.

Phase 4 can store claim records and evidence links before full accounting integration. Phase 5 must convert hold/refund/correction proposals into Seal Ledger and Overbill actions through explicit refs.

## Owned Responsibilities

Overclaim owns:

- Claim intake, idempotency, party validation, and claim classification.
- Evidence link records, evidence visibility rules, redaction metadata, and evidence completeness status.
- Challenge-window and appeal-window timers.
- Hold request records and settlement-finality boundary checks.
- Resolution proposals, final resolutions, appeals, withdrawals, and rejection records.
- Refund, correction, release, and payout-hold proposal refs.
- Claim audit exports and replay bundles.
- Cross-service handoff records that let downstream services act without reading Overclaim internals.

Overclaim must not directly alter external service state. It sends signed requests or proposals and records returned refs.

## Data Model

The first implementation should define:

- `claim_record`: claim id, claim type, affected workload/app/namespace/provider/account refs, tenant id, opener identity, respondent identity, requested remedy, severity, policy version, state, deadlines, and trace id.
- `claim_party`: party type, identity refs, tenant refs, role in claim, notification channel refs, response deadline, and visibility class.
- `evidence_link`: evidence id, source service, source record ref, evidence kind, integrity hash, retention class, redaction profile, visibility scope, and access decision refs.
- `party_statement`: claim id, party id, statement type, content ref, attachment refs, signature, submitted-at, and redaction refs.
- `challenge_window`: claim id, window type, start/end timestamps, allowed actions, hold behavior, escalation rule, and expiration outcome.
- `hold_request`: claim id, target account/ledger/payout/settlement refs, hold reason, amount/resource dimensions if known, partial-hold rule, expiry, and downstream response refs.
- `correction_proposal`: claim id, affected rollup or ledger refs, corrected dimensions, reason code, approving policy version, downstream service target, and resulting entry refs.
- `refund_proposal`: claim id, affected bill/receipt/account refs, refund scope, reason code, downstream Overbill refs, and payer/payee visibility refs.
- `resolution_record`: claim id, decision, remedy, evidence refs, reason codes, resolver identity or service, signed decision, downstream action refs, and finality marker.
- `appeal_record`: source resolution ref, appeal reason, new evidence refs, appeal window, decision refs, and final state.
- `audit_export`: claim id, export scope, redaction profile, included refs, integrity hash, generated-by, and generated-at.
- `claim_replay_bundle`: claim record, policy version, evidence refs, party statements, hold refs, downstream responses, and resolution refs.

Common envelope fields:

- `id`, `tenant_id`, `actor_id`, `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

Overclaim APIs must be stable for CLI, SDK, admin UI, native apps, and service integrations:

- `POST /claims`: open a claim with affected refs, claim type, requested remedy, and initial evidence refs.
- `GET /claims/{claim_id}`: read authorized claim state, parties, deadlines, evidence summary, and downstream action refs.
- `POST /claims/{claim_id}/evidence`: attach evidence refs or statements.
- `POST /claims/{claim_id}/holds`: request a settlement, payout, ledger, or billing hold.
- `POST /claims/{claim_id}/challenge-window`: open or extend an allowed challenge window.
- `POST /claims/{claim_id}/resolution-proposals`: propose refund, correction, release, denial, trust action, or manual review.
- `POST /claims/{claim_id}/resolve`: create final resolution and downstream action refs.
- `POST /claims/{claim_id}/appeals`: open an appeal from a resolution record.
- `POST /claims/{claim_id}/withdraw`: withdraw a claim when policy allows it.
- `GET /claims/{claim_id}/explain`: return evidence-backed reason codes and redacted resolution explanation.
- `POST /claims/{claim_id}/export`: create an audit export with a declared redaction profile.

API requirements:

- Mutating endpoints require actor identity, tenant context, signature or service credential, trace id, and idempotency key.
- Claim reads must enforce party visibility and redaction rules.
- Hold and correction endpoints must be idempotent and store downstream response refs.
- Resolution endpoints must require evidence refs and policy-version refs.

## Event Surface

- `overclaim.claim_opened`: claim accepted with affected refs and deadlines.
- `overclaim.claim_rejected`: claim denied at intake with reason codes.
- `overclaim.evidence_attached`: evidence link or party statement added.
- `overclaim.hold_requested`: hold request sent to accounting, payout, or settlement target.
- `overclaim.hold_acknowledged`: downstream service confirmed hold or returned denial refs.
- `overclaim.challenge_window_opened`: challenge or response window started.
- `overclaim.resolution_proposed`: remedy proposal created.
- `overclaim.resolved`: final resolution recorded.
- `overclaim.appealed`: appeal opened from a resolution.
- `overclaim.finalized`: finality marker created after all allowed windows close.
- `overclaim.export_created`: redacted audit export generated.

Events must include claim id, claim type, affected refs, policy version, evidence refs, and trace id. User-facing event channels must not include private payloads or secret material.

## Core Workflow

1. A user, provider, native app, operator, or system service opens a claim with affected refs and requested remedy.
2. Overclaim validates identity, tenant, claim type, deadlines, and visibility through Overguard.
3. Overclaim records claim parties, evidence refs, and current affected-state snapshots.
4. If settlement or payout is still challengeable, Overclaim sends hold requests to the relevant accounting or payout service.
5. Parties attach statements and evidence within the challenge window.
6. Overclaim may request challenge tasks, trust review, usage rollup review, namespace review, or incident review.
7. A resolution proposal is created with reason codes and downstream action refs.
8. Overclaim records final resolution, appeal windows, and finality marker.
9. Accounting, trust, namespace, and native app services execute their own changes from the resolution refs.
10. Overclaim emits audit export refs for authorized review.

## State Machine

Claim lifecycle:

1. `draft`: claim data is being assembled before submission.
2. `submitted`: intake received and idempotency key accepted.
3. `triaging`: claim type, party eligibility, affected refs, and deadlines are being checked.
4. `rejected`: claim is invalid, late, unauthorized, or unsupported.
5. `evidence_open`: parties may attach evidence and statements.
6. `hold_requested`: downstream hold has been requested.
7. `held`: downstream hold was acknowledged.
8. `under_review`: resolver, automated policy, challenge, or incident review is active.
9. `challenge_window`: response, challenge, or appeal window is running.
10. `resolution_proposed`: remedy proposal is available.
11. `resolved`: signed resolution exists.
12. `appealed`: appeal is active.
13. `final`: claim is closed and finality marker is recorded.
14. `withdrawn`: opener withdrew the claim before finality.
15. `expired`: required response or evidence window expired.

Accounting effect lifecycle:

1. `none`: no accounting action requested.
2. `hold_pending`: hold request sent.
3. `hold_active`: downstream hold acknowledged.
4. `release_pending`: release requested.
5. `correction_pending`: correction entry requested.
6. `refund_pending`: refund requested.
7. `effect_recorded`: downstream action returned immutable refs.
8. `effect_denied`: downstream action was denied with reason refs.

## Policy And Security

- Claim admission must check actor role, tenant membership, affected record visibility, claim window, and requested remedy.
- Evidence visibility is separate from claim visibility; a party may see a reason code without seeing raw private data.
- Secret-bearing evidence must be stored or referenced through Overvault-compatible refs and redaction profiles.
- Operator resolutions require signed action, reason code, policy version, and evidence refs.
- Automated resolutions must be replayable from stored policy versions and evidence refs.
- Holds must respect finality boundaries; if final settlement is no longer challengeable, Overclaim records why it cannot hold.
- Public-provider disputes must preserve fraud-control visibility limits while giving providers an appealable summary.
- Repeated abusive claims may trigger rate limits, but denials must still be audit logged.

## Metering And Accounting

Overclaim does not calculate prices, charge users, or mutate balances. It coordinates accounting effects through refs:

- Hold requests to Seal Ledger, Overbill, or Provider Payout Service.
- Refund proposals that Overbill turns into explicit refund records.
- Correction proposals that Seal Ledger turns into append-only correction entries.
- Release requests when claims are resolved or expired.
- Claim handling effort, evidence storage, audit export generation, and challenge-linked usage for system cost visibility.

All accounting effects must remain append-only. Corrections and refunds must create new records instead of editing original usage, invoice, payout, or ledger records.

## Observability And Operations

- Dashboards should show open claims by type, severity, affected service, hold status, deadline, resolution age, appeal rate, and downstream action status.
- Operators need evidence timelines that join Overwatch traces, usage rollups, challenge outcomes, trust evidence, billing refs, and payout refs.
- Alerts should fire on approaching finality deadlines, stuck holds, repeated provider disputes, high refund/correction rates, and claim clusters tied to incidents.
- Audit exports must be reproducible from claim refs and redaction profiles.
- Retention policy must preserve final dispute evidence long enough for accounting, compliance, stewardship, and public-provider review.

## Failure Modes And Recovery

- Missing affected refs: reject intake with actionable reason codes.
- Downstream hold service unavailable: keep claim in `hold_requested` with retry and deadline alerts.
- Evidence source unavailable: attach source-unavailable evidence status rather than dropping the claim.
- Conflicting evidence: preserve both refs and require resolver decision or challenge task.
- Claim window expires: resolve according to policy and record expiration evidence.
- Accounting action denied: record denial refs and allow resolver to propose an alternate remedy.
- Resolver error: mark review blocked, preserve all state, and allow replay.
- Duplicate claims: link duplicate refs and keep one canonical claim when policy allows.

## Validation Plan

The service implementation plan lists these requirements:

- Disputed jobs can hold settlement.
- Corrections append new ledger entries instead of editing history.
- Final resolution cites evidence.

Additional SDS-level validation:

- Contract tests for claim intake, evidence attachment, hold request, challenge window, resolution, appeal, withdraw, explain, and export APIs.
- Policy tests for party visibility, claim windows, affected-record access, and finality boundaries.
- Integration tests with Overmeter, Seal Ledger, Overbill, and Provider Payout Service stubs proving hold/refund/correction refs are idempotent.
- Redaction tests proving private workload data and secrets do not appear in unauthorized claim views or exports.
- Replay tests proving a resolution can be reconstructed from claim, policy, evidence, and downstream refs.
- Abuse tests for duplicate and repeated bad-faith claims.
- Public-provider dispute tests covering challenge failure, payout hold, appeal, and final resolution.

## Build Breakdown

1. Define claim, party, evidence, hold, challenge-window, proposal, resolution, appeal, finality, and export schemas.
2. Implement claim intake and authorized read APIs.
3. Add evidence linking, statements, redaction profiles, and Overwatch refs.
4. Add hold request refs and settlement-finality checks.
5. Add refund, correction, release, and payout-hold proposal refs for Phase 5 accounting.
6. Add resolution, appeal, and finality workflows.
7. Integrate Challenge Task Service and Oververify evidence.
8. Add namespace and native-app claim types after core execution and accounting claims are stable.
9. Add dashboards, export tooling, and claim-cluster analysis.

## Handoff And Downstream Use

Overclaim hands hold, release, refund, correction, payout, trust, and namespace action refs to the owning services. Overwatch, central AI stewardship, fraud controls, incident response, SDK, CLI, admin UI, and native apps use Overclaim as the durable source for dispute status and resolution evidence.

## Open Design Questions

Resolved decisions:

- Phase 4 allows record-only claims whose affected refs already exist in the private execution and trust stack: private workload result disputes, signed usage-rollup disputes, challenge-result disputes, provider/node eligibility disputes, policy-admission disputes, and provisional settlement or payout-hold requests. These claims may create claim records, evidence links, challenge windows, hold requests, correction proposals, and resolution refs, but they must not execute refunds, ledger corrections, payout releases, native-app billing adjustments, or namespace mutations until the owning Phase 5 or later service accepts the downstream refs. Billing, native-app, and namespace claim types may be catalogued early only when the owner service can supply stable affected refs; otherwise intake returns an audit-logged unsupported or not-yet-integrated reason code.
- Automatic holds are allowed only before finality: open Overmeter dispute windows, ORU reservations or held funds, Seal Ledger settlement candidates without a finality marker, Provider Payout items in `earning_observed`, `in_dispute_window`, `hold_pending`, `held`, or pre-submission `eligible` states, and deterministic payout batches that have not been submitted to external rails. Manual stewardship review is required after a final settlement marker, after Overbill or payout instructions have been submitted or paid, for public-provider fraud or Sybil clusters, for regulated or secret-bearing evidence, for cross-tenant/systemic incidents, or when the requested remedy would reverse a prior final decision. In those cases Overclaim records correction, reversal, refund, or hold proposals and lets Seal Ledger, Overbill, Provider Payout Service, Oververify, or the namespace owner execute append-only effects.
- Default windows are policy-versioned by claim class, not hard-coded in clients. Phase 4 private workload claims start with a 48-hour evidence and challenge window after the signed rollup or result receipt, plus a 7-day appeal window after resolution. Public-provider Phase 11 claims use stricter payout protection: a 7-day challenge or payout-hold window for payable work and a 14-day appeal window, with faster temporary holds for high-confidence fraud, sandbox escape, secret-access, or fabricated-evidence signals. Native app service disputes default to a 14-day receipt or usage-summary challenge window and a 14-day appeal window because end-user billing and service-delivery evidence can arrive later. Stewardship may extend windows by signed action when incidents, outages, compliance holds, or missing owner-service evidence make the default unsafe.
- Long-term retention applies to claim records, party records, finality markers, reason codes, policy versions, signatures, integrity hashes, Overwatch audit refs, downstream hold/release/refund/correction refs, appeal records, public-provider fraud or anti-Sybil signal refs, and replay bundles needed for accounting, compliance, stewardship, and provider payout review. Raw tenant payloads, secrets, private workload data, sensitive payout/tax/identity material, exact fraud heuristics, provider-private correlation data, and unrelated third-party evidence must stay in the owning service or Overvault-compatible refs; Overclaim stores redacted summaries, visibility classes, hashes, and access-decision refs. Raw usage detail may be retained for the normal Overmeter dispute period and pinned only while an active claim, hold, audit, appeal, or compliance review requires it.
- Repeated abusive claims create evidence-backed abuse markers, duplicate-link records, rate limits, cooldowns, higher evidence requirements for non-urgent filings, and reputation or eligibility signals for Oververify, Reputation and Anti-Sybil Service, Fraud Control Service, and Overguard. They must not create a blanket loss of dispute rights. Safety, payout, private-data, account, and finality-affecting claims still receive an authenticated intake path, reason-coded rejection or triage, and appealable explanations. Abuse controls should suppress duplicate or bad-faith volume while preserving one canonical claim per affected ref and a manual review path for severe or plausibly valid disputes.
