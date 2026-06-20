# SUB BUILD PLAN #39 - Overasset

Attached SDS: [docs/sds/accounting/overasset.md](../sds/accounting/overasset.md)

## Purpose

This sub-build plan turns SDS #39 into an implementation sequence for Overasset. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overasset is a Phase 5 accounting and rights service that records evidence-backed operational rights, ownership evidence refs, grant-right refs, capacity-claim rights, service/app ownership refs, delegation, revocation, dispute, correction, and replay records. It expands in Phase 8 for storage entitlement refs and namespace-bound rights. It must not become an NFT system, speculative market, broad legal-title registry, storage access controller, namespace authority, ORU balance mutator, or Seal Ledger mutation path.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #39: Overasset](../sds/accounting/overasset.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overasset plan](../service_catalog/accounting/overasset.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical schemas, signed envelopes, idempotency, trace ids, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass owner refs, Overtenant scope, Overkey signing/delegation refs, Overgate request discipline, Overregistry refs, Overwatch audit, and Overqueue-safe command context. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Overclaim dispute/finality refs, Oververify evidence, Policy Dry-Run checks, and challenge/trust evidence. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Controls the first build point: evidence-backed non-speculative utility/right records, capacity-claim rights, grant-right refs, service/app ownership refs, Seal Ledger/ORU refs, transfer blocking, revocation, dispute/correction links, and replay bundles. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Controls storage entitlement refs, namespace-right bindings, route/asset refs, dataset/model/media/package rights metadata, and Overstore/Overbase/Overvault/Universal Namespace expansion. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies trusted-federation and public-interest grant/resource context that can create purpose-scoped right refs without broad transferability. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider capacity, fraud, payout-hold, low-sensitivity, and reputation constraints that may affect right visibility and transfer blocking. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies Wallet and Usage Center, admin UI, SDK, CLI, native apps, and central AI consumers of authorized right summaries. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance boundaries, incident handoffs, threat-model reviews, stewardship reports, audit exports, retention controls, migration governance, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #39 first build work aligned to master Phase 5, with Phase 8 storage/namespace expansion and later public, native-app, and governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, or customer-count drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 8, 11, 12, and 13 | Attach SDS #39, freeze Overasset authority, preserve Phase 5 as first build point, and record Phase 8 expansion gates. |
| 2 | Master Phases 0, 1, 4, and 5 | Build Rust contracts, schemas, right types, lifecycle states, reason codes, fixtures, and replay commitments. |
| 3 | Master Phases 1, 4, and 5 | Implement evidence intake, source validation, policy checks, right creation, read, and explanation behavior. |
| 4 | Master Phases 4 and 5 | Implement append-only lifecycle transitions for delegation, expiry, revocation, correction, dispute attachment, and replay. |
| 5 | Master Phases 3, 4, and 5 | Build Phase 5 accounting/operations right classes for grant rights, capacity claims, service/app ownership refs, and resource-right records. |
| 6 | Master Phases 4, 5, 10, 11, and 13 | Add narrow transfer request/finality paths, transfer blocking, compliance restrictions, public-provider constraints, and anti-speculation guardrails. |
| 7 | Master Phase 8 | Expand into storage entitlement refs, namespace-right bindings, route/asset bindings, and Overstore/Overbase/Overvault/Universal Namespace integrations. |
| 8 | Master Phases 5, 6, 8, 12, and 13 | Expose wallet/admin/SDK/native-app/stewardship read models, explanations, redaction profiles, and client handoffs without moving authority into clients. |
| 9 | Master Phases 7, 8, and 13 | Harden operations, replay, backfill, native persistence handoffs, grid-resident runtime, governance, retention, and migration controls. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Overasset core is a Rust service/module using shared contract types, Tokio for bounded async workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Right records, evidence refs, storage entitlements, namespace bindings, capacity claims, grant rights, service ownership refs, delegations, transfers, revocations, disputes, replay bundles, API objects, events, fixtures, and reason-code catalogs use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating endpoints require signed actor or service envelopes, tenant context, owner refs, trace id, idempotency key, schema version, policy refs, source evidence refs, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for source evidence fingerprints, replay bundles, export bundles, schema fixtures, and deterministic comparison tests.
- Overasset may later persist records through Overbase, replay/export artifacts through Overstore, and private/compliance refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, market trading, or external rights marketplaces the platform boundary.
- Overasset supplies scoped right facts to owning services and policy. It does not enforce storage access directly, mutate ORU balances, append Seal Ledger entries, own namespace truth, create billing/payment refs, run payout batches, or define legal title beyond cited evidence.
- `resource_right_record` is the canonical root right record shape from SDS #39. Transferability defaults to false. Transfer, delegation, and reassignment are allowed only when evidence, policy, dispute, tenant, compliance, and jurisdiction flags allow them.
- Planning and implementation must avoid speculative asset behavior, royalties, artificial scarcity, market price discovery, broad legal-title claims, hidden pricing tables, revenue projections, customer-count assumptions, and direct external payment calls.

## Phase 1: SDS Attachment, Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #39.**
  - Design: Link this document from the Overasset SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/accounting/overasset.md`, `docs/service_catalog/accounting/overasset.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #39 returns both the Overasset SDS and this sub-build plan.

- **1.2 Freeze Overasset authority boundaries.**
  - Design: Record that Overasset owns operational right schemas, ownership evidence refs, scoped entitlement refs, lifecycle transitions, right explanations, audit/replay bundles, and policy-consumable right refs.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms it does not own storage access enforcement, Universal Namespace ownership truth, Overpass identity, ORU balance mutation, Seal Ledger entries, Overbill billing records, Overgrant allocation policy, or Overclaim finality.

- **1.3 Preserve master Phase 5 as the first build point.**
  - Design: Keep first implementation in Phase 5 for minimum right records needed by accounting and operations: grant-right refs, capacity-claim rights, and service/app ownership refs backed by signed evidence.
  - Output: Phase-gate note that Phase 0, Phase 1, and Phase 4 are prerequisites, Phase 5 is first build, and Phase 8 adds storage/namespace bindings.
  - Validation: Review proves this plan does not defer all Overasset work to Phase 8 and does not build storage/namespace bindings before their owner services exist.

- **1.4 Carry forward resolved SDS #39 decisions.**
  - Design: Preserve evidence-first creation, non-transferable-by-default behavior, narrow operational transfer classes, expiry as projection state rather than deletion, wallet/admin visibility separation, and least-detail private storage/vault redaction.
  - Output: Resolved-decision checklist tied to implementation reviews.
  - Validation: Review rejects tradable market objects, NFT-like ownership, broad resale, royalties, transfer during active disputes, deletion of history on expiry, and raw private storage/vault evidence in user views.

- **1.5 Define owner-service and consumer boundaries.**
  - Design: Create a dependency matrix for Seal Ledger, ORU Account Service, Overgrant, Overclaim, Universal Namespace Service, Overstore, Overbase, Overvault, Overregistry, Overguard, Overwatch, Wallet and Usage Center, admin UI, SDK, CLI, native apps, and central AI stewardship.
  - Output: Boundary matrix listing consumed refs, emitted right refs, final authority owner, redaction class, replay evidence, expiry behavior, transferability class, and later phase gate.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, trace ids, policy refs, and Overwatch audit rather than direct privileged state reads.

## Phase 2: Rust Contracts, Schemas, Right Types, And Fixtures

### Work Items

- **2.1 Create the Overasset Rust contract module.**
  - Design: Add contract types for resource rights, ownership evidence refs, storage entitlement refs, namespace bindings, capacity claims, grant rights, service ownership refs, delegation records, transfer records, revocation records, dispute links, replay bundles, API errors, and events.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, right-type enums, lifecycle enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Seal Ledger, Overbill, Overgrant, Overclaim, Universal Namespace Service, Overstore, Overbase, and Overvault internals.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for create, read, explain, delegate, transfer, revoke, dispute attach, replay, owner query, namespace query, events, and export bundles.
  - Output: Schema files, valid examples, invalid examples, generated validators, compatibility notes, and docs-facing examples.
  - Validation: Schema tests reject missing tenant scope, owner refs, actor/service identity, trace id, idempotency key, right type, source evidence refs, policy refs, state, allowed use, visibility class, or audit refs where required.

- **2.3 Define right-type and evidence catalogs.**
  - Design: Encode resource-right records, capacity-claim rights, grant-right refs, service/app ownership refs, storage entitlement refs, namespace-right bindings, dataset/model/media/package metadata refs, delegation records, transfer records, revocation records, and dispute links.
  - Output: Right-type catalog, source evidence catalog, visibility classes, allowed-use classes, transferability classes, and compatibility notes.
  - Validation: Tests prove every right type names its source service, source record ref, integrity hash or signed ref, allowed use, owner scope, policy refs, and redaction profile.

- **2.4 Model lifecycle and transition state machines.**
  - Design: Encode lifecycle states `draft`, `pending_policy`, `active`, `delegated`, `transfer_requested`, `transfer_blocked`, `transferred`, `disputed`, `restricted`, `revoked`, `expired`, `corrected`, and `tombstoned`.
  - Output: Transition tables, Rust enums, JSON Schema enums, event emission rules, invalid transition fixtures, and lifecycle review notes.
  - Validation: State tests reject direct deletion, broadening access from an expired right, transfer from disputed or restricted states, and correction without replacement/evidence refs.

- **2.5 Create deterministic Overasset fixtures.**
  - Design: Build fixtures for right creation, evidence validation, policy denial, delegation, revocation, expiry, correction, dispute attach, transfer block, allowed operational reassignment, replay, owner query, namespace query, and redaction.
  - Output: Fixture directory, expected state projections, reason codes, Overwatch events, explanation examples, export examples, and invalid examples.
  - Validation: Fixture tests produce stable projections, state transitions, redaction behavior, replay hashes, and transfer-blocking outcomes across repeated runs.

## Phase 3: Evidence Intake, Policy Checks, Creation, Reads, And Explanations

### Work Items

- **3.1 Implement source evidence intake.**
  - Design: Accept source refs from Seal Ledger, ORU Account Service, Overgrant, Overregistry, Universal Namespace Service, Overstore, Overbase, Overvault, Overclaim, and signed service/operator refs.
  - Output: Evidence resolver interfaces, source-service allowlist, integrity/hash validation, source freshness fields, and unavailable-source behavior.
  - Validation: Tests reject missing source service, stale or unknown source refs, wrong tenant scope, mismatched owner refs, unsupported evidence kind, and unsigned or unhashable evidence.

- **3.2 Implement right creation.**
  - Design: Create evidence-backed rights only after owner refs, tenant scope, allowed use, validity window, transferability default, compliance flags, and Overguard policy refs are valid.
  - Output: `POST /assets/rights` handler, idempotency behavior, initial lifecycle projection, reason-code mapping, and `overasset.right_created` events.
  - Validation: API tests reject creation without source evidence, actor/service identity, tenant context, idempotency key, trace id, policy refs, owner refs, allowed use, and audit refs.

- **3.3 Implement authorized read and owner query APIs.**
  - Design: Provide authorized right summaries by right id and owner id while applying tenant, owner, delegate, service-account, operator, and compliance visibility profiles.
  - Output: `GET /assets/rights/{right_id}`, `GET /assets/by-owner/{owner_id}`, pagination, redaction profiles, stable errors, and access audit events.
  - Validation: Security tests reject cross-tenant reads, unauthorized provider/private evidence reads, raw storage/vault refs, and other-owner evidence leakage.

- **3.4 Implement explanation API.**
  - Design: Explain current right state from source evidence, lifecycle transitions, policy refs, transfer/delegation/dispute state, redacted history, and stable reason codes.
  - Output: `GET /assets/rights/{right_id}/explain`, explanation profiles for user, provider, service, operator, compliance, and wallet/admin consumers.
  - Validation: Explanation tests prove user-safe views expose enough reason-code detail for remediation while hiding provider-private, fraud, compliance, secret, and raw ledger internals.

- **3.5 Emit audit and observability facts.**
  - Design: Emit right creation, read, explanation, blocked creation, validation failure, and source-unavailable facts with actor/service identity, tenant context, trace id, policy refs, evidence refs, and redaction class.
  - Output: Event contracts, metrics counters, Overwatch handoff refs, dashboard fields, and audit export fields.
  - Validation: Observability tests prove every mutating or privileged read path emits append-only audit evidence and no telemetry creates billing, payout, transfer, or storage-access authority.

## Phase 4: Lifecycle Transitions, Delegation, Revocation, Expiry, Correction, And Replay

### Work Items

- **4.1 Implement scoped delegation.**
  - Design: Allow delegation only within the original right scope, allowed operations, purpose constraints, expiry, revocation conditions, Overguard policy refs, and Overwatch audit.
  - Output: `POST /assets/rights/{right_id}/delegations`, delegation records, revocation hooks, subdelegation rules, and `overasset.delegation_created`/`overasset.delegation_revoked` events.
  - Validation: Tests prove delegation cannot broaden owner scope, data access, tenant scope, purpose scope, storage operations, namespace authority, or transferability.

- **4.2 Implement expiry and revocation transitions.**
  - Design: Record expiry and revocation as append-only transitions that change current projection without deleting downstream namespace, storage, ledger, or audit history.
  - Output: `POST /assets/rights/{right_id}/revocations`, expiry worker hooks, revocation records, downstream effect refs, and audit events.
  - Validation: Tests prove expired/revoked rights cannot create new access, delegation, transfer, route broadening, grant authorization, or storage expansion unless a new evidence-backed transition allows it.

- **4.3 Implement correction transitions.**
  - Design: Treat corrections as append-only replacements or amendments with source evidence refs, old projection, new projection, reason codes, and replay evidence.
  - Output: Correction record contract, correction handler, replacement refs, explanation changes, and `overasset.correction_applied` events.
  - Validation: Tests prove corrections never rewrite old right records and replay can show original state, correction evidence, resulting projection, and downstream effect refs.

- **4.4 Implement Overclaim dispute attachment.**
  - Design: Attach dispute refs, hold effects, disputed scope, correction refs, finality refs, visibility class, and transfer/blocking behavior from Overclaim.
  - Output: `POST /assets/rights/{right_id}/disputes`, dispute link record, hold/block mapping, explanation updates, and `overasset.dispute_attached` events.
  - Validation: Tests prove active disputes block transfer or broadening access unless claim policy explicitly allows a narrow action.

- **4.5 Implement replay bundles.**
  - Design: Reconstruct right state from right refs, source evidence refs, policy versions, lifecycle transition refs, downstream service refs, and audit events.
  - Output: `POST /assets/rights/{right_id}/replay`, replay bundle writer, hash comparison, mismatch reason codes, and export fields.
  - Validation: Replay tests reconstruct current state from stored refs, detect missing evidence, preserve all old states, and flag mismatches without silently widening rights.

## Phase 5: Phase 5 Accounting And Operational Right Classes

### Work Items

- **5.1 Build grant-right refs.**
  - Design: Create grant-backed rights from Overgrant source account refs, beneficiary refs, purpose scope, resource dimensions, quotas, reporting requirements, abuse throttle refs, expiry, and revocation refs.
  - Output: Grant-right schema, creation adapter, read/explain fields, reporting handoff fields, and `overasset.right_created` fixtures.
  - Validation: Tests prove grant-funded rights preserve purpose scope, do not become broad spend authority, and expire/revoke only through evidence-backed refs.

- **5.2 Build capacity-claim rights.**
  - Design: Create capacity claims from provider/node/resource refs, capacity class, ORU dimensions, reservation refs, Overmark refs, challenge/trust refs, validity window, and dispute refs.
  - Output: Capacity-claim schema, source validation, policy input facts, owner/provider redaction profiles, and dashboard fields.
  - Validation: Tests prove capacity claims cite ORU/Seal Ledger/Overmark/trust evidence, cannot bypass scheduler/lease/policy authority, and do not create payout authority.

- **5.3 Build service and app ownership refs.**
  - Design: Create ownership refs for apps, native services, system services, packages, datasets, models, and media from Overregistry refs plus signed owner authority.
  - Output: Service ownership schema, registry adapter, owner/delegate read model, transfer-blocking defaults, and audit events.
  - Validation: Tests prove service/app ownership refs do not replace Overregistry records, Overpass identity, package validation, namespace authority, or deployment authority.

- **5.4 Build accounting evidence links.**
  - Design: Link rights to Seal Ledger refs, ORU dimension refs, Overbill receipt/invoice refs, Overgrant refs, and Overclaim correction/dispute refs without mutating accounting services.
  - Output: Accounting evidence schema, source-ref index, statement/read fields, and replay inputs.
  - Validation: Tests prove Overasset never appends Seal Ledger entries, mutates ORU balances, creates invoices, changes payout state, or treats receipt refs as ownership without policy/evidence.

- **5.5 Build Phase 5 right dashboards.**
  - Design: Track active rights by type, disputed rights, transfer requests, blocked transfers, expired rights, revoked rights, grant-backed rights, capacity claims, and correction rates.
  - Output: Metrics schema, dashboard definitions, alert rules, Overwatch aggregation, and runbook links.
  - Validation: Operations tests prove alerts fire for rights without source evidence, unauthorized transfer attempts, disputed rights used for access, replay mismatches, and blocked transfer spikes.

## Phase 6: Transfers, Blocking, Compliance, And Anti-Speculation Controls

### Work Items

- **6.1 Implement transfer request workflow.**
  - Design: Support request, approve, deny, and finalize actions only for narrow operational transfers allowed by evidence, policy, owner authority, dispute state, compliance flags, and jurisdiction rules.
  - Output: `POST /assets/rights/{right_id}/transfers`, transfer records, approval/denial/finality refs, and `overasset.transfer_requested`/`overasset.transfer_finalized`/`overasset.transfer_denied` events.
  - Validation: Tests prove transfer defaults to blocked and only allowed classes can finalize.

- **6.2 Enforce blocked transfer classes.**
  - Design: Block public/global names, native-app/system-service roots, provider-capacity claims, grant-funded public-interest rights, regulated or secret-bearing rights, disputed rights, payout/settlement-affecting rights, and cross-tenant/cross-jurisdiction transfers unless later policy explicitly allows them.
  - Output: Blocklist rules, reason-code catalog, policy input facts, operator review fields, and denial fixtures.
  - Validation: Tests prove blocked classes cannot be bypassed through delegation, correction, expiry, stale reads, or downstream service handoffs.

- **6.3 Implement compliance and legal restriction flags.**
  - Design: Attach legal/compliance refs, region/jurisdiction flags, retention flags, appeal refs, review-required state, and Compliance Boundary handoff refs where needed.
  - Output: Compliance metadata contract, restricted-state mapping, export fields, and review queues.
  - Validation: Compliance tests prove rights become restricted or blocked when policy changes and do not erase source, namespace, storage, ledger, or audit history.

- **6.4 Implement anti-speculation guardrails.**
  - Design: Exclude market price, royalty, resale, artificial scarcity, collectible supply, token metadata, external marketplace, and blockchain dependency fields from core contracts.
  - Output: Negative schema tests, review checklist, docs guardrails, and fixture scan rules.
  - Validation: Anti-NFT tests verify no market price, royalty, speculative supply, on-chain, or marketplace field is required for any right.

- **6.5 Integrate public/federation constraints.**
  - Design: Prepare right visibility and transfer blocking for trusted federation, public-interest pools, public-provider capacity, fraud controls, reputation refs, payout holds, and stewardship reports.
  - Output: Federation/public constraint matrix, public-provider redaction profile, public-interest grant-right refs, and reporting handoff fields.
  - Validation: Tests prove public-provider and public-interest rights remain bounded, evidence-backed, redacted, and reviewable without broad market transfer.

## Phase 7: Phase 8 Storage And Namespace Entitlement Expansion

### Work Items

- **7.1 Build storage entitlement refs.**
  - Design: Create storage entitlement refs for Overstore, Overbase, and Overvault with service ref, object/collection/vault ref, allowed operations, quota/dimensions, retention policy, expiry, revocation refs, and redaction profile.
  - Output: Storage entitlement schema, source adapters, read/explain fields, and downstream effect refs.
  - Validation: Tests prove storage/vault rights do not expose raw object paths, decrypted content, secret names/values, raw key refs, sensitive object ids, or provider/node topology to unauthorized readers.

- **7.2 Build namespace-right bindings.**
  - Design: Bind namespace id, name/route/asset ref, owner refs, delegation refs, transfer refs, tombstone refs, dispute refs, and Universal Namespace Service refs.
  - Output: Namespace binding schema, `GET /assets/by-namespace/{namespace_id}`, transfer/delegation constraints, and explanation fields.
  - Validation: Tests prove namespace rights respect tombstones, verification markers, anti-squatting controls, reserved names, route history, and namespace-owner authority.

- **7.3 Build route and asset binding refs.**
  - Design: Connect app routes, service endpoints, asset refs, package refs, dataset/model/media refs, and Overmesh route-resolution refs without owning the underlying route or object records.
  - Output: Binding contracts, source-ref index, route/asset explanation fields, and policy input facts.
  - Validation: Tests prove Overasset refs cannot hijack routes, bypass Overmesh resolution policy, or replace package/object/namespace validation.

- **7.4 Integrate storage/namespace dispute behavior.**
  - Design: Apply Overclaim disputes, namespace tombstones, storage retention/legal holds, purge eligibility, and downstream effect refs to current right projections.
  - Output: Dispute/hold mapping, retention-state mapping, redacted explanations, and audit timeline.
  - Validation: Tests prove storage owner services govern retention, legal holds, backups, and purge eligibility while Overasset records entitlement state and replay evidence.

- **7.5 Build Phase 8 replay across owner services.**
  - Design: Reconstruct storage and namespace right state from Overasset records plus Universal Namespace Service, Overstore, Overbase, Overvault, Overclaim, Overguard, and Overwatch refs.
  - Output: Cross-service replay bundle, mismatch detector, source freshness map, and export profile.
  - Validation: Integration tests prove replay detects missing downstream refs, stale namespace/storage state, unauthorized entitlement broadening, and redaction mistakes.

## Phase 8: Client Views, Explanations, Wallet, Admin, SDK, And Stewardship

### Work Items

- **8.1 Build wallet-ready right summaries.**
  - Design: Provide user/org-actionable summaries for rights the viewer owns, controls, receives by delegation, can dispute, or can renew.
  - Output: Wallet summary contract for active and expiring storage entitlements, grant-backed rights, namespace handles/routes, app/service ownership refs, delegations, revocations, disputes, receipts, and safe evidence summaries.
  - Validation: Wallet tests prove summaries stay redacted, cite stable reason codes, and cannot be used as transfer or access authority without live policy/source validation.

- **8.2 Build admin/operator timelines.**
  - Design: Provide broader redacted timelines for operators showing blocked transfer reasons, correction refs, compliance markers, provider-capacity context, replay bundles, and source-service health.
  - Output: Admin/operator read models, filters, pagination, export fields, and access audit events.
  - Validation: Security tests prove operator views require authorization, log access through Overwatch, and do not expose secret-bearing or provider-private details outside authorized scopes.

- **8.3 Build SDK, CLI, and API client contracts.**
  - Design: Generate client contracts for create, read, explain, delegate, transfer request, revoke, dispute attach, replay, owner query, and namespace query flows.
  - Output: SDK/CLI examples, stable JSON output, error catalog, idempotency examples, and docs snippets.
  - Validation: Client tests prove generated bindings preserve signed envelopes, trace ids, idempotency keys, schema versions, reason codes, and redaction profiles.

- **8.4 Build native app and central AI stewardship views.**
  - Design: Provide native apps and central AI stewardship with authorized summaries for grant rights, ownership refs, purpose-scoped rights, disputes, blocked transfers, and replay evidence.
  - Output: Native-app read contracts, stewardship report fields, central AI summary schema, and audit hooks.
  - Validation: Tests prove consumers cannot infer private refs, mutate rights, override policy, broaden access, or turn right summaries into pricing, payout, grant, or ledger authority.

- **8.5 Build evidence export profiles.**
  - Design: Create user-safe, owner-service, operator, compliance, incident, stewardship, and migration export profiles with explicit redaction and retention behavior.
  - Output: Export schemas, BLAKE3 hash fields, source evidence refs, replay refs, and retention metadata.
  - Validation: Export tests prove sensitive refs remain hidden unless authorized and every exported right can be replayed from stored evidence and transition refs.

## Phase 9: Operations, Native Persistence, Grid Residency, Governance, And Migration

### Work Items

- **9.1 Build operations dashboards and runbooks.**
  - Design: Track right counts by type/state, transfer requests, blocked transfers, disputes, revocations, expiries, correction rates, replay mismatches, source-service health, and redaction failures.
  - Output: Metrics schema, dashboard definitions, alert rules, Overwatch event aggregation, and runbook links.
  - Validation: Operations tests prove alerts fire for unauthorized transfer attempts, rights without source evidence, disputed rights used for access, replay mismatches, and redaction policy failures.

- **9.2 Harden replay, recompute, and backfill.**
  - Design: Support scoped recompute by right id, owner, tenant, source service, right type, policy version, state, and evidence checkpoint, plus resumable backfill and mismatch diff reports.
  - Output: Recompute worker, backfill run records, replay comparison model, operator controls, and repair audit refs.
  - Validation: Tests prove recompute is idempotent, bounded, resumable, replayable, preserves old projections during review, and never silently widens rights.

- **9.3 Prepare native Overbase, Overstore, and Overvault persistence handoffs.**
  - Design: Move right/projection records to native Overbase when available, replay/export artifacts to Overstore where appropriate, and private/compliance refs to Overvault without changing API semantics.
  - Output: Persistence adapter interfaces, migration plan, artifact refs, private-ref handoffs, retention hooks, and export format.
  - Validation: Migration tests prove no conventional database/object/vault boundary becomes canonical, records remain replayable, and private/compliance refs stay behind owning service access controls.

- **9.4 Prepare grid-resident protected operation.**
  - Design: Package Overasset as a protected grid-resident system workload with service identity, config contracts, secret refs, health checks, failover behavior, restore drills, maintenance mode, replay pause/resume, and signed operator actions.
  - Output: System-service packaging notes, runtime config contract, backup/restore hooks, health endpoints, maintenance controls, and break-glass audit rules.
  - Validation: Grid tests prove restart, failover, restore, replay pause/resume, and maintenance mode preserve append-only history and do not emit stale broad rights after recovery.

- **9.5 Add governance, compliance, threat-model, and incident handoffs.**
  - Design: Integrate Compliance Boundary policy refs, incident response refs, threat-model findings, stewardship reporting, migration controls, retention/export policy, region-specific restrictions, and audit exports.
  - Output: Governance checklist, compliance export schema, threat-model test list, incident handoff refs, stewardship report fields, and retention policy.
  - Validation: Governance tests prove high-impact right changes, transfer finality, delegation changes, correction/replay changes, and projection repairs require signed action, evidence refs, Overwatch audit, and retention-compliant exports.

## Phase 10: Validation, Documentation Alignment, Queue State, And Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work-item structure, Design/Output/Validation fields, alignment review, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #39`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, work-item sections, final newline, and tab-free formatting.

- **10.2 Validate tech-stack alignment.**
  - Design: Scan for accidental conventional cloud, SaaS-admin, blockchain/NFT, speculative-market, broad legal-title, pricing, revenue, customer-count, mutable-accounting, direct-ledger-mutation, direct-storage-access, and external-payment assumptions.
  - Output: Stack-alignment scan result and any corrected wording.
  - Validation: Scan shows only approved negative-control references, native Overrid service names, or explicit non-choice guardrails from `docs/overrid_tech_stack_choice.md`.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk links.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated docs and link-check evidence.
  - Validation: Local Markdown link validation returns no missing local targets for changed docs.

- **10.4 Validate Overasset authority and phase gates.**
  - Design: Verify every planned behavior preserves Phase 5 as first implementation for evidence-backed utility/right records and Phase 8 as storage/namespace entitlement expansion.
  - Output: Authority-boundary checklist and implementation handoff notes.
  - Validation: Review confirms Overasset does not own storage access enforcement, namespace truth, ORU/Seal Ledger mutation, billing/payment refs, payout batches, grant policy, Overclaim finality, or speculative asset behavior.

- **10.5 Reindex and verify retrieval handoff.**
  - Design: Refresh Docdex for the new plan and linked docs, then verify search returns the SDS #39 plan and source docs in the aligned result set.
  - Output: Docdex index refresh evidence, retrieval evidence, queue/progress notes, repo memory, and implementation handoff note.
  - Validation: Docdex index succeeds, Docdex search for `SUB BUILD PLAN #39 Overasset Phase 5 evidence-backed rights Phase 8 namespace storage entitlement` returns the new sub-build plan in the result set, and `docdexd run-tests` blocker is recorded if no test runner is configured.

## Alignment Review

- SDS #39 already contains resolved open-question decisions for evidence-first implementation, narrow first-deployment transferability, expiry as projection state, wallet/admin visibility, and least-detail private storage/vault redaction. This pass corrects the phase wording so Phase 5 is the first build point and Phase 8 is the storage/namespace expansion gate.
- The service catalog now matches the SDS and master plan: Overasset starts in Phase 5 for evidence-backed operational rights and expands in Phase 8 for storage entitlement and namespace-bound rights.
- The master Phase 0 through Phase 13 order remains valid. This pass adds SDS #39 to the per-SDS index and keeps Overasset in Phase 5 while preserving Phase 8 as the platform-binding expansion.
- The build-plan crosswalk remains valid. This pass adds SDS #39 to the sub-build-plan index with later public-provider, native-app, and governance hardening gates.
- The accepted Rust-first/native-Overrid stack remains authoritative. The plan uses Rust, Tokio, shared contracts, canonical JSON plus JSON Schema, signed envelopes, Ed25519, BLAKE3/content hashes, and native Overrid persistence/storage/private-ref boundaries.

## Exit Gate

SUB BUILD PLAN #39 is complete when the new plan is linked from the SDS, service catalog, master build plan, and service-catalog alignment docs; Phase 5 remains the first build point; Phase 8 remains the storage/namespace expansion gate; the plan has 10 phases with well-designed Design/Output/Validation work items; queue/progress docs record the pass; local link and structure validation pass; stack guardrail scans show no accidental conventional-cloud, NFT, speculative-market, external-payment, or broad-legal-title drift; and Docdex retrieval can find the new plan with SDS #39 context.
