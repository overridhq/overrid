# SUB BUILD PLAN #40 - Overbill

Attached SDS: [docs/sds/accounting/overbill.md](../sds/accounting/overbill.md)

## Purpose

This sub-build plan turns SDS #40 into an implementation sequence for Overbill. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overbill is a Phase 5 accounting and billing service. It owns receipts, invoices, invoice line items, payment-provider refs, signed external payment events, refund records, chargeback records, payout batch inputs, payout hold views, account statements, audit exports, and reconciliation jobs. It bridges internal ORU and Seal Ledger accounting to external payment rails only through auditable refs and adapters. It must not create usage truth, mutate ORU balances, rewrite Seal Ledger history, execute provider payouts alone, adjudicate disputes, store raw payment secrets, create per-operation external payment calls, or introduce pricing, revenue, customer-count, blockchain, NFT, or speculative-token assumptions.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #40: Overbill](../sds/accounting/overbill.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, accounting boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overbill plan](../service_catalog/accounting/overbill.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical schemas, signed envelopes, idempotency, trace ids, deterministic fixtures, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass billable identities, Overtenant tenant scope, Overkey signing refs, Overgate request discipline, Overwatch audit, Overregistry refs, and Overqueue-safe command context. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies private workload, lease, run, workload package, and raw usage refs that later become Overmeter rollups and accounting inputs. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy refs, Overclaim dispute/refund/correction refs, Workload Classifier facts, Oververify evidence, and Policy Dry-Run precheck refs. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Controls the first build point: billing schemas, receipts, invoices, payment-provider refs, refunds, chargebacks, payout batch inputs, payout holds, account statements, audit exports, reconciliation, and no per-operation external payment rails. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service operation, failover, restore, maintenance, and grid-resident hardening for billing workers and reconciliation jobs. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase persistence, Overstore statement/export artifacts, Overvault payment/tax/compliance/private refs, namespace refs, retention, backup/restore, and migration handoffs. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies public-interest grant, purpose-tag, reporting, and federation context that may affect billing summaries and export scopes. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider payout visibility, reputation/fraud/payout-hold constraints, sandbox accounting limits, and public low-sensitivity resource billing boundaries. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies Wallet and Usage Center, SDK, CLI, admin UI, native apps, and central AI consumers of authorized billing records and statements. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance boundaries, incident handoffs, threat-model reviews, stewardship reports, audit exports, retention controls, migration governance, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #40 first build work aligned to master Phase 5, with earlier control/execution/trust prerequisites and later grid, storage/private-ref, public-provider, native-app, and governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, pricing, revenue, customer-count, or external-payment-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 8, 11, 12, and 13 | Attach SDS #40, freeze billing authority, preserve Phase 5 as first build point, and record prerequisites plus expansion gates. |
| 2 | Master Phases 0, 1, 4, and 5 | Build Rust contracts, schemas, billing document states, adapter refs, reason codes, fixtures, and replay commitments. |
| 3 | Master Phases 3, 4, and 5 | Implement receipt, invoice, line-item, and billing-document generation from Seal Ledger, ORU Account Service, and Overmeter refs. |
| 4 | Master Phases 5, 8, 12, and 13 | Build account statements, wallet/admin/provider read models, authorized export views, redaction, and consumer handoffs. |
| 5 | Master Phases 5, 8, and 13 | Implement sandbox and real payment-provider adapter contracts, payment intents, signed event ingestion, tokenized refs, and idempotent external state mapping. |
| 6 | Master Phases 4, 5, and 13 | Implement refunds, chargebacks, corrections, dispute-linked invoice/payment states, and append-only accounting handoffs through Overclaim and Seal Ledger refs. |
| 7 | Master Phases 5, 10, 11, and 13 | Implement payout batch inputs, payout hold views, public-provider eligibility checkpoints, Provider Payout Service handoffs, and blocked-item visibility. |
| 8 | Master Phases 5, 8, and 13 | Build reconciliation jobs, blocking rules, jurisdiction/compliance refs, audit exports, retention, integrity hashes, and repair queues. |
| 9 | Master Phases 7, 8, 12, and 13 | Harden operations, replay, backfill, grid-resident workers, incident response, dashboards, alerts, and scale controls. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, billing authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Overbill core is a Rust service/module using shared contract types, Tokio for bounded async adapter/reconciliation workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Receipts, invoices, line items, payment refs, external payment events, refund records, chargeback records, payout batch inputs, payout hold views, statements, audit exports, reconciliation jobs, API objects, events, fixtures, and reason-code catalogs use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating endpoints require signed actor or service envelopes, tenant context, account/provider refs, trace id, idempotency key, schema version, source ledger/ORU refs, policy refs, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for source refs, payment-event replay bundles, statement/export bundles, fixture integrity, and deterministic reconciliation comparison tests.
- Sensitive payment, tax, identity-verification, payout-destination, and compliance payloads stay in Overvault or approved payment-provider tokenization paths. Overbill stores bounded summaries, tokenized refs, redacted event summaries, and audit refs.
- Payment-provider support is adapter-contract support, not a core dependency on a named payment SaaS. Phase 5 must ship a deterministic `sandbox_payment_adapter` for local/private validation plus one jurisdiction-selected real external-rail adapter where deployment policy requires fiat funding, refunds, chargebacks, or payout-result events.
- Internal ORU transitions must not call external payment providers per tiny operation. External payment intents are aggregated by account, statement period, funding threshold, payout period, or explicit operator/user action.
- Overbill may later persist billing records through Overbase, store statement/export artifacts through Overstore, and store private/token/compliance refs through Overvault. It must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative tokens, pricing tables, revenue projections, or customer-count assumptions the platform boundary.
- Overbill never creates usage truth, appends Seal Ledger entries directly, mutates ORU projections, owns payout execution, adjudicates disputes, or bypasses Compliance Boundary Service for jurisdiction metadata.

## Phase 1: SDS Attachment, Billing Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #40.**
  - Design: Link this document from the Overbill SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/accounting/overbill.md`, `docs/service_catalog/accounting/overbill.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #40 returns both the Overbill SDS and this sub-build plan.

- **1.2 Freeze Overbill authority boundaries.**
  - Design: Record that Overbill owns billing document schemas, lifecycle state, external payment-provider refs, signed payment event ingestion, refunds, chargebacks, payout batch inputs, hold views, statements, audit exports, and reconciliation jobs.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms it does not own Overmeter usage truth, Seal Ledger append-only accounting truth, ORU balance projection, Provider Payout execution, Overclaim finality, raw payment credentials, or Compliance Boundary jurisdiction authority.

- **1.3 Preserve master Phase 5 as the first build point.**
  - Design: Keep first implementation in Phase 5 because Overbill requires Phase 0 contracts, Phase 1 identity/audit, Phase 3 usage refs, Phase 4 policy/dispute refs, and Phase 5 ORU/Seal Ledger accounting primitives.
  - Output: Phase-gate note that Phase 0 through Phase 4 are prerequisites, Phase 5 is first build, and Phases 7, 8, 10, 11, 12, and 13 are expansion or hardening gates.
  - Validation: Review proves this plan does not move billing before authoritative usage/accounting refs and does not defer core receipts/invoices behind native apps or public providers.

- **1.4 Carry forward resolved SDS #40 decisions.**
  - Design: Preserve adapter-contract-first payment support, minimum private-launch document set, rollup/settlement-boundary billing, jurisdiction metadata as refs, and blocking reconciliation for unsafe money-moving mismatches.
  - Output: Resolved-decision checklist tied to implementation reviews.
  - Validation: Review rejects named payment-provider lock-in, full invoices for every internal ORU transition, raw tax/payment secrets in Overbill, spending credit before payment reconciliation, and non-blocking behavior for unsafe money-moving mismatches.

- **1.5 Define owner-service and consumer boundaries.**
  - Design: Create a dependency matrix for Overmeter, Seal Ledger, ORU Account Service, Overclaim, Provider Payout Service, Overpass, Overtenant, Overkey, Overvault, Overwatch, Compliance Boundary Service, payment-provider adapters, Wallet and Usage Center, admin UI, SDK, CLI, native apps, and central AI stewardship.
  - Output: Boundary matrix listing consumed refs, emitted billing refs, final authority owner, redaction class, reconciliation checkpoint, replay evidence, blocking behavior, and later phase gate.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, trace ids, policy refs, and Overwatch audit instead of direct privileged state reads.

## Phase 2: Rust Contracts, Schemas, Billing States, And Fixtures

### Work Items

- **2.1 Create the Overbill Rust contract module.**
  - Design: Add contract types for usage receipts, invoices, line items, payment-provider refs, external payment events, refund records, chargeback records, payout batch inputs, payout hold views, account statements, audit exports, reconciliation jobs, API errors, and events.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Seal Ledger, ORU Account Service, Overmeter, Overclaim, Provider Payout Service, and Overvault internals.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for receipt create/read, invoice create/read, line item read, payment intent, payment event, refund, chargeback, payout batch input, hold view, statement, audit export, reconciliation, events, and replay bundles.
  - Output: Schema files, valid examples, invalid examples, generated validators, compatibility notes, and docs-facing examples.
  - Validation: Schema tests reject missing tenant scope, account/provider refs, source ledger/ORU refs, actor/service identity, trace id, idempotency key, policy refs, redaction class, or audit refs where required.

- **2.3 Define billing document and reason-code catalogs.**
  - Design: Encode document classes, source-ref kinds, payment states, refund/chargeback reason codes, payout-hold reason codes, mismatch reason codes, jurisdiction-ref kinds, redaction classes, and export scopes.
  - Output: Catalog files, review tables, schema enums, fixture references, and compatibility notes for clients and audit exports.
  - Validation: Tests prove every catalog entry names its source authority, allowed state transitions, redaction profile, replay inputs, and consumer visibility.

- **2.4 Model invoice, payment, refund, chargeback, payout, and reconciliation state machines.**
  - Design: Encode invoice states from draft through paid/disputed/refunded/corrected/voided, payment states from intent through captured/failed/refunded/chargeback, and reconciliation states from queued through mismatch/blocked/repaired/complete.
  - Output: Transition tables, Rust enums, JSON Schema enums, event emission rules, invalid transition fixtures, and lifecycle review notes.
  - Validation: State tests reject direct history edits, paid state without source refs, spendable funding before reconciliation, chargeback closure without evidence refs, and payout submission with active holds.

- **2.5 Create deterministic Overbill fixtures.**
  - Design: Build fixtures for receipt generation, invoice generation, payment intent creation, signed provider events, refund records, chargeback records, payout batch inputs, payout holds, statements, audit exports, reconciliation mismatch, replay, redaction, and duplicate event handling.
  - Output: Fixture directory, expected projections, reason codes, Overwatch events, statement/export examples, adapter examples, and invalid examples.
  - Validation: Fixture tests produce stable document ids, state transitions, redaction behavior, replay hashes, mismatch classifications, and idempotency outcomes across repeated runs.

## Phase 3: Receipts, Invoices, Line Items, And Billing Document Generation

### Work Items

- **3.1 Implement source accounting ref resolution.**
  - Design: Resolve Seal Ledger entries, ORU account projections, Overmeter rollups, account owner refs, policy refs, dispute windows, grant refs, and workload/app/native-service refs before document creation.
  - Output: Source resolver interfaces, source-service allowlist, checkpoint validation, freshness rules, and unavailable-source behavior.
  - Validation: Tests reject billing document creation from untrusted counters, missing ledger refs, stale checkpoints, mismatched tenant/account refs, and unsigned or unhashable evidence.

- **3.2 Implement usage receipt generation.**
  - Design: Create receipts from Seal Ledger and ORU refs at lease/run settlement, rollup, service-account, native-service, or explicit statement boundaries rather than per operation.
  - Output: `POST /billing/receipts` handler, idempotent receipt generator, receipt read API, receipt events, source-ref index, and redaction profiles.
  - Validation: API tests prove receipts cannot be created without source refs and internal ORU transitions do not trigger per-operation external payment calls.

- **3.3 Implement invoice and line item generation.**
  - Design: Create invoices from receipts, billing periods, external funding obligations, organization billing periods, provider payout periods, or explicit user/operator actions.
  - Output: `POST /billing/invoices` handler, line-item generator, invoice read API, invoice lifecycle events, due-date policy fields, and amount-summary fields.
  - Validation: Tests prove full invoices are not generated for every internal ORU transition and every invoice line item cites source receipt, ledger, ORU, grant, workload/app/service, tax/compliance, and audit refs as applicable.

- **3.4 Enforce document idempotency and aggregation boundaries.**
  - Design: Use idempotency keys, statement windows, ledger checkpoints, receipt refs, funding thresholds, payout periods, and explicit action refs to prevent duplicate receipts or invoices.
  - Output: Idempotency store contract, duplicate detection, aggregation-window rules, conflict reason codes, and operator review fields.
  - Validation: Tests prove duplicate commands and duplicate source refs do not create duplicate payable documents, and idempotency conflicts are visible without mutating historical records.

- **3.5 Emit billing document events and audit evidence.**
  - Design: Emit receipt, invoice, line-item, blocked-generation, duplicate, and stale-source events with source refs, account/provider refs, tenant context, trace id, policy refs, redaction class, and audit refs.
  - Output: Event contracts, Overwatch handoff refs, metrics counters, dashboard fields, and replay inputs.
  - Validation: Observability tests prove every document creation or rejection emits append-only evidence and no event mutates Seal Ledger, ORU Account Service, Provider Payout Service, or Overclaim state.

## Phase 4: Statements, Billing Views, Redaction, And Consumers

### Work Items

- **4.1 Implement account statements.**
  - Design: Generate account statements by period from ORU projection checkpoints, receipt refs, invoice refs, refund/correction refs, payment refs, and source ledger checkpoints.
  - Output: `GET /billing/accounts/{account_id}/statements`, statement generator, statement event, statement export ref, and freshness requirements.
  - Validation: Tests prove statements refresh from current Seal Ledger/ORU checkpoints before issuance and stale statements are marked rather than silently treated as current.

- **4.2 Build Wallet and Usage Center read models.**
  - Design: Expose user-safe receipt collections, invoices, payment status, refund status, chargeback status, account statements, and redacted audit/export links.
  - Output: Wallet read model, pagination, status summaries, stable reason codes, user-facing redaction profiles, and correction/refund explanation fields.
  - Validation: Security tests reject provider-private, fraud, tax, payment-token, raw ledger, and secret-bearing detail leakage in wallet views.

- **4.3 Build provider and organization billing views.**
  - Design: Expose provider-visible payout batch input status, payout holds, earning refs, chargeback exposure, statement periods, and organization invoice/status views without moving payout authority into Overbill.
  - Output: Provider/organization read APIs, blocked-item summaries, payout-status refs, redaction profiles, and dashboard handoff fields.
  - Validation: Tests prove provider views cannot reveal other-provider data, raw fraud/compliance internals, private workload payloads, or payout execution controls owned by Provider Payout Service.

- **4.4 Build admin, SDK, CLI, native-app, and central AI read handoffs.**
  - Design: Provide stable contract-backed read surfaces for authorized billing timelines, statements, audit refs, reconciliation status, and explanation summaries.
  - Output: API response contracts, SDK/CLI examples, admin view fields, native-app fields, central AI summary fields, and role-based access profiles.
  - Validation: Integration tests prove clients consume Overbill refs through Overgate/authorized APIs and do not rely on direct storage, raw payment-provider payloads, or privileged service internals.

- **4.5 Implement redaction and access-policy enforcement.**
  - Design: Apply account, tenant, provider, role, compliance, fraud, payment-token, private-workload, native-app, operator, and audit-export redaction classes consistently across reads and exports.
  - Output: Redaction policy map, access middleware hooks, fixtures for each actor class, and stable denial reason codes.
  - Validation: Redaction tests prove each view exposes enough status for remediation while hiding provider-private, user-private, fraud, tax, compliance, secret, and raw payment-provider details from unauthorized readers.

## Phase 5: Payment-Provider Adapter Contracts And Event Ingestion

### Work Items

- **5.1 Implement the deterministic sandbox payment adapter.**
  - Design: Provide local/private validation for payment intents, authorizations, captures, failures, cancellations, refunds, chargebacks, payout-result refs, signatures, and replay without depending on external payment SaaS.
  - Output: `sandbox_payment_adapter`, deterministic fixtures, signed event generator, failure-mode scenarios, and replay examples.
  - Validation: Adapter tests prove deterministic behavior, idempotent event ids, signature validation, duplicate event handling, refund/chargeback coverage, and no network dependency for local validation.

- **5.2 Define real payment-provider adapter requirements.**
  - Design: Specify a deployment-configured adapter interface for tokenized customer/payment/payout-destination refs, signed events, idempotent intents, refunds, chargebacks, payout-result refs, and reconciliation replay.
  - Output: Adapter trait, capability manifest, configuration schema, provider-name-as-config rule, and conformance tests.
  - Validation: Review proves provider-specific business logic stays in adapters/plugins and Overbill owns only contract refs, state mapping, idempotency, audit, and reconciliation.

- **5.3 Implement payment intent creation.**
  - Design: Create external payment intents only when fiat funding, settlement, refund, or explicit user/operator action requires external rails, using tokenized refs and aggregation boundaries.
  - Output: `POST /billing/payments/intents`, idempotency handling, payment-provider ref records, state transitions, and `overbill.payment_intent_created` events.
  - Validation: Tests prove payment intents are not created for every small internal ORU transition and spendable credit is unavailable until provider event state, Overbill record, Seal Ledger entry, and ORU projection reconcile.

- **5.4 Implement signed payment event ingestion.**
  - Design: Ingest provider webhooks/events with signature verification, external event id dedupe, state mapping, raw payload refs or redacted summaries, and mismatch detection.
  - Output: `POST /billing/payments/events`, signature verifier, event mapper, duplicate detector, mismatch records, and alert events.
  - Validation: Tests reject unsigned events, invalid signatures, duplicate events, idempotency conflicts, unknown payment refs, and events that would silently rewrite historical billing state.

- **5.5 Link payment events to Overvault and reconciliation refs.**
  - Design: Store raw payment credentials, customer tokens, payment tokens, payout destination refs, tax secrets, and private compliance payloads outside Overbill through Overvault or approved provider tokenization.
  - Output: Vault-ref contracts, redacted payment-event summaries, payment replay bundle fields, audit refs, and data-class labels.
  - Validation: Security tests prove no raw credential, tax document, identity secret, or private compliance payload is stored in Overbill records or exported in unauthorized views.

## Phase 6: Refunds, Chargebacks, Corrections, And Dispute-Linked Billing State

### Work Items

- **6.1 Implement refund records.**
  - Design: Create refund records from Overclaim refs, provider refs, invoice/receipt/payment refs, Seal Ledger correction/refund refs, reason codes, state, and finality refs.
  - Output: `POST /billing/refunds`, refund record schema, refund state machine, provider-ref handoff, and `overbill.refund_recorded` events.
  - Validation: Tests prove refunds create append-only records and downstream correction refs instead of editing receipts, invoices, payment events, or ledger history.

- **6.2 Implement chargeback records.**
  - Design: Ingest or update chargeback state with provider refs, disputed amount summary, evidence refs, Overclaim refs, deadlines, state, outcome refs, and redacted views.
  - Output: `POST /billing/chargebacks`, chargeback record schema, deadline alerts, evidence bundle refs, and `overbill.chargeback_opened`/`overbill.chargeback_resolved` events.
  - Validation: Tests prove chargebacks do not reverse Seal Ledger entries directly, deadlines alert operators, and evidence views respect fraud, provider, compliance, and user redaction classes.

- **6.3 Implement correction handoffs.**
  - Design: Turn billing/refund/chargeback outcomes into correction request refs for owning accounting services while preserving original document state and replacement refs.
  - Output: Correction handoff contract, ledger/accounting ref fields, Overclaim finality links, replacement document refs, and explanation updates.
  - Validation: Tests prove corrections never rewrite old billing documents and replay can show original state, correction evidence, replacement refs, and resulting projections.

- **6.4 Implement dispute-linked invoice and payment states.**
  - Design: Map Overclaim disputes, chargebacks, payout holds, refund requests, and compliance holds to invoice/payment states without making Overbill the dispute adjudicator.
  - Output: State mapping table, blocked/review-required fields, read-model status explanations, and hold/release event refs.
  - Validation: Tests prove active disputes or unsafe chargebacks block affected money-moving records while unrelated accounts, statements, and payouts continue where safe.

- **6.5 Emit append-only dispute, refund, and chargeback events.**
  - Design: Emit events for refund requests, refund records, chargeback open/resolved, correction requested, invoice disputed, payment failed, and payment confirmed with source refs and redaction classes.
  - Output: Event contracts, Overwatch handoff refs, replay inputs, dashboard counters, and export fields.
  - Validation: Event tests prove every state change has actor/service identity, tenant context, trace id, idempotency key, source refs, policy refs, evidence refs, and no destructive history update.

## Phase 7: Payout Batch Inputs, Hold Views, And Provider Payout Handoffs

### Work Items

- **7.1 Implement payout batch input preparation.**
  - Design: Build payout batch inputs from provider ids, earning ledger refs, hold refs, dispute refs, payout eligibility refs, batch windows, and reconciliation checkpoints.
  - Output: `POST /billing/payout-batches`, payout batch input schema, source-ref validator, batch status refs, and `overbill.payout_batch_prepared` events.
  - Validation: Tests prove Overbill prepares inputs only and does not execute payouts, mutate provider payout records, or submit batches with stale/unsafe reconciliation checkpoints.

- **7.2 Implement provider payout hold views.**
  - Design: Expose held provider amounts/dimensions from Overclaim, fraud/reputation, compliance, challenge, operator, and payout eligibility refs with release conditions and expiry.
  - Output: `GET /billing/providers/{provider_id}/holds`, hold view schema, redaction profiles, release-condition fields, and hold visibility events.
  - Validation: Tests prove held provider values cannot be paid out, hold release denial keeps held state, and provider views redact sensitive fraud, compliance, and dispute internals.

- **7.3 Integrate Provider Payout Service handoffs.**
  - Design: Send batch input refs to Provider Payout Service and store returned payout status refs, blocked item refs, submitted batch refs, and payout-result refs without owning execution.
  - Output: Provider Payout adapter interface, response-ref schema, status update records, and reconciliation input fields.
  - Validation: Integration tests prove Provider Payout Service remains the payout lifecycle owner and Overbill only records inputs, refs, statuses, and reconciliation evidence.

- **7.4 Enforce public-provider and low-sensitivity constraints.**
  - Design: Apply Phase 11 public-provider constraints, reputation/anti-Sybil recommendations, fraud refs, sandbox profile refs, and low-sensitivity eligibility before payout batch preparation.
  - Output: Eligibility-check fields, blocked payout reason codes, public-provider hold fixtures, and operator review fields.
  - Validation: Tests prove unresolved anti-Sybil/fraud/eligibility holds block affected payout items while preserving provider-visible status and audit refs.

- **7.5 Link payout visibility into statements and exports.**
  - Design: Include provider payout batch input refs, hold refs, payout status refs, payout-result refs, and blocked-item summaries in provider statements and audit exports.
  - Output: Statement fields, export fields, provider timeline projection, and redacted summaries.
  - Validation: Tests prove provider statements show enough payout status for remediation without exposing other providers, raw fraud evidence, payment credentials, or internal operator-only data.

## Phase 8: Reconciliation Jobs, Compliance Refs, And Audit Exports

### Work Items

- **8.1 Implement reconciliation jobs.**
  - Design: Reconcile billing docs, Seal Ledger checkpoints, ORU projections, payment-provider event refs, refund/chargeback records, payout batch inputs, and Provider Payout status refs by scoped job.
  - Output: `POST /billing/reconcile`, reconciliation worker, job state machine, mismatch records, repair action refs, and `overbill.reconciliation_completed` events.
  - Validation: Tests prove jobs can run by account, tenant, provider, payment provider, statement period, payout batch, and ledger checkpoint with deterministic mismatch classification.

- **8.2 Enforce blocking rules for unsafe money-moving mismatches.**
  - Design: Block affected invoices, funding records, payout items, or batches on missing ledger/ORU refs, webhook signature failure, idempotency conflict, checkpoint mismatch, active dispute hold, compliance hold, or payout eligibility hold.
  - Output: Blocking policy table, five-minute alert rule, fifteen-minute freeze rule, blocked-state events, and operator review queue fields.
  - Validation: Tests prove unsafe money-moving mismatches block only affected records, while non-money-moving statement/export mismatches can be marked stale and retried without blocking unrelated accounts.

- **8.3 Implement redacted audit exports.**
  - Design: Generate export bundles from billing docs, ledger refs, ORU refs, payment refs, refund/chargeback refs, payout refs, reconciliation refs, jurisdiction refs, redaction profile, integrity hash, and generated-by refs.
  - Output: `POST /billing/audit-exports`, export bundle schema, BLAKE3 integrity hash, Overstore artifact refs, Overvault private refs, and export events.
  - Validation: Export tests prove included docs are source-linked, integrity hashes are stable, raw secrets are omitted, and role-scoped exports differ only by redaction rules.

- **8.4 Implement jurisdiction and compliance metadata refs.**
  - Design: Consume ref-based metadata from Compliance Boundary Service, Overtenant/Overpass identity, Overvault, and payment-provider adapters without storing raw tax, identity, or payment secrets.
  - Output: Jurisdiction metadata contract, tax/compliance ref fields, invoice localization refs, retention/redaction profile refs, and compliance handoff fields.
  - Validation: Compliance tests prove invoices and exports carry required billing country/region, account classification, tax profile refs, service/resource category, purpose tag, place-of-supply/service-region refs, currency/payment rail refs, and audit/evidence refs.

- **8.5 Implement retention, replay, and export integrity controls.**
  - Design: Link records to retention policy, redaction profile, replay bundle, export hash, source checkpoints, and migration refs so old billing history remains auditable.
  - Output: Retention metadata, replay bundle writer, export integrity verifier, migration notes, and stale/export repair commands.
  - Validation: Tests prove replay reconstructs billing state from refs, detects missing evidence, preserves historical states, and never widens data visibility during export or migration.

## Phase 9: Operations, Replay, Grid Hardening, And Incident Response

### Work Items

- **9.1 Build Overbill dashboards and alerts.**
  - Design: Track receipts, invoices, payment states, failed events, refunds, chargebacks, payout batch prep, payout holds, reconciliation mismatches, stale statements, audit exports, and blocked money-moving records.
  - Output: Metrics schema, dashboard definitions, alert rules, Overwatch aggregation, and runbook links.
  - Validation: Operations tests prove alerts fire on signature failures, reconciliation mismatches, stuck refunds, chargeback deadlines, payout batches blocked by holds, and billing docs without source refs.

- **9.2 Implement scoped replay and repair workflows.**
  - Design: Reconstruct account/provider billing timelines from source refs, payment events, ledger checkpoints, ORU checkpoints, refund/chargeback records, payout refs, and audit events.
  - Output: Replay command/API, repair queue, mismatch review fields, old/new projection refs, and repair outcome events.
  - Validation: Replay tests reconstruct current and historical state, detect missing source refs, keep unsafe records blocked, and avoid silently changing invoice/payment/payout states.

- **9.3 Harden grid-resident workers.**
  - Design: Move billing workers, adapter event ingestion, reconciliation jobs, statement/export generation, and replay tasks into protected grid-resident operation when Phase 7 prerequisites exist.
  - Output: Worker supervision model, lease/system-service refs, failover/restore behavior, backpressure limits, and maintenance runbooks.
  - Validation: Failover tests prove idempotency prevents duplicate external calls or duplicate documents after restart, restore, retry, or worker handoff.

- **9.4 Implement backfill, migration, and checkpoint management.**
  - Design: Support bounded backfill from ledger/payment/payout refs, schema-version migration, checkpoint repair, export reissue, and redaction-profile changes without history mutation.
  - Output: Migration commands, checkpoint tables, backfill runbooks, replay verification, and operator approval gates.
  - Validation: Migration tests prove old documents remain readable and replayable, new projections cite migration refs, and backfills cannot create payable documents without source refs.

- **9.5 Add incident-response and threat-model controls.**
  - Design: Define incident flows for payment-provider compromise, signature failures, duplicated events, payout-blocking mismatches, refund abuse, chargeback deadlines, export leaks, and compliance holds.
  - Output: Threat model checklist, incident runbooks, escalation refs, freeze/release commands, audit export procedures, and stewardship report fields.
  - Validation: Security review proves incidents can freeze affected money-moving records without shutting down unrelated internal ORU accounting or rewriting history.

## Phase 10: Validation, Documentation Alignment, Queue Closure, And Handoff

### Work Items

- **10.1 Run contract and API validation.**
  - Design: Validate every Overbill API, schema, state transition, event, fixture, and error path against the SDS and source alignment table.
  - Output: Contract tests for receipt, invoice, payment intent, payment event, refund, chargeback, payout batch, hold view, statement, audit export, reconciliation, and replay APIs.
  - Validation: Test evidence proves mutating endpoints require signed envelopes, tenant context, trace id, idempotency key, source refs, policy refs, and stable reason codes.

- **10.2 Run integration validation across accounting and payment boundaries.**
  - Design: Validate Overbill with Overmeter, Seal Ledger, ORU Account Service, Overclaim, Provider Payout Service, Overvault, Overwatch, Compliance Boundary Service, and payment-provider adapters.
  - Output: Integration test matrix, service boundary assertions, fixture traces, replay bundles, and mismatch scenarios.
  - Validation: Tests prove Overbill does not create usage truth, mutate ORU balances, rewrite Seal Ledger history, execute payouts, adjudicate disputes, or store raw secrets.

- **10.3 Run guardrail and negative-control scans.**
  - Design: Scan implementation and docs for conventional cloud-product drift, raw payment-secret storage, per-operation external payment calls, named-provider lock-in, pricing/revenue/customer-count assumptions, blockchain/NFT language, and mutable-token behavior.
  - Output: Guardrail scan report, allowed negative-control list, review notes, and required corrections.
  - Validation: Scan passes with only allowed native Overrid service names, adapter-contract wording, and explicit non-choice guardrails.

- **10.4 Keep documentation and queue state aligned.**
  - Design: Link this plan from the Overbill SDS, service catalog entry, master build plan, build-plan crosswalk, build-plan progress, and Codex55 queue progress.
  - Output: Updated docs, queue state, queue progress, Docdex targeted index refresh, and search evidence.
  - Validation: Local Markdown link checks, JSON validation, Docdex search, Docdex stats, and queue validation all pass or record explicit blockers.

- **10.5 Prepare implementation handoff.**
  - Design: Produce a handoff package naming crates/modules, schemas, APIs, fixtures, tests, phase gates, owner services, non-goals, redaction classes, adapter contracts, and validation commands.
  - Output: Implementation-ready checklist and acceptance criteria for Overbill Phase 5 work plus later Phase 7/8/10/11/12/13 expansion.
  - Validation: Review confirms the plan is internally consistent, aligned with SDS #40, aligned with docs/build_plan conventions, aligned with master phases 0 through 13, and compliant with `docs/overrid_tech_stack_choice.md`.
