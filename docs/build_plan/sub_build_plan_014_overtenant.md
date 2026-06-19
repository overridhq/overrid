# SUB BUILD PLAN #14 - Overtenant

Attached SDS: [docs/sds/control_plane/overtenant.md](../sds/control_plane/overtenant.md)

## Purpose

This sub-build plan turns SDS #14 into an implementation sequence for Overtenant. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overtenant is the tenant-boundary and role-authority service for Overrid. It owns tenant lifecycle records, memberships, role bindings, quota and budget refs, suspension state, subtenant relationships, private-swarm refs, white-label refs, offboarding records, and fast tenant-check APIs so every protected object and command can be scoped before downstream services act.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #14: Overtenant](../sds/control_plane/overtenant.md) | Controls Overtenant purpose, owned data, API surface, events, state machines, policy/security boundaries, validation, resolved open-question decisions, and downstream handoff. |
| [Overtenant service plan](../service_catalog/control_plane/overtenant.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared tenant schemas, command envelopes, local stack, deterministic fixtures, integration harnesses, and API/event discipline required before Overtenant implementation. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Controls the first build point for Overtenant as the tenant, membership, role, quota-scope, and suspension primitive in the signed control-plane path. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overqueue, scheduler, lease, runner, and execution consumers that must re-check tenant state before protected work starts. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard, policy dry-run, verification, challenge, dispute, and trust consumers without moving policy finality into Overtenant. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies quota, budget, ORU, ledger, billing, grant, and payout refs while keeping billing and ledger mutation outside Overtenant. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies SDK, CLI, admin UI, Docdex, Mcoda, Codali, and adapter clients that create tenants, inspect roles, and consume tenant-check outcomes. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service workload, backup, restore, failover, rolling update, and break-glass operation requirements. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, namespace, Overmesh, and Overasset consumers that need tenant-scoped state, storage, routes, and rights refs. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies cross-tenant federation, public-interest pool, purpose-tag, and grant consumers that require explicit tenant and subtenant boundaries. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider, sandbox, fraud, reputation, challenge, and payout-hold consumers that must respect tenant suspension and public-scope limits. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies native app, mobile, wallet, directory, search, messaging, workspace, social, maps, and AI client tenant-boundary consumers. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies migration, threat modeling, compliance, incident, PIP, stewardship reporting, and legal-retention hardening for tenant boundaries and offboarding. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #14 first build work aligned to master Phase 1, with later hardening through policy, accounting refs, product clients, grid operation, namespace/storage, federation, native apps, and governance. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first control-plane services, Axum/Tower/Hyper-style HTTP, signed command envelopes, Ed25519, BLAKE3 refs, canonical JSON plus JSON Schema, native Overwatch evidence, and no conventional cloud product-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 1 | Attach SDS #14 to the build-plan layer, freeze Overtenant as tenant-boundary authority, and preserve resolved role/cache/subtenant/offboarding decisions. |
| 2 | Master Phases 0 and 1 | Build the Rust service skeleton, shared tenant schemas, local Overrid-shaped storage, and deterministic tenant fixtures. |
| 3 | Master Phase 1 | Implement tenant lifecycle, tenant records, tenant state machine, idempotent creation, and audit evidence. |
| 4 | Master Phase 1 | Implement memberships, role definitions, role bindings, service-account bindings, role lifecycles, and authority checks. |
| 5 | Master Phases 1, 3, and 4 | Implement internal tenant-check APIs, authorization revisions, cache invalidation, fail-closed behavior, and Overgate/Overqueue/Overguard enforcement handoff. |
| 6 | Master Phase 5 | Add quota scope, budget, ORU, accounting-owner, grant, and usage-relevant refs without billing or ledger mutation. |
| 7 | Master Phases 2, 8, 10, and 12 | Add subtenant tree rules, private-swarm refs, white-label refs, namespace/storage route refs, federation scopes, and native-app tenant projection rules. |
| 8 | Master Phases 4, 5, 8, and 13 | Implement suspension, read-only, recovery, offboarding, retained evidence, cleanup task refs, and compliance/dispute retention boundaries. |
| 9 | Master Phases 6, 7, 10, 11, and 12 | Harden SDK, CLI, admin, product, federation, public-provider, native-app, mobile, and grid-resident operation handoffs. |
| 10 | Master Phase 1 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, migration, governance, and final implementation gates. |

## Tech Stack Guardrails

- Overtenant core is a Rust control-plane service using Tokio and Axum/Tower/Hyper-style HTTP, with rustls/mTLS where early seed control-plane transport requires it.
- Tenant, membership, role, quota, suspension, subtenant, private-swarm, white-label, offboarding, API error, event, fixture, and compatibility records use canonical JSON plus JSON Schema from the shared schema package.
- Ed25519 signed command envelopes, idempotency keys, trace ids, tenant ids, actor refs, role refs, schema versions, stable reason codes, privacy labels, and append-only Overwatch events are required for mutating APIs.
- BLAKE3 is used for content/evidence hashes, export/delete proof refs, offboarding manifest refs, and hash-linked audit evidence where hashes are needed.
- Tenant, membership, role, suspension, and offboarding history is append-only. Corrections, revocations, recoveries, and closures create state transitions or new records rather than overwriting history.
- Tenant-check caches are bounded optimizations keyed by tenant id, actor or service-account id, role set, command class, revision, and policy context. Stale or unknown cursors must call Overtenant live, and high-risk mutations fail closed when live tenant state cannot be checked.
- Overtenant persists state through Overrid-owned abstractions or Overrid-shaped local stubs during early phases. It must not make PostgreSQL, Redis, Kafka, NATS, RabbitMQ, S3, MinIO, Vault, cloud KMS, or similar products the platform boundary.
- Overtenant owns tenant authority but not identity proof, credentials, workload policy finality, queue state, registry content, execution placement, ledger finality, billing, provider payouts, native app business records, raw private content, or secret custody.
- Tenant and offboarding docs stay structural. They must not encode pricing tables, revenue projections, customer-count assumptions, blockchain mechanics, NFT mechanics, or public-market shortcuts.

## Phase 1: SDS Attachment, Tenant Authority, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #14.**
  - Design: Link this document from the numbered Overtenant SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/control_plane/overtenant.md`, `docs/service_catalog/control_plane/overtenant.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #14 returns both the Overtenant SDS and this sub-build plan.

- **1.2 Freeze Overtenant as tenant-boundary authority.**
  - Design: Record that Overtenant owns tenant lifecycle, membership, role binding, quota scope, suspension, private-swarm refs, white-label refs, subtenant relationships, offboarding records, and tenant-check APIs.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overtenant owns tenant authority but not identity proof, credentials, policy finality, queue state, registry content, ledger mutation, billing, native app records, or secret custody.

- **1.3 Preserve master Phase 1 as the first build point.**
  - Design: Keep first implementation in master Phase 1 because signed workload commands need tenant scope, role authority, suspension checks, and audit context before reaching registry or queue state.
  - Output: Phase-gate note that SDS #14 starts in Phase 1 and expands later through policy, accounting, product, grid-resident, namespace/storage, federation, native-app, and governance gates.
  - Validation: Review proves this plan does not move Overtenant into Phase 0 and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #14 decisions for Phase 1 role classes, operator as a system/safety identity class, authorization revision caching, single-parent acyclic subtenant trees, explicit inheritance refs, offboarding evidence retention, and state-specific capability blocking.
  - Output: Resolved-decision checklist tied to SDS #14 open-question answers.
  - Validation: Review rejects plans that make operator a normal tenant role, use external pub/sub for day-one cache invalidation, allow implicit subtenant inheritance, delete history during offboarding, or treat tenant states as a single binary active flag.

- **1.5 Define runtime authority boundaries.**
  - Design: Require Overtenant mutating APIs to flow through Overgate-admitted commands while internal tenant-check helpers remain read-only, bounded, and purpose-scoped for approved service accounts.
  - Output: Boundary matrix for Overgate admission, Overpass identity refs, Overkey credential refs, Overwatch audit refs, Overguard policy refs, Overregistry object refs, Overqueue checks, accounting refs, and downstream consumers.
  - Validation: Design review rejects direct tenant mutation by downstream services and rejects read paths that leak membership, role, quota, or offboarding details outside tenant, actor, service-account, command-class, data-class, or policy scope.

## Phase 2: Rust Service Skeleton, Schemas, And Local Fixtures

### Work Items

- **2.1 Create the Overtenant Rust service crate.**
  - Design: Add an Overtenant service crate under the control-plane workspace using Tokio, Axum/Tower/Hyper-style HTTP, shared config loading, tracing setup, and dependency injection for storage, schema validation, Overgate, Overpass, Overkey, Overwatch, Overguard, Overregistry, Overqueue, and accounting-ref clients.
  - Output: Service crate, module layout, local-stack service entrypoint, and testable handler boundaries.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Overtenant stays separate from Overpass, Overkey, Overgate, Overregistry, Overwatch, Overguard, Overqueue, Overmeter, ORU, Seal Ledger, and Overbill internals.

- **2.2 Define canonical tenant schemas.**
  - Design: Add shared schemas for `tenant_record`, `membership_record`, `role_binding`, `role_definition`, `quota_scope_ref`, `suspension_record`, `subtenant_link`, `private_swarm_ref`, `white_label_ref`, `offboarding_record`, tenant-check request/response, API errors, and Overwatch event payloads.
  - Output: JSON Schema files, Rust generated or hand-written types, compatibility fixtures, and stable reason-code enums.
  - Validation: Schema tests reject missing tenant id, lifecycle state, owner identity ref, actor ref, trace id, schema version, audit refs, revision, state-transition reason, and privacy labels where required.

- **2.3 Define Phase 1 role and state fixtures.**
  - Design: Model `owner`, `admin`, `developer`, `viewer`, and `service_account` as Phase 1 role classes; model operator as a system/operator identity class for signed safety actions only.
  - Output: Valid and invalid role fixtures, lifecycle fixtures, and state-specific capability fixtures for pending, active, read_only, suspended, offboarding, closed, and tombstoned tenants.
  - Validation: Fixture tests prove owner-only tenant creation/transfer/offboarding approval, admin membership management, developer manifest/workload submission, viewer read-only access, service-account command scoping, and operator safety-action isolation.

- **2.4 Implement Overrid-owned tenant storage boundaries.**
  - Design: Define repositories for tenant records, membership records, role bindings, role definitions, quota refs, suspension records, authorization revisions, subtenant links, private-swarm refs, white-label refs, offboarding records, and append-only transitions.
  - Output: Repository traits, local storage adapter, migration hooks, revision update model, and append-only transition model.
  - Validation: Tests prove tenant, membership, role, suspension, and offboarding history cannot be overwritten in place and storage never persists raw secrets, raw private content, raw private evidence, payment details, or hidden policy overrides.

- **2.5 Connect local development and integration harness fixtures.**
  - Design: Wire Overtenant into the loopback-only local stack and integration harness with deterministic owners, admins, developers, viewers, service accounts, tenant states, role grants, suspension states, quota refs, subtenants, and offboarding scenarios.
  - Output: Local service config, seed tenant fixtures, role matrix fixtures, and harness scenario names.
  - Validation: Local smoke tests can create a tenant, bind roles, call internal tenant-check, block a suspended tenant, and emit Overwatch-compatible events without production credentials or non-Overrid product dependencies.

## Phase 3: Tenant Lifecycle, State Machine, And Audit Evidence

### Work Items

- **3.1 Implement tenant creation.**
  - Design: Support `POST /v1/tenants` for Overgate-admitted tenant creation with owner identity ref, actor ref, idempotency key, trace id, schema version, tenant type, display handle ref, and initial audit refs.
  - Output: Tenant creation handler, command validator, idempotent response behavior, and `overtenant.tenant_created` event payload.
  - Validation: API tests cover valid creation, missing owner identity, wrong actor authority, duplicate idempotency keys, conflicting tenant payloads, invalid tenant type, missing trace id, and missing audit refs.

- **3.2 Implement tenant read and visibility rules.**
  - Design: Support `GET /v1/tenants/{tenant_id}` with caller-specific filtering for owners, admins, members, service accounts, operators, external viewers, and internal services.
  - Output: Tenant read handler, field-policy matrix, and redaction fixtures.
  - Validation: Tenant isolation tests prove private metadata, quota refs, suspension details, offboarding details, service-account refs, and audit refs are visible only to authorized callers and never leak across tenant boundaries.

- **3.3 Implement tenant lifecycle transitions.**
  - Design: Support `POST /v1/tenants/{tenant_id}/state` for pending, active, read_only, suspended, offboarding, closed, and tombstoned transitions with expected prior state, reason code, actor authority, effective time, and audit refs.
  - Output: State transition handler, transition records, revision increments, and `overtenant.tenant_state_changed` event payload.
  - Validation: State-machine tests reject undocumented transitions, stale expected-state changes, unauthorized transitions, missing reason codes, missing evidence refs, and attempts to reactivate closed or tombstoned tenants.

- **3.4 Implement state-specific capability blocking.**
  - Design: Define capability gates for reads, writes, workload queueing, scheduling, accounting settlement, setup mutations, appeals, recovery, exports, cleanup, dispute, and audit lookup by tenant state.
  - Output: Capability matrix, tenant-check denial reason codes, and state-specific fixtures.
  - Validation: Tests prove pending, read_only, suspended, offboarding, closed, and tombstoned tenants block or allow exactly the capabilities described in SDS #14.

- **3.5 Emit audit evidence for lifecycle operations.**
  - Design: Emit Overwatch-compatible events for tenant creation, state changes, recovery, closure, tombstone reservation, and denied lifecycle mutations.
  - Output: Event builder, Overwatch client, event schemas, and audit evidence refs.
  - Validation: Audit tests prove lifecycle events include tenant id, actor id, prior state, next state, reason code, trace id, schema version, revision, and audit refs while excluding raw private content and secrets.

## Phase 4: Memberships, Roles, Service Accounts, And Authority Checks

### Work Items

- **4.1 Implement membership invite and add flows.**
  - Design: Support `POST /v1/tenants/{tenant_id}/memberships` for invited and direct membership paths with tenant scope, identity ref, inviter authority, membership state, idempotency, expiry, and audit refs.
  - Output: Membership handler, invite/add records, revision increments, and member invitation/addition events.
  - Validation: Membership tests cover invite, accept, add, list, suspend, revoke, expire, wrong-tenant actor, missing identity, duplicate invite, stale membership, and denied cross-tenant access.

- **4.2 Implement membership reads and filters.**
  - Design: Support `GET /v1/tenants/{tenant_id}/memberships` with pagination, state filters, role filters, identity filters, and caller-specific field redaction.
  - Output: Membership list handler, filter contracts, and redaction fixtures.
  - Validation: Read tests prove owners/admins can inspect tenant membership while developers, viewers, service accounts, and external callers receive only allowed fields.

- **4.3 Implement role definitions and role bindings.**
  - Design: Support owner, admin, developer, viewer, and service_account roles with permission sets, scope, grantor ref, effective time, expiry, state, inheritance markers, service-account flag, and compatibility version.
  - Output: Role-definition schema, role-binding repository, grant/update handler, revoke handler, and role grant/revoke events.
  - Validation: Role tests prove owner/admin/developer/viewer/service_account behavior, expired role denial, revoked role denial, wrong grantor denial, stale expected-state denial, and immutable role history.

- **4.4 Implement service-account binding rules.**
  - Design: Bind service accounts to tenant-scoped credential refs, command classes, service scopes, expiry, rotation expectations, and revocation state without storing credentials or deciding key validity.
  - Output: Service-account role schema, binding handlers, Overkey ref contract, and service-account denial reason codes.
  - Validation: Tests prove service-account checks require tenant scope, command class, active role binding, active credential ref from Overkey, and explicit service scope.

- **4.5 Implement authority check helpers.**
  - Design: Centralize role authority checks for tenant creation, membership mutation, role grant, role revoke, quota-ref mutation, suspension proposal, recovery, and offboarding approval.
  - Output: Authority-check module, reason-code mapping, and negative fixtures.
  - Validation: Tests prove unsafe mutations are denied before state changes and denial events preserve trace ids, tenant ids, actor refs, role refs, reason codes, and audit evidence.

## Phase 5: Tenant-Check API, Revisions, Cache Invalidation, And Enforcement Handoff

### Work Items

- **5.1 Implement internal tenant-check API.**
  - Design: Support `POST /v1/internal/tenant-check` for Overgate and approved services, returning tenant state, membership state, role set, service-account scope, quota scope ref, suspension/read-only/offboarding reason codes, revision, and cache directives.
  - Output: Internal tenant-check handler, typed request/response schema, and service-account authorization rules.
  - Validation: Tenant-check tests cover owner, admin, developer, viewer, service account, system service, external viewer, expired role, revoked role, suspended tenant, read-only tenant, offboarding tenant, and missing tenant cases.

- **5.2 Implement authorization revisions.**
  - Design: Increment an Overtenant-owned authorization revision on every tenant, membership, role, quota, suspension, service-account, subtenant, private-swarm, white-label, or offboarding mutation.
  - Output: Revision repository, revision update helper, revision read model, and mutation-to-revision map.
  - Validation: Revision tests prove every protected mutation increments the right tenant revision and records affected tenant, identity, role binding, service account, and command-class invalidation refs.

- **5.3 Implement lightweight invalidation cursors.**
  - Design: Provide invalidation cursor responses before a mature event bus exists, avoiding required Redis/NATS/Kafka-style pub/sub while still allowing Overgate and services to detect stale tenant-check caches.
  - Output: Cursor schema, invalidation response fields, cache-key guidance, and stale-cursor denial reason codes.
  - Validation: Cache tests prove cache keys include tenant id, actor or service-account id, role set, command class, and revision; stale, missing, or unknown cursors bypass cache and call Overtenant live.

- **5.4 Integrate Overgate and Overqueue enforcement.**
  - Design: Require Overgate to block protected mutations for inactive tenant states and require Overqueue or scheduler consumers to re-check tenant state before ready work, scheduling, lease creation, and runner start.
  - Output: Enforcement handoff contract, denial reason codes, and queue/scheduler re-check fixtures.
  - Validation: Integration tests prove suspended or read-only tenants cannot submit new protected work, stale queued work is blocked before execution, and existing queryable state remains available according to state-specific capability rules.

- **5.5 Integrate Overguard policy handoff.**
  - Design: Provide tenant, role, quota, suspension, data-class, and command-class facts to Overguard while leaving workload policy finality and policy dry-run semantics in Overguard.
  - Output: Policy handoff schema, Overguard caller contract, and replay fixture refs.
  - Validation: Policy tests prove Overtenant supplies scope facts and reason codes but does not become the policy engine or silently override Overguard decisions.

## Phase 6: Quota, Budget, ORU, Grant, And Accounting References

### Work Items

- **6.1 Implement quota scope refs.**
  - Design: Store tenant id, quota class, policy ref, budget ref, ORU account ref, accounting owner ref, effective state, actor ref, and audit refs without settling usage.
  - Output: Quota-scope schema, mutation handler, read handler, and `overtenant.quota_scope_changed` event payload.
  - Validation: Quota tests reject missing tenant, missing owner, missing audit refs, unauthorized changes, incompatible quota class, direct balance mutation fields, and hidden pricing fields.

- **6.2 Implement budget and ORU account refs.**
  - Design: Link tenants and subtenants to external budget, grant, ORU account, wallet, and accounting-owner refs for later Overmeter, ORU, Seal Ledger, Overgrant, Overbill, provider payout, and wallet consumers.
  - Output: Accounting-ref schema, compatibility fixtures, and handoff checklist.
  - Validation: Boundary tests prove Overtenant never mutates ORU balances, Seal Ledger entries, invoices, grants, provider payouts, wallet accounts, or payment-provider details.

- **6.3 Emit usage-relevant tenant events.**
  - Design: Emit usage-relevant events for tenant creation, membership changes, role changes, quota changes, suspension, recovery, offboarding, and closure so Overmeter and accounting services can attribute usage externally.
  - Output: Usage-relevant event checklist, Overmeter handoff contract, and audit refs.
  - Validation: Accounting handoff tests prove events are sufficient for usage attribution while excluding direct billing logic, pricing tables, revenue assumptions, and customer-count assumptions.

- **6.4 Implement grant and public-interest scope refs.**
  - Design: Model grant-funded, sponsored, public-interest, and purpose-scoped quota refs as structural references owned by Overgrant and related services.
  - Output: Grant-ref fields, purpose-scope fields, and future-owned eligibility refs.
  - Validation: Grant tests prove Overtenant can cite grants and purpose scopes but cannot allocate grant balances, approve public-interest eligibility, or override Overgrant rules.

- **6.5 Implement quota and accounting observability.**
  - Design: Expose counts and reports for tenants by quota class, missing accounting refs, stale budget refs, orphaned quota scopes, and blocked accounting handoff refs.
  - Output: Audit-safe metrics, admin report shape, and remediation reason codes.
  - Validation: Metrics tests prove labels avoid private data, payment details, tenant leakage to low-privilege callers, and high-cardinality unbounded values.

## Phase 7: Subtenants, Private Swarms, White Labeling, Namespace, And Federation Scope

### Work Items

- **7.1 Implement acyclic subtenant links.**
  - Design: Support single-parent acyclic subtenant trees with child tenant id, parent tenant id, isolation rule, explicit inheritance refs, billing scope ref, and audit refs.
  - Output: Subtenant-link schema, mutation handler, cycle detection, and read model.
  - Validation: Subtenant tests reject cycles, multiple parents, implicit parent access, upward child access, sibling access, missing isolation rule, and hidden billing inheritance.

- **7.2 Implement default-deny role inheritance.**
  - Design: Require explicit delegated role bindings for cross-tenant or parent-to-child access, naming target tenant, command class, effective time, expiry, and audit refs.
  - Output: Delegated-role schema, inheritance denial reason codes, and cross-tenant fixtures.
  - Validation: Tests prove parent authority does not automatically grant child access, child roles never grant upward access, sibling access is denied by default, and delegated access is scoped to named command classes.

- **7.3 Implement private-swarm refs.**
  - Design: Store tenant-to-private-swarm refs for resource pool ref, allowed node refs, trust class, policy refs, state, and audit refs while leaving node inventory, scheduling, and execution authority outside Overtenant.
  - Output: Private-swarm-ref schema, mutation/read handlers, and scheduler/policy handoff checklist.
  - Validation: Tests reject private-swarm refs that bypass Overguard policy, Overregistry capability facts, Overqueue scheduling checks, Overmeter usage attribution, or accounting refs.

- **7.4 Implement white-label and namespace refs.**
  - Design: Store white-label display refs, route refs, namespace refs, owner-display refs, app refs, and public/private visibility flags while leaving namespace ownership and route resolution to the Phase 8 namespace/Overmesh services.
  - Output: White-label-ref schema, namespace-ref fields, and Phase 8 handoff contract.
  - Validation: Namespace tests prove Overtenant does not become the namespace service, does not assign routes by itself, and does not expose private tenant identifiers in public display projections.

- **7.5 Define federation and native-app tenant scope.**
  - Design: Provide tenant and subtenant scope rules for trusted federation, public-interest pools, public-provider onboarding, native apps, mobile gateways, wallet, directory, search, messaging, workspace, social, maps, and central AI clients.
  - Output: Federation/native handoff checklist, public-scope constraints, and catalog-safe tenant projection rules.
  - Validation: Federation/native tests prove clients use normal tenant-check APIs, respect public/private scope, and cannot bypass tenant suspension, quota refs, or delegated role boundaries.

## Phase 8: Suspension, Recovery, Offboarding, Retention, And Cleanup Evidence

### Work Items

- **8.1 Implement suspension records.**
  - Design: Store tenant id, suspension class, reason code, initiated by, effective time, read-only flag, blocked command classes, appeal refs, recovery refs, and audit refs.
  - Output: Suspension schema, apply/release handlers, state-transition events, and denial reason codes.
  - Validation: Suspension tests prove suspended tenants block ordinary reads of sensitive surfaces, all ordinary writes, new queueing, and scheduling while preserving appeal, recovery, operator review, audit, dispute, and required accounting hold/correction paths.

- **8.2 Implement read-only and recovery flows.**
  - Design: Support read-only state, signed recovery actions, role recovery, owner recovery, suspension release, and emergency operator safety actions with evidence refs and expected prior state.
  - Output: Recovery handlers, operator action contract, break-glass audit rules, and recovery event payloads.
  - Validation: Recovery tests prove operator actions are signed, audited, safety-scoped, and never treated as ordinary tenant membership role grants.

- **8.3 Implement offboarding start and workflow refs.**
  - Design: Support `POST /v1/tenants/{tenant_id}/offboarding` once cleanup services exist, with requested by, reason code, freeze state, export refs, deletion refs, retention refs, cleanup tasks, accounting hold refs, and audit refs.
  - Output: Offboarding handler, offboarding record schema, cleanup task refs, and `overtenant.offboarding_started` event payload.
  - Validation: Offboarding tests prove active product reads/writes close according to state rules while export, retention, cleanup, dispute, and accounting-finalization paths remain explicitly scoped.

- **8.4 Define retained evidence boundaries.**
  - Design: Retain tenant id/tombstone, lifecycle/offboarding records, signed command refs, role history, suspension/dispute refs, export/delete manifests, retention class, cleanup states, route/namespace closure refs, quota/budget/accounting refs, Overwatch event refs, and hashes or refs proving export/delete completion.
  - Output: Retained-evidence schema, redaction rules, Overstore/Overvault/accounting/compliance handoff refs, and deletion-proof fixture set.
  - Validation: Retention tests prove raw user content, private payloads, and secrets are removed, vaulted, redacted, or reduced to hashes/refs according to owning-service retention rules.

- **8.5 Implement offboarding completion and tombstone rules.**
  - Design: Close tenants only after required cleanup tasks, export/delete evidence, route closure, namespace closure, accounting holds/corrections, dispute windows, and retention refs are recorded; reserve tombstoned ids permanently.
  - Output: Offboarding completion handler, tombstone record, blocked-cleanup reports, and `overtenant.offboarding_completed` event payload.
  - Validation: Completion tests reject closure with missing cleanup evidence, unresolved required holds, missing tombstone reservation, missing audit refs, or attempts to reuse a tombstoned tenant id.

## Phase 9: SDK, CLI, Admin, Product, Grid, Federation, And Native Handoff

### Work Items

- **9.1 Harden SDK and CLI tenant bindings.**
  - Design: Provide generated contract bindings and Rust-first SDK/CLI flows for tenant create/read/state, membership invite/list, role grant/revoke, tenant-check diagnostics, quota refs, suspension, recovery, and offboarding.
  - Output: SDK/CLI examples, stable JSON output shapes, and error/reason-code mappings.
  - Validation: SDK/CLI tests prove clients use generated contracts, pass idempotency and trace ids, decode stable reason codes, and do not bypass Overgate for tenant mutations.

- **9.2 Implement admin and operator views.**
  - Design: Expose tenant-isolated views for lifecycle state, membership, role drift, orphaned service accounts, quota refs, suspension/read-only/offboarding state, invalidation revisions, cache staleness, and blocked cleanup tasks.
  - Output: Admin read-model requirements and operator diagnostic endpoints or UI contract.
  - Validation: Admin tests prove authorized operators can diagnose tenant issues while tenant users cannot see cross-tenant private metadata, hidden refs, raw evidence, private hashes, or unrestricted payload content.

- **9.3 Define product and adapter handoff.**
  - Design: Document how Docdex, Mcoda, Codali, AI gateway, encrypted RAG, runtime bridge, node agents, workers, and product clients create tenants, bind service accounts, check roles, cite quota refs, and handle suspension/offboarding.
  - Output: Product tenant checklist and integration fixtures for Phase 6 and later consumers.
  - Validation: Product integration tests fail when clients bypass Overgate, omit tenant ids, omit owner refs, skip tenant-checks, skip trace ids, skip idempotency, or rely on unaudited local tenant shortcuts.

- **9.4 Prepare grid-resident operations behavior.**
  - Design: Define system-service workload needs for Overtenant, including protected placement, replicated state, backup, restore, failover, rolling update, rollback, maintenance mode, break-glass controls, and incident runbooks.
  - Output: Phase 7 operations checklist for Overtenant.
  - Validation: Grid-readiness review confirms founder seed hardware can later be removed from the normal path without changing tenant-check, revision, role, suspension, or offboarding contracts.

- **9.5 Define public and native app projection rules.**
  - Design: Provide projection rules for public-provider, federation, native app, mobile, wallet, directory, search, messaging, workspace, social, maps, and central AI surfaces that need tenant display or organization context.
  - Output: Public/native projection schema, redaction rules, review/takedown refs, and field-policy matrix.
  - Validation: Public/native tests prove projections exclude private tenant ids where inappropriate, raw owner identity ids, private audit refs, service-account refs, offboarding evidence, raw traces, and non-public policy/accounting refs.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #14`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first control-plane implementation, Axum/Tower/Hyper-style HTTP, Ed25519, BLAKE3, signed command envelopes, canonical JSON plus JSON Schema, native Overwatch evidence, and Overrid-owned storage boundaries.
  - Output: Tech-stack alignment checklist for Overtenant.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #14 is represented as a Phase 1 control-plane service with Phase 0 prerequisites and later expansion through policy, accounting refs, product clients, grid-resident operation, namespace/storage, federation, native apps, and governance.
  - Output: Updated master-plan and crosswalk rows for SDS #14.
  - Validation: Review confirms only per-SDS sub-build indexing changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #14 and the Overtenant service plan link back to this sub-build plan and preserve Overtenant as tenant-boundary and role-authority service.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare Overtenant implementation gates.**
  - Design: Require tests for tenant create/read/state, membership invite/list/state, role definition, role grant/revoke, service-account binding, authority checks, internal tenant-check, revisions, invalidation cursors, Overgate/Overqueue enforcement, Overguard handoff, quota/accounting refs, subtenant trees, private-swarm refs, white-label refs, suspension, recovery, offboarding, retained evidence, admin/product/native handoff, grid operation, and migration/governance hardening.
  - Output: Final validation checklist for Overtenant implementation.
  - Validation: Handoff review confirms Overgate, Overpass, Overkey, Overregistry, Overwatch, Overqueue, Overguard, Overmeter, ORU, Seal Ledger, Overbill, Overgrant, Overasset, SDK, CLI, admin UI, adapters, federation services, public-provider services, native apps, mobile services, and grid-resident system services can depend on Overtenant without moving their runtime authority into Overtenant.

## Alignment Review

- The sub-build plan keeps Overtenant first build work in master Phase 1, matching SDS #14, the service catalog entry, Phase 1 plan, master build plan, and build-plan crosswalk.
- The plan treats master Phase 0 as prerequisite work for shared tenant schemas, local stack, fixtures, integration harness, and shared API/event discipline, not as the Overtenant implementation phase.
- The plan treats later phases as expansion or hardening gates: queue/scheduler re-checks in Phase 3, policy/trust in Phase 4, quota/accounting refs in Phase 5, product clients in Phase 6, grid-resident operation in Phase 7, storage/namespace/private data refs in Phase 8, federation/public pools in Phases 10 and 11, native/mobile clients in Phase 12, and governance/compliance hardening in Phase 13.
- The plan carries forward SDS #14 resolved decisions for Phase 1 role classes, operator-as-system-safety identity, authorization revision invalidation, single-parent acyclic subtenant trees, explicit inheritance refs, retained offboarding evidence, and state-specific capability blocking.
- The plan keeps Overtenant narrow: no identity proof ownership, no credential custody, no policy finality, no queue ownership, no registry content ownership, no execution placement, no ORU or Seal Ledger mutation, no billing or payout ownership, no native app business record ownership, no raw secret custody, and no conventional cloud product-boundary assumptions.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #14 is complete when a builder can implement Overtenant as the Phase 1 Rust control-plane tenant-boundary authority with canonical tenant schemas, role and state fixtures, tenant lifecycle APIs, membership APIs, role definitions and bindings, service-account binding rules, authority checks, internal tenant-check APIs, authorization revisions, lightweight invalidation cursors, Overgate/Overqueue/Overguard enforcement handoffs, quota and accounting refs without ledger mutation, subtenant single-parent default-deny rules, private-swarm and white-label refs, namespace/federation/native tenant projections, suspension and recovery flows, offboarding records, retained-evidence boundaries, tombstone rules, SDK/CLI/admin/product handoffs, grid-resident operation requirements, implementation validation gates, and documentation links that preserve the master Phase 0 through Phase 13 order.
