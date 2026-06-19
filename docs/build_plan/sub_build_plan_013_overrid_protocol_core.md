# SUB BUILD PLAN #13 - Overrid Protocol Core

Attached SDS: [docs/sds/control_plane/overrid_protocol_core.md](../sds/control_plane/overrid_protocol_core.md)

## Purpose

This sub-build plan turns SDS #13 into an implementation sequence for Overrid Protocol Core. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overrid Protocol Core is the common rulebook and conformance layer for the ecosystem. It defines protocol artifacts, machine-readable rules, envelopes, state-machine conventions, service ownership boundaries, compatibility policy, fixtures, and reports so every service can behave like one auditable resource-allocation system without turning Protocol Core into a deployed runtime microservice.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #13: Overrid Protocol Core](../sds/control_plane/overrid_protocol_core.md) | Controls protocol purpose, non-goals, artifact model, envelope rules, lifecycle, validation, open-question decisions, and PIP handoff. |
| [Overrid Protocol Core service plan](../service_catalog/control_plane/overrid_protocol_core.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, validation, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies repository layout, shared schema package, local stack, deterministic fixtures, integration harnesses, API/event discipline, and the first Protocol Core artifacts. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies the first signed control-plane golden trace that must conform to Protocol Core. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies execution-state, package, queue, lease, runner, result, retry, cancellation, and usage-event protocol expansions. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies policy, verification, challenge, reputation, dispute, and evidence protocol expansions without moving policy finality into Protocol Core. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies usage, ledger, billing, grant, payout, dispute, and rights-event protocol rules without blockchain, NFT, pricing, revenue, or customer-count assumptions. |
| [Phase 6: First Product Integration](phase_06_first_product_integration.md) | Supplies SDK, CLI, admin UI, adapter, Docdex, Mcoda, Codali, and AI gateway conformance consumers. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protocol requirements for protected system-service workloads, health, failover, backup, restore, and rolling updates. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, Overmesh, namespace, storage, secret-ref, and Overasset protocol expansions. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies deployment package, release, validation, planner, rollback, and rollout protocol expansions. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies federation, purpose-tag, donation, public-interest, and trusted partner protocol requirements. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider, sandbox, fraud, reputation, challenge, and payout-hold protocol requirements. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies native app, mobile, wallet, directory, search, messaging, maps, workspace, social, and stewardship client protocol requirements. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies formal PIP governance, public reporting, compliance, migration, incident, threat-modeling, and protocol-evolution hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #13 first build work aligned to master Phase 0, with later conformance expansion and Phase 13 PIP governance. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first protocol tooling, canonical JSON plus JSON Schema, generated Rust types, signed command envelopes, Ed25519, BLAKE3 refs, native Overwatch evidence, and no conventional cloud product-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 0 | Attach SDS #13 to the build-plan layer, freeze Protocol Core as the specification/conformance authority, and preserve non-runtime boundaries. |
| 2 | Master Phase 0 | Create the protocol artifact layout, machine-readable definition package, fixture structure, and version lifecycle before control-plane services depend on it. |
| 3 | Master Phases 0 and 1 | Define command, event, audit, error, reason-code, trace, idempotency, signature, schema-version, and privacy-label rules for the first signed control-plane path. |
| 4 | Master Phases 0, 1, 3, 4, 5, 8, and 13 | Define state-machine and service-boundary rules that protect ownership across control-plane, execution, policy, accounting, storage, namespace, and governance flows. |
| 5 | Master Phases 0 and 1 | Build conformance fixtures, reports, and the golden trace for Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, and Overqueue. |
| 6 | Master Phases 0, 1, and 6 | Connect the shared schema package, generated Rust types, SDK, CLI, admin UI, adapters, and product clients to the protocol definitions. |
| 7 | Master Phases 0 and 13 | Add compatibility classes, deprecation, migration, rollback, pre-PIP change controls, and Phase 13 PIP handoff. |
| 8 | Master Phases 2 through 12 | Expand protocol coverage for private swarm, execution, trust, metering, grid-resident operation, storage, deployment, federation, public pools, and native apps. |
| 9 | Master Phases 7 and 13 | Add drift detection, conformance coverage, public governance reporting, incident refs, and protocol evidence boundaries. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, tech-stack alignment, documentation links, queue state, master-plan alignment, and final implementation gates. |

## Tech Stack Guardrails

- Protocol Core artifacts live as docs, canonical JSON plus JSON Schema, fixtures, generated Rust types, validation tooling, and reports; they do not require a deployed Protocol Core microservice.
- Rust is the implementation language for protocol checkers, validators, report generators, shared contract crates, CLI checks, and integration harness support.
- Canonical JSON plus JSON Schema is the docs-facing and fixture-facing authority for command envelopes, event envelopes, audit envelopes, error shapes, reason-code registries, privacy labels, compatibility records, conformance manifests, and state-machine definitions.
- Protobuf may be generated or introduced later only for compact internal service/RPC/event contracts where binary compatibility matters; it must not replace JSON Schema as the protocol fixture and documentation authority.
- Signed command envelopes, idempotency keys, trace ids, tenant ids, actor refs, schema versions, stable reason codes, privacy classifications, Ed25519 signatures, BLAKE3 content hashes, and append-only Overwatch evidence refs are required where protocol rules call for them.
- Protocol Core defines private-storage and service-boundary rules by refs and events. It must not grant hidden write access, direct private-table reads, or dual-writer shortcuts between services.
- Protocol Core does not store production records, raw secrets, raw private inputs, wallet balances, payout details, private evidence, or unrestricted native app content.
- Protocol Core does not decide policy, placement, execution, tenancy, credential validity, accounting mutation, ledger finality, disputes, payouts, billing, or governance approval.
- Protocol tooling must use Overrid-owned storage and fixture boundaries or local Overrid-shaped stubs. It must not turn PostgreSQL, Redis, Kafka, NATS, RabbitMQ, S3, MinIO, Vault, cloud KMS, blockchain, NFT mechanics, or conventional SaaS admin framing into product boundaries.
- Protocol docs and reports stay structural. They must not encode pricing tables, revenue projections, customer-count assumptions, market assumptions, or speculative public-market shortcuts.

## Phase 1: SDS Attachment, Rulebook Authority, And Non-Runtime Boundary

### Work Items

- **1.1 Attach the build plan to SDS #13.**
  - Design: Link this document from the numbered Protocol Core SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/control_plane/overrid_protocol_core.md`, `docs/service_catalog/control_plane/overrid_protocol_core.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #13 returns both the Protocol Core SDS and this sub-build plan.

- **1.2 Freeze Protocol Core as the common rulebook.**
  - Design: Record that Protocol Core owns protocol definitions, shared rule text, machine-readable rule files, conformance fixtures, compatibility reports, and PIP handoff rules.
  - Output: Authority checklist for protocol implementation reviews.
  - Validation: Architecture review confirms Protocol Core owns specification and conformance rules but not runtime service state, policy finality, scheduling, execution, tenancy, credentials, accounting, disputes, payouts, or evidence storage.

- **1.3 Preserve master Phase 0 as the first build point.**
  - Design: Keep first implementation in master Phase 0 because the repository layout, shared schema package, local stack, integration harness, and API/event discipline must exist before Phase 1 services can conform.
  - Output: Phase-gate note that SDS #13 starts in Phase 0 and expands through later service domains and Phase 13 governance.
  - Validation: Review proves this plan does not move Protocol Core into master Phase 1 as a runtime service and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve the SDS #13 decisions for canonical JSON plus JSON Schema, docs-facing Markdown rule text, Rust validation and generated types, pre-PIP low-risk change classes, Phase 1 conformance scope, release-blocking service ownership conflicts, and public-report redaction boundaries.
  - Output: Resolved-decision checklist tied to SDS #13 open-question answers.
  - Validation: Review proves the plan does not replace JSON Schema with Protobuf as the fixture authority, accept risky pre-PIP changes, weaken Phase 1 conformance, allow dual writers, or expose private evidence in public reports.

- **1.5 Define non-runtime implementation boundaries.**
  - Design: Treat optional HTTP or UI surfaces as convenience wrappers around protocol artifacts and reports, never as the source of protocol truth.
  - Output: Boundary matrix for protocol docs, machine-readable definitions, generated types, CLI checks, CI checks, integration harness fixtures, conformance reports, and PIP records.
  - Validation: Design review rejects plans that create a privileged Protocol Core service with mutable runtime state or hidden write authority.

## Phase 2: Protocol Artifact Layout, Machine-Readable Definitions, And Fixtures

### Work Items

- **2.1 Create the protocol artifact layout.**
  - Design: Define `protocol/specs`, `protocol/definitions`, `protocol/conformance`, `protocol/check`, `protocol/report`, `protocol/reason-codes`, and `protocol/state-machines` ownership inside the Rust workspace and docs tree.
  - Output: Directory layout, ownership rules, README conventions, and package boundaries for protocol artifacts.
  - Validation: Layout checks confirm protocol artifacts are discoverable by shared schema generation, integration tests, CLI checks, CI, and docs without creating a runtime service.

- **2.2 Define protocol spec metadata.**
  - Design: Model `protocol_spec` with spec id, version, status, scope, owner, source refs, compatibility class, PIP refs, replacement refs, migration notes, rollback notes, and conformance status.
  - Output: JSON Schema, Rust type, example specs, and invalid fixtures for spec metadata.
  - Validation: Schema tests reject missing owner, scope, version, compatibility class, lifecycle status, source refs, and migration fields where required.

- **2.3 Define machine-readable rule families.**
  - Design: Add machine-readable families for command envelopes, event and audit envelopes, API errors, reason codes, trace/idempotency/signature/schema-version/privacy-label primitives, service-boundary metadata, compatibility classifications, conformance manifests, golden trace schemas, and Phase 0/1 state-machine definitions.
  - Output: Definition files, schema refs, fixture manifests, and generation manifests.
  - Validation: Fixture tests prove every day-one family has valid examples, invalid examples, stable reason codes, and generated Rust validation hooks.

- **2.4 Define lifecycle states for protocol artifacts.**
  - Design: Implement draft, reviewed, implemented_in_schema, conformance_ready, accepted, deprecated, retired, and superseded as the artifact lifecycle.
  - Output: State-machine definition, allowed transition map, required evidence refs, and report fields.
  - Validation: State tests reject direct accepted-to-retired changes without deprecation/migration notes and reject accepted rules that lack schema and conformance evidence.

- **2.5 Connect deterministic fixtures to the integration harness.**
  - Design: Provide deterministic valid and invalid payload fixtures for every initial rule family and wire them into the Phase 0 integration harness.
  - Output: Fixture manifest, harness scenarios, fixture naming rules, expected reason codes, and golden report samples.
  - Validation: Harness checks fail closed when fixtures are missing, ambiguous, stale against schemas, or missing expected denial reason codes.

## Phase 3: Command, Event, Audit, Error, And Trace Discipline

### Work Items

- **3.1 Define canonical command envelope rules.**
  - Design: Require command id, tenant ref, actor ref, owner/service refs where applicable, idempotency key, trace id, schema version, payload type, signature requirements, privacy labels, and denial shape.
  - Output: Command envelope rule schema, valid/invalid fixtures, generated validator, and reason-code mapping.
  - Validation: Tests reject missing tenant, actor, trace, idempotency, schema version, signature, payload type, privacy label, or service authority where required.

- **3.2 Define canonical event and audit envelope rules.**
  - Design: Require event id, source service, subject, tenant, actor or system actor, event type, sequence, occurred time, trace id, schema version, privacy classification, evidence refs, and correction model.
  - Output: Event envelope schema, audit envelope schema, append-only event rules, correction fixtures, and Overwatch handoff contract.
  - Validation: Tests reject unversioned events, unclassified events, missing evidence refs, mutable history, private payload leakage, and events without trace continuity.

- **3.3 Define API error and reason-code discipline.**
  - Design: Define stable error shape, reason-code registry, retryability, actor-facing message class, operator detail class, evidence refs, and compatibility behavior for new reason codes.
  - Output: Reason-code registry, error schema, negative fixtures, and service-family mappings.
  - Validation: Tests prove denials emit stable reason codes, trace ids, schema versions, privacy-safe details, and no raw secret or private evidence content.

- **3.4 Define idempotency and replay-window rules.**
  - Design: Specify idempotent success replay, conflicting duplicate denial, replay-window metadata, tenant/actor scoping, trace reuse rules, and accepted side-effect constraints.
  - Output: Idempotency rule file, replay-window fixtures, and service conformance scenarios.
  - Validation: Golden tests cover duplicate success, conflicting duplicate, expired replay window, wrong actor, wrong tenant, and partial side-effect recovery behavior.

- **3.5 Define signature, hash, and privacy-label primitives.**
  - Design: Specify Ed25519 signing expectations, BLAKE3 content-hash refs, rustls/mTLS transport context where applicable, privacy labels, secret refs, payload redaction, and evidence-ref boundaries.
  - Output: Primitive definition files, validator hooks, example signed payloads, and redaction fixtures.
  - Validation: Tests reject unsigned required commands, mismatched hashes, missing privacy labels, raw secret payloads, raw private evidence, and public report leakage.

## Phase 4: State Machines, Service Ownership, And Boundary Conflict Rules

### Work Items

- **4.1 Define Phase 0/1 state-machine requirements.**
  - Design: Define deterministic lifecycle conventions for identity, tenant, key, registry, audit, queue, protocol artifact, and command-processing flows.
  - Output: State-machine definition files, allowed transition maps, terminal states, correction rules, and fixtures.
  - Validation: Tests reject undocumented transitions, missing transition events, terminal-state mutation, and side effects without append-only evidence.

- **4.2 Define cross-domain state-machine templates.**
  - Design: Add reusable conventions for workload, manifest, package, lease, execution, usage, ledger, dispute, governance, namespace, storage, deployment, federation, public-provider, and native-app flows.
  - Output: Template state definitions, owning-service mapping, phase availability notes, and expansion checklist.
  - Validation: Template review proves later services can extend domain states without bypassing ownership, evidence, compatibility, or privacy rules.

- **4.3 Define service ownership boundary rules.**
  - Design: Require one owning writer service, allowed readers, emitted events, consumed events, private-storage boundaries, migration notes, and forbidden shortcuts for every shared domain object.
  - Output: `service_boundary_rule` schema, boundary matrix, fixture examples, and conflict report format.
  - Validation: Boundary tests reject dual writers, direct private storage reads, hidden bypass APIs, and missing event/ref contracts.

- **4.4 Define boundary conflict recovery.**
  - Design: Treat service SDS ownership conflicts as release blockers that require explicit SDS, service catalog, shared schema, conformance fixture, and build-plan crosswalk updates before implementation proceeds.
  - Output: Conflict classification, blocker report, required update checklist, and pre-Phase-13 decision evidence rules.
  - Validation: Release checks fail when a conflict lacks one named owning writer, migration notes, rollback notes, or forbidden-shortcut statements.

- **4.5 Define public/private evidence boundaries.**
  - Design: Distinguish Overwatch evidence refs, protocol reports, PIP summaries, incident refs, compliance refs, and public report summaries from private event bodies and raw evidence.
  - Output: Evidence boundary matrix and report redaction schema.
  - Validation: Privacy tests prove public reports never include tenant ids, actor ids, raw command/event payloads, trace bodies, private audit refs, secret refs, fraud heuristics, sensitive topology, payment/compliance evidence, or private user data.

## Phase 5: Conformance Fixtures, Golden Traces, And CI Checks

### Work Items

- **5.1 Define the Phase 1 conformance suite.**
  - Design: Require strict command envelope validation, generated-schema validator use, tenant and actor refs, signature checks, trace propagation, idempotency behavior, stable denials, Overwatch-compatible events, append-only audit refs, privacy labels, and deterministic state transitions.
  - Output: Phase 1 conformance manifest and service applicability matrix.
  - Validation: Conformance tests fail each Phase 1 participant that lacks any required behavior.

- **5.2 Implement the signed control-plane golden trace.**
  - Design: Specify a signed tenant-scoped synthetic workload command admitted through Overgate, backed by Overpass/Overtenant/Overkey refs, accepted into Overregistry, recorded in Overwatch, and placed into durable Overqueue pending state.
  - Output: Golden trace fixtures, expected events, expected state transitions, expected reason codes, and report sample.
  - Validation: Golden test proves trace id continuity and side-effect order from command admission to pending queue state.

- **5.3 Define negative conformance paths.**
  - Design: Cover invalid schema, invalid signature, duplicate idempotency key, conflicting duplicate, missing tenant, missing actor, missing privacy label, wrong service writer, unsupported schema version, and invalid state transition.
  - Output: Negative fixture set, expected denial reports, and stable reason-code assertions.
  - Validation: Tests prove invalid paths fail before unsafe side effects and preserve audit-safe denial evidence.

- **5.4 Add protocol check commands.**
  - Design: Provide Rust protocol-check commands for schemas, fixtures, state machines, reason codes, golden traces, compatibility records, docs links, and report generation.
  - Output: `protocol/check` command plan, CI hook contract, and local developer commands.
  - Validation: Local and CI checks fail closed on missing fixtures, broken schema refs, invalid links, duplicate reason codes, unclassified changes, or stale generated types.

- **5.5 Generate conformance reports.**
  - Design: Report service, package, SDK, CLI, adapter, and harness conformance by protocol version, rule family, build phase, failure class, and evidence ref.
  - Output: `protocol/report` format, report examples, coverage fields, and failure summaries.
  - Validation: Report tests prove failures are actionable, privacy-safe, versioned, and tied to fixture ids and rule refs.

## Phase 6: Shared Schema Package, Codegen, SDK, CLI, And Product Binding

### Work Items

- **6.1 Connect protocol definitions to the shared schema package.**
  - Design: Make shared schemas import or generate from protocol definitions without manual drift between Markdown rule text, JSON Schema files, Rust types, fixtures, and docs-facing examples.
  - Output: Generation manifest, schema package integration plan, and drift-check rules.
  - Validation: Drift checks fail when rule text, JSON Schema, generated Rust types, fixtures, or reports disagree.

- **6.2 Generate Rust validation and contract types.**
  - Design: Generate or maintain Rust types for protocol specs, command envelopes, events, audit records, errors, reason codes, state machines, boundary rules, compatibility records, conformance manifests, and reports.
  - Output: Rust contract crate plan, validator API, and compatibility tests.
  - Validation: Compile and schema-roundtrip tests prove generated types match the accepted JSON Schema authority.

- **6.3 Define SDK and CLI protocol consumption.**
  - Design: Ensure SDK and CLI clients use generated protocol contracts for signing, idempotency, trace propagation, reason-code decoding, stable JSON output, conformance checks, and local fixture validation.
  - Output: SDK/CLI protocol checklist and command examples.
  - Validation: Client tests fail when SDK or CLI bypasses generated contracts, omits trace/idempotency, hides reason codes, or prints unstable JSON.

- **6.4 Define admin UI and product diagnostics consumption.**
  - Design: Specify how admin/developer UI, Docdex, Mcoda, Codali, AI gateway, encrypted RAG, and adapters consume conformance reports, reason-code registries, state-machine coverage, and compatibility status.
  - Output: Diagnostic contract and product integration fixtures.
  - Validation: Product tests prove diagnostic surfaces show protocol status without exposing private evidence, raw traces, secrets, or tenant-private payloads.

- **6.5 Define TypeScript/web binding limits.**
  - Design: Permit TypeScript/web bindings for browser/client surfaces generated from protocol definitions while keeping Rust as the protocol tooling and core runtime implementation language.
  - Output: Web binding guardrail note and generated TypeScript contract plan for UI/client surfaces.
  - Validation: Review rejects plans that make TypeScript the core protocol checker, scheduler, storage, node-agent, or grid runtime.

## Phase 7: Compatibility, Migration, Deprecation, And PIP Handoff

### Work Items

- **7.1 Define compatibility classes.**
  - Design: Classify changes as additive, deprecated, breaking, migration-required, retired, superseded, and report-only, with rule-family-specific consequences.
  - Output: Compatibility schema, classifier fixtures, and report fields.
  - Validation: Compatibility tests prove every protocol change receives one class and every non-additive change requires migration or rollback notes.

- **7.2 Define pre-Phase-13 change controls.**
  - Design: Before the formal PIP registry exists, allow only low-risk compatible changes without a PIP and require SDS/schema/fixture/report updates for risky protocol changes.
  - Output: Pre-PIP change checklist, low-risk change list, release-blocker list, and decision evidence template.
  - Validation: Checks reject risky pre-PIP changes to envelopes, signatures, tenancy, service writer boundaries, privacy, secrets, accounting, namespace, policy, deletion, SDK behavior, public reports, or accepted state transitions.

- **7.3 Define deprecation and migration windows.**
  - Design: Require replacement refs, supported version windows, migration notes, rollback notes, conformance impact, generated-type impact, and client communication status for deprecated or breaking rules.
  - Output: Migration record schema and fixture examples.
  - Validation: Tests reject deprecation without replacement and reject breaking changes without migration, rollback, security, privacy, and compatibility sections.

- **7.4 Define Phase 13 PIP handoff.**
  - Design: Route non-trivial protocol changes into the PIP registry once Phase 13 governance begins, with security, privacy, compatibility, migration, rollback, public summary, and conformance sections.
  - Output: PIP reference schema, PIP handoff checklist, and accepted-version linking rules.
  - Validation: PIP simulation proves a later protocol change can be proposed, reviewed, accepted, migrated, rolled back, and reported without losing conformance evidence.

- **7.5 Define protocol exception handling.**
  - Design: Allow temporary exceptions only as versioned, evidence-backed, time-bounded compatibility records with owner, affected service, compensating controls, remediation plan, and public/private reporting classification.
  - Output: Exception record schema and exception report format.
  - Validation: Release checks fail if exceptions are open-ended, ownerless, missing remediation, or used to bypass service ownership boundaries.

## Phase 8: Downstream Domain Expansion And Phase-Gate Adoption

### Work Items

- **8.1 Expand private swarm and execution protocol rules.**
  - Design: Add rules for node registration, heartbeat, hardware discovery, benchmark refs, workload manifests, package refs, scheduler facts, leases, runner state, result handoff, retries, cancellation, timeouts, dead letters, and usage events.
  - Output: Phase 2/3 protocol expansion checklist and fixtures.
  - Validation: Execution conformance tests prove private swarm and execution services use exact refs, versions, evidence, state transitions, and reason codes.

- **8.2 Expand trust, policy, verification, and dispute rules.**
  - Design: Add policy dry-run, workload classification, provider verification, challenge, reputation, anti-Sybil, evidence, dispute, refund, correction, and trust-score protocol rules.
  - Output: Phase 4 protocol expansion definitions and fixtures.
  - Validation: Trust tests prove policy/verification services own decisions while Protocol Core only defines envelopes, refs, reason codes, evidence shapes, and state rules.

- **8.3 Expand metering, accounting, grants, and rights rules.**
  - Design: Add usage dimensions, ORU refs, Seal Ledger refs, Overbill refs, grant refs, payout refs, dispute refs, Overasset rights refs, and HTTP 402-style settlement event discipline without pricing or speculative economics.
  - Output: Phase 5 accounting protocol definitions and fixtures.
  - Validation: Accounting tests prove replayability from signed usage/policy/ledger evidence and reject blockchain, NFT, per-operation friction, pricing, revenue, or customer-count assumptions.

- **8.4 Expand data, storage, namespace, deployment, and grid rules.**
  - Design: Add Overbase, Overstore, Overvault, Overmesh, namespace, storage, secret-ref, route, backup, restore, failover, system-service workload, Overpack deployment, release, validator, planner, and rollback protocol rules.
  - Output: Phase 7/8/9 protocol expansion definitions and fixtures.
  - Validation: Data/deployment tests prove private refs stay private, generated reports stay redacted, and deployment decisions cite accepted package/protocol versions.

- **8.5 Expand federation, public pool, native app, mobile, and AI rules.**
  - Design: Add federation template, purpose tag, public-interest pool, public provider, sandbox profile, fraud control, native app, mobile, wallet, directory, search, messaging, maps, workspace, social, central AI, and RAG adapter protocol rules.
  - Output: Phase 10/11/12 protocol expansion checklist and fixtures.
  - Validation: Public/native tests prove public reports and catalogs use redacted summaries and normal Overrid contracts without private data leakage or public-market shortcuts.

## Phase 9: Reports, Drift Detection, Public Governance, And Operations Evidence

### Work Items

- **9.1 Implement protocol drift detection.**
  - Design: Compare SDS text, service catalog files, protocol rule files, shared schemas, generated types, fixtures, conformance reports, and implemented behavior where harness results exist.
  - Output: Drift report format, severity rules, and release-blocker classes.
  - Validation: Drift checks flag missing schema updates, stale generated types, untested state transitions, undocumented reason codes, and SDS/service-boundary mismatches.

- **9.2 Implement conformance coverage reporting.**
  - Design: Report command, event, audit, error, state-machine, boundary, compatibility, migration, reason-code, privacy-label, and golden-trace coverage by service family and build phase.
  - Output: Coverage report examples and thresholds.
  - Validation: Coverage tests fail when a required Phase 1 service lacks minimum conformance and warn when later phase services lack planned fixtures.

- **9.3 Define public governance report boundaries.**
  - Design: Publish accepted protocol versions, compatibility classes, deprecation/migration windows, PIP refs and redacted summaries, service conformance status, reason-code coverage, state-machine coverage, public incident/rollback refs, and public exceptions.
  - Output: Public report schema and redaction rules.
  - Validation: Public report tests prove private user data, tenant ids, actor ids, trace bodies, command payloads, private evidence, secret refs, fraud heuristics, sensitive topology, payment evidence, and compliance evidence stay private.

- **9.4 Define operator and incident evidence refs.**
  - Design: Link protocol reports to Overwatch, incident response, threat modeling, compliance boundary, migration tooling, stewardship reporting, and PIP records using stable evidence refs.
  - Output: Evidence-ref matrix and incident/protocol report handoff checklist.
  - Validation: Incident drills prove operators can trace protocol regressions without exposing private payload bodies in public or low-privilege reports.

- **9.5 Define grid-resident protocol operations.**
  - Design: Specify how protocol checks and reports run as trusted maintenance workloads once the backbone becomes grid-resident, including protected placement, readonly inputs, report signing, rollback, and break-glass rules.
  - Output: Phase 7 operations checklist for protocol tooling.
  - Validation: Grid-readiness review proves protocol tooling can run off founder seed hardware without changing the source-of-truth artifact model.

## Phase 10: Validation, Documentation Alignment, And Final Gates

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #13`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first protocol tooling, canonical JSON plus JSON Schema, generated Rust types, Ed25519, BLAKE3, signed command envelopes, native Overwatch evidence refs, and Overrid-owned boundaries.
  - Output: Tech-stack alignment checklist for Protocol Core.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #13 is represented as a Phase 0 protocol specification/conformance layer with later conformance expansion and Phase 13 PIP governance.
  - Output: Updated master-plan and crosswalk rows for SDS #13.
  - Validation: Review confirms only per-SDS sub-build indexing changed; no master phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #13 and the Protocol Core service plan link back to this sub-build plan and preserve Protocol Core as a non-runtime specification/conformance layer.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare protocol implementation gates.**
  - Design: Require tests for protocol artifact layout, spec metadata, rule-family schemas, lifecycle states, command/event/audit/error envelopes, idempotency, signatures, privacy labels, state machines, service boundaries, conformance fixtures, golden trace, protocol checks, reports, schema/codegen integration, compatibility, migration, pre-PIP controls, PIP handoff, downstream phase expansions, drift detection, public reporting, and operational evidence refs.
  - Output: Final validation checklist for Protocol Core implementation.
  - Validation: Handoff review confirms every later service can consume Protocol Core rules without moving runtime authority, production state, policy decisions, accounting mutation, credential verification, private evidence storage, or governance approval into Protocol Core.

## Alignment Review

- The sub-build plan keeps Protocol Core first build work in master Phase 0, matching SDS #13, the service catalog entry, Phase 0 plan, master build plan, and build-plan crosswalk.
- The plan treats Protocol Core as a specification, machine-readable definition, conformance fixture, validator, report, and governance-handoff layer, not as a deployed runtime control-plane service.
- The plan treats master Phase 1 as the first service-conformance consumer through the signed control-plane golden trace, not as the first implementation point for Protocol Core itself.
- The plan uses later master phases as expansion or hardening gates: private swarm and execution in Phases 2 and 3, trust/policy in Phase 4, usage/accounting in Phase 5, product clients in Phase 6, grid operation in Phase 7, data/storage/namespace in Phase 8, deployment in Phase 9, federation/public pools in Phases 10 and 11, native/mobile/AI clients in Phase 12, and PIP/public governance hardening in Phase 13.
- The plan carries forward SDS #13 resolved decisions for canonical JSON plus JSON Schema, Rust validation and generated types, docs-facing Markdown rule text, Protobuf as later compact internal contract support only, pre-PIP change limits, minimum Phase 1 conformance, service-boundary conflict blockers, and redacted public reporting.
- The plan keeps Protocol Core narrow: no production record storage, no runtime policy finality, no scheduler placement, no execution state, no tenant authority, no credential validity decisions, no ORU or Seal Ledger mutation, no billing/payout/dispute ownership, no raw secret custody, no raw private evidence exposure, and no conventional cloud product-boundary assumptions.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #13 is complete when a builder can implement Protocol Core as the Phase 0 specification and conformance authority with protocol artifact layout, spec metadata, machine-readable canonical JSON plus JSON Schema rule families, protocol artifact lifecycle states, deterministic fixtures, command/event/audit/error envelopes, reason-code discipline, idempotency and replay-window rules, signature/hash/privacy primitives, state-machine definitions, service ownership boundaries, conflict recovery rules, evidence boundary rules, Phase 1 conformance suite, signed control-plane golden trace, negative conformance paths, Rust protocol check commands, conformance reports, shared schema and codegen integration, SDK/CLI/admin/product binding rules, TypeScript/web binding limits, compatibility classes, pre-PIP change controls, deprecation/migration/rollback records, Phase 13 PIP handoff, protocol exception handling, downstream phase expansion rules, drift detection, coverage reporting, public governance report boundaries, incident and operations evidence refs, grid-resident operations behavior, and documentation links that preserve the master Phase 0 through Phase 13 order.
