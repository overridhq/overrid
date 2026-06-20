# SUB BUILD PLAN #41 - Overgrant

Attached SDS: [docs/sds/accounting/overgrant.md](../sds/accounting/overgrant.md)

## Purpose

This sub-build plan turns SDS #41 into an implementation sequence for Overgrant. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overgrant is a Phase 5 accounting and allocation service that manages grant programs, grant source refs, eligibility rule bundles, purpose-scoped quotas, fairness windows, grant authorization refs, usage/reporting refs, suspensions, revocations, corrections, and replay evidence. It expands in Phase 10 for cross-tenant grants, stewarded purpose tags, and public-interest pools. It must not mint money, mutate ORU balances, append Seal Ledger entries directly, replace Overguard policy decisions, replace Oversched placement, own purpose-tag definitions, execute payouts, create billing/refund records, or create speculative token, NFT, yield, or tradeable grant-market behavior.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #41: Overgrant](../sds/accounting/overgrant.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overgrant plan](../service_catalog/accounting/overgrant.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical schemas, signed envelopes, idempotency, trace ids, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identity refs, Overtenant tenant scope, Overkey signing/service refs, Overgate request discipline, Overwatch audit, Overregistry refs, and Overqueue-safe command context. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies workload, package, lease, scheduler, runner, raw usage, and reservation refs that grant authorizations bind to after policy approval. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Overclaim dispute/correction refs, Oververify evidence, Policy Dry-Run previews, and challenge/trust evidence. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Controls the first build point: local/private grant source refs, eligible parties, purpose scopes, dimensions, quota windows, fairness windows, authorization refs, reporting requirements, abuse controls, and accounting handoffs. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service operation, failover, restore, maintenance, and grid-resident hardening for allocation/reconciliation workers. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase persistence, Overstore replay/report artifacts, Overvault private sponsor/evidence refs, namespace refs, retention, backup/restore, and migration handoffs. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Controls cross-tenant grants, verified purpose tags, trusted federation allocation, public-interest pools, public reporting, and partner-swarm expansion. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider constraints, low-sensitivity workload eligibility, fraud/reputation/payout-hold constraints, and public-node grant-use limits. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies Wallet and Usage Center, admin UI, SDK, CLI, native apps, central AI stewardship interfaces, and beneficiary/sponsor-facing grant views. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance boundaries, incident handoffs, threat-model reviews, stewardship reports, audit exports, retention controls, migration governance, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #41 first build work aligned to master Phase 5, with Phase 10 federation/public-interest expansion and later public-provider, native-app, and governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, customer-count, or external-payment-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 10, 11, 12, and 13 | Attach SDS #41, freeze Overgrant authority, preserve Phase 5 as first build point, and record Phase 10 expansion gates. |
| 2 | Master Phases 0, 1, 4, and 5 | Build Rust contracts, schemas, rule bundles, quota/fairness types, reason codes, fixtures, and replay commitments. |
| 3 | Master Phases 1, 4, and 5 | Implement grant program lifecycle, source validation, activation, rule versioning, admin reads, and audit events. |
| 4 | Master Phases 4 and 5 | Implement evaluation, authorization, Overguard policy integration, purpose scope validation, and bounded batch decisions. |
| 5 | Master Phases 3, 4, and 5 | Implement quota windows, deterministic fairness, authorization consumption, scheduler/budget signals, and abuse throttles. |
| 6 | Master Phases 5, 8, 12, and 13 | Implement usage refs, reporting snapshots, redaction, wallet/sponsor views, central AI summaries, and reconciliation checks. |
| 7 | Master Phases 4, 5, and 13 | Implement suspension, revocation, expiry, correction, dispute, appeal, review, and replay behavior. |
| 8 | Master Phase 10, with Phase 11 constraints | Expand to cross-tenant grants, stewarded purpose tags, public-interest pool templates, federation policy, and public-safe reports. |
| 9 | Master Phases 7, 8, 12, and 13 | Harden operations, replay, recompute, native persistence, grid-resident runtime, governance, compliance, and migration controls. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Overgrant core is a Rust service/module using shared contract types, Tokio for bounded async workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Grant programs, source refs, eligibility rule bundles, quota windows, fairness windows, authorizations, reservations, usage refs, reporting snapshots, suspensions, revocations, corrections, API objects, events, fixtures, and reason-code catalogs use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating endpoints require signed actor or service envelopes, tenant context, sponsor/source refs, beneficiary refs, trace id, idempotency key, schema version, policy refs, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for source evidence fingerprints, replay bundles, report/export bundles, schema fixtures, and deterministic quota/fairness comparison tests.
- Overgrant may later persist program and projection records through Overbase, replay/report artifacts through Overstore, and private sponsor/donor/evidence refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, external grant marketplaces, or external payment providers the platform boundary.
- Phase 5 allows only local/private grant primitives with explicit source refs and non-public purpose scopes. Phase 10 owns stewarded public-interest purpose tags, cross-tenant grant behavior, and public-interest pool reporting.
- Overgrant supplies allocation facts and authorization/report refs. It never mints ORU, mutates ORU projections, appends Seal Ledger entries directly, owns payout execution, creates billing/refund records, replaces Overguard admission, replaces Oversched placement, or defines Purpose Tag Registry taxonomy.
- Planning and implementation must avoid speculative grant tokens, yield mechanics, tradeable coupons, NFT drops, pricing tables, revenue projections, customer-count assumptions, and sponsor-controlled opaque allocation ranking.

## Phase 1: SDS Attachment, Overgrant Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #41.**
  - Design: Link this document from the Overgrant SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/accounting/overgrant.md`, `docs/service_catalog/accounting/overgrant.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #41 returns both the Overgrant SDS and this sub-build plan.

- **1.2 Freeze Overgrant authority boundaries.**
  - Design: Record that Overgrant owns program lifecycle records, source refs, eligibility rule bundles, quota/fairness windows, authorization refs, usage/reporting refs, abuse throttles, suspension/revocation/correction refs, and replay bundles.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overgrant does not own ORU balances, Seal Ledger append-only truth, Overguard final admission, Oversched placement, Purpose Tag Registry definitions, Overbill billing records, Provider Payout execution, or Overclaim finality.

- **1.3 Preserve master Phase 5 as the first build point.**
  - Design: Keep first implementation in Phase 5 for tenant-local/private grant primitives that depend on Phase 0 contracts, Phase 1 identity/audit, Phase 3 execution refs, and Phase 4 policy/trust refs.
  - Output: Phase-gate note that Phase 0 through Phase 4 are prerequisites, Phase 5 is first build, and Phase 10 adds cross-tenant/public-interest behavior.
  - Validation: Review proves this plan does not move public-interest or cross-tenant grants into Phase 5 and does not defer local/private grant primitives behind Phase 10.

- **1.4 Carry forward resolved SDS #41 decisions.**
  - Design: Preserve tenant-local Phase 5 approval scope, central AI stewardship review gates, low-stakes private purpose scopes, beneficiary redaction, deterministic weighted fair share, and public-safe Phase 10 reporting fields.
  - Output: Resolved-decision checklist tied to implementation reviews.
  - Validation: Review rejects cross-tenant Phase 5 grants, stewarded public-interest claims before Phase 10, beneficiary access to raw source accounts, opaque ML allocation, sponsor-controlled ranking, and private data leakage in public reports.

- **1.5 Define owner-service and consumer boundaries.**
  - Design: Create a dependency matrix for ORU Account Service, Seal Ledger, Overguard, Oversched, Overmark, Overclaim, Purpose Tag Registry, Overwatch, Overtenant, Overpass, Overkey, Wallet and Usage Center, Public-Interest Pool Service, native services, central AI stewardship, SDK, CLI, and admin UI.
  - Output: Boundary matrix listing consumed refs, emitted grant refs, final authority owner, redaction class, replay evidence, blocking behavior, and later phase gate.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, trace ids, policy refs, and Overwatch audit rather than direct privileged state reads.

## Phase 2: Rust Contracts, Schemas, Rule Bundles, And Fixtures

### Work Items

- **2.1 Create the Overgrant Rust contract module.**
  - Design: Add contract types for grant programs, source refs, rule bundles, quota windows, fairness windows, authorizations, reservation refs, usage refs, abuse controls, reporting snapshots, replay bundles, API errors, and events.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, rule-kind enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from ORU Account Service, Seal Ledger, Overguard, Oversched, Purpose Tag Registry, Overbill, Provider Payout Service, and Overclaim internals.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for program create/read/activate, rule versioning, evaluate, authorize, consume, suspend, revoke, correction, report, replay, events, and export bundles.
  - Output: Schema files, valid examples, invalid examples, generated validators, compatibility notes, and docs-facing examples.
  - Validation: Schema tests reject missing tenant scope, sponsor/source refs, beneficiary refs, actor/service identity, trace id, idempotency key, purpose scope, resource dimensions, policy refs, state, and audit refs where required.

- **2.3 Define rule, purpose, quota, and reason-code catalogs.**
  - Design: Encode eligible identity/tenant/app/workload classes, local/private purpose scopes, rule inputs, quota dimensions, fairness strategies, denial reason codes, abuse throttle reason codes, reporting scopes, and redaction classes.
  - Output: Catalog files, review tables, schema enums, fixture references, and compatibility notes for clients and audit exports.
  - Validation: Tests prove every catalog entry names its source authority, allowed phase, input facts, redaction profile, replay inputs, and stable reason code.

- **2.4 Model program and authorization state machines.**
  - Design: Encode grant program states from draft through source_pending, policy_review, active, paused, quota_exhausted, expired, suspended, revoked, closed, and corrected; encode authorization states from requested through eligible, denied, reserved, consumed, expired, revoked, and corrected.
  - Output: Transition tables, Rust enums, JSON Schema enums, event emission rules, invalid transition fixtures, and lifecycle review notes.
  - Validation: State tests reject silent edits to beneficiary, purpose, quota, source, resource dimension, reporting scope, or state history.

- **2.5 Create deterministic Overgrant fixtures.**
  - Design: Build fixtures for program creation, source validation, rule versioning, evaluation, authorization, denial, quota exhaustion, fairness throttling, consumption, reporting, suspension, revocation, correction, replay, redaction, and duplicate handling.
  - Output: Fixture directory, expected projections, reason codes, Overwatch events, report examples, replay examples, and invalid examples.
  - Validation: Fixture tests produce stable authorization ids, denial reason codes, quota/fairness decisions, redaction behavior, replay hashes, and idempotency outcomes across repeated runs.

## Phase 3: Grant Program Lifecycle, Source Validation, Activation, And Audit

### Work Items

- **3.1 Implement source-ref validation.**
  - Design: Validate ORU account refs, grant-pool refs, reserve refs, native-service surplus refs, sponsorship refs, donation refs, and stewardship allocation refs without copying balance truth into Overgrant.
  - Output: Source resolver interfaces, source-service allowlist, source-state checks, source freshness rules, and unavailable-source behavior.
  - Validation: Tests reject missing, suspended, stale, cross-tenant, underfunded, unsigned, or wrong-purpose source refs with stable reason codes.

- **3.2 Implement draft program creation.**
  - Design: Create draft grant programs with sponsor/owner refs, tenant scope, source refs, proposed eligible parties, purpose scope, dimensions, quotas, reporting requirements, and governance refs.
  - Output: `POST /grant-programs`, idempotent create behavior, draft state projection, command audit, and `overgrant.program_created` events.
  - Validation: API tests reject creation without signed actor/service identity, tenant context, idempotency key, trace id, source refs, policy refs, and audit refs.

- **3.3 Implement activation flow.**
  - Design: Activate only after source validation, actor authority, tenant authority, Overguard policy checks, rule-bundle readiness, and reporting requirements pass.
  - Output: `POST /grant-programs/{id}/activate`, activation state transition, policy refs, denial reason codes, and `overgrant.program_activated` events.
  - Validation: Tests prove missing source, missing policy, missing authority, public-interest purpose without Phase 10 refs, and cross-tenant scope before Phase 10 keep programs inactive.

- **3.4 Implement rule-bundle versioning.**
  - Design: Attach new eligibility/quota rule versions as append-only bundles with effective windows, migration behavior, replay inputs, and current-version pointers.
  - Output: `POST /grant-programs/{id}/rules`, rule version records, compatibility checks, and `overgrant.rules_versioned` events.
  - Validation: Tests prove existing authorizations keep their original rule version and new rules cannot silently edit prior decisions.

- **3.5 Implement authorized program reads and audit events.**
  - Design: Provide program metadata, state, rule versions, quota windows, reporting status, and redacted source/beneficiary views by role.
  - Output: `GET /grant-programs/{id}`, role-aware redaction profiles, pagination/index fields, metrics counters, and Overwatch audit refs.
  - Validation: Security tests reject raw source-account data, sponsor-private terms, donor metadata, private workload content, payment refs, and fraud/compliance flags in unauthorized reads.

## Phase 4: Eligibility, Policy, Purpose Scope, And Authorization

### Work Items

- **4.1 Implement side-effect-free evaluation.**
  - Design: Evaluate candidate workloads/apps/users against eligibility, tenant scope, purpose scope, rule version, source availability, policy refs, and missing prerequisites without reserving resources.
  - Output: `POST /grant-programs/{id}/evaluate`, deterministic allow/deny/review results, missing-fact reports, and `overgrant.eligibility_evaluated` events.
  - Validation: Tests prove evaluation has no quota side effects and returns stable reason codes for ineligible identity, workload class, purpose, source, jurisdiction, trust, or policy facts.

- **4.2 Implement authorization creation.**
  - Design: Create immutable grant authorizations only after eligibility, quota, fairness, source, purpose, and Overguard policy checks pass.
  - Output: `POST /grant-programs/{id}/authorize`, authorization records, denial records, replay refs, and `overgrant.authorization_created`/`overgrant.authorization_denied` events.
  - Validation: Tests prove authorizations cannot be edited after issuance and denial records carry deterministic reason codes without consuming quota.

- **4.3 Integrate Overguard policy checks.**
  - Design: Send input facts to Overguard for admission policy and preserve policy decision refs, matched-rule refs, dry-run refs, and missing fact refs.
  - Output: Overguard adapter contract, policy input bundle schema, decision refs, and denied/review state handling.
  - Validation: Tests prove Overgrant cannot bypass Overguard, cannot treat missing policy as allow, and cannot broaden grant scope beyond policy results.

- **4.4 Enforce purpose-scope boundaries.**
  - Design: In Phase 5 allow only explicit private/local scopes such as `tenant_internal`, `sponsored_private`, `education_internal`, `research_internal`, `opensource_internal`, `trial_credit`, and `operator_test`; require Phase 10 Purpose Tag Registry refs for stewarded public-interest tags.
  - Output: Purpose-scope validator, phase-gate matrix, reason-code fixtures, and docs-facing examples.
  - Validation: Tests deny science, education, medical, opensource, climate, public infrastructure, emergency, and other public-interest claims before Phase 10 tag/evidence refs exist.

- **4.5 Implement bounded batch evaluation and authorization.**
  - Design: Allow bounded batch evaluation/authorization only for one program, one rule version, one source scope, deterministic item ordering, and per-item reason codes.
  - Output: Batch request/response schema, size limits, partial-denial behavior, idempotency behavior, and audit events.
  - Validation: Tests reject mixed rule versions, unbounded batches, nondeterministic ordering, cross-tenant mixing, and batch failures that hide per-item denials.

## Phase 5: Quota, Fairness, Reservation Binding, Scheduler Signals, And Abuse Controls

### Work Items

- **5.1 Implement quota-window locking and idempotency.**
  - Design: Track resource dimensions, per-window caps, per-party caps, concurrency caps, reset behavior, rollover behavior, and deterministic conflict handling.
  - Output: Quota window engine, idempotency key contract, conflict reason codes, and `overgrant.quota_exhausted` events.
  - Validation: Tests prove quota races return deterministic conflict codes and old authorizations are not edited when a window becomes exhausted.

- **5.2 Implement deterministic fairness windows.**
  - Design: Use replayable weighted fair share with per-party caps, burst limits, cooldowns, starvation-prevention minimums, concentration alerts, and sponsor weight constraints.
  - Output: Fairness engine, input snapshot schema, denial/throttle reason codes, and replay fixtures.
  - Validation: Tests prove the same rule version, quota window, usage refs, and request order reproduce the same allow/throttle/deny decisions without opaque ML allocation.

- **5.3 Implement authorization consumption binding.**
  - Design: Bind a grant authorization to ORU reservation refs or scheduler request refs exactly once, preserving unconsumed/expired behavior when scheduler reservation fails.
  - Output: `POST /grant-authorizations/{id}/consume`, consumed state transition, reservation refs, scheduler failure refs, and `overgrant.authorization_consumed` events.
  - Validation: Tests prove one authorization cannot be consumed twice with different reservation refs and failed scheduler reservations do not silently consume grant quota.

- **5.4 Emit Oversched and Overmark-facing facts.**
  - Design: Provide grant-backed placement/budget facts such as dimensions, quota remaining, priority weight, fairness state, purpose scope, expiry, and policy refs without owning placement or price/rate authority.
  - Output: Scheduler signal schema, Overmark budget preview refs, placement input facts, and stable missing-prerequisite errors.
  - Validation: Integration tests prove Oversched remains final placement authority and Overmark remains bounded resource-card/reference-signal authority.

- **5.5 Implement abuse throttles and challenge requirements.**
  - Design: Attach throttle, suspension candidate, challenge required, review required, or appeal required records based on rule bundles, Overguard facts, Overclaim refs, and fraud/verification signals.
  - Output: Abuse control records, throttle events, review queues, dashboard fields, and reason-code fixtures.
  - Validation: Tests prove abuse controls block new authorizations while preserving old evidence and never mutate Seal Ledger or ORU history directly.

## Phase 6: Usage Reconciliation, Reporting Snapshots, Redaction, And Consumer Views

### Work Items

- **6.1 Link usage refs from Seal Ledger and Overmeter.**
  - Design: Observe grant-backed usage through Seal Ledger, Overmeter rollups, ORU reservation refs, scheduler refs, refund refs, correction refs, and expiry refs without creating accounting truth.
  - Output: Grant usage ref schema, source-ref index, usage observation worker, and `overgrant.usage_observed` audit events.
  - Validation: Tests prove grant usage visibility comes from authoritative refs and Overgrant cannot append ledger entries, mutate ORU projections, or fabricate usage.

- **6.2 Build reporting snapshots.**
  - Design: Reconcile authorized, reserved, consumed, expired, revoked, refunded, corrected, denied, throttled, and blocked totals by reporting period, dimension, program, purpose scope, and audience.
  - Output: `GET /grant-programs/{id}/report`, snapshot worker, report schema, freshness fields, and `overgrant.report_snapshot_created` events.
  - Validation: Tests prove snapshots cite source checkpoints, stale snapshots are marked, and reconciliation gaps become explicit blocker records.

- **6.3 Implement beneficiary, sponsor, steward, and operator redaction.**
  - Design: Apply role-aware redaction so beneficiaries see authorized dimensions, quota window, purpose scope, expiry, restriction summary, and allowed sponsor display refs but not private source/account/sponsor internals.
  - Output: Redaction policy map, view schemas, fixtures for each audience, and stable denial reason codes.
  - Validation: Security tests prove raw source account ids, balances, reserve strategy, payment refs, donor-private metadata, sponsor account history, native-service surplus internals, fraud flags, and compliance flags stay hidden from unauthorized views.

- **6.4 Build wallet, native-service, and central AI summaries.**
  - Design: Provide Wallet and Usage Center, native services, central AI stewardship, SDK, CLI, and admin UI with authorized summaries for allocation, remaining quota, reporting obligations, restrictions, and replay refs.
  - Output: Consumer read contracts, SDK/CLI examples, native-service fields, stewardship summary fields, and audit hooks.
  - Validation: Tests prove consumers cannot infer private refs, mutate grants, override policy, broaden purpose scopes, or turn summaries into billing, payout, or ledger authority.

- **6.5 Implement reconciliation blockers and repair queues.**
  - Design: Detect mismatches between authorizations, reservations, consumed usage, refunds, corrections, expiry, and reports; block unsafe summaries until refs reconcile.
  - Output: Reconciliation job, mismatch reason codes, repair queue records, operator review fields, and report-blocking behavior.
  - Validation: Tests prove mismatches do not silently publish sponsor/stewardship reports and repair actions require signed evidence with Overwatch audit refs.

## Phase 7: Suspension, Revocation, Expiry, Correction, Dispute, And Replay

### Work Items

- **7.1 Implement program suspension and pause behavior.**
  - Design: Suspend or pause all or part of a program using signed operator/stewardship evidence, policy refs, affected scope, existing authorization handling, and reporting impact.
  - Output: `POST /grant-programs/{id}/suspend`, pause/suspension records, scope filters, explanation updates, and `overgrant.program_suspended` events.
  - Validation: Tests prove suspension stops new authorizations while preserving consumed refs, old evidence, and reporting history.

- **7.2 Implement revocation and expiry.**
  - Design: Revoke remaining allocation or specific authorizations and expire programs/authorizations by window without editing historical decisions.
  - Output: `POST /grant-programs/{id}/revoke`, expiry worker, revocation records, downstream effect refs, and `overgrant.program_revoked` events.
  - Validation: Tests prove revoked or expired grants cannot create new reservations, and prior usage/reporting refs remain replayable.

- **7.3 Implement corrections and Overclaim dispute links.**
  - Design: Attach correction refs after dispute, refund, abuse review, source correction, or reporting repair while preserving original authorization and report facts.
  - Output: `POST /grant-programs/{id}/corrections`, correction records, Overclaim refs, replacement/refund refs, explanation changes, and `overgrant.correction_recorded` events.
  - Validation: Tests prove corrections never rewrite old authorizations and replay shows original facts, correction evidence, resulting projection, and downstream effect refs.

- **7.4 Implement appeals and review evidence.**
  - Design: Support challenge-required, appeal-open, appeal-denied, appeal-approved, finality-ref, and review-required states with actor authority, evidence refs, deadlines, and redaction.
  - Output: Appeal/review schema, deadline alerts, review queues, explanation fields, and access controls.
  - Validation: Tests prove appeals cannot bypass active policy, fraud, compliance, purpose-tag, or public-provider restrictions without explicit finality refs.

- **7.5 Implement replay bundles.**
  - Design: Reconstruct program and authorization state from source refs, rule versions, policy refs, quota/fairness inputs, lifecycle transitions, usage refs, report snapshots, and audit events.
  - Output: `GET /grant-authorizations/{id}/replay`, replay bundle writer, hash comparison, mismatch reason codes, and export fields.
  - Validation: Replay tests reconstruct current state from stored refs, detect missing evidence, preserve all old states, and flag mismatches without silently widening allocation.

## Phase 8: Phase 10 Federation And Public-Interest Expansion

### Work Items

- **8.1 Add trusted federation grant source expansion.**
  - Design: Support known universities, companies, research labs, nonprofits, family/community clouds, trusted partner swarms, and legally appropriate public-service partners with explicit identity, operator, resource, policy, and dispute contact refs.
  - Output: Federation source-ref schema, participant template refs, cross-tenant source checks, and eligibility gates.
  - Validation: Tests prove Phase 10 grant sources require federation participant evidence and cannot be used by Phase 5 local/private programs.

- **8.2 Integrate Purpose Tag Registry for stewarded tags.**
  - Design: Require active purpose-tag versions, evidence requirement bundles, review history, policy exports, and replayable validation refs for science, education, medical, opensource, climate, public infrastructure, emergency, and later approved public-interest tags.
  - Output: Purpose Tag Registry adapter, tag-version refs, evidence refs, missing-evidence reason codes, and report fields.
  - Validation: Tests prove public-interest tags deny when tag refs, evidence requirements, review refs, or policy exports are missing.

- **8.3 Build public-interest pool grant templates.**
  - Design: Model pool accounts, contributed resource classes, eligible grantees, quotas, fairness rules, abuse throttles, outcome-report refs, renewal, revocation, and correction notices.
  - Output: Public-interest program templates, template validation, reporting contract, and Public-Interest Pool Service handoff refs.
  - Validation: Integration tests prove Public-Interest Pool Service consumes Overgrant allocation primitives through APIs and does not read private grant storage.

- **8.4 Implement cross-tenant grant policies.**
  - Design: Add explicit tenant/source/beneficiary boundaries, owning policy, log/evidence ownership, dispute routing, settlement refs, and correction ownership for cross-tenant grants.
  - Output: Cross-tenant policy bundle schema, Overguard input facts, Overclaim handoff refs, and federation billing/dispute boundaries.
  - Validation: Tests prove cross-tenant grant attempts are denied before Phase 10 policy refs exist and public-provider use still requires Phase 11 low-sensitivity policy.

- **8.5 Build public-safe reporting profiles.**
  - Design: Publish aggregate fields such as pool id/name, purpose tag refs, source classes, authorized/consumed dimensions, quota status, approved public grantee refs, denial/throttle totals, fairness summaries, accounting checkpoint refs, outcome-report refs, and correction notices.
  - Output: Public report schema, redaction thresholds, report templates, export hashes, and stewardship-report handoff fields.
  - Validation: Tests prove public reports exclude private workload contents, raw evidence, raw account details, payment data, sponsor-private terms, fraud heuristics, private central-AI reasoning, and fields below aggregation thresholds.

## Phase 9: Operations, Replay, Native Persistence, Grid Residency, And Governance

### Work Items

- **9.1 Build operations dashboards and runbooks.**
  - Design: Track program counts by state, authorizations by result/reason, quota remaining/exhausted by dimension/window, throttle/appeal counts, source validation failures, reporting lag, and reconciliation gaps.
  - Output: Metrics schema, dashboard definitions, alert rules, Overwatch aggregation, and runbook links.
  - Validation: Operations tests prove alerts fire for missing sources, quota conflicts, missing policy refs, report lag, reconciliation blockers, and redaction failures.

- **9.2 Harden replay, recompute, and backfill.**
  - Design: Support scoped recompute by program, authorization, tenant, source, rule version, policy version, reporting period, dimension, and evidence checkpoint, plus resumable backfill and mismatch diff reports.
  - Output: Recompute worker, backfill run records, replay comparison model, operator controls, and repair audit refs.
  - Validation: Tests prove recompute is idempotent, bounded, resumable, replayable, preserves old projections during review, and never silently broadens eligibility or quota.

- **9.3 Prepare native Overbase, Overstore, and Overvault persistence handoffs.**
  - Design: Move program/projection records to native Overbase when available, replay/report artifacts to Overstore where appropriate, and private sponsor/donor/evidence refs to Overvault without changing API semantics.
  - Output: Persistence adapter interfaces, migration plan, artifact refs, private-ref handoffs, retention hooks, and export format.
  - Validation: Migration tests prove no conventional database/object/vault boundary becomes canonical, records remain replayable, and private source/evidence refs stay behind owning service access controls.

- **9.4 Prepare grid-resident protected operation.**
  - Design: Package Overgrant as a protected grid-resident system workload with service identity, config contracts, secret/private refs, health checks, failover behavior, restore drills, maintenance mode, replay pause/resume, and signed operator actions.
  - Output: System-service packaging notes, runtime config contract, backup/restore hooks, health endpoints, maintenance controls, and break-glass audit rules.
  - Validation: Grid tests prove restart, failover, restore, replay pause/resume, and maintenance mode preserve append-only history and do not emit stale grant authorizations after recovery.

- **9.5 Add governance, compliance, threat-model, and incident handoffs.**
  - Design: Integrate Compliance Boundary policy refs, incident response refs, threat-model findings, stewardship reporting, migration controls, retention/export policy, region-specific restrictions, and audit exports.
  - Output: Governance checklist, compliance export schema, threat-model test list, incident handoff refs, stewardship report fields, and retention policy.
  - Validation: Governance tests prove high-impact grant changes, stewarded tag approvals, cross-tenant policy changes, correction/replay changes, and projection repairs require signed action, evidence refs, Overwatch audit, and retention-compliant exports.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #41`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, work-item sections, final newline, and tab-free formatting.

- **10.2 Validate tech-stack alignment.**
  - Design: Scan for accidental conventional cloud, SaaS-admin, blockchain/NFT, speculative-market, external-payment, pricing, revenue, customer-count, direct-ledger-mutation, ORU-balance-mutation, and public-interest-before-Phase-10 assumptions.
  - Output: Stack-alignment scan result and any corrected wording.
  - Validation: Scan shows only approved negative-control references, native Overrid service names, or explicit non-choice guardrails from `docs/overrid_tech_stack_choice.md`.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk links.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated docs and link-check evidence.
  - Validation: Local Markdown link validation returns no missing local targets for changed docs.

- **10.4 Validate Overgrant authority and phase gates.**
  - Design: Verify every planned behavior preserves Phase 5 as the first implementation point for local/private grant primitives and Phase 10 as the cross-tenant/stewarded public-interest expansion.
  - Output: Authority-boundary checklist and implementation handoff notes.
  - Validation: Review confirms Overgrant does not own ORU balance truth, Seal Ledger append-only entries, Overguard final admission, Oversched placement, Purpose Tag Registry definitions, Overbill records, Provider Payout execution, Overclaim finality, or speculative grant behavior.

- **10.5 Reindex and verify retrieval handoff.**
  - Design: Refresh Docdex for the new plan and linked docs, then verify search returns the SDS #41 plan and source docs in the aligned result set.
  - Output: Docdex index refresh evidence, retrieval evidence, queue/progress notes, repo memory, and implementation handoff note.
  - Validation: Docdex index succeeds, Docdex search for `SUB BUILD PLAN #41 Overgrant Phase 5 grant authorization Phase 10 public-interest pool` returns the new sub-build plan in the result set, and `docdexd run-tests` blocker is recorded if no test runner is configured.

## Alignment Review

- SDS #41 already contains resolved open-question decisions for tenant-local Phase 5 approval, central AI stewardship review gates, low-stakes private purpose scopes, beneficiary redaction, deterministic weighted fair share, and public-safe Phase 10 reporting. This pass adds the sub-build-plan backlink and does not require SDS content correction.
- The service catalog already matches the SDS and master plan: Overgrant starts in Phase 5 for local/private grant primitives and expands in Phase 10 for federation and public-interest pools. This pass adds the sub-build-plan backlink.
- The master Phase 0 through Phase 13 order remains valid. This pass adds SDS #41 to the per-SDS index and keeps Overgrant in Phase 5 while preserving Phase 10 as the stewarded public-interest and cross-tenant expansion.
- The build-plan crosswalk remains valid. This pass adds SDS #41 to the sub-build-plan index with Phase 5 first-build alignment and Phase 10 expansion.
- The accepted Rust-first/native-Overrid stack remains authoritative. The plan uses Rust, Tokio, shared contracts, canonical JSON plus JSON Schema, signed envelopes, Ed25519, BLAKE3/content hashes, and native Overrid persistence/storage/private-ref boundaries.

## Exit Gate

SUB BUILD PLAN #41 is complete when the new plan is linked from the SDS, service catalog, master build plan, and service-catalog alignment docs; Phase 5 remains the first build point for local/private grant primitives; Phase 10 remains the cross-tenant/stewarded public-interest expansion gate; the plan has 10 phases with well-designed Design/Output/Validation work items; queue/progress docs record the pass; local link and structure validation pass; stack guardrail scans show no accidental conventional-cloud, NFT, speculative-market, external-payment, pricing, revenue, customer-count, ORU-balance-mutation, direct-ledger-mutation, or public-interest-before-Phase-10 drift; and Docdex retrieval can find the new plan with SDS #41 context.
