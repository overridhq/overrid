SDS #41

# Overgrant SDS

## Purpose

Manage programmable resource allocation for sponsored work, grant-funded work, donations, purpose-scoped quotas, priority allocation, and public-interest resource pools.

Overgrant is the allocation policy and evidence layer for "who may consume which resources under which sponsor, purpose, quota, and time window." It does not mint money, issue tokens, or create speculative drops. It authorizes ORU-backed resource use and records grant evidence so Seal Ledger, Overguard, Oversched, native apps, and central AI stewardship can enforce and audit sponsored usage.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overgrant.md](../../service_catalog/accounting/overgrant.md) |
| SDS sub-build plan | [SUB BUILD PLAN #41 - Overgrant](../../build_plan/sub_build_plan_041_overgrant.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md), [Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md) |

## Service Family

- Family: Accounting, credits, billing, and rights
- Owning layer: grant allocation, sponsorship policy, purpose-scoped quota, allocation evidence, grant usage reporting, and grant revocation/correction
- Primary data scope: grant programs, grant source refs, eligible identities/tenants/apps/workload classes, purpose tag refs, resource dimensions, quota windows, allocation reservations, grant usage refs, fairness windows, abuse throttles, reporting snapshots, and revocation/correction refs
- First build phase from service plan: [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) primitives; federation and public-interest expansion in [Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md).

## Problem Statement

Overrid needs sponsored and purpose-scoped resource allocation without turning grants into blockchain tokens, NFT-style assets, or opaque administrator promises. Native service surplus, research sponsorships, educational pools, central AI stewardship choices, and partner federation programs all need enforceable quotas and evidence trails.

The ill-design to avoid is treating a grant as a balance counter that can be spent by any workload once a label is attached. Grants must be policy-bound allocation programs: each use needs eligibility evidence, resource dimensions, time windows, purpose tags, fairness checks, and Seal Ledger visibility.

## Goals

- Define grant programs with source account refs, eligible parties, purpose tags, resource dimensions, quota windows, fairness rules, and abuse controls.
- Evaluate grant eligibility before scheduling or ORU reservation.
- Produce grant authorization refs that Overguard, Oversched, ORU Account Service, and Seal Ledger can consume.
- Record grant-backed usage and reporting snapshots without duplicating Seal Ledger accounting truth.
- Support suspension, revocation, expiry, correction, and appeal evidence.
- Prepare for Phase 10 cross-tenant grants and public-interest pools without forcing those capabilities into Phase 5.
- Keep grants as resource allocation contracts, not speculative token drops or transferable market objects.

## Non-Goals

- Do not mint ORU, mutate ORU balances, or append Seal Ledger entries directly.
- Do not replace Overguard policy decisions; Overgrant supplies allocation facts and recommendations.
- Do not replace Oversched placement; it only supplies grant-backed placement inputs and budget constraints.
- Do not own purpose tag definitions. Purpose Tag Registry owns stewarded tags and evidence requirements.
- Do not execute provider payouts, billing, or refunds.
- Do not create speculative grant tokens, NFT coupons, yield mechanics, or tradeable grant markets.
- Do not make cross-tenant public-interest pool behavior part of the minimum Phase 5 implementation.

## Primary Actors And Clients

- Central AI stewardship, creating or recommending resource allocations for public-interest work.
- Native services, routing surplus or sponsored resource budgets into grant programs.
- Organizations, sponsors, nonprofits, universities, research groups, and trusted partners, defining eligible usage.
- Overguard, requesting allocation facts during policy decisions.
- Oversched, requesting grant-backed placement and quota signals.
- ORU Account Service and Seal Ledger, consuming grant source refs, reservation refs, usage refs, and correction refs.
- Wallet and Usage Center, showing grant-funded usage and remaining allocation where authorized.
- Public-Interest Pool Service and federation services, expanding grant programs in Phase 10.
- Overclaim, receiving disputes about grant eligibility, revocation, or misuse.

## Dependencies

- [ORU Account Service](oru_account_service.md) for account refs, budget prechecks, and grant-pool account projections.
- [Seal Ledger](seal_ledger.md) for grant allocation, reservation, settlement, correction, and usage history.
- [Overguard](../trust_policy_verification/overguard.md) for policy admission decisions and denial reason codes.
- [Oversched](../execution_scheduling/oversched.md) for placement decisions that include grant-backed quotas or priority.
- [Overmark](overmark.md) for bounded resource cards and budget previews.
- [Overclaim](../trust_policy_verification/overclaim.md) for disputes, correction proposals, and appeal finality.
- [Purpose Tag Registry](../federation_public/purpose_tag_registry.md) for verified purpose tags and evidence requirements.
- [Overwatch](../control_plane/overwatch.md) for audit events and reporting evidence.
- [Overtenant](../control_plane/overtenant.md), [Overpass](../control_plane/overpass.md), and [Overkey](../control_plane/overkey.md) for tenant, identity, and signed actor context.

## Owned Responsibilities

Overgrant owns:

- Grant program lifecycle records.
- Eligibility rule bundles and their versioned evaluation inputs.
- Grant source refs, grant pool refs, sponsor refs, and intended beneficiary refs.
- Purpose tag refs and evidence refs required by a grant.
- Resource dimension caps, per-window quotas, per-party limits, and fairness windows.
- Grant authorization refs used by Overguard, Oversched, and accounting prechecks.
- Grant usage observation refs derived from Seal Ledger and Overmeter evidence.
- Abuse throttle records, suspension records, revocation refs, and correction refs.
- Reporting snapshots for sponsors, central AI stewardship, public-interest pools, and authorized users.
- Replay bundles that explain why a workload was eligible or denied under a grant program version.

Overgrant does not own source funds, raw balances, usage truth, final policy admission, final placement, purpose tag definitions, or payout execution.

## Data Model

The first implementation must define these logical records:

- `grant_program`: stable program id, owner/sponsor refs, tenant scope, source account refs, purpose tag refs, state, policy refs, reporting requirements, and governance refs.
- `grant_source_ref`: ORU account, donation, sponsorship, native-service surplus, or stewardship allocation source. It is a reference, not a balance copy.
- `eligibility_rule_bundle`: versioned rule set describing eligible identities, tenants, apps, workload classes, purpose tags, data classes, jurisdictions, trust levels, and evidence requirements.
- `quota_window`: resource dimensions, per-window cap, per-party cap, concurrency cap, reset policy, start/end time, and rollover behavior.
- `fairness_window`: allocation share, starvation protection, priority weight, max burst, and reason codes for throttling.
- `grant_authorization`: immutable decision ref produced after eligibility, quota, policy, and source checks.
- `grant_reservation_ref`: reference to an ORU reservation or scheduler request that consumed a grant authorization.
- `grant_usage_ref`: Seal Ledger/Overmeter refs proving actual use, settlement, refund, correction, or expiry.
- `abuse_control_record`: throttle, suspension, challenge requirement, revocation, appeal, or correction state.
- `reporting_snapshot`: sponsor/stewardship/public-interest reporting period, consumed dimensions, remaining authorized capacity, blocked requests, and evidence refs.

Common envelope fields:

- `id`
- `tenant_id`
- `program_id`
- `actor_id` or `service_account_id`
- `beneficiary_ref`
- `trace_id`
- `idempotency_key`
- `state`
- `schema_version`
- `policy_refs`
- `audit_refs`
- `created_at`
- `updated_at`

## API Surface

Phase 5 should expose a compact command/query contract:

- `POST /grant-programs`: create a draft grant program with source refs, owner refs, eligibility draft, quotas, and reporting requirements.
- `POST /grant-programs/{id}/activate`: activate a program after source, policy, and identity checks pass.
- `POST /grant-programs/{id}/rules`: submit a new eligibility/quota rule-bundle version.
- `POST /grant-programs/{id}/evaluate`: evaluate a workload/app/user against grant eligibility without reserving resources.
- `POST /grant-programs/{id}/authorize`: create a grant authorization ref for an eligible workload request.
- `POST /grant-authorizations/{id}/consume`: bind an authorization to ORU reservation or scheduler refs.
- `POST /grant-programs/{id}/suspend`: suspend all or part of a program with signed operator/stewardship evidence.
- `POST /grant-programs/{id}/revoke`: revoke remaining allocation or a specific authorization.
- `POST /grant-programs/{id}/corrections`: attach correction refs after dispute, refund, or abuse review.
- `GET /grant-programs/{id}`: read authorized program metadata, current state, and rule versions.
- `GET /grant-programs/{id}/report`: read a bounded reporting snapshot with redacted evidence links.
- `GET /grant-authorizations/{id}/replay`: return the decision bundle used for eligibility and quota evaluation.

API rules:

- Mutating commands require signed actor/service identity, tenant context, idempotency key, trace id, and policy refs.
- Authorizations must be immutable after issuance; corrections create new refs.
- Reads must redact private beneficiary, workload, and sponsor data based on role and reporting scope.
- Batch authorization is allowed only with bounded size, one rule-bundle version, and deterministic per-item reason codes.

## Event Surface

- `overgrant.program_created`
- `overgrant.program_activated`
- `overgrant.rules_versioned`
- `overgrant.eligibility_evaluated`
- `overgrant.authorization_created`
- `overgrant.authorization_denied`
- `overgrant.authorization_consumed`
- `overgrant.quota_exhausted`
- `overgrant.throttle_applied`
- `overgrant.program_suspended`
- `overgrant.program_revoked`
- `overgrant.correction_recorded`
- `overgrant.report_snapshot_created`

Events must carry program id, tenant id, actor/service account id, rule version, relevant purpose tag refs, stable reason codes, trace id, idempotency key, and Overwatch audit refs. Events must not expose private workload content when a workload ref is enough.

## Core Workflow

1. Sponsor, central AI stewardship, native service, or authorized tenant creates a draft grant program.
2. Overgrant validates source refs through ORU Account Service and checks identity/tenant authority.
3. Rule bundle is attached with eligible parties, purpose tags, resource dimensions, quotas, fairness, and abuse controls.
4. Overguard evaluates activation policy; accepted programs become active.
5. Workload planning calls `evaluate` or `authorize` before scheduling.
6. Overgrant returns authorization or denial with reason codes and replay refs.
7. Oversched and ORU Account Service bind accepted authorizations to reservation attempts.
8. Seal Ledger entries and Overmeter rollups create usage refs.
9. Reporting snapshots reconcile authorizations, reservations, usage, expiry, revocations, and corrections.
10. Abuse, dispute, or sponsor changes create suspension, revocation, correction, or appeal records.

## State Machine

Grant program states:

1. `draft`: program is being assembled and cannot authorize use.
2. `source_pending`: source account, sponsor, or pool refs are awaiting validation.
3. `policy_review`: Overguard/stewardship policy checks are pending.
4. `active`: program can evaluate and authorize requests.
5. `paused`: new authorizations are blocked, existing consumed refs remain auditable.
6. `quota_exhausted`: no remaining quota for the relevant window or dimension.
7. `expired`: time window ended and no new authorizations are allowed.
8. `suspended`: abuse, dispute, compliance, or policy issue blocks use.
9. `revoked`: remaining allocation is cancelled by authorized action or policy.
10. `closed`: reporting and settlement reconciliation completed.
11. `corrected`: a later correction ref supersedes a prior reporting or authorization fact.

Grant authorization states:

- `requested`
- `eligible`
- `denied`
- `reserved`
- `consumed`
- `expired`
- `revoked`
- `corrected`

State transitions are append-only. A grant authorization cannot be silently edited to change beneficiary, quota, purpose, or resource dimension.

## Policy And Security

- Grant eligibility is deny-by-default when source, rule, purpose tag, workload class, tenant, identity, or policy facts are missing.
- Grant authorizations must include policy decision refs and rule-bundle version refs.
- Purpose-tagged grants must carry evidence refs required by Purpose Tag Registry.
- Cross-tenant grants require explicit Phase 10 federation rules; Phase 5 must not silently allow them.
- Public-provider use requires Phase 11 public low-sensitivity policy; grants cannot bypass public-node restrictions.
- Sponsor-facing reports must redact private workload data unless the sponsor is also authorized to see it.
- Operator overrides require signed action envelopes and Overwatch evidence.
- Grant source refs must never expose raw secrets, payment credentials, or private account internals.

## Metering And Accounting

- Overgrant does not maintain balance truth. Seal Ledger owns append-only accounting entries.
- ORU Account Service projects grant-pool, sponsor, reserve, escrow, and beneficiary balances from ledger entries.
- Grant authorizations must include resource dimensions such as CPU-ORU, GPU-ORU, STOR-ORU, NET-ORU, MEM-ORU, DATA-ORU, or Service-ORU.
- Grant usage reports reconcile `authorized`, `reserved`, `consumed`, `expired`, `revoked`, `refunded`, and `corrected` totals.
- Internal grant-backed usage must not trigger per-operation external payment calls.
- Near-cost native service surplus routing is a structural allocation input, not a revenue forecast.

## Observability And Operations

Expose:

- Active, paused, exhausted, expired, suspended, and revoked program counts.
- Authorization success/denial counts by reason code and rule version.
- Quota remaining and quota exhaustion by dimension/window.
- Abuse throttle and appeal counts.
- Source-ref validation failures.
- Reporting snapshot lag.
- Reconciliation gaps between authorizations, reservations, and Seal Ledger usage refs.
- Top blocked integrations by missing dependency or policy fact.

Operators need drill-down by program id, sponsor ref, purpose tag, tenant, workload class, and resource dimension.

## Failure Modes And Recovery

- Missing source account ref: deny activation or authorization with `grant_source_missing`.
- Source account insufficient or suspended: deny authorization with `grant_source_unavailable`.
- Purpose tag evidence missing: deny with `purpose_evidence_missing`.
- Quota race: use idempotent authorization and deterministic quota-window locking; later collisions return `quota_window_conflict`.
- Scheduler reservation fails after authorization: keep authorization unconsumed or expired with scheduler failure ref.
- Ledger reconciliation mismatch: create `grant_reconciliation_blocked` and require correction refs.
- Abuse signal arrives after authorization: suspend future authorizations and hold/correct affected usage through Overclaim and Seal Ledger refs.
- Sponsor revokes program: stop new authorizations; keep prior usage refs immutable.

## Validation Plan

Required tests:

- Eligible workload receives a grant authorization with stable policy, purpose, quota, and source refs.
- Ineligible workload is denied with deterministic reason codes.
- Authorization cannot be consumed twice with different reservation refs.
- Quota exhaustion blocks new authorizations without editing old authorizations.
- Grant usage appears through Seal Ledger refs and reporting snapshots.
- Suspension and revocation stop new use but preserve evidence for previous use.
- Cross-tenant grants are denied before Phase 10 federation rules exist.
- Public-provider grant use cannot bypass Phase 11 low-sensitivity policy.
- Reporting redaction prevents sponsor views from leaking private workload contents.

## Build Breakdown

1. Define `grant_program`, `eligibility_rule_bundle`, `quota_window`, and `grant_authorization` schemas.
2. Implement local/private Phase 5 grant programs with one tenant and explicit source refs.
3. Add eligibility evaluation and authorization APIs.
4. Add ORU Account Service and Seal Ledger refs for source, reservation, usage, correction, and reporting.
5. Add Overguard policy checks and stable denial reason codes.
6. Add Oversched/Overmark-facing placement and budget facts.
7. Add suspension, revocation, expiry, and correction flows.
8. Add reporting snapshots and reconciliation checks.
9. Leave Phase 10 cross-tenant and public-interest-pool expansion behind explicit federation feature gates.

## Handoff And Downstream Use

Overgrant feeds:

- Overguard with grant eligibility and allocation facts.
- Oversched with grant-backed placement and quota inputs.
- ORU Account Service and Seal Ledger with source, reservation, usage, refund, correction, and reporting refs.
- Public-Interest Pool Service with programmable allocation primitives in Phase 10.
- Central AI stewardship and native services with auditable surplus/donation routing.
- Wallet and Usage Center with authorized grant usage views.

Downstream services must consume Overgrant via authorization and report APIs, not by reading private grant storage.

## Open Design Questions

- Phase 5 tenant-local approval is enough for single-tenant private grants that spend from the tenant's own grant-pool or reserve account, stay inside approved workload/data classes, use explicit source refs, and do not claim stewarded public-interest purpose tags. Central AI stewardship approval, or a stewardship review action routed through the owning services, is required for native-service surplus routing, public-interest allocations, cross-tenant grants, federation grants, medical/regulatory/safety-sensitive programs, emergency/public-infrastructure programs, or any program whose rules affect public reporting, fraud interventions, or broad ecosystem allocation. Central AI may recommend grants, throttles, or allocation changes, but Overgrant, Public-Interest Pool Service, ORU Account Service, Seal Ledger, Overguard, and review bodies own the final mutable actions.
- Phase 5 may allow only low-stakes self-asserted purpose scopes such as `tenant_internal`, `sponsored_private`, `education_internal`, `research_internal`, `opensource_internal`, `trial_credit`, and `operator_test`, and those tags must remain local/private, explicitly non-public-interest, and ineligible for cross-tenant federation or public reporting claims. Stewarded public-interest tags such as science, education, medical, opensource, climate, public infrastructure, emergency response, and other donation/surplus categories must wait for Phase 10 Purpose Tag Registry maturity with active tag-version refs, evidence requirement bundles, review history, policy exports, and replayable validation refs.
- Beneficiaries should see only a redacted grant source class, sponsor display ref where policy allows, authorized dimensions, quota window, purpose scope, expiry, restriction summary, and reporting obligations. They must not see raw source-account ids, balances, reserve strategy, payment-provider refs, donor-private metadata, sponsor account history, native-service surplus internals, or fraud/compliance flags. Steward, auditor, and owning-service views may resolve stronger evidence refs under audience policy, but beneficiary and public views should rely on redacted source refs plus Seal Ledger/ORU proof summaries.
- Phase 5 fairness should use deterministic weighted fair share with per-party caps, per-window quotas, concurrency caps, burst limits, cooldowns, and starvation-prevention minimums. The first algorithm should be replayable from grant rule version, quota window, usage refs, and denial reason codes; it should not require opaque ML allocation or sponsor-controlled ranking. Sponsor preference may set eligible classes, weights, and explicit reserved shares, but Overgrant/Overguard must enforce hard caps, abuse throttles, and concentration alerts so one sponsor, tenant, grantee, app, or workload class cannot drain the pool without visible policy evidence.
- Phase 10 public-interest pool reports should publish only aggregate, public-safe fields: pool id/name, active purpose tag refs and versions, contribution/source classes, resource dimensions authorized and consumed, quota window status, grantee or project public refs where approved, allocation counts, denial/throttle reason-code totals, fairness-window summaries, usage/accounting checkpoint refs, outcome-report refs, renewal/revocation state, correction/retraction notices, and report template/version refs. Public reports must exclude private workload contents, raw evidence, raw account details, payment data, sponsor-private terms, fraud heuristics, private central-AI reasoning, and any field below aggregation/redaction thresholds.
