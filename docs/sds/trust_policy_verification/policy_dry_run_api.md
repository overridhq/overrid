SDS #35

# Policy Dry-Run API SDS

## Purpose

Let developers, native apps, AI-generated deployment flows, SDKs, CLI users, and operators preview policy outcomes before submitting real work.

Policy Dry-Run API is a side-effect-free wrapper around Overguard policy evaluation. It assembles declared inputs, resolves safe fact snapshots, calls the same evaluator used by real admission, and returns allow/deny/block previews, matched rules, reason codes, expected placement class, estimated reservation requirements, missing prerequisites, and remediation hints.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [policy_dry_run_api.md](../../service_catalog/trust_policy_verification/policy_dry_run_api.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md) |

## Service Family

- Family: Trust, policy, verification, and disputes
- Owning layer: Side-effect-free policy preview, developer ergonomics, native app permission previews, and dry-run audit records
- Primary data scope: dry-run requests, declared inputs, fact snapshots, dry-run results, matched-rule previews, reason-code responses, missing prerequisite records, estimated reservation refs, comparison records, and retention metadata
- First build phase from service plan: [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md).

## Problem Statement

Developers and native services need to know whether a workload or app action will pass policy before they submit it. Without dry runs, users discover missing manifests, denied egress, insufficient trust, missing secret grants, wrong workload class, quota issues, or budget prerequisites only after building or submitting real work.

The preview path must match real admission when inputs remain unchanged, but it must not mutate queue, workload, lease, billing, vault, grant, or scheduler state. It must be useful enough for SDK, CLI, admin UI, AI-generated deployment planning, and native app permission prompts.

## Goals

- Expose a stable dry-run request and response contract for SDK, CLI, admin UI, native apps, adapters, and AI-generated deployment flows.
- Use Overguard as the policy evaluator so dry-run and real admission decisions stay aligned.
- Return allow, deny, blocked, and review-required previews with matched rules and reason codes.
- Return expected workload class, required trust class, provider class restrictions, sandbox profile, egress decision, secret prerequisites, cache scope, and estimated reservation requirements.
- Return missing prerequisites and remediation hints that are actionable for developers.
- Persist stable dry-run ids for audit, debugging, support, and comparison against later real admission decisions.
- Redact private dependency facts while preserving enough detail for users to fix submissions.
- Support replay from stored fact snapshots and policy versions.

## Non-Goals

- Do not enqueue work, reserve resources, create leases, mount secrets, mutate vault grants, create bills, or settle ORU.
- Do not replace Overguard. The dry-run API is a consumer and presenter of Overguard decisions.
- Do not guarantee future admission if inputs, facts, quotas, budgets, trust signals, or policy versions change.
- Do not bypass identity, tenant, manifest, secret, or data-sensitivity rules for convenience.
- Do not expose private provider details, tenant-private facts, secret values, or fraud-control internals.
- Do not become a pricing or revenue-estimation tool.

## Primary Actors And Clients

- SDK and CLI users validating manifests, app actions, and deployment plans before submission.
- Admin and Developer UI showing policy previews, remediation hints, and rule explanations.
- Native apps presenting permission previews before requesting sensitive resources.
- AI Gateway Router, Personal AI Assistant, and deployment agents checking generated plans before creating real commands.
- Overgate, using dry-run ids and comparison records during later real admission.
- Overguard, evaluating dry-run policy facts and returning decision refs.
- Overregistry, Overtenant, Overvault, Oververify, Overgrant, Overmark, and ORU Account Service, supplying safe fact snapshots or precheck refs.
- Overwatch, recording dry-run audit events and replay refs.

## Dependencies

- [Overguard](overguard.md) for policy bundle evaluation, reason codes, matched rules, decisions, and replay behavior.
- [Overgate](../control_plane/overgate.md) for authenticated caller context, tenant scope, trace id, and idempotency.
- [Overregistry](../control_plane/overregistry.md) for manifests, package refs, app refs, provider classes, and capability facts.
- [Overtenant](../control_plane/overtenant.md) for tenant state, quota scopes, memberships, and suspension flags.
- [Overpack](../execution_scheduling/overpack.md) and [Package Validator](../deployment_grid/package_validator.md) for manifest and package validation summaries.
- [Overvault](../data_storage_namespace/overvault.md) for secret ref classes and access prerequisites, never raw secrets.
- [Oververify](oververify.md) for provider and node trust/eligibility signals.
- [Overgrant](../accounting/overgrant.md), [Overmark](../accounting/overmark.md), [ORU Account Service](../accounting/oru_account_service.md), and [Overbill](../accounting/overbill.md) for grant, cost-class, budget, and reservation-precheck facts.
- [Overwatch](../control_plane/overwatch.md) for dry-run events, retention, evidence refs, and replay audit.

Phase 4 can return placeholder budget-precheck details, but the response schema must already distinguish estimated reservation requirements from actual reservations.

## Owned Responsibilities

Policy Dry-Run API owns:

- Dry-run request validation, normalization, idempotency, and tenant-scoped read permissions.
- Side-effect-free fact snapshot assembly.
- Calling Overguard in dry-run mode with the same evaluator and policy bundle semantics used by real admission.
- Response shaping for SDK, CLI, admin UI, native apps, and AI-generated plan consumers.
- Missing prerequisite and remediation hint formatting.
- Dry-run record retention, replay, and comparison against later real admission.
- Redaction profiles for user, operator, provider, and app views.

It must not mutate external service state beyond creating dry-run records and Overwatch-compatible audit events.

## Data Model

The first implementation should define:

- `dry_run_request`: dry-run id, caller identity, tenant id, app/service account, workload/action type, manifest refs or inline manifest draft, declared workload class, data class, secret refs, egress requirements, target provider class, resource card, cache hints, grant/budget refs, and trace id.
- `declared_input`: normalized command, manifest summary, package refs, permissions, resource dimensions, storage refs, vault refs, egress refs, workload class, data class, and expected execution mode.
- `fact_snapshot`: source service refs, policy-relevant fact versions, quota snapshot, trust snapshot, budget precheck snapshot, secret prerequisite snapshot, provider eligibility snapshot, and collected-at timestamp.
- `dry_run_result`: dry-run id, policy decision state, matched-rule refs, reason codes, policy bundle version, evaluator version, expected placement class, required trust class, sandbox profile, egress decision, secret prerequisites, estimated reservation, missing prerequisites, remediation hints, and expiry.
- `matched_rule_preview`: rule id, rule category, decision effect, reason code, redacted explanation, and related input refs.
- `missing_prerequisite`: prerequisite type, missing ref, blocking reason, remediation action, owning service, and user-safe link or command hint.
- `estimated_reservation`: resource dimensions, tentative ORU dimensions, budget/grant refs, insufficient-resource reason, and no-reservation attestation.
- `dry_run_response_object`: SDK/CLI stable response schema, human message, machine-readable codes, and next-action hints.
- `dry_run_comparison_record`: dry-run id, later admission decision id, matching input refs, changed fact refs, changed policy refs, match/mismatch state, and mismatch reason.
- `dry_run_replay_bundle`: dry-run request, declared input, fact snapshot, Overguard decision refs, response object, and event refs.

Common envelope fields:

- `id`, `tenant_id`, `actor_id`, `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

Policy Dry-Run API exposes preview endpoints:

- `POST /policy/dry-runs`: create a dry run from manifest/action inputs and return a stable result.
- `GET /policy/dry-runs/{dry_run_id}`: read authorized dry-run input summary, result, expiry, and remediation hints.
- `GET /policy/dry-runs/{dry_run_id}/explain`: read matched rules, reason codes, policy version, and redacted fact refs.
- `POST /policy/dry-runs/{dry_run_id}/replay`: re-evaluate from stored request, fact snapshot, and policy bundle.
- `POST /policy/dry-runs/{dry_run_id}/compare`: compare a dry run with a later real admission decision.
- `POST /policy/dry-runs:batch`: evaluate a bounded batch for deployment plans, migration tooling, or AI-generated manifests.
- `GET /policy/dry-runs/reason-codes`: list dry-run-presentable reason codes and remediation metadata.

API requirements:

- All mutating endpoints require actor identity, tenant context, trace id, and idempotency key.
- Batch dry runs must be bounded to protect policy and dependency services.
- Responses must include machine-readable reason codes, not only human text.
- Dry-run records must carry expiry because facts and policy versions can change.
- Dry-run replay must be clearly labeled as replay, not current admission.

## Event Surface

- `policy_dry_run.requested`: dry-run request accepted.
- `policy_dry_run.invalid`: dry-run request rejected before evaluation.
- `policy_dry_run.fact_snapshot_created`: side-effect-free fact snapshot captured.
- `policy_dry_run.evaluated`: Overguard evaluation completed.
- `policy_dry_run.completed`: response object persisted and returned.
- `policy_dry_run.blocked`: missing facts or dependency state prevented a useful preview.
- `policy_dry_run.replayed`: stored dry run replayed.
- `policy_dry_run.compared`: dry run compared with real admission.
- `policy_dry_run.expired`: dry-run result expired.

Events must include dry-run id, tenant id, policy bundle version, reason codes, fact snapshot refs, and trace id. They must not include raw secrets, private payloads, or provider-private details.

## Core Workflow

1. Caller submits a dry-run request through Overgate, SDK, CLI, admin UI, native app, or AI-generated deployment flow.
2. Policy Dry-Run API validates request shape, tenant scope, manifest/action draft, and idempotency key.
3. The service assembles a side-effect-free fact snapshot from declared inputs and dependency refs.
4. The service calls Overguard in dry-run mode with the same evaluator used by real admission.
5. Overguard returns a decision state, matched rules, reason codes, policy version, and safe evidence refs.
6. Policy Dry-Run API shapes the result into developer-facing and machine-readable response fields.
7. The service persists dry-run id, request summary, fact snapshot, decision refs, response object, expiry, and replay bundle.
8. Caller receives allow/deny/block preview, missing prerequisites, remediation hints, and warnings about fact/policy drift.
9. If real admission happens later, Overgate or operators can compare it to the dry-run result.

## State Machine

Dry-run lifecycle:

1. `submitted`: request accepted with idempotency key.
2. `validating`: input shape, tenant scope, and manifest/action refs are being checked.
3. `invalid`: input cannot be evaluated.
4. `collecting_facts`: side-effect-free fact snapshot is being assembled.
5. `evaluating`: Overguard is evaluating the dry-run decision.
6. `completed`: response object and replay bundle are persisted.
7. `blocked`: missing prerequisites or dependency state prevents evaluation.
8. `expired`: dry-run result is no longer valid for comparison.
9. `replayed`: stored dry-run facts were replayed.
10. `compared`: dry run was compared with real admission.

Dry-run records are append-only. Re-running a dry run creates a new result version unless it is an exact idempotent retry.

## Policy And Security

- Dry-run callers must have the same visibility required to know whether the action is allowed, even though no work is submitted.
- Secret refs may be checked for existence and prerequisite grants, but secret values must never be fetched or shown.
- Private provider, fraud, trust, and quota details must be redacted from user-facing output.
- A dry run cannot be used as a capability token; real admission must evaluate current facts again unless policy explicitly accepts a still-valid dry-run comparison.
- Dry runs must not create workload records, queue items, leases, reservations, vault mounts, bills, payouts, or grant consumption.
- Missing prerequisites must identify the owning service and safe remediation path.
- Batch dry runs must enforce rate limits and size limits.
- AI-generated deployment flows must display or log machine-readable reasons rather than silently bypassing denied rules.

## Metering And Accounting

Policy Dry-Run API does not charge, reserve, bill, or settle. It records internal usage facts:

- Dry-run evaluation count, batch size, fact snapshot count, matched-rule count, replay count, comparison count, and response size.
- Estimated reservation fields that are clearly labeled as estimates and carry no accounting authority.
- Budget or grant prerequisite refs that point to owning services without consuming budget.
- Audit and storage usage for dry-run retention.

Any future paid/native-service UX around dry-run volume must still use normal Overmeter, ORU, and Overbill records and must avoid per-call external payment friction.

## Observability And Operations

- Dashboards should show dry-run volume, allow/deny/block rates, top reason codes, missing prerequisite distribution, batch usage, replay mismatch rate, and fact-collection dependency health.
- Developers need logs that connect dry-run ids to later admission decisions and Overguard decision refs.
- Operators need redaction-aware views for policy bundle rollout comparisons and dry-run/admission mismatches.
- Alerts should fire on sudden dry-run/admission mismatch spikes, dependency failures, excessive batch size attempts, and stale policy bundle references.
- Retention policy should preserve dry-run records long enough for debugging and support without storing unnecessary private data.

## Failure Modes And Recovery

- Invalid manifest or action input: return `invalid` with schema and remediation reason codes.
- Missing dependency fact: return `blocked` with owning service and safe next action.
- Dependency outage: return retryable blocked result and avoid partial hidden facts.
- Overguard mismatch between dry-run and real admission: create comparison record showing changed facts, changed policy, or evaluator drift.
- Dry-run record expired: require new dry run before relying on result.
- Batch request too large: reject with limit reason code and partial-evaluation guidance if policy allows it.
- Replay mismatch from stored facts: mark policy integrity issue and alert operators.

## Validation Plan

The service implementation plan lists these requirements:

- Dry-run and real admission decisions match when inputs are unchanged.
- Missing prerequisite messages are actionable.
- Dry-runs never mutate workload or billing state.

Additional SDS-level validation:

- Contract tests for create, read, explain, replay, compare, batch, and reason-code APIs.
- Side-effect tests proving dry runs create no queue items, leases, reservations, vault mounts, bills, payouts, or workload records.
- Policy parity tests proving dry-run and real admission use the same Overguard evaluator and match when fact/policy inputs are unchanged.
- Drift tests proving mismatches explain changed facts, changed policy version, or evaluator differences.
- Redaction tests for secret refs, provider-private facts, fraud signals, quota facts, and trust evidence.
- SDK/CLI response tests proving machine-readable reason codes and missing prerequisite hints are stable.
- Batch limit and rate-limit tests.

## Build Breakdown

1. Define dry-run request, declared input, fact snapshot, result, matched-rule preview, missing prerequisite, estimated reservation, response, comparison, and replay schemas.
2. Implement single dry-run create/read/explain APIs.
3. Wire dry-run evaluation to Overguard using the real policy evaluator in dry-run mode.
4. Add missing prerequisite and remediation formatting for SDK, CLI, and admin UI.
5. Add stable dry-run ids, retention, expiry, and Overwatch events.
6. Add replay and dry-run/admission comparison.
7. Add bounded batch dry-run support for deployment plans and AI-generated manifests.
8. Add native app permission-preview response profiles.

## Handoff And Downstream Use

Policy Dry-Run API hands dry-run ids, response objects, reason codes, matched-rule refs, remediation hints, replay bundles, and comparison records to SDK, CLI, admin UI, native apps, AI Gateway Router, Personal AI Assistant, Overgate, Overguard, Overwatch, and Overclaim.

## Open Design Questions

Resolved decisions:

- Dry-run retention is classed by sensitivity and follow-up use. The default developer record keeps the stable response object, policy refs, redacted fact refs, input digests, reason codes, remediation hints, expiry, and Overwatch audit refs for 30 days. Secret-bearing, regulated, private-data, native-app permission, and provider-sensitive dry runs keep caller-visible response records for 30 days but retain request summaries and replay bundles for only 7 days unless a comparison, support case, Overclaim dispute, incident, rollout investigation, legal hold, or compliance policy pins them. Dry-run/admission comparison records, mismatch records, and policy rollout evidence retain redacted replay bundles for 90 days by default, then archive only hashes, policy/evaluator versions, reason codes, changed-fact refs, and Overwatch evidence refs. Raw secrets, raw private payloads, provider-private details, fraud heuristics, and private quota/trust internals are never copied into dry-run storage; the dry-run record stores refs, hashes, redaction classes, and owning-service links.
- The stable public SDK and CLI contract includes `dry_run_id`, `state`, `expires_at`, `decision_preview`, stable reason codes, user-safe messages, remediation hints, missing prerequisites, owning-service refs for prerequisites, declared workload/data class, expected placement class, required trust class, sandbox profile label, egress/cache-scope outcome, estimated reservation dimensions clearly marked as non-authoritative, policy bundle version, input digest, replay/compare availability, and trace id. Admin-only or operator-only diagnostics include full matched-rule structure, dependency fact snapshot internals, raw or high-resolution quota/trust/provider/fraud facts, challenge details, evaluator debug traces, policy rollout internals, dependency health traces, other-tenant/provider evidence, and any secret, private-data, incident, payout, or compliance detail beyond the caller's normal visibility. User-facing APIs may expose redacted matched-rule summaries only through stable reason codes and safe evidence refs.
- A fresh successful dry run cannot become a capability token and cannot skip real admission. Real admission must always call Overguard against current identity, tenant, manifest, package, policy, quota, trust, grant, budget, secret, egress, cache, and provider facts before work reaches Overqueue, Oversched, Overrun, Overvault, Overgrant, or accounting paths. A still-valid dry run may shorten admission only by reusing normalized declared input, package-validation refs, input digests, idempotency context, and safe fact refs as candidate inputs when the actor, tenant, manifest/package refs, policy bundle, fact versions, and visibility scope still match. Any changed fact, expired record, higher-risk workload class, secret-bearing or regulated action, provider-class change, route/cache/egress change, or policy rollout forces a fresh fact snapshot and full evaluation. The admission result must record the dry-run id only as comparison evidence.
- Batch limits start conservative and are enforced by Overgate rate limits plus Overguard dependency health. Phase 4 synchronous SDK/CLI batches are limited to 25 items per request. Native-app permission-preview batches are limited to 10 user-visible actions per request. AI-generated deployment plans and deployment-planner flows may submit asynchronous, idempotent batches of up to 100 items, with at most three active batches per actor and tenant and a default tenant budget of 500 dry-run items per hour until operations data justifies a higher classed limit. Secret-bearing, regulated, system-service, cross-tenant, public-provider, or provider-sensitive items use the stricter native-app limit unless an operator/service policy grants a larger bounded batch. Batch responses are per-item, may be partial, and must include retryable blocked states instead of hiding dependency pressure.
- Native app permission previews must explain the user-visible effect, not the internal policy graph. Copy should say that the app "would need" a permission, data class, egress class, trust class, or resource class before submission; whether current policy would allow, deny, block, or require review; what the user or app developer can fix; and that no work has started, no secret has been mounted, no route has been opened, no resource has been reserved, and no ORU/billing action has occurred. The preview should use domain phrases such as "private data", "trusted execution", "external network access", "secret access", "public-provider capacity", and "review required" instead of exposing rule ids, provider internals, fraud heuristics, topology, quota details, or challenge evidence. Advanced diagnostics remain available only through authorized developer/admin views with redacted reason-code and evidence-ref detail.
