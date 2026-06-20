# SUB BUILD PLAN #35 - Policy Dry-Run API

Attached SDS: [docs/sds/trust_policy_verification/policy_dry_run_api.md](../sds/trust_policy_verification/policy_dry_run_api.md)

## Purpose

This sub-build plan turns SDS #35 into an implementation sequence for the Policy Dry-Run API. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Policy Dry-Run API is the side-effect-free policy preview surface for Overrid. It validates declared workload or app-action inputs, assembles safe fact snapshots, calls the same Overguard evaluator used by real admission, and returns allow, deny, blocked, or review-required previews with matched rules, reason codes, expected placement class, secret prerequisites, estimated reservation requirements, missing prerequisites, remediation hints, replay bundles, comparison records, and client-specific redaction profiles. It does not enqueue work, reserve resources, mount secrets, create leases, bill, settle ORU, or become a capability token.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #35: Policy Dry-Run API](../sds/trust_policy_verification/policy_dry_run_api.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machine, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Policy Dry-Run API service plan](../service_catalog/trust_policy_verification/policy_dry_run_api.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, signed envelopes, idempotency, trace ids, stable reason codes, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overtenant, Overregistry, Overkey, Overwatch, Overqueue, identity, tenant, credential, audit, and command primitives used by dry-run requests and reads. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies workload manifests, placement candidates, resource cards, lease context, runner refs, queue boundaries, and private execution facts that dry runs preview before real mutation. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Controls Policy Dry-Run API's first build point for side-effect-free policy previews, Overguard evaluator reuse, reason codes, fact snapshots, missing prerequisites, and dry-run evidence. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies budget, grant, cost-class, resource-card, and reservation-precheck refs while preserving accounting mutation in owning services. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service placement, failover, restore, maintenance, and grid-resident hardening for dry-run and policy services. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase persistence, Overstore evidence artifacts, Overvault private/secret prerequisite refs, namespace route refs, and retention/export handoffs. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies deployment-planner and release-strategy consumers that need bounded batch dry runs before submitting real plans. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Controls public-provider preview limits, public sandbox labels, anti-Sybil/fraud refs, strict low-sensitivity boundaries, and public-safe explanations. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies SDK, CLI, admin UI, wallet/usage, native app, mobile, personal AI, and central AI consumers of dry-run previews, reason codes, and remediation hints. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies stewardship, compliance, incident, threat-model, PIP, retention, audit-export, migration, and scale hardening for policy-preview behavior. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #35 first build work aligned to master Phase 4, with later Phase 5, Phase 7, Phase 8, Phase 9, Phase 11, Phase 12, and Phase 13 gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, or customer-count drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 7, 8, 9, 11, 12, and 13 | Attach SDS #35, freeze dry-run authority, preserve Phase 4 as first build point, and record later accounting, grid, storage, deployment, public, native-app, and governance gates. |
| 2 | Master Phases 0, 1, and 4 | Build Rust contracts, canonical schemas, state enums, stable reason codes, request/response objects, deterministic fixtures, and redaction profiles before policy previews become visible. |
| 3 | Master Phases 1, 3, 4, and 9 | Implement authenticated request intake, declared input normalization, manifest/action validation, idempotency, and bounded batch request validation. |
| 4 | Master Phases 1, 3, 4, 5, and 8 | Assemble side-effect-free fact snapshots from owner-service refs, including package, tenant, quota, trust, budget-precheck, secret-prerequisite, provider eligibility, cache, route, and public-provider facts. |
| 5 | Master Phases 4, 5, 8, and 11 | Call Overguard in dry-run mode with the same evaluator, produce matched rules, reason codes, placement class, required trust class, sandbox profile, egress/cache outcomes, secret prerequisites, and non-authoritative reservation estimates. |
| 6 | Master Phases 1, 4, 7, 8, and 13 | Persist dry-run records, replay bundles, expiry, retention classes, comparison refs, Overwatch events, redaction metadata, and audit projections. |
| 7 | Master Phases 4, 9, 12, and 13 | Expose create, read, explain, replay, compare, batch, and reason-code APIs with bounded workloads and role-aware projections. |
| 8 | Master Phases 6, 9, 12, and 13 | Integrate SDK, CLI, admin UI, native app permission previews, deployment planner, AI-generated deployment flows, personal AI, and central AI consumers. |
| 9 | Master Phases 3, 4, 5, 7, 8, 9, 11, 12, and 13 | Harden cross-service handoffs, grid-resident behavior, storage/retention, public-provider previews, accounting prechecks, admission comparison, and governance controls. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, side-effect controls, redaction, queue state, and implementation handoff gates. |

## Tech Stack Guardrails

- Policy Dry-Run API core is a Rust service/module using shared contract types, Tokio for async fact collection and bounded batch workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Dry-run requests, declared inputs, fact snapshots, dry-run results, matched-rule previews, missing prerequisites, estimated reservations, response objects, comparison records, replay bundles, events, API errors, fixtures, redaction profiles, and reason-code catalogs use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating dry-run APIs require signed actor or service envelopes, tenant scope, idempotency keys, trace ids, schema versions, policy refs, evidence refs, stable reason codes, expiry, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for input digests, fact snapshot commitments, replay bundles, comparison records, and deterministic golden tests.
- The service may later persist records through native Overbase, evidence artifacts through Overstore, and private/compliance refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, or external workflow products the platform boundary.
- Dry-run output may cite quota, grant, budget, cost-class, and reservation-precheck refs, but estimated reservation fields are non-authoritative and cannot reserve, hold, bill, settle, price, refund, correct, invoice, or mutate accounting state.
- Dry-run records are evidence and ergonomics artifacts, not capability tokens. Real admission must re-evaluate current facts unless a future policy explicitly accepts a still-valid comparison under the same actor, tenant, input, policy, fact, and visibility scope.
- User-facing explanations must separate stable reason codes and remediation hints from operator-only internals. Secret values, private payloads, provider-private details, fraud heuristics, topology, quota internals, challenge internals, other-tenant facts, payout material, legal holds, and incident details remain redacted.
- Batch dry runs are bounded by actor, tenant, dependency health, item count, active batch count, workload class, sensitivity class, and public-provider policy. Partial results must be explicit and reason-coded.
- Planning and implementation must avoid blockchain, NFT, speculative token mechanics, pricing tables, revenue projections, customer-count assumptions, per-preview external payment calls, and hidden dependency reads.

## Phase 1: SDS Attachment, Dry-Run Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #35.**
  - Design: Link this document from the numbered Policy Dry-Run API SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/trust_policy_verification/policy_dry_run_api.md`, `docs/service_catalog/trust_policy_verification/policy_dry_run_api.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #35 returns both the Policy Dry-Run API SDS and this sub-build plan.

- **1.2 Freeze Policy Dry-Run API as a preview and explanation authority only.**
  - Design: Record that the service owns dry-run request validation, declared input normalization, side-effect-free fact snapshots, Overguard dry-run calls, response shaping, remediation hints, retention, replay, comparison, and client-specific redaction.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms the service does not own policy authoring, real admission, queue state, scheduler placement, lease reservation, runner execution, secret mounting, grant consumption, ORU mutation, billing, payout, pricing, or settlement.

- **1.3 Preserve master Phase 4 as the first build point.**
  - Design: Keep first implementation in Phase 4 because dry runs need Phase 0 contracts, Phase 1 identity/tenant/audit, Phase 3 manifests and workload context, and Overguard policy semantics before a useful preview can exist.
  - Output: Phase-gate note that Phase 5 supplies accounting refs, Phase 7 grid-resident operation, Phase 8 native persistence/private refs, Phase 9 deployment-plan consumers, Phase 11 public-provider rules, Phase 12 client consumers, and Phase 13 governance hardening.
  - Validation: Review proves this plan does not move reservations, ledger mutation, secret access, queue admission, public-provider broadening, native-app production flows, or governance overrides into Phase 4 prematurely.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #35 decisions for classed retention, stable SDK/CLI response shape, real admission not being skipped, bounded batch sizes, and native app permission-preview language.
  - Output: Resolved-decision checklist tied to SDS #35 open-question answers.
  - Validation: Review rejects long-lived raw sensitive replay bundles, public contracts that leak admin/operator diagnostics, dry-run capability-token behavior, unbounded batches, and native app copy that exposes internal policy graphs.

- **1.5 Define fact-owner and consumer boundaries.**
  - Design: Create a dependency matrix for Overgate, Overguard, Overregistry, Overtenant, Overpack, Package Validator, Overvault, Oververify, Overgrant, Overmark, ORU Account Service, Overbill, Overwatch, Overmesh, Overcache, SDK, CLI, admin UI, native apps, AI Gateway Router, Personal AI Assistant, deployment planner, Overqueue, Oversched, Overrun, Overclaim, and central AI stewardship.
  - Output: Boundary matrix listing consumed refs, emitted dry-run refs, final authority owner, freshness owner, redaction profile, replay evidence, expiry behavior, and later phase gate for each dependency.
  - Validation: Review confirms every handoff uses explicit APIs, versioned facts, signed refs, reason codes, policy versions, trace ids, and Overwatch evidence rather than privileged direct state reads.

## Phase 2: Rust Contracts, Schemas, Reason Codes, And Fixtures

### Work Items

- **2.1 Create the Policy Dry-Run API Rust contract module.**
  - Design: Add contract types for dry-run request, declared input, fact snapshot, dry-run result, matched-rule preview, missing prerequisite, estimated reservation, response object, comparison record, replay bundle, state enums, API errors, and events.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, state enums, reason-code enums, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overguard policy authoring, real admission, queue, scheduler, vault, and accounting internals.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for create, read, explain, replay, compare, batch, reason-code listing, declared input, fact snapshot, result, missing prerequisite, estimated reservation, redacted matched rule, and client response.
  - Output: Schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing actor, tenant, trace id, idempotency key, workload/action type, manifest or action refs, declared sensitivity, policy refs, schema version, expiry, and stable reason codes.

- **2.3 Model dry-run state and replay lifecycle.**
  - Design: Encode submitted, validating, invalid, collecting_facts, evaluating, completed, blocked, expired, replayed, compared, and replay_mismatch states with legal transitions and append-only result versions.
  - Output: State transition tables, Rust enums, JSON Schema enums, event emission rules, and invalid transition fixtures.
  - Validation: State tests reject mutable result rewrites, compare before completed, replay after missing retained refs, completion without policy refs, and exact idempotent retry conflicts.

- **2.4 Define reason-code, prerequisite, and remediation catalogs.**
  - Design: Model stable public codes, developer/admin codes, operator-only wrapper codes, missing-prerequisite types, owner-service refs, remediation hints, severity, redaction class, deprecation state, and publication metadata.
  - Output: Catalog files, generated client fixtures, docs snippets, compatibility rules, and migration notes for wording changes.
  - Validation: Tests prove code identifiers remain stable when text changes and operator-only categories still expose safe wrapper codes, trace ids, support/appeal paths where allowed, and redacted evidence refs.

- **2.5 Create deterministic dry-run fixtures.**
  - Design: Build fixtures for valid private workload preview, invalid manifest, missing secret grant, denied egress, insufficient quota, stale trust signal, public low-sensitivity allow, public-provider secret denial, blocked dependency, batch partial result, replay, compare mismatch, and native app permission preview.
  - Output: Fixture directory, expected API responses, Overwatch events, input digests, fact snapshot hashes, replay bundles, redacted explanations, and invalid examples.
  - Validation: Fixture tests produce stable decisions, reason codes, matched-rule previews, remediation hints, snapshot hashes, replay behavior, and redaction across repeated runs.

## Phase 3: Request Intake, Declared Inputs, And Idempotent Validation

### Work Items

- **3.1 Implement authenticated dry-run create intake.**
  - Design: Support signed `POST /policy/dry-runs` through Overgate with actor identity, tenant scope, app/service account, trace id, idempotency key, request class, declared sensitivity, and workload or action input.
  - Output: Create handler, auth context adapter, request validator, idempotency store interface, dry-run id generation, and `policy_dry_run.requested` events.
  - Validation: API tests reject unauthenticated actors, missing tenant context, unsupported action type, missing trace id, conflicting idempotency body, and attempts to use dry-run ids as admission tokens.

- **3.2 Normalize declared workload and app-action inputs.**
  - Design: Convert manifest refs, inline drafts, package refs, permission previews, resource cards, data class, egress requirements, cache hints, secret refs, provider class, grant/budget refs, and expected execution mode into canonical declared input.
  - Output: Declared input normalizer, input digest, schema-versioned command summary, invalid-input reason codes, and client-safe validation errors.
  - Validation: Tests prove normalization is deterministic, input digests change when meaningful fields change, inline drafts cannot bypass package validation, and unknown sensitivity or workload class fails closed.

- **3.3 Validate manifest, package, and deployment-plan inputs without mutation.**
  - Design: Read Package Validator and Overpack validation refs or validate draft shape locally where allowed, without registering packages, uploading artifacts, enqueueing work, or creating deployment records.
  - Output: Package/manifest precheck adapter, deployment-plan item validator, draft validation result model, and blocked-state mapping.
  - Validation: Tests prove invalid package refs, missing manifest versions, malformed deployment-plan items, and unsupported workload modes return invalid or blocked results without side effects.

- **3.4 Implement bounded batch dry-run intake.**
  - Design: Support `POST /policy/dry-runs:batch` with item limits, actor/tenant active-batch limits, sensitivity-class caps, public-provider stricter limits, asynchronous batch option, idempotency per batch, and per-item result states.
  - Output: Batch handler, batch state model, item limit enforcement, active-batch accounting, partial-result format, and `policy_dry_run.batch_requested` event.
  - Validation: Tests prove Phase 4 synchronous SDK/CLI batches cap at 25, native permission batches cap at 10, deployment-plan async batches cap at 100 under policy, and stricter classes override broad batch limits.

- **3.5 Add request rate, visibility, and replay-safety guards.**
  - Design: Enforce caller visibility before revealing whether an action would be allowed, apply rate limits through Overgate, classify replay retention by sensitivity, and block dry runs that would reveal hidden provider, tenant, secret, fraud, or quota internals.
  - Output: Visibility checker, dry-run rate-limit metadata, sensitivity classification, replay-retention class, and redacted invalid/blocked errors.
  - Validation: Authorization tests prove callers cannot use dry runs to enumerate other tenants, provider internals, secret refs, private quota state, fraud signals, or policy details outside normal visibility.

## Phase 4: Side-Effect-Free Fact Snapshot Assembly

### Work Items

- **4.1 Assemble owner-service fact snapshots.**
  - Design: Collect policy-relevant refs from declared inputs and dependency services using read-only, signed, versioned, tenant-scoped requests with source service, fact version, collected-at timestamp, expiry, and redaction class.
  - Output: Fact snapshot assembler, owner-service adapter registry, fact version map, snapshot hash, dependency health mapping, and `policy_dry_run.fact_snapshot_created` events.
  - Validation: Tests prove fact collection cannot mutate queue, workload, lease, vault, grant, billing, settlement, or provider state and that missing critical facts create blocked results.

- **4.2 Collect identity, tenant, package, and workload facts.**
  - Design: Resolve Overgate, Overtenant, Overregistry, Overpack, Package Validator, and Overqueue-safe context for actor, tenant, membership, suspension, quota scope, package/manifests, workload class, data class, and command eligibility.
  - Output: Identity/tenant/package fact adapter, stale fact detection, invalid-state reason codes, and user-safe missing prerequisite hints.
  - Validation: Tests reject suspended tenants, unknown identities, unauthorized apps, stale package refs, wrong tenant context, missing manifest validation, and incomplete workload classes.

- **4.3 Collect trust, provider, cache, route, and public-provider facts.**
  - Design: Resolve Oververify eligibility refs, public-provider onboarding refs, anti-Sybil/fraud refs where permitted, Overmesh route class, Overcache scope, provider class, trust class, challenge/recheck state, and sandbox profile facts.
  - Output: Trust/provider/cache/route adapter, public-provider redaction rules, trust freshness rules, and provider eligibility blocked-state mapping.
  - Validation: Tests prove public-provider facts cannot satisfy private, regulated, secret-bearing, or system-service previews and public-facing output does not expose fraud heuristics or challenge internals.

- **4.4 Collect secret, private-state, and data-movement prerequisites.**
  - Design: Resolve Overvault secret ref classes, access prerequisites, mount-lease prerequisites, private storage refs, data movement constraints, egress requirements, region/jurisdiction refs, and compliance boundary refs without fetching raw secrets or private payloads.
  - Output: Secret prerequisite snapshot, private-data redaction model, egress/cache prerequisite records, and blocked/denied reason codes.
  - Validation: Tests prove raw secrets are never fetched or stored, secret-bearing workloads cannot be previewed onto public providers, and user-facing hints name owner services without leaking private endpoint details.

- **4.5 Collect accounting precheck facts without reservation.**
  - Design: Resolve grant, cost-class, resource-card, budget, quota, reserve-required, sponsorship, usage, and estimated reservation dimensions from owning services as non-authoritative facts.
  - Output: Budget/grant/quota precheck adapter, estimated reservation input model, no-reservation attestation, and accounting redaction rules.
  - Validation: Tests prove no ORU balances, Seal Ledger entries, Overbill records, Overgrant allocations, Overlease reservations, payout holds, invoices, receipts, prices, or external payment calls are created or mutated.

## Phase 5: Overguard Dry-Run Evaluation And Result Shaping

### Work Items

- **5.1 Call Overguard with the real evaluator in dry-run mode.**
  - Design: Route snapshots into the same deterministic evaluator used for real admission, marking the request as dry_run, side-effect-free, non-capability-token, and same-or-shorter validity than admission decisions.
  - Output: Overguard dry-run adapter, evaluator-version refs, policy bundle refs, matched-rule refs, decision state mapping, and `policy_dry_run.evaluated` events.
  - Validation: Policy parity tests prove dry-run and real admission match when actor, tenant, input, policy, facts, visibility scope, and evaluator version are unchanged.

- **5.2 Shape decision previews and matched-rule summaries.**
  - Design: Convert Overguard output into allow, deny, blocked, review-required, invalid, expired, replayed, or compared response states with matched rules, reason codes, policy refs, evidence refs, and redacted explanations.
  - Output: Dry-run result mapper, matched-rule preview builder, user/developer/admin/operator redaction profiles, and client response object.
  - Validation: Tests prove operator-only rules remain wrapped, user-facing messages stay actionable, and matched-rule summaries never reveal raw provider, fraud, secret, quota, challenge, or other-tenant internals.

- **5.3 Compute expected placement, trust, sandbox, egress, and cache outcomes.**
  - Design: Derive expected placement class, required trust class, provider class restrictions, sandbox profile, egress outcome, cache scope outcome, region/compliance outcome, and recheck or review requirements from policy results.
  - Output: Placement preview fields, trust/sandbox/cache/egress outcome fields, reason-code mappings, and remediation hints.
  - Validation: Tests prove public-provider previews remain restricted to public low-sensitivity classes, denied egress fails closed, stale trust creates blocked/recheck states, and private cache scope cannot widen silently.

- **5.4 Format missing prerequisites and remediation hints.**
  - Design: Map missing manifest, package, tenant quota, grant, budget, secret access, trust, provider eligibility, route, cache, egress, app permission, deployment-plan, and dependency facts to owner-service refs and safe next actions.
  - Output: Missing prerequisite records, remediation hint catalog, SDK/CLI command hints, admin UI link refs, native app copy fields, and ownership metadata.
  - Validation: Tests prove hints are specific enough to act on, cite the owning service, and avoid raw secret values, private payloads, fraud details, topology, payout internals, or unsupported auto-fix behavior.

- **5.5 Compute non-authoritative estimated reservation requirements.**
  - Design: Produce resource dimensions, tentative ORU dimensions, budget/grant refs, reserve-required flags, insufficiency reasons, and no-reservation attestation without creating holds, reserves, bills, prices, or accounting entries.
  - Output: Estimated reservation object, accounting-owner refs, warning fields, and comparison-safe snapshot refs.
  - Validation: Tests prove estimated reservation is clearly labeled non-authoritative and downstream services still require real admission and owner-service reservation before execution.

## Phase 6: Records, Events, Expiry, Retention, And Replay Bundles

### Work Items

- **6.1 Persist dry-run records with expiry and replacement links.**
  - Design: Store dry-run id, request summary, declared input digest, fact snapshot refs, result object, policy refs, evaluator version, expiry, retention class, replay availability, redaction class, and replacement/version links.
  - Output: Dry-run repository interface, append-only record model, lookup indexes, expiry model, and migration hooks for native Overbase later.
  - Validation: Tests prove records are append-only, exact idempotent retries return the same result, changed input creates a new result version, and expired records cannot be relied on for current admission.

- **6.2 Publish Overwatch-compatible dry-run events.**
  - Design: Emit requested, invalid, fact_snapshot_created, evaluated, completed, blocked, replayed, compared, expired, and batch item events with dry-run id, tenant id, policy bundle version, reason codes, fact snapshot refs, and trace id.
  - Output: Event schemas, append-only Overwatch client, retry behavior, failure handling, and event fixture set.
  - Validation: Event tests prove all paths emit required audit events without raw secrets, private payloads, provider-private details, fraud heuristics, or hidden quota/trust internals.

- **6.3 Implement classed retention and redaction storage.**
  - Design: Apply SDS #35 retention rules for default developer records, secret-bearing or regulated records, native app permission previews, provider-sensitive records, comparison evidence, mismatch records, support cases, legal holds, and incident pins.
  - Output: Retention policy table, redaction-class metadata, purge/expiry worker, hold/pin rules, and archive hash behavior.
  - Validation: Tests prove default response records retain for 30 days, sensitive replay bundles default to 7 days unless pinned, comparison evidence defaults to 90 days, and raw secrets or private payloads are never copied into dry-run storage.

- **6.4 Build replay bundles and integrity commitments.**
  - Design: Capture request summary, declared input, fact snapshot refs, Overguard decision refs, response object, event refs, policy/evaluator versions, redaction metadata, and BLAKE3 commitments for deterministic replay.
  - Output: Replay bundle contract, hash commitments, export fixtures, replay availability field, and integrity mismatch state.
  - Validation: Replay tests reproduce decisions from retained refs and mark mismatches as policy integrity or changed-fact events without deleting or mutating original records.

- **6.5 Build dry-run and admission comparison records.**
  - Design: Compare dry-run id to later real admission decision id, matching input refs, changed fact refs, changed policy refs, visibility scope, expiry state, match/mismatch state, and mismatch reason.
  - Output: Comparison record schema, compare worker, admission handoff contract, mismatch events, and support/debug projections.
  - Validation: Tests prove comparison distinguishes unchanged matches, changed inputs, changed facts, changed policy, evaluator drift, expired dry runs, stricter workload class, and visibility mismatch.

## Phase 7: Read, Explain, Replay, Compare, Batch, And Reason-Code APIs

### Work Items

- **7.1 Implement dry-run read APIs.**
  - Design: Support `GET /policy/dry-runs/{dry_run_id}` with authorized input summary, result, expiry, remediation hints, replay/compare availability, redaction class, and trace/evidence refs.
  - Output: Read handler, projection contracts, pagination/version behavior where needed, stale/expired headers, and read audit events.
  - Validation: Authorization tests prove callers see only dry runs they are allowed to know about and cannot use reads to enumerate other tenants, providers, apps, secret refs, or private facts.

- **7.2 Implement explain APIs.**
  - Design: Support `GET /policy/dry-runs/{dry_run_id}/explain` with matched rules, reason codes, policy version, user-safe fact refs, remediation hints, and role-aware redaction for user, developer, admin, operator, steward, auditor, native app, and central AI views.
  - Output: Explain handler, explanation bundle schema, redaction profiles, remediation hint mapper, and export events.
  - Validation: Redaction tests prove explanations remain useful while hiding rule internals, fraud heuristics, provider topology, secret/private-data risk details, other-tenant evidence, legal hold details, and operator notes.

- **7.3 Implement replay APIs.**
  - Design: Support `POST /policy/dry-runs/{dry_run_id}/replay` using retained request, fact snapshot, policy bundle, evaluator version, and replay bundle while labeling replay as historical, not current admission.
  - Output: Replay endpoint, replay result schema, mismatch record, replay events, and replay retention error mapping.
  - Validation: Replay tests prove historical replay never creates current admission authority and blocked retention states return safe errors instead of exposing missing sensitive refs.

- **7.4 Implement compare APIs.**
  - Design: Support `POST /policy/dry-runs/{dry_run_id}/compare` against later real admission decision refs with input/fact/policy/evaluator comparisons and redacted mismatch explanations.
  - Output: Compare endpoint, comparison record, mismatch taxonomy, support/debug views, and admission-link audit event.
  - Validation: Tests prove unchanged dry-run/admission paths match, changed inputs/facts/policy/evaluator produce reason-coded mismatches, and comparison does not mutate admission, billing, queue, lease, or vault state.

- **7.5 Implement reason-code and batch result APIs.**
  - Design: Support `GET /policy/dry-runs/reason-codes` and bounded batch result reads with code metadata, remediation metadata, deprecation state, localization-safe keys, batch item status, partial result markers, and retry guidance.
  - Output: Reason-code listing API, generated client catalog, batch read handler, partial-result schema, and compatibility fixtures.
  - Validation: Client tests prove code identifiers remain stable, batch partials are explicit, and clients can render safe remediation without depending on operator-only details.

## Phase 8: SDK, CLI, Admin UI, Native App, And AI Consumer Profiles

### Work Items

- **8.1 Publish generated Rust SDK and CLI contracts.**
  - Design: Generate Rust SDK bindings first and CLI output schemas for dry-run create, read, explain, replay, compare, batch, reason-code listing, input digest display, and stable JSON output.
  - Output: Rust SDK methods, CLI commands, example outputs, golden fixtures, and compatibility tests.
  - Validation: Client tests prove SDK/CLI carry signed envelopes, idempotency keys, trace ids, stable reason codes, expiry, dry-run warning labels, and no privileged bypass around Overgate or Overguard.

- **8.2 Publish TypeScript/web bindings only for client surfaces.**
  - Design: Generate TypeScript bindings for web UI, native app web surfaces, adapters, and admin/developer UI from the same contracts without moving core runtime logic into TypeScript.
  - Output: Generated TypeScript types, UI projection notes, schema compatibility tests, and no-privileged-client guardrails.
  - Validation: Tests prove TypeScript clients cannot access operator-only fields, cannot create real admissions from dry-run ids, and cannot mutate queue, vault, lease, grant, or accounting state.

- **8.3 Implement admin and developer UI projections.**
  - Design: Support developer/admin views for input summary, decision preview, reason codes, missing prerequisites, remediation links, matched-rule summaries, replay/compare state, batch progress, and dry-run/admission mismatch diagnostics.
  - Output: UI data model, projection contracts, access rules, link refs, table/filter fields, and redaction notes.
  - Validation: UI tests prove developer views are actionable while hiding operator-only details and admin views remain scoped by tenant, app, service account, provider, and policy permissions.

- **8.4 Implement native app permission-preview profiles.**
  - Design: Shape native app copy around user-visible effect, permission/data/egress/trust/resource classes, allow/deny/block/review state, fix path, and no-work-started/no-secret-mounted/no-reservation/no-billing guarantees.
  - Output: Native app response profile, mobile-safe fields, localization-safe reason keys, copy fixtures, and permission preview examples.
  - Validation: Tests prove native copy avoids internal policy graphs, rule ids, provider internals, fraud heuristics, topology, quota details, challenge evidence, and misleading approval guarantees.

- **8.5 Integrate AI-generated deployment and stewardship consumers.**
  - Design: Provide AI Gateway Router, Personal AI Assistant, deployment planner, release strategy, central AI stewardship, and migration tooling with bounded dry-run batches, reason-code summaries, safe remediation, and replay/compare refs.
  - Output: AI/deployment consumer contracts, batch item fixtures, central AI redaction profile, and command-generation guardrails.
  - Validation: Tests prove AI flows display or log machine-readable reasons, do not silently bypass denied rules, respect batch limits, and cannot convert dry-run previews into real commands without normal admission.

## Phase 9: Cross-Service Handoffs, Public-Provider, Grid, Storage, And Governance Hardening

### Work Items

- **9.1 Integrate Overgate and real admission comparison.**
  - Design: Route dry-run requests through Overgate, expose dry-run ids to later Overgate admission comparison, and preserve real admission as the mandatory current-fact evaluation path.
  - Output: Overgate adapter, admission comparison handoff, dry-run id evidence fields, idempotency propagation, and fail-closed error mapping.
  - Validation: End-to-end tests prove real admission re-evaluates current facts and records dry-run id only as comparison evidence unless future policy explicitly allows a still-valid same-scope comparison.

- **9.2 Integrate Overguard, Oververify, Overvault, and Overclaim.**
  - Design: Consume Overguard evaluations, Oververify trust/eligibility refs, Overvault secret prerequisite refs, and Overclaim dispute/comparison refs without moving their authority into Policy Dry-Run API.
  - Output: Consumer/producer adapters, freshness rules, owner-service error mapping, replay evidence refs, and redaction profiles.
  - Validation: Integration tests prove owner services retain authority for policy evaluation, trust signals, secret access, claims, disputes, holds, appeals, and finality.

- **9.3 Integrate accounting precheck owners without mutation.**
  - Design: Consume Overgrant, Overmark, ORU Account Service, Overbill, Seal Ledger, Overmeter, and Overlease refs for budget, grant, cost-class, quota, usage, and reservation prerequisites while preserving mutations in owning services.
  - Output: Accounting precheck contract, no-reservation attestation, owner-service links, missing prerequisite hints, and no-mutation tests.
  - Validation: Tests prove dry runs never create holds, reserves, invoices, receipts, payments, refunds, payouts, corrections, ledger entries, prices, or external payment calls.

- **9.4 Harden public-provider, deployment, and grid-resident use.**
  - Design: Apply Phase 11 public low-sensitivity constraints, Phase 9 deployment-plan batch previews, and Phase 7 grid-resident system-service operation with health/readiness, failover, restore, maintenance mode, and rollback behavior.
  - Output: Public-provider preview matrix, deployment batch profile, grid-resident service manifest, health/readiness contract, failover/restore fixtures, and public-safe explanation variants.
  - Validation: Tests prove public-provider previews cannot widen eligibility, deployment batches remain bounded and idempotent, and grid-resident failures return blocked/retryable states without stale or hidden facts.

- **9.5 Add Phase 8 persistence and Phase 13 governance controls.**
  - Design: Prepare native persistence through Overbase, evidence artifacts through Overstore, private/compliance refs through Overvault, retention/export through governance policy, incident response hooks, threat-model checks, PIP-governed changes, and audit exports.
  - Output: Persistence migration contract, retention/export schemas, governance workflow refs, incident handoff notes, threat-model checklist, and public-safe aggregate reporting.
  - Validation: Governance tests prove dry-run retention, replay, comparison, redaction, incident, and policy-change behavior have signer, policy version, reason codes, evidence refs, retention class, and audit-export paths.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Run contract, API, and fixture validation.**
  - Design: Validate create, read, explain, replay, compare, batch, reason-code listing, request schemas, fact snapshots, result schemas, missing prerequisites, estimated reservations, events, and replay bundles.
  - Output: Contract tests, API tests, invalid-fixture tests, compatibility reports, generated binding checks, and fixture snapshots.
  - Validation: Tests pass for required fields, signed envelopes, idempotency, trace ids, tenant scope, policy refs, fact refs, evidence refs, stable reason codes, expiry, redaction profiles, and schema-version compatibility.

- **10.2 Run parity, side-effect, replay, and comparison validation.**
  - Design: Exercise dry-run versus real admission parity, no-mutation behavior, dependency blocked states, replay, compare, batch partials, expired records, changed facts, changed policy, and evaluator drift.
  - Output: Parity suite, no-side-effect suite, replay suite, compare suite, batch suite, and mismatch taxonomy fixtures.
  - Validation: Tests prove dry-run and admission match under unchanged inputs, dry runs create no queue/lease/vault/grant/accounting mutations, replay is historical, and mismatches are reason-coded.

- **10.3 Run security, privacy, and redaction negative controls.**
  - Design: Prove unauthorized actors cannot use dry runs to infer hidden facts, raw secrets cannot enter dry-run storage, public-provider and fraud details stay redacted, and dry-run ids cannot authorize real work.
  - Output: Authorization tests, redaction tests, no-secret-storage scan, no-capability-token scan, hidden-read scan, and operator-only reason-code scan.
  - Validation: Negative tests fail closed and explanations remain useful without exposing private provider data, fraud heuristics, secrets, tenant-private facts, compliance/legal detail, challenge internals, or operator notes.

- **10.4 Validate documentation, phase alignment, and queue state.**
  - Design: Check this plan against SDS #35, the Policy Dry-Run API service catalog entry, master Phase 0 through Phase 13 order, service_catalog_alignment, Phase 4, Phase 5, Phase 7, Phase 8, Phase 9, Phase 11, Phase 12, Phase 13, and `docs/overrid_tech_stack_choice.md`.
  - Output: Link-check results, phase-table verification, work-item count verification, stale-note scan, stack-guardrail scan, Docdex search evidence, queue JSON update, and queue progress update.
  - Validation: Validation proves the plan has 10 phases numbered 1 through 10, five work items per phase, Design/Output/Validation fields, no external product-boundary drift, no pricing/revenue/customer-count drift, and no required master phase reordering.

- **10.5 Prepare implementation handoff.**
  - Design: Convert this documentation into build-entry criteria for contracts, request intake, fact snapshots, Overguard dry-run calls, result shaping, records/events/retention, replay/compare/batch APIs, clients, cross-service hardening, and validation.
  - Output: Implementation gate checklist, dependency readiness checklist, risk register, first coding-task candidate list, owner-service handoff checklist, and open engineering choice notes.
  - Validation: Handoff review confirms Phase 4 work can start after Phase 0 through Phase 3 prerequisites exist, Phase 5 accounting actions remain owner-service effects, Phase 7/8/9/11/12/13 expansions remain gated, and future builders can implement without reinterpreting SDS #35.
