SDS #48

# Grid-Resident Service Packager SDS

## Purpose

Package Overrid core services as protected, policy-bound, grid-resident system workloads.

Grid-Resident Service Packager turns backbone services into deployable service packages with runtime artifacts, config contracts, secret refs, health/readiness commands, migration commands, backup/restore commands, rollback commands, privilege profiles, and provenance. It does not decide placement, execute deployments, validate arbitrary third-party packages, run services, or choose release strategy.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [grid_resident_service_packager.md](../../service_catalog/deployment_grid/grid_resident_service_packager.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md) |

## Service Family

- Family: Deployment and grid-resident backbone
- Owning layer: System-service package contract and release artifact preparation
- Primary data scope: system-service package manifests, runtime artifact refs, config schemas, secret contracts, health/readiness commands, migration/backup/restore/rollback command contracts, privilege profiles, version pins, and provenance refs
- First build phase from service plan: [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md)

## Problem Statement

Overrid begins on founder hardware, but the backbone must eventually run inside the grid. Core services cannot be migrated safely if every service has ad hoc runtime flags, hidden secrets, undocumented restore behavior, missing health checks, or manual rollback steps. The grid needs a repeatable package contract for system services before trusted nodes can host Overgate, Overregistry, Overqueue, Oversched, Overmeter, Overwatch, Overguard, Overpass, supporting stores, and internal observability.

This service creates that package contract. It makes system-service packages explicit, signed, inspectable, and ready for package validation, deployment planning, release strategy, backup/restore, and failover.

## Goals

- Define and produce system-service package manifests for Overrid backbone services.
- Standardize runtime artifact refs, command entrypoints, config schemas, secret refs, health checks, readiness checks, metrics, logs, and audit outputs.
- Include migration, backup, restore, and rollback command contracts in every package before a service can become grid-resident.
- Bind every package to system-service workload-class rules, privilege profiles, data classes, and placement constraints.
- Record artifact hashes, build provenance, signer refs, source refs, dependency locks, and compatibility windows.
- Feed Package Validator, Deployment Planner, Release Strategy Service, Backup and Restore Service, and Failover and Recovery Coordinator with stable package facts.
- Support a migration sequence that starts with a non-critical service and ends with founder hardware removed from the normal production path.

## Non-Goals

- Do not deploy, schedule, or run the packaged service; Deployment Planner, Oversched, Overlease, Overcell, and Overrun own those stages.
- Do not validate arbitrary app packages; Package Validator validates package evidence and policy compatibility.
- Do not decide release strategy, route weights, or canary promotion.
- Do not store raw secrets in package manifests; use Overvault refs and secret contract names.
- Do not bypass System-Service Workload Class placement rules or Overguard policy checks.
- Do not package unknown public-node workloads as backbone services.
- Do not add pricing, revenue, customer-count, blockchain, NFT, or per-transaction-fee assumptions.

## Primary Actors And Clients

- Core service maintainers preparing Overrid backbone services for grid execution.
- Package Validator consuming package manifests, artifact hashes, dependency locks, command contracts, and privilege profiles.
- Deployment Planner consuming validated package records and config requirements.
- Release Strategy Service consuming version pins, compatibility windows, and rollback command refs.
- Backup and Restore Service consuming backup/restore command contracts and state target metadata.
- Failover and Recovery Coordinator consuming health/readiness, restore, promotion, and rollback command contracts.
- Overregistry storing immutable package records and refs.
- Overvault, Overbase, Overstore, Overmesh, and Overwatch consuming package-declared integration contracts.

## Dependencies

- [Overpack](../execution_scheduling/overpack.md) for package envelopes, artifact refs, provenance, runtime contracts, and manifest compatibility rules.
- [Overregistry](../control_plane/overregistry.md) for immutable package records, version refs, and accepted manifest facts.
- [Overvault](../data_storage_namespace/overvault.md) for secret scopes, secret refs, and mount contract definitions.
- [Overrun](../execution_scheduling/overrun.md) and [Overcell](../execution_scheduling/overcell.md) for executable runtime contracts and node-side command expectations.
- [System-Service Workload Class](system_service_workload_class.md) for privileged but constrained workload rules.
- [Package Validator](package_validator.md) for schema, artifact, provenance, privilege, and policy validation reports.
- [Backup and Restore Service](backup_restore_service.md) for backup target and restore command expectations.
- [Overwatch](../control_plane/overwatch.md) for logs, health, metrics, traces, and audit event requirements.

## Owned Responsibilities

Grid-Resident Service Packager owns:

- System-service package manifest schema and service-specific manifest generation.
- Runtime artifact declarations, artifact hash refs, source/build provenance refs, and dependency locks.
- Config contract definitions with defaults, required overrides, validation rules, and migration compatibility.
- Secret contract definitions that name Overvault scopes and mount behavior without exposing raw secret values.
- Health/readiness/check command definitions and expected result schemas.
- Migration, backup, restore, rollback, drain, and diagnostics command contracts.
- Privilege profile and capability declaration for system-service workload-class admission.
- Package compatibility matrix across service version, schema version, state version, and runtime version.
- Package handoff records for validator, planner, release, backup, and failover services.

## Data Model

- `system_service_package_manifest`: package root with `package_id`, `service_id`, `service_name`, `version`, `schema_version`, `workload_class`, `artifact_refs`, `config_contract_ref`, `secret_contract_ref`, `command_contract_refs`, `privilege_profile_ref`, `provenance_refs`, `compatibility_window`, `state`, and `audit_refs`.
- `runtime_artifact_ref`: immutable artifact declaration with `artifact_id`, `artifact_type`, `content_hash`, `store_ref`, `build_ref`, `source_ref`, `runtime_card`, `entrypoint`, and `allowed_platforms`.
- `config_contract`: config schema with `config_key`, `type`, `default_ref`, `required`, `data_class`, `validation_rule_refs`, `restart_required`, and `migration_behavior`.
- `secret_contract`: secret requirement with `secret_name`, `vault_scope_ref`, `mount_path_or_env_ref`, `rotation_supported`, `least_privilege_scope`, and `redaction_rule`.
- `service_command_contract`: command spec for `health`, `readiness`, `migrate`, `backup`, `restore`, `rollback`, `drain`, `diagnose`, and `pre_stop` with inputs, expected outputs, timeouts, and idempotency expectations.
- `privilege_profile`: declared capabilities, network scopes, storage scopes, service accounts, route scopes, data classes, and denial behavior when a privilege is missing.
- `package_handoff_record`: validator/planner/release/backup/failover readiness record with target service refs, required validations, and blocking gaps.

All accepted package records are immutable by version. Corrections produce superseding package versions and link to prior rejected or retired versions.

## API Surface

- `POST /system-service-packages`: creates a draft package manifest from service metadata, artifact refs, config contracts, and command contracts.
- `PUT /system-service-packages/{package_id}/contracts`: updates draft config, secret, command, privilege, and compatibility contracts before submission.
- `POST /system-service-packages/{package_id}/submit`: freezes a package version for validation and registry publication.
- `GET /system-service-packages/{package_id}`: returns package manifest, version state, compatibility, and redacted contract summaries.
- `GET /system-service-packages/{package_id}/handoff`: returns validator, planner, release, backup, and failover handoff requirements and gaps.
- `POST /system-service-packages/{package_id}/retire`: marks a package version retired after replacement and policy checks.
- `POST /system-service-packages/{package_id}/supersede`: links a new package version to a prior version with migration and rollback notes.
- `GET /system-services/{service_id}/package-versions`: lists package versions, validation state, release eligibility, and active deployment refs.

Mutating APIs require service maintainer or system-service automation identity, tenant/system scope, trace id, idempotency key, and policy decision refs. Stable errors include `artifact_hash_missing`, `secret_contract_invalid`, `command_contract_missing`, `privilege_profile_denied`, `backup_contract_missing`, `restore_contract_missing`, `rollback_not_defined`, and `incompatible_state_version`.

## Event Surface

- `grid_service_packager.package_drafted`: draft package was created.
- `grid_service_packager.contract_updated`: config, secret, command, or privilege contract changed in draft state.
- `grid_service_packager.package_submitted`: package version was frozen for validation.
- `grid_service_packager.package_registered`: accepted package refs were written to Overregistry.
- `grid_service_packager.handoff_ready`: package has the minimum contracts needed by validator, planner, backup, release, and failover services.
- `grid_service_packager.handoff_blocked`: package is missing required contracts or compatibility facts.
- `grid_service_packager.package_superseded`: a new package version superseded an older version.
- `grid_service_packager.package_retired`: a package version is no longer eligible for new deployments.

Events include service id, package id, version, workload class, manifest hash, signer refs, compatibility refs, and redacted evidence refs.

## Core Workflow

1. Select a core service candidate for grid migration according to Phase 7 migration order.
2. Collect runtime artifact refs, source refs, build provenance, dependency locks, and runtime-card requirements.
3. Define config schema, secret refs, service accounts, network scopes, storage scopes, route scopes, and audit/log/metric outputs.
4. Define health, readiness, migrate, backup, restore, rollback, drain, and diagnostics command contracts.
5. Attach system-service workload-class privilege profile and placement constraints.
6. Freeze the package version and submit it to Package Validator.
7. Publish accepted package facts to Overregistry.
8. Produce handoff records for Deployment Planner, Release Strategy Service, Backup and Restore Service, and Failover and Recovery Coordinator.
9. Supersede or retire package versions as services mature and move off founder hardware.

## State Machine

Package lifecycle:

1. `draft`: contracts are editable and unpublished.
2. `submitted`: package version is frozen and awaiting validation.
3. `validation_failed`: validator rejected the package with stable reason codes.
4. `validated`: package passed required validation checks.
5. `registered`: immutable package version is published to Overregistry.
6. `release_eligible`: release strategy and deployment planner can use this package for new deployments.
7. `active`: at least one grid-resident workload uses this package.
8. `superseded`: a newer package version replaces this one for new deployments.
9. `retired`: no new deployment may use this package.
10. `revoked`: policy or security issue makes the package ineligible.

Contract lifecycle:

1. `missing`
2. `drafted`
3. `complete`
4. `validated`
5. `incompatible`
6. `superseded`

No submitted package version is mutated in place. A changed artifact, command, config schema, or privilege profile creates a new package version.

## Policy And Security

- Require signed artifacts and signer refs before submission.
- Require Overguard policy checks for system-service workload class, privilege profile, service account scope, route scope, data class, and node eligibility.
- Treat package manifests as high-trust system artifacts; reject unknown or unverified package sources.
- Keep raw secrets out of package records and event payloads; only names, refs, scopes, and redaction rules are stored.
- Require least-privilege declarations for network, storage, vault, route, and control-plane access.
- Require command contracts for backup, restore, rollback, and drain before a service can be release eligible.
- Require compatibility windows so Deployment Planner cannot deploy packages against incompatible state or schema versions.
- Make package revocation auditable and link it to affected deployments and release pins.

## Metering And Accounting

- Emit package build, validation, registry publication, and package-handoff usage events where material.
- Tag package records with service id, workload class, artifact size, validation workload, and storage refs for Overmeter rollups.
- Keep system-service packaging overhead visible as backbone operating cost, separate from user workload usage.
- Use ORU and Seal Ledger through accounting services only when package operations consume measurable grid resources.
- Do not encode per-package fees, revenue projections, or speculative product economics.

## Observability And Operations

- Expose package service health, draft/submitted/validated counts, validation lag, handoff gaps, retired/revoked package counts, and compatibility warnings.
- Provide operator views showing package readiness for deployment, backup, restore, rollback, release, and failover.
- Track missing contracts by service so Phase 7 migration blockers are visible.
- Support deterministic package diff between two versions.
- Support package provenance review for security and incident response.
- Maintain redacted package summaries for stewardship reporting without exposing private routes, secret names beyond allowed scope, or security-sensitive topology.

## Failure Modes And Recovery

- Missing artifact hash or provenance: block submission.
- Secret contract contains raw secret material: reject draft update and emit security event.
- Health/readiness command missing: block validation and release eligibility.
- Backup/restore/rollback command missing: block grid-resident promotion for stateful services.
- Privilege profile too broad: validator or Overguard denies release eligibility.
- Runtime contract incompatible with Overrun or Overcell: keep package in validation failed state.
- Package version deployed and later revoked: notify Release Strategy Service, Deployment Planner, Failover and Recovery Coordinator, and incident response with affected package refs.
- Registry publication failure: keep package validated but unregistered and retry with idempotent registry command.
- Handoff gap discovered after validation: create a superseding package version or mark release eligibility blocked.

## Validation Plan

- A non-critical core service can be packaged with artifact, config, secret, health, readiness, migration, backup, restore, rollback, and diagnostics contracts.
- Package Validator catches missing command contracts, invalid secret refs, overly broad privileges, and incompatible runtime cards.
- Deployment Planner can consume a validated package handoff without bespoke service-specific code.
- Backup and Restore Service can call declared backup/restore command contracts in a drill.
- Failover and Recovery Coordinator can read health/readiness and drain/promote requirements from the package.
- A package version can be superseded without mutating the old accepted manifest.
- Rollback to a previous package version restores the expected service command set and compatibility facts.

## Build Breakdown

1. Define system-service package manifest, runtime artifact, config contract, secret contract, command contract, privilege profile, and handoff schemas.
2. Package one non-critical observability replica with full contracts.
3. Add Package Validator integration and stable validation reason codes.
4. Add Overregistry publication for immutable accepted package versions.
5. Add backup/restore and rollback command contract enforcement.
6. Add package diff, supersede, retire, and revoke flows.
7. Package queue worker, policy service, metering service, API ingress replica, and primary control-plane services in the Phase 7 migration order.
8. Prove one packaged service can deploy, roll back, back up, restore, and fail over through the other deployment-grid services.

## Handoff And Downstream Use

Grid-Resident Service Packager hands package evidence to Package Validator first. Accepted package records then feed Deployment Planner, Release Strategy Service, Backup and Restore Service, Failover and Recovery Coordinator, Overregistry, and operator tooling.

Implementation tickets should treat this service as the package-contract authority for backbone services. It should not be expanded into a universal app store or deployment executor.

## Open Design Questions

- The first actual repo implementation target is a non-critical Overwatch/internal-observability replica, not a primary control-plane writer. That matches the Phase 7 migration order, exercises artifact, config, secret-ref, health, readiness, diagnostics, metrics, log, provenance, and handoff contracts, and avoids putting Overgate, Overqueue, Seal Ledger, Overguard policy, active Overregistry manifests, or other write-head state on the first grid-resident package. The first package should be allowed only as `eligible_for_test` or `eligible_for_noncritical` until Package Validator, System-Service Workload Class, Backup and Restore Service, Release Strategy Service, and Failover and Recovery Coordinator all have usable evidence refs.
- Stateless service packages still need an explicit command contract, but their state commands can declare `no_state` behavior instead of pretending to back up data they do not own. The minimum stateless contract is health, readiness, diagnose, pre-stop, drain when route or queue traffic exists, rollback-to-previous-artifact, log/metric/audit output declarations, and explicit no-op or not-applicable migrate/backup/restore fields with reason codes. Stateful control-plane packages additionally require idempotent migrate, backup, restore, rollback, drain, quiesce/fence, verify, and reconcile commands with checkpoint or cursor inputs, expected output schemas, restore-order hints, Overvault grant refs, compatibility windows, writer-guard prerequisites, and rollback evidence refs.
- General Overpack application packages and system-service packages should share the manifest envelope, schema version, package kind, tenant or system scope, artifact refs, BLAKE3/content hashes, signatures, provenance refs, dependency locks, SBOM refs, runtime cards, entrypoints, platform constraints, config-schema shape, secret-ref declaration shape, permission declarations, health/readiness declarations, validation report refs, compatibility/deprecation fields, and immutable version/supersession behavior. System-service-only fields are the `system_service` workload class, eligible-service refs, class-version refs, privileged service-account scopes, control-plane route/storage/vault/network privilege profiles, mandatory backup/restore/migration/rollback/drain command contracts, node eligibility and placement guardrail refs, founder-migration refs, break-glass/operator-action requirements, release/failover handoff records, and active-writer or restore promotion gates.
- Early internal packages are trusted by Package Validator only under a bounded internal system-service validation profile. The package manifest must be canonical JSON/JSON Schema, signed with an Overkey-registered Ed25519 system-service maintainer or build-service key authorized in Overregistry, and tied to BLAKE3 manifest and artifact hashes. Provenance must include source ref, build recipe, Rust/Cargo workspace or runtime-card version, dependency lock, SBOM or documented v0 dependency inventory, builder identity, schema/ruleset version, artifact store ref, signer authority ref, and Overwatch audit ref. Until reproducible CI and full rule coverage exist, these packages remain private-grid/internal only and cannot be promoted to public, production-primary, or founder-hardware-removal use without the stricter system-service validation profile passing.
- Package revocation propagates as a classed control signal rather than an unconditional process kill. Overregistry records the revoked package version immediately, Package Validator blocks new validation and placement, System-Service Workload Class marks affected service/package tuples ineligible for new placements, and Release Strategy Service freezes promotion/version-pin advancement. Active deployments receive a severity-based response: low-risk metadata or dependency warnings keep the current package pinned while a superseding package is prepared; runtime or policy incompatibility drains traffic and rolls forward through Deployment Planner and Release Strategy; security, secret, artifact-integrity, or writer-safety revocations quarantine the package, block new routes, and hand affected system-service scopes to Failover and Recovery Coordinator. Stateful services must preserve current traffic or writer state until backup/restore refs, writer guards, route shifts, and rollback plans are safe, unless Overguard/incident policy explicitly requires emergency stop.
