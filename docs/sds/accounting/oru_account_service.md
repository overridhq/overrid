SDS #38

# ORU Account Service SDS

## Purpose

Manage Overrid Resource Unit accounts as the internal non-speculative utility credit layer for resource usage, reservations, holds, refunds, corrections, grants, native service charges, provider earnings, and machine-to-machine settlement.

ORU Account Service is the account and balance projection service for ORU. It must derive balances and state from append-only Seal Ledger entries and signed usage/accounting refs rather than acting like a mutable token ledger, blockchain wallet, or speculative currency system.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [oru_account_service.md](../../service_catalog/accounting/oru_account_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) |

## Service Family

- Family: Accounting, credits, billing, and rights
- Owning layer: ORU account model, balance projection, account lifecycle, resource dimensions, transition refs, wallet views, and accounting-read APIs
- Primary data scope: ORU accounts, account owners, account scopes, resource dimensions, balance projections, transition refs, reservation refs, hold refs, grant refs, refund/correction refs, wallet display records, and audit refs
- First build phase from service plan: [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md).

## Problem Statement

Overrid needs a usable resource-accounting unit without blockchain overhead, NFT speculation, or per-operation external payment calls. Users, apps, providers, grants, native services, and system services need resource balances that are understandable, auditable, and tied to real CPU, GPU, storage, network, memory, data, and service usage.

The account service must expose wallet and admin views while preserving the accounting truth: transitions are backed by Seal Ledger entries, Overmeter rollups, Overclaim dispute refs, grant refs, or operator/system refs. Direct mutable balance counters would create drift, double-spend risk, and dispute problems.

## Goals

- Define account types for person, organization, app, native service, provider, grant pool, escrow/hold, reserve, and system service.
- Track explicit ORU dimensions: CPU-ORU, GPU-ORU, STOR-ORU, NET-ORU, MEM-ORU, DATA-ORU, and Service-ORU.
- Model account state, owner refs, tenant scope, compliance flags, suspension, delegation, and visibility rules.
- Derive available, reserved, held, spent, earned, sponsored, refunded, corrected, expired, and revoked balances from append-only refs.
- Prevent double spending between available, reserved, and held states.
- Expose wallet/admin balance views, statement views, and account history without exposing private ledger internals unnecessarily.
- Provide low-friction budget/preauthorization refs for HTTP 402-style machine-to-machine payments without per-call external payment friction.
- Keep ORU internal and non-speculative: no blockchain, no token market, no NFT mechanics, and no transaction toll model.

## Non-Goals

- Do not be Seal Ledger. Seal Ledger owns append-only accounting entries and settlement history.
- Do not price resources or define market rates. Overmark provides bounded resource-card and reference-rate signals where needed.
- Do not invoice, charge cards, handle taxes, or manage external payment rails. Overbill owns those flows.
- Do not issue provider payout batches. Provider Payout Service owns payout workflow.
- Do not grant sponsored allocation rules. Overgrant owns grant eligibility and purpose scope.
- Do not mutate balances without ledger-backed transition refs.
- Do not expose ORU as a speculative token, blockchain asset, or externally tradable currency.

## Primary Actors And Clients

- Wallet and Usage Center, displaying balances, reservations, holds, receipts, and statements.
- Overbill, creating receipts, invoices, external payment refs, refunds, and account statements.
- Seal Ledger, supplying authoritative append-only entries for balance projection.
- Overmeter, supplying signed usage rollups.
- Overgrant, allocating sponsored and purpose-scoped ORU.
- Provider Payout Service, reading provider earning, hold, and payout eligibility balances.
- Native services, apps, AI Gateway Router, and system services, reserving or checking budget refs before usage.
- Overclaim, creating dispute hold, correction, refund, and release refs.
- Overpass, Overtenant, Overkey, admin UI, CLI, SDK, and central AI stewardship, reading account scope and authorized views.

## Dependencies

- [Seal Ledger](seal_ledger.md) for reservation, settlement, hold, release, refund, correction, earning, grant allocation, native service usage, and system-service usage entries.
- [Overmeter](../execution_scheduling/overmeter.md) for signed usage rollups and dispute windows.
- [Overbill](overbill.md) for invoices, receipts, external payment refs, refunds, and account statements.
- [Overgrant](overgrant.md) for sponsored, grant-funded, and purpose-scoped allocation refs.
- [Overclaim](../trust_policy_verification/overclaim.md) for dispute, hold, correction, release, and finality refs.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), and [Overkey](../control_plane/overkey.md) for account owner identity, tenant scope, credentials, and delegated access.
- [Overwatch](../control_plane/overwatch.md) for audit events, account lifecycle evidence, and export refs.

## Owned Responsibilities

ORU Account Service owns:

- ORU account lifecycle and account metadata.
- Balance projection by resource dimension and state.
- Account visibility, delegation, and wallet/admin read models.
- Account state such as active, suspended, closed, or restricted.
- Preauthorization and budget check refs that do not mutate external payment rails.
- Statement-ready account views and transition summaries.
- Consistency checks between projected balances and Seal Ledger entries.

It must not create financial truth independent of Seal Ledger. Every balance-changing projection must cite authoritative refs.

## Data Model

The first implementation should define:

- `oru_account`: account id, account type, owner refs, tenant scope, compliance flags, suspension state, visibility class, allowed dimensions, and audit refs.
- `account_owner_ref`: person, organization, app, native service, provider, grant pool, escrow/hold, reserve, or system-service owner refs.
- `oru_dimension`: dimension id, unit name, resource class, display precision, allowed account types, and conversion/aggregation restrictions.
- `balance_projection`: account id, dimension, available, reserved, held, spent, earned, sponsored, refunded, corrected, expired, revoked, projection version, source ledger checkpoint, and computed-at timestamp.
- `transition_ref`: account id, ledger entry ref, transition type, dimension, amount, source service, source record ref, policy refs, dispute refs, and resulting projection refs.
- `reservation_ref`: account id, workload/service/app ref, resource dimensions, reservation window, expiry, release condition, and ledger refs.
- `hold_ref`: account id, hold type, claim refs, affected dimensions, expiry, release/correction refs, and downstream service refs.
- `grant_allocation_ref`: source grant account, beneficiary account, purpose scope, dimensions, quota, time window, abuse throttle refs, and Overgrant refs.
- `budget_precheck_ref`: account id, requested dimensions, available projection refs, policy decision refs, grant refs, result, expiry, and no-reservation attestation.
- `wallet_display_record`: account id, viewer scope, redacted balances, active holds, reservations, recent transitions, receipts refs, and statement refs.
- `account_statement_view`: account id, time window, opening projection, closing projection, transition summaries, receipt refs, dispute refs, export hash, and redaction profile.
- `projection_replay_bundle`: account id, source ledger checkpoint, ledger refs, projection algorithm version, balance projection result, and audit event refs.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account, `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

ORU Account Service exposes account and read-model APIs:

- `POST /oru/accounts`: create an ORU account for an eligible owner ref.
- `GET /oru/accounts/{account_id}`: read authorized account metadata and state.
- `GET /oru/accounts/{account_id}/balances`: read balance projections by dimension and state.
- `GET /oru/accounts/{account_id}/transitions`: read transition refs with redacted source details.
- `POST /oru/accounts/{account_id}/budget-prechecks`: check available budget or grant coverage without reserving funds.
- `GET /oru/accounts/{account_id}/wallet-view`: read wallet-ready balances, holds, reservations, and receipts.
- `GET /oru/accounts/{account_id}/statements`: read statement summaries and export refs.
- `POST /oru/accounts/{account_id}/recompute`: recompute balance projection from Seal Ledger checkpoint.
- `POST /oru/accounts/{account_id}/suspend`: suspend, restrict, reinstate, or close account with signed evidence.

API requirements:

- Mutating endpoints require actor identity, tenant context, trace id, idempotency key, and policy/authority refs.
- Balance-changing effects must come from Seal Ledger entries or accepted source refs; direct balance mutation is forbidden.
- Reads must enforce owner, tenant, role, and data-class filtering.
- Budget prechecks must be explicitly non-reserving unless a later ledger reservation entry is created by the owning flow.

## Event Surface

- `oru.account_created`: account created for owner ref.
- `oru.account_state_changed`: account active/suspended/restricted/closed state changed.
- `oru.balance_projected`: projection recomputed from ledger refs.
- `oru.transition_seen`: new ledger-backed transition affected projected balance.
- `oru.reservation_visible`: reservation ref appears in projection.
- `oru.hold_visible`: hold ref appears in projection.
- `oru.budget_prechecked`: budget/preauthorization check completed.
- `oru.statement_created`: statement/export view created.
- `oru.projection_mismatch`: consistency check found mismatch with ledger checkpoint.

Events must include account id, dimension refs, source refs, ledger checkpoint where applicable, and trace id.

## Core Workflow

1. Identity, tenant, app, provider, grant, reserve, or system-service owner requests an ORU account.
2. ORU Account Service validates owner refs and policy through Overpass, Overtenant, and Overguard/authority refs.
3. Seal Ledger entries record reservations, settlements, holds, releases, refunds, corrections, earnings, grants, and usage.
4. ORU Account Service projects balances from ledger checkpoints and transition refs.
5. Wallet, Overbill, Provider Payout Service, Overgrant, native services, and AI/service flows read projections and precheck refs.
6. Overclaim disputes create hold/correction/release refs that appear in projections.
7. Statements and wallet views present redacted, user-safe summaries.
8. Projection replay verifies balances from ledger history when disputes or audits occur.

## State Machine

Account lifecycle:

1. `requested`: account creation requested.
2. `active`: account can receive and spend allowed ORU dimensions.
3. `restricted`: account has limited operations due to policy, compliance, or dispute.
4. `suspended`: new spending/reservation is blocked.
5. `closing`: account is being drained, corrected, or migrated.
6. `closed`: account is no longer usable for new activity.
7. `revoked`: account was invalidated for severe policy or integrity reasons.

ORU transition states:

1. `available`: usable balance.
2. `reserved`: committed for a workload, app, or service operation.
3. `held`: blocked by dispute, payout hold, compliance, or review.
4. `spent`: consumed by settled usage.
5. `earned`: earned by provider or service.
6. `sponsored`: allocated by grant or stewardship source.
7. `refunded`: returned through refund entry.
8. `corrected`: adjusted by correction entry.
9. `expired`: no longer valid after expiry rule.
10. `revoked`: invalidated by policy or integrity action.

Transitions are append-only refs. Projection changes do not rewrite the source transition.

## Policy And Security

- Every account must have an owner ref, tenant scope, and visibility class.
- Suspended or restricted accounts cannot spend or reserve unless policy explicitly allows a correction/release path.
- Held and reserved ORU cannot be double-spent.
- Grant-funded ORU must preserve purpose scope and reporting refs.
- Provider earnings may be visible but not payable if payout holds or disputes apply.
- Wallet views must redact sensitive provider, dispute, or fraud details.
- Service-to-service prechecks must have expiry and must not act as unlimited spending authority.
- Manual account actions require signed operator/service action, evidence refs, and Overwatch audit.

## Metering And Accounting

ORU Account Service is accounting infrastructure, but it does not create usage truth by itself:

- Overmeter creates signed usage rollups.
- Seal Ledger creates append-only accounting entries.
- ORU Account Service projects balances from those entries.
- Overbill creates receipts, invoices, payment refs, refunds, and statements.
- Overgrant defines purpose-scoped allocations.
- Overclaim controls dispute holds and correction/refund proposals.

Machine-to-machine payment flows should use preauthorized budget, small usage holds, rollup settlement, and receipt delivery without calling external payment rails per tiny operation.

## Observability And Operations

- Dashboards should show account counts by type/state, projected balance totals by dimension, reservations, holds, mismatches, stale projections, budget precheck failures, and statement generation.
- Operators need account timelines that join ledger entries, usage rollups, disputes, grants, receipts, and payout refs.
- Alerts should fire on projection mismatches, negative available balances, double-spend attempts, stale ledger checkpoints, and suspended accounts with active reservations.
- Projection recompute must support scoped replay by account, tenant, dimension, and checkpoint.

## Failure Modes And Recovery

- Ledger unavailable: freeze projection updates and show last checkpoint with stale marker.
- Projection mismatch: mark account review-required, preserve old projection, and recompute from ledger refs.
- Duplicate transition refs: ignore duplicate by idempotency key and ledger entry id.
- Negative available projection: block spending, emit integrity event, and require correction review.
- Hold release denied: keep held state and attach downstream denial refs.
- Account owner deleted or merged: preserve tombstone/merge refs and migrate only through signed transition.
- Budget precheck expires: require a new precheck before admission or service use.

## Validation Plan

The service implementation plan lists these requirements:

- Balances derive from append-only ledger entries.
- Held and reserved credits cannot be double-spent.
- Refunds and corrections append new transitions.

Additional SDS-level validation:

- Contract tests for account create/read, balance read, transitions, budget precheck, wallet view, statements, recompute, and suspension APIs.
- Projection tests proving balances are derived from Seal Ledger fixtures.
- State tests for available, reserved, held, spent, earned, sponsored, refunded, corrected, expired, and revoked transitions.
- Double-spend tests covering simultaneous reservations and holds.
- Redaction tests for wallet/admin/provider/service views.
- Replay tests proving projection can be reconstructed from ledger checkpoints.
- Machine-to-machine precheck tests proving prechecks do not reserve unless a ledger entry exists.

## Build Breakdown

1. Define account, owner, dimension, balance projection, transition, reservation, hold, grant, precheck, wallet view, statement, and replay schemas.
2. Implement account lifecycle APIs.
3. Implement Seal Ledger-backed projection from fixture entries.
4. Add reservations, holds, refunds, corrections, earnings, grants, and expiry projection logic.
5. Add wallet/admin read models and statement views.
6. Add budget/preauthorization prechecks for service-to-service usage.
7. Add consistency checks, replay, and projection repair workflows.
8. Integrate Overbill, Overgrant, Overclaim, Provider Payout Service, native apps, and central AI stewardship reads.

## Handoff And Downstream Use

ORU Account Service powers Wallet and Usage Center, Overbill, Overgrant, Provider Payout Service, native app usage, AI Gateway Router budgeting, system-service spending, grant allocation, and HTTP 402-style service-to-service settlement.

## Open Design Questions

Resolved decisions:

- Wallet and admin views may aggregate ORU dimensions for readability, but accounting, admission, reservation, settlement, grant, hold, and dispute logic must remain explicit per dimension. User-facing summaries can group CPU-ORU and GPU-ORU under compute, STOR-ORU and DATA-ORU under storage/data, NET-ORU under network, and Service-ORU under app/service usage, but the detailed view must always expose available, reserved, held, spent, earned, sponsored, refunded, corrected, expired, and revoked state by original dimension. No service may silently convert one ORU dimension into another as fungible balance; any cross-dimension display, rate band, invoice line, or budget explanation must cite Overmark, Overbill, grant, or statement refs and stay presentation-only unless Seal Ledger records an explicit transition.
- Budget prechecks are short-lived, non-reserving facts. They remain valid only until the earliest of their explicit expiry, a newer balance projection checkpoint with affected transitions, account state change, delegation or policy change, grant/quota change, active hold/release/correction, or workload/app admission fact change. Default validity is classed by risk: wallet and developer previews may live up to 60 seconds; ordinary app/service admission prechecks up to 15 seconds; high-cost GPU, private-data, grant-funded, public-provider, or system-service prechecks up to 5 seconds or one admission attempt; batch/system flows must bind each item to its own expiry and idempotency key. A later Seal Ledger reservation, hold, or settlement entry supersedes the precheck rather than mutating it.
- Spending delegation is allowed only from active person, organization, app, native-service, and narrowly scoped system-service accounts when Overpass/Overtenant authority, Overkey signing refs, Overguard policy refs, quota/dimension limits, purpose scope, expiry, revocation path, and Overwatch audit refs are present. Delegated apps or agents receive capped budget authority by account, dimension, service/workload class, time window, and purpose; they cannot delegate onward unless the original delegation explicitly allows a narrower subdelegation. Grant-pool accounts delegate only grant authorization refs through Overgrant, not broad spend authority. Escrow, hold, reserve, suspended, closing, revoked, and provider payout accounts cannot delegate spend authority; their movement is controlled by the owning hold, reservation, payout, dispute, grant, or correction service.
- Wallet responsiveness should use event-driven projection updates plus bounded checkpoints rather than a slow full replay for every read. Active accounts with open wallet sessions, active reservations, holds, grants, or service-to-service spend should update their read projection from Seal Ledger append events within a few seconds and mark wallet views stale if the visible checkpoint is older than 30 seconds. Durable projection checkpoints should be created at least every 15 minutes for active accounts, every 1,000 relevant transitions, at statement/export boundaries, and immediately after high-impact transitions such as hold, release, refund, correction, grant allocation, account suspension, or closure movement. Idle accounts can checkpoint lazily on demand, but budget prechecks and statements must first refresh to a current ledger checkpoint.
- Account closure is a staged drain and tombstone workflow, not a delete. Entering `closing` immediately blocks new spending, new delegation, and new reservations while preserving statement/export access. Active reservations must either settle, expire, or release through Seal Ledger refs; active holds remain held until Overclaim, compliance, payout, or operator finality releases or corrects them; grant authorizations must be consumed, expired, revoked, or returned to the grant source through Overgrant refs; provider earnings and payout holds remain under Provider Payout Service and Overbill. The account can move to `closed` only after active reservations are zero, blocking holds are resolved or migrated to an escrow/hold account, grant refs are reconciled, statements are sealed, and replay/tombstone refs are written. Severe policy revocation may freeze use immediately, but historical ledger, wallet, dispute, and audit refs remain replayable.
