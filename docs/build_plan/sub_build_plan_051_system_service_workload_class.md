# SUB BUILD PLAN #51 - System-Service Workload Class

Attached SDS: [docs/sds/deployment_grid/system_service_workload_class.md](../sds/deployment_grid/system_service_workload_class.md)

## Purpose

This sub-build plan turns SDS #51 into an implementation sequence for System-Service Workload Class. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

System-Service Workload Class is the versioned eligibility and guardrail contract for protected Overrid backbone placement. It owns class definitions, eligible service records, node eligibility requirements, placement guardrails, operational requirement bundles, signed operator action requirements, evaluation snapshots, and override records. It does not schedule workloads, issue leases, build packages, validate package internals, deploy services, choose release strategies, execute backups, perform restores, or perform live failover.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #51: System-Service Workload Class](../sds/deployment_grid/system_service_workload_class.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [System-Service Workload Class service plan](../service_catalog/deployment_grid/system_service_workload_class.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant/system scope, Overkey signatures, Overgate ingress, Overregistry service/package facts, Overwatch audit, and Overqueue state prerequisites. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies Overcell node enrollment, Hardware Discovery, Benchmark Runner capacity refs, command acceptance, heartbeat evidence, and private node identity prerequisites. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack workload/package refs, Oversched placement consumers, Overlease execution eligibility, Overrun execution state, and Overmeter raw usage facts. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Oververify node/operator evidence, Challenge Task Service checks, Overclaim disputes, and Overmesh private routing foundations. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies system-service usage dimensions, Overmeter events, ORU/Seal Ledger/Overbill accounting evidence refs, and stewardship cost visibility without accounting mutation by this service. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Controls the first build point: protected workload class, trusted placement rules, operational controls, signed operator actions, first non-critical migration, and founder-hardware removal gates. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase, Overstore, Overvault, Universal Namespace Service, and Overmesh refs that class rules may require but do not own. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies Deployment Planner, Release Strategy Service, Package Validator, and shared Overpack deployment handoffs that must consume evaluation refs rather than copy class logic. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies incident response, threat modeling, compliance retention, audit export, security review, break-glass governance, and stewardship reporting hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #51 first build work aligned to master Phase 7, with earlier prerequisites, Phase 8/9 consumers, and Phase 13 hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 2, 3, 4, 5, 7, 8, 9, and 13 | Attach SDS #51, freeze authority boundaries, preserve Phase 7 as first build point, and record prerequisite and downstream owner gates. |
| 2 | Master Phases 0, 1, 4, and 7 | Define Rust contracts, canonical schemas, lifecycles, stable reason codes, redaction classes, signed envelope requirements, and deterministic fixtures. |
| 3 | Master Phases 1, 4, 7, and 13 | Implement class version and eligible service records, approval, activation, supersession, retirement, revocation, and immutable replay semantics. |
| 4 | Master Phases 2, 3, 4, and 7 | Implement node eligibility requirements, Oververify evidence intake, hard-denial rules, placement guardrails, and freshness-window enforcement. |
| 5 | Master Phases 4, 5, 7, 8, 9, and 13 | Implement package, backup, restore, rollback, failover, logging, audit, metering, incident, and owner-service readiness controls. |
| 6 | Master Phases 1, 4, 7, and 13 | Implement signed operator action evaluation, expiring overrides, two-signer break-glass, and founder fallback/removal action gates. |
| 7 | Master Phases 3, 4, 7, 8, and 9 | Implement side-effect-free placement evaluation snapshots and integrate with Overguard, Oversched, Deployment Planner, and Release Strategy Service. |
| 8 | Master Phases 6, 7, 8, 9, and 13 | Prove the first non-critical system-service eligibility path, migration sequence gates, redacted reports, and operator/developer surfaces. |
| 9 | Master Phases 5, 7, 9, and 13 | Add event streams, observability, diagnostics, metering handoffs, retention, redaction, audit export, incident, and stewardship hooks. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- System-Service Workload Class core is a Rust service/module using shared contract crates, Tokio for bounded evaluation workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Class definitions, eligible service records, node eligibility requirements, placement guardrails, operational bundles, operator action requirements, evaluation snapshots, overrides, events, fixtures, redaction profiles, audit exports, and API objects use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed system-operator or service-account envelopes, tenant/system scope, trace id, idempotency key, schema version, Overguard policy refs, Overwatch audit refs, stable reason codes, and append-only evidence refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for class definition versions, evaluation snapshots, evidence bundles, fixtures, redacted reports, compatibility windows, and deterministic replay comparisons.
- The service may point to Overguard, Oververify, Oversched, Overlease, Overcell, Hardware Discovery, Benchmark Runner, Challenge Task Service, Overclaim, Grid-Resident Service Packager, Package Validator, Backup and Restore Service, Failover and Recovery Coordinator, Deployment Planner, Release Strategy Service, Overwatch, Overmesh, Overmeter, ORU Account Service, Seal Ledger, Overbill, Incident Response, Compliance Boundary, Threat Modeling, Stewardship Reporting, CLI, SDK, and Admin UI, but it must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, revenue projections, customer-count assumptions, raw secret storage, scheduler ownership, package-builder ownership, package-validator ownership, deployment-planner ownership, release-strategy ownership, backup-storage ownership, live-failover ownership, accounting mutation, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Workload-Class Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #51.**
  - Design: Link this document from the System-Service Workload Class SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/deployment_grid/system_service_workload_class.md`, `docs/service_catalog/deployment_grid/system_service_workload_class.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #51 returns both the System-Service Workload Class SDS and this sub-build plan.

- **1.2 Freeze class-authority boundaries.**
  - Design: Record that the service owns system-service class definitions, eligible service records, node eligibility requirements, placement guardrails, operational requirement bundles, operator action requirements, evaluation snapshots, and override records.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the service does not schedule, lease, build packages, validate package internals, deploy, roll out releases, execute backups, restore state, perform failover, enroll providers, maintain mutable trust scores, or mutate accounting state.

- **1.3 Preserve master Phase 7 as the first build point.**
  - Design: Keep first implementation in Phase 7 after control-plane, private-swarm, execution, trust, policy, verification, audit, and accounting prerequisites exist.
  - Output: Phase-gate note that Phase 7 builds protected system-service eligibility, Phase 8/9 consume refs, and Phase 13 hardens governance, incident, retention, and compliance flows.
  - Validation: Review proves this plan does not move Phase 8 data-platform ownership into Phase 7, does not require Phase 9 app deployment for first system-service class evaluation, and does not reorder master Phase 0 through Phase 13.

- **1.4 Carry forward resolved SDS #51 decisions.**
  - Design: Preserve the resolved decisions for evidence-checklist eligibility, first non-critical Overwatch/internal-observability replica, freshness windows by eligibility state, two independent signatures for safety-boundary break-glass, and redacted public placement reports.
  - Output: Resolved-decision checklist tied to eligibility states, first proof, freshness policy, break-glass, public reporting, and operator-only evidence.
  - Validation: Review rejects score-based eligibility, primary-control-plane first target, stale critical evidence, one-signer safety-boundary overrides, public exposure of private topology, and any break-glass path that makes unknown public nodes eligible.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Overguard, Oververify, Oversched, Overlease, Overcell, Hardware Discovery, Benchmark Runner, Challenge Task Service, Overclaim, Grid-Resident Service Packager, Package Validator, Backup and Restore Service, Failover and Recovery Coordinator, Deployment Planner, Release Strategy Service, Overwatch, Overmesh, Overmeter, ORU, Seal Ledger, Overbill, Incident Response, Compliance Boundary, Stewardship Reporting, CLI, SDK, and Admin UI.
  - Output: Boundary matrix listing owner, input refs, output refs, freshness rule, policy refs, evidence refs, denial authority, redaction class, downstream consumer, and rejection behavior.
  - Validation: Review confirms every handoff uses explicit APIs, immutable refs, signed evidence, stable reason codes, trace ids, idempotency keys, policy refs, and Overwatch events rather than privileged shared records or hidden control paths.

## Phase 2: Rust Contracts, Canonical Schemas, Reason Codes, And Fixtures

### Work Items

- **2.1 Create the System-Service Workload Class Rust contract module.**
  - Design: Add contract types for class definitions, eligible services, node requirements, placement guardrails, operational bundles, operator actions, evaluation snapshots, overrides, API errors, events, redaction profiles, and audit exports.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, eligibility-state enums, action-type enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overguard, Oververify, Oversched, Package Validator, Deployment Planner, Release Strategy Service, Backup and Restore, Failover Coordinator, Overwatch, and accounting internals.

- **2.2 Define class definition and compatibility schemas.**
  - Design: Model `system_service_class_definition` with class id, version, description, allowed service scopes, allowed package profiles, required controls, forbidden provider classes, compatibility window, state, policy refs, signer refs, and audit refs.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, class hash fixtures, and compatibility-window examples.
  - Validation: Schema tests reject missing class id, version, required controls, forbidden provider classes, compatibility window, policy refs, signer refs, schema version, stable reason codes, or audit refs.

- **2.3 Define eligible service and node requirement schemas.**
  - Design: Model `eligible_service_record` and `node_eligibility_requirement` with service criticality, package refs, backup/failover/release refs, current class version, operator identity, node identity, uptime, network, storage/backup, security, region, dispute, and system-approval evidence requirements.
  - Output: Service eligibility schema, node requirement schema, evidence-ref shape, blocking-gap shape, lifecycle examples, and negative fixtures.
  - Validation: Tests reject missing package requirements, backup requirements, failover requirements, release requirements, current class version, operator evidence, node evidence, dispute status, or explicit system-service approval.

- **2.4 Define guardrail, operational bundle, operator action, and override schemas.**
  - Design: Model placement guardrails, operational requirement bundles, operator action requirements, and override records with candidate filters, required facts, hard denials, warnings, reason codes, required commands, signer roles, second approval, expiry, incident refs, policy refs, and audit refs.
  - Output: Guardrail schema, operational bundle schema, operator action schema, override schema, redaction classes, stable errors, and replay examples.
  - Validation: Tests prove hard denials cannot be downgraded to warnings, operational bundles require owner-service refs, overrides expire, and break-glass records require policy and Overwatch refs.

- **2.5 Create deterministic eligibility fixtures.**
  - Design: Build fixtures for valid class, missing control, unknown public node, public sandbox node, missing operator identity, stale heartbeat, missing package contract, missing backup/restore evidence, missing failover evidence, active dispute, unsigned maintenance action, expired override, two-signer break-glass, redacted report, and class revocation.
  - Output: Fixture directory, canonical inputs, expected decisions, expected reason codes, expected events, evaluation hashes, redacted views, and replay bundles.
  - Validation: Fixture tests produce stable ids, BLAKE3 hashes, eligibility states, denial reason codes, warning codes, audit refs, and redacted projections across repeated runs.

## Phase 3: Class Version Lifecycle And Eligible Service Registry

### Work Items

- **3.1 Implement class version proposal and read APIs.**
  - Design: Add `POST /system-workload-classes` and `GET /system-workload-classes/{class_id}/versions/{version}` for draft creation, idempotent proposal, policy-ref attachment, review state, active-state reads, and compatibility notes.
  - Output: API handlers, request/response schemas, idempotency behavior, signed envelope checks, stable errors, read projections, and `system_workload_class.version_proposed` events.
  - Validation: API tests cover valid proposal, duplicate idempotency key, missing required controls, unsupported provider class, missing signer, missing policy ref, unauthorized actor, version lookup, and redacted read behavior.

- **3.2 Implement class version approval, activation, supersession, retirement, and revocation.**
  - Design: Enforce lifecycle transitions from draft through under_review, approved, active, superseded, retired, and revoked without rewriting prior decisions.
  - Output: Lifecycle state machine, transition APIs or internal commands, transition events, compatibility-window enforcement, replacement refs, and revocation notifications.
  - Validation: Tests prove accepted class versions are immutable, superseding versions link prior versions, retired versions reject new placements, revoked versions block new placements, and historical evaluation snapshots remain replayable.

- **3.3 Implement eligible service record APIs.**
  - Design: Add `POST /system-services/{service_id}/eligibility` and `GET /system-services/{service_id}/eligibility` for service eligibility proposals, blocking gaps, class-version binding, package requirements, backup/failover/release refs, and current state.
  - Output: Service eligibility handlers, eligibility lifecycle, blocking-gap projection, service read model, state transition events, and stable errors.
  - Validation: Tests prove missing package refs, missing backup refs, missing failover refs, missing release refs, incompatible class versions, or missing owner-service evidence keep service eligibility blocked.

- **3.4 Implement service eligibility lifecycle semantics.**
  - Design: Enforce service states from proposed through blocked, eligible_for_test, eligible_for_noncritical, eligible_for_control_plane, active, suspended, and retired with owner-service evidence requirements at each state.
  - Output: Eligibility transition rules, state-specific required refs, transition blocker model, operator review projection, and event mapping.
  - Validation: Tests prove services cannot skip states, non-critical eligibility cannot imply control-plane eligibility, suspended services cannot receive new placements, and retired services remain readable for audit.

- **3.5 Implement class and service read models.**
  - Design: Build projections for active class version, pending updates, eligible services, blocked services, stale refs, class compatibility, service state, current blockers, and safe operator summaries.
  - Output: Query APIs, filters, pagination, timeline output, redacted public summary, operator-only summary, and projection fixtures.
  - Validation: Contract tests prove clients can display eligibility state, blockers, required controls, compatibility windows, and next actions without reading raw policy traces, private topology, raw secrets, or protected provider evidence.

## Phase 4: Node Evidence Requirements And Placement Guardrails

### Work Items

- **4.1 Implement Oververify evidence intake.**
  - Design: Consume verified operator identity, node identity, security baseline, uptime, network, storage/backup capability, region, dispute status, certification, and `system_service` eligibility signals through immutable evidence refs.
  - Output: Oververify adapter, evidence ref validator, missing-evidence mapper, stale-evidence mapper, redacted evidence summaries, and evidence fixtures.
  - Validation: Tests prove missing or stale operator/node/security/dispute/system-service evidence denies placement without guessing or falling back to unaudited local facts.

- **4.2 Implement node eligibility requirement evaluation.**
  - Design: Evaluate node/provider requirements for test, noncritical, control-plane, and active system-service states using class policy, evidence freshness, package/service criticality, and hard-denial rules.
  - Output: Node eligibility evaluator, state-specific required facts, failure summaries, freshness windows, denial reason codes, and warning reason codes.
  - Validation: Tests prove `eligible_for_test`, `eligible_for_noncritical`, `eligible_for_control_plane`, and `active` use distinct freshness and evidence gates, with expired critical refs blocking new placement.

- **4.3 Publish placement guardrails to Overguard and Oversched.**
  - Design: Expose guardrail facts and candidate-filter rules for Overguard decisions and Oversched candidate filtering without allowing either consumer to mutate class definitions locally.
  - Output: Guardrail read API, Overguard fact-bundle adapter, Oversched filter adapter, reason-code mapping, and cache/freshness rules.
  - Validation: Integration tests prove Oversched receives stable hard-denial and warning reasons, Overguard receives class-policy facts, and downstream services store evaluation refs rather than copying local guardrail logic.

- **4.4 Enforce hard denials for unsafe provider classes.**
  - Design: Hard-deny unknown public nodes, public sandbox nodes, public low-sensitivity providers, active-dispute providers, missing critical evidence, stale critical evidence, and missing system-service approval.
  - Output: Hard-denial ruleset, denial precedence order, redacted denial projection, audit event refs, and negative fixtures.
  - Validation: Tests prove unsafe provider classes cannot be made eligible through warning-only behavior, low-risk service scope, release pressure, scheduler preference, or break-glass override.

- **4.5 Implement freshness windows and stale evidence handling.**
  - Design: Apply SDS #51 freshness windows for live health, route, lease, heartbeat, replacement readiness, policy, class, package, backup, failover, writer, quorum, fencing, and static evidence refs.
  - Output: Freshness policy module, clock abstraction, stale-ref reason codes, reevaluation hooks, operator diagnostics, and time-shift fixtures.
  - Validation: Tests prove stale live facts block promotion at the right eligibility state, static evidence uses Oververify expiry windows, and every placement/release/failover/rollback/founder-migration step reevaluates freshness before action.

## Phase 5: Operational Control Requirements And Owner-Service Readiness

### Work Items

- **5.1 Implement package command-contract checks.**
  - Design: Require Grid-Resident Service Packager and Package Validator refs for health, readiness, diagnostics, backup, restore, rollback, drain, migration, privilege profile, package validation, and compatibility requirements.
  - Output: Package-control checker, required command list, validation report ref checks, privilege-profile ref checks, compatibility blockers, and owner-service gap projection.
  - Validation: Tests prove services remain blocked when package contracts are missing, validation report refs are stale, privilege profiles are too broad, or package class version is incompatible.

- **5.2 Implement backup and restore readiness checks.**
  - Design: Require Backup and Restore Service refs for backup policy, backup runs, snapshot sets, restore plans, restore drills, integrity verification, retention, and stateful control-plane migration readiness.
  - Output: Backup/restore readiness checker, drill freshness rule, restore gap reason codes, disaster-recovery evidence refs, and stateful-service blockers.
  - Validation: Tests prove missing backup, missing restore plan, stale restore drill, failed integrity verification, or missing retention evidence blocks stateful service eligibility and founder-hardware removal.

- **5.3 Implement failover and recovery readiness checks.**
  - Design: Require Failover and Recovery Coordinator refs for health snapshots, failover decisions, recovery plans, writer guards, route shifts, replacement capacity, queue-worker failover, restore-backed recovery, and recovery drills.
  - Output: Failover-readiness checker, recovery gap reason codes, drill refs, writer/fence refs, route-shift refs, and owner-service handoff notes.
  - Validation: Tests prove missing writer guards, stale health snapshots, missing recovery drills, absent replacement capacity, or unresolved route/queue/drain blockers keep service eligibility below control-plane or active states.

- **5.4 Implement release, update, rollback, and freeze readiness checks.**
  - Design: Require Release Strategy Service and Deployment Planner refs for deployment-plan readiness, release plan state, update path, rollback path, version pins, freeze state, approvals, health gates, and package revocation checks.
  - Output: Release/update readiness checker, deployment-plan preflight refs, rollback blockers, freeze blockers, approval blockers, and package-revocation handling.
  - Validation: Tests prove class evaluation blocks update, rollback, or founder-exit actions when deployment-plan refs, release strategy refs, freeze state, approval refs, or package revocation facts are missing or stale.

- **5.5 Implement logging, audit, metering, and incident hook requirements.**
  - Design: Require Overwatch evidence, trace refs, metrics/log declarations, incident-link behavior, Overmeter usage dimensions, ORU/Seal Ledger/Overbill refs where applicable, and Phase 13 governance hooks before higher-risk states.
  - Output: Operational requirement bundle evaluator, evidence writer checks, usage-event mapping, incident hook refs, and audit/export gap projection.
  - Validation: Tests prove services cannot advance without Overwatch audit refs, traceability, usage dimensions, incident hooks where needed, and redacted operator diagnostics; accounting mutation remains outside this service.

## Phase 6: Operator Actions, Overrides, And Break-Glass Controls

### Work Items

- **6.1 Implement operator action requirement records.**
  - Design: Define action types, required signer roles, second-approval requirements, expiry, incident refs, break-glass disclosure behavior, allowed automation scopes, and forbidden override scopes.
  - Output: Operator action requirement schema, signer-role map, action-type registry, expiry rules, incident-link rules, and fixtures.
  - Validation: Tests reject unknown action types, missing signer roles, missing expiry, missing incident refs where required, and action requirements that could allow unknown public nodes.

- **6.2 Implement signed operator action evaluation.**
  - Design: Add `POST /system-workload-classes/operator-actions/evaluate` for maintenance, rollback, migration, founder fallback/removal, stateful writer promotion, stale-evidence override, policy-denial override, and break-glass checks.
  - Output: Evaluation API, signature verifier, policy-ref checker, action result schema, audit refs, redacted reasons, and `system_workload_class.operator_action_evaluated` events.
  - Validation: API tests prove unsigned, expired, wrong-role, one-signer-for-two-signer, stale-policy, missing-incident, and unsafe-provider actions are denied with stable reason codes.

- **6.3 Implement override record lifecycle.**
  - Design: Add `POST /system-workload-classes/overrides` for signed, expiring, policy-approved class overrides with scope, expiry, operator refs, incident refs, Overwatch evidence, and revocation behavior.
  - Output: Override API, lifecycle states, expiry scheduler contract, read projections, revocation behavior, and `system_workload_class.override_recorded` events.
  - Validation: Tests prove overrides require Overguard approval, expire, cannot expand to forbidden provider classes, remain replayable, and are visible in operator/audit views with redaction.

- **6.4 Enforce two-signer break-glass for safety-boundary changes.**
  - Design: Require two independent operator signatures for class overrides, stale-evidence overrides, founder-hardware removal/fallback, writer promotion, fencing/quorum bypass, restore-backed cutover, destructive rollback, release-freeze override, package-revocation exception, route shift, and Overvault/secret-grant expansion.
  - Output: Two-signer enforcement module, signer-independence checks, action classification matrix, disclosure behavior, and negative fixtures.
  - Validation: Tests prove one signer is never sufficient for safety-boundary actions, routine diagnostics stay one-signer or automation only when policy allows, and break-glass cannot make unknown public/public-sandbox/low-sensitivity nodes eligible.

- **6.5 Implement founder fallback and founder removal action gates.**
  - Design: Treat founder-hardware fallback/removal as explicit operator actions requiring class, package, backup, restore, failover, release, freshness, drill, policy, and approval evidence.
  - Output: Founder action gate checker, required evidence matrix, blocker reasons, emergency fallback refs, removal approval refs, and audit events.
  - Validation: Tests prove founder hardware remains emergency fallback until class, backup, restore, failover, rollback, package, release, and rehearsal evidence are current under active policy.

## Phase 7: Placement Evaluation, Snapshots, And Downstream Integrations

### Work Items

- **7.1 Implement side-effect-free placement evaluation.**
  - Design: Add `POST /system-workload-classes/evaluate-placement` for service, package, node, operator, class version, policy refs, operational controls, and freshness facts without scheduling side effects.
  - Output: Placement evaluation API, input loader, fact snapshot, decision result, warnings, hard denials, stable errors, and idempotent evaluation ids.
  - Validation: Tests prove evaluation returns allow/deny/review results without creating leases, scheduling work, changing routes, deploying packages, mutating releases, or touching backup/failover state.

- **7.2 Implement immutable evaluation snapshots and reads.**
  - Design: Store `eligibility_evaluation_snapshot` records with input refs, class version, policy decision refs, result, reason codes, warnings, redaction class, and audit refs.
  - Output: Snapshot writer, BLAKE3 snapshot hash, `GET /system-workload-classes/evaluations/{evaluation_id}`, timeline projection, and replay fixtures.
  - Validation: Tests prove snapshots are immutable, corrected evidence creates new snapshots, old snapshots replay, redacted views hide private topology, and snapshot hashes are stable.

- **7.3 Integrate with Oversched candidate filtering.**
  - Design: Provide Oversched with candidate filters and evaluation refs so it can filter system-service candidates before lease requests while preserving scheduling ownership in Oversched.
  - Output: Oversched adapter contract, candidate filter output, denial reason mapping, replay refs, freshness refresh behavior, and scheduler handoff events.
  - Validation: Integration tests prove Oversched blocks unknown public nodes, disputed nodes, stale evidence, missing controls, and incompatible class versions before lease request creation.

- **7.4 Integrate with Deployment Planner and Release Strategy Service preflight.**
  - Design: Provide class and evaluation refs for deployment-plan readiness, release-plan readiness, update, rollback, freeze, version pin, and promotion checks without owning deployment graphs or release strategy.
  - Output: Deployment Planner adapter, Release Strategy adapter, preflight fact bundle, missing-prerequisite result, and owner-service handoff docs.
  - Validation: Tests prove deployment and release services can require evaluation refs, reject stale refs, and preserve class decisions without copying class policy locally.

- **7.5 Publish downstream integration rules.**
  - Design: Document that Overguard, Oversched, Deployment Planner, Release Strategy Service, Grid-Resident Service Packager, Backup and Restore Service, Failover and Recovery Coordinator, Overwatch, and operator tooling must preserve evaluation refs instead of duplicating class logic.
  - Output: Integration rules, contract examples, consumer checklist, rejection authority table, and anti-copying review criteria.
  - Validation: Review confirms every consumer has a clear owner boundary, required refs, freshness expectations, and failure behavior.

## Phase 8: Phase 7 Migration Proof And Reporting Surfaces

### Work Items

- **8.1 Prove first non-critical observability eligibility.**
  - Design: Advance a non-critical Overwatch/internal-observability replica from proposed to eligible_for_test and eligible_for_noncritical only after package, health, readiness, diagnostics, no-primary-state or restore semantics, class evaluation, release plan, rollback path, and drill evidence exist.
  - Output: First-proof eligibility record, evaluation snapshot, required evidence checklist, blocked-state examples, release/rollback refs, and Overwatch audit refs.
  - Validation: Review proves the first target does not own primary append/checkpoint heads, raw secrets, tenant-private data, user-facing critical paths, or founder-exit risk.

- **8.2 Expand through the Phase 7 migration sequence.**
  - Design: Gate read-only registry/API replicas, workers, queue workers, policy, metering, API ingress, primary control-plane paths, and founder-hardware removal behind state-specific class, package, backup, failover, release, and drill evidence.
  - Output: Migration-sequence eligibility matrix, per-step required refs, promotion blockers, rollback refs, and founder-exit gate report.
  - Validation: Tests prove each step can be blocked independently and later steps cannot inherit eligibility from earlier lower-risk steps.

- **8.3 Implement redacted public and tenant-facing reports.**
  - Design: Expose only safe stewardship facts: service family, workload class, class version, decision state, eligibility state, freshness band, package/profile refs or hash prefixes, safe reason codes, broad failure-domain label when allowed, timestamp, and redacted Overwatch refs.
  - Output: Public report schema, tenant report schema, redaction profile, safe reason-code list, and report fixtures.
  - Validation: Tests prove reports hide node/provider identity, topology, endpoints, route internals, lease details, backup manifests, object placement, ledger checkpoints, queue internals, registry diffs, Overvault refs, signer identities, break-glass details, incident evidence, and cross-tenant dependency graphs.

- **8.4 Implement SDK, CLI, and admin/operator surfaces.**
  - Design: Provide generated client operations for read class, propose class, read service eligibility, evaluate placement, evaluate operator action, record override, list evaluations, list blockers, simulate eligibility, and read redacted reports.
  - Output: SDK operation contracts, Rust SDK hooks, TypeScript/web bindings where needed, CLI command specs, Admin UI view contracts, stable JSON examples, and error examples.
  - Validation: Contract tests prove clients pass signed envelopes, trace ids, idempotency keys, schema versions, policy refs, stable reason codes, redaction rules, and role-scoped operator access through generated contracts.

- **8.5 Implement simulation and dry-run support.**
  - Design: Simulate eligibility state, placement denials, freshness windows, override effects, migration steps, first non-critical proof, and founder-exit gates from fixture and recorded evidence without scheduling or deployment side effects.
  - Output: Simulation API, fixture input schema, simulated timeline output, missing-prerequisite summaries, expected reason codes, and replay packs.
  - Validation: Tests prove simulation is side-effect-free, replayable, redacted, and useful for operator review without mutating class, policy, scheduler, deployment, release, backup, failover, route, package, or accounting truth.

## Phase 9: Observability, Metering, Retention, And Governance Hooks

### Work Items

- **9.1 Emit class and eligibility event streams.**
  - Design: Emit version proposed, version approved, service marked eligible, service blocked, placement evaluated, placement denied, operator action evaluated, override recorded, version retired, version superseded, and version revoked events with redacted refs.
  - Output: Event schemas, Overwatch event writer, idempotency behavior, retry behavior, audit refs, and event fixtures.
  - Validation: Tests prove every state transition and evaluation emits replayable evidence while tenant/public projections hide private topology, policy traces, node/provider evidence, and secret-bearing refs.

- **9.2 Implement operational monitoring and diagnostics.**
  - Design: Track active class versions, pending class versions, eligible services, blocked services, stale evidence counts, override records, placement denial counts, common reason codes, operator action failures, migration blockers, and founder-exit blockers.
  - Output: Health endpoint, metrics/events, operator diagnostics, blocked-service query, stale-evidence query, failure-reason dashboard contract, and Overwatch refs.
  - Validation: Tests prove diagnostic views require authorized operator scope, tenant-facing views are redacted, and stale evidence or blocked placement states emit reason-coded evidence.

- **9.3 Emit system-service eligibility usage facts.**
  - Design: Emit material class proposal, placement evaluation, operator-action evaluation, override, migration-gate, and report-generation usage facts through Overmeter where useful without direct accounting mutation.
  - Output: Usage event schema, Overmeter handoff refs, resource dimensions, stewardship reporting tags, ORU/Seal Ledger/Overbill refs where available, and public-reporting classification.
  - Validation: Tests prove usage facts distinguish backbone operating cost from user workload usage and never create pricing, revenue, ORU balance transitions, invoices, payouts, or ledger entries directly.

- **9.4 Implement retention, redaction, and audit export.**
  - Design: Retain class versions, eligibility records, evidence refs, evaluation snapshots, operator actions, overrides, denial reason codes, redacted reports, and audit exports according to class state, system-service scope, incident refs, and compliance boundary rules.
  - Output: Retention policy schema, redaction classifier, audit export schema, incident/compliance pins, expiry scheduler contract, and evidence export.
  - Validation: Tests prove system-service, override, break-glass, founder-exit, incident, compliance, and public-report decisions preserve required evidence while raw private data, secret refs beyond allowed scope, protected topology, and raw provider evidence are not exposed.

- **9.5 Add incident, threat-model, compliance, and stewardship hooks.**
  - Design: Hand class/evaluation evidence to Incident Response, Threat Modeling and Security Review Tracker, Compliance Boundary, Stewardship Reporting, and PIP governance for high-risk overrides, repeated denials, break-glass, founder-exit, revocation, and safety-boundary changes.
  - Output: Governance handoff schema, incident trigger refs, threat-model checklist refs, compliance export refs, stewardship summary refs, and PIP evidence refs.
  - Validation: Review proves governance hooks are evidence refs and workflow handoffs, not centralization of incident response, compliance authority, stewardship publication, or PIP decisions inside System-Service Workload Class.

## Phase 10: Validation, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack and authority guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, scheduler ownership, package-builder ownership, package-validator ownership, deployment-planner ownership, release-strategy ownership, backup-storage ownership, failover ownership, accounting mutation, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools, local stubs, owner-service refs, or downstream APIs into this service's product boundary.

- **10.3 Validate SDS #51 build-breakdown coverage.**
  - Design: Map every SDS #51 build-breakdown item to this plan: schemas, read APIs, placement evaluation, Overguard/Oververify facts, operational control checks, signed operator actions, lifecycle transitions, Oversched/Deployment Planner integration, and non-critical placement proof.
  - Output: Coverage matrix, API checklist, workflow checklist, fixture checklist, and integration-test targets.
  - Validation: Review proves no SDS #51 build-breakdown item is missing and the plan preserves side-effect-free class evaluation rather than scheduler, release, backup, failover, or deployment ownership.

- **10.4 Validate SDS, service catalog, master plan, crosswalk, and queue alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, build-plan crosswalk, queue state, queue progress, and build-plan progress.
  - Output: Updated backlinks and index rows for SDS #51, queue state update, queue progress update, and build-plan progress evidence.
  - Validation: JSON validation passes; local link checks pass; queue validation confirms `051-build-plan` is complete, no materialized task is running, and `052-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders source alignment, contracts, class/service/node eligibility, guardrails, operational controls, operator actions, overrides, placement evaluations, downstream integrations, migration proof, reporting, client surfaces, evidence, operations, governance hooks, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.

## Alignment Review

- The sub-build plan keeps System-Service Workload Class first build work in master Phase 7 because protected backbone placement depends on private node evidence, policy decisions, package/backup/failover/release controls, signed operator actions, and replayable Overwatch evidence.
- The plan treats Overguard as the policy decision owner; System-Service Workload Class supplies versioned class facts and required policy refs but does not replace Overguard.
- The plan treats Oververify as the provider/node/operator evidence owner; this service consumes evidence refs and freshness windows but does not maintain mutable trust scores or provider reputation.
- The plan treats Oversched as the scheduler and Overlease as the lease owner; this service evaluates eligibility and publishes guardrails without creating placements or leases.
- The plan treats Grid-Resident Service Packager and Package Validator as package-contract and validation-report owners; this service requires package refs and validation refs but does not build or validate packages.
- The plan treats Backup and Restore Service, Failover and Recovery Coordinator, Deployment Planner, and Release Strategy Service as downstream owner services; System-Service Workload Class checks readiness refs and preserves evaluation refs without performing restore, failover, deployment, or release work.
- The plan treats Overwatch as the audit/evidence owner and Overmeter/ORU/Seal Ledger/Overbill as accounting owners; this service emits evidence and usage facts without mutating accounting truth.
- The plan preserves master Phase 0 through Phase 13 ordering and uses earlier phases as prerequisites, Phase 8/9 as downstream consumers, and Phase 13 as governance/security/compliance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service/contracts, native Overrid boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.
