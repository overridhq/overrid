SDS #54

# Public-Interest Pool Service SDS

## Purpose

Manage donated, sponsored, or stewardship-directed capacity for approved public-interest work.

Public-Interest Pool Service defines pool accounts, contributed resource scopes, eligible grantees, purpose requirements, quotas, fairness rules, abuse controls, usage reports, and outcome-report hooks. It does not mint credits, maintain balances, adjudicate grants alone, schedule workloads, validate purpose tags, or decide central AI stewardship priorities by itself.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [public_interest_pool_service.md](../../service_catalog/federation_public/public_interest_pool_service.md) |
| SDS sub-build plan | [SUB BUILD PLAN #54 - Public-Interest Pool Service](../../build_plan/sub_build_plan_054_public_interest_pool_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md) |

## Service Family

- Family: Federation and public capacity
- Owning layer: Purpose-aware public-interest capacity allocation and reporting
- Primary data scope: pool definitions, contribution refs, purpose tag requirements, eligible grantee scopes, quota windows, fairness rules, grant allocation refs, abuse controls, usage reports, outcome-report refs, renewal, revocation, and stewardship routing refs
- First build phase from service plan: [Phase 10: Trusted Federation and Public-Interest Pools](../../build_plan/phase_10_trusted_federation_public_interest_pools.md)

## Problem Statement

Overrid’s native services and trusted partners can direct surplus, donated, or sponsored capacity toward science, education, medical work, open source, climate, public infrastructure, and other public-interest efforts. Without a pool service, this capacity would be informal and hard to audit: unclear eligibility, weak quotas, no fairness rules, no usage reconciliation, and no evidence for why a project received support.

The service must turn public-interest capacity into accountable resource allocation without speculative token drops or opaque charity accounting.

## Goals

- Define public-interest pools with source refs, purpose tags, contribution scopes, eligible grantees, quotas, fairness rules, and reporting requirements.
- Connect pools to Overgrant for programmable resource allocation, ORU Account Service for projections, Seal Ledger for immutable accounting evidence, and Overmeter for usage.
- Enforce purpose tag eligibility and evidence requirements through Purpose Tag Registry and Overguard.
- Track per-grantee quota use, pool exhaustion, renewal windows, revocation, and abuse throttles.
- Produce public-interest usage reports and outcome-report hooks without leaking private workload data.
- Support future native-service surplus routing through central AI stewardship rules as structural allocation, not private extraction.

## Non-Goals

- Do not define purpose tags or eligibility criteria globally; Purpose Tag Registry owns tag definitions and evidence requirements.
- Do not create grant authorizations directly; Overgrant owns grant programs and allocations.
- Do not maintain balances or ledger history; ORU Account Service and Seal Ledger own accounting views and entries.
- Do not schedule or execute workloads; scheduling/execution services own capacity use.
- Do not decide whether a scientific or public-interest project is valuable by itself; stewardship and governance services provide approval rules.
- Do not route private, regulated, secret-bearing, or system-service workloads into public pools unless the relevant policy explicitly allows them under trusted capacity.
- Do not add pricing, revenue forecasts, customer counts, blockchain mechanics, NFT mechanics, or per-transaction fees.

## Primary Actors And Clients

- Central AI stewardship mechanisms proposing public-interest allocation rules.
- Native services routing surplus to approved pools through stewardship policy.
- Trusted organizations contributing capacity through federation templates.
- Grantees, projects, researchers, nonprofits, open-source maintainers, and public-service teams requesting pool capacity.
- Overgrant creating allocation authorizations.
- Purpose Tag Registry validating purpose tags and evidence requirements.
- Overguard enforcing eligibility, quota, workload, data-class, and abuse policy.
- Overmeter, ORU Account Service, Seal Ledger, and Overbill reporting usage and accounting refs.
- Overwatch and stewardship reporting publishing evidence-safe summaries.

## Dependencies

- [Overgrant](../accounting/overgrant.md) for grant programs, grant authorizations, quota windows, revocation, and correction evidence.
- [ORU Account Service](../accounting/oru_account_service.md) and [Seal Ledger](../accounting/seal_ledger.md) for ledger-derived projections and immutable accounting refs.
- [Overmeter](../execution_scheduling/overmeter.md) for usage rollups.
- [Purpose Tag Registry](purpose_tag_registry.md) for verified purpose tags and evidence requirements.
- [Overguard](../trust_policy_verification/overguard.md) for eligibility, quota, abuse, workload-class, and data-class decisions.
- [Federation Template Service](federation_template_service.md) for trusted contributed capacity boundaries.
- [Fraud Control Service](fraud_control_service.md) and [Overclaim](../trust_policy_verification/overclaim.md) for abuse throttles, holds, disputes, corrections, and appeals.
- [Stewardship Reporting Service](../governance_ops/stewardship_reporting_service.md) for later public reporting and central AI investment transparency.

## Owned Responsibilities

Public-Interest Pool Service owns:

- Public-interest pool definitions and lifecycle.
- Contributed capacity refs and sponsor/stewardship source refs.
- Eligible grantee scopes and evidence refs.
- Pool-specific purpose tag requirements and workload/data-class restrictions.
- Quota and fairness rule records per pool and grantee.
- Pool allocation request records and Overgrant handoff refs.
- Abuse throttle refs and revocation/renewal records.
- Usage report and outcome-report refs.
- Public/redacted reporting summaries for pool activity.

## Data Model

- `public_interest_pool`: `pool_id`, `name`, `purpose_tag_refs`, `source_refs`, `capacity_scope_refs`, `eligible_grantee_rules`, `quota_rules`, `fairness_rules`, `reporting_requirements`, `state`, and `policy_refs`.
- `pool_contribution`: contributed or sponsored capacity with `contribution_id`, `source_type`, `federation_instance_ref`, `native_service_surplus_ref`, `grant_source_ref`, `resource_dimensions`, `availability_window`, `restrictions`, and `audit_refs`.
- `grantee_eligibility_record`: grantee identity/tenant refs, purpose evidence refs, approved workload classes, data classes, quotas, renewal window, and state.
- `pool_allocation_request`: request for pool support with `request_id`, `pool_id`, `grantee_ref`, `purpose_tag_refs`, `requested_dimensions`, `time_window`, `policy_refs`, `grant_refs`, and `state`.
- `fairness_window`: per-grantee and per-purpose allocation history with quota, usage, throttles, and renewal/refill behavior.
- `pool_usage_report`: usage summary with Overmeter rollup refs, ORU/Seal Ledger refs, grant refs, purpose refs, grantee refs, and redaction profile.
- `outcome_report_ref`: optional grantee-supplied or stewardship-supplied result refs with visibility controls.

Pool definitions are versioned. Allocation and usage records are append-only except for explicit correction records linked to Overgrant, Overclaim, or accounting refs.

## API Surface

- `POST /public-interest-pools`: creates a draft pool with purpose tags, source refs, contribution scopes, eligibility rules, quotas, fairness rules, and reporting requirements.
- `POST /public-interest-pools/{pool_id}/activate`: activates a pool after Overguard, purpose tag, and accounting-boundary checks pass.
- `GET /public-interest-pools/{pool_id}`: returns pool status, purpose scope, contribution summaries, eligibility rules, quota status, and redacted reports.
- `POST /public-interest-pools/{pool_id}/contributions`: attaches donated, sponsored, federation, or native-service surplus capacity refs.
- `POST /public-interest-pools/{pool_id}/eligibility/evaluate`: evaluates whether a grantee/workload/purpose request is eligible.
- `POST /public-interest-pools/{pool_id}/allocation-requests`: creates a pool allocation request and Overgrant handoff refs.
- `GET /public-interest-pools/{pool_id}/allocation-requests/{request_id}`: returns allocation state, quota/fairness status, grant refs, and denial reasons.
- `POST /public-interest-pools/{pool_id}/throttles`: records abuse throttle or fraud-control refs for a grantee or pool scope.
- `GET /public-interest-pools/{pool_id}/reports`: returns redacted usage and outcome reports.
- `POST /public-interest-pools/{pool_id}/renewals`: renews, expires, or revokes pool or grantee eligibility.

Mutating APIs require actor identity, tenant or stewardship scope, trace id, idempotency key, policy refs, and purpose tag refs where applicable. Stable errors include `purpose_tag_not_verified`, `grantee_not_eligible`, `quota_exceeded`, `fairness_window_exhausted`, `pool_exhausted`, `contribution_not_available`, `abuse_throttle_active`, and `grant_handoff_failed`.

## Event Surface

- `public_interest_pool.drafted`: pool draft created.
- `public_interest_pool.activated`: pool became active.
- `public_interest_pool.contribution_added`: capacity contribution attached.
- `public_interest_pool.eligibility_evaluated`: grantee/workload request evaluated.
- `public_interest_pool.allocation_requested`: pool allocation request opened.
- `public_interest_pool.allocation_authorized`: Overgrant authorization linked.
- `public_interest_pool.allocation_denied`: request denied with reason codes.
- `public_interest_pool.quota_exhausted`: pool, purpose, or grantee quota exhausted.
- `public_interest_pool.throttle_applied`: abuse throttle applied or updated.
- `public_interest_pool.report_published`: redacted usage or outcome report published.
- `public_interest_pool.eligibility_revoked`: grantee or pool eligibility revoked.
- `public_interest_pool.retired`: pool closed with final reports.

Events include pool id, grantee refs, purpose tag refs, contribution refs, grant refs, usage/accounting refs, policy refs, and redacted evidence refs.

## Core Workflow

1. Draft a pool with source refs, contribution scopes, purpose tags, eligible grantees, quotas, fairness rules, and reporting requirements.
2. Validate purpose tags and evidence requirements with Purpose Tag Registry.
3. Ask Overguard to approve pool eligibility, workload/data-class boundaries, quota rules, and abuse controls.
4. Attach donated, sponsored, federation, or native-service surplus capacity refs.
5. Evaluate grantee allocation requests against purpose evidence, quota, fairness, and abuse state.
6. Hand accepted requests to Overgrant for grant authorization.
7. Track usage through Overmeter, ORU Account Service, Seal Ledger, and grant refs.
8. Publish redacted usage/outcome reports and renew, throttle, revoke, or retire pool participation as evidence changes.

## State Machine

Pool lifecycle:

1. `draft`
2. `preflighting`
3. `active`
4. `paused`
5. `exhausted`
6. `renewal_pending`
7. `retired`
8. `revoked`

Allocation request lifecycle:

1. `submitted`
2. `eligibility_checking`
3. `quota_checking`
4. `grant_handoff_pending`
5. `authorized`
6. `denied`
7. `throttled`
8. `expired`
9. `corrected`

Grantee eligibility lifecycle:

1. `proposed`
2. `evidence_required`
3. `eligible`
4. `quota_limited`
5. `throttled`
6. `suspended`
7. `revoked`
8. `renewal_required`

## Policy And Security

- Require verified purpose tags and evidence before purpose-limited allocation.
- Enforce workload class, data class, storage/vault, route, and provider boundary rules through Overguard before allocation.
- Deny private, regulated, secret-bearing, or system-service usage unless a trusted template and specific policy allow it.
- Keep contributor, grantee, and workload private data redacted in public reports.
- Apply abuse throttles and fraud-control refs without deleting usage history.
- Require correction paths for false abuse signals, eligibility mistakes, and reporting errors.
- Do not expose raw central AI stewardship reasoning when policy requires summary-only reporting; store evidence refs and public-safe summaries.

## Metering And Accounting

- Link every pool allocation to Overgrant authorization refs and Overmeter usage refs.
- Use ORU Account Service and Seal Ledger refs for balance projection and immutable usage/accounting evidence.
- Store pool contribution and allocation refs, not balances, prices, invoices, payouts, or ledger entries.
- Native-service surplus routing is a structural stewardship allocation input, not a revenue forecast.
- Reports must reconcile pool usage with grant refs, usage rollups, and Seal Ledger refs.
- Do not charge per internal allocation event or encode speculative financial assumptions.

## Observability And Operations

- Expose active pools, contribution availability, allocation queue, quota exhaustion, fairness windows, throttles, report freshness, and renewal deadlines.
- Provide operator/stewardship views for purpose tags, grantee eligibility, contribution scopes, grant handoff status, and reporting gaps.
- Support simulation mode for a proposed grantee/workload before allocation.
- Alert on pool exhaustion, abnormal allocation concentration, stale outcome reports, and abuse throttle bursts.
- Provide public-safe reports for pool activity, support delivered, and high-level outcomes.

## Failure Modes And Recovery

- Purpose tag missing or unverified: deny allocation until Purpose Tag Registry evidence exists.
- Contribution unavailable or expired: block new allocation and mark pool degraded or exhausted.
- Overgrant handoff fails: keep request pending or denied with reason code; do not invent authorization locally.
- Usage report does not reconcile: mark report blocked and link accounting/Overmeter refs for investigation.
- Abuse throttle active: deny or limit allocation and expose correction path.
- Grantee evidence expires: move eligibility to `renewal_required` and block new allocation.
- Pool policy revoked: pause pool and prevent new allocations while preserving existing accounting evidence.

## Validation Plan

- Eligible grantee can receive pool capacity within quota and fairness windows.
- Ineligible purpose tag, expired evidence, or disallowed workload class is denied with stable reason code.
- Pool allocation creates Overgrant handoff refs and usage can reconcile with Overmeter and Seal Ledger refs.
- Abuse throttle blocks or limits allocation without deleting historical usage.
- Public reports redact private workload and grantee data while preserving accountable usage summaries.
- Native-service surplus routing can attach to a pool as a contribution ref without financial projections.
- Pool replay reconstructs eligibility, quota, grant, usage, and reporting decisions.

## Build Breakdown

1. Define pool, contribution, grantee eligibility, allocation request, fairness window, usage report, and outcome-report schemas.
2. Implement pool draft, activation, contribution, and read APIs.
3. Add Purpose Tag Registry and Overguard preflight checks.
4. Add eligibility/quota/fairness evaluation and Overgrant handoff.
5. Add Overmeter, ORU Account Service, Seal Ledger, and Overwatch refs for usage reporting.
6. Add abuse throttle, renewal, revocation, and correction flows.
7. Add redacted public-interest reports and outcome-report hooks.
8. Prove one pool enforces purpose tag, quota, fairness, usage reconciliation, and denial paths.

## Handoff And Downstream Use

Public-Interest Pool Service hands pool, allocation, quota, report, and purpose refs to Overgrant, Overguard, Overmeter, ORU Account Service, Seal Ledger, Federation Template Service, Fraud Control Service, stewardship reporting, central AI stewardship mechanisms, SDK, CLI, and admin UI.

Downstream services should attach pool refs to grant and usage events instead of treating public-interest capacity as informal free capacity.

## Open Design Questions

Resolved decisions:

- The first Phase 10 proof pool should register the full stewarded purpose-tag set named in the build plan, but only enable allocation for `science`, `education`, and `opensource` tag versions at first. These tags are evidenceable with public project, institution, curriculum, repository, or research-output refs and avoid the heavier privacy, compliance, and outcome-verification burden of early `medical`, `climate`, and `public_infrastructure` allocations. The deferred tags should still exist as inactive or review-only Purpose Tag Registry records so Overgrant, Overguard, and reporting can prove that the proof pool is a narrow policy choice rather than a different taxonomy.
- A grantee becomes eligible for the proof pool when it has a verified tenant or organization identity, an active Purpose Tag Registry validation ref for the requested tag version, a signed purpose/workload attestation, public or steward-visible evidence refs appropriate to the tag, an Overguard dry-run allowing the workload and data class, and no active abuse throttle, suspension, or unresolved eligibility dispute. The proof threshold should accept public artifacts such as institutional pages, project repositories, curriculum links, grant letters, or published research/output refs before requiring manual committee review. Higher-risk, private, regulated, sensitive, or disputed claims must return `needs_more_evidence` or `grantee_not_eligible` and move to steward review instead of lowering the threshold.
- Central AI stewardship recommendations should be stored as append-only `public_interest_recommendation` records, not as direct allocation commands. Each recommendation must include pool refs, grantee/candidate refs, purpose tag version refs, evidence package refs, fairness and quota facts, usage/outcome refs where available, model/run provenance, route refs, confidence, proportionality summary, policy threshold refs, expiry, review state, owning-service target, and appeal/correction path. Humans audit the recommendation through Overwatch replay and redacted reasoning summaries, while Public-Interest Pool Service and Overgrant remain responsible for pool state and grant authorization.
- Public reports should expose only redacted stewardship facts: active pool name/status, enabled purpose tag versions, aggregate contributed and consumed resource dimensions, aggregate grant/source classes, quota and exhaustion summaries, public-safe denial reason-code totals, high-level outcome refs, correction/retraction notices, and redacted Overwatch/Stewardship Reporting refs. Participant-only reports may show the participant's own contribution, allocation request state, quota/fairness window, grant authorization refs, usage rollups, reporting obligations, outcome evidence refs, and active throttle/appeal refs. Operator/steward reports may include stronger evidence refs, identity/contact refs, abuse/fraud refs, and central-AI evidence-package details under audience-specific redaction policy, but raw private workload data, secrets, payment details, and sensitive fraud internals stay out of public artifacts.
- Oversubscription should use a weighted max-min fair-share rule implemented through Overgrant/Public-Interest Pool `fairness_window` records: each eligible grantee receives a per-window cap, burst cap, and refill/expiry behavior; unused allocations return to the pool after the reservation window; and no grantee can exceed its fair share while other eligible grantees in the same purpose pool remain unsatisfied. The first proof pool should use equal weights inside the enabled purpose tags, with small newcomer/starvation protection and abuse throttle overrides. Later weights may be added only as versioned policy facts tied to purpose priority, emergency windows, or stewardship-approved public-interest urgency, never as an opaque central-AI override.
