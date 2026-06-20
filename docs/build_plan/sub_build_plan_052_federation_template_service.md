# SUB BUILD PLAN #52 - Federation Template Service

Attached SDS: [docs/sds/federation_public/federation_template_service.md](../sds/federation_public/federation_template_service.md)

## Purpose

This sub-build plan turns SDS #52 into an implementation sequence for Federation Template Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Federation Template Service is the trusted-partner capacity contract layer. It owns versioned federation templates, participant role requirements, capacity contribution scopes, workload/data-class eligibility rules, operational terms, accounting-boundary refs, dispute-boundary refs, federation instance records, readiness preflight, and usage-boundary refs. It does not onboard unknown public providers, schedule work, execute workloads, maintain accounts, mutate accounting state, adjudicate disputes, verify identity directly, or decide final policy.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #52: Federation Template Service](../sds/federation_public/federation_template_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Federation Template Service plan](../service_catalog/federation_public/federation_template_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant organization/tenant boundaries, Overkey signing refs, Overgate ingress, Overregistry service facts, Overwatch audit, and Overqueue prerequisites. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Oververify identity/evidence refs, Overclaim dispute records, and Overmesh private route foundations. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overgrant resource-allocation refs, Overmeter usage facts, ORU account views, Seal Ledger streams, Overbill refs, Provider Payout refs, and accounting evidence without direct accounting mutation by this service. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies signed deployable app/package refs, package validation refs, deployment-plan refs, and release/readiness refs consumed by trusted partner swarm proofs. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Controls the first build point: known-participant federation templates, trusted partner swarm proof, purpose-limited templates, public-interest handoffs, billing/accounting boundaries, and dispute boundaries. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies later public-provider hardening boundaries; unknown public-provider onboarding remains out of scope for SDS #52. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies incident response, compliance retention, threat review, public-reporting, audit export, and stewardship reporting hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #52 first build work aligned to master Phase 10, with earlier phases as prerequisites and Phase 11/13 as later hardening gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 9, 10, 11, and 13 | Attach SDS #52, freeze trusted-federation authority boundaries, preserve Phase 10 as first build, and identify prerequisite and downstream owner-service gates. |
| 2 | Master Phases 0, 1, 4, 5, and 10 | Define Rust contracts, canonical schemas, lifecycle states, operational terms, reason codes, signed refs, redaction classes, and deterministic fixtures. |
| 3 | Master Phases 1, 4, 5, 9, and 10 | Implement template draft, submit, review, approve, activate, supersede, retire, and revoke lifecycle with immutable version replay. |
| 4 | Master Phases 1, 4, 5, and 10 | Implement federation instance creation, participant readiness, capacity-scope readiness, identity/evidence freshness, and suspension propagation. |
| 5 | Master Phases 4, 5, 10, and 11 | Implement eligibility rules, Overguard policy, purpose-tag checks, deny-by-default preflight, and Phase 10 data-class guardrails that do not leak into Phase 11 public-provider paths. |
| 6 | Master Phases 5, 10, and 13 | Implement accounting-boundary refs, Overgrant handoffs, usage-boundary refs, dispute-boundary checks, hold behavior, and settlement/evidence refs without accounting mutation. |
| 7 | Master Phases 3, 4, 5, 9, and 10 | Integrate downstream consumers and prove the trusted partner swarm template with package, policy, accounting, dispute, usage, and audit refs. |
| 8 | Master Phases 6, 10, 12, and 13 | Add public-interest templates, reports, simulation, SDK, CLI, admin/developer UI, and redacted stewardship surfaces. |
| 9 | Master Phases 10, 11, and 13 | Add operations, observability, stale-evidence alerts, retention, redaction, audit export, incident, compliance, and stewardship hooks. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Federation Template Service core is a Rust service/module using shared contract crates, Tokio for bounded lifecycle/preflight workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Federation templates, participant role requirements, capacity contribution scopes, workload eligibility rules, accounting boundaries, dispute boundaries, federation instances, readiness preflight results, usage-boundary refs, reports, events, fixtures, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed verified-operator envelopes, tenant/organization context, trace id, idempotency key, schema version, Overguard policy refs, Oververify evidence refs, Overwatch audit refs, stable reason codes, and append-only evidence refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for template versions, instance snapshots, readiness preflight bundles, usage-boundary refs, report projections, audit exports, and deterministic replay fixtures.
- Federation Template Service may point to Overtenant, Overpass, Overkey, Oververify, Overguard, Overgrant, Overmeter, Overbill, ORU Account Service, Seal Ledger, Provider Payout Service, Overclaim, Overwatch, Overpack, Package Validator, Deployment Planner, Public-Interest Pool Service, Purpose Tag Registry, SDK, CLI, Admin UI, Central AI Service, Incident Response, Compliance Boundary, and Stewardship Reporting, but it must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, revenue projections, customer-count assumptions, raw secret storage, public-provider onboarding, scheduling, workload execution, grant creation, account balances, invoices, payouts, ledger entries, dispute adjudication, final policy decisions, identity verification, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Federation Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #52.**
  - Design: Link this document from the Federation Template Service SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/federation_public/federation_template_service.md`, `docs/service_catalog/federation_public/federation_template_service.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #52 returns both the Federation Template Service SDS and this sub-build plan.

- **1.2 Freeze trusted-federation authority boundaries.**
  - Design: Record that the service owns template versions, participant roles, capacity scopes, eligibility rules, operational terms, accounting/dispute boundary refs, instance records, readiness preflight, usage-boundary refs, and report projections.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the service does not onboard unknown public providers, schedule, lease, execute, validate package internals, create grants, mutate accounts, write ledger entries, issue payouts, adjudicate claims, verify identities directly, or replace Overguard policy decisions.

- **1.3 Preserve master Phase 10 as the first build point.**
  - Design: Keep first implementation in Phase 10 because trusted federation depends on control-plane identity/tenancy, verification/policy, accounting refs, deployable-package readiness, and public-interest purpose-tag foundations.
  - Output: Phase-gate note that earlier phases are prerequisites, Phase 10 builds known-participant federation, Phase 11 hardens public-provider separation, and Phase 13 hardens governance/reporting.
  - Validation: Review proves this plan does not move trusted federation into Phases 0 through 9, does not require Phase 11 public supply for first value, and does not reorder master Phase 0 through Phase 13.

- **1.4 Carry forward resolved SDS #52 decisions.**
  - Design: Preserve the decisions that the first proof is a trusted partner swarm, trusted-federation data classes are tiered, terms are operational refs not pricing forecasts, cross-tenant work requires a concrete dispute boundary, and reports use audience-specific redaction.
  - Output: Resolved-decision checklist tied to trusted partner proof, data-class allowlists, `terms_kind`, `federation_dispute_boundary`, and public/participant/operator/Central-AI report profiles.
  - Validation: Review rejects direct public-provider onboarding, informal operator promises, hidden private-data expansion, concrete price/invoice/balance storage, missing dispute owners, raw secret exposure, and public reports that expose provider-private topology or payment details.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Overtenant, Overpass, Overkey, Oververify, Overguard, Overgrant, Overmeter, Overbill, ORU Account Service, Seal Ledger, Provider Payout Service, Overclaim, Overwatch, Overpack, Package Validator, Deployment Planner, Purpose Tag Registry, Public-Interest Pool Service, SDK, CLI, Admin UI, Central AI Service, Incident Response, Compliance Boundary, and Stewardship Reporting.
  - Output: Boundary matrix listing owner, input refs, output refs, freshness rule, policy refs, evidence refs, redaction class, downstream consumer, and rejection behavior.
  - Validation: Review confirms every handoff uses explicit APIs, immutable refs, signed evidence, stable reason codes, trace ids, idempotency keys, policy refs, and Overwatch events rather than privileged shared records or hidden control paths.

## Phase 2: Rust Contracts, Schemas, Terms, And Fixtures

### Work Items

- **2.1 Create the Federation Template Rust contract module.**
  - Design: Add contract types for template definitions, participant role requirements, capacity scopes, eligibility rules, accounting boundaries, dispute boundaries, federation instances, preflight results, usage-boundary refs, events, reports, redaction profiles, and stable errors.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, `terms_kind` enums, template-type enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overtenant, Oververify, Overguard, Overgrant, Overbill, ORU, Seal Ledger, Provider Payout, Overclaim, scheduling, and deployment internals.

- **2.2 Define template, participant, and capacity schemas.**
  - Design: Model `federation_template`, `participant_role_requirement`, and `capacity_contribution_scope` with template type, version, participant roles, required identity tiers, operator contacts, verification refs, dimensions, geography, quota windows, availability windows, maintenance windows, policy refs, and audit refs.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, template hash fixtures, and compatibility examples.
  - Validation: Schema tests reject missing template id/version, participant type, required identity tier, verification ref, capacity dimension, quota window, policy ref, schema version, stable reason code, trace id, or audit ref.

- **2.3 Define eligibility, accounting, dispute, and instance schemas.**
  - Design: Model `workload_eligibility_rule`, `federation_accounting_boundary`, `federation_dispute_boundary`, and `federation_instance` with workload class, data class, purpose tags, tenant scope, route/storage/vault restrictions, sponsor/payer refs, provider-earning refs, hold rules, evidence/log owner, response SLA, appeal route, effective window, status, and audit refs.
  - Output: Eligibility schema, accounting-boundary schema, dispute-boundary schema, instance schema, lifecycle examples, and negative fixtures.
  - Validation: Tests reject missing workload class, data class, tenant scope, purpose ref when required, accounting boundary, dispute owner, evidence owner, hold target, effective window, policy ref, or Overwatch ref.

- **2.4 Define operational terms and report redaction schemas.**
  - Design: Model operational terms as refs and categories such as donated capacity, reciprocal overflow, accountable near-cost, invoice-backed sponsor, grant-sponsored, and emergency sponsored without storing concrete prices, balances, invoices, payouts, or settlement entries.
  - Output: `terms_kind` schema, term-ref schema, public report schema, participant report schema, operator report schema, Central-AI report schema, redaction profile, and examples.
  - Validation: Tests prove term records store only signed refs, hashes, categories, windows, obligations, compatible accounting refs, and report projections while rejecting raw payment details, forecasts, customer counts, and provider-private topology in public reports.

- **2.5 Create deterministic federation fixtures.**
  - Design: Build fixtures for valid trusted partner swarm, university template, research-only template, public-interest template, missing identity, missing accounting boundary, missing dispute boundary, purpose tag absent, data class denied, participant suspended, stale verification, Overguard unavailable, and redacted reports.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected reason codes, preflight hashes, usage-boundary refs, report projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, BLAKE3 hashes, lifecycle states, denial reason codes, audit refs, and redacted views across repeated runs.

## Phase 3: Template Version Lifecycle And Registry

### Work Items

- **3.1 Implement template draft and read APIs.**
  - Design: Add `POST /federation-templates` and `GET /federation-templates/{template_id}/versions/{version}` for draft creation, idempotent proposal, signed-operator checks, policy refs, redacted reads, and version lookup.
  - Output: API handlers, request/response schemas, idempotency behavior, signed envelope checks, stable errors, read projections, and `federation_template.drafted` events.
  - Validation: API tests cover valid draft, duplicate idempotency key, missing participant roles, unsupported data class, missing policy refs, unauthorized actor, version lookup, and redacted read behavior.

- **3.2 Implement submit and policy-review lifecycle.**
  - Design: Freeze draft templates on submit, request Overguard template policy review, validate deny-by-default behavior, and prevent mutable template edits after submission without a new revision.
  - Output: Submit API, review state machine, immutable input snapshot, policy-review adapter, validation report refs, and `federation_template.submitted` events.
  - Validation: Tests prove submitted templates cannot silently change participant roles, capacity scopes, eligibility rules, operational terms, accounting boundary, dispute boundary, policy refs, or report profiles.

- **3.3 Implement approval and activation lifecycle.**
  - Design: Activate template versions only after Overguard approval, operator approval, required identity/evidence prerequisites, accounting-boundary readiness, dispute-boundary readiness, and purpose-tag readiness are present.
  - Output: Approval API, activation command, active-template projection, signer checks, policy compatibility checks, and `federation_template.approved` events.
  - Validation: Tests prove missing policy approval, missing operator approval, stale evidence, missing accounting boundary, missing dispute boundary, or unverified purpose tag blocks activation.

- **3.4 Implement supersede, retire, deny, and revoke lifecycle.**
  - Design: Support version supersession, template retirement, policy denial, and revocation without rewriting historical instance usage-boundary refs or prior decisions.
  - Output: Lifecycle transition APIs or commands, transition events, replacement refs, denial reasons, revocation behavior, and timeline projections.
  - Validation: Tests prove superseded templates link replacement versions, retired/revoked templates reject new instances, denied templates remain explainable, and historical usage-boundary refs remain readable for audit.

- **3.5 Implement template registry projections.**
  - Design: Build read models for active templates, pending templates, denied templates, retired templates, supported template types, required participant roles, allowed data classes, purpose tags, stale refs, and report visibility.
  - Output: Query APIs, filters, pagination, redacted public summary, participant summary, operator summary, and projection fixtures.
  - Validation: Contract tests prove clients can display template state, roles, capacity scope, allowed workload/data classes, purpose tags, blockers, and next actions without raw policy traces, private topology, raw secrets, or payment details.

## Phase 4: Federation Instance Binding And Participant Readiness

### Work Items

- **4.1 Implement federation instance creation.**
  - Design: Add `POST /federation-instances` to bind active template versions to verified participants, tenants, effective windows, grant/accounting refs, dispute refs, and policy refs.
  - Output: Instance API, request/response schemas, idempotency behavior, active-template compatibility checks, participant binding model, and `federation_template.instance_created` events.
  - Validation: Tests prove inactive templates, wrong version refs, missing tenant scope, missing participant refs, missing effective window, missing grant/accounting refs, or duplicate idempotency keys are handled with stable reason codes.

- **4.2 Validate participant identity and role readiness.**
  - Design: Consume Overtenant, Overpass, Overkey, and Oververify refs for organization identity, operator identity, signing authority, participant role fit, suspension state, evidence freshness, and membership boundaries.
  - Output: Participant-readiness checker, identity-ref validator, signing-authority check, suspension mapper, missing-evidence mapper, and stale-evidence fixtures.
  - Validation: Tests prove missing/stale identity, missing operator contact, wrong role authority, suspended participant, expired signing ref, or missing verification evidence blocks instance activation.

- **4.3 Validate capacity contribution readiness.**
  - Design: Check declared capacity dimensions, availability windows, quota windows, region/geography, maintenance windows, reliability expectations, recipient scope, package/deployment refs where relevant, and provider obligation refs.
  - Output: Capacity-readiness checker, capacity status lifecycle, unavailable/exhausted/paused mapping, operator diagnostics, and readiness fixtures.
  - Validation: Tests prove missing dimensions, expired windows, incompatible region, paused capacity, exhausted quota, stale benchmark/evidence refs, or unsupported recipient scope blocks preflight with stable reason codes.

- **4.4 Implement instance lifecycle transitions.**
  - Design: Enforce instance states from proposed through awaiting_participant_evidence, preflighting, active, suspended, renewal_pending, retired, and revoked with explicit state transition events.
  - Output: Instance state machine, transition APIs or commands, lifecycle events, timeline projection, renewal refs, and revocation behavior.
  - Validation: Tests prove instances cannot skip readiness states, suspended participants suspend affected instances, renewal requires fresh evidence, retired instances reject new usage, and revoked instances remain replayable for audit.

- **4.5 Publish participant and instance read models.**
  - Design: Build projections for active instances, blocked instances, participant scopes, capacity scopes, stale evidence, missing accounting/dispute facts, purpose-tag blockers, and allowed audience views.
  - Output: Query APIs, participant view, tenant view, operator view, redacted public view, and instance timeline output.
  - Validation: Contract tests prove reports expose only audience-appropriate template, instance, participant, capacity, quota, and evidence refs while protecting contacts, private topology, raw secrets, and payment details.

## Phase 5: Eligibility, Policy, Purpose Tags, And Preflight

### Work Items

- **5.1 Implement workload and data-class eligibility evaluation.**
  - Design: Evaluate workload class, data class, tenant eligibility, route/storage/vault restrictions, purpose tags, geography, participant role, and instance status before any cross-tenant usage-boundary refs are issued.
  - Output: Eligibility evaluator, allowed/denied result schema, denial precedence, stable reason codes, warning codes, and eligibility fixtures.
  - Validation: Tests prove ineligible workload classes, denied data classes, unsupported route/storage/vault restrictions, missing tenant scope, suspended instances, or stale policy refs deny usage before scheduling.

- **5.2 Integrate Overguard policy review and preflight.**
  - Design: Ask Overguard to validate template policy, instance readiness, workload/data-class eligibility, purpose-tag compatibility, tenant boundaries, and deny-by-default behavior without local policy duplication.
  - Output: Overguard adapter, policy fact bundle, policy decision refs, preflight result, policy-stale handling, and `federation_template.instance_preflight_passed/failed` events.
  - Validation: Tests prove Overguard denial, Overguard unavailability, stale policy refs, missing decision refs, or policy-template mismatch blocks activation and cross-tenant scheduling.

- **5.3 Implement purpose-tag readiness checks.**
  - Design: Integrate Purpose Tag Registry refs for science, education, medical, opensource, climate, public infrastructure, emergency, and later stewarded categories before purpose-limited or public-interest usage.
  - Output: Purpose-tag checker, evidence-ref validator, purpose eligibility result, tag expiry handling, and purpose fixtures.
  - Validation: Tests prove purpose-limited templates require verified tags, missing/stale purpose evidence fails with `purpose_tag_not_verified`, and public-interest capacity cannot activate from informal purpose claims.

- **5.4 Enforce trusted-federation data-class guardrails.**
  - Design: Apply SDS #52 tiering: default proof supports `public`, `public_low_sensitivity`, and `grant_funded_public_interest`; later explicit templates may support tightly scoped `organization_private` or `tenant_private`; `user_private`, `secret_bearing`, `regulated`, and `system_service` stay denied unless a future template/version proves controls.
  - Output: Data-class ruleset, guardrail matrix, denial reason codes, explicit-template override requirements, and negative fixtures.
  - Validation: Tests prove denied classes cannot fall through to public-provider paths, organization/private exceptions require named participants and restrictions, and Phase 11 public nodes never inherit trusted-federation privileges.

- **5.5 Implement side-effect-free readiness preflight.**
  - Design: Add `POST /federation-instances/{instance_id}/preflight` to check identity, tenant, policy, accounting, purpose, capacity, data class, dispute boundary, reporting, and evidence freshness without scheduling or accounting side effects.
  - Output: Preflight API, input snapshot, readiness result, missing-prerequisite list, BLAKE3 preflight hash, event refs, and preflight timeline.
  - Validation: Tests prove preflight is idempotent, replayable, side-effect-free, blocks missing prerequisites, and does not create leases, schedule jobs, create grants, mutate accounts, create invoices, write ledger entries, or adjudicate disputes.

## Phase 6: Accounting, Grants, Usage Boundaries, And Disputes

### Work Items

- **6.1 Implement accounting-boundary ref validation.**
  - Design: Validate sponsor account refs, payer refs, provider earning refs, billing document refs, hold rules, Seal Ledger stream refs, reporting duties, and compatible Overgrant/Overbill refs without owning balances or settlement entries.
  - Output: Accounting-boundary checker, accounting-ref schema, missing/ref-stale reason codes, owner-service handoff notes, and fixtures.
  - Validation: Tests prove missing sponsor/payer/provider-earning refs, unsupported hold rules, stale billing refs, or missing ledger stream refs block activation while Federation Template Service never mutates ORU, invoices, payouts, or Seal Ledger entries.

- **6.2 Implement Overgrant and public-interest allocation handoffs.**
  - Design: Expose template and instance facts that Overgrant and Public-Interest Pool Service can use to create grant-funded, donated, reciprocal, or emergency-sponsored resource allocations.
  - Output: Overgrant handoff contract, public-interest pool handoff refs, purpose-limited allocation facts, quota/fairness refs, and grant compatibility examples.
  - Validation: Integration tests prove Overgrant receives stable refs and remains the grant owner; this service does not create grants, modify grant balances, or decide final allocation fairness.

- **6.3 Implement usage-boundary query.**
  - Design: Add `GET /federation-instances/{instance_id}/usage-boundary` to return the refs downstream services must attach to cross-tenant usage: template id/version, instance id, participant refs, tenant refs, policy refs, grant/accounting refs, purpose tags, dispute refs, and redaction class.
  - Output: Usage-boundary API, response schema, freshness checks, redacted projections, consumer examples, and `usage_boundary_issued` audit events.
  - Validation: Contract tests prove Overguard, Overgrant, Overmeter, Overbill, Overclaim, Oversched, Deployment Planner, and Public-Interest Pool Service can consume usage-boundary refs without copying template policy locally.

- **6.4 Implement dispute-boundary validation.**
  - Design: Require concrete `federation_dispute_boundary` fields before cross-tenant usage: claimant/respondent roles, participant contacts, evidence owner, logs owner, accounting/hold target refs, response SLA, challenge/appeal window, redaction profile, escalation route, Overclaim claim-type refs, and finality behavior.
  - Output: Dispute-boundary checker, missing-field reason codes, Overclaim handoff refs, hold behavior refs, redaction profile, and negative fixtures.
  - Validation: Tests prove missing or stale dispute facts fail with `dispute_boundary_missing` and scheduling cannot proceed on informal operator promises.

- **6.5 Implement dispute, correction, and suspension handoffs.**
  - Design: Publish instance suspension, dispute, hold, correction, appeal, and retirement refs to Overclaim, Overguard, Overgrant, Overbill, Overmeter, Oversched, and Deployment Planner.
  - Output: Suspension propagation events, dispute handoff contract, correction refs, hold refs, audit timeline, and consumer checklist.
  - Validation: Tests prove participant suspension prevents new usage, historical usage stays readable, dispute holds route to owning services, and corrections are recorded by owner-service refs rather than rewritten locally.

## Phase 7: Downstream Integrations And Trusted Partner Swarm Proof

### Work Items

- **7.1 Integrate with scheduling and deployment consumers.**
  - Design: Provide usage-boundary refs and preflight decisions to Oversched, Overlease, Overcell, Overrun, Deployment Planner, Package Validator, Overpack, and Release Strategy Service without owning scheduling, leasing, execution, validation, or deployment graphs.
  - Output: Consumer adapter contracts, candidate/placement input refs, deployment-plan input refs, package/readiness refs, and handoff examples.
  - Validation: Integration tests prove consumers store usage-boundary refs and policy refs instead of duplicating template logic or bypassing Overguard.

- **7.2 Integrate with metering and billing consumers.**
  - Design: Provide instance and accounting-boundary refs to Overmeter, ORU Account Service, Overbill, Provider Payout Service, and Seal Ledger for usage facts, account projections, invoices, payout eligibility, holds, and ledger streams.
  - Output: Metering/billing handoff schema, usage dimension tags, accounting refs, hold refs, and reconciliation examples.
  - Validation: Tests prove cross-tenant usage emits required refs and this service remains read/ref authority only, with no direct pricing, balance, payout, invoice, or ledger mutation.

- **7.3 Prove the first trusted partner swarm template.**
  - Design: Implement a known organization/trusted partner swarm template using verified participants, operator contacts, package-validated workloads, explicit tenant boundary, Overguard policy, Overgrant/Overbill refs, Overclaim refs, and Overwatch audit.
  - Output: Trusted partner swarm template, federation instance, preflight result, usage-boundary refs, report projections, and replay bundle.
  - Validation: Scenario tests prove a known partner can expose only approved low-risk capacity under explicit tenant, policy, accounting, and dispute boundaries.

- **7.4 Prove data-class denial and suspension behavior.**
  - Design: Exercise denied data classes, missing dispute boundary, stale verification, participant suspension, policy conflict, and incorrect template version cases against the trusted partner proof.
  - Output: Negative scenario fixtures, denial events, suspension events, usage-boundary rejection records, and redacted operator diagnostics.
  - Validation: Tests prove user_private, secret_bearing, regulated, and system_service classes do not run under the first proof, suspended participants block new usage, and historical decisions remain replayable.

- **7.5 Publish downstream integration rules.**
  - Design: Document that consumers must attach federation refs to work, usage, billing, grants, disputes, reports, and deployment evidence rather than copying template rules into local logic.
  - Output: Integration rules, consumer checklist, rejection authority table, API examples, event examples, and anti-copying review criteria.
  - Validation: Review confirms every consumer has a clear owner boundary, required refs, freshness expectations, redaction class, and failure behavior.

## Phase 8: Public-Interest Templates, Reports, And Client Surfaces

### Work Items

- **8.1 Implement public-interest and purpose-limited templates.**
  - Design: Build research-only, education-only, nonprofit, emergency/disaster-response, and grant-funded public-interest template variants after the trusted partner swarm lifecycle is proven.
  - Output: Template variants, purpose-tag refs, public-interest pool refs, quota/fairness refs, reporting requirements, and validation fixtures.
  - Validation: Tests prove public-interest templates require Purpose Tag Registry evidence, Public-Interest Pool Service refs, Overgrant refs, abuse controls, and redacted reports before activation.

- **8.2 Implement simulation and dry-run support.**
  - Design: Simulate template approval, instance readiness, workload eligibility, data-class denial, accounting-boundary gaps, dispute-boundary gaps, purpose-tag gaps, suspension, report redaction, and usage-boundary output without side effects.
  - Output: Simulation API, fixture input schema, simulated timeline, missing-prerequisite summaries, expected reason codes, and replay packs.
  - Validation: Tests prove simulation is side-effect-free, replayable, redacted, and useful for operator review without mutating templates, instances, policy, grants, accounts, ledger, scheduling, or deployment truth.

- **8.3 Implement audience-specific report projections.**
  - Design: Provide public, participant-only, operator-only, and Central-AI-only reports with the SDS #52 redaction boundaries for aggregate dimensions, purpose tags, usage rollups, accounting/grant refs, dispute status, evidence obligations, policy decisions, and stewardship summaries.
  - Output: Report APIs, projection schemas, redaction profiles, public report examples, participant report examples, operator report examples, and Central-AI summary examples.
  - Validation: Tests prove public reports hide contacts, payment details, private payloads, provider-private topology, raw secrets, and final decision authority while operator and participant views remain scoped.

- **8.4 Implement SDK and CLI operations.**
  - Design: Add generated client operations for draft template, submit template, approve template, read template, create instance, preflight instance, suspend instance, read usage boundary, simulate, and read reports.
  - Output: SDK operation contracts, Rust SDK hooks, TypeScript/web bindings where needed, CLI command specs, stable JSON examples, and error examples.
  - Validation: Contract tests prove clients pass signed envelopes, trace ids, idempotency keys, schema versions, policy refs, evidence refs, stable reason codes, and redaction rules through generated contracts.

- **8.5 Implement admin/developer UI views.**
  - Design: Provide quiet operational views for templates, template versions, participant roles, capacity scopes, instance readiness, preflight blockers, accounting/dispute boundaries, usage refs, suspension state, reports, and simulation results.
  - Output: Admin UI view contracts, action enablement rules, read projections, role-scoped operator views, participant views, and audit refs for privileged views.
  - Validation: UI contract tests prove disabled actions reflect missing policy/accounting/dispute/purpose evidence, report views redact by audience, and every privileged action emits Overwatch audit refs.

## Phase 9: Operations, Evidence, Redaction, And Governance Hooks

### Work Items

- **9.1 Emit template and instance event streams.**
  - Design: Emit draft, submit, approve, deny, instance created, preflight passed, preflight failed, usage-boundary issued, instance suspended, instance retired, template superseded, and template revoked events with redacted refs.
  - Output: Event schemas, Overwatch event writer, idempotency behavior, retry behavior, audit refs, and event fixtures.
  - Validation: Tests prove every lifecycle transition and preflight emits replayable evidence while tenant/public projections hide sensitive organization contacts, topology, private payloads, and payment details.

- **9.2 Implement operational monitoring and diagnostics.**
  - Design: Track active templates, active instances, pending reviews, blocked instances, suspended participants, stale evidence, accounting-boundary gaps, dispute-boundary gaps, purpose-tag gaps, policy-denial counts, and usage-boundary issuance.
  - Output: Health endpoint, metrics/events, operator diagnostics, stale-evidence query, blocked-instance query, failure-reason dashboard contract, and Overwatch refs.
  - Validation: Tests prove diagnostic views require authorized operator scope, tenant/participant views are redacted, and stale or blocked states emit stable reason-code evidence.

- **9.3 Implement retention, redaction, and audit export.**
  - Design: Retain templates, versions, instance refs, preflight snapshots, usage-boundary refs, participant refs, accounting/dispute refs, report projections, suspension refs, denial reason codes, and audit exports according to template state, incident refs, and compliance boundary rules.
  - Output: Retention policy schema, redaction classifier, audit export schema, incident/compliance pins, expiry scheduler contract, and evidence export.
  - Validation: Tests prove public-interest, dispute, suspension, compliance, and public-report evidence is retained while raw private payloads, raw secrets, provider-private topology, and payment details are not exposed.

- **9.4 Add incident, fraud, compliance, and stewardship hooks.**
  - Design: Hand suspicious usage-boundary patterns, repeated policy denials, participant suspension, dispute escalation, missing evidence, purpose-tag abuse, and report summaries to Fraud Control Service, Incident Response, Compliance Boundary, Stewardship Reporting, and Central AI Service.
  - Output: Governance handoff schema, incident trigger refs, fraud-signal refs, compliance export refs, stewardship summary refs, and Central-AI redacted evidence refs.
  - Validation: Review proves governance hooks are evidence refs and workflow handoffs, not centralization of fraud control, incident response, compliance authority, stewardship publication, or Central-AI decision authority inside Federation Template Service.

- **9.5 Implement Phase 11 separation checks.**
  - Design: Add hard checks and reports showing that unknown public-provider onboarding, public sandbox profiles, public-provider anti-Sybil risk, and public payout controls are Phase 11 owner-service responsibilities, not template shortcuts.
  - Output: Phase 11 separation checklist, public-provider denial fixtures, public-provider handoff refs, and crosswalk notes.
  - Validation: Tests prove unknown public providers cannot instantiate trusted templates and public-provider paths cannot inherit trusted-partner data-class, dispute, accounting, or policy privileges.

## Phase 10: Validation, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack and authority guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, template-owned-public-onboarding, template-owned-scheduling, template-owned-execution, template-owned-grants, template-owned-accounting, template-owned-disputes, template-owned-final-policy, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools, local stubs, owner-service refs, or downstream APIs into Federation Template Service's product boundary.

- **10.3 Validate SDS #52 build-breakdown coverage.**
  - Design: Map every SDS #52 build-breakdown item to this plan: schemas, template lifecycle, instance preflight, owner-service refs, usage-boundary query, admin/CLI summaries, simulation, trusted partner swarm proof, and public-interest template proof.
  - Output: Coverage matrix, API checklist, workflow checklist, fixture checklist, and integration-test targets.
  - Validation: Review proves no SDS #52 build-breakdown item is missing and the plan preserves trusted federation as a Phase 10 service separate from Phase 11 public supply.

- **10.4 Validate SDS, service catalog, master plan, crosswalk, and queue alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, build-plan crosswalk, queue state, queue progress, and build-plan progress.
  - Output: Updated backlinks and index rows for SDS #52, queue state update, queue progress update, and build-plan progress evidence.
  - Validation: JSON validation passes; local link checks pass; queue validation confirms `052-build-plan` is complete, no materialized task is running, and `053-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders source alignment, contracts, schemas, lifecycle, instance readiness, policy/preflight, accounting/dispute boundaries, usage-boundary refs, downstream integrations, trusted partner proof, reports, client surfaces, operations, governance hooks, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.

## Alignment Review

- The sub-build plan keeps Federation Template Service first build work in master Phase 10 because trusted known-participant capacity depends on identity/tenancy, verification/policy, accounting refs, package/deployment readiness, and Phase 10 purpose/public-interest rails.
- The plan keeps trusted federation separate from Phase 11 public-provider onboarding; Public Provider Onboarding, Public Sandbox Profile, Fraud Control Service, Reputation and Anti-Sybil Service, public payout holds, and public low-sensitivity sandboxing remain downstream Phase 11 responsibilities.
- The plan treats Overtenant, Overpass, Overkey, and Oververify as identity, tenancy, signing, and evidence owners; Federation Template Service consumes refs and freshness windows but does not verify identity directly.
- The plan treats Overguard as the final policy decision owner; Federation Template Service provides template and instance facts, required policy refs, and preflight requests without replacing policy evaluation.
- The plan treats Overgrant, ORU Account Service, Overmeter, Seal Ledger, Overbill, and Provider Payout Service as allocation/accounting owners; Federation Template Service stores accounting-boundary refs and usage-boundary refs without mutating balances, invoices, payouts, or ledger entries.
- The plan treats Overclaim as dispute owner; Federation Template Service requires concrete dispute-boundary refs and hands off claims, holds, appeals, corrections, and finality behavior.
- The plan treats Oversched, Overlease, Overcell, Overrun, Deployment Planner, Package Validator, Release Strategy Service, Purpose Tag Registry, and Public-Interest Pool Service as downstream or adjacent owner services; consumers attach federation refs instead of copying template logic.
- The plan preserves master Phase 0 through Phase 13 ordering and uses earlier phases as prerequisites, Phase 10 as the first trusted-federation build, Phase 11 as public-provider separation hardening, and Phase 13 as governance/security/compliance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service/contracts, native Overrid boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.
