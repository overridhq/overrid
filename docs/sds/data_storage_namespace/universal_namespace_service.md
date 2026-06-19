SDS #30

# Universal Namespace Service SDS

## Purpose

Create and govern human-readable names for people, organizations, apps, services, agents, swarms, communities, tags, assets, routes, native app pages, and public or private references inside Overrid.

The Universal Namespace Service is the address and naming layer. It owns normalized names, ownership refs, delegation, transfers, route bindings, verification markers, disputes, tombstones, and resolution behavior. It does not own identity, assets, ledger state, object storage, or private connectivity.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [universal_namespace_service.md](../../service_catalog/data_storage_namespace/universal_namespace_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md) |

## Service Family

- Family: Data, storage, and namespace
- Owning layer: Names, normalized identifiers, route bindings, delegation, transfer, verification, dispute, and resolution records
- Primary data scope: namespace records, name claims, normalized names, owner refs, target refs, route bindings, delegation grants, transfer records, verification markers, dispute cases, reservation records, tombstones, and resolution cache entries
- First build phase from service plan: [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md)

## Problem Statement

Overrid needs a single human and AI-facing address layer. Users should not need to know internal ids for people, apps, services, agents, swarms, directory listings, assets, route targets, messaging destinations, search subjects, or native app pages. At the same time, names can be abused through impersonation, squatting, route hijack, misleading tags, abandoned names, and unauthorized transfers.

The Universal Namespace Service gives the ecosystem readable names while keeping identity, rights, route, policy, audit, and dispute boundaries explicit.

## Goals

- Define normalized, typed namespace records for people, organizations, apps, services, agents, swarms, tags, assets, routes, and native app pages.
- Resolve names to identity, app, service, route, asset, storage, or namespace target refs.
- Support ownership, delegation, transfer, verification markers, route binding, and tombstone records.
- Prevent unauthorized transfers and route changes.
- Provide dispute records for impersonation, squatting, misleading names, abandoned names, and route hijack.
- Integrate namespace lookup with messaging, search, maps, directory listings, native apps, Overmesh routes, and AI agents.
- Preserve audit evidence for every claim, binding, transfer, and dispute decision.

## Non-Goals

- Do not replace Overpass identity records or usernames.
- Do not own asset rights or financial ownership. Overasset and Seal Ledger supply rights and evidence refs.
- Do not own private network routes or endpoint health. Overmesh resolves and enforces connectivity.
- Do not act as a global public DNS replacement in v0.
- Do not automatically delete disputed names without evidence, appeal path, and audit records.
- Do not create NFT-like speculative name assets.
- Do not expose private route targets or private identities through public name resolution.

## Primary Actors And Clients

- Users, organizations, and service accounts claiming and managing names.
- Native apps using names for messaging, search, social, maps, directory listings, workspace, wallet, and central AI stewardship.
- Overpass, supplying identity and account refs.
- Overasset and Seal Ledger, supplying rights, entitlement, and ownership evidence refs.
- Overmesh, resolving route bindings to service endpoints and private connectivity refs.
- Overguard, checking naming, transfer, route, data-class, abuse, and dispute policy.
- Overwatch and Overclaim, consuming audit, dispute, and correction evidence.
- SDK, CLI, admin UI, and product adapters resolving names and managing bindings.

## Dependencies

- [Overpass](../control_plane/overpass.md) for identity, account, username, and subject refs.
- [Overtenant](../control_plane/overtenant.md) for tenant, organization, membership, and role scope.
- [Overkey](../control_plane/overkey.md) for signed claim, transfer, delegation, and route-binding actions.
- [Overguard](../trust_policy_verification/overguard.md) for naming policy, abuse checks, route policy, and dispute policy.
- [Overasset](../accounting/overasset.md) for operational rights and entitlement refs.
- [Seal Ledger](../accounting/seal_ledger.md) for non-speculative ownership and correction evidence where required.
- [Overmesh](../execution_scheduling/overmesh.md) for route resolution, endpoint refs, and connectivity policy.
- [Overwatch](../control_plane/overwatch.md) for audit and evidence records.
- [Overclaim](../trust_policy_verification/overclaim.md) for disputes and correction workflows.
- [Overbase](overbase.md) for namespace record storage once Phase 8 primitives exist.

## Owned Responsibilities

The Universal Namespace Service owns:

- Name normalization, uniqueness rules, namespace type definitions, and reserved-name rules.
- Namespace claim, reservation, activation, suspension, tombstone, and release lifecycle.
- Owner refs, target refs, delegation grants, transfer records, and route bindings.
- Verification markers for official, community, app, organization, service, and public-interest contexts.
- Resolution APIs with privacy-aware target disclosure.
- Dispute records for impersonation, squatting, route hijack, misleading names, abandoned names, and unauthorized transfer.
- Resolution cache invalidation and route-binding audit refs.
- Namespace usage events and policy-denial evidence.

The service must separate a readable name from the underlying identity or asset. Names point to refs; they are not the identity, asset, ledger, or route system itself.

## Data Model

The first implementation should define:

- `namespace_record`: namespace id, raw name, normalized name, namespace type, scope, owner ref, target refs, visibility, data class, current state, and policy refs.
- `name_claim`: claim id, normalized name, claimant ref, tenant/org scope, evidence refs, reservation type, expiration, and decision state.
- `namespace_owner_ref`: namespace id, owner type, owner id/ref, rights refs, effective time, transfer eligibility, and audit refs.
- `target_ref`: namespace id, target type, target id/ref, visibility, priority, route policy refs, and disclosure rule.
- `route_binding`: namespace id, route kind, Overmesh route ref, app/service route, storage ref, api route, native app page ref, ttl, and health refs.
- `delegation_record`: delegator ref, delegate ref, namespace scope, allowed operations, ttl, revocation state, and audit refs.
- `transfer_record`: source owner, target owner, namespace id, required signatures, rights refs, policy refs, decision state, and rollback/tombstone behavior.
- `verification_marker`: namespace id, marker kind, issuer ref, evidence refs, expiration, revocation state, and display policy.
- `dispute_record`: namespace id, dispute type, claimant refs, evidence refs, affected routes, temporary restrictions, decision refs, appeal refs, and correction refs.
- `reservation_record`: reserved normalized name, reason, reserved-by service or policy, release policy, and audit refs.
- `namespace_tombstone`: normalized name, former namespace id, reason, visibility, reuse policy, dispute refs, and retention.
- `resolution_cache_entry`: lookup key, resolved target refs, route refs, policy refs, visibility, ttl, invalidation refs, and cache state.

Common envelope fields:

- `id`, `tenant_id`, `actor_id` or service account.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

The v0 API should support claim, resolve, bind, delegate, transfer, verify, and dispute operations:

- `POST /namespace/claims`: claim or reserve a normalized name.
- `GET /namespace/{name}`: resolve a name with privacy-aware target filtering.
- `GET /namespace/{namespace_id}`: read authorized namespace metadata.
- `POST /namespace/{namespace_id}/targets`: add or update target refs.
- `POST /namespace/{namespace_id}/routes`: add or update route bindings.
- `POST /namespace/{namespace_id}/delegations`: grant namespace management rights.
- `POST /namespace/{namespace_id}/delegations/{delegation_id}/revoke`: revoke delegation.
- `POST /namespace/{namespace_id}/transfer`: start a signed transfer workflow.
- `POST /namespace/{namespace_id}/verification`: add or revoke verification markers.
- `POST /namespace/{namespace_id}/disputes`: open dispute workflow.
- `POST /namespace/{namespace_id}/suspend`: policy or dispute-based suspension.
- `POST /namespace/{namespace_id}/release`: release a name according to reuse and tombstone policy.

API requirements:

- Mutating actions require signed actor or service-account identity, tenant/org scope, trace id, and idempotency key.
- Resolution must apply visibility, data-class, tenant, role, and route-disclosure policy.
- Transfers require source and target authorization plus any rights or ledger evidence refs.
- Route changes must emit audit events and invalidate resolution cache entries.
- Dispute operations must preserve evidence, temporary restrictions, appeal refs, and correction refs.

## Event Surface

- `namespace.claim_requested`
- `namespace.claim_accepted`
- `namespace.claim_denied`
- `namespace.target_bound`
- `namespace.route_bound`
- `namespace.route_changed`
- `namespace.delegation_created`
- `namespace.delegation_revoked`
- `namespace.transfer_requested`
- `namespace.transfer_completed`
- `namespace.verification_added`
- `namespace.verification_revoked`
- `namespace.dispute_opened`
- `namespace.dispute_restricted`
- `namespace.dispute_resolved`
- `namespace.suspended`
- `namespace.tombstoned`
- `namespace.resolution_cache_invalidated`

Events must include namespace, normalized name, actor, owner, target, route, policy, dispute, and trace refs. Private target details must be redacted when the event is not for an authorized operator or service.

## Core Workflow

1. Actor submits a name claim with namespace type, scope, target refs, and evidence refs.
2. Service normalizes the name and checks reservation, uniqueness, abuse, impersonation, and tenant/org policy.
3. Allowed claim creates a namespace record and owner refs.
4. Owner binds targets and routes through policy-checked signed actions.
5. Resolution requests return authorized target refs and route refs according to visibility and data class.
6. Delegation grants limited management rights to other actors or service accounts.
7. Transfer workflows collect required signatures and rights/ledger evidence before owner change.
8. Verification markers are issued, expired, or revoked by authorized issuers.
9. Disputes can restrict route changes or visibility until Overclaim and policy decisions resolve them.
10. Tombstones preserve audit and reuse policy after release, deletion, dispute, or correction.

## State Machine

Namespace lifecycle:

1. `claim_requested`: claim submitted and pending checks.
2. `reserved`: name is held by policy or temporary reservation.
3. `active`: namespace resolves according to policy.
4. `delegated`: management rights exist for delegate refs.
5. `pending_transfer`: transfer workflow is active.
6. `transferred`: ownership changed through signed workflow.
7. `verified`: at least one active verification marker exists.
8. `disputed`: dispute is open and may restrict changes.
9. `restricted`: resolution or route changes are limited by policy or dispute.
10. `suspended`: normal use is blocked.
11. `released`: owner released the name and reuse policy is active.
12. `tombstoned`: historical record is retained and normal reuse is blocked or delayed.

Route-binding lifecycle:

1. `proposed`
2. `policy_checked`
3. `active`
4. `degraded`
5. `replaced`
6. `revoked`
7. `disputed`

History is append-only. Correction records create new versions rather than rewriting name ownership or route history.

## Policy And Security

- Names are normalized before uniqueness and abuse checks.
- Reserved system names, misleading names, impersonation patterns, route-hijack patterns, and public-interest names require explicit policy.
- Ownership changes require signed transfer workflow and rights refs where applicable.
- Public resolution must not expose private identities, tenant-private routes, secret-bearing endpoints, or non-public storage refs.
- Verification markers must be issuer-scoped, time-bounded, revocable, and auditable.
- Dispute restrictions should be evidence-based and appealable; automatic deletion is not an acceptable first response.
- Delegation grants must be narrow, ttl-bound where appropriate, and revocable.
- Route binding must check Overmesh policy and must invalidate stale cache entries.
- Names and namespace rights must not become speculative NFT-like assets.

## Metering And Accounting

The Universal Namespace Service emits raw usage events; it does not bill:

- Claim attempts, accepted claims, denied claims, resolution reads, route changes, delegation changes, transfers, verification changes, disputes, cache invalidations, and policy-denied operations.
- Usage should link to tenant, actor, namespace type, app/service, data class, operation kind, and policy refs.
- Rights, ownership, and correction evidence can reference Overasset and Seal Ledger, but the namespace service never mutates ORU balances or settlement entries.
- Near-cost native app economics belong to billing/accounting layers, not namespace logic.

## Observability And Operations

- Operators need views for claim queues, denied claims, reserved-name hits, active disputes, route changes, transfer workflows, verification markers, cache invalidations, and abuse reason trends.
- Health checks should cover normalization, uniqueness index, Overguard, Overpass, Overasset, Seal Ledger refs where used, Overmesh route checks, Overwatch emission, and cache invalidation.
- Resolution latency and cache hit rate should be tracked by namespace type and visibility.
- Dispute and suspension views must show evidence refs, restrictions, appeal state, and correction path.
- Route-binding diagnostics must separate namespace issues from Overmesh endpoint-health issues.

## Failure Modes And Recovery

- Name normalization conflict: return existing namespace or conflict reason without creating a duplicate.
- Reserved or prohibited name: deny claim with safe policy reason.
- Missing owner or rights refs: block claim, transfer, or route binding until refs are valid.
- Unauthorized transfer: deny and emit audit event.
- Route hijack attempt: restrict route change, open dispute or incident refs, and preserve previous route.
- Stale route cache: invalidate on every route/target/visibility change and serve stale data only when policy allows.
- Dispute opened after route binding: keep current safe route or restrict resolution according to dispute policy.
- Overmesh unavailable: keep namespace active but mark route resolution degraded.
- Correction required: append correction record, update current pointer, and preserve tombstone/history.

## Validation Plan

The service implementation plan lists these requirements:

- A name resolves to the right identity, app, service, route, or asset.
- Unauthorized transfers are denied.
- Route changes emit audit events.

Additional SDS-level validation:

- Contract tests for claim, resolve, target bind, route bind, delegate, transfer, verify, dispute, suspend, release, and cache invalidation APIs.
- Normalization tests for case, whitespace, separator, Unicode-equivalent, reserved, and confusing names.
- Tenant, organization, role, and data-class resolution tests.
- Unauthorized transfer, delegation abuse, and route-hijack tests.
- Public versus private resolution redaction tests.
- Dispute tests for impersonation, squatting, abandoned names, misleading names, and route hijack.
- Audit and usage emission tests for every mutating operation.

## Build Breakdown

1. Define namespace record, claim, owner ref, target ref, route binding, delegation, transfer, verification, dispute, reservation, tombstone, and resolution cache schemas.
2. Implement normalization, uniqueness, reserved-name, and claim checks.
3. Add ownership, target binding, and route binding APIs.
4. Add resolution API with visibility and data-class filtering.
5. Add delegation and signed transfer workflows.
6. Add verification markers and issuer policy.
7. Add disputes for impersonation, squatting, misleading names, abandoned names, unauthorized transfer, and route hijack.
8. Integrate namespace lookup with messaging, search, maps, directory listings, app routes, and Overmesh route resolution.

## Handoff And Downstream Use

The Universal Namespace Service becomes the human and AI-facing address layer for Overrid. Messaging should resolve usernames through it, search should index authorized namespace metadata, maps and directory listings should bind public place/business names carefully, native apps should use namespace route refs, and Overmesh should consume route bindings without owning the name.

## Open Design Questions

Resolved decisions:

- Namespace uniqueness is classed by authority boundary. Globally unique names are required for protocol and system-service ids, public person and organization handles, public native-app ids, public service ids, public route roots, public asset refs, purpose tags, and grid/system-service route refs. Tenant-local names cover tenant member aliases, service-account aliases, internal service names, private node labels, tenant app aliases, quota or billing display refs, and tenant route aliases. App-local and community-local names resolve only inside their app, package, or community authority and must not leak cross-tenant existence signals.
- Day-one reserved names include the canonical Overrid brand, protocol, system-service, route-root, native-app, and operator/admin words: `overrid`, `oru`, `seal-ledger`, `system`, `root`, `admin`, `api`, `support`, every canonical service id such as `overgate`, `overpass`, `overkey`, `overtenant`, `overregistry`, `overwatch`, `overqueue`, `overguard`, `oververify`, `overclaim`, `overmesh`, `oversched`, `overlease`, `overrun`, `overmeter`, `overpack`, `overcache`, `overbase`, `overstore`, `overvault`, and `overasset`, plus native app roots such as `wallet`, `search`, `maps`, `messaging`, `workspace`, `directory`, `social`, and `central-ai`. Confusing, Unicode-equivalent, separator-equivalent, and high-risk variants inherit the reservation until Overguard policy explicitly releases a scoped exception.
- Anti-squatting and impersonation restrictions use a graduated evidence threshold. Hard denials apply when the normalized name collides with an existing active or reserved record, violates a tombstone/no-reuse rule, lacks owner authority, or targets a protected system/native-app route. Temporary restrictions may be applied from credible evidence refs such as verified identity or organization refs, Overasset or Seal Ledger rights refs, Overwatch route/claim evidence, prior verification markers, route-hijack indicators, or an admitted Overclaim dispute. Permanent transfer, reassignment, suspension, or tombstone requires an Overclaim resolution with affected-party notice or response window, Overguard reason-coded policy decision, appeal/finality refs, and Overwatch audit evidence.
- Verification markers are issuer-scoped, time-bounded, revocable records, not identity or ownership by themselves. Organization markers require Overpass/Overtenant organization authority and signed issuer credentials; community markers are issued by the community owner or delegated moderator scope and display only within that community unless promoted by policy; public-interest markers require authorized stewardship or public-interest service issuance with evidence refs and renewal rules. Markers can be revoked by issuer action, expiry, lost owner/issuer authority, credential compromise, abuse policy, Overclaim resolution, or Overguard emergency block, and every issuance or revocation must preserve marker kind, issuer ref, target ref, evidence refs, display policy, reason codes, and audit refs.
- Route-binding changes are tiered by risk. Low-risk tenant-local or app-local changes by the current owner/delegate, within the same target type, same visibility, and same data class, can activate immediately after signed Overguard policy checks and Overwatch emission. Public/global route roots, verified organization or public-interest names, target-type changes, visibility or data-class broadening, `storage_ref`, `identity_ref`, `asset_ref`, regulated/private/secret-bearing routes, high-traffic native-app routes, disputed names, and stale or degraded route targets require delay, review, or hold windows. Multi-signature approval is required for system-service route roots, native-app public roots, cross-tenant or cross-owner transfers, asset/ledger-backed namespace rights, emergency recovery, and any route change under active dispute or incident; Overmesh resolves only the active approved binding while prior versions remain replayable.
