SDS #24

# Overpack SDS

## Purpose

Define the manifest and package contract that tells Overrid what a workload or application is allowed to run, which resources it needs, which inputs and outputs it touches, which policies apply, and how integrity can be verified before execution or deployment.

Overpack begins in Phase 3 as a strict workload manifest for the private execution loop. It later expands in Phase 9 into an application-intent manifest for repeatable app deployment.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overpack.md](../../service_catalog/execution_scheduling/overpack.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 3: Private Execution Loop](../../build_plan/phase_03_private_execution_loop.md), [Phase 9: Overpack Deployment Platform](../../build_plan/phase_09_overpack_deployment_platform.md) |

## Service Family

- Family: Execution and scheduling
- Owning layer: Manifest, package, provenance, runtime intent, and compatibility contract
- Primary data scope: workload manifests, application-intent manifests, artifact refs, hashes, signatures, provenance records, runtime contracts, permission declarations, resource cards, validation reports, and compatibility reports
- First build phase from service plan: workload manifest in [Phase 3](../../build_plan/phase_03_private_execution_loop.md); deployment platform in [Phase 9](../../build_plan/phase_09_overpack_deployment_platform.md)

## Problem Statement

The execution loop needs a small but strict package contract. Overrun cannot safely run a command, container, WASI module, or model job from informal instructions. Oversched cannot place a workload without resource, data, runtime, egress, secret, timeout, and retry declarations. Overguard cannot decide policy without explicit sensitivity and permission information.

Overpack provides that contract. It is the canonical manifest language for work and apps, not the runner, scheduler, storage service, or deployment planner.

## Goals

- Define Overpack v0 workload manifests for command jobs, container jobs, WASI jobs where feasible, model inference jobs, inputs, outputs, resource cards, workload class, data sensitivity, egress policy, secret policy, timeout, and retry policy.
- Include artifact hashes, signatures, provenance refs, runtime contract, permission declarations, dependency locks, and SBOM refs.
- Make manifests strict enough for Overguard, Oversched, Overlease, and Overrun to reason from.
- Support schema versioning, compatibility checks, deprecation, and migration.
- Register accepted manifest versions through Overregistry.
- Provide validation outputs that Package Validator, Deployment Planner, SDK, CLI, and admin UI can use.
- Expand to application-intent manifests in Phase 9 without breaking Phase 3 workload manifests.

## Non-Goals

- Do not execute workloads. Overrun owns execution.
- Do not decide placement. Oversched owns placement.
- Do not own durable object bytes. Overstore owns artifacts and objects.
- Do not replace Package Validator; Overpack defines schemas and compatibility rules while validators perform deeper checks.
- Do not bypass Overguard policy by embedding privileged declarations.
- Do not encode pricing or revenue forecasts.
- Do not accept AI-generated deployment proposals unless they pass the same schema, provenance, policy, and budget checks.

## Primary Actors And Clients

- SDK and CLI creating, validating, signing, and submitting manifests.
- Overregistry storing accepted manifest versions and provenance refs.
- Overguard reading workload class, data sensitivity, egress, secrets, and permissions.
- Oversched reading resource cards and runtime compatibility.
- Overlease reading resource reservation needs.
- Overrun verifying package integrity and runtime contract before execution.
- Package Validator and Deployment Planner expanding validation and deployment in Phase 9.
- AI Gateway/central AI tools proposing manifests that must remain policy-compatible.

## Dependencies

- Shared schema package for manifest schemas and generated validators.
- [Overregistry](../control_plane/overregistry.md) for immutable manifest records.
- [Overguard](../trust_policy_verification/overguard.md) for policy compatibility decisions.
- [Overrun](overrun.md) for manifest verification and execution.
- [Package Validator](../deployment_grid/package_validator.md) for deeper package and deployment validation.
- [Overstore](../data_storage_namespace/overstore.md) for artifact refs after storage platform exists.
- [Overkey](../control_plane/overkey.md) for manifest/package signatures.
- [Deployment Planner](../deployment_grid/deployment_planner.md) for Phase 9 deployment execution.

Phase 3 can ship only the workload manifest subset; Phase 9 adds application deployment intent.

## Owned Responsibilities

Overpack owns:

- Manifest schema definitions, semantic versions, compatibility rules, examples, and fixtures.
- Workload manifest contract and later application-intent manifest contract.
- Artifact ref, hash, signature, provenance, dependency lock, and SBOM field definitions.
- Runtime contract declarations for command, container, WASI, model, service, and future runtime types.
- Permission declarations for egress, secrets, storage, data class, model access, namespace routes, and service calls.
- Resource card declarations consumed by scheduling and leasing.
- Manifest lifecycle and validation reports.

Overpack must not store raw secrets or decide policy. It declares intent; other services validate and enforce it.

## Data Model

The first implementation should define:

- `manifest_envelope`: manifest id, schema version, manifest kind, tenant/app scope, author, signature refs, created_at, compatibility version, and deprecation refs.
- `workload_manifest`: job kind, command/container/WASI/model spec, input refs, output refs, runtime contract, resource card, workload class, data sensitivity, egress policy, secrets policy, timeout, retry policy, and observability refs.
- `application_intent_manifest`: app identity, services, runtime cards, data needs, storage needs, model needs, permissions, wallet budget, billing rules, routes, geography, scaling, security, health checks, and observability.
- `artifact_ref`: artifact type, object ref, content hash, signature ref, size, media/runtime type, storage class, and retention hint.
- `package_provenance`: source repo/build ref, builder identity, dependency locks, SBOM refs, base image/module refs, generated-by refs, and policy compatibility refs.
- `runtime_contract`: runtime type, entrypoint, args, environment refs, mount rules, allowed syscalls/capabilities where relevant, timeout, resource limits, and cleanup expectation.
- `permission_declaration`: egress, secrets, data, storage, namespace, service, model, RAG, and route permissions with least-privilege scope.
- `manifest_validation_report`: schema result, signature result, artifact integrity result, policy dry-run refs, budget compatibility, route ownership checks, and errors/warnings.

Common envelope fields:

- `id`, `tenant_id`, `actor_id`, `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

Overpack can be implemented as shared schemas plus a small validation/registration surface:

- `POST /overpack/manifests/validate`: validate schema, signatures, artifact refs, and basic compatibility.
- `POST /overpack/manifests`: submit a signed manifest for registration through Overregistry.
- `GET /overpack/manifests/{manifest_id}`: read authorized manifest metadata and validation state.
- `GET /overpack/schemas/{kind}/{version}`: read manifest schema and compatibility rules.
- `POST /overpack/manifests/{manifest_id}/dry-run-policy`: ask Overguard for a policy compatibility result.
- `POST /overpack/manifests/{manifest_id}/deprecate`: deprecate a manifest version with reason.
- `POST /overpack/compatibility`: compare a manifest against target runtime, scheduler, policy, or deployment-platform capabilities.

API requirements:

- Manifest submission requires actor identity, tenant context, trace id, idempotency key, and signature metadata.
- Registered manifests are immutable; changes create new versions.
- Validation reports must be stable enough for SDK/CLI/admin UI automation.
- Policy dry-run results are advisory until the workload or deployment is actually submitted.
- Reads must redact secret refs and private artifact details when caller lacks access.

## Event Surface

- `overpack.manifest_validated`: manifest validation completed.
- `overpack.manifest_rejected`: manifest rejected with reason codes.
- `overpack.manifest_registered`: manifest registered through Overregistry.
- `overpack.manifest_deprecated`: manifest version deprecated.
- `overpack.compatibility_checked`: compatibility report generated.
- `overpack.policy_dry_run_completed`: policy dry-run result attached.
- `overpack.provenance_recorded`: provenance refs accepted.

Events should include manifest refs, hashes, validation status, and reason codes, not full private manifest content unless authorized.

## Core Workflow

1. Developer, AI tool, SDK, CLI, or native app produces a workload manifest.
2. Overpack validates schema version, manifest kind, required fields, resource card, runtime contract, permissions, and signatures.
3. Artifact refs, hashes, dependency locks, and provenance refs are checked for shape and availability.
4. Optional policy dry-run checks workload class, data class, egress, secrets, and budget compatibility.
5. Accepted manifest versions are registered in Overregistry.
6. Overqueue receives a workload command referencing the accepted manifest.
7. Oversched, Overlease, and Overrun consume manifest fields through stable contracts.
8. Phase 9 adds application-intent manifests and deployment planner compatibility checks.

## State Machine

Manifest lifecycle:

1. `draft`: manifest exists locally or in a mutable workspace before submission.
2. `submitted`: signed manifest submitted for validation.
3. `schema_valid`: required schema checks passed.
4. `integrity_checked`: signatures, hashes, and artifact refs checked.
5. `policy_checked`: policy dry-run or required policy compatibility check completed.
6. `registered`: immutable manifest version accepted in Overregistry.
7. `rejected`: validation failed with reason codes.
8. `deprecated`: manifest remains readable but new use is discouraged or blocked by policy.
9. `revoked`: manifest version is blocked due to security, provenance, or incident evidence.
10. `superseded`: newer compatible version exists.

Application-intent lifecycle in Phase 9 may add `planned`, `provisioning`, `deployed`, `rolling_update`, and `rolled_back` in deployment services, but Overpack itself should still treat the manifest version as immutable input.

## Policy And Security

- Manifest signatures and artifact hashes are mandatory for execution-eligible packages.
- Secret values must never appear in manifests; manifests reference Overvault secret policies and mount refs.
- Egress, data class, storage, model, RAG, route, and service permissions must be explicit and least-privilege.
- Unknown or unsupported manifest fields should fail closed unless compatibility rules permit warnings for non-execution metadata.
- AI-generated manifests must pass the same validation, policy dry-run, and provenance requirements as human-authored manifests.
- Revocation must be possible when a package, dependency, or provenance source is found unsafe.
- Registered manifest history is append-only; corrections create new versions or revocation records.

## Metering And Accounting

Overpack does not meter runtime usage directly, but it defines accounting-relevant intent:

- Resource cards inform Overlease reservations and Overmeter attribution.
- Workload/app ids, package refs, model refs, service ids, and billing rules become dimensions in usage events.
- Budget declarations in application-intent manifests are policy/accounting inputs, not direct payment actions.
- Manifest validation and package validation may emit small system-service usage events.
- Do not encode price or business-volume projections inside manifests.

## Observability And Operations

- Builders need validation reports with reason codes, schema paths, compatibility status, and remediation hints.
- Operators need views for registered manifests, rejected submissions, revoked versions, deprecated versions, provenance warnings, and AI-generated proposal diffs.
- Health checks should confirm schema registry, signer verification, Overregistry registration, Overguard dry-run path, and artifact-ref lookup.
- Migration tooling must preserve old schemas and provide compatibility reports before changing manifest versions.
- Diff tools should show permission/resource/budget changes clearly before deployment.

## Failure Modes And Recovery

- Unknown schema version: reject with supported versions and migration hint.
- Missing signature or hash: reject before queue submission.
- Artifact ref unavailable: mark validation failed or blocked until object exists.
- Policy dry-run unavailable: keep manifest validation separate and mark policy compatibility unknown.
- Unsupported runtime: reject for execution and show compatible runtime targets.
- Revoked dependency: mark manifest revoked or blocked and notify dependent workloads/deployments.
- AI proposal over-permissioned: return minimization warnings/errors and require revision.

## Validation Plan

The service implementation plan lists these requirements:

- Invalid packages are rejected before execution.
- Overrun can verify package integrity from the manifest.
- Deployment planner can provision resources from one signed application manifest.

Additional SDS-level validation:

- Schema tests for command, container, WASI, model inference, and service manifests.
- Signature/hash validation tests.
- Secret policy tests proving raw secrets are rejected.
- Egress/data-class/permission validation tests.
- Resource-card tests proving scheduler and lease inputs are complete.
- Overrun integrity tests proving execution can verify manifest artifacts.
- Phase 9 application-intent fixture tests for runtime, data, storage, routes, billing, scaling, and health declarations.
- Compatibility tests for schema migration and deprecation.

## Build Breakdown

1. Define Overpack v0 workload manifest schema and fixtures.
2. Add artifact hashes, signatures, runtime contract, egress policy, secret policy, retry policy, timeout, and resource cards.
3. Add SDK/CLI validation and registration through Overregistry.
4. Add Overguard policy dry-run compatibility.
5. Add Overrun verification of package integrity from the manifest.
6. Add dependency lock and SBOM refs.
7. Add application-intent manifest in Phase 9.
8. Add compatibility checks for AI-generated package and deployment proposals.

The Phase 3 target is a strict workload manifest. The Phase 9 target is repeatable app deployment from a signed application-intent manifest.

## Handoff And Downstream Use

Overpack feeds Overregistry, Overguard, Oversched, Overlease, Overrun, Package Validator, Deployment Planner, SDK, CLI, admin UI, and AI-generated deployment tooling. Consumers should use the manifest contract and validation reports rather than parsing informal package instructions.

## Open Design Questions

- Resolved: Overpack v0 must make signed command-contract jobs and OCI/container jobs execution-eligible before the first real private workload. Model-inference workloads may run in v0 only through those supported runtime kinds, normally as an OCI runtime card with explicit model/artifact refs, GPU resource refs when needed, and metering dimensions. WASI/Wasmtime is schema-recognized only where the Phase 3 Overrun adapter is tested; unsupported WASI, native-service, and standalone model runtimes may validate as declared future intent but must not become execution-eligible until their runtime cards, validator profile, and runner adapter exist.
- Resolved: unknown manifest fields fail closed by default during early development. Execution, policy, accounting, secret, egress, storage, route, resource, runtime, artifact, signature, provenance, dependency, SBOM, and validation sections must reject unknown keys under the canonical JSON Schema. The only permissive surface is an explicitly typed non-execution metadata/extension map that remains covered by the manifest content hash but cannot affect policy admission, scheduling, leasing, metering, or deployment semantics.
- Resolved: the first canonical dependency evidence is an Overpack `dependency_lock` object that records native lock-file refs and artifact digests with BLAKE3/content hashes. Rust-built packages require `Cargo.lock` evidence; OCI/container packages require immutable image digest, base-image digest, and layer or bundle refs. The first canonical SBOM family is CycloneDX JSON normalized through the shared schema package because it matches the JSON/JSON Schema manifest path; SPDX or tool-native SBOMs can be accepted later as alternate attachments, but Package Validator should normalize them before policy or provenance checks.
- Resolved: validation reports should embed only the compact policy dry-run summary needed for automation: dry-run ref, decision state, policy bundle/evaluator version, expiry, matched stable reason codes, blocking missing prerequisites, required trust/provider/sandbox classes, egress/secret/cache/storage/route summaries, and budget or reservation precheck summary clearly marked as non-authoritative estimate. Full matched-rule traces, fact snapshots, private evidence refs, operator-only explanations, replay bundles, and dependency-private details remain external refs through Policy Dry-Run API, Overguard, and Overwatch.
- Resolved: AI-generated manifest diffs must be deterministic, risk-first, and machine-readable with a human review view. Diffs should group changes by permissions, budget/ORU exposure, routes and traffic, resources/runtime, storage, vault, model/RAG refs, data class, egress, provenance, health/scaling, and rollback behavior; each item should show old versus new effective scope, severity, reason codes, validation/dry-run refs, route ownership or traffic impact, estimated budget impact, and whether the change expands, narrows, or preserves privilege. Operator views must redact secrets and provider-private topology, and high-risk permission, budget, route, secret, or privileged-runtime expansion requires explicit signed approval or revision.
