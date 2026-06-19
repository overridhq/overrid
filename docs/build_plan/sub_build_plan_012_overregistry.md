# SUB BUILD PLAN #12 - Overregistry

Attached SDS: [docs/sds/control_plane/overregistry.md](../sds/control_plane/overregistry.md)

## Purpose

This sub-build plan turns SDS #12 into an implementation sequence for Overregistry. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overregistry is the immutable, versioned fact store for Overrid declarations. It accepts admitted manifest commands, validates them against shared schemas and referenced records, stores accepted versions by content hash, and gives policy, scheduling, packaging, verification, federation, deployment, and native app catalog consumers replayable facts without taking over their runtime authority.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #12: Overregistry](../sds/control_plane/overregistry.md) | Controls Overregistry purpose, record families, API surface, lifecycle states, validation, resolved open-question decisions, and downstream replay rules. |
| [Overregistry service plan](../service_catalog/control_plane/overregistry.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, local stack, deterministic fixtures, API/event conventions, and integration harnesses required before Overregistry implementation. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Controls the first build point for Overregistry as the manifest and declared-facts primitive in the signed control-plane path. |
| [Phase 2: Seed Private Swarm](phase_02_seed_private_swarm.md) | Supplies provider, node, hardware, benchmark, and capability facts that later become registry records. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies package, workload, resource-card, scheduler, lease, runner, result, and revocation re-check consumers. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard, Oververify, challenge, dispute, and trust evidence consumers without moving policy or certification authority into Overregistry. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage attribution and accounting refs without moving ledger mutation or pricing assumptions into Overregistry. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies SDK, CLI, admin UI, Docdex, Mcoda, Codali, and adapter clients that submit and inspect registry-backed manifests. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service workload, backup, restore, failover, rolling update, and break-glass operation requirements. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies deployment package, release, validation, and planner records that depend on package and manifest provenance. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies purpose tags, federation templates, public-interest pool records, and catalog-safe publication paths. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public provider, sandbox, fraud, reputation, and challenge consumers that cite registry versions. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies native app, mobile, directory, search, wallet, workspace, messaging, social, maps, and stewardship catalog consumers. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies migration, threat modeling, compliance, incident, PIP, and reporting hardening for registry schema and fact authority. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #12 first build work aligned to master Phase 1, with later expansion through capability, package, federation, native app, and governance phases. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first control-plane services, Axum/Tower/Hyper-style HTTP, signed command envelopes, Ed25519, BLAKE3 refs, canonical JSON plus JSON Schema, native Overwatch evidence, and no conventional cloud product-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 1 | Attach SDS #12 to the build-plan layer, freeze Overregistry as the versioned declared-facts authority, and preserve resolved registry decisions. |
| 2 | Master Phases 0 and 1 | Build the Rust service skeleton, canonical registry schemas, local Overrid-shaped storage, and deterministic test fixtures. |
| 3 | Master Phase 1 | Implement Phase 1 manifest submission, schema validation, immutable accepted-version writes, and first record families. |
| 4 | Master Phases 1 and 3 | Implement current and historical reads, tenant/catalog filters, exact-version replay, and downstream handoff contracts. |
| 5 | Master Phases 2 and 3 | Expand provider, node, resource, benchmark, and capability records while distinguishing claimed from verified facts. |
| 6 | Master Phases 3 and 9 | Add package provenance, validator refs, runtime contracts, deployment package records, and execution revocation re-checks. |
| 7 | Master Phases 4, 10, and 11 | Add policy/trust evidence refs, revocation propagation, catalog-safe publication, federation, and public-provider guardrails. |
| 8 | Master Phases 5 and 7 | Emit Overwatch evidence, provide usage-relevant refs, expose observability, and prepare grid-resident operation. |
| 9 | Master Phases 6, 10, and 12 | Harden SDK, CLI, admin, product, federation, mobile, and native-app catalog handoff contracts. |
| 10 | Master Phase 1 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, migration, governance, and final implementation gates. |

## Tech Stack Guardrails

- Overregistry core is a Rust service using Tokio and Axum/Tower/Hyper-style HTTP, with rustls/mTLS where early seed control-plane transport requires it.
- Registry records, manifest payloads, schema refs, validation reports, API errors, events, fixtures, catalog records, and compatibility reports use canonical JSON plus JSON Schema from the shared schema package.
- Ed25519 signed command envelopes, idempotency keys, trace ids, tenant ids, owner refs, schema versions, stable reason codes, and append-only Overwatch events are required for mutating APIs.
- BLAKE3 is used for content hashes, manifest hashes, package/artifact refs, validator evidence refs, and hash-linked audit evidence where hashes are needed.
- Accepted record content is immutable. Corrections, schema migrations, catalog changes, suspensions, deprecations, and revocations append new versions or state-transition metadata.
- Overregistry persists state through Overrid-owned abstractions or Overrid-shaped local stubs during early phases. It must not make PostgreSQL, Redis, Kafka, NATS, RabbitMQ, S3, MinIO, Vault, cloud KMS, or similar products the platform boundary.
- Overregistry stores declared facts and refs. It must not store raw secrets, raw private inputs, raw private evidence, mutable hidden overrides, wallet balances, payment details, or unrestricted app content.
- Overregistry does not own policy finality, placement choice, execution state, provider verification, challenge adjudication, resource leases, ORU balance mutation, Seal Ledger mutation, provider payouts, or billing integration.
- Public catalog views are redacted projections from explicit catalog commands and policy refs. Catalog visibility must not leak tenant ids, raw owner identity ids, command ids, trace ids, private audit refs, artifact refs, input/output refs, secret refs, or non-public payload hashes.
- Registry facts remain structural and non-speculative. Overregistry must not implement blockchain, NFT mechanics, pricing tables, revenue projections, customer-count assumptions, or public-market shortcut behavior.

## Phase 1: SDS Attachment, Authority, And Boundary Rules

### Work Items

- **1.1 Attach the build plan to SDS #12.**
  - Design: Link this document from the numbered Overregistry SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/control_plane/overregistry.md`, `docs/service_catalog/control_plane/overregistry.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #12 returns both the Overregistry SDS and this sub-build plan.

- **1.2 Freeze Overregistry as the declared-facts authority.**
  - Design: Record that Overregistry owns immutable versioned records for resource, workload, package, provider, node capability, native app, schema version, purpose tag, catalog, and service facts.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overregistry owns versioned facts but not policy decisions, scheduler placement, execution state, provider trust certification, accounting mutation, or secret custody.

- **1.3 Preserve master Phase 1 as the first build point.**
  - Design: Keep first implementation in master Phase 1 because Overregistry is needed before signed workload commands can cite accepted manifests and reach durable Overqueue state.
  - Output: Phase-gate note that SDS #12 starts in Phase 1 and expands later through provider capability, package provenance, policy, metering, product, grid-resident, federation, native app, and governance gates.
  - Validation: Review proves this plan does not move Overregistry into Phase 0 or change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve the SDS #12 decisions for early manifest-family schemas, catalog-safe fields, current plus previous stable major compatibility, package provenance requirements, and revocation propagation through downstream re-checks.
  - Output: Resolved-decision checklist tied to SDS #12 open-question answers.
  - Validation: Review proves the plan does not reopen settled decisions or replace them with mutable manifests, broad public catalog payloads, silent schema downgrades, package-name-only provenance, or completed-history rewrites.

- **1.5 Define runtime authority boundaries.**
  - Design: Require Overregistry mutating APIs to flow through Overgate-admitted commands while internal read helpers remain caller-scoped and purpose-scoped for approved service accounts.
  - Output: Boundary matrix for Overgate admission, Overpass owner refs, Overtenant scope refs, Overkey signer refs, Overwatch audit refs, Overguard policy refs, Oververify evidence refs, and downstream version consumers.
  - Validation: Design review rejects direct registry mutation by downstream services and rejects read paths that leak records outside tenant, catalog, role, data-class, or policy scope.

## Phase 2: Rust Service Skeleton, Schemas, And Local Fixtures

### Work Items

- **2.1 Create the Overregistry Rust service crate.**
  - Design: Add an Overregistry service crate under the control-plane workspace using Tokio, Axum/Tower/Hyper-style HTTP, shared config loading, tracing setup, and dependency injection for storage, schema validation, Overgate, Overpass, Overtenant, Overkey, Overwatch, and policy callbacks.
  - Output: Service crate, module layout, local-stack service entrypoint, and testable handler boundaries.
  - Validation: Compile and service-start checks pass once implementation exists; crate layout review confirms Overregistry stays separate from Overgate, Overpass, Overtenant, Overkey, Overwatch, Overguard, Oversched, Overpack, Oververify, Overmeter, and accounting internals.

- **2.2 Define canonical registry envelope schemas.**
  - Design: Add shared schemas for `manifest_record`, `manifest_command`, `manifest_version`, `manifest_state_transition`, `schema_version_ref`, validation result refs, API errors, pagination, filters, and Overwatch event payloads.
  - Output: JSON Schema files, Rust generated or hand-written types, compatibility fixtures, and stable reason-code enums.
  - Validation: Schema tests reject missing manifest type, tenant id, owner identity id, command id, trace id, schema version, content hash, validation state, audit refs, and predecessor links where required.

- **2.3 Define Phase 1 manifest-family schemas.**
  - Design: Model workload, resource, package, provider, native app, node capability, catalog, purpose tag, and schema-ref record families early while gating writes by master phase readiness.
  - Output: Registry family schema set, phase-availability metadata, and fixture records for accepted, rejected, superseded, deprecated, suspended, revoked, and archived states.
  - Validation: Schema tests prove Phase 1 can validate all families but the Phase 1 exit gate only requires accepted workload, resource, package, provider, and schema-ref records for the signed synthetic workload path.

- **2.4 Implement Overrid-owned immutable storage boundaries.**
  - Design: Define repositories for immutable record bodies, version indexes, state transitions, schema compatibility reports, validation evidence refs, catalog projections, and local Overrid-shaped storage stubs for early phases.
  - Output: Repository traits, local storage adapter, migration hooks, content-addressed write model, and append-only transition model.
  - Validation: Tests prove accepted content cannot be edited in place and storage never persists raw secrets, private input payloads, raw private evidence, payment details, or mutable hidden overrides.

- **2.5 Connect local development and integration harness fixtures.**
  - Design: Wire Overregistry into the loopback-only local stack and integration harness with deterministic tenants, owners, commands, schemas, workloads, resources, packages, providers, catalog records, and rejected-manifest fixtures.
  - Output: Local service config, fixture refs, seed records, and harness scenario names.
  - Validation: Local smoke tests can submit a signed fixture manifest through Overgate and read the accepted registry version without production credentials or non-Overrid product dependencies.

## Phase 3: Manifest Submission, Validation, And Immutable Versions

### Work Items

- **3.1 Implement signed manifest submission.**
  - Design: Support `POST /v1/manifests` for Overgate-admitted manifest commands with tenant context, owner identity, idempotency key, trace id, schema version, command id, manifest family, and content hash.
  - Output: Manifest submission handler, command validator, idempotent response behavior, and submitted event payload.
  - Validation: API tests cover valid submission, duplicate idempotency keys, conflicting content hashes, missing owner refs, missing tenant scope, invalid manifest family, wrong actor, and schema-version denial.

- **3.2 Implement schema and reference validation.**
  - Design: Validate canonical JSON against shared schemas, then resolve referenced owner, tenant, package, resource, provider, schema, and secret-ref boundaries without dereferencing raw private payloads.
  - Output: Validation pipeline, validation report refs, stable denial reason codes, and rejected-manifest evidence records.
  - Validation: Tests reject malformed schemas, unknown required refs, raw secrets, raw private inputs, mutable override fields, unsupported schema versions, and references outside caller authority.

- **3.3 Implement immutable accepted-version writes.**
  - Design: Store accepted record bodies by BLAKE3 content hash with manifest id, version, predecessor version, active pointer, schema version, validation report ref, command id, trace id, tenant id, owner id, and audit refs.
  - Output: Accepted-version write path, version index, active pointer update, and accepted event payload.
  - Validation: Immutability tests prove accepted record bodies cannot be edited, replaced, or deleted in place and duplicate content hashes return the prior accepted version when allowed.

- **3.4 Implement replacement version flow.**
  - Design: Support `POST /v1/manifests/{manifest_id}/versions` for corrections and updates that link to the prior accepted version and leave old versions readable for replay.
  - Output: Replacement handler, predecessor/successor links, superseded event, and conflict reason codes.
  - Validation: Version tests reject updates without prior-version expectation, conflicting successor links, silent active-pointer changes, downgrades blocked by compatibility policy, and replacement by suspended or wrong-tenant actors.

- **3.5 Implement Phase 1 exit-gate records.**
  - Design: Ensure Phase 1 can accept the minimal workload, resource, package, provider, and schema-ref records needed for a signed synthetic workload to cite accepted manifests and reach Overqueue.
  - Output: Phase 1 registry record set, fixture manifests, and Overqueue handoff refs.
  - Validation: Integration tests prove a signed synthetic workload can reference accepted registry ids, versions, content hashes, and schema versions before entering pending queue state.

## Phase 4: Query APIs, Tenant Filters, And Replay Contracts

### Work Items

- **4.1 Implement caller-visible current reads.**
  - Design: Implement `GET /v1/manifests/{manifest_id}` and family-specific reads with tenant, role, owner, catalog visibility, data-class, and lifecycle-state filtering.
  - Output: Current-read handlers, field-policy matrix, and redaction fixtures.
  - Validation: Tenant isolation tests prove private records are not visible across tenants and catalog readers cannot see non-public command ids, trace ids, private audit refs, artifact refs, secret refs, or raw payload hashes.

- **4.2 Implement historical version reads.**
  - Design: Implement `GET /v1/manifests/{manifest_id}/versions` and exact-version reads so downstream decisions can replay from the facts that existed at admission, scheduling, lease, execution, settlement, or dispute time.
  - Output: Version-list handler, exact-version read handler, retention rules, and replay response shape.
  - Validation: Replay tests reconstruct policy and scheduler inputs from exact registry ids, versions, content hashes, schema versions, and validation report refs.

- **4.3 Implement provider, node, package, and catalog read APIs.**
  - Design: Add `GET /v1/providers/{provider_id}`, `GET /v1/nodes/{node_id}/capabilities`, `GET /v1/packages/{package_id}/versions/{version}`, `GET /v1/catalog`, and `GET /v1/admin/registry/records/{record_id}` with authority-aware filtering.
  - Output: Typed read handlers, admin read model, pagination, sort, and filter contracts.
  - Validation: API tests cover public, tenant-local, owner, operator, internal-service, missing, revoked, suspended, superseded, deprecated, and archived records.

- **4.4 Define downstream replay contracts.**
  - Design: Require Overguard, Oversched, Overpack, Overqueue, Overlease, Overrun, Oververify, deployment planners, federation services, and native app catalogs to cite registry ids, versions, hashes, and schema versions.
  - Output: Downstream contract checklist and fixture calls.
  - Validation: Integration tests fail when downstream services cite mutable handles, latest-only reads, package names without versions, or records without content hashes and schema versions.

- **4.5 Implement cache and invalidation guidance.**
  - Design: Treat registry read caches as bounded optimizations keyed by record id, version, state epoch, tenant scope, caller purpose, field policy, and schema compatibility epoch.
  - Output: Cache guidance in read responses and invalidation events for accepted, superseded, deprecated, suspended, revoked, archived, catalog-changed, and schema-migrated records.
  - Validation: Cache tests prove protected operations re-check exact registry state and do not rely on stale positive reads after revocation, suspension, supersede, schema block, or catalog takedown.

## Phase 5: Provider, Node Capability, And Resource Fact Expansion

### Work Items

- **5.1 Implement provider records.**
  - Design: Store provider identity, tenant or federation scope, onboarding state, verification refs, dispute refs, eligibility refs, owner refs, and lifecycle transitions without certifying trust by itself.
  - Output: Provider record schema, mutation handlers, read handlers, and provider lifecycle events.
  - Validation: Provider tests reject missing identity refs, wrong tenant scope, unauthorized federation scope, mutable trust shortcuts, missing audit refs, and direct payout or accounting fields.

- **5.2 Implement node capability records.**
  - Design: Store node id, hardware class, CPU, memory, GPU, storage, network, benchmark refs, observed capability, claimed capability, verified capability, last verified time, and stale-state metadata.
  - Output: Node capability schema, write path, read path, and capability-recorded event.
  - Validation: Capability tests distinguish claimed from verified facts and reject scheduler eligibility decisions that treat claimed facts as verified evidence.

- **5.3 Integrate hardware discovery and benchmark refs.**
  - Design: Accept hardware discovery and benchmark runner outputs as registry refs with schema versions, validator versions, benchmark profiles, evidence refs, and expiration or freshness metadata.
  - Output: Hardware/benchmark ingestion contract and stale capability report.
  - Validation: Tests prove stale, missing, tampered, wrong-node, wrong-tenant, or incompatible benchmark refs cannot silently upgrade scheduler eligibility.

- **5.4 Implement resource manifest records.**
  - Design: Store provider id, node id, resource class, region or locality, capacity summary, trust class, availability state, capability refs, verification refs, policy refs, and dependency refs.
  - Output: Resource manifest schema, validation rules, and resource read model.
  - Validation: Resource tests reject missing provider/node refs, unsupported resource classes, raw locality details in public catalog projections, incompatible trust classes, and unverified capability promotion.

- **5.5 Prepare scheduler and queue handoff.**
  - Design: Provide Oversched and Overqueue with exact registry facts for workload/resource matching, readiness checks, eligibility filters, and revocation-aware queue behavior.
  - Output: Scheduler read contract, queue-readiness registry check, and stale-fact denial reason codes.
  - Validation: Handoff tests prove queued and scheduled work re-checks resource, provider, node, schema, and package record state before scheduler fetch, lease creation, and runner start.

## Phase 6: Package Provenance, Validation, And Execution Handoff

### Work Items

- **6.1 Implement package manifest records.**
  - Design: Store package id, package version, artifact refs, BLAKE3/content hashes, signature refs, signer or builder identity refs, source/build refs, dependency lock refs, SBOM refs where needed, runtime class, runtime contract refs, permission declarations, validator refs, and policy compatibility refs.
  - Output: Package manifest schema, package write handler, package read handler, and package-recorded event.
  - Validation: Package tests reject package-name-only provenance, mutable external URLs as sole artifact truth, missing hashes, missing signer refs, missing runtime contracts, raw secrets, and unsupported permission declarations.

- **6.2 Integrate package validator evidence.**
  - Design: Accept package validator reports as versioned registry refs with validator version, ruleset version, runtime profile, validation outcome, evidence refs, and retryability metadata.
  - Output: Validator-ref schema, validator handoff contract, and validation report read model.
  - Validation: Validator tests prove denied, expired, incompatible, missing, or wrong-ruleset reports block execution-ready package records.

- **6.3 Support Overpack deployment records.**
  - Design: Expand registry facts for Overpack package manifests, deployment intents, release refs, package validator refs, planner refs, and rollout compatibility without making Overregistry a deployer.
  - Output: Deployment record schemas and Phase 9 handoff checklist.
  - Validation: Deployment tests prove Overregistry stores deployable facts and refs but does not schedule rollout, mutate runtime state, or bypass package validation.

- **6.4 Implement execution revocation re-checks.**
  - Design: Append revocation transitions for exact registry id/version/content hash and require Overqueue, Oversched, Overlease, and Overrun to re-check registry state before readiness, scheduler fetch, lease creation, and runner start.
  - Output: Revocation check contract, blocked-state reason codes, and cancellation/dead-letter handoff rules.
  - Validation: Execution tests prove revoked workload, package, provider, node, resource, or schema refs block new execution and running work follows severity-based policy without rewriting completed history.

- **6.5 Preserve completed-work replay.**
  - Design: Ensure completed work cites the exact registry facts that were active when accepted and receives follow-up revocation, dispute, or accounting refs rather than rewritten registry history.
  - Output: Completed-work replay contract and audit replay checklist.
  - Validation: Replay tests prove completed job audit uses historical registry versions while new retries and new scheduling obey current revoked/suspended/deprecated state.

## Phase 7: Policy, Trust, Revocation, And Catalog Safety Handoff

### Work Items

- **7.1 Add Overguard policy refs.**
  - Design: Store policy refs and decision refs cited by manifests while leaving policy admission, denial, dry-run, and final policy semantics with Overguard.
  - Output: Policy-ref schema, policy decision ref field rules, and denial reason mapping.
  - Validation: Policy tests prove Overregistry can reject missing required refs but does not become the policy engine or silently override Overguard decisions.

- **7.2 Add Oververify and challenge evidence refs.**
  - Design: Store verification, challenge, reputation, anti-Sybil, dispute, and trust evidence refs against provider, node, package, and catalog records without certifying trust by itself.
  - Output: Evidence-ref schema, evidence lifecycle fields, and trust-evidence read filters.
  - Validation: Trust tests prove claimed provider/node facts remain distinct from verified facts and raw private evidence is represented by refs, hashes, redacted summaries, and trace ids.

- **7.3 Implement state transitions for suspension, deprecation, revocation, and archive.**
  - Design: Append state transitions to accepted versions with actor refs, reason codes, evidence refs, effective time, expected prior state, and Overwatch events.
  - Output: State transition handlers, transition records, and state-change events.
  - Validation: State tests reject direct record-body edits, unauthorized transitions, missing evidence refs, cross-tenant transitions, and stale expected-state transitions.

- **7.4 Implement catalog-safe public projections.**
  - Design: Restrict public catalog fields to explicitly published redacted metadata: record ref, manifest family, active version, compatibility class, schema family/version, public title or handle ref, purpose tag refs, owner display ref, visibility state, review/takedown refs, high-level capability class, public app/service refs, and redacted audit/evidence refs.
  - Output: Catalog projection schema, catalog visibility commands, and takedown/hidden states.
  - Validation: Catalog tests prove private tenant ids, raw owner identity ids, command ids, trace ids, private audit refs, artifact refs, input/output refs, package internals, node locality/capacity details, policy refs, secret refs, dispute refs, and non-public hashes stay private by default.

- **7.5 Add federation and public-provider guardrails.**
  - Design: Support purpose tags, federation templates, public-interest pool refs, public sandbox profiles, fraud controls, public provider onboarding, and reputation refs as registry-backed facts gated by owning services.
  - Output: Federation/public record family plan and consumer checklist.
  - Validation: Federation tests prove public-provider or public-catalog records require explicit policy and review refs and never expose private capability, payout, secret, or raw trust evidence fields.

## Phase 8: Events, Observability, Metering Refs, And Grid Operations

### Work Items

- **8.1 Emit Overwatch-compatible registry events.**
  - Design: Emit `overregistry.manifest_submitted`, `manifest_validated`, `manifest_rejected`, `manifest_accepted`, `manifest_superseded`, `provider_registered`, `node_capability_recorded`, `package_recorded`, `catalog_entry_changed`, and `record_deprecated` events.
  - Output: Event builder, Overwatch client, event schemas, and event-to-state transition map.
  - Validation: Audit tests prove events include record refs, content hashes, schema versions, owner refs, tenant refs, trace ids, and reason codes while excluding raw private payloads and raw secrets.

- **8.2 Implement audit-safe metrics and traces.**
  - Design: Record counts by type, tenant, owner, state, schema version, visibility, rejection reason, current/historical version age, stale provider/capability state, package provenance coverage, catalog visibility, and takedown state.
  - Output: Rust tracing and OpenTelemetry-compatible metric hooks with Overwatch as authoritative audit evidence.
  - Validation: Metrics tests prove labels avoid private data, tenant leakage, raw payloads, secrets, raw evidence, non-public hashes, and high-cardinality unbounded values.

- **8.3 Emit usage-relevant refs without accounting mutation.**
  - Design: Emit usage-relevant events for manifest creation, updates, catalog changes, package records, provider records, node capability records, validator reports, and operator actions so Overmeter can attribute usage externally.
  - Output: Overmeter handoff contract and usage-relevant event checklist.
  - Validation: Accounting-boundary tests prove Overregistry does not mutate ORU balances, Seal Ledger entries, invoices, grant pools, provider payouts, wallet accounts, or Overasset rights.

- **8.4 Implement readiness, failure behavior, and dependency state.**
  - Design: Separate liveness from readiness for storage, shared schemas, Overgate, Overpass, Overtenant, Overkey, Overwatch, Overguard, Oververify, Overpack, Overqueue, and policy/catalog dependencies.
  - Output: `GET /v1/healthz`, `GET /v1/readyz`, dependency matrix, degraded-state reason codes, and emergency-buffer rules.
  - Validation: Readiness tests fail closed when Overwatch is unavailable for accepted-version writes and reject unsafe registry mutations when required authority dependencies are missing.

- **8.5 Prepare grid-resident operations behavior.**
  - Design: Define system-service workload needs for Overregistry, including protected placement, replicated state, backup, restore, failover, rolling update, rollback, maintenance mode, break-glass controls, and incident runbooks.
  - Output: Phase 7 operations checklist for Overregistry.
  - Validation: Grid-readiness review confirms founder seed hardware can later be removed from the normal path without changing registry public or internal exact-version contracts.

## Phase 9: SDK, CLI, Admin, Product, Federation, And Native-App Handoff

### Work Items

- **9.1 Harden SDK and CLI registry bindings.**
  - Design: Provide generated contract bindings and Rust-first SDK/CLI flows for manifest submission, version replacement, exact-version reads, provider reads, node capability reads, package reads, catalog queries, and admin diagnostics.
  - Output: SDK/CLI contract examples, stable JSON output shapes, and error/reason-code mappings.
  - Validation: SDK/CLI tests prove clients use generated contracts, pass idempotency and trace ids, decode stable reason codes, and never use latest-only mutable handles for protected decisions.

- **9.2 Implement admin and operator views.**
  - Design: Expose tenant-isolated views for record state, rejected manifests, schema versions, compatibility reports, package provenance coverage, stale capability records, revocation propagation, catalog takedowns, and migration state.
  - Output: Admin read-model requirements and operator diagnostic endpoints or UI contract.
  - Validation: Admin tests prove authorized operators can diagnose registry issues while tenant users cannot see cross-tenant private metadata, hidden refs, raw evidence, private hashes, or unrestricted payload content.

- **9.3 Define product and adapter manifest handoff.**
  - Design: Document how Docdex, Mcoda, Codali, AI gateway, encrypted RAG, runtime bridge, node agents, workers, and product clients submit and consume registry-backed manifests.
  - Output: Product manifest checklist and integration fixtures for Phase 6 and later consumers.
  - Validation: Product integration tests fail when clients bypass Overgate, omit owner refs, omit registry versions, skip schema version refs, skip trace ids, skip idempotency, or rely on unaudited local manifest shortcuts.

- **9.4 Define federation and native-app catalog handoff.**
  - Design: Provide handoff rules for purpose tags, public-interest pools, public sandbox profiles, provider onboarding, native apps, mobile services, directory listings, search, messaging, wallet, maps, workspace, social, and central AI stewardship catalog records.
  - Output: Federation/native catalog checklist for catalog-safe projections, owner display refs, purpose tags, visibility states, review/takedown refs, policy refs, and redacted evidence refs.
  - Validation: Native/federation tests prove apps and public pools use normal Overregistry catalog APIs, respect tenant and public scopes, and do not store private app content or raw evidence inside catalog projections.

- **9.5 Define migration and backfill paths.**
  - Design: Provide migration steps for placeholder manifests, package refs, provider refs, node capability refs, schema refs, native app records, catalog entries, purpose tags, and historical audit refs into stable registry versions.
  - Output: Migration checklist, compatibility fixtures, and rollback rules.
  - Validation: Migration tests prove old refs map to registry ids and exact versions without record takeover, catalog leakage, package-provenance loss, schema replay loss, or audit loss.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #12`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first control-plane implementation, Axum/Tower/Hyper-style HTTP, Ed25519, BLAKE3, signed command envelopes, canonical JSON plus JSON Schema, native Overwatch evidence, and Overrid-owned storage boundaries.
  - Output: Tech-stack alignment checklist for Overregistry.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #12 is represented as a Phase 1 control-plane service with later hardening through provider capability, package provenance, policy/trust, metering refs, product clients, grid-resident operation, federation/public pools, native app catalogs, and governance.
  - Output: Updated master-plan and crosswalk rows for SDS #12.
  - Validation: Review confirms only per-SDS sub-build indexing changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #12 and the Overregistry service plan link back to this sub-build plan and preserve Overregistry's immutable versioned fact-store boundary.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare registry contract and downstream handoff gates.**
  - Design: Require tests for manifest submission, schema validation, immutable writes, version replacement, current reads, historical reads, tenant filtering, catalog filtering, provider records, node capability records, package records, validator refs, revocation transitions, replay, Overwatch events, readiness, migration, and downstream consumers.
  - Output: Final validation checklist for Overregistry implementation.
  - Validation: Handoff review confirms Overgate, Overpass, Overtenant, Overkey, Overwatch, Overguard, Oververify, Overqueue, Oversched, Overpack, Overlease, Overrun, Overmeter, deployment planners, SDK, CLI, admin UI, adapters, federation services, native apps, mobile services, and grid-resident system services can depend on Overregistry facts without moving their runtime authority into Overregistry.

## Alignment Review

- The sub-build plan keeps Overregistry first build work in master Phase 1, matching SDS #12, the service catalog entry, Phase 1 plan, master build plan, and build-plan crosswalk.
- The plan treats master Phase 0 as prerequisite work for schemas, local stack, fixtures, test harness, and shared API/event discipline, not as the Overregistry implementation phase.
- The plan treats later phases as expansion or hardening gates: node/provider capabilities in Phase 2, execution and package provenance in Phase 3, trust/policy/revocation in Phase 4, usage/accounting refs in Phase 5, product clients in Phase 6, grid-resident operations in Phase 7, package deployment in Phase 9, federation/public catalogs in Phases 10 and 11, native/mobile catalogs in Phase 12, and governance/compliance hardening in Phase 13.
- The plan carries forward SDS #12 resolved decisions for early manifest-family schemas, catalog-safe redacted fields, current plus previous stable major compatibility after external dependencies, Phase 3 package provenance requirements, and downstream revocation re-checks.
- The plan keeps Overregistry narrow: no policy finality, no scheduler placement ownership, no execution state, no provider trust certification, no raw secret storage, no raw private input custody, no ORU or Seal Ledger mutation, no billing or payout ownership, no raw private evidence exposure, and no conventional cloud product boundary assumptions.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #12 is complete when a builder can implement Overregistry as the Phase 1 Rust control-plane declared-facts authority with canonical registry schemas, Phase 1 manifest-family models, signed manifest submission, schema/reference validation, immutable accepted-version writes, replacement version flow, current and historical read APIs, tenant/catalog filtering, exact-version replay contracts, provider and node capability records, package provenance and validator refs, revocation transitions with downstream re-checks, catalog-safe public projections, federation and public-provider guardrails, Overwatch-compatible events, usage-relevant refs without accounting mutation, audit-safe observability, readiness/degraded-state behavior, grid-resident operation requirements, SDK/CLI/admin/product/federation/native/mobile handoff rules, migration/governance hardening, and documentation links that preserve the master Phase 0 through Phase 13 order.
