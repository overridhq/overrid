# Universal Namespace Service Implementation Plan

## Objective

Create human-readable names for people, organizations, apps, services, agents, swarms, communities, tags, assets, and routes.

## First Build Phase

[Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).

## Dependencies

- Overpass identities, accounts, usernames, and subject refs.
- Overtenant tenant, organization, membership, and role scope.
- Overkey signed claim, delegation, transfer, verification, and route-binding actions.
- Overguard naming, abuse, transfer, route, visibility, data-class, and dispute policy.
- Overasset operational rights and entitlement refs.
- Seal Ledger non-speculative ownership and correction evidence where required.
- Overmesh route resolution, endpoint refs, route health, and connectivity policy.
- Overwatch claim, route, transfer, verification, dispute, restriction, and cache-invalidation evidence.
- Overclaim dispute, appeal, correction, and finality workflows.
- Overbase namespace record storage once Phase 8 primitives exist.

## Development Order

1. Define namespace record, claim, owner-ref, target-ref, route-binding, delegation, transfer, verification, dispute, reservation, tombstone, and resolution-cache schemas.
2. Implement normalization, uniqueness, reserved-name, scoped-name, tombstone/no-reuse, and claim-decision checks.
3. Add owner refs, target refs, route bindings, privacy-aware resolution, and resolution-cache invalidation.
4. Add delegation grants, revocation, signed transfers, rights/evidence refs, releases, tombstones, and correction-preserving history.
5. Add issuer-scoped verification markers, reserved-name enforcement, anti-squatting controls, impersonation controls, abuse restrictions, and public-interest/community marker policy.
6. Add dispute records and Overclaim handoff for impersonation, squatting, misleading names, abandoned names, unauthorized transfer, and route hijack.
7. Integrate namespace lookup with messaging, search, maps, directory listings, native app routes, agent/service names, Overpack deployment routes, and Overmesh route resolution.
8. Add Overwatch audit evidence, Overmeter raw usage facts, dependency health checks, backup/restore handoffs, and operator review surfaces.

## Contracts And Interfaces

- Claim and reservation API.
- Namespace metadata and privacy-aware resolution API.
- Target-ref API.
- Route binding API.
- Delegation grant and revocation records.
- Signed transfer workflow records.
- Verification marker issuance, expiry, and revocation records.
- Dispute, restriction, appeal, and correction hook contracts.
- Release, tombstone, and reuse-policy records.
- Resolution cache and invalidation events.
- Raw usage event contract.

## Detailed SDS

The detailed design contract lives in [Universal Namespace Service SDS](../../sds/data_storage_namespace/universal_namespace_service.md).

- [SUB BUILD PLAN #30 - Universal Namespace Service](../../build_plan/sub_build_plan_030_universal_namespace_service.md)

## Design Alignment

- Treat the namespace service as the readable naming and route-binding layer, not the identity authority, asset-right system, ledger, object store, or private network.
- Store normalized names, owner refs, target refs, route bindings, delegation grants, transfers, verification markers, disputes, tombstones, and resolution cache entries.
- Use Overpass for identity refs, Overasset and Seal Ledger for rights/evidence refs, Overmesh for route resolution, and Overclaim for disputes.
- Keep public resolution privacy-aware so private identities, routes, storage refs, and secret-bearing endpoints are not leaked.
- Keep globally unique, tenant-local, app-local, and community-local uniqueness scopes explicit; day-one reserved names include canonical Overrid, service, native app, route-root, and operator/admin names plus confusing variants.
- Verification markers are issuer-scoped, time-bounded, revocable evidence records; they are not ownership or identity records by themselves.
- Route-binding approvals are tiered by risk, with public/global roots, verified names, data-class broadening, disputed names, asset/ledger-backed rights, and system-service roots requiring review, hold windows, or multi-signature approval.
- Emit raw usage facts only; ORU balances, Seal Ledger mutation, billing, settlement, payout, pricing, and external payment rails remain outside namespace logic.

## Validation

- A name resolves to the right identity, app, service, route, or asset.
- Unauthorized transfers are denied.
- Route changes emit audit events.
- Normalization tests catch case, whitespace, separator, Unicode-equivalent, confusing, reserved, scoped, and tombstoned names.
- Public resolution redacts private identities, tenant-private routes, secret-bearing endpoints, private storage refs, and cross-tenant existence signals.
- Verification, delegation, transfer, dispute, restriction, release, tombstone, and cache-invalidation paths are replayable from policy, evidence, usage, and audit refs.

## Handoff

The namespace service becomes the human and AI-facing address layer for Overrid.
