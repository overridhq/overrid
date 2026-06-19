SDS #37

# Workload Classifier SDS

## Purpose

Normalize workload sensitivity, data class, secret-bearing status, public-provider eligibility, system-service status, and allowed execution environments before policy evaluation and scheduling.

Workload Classifier is a deterministic classification and reason-code service. It turns workload manifests, package metadata, data declarations, secret refs, egress declarations, tenant/app context, and policy class definitions into versioned classification decisions consumed by Overguard, Policy Dry-Run API, Oversched, Overrun, public-pool controls, and compliance boundaries.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [workload_classifier.md](../../service_catalog/trust_policy_verification/workload_classifier.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md) |

## Service Family

- Family: Trust, policy, verification, and disputes
- Owning layer: Workload and data sensitivity classification, classification reason codes, policy input facts, and classification replay
- Primary data scope: workload class definitions, data class definitions, classification input snapshots, classification decisions, downgrade/deny reason codes, policy input facts, manifest validation refs, and replay bundles
- First build phase from service plan: [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md).

## Problem Statement

Overrid cannot safely schedule workloads unless it knows what kind of workload is being submitted. A workload that declares itself public low-sensitivity may still reference tenant-private data, secrets, regulated data, privileged system-service routes, or egress that makes public placement unsafe.

Classification must happen before Overguard admission, policy dry-run, scheduler placement, runner preflight, public-provider eligibility, cache scope selection, and compliance reporting. The classification must be explainable and replayable from stored facts so denials, downgrades, and disputes can be reviewed.

## Goals

- Define canonical workload classes: system service, private tenant, trusted federation, public low-sensitivity, research/public-interest, regulated, and secret-bearing.
- Define data classes used by storage, cache, backup, policy, and execution: public, public low-sensitivity, tenant private, user private, organization private, secret-bearing, regulated, system-service, and grant-funded public-interest.
- Validate declared workload class against manifest facts, data refs, secret refs, package permissions, egress rules, storage refs, app/native-service type, and tenant policy.
- Produce deterministic classification decisions with matched rules, reason codes, confidence, policy/class version, and safe remediation hints.
- Downgrade or deny unsafe declared classes before execution.
- Expose classification facts to Overguard, Policy Dry-Run API, Oversched, Overrun, Overcache, Overmesh, Overvault, SDK, CLI, admin UI, and compliance services.
- Make classification replayable from stored input snapshots and class definition versions.

## Non-Goals

- Do not replace Overguard. Overguard makes admission decisions from classification and other policy facts.
- Do not schedule or execute workloads. Oversched and Overrun own placement and execution.
- Do not inspect raw private data payloads to classify work unless an authorized service provides a safe metadata ref.
- Do not grant secret access. Overvault owns secret grants and mount leases.
- Do not mutate package, registry, tenant, storage, or billing state.
- Do not allow user-provided labels to override manifest evidence or policy class definitions.
- Do not create pricing or financial projections.

## Primary Actors And Clients

- Overgate, submitting classification requests before admission.
- Overpack and Package Validator, supplying manifest, package, runtime contract, permission, and provenance refs.
- Overregistry, supplying package records, app/native-service records, provider class facts, and schema refs.
- Overtenant, supplying tenant policy overlays, quota-scope, suspension, and compliance flags.
- Overvault, supplying secret-ref metadata and secret-bearing prerequisites without exposing secret values.
- Overbase, Overstore, and Overcache, supplying data/storage/cache class refs where available.
- Overguard and Policy Dry-Run API, consuming classification decisions as policy input facts.
- Oversched, Overrun, Overmesh, and Overcache, consuming placement, sandbox, route, and cache-scope restrictions.
- Compliance Boundary Service, Overclaim, admin UI, CLI, SDK, and central AI review, consuming explanations and replay refs.

## Dependencies

- [Overpack](../execution_scheduling/overpack.md) for workload manifest, application-intent manifest, resource card, runtime contract, permission refs, egress declarations, and storage refs.
- [Package Validator](../deployment_grid/package_validator.md) for validation reports and package trust facts.
- [Overregistry](../control_plane/overregistry.md) for versioned package, app, native-service, provider, schema, and class definition refs.
- [Overtenant](../control_plane/overtenant.md) for tenant state, policy overlays, data handling restrictions, and compliance flags.
- [Overvault](../data_storage_namespace/overvault.md) for secret ref classes, access prerequisites, and secret-bearing metadata.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), and [Overcache](../execution_scheduling/overcache.md) for data class, object class, and cache trust-scope refs.
- [Overguard](overguard.md) for policy bundle refs and reason-code registry consumption.
- [Overwatch](../control_plane/overwatch.md) for classification events, evidence bundles, and replay refs.

## Owned Responsibilities

Workload Classifier owns:

- Workload class and data class definition versions.
- Classification input schema and manifest/data/secret/egress evidence normalization.
- Deterministic classification decisions.
- Downgrade, deny, unknown, and review-required reason codes.
- Policy input facts exported to Overguard and Policy Dry-Run API.
- Replay bundles for classification decisions.
- Redacted classification explanations for developers and operators.

The classifier must treat the strictest applicable class as authoritative when evidence conflicts.

## Data Model

The first implementation should define:

- `workload_class_definition`: class id, name, version, allowed data classes, allowed provider classes, secret policy, egress policy, sandbox requirements, cache scope, and placement restrictions.
- `data_class_definition`: data class id, version, sensitivity, storage requirements, backup rules, cache rules, allowed workloads, allowed egress, and retention hints.
- `classification_request`: workload id, tenant id, app/service account, manifest id/version, declared workload class, declared data classes, secret refs, egress refs, storage refs, package validation refs, and trace id.
- `classification_input_snapshot`: request refs, manifest summary, package permissions, data refs, secret metadata refs, tenant policy refs, native-service/system-service refs, and collected-at timestamp.
- `classification_rule_match`: rule id, class definition version, input refs, pass/fail/unknown state, reason code, severity, and remediation hint.
- `classification_decision`: decision id, request id, final workload class, data class set, secret-bearing flag, regulated flag, system-service flag, public-provider allowed flag, required trust class, sandbox profile, cache scope, egress class, state, matched rules, and replay bundle ref.
- `classification_reason_code`: stable code, category, user-safe message, operator message, remediation hint, redaction class, and deprecation state.
- `classification_override_request`: source decision, requested class, justification, evidence refs, approver refs, expiry, and replacement decision ref.
- `policy_input_fact`: decision id, exported fact key, exported fact value, class version, reason code refs, and consumer service.
- `classification_replay_bundle`: request, input snapshot, class definitions, rule matches, decision, evaluator version, and Overwatch event refs.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account, `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

The API is internal, developer-facing through SDK/CLI, and operator-facing:

- `POST /workload-classifications`: classify a workload manifest or app action draft.
- `POST /workload-classifications:batch`: classify bounded batches for deployment plans or migration checks.
- `GET /workload-classifications/{decision_id}`: read classification state, final class, reason codes, and exported facts.
- `GET /workload-classifications/{decision_id}/explain`: read matched rules, redacted input refs, remediation hints, and replay refs.
- `POST /workload-classifications/{decision_id}/replay`: reconstruct classification from stored input snapshot and class definitions.
- `GET /workload-classes`: list current class definitions for SDK, CLI, admin UI, and documentation generation.
- `GET /data-classes`: list current data class definitions and allowed workload relationships.
- `POST /workload-classifications/{decision_id}/overrides`: request a signed classification override where policy allows it.

API requirements:

- Mutating endpoints require actor identity, tenant context, trace id, and idempotency key.
- Classification reads must redact secret refs, private data refs, and provider-private details.
- Batch classification must be bounded and deterministic.
- Override APIs must create replacement decisions; they must not edit prior decisions.

## Event Surface

- `workload_classifier.requested`: classification request accepted.
- `workload_classifier.invalid`: request rejected before classification.
- `workload_classifier.decision_created`: classification decision created.
- `workload_classifier.downgraded`: declared class was downgraded to stricter class.
- `workload_classifier.denied`: classification cannot produce an execution-safe class.
- `workload_classifier.review_required`: evidence requires manual/stewardship review.
- `workload_classifier.override_requested`: override requested.
- `workload_classifier.override_applied`: replacement decision created from approved override.
- `workload_classifier.replay_completed`: replay completed for audit.

Events must include decision id, class version, request refs, reason codes, evidence refs, and trace id.

## Core Workflow

1. Overgate, SDK, CLI, Policy Dry-Run API, or deployment tooling submits a manifest/action for classification.
2. Workload Classifier validates request shape and loads class definitions.
3. The classifier builds an input snapshot from manifest, package validation, data refs, secret refs, egress refs, tenant policy, and app/native-service metadata.
4. Rules evaluate declared class against evidence.
5. The strictest applicable class is selected, or the request becomes denied/unknown/review-required.
6. The service writes a classification decision, reason codes, exported policy facts, and replay bundle.
7. Overguard consumes the decision for admission.
8. Policy Dry-Run API, SDK, CLI, and admin UI present safe explanations and remediation hints.
9. Oversched, Overrun, Overmesh, Overcache, and Overvault consume placement, sandbox, route, cache, and secret-bearing restrictions through Overguard decision refs.

## State Machine

Classification decision lifecycle:

1. `submitted`: request accepted.
2. `validating`: manifest and input refs are being checked.
3. `collecting_facts`: required fact refs are being assembled.
4. `evaluating`: class rules are running.
5. `classified`: final workload/data classes are assigned.
6. `downgraded`: declared class was replaced by a stricter class.
7. `denied`: no safe execution class is available.
8. `unknown`: required facts are missing or stale.
9. `review_required`: manual or compliance review is required.
10. `overridden`: signed override created a replacement decision.
11. `expired`: decision is no longer valid for admission.

Class definitions are versioned. Changing a class definition does not mutate old decisions.

## Policy And Security

- Secret-bearing workloads must never be classified as public low-sensitivity.
- Regulated workloads must require regulated class handling even if package metadata is incomplete.
- System-service workloads require trusted placement and system-service workload-class controls.
- Public low-sensitivity classification requires no secrets, no private tenant data, no regulated data, capped runtime eligibility, and compatible egress.
- Missing critical facts produce denied, unknown, or review-required state, never broad permission.
- User-declared class is only a hint; manifest and data evidence can force stricter classification.
- Overrides require signed action, evidence refs, expiry, and Overwatch audit.
- Explanations must avoid exposing raw secrets, private data contents, or protected compliance markers.

## Metering And Accounting

Workload Classifier does not bill, settle, or reserve resources. It emits usage facts:

- Classification count, batch count, class distribution, downgrade count, deny count, review-required count, override count, and replay count.
- Classification decision refs for Overguard, Overmeter, Overbill, Overclaim, and audit exports.
- Public-provider eligibility and required trust class facts that influence later reservation and accounting decisions.

Accounting services must use classification refs as evidence, not as pricing inputs.

## Observability And Operations

- Dashboards should show class distribution, downgrade reasons, denial reasons, unknown fact sources, review queues, class-definition rollout, and replay health.
- Operators need decision timelines linking manifest refs, class versions, matched rules, Overguard decisions, scheduler outcomes, and disputes.
- Alerts should fire on sudden public-low-sensitivity spikes, unexpected secret-bearing downgrades, unknown classification increases, and class-definition replay mismatches.
- Class-definition rollout must support staged activation, fixture replay, rollback, and compatibility reporting.

## Failure Modes And Recovery

- Missing manifest refs: reject or mark unknown with actionable reason.
- Missing data class refs: apply stricter default or require review based on policy.
- Conflicting manifest and storage facts: select stricter class and emit downgrade reason.
- Secret ref metadata unavailable: classify as secret-bearing or block until metadata is available.
- Class-definition parse failure: keep prior active definitions and reject new version.
- Replay mismatch: mark integrity issue and preserve evaluator version.
- Bad override: create correction decision and Overclaim/Overwatch evidence rather than mutating history.

## Validation Plan

The service implementation plan lists these requirements:

- Secret-bearing workloads cannot be classified as public low-sensitivity.
- System-service workloads require trusted placement.
- Classification decisions are visible in scheduler reasoning.

Additional SDS-level validation:

- Contract tests for classify, batch classify, read, explain, replay, class listing, data class listing, and override APIs.
- Fixture tests for system service, private tenant, trusted federation, public low-sensitivity, public-interest, regulated, and secret-bearing classes.
- Downgrade tests proving declared public workloads with secrets/private data become stricter classes or denials.
- Policy integration tests proving Overguard consumes exported classification facts.
- Scheduler integration tests proving Oversched explanations include class decision refs.
- Redaction tests for secret/data/provider/compliance refs.
- Replay tests proving stored inputs and class definitions reproduce decisions.

## Build Breakdown

1. Define workload class, data class, request, input snapshot, rule match, decision, reason code, override, policy fact, and replay schemas.
2. Implement class definition registry and listing APIs.
3. Implement manifest and package-fact classification.
4. Add data class, secret-bearing, egress, tenant policy, and system-service checks.
5. Add downgrade/deny/review reason codes and explanations.
6. Export policy input facts to Overguard and Policy Dry-Run API.
7. Add scheduler/runner/cache/vault handoff refs.
8. Add replay, class rollout, override, and dashboard support.

## Handoff And Downstream Use

Workload Classifier hands classification decision refs to Overguard, Policy Dry-Run API, Oversched, Overrun, Overmesh, Overcache, Overvault, Compliance Boundary Service, Overclaim, SDK, CLI, admin UI, and central AI governance.

## Open Design Questions

Resolved decisions:

- Incomplete data refs do not fall back to public, trusted federation, or research/public-interest eligibility. The default result is `unknown` or `review_required` with `public_provider_allowed=false`, deny-by-default egress, private/trusted placement requirements, narrow cache scope, and a stable `data_refs_incomplete` reason code. If policy requires a temporary workload-class label for downstream filtering, use the strictest evidenced class: system-service refs force `system_service`, secret refs force `secret_bearing`, compliance markers force `regulated`, and otherwise unresolved tenant/app/private storage refs force `private_tenant`. The classifier must not invent sensitivity by inspecting raw private payloads, and it must not treat missing owner-service facts as permission. Public low-sensitivity placement becomes available only after the owning data/storage/vault/compliance services provide versioned refs proving no secrets, no private data, no regulated data, no system-service responsibility, compatible egress, and compatible sandbox/cache rules.
- Normal developers, SDK/CLI callers, and native-app clients should see the final class, decision state, class/data definition versions, public-provider eligibility, required trust class, sandbox/cache/egress labels, stable reason codes, user-safe messages, missing-prerequisite owners, remediation hints, dry-run or decision ids, trace id, and redacted evidence refs. They should not see raw private data refs, raw secret refs, provider-private facts, fraud heuristics, compliance marker internals, other-tenant facts, exact rule thresholds, or operator notes. Compliance operators and authorized stewards can see matched rules, input snapshot structure, owner-service fact refs, redaction classes, policy/class rollout state, replay bundles, override history, and compliance boundary markers through role-scoped views, but raw secrets and raw private payloads still stay behind Overvault/owner-service access controls. The same stable reason code may therefore have three renderings: user-safe, developer/operator-redacted, and compliance/operator evidence view.
- Full historical replay is required before activating any class-definition or evaluator change that can broaden eligibility, narrow a denial, change public-provider placement, change secret-bearing or regulated handling, change system-service controls, alter data-class allowlists, alter egress/cache/sandbox requirements, change required trust class, change override semantics, change compliance-boundary marker interpretation, or change downgrade/deny/review reason semantics. Replay must cover all still-active decisions, dry runs, overrides, public-provider evaluations, and dispute/compliance windows affected by the changed definition, plus golden fixtures for every workload/data class. Editorial wording changes, additional translations, UI display labels, documentation-only examples, and new reason-code text that preserves the same stable code and effect do not require full historical replay, but still need schema/compatibility checks and canary comparison when exposed through SDK, CLI, admin UI, or native apps.
- Manual review is required when authentic facts conflict, are stale but recoverable, or need a signed exception: ambiguous data ownership, jurisdiction or compliance-boundary uncertainty, possible regulated/private-data markers without enough owner-service evidence, override requests, high-impact native-app permission changes, class rollout replay mismatches, broad public-provider eligibility changes, and cases where automatic denial would incorrectly lock an appealable user/provider path. Automatic denial is still mandatory for non-negotiable safety violations: unsigned or invalid manifests, missing identity/tenant/manifest facts, attempted public placement with secrets/private data/regulated data/system-service work, prohibited egress, invalid package provenance, unavailable required secret metadata, unsupported sandbox requirements, or policy-expired decisions. Review-required is a blocked state, not provisional permission; it needs signed reviewer/steward action, evidence refs, expiry, Overwatch audit refs, and replacement decisions rather than editing the original classification.
- Native apps should present class changes through Policy Dry-Run API previews before submission. The UI should compare the app's declared intent with the classifier result in user-facing terms: for example, "public execution changed to trusted private execution," "secret access required," "external network access denied," "regulated review required," or "public-provider capacity unavailable." It should show the resulting permission/data/egress/resource class, whether submission would allow, deny, block, or require review, what the user or app developer can change, and that no work has started, no secret has been mounted, no route has opened, no lease or reservation exists, and no ORU/billing action has occurred. Material broadening such as new private-data access, secret use, regulated handling, public-provider execution, external egress, system-service responsibility, or larger resource class requires explicit confirmation or manifest revision plus a fresh dry run. Native apps must hide internal rule ids, fraud/provider details, topology, exact trust thresholds, raw secret refs, raw data refs, and compliance internals unless the viewer is in an authorized developer/admin/compliance view with redacted evidence refs.
