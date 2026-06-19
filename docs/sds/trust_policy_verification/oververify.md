SDS #34

# Oververify SDS

## Purpose

Maintain provider and node verification records from identity evidence, node enrollment evidence, benchmark evidence, challenge outcomes, result checks, reliability history, dispute history, abuse markers, certification records, and explainable trust signals.

Oververify is the evidence-backed trust and eligibility service. It publishes scheduler and policy signals from stored evidence. It does not execute challenges, run benchmarks, schedule workloads, handle payouts, or hide trust behind opaque scores.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [oververify.md](../../service_catalog/trust_policy_verification/oververify.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md) |

## Service Family

- Family: Trust, policy, verification, and disputes
- Owning layer: Provider/node verification records, evidence normalization, trust signals, eligibility publication, certification, and trust explanation
- Primary data scope: provider verification records, node verification records, attestation evidence, benchmark evidence refs, challenge outcome refs, reliability windows, dispute markers, abuse markers, trust signals, eligibility signals, certification records, and explanation bundles
- First build phase from service plan: [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md).

## Problem Statement

Overrid must know which providers and nodes are eligible for private workloads, trusted federation, public low-sensitivity work, system-service workloads, payout, grants, and future public-provider participation. Trust cannot be a one-time checkbox. Hardware can drift, benchmarks can become stale, challenge failures can appear, disputes can reveal invalid work, and public providers can behave adversarially.

Oververify must turn evidence into explainable signals that Overguard, Oversched, Overclaim, Provider Payout Service, Fraud Control Service, and central AI stewardship can use. The signals must be correctable and auditable, not speculative reputation theater.

## Goals

- Store provider and node verification records with identity, enrollment, ownership, capability, benchmark, challenge, reliability, dispute, and abuse evidence refs.
- Validate benchmark and challenge evidence before it affects eligibility.
- Publish explicit eligibility signals for workload classes, trust classes, provider classes, payout status, and review requirements.
- Produce explainable trust signals from evidence categories and policy versions rather than opaque global scores.
- Support downgrade, probation, suspension, revocation, recheck, and certification workflows.
- Preserve evidence history and signal recomputation history for disputes and incident review.
- Support private trusted nodes first, then stricter public-provider verification and anti-Sybil handoff in Phase 11.
- Feed Overguard, Oversched, Challenge Task Service, Overclaim, Provider Payout Service, public-provider onboarding, and central AI review through stable contracts.

## Non-Goals

- Do not run benchmarks. Benchmark Runner owns benchmark execution and raw result evidence.
- Do not issue or execute challenges. Challenge Task Service owns challenge orchestration.
- Do not schedule workloads. Oversched owns placement decisions.
- Do not enforce admission policy directly. Overguard consumes verification signals.
- Do not mutate payouts, holds, or ledger records. Accounting services own those effects.
- Do not claim full identity truth or legal compliance without explicit evidence levels and compliance refs.
- Do not publish a single unexplained trust number as the only scheduler or policy signal.

## Primary Actors And Clients

- Overregistry, supplying provider, node, resource, and capability records.
- Overcell, supplying node enrollment, heartbeat, and command-acceptance refs.
- Hardware Discovery and Benchmark Runner, supplying inventory and measured-capacity evidence.
- Challenge Task Service, supplying challenge outcomes and anomaly evidence.
- Overwatch, supplying event history, incidents, evidence bundles, and audit refs.
- Overclaim, supplying dispute outcomes, appeals, and corrections.
- Overguard and Oversched, consuming trust, eligibility, and required-recheck signals.
- Provider Payout Service, Overbill, Overgrant, and public-provider onboarding, consuming payout eligibility and hold-risk signals.
- Fraud Control Service, Reputation and Anti-Sybil Service, and central AI stewardship, consuming risk and explanation bundles.
- Providers, operators, admin UI, CLI, and SDK, reading authorized verification status and remediation hints.

## Dependencies

- [Overregistry](../control_plane/overregistry.md) for provider, node, capability, package, app, and public-provider records.
- [Overcell](../execution_scheduling/overcell.md) for node enrollment, heartbeat, lifecycle, and command-acceptance refs.
- [Hardware Discovery](../execution_scheduling/hardware_discovery.md) for observed inventory and runtime facts.
- [Benchmark Runner](../execution_scheduling/benchmark_runner.md) for benchmark suite refs, measured capacity, raw samples, anomaly refs, and invalidations.
- [Challenge Task Service](challenge_task_service.md) for challenge assignments, results, consequence proposals, and replay bundles.
- [Overwatch](../control_plane/overwatch.md) for audit events, evidence bundles, incidents, trace refs, and integrity checkpoints.
- [Overclaim](overclaim.md) for dispute outcomes, appeal state, resolution refs, and correction markers.
- [Overguard](overguard.md) and [Oversched](../execution_scheduling/oversched.md) for policy and scheduler consumption of eligibility signals.
- [Provider Payout Service](../accounting/provider_payout_service.md), [Overbill](../accounting/overbill.md), and [Seal Ledger](../accounting/seal_ledger.md) for hold and payout eligibility consumers.
- [Public Provider Onboarding](../federation_public/public_provider_onboarding.md), [Fraud Control Service](../federation_public/fraud_control_service.md), and [Reputation and Anti-Sybil Service](reputation_anti_sybil_service.md) for Phase 11 public-supply hardening.

## Owned Responsibilities

Oververify owns:

- Provider verification records and verification levels.
- Node verification records and node eligibility state.
- Evidence intake validation from registry, node, hardware, benchmark, challenge, dispute, reliability, and abuse sources.
- Trust signal computation and explanation.
- Workload-class eligibility signals for policy and scheduler consumers.
- Certification records, recheck requirements, probation, suspension, revocation, and reinstatement records.
- Signal recomputation history and replay bundles.
- Provider-facing remediation hints and operator-facing evidence timelines.

Oververify must keep raw evidence refs linked to source services and must not silently replace evidence with summary-only fields.

## Data Model

The first implementation should define:

- `provider_verification_record`: provider id, identity level, ownership refs, contact/payout eligibility refs, accepted workload classes, jurisdiction/region refs, policy acknowledgement, verification state, and current eligibility refs.
- `node_verification_record`: node id, provider id, enrollment refs, hardware inventory refs, benchmark refs, heartbeat refs, software version refs, trust class, workload eligibility, recheck requirements, and node state.
- `attestation_evidence`: source service, attestation type, evidence level, signature, integrity hash, expiry, revocation refs, and validation state.
- `benchmark_evidence_ref`: benchmark suite id/version, node id, measured resource dimensions, raw sample refs, normalized result refs, anomaly refs, invalidation refs, and freshness window.
- `challenge_outcome_ref`: challenge assignment id, challenge type, result state, severity, evidence refs, consequence proposal refs, dispute refs, and expiry/recheck window.
- `reliability_window`: provider or node id, time window, completed workloads, failed workloads, timeouts, cancellations, no-shows, result inconsistencies, incident refs, and confidence score explanation.
- `dispute_marker`: claim id, affected refs, status, resolution, severity, hold refs, correction refs, and impact on verification.
- `abuse_marker`: source service, marker type, severity, confidence, evidence refs, expiry, appeal refs, and visibility class.
- `trust_signal`: target id, signal domain, signal value, allowed workload classes, denied workload classes, confidence, reason codes, evidence refs, policy version, and computed-at timestamp.
- `eligibility_signal`: target id, consumer service, eligibility state, required restrictions, recheck requirement, hold requirement, and expiry.
- `certification_record`: provider/node id, certification type, scope, issuing policy, evidence refs, validity window, renewal rule, revocation refs, and public visibility flag.
- `trust_explanation_bundle`: target id, current signals, evidence summary, reason codes, redaction profile, remediation hints, and replay bundle ref.
- `signal_recompute_job`: target scope, trigger refs, old signal refs, new signal refs, evaluator version, policy version, and replay result.

Common envelope fields:

- `id`, `provider_id`, `node_id` where applicable, `tenant_id` when scoped, `trace_id`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

Oververify APIs are internal, provider-facing, and operator-facing:

- `POST /verify/providers/{provider_id}/evidence`: attach provider verification evidence refs.
- `POST /verify/nodes/{node_id}/evidence`: attach node verification evidence refs.
- `POST /verify/benchmarks/{node_id}`: ingest validated benchmark evidence refs.
- `POST /verify/challenges/{assignment_id}`: ingest challenge outcome refs.
- `POST /verify/disputes/{claim_id}`: ingest dispute resolution refs.
- `POST /verify/recompute`: recompute trust and eligibility signals for a provider, node, or cohort.
- `GET /verify/providers/{provider_id}`: read authorized provider verification status and eligibility.
- `GET /verify/nodes/{node_id}`: read authorized node verification status and eligibility.
- `GET /verify/eligibility/{target_id}`: read scheduler/policy-ready eligibility signals.
- `GET /verify/{target_id}/explain`: return evidence-backed explanation and remediation hints.
- `POST /verify/{target_id}/certifications`: issue, renew, revoke, or suspend a certification record.
- `POST /verify/{target_id}/state`: apply signed probation, suspension, revocation, or reinstatement action.

API requirements:

- Evidence ingest must validate source service, signature or integrity hash, freshness, and target identity.
- Signal reads must declare consumer service and redaction class.
- Recompute requests must be idempotent and must record evaluator and policy versions.
- Provider-facing explanations must include appealable reason codes and remediation hints without exposing private fraud heuristics.

## Event Surface

- `oververify.provider_record_created`: provider verification record created.
- `oververify.node_record_created`: node verification record created.
- `oververify.evidence_attached`: evidence ref accepted.
- `oververify.evidence_rejected`: evidence ref rejected with reason code.
- `oververify.benchmark_ingested`: benchmark evidence accepted.
- `oververify.challenge_ingested`: challenge outcome accepted.
- `oververify.signals_recomputed`: trust and eligibility signals changed or confirmed.
- `oververify.eligibility_changed`: scheduler or policy eligibility changed.
- `oververify.certification_changed`: certification issued, renewed, suspended, revoked, or expired.
- `oververify.state_changed`: target moved to probation, degraded, suspended, revoked, or reinstated.
- `oververify.explanation_exported`: explanation bundle generated.

Events must include target id, evidence refs, signal refs, reason codes, policy/evaluator version, and trace id.

## Core Workflow

1. Provider and node records are created from Overregistry and Overcell enrollment facts.
2. Oververify ingests identity, enrollment, hardware, benchmark, challenge, dispute, reliability, and abuse evidence refs.
3. Evidence validators check source integrity, freshness, target identity, and policy compatibility.
4. Signal recomputation evaluates evidence categories against the verification policy version.
5. Oververify publishes trust and eligibility signals for Overguard, Oversched, payout, grant, public-provider, and central AI consumers.
6. Challenge failures, dispute outcomes, stale benchmarks, reliability drops, or abuse markers trigger downgrade, recheck, probation, suspension, or hold recommendations.
7. Provider-facing explanations expose reason codes, evidence summaries, and remediation steps.
8. Overclaim handles appeals or disputes, and accepted corrections trigger signal recomputation.

## State Machine

Provider/node verification lifecycle:

1. `unverified`: record exists but has insufficient evidence.
2. `pending_evidence`: evidence is expected or under validation.
3. `verified`: minimum evidence requirements are met for listed scopes.
4. `eligible`: target may receive listed workload classes under restrictions.
5. `probation`: target may run only restricted workloads until recheck or review.
6. `degraded`: trust or eligibility reduced because of stale, failed, or disputed evidence.
7. `recheck_required`: target must pass benchmark, challenge, identity, or enrollment recheck.
8. `suspended`: target cannot receive new work until review or correction.
9. `revoked`: target is no longer eligible under current policy.
10. `disputed`: claim or appeal is active and may affect eligibility.
11. `retired`: node or provider voluntarily leaves or is replaced.

Evidence lifecycle:

1. `received`: evidence ref submitted.
2. `validating`: source and freshness checks are running.
3. `accepted`: evidence can affect signals.
4. `rejected`: evidence failed validation.
5. `superseded`: newer evidence replaced its effect.
6. `expired`: freshness window passed.
7. `disputed`: evidence is under claim or appeal.

Signals are append-only. A recompute creates new signal records linked to prior signals and trigger refs.

## Policy And Security

- Trust signals must cite evidence categories and reason codes; opaque scores alone are not acceptable.
- Eligibility must be workload-class specific, especially for private tenant, trusted federation, public low-sensitivity, regulated/secret-bearing, and system-service workloads.
- Secret-bearing and regulated work require stronger verification levels and must never inherit public-provider eligibility.
- Public-provider signals must include anti-Sybil, payout, challenge, abuse, and rate-limit context from Phase 11 services.
- Evidence source services must be authenticated and versioned.
- Provider-facing explanations must support correction and appeal without revealing private tenant data or fraud heuristics.
- Manual verification actions require signed operator identity, evidence refs, expiry where applicable, and Overwatch audit.
- Reinstatement after suspension must require explicit evidence and policy-version refs.

## Metering And Accounting

Oververify does not bill or settle. It emits accounting-relevant trust refs:

- Payout eligibility and payout-hold recommendation refs for Provider Payout Service.
- Grant eligibility refs for Overgrant when trust class affects sponsored resources.
- Provider risk and verification class refs for Overmark cost-class and placement context.
- Dispute and challenge refs for Seal Ledger and Overbill correction workflows.
- Verification computation, explanation export, and evidence storage usage facts for internal cost visibility.

Accounting consumers must act through their own services and append-only records.

## Observability And Operations

- Dashboards should show provider/node verification state, evidence freshness, benchmark freshness, challenge outcomes, reliability windows, dispute impact, suspension/reinstatement counts, and eligibility distribution by workload class.
- Operators need evidence timelines that join enrollment, inventory, benchmark, challenge, workload, dispute, and payout refs.
- Alerts should fire on expired evidence for active nodes, sudden reliability drops, repeated challenge failures, high public-provider anomaly rates, and trust-signal recompute failures.
- Signal recomputation must support scoped backfills, dry runs, replay, and audit export.
- Certification records must have expiry, renewal, and revocation monitoring.

## Failure Modes And Recovery

- Missing evidence: keep target unverified or blocked for affected workload classes.
- Stale benchmark: downgrade or require recheck rather than assuming capacity remains valid.
- Conflicting evidence: apply stricter eligibility or require review based on policy.
- Challenge failure under dispute: hold or restrict according to claim state and severity.
- Recompute failure: preserve previous signals, mark target recheck or review required, and alert operators.
- Evidence source outage: do not accept new unauthenticated evidence; keep current signals until expiry policy forces downgrade.
- False positive suspension: use Overclaim appeal/correction refs to recompute signals.
- Public-provider Sybil cluster: feed Fraud Control Service and Reputation and Anti-Sybil Service before broad eligibility changes.

## Validation Plan

The service implementation plan lists these requirements:

- Scheduler eligibility changes when verification evidence changes.
- Challenge failures reduce trust or create holds.
- Trust scores can be explained from stored evidence.

Additional SDS-level validation:

- Contract tests for provider evidence, node evidence, benchmark ingest, challenge ingest, dispute ingest, recompute, eligibility read, explain, certification, and state-change APIs.
- Evidence validation tests for source integrity, freshness, target identity, rejection, expiration, and supersession.
- Signal recomputation tests proving benchmark, challenge, dispute, reliability, and abuse evidence affect eligibility as expected.
- Scheduler integration tests proving Oversched consumes eligibility changes.
- Policy integration tests proving Overguard denies insufficient trust classes.
- Redaction tests for provider-facing and operator-facing explanations.
- Appeal/correction tests proving Overclaim outcomes recompute signals without mutating prior evidence.

## Build Breakdown

1. Define provider, node, evidence, benchmark, challenge, dispute, trust-signal, eligibility-signal, certification, explanation, and recompute schemas.
2. Implement provider and node verification records from Overregistry and Overcell refs.
3. Add benchmark evidence validation and freshness logic.
4. Add challenge outcome ingestion and consequence mapping to eligibility.
5. Add reliability, dispute, and abuse markers.
6. Add explainable trust and eligibility signal computation.
7. Add scheduler and policy read APIs for Overguard and Oversched.
8. Add certification, probation, suspension, revocation, reinstatement, and appeal/correction workflows.
9. Add public-provider verification hardening for Phase 11.

## Handoff And Downstream Use

Oververify hands eligibility and trust-signal refs to Overguard, Oversched, Challenge Task Service, Provider Payout Service, Overgrant, Overmark, Public Provider Onboarding, Fraud Control Service, Reputation and Anti-Sybil Service, Overclaim, central AI stewardship, SDK, CLI, and admin UI.

## Open Design Questions

Resolved decisions:

- Mandatory evidence is workload-class specific. Every provider or node needs provider identity/account refs where applicable, node enrollment refs, software-version refs, policy acknowledgement refs, Overwatch audit refs, and source-authenticated evidence before any eligibility signal is published. `system_service`, `regulated_or_secret_bearing`, and secret-bearing private workloads require the strongest evidence set: explicit operator or service authority refs, current Overcell heartbeat and command-acceptance refs, current Hardware Discovery and Benchmark Runner refs for claimed capacity, current Challenge Task Service refs for liveness, sandbox behavior, resource class, and result consistency, no blocking Overclaim dispute or high-severity abuse marker, Overguard policy decision refs, and any Overkey/Overvault secret or compliance refs required by policy. `private_tenant` and `trusted_federation` workloads require trusted-node or federation-provider identity, ownership/enrollment, benchmark freshness, challenge freshness, reliability windows, dispute/abuse history, region or jurisdiction refs where policy needs them, and explicit workload-class eligibility. `public_low_sensitivity` eligibility requires public-provider onboarding, identity tier, public sandbox compatibility, node uniqueness or anti-Sybil/risk refs, resource evidence, challenge history, payout/hold visibility, and must always deny private data, secrets, regulated data, and system-service work. `research_public_interest` uses the stricter of its data-sensitivity policy and capacity-source policy; public capacity also requires public-provider sandbox, duplicate-execution, fraud-control, and grant-risk refs.
- Founder-owned seed hardware starts as a private controlled bootstrap environment, not as permanently trusted infrastructure. In Phases 2 through 4, founder-owned nodes may receive private seed and narrow system-service work only after operator-owned inventory evidence, Overcell enrollment, heartbeat and command acceptance, hardware discovery, benchmark evidence, baseline challenge refs, software-version refs, and Overwatch audit refs are present. Trusted federation providers start more restricted: they need provider identity and organization refs, node ownership/enrollment evidence, policy acknowledgements, region or jurisdiction facts, benchmark and challenge evidence, reliability history or probationary windows, no unresolved high-severity disputes or abuse markers, and an explicit federation agreement, grant, or certification scope before `trusted_federation` eligibility is published. Neither founder-owned nor federation status can automatically grant public-provider, regulated, secret-bearing, payout, or future system-service eligibility; each class needs its own evidence and policy version.
- Trust confidence decays through policy-versioned freshness windows on each evidence category, not through an unexplained global score. Heartbeat and command-acceptance evidence expire fastest and should move active placement to `recheck_required`, `probation`, or scheduler restriction when stale. Benchmark evidence decays by resource dimension: stale GPU, CPU, storage, bandwidth, or runtime samples reduce the corresponding capacity confidence and may cap placement before fully blocking the node. Challenge outcomes decay by challenge family and risk class: recent high-confidence failures override age, while stale pass results require recheck before higher-trust work. Reliability, dispute, abuse, and anti-Sybil windows decay only by appending new signal records with policy/evaluator versions; accepted Overclaim corrections can restore confidence through recompute but never delete prior evidence. When multiple evidence windows conflict, Oververify publishes the stricter workload-class eligibility until fresh evidence or a correction ref justifies restoration.
- Automatic payout-hold requests are limited to trust changes that can affect earned value, settlement safety, fraud exposure, or public-provider payout risk. Oververify should emit payout-hold recommendation refs for fabricated evidence, impossible benchmark claims, high-confidence challenge or duplicate-execution fraud, repeated controlled-result inconsistency, sandbox escape, unauthorized egress, attempted secret or private-data access, active high-severity Overclaim disputes, public-provider Sybil or fraud-control markers, suspension or revocation for integrity reasons, and compliance or payout-eligibility blockers. Missing, stale, inconclusive, or low-severity evidence should normally create scheduler restrictions, recheck requirements, probation, lower capacity confidence, or review-required signals without a payout hold unless the target already has payable work in an affected dispute or fraud window. Oververify only recommends holds with evidence refs, reason codes, severity, scope, and release conditions; Provider Payout Service, Overclaim, Seal Ledger, Overbill, and other accounting services own the actual hold, release, correction, and finality records.
- Provider-facing explanations should be actionable and appealable, not forensic. Providers may see current verification state, affected provider/node/resource/workload classes, user-safe reason codes, policy and evaluator versions, freshness state, non-sensitive metric bands or confidence deltas, redacted evidence refs, remediation steps, recheck requirements, dispute or appeal windows, and whether the outcome is a scheduler restriction, payout-hold recommendation, probation, suspension, or revocation. Provider-facing output must not expose challenge payload selection, randomization strategy, comparator internals, fraud or anti-Sybil heuristics, other-provider data, private tenant evidence, secret or regulated data, exact provider topology, raw payout/identity material, operator notes, or sensitive incident details. Operator and stewardship views may dereference deeper evidence only through Overwatch redaction profiles, access-decision refs, and audit exports.
