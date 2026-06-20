SDS #39

# Overasset SDS

## Purpose

Represent operational ownership, entitlements, and resource-right references for Overrid resources, storage leases, capacity claims, grant rights, namespace-linked rights, app/service ownership refs, dataset/model/media/package rights metadata, and transferable utility rights where legally enabled.

Overasset is not an NFT system. It is a utility rights and ownership-reference layer inside Overrid. Its records must be evidence-backed, policy-bound, dispute-aware, and useful for operations rather than speculation.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overasset.md](../../service_catalog/accounting/overasset.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md), [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md) |
| SDS sub-build plan | [SUB BUILD PLAN #39 - Overasset](../../build_plan/sub_build_plan_039_overasset.md) |

## Service Family

- Family: Accounting, credits, billing, and rights
- Owning layer: Operational ownership records, resource-right refs, storage entitlement refs, namespace bindings, transfer/delegation audit, and dispute/correction links
- Primary data scope: resource-right records, entitlement records, ownership evidence refs, delegation records, transfer records, revocation records, namespace/storage bindings, grant-right refs, ledger refs, registry refs, dispute refs, and audit exports
- First build phase from service plan: [Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) for first evidence-backed accounting and operational right records; [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md) expands storage entitlement and namespace-bound right refs.

## Problem Statement

Overrid needs a way to represent who controls an operational resource right without importing the broken mechanics of NFTs or speculative asset markets. A user, app, organization, provider, grant pool, native service, or system service may need records that show rights to storage, namespace bindings, capacity reservations, model/dataset/media/package metadata, grant eligibility, service ownership, or revocable delegation.

Those records must be useful for policy and operations: they should cite ledger, registry, grant, namespace, storage, and dispute evidence. They must not imply broad ownership beyond the right's scope, allow unauthorized transfer, bypass tenant policy, or create fake scarcity for speculation.

## Goals

- Define operational right records for resource rights, storage entitlements, namespace-linked rights, app/service ownership refs, dataset/model/media/package rights metadata, capacity claims, grant rights, and transfer/delegation refs.
- Link every right to source evidence from Seal Ledger, Overregistry, Overgrant, Universal Namespace Service, Overstore, Overbase, Overvault, Overclaim, or signed operator/service refs.
- Support delegation, transfer, revocation, expiry, correction, and dispute states where policy and law allow them.
- Keep rights scoped by tenant, owner, purpose, resource dimension, workload/app/service, jurisdiction/compliance flags, and allowed use.
- Provide policy-readable refs to Overguard, Universal Namespace Service, Overstore, Overvault, Overgrant, Overbill, and native apps.
- Preserve append-only history and evidence-backed explanations.
- Explicitly reject NFT-style speculation, royalty mechanics, artificial scarcity markets, and blockchain dependency.

## Non-Goals

- Do not implement NFTs, collectible tokens, speculative trading, on-chain ownership, royalties, or market-price discovery.
- Do not define legal title beyond what the right record and evidence refs actually support.
- Do not bypass Overguard policy, Overclaim disputes, tenant restrictions, compliance boundaries, or storage/namespace access rules.
- Do not mutate ORU balances or Seal Ledger entries. Accounting services own financial state.
- Do not replace Universal Namespace Service ownership, Overpass identity, or storage access controls.
- Do not make a right transferable unless policy, evidence, and jurisdiction flags allow it.
- Do not store private content; reference storage/vault/object records through authorized refs.

## Primary Actors And Clients

- Universal Namespace Service, binding names/routes/assets to ownership and delegation refs.
- Overstore, Overbase, and Overvault, consuming entitlement refs for storage, object, state, and sensitive data access.
- Overgrant, creating sponsored or purpose-scoped rights.
- Seal Ledger and ORU Account Service, providing accounting evidence and resource dimension refs.
- Overregistry, supplying app/service/resource/package/provider records.
- Overclaim, attaching disputes, corrections, revocations, and finality refs.
- Overguard, checking whether a right can be exercised, transferred, delegated, or revoked.
- Wallet and Usage Center, admin UI, SDK, CLI, native apps, and central AI stewardship, reading authorized ownership/entitlement summaries.

## Dependencies

- [Seal Ledger](seal_ledger.md) and [ORU Account Service](oru_account_service.md) for accounting evidence, resource dimensions, reservations, grants, holds, and correction refs.
- [Overgrant](overgrant.md) for sponsored, purpose-scoped, and grant-funded right refs.
- [Overclaim](../trust_policy_verification/overclaim.md) for disputes, corrections, holds, appeals, and finality refs.
- [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) for namespace ownership, route binding, transfer, delegation, tombstone, and dispute refs.
- [Overstore](../data_storage_namespace/overstore.md), [Overbase](../data_storage_namespace/overbase.md), and [Overvault](../data_storage_namespace/overvault.md) for storage, structured-state, object, sensitive-state, and secret entitlement refs.
- [Overregistry](../control_plane/overregistry.md) for app, service, resource, package, dataset/model/media, provider, and schema refs.
- [Overguard](../trust_policy_verification/overguard.md) for right exercise, transfer, delegation, revocation, and access policy decisions.
- [Overwatch](../control_plane/overwatch.md) for audit events, evidence bundles, and replay refs.

## Owned Responsibilities

Overasset owns:

- Operational right record schema and lifecycle.
- Ownership evidence refs and scoped entitlement refs.
- Delegation, transfer, revocation, expiry, correction, and dispute state.
- Namespace/storage/service/resource binding refs where the right itself must be cited.
- Right explanation, audit, and replay bundles.
- Policy-consumable right refs for access and operational flows.

Overasset must not enforce storage access directly. It supplies entitlement/right facts to policy and owning services.

## Data Model

The first implementation should define:

- `resource_right_record`: right id, right type, owner refs, tenant scope, source evidence refs, allowed use, resource dimensions, validity window, transferability, delegation policy, and current state.
- `ownership_evidence_ref`: source service, source record ref, evidence kind, integrity hash, legal/compliance flag, visibility class, and dispute refs.
- `storage_entitlement_ref`: storage service, object/collection/vault ref, allowed operations, quota/dimensions, retention policy, expiry, and revocation refs.
- `namespace_right_binding`: namespace id, name/route/asset ref, owner refs, delegation refs, transfer refs, tombstone refs, dispute refs, and namespace service refs.
- `capacity_claim_right`: provider/node/resource refs, capacity class, ORU dimension, reservation refs, validity window, Overmark refs, and challenge/trust refs.
- `grant_right_ref`: grant id, source account, beneficiary refs, purpose scope, resource dimensions, reporting requirement, expiry, and abuse throttle refs.
- `service_ownership_ref`: app/native service/system service ref, owner/delegate refs, operational permissions, transfer policy, and registry refs.
- `delegation_record`: right id, delegator, delegatee, scope, allowed operations, expiry, revocation conditions, and audit refs.
- `transfer_record`: right id, from owner, to owner, policy decision refs, evidence refs, legal/compliance refs, dispute hold refs, finality refs, and state.
- `revocation_record`: right id, revocation reason, source service, evidence refs, effective time, appeal refs, and downstream effect refs.
- `asset_dispute_link`: right id, claim id, disputed scope, hold effect, correction refs, finality refs, and visibility class.
- `asset_replay_bundle`: right refs, evidence refs, policy version, state transition refs, downstream service refs, and audit events.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account, `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

Overasset APIs are internal and platform-facing:

- `POST /assets/rights`: create a resource right from evidence refs.
- `GET /assets/rights/{right_id}`: read authorized right summary, owner refs, scope, state, and evidence refs.
- `GET /assets/rights/{right_id}/explain`: read reason codes, source evidence, transfer/delegation/dispute state, and redacted history.
- `POST /assets/rights/{right_id}/delegations`: create or revoke scoped delegation.
- `POST /assets/rights/{right_id}/transfers`: request, approve, deny, or finalize transfer where allowed.
- `POST /assets/rights/{right_id}/revocations`: revoke or expire a right with evidence refs.
- `POST /assets/rights/{right_id}/disputes`: attach Overclaim dispute refs.
- `POST /assets/rights/{right_id}/replay`: reconstruct right state from evidence and transition refs.
- `GET /assets/by-namespace/{namespace_id}`: read authorized namespace-bound rights.
- `GET /assets/by-owner/{owner_id}`: read owner-scoped operational rights.

API requirements:

- Mutating endpoints require actor/service identity, tenant context, idempotency key, trace id, and Overguard policy refs.
- Transfer and delegation endpoints must check dispute, compliance, legal, tenant, and purpose-scope restrictions.
- Reads must redact private storage/vault refs and provider-private evidence.
- Replay must use stored refs and policy versions.

## Event Surface

- `overasset.right_created`: operational right created.
- `overasset.right_updated`: scoped metadata or state changed.
- `overasset.delegation_created`: delegation created.
- `overasset.delegation_revoked`: delegation revoked or expired.
- `overasset.transfer_requested`: transfer requested.
- `overasset.transfer_finalized`: transfer finalized with policy/evidence refs.
- `overasset.transfer_denied`: transfer denied with reason codes.
- `overasset.revoked`: right revoked or expired.
- `overasset.dispute_attached`: Overclaim dispute linked.
- `overasset.correction_applied`: correction record created.
- `overasset.replay_completed`: replay completed for audit.

Events must include right id, owner refs, scope, source evidence refs, policy refs, and trace id.

## Core Workflow

1. A source service creates evidence for an operational right, such as a storage entitlement, namespace binding, grant allocation, capacity claim, or service ownership ref.
2. Overasset validates source refs, owner refs, tenant scope, transferability, compliance flags, and policy through Overguard.
3. The service creates a resource right record with scoped allowed use and evidence refs.
4. Owning services consume the right ref for policy/access decisions; they do not infer broad rights from owner identity alone.
5. Delegation, transfer, expiry, revocation, or dispute creates append-only transition records.
6. Overclaim attaches disputes and corrections without deleting right history.
7. Replay reconstructs the current right state from source evidence and transition refs.

## State Machine

Right lifecycle:

1. `draft`: right request is being assembled.
2. `pending_policy`: policy and evidence checks are running.
3. `active`: right can be referenced by owning services.
4. `delegated`: scoped delegation exists.
5. `transfer_requested`: transfer is proposed but not final.
6. `transfer_blocked`: policy, dispute, compliance, or legal restriction blocks transfer.
7. `transferred`: ownership changed through finalized transfer.
8. `disputed`: Overclaim dispute is active.
9. `restricted`: right can be used only under limited conditions.
10. `revoked`: right was revoked by evidence/policy.
11. `expired`: validity window ended.
12. `corrected`: correction record replaced or amended effective state.
13. `tombstoned`: historical marker remains but right is no longer active.

Transitions are append-only. Current state is a projection from transition refs.

## Policy And Security

- Rights are scoped facts, not universal ownership claims.
- Every right must cite source evidence and allowed use.
- Transferability defaults to false unless evidence and policy allow it.
- Disputed rights cannot transfer or broaden access until claim policy allows it.
- Grant-funded rights must preserve purpose scope and reporting requirements.
- Storage/vault rights must not expose raw object, private record, or secret contents.
- Namespace rights must respect tombstones, verification markers, anti-squatting controls, delegation, and disputes.
- Manual corrections and revocations require signed action, evidence refs, and Overwatch audit.
- No right may create NFT-like speculative behavior or market-price claims.

## Metering And Accounting

Overasset does not bill or settle. It records rights and evidence refs:

- Seal Ledger refs for resource rights, grants, holds, corrections, and transfer effects.
- ORU dimension refs for capacity, storage, or service-unit rights.
- Overbill receipt/invoice refs where an external billing event created or changed a right.
- Overgrant purpose-scope refs for sponsored rights.
- Overclaim dispute/correction refs affecting right state.

Accounting effects remain in Seal Ledger, ORU Account Service, Overbill, and Provider Payout Service.

## Observability And Operations

- Dashboards should show active rights by type, disputed rights, transfer requests, blocked transfers, expired rights, revoked rights, namespace-bound rights, storage entitlements, and correction rates.
- Operators need timelines that join source evidence, policy decisions, namespace/storage refs, ledger refs, dispute refs, and transfer history.
- Alerts should fire on unauthorized transfer attempts, rights without source evidence, disputed rights used for access, and replay mismatches.
- Right projection must support scoped replay by owner, namespace, storage ref, grant, or service.

## Failure Modes And Recovery

- Missing source evidence: reject creation or mark pending evidence.
- Policy unavailable: keep pending or block transfer/delegation.
- Conflicting ownership refs: mark disputed and require Overclaim.
- Transfer finalized in downstream service but not recorded: attach correction refs and replay.
- Storage entitlement revoked downstream: create revocation transition and notify consumers.
- Replay mismatch: mark integrity incident and preserve all refs.
- Legal/compliance restriction changes: restrict affected rights and require re-evaluation.

## Validation Plan

The service implementation plan lists these requirements:

- Rights are utility records, not speculative NFTs.
- Transfers cannot bypass policy, legal, or dispute restrictions.
- Ownership evidence is explainable from ledger/registry facts.

Additional SDS-level validation:

- Contract tests for create, read, explain, delegate, transfer, revoke, dispute attach, replay, owner query, and namespace query APIs.
- Policy tests proving transfer defaults to blocked unless allowed by evidence and policy.
- Dispute tests proving active disputes block transfer or broadening access.
- Namespace/storage integration tests proving rights are consumed as refs and do not bypass access policy.
- Redaction tests for storage/vault/private evidence.
- Replay tests proving current right state derives from source evidence and transition refs.
- Anti-NFT tests verifying no market price, royalty, speculative supply, or blockchain dependency fields are required for rights.

## Build Breakdown

1. Define resource right, evidence, entitlement, namespace binding, capacity claim, grant right, service ownership, delegation, transfer, revocation, dispute, and replay schemas.
2. Implement create/read/explain APIs for non-transferable storage or namespace rights.
3. Add delegation, expiry, revocation, and correction transitions.
4. Add Overclaim dispute hooks and transfer-blocking behavior.
5. Add transfer flow where policy/legal flags allow it.
6. Integrate Universal Namespace Service, Overstore, Overvault, Overgrant, Seal Ledger, and Overguard refs.
7. Add replay, dashboards, and audit exports.

## Handoff And Downstream Use

Overasset supports namespace rights, storage leases, capacity reservations, grants, app/service ownership, operational ownership flows, native app entitlements, Overguard policy checks, Overclaim disputes, and central AI stewardship audits.

## Open Design Questions

Resolved decisions:

- First implementation order is evidence-first and non-transferable by default. Phase 5 should create the minimum right records needed by accounting and operations: grant-right refs from Overgrant, capacity-claim rights backed by ORU/Seal Ledger/Overmark evidence, and service/app ownership refs backed by Overregistry and signed owner authority. Phase 8 then adds storage entitlement refs for Overstore/Overbase/Overvault and namespace-right bindings once Universal Namespace Service, Overstore, Overvault, Overguard, Overclaim, and retention/tombstone rules exist. Public namespace transfers, dataset/model/media/package rights metadata, and cross-owner operational transfers come after these refs are replayable and policy-covered; no right type starts as a tradable market object.
- First-deployment transferability is limited to narrow operational transfers, not resale, royalties, market trading, or NFT-like ownership. Allowed v0 transfer classes are same-tenant service/app ownership handoff, organization-admin reassignment, explicit delegation or subdelegation within the original right scope, grant authorization return/reassignment by the grant owner, storage entitlement migration to a replacement owner/account after Overguard approval, and low-risk private namespace transfer after Universal Namespace Service checks. Public/global names, native-app or system-service roots, provider-capacity claims, grant-funded public-interest rights, regulated or secret-bearing rights, disputed rights, payout/settlement-affecting rights, and cross-tenant/cross-jurisdiction transfers remain blocked or review-required until Compliance Boundary Service, Overclaim finality, and jurisdiction profiles explicitly allow them.
- Expiry changes the current Overasset projection; it does not erase namespace, storage, ledger, or audit history. An expired right cannot be used for new access, transfer, delegation, route broadening, grant authorization, or storage expansion unless a renewal or correction creates a new evidence-backed transition. Universal Namespace Service tombstones, reservation/no-reuse windows, verification markers, and route history remain governed by namespace policy rather than by the expired right alone. Overstore and Overvault retention, legal/compliance holds, backups, disputes, and purge eligibility remain governed by the storage owner service; Overasset records only the expired entitlement state, downstream effect refs, and replay evidence.
- Wallet and Usage Center should show user/org-actionable summaries for rights the viewer owns, controls, receives by delegation, or can dispute: active and expiring storage entitlements, grant-backed rights, namespace handles/routes, app/service ownership refs, permission/delegation status, revocations, disputes, receipts, and safe evidence summaries. Admin/operator surfaces may show broader redacted timelines, blocked transfer reasons, correction refs, compliance markers, provider-capacity context, and replay bundles. Provider-private evidence, fraud or anti-abuse internals, raw ledger internals, other-tenant refs, secret-bearing storage/vault refs, system-service emergency actions, and compliance/legal details stay operator-only or source-service-only behind stable reason codes.
- Private storage and vault-backed rights use a least-detail redaction profile. Default user-facing views may expose the right id, owning account/organization, service category, data class, allowed operation class, quota or resource-dimension band, expiry/retention class, current state, reason codes, dispute/revocation status, and hashed or opaque evidence refs. They must not expose decrypted content, secret names or values, raw vault key refs, private object paths, sensitive object ids, chunk hashes when linkable, placement/provider/node topology, private route targets, unrelated owner refs, raw payment/tax/compliance material, or fraud heuristics. Exact refs are available only to authorized owner services, narrowly scoped admins/operators, and replay/export jobs under Overguard, Overvault/Overstore, Overwatch, and Compliance Boundary policy.
