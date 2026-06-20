SDS #31

# Challenge Task Service SDS

## Purpose

Issue controlled verification workloads that test provider and node claims instead of trusting self-reported capacity, benchmark history, uptime, or result behavior.

Challenge Task Service is the active testing layer for Overrid trust. It creates safe challenge manifests, schedules them through normal execution rails, records evidence, and proposes consequences to Oververify, payout-hold, reputation, fraud-control, and dispute systems.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [challenge_task_service.md](../../service_catalog/trust_policy_verification/challenge_task_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md), [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md) |
| Sub-build plan | [SUB BUILD PLAN #31 - Challenge Task Service](../../build_plan/sub_build_plan_031_challenge_task_service.md) |

## Service Family

- Family: Trust, policy, verification, and disputes
- Owning layer: Active provider and node challenge orchestration
- Primary data scope: challenge templates, challenge manifests, target snapshots, assignments, run refs, result records, evidence bundle refs, consequence proposals, challenge rate limits, and public-provider challenge profiles
- First build phase from service plan: [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md) for trusted nodes; [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md) for public providers.

## Problem Statement

Overrid cannot protect users, tenants, or native apps if provider trust is based only on enrollment claims and one-time benchmarks. Nodes can misreport hardware, pass early checks and later degrade, behave differently under real work, or cheat when public-provider incentives appear.

Challenge tasks must create recurring, auditable tests that are safe to run and hard to game. They must validate liveness, GPU capability, benchmark consistency, result consistency, sandbox behavior, and reliability without exposing private tenant data or turning challenges into arbitrary punishment.

## Goals

- Define safe challenge types for node liveness, GPU class, benchmark recheck, result consistency, reliability, sandbox compliance, and impossible-claim detection.
- Schedule challenge work through Overqueue, Oversched, Overlease, Overcell, and Overrun so challenge execution follows the same policy, lease, audit, and metering rails as normal work.
- Preserve synthetic or public test payloads only; challenge tasks must never inspect private workload payloads or tenant secrets.
- Record challenge target snapshots, run evidence, normalized results, anomaly markers, and replay bundles.
- Feed signed outcome refs into Oververify so trust, eligibility, and scheduler signals can change from evidence.
- Propose bounded consequences such as eligibility downgrade, throttling, payout hold request, manual review, recheck, or dispute opening.
- Rate-limit challenge frequency by target, provider, tenant, policy version, and public-provider risk level.
- Support stronger public-provider challenge policy in Phase 11 without weakening Phase 4 trusted-node behavior.

## Non-Goals

- Do not generate final trust scores. Oververify owns trust and eligibility signals.
- Do not schedule or reserve resources directly. Oversched and Overlease own placement and reservation.
- Do not execute workloads. Overcell and Overrun own node-side command handling and sandbox execution.
- Do not mutate payout or ledger state. Provider Payout Service, Seal Ledger, and Overbill own accounting effects.
- Do not use private customer workloads as challenge material.
- Do not punish providers without recorded evidence, reason codes, and appeal/dispute links.
- Do not treat public-provider challenge policy as safe for private, regulated, secret-bearing, or system-service workloads.

## Primary Actors And Clients

- Oververify, requesting challenges and consuming outcome refs.
- Overguard, evaluating whether a challenge may be issued for a target and challenge class.
- Overqueue, Oversched, Overlease, Overcell, and Overrun, executing challenge work through normal workload rails.
- Benchmark Runner and Hardware Discovery, supplying baseline evidence and expected hardware facts.
- Overwatch, storing events and evidence bundles.
- Overclaim, opening disputes or appeals when a provider challenges an outcome.
- Provider Payout Service, Overbill, and Seal Ledger, consuming hold proposals after Phase 5 integration.
- Fraud Control Service, Reputation and Anti-Sybil Service, and central AI review, consuming challenge-derived risk signals in later phases.
- Admin UI, CLI, and SDK, exposing authorized challenge status, explanations, and replay refs.

## Dependencies

- [Oververify](oververify.md) for verification records, challenge requests, trust-signal updates, and eligibility changes.
- [Overguard](overguard.md) for challenge-admission policy, target safety, workload class, and public-provider restrictions.
- [Overcell](../execution_scheduling/overcell.md) for node heartbeat, command acceptance, and node identity refs.
- [Benchmark Runner](../execution_scheduling/benchmark_runner.md) and [Hardware Discovery](../execution_scheduling/hardware_discovery.md) for baseline capacity and inventory evidence.
- [Overqueue](../control_plane/overqueue.md), [Oversched](../execution_scheduling/oversched.md), [Overlease](../execution_scheduling/overlease.md), and [Overrun](../execution_scheduling/overrun.md) for challenge execution.
- [Overwatch](../control_plane/overwatch.md) for evidence bundles, challenge events, audit refs, and replay evidence.
- [Overclaim](overclaim.md) for disputes, appeals, and challenge-window records.
- [Overmeter](../execution_scheduling/overmeter.md) for challenge usage rollups when challenge cost must be attributed.
- [Provider Payout Service](../accounting/provider_payout_service.md), [Seal Ledger](../accounting/seal_ledger.md), and [Overbill](../accounting/overbill.md) for hold and correction integration after accounting is available.

Early Phase 4 builds may stub payout and public-provider outputs, but the stubs must preserve final refs and consequence semantics.

## Owned Responsibilities

Challenge Task Service owns:

- Challenge template and manifest lifecycle.
- Challenge target selection rules supplied by Oververify and Overguard.
- Safe payload selection and challenge workload packaging.
- Assignment creation, idempotency, rate limiting, and replay refs.
- Challenge result normalization and outcome classification.
- Consequence proposal generation with explicit reason codes.
- Challenge evidence linkage to Overwatch and Oververify.
- Provider-visible explanation records with redaction.

Challenge Task Service must not read node-local files, private workload payloads, tenant secrets, or dependency storage directly. Every challenge decision must be traceable to explicit input refs and a challenge policy version.

## Data Model

The first implementation should define:

- `challenge_template`: challenge type, version, risk class, required resource class, allowed workload classes, synthetic payload refs, expected result shape, timeout, retry policy, and safety notes.
- `challenge_manifest`: Overpack-compatible workload manifest id/version, challenge template ref, payload hash, expected verifier, result normalization method, sandbox profile, egress rule, and no-private-data attestation.
- `challenge_target_snapshot`: provider id, node id, node trust class, capability refs, benchmark refs, heartbeat refs, previous challenge refs, public-provider risk level, payout-hold refs, and collected-at timestamp.
- `challenge_assignment`: assignment id, target snapshot ref, challenge template ref, manifest ref, issuing reason, issuer service, idempotency key, rate-limit bucket, policy version, and trace id.
- `challenge_run_ref`: queue item id, scheduling request id, lease id, overrun execution id, start/end timestamps, terminal state, usage rollup refs, and Overwatch event refs.
- `challenge_result`: assignment id, observed outputs, normalized metrics, expected result hash or comparator, pass/fail/inconclusive state, anomaly codes, confidence level, and verifier version.
- `consequence_proposal`: target id, result refs, severity, proposed action, hold request ref, eligibility effect, recheck window, dispute window, and approver requirements.
- `challenge_rate_limit_record`: target id, provider id, challenge type, policy version, recent assignments, cooldown, and exception refs.
- `public_provider_challenge_profile`: public onboarding tier, anti-Sybil refs, required challenge cadence, duplicate-execution policy, payout-hold trigger thresholds, and maximum public workload class.
- `challenge_replay_bundle`: assignment, target snapshot, challenge manifest, execution refs, result refs, policy version, and consequence mapping version.

Common envelope fields:

- `id`, `tenant_id` or system-service account, `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

The API is mostly internal and operator-facing:

- `POST /challenges/templates`: create or update a challenge template version.
- `POST /challenges/issue`: issue a challenge for a provider, node, verification record, or public-provider onboarding flow.
- `POST /challenges/issue:dry-run`: check whether a challenge would be allowed and which target facts would be used.
- `GET /challenges/{assignment_id}`: read challenge state, target summary, execution refs, outcome, and consequence proposal.
- `GET /challenges/{assignment_id}/explain`: return reason codes, policy version, target facts, run refs, and redacted evidence links.
- `POST /challenges/{assignment_id}/result`: accept normalized challenge results from the verifier path.
- `POST /challenges/{assignment_id}/cancel`: cancel a pending or scheduled challenge when policy, operator, or target state changes.
- `POST /challenges/{assignment_id}/replay`: reconstruct outcome and consequence proposal from stored refs in audit mode.
- `GET /challenges/targets/{target_id}/history`: read authorized challenge history and current cooldown state.

API requirements:

- Mutating APIs require service identity, trace id, idempotency key, and Overguard challenge-admission refs.
- Provider-visible reads must redact private scheduler, node, and fraud details while preserving appealable reason codes.
- Result submission must be accepted only from trusted challenge verifier paths and must include execution refs.
- Public-provider challenges must carry Phase 11 public workload eligibility refs once public supply is enabled.

## Event Surface

- `challenge.template_created`: challenge template version registered.
- `challenge.issue_requested`: challenge issuance requested with target and policy refs.
- `challenge.issue_denied`: challenge rejected by policy, cooldown, or invalid target facts.
- `challenge.assignment_created`: assignment created and ready for queueing.
- `challenge.execution_scheduled`: challenge queue, scheduler, and lease refs attached.
- `challenge.execution_completed`: execution terminal state received.
- `challenge.result_recorded`: normalized result and evidence refs stored.
- `challenge.failed`: failure recorded with anomaly and severity codes.
- `challenge.inconclusive`: result could not support a trust change.
- `challenge.consequence_proposed`: eligibility, throttle, recheck, hold, or dispute proposal created.
- `challenge.closed`: challenge completed with final handoff refs.

Events must include challenge template version, assignment id, target id, policy version, evidence refs, and trace id. They must not include private workload data, secrets, or raw provider-private details in user-facing channels.

## Core Workflow

1. Oververify, public onboarding, fraud controls, or operator review requests a challenge for a provider or node.
2. Challenge Task Service gathers target facts and checks cooldown, policy, workload class, and safety rules through Overguard.
3. The service selects a challenge template and builds a safe challenge manifest.
4. The service creates a challenge assignment and replay bundle shell.
5. The challenge is submitted to Overqueue and scheduled like ordinary low-risk work.
6. Overlease and Overrun refs are attached to the assignment.
7. The verifier collects outputs, metrics, logs, and evidence refs.
8. Challenge Task Service normalizes the result and records pass, fail, or inconclusive outcome.
9. Consequence mapping creates a bounded proposal for Oververify, payout holds, reputation, fraud controls, or Overclaim.
10. Oververify updates trust and eligibility from the outcome, while Overwatch preserves evidence for replay.

## State Machine

Challenge assignment lifecycle:

1. `requested`: challenge request received.
2. `checking_policy`: target facts, cooldown, and policy are being evaluated.
3. `denied`: policy, cooldown, or invalid target state blocks issuance.
4. `prepared`: challenge manifest and assignment were created.
5. `queued`: assignment submitted to Overqueue.
6. `scheduled`: scheduler selected a node or route.
7. `leased`: Overlease returned a lease proof.
8. `running`: Overrun is executing the challenge.
9. `result_pending`: execution completed and verifier is normalizing results.
10. `passed`: challenge matched expected behavior.
11. `failed`: challenge detected a material issue.
12. `inconclusive`: evidence is insufficient for trust change.
13. `consequence_proposed`: bounded effect proposal has been emitted.
14. `disputed`: provider or operator opened a challenge dispute.
15. `closed`: handoff refs are complete.
16. `cancelled`: challenge was stopped before terminal result.

Challenge history is append-only. Rechecks and appeals create new assignment or dispute records rather than rewriting old outcomes.

## Policy And Security

- Challenge workloads must use synthetic, public, or purpose-built payloads only.
- Challenge manifests must declare `no_private_data=true` and deny secret injection unless a test explicitly validates the absence of secret access.
- Public-provider targets are limited to public low-sensitivity challenge classes and hardened sandbox profiles.
- Challenge issuance must be rate-limited by target and provider to avoid harassment, waste, or predictable gaming.
- Randomization may select time, target, payload, or duplicate-execution path, but the randomization seed/ref must be auditable after the fact.
- Consequences must be mapped by versioned rules and must include dispute windows.
- Manual override requires signed operator action, reason code, Overwatch evidence, and new consequence version.
- Provider-visible explanations must be appealable without exposing fraud heuristics that would make challenges easy to game.

## Metering And Accounting

Challenge Task Service does not bill or settle, but it produces accounting-relevant refs:

- Challenge workload usage refs from Overmeter.
- Issuer account or system-service account responsible for challenge cost.
- Provider payout hold proposals when failures affect payable work.
- Refund or correction trigger refs for Overclaim and Overbill when a challenge proves prior work was invalid.
- Resource-class and challenge-type metrics for cost control.

Internal challenge accounting must use ORU and Seal Ledger refs after Phase 5. It must not create per-challenge external payment calls or speculative token mechanics.

## Observability And Operations

- Dashboards should show challenge issuance rate, pass/fail/inconclusive ratio, cooldown blocks, execution failures, consequence proposals, dispute rate, and public-provider challenge cadence.
- Operators need target timelines that join verification records, challenge history, benchmark evidence, disputes, and payout holds.
- Alerts should fire on impossible benchmark claims, repeated challenge failures, verifier inconsistency, public-provider anomaly clusters, and high inconclusive rates.
- Health checks should cover Overguard, Overqueue, Oversched, Overlease, Overrun, Oververify, Overwatch, and result verifier freshness.
- Challenge template rollout must support staged activation, canary target groups, rollback, and replay against historical target snapshots.

## Failure Modes And Recovery

- Missing target facts: deny issuance or mark blocked with a safe reason code.
- Stale benchmark or heartbeat evidence: issue a liveness or recheck challenge before stronger consequences.
- Scheduler cannot place challenge: keep assignment blocked and retry according to challenge policy.
- Challenge execution fails from platform error: mark inconclusive unless evidence proves target fault.
- Verifier output is malformed: quarantine result, emit verifier failure, and require re-run.
- Provider disputes result: hand off to Overclaim with evidence refs and freeze consequence escalation where policy requires.
- Challenge system outage: assignments resume from append-only state and idempotency keys.
- Public-provider abuse cluster: escalate to Fraud Control Service and require stricter challenge cadence or payout holds.

## Validation Plan

The service implementation plan lists these requirements:

- Failed challenge produces visible trust and eligibility change.
- Challenge tasks cannot access private workload data.
- Repeated challenges are rate-limited and auditable.

Additional SDS-level validation:

- Contract tests for template creation, issuance, dry-run, result submission, explain, cancel, and replay APIs.
- Safety tests proving challenge manifests cannot request tenant secrets or private workload payloads.
- Policy tests for trusted-node and public-provider challenge classes.
- Determinism tests proving the same assignment, result, and consequence mapping produce the same outcome.
- Scheduler integration tests proving challenges use normal Overqueue, Oversched, Overlease, and Overrun refs.
- Redaction tests for provider-visible explanations and operator-only evidence.
- Dispute tests proving challenged outcomes hand off to Overclaim without mutating original result records.

## Build Breakdown

1. Define challenge template, manifest, assignment, result, rate-limit, and replay schemas.
2. Implement trusted-node liveness and capability challenges.
3. Add benchmark recheck and result consistency challenges.
4. Route challenge execution through the private execution loop.
5. Record normalized results, evidence refs, and consequence proposals.
6. Integrate outcome refs into Oververify eligibility and scheduler trust signals.
7. Add Overclaim dispute handoff and payout-hold proposal refs.
8. Add public-provider challenge profiles, duplicate execution, and stricter fraud controls in Phase 11.
9. Add dashboards, replay tooling, and challenge-template rollout controls.

## Handoff And Downstream Use

Challenge outcomes feed Oververify for trust and eligibility, Oversched for placement filtering, Provider Payout Service for hold decisions, Fraud Control Service for public-provider risk, Overclaim for disputes, and central AI stewardship for ecosystem-level fraud review.

## Open Design Questions

Resolved decisions:

- Before a provider or node can receive private tenant work, it must have current Phase 4 trusted-node challenge refs for liveness, command acceptance, sandbox smoke behavior, claimed resource class, benchmark freshness or recheck, and result consistency for the workload classes it wants to run. Private, regulated, secret-bearing, and system-service eligibility also requires Oververify workload-class eligibility and an Overguard admission decision; Challenge Task Service only supplies outcome refs and must not make the trust or scheduling decision itself.
- Challenge cadence is classed and risk-driven rather than a single fixed interval. Founder or trusted private hardware is challenged at enrollment, after material hardware/software/runtime changes, when benchmark or heartbeat evidence becomes stale, after incidents or disputes, and through low-rate randomized freshness checks. Public providers in Phase 11 receive stricter onboarding, random, duplicate-execution, anomaly-triggered, and pre-payout or volume-triggered challenges, with cooldowns to prevent harassment but with shorter freshness windows and less predictable timing than founder hardware.
- Automatic suspension is reserved for high-confidence failures that create direct safety, integrity, or fraud risk: failed liveness or command-acceptance for an active target, sandbox escape or unauthorized egress, attempted secret or private-data access, falsified resource class, impossible benchmark claim, fabricated challenge evidence, repeated controlled-result inconsistency, or public-provider duplicate-execution fraud. Ambiguous, inconclusive, platform-caused, first low-severity reliability, or actively disputed failures should create probation, recheck, review, or payout-hold proposals rather than irreversible punishment.
- Provider-visible verifier detail must be enough to remediate and appeal but not enough to train against challenges. Providers may see challenge family, affected resource or workload class, pass/fail/inconclusive state, stable reason codes, policy and verifier versions, non-sensitive metric bands or deltas, remediation steps, recheck windows, dispute windows, and redacted evidence refs. Exact payload selection, randomization strategy, comparator internals, private fraud heuristics, other-provider data, tenant-private evidence, secrets, and sensitive operator notes remain operator-only or audit-only through Overwatch redaction profiles.
- Challenge cost ownership follows the issuing reason and is settled through ORU, Seal Ledger, Overbill, Provider Payout Service, and Overgrant refs, not direct external per-challenge payment calls. System-funded challenges include baseline onboarding, policy-required freshness, ecosystem integrity checks, template canaries, and system-triggered incident review. Provider-funded-after-failure applies to rechecks, reinstatement attempts, repeated no-shows, impossible claims, or proven provider-caused failure, usually as payout-hold or correction proposals. Grant-funded challenges apply to public-interest pools, research/public-interest workloads, stewardship audits, and sponsored public-provider capacity.
