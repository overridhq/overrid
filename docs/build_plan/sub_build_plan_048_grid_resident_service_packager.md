# SUB BUILD PLAN #48 - Grid-Resident Service Packager

Attached SDS: [docs/sds/deployment_grid/grid_resident_service_packager.md](../sds/deployment_grid/grid_resident_service_packager.md)

## Purpose

This sub-build plan turns SDS #48 into an implementation sequence for Grid-Resident Service Packager. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Grid-Resident Service Packager is the package-contract authority for protected Overrid system services. It owns system-service package manifests, runtime artifact refs, config contracts, secret contracts, health/readiness commands, migration/backup/restore/rollback/drain/diagnostics command contracts, privilege profiles, compatibility windows, package handoff records, package lifecycle transitions, and package provenance without deploying services, validating arbitrary app packages, choosing release strategy, scheduling workloads, running workloads, or storing raw secrets.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #48: Grid-Resident Service Packager](../sds/deployment_grid/grid_resident_service_packager.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Grid-Resident Service Packager service plan](../service_catalog/deployment_grid/grid_resident_service_packager.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, deterministic fixtures, canonical JSON/JSON Schema discipline, signed command envelopes, stable reason codes, trace ids, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate ingress, Overpass identity, Overtenant scope, Overkey signatures, Overregistry refs, Overwatch audit, Overqueue state, and system-service automation identities. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack manifest-envelope conventions, Overrun/Overcell runtime cards, Oversched/Overlease execution facts, Overmeter usage facts, and early package validation prerequisites. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard decisions, System-Service Workload Class facts, node eligibility, workload/data classification, policy dry-run behavior, and replayable denial reasons. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies system-service usage recording, accounting evidence refs, ORU/Overbill/Seal Ledger hooks, and stewardship cost visibility without pricing or revenue assumptions. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Controls the first build point: system-service workload class, trusted placement, service packaging, backup/restore/rollback command requirements, non-critical first migration, and founder-hardware removal gates. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies later native Overbase, Overstore, Overvault, Universal Namespace Service, and Overmesh depth for structured state, artifacts, secrets, routes, namespace refs, and restore metadata. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies later shared Overpack application-intent expansion, Deployment Planner handoffs, Release Strategy handoffs, package validation/reporting depth, and app-deployment compatibility rules. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies incident response, threat modeling, security review, revocation hardening, audit exports, compliance boundaries, reliability drills, and public reporting controls. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #48 first build work aligned to master Phase 7, with Phase 8/9 integrations and Phase 13 governance, incident, security, and reliability hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 7, 8, 9, and 13 | Attach SDS #48, freeze authority boundaries, preserve Phase 7 as first build point, and record later storage/deployment/governance gates. |
| 2 | Master Phases 0, 1, 3, 4, and 7 | Define Rust contracts, canonical schemas, manifest lifecycle, immutable version semantics, fixtures, reason codes, and signed evidence refs. |
| 3 | Master Phases 0, 1, 3, 7, 8, and 9 | Implement artifact, provenance, dependency, config, secret-ref, runtime-card, and manifest-envelope authoring without raw secrets or conventional product boundaries. |
| 4 | Master Phases 1, 3, 4, 7, 8, and 9 | Implement command contracts, privilege profiles, system-service workload-class gates, compatibility windows, and handoff readiness records. |
| 5 | Master Phases 1, 3, 4, 7, and 13 | Package the first non-critical Overwatch/internal-observability replica under bounded internal validation and non-critical eligibility gates. |
| 6 | Master Phases 1, 3, 4, 7, 8, 9, and 13 | Implement submission, validator handoff, Overregistry publication, release eligibility, lifecycle states, events, and immutable accepted records. |
| 7 | Master Phases 4, 7, 8, 9, and 13 | Implement package diff, supersede, retire, revoke, severity-classed revocation propagation, rollback refs, and incident/security handoffs. |
| 8 | Master Phases 3, 4, 5, 7, 8, 9, and 13 | Expand packaging through the Phase 7 migration sequence while preserving writer, restore, validation, release, and founder-exit gates. |
| 9 | Master Phases 5, 6, 7, 9, 12, and 13 | Add APIs, CLI/SDK/admin surfaces, operational projections, metering hooks, redaction, and downstream owner-service handoffs. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Grid-Resident Service Packager core is a Rust service/module using shared contract crates, Tokio for bounded workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Package manifests, config contracts, secret contracts, command contracts, privilege profiles, compatibility windows, handoff records, package events, fixtures, redaction profiles, audit exports, and API objects use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed service-maintainer or build-service envelopes, tenant/system scope, trace id, idempotency key, schema version, policy refs, stable reason codes, and append-only Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for manifests, runtime artifacts, dependency locks, SBOM or dependency inventories, handoff records, validation refs, replay bundles, and deterministic fixture comparison.
- Package records may point to Overpack, Overregistry, Overvault, Overrun, Overcell, System-Service Workload Class, Package Validator, Backup and Restore Service, Deployment Planner, Release Strategy Service, Failover and Recovery Coordinator, Overwatch, Overbase, Overstore, Overmesh, Overmeter, ORU, Overbill, Seal Ledger, Incident Response Service, and Stewardship Reporting Service, but the packager must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, revenue projections, customer-count assumptions, raw secret storage, direct payment execution, release-strategy ownership, deployment-planner ownership, package-validator ownership, backup storage ownership, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Package Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #48.**
  - Design: Link this document from the Grid-Resident Service Packager SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/deployment_grid/grid_resident_service_packager.md`, `docs/service_catalog/deployment_grid/grid_resident_service_packager.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #48 returns both the Grid-Resident Service Packager SDS and this sub-build plan.

- **1.2 Freeze package-contract authority boundaries.**
  - Design: Record that the packager owns package manifests, artifact refs, config contracts, secret contracts, command contracts, privilege profiles, compatibility windows, provenance, lifecycle transitions, handoff records, and package diff/supersession facts.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the packager does not deploy, schedule, run workloads, validate arbitrary app packages, choose release strategy, store raw secrets, bypass Overguard/System-Service Workload Class, own backup storage, or package unknown public-node workloads as backbone services.

- **1.3 Preserve master Phase 7 as the first build point.**
  - Design: Keep first implementation in Phase 7 after control-plane, identity, policy, audit, accounting, private execution, and product-integration prerequisites exist.
  - Output: Phase-gate note that Phase 7 builds system-service package contracts, Phase 8 adds native storage/vault/namespace depth, Phase 9 consumes shared package/deployment/release handoffs, and Phase 13 hardens incident/security/governance review.
  - Validation: Review proves this plan does not move Phase 8 data-platform ownership into Phase 7, does not make Phase 9 app deployment a prerequisite for first system-service packaging, and does not reorder master Phase 0 through Phase 13.

- **1.4 Carry forward resolved SDS #48 decisions.**
  - Design: Preserve the first target as a non-critical Overwatch/internal-observability replica, explicit `no_state` semantics for stateless packages, shared Overpack fields only where common, system-service-only fields for privileged backbone packaging, bounded private-grid validation for early packages, and severity-classed package revocation.
  - Output: Resolved-decision checklist tied to contracts, first package, validator handoff, lifecycle states, revocation, release/failover handoffs, and founder-exit gates.
  - Validation: Review rejects primary control-plane writer packaging as the first target, fake backup semantics for stateless packages, raw-secret manifests, public/promoted package eligibility without stricter validation evidence, and unconditional kill-style revocation for stateful services.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Overpack, Overregistry, Overvault, Overrun, Overcell, System-Service Workload Class, Package Validator, Backup and Restore Service, Deployment Planner, Release Strategy Service, Failover and Recovery Coordinator, Overwatch, Overguard, Overbase, Overstore, Overmesh, Overmeter, ORU Account Service, Overbill, Seal Ledger, Incident Response Service, CLI, SDK, and Admin UI.
  - Output: Boundary matrix listing owner, input refs, output refs, package facts, command contract, validation rule, compatibility gate, policy gate, audit refs, redaction class, and downstream consumer.
  - Validation: Review confirms every handoff uses explicit APIs, immutable refs, signed evidence, reason codes, trace ids, policy refs, idempotency keys, and Overwatch events rather than privileged shared tables or hidden control paths.

## Phase 2: Rust Contracts, Canonical Schemas, Lifecycles, And Fixtures

### Work Items

- **2.1 Create the Grid-Resident Service Packager Rust contract module.**
  - Design: Add contract types for package manifests, runtime artifacts, config contracts, secret contracts, command contracts, privilege profiles, compatibility windows, handoff records, lifecycle transitions, API errors, events, redaction profiles, and audit exports.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, package-kind enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Package Validator, Deployment Planner, Release Strategy Service, Backup and Restore Service, Failover and Recovery Coordinator, Overvault storage, Overregistry internals, and workload execution.

- **2.2 Define canonical system-service package manifest schemas.**
  - Design: Model `system_service_package_manifest` with `package_id`, `service_id`, `service_name`, `version`, `schema_version`, `workload_class`, `artifact_refs`, `config_contract_ref`, `secret_contract_ref`, `command_contract_refs`, `privilege_profile_ref`, `provenance_refs`, `compatibility_window`, `state`, and `audit_refs`.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and manifest hash fixtures.
  - Validation: Schema tests reject missing package id, service id, workload class, artifact refs, command contract refs, privilege profile refs, signer refs, compatibility window, schema version, lifecycle state, trace id, or stable reason codes.

- **2.3 Define artifact, provenance, and dependency evidence schemas.**
  - Design: Model `runtime_artifact_ref`, source refs, build refs, dependency lock refs, SBOM or `dependency_inventory_v0` refs, builder identity, runtime-card refs, allowed platforms, BLAKE3 hashes, signer authority refs, and Overwatch audit refs.
  - Output: Artifact schema, provenance schema, dependency evidence schema, canonical hash examples, signature examples, source/build examples, and invalid dependency fixtures.
  - Validation: Tests reject artifact refs without content hash, store ref, build/source provenance, signer authority, runtime card, dependency evidence, allowed platform, or redacted audit refs.

- **2.4 Define package lifecycle, contract lifecycle, and immutable version rules.**
  - Design: Model package states from draft through submitted, validation_failed, validated, registered, release_eligible, active, superseded, retired, and revoked, plus contract states from missing through drafted, complete, validated, incompatible, and superseded.
  - Output: Lifecycle state machine, transition rules, supersession model, immutable accepted-version behavior, retry/idempotency semantics, and state-transition events.
  - Validation: Tests prove submitted and accepted package versions cannot be mutated in place, draft changes are allowed only before submission, and corrections create superseding package versions linked to prior rejected or retired versions.

- **2.5 Create deterministic package fixtures.**
  - Design: Build fixtures for valid draft, missing artifact hash, raw secret rejection, missing health/readiness command, stateless `no_state` package, stateful package, too-broad privilege profile, incompatible runtime card, validation failed, registered, superseded, retired, revoked, and handoff blocked.
  - Output: Fixture directory, canonical inputs, expected events, expected handoff records, reason codes, manifest hashes, signatures, redacted views, and replay bundles.
  - Validation: Fixture tests produce stable ids, BLAKE3 hashes, signatures, state transitions, denial reasons, audit refs, and idempotency outcomes across repeated runs.

## Phase 3: Artifact, Provenance, Config, Secret, And Manifest Authoring

### Work Items

- **3.1 Implement runtime artifact reference authoring.**
  - Design: Collect immutable runtime image/package artifact refs, content hashes, store refs, build refs, source refs, runtime cards, entrypoints, dependency locks, SBOM or dependency inventory refs, and allowed platform declarations.
  - Output: Artifact authoring API, artifact validator, artifact-ref diff support, provenance recorder, and `grid_service_packager.artifact_ref_added` style internal events where needed.
  - Validation: Tests prove missing hashes, mutable refs, unsupported platform refs, missing provenance, incompatible runtime cards, and unsigned artifacts block package submission.

- **3.2 Implement config contract authoring.**
  - Design: Define config schemas with keys, types, defaults or default refs, required overrides, data classes, validation rule refs, restart requirements, migration behavior, and compatibility effects.
  - Output: Config-contract APIs, schema validators, defaults model, migration behavior flags, redacted summaries, and config diff views.
  - Validation: Tests reject unknown config types, missing required overrides, incompatible migration behavior, unsafe default refs, wrong data class, and draft updates after submission.

- **3.3 Implement secret contract authoring without raw secret storage.**
  - Design: Define secret requirements by name, Overvault scope ref, mount path or environment ref, rotation support, least-privilege scope, redaction rule, and denial behavior when a required secret is unavailable.
  - Output: Secret-contract APIs, Overvault ref validator, raw-secret detector, redaction profile, rotation flag model, and least-privilege review fixtures.
  - Validation: Tests prove raw secret material is rejected, only scoped refs are stored, redacted summaries never expose protected values, and missing vault scope refs block package readiness.

- **3.4 Implement shared Overpack envelope compatibility.**
  - Design: Reuse general Overpack fields only for manifest envelope, schema version, package kind, tenant/system scope, artifact refs, content hashes, signatures, provenance refs, dependency locks, SBOM refs, runtime cards, entrypoints, platform constraints, config-schema shape, secret-ref declaration shape, permission declarations, health/readiness declarations, validation report refs, compatibility/deprecation fields, and immutable version/supersession behavior.
  - Output: Shared-envelope mapping, compatibility tests, generated docs examples, and package-kind discriminator fixtures.
  - Validation: Review confirms system-service-only fields do not leak into general app manifests and general app deployment intent does not become a prerequisite for Phase 7 system-service packages.

- **3.5 Implement draft manifest composition and redacted reads.**
  - Design: Compose draft manifests from artifact, config, secret, command, privilege, compatibility, and provenance parts while exposing redacted summaries to authorized operators and downstream services.
  - Output: `POST /system-service-packages`, `GET /system-service-packages/{package_id}`, draft query projection, redaction profiles, package hash calculation, and draft update events.
  - Validation: Tests prove draft composition is deterministic, redaction hides private routes and secret details, repeated create calls are idempotent, and draft reads include blocking gaps without exposing raw protected content.

## Phase 4: Command Contracts, Privilege Profiles, Compatibility, And Handoff Readiness

### Work Items

- **4.1 Implement health, readiness, diagnostics, and pre-stop command contracts.**
  - Design: Define command specs for health, readiness, diagnose, pre-stop, and drain where route or queue traffic exists, with inputs, expected outputs, timeouts, idempotency expectations, result schemas, and log/metric/audit output declarations.
  - Output: Command-contract schema, command authoring API, valid/invalid examples, timeout model, idempotency model, and command output validators.
  - Validation: Tests reject packages missing health/readiness, diagnostics, pre-stop, traffic drain where applicable, expected output schemas, timeouts, idempotency declarations, or audit output declarations.

- **4.2 Implement migrate, backup, restore, rollback, verify, reconcile, and state ownership contracts.**
  - Design: Split stateless and stateful behavior: stateless packages declare explicit `no_state` migrate/backup/restore semantics with reason codes, while stateful packages require idempotent migrate, backup, restore, rollback, drain, quiesce/fence, verify, and reconcile contracts with checkpoint or cursor inputs.
  - Output: State ownership model, `no_state` contract shape, stateful command contract shape, restore-order hints, Overvault grant refs, writer-guard prerequisites, compatibility windows, and rollback evidence refs.
  - Validation: Tests reject fake no-op state commands without explicit reason codes, stateful packages without restore/rollback/fence/verify/reconcile commands, and control-plane packages without checkpoint or writer-guard refs.

- **4.3 Implement system-service privilege profiles and class refs.**
  - Design: Define declared capabilities, network scopes, storage scopes, route scopes, vault scopes, service accounts, data classes, denial behavior, system-service workload-class version refs, node eligibility guardrail refs, and break-glass/operator-action requirements.
  - Output: Privilege profile schema, System-Service Workload Class adapter, Overguard fact bundle, profile diff view, and least-privilege denial reason codes.
  - Validation: Tests prove too-broad profiles, missing class refs, unknown public-node eligibility, missing policy refs, wrong service account scopes, and missing denial behavior block release eligibility.

- **4.4 Implement compatibility windows and dependency gates.**
  - Design: Track service version, package schema version, state version, runtime version, config migration behavior, dependency lock version, class version, validation ruleset version, and supported upgrade/rollback windows.
  - Output: Compatibility matrix, compatibility API, blocking gap model, deprecated/incompatible state handling, and compatibility diff fixtures.
  - Validation: Tests prove Deployment Planner, Release Strategy Service, Backup and Restore Service, and Failover and Recovery Coordinator cannot use a package when state/schema/runtime/ruleset windows are incompatible.

- **4.5 Build handoff readiness records for owner services.**
  - Design: Produce `package_handoff_record` objects for Package Validator, Deployment Planner, Release Strategy Service, Backup and Restore Service, Failover and Recovery Coordinator, Overregistry, and operator tooling with required validations and blocking gaps.
  - Output: `GET /system-service-packages/{package_id}/handoff`, handoff gap reason codes, downstream-specific projections, and redacted evidence refs.
  - Validation: Tests prove handoff records include package id, service id, version, manifest hash, signer refs, required validations, command contract refs, compatibility refs, and blocking gaps without giving the packager ownership of downstream decisions.

## Phase 5: First Non-Critical Package And Bounded Internal Validation

### Work Items

- **5.1 Select and scope the first package target.**
  - Design: Use a non-critical Overwatch/internal-observability replica as the first repo implementation target, not Overgate, Overqueue, Seal Ledger, Overguard policy, active Overregistry manifests, or any primary control-plane writer.
  - Output: First-target package brief, eligibility boundary, excluded-service list, failure-domain notes, and acceptance criteria for `eligible_for_test` and `eligible_for_noncritical`.
  - Validation: Review proves the first target exercises artifact, config, secret-ref, health, readiness, diagnostics, metrics, log, provenance, and handoff contracts without carrying write-head state or founder-exit risk.

- **5.2 Package the non-critical observability replica end to end.**
  - Design: Author artifact refs, config contract, secret contract, health/readiness/diagnostics/pre-stop/drain command contracts, no-state declarations where applicable, privilege profile, compatibility window, and provenance bundle.
  - Output: Draft package manifest, redacted operator summary, manifest hash, signature refs, handoff record, and deterministic fixtures.
  - Validation: Tests prove the package can move from draft to submitted with complete contracts and remains blocked from production-primary or founder-hardware-removal use.

- **5.3 Enforce bounded internal validation profile for early packages.**
  - Design: Limit early internal packages to a private-grid system-service validation profile using canonical JSON/JSON Schema, Overkey-registered Ed25519 maintainer or build-service signatures, BLAKE3 manifest/artifact hashes, source/build/dependency/SBOM or v0 inventory provenance, builder/runtime metadata, schema/ruleset versions, artifact-store refs, signer authority refs, and Overwatch audit evidence.
  - Output: Internal validation profile, required evidence checklist, signer-authority lookup, validation handoff refs, and promotion-blocking rules.
  - Validation: Tests prove early packages cannot become public, production-primary, or founder-hardware-removal eligible until the stricter system-service validation profile passes.

- **5.4 Produce validator, backup, release, deployment, and failover handoff gaps.**
  - Design: Generate handoff records that show what Package Validator, Backup and Restore Service, Release Strategy Service, Deployment Planner, and Failover and Recovery Coordinator need before each eligibility state can advance.
  - Output: Gap report, downstream readiness matrix, missing-contract reason codes, operator view, and action list.
  - Validation: Tests prove missing backup/restore, rollback, route, release, failover, privilege, validation, or compatibility facts produce explicit `handoff_blocked` evidence rather than hidden failure.

- **5.5 Validate non-critical rollback and supersession before broader migration.**
  - Design: Prove the first package can supersede a draft, retire an accepted version, roll back to a prior artifact, and preserve immutable version records without mutating accepted manifests.
  - Output: Supersession fixture, rollback fixture, retired-package record, audit events, and package diff report.
  - Validation: Tests prove rollback restores expected command sets and compatibility facts, old accepted manifests remain readable, and downstream refs point to the correct package version.

## Phase 6: Submission, Validator Handoff, Registry Publication, And Release Eligibility

### Work Items

- **6.1 Implement package submission and freeze behavior.**
  - Design: Add `PUT /system-service-packages/{package_id}/contracts` for draft-only contract updates and `POST /system-service-packages/{package_id}/submit` to freeze a package version for validation and registry publication.
  - Output: Submission API, freeze state transition, idempotency behavior, mutation blocker, `grid_service_packager.package_submitted` event, and stable errors.
  - Validation: Tests prove submitted package versions cannot be mutated, duplicate submissions are idempotent, missing evidence blocks submission, and all submission attempts produce audit refs.

- **6.2 Implement Package Validator handoff and validation state intake.**
  - Design: Send frozen package refs, manifest hash, artifact refs, command contracts, privilege profile, provenance refs, compatibility facts, and validation profile to Package Validator while ingesting reason-coded validation outcomes.
  - Output: Validator adapter, validation request record, validation report refs, validation_failed and validated states, and rejection mapping.
  - Validation: Contract tests prove validation outcomes are version-specific, rejected packages do not advance, accepted packages retain immutable validation refs, and the packager does not own validator internals.

- **6.3 Implement Overregistry publication for accepted package versions.**
  - Design: Publish accepted package refs, manifest hash, signer refs, version state, validation report refs, compatibility refs, and supersession links to Overregistry after validation passes.
  - Output: Registry publication adapter, retry/idempotency behavior, `grid_service_packager.package_registered` event, and publication failure handling.
  - Validation: Tests prove registry publication failure leaves the package validated but unregistered, retry is idempotent, and registered records remain immutable by version.

- **6.4 Implement release-eligibility gating.**
  - Design: Move packages to `release_eligible` only when validation, registry publication, command contracts, compatibility windows, privilege profiles, backup/restore/rollback/drain rules, and downstream handoff gaps satisfy the target eligibility class.
  - Output: Release-eligibility evaluator, eligibility classes, blocking reason codes, state transition events, and operator status views.
  - Validation: Tests prove packages can be `eligible_for_test`, `eligible_for_noncritical`, or blocked without implying primary production or founder-hardware-removal eligibility.

- **6.5 Implement package version listing and active deployment refs.**
  - Design: Add `GET /system-services/{service_id}/package-versions` with validation state, release eligibility, active deployment refs, supersession links, retired/revoked state, and redacted compatibility summaries.
  - Output: Package-version query API, pagination, filters, redaction profiles, and active-deployment refs owned by Deployment Planner/Release Strategy inputs.
  - Validation: Tests prove version lists are ordered, immutable records remain readable, active refs do not become packager-owned deployment truth, and tenant/operator views are properly redacted.

## Phase 7: Diff, Supersede, Retire, Revoke, And Incident Handoffs

### Work Items

- **7.1 Implement deterministic package diff.**
  - Design: Compare two package versions across artifacts, config, secrets, commands, privileges, compatibility, provenance, validation refs, lifecycle state, and handoff gaps.
  - Output: Machine-readable diff, compact reviewer view, risk grouping, redacted operator projection, and diff hash.
  - Validation: Tests prove diffs are stable, redact protected topology and secret details, highlight privilege expansion and command-contract changes, and link to source package versions.

- **7.2 Implement supersede and retire flows.**
  - Design: Add `POST /system-service-packages/{package_id}/supersede` and `POST /system-service-packages/{package_id}/retire` with migration notes, rollback notes, replacement refs, policy refs, and downstream notification refs.
  - Output: Supersede API, retire API, lifecycle transitions, replacement graph, audit events, and blocked-state reasons.
  - Validation: Tests prove active package versions cannot be retired unsafely, supersession preserves rollback notes, and retired package versions are ineligible for new deployments.

- **7.3 Implement package revocation severity classes.**
  - Design: Propagate revocation as a classed control signal: low-risk metadata/dependency warnings keep current pins while superseding packages are prepared; runtime or policy incompatibility drains traffic and rolls forward; security, secret, artifact-integrity, or writer-safety revocations quarantine and hand affected scopes to Failover and Recovery Coordinator.
  - Output: Revocation model, severity classes, policy refs, owner-service notification contracts, and `grid_service_packager.package_revoked` event.
  - Validation: Tests prove revocation immediately blocks new validation and placement through Overregistry, Package Validator, System-Service Workload Class, and Release Strategy refs while preserving stateful traffic/writer safety unless incident policy requires emergency stop.

- **7.4 Implement affected package and deployment impact projections.**
  - Design: Identify affected service/package tuples, validation ruleset versions, signer refs, artifact refs, active deployment refs, release pins, backup/restore command refs, and failover handoff refs.
  - Output: Impact query, affected-scope report, downstream notification refs, incident handoff payload, and redacted stewardship summary.
  - Validation: Tests prove impact reports include enough refs for Release Strategy Service, Deployment Planner, Failover and Recovery Coordinator, Incident Response Service, and Stewardship Reporting Service without exposing raw secrets or private topology.

- **7.5 Implement security and incident evidence retention hooks.**
  - Design: Preserve package revocation, supersession, retirement, signer, artifact, manifest, validation, handoff, and incident refs under Phase 13 retention and redaction rules.
  - Output: Evidence retention profile, Incident Response handoff, threat-model review checklist, audit export schema, and security follow-up refs.
  - Validation: Tests prove security-critical package evidence remains replayable, redacted views are safe, and raw package internals expire or remain pinned only under authorized retention refs.

## Phase 8: Phase 7 Migration Sequence And Founder-Exit Packaging Gates

### Work Items

- **8.1 Package the read-only registry or API replica after non-critical success.**
  - Design: Advance only after the first observability package passes non-critical validation, then package a read-only registry/API replica with explicit read-only privilege profile, route/readiness checks, rollback contract, and no writer promotion.
  - Output: Read-only replica package, class eligibility refs, route handoff refs, compatibility window, and rollback package refs.
  - Validation: Tests prove read-only packages cannot mutate active Overregistry manifests, route changes require owner-service evidence, and rollback preserves read-only command contracts.

- **8.2 Package low-risk worker and queue worker services.**
  - Design: Package worker process and queue worker targets with Overqueue lane semantics, drain/pre-stop commands, Overlease proof refs, duplicate-execution guard refs, health/readiness checks, and rollback behavior.
  - Output: Worker package manifests, queue-worker package manifests, drain contracts, lease refs, and handoff records for Oversched, Overlease, Overqueue, and Failover.
  - Validation: Tests prove queue worker packages cannot advance without duplicate-execution prevention, lane drain behavior, lease refs, and idempotent rollback/evidence contracts.

- **8.3 Package policy and metering services with stricter privilege boundaries.**
  - Design: Package Overguard/policy and Overmeter/metering services only with explicit policy bundle refs, usage-fact boundaries, audit exports, least-privilege access, compatibility windows, and restore/failover handoff refs.
  - Output: Policy-service package, metering-service package, privilege diff views, audit refs, and blocked-gate checklists.
  - Validation: Tests prove policy packages cannot modify policy truth outside owner contracts, metering packages cannot mutate accounting truth, and both remain blocked without restore/failover evidence.

- **8.4 Package API ingress and primary control-plane paths only after safety gates.**
  - Design: Prepare API ingress replicas and later primary control-plane paths with route scopes, service-account scopes, break-glass rules, backup/restore/rollback/drain contracts, writer/restore gates, and compatibility proof.
  - Output: API ingress package, primary-path package readiness records, route/failover handoff refs, operator approval requirements, and founder-exit blocker reasons.
  - Validation: Tests prove primary control-plane packages remain in blocked or approval-required states until Package Validator, System-Service Workload Class, Backup and Restore Service, Release Strategy Service, and Failover and Recovery Coordinator provide usable evidence refs.

- **8.5 Enforce founder-hardware removal packaging gates.**
  - Design: Require every migrated backbone package to have current validation, registry publication, restore/rollback/failover command contracts, package diff/revocation behavior, recent drill evidence, and owner-service handoff readiness before founder hardware leaves the normal path.
  - Output: Founder-exit packaging checklist, package eligibility report, drill refs, rollback package refs, incident refs, and approval requirements.
  - Validation: Review proves founder hardware remains emergency fallback until restore, failover, rollback, package revocation, and rehearsal evidence pass under current policies.

## Phase 9: APIs, Operations, Metering, Interfaces, And Downstream Handoffs

### Work Items

- **9.1 Implement package service APIs and status projections.**
  - Design: Expose create, update draft contracts, submit, read manifest, read handoff, retire, supersede, version list, and package status/timeline APIs with signed envelopes, idempotency keys, trace ids, policy refs, stable errors, and redacted views.
  - Output: API handlers, status projection, timeline projection, filters, pagination, error examples, and event projection.
  - Validation: Tests prove APIs enforce auth, scope, schema version, idempotency, redaction, stable reason codes, and immutable-version rules.

- **9.2 Implement CLI, SDK, and Admin UI package surfaces.**
  - Design: Add generated client operations for draft package creation, contract updates, submission, handoff reads, package diff, supersede, retire, revoke status, version list, and readiness summaries.
  - Output: CLI command contracts, SDK bindings, Admin UI view contracts, stable JSON output, error examples, and trace propagation.
  - Validation: Contract tests prove client commands pass signed envelopes, trace ids, idempotency keys, schema versions, policy refs, stable reason codes, and redaction rules through generated contracts.

- **9.3 Emit system-service packaging usage facts.**
  - Design: Emit package build, validation handoff, registry publication, diff, supersede, retire, revoke, and package-handoff usage events where material through Overmeter and accounting hooks without direct balance or ledger mutation.
  - Output: Usage event schema, Overmeter handoff refs, ORU/Overbill/Seal Ledger refs where available, resource dimensions, and public-reporting classification.
  - Validation: Tests prove package operations distinguish backbone operating cost from user workload usage and never create pricing, revenue, balance, or ledger mutations directly.

- **9.4 Feed deployment, release, backup, failover, incident, and stewardship owners.**
  - Design: Provide package refs, manifest hashes, command refs, compatibility refs, validation refs, revocation refs, diff refs, and handoff gaps to Deployment Planner, Release Strategy Service, Backup and Restore Service, Failover and Recovery Coordinator, Incident Response Service, Stewardship Reporting Service, Overwatch, and operator tooling.
  - Output: Owner-service event contracts, handoff APIs, incident evidence bundle, report refs, deployment/release/restore/failover handoff refs, and follow-up backlog entries.
  - Validation: Integration tests prove owner services consume refs through APIs and events, not packager-owned private records, and can reject or block package progress with explicit reason codes.

- **9.5 Harden redaction, audit, and operator diagnostics.**
  - Design: Redact private route details, secret names beyond allowed scope, protected topology, security-sensitive dependency details, raw health content, incident-sensitive refs, and cross-tenant facts from user-visible responses while giving authorized operators signed diagnostic views.
  - Output: Redaction profile, role-gated operator diagnostics, audit export schema, break-glass view policy, and security review checklist.
  - Validation: Tests prove unauthorized views cannot access protected topology, secret details, private evidence, raw package internals, or cross-tenant data, and every operator view emits Overwatch audit refs.

## Phase 10: Validation, Security Review, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, packager-owned-deployment, packager-owned-release-strategy, packager-owned-validation, packager-owned-backup-storage, packager-owned-workload-execution, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools, local stubs, or downstream owner services into the packager's product boundary.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated backlinks and index rows for SDS #48.
  - Validation: Local link checks pass and Docdex search returns the SDS, service plan, crosswalk row, and this sub-build plan in aligned results.

- **10.4 Validate queue state and progress evidence.**
  - Design: Mark `048-build-plan` complete in the Codex55 queue, update `.codex55_sds_queue/progress.md`, append validation evidence to `docs/build_plan/progress.md`, and preserve no running materialized tasks after this queue item.
  - Output: Updated queue JSON, queue progress summary, and build-plan progress notes.
  - Validation: JSON validation passes; queue validation confirms `048-build-plan` is complete, no materialized task is running, and the next materialized build-plan task can be created by the queue runner.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders source alignment, contracts, artifact/provenance/config/secret authoring, command/privilege/compatibility gates, first non-critical package, validator/registry handoffs, lifecycle/revocation flows, migration sequence, operational interfaces, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.
