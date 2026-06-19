SDS #44

# Seal Ledger SDS

## Purpose

Provide append-only internal accounting for ORU reservations, settlements, holds, releases, refunds, corrections, provider earnings, grant allocations, native service usage, system-service usage, dispute refs, and settlement history.

Seal Ledger is Overrid's accounting truth layer. It replaces the need for blockchain-style settlement inside the ecosystem by using signed, append-only, replayable entries with explicit evidence refs. It is not a cryptocurrency, public blockchain, mining network, speculative token ledger, NFT ledger, or per-transaction fee mechanism.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [seal_ledger.md](../../service_catalog/accounting/seal_ledger.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) |

## Service Family

- Family: Accounting, credits, billing, and rights
- Owning layer: append-only internal accounting entries, sequence integrity, entry validation, derived accounting indexes, reconciliation, checkpointing, and audit export
- Primary data scope: ledger streams, ledger entries, entry batches, account refs, resource dimensions, source evidence refs, hash/checkpoint refs, query indexes, correction refs, export bundles, and replay proofs
- First build phase from service plan: [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Problem Statement

Overrid needs accountably metered resource usage without blockchain greed mechanics, speculative tokens, NFT ownership claims, or external payment calls for every small machine-to-machine operation. Usage must turn into durable accounting facts that can be queried, audited, corrected, and replayed.

The ill-design to avoid is a mutable wallet table where balances are overwritten, or a blockchain copy that introduces fees, speculation, mining, and governance drift. Seal Ledger must be simpler and stricter: append entries, validate invariants, preserve evidence, derive balances elsewhere, and correct by new entries only.

## Goals

- Define append-only ledger entry schemas and invariants for ORU accounting.
- Accept idempotent append commands from trusted accounting producers.
- Record reservation, settlement, hold, release, refund, correction, provider earning, grant allocation, native service usage, and system-service usage entries.
- Preserve source refs from Overmeter rollups, Overguard decisions, Overclaim disputes, Overgrant authorizations, Overbill refs, provider payout refs, and operator actions.
- Provide deterministic query indexes by account, tenant, workload, provider, dispute, grant, native service, system service, and resource dimension.
- Support replay, reconciliation, checkpoint verification, audit export, and correction-by-new-entry.
- Keep internal accounting independent from external payment rails and speculative-token mechanics.

## Non-Goals

- Do not expose mutable balance counters as accounting truth. ORU Account Service derives balance projections.
- Do not create billing documents, invoices, receipts, or payment-provider refs. Overbill owns those.
- Do not calculate resource reference bands or placement hints. Overmark owns those.
- Do not own raw usage measurement. Overmeter owns signed rollups.
- Do not adjudicate disputes. Overclaim owns dispute state and finality.
- Do not execute payouts. Provider Payout Service and Overbill coordinate payout batches and external rails.
- Do not implement public blockchain consensus, mining, token trading, NFT minting, gas fees, or transaction tolls.

## Primary Actors And Clients

- Overmeter, submitting signed usage rollup settlement candidates.
- ORU Account Service, deriving account state and wallet views from ledger entries.
- Overbill, producing receipts, invoices, statements, refunds, and external payment refs from ledger history.
- Provider Payout Service, deriving provider earning and hold/release refs.
- Overgrant, referencing grant allocation, reservation, usage, and correction entries.
- Overclaim, linking holds, releases, refunds, corrections, appeals, and finality refs.
- Wallet and Usage Center, reading authorized ledger-derived views through ORU Account Service and Overbill.
- Overwatch, audit export consumers, central AI stewardship, and compliance tools, reading signed/replayable evidence.
- Backup and Restore Service, verifying ledger backup/restore integrity.

## Dependencies

- [ORU Account Service](oru_account_service.md) for account refs, account dimensions, and derived balance projections.
- [Overmeter](../execution_scheduling/overmeter.md) for signed usage rollups and dispute windows.
- [Overguard](../trust_policy_verification/overguard.md) for policy decision refs where accounting transitions depend on admission or enforcement.
- [Overclaim](../trust_policy_verification/overclaim.md) for dispute, hold, release, refund, correction, and finality refs.
- [Overbill](overbill.md) for billing document refs and external payment-provider refs.
- [Overgrant](overgrant.md) for grant source, authorization, usage, and reporting refs.
- [Provider Payout Service](provider_payout_service.md) for payout batch, payout result, reversal, and correction refs.
- [Overwatch](../control_plane/overwatch.md) for audit events and integrity evidence.
- [Overkey](../control_plane/overkey.md) for signing key metadata and verification refs.

## Owned Responsibilities

Seal Ledger owns:

- Append-only entry stream definitions and sequence invariants.
- Entry validation by schema, producer authority, idempotency key, evidence refs, account refs, dimensions, and state transition rules.
- Entry batches and atomic append semantics where a transition needs multiple account-side records.
- Ledger stream partitioning by tenant/system scope where appropriate.
- Query indexes by account, tenant, workload, provider, dispute, grant, native service, system service, source rollup, and resource dimension.
- Checkpoint/hash-chain refs for integrity verification. These are integrity proofs, not blockchain consensus.
- Reconciliation reports against Overmeter, ORU Account Service, Overbill, Overgrant, Provider Payout, and Overclaim refs.
- Audit export bundles and replay proofs.
- Correction-by-new-entry semantics and supersession refs.

Seal Ledger does not own mutable wallet state, payment rails, pricing, scheduling, trust scoring, dispute resolution, or backup execution.

## Data Model

The first implementation must define:

- `ledger_stream`: scope, stream id, sequence range, schema version, checkpoint policy, retention policy, and integrity refs.
- `ledger_entry`: stable id, stream id, sequence number, entry type, dimensions, amount/value refs, debit/credit-style account refs where needed, source evidence refs, policy refs, and audit refs.
- `entry_batch`: atomic group of ledger entries with batch id, idempotency key, expected stream sequence, source command ref, and batch validation result.
- `account_ref`: ORU account id, account owner ref, account type, tenant scope, and state snapshot ref supplied by ORU Account Service.
- `source_evidence_ref`: Overmeter rollup, Overguard decision, Overclaim finality, Overgrant authorization, Overbill document/payment ref, Provider Payout ref, operator action, or migration ref.
- `ledger_index`: account, tenant, workload, provider, dispute, grant, native service, system service, dimension, and time-window indexes.
- `checkpoint_record`: stream id, sequence range, hash/checksum, signer refs, created_at, and verification status.
- `reconciliation_report`: compared refs, expected totals, observed totals, mismatch reason codes, and correction refs.
- `audit_export`: bounded export metadata, redaction policy, checksum, signer refs, and consumer authorization refs.

Required entry types:

- `reservation`
- `settlement`
- `hold`
- `release`
- `refund`
- `correction`
- `provider_earning`
- `grant_allocation`
- `native_service_usage`
- `system_service_usage`
- `external_payment_credit_ref`
- `external_payment_debit_ref`
- `migration_import`
- `checkpoint`

Common envelope fields:

- `id`
- `stream_id`
- `sequence_number`
- `tenant_id` or `system_scope`
- `actor_id` or `service_account_id`
- `trace_id`
- `idempotency_key`
- `schema_version`
- `entry_type`
- `state_transition_ref`
- `resource_dimensions`
- `policy_refs`
- `source_evidence_refs`
- `audit_refs`
- `created_at`

## API Surface

Phase 5 should expose:

- `POST /ledger/append`: append a single validated entry.
- `POST /ledger/append-batch`: append an atomic batch with expected stream sequence and idempotency key.
- `GET /ledger/entries/{id}`: read one authorized entry with source refs and redaction.
- `GET /ledger/query`: bounded query by account, tenant, workload, provider, dispute, grant, native service, system service, dimension, and time window.
- `GET /ledger/streams/{id}/checkpoint`: read checkpoint and verification status.
- `POST /ledger/checkpoints`: create an integrity checkpoint for a stream range.
- `POST /ledger/reconcile`: create a reconciliation report for a bounded source/accounting period.
- `POST /ledger/corrections`: append correction entries referencing prior entries and finality refs.
- `GET /ledger/replay`: return ordered entries, checkpoints, and source refs for audit replay.
- `POST /ledger/audit-exports`: create an authorized bounded export with redaction and checksum refs.

API rules:

- Append operations require service-account authority and accepted producer type.
- Idempotency keys must prevent duplicate appends for the same source command.
- Expected sequence checks must prevent out-of-order and split-brain writes.
- Queries must be bounded and role-filtered.
- Corrections can only append new entries referencing prior entries; historical entries remain immutable.
- External payment refs are references only and must not trigger payment-provider calls from Seal Ledger.

## Event Surface

- `seal_ledger.entry_appended`
- `seal_ledger.batch_appended`
- `seal_ledger.append_denied`
- `seal_ledger.checkpoint_created`
- `seal_ledger.checkpoint_verified`
- `seal_ledger.reconciliation_created`
- `seal_ledger.reconciliation_mismatch`
- `seal_ledger.correction_appended`
- `seal_ledger.audit_export_created`

Events must include stream id, sequence range, entry ids, entry type, producer service account, source evidence refs, trace id, idempotency key, checkpoint refs, and Overwatch audit refs. Events must not leak private workload details when evidence refs are sufficient.

## Core Workflow

1. A producer service submits an append or append-batch command with source evidence refs.
2. Seal Ledger validates producer authority, account refs, dimensions, entry type, idempotency key, expected sequence, and source evidence.
3. The entry or batch is appended atomically to the appropriate stream.
4. Indexes update for bounded queries.
5. Checkpoints are created periodically or at configured sequence ranges.
6. ORU Account Service, Overbill, Provider Payout Service, Overgrant, Overclaim, and reporting tools derive views from entries.
7. Reconciliation compares expected source refs against ledger state.
8. Corrections append new entries when disputes, refunds, reversals, migration errors, or operator-signed corrections require repair.

## State Machine

Append command states:

1. `received`: command accepted at ingress.
2. `validated`: schema, producer authority, account refs, and evidence refs passed.
3. `sequence_reserved`: stream sequence range is reserved for append.
4. `appended`: entries are durably written.
5. `indexed`: query indexes reflect entries.
6. `checkpointed`: entry range is covered by an integrity checkpoint.
7. `rejected`: command failed validation and created denial evidence.
8. `corrected`: later correction entries reference one or more prior entries.

Ledger entry states:

- `appended`
- `checkpointed`
- `superseded_by_correction`
- `exported`

Entries are never edited or deleted as a normal accounting operation.

## Policy And Security

- Only authorized producer services may append specific entry types.
- Every append must carry source evidence refs and producer signing refs.
- Resource dimensions must be explicit and compatible with ORU Account Service definitions.
- External payment refs must be tokenized/bounded references owned by Overbill.
- Public-provider, grant, federation, native-service, and system-service entries must include the relevant policy/finality refs.
- Restore/migration imports must be marked as such and must not masquerade as original runtime entries.
- Break-glass or operator-signed correction actions require Overwatch evidence and extra audit export visibility.
- Hash/checkpoint integrity protects against tampering but must not be described as public blockchain consensus.

## Metering And Accounting

- Seal Ledger is the accounting source for ORU transitions and settlement history.
- ORU Account Service derives balances from ledger entries; it must never become an independent mutable balance source.
- Overbill derives receipts/invoices/payment refs from ledger entries; billing docs cannot rewrite ledger history.
- Provider Payout Service derives earning and hold views from ledger entries; payout results create new refs/corrections.
- Overgrant usage, native service usage, and system-service usage must remain queryable by explicit dimensions.
- Internal accounting must avoid per-operation external payment calls and support low-friction machine-to-machine usage settlement.

## Observability And Operations

Expose:

- Append success/denial counts by producer and entry type.
- Stream lag, checkpoint lag, and index lag.
- Idempotency duplicate counts.
- Sequence conflict counts.
- Reconciliation mismatch counts and unresolved mismatch age.
- Correction counts by reason code.
- Query latency by index type.
- Audit export count, size, and consumer.
- Backup/restore checkpoint verification status.

Operators need a replay view by entry, batch, account, workload, provider, dispute, grant, and checkpoint.

## Failure Modes And Recovery

- Duplicate append command: return original entry/batch refs via idempotency.
- Producer lacks entry-type authority: reject with `producer_not_authorized`.
- Account ref invalid/suspended: reject with `account_ref_invalid`.
- Source evidence missing: reject with `source_evidence_missing`.
- Expected sequence mismatch: reject or retry after reading stream head; do not overwrite.
- Partial batch failure: append nothing and preserve rejection evidence.
- Index update failure after durable append: mark index lag and rebuild from stream; do not reappend.
- Checkpoint mismatch: quarantine affected range, raise incident, and require restore/reconciliation process.
- Backup restore imports: append `migration_import` or restore metadata refs rather than editing original entries.
- Incorrect historical entry: append `correction` with finality refs.

## Validation Plan

Required tests:

- Ledger entries are append-only and cannot be updated in place.
- Idempotent append returns the same result for duplicate source commands.
- Batch append is atomic.
- Sequence conflicts are detected.
- Balance views from ORU Account Service reconcile with ledger entries.
- Disputes, refunds, and corrections create new entries instead of mutating history.
- Provider earning, grant allocation, native service usage, and system-service usage entries are queryable by refs and dimensions.
- Checkpoint verification detects tampered entries.
- Audit export preserves ordering, checksums, redaction, and source refs.
- External payment refs do not trigger external payment calls from Seal Ledger.

## Build Breakdown

1. Define ledger stream, entry, batch, account ref, source evidence ref, and checkpoint schemas.
2. Implement authorized append and append-batch APIs with idempotency and expected-sequence checks.
3. Implement required entry types for Phase 5: reservation, settlement, hold, release, refund, correction, provider earning, grant allocation, native service usage, and system-service usage.
4. Add bounded indexes by account, tenant, workload, provider, dispute, grant, native service, system service, dimension, and time.
5. Add checkpoint creation and verification.
6. Add replay and audit export APIs.
7. Add reconciliation jobs against Overmeter, ORU Account Service, Overbill, Overclaim, Overgrant, and Provider Payout refs.
8. Add correction-by-new-entry flow and operator-signed correction evidence.
9. Add backup/restore verification hooks for Phase 7.

## Handoff And Downstream Use

Seal Ledger feeds:

- ORU Account Service balance projection.
- Overbill receipts, invoices, statements, payment refs, and audit exports.
- Provider Payout earning and hold views.
- Overgrant grant usage reports.
- Overclaim dispute settlement/correction flows.
- Wallet and Usage Center user/provider views.
- Central AI stewardship and reporting services.
- Backup and Restore Service integrity drills.

Downstream services must store ledger entry refs rather than copying mutable accounting state.

## Open Design Questions

- Resolved: Phase 5 should use tenant-scoped accounting streams plus a small number of `system_scope` streams, not one global ledger stream and not account-per-stream fragmentation. Each stream has a single ordered sequence, expected-sequence append checks, BLAKE3 checkpoint ranges, and indexes by account, workload, provider, dispute, grant, native service, system service, resource dimension, and time. Cross-account transitions inside one tenant or system scope use `entry_batch` atomicity; cross-tenant/federation settlement remains out of Phase 5 unless an explicit bridge/import entry records the boundary. Phase 7 restore/failover should promote these streams as segmentable checkpoint ranges with stream epoch, segment id, previous checkpoint hash, writer key ref, and replay proof, allowing grid-resident replication and leader/failover choices without changing Phase 5 entry semantics.
- Resolved: v0 checkpoints are created and signed by the Seal Ledger stream writer key managed through Overkey, with signer metadata and verification refs recorded in the checkpoint record. Independent verifier signatures are optional for normal Phase 5 private accounting but required before backup/restore finality, audit-export finality, migration cutover, and any Phase 7 grid-resident failover promotion. Those verifier signatures come from separate Overwatch/Backup and Restore/system-verifier service accounts after replaying the checkpoint range; they are integrity countersignatures, not blockchain consensus or mining.
- Resolved: v0 requires paired debit/credit-style entries whenever an accounting transition moves value between account scopes, balance states, or accountable owners. This includes reservations into reserved/escrow state, settlements from payer or grant reserve into spent/earned/service accounts, releases back from reserved or held state, refunds, balance-affecting corrections, provider earnings, grant allocations, native-service usage, system-service usage, external payment credit/debit refs, and migration imports that establish opening balances. Pure integrity or metadata records such as checkpoints, audit-export markers, reconciliation reports, and zero-amount evidence markers do not need paired entries. Holds only require paired entries when they actually move an amount into held state; evidence-only hold markers may remain single marker entries.
- Resolved: Seal Ledger audit bundles should retain accounting truth indefinitely while exporting private workload evidence only through redacted refs. Ledger entries, sequence numbers, source rollup ids, ORU dimensions, schema/normalizer versions, hashes, checkpoint refs, correction/refund/hold refs, and replay metadata remain durable. Full private workload refs and payload-adjacent details follow the Overmeter Phase 5 default: at least 90 days after signed rollup and receipt creation, and longer when a dispute, correction, fraud/security review, provider payout hold, grant report, compliance export, or incident is active, until finality plus the configured audit margin. After that, export bundles keep redacted summaries and stable evidence refs, not raw prompts, outputs, logs, secrets, exact node addresses, provider-private topology, or private tenant data unless a declared redaction profile and consumer authorization allow deeper Overwatch/Overvault-backed evidence access.
- Resolved: Founder-hardware bootstrap moves into grid-resident stores through explicit `migration_import` batches rather than edits to historical entries. Each import batch records source stream id, source sequence/checkpoint range, export snapshot ref, cutoff time, source and destination stream epochs, BLAKE3/checksum refs, signer/verifier refs, import reason, and old-to-new account/stream mapping. Imported opening balances use paired entries against a `migration_control` or `opening_balance` account so replay stays balanced; original bootstrap entries remain referenced as source evidence and are marked read-only/superseded by cutover refs. The cutover flow is export, verify, shadow replay, append `migration_import`, countersign, then mark the founder-hardware stream closed/tombstoned for new writes while preserving it for audit replay.
