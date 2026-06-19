SDS #10

# Overpass SDS

## Purpose

Build Overrid identity and namespace records for people, organizations, nodes, apps, native services, service accounts, system services, swarms, agents, communities, tags, routes, and later human-readable names.

Overpass is the durable identity and namespace authority. It gives Overgate, Overtenant, Overkey, Overregistry, native apps, messaging, search, directory, wallet, AI assistant, and grid services stable references for who or what is acting, what state that identity is in, and how public or tenant-local names resolve.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overpass.md](../../service_catalog/control_plane/overpass.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md), [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md) |
| Sub-build plan | [SUB BUILD PLAN #10 - Overpass](../../build_plan/sub_build_plan_010_overpass.md) |

## Service Family

- Family: Control plane.
- Owning layer: Identity, subject references, namespace records, and route-binding references.
- Primary data scope: identity records, identity lifecycle records, namespace records, route bindings, owner refs, verification markers, and dispute refs.
- First build phase from service plan: Overpass-lite in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); broader namespace in [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).

## Problem Statement

Overrid cannot govern resources, route messages, map usernames, verify service accounts, or stop abuse without a stable identity layer. The system needs identity records that survive key rotation, app migration, tenant changes, disputes, and route changes. Overpass provides those records and separates durable identity from credentials, tenant authorization, and asset ownership.

## Goals

- Define stable identity records for people, organizations, nodes, apps, native services, service accounts, system services, and central AI-related system actors.
- Provide identity lifecycle states that Overgate and downstream services can enforce.
- Connect identity references into tenant, key, manifest, workload, message, directory, wallet, AI assistant, and native-service records.
- Add namespace records for human-readable names and routes without making names the identity primary key.
- Support verification markers, ownership refs, delegated refs, and dispute hooks.
- Preserve audit history and tombstones so old references cannot be silently reassigned.
- Support future grid operation where core tools run without founder-owned servers.

## Non-Goals

- Do not store credentials or private keys; Overkey and Overvault own those.
- Do not decide tenant authorization; Overtenant and policy services own membership and permissions.
- Do not become a social profile, directory listing, wallet, or messaging service.
- Do not make usernames or route names transferable speculative assets.
- Do not delete identity history in a way that allows impersonation or audit loss.
- Do not encode charge tables, customer-count assumptions, or economic projections.

## Primary Actors And Clients

- Overgate resolving actor identity.
- Overtenant resolving membership and ownership references.
- Overkey attaching credentials to subjects.
- Overregistry and Overrun attaching apps, native services, nodes, and workloads to identities.
- Native apps for social, messaging, directory, workspace, maps, search, wallet, and AI assistant identity references.
- Node agents, service accounts, and system services.
- Operators handling suspension, dispute, and recovery flows.
- Central AI governance workflows reviewing fraud, abuse, and malicious entity signals.

## Dependencies

- Shared identity and namespace schemas.
- Overgate for signed identity lifecycle commands.
- Overtenant for tenant membership, owner refs, and role bindings.
- Overkey for credential references and key lifecycle.
- Overwatch event log for identity and namespace audit evidence.
- Overguard for suspension, abuse, verification, and dispute policy checks once available.
- Overasset for ownership/resource-right refs in later phases.
- Seal Ledger and wallet services for account refs that must resolve to stable subjects.

## Owned Responsibilities

Overpass owns:

- Identity record creation and lifecycle state.
- Subject refs for person, organization, node, app, native service, service account, system service, swarm, agent, community, and central AI system actors.
- Public and tenant-local namespace records.
- Route-binding refs for apps, native services, messaging handles, directory entries, and service endpoints.
- Verification markers and evidence refs.
- Tombstones and reassignment protection.
- Identity merge, recovery, and dispute references when policy permits.
- Query surfaces for identity resolution that return only allowed fields.

Overpass does not own credentials, tenant authorization, content moderation decisions, usage accounting, or user profile content beyond identity references.

## Data Model

The first implementation should define these records:

- `identity_record`: identity id, identity type, display handle, tenant scope where applicable, lifecycle state, created by, created time, updated time, tombstone state, and audit refs.
- `identity_subject_ref`: stable ref used by commands, manifests, credentials, messages, wallet accounts, native apps, and service records.
- `namespace_record`: namespace id, namespace type, name, normalized name, scope, target ref, state, reservation refs, and audit refs.
- `route_binding`: route id, namespace ref, target service or app ref, tenant scope, route class, state, verification refs, and conflict refs.
- `membership_ref`: tenant id, identity id, role ref, owner ref, source service, and state mirrored from Overtenant where needed.
- `verification_marker`: marker id, identity id, marker type, evidence ref, issuer identity, expiry, state, and dispute refs.
- `identity_link`: source identity, target identity, relationship type, confidence or verification ref, state, and policy refs.
- `tombstone_record`: identity or namespace ref, tombstone reason, effective time, reserved names, no-reuse window, and audit refs.
- `dispute_ref`: disputed identity or namespace, claimant identity, reason code, evidence refs, current state, and policy refs.

Identity states must include `pending`, `active`, `disabled`, `suspended`, `tombstoned`, and `merged`.

## API Surface

Phase 1 should expose identity basics:

- `POST /v1/identities`: create person, organization, node, app, native-service, service-account, or system-service identity.
- `GET /v1/identities/{identity_id}`: resolve allowed identity fields.
- `GET /v1/identities:resolve`: resolve by subject ref, namespace, or route ref according to caller permissions.
- `POST /v1/identities/{identity_id}/state`: transition identity state with reason and audit evidence.
- `POST /v1/identities/{identity_id}/links`: create relationship or delegation refs where policy allows.
- `POST /v1/namespaces`: reserve or bind namespace records after Phase 8 begins.
- `GET /v1/namespaces:resolve`: resolve public or tenant-local names to target refs.
- `POST /v1/routes`: bind route refs to apps, native services, messaging handles, or service endpoints once route binding is enabled.
- `GET /v1/healthz` and `GET /v1/readyz`: liveness and dependency readiness.

Admin and dispute APIs must be signed through Overgate, tenant-aware, and audited.

## Event Surface

Overpass should emit these Overwatch-compatible events:

- `overpass.identity_requested`: identity creation command received.
- `overpass.identity_created`: identity record created.
- `overpass.identity_state_changed`: lifecycle state changed.
- `overpass.identity_suspended`: identity lost mutating privileges.
- `overpass.identity_tombstoned`: identity retired while preserving history.
- `overpass.identity_merged`: identity link or merge recorded.
- `overpass.namespace_reserved`: namespace held for an identity or tenant.
- `overpass.namespace_bound`: namespace target ref changed through approved command.
- `overpass.route_bound`: route ref attached to a target.
- `overpass.verification_marker_added`: verification evidence attached.
- `overpass.dispute_opened`: dispute ref created for identity or namespace.

Events should use refs for private evidence and must never expose hidden profile data by default.

## Core Workflow

1. Overgate admits a signed identity or namespace command.
2. Overpass validates schema, actor permission, tenant scope, and requested identity type.
3. Overpass checks uniqueness and tombstone rules for subject refs and namespace names.
4. Overpass creates or updates identity, namespace, route, verification, or dispute records.
5. Overpass emits audit events with trace id, actor id, tenant id, target refs, and reason code.
6. Downstream services resolve identities through query APIs and cache only allowed fields.
7. Suspension, tombstone, merge, and dispute transitions propagate through events so Overgate and downstream services can block protected mutations.

## State Machine

Identity lifecycle:

1. `pending`: identity exists but cannot perform protected mutations.
2. `active`: identity may act according to tenant, credential, and policy state.
3. `disabled`: owner or operator disabled normal use; restoration may be allowed.
4. `suspended`: policy or operator action blocks protected mutation pending review or expiry.
5. `merged`: identity redirects to another stable identity ref while preserving history.
6. `tombstoned`: identity is retired and cannot be restored except through a formal recovery process.

Namespace lifecycle:

1. `reserved`: name is held but not publicly bound.
2. `active`: name resolves to a target ref.
3. `locked`: name cannot change because of dispute, policy, or protection rules.
4. `transferring`: approved target change is in progress.
5. `released`: name no longer resolves but remains protected by no-reuse policy.
6. `tombstoned`: name is retired and cannot be reassigned.

State transitions must be append-only. Names and identities cannot be reused in a way that breaks old evidence.

## Policy And Security

- Stable identity ids, not usernames, are the authority for commands and audit.
- Suspended, disabled, tombstoned, or merged identities must not mutate protected state unless a recovery policy explicitly allows a narrow action.
- Namespace names must be normalized consistently and protected against lookalike and collision attacks.
- Sensitive identity fields require tenant, role, and data-class filtering.
- Identity deletion must be tombstone-based to prevent impersonation and audit loss.
- Verification markers require issuer identity, evidence ref, expiry or renewal rule, and revocation path.
- Route bindings must be owned by identities or tenants with explicit authority.
- Central AI and operator review flows must receive evidence refs and reason codes, not private data dumps.

## Metering And Accounting

Overpass is not an accounting authority. It should:

- Emit usage-relevant events for identity creation, namespace reservation, route binding, verification marker changes, and operator actions.
- Provide identity refs for Overmeter attribution, ORU account refs, wallet accounts, native service billing, and Seal Ledger evidence.
- Avoid direct ORU or Seal Ledger mutation.
- Preserve enough stable refs for low-friction machine-to-machine settlement and dispute resolution.
- Keep namespace and identity operations independent from speculative asset mechanics.

## Observability And Operations

Overpass should expose:

- Identity counts by type and state.
- Namespace counts by scope and state.
- Suspended, disabled, disputed, and tombstoned identity views.
- Name collision and normalization diagnostics.
- Route-binding health and stale-target reports.
- Audit search hooks through Overwatch.
- Cache invalidation events for Overgate, Overtenant, Overkey, messaging, search, wallet, and native apps.
- Migration tooling for schema changes and namespace normalization rules.

## Failure Modes And Recovery

- Duplicate identity request with same idempotency key: return prior compatible result.
- Duplicate namespace normalized name: deny or open dispute according to policy.
- Suspended actor attempts mutation: deny with stable reason code.
- Missing Overtenant authority: block tenant-scoped identity or namespace changes.
- Overwatch unavailable for lifecycle mutation: fail closed unless an emergency buffer is active.
- Tombstoned identity referenced by active command: deny protected mutation and return replacement ref if merged.
- Namespace target missing or tombstoned: lock or suspend route binding until corrected.
- Accidental suspension: require signed recovery command, reason, and audit evidence.

## Validation Plan

The service implementation plan lists these specific validation requirements:

- Every command resolves actor identity.
- Suspended identities cannot mutate protected state.
- Namespace records resolve to identity, app, service, route, or asset records.

Additional SDS-level validation:

- Contract tests cover identity create/read/resolve/state, namespace reserve/bind/resolve, and route binding.
- State-machine tests prove invalid transitions are denied and history is preserved.
- Tenant isolation tests prove private identity fields and tenant-local namespaces are filtered.
- Suspension tests prove Overgate blocks protected mutations for suspended identities.
- Tombstone tests prove retired names and identities are not silently reused.
- Normalization tests cover case, Unicode lookalikes where supported, reserved words, and route collisions.
- Dispute tests prove evidence refs and reason codes are preserved.

## Build Breakdown

1. Define identity, subject ref, lifecycle state, namespace, route, verification marker, tombstone, and dispute schemas.
2. Implement Overpass-lite identity storage for people, organizations, nodes, apps, native services, service accounts, and system services.
3. Add identity resolution API for Overgate, Overtenant, and Overkey.
4. Add lifecycle transitions for pending, active, disabled, suspended, tombstoned, and merged states.
5. Add identity refs into tenant, manifest, credential, and command flows.
6. Add human-readable namespace and route-binding records in [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).
7. Add verification marker, delegation, recovery, and dispute hooks.

The Phase 1 build should be small but must use real identity refs so later native apps and grid services do not need a migration away from placeholder usernames.

## Handoff And Downstream Use

Overpass feeds Overtenant, Overregistry, Overmesh, Overasset, Seal Ledger, wallet, messaging, search, directory listing, social, workspace, personal AI assistant, and other native apps.

Downstream services should store stable identity refs and resolve display names or routes through Overpass. If a service needs a new identity type, namespace class, or route binding, update this SDS, the shared schema package, the service implementation plan, and the build-plan crosswalk together.

## Open Design Questions

Resolved decisions:

- The first seed-hardware control-plane flow must support person, organization, node, app, native-service, service-account, and system-service identities. This is the Phase 1 Overpass-lite set needed for tenant creation, seed node registration, signed SDK/CLI and service-account commands, manifest ownership, system-service audit refs, and a synthetic workload reaching Overqueue. Swarm, agent, community, tag, public route, asset, and broader human-readable namespace identities may be modeled in schemas as reserved enum values, but they are not required mutating surfaces until later phases prove the private execution and namespace platform paths.
- Tombstoned usernames, handles, route names, and namespace labels are never immediately reusable. Phase 1 should treat tombstones as permanent for identity ids and subject refs, and should reserve display handles for at least 1 year after tombstone, merge, suspension-for-abuse, or confirmed impersonation. Phase 8 namespace records should make the no-reuse window classed and policy-controlled: ordinary tenant-local names may use a 1-year minimum, public/global names and native-app route names require indefinite reservation unless a signed recovery or dispute outcome explicitly rebinds them, and high-risk or abuse-related names remain permanently locked. All releases, transfers, and exceptions require Overgate admission, Overwatch evidence, reason codes, and no speculative sale or NFT-style ownership path.
- Namespace scope is classed by authority boundary. Global namespace classes are protocol/system-service ids, public person or organization handles, public native-app ids, public service ids, public route roots, public asset refs, purpose tags, and grid/system-service route refs. Tenant-local classes include tenant member aliases, service-account aliases, internal service names, private node labels, tenant app aliases, quota/billing display refs, and tenant route aliases. App-local classes include page, document, collection, workspace, automation, model, dataset, media, and internal app route names that only resolve inside one app or package authority. Community-local classes include community handles, channels, groups, local tags, moderation labels, and community-owned route aliases. Phase 1 should implement only the identity ids and minimal display-handle resolution needed by Overgate, Overtenant, Overkey, and Overregistry; Phase 8 introduces the universal namespace, route binding, delegation, transfer, anti-squatting, and dispute machinery.
- Central AI and operators may issue verification markers that expose marker type, issuer identity, target identity or namespace ref, evidence ref, confidence or decision class where needed, expiry/renewal rule, revocation path, and stable reason codes. Allowed marker classes include founder-seed-node verified, organization ownership verified, service/system-service controlled-by verified, native-app publisher verified, public-interest/stewardship reviewed, abuse/fraud risk flagged, recovery-reviewed, dispute-pending, dispute-resolved, route-risk flagged, and namespace-protection marker. The marker payload must use evidence refs, hashes, redacted summaries, and Overwatch trace ids rather than private documents, secrets, payment details, credential material, or unrestricted profile data; callers receive only fields allowed by tenant, role, data class, and policy.
- Identity merge and recovery are append-only, policy-gated state transitions, not direct edits. Lost-credential recovery starts as a signed recovery command when any valid protected credential remains, or as a break-glass/operator-assisted recovery request with offline evidence refs when all credentials are lost. Malicious rotation or suspected compromise freezes the affected identity, credentials, namespace bindings, and high-risk service-account actions; Overkey revokes or suspends credentials, Overpass opens a recovery/dispute ref, and Overgate blocks protected mutations until review reaches a signed outcome. A merge creates an `identity_link` from source to target, marks the source `merged`, preserves tombstones/no-reuse protections, emits Overwatch events, and returns replacement refs on reads. Rebinding credentials, tenant ownership, namespace records, wallet/account refs, and route bindings is done through explicit service-owned commands with idempotency keys, reason codes, evidence refs, and compatibility tests; raw private evidence and secret material never move through Overpass.
