SDS #33

# Overguard SDS

## Purpose

Enforce admission policy for workload class, data sensitivity, tenant quota, package trust, sandboxing, compliance boundaries, egress, secret access, provider eligibility, cache scope, abuse prevention, and budget prechecks before work reaches execution.

Overguard is the policy decision engine for Overrid. It produces immutable, reason-coded decisions from versioned input facts and policy bundles. It does not schedule, execute, store secrets, score trust, or mutate accounting state.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overguard.md](../../service_catalog/trust_policy_verification/overguard.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md) |

## Service Family

- Family: Trust, policy, verification, and disputes
- Owning layer: Policy evaluation, admission decisions, reason codes, policy versioning, replay, and policy rollout
- Primary data scope: policy bundles, input fact bundles, admission contexts, policy decisions, matched rules, reason codes, provider eligibility requirements, quota/budget precheck refs, override records, and replay bundles
- First build phase from service plan: [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md).

## Problem Statement

Overrid must reject unsafe work before it enters the queue, scheduler, node runner, vault mount path, public-provider pool, or accounting flow. A workload may be invalid because of tenant quota, workload class, data sensitivity, secret use, egress requirements, package provenance, provider trust, jurisdiction, cache scope, budget state, or abuse signals.

These decisions must be explainable and replayable. Operators, developers, native apps, SDKs, CLI users, Overclaim, and central AI review need stable reason codes and matched-rule refs, not opaque allow/deny behavior.

## Goals

- Evaluate real admission requests and dry-run requests from the same policy engine.
- Define policy input facts for actor, tenant, workload, package, data class, secret refs, egress, provider class, quota, budget precheck, cache scope, region, and abuse markers.
- Produce immutable decisions with allow, deny, blocked, or review-required state.
- Return stable reason codes, matched rules, policy bundle version, evidence refs, and safe remediation hints.
- Keep workload class, data sensitivity, secret access, egress, package trust, provider eligibility, cache scope, and quota checks explicit.
- Support policy replay from stored input facts and policy versions.
- Support staged policy rollout, canary evaluation, emergency blocks, and signed overrides.
- Make denial information visible to SDK, CLI, admin UI, and Policy Dry-Run API without leaking private dependency facts.

## Non-Goals

- Do not enqueue work. Overqueue owns durable queue state.
- Do not place work. Oversched owns candidate filtering and placement decisions.
- Do not reserve resources or mutate balances. Overlease, ORU Account Service, Seal Ledger, and Overbill own those states.
- Do not inject secrets or grant vault access. Overvault owns secret access decisions and mount leases; Overguard only decides whether a workload may request that path.
- Do not create trust scores. Oververify and Reputation and Anti-Sybil Service own trust and reputation signals.
- Do not execute workloads, inspect private payloads, or bypass workload manifests.
- Do not create hidden per-transaction fees, speculative economics, blockchain, or NFT behavior.

## Primary Actors And Clients

- Overgate, submitting admission requests before queueing or mutating platform state.
- Policy Dry-Run API, submitting side-effect-free preview requests.
- Overpack and Package Validator, supplying workload manifests, permission refs, resource cards, and provenance evidence.
- Overtenant, supplying tenant quotas, role bindings, suspension state, and tenant policy overlays.
- Overregistry, supplying package, app, provider, node, workload-class, and capability facts.
- Oververify, Reputation and Anti-Sybil Service, and public-provider onboarding, supplying trust and eligibility refs.
- Overvault, supplying secret ref metadata and access prerequisites.
- Overgrant, Overmark, ORU Account Service, and Overbill, supplying grant, cost-class, budget, and reservation-precheck facts.
- Overqueue, Oversched, Overrun, Overmesh, Overcache, and native apps, consuming policy refs.
- Overwatch and Overclaim, consuming decisions, reason codes, and replay bundles.

## Dependencies

- [Overgate](../control_plane/overgate.md) for authenticated command context, actor identity, tenant scope, trace id, and idempotency.
- [Overregistry](../control_plane/overregistry.md) for versioned manifests, providers, nodes, apps, package metadata, and capability facts.
- [Overtenant](../control_plane/overtenant.md) for tenant state, quota-scope refs, role bindings, and suspension markers.
- [Overpack](../execution_scheduling/overpack.md) and [Package Validator](../deployment_grid/package_validator.md) for manifest, provenance, permissions, sandbox, and runtime-contract facts.
- [Oververify](oververify.md) and [Reputation and Anti-Sybil Service](reputation_anti_sybil_service.md) for trust, eligibility, dispute, and public-provider risk signals.
- [Overvault](../data_storage_namespace/overvault.md) for secret ref classes, grant prerequisites, and mount-lease constraints.
- [Overmesh](../execution_scheduling/overmesh.md) and [Overcache](../execution_scheduling/overcache.md) for routing, egress, cache, and trust-scope constraints.
- [Overgrant](../accounting/overgrant.md), [Overmark](../accounting/overmark.md), [ORU Account Service](../accounting/oru_account_service.md), and [Overbill](../accounting/overbill.md) for grant, resource-card, budget, and precheck refs.
- [Overwatch](../control_plane/overwatch.md) for decision events, evidence bundles, policy rollout audit, and replay refs.

Phase 4 may use deterministic budget and grant fixtures, but the decision record must preserve final fields so Phase 5 accounting integration does not change policy semantics.

## Owned Responsibilities

Overguard owns:

- Policy bundle, rule, reason-code, and rollout metadata.
- Admission input fact schema and validation.
- Deterministic policy evaluation for real admission and dry-run paths.
- Policy decision records, matched-rule refs, evidence refs, and replay bundles.
- Safe remediation hints for users and developers.
- Override request records and signed override policy.
- Emergency block rules and deny-by-default behavior for missing critical facts.
- Policy compatibility tests and canary policy comparisons.

Overguard must consume facts through service contracts. It must not read dependency storage directly or treat missing facts as permission.

## Data Model

The first implementation should define:

- `policy_bundle`: bundle id, semantic version, rule set refs, compatibility date, rollout stage, emergency block refs, owner service, signature, and activation window.
- `policy_rule`: rule id, bundle version, domain, input selectors, condition expression or compiled rule ref, decision effect, severity, reason code refs, and remediation template.
- `reason_code`: stable code, category, user-safe message, operator message, severity, remediation hint, redaction class, and deprecation state.
- `admission_context`: request id, actor id, tenant id, service account, workload id, manifest id/version, command type, target environment, trace id, and idempotency key.
- `input_fact_bundle`: actor facts, tenant facts, manifest facts, workload class, data class, secret refs, egress requirements, package provenance, provider class, quota facts, grant refs, budget precheck refs, trust refs, cache scope, region refs, and evidence refs.
- `policy_decision`: decision id, admission context ref, bundle version, state, matched rules, reason codes, evidence refs, required trust class, allowed provider classes, sandbox profile refs, secret access prerequisites, egress decision, and replay bundle ref.
- `matched_rule`: rule id, input fact refs, pass/fail/unknown state, effect, reason code, and redacted explanation.
- `quota_budget_precheck`: tenant quota state, resource dimensions, grant/budget refs, reservation required flag, insufficient-resource reason, and downstream precheck refs.
- `policy_override_request`: source decision, requester, requested effect, justification, evidence refs, expiry, approver refs, and resulting decision version.
- `policy_rollout_record`: bundle version, target scope, canary percentage or tenant set, activation state, comparison results, rollback refs, and audit refs.
- `policy_replay_bundle`: policy bundle, input fact bundle, evaluator version, decision result, matched rules, and event refs.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account, `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

Overguard exposes internal policy APIs and administrator policy-management APIs:

- `POST /policy/admission/evaluate`: evaluate an authenticated command or workload submission and return a decision.
- `POST /policy/admission/batch-evaluate`: evaluate bounded batches for scheduler or migration checks.
- `GET /policy/decisions/{decision_id}`: read authorized decision state, matched rules, reason codes, and redacted evidence refs.
- `GET /policy/decisions/{decision_id}/explain`: return user-safe and operator-safe explanations according to viewer permissions.
- `POST /policy/decisions/{decision_id}/replay`: reconstruct the decision from stored input facts and policy bundle.
- `POST /policy/bundles`: register a new policy bundle version.
- `POST /policy/bundles/{bundle_id}/rollout`: stage, canary, activate, pause, or retire a bundle.
- `GET /policy/reason-codes`: list stable reason codes for SDK, CLI, admin UI, and native apps.
- `POST /policy/overrides`: request a signed override or emergency action when policy allows it.

API requirements:

- Evaluation endpoints must accept explicit fact refs or signed fact snapshots; no hidden dependency reads may change a stored decision after the fact.
- Real admission may create a policy decision record and event, but must not enqueue or reserve work by itself.
- Dry-run callers use the same evaluator through Policy Dry-Run API and must be marked side-effect-free.
- Reads must redact dependency-private facts while keeping reason codes stable.
- Replays must use stored policy bundle and input fact refs, not current live facts.

## Event Surface

- `overguard.policy_bundle_registered`: new policy bundle available.
- `overguard.policy_rollout_changed`: rollout state changed.
- `overguard.admission_evaluated`: admission decision created.
- `overguard.admission_allowed`: workload or command passed policy.
- `overguard.admission_denied`: workload or command denied with reason codes.
- `overguard.admission_blocked`: missing facts, dependency state, or review requirement blocks decision.
- `overguard.override_requested`: signed override requested.
- `overguard.override_applied`: override created a new decision version.
- `overguard.replay_completed`: decision replay completed.
- `overguard.emergency_block_applied`: emergency policy block activated.

Events must include decision id, bundle version, request refs, reason codes, evidence refs, and trace id. They must not include raw private payloads or secrets.

## Core Workflow

1. Overgate receives a signed request and resolves actor, tenant, manifest, and command context.
2. Overguard receives an admission evaluation request with explicit fact refs and policy target.
3. Overguard validates the input fact bundle against the policy schema.
4. Overguard evaluates workload class, data class, package trust, tenant quota, secret access, egress, provider eligibility, cache scope, region, grant, budget precheck, and abuse rules.
5. Overguard writes an immutable decision with matched rules, reason codes, policy bundle version, evidence refs, and replay bundle.
6. Allowed real admissions continue to Overqueue or the target service with the decision ref.
7. Denied or blocked admissions return user-safe reason codes and remediation hints.
8. Overwatch records policy events.
9. Overclaim, incident response, or central AI review may replay the decision during disputes or investigations.

## State Machine

Policy bundle lifecycle:

1. `draft`: rules are being authored and tested.
2. `registered`: signed bundle exists but is inactive.
3. `staged`: bundle is available for dry-run and canary comparison.
4. `canary`: bundle evaluates selected scopes alongside the active policy.
5. `active`: bundle is authoritative for its scope.
6. `paused`: rollout is stopped without deleting bundle history.
7. `retired`: bundle no longer handles new decisions.
8. `revoked`: bundle is blocked due to safety or integrity issue.

Policy decision lifecycle:

1. `received`: evaluation request accepted.
2. `facts_validated`: input facts pass schema and source checks.
3. `evaluating`: rules are running.
4. `allowed`: request may proceed under listed conditions.
5. `denied`: request must not proceed.
6. `blocked`: missing dependency facts, stale facts, or required review prevents decision.
7. `review_required`: manual or stewardship review is required before action.
8. `overridden`: later signed override created a replacement decision.
9. `expired`: decision is no longer valid for admission.

Decision records are immutable. Overrides and corrections create new decision records linked to the original.

## Policy And Security

- Deny by default for missing identity, tenant, manifest, workload class, data class, secret refs, provider eligibility, or required budget facts.
- Workload classes must include system service, private tenant, trusted federation, public low-sensitivity, research/public-interest, and regulated/secret-bearing semantics from Phase 4.
- Secret-bearing workloads must require Overvault prerequisites and must not be eligible for public-provider placement.
- Public-provider workloads are limited to strict public low-sensitivity classes with no secrets, capped runtime, capped resource allocation, and deny-by-default egress.
- Reason codes must be stable even when wording changes.
- Policy input facts must be signed or versioned by their owner service.
- Operator overrides require explicit expiry, reason, evidence, and approver refs.
- Emergency blocks must be auditable and narrowly scoped.
- Policy explanations must be useful to developers without exposing private provider data, fraud heuristics, secrets, or tenant-private facts.

## Metering And Accounting

Overguard does not bill, settle, or reserve funds. It emits policy usage facts:

- Evaluation count, matched-rule count, deny/block count, replay count, policy bundle comparisons, and override requests.
- Quota and budget precheck refs that downstream services can use for reservation or denial.
- Grant and cost-class refs from Overgrant and Overmark when they influence admission.
- Policy decision refs that Overmeter, Overbill, Seal Ledger, and Overclaim can cite later.

Policy evaluation should not create external payment calls or per-operation fee friction. Accounting integration remains through ORU and Seal Ledger refs after Phase 5.

## Observability And Operations

- Dashboards should show decision volume, allow/deny/block rates, top reason codes, policy bundle rollout state, replay success, override activity, and dependency freshness.
- Operators need per-decision explanation views, matched rules, input fact refs, policy version, and redaction-aware evidence links.
- Policy bundle changes must run compatibility tests, golden fixtures, canary comparisons, and rollback checks before activation.
- Alerts should fire on sudden denial spikes, unexpected public-provider allowance, secret-bearing workload eligibility changes, stale trust facts, and emergency block activation.
- Reason code registry changes must be versioned and published to SDK, CLI, admin UI, and native apps.

## Failure Modes And Recovery

- Missing critical fact: return blocked or denied reason code; do not allow by default.
- Dependency outage: use last valid fact only when policy explicitly allows it and records staleness; otherwise block.
- Policy bundle parse or signature failure: reject bundle and keep active version.
- Evaluation timeout: return blocked with retryable reason and evidence refs.
- Conflicting facts: use stricter rule or require review according to policy domain.
- Bad rollout: pause or roll back policy bundle and preserve decision history.
- Override abuse: require approver thresholds, expiry, and audit export.
- Replay mismatch: mark policy integrity incident and preserve evaluator version evidence.

## Validation Plan

The service implementation plan lists these requirements:

- Denials happen before execution.
- Decisions are replayable from stored facts and policy version.
- Reason codes are stable and visible to SDK/CLI/admin UI.

Additional SDS-level validation:

- Contract tests for evaluate, batch-evaluate, read, explain, replay, bundle registration, rollout, reason-code listing, and override APIs.
- Policy fixture tests for workload class, data sensitivity, tenant quota, package trust, egress, secret access, provider eligibility, cache scope, budget precheck, and public-provider restrictions.
- Deny-by-default tests for missing or stale critical facts.
- Replay tests proving stored facts and policy versions reproduce decisions.
- Redaction tests for user, operator, provider, and central AI explanation views.
- Rollout tests for staged, canary, active, paused, retired, and emergency block states.
- Integration tests proving allowed decisions are required by Overqueue, Oversched, Overrun, Overvault, Overgrant, and public-provider execution paths.

## Build Breakdown

1. Define policy bundle, rule, reason-code, input fact, decision, matched-rule, rollout, override, and replay schemas.
2. Implement admission evaluation for workload class, data class, package trust, tenant quota, and egress.
3. Add secret access, provider eligibility, trust class, cache scope, region, and abuse checks.
4. Add budget and grant precheck refs without mutating accounting state.
5. Add immutable decision storage, reason-code registry, and Overwatch events.
6. Add replay endpoint and golden-policy tests.
7. Add policy bundle rollout, canary comparison, emergency block, and override workflows.
8. Wire Policy Dry-Run API, SDK, CLI, admin UI, Overqueue, Oversched, Overrun, Overvault, and Overclaim to consume decisions.

## Handoff And Downstream Use

Overguard hands decision refs to Overgate, Overqueue, Oversched, Overrun, Overvault, Overgrant, Overclaim, Policy Dry-Run API, SDK, CLI, admin UI, native apps, and public-provider execution. Downstream services must treat the decision ref as a required input, not as a best-effort log.

## Open Design Questions

Resolved decisions:

- Policy bundles use canonical JSON/JSON Schema policy data plus a bounded Overguard-owned predicate expression format, evaluated by the Rust policy engine. Rules are not arbitrary scripts: they may only use typed input selectors, boolean composition, equality, set membership, numeric thresholds, freshness windows, explicit effect precedence, stable reason-code refs, and signed evidence refs. Internal evaluation contracts may use Protobuf for compact service traffic, but the authored policy bundle remains canonical, hashable, signed, fixture-tested, and replayable. Any future richer representation must compile to the same deterministic intermediate form, preserve golden replay fixtures, and avoid hidden dependency reads, host callbacks, wall-clock access outside the input fact bundle, or non-deterministic ordering.
- Decision TTLs are classed and bounded by the earliest of the policy TTL, input fact expiry, policy bundle rollback or emergency block, manifest/package change, trust/quota/budget/secret change, and lease or route validity. Maximum allow-decision TTLs are: `regulated_or_secret_bearing` two minutes and one admission attempt only; `system_service` five minutes with signed release or operator/service authority refs; `public_low_sensitivity` five minutes because public-provider eligibility is volatile; `trusted_federation` ten minutes; `private_tenant` fifteen minutes; and `research_public_interest` fifteen minutes on trusted/private capacity or five minutes when public-provider capacity, grant-risk, or duplicate-execution policy is involved. A workload that starts before expiry continues under Overlease/Overrun lease validity and the recorded decision ref, but retries, reschedules, secret mounts, provider-class changes, or queue reactivation after expiry require re-evaluation. Dry-run decisions use the same or shorter expiry and are not capability tokens unless a policy explicitly accepts a still-valid comparison.
- Automated service identity may create only bounded, evidence-backed decisions that do not widen access: idempotent replays, dependency-freshness refreshes, deny or block decisions, narrow emergency blocks, rollback to the previous active policy bundle, stale-decision invalidation, challenge-required or review-required outcomes, and downstream hold or quarantine recommendations. Human stewardship, or a stricter signed multi-approver policy where configured, is required for any allow-over-deny or allow-over-block action, TTL extension beyond class maximums, secret-bearing or regulated workload exception, system-service deployment override, public-provider eligibility expansion, egress/data/cache-scope widening, use of stale trust or budget facts beyond policy, release of an emergency block, waiver of challenge or verification prerequisites, cross-tenant visibility exception, or override that can affect settlement, payout, compliance, or finality.
- Policy canaries run inside the Overguard and Overwatch operator boundary against stored input fact bundle refs, redacted fact summaries, and policy bundle versions. Canary records store old/new decision states, matched rule ids, stable reason codes, visibility class, diff category, hashes of sensitive fact refs, and aggregate impact counts; they do not store raw private payloads, secret values, fraud heuristics, tenant-private facts, exact provider topology, or other-provider evidence in caller-visible output. Non-operator callers may see only user-safe outcomes such as `would_still_allow`, `would_deny`, `would_require_review`, changed remediable reason-code categories, and remediation hints that match their normal visibility. Operator and stewardship views can dereference detailed diffs through Overwatch redaction profiles, access-decision refs, and audit exports.
- Reason codes are user-remediable when the actor or developer can safely fix the submission without learning protected system details: schema or manifest errors, missing package validation, unsupported runtime, undeclared workload or data class, missing secret grant, denied egress class, cache-scope mismatch, quota or budget prerequisite, expired dry-run/admission, missing Overgrant or Overmark ref, coarse insufficient trust class, region or jurisdiction mismatch when selectable, idempotency conflict, and dependency-owned missing prerequisite. Operator-only reason codes cover fraud and anti-Sybil clusters, challenge randomization or comparator internals, private provider capacity/topology, other-tenant evidence, raw secret/private-data risk details, compliance or legal hold detail, incident response markers, emergency block internals, replay or policy-integrity mismatch, abuse throttling heuristics, and signed override or revocation rationale. Operator-only denials must still expose a stable user-safe wrapper code, trace id, appeal or support path where allowed, and redacted evidence refs instead of opaque failure text.
