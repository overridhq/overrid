# SUB BUILD PLAN #32 - Overclaim

Attached SDS: [docs/sds/trust_policy_verification/overclaim.md](../sds/trust_policy_verification/overclaim.md)

## Purpose

This sub-build plan turns SDS #32 into an implementation sequence for Overclaim. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overclaim is the append-only dispute, evidence, hold, remedy proposal, appeal, and finality coordination layer for Overrid. It records claims and downstream action refs, but it does not rewrite usage, ledger, billing, payout, verification, namespace, or native-app records in place.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #32: Overclaim](../sds/trust_policy_verification/overclaim.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, accounting boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overclaim service plan](../service_catalog/trust_policy_verification/overclaim.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, signed command envelopes, idempotency, trace ids, stable reason codes, local fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies identity, tenant, credential, Overgate, Overwatch, Overqueue, service-account, audit, and command primitives used by claim intake. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies workload, queue, schedule, lease, runner, result, timeout, cancellation, and raw metering refs that execution claims cite. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Controls Overclaim's first build point for record-only claims, evidence links, dispute windows, hold requests, and policy/finality decisions. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes Overclaim hold, release, refund, correction, payout-hold, and settlement refs while keeping accounting mutation in the owning services. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase, Overstore, Overvault, namespace, route, and encrypted evidence-ref expansion. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Controls public-provider dispute, fraud, challenge, appeal, and payout-hold expansion under low-sensitivity constraints. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies native app claim clients, user-facing dispute views, wallet/usage center handoffs, and app-service dispute classes. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies incident, compliance, retention, threat-model, stewardship, audit-export, appeal, and migration hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #32 first build work aligned to master Phase 4, with Phase 5 settlement integration and later public, native-app, and governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid storage/evidence/accounting boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, or customer-count drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 8, 11, 12, and 13 | Attach SDS #32, freeze Overclaim authority, preserve Phase 4 as first build point, and record later accounting, storage, public, native-app, and governance gates. |
| 2 | Master Phases 0, 1, 3, and 4 | Build Rust contracts, JSON Schemas, state machines, deadlines, reason codes, and deterministic fixtures before claim side effects. |
| 3 | Master Phases 1, 3, and 4 | Implement claim intake, party validation, affected-ref validation, idempotency, admission, and authorized reads from existing execution and trust facts. |
| 4 | Master Phases 1, 3, 4, 8, and 13 | Bind evidence refs, statements, redaction profiles, Overwatch audit bundles, Overvault-compatible secret refs, and export-safe visibility. |
| 5 | Master Phases 4 and 5 | Implement challenge windows, hold requests, finality checks, downstream hold acknowledgements, and expiration behavior without accounting mutation. |
| 6 | Master Phases 4, 5, and 13 | Implement resolution proposals, final resolutions, appeals, withdrawals, correction/refund/release refs, and finality markers. |
| 7 | Master Phases 4, 5, 8, 11, and 12 | Hand off claim outcome refs to Oververify, accounting, namespace, public-provider, and native-app consumers through explicit APIs and events. |
| 8 | Master Phases 7, 8, and 13 | Harden persistence, replay bundles, recovery, retention, derived indexes, backup/restore, and grid-resident operational readiness. |
| 9 | Master Phases 11, 12, and 13 | Expand public-provider, abuse, fraud, native-app, governance, compliance, stewardship, and reporting flows after their owning phases exist. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, phase boundaries, negative controls, queue state, and implementation handoff gates. |

## Tech Stack Guardrails

- Overclaim core is a Rust service/module using shared contract types, Tokio for async workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Claim records, parties, evidence links, statements, challenge windows, hold requests, correction proposals, refund proposals, resolution records, appeal records, audit exports, replay bundles, events, API errors, fixtures, and reason codes use canonical JSON plus JSON Schema for docs-facing and test fixtures. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating APIs require signed actor or service envelopes, tenant scope, idempotency keys, trace ids, schema versions, policy refs, evidence refs, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for evidence commitments, statement attachments, replay bundles, audit exports, downstream refs, and finality reports.
- Overclaim stores or references protected evidence through native Overbase, Overstore, and Overvault boundaries when those phases are ready. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, or external workflow products the platform boundary.
- Overclaim is a coordinator, not the mutator of accounting, payout, usage, verification, namespace, trust-score, or native-app service state. Owning services execute effects from signed refs and return immutable response refs.
- Evidence visibility is separate from claim visibility. Provider, user, tenant, operator, steward, auditor, native-app, and public-report views must use explicit redaction profiles.
- Public-provider dispute behavior stays gated until Phase 11 low-sensitivity public rails, fraud controls, reputation controls, payout holds, and sandbox limits exist.
- Native-app dispute behavior stays gated until Phase 12 clients and app-service evidence refs exist.
- Planning and implementation must avoid blockchain, NFT, speculative token mechanics, pricing tables, revenue projections, customer-count assumptions, or per-claim external payment calls. Cost, holds, refunds, corrections, and payouts are represented through ORU, Seal Ledger, Overbill, Overmeter, Provider Payout Service, Overgrant, and Overasset refs.

## Phase 1: SDS Attachment, Overclaim Authority, And Finality Gates

### Work Items

- **1.1 Attach the build plan to SDS #32.**
  - Design: Link this document from the numbered Overclaim SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/trust_policy_verification/overclaim.md`, `docs/service_catalog/trust_policy_verification/overclaim.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #32 returns both the Overclaim SDS and this sub-build plan.

- **1.2 Freeze Overclaim as dispute coordination, not effect mutation.**
  - Design: Record that Overclaim owns claim intake, party records, evidence links, challenge windows, hold requests, remedy proposals, resolution records, appeals, finality markers, replay bundles, and audit exports.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overclaim does not own balance changes, payout execution, usage rollup mutation, trust-score updates, scheduler placement, namespace mutation, native-app service delivery, fraud scoring, or support inbox behavior.

- **1.3 Preserve master Phase 4 as the first build point.**
  - Design: Keep first implementation in Phase 4 because record-only claims require Phase 3 execution/metering refs and Phase 4 policy/evidence/trust context before they can safely influence settlement.
  - Output: Phase-gate note that Phase 5 executes accounting effects, Phase 8 expands storage/private evidence refs, Phase 11 expands public-provider disputes, Phase 12 expands native-app disputes, and Phase 13 hardens governance.
  - Validation: Review proves this plan does not move refunds, ledger corrections, payout releases, native-app billing adjustments, namespace mutations, or public-provider decisions into Phase 4.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #32 decisions for Phase 4 record-only claim classes, pre-finality automatic holds, policy-versioned windows, long-term retention boundaries, and abuse controls that preserve severe-claim access.
  - Output: Resolved-decision checklist tied to SDS #32 open-question answers.
  - Validation: Review rejects unsupported claim types without owner refs, post-finality automatic mutation, hard-coded client windows, raw secret retention in Overclaim, and blanket dispute-right loss for abusive claimants.

- **1.5 Define Overclaim dependency boundaries.**
  - Design: Create a dependency matrix for Overgate, Overguard, Overwatch, Overmeter, Oververify, Challenge Task Service, Seal Ledger, Overbill, ORU Account Service, Provider Payout Service, Overpass, Overtenant, Universal Namespace Service, Overbase, Overstore, Overvault, fraud controls, incident response, SDK, CLI, admin UI, and native apps.
  - Output: Boundary matrix listing consumed refs, emitted refs, final authority owner, retry owner, redaction profile, replay evidence, finality gate, and later phase gate for each dependency.
  - Validation: Review confirms every handoff uses explicit APIs, refs, reason codes, policy versions, signatures, and Overwatch evidence rather than privileged direct state writes.

## Phase 2: Rust Contracts, Schemas, State Machines, And Fixtures

### Work Items

- **2.1 Create the Overclaim Rust contract module.**
  - Design: Add contract types for claim records, parties, evidence links, statements, challenge windows, hold requests, correction proposals, refund proposals, resolution records, appeal records, audit exports, replay bundles, state enums, API errors, events, and reason codes.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, stable reason-code catalog, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from accounting, payout, verification, namespace, fraud, and native-app internals.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for claim create/read, party, evidence attachment, statement, hold request, challenge window, resolution proposal, final resolution, appeal, withdrawal, explain response, export request, and replay bundle.
  - Output: Schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing actor identity, tenant id, affected refs, claim type, requested remedy, policy version, trace id, idempotency key, evidence refs, visibility class, and stable reason codes where required.

- **2.3 Define claim-class and remedy taxonomy.**
  - Design: Model execution, usage-rollup, challenge-result, provider/node eligibility, policy-admission, settlement, payout-hold, namespace, native-app, public-provider, fraud, abuse, and incident-linked claim classes with allowed remedies.
  - Output: Claim-class catalog, remedy matrix, owner-service mapping, unsupported-class denial codes, and phase-gate metadata.
  - Validation: Tests prove early Phase 4 intake accepts only claim classes with stable affected refs and returns audit-logged unsupported or not-yet-integrated reason codes for premature billing, native-app, namespace, or public-provider classes.

- **2.4 Define claim and accounting-effect state machines.**
  - Design: Model draft, submitted, triaging, rejected, evidence_open, hold_requested, held, under_review, challenge_window, resolution_proposed, resolved, appealed, final, withdrawn, and expired claim states plus none, hold_pending, hold_active, release_pending, correction_pending, refund_pending, effect_recorded, and effect_denied accounting-effect states.
  - Output: State transition table, legal transition rules, terminal/overlay state semantics, finality marker rules, and event refs.
  - Validation: State tests reject holds after finality without manual stewardship review, resolution without evidence refs, appeal outside the policy window, and history rewriting after finality.

- **2.5 Create deterministic fixtures and harness scenarios.**
  - Design: Build fixtures for private workload dispute, signed usage-rollup dispute, challenge-result dispute, policy denial dispute, provisional settlement hold, payout hold, duplicate claim, abusive claim, public-provider gate, native-app gate, secret-bearing evidence, and post-finality correction proposal.
  - Output: Fixture directory, expected API responses, events, Overwatch refs, downstream stub refs, redacted explain responses, and replay hashes.
  - Validation: Fixture tests produce stable output and prove rejected, held, resolved, appealed, expired, duplicate, abusive, and finality-blocked paths remain append-only.

## Phase 3: Claim Intake, Party Validation, And Authorized Reads

### Work Items

- **3.1 Implement claim create and dry-run APIs.**
  - Design: Support signed `POST /claims` and dry-run intake for execution, usage, challenge, trust, policy, settlement, and payout-hold claim classes that have stable affected refs.
  - Output: Intake handlers, dry-run response, request validators, idempotent accepted/rejected responses, and `overclaim.claim_opened` or `overclaim.claim_rejected` events.
  - Validation: API tests reject unauthorized actors, missing tenant context, unsupported claim classes, missing affected refs, stale claim windows, conflicting idempotency keys, and requested remedies outside the class matrix.

- **3.2 Validate parties and affected refs.**
  - Design: Resolve opener, respondent, tenant, provider, node, workload, usage rollup, challenge result, policy decision, settlement candidate, payout item, namespace, and native-app refs through owning services.
  - Output: Party validator, affected-ref validator, owner-service adapters, source snapshots, missing-ref denial records, and traceable dependency errors.
  - Validation: Tests prove Overclaim does not invent parties or affected facts, and unavailable owner services produce retryable blocked or rejected states with reason codes.

- **3.3 Integrate Overguard claim admission.**
  - Design: Submit actor role, tenant scope, affected refs, claim class, requested remedy, evidence visibility, secret-bearing flags, compliance class, and claim window to Overguard before creating runnable claim state.
  - Output: Overguard admission client, policy refs, matched rule refs, deny reason mapping, and audit-safe dry-run output.
  - Validation: Policy tests prove private data, cross-tenant records, regulated evidence, secret-bearing attachments, public-provider fraud details, and finality-affecting remedies deny or require manual review as policy dictates.

- **3.4 Implement authorized claim reads.**
  - Design: Return claim state, parties, deadlines, evidence summaries, hold status, downstream refs, reason codes, and redacted explanations according to role and visibility class.
  - Output: `GET /claims/{claim_id}`, list/query surface, pagination, role-aware projection, redaction metadata, and read audit events.
  - Validation: Authorization tests prove openers, respondents, tenant admins, providers, operators, stewards, auditors, native apps, and public-report views see only allowed fields.

- **3.5 Implement duplicate and canonical-claim handling.**
  - Design: Detect duplicate affected refs, duplicate idempotency keys, repeated bad-faith filings, and linked claims while preserving one canonical claim when policy allows.
  - Output: Duplicate-link records, canonical claim refs, cooldown records, abuse markers, and explanation fields.
  - Validation: Tests prove duplicate claims do not create duplicate holds or remedy proposals, and severe safety, payout, private-data, account, and finality claims still have an authenticated intake path.

## Phase 4: Evidence Links, Statements, Redaction, And Overwatch Binding

### Work Items

- **4.1 Implement evidence attachment and party statements.**
  - Design: Accept evidence refs and statements with source service, source record ref, evidence kind, integrity hash, retention class, redaction profile, visibility scope, submitted-at, and signer metadata.
  - Output: `POST /claims/{claim_id}/evidence`, party statement records, attachment refs, validation errors, and `overclaim.evidence_attached` events.
  - Validation: API tests reject raw secret payloads, missing source refs, weak integrity fields, unauthorized submitters, closed claim states, and evidence outside the allowed window.

- **4.2 Bind Overwatch evidence bundles.**
  - Design: Link claim timelines to Overwatch traces, audit events, incident refs, usage refs, challenge refs, policy decision refs, downstream action refs, and export integrity records.
  - Output: Overwatch evidence client, evidence bundle refs, claim timeline projection, and replay bundle shell.
  - Validation: Replay tests prove a claim timeline can be reconstructed from Overclaim records and Overwatch refs without raw private workload payloads.

- **4.3 Implement redaction profiles and visibility classes.**
  - Design: Separate claim visibility from evidence visibility using opener, respondent, provider, tenant, operator, steward, auditor, native-app, public-report, and fraud-sensitive profiles.
  - Output: Redaction profile catalog, projection rules, evidence summary fields, access-decision refs, and explain-view variants.
  - Validation: Redaction tests prove secret-bearing evidence, private workload data, payout-sensitive data, tax/identity material, exact fraud heuristics, and unrelated third-party evidence do not leak to unauthorized views.

- **4.4 Add Overvault-compatible secret and private-evidence refs.**
  - Design: Store secret-bearing or private evidence as refs to owning services and later Overvault-compatible records, while Overclaim keeps hashes, summaries, visibility class, and access-decision refs.
  - Output: Secret-ref contract, private-evidence metadata, redacted summary writer, retention class, and blocked-access reason codes.
  - Validation: Tests prove Overclaim never stores raw tenant secrets, private workload payloads, sensitive payout/tax/identity material, or secret-bearing endpoint details.

- **4.5 Build audit export and replay bundle primitives.**
  - Design: Generate claim replay bundles and audit exports with declared scope, redaction profile, included refs, integrity hash, generated-by, and generated-at.
  - Output: `POST /claims/{claim_id}/export`, `claim_replay_bundle`, export refs, BLAKE3 report hash, and `overclaim.export_created` events.
  - Validation: Export tests prove authorized exports include provenance and integrity refs while redacting private data and preserving enough structure for accounting, compliance, stewardship, and provider payout review.

## Phase 5: Challenge Windows, Holds, And Finality Coordination

### Work Items

- **5.1 Implement challenge and response windows.**
  - Design: Open policy-versioned evidence, challenge, response, appeal, native-app, and public-provider windows with class-specific deadlines and extension rules.
  - Output: `POST /claims/{claim_id}/challenge-window`, window records, deadline timers, extension refs, expiration outcomes, and `overclaim.challenge_window_opened` events.
  - Validation: Tests prove Phase 4 private workload claims default to the SDS-defined 48-hour evidence/challenge and 7-day appeal windows unless policy overrides, and clients cannot hard-code or bypass policy-versioned windows.

- **5.2 Implement pre-finality hold requests.**
  - Design: Request holds for open Overmeter dispute windows, ORU reservations or held funds, Seal Ledger settlement candidates without finality markers, and Provider Payout items still in challengeable states.
  - Output: `POST /claims/{claim_id}/holds`, hold request records, target refs, partial-hold rules, expiry, downstream response refs, and `overclaim.hold_requested` events.
  - Validation: Hold tests prove disputed jobs can block settlement progression while Overclaim does not create ledger entries, mutate balances, or release payouts directly.

- **5.3 Implement downstream hold acknowledgements and denials.**
  - Design: Record hold acknowledgements, denials, retryable errors, expiry, partial holds, and unavailable-owner states from Seal Ledger, Overbill, Provider Payout Service, ORU Account Service, and settlement targets.
  - Output: Hold response records, `overclaim.hold_acknowledged` events, retry metadata, and operator deadline alerts.
  - Validation: Integration tests prove unavailable downstream services keep claims in `hold_requested` with retry and deadline alerts rather than silently dropping or fabricating hold state.

- **5.4 Enforce settlement-finality boundaries.**
  - Design: Check finality markers, external payout submission, paid status, final Overbill instructions, prior final decisions, and stewardship review requirements before accepting hold, refund, correction, or reversal proposals.
  - Output: Finality checker, blocked-action records, manual-review refs, and reason-coded explanations.
  - Validation: Tests prove automatic holds are denied after finality and Overclaim records append-only correction or reversal proposals for owning services instead of mutating final records.

- **5.5 Implement expiration and withdrawal behavior.**
  - Design: Expire claims, windows, holds, and evidence requests according to policy while allowing withdrawal before finality when policy allows.
  - Output: Expiration worker, `POST /claims/{claim_id}/withdraw`, withdrawal records, release proposal refs, and terminal-state events.
  - Validation: Tests prove expired or withdrawn claims release only through owner-service refs, preserve audit records, and do not erase evidence, holds, or prior decisions.

## Phase 6: Resolution, Appeals, And Remedy Proposals

### Work Items

- **6.1 Implement resolution proposal APIs.**
  - Design: Propose refund, correction, release, denial, trust action, manual review, payout hold, namespace action, native-app adjustment, or no-action remedies with evidence refs and policy versions.
  - Output: `POST /claims/{claim_id}/resolution-proposals`, proposal records, required-approver rules, downstream target refs, and `overclaim.resolution_proposed` events.
  - Validation: API tests reject proposals without evidence refs, policy refs, affected refs, allowed remedy class, signer identity, or downstream target ownership.

- **6.2 Implement final resolution records.**
  - Design: Record signed final decisions with remedy, evidence refs, reason codes, resolver identity or service, downstream action refs, appeal window, and finality marker plan.
  - Output: `POST /claims/{claim_id}/resolve`, resolution records, signed decision payload, downstream handoff refs, and `overclaim.resolved` events.
  - Validation: Resolution tests prove final decisions are replayable from claim, policy, evidence, statements, hold refs, downstream responses, and reason codes.

- **6.3 Implement appeal workflow.**
  - Design: Allow appeals from resolution records with appeal reason, new evidence refs, deadline, required resolver class, freeze behavior, and final appeal decision refs.
  - Output: `POST /claims/{claim_id}/appeals`, appeal records, appeal-window timers, challenged-resolution status, and `overclaim.appealed` events.
  - Validation: Appeal tests prove appeals cannot rewrite source resolutions and can freeze consequence escalation only where policy requires.

- **6.4 Create refund, correction, and release proposal refs.**
  - Design: Emit explicit refs for Overbill refunds, Seal Ledger correction entries, ORU holds/releases, Provider Payout holds/releases, Overmeter correction candidates, and downstream settlement actions.
  - Output: Remedy handoff records, correlation ids, target service refs, retry metadata, and downstream response tracking.
  - Validation: Integration tests prove corrections append new ledger/accounting records through owning services instead of editing original usage, invoice, payout, or ledger records.

- **6.5 Implement finality markers and explanation records.**
  - Design: Create finality markers after allowed windows close and generate evidence-backed explanations that disclose stable reason codes without exposing private evidence or fraud heuristics.
  - Output: `overclaim.finalized` events, finality marker records, `GET /claims/{claim_id}/explain`, and role-aware explanation profiles.
  - Validation: Finality tests prove closed claims cannot be reopened without a new claim or policy-approved appeal/correction path, and explanations remain useful but redacted.

## Phase 7: Cross-Service Handoffs And Product Clients

### Work Items

- **7.1 Integrate Overmeter, Seal Ledger, ORU, and Overbill handoffs.**
  - Design: Send signed hold, release, refund, correction, and dispute-window refs to accounting owners after Phase 5 rails exist.
  - Output: Accounting handoff clients, idempotent retry behavior, returned immutable refs, and claim status fields for downstream action state.
  - Validation: Tests prove Overclaim cannot mutate balances, entries, invoices, receipts, payouts, usage rollups, prices, or external payment rails directly.

- **7.2 Integrate Provider Payout Service handoffs.**
  - Design: Coordinate payout holds, releases, denial refs, appeal windows, fraud review refs, and post-finality correction proposals for provider earnings.
  - Output: Payout handoff client, payout-item state mapping, public-provider phase gates, and provider-visible redacted explanations.
  - Validation: Tests prove payout holds apply only before allowed finality and public-provider fraud or Sybil clusters require manual stewardship review where SDS #32 requires it.

- **7.3 Integrate Oververify and Challenge Task Service evidence.**
  - Design: Consume challenge outcomes, trust evidence, provider eligibility refs, challenge failure refs, and dispute refs while sending claim outcomes back to Oververify as evidence.
  - Output: Trust/evidence handoff records, challenge-result dispute support, Oververify outcome refs, and replay bundle links.
  - Validation: Tests prove Overclaim records disputes and outcomes but does not directly set trust scores, provider eligibility, scheduler eligibility, or workload-class eligibility.

- **7.4 Integrate namespace and route claim handoffs.**
  - Design: Support namespace ownership, route binding, delegation, transfer, tombstone, verification marker, and privacy-aware resolution claims after the Universal Namespace Service provides stable affected refs.
  - Output: Namespace claim class, owner-service handoff refs, privacy-aware explanation records, and route/identity correlation ids.
  - Validation: Tests prove namespace claims remain gated until Phase 8 owner refs exist and Overclaim cannot mutate names, routes, ownership, or tombstones directly.

- **7.5 Integrate SDK, CLI, admin UI, and native-app clients.**
  - Design: Expose claim create/read/evidence/appeal/explain/export flows through generated SDKs, Rust CLI, admin/developer UI, wallet/usage center, native app dispute surfaces, and service adapters.
  - Output: Client contract map, generated bindings, command examples, role-aware UI projection notes, and app-service claim gates.
  - Validation: Client tests prove TypeScript remains limited to generated bindings and UI/native-app surfaces, while claim runtime, policy, replay, and remedy coordination remain Rust-first.

## Phase 8: Persistence, Replay, Operations, And Recovery Hardening

### Work Items

- **8.1 Implement Overbase-backed claim persistence when Phase 8 exists.**
  - Design: Move from local Phase 4 record storage to Overbase collections/indexes for claims, parties, windows, holds, proposals, resolutions, appeals, finality markers, and replay metadata.
  - Output: Overbase schema, collection/index definitions, migration path, consistency policy, and query projections.
  - Validation: Persistence tests prove claim reads, state transitions, duplicate detection, deadlines, and replay queries survive restart and migration without record rewriting.

- **8.2 Implement Overstore and Overvault evidence-ref expansion.**
  - Design: Store large artifacts, audit exports, replay reports, redacted bundles, and private/secret refs through native Overstore and Overvault boundaries where appropriate.
  - Output: Artifact refs, upload/download grants, encrypted private refs, retention metadata, and redaction profile links.
  - Validation: Storage tests prove raw private evidence stays in owner services or Overvault-compatible refs and public exports use redacted Overstore artifacts only.

- **8.3 Add operational dashboards, metrics, and alerts.**
  - Design: Track open claims by type, severity, affected service, hold status, deadline, resolution age, appeal rate, downstream action state, duplicate/abuse clusters, and export generation.
  - Output: Metrics contract, dashboard queries, health/readiness views, Overwatch event families, and alert triggers.
  - Validation: Observability tests prove dashboards derive from stored refs and do not expose raw private payloads, secrets, payout-sensitive data, or fraud heuristics in broad views.

- **8.4 Implement recovery, reconciliation, and derived-index rebuild.**
  - Design: Resume claims from append-only state, reconcile downstream hold/action refs, retry failed handoffs, rebuild derived indexes, quarantine malformed evidence, and detect timer drift.
  - Output: Recovery command, reconciliation report, retry queue, quarantine records, index rebuild command, and recovery events.
  - Validation: Recovery tests prove service outage, partial downstream failure, malformed evidence, expired timers, and derived-index loss recover without duplicate holds, duplicate refunds, or rewritten outcomes.

- **8.5 Add backup, restore, retention, and migration hooks.**
  - Design: Preserve records needed for accounting, compliance, stewardship, provider payout review, appeals, fraud controls, and audit exports while pruning raw detail through owner-service policies.
  - Output: Retention policy refs, backup/restore manifests, migration checklist, redaction-preserving export path, and purge-denial rules for active claims.
  - Validation: Retention tests prove final dispute records, reason codes, signatures, hashes, downstream refs, and replay bundles remain durable while raw usage detail expires unless pinned by active claim, hold, audit, appeal, or compliance review.

## Phase 9: Public-Provider, Abuse, Native-App, And Governance Expansion

### Work Items

- **9.1 Add Phase 11 public-provider dispute profiles.**
  - Design: Support public-provider challenge failure, duplicate execution, payout hold, sandbox escape, secret-access signal, fabricated evidence, fraud/Sybil cluster, appeal, and final-resolution claim profiles.
  - Output: Public-provider claim classes, stricter payout protection windows, fraud/reputation handoff refs, sandbox evidence refs, and provider-visible explanations.
  - Validation: Tests prove public-provider claims stay gated until Phase 11 and public nodes still cannot receive private, regulated, secret-bearing, or system-service workload claims.

- **9.2 Add abuse controls without blanket dispute-right loss.**
  - Design: Add duplicate-link records, cooldowns, higher evidence requirements, rate limits, canonical claim selection, reputation/eligibility signals, and manual review paths for severe or plausibly valid claims.
  - Output: Abuse marker records, suppression rules, reason-coded denial/explain responses, fraud/reputation handoff refs, and review queue refs.
  - Validation: Abuse tests prove bad-faith volume is suppressed while safety, payout, private-data, account, and finality-affecting claims retain an authenticated intake path.

- **9.3 Add Phase 12 native-app dispute expansion.**
  - Design: Support wallet/usage, search, directory, messaging, social, maps, workspace, personal AI, and app-service claims only after native services emit stable evidence and receipt refs.
  - Output: Native-app claim class map, receipt/usage-summary dispute windows, app-service owner refs, client projection rules, and user-facing explanation profiles.
  - Validation: Tests prove native-app disputes default to policy-versioned windows, cannot bypass app owner refs, and do not expose other users, private app state, or service internals.

- **9.4 Add incident, compliance, and stewardship review flows.**
  - Design: Route cross-tenant/systemic incidents, regulated evidence, secret-bearing evidence, fraud clusters, post-finality reversals, and public-provider disputes through signed stewardship or compliance review.
  - Output: Incident handoff refs, compliance boundary refs, stewardship decision records, threat-model evidence, and audit export profiles.
  - Validation: Governance tests prove manual review is required where SDS #32 requires it and every override records signer, policy version, evidence refs, reason codes, and downstream action refs.

- **9.5 Add aggregate reporting and public-safe exports.**
  - Design: Produce redacted aggregates for claim volumes, resolution age, appeal rate, payout holds, refund/correction classes, public-provider dispute classes, and incident-linked clusters.
  - Output: Reporting schema, public-safe aggregate API, stewardship report refs, compliance export refs, and redaction checks.
  - Validation: Reporting tests prove public or broad internal exports exclude raw private payloads, secrets, exact fraud heuristics, payout/tax/identity material, provider-private correlation data, and unrelated third-party evidence.

## Phase 10: Validation, Documentation Alignment, And Build Handoff

### Work Items

- **10.1 Run contract and API validation.**
  - Design: Validate claim create, dry-run, read, list, evidence attach, hold request, challenge window, resolution proposal, resolve, appeal, withdraw, explain, export, replay, and owner-service handoff APIs against schemas.
  - Output: Contract tests, API tests, invalid-fixture tests, compatibility reports, and generated binding checks.
  - Validation: Tests pass for required fields, signed envelopes, idempotency, trace ids, tenant scope, policy refs, evidence refs, stable reason codes, redaction profiles, and schema-version compatibility.

- **10.2 Run privacy, security, and finality negative controls.**
  - Design: Prove unauthorized actors cannot view claims or evidence, raw secrets cannot enter Overclaim storage, public-provider details stay redacted, and finality boundaries block automatic mutation.
  - Output: Security test suite, redaction test suite, finality test suite, abuse test suite, and no-private-data scan.
  - Validation: Negative tests fail closed and explanations remain useful without exposing sensitive internals.

- **10.3 Run cross-service integration validation.**
  - Design: Exercise Overclaim with Overgate, Overguard, Overwatch, Overmeter, Oververify, Challenge Task Service, Seal Ledger, Overbill, ORU Account Service, Provider Payout Service, Universal Namespace Service, SDK, CLI, admin UI, and native-app stubs.
  - Output: Integration scenario set, deterministic refs, replay bundles, downstream action records, and reconciliation reports.
  - Validation: Successful, rejected, held, resolved, appealed, expired, duplicate, abusive, public-provider-gated, native-app-gated, and post-finality scenarios produce distinct states and replayable evidence.

- **10.4 Validate documentation and phase alignment.**
  - Design: Check this plan against SDS #32, the service catalog entry, master Phase 0 through Phase 13 order, service_catalog_alignment, Phase 4, Phase 5, Phase 8, Phase 11, Phase 12, Phase 13, and `docs/overrid_tech_stack_choice.md`.
  - Output: Link-check results, phase-table verification, work-item count verification, stale-note scan, stack-guardrail scan, Docdex search evidence, and queue/progress updates.
  - Validation: Validation proves the plan has 10 phases numbered 1 through 10, five work items per phase, Design/Output/Validation fields, no external product-boundary drift, and no required master phase reordering.

- **10.5 Hand off implementation gates.**
  - Design: Convert this documentation into build-entry criteria for contracts, schemas, fixtures, Rust service skeleton, claim intake, evidence binding, holds/finality, remedy proposals, handoffs, persistence, public-provider gating, native-app gating, and governance readiness.
  - Output: Implementation gate checklist, dependency readiness checklist, risk register, and first coding-task candidate list.
  - Validation: Handoff review confirms Phase 4 work can start after Phase 0 through Phase 3 prerequisites exist, Phase 5 accounting actions remain owner-service effects, and Phase 11/12/13 expansions remain gated until their owning rails are ready.
