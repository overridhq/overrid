SDS #40

# Overbill SDS

## Purpose

Handle billing records, invoices, receipts, payment-provider refs, taxes/compliance metadata, refunds, chargebacks, provider payout batches, payout holds, account statements, and audit exports around Overrid's internal ORU and Seal Ledger accounting layer.

Overbill is the bridge between internal resource accounting and external payment rails. It must keep external payment state linked and auditable without rewriting Seal Ledger history or introducing per-operation external payment friction.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overbill.md](../../service_catalog/accounting/overbill.md) |
| SDS sub-build plan | [SUB BUILD PLAN #40 - Overbill](../../build_plan/sub_build_plan_040_overbill.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) |

## Service Family

- Family: Accounting, credits, billing, and rights
- Owning layer: Billing documents, receipts, invoices, payment-provider refs, refunds, chargebacks, payout batch refs, payout holds, statements, and audit export
- Primary data scope: usage receipts, invoices, payment intents/refs, external payment events, refund records, chargeback records, payout batches, payout holds, tax/compliance metadata, account statements, audit exports, and reconciliation jobs
- First build phase from service plan: [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Problem Statement

Overrid needs billing records that make internal ORU usage understandable and externally payable where required. Users need receipts and statements. Providers need payout visibility. Disputes need holds and corrections. External payment providers may be needed for funding accounts or settling fiat obligations, but internal usage should not trigger a payment-provider call for every small operation.

Overbill must preserve the hierarchy: Overmeter records usage, Seal Ledger records internal accounting entries, ORU Account Service projects balances, and Overbill produces billing documents and external payment refs from those records. Refunds and chargebacks must create auditable corrections, not edits to old ledger history.

## Goals

- Create usage receipts from Seal Ledger and ORU transition refs.
- Create invoices and statements for users, organizations, apps, native services, providers, grant pools, and system-service accounts where needed.
- Integrate external payment providers behind a narrow adapter contract.
- Batch or aggregate external payment interactions where possible.
- Record refunds, chargebacks, failed payments, and correction refs append-only.
- Coordinate provider payout batches and payout holds through Provider Payout Service.
- Link every billing document to source ledger, ORU, Overmeter, Overclaim, and payment-provider refs.
- Support audit and compliance exports without leaking private workload or provider data.
- Keep billing structural and avoid pricing/customer-count/revenue projection assumptions.

## Non-Goals

- Do not create usage truth. Overmeter and Seal Ledger own usage and internal accounting truth.
- Do not mutate ORU balances directly. ORU Account Service projects balances from ledger entries.
- Do not execute provider payouts alone. Provider Payout Service owns payout workflow.
- Do not adjudicate disputes. Overclaim owns dispute state and correction/refund proposals.
- Do not call external payment providers for every small internal ORU transition.
- Do not store raw payment credentials or secrets outside the approved vault/payment-provider tokenization path.
- Do not implement speculative token economics, blockchain settlement, NFTs, or per-transaction tolls.

## Primary Actors And Clients

- Wallet and Usage Center, reading receipts, invoices, statements, refunds, and payment status.
- ORU Account Service, supplying account projections and wallet views.
- Seal Ledger, supplying append-only accounting entries and settlement refs.
- Overmeter, supplying usage rollups and dispute windows.
- Overclaim, supplying holds, refund proposals, correction proposals, release refs, and dispute finality.
- Provider Payout Service, receiving payout batch inputs and returning payout status refs.
- Payment provider adapters, handling external payment intents, confirmations, failures, refunds, and chargebacks.
- Admin UI, CLI, SDK, native apps, central AI stewardship, and compliance export consumers, reading authorized billing state and audit exports.

## Dependencies

- [Seal Ledger](seal_ledger.md) for reservation, settlement, hold, release, refund, correction, provider earning, grant allocation, native service usage, and system-service usage entries.
- [ORU Account Service](oru_account_service.md) for account projections, account state, wallet views, and account owner refs.
- [Overmeter](../execution_scheduling/overmeter.md) for signed usage rollups and dispute windows.
- [Overclaim](../trust_policy_verification/overclaim.md) for dispute holds, refund proposals, correction proposals, appeal state, and finality refs.
- [Provider Payout Service](provider_payout_service.md) for payout batch lifecycle, provider earning refs, hold refs, and payout status.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), and [Overkey](../control_plane/overkey.md) for billable identity, tenant scope, service accounts, and authorized access.
- [Overvault](../data_storage_namespace/overvault.md) for payment token refs, tax/compliance secret refs, and sensitive billing metadata where needed.
- [Overwatch](../control_plane/overwatch.md) for billing events, audit evidence, reconciliation alerts, and export refs.

## Owned Responsibilities

Overbill owns:

- Invoice, receipt, payment-provider ref, refund, chargeback, statement, payout batch input, payout hold view, and audit export schemas.
- Billing document lifecycle and idempotent document generation.
- External payment-provider adapter contract and event ingestion.
- Refund and chargeback records linked to correction/refund ledger refs.
- Reconciliation between billing docs, Seal Ledger entries, ORU projections, and payment-provider events.
- Statement and audit export generation with redaction.
- Payment and billing status views for wallet, admin, SDK, CLI, native apps, and providers.

Overbill must not edit Seal Ledger entries, ORU projections, or provider payout records directly. It records refs and sends requests to owning services.

## Data Model

The first implementation should define:

- `usage_receipt`: receipt id, account id, tenant id, source ledger refs, ORU dimension totals, usage rollup refs, policy refs, dispute window, issued-at, and redaction profile.
- `invoice`: invoice id, billing account refs, line item refs, source receipt refs, statement period, tax/compliance metadata refs, amount summary, payment state, due date, and audit refs.
- `invoice_line_item`: source ledger refs, ORU dimensions, service/native app/workload refs, grant refs, discount/credit refs where applicable, and redacted description.
- `payment_provider_ref`: provider name, external payment id, tokenized customer/payment refs, state, amount summary, idempotency key, created-at, updated-at, and vault refs.
- `external_payment_event`: provider ref, event type, external event id, signature validation state, raw payload ref or redacted summary, mapped state, and reconciliation refs.
- `refund_record`: refund id, invoice/receipt/payment refs, Overclaim refs, Seal Ledger correction/refund refs, provider refs, state, reason codes, and finality refs.
- `chargeback_record`: chargeback id, payment refs, external provider refs, disputed amount, evidence refs, Overclaim refs, state, deadline, and outcome refs.
- `payout_batch_input`: provider ids, earning ledger refs, hold refs, dispute refs, payout eligibility refs, batch window, and Provider Payout Service response refs.
- `payout_hold_view`: provider id, hold refs, source dispute/challenge/reputation refs, held dimensions/amounts, release condition, and expiry.
- `account_statement`: account id, period, opening/closing balance projection refs, receipt refs, invoice refs, refund/correction refs, payment refs, and export refs.
- `audit_export`: export id, scope, account/provider/tenant refs, redaction profile, included document refs, integrity hash, generated-at, and generated-by refs.
- `reconciliation_job`: scope, source ledger checkpoint, payment-provider event refs, billing doc refs, mismatch refs, repair action refs, and status.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account, `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

Overbill APIs are account-facing, provider-facing, operator-facing, and service-facing:

- `POST /billing/receipts`: create usage receipts from ledger/ORU refs.
- `GET /billing/receipts/{receipt_id}`: read authorized receipt.
- `POST /billing/invoices`: create invoice from receipts or statement period.
- `GET /billing/invoices/{invoice_id}`: read authorized invoice.
- `POST /billing/payments/intents`: create external payment intent/ref when needed.
- `POST /billing/payments/events`: ingest signed external payment-provider events.
- `POST /billing/refunds`: request or record refund from Overclaim/ledger refs.
- `POST /billing/chargebacks`: ingest or update chargeback state.
- `POST /billing/payout-batches`: prepare payout batch input for Provider Payout Service.
- `GET /billing/providers/{provider_id}/holds`: read provider payout hold view.
- `GET /billing/accounts/{account_id}/statements`: read or generate statement.
- `POST /billing/audit-exports`: generate redacted audit/compliance export.
- `POST /billing/reconcile`: reconcile billing docs, ledger refs, ORU projections, and external payment events.

API requirements:

- Mutating endpoints require actor/service identity, tenant context, trace id, and idempotency key.
- Payment event ingestion must verify provider signatures and idempotency.
- Billing documents must cite source ledger/ORU refs.
- Refund and chargeback flows must create new records and downstream correction refs, never edit historical receipts.
- Reads must enforce account, tenant, provider, role, and data-class access.

## Event Surface

- `overbill.receipt_created`: usage receipt created.
- `overbill.invoice_created`: invoice created.
- `overbill.payment_intent_created`: payment intent/ref created.
- `overbill.payment_event_received`: external payment event accepted.
- `overbill.payment_failed`: payment failure recorded.
- `overbill.payment_confirmed`: payment confirmation recorded.
- `overbill.refund_requested`: refund requested with source refs.
- `overbill.refund_recorded`: refund record created.
- `overbill.chargeback_opened`: chargeback received or opened.
- `overbill.chargeback_resolved`: chargeback outcome recorded.
- `overbill.payout_batch_prepared`: payout batch input prepared.
- `overbill.payout_hold_visible`: payout hold included in provider view.
- `overbill.statement_created`: account statement created.
- `overbill.audit_export_created`: redacted export generated.
- `overbill.reconciliation_mismatch`: mismatch found.
- `overbill.reconciliation_completed`: reconciliation completed.

Events must include billing document refs, source ledger refs, account/provider refs where applicable, trace id, and redaction class.

## Core Workflow

1. Overmeter produces signed usage rollups.
2. Seal Ledger creates append-only entries for reservation, settlement, hold, release, refund, correction, earning, grant allocation, native service usage, or system-service usage.
3. ORU Account Service projects balances and account states.
4. Overbill creates receipts, invoices, statements, and billing views from those refs.
5. External payment intents are created only when fiat funding or settlement requires external rails.
6. Payment-provider events are ingested, verified, mapped, and reconciled.
7. Overclaim disputes create holds, refunds, chargeback evidence, or correction requests.
8. Provider payout batch inputs are prepared after hold/dispute/reputation checks and handed to Provider Payout Service.
9. Refunds, chargebacks, and corrections create new records and downstream ledger/accounting refs.
10. Audit exports join billing docs, ledger refs, payment refs, and redaction profiles.

## State Machine

Invoice lifecycle:

1. `draft`: invoice is being assembled.
2. `issued`: invoice is issued.
3. `payment_pending`: external payment or internal settlement is pending.
4. `paid`: payment or internal settlement is confirmed.
5. `partially_paid`: partial payment is recorded.
6. `overdue`: due date passed.
7. `disputed`: Overclaim or chargeback is active.
8. `refunded`: refund record exists.
9. `corrected`: correction record exists.
10. `voided`: invoice invalidated by policy before finality.

Payment lifecycle:

1. `intent_created`: external payment intent/ref exists.
2. `authorized`: provider authorized payment.
3. `captured`: provider captured payment.
4. `failed`: payment failed.
5. `cancelled`: payment was cancelled.
6. `refunded`: refund initiated or completed.
7. `chargeback_open`: chargeback is active.
8. `chargeback_resolved`: chargeback outcome recorded.

Receipt and ledger history are append-only. Corrections create new records.

## Policy And Security

- Billing documents must cite source accounting refs and cannot be generated from untrusted local counters.
- External payment credentials and sensitive billing metadata must be tokenized or stored through approved vault/payment provider refs.
- Payment-provider webhook/event ingestion must verify signatures and idempotency.
- Refunds and chargebacks require Overclaim, provider event, or policy/evidence refs.
- Provider payout batches must check holds, disputes, reputation/anti-Sybil risk, and payout eligibility before handoff.
- User-facing documents must redact provider-private, workload-private, fraud, and secret-bearing details.
- External payment operations should be batched or aggregated where possible.
- No billing flow may rewrite Seal Ledger history.

## Metering And Accounting

Overbill sits on top of accounting refs:

- Overmeter supplies usage rollups.
- Seal Ledger supplies authoritative entries.
- ORU Account Service supplies account projections.
- Overbill supplies receipts, invoices, payment refs, refunds, chargebacks, statements, and exports.
- Provider Payout Service supplies payout statuses.
- Overclaim supplies dispute and correction refs.

Internal ORU transitions must work without external payment calls per tiny operation. External payments should fund, settle, refund, or reconcile aggregated obligations.

## Observability And Operations

- Dashboards should show receipts, invoices, payment states, failed payment events, refunds, chargebacks, payout batch prep, payout holds, reconciliation mismatches, and audit exports.
- Operators need account/provider billing timelines that join ledger, ORU, receipt, invoice, payment, refund, chargeback, payout, and claim refs.
- Alerts should fire on payment-provider signature failures, reconciliation mismatches, stuck refunds, chargeback deadlines, payout batches blocked by holds, and invoices generated without source refs.
- Reconciliation jobs must support scoped replay by account, tenant, provider, statement period, payment provider, and ledger checkpoint.

## Failure Modes And Recovery

- Missing source ledger refs: reject document generation.
- Payment provider outage: keep payment intent pending and avoid duplicate external calls.
- Duplicate payment event: dedupe by external event id and idempotency refs.
- Webhook signature failure: reject event and alert operators.
- Chargeback deadline approaching: alert and attach evidence refs.
- Refund downstream denied: record denial refs and hand back to Overclaim.
- Payout batch includes held provider: exclude or mark blocked with hold refs.
- Reconciliation mismatch: freeze affected document status, emit mismatch event, and run repair/replay.
- External payment succeeds but internal record fails: recover from provider event id and idempotency key.

## Validation Plan

The service implementation plan lists these requirements:

- Internal ORU transitions work without per-operation external payment calls.
- External payment state is linked but does not rewrite Seal Ledger history.
- Refunds and chargebacks create auditable corrections.

Additional SDS-level validation:

- Contract tests for receipt, invoice, payment intent, payment event, refund, chargeback, payout batch, hold view, statement, export, and reconciliation APIs.
- Ledger-link tests proving billing docs cannot be created without source refs.
- Payment adapter tests for idempotency, signed event validation, failure, cancellation, refund, and chargeback events.
- No-per-operation external payment tests for internal ORU settlement flows.
- Refund/chargeback tests proving corrections are append-only and cite Overclaim/ledger refs.
- Redaction tests for user, provider, operator, native app, and audit-export views.
- Reconciliation tests across Seal Ledger, ORU projections, invoices, receipts, and payment provider events.

## Build Breakdown

1. Define receipt, invoice, line item, payment ref, external payment event, refund, chargeback, payout batch input, payout hold view, statement, audit export, and reconciliation schemas.
2. Implement receipt and invoice generation from Seal Ledger and ORU refs.
3. Add wallet/account statement reads.
4. Add narrow payment provider adapter and signed event ingestion.
5. Add refund, chargeback, and correction records linked to Overclaim.
6. Add payout batch input and payout hold views for Provider Payout Service.
7. Add reconciliation jobs and audit exports.
8. Add dashboards, alerts, and redaction tests.

## Handoff And Downstream Use

Overbill connects Seal Ledger, ORU Account Service, Overclaim, Provider Payout Service, payment-provider adapters, Wallet and Usage Center, native apps, admin UI, SDK, CLI, compliance exports, and central AI stewardship.

## Open Design Questions

Resolved decisions:

- First payment-provider support is adapter-contract support, not a core dependency on a named payment SaaS. Phase 5 must ship a deterministic `sandbox_payment_adapter` for local/private validation plus one jurisdiction-selected real external-rail adapter for fiat account funding, refunds, chargebacks, and payout result events. Every real adapter must support tokenized customer, payment, and payout-destination refs, signed event ingestion, idempotent intents, refund and chargeback events, payout-result refs, reconciliation replay, and Overvault-backed sensitive metadata. Provider names remain deployment config or plugin choices; Overbill owns the contract and refs, not provider-specific business logic.
- The first private workload launch requires the minimum auditable document set: usage receipts from Seal Ledger and ORU refs, invoice line items, account statements, payment-provider refs only where external funding or settlement is used, refund records, payout batch inputs, payout hold views, reconciliation jobs, and redacted audit exports. Full invoices are required when an external fiat payment, funding event, organization billing period, or provider payout period needs a payable document; they are not generated for every internal ORU transition. Later native apps add richer wallet receipt collections, app/service statements, organization invoices, provider-visible payout status, chargeback documents, compliance exports, and user-triggered statement/export jobs.
- High-volume machine-to-machine usage is billed from rollup and settlement boundaries, not per operation. Phase 5 defaults are per lease/run settlement receipts, wallet-fresh summaries within the ORU Account Service checkpoint cadence, hourly settlement invoice candidates for active service accounts, and billing-cycle or on-demand invoices/statements for people, organizations, apps, providers, and native services. Statements and exports must refresh from a current Seal Ledger/ORU checkpoint before issuance, while external payment intents remain aggregated by account, statement period, funding threshold, payout period, or explicit operator/user action.
- Jurisdiction metadata is ref-based and supplied through Compliance Boundary Service, Overtenant/Overpass identity, Overvault, and payment-provider adapters. Required metadata includes billing country/region, account/person/organization/provider/native-service classification, tax profile and exemption/reverse-charge refs where applicable, tax id or identity-verification refs stored outside Overbill, billing address refs, service/resource category, purpose tag, place-of-supply or service-region refs, currency/payment rail, invoice sequence/localization requirements, retention/redaction profile, payment-provider customer/payment/refund/chargeback refs, and audit/evidence refs. Overbill stores bounded summaries and refs; raw tax documents, payment credentials, identity secrets, and private compliance payloads stay in Overvault or the approved provider/tokenization path.
- Money-moving reconciliation is blocking when source truth is missing or unsafe. Missing ledger/ORU refs, webhook signature failure, idempotency conflict, checkpoint/tamper mismatch, active dispute hold, compliance hold, or payout eligibility hold blocks the affected invoice, funding record, payout item, or batch immediately. Account funding may enter `payment_pending`, but spendable credit is unavailable until payment-provider event state, Overbill record, Seal Ledger entry, and ORU projection reconcile; Phase 5 should alert after 5 minutes of mismatch and freeze the affected funding record after 15 minutes. Payout batches require a clean eligibility and reconciliation checkpoint no older than 15 minutes before submission; any high-severity mismatch or unresolved submitted-payout mismatch beyond 15 minutes blocks only the affected batch/items and emits Overwatch evidence. Non-money-moving statement/export mismatches can be marked stale and retried without blocking unrelated accounts or payouts.
