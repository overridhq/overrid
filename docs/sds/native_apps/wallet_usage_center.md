SDS #74

# Wallet and Usage Center SDS

## Purpose

Build the user-facing control panel for ORU balances, usage, grants, holds, refunds, receipts, app permissions, provider earnings, and service costs.

Wallet and Usage Center is the native account-visibility and permission-control app for Overrid. It owns wallet views, usage dashboards, statement/export requests, receipt collections, permission control records, notification prefs, dispute handoff refs, and privacy audit views. It does not own ORU balance truth, Seal Ledger entries, payment settlement, grants, billing, payouts, or resource-rate policy. Its job is to make resource usage and app permissions understandable and controllable without becoming a mutable accounting ledger.

The wallet must make the ORU-first economy visible: users can buy ORU, earn ORU from approved resource contribution or legitimate services, spend ORU on native services and third-party apps, and view provider earnings. Cash-out is shown as a separate provider-eligibility flow, not as a direct withdrawal of bought ORU.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [wallet_usage_center.md](../../service_catalog/native_apps/wallet_usage_center.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md) |
| Sub-build plan | [SUB BUILD PLAN #74 - Wallet and Usage Center](../../build_plan/sub_build_plan_074_wallet_usage_center.md) |

## Service Family

- Family: Native applications
- Owning layer: Native public utility application layer for user-visible ORU, usage, receipts, statements, permissions, privacy controls, and dispute handoffs
- Primary data scope: wallet view records, account display prefs, usage summaries, receipt collections, statement/export jobs, permission control records, revocation requests, privacy audit refs, dispute handoff refs, notification prefs, usage refs, and audit refs
- First build phase from service plan: [Phase 12: Native Application Layer](../../build_plan/phase_12_native_application_layer.md)

## Problem Statement

Overrid replaces speculative tokens, per-transaction fee friction, and opaque platform bills with ORU credits, Seal Ledger accounting, usage rollups, grants, holds, refunds, receipts, and near-cost native services. Users need a first native app that lets them see what is happening: balances by ORU dimension, resource usage by app/service, active reservations and holds, grants, receipts, statements, app permissions, privacy access, and disputes.

The design risk is turning the wallet UI into an accounting authority. Wallet and Usage Center must be a user-facing projection and control surface. It reads ORU Account Service, Seal Ledger, Overbill, Overgrant, Overmeter, Overclaim, and native app permission refs; it does not mutate balances directly or create ledger truth.

## Goals

- Define wallet view, account selector, usage dashboard, receipt collection, statement/export, permission control, revocation, privacy audit, notification, dispute handoff, and usage records.
- Display ORU balances, reservations, holds, grants, sponsored credits, refunds, corrections, receipts, statements, and account history through authoritative refs.
- Show bought, earned, sponsored, held, spendable, and payout-eligible states clearly so users understand what can be spent inside Overrid and what can enter provider payout review.
- Show resource usage by actor, organization, app, service, native app, model route, storage, compute, network, data, and time window.
- Let users review, narrow, revoke, and audit app permissions for native services, AI tools, search, messaging, storage, location, and workspace access.
- Provide export and statement jobs without leaking sensitive ledger, fraud, provider, or private-app details.
- Provide dispute/claim handoffs to Overclaim, Overbill, Overgrant, or source services.
- Make first Phase 12 native app useful because every other native app depends on usage, permissions, balances, and receipts.

## Non-Goals

- Do not be ORU Account Service, Seal Ledger, Overbill, Overgrant, Overmeter, Overmark, Provider Payout Service, Overclaim, or an external payment processor.
- Do not mutate balances, issue ledger entries, price resources, settle payments, grant credits, release holds, create refunds, calculate taxes, or execute payouts directly.
- Do not expose ORU as a speculative token, blockchain asset, NFT, tradable currency, or per-transaction toll model.
- Do not store card/payment secrets, vault secrets, private ledger internals, provider-sensitive payout details, fraud internals, or unrelated user profiling data.
- Do not override app-specific permissions or source-service data access policies; it can request revocations through owning services.
- Do not add pricing, customer-count, revenue projections, hardcoded charges, or marketplace assumptions.

## Primary Actors And Clients

- Users reviewing balances, usage, receipts, statements, permissions, privacy audit, grants, holds, refunds, and disputes.
- Organizations and admins reviewing organization accounts, app permissions, service usage, statements, and delegated access.
- Native apps sending usage/permission/receipt refs and receiving permission revocation or audit handoffs.
- Personal AI Assistant helping explain usage, summarize receipts, draft disputes, or suggest permission cleanup with user permission.
- ORU Account Service, Seal Ledger, Overbill, Overgrant, Overmeter, Overclaim, Overpass, Overtenant, Overkey, Overguard, Overwatch, Overmark, Provider Payout Service, Central AI Stewardship Interface, Mobile SDK, and Mobile Backend Gateway.

## Dependencies

- [ORU Account Service](../accounting/oru_account_service.md) for account metadata, balance projections, reservations, holds, grants, sponsored credits, and budget/precheck refs.
- [Seal Ledger](../accounting/seal_ledger.md) for append-only accounting entries and authoritative ledger refs.
- [Overbill](../accounting/overbill.md) for receipts, invoices, statements, refunds, external payment refs, and account export refs.
- [Overgrant](../accounting/overgrant.md) for sponsored credit, grant, purpose-scope, donation, and stewardship allocation refs.
- [Overmeter](../execution_scheduling/overmeter.md) for signed usage rollups by resource dimension, app, service, workload, model route, and time window.
- [Overclaim](../trust_policy_verification/overclaim.md) for disputes, corrections, holds, releases, refund proposals, and claim state.
- [Overmark](../accounting/overmark.md) for resource-card/reference-rate visibility where needed, without hardcoding pricing into the wallet app.
- [Overpass](../control_plane/overpass.md), [Overtenant](../control_plane/overtenant.md), [Overkey](../control_plane/overkey.md), [Overguard](../trust_policy_verification/overguard.md), and [Overwatch](../control_plane/overwatch.md) for identity, tenant/org roles, credentials, permission checks, policy, and audit.
- Native apps and AI/RAG/model-routing services for app permission records, usage refs, receipts, and privacy audit refs.

## Owned Responsibilities

Wallet and Usage Center owns:

- User and organization wallet view preferences, account selectors, visibility filters, and notification prefs.
- Balance view records derived from ORU Account Service projections and Seal Ledger checkpoints.
- Usage dashboard records derived from Overmeter rollups and app/native-service refs.
- Receipt collection views derived from Overbill and source-service receipt refs.
- Statement and export request records, export manifests, redaction profiles, and delivery refs.
- App permission control records, revocation/narrowing requests, privacy audit views, and permission history refs.
- Dispute handoff records that link usage, receipt, grant, hold, refund, statement, or app-permission issues to Overclaim/source services.
- User-visible explanations, replay refs, and audit summaries for wallet/usage decisions.
- Usage refs for wallet reads, exports, permission changes, dispute handoffs, and statement generation.

Wallet and Usage Center does not own account projections, ledger entries, billing settlement, external payment rails, grant eligibility, payout eligibility, usage measurement truth, final dispute decisions, or resource-rate policy.

## Data Model

- `wallet_profile`: owner actor/org refs, default account refs, display prefs, alert prefs, privacy/audit prefs, export prefs, and state.
- `wallet_account_selector`: viewer refs, account refs, role/visibility class, allowed dimensions, redaction class, and audit refs.
- `wallet_balance_view`: account refs, dimension summaries, available/reserved/held/spent/earned/sponsored/refunded/corrected projections, source checkpoint refs, stale markers, and display refs.
- `usage_dashboard`: viewer refs, time window, resource dimensions, app/service/native app/model route refs, rollup refs, filters, aggregation level, and state.
- `usage_line_item`: source service refs, operation class, resource dimensions, quantity refs, receipt refs, dispute refs, redaction class, and usage refs.
- `receipt_collection`: viewer/account refs, receipt refs, service/app refs, time window, status, refund/correction refs, claim refs, and export refs.
- `statement_export_job`: account refs, time window, included dimensions, redaction profile, requested format, Overbill/export refs, delivery refs, expiry, and state.
- `app_permission_control`: app/service refs, actor/org refs, permission class, scope, purpose, expiry, revocation state, owning service refs, and audit refs.
- `permission_revocation_request`: permission refs, requested action, owning service, policy refs, result refs, failure reason, and state.
- `privacy_audit_view`: viewer refs, app/service refs, context/data access refs, assistant/tool refs, permission refs, audit event refs, redaction class, and time window.
- `wallet_dispute_handoff`: target refs, dispute type, source service refs, receipt/usage/grant/hold/refund/permission refs, evidence refs, Overclaim refs, and state.
- `wallet_notification_pref`: actor/org refs, alert type, threshold refs, delivery route refs, quiet hours, and state.
- `wallet_usage_ref`: wallet-read/export/statement/permission/dispute/privacy-audit usage, Overmeter refs, and receipt refs.

Common envelope fields: `id`, `tenant_id`, `actor_id`, `organization_id`, `app_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

- `GET /wallet/accounts`: lists authorized ORU accounts and display scopes for the viewer.
- `GET /wallet/accounts/{account_id}/balances`: returns wallet-safe balance projections by ORU dimension and state.
- `GET /wallet/accounts/{account_id}/usage`: returns usage dashboard data by app, service, native app, resource dimension, and time window.
- `GET /wallet/accounts/{account_id}/receipts`: returns receipt collection entries with dispute/refund/correction refs.
- `GET /wallet/accounts/{account_id}/holds`: returns active reservations, holds, grants, sponsored credits, and release/correction refs.
- `POST /wallet/accounts/{account_id}/statements`: creates a statement/export request.
- `GET /wallet/statements/{statement_id}`: returns statement/export job state and delivery refs.
- `GET /wallet/permissions`: lists app/service/tool/native-app permissions visible to the user/org.
- `POST /wallet/permissions/{permission_id}/narrow`: requests a narrower scope from the owning service.
- `POST /wallet/permissions/{permission_id}/revoke`: requests permission revocation from the owning service.
- `GET /wallet/privacy-audit`: returns privacy/audit views by app/service/tool/time window.
- `POST /wallet/disputes`: opens a dispute handoff for usage, receipt, hold, grant, refund, statement, or permission issue.
- `PATCH /wallet/profile`: updates display, notification, privacy, and export preferences.
- `GET /wallet/replay/{record_id}`: reconstructs wallet view, usage view, statement/export, permission, revocation, dispute, and audit decisions.

Mutating APIs require signed actor identity, tenant/org role refs, trace id, idempotency key, account/view authorization, policy refs, and source-service refs. Stable errors include `account_not_visible`, `balance_projection_stale`, `usage_rollup_unavailable`, `receipt_not_visible`, `statement_scope_denied`, `permission_owner_required`, `permission_revocation_denied`, `privacy_audit_denied`, `dispute_target_invalid`, `source_service_required`, and `wallet_state_conflict`.

## Event Surface

- `wallet_usage_center.wallet_profile_updated`: profile/display/notification prefs changed.
- `wallet_usage_center.balance_view_requested`: balance view requested from ORU projections.
- `wallet_usage_center.usage_view_requested`: usage dashboard requested from Overmeter rollups.
- `wallet_usage_center.receipts_view_requested`: receipt collection view requested.
- `wallet_usage_center.statement_requested`: statement/export job requested.
- `wallet_usage_center.statement_ready`: statement/export job completed.
- `wallet_usage_center.permission_listed`: permission view rendered for viewer scope.
- `wallet_usage_center.permission_narrow_requested`: permission narrowing requested from owner service.
- `wallet_usage_center.permission_revoke_requested`: permission revocation requested from owner service.
- `wallet_usage_center.privacy_audit_viewed`: privacy audit view requested.
- `wallet_usage_center.dispute_handoff_created`: dispute handoff created.
- `wallet_usage_center.usage_emitted`: wallet usage refs emitted.

Events include viewer/account refs, source service refs, account projection refs, receipt refs, statement refs, permission refs, dispute refs, redaction class, reason codes, audit refs, and usage refs. Events must not include card/payment secrets, raw ledger internals beyond viewer authorization, vault secrets, private app data, fraud internals, or provider-sensitive payout internals.

## Core Workflow

1. User opens Wallet and selects a personal or organization account visible through Overpass/Overtenant roles.
2. Wallet requests account projections from ORU Account Service and receipt/statement refs from Overbill, using Seal Ledger checkpoints as provenance.
3. Wallet requests usage rollups from Overmeter and groups them by app, native service, resource dimension, time window, and operation class.
4. Wallet lists app/service permissions and privacy audit refs from owning services and Overwatch.
5. User narrows or revokes a permission. Wallet creates a request to the owning service and records result/failure refs.
6. User requests statement/export. Wallet creates a statement/export job with redaction and delivery refs, while Overbill/source services own authoritative statement records.
7. User opens a dispute for a usage, receipt, hold, grant, refund, permission, or statement issue. Wallet creates an Overclaim/source-service handoff.
8. Usage and audit records for the wallet app itself flow to Overmeter, Wallet receipt refs, and Overwatch.

## State Machine

Wallet profile lifecycle:

1. `created`
2. `active`
3. `restricted`
4. `archived`
5. `deleted`

Balance/usage view lifecycle:

1. `requested`
2. `source_refs_loaded`
3. `redacted`
4. `ready`
5. `stale`
6. `failed`
7. `expired`

Statement/export lifecycle:

1. `requested`
2. `scope_checked`
3. `building`
4. `ready`
5. `delivered`
6. `expired`
7. `failed`
8. `cancelled`

Permission control lifecycle:

1. `listed`
2. `active`
3. `narrow_requested`
4. `revoke_requested`
5. `owner_service_pending`
6. `narrowed`
7. `revoked`
8. `denied`
9. `failed`

Dispute handoff lifecycle:

1. `draft`
2. `submitted`
3. `source_validated`
4. `claim_opened`
5. `awaiting_source`
6. `resolved`
7. `rejected`
8. `closed`

State transitions are append-only. Wallet view records can expire or be recomputed; source accounting refs remain authoritative.

## Policy And Security

- Wallet reads are deny-by-default unless the viewer has owner, delegated, organization, app, or service visibility refs.
- Balance projections must cite ORU Account Service and Seal Ledger checkpoint refs; Wallet cannot create balance-changing state.
- Usage views must cite Overmeter rollups and source-service refs; Wallet cannot invent usage truth.
- Permission revocation/narrowing requests must route to the owning service; Wallet cannot directly edit another service's private grants unless that service exposes a revocation contract.
- Statements and exports require redaction profiles for provider-sensitive, fraud-sensitive, private-app, and organization-only details.
- Privacy audit views should show app/service/tool access in user-understandable form without exposing unrelated users or fraud internals.
- Dispute handoffs preserve evidence refs and never silently delete ledger, usage, or source records.
- Wallet must avoid speculative token framing, blockchain/NFT mechanics, hidden fees, and per-transaction external payment friction.

## Metering And Accounting

- Emit usage refs for balance reads, usage dashboard reads, receipt reads, statement/export jobs, permission list/narrow/revoke actions, privacy audit views, dispute handoffs, notification delivery, replay, compute, storage, and bandwidth.
- Link usage to viewer, account, organization, source service, resource dimension, time window, permission refs, statement refs, dispute refs, Overmeter refs, and wallet receipt refs.
- Wallet and Usage Center does not create account balances, ledger entries, charges, grants, refunds, holds, payouts, invoices, or resource rates.
- ORU/Seal Ledger/Overbill/Overgrant/Overmeter remain authoritative sources for accounting truth.
- Do not encode hardcoded prices, revenue projections, paid visibility, or per-transaction fees.

## Observability And Operations

- Expose balance view latency, projection staleness, usage-rollup latency, receipt lookup failures, statement/export duration, permission revocation success/failure, privacy audit read volume, dispute handoff volume, source-service failures, and usage emission status.
- Alert on stale balance projections, missing ledger checkpoints, unexpected negative/held projection display, permission revocation failures, export failures, privacy audit access denials, dispute handoff failures, and missing usage refs.
- Provide user-visible explanation for each balance, hold, grant, refund, correction, receipt, permission, and dispute status.
- Provide operator diagnostics using source refs, reason codes, and redacted summaries without exposing private payment or fraud internals.
- Provide replay for wallet display, source refs, permissions, statements, exports, disputes, and wallet usage.

## Failure Modes And Recovery

- ORU projection unavailable: show last checkpoint with stale marker and retry source fetch.
- Seal Ledger checkpoint mismatch: hide affected projection details, show review-required state, and route to ORU Account Service reconciliation.
- Overmeter rollup missing: show partial usage with source unavailable reason and retry refs.
- Receipt not visible or unavailable: show usage line with receipt pending and Overbill/source refs.
- Permission owning service unavailable: queue revocation/narrow request and show pending state.
- Revocation denied: show owner-service reason code and corrective path.
- Statement/export fails: preserve job state, reason refs, and retry/cancel options.
- Dispute target invalid: keep draft and show required source refs.
- Usage emission fails: mark wallet operation usage pending and reconcile before final receipt visibility.

## Validation Plan

- Users and authorized org roles can view ORU balances, holds, grants, refunds, corrections, usage, receipts, and statements through authoritative refs.
- Wallet cannot mutate balances, ledger entries, grants, refunds, holds, payouts, invoices, or resource rates.
- Usage dashboards cite Overmeter rollups and source-service refs.
- Permission listing, narrowing, revocation, and privacy audit flows route to owning services and preserve audit refs.
- Statement/export jobs apply redaction and do not leak private ledger/provider/fraud/app internals.
- Dispute handoffs create Overclaim/source-service refs for usage, receipt, hold, grant, refund, statement, and permission issues.
- The app avoids speculative token/blockchain/NFT/per-transaction-fee framing.
- Usage refs for Wallet operations flow to Overmeter and receipts.
- Replay reconstructs balance view, usage view, receipts, statement/export, permission, privacy audit, dispute, and usage decisions.

## Build Breakdown

1. Define wallet profile, account selector, balance view, usage dashboard, usage line item, receipt collection, statement/export, permission control, revocation request, privacy audit, dispute handoff, notification pref, and usage schemas.
2. Implement account list, balance, usage, receipts, holds/grants/refunds, statements, permissions, privacy audit, dispute, profile, and replay APIs.
3. Integrate ORU Account Service, Seal Ledger refs, Overbill receipts/statements, Overgrant grants, Overmeter rollups, Overclaim disputes, and Overwatch audit.
4. Add native-app permission inventory and owner-service revocation/narrowing contracts.
5. Add statement/export redaction profiles and delivery refs.
6. Add user/org UI flows for balance visibility, usage inspection, receipts, permissions, privacy audit, disputes, and notifications.
7. Add Personal AI Assistant explanatory tool permissions for wallet summaries and dispute drafts.
8. Add mobile-friendly APIs, offline cached read models where safe, usage metering, and operational diagnostics.

## Handoff And Downstream Use

- Every native app links usage, receipts, permissions, and privacy audit refs to Wallet and Usage Center.
- Personal AI Assistant uses Wallet tools to explain usage and permissions only with user permission.
- ORU Account Service, Seal Ledger, Overbill, Overgrant, Overmeter, and Overclaim remain authoritative for accounting/dispute flows.
- Central AI Stewardship Interface reads aggregate stewardship and surplus/donation refs through accounting/governance services, not wallet-private records.
- Mobile SDK and Mobile Backend Gateway expose wallet balance, usage, receipt, permission, and privacy controls to mobile apps.
- Native app onboarding uses Wallet permission surfaces so users can review and revoke resource/data access.

## Open Design Questions

Resolved decisions:

- Mobile offline mode must provide only read-only cached wallet views: account selectors, last successful balance projections, hold/reservation/grant/refund/correction summaries, recent receipt and statement refs, permission inventory, notification prefs, and privacy-audit summaries that were already authorized before disconnect. Active wallet sessions mark balance and usage projections stale when their source ORU/Seal Ledger checkpoint is older than 30 seconds; cached personal summaries may remain visible for up to 24 hours as offline snapshots, while immutable receipt/statement refs may remain visible until their export/cache expiry. Budget prechecks, spend decisions, statement issuance, dispute submission, permission expansion, and any accounting-changing action require live revalidation through Mobile Backend Gateway, Overgate, ORU Account Service, Seal Ledger, Overbill, Overguard, and the owning service before acceptance.
- Redaction is audience-classed and source-service enforced. Individual users see their own account projections, usage, receipts, grants, holds, statements, permissions, privacy-audit summaries, and dispute refs without provider-private, fraud, organization-only, or unrelated-user details. Organization administrators see organization-wide balances, service/app usage, holds, grants, statements, delegated access, and audit summaries, but user-private content, exact private-workload payloads, fraud internals, provider-sensitive payout details, and secret-bearing refs remain redacted unless a narrower policy grants them. Delegated accountants see accounting documents, statements, receipts, refunds, tax/compliance refs, and reconciliation summaries for assigned accounts, but cannot see app-private payloads, raw privacy-audit content, or permission-control authority outside the delegation. App owners see their own app's usage, receipts, permission state, quota/budget refs, and aggregate user/service summaries, but not cross-app balances, payment-provider refs, unrelated personal account details, private ledger internals, or organization/provider-sensitive data.
- Immediate revocation is required for permissions that can create new private-data access, spend authority, credential/session/device authority, secret or Overvault access, AI/RAG context access, location/contact/message/workspace/private-media access, payment or grant delegation, push delivery of sensitive content, child/safety-sensitive access, or policy-blocked/compromised access. Wallet records the revocation request and the owning service must deny new privileged use immediately, even if later cleanup of caches, indexes, notifications, replicas, or audit exports is asynchronous. Queued owner-service revocation is allowed only for non-sensitive, already-public, idempotent, or cleanup-oriented permissions such as notification preferences, public listing visibility, public search/index hints, low-risk personalization, or historical export delivery; queued requests must carry expiry, idempotency, owner-service refs, user-visible pending state, and fail-closed behavior if the permission becomes high risk.
- Long-running disputes appear as separate `disputed`, `hold_pending`, `held`, `awaiting_source`, `under_review`, `appeal_window`, or `finality_pending` overlays on balance and usage summaries. Wallet must keep original usage, receipt, grant, hold, and ledger refs visible according to redaction rules, show affected amounts/dimensions as contested or held, and link to Overclaim, Overbill, ORU Account Service, Seal Ledger, and source-service refs. It must not apply refunds, corrections, releases, or final balance changes until the owning accounting service returns append-only Seal Ledger/Overbill/ORU refs. Summary totals should distinguish current available balance from contested/held/provisional amounts and should show claim deadlines or finality markers without implying that Overclaim or Wallet has rewritten accounting truth.
