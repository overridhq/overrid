# SUB BUILD PLAN #38 - ORU Account Service

Attached SDS: [docs/sds/accounting/oru_account_service.md](../sds/accounting/oru_account_service.md)

## Purpose

This sub-build plan turns SDS #38 into an implementation sequence for ORU Account Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

ORU Account Service is a Phase 5 accounting projection service. It owns ORU account metadata, explicit resource dimensions, ledger-derived balance projections, short-lived budget prechecks, wallet/admin read models, statement views, and replay bundles. It derives balances from Seal Ledger entries and signed source refs; it must not become a mutable token ledger, blockchain wallet, speculative currency, pricing engine, Seal Ledger, Overbill, Overgrant, Provider Payout Service, or external payment rail.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #38: ORU Account Service](../sds/accounting/oru_account_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [ORU Account Service plan](../service_catalog/accounting/oru_account_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical schemas, signed envelopes, idempotency, trace ids, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass owner refs, Overtenant scope, Overkey signing/delegation refs, Overgate request discipline, Overregistry refs, Overwatch audit, and Overqueue-safe command context. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overmeter raw usage facts, Overlease reservation context, Overrun execution refs, Overpack workload refs, and private-swarm accounting handoff prerequisites. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard decisions, Workload Classifier facts, Oververify evidence, Policy Dry-Run API prechecks, Overclaim dispute/hold refs, and Challenge Task Service evidence. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Controls the first build point: ORU account model, resource dimensions, state machine, signed rollups, ledger-backed transitions, holds, corrections, refunds, grants, Overmark refs, Overbill handoffs, and service-to-service budget checks. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service operation, failover, restore, maintenance, operator action, and grid-resident hardening for accounting projection services. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase persistence, Overstore statement/replay artifacts, Overvault private/compliance refs, namespace refs, migration, retention, and backup/restore handoffs. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies trusted-federation and public-interest accounting context through Overgrant, federation templates, public-interest pools, purpose tags, and reporting refs. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider earning visibility, public-pool holds, reputation and fraud refs, sandbox/accounting constraints, and public low-sensitivity resource consumption boundaries. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies Wallet and Usage Center, SDK, CLI, admin UI, native app, AI Gateway Router, and central AI consumers of authorized balance, budget, receipt, and statement projections. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance boundaries, incident handoffs, threat-model reviews, stewardship reports, audit exports, retention controls, migration governance, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #38 first build work aligned to master Phase 5, with prerequisite control/trust/execution refs and later storage, public-provider, native-app, and governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, or customer-count drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 8, 11, 12, and 13 | Attach SDS #38, freeze projection authority, preserve Phase 5 as first build point, and record prerequisites plus expansion gates. |
| 2 | Master Phases 0, 1, 4, and 5 | Build Rust contracts, schemas, ORU dimensions, state enums, reason codes, fixtures, and replay commitments. |
| 3 | Master Phases 1, 4, and 5 | Implement owner validation, account lifecycle, account state, delegation, suspension, and closure guardrails. |
| 4 | Master Phases 3, 4, 5, and 8 | Build Seal Ledger-backed projection, transitions, checkpoints, replay bundles, consistency checks, and stale markers. |
| 5 | Master Phases 4 and 5 | Implement reservations, holds, refunds, corrections, grants, earnings, expiry, and double-spend prevention from ledger refs. |
| 6 | Master Phases 4, 5, 6, 9, 11, and 12 | Expose short-lived non-reserving budget prechecks and service-to-service settlement refs for admission, apps, native services, and public-provider flows. |
| 7 | Master Phases 5, 6, 12, and 13 | Build wallet/admin read models, statement views, redaction, account history, exports, and user-facing dimension summaries. |
| 8 | Master Phases 4, 5, 10, 11, 12, and 13 | Integrate Overbill, Overgrant, Overclaim, Provider Payout Service, Overmeter, native apps, central AI, federation, and stewardship consumers without moving their authority into ORU Account Service. |
| 9 | Master Phases 7, 8, and 13 | Harden operations, recompute, mismatch recovery, native persistence, grid-resident behavior, compliance, migration, retention, and threat-model gates. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, accounting authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- ORU Account Service core is a Rust service/module using shared contract types, Tokio for bounded async projection/recompute workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- ORU accounts, owner refs, dimensions, balance projections, transitions, reservations, holds, grants, budget prechecks, wallet views, statements, replay bundles, API objects, events, fixtures, and reason-code catalogs use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating endpoints require signed actor or service envelopes, tenant context, owner refs, trace id, idempotency key, schema version, policy refs, evidence refs, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for ledger checkpoints, projection replay bundles, statement exports, schema fixtures, and deterministic comparison tests.
- The service may later persist account records through Overbase, statement/replay artifacts through Overstore, and private/compliance refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, external accounting SaaS, or external workflow products the platform boundary.
- Balance-changing truth comes from Seal Ledger entries and signed source refs. ORU Account Service projections must not become mutable counters, direct ledger mutation, provider payout mutation, invoice mutation, card charging, external payment handling, or speculative token behavior.
- ORU dimensions remain explicit in admission, reservation, settlement, grant, hold, and dispute logic. Aggregated wallet labels are presentation-only unless Seal Ledger records an explicit transition.
- Budget prechecks are short-lived, non-reserving facts. A later ledger reservation, hold, settlement, refund, correction, grant, account state change, projection checkpoint, or policy/delegation change supersedes the precheck rather than mutating it.
- Planning and implementation must avoid blockchain, NFTs, externally tradable credits, broad fungible dimension conversion, hidden pricing tables, revenue projections, customer-count assumptions, per-operation external payment calls, and conventional SaaS-admin framing.

## Phase 1: SDS Attachment, Projection Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #38.**
  - Design: Link this document from the ORU Account Service SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/accounting/oru_account_service.md`, `docs/service_catalog/accounting/oru_account_service.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #38 returns both the ORU Account Service SDS and this sub-build plan.

- **1.2 Freeze projection authority boundaries.**
  - Design: Record that ORU Account Service owns account lifecycle, account metadata, owner refs, resource dimensions, balance projections, read models, short-lived prechecks, wallet views, statement views, consistency checks, replay bundles, and account-level visibility.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms it does not own Seal Ledger append-only entries, Overmeter usage truth, Overbill invoices/payment refs, Overgrant allocation policy, Provider Payout batches, Overmark reference rates, Overclaim finality, or external payment rails.

- **1.3 Preserve master Phase 5 as the first build point.**
  - Design: Keep first implementation in Phase 5 because ORU accounts require Phase 0 contracts, Phase 1 identity/audit, Phase 3 usage facts, Phase 4 policy/dispute refs, and Phase 5 ledger/accounting primitives.
  - Output: Phase-gate note that Phase 0 through Phase 4 are prerequisites, Phase 5 is first build, and Phases 7, 8, 10, 11, 12, and 13 are expansion or hardening gates.
  - Validation: Review proves this plan does not move ORU accounting before usage/policy evidence and does not defer core ORU account projection behind native apps or public providers.

- **1.4 Carry forward resolved SDS #38 decisions.**
  - Design: Preserve dimension-specific accounting, presentation-only aggregation, classed short-lived prechecks, bounded spending delegation, event-driven projection freshness, durable checkpoints, staged account closure, and tombstone-not-delete semantics.
  - Output: Resolved-decision checklist tied to implementation reviews.
  - Validation: Review rejects silent dimension conversion, long-lived non-reserving authority, delegation from hold/reserve/suspended/closing/revoked/provider-payout accounts, stale wallet views without markers, and destructive account deletion.

- **1.5 Define owner-service and consumer boundaries.**
  - Design: Create a dependency matrix for Seal Ledger, Overmeter, Overbill, Overgrant, Overclaim, Provider Payout Service, Overmark, Overguard, Policy Dry-Run API, Overpass, Overtenant, Overkey, Overwatch, Wallet and Usage Center, native services, AI Gateway Router, central AI stewardship, SDK, CLI, admin UI, and governance services.
  - Output: Boundary matrix listing consumed refs, emitted projection/precheck refs, final authority owner, freshness owner, redaction class, replay evidence, expiry behavior, and later phase gate.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, trace ids, policy refs, and Overwatch evidence rather than direct privileged state reads.

## Phase 2: Rust Contracts, Schemas, ORU Dimensions, And Fixtures

### Work Items

- **2.1 Create the ORU Account Service Rust contract module.**
  - Design: Add contract types for ORU accounts, owner refs, account states, dimensions, balance projections, transition refs, reservation refs, hold refs, grant allocation refs, budget precheck refs, wallet records, statement views, replay bundles, API errors, and events.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, state enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Seal Ledger, Overbill, Overgrant, Provider Payout, Overmark, and Overclaim internals.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for account create/read, balance read, transition read, budget precheck, wallet view, statements, recompute, suspension, account closure, events, replay bundles, and consumer projections.
  - Output: Schema files, valid examples, invalid examples, generated validators, compatibility notes, and docs-facing examples.
  - Validation: Schema tests reject missing owner refs, tenant scope, actor/service account, trace id, idempotency key, dimension, state, ledger checkpoint, policy refs, evidence refs, or redaction class.

- **2.3 Define ORU dimensions and display aggregation rules.**
  - Design: Encode CPU-ORU, GPU-ORU, STOR-ORU, NET-ORU, MEM-ORU, DATA-ORU, and Service-ORU with allowed account types, display precision, conversion restrictions, and presentation-only aggregate labels.
  - Output: Dimension catalog, aggregate display map, compatibility notes, fixture examples, and validation error codes for unsupported conversions.
  - Validation: Tests prove admission, reservation, settlement, grant, hold, and dispute logic remains dimension-specific and no aggregate display label creates fungible cross-dimension balance.

- **2.4 Model account and transition state machines.**
  - Design: Encode account states `requested`, `active`, `restricted`, `suspended`, `closing`, `closed`, and `revoked`, plus transition states `available`, `reserved`, `held`, `spent`, `earned`, `sponsored`, `refunded`, `corrected`, `expired`, and `revoked`.
  - Output: Transition tables, Rust enums, JSON Schema enums, event emission rules, invalid transition fixtures, and lifecycle review notes.
  - Validation: State tests reject direct balance mutation, spend from held/reserved states, new reservations from closing/suspended/revoked accounts, and closure before reservations, holds, grants, statements, and tombstones are reconciled.

- **2.5 Create deterministic accounting fixtures.**
  - Design: Build fixtures for account creation, ledger checkpoint projection, reservations, holds, releases, refunds, corrections, earnings, grants, expiry, delegation, budget prechecks, stale projections, mismatches, closure, and replay.
  - Output: Fixture directory, expected projections, reason codes, Overwatch events, statement examples, redaction examples, and invalid examples.
  - Validation: Fixture tests produce stable projections, state transitions, stale markers, redaction behavior, replay hashes, and double-spend prevention across repeated runs.

## Phase 3: Account Lifecycle, Owner Validation, Delegation, And Closure

### Work Items

- **3.1 Implement account creation and owner validation.**
  - Design: Validate person, organization, app, native service, provider, grant pool, escrow/hold, reserve, and system-service owner refs through Overpass, Overtenant, Overkey, Overregistry, and policy refs.
  - Output: `POST /oru/accounts` handler, owner resolver, tenant-scope validator, state initializer, idempotency behavior, and `oru.account_created` events.
  - Validation: API tests reject unknown owners, wrong tenant scope, missing credentials, unsupported account type/dimension pairs, duplicate idempotency bodies, and unsigned creation commands.

- **3.2 Implement account read, state, and visibility controls.**
  - Design: Enforce owner, tenant, role, data-class, suspension, restriction, provider, grant, operator, and service-account visibility for account metadata and state.
  - Output: `GET /oru/accounts/{account_id}` handler, read model, redaction profiles, stable errors, and `oru.account_state_changed` events.
  - Validation: Security tests reject cross-tenant reads, unauthorized provider earnings views, hidden compliance flags, and raw dispute/fraud/private refs outside authorized roles.

- **3.3 Implement bounded spending delegation.**
  - Design: Allow capped delegation only from eligible active accounts with Overpass/Overtenant authority, Overkey signing refs, Overguard policy refs, dimension limits, purpose scope, expiry, revocation path, and Overwatch audit.
  - Output: Delegation contract, validation helper, capped authority records, revocation hooks, and subdelegation rules.
  - Validation: Tests prove grant pools delegate only grant authorization refs, apps/agents cannot delegate onward unless explicitly narrower, and escrow/hold/reserve/suspended/closing/revoked/provider payout accounts cannot delegate spend authority.

- **3.4 Implement suspension, restriction, reinstatement, and closure commands.**
  - Design: Support signed operator/service actions that suspend, restrict, reinstate, enter closing, close, or revoke accounts with evidence refs and allowed correction/release paths.
  - Output: `POST /oru/accounts/{account_id}/suspend` style state-change handler, closure command model, tombstone refs, and audit events.
  - Validation: State tests prove restricted or suspended accounts cannot spend/reserve except allowed correction/release flows and closure is a staged drain that preserves statements, disputes, wallet views, and replay refs.

- **3.5 Produce account lifecycle timelines.**
  - Design: Join account creation, state changes, delegation changes, policy refs, suspension, closure, ledger checkpoint refs, statements, and Overwatch audit into authorized timelines.
  - Output: Account timeline projection, operator view, wallet-safe view, export fields, and trace links.
  - Validation: Timeline tests prove every lifecycle change cites actor/service identity, tenant context, trace id, idempotency key, evidence refs, and Overwatch audit refs.

## Phase 4: Seal Ledger Projection, Checkpoints, Replay, And Consistency

### Work Items

- **4.1 Build the Seal Ledger checkpoint reader.**
  - Design: Consume ledger checkpoints and append-only entries for reservations, settlements, holds, releases, refunds, corrections, earnings, grants, native service usage, and system-service usage.
  - Output: Ledger reader interface, checkpoint cursor, idempotent entry ingestion, source freshness map, and unavailable-ledger behavior.
  - Validation: Tests prove ledger-unavailable state freezes projection updates, marks projections stale, preserves last checkpoint, and never invents balance-changing state.

- **4.2 Implement balance projection computation.**
  - Design: Compute available, reserved, held, spent, earned, sponsored, refunded, corrected, expired, and revoked balances by original ORU dimension from ledger entries and signed source refs.
  - Output: Projection engine, deterministic ordering, source-ref reducer, projection version, source checkpoint fields, and `oru.balance_projected` events.
  - Validation: Golden tests prove unchanged ledger inputs produce stable projections and negative available, duplicate refs, or inconsistent states trigger review-required mismatch handling.

- **4.3 Implement transition and source-ref indexing.**
  - Design: Index transition refs by account, dimension, ledger entry, source service, workload/app/service ref, dispute ref, grant ref, reservation ref, hold ref, and policy ref.
  - Output: Transition read model, `GET /oru/accounts/{account_id}/transitions`, pagination, redaction, and `oru.transition_seen` events.
  - Validation: Tests prove duplicate transition refs are ignored by idempotency key plus ledger entry id and transition history does not expose private source details to unauthorized readers.

- **4.4 Implement projection checkpoints and wallet freshness.**
  - Design: Refresh active wallet/session projections from Seal Ledger append events within a few seconds, mark visible projections stale after 30 seconds, and checkpoint active accounts every 15 minutes, every 1,000 transitions, statement/export boundaries, or high-impact transitions.
  - Output: Projection checkpoint worker, stale marker model, active-account tracker, event-driven refresh hooks, and checkpoint metrics.
  - Validation: Freshness tests prove active wallet views mark stale when checkpoint age exceeds policy and statements/prechecks refresh to a current checkpoint before acceptance.

- **4.5 Implement replay bundles and mismatch recovery.**
  - Design: Store account id, ledger checkpoint, ledger refs, projection algorithm version, computed projection, mismatch refs, old projection, new projection, and Overwatch audit for replay and repair.
  - Output: `POST /oru/accounts/{account_id}/recompute`, replay bundle writer, mismatch state, repair workflow, and `oru.projection_mismatch` events.
  - Validation: Replay tests reconstruct projections from checkpoint refs, preserve old projections during review, and block spending while negative available or mismatched projections are unresolved.

## Phase 5: Reservations, Holds, Grants, Refunds, Corrections, And Double-Spend Controls

### Work Items

- **5.1 Project reservation refs.**
  - Design: Read reservation ledger refs for workloads, apps, native services, service accounts, system services, and public-provider flows while preserving reservation ownership in the creating service.
  - Output: Reservation projection, `oru.reservation_visible` events, expiry behavior, release conditions, and wallet/admin read fields.
  - Validation: Tests prove reserved ORU cannot be spent again, expired reservations release only through ledger refs, and stale reservations on suspended/closing accounts trigger review alerts.

- **5.2 Project holds and release refs.**
  - Design: Read dispute, payout, compliance, fraud, operator, and review hold refs plus release, denial, correction, and finality refs from owning services.
  - Output: Hold projection, `oru.hold_visible` events, hold timeline, release condition fields, and redaction profiles.
  - Validation: Tests prove held ORU cannot be spent or paid out, hold release denial keeps held state, and provider/wallet views redact sensitive fraud, payout, and dispute internals.

- **5.3 Project refunds and corrections.**
  - Design: Treat refunds and corrections as append-only ledger-backed transitions that reference Overbill, Overclaim, operator, source-service, or compliance evidence.
  - Output: Refund/correction projection, statement summary fields, correction reason codes, replacement refs, and audit events.
  - Validation: Tests prove refunds/corrections never rewrite old transitions and replay can show original, correction, resulting projection, evidence refs, and statement impact.

- **5.4 Project grants and sponsored balances.**
  - Design: Consume Overgrant authorization refs, grant-pool source account refs, beneficiary account refs, purpose scope, dimensions, quota, time window, abuse throttle refs, and reporting refs.
  - Output: Grant allocation projection, sponsored balance fields, grant source/beneficiary links, and reporting handoff fields.
  - Validation: Tests prove grant-funded ORU preserves purpose scope, cannot become broad spend authority, expires or revokes through ledger/Overgrant refs, and reports by original dimension.

- **5.5 Enforce double-spend and dimension integrity checks.**
  - Design: Detect simultaneous reservations, holds, grants, releases, refunds, corrections, expiry, and closure transitions that could overdraw or silently convert dimensions.
  - Output: Integrity checker, negative-balance blocker, duplicate/ref conflict reason codes, review-required state, and operator alerts.
  - Validation: Double-spend tests cover concurrent reservations and holds, cross-dimension display summaries, grant/release race conditions, and closure drains without allowing unavailable balance to be spent.

## Phase 6: Budget Prechecks, Service-To-Service Settlement Refs, And Admission Handoffs

### Work Items

- **6.1 Implement short-lived budget prechecks.**
  - Design: Expose `POST /oru/accounts/{account_id}/budget-prechecks` as a non-reserving fact that cites requested dimensions, projection refs, policy refs, grant refs, expiry, and no-reservation attestation.
  - Output: Budget precheck handler, expiry class policy, idempotency behavior, supersession rules, and `oru.budget_prechecked` events.
  - Validation: Tests prove prechecks do not reserve funds, expire according to wallet/developer/app/high-cost/system/batch classes, and are superseded by newer projections, state changes, holds, grants, or ledger reservations.

- **6.2 Integrate Overguard and Policy Dry-Run accounting facts.**
  - Design: Feed policy/dry-run flows with account state, dimensions, current projection checkpoint, grant coverage, hold/restriction flags, precheck expiry, and missing prerequisite refs.
  - Output: Policy input projection, dry-run accounting fact contract, reason-code mapping, and no-side-effect fixtures.
  - Validation: Integration tests prove dry-run and policy precheck consumers create no queue item, lease, reservation, ledger entry, invoice, payout, refund, or settlement mutation.

- **6.3 Integrate Overlease, Oversched, Overrun, and Overpack admission handoffs.**
  - Design: Provide reservation-precheck refs, expiry, resource dimensions, workload/app/service refs, lease refs, package refs, and failure reason codes to scheduling and execution admission flows.
  - Output: Admission projection, scheduler/runner response fields, reservation prerequisites, and failure fixtures.
  - Validation: Tests prove admission fails closed on expired prechecks, stale projections, held/reserved balance conflicts, suspended/restricted accounts, missing grant purpose scope, or incompatible workload class.

- **6.4 Support low-friction service-to-service settlement refs.**
  - Design: Use preauthorized budget, small usage holds, rollup settlement refs, receipt refs, and statement refs for HTTP 402-style service-to-service usage without per-operation external payment calls.
  - Output: Settlement-precheck contract, service account profile, usage attribution fields, receipt handoff refs, and budget exhaustion behavior.
  - Validation: Tests prove ORU Account Service does not call external payment rails, create Overbill documents, mutate ledger state, or treat prechecks as unlimited spend authority.

- **6.5 Emit usage and projection facts without pricing authority.**
  - Design: Emit account creation, projection, precheck, recompute, stale, mismatch, wallet read, statement read, and integration usage facts for Overmeter/Overwatch reporting.
  - Output: Usage fact schema, metrics counters, Overmeter handoff refs, dashboard fields, and audit export fields.
  - Validation: Tests prove telemetry supports accountability without pricing assumptions, revenue projections, customer-count assumptions, direct accounting mutation, or external payment decisions.

## Phase 7: Wallet, Admin, Statement, History, And Redaction Surfaces

### Work Items

- **7.1 Build wallet-ready balance views.**
  - Design: Provide authorized wallet views with account selector, dimension-specific balances, presentation-only aggregate groups, active reservations, holds, grants, refunds, corrections, receipts, stale markers, and statement refs.
  - Output: `GET /oru/accounts/{account_id}/wallet-view`, wallet display record, aggregate display map, stale status, and redaction profile.
  - Validation: Wallet tests prove detailed views always expose original dimensions and state buckets, aggregate labels remain presentation-only, and stale wallet projections are visible rather than hidden.

- **7.2 Build admin and operator read views.**
  - Design: Provide authorized admin/operator views for account state, lifecycle timeline, projection checkpoint, mismatches, delegation, closure drains, hold/release status, grant refs, statement refs, and owner-service links.
  - Output: Admin read model, operator timeline, access audit events, filters, pagination, and export fields.
  - Validation: Security tests prove admin/operator views require role authorization, log access through Overwatch, and do not expose raw private/payout/fraud/compliance refs beyond authorized scopes.

- **7.3 Build account statements and export refs.**
  - Design: Produce opening projection, closing projection, transition summaries, receipt refs, dispute refs, grant refs, export hash, redaction profile, and current checkpoint assertion for a time window.
  - Output: `GET /oru/accounts/{account_id}/statements`, `oru.statement_created` events, statement export refs, and statement fixtures.
  - Validation: Statement tests prove exports refresh to a current ledger checkpoint, cite source refs, preserve redaction, and can be replayed from ledger checkpoints and transition refs.

- **7.4 Build account history and transition explanations.**
  - Design: Provide user-safe, provider-safe, service-safe, operator, and compliance explanation profiles for transitions, holds, refunds, corrections, grants, prechecks, and closure movement.
  - Output: Explanation projection, reason-code catalog, remediation hints, source-service owner fields, and redaction fixtures.
  - Validation: Redaction tests prove provider/private/dispute/fraud/compliance details are hidden from unauthorized views while enough reason-code detail remains for remediation and appeals.

- **7.5 Support offline/cached client read behavior.**
  - Design: Define read-only cached wallet summaries, receipt refs, statement refs, stale projection markers, reconnect refresh behavior, and blocked live actions for mobile/native clients.
  - Output: Cached read contract, mobile/native app profile, offline stale policy, and live-revalidation requirements.
  - Validation: Tests prove budget prechecks, spend decisions, statement issuance, dispute submission, permission expansion, and accounting-changing actions require live revalidation through owning services.

## Phase 8: Overbill, Overgrant, Overclaim, Payout, Native App, And Stewardship Integrations

### Work Items

- **8.1 Integrate Overbill and receipt/statement handoffs.**
  - Design: Provide account projections, account state, wallet views, statement refs, receipt refs, refund/correction refs, and external payment reference links to Overbill without owning invoices or payment rails.
  - Output: Overbill projection contract, statement handoff, receipt read refs, refund/correction handoff refs, and integration fixtures.
  - Validation: Tests prove Overbill owns invoices, receipts, external payment refs, taxes/compliance metadata, payment intents, and chargebacks while ORU Account Service only projects ledger-backed account state.

- **8.2 Integrate Overgrant and public-interest allocation reads.**
  - Design: Provide source account projection, beneficiary account projection, grant pool state, sponsored balances, purpose scope, quota, expiry, abuse throttle refs, and reporting refs to Overgrant and pool services.
  - Output: Grant consumer contract, public-interest pool projection, federation template accounting refs, and stewardship reporting fields.
  - Validation: Tests prove Overgrant owns grant rules and authorization refs, while ORU Account Service only projects grant-backed balances and reports source/beneficiary state by dimension.

- **8.3 Integrate Overclaim and accounting-impact disputes.**
  - Design: Consume dispute hold, correction, release, refund, finality, and appeal refs from Overclaim and provide affected projection, held/contested summaries, source refs, and redacted wallet/admin views.
  - Output: Dispute projection contract, hold/correction/release state mapping, Overclaim ack fields, and appeal view fields.
  - Validation: Tests prove Overclaim owns claim workflow and finality while ORU Account Service reflects append-only accounting effects only after ledger-backed refs exist.

- **8.4 Integrate Provider Payout Service and public-provider accounting reads.**
  - Design: Provide provider earned, held, restricted, correction, payout-ready, payout-blocked, and statement-ready projections with payout-hold refs and redaction classes.
  - Output: Provider payout projection, payout-read API profile, hold/release reason-code mapping, and public-provider fixtures.
  - Validation: Tests prove Provider Payout Service owns payout batches, external payout status, and payout holds, while ORU Account Service never creates payout mutations or exposes provider-private payout internals broadly.

- **8.5 Integrate native services, AI Gateway Router, wallet, admin UI, SDK, CLI, and central AI.**
  - Design: Provide stable client profiles for account selection, budget checks, usage visibility, grants, holds, receipts, statements, native app costs by dimension, service-account usage, and stewardship summaries.
  - Output: Client read contracts, SDK/CLI examples, admin UI fields, native-app projection schemas, central AI stewardship read schema, and audit hooks.
  - Validation: Client tests prove consumers use stable reason codes and projection refs, cannot mutate balances, cannot bypass live revalidation, and cannot turn summaries into pricing, payout, grant, or ledger authority.

## Phase 9: Operations, Recompute, Native Persistence, Grid Residency, And Governance

### Work Items

- **9.1 Build dashboards, alerts, and operator runbooks.**
  - Design: Track account counts by type/state, projected totals by dimension/state, reservations, holds, mismatches, stale projections, precheck failures, statement generation, closure drains, and source-service health.
  - Output: Metrics schema, dashboard definitions, alert rules, Overwatch event aggregation, and runbook links.
  - Validation: Operations tests prove alerts fire for projection mismatches, negative available balances, double-spend attempts, stale ledger checkpoints, suspended accounts with active reservations, and high precheck expiry failures.

- **9.2 Harden recompute, repair, and backfill.**
  - Design: Support scoped recompute by account, tenant, dimension, checkpoint, source service, and policy version, plus resumable backfill, repair review, and mismatch diff reports.
  - Output: Recompute worker, backfill run records, replay comparison model, operator controls, and repair audit refs.
  - Validation: Tests prove recompute is idempotent, bounded, resumable, replayable, preserves old projections during review, and never silently widens available balances.

- **9.3 Prepare native Overbase, Overstore, and Overvault handoffs.**
  - Design: Move account/projection records to native Overbase when available, statement/replay/export artifacts to Overstore where appropriate, and private/compliance refs to Overvault without changing API semantics.
  - Output: Persistence adapter interfaces, migration plan, artifact refs, private-ref handoffs, retention hooks, and export format.
  - Validation: Migration tests prove no conventional database/object/vault boundary becomes canonical, records remain replayable, and private/compliance refs stay behind owning service access controls.

- **9.4 Prepare grid-resident protected operation.**
  - Design: Package the service as a protected grid-resident system workload with service identity, config contracts, secret refs, health checks, failover behavior, restore drills, maintenance mode, recompute pause/resume, and signed operator actions.
  - Output: System-service packaging notes, runtime config contract, backup/restore hooks, health endpoints, maintenance controls, and break-glass audit rules.
  - Validation: Grid tests prove restart, failover, restore, recompute pause/resume, and maintenance mode preserve append-only projection history and do not emit stale broad spend authority after recovery.

- **9.5 Add governance, compliance, threat-model, and incident handoffs.**
  - Design: Integrate Compliance Boundary policy refs, incident response refs, threat-model findings, stewardship reporting, migration controls, retention/export policy, region-specific restrictions, and audit exports.
  - Output: Governance checklist, compliance export schema, threat-model test list, incident handoff refs, stewardship report fields, and retention policy.
  - Validation: Governance tests prove high-impact account actions, closure drains, delegation changes, correction/replay changes, and projection repairs require signed action, evidence refs, Overwatch audit, and retention-compliant exports.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #38`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, work-item sections, final newline, and tab-free formatting.

- **10.2 Validate tech-stack alignment.**
  - Design: Scan for accidental conventional cloud, SaaS-admin, blockchain/NFT, pricing, revenue, customer-count, mutable-token, external-payment, direct-ledger-mutation, and broad fungible-dimension assumptions.
  - Output: Stack-alignment scan result and any corrected wording.
  - Validation: Scan shows only approved negative-control references or explicit non-choice guardrails from `docs/overrid_tech_stack_choice.md`.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk links.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog detailed-SDS section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated docs and link-check evidence.
  - Validation: Local Markdown link validation returns no missing local targets for changed docs.

- **10.4 Validate accounting authority and consumer handoffs.**
  - Design: Verify every planned behavior preserves Seal Ledger as append-only truth, Overbill as billing/payment-ref owner, Overgrant as grant owner, Provider Payout Service as payout owner, Overclaim as dispute/finality owner, Overmark as reference-rate owner, and Wallet as read-only projection consumer.
  - Output: Authority-boundary checklist and implementation handoff notes.
  - Validation: Review confirms each SDS validation requirement has at least one planned fixture and no consumer can mutate balances through a read model or precheck.

- **10.5 Reindex and verify retrieval handoff.**
  - Design: Refresh Docdex for the new plan and linked docs, then verify search returns the SDS #38 plan and source docs in the aligned result set.
  - Output: Docdex index refresh evidence, retrieval evidence, queue/progress notes, repo memory, and implementation handoff note.
  - Validation: Docdex index succeeds, Docdex search for `SUB BUILD PLAN #38 ORU Account Service ledger-derived balance projection Phase 5` returns the new sub-build plan in the result set, and `docdexd run-tests` blocker is recorded if no test runner is configured.

## Alignment Review

- SDS #38 already contains resolved open-question decisions for dimension aggregation, budget precheck expiry, spending delegation, wallet projection freshness, checkpoints, and staged account closure. No SDS correction is required for this pass beyond linking this build plan.
- The service catalog remains aligned to Phase 5 as the first build phase; this pass adds the sub-build-plan backlink.
- The master Phase 0 through Phase 13 order remains valid. This pass adds SDS #38 to the per-SDS index and keeps ORU Account Service in Phase 5 as the account/balance projection layer between signed usage/ledger refs and wallet/admin/accounting consumers.
- The build-plan crosswalk remains valid. This pass adds SDS #38 to the sub-build-plan index with later storage, public-provider, native-app, and governance hardening gates.
- The accepted Rust-first/native-Overrid stack remains authoritative. The plan uses Rust, Tokio, shared contracts, canonical JSON plus JSON Schema, signed envelopes, Ed25519, BLAKE3/content hashes, and native Overrid persistence/storage/private-ref boundaries.

## Exit Gate

SUB BUILD PLAN #38 is complete when the new plan is linked from the SDS, service catalog, master build plan, and service-catalog alignment docs; Phase 5 remains the first build point; the plan has 10 phases with well-designed Design/Output/Validation work items; queue/progress docs record the pass; local link and structure validation pass; stack guardrail scans show no accidental conventional-cloud, mutable-token, external-payment, or broad-dimension drift; and Docdex retrieval can find the new plan with SDS #38 context.
