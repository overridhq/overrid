SDS #55

# Public Provider Onboarding SDS

## Purpose

Admit unknown or semi-trusted providers only into tightly bounded public low-sensitivity capacity.

Public Provider Onboarding collects provider identity, node identity, contact, payout eligibility refs, resource claims, software version, region, accepted workload classes, policy acknowledgements, and initial verification refs. It publishes allowed public capability records only after policy and verification gates pass. It does not grant broad trust, assign private work, run verification itself, decide reputation, execute payouts, or weaken public sandbox restrictions.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [public_provider_onboarding.md](../../service_catalog/federation_public/public_provider_onboarding.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md) |

## Service Family

- Family: Federation and public capacity
- Owning layer: Public-provider enrollment, limited eligibility publication, and onboarding evidence
- Primary data scope: provider enrollment records, node enrollment refs, identity tier refs, policy acknowledgements, resource claims, software-version attestations, region/jurisdiction facts, payout eligibility refs, public workload acceptance contracts, eligibility publication refs, and onboarding audit evidence
- First build phase from service plan: [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md)

## Problem Statement

Unknown public providers can help Overrid scale only if the risk is bounded. Public nodes can fail, cheat, disappear, collude, leak data, or misrepresent resources. Onboarding must create a starting eligibility level without implying broad trust. The platform must prevent normal scheduler behavior from placing private, regulated, secret-bearing, or system-service workloads on public nodes.

This service defines the front door for public providers: collect claims, record acknowledgements, trigger verification and anti-Sybil checks, publish only restricted capability records, and make all denials and eligibility reductions auditable.

## Goals

- Collect provider identity level, node identity, contact, payout eligibility refs, resource claims, software version, jurisdiction/region, accepted workload classes, and policy acknowledgements.
- Enforce that new public providers only start in public low-sensitivity eligibility.
- Trigger Oververify, Reputation and Anti-Sybil Service, Challenge Task Service, Hardware Discovery, Benchmark Runner, and Fraud Control follow-up where required.
- Publish only allowed capability records to Overregistry and scheduling/policy rails.
- Attach public sandbox profile requirements to all public-provider eligibility.
- Make failed verification, missing acknowledgement, stale software, payout risk, and fraud signals reduce or block eligibility.
- Preserve provider correction, appeal, and offboarding paths.

## Non-Goals

- Do not verify identity or trust directly; Oververify and Reputation and Anti-Sybil Service produce verification and risk evidence.
- Do not install or supervise nodes; Node Installer and Overcell own node agent lifecycle.
- Do not benchmark resources; Benchmark Runner owns measured capacity evidence.
- Do not execute workloads, schedule work, or issue leases.
- Do not decide payouts, holds, or external payment eligibility; Overbill, Provider Payout Service, Overclaim, and accounting services own those records.
- Do not allow public providers to host private, regulated, secret-bearing, or system-service workloads.
- Do not add pricing, revenue forecasts, customer counts, blockchain mechanics, NFT mechanics, or per-transaction fee economics.

## Primary Actors And Clients

- Public provider applicant submitting enrollment and node claims.
- Overgate, Overpass, Overkey, and Overtenant handling identity, credentials, and tenant/account boundaries.
- Overcell and Node Installer providing node-agent enrollment and version evidence.
- Hardware Discovery and Benchmark Runner validating resource claims.
- Oververify and Reputation and Anti-Sybil Service providing verification tier and risk refs.
- Public Sandbox Profile, Overguard, Oversched, and Overregistry consuming public eligibility records.
- Overbill and Provider Payout Service consuming payout eligibility refs and hold status.
- Fraud Control Service, Challenge Task Service, Overwatch, and Overclaim consuming onboarding evidence and corrections.

## Dependencies

- [Overpass](../control_plane/overpass.md), [Overkey](../control_plane/overkey.md), and [Overtenant](../control_plane/overtenant.md) for provider identity, credentials, and tenant/account scope.
- [Node Installer](../execution_scheduling/node_installer.md) and [Overcell](../execution_scheduling/overcell.md) for node-agent enrollment, heartbeat, and software version evidence.
- [Hardware Discovery](../execution_scheduling/hardware_discovery.md) and [Benchmark Runner](../execution_scheduling/benchmark_runner.md) for observed resource claims and measured capacity evidence.
- [Oververify](../trust_policy_verification/oververify.md) and [Reputation and Anti-Sybil Service](../trust_policy_verification/reputation_anti_sybil_service.md) for identity tiers, anti-Sybil signals, risk, and eligibility recommendations.
- [Public Sandbox Profile](public_sandbox_profile.md), [Workload Classifier](../trust_policy_verification/workload_classifier.md), and [Overguard](../trust_policy_verification/overguard.md) for allowed workload/data-class boundaries.
- [Overregistry](../control_plane/overregistry.md) for published capability and provider records.
- [Overbill](../accounting/overbill.md), [Provider Payout Service](../accounting/provider_payout_service.md), [Fraud Control Service](fraud_control_service.md), and [Overclaim](../trust_policy_verification/overclaim.md) for payout eligibility, holds, disputes, and corrections.

## Owned Responsibilities

Public Provider Onboarding owns:

- Public provider enrollment application records.
- Provider contact and policy acknowledgement records.
- Node enrollment refs and accepted software-version requirements.
- Declared resource claim records and verification-request refs.
- Initial public workload acceptance contract.
- Payout eligibility reference capture and hold/status visibility.
- Public eligibility publication records sent to Overregistry, Overguard, and scheduling.
- Onboarding denial, suspension, correction, appeal, and offboarding records.
- Redacted provider onboarding summaries for operators and public-provider support.

## Data Model

- `public_provider_enrollment`: `enrollment_id`, `provider_ref`, `tenant_ref`, `identity_level_ref`, `contact_ref`, `region_or_jurisdiction`, `policy_ack_refs`, `state`, `submitted_at`, and `audit_refs`.
- `public_node_enrollment`: node refs, Overcell enrollment refs, software version, hardware discovery refs, benchmark refs, sandbox compatibility, network facts, and heartbeat readiness.
- `resource_claim`: declared compute/GPU/storage/network/model/service dimensions, observed evidence refs, benchmark refs, confidence, and publication state.
- `public_workload_acceptance_contract`: allowed workload classes, data classes, runtime caps, memory caps, egress policy, no-secret flag, no-private-data flag, no-system-service flag, and sandbox profile ref.
- `verification_tier_ref`: refs to Oververify and anti-Sybil outcomes with freshness, confidence, and appeal status.
- `payout_eligibility_ref`: Overbill/Provider Payout Service refs, payout hold refs, and allowed payout state without storing payment details.
- `capability_publication`: published provider/node/capability facts with allowed scopes, policy refs, registry refs, and expiry.
- `onboarding_correction`: provider correction, appeal, or offboarding evidence with decision refs and downstream update refs.

Enrollment records are append-only after submission. Changed resource claims, software versions, verification tiers, or eligibility create new evaluation/publication records.

## API Surface

- `POST /public-providers/enrollments`: creates a provider enrollment request with identity, contact, region, policy acknowledgements, and payout eligibility refs.
- `POST /public-providers/{provider_id}/nodes`: attaches a node enrollment ref, software version, Overcell ref, and resource claims.
- `POST /public-providers/{provider_id}/acknowledgements`: records accepted public-provider policy, sandbox, payout, dispute, and fraud-control terms.
- `POST /public-providers/{provider_id}/verification-request`: requests verification, anti-Sybil evaluation, benchmark, discovery, or challenge follow-up.
- `POST /public-providers/{provider_id}/eligibility/evaluate`: evaluates provider/node eligibility for public low-sensitivity capacity.
- `GET /public-providers/{provider_id}/eligibility`: returns current eligibility, blockers, verification tier refs, sandbox refs, and capability publication refs.
- `POST /public-providers/{provider_id}/publish-capabilities`: publishes allowed public capability records after policy checks.
- `POST /public-providers/{provider_id}/suspend`: suspends onboarding or public eligibility with reason and appeal path.
- `POST /public-providers/{provider_id}/corrections`: records provider correction, appeal, or offboarding evidence.

Mutating APIs require actor/provider identity, trace id, idempotency key, policy refs, and source evidence refs. Stable errors include `identity_level_insufficient`, `policy_ack_missing`, `node_agent_missing`, `software_version_stale`, `resource_claim_unverified`, `sandbox_not_compatible`, `payout_hold_active`, `anti_sybil_risk_high`, and `workload_class_not_public`.

## Event Surface

- `public_provider_onboarding.enrollment_submitted`: provider enrollment request accepted.
- `public_provider_onboarding.policy_acknowledged`: policy acknowledgement recorded.
- `public_provider_onboarding.node_attached`: node enrollment ref attached.
- `public_provider_onboarding.verification_requested`: verification, anti-Sybil, challenge, benchmark, or discovery request emitted.
- `public_provider_onboarding.eligibility_evaluated`: eligibility evaluation completed.
- `public_provider_onboarding.capabilities_published`: public capability records published.
- `public_provider_onboarding.eligibility_denied`: provider or node denied with reason codes.
- `public_provider_onboarding.eligibility_reduced`: verification or fraud evidence reduced eligibility.
- `public_provider_onboarding.suspended`: provider/node suspended.
- `public_provider_onboarding.correction_recorded`: correction or appeal evidence recorded.
- `public_provider_onboarding.offboarded`: provider exited public pool eligibility.

Events include provider refs, node refs, verification tier refs, sandbox refs, allowed workload class refs, policy refs, registry refs, payout/hold refs where allowed, and redacted evidence refs.

## Core Workflow

1. Provider submits enrollment with identity refs, contact refs, region, policy acknowledgements, payout eligibility refs, and initial node/resource claims.
2. Onboarding validates required fields, software version, Overcell/node refs, and public-policy acknowledgement.
3. Trigger Oververify, anti-Sybil, hardware discovery, benchmark, and challenge follow-up as required by risk tier.
4. Build a public workload acceptance contract that allows only public low-sensitivity workloads with sandbox restrictions.
5. Ask Overguard whether eligibility can be published based on verification tier, risk, software, resource evidence, payout status, and sandbox compatibility.
6. Publish only allowed capability records to Overregistry, Overguard, and scheduling rails.
7. Monitor updates from fraud, verification, anti-Sybil, payout holds, disputes, and provider corrections.
8. Reduce, suspend, restore, or offboard eligibility through explicit records and Overwatch evidence.

## State Machine

Enrollment lifecycle:

1. `draft`
2. `submitted`
3. `awaiting_acknowledgement`
4. `awaiting_verification`
5. `awaiting_node_evidence`
6. `eligible_public_limited`
7. `blocked`
8. `suspended`
9. `offboarded`
10. `rejected`

Node eligibility lifecycle:

1. `claimed`
2. `agent_pending`
3. `discovery_pending`
4. `benchmark_pending`
5. `sandbox_checking`
6. `publishable_public_limited`
7. `published`
8. `degraded`
9. `revoked`

Capability publication lifecycle:

1. `draft`
2. `policy_checked`
3. `published`
4. `expired`
5. `reduced`
6. `suspended`
7. `revoked`

## Policy And Security

- New public providers can only become eligible for public low-sensitivity workloads.
- Scheduler and policy rails must hard-deny private, regulated, secret-bearing, and system-service workloads for public providers.
- Require public sandbox profile compatibility before capability publication.
- Require policy acknowledgement for fraud controls, challenge tasks, payout holds, dispute windows, data restrictions, and offboarding rules.
- Treat all resource claims as untrusted until observed by Hardware Discovery and measured by Benchmark Runner where required.
- Store payout eligibility refs only; do not store payment credentials or external payment details.
- Keep provider contact and identity details redacted from public capability records.
- Preserve correction and appeal paths for failed verification, false fraud signals, and onboarding mistakes.

## Metering And Accounting

- Emit onboarding, verification request, discovery/benchmark handoff, and capability publication usage to Overmeter where material.
- Link public-provider work to provider refs, node refs, public workload class, sandbox profile, payout eligibility refs, and fraud/hold refs.
- Provider payout eligibility is referenced from Overbill/Provider Payout Service; onboarding does not compute earnings or create payments.
- Seal Ledger and ORU records remain downstream accounting evidence, not onboarding state.
- Do not encode onboarding fees, financial projections, price forecasts, or per-transaction costs.

## Observability And Operations

- Expose enrollment queue, blocked enrollments, missing acknowledgements, verification backlog, stale software versions, unpublished capability records, suspended providers, and appeal/correction queues.
- Provide provider-facing status with actionable reason codes and no internal policy leakage.
- Provide operator views for risk, evidence freshness, sandbox compatibility, payout hold state, and publication refs.
- Alert when public providers attempt to accept disallowed workload classes or stale node software persists.
- Support replay of eligibility publication from enrollment facts, verification refs, policy refs, and sandbox refs.

## Failure Modes And Recovery

- Missing policy acknowledgement: block enrollment and return required acknowledgement ids.
- Identity tier insufficient: keep enrollment blocked and request verification.
- Node agent missing or stale: block capability publication.
- Resource claims unverified: publish no capacity or publish reduced capacity according to policy.
- Public sandbox incompatible: deny workload eligibility until fixed.
- Anti-Sybil risk high: block or rate-limit publication and link appeal path.
- Payout hold active: allow or block work according to policy but clearly mark payout eligibility state.
- Fraud signal arrives after publication: reduce or suspend eligibility and emit registry/policy update.
- Provider correction accepted: publish a new eligibility evaluation and downstream update refs.

## Validation Plan

- New public providers cannot receive private, regulated, secret-bearing, or system-service workloads.
- Missing policy acknowledgement blocks enrollment.
- Failed verification reduces or blocks eligibility with stable reason codes.
- Public sandbox incompatibility prevents capability publication.
- Resource claims are not published as trusted capacity until evidence exists.
- Published capability records include only allowed public low-sensitivity scopes and redacted provider facts.
- Fraud, anti-Sybil, payout hold, or dispute signals reduce/suspend eligibility without deleting history.
- Onboarding replay reconstructs why a provider/node was eligible, denied, reduced, suspended, or offboarded.

## Build Breakdown

1. Define provider enrollment, node enrollment, resource claim, public workload acceptance, verification tier, payout eligibility, capability publication, and correction schemas.
2. Implement enrollment and acknowledgement APIs with stable reason codes.
3. Add node attachment, software version, Overcell ref, hardware discovery, and benchmark handoff.
4. Add verification, anti-Sybil, fraud, challenge, and payout eligibility refs.
5. Add public workload acceptance contract and public sandbox profile gate.
6. Add Overguard eligibility evaluation and Overregistry capability publication.
7. Add suspension, reduction, correction, appeal, and offboarding flows.
8. Prove an unknown public node can register only into bounded public low-sensitivity capacity.

## Handoff And Downstream Use

Public Provider Onboarding hands limited public eligibility refs to Overregistry, Overguard, Oversched, Public Sandbox Profile, Fraud Control Service, Reputation and Anti-Sybil Service, Oververify, Challenge Task Service, Overbill, Provider Payout Service, Overclaim, SDK, CLI, and admin UI.

Downstream services must consume published capability and eligibility refs rather than trusting provider-submitted claims.

## Open Design Questions

Resolved decisions:

- The minimum tier for the first real public low-sensitivity workload is `public_provider_limited`: an Overpass/Overtenant provider identity with verified control of the provider account, reachable contact refs, region or jurisdiction refs where policy requires them, current public-provider policy acknowledgements, an Overcell node enrollment with current heartbeat and software-version refs, public sandbox compatibility, Oververify `public_low_sensitivity` eligibility, Reputation and Anti-Sybil signals no worse than restricted/new-provider risk, and an Overguard allow decision for the specific public-low-sensitivity workload. Lower tiers may enroll, attach nodes, run onboarding checks, and receive challenge/discovery tasks, but they cannot receive requester workloads. This tier never grants private tenant, trusted federation, regulated, secret-bearing, or system-service eligibility.
- Before Benchmark Runner evidence exists, only candidate capability facts may be published, and they must be marked as restricted, expiring, and non-authoritative. Self-declared resource claims stay at confidence no higher than `0.25` and are visible mainly to onboarding, verification, challenge, and operator flows. Hardware Discovery or Overcell-observed facts may publish coarse capability presence at confidence no higher than `0.50`, such as CPU architecture/core band, memory band, GPU count/model family, storage class, network reachability class, runtime support, public sandbox compatibility, and locality refs. Schedulable capacity, earning-capable capacity, higher confidence, and normalized resource-card publication require benchmark and challenge evidence; otherwise Oversched may use the facts only for bounded probes, challenges, or heavily capped public test work.
- Public providers must acknowledge the current public-provider, sandbox, fraud-control, payout-hold, dispute, offboarding, and data-restriction terms during enrollment, before first capability publication, and whenever a material policy, sandbox profile, payout/dispute rule, region/compliance requirement, or workload-acceptance contract changes. Active providers reacknowledge at least every 90 days; probationary, recently suspended, recently restored, or high-risk providers reacknowledge at least every 30 days while the stricter state remains active. A stale acknowledgement blocks new capability publication and new work until refreshed, but historical eligibility, usage, and accounting evidence remains append-only.
- Work may continue while payout is delayed only when Provider Payout Service, Overguard, Oververify, and Reputation/Anti-Sybil agree that the hold is payout-delay-only and the provider remains eligible for bounded public low-sensitivity work. Non-blocking states include new-provider maturation hold, normal public-provider dispute window, inconclusive or pending challenge review, duplicate-execution-required review, active public-pool throttle, payout batch period or rail-minimum waiting, external rail unavailable, and payout destination review where no compliance or ownership blocker exists. Blocking states stop new work as well as payout: high-confidence fraud or Sybil evidence, confirmed challenge or duplicate-execution fraud, sandbox escape, unauthorized egress, attempted secret/private/regulated data access, payout destination revocation, illegal or expired compliance facts, unresolved high-severity Overclaim dispute, suspension, or any Overguard denial.
- Provider-facing explanations may show the affected provider, node, payout period, public workload class, current onboarding/eligibility state, safe reason-code categories, severity and confidence bands, policy/evaluator versions, acknowledgement freshness, sandbox/profile refs, redacted evidence refs, remediation steps, recheck options, hold/throttle class, and Overclaim appeal or correction refs. They must not expose raw identity or payout material, other-provider identities, cluster membership, exact IP/device/fingerprint/payout hashes, fraud thresholds, model weights, challenge payloads or randomization, private tenant evidence, secret refs, topology, operator notes, incident-response details, or internal anti-Sybil/fraud heuristics. Operator and stewardship views can dereference deeper evidence only through Overwatch redaction profiles and audited access decisions.
