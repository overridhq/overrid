SDS #49

# Package Validator SDS

## Purpose

Validate workload, application, and system-service packages before execution, deployment, or grid-resident promotion.

Package Validator produces deterministic validation reports for manifests, artifacts, runtime contracts, signatures, provenance, dependency locks, permission declarations, policy compatibility, and package safety. It does not run workloads, build packages, decide placement, approve policy by itself, or deploy applications.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [package_validator.md](../../service_catalog/deployment_grid/package_validator.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Sub-build plan | [SUB BUILD PLAN #49 - Package Validator](../../build_plan/sub_build_plan_049_package_validator.md) |
| Build phase alignment | [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md), [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md), [Phase 9: Overpack Deployment Platform](../../build_plan/phase_09_overpack_deployment_platform.md) |

## Service Family

- Family: Deployment and grid-resident backbone
- Owning layer: Package admission evidence and validation pipeline
- Primary data scope: validation requests, package manifests, artifact hash refs, signature checks, dependency locks, SBOM refs, permission analysis, runtime contract checks, policy compatibility results, warnings, errors, and validation reports
- First build phase from service plan: [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md) for workload packages; [Phase 9: Overpack Deployment Platform](../../build_plan/phase_09_overpack_deployment_platform.md) for application deployment

## Problem Statement

Overrid cannot safely execute packages from developers, AI-generated tools, native apps, or system-service maintainers without a stable validation boundary. If package checks are hidden inside the runner or deployment planner, invalid manifests, unsafe permissions, missing artifact hashes, stale signatures, broken runtime contracts, or incompatible policy assumptions can slip into execution.

Package Validator makes validation explicit and replayable. Its report lets Overrun reject unsafe work before execution, lets Deployment Planner block unsafe app deployments before provisioning, and lets Grid-Resident Service Packager prove backbone service packages have the contracts required for backup, rollback, and failover.

## Goals

- Validate manifest schema, schema version, signature, signer authority, artifact hash refs, runtime contracts, and package provenance.
- Verify dependency locks, SBOM refs, package compatibility, and declared permissions.
- Produce stable warning and error codes that SDK, CLI, admin UI, Overrun, Deployment Planner, and AI-generated package tools can display or act on.
- Provide policy-compatibility input for Overguard and Policy Dry-Run API without replacing final policy decisions.
- Support different validation profiles for Phase 3 workload packages, Phase 7 system-service packages, and Phase 9 application-intent manifests.
- Make validation deterministic from stored inputs, validator version, ruleset version, and dependency facts.
- Prevent AI-generated packages from bypassing trust, permission, policy, billing, or provenance rails.

## Non-Goals

- Do not execute workloads or run untrusted package code outside a bounded inspection sandbox.
- Do not deploy apps, provision infrastructure, allocate routes, reserve budget, or choose release strategy.
- Do not decide final policy admission; Overguard owns policy decisions.
- Do not build packages or generate manifests; Overpack and Grid-Resident Service Packager own package contracts.
- Do not store artifacts as canonical object storage; use Overstore refs.
- Do not mutate ORU balances, Seal Ledger entries, billing documents, or payout state.
- Do not add pricing, revenue, customer-count, blockchain, NFT, or per-transaction-fee assumptions.

## Primary Actors And Clients

- Overrun requesting validation before executing Phase 3 workload packages.
- Deployment Planner requesting validation report refs before provisioning an application.
- Grid-Resident Service Packager submitting system-service package versions.
- Overpack providing package envelopes, manifest schemas, artifact refs, and provenance.
- Overregistry providing accepted package records and schema refs.
- Overguard and Policy Dry-Run API consuming compatibility facts and declared permission facts.
- SDK, CLI, admin/developer UI, and AI-generated package assistants showing warnings, errors, and remediation hints.
- Overwatch recording validation events and evidence bundles.

## Dependencies

- [Overpack](../execution_scheduling/overpack.md) for package envelope, manifest sections, runtime contracts, artifact refs, and provenance conventions.
- [Overregistry](../control_plane/overregistry.md) for package records, schema versions, accepted runtime cards, signer refs, and compatibility facts.
- [Overguard](../trust_policy_verification/overguard.md) for policy compatibility checks and final admission decisions.
- [Policy Dry-Run API](../trust_policy_verification/policy_dry_run_api.md) for side-effect-free policy preview used in developer tooling and AI-generated deployment review.
- [Overrun](../execution_scheduling/overrun.md) for execution preflight and inspection sandbox constraints.
- [Overstore](../data_storage_namespace/overstore.md) for artifact refs, package bundles, SBOM refs, and validation report storage where needed.
- [Overvault](../data_storage_namespace/overvault.md) for secret-reference validation in system-service and app packages.
- [Workload Classifier](../trust_policy_verification/workload_classifier.md) for workload/data-class facts used in validation and policy compatibility.
- [Overwatch](../control_plane/overwatch.md) for validation traces, audit events, and evidence refs.

## Owned Responsibilities

Package Validator owns:

- Validation profile definitions for workload, application, system-service, native-app, and AI-generated package classes.
- Validation request and validation report schemas.
- Schema, signature, signer, artifact hash, dependency lock, SBOM, runtime contract, permission, and provenance checks.
- Permission minimization warnings and high-risk capability findings.
- Policy compatibility result objects that Overguard can use as input facts.
- Stable validation reason codes, remediation hints, and report severity semantics.
- Validation sandbox rules for static inspection and bounded dynamic checks.
- Report replay data: validator version, ruleset version, input refs, dependency facts, and hash refs.

## Data Model

- `validation_request`: request with `request_id`, `tenant_id`, `actor_id`, `package_ref`, `package_hash`, `package_type`, `validation_profile`, `ruleset_version`, `policy_context_ref`, `requested_by_service`, `trace_id`, and `state`.
- `validation_input_snapshot`: immutable refs used during validation, including manifest ref, artifact refs, registry facts, signer facts, SBOM refs, dependency locks, runtime-card refs, secret refs, and classifier facts.
- `validation_report`: result with `report_id`, `request_id`, `package_hash`, `validator_version`, `ruleset_version`, `profile`, `state`, `passed`, `warnings`, `errors`, `policy_compatibility_ref`, `permission_findings_ref`, `provenance_findings_ref`, `created_at`, and `audit_refs`.
- `validation_check_result`: one check with `check_id`, `category`, `severity`, `result`, `reason_code`, `evidence_ref`, `remediation_hint`, and `blocking`.
- `policy_compatibility_result`: side-effect-free summary of required policies, matched policy facts, missing prerequisites, incompatible permissions, data-class conflicts, and budget/route precheck dependencies.
- `permission_finding`: declared versus observed or inferred capability request, data class, network scope, storage scope, route scope, vault scope, and minimization recommendation.
- `validator_ruleset`: versioned validation rule bundle with profile, check ordering, severity mapping, compatibility window, and deprecation state.

Reports are immutable. If package refs, rulesets, registry facts, or policy context changes, callers request a new validation report.

## API Surface

- `POST /package-validations`: submits a package ref, package hash, package type, validation profile, and policy context for validation.
- `GET /package-validations/{request_id}`: returns validation request state and final report ref when available.
- `GET /validation-reports/{report_id}`: returns the immutable validation report with redacted evidence refs.
- `GET /validation-reports/{report_id}/checks`: returns per-check results, warnings, errors, and remediation hints.
- `POST /validation-reports/{report_id}/policy-preview`: asks Policy Dry-Run API or Overguard for a side-effect-free compatibility preview using report facts.
- `GET /validator-rulesets`: lists supported validation profiles, ruleset versions, deprecations, and compatibility windows.
- `POST /package-validations/{request_id}/replay`: re-runs validation with the original input refs and ruleset version for audit comparison.

Mutating endpoints require actor or service identity, tenant scope, trace id, idempotency key, package hash, and validation profile. Stable errors include `schema_invalid`, `signature_invalid`, `signer_not_authorized`, `artifact_hash_mismatch`, `dependency_lock_missing`, `sbom_missing`, `runtime_contract_invalid`, `permission_too_broad`, `policy_incompatible`, `secret_ref_invalid`, and `ruleset_deprecated`.

## Event Surface

- `package_validator.validation_requested`: validation request accepted.
- `package_validator.validation_started`: validator started processing with profile and ruleset.
- `package_validator.check_completed`: one check completed; high-volume payloads may be summarized with refs.
- `package_validator.validation_passed`: report passed all blocking checks.
- `package_validator.validation_failed`: report contains blocking errors.
- `package_validator.validation_warned`: report passed with non-blocking warnings.
- `package_validator.policy_preview_completed`: policy compatibility preview was attached.
- `package_validator.ruleset_deprecated`: ruleset can no longer be used for new validation except replay.
- `package_validator.replay_completed`: replay comparison completed for audit.

Events include request id, report id where available, package hash, package type, validation profile, ruleset version, trace id, and redacted evidence refs.

## Core Workflow

1. Accept validation request with package ref, package hash, type, profile, and policy context.
2. Fetch package envelope, manifest, artifact refs, registry facts, signer facts, dependency locks, SBOM refs, runtime cards, and secret refs.
3. Verify schema version, manifest shape, package hash, artifact hashes, signature, and signer authority.
4. Check runtime contract compatibility with Overrun/Overcell and package type.
5. Check declared permissions, data classes, route scopes, storage scopes, vault scopes, and service account needs.
6. Check dependency lock, SBOM, provenance, and known compatibility windows.
7. Produce validation report with blocking errors, warnings, and remediation hints.
8. Attach policy compatibility preview when requested or required by deployment/app profiles.
9. Store report refs and emit Overwatch-compatible evidence.

## State Machine

Validation request lifecycle:

1. `submitted`
2. `input_fetching`
3. `input_ready`
4. `checking`
5. `awaiting_policy_preview`
6. `passed`
7. `passed_with_warnings`
8. `failed`
9. `cancelled`
10. `replay_only`

Validation report lifecycle:

1. `drafting`
2. `complete`
3. `superseded_by_new_ruleset`
4. `invalidated_by_package_change`
5. `retained_for_replay`

Check result lifecycle:

1. `not_run`
2. `running`
3. `pass`
4. `warn`
5. `fail`
6. `skipped_with_reason`

Completed reports are immutable. A changed package, policy context, registry fact, or ruleset produces a new report rather than mutating an old one.

## Policy And Security

- Treat all package inputs as untrusted until schema, hash, signature, and provenance checks pass.
- Use bounded inspection sandboxes for dynamic or archive-expansion checks; never run package application logic as part of validation.
- Reject packages that reference raw secrets rather than Overvault refs.
- Require signer authority checks for system-service packages and privileged workload classes.
- Produce compatibility facts for Overguard but do not convert validation pass into policy admission.
- Deny AI-generated package bypasses by requiring the same validation profile, permission minimization, policy preview, and provenance checks as human-authored packages.
- Redact secrets, private source metadata, internal topology, and package contents from public-facing report summaries.
- Version validator rulesets so policy changes and validation behavior can be audited later.

## Metering And Accounting

- Emit validation CPU, storage, artifact-transfer, sandbox, and queue usage dimensions to Overmeter when validation consumes material resources.
- Link usage to tenant, actor, package hash, app id or service id, validation profile, and requesting service.
- Use ORU budget prechecks for expensive validation profiles before deep scanning or dynamic inspection.
- Keep validation reports separate from billing and ledger state; Overbill and ORU Account Service decide billing documents and balance projections from metered usage.
- Do not encode validation fees or financial forecasts in the validator.

## Observability And Operations

- Expose validator health, queue depth, validation latency by profile, ruleset version adoption, report pass/fail/warn counts, sandbox failure counts, and artifact-fetch failure counts.
- Provide report search by package hash, manifest ref, service id, app id, profile, ruleset, and reason code.
- Track most common blocking errors and warnings to guide SDK/CLI remediation.
- Support replay mode and diff mode between two reports or two rulesets.
- Maintain ruleset deprecation schedules and compatibility windows.
- Provide operator diagnostics for stuck validations, missing registry facts, stale schema refs, and Overstore fetch failures.

## Failure Modes And Recovery

- Package ref missing or artifact unavailable: keep request failed with `artifact_unavailable` and no side effects.
- Hash mismatch: fail validation and emit security evidence.
- Signature invalid or signer unauthorized: fail before deeper checks.
- Runtime contract incompatible: fail with expected runtime-card constraints.
- SBOM or dependency lock missing: fail or warn according to profile severity.
- Permission set too broad: fail privileged/system packages; warn or fail app packages according to policy context.
- Policy preview dependency unavailable: mark report complete but `policy_preview_missing` or block profiles that require preview.
- Sandbox timeout: fail bounded dynamic checks without running unbounded package code.
- Ruleset bug discovered: deprecate ruleset, require revalidation, and retain old reports for audit.

## Validation Plan

- Invalid manifest schema fails with stable `schema_invalid` reason code.
- Hash-mismatched artifacts fail before runtime or policy checks.
- Signature and signer authority failures block execution and deployment.
- Missing SBOM/dependency locks are handled by profile-specific severity.
- Runtime contract mismatch blocks Overrun execution in Phase 3.
- System-service package missing backup, restore, or rollback contracts fails before grid-resident promotion.
- AI-generated package with excessive permissions fails or warns according to policy profile and cannot bypass Overguard.
- Validation report is stable enough for SDK, CLI, admin UI, Overrun, Deployment Planner, and Package Validator replay.
- Replaying the same report with original refs and ruleset returns the same check results.

## Build Breakdown

1. Define validation request, report, check result, policy compatibility, permission finding, and ruleset schemas.
2. Implement Phase 3 workload package checks: schema, signature, artifact hashes, runtime contract, and basic permissions.
3. Add stable report read APIs and SDK/CLI-friendly reason codes.
4. Add Overguard/Policy Dry-Run compatibility result integration.
5. Add SBOM, dependency lock, provenance, and permission minimization checks.
6. Add system-service package profile for Phase 7 command contracts, privilege profiles, and compatibility windows.
7. Add Phase 9 application-intent manifest profile and AI-generated package diff/safety checks.
8. Add report replay, ruleset deprecation, and report diff features.

## Handoff And Downstream Use

Package Validator hands immutable validation reports to Overrun, Deployment Planner, Grid-Resident Service Packager, Release Strategy Service, SDK, CLI, admin/developer UI, and Overguard. Downstream services should treat the report as evidence, not as final policy admission or execution approval.

When validation rules change, downstream services should require a fresh report only where the package or target profile is affected by the changed ruleset.

## Open Design Questions

- Phase 3 validation must block execution unless the package has a valid Overpack workload schema, supported schema/profile version, immutable package hash, BLAKE3 artifact hashes, Overkey Ed25519 signature, signer authority ref, artifact-ref availability, narrow Overrun/Overcell runtime-card compatibility, explicit resource card, timeout/retry policy, data-sensitivity declaration, egress and secret policy shape, no raw secrets, basic permission declarations, minimum dependency/SBOM inventory, and side-effect-free Overguard or Policy Dry-Run compatibility facts for the declared workload class. Phase 7 system-service promotion adds command-contract, health/readiness, backup/restore/rollback/drain, privileged signer, node-class, and failover handoff checks. Phase 9 can wait for full application-intent validation: app route ownership, Overbase/Overstore/Overvault provisioning compatibility, budget and billing hook compatibility, multi-service dependency graphs, release health gates, canary/blue-green strategy compatibility, richer AI-generated package diffs, and deployment-wide policy previews. Signatures, hashes, runtime contracts, raw-secret rejection, and basic policy facts must not wait until Phase 9.
- Early private workloads use an Overpack-native canonical JSON/JSON Schema `dependency_inventory_v0` rather than making SPDX, CycloneDX, or any external scanner the product boundary. The minimum record includes manifest id/version, package hash, source/build ref, builder identity, runtime-card/toolchain ref such as Rust/Cargo or OCI/WASI profile, dependency lock ref or inline lock, each dependency name/ecosystem/version or revision, resolved artifact digest, BLAKE3 content hash where available, base image or module digest for container/WASI workloads, license/security metadata when known, SBOM ref when produced, and an explicit `no_external_dependencies` attestation for single-binary or fixture packages. Missing locks block Phase 3 execution for package-managed or container workloads; unknown optional metadata is allowed only as a warning under private-grid profiles.
- Local dynamic checks are limited to validator-owned, bounded inspection that cannot perform external side effects: archive expansion with path, size, count, and timeout limits; manifest and schema parsing; hash and signature verification; OCI/WASI metadata inspection without running the package entrypoint; dependency-lock and SBOM consistency checks; runtime-card compatibility probing against a throwaway no-network/no-secret sandbox; and health/readiness command shape checks using synthetic fixtures only when the command is explicitly declared safe for validation. Package application logic, migrations, network calls, real secret mounts, route changes, storage writes, billing/accounting actions, and workload execution remain outside validation and must be exercised later by Overrun, Deployment Planner, Release Strategy Service, or controlled test deployments.
- AI-generated package diffs are presented as a stable machine-readable diff plus a compact reviewer view grouped by risk, not as a raw file diff. The first screen shows blocking errors, warnings, privilege expansions, new egress, new Overvault scopes, new data classes, route/budget changes, runtime changes, dependency/provenance changes, generated-by refs, policy-preview deltas, and remediation reason codes. Reviewers can expand each manifest section to compare previous versus proposed canonical JSON with redacted secret and topology details, and every finding links to validation check ids, Overwatch evidence refs, signer/provenance refs, and Policy Dry-Run results. CLI/SDK output should use the same stable reason codes so human UI, automation, and incident review see the same facts.
- Retention is classed by report use. Normal Phase 3 private-workload validation reports, check summaries, ruleset refs, package hashes, signer refs, and redacted evidence refs are retained for at least 180 days. Reports that gate Phase 7 system-service promotion, Phase 9 production application deployment, route activation, release rollback, package revocation, public-provider use, or accounting-impacting work are retained for seven years through Overwatch-compatible evidence bundles. Any report pinned by Overclaim dispute, Incident Response, Compliance Boundary, Stewardship Reporting, PIP review, or security investigation is retained until the case closes plus the applicable audit window. Raw extracted package contents, sandbox scratch data, private source snippets, and sensitive topology are not retained as report payloads; they expire on short operational windows and remain available only through authorized Overstore/Overvault/Overwatch refs when policy requires preservation.
