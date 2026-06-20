# SUB BUILD PLAN #49 - Package Validator

Attached SDS: [docs/sds/deployment_grid/package_validator.md](../sds/deployment_grid/package_validator.md)

## Purpose

This sub-build plan turns SDS #49 into an implementation sequence for Package Validator. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Package Validator is the deterministic validation-report boundary for workload packages, system-service packages, application-intent manifests, native-app packages, and AI-generated package proposals. It owns validation requests, immutable validation reports, check results, validation profiles, rulesets, policy-compatibility result objects, permission/provenance findings, replay inputs, report diffs, ruleset deprecation behavior, stable reason codes, remediation hints, and evidence refs. It does not execute workloads, build packages, choose placement, deploy apps, choose release strategy, approve final policy admission, store canonical artifacts, store raw secrets, or mutate billing, ORU, payout, or Seal Ledger state.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #49: Package Validator](../sds/deployment_grid/package_validator.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Package Validator service plan](../service_catalog/deployment_grid/package_validator.md) | Controls the service-catalog objective, first build phases, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, stable reason-code discipline, signed envelopes, trace ids, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate ingress, Overpass identities, Overtenant scope, Overkey key/signature refs, Overregistry package/schema refs, Overwatch audit, and Overqueue state prerequisites. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Controls the first build point: workload package validation before Overrun execution, including schema/profile version, signatures, hashes, runtime compatibility, permissions, no raw secrets, dependency evidence, and basic policy facts. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard decisions, Policy Dry-Run previews, workload/data class facts, signer/policy compatibility facts, replayable denials, and side-effect-free admission previews. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies validation usage dimensions, budget precheck refs, accounting evidence refs, and cost visibility without pricing, revenue, or direct balance mutation. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies system-service package validation expansion for command contracts, health/readiness, backup/restore/rollback/drain, privileged signers, node class, compatibility windows, and failover handoff checks. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies later Overbase, Overstore, Overvault, Universal Namespace Service, and Overmesh refs that application and system-service packages can validate without the validator owning those services' truth. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies application-intent validation, app route/data/storage/vault/provisioning compatibility, budget/billing hook compatibility, release health gates, AI-generated package diffs, and deployment-wide policy previews. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies incident response, threat modeling, audit export, compliance retention, ruleset governance, report retention, redaction, and security review hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #49 first build work aligned to master Phase 3, with Phase 7 system-service validation, Phase 9 app validation, and Phase 13 governance hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 7, 9, and 13 | Attach SDS #49, freeze validator authority boundaries, preserve Phase 3 as first build, and record Phase 7, Phase 9, and Phase 13 expansion gates. |
| 2 | Master Phases 0, 1, 3, 4, and 13 | Define Rust contracts, canonical schemas, validation profiles, rulesets, reason codes, immutable report semantics, and fixtures. |
| 3 | Master Phases 0, 1, 3, 4, 8, and 9 | Implement immutable input snapshots, package refs, manifest/schema checks, artifact hashes, signatures, provenance, dependency inventory, SBOM, and redacted evidence refs. |
| 4 | Master Phases 3, 4, 8, and 9 | Implement runtime compatibility, permission minimization, secret-ref validation, policy compatibility facts, and bounded side-effect-free inspection. |
| 5 | Master Phases 1, 3, 4, 5, and 6 | Build the Phase 3 workload-validation API and report path consumed by Overrun, SDK, CLI, Admin UI, Overregistry, Overwatch, and Overmeter. |
| 6 | Master Phases 4, 7, 8, 9, and 13 | Add the Phase 7 system-service package profile for command contracts, privileged signers, backup/restore/rollback/drain, node class, compatibility windows, and failover readiness. |
| 7 | Master Phases 4, 5, 8, 9, 11, and 13 | Add the Phase 9 application-intent and AI-generated package validation profiles, including deployment, route, budget, release, and public-provider compatibility checks. |
| 8 | Master Phases 3, 7, 9, and 13 | Implement replay, report diff, ruleset diff, ruleset deprecation, retention classes, incident pins, and audit comparison behavior. |
| 9 | Master Phases 5, 6, 9, 12, and 13 | Add operational APIs, client surfaces, observability, metering handoffs, redaction, diagnostics, and downstream owner-service handoffs. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Package Validator core is a Rust service/module using shared contract crates, Tokio for bounded workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Validation requests, input snapshots, validation reports, check results, rulesets, policy-compatibility results, permission findings, provenance findings, replay bundles, report diffs, ruleset diffs, API objects, event payloads, fixtures, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed actor or service-account envelopes, tenant/system scope, trace id, idempotency key, package hash, validation profile, schema version, policy refs, stable reason codes, and append-only Overwatch evidence refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for package hashes, manifest refs, artifact refs, dependency inventories, SBOM refs, validation reports, replay inputs, report diffs, and deterministic fixture comparison.
- Dependency evidence starts with Overpack-native canonical JSON/JSON Schema `dependency_inventory_v0`; SPDX, CycloneDX, or external scanners can be normalized inputs later but must not become the product boundary.
- Package Validator may point to Overpack, Overregistry, Overrun, Overcell, Overguard, Policy Dry-Run API, Workload Classifier, Overstore, Overvault, Overwatch, Overmeter, ORU, Overbill, Seal Ledger, Grid-Resident Service Packager, Deployment Planner, Release Strategy Service, Backup and Restore Service, Failover and Recovery Coordinator, Incident Response Service, SDK, CLI, Admin UI, native apps, and AI-generated package tools, but the validator must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, revenue projections, customer-count assumptions, raw secret storage, package building, workload execution, final policy admission, deployment planning, release strategy, billing mutation, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Validator Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #49.**
  - Design: Link this document from the Package Validator SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/deployment_grid/package_validator.md`, `docs/service_catalog/deployment_grid/package_validator.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #49 returns both the Package Validator SDS and this sub-build plan.

- **1.2 Freeze validator authority boundaries.**
  - Design: Record that Package Validator owns deterministic validation requests, input snapshots, immutable reports, check results, profile/ruleset behavior, policy compatibility facts, permission/provenance findings, stable reason codes, remediation hints, replay inputs, and report diffs.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms Package Validator does not execute workloads, deploy applications, build packages, choose placement, choose release strategy, approve final policy admission, own artifact storage, store raw secrets, mutate accounting, or bypass Overguard and owner-service APIs.

- **1.3 Preserve Phase 3 as the first build point.**
  - Design: Keep first implementation in master Phase 3 because Overrun needs workload-package validation before execution can safely start.
  - Output: Phase-gate note that Phase 3 builds workload checks, Phase 7 adds system-service promotion checks, Phase 9 adds application-intent and AI-generated package checks, and Phase 13 hardens governance and retention.
  - Validation: Review proves this plan does not defer signatures, hashes, runtime-contract checks, raw-secret rejection, basic permission facts, or policy compatibility facts to Phase 9.

- **1.4 Carry forward resolved SDS #49 decisions.**
  - Design: Preserve Phase 3 blocking checks, Overpack-native `dependency_inventory_v0`, bounded dynamic inspection only, risk-grouped AI-generated package diffs, and report retention classes from the SDS open-question answers.
  - Output: Resolved-decision checklist tied to validation profiles, dependency evidence, sandbox limits, AI diff views, and retention policy.
  - Validation: Review rejects external scanner product-boundary assumptions, unbounded package execution, raw file diffs as the primary reviewer view, and short retention for reports that gate system-service or production deployment decisions.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Overpack, Overregistry, Overrun, Overcell, Overguard, Policy Dry-Run API, Workload Classifier, Overstore, Overvault, Overwatch, Overmeter, ORU Account Service, Overbill, Seal Ledger, Grid-Resident Service Packager, Deployment Planner, Release Strategy Service, Backup and Restore Service, Failover and Recovery Coordinator, SDK, CLI, Admin UI, native apps, and AI package tools.
  - Output: Boundary matrix listing owner, input refs, output refs, validation facts, freshness rules, profile/ruleset versions, policy refs, audit refs, redaction class, and downstream consumer.
  - Validation: Review confirms every handoff uses explicit APIs, immutable refs, signed evidence, reason codes, trace ids, idempotency keys, policy refs, and Overwatch events rather than privileged shared records or hidden control paths.

## Phase 2: Rust Contracts, Schemas, Rulesets, And Fixtures

### Work Items

- **2.1 Create the Package Validator Rust contract module.**
  - Design: Add contract types for validation requests, input snapshots, validation reports, check results, policy-compatibility results, permission findings, provenance findings, rulesets, validation profiles, report lifecycles, errors, events, redaction profiles, replay inputs, and audit exports.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, profile enums, lifecycle enums, severity enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overpack, Overrun, Overguard, Deployment Planner, Release Strategy Service, Overstore, Overvault, Overbill, and Seal Ledger internals.

- **2.2 Define validation request, snapshot, report, and check schemas.**
  - Design: Model `validation_request`, `validation_input_snapshot`, `validation_report`, and `validation_check_result` with tenant scope, actor/service scope, package ref/hash, package type, validation profile, ruleset version, policy context ref, input refs, state, pass/warn/fail status, severity, blocking flag, remediation hint, and audit refs.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and report hash fixtures.
  - Validation: Schema tests reject missing tenant, actor/service scope, package hash, validation profile, ruleset version, trace id, idempotency key, state, reason code, severity, blocking flag, audit refs, or package ref.

- **2.3 Define validation profiles and rulesets.**
  - Design: Model workload, system-service, application, native-app, AI-generated, replay-only, and future public-provider profiles with ordered checks, severity mapping, compatibility windows, deprecation state, and profile-specific required evidence.
  - Output: `validator_ruleset` schema, profile matrix, check-order registry, severity policy, compatibility-window model, deprecation state machine, and negative fixtures.
  - Validation: Tests prove profile changes create new ruleset versions, deprecated rulesets are blocked for new validation except replay, and replay uses the original ruleset version without silently upgrading outcomes.

- **2.4 Define stable reason codes and remediation hints.**
  - Design: Normalize schema, signature, signer, artifact, dependency, SBOM, runtime, permission, policy, secret-ref, sandbox, ruleset, and replay errors into stable machine-readable codes with safe remediation text.
  - Output: Reason-code registry, remediation-hint catalog, localization-safe message keys, CLI/SDK/Admin UI mapping, and compatibility tests.
  - Validation: Tests prove automation can branch on stable codes and tenant-facing text does not leak private package contents, topology, secret names beyond allowed scope, raw policy traces, or cross-tenant facts.

- **2.5 Create deterministic validator fixtures.**
  - Design: Build fixtures for valid workload, invalid schema, hash mismatch, invalid signature, unauthorized signer, missing dependency inventory, missing SBOM, runtime mismatch, raw secret, broad permission, policy incompatible, policy preview missing, sandbox timeout, ruleset deprecated, system-service missing backup/restore, app route conflict, and AI privilege expansion.
  - Output: Fixture directory, canonical inputs, expected reports, expected check lists, expected reason codes, report hashes, replay bundles, and redacted views.
  - Validation: Fixture tests produce stable ids, BLAKE3 hashes, validation results, reason codes, report hashes, audit refs, and redacted projections across repeated runs.

## Phase 3: Input Snapshots, Integrity, Signatures, And Provenance

### Work Items

- **3.1 Implement immutable input snapshot assembly.**
  - Design: Fetch and freeze manifest refs, artifact refs, registry facts, signer facts, SBOM refs, dependency locks, runtime-card refs, secret refs, classifier facts, policy context refs, package hash, and trace refs before checks run.
  - Output: Input snapshot builder, snapshot hash, input availability checks, redacted snapshot projection, fetch-failure reasons, and `package_validator.validation_started` events.
  - Validation: Tests prove changed package refs, changed registry facts, missing inputs, unavailable artifacts, and mismatched package hashes produce new or failed validation states rather than mutating completed reports.

- **3.2 Implement manifest schema and profile validation.**
  - Design: Validate package envelope, manifest kind, schema version, package type, validation profile, required fields, compatibility window, and unknown execution-affecting fields before deeper checks.
  - Output: Schema validator, profile selector, compatibility-window checker, unknown-field policy, invalid-manifest report builder, and reason-code mappings.
  - Validation: Tests prove invalid schema, unsupported schema version, wrong package type/profile pairing, missing compatibility window, and unknown execution-affecting fields fail before runtime, policy, or deployment checks.

- **3.3 Implement artifact hash, signature, and signer authority checks.**
  - Design: Verify BLAKE3/content hashes, manifest/package signatures, Ed25519 signer authority refs, signer revocation state, key rotation metadata, artifact availability, and signature coverage over canonical payloads.
  - Output: Hash verifier, signature verifier, signer-authority adapter, artifact-availability checker, canonicalization vectors, and security reason codes.
  - Validation: Tests prove hash mismatch, mutable artifact refs, partial signature coverage, revoked keys, wrong tenant/service signer, missing artifact refs, and signer authority mismatch block validation before downstream services act.

- **3.4 Implement dependency inventory, lock, and SBOM checks.**
  - Design: Validate Overpack-native `dependency_inventory_v0`, dependency lock refs, immutable dependency digests, optional CycloneDX or SPDX normalized refs, base image/module digest refs, and explicit `no_external_dependencies` attestations.
  - Output: Dependency evidence parser, lock checker, SBOM ref checker, inventory normalization, profile-specific severity mapping, and invalid dependency fixtures.
  - Validation: Tests prove Phase 3 package-managed and container workloads block on missing locks or missing inventory, optional unknown metadata warns under private-grid profiles only, and external SBOM/scanner formats are normalized inputs rather than product boundaries.

- **3.5 Implement provenance and redacted evidence refs.**
  - Design: Record source/build refs, builder identity, generated-by refs, runtime-card refs, artifact refs, dependency refs, signer refs, policy refs, and Overwatch refs without retaining private source snippets as report payloads.
  - Output: Provenance finding schema, redaction rules, source/build ref projection, generated-by projection, evidence-ref registry, and provenance warning codes.
  - Validation: Tests prove operators can answer what was validated, by whom, from which artifact and dependency facts, while tenant/public views hide private source contents, protected topology, raw package internals, and sensitive evidence.

## Phase 4: Runtime, Permission, Secret, Policy, And Sandbox Checks

### Work Items

- **4.1 Implement Overrun and Overcell runtime-contract compatibility.**
  - Design: Validate runtime cards, entrypoints, command/OCI/WASI support, timeouts, retry/cleanup policies, resource cards, accelerator requirements, mount rules, and package type compatibility against Overrun and Overcell constraints.
  - Output: Runtime compatibility checker, expected runtime-card constraints, mismatch reason codes, profile-specific blocking behavior, and runtime fixtures.
  - Validation: Tests prove unsupported runtime, untested WASI, hidden shell execution, missing entrypoint, unbounded timeout, unsafe mounts, missing resource cards, and undeclared accelerator needs block execution-ready profiles.

- **4.2 Implement permission, egress, data-class, storage, route, and vault-scope findings.**
  - Design: Compare declared permissions against inferred or observed package needs, data class, egress scope, storage scope, route scope, vault scope, service account scope, and minimization rules.
  - Output: Permission finding model, minimization hints, high-risk capability flags, declared-vs-inferred summary, and stable risk reason codes.
  - Validation: Tests prove raw broad permissions, missing denial/default behavior, undeclared egress, privileged storage/route/vault scope, and data-class conflicts fail or warn according to profile severity.

- **4.3 Implement secret-reference validation and raw-secret rejection.**
  - Design: Validate Overvault secret refs, required secret declarations, mount/env refs, vault scope, rotation support, least-privilege scope, and redaction behavior while rejecting raw secret material in manifests, reports, fixtures, and logs.
  - Output: Secret-ref checker, raw-secret detector, redaction profile, vault-scope mismatch reason codes, and secret-ref fixtures.
  - Validation: Tests prove raw secrets are rejected before validation passes, redacted reports never expose protected values, missing required vault refs block relevant profiles, and the validator does not become a vault or secret owner.

- **4.4 Implement Overguard and Policy Dry-Run compatibility facts.**
  - Design: Produce side-effect-free policy compatibility input facts and consume Overguard or Policy Dry-Run previews without converting validation success into final policy admission.
  - Output: `policy_compatibility_result`, missing-prerequisite refs, matched/missing policy fact summaries, data-class conflict summaries, preview freshness rules, and `policy_preview_missing` behavior.
  - Validation: Tests prove policy denial, missing policy facts, stale preview, unavailable policy preview, and incompatible data/permission facts produce replayable report states while final admission remains owned by Overguard.

- **4.5 Implement bounded inspection sandbox checks.**
  - Design: Allow only validator-owned, side-effect-free inspection: archive expansion with path/size/count/time limits, manifest parsing, hash/signature verification, OCI/WASI metadata inspection, dependency/SBOM consistency, runtime-card compatibility probing against throwaway no-network/no-secret fixtures, and safe health/readiness shape checks.
  - Output: Sandbox policy, inspection runner, timeout model, scratch cleanup behavior, no-network/no-secret guardrails, and sandbox failure reason codes.
  - Validation: Tests prove package application logic, migrations, network calls, real secret mounts, route changes, storage writes, billing/accounting actions, and workload execution never run inside validation.

## Phase 5: Phase 3 Workload Validation Path And Reports

### Work Items

- **5.1 Implement workload validation APIs.**
  - Design: Add `POST /package-validations`, `GET /package-validations/{request_id}`, `GET /validation-reports/{report_id}`, `GET /validation-reports/{report_id}/checks`, `GET /validator-rulesets`, and replay endpoints for Phase 3 workload packages.
  - Output: API handlers, request/response schemas, idempotency behavior, pagination/filter rules, auth/scope checks, stable error format, and event emissions.
  - Validation: API tests cover valid validation, invalid schema, duplicate idempotency keys, missing package hash, wrong tenant, unsupported profile, unauthorized read, report lookup, check lookup, and ruleset lookup.

- **5.2 Enforce Phase 3 execution-blocking checks.**
  - Design: Require valid Overpack workload schema/profile version, immutable package hash, BLAKE3 artifact hashes, Overkey Ed25519 signature, signer authority, artifact availability, narrow Overrun/Overcell runtime-card compatibility, explicit resource card, timeout/retry policy, data sensitivity, egress shape, secret policy shape, no raw secrets, minimum dependency evidence, and side-effect-free policy facts.
  - Output: Workload profile checker, blocking error report, warning report, profile severity table, and execution-ready gate output.
  - Validation: Tests prove signatures, hashes, runtime contracts, raw-secret rejection, and basic policy facts cannot be postponed to Phase 7 or Phase 9 and invalid workloads fail before Overrun receives executable work.

- **5.3 Produce SDK/CLI/Admin UI-friendly reports.**
  - Design: Return compact reports with pass/fail/warn state, stable reason codes, remediation hints, severity, blocking flags, profile/ruleset versions, input refs, evidence refs, redacted package facts, and check ordering.
  - Output: Report read model, check read model, CLI/SDK output fixtures, Admin UI view contract, reason-code docs, and remediation examples.
  - Validation: Contract tests prove clients can present actionable errors without parsing raw private package payloads, raw policy traces, protected topology, or secret material.

- **5.4 Hand off validation evidence to Overrun, Overregistry, and Overwatch.**
  - Design: Provide validation report refs, package hash, ruleset version, signer refs, reason codes, and blocking state to Overrun execution preflight, Overregistry accepted package records, and Overwatch evidence trails.
  - Output: Consumer contract, event schema, registry adapter shape, runner preflight refs, audit refs, and retry/idempotency behavior.
  - Validation: Integration tests prove Overrun rejects missing/stale/failing reports, Overregistry records immutable validation refs, and Overwatch audit events include enough evidence for replay without exposing private report internals.

- **5.5 Emit validation usage and budget-precheck facts without billing mutation.**
  - Design: Emit validation CPU, storage, artifact-transfer, sandbox, queue, and operator-attention dimensions to Overmeter where material, and request ORU budget prechecks for expensive validation profiles before deep scanning.
  - Output: Usage event schema, budget precheck ref, Overmeter handoff, resource dimension mapping, and billing-boundary notes.
  - Validation: Tests prove validation usage never creates pricing, revenue, balance transitions, invoices, payouts, or Seal Ledger entries directly; accounting owners consume usage facts later.

## Phase 6: Phase 7 System-Service Promotion Validation

### Work Items

- **6.1 Define the system-service validation profile.**
  - Design: Extend validation profiles for Phase 7 system-service package promotion with package type, service id, workload class, privileged signer, node-class eligibility, compatibility windows, backup/restore/rollback/drain requirements, and failover handoff checks.
  - Output: System-service profile schema, severity matrix, eligibility output, profile fixtures, and promotion blocker reason codes.
  - Validation: Tests prove system-service packages cannot pass with ordinary workload-only profiles, missing privileged signer refs, unknown node-class eligibility, stale ruleset versions, or missing service/package identity.

- **6.2 Validate command contracts for system services.**
  - Design: Check health, readiness, diagnostics, pre-stop, drain, migrate, backup, restore, rollback, verify, reconcile, and state ownership contracts according to service statefulness.
  - Output: Command-contract checker, stateless `no_state` reason handling, stateful command requirements, expected output schemas, timeout/idempotency checks, and command fixtures.
  - Validation: Tests prove fake no-op state commands without explicit `no_state` reasons fail, stateful services require restore/rollback/fence/verify/reconcile contracts, and missing health/readiness blocks promotion.

- **6.3 Validate backup, restore, rollback, and failover readiness.**
  - Design: Require Backup and Restore Service refs, restore drill refs where needed, rollback package refs, writer/fence prerequisites, drain behavior, Failover and Recovery Coordinator handoff refs, and compatibility with current package version.
  - Output: Recovery-readiness checker, restore/rollback/failover fact bundle, promotion gap report, and founder-exit blocker refs.
  - Validation: Tests prove packages missing backup/restore/rollback/drain/failover evidence are blocked before grid-resident promotion and founder-hardware removal gates cannot pass on validation reports alone.

- **6.4 Validate privilege profiles and compatibility windows.**
  - Design: Check declared capabilities, network/storage/route/vault scopes, service accounts, data classes, break-glass rules, system-service workload-class version, validation ruleset version, runtime version, state version, config migration behavior, and supported upgrade/rollback window.
  - Output: Privilege and compatibility checker, least-privilege findings, compatibility matrix, profile diff, and denial reason catalog.
  - Validation: Tests prove too-broad privilege profiles, missing class refs, wrong service-account scopes, incompatible state/runtime/config/ruleset windows, and missing denial behavior block release eligibility.

- **6.5 Hand off system-service validation to deployment-grid owners.**
  - Design: Provide report refs to Grid-Resident Service Packager, Deployment Planner, Release Strategy Service, Backup and Restore Service, Failover and Recovery Coordinator, Overregistry, Overwatch, and operator tooling without owning their decisions.
  - Output: Handoff contract, report freshness rule, downstream-specific projection, redacted evidence refs, and owner-service event payloads.
  - Validation: Integration tests prove owner services consume report refs through APIs/events, can reject or block promotion with their own reason codes, and never rely on Package Validator as the deployment, release, backup, or failover owner.

## Phase 7: Phase 9 Application-Intent And AI Package Validation

### Work Items

- **7.1 Define the application-intent validation profile.**
  - Design: Validate app identity, services, runtime cards, data needs, storage needs, vault needs, model needs, permissions, budget refs, billing-rule refs, routes, geography, scaling, health, observability, rollback intent, and deployment compatibility refs.
  - Output: App validation profile schema, app package fixtures, profile severity mapping, blocked-state reasons, and compatibility output.
  - Validation: Tests prove app validation expresses compatibility facts for Deployment Planner and Release Strategy Service without provisioning infrastructure, activating routes, or choosing rollout strategy.

- **7.2 Validate data, storage, vault, namespace, and route compatibility.**
  - Design: Check Overbase collection refs, Overstore artifact/object refs, Overvault grant refs, Universal Namespace ownership refs, Overmesh route refs, route ownership, storage retention, backup expectations, and private data needs.
  - Output: Owner-service compatibility fact bundle, missing prerequisite summary, route/vault/storage/data denial reasons, and redacted ref summaries.
  - Validation: Tests prove missing owner-service facts block app validation without moving Overbase, Overstore, Overvault, namespace, route, or mesh authority into Package Validator.

- **7.3 Validate budget, billing hook, release, and health-gate compatibility.**
  - Design: Check ORU budget precheck refs, Overbill hook compatibility, resource-card/rate-band refs, deployment health checks, release strategy refs, canary/blue-green/rolling compatibility, rollback behavior, and route activation readiness.
  - Output: Budget/release compatibility result, health-gate fact bundle, billing hook compatibility refs, strategy compatibility refs, and denial reason codes.
  - Validation: Tests prove Package Validator can warn or block on missing compatibility facts while Overbill, ORU Account Service, Overmark, Deployment Planner, and Release Strategy Service retain final owner authority.

- **7.4 Implement risk-grouped AI-generated package diffs.**
  - Design: Compare previous versus proposed canonical package facts by blocking errors, warnings, privilege expansions, egress, Overvault scopes, data classes, route/budget changes, runtime changes, dependency/provenance changes, generated-by refs, policy-preview deltas, and remediation codes.
  - Output: Machine-readable diff, compact reviewer view, risk grouping, check-id links, evidence-ref links, redaction rules, and CLI/SDK/Admin UI fixtures.
  - Validation: Tests prove AI-generated packages pass the same schema, signature, provenance, validation, policy, budget, and permission checks as human-authored packages and cannot use raw file diffs to hide risky manifest changes.

- **7.5 Prepare public-provider and native-app validation constraints.**
  - Design: Add future-compatible validation facts for low-sensitivity public-provider packages, native app packages, mobile clients, personal AI surfaces, public sandbox profiles, challenge refs, fraud-control refs, reputation refs, payout-hold refs, and purpose tags.
  - Output: Public/native constraint fields, blocked-state fixtures, redacted client views, and follow-up backlog entries.
  - Validation: Review confirms sensitive, secret-bearing, private-data, privileged-runtime, or unknown-provider packages remain blocked until owning policy, verification, sandbox, fraud, and public-provider services allow them.

## Phase 8: Replay, Diff, Ruleset Deprecation, And Retention

### Work Items

- **8.1 Implement validation replay from original refs.**
  - Design: Re-run validation with original input refs, package hash, ruleset version, profile, policy context refs, registry facts, signer facts, dependency facts, and sandbox limits for audit comparison.
  - Output: Replay API, replay bundle schema, comparison result, report hash comparison, and `package_validator.replay_completed` events.
  - Validation: Tests prove replay returns the same check results for unchanged refs and clearly marks differences caused by changed package refs, registry facts, policy context, signer state, dependency facts, or ruleset versions.

- **8.2 Implement report and ruleset diff.**
  - Design: Compare two reports or two rulesets by check order, severity mapping, blocking behavior, policy facts, permission/provenance findings, profile requirements, input refs, and resulting pass/fail/warn state.
  - Output: Report diff schema, ruleset diff schema, risk grouping, redacted reviewer view, and machine-readable output.
  - Validation: Tests prove diffs are deterministic, redacted, stable across repeated runs, and usable by CLI, SDK, Admin UI, incident response, and governance review.

- **8.3 Implement ruleset deprecation and compatibility windows.**
  - Design: Mark rulesets as supported, deprecated, blocked-for-new-validation, replay-only, or superseded while preserving old report readability and affected-package lookup.
  - Output: Ruleset lifecycle state machine, compatibility-window model, affected-report query, notification event, and remediation guidance.
  - Validation: Tests prove deprecated rulesets cannot be used for new validation except replay, downstream services can require fresh reports when affected, and old reports remain readable for audit.

- **8.4 Implement report retention classes.**
  - Design: Retain normal Phase 3 private-workload reports, check summaries, ruleset refs, package hashes, signer refs, and redacted evidence refs for at least 180 days; retain Phase 7 system-service, Phase 9 production app, route activation, rollback, revocation, public-provider, and accounting-impact reports for seven years through Overwatch-compatible evidence bundles.
  - Output: Retention policy schema, retention classifier, redaction behavior, expiry scheduler contract, and audit export refs.
  - Validation: Tests prove retention classes match SDS #49 and raw extracted package contents, sandbox scratch data, private source snippets, and sensitive topology are not retained as report payloads beyond short operational windows.

- **8.5 Implement incident, dispute, compliance, and security pins.**
  - Design: Pin relevant report refs until case closure plus applicable audit window when Overclaim dispute, Incident Response, Compliance Boundary, Stewardship Reporting, PIP review, or security investigation refs require preservation.
  - Output: Evidence pin model, incident/dispute/compliance refs, audit export schema, pin lifecycle events, and release/unpin behavior.
  - Validation: Tests prove pinned reports remain replayable and redacted, raw sensitive payloads are excluded unless authorized owner refs preserve them, and unpin behavior cannot delete required audit evidence early.

## Phase 9: APIs, Interfaces, Operations, And Downstream Handoffs

### Work Items

- **9.1 Harden validator API surfaces and projections.**
  - Design: Expand read, search, replay, diff, ruleset, profile, check-result, report-status, timeline, and operator diagnostic APIs with signed scopes, idempotency, trace ids, schema versions, pagination, filters, and redacted outputs.
  - Output: API handlers, query projections, timeline projection, status projection, filters, pagination, stable errors, and OpenAPI/contract examples.
  - Validation: API tests prove auth, tenant/system scope, idempotency, redaction, stable reason codes, immutable report behavior, replay authorization, and report-search filtering work as documented.

- **9.2 Implement CLI, SDK, Admin UI, and AI-tool surfaces.**
  - Design: Add generated client operations for submit validation, read status, read report, list checks, replay, diff, list rulesets, show remediation hints, inspect AI package risk groups, and request policy preview.
  - Output: CLI command contracts, SDK bindings, Admin UI view contracts, AI-tool read contracts, stable JSON output, exit-code rules, error examples, and trace propagation.
  - Validation: Contract tests prove clients pass signed envelopes, trace ids, idempotency keys, schema versions, policy refs, stable reason codes, and redaction rules through generated contracts.

- **9.3 Implement operational monitoring and diagnostics.**
  - Design: Track health, queue depth, validation latency by profile, ruleset adoption, report pass/fail/warn counts, sandbox failures, artifact-fetch failures, policy-preview failures, most common blocking errors, and stuck validations.
  - Output: Health endpoint, metrics/events, operator diagnostics, stuck-validation query, failure-reason dashboard contract, and Overwatch event refs.
  - Validation: Tests prove diagnostic views require authorized operator scope, tenant-facing views are redacted, and stuck validation recovery emits reason-coded evidence.

- **9.4 Emit validation usage and owner-service events.**
  - Design: Emit events for requested, started, check completed, passed, failed, warned, policy preview completed, ruleset deprecated, replay completed, retention pinned, and report diff generated; hand usage facts to Overmeter where material.
  - Output: Event schemas, Overmeter handoff refs, Overwatch event refs, resource dimensions, retry/idempotency behavior, and redaction profiles.
  - Validation: Integration tests prove downstream owners consume refs through APIs/events and validation usage never mutates ORU, Overbill, Seal Ledger, payout, route, package, deployment, release, backup, or failover truth directly.

- **9.5 Harden redaction, report sharing, and operator diagnostics.**
  - Design: Redact package contents, private source refs, protected topology, secret refs beyond allowed scope, raw policy traces, sensitive dependency details, internal route data, and cross-tenant facts while giving authorized operators signed diagnostic views.
  - Output: Redaction profile, role-gated operator view, tenant-safe report view, public-safe summary, audit export schema, and break-glass view policy.
  - Validation: Tests prove unauthorized views cannot access protected content and every operator diagnostic or break-glass view emits Overwatch audit refs with reason codes.

## Phase 10: Validation, Security Review, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack and authority guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, validator-owned-execution, validator-owned-deployment, validator-owned-release-strategy, validator-owned-policy-admission, validator-owned-accounting, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools, normalized input formats, local stubs, or downstream owner services into Package Validator's product boundary.

- **10.3 Validate workload, system-service, app, replay, and retention coverage.**
  - Design: Run focused checks for Phase 3 workload validation, Phase 7 system-service validation, Phase 9 app/AI validation, replay/diff/ruleset deprecation, report retention, redaction, and downstream handoffs.
  - Output: Coverage matrix, fixture checklist, profile/ruleset checklist, retention checklist, and integration-test targets.
  - Validation: Review proves each SDS #49 build-breakdown item maps to at least one sub-build phase and no required Phase 3 blocker is deferred to later app-deployment work.

- **10.4 Validate SDS, service catalog, master plan, crosswalk, and queue alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, build-plan crosswalk, Phase 7 wording, queue state, queue progress, and build-plan progress.
  - Output: Updated backlinks and index rows for SDS #49, queue state update, queue progress update, and build-plan progress evidence.
  - Validation: JSON validation passes; local link checks pass; queue validation confirms `049-build-plan` is complete, no materialized task is running, and `050-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders source alignment, contracts, snapshots, integrity/signature/provenance, runtime/permission/policy/sandbox checks, Phase 3 workload reports, Phase 7 system-service promotion checks, Phase 9 app/AI validation, replay/diff/retention, operations/interfaces, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.

## Alignment Review

- The sub-build plan keeps Package Validator first build work in master Phase 3 because Overrun must reject unsafe workload packages before private execution.
- The plan adds Phase 7 system-service validation as an expansion for grid-resident promotion, matching SDS #49, SDS #48, the Phase 7 service-packaging workstream, and the deployment-grid service catalog.
- The plan keeps application-intent manifest validation, route/provisioning/budget/release compatibility, and AI-generated package diffs in master Phase 9, matching Overpack and Deployment Planner boundaries.
- The plan treats Overguard and Policy Dry-Run API as policy owners; Package Validator produces compatibility facts and report evidence, not final policy admission.
- The plan treats Overstore and Overvault as artifact/secret ref owners; Package Validator validates refs and redacts evidence, not raw object bytes or secret values.
- The plan treats ORU, Overbill, Overmeter, and Seal Ledger as accounting owners; Package Validator emits usage dimensions and budget precheck refs without pricing, revenue, balance, payout, or ledger mutation.
- The plan preserves master Phase 0 through Phase 13 ordering and uses later phases only for system-service, application-deployment, public-provider/native-app, and governance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service/contracts, native Overrid boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.
