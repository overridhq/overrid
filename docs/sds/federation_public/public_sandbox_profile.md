SDS #56

# Public Sandbox Profile SDS

## Purpose

Prevent public nodes from receiving secrets, private data, regulated data, or backbone workloads.

Public Sandbox Profile defines the hardened runtime profile for unknown or semi-trusted public providers. It translates public low-sensitivity workload rules into enforceable restrictions for mounts, secrets, filesystem access, network egress, runtime duration, memory, output validation, artifact quarantine, and privacy-preserving logs. It does not onboard providers, schedule workloads, execute containers, verify trust, or adjudicate fraud; it gives Overguard, Oversched, Overrun, and Overcell a versioned sandbox contract they can enforce and audit.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [public_sandbox_profile.md](../../service_catalog/federation_public/public_sandbox_profile.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md) |

## Service Family

- Family: Federation and public capacity
- Owning layer: Public low-sensitivity runtime safety and sandbox policy profiles
- Primary data scope: sandbox profile versions, restriction sets, workload-class bindings, secret and mount denials, egress rules, runtime caps, output validation rules, artifact quarantine refs, log redaction profiles, and enforcement evidence
- First build phase from service plan: [Phase 11: Limited Public Low-Sensitivity Pool](../../build_plan/phase_11_limited_public_low_sensitivity_pool.md)

## Problem Statement

Public providers are adversarial until proven otherwise. Even honest public nodes can be misconfigured, disappear, leak logs, overclaim resources, or mishandle artifacts. If a private, regulated, secret-bearing, or system-service workload reaches a public node through normal scheduler behavior, the ecosystem loses its core safety boundary.

Public Sandbox Profile must make that failure path structurally hard. The scheduler and runner need a versioned profile that says exactly which workload classes are allowed, which data classes are denied, which runtime capabilities are disabled, how output is validated, and how suspicious artifacts are quarantined.

## Goals

- Define public low-sensitivity sandbox profile versions with explicit restriction sets.
- Deny secret injection, private data mounts, regulated data mounts, and system-service placement before execution.
- Specify filesystem, network, runtime, memory, process, artifact, and log restrictions for public nodes.
- Produce profile evaluation records that Overguard, Oversched, Overrun, and Overwatch can replay.
- Bind public sandbox profiles to Public Provider Onboarding eligibility and Workload Classifier output.
- Quarantine outputs and artifacts that exceed policy, fail validation, or carry unexpected sensitive markers.
- Keep logs useful for debugging without exposing private input, secret refs, or user content.

## Non-Goals

- Do not onboard public providers or publish provider capability records; Public Provider Onboarding owns that boundary.
- Do not schedule workloads, issue leases, or choose nodes; Oversched and Overlease own placement and reservation.
- Do not run workloads or enforce local sandbox primitives directly; Overrun and Overcell enforce the selected profile on the node.
- Do not decide trust, reputation, fraud, payout holds, or disputes.
- Do not define public purpose tags or public-interest pool eligibility.
- Do not create exceptions that allow private, regulated, secret-bearing, or system-service workloads on unknown public nodes.
- Do not add pricing, financial projections, customer counts, blockchain mechanics, NFT mechanics, or per-transaction fee economics.

## Primary Actors And Clients

- Public Provider Onboarding attaching sandbox requirements to public-provider eligibility.
- Workload Classifier publishing workload class and data-class facts.
- Overguard evaluating whether a workload can use a public sandbox profile.
- Oversched filtering candidate public nodes by sandbox compatibility.
- Overrun and Overcell enforcing runtime restrictions on assigned public nodes.
- Overstore and Overvault providing object refs and secret refs that must be denied or redacted as required.
- Fraud Control Service, Challenge Task Service, Overwatch, and Overclaim consuming sandbox evidence.
- SDK, CLI, admin UI, and provider-facing surfaces showing clear denial reasons.

## Dependencies

- [Public Provider Onboarding](public_provider_onboarding.md) for provider/node eligibility and public sandbox compatibility refs.
- [Workload Classifier](../trust_policy_verification/workload_classifier.md) for workload class, data class, secret-use, and system-service facts.
- [Overguard](../trust_policy_verification/overguard.md) for policy decisions and profile admission.
- [Oversched](../execution_scheduling/oversched.md) and [Overlease](../execution_scheduling/overlease.md) for candidate filtering and lease creation.
- [Overrun](../execution_scheduling/overrun.md), [Overcell](../execution_scheduling/overcell.md), and [Overpack](../execution_scheduling/overpack.md) for manifest interpretation and node-side enforcement.
- [Overstore](../data_storage_namespace/overstore.md) and [Overvault](../data_storage_namespace/overvault.md) for object, artifact, and secret reference checks.
- [Overwatch](../control_plane/overwatch.md), [Fraud Control Service](fraud_control_service.md), and [Challenge Task Service](../trust_policy_verification/challenge_task_service.md) for evidence, abuse signals, and challenge escalation.

## Owned Responsibilities

Public Sandbox Profile owns:

- Sandbox profile version records and compatibility windows.
- Restriction sets for filesystem, mounts, secrets, environment, network, process, runtime, memory, CPU/GPU, storage, and artifact handling.
- Workload-class and data-class binding rules for public low-sensitivity jobs.
- Denial rules for private, regulated, secret-bearing, and system-service workloads.
- Output validation and artifact quarantine rule sets.
- Privacy-preserving log redaction profile definitions.
- Profile evaluation records and stable reason codes.
- Profile deprecation, supersession, emergency disablement, and rollout records.

## Data Model

- `sandbox_profile_version`: profile id, version, status, compatibility window, workload class allowlist, data class allowlist, profile hash, signer refs, rollout refs, and deprecation refs.
- `restriction_set`: filesystem mode, mount allowlist, secret policy, environment variable policy, process limits, runtime caps, memory caps, CPU/GPU caps, storage caps, network egress rules, syscall/container profile refs, and cleanup requirements.
- `public_workload_binding`: workload class, data class, provider eligibility refs, public provider tier refs, sandbox profile ref, Overguard policy refs, and scheduler filter facts.
- `secret_mount_denial`: rejected secret refs, private data refs, regulated data refs, system-service refs, reason code, and caller-visible explanation.
- `output_validation_rule_set`: expected output schema, allowed artifact types, size caps, checksum requirements, content marker checks, malware/safety scan refs where applicable, and quarantine triggers.
- `artifact_quarantine_record`: artifact ref, workload ref, provider ref, validation failure, quarantine location ref, retention policy, review status, release/deny decision refs, and audit refs.
- `log_redaction_profile`: allowed log fields, redaction rules, sampling rules, trace-only fields, provider-visible fields, and user-visible fields.
- `sandbox_evaluation`: request facts, classifier refs, provider refs, profile refs, matched rules, denied rules, policy decision refs, enforcement handoff refs, and replay bundle refs.

Sandbox profile versions are immutable after activation. Emergency disables and corrections create new records or status transitions, never silent edits to past decisions.

## API Surface

- `POST /public-sandbox/profiles`: creates a draft sandbox profile version.
- `POST /public-sandbox/profiles/{profile_id}/activate`: activates a profile after policy and validation checks.
- `POST /public-sandbox/profiles/{profile_id}/deprecate`: marks a profile deprecated or emergency-disabled with reason and replacement refs.
- `GET /public-sandbox/profiles/{profile_id}`: returns profile metadata, compatibility, and redacted restriction summary.
- `POST /public-sandbox/evaluate`: evaluates workload facts, provider refs, and requested resources against public sandbox rules.
- `POST /public-sandbox/denials/secret-mount`: records a denied secret, private data, regulated data, or system-service mount attempt.
- `POST /public-sandbox/output-validation`: records output validation results and quarantine decisions.
- `GET /public-sandbox/quarantine/{quarantine_id}`: returns quarantine status and permitted evidence refs.
- `GET /public-sandbox/replay/{evaluation_id}`: reconstructs a profile evaluation from stored facts and policy refs.

Mutating APIs require actor or service identity, tenant/system scope, idempotency key, trace id, policy refs, and Overwatch audit refs. Stable reason codes include `public_profile_missing`, `workload_class_not_public`, `data_class_disallowed`, `secret_ref_denied`, `private_mount_denied`, `system_service_denied`, `egress_denied`, `runtime_cap_exceeded`, `memory_cap_exceeded`, `artifact_quarantined`, and `profile_deprecated`.

## Event Surface

- `public_sandbox_profile.profile_created`: profile draft created.
- `public_sandbox_profile.profile_activated`: profile version activated.
- `public_sandbox_profile.profile_deprecated`: profile deprecated, disabled, or superseded.
- `public_sandbox_profile.evaluation_requested`: workload/provider/profile evaluation requested.
- `public_sandbox_profile.evaluation_allowed`: profile evaluation allowed public low-sensitivity execution.
- `public_sandbox_profile.evaluation_denied`: evaluation denied with stable reason codes.
- `public_sandbox_profile.secret_or_mount_denied`: secret, private mount, regulated mount, or system-service mount rejected.
- `public_sandbox_profile.output_validated`: output validation completed.
- `public_sandbox_profile.artifact_quarantined`: artifact quarantined for review.
- `public_sandbox_profile.quarantine_resolved`: quarantined artifact released, deleted, retained, or escalated.

Events carry workload refs, provider refs, node refs, profile refs, policy refs, classifier refs, enforcement refs, and redacted evidence refs. They must not include raw private content, secret material, or sensitive artifact content.

## Core Workflow

1. Define or update a public sandbox profile version with explicit restrictions and validation rules.
2. Validate the profile against Phase 11 public low-sensitivity constraints and activate it through Overguard policy.
3. Public Provider Onboarding binds compatible public providers to an active sandbox profile.
4. Workload Classifier publishes workload/data-class facts for a submitted workload.
5. Overguard calls `evaluate` before queueing or placement; private, regulated, secret-bearing, or system-service facts hard-deny the request.
6. Oversched filters candidate public nodes to those with compatible profile refs and required caps.
7. Overrun receives the selected profile and enforces mounts, secrets, filesystem, network, runtime, memory, process, and artifact restrictions.
8. Output validation records pass/quarantine decisions and emits evidence to Overwatch, Fraud Control, Overclaim, or provider correction flows where needed.

## State Machine

Profile lifecycle:

1. `draft`
2. `validated`
3. `active`
4. `deprecated`
5. `emergency_disabled`
6. `superseded`

Evaluation lifecycle:

1. `requested`
2. `facts_loaded`
3. `policy_checked`
4. `allowed`
5. `denied`
6. `enforcement_handoff_recorded`
7. `completed`

Artifact quarantine lifecycle:

1. `flagged`
2. `quarantined`
3. `review_pending`
4. `released`
5. `deleted`
6. `retained_for_dispute`
7. `escalated`

## Policy And Security

- Deny by default when workload classification, provider eligibility, profile version, or policy refs are missing or stale.
- Public nodes may receive only public low-sensitivity workloads with no secrets, no private tenant data, no regulated data, and no system-service responsibilities.
- Secret refs from Overvault must be rejected before queue or lease creation unless the profile explicitly supports a non-secret public token class; the first build should not include such exceptions.
- Filesystem access must be minimal, ephemeral, and tied to declared input/output refs.
- Network egress is deny-by-default and allowlisted by destination class, not arbitrary hostnames supplied by the workload.
- Logs must redact user content, secret-looking values, private refs, provider-sensitive internals, and anti-fraud internals.
- Quarantine review must preserve evidence while preventing the provider or requester from retrieving unsafe artifacts.

## Metering And Accounting

- Emit usage-relevant events for sandbox evaluation, output validation, quarantine storage, and review work.
- Link sandbox usage to workload refs, tenant refs, provider refs, public workload class, profile version, and artifact refs.
- Public sandbox enforcement does not create prices, payouts, or ledger entries directly; Overmeter, Seal Ledger, Overbill, and Provider Payout Service consume usage and hold refs downstream.
- Quarantine storage and review work must be visible for accounting and dispute evidence without exposing artifact content.
- Do not encode financial projections, public-provider fee schedules, or per-transaction economics.

## Observability And Operations

- Expose active profile versions, deprecated profiles, compatibility windows, deny counts by reason code, public provider compatibility, quarantine backlog, output validation failures, and stale profile use.
- Alert on any attempted secret mount, private data mount, regulated data mount, or system-service assignment to a public provider.
- Provide replay tooling for profile evaluations and output validation decisions.
- Support emergency profile disablement with signed operator action and Overwatch evidence.
- Provide provider-facing denial explanations that are actionable without exposing private policy internals.

## Failure Modes And Recovery

- Missing profile: deny placement and require a compatible active profile.
- Stale profile: deny or force re-evaluation under the current profile.
- Classifier unavailable: deny public placement rather than guessing workload sensitivity.
- Provider eligibility unavailable: deny public placement and request onboarding refresh.
- Overrun cannot enforce a required restriction: fail before execution and emit enforcement failure evidence.
- Output validation fails: quarantine artifact, preserve refs, and block normal delivery.
- Quarantine review times out: keep artifact isolated and escalate to Overclaim or operator review.
- Emergency restriction update: disable affected profile and require new evaluations for in-flight public jobs where possible.

## Validation Plan

- Private, regulated, secret-bearing, and system-service workloads are denied before queue, lease, or execution.
- Workloads with secret refs cannot receive public sandbox placement.
- Public low-sensitivity jobs receive the expected filesystem, runtime, memory, and egress caps.
- Overrun rejects assignments when it cannot enforce a required profile restriction.
- Output validation can pass expected public outputs and quarantine unexpected artifacts.
- Logs are redacted according to the selected log redaction profile.
- Profile deprecation prevents new placement while preserving replay of historical decisions.
- Evaluation replay reconstructs matched rules, facts, policy refs, and enforcement handoff refs.

## Build Breakdown

1. Define sandbox profile version, restriction set, binding, evaluation, output validation, quarantine, and log redaction schemas.
2. Implement profile creation, activation, deprecation, and redacted read APIs.
3. Implement `evaluate` with deny-by-default handling for missing classification, provider, profile, or policy refs.
4. Integrate with Workload Classifier, Overguard, Public Provider Onboarding, Oversched, and Overrun.
5. Add secret/mount denial evidence and public-provider denial explanations.
6. Add output validation and artifact quarantine workflows.
7. Add operational dashboards, replay tooling, emergency disablement, and validation tests.

## Handoff And Downstream Use

Public Sandbox Profile hands active profile refs, evaluation refs, denial reason codes, enforcement refs, and quarantine refs to Overguard, Oversched, Overlease, Overrun, Overcell, Overwatch, Public Provider Onboarding, Fraud Control Service, Challenge Task Service, Provider Payout Service, SDK, CLI, admin UI, and provider-facing status surfaces.

Downstream services must consume profile and evaluation refs rather than recreating sandbox rules locally.

## Open Design Questions

Resolved decisions:

- Public low-sensitivity egress is offline by default. The first Phase 11 profiles should support only three egress classes: `offline_public`, `overrid_control_artifact_only`, and tightly scoped `declared_public_fetch` for manifest-declared public endpoints or Overstore/Overmesh artifact transfer refs approved by Workload Classifier and Overguard. Challenge tasks, duplicate-execution probes, benchmark probes, deterministic transforms, rendering/transcoding over already-public inputs, public artifact verification, and any job with incomplete egress facts run fully offline except for required Overrid control-plane/reporting traffic. Open internet crawling, arbitrary hostnames, webhooks, private-network access, credentialed third-party APIs, peer discovery, and provider-supplied egress rules are not in the first public sandbox profile; adding them later requires a new profile version, replay fixtures, Overguard policy activation, and no secrets/private/regulated/system-service facts.
- Required output validators before user delivery are manifest-output conformance, declared artifact count and type checks, size caps, BLAKE3/content-hash verification, schema or media-shape validation where the manifest declares structure, unexpected-executable/archive checks, sensitive marker scans for secret-looking values, private refs, regulated markers, system-service refs, and log redaction checks. Artifact-type validators may be implemented as native Rust validators or controlled internal adapters, but the Public Sandbox Profile boundary remains the profile rule and quarantine decision, not an external SaaS scanner. If a required validator is unavailable, inconclusive, mismatched, or detects undeclared or sensitive output, the artifact stays quarantined with Overwatch evidence and cannot be returned through normal user delivery until a replacement validation or Overclaim/operator decision releases it.
- Provider-facing denial detail may show the affected provider or node, public workload class, profile id/version/status, safe reason-code category, severity and confidence band, policy/evaluator version, acknowledgement or sandbox freshness, redacted evidence refs, remediation steps, recheck options, and Overclaim/correction refs. It must not expose raw private tenant evidence, secret refs, data refs, regulated markers, other-provider identities, exact node/IP/device/fingerprint/payout identifiers, challenge payloads or randomization, fraud thresholds, model weights, topology, anti-Sybil or fraud heuristics, operator notes, incident-response details, or central-AI private context. Operator and stewardship views can dereference deeper evidence only through Overwatch redaction profiles and signed access decisions.
- Emergency disablement stops new evaluations and placement for the affected profile immediately. Pending, queued, leased-but-not-started, and preflight assignments must be cancelled, rejected, or requeued for a fresh evaluation against a replacement active profile. Already running attempts receive a signed cancellation when the disabled profile affects isolation, egress, output validation, secret/mount denial, or any other safety-critical restriction; their partial outputs, logs, and usage refs are quarantined and withheld from normal delivery and payout finality until revalidated. If the disablement is explicitly non-safety, such as planned deprecation metadata, policy may allow an already-running fully offline attempt to finish, but outputs still require validation under the replacement or superseding profile before release. No in-flight job silently migrates profiles without a new evaluation, enforcement handoff, and Overwatch evidence.
- Overguard prechecks identity, tenant, classification, data/secret/regulated/system-service denial facts, active profile status, provider eligibility, egress class, policy refs, and public-placement admission before queue or lease creation. Oversched prechecks candidate compatibility, active profile refs, resource/runtime caps, node software and Overcell capability facts, sandbox compatibility, locality, and no-candidate reason codes before scoring and lease requests. Overrun and Overcell must enforce the execution controls locally: lease/profile verification, manifest and artifact hash verification, no secret/private/regulated/system-service mounts, ephemeral filesystem and mount permissions, environment sanitization, process/syscall/container profile, CPU/GPU/memory/storage/wall-clock caps, deny-by-default egress firewall, log redaction, output capture, validation handoff, quarantine, cancellation, timeout, and cleanup. Any restriction that affects actual execution must be checked both before placement and at the runner; if Overrun cannot enforce a selected restriction, it rejects before process start and emits enforcement-failure evidence.
